//! Zero-heap chunked Lazy Field Evaluator (`VFieldEvaluator`), prototype point instancing,
//! and welded instance realization.

use super::{p_add, p_cross, p_normalize, p_scale, p_sub, CHUNK_SIZE};
use crate::*;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    // ── Morphism 1: Lazy Field Evaluator ─────────────────────────────────────

    pub(super) fn manifold_evaluate_field(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "domain",
                "expression",
                "amplitude",
                "frequency",
                "apply_displacement",
            ],
        )?;

        let mesh = {
            let Artifact::Mesh(m) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            m.clone()
        };

        let domain = match a.get("domain") {
            Some(v) => text(v)?,
            None => "point",
        };
        match domain {
            "point" | "edge" | "face" | "instance" => {}
            _ => {
                return Err(format!(
                    "Unknown domain: {domain}; expected 'point', 'edge', 'face', or 'instance'"
                ))
            }
        }

        let expression = match a.get("expression") {
            Some(v) => text(v)?,
            None => "radial_wave",
        };
        match expression {
            "radial_wave" | "height_gradient" | "curvature_proxy" | "normal_displace" => {}
            _ => {
                return Err(format!(
                    "Unknown expression: {expression}; expected 'radial_wave', 'height_gradient', 'curvature_proxy', or 'normal_displace'"
                ))
            }
        }

        let amplitude = number(a, "amplitude", 0.15)?;
        if !amplitude.is_finite() || !(-10.0..=10.0).contains(&amplitude) {
            return Err("amplitude outside -10.0..=10.0 range".into());
        }

        let frequency = number(a, "frequency", 2.0)?;
        if !frequency.is_finite() || !(0.01..=100.0).contains(&frequency) {
            return Err("frequency outside 0.01..=100.0 range".into());
        }

        let apply_displacement = boolean(a, "apply_displacement", true)?;

        // Compute smooth unit normals for vertices
        let mut vertex_normals = vec![[0.0f32; 3]; mesh.positions.len()];
        for tri in &mesh.triangles {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            if i0 < mesh.positions.len() && i1 < mesh.positions.len() && i2 < mesh.positions.len() {
                let p0 = mesh.positions[i0];
                let p1 = mesh.positions[i1];
                let p2 = mesh.positions[i2];
                let cross = p_cross(p_sub(p1, p0), p_sub(p2, p0));
                for &idx in &[i0, i1, i2] {
                    vertex_normals[idx][0] += cross[0];
                    vertex_normals[idx][1] += cross[1];
                    vertex_normals[idx][2] += cross[2];
                }
            }
        }
        for n in &mut vertex_normals {
            *n = p_normalize(*n);
        }

        let centroid = [
            (mesh.min[0] + mesh.max[0]) * 0.5,
            (mesh.min[1] + mesh.max[1]) * 0.5,
            (mesh.min[2] + mesh.max[2]) * 0.5,
        ];

        // Gather domain elements & coordinate positions
        let mut element_positions: Vec<[f32; 3]> = Vec::new();
        match domain {
            "point" => {
                element_positions.clone_from(&mesh.positions);
            }
            "edge" => {
                let mut edge_set = std::collections::BTreeSet::new();
                for tri in &mesh.triangles {
                    for i in 0..3 {
                        let u = tri[i];
                        let v = tri[(i + 1) % 3];
                        edge_set.insert((u.min(v), u.max(v)));
                    }
                }
                for (u, v) in edge_set {
                    let pu = mesh.positions.get(u as usize).copied().unwrap_or([0.0; 3]);
                    let pv = mesh.positions.get(v as usize).copied().unwrap_or([0.0; 3]);
                    element_positions.push(p_scale(p_add(pu, pv), 0.5));
                }
            }
            "face" => {
                for tri in &mesh.triangles {
                    let p0 = mesh
                        .positions
                        .get(tri[0] as usize)
                        .copied()
                        .unwrap_or([0.0; 3]);
                    let p1 = mesh
                        .positions
                        .get(tri[1] as usize)
                        .copied()
                        .unwrap_or([0.0; 3]);
                    let p2 = mesh
                        .positions
                        .get(tri[2] as usize)
                        .copied()
                        .unwrap_or([0.0; 3]);
                    element_positions.push(p_scale(p_add(p_add(p0, p1), p2), 1.0 / 3.0));
                }
            }
            "instance" => {
                element_positions.push(centroid);
            }
            _ => unreachable!(),
        }

        let total_elements = element_positions.len();
        let chunk_count = if total_elements == 0 {
            0
        } else {
            (total_elements + CHUNK_SIZE - 1) / CHUNK_SIZE
        };

        self.charge(total_elements * 4 + 1024, total_elements * 16 + 2048)?;

        let mut evaluated_values = Vec::with_capacity(total_elements);
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;
        let mut sum_val = 0.0f64;

        // Zero-heap chunked Lazy Field Evaluator (`VFieldEvaluator`)
        for chunk_idx in 0..chunk_count {
            let start = chunk_idx * CHUNK_SIZE;
            let end = (start + CHUNK_SIZE).min(total_elements);
            let mut chunk_buf = [0.0f64; CHUNK_SIZE];

            for (i, elem_pos) in element_positions[start..end].iter().enumerate() {
                let p = *elem_pos;
                let val = match expression {
                    "radial_wave" => {
                        let dx = (p[0] - centroid[0]) as f64;
                        let dy = (p[1] - centroid[1]) as f64;
                        let dz = (p[2] - centroid[2]) as f64;
                        let r = (dx * dx + dy * dy + dz * dz).sqrt();
                        amplitude * (frequency * r).sin()
                    }
                    "height_gradient" => {
                        let dy = (p[1] - centroid[1]) as f64;
                        amplitude * (frequency * dy).sin()
                    }
                    "curvature_proxy" => {
                        let dx = (p[0] - centroid[0]) as f64;
                        let dy = (p[1] - centroid[1]) as f64;
                        let dz = (p[2] - centroid[2]) as f64;
                        amplitude * ((frequency * (dx + dy + dz)).cos() * 0.5)
                    }
                    "normal_displace" => {
                        let dist = ((p[0] * p[0] + p[1] * p[1] + p[2] * p[2]) as f64).sqrt();
                        amplitude * (frequency * dist).cos()
                    }
                    _ => unreachable!(),
                };
                chunk_buf[i] = val;
                min_val = min_val.min(val);
                max_val = max_val.max(val);
                sum_val += val;
            }

            evaluated_values.extend_from_slice(&chunk_buf[..(end - start)]);
        }

        let (min_value, max_value, mean_value) = if total_elements == 0 {
            (0.0, 0.0, 0.0)
        } else {
            (min_val, max_val, sum_val / (total_elements as f64))
        };

        // Displace mesh vertices along normals if requested
        let displaced_positions = if apply_displacement && !mesh.positions.is_empty() {
            let mut new_pos = mesh.positions.clone();
            match domain {
                "point" => {
                    for (i, p) in new_pos.iter_mut().enumerate() {
                        let d = evaluated_values.get(i).copied().unwrap_or(0.0) as f32;
                        let n = vertex_normals[i];
                        *p = p_add(*p, p_scale(n, d));
                    }
                }
                "face" => {
                    let mut vert_disp = vec![0.0f32; mesh.positions.len()];
                    let mut vert_cnt = vec![0u32; mesh.positions.len()];
                    for (f_idx, tri) in mesh.triangles.iter().enumerate() {
                        let d = evaluated_values.get(f_idx).copied().unwrap_or(0.0) as f32;
                        for &v in tri {
                            let idx = v as usize;
                            if idx < vert_disp.len() {
                                vert_disp[idx] += d;
                                vert_cnt[idx] += 1;
                            }
                        }
                    }
                    for (i, p) in new_pos.iter_mut().enumerate() {
                        if vert_cnt[i] > 0 {
                            let d = vert_disp[i] / (vert_cnt[i] as f32);
                            let n = vertex_normals[i];
                            *p = p_add(*p, p_scale(n, d));
                        }
                    }
                }
                "edge" | "instance" => {
                    let uniform_disp = mean_value as f32;
                    for (i, p) in new_pos.iter_mut().enumerate() {
                        let n = vertex_normals[i];
                        *p = p_add(*p, p_scale(n, uniform_disp));
                    }
                }
                _ => {}
            }
            new_pos
        } else {
            mesh.positions.clone()
        };

        let mesh_handle = self.store_mesh_positions(displaced_positions, mesh.triangles)?;

        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh_handle),
            ("domain".into(), Value::String(domain.to_string())),
            ("element_count".into(), Value::U64(total_elements as u64)),
            ("chunk_count".into(), Value::U64(chunk_count as u64)),
            ("min_value".into(), Value::F64(min_value)),
            ("max_value".into(), Value::F64(max_value)),
            ("mean_value".into(), Value::F64(mean_value)),
        ])))
    }

    // ── Morphism 4: Instance on Points ───────────────────────────────────────

    pub(super) fn manifold_instance_on_points(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "prototype",
                "points",
                "points_mesh",
                "scales",
                "scale",
                "rotations",
            ],
        )?;

        let proto_val = required(a, "prototype")?;
        let proto = {
            let Artifact::Mesh(m) = self.workspace.get(proto_val)? else {
                return Err("Expected mesh handle for 'prototype'".into());
            };
            m.clone()
        };

        // Parse target points
        let points: Vec<[f64; 3]> = if let Some(pts_val) = a.get("points") {
            let list_val = list(pts_val)?;
            let mut pts = Vec::with_capacity(list_val.len());
            for v in list_val {
                pts.push(vector::<3>(v)?);
            }
            pts
        } else if let Some(mesh_val) = a.get("points_mesh") {
            let Artifact::Mesh(pm) = self.workspace.get(mesh_val)? else {
                return Err("Expected mesh handle for 'points_mesh'".into());
            };
            pm.positions
                .iter()
                .map(|p| [p[0] as f64, p[1] as f64, p[2] as f64])
                .collect()
        } else {
            vec![[0.0, 0.0, 0.0], [1.5, 0.0, 0.0], [0.0, 1.5, 0.0]]
        };

        if points.is_empty() {
            return Err("At least one target point required".into());
        }

        // Parse scales
        let default_scale = number(a, "scale", 1.0)?;
        if !default_scale.is_finite() || default_scale <= 0.0 {
            return Err("scale must be a finite positive number".into());
        }

        let scales: Vec<[f64; 3]> = if let Some(scales_val) = a.get("scales") {
            let list_val = list(scales_val)?;
            let mut sc = Vec::with_capacity(points.len());
            for v in list_val {
                sc.push(vector::<3>(v)?);
            }
            while sc.len() < points.len() {
                sc.push([default_scale, default_scale, default_scale]);
            }
            sc
        } else {
            vec![[default_scale, default_scale, default_scale]; points.len()]
        };

        let instance_count = points.len();
        let total_v = instance_count * proto.positions.len();
        let total_t = instance_count * proto.triangles.len();

        if total_v > 200_000 || total_t > 400_000 {
            return Err("Mesh size exceeds limit".into());
        }

        self.charge(total_v * 4 + 1024, total_v * 16 + total_t * 12 + 2048)?;

        // Build preview merged mesh
        let mut preview_positions = Vec::with_capacity(total_v);
        let mut preview_triangles = Vec::with_capacity(total_t);

        let mut instance_records = Vec::with_capacity(instance_count);

        for (k, pt) in points.iter().enumerate() {
            let s = scales[k];
            let v_offset = preview_positions.len() as u32;

            for p in &proto.positions {
                preview_positions.push([
                    (p[0] as f64 * s[0] + pt[0]) as f32,
                    (p[1] as f64 * s[1] + pt[1]) as f32,
                    (p[2] as f64 * s[2] + pt[2]) as f32,
                ]);
            }

            for tri in &proto.triangles {
                preview_triangles.push([
                    tri[0] + v_offset,
                    tri[1] + v_offset,
                    tri[2] + v_offset,
                ]);
            }

            let inst_rec = BTreeMap::from([
                (
                    "position".into(),
                    Value::List(vec![
                        Value::F64(pt[0]),
                        Value::F64(pt[1]),
                        Value::F64(pt[2]),
                    ]),
                ),
                (
                    "scale".into(),
                    Value::List(vec![Value::F64(s[0]), Value::F64(s[1]), Value::F64(s[2])]),
                ),
            ]);
            instance_records.push(Value::Record(inst_rec));
        }

        let preview_mesh_handle =
            self.store_mesh_positions(preview_positions, preview_triangles)?;

        Ok(Value::Record(BTreeMap::from([
            ("prototype".into(), proto_val.clone()),
            ("instance_count".into(), Value::U64(instance_count as u64)),
            ("instances".into(), Value::List(instance_records)),
            ("mesh".into(), preview_mesh_handle),
        ])))
    }

    // ── Morphism 5: Realize Instances ────────────────────────────────────────

    pub(super) fn manifold_realize_instances(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["instances"])?;

        let inst_arg = required(a, "instances")?;
        let inst_rec = record(inst_arg)?;

        let proto_handle = required(inst_rec, "prototype")?;
        let proto = {
            let Artifact::Mesh(m) = self.workspace.get(proto_handle)? else {
                return Err("Expected mesh handle for 'prototype'".into());
            };
            m.clone()
        };

        let mut instance_transforms: Vec<([f64; 3], [f64; 3])> = Vec::new();

        if let Some(list_val) = inst_rec.get("instances") {
            for entry in list(list_val)? {
                let rec = record(entry)?;
                let pos = vector::<3>(required(rec, "position")?)?;
                let sc = if let Some(sc_val) = rec.get("scale") {
                    vector::<3>(sc_val)?
                } else {
                    [1.0, 1.0, 1.0]
                };
                instance_transforms.push((pos, sc));
            }
        } else if let Some(pts_val) = inst_rec.get("points") {
            for entry in list(pts_val)? {
                let pos = vector::<3>(entry)?;
                instance_transforms.push((pos, [1.0, 1.0, 1.0]));
            }
        } else {
            return Err("Missing 'instances' or 'points' in instances record".into());
        }

        let total_v = instance_transforms.len() * proto.positions.len();
        let total_t = instance_transforms.len() * proto.triangles.len();

        if total_v > 200_000 || total_t > 400_000 {
            return Err("Mesh size exceeds limit".into());
        }

        self.charge(total_v * 8 + 2048, total_v * 24 + total_t * 12 + 4096)?;

        // Weld coincident vertices using spatial hashing (grid cell = 1e-4)
        let weld_grid = 1e4f64;
        let mut welded_positions: Vec<[f32; 3]> = Vec::new();
        let mut welded_triangles: Vec<[u32; 3]> = Vec::with_capacity(total_t);
        let mut vert_map: BTreeMap<(i64, i64, i64), u32> = BTreeMap::new();

        for (pt, sc) in &instance_transforms {
            let mut local_remap = Vec::with_capacity(proto.positions.len());

            for p in &proto.positions {
                let wx = (p[0] as f64 * sc[0] + pt[0]) as f32;
                let wy = (p[1] as f64 * sc[1] + pt[1]) as f32;
                let wz = (p[2] as f64 * sc[2] + pt[2]) as f32;

                let kx = (wx as f64 * weld_grid).round() as i64;
                let ky = (wy as f64 * weld_grid).round() as i64;
                let kz = (wz as f64 * weld_grid).round() as i64;

                let idx = *vert_map.entry((kx, ky, kz)).or_insert_with(|| {
                    let new_id = welded_positions.len() as u32;
                    welded_positions.push([wx, wy, wz]);
                    new_id
                });
                local_remap.push(idx);
            }

            for tri in &proto.triangles {
                let i0 = local_remap[tri[0] as usize];
                let i1 = local_remap[tri[1] as usize];
                let i2 = local_remap[tri[2] as usize];
                if i0 != i1 && i1 != i2 && i2 != i0 {
                    welded_triangles.push([i0, i1, i2]);
                }
            }
        }

        self.store_mesh_positions(welded_positions, welded_triangles)
    }
}

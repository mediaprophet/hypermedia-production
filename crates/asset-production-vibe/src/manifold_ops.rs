use super::*;
use std::collections::BTreeMap;
use qualia_core_db::hypermedia_authoring::Camera3D;
use qualia_core_db::container_10d::header::Container10dHeader;
use qualia_core_db::container_10d::section::{parse_section_table, SectionType};
use qualia_core_db::container_10d::surface_reading::decode_surface_reading;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, decode_10d_mesh, Compile10dExtras,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg, boolean_3::Boolean3Op, DecimateOptions, Point3,
};
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn manifold_ext(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.manifold.boolean_3d"
            | "manifold.boolean_3d"
            | "hypermedia.mesh.boolean_3d" => self.manifold_boolean_3d(a),
            "hypermedia.manifold.simplify"
            | "manifold.simplify"
            | "hypermedia.mesh.simplify" => self.manifold_simplify(a),
            "hypermedia.manifold.import_mesh"
            | "manifold.import_mesh"
            | "hypermedia.mesh.import" => self.manifold_import_mesh(a),
            "hypermedia.manifold.project_dxf"
            | "manifold.project_dxf"
            | "hypermedia.mesh.dxf" => self.manifold_project_dxf(a),
            "hypermedia.manifold.compile_10d_pbr"
            | "manifold.compile_10d_pbr"
            | "hypermedia.mesh.compile_10d_pbr" => self.manifold_compile_10d_pbr(a),
            "hypermedia.manifold.loft" | "manifold.loft" | "hypermedia.mesh.loft" => {
                self.manifold_loft(a)
            }
            _ => Err(format!("Unknown manifold operation: {id}")),
        }
    }

    fn manifold_boolean_3d(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["a", "b", "op"])?;
        let op_str = text(required(a, "op")?)?;
        let op = match op_str {
            "union" => Boolean3Op::Union,
            "intersection" => Boolean3Op::Intersection,
            "difference" => Boolean3Op::Difference,
            _ => {
                return Err(format!(
                    "Unknown boolean operation: {op_str}; expected 'union', 'intersection', or 'difference'"
                ))
            }
        };

        let (va, ta) = {
            let Artifact::Mesh(mesh_a) = self.workspace.get(required(a, "a")?)? else {
                return Err("Expected mesh handle for 'a'".into());
            };
            let va: Vec<Point3> = mesh_a
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let ta = mesh_a.triangles.clone();
            (va, ta)
        };

        let (vb, tb) = {
            let Artifact::Mesh(mesh_b) = self.workspace.get(required(a, "b")?)? else {
                return Err("Expected mesh handle for 'b'".into());
            };
            let vb: Vec<Point3> = mesh_b
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let tb = mesh_b.triangles.clone();
            (vb, tb)
        };

        // Production envelope matches mesh_budget (200k/400k), not the old 50k/100k authoring cap.
        let req_v = cg::required_vertices_3(va.len(), vb.len(), ta.len(), tb.len());
        let req_t = cg::required_triangles_3(ta.len(), tb.len());
        let max_v = req_v.min(200_000);
        let max_t = req_t.min(400_000);
        self.mesh_budget(max_v, max_t)?;

        let mut out_v = vec![Point3::default(); max_v];
        let mut out_t = vec![[0u32; 3]; max_t];

        let (nv, nt) = cg::boolean_3(
            &va,
            &ta,
            &vb,
            &tb,
            op,
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Boolean 3D failed: {e:?}"))?;

        out_v.truncate(nv);
        out_t.truncate(nt);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_simplify(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "target_triangles", "max_error"])?;
        let target_triangles = integer(required(a, "target_triangles")?, "target_triangles")?;
        if target_triangles < 4 || target_triangles > 200_000 {
            return Err("target_triangles outside 4..=200000 range".into());
        }
        let max_error = number(a, "max_error", 0.05)?;
        if !max_error.is_finite() || max_error < 1e-6 || max_error > 1e4 {
            return Err("max_error outside 1e-6..=1e4 range".into());
        }

        let (vertices, triangles) = {
            let Artifact::Mesh(mesh) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle".into());
            };
            let v: Vec<Point3> = mesh
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let t = mesh.triangles.clone();
            (v, t)
        };

        let nv = vertices.len();
        let nt = triangles.len();
        if nt < 4 || nv < 3 {
            return Err("Mesh has too few triangles or vertices for simplification".into());
        }

        self.charge((nv + nt) * 20, nv * 64 + nt * 32)?;

        let target_faces = target_triangles.min(nt);
        let mut out_v = vec![Point3::default(); nv];
        let mut out_t = vec![[0u32; 3]; nt];

        let report = cg::decimate_qem(
            &vertices,
            &triangles,
            DecimateOptions {
                target_faces,
                max_error: Some(max_error),
            },
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Simplify decimation failed: {e:?}"))?;

        out_v.truncate(report.vertices);
        out_t.truncate(report.faces);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_import_mesh(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["input", "format"])?;
        let key = text(required(a, "input")?)?;
        let explicit_format = if let Some(f_val) = a.get("format") {
            let f = text(f_val)?;
            match f {
                "obj" | "stl" | "glb" => Some(f.to_string()),
                _ => return Err(format!("Unsupported format: {f}; expected 'obj', 'stl', or 'glb'")),
            }
        } else {
            None
        };

        let (bytes, key_ext) = {
            let res = self
                .inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or_else(|| format!("Missing retained input: {key}"))?;
            let ext = std::path::Path::new(&res.key)
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_ascii_lowercase());
            (res.bytes.clone(), ext)
        };

        let n = bytes.len();
        self.charge(n * 4, n * 8)?;

        let hint = explicit_format.as_deref().or(key_ext.as_deref());
        let imported = qualia_core_db::render::assets::import_asset_with_normals(&bytes, hint)
            .map_err(|e| format!("Import mesh failed: {e}"))?;
        let mesh = imported.mesh;

        self.mesh_budget(mesh.positions.len(), mesh.triangles.len())?;
        self.workspace
            .insert(Artifact::Mesh(mesh), self.limits)
    }

    fn manifold_project_dxf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "mesh", "layer", "dxf", "input"])?;
        let layer = match a.get("layer") {
            Some(v) => text(v)?,
            None => "0",
        };

        if a.get("mesh").is_none() && a.get("profile").is_none() {
            if let Some(dxf_val) = a.get("dxf").or_else(|| a.get("input")) {
                return self.manifold_parse_dxf(dxf_val);
            }
        }

        let profile_opt = if let Some(pv) = a.get("profile") {
            let Artifact::Profile(pts) = self.workspace.get(pv)? else {
                return Err("Expected profile handle for 'profile'".into());
            };
            Some(pts.clone())
        } else {
            None
        };

        let mesh_opt = if let Some(mv) = a.get("mesh") {
            let Artifact::Mesh(m) = self.workspace.get(mv)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            Some(m.clone())
        } else {
            None
        };

        if profile_opt.is_none() && mesh_opt.is_none() {
            return Err("project_dxf requires at least one of 'profile' or 'mesh'".into());
        }

        let n_profile = profile_opt.as_ref().map(|p| p.len()).unwrap_or(0);
        let n_mesh = mesh_opt.as_ref().map(|m| m.triangles.len()).unwrap_or(0);
        let total_entities = n_profile + n_mesh;
        self.charge(total_entities * 10 + 1024, total_entities * 256 + 4096)?;

        use std::fmt::Write;
        let mut dxf = String::with_capacity(total_entities * 128 + 256);
        dxf.push_str("0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1015\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n");

        if let Some(profile) = profile_opt {
            dxf.push_str("0\nLWPOLYLINE\n8\n");
            dxf.push_str(layer);
            dxf.push_str("\n90\n");
            let _ = write!(&mut dxf, "{}\n", profile.len());
            dxf.push_str("70\n1\n");
            for p in &profile {
                dxf.push_str("10\n");
                let _ = write!(&mut dxf, "{}\n", p[0]);
                dxf.push_str("20\n");
                let _ = write!(&mut dxf, "{}\n", p[1]);
            }
        }

        if let Some(mesh) = mesh_opt {
            for tri in &mesh.triangles {
                let i0 = tri[0] as usize;
                let i1 = tri[1] as usize;
                let i2 = tri[2] as usize;
                if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
                    continue;
                }
                let p0 = mesh.positions[i0];
                let p1 = mesh.positions[i1];
                let p2 = mesh.positions[i2];

                dxf.push_str("0\n3DFACE\n8\n");
                dxf.push_str(layer);
                dxf.push('\n');

                dxf.push_str("10\n");
                let _ = write!(&mut dxf, "{}\n", p0[0]);
                dxf.push_str("20\n");
                let _ = write!(&mut dxf, "{}\n", p0[1]);
                dxf.push_str("30\n");
                let _ = write!(&mut dxf, "{}\n", p0[2]);

                dxf.push_str("11\n");
                let _ = write!(&mut dxf, "{}\n", p1[0]);
                dxf.push_str("21\n");
                let _ = write!(&mut dxf, "{}\n", p1[1]);
                dxf.push_str("31\n");
                let _ = write!(&mut dxf, "{}\n", p1[2]);

                dxf.push_str("12\n");
                let _ = write!(&mut dxf, "{}\n", p2[0]);
                dxf.push_str("22\n");
                let _ = write!(&mut dxf, "{}\n", p2[1]);
                dxf.push_str("32\n");
                let _ = write!(&mut dxf, "{}\n", p2[2]);

                // AutoCAD 3DFACE repeats vertex 3 as vertex 4 for triangles
                dxf.push_str("13\n");
                let _ = write!(&mut dxf, "{}\n", p2[0]);
                dxf.push_str("23\n");
                let _ = write!(&mut dxf, "{}\n", p2[1]);
                dxf.push_str("33\n");
                let _ = write!(&mut dxf, "{}\n", p2[2]);
                dxf.push('\n');
            }
        }

        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        self.encode("dxf", dxf.into_bytes())
    }

    fn manifold_parse_dxf(&mut self, dxf_val: &Value) -> Result<Value, String> {
        let resource = self.workspace.encoded(dxf_val)?;
        if resource.kind != "dxf" {
            return Err("Expected encoded 'dxf' resource handle".into());
        }
        let parsed = parse_ascii_dxf(&resource.bytes)?;
        self.charge(
            parsed.positions.len() * 8 + parsed.profile.len() * 4 + 256,
            parsed.positions.len() * 48 + parsed.triangles.len() * 12 + 1024,
        )?;

        let mut out = BTreeMap::new();
        out.insert(
            "lwpolyline_count".into(),
            Value::U64(parsed.lwpolyline_count as u64),
        );
        out.insert("face_count".into(), Value::U64(parsed.triangles.len() as u64));
        if !parsed.profile.is_empty() {
            out.insert("profile".into(), self.store_profile(parsed.profile)?);
        }
        if !parsed.positions.is_empty() && !parsed.triangles.is_empty() {
            let vertices: Vec<Point3> = parsed
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            out.insert("mesh".into(), self.store_mesh(vertices, parsed.triangles)?);
        }
        Ok(Value::Record(out))
    }

    fn manifold_loft(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["a", "b", "sections", "caps", "interpolation", "samples"])?;
        let interpolation = match a.get("interpolation") {
            Some(v) => text(v)?,
            None => "linear",
        };
        if interpolation != "linear" && interpolation != "cubic" {
            return Err(
                "Unknown interpolation: expected 'linear' (ruled) or 'cubic' (Catmull-Rom)"
                    .into(),
            );
        }
        let samples = count(a, "samples", 4, 16)?;
        let caps = boolean(a, "caps", true)?;

        let mut rings = if let Some(sections_val) = a.get("sections") {
            let ring_vals = list(sections_val)?;
            if ring_vals.len() < 2 || ring_vals.len() > 256 {
                return Err("Multi-section loft requires 2..=256 sections".into());
            }
            let mut rings = Vec::with_capacity(ring_vals.len());
            let mut ring_len = 0usize;
            for (idx, rv) in ring_vals.iter().enumerate() {
                let pts = loft_points3(rv)?;
                if idx == 0 {
                    ring_len = pts.len();
                } else if pts.len() != ring_len {
                    return Err("All loft sections must have identical point counts".into());
                }
                rings.push(pts);
            }
            rings
        } else {
            let first = loft_points3(required(a, "a")?)?;
            let second = loft_points3(required(a, "b")?)?;
            if first.len() != second.len() {
                return Err("Loft curves need corresponding point counts".into());
            }
            vec![first, second]
        };

        if interpolation == "cubic" && rings.len() >= 3 {
            rings = densify_loft_rings_cubic(&rings, samples);
        }

        let ring_len = rings[0].len();
        let n_sec = rings.len();
        let nv = n_sec * ring_len + if caps { 2 } else { 0 };
        let nt = (n_sec - 1) * ring_len * 2 + if caps { 2 * ring_len } else { 0 };
        self.mesh_budget(nv, nt)?;

        let mut vertices = Vec::with_capacity(nv);
        let mut triangles = Vec::with_capacity(nt);
        for ring in &rings {
            vertices.extend_from_slice(ring);
        }
        for s in 0..(n_sec - 1) {
            let base_a = (s * ring_len) as u32;
            let base_b = ((s + 1) * ring_len) as u32;
            for i in 0..ring_len {
                let next_i = (i + 1) % ring_len;
                let i00 = base_a + i as u32;
                let i01 = base_a + next_i as u32;
                let i10 = base_b + i as u32;
                let i11 = base_b + next_i as u32;
                triangles.push([i00, i10, i11]);
                triangles.push([i00, i11, i01]);
            }
        }
        if caps {
            for (ring_idx, flip) in [(0usize, true), (n_sec - 1, false)] {
                let ring = &rings[ring_idx];
                let mut cx = 0.0;
                let mut cy = 0.0;
                let mut cz = 0.0;
                for p in ring {
                    cx += p.x;
                    cy += p.y;
                    cz += p.z;
                }
                let inv = 1.0 / (ring_len as f64);
                let c_idx = vertices.len() as u32;
                vertices.push(Point3::new(cx * inv, cy * inv, cz * inv));
                let base = (ring_idx * ring_len) as u32;
                for i in 0..ring_len {
                    let next_i = (i + 1) % ring_len;
                    if flip {
                        triangles.push([c_idx, base + next_i as u32, base + i as u32]);
                    } else {
                        triangles.push([c_idx, base + i as u32, base + next_i as u32]);
                    }
                }
            }
        }
        self.store_mesh(vertices, triangles)
    }

    fn manifold_compile_10d_pbr(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "albedo",
                "source_10d",
                "topology",
                "spatial_index",
                "camera_position",
                "camera_target",
                "camera_up",
                "camera_fov",
                "camera_near",
            ],
        )?;
        let topology = boolean(a, "topology", true)?;
        let spatial_index = boolean(a, "spatial_index", true)?;

        let mesh = {
            let Artifact::Mesh(m) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            m.clone()
        };

        let albedo = if let Some(av) = a.get("albedo") {
            let Artifact::Rgba(rgba) = self.workspace.get(av)? else {
                return Err("Expected rgba handle for 'albedo'".into());
            };
            Some(rgba.clone())
        } else {
            None
        };

        let source_10d_colors = if let Some(sv) = a.get("source_10d") {
            let res = self.workspace.encoded(sv)?;
            decode_source_10d_vertex_colors(&res.bytes)
        } else {
            None
        };

        let n = mesh.positions.len() + mesh.triangles.len();
        self.charge(n * 50 + 2048, n * 256 + 16384)?;

        // Compute smooth area-weighted per-vertex unit normals of length mesh.vertex_count()
        let mut normals = vec![[0.0f32; 3]; mesh.positions.len()];
        for tri in &mesh.triangles {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
                continue;
            }
            let p0 = mesh.positions[i0];
            let p1 = mesh.positions[i1];
            let p2 = mesh.positions[i2];
            let e1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let cross = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];
            for &idx in &[i0, i1, i2] {
                normals[idx][0] += cross[0];
                normals[idx][1] += cross[1];
                normals[idx][2] += cross[2];
            }
        }
        for n in &mut normals {
            let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
            if len_sq > 1e-12 {
                let inv = 1.0 / len_sq.sqrt();
                n[0] *= inv;
                n[1] *= inv;
                n[2] *= inv;
            } else {
                *n = [0.0, 1.0, 0.0];
            }
        }

        // Sample albedo pixels or transfer nearest-vertex colors from source_10d.
        // Godot Camera3D look-at projects the albedo when camera_position is set;
        // spherical UV remains the fallback for backfaces, off-frame, and keyed black.
        let surface_reading: Option<Vec<[f32; 4]>> = if let Some(rgba) = albedo {
            if rgba.width == 0 || rgba.height == 0 || rgba.pixels.is_empty() {
                return Err("Albedo image has empty dimensions".into());
            }
            let centroid = mesh.centroid();
            let camera_eye = match a.get("camera_position") {
                Some(v) => Some(vector::<3>(v)?),
                None => None,
            };
            let camera_target = match a.get("camera_target") {
                Some(v) => vector::<3>(v)?,
                None => [centroid[0] as f64, centroid[1] as f64, centroid[2] as f64],
            };
            let camera_up = match a.get("camera_up") {
                Some(v) => vector::<3>(v)?,
                None => [0.0, 1.0, 0.0],
            };
            let camera_fov = number(a, "camera_fov", 45.0)?;
            let camera_near = number(a, "camera_near", 0.1)?;
            let mut readings = Vec::with_capacity(mesh.positions.len());
            let pi = std::f64::consts::PI;
            let two_pi = 2.0 * pi;
            let max_x = (rgba.width - 1) as f64;
            let max_y = (rgba.height - 1) as f64;
            let vw = rgba.width as f64;
            let vh = rgba.height as f64;

            let sample_uv = |u: f64, v: f64| -> [f32; 4] {
                let px = ((u * max_x).round() as usize).min(rgba.width as usize - 1);
                let py = ((v * max_y).round() as usize).min(rgba.height as usize - 1);
                let idx = py * rgba.width as usize + px;
                let pix = rgba.pixels.get(idx).copied().unwrap_or([255, 255, 255, 255]);
                [
                    pix[0] as f32 / 255.0,
                    pix[1] as f32 / 255.0,
                    pix[2] as f32 / 255.0,
                    pix[3] as f32 / 255.0,
                ]
            };

            for (i, p) in mesh.positions.iter().enumerate() {
                let dx = (p[0] - centroid[0]) as f64;
                let dy = (p[1] - centroid[1]) as f64;
                let dz = (p[2] - centroid[2]) as f64;
                let r = (dx * dx + dy * dy + dz * dz).sqrt();
                let spherical = if r < 1e-9 {
                    sample_uv(0.5, 0.5)
                } else {
                    let phi = dz.atan2(dx);
                    let u = (phi / two_pi + 0.5).clamp(0.0, 1.0);
                    let lat = (dy / r).clamp(-1.0, 1.0).asin();
                    let v = (lat / pi + 0.5).clamp(0.0, 1.0);
                    sample_uv(u, v)
                };

                let projected = if let Some(eye) = camera_eye {
                    let world = [p[0] as f64, p[1] as f64, p[2] as f64];
                    let n = normals[i];
                    let view = [eye[0] - world[0], eye[1] - world[1], eye[2] - world[2]];
                    let facing = n[0] as f64 * view[0] + n[1] as f64 * view[1] + n[2] as f64 * view[2];
                    if facing > 0.0 {
                        Camera3D::unproject_position_look_at(
                            eye,
                            camera_target,
                            camera_up,
                            camera_fov,
                            world,
                            (vw, vh),
                            camera_near,
                        )
                        .and_then(|px| {
                            if px[0] < 0.0 || px[1] < 0.0 || px[0] >= vw || px[1] >= vh {
                                return None;
                            }
                            let color = sample_uv(px[0] / vw, px[1] / vh);
                            // Key near-black studio backdrop so it does not stain the mesh.
                            if color[0].max(color[1]).max(color[2]) < 0.05 {
                                None
                            } else {
                                Some(color)
                            }
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };

                readings.push(projected.unwrap_or(spherical));
            }
            Some(readings)
        } else if let Some((ref_pos, ref_cols)) = source_10d_colors {
            Some(transfer_nearest_vertex_colors(
                &mesh.positions,
                &ref_pos,
                &ref_cols,
            ))
        } else {
            None
        };

        let bytes = match compile_mesh_to_10d_with_extras_and_frames(
            &mesh,
            &[],
            None,
            Compile10dExtras {
                topology,
                spatial_index,
            },
            surface_reading.as_deref(),
            Some(&normals),
            None,
        ) {
            Ok(b) => b,
            Err(_) if topology => compile_mesh_to_10d_with_extras_and_frames(
                &mesh,
                &[],
                None,
                Compile10dExtras {
                    topology: false,
                    spatial_index,
                },
                surface_reading.as_deref(),
                Some(&normals),
                None,
            )
            .map_err(|e| format!("10D PBR compile failed: {e:?}"))?,
            Err(e) => return Err(format!("10D PBR compile failed: {e:?}")),
        };

        self.encode("10d", bytes)
    }
}

fn decode_source_10d_vertex_colors(bytes: &[u8]) -> Option<(Vec<[f32; 3]>, Vec<[f32; 4]>)> {
    let header = Container10dHeader::parse(bytes).ok()?;
    let descs = parse_section_table(bytes, &header).ok()?;
    let ref_mesh = decode_10d_mesh(bytes).ok()?;
    for d in descs {
        if d.typ() == Some(SectionType::SurfaceReading) {
            let start = d.byte_offset as usize;
            let end = start.saturating_add(d.byte_length as usize);
            let slice = bytes.get(start..end)?;
            let cols = decode_surface_reading(slice)?;
            if cols.len() == ref_mesh.positions.len() && !cols.is_empty() {
                return Some((ref_mesh.positions, cols));
            }
        }
    }
    None
}

fn transfer_nearest_vertex_colors(
    target_positions: &[[f32; 3]],
    ref_positions: &[[f32; 3]],
    ref_colors: &[[f32; 4]],
) -> Vec<[f32; 4]> {
    let mut out = Vec::with_capacity(target_positions.len());
    for p in target_positions {
        let mut best_d2 = f32::INFINITY;
        let mut best_col = [0.65, 0.72, 0.68, 1.0];
        for (rp, rc) in ref_positions.iter().zip(ref_colors.iter()) {
            let dx = p[0] - rp[0];
            let dy = p[1] - rp[1];
            let dz = p[2] - rp[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            if d2 < best_d2 {
                best_d2 = d2;
                best_col = *rc;
            }
        }
        out.push(best_col);
    }
    out
}

fn loft_points3(v: &Value) -> Result<Vec<Point3>, String> {
    let values = list(v)?;
    if values.len() < 3 || values.len() > 4096 {
        return Err("Loft section point count outside 3..=4096".into());
    }
    values
        .iter()
        .map(|item| {
            let p = vector::<3>(item)?;
            if p.iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
                return Err("Coordinate outside +/-1000000 range".into());
            }
            Ok(Point3::new(p[0], p[1], p[2]))
        })
        .collect()
}

fn catmull_rom_scalar(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

fn catmull_rom_point3(p0: Point3, p1: Point3, p2: Point3, p3: Point3, t: f64) -> Point3 {
    Point3::new(
        catmull_rom_scalar(p0.x, p1.x, p2.x, p3.x, t),
        catmull_rom_scalar(p0.y, p1.y, p2.y, p3.y, t),
        catmull_rom_scalar(p0.z, p1.z, p2.z, p3.z, t),
    )
}

/// Cubic Catmull-Rom densify of loft stations. Linear/ruled skinning is applied after.
fn densify_loft_rings_cubic(rings: &[Vec<Point3>], samples: usize) -> Vec<Vec<Point3>> {
    let n = rings.len();
    let ring_len = rings[0].len();
    let steps = samples.max(1);
    let mut out = Vec::with_capacity((n - 1) * steps + 1);
    out.push(rings[0].clone());
    for seg in 0..(n - 1) {
        let i0 = if seg == 0 { 0 } else { seg - 1 };
        let i1 = seg;
        let i2 = seg + 1;
        let i3 = (seg + 2).min(n - 1);
        for s in 1..=steps {
            if s == steps {
                out.push(rings[i2].clone());
                continue;
            }
            let t = s as f64 / steps as f64;
            let mut ring = Vec::with_capacity(ring_len);
            for k in 0..ring_len {
                ring.push(catmull_rom_point3(
                    rings[i0][k],
                    rings[i1][k],
                    rings[i2][k],
                    rings[i3][k],
                    t,
                ));
            }
            out.push(ring);
        }
    }
    out
}

struct ParsedDxf {
    profile: Vec<[f64; 2]>,
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
    lwpolyline_count: usize,
}

fn parse_ascii_dxf(bytes: &[u8]) -> Result<ParsedDxf, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "DXF is not valid UTF-8")?;
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.len() < 4 {
        return Err("DXF payload too short".into());
    }
    let mut weld: BTreeMap<(i64, i64, i64), u32> = BTreeMap::new();
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    let mut profile: Vec<[f64; 2]> = Vec::new();
    let mut lwpolyline_count = 0usize;

    let weld_vertex = |weld: &mut BTreeMap<(i64, i64, i64), u32>,
                       positions: &mut Vec<[f32; 3]>,
                       p: [f32; 3]|
     -> u32 {
        let key = (
            (p[0] as f64 * 1e5).round() as i64,
            (p[1] as f64 * 1e5).round() as i64,
            (p[2] as f64 * 1e5).round() as i64,
        );
        if let Some(&idx) = weld.get(&key) {
            idx
        } else {
            let idx = positions.len() as u32;
            weld.insert(key, idx);
            positions.push(p);
            idx
        }
    };

    let mut i = 0usize;
    let mut entity = "";
    let mut face = [[0.0f32; 3]; 4];
    let mut poly: Vec<[f64; 2]> = Vec::new();
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut z = 0.0f64;
    while i + 1 < lines.len() {
        let code: i32 = lines[i].parse().unwrap_or(i32::MIN);
        let val = lines[i + 1];
        i += 2;
        if code == i32::MIN {
            continue;
        }
        match code {
            0 => {
                if entity == "3DFACE" {
                    let i0 = weld_vertex(&mut weld, &mut positions, face[0]);
                    let i1 = weld_vertex(&mut weld, &mut positions, face[1]);
                    let i2 = weld_vertex(&mut weld, &mut positions, face[2]);
                    let i3 = weld_vertex(&mut weld, &mut positions, face[3]);
                    if i0 != i1 && i1 != i2 && i2 != i0 {
                        triangles.push([i0, i1, i2]);
                    }
                    if i3 != i2 && i3 != i0 && i0 != i2 {
                        triangles.push([i0, i2, i3]);
                    }
                    face = [[0.0; 3]; 4];
                }
                if entity == "LWPOLYLINE" && !poly.is_empty() {
                    if profile.is_empty() {
                        profile = poly.clone();
                    }
                    lwpolyline_count += 1;
                    poly.clear();
                }
                entity = val;
            }
            10 => x = val.parse().unwrap_or(0.0),
            20 => {
                y = val.parse().unwrap_or(0.0);
                if entity == "LWPOLYLINE" {
                    poly.push([x, y]);
                }
            }
            30 => z = val.parse().unwrap_or(0.0),
            11 | 12 | 13 | 21 | 22 | 23 | 31 | 32 | 33 => {
                let n = val.parse().unwrap_or(0.0) as f32;
                match code {
                    11 => face[1][0] = n,
                    21 => face[1][1] = n,
                    31 => face[1][2] = n,
                    12 => face[2][0] = n,
                    22 => face[2][1] = n,
                    32 => face[2][2] = n,
                    13 => face[3][0] = n,
                    23 => face[3][1] = n,
                    33 => face[3][2] = n,
                    _ => {}
                }
            }
            _ => {}
        }
        if entity == "3DFACE" {
            match code {
                10 => face[0][0] = x as f32,
                20 => face[0][1] = y as f32,
                30 => face[0][2] = z as f32,
                _ => {}
            }
        }
    }
    if entity == "3DFACE" {
        let i0 = weld_vertex(&mut weld, &mut positions, face[0]);
        let i1 = weld_vertex(&mut weld, &mut positions, face[1]);
        let i2 = weld_vertex(&mut weld, &mut positions, face[2]);
        let i3 = weld_vertex(&mut weld, &mut positions, face[3]);
        if i0 != i1 && i1 != i2 && i2 != i0 {
            triangles.push([i0, i1, i2]);
        }
        if i3 != i2 && i3 != i0 && i0 != i2 {
            triangles.push([i0, i2, i3]);
        }
    }
    if entity == "LWPOLYLINE" && !poly.is_empty() {
        if profile.is_empty() {
            profile = poly;
        }
        lwpolyline_count += 1;
    }
    if positions.is_empty() && profile.is_empty() {
        return Err("DXF contained no LWPOLYLINE or 3DFACE entities".into());
    }
    Ok(ParsedDxf {
        profile,
        positions,
        triangles,
        lwpolyline_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use vibe::Value;

    fn make_test_host<'a>(
        limits: &'a ToolLimits,
        cancel: &'a AtomicBool,
        inputs: &'a [Resource],
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn vec3(x: f64, y: f64, z: f64) -> Value {
        Value::List(vec![Value::F64(x), Value::F64(y), Value::F64(z)])
    }

    fn unit_cube_mesh(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];
        let triangles = vec![
            [0, 3, 2],
            [0, 2, 1],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
        ];
        let vertices: Vec<Point3> = positions
            .iter()
            .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        host.store_mesh(vertices, triangles).expect("store cube")
    }

    fn translated_cube(host: &mut AssetHost<'_>, dx: f64, dy: f64, dz: f64) -> Value {
        let positions = vec![
            [-1.0 + dx, -1.0 + dy, -1.0 + dz],
            [1.0 + dx, -1.0 + dy, -1.0 + dz],
            [1.0 + dx, 1.0 + dy, -1.0 + dz],
            [-1.0 + dx, 1.0 + dy, -1.0 + dz],
            [-1.0 + dx, -1.0 + dy, 1.0 + dz],
            [1.0 + dx, -1.0 + dy, 1.0 + dz],
            [1.0 + dx, 1.0 + dy, 1.0 + dz],
            [-1.0 + dx, 1.0 + dy, 1.0 + dz],
        ];
        let triangles = vec![
            [0, 3, 2],
            [0, 2, 1],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
        ];
        let vertices: Vec<Point3> = positions
            .iter()
            .map(|p| Point3::new(p[0], p[1], p[2]))
            .collect();
        host.store_mesh(vertices, triangles).expect("store translated cube")
    }

    fn square_ring(z: f64, s: f64) -> Value {
        Value::List(vec![
            vec3(-s, -s, z),
            vec3(s, -s, z),
            vec3(s, s, z),
            vec3(-s, s, z),
        ])
    }

    #[test]
    fn test_manifold_loft_linear_remains_ruled_cubic_bends() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut linear_args = BTreeMap::new();
        linear_args.insert(
            "sections".into(),
            Value::List(vec![
                square_ring(0.0, 1.0),
                square_ring(1.0, 2.0),
                square_ring(2.0, 1.0),
            ]),
        );
        linear_args.insert("caps".into(), Value::Bool(false));
        linear_args.insert("interpolation".into(), Value::String("linear".into()));

        let linear_h = host
            .manifold_ext("hypermedia.manifold.loft", &linear_args)
            .expect("linear loft");
        let (linear_n, linear_t, mid) = {
            let Artifact::Mesh(linear) = host.workspace.get(&linear_h).expect("linear mesh") else {
                panic!("expected mesh");
            };
            // 3 rings * 4 verts, 2 spans * 4 quads * 2 tris
            (linear.positions.len(), linear.triangles.len(), linear.positions[4])
        };
        assert_eq!(linear_n, 12);
        assert_eq!(linear_t, 16);
        // Ruled: first vertex of the middle ring is the middle station (-2, -2, 1).
        assert!((mid[0] + 2.0).abs() < 1e-5 && (mid[2] - 1.0).abs() < 1e-5);

        let origin_linear = host
            .invoke(
                "hypermedia.manifold.loft",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert("a".into(), square_ring(0.0, 1.0));
                    a.insert("b".into(), square_ring(1.0, 1.0));
                    a
                }),
            )
            .expect("origin loft");
        {
            let Artifact::Mesh(origin) = host.workspace.get(&origin_linear).expect("origin mesh")
            else {
                panic!("expected mesh");
            };
            assert!(origin.triangles.len() >= 4);
        }

        let mut cubic_args = linear_args.clone();
        cubic_args.insert("interpolation".into(), Value::String("cubic".into()));
        cubic_args.insert("samples".into(), Value::U64(4));
        let cubic_h = host
            .manifold_ext("hypermedia.manifold.loft", &cubic_args)
            .expect("cubic loft");
        let Artifact::Mesh(cubic) = host.workspace.get(&cubic_h).expect("cubic mesh") else {
            panic!("expected mesh");
        };
        assert!(cubic.positions.len() > linear_n);
        // First densified ring after station 0 is Catmull-Rom, not the linear chord of size 1.25.
        let densified = cubic.positions[4];
        let linear_chord = 1.0 + 0.25 * (2.0 - 1.0);
        assert!(
            (densified[0].abs() - linear_chord as f32).abs() > 1e-4,
            "cubic loft must leave the ruled chord, got {}",
            densified[0]
        );
        for p in &cubic.positions {
            assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
        }
    }

    #[test]
    fn test_manifold_boolean_3d_production_union_difference() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let a = unit_cube_mesh(&mut host);
        let b = translated_cube(&mut host, 0.6, 0.4, 0.5);

        let mut union_args = BTreeMap::new();
        union_args.insert("a".into(), a.clone());
        union_args.insert("b".into(), b.clone());
        union_args.insert("op".into(), Value::String("union".into()));
        let union_h = host
            .invoke("hypermedia.manifold.boolean_3d", &Value::Record(union_args))
            .expect("boolean union");
        {
            let Artifact::Mesh(union_mesh) = host.workspace.get(&union_h).expect("union mesh")
            else {
                panic!("expected mesh");
            };
            assert!(
                union_mesh.triangles.len() >= 12,
                "union must keep a solid, got {} tris",
                union_mesh.triangles.len()
            );
            for p in &union_mesh.positions {
                assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
            }
        }

        let mut diff_args = BTreeMap::new();
        diff_args.insert("a".into(), a);
        diff_args.insert("b".into(), b);
        diff_args.insert("op".into(), Value::String("difference".into()));
        let diff_h = host
            .invoke(
                "hypermedia.manifold.boolean_3d",
                &Value::Record(diff_args),
            )
            .expect("boolean difference");
        let Artifact::Mesh(diff_mesh) = host.workspace.get(&diff_h).expect("diff mesh") else {
            panic!("expected mesh");
        };
        assert!(!diff_mesh.triangles.is_empty());
        assert!(!diff_mesh.positions.is_empty());
    }

    #[test]
    fn test_manifold_project_dxf_mesh_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let cube = unit_cube_mesh(&mut host);
        let mut export_args = BTreeMap::new();
        export_args.insert("mesh".into(), cube);
        export_args.insert("layer".into(), Value::String("SOLID".into()));
        let dxf_h = host
            .invoke(
                "hypermedia.manifold.project_dxf",
                &Value::Record(export_args),
            )
            .expect("dxf export");
        {
            let encoded = host.workspace.encoded(&dxf_h).expect("dxf bytes");
            assert_eq!(encoded.kind, "dxf");
            let text = std::str::from_utf8(&encoded.bytes).unwrap();
            assert!(text.contains("3DFACE"));
            assert!(text.contains("AC1015"));
        }

        let mut import_args = BTreeMap::new();
        import_args.insert("dxf".into(), dxf_h);
        let parsed = host
            .manifold_ext("hypermedia.manifold.project_dxf", &import_args)
            .expect("dxf import");
        let rec = record(&parsed).expect("record");
        let face_count = integer(required(rec, "face_count").unwrap(), "face_count").unwrap();
        assert_eq!(face_count, 12);
        let mesh_h = rec.get("mesh").expect("mesh handle");
        let Artifact::Mesh(roundtrip) = host.workspace.get(mesh_h).expect("rt mesh") else {
            panic!("expected mesh");
        };
        assert_eq!(roundtrip.triangles.len(), 12);
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &roundtrip.positions {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        assert!((min[0] + 1.0).abs() < 1e-3 && (max[0] - 1.0).abs() < 1e-3);
        assert!((min[2] + 1.0).abs() < 1e-3 && (max[2] - 1.0).abs() < 1e-3);
    }
}

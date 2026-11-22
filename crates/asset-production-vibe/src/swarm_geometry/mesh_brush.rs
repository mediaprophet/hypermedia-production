//! Multilevel midpoint refinement, bounded brush strokes, and UV displacement baking.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use color_transforms::Transfer;
use qualia_core_db::render::mesh_tangent_frames::tangents_from_uvs_into;

use super::*;

#[derive(Clone, Copy)]
enum Stroke {
    Normal(f64),
    Clay(Point),
    Flatten {
        normal: Point,
        weight: f64,
    },
    Grab(Point),
    Pinch(f64),
    Crease {
        normal: Point,
        depth: f64,
        pinch: f64,
    },
}

impl AssetHost<'_> {
    pub(crate) fn sculpt_multires(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "levels",
                "strokes",
                "displacement_resolution",
                "displacement_range_m",
            ],
        )?;
        let handle = required(args, "mesh")?;
        let source = source_mesh(self, handle)?;
        let coarse = &source.surface.mesh;
        let n = coarse.positions.len();
        if !source.skin_weights.is_empty() || !source.morph_targets.is_empty() {
            return Err(
                "Midpoint sculpt requires geometry without a bound skin or morph stream".into(),
            );
        }
        let uv = source
            .surface
            .texture_coordinates_0
            .as_ref()
            .ok_or("Displacement baking requires vertex-aligned UV0")?;
        if uv.len() != n
            || uv.iter().flatten().any(|uv| !(0.0..=1.0).contains(uv))
            || !valid_optional_attributes(&source.uv1, n)
            || !valid_optional_attributes(&source.vertex_colors, n)
        {
            return Err("Invalid finite vertex-aligned UV or colour stream for sculpting".into());
        }
        let _ = material_slots(source)?;
        let levels = args
            .get("levels")
            .map(|v| integer(v, "levels"))
            .transpose()?
            .unwrap_or(1);
        if levels > 3 {
            return Err("Sculpt levels must be between zero and three".into());
        }
        let multiplier = 4usize.pow(levels as u32);
        let refined = coarse.triangles.len().saturating_mul(multiplier);
        let vertex_upper = n.saturating_add(refined - coarse.triangles.len());
        if refined > MAX_TRIANGLES || vertex_upper > MAX_VERTICES {
            return Err("Requested sculpt subdivision exceeds its bounded geometry profile".into());
        }
        let resolution = count(args, "displacement_resolution", 256, 1024)?;
        let range = number(args, "displacement_range_m", 1.0)?;
        if !(8..=1024).contains(&resolution) || !(1e-8..=100.0).contains(&range) {
            return Err("Vector displacement requires an 8–1024 square atlas and a finite positive metre range".into());
        }
        let authored = list(required(args, "strokes")?)?;
        if !(1..=64).contains(&authored.len()) {
            return Err("Sculpt requires one to 64 strokes".into());
        }
        let brushes = authored
            .iter()
            .map(decode_stroke)
            .collect::<Result<Vec<_>, _>>()?;
        let pixels = resolution * resolution;
        self.charge(
            refined
                .saturating_mul(192)
                .saturating_add(vertex_upper.saturating_mul(80 * brushes.len())),
            (vertex_upper * 320)
                .saturating_add(refined * 72)
                .saturating_add(pixels * 8),
        )?;
        let source = source_mesh(self, handle)?;
        let mut output = (*source).clone();
        for _ in 0..levels {
            subdivide(&mut output)?;
        }
        let base = output.surface.mesh.positions.clone();
        let triangles = &output.surface.mesh.triangles;
        let base_face_normals = triangles
            .iter()
            .map(|tri| face_normal(tri.map(|idx| point(base[idx as usize]))))
            .collect::<Result<Vec<_>, _>>()?;
        let mut locked = vec![false; base.len()];
        for (face, tri) in triangles.iter().enumerate() {
            if protected(&output, face) {
                for &idx in tri {
                    locked[idx as usize] = true;
                }
            }
        }
        let mut edited = base.clone();
        for (origin, radius, brush) in brushes {
            let normals = matches!(brush, Stroke::Normal(_))
                .then(|| vertex_normals(&edited, triangles))
                .transpose()?;
            for (i, slot) in edited.iter_mut().enumerate() {
                let position = point(*slot);
                let from_tip = sub(position, origin);
                let normalized_distance = length(from_tip) / radius;
                if normalized_distance >= 1.0 {
                    continue;
                }
                let blend = (1.0 - normalized_distance).powi(2) * (1.0 + 2.0 * normalized_distance);
                let displacement = match brush {
                    Stroke::Normal(amount) => mul(
                        normals.as_ref().expect("normal stroke has frames")[i],
                        amount,
                    ),
                    Stroke::Clay(delta) | Stroke::Grab(delta) => delta,
                    Stroke::Flatten { normal, weight } => {
                        mul(normal, -dot(normal, from_tip) * weight)
                    }
                    Stroke::Pinch(weight) => mul(from_tip, -weight),
                    Stroke::Crease {
                        normal,
                        depth,
                        pinch,
                    } => {
                        let tangent = sub(from_tip, mul(normal, dot(normal, from_tip)));
                        add(mul(tangent, -pinch), mul(normal, -depth))
                    }
                };
                let next = add(position, mul(displacement, blend)).map(|x| x as f32);
                if locked[i] && next != *slot {
                    return Err("Sculpt stroke would displace a vertex on a protected face".into());
                }
                if next.iter().any(|x| !x.is_finite() || x.abs() > 1e6) {
                    return Err("Sculpt moved a vertex outside the finite local frame".into());
                }
                *slot = next;
            }
        }
        let mut max_move: f64 = 0.0;
        let delta = edited
            .iter()
            .zip(&base)
            .map(|(&p, &q)| {
                let d = sub(point(p), point(q));
                max_move = max_move.max(length(d));
                d
            })
            .collect::<Vec<_>>();
        if max_move == 0.0 {
            return Err("Authored strokes leave the admitted mesh unchanged".into());
        }
        if delta.iter().flatten().any(|d| d.abs() > range) {
            return Err(format!(
                "Sculpt displacement exceeds the authored per-axis map range of {range} m"
            ));
        }
        for (tri, compare) in triangles.iter().zip(base_face_normals) {
            if dot(
                face_normal(tri.map(|idx| point(edited[idx as usize])))?,
                compare,
            ) <= 0.01
            {
                return Err("Sculpt would invert or collapse a refined triangle".into());
            }
        }
        let uv = output
            .surface
            .texture_coordinates_0
            .as_ref()
            .ok_or("Missing refined UV0")?;
        let spans = triangles
            .iter()
            .map(|tri| {
                let p = tri.map(|i| uv[i as usize].map(f64::from));
                let area = orient(p[0], p[1], p[2]);
                if area.abs() < 1e-14 {
                    return Err("Sculpt displacement UV0 contains a degenerate triangle".into());
                }
                let limits = [0, 1].map(|axis| {
                    let min = p.iter().map(|v| v[axis]).fold(f64::INFINITY, f64::min);
                    let max = p.iter().map(|v| v[axis]).fold(f64::NEG_INFINITY, f64::max);
                    let begin = (min * resolution as f64 - 0.5)
                        .ceil()
                        .clamp(0.0, resolution as f64) as usize;
                    let end = ((max * resolution as f64 - 0.5).floor() + 1.0)
                        .clamp(0.0, resolution as f64) as usize;
                    (begin, end)
                });
                Ok((p, area, limits))
            })
            .collect::<Result<Vec<_>, String>>()?;
        let raster_work = spans
            .iter()
            .fold(0usize, |work, (_, _, [(x0, x1), (y0, y1)])| {
                work.saturating_add(
                    x1.saturating_sub(*x0)
                        .saturating_mul(y1.saturating_sub(*y0)),
                )
            });
        self.charge(
            raster_work.saturating_mul(64),
            (vertex_upper * 320)
                .saturating_add(refined * 184)
                .saturating_add(pixels * 8),
        )?;
        let mut texels = vec![[32768, 32768, 32768, 0]; pixels];
        let mut covered = 0usize;
        for (&tri, (p, area, [(x0, x1), (y0, y1)])) in triangles.iter().zip(spans) {
            let attributes = tri.map(|vertex| delta[vertex as usize]);
            for y in y0..y1 {
                for x in x0..x1 {
                    let sample = [
                        (x as f64 + 0.5) / resolution as f64,
                        (y as f64 + 0.5) / resolution as f64,
                    ];
                    let w = [
                        orient(p[1], p[2], sample) / area,
                        orient(p[2], p[0], sample) / area,
                        orient(p[0], p[1], sample) / area,
                    ];
                    if w.iter().any(|&v| v < -1e-10) {
                        continue;
                    }
                    let value = [0, 1, 2].map(|i| {
                        (32768.0
                            + 32767.0
                                * (w[0] * attributes[0][i]
                                    + w[1] * attributes[1][i]
                                    + w[2] * attributes[2][i])
                                / range)
                            .round()
                            .clamp(0.0, 65535.0) as u16
                    });
                    let old = &mut texels[y * resolution + x];
                    if old[3] == u16::MAX {
                        if old[..3].iter().zip(value).any(|(&a, b)| a.abs_diff(b) > 1) {
                            return Err("Overlapping UV islands assign different vector displacement to the same texel".into());
                        }
                    } else {
                        *old = [value[0], value[1], value[2], u16::MAX];
                        covered += 1;
                    }
                }
            }
        }
        if covered == 0 {
            return Err("Displacement map resolution leaves every UV face unsampled".into());
        }
        let normals = vertex_normals(&edited, triangles)?
            .into_iter()
            .map(|n| n.map(|x| x as f32))
            .collect::<Vec<_>>();
        let indices = triangles.iter().flatten().copied().collect::<Vec<_>>();
        let mut tangents = vec![[0.0; 4]; edited.len()];
        let mut scratch = vec![[0.0; 6]; edited.len()];
        tangents_from_uvs_into(&edited, uv, &normals, &indices, &mut tangents, &mut scratch)
            .map_err(|error| format!("Sculpt tangent reconstruction: {error:?}"))?;
        let (min, max) = bounds(&edited);
        output.surface.mesh.positions = edited;
        output.surface.mesh.min = min;
        output.surface.mesh.max = max;
        output.surface.normals = Some(normals);
        output.surface.tangents = Some(tangents);
        let displacement_map16 = self.store_rgba16(
            resolution as u32,
            resolution as u32,
            texels,
            Some(Transfer::Linear),
        )?;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(output)))?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh),
            ("displacement_map16".into(), displacement_map16),
            ("displacement_range_m".into(), Value::F64(range)),
            ("max_vertex_displacement_m".into(), Value::F64(max_move)),
            ("covered_texels".into(), Value::U64(covered as u64)),
            ("levels".into(), Value::U64(levels as u64)),
        ])))
    }
}

fn valid_optional_attributes<const N: usize>(fields: &[[f32; N]], length: usize) -> bool {
    fields.is_empty() || (fields.len() == length && fields.iter().flatten().all(|v| v.is_finite()))
}

fn decode_stroke(value: &Value) -> Result<(Point, f64, Stroke), String> {
    let row = record(value)?;
    let origin = vector::<3>(required(row, "center_m")?)?;
    let radius = num(required(row, "radius_m")?)?;
    if origin.iter().any(|v| v.abs() > 1e6) || !(1e-7..=1e3).contains(&radius) {
        return Err("Sculpt stroke center or radius is outside the local geometry profile".into());
    }
    let normal = || normalized(vector(required(row, "normal")?)?);
    let within = |key: &str, min: f64, max: f64| -> Result<f64, String> {
        let value = num(required(row, key)?)?;
        if (min..=max).contains(&value) {
            Ok(value)
        } else {
            Err(format!("{key} outside admitted limits"))
        }
    };
    let brush = match text(required(row, "mode")?)? {
        "inflate" => {
            fields(row, &["mode", "center_m", "radius_m", "amount_m"])?;
            Stroke::Normal(within("amount_m", -radius, radius)?)
        }
        "clay" => {
            fields(row, &["mode", "center_m", "radius_m", "normal", "amount_m"])?;
            Stroke::Clay(mul(normal()?, within("amount_m", -radius, radius)?))
        }
        "flatten" => {
            fields(row, &["mode", "center_m", "radius_m", "normal", "weight"])?;
            Stroke::Flatten {
                normal: normal()?,
                weight: within("weight", 0.0, 1.0)?,
            }
        }
        "grab" => {
            fields(row, &["mode", "center_m", "radius_m", "delta_m"])?;
            let delta = vector(required(row, "delta_m")?)?;
            if length(delta) > radius {
                return Err("Grab may not exceed the authored stroke radius".into());
            }
            Stroke::Grab(delta)
        }
        "pinch" => {
            fields(row, &["mode", "center_m", "radius_m", "weight"])?;
            Stroke::Pinch(within("weight", 0.0, 0.95)?)
        }
        "crease" => {
            fields(
                row,
                &[
                    "mode", "center_m", "radius_m", "normal", "depth_m", "weight",
                ],
            )?;
            Stroke::Crease {
                normal: normal()?,
                depth: within("depth_m", 0.0, radius)?,
                pinch: within("weight", 0.0, 0.95)?,
            }
        }
        _ => return Err("Unsupported authored sculpt stroke mode".into()),
    };
    Ok((origin, radius, brush))
}

fn subdivide(output: &mut AttributedMesh) -> Result<(), String> {
    let triangles = std::mem::take(&mut output.surface.mesh.triangles);
    let face_masks = std::mem::take(&mut output.protected_faces);
    let mut edges = BTreeMap::new();
    output.surface.mesh.triangles.reserve(triangles.len() * 4);
    for (face, [a, b, c]) in triangles.into_iter().enumerate() {
        let ab = midpoint_index(output, &mut edges, sorted_edge(a, b))?;
        let bc = midpoint_index(output, &mut edges, sorted_edge(b, c))?;
        let ca = midpoint_index(output, &mut edges, sorted_edge(c, a))?;
        output
            .surface
            .mesh
            .triangles
            .extend([[a, ab, ca], [ab, b, bc], [ca, bc, c], [ab, bc, ca]]);
        if let Some(&mask) = face_masks.get(face) {
            output.protected_faces.extend([mask; 4]);
        }
    }
    for range in &mut output.surface.submeshes {
        range.first_index *= 4;
        range.index_count *= 4;
    }
    Ok(())
}

fn midpoint_index(
    output: &mut AttributedMesh,
    edges: &mut BTreeMap<Edge, u32>,
    key: Edge,
) -> Result<u32, String> {
    if let Some(&index) = edges.get(&key) {
        return Ok(index);
    }
    let (a, b) = (key.0 as usize, key.1 as usize);
    let positions = &mut output.surface.mesh.positions;
    let index = positions.len() as u32;
    append_midpoint(positions, a, b);
    append_midpoint(
        output
            .surface
            .texture_coordinates_0
            .as_mut()
            .ok_or("Missing subdivided UV0")?,
        a,
        b,
    );
    append_midpoint(&mut output.uv1, a, b);
    append_midpoint(&mut output.vertex_colors, a, b);
    edges.insert(key, index);
    Ok(index)
}

fn append_midpoint<const N: usize>(stream: &mut Vec<[f32; N]>, a: usize, b: usize) {
    if !stream.is_empty() {
        stream.push(std::array::from_fn(|i| {
            ((f64::from(stream[a][i]) + f64::from(stream[b][i])) * 0.5) as f32
        }));
    }
}

fn vertex_normals(positions: &[[f32; 3]], triangles: &[[u32; 3]]) -> Result<Vec<Point>, String> {
    let mut normals = vec![[0.0; 3]; positions.len()];
    for tri in triangles {
        let [a, b, c] = tri.map(|idx| point(positions[idx as usize]));
        let cross = cross(sub(b, a), sub(c, a));
        for &i in tri {
            normals[i as usize] = add(normals[i as usize], cross);
        }
    }
    normals.into_iter().map(normalized).collect()
}

fn orient(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn bounds(points: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    points.iter().fold(
        ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
        |(lo, hi), p| {
            (
                std::array::from_fn(|i| lo[i].min(p[i])),
                std::array::from_fn(|i| hi[i].max(p[i])),
            )
        },
    )
}

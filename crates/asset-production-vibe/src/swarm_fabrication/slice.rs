//! Geometry-only, fixed-height area masks for prototype layer inspection.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{
    bounded_number, bounds, check_closed_shell, fields, mesh_handle, point, required, text, Args,
    AssetHost, Value,
};
use chronicle_office_codecs::zip::ZipPackage;
use qualia_core_db::specialized_libs::computational_geometry::{
    segment_segment_intersect_2d, slice_mesh_with_plane, Point2, Point3, SegmentIntersection2d,
};
use serde_json::json;
use std::{
    collections::{BTreeMap, BTreeSet},
    mem::size_of,
};

const MAX_LAYERS: usize = 256;
const MAX_SAMPLE_SLOTS: usize = 8 * 1024 * 1024;
const MAX_SECTION_FACES: usize = 100_000;
const MAX_SECTION_SEGMENTS: usize = 4096;
const GRID_QUANTUM_M: f64 = 1e-9;

impl AssetHost<'_> {
    pub(crate) fn slice_and_toolpath(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["mesh", "process", "layer_height_mm", "pixel_pitch_mm"],
        )?;
        let process = text(required(args, "process")?)?;
        if process != "sla_geometry_masks" {
            return Err("Slice branch supports sla_geometry_masks".into());
        }
        let layer = bounded_number(args, "layer_height_mm", None, 0.01, 10.0)?;
        let pitch = bounded_number(args, "pixel_pitch_mm", None, 0.005, 10.0)?;
        let mesh = mesh_handle(self, required(args, "mesh")?)?;
        let mesh = &mesh.surface.mesh;
        if mesh.triangles.len() > MAX_SECTION_FACES {
            return Err(format!(
                "Layer-mask branch supports at most {MAX_SECTION_FACES} faces"
            ));
        }
        check_closed_shell(self, mesh)?;
        let (min, max) = bounds(&mesh.positions);
        let min = min.map(|v| f64::from(v) * 1000.0);
        let max = max.map(|v| f64::from(v) * 1000.0);
        let span = super::sub(max, min);
        if span.iter().any(|&v| v <= 0.0) {
            return Err("Layer masks require nonzero three-dimensional bounds".into());
        }
        let n = (span[2] / layer).ceil() as usize;
        let width = (span[0] / pitch).ceil() as usize + 2;
        let height = (span[1] / pitch).ceil() as usize + 2;
        let pixels = width
            .checked_mul(height)
            .and_then(|v| v.checked_mul(n))
            .filter(|&v| {
                v <= MAX_SAMPLE_SLOTS && n <= MAX_LAYERS && width <= 4096 && height <= 4096
            })
            .ok_or("Layer mask stack exceeds pixel, layer or axis budget")?;
        let origin = [min[0] - pitch, min[1] - pitch];
        self.charge(
            pixels * 16 + n * mesh.triangles.len() * 32,
            pixels * 4
                + 64 * n
                + mesh.triangles.len() * (size_of::<[Point3; 2]>() + 160)
                + mesh.positions.len() * size_of::<Point3>(),
        )?;

        let points = mesh
            .positions
            .iter()
            .map(|&v| point(v.map(f64::from)))
            .collect::<Vec<_>>();
        let mut vertex_levels = points.iter().map(|p| p.z).collect::<Vec<_>>();
        vertex_levels.sort_unstable_by(f64::total_cmp);
        vertex_levels.dedup();
        let mut segments = vec![[Point3::default(); 2]; mesh.triangles.len()];
        let mut image = vec![0; width * height];
        let mut archive = ZipPackage::new();
        let mut layer_meta = Vec::with_capacity(n);
        let mut total_material_voxel_mm3 = 0.0;
        for index in 0..n {
            self.charge(0, 0)?;
            let z0 = min[2] + index as f64 * layer;
            let z1 = (z0 + layer).min(max[2]);
            let sample_z_m = clear_sample_level(z0 / 1000.0, z1 / 1000.0, &vertex_levels)
                .ok_or_else(|| format!("Layer {index} has no numerically unambiguous sample near its midpoint; extent_mm=[{z0}, {z1}]"))?;
            let sample_z = sample_z_m * 1000.0;
            let count = slice_mesh_with_plane(
                &points,
                &mesh.triangles,
                Point3::new(0.0, 0.0, sample_z_m),
                Point3::new(0.0, 0.0, 1.0),
                &mut segments,
            )
            .map_err(|e| format!("Native plane slice at layer {index}: {e:?}"))?;
            if count > MAX_SECTION_SEGMENTS {
                return Err(format!("Layer {index} exceeds the simple-contour budget of {MAX_SECTION_SEGMENTS} section segments"));
            }
            self.charge(
                count * height * 8 + count * count * 4,
                count * 160 + height * 8,
            )?;
            let loops = qualify_closed_segments(&segments[..count])
                .map_err(|error| format!("Layer {index}: {error}"))?;
            let covered = rasterize(&segments[..count], origin, pitch, width, height, &mut image)
                .map_err(|error| format!("Layer {index}: {error}"))?;
            total_material_voxel_mm3 += covered as f64 * pitch * pitch * (z1 - z0);
            let path = format!("layers/{index:04}.png");
            archive.insert(
                path.clone(),
                encode_mask(&image, width as u32, height as u32)?,
            );
            layer_meta.push(json!({"path": path, "start_z_mm": z0, "end_z_mm": z1,
                "sample_z_mm": sample_z, "closed_contour_count": loops, "occupied_pixels": covered}));
        }
        if total_material_voxel_mm3 == 0.0 {
            return Err(
                "Source yielded no occupied layer-mask pixel centers at the chosen resolution"
                    .into(),
            );
        }
        let meta = json!({
            "schema": "qualia.geometry-layer-mask-stack/v1",
            "channel": "grayscale_8bit_inside_255", "containment_rule": "even_odd_scanline",
            "origin_xy_mm": origin, "row_axis": "positive_y", "column_axis": "positive_x",
            "width_px": width, "height_px": height, "pixel_pitch_mm": pitch, "nominal_layer_height_mm": layer,
            "layers": layer_meta, "sampled_volume_mm3": total_material_voxel_mm3,
        });
        let archive_meta = serde_json::to_vec(&meta).map_err(|e| e.to_string())?;
        archive.insert("layer-stack.json", archive_meta.clone());
        let bytes = archive.write_to_bytes();
        let masks = self.admit_encoded("zip", bytes)?;
        let receipt = self.admit_encoded("json", archive_meta)?;
        Ok(Value::Record(
            [
                ("masks".into(), masks),
                ("receipt".into(), receipt),
                ("layer_count".into(), Value::U64(n as u64)),
            ]
            .into(),
        ))
    }
}

fn clear_sample_level(start: f64, end: f64, vertex_levels: &[f64]) -> Option<f64> {
    let midpoint = start.midpoint(end);
    let deltas = std::iter::once(0).chain((1..=16).flat_map(|step| [step, -step]));
    deltas.into_iter().find_map(|step| {
        let sample = midpoint + f64::from(step) * 4.0 * GRID_QUANTUM_M;
        let blocking = vertex_levels.partition_point(|&z| z < sample - GRID_QUANTUM_M);
        (sample > start
            && sample < end
            && vertex_levels
                .get(blocking)
                .is_none_or(|&z| z > sample + GRID_QUANTUM_M))
        .then_some(sample)
    })
}

fn qualify_closed_segments(segments: &[[Point3; 2]]) -> Result<usize, String> {
    let vertex = |v: Point3| {
        [
            (v.x / GRID_QUANTUM_M).round() as i64,
            (v.y / GRID_QUANTUM_M).round() as i64,
        ]
    };
    let mut graph = BTreeMap::<[i64; 2], Vec<[i64; 2]>>::new();
    let mut edges = BTreeSet::new();
    for &[p, q] in segments {
        let a = vertex(p);
        let b = vertex(q);
        if a == b || !edges.insert((a.min(b), a.max(b))) {
            return Err("section contains a collapsed or repeated segment".into());
        }
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }
    if let Some((vertex, neighbors)) = graph.iter().find(|(_, neighbors)| neighbors.len() != 2) {
        return Err(format!(
            "section does not form closed contours at snapped XY={vertex:?} nm, degree={}",
            neighbors.len()
        ));
    }
    let edges = edges.into_iter().collect::<Vec<_>>();
    let projected = |[x, y]: [i64; 2]| Point2::new(x as f64, y as f64);
    for (i, &(a, b)) in edges.iter().enumerate() {
        for &(c, d) in edges.iter().skip(i + 1) {
            if c[0] > a[0].max(b[0]) {
                break;
            }
            let intersection = segment_segment_intersect_2d(
                projected(a),
                projected(b),
                projected(c),
                projected(d),
            );
            let shared_endpoint = a == c || a == d || b == c || b == d;
            if intersection == SegmentIntersection2d::Overlap
                || (!shared_endpoint && intersection != SegmentIntersection2d::Disjoint)
            {
                return Err(format!(
                    "section boundaries intersect at segments ({a:?}, {b:?}) and ({c:?}, {d:?})"
                ));
            }
        }
    }
    let mut used = BTreeSet::new();
    let mut count = 0;
    for &start in graph.keys() {
        if used.contains(&start) {
            continue;
        }
        let mut previous = start;
        let mut current = graph[&start][0];
        used.insert(start);
        while current != start {
            if !used.insert(current) {
                return Err("section cycles revisit another contour".into());
            }
            let adjacent = &graph[&current];
            let next = if adjacent[0] == previous {
                adjacent[1]
            } else {
                adjacent[0]
            };
            previous = current;
            current = next;
        }
        count += 1;
    }
    Ok(count)
}

fn rasterize(
    segments: &[[Point3; 2]],
    origin: [f64; 2],
    spacing: f64,
    width: usize,
    height: usize,
    image: &mut [u8],
) -> Result<usize, String> {
    image.fill(0);
    let mut crossings = Vec::with_capacity(segments.len());
    let mut area = 0;
    for row in 0..height {
        let y = origin[1] + (row as f64 + 0.5) * spacing;
        crossings.clear();
        for &[a, b] in segments {
            // Use snapped endpoints for the same incidence semantics as contour qualification.
            let snap_mm = |x: f64| (x / GRID_QUANTUM_M).round() * GRID_QUANTUM_M * 1000.0;
            let [ay, by] = [snap_mm(a.y), snap_mm(b.y)];
            if (ay <= y && y < by) || (by <= y && y < ay) {
                let u = (y - ay) / (by - ay);
                crossings.push((1.0 - u) * snap_mm(a.x) + u * snap_mm(b.x));
            }
        }
        crossings.sort_unstable_by(f64::total_cmp);
        if crossings.len() % 2 != 0 {
            return Err(format!("scanline {row} has odd parity"));
        }
        for pair in crossings.chunks_exact(2) {
            let slot = |x: f64| {
                (((x - origin[0]) / spacing - 0.5_f64).ceil() as isize).clamp(0, width as isize)
                    as usize
            };
            let start = slot(pair[0]);
            let end = slot(pair[1]);
            image[row * width + start..row * width + end].fill(255);
            area += end - start;
        }
    }
    Ok(area)
}

fn encode_mask(values: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut output, width, height);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        encoder
            .write_header()
            .map_err(|e| e.to_string())?
            .write_image_data(values)
            .map_err(|e| e.to_string())?;
    }
    Ok(output)
}

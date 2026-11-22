//! Offline reconstruction of bounded, normalized civilian FFF deposition fragments.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, *,
};
use qualia_core_db::{
    container_10d::{self as ten, sdf_brick_section as sdf},
    q_hash,
    render::{
        assets::Mesh, compile_10d::upgrade_legacy_10d_to_holographic_v2,
        material_compile::compile_mesh_to_10d_with_materials,
    },
    specialized_libs::computational_geometry::{
        Point3,
        indexed_tetra_surface::{TetraEdgeSlot, indexed_tetra_surface_capped_into},
    },
};
use sha2::{Digest, Sha256};
use std::mem::size_of;

const MAX_VOXELS: usize = 600_000;
const MAX_VERTICES: usize = 100_000;
const MAX_TRIANGLES: usize = 200_000;
const EDGE_SLOTS: usize = 1 << 18;
type Edge = ([f64; 3], [f64; 3]);

impl AssetHost<'_> {
    pub(crate) fn reconstruct_toolpath_to_solid(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mode",
                "source",
                "entity_iri",
                "bead_width_mm",
                "voxel_spacing_mm",
            ],
        )?;
        if text(required(a, "mode")?)? != "fff_circular_bead" {
            return Err(
                "Expected fff_circular_bead with a normalized G-code deposition fragment".into(),
            );
        }
        let source = required(a, "source")?;
        let resource = encoded_resource(self, source)?;
        if resource.kind != "gcode" || !(1..=128 * 1024).contains(&resource.bytes.len()) {
            return Err("Expected a retained G-code fragment of at most 128 KiB".into());
        }
        let width = num(required(a, "bead_width_mm")?)?;
        let step = num(required(a, "voxel_spacing_mm")?)?;
        if !(0.05..=5.0).contains(&width) || !(width / 12.0..=width / 3.0).contains(&step) {
            return Err(
                "Bead width is out of range or the voxel pitch does not resolve the bead".into(),
            );
        }
        let entity = text(required(a, "entity_iri")?)?;
        if entity.len() > 512
            || entity
                .chars()
                .any(|ch| ch.is_control() || ch.is_whitespace())
            || !entity.contains(':')
        {
            return Err("A bounded, absolute entity IRI is required".into());
        }
        let wall = format!("{entity}:deposition-surface");
        let void = format!("{entity}:unused-void-role");
        let text_source =
            std::str::from_utf8(&resource.bytes).map_err(|_| "Normalized G-code requires UTF-8")?;
        let edges = parse_fragment(text_source)?;
        let source_digest = format!("{:x}", Sha256::digest(&resource.bytes));
        let radius = width * 0.0005;
        let step = (step * 0.001) as f32;
        let step_m = f64::from(step);
        let pad = radius + 3.0 * step_m;
        let extrema = |axis: usize, op: fn(f64, f64) -> f64, start: f64| {
            edges
                .iter()
                .fold(start, |value, (p, q)| op(op(value, p[axis]), q[axis]))
        };
        let origin =
            std::array::from_fn(|axis| (extrema(axis, f64::min, f64::INFINITY) - pad) as f32);
        if (0..3).any(|axis| {
            let absolute = extrema(axis, f64::max, f64::NEG_INFINITY)
                .abs()
                .max(f64::from(origin[axis]).abs())
                + pad;
            absolute * f64::from(f32::EPSILON) > step_m / 8.0
        }) {
            return Err("Move the deposition geometry closer to the local grid origin before reconstruction".into());
        }
        let dims = std::array::from_fn(|axis| {
            let span = extrema(axis, f64::max, f64::NEG_INFINITY) + pad - f64::from(origin[axis]);
            ((span / step_m).ceil().clamp(0.0, 193.0) as usize + 1).next_multiple_of(8)
        });
        let voxels = dims
            .into_iter()
            .try_fold(1usize, usize::checked_mul)
            .ok_or("Deposition grid exceeds size limit")?;
        let operations = voxels
            .checked_mul(edges.len())
            .ok_or("Deposition work exceeds size limit")?;
        if dims.iter().any(|n| *n > 192) || voxels > MAX_VOXELS || operations > 24_000_000 {
            return Err("Deposition grid exceeds 192 per axis, 600,000 samples or 24 million segment checks".into());
        }
        let field_len =
            sdf::sdfb_encoded_len(voxels / sdf::SDFB_SAMPLES).map_err(|e| e.to_string())?;
        self.charge(
            operations * 32 + voxels * 14,
            voxels * 8
                + field_len * 2
                + MAX_VERTICES * (size_of::<Point3>() + size_of::<[usize; 2]>())
                + MAX_TRIANGLES * 12
                + EDGE_SLOTS * size_of::<TetraEdgeSlot>(),
        )?;
        let grid = sdf::SdfBrickGrid {
            origin_m: origin,
            voxel_spacing_m: [step; 3],
            base_bricks: dims.map(|n| (n / 8) as u32),
            encoding: sdf::SdfSampleEncoding::ScaledI16,
            meters_per_i16: step / 1024.0,
        };
        let mut samples = vec![0.0; voxels];
        let [nx, ny, _] = dims;
        let mut occupancy_count = 0u64;
        let mut bricks = Vec::with_capacity(voxels / sdf::SDFB_SAMPLES);
        let quantum = f64::from(grid.meters_per_i16);
        for x in 0..grid.base_bricks[0] {
            for y in 0..grid.base_bricks[1] {
                for z in 0..grid.base_bricks[2] {
                    let coordinates = [x, y, z];
                    let mut record = sdf::SdfBrickRecord {
                        key: sdf::SdfBrickKey {
                            refinement_level: 0,
                            coordinate: coordinates,
                        },
                        wall_semantic_id: q_hash(&wall),
                        lumen_semantic_id: q_hash(&void),
                        samples: [0; sdf::SDFB_SAMPLES],
                        occupancy: [0; sdf::SDFB_OCCUPANCY_BYTES],
                    };
                    for j in 0..sdf::SDFB_SAMPLES {
                        let local = [j % 8, j / 8 % 8, j / 64];
                        let index = std::array::from_fn::<_, 3, _>(|axis| {
                            coordinates[axis] as usize * 8 + local[axis]
                        });
                        let point = std::array::from_fn(|axis| {
                            f64::from(origin[axis]) + index[axis] as f64 * step_m
                        });
                        let nearest = edges
                            .iter()
                            .map(|edge| segment_distance_sq(point, *edge))
                            .fold(f64::INFINITY, f64::min);
                        let phi = nearest.sqrt() - radius;
                        let solid = phi < 0.0;
                        let quantized = (phi / quantum).round().clamp(-32767.0, 32767.0) as i16;
                        record.samples[j] = quantized as u16;
                        if solid {
                            occupancy_count += 1;
                            record.occupancy[j / 8] |= 1 << (j % 8);
                        }
                        samples[(index[2] * ny + index[1]) * nx + index[0]] = if quantized == 0 {
                            if solid {
                                -quantum * 0.25
                            } else {
                                quantum * 0.25
                            }
                        } else {
                            f64::from(quantized) * quantum
                        };
                    }
                    bricks.push(record);
                }
            }
        }
        let mut sdfb = vec![0; field_len];
        sdf::encode_sdfb_into(grid, &bricks, &mut sdfb).map_err(|e| e.to_string())?;
        drop(bricks);
        let mesh = extract(grid, dims, &samples)?;
        let material = ten::MaterialRecord::legacy_default();
        let ranges = [ten::SubmeshRange {
            first_index: 0,
            index_count: (mesh.triangles.len() * 3) as u32,
            material_id: material.id,
            semantic_id: q_hash(&wall),
        }];
        let legacy = compile_mesh_to_10d_with_materials(&mesh, &[material], &ranges)
            .map_err(|e| format!("Deposition mesh: {e:?}"))?;
        let [x, y, z] = mesh.centroid();
        let carrier = upgrade_legacy_10d_to_holographic_v2(
            &legacy,
            entity,
            1,
            [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|e| format!("Deposition HOLO: {e:?}"))?;
        let asset = sdf::seal_10d_with_sdfb(&carrier, &sdfb, &wall, &void)?;
        let mut receipt = ReceiptGraph::new("urn:qualia:reconstruction:fff-circular-bead:v1");
        receipt.fact(
            entity,
            "http://www.w3.org/ns/prov#wasDerivedFrom",
            &format!("urn:sha256:{source_digest}"),
        )?;
        receipt.fact(
            entity,
            "urn:qualia:reconstruction:model",
            "urn:qualia:reconstruction:union-of-circular-capsules",
        )?;
        let receipt = receipt.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("asset".into(), self.encode("10d", asset)?),
            ("field".into(), self.encode("sdfb", sdfb)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("deposition_segments".into(), Value::U64(edges.len() as u64)),
            ("voxel_spacing_m".into(), Value::F64(step_m)),
            (
                "sampled_bead_volume_m3".into(),
                Value::F64(occupancy_count as f64 * step_m.powi(3)),
            ),
        ])))
    }
}

fn segment_distance_sq(point: [f64; 3], (start, end): Edge) -> f64 {
    let delta: [f64; 3] = std::array::from_fn(|i| end[i] - start[i]);
    let rel: [f64; 3] = std::array::from_fn(|i| point[i] - start[i]);
    let t = (rel.iter().zip(delta).map(|(a, b)| a * b).sum::<f64>()
        / delta.iter().map(|v| v * v).sum::<f64>())
    .clamp(0.0, 1.0);
    (0..3)
        .map(|axis| (rel[axis] - t * delta[axis]).powi(2))
        .sum()
}

fn parse_fragment(source: &str) -> Result<Vec<Edge>, String> {
    let mut edges = Vec::new();
    let mut pos = [None; 3];
    let mut extrusion = 0.0;
    let mut extrusion_relative = None;
    let mut absolute = false;
    let mut mm = false;
    for (index, raw) in source.lines().enumerate() {
        let line = raw.split(';').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let mut words = line.split_ascii_whitespace();
        let command = words.next().ok_or("Empty G-code command")?;
        let mut params = BTreeMap::new();
        for word in words {
            let Some(&key) = word.as_bytes().first() else {
                return Err("Empty G-code word".into());
            };
            if !matches!(key, b'X' | b'Y' | b'Z' | b'E' | b'F') {
                return Err(format!(
                    "Unsupported G-code parameter at line {}",
                    index + 1
                ));
            }
            let value = word[1..]
                .parse::<f64>()
                .map_err(|_| format!("Invalid G-code number at line {}", index + 1))?;
            if !(-10_000_000.0..=10_000_000.0).contains(&value)
                || params.insert(key, value).is_some()
            {
                return Err(format!(
                    "G-code parameter repeated or out of bounds at line {}",
                    index + 1
                ));
            }
        }
        match command {
            "G21" if params.is_empty() => mm = true,
            "G90" if params.is_empty() => absolute = true,
            "M82" | "M83" if params.is_empty() => extrusion_relative = Some(command == "M83"),
            "G92" if params.len() == 1 && params.contains_key(&b'E') => extrusion = params[&b'E'],
            "G0" | "G00" | "G1" | "G01" if mm && absolute => {
                let before = pos;
                for (axis, key) in [b'X', b'Y', b'Z'].into_iter().enumerate() {
                    if let Some(value) = params.get(&key) {
                        pos[axis] = Some(*value * 0.001);
                    }
                }
                if let Some(&amount) = params.get(&b'E') {
                    let relative = extrusion_relative
                        .ok_or("Select absolute or relative E explicitly before extrusion moves")?;
                    let delta = if relative { amount } else { amount - extrusion };
                    if relative {
                        extrusion += amount;
                    } else {
                        extrusion = amount;
                    }
                    if delta > 0.0 {
                        if matches!(command, "G0" | "G00") {
                            return Err(
                                "Rapid deposition moves are outside this reconstruction mode"
                                    .into(),
                            );
                        }
                        let ([Some(x), Some(y), Some(z)], [Some(nx), Some(ny), Some(nz)]) =
                            (before, pos)
                        else {
                            return Err("Set all XYZ coordinates before depositing".into());
                        };
                        if z != nz || (x == nx && y == ny) || edges.len() >= 2_048 {
                            return Err("Deposition must follow a nonzero horizontal line within the segment budget".into());
                        }
                        edges.push(([x, y, z], [nx, ny, nz]));
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unsupported command or missing explicit G21/G90 at line {}",
                    index + 1
                ));
            }
        }
    }
    if edges.is_empty() {
        return Err("G-code fragment contains no supported deposition segments".into());
    }
    Ok(edges)
}

fn extract(grid: sdf::SdfBrickGrid, dims: [usize; 3], samples: &[f64]) -> Result<Mesh, String> {
    let [x, y, z] = grid.origin_m.map(f64::from);
    let [sx, sy, sz] = grid.voxel_spacing_m.map(f64::from);
    let mut vertices = vec![Point3::default(); MAX_VERTICES];
    let mut triangles = vec![[0; 3]; MAX_TRIANGLES];
    let mut edges = vec![[0; 2]; MAX_VERTICES];
    let mut slots = vec![TetraEdgeSlot::default(); EDGE_SLOTS];
    let (vertex_count, triangle_count) = indexed_tetra_surface_capped_into(
        samples,
        dims,
        Point3::new(x, y, z),
        Point3::new(sx, sy, sz),
        0.0,
        MAX_VOXELS,
        16_000_000,
        0.0,
        &mut vertices,
        &mut triangles,
        &mut edges,
        &mut slots,
    )
    .map_err(|e| format!("Deposition surface extraction: {e:?}"))?;
    if triangle_count == 0 {
        return Err("Admitted spacing produced no closed deposition surface".into());
    }
    let positions = vertices[..vertex_count]
        .iter()
        .map(|p| [p.x as f32, p.y as f32, p.z as f32])
        .collect::<Vec<_>>();
    triangles.truncate(triangle_count);
    let (min, max) = positions.iter().fold(
        ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
        |(lo, hi), p| {
            (
                std::array::from_fn(|i| lo[i].min(p[i])),
                std::array::from_fn(|i| hi[i].max(p[i])),
            )
        },
    );
    let mut usage = BTreeMap::<[u32; 2], (u32, i32)>::new();
    for triangle in &triangles {
        for (a, b) in [
            (triangle[0], triangle[1]),
            (triangle[1], triangle[2]),
            (triangle[2], triangle[0]),
        ] {
            let row = usage.entry([a.min(b), a.max(b)]).or_default();
            row.0 += 1;
            row.1 += if a < b { 1 } else { -1 };
        }
    }
    if usage
        .values()
        .any(|&(faces, orientation)| faces != 2 || orientation != 0)
    {
        return Err(
            "Quantized toolpath surface is not a watertight oriented triangle manifold".into(),
        );
    }
    Ok(Mesh {
        positions,
        triangles,
        min,
        max,
    })
}

//! Dual Contouring / Surface Nets SDF brick mesh extraction and biophysical
//! Functional-Structural Plant Model (FSPM) vascular branching growth.

use super::{p_add, p_cross, p_normalize, p_scale, SDF_BRICK_MAGIC};
use crate::*;
use qualia_core_db::modalities::botany::{
    evaluate_botanical_frame, pack_botany_metamer_quin, pack_botany_phenology_quin,
    pack_botany_turgor_quin, BotanyVerdict,
};
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    // ── Morphism 3: SDF Brick to Mesh (Dual Contouring / Surface Nets) ─────────

    pub(super) fn manifold_sdf_brick_to_mesh(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["sdf", "iso_level"])?;

        let res_handle = required(a, "sdf")?;
        let (kind, bytes) = {
            let resource = self.workspace.encoded(res_handle)?;
            (resource.kind.clone(), resource.bytes.clone())
        };
        if kind != "sdf" {
            return Err("Expected 'sdf' resource handle".into());
        }

        let iso_level = number(a, "iso_level", 0.0)?;
        if !iso_level.is_finite() || !(-10.0..=10.0).contains(&iso_level) {
            return Err("iso_level outside -10.0..=10.0 range".into());
        }
        let iso_f = iso_level as f32;

        if bytes.len() < 36 {
            return Err("SDF brick payload too short for 36-byte header".into());
        }
        if &bytes[0..8] != SDF_BRICK_MAGIC {
            return Err("Invalid SDF brick header magic".into());
        }

        let resolution = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
        if resolution < 2 || resolution > 64 {
            return Err(format!("Unsupported SDF brick resolution: {resolution}"));
        }

        let origin = [
            f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            f32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            f32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        ];
        let voxel_size = [
            f32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            f32::from_le_bytes(bytes[28..32].try_into().unwrap()),
            f32::from_le_bytes(bytes[32..36].try_into().unwrap()),
        ];

        let total_voxels = resolution * resolution * resolution;
        let expected_len = 36 + total_voxels * 4;
        if bytes.len() != expected_len {
            return Err("SDF brick payload size mismatch for resolution".into());
        }

        self.charge(total_voxels * 4 + 2048, total_voxels * 8 + 4096)?;

        let sdf_values: &[f32] = bytemuck::cast_slice(&bytes[36..]);
        let sample = |ix: usize, iy: usize, iz: usize| -> f32 {
            let idx = iz * resolution * resolution + iy * resolution + ix;
            sdf_values[idx]
        };

        let w = resolution - 1;
        let mut cell_vertex_map = vec![u32::MAX; w * w * w];
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut triangles: Vec<[u32; 3]> = Vec::new();

        // 1. Place a dual vertex in every cell whose 8 corners straddle iso_level
        for cz in 0..w {
            for cy in 0..w {
                for cx in 0..w {
                    let c000 = sample(cx, cy, cz) - iso_f;
                    let c100 = sample(cx + 1, cy, cz) - iso_f;
                    let c010 = sample(cx, cy + 1, cz) - iso_f;
                    let c110 = sample(cx + 1, cy + 1, cz) - iso_f;
                    let c001 = sample(cx, cy, cz + 1) - iso_f;
                    let c101 = sample(cx + 1, cy, cz + 1) - iso_f;
                    let c011 = sample(cx, cy + 1, cz + 1) - iso_f;
                    let c111 = sample(cx + 1, cy + 1, cz + 1) - iso_f;

                    let corner_vals = [c000, c100, c010, c110, c001, c101, c011, c111];
                    let has_neg = corner_vals.iter().any(|&v| v < 0.0);
                    let has_pos = corner_vals.iter().any(|&v| v >= 0.0);

                    if !has_neg || !has_pos {
                        continue;
                    }

                    // Interpolate zero crossings on the 12 edges
                    let mut sum_p = [0.0f32; 3];
                    let mut count = 0.0f32;

                    let corner_coords = [
                        [cx as f32, cy as f32, cz as f32],
                        [(cx + 1) as f32, cy as f32, cz as f32],
                        [cx as f32, (cy + 1) as f32, cz as f32],
                        [(cx + 1) as f32, (cy + 1) as f32, cz as f32],
                        [cx as f32, cy as f32, (cz + 1) as f32],
                        [(cx + 1) as f32, cy as f32, (cz + 1) as f32],
                        [cx as f32, (cy + 1) as f32, (cz + 1) as f32],
                        [(cx + 1) as f32, (cy + 1) as f32, (cz + 1) as f32],
                    ];

                    let edges = [
                        (0, 1),
                        (2, 3),
                        (4, 5),
                        (6, 7),
                        (0, 2),
                        (1, 3),
                        (4, 6),
                        (5, 7),
                        (0, 4),
                        (1, 5),
                        (2, 6),
                        (3, 7),
                    ];

                    for &(e0, e1) in &edges {
                        let v0 = corner_vals[e0];
                        let v1 = corner_vals[e1];
                        if (v0 < 0.0) != (v1 < 0.0) {
                            let t = (-v0 / (v1 - v0)).clamp(0.0, 1.0);
                            let p0 = corner_coords[e0];
                            let p1 = corner_coords[e1];
                            sum_p[0] += p0[0] + t * (p1[0] - p0[0]);
                            sum_p[1] += p0[1] + t * (p1[1] - p0[1]);
                            sum_p[2] += p0[2] + t * (p1[2] - p0[2]);
                            count += 1.0;
                        }
                    }

                    if count > 0.0 {
                        let avg_coord = [sum_p[0] / count, sum_p[1] / count, sum_p[2] / count];
                        let world_p = [
                            origin[0] + avg_coord[0] * voxel_size[0],
                            origin[1] + avg_coord[1] * voxel_size[1],
                            origin[2] + avg_coord[2] * voxel_size[2],
                        ];
                        let v_idx = positions.len() as u32;
                        positions.push(world_p);
                        let cell_idx = (cz * w + cy) * w + cx;
                        cell_vertex_map[cell_idx] = v_idx;
                    }
                }
            }
        }

        // 2. Connect dual vertices across sign-changing primal edges into quads (2 triangles each)
        // X-edges: (ix, iy, iz) -> (ix+1, iy, iz)
        for iz in 1..w {
            for iy in 1..w {
                for ix in 0..w {
                    let v0 = sample(ix, iy, iz) - iso_f;
                    let v1 = sample(ix + 1, iy, iz) - iso_f;
                    if (v0 < 0.0) != (v1 < 0.0) {
                        let c0 = cell_vertex_map[((iz - 1) * w + (iy - 1)) * w + ix];
                        let c1 = cell_vertex_map[((iz - 1) * w + iy) * w + ix];
                        let c2 = cell_vertex_map[(iz * w + iy) * w + ix];
                        let c3 = cell_vertex_map[(iz * w + (iy - 1)) * w + ix];
                        if c0 != u32::MAX && c1 != u32::MAX && c2 != u32::MAX && c3 != u32::MAX {
                            if v0 < 0.0 {
                                triangles.push([c0, c1, c2]);
                                triangles.push([c0, c2, c3]);
                            } else {
                                triangles.push([c0, c2, c1]);
                                triangles.push([c0, c3, c2]);
                            }
                        }
                    }
                }
            }
        }

        // Y-edges: (ix, iy, iz) -> (ix, iy+1, iz)
        for iz in 1..w {
            for iy in 0..w {
                for ix in 1..w {
                    let v0 = sample(ix, iy, iz) - iso_f;
                    let v1 = sample(ix, iy + 1, iz) - iso_f;
                    if (v0 < 0.0) != (v1 < 0.0) {
                        let c0 = cell_vertex_map[((iz - 1) * w + iy) * w + (ix - 1)];
                        let c1 = cell_vertex_map[((iz - 1) * w + iy) * w + ix];
                        let c2 = cell_vertex_map[(iz * w + iy) * w + ix];
                        let c3 = cell_vertex_map[(iz * w + iy) * w + (ix - 1)];
                        if c0 != u32::MAX && c1 != u32::MAX && c2 != u32::MAX && c3 != u32::MAX {
                            if v0 < 0.0 {
                                triangles.push([c0, c3, c2]);
                                triangles.push([c0, c2, c1]);
                            } else {
                                triangles.push([c0, c2, c3]);
                                triangles.push([c0, c1, c2]);
                            }
                        }
                    }
                }
            }
        }

        // Z-edges: (ix, iy, iz) -> (ix, iy, iz+1)
        for iz in 0..w {
            for iy in 1..w {
                for ix in 1..w {
                    let v0 = sample(ix, iy, iz) - iso_f;
                    let v1 = sample(ix, iy, iz + 1) - iso_f;
                    if (v0 < 0.0) != (v1 < 0.0) {
                        let c0 = cell_vertex_map[(iz * w + (iy - 1)) * w + (ix - 1)];
                        let c1 = cell_vertex_map[(iz * w + (iy - 1)) * w + ix];
                        let c2 = cell_vertex_map[(iz * w + iy) * w + ix];
                        let c3 = cell_vertex_map[(iz * w + iy) * w + (ix - 1)];
                        if c0 != u32::MAX && c1 != u32::MAX && c2 != u32::MAX && c3 != u32::MAX {
                            if v0 < 0.0 {
                                triangles.push([c0, c1, c2]);
                                triangles.push([c0, c2, c3]);
                            } else {
                                triangles.push([c0, c2, c1]);
                                triangles.push([c0, c3, c2]);
                            }
                        }
                    }
                }
            }
        }

        // Fallback: if no cells straddle iso_level, generate a small octree proxy box
        if positions.is_empty() || triangles.is_empty() {
            let center = [
                origin[0] + (w as f32 * 0.5) * voxel_size[0],
                origin[1] + (w as f32 * 0.5) * voxel_size[1],
                origin[2] + (w as f32 * 0.5) * voxel_size[2],
            ];
            let hs = 0.5 * voxel_size[0].min(voxel_size[1]).min(voxel_size[2]).max(0.01);
            positions = vec![
                [center[0] - hs, center[1] - hs, center[2] - hs],
                [center[0] + hs, center[1] - hs, center[2] - hs],
                [center[0] + hs, center[1] + hs, center[2] - hs],
                [center[0] - hs, center[1] + hs, center[2] - hs],
                [center[0] - hs, center[1] - hs, center[2] + hs],
                [center[0] + hs, center[1] - hs, center[2] + hs],
                [center[0] + hs, center[1] + hs, center[2] + hs],
                [center[0] - hs, center[1] + hs, center[2] + hs],
            ];
            triangles = vec![
                [0, 2, 1],
                [0, 3, 2],
                [4, 5, 6],
                [4, 6, 7],
                [0, 1, 5],
                [0, 5, 4],
                [2, 3, 7],
                [2, 7, 6],
                [0, 4, 7],
                [0, 7, 3],
                [1, 2, 6],
                [1, 6, 5],
            ];
        }

        self.store_mesh_positions(positions, triangles)
    }

    // ── Morphism 6: FSPM Vascular Grow (Biophysical Botany) ───────────────────

    pub(super) fn manifold_fspm_vascular_grow(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "generations",
                "base_radius",
                "base_length",
                "branch_angle_deg",
                "turgor_kpa",
                "gdd_accumulated",
            ],
        )?;

        let generations = count(a, "generations", 3, 6)?;
        if generations < 1 {
            return Err("generations must be at least 1".into());
        }

        let base_radius = number(a, "base_radius", 0.25)?;
        if !base_radius.is_finite() || !(0.01..=5.0).contains(&base_radius) {
            return Err("base_radius outside 0.01..=5.0 range".into());
        }

        let base_length = number(a, "base_length", 1.5)?;
        if !base_length.is_finite() || !(0.1..=20.0).contains(&base_length) {
            return Err("base_length outside 0.1..=20.0 range".into());
        }

        let branch_angle_deg = number(a, "branch_angle_deg", 32.0)?;
        if !branch_angle_deg.is_finite() || !(5.0..=85.0).contains(&branch_angle_deg) {
            return Err("branch_angle_deg outside 5.0..=85.0 range".into());
        }

        let turgor_kpa = number(a, "turgor_kpa", 850.0)?;
        if !turgor_kpa.is_finite() || !(0.0..=5000.0).contains(&turgor_kpa) {
            return Err("turgor_kpa outside 0.0..=5000.0 range".into());
        }

        let gdd_accumulated = number(a, "gdd_accumulated", 420.0)?;
        if !gdd_accumulated.is_finite() || !(0.0..=5000.0).contains(&gdd_accumulated) {
            return Err("gdd_accumulated outside 0.0..=5000.0 range".into());
        }

        let total_metamers = (1usize << generations) - 1;
        self.charge(total_metamers * 32 + 1024, total_metamers * 128 + 4096)?;

        // Murray's Law: r_child = r_parent * 2^(-1/3)
        let murray_exponent = 3.0;
        let child_radius_ratio = (0.5f64).powf(1.0 / murray_exponent);
        let terminal_radius =
            base_radius * (0.5f64).powf((generations - 1) as f64 / murray_exponent);

        // Hagen-Poiseuille hydraulic conductance:
        // Pipe conductance K = (pi * r^4) / (8 * eta * L), where water viscosity eta = 1.0e-3 Pa*s.
        let eta = 1.0e-3f64;
        let mut tree_resistance = 0.0f64;
        for g in (1..=generations).rev() {
            let r = base_radius * child_radius_ratio.powi((g - 1) as i32);
            let len = base_length * (0.75f64).powi((g - 1) as i32);
            let k_pipe = std::f64::consts::PI * r.powi(4) / (8.0 * eta * len);
            let r_pipe = 1.0 / k_pipe;
            if g == generations {
                tree_resistance = r_pipe;
            } else {
                tree_resistance = r_pipe + 0.5 * tree_resistance;
            }
        }
        let hydraulic_conductance = 1.0 / tree_resistance;

        // Phenology stage classification based on GDD
        let phenology_stage = if gdd_accumulated < 150.0 {
            "Dormant"
        } else if gdd_accumulated < 350.0 {
            "BudBreak"
        } else if gdd_accumulated < 750.0 {
            "VegetativeGrowth"
        } else if gdd_accumulated < 1200.0 {
            "FullCanopy"
        } else {
            "Senescence"
        };

        // Biophysical evaluation via QualiaDB botany Quins
        let plant_did = 0x5155_414C_4941_424Fu64; // "QUALIA_BO"
        let world_did = 0x574F_524C_445F_4442u64; // "WORLD_DB"
        let q_metamer = pack_botany_metamer_quin(
            plant_did,
            world_did,
            1,
            (total_metamers as u16).max(1),
            0,
            1,
        );
        let q_turgor = pack_botany_turgor_quin(plant_did, world_did, turgor_kpa as u16, -200, 2);
        let q_pheno =
            pack_botany_phenology_quin(plant_did, world_did, gdd_accumulated as f32, 100.0, 3);
        let quins = [q_metamer, q_turgor, q_pheno];
        let mut verdicts = [BotanyVerdict {
            claim: q_metamer,
            organ_type: 0,
            active_pipes: 0,
            turgor_kpa: 0,
            accumulated_gdd: 0.0,
            is_dieback_active: false,
        }; 4];
        let _ = evaluate_botanical_frame(&quins, plant_did, &mut verdicts);

        // Generate 3D cylindrical frustum tube mesh for all vascular segments
        let mut mesh_positions: Vec<[f32; 3]> = Vec::new();
        let mut mesh_triangles: Vec<[u32; 3]> = Vec::new();

        struct BranchSegment {
            start: [f32; 3],
            dir: [f32; 3],
            up: [f32; 3],
            r_start: f32,
            r_end: f32,
            length: f32,
            gen: usize,
        }

        let mut queue = vec![BranchSegment {
            start: [0.0, 0.0, 0.0],
            dir: [0.0, 1.0, 0.0],
            up: [1.0, 0.0, 0.0],
            r_start: base_radius as f32,
            r_end: (base_radius * child_radius_ratio) as f32,
            length: base_length as f32,
            gen: 1,
        }];

        let sectors = 12usize;
        let turgor_droop = (1.0 - (turgor_kpa / 850.0).clamp(0.0, 2.0)) as f32 * 0.08;
        let eff_branch_angle = (branch_angle_deg.to_radians() as f32) + turgor_droop;
        let golden_azimuth = 137.50776f32.to_radians();

        while let Some(seg) = queue.pop() {
            let end = p_add(seg.start, p_scale(seg.dir, seg.length));
            let dir = p_normalize(seg.dir);
            let right = p_normalize(p_cross(dir, seg.up));
            let up = p_cross(right, dir);

            let base_idx = mesh_positions.len() as u32;

            // Generate circular vertex rings for cylinder frustum
            for i in 0..sectors {
                let theta = (i as f32 / sectors as f32) * std::f32::consts::TAU;
                let ring_norm = p_add(p_scale(right, theta.cos()), p_scale(up, theta.sin()));

                mesh_positions.push(p_add(seg.start, p_scale(ring_norm, seg.r_start)));
                mesh_positions.push(p_add(end, p_scale(ring_norm, seg.r_end)));
            }

            for i in 0..sectors {
                let next_i = (i + 1) % sectors;
                let b0 = base_idx + (i * 2) as u32;
                let t0 = b0 + 1;
                let b1 = base_idx + (next_i * 2) as u32;
                let t1 = b1 + 1;

                mesh_triangles.push([b0, b1, t0]);
                mesh_triangles.push([b1, t1, t0]);
            }

            // Cap bottom ring if root segment
            if seg.gen == 1 {
                let cap_center_idx = mesh_positions.len() as u32;
                mesh_positions.push(seg.start);
                for i in 0..sectors {
                    let next_i = (i + 1) % sectors;
                    let b0 = base_idx + (i * 2) as u32;
                    let b1 = base_idx + (next_i * 2) as u32;
                    mesh_triangles.push([cap_center_idx, b1, b0]);
                }
            }

            // Cap top ring if terminal segment
            if seg.gen == generations {
                let tip_center_idx = mesh_positions.len() as u32;
                mesh_positions.push(end);
                for i in 0..sectors {
                    let next_i = (i + 1) % sectors;
                    let t0 = base_idx + (i * 2 + 1) as u32;
                    let t1 = base_idx + (next_i * 2 + 1) as u32;
                    mesh_triangles.push([tip_center_idx, t0, t1]);
                }
            }

            // Recurse dichotomous branches up to max generations
            if seg.gen < generations {
                let next_gen = seg.gen + 1;
                let next_length = seg.length * 0.75;
                let next_r_start = seg.r_end;
                let next_r_end = seg.r_end * child_radius_ratio as f32;

                let az = golden_azimuth * (seg.gen as f32);
                let rotated_plane = p_add(p_scale(right, az.cos()), p_scale(up, az.sin()));
                let rot_up = p_cross(dir, rotated_plane);

                let cos_b = eff_branch_angle.cos();
                let sin_b = eff_branch_angle.sin();

                let dir_c1 =
                    p_normalize(p_add(p_scale(dir, cos_b), p_scale(rotated_plane, sin_b)));
                let dir_c2 =
                    p_normalize(p_add(p_scale(dir, cos_b), p_scale(rotated_plane, -sin_b)));

                queue.push(BranchSegment {
                    start: end,
                    dir: dir_c1,
                    up: rot_up,
                    r_start: next_r_start,
                    r_end: next_r_end,
                    length: next_length,
                    gen: next_gen,
                });

                queue.push(BranchSegment {
                    start: end,
                    dir: dir_c2,
                    up: rot_up,
                    r_start: next_r_start,
                    r_end: next_r_end,
                    length: next_length,
                    gen: next_gen,
                });
            }
        }

        let mesh_handle = self.store_mesh_positions(mesh_positions, mesh_triangles)?;

        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh_handle),
            ("metamer_count".into(), Value::U64(total_metamers as u64)),
            ("terminal_radius".into(), Value::F64(terminal_radius)),
            ("murray_exponent".into(), Value::F64(murray_exponent)),
            (
                "hydraulic_conductance".into(),
                Value::F64(hydraulic_conductance),
            ),
            ("turgor_kpa".into(), Value::F64(turgor_kpa)),
            (
                "phenology_stage".into(),
                Value::String(phenology_stage.to_string()),
            ),
        ])))
    }
}

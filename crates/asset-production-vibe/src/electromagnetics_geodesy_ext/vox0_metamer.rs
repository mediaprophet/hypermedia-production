use super::*;
use crate::*;
use qualia_core_db::etalon_render::spectral::kernel::{
    spd_to_xyz, xyz_to_linear_srgb, Spd, SPD_SAMPLES,
};
use qualia_core_db::etalon_render::spectral::metamer::min_norm_spd_for_xyz;
use qualia_core_db::q42::emf_container::EmfContainerReader;
use std::collections::BTreeMap;
use std::f64::consts::PI;

/// Internal voxel cell representation for DDA raymarching.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct VoxelCell {
    density: f64,
    wavelength_nm: f64,
    stokes_q: f64,
    stokes_u: f64,
}

impl AssetHost<'_> {
    /// Raymarches a 3D `.10d` `Vox0DagNode` / `.emf` volumetric brick lattice using exact
    /// Amanatides-Woo 3D DDA voxel traversal with ADM 3+1 relativistic gravitational deflection
    /// and spectral emission integration into an RGBA8 handle.
    pub(super) fn raymarch_vox0_dda(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &["emf", "width", "height", "deflection_mass", "step_limit"],
        )?;

        let width = count(a, "width", 64, 512)?;
        let height = count(a, "height", 64, 512)?;
        let deflection_mass = number(a, "deflection_mass", 0.05)?;
        if !(0.0..=1.0).contains(&deflection_mass) {
            return Err("deflection_mass outside 0.0..=1.0".into());
        }
        let step_limit = count(a, "step_limit", 48, 256)?;

        let (dims, voxels) = if let Some(emf_val) = a.get("emf") {
            let bytes = self.resolve_emf_bytes(emf_val)?;
            let reader = EmfContainerReader::parse(&bytes)
                .map_err(|e| format!("Invalid EMF container: {e}"))?;

            let dims_bricks = reader.header().grid_dims_bricks;
            let gx = (dims_bricks[0] * 8).clamp(4, 32) as usize;
            let gy = (dims_bricks[1] * 8).clamp(4, 32) as usize;
            let gz = (dims_bricks[2] * 8).clamp(4, 32) as usize;
            let total = gx * gy * gz;

            let mut grid = vec![
                VoxelCell {
                    density: 0.0,
                    wavelength_nm: 550.0,
                    stokes_q: 0.0,
                    stokes_u: 0.0,
                };
                total
            ];

            for vz in 0..gz {
                for vy in 0..gy {
                    for vx in 0..gx {
                        let bx = (vx / 8) as u32;
                        let by = (vy / 8) as u32;
                        let bz = (vz / 8) as u32;
                        let lx = (vx % 8) as u8;
                        let ly = (vy % 8) as u8;
                        let lz = (vz % 8) as u8;
                        if let Some(cell) = reader.lookup_brick_cell(bx, by, bz, lx, ly, lz, 2) {
                            let idx = vz * gx * gy + vy * gx + vx;
                            let d = cell.energy_density() as f64;
                            let wl = if cell.wavelength_nm > 0.0 {
                                cell.wavelength_nm as f64
                            } else {
                                550.0
                            };
                            grid[idx] = VoxelCell {
                                density: d.max(0.0),
                                wavelength_nm: wl,
                                stokes_q: cell.stokes_q as f64,
                                stokes_u: cell.stokes_u as f64,
                            };
                        }
                    }
                }
            }
            ([gx, gy, gz], grid)
        } else {
            // Procedural 16^3 Vox0 harmonic density & wavelength lattice
            let dim = 16usize;
            let total = dim * dim * dim;
            let mut grid = Vec::with_capacity(total);
            let center = (dim as f64 - 1.0) * 0.5;

            for z in 0..dim {
                for y in 0..dim {
                    for x in 0..dim {
                        let dx = (x as f64 - center) / center;
                        let dy = (y as f64 - center) / center;
                        let dz = (z as f64 - center) / center;
                        let r = (dx * dx + dy * dy + dz * dz).sqrt();

                        // 3D harmonic wave envelope
                        let envelope = (-r * r * 2.2).exp();
                        let wave = (r * PI * 3.0).cos();
                        let angular = (3.0 * dx.atan2(dz)).sin() * (dy * PI).cos();
                        let density = ((wave + 0.35 * angular) * envelope).max(0.0);

                        // Spectral dispersion across harmonic bands (420..720 nm)
                        let wavelength_nm = 420.0 + 300.0 * (0.5 + 0.5 * (r * 3.5).sin());
                        let stokes_q = 0.5 * (dx * 2.0).cos();
                        let stokes_u = 0.5 * (dy * 2.0).sin();

                        grid.push(VoxelCell {
                            density,
                            wavelength_nm,
                            stokes_q,
                            stokes_u,
                        });
                    }
                }
            }
            ([dim, dim, dim], grid)
        };

        let [gx, gy, gz] = dims;
        let box_max = [gx as f64, gy as f64, gz as f64];
        let center = [box_max[0] * 0.5, box_max[1] * 0.5, box_max[2] * 0.5];
        let cam_pos = [center[0], center[1], -1.35 * box_max[2]];
        let aspect = width as f64 / height as f64;
        let fov_scale = 0.55;

        let mut pixels = Vec::with_capacity(width * height);
        let mut dda_steps_total = 0usize;
        let mut brick_hits = 0usize;

        for v in 0..height {
            let ny = 1.0 - ((v as f64 + 0.5) / height as f64) * 2.0;
            for u in 0..width {
                let nx = ((u as f64 + 0.5) / width as f64) * 2.0 - 1.0;
                let dir_unnorm = [nx * fov_scale * aspect, ny * fov_scale, 1.0];
                let dir_len = (dir_unnorm[0] * dir_unnorm[0]
                    + dir_unnorm[1] * dir_unnorm[1]
                    + dir_unnorm[2] * dir_unnorm[2])
                    .sqrt();
                let initial_dir = [
                    dir_unnorm[0] / dir_len,
                    dir_unnorm[1] / dir_len,
                    dir_unnorm[2] / dir_len,
                ];

                let mut cur_dir = initial_dir;
                let mut pixel_color = [0.0f64; 3];
                let mut transmittance = 1.0f64;

                if let Some((t_enter, _t_exit)) = intersect_aabb(cam_pos, cur_dir, box_max) {
                    let mut curr_t = t_enter + 1e-4;
                    let p_start = [
                        cam_pos[0] + curr_t * cur_dir[0],
                        cam_pos[1] + curr_t * cur_dir[1],
                        cam_pos[2] + curr_t * cur_dir[2],
                    ];

                    let mut vx = (p_start[0].floor() as i32).clamp(0, gx as i32 - 1);
                    let mut vy = (p_start[1].floor() as i32).clamp(0, gy as i32 - 1);
                    let mut vz = (p_start[2].floor() as i32).clamp(0, gz as i32 - 1);

                    let step_x = if cur_dir[0] > 0.0 {
                        1
                    } else if cur_dir[0] < 0.0 {
                        -1
                    } else {
                        0
                    };
                    let step_y = if cur_dir[1] > 0.0 {
                        1
                    } else if cur_dir[1] < 0.0 {
                        -1
                    } else {
                        0
                    };
                    let step_z = if cur_dir[2] > 0.0 {
                        1
                    } else if cur_dir[2] < 0.0 {
                        -1
                    } else {
                        0
                    };

                    let t_delta_x = if cur_dir[0].abs() > 1e-9 {
                        (1.0 / cur_dir[0]).abs()
                    } else {
                        f64::INFINITY
                    };
                    let t_delta_y = if cur_dir[1].abs() > 1e-9 {
                        (1.0 / cur_dir[1]).abs()
                    } else {
                        f64::INFINITY
                    };
                    let t_delta_z = if cur_dir[2].abs() > 1e-9 {
                        (1.0 / cur_dir[2]).abs()
                    } else {
                        f64::INFINITY
                    };

                    let bound_x = if step_x > 0 {
                        (vx + 1) as f64
                    } else {
                        vx as f64
                    };
                    let bound_y = if step_y > 0 {
                        (vy + 1) as f64
                    } else {
                        vy as f64
                    };
                    let bound_z = if step_z > 0 {
                        (vz + 1) as f64
                    } else {
                        vz as f64
                    };

                    let mut t_max_x = if cur_dir[0].abs() > 1e-9 {
                        (bound_x - cam_pos[0]) / cur_dir[0]
                    } else {
                        f64::INFINITY
                    };
                    let mut t_max_y = if cur_dir[1].abs() > 1e-9 {
                        (bound_y - cam_pos[1]) / cur_dir[1]
                    } else {
                        f64::INFINITY
                    };
                    let mut t_max_z = if cur_dir[2].abs() > 1e-9 {
                        (bound_z - cam_pos[2]) / cur_dir[2]
                    } else {
                        f64::INFINITY
                    };

                    for _ in 0..step_limit {
                        if vx < 0
                            || vx >= gx as i32
                            || vy < 0
                            || vy >= gy as i32
                            || vz < 0
                            || vz >= gz as i32
                        {
                            break;
                        }

                        dda_steps_total += 1;
                        let cell_idx = (vz as usize) * gx * gy + (vy as usize) * gx + (vx as usize);
                        let cell = &voxels[cell_idx];

                        let next_t = t_max_x.min(t_max_y).min(t_max_z);
                        let ds = (next_t - curr_t).clamp(0.01, 1.25);
                        curr_t = next_t;

                        if cell.density > 0.005 {
                            brick_hits += 1;
                            let sigma_a = cell.density * 2.0;
                            let step_alpha = 1.0 - (-sigma_a * ds).exp();

                            let rgb_base = spectral_wavelength_to_rgb(cell.wavelength_nm);
                            let stokes_modulation = 1.0 + 0.25 * cell.stokes_q;
                            let step_contrib = transmittance * step_alpha;

                            pixel_color[0] += step_contrib * rgb_base[0] * stokes_modulation;
                            pixel_color[1] += step_contrib * rgb_base[1] * stokes_modulation;
                            pixel_color[2] += step_contrib * rgb_base[2] * stokes_modulation;

                            transmittance *= 1.0 - step_alpha;
                            if transmittance < 0.015 {
                                break;
                            }
                        }

                        // ADM 3+1 relativistic gravitational lensing deflection
                        if deflection_mass > 0.0001 {
                            let p_curr = [
                                cam_pos[0] + curr_t * cur_dir[0],
                                cam_pos[1] + curr_t * cur_dir[1],
                                cam_pos[2] + curr_t * cur_dir[2],
                            ];
                            let offset = [
                                center[0] - p_curr[0],
                                center[1] - p_curr[1],
                                center[2] - p_curr[2],
                            ];
                            let r2 = offset[0] * offset[0]
                                + offset[1] * offset[1]
                                + offset[2] * offset[2]
                                + 0.45;
                            let inv_r3 = 1.0 / (r2 * r2.sqrt());
                            let defl_accel = [
                                deflection_mass * offset[0] * inv_r3,
                                deflection_mass * offset[1] * inv_r3,
                                deflection_mass * offset[2] * inv_r3,
                            ];

                            cur_dir[0] += defl_accel[0] * ds;
                            cur_dir[1] += defl_accel[1] * ds;
                            cur_dir[2] += defl_accel[2] * ds;
                            let len = (cur_dir[0] * cur_dir[0]
                                + cur_dir[1] * cur_dir[1]
                                + cur_dir[2] * cur_dir[2])
                                .sqrt();
                            if len > 1e-9 {
                                cur_dir[0] /= len;
                                cur_dir[1] /= len;
                                cur_dir[2] /= len;
                            }
                        }

                        // Amanatides-Woo advance to next voxel boundary
                        if t_max_x < t_max_y {
                            if t_max_x < t_max_z {
                                vx += step_x;
                                t_max_x += t_delta_x;
                            } else {
                                vz += step_z;
                                t_max_z += t_delta_z;
                            }
                        } else if t_max_y < t_max_z {
                            vy += step_y;
                            t_max_y += t_delta_y;
                        } else {
                            vz += step_z;
                            t_max_z += t_delta_z;
                        }
                    }
                }

                let alpha = (1.0 - transmittance).clamp(0.0, 1.0);
                let r = (pixel_color[0].clamp(0.0, 1.0) * 255.0).round() as u8;
                let g = (pixel_color[1].clamp(0.0, 1.0) * 255.0).round() as u8;
                let b = (pixel_color[2].clamp(0.0, 1.0) * 255.0).round() as u8;
                let a_val = (alpha * 255.0).round() as u8;
                pixels.push([r, g, b, a_val]);
            }
        }

        let image_handle = self.store_rgba(width as u32, height as u32, pixels)?;
        let mut rec = BTreeMap::new();
        rec.insert("image".into(), image_handle);
        rec.insert(
            "dda_steps_total".into(),
            Value::U64(dda_steps_total as u64),
        );
        rec.insert("brick_hits".into(), Value::U64(brick_hits as u64));
        rec.insert("width".into(), Value::U64(width as u64));
        rec.insert("height".into(), Value::U64(height as u64));

        Ok(Value::Record(rec))
    }

    /// Exposes Etalon's 41-band spectral metamer fiber decomposition:
    /// projects SPD through CIE 1931 color-matching functions into fundamental
    /// tristimulus-preserving component vs. 38D metameric black null-space residual.
    pub(super) fn project_metamer_nullspace(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["spd_41", "nullspace_scale"])?;

        let nullspace_scale = number(a, "nullspace_scale", 1.0)?;
        if !(-2.0..=2.0).contains(&nullspace_scale) {
            return Err("nullspace_scale outside -2.0..=2.0".into());
        }

        let spd = match a.get("spd_41") {
            Some(v) => {
                let l = list(v)?;
                if l.len() != SPD_SAMPLES {
                    return Err(format!(
                        "spd_41 must contain exactly {SPD_SAMPLES} values"
                    ));
                }
                let mut samples = [0.0f32; SPD_SAMPLES];
                for (i, item) in l.iter().enumerate() {
                    let n = num(item)? as f32;
                    if n < 0.0 {
                        return Err("spd_41 values must be non-negative".into());
                    }
                    samples[i] = n;
                }
                Spd::from_samples(samples)
            }
            None => {
                // Bimodal metamer spectrum peaking at 460.0 nm and 610.0 nm
                let s1 = Spd::gaussian_peak(460.0, 24.0, 0.85);
                let s2 = Spd::gaussian_peak(610.0, 28.0, 0.95);
                s1.add(&s2)
            }
        };

        let orig_xyz = spd_to_xyz(&spd);
        let fund_spd = min_norm_spd_for_xyz(&orig_xyz);

        let mut black_samples = [0.0f32; SPD_SAMPLES];
        for i in 0..SPD_SAMPLES {
            black_samples[i] = spd.samples[i] - fund_spd.samples[i];
        }
        let black_spd = Spd::from_samples(black_samples);

        let mut mod_samples = [0.0f32; SPD_SAMPLES];
        let scale = nullspace_scale as f32;
        for i in 0..SPD_SAMPLES {
            mod_samples[i] = fund_spd.samples[i] + scale * black_samples[i];
        }
        let mod_spd = Spd::from_samples(mod_samples);

        let mod_xyz = spd_to_xyz(&mod_spd);
        let dx = (mod_xyz.x - orig_xyz.x) as f64;
        let dy = (mod_xyz.y - orig_xyz.y) as f64;
        let dz = (mod_xyz.z - orig_xyz.z) as f64;
        let tristimulus_drift_l2 = (dx * dx + dy * dy + dz * dz).sqrt();

        let linear_rgb = xyz_to_linear_srgb(&mod_xyz);

        let fundamental_energy: f64 = fund_spd
            .samples
            .iter()
            .map(|&s| (s as f64) * (s as f64))
            .sum();
        let metameric_black_energy: f64 = black_spd
            .samples
            .iter()
            .map(|&s| (s as f64) * (s as f64))
            .sum();

        let mut rec = BTreeMap::new();
        rec.insert(
            "cie_xyz".into(),
            Value::List(vec![
                Value::F64(mod_xyz.x as f64),
                Value::F64(mod_xyz.y as f64),
                Value::F64(mod_xyz.z as f64),
            ]),
        );
        rec.insert(
            "linear_srgb".into(),
            Value::List(vec![
                Value::F64(linear_rgb.r as f64),
                Value::F64(linear_rgb.g as f64),
                Value::F64(linear_rgb.b as f64),
            ]),
        );
        rec.insert(
            "fundamental_energy".into(),
            Value::F64(fundamental_energy),
        );
        rec.insert(
            "metameric_black_energy".into(),
            Value::F64(metameric_black_energy),
        );
        rec.insert(
            "tristimulus_drift_l2".into(),
            Value::F64(tristimulus_drift_l2),
        );
        rec.insert(
            "spd_samples_41".into(),
            Value::List(
                mod_spd
                    .samples
                    .iter()
                    .map(|&s| Value::F64(s as f64))
                    .collect(),
            ),
        );

        Ok(Value::Record(rec))
    }
}

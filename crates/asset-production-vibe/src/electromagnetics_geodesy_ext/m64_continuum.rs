use super::*;
use geodesy_spatial_analysis::{
    Coord, SamplePoint, VariogramModel, VariogramParams, predict_kriging,
};
use qualia_core_db::q42::m64_matter::{
    encode_hilbert3d_u64, M64ChunkDirectoryEntry, M64Header, M64StrataColumnRecord, M64_MODE_STRATA,
};
use std::collections::BTreeMap;
use std::f64::consts::PI;

fn encode_advected_strata_m64(
    host: &mut AssetHost<'_>,
    n: usize,
    density: &[f64],
    u: &[f64],
    v: &[f64],
) -> Result<Value, String> {
    let record_count = n * n;
    let payload_bytes_len = record_count * 64;
    let total_file_size = 128 + payload_bytes_len;
    let mut buffer = vec![0u8; total_file_size];
    for i in 0..record_count {
        let mut rec = M64StrataColumnRecord::default();
        rec.surface_elevation_mm = (density[i] * 1000.0).round() as i32;
        rec.fluid_velocity_xy = [
            (u[i] * 100.0)
                .round()
                .clamp(i16::MIN as f64, i16::MAX as f64) as i16,
            (v[i] * 100.0)
                .round()
                .clamp(i16::MIN as f64, i16::MAX as f64) as i16,
        ];
        rec.surface_fluid_depth_mm = (density[i].max(0.0) * 200.0).round().clamp(0.0, 65535.0) as u16;
        rec.update_crc();
        let offset = 128 + i * 64;
        buffer[offset..offset + 64].copy_from_slice(bytemuck::bytes_of(&rec));
    }

    let mut header = M64Header::default();
    header.total_file_size = total_file_size as u64;
    header.chunk_count = 1;
    header.chunk_table_offset = 64;
    header.spatial_origin = [0.0, 0.0];
    header.elevation_datum = [0.0, 1.0];
    header.update_crc();

    let mut entry = M64ChunkDirectoryEntry {
        chunk_uuid: [0x64; 16],
        spatial_hilbert_index: encode_hilbert3d_u64(0, 0, 0, 2),
        byte_offset: 128,
        byte_length: payload_bytes_len as u32,
        record_count: record_count as u32,
        srid_geodetic_code: 4326,
        lut_bank_epoch_id: 1,
        chunk_record_mode: M64_MODE_STRATA,
        compression_flags: 0,
        bao_root_hash_prefix: [0u8; 12],
        entry_crc32c: 0,
    };
    entry.update_crc();

    buffer[..64].copy_from_slice(bytemuck::bytes_of(&header));
    buffer[64..128].copy_from_slice(bytemuck::bytes_of(&entry));
    host.encode("m64", buffer)
}

impl AssetHost<'_> {
    /// Solves divergence-free pressure projection via Jacobi Poisson iterations,
    /// advances geodetic density via second-order MacCormack advection, and applies
    /// Eikonal fast-sweeping signed-distance redistancing over a `.m64` field grid.
    pub(crate) fn advect_m64_grid(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "grid_side",
                "steps",
                "dt",
                "wind_velocity",
                "poisson_iterations",
            ],
        )?;

        let grid_side = count(a, "grid_side", 32, 128)?;
        if grid_side < 8 {
            return Err("grid_side minimum is 8".into());
        }
        let steps = count(a, "steps", 6, 64)?;
        let dt = number(a, "dt", 0.1)?;
        if !(0.001..=1.0).contains(&dt) {
            return Err("dt outside 0.001..=1.0".into());
        }

        let wind = match a.get("wind_velocity") {
            Some(v) => vector::<2>(v)?,
            None => [1.2, 0.8],
        };
        let poisson_iterations = count(a, "poisson_iterations", 12, 64)?;

        let n = grid_side;
        self.charge(n * n * (steps + poisson_iterations + 8), n * n * 80 + 16384)?;
        let mut u = vec![0.0f64; n * n];
        let mut v = vec![0.0f64; n * n];
        let mut density = vec![0.0f64; n * n];

        // Initialize velocity with background wind + atmospheric vortex and density plume
        for y in 0..n {
            let fy = y as f64 / n as f64;
            for x in 0..n {
                let fx = x as f64 / n as f64;
                let i = y * n + x;
                let theta = 2.0 * PI;
                u[i] = wind[0] + 0.65 * (theta * fy).sin() * (theta * fx).cos();
                v[i] = wind[1] - 0.65 * (theta * fy).cos() * (theta * fx).sin();

                let cx = 0.35 * n as f64;
                let cy = 0.35 * n as f64;
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                let r2 = (dx * dx + dy * dy) / ((n as f64 * 0.18).powi(2));
                density[i] = (-r2).exp() + 0.15 * (4.0 * theta * fx).cos().max(0.0);
            }
        }

        let get_field = |arr: &[f64], x: i32, y: i32| -> f64 {
            let nx = ((x % n as i32 + n as i32) % n as i32) as usize;
            let ny = ((y % n as i32 + n as i32) % n as i32) as usize;
            arr[ny * n + nx]
        };

        // Compute divergence before Jacobi Poisson projection
        let mut div = vec![0.0f64; n * n];
        let mut max_divergence_before = 0.0f64;
        for y in 0..n as i32 {
            for x in 0..n as i32 {
                let d = (get_field(&u, x + 1, y) - get_field(&u, x - 1, y)
                    + get_field(&v, x, y + 1)
                    - get_field(&v, x, y - 1))
                    * 0.5;
                let idx = (y as usize) * n + (x as usize);
                div[idx] = d;
                if d.abs() > max_divergence_before {
                    max_divergence_before = d.abs();
                }
            }
        }

        // Jacobi Poisson solver for pressure
        let mut p = vec![0.0f64; n * n];
        let mut p_next = vec![0.0f64; n * n];
        for _ in 0..poisson_iterations {
            for y in 0..n as i32 {
                for x in 0..n as i32 {
                    let p_left = get_field(&p, x - 1, y);
                    let p_right = get_field(&p, x + 1, y);
                    let p_down = get_field(&p, x, y - 1);
                    let p_up = get_field(&p, x, y + 1);
                    let d = div[(y as usize) * n + (x as usize)];
                    p_next[(y as usize) * n + (x as usize)] =
                        0.25 * (p_left + p_right + p_down + p_up - d);
                }
            }
            std::mem::swap(&mut p, &mut p_next);
        }

        // Project velocity field to make it divergence-free
        for y in 0..n as i32 {
            for x in 0..n as i32 {
                let grad_x = (get_field(&p, x + 1, y) - get_field(&p, x - 1, y)) * 0.5;
                let grad_y = (get_field(&p, x, y + 1) - get_field(&p, x, y - 1)) * 0.5;
                let idx = (y as usize) * n + (x as usize);
                u[idx] -= grad_x;
                v[idx] -= grad_y;
            }
        }

        // Compute divergence after projection
        let mut max_divergence_after = 0.0f64;
        for y in 0..n as i32 {
            for x in 0..n as i32 {
                let d = (get_field(&u, x + 1, y) - get_field(&u, x - 1, y)
                    + get_field(&v, x, y + 1)
                    - get_field(&v, x, y - 1))
                    * 0.5;
                if d.abs() > max_divergence_after {
                    max_divergence_after = d.abs();
                }
            }
        }

        // MacCormack second-order advection of density
        for _ in 0..steps {
            let mut phi_star = vec![0.0f64; n * n];
            let mut phi_star_star = vec![0.0f64; n * n];

            // Forward step (Semi-Lagrangian)
            for y in 0..n {
                for x in 0..n {
                    let idx = y * n + x;
                    let back_x = x as f64 - dt * u[idx];
                    let back_y = y as f64 - dt * v[idx];
                    phi_star[idx] = sample_bilinear_2d(&density, n, back_x, back_y);
                }
            }

            // Backward step
            for y in 0..n {
                for x in 0..n {
                    let idx = y * n + x;
                    let fwd_x = x as f64 + dt * u[idx];
                    let fwd_y = y as f64 + dt * v[idx];
                    phi_star_star[idx] = sample_bilinear_2d(&phi_star, n, fwd_x, fwd_y);
                }
            }

            // MacCormack correction with BPS monotonic limiter
            for y in 0..n {
                for x in 0..n {
                    let idx = y * n + x;
                    let back_x = x as f64 - dt * u[idx];
                    let back_y = y as f64 - dt * v[idx];
                    let (min_val, max_val) = neighborhood_min_max(&density, n, back_x, back_y);
                    let corrected = phi_star[idx] + 0.5 * (density[idx] - phi_star_star[idx]);
                    density[idx] = corrected.clamp(min_val, max_val);
                }
            }
        }

        // Thermal slope relaxation of advected density (geomorphic erosion rung).
        let density_f32: Vec<f32> = density.iter().map(|d| (*d as f32).clamp(0.0, 1.0)).collect();
        let relaxed = terrain_erosion_solvers::thermal_shape(&density_f32, n, 0.15);
        for (slot, r) in density.iter_mut().zip(relaxed) {
            *slot = f64::from(r);
        }

        // Eikonal fast-sweeping signed-distance redistancing
        let iso = 0.40;
        let mut dist = vec![1e6f64; n * n];
        for y in 0..n as i32 {
            for x in 0..n as i32 {
                let idx = (y as usize) * n + (x as usize);
                let val = density[idx];
                let is_above = val >= iso;
                let mut is_interface = false;

                for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let neighbor = get_field(&density, x + dx, y + dy);
                    if (neighbor >= iso) != is_above {
                        is_interface = true;
                        break;
                    }
                }

                if is_interface {
                    let d = (val - iso).abs() / (val.abs() + 0.1);
                    dist[idx] = if is_above { d * 0.5 } else { -d * 0.5 };
                } else if !is_above {
                    dist[idx] = -1e6;
                }
            }
        }

        // Fast-sweeping 4 directional passes
        for pass in 0..4 {
            let x_range: Vec<i32> = if pass % 2 == 0 {
                (0..n as i32).collect()
            } else {
                (0..n as i32).rev().collect()
            };
            let y_range: Vec<i32> = if pass < 2 {
                (0..n as i32).collect()
            } else {
                (0..n as i32).rev().collect()
            };

            for &y in &y_range {
                for &x in &x_range {
                    let idx = (y as usize) * n + (x as usize);
                    if dist[idx].abs() < 0.25 {
                        continue;
                    }

                    let d_left = get_field(&dist, x - 1, y).abs();
                    let d_right = get_field(&dist, x + 1, y).abs();
                    let d_down = get_field(&dist, x, y - 1).abs();
                    let d_up = get_field(&dist, x, y + 1).abs();

                    let a = d_left.min(d_right);
                    let b = d_down.min(d_up);

                    let candidate = if (a - b).abs() >= 1.0 {
                        a.min(b) + 1.0
                    } else {
                        0.5 * (a + b + (2.0 - (a - b) * (a - b)).max(0.0).sqrt())
                    };

                    let sign = if dist[idx] >= 0.0 { 1.0 } else { -1.0 };
                    if candidate < dist[idx].abs() {
                        dist[idx] = sign * candidate;
                    }
                }
            }
        }

        // Color-mapped visualization RGBA8
        let mut pixels = Vec::with_capacity(n * n);
        for y in 0..n {
            for x in 0..n {
                let idx = y * n + x;
                let dens = density[idx].clamp(0.0, 1.25);
                let d = dist[idx];

                // Palette: deep ocean blue -> emerald -> amber -> snow white
                let (mut r, mut g, mut b) = if dens < 0.2 {
                    let t = dens / 0.2;
                    (
                        (15.0 * (1.0 - t) + 30.0 * t) as u8,
                        (32.0 * (1.0 - t) + 120.0 * t) as u8,
                        (67.0 * (1.0 - t) + 210.0 * t) as u8,
                    )
                } else if dens < 0.5 {
                    let t = (dens - 0.2) / 0.3;
                    (
                        (30.0 * (1.0 - t) + 46.0 * t) as u8,
                        (120.0 * (1.0 - t) + 204.0 * t) as u8,
                        (210.0 * (1.0 - t) + 113.0 * t) as u8,
                    )
                } else if dens < 0.8 {
                    let t = (dens - 0.5) / 0.3;
                    (
                        (46.0 * (1.0 - t) + 241.0 * t) as u8,
                        (204.0 * (1.0 - t) + 196.0 * t) as u8,
                        (113.0 * (1.0 - t) + 15.0 * t) as u8,
                    )
                } else {
                    let t = ((dens - 0.8) / 0.45).clamp(0.0, 1.0);
                    (
                        (241.0 * (1.0 - t) + 252.0 * t) as u8,
                        (196.0 * (1.0 - t) + 252.0 * t) as u8,
                        (15.0 * (1.0 - t) + 255.0 * t) as u8,
                    )
                };

                // Modulate isoline contours from redistanced signed-distance field
                if (d * 8.0).sin().abs() > 0.94 {
                    r = (r as f64 * 0.8).round() as u8;
                    g = (g as f64 * 0.8).round() as u8;
                    b = (b as f64 * 0.8).round() as u8;
                }

                pixels.push([r, g, b, 255]);
            }
        }

        let preview_rgba = self.store_rgba(n as u32, n as u32, pixels)?;
        let mean_advected_density = density.iter().sum::<f64>() / (n * n) as f64;
        let m64 = encode_advected_strata_m64(self, n, &density, &u, &v)?;

        let last = n - 1;
        let samples = [
            SamplePoint::new(0.0, 0.0, density[0]),
            SamplePoint::new(last as f64, 0.0, density[last]),
            SamplePoint::new(0.0, last as f64, density[last * n]),
            SamplePoint::new(last as f64, last as f64, density[last * n + last]),
        ];
        let vparams = VariogramParams::new(VariogramModel::Spherical, 0.0, 1.0, n as f64);
        let krig = predict_kriging(&samples, Coord::new(n as f64 * 0.5, n as f64 * 0.5), &vparams);

        let mut rec = BTreeMap::new();
        rec.insert("preview_rgba".into(), preview_rgba);
        rec.insert(
            "max_divergence_before".into(),
            Value::F64(max_divergence_before),
        );
        rec.insert(
            "max_divergence_after".into(),
            Value::F64(max_divergence_after),
        );
        rec.insert(
            "mean_advected_density".into(),
            Value::F64(mean_advected_density),
        );
        rec.insert("grid_side".into(), Value::U64(n as u64));
        rec.insert("m64".into(), m64);
        rec.insert("record_mode".into(), Value::String("strata".into()));
        if let Some(k) = krig {
            rec.insert("kriging_estimate".into(), Value::F64(k.estimate));
            rec.insert("kriging_variance".into(), Value::F64(k.variance));
        }

        Ok(Value::Record(rec))
    }

    /// Evaluates Bekker / Mohr-Coulomb terramechanics pressure-sinkage, tractive shear stress,
    /// compaction resistance, and coupled Archimedes + Gerstner wave hydrodynamic buoyancy.
    pub(crate) fn couple_continuum_xpbd(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "wheel_load_n",
                "contact_width_m",
                "contact_length_m",
                "soil_kc",
                "soil_kphi",
                "soil_n",
                "soil_cohesion_pa",
                "soil_friction_deg",
                "submerged_volume_m3",
                "wave_amplitude_m",
            ],
        )?;

        let wheel_load_n = number(a, "wheel_load_n", 4500.0)?;
        if !(10.0..=500_000.0).contains(&wheel_load_n) {
            return Err("wheel_load_n outside 10.0..=500000.0".into());
        }

        let contact_width_m = number(a, "contact_width_m", 0.35)?;
        if !(0.05..=5.0).contains(&contact_width_m) {
            return Err("contact_width_m outside 0.05..=5.0".into());
        }

        let contact_length_m = number(a, "contact_length_m", 0.45)?;
        if !(0.05..=5.0).contains(&contact_length_m) {
            return Err("contact_length_m outside 0.05..=5.0".into());
        }

        let soil_kc = number(a, "soil_kc", 15000.0)?;
        if soil_kc < 0.0 {
            return Err("soil_kc must be non-negative".into());
        }

        let soil_kphi = number(a, "soil_kphi", 850000.0)?;
        if soil_kphi < 0.0 {
            return Err("soil_kphi must be non-negative".into());
        }

        let soil_n = number(a, "soil_n", 0.8)?;
        if !(0.05..=3.0).contains(&soil_n) {
            return Err("soil_n outside 0.05..=3.0".into());
        }

        let soil_cohesion_pa = number(a, "soil_cohesion_pa", 12000.0)?;
        if soil_cohesion_pa < 0.0 {
            return Err("soil_cohesion_pa must be non-negative".into());
        }

        let soil_friction_deg = number(a, "soil_friction_deg", 31.0)?;
        if !(0.0..=85.0).contains(&soil_friction_deg) {
            return Err("soil_friction_deg outside 0.0..=85.0".into());
        }

        let submerged_volume_m3 = number(a, "submerged_volume_m3", 0.65)?;
        if !(0.0..=1000.0).contains(&submerged_volume_m3) {
            return Err("submerged_volume_m3 outside 0.0..=1000.0".into());
        }

        let wave_amplitude_m = number(a, "wave_amplitude_m", 0.4)?;
        if !(0.0..=20.0).contains(&wave_amplitude_m) {
            return Err("wave_amplitude_m outside 0.0..=20.0".into());
        }

        let b = contact_width_m;
        let l = contact_length_m;
        let patch_area = b * l;
        let sigma = wheel_load_n / patch_area;

        // Effective Bekker modulus k_eq = (kc / b) + kphi
        let k_eq = (soil_kc / b) + soil_kphi;
        if k_eq <= 0.0 {
            return Err("Effective soil modulus (kc/b + kphi) must be positive".into());
        }

        // Bekker sinkage depth z = (sigma / k_eq)^(1 / n)
        let sinkage_m = (sigma / k_eq).powf(1.0 / soil_n);

        // Mohr-Coulomb maximum shear stress: tau_max = c + sigma * tan(phi)
        let phi_rad = soil_friction_deg * (PI / 180.0);
        let max_shear_pa = soil_cohesion_pa + sigma * phi_rad.tan();

        // Bekker compaction resistance: Rc = (b / (n + 1)) * k_eq * z^(n + 1)
        let compaction_resistance_n = (b / (soil_n + 1.0)) * k_eq * sinkage_m.powf(soil_n + 1.0);

        // Maximum gross tractive effort: H = A * tau_max
        let gross_traction_n = patch_area * max_shear_pa;

        // Net drawbar pull: DP = H - Rc
        let drawbar_pull_n = gross_traction_n - compaction_resistance_n;

        // Hydrodynamic Archimedes + Gerstner wave dynamic buoyancy force:
        // Ocean water density = 1025.0 kg/m³, standard gravity = 9.80665 m/s²
        const WATER_DENSITY_KG_M3: f64 = 1025.0;
        const GRAVITY_M_S2: f64 = 9.80665;
        let fb_static = WATER_DENSITY_KG_M3 * GRAVITY_M_S2 * submerged_volume_m3;
        let char_length = submerged_volume_m3.cbrt().max(0.1);
        let wave_dynamic_factor = 1.0 + (wave_amplitude_m * 0.15) / (char_length + 0.5);
        let buoyancy_force_n = fb_static * wave_dynamic_factor;

        let mut rec = BTreeMap::new();
        rec.insert("sinkage_m".into(), Value::F64(sinkage_m));
        rec.insert("max_shear_pa".into(), Value::F64(max_shear_pa));
        rec.insert("drawbar_pull_n".into(), Value::F64(drawbar_pull_n));
        rec.insert(
            "compaction_resistance_n".into(),
            Value::F64(compaction_resistance_n),
        );
        rec.insert("buoyancy_force_n".into(), Value::F64(buoyancy_force_n));

        Ok(Value::Record(rec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::q42::m64_matter::{
        has_m64_magic, M64PayloadInterval, M64ReadLimits, M64Reader, M64StrataColumnRecord,
        M64_MODE_STRATA,
    };
    use std::sync::atomic::AtomicBool;

    fn make_host<'a>(limits: &'a ToolLimits, cancel: &'a AtomicBool) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn advect_m64_grid_strata_kriging_erosion_two_produces_match() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_host(&limits, &cancel);

        let mut args = BTreeMap::new();
        args.insert("grid_side".into(), Value::U64(8));
        args.insert("steps".into(), Value::U64(2));
        args.insert("dt".into(), Value::F64(0.08));
        args.insert("poisson_iterations".into(), Value::U64(8));

        let a = host
            .advect_m64_grid(&args)
            .expect("advect a");
        let b = host
            .advect_m64_grid(&args)
            .expect("advect b");
        let Value::Record(ra) = a else { panic!("record") };
        let Value::Record(rb) = b else { panic!("record") };

        assert_eq!(ra.get("grid_side"), rb.get("grid_side"));
        assert_eq!(ra.get("max_divergence_before"), rb.get("max_divergence_before"));
        assert_eq!(ra.get("max_divergence_after"), rb.get("max_divergence_after"));
        assert_eq!(ra.get("mean_advected_density"), rb.get("mean_advected_density"));
        assert_eq!(ra.get("kriging_estimate"), rb.get("kriging_estimate"));
        assert_eq!(ra.get("record_mode"), Some(&Value::String("strata".into())));

        let estimate = match ra.get("kriging_estimate") {
            Some(Value::F64(v)) => *v,
            _ => panic!("kriging_estimate object"),
        };
        let variance = match ra.get("kriging_variance") {
            Some(Value::F64(v)) => *v,
            _ => panic!("kriging_variance object"),
        };
        assert!(estimate.is_finite(), "advected kriging estimate");
        assert!(variance >= 0.0);

        let m64_a = host
            .workspace
            .encoded(ra.get("m64").expect("m64 handle"))
            .expect("encoded m64 a");
        let m64_b = host
            .workspace
            .encoded(rb.get("m64").expect("m64 handle"))
            .expect("encoded m64 b");
        assert_eq!(m64_a.kind, "m64");
        assert_eq!(m64_a.bytes, m64_b.bytes);
        assert!(has_m64_magic(&m64_a.bytes));

        let limits_r = M64ReadLimits {
            max_file_bytes: m64_a.bytes.len() + 1024,
            max_chunks: 4,
            max_records: 8192,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        let reader = M64Reader::parse(&m64_a.bytes, limits_r, &mut scratch).expect("STRATA parse");
        assert_eq!(
            reader.chunk(0).unwrap().directory_entry().chunk_record_mode,
            M64_MODE_STRATA
        );
        assert_eq!(reader.verification().record_crc_checked, 64);
        let rec: M64StrataColumnRecord =
            bytemuck::pod_read_unaligned(&m64_a.bytes[128..192]);
        rec.validate().expect("advected STRATA CRC");
        assert!(rec.layer_thickness_mm.iter().all(|&t| t > 0));
    }
}

//! Offline receive-only scalar multipath in a fixed virtual classroom.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::f64::consts::{PI, TAU};

use qualia_core_db::{
    q42::field_spectrum::VACUUM_SPEED_OF_LIGHT_M_PER_S as C,
    specialized_libs::computational_geometry::{Point2, poisson_disk_2d},
};
use serde_json::json;

use crate::{Args, AssetHost, Value, fields, integer, required, text};

use super::profile_result;

const ROOM_M: [f64; 3] = [8.0, 10.0, 3.2];
const SOURCE_M: [f64; 3] = [2.0, 3.0, 1.55];
const PROBE_Z_M: f64 = 1.20;
const MIN_PROBE_SEPARATION_M: f64 = 0.30;
const POISSON_ATTEMPTS: usize = 24;
const CENTER_HZ: f64 = 2.4e9;
const SPAN_HZ: f64 = 20e6;
const WALLS: [&str; 7] = [
    "direct", "west", "east", "south", "north", "floor", "ceiling",
];

impl AssetHost<'_> {
    pub(crate) fn trace_rf_multipath_and_radar(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "surface_model",
                "probe_count",
                "frequency_bins",
                "seed",
            ],
        )?;
        if text(required(args, "mode")?)? != "synthetic_classroom_receive_only" {
            return Err("Expected synthetic_classroom_receive_only mode".into());
        }
        let surface_model = text(required(args, "surface_model")?)?;
        let reflections = match surface_model {
            "free_space" => [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "floor_reflection" => [1.0, 0.0, 0.0, 0.0, 0.0, -0.55, 0.0],
            "six_surfaces" => [1.0, -0.35, -0.35, -0.25, -0.25, -0.55, -0.20],
            _ => return Err("Expected free_space, floor_reflection or six_surfaces".into()),
        };
        let probe_count = integer(required(args, "probe_count")?, "probe_count")?;
        let bins = integer(required(args, "frequency_bins")?, "frequency_bins")?;
        if !(1..=128).contains(&probe_count) || !(1..=128).contains(&bins) {
            return Err("Probe and frequency bin counts must be within 1..=128".into());
        }
        let seed = match args.get("seed") {
            None => 42,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            Some(_) => return Err("seed must be a nonnegative integer".into()),
        };
        let min = Point2::new(0.75, 0.75);
        let max = Point2::new(ROOM_M[0] - 0.75, ROOM_M[1] - 0.75);
        let cell = MIN_PROBE_SEPARATION_M / std::f64::consts::SQRT_2;
        let grid_cells =
            (((max.x - min.x) / cell).ceil() as usize) * (((max.y - min.y) / cell).ceil() as usize);
        self.charge(
            grid_cells + 2 * probe_count * POISSON_ATTEMPTS * 25 + probe_count * bins * 256,
            grid_cells * 4 + probe_count * 640 + probe_count * bins * 16,
        )?;
        let mut grid = vec![0; grid_cells];
        let mut active = vec![0; probe_count];
        let mut probes = vec![Point2::default(); probe_count];
        let emitted = poisson_disk_2d(
            min,
            max,
            MIN_PROBE_SEPARATION_M,
            seed,
            POISSON_ATTEMPTS,
            &mut grid,
            &mut active,
            &mut probes,
        )
        .map_err(|error| format!("Native classroom blue-noise probe sampler: {error:?}"))?;
        if emitted != probe_count {
            return Err("Virtual classroom probe sampler exhausted its domain".into());
        }
        self.charge(0, 0)?;

        let frequencies: Vec<f64> = (0..bins)
            .map(|at| {
                if bins == 1 {
                    CENTER_HZ
                } else {
                    CENTER_HZ + SPAN_HZ * (at as f64 / (bins - 1) as f64 - 0.5)
                }
            })
            .collect();
        let mut positions = Vec::with_capacity(probe_count);
        let mut path_records = Vec::with_capacity(probe_count);
        let mut delay_spread = Vec::with_capacity(probe_count);
        let mut samples = Vec::with_capacity(probe_count * bins * 4);
        let images = image_sources();
        for (index, probe) in probes.iter().enumerate() {
            if index % 16 == 0 {
                self.charge(0, 0)?;
            }
            let position = [probe.x, probe.y, PROBE_Z_M];
            positions.push(position);
            let paths = images
                .iter()
                .zip(reflections)
                .zip(WALLS)
                .filter(|&((_, gain), _)| gain != 0.0)
                .map(|((&image, gain), wall)| {
                    let length = (image[0] - position[0])
                        .hypot(image[1] - position[1])
                        .hypot(image[2] - position[2]);
                    Path {
                        wall,
                        length_m: length,
                        delay_s: length / C,
                        gain_per_m: gain / length,
                    }
                })
                .collect::<Vec<_>>();
            let total_power: f64 = paths.iter().map(|p| p.gain_per_m.powi(2)).sum();
            let mean_delay = paths
                .iter()
                .map(|p| p.gain_per_m.powi(2) * p.delay_s)
                .sum::<f64>()
                / total_power;
            let spread = (paths
                .iter()
                .map(|p| p.gain_per_m.powi(2) * (p.delay_s - mean_delay).powi(2))
                .sum::<f64>()
                / total_power)
                .sqrt();
            delay_spread.push(spread);
            for &frequency in &frequencies {
                let mut real = 0.0;
                let mut imaginary = 0.0;
                for path in &paths {
                    let (sin, cos) = (-TAU * frequency * path.delay_s).sin_cos();
                    real += path.gain_per_m * cos;
                    imaginary += path.gain_per_m * sin;
                }
                // Unit-free isotropic voltage transfer; the squared magnitude
                // is a received-to-reference power ratio for this scalar model.
                let scale = C / (4.0 * PI * frequency);
                let real = scale * real;
                let imaginary = scale * imaginary;
                let coherent_power = real * real + imaginary * imaginary;
                let incoherent_power = scale * scale * total_power;
                samples.extend([
                    real as f32,
                    imaginary as f32,
                    coherent_power as f32,
                    incoherent_power as f32,
                ]);
            }
            path_records.push(
                paths
                    .into_iter()
                    .map(|path| {
                        json!({
                            "wall": path.wall, "length_m": path.length_m, "delay_s": path.delay_s,
                            "real_reflection_over_path_length_m_inv": path.gain_per_m,
                        })
                    })
                    .collect::<Vec<_>>(),
            );
        }
        profile_result(
            self,
            "synthetic-classroom-rf",
            vec![probe_count as u32, bins as u32, 4],
            samples,
            json!({
                "mode": "synthetic_classroom_receive_only", "surface_model": surface_model,
                "phase_convention": "exp(-i * 2pi * f * delay)", "reflection_model": "scalar real first-order specular reflections",
                "axes": ["receiver", "frequency", "channel"],
                "channels": ["voltage_transfer_real", "voltage_transfer_imaginary", "coherent_power_ratio", "incoherent_power_ratio"],
                "room_size_xyz_m": ROOM_M, "virtual_source_xyz_m": SOURCE_M, "wave_speed_m_s": C,
                "frequencies_hz": frequencies, "probe_positions_xyz_m": positions, "rms_delay_spread_s": delay_spread,
                "sample_seed": seed, "minimum_probe_separation_m": MIN_PROBE_SEPARATION_M, "paths_by_probe": path_records,
            }),
        )
    }

    pub(crate) fn reconstruct_sar_isar_tomography(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "surface_model",
                "probe_count",
                "frequency_bins",
                "delay_bins",
                "seed",
            ],
        )?;
        if text(required(args, "mode")?)? != "recover_synthetic_classroom_delay_profile" {
            return Err("Expected recover_synthetic_classroom_delay_profile mode".into());
        }
        let surface_model = match args.get("surface_model") {
            Some(v) => text(v)?,
            None => "six_surfaces",
        };
        let reflections = match surface_model {
            "free_space" => [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            "floor_reflection" => [1.0, 0.0, 0.0, 0.0, 0.0, -0.55, 0.0],
            "six_surfaces" => [1.0, -0.35, -0.35, -0.25, -0.25, -0.55, -0.20],
            _ => return Err("Expected free_space, floor_reflection or six_surfaces".into()),
        };
        let probe_count = match args.get("probe_count") {
            Some(v) => integer(v, "probe_count")?,
            None => 8,
        };
        let bins = match args.get("frequency_bins") {
            Some(v) => integer(v, "frequency_bins")?,
            None => 64,
        };
        let delay_bins = match args.get("delay_bins") {
            Some(v) => integer(v, "delay_bins")?,
            None => 64,
        };
        if !(1..=128).contains(&probe_count)
            || !(8..=128).contains(&bins)
            || !(16..=256).contains(&delay_bins)
        {
            return Err(
                "probe_count must be 1..=128, frequency_bins 8..=128, and delay_bins 16..=256"
                    .into(),
            );
        }
        let seed = match args.get("seed") {
            None => 42,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            Some(_) => return Err("seed must be a nonnegative integer".into()),
        };
        let min = Point2::new(0.75, 0.75);
        let max = Point2::new(ROOM_M[0] - 0.75, ROOM_M[1] - 0.75);
        let cell = MIN_PROBE_SEPARATION_M / std::f64::consts::SQRT_2;
        let grid_cells =
            (((max.x - min.x) / cell).ceil() as usize) * (((max.y - min.y) / cell).ceil() as usize);
        self.charge(
            grid_cells + probe_count * bins * delay_bins * 32,
            grid_cells * 4 + probe_count * delay_bins * 16,
        )?;
        let mut grid = vec![0; grid_cells];
        let mut active = vec![0; probe_count];
        let mut probes = vec![Point2::default(); probe_count];
        let emitted = poisson_disk_2d(
            min,
            max,
            MIN_PROBE_SEPARATION_M,
            seed,
            POISSON_ATTEMPTS,
            &mut grid,
            &mut active,
            &mut probes,
        )
        .map_err(|error| format!("Native classroom blue-noise probe sampler: {error:?}"))?;
        if emitted != probe_count {
            return Err("Virtual classroom probe sampler exhausted its domain".into());
        }

        let images = image_sources();
        let mut samples = Vec::with_capacity(probe_count * delay_bins * 3);
        let mut recovered_records = Vec::with_capacity(probe_count);
        let mut max_direct_length_error_m = 0.0_f64;

        for (index, probe) in probes.iter().enumerate() {
            if index % 8 == 0 {
                self.charge(0, 0)?;
            }
            let position = [probe.x, probe.y, PROBE_Z_M];
            let paths = images
                .iter()
                .zip(reflections)
                .zip(WALLS)
                .filter(|&((_, gain), _)| gain != 0.0)
                .map(|((&image, gain), wall)| {
                    let length = (image[0] - position[0])
                        .hypot(image[1] - position[1])
                        .hypot(image[2] - position[2]);
                    Path {
                        wall,
                        length_m: length,
                        delay_s: length / C,
                        gain_per_m: gain / length,
                    }
                })
                .collect::<Vec<_>>();
            let direct_length_m = paths[0].length_m;
            let min_len_m = (direct_length_m - 1.5).max(0.25);
            let max_len_m = paths
                .iter()
                .map(|p| p.length_m)
                .fold(direct_length_m, f64::max)
                + 1.5;
            let step_len_m = (max_len_m - min_len_m) / (delay_bins - 1) as f64;
            let recovery_span_hz = C / (2.0 * step_len_m);
            let recovery_frequencies: Vec<f64> = (0..bins)
                .map(|at| CENTER_HZ + recovery_span_hz * (at as f64 / (bins - 1) as f64 - 0.5))
                .collect();

            // Forward classroom frequency response H(f) with carrier-compensated matched spectrum.
            let spectrum: Vec<[f64; 2]> = recovery_frequencies
                .iter()
                .map(|&frequency| {
                    let mut re = 0.0;
                    let mut im = 0.0;
                    for path in &paths {
                        let (sin, cos) = (-TAU * frequency * path.delay_s).sin_cos();
                        re += path.gain_per_m * cos;
                        im += path.gain_per_m * sin;
                    }
                    [re, im]
                })
                .collect();

            // Coherent delay-domain matched correlation centered on the direct classroom path.
            let mut peak_power = -1.0_f64;
            let mut peak_bin = 0usize;
            let mut bin_powers = Vec::with_capacity(delay_bins);
            for k in 0..delay_bins {
                let candidate_len_m = min_len_m + k as f64 * step_len_m;
                let candidate_delay_s = candidate_len_m / C;
                let mut acc_re = 0.0;
                let mut acc_im = 0.0;
                for (&frequency, &[s_re, s_im]) in recovery_frequencies.iter().zip(&spectrum) {
                    let (sin, cos) = (TAU * frequency * candidate_delay_s).sin_cos();
                    acc_re += s_re * cos - s_im * sin;
                    acc_im += s_re * sin + s_im * cos;
                }
                let norm = 1.0 / bins as f64;
                let re = acc_re * norm;
                let im = acc_im * norm;
                let power = re * re + im * im;
                if power > peak_power {
                    peak_power = power;
                    peak_bin = k;
                }
                bin_powers.push(power);
                samples.extend([re as f32, im as f32, power as f32]);
            }
            let recovered_direct_len_m = min_len_m + peak_bin as f64 * step_len_m;
            let length_error_m = (recovered_direct_len_m - direct_length_m).abs();
            if length_error_m > step_len_m + 1e-9 {
                return Err(
                    "Classroom delay recovery peak drifted outside the direct-path resolution cell"
                        .into(),
                );
            }
            max_direct_length_error_m = max_direct_length_error_m.max(length_error_m);
            let sidelobe_power = bin_powers
                .iter()
                .enumerate()
                .filter(|&(k, _)| k.abs_diff(peak_bin) > 2)
                .map(|(_, &p)| p)
                .fold(1e-30_f64, f64::max);
            let pslr_db = 10.0 * (sidelobe_power / peak_power.max(1e-30)).log10();
            recovered_records.push(json!({
                "probe_xyz_m": position,
                "true_direct_length_m": direct_length_m,
                "recovered_direct_length_m": recovered_direct_len_m,
                "direct_length_error_m": length_error_m,
                "peak_delay_s": recovered_direct_len_m / C,
                "peak_power": peak_power,
                "pslr_db": pslr_db,
            }));
        }

        let nominal_delay_resolution_s = 1.0 / SPAN_HZ;
        profile_result(
            self,
            "synthetic-classroom-delay-recovery",
            vec![probe_count as u32, delay_bins as u32, 3],
            samples,
            json!({
                "mode": "recover_synthetic_classroom_delay_profile",
                "surface_model": surface_model,
                "axes": ["receiver", "delay_bin", "channel"],
                "channels": ["matched_real", "matched_imaginary", "matched_power"],
                "room_size_xyz_m": ROOM_M,
                "virtual_source_xyz_m": SOURCE_M,
                "wave_speed_m_s": C,
                "center_frequency_hz": CENTER_HZ,
                "bandwidth_hz": SPAN_HZ,
                "nominal_delay_resolution_s": nominal_delay_resolution_s,
                "max_direct_length_error_m": max_direct_length_error_m,
                "probes": recovered_records,
            }),
        )
    }
}

struct Path {
    wall: &'static str,
    length_m: f64,
    delay_s: f64,
    gain_per_m: f64,
}

fn image_sources() -> [[f64; 3]; 7] {
    let mut images = [SOURCE_M; 7];
    for (axis, &extent) in ROOM_M.iter().enumerate() {
        images[1 + axis * 2][axis] = -SOURCE_M[axis];
        images[2 + axis * 2][axis] = 2.0 * extent - SOURCE_M[axis];
    }
    images
}

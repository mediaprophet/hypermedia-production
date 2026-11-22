//! Analytic, explicitly scoped cryosphere, water-quality and geotechnical profiles.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::f64::consts::{PI, TAU};

use qualia_core_db::solvers::statistics::distributions::special::erf;
use serde_json::json;

use crate::{Args, AssetHost, Value, fields, list, num, record, required, text};

use super::{bounded, profile_result};

const MAX_AXIS: usize = 4096;

impl AssetHost<'_> {
    pub(crate) fn solve_glaciology_and_permafrost(&mut self, args: &Args) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "stefan_one_phase" => stefan(self, args),
            "glen_shallow_ice_column" => shallow_ice(self, args),
            _ => Err("Expected stefan_one_phase or glen_shallow_ice_column".into()),
        }
    }

    pub(crate) fn solve_limnology_estuarine_biogeochemistry(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "residence_times_s",
                "saturation_o2_mg_l",
                "initial_o2_mg_l",
                "initial_bod_mg_l",
                "bod_removal_s_inv",
                "deoxygenation_s_inv",
                "reaeration_s_inv",
            ],
        )?;
        if text(required(args, "mode")?)? != "streeter_phelps" {
            return Err("The supported oxygen balance mode is streeter_phelps".into());
        }
        let time = axis(args, "residence_times_s", 0.0, 1e10)?;
        self.charge(time.len() * 28, time.len() * 28)?;
        let saturation = bounded(args, "saturation_o2_mg_l", 0.01, 25.0)?;
        let initial_o2 = bounded(args, "initial_o2_mg_l", 0.0, 25.0)?;
        let bod0 = bounded(args, "initial_bod_mg_l", 0.0, 1e6)?;
        let removal = bounded(args, "bod_removal_s_inv", 0.0, 1.0)?;
        let deoxygenation = bounded(args, "deoxygenation_s_inv", 0.0, removal)?;
        let reaeration = bounded(args, "reaeration_s_inv", 0.0, 1.0)?;

        // EPA 600/2-78-001, equations 1 and 2. The convolution form stays finite at
        // coincident rate constants and permits BOD removal by sedimentation.
        let common_rate = removal.min(reaeration);
        let gap = (removal - reaeration).abs();
        let normalized_state = |t: f64| {
            let convolution = if gap == 0.0 {
                t
            } else {
                -(-gap * t).exp_m1() / gap
            };
            let deficit = (saturation - initial_o2) * (-(reaeration - common_rate) * t).exp()
                + deoxygenation * bod0 * convolution;
            let bod = bod0 * (-(removal - common_rate) * t).exp();
            (deficit, bod)
        };
        let evaluate = |t: f64| {
            let (deficit, bod) = normalized_state(t);
            let damping = (-common_rate * t).exp();
            (deficit * damping, bod * damping)
        };
        let rate = |t| {
            let (deficit, bod) = normalized_state(t);
            deoxygenation * bod - reaeration * deficit
        };
        let end = time[time.len() - 1];
        let mut boundary = 0.0;
        let mut extrema = vec![0.0, end];
        if rate(0.0) > 0.0 && rate(end) < 0.0 {
            let mut upper = end;
            for _ in 0..72 {
                let midpoint = 0.5 * (boundary + upper);
                if rate(midpoint) > 0.0 {
                    boundary = midpoint;
                } else {
                    upper = midpoint;
                }
            }
            extrema.push(0.5 * (boundary + upper));
        }
        let sag_time = extrema
            .into_iter()
            .max_by(|&a, &b| evaluate(a).0.total_cmp(&evaluate(b).0))
            .unwrap_or(0.0);
        let min_o2 = saturation - evaluate(sag_time).0;
        if min_o2 < -1e-9 {
            return Err("Streeter-Phelps solution leaves the oxygenated model domain before the final travel time".into());
        }
        let mut samples = Vec::with_capacity(time.len() * 3);
        for &t in &time {
            let (deficit, bod) = evaluate(t);
            samples.extend([
                (saturation - deficit).max(0.0) as f32,
                bod as f32,
                deficit as f32,
            ]);
        }
        profile_result(
            self,
            "streeter-phelps",
            vec![time.len() as u32, 3],
            samples,
            json!({
                "model": "streeter_phelps", "axes": ["residence_time_s", "channel"], "residence_times_s": time,
                "channels": ["dissolved_o2_mg_l", "remaining_bod_mg_l", "o2_deficit_mg_l"],
                "boundary": {"saturation_o2_mg_l": saturation, "initial_o2_mg_l": initial_o2, "initial_bod_mg_l": bod0},
                "rates_s_inv": {"bod_removal": removal, "deoxygenation": deoxygenation, "reaeration": reaeration},
                "peak_deficit_time_s": sag_time, "minimum_o2_mg_l": min_o2.max(0.0),
            }),
        )
    }

    pub(crate) fn solve_geotechnical_slope_and_tunnel(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "peck_gaussian" => peck(self, args),
            "bishop_simplified" => bishop(self, args),
            _ => Err("Expected peck_gaussian or bishop_simplified".into()),
        }
    }
}

fn axis(args: &Args, key: &str, low: f64, high: f64) -> Result<Vec<f64>, String> {
    let contents = list(required(args, key)?)?;
    if contents.is_empty() || contents.len() > MAX_AXIS {
        return Err(format!("{key} must have 1..={MAX_AXIS} coordinates"));
    }
    let samples: Vec<f64> = contents.iter().map(num).collect::<Result<_, _>>()?;
    if samples.iter().any(|v| !(low..=high).contains(v)) || samples.windows(2).any(|v| v[0] >= v[1])
    {
        return Err(format!(
            "{key} must be strictly increasing and within [{low}, {high}]"
        ));
    }
    Ok(samples)
}

fn stefan(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &[
            "mode",
            "times_s",
            "depths_m",
            "phase_temperature_k",
            "surface_temperature_k",
            "thermal_diffusivity_m2_s",
            "specific_heat_j_kg_k",
            "latent_heat_j_kg",
        ],
    )?;
    let time = axis(args, "times_s", 0.0, 1e10)?;
    let depth = axis(args, "depths_m", 0.0, 1e4)?;
    let cells = time.len() * depth.len();
    if cells > 1_048_576 {
        return Err("Stefan profile exceeds 1,048,576 samples".into());
    }
    host.charge(cells * 32, cells * 4 + time.len() * 16 + depth.len() * 8)?;
    let phase = bounded(args, "phase_temperature_k", 180.0, 300.0)?;
    let surface = bounded(args, "surface_temperature_k", 150.0, 350.0)?;
    let kappa = bounded(args, "thermal_diffusivity_m2_s", 1e-10, 1e-3)?;
    let heat_capacity = bounded(args, "specific_heat_j_kg_k", 1.0, 1e4)?;
    let latent_heat = bounded(args, "latent_heat_j_kg", 1e3, 1e7)?;
    let stefan_number = heat_capacity * (surface - phase).abs() / latent_heat;
    if stefan_number <= 0.0 {
        return Err("The surface and phase temperatures must differ".into());
    }
    let (mut low, mut high) = (0.0_f64, 6.0_f64);
    for _ in 0..72 {
        let lambda = (low + high) * 0.5;
        let ste = PI.sqrt() * lambda * (lambda * lambda).exp() * erf(lambda);
        if ste < stefan_number {
            low = lambda;
        } else {
            high = lambda;
        }
    }
    let lambda = (low + high) * 0.5;
    let normalization = erf(lambda);
    let mut front_depth_m = Vec::with_capacity(time.len());
    let mut samples = Vec::with_capacity(cells);
    for &t in &time {
        let length = 2.0 * (kappa * t).sqrt();
        let front = lambda * length;
        front_depth_m.push(front);
        for &z in &depth {
            let temperature = if z == 0.0 {
                surface
            } else if z >= front {
                phase
            } else {
                surface + (phase - surface) * erf(z / length) / normalization
            };
            samples.push(temperature as f32);
        }
    }
    profile_result(
        host,
        "stefan-one-phase",
        vec![time.len() as u32, depth.len() as u32],
        samples,
        json!({
            "model": "stefan_one_phase", "transition": if surface > phase {"thaw"} else {"freeze"},
            "axes": ["time_s", "depth_m"], "unit": "K", "times_s": time, "depths_m": depth, "front_depth_m": front_depth_m,
            "lambda": lambda, "stefan_number": stefan_number, "thermal_diffusivity_m2_s": kappa,
            "phase_temperature_k": phase, "surface_temperature_k": surface,
            "specific_heat_j_kg_k": heat_capacity, "latent_heat_j_kg": latent_heat,
        }),
    )
}

fn shallow_ice(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &[
            "mode",
            "heights_above_bed_m",
            "thickness_m",
            "surface_slope_tangent",
            "density_kg_m3",
            "glen_exponent",
            "glen_rate_pa_to_minus_n_s_inv",
            "basal_sliding_m_yr",
        ],
    )?;
    let thickness = bounded(args, "thickness_m", 0.1, 5000.0)?;
    let height = axis(args, "heights_above_bed_m", 0.0, thickness)?;
    let slope = bounded(args, "surface_slope_tangent", 0.0, 0.3)?;
    let density = bounded(args, "density_kg_m3", 600.0, 1000.0)?;
    let exponent = bounded(args, "glen_exponent", 1.0, 5.0)?;
    let flow = bounded(args, "glen_rate_pa_to_minus_n_s_inv", 1e-42, 1e-5)?;
    let sliding = bounded(args, "basal_sliding_m_yr", 0.0, 20_000.0)?;
    host.charge(height.len() * 40, height.len() * 24)?;
    let driving = density * 9.80665 * slope;
    let n_plus_one = exponent + 1.0;
    let surface_deformation =
        2.0 * flow * (driving * thickness).powf(exponent) * thickness / n_plus_one * 31_557_600.0;
    let surface_velocity = sliding + surface_deformation;
    if !surface_velocity.is_finite() || surface_velocity > 20_000.0 {
        return Err("Computed surface velocity exceeds the supported 20,000 m/year".into());
    }
    let samples = height
        .iter()
        .flat_map(|z| {
            let column_fraction = z / thickness;
            let strain_velocity = if column_fraction == 1.0 {
                surface_deformation
            } else {
                surface_deformation * -(n_plus_one * (-column_fraction).ln_1p()).exp_m1()
            };
            [
                (sliding + strain_velocity) as f32,
                (driving * (thickness - z)) as f32,
            ]
        })
        .collect();
    profile_result(
        host,
        "glen-shallow-ice-column",
        vec![height.len() as u32, 2],
        samples,
        json!({
            "model": "glen_shallow_ice_column", "axes": ["height_above_bed_m", "channel"],
            "channels": ["velocity_m_yr", "shear_stress_pa"], "heights_above_bed_m": height,
            "thickness_m": thickness, "surface_slope_tangent": slope, "density_kg_m3": density,
            "glen_exponent": exponent, "glen_rate_pa_to_minus_n_s_inv": flow, "basal_sliding_m_yr": sliding,
            "surface_velocity_m_yr": surface_velocity,
            "depth_integrated_flux_m2_yr": thickness * (sliding + surface_deformation * n_plus_one / (n_plus_one + 1.0)),
        }),
    )
}

fn peck(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(
        args,
        &[
            "mode",
            "offsets_m",
            "radius_m",
            "axis_depth_m",
            "volume_loss_fraction",
            "trough_width_coefficient",
        ],
    )?;
    let offsets = axis(args, "offsets_m", -1e4, 1e4)?;
    let radius = bounded(args, "radius_m", 0.1, 50.0)?;
    let depth = bounded(args, "axis_depth_m", radius + 0.01, 1000.0)?;
    let loss = bounded(args, "volume_loss_fraction", 0.0, 0.1)?;
    let width = bounded(args, "trough_width_coefficient", 0.05, 2.0)? * depth;
    let volume_loss_m2 = PI * radius * radius * loss;
    let maximum_settlement = volume_loss_m2 / (TAU.sqrt() * width);
    if maximum_settlement > 5.0 {
        return Err("Center settlement exceeds the supported 5 m".into());
    }
    host.charge(offsets.len() * 14, offsets.len() * 20)?;
    let samples = offsets
        .iter()
        .map(|x| (-maximum_settlement * (-0.5 * (x / width).powi(2)).exp()) as f32)
        .collect();
    profile_result(
        host,
        "peck-gaussian",
        vec![offsets.len() as u32],
        samples,
        json!({
            "model": "peck_gaussian", "axis": "cross_tunnel_offset_m", "offsets_m": offsets,
            "unit": "M", "upward_positive": true, "radius_m": radius, "axis_depth_m": depth,
            "volume_loss_fraction": loss, "integrated_trough_area_m2": volume_loss_m2,
            "inflection_distance_m": width, "center_settlement_m": -maximum_settlement,
        }),
    )
}

struct Slice {
    sine: f64,
    cosine: f64,
    friction: f64,
    resistance: f64,
    driving: f64,
}

fn bishop(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["mode", "circular_slip_slices"])?;
    let slices = list(required(args, "circular_slip_slices")?)?;
    if !(2..=2048).contains(&slices.len()) {
        return Err("Bishop mode requires 2..=2048 circular slip slices".into());
    }
    host.charge(slices.len() * 256, slices.len() * 144)?;
    let mut terms = Vec::with_capacity(slices.len());
    let mut input_rows = Vec::with_capacity(slices.len());
    for slice in slices {
        let row = record(slice)?;
        fields(
            row,
            &[
                "horizontal_width_m",
                "weight_n_m",
                "base_inclination_deg",
                "cohesion_pa",
                "friction_angle_deg",
                "pore_force_n_m",
            ],
        )?;
        let width = bounded(row, "horizontal_width_m", 1e-3, 1e3)?;
        let weight = bounded(row, "weight_n_m", 0.001, 1e12)?;
        let pore = bounded(row, "pore_force_n_m", 0.0, weight)?;
        let inclination = bounded(row, "base_inclination_deg", -80.0, 80.0)?;
        let (sine, cosine) = inclination.to_radians().sin_cos();
        let cohesion = bounded(row, "cohesion_pa", 0.0, 1e8)?;
        let friction_deg = bounded(row, "friction_angle_deg", 0.0, 60.0)?;
        let friction = friction_deg.to_radians().tan();
        // USACE EM 1110-2-1902, Appendix C-3. The pore force is the line integral
        // along the slice base, and external water moments are outside this mode.
        terms.push(Slice {
            sine,
            cosine,
            friction,
            resistance: cohesion * width + (weight - pore) * friction,
            driving: weight * sine,
        });
        input_rows.push([width, weight, inclination, cohesion, friction_deg, pore]);
    }
    let driving: f64 = terms.iter().map(|term| term.driving).sum();
    let scale: f64 = terms.iter().map(|term| term.driving.abs()).sum();
    if driving <= scale * 1e-10 {
        return Err("Circular slip surface has no resolved positive driving moment".into());
    }
    let evaluate = |factor: f64| {
        terms
            .iter()
            .map(|slice| {
                let m_alpha = slice.cosine + slice.sine * slice.friction / factor;
                (m_alpha > 1e-8).then_some(slice.resistance / m_alpha)
            })
            .try_fold(0.0, |sum, term| term.map(|v| sum + v))
    };
    let mut factor = 1.0;
    let mut residual = 0.0;
    let mut converged = false;
    for _ in 0..128 {
        let next = evaluate(factor)
            .ok_or("Invalid base reaction in simplified Bishop iteration")?
            / driving;
        residual = next - factor;
        if !next.is_finite() || next <= 0.0 {
            return Err("Bishop iteration yielded no positive factor".into());
        }
        if residual.abs() <= 1e-9 * factor.max(1.0) {
            converged = true;
            break;
        }
        factor = next;
    }
    if !converged || factor > 20.0 {
        return Err(
            "Bishop iteration failed to converge within the supported safety factor domain".into(),
        );
    }
    let samples = terms
        .iter()
        .flat_map(|slice| {
            let m_alpha = slice.cosine + slice.sine * slice.friction / factor;
            [
                (slice.resistance / m_alpha) as f32,
                slice.driving as f32,
                m_alpha as f32,
            ]
        })
        .collect();
    profile_result(
        host,
        "bishop-simplified",
        vec![slices.len() as u32, 3],
        samples,
        json!({
            "model": "bishop_simplified", "axes": ["slice", "channel"],
            "channels": ["resistance_n_m", "signed_driving_n_m", "m_alpha"], "safety_factor": factor, "factor_residual": residual,
            "slice_input_columns": ["horizontal_width_m", "weight_n_m", "base_inclination_deg", "cohesion_pa", "friction_angle_deg", "pore_force_n_m"],
            "slice_inputs": input_rows,
        }),
    )
}

//! Independent C02–C09 numerical checks over bounded supplied data.

use std::collections::BTreeMap;

use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    q42::{
        emf_operators::{self as emf, gpu_parity},
        science_validation as oracle,
    },
};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{fields, integer, list, num, record, required, text, vector, Args, AssetHost, Value};

const GRAPH: &str = "urn:qualia:science:validation:v1";
const MAX_SAMPLES: usize = 65_536;

impl AssetHost<'_> {
    pub(crate) fn validate_c02_passivity_and_oracles(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["task", "parameters"])?;
        let task = text(required(args, "task")?)?;
        let params = record(required(args, "parameters")?)?;
        let metrics = evaluate(self, task, params)?;
        if metrics.values().any(|value| !value.is_finite()) {
            return Err("Scientific oracle produced a non-finite result".into());
        }
        let inputs = crate::living_world_anatomy::species_spec::vibe_value_to_json(&Value::Record(
            params.clone(),
        ));
        let canonical =
            serde_json::to_vec(&(task, inputs)).map_err(|error| format!("{error:?}"))?;
        let subject = format!(
            "urn:qualia:science:measurement:{}",
            blake3::hash(&canonical).to_hex()
        );
        let mut graph = ReceiptGraph::new(GRAPH);
        graph.fact(
            &subject,
            "urn:qualia:science:oracle",
            &format!("urn:qualia:science:{task}"),
        )?;
        for (&name, &value) in &metrics {
            graph.fact(
                &subject,
                &format!("urn:qualia:science:{name}"),
                &format!("urn:qualia:science:double:{:016x}", value.to_bits()),
            )?;
        }
        let receipt = graph.seal(self.limits.resource_bytes.min(4 * 1024 * 1024))?;
        let metrics = metrics
            .into_iter()
            .map(|(name, value)| (name.into(), Value::F64(value)))
            .collect();
        Ok(Value::Record(BTreeMap::from([
            ("metrics".into(), Value::Record(metrics)),
            ("certificate".into(), self.encode("q42", receipt)?),
        ])))
    }
}

type Metrics = BTreeMap<&'static str, f64>;

fn evaluate(host: &mut AssetHost<'_>, task: &str, args: &Args) -> Result<Metrics, String> {
    let value = |key| required(args, key);
    let scalar = |key| num(value(key)?);
    Ok(match task {
        "poynting_balance" => {
            fields(
                args,
                &[
                    "div_flux_w_m3",
                    "energy_rate_w_m3",
                    "loss_w_m3",
                    "scale_w_m3",
                ],
            )?;
            let residual = oracle::verify_poynting_conservation_balance(
                scalar("div_flux_w_m3")?,
                scalar("energy_rate_w_m3")?,
                scalar("loss_w_m3")?,
                scalar("scale_w_m3")?,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("residual_w_m3", residual)])
        }
        "yee_cfl" => {
            fields(
                args,
                &[
                    "cell_size_m",
                    "step_s",
                    "epsilon_relative",
                    "mu_relative",
                    "wavevector_rad_m",
                ],
            )?;
            let (limit, omega) = oracle::verify_yee_cfl_and_dispersion(
                vector(value("cell_size_m")?)?,
                scalar("step_s")?,
                scalar("epsilon_relative")?,
                scalar("mu_relative")?,
                vector(value("wavevector_rad_m")?)?,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("cfl_max_s", limit), ("angular_frequency_rad_s", omega)])
        }
        "scattering_passivity" => {
            fields(args, &["matrix"])?;
            let rows = list(value("matrix")?)?;
            if !(1..=4).contains(&rows.len()) {
                return Err("Scattering matrix requires 1–4 ports".into());
            }
            let mut matrix = [[oracle::Complex64::ZERO; 4]; 4];
            for (out, input) in matrix.iter_mut().zip(rows) {
                let cols = list(input)?;
                if cols.len() != rows.len() {
                    return Err("Scattering matrix must be square".into());
                }
                for (out, input) in out.iter_mut().zip(cols) {
                    let [re, im] = vector::<2>(input)?;
                    if re.hypot(im) > 1.0 + 64.0 * f64::EPSILON {
                        return Err(
                            "A scattering coefficient already exceeds the passive bound".into()
                        );
                    }
                    *out = oracle::Complex64 { re, im };
                }
            }
            let norm = oracle::verify_scattering_matrix_passivity(&matrix, rows.len())
                .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("largest_singular_value", norm)])
        }
        "haptic_mechanics" => {
            fields(
                args,
                &[
                    "normal_force_n",
                    "contact_area_m2",
                    "spring_stiffness_n_m",
                    "displacement_m",
                    "force_local_n",
                    "parent_from_local",
                ],
            )?;
            let rotation = square_rows(value("parent_from_local")?)?;
            oracle::verify_stationary_proper_rotation_matrix(&rotation, false, false)
                .map_err(|error| format!("{error:?}"))?;
            let result = oracle::evaluate_haptic_mechanical_oracle(
                scalar("normal_force_n")?,
                scalar("contact_area_m2")?,
                scalar("spring_stiffness_n_m")?,
                scalar("displacement_m")?,
                vector(value("force_local_n")?)?,
                &rotation,
                oracle::HapticRole::ReceptorModel,
                false,
                false,
                0,
                0,
            )
            .map_err(|error| format!("{error:?}"))?;
            let [x, y, z] = result.parent_force_n;
            BTreeMap::from([
                ("contact_pressure_pa", result.contact_pressure_pa),
                ("restoring_force_n", result.restoring_force_n),
                ("force_parent_x_n", x),
                ("force_parent_y_n", y),
                ("force_parent_z_n", z),
            ])
        }
        "ultrasound_mode07" => {
            fields(
                args,
                &[
                    "sound_speed_m_s",
                    "range_m",
                    "sample_rate_hz",
                    "frequency_hz",
                    "derated_pressure_pa",
                    "radial_speed_m_s",
                    "reported_thermal_index",
                    "thermal_model_iri",
                ],
            )?;
            let model = args
                .get("thermal_model_iri")
                .map(canonical_iri)
                .transpose()?;
            let result = oracle::evaluate_ultrasound_mode07_oracle(
                scalar("sound_speed_m_s")?,
                scalar("range_m")?,
                scalar("sample_rate_hz")?,
                scalar("frequency_hz")?,
                scalar("derated_pressure_pa")?,
                scalar("radial_speed_m_s")?,
                args.get("reported_thermal_index").map(num).transpose()?,
                model,
                false,
            )
            .map_err(|error| format!("{error:?}"))?;
            let mut metrics = BTreeMap::from([
                ("round_trip_time_s", result.round_trip_tof_s),
                ("round_trip_samples", result.round_trip_samples),
                ("mechanical_index", result.mechanical_index),
                ("doppler_shift_hz", result.doppler_shift_hz),
            ]);
            if let Some(index) = result.reported_thermal_index {
                metrics.insert("reported_thermal_index", index);
            }
            metrics
        }
        "bioelectric_dipole" => {
            fields(
                args,
                &[
                    "dipole_a_m",
                    "source_m",
                    "observation_m",
                    "reference_electrode_iri",
                    "conductivity_s_m",
                ],
            )?;
            canonical_iri(value("reference_electrode_iri")?)?;
            let potential = oracle::evaluate_bioelectric_dipole_potential_v(
                vector(value("dipole_a_m")?)?,
                vector(value("source_m")?)?,
                Some(vector(value("observation_m")?)?),
                true,
                scalar("conductivity_s_m")?,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("potential_v", potential)])
        }
        "beer_lambert" => {
            fields(
                args,
                &[
                    "density_kg_m3",
                    "mass_attenuation_m2_kg",
                    "path_m",
                    "fluence_m2",
                ],
            )?;
            let result = oracle::evaluate_ionizing_beer_lambert_oracle(
                scalar("density_kg_m3")?,
                scalar("mass_attenuation_m2_kg")?,
                scalar("path_m")?,
                scalar("fluence_m2")?,
                false,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([
                ("linear_attenuation_m_inv", result.linear_attenuation_per_m),
                ("transmitted_fluence_m2", result.transmitted_fluence_per_m2),
            ])
        }
        "dose_from_deposition" => {
            fields(
                args,
                &[
                    "deposited_energy_j",
                    "absorber_mass_kg",
                    "weighting_model_iri",
                    "weighting_factor",
                ],
            )?;
            let absorbed = oracle::evaluate_absorbed_dose_gy(
                scalar("deposited_energy_j")?,
                scalar("absorber_mass_kg")?,
            )
            .map_err(|error| format!("{error:?}"))?;
            let mut metrics = BTreeMap::from([("absorbed_dose_gy", absorbed)]);
            match (
                args.get("weighting_factor"),
                args.get("weighting_model_iri"),
            ) {
                (Some(factor), Some(model)) => {
                    let equivalent = oracle::evaluate_equivalent_dose_sv(
                        absorbed,
                        num(factor)?,
                        Some(canonical_iri(model)?),
                    )
                    .map_err(|error| format!("{error:?}"))?;
                    metrics.insert("equivalent_dose_sv", equivalent);
                }
                (None, None) => {}
                _ => {
                    return Err(
                        "Equivalent dose requires both weighting factor and model IRI".into(),
                    )
                }
            }
            metrics
        }
        "ece_brier" => {
            fields(args, &["probabilities", "outcomes", "bins", "max_ece"])?;
            let probs = list(value("probabilities")?)?;
            let outcomes = list(value("outcomes")?)?;
            if probs.is_empty() || probs.len() > MAX_SAMPLES || probs.len() != outcomes.len() {
                return Err(
                    "Calibration requires 1–65536 aligned probability/outcome samples".into(),
                );
            }
            host.charge(probs.len() * 8, probs.len() * 16)?;
            let max = scalar("max_ece")?;
            if !(0.0..=1.0).contains(&max) {
                return Err("Maximum ECE must lie in [0, 1]".into());
            }
            let probs = probs.iter().map(num).collect::<Result<Vec<_>, _>>()?;
            let outcomes = outcomes
                .iter()
                .map(|item| match item {
                    Value::Bool(value) => Ok(*value),
                    _ => Err("Calibration outcomes must be boolean"),
                })
                .collect::<Result<Vec<_>, _>>()?;
            let result = oracle::evaluate_ece_and_brier(
                &probs,
                &outcomes,
                integer(value("bins")?, "bins")?,
                max,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([
                (
                    "expected_calibration_error",
                    result.expected_calibration_error,
                ),
                (
                    "maximum_calibration_error",
                    result.maximum_calibration_error,
                ),
                ("brier_score", result.brier_score),
            ])
        }
        "mahalanobis_nees_3d" => {
            fields(args, &["observation", "mean", "covariance"])?;
            let score = oracle::mahalanobis_nees_3d(
                vector(value("observation")?)?,
                vector(value("mean")?)?,
                &square_rows(value("covariance")?)?,
            )
            .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([("squared_mahalanobis", score)])
        }
        "yee_f32_reference_parity" => {
            fields(args, &["cells_x", "steps"])?;
            let cells = integer(value("cells_x")?, "cells_x")?;
            let steps = integer(value("steps")?, "steps")?;
            if !(4..=4_096).contains(&cells) || !(1..=4_096).contains(&steps) {
                return Err("Yee reference parity requires 4–4096 cells and 1–4096 steps".into());
            }
            host.charge(cells * steps * 64, cells * 1024)?;
            let report = gpu_parity::verify_cpu_f64_vs_gpu_f32_yee_parity(cells, steps)
                .map_err(|error| format!("{error:?}"))?;
            BTreeMap::from([
                ("electric_max_abs_diff", report.max_e_abs_diff),
                ("magnetic_max_abs_diff", report.max_h_abs_diff),
                ("electric_relative_l2", report.e_rel_l2_diff),
                ("magnetic_relative_l2", report.h_rel_l2_diff),
                ("electric_budget", report.e_budget),
                ("magnetic_budget", report.h_budget),
            ])
        }
        "yee_spatial_invariants" => yee_spatial_invariants(host, args)?,
        _ => return Err(format!("Unknown scientific oracle {task}")),
    })
}

fn yee_spatial_invariants(host: &mut AssetHost<'_>, args: &Args) -> Result<Metrics, String> {
    fields(
        args,
        &[
            "dimensions",
            "cell_size_m",
            "electric",
            "magnetic",
            "cell",
            "divergence_tolerance",
            "circulation_tolerance",
        ],
    )?;
    let [nx, ny, nz] = index3(required(args, "dimensions")?, "dimensions")?;
    let [dx, dy, dz] = vector(required(args, "cell_size_m")?)?;
    let grid = emf::Grid3dSpec {
        nx,
        ny,
        nz,
        dx,
        dy,
        dz,
    };
    let n = grid.validate().map_err(|error| format!("{error:?}"))?;
    if n > MAX_SAMPLES || [nx, ny, nz].into_iter().any(|size| size < 4) {
        return Err(
            "Periodic Yee validation needs at least four cells per axis and at most 65536 samples"
                .into(),
        );
    }
    let [ix, iy, iz] = index3(required(args, "cell")?, "cell")?;
    if ix >= nx || iy >= ny || iz >= nz {
        return Err("Stokes cell is outside the grid".into());
    }
    let div_tolerance = num(required(args, "divergence_tolerance")?)?;
    let circulation_tolerance = num(required(args, "circulation_tolerance")?)?;
    if div_tolerance < 0.0 || circulation_tolerance < 0.0 {
        return Err("Yee tolerances must be nonnegative".into());
    }
    host.charge(n * 128, n * 96)?;
    let [ex, ey, ez] = components(required(args, "electric")?, n)?;
    let [hx, hy, hz] = components(required(args, "magnetic")?, n)?;
    let [mut cx, mut cy, mut cz, mut div] = std::array::from_fn(|_| vec![0.0; n]);
    let bc = emf::BoundaryCondition::Periodic;
    let (e_div, h_div) = emf::verify_div_curl_zero_2nd(
        &grid, bc, &ex, &ey, &ez, &hx, &hy, &hz, &mut cx, &mut cy, &mut cz, &mut div,
    )
    .map_err(|error| format!("{error:?}"))?;
    emf::yee_curl_e_2nd(&grid, bc, &ex, &ey, &ez, &mut cx, &mut cy, &mut cz)
        .map_err(|error| format!("{error:?}"))?;
    let stokes =
        emf::verify_stokes_circulation_cell(&grid, bc, &ex, &ey, &ez, &cx, &cy, &cz, ix, iy, iz)
            .map_err(|error| format!("{error:?}"))?;
    ensure_finite(&[&cx, &cy, &cz, &div, &stokes, &[e_div, h_div]])?;
    let max_stokes = stokes.into_iter().map(f64::abs).fold(0.0, f64::max);
    if e_div.abs().max(h_div.abs()) > div_tolerance || max_stokes > circulation_tolerance {
        return Err("Yee vector identities exceed the supplied numerical budgets".into());
    }
    emf::yee_curl_e_4th(&grid, bc, &ex, &ey, &ez, &mut cx, &mut cy, &mut cz)
        .map_err(|error| format!("{error:?}"))?;
    ensure_finite(&[&cx, &cy, &cz])?;
    let curl_e = cx
        .iter()
        .chain(&cy)
        .chain(&cz)
        .map(|v| v.abs())
        .fold(0.0, f64::max);
    emf::yee_curl_h_4th(&grid, bc, &hx, &hy, &hz, &mut cx, &mut cy, &mut cz)
        .map_err(|error| format!("{error:?}"))?;
    ensure_finite(&[&cx, &cy, &cz])?;
    let curl_h = cx
        .iter()
        .chain(&cy)
        .chain(&cz)
        .map(|v| v.abs())
        .fold(0.0, f64::max);
    Ok(BTreeMap::from([
        ("electric_div_curl_max", e_div),
        ("magnetic_div_curl_max", h_div),
        ("stokes_residual_max", max_stokes),
        ("electric_fang_curl_max", curl_e),
        ("magnetic_fang_curl_max", curl_h),
    ]))
}

fn ensure_finite(arrays: &[&[f64]]) -> Result<(), String> {
    if arrays
        .iter()
        .any(|array| array.iter().any(|v| !v.is_finite()))
    {
        Err("Yee operator produced non-finite samples".into())
    } else {
        Ok(())
    }
}

fn index3(value: &Value, field: &str) -> Result<[usize; 3], String> {
    let [x, y, z] = list(value)? else {
        return Err(format!("{field} requires three indices"));
    };
    Ok([integer(x, field)?, integer(y, field)?, integer(z, field)?])
}

fn components(value: &Value, cells: usize) -> Result<[Vec<f64>; 3], String> {
    let [x, y, z] = list(value)? else {
        return Err("Field requires three component arrays".into());
    };
    let read = |value| {
        let samples = list(value)?;
        if samples.len() != cells {
            return Err("Field component shape differs from the Yee grid".into());
        }
        samples.iter().map(num).collect::<Result<Vec<_>, String>>()
    };
    Ok([read(x)?, read(y)?, read(z)?])
}

fn canonical_iri(value: &Value) -> Result<&str, String> {
    let iri = text(value)?;
    if iri.len() > 4096 {
        return Err("Oracle IRI exceeds 4096 bytes".into());
    }
    resource_identity_meta(iri).map_err(|error| format!("{error:?}"))?;
    Ok(iri)
}

fn square_rows(value: &Value) -> Result<[[f64; 3]; 3], String> {
    let [x, y, z] = list(value)? else {
        return Err("Expected a 3×3 matrix".into());
    };
    Ok([vector(x)?, vector(y)?, vector(z)?])
}

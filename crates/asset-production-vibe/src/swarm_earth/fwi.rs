//! One-dimensional longitudinal elastic inversion with a discrete time adjoint.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Value, fields, integer, list, num, record, required, text};

use super::{bounded, lattice, profile_result};

struct Experiment {
    cells: usize,
    spacing_m: f64,
    dt_s: f64,
    density: Vec<f64>,
    face_velocity_coeff: Vec<f64>,
    source: usize,
    source_samples: Vec<f64>,
    receivers: Vec<usize>,
    observation: Vec<f64>,
    inverse_squared_scale: f64,
}

struct Shot {
    loss: f64,
    receivers_pa: Vec<f64>,
    pressure_increment: Vec<f64>,
}

impl AssetHost<'_> {
    pub(crate) fn solve_elastic_fwi_adjoint_tomography(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "cell_size_m",
                "dt_s",
                "initial_p_wave_speed_m_s",
                "density_kg_m3",
                "source_cell",
                "source_pressure_increment_pa",
                "receiver_cells",
                "observed_receiver_pressure_pa",
                "iterations",
                "max_log_speed_step",
            ],
        )?;
        if text(required(args, "mode")?)? != "longitudinal_1d" {
            return Err("Expected the longitudinal_1d elastic FWI mode".into());
        }
        let spacing = bounded(args, "cell_size_m", 0.1, 20_000.0)?;
        let dt = bounded(args, "dt_s", 1e-7, 20.0)?;
        // Interface density is the arithmetic mean. The global spectral bound
        // with heterogeneous density is sqrt(2) times the homogeneous CFL.
        let speed_bound = (0.65 * spacing / dt).min(15_000.0);
        if speed_bound < 300.0 {
            return Err("Elastic inversion time step exceeds the stable Courant range".into());
        }
        let speed = numbers(
            args,
            "initial_p_wave_speed_m_s",
            8..=256,
            300.0,
            speed_bound,
        )?;
        let cells = speed.len();
        let density = numbers(args, "density_kg_m3", cells..=cells, 500.0, 12_000.0)?;
        let mut face_velocity_coeff = vec![0.0; cells + 1];
        for (face, adjacent) in density.windows(2).enumerate() {
            face_velocity_coeff[face + 1] = 2.0 * dt / spacing / (adjacent[0] + adjacent[1]);
        }
        let source = integer(required(args, "source_cell")?, "source_cell")?;
        if source >= cells {
            return Err("Source cell lies outside the inversion grid".into());
        }
        let source_samples = numbers(args, "source_pressure_increment_pa", 8..=4096, -1e9, 1e9)?;
        if source_samples.iter().all(|&x| x == 0.0) {
            return Err("The elastic inversion source needs a nonzero signal".into());
        }
        let steps = source_samples.len();
        let authored_receivers = list(required(args, "receiver_cells")?)?;
        if !(1..=16).contains(&authored_receivers.len()) {
            return Err("Elastic inversion requires 1..=16 receivers".into());
        }
        let receivers = authored_receivers
            .iter()
            .map(|v| integer(v, "receiver cell"))
            .collect::<Result<Vec<_>, _>>()?;
        if receivers.iter().any(|&r| r >= cells) || receivers.windows(2).any(|r| r[0] >= r[1]) {
            return Err(
                "Receiver cells must be sorted, unique and inside the inversion grid".into(),
            );
        }
        let observations = steps * receivers.len();
        let observation = numbers(
            args,
            "observed_receiver_pressure_pa",
            observations..=observations,
            -1e10,
            1e10,
        )?;
        let pressure_scale = observation
            .iter()
            .chain(&source_samples)
            .fold(1.0_f64, |m, v| m.max(v.abs()));
        let experiment = Experiment {
            cells,
            spacing_m: spacing,
            dt_s: dt,
            density,
            face_velocity_coeff,
            source,
            source_samples,
            receivers,
            observation,
            inverse_squared_scale: 1.0 / (pressure_scale * pressure_scale * observations as f64),
        };
        let iterations = integer(required(args, "iterations")?, "iterations")?;
        if !(1..=16).contains(&iterations) {
            return Err("Elastic inversion iterations must be within 1..=16".into());
        }
        let step_limit = bounded(args, "max_log_speed_step", 1e-4, 0.25)?;
        let log_bounds = [300.0_f64.ln(), speed_bound.ln()];
        let mut model: Vec<f64> = speed.iter().map(|v| v.ln()).collect();
        let mut shot = forward(self, &experiment, &model, true)?;
        let mut objective = vec![shot.loss];
        let mut outcome = "iteration_limit";
        for _ in 0..iterations {
            let gradient = adjoint(self, &experiment, &model, &shot)?;
            let max_gradient = gradient
                .iter()
                .fold(0.0_f64, |largest, g| largest.max(g.abs()));
            if max_gradient <= 1e-14 || shot.loss == 0.0 {
                outcome = "stationary";
                break;
            }
            let mut alpha = step_limit / max_gradient;
            let mut success = false;
            for _ in 0..8 {
                let candidate: Vec<f64> = model
                    .iter()
                    .zip(&gradient)
                    .map(|(&old, &g)| (old - alpha * g).clamp(log_bounds[0], log_bounds[1]))
                    .collect();
                let directional: f64 = candidate
                    .iter()
                    .zip(&model)
                    .zip(&gradient)
                    .map(|((&a, &b), &g)| (a - b) * g)
                    .sum();
                if directional >= 0.0 {
                    break;
                }
                let trial = forward(self, &experiment, &candidate, false)?;
                if trial.loss < shot.loss && trial.loss <= shot.loss + 1e-4 * directional {
                    model = candidate;
                    shot = forward(self, &experiment, &model, true)?;
                    objective.push(shot.loss);
                    success = true;
                    break;
                }
                alpha *= 0.5;
            }
            if !success {
                outcome = "no_descent_within_line_search";
                break;
            }
        }
        let gradient = adjoint(self, &experiment, &model, &shot)?;
        let samples = speed
            .iter()
            .zip(&model)
            .zip(gradient)
            .flat_map(|((&initial, &update), gradient)| {
                [initial as f32, update.exp() as f32, gradient as f32]
            })
            .collect();
        let mut acquisition = Sha256::new();
        acquisition.update((cells as u32).to_le_bytes());
        acquisition.update((steps as u32).to_le_bytes());
        acquisition.update((source as u32).to_le_bytes());
        acquisition.update(spacing.to_le_bytes());
        acquisition.update(dt.to_le_bytes());
        for &r in &experiment.receivers {
            acquisition.update((r as u32).to_le_bytes());
        }
        for &value in experiment
            .density
            .iter()
            .chain(&speed)
            .chain(&experiment.source_samples)
            .chain(&experiment.observation)
        {
            acquisition.update(value.to_le_bytes());
        }
        let traces: Vec<f32> = shot.receivers_pa.iter().map(|&v| v as f32).collect();
        if traces.iter().any(|v| !v.is_finite()) {
            return Err("Elastic inversion generated nonfinite receiver traces".into());
        }
        let mut trace_hasher = Sha256::new();
        for value in &traces {
            trace_hasher.update(value.to_le_bytes());
        }
        let result = profile_result(
            self,
            "longitudinal-fwi-1d",
            vec![cells as u32, 3],
            samples,
            json!({
                "model": "longitudinal_1d", "axes": ["cell", "channel"],
                "channels": ["initial_p_wave_speed_m_s", "updated_p_wave_speed_m_s", "d_normalized_objective_d_log_p_wave_speed"],
                "objective": "half_mean_squared_pressure_residual_normalized_by_scale_squared", "pressure_scale_pa": pressure_scale,
                "objective_history": objective, "outcome": outcome, "cell_size_m": spacing, "dt_s": dt,
                "receiver_cells": &experiment.receivers, "receiver_trace_axes": ["time_step", "receiver"],
                "first_receiver_sample_time_s": dt, "source_cell": source, "boundary": "impermeable_reflective",
                "inputs_sha256": format!("{:x}", acquisition.finalize()), "receivers_f32_sha256": format!("{:x}", trace_hasher.finalize()),
            }),
        )?;
        let mut output = record(&result)?.clone();
        output.insert(
            "receivers".into(),
            lattice(
                self,
                vec![steps as u32, experiment.receivers.len() as u32],
                traces,
            )?,
        );
        Ok(Value::Record(output))
    }
}

fn numbers(
    args: &Args,
    key: &str,
    count: std::ops::RangeInclusive<usize>,
    lo: f64,
    hi: f64,
) -> Result<Vec<f64>, String> {
    let values = list(required(args, key)?)?;
    if !count.contains(&values.len()) {
        return Err(format!(
            "{key} must contain {}..={} entries",
            count.start(),
            count.end()
        ));
    }
    let samples: Vec<f64> = values.iter().map(num).collect::<Result<_, _>>()?;
    if samples.iter().any(|x| !(lo..=hi).contains(x)) {
        return Err(format!("{key} must be within [{lo}, {hi}]"));
    }
    Ok(samples)
}

fn pressure_coeff(experiment: &Experiment, log_speed: &[f64]) -> Vec<f64> {
    experiment
        .density
        .iter()
        .zip(log_speed)
        .map(|(&rho, &log_c)| rho * (2.0 * log_c).exp() * experiment.dt_s / experiment.spacing_m)
        .collect()
}

fn shot_scratch(e: &Experiment) -> usize {
    e.cells * e.source_samples.len() * 16 + e.observation.len() * 24 + e.cells * 72
}

fn forward(
    host: &mut AssetHost<'_>,
    e: &Experiment,
    log_speed: &[f64],
    history: bool,
) -> Result<Shot, String> {
    host.charge(e.cells * e.source_samples.len() * 19, shot_scratch(e))?;
    let coeff = pressure_coeff(e, log_speed);
    let mut pressure = vec![0.0; e.cells];
    let mut velocity = vec![0.0; e.cells + 1];
    let mut predicted = Vec::with_capacity(e.observation.len());
    let mut pressure_increment = Vec::with_capacity(if history {
        e.cells * e.source_samples.len()
    } else {
        0
    });
    for &sample in &e.source_samples {
        for face in 1..e.cells {
            velocity[face] -= e.face_velocity_coeff[face] * (pressure[face] - pressure[face - 1]);
        }
        for cell in 0..e.cells {
            let increment = -coeff[cell] * (velocity[cell + 1] - velocity[cell]);
            pressure[cell] += increment;
            if history {
                pressure_increment.push(increment);
            }
        }
        pressure[e.source] += sample;
        predicted.extend(e.receivers.iter().map(|&receiver| pressure[receiver]));
    }
    let loss = 0.5
        * e.inverse_squared_scale
        * predicted
            .iter()
            .zip(&e.observation)
            .map(|(&p, &o)| (p - o).powi(2))
            .sum::<f64>();
    if !loss.is_finite() {
        return Err("Elastic waveform objective became nonfinite".into());
    }
    Ok(Shot {
        loss,
        receivers_pa: predicted,
        pressure_increment,
    })
}

fn adjoint(
    host: &mut AssetHost<'_>,
    e: &Experiment,
    log_speed: &[f64],
    shot: &Shot,
) -> Result<Vec<f64>, String> {
    host.charge(e.cells * e.source_samples.len() * 23, shot_scratch(e))?;
    let coeff = pressure_coeff(e, log_speed);
    let mut pressure = vec![0.0; e.cells];
    let mut velocity = vec![0.0; e.cells + 1];
    let mut gradient = vec![0.0; e.cells];
    for n in (0..e.source_samples.len()).rev() {
        for (i, &receiver) in e.receivers.iter().enumerate() {
            let trace = n * e.receivers.len() + i;
            pressure[receiver] +=
                e.inverse_squared_scale * (shot.receivers_pa[trace] - e.observation[trace]);
        }
        // Differentiate c² with respect to log(c). The array holds the exact
        // pressure increment before the source contribution in the forward pass.
        for cell in 0..e.cells {
            gradient[cell] += 2.0 * pressure[cell] * shot.pressure_increment[n * e.cells + cell];
        }
        for face in 1..e.cells {
            velocity[face] += coeff[face] * pressure[face] - coeff[face - 1] * pressure[face - 1];
        }
        for face in 1..e.cells {
            let product = e.face_velocity_coeff[face] * velocity[face];
            pressure[face - 1] += product;
            pressure[face] -= product;
        }
    }
    if gradient.iter().any(|v| !v.is_finite()) {
        return Err("Elastic adjoint gradient became nonfinite".into());
    }
    Ok(gradient)
}

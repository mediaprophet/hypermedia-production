//! Periodic linear shallow-water 4D-Var with the exact discrete wave adjoint.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{input_plane, int_arg, number_arg, output_plane, seal_diagnostics};
use crate::{Args, AssetHost, Value, fields, list, record, required, text};

type State = [f64; 2];

struct Observation {
    cell: usize,
    height: f64,
    weight: f64,
}

struct Wave {
    steps: usize,
    cfl: f64,
    background_weight: f64,
    observations: Vec<Vec<Observation>>,
}

impl AssetHost<'_> {
    pub(crate) fn solve_4dvar_fluid_and_contact_adjoint(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "cell_count",
                "cell_size_m",
                "mean_depth_m",
                "assimilation_window_s",
                "steps",
                "iterations",
                "background_weight",
                "initial_height_m",
                "initial_velocity_m_s",
                "observations",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "linear_swe_periodic_4dvar" {
                return Err("Supported solver is linear_swe_periodic_4dvar".into());
            }
        }
        let cells = int_arg(args, "cell_count", None, 3..=512)?;
        let steps = int_arg(args, "steps", None, 1..=1024)?;
        let iterations = int_arg(args, "iterations", Some(16), 1..=64)?;
        let spacing = number_arg(args, "cell_size_m", None, 0.01..=1e6)?;
        let depth = number_arg(args, "mean_depth_m", None, 0.001..=1e4)?;
        let duration = number_arg(args, "assimilation_window_s", None, 1e-4..=1e6)?;
        let background_weight = number_arg(args, "background_weight", Some(0.05), 1e-6..=1e5)?;
        let speed = (9.80665 * depth).sqrt();
        let cfl = speed * (duration / steps as f64) / spacing;
        if cfl > 1.0 {
            return Err(format!("Linear shallow-water CFL {cfl:.9e} exceeds 1"));
        }
        let limit = depth * 0.15;
        let velocity_scale = depth / speed;
        let shape = [cells as u32];
        let height = optional_plane(self, args, "initial_height_m", &shape, limit)?;
        let flow = optional_plane(
            self,
            args,
            "initial_velocity_m_s",
            &shape,
            limit / velocity_scale,
        )?;
        let prior: Vec<State> = height
            .iter()
            .zip(flow)
            .map(|(&eta, velocity)| [eta, velocity * velocity_scale])
            .collect();
        let values = list(required(args, "observations")?)?;
        if values.is_empty() || values.len() > 8192 {
            return Err("Use 1..=8192 observations".into());
        }
        let mut observations: Vec<_> = (0..=steps).map(|_| Vec::new()).collect();
        for value in values {
            let row = record(value)?;
            fields(row, &["step", "cell", "height_m", "weight"])?;
            let step = int_arg(row, "step", None, 0..=steps)?;
            let cell = int_arg(row, "cell", None, 0..=cells - 1)?;
            let height = number_arg(row, "height_m", None, -limit..=limit)?;
            let weight = number_arg(row, "weight", Some(1.0), 1e-6..=1e6)?;
            observations[step].push(Observation {
                cell,
                height,
                weight,
            });
        }
        let wave = Wave {
            steps,
            cfl,
            background_weight,
            observations,
        };
        let scratch = cells * 16 * 6 + values.len() * 32 + (steps + 1) * 40;
        let update_work = steps * cells * 68 + values.len() * 32;
        let mut state = prior.clone();
        let mut history = Vec::with_capacity(iterations + 1);
        let mut initial_cost = 0.0;
        let mut gradient_norm = 0.0;
        let mut final_state = Vec::new();
        let mut peak_amplitude = 0.0;
        for iteration in 0..=iterations {
            self.charge(update_work, scratch)?;
            let (cost, gradient, endpoint, peak) = wave.evaluate(&state, &prior)?;
            if iteration == 0 {
                initial_cost = cost;
            }
            history.push(Value::F64(cost));
            gradient_norm = norm_square(&gradient);
            final_state = endpoint;
            peak_amplitude = peak;
            if iteration == iterations || gradient_norm <= 1e-24 {
                break;
            }
            self.charge(update_work / 2, scratch)?;
            let curvature = wave.curvature(&gradient);
            if !curvature.is_finite() || curvature <= 0.0 {
                return Err("Linear wave observation Hessian lost positive curvature".into());
            }
            let step = gradient_norm / curvature;
            for (point, direction) in state.iter_mut().zip(gradient) {
                for k in 0..2 {
                    point[k] -= step * direction[k];
                }
                if point.iter().any(|x| !x.is_finite()) {
                    return Err("Nonfinite 4D-Var control update".into());
                }
            }
        }
        if peak_amplitude > limit {
            return Err(format!(
                "Linear shallow-water amplitude {peak_amplitude:.12e} m exceeds {limit:.12e} m"
            ));
        }
        let [eta0, u0] = planes(&state, velocity_scale);
        let [etaf, uf] = planes(&final_state, velocity_scale);
        let initial = output_plane(self, shape.to_vec(), &[&eta0, &u0])?;
        let final_field = output_plane(self, shape.to_vec(), &[&etaf, &uf])?;
        let final_cost = match history.last() {
            Some(Value::F64(value)) => *value,
            _ => return Err("Missing 4D-Var objective".into()),
        };
        let hash = wave.run_hash(&prior, [spacing, depth, duration], iterations);
        let metrics = [
            ("initial_objective", initial_cost),
            ("final_objective", final_cost),
            ("final_gradient_norm", gradient_norm.sqrt()),
            ("cfl", cfl),
            ("assimilation_window_s", duration),
        ];
        let receipt = seal_diagnostics(self, "linear-swe-4dvar", &hash, &metrics)?;
        Ok(Value::Record(Args::from([
            ("initial_conditions".into(), initial),
            ("final_conditions".into(), final_field),
            ("objective_history".into(), Value::List(history.into())),
            ("gradient_norm".into(), Value::F64(gradient_norm.sqrt())),
            ("receipt".into(), receipt),
        ])))
    }
}

fn optional_plane(
    host: &AssetHost<'_>,
    args: &Args,
    key: &str,
    shape: &[u32; 1],
    limit: f64,
) -> Result<Vec<f64>, String> {
    args.get(key)
        .map(|value| input_plane(host, value, shape, -limit..=limit, key))
        .transpose()
        .map(|value| value.unwrap_or_else(|| vec![0.0; shape[0] as usize]))
}

impl Wave {
    fn evaluate(
        &self,
        initial: &[State],
        prior: &[State],
    ) -> Result<(f64, Vec<State>, Vec<State>, f64), String> {
        let mut prediction = initial.to_vec();
        let mut next = vec![[0.0; 2]; initial.len()];
        let mut residuals = Vec::with_capacity(self.steps + 1);
        let mut cost = 0.0;
        let mut peak: f64 = 0.0;
        for (step, observations) in self.observations.iter().enumerate() {
            if step > 0 {
                wave_step(&prediction, &mut next, self.cfl, false);
                std::mem::swap(&mut prediction, &mut next);
            }
            for value in prediction.iter().flatten() {
                peak = peak.max(value.abs());
            }
            let mut residual = Vec::with_capacity(observations.len());
            for observation in observations {
                let delta = prediction[observation.cell][0] - observation.height;
                cost += 0.5 * observation.weight * delta * delta;
                residual.push(observation.weight * delta);
            }
            residuals.push(residual);
        }
        let mut gradient = vec![[0.0; 2]; initial.len()];
        for step in (0..=self.steps).rev() {
            if step < self.steps {
                wave_step(&gradient, &mut next, self.cfl, true);
                std::mem::swap(&mut gradient, &mut next);
            }
            for (observation, &residual) in self.observations[step].iter().zip(&residuals[step]) {
                gradient[observation.cell][0] += residual;
            }
        }
        for ((gradient, initial), prior) in gradient.iter_mut().zip(initial).zip(prior) {
            for k in 0..2 {
                let delta = initial[k] - prior[k];
                gradient[k] += self.background_weight * delta;
                cost += 0.5 * self.background_weight * delta * delta;
            }
        }
        if !cost.is_finite() || gradient.iter().flatten().any(|x| !x.is_finite()) {
            return Err("Nonfinite linear shallow-water assimilation objective or gradient".into());
        }
        Ok((cost, gradient, prediction, peak))
    }

    fn curvature(&self, direction: &[State]) -> f64 {
        // H is the sum of A_t^T H_t^T R_t H_t A_t and the background weight.
        // Its directional form can be evaluated using one tangent propagation.
        let mut curvature = self.background_weight * norm_square(direction);
        let mut prediction = direction.to_vec();
        let mut next = vec![[0.0; 2]; direction.len()];
        for (step, observations) in self.observations.iter().enumerate() {
            if step > 0 {
                wave_step(&prediction, &mut next, self.cfl, false);
                std::mem::swap(&mut prediction, &mut next);
            }
            for observation in observations {
                curvature += observation.weight * prediction[observation.cell][0].powi(2);
            }
        }
        curvature
    }

    fn run_hash(&self, prior: &[State], physical: [f64; 3], iterations: usize) -> String {
        let mut hasher = blake3::Hasher::new_derive_key("qualia continuum linear SWE 4DVar v1");
        for value in [self.steps, prior.len(), iterations] {
            hasher.update(&(value as u64).to_le_bytes());
        }
        for value in physical
            .into_iter()
            .chain([self.cfl, self.background_weight])
        {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        for &value in prior.iter().flatten() {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        for entries in &self.observations {
            hasher.update(&(entries.len() as u64).to_le_bytes());
            for observation in entries {
                hasher.update(&(observation.cell as u64).to_le_bytes());
                hasher.update(&observation.height.to_bits().to_le_bytes());
                hasher.update(&observation.weight.to_bits().to_le_bytes());
            }
        }
        hasher.finalize().to_hex().to_string()
    }
}

fn wave_step(state: &[State], next: &mut [State], cfl: f64, transpose: bool) {
    let n = state.len();
    let cross = if transpose { -0.5 * cfl } else { 0.5 * cfl };
    for i in 0..n {
        let left = state[(i + n - 1) % n];
        let right = state[(i + 1) % n];
        for k in 0..2 {
            next[i][k] = (1.0 - cfl) * state[i][k]
                + 0.5 * cfl * (left[k] + right[k])
                + cross * (left[1 - k] - right[1 - k]);
        }
    }
}

fn norm_square(points: &[State]) -> f64 {
    points.iter().flatten().map(|x| x * x).sum()
}

fn planes(points: &[State], velocity_scale: f64) -> [Vec<f64>; 2] {
    [
        points.iter().map(|point| point[0]).collect(),
        points
            .iter()
            .map(|point| point[1] / velocity_scale)
            .collect(),
    ]
}

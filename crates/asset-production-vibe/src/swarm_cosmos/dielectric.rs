//! Discrete adjoint of a fixed, passive one-dimensional dielectric slab example.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{names, scalar};
use crate::{
    artifact::FieldLattice, fields, required, text, Args, AssetHost, TypedArtifact, Value,
};
use serde_json::{Map as JsonMap, Value as Json};
use std::{collections::BTreeMap, sync::Arc};

const TRIAL_SCHEMA: &str = "qualia.passive_dielectric_slab_trial.v1";
const MAX_TRIAL_BYTES: usize = 128 * 1024;
const COURANT: f64 = 0.5;
const MIN_PERMITTIVITY: f64 = 1.0;
const MAX_PERMITTIVITY: f64 = 12.0;

enum Reference {
    Authored(Vec<f64>),
    Dielectric(f64),
}

struct Trial {
    points: usize,
    samples: usize,
    reference: Reference,
}

struct Slab {
    courant_squared_over_epsilon: Vec<f64>,
    derivative_wrt_epsilon: Vec<f64>,
    probe: usize,
}

impl AssetHost<'_> {
    pub(crate) fn solve_adjoint_fdtd_inverse_design(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["trial", "relative_permittivity"])?;
        let key = text(required(args, "trial")?)?;
        let epsilon = scalar(
            args,
            "relative_permittivity",
            None,
            MIN_PERMITTIVITY,
            MAX_PERMITTIVITY,
        )?;
        let input = self
            .inputs
            .iter()
            .find(|input| input.key == key)
            .ok_or_else(|| format!("Missing retained dielectric wave trial {key}"))?;
        if input.kind != "json" || !(1..=MAX_TRIAL_BYTES).contains(&input.bytes.len()) {
            return Err(
                "Dielectric wave trial must be a retained JSON resource of at most 128 KiB".into(),
            );
        }
        let trial = Trial::decode(&input.bytes)?;
        let n = trial.points;
        let history_len = n * (trial.samples + 2);
        self.charge(
            history_len * 80,
            history_len * 8 + n * 128 + trial.samples * 64,
        )?;
        let slab = Slab::new(n, epsilon);
        let references = match trial.reference {
            Reference::Authored(reference) => reference,
            Reference::Dielectric(reference) => {
                let reference_slab = Slab::new(n, reference);
                let (mut previous, mut current) = initial_fields(n);
                let mut next = vec![0.0; n];
                let mut trace = Vec::with_capacity(trial.samples);
                for sample in 0..trial.samples {
                    if sample % 64 == 0 {
                        self.charge(0, 0)?;
                    }
                    reference_slab.advance(&previous, &current, &mut next);
                    trace.push(next[slab.probe]);
                    std::mem::swap(&mut previous, &mut current);
                    std::mem::swap(&mut current, &mut next);
                }
                trace
            }
        };
        let mut history = vec![0.0; history_len];
        let (previous, current) = initial_fields(n);
        history[..n].copy_from_slice(&previous);
        history[n..2 * n].copy_from_slice(&current);

        let mut predictions = Vec::with_capacity(trial.samples);
        let mut residuals = Vec::with_capacity(trial.samples);
        let mut tangent_previous = vec![0.0; n];
        let mut tangent_current = vec![0.0; n];
        let mut tangent_next = vec![0.0; n];
        let mut loss = 0.0;
        let mut tangent_gradient = 0.0;
        let norm = 1.0 / trial.samples as f64;
        for (sample, reference) in references.iter().enumerate() {
            if sample % 64 == 0 {
                self.charge(0, 0)?;
            }
            let (past, future) = history.split_at_mut((sample + 2) * n);
            let previous = &past[sample * n..(sample + 1) * n];
            let current = &past[(sample + 1) * n..];
            let next = &mut future[..n];
            slab.advance(previous, current, next);

            // Forward-mode derivative of the same discrete step. Comparing it
            // with the reverse sweep checks the adjoint on the admitted trial.
            slab.advance(&tangent_previous, &tangent_current, &mut tangent_next);
            for i in 1..n - 1 {
                tangent_next[i] += slab.derivative_wrt_epsilon[i] * laplacian(current, i);
            }
            let prediction = next[slab.probe];
            let residual = prediction - reference;
            predictions.push(prediction);
            residuals.push(residual);
            loss += 0.5 * norm * residual * residual;
            tangent_gradient += norm * residual * tangent_next[slab.probe];
            std::mem::swap(&mut tangent_previous, &mut tangent_current);
            std::mem::swap(&mut tangent_current, &mut tangent_next);
        }

        let mut next_adjoint = vec![0.0; n];
        let mut current_adjoint = vec![0.0; n];
        let mut previous_adjoint = vec![0.0; n];
        let mut gradient = 0.0;
        for (sample, &residual) in residuals.iter().enumerate().rev() {
            if sample % 64 == 0 {
                self.charge(0, 0)?;
            }
            let state = &history[(sample + 1) * n..(sample + 2) * n];
            next_adjoint[slab.probe] += norm * residual;
            for i in 1..n - 1 {
                gradient += next_adjoint[i] * slab.derivative_wrt_epsilon[i] * laplacian(state, i);
                // The transpose applies weighted neighbours, since the forward
                // permittivity coefficient multiplies the row Laplacian.
                let weighted = |j| slab.courant_squared_over_epsilon[j] * next_adjoint[j];
                current_adjoint[i] +=
                    2.0 * next_adjoint[i] + weighted(i - 1) - 2.0 * weighted(i) + weighted(i + 1);
                previous_adjoint[i] = -next_adjoint[i];
            }
            std::mem::swap(&mut next_adjoint, &mut current_adjoint);
            std::mem::swap(&mut current_adjoint, &mut previous_adjoint);
        }

        let gradient_difference = (gradient - tangent_gradient).abs();
        let tolerance = 1e-9 * (1.0 + gradient.abs().max(tangent_gradient.abs()));
        if !loss.is_finite() || !gradient_difference.is_finite() || gradient_difference > tolerance
        {
            return Err(
                "Dielectric adjoint and forward sensitivity disagree on the retained trial".into(),
            );
        }
        let samples = predictions
            .into_iter()
            .chain(references)
            .chain(residuals)
            .map(|v| v as f32)
            .collect::<Vec<_>>();
        if samples.iter().any(|v| !v.is_finite()) {
            return Err("Dielectric wave trial exceeds finite DenseF32 output".into());
        }
        let probe_history = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![3, trial.samples as u32],
                samples,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("probe_history".into(), probe_history),
            (
                "probe_channels".into(),
                names(["prediction", "reference", "residual"]),
            ),
            (
                "probe_position_domain_fraction".into(),
                Value::F64(slab.probe as f64 / (n - 1) as f64),
            ),
            (
                "timestep_domain_crossing_fraction".into(),
                Value::F64(COURANT / (n - 1) as f64),
            ),
            ("relative_permittivity".into(), Value::F64(epsilon)),
            ("half_mean_square_error".into(), Value::F64(loss)),
            (
                "gradient_relative_permittivity".into(),
                Value::F64(gradient),
            ),
            (
                "tangent_gradient_relative_permittivity".into(),
                Value::F64(tangent_gradient),
            ),
            (
                "adjoint_tangent_absolute_difference".into(),
                Value::F64(gradient_difference),
            ),
            (
                "model".into(),
                Value::String("fixed_passive_dielectric_slab_scalar_fdtd".into()),
            ),
        ])))
    }
}

impl Trial {
    fn decode(bytes: &[u8]) -> Result<Self, String> {
        let value: Json =
            serde_json::from_slice(bytes).map_err(|err| format!("Dielectric trial JSON: {err}"))?;
        let object = value
            .as_object()
            .ok_or("Dielectric trial must be a JSON object")?;
        let keys = [
            "schema",
            "grid_points",
            "sample_count",
            "probe_reference",
            "reference_relative_permittivity",
        ];
        if object.keys().any(|key| !keys.contains(&key.as_str()))
            || object.get("schema").and_then(Json::as_str) != Some(TRIAL_SCHEMA)
        {
            return Err(format!(
                "Dielectric trial must use schema {TRIAL_SCHEMA} and its declared fields"
            ));
        }
        let points = trial_count(object, "grid_points", 64, 256)?;
        let samples = trial_count(object, "sample_count", 16, 4_096)?;
        let reference = match (
            object.get("probe_reference"),
            object.get("reference_relative_permittivity"),
        ) {
            (Some(lane), None) => {
                let lane = lane
                    .as_array()
                    .filter(|lane| lane.len() == samples)
                    .ok_or("probe_reference must contain sample_count normalized samples")?;
                Reference::Authored(
                    lane.iter()
                        .map(|item| {
                            item.as_f64()
                                .filter(|v| (-4.0..=4.0).contains(v))
                                .ok_or("probe_reference samples must be in [-4, 4]")
                        })
                        .collect::<Result<Vec<_>, _>>()?,
                )
            }
            (None, Some(epsilon)) => Reference::Dielectric(
                epsilon
                    .as_f64()
                    .filter(|v| (MIN_PERMITTIVITY..=MAX_PERMITTIVITY).contains(v))
                    .ok_or("reference_relative_permittivity must be in [1, 12]")?,
            ),
            _ => {
                return Err(
                    "Provide exactly one of probe_reference or reference_relative_permittivity"
                        .into(),
                )
            }
        };
        Ok(Self {
            points,
            samples,
            reference,
        })
    }
}

fn trial_count(
    object: &JsonMap<String, Json>,
    key: &str,
    minimum: u64,
    maximum: u64,
) -> Result<usize, String> {
    object
        .get(key)
        .and_then(Json::as_u64)
        .filter(|value| (minimum..=maximum).contains(value))
        .map(|value| value as usize)
        .ok_or_else(|| format!("{key} must be an integer in [{minimum}, {maximum}]"))
}

impl Slab {
    fn new(points: usize, epsilon: f64) -> Self {
        let last = points - 1;
        let mut coefficient = vec![COURANT * COURANT; points];
        let mut derivative = vec![0.0; points];
        for i in last.div_ceil(3)..(2 * last).div_ceil(3) {
            coefficient[i] = COURANT * COURANT / epsilon;
            derivative[i] = -(COURANT * COURANT) / (epsilon * epsilon);
        }
        Self {
            courant_squared_over_epsilon: coefficient,
            derivative_wrt_epsilon: derivative,
            probe: (5 * last + 3) / 6,
        }
    }

    fn advance(&self, previous: &[f64], current: &[f64], next: &mut [f64]) {
        for i in 1..current.len() - 1 {
            next[i] = 2.0 * current[i] - previous[i]
                + self.courant_squared_over_epsilon[i] * laplacian(current, i);
        }
    }
}

fn laplacian(field: &[f64], i: usize) -> f64 {
    field[i - 1] - 2.0 * field[i] + field[i + 1]
}

fn initial_fields(n: usize) -> (Vec<f64>, Vec<f64>) {
    let dx = 1.0 / (n - 1) as f64;
    let pulse = |x: f64| (1.0 - ((x - 0.17) / 0.10).powi(2)).max(0.0).powi(4);
    let mut previous = vec![0.0; n];
    let mut current = vec![0.0; n];
    for i in 1..n - 1 {
        let x = i as f64 * dx;
        previous[i] = pulse(x + COURANT * dx);
        current[i] = pulse(x);
    }
    (previous, current)
}

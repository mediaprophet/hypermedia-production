//! Fixed dimensionless mass-accounting benchmark for a native ODE integrator.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::solvers::{
    SolverConfig,
    calculus::{ODEFunction, RungeKutta4Static},
};

use super::{numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, required, text};

const SAMPLE_INTERVAL: f64 = 0.125;
const STEP: f64 = 1.0 / 256.0;
const INTERVAL_COUNT: usize = 192;
const STEPS_PER_INTERVAL: usize = 32;
const TOLERANCE: f64 = 1.0e-9;

struct TwoReservoirCapture;

impl ODEFunction for TwoReservoirCapture {
    fn derivatives(&self, _time: f64, state: &[f64; 4]) -> [f64; 4] {
        let exchange = 0.5 * state[0] - state[1];
        let capture = 0.5 * state[0];
        [-exchange - capture, exchange, capture, 0.0]
    }
}

/// The reversible pair has eigenvalues `-1 ± sqrt(1/2)`. Account for the
/// material captured in the sink independently of its numerical integration.
fn analytical_reference(time: f64) -> [f64; 3] {
    let gap = std::f64::consts::FRAC_1_SQRT_2;
    let slow = ((-1.0 + gap) * time).exp();
    let fast = ((-1.0 - gap) * time).exp();
    let primary = 0.5 * (slow + fast);
    let secondary = 0.25 * (slow - fast) / gap;
    [primary, secondary, 1.0 - primary - secondary]
}

impl AssetHost<'_> {
    /// Benchmark a fixed two-reservoir exchange with one irreversible capture sink.
    pub(crate) fn solve_pbpk_pd_adme_clearance(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["operation"])?;
        if text(required(args, "operation")?)? != "normalized_two_reservoir_capture" {
            return Err("Supported operation is normalized_two_reservoir_capture".into());
        }
        self.charge(
            96 * STEPS_PER_INTERVAL * INTERVAL_COUNT,
            4096 * (INTERVAL_COUNT + 1),
        )?;

        let mut solver = RungeKutta4Static::new(
            STEP,
            SolverConfig {
                max_iterations: STEPS_PER_INTERVAL as u32,
                ..SolverConfig::default()
            },
        );
        let mut rows = Vec::with_capacity(INTERVAL_COUNT + 1);
        let mut state = [1.0, 0.0, 0.0, 0.0];
        let mut worst_balance = 0.0_f64;
        let mut worst_reference = 0.0_f64;
        for sample in 0..=INTERVAL_COUNT {
            let time = sample as f64 * SAMPLE_INTERVAL;
            if sample != 0 {
                let prior_capture = state[2];
                state = solver
                    .integrate(&TwoReservoirCapture, time - SAMPLE_INTERVAL, state, time)
                    .map_err(|error| format!("Dimensionless capture RK4 integration: {error:?}"))?
                    .y;
                if state[2] < prior_capture {
                    return Err(format!("Capture decreased at normalized time {time}"));
                }
            }
            let total = state[0] + state[1] + state[2];
            let balance = (total - 1.0).abs();
            let reference_error = analytical_reference(time)
                .iter()
                .zip(&state)
                .fold(0.0_f64, |error, (expected, actual)| {
                    error.max((expected - actual).abs())
                });
            if state[..3]
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
                || balance > TOLERANCE
                || reference_error > TOLERANCE
            {
                return Err(format!(
                    "Mass accounting failed at normalized time {time}: balance {balance:.9e}, reference error {reference_error:.9e}"
                ));
            }
            worst_balance = worst_balance.max(balance);
            worst_reference = worst_reference.max(reference_error);
            rows.push(numeric_row([
                time,
                state[0],
                state[1],
                state[2],
                total,
                balance,
                reference_error,
            ]));
        }
        let mut output = table_receipt(
            self,
            "urn:qualia:civic:normalized-two-reservoir-capture-rk4",
            &[
                "normalized_time",
                "primary_fraction",
                "secondary_fraction",
                "captured_fraction",
                "total_fraction",
                "mass_balance_residual",
                "max_reference_error",
            ],
            &rows,
        )?;
        output.insert(
            "max_mass_balance_residual".into(),
            Value::F64(worst_balance),
        );
        output.insert("max_reference_error".into(), Value::F64(worst_reference));
        Ok(Value::Record(output))
    }
}

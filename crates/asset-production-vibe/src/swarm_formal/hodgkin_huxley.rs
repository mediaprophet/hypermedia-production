//! Offline fixed single-compartment benchmark of Hodgkin and Huxley (1952).
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::solvers::{
    calculus::{ODEFunction, RungeKutta4Static},
    SolverConfig,
};
use sha2::{Digest, Sha256};

use crate::{
    fields, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, Args, AssetHost,
    TypedArtifact, Value,
};

use super::{budget, count_value};

const CONTEXT: &str = "urn:qualia:benchmark:hodgkin-huxley-1952-single-compartment-figure-13-v1";
const SOURCE: &str = "https://doi.org/10.1113/jphysiol.1952.sp004764";
const SAMPLES: usize = 321;
const SAMPLE_PERIOD_MS: f64 = 1.0 / 16.0;
const STEPS_PER_SAMPLE: u32 = 16;
const STEP_MS: f64 = SAMPLE_PERIOD_MS / STEPS_PER_SAMPLE as f64;
const INITIAL_DEPOLARIZATION_MV: f64 = 15.0;

impl AssetHost<'_> {
    pub(crate) fn solve_hodgkin_huxley_connectome_gtop(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["arena_budget_bytes"])?;
        let columns = ["time_ms", "depolarization_mv", "m", "h", "n"];
        let bytes = (SAMPLES + 1) * columns.len() * 64;
        budget(args, bytes)?;
        self.charge((SAMPLES - 1) * STEPS_PER_SAMPLE as usize * 4 * 128, bytes)?;

        let mut book = Workbook::new();
        let sheet_id = book.add_sheet("Hodgkin–Huxley 1952 Figure 13 numerical benchmark");
        let sheet = book
            .sheet_mut(sheet_id)
            .ok_or("Cannot create the benchmark worksheet")?;
        for (column, name) in columns.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::Text(name.into()));
        }

        // The article defines the rates at 6.3 °C. Figure 13 uses 6 °C.
        let model = ClassicalMembrane {
            gate_rate_scale: 3.0f64.powf((6.0 - 6.3) / 10.0),
        };
        let config = SolverConfig {
            max_iterations: STEPS_PER_SAMPLE,
            step_size: STEP_MS,
            ..SolverConfig::default()
        };
        let mut solver = RungeKutta4Static::new(STEP_MS, config);
        let mut state = model.initial_state();
        let mut peak = (0.0, INITIAL_DEPOLARIZATION_MV);
        let mut previous_time = 0.0;
        let mut digest = Sha256::new();
        digest.update(CONTEXT.as_bytes());
        for sample in 0..SAMPLES {
            let time = sample as f64 * SAMPLE_PERIOD_MS;
            if sample != 0 {
                // All times and steps are binary fractions, so each interval is
                // exactly STEPS_PER_SAMPLE native RK4 steps.
                state = solver
                    .integrate(&model, previous_time, state, time)
                    .map_err(|error| {
                        format!("Classical membrane integration failed at {time} ms: {error:?}")
                    })?
                    .y;
            }
            previous_time = time;
            let [v, m, h, n] = state;
            if !v.is_finite()
                || [m, h, n]
                    .into_iter()
                    .any(|gate| !(0.0..=1.0).contains(&gate))
            {
                return Err(format!("Invalid classical membrane state at {time} ms"));
            }
            let depolarization = -v;
            if depolarization > peak.1 {
                peak = (time, depolarization);
            }
            for (column, value) in [time, depolarization, m, h, n].into_iter().enumerate() {
                digest.update(value.to_le_bytes());
                sheet.set_value(
                    CellRef::new(sample as u32 + 1, column as u32),
                    CellValue::Number(value),
                );
            }
        }

        let trace_iri = format!("urn:sha256:{:x}", digest.finalize());
        let mut graph = ReceiptGraph::new(CONTEXT);
        graph.fact(&trace_iri, "urn:qualia:benchmark:model", CONTEXT)?;
        graph.fact(CONTEXT, "http://purl.org/dc/terms/source", SOURCE)?;
        graph.fact(
            CONTEXT,
            "urn:qualia:benchmark:integrator",
            "urn:qualia:integrator:fixed-step-rk4",
        )?;
        let sealed = graph.seal(self.limits.resource_bytes)?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("sample_count".into(), count_value(SAMPLES)),
            ("sampled_peak_depolarization_mv".into(), Value::F64(peak.1)),
            ("sampled_peak_time_ms".into(), Value::F64(peak.0)),
        ])))
    }
}

struct ClassicalMembrane {
    gate_rate_scale: f64,
}

impl ClassicalMembrane {
    fn initial_state(&self) -> [f64; 4] {
        let [(a_m, b_m), (a_h, b_h), (a_n, b_n)] = gate_rates(0.0);
        [
            -INITIAL_DEPOLARIZATION_MV,
            a_m / (a_m + b_m),
            a_h / (a_h + b_h),
            a_n / (a_n + b_n),
        ]
    }
}

impl ODEFunction for ClassicalMembrane {
    fn derivatives(&self, _time: f64, &[v, m, h, n]: &[f64; 4]) -> [f64; 4] {
        // Equation (26), at zero applied current. Keep the paper's voltage
        // sign internally and report the plotted coordinate −V in the table.
        let i_k = 36.0 * n.powi(4) * (v - 12.0);
        let i_na = 120.0 * m.powi(3) * h * (v + 115.0);
        let i_leak = 0.3 * (v + 10.613);
        let capacitance = 1.0;
        let [(a_m, b_m), (a_h, b_h), (a_n, b_n)] = gate_rates(v);
        let gate_derivative =
            |gate, alpha, beta| self.gate_rate_scale * (alpha * (1.0 - gate) - beta * gate);
        [
            -(i_k + i_na + i_leak) / capacitance,
            gate_derivative(m, a_m, b_m),
            gate_derivative(h, a_h, b_h),
            gate_derivative(n, a_n, b_n),
        ]
    }
}

fn gate_rates(v: f64) -> [(f64, f64); 3] {
    [
        (exponential_ratio((v + 25.0) / 10.0), 4.0 * (v / 18.0).exp()),
        (
            0.07 * (v / 20.0).exp(),
            1.0 / (1.0 + ((v + 30.0) / 10.0).exp()),
        ),
        (
            0.1 * exponential_ratio((v + 10.0) / 10.0),
            0.125 * (v / 80.0).exp(),
        ),
    ]
}

fn exponential_ratio(x: f64) -> f64 {
    // x / (exp(x) − 1) has a removable singularity at zero.
    if x == 0.0 {
        1.0
    } else {
        x / x.exp_m1()
    }
}

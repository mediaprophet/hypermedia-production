//! Linear finite port-Hamiltonian interconnection with a discrete power balance.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::solvers::linear_algebra::lu::{lu_decompose, Lu};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, Args, AssetHost, TypedArtifact, Value,
};

use super::budget;

const MAX_N: usize = 64;
const MAX_STEPS: usize = 4096;
const CONTEXT: &str = "urn:qualia:physics:port-hamiltonian-midpoint-v1";

#[derive(Clone, Copy)]
struct Port {
    initial: f64,
    stiffness: f64,
    resistance: f64,
    supplied_flow: f64,
}

#[derive(Clone, Copy)]
struct Step {
    exergy: f64,
    power: f64,
    heat: f64,
    entropy: f64,
    residual: f64,
}

impl AssetHost<'_> {
    pub(crate) fn couple_exergetic_port_hamiltonian_operad(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "ports",
                "connections",
                "dt_s",
                "steps",
                "temperature_k",
                "arena_budget_bytes",
            ],
        )?;
        let input = list(required(args, "ports")?)?;
        let n = input.len();
        if !(1..=MAX_N).contains(&n) {
            return Err(
                "The bounded quadratic storage system admits 1 through 64 scalar ports".into(),
            );
        }
        let dt = num(required(args, "dt_s")?)?;
        let steps = integer(required(args, "steps")?, "steps")?;
        if !(1e-9..=1e3).contains(&dt) || !(1..=MAX_STEPS).contains(&steps) {
            return Err("dt_s or steps is outside the bounded integration scope".into());
        }
        let temperature = num(required(args, "temperature_k")?)?;
        if !(1e-3..=1e6).contains(&temperature) {
            return Err("Temperature must be positive and within 1e-3..=1e6 K".into());
        }
        let edges = list(required(args, "connections")?)?;
        if edges.len() > n * (n - 1) / 2 {
            return Err("Each skew interconnection requires a unique pair of ports".into());
        }
        let scratch = n * n * 3 * size_of::<f64>()
            + (steps + 1) * size_of::<Step>()
            + MAX_N * (size_of::<Port>() + 5 * size_of::<f64>());
        budget(args, scratch)?;
        self.charge(n * n * n + 3 * n * n * steps, scratch)?;
        let mut ports = Vec::with_capacity(n);
        for value in input {
            let row = record(value)?;
            fields(row, &["state", "stiffness", "resistance", "external_flow"])?;
            let port = Port {
                initial: num(required(row, "state")?)?,
                stiffness: num(required(row, "stiffness")?)?,
                resistance: num(required(row, "resistance")?)?,
                supplied_flow: num(required(row, "external_flow")?)?,
            };
            if !(1e-12..=1e12).contains(&port.stiffness)
                || !(0.0..=1e12).contains(&port.resistance)
                || port.initial.abs() > 1e12
                || port.supplied_flow.abs() > 1e12
            {
                return Err("Invalid quadratic storage or resistive port parameter".into());
            }
            ports.push(port);
        }
        let mut skew = vec![0.0; n * n];
        let mut used = [0_u64; MAX_N];
        for value in edges {
            let row = record(value)?;
            fields(row, &["from", "to", "gain"])?;
            let from = integer(required(row, "from")?, "from")?;
            let to = integer(required(row, "to")?, "to")?;
            let gain = num(required(row, "gain")?)?;
            if from >= n || to >= n || from == to || gain.abs() > 1e12 {
                return Err("Invalid skew interconnection".into());
            }
            if used[from] & (1 << to) != 0 {
                return Err("Repeated skew interconnection".into());
            }
            used[from] |= 1 << to;
            used[to] |= 1 << from;
            skew[from * n + to] = gain;
            skew[to * n + from] = -gain;
        }
        let problem = problem_identity(&ports, &skew, dt, steps, temperature);
        for (i, row) in skew.chunks_exact_mut(n).enumerate() {
            row[i] = -ports[i].resistance;
            for (value, port) in row.iter_mut().zip(&ports) {
                *value *= 0.5 * dt * port.stiffness;
            }
        }
        // (I - h A/2) x_next = (I + h A/2) x + h f_external.
        let lhs = skew
            .iter()
            .enumerate()
            .map(|(index, &value)| (if index % n == index / n { 1.0 } else { 0.0 }) - value)
            .collect::<Vec<_>>();
        let lu = lu_decompose(n, &lhs).map_err(|error| format!("{error:?}"))?;
        if lu.singular || lu.lu.iter().any(|value| !value.is_finite()) {
            return Err("The implicit midpoint system is numerically singular".into());
        }
        let mut trace = vec![
            Step {
                exergy: 0.0,
                power: 0.0,
                heat: 0.0,
                entropy: 0.0,
                residual: 0.0
            };
            steps + 1
        ];
        let mut state = [0.0; MAX_N];
        simulate(&ports, &skew, &lu, dt, temperature, &mut trace, &mut state)?;
        let max_residual = trace
            .iter()
            .map(|row| row.residual.abs())
            .fold(0.0_f64, f64::max);
        let mut book = Workbook::new();
        let id = book.add_sheet("Exergetic balance");
        let sheet = book
            .sheet_mut(id)
            .ok_or("Cannot create port-Hamiltonian trace")?;
        for (column, title) in [
            "time_s",
            "stored_exergy_j",
            "external_power_w",
            "dissipation_w",
            "entropy_production_w_k",
            "balance_residual_j",
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(
                CellRef::new(0, column as u32),
                CellValue::Text(title.into()),
            );
        }
        for (i, row) in trace.iter().enumerate() {
            for (column, value) in [
                i as f64 * dt,
                row.exergy,
                row.power,
                row.heat,
                row.entropy,
                row.residual,
            ]
            .into_iter()
            .enumerate()
            {
                sheet.set_value(
                    CellRef::new(i as u32 + 1, column as u32),
                    CellValue::Number(value),
                );
            }
        }
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &problem,
            "urn:qualia:physics:verified-discrete-balance",
            CONTEXT,
        )?;
        receipt.fact(
            &problem,
            "urn:qualia:physics:maximum-balance-residual-j-bits",
            &format!("urn:qualia:binary64:{:016x}", max_residual.to_bits()),
        )?;
        let sealed = receipt.seal(self.limits.resource_bytes)?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("max_balance_residual_j".into(), Value::F64(max_residual)),
            (
                "final_state".into(),
                Value::List(state[..n].iter().copied().map(Value::F64).collect()),
            ),
        ])))
    }
}

fn simulate(
    ports: &[Port],
    midpoint: &[f64],
    lu: &Lu,
    dt: f64,
    temperature: f64,
    trace: &mut [Step],
    state: &mut [f64; MAX_N],
) -> Result<(), String> {
    let n = ports.len();
    let mut rhs = [0.0; MAX_N];
    let mut next = [0.0; MAX_N];
    for (coordinate, port) in state.iter_mut().zip(ports) {
        *coordinate = port.initial;
    }
    trace[0].exergy = exergy(ports, state)?;
    let mut previous = trace[0].exergy;
    for (i, step) in trace.iter_mut().skip(1).enumerate() {
        for (j, row) in midpoint.chunks_exact(n).enumerate() {
            rhs[j] = state[j]
                + dt * ports[j].supplied_flow
                + row.iter().zip(&state[..n]).map(|(a, x)| a * x).sum::<f64>();
        }
        solve_into(lu, &rhs[..n], &mut next[..n])?;
        let new_energy = exergy(ports, &next)?;
        let (mut power, mut heat) = (0.0, 0.0);
        for (j, port) in ports.iter().enumerate() {
            let effort = 0.5 * (state[j] + next[j]) * port.stiffness;
            power += effort * port.supplied_flow;
            heat += (port.resistance * effort) * effort;
        }
        let entropy = heat / temperature;
        let balance = (power - heat) * dt;
        let residual = (new_energy - previous) - balance;
        let cancellation = 128.0 * f64::EPSILON * (previous + new_energy).max(1.0);
        let tolerance = cancellation + 1e-9 * balance.abs();
        if !residual.is_finite()
            || residual.abs() > tolerance
            || !power.is_finite()
            || power.abs() > 1e15
            || !heat.is_finite()
            || !entropy.is_finite()
            || entropy > 1e12
        {
            return Err(format!(
                "Port-Hamiltonian power-balance gate failed at step {}, residual {residual} J",
                i + 1
            ));
        }
        *step = Step {
            exergy: new_energy,
            power,
            heat,
            entropy,
            residual,
        };
        state[..n].copy_from_slice(&next[..n]);
        previous = new_energy;
    }
    Ok(())
}

fn solve_into(lu: &Lu, rhs: &[f64], x: &mut [f64]) -> Result<(), String> {
    let n = lu.n;
    for i in 0..n {
        let mut value = rhs[lu.pivots[i]];
        for j in 0..i {
            value -= lu.lu[i * n + j] * x[j];
        }
        x[i] = value;
    }
    for i in (0..n).rev() {
        let mut value = x[i];
        for j in i + 1..n {
            value -= lu.lu[i * n + j] * x[j];
        }
        x[i] = value / lu.lu[i * n + i];
        if !x[i].is_finite() {
            return Err("Nonfinite implicit midpoint state".into());
        }
    }
    Ok(())
}

fn exergy(ports: &[Port], coordinates: &[f64]) -> Result<f64, String> {
    let result = ports
        .iter()
        .zip(coordinates)
        .map(|(port, state)| 0.5 * port.stiffness * state * state)
        .sum::<f64>();
    if result.is_finite() && result <= 1e18 {
        Ok(result)
    } else {
        Err("Stored exergy is outside the admitted range".into())
    }
}

fn problem_identity(
    ports: &[Port],
    skew: &[f64],
    dt: f64,
    steps: usize,
    temperature: f64,
) -> String {
    let mut sha = Sha256::new();
    sha.update(b"Qualia quadratic storage midpoint v1\0");
    sha.update((ports.len() as u64).to_le_bytes());
    sha.update((steps as u64).to_le_bytes());
    for value in [dt, temperature] {
        sha.update(value.to_le_bytes());
    }
    for port in ports {
        for value in [
            port.initial,
            port.stiffness,
            port.resistance,
            port.supplied_flow,
        ] {
            sha.update(value.to_le_bytes());
        }
    }
    for value in skew {
        sha.update(value.to_le_bytes());
    }
    format!("urn:sha256:{:x}", sha.finalize())
}

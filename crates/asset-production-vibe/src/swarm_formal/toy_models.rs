//! Closed-form dimensionless teaching models for exchange and spin channels.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::BTreeMap,
    f64::consts::{LN_2, TAU},
    sync::Arc,
};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    required, Args, AssetHost, TypedArtifact, Value,
};

use super::{budget, count_value};

const EXCHANGE_CONTEXT: &str = "urn:qualia:teaching:closed-normalized-reversible-a-b-v1";
const SPIN_CONTEXT: &str = "urn:qualia:teaching:abstract-bloch-x-rotation-dephasing-v1";
const MAX_HORIZON: f64 = 32.0;
const MAX_SAMPLES: usize = 4096;
const BLOCH_ROUNDOFF: f64 = 16.0 * f64::EPSILON;

impl AssetHost<'_> {
    pub(crate) fn solve_reaction_network(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "initial_b_fraction",
                "equilibrium_b_fraction",
                "relaxation_horizon",
                "sample_count",
                "arena_budget_bytes",
            ],
        )?;
        let initial = bounded(args, "initial_b_fraction", 0.0, 1.0)?;
        let equilibrium = bounded(args, "equilibrium_b_fraction", 1e-6, 1.0 - 1e-6)?;
        let (horizon, samples) = sampling(args)?;
        let headers = [
            "normalized_relaxation",
            "fraction_a",
            "fraction_b",
            "normalized_net_a_to_b",
        ];
        self.charge(samples * 20, table_budget(args, samples, headers.len())?)?;

        // In normalized time τ, dB/dτ = p(1-B) - (1-p)B = p-B.
        // The exact semigroup is a convex interpolation between initial B and p.
        let rows = sample_grid(horizon, samples).map(|time| {
            let b = (-(-time).exp_m1()).mul_add(equilibrium - initial, initial);
            Ok([time, 1.0 - b, b, equilibrium - b])
        });
        let workbook = trace("Abstract normalized exchange A ↔ B", headers, rows)?;
        let mut receipt = receipt(EXCHANGE_CONTEXT, [initial, equilibrium, horizon], samples)?;
        receipt.fact(
            EXCHANGE_CONTEXT,
            "urn:qualia:teaching:solution-form",
            "urn:qualia:teaching:analytic-closed-two-state-conservative-semigroup",
        )?;
        let sealed = receipt.seal(self.limits.resource_bytes)?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("sample_count".into(), count_value(samples)),
            (
                "equilibrium_a_fraction".into(),
                Value::F64(1.0 - equilibrium),
            ),
            ("equilibrium_b_fraction".into(), Value::F64(equilibrium)),
            ("half_relaxation".into(), Value::F64(LN_2)),
        ])))
    }

    pub(crate) fn evaluate_bio_coherence(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "initial_bloch",
                "turns_per_unit",
                "dephasing_per_unit",
                "relaxation_horizon",
                "sample_count",
                "arena_budget_bytes",
            ],
        )?;
        let initial = list(required(args, "initial_bloch")?)?;
        let [x, y, z] = initial else {
            return Err(
                "An abstract two-level state requires exactly three Bloch coordinates".into(),
            );
        };
        let initial = [num(x)?, num(y)?, num(z)?];
        if norm_squared(initial) > 1.0 {
            return Err("A density state must lie inside the closed Bloch ball".into());
        }
        let turns = bounded(args, "turns_per_unit", -4.0, 4.0)?;
        let damping = bounded(args, "dephasing_per_unit", 0.0, 4.0)?;
        let (horizon, samples) = sampling(args)?;
        let headers = [
            "normalized_time",
            "unitary_x",
            "unitary_y",
            "unitary_z",
            "unitary_population_0",
            "unitary_l1_coherence_z_basis",
            "unitary_purity",
            "dephased_x",
            "dephased_y",
            "dephased_z",
            "dephased_population_0",
            "dephased_l1_coherence_z_basis",
            "dephased_purity",
        ];
        self.charge(samples * 80, table_budget(args, samples, headers.len())?)?;
        let rows = sample_grid(horizon, samples).map(|time| {
            let (unitary, dephased) = spin_channels(initial, turns, damping, time);
            let a = density_observables(unitary)?;
            let b = density_observables(dephased)?;
            Ok([
                time,
                unitary[0],
                unitary[1],
                unitary[2],
                a[0],
                a[1],
                a[2],
                dephased[0],
                dephased[1],
                dephased[2],
                b[0],
                b[1],
                b[2],
            ])
        });
        let workbook = trace("Abstract two-level Bloch channels", headers, rows)?;
        let mut receipt = receipt(
            SPIN_CONTEXT,
            [initial[0], initial[1], initial[2], turns, damping, horizon],
            samples,
        )?;
        receipt.fact(
            SPIN_CONTEXT,
            "urn:qualia:teaching:solution-form",
            "urn:qualia:teaching:exact-commuting-x-axis-rotation-and-dephasing",
        )?;
        let sealed = receipt.seal(self.limits.resource_bytes)?;
        let (unitary, dephased) = spin_channels(initial, turns, damping, horizon);
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(workbook)))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("sample_count".into(), count_value(samples)),
            (
                "final_unitary_bloch".into(),
                Value::List(unitary.into_iter().map(Value::F64).collect()),
            ),
            (
                "final_dephased_bloch".into(),
                Value::List(dephased.into_iter().map(Value::F64).collect()),
            ),
        ])))
    }
}

fn bounded(args: &Args, key: &str, min: f64, max: f64) -> Result<f64, String> {
    let value = num(required(args, key)?)?;
    if (min..=max).contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "{key} is outside the dimensionless teaching interval {min}..={max}"
        ))
    }
}

fn sampling(args: &Args) -> Result<(f64, usize), String> {
    let horizon = bounded(args, "relaxation_horizon", 1e-6, MAX_HORIZON)?;
    let samples = integer(required(args, "sample_count")?, "sample_count")?;
    if !(2..=MAX_SAMPLES).contains(&samples) {
        return Err("sample_count must be in 2..=4096".into());
    }
    Ok((horizon, samples))
}

fn sample_grid(horizon: f64, samples: usize) -> impl Iterator<Item = f64> {
    let denominator = (samples - 1) as f64;
    (0..samples).map(move |index| horizon * (index as f64 / denominator))
}

fn table_budget(args: &Args, samples: usize, columns: usize) -> Result<usize, String> {
    // Reserve bounded cold-table authoring storage. The analytic sample kernel
    // evaluates one row at a time with fixed scalar scratch.
    let bytes = (samples + 1) * columns * 64;
    budget(args, bytes)?;
    Ok(bytes)
}

fn trace<const N: usize>(
    title: &str,
    headers: [&str; N],
    rows: impl Iterator<Item = Result<[f64; N], String>>,
) -> Result<Workbook, String> {
    let mut workbook = Workbook::new();
    let sheet = workbook.add_sheet(title);
    let sheet = workbook
        .sheet_mut(sheet)
        .ok_or("Cannot create the teaching-model worksheet")?;
    for (column, header) in headers.into_iter().enumerate() {
        sheet.set_value(
            CellRef::new(0, column as u32),
            CellValue::Text(header.into()),
        );
    }
    for (index, values) in rows.enumerate() {
        for (column, value) in values?.into_iter().enumerate() {
            if !value.is_finite() {
                return Err("A teaching-model row contains nonfinite output".into());
            }
            sheet.set_value(
                CellRef::new(index as u32 + 1, column as u32),
                CellValue::Number(value),
            );
        }
    }
    Ok(workbook)
}

fn norm_squared([x, y, z]: [f64; 3]) -> f64 {
    x.mul_add(x, y.mul_add(y, z * z))
}

fn spin_channels([x, y, z]: [f64; 3], turns: f64, damping: f64, time: f64) -> ([f64; 3], [f64; 3]) {
    let (sin, cos) = (TAU * turns * time).sin_cos();
    let transverse_y = y.mul_add(cos, -z * sin);
    let transverse_z = y.mul_add(sin, z * cos);
    let coherence_factor = (-damping * time).exp();
    (
        [x, transverse_y, transverse_z],
        [
            x,
            coherence_factor * transverse_y,
            coherence_factor * transverse_z,
        ],
    )
}

fn density_observables([x, y, z]: [f64; 3]) -> Result<[f64; 3], String> {
    let norm = norm_squared([x, y, z]);
    if norm > 1.0 + BLOCH_ROUNDOFF {
        return Err(
            "The computed spin channel left the Bloch ball beyond binary64 roundoff".into(),
        );
    }
    // ρ = ½(I + x X + y Y + z Z): population and L1 coherence are in the Z basis.
    Ok([
        0.5 * (1.0 + z.clamp(-1.0, 1.0)),
        x.hypot(y).min(1.0),
        0.5 * (1.0 + norm.min(1.0)),
    ])
}

fn receipt<const N: usize>(
    context: &str,
    parameters: [f64; N],
    samples: usize,
) -> Result<ReceiptGraph, String> {
    let mut sha = Sha256::new();
    sha.update(context.as_bytes());
    sha.update((samples as u64).to_le_bytes());
    for parameter in parameters {
        sha.update(parameter.to_le_bytes());
    }
    let identity = format!("urn:sha256:{:x}", sha.finalize());
    let mut receipt = ReceiptGraph::new(context);
    receipt.fact(
        &identity,
        "urn:qualia:teaching:dimensionless-model",
        context,
    )?;
    receipt.fact(
        &identity,
        "urn:qualia:teaching:sample-count",
        &format!("urn:qualia:integer:{samples}"),
    )?;
    Ok(receipt)
}

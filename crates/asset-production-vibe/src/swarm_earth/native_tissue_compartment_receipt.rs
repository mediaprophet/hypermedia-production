//! Archived numerical accounting and a fixed dimensionless saturation fixture.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeSet;

use qualia_core_db::{bundle::semantic_model::SemanticModelBuilder, q_hash, NQuin};
use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{fields, integer, list, num, record, required, text, vector, Args, AssetHost, Value};

use super::{bounded, lattice};

const ARCHIVE_MODE: &str = "verify_archived_tissue_and_compartment_receipt";
const FIXTURE_MODE: &str = "step_sterile_wound_repair_fixture";
const TOLERANCE: f64 = 1e-12;
const CELLS: usize = 8;
const RECEIPT_BYTES: usize = 1_048_576;
const RECEIPT_SCRATCH: usize = 4 * RECEIPT_BYTES;

impl AssetHost<'_> {
    pub(crate) fn simulate_immunology_wound_and_seir_network(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            ARCHIVE_MODE => {
                fields(args, &["mode", "archive"])?;
                let archive = match args.get("archive") {
                    Some(value) => Archive::read(value)?,
                    None => Archive::fixture(),
                };
                archive.verify(self)
            }
            FIXTURE_MODE => {
                fields(args, &["mode", "steps"])?;
                let steps = match args.get("steps") {
                    Some(value) => integer(value, "steps")?,
                    None => 16,
                };
                if !(1..=64).contains(&steps) {
                    return Err("Sterile fixture steps must be in 1..=64".into());
                }
                step_fixture(self, steps)
            }
            _ => Err(format!(
                "Supported modes are {ARCHIVE_MODE} and {FIXTURE_MODE}"
            )),
        }
    }
}

#[derive(Clone, Copy)]
struct ArchiveRow {
    pools: [f64; 4],
    margin: [f64; 4],
    density: f64,
}

impl ArchiveRow {
    fn values(self) -> [f64; 9] {
        let [a, b, c, d] = self.pools;
        let [m0, m1, m2, stiffness] = self.margin;
        [a, b, c, d, m0, m1, m2, stiffness, self.density]
    }
}

struct Archive {
    rows: Vec<ArchiveRow>,
    matrix: [[f64; 2]; 2],
    numerator: f64,
    denominator: f64,
}

impl Archive {
    fn fixture() -> Self {
        Self {
            rows: vec![
                ArchiveRow {
                    pools: [0.5, 0.25, 0.125, 0.125],
                    margin: [0.5, 0.25, 0.125, 0.25],
                    density: 0.5,
                },
                ArchiveRow {
                    pools: [0.5, 0.125, 0.25, 0.125],
                    margin: [0.375, 0.25, 0.25, 0.375],
                    density: 0.625,
                },
                ArchiveRow {
                    pools: [0.5, 0.125, 0.125, 0.25],
                    margin: [0.25, 0.125, 0.375, 0.5],
                    density: 0.75,
                },
            ],
            matrix: [[0.0, 1.5], [0.375, 0.0]],
            numerator: 0.375,
            denominator: 0.5,
        }
    }

    fn read(value: &Value) -> Result<Self, String> {
        let archive = record(value)?;
        fields(
            archive,
            &[
                "rows",
                "normalized_matrix",
                "ratio_numerator",
                "ratio_denominator",
            ],
        )?;
        let raw_rows = list(required(archive, "rows")?)?;
        if !(2..=128).contains(&raw_rows.len()) {
            return Err("Archived accounting requires 2..=128 rows".into());
        }
        let rows = raw_rows
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(row, &["pools", "margin", "density"])?;
                Ok(ArchiveRow {
                    pools: vector(required(row, "pools")?)?,
                    margin: vector(required(row, "margin")?)?,
                    density: num(required(row, "density")?)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let [first, second] = list(required(archive, "normalized_matrix")?)? else {
            return Err("normalized_matrix requires two rows of two entries".into());
        };
        Ok(Self {
            rows,
            matrix: [vector(first)?, vector(second)?],
            numerator: bounded(archive, "ratio_numerator", 0.0, 8.0)?,
            denominator: bounded(archive, "ratio_denominator", 1e-6, 1.0)?,
        })
    }

    fn verify(self, host: &mut AssetHost<'_>) -> Result<Value, String> {
        host.charge(24_000 * self.rows.len(), RECEIPT_SCRATCH)?;
        let (radius, ratio) = self.matrix_check()?;
        let initial: f64 = self.rows[0].pools.iter().sum();
        let mut peak_error = 0.0_f64;
        let mut previous_stiffness = 0.0;
        for (index, row) in self.rows.iter().enumerate() {
            if row
                .pools
                .iter()
                .chain(&row.margin)
                .any(|value| !value.is_finite() || *value < 0.0)
            {
                return Err(format!(
                    "Archived row {index} requires finite nonnegative accounting and margin values"
                ));
            }
            if !(0.0..=1.0).contains(&row.density) {
                return Err(format!(
                    "Archived row {index} exceeds the unit density range"
                ));
            }
            let error = (row.pools.iter().sum::<f64>() - initial).abs();
            if !error.is_finite() || error > TOLERANCE {
                return Err(format!(
                    "Archived row {index} violates closed-pool conservation"
                ));
            }
            if row.margin[3] < previous_stiffness {
                return Err(format!(
                    "Archived row {index} decreases the stiffness index"
                ));
            }
            previous_stiffness = row.margin[3];
            peak_error = peak_error.max(error);
        }
        let counts = self.rows.len();
        let columns = [
            "pool_0",
            "pool_1",
            "pool_2",
            "pool_3",
            "margin_0",
            "margin_1",
            "margin_2",
            "stiffness_index",
            "density",
        ];
        let raw = self.rows.iter().map(|row| row.values());
        let mut receipt = CheckedReceipt::new(
            ARCHIVE_MODE,
            raw.clone()
                .flatten()
                .chain(self.matrix.into_iter().flatten())
                .chain([self.numerator, self.denominator]),
        );
        for (index, row) in raw.clone().enumerate() {
            let subject = format!("{}:row:{index}", receipt.context);
            for (&name, value) in columns.iter().zip(row) {
                receipt.number(&subject, name, value)?;
            }
        }
        for (name, value) in [
            ("matrix_00", self.matrix[0][0]),
            ("matrix_01", self.matrix[0][1]),
            ("matrix_10", self.matrix[1][0]),
            ("matrix_11", self.matrix[1][1]),
            ("ratio_numerator", self.numerator),
            ("ratio_denominator", self.denominator),
            ("spectral_radius", radius),
            ("archived_ratio", ratio),
            ("max_closed_pool_residual", peak_error),
            ("closed_pool_tolerance", TOLERANCE),
        ] {
            receipt.metric(name, value)?;
        }
        finish(
            host,
            vec![counts as u32, 9],
            raw.flatten(),
            json!({
                "mode": ARCHIVE_MODE, "unit": "UNITLESS", "columns": columns,
                "rows": counts, "initial_pool_sum": initial, "max_closed_pool_residual": peak_error,
                "spectral_radius": radius, "archived_ratio": ratio, "scope": "offline_arithmetic_only",
                "matrix_check": "archived_matrix_and_ratio_arithmetic",
                "stiffness_source": "supplied_dimensionless_margin_index"
            }),
            receipt,
        )
    }

    fn matrix_check(&self) -> Result<(f64, f64), String> {
        let [[a, b], [c, d]] = self.matrix;
        if [a, b, c, d]
            .iter()
            .any(|entry| !(0.0..=8.0).contains(entry))
        {
            return Err("Archived normalized matrix entries must be in [0,8]".into());
        }
        // For a nonnegative real 2×2 matrix the larger real eigenvalue gives its radius.
        let radius = 0.5 * (a + d + ((a - d).powi(2) + 4.0 * b * c).sqrt());
        let ratio = self.numerator / self.denominator;
        if !ratio.is_finite() || (radius - ratio).abs() > TOLERANCE {
            return Err("Archived matrix spectral radius does not match the supplied ratio".into());
        }
        Ok((radius, ratio))
    }
}

fn step_fixture(host: &mut AssetHost<'_>, steps: usize) -> Result<Value, String> {
    host.charge(32_000 * (steps + 1), RECEIPT_SCRATCH)?;
    // These are dimensionless numerical fixtures. Constants are fixed in the
    // source so this profile has a reproducible finite-volume reference.
    const DRIFT: [f64; CELLS - 1] = [0.0125, 0.025, 0.05, 0.0, -0.05, -0.025, -0.0125];
    const DIFFUSIVITY: f64 = 0.05;
    const STEP: f64 = 0.25;
    const RELAXATION: f64 = 0.125;
    let mut density = [0.75, 0.625, 0.5, 0.125, 0.125, 0.5, 0.625, 0.75];
    let mut stiffness = [0.75, 0.625, 0.5, 0.25, 0.25, 0.5, 0.625, 0.75];
    let initial: f64 = density.iter().sum();
    let mut peak_error = 0.0_f64;
    let mut profile = Vec::with_capacity((steps + 1) * CELLS * 2);
    for index in 0..=steps {
        profile.extend(
            density
                .into_iter()
                .zip(stiffness)
                .flat_map(|(rho, value)| [rho, value]),
        );
        if index == steps {
            break;
        }
        let mut flux = [0.0; CELLS + 1];
        for (i, &drift) in DRIFT.iter().enumerate() {
            let [left, right] = [density[i], density[i + 1]];
            // Local Lax–Friedrichs flux for the saturated scalar with no-flux exterior faces.
            let saturated = |value: f64| value * (1.0 - value);
            flux[i + 1] = (DIFFUSIVITY + 0.5 * drift.abs()) * (left - right)
                + 0.5 * drift * (saturated(left) + saturated(right));
        }
        for i in 0..CELLS {
            density[i] += STEP * (flux[i] - flux[i + 1]);
            let next = stiffness[i] + STEP * RELAXATION * density[i] * (1.0 - stiffness[i]);
            if !(0.0..=1.0).contains(&density[i]) || !(stiffness[i]..=1.0).contains(&next) {
                return Err(format!(
                    "Sterile fixture failed its saturation or monotonicity check at step {}",
                    index + 1
                ));
            }
            stiffness[i] = next;
        }
        let error = (density.iter().sum::<f64>() - initial).abs();
        if error > TOLERANCE {
            return Err(format!(
                "Sterile fixture failed closed-pool conservation at step {}",
                index + 1
            ));
        }
        peak_error = peak_error.max(error);
    }
    let mut receipt = CheckedReceipt::new(FIXTURE_MODE, profile.iter().copied());
    for (index, cell) in profile.chunks_exact(2).enumerate() {
        let subject = format!(
            "{}:step:{}:cell:{}",
            receipt.context,
            index / CELLS,
            index % CELLS
        );
        receipt.number(&subject, "density", cell[0])?;
        receipt.number(&subject, "stiffness_index", cell[1])?;
    }
    receipt.metric("max_closed_pool_residual", peak_error)?;
    receipt.metric("closed_pool_tolerance", TOLERANCE)?;
    finish(
        host,
        vec![(steps + 1) as u32, CELLS as u32, 2],
        profile,
        json!({
            "mode": FIXTURE_MODE, "unit": "UNITLESS", "axes": ["step", "cell", "channel"],
            "channels": ["density", "stiffness_index"], "steps": steps, "cells": CELLS,
            "initial_pool_sum": initial, "max_closed_pool_residual": peak_error, "scope": "fixed_numerical_fixture"
        }),
        receipt,
    )
}

struct CheckedReceipt {
    context: String,
    writer: ReceiptGraph,
    expected: BTreeSet<[u64; 6]>,
}

impl CheckedReceipt {
    fn new(mode: &str, values: impl IntoIterator<Item = f64>) -> Self {
        let mut digest = Sha256::new();
        digest.update(mode.as_bytes());
        for value in values {
            digest.update(value.to_bits().to_le_bytes());
        }
        let context = format!(
            "urn:qualia:dimensionless-audit:v1:{mode}:{:x}",
            digest.finalize()
        );
        Self {
            writer: ReceiptGraph::new(&context),
            context,
            expected: BTreeSet::new(),
        }
    }

    fn metric(&mut self, name: &str, value: f64) -> Result<(), String> {
        let subject = self.context.clone();
        self.number(&subject, name, value)
    }

    fn number(&mut self, subject: &str, name: &str, value: f64) -> Result<(), String> {
        if !value.is_finite() {
            return Err(format!("Cannot seal a nonfinite accounting value {name}"));
        }
        let predicate = format!("urn:qualia:dimensionless-audit:property:{name}");
        let object = format!("urn:qualia:binary64:{:016x}", value.to_bits());
        let [s, p, o, c] = [subject, &predicate, &object, &self.context].map(q_hash);
        if [s, p, o, c].contains(&0) {
            return Err("Accounting receipt contains a zero lexical identifier".into());
        }
        self.writer.fact(subject, &predicate, &object)?;
        self.expected
            .insert([s, p, o, c, 0, NQuin::calculate_parity(s, p, o, c, 0)]);
        Ok(())
    }

    fn seal(self, budget: usize) -> Result<Vec<u8>, String> {
        let bytes = self.writer.seal(budget)?;
        let reader = SemanticModelBuilder::new(budget)
            .merge_q42(&bytes)
            .map_err(|error| format!("Read back dimensionless Q42 receipt: {error}"))?;
        let rows = reader.snapshot_quins();
        for row in rows {
            reader
                .check_quin_tokens(row)
                .map_err(|error| error.to_string())?;
        }
        let actual = rows
            .iter()
            .map(|row| {
                [
                    row.subject,
                    row.predicate,
                    row.object,
                    row.context,
                    row.metadata,
                    row.parity,
                ]
            })
            .collect::<BTreeSet<_>>();
        if actual != self.expected || rows.len() != self.expected.len() {
            return Err("Dimensionless receipt did not preserve all six NQuin columns".into());
        }
        Ok(bytes)
    }
}

fn finish(
    host: &mut AssetHost<'_>,
    axes: Vec<u32>,
    values: impl IntoIterator<Item = f64>,
    report: Json,
    receipt: CheckedReceipt,
) -> Result<Value, String> {
    let samples = values
        .into_iter()
        .map(|value| value as f32)
        .collect::<Vec<_>>();
    if samples.iter().any(|value| !value.is_finite()) {
        return Err("Profile value exceeds finite field precision".into());
    }
    let bytes = receipt.seal(host.limits.resource_bytes.min(RECEIPT_BYTES))?;
    let report = serde_json::to_vec(&report).map_err(|error| error.to_string())?;
    Ok(Value::Record(Args::from([
        ("field".into(), lattice(host, axes, samples)?),
        ("report".into(), host.encode("json", report)?),
        ("receipt".into(), host.encode("q42", bytes)?),
    ])))
}

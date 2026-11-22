//! Bounded offline civics engineering operations.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod calibration;
mod chemistry_diagnostics;
mod circuit;
mod control;
mod cure;
mod harness;
mod hydraulics;
mod mass_balance;
mod microgrid;
mod piezo;
mod traffic;

use std::sync::Arc;

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::solvers::linear_algebra::qr::{qr_factor, qr_solve_least_squares};
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{Args, AssetHost, TypedArtifact, Value, num, number, required};

fn bounded(args: &Args, key: &str, default: Option<f64>, lo: f64, hi: f64) -> Result<f64, String> {
    let value = match default {
        Some(default) => number(args, key, default)?,
        None => num(required(args, key)?)?,
    };
    if (lo..=hi).contains(&value) {
        Ok(value)
    } else {
        Err(format!("{key} outside [{lo}, {hi}]"))
    }
}

fn least_squares(
    rows: usize,
    cols: usize,
    mut a: Vec<f64>,
    mut b: Vec<f64>,
) -> Result<Vec<f64>, String> {
    let mut tau = vec![0.0; cols];
    let mut x = vec![0.0; cols];
    qr_factor(rows, cols, &mut a, &mut tau).map_err(|err| format!("QR factorization: {err:?}"))?;
    qr_solve_least_squares(rows, cols, &a, &tau, &mut b, &mut x)
        .map_err(|err| format!("Singular or ill-conditioned engineering system: {err:?}"))?;
    if x.iter().any(|v| !v.is_finite()) {
        return Err("Engineering solve produced a nonfinite result".into());
    }
    Ok(x)
}

/// Caller precharges scenario work and scratch before forming the table. A stable
/// cell digest binds the typed result to its receipt without embedding every row
/// in the receipt graph.
fn table_receipt(
    host: &mut AssetHost<'_>,
    method: &str,
    columns: &[&str],
    rows: &[Vec<CellValue>],
) -> Result<Args, String> {
    let mut book = Workbook::new();
    let sheet_id = book.add_sheet("Result");
    let sheet = book
        .sheet_mut(sheet_id)
        .ok_or("Unable to allocate workbook result sheet")?;
    let mut digest = Sha256::new();
    digest.update(method.as_bytes());
    digest.update((columns.len() as u64).to_le_bytes());
    digest.update((rows.len() as u64).to_le_bytes());
    for (column, name) in columns.iter().enumerate() {
        digest.update((name.len() as u64).to_le_bytes());
        digest.update(name.as_bytes());
        sheet.set_value(CellRef::new(0, column as u32), CellValue::text(*name));
    }
    for (row, cells) in rows.iter().enumerate() {
        if cells.len() != columns.len() {
            return Err("Engineering workbook row has the wrong width".into());
        }
        for (column, cell) in cells.iter().enumerate() {
            match cell {
                CellValue::Number(value) if value.is_finite() => {
                    digest.update([0]);
                    digest.update(value.to_bits().to_le_bytes());
                }
                CellValue::Bool(value) => digest.update([1, u8::from(*value)]),
                CellValue::Text(value) => {
                    digest.update([2]);
                    digest.update((value.len() as u64).to_le_bytes());
                    digest.update(value.as_bytes());
                }
                _ => return Err("Unsupported or nonfinite engineering workbook cell".into()),
            }
            sheet.set_value(CellRef::new(row as u32 + 1, column as u32), cell.clone());
        }
    }
    let hash = format!("{:x}", digest.finalize());
    let result = format!("urn:sha256:{hash}");
    let mut graph = ReceiptGraph::new("urn:qualia:civic:simulation");
    graph.fact(&result, "urn:qualia:civic:method", method)?;
    graph.fact(
        &result,
        "urn:qualia:civic:scope",
        "urn:qualia:civic:offline-analysis",
    )?;
    let bytes = graph.seal(host.limits.resource_bytes)?;
    let receipt = host.encode("q42", bytes)?;
    let cube = host.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
    Ok(Args::from([
        ("table".into(), cube),
        ("receipt".into(), receipt),
        ("table_sha256".into(), Value::String(hash)),
    ]))
}

fn numeric_row<const N: usize>(values: [f64; N]) -> Vec<CellValue> {
    values.into_iter().map(CellValue::number).collect()
}

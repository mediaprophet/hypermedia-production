//! Inverse of the dimensionless A ↔ B teaching fixture.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    fields, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, required, Args,
    AssetHost, TypedArtifact, Value,
};
use chronicle_tabular_engine::{CellRef, Sheet, Value as CellValue, Workbook};
use sha2::{Digest, Sha256};

const CONTEXT: &str = "urn:qualia:teaching:inverse-normalized-a-b-v1";
const SHEET_NAME: &str = "Abstract normalized exchange A ↔ B";
const HEADERS: [&str; 4] = [
    "normalized_relaxation",
    "fraction_a",
    "fraction_b",
    "normalized_net_a_to_b",
];
const MAX_SAMPLES: usize = 4096;
const ROUNDING_ALLOWANCE: f64 = 32.0 * f64::EPSILON;
const MAX_RATE_ERROR_BOUND: f64 = 1e-3;

struct Sample {
    time: f64,
    fraction_b: f64,
    drive: f64,
}

impl AssetHost<'_> {
    pub(super) fn inverse_normalized_exchange(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "table"])?;
        let handle = required(args, "table")?;
        let count = exchange_table(self, handle)?.1;
        let budget = self.limits.resource_bytes.min(1024 * 1024);
        self.charge(count * 192, budget + count * std::mem::size_of::<Sample>())?;
        let (sheet, _) = exchange_table(self, handle)?;
        let mut samples = Vec::with_capacity(count);
        let mut evidence = Sha256::new();
        evidence.update(b"qualia.inverse-normalized-a-b/v1");
        evidence.update((count as u32).to_le_bytes());
        for index in 0..count {
            let row = index as u32 + 1;
            let [time, a, fraction_b, drive] = read_sample(sheet, row)?;
            if !(0.0..=32.0).contains(&time)
                || !(0.0..=1.0).contains(&a)
                || !(0.0..=1.0).contains(&fraction_b)
                || !(-1.0..=1.0).contains(&drive)
                || (a + fraction_b - 1.0).abs() > ROUNDING_ALLOWANCE
            {
                return Err(format!("Invalid normalized exchange values at row {row}"));
            }
            for value in [time, a, fraction_b, drive] {
                evidence.update(value.to_bits().to_le_bytes());
            }
            samples.push(Sample {
                time,
                fraction_b,
                drive,
            });
        }
        let first = &samples[0];
        let horizon = samples[count - 1].time;
        if first.time != 0.0 || horizon < 1e-6 {
            return Err(
                "Exchange table requires a zero origin and a horizon of at least 0.000001".into(),
            );
        }
        let initial = first.fraction_b;
        // The normalized drive column is p - B. Recover p from that invariant
        // before using independently observed population and time to infer rate.
        let equilibrium = initial + first.drive;
        if !(1e-6 - ROUNDING_ALLOWANCE..=1.0 - 1e-6 + ROUNDING_ALLOWANCE).contains(&equilibrium) {
            return Err("The recovered equilibrium is outside the exchange fixture bounds".into());
        }
        let equilibrium = equilibrium.clamp(1e-6, 1.0 - 1e-6);
        let initial_gap = equilibrium - initial;
        let mut max_trace_error = 0.0_f64;
        let mut rate_probe: Option<(f64, f64, usize)> = None;
        for (index, sample) in samples.iter().enumerate() {
            let time = sample.time;
            let grid_time = horizon * (index as f64 / (count - 1) as f64);
            let exact_b = (-(-time).exp_m1()).mul_add(initial_gap, initial);
            let error = (sample.fraction_b - exact_b).abs();
            if (time - grid_time).abs() > ROUNDING_ALLOWANCE * horizon
                || (index > 0 && time <= samples[index - 1].time)
                || (sample.fraction_b + sample.drive - equilibrium).abs() > ROUNDING_ALLOWANCE
                || error > ROUNDING_ALLOWANCE
            {
                return Err(format!(
                    "Exchange table violates the forward fixture at row {}",
                    index + 1
                ));
            }
            max_trace_error = max_trace_error.max(error);
            if time == 0.0 || initial_gap.abs() <= ROUNDING_ALLOWANCE {
                continue;
            }
            let current_gap = equilibrium - sample.fraction_b;
            let remaining = current_gap / initial_gap;
            let progress = (sample.fraction_b - initial) / initial_gap;
            if !(0.0..1.0).contains(&remaining) || progress <= 0.0 {
                continue;
            }
            let log_remaining = if progress < 0.5 {
                (-progress).ln_1p()
            } else {
                remaining.ln()
            };
            let rate = -log_remaining / time;
            // Sensitivity of the log-rate estimate to fractional readout.
            // Reject nearly static and nearly saturated traces without assigning a rate.
            let error_bound = 2.0 * ROUNDING_ALLOWANCE / (time * current_gap.abs());
            if rate.is_finite()
                && error_bound <= MAX_RATE_ERROR_BOUND
                && rate_probe.is_none_or(|(_, best, _)| error_bound < best)
            {
                rate_probe = Some((rate, error_bound, index));
            }
        }
        let (rate, rate_error_bound, rate_probe) =
            rate_probe.ok_or("Exchange trace does not resolve a normalized relaxation rate")?;
        if (rate - 1.0).abs() > rate_error_bound {
            return Err("The inferred rate is inconsistent with the normalized fixture".into());
        }

        let mut receipt = ReceiptGraph::new(CONTEXT);
        let subject = format!(
            "urn:qualia:teaching:exchange-trace:sha256:{:x}",
            evidence.finalize()
        );
        receipt.fact(
            &subject,
            "urn:qualia:teaching:inverse-clock",
            "urn:qualia:teaching:normalized-relaxation-coordinate",
        )?;
        for (name, value) in [
            ("initial-b-fraction", initial),
            ("equilibrium-b-fraction", equilibrium),
            ("relaxation-horizon", horizon),
            ("normalized-relaxation-rate", rate),
            ("rate-rounding-bound", rate_error_bound),
            ("max-population-residual", max_trace_error),
        ] {
            receipt.fact(
                &subject,
                &format!("urn:qualia:teaching:{name}"),
                &format!("urn:qualia:dimensionless:f64:{:016x}", value.to_bits()),
            )?;
        }
        receipt.fact(
            &subject,
            "urn:qualia:teaching:sample-count",
            &format!("urn:qualia:integer:{count}"),
        )?;
        receipt.fact(
            &subject,
            "urn:qualia:teaching:rate-probe-index",
            &format!("urn:qualia:integer:{rate_probe}"),
        )?;
        let sealed = receipt.seal(budget)?;
        let receipt = self.admit_encoded("q42", sealed)?;
        let forward_args = Value::Record(
            [
                ("initial_b_fraction".into(), Value::F64(initial)),
                ("equilibrium_b_fraction".into(), Value::F64(equilibrium)),
                ("relaxation_horizon".into(), Value::F64(horizon)),
                ("sample_count".into(), Value::U64(count as u64)),
            ]
            .into(),
        );
        Ok(Value::Record(
            [
                ("forward_args".into(), forward_args),
                ("equilibrium_b_fraction".into(), Value::F64(equilibrium)),
                ("normalized_relaxation_rate".into(), Value::F64(rate)),
                ("rate_rounding_bound".into(), Value::F64(rate_error_bound)),
                (
                    "max_population_residual".into(),
                    Value::F64(max_trace_error),
                ),
                ("receipt".into(), receipt),
            ]
            .into(),
        ))
    }
}

fn literal(sheet: &Sheet, row: u32, column: u32) -> Result<&CellValue, String> {
    let cell = sheet
        .cells
        .get(&CellRef::new(row, column))
        .filter(|cell| cell.formula.is_none())
        .ok_or_else(|| {
            format!("Exchange table requires a literal at row {row}, column {column}")
        })?;
    Ok(&cell.value)
}

fn read_sample(sheet: &Sheet, row: u32) -> Result<[f64; 4], String> {
    let mut values = [0.0; 4];
    for (column, output) in values.iter_mut().enumerate() {
        let CellValue::Number(value) = literal(sheet, row, column as u32)? else {
            return Err(format!(
                "Exchange table requires a numeric literal at row {row}, column {column}"
            ));
        };
        *output = *value;
    }
    Ok(values)
}

fn exchange_table<'a>(
    host: &'a AssetHost<'_>,
    handle: &Value,
) -> Result<(&'a Sheet, usize), String> {
    let TypedArtifact::TabularCube(workbook) = host.workspace.typed(handle)? else {
        return Err("Exchange inversion requires a typed TabularCube".into());
    };
    let sheet = exchange_sheet(workbook)?;
    let cells = sheet.cells.len();
    if cells % HEADERS.len() != 0
        || !(3 * HEADERS.len()..=(MAX_SAMPLES + 1) * HEADERS.len()).contains(&cells)
    {
        return Err("Exchange table requires 2 through 4096 complete four-column samples".into());
    }
    for (column, header) in HEADERS.into_iter().enumerate() {
        match literal(sheet, 0, column as u32)? {
            CellValue::Text(value) if value.as_str() == header => {}
            _ => {
                return Err(format!(
                    "Exchange table requires header {header} at column {column}"
                ))
            }
        }
    }
    Ok((sheet, cells / HEADERS.len() - 1))
}

fn exchange_sheet(workbook: &Workbook) -> Result<&Sheet, String> {
    let sheet = match workbook.sheets.as_slice() {
        [sheet] => sheet,
        [blank, sheet]
            if blank.name == "Sheet1"
                && blank.cells.is_empty()
                && blank.spill.is_empty()
                && blank.spill_ranges.is_empty()
                && blank.merges.is_empty() =>
        {
            sheet
        }
        _ => return Err("Exchange table requires a single teaching worksheet".into()),
    };
    if sheet.name != SHEET_NAME
        || !sheet.spill.is_empty()
        || !sheet.spill_ranges.is_empty()
        || !sheet.merges.is_empty()
        || !workbook.names.is_empty()
    {
        return Err("Exchange inversion requires the literal normalized A ↔ B worksheet".into());
    }
    Ok(sheet)
}

//! Dimensionless population exercises and stationary ambient air diagnostics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use chronicle_tabular_engine::Value as CellValue;

use super::{bounded, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, list, num, record, required, text};

impl AssetHost<'_> {
    /// Classroom model for abstract populations A → B → C. No physical species,
    /// units, transport, material parameters, dose or emission data are accepted.
    pub(crate) fn solve_boltzmann_radiation_and_bateman(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &["operation", "normalized_times", "relative_stage_rate"],
        )?;
        if text(required(args, "operation")?)? != "normalized_two_stage_population" {
            return Err("Supported operation is normalized_two_stage_population".into());
        }
        let times = list(required(args, "normalized_times")?)?;
        if !(1..=4096).contains(&times.len()) {
            return Err("Population exercise requires 1–4096 time samples".into());
        }
        self.charge(64 * times.len(), 4096 * times.len())?;
        let relative_stage_rate = bounded(args, "relative_stage_rate", None, 0.0, 100.0)?;
        let separation = (relative_stage_rate - 1.0).abs();
        let mut previous = -1.0;
        let mut worst = 0.0_f64;
        let mut rows = Vec::with_capacity(times.len());
        for time in times {
            let t = num(time)?;
            if !(0.0..=1000.0).contains(&t) || t <= previous {
                return Err("Normalized times must strictly increase within [0, 1000]".into());
            }
            previous = t;
            let a = (-t).exp();
            let converted = -(-t).exp_m1();
            let b = if relative_stage_rate == 0.0 {
                converted
            } else if separation == 0.0 {
                t * a
            } else {
                let slower = relative_stage_rate.min(1.0);
                -(-separation * t).exp_m1() / separation * (-slower * t).exp()
            };
            let terminal = converted - b;
            if terminal < -16.0 * f64::EPSILON {
                return Err(format!(
                    "Terminal population lost positivity at normalized time {t}: {terminal:.9e}"
                ));
            }
            let c = terminal.max(0.0);
            let conservation = (a + b + c - 1.0).abs();
            if conservation > 16.0 * f64::EPSILON {
                return Err(format!(
                    "Population conservation residual {conservation:.9e} at normalized time {t}"
                ));
            }
            worst = worst.max(conservation);
            rows.push(numeric_row([t, relative_stage_rate, a, b, c, conservation]));
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:normalized-two-stage-population",
            &[
                "normalized_time",
                "relative_stage_rate",
                "population_a",
                "population_b",
                "population_c",
                "conservation_residual",
            ],
            &rows,
        )?;
        out.insert("max_conservation_residual".into(), Value::F64(worst));
        Ok(Value::Record(out))
    }

    /// Evaluates an observed ambient NO/NO₂/O₃ snapshot against the Leighton
    /// stationary relation with observed/estimated local rate coefficients.
    /// This is a diagnostic of two rates, with no source or transport model.
    pub(crate) fn solve_atmospheric_photochemistry_dispersion(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["operation", "observations"])?;
        if text(required(args, "operation")?)? != "ambient_photostationary_diagnostic" {
            return Err("Supported operation is ambient_photostationary_diagnostic".into());
        }
        let observations = list(required(args, "observations")?)?;
        if !(1..=2048).contains(&observations.len()) {
            return Err("Ambient diagnostic requires 1–2048 observation snapshots".into());
        }
        self.charge(128 * observations.len(), 6144 * observations.len())?;
        let mut rows = Vec::with_capacity(observations.len());
        let mut informative = 0;
        let mut worst = 0.0_f64;
        for (i, observation) in observations.iter().enumerate() {
            let row = record(observation)?;
            fields(
                row,
                &[
                    "no_ppb",
                    "no2_ppb",
                    "ozone_ppb",
                    "no2_photolysis_per_s",
                    "oxidation_per_ppb_s",
                ],
            )?;
            let no = bounded(row, "no_ppb", None, 0.0, 500.0)?;
            let no2 = bounded(row, "no2_ppb", None, 0.0, 500.0)?;
            let o3 = bounded(row, "ozone_ppb", None, 0.0, 500.0)?;
            let photolysis = bounded(row, "no2_photolysis_per_s", None, 0.0, 1.0)?;
            let oxidation = bounded(row, "oxidation_per_ppb_s", None, 0.0, 0.1)?;
            // Leighton, Photochemistry of Air Pollution (1961): J[NO₂] = k[NO][O₃].
            let release = photolysis * no2;
            let uptake = oxidation * no * o3;
            let difference = release - uptake;
            let total = release + uptake;
            let available = total > 0.0;
            let normalized = if available { difference / total } else { 0.0 };
            if available {
                informative += 1;
                worst = worst.max(normalized.abs());
            }
            let mut output = numeric_row([
                i as f64, no, no2, o3, photolysis, oxidation, release, uptake, difference,
                normalized,
            ]);
            output.push(CellValue::Bool(available));
            rows.push(output);
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:ambient-leighton-snapshot",
            &[
                "observation",
                "no_ppb",
                "no2_ppb",
                "ozone_ppb",
                "photolysis_per_s",
                "oxidation_per_ppb_s",
                "photolysis_rate_ppb_s",
                "oxidation_rate_ppb_s",
                "stationary_difference_ppb_s",
                "normalized_difference",
                "diagnostic_defined",
            ],
            &rows,
        )?;
        out.insert("defined_diagnostic_count".into(), Value::U64(informative));
        out.insert(
            "max_abs_normalized_difference".into(),
            if informative > 0 {
                Value::F64(worst)
            } else {
                Value::Null
            },
        );
        Ok(Value::Record(out))
    }
}

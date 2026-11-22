//! Offline radial calibration and frame alignment from measured samples.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded, least_squares, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, list, record, required, vector};

impl AssetHost<'_> {
    pub(crate) fn calibrate_icvfx_led_volume_stage(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "image_samples",
                "clock_samples_s",
                "frame_rate_hz",
                "overscan_ratio",
                "max_normalized_fit_rmse",
            ],
        )?;
        let samples = list(required(args, "image_samples")?)?;
        let stamps = list(required(args, "clock_samples_s")?)?;
        if !(3..=4096).contains(&samples.len()) || !(2..=4096).contains(&stamps.len()) {
            return Err(
                "Calibration requires 3–4096 image samples and 2–4096 timestamp pairs".into(),
            );
        }
        self.charge(
            (samples.len() + stamps.len()) * 64,
            (samples.len() + stamps.len()) * 4096,
        )?;
        let fps = bounded(args, "frame_rate_hz", None, 23.976, 240.0)?;
        let overscan = bounded(args, "overscan_ratio", Some(1.0), 1.0, 2.0)?;
        let rmse_limit = bounded(args, "max_normalized_fit_rmse", None, 1e-9, 1.0)?;
        let mut pairs = Vec::with_capacity(samples.len());
        let mut a = Vec::with_capacity(4 * samples.len());
        let mut b = Vec::with_capacity(2 * samples.len());
        let mut max_radius2 = 0.0_f64;
        for value in samples {
            let row = record(value)?;
            fields(row, &["ideal", "observed"])?;
            let [x, y] = vector::<2>(required(row, "ideal")?)?;
            let [u, v] = vector::<2>(required(row, "observed")?)?;
            if [x, y, u, v].iter().any(|coordinate| coordinate.abs() > 8.0) {
                return Err(
                    "Calibration coordinates must be normalized around a supplied optical center"
                        .into(),
                );
            }
            let r2 = x * x + y * y;
            max_radius2 = max_radius2.max(r2);
            a.extend([x * r2, x * r2 * r2, y * r2, y * r2 * r2]);
            b.extend([u - x, v - y]);
            pairs.push(([x, y], [u, v]));
        }
        let fit = least_squares(2 * samples.len(), 2, a, b)?;
        let [k1, k2] = [fit[0], fit[1]];
        let derivative = |radius2: f64| 1.0 + 3.0 * k1 * radius2 + 5.0 * k2 * radius2 * radius2;
        let radius2 = max_radius2 * overscan * overscan;
        let interior = if k2 > 0.0 {
            (-3.0 * k1 / (10.0 * k2)).clamp(0.0, radius2)
        } else {
            radius2
        };
        let min_slope = derivative(radius2).min(derivative(interior)).min(1.0);
        if !min_slope.is_finite() || min_slope <= 0.0 {
            return Err(format!(
                "Fitted radial projection folds over the requested overscan: derivative {min_slope}"
            ));
        }
        let mut rows = Vec::with_capacity(pairs.len() + stamps.len() + 1);
        let mut errors = 0.0;
        for (i, ([x, y], [u, v])) in pairs.into_iter().enumerate() {
            let r2 = x * x + y * y;
            let radial = 1.0 + k1 * r2 + k2 * r2 * r2;
            let p = [x * radial, y * radial];
            let err = (p[0] - u).hypot(p[1] - v);
            errors += err * err;
            rows.push(numeric_row([0.0, i as f64, u, v, p[0], p[1], err]));
        }
        let rmse = (errors / samples.len() as f64).sqrt();
        if rmse > rmse_limit {
            return Err(format!(
                "Radial fit residual {rmse:.9e} exceeds requested {rmse_limit:.9e}"
            ));
        }
        let mut clock = Vec::with_capacity(stamps.len());
        let mut previous = None::<[f64; 2]>;
        for value in stamps {
            let pair = vector::<2>(value)?;
            if pair.iter().any(|t| !(0.0..=1e9).contains(t))
                || previous.is_some_and(|last| last[0] >= pair[0] || last[1] >= pair[1])
            {
                return Err("Calibration timestamp pairs must be nonnegative and strictly ordered in both clocks".into());
            }
            if (pair[1] - pair[0]).abs() > 0.1 {
                return Err("Observed clock offset outside ±0.1 s calibration scope".into());
            }
            clock.push(pair);
            previous = Some(pair);
        }
        let phase = clock
            .iter()
            .map(|[local, reference]| reference - local)
            .sum::<f64>()
            / clock.len() as f64;
        let mut worst_clock_residual = 0.0_f64;
        let epoch = clock[0][1];
        for (i, [local, reference]) in clock.into_iter().enumerate() {
            let corrected = local + phase;
            let residual = corrected - reference;
            worst_clock_residual = worst_clock_residual.max(residual.abs());
            rows.push(numeric_row([
                1.0,
                i as f64,
                local,
                reference,
                corrected,
                ((corrected - epoch) * fps).round(),
                residual,
            ]));
        }
        // Record the coefficients in the attested table along with the samples.
        rows.push(numeric_row([2.0, 0.0, fps, overscan, k1, k2, phase]));
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:offline-radial-clock-fit",
            &[
                "record_kind",
                "index",
                "observed_x_or_local_s",
                "observed_y_or_reference_s",
                "fitted_x_or_corrected_s",
                "fitted_y_or_frame",
                "residual",
            ],
            &rows,
        )?;
        out.insert("radial_k1".into(), Value::F64(k1));
        out.insert("radial_k2".into(), Value::F64(k2));
        out.insert("normalized_fit_rmse".into(), Value::F64(rmse));
        out.insert("clock_phase_offset_s".into(), Value::F64(phase));
        out.insert(
            "max_clock_residual_s".into(),
            Value::F64(worst_clock_residual),
        );
        Ok(Value::Record(out))
    }
}

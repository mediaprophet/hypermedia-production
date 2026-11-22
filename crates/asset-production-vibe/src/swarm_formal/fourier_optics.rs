//! Dimensionless one-dimensional split-step Fourier optics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of, sync::Arc};

use qualia_core_db::audio::qac::spatial_dsp::fft_radix2_interleaved_f32;
use sha2::{Digest, Sha256};

use crate::{
    artifact::FieldLattice, fields, integer, list, num,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, required, Args, AssetHost,
    TypedArtifact, Value,
};

use super::budget;

const CONTEXT: &str = "urn:qualia:optics:dimensionless-defocusing-ssfm-v1";
const MAX_SAMPLES: usize = 1024;
const MAX_STEPS: usize = 128;
const MASS_TOLERANCE: f64 = 2e-5;

impl AssetHost<'_> {
    pub(crate) fn solve_adaptive_optics_and_nonlinear_nlse(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "field",
                "phase_screen_rad",
                "dispersion",
                "defocusing",
                "step",
                "steps",
                "arena_budget_bytes",
            ],
        )?;
        let input = list(required(args, "field")?)?;
        let n = input.len();
        if !(8..=MAX_SAMPLES).contains(&n) || !n.is_power_of_two() {
            return Err(
                "Fourier optics admits a power-of-two complex signal of 8 through 1024 samples"
                    .into(),
            );
        }
        let phase_screen = args.get("phase_screen_rad").map(list).transpose()?;
        if phase_screen.is_some_and(|values| values.len() != n) {
            return Err("The simulated phase screen must match the complex signal length".into());
        }
        let dispersion = bounded(required(args, "dispersion")?, 0.0, 0.04, "dispersion")?;
        let defocusing = bounded(required(args, "defocusing")?, 0.0, 1.0, "defocusing")?;
        let step = bounded(required(args, "step")?, 1e-5, 0.05, "step")?;
        let steps = integer(required(args, "steps")?, "steps")?;
        if !(1..=MAX_STEPS).contains(&steps) {
            return Err("Fourier optics steps must be in 1..=128".into());
        }
        let scratch = n * (2 * size_of::<f32>() + size_of::<[f64; 2]>());
        budget(args, scratch)?;
        self.charge(n * (12 * n.ilog2() as usize + 12) * steps, scratch)?;

        let mut field = Vec::with_capacity(2 * n);
        let mut sha = Sha256::new();
        sha.update(CONTEXT.as_bytes());
        sha.update((n as u64).to_le_bytes());
        sha.update((steps as u64).to_le_bytes());
        for value in [dispersion, defocusing, step] {
            sha.update(value.to_le_bytes());
        }
        for value in input {
            let pair = list(value)?;
            let [real, imaginary] = pair else {
                return Err("Each Fourier optics field sample is [real, imaginary]".into());
            };
            let real = num(real)?;
            let imaginary = num(imaginary)?;
            if real.hypot(imaginary) > 1.0 {
                return Err("Dimensionless input amplitude must be at most one".into());
            }
            let stored = [real as f32, imaginary as f32];
            for value in stored {
                sha.update(value.to_le_bytes());
            }
            field.extend(stored);
        }
        let initial_power = power(&field);
        if initial_power < 1e-12 {
            return Err("Input field has no resolvable power".into());
        }
        let coherence_before = coherent_ratio(&field);
        if let Some(phase_screen) = phase_screen {
            sha.update([1]);
            for (pair, phase) in field.chunks_exact_mut(2).zip(phase_screen) {
                let phase = bounded(
                    phase,
                    -std::f64::consts::PI,
                    std::f64::consts::PI,
                    "phase_screen_rad sample",
                )?;
                sha.update(phase.to_le_bytes());
                let (sin, cos) = (-phase).sin_cos();
                rotate(pair, cos, sin);
            }
        } else {
            sha.update([0]);
        }
        let coherence_after = coherent_ratio(&field);
        let propagator: Vec<[f64; 2]> = (0..n)
            .map(|index| {
                let mode = if index < n / 2 {
                    index as f64
                } else {
                    index as f64 - n as f64
                };
                let (sin, cos) = (-0.5 * dispersion * step * mode * mode).sin_cos();
                [cos, sin]
            })
            .collect();
        let mut maximum_relative_power_error = 0.0_f64;
        for index in 0..steps {
            nonlinear_half_step(&mut field, defocusing * step * 0.5);
            fft_radix2_interleaved_f32(&mut field, false).map_err(|error| format!("{error:?}"))?;
            for (pair, &[cos, sin]) in field.chunks_exact_mut(2).zip(&propagator) {
                rotate(pair, cos, sin);
            }
            fft_radix2_interleaved_f32(&mut field, true).map_err(|error| format!("{error:?}"))?;
            nonlinear_half_step(&mut field, defocusing * step * 0.5);
            let error = ((power(&field) - initial_power) / initial_power).abs();
            if !error.is_finite() || error > MASS_TOLERANCE {
                return Err(format!("Fourier optics power conservation failed at step {}, relative residual {error}", index + 1));
            }
            maximum_relative_power_error = maximum_relative_power_error.max(error);
        }
        let identity = format!("urn:sha256:{:x}", sha.finalize());
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(&identity, "urn:qualia:optics:propagation-model", CONTEXT)?;
        receipt.fact(
            &identity,
            "urn:qualia:optics:power-residual-bits",
            &format!(
                "urn:qualia:binary64:{:016x}",
                maximum_relative_power_error.to_bits()
            ),
        )?;
        let sealed = receipt.seal(self.limits.resource_bytes)?;
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![n as u32, 2],
                samples: field,
            },
        )))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("field".into(), field),
            ("receipt".into(), receipt),
            (
                "normalized_propagation".into(),
                Value::F64(steps as f64 * step),
            ),
            (
                "maximum_relative_power_error".into(),
                Value::F64(maximum_relative_power_error),
            ),
            (
                "coherent_ratio_before_phase_compensation".into(),
                Value::F64(coherence_before),
            ),
            (
                "coherent_ratio_after_phase_compensation".into(),
                Value::F64(coherence_after),
            ),
        ])))
    }
}

fn bounded(value: &Value, low: f64, high: f64, name: &str) -> Result<f64, String> {
    let value = num(value)?;
    if (low..=high).contains(&value) {
        Ok(value)
    } else {
        Err(format!("{name} exceeds the dimensionless optics bound"))
    }
}

fn rotate(pair: &mut [f32], cos: f64, sin: f64) {
    let real = f64::from(pair[0]);
    let imaginary = f64::from(pair[1]);
    pair[0] = (cos * real - sin * imaginary) as f32;
    pair[1] = (sin * real + cos * imaginary) as f32;
}

fn nonlinear_half_step(field: &mut [f32], scaling: f64) {
    for pair in field.chunks_exact_mut(2) {
        let real = f64::from(pair[0]);
        let imaginary = f64::from(pair[1]);
        let (sin, cos) = (-scaling * (real * real + imaginary * imaginary)).sin_cos();
        rotate(pair, cos, sin);
    }
}

fn power(field: &[f32]) -> f64 {
    field.iter().map(|&value| f64::from(value).powi(2)).sum()
}

fn coherent_ratio(field: &[f32]) -> f64 {
    let (mut real, mut imaginary, mut amplitude) = (0.0, 0.0, 0.0);
    for pair in field.chunks_exact(2) {
        let re = f64::from(pair[0]);
        let im = f64::from(pair[1]);
        real += re;
        imaginary += im;
        amplitude += re.hypot(im);
    }
    (real.hypot(imaginary) / amplitude).powi(2).min(1.0)
}

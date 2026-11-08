//! Second-order IIR biquad filters designed from the RBJ Audio EQ Cookbook formulas.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Filter response shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass,
    Notch,
    Peak,
    LowShelf,
    HighShelf,
    AllPass,
}

/// Normalized biquad coefficients ($a_0 = 1$).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BiquadCoeffs {
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
}

impl Default for BiquadCoeffs {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl BiquadCoeffs {
    pub const IDENTITY: Self = Self {
        b0: 1.0,
        b1: 0.0,
        b2: 0.0,
        a1: 0.0,
        a2: 0.0,
    };

    /// Designs an audio filter from frequency, Q factor, gain (dB), and sample rate.
    pub fn design(kind: FilterType, freq: f32, q: f32, gain_db: f32, sample_rate: f32) -> Self {
        let sr = (sample_rate as f64).clamp(8000.0, 384_000.0);
        let f = (freq as f64).clamp(10.0, sr * 0.49);
        let q_val = (q as f64).clamp(0.05, 50.0);
        let g = (gain_db as f64).clamp(-48.0, 48.0);

        let w0 = 2.0 * PI * f / sr;
        let (sw, cw) = w0.sin_cos();
        let alpha = sw / (2.0 * q_val);
        let a = 10f64.powf(g / 40.0);

        let (b0, b1, b2, a0, a1, a2) = match kind {
            FilterType::LowPass => (
                (1.0 - cw) / 2.0,
                1.0 - cw,
                (1.0 - cw) / 2.0,
                1.0 + alpha,
                -2.0 * cw,
                1.0 - alpha,
            ),
            FilterType::HighPass => (
                (1.0 + cw) / 2.0,
                -(1.0 + cw),
                (1.0 + cw) / 2.0,
                1.0 + alpha,
                -2.0 * cw,
                1.0 - alpha,
            ),
            FilterType::BandPass => (
                alpha,
                0.0,
                -alpha,
                1.0 + alpha,
                -2.0 * cw,
                1.0 - alpha,
            ),
            FilterType::Notch => (
                1.0,
                -2.0 * cw,
                1.0,
                1.0 + alpha,
                -2.0 * cw,
                1.0 - alpha,
            ),
            FilterType::Peak => (
                1.0 + alpha * a,
                -2.0 * cw,
                1.0 - alpha * a,
                1.0 + alpha / a,
                -2.0 * cw,
                1.0 - alpha / a,
            ),
            FilterType::LowShelf => {
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                let sqa = 2.0 * a.sqrt() * alpha;
                (
                    a * (ap1 - am1 * cw + sqa),
                    2.0 * a * (am1 - ap1 * cw),
                    a * (ap1 - am1 * cw - sqa),
                    ap1 + am1 * cw + sqa,
                    -2.0 * (am1 + ap1 * cw),
                    ap1 + am1 * cw - sqa,
                )
            }
            FilterType::HighShelf => {
                let ap1 = a + 1.0;
                let am1 = a - 1.0;
                let sqa = 2.0 * a.sqrt() * alpha;
                (
                    a * (ap1 + am1 * cw + sqa),
                    -2.0 * a * (am1 + ap1 * cw),
                    a * (ap1 + am1 * cw - sqa),
                    ap1 - am1 * cw + sqa,
                    2.0 * (am1 - ap1 * cw),
                    ap1 - am1 * cw - sqa,
                )
            }
            FilterType::AllPass => (
                1.0 - alpha,
                -2.0 * cw,
                1.0 + alpha,
                1.0 + alpha,
                -2.0 * cw,
                1.0 - alpha,
            ),
        };

        Self {
            b0: (b0 / a0) as f32,
            b1: (b1 / a0) as f32,
            b2: (b2 / a0) as f32,
            a1: (a1 / a0) as f32,
            a2: (a2 / a0) as f32,
        }
    }
}

/// Direct Form II Transposed biquad filter instance.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct BiquadFilter {
    pub coeffs: BiquadCoeffs,
    s1: f32,
    s2: f32,
}

impl BiquadFilter {
    pub fn new(coeffs: BiquadCoeffs) -> Self {
        Self {
            coeffs,
            s1: 0.0,
            s2: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }

    #[inline]
    pub fn process_sample(&mut self, x: f32) -> f32 {
        let y = self.coeffs.b0 * x + self.s1;
        self.s1 = self.coeffs.b1 * x - self.coeffs.a1 * y + self.s2;
        self.s2 = self.coeffs.b2 * x - self.coeffs.a2 * y;
        // Denormal flushing
        if self.s1.abs() < 1e-15 {
            self.s1 = 0.0;
        }
        if self.s2.abs() < 1e-15 {
            self.s2 = 0.0;
        }
        y
    }

    pub fn process_buffer(&mut self, buffer: &mut [f32]) {
        for s in buffer.iter_mut() {
            *s = self.process_sample(*s);
        }
    }
}

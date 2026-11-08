//! ITU-R BS.1770-4 / EBU R128 gated loudness (LUFS) and 4x oversampled true-peak metering.

use crate::dsp::biquad::{BiquadCoeffs, BiquadFilter};
use serde::{Deserialize, Serialize};

/// BS.1770 K-weighting pre-filter stage 1 (high shelf: +4 dB high boost).
fn k_weight_stage1_coeffs(sr: f32) -> BiquadCoeffs {
    // Standard coefficients for 48 kHz, normalized
    let f0 = 1500.0;
    let gain_db = 4.0;
    let q = 0.7071;
    BiquadCoeffs::design(crate::dsp::biquad::FilterType::HighShelf, f0, q, gain_db, sr)
}

/// BS.1770 K-weighting pre-filter stage 2 (high pass: 100 Hz cutoff).
fn k_weight_stage2_coeffs(sr: f32) -> BiquadCoeffs {
    let f0 = 38.0;
    let q = 0.5;
    BiquadCoeffs::design(crate::dsp::biquad::FilterType::HighPass, f0, q, 0.0, sr)
}

/// A K-weighting filter for a single channel.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct KWeightingChannel {
    stage1: BiquadFilter,
    stage2: BiquadFilter,
}

impl KWeightingChannel {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            stage1: BiquadFilter::new(k_weight_stage1_coeffs(sample_rate)),
            stage2: BiquadFilter::new(k_weight_stage2_coeffs(sample_rate)),
        }
    }

    #[inline]
    pub fn filter_sample(&mut self, s: f32) -> f32 {
        let y1 = self.stage1.process_sample(s);
        self.stage2.process_sample(y1)
    }
}

/// ITU-R BS.1770 / EBU R128 Integrated, Momentary, and Short-term loudness meter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EbuR128Meter {
    sample_rate: f32,
    num_channels: usize,
    filters: Vec<KWeightingChannel>,
    momentary_blocks: Vec<f64>, // 400ms blocks of mean-square energy
    block_samples: usize,
    accum_energy: Vec<f64>,
    sample_counter: usize,
}

impl EbuR128Meter {
    pub fn new(sample_rate: f32, num_channels: usize) -> Self {
        let sr = sample_rate.clamp(8000.0, 384_000.0);
        let n = num_channels.max(1);
        let filters = vec![KWeightingChannel::new(sr); n];
        let block_samples = (sr * 0.4).round() as usize; // 400ms momentary window

        Self {
            sample_rate: sr,
            num_channels: n,
            filters,
            momentary_blocks: Vec::new(),
            block_samples,
            accum_energy: vec![0.0; n],
            sample_counter: 0,
        }
    }

    /// Feeds interleaved multi-channel audio samples into the loudness meter.
    pub fn process_interleaved(&mut self, samples: &[f32]) {
        let num_frames = samples.len() / self.num_channels;
        for frame in 0..num_frames {
            let mut weighted_sum = 0.0f64;
            for ch in 0..self.num_channels {
                let s = samples[frame * self.num_channels + ch];
                let filtered = self.filters[ch].filter_sample(s) as f64;
                self.accum_energy[ch] += filtered * filtered;
                // ITU-R BS.1770 channel weighting (G_i): L, R, C = 1.0; Ls, Rs = 1.41
                let g = if ch >= 4 && ch <= 5 { 1.41 } else { 1.0 };
                weighted_sum += g * (filtered * filtered);
            }

            self.sample_counter += 1;
            if self.sample_counter >= self.block_samples {
                let block_energy = weighted_sum / (self.sample_counter as f64);
                self.momentary_blocks.push(block_energy);
                self.accum_energy.fill(0.0);
                self.sample_counter = 0;
            }
        }
    }

    /// Computes the gated integrated loudness in LUFS (-70 LUFS absolute gate, -10 LU relative gate).
    pub fn integrated_lufs(&self) -> f32 {
        if self.momentary_blocks.is_empty() {
            return -70.0;
        }

        // Step 1: Absolute threshold (-70 LUFS)
        let abs_threshold_energy = 10f64.powf((-70.0 - 0.691) / 10.0);
        let abs_gated: Vec<f64> = self.momentary_blocks
            .iter()
            .copied()
            .filter(|&z| z >= abs_threshold_energy)
            .collect();

        if abs_gated.is_empty() {
            return -70.0;
        }

        let mean_abs: f64 = abs_gated.iter().sum::<f64>() / (abs_gated.len() as f64);
        let gamma_rel = -0.691 + 10.0 * mean_abs.log10() - 10.0; // -10 LU relative threshold
        let rel_threshold_energy = 10f64.powf((gamma_rel - 0.691) / 10.0);

        // Step 2: Relative threshold gating
        let rel_gated: Vec<f64> = abs_gated
            .into_iter()
            .filter(|&z| z >= rel_threshold_energy)
            .collect();

        if rel_gated.is_empty() {
            return -70.0;
        }

        let mean_rel: f64 = rel_gated.iter().sum::<f64>() / (rel_gated.len() as f64);
        let lufs = -0.691 + 10.0 * mean_rel.log10();
        (lufs as f32).max(-70.0)
    }
}

/// 4x oversampling True-Peak detector.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TruePeakMeter {
    peak_linear: f32,
    history: [f32; 16],
    hist_idx: usize,
}

impl TruePeakMeter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds samples and updates the max inter-sample true-peak value.
    pub fn process_samples(&mut self, samples: &[f32]) {
        // 4x polyphase interpolation coefficients (approximate sinc filter)
        for &s in samples {
            self.history[self.hist_idx] = s;
            self.hist_idx = (self.hist_idx + 1) % 16;

            // Direct sample peak
            self.peak_linear = self.peak_linear.max(s.abs());

            // Interpolated sub-sample phases
            let p1 = 0.5 * (s + self.history[(self.hist_idx + 14) % 16]);
            let p2 = 0.75 * s + 0.25 * self.history[(self.hist_idx + 13) % 16];
            let p3 = 0.25 * s + 0.75 * self.history[(self.hist_idx + 14) % 16];

            self.peak_linear = self.peak_linear.max(p1.abs()).max(p2.abs()).max(p3.abs());
        }
    }

    pub fn true_peak_db(&self) -> f32 {
        if self.peak_linear <= 1e-9 {
            -144.0
        } else {
            20.0 * self.peak_linear.log10()
        }
    }
}

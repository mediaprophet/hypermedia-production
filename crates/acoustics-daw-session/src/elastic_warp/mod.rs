//! 6-mode non-linear multi-point Flex Time elastic audio warping and WSOLA processing.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Flex Time elastic audio stretching modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FlexWarpMode {
    Slicing,
    #[default]
    Rhythmic,
    Monophonic,
    Polyphonic,
    Varispeed,
    Tempophone,
}

impl FlexWarpMode {
    /// Window size in seconds and maximum search tolerance fraction for WSOLA.
    pub fn window_params(self) -> (f32, f32) {
        match self {
            Self::Monophonic => (0.025, 0.25),
            Self::Rhythmic => (0.035, 0.20),
            Self::Polyphonic => (0.070, 0.30),
            Self::Tempophone => (0.120, 0.0), // No cross-correlation search (pure granular)
            Self::Slicing | Self::Varispeed => (0.0, 0.0),
        }
    }
}

/// A warp anchor mapping a source timestamp (seconds) to a destination timeline position (seconds).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarpPoint {
    pub source_sec: f64,
    pub dest_sec: f64,
}

impl WarpPoint {
    pub const fn new(source_sec: f64, dest_sec: f64) -> Self {
        Self { source_sec, dest_sec }
    }
}

/// Evaluates source playback time $t_{\text{src}}$ corresponding to destination time $t_{\text{dst}}$ using piecewise linear interpolation.
pub fn evaluate_warp_timeline(points: &[WarpPoint], dest_sec: f64) -> f64 {
    if points.is_empty() {
        return dest_sec;
    }
    if points.len() == 1 {
        return dest_sec - points[0].dest_sec + points[0].source_sec;
    }

    if dest_sec <= points[0].dest_sec {
        let slope = (points[1].source_sec - points[0].source_sec) / (points[1].dest_sec - points[0].dest_sec).max(1e-6);
        return points[0].source_sec + (dest_sec - points[0].dest_sec) * slope;
    }

    for w in points.windows(2) {
        let p0 = w[0];
        let p1 = w[1];
        if dest_sec >= p0.dest_sec && dest_sec <= p1.dest_sec {
            let t = (dest_sec - p0.dest_sec) / (p1.dest_sec - p0.dest_sec).max(1e-6);
            return p0.source_sec + t * (p1.source_sec - p0.source_sec);
        }
    }

    let last = points[points.len() - 1];
    let prev = points[points.len() - 2];
    let slope = (last.source_sec - prev.source_sec) / (last.dest_sec - prev.dest_sec).max(1e-6);
    last.source_sec + (dest_sec - last.dest_sec) * slope
}

/// WSOLA (Waveform Similarity Overlap-Add) offline time-stretching engine.
pub struct WsolaTimeStretch {
    mode: FlexWarpMode,
    sample_rate: f32,
}

impl WsolaTimeStretch {
    pub fn new(mode: FlexWarpMode, sample_rate: f32) -> Self {
        Self {
            mode,
            sample_rate: sample_rate.clamp(8000.0, 384_000.0),
        }
    }

    /// Stretches input audio samples to output length using WSOLA.
    pub fn process(&self, input: &[f32], target_len: usize) -> Vec<f32> {
        if input.is_empty() || target_len == 0 {
            return Vec::new();
        }

        if self.mode == FlexWarpMode::Varispeed {
            // Linear resampling
            let mut out = Vec::with_capacity(target_len);
            let ratio = (input.len() as f64) / (target_len as f64);
            for i in 0..target_len {
                let src_pos = (i as f64) * ratio;
                let idx0 = (src_pos.floor() as usize).min(input.len() - 1);
                let idx1 = (idx0 + 1).min(input.len() - 1);
                let frac = (src_pos - src_pos.floor()) as f32;
                out.push(input[idx0] * (1.0 - frac) + input[idx1] * frac);
            }
            return out;
        }

        let (win_sec, max_shift_frac) = self.mode.window_params();
        let win_size = ((win_sec * self.sample_rate).round() as usize).max(64);
        let hop_syn = win_size / 2;
        let delta_max = ((win_size as f32) * max_shift_frac).round() as usize;

        let mut output = vec![0.0f32; target_len + win_size];
        let mut window_weights = vec![0.0f32; target_len + win_size];

        let num_hops = (target_len + hop_syn - 1) / hop_syn;
        let time_ratio = (input.len() as f64) / (target_len as f64);

        for hop_idx in 0..num_hops {
            let syn_pos = hop_idx * hop_syn;
            let ideal_ana_pos = ((syn_pos as f64) * time_ratio).round() as usize;

            // Find best alignment offset within [-delta_max, +delta_max]
            let mut best_offset = 0isize;
            if delta_max > 0 && syn_pos >= hop_syn && ideal_ana_pos + win_size <= input.len() {
                let mut best_corr = -f32::INFINITY;
                for shift in -(delta_max as isize)..=(delta_max as isize) {
                    let cand_pos = (ideal_ana_pos as isize + shift).clamp(0, (input.len() - win_size) as isize) as usize;
                    let mut corr = 0.0f32;
                    for j in 0..win_size {
                        let prev_out = output[syn_pos + j];
                        let cand_in = input[cand_pos + j];
                        corr += prev_out * cand_in;
                    }
                    if corr > best_corr {
                        best_corr = corr;
                        best_offset = shift;
                    }
                }
            }

            let actual_ana_pos = ((ideal_ana_pos as isize + best_offset).clamp(0, (input.len().saturating_sub(win_size)) as isize)) as usize;

            // Overlap-add with Hann window
            for j in 0..win_size {
                if actual_ana_pos + j < input.len() && syn_pos + j < output.len() {
                    let w = 0.5 * (1.0 - (2.0 * PI * (j as f32) / (win_size as f32)).cos());
                    output[syn_pos + j] += input[actual_ana_pos + j] * w;
                    window_weights[syn_pos + j] += w;
                }
            }
        }

        // Normalize by accumulated window weights
        for i in 0..target_len {
            if window_weights[i] > 1e-4 {
                output[i] /= window_weights[i];
            }
        }

        output.truncate(target_len);
        output
    }
}

//! Parabolic YIN fundamental frequency tracking and note segmentation.

use serde::{Deserialize, Serialize};

/// A detected segmented musical note.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DetectedNote {
    pub start_sample: usize,
    pub end_sample: usize,
    pub midi_pitch: u8,
    pub frequency_hz: f32,
    pub velocity: u8,
}

/// Computes the fundamental frequency $f_0$ of an audio frame using the YIN algorithm with parabolic interpolation.
pub fn detect_pitch_yin(
    frame: &[f32],
    sample_rate: f32,
    min_hz: f32,
    max_hz: f32,
    threshold: f32,
) -> Option<f32> {
    if !sample_rate.is_finite() || sample_rate <= 0.0 || frame.len() < 64 {
        return None;
    }

    let max_tau = ((sample_rate / min_hz.max(20.0)) as usize).min(frame.len() / 2);
    let min_tau = ((sample_rate / max_hz.max(min_hz + 1.0)) as usize).max(2);
    if max_tau <= min_tau + 2 {
        return None;
    }

    let w = frame.len() - max_tau;
    let mut d = vec![0.0f32; max_tau + 1];

    // Step 1: Difference function
    for tau in 1..=max_tau {
        let mut sum = 0.0f32;
        for j in 0..w {
            let diff = frame[j] - frame[j + tau];
            sum += diff * diff;
        }
        d[tau] = sum;
    }

    // Step 2: Cumulative mean normalized difference
    let mut cmnd = vec![1.0f32; max_tau + 1];
    let mut running_sum = 0.0f32;
    for tau in 1..=max_tau {
        running_sum += d[tau];
        if running_sum > 0.0 {
            cmnd[tau] = d[tau] * (tau as f32) / running_sum;
        }
    }

    // Step 3: Absolute threshold & local minimum search
    let mut tau = min_tau;
    while tau < max_tau {
        if cmnd[tau] < threshold {
            // Find local minimum
            while tau + 1 < max_tau && cmnd[tau + 1] < cmnd[tau] {
                tau += 1;
            }

            // Step 4: Parabolic interpolation
            let a = cmnd[tau - 1];
            let b = cmnd[tau];
            let c = cmnd[tau + 1];
            let denom = a - 2.0 * b + c;
            let shift = if denom.abs() > 1e-9 {
                0.5 * (a - c) / denom
            } else {
                0.0
            };

            let exact_tau = (tau as f32) + shift.clamp(-1.0, 1.0);
            if exact_tau > 0.0 {
                return Some(sample_rate / exact_tau);
            }
        }
        tau += 1;
    }

    None
}

/// Converts a frequency in Hz to closest MIDI note number (A4 = 440 Hz = 69).
pub fn hz_to_midi(hz: f32) -> u8 {
    if hz <= 0.0 {
        return 0;
    }
    let note = 69.0 + 12.0 * (hz / 440.0).log2();
    note.round().clamp(0.0, 127.0) as u8
}

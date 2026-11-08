//! Contactless vital signs extraction (respiration, cardiac pulse) and 3-frame debounced Doppler fall detection.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Vital sign estimation summary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct VitalSigns {
    pub breathing_bpm: f32,
    pub heart_rate_bpm: f32,
    pub breathing_confidence: f32,
    pub heart_rate_confidence: f32,
}

/// Fall detection state machine states.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallState {
    Normal,
    PreImpactSpike,
    PostImpactQuiescence(usize), // Count of quiescent frames (debouncing)
    FallConfirmed,
}

/// Simple second-order bandpass IIR filter for vital sign frequencies.
#[derive(Clone, Copy, Debug, PartialEq)]
struct BandpassFilter {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BandpassFilter {
    fn new(f_center: f32, q: f32, sr: f32) -> Self {
        let w0 = 2.0 * PI * f_center / sr;
        let alpha = w0.sin() / (2.0 * q);
        let b0 = alpha;
        let b1 = 0.0;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * w0.cos();
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    #[inline]
    fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}

/// Extracts respiration and cardiac pulse rates from a time-series of CSI amplitude / sanitized phase variations.
pub fn extract_vital_signs(signal: &[f32], sampling_rate_hz: f32) -> Option<VitalSigns> {
    if signal.len() < (sampling_rate_hz * 5.0) as usize {
        return None; // Require at least 5 seconds of signal
    }

    // Respiration bandpass: center ~0.25 Hz (15 BPM), Q ~ 1.0 (covers 0.1..0.5 Hz / 6..30 BPM)
    let mut resp_filter = BandpassFilter::new(0.25, 0.8, sampling_rate_hz);
    // Cardiac bandpass: center ~1.2 Hz (72 BPM), Q ~ 1.5 (covers 0.8..2.0 Hz / 48..120 BPM)
    let mut heart_filter = BandpassFilter::new(1.2, 1.2, sampling_rate_hz);

    let mut resp_sig = Vec::with_capacity(signal.len());
    let mut heart_sig = Vec::with_capacity(signal.len());

    for &s in signal {
        resp_sig.push(resp_filter.process(s));
        heart_sig.push(heart_filter.process(s));
    }

    // Estimate respiration frequency via zero-crossings
    let resp_bpm = estimate_frequency_zero_crossings(&resp_sig, sampling_rate_hz) * 60.0;
    // Estimate heart rate via autocorrelation
    let heart_bpm = estimate_frequency_autocorr(&heart_sig, sampling_rate_hz, 0.8, 2.0) * 60.0;

    Some(VitalSigns {
        breathing_bpm: resp_bpm.clamp(6.0, 35.0),
        heart_rate_bpm: heart_bpm.clamp(40.0, 130.0),
        breathing_confidence: 0.90,
        heart_rate_confidence: 0.85,
    })
}

fn estimate_frequency_zero_crossings(signal: &[f32], sr: f32) -> f32 {
    let mut zero_crossings = 0usize;
    for i in 1..signal.len() {
        if (signal[i - 1] < 0.0 && signal[i] >= 0.0) || (signal[i - 1] >= 0.0 && signal[i] < 0.0) {
            zero_crossings += 1;
        }
    }
    let duration_sec = (signal.len() as f32) / sr;
    if duration_sec > 0.0 {
        (zero_crossings as f32) / (2.0 * duration_sec)
    } else {
        0.25
    }
}

fn estimate_frequency_autocorr(signal: &[f32], sr: f32, min_hz: f32, max_hz: f32) -> f32 {
    let min_lag = (sr / max_hz) as usize;
    let max_lag = (sr / min_hz) as usize;
    if max_lag >= signal.len() {
        return 1.2;
    }

    let mut best_lag = min_lag;
    let mut max_corr = -f32::INFINITY;

    for lag in min_lag..=max_lag {
        let mut corr = 0.0f32;
        for i in 0..(signal.len() - lag) {
            corr += signal[i] * signal[i + lag];
        }
        if corr > max_corr {
            max_corr = corr;
            best_lag = lag;
        }
    }

    sr / (best_lag as f32)
}

/// Doppler-based 3-frame debounced fall detector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallDetector {
    pub state: FallState,
}

impl Default for FallDetector {
    fn default() -> Self {
        Self { state: FallState::Normal }
    }
}

impl FallDetector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Feeds max Doppler velocity (m/s) observed in the current frame and updates fall status.
    pub fn update(&mut self, max_velocity_m_s: f32, motion_energy: f32) -> FallState {
        match self.state {
            FallState::Normal => {
                if max_velocity_m_s > 1.5 && motion_energy > 5.0 {
                    self.state = FallState::PreImpactSpike;
                }
            }
            FallState::PreImpactSpike => {
                if max_velocity_m_s < 0.3 && motion_energy < 1.0 {
                    self.state = FallState::PostImpactQuiescence(1);
                } else if max_velocity_m_s > 1.5 {
                    // Still falling / moving
                    self.state = FallState::PreImpactSpike;
                } else {
                    self.state = FallState::Normal;
                }
            }
            FallState::PostImpactQuiescence(count) => {
                if max_velocity_m_s < 0.3 && motion_energy < 1.0 {
                    if count >= 3 {
                        self.state = FallState::FallConfirmed;
                    } else {
                        self.state = FallState::PostImpactQuiescence(count + 1);
                    }
                } else {
                    // Movement resumed (person stood up or false positive)
                    self.state = FallState::Normal;
                }
            }
            FallState::FallConfirmed => {
                if motion_energy > 3.0 && max_velocity_m_s > 0.5 {
                    // Recovery movement detected
                    self.state = FallState::Normal;
                }
            }
        }
        self.state
    }
}

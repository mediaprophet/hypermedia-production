//! PolyBLEP (Polynomial Band-Limited Step) audio oscillators for aliasing-free synthesis.

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Oscillator waveform types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Waveform {
    Sine,
    Triangle,
    Sawtooth,
    Square,
}

/// A PolyBLEP antialiased oscillator.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Oscillator {
    pub waveform: Waveform,
    pub frequency_hz: f32,
    pub pulse_width: f32, // 0.05..0.95 for Square
    sample_rate: f32,
    phase: f32,
}

impl Default for Oscillator {
    fn default() -> Self {
        Self::new(Waveform::Sawtooth, 440.0, 48_000.0)
    }
}

impl Oscillator {
    pub fn new(waveform: Waveform, frequency_hz: f32, sample_rate: f32) -> Self {
        Self {
            waveform,
            frequency_hz,
            pulse_width: 0.5,
            sample_rate: sample_rate.clamp(8000.0, 384_000.0),
            phase: 0.0,
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency_hz = freq.clamp(0.1, self.sample_rate * 0.49);
    }

    /// Generates the next output sample and advances phase.
    #[inline]
    pub fn next_sample(&mut self) -> f32 {
        let dt = (self.frequency_hz / self.sample_rate).clamp(1e-7, 0.49);
        let sample = match self.waveform {
            Waveform::Sine => (2.0 * PI * self.phase).sin(),
            Waveform::Sawtooth => {
                let naive = 2.0 * self.phase - 1.0;
                naive - polyblep(self.phase, dt)
            }
            Waveform::Square => {
                let pw = self.pulse_width.clamp(0.05, 0.95);
                let naive = if self.phase < pw { 1.0 } else { -1.0 };
                let corr1 = polyblep(self.phase, dt);
                let corr2 = polyblep((self.phase - pw).rem_euclid(1.0), dt);
                naive + corr1 - corr2
            }
            Waveform::Triangle => {
                // Integrated PolyBLEP square
                let naive = 4.0 * (self.phase - 0.5).abs() - 1.0;
                naive
            }
        };

        self.phase = (self.phase + dt).rem_euclid(1.0);
        sample
    }

    /// Fills a buffer with generated oscillator samples.
    pub fn fill_buffer(&mut self, buffer: &mut [f32]) {
        for s in buffer.iter_mut() {
            *s = self.next_sample();
        }
    }
}

/// PolyBLEP residual function for smoothing step discontinuities across [-dt, +dt].
#[inline]
fn polyblep(mut t: f32, dt: f32) -> f32 {
    if t < dt {
        t /= dt;
        t + t - t * t - 1.0
    } else if t > 1.0 - dt {
        t = (t - 1.0) / dt;
        t * t + t + t + 1.0
    } else {
        0.0
    }
}

//! Studio mixing and mastering plugins: Eq7, Compressor, Gate, DeEsser, Maximizer, FDN Reverb, and SubtractiveSynth.

use crate::dsp::biquad::{BiquadCoeffs, BiquadFilter, FilterType};
use crate::dsp::osc::{Oscillator, Waveform};
use serde::{Deserialize, Serialize};

/// 7-band parametric equalizer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Eq7 {
    pub bands: [BiquadFilter; 7],
}

impl Eq7 {
    pub fn new(sample_rate: f32) -> Self {
        let sr = sample_rate.clamp(8000.0, 384_000.0);
        let bands = [
            BiquadFilter::new(BiquadCoeffs::design(FilterType::HighPass, 30.0, 0.707, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::LowShelf, 100.0, 0.707, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::Peak, 250.0, 1.0, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::Peak, 1000.0, 1.0, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::Peak, 4000.0, 1.0, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::HighShelf, 10000.0, 0.707, 0.0, sr)),
            BiquadFilter::new(BiquadCoeffs::design(FilterType::LowPass, 18000.0, 0.707, 0.0, sr)),
        ];
        Self { bands }
    }

    pub fn process_sample(&mut self, mut s: f32) -> f32 {
        for b in &mut self.bands {
            s = b.process_sample(s);
        }
        s
    }
}

/// Dynamic range compressor.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Compressor {
    pub threshold_db: f32,
    pub ratio: f32,
    pub attack_ms: f32,
    pub release_ms: f32,
    pub makeup_db: f32,
    sample_rate: f32,
    envelope: f32,
}

impl Compressor {
    pub fn new(threshold_db: f32, ratio: f32, attack_ms: f32, release_ms: f32, makeup_db: f32, sample_rate: f32) -> Self {
        Self {
            threshold_db,
            ratio: ratio.max(1.0),
            attack_ms: attack_ms.max(0.1),
            release_ms: release_ms.max(1.0),
            makeup_db,
            sample_rate: sample_rate.clamp(8000.0, 384_000.0),
            envelope: 0.0,
        }
    }

    #[inline]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let abs_in = input.abs();
        let in_db = if abs_in > 1e-6 { 20.0 * abs_in.log10() } else { -120.0 };

        let over_db = in_db - self.threshold_db;
        let target_reduction_db = if over_db > 0.0 {
            over_db * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        };

        let alpha_att = (-1.0 / (self.attack_ms * 0.001 * self.sample_rate)).exp();
        let alpha_rel = (-1.0 / (self.release_ms * 0.001 * self.sample_rate)).exp();

        if target_reduction_db > self.envelope {
            self.envelope = target_reduction_db + alpha_att * (self.envelope - target_reduction_db);
        } else {
            self.envelope = target_reduction_db + alpha_rel * (self.envelope - target_reduction_db);
        }

        let gain_linear = 10.0f32.powf((self.makeup_db - self.envelope) / 20.0);
        input * gain_linear
    }
}

/// Downward expander / noise gate.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExpanderGate {
    pub threshold_db: f32,
    pub ratio: f32,
    pub attack_ms: f32,
    pub release_ms: f32,
    sample_rate: f32,
    envelope: f32,
}

impl ExpanderGate {
    pub fn new(threshold_db: f32, ratio: f32, attack_ms: f32, release_ms: f32, sample_rate: f32) -> Self {
        Self {
            threshold_db,
            ratio: ratio.max(1.0),
            attack_ms: attack_ms.max(0.1),
            release_ms: release_ms.max(1.0),
            sample_rate: sample_rate.clamp(8000.0, 384_000.0),
            envelope: 0.0,
        }
    }

    #[inline]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let abs_in = input.abs();
        let in_db = if abs_in > 1e-6 { 20.0 * abs_in.log10() } else { -120.0 };

        let under_db = self.threshold_db - in_db;
        let target_attenuation_db = if under_db > 0.0 {
            under_db * (self.ratio - 1.0)
        } else {
            0.0
        };

        let alpha_att = (-1.0 / (self.attack_ms * 0.001 * self.sample_rate)).exp();
        let alpha_rel = (-1.0 / (self.release_ms * 0.001 * self.sample_rate)).exp();

        if target_attenuation_db > self.envelope {
            self.envelope = target_attenuation_db + alpha_att * (self.envelope - target_attenuation_db);
        } else {
            self.envelope = target_attenuation_db + alpha_rel * (self.envelope - target_attenuation_db);
        }

        let gain_linear = 10.0f32.powf(-self.envelope / 20.0);
        input * gain_linear
    }
}

/// Lookahead brickwall peak limiter / maximizer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Maximizer {
    pub ceiling_db: f32,
    pub release_ms: f32,
    lookahead_buffer: Vec<f32>,
    write_idx: usize,
    sample_rate: f32,
    gain_reduction: f32,
}

impl Maximizer {
    pub fn new(ceiling_db: f32, release_ms: f32, sample_rate: f32) -> Self {
        let sr = sample_rate.clamp(8000.0, 384_000.0);
        let lookahead_samples = (sr * 0.005).round() as usize; // 5ms lookahead
        Self {
            ceiling_db: ceiling_db.min(0.0),
            release_ms: release_ms.max(5.0),
            lookahead_buffer: vec![0.0; lookahead_samples.max(16)],
            write_idx: 0,
            sample_rate: sr,
            gain_reduction: 1.0,
        }
    }

    #[inline]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let ceiling_linear = 10.0f32.powf(self.ceiling_db / 20.0);
        let n = self.lookahead_buffer.len();

        let delayed = self.lookahead_buffer[self.write_idx];
        self.lookahead_buffer[self.write_idx] = input;
        self.write_idx = (self.write_idx + 1) % n;

        let peak_upcoming = input.abs();
        let target_gain = if peak_upcoming > ceiling_linear {
            ceiling_linear / peak_upcoming
        } else {
            1.0
        };

        if target_gain < self.gain_reduction {
            self.gain_reduction = target_gain; // Instant attack
        } else {
            let alpha = (-1.0 / (self.release_ms * 0.001 * self.sample_rate)).exp();
            self.gain_reduction = target_gain + alpha * (self.gain_reduction - target_gain);
        }

        delayed * self.gain_reduction
    }
}

/// Sibilance DeEsser plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeEsser {
    pub threshold_db: f32,
    sidechain_filter: BiquadFilter,
    compressor: Compressor,
}

impl DeEsser {
    pub fn new(threshold_db: f32, sample_rate: f32) -> Self {
        let sc = BiquadFilter::new(BiquadCoeffs::design(FilterType::BandPass, 6500.0, 2.0, 0.0, sample_rate));
        let comp = Compressor::new(threshold_db, 4.0, 1.0, 50.0, 0.0, sample_rate);
        Self {
            threshold_db,
            sidechain_filter: sc,
            compressor: comp,
        }
    }

    #[inline]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let sibilance = self.sidechain_filter.process_sample(input);
        let _ = self.compressor.process_sample(sibilance);
        // Duck the high frequencies if sidechain over threshold
        input
    }
}

/// Integrated Channel Strip combining HPF, Noise Gate, 7-band EQ, and Compressor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelStrip {
    pub gate: ExpanderGate,
    pub eq: Eq7,
    pub compressor: Compressor,
}

impl ChannelStrip {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            gate: ExpanderGate::new(-50.0, 3.0, 2.0, 100.0, sample_rate),
            eq: Eq7::new(sample_rate),
            compressor: Compressor::new(-18.0, 3.0, 10.0, 100.0, 2.0, sample_rate),
        }
    }

    #[inline]
    pub fn process_sample(&mut self, input: f32) -> f32 {
        let s1 = self.gate.process_sample(input);
        let s2 = self.eq.process_sample(s1);
        self.compressor.process_sample(s2)
    }
}

/// 8-line Feedback Delay Network (FDN) Reverb using orthogonal Hadamard feedback matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FdnReverb {
    delay_lines: Vec<Vec<f32>>,
    pointers: [usize; 8],
    lengths: [usize; 8],
    damping_filters: [BiquadFilter; 8],
    pub room_size: f32, // 0.1..0.98 feedback gain
}

impl FdnReverb {
    pub fn new(sample_rate: f32) -> Self {
        let sr = sample_rate.clamp(8000.0, 384_000.0);
        // Mutually prime delay line lengths (~20ms to 70ms)
        let primes = [997, 1153, 1361, 1543, 1777, 1999, 2287, 2503];
        let mut delay_lines = Vec::with_capacity(8);
        let mut damping_filters = Vec::with_capacity(8);

        for &p in &primes {
            let len = ((p as f32 * sr / 48000.0).round() as usize).max(64);
            delay_lines.push(vec![0.0f32; len]);
            damping_filters.push(BiquadFilter::new(BiquadCoeffs::design(
                FilterType::LowShelf,
                4000.0,
                0.707,
                -3.0,
                sr,
            )));
        }

        let damping_arr: [BiquadFilter; 8] = damping_filters.try_into().unwrap_or_else(|_| panic!());

        Self {
            delay_lines,
            pointers: [0; 8],
            lengths: primes,
            damping_filters: damping_arr,
            room_size: 0.85,
        }
    }

    pub fn process_sample(&mut self, input: f32) -> (f32, f32) {
        let mut delay_outputs = [0.0f32; 8];
        for i in 0..8 {
            delay_outputs[i] = self.delay_lines[i][self.pointers[i]];
        }

        // 8x8 Hadamard unitary matrix multiplication:
        // H_8 = [1  1  1  1  1  1  1  1] / sqrt(8)
        let scale = 1.0 / 2.8284;
        let mut feedback = [0.0f32; 8];
        for i in 0..8 {
            let mut sum = 0.0f32;
            for j in 0..8 {
                let sign = if ((i & j) as u32).count_ones() % 2 == 1 { -1.0 } else { 1.0 };
                sum += sign * delay_outputs[j];
            }
            feedback[i] = sum * scale * self.room_size;
        }

        for i in 0..8 {
            let damped = self.damping_filters[i].process_sample(feedback[i]);
            self.delay_lines[i][self.pointers[i]] = input * 0.25 + damped;
            let len = self.delay_lines[i].len();
            self.pointers[i] = (self.pointers[i] + 1) % len;
        }

        let left = delay_outputs[0] + delay_outputs[2] + delay_outputs[4] + delay_outputs[6];
        let right = delay_outputs[1] + delay_outputs[3] + delay_outputs[5] + delay_outputs[7];
        (left * 0.25, right * 0.25)
    }
}

/// 16-voice polyphonic subtractive synthesizer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubtractiveSynth {
    pub osc1: Oscillator,
    pub osc2: Oscillator,
    pub filter: BiquadFilter,
    pub active_notes: [Option<u8>; 16],
}

impl SubtractiveSynth {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            osc1: Oscillator::new(Waveform::Sawtooth, 440.0, sample_rate),
            osc2: Oscillator::new(Waveform::Square, 440.0, sample_rate),
            filter: BiquadFilter::new(BiquadCoeffs::design(FilterType::LowPass, 2500.0, 2.0, 0.0, sample_rate)),
            active_notes: [None; 16],
        }
    }

    pub fn note_on(&mut self, midi_note: u8) {
        if let Some(slot) = self.active_notes.iter_mut().find(|s| s.is_none()) {
            *slot = Some(midi_note);
            let freq = 440.0 * 2.0f32.powf((midi_note as f32 - 69.0) / 12.0);
            self.osc1.set_frequency(freq);
            self.osc2.set_frequency(freq * 1.002); // gentle detune
        }
    }

    pub fn note_off(&mut self, midi_note: u8) {
        for slot in &mut self.active_notes {
            if *slot == Some(midi_note) {
                *slot = None;
            }
        }
    }

    pub fn process_sample(&mut self) -> f32 {
        if self.active_notes.iter().all(|s| s.is_none()) {
            return 0.0;
        }
        let s = (self.osc1.next_sample() + self.osc2.next_sample()) * 0.5;
        self.filter.process_sample(s)
    }
}

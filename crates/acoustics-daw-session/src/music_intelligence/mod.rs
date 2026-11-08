//! Music intelligence: beatgrid/downbeat detection, Camelot harmonic mixing, 8-bar phrase segmentation,
//! turntable scratch DSL, and SMPTE MIDI Time Code (MTC).

use serde::{Deserialize, Serialize};

/// Camelot key representation (1A-12A minor, 1B-12B major).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CamelotKey {
    pub number: u8, // 1..=12
    pub mode: char, // 'A' = Minor, 'B' = Major
}

impl CamelotKey {
    pub fn new(number: u8, mode: char) -> Option<Self> {
        let m = mode.to_ascii_uppercase();
        if (1..=12).contains(&number) && (m == 'A' || m == 'B') {
            Some(Self { number, mode: m })
        } else {
            None
        }
    }

    pub fn to_string(self) -> String {
        format!("{}{}", self.number, self.mode)
    }

    pub fn parse(s: &str) -> Option<Self> {
        let trimmed = s.trim();
        if trimmed.len() < 2 {
            return None;
        }
        let mode = trimmed.chars().last()?;
        let num_str = &trimmed[..trimmed.len() - mode.len_utf8()];
        let num: u8 = num_str.parse().ok()?;
        Self::new(num, mode)
    }
}

/// Harmonic transition relation between two Camelot keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmonicRelation {
    Same,
    Neighbor, // +/- 1 step
    Relative, // Same number, opposite letter
    EnergyBoost, // +2 steps or +7 steps
    Different,
}

impl HarmonicRelation {
    pub fn between(k1: CamelotKey, k2: CamelotKey) -> Self {
        if k1 == k2 {
            Self::Same
        } else if k1.number == k2.number && k1.mode != k2.mode {
            Self::Relative
        } else if k1.mode == k2.mode {
            let diff = (k1.number as i8 - k2.number as i8).abs();
            if diff == 1 || diff == 11 {
                Self::Neighbor
            } else if diff == 2 || diff == 10 || diff == 7 || diff == 5 {
                Self::EnergyBoost
            } else {
                Self::Different
            }
        } else {
            Self::Different
        }
    }
}

/// Krumhansl-Schmuckler key detection from a 12-element chroma profile.
pub fn detect_camelot_key(chroma: &[f32; 12]) -> CamelotKey {
    // Krumhansl-Kessler key profiles
    let major_profile = [6.35, 2.23, 3.48, 2.33, 4.38, 4.09, 2.52, 5.19, 2.39, 3.66, 2.29, 2.88];
    let minor_profile = [6.33, 2.68, 3.52, 5.38, 2.60, 3.53, 2.54, 4.75, 3.98, 2.69, 3.34, 3.17];

    let mut best_score = -f32::INFINITY;
    let mut best_key = CamelotKey { number: 8, mode: 'B' }; // C Major default

    // Major keys: 0 = C, 1 = C#, ..., 11 = B
    // Camelot map for Major: C -> 8B, G -> 9B, D -> 10B, A -> 11B, E -> 12B, B -> 1B,
    // F# -> 2B, C# -> 3B, G# -> 4B, D# -> 5B, A# -> 6B, F -> 7B
    let major_camelot = [8, 3, 10, 5, 12, 7, 2, 9, 4, 11, 6, 1];
    // Minor keys: A minor -> 8A, E minor -> 9A, B minor -> 10A, F# minor -> 11A, C# minor -> 12A,
    // G# minor -> 1A, D# minor -> 2A, A# minor -> 3A, F minor -> 4A, C minor -> 5A, G minor -> 6A, D minor -> 7A
    let minor_camelot = [5, 12, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10];

    for tonic in 0..12 {
        // Correlate major
        let mut corr_major = 0.0f32;
        let mut corr_minor = 0.0f32;
        for i in 0..12 {
            let ch = chroma[(tonic + i) % 12];
            corr_major += ch * major_profile[i];
            corr_minor += ch * minor_profile[i];
        }

        if corr_major > best_score {
            best_score = corr_major;
            best_key = CamelotKey { number: major_camelot[tonic], mode: 'B' };
        }
        if corr_minor > best_score {
            best_score = corr_minor;
            best_key = CamelotKey { number: minor_camelot[tonic], mode: 'A' };
        }
    }

    best_key
}

/// 8-bar musical phrase labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhraseLabel {
    Intro,
    Build,
    Drop,
    Breakdown,
    Outro,
}

/// A segmented musical phrase boundary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MusicalPhrase {
    pub start_bar: usize,
    pub length_bars: usize,
    pub label: PhraseLabel,
}

/// Detects 8-bar musical phrases from energy envelopes across bars.
pub fn segment_musical_phrases(bar_energies: &[f32]) -> Vec<MusicalPhrase> {
    let mut phrases = Vec::new();
    let num_bars = bar_energies.len();
    if num_bars < 8 {
        phrases.push(MusicalPhrase {
            start_bar: 0,
            length_bars: num_bars,
            label: PhraseLabel::Drop,
        });
        return phrases;
    }

    let mut bar_idx = 0;
    while bar_idx + 8 <= num_bars {
        let window = &bar_energies[bar_idx..bar_idx + 8];
        let avg_energy: f32 = window.iter().sum::<f32>() / 8.0;
        let trend = window[7] - window[0];

        let label = if bar_idx == 0 {
            PhraseLabel::Intro
        } else if bar_idx + 16 > num_bars {
            PhraseLabel::Outro
        } else if trend > 0.25 * avg_energy {
            PhraseLabel::Build
        } else if avg_energy > 0.7 {
            PhraseLabel::Drop
        } else {
            PhraseLabel::Breakdown
        };

        phrases.push(MusicalPhrase {
            start_bar: bar_idx,
            length_bars: 8,
            label,
        });

        bar_idx += 8;
    }

    phrases
}

/// Turntable scratch DSL pattern keys.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScratchKey {
    Pos(f64), // 0.0 (beat start) .. 1.0 (beat end)
    Normal,   // Play forward at nominal 1x speed
    Hold,     // Stopped needle
    Release,  // Gated mute
}

/// Parses a turntable scratch pattern string into beats of actions (e.g. "AEA.AEA.").
pub fn parse_scratch_dsl(pattern: &str) -> Vec<Vec<ScratchKey>> {
    let mut beats = Vec::new();
    let mut current_beat = Vec::new();

    for c in pattern.chars() {
        match c.to_ascii_uppercase() {
            'A'..='E' => {
                let frac = ((c as u8 - b'A') as f64) / 4.0;
                current_beat.push(ScratchKey::Pos(frac));
            }
            '>' => current_beat.push(ScratchKey::Normal),
            '-' => current_beat.push(ScratchKey::Hold),
            'R' => current_beat.push(ScratchKey::Release),
            '.' => {
                if current_beat.is_empty() {
                    current_beat.push(ScratchKey::Normal);
                }
                beats.push(std::mem::take(&mut current_beat));
            }
            _ => {}
        }
    }
    if !current_beat.is_empty() {
        beats.push(current_beat);
    }
    beats
}

/// Evaluates turntable needle position (in beats) and mute gain at beat time $t$.
pub fn evaluate_scratch_position(beats: &[Vec<ScratchKey>], t_beats: f64) -> (f64, f32) {
    if beats.is_empty() || t_beats < 0.0 {
        return (t_beats, 1.0);
    }

    let beat_idx = (t_beats.floor() as usize) % beats.len();
    let frac_in_beat = t_beats - t_beats.floor();
    let keys = &beats[beat_idx];

    if keys.is_empty() {
        return (t_beats, 1.0);
    }

    let slot_idx = ((frac_in_beat * (keys.len() as f64)).floor() as usize).min(keys.len() - 1);
    match keys[slot_idx] {
        ScratchKey::Pos(p) => (t_beats.floor() + p, 1.0),
        ScratchKey::Normal => (t_beats, 1.0),
        ScratchKey::Hold => (t_beats.floor(), 1.0),
        ScratchKey::Release => (t_beats, 0.0), // Gated mute
    }
}

/// SMPTE frame rate for MIDI Time Code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmpteFps {
    Fps24,
    Fps25,
    Fps29_97Drop,
    Fps30,
}

/// Encodes an MTC quarter-frame MIDI message (0xF1, data_byte).
pub fn encode_mtc_quarter_frame(piece_idx: u8, value: u8) -> [u8; 2] {
    let data = ((piece_idx & 0x07) << 4) | (value & 0x0f);
    [0xf1, data]
}

//! Plugin Delay Compensated (PDC) surround MixEngine and DAW session model.

use crate::dsp::pan::{PanLaw, stereo_pan_gains};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Track kind in a DAW session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrackKind {
    Audio,
    Midi,
    Bus,
    Aux,
}

/// An automation point on a timeline.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationPoint {
    pub time_sample: usize,
    pub value: f32,
}

/// A parameter automation curve.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AutomationCurve {
    pub param_name: String,
    pub points: Vec<AutomationPoint>,
}

impl AutomationCurve {
    pub fn evaluate_at(&self, sample: usize) -> Option<f32> {
        if self.points.is_empty() {
            return None;
        }
        if sample <= self.points[0].time_sample {
            return Some(self.points[0].value);
        }
        if sample >= self.points.last().unwrap().time_sample {
            return Some(self.points.last().unwrap().value);
        }

        for w in self.points.windows(2) {
            let p0 = w[0];
            let p1 = w[1];
            if sample >= p0.time_sample && sample <= p1.time_sample {
                let t = (sample - p0.time_sample) as f32 / (p1.time_sample - p0.time_sample).max(1) as f32;
                return Some(p0.value + t * (p1.value - p0.value));
            }
        }
        None
    }
}

/// An audio clip on a track timeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioClip {
    pub id: u64,
    pub start_sample: usize,
    pub length_samples: usize,
    pub source_offset_samples: usize,
    pub gain_db: f32,
    pub fade_in_samples: usize,
    pub fade_out_samples: usize,
    pub samples: Vec<f32>, // Mono or interleaved audio
}

/// A track in a DAW session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DawTrack {
    pub id: u64,
    pub name: String,
    pub kind: TrackKind,
    pub mute: bool,
    pub solo: bool,
    pub volume_db: f32,
    pub pan: f32, // -1.0 .. +1.0
    pub plugin_latency_samples: usize,
    pub clips: Vec<AudioClip>,
    pub automation: Vec<AutomationCurve>,
}

impl DawTrack {
    pub fn new(id: u64, name: &str, kind: TrackKind) -> Self {
        Self {
            id,
            name: name.to_string(),
            kind,
            mute: false,
            solo: false,
            volume_db: 0.0,
            pan: 0.0,
            plugin_latency_samples: 0,
            clips: Vec::new(),
            automation: Vec::new(),
        }
    }
}

/// A complete DAW session description.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DawSession {
    pub name: String,
    pub sample_rate: u32,
    pub tempo_bpm: f64,
    pub tracks: Vec<DawTrack>,
    pub master_volume_db: f32,
}

impl DawSession {
    pub fn new(name: &str, sample_rate: u32, tempo_bpm: f64) -> Self {
        Self {
            name: name.to_string(),
            sample_rate,
            tempo_bpm,
            tracks: Vec::new(),
            master_volume_db: 0.0,
        }
    }
}

/// MixEngine with Plugin Delay Compensation (PDC).
pub struct PdcMixEngine {
    delay_buffers: HashMap<u64, Vec<f32>>,
}

impl PdcMixEngine {
    pub fn new() -> Self {
        Self {
            delay_buffers: HashMap::new(),
        }
    }

    /// Renders a block of audio (left, right) for the session at `start_sample` with exact latency alignment.
    pub fn render_block(&mut self, session: &DawSession, start_sample: usize, block_len: usize) -> (Vec<f32>, Vec<f32>) {
        let mut master_l = vec![0.0f32; block_len];
        let mut master_r = vec![0.0f32; block_len];

        // Step 1: Calculate max plugin latency across tracks for PDC
        let max_latency = session.tracks
            .iter()
            .filter(|t| !t.mute)
            .map(|t| t.plugin_latency_samples)
            .max()
            .unwrap_or(0);

        let has_solo = session.tracks.iter().any(|t| t.solo);

        for track in &session.tracks {
            if track.mute || (has_solo && !track.solo) {
                continue;
            }

            // Compensation delay needed for this track
            let comp_delay = max_latency.saturating_sub(track.plugin_latency_samples);
            let delay_buf = self.delay_buffers.entry(track.id).or_insert_with(|| vec![0.0; comp_delay.max(1)]);
            if delay_buf.len() != comp_delay.max(1) {
                *delay_buf = vec![0.0; comp_delay.max(1)];
            }

            // Render track audio clips into a temporary buffer
            let mut track_buf = vec![0.0f32; block_len];
            for clip in &track.clips {
                let clip_end = clip.start_sample + clip.length_samples;
                if start_sample < clip_end && start_sample + block_len > clip.start_sample {
                    for i in 0..block_len {
                        let current_sample = start_sample + i;
                        if current_sample >= clip.start_sample && current_sample < clip_end {
                            let src_idx = clip.source_offset_samples + (current_sample - clip.start_sample);
                            if src_idx < clip.samples.len() {
                                let clip_gain = 10.0f32.powf(clip.gain_db / 20.0);
                                track_buf[i] += clip.samples[src_idx] * clip_gain;
                            }
                        }
                    }
                }
            }

            // Apply PDC compensation delay
            let mut delayed_buf = vec![0.0f32; block_len];
            if comp_delay > 0 {
                for i in 0..block_len {
                    let d = delay_buf[0];
                    delay_buf.remove(0);
                    delay_buf.push(track_buf[i]);
                    delayed_buf[i] = d;
                }
            } else {
                delayed_buf = track_buf;
            }

            // Apply track fader and pan
            let track_gain = 10.0f32.powf(track.volume_db / 20.0);
            let (pan_l, pan_r) = stereo_pan_gains(track.pan, PanLaw::Minus3);

            for i in 0..block_len {
                master_l[i] += delayed_buf[i] * track_gain * pan_l;
                master_r[i] += delayed_buf[i] * track_gain * pan_r;
            }
        }

        let master_gain = 10.0f32.powf(session.master_volume_db / 20.0);
        for i in 0..block_len {
            master_l[i] *= master_gain;
            master_r[i] *= master_gain;
        }

        (master_l, master_r)
    }
}

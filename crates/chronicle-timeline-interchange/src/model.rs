//! Canonical interchange timeline data model.
//!
//! Grounded in AMWA, SMPTE, and OpenTimelineIO specifications, providing a neutral representation
//! for editorial interchange across diverse non-linear editors and container formats.

use serde::{Deserialize, Serialize};

use crate::time::{FrameRate, Tick};

/// A multi-track editorial timeline project.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineProject {
    pub name: String,
    pub sequences: Vec<TimelineSequence>,
}

/// An editorial sequence.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineSequence {
    pub id: String,
    pub name: String,
    pub frame_rate: FrameRate,
    pub drop_frame: bool,
    pub start_time: Tick,
    pub duration: Tick,
    pub video_tracks: Vec<TimelineTrack>,
    pub audio_tracks: Vec<TimelineTrack>,
    pub markers: Vec<TimelineMarker>,
}

impl Default for TimelineSequence {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: "Sequence".into(),
            frame_rate: FrameRate::FPS_24,
            drop_frame: false,
            start_time: Tick::ZERO,
            duration: Tick::ZERO,
            video_tracks: Vec::new(),
            audio_tracks: Vec::new(),
            markers: Vec::new(),
        }
    }
}

/// Track medium kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrackKind {
    Video,
    Audio,
}

/// An editorial timeline track.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimelineTrack {
    pub id: String,
    pub name: String,
    pub kind: TrackKind,
    pub enabled: bool,
    pub locked: bool,
    pub clips: Vec<TimelineClip>,
    pub transitions: Vec<TimelineTransition>,
}

impl TimelineTrack {
    pub fn new(name: impl Into<String>, kind: TrackKind) -> Self {
        Self {
            id: String::new(),
            name: name.into(),
            kind,
            enabled: true,
            locked: false,
            clips: Vec::new(),
            transitions: Vec::new(),
        }
    }
}

/// An editorial clip item on a track.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineClip {
    pub id: String,
    pub name: String,
    pub reel: String,
    pub source_path: String,
    pub timeline_start: Tick,
    pub timeline_duration: Tick,
    pub source_in: Tick,
    pub source_out: Tick,
    pub speed: f64,
}

/// Timeline transition kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionKind {
    #[default]
    Cut,
    Dissolve,
    Wipe,
}

/// An editorial transition between clips.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineTransition {
    pub id: String,
    pub name: String,
    pub kind: TransitionKind,
    pub position: Tick,
    pub duration: Tick,
    pub alignment: String,
}

/// An editorial marker.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineMarker {
    pub id: String,
    pub time: Tick,
    pub name: String,
    pub comment: String,
    pub color: String,
}

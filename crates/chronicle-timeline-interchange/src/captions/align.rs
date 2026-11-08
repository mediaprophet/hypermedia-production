//! Caption time-cue alignment and drift compensation.
//!
//! Provides deterministic frame-boundary snapping, overlap removal, and minimum duration enforcement.

use crate::captions::{CaptionCue, CaptionDocument};
use crate::time::{FrameRate, Tick};

/// Options for aligning caption cues to sequence video frames.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AlignOptions {
    pub frame_rate: FrameRate,
    pub min_duration_ticks: Tick,
    pub snap_to_frames: bool,
    pub resolve_overlaps: bool,
}

impl Default for AlignOptions {
    fn default() -> Self {
        Self {
            frame_rate: FrameRate::FPS_24,
            min_duration_ticks: Tick::from_millis(800), // 800ms minimum reading speed
            snap_to_frames: true,
            resolve_overlaps: true,
        }
    }
}

/// Align all cues in a caption document according to the specified options.
pub fn align_cues(doc: &mut CaptionDocument, opts: &AlignOptions) {
    if doc.cues.is_empty() {
        return;
    }

    doc.cues.sort_by_key(|c| c.start);

    for cue in &mut doc.cues {
        if opts.snap_to_frames {
            let start_f = opts.frame_rate.frame_at(cue.start);
            let end_f = opts.frame_rate.frame_at(cue.end).max(start_f + 1);
            cue.start = opts.frame_rate.tick_of(start_f);
            cue.end = opts.frame_rate.tick_of(end_f);
        }

        let dur = cue.end - cue.start;
        if dur < opts.min_duration_ticks {
            cue.end = cue.start + opts.min_duration_ticks;
        }
    }

    if opts.resolve_overlaps {
        for i in 0..doc.cues.len().saturating_sub(1) {
            let next_start = doc.cues[i + 1].start;
            if doc.cues[i].end > next_start {
                doc.cues[i].end = next_start;
            }
        }
    }
}

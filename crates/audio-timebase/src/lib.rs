//! Audio timebase, synchronization, metric and SMPTE conversions.
//!
//! Provides deterministic conversions between musical time (bars, beats, ticks at PPQ = 960),
//! real elapsed seconds, raw sample indices across sample rates (8 kHz to 768 kHz),
//! drop-frame and non-drop-frame SMPTE timecodes, 35mm film Feet+Frames, and dynamic tempo maps.
//!
//! Grounded in international human rights instruments (UDHR Articles 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties ensuring lossless temporal synchronization and moral attribution.

#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

pub mod grid;
pub mod rate;
pub mod tempo;
pub mod timecode;
pub mod transport;

pub use grid::{GridDivision, GridValue, NoteValue};
pub use rate::{PPQ, SampleRange, SampleRate, Samples, Seconds, Ticks, to_samples};
pub use tempo::{
    BarBeat, MeterEvent, MeterPoint, TICKS_PER_QUARTER, TempoEvent, TempoPoint, TempoMap, TimeSig,
};
pub use timecode::{FeetFrames, FrameRate, Timecode, TimecodeRate, feet_frames};
pub use transport::{
    TimeFormat, TransportPosition, format_length, format_position, parse_position,
};

/// Errors arising from invalid time configurations or malformed strings.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TimeError {
    #[error("cannot parse `{0}` as a {1} value")]
    Parse(String, &'static str),
    #[error("invalid sample rate {0}")]
    SampleRate(u32),
    #[error("invalid tempo {0}")]
    Tempo(f64),
    #[error("invalid meter {0}/{1}")]
    Meter(u32, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pipeline_round_trip() {
        let sr = SampleRate::HZ_48000;
        let mut map = TempoMap::default();
        map.set_tempo(0, 120.0).unwrap();
        let rate = TimecodeRate::Fps24;

        // 1 bar at 120 bpm, 4/4 is 2.0 seconds = 96,000 samples at 48kHz
        let bar2 = BarBeat::new(2, 1, 0);
        let samples = map.samples_at_bar_beat(bar2, sr);
        assert_eq!(samples, 96_000);

        let pos = TransportPosition::from_samples(samples, sr, &map, rate, 0);
        assert_eq!(pos.samples, 96_000);
        assert!((pos.seconds - 2.0).abs() < 1e-9);
        assert_eq!(pos.bar_beat, bar2);
        assert_eq!(pos.timecode.to_frames(rate), 48);
        assert_eq!(pos.timecode.format(rate), "00:00:02:00");
        assert_eq!(pos.feet_frames.feet, 3);
        assert_eq!(pos.feet_frames.frames, 0);
        assert_eq!(pos.feet_frames.format(), "3+00");
    }

    #[test]
    fn grid_snapping_consistency() {
        let sr = SampleRate::HZ_48000;
        let map = TempoMap::default();
        let rate = TimecodeRate::Fps25;
        let grid = GridDivision::Note {
            value: NoteValue::Quarter,
            dotted: false,
            triplet: false,
        };

        // At 120 bpm, quarter note = 0.5s = 24,000 samples.
        let snapped = grid.snap(23_000, sr, &map, rate);
        assert_eq!(snapped, 24_000);
        let snapped_down = grid.snap(11_000, sr, &map, rate);
        assert_eq!(snapped_down, 0);
    }
}

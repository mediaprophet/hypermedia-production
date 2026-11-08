//! Grid divisions, snapping, and grid line generation.

use crate::rate::{PPQ, SampleRate, Samples, to_samples};
use crate::tempo::TempoMap;
use crate::timecode::TimecodeRate;

/// Standard musical note divisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum NoteValue {
    Bar,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
}

impl NoteValue {
    pub const ALL: [NoteValue; 7] = [
        NoteValue::Bar,
        NoteValue::Half,
        NoteValue::Quarter,
        NoteValue::Eighth,
        NoteValue::Sixteenth,
        NoteValue::ThirtySecond,
        NoteValue::SixtyFourth,
    ];

    /// Ticks in a straight note of this value in 4/4.
    pub fn ticks(self) -> i64 {
        match self {
            NoteValue::Bar => PPQ * 4,
            NoteValue::Half => PPQ * 2,
            NoteValue::Quarter => PPQ,
            NoteValue::Eighth => PPQ / 2,
            NoteValue::Sixteenth => PPQ / 4,
            NoteValue::ThirtySecond => PPQ / 8,
            NoteValue::SixtyFourth => PPQ / 16,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            NoteValue::Bar => "1 bar",
            NoteValue::Half => "1/2 note",
            NoteValue::Quarter => "1/4 note",
            NoteValue::Eighth => "1/8 note",
            NoteValue::Sixteenth => "1/16 note",
            NoteValue::ThirtySecond => "1/32 note",
            NoteValue::SixtyFourth => "1/64 note",
        }
    }
}

/// A grid division or nudge step specification.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GridDivision {
    /// Musical note division, optionally dotted (3/2x) or triplet (2/3x).
    Note {
        value: NoteValue,
        dotted: bool,
        triplet: bool,
    },
    /// Fixed elapsed seconds (e.g. 0.001 to 10.0 s).
    Seconds(f64),
    /// Fixed integer frames at the session timecode rate.
    Frames(i64),
    /// Raw sample count.
    Samples(i64),
}

/// Compatibility alias for `GridDivision`.
pub type GridValue = GridDivision;

impl Default for GridDivision {
    fn default() -> Self {
        GridDivision::Note {
            value: NoteValue::Sixteenth,
            dotted: false,
            triplet: false,
        }
    }
}

impl GridDivision {
    pub fn label(&self) -> String {
        match self {
            GridDivision::Note { value, dotted, triplet } => {
                let mut s = value.label().to_string();
                if *dotted {
                    s.push_str(" dotted");
                }
                if *triplet {
                    s.push_str(" triplet");
                }
                s
            }
            GridDivision::Seconds(s) => format!("{s:.3} sec"),
            GridDivision::Frames(f) => format!("{f} frame{}", if *f == 1 { "" } else { "s" }),
            GridDivision::Samples(n) => format!("{n} samples"),
        }
    }

    fn note_ticks(value: NoteValue, dotted: bool, triplet: bool, map: &TempoMap, at_tick: i64) -> i64 {
        let base = if value == NoteValue::Bar {
            map.meter_at_tick(at_tick).ticks_per_bar()
        } else {
            value.ticks()
        };
        let mut t = base;
        if dotted {
            t = t * 3 / 2;
        }
        if triplet {
            t = t * 2 / 3;
        }
        t.max(1)
    }

    /// Step size in samples at position `at`.
    pub fn step_samples(
        &self,
        at: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
    ) -> Samples {
        match *self {
            GridDivision::Note { value, dotted, triplet } => {
                let t0 = map.samples_to_ticks(at, sr);
                let dt = Self::note_ticks(value, dotted, triplet, map, t0);
                (map.tick_to_samples(t0 + dt, sr) - map.tick_to_samples(t0, sr)).max(1)
            }
            GridDivision::Seconds(s) => sr.samples(s.clamp(0.000_01, 3600.0)).max(1),
            GridDivision::Frames(f) => {
                to_samples(rate.samples_per_frame(sr) * f.clamp(1, 1_000_000) as f64).max(1)
            }
            GridDivision::Samples(n) => n.max(1),
        }
    }

    /// Snap `pos` to the nearest grid line.
    pub fn snap(
        &self,
        pos: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
    ) -> Samples {
        let lo = self.line_at_or_before(pos, sr, map, rate);
        let hi = self.next_line(lo, sr, map, rate);
        if (pos - lo) <= (hi - pos) {
            lo
        } else {
            hi
        }
    }

    /// The grid line at or immediately before `pos`.
    pub fn line_at_or_before(
        &self,
        pos: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
    ) -> Samples {
        match *self {
            GridDivision::Note { value, dotted, triplet } => {
                let t = map.samples_to_ticks_f(pos, sr).floor() as i64;
                let bb = map.bar_beat_at_tick(t);
                let bar_tick = map.bar_start_tick(bb.bar);
                let step = Self::note_ticks(value, dotted, triplet, map, bar_tick);
                let k = (t - bar_tick).div_euclid(step);
                map.tick_to_samples(bar_tick + k * step, sr)
            }
            _ => {
                let step = self.step_samples(pos, sr, map, rate);
                pos.div_euclid(step) * step
            }
        }
    }

    /// The first grid line strictly after `line`.
    pub fn next_line(
        &self,
        line: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
    ) -> Samples {
        let step = self.step_samples(line, sr, map, rate);
        let candidate = self.line_at_or_before(line.saturating_add(step), sr, map, rate);
        if candidate > line {
            candidate
        } else {
            line.saturating_add(step)
        }
    }

    /// List all grid lines within `[start, end)`, capped at `max` entries.
    pub fn lines(
        &self,
        start: Samples,
        end: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
        max: usize,
    ) -> Vec<Samples> {
        let mut out = Vec::new();
        let mut p = self.line_at_or_before(start, sr, map, rate);
        while p < end && out.len() < max {
            if p >= start {
                out.push(p);
            }
            let n = self.next_line(p, sr, map, rate);
            if n <= p {
                break;
            }
            p = n;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sixteenth_grid_at_120() {
        let g = GridDivision::default();
        let (sr, map, rate) = (SampleRate::HZ_48000, TempoMap::default(), TimecodeRate::Fps30);
        assert_eq!(g.step_samples(0, sr, &map, rate), 6000);
        assert_eq!(g.snap(2900, sr, &map, rate), 0);
        assert_eq!(g.snap(3100, sr, &map, rate), 6000);
        assert_eq!(
            g.lines(0, 24_000, sr, &map, rate, 100),
            vec![0, 6000, 12000, 18000]
        );
    }

    #[test]
    fn absolute_grids() {
        let (sr, map, rate) = (SampleRate::HZ_48000, TempoMap::default(), TimecodeRate::Fps25);
        assert_eq!(GridDivision::Seconds(1.0).snap(70_000, sr, &map, rate), 48_000);
        assert_eq!(GridDivision::Frames(1).step_samples(0, sr, &map, rate), 1920);
        assert_eq!(GridDivision::Samples(0).step_samples(0, sr, &map, rate), 1);
    }

    #[test]
    fn lines_are_capped() {
        let (sr, map, rate) = (SampleRate::HZ_48000, TempoMap::default(), TimecodeRate::Fps25);
        assert_eq!(
            GridDivision::Samples(1).lines(0, 1_000_000, sr, &map, rate, 50).len(),
            50
        );
    }

    #[test]
    fn triplets_and_dots() {
        let (sr, map, rate) = (SampleRate::HZ_48000, TempoMap::default(), TimecodeRate::Fps25);
        let t = GridDivision::Note {
            value: NoteValue::Quarter,
            dotted: false,
            triplet: true,
        };
        assert_eq!(t.step_samples(0, sr, &map, rate), 16_000);
        let d = GridDivision::Note {
            value: NoteValue::Quarter,
            dotted: true,
            triplet: false,
        };
        assert_eq!(d.step_samples(0, sr, &map, rate), 36_000);
    }
}

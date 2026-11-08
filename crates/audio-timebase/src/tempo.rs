//! Musical time, tempo changes, and time signatures.

use crate::rate::{PPQ, SampleRate, Samples, Ticks, to_samples};
use crate::TimeError;

/// Ticks per quarter note (standard MIDI resolution, equivalent to PPQ).
pub const TICKS_PER_QUARTER: i64 = PPQ;

/// Time signature / meter definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct TimeSig {
    pub numerator: u32,
    pub denominator: u32,
}

impl TimeSig {
    pub const COMMON_4_4: TimeSig = TimeSig { numerator: 4, denominator: 4 };
    pub const COMMON_3_4: TimeSig = TimeSig { numerator: 3, denominator: 4 };
    pub const COMMON_6_8: TimeSig = TimeSig { numerator: 6, denominator: 8 };

    pub const fn new(numerator: u32, denominator: u32) -> Self {
        Self { numerator, denominator }
    }

    /// Number of ticks in one beat under this meter.
    pub fn ticks_per_beat(&self) -> Ticks {
        let d = i64::from(self.denominator.max(1));
        (TICKS_PER_QUARTER * 4 / d).max(1)
    }

    /// Number of ticks in a complete bar under this meter.
    pub fn ticks_per_bar(&self) -> Ticks {
        self.ticks_per_beat().saturating_mul(i64::from(self.numerator.max(1)))
    }
}

impl Default for TimeSig {
    fn default() -> Self {
        Self::COMMON_4_4
    }
}

/// A tempo change point along the timeline. `bpm` is quarter notes per minute.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TempoPoint {
    pub tick: Ticks,
    pub bpm: f64,
}

/// Compatibility alias for `TempoPoint`.
pub type TempoEvent = TempoPoint;

/// A meter (time signature) change point at a tick position (snapped to bar boundaries).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MeterPoint {
    pub tick: Ticks,
    pub sig: TimeSig,
}

/// Compatibility alias for `MeterPoint`.
pub type MeterEvent = MeterPoint;

impl MeterPoint {
    pub fn new(tick: Ticks, numerator: u32, denominator: u32) -> Self {
        Self {
            tick,
            sig: TimeSig::new(numerator, denominator),
        }
    }

    pub fn numerator(&self) -> u32 {
        self.sig.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.sig.denominator
    }

    pub fn ticks_per_beat(&self) -> Ticks {
        self.sig.ticks_per_beat()
    }

    pub fn ticks_per_bar(&self) -> Ticks {
        self.sig.ticks_per_bar()
    }
}

/// A musical position: 1-based bar and beat plus ticks within the beat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BarBeat {
    pub bar: i64,
    pub beat: i64,
    pub tick: i64,
}

impl BarBeat {
    pub const fn new(bar: i64, beat: i64, tick: i64) -> Self {
        Self { bar, beat, tick }
    }
}

/// Dynamic tempo and meter map for a musical piece or session.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TempoMap {
    tempos: Vec<TempoPoint>,
    meters: Vec<MeterPoint>,
}

impl Default for TempoMap {
    fn default() -> Self {
        Self {
            tempos: vec![TempoPoint { tick: 0, bpm: 120.0 }],
            meters: vec![MeterPoint { tick: 0, sig: TimeSig::COMMON_4_4 }],
        }
    }
}

fn valid_bpm(bpm: f64) -> Result<f64, TimeError> {
    if bpm.is_finite() && (5.0..=1000.0).contains(&bpm) {
        Ok(bpm)
    } else {
        Err(TimeError::Tempo(bpm))
    }
}

impl TempoMap {
    pub fn new(bpm: f64, numerator: u32, denominator: u32) -> Result<Self, TimeError> {
        let mut m = Self::default();
        m.set_tempo(0, bpm)?;
        m.set_meter(0, numerator, denominator)?;
        Ok(m)
    }

    pub fn tempos(&self) -> &[TempoPoint] {
        &self.tempos
    }

    pub fn meters(&self) -> &[MeterPoint] {
        &self.meters
    }

    /// Insert or update the tempo at `tick`.
    pub fn set_tempo(&mut self, tick: Ticks, bpm: f64) -> Result<(), TimeError> {
        let bpm = valid_bpm(bpm)?;
        let tick = tick.max(0);
        match self.tempos.binary_search_by(|e| e.tick.cmp(&tick)) {
            Ok(i) => {
                if let Some(e) = self.tempos.get_mut(i) {
                    e.bpm = bpm;
                }
            }
            Err(i) => self.tempos.insert(i, TempoPoint { tick, bpm }),
        }
        Ok(())
    }

    /// Remove a tempo event (the initial event at tick 0 is pinned).
    pub fn remove_tempo(&mut self, tick: Ticks) -> bool {
        if tick == 0 {
            return false;
        }
        let before = self.tempos.len();
        self.tempos.retain(|e| e.tick != tick);
        before != self.tempos.len()
    }

    /// Insert or update the meter at `tick` (snapped to the bar line at or before it).
    pub fn set_meter(&mut self, tick: Ticks, numerator: u32, denominator: u32) -> Result<(), TimeError> {
        if !(1..=64).contains(&numerator) || ![1, 2, 4, 8, 16, 32, 64].contains(&denominator) {
            return Err(TimeError::Meter(numerator, denominator));
        }
        let tick = if tick <= 0 {
            0
        } else {
            self.bar_start_tick(self.bar_beat_at_tick(tick).bar)
        };
        let ev = MeterPoint::new(tick, numerator, denominator);
        match self.meters.binary_search_by(|e| e.tick.cmp(&tick)) {
            Ok(i) => {
                if let Some(e) = self.meters.get_mut(i) {
                    *e = ev;
                }
            }
            Err(i) => self.meters.insert(i, ev),
        }
        Ok(())
    }

    /// Remove a meter event (the initial event at tick 0 is pinned).
    pub fn remove_meter(&mut self, tick: Ticks) -> bool {
        if tick == 0 {
            return false;
        }
        let before = self.meters.len();
        self.meters.retain(|e| e.tick != tick);
        before != self.meters.len()
    }

    /// Active tempo at `tick`.
    pub fn tempo_at_tick(&self, tick: Ticks) -> f64 {
        let mut bpm = self.tempos.first().map_or(120.0, |e| e.bpm);
        for e in &self.tempos {
            if e.tick <= tick {
                bpm = e.bpm;
            } else {
                break;
            }
        }
        bpm
    }

    /// Active meter at `tick`.
    pub fn meter_at_tick(&self, tick: Ticks) -> MeterPoint {
        let mut m = self.meters.first().copied().unwrap_or(MeterPoint {
            tick: 0,
            sig: TimeSig::COMMON_4_4,
        });
        for e in &self.meters {
            if e.tick <= tick {
                m = *e;
            } else {
                break;
            }
        }
        m
    }

    fn samples_per_tick(bpm: f64, sr: SampleRate) -> f64 {
        sr.as_f64() * 60.0 / (bpm * TICKS_PER_QUARTER as f64)
    }

    /// Convert ticks to samples.
    pub fn tick_to_samples(&self, tick: Ticks, sr: SampleRate) -> Samples {
        to_samples(self.tick_to_samples_f(tick, sr))
    }

    fn tick_to_samples_f(&self, tick: Ticks, sr: SampleRate) -> f64 {
        let mut acc = 0.0;
        let mut seg_tick = 0i64;
        let mut bpm = self.tempos.first().map_or(120.0, |e| e.bpm);
        for e in self.tempos.iter().skip(1) {
            if e.tick >= tick {
                break;
            }
            acc += (e.tick - seg_tick) as f64 * Self::samples_per_tick(bpm, sr);
            seg_tick = e.tick;
            bpm = e.bpm;
        }
        acc + (tick - seg_tick) as f64 * Self::samples_per_tick(bpm, sr)
    }

    /// Convert samples to fractional ticks.
    pub fn samples_to_ticks_f(&self, s: Samples, sr: SampleRate) -> f64 {
        let s = s as f64;
        let mut acc = 0.0;
        let mut seg_tick = 0i64;
        let mut bpm = self.tempos.first().map_or(120.0, |e| e.bpm);
        for e in self.tempos.iter().skip(1) {
            let seg = (e.tick - seg_tick) as f64 * Self::samples_per_tick(bpm, sr);
            if acc + seg > s {
                break;
            }
            acc += seg;
            seg_tick = e.tick;
            bpm = e.bpm;
        }
        seg_tick as f64 + (s - acc) / Self::samples_per_tick(bpm, sr)
    }

    /// Convert samples to integer ticks.
    pub fn samples_to_ticks(&self, s: Samples, sr: SampleRate) -> Ticks {
        to_samples(self.samples_to_ticks_f(s, sr))
    }

    /// Starting tick of 1-based `bar`.
    pub fn bar_start_tick(&self, bar: i64) -> Ticks {
        let mut cur_bar = 1i64;
        let mut cur_tick = 0i64;
        let first = self.meter_at_tick(0);
        if bar < 1 {
            return (bar - 1).saturating_mul(first.ticks_per_bar());
        }
        for (i, m) in self.meters.iter().enumerate() {
            let next_tick = self.meters.get(i + 1).map(|n| n.tick);
            let tpb = m.ticks_per_bar();
            let bars_in_seg = next_tick.map(|nt| ((nt - cur_tick) + tpb - 1) / tpb);
            match bars_in_seg {
                Some(n) if bar >= cur_bar + n => {
                    cur_bar += n;
                    cur_tick = next_tick.unwrap_or(cur_tick);
                }
                _ => return cur_tick.saturating_add((bar - cur_bar).saturating_mul(tpb)),
            }
        }
        cur_tick
    }

    /// Musical `BarBeat` position at `tick`.
    pub fn bar_beat_at_tick(&self, tick: Ticks) -> BarBeat {
        let first = self.meter_at_tick(0);
        if tick < 0 {
            let tpb = first.ticks_per_bar();
            let bars_back = (-tick + tpb - 1) / tpb;
            let bar_tick = -bars_back * tpb;
            let within = tick - bar_tick;
            let beat_t = first.ticks_per_beat();
            return BarBeat {
                bar: 1 - bars_back,
                beat: within / beat_t + 1,
                tick: within % beat_t,
            };
        }
        let mut bar = 1i64;
        let mut seg_tick = 0i64;
        for (i, m) in self.meters.iter().enumerate() {
            let tpb = m.ticks_per_bar();
            if let Some(n) = self.meters.get(i + 1)
                && tick >= n.tick
            {
                bar += ((n.tick - seg_tick) + tpb - 1) / tpb;
                seg_tick = n.tick;
                continue;
            }
            let rel = tick - seg_tick;
            let bars = rel / tpb;
            let within = rel % tpb;
            let beat_t = m.ticks_per_beat();
            return BarBeat {
                bar: bar + bars,
                beat: within / beat_t + 1,
                tick: within % beat_t,
            };
        }
        BarBeat { bar, beat: 1, tick: 0 }
    }

    /// Musical `BarBeat` to absolute `tick`.
    pub fn tick_at_bar_beat(&self, bb: BarBeat) -> Ticks {
        let start = self.bar_start_tick(bb.bar);
        let m = self.meter_at_tick(start);
        start
            .saturating_add((bb.beat - 1).max(0).saturating_mul(m.ticks_per_beat()))
            .saturating_add(bb.tick)
    }

    /// Musical `BarBeat` at sample index `s`.
    pub fn bar_beat_at(&self, s: Samples, sr: SampleRate) -> BarBeat {
        self.bar_beat_at_tick(self.samples_to_ticks(s, sr))
    }

    /// Sample index at musical `BarBeat`.
    pub fn samples_at_bar_beat(&self, bb: BarBeat, sr: SampleRate) -> Samples {
        self.tick_to_samples(self.tick_at_bar_beat(bb), sr)
    }

    /// Scale all tempos by a constant ratio.
    pub fn scale_tempos(&mut self, factor: f64) -> Result<(), TimeError> {
        let mut out = self.tempos.clone();
        for e in &mut out {
            e.bpm = valid_bpm(e.bpm * factor)?;
        }
        self.tempos = out;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_map_is_120_4_4() {
        let m = TempoMap::default();
        let sr = SampleRate::HZ_48000;
        assert_eq!(
            m.samples_at_bar_beat(BarBeat { bar: 2, beat: 1, tick: 0 }, sr),
            96_000
        );
        assert_eq!(m.bar_beat_at(96_000, sr), BarBeat { bar: 2, beat: 1, tick: 0 });
        assert_eq!(m.bar_beat_at(24_000, sr), BarBeat { bar: 1, beat: 2, tick: 0 });
    }

    #[test]
    fn tempo_change_is_integrated() {
        let mut m = TempoMap::default();
        let sr = SampleRate::HZ_48000;
        m.set_tempo(TICKS_PER_QUARTER * 4, 60.0).unwrap();
        assert_eq!(
            m.tick_to_samples(TICKS_PER_QUARTER * 8, sr),
            96_000 + 192_000
        );
        assert_eq!(
            m.samples_to_ticks(96_000 + 192_000, sr),
            TICKS_PER_QUARTER * 8
        );
    }

    #[test]
    fn meter_change_counts_bars() {
        let mut m = TempoMap::default();
        m.set_meter(TICKS_PER_QUARTER * 8, 3, 4).unwrap();
        assert_eq!(
            m.bar_beat_at_tick(TICKS_PER_QUARTER * 8),
            BarBeat { bar: 3, beat: 1, tick: 0 }
        );
        assert_eq!(
            m.bar_beat_at_tick(TICKS_PER_QUARTER * 11),
            BarBeat { bar: 4, beat: 1, tick: 0 }
        );
        assert_eq!(m.bar_start_tick(4), TICKS_PER_QUARTER * 11);
        let bb = BarBeat { bar: 4, beat: 2, tick: 10 };
        assert_eq!(m.bar_beat_at_tick(m.tick_at_bar_beat(bb)), bb);
    }

    #[test]
    fn eighth_note_meters() {
        let m = TempoMap::new(120.0, 6, 8).unwrap();
        assert_eq!(
            m.bar_beat_at_tick(TICKS_PER_QUARTER * 3),
            BarBeat { bar: 2, beat: 1, tick: 0 }
        );
        assert_eq!(
            m.bar_beat_at_tick(TICKS_PER_QUARTER / 2),
            BarBeat { bar: 1, beat: 2, tick: 0 }
        );
    }

    #[test]
    fn rejects_bad_values() {
        let mut m = TempoMap::default();
        assert!(m.set_tempo(0, f64::NAN).is_err());
        assert!(m.set_tempo(0, 0.0).is_err());
        assert!(m.set_meter(0, 0, 4).is_err());
        assert!(m.set_meter(0, 4, 3).is_err());
        assert!(!m.remove_tempo(0));
    }

    #[test]
    fn negative_positions() {
        let m = TempoMap::default();
        assert_eq!(
            m.bar_beat_at_tick(-TICKS_PER_QUARTER * 4),
            BarBeat { bar: 0, beat: 1, tick: 0 }
        );
        assert_eq!(
            m.bar_beat_at_tick(-TICKS_PER_QUARTER),
            BarBeat { bar: 0, beat: 4, tick: 0 }
        );
    }
}

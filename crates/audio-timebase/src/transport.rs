//! Unified transport position, timeline displays, and timebase conversions.

use crate::grid::GridDivision;
use crate::rate::{SampleRate, Samples, Seconds, Ticks};
use crate::tempo::{BarBeat, TempoMap};
use crate::timecode::{FeetFrames, Timecode, TimecodeRate, feet_frames};
use crate::TimeError;

/// Rulers and counter timebase display formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum TimeFormat {
    BarsBeats,
    #[default]
    MinSecs,
    Timecode,
    FeetFrames,
    Samples,
}

impl TimeFormat {
    pub const ALL: [TimeFormat; 5] = [
        TimeFormat::BarsBeats,
        TimeFormat::MinSecs,
        TimeFormat::Timecode,
        TimeFormat::FeetFrames,
        TimeFormat::Samples,
    ];

    pub fn label(self) -> &'static str {
        match self {
            TimeFormat::BarsBeats => "Bars|Beats",
            TimeFormat::MinSecs => "Min:Secs",
            TimeFormat::Timecode => "Timecode",
            TimeFormat::FeetFrames => "Feet+Frames",
            TimeFormat::Samples => "Samples",
        }
    }

    pub fn id(self) -> &'static str {
        match self {
            TimeFormat::BarsBeats => "bars_beats",
            TimeFormat::MinSecs => "min_secs",
            TimeFormat::Timecode => "timecode",
            TimeFormat::FeetFrames => "feet_frames",
            TimeFormat::Samples => "samples",
        }
    }

    pub fn from_id(id: &str) -> Option<TimeFormat> {
        TimeFormat::ALL
            .into_iter()
            .find(|f| f.id() == id || f.label().eq_ignore_ascii_case(id))
    }
}

/// Comprehensive, multi-timebase transport position snapshot.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransportPosition {
    pub samples: Samples,
    pub seconds: Seconds,
    pub ticks: Ticks,
    pub bar_beat: BarBeat,
    pub timecode: Timecode,
    pub feet_frames: FeetFrames,
}

impl TransportPosition {
    /// Calculate all multi-timebase coordinates from raw audio samples.
    pub fn from_samples(
        s: Samples,
        sr: SampleRate,
        map: &TempoMap,
        rate: TimecodeRate,
        tc_start: Samples,
    ) -> Self {
        Self {
            samples: s,
            seconds: sr.seconds(s),
            ticks: map.samples_to_ticks(s, sr),
            bar_beat: map.bar_beat_at(s, sr),
            timecode: Timecode::from_samples(s.saturating_add(tc_start), rate, sr),
            feet_frames: FeetFrames::from_samples(s, rate, sr),
        }
    }

    /// Format this position according to the specified `TimeFormat`.
    pub fn format(&self, fmt: TimeFormat, rate: TimecodeRate) -> String {
        match fmt {
            TimeFormat::Samples => self.samples.to_string(),
            TimeFormat::MinSecs => {
                let neg = self.samples < 0;
                let ms_total = (self.seconds.abs() * 1000.0).floor() as i64;
                let mins = ms_total / 60_000;
                let secs = (ms_total / 1000) % 60;
                let ms = ms_total % 1000;
                format!("{}{mins}:{secs:02}.{ms:03}", if neg { "-" } else { "" })
            }
            TimeFormat::Timecode => self.timecode.format(rate),
            TimeFormat::FeetFrames => self.feet_frames.format(),
            TimeFormat::BarsBeats => {
                format!(
                    "{}| {}| {:03}",
                    self.bar_beat.bar, self.bar_beat.beat, self.bar_beat.tick
                )
            }
        }
    }
}

fn min_secs(s: Samples, sr: SampleRate) -> String {
    let neg = s < 0;
    let ms_total = (sr.seconds(s.saturating_abs()) * 1000.0).floor() as i64;
    let mins = ms_total / 60_000;
    let secs = (ms_total / 1000) % 60;
    let ms = ms_total % 1000;
    format!("{}{mins}:{secs:02}.{ms:03}", if neg { "-" } else { "" })
}

/// Format an absolute sample position into a string.
pub fn format_position(
    s: Samples,
    fmt: TimeFormat,
    sr: SampleRate,
    map: &TempoMap,
    rate: TimecodeRate,
    tc_start: Samples,
) -> String {
    match fmt {
        TimeFormat::Samples => s.to_string(),
        TimeFormat::MinSecs => min_secs(s, sr),
        TimeFormat::Timecode => {
            Timecode::from_samples(s.saturating_add(tc_start), rate, sr).format(rate)
        }
        TimeFormat::FeetFrames => {
            let (ft, fr) = feet_frames(s, rate, sr);
            format!("{ft}+{fr:02}")
        }
        TimeFormat::BarsBeats => {
            let bb = map.bar_beat_at(s, sr);
            format!("{}| {}| {:03}", bb.bar, bb.beat, bb.tick)
        }
    }
}

/// Format a duration or length starting at `start`.
pub fn format_length(
    start: Samples,
    len: Samples,
    fmt: TimeFormat,
    sr: SampleRate,
    map: &TempoMap,
    rate: TimecodeRate,
) -> String {
    match fmt {
        TimeFormat::BarsBeats => {
            let t0 = map.samples_to_ticks(start, sr);
            let t1 = map.samples_to_ticks(start.saturating_add(len), sr);
            let m = map.meter_at_tick(t0);
            let d = (t1 - t0).max(0);
            let bars = d / m.ticks_per_bar().max(1);
            let rem = d % m.ticks_per_bar().max(1);
            format!(
                "{bars}| {}| {:03}",
                rem / m.ticks_per_beat().max(1),
                rem % m.ticks_per_beat().max(1)
            )
        }
        TimeFormat::Timecode => Timecode::from_samples(len, rate, sr).format(rate),
        other => format_position(len, other, sr, map, rate, 0),
    }
}

/// Parse a user-entered position string into samples.
pub fn parse_position(
    text: &str,
    fmt: TimeFormat,
    sr: SampleRate,
    map: &TempoMap,
    rate: TimecodeRate,
    tc_start: Samples,
) -> Result<Samples, TimeError> {
    let t = text.trim();
    let err = || TimeError::Parse(t.to_string(), fmt.label());
    match fmt {
        TimeFormat::Samples => t.replace([',', '_'], "").parse::<i64>().map_err(|_| err()),
        TimeFormat::MinSecs => {
            let (neg, body) = match t.strip_prefix('-') {
                Some(r) => (true, r),
                None => (false, t),
            };
            let (m, s) = match body.split_once(':') {
                Some((m, s)) => (
                    m.trim().parse::<f64>().map_err(|_| err())?,
                    s.trim().parse::<f64>().map_err(|_| err())?,
                ),
                None => (0.0, body.parse::<f64>().map_err(|_| err())?),
            };
            if !m.is_finite() || !s.is_finite() || m < 0.0 || s < 0.0 {
                return Err(err());
            }
            let v = sr.samples(m * 60.0 + s);
            Ok(if neg { -v } else { v })
        }
        TimeFormat::Timecode => {
            Ok(Timecode::parse(t)?.to_samples(rate, sr).saturating_sub(tc_start))
        }
        TimeFormat::FeetFrames => {
            let ff = FeetFrames::parse(t)?;
            Ok(ff.to_samples(rate, sr))
        }
        TimeFormat::BarsBeats => {
            let parts: Vec<i64> = t
                .split(['|', '.', ' '])
                .filter(|p| !p.is_empty())
                .map(|p| p.parse::<i64>())
                .collect::<Result<_, _>>()
                .map_err(|_| err())?;
            let bb = BarBeat {
                bar: *parts.first().ok_or_else(err)?,
                beat: parts.get(1).copied().unwrap_or(1).max(1),
                tick: parts.get(2).copied().unwrap_or(0).max(0),
            };
            Ok(map.samples_at_bar_beat(bb, sr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> (SampleRate, TempoMap, TimecodeRate) {
        (SampleRate::HZ_48000, TempoMap::default(), TimecodeRate::Fps30)
    }

    #[test]
    fn format_min_secs() {
        let (sr, map, rate) = ctx();
        assert_eq!(
            format_position(0, TimeFormat::MinSecs, sr, &map, rate, 0),
            "0:00.000"
        );
        assert_eq!(
            format_position(48_000 * 75 + 24_000, TimeFormat::MinSecs, sr, &map, rate, 0),
            "1:15.500"
        );
    }

    #[test]
    fn transport_position_snapshot() {
        let (sr, map, rate) = ctx();
        let pos = TransportPosition::from_samples(48_000, sr, &map, rate, 0);
        assert_eq!(pos.samples, 48_000);
        assert_eq!(pos.format(TimeFormat::MinSecs, rate), "0:01.000");
        assert_eq!(pos.format(TimeFormat::Timecode, rate), "00:00:01:00");
    }

    #[test]
    fn parse_various_formats() {
        let (sr, map, rate) = ctx();
        assert_eq!(
            parse_position("1:15.500", TimeFormat::MinSecs, sr, &map, rate, 0).unwrap(),
            48_000 * 75 + 24_000
        );
        assert_eq!(
            parse_position("00:00:01:00", TimeFormat::Timecode, sr, &map, rate, 0).unwrap(),
            48_000
        );
        assert_eq!(
            parse_position("1+08", TimeFormat::FeetFrames, sr, &map, TimecodeRate::Fps24, 0).unwrap(),
            48_000
        );
    }
}

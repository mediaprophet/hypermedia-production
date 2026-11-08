//! Fundamental sample rate, sample counts, real seconds, and musical ticks.

use crate::TimeError;

/// Position or length in raw audio samples.
pub type Samples = i64;

/// Position or length in real elapsed seconds.
pub type Seconds = f64;

/// Position or length in musical pulses / ticks.
pub type Ticks = i64;

/// Pulses per quarter note (standard DAW / MIDI PPQ resolution).
pub const PPQ: i64 = 960;

/// Round a float sample count to `i64`, mapping NaN to 0 and saturating infinities.
pub fn to_samples(v: f64) -> Samples {
    if v.is_nan() {
        0
    } else if v >= i64::MAX as f64 {
        i64::MAX
    } else if v <= i64::MIN as f64 {
        i64::MIN
    } else {
        v.round() as i64
    }
}

/// Sample rate in Hertz (supported range: 8,000 Hz to 768,000 Hz).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct SampleRate(u32);

impl SampleRate {
    pub const HZ_44100: SampleRate = SampleRate(44_100);
    pub const HZ_48000: SampleRate = SampleRate(48_000);
    pub const HZ_88200: SampleRate = SampleRate(88_200);
    pub const HZ_96000: SampleRate = SampleRate(96_000);
    pub const HZ_176400: SampleRate = SampleRate(176_400);
    pub const HZ_192000: SampleRate = SampleRate(192_000);
    pub const HZ_352800: SampleRate = SampleRate(352_800);
    pub const HZ_384000: SampleRate = SampleRate(384_000);

    pub const COMMON: [u32; 8] = [
        44_100, 48_000, 88_200, 96_000, 176_400, 192_000, 352_800, 384_000,
    ];

    pub fn new(hz: u32) -> Result<Self, TimeError> {
        if (8_000..=768_000).contains(&hz) {
            Ok(SampleRate(hz))
        } else {
            Err(TimeError::SampleRate(hz))
        }
    }

    pub fn hz(self) -> u32 {
        self.0
    }

    pub fn as_f64(self) -> f64 {
        f64::from(self.0)
    }

    /// Convert samples to real seconds.
    pub fn seconds(self, s: Samples) -> Seconds {
        s as f64 / self.as_f64()
    }

    /// Convert real seconds to samples, rounded to nearest and saturated.
    pub fn samples(self, seconds: Seconds) -> Samples {
        to_samples(seconds * self.as_f64())
    }
}

impl Default for SampleRate {
    fn default() -> Self {
        SampleRate::HZ_48000
    }
}

/// A half-open sample range `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct SampleRange {
    pub start: Samples,
    pub end: Samples,
}

impl SampleRange {
    pub fn new(a: Samples, b: Samples) -> Self {
        if a <= b {
            SampleRange { start: a, end: b }
        } else {
            SampleRange { start: b, end: a }
        }
    }
    pub fn point(at: Samples) -> Self {
        SampleRange { start: at, end: at }
    }
    pub fn len(&self) -> Samples {
        self.end.saturating_sub(self.start)
    }
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    pub fn contains(&self, s: Samples) -> bool {
        s >= self.start && s < self.end
    }
    pub fn overlaps(&self, o: &SampleRange) -> bool {
        self.start < o.end && o.start < self.end
    }
    pub fn intersect(&self, o: &SampleRange) -> Option<SampleRange> {
        let s = self.start.max(o.start);
        let e = self.end.min(o.end);
        (s < e).then_some(SampleRange { start: s, end: e })
    }
    pub fn shifted(&self, by: Samples) -> SampleRange {
        SampleRange {
            start: self.start.saturating_add(by),
            end: self.end.saturating_add(by),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_rate_bounds() {
        assert!(SampleRate::new(48_000).is_ok());
        assert!(SampleRate::new(0).is_err());
        assert!(SampleRate::new(10_000_000).is_err());
    }

    #[test]
    fn seconds_round_trip() {
        let sr = SampleRate::HZ_44100;
        assert_eq!(sr.samples(1.0), 44_100);
        assert!((sr.seconds(22_050) - 0.5).abs() < 1e-12);
    }

    #[test]
    fn to_samples_is_total() {
        assert_eq!(to_samples(f64::NAN), 0);
        assert_eq!(to_samples(f64::INFINITY), i64::MAX);
        assert_eq!(to_samples(f64::NEG_INFINITY), i64::MIN);
    }

    #[test]
    fn range_ops() {
        let a = SampleRange::new(10, 0);
        assert_eq!(a, SampleRange { start: 0, end: 10 });
        assert_eq!(a.intersect(&SampleRange::new(5, 20)), Some(SampleRange::new(5, 10)));
        assert_eq!(a.intersect(&SampleRange::new(10, 20)), None);
        assert!(a.contains(9) && !a.contains(10));
    }
}

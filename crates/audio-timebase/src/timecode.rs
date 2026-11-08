//! SMPTE timecode (drop-frame and non-drop-frame) and 35mm Feet+Frames film timebases.

use crate::rate::{SampleRate, Samples};
use crate::TimeError;

/// Standard SMPTE and high-frame-rate timecode rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub enum TimecodeRate {
    Fps23_976,
    #[default]
    Fps24,
    Fps25,
    Fps29_97,
    Fps29_97Drop,
    Fps30,
    Fps47_952,
    Fps48,
    Fps50,
    Fps59_94,
    Fps59_94Drop,
    Fps60,
    Fps100,
    Fps119_88,
    Fps120,
}

/// Compatibility alias for `TimecodeRate`.
pub type FrameRate = TimecodeRate;

impl TimecodeRate {
    pub const ALL: [TimecodeRate; 15] = [
        TimecodeRate::Fps23_976,
        TimecodeRate::Fps24,
        TimecodeRate::Fps25,
        TimecodeRate::Fps29_97,
        TimecodeRate::Fps29_97Drop,
        TimecodeRate::Fps30,
        TimecodeRate::Fps47_952,
        TimecodeRate::Fps48,
        TimecodeRate::Fps50,
        TimecodeRate::Fps59_94,
        TimecodeRate::Fps59_94Drop,
        TimecodeRate::Fps60,
        TimecodeRate::Fps100,
        TimecodeRate::Fps119_88,
        TimecodeRate::Fps120,
    ];

    /// Exact rational frame rate (numerator, denominator).
    pub fn rational(self) -> (i64, i64) {
        match self {
            TimecodeRate::Fps23_976 => (24_000, 1001),
            TimecodeRate::Fps24 => (24, 1),
            TimecodeRate::Fps25 => (25, 1),
            TimecodeRate::Fps29_97 | TimecodeRate::Fps29_97Drop => (30_000, 1001),
            TimecodeRate::Fps30 => (30, 1),
            TimecodeRate::Fps47_952 => (48_000, 1001),
            TimecodeRate::Fps48 => (48, 1),
            TimecodeRate::Fps50 => (50, 1),
            TimecodeRate::Fps59_94 | TimecodeRate::Fps59_94Drop => (60_000, 1001),
            TimecodeRate::Fps60 => (60, 1),
            TimecodeRate::Fps100 => (100, 1),
            TimecodeRate::Fps119_88 => (120_000, 1001),
            TimecodeRate::Fps120 => (120, 1),
        }
    }

    /// Nominal frames per second integer base.
    pub fn nominal(self) -> i64 {
        let (n, d) = self.rational();
        (n + d / 2) / d
    }

    /// Whether this is a drop-frame SMPTE timecode rate.
    pub fn is_drop(self) -> bool {
        matches!(self, TimecodeRate::Fps29_97Drop | TimecodeRate::Fps59_94Drop)
    }

    /// Short human-readable display label.
    pub fn label(self) -> &'static str {
        match self {
            TimecodeRate::Fps23_976 => "23.976",
            TimecodeRate::Fps24 => "24",
            TimecodeRate::Fps25 => "25",
            TimecodeRate::Fps29_97 => "29.97",
            TimecodeRate::Fps29_97Drop => "29.97 Drop",
            TimecodeRate::Fps30 => "30",
            TimecodeRate::Fps47_952 => "47.952",
            TimecodeRate::Fps48 => "48",
            TimecodeRate::Fps50 => "50",
            TimecodeRate::Fps59_94 => "59.94",
            TimecodeRate::Fps59_94Drop => "59.94 Drop",
            TimecodeRate::Fps60 => "60",
            TimecodeRate::Fps100 => "100",
            TimecodeRate::Fps119_88 => "119.88",
            TimecodeRate::Fps120 => "120",
        }
    }

    /// Total whole frames at a sample position (floored).
    pub fn frames_at(self, s: Samples, sr: SampleRate) -> i64 {
        let (n, d) = self.rational();
        let num = i128::from(s) * i128::from(n);
        let den = i128::from(sr.hz()) * i128::from(d);
        if den == 0 {
            return 0;
        }
        i64::try_from(num.div_euclid(den)).unwrap_or(0)
    }

    /// Sample position at the beginning of frame `f`.
    pub fn samples_at_frame(self, f: i64, sr: SampleRate) -> Samples {
        let (n, d) = self.rational();
        if n == 0 {
            return 0;
        }
        let v = (i128::from(f) * i128::from(sr.hz()) * i128::from(d) + i128::from(n) / 2) / i128::from(n);
        i64::try_from(v).unwrap_or(0)
    }

    /// Samples per frame (fractional).
    pub fn samples_per_frame(self, sr: SampleRate) -> f64 {
        let (n, d) = self.rational();
        sr.as_f64() * d as f64 / n as f64
    }
}

/// A standard SMPTE timecode label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Timecode {
    pub negative: bool,
    pub hours: i64,
    pub minutes: i64,
    pub seconds: i64,
    pub frames: i64,
}

impl Timecode {
    pub const fn new(negative: bool, hours: i64, minutes: i64, seconds: i64, frames: i64) -> Self {
        Self {
            negative,
            hours,
            minutes,
            seconds,
            frames,
        }
    }

    /// Compute timecode label for an absolute frame count.
    pub fn from_frames(frames: i64, rate: TimecodeRate) -> Timecode {
        let negative = frames < 0;
        let mut f = frames.saturating_abs();
        let base = rate.nominal().max(1);
        if rate.is_drop() {
            let drop = if base >= 60 { 4 } else { 2 };
            let per_10min = base * 600 - drop * 9;
            let per_min = base * 60 - drop;
            let tens = f / per_10min;
            let rem = f % per_10min;
            let extra = if rem > drop {
                drop * 9 * tens + drop * ((rem - drop) / per_min)
            } else {
                drop * 9 * tens
            };
            f += extra;
        }
        let frames_l = f % base;
        let total_secs = f / base;
        Timecode {
            negative,
            hours: total_secs / 3600,
            minutes: (total_secs / 60) % 60,
            seconds: total_secs % 60,
            frames: frames_l,
        }
    }

    /// Absolute frame count corresponding to this timecode label.
    pub fn to_frames(&self, rate: TimecodeRate) -> i64 {
        let base = rate.nominal().max(1);
        let total_min = self.hours * 60 + self.minutes;
        let mut f = (total_min * 60 + self.seconds) * base + self.frames;
        if rate.is_drop() {
            let drop = if base >= 60 { 4 } else { 2 };
            f -= drop * (total_min - total_min / 10);
        }
        if self.negative { -f } else { f }
    }

    /// Derive timecode label from sample position.
    pub fn from_samples(s: Samples, rate: TimecodeRate, sr: SampleRate) -> Timecode {
        Timecode::from_frames(rate.frames_at(s, sr), rate)
    }

    /// Sample index at the start of this timecode.
    pub fn to_samples(&self, rate: TimecodeRate, sr: SampleRate) -> Samples {
        rate.samples_at_frame(self.to_frames(rate), sr)
    }

    /// Format as standard `HH:MM:SS:FF` or drop-frame `HH:MM:SS;FF`.
    pub fn format(&self, rate: TimecodeRate) -> String {
        let sep = if rate.is_drop() { ';' } else { ':' };
        let sign = if self.negative { "-" } else { "" };
        format!(
            "{sign}{:02}:{:02}:{:02}{sep}{:02}",
            self.hours, self.minutes, self.seconds, self.frames
        )
    }

    /// Parse timecode string (`HH:MM:SS:FF`, with `:` / `;` / `.` separators).
    pub fn parse(text: &str) -> Result<Timecode, TimeError> {
        let t = text.trim();
        let (negative, t) = match t.strip_prefix('-') {
            Some(r) => (true, r),
            None => (false, t),
        };
        let parts: Vec<&str> = t.split([':', ';', '.']).collect();
        if parts.is_empty() || parts.len() > 4 {
            return Err(TimeError::Parse(text.to_string(), "timecode"));
        }
        let mut nums = [0i64; 4];
        let offset = 4 - parts.len();
        for (i, p) in parts.iter().enumerate() {
            let v: i64 = p
                .trim()
                .parse()
                .map_err(|_| TimeError::Parse(text.to_string(), "timecode"))?;
            if v < 0 {
                return Err(TimeError::Parse(text.to_string(), "timecode"));
            }
            if let Some(slot) = nums.get_mut(offset + i) {
                *slot = v;
            }
        }
        Ok(Timecode {
            negative,
            hours: nums[0],
            minutes: nums[1],
            seconds: nums[2],
            frames: nums[3],
        })
    }
}

/// 35mm film position in Feet + Frames (16 frames per foot).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
pub struct FeetFrames {
    pub feet: i64,
    pub frames: i64,
}

impl FeetFrames {
    pub const FRAMES_PER_FOOT: i64 = 16;

    pub const fn new(feet: i64, frames: i64) -> Self {
        Self { feet, frames }
    }

    /// Derive FeetFrames from raw audio samples.
    pub fn from_samples(s: Samples, rate: TimecodeRate, sr: SampleRate) -> Self {
        let f = rate.frames_at(s, sr);
        Self {
            feet: f.div_euclid(Self::FRAMES_PER_FOOT),
            frames: f.rem_euclid(Self::FRAMES_PER_FOOT),
        }
    }

    /// Total frames represented by this FeetFrames value.
    pub fn to_total_frames(&self) -> i64 {
        self.feet.saturating_mul(Self::FRAMES_PER_FOOT).saturating_add(self.frames)
    }

    /// Convert FeetFrames to audio samples.
    pub fn to_samples(&self, rate: TimecodeRate, sr: SampleRate) -> Samples {
        rate.samples_at_frame(self.to_total_frames(), sr)
    }

    /// Format as standard `FEET+FRAMES` (e.g. `1+08`).
    pub fn format(&self) -> String {
        format!("{}+{:#02}", self.feet, self.frames)
    }

    /// Parse `FEET+FRAMES` format string.
    pub fn parse(text: &str) -> Result<Self, TimeError> {
        let t = text.trim();
        let (ft, fr) = t.split_once('+').ok_or_else(|| TimeError::Parse(t.to_string(), "feet_frames"))?;
        let feet: i64 = ft.trim().parse().map_err(|_| TimeError::Parse(t.to_string(), "feet_frames"))?;
        let frames: i64 = fr.trim().parse().map_err(|_| TimeError::Parse(t.to_string(), "feet_frames"))?;
        Ok(Self { feet, frames })
    }
}

/// Compatibility function returning raw `(feet, frames)` tuple for 35mm film.
pub fn feet_frames(s: Samples, rate: TimecodeRate, sr: SampleRate) -> (i64, i64) {
    let ff = FeetFrames::from_samples(s, rate, sr);
    (ff.feet, ff.frames)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_drop_round_trip() {
        let sr = SampleRate::HZ_48000;
        let tc = Timecode::from_samples(48_000 * 3661, TimecodeRate::Fps25, sr);
        assert_eq!(tc.format(TimecodeRate::Fps25), "01:01:01:00");
        assert_eq!(tc.to_samples(TimecodeRate::Fps25, sr), 48_000 * 3661);
    }

    #[test]
    fn drop_frame_labels() {
        let r = TimecodeRate::Fps29_97Drop;
        assert_eq!(Timecode::from_frames(1800, r).format(r), "00:01:00;02");
        assert_eq!(Timecode::from_frames(1799, r).format(r), "00:00:59;29");
        assert_eq!(Timecode::from_frames(17_982, r).format(r), "00:10:00;00");
        for f in [0, 1, 1799, 1800, 1801, 17_981, 17_982, 107_892, 500_000] {
            assert_eq!(Timecode::from_frames(f, r).to_frames(r), f, "frame {f}");
        }
    }

    #[test]
    fn parse_variants() {
        let tc = Timecode::parse("01:02:03:04").unwrap();
        assert_eq!((tc.hours, tc.minutes, tc.seconds, tc.frames), (1, 2, 3, 4));
        let tc = Timecode::parse("3:04").unwrap();
        assert_eq!((tc.seconds, tc.frames), (3, 4));
        assert!(Timecode::parse("a:b").is_err());
        assert!(Timecode::parse("1:2:3:4:5").is_err());
    }

    #[test]
    fn feet_and_frames() {
        let sr = SampleRate::HZ_48000;
        assert_eq!(feet_frames(48_000, TimecodeRate::Fps24, sr), (1, 8));
        let ff = FeetFrames::from_samples(48_000, TimecodeRate::Fps24, sr);
        assert_eq!(ff.format(), "1+08");
        assert_eq!(FeetFrames::parse("1+08").unwrap(), FeetFrames::new(1, 8));
    }
}

//! Timecode and exact rational media time representations.
//!
//! Grounded in SMPTE ST 12-1, ST 2067-2, and international interoperability standards.

use serde::{Deserialize, Serialize};

/// High-resolution integer ticks per second (254,016,000,000 Hz),
/// divisible by 24, 25, 30, 48, 50, 60, 1000, 44100, 48000, 96000, 192000.
pub const TICKS_PER_SECOND: i64 = 254_016_000_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tick(pub i64);

impl Tick {
    pub const ZERO: Self = Self(0);

    pub fn from_rational(pts: i64, num: i64, den: i64) -> Self {
        if den == 0 {
            return Self::ZERO;
        }
        let numerator = (pts as i128) * (num as i128) * (TICKS_PER_SECOND as i128);
        Self((numerator / (den as i128)) as i64)
    }

    pub fn to_rational(self, num: i64, den: i64) -> i64 {
        let unit = (num as i128) * (TICKS_PER_SECOND as i128);
        if unit == 0 {
            return 0;
        }
        let numerator = (self.0 as i128) * (den as i128);
        (numerator / unit) as i64
    }

    pub fn from_seconds(s: f64) -> Self {
        Self((s * TICKS_PER_SECOND as f64).round() as i64)
    }

    pub fn as_seconds(self) -> f64 {
        self.0 as f64 / TICKS_PER_SECOND as f64
    }

    pub fn from_millis(ms: i64) -> Self {
        Self(ms * (TICKS_PER_SECOND / 1000))
    }

    pub fn as_millis(self) -> i64 {
        self.0 / (TICKS_PER_SECOND / 1000)
    }
}

impl std::ops::Add for Tick {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Tick {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FrameRate {
    pub num: i64,
    pub den: i64,
}

impl Default for FrameRate {
    fn default() -> Self {
        Self::FPS_24
    }
}

impl FrameRate {
    pub const FPS_23_976: Self = Self { num: 24000, den: 1001 };
    pub const FPS_24: Self = Self { num: 24, den: 1 };
    pub const FPS_25: Self = Self { num: 25, den: 1 };
    pub const FPS_29_97: Self = Self { num: 30000, den: 1001 };
    pub const FPS_30: Self = Self { num: 30, den: 1 };
    pub const FPS_50: Self = Self { num: 50, den: 1 };
    pub const FPS_59_94: Self = Self { num: 60000, den: 1001 };
    pub const FPS_60: Self = Self { num: 60, den: 1 };

    pub fn as_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn frame_at(self, t: Tick) -> i64 {
        t.to_rational(self.num, self.den)
    }

    pub fn tick_of(self, frame: i64) -> Tick {
        Tick::from_rational(frame, self.den, self.num)
    }

    pub fn is_drop_frame_compatible(self) -> bool {
        self == Self::FPS_29_97 || self == Self::FPS_59_94
    }
}

/// Convert frame count to SMPTE timecode (HH:MM:SS:FF or HH:MM:SS;FF).
pub fn format_timecode(frames: i64, rate: FrameRate, drop_frame: bool) -> String {
    if drop_frame && rate.is_drop_frame_compatible() {
        let (fps_int, drop_per_min) = if rate == FrameRate::FPS_59_94 {
            (60, 4)
        } else {
            (30, 2)
        };
        let frames_per_10min = fps_int * 60 * 10 - drop_per_min * 9;
        let frames_per_min = fps_int * 60 - drop_per_min;

        let mut f = frames.max(0);
        let d = f / frames_per_10min;
        let m = f % frames_per_10min;
        if m > drop_per_min {
            f += drop_per_min * 9 * d + drop_per_min * ((m - drop_per_min) / frames_per_min);
        } else {
            f += drop_per_min * 9 * d;
        }

        let ff = f % fps_int;
        let ss = (f / fps_int) % 60;
        let mm = (f / (fps_int * 60)) % 60;
        let hh = f / (fps_int * 3600);
        format!("{hh:02}:{mm:02}:{ss:02};{ff:02}")
    } else {
        let fps = (rate.as_f64().round() as i64).max(1);
        let f = frames.max(0);
        let ff = f % fps;
        let ss = (f / fps) % 60;
        let mm = (f / (fps * 60)) % 60;
        let hh = f / (fps * 3600);
        format!("{hh:02}:{mm:02}:{ss:02}:{ff:02}")
    }
}

/// Parse a timecode string (`HH:MM:SS:FF` or `HH:MM:SS;FF`) into frame count.
pub fn parse_timecode(s: &str, rate: FrameRate) -> Option<(i64, bool)> {
    let s = s.trim();
    let is_df = s.contains(';') || s.contains(',');
    let parts: Vec<&str> = s.split([':', ';', '.', ',']).collect();
    if parts.len() != 4 {
        return None;
    }
    let h: i64 = parts[0].parse().ok()?;
    let m: i64 = parts[1].parse().ok()?;
    let sec: i64 = parts[2].parse().ok()?;
    let f: i64 = parts[3].parse().ok()?;

    if is_df && rate.is_drop_frame_compatible() {
        let (fps_int, drop) = if rate == FrameRate::FPS_59_94 { (60, 4) } else { (30, 2) };
        let total_minutes = h * 60 + m;
        let total_frames = ((total_minutes * 60 + sec) * fps_int + f) - drop * (total_minutes - total_minutes / 10);
        Some((total_frames, true))
    } else {
        let fps = (rate.as_f64().round() as i64).max(1);
        let total_frames = ((h * 60 + m) * 60 + sec) * fps + f;
        Some((total_frames, false))
    }
}

/// Parse clock format: `HH:MM:SS.mmm` or `HH:MM:SS,mmm` into `Tick`.
pub fn parse_clock(s: &str) -> Option<Tick> {
    let s = s.trim();
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() < 2 || parts.len() > 3 {
        return None;
    }
    let (h, m, sec_part) = if parts.len() == 3 {
        (parts[0].parse::<i64>().ok()?, parts[1].parse::<i64>().ok()?, parts[2])
    } else {
        (0, parts[0].parse::<i64>().ok()?, parts[1])
    };

    let (sec, ms) = if let Some((s_str, ms_str)) = sec_part.split_once(['.', ',']) {
        let sec: i64 = s_str.parse().ok()?;
        let mut ms_val: i64 = ms_str.parse().ok()?;
        let digits = ms_str.len();
        if digits == 1 {
            ms_val *= 100;
        } else if digits == 2 {
            ms_val *= 10;
        } else if digits > 3 {
            ms_val /= 10i64.pow((digits - 3) as u32);
        }
        (sec, ms_val)
    } else {
        (sec_part.parse::<i64>().ok()?, 0)
    };

    let total_ms = ((h * 60 + m) * 60 + sec) * 1000 + ms;
    Some(Tick::from_millis(total_ms))
}

/// Format `Tick` as SRT/WebVTT clock (`HH:MM:SS,mmm` or `HH:MM:SS.mmm`).
pub fn format_clock(t: Tick, comma: bool) -> String {
    let ms = t.as_millis().max(0);
    let s = ms / 1000;
    let rem_ms = ms % 1000;
    let sec = s % 60;
    let min = (s / 60) % 60;
    let hour = s / 3600;
    let sep = if comma { ',' } else { '.' };
    format!("{hour:02}:{min:02}:{sec:02}{sep}{rem_ms:03}")
}

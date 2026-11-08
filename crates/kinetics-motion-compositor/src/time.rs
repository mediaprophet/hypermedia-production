//! Exact rational media time and ticks.

use serde::{Deserialize, Serialize};

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

impl FrameRate {
    pub const FPS_24: Self = Self { num: 24, den: 1 };
    pub const FPS_25: Self = Self { num: 25, den: 1 };
    pub const FPS_30: Self = Self { num: 30, den: 1 };
    pub const FPS_60: Self = Self { num: 60, den: 1 };

    pub fn as_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }

    pub fn frame_at(self, t: Tick) -> f64 {
        t.as_seconds() * self.as_f64()
    }

    pub fn tick_of_frame(self, f: f64) -> Tick {
        Tick::from_seconds(f / self.as_f64())
    }
}

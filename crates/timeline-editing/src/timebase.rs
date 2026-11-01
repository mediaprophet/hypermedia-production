//! Exact rational media time adapted from FilmCraft's `filmcraft-time` crate.

/// Integer ticks per second. This value represents the common frame rates and
/// audio sample rates used by the source media timeline without per-edit float
/// rounding.
pub const TICKS_PER_SECOND: i64 = 254_016_000_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub i64);

impl Tick {
    pub const ZERO: Self = Self(0);

    /// Convert a rational timestamp `pts * num / den` seconds. A zero timebase
    /// maps to zero, matching the source operation's total conversion rule.
    pub fn from_rational(pts: i64, num: i64, den: i64) -> Self {
        if den == 0 {
            return Self::ZERO;
        }
        let Some(numerator) = i128::from(pts)
            .checked_mul(i128::from(num))
            .and_then(|n| n.checked_mul(i128::from(TICKS_PER_SECOND)))
        else {
            return Self(if (pts < 0) ^ (num < 0) ^ (den < 0) {
                i64::MIN
            } else {
                i64::MAX
            });
        };
        Self(clamp_i64(numerator.div_euclid(i128::from(den))))
    }

    /// Inverse rational conversion rounded down in the requested timebase.
    pub fn to_rational_floor(self, num: i64, den: i64) -> i64 {
        let numerator = i128::from(self.0) * i128::from(den);
        let Some(unit) = i128::from(num).checked_mul(i128::from(TICKS_PER_SECOND)) else {
            return 0;
        };
        if unit == 0 {
            return 0;
        }
        clamp_i64(numerator.div_euclid(unit))
    }

    /// Convert a count of units at `per_second`, such as audio samples,
    /// truncating toward zero as the source primitive does.
    pub fn from_units(units: i64, per_second: i64) -> Self {
        if per_second == 0 {
            return Self::ZERO;
        }
        Self(clamp_i64(
            (i128::from(units) * i128::from(TICKS_PER_SECOND))
                .checked_div(i128::from(per_second))
                .unwrap_or(0),
        ))
    }

    /// Convert to units at `per_second`, rounded down (unlike `from_units`,
    /// which preserves the source's truncation-toward-zero rule).
    pub fn to_units_floor(self, per_second: i64) -> i64 {
        if per_second == 0 {
            return 0;
        }
        clamp_i64(
            (i128::from(self.0) * i128::from(per_second))
                .div_euclid(i128::from(TICKS_PER_SECOND)),
        )
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    /// Floor-scale by a rational factor. A zero divisor maps to zero, matching
    /// the source time primitive's non-panicking conversion behavior.
    pub fn mul_ratio(self, numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            return Self::ZERO;
        }
        Self(clamp_i64(
            (i128::from(self.0) * i128::from(numerator))
                .div_euclid(i128::from(denominator)),
        ))
    }

    pub fn max(self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }

    pub fn min(self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }
}

/// A half-open media interval `[start, start + duration)`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeRange {
    pub start: Tick,
    pub duration: Tick,
}

impl TimeRange {
    pub const fn new(start: Tick, duration: Tick) -> Self {
        Self { start, duration }
    }

    pub fn end(self) -> Tick {
        self.start
            .checked_add(self.duration)
            .unwrap_or(Tick(if self.duration.0 < 0 { i64::MIN } else { i64::MAX }))
    }

    pub fn is_empty(self) -> bool {
        self.duration.0 <= 0
    }
}

/// A rational frame rate (`num / den` frames per second).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FrameRate {
    pub num: i64,
    pub den: i64,
}

impl Default for FrameRate {
    fn default() -> Self {
        Self::FPS_23_976
    }
}

impl FrameRate {
    pub const FPS_23_976: Self = Self { num: 24_000, den: 1_001 };
    pub const FPS_24: Self = Self { num: 24, den: 1 };
    pub const FPS_25: Self = Self { num: 25, den: 1 };
    pub const FPS_29_97: Self = Self { num: 30_000, den: 1_001 };
    pub const FPS_30: Self = Self { num: 30, den: 1 };
    pub const FPS_48: Self = Self { num: 48, den: 1 };
    pub const FPS_50: Self = Self { num: 50, den: 1 };
    pub const FPS_59_94: Self = Self { num: 60_000, den: 1_001 };
    pub const FPS_60: Self = Self { num: 60, den: 1 };
    pub const FPS_119_88: Self = Self { num: 120_000, den: 1_001 };
    pub const FPS_120: Self = Self { num: 120, den: 1 };

    pub fn new(num: i64, den: i64) -> Self {
        let divisor = gcd(num.unsigned_abs(), den.unsigned_abs()).max(1) as i128;
        Self {
            num: (i128::from(num) / divisor) as i64,
            den: (i128::from(den) / divisor) as i64,
        }
    }

    /// Invalid or non-positive rates use the source's documented default.
    pub fn sane(self) -> Self {
        if self.num > 0 && self.den > 0 {
            self
        } else {
            Self::default()
        }
    }

    pub fn frame_duration(self) -> Tick {
        let rate = self.sane();
        Tick(clamp_i64(
            (i128::from(TICKS_PER_SECOND) * i128::from(rate.den))
                .div_euclid(i128::from(rate.num)),
        ))
    }

    /// Index of the frame containing `time`, rounded down.
    pub fn frame_at(self, time: Tick) -> i64 {
        let rate = self.sane();
        clamp_i64(frame_index(rate, time))
    }

    /// Start tick for frame `frame`, rounded up so the tick belongs to that frame.
    pub fn tick_of(self, frame: i64) -> Tick {
        let rate = self.sane();
        Tick(clamp_i64(frame_boundary(rate, i128::from(frame))))
    }

    /// Snap down to a frame boundary.
    pub fn snap(self, time: Tick) -> Tick {
        let rate = self.sane();
        Tick(clamp_i64(frame_boundary(rate, frame_index(rate, time))))
    }
}

/// Keep the mathematical frame index wide until the public API needs an i64.
/// Products of the input tick and rate numerator fit in i128.
fn frame_index(rate: FrameRate, time: Tick) -> i128 {
    (i128::from(time.0) * i128::from(rate.num))
        .div_euclid(i128::from(TICKS_PER_SECOND) * i128::from(rate.den))
}

/// Ceiling division for a frame boundary, matching FilmCraft's exact rational
/// path. Only the final public Tick is narrowed; an overflowing wide numerator
/// maps to the corresponding endpoint before that final clamp.
fn frame_boundary(rate: FrameRate, frame: i128) -> i128 {
    let Some(numerator) = frame
        .checked_mul(i128::from(TICKS_PER_SECOND))
        .and_then(|n| n.checked_mul(i128::from(rate.den)))
    else {
        return if frame < 0 { i128::MIN } else { i128::MAX };
    };
    let denominator = i128::from(rate.num);
    let quotient = numerator.div_euclid(denominator);
    quotient + if numerator.rem_euclid(denominator) == 0 { 0 } else { 1 }
}

fn clamp_i64(value: i128) -> i64 {
    value.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_rates_and_audio_units_have_exact_boundaries() {
        let ntsc = FrameRate::FPS_23_976;
        assert_eq!(ntsc.tick_of(24_000), Tick(TICKS_PER_SECOND * 1_001));
        assert_eq!(ntsc.frame_at(ntsc.tick_of(24_000)), 24_000);
        assert_eq!(FrameRate::FPS_24.tick_of(24), Tick(TICKS_PER_SECOND));
        assert_eq!(Tick::from_units(44_100, 44_100), Tick(TICKS_PER_SECOND));
        assert_eq!(Tick(TICKS_PER_SECOND).to_units_floor(48_000), 48_000);
    }

    #[test]
    fn signed_unit_conversion_keeps_source_truncation_distinct_from_floor() {
        assert_eq!(Tick::from_units(-1, 13), Tick(-19_539_692_307));
        assert_eq!(Tick(-1).to_units_floor(13), -1);
    }

    #[test]
    fn rational_container_time_round_trips_to_its_timebase() {
        let pts = 270_000_i64;
        let time = Tick::from_rational(pts, 1, 90_000);
        assert_eq!(time, Tick(TICKS_PER_SECOND * 3));
        assert_eq!(time.to_rational_floor(1, 90_000), pts);
    }

    #[test]
    fn invalid_rates_and_zero_denominators_do_not_panic() {
        assert_eq!(FrameRate::new(0, 1).sane(), FrameRate::default());
        assert_eq!(Tick::from_rational(50, 1, 0), Tick::ZERO);
        assert_eq!(Tick(10).mul_ratio(3, 0), Tick::ZERO);
    }

    #[test]
    fn snap_keeps_exotic_frame_indices_wide_until_the_tick_endpoint() {
        let rate = FrameRate::new(i64::MAX, 1);
        let time = Tick(i64::MAX);
        // The source frame index is far larger than i64, but its boundary still
        // lands at the representable input tick. Narrowing the index first
        // incorrectly snaps this timestamp to a handful of ticks.
        assert!(frame_index(rate.sane(), time) > i128::from(i64::MAX));
        assert_ne!(rate.tick_of(rate.frame_at(time)), time);
        assert_eq!(rate.snap(time), Tick(i64::MAX));
    }
}

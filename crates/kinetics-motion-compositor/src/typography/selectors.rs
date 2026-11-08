//! Text selectors: Range selector shapes, modes, and easing.
//!
//! Grounded in professional motion graphics text animation semantics.

use serde::{Deserialize, Serialize};

/// Range selector profile shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Shape {
    #[default]
    Square,
    RampUp,
    RampDown,
    Triangle,
    Round,
    Smooth,
}

/// Selector combination mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[default]
    Add,
    Subtract,
    Intersect,
    Min,
    Max,
    Difference,
}

impl Mode {
    pub fn initial(self) -> f64 {
        match self {
            Mode::Subtract | Mode::Intersect | Mode::Min => 1.0,
            Mode::Add | Mode::Max | Mode::Difference => 0.0,
        }
    }

    pub fn combine(self, acc: f64, v: f64) -> f64 {
        let r = match self {
            Mode::Add => acc + v,
            Mode::Subtract => acc - v,
            Mode::Intersect => acc * v,
            Mode::Min => acc.min(v),
            Mode::Max => acc.max(v),
            Mode::Difference => (acc - v).abs(),
        };
        r.clamp(-1.0, 1.0)
    }
}

/// Unit counting metric.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BasedOn {
    #[default]
    Characters,
    CharactersExcludingSpaces,
    Words,
    Lines,
}

/// Range selector configuration.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RangeSelector {
    pub start: f64,
    pub end: f64,
    pub offset: f64,
    pub amount: f64,
    pub shape: Shape,
    pub smoothness: f64,
    pub ease_high: f64,
    pub ease_low: f64,
    pub randomize_order: bool,
    pub random_seed: u32,
}

impl Default for RangeSelector {
    fn default() -> Self {
        Self {
            start: 0.0,
            end: 1.0,
            offset: 0.0,
            amount: 1.0,
            shape: Shape::Square,
            smoothness: 1.0,
            ease_high: 0.0,
            ease_low: 0.0,
            randomize_order: false,
            random_seed: 0,
        }
    }
}

impl RangeSelector {
    /// Evaluate the selector weight in [-1, 1] for unit index `i` out of `total_units`.
    pub fn evaluate_unit(&self, i: usize, total_units: usize) -> f64 {
        if total_units == 0 {
            return 0.0;
        }

        let u_idx = if self.randomize_order {
            pseudo_permute(i, total_units, self.random_seed)
        } else {
            i
        };

        let unit_norm = u_idx as f64 / total_units.max(1) as f64;
        let s = (self.start + self.offset).clamp(0.0, 1.0);
        let e = (self.end + self.offset).clamp(0.0, 1.0);

        if (e - s).abs() < 1e-6 {
            return 0.0;
        }

        let f = (unit_norm - s) / (e - s);
        let raw = shape_value(self.shape, f);
        let eased = apply_ease(raw, self.ease_high, self.ease_low);
        (eased * self.amount).clamp(-1.0, 1.0)
    }
}

pub fn shape_value(shape: Shape, f: f64) -> f64 {
    match shape {
        Shape::Square => {
            if (0.0..=1.0).contains(&f) {
                1.0
            } else {
                0.0
            }
        }
        Shape::RampUp => f.clamp(0.0, 1.0),
        Shape::RampDown => 1.0 - f.clamp(0.0, 1.0),
        Shape::Triangle => {
            if !(0.0..=1.0).contains(&f) {
                0.0
            } else if f <= 0.5 {
                f * 2.0
            } else {
                (1.0 - f) * 2.0
            }
        }
        Shape::Round => {
            if !(0.0..=1.0).contains(&f) {
                0.0
            } else {
                let x = f * 2.0 - 1.0;
                (1.0 - x * x).max(0.0).sqrt()
            }
        }
        Shape::Smooth => {
            if !(0.0..=1.0).contains(&f) {
                0.0
            } else {
                0.5 - 0.5 * (f * std::f64::consts::PI).cos()
            }
        }
    }
}

fn apply_ease(v: f64, high: f64, low: f64) -> f64 {
    let mut out = v;
    if high != 0.0 && v > 0.5 {
        let t = (v - 0.5) * 2.0;
        let p = if high > 0.0 { 1.0 + high * 2.0 } else { 1.0 / (1.0 - high * 2.0) };
        out = 0.5 + 0.5 * t.powf(p);
    }
    if low != 0.0 && v < 0.5 {
        let t = v * 2.0;
        let p = if low > 0.0 { 1.0 + low * 2.0 } else { 1.0 / (1.0 - low * 2.0) };
        out = 0.5 * t.powf(p);
    }
    out
}

fn pseudo_permute(idx: usize, total: usize, seed: u32) -> usize {
    let prime = 2654435761u64;
    let hash = (idx as u64 ^ seed as u64).wrapping_mul(prime);
    (hash % total as u64) as usize
}

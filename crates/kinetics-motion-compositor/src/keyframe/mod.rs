//! Keyframe structures, temporal ease velocity graphs, and spatial paths.

pub mod bezier;
pub mod spatial;

use serde::{Deserialize, Serialize};

use crate::keyframe::bezier::ease_progress;
use crate::keyframe::spatial::ArcLengthTable;
use crate::time::Tick;

/// Interpolation mode on one side of a keyframe.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Interp {
    #[default]
    Linear,
    Bezier,
    Hold,
}

/// Keyframe temporal ease (speed and influence).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ease {
    pub speed: f64,
    pub influence: f64,
}

impl Default for Ease {
    fn default() -> Self {
        Self {
            speed: 0.0,
            influence: 1.0 / 6.0,
        }
    }
}

impl Ease {
    pub const EASY: Self = Self {
        speed: 0.0,
        influence: 1.0 / 3.0,
    };
}

/// An animated multi-dimensional value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Float(f64),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Vec4([f64; 4]),
}

impl Value {
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::Float(f) => *f,
            Value::Vec2(v) => v[0],
            Value::Vec3(v) => v[0],
            Value::Vec4(v) => v[0],
        }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => Value::Float(a + (b - a) * t),
            (Value::Vec2(a), Value::Vec2(b)) => {
                Value::Vec2([a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t])
            }
            (Value::Vec3(a), Value::Vec3(b)) => Value::Vec3([
                a[0] + (b[0] - a[0]) * t,
                a[1] + (b[1] - a[1]) * t,
                a[2] + (b[2] - a[2]) * t,
            ]),
            (Value::Vec4(a), Value::Vec4(b)) => Value::Vec4([
                a[0] + (b[0] - a[0]) * t,
                a[1] + (b[1] - a[1]) * t,
                a[2] + (b[2] - a[2]) * t,
                a[3] + (b[3] - a[3]) * t,
            ]),
            _ => self.clone(),
        }
    }
}

/// A keyframe on a timeline property.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Keyframe {
    pub time: Tick,
    pub value: Value,
    pub in_interp: Interp,
    pub out_interp: Interp,
    pub in_ease: Vec<Ease>,
    pub out_ease: Vec<Ease>,
    pub continuous: bool,
    pub auto_bezier: bool,
    pub spatial_in: [f64; 3],
    pub spatial_out: [f64; 3],
    pub roving: bool,
}

impl Keyframe {
    pub fn new(time: Tick, value: Value) -> Self {
        Self {
            time,
            value,
            in_interp: Interp::Linear,
            out_interp: Interp::Linear,
            in_ease: Vec::new(),
            out_ease: Vec::new(),
            continuous: false,
            auto_bezier: false,
            spatial_in: [0.0; 3],
            spatial_out: [0.0; 3],
            roving: false,
        }
    }

    pub fn hold(mut self) -> Self {
        self.in_interp = Interp::Hold;
        self.out_interp = Interp::Hold;
        self
    }

    pub fn eased(mut self) -> Self {
        self.in_interp = Interp::Bezier;
        self.out_interp = Interp::Bezier;
        self.in_ease = vec![Ease::EASY];
        self.out_ease = vec![Ease::EASY];
        self
    }
}

/// An animated keyframe track.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct KeyframeTrack {
    pub keyframes: Vec<Keyframe>,
}

impl KeyframeTrack {
    pub fn evaluate(&self, t: Tick) -> Option<Value> {
        if self.keyframes.is_empty() {
            return None;
        }
        if t <= self.keyframes[0].time {
            return Some(self.keyframes[0].value.clone());
        }
        let last_idx = self.keyframes.len() - 1;
        if t >= self.keyframes[last_idx].time {
            return Some(self.keyframes[last_idx].value.clone());
        }

        // Find surrounding keyframes
        let mut idx = 0;
        while idx < last_idx && self.keyframes[idx + 1].time <= t {
            idx += 1;
        }

        let k0 = &self.keyframes[idx];
        let k1 = &self.keyframes[idx + 1];

        if k0.out_interp == Interp::Hold {
            return Some(k0.value.clone());
        }

        let dt = (k1.time - k0.time).as_seconds();
        if dt <= 0.0 {
            return Some(k0.value.clone());
        }
        let local_t = (t - k0.time).as_seconds() / dt;

        let progress = if k0.out_interp == Interp::Bezier || k1.in_interp == Interp::Bezier {
            let out_ease = k0.out_ease.first().copied().unwrap_or_default();
            let in_ease = k1.in_ease.first().copied().unwrap_or_default();
            ease_progress(out_ease.speed, out_ease.influence, in_ease.speed, in_ease.influence, local_t)
        } else {
            local_t
        };

        // If spatial 3D property
        if let (Value::Vec3(p0), Value::Vec3(p3)) = (&k0.value, &k1.value) {
            let p1 = [p0[0] + k0.spatial_out[0], p0[1] + k0.spatial_out[1], p0[2] + k0.spatial_out[2]];
            let p2 = [p3[0] + k1.spatial_in[0], p3[1] + k1.spatial_in[1], p3[2] + k1.spatial_in[2]];
            let table = ArcLengthTable::build(*p0, p1, p2, *p3, 32);
            let u = table.u_at_progress(progress);
            let pt = spatial::eval_spatial_bezier(*p0, p1, p2, *p3, u);
            return Some(Value::Vec3(pt));
        }

        Some(k0.value.lerp(&k1.value, progress))
    }
}

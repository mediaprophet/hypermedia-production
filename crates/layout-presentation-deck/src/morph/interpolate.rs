//! Transform and geometry interpolation for morphing.

use crate::model::types::{Rgba, Xfrm};

pub fn clamp(val: f64, min: f64, max: f64) -> f64 {
    if val.is_nan() {
        min
    } else if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    let v = a + (b - a) * t;
    if v.is_finite() {
        v
    } else if t < 0.5 {
        a
    } else {
        b
    }
}

pub fn lerp_rgba(a: Rgba, b: Rgba, t: f64) -> Rgba {
    let t = clamp(t, 0.0, 1.0) as f32;
    Rgba {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

/// Interpolate a box from `a` to `b` at `t` (0..1):
/// - Position and size linearly
/// - Rotation along the shortest way round (-180 to 180 degrees difference)
/// - Flips switch at midpoint (t >= 0.5)
pub fn morph_xfrm(a: Xfrm, b: Xfrm, t: f64) -> Xfrm {
    let t = clamp(t, 0.0, 1.0);
    if t >= 1.0 {
        return b;
    }
    if t <= 0.0 {
        return a;
    }

    let ra = if a.rot.is_finite() { a.rot } else { 0.0 };
    let rb = if b.rot.is_finite() { b.rot } else { 0.0 };
    let mut d = (rb - ra).rem_euclid(360.0);
    if d > 180.0 {
        d -= 360.0;
    }
    let rot = (ra + d * t).rem_euclid(360.0);
    let late = t >= 0.5;

    Xfrm {
        x: lerp(a.x, b.x, t),
        y: lerp(a.y, b.y, t),
        w: lerp(a.w, b.w, t),
        h: lerp(a.h, b.h, t),
        rot: if rot.is_finite() { rot } else { 0.0 },
        flip_h: if late { b.flip_h } else { a.flip_h },
        flip_v: if late { b.flip_v } else { a.flip_v },
    }
}

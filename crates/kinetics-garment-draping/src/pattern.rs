//! 2D garment pattern geometry: Bézier boundary curves, seams, notches, darts, and avatar landmarks.
//! Grounded in ergonomic anthropometry and informatics fiduciary guarantees.

use crate::math::{DVec2, DVec3};
use serde::{Deserialize, Serialize};

/// Cubic Bézier curve evaluator.
pub fn eval_cubic_bezier(p: [DVec2; 4], t: f64) -> DVec2 {
    let t = t.clamp(0.0, 1.0);
    let a = p[0].lerp(p[1], t);
    let b = p[1].lerp(p[2], t);
    let c = p[2].lerp(p[3], t);
    a.lerp(b, t).lerp(b.lerp(c, t), t)
}

/// Adaptive subdivision of a cubic Bézier curve within given tolerance in meters.
pub fn flatten_cubic_bezier(p: [DVec2; 4], tolerance: f64) -> Vec<DVec2> {
    let mut out = vec![p[0]];
    fn split(p: [DVec2; 4], tol: f64, depth: u32, out: &mut Vec<DVec2>) {
        let chord = p[3] - p[0];
        let d = chord.length();
        let err = if d > 1e-12 {
            let n = DVec2::new(-chord.y, chord.x) / d;
            ((p[1] - p[0]).dot(n)).abs().max(((p[2] - p[0]).dot(n)).abs())
        } else {
            (p[1] - p[0]).length().max((p[2] - p[0]).length())
        };

        if err <= tol || depth >= 16 {
            out.push(p[3]);
            return;
        }

        let a = (p[0] + p[1]) * 0.5;
        let b = (p[1] + p[2]) * 0.5;
        let c = (p[2] + p[3]) * 0.5;
        let d = (a + b) * 0.5;
        let e = (b + c) * 0.5;
        let f = (d + e) * 0.5;

        split([p[0], a, d, f], tol, depth + 1, out);
        split([f, e, c, p[3]], tol, depth + 1, out);
    }

    split(p, tolerance.max(1e-5), 0, &mut out);
    out
}

/// Evaluates a point along an arc-length polyline at a normalized fraction in [0, 1].
pub fn polyline_arclength_point(points: &[DVec2], fraction: f64) -> DVec2 {
    if points.is_empty() {
        return DVec2::ZERO;
    }
    if points.len() == 1 {
        return points[0];
    }

    let total_len: f64 = points.windows(2).map(|w| w[0].distance(w[1])).sum();
    let target = total_len * fraction.clamp(0.0, 1.0);
    let mut accum = 0.0;

    for w in points.windows(2) {
        let seg_len = w[0].distance(w[1]);
        if accum + seg_len >= target && seg_len > 1e-12 {
            let t = (target - accum) / seg_len;
            return w[0].lerp(w[1], t);
        }
        accum += seg_len;
    }

    *points.last().unwrap_or(&DVec2::ZERO)
}

/// A seam connecting two panel edges with an elastic penalty coefficient.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeamCoupling {
    pub id: String,
    pub panel_a: String,
    pub panel_b: String,
    pub stiffness: f64,
    pub notches: Vec<[f64; 2]>,
}

/// Anthropometric body landmark.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvatarLandmark {
    pub id: String,
    pub name: String,
    pub position_m: DVec3,
}

/// Set of body measurements computed from landmarks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyMeasurements {
    pub height_m: f64,
    pub chest_circumference_m: f64,
    pub waist_circumference_m: f64,
    pub hip_circumference_m: f64,
    pub inseam_m: f64,
}

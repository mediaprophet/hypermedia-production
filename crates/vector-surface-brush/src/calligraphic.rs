//! Calligraphic brush: elliptical nib swept along paths with support points and variation.

use kurbo::{BezPath, Point, Vec2};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::{Anchor, AnchorKind, PathData, SubPath};

use crate::track::{Rng, Track, normal, seed_of, tracks};
use crate::BrushInputSample;

const MAX_STEPS: f64 = 1024.0;

/// How a brush property varies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Variation {
    #[default]
    Fixed,
    Random,
    Pressure,
}

/// Calligraphic brush definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CalligraphicBrush {
    /// Nib angle in degrees, counter-clockwise from horizontal.
    pub angle: f64,
    /// Minor/major axis ratio in percent (1..=100).
    pub roundness: f64,
    /// Nib diameter in points.
    pub size: f64,
    /// Variation [angle, roundness, size].
    pub variation: [f64; 3],
    /// Modes for [angle, roundness, size].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<[Variation; 3]>,
}

impl Default for CalligraphicBrush {
    fn default() -> Self {
        Self {
            angle: 0.0,
            roundness: 100.0,
            size: 3.0,
            variation: [0.0; 3],
            modes: None,
        }
    }
}

impl CalligraphicBrush {
    pub const RANGES: [(f64, f64); 3] = [(-180.0, 180.0), (0.0, 100.0), (0.0, 1296.0)];

    pub fn modes(&self) -> [Variation; 3] {
        self.modes.unwrap_or_else(|| self.variation.map(|v| if v != 0.0 { Variation::Random } else { Variation::Fixed }))
    }

    pub fn uses_pressure(&self) -> bool {
        self.modes().contains(&Variation::Pressure)
    }

    pub fn expand(&self, bp: &BezPath, weight: f64, samples: Option<&[BrushInputSample]>, salt: &str) -> Vec<SubPath> {
        let mut rng = Rng::new(seed_of(bp, salt));
        let base = randomized(self, &mut rng);
        if self.uses_pressure() {
            let nib_fn = |f: f64| {
                let press = sample_pressure(samples, f);
                let [angle, round, size] = pressured(self, base, press);
                Nib::new(angle, round / 100.0, size * weight)
            };
            let max_s = pressured(self, base, 0.0)[2].max(pressured(self, base, 1.0)[2]) * weight;
            tracks(bp, tolerance(max_s))
                .iter()
                .flat_map(|t| sweep(t, &nib_fn, Some((t.len() / MAX_STEPS).max(0.25))))
                .collect()
        } else {
            let [angle, round, size] = base;
            let nib = Nib::new(angle, round / 100.0, size * weight);
            tracks(bp, tolerance(size * weight))
                .iter()
                .flat_map(|t| sweep(t, &|_| nib, None))
                .collect()
        }
    }
}

fn sample_pressure(samples: Option<&[BrushInputSample]>, f: f64) -> f64 {
    let Some(s) = samples else { return 0.5 };
    if s.is_empty() {
        return 0.5;
    }
    if s.len() == 1 {
        return s[0].pressure.clamp(0.0, 1.0);
    }
    let total_len = s.last().unwrap().distance.max(1e-6);
    let target_d = f * total_len;
    let idx = s.partition_point(|p| p.distance < target_d).saturating_sub(1);
    let next_idx = (idx + 1).min(s.len() - 1);
    if idx == next_idx {
        return s[idx].pressure.clamp(0.0, 1.0);
    }
    let seg = s[next_idx].distance - s[idx].distance;
    let t = if seg > 1e-9 { ((target_d - s[idx].distance) / seg).clamp(0.0, 1.0) } else { 0.0 };
    (s[idx].pressure + (s[next_idx].pressure - s[idx].pressure) * t).clamp(0.0, 1.0)
}

fn randomized(c: &CalligraphicBrush, rng: &mut Rng) -> [f64; 3] {
    let values = [c.angle, c.roundness, c.size];
    let mut out = values;
    for (i, (v, mode)) in out.iter_mut().zip(c.modes()).enumerate() {
        let (Some(&value), Some(&var), Some(&(lo, hi))) = (values.get(i), c.variation.get(i), CalligraphicBrush::RANGES.get(i)) else { continue };
        if mode == Variation::Random && var > 0.0 {
            let x = value + rng.range(-var, var);
            *v = if x.is_finite() { x.clamp(lo, hi) } else { value.clamp(lo, hi) };
        }
    }
    let [angle, round, size] = out;
    [angle, round.clamp(1.0, 100.0), size.max(0.05)]
}

fn pressured(c: &CalligraphicBrush, base: [f64; 3], p: f64) -> [f64; 3] {
    let values = [c.angle, c.roundness, c.size];
    let mut out = base;
    for (i, (v, mode)) in out.iter_mut().zip(c.modes()).enumerate() {
        let (Some(&value), Some(&var), Some(&(lo, hi))) = (values.get(i), c.variation.get(i), CalligraphicBrush::RANGES.get(i)) else { continue };
        if mode == Variation::Pressure {
            let x = value + var * (2.0 * p.clamp(0.0, 1.0) - 1.0);
            *v = if x.is_finite() { x.clamp(lo, hi) } else { value.clamp(lo, hi) };
        }
    }
    let [angle, round, size] = out;
    [angle, round.clamp(1.0, 100.0), size.max(0.05)]
}

fn tolerance(scale: f64) -> f64 {
    (0.02 * scale).clamp(0.01, 0.25)
}

/// Elliptical nib support representation.
#[derive(Clone, Copy, Debug)]
pub struct Nib {
    pub u: Vec2,
    pub v: Vec2,
    pub a: f64,
    pub b: f64,
}

impl Nib {
    pub fn new(angle_deg: f64, roundness: f64, size: f64) -> Self {
        let t = angle_deg.to_radians();
        let u = Vec2::new(t.cos(), -t.sin());
        let v = Vec2::new(-u.y, u.x);
        let a = (size / 2.0).max(1e-4);
        Nib { u, v, a, b: (a * roundness.clamp(0.01, 1.0)).max(1e-4) }
    }

    pub fn support(&self, d: Vec2) -> Vec2 {
        let (du, dv) = (self.u.dot(d), self.v.dot(d));
        let den = ((self.a * du).powi(2) + (self.b * dv).powi(2)).sqrt().max(1e-12);
        (self.u * (self.a * self.a * du) + self.v * (self.b * self.b * dv)) / den
    }
}

fn rot(base: Vec2, angle: f64) -> Vec2 {
    base * angle.cos() + normal(base) * angle.sin()
}

fn samples_along(t: &Track, step: Option<f64>) -> Vec<(Point, Vec2, f64)> {
    let mut out = vec![];
    let n = t.segments();
    let len = t.len().max(1e-12);
    for i in 0..=n {
        let p = t.verts[i];
        let f = t.cum[i] / len;
        if t.corner[i] {
            let (a, b) = t.in_out(i);
            let turn = a.cross(b).atan2(a.dot(b));
            let steps = (turn.abs() / 10f64.to_radians()).ceil().max(1.0) as usize;
            for k in 0..=steps {
                out.push((p, rot(a, turn * k as f64 / steps as f64), f));
            }
        } else {
            out.push((p, t.tangents[i], f));
        }
        if let Some(step) = step
            && i < n
        {
            let seg = t.cum[i + 1] - t.cum[i];
            let pieces = (seg / step).ceil().clamp(1.0, MAX_STEPS) as usize;
            for k in 1..pieces {
                let u = k as f64 / pieces as f64;
                out.push((p.lerp(t.verts[i + 1], u), t.dirs[i], (t.cum[i] + seg * u) / len));
            }
        }
    }
    out
}

pub fn sweep(t: &Track, nib: &dyn Fn(f64) -> Nib, step: Option<f64>) -> Vec<SubPath> {
    let s = samples_along(t, step);
    let left: Vec<Point> = s.iter().map(|(p, tg, f)| *p + nib(*f).support(normal(*tg))).collect();
    let right: Vec<Point> = s.iter().map(|(p, tg, f)| *p + nib(*f).support(-normal(*tg))).collect();
    if t.closed {
        let mut r = right;
        r.reverse();
        return vec![SubPath::polyline(&left, true), SubPath::polyline(&r, true)];
    }
    let cap = |(p, tg, f): (Point, Vec2, f64), from: f64, to: f64, out: &mut Vec<Point>| {
        let (steps, nib) = (16, nib(f));
        let norm = normal(tg);
        for i in 1..steps {
            let a = from + (to - from) * (i as f64 / steps as f64);
            out.push(p + nib.support(rot(norm, a)));
        }
    };
    let mut poly = left;
    cap(s[s.len() - 1], 0.0, -std::f64::consts::PI, &mut poly);
    poly.extend(right.into_iter().rev());
    cap(s[0], -std::f64::consts::PI, 0.0, &mut poly);
    vec![SubPath::polyline(&poly, true)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nib_support_axes() {
        let nib = Nib::new(0.0, 0.5, 10.0);
        let s_x = nib.support(Vec2::new(1.0, 0.0));
        assert!((s_x.x - 5.0).abs() < 1e-6);
        let s_y = nib.support(Vec2::new(0.0, 1.0));
        assert!((s_y.y - 2.5).abs() < 1e-6);
    }

    #[test]
    fn calligraphic_sweep() {
        let brush = CalligraphicBrush {
            angle: 45.0,
            roundness: 50.0,
            size: 10.0,
            ..Default::default()
        };
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((100.0, 0.0));
        let pieces = brush.expand(&bp, 1.0, None, "test");
        assert_eq!(pieces.len(), 1);
        let pd = PathData::single(pieces[0].clone());
        let b = pd.bounds().unwrap();
        assert!(b.width() > 100.0);
        assert!(b.height() > 5.0);
    }
}

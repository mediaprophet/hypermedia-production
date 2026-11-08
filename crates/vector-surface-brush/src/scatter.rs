//! Scatter brush: stamps copies of art along the path with randomized or pressure-modulated parameters.

use kurbo::{Affine, BezPath, Point, Rect, Vec2};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::PathData;

use crate::calligraphic::Variation;
use crate::track::{Rng, Track, normal, seed_of, tracks};
use crate::BrushInputSample;

/// Scatter brush definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScatterBrush {
    pub art: PathData,
    pub size: (f64, f64),
    pub spacing: (f64, f64),
    pub scatter: (f64, f64),
    pub rotation: (f64, f64),
    pub rotation_relative_to_path: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<[Variation; 4]>,
}

impl Default for ScatterBrush {
    fn default() -> Self {
        Self {
            art: default_dot_art(),
            size: (100.0, 100.0),
            spacing: (100.0, 100.0),
            scatter: (0.0, 0.0),
            rotation: (0.0, 0.0),
            rotation_relative_to_path: false,
            modes: None,
        }
    }
}

pub fn default_dot_art() -> PathData {
    vector_locus_geometry::shapes::ellipse(Rect::new(-3.0, -3.0, 3.0, 3.0))
}

impl ScatterBrush {
    pub const RANGES: [(f64, f64); 4] = [(1.0, 10000.0), (1.0, 10000.0), (-1000.0, 1000.0), (-180.0, 180.0)];

    pub fn modes(&self) -> [Variation; 4] {
        self.modes.unwrap_or_else(|| {
            [self.size, self.spacing, self.scatter, self.rotation].map(|(a, b)| if (a - b).abs() > 1e-6 { Variation::Random } else { Variation::Fixed })
        })
    }

    pub fn expand(&self, bp: &BezPath, weight: f64, samples: Option<&[BrushInputSample]>, salt: &str) -> Vec<PathData> {
        let mut rng = Rng::new(seed_of(bp, salt));
        let art_bounds = self.art.bounds().unwrap_or(Rect::new(-5.0, -5.0, 5.0, 5.0));
        let (art_w, art_h) = (art_bounds.width().max(1.0), art_bounds.height().max(1.0));
        let art_center = art_bounds.center();

        let mut out = Vec::new();
        for t in tracks(bp, 0.1) {
            let l = t.len();
            if l <= 1e-6 {
                continue;
            }
            let mut s = 0.0;
            while s <= l {
                let f = s / l;
                let pressure = sample_pressure(samples, f);
                let (scale_pct, spacing_pct, scatter_pct, rot_deg) = self.eval_params(pressure, &mut rng);
                let size_scale = (scale_pct / 100.0) * weight;

                let (pos, tg) = t.frame(s);
                let norm = normal(tg);
                let offset_dist = (scatter_pct / 100.0) * art_h * size_scale;
                let stamped_pos = pos + norm * offset_dist;

                let mut angle = rot_deg.to_radians();
                if self.rotation_relative_to_path {
                    angle += tg.y.atan2(tg.x);
                }

                let xform = Affine::translate(stamped_pos.to_vec2())
                    * Affine::rotate(angle)
                    * Affine::scale(size_scale)
                    * Affine::translate(-art_center.to_vec2());

                out.push(self.art.transformed(xform));

                let step = ((spacing_pct / 100.0) * art_w * size_scale).max(1.0);
                s += step;
            }
        }
        out
    }

    fn eval_params(&self, pressure: f64, rng: &mut Rng) -> (f64, f64, f64, f64) {
        let modes = self.modes();
        let mut eval_one = |(lo, hi): (f64, f64), mode: Variation| match mode {
            Variation::Fixed => lo,
            Variation::Random => rng.range(lo, hi),
            Variation::Pressure => lo + (hi - lo) * pressure.clamp(0.0, 1.0),
        };
        (
            eval_one(self.size, modes[0]),
            eval_one(self.spacing, modes[1]),
            eval_one(self.scatter, modes[2]),
            eval_one(self.rotation, modes[3]),
        )
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scatter_stamps() {
        let brush = ScatterBrush {
            spacing: (50.0, 50.0),
            ..Default::default()
        };
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((100.0, 0.0));
        let pieces = brush.expand(&bp, 1.0, None, "salt");
        assert!(pieces.len() >= 10);
    }
}

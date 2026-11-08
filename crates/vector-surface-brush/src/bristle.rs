//! Bristle brush: bundles of individual bristle strands simulated along the path.

use kurbo::{BezPath, Point};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::{PathData, SubPath};

use crate::calligraphic::{Nib, sweep};
use crate::track::{Rng, Track, normal, seed_of, tracks};

/// Tip shapes for bristle brushes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BristleShape {
    #[default]
    RoundPoint,
    FlatPoint,
    RoundBlunt,
    FlatBlunt,
    RoundCurve,
    FlatCurve,
    RoundAngle,
    FlatAngle,
    RoundFan,
    FlatFan,
}

/// A single rendered strand of a bristle brush.
#[derive(Clone, Debug, PartialEq)]
pub struct BristleStrand {
    pub path: PathData,
    pub opacity: f32,
}

/// Bristle brush definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BristleBrush {
    pub shape: BristleShape,
    /// Brush size in points.
    pub size: f64,
    /// Bristle length in percent (25..=300).
    pub length: f64,
    /// Density in percent (1..=100) — controls number of strands.
    pub density: f64,
    /// Thickness in percent (1..=100) — strand width.
    pub thickness: f64,
    /// Paint opacity in percent (1..=100).
    pub opacity: f64,
    /// Stiffness in percent (1..=100).
    pub stiffness: f64,
}

impl Default for BristleBrush {
    fn default() -> Self {
        Self {
            shape: BristleShape::RoundPoint,
            size: 6.0,
            length: 100.0,
            density: 50.0,
            thickness: 40.0,
            opacity: 75.0,
            stiffness: 50.0,
        }
    }
}

impl BristleBrush {
    pub fn expand(&self, bp: &BezPath, weight: f64, salt: &str) -> Vec<BristleStrand> {
        let mut rng = Rng::new(seed_of(bp, salt));
        let size = self.size.max(0.1) * weight;
        let strands = (3.0 + self.density.clamp(1.0, 100.0) / 100.0 * 13.0).round() as usize;
        let thick = (size * self.thickness.clamp(1.0, 100.0) / 100.0 * 0.45).max(0.15);
        let flat = matches!(
            self.shape,
            BristleShape::FlatPoint
                | BristleShape::FlatBlunt
                | BristleShape::FlatCurve
                | BristleShape::FlatAngle
                | BristleShape::FlatFan
        );
        let wobble = (100.0 - self.stiffness.clamp(1.0, 100.0)) / 100.0;
        let ragged = self.length.clamp(25.0, 300.0) / 100.0;
        let base_opacity = (self.opacity.clamp(1.0, 100.0) / 100.0) as f32;

        let mut out = Vec::new();
        for t in tracks(bp, 0.1) {
            let l = t.len();
            for k in 0..strands {
                let lane = (k as f64 + 0.5) / strands as f64 * 2.0 - 1.0;
                let off = if flat { lane } else { lane.signum() * lane.abs().powf(1.4) } * size / 2.0
                    + rng.range(-wobble, wobble) * size * 0.08;
                let edge = 1.0 - lane.abs();
                let trim = |r: &mut Rng| r.unit() * ragged * size * (1.2 - edge) * 0.8;
                let (t0, t1) = (trim(&mut rng), trim(&mut rng));
                let Some(sp) = offset_track(&t, off, t0.min(l * 0.3), t1.min(l * 0.3)) else { continue };
                let w = thick * (0.6 + 0.4 * edge) * rng.range(0.8, 1.2);
                let nib = Nib::new(0.0, 1.0, w);
                let subs: Vec<SubPath> = tracks(&sp, 0.05).iter().flat_map(|tt| sweep(tt, &|_| nib, None)).collect();
                if subs.is_empty() {
                    continue;
                }
                let strand_op = (base_opacity * rng.range(0.7, 1.0) as f32).clamp(0.0, 1.0);
                out.push(BristleStrand {
                    path: PathData::new(subs),
                    opacity: strand_op,
                });
            }
        }
        out
    }
}

fn offset_track(t: &Track, d: f64, trim0: f64, trim1: f64) -> Option<BezPath> {
    let l = t.len();
    let (s0, s1) = (trim0.clamp(0.0, l), (l - trim1).clamp(0.0, l));
    if s1 - s0 < 1e-6 {
        return None;
    }
    let mut bp = BezPath::new();
    let mut first = true;
    let mut push = |p: Point, bp: &mut BezPath| {
        if first {
            bp.move_to(p);
            first = false;
        } else {
            bp.line_to(p);
        }
    };
    push(t.map(s0, d), &mut bp);
    for (i, c) in t.cum.iter().enumerate() {
        if *c > s0 && *c < s1 {
            push(t.verts[i] + normal(t.tangents[i]) * d, &mut bp);
        }
    }
    push(t.map(s1, d), &mut bp);
    if t.closed && trim0 <= 0.0 && trim1 <= 0.0 {
        bp.close_path();
    }
    Some(bp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bristle_bundles() {
        let brush = BristleBrush::default();
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((100.0, 0.0));
        let strands = brush.expand(&bp, 1.0, "salt");
        assert!(!strands.is_empty());
        assert!(strands.len() >= 3);
        for s in strands {
            assert!(s.opacity > 0.0 && s.opacity <= 1.0);
            assert!(!s.path.is_empty());
        }
    }
}

//! Pattern brush: modular tiles repeated along stroke segments and corners.

use kurbo::{Affine, BezPath, Point, Rect, Vec2};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::PathData;

use crate::track::{Track, normal, tracks};

/// How tiles fit into segment lengths that are not a whole number of tiles long.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PatternFit {
    /// Stretch tiles to fit exactly.
    #[default]
    Stretch,
    /// Add extra space evenly between tiles.
    AddSpace,
    /// Round to nearest tile count.
    Approximate,
}

/// Pattern brush definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PatternBrush {
    pub side: PathData,
    pub outer_corner: Option<PathData>,
    pub inner_corner: Option<PathData>,
    pub start_cap: Option<PathData>,
    pub end_cap: Option<PathData>,
    pub scale: f64,
    pub spacing: f64,
    pub fit: PatternFit,
    pub flip_along: bool,
    pub flip_across: bool,
}

impl Default for PatternBrush {
    fn default() -> Self {
        Self {
            side: default_stitch_tile(),
            outer_corner: None,
            inner_corner: None,
            start_cap: None,
            end_cap: None,
            scale: 100.0,
            spacing: 0.0,
            fit: PatternFit::Stretch,
            flip_along: false,
            flip_across: false,
        }
    }
}

pub fn default_stitch_tile() -> PathData {
    vector_locus_geometry::shapes::rounded_rectangle(Rect::new(0.0, -2.0, 10.0, 2.0), 1.0)
}

impl PatternBrush {
    pub fn expand(&self, bp: &BezPath, weight: f64) -> Vec<PathData> {
        let side_bounds = self.side.bounds().unwrap_or(Rect::new(0.0, -5.0, 10.0, 5.0));
        let base_tile_w = side_bounds.width().max(1.0);
        let tile_scale = (self.scale / 100.0) * weight;
        let scaled_tile_w = base_tile_w * tile_scale;
        let spacing_len = (self.spacing / 100.0) * scaled_tile_w;
        let step_len = (scaled_tile_w + spacing_len).max(1.0);

        let mut out = Vec::new();
        for t in tracks(bp, 0.1) {
            let l = t.len();
            if l <= 1e-6 {
                continue;
            }

            let num_tiles = (l / step_len).round().max(1.0) as usize;
            let actual_step = match self.fit {
                PatternFit::Stretch | PatternFit::Approximate => l / num_tiles as f64,
                PatternFit::AddSpace => step_len,
            };

            for i in 0..num_tiles {
                let s = (i as f64 + 0.5) * actual_step;
                if s > l {
                    break;
                }
                let (pos, tg) = t.frame(s);
                let angle = tg.y.atan2(tg.x);

                let flip_x = if self.flip_along { -1.0 } else { 1.0 };
                let flip_y = if self.flip_across { -1.0 } else { 1.0 };

                let xform = Affine::translate(pos.to_vec2())
                    * Affine::rotate(angle)
                    * Affine::scale_non_uniform(tile_scale * flip_x, tile_scale * flip_y)
                    * Affine::translate(-side_bounds.center().to_vec2());

                out.push(self.side.transformed(xform));
            }

            // Optional start and end caps
            if !t.closed {
                if let Some(cap) = &self.start_cap {
                    let (pos, tg) = t.frame(0.0);
                    let angle = tg.y.atan2(tg.x);
                    let xform = Affine::translate(pos.to_vec2()) * Affine::rotate(angle) * Affine::scale(tile_scale);
                    out.push(cap.transformed(xform));
                }
                if let Some(cap) = &self.end_cap {
                    let (pos, tg) = t.frame(l);
                    let angle = tg.y.atan2(tg.x);
                    let xform = Affine::translate(pos.to_vec2()) * Affine::rotate(angle) * Affine::scale(tile_scale);
                    out.push(cap.transformed(xform));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_brush_repeats() {
        let brush = PatternBrush::default();
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((100.0, 0.0));
        let pieces = brush.expand(&bp, 1.0);
        assert!(!pieces.is_empty());
        assert!(pieces.len() >= 8);
    }
}

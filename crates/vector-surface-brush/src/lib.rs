//! Categorical surface brush procedural engines: calligraphic, scatter, art spine warp,
//! pattern, and bristle brushes.
#![forbid(unsafe_code)]

pub mod art;
pub mod bristle;
pub mod calligraphic;
pub mod pattern;
pub mod scatter;
pub mod track;

use kurbo::{BezPath, Point, Vec2};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::PathData;

pub use art::{ArtBrush, ArtScale, Direction};
pub use bristle::{BristleBrush, BristleShape, BristleStrand};
pub use calligraphic::{CalligraphicBrush, Nib, Variation};
pub use pattern::{PatternBrush, PatternFit};
pub use scatter::ScatterBrush;
pub use track::{Rng, Track, normal, seed_of, tracks};

/// A timestamped and pressure-attributed stylus or gesture sample along a stroke.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrushInputSample {
    pub point: Point,
    pub pressure: f64,
    pub tilt_x: f64,
    pub tilt_y: f64,
    pub distance: f64,
    pub time_ms: f64,
}

impl Default for BrushInputSample {
    fn default() -> Self {
        Self {
            point: Point::ZERO,
            pressure: 1.0,
            tilt_x: 0.0,
            tilt_y: 0.0,
            distance: 0.0,
            time_ms: 0.0,
        }
    }
}

/// A rendered output piece from an expanded brush stroke.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrushedPiece {
    pub path: PathData,
    pub color_rgba: [f32; 4],
    pub opacity: f32,
}

/// Unified enumeration of all procedural brush engines.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Brush {
    Calligraphic(CalligraphicBrush),
    Scatter(ScatterBrush),
    Art(ArtBrush),
    Pattern(PatternBrush),
    Bristle(BristleBrush),
}

/// Expands any procedural brush stroke into vector geometric pieces.
pub fn expand_brush_stroke(
    brush: &Brush,
    path: &BezPath,
    weight: f64,
    samples: Option<&[BrushInputSample]>,
) -> Vec<BrushedPiece> {
    let w = if weight > 0.0 { weight } else { 1.0 };
    match brush {
        Brush::Calligraphic(c) => {
            let subpaths = c.expand(path, w, samples, "calli");
            if subpaths.is_empty() {
                Vec::new()
            } else {
                vec![BrushedPiece {
                    path: PathData::new(subpaths),
                    color_rgba: [0.0, 0.0, 0.0, 1.0],
                    opacity: 1.0,
                }]
            }
        }
        Brush::Scatter(s) => s
            .expand(path, w, samples, "scatter")
            .into_iter()
            .map(|p| BrushedPiece {
                path: p,
                color_rgba: [0.0, 0.0, 0.0, 1.0],
                opacity: 1.0,
            })
            .collect(),
        Brush::Art(a) => a
            .expand(path, w)
            .into_iter()
            .map(|p| BrushedPiece {
                path: p,
                color_rgba: [0.0, 0.0, 0.0, 1.0],
                opacity: 1.0,
            })
            .collect(),
        Brush::Pattern(p) => p
            .expand(path, w)
            .into_iter()
            .map(|path_data| BrushedPiece {
                path: path_data,
                color_rgba: [0.0, 0.0, 0.0, 1.0],
                opacity: 1.0,
            })
            .collect(),
        Brush::Bristle(b) => b
            .expand(path, w, "bristle")
            .into_iter()
            .map(|strand| BrushedPiece {
                path: strand.path,
                color_rgba: [0.0, 0.0, 0.0, 1.0],
                opacity: strand.opacity,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_line() -> BezPath {
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((100.0, 0.0));
        bp
    }

    #[test]
    fn expand_all_five_brush_types() {
        let line = test_line();

        // 1. Calligraphic
        let c = Brush::Calligraphic(CalligraphicBrush {
            size: 8.0,
            ..Default::default()
        });
        let out_c = expand_brush_stroke(&c, &line, 1.0, None);
        assert_eq!(out_c.len(), 1);

        // 2. Scatter
        let s = Brush::Scatter(ScatterBrush {
            spacing: (50.0, 50.0),
            ..Default::default()
        });
        let out_s = expand_brush_stroke(&s, &line, 1.0, None);
        assert!(!out_s.is_empty());

        // 3. Art
        let a = Brush::Art(ArtBrush::default());
        let out_a = expand_brush_stroke(&a, &line, 1.0, None);
        assert_eq!(out_a.len(), 1);

        // 4. Pattern
        let p = Brush::Pattern(PatternBrush::default());
        let out_p = expand_brush_stroke(&p, &line, 1.0, None);
        assert!(!out_p.is_empty());

        // 5. Bristle
        let b = Brush::Bristle(BristleBrush::default());
        let out_b = expand_brush_stroke(&b, &line, 1.0, None);
        assert!(out_b.len() >= 3);
        assert!(out_b.iter().all(|strand| strand.opacity > 0.0));
    }
}

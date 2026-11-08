//! Art brush: vector art bent along a curved path spine by arc length.

use std::f64::consts::FRAC_PI_2;

use kurbo::{Affine, BezPath, PathEl, Point, Rect, Vec2};
use serde::{Deserialize, Serialize};
use vector_locus_geometry::{PathData, SubPath};

use crate::track::{Track, tracks};

/// The direction art runs in the brush definition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// How art scales along the stroke.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "camelCase")]
pub enum ArtScale {
    Proportional,
    Stretch,
    StretchBetweenGuides { start: f64, end: f64 },
}

impl Default for ArtScale {
    fn default() -> Self {
        ArtScale::Stretch
    }
}

/// Art brush definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtBrush {
    pub art: PathData,
    pub direction: Direction,
    pub scale: ArtScale,
    pub flip_along: bool,
    pub flip_across: bool,
    pub width_ratio: f64,
}

impl Default for ArtBrush {
    fn default() -> Self {
        Self {
            art: default_arrow_art(),
            direction: Direction::LeftToRight,
            scale: ArtScale::Stretch,
            flip_along: false,
            flip_across: false,
            width_ratio: 100.0,
        }
    }
}

pub fn default_arrow_art() -> PathData {
    let mut sp = SubPath::polyline(
        &[
            Point::new(0.0, -10.0),
            Point::new(70.0, -10.0),
            Point::new(70.0, -25.0),
            Point::new(100.0, 0.0),
            Point::new(70.0, 25.0),
            Point::new(70.0, 10.0),
            Point::new(0.0, 10.0),
        ],
        true,
    );
    PathData::single(sp)
}

impl ArtBrush {
    pub fn expand(&self, bp: &BezPath, weight: f64) -> Vec<PathData> {
        let (oriented_art, b) = self.orient_art();
        let (art_w, art_h) = (b.width().max(1.0), b.height().max(1.0));
        let art_mid_y = (b.y0 + b.y1) / 2.0;

        let mut out = Vec::new();
        for t in tracks(bp, 0.1) {
            let path_len = t.len();
            if path_len <= 1e-6 {
                continue;
            }

            let (along_scale, cross_scale) = match self.scale {
                ArtScale::Proportional => {
                    let s = path_len / art_w;
                    (s, s * (self.width_ratio / 100.0) * weight)
                }
                ArtScale::Stretch => {
                    let s_along = path_len / art_w;
                    let s_cross = (self.width_ratio / 100.0) * weight;
                    (s_along, s_cross)
                }
                ArtScale::StretchBetweenGuides { start, end } => {
                    let middle = (end - start).max(1.0);
                    let non_stretching = art_w - middle;
                    let s_along = if path_len > non_stretching { (path_len - non_stretching) / middle } else { 1.0 };
                    let s_cross = (self.width_ratio / 100.0) * weight;
                    (s_along, s_cross)
                }
            };

            let warp_pt = |p: Point| -> Point {
                let u = match self.scale {
                    ArtScale::StretchBetweenGuides { start, end } => {
                        let x = p.x - b.x0;
                        if x < start {
                            x
                        } else if x <= end {
                            start + (x - start) * along_scale
                        } else {
                            start + (end - start) * along_scale + (x - end)
                        }
                    }
                    _ => (p.x - b.x0) * along_scale,
                };
                let v = (p.y - art_mid_y) * cross_scale;
                t.map(u, v)
            };

            let warped = warp_path_geometry(&oriented_art, &warp_pt);
            out.push(warped);
        }
        out
    }

    fn orient_art(&self) -> (PathData, Rect) {
        let rot = match self.direction {
            Direction::LeftToRight => Affine::IDENTITY,
            Direction::RightToLeft => Affine::scale_non_uniform(-1.0, 1.0),
            Direction::TopToBottom => Affine::rotate(-FRAC_PI_2),
            Direction::BottomToTop => Affine::rotate(FRAC_PI_2),
        };
        let flip_x = if self.flip_along { -1.0 } else { 1.0 };
        let flip_y = if self.flip_across { -1.0 } else { 1.0 };
        let xform = Affine::scale_non_uniform(flip_x, flip_y) * rot;

        let oriented = self.art.transformed(xform);
        let b = oriented.bounds().unwrap_or(Rect::new(0.0, -10.0, 100.0, 10.0));
        (oriented, b)
    }
}

fn warp_path_geometry(p: &PathData, f: &dyn Fn(Point) -> Point) -> PathData {
    let bp = p.to_bezpath();
    let mut subs = Vec::new();
    let mut cur: Vec<Point> = Vec::new();
    let mut els = Vec::new();
    kurbo::flatten(bp.iter(), 0.05, |e| els.push(e));

    let finish = |cur: &mut Vec<Point>, closed: bool, subs: &mut Vec<SubPath>| {
        let pts = std::mem::take(cur);
        if pts.len() < 2 {
            return;
        }
        let mapped: Vec<Point> = pts.into_iter().map(f).collect();
        subs.push(SubPath::polyline(&mapped, closed));
    };

    for e in els {
        match e {
            PathEl::MoveTo(q) => {
                finish(&mut cur, false, &mut subs);
                cur.push(q);
            }
            PathEl::LineTo(q) => cur.push(q),
            PathEl::ClosePath => {
                if cur.len() > 1 && cur[0].distance(cur[cur.len() - 1]) < 1e-9 {
                    cur.pop();
                }
                finish(&mut cur, true, &mut subs);
            }
            _ => {}
        }
    }
    finish(&mut cur, false, &mut subs);
    PathData::new(subs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn art_brush_warps_along_curve() {
        let brush = ArtBrush::default();
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.curve_to((50.0, 50.0), (100.0, -50.0), (150.0, 0.0));
        let pieces = brush.expand(&bp, 1.0);
        assert_eq!(pieces.len(), 1);
        let b = pieces[0].bounds().unwrap();
        assert!(b.width() > 100.0);
    }
}

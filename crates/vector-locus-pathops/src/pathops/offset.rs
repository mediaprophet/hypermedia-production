//! Offset path and outline stroke geometry operations.
//!
//! Grounded in authoring rights and creative preservation (UDHR Art. 19, 27).

use kurbo::{BezPath, PathEl, Stroke, StrokeOpts};
use vector_locus_geometry::{FillRule, PathData, SubPath};

pub use kurbo::{Cap, Join};

use crate::pathops::boolean::{boolean, rings_to_path, BoolOp, DEFAULT_PRECISION};

const STROKE_TOL: f64 = 1e-3;

fn close_all(bp: &BezPath) -> BezPath {
    let mut out = BezPath::new();
    let mut open = false;
    for el in bp.iter() {
        match el {
            PathEl::MoveTo(_) => {
                if open {
                    out.close_path();
                }
                open = true;
            }
            PathEl::ClosePath => open = false,
            _ => {}
        }
        out.push(el);
    }
    if open {
        out.close_path();
    }
    out
}

fn stroke_bez(bp: &BezPath, width: f64, cap: Cap, join: Join, miter_limit: f64) -> BezPath {
    let style = Stroke::new(width).with_join(join).with_caps(cap).with_miter_limit(miter_limit.max(1.0));
    close_all(&kurbo::stroke(bp.iter(), &style, &StrokeOpts::default(), STROKE_TOL))
}

/// Outline stroke: generate the filled compound path representing the painted stroke of `path`.
pub fn outline_stroke(path: &PathData, width: f64, cap: Cap, join: Join, miter_limit: f64) -> PathData {
    if width <= 0.0 || !width.is_finite() || path.is_empty() {
        return PathData::default();
    }
    let stroked = stroke_bez(&path.to_bezpath(), width, cap, join, miter_limit);
    PathData::from_bezpath(&stroked)
}

/// Offset path: expands (positive) or contracts (negative) a closed path by `offset`.
pub fn offset_path(path: &PathData, offset: f64, join: Join, miter_limit: f64) -> PathData {
    if offset == 0.0 || path.is_empty() {
        return path.clone();
    }
    let width = offset.abs() * 2.0;
    let outlined = outline_stroke(path, width, Cap::Round, join, miter_limit);
    if offset > 0.0 {
        boolean(path, &outlined, BoolOp::Union)
    } else {
        boolean(path, &outlined, BoolOp::Difference)
    }
}

/// Area covered by multiple stroke outlines as a single clean compound path.
pub fn stroke_region(outlines: &[BezPath]) -> PathData {
    if outlines.is_empty() {
        return PathData::default();
    }
    let mut acc = PathData::from_bezpath(&outlines[0]);
    for o in &outlines[1..] {
        let p = PathData::from_bezpath(o);
        acc = boolean(&acc, &p, BoolOp::Union);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use kurbo::Rect;
    use vector_locus_geometry::shapes;

    #[test]
    fn outline_stroke_expands_line() {
        let l = shapes::line(kurbo::Point::new(0.0, 0.0), kurbo::Point::new(100.0, 0.0));
        let stroked = outline_stroke(&l, 10.0, Cap::Butt, Join::Miter, 4.0);
        assert!(!stroked.is_empty());
        let b = stroked.bounds().unwrap();
        assert!(b.height() >= 9.9 && b.height() <= 10.1);
    }

    #[test]
    fn offset_path_inflates_rectangle() {
        let r = shapes::rectangle(Rect::new(10.0, 10.0, 50.0, 50.0));
        let expanded = offset_path(&r, 5.0, Join::Round, 4.0);
        assert!(!expanded.is_empty());
        let b = expanded.bounds().unwrap();
        assert!(b.width() > 40.0);
    }
}

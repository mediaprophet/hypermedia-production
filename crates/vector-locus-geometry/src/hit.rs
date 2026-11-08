//! Hit testing primitives for vector paths and outlines.

use std::borrow::Cow;

use kurbo::{BezPath, ParamCurveNearest, PathEl, Point, Rect, Shape};

use crate::path::{FillRule, PathData};

/// Is `p` inside the filled area of `path` under `rule`? Open subpaths are implicitly closed.
pub fn fill_contains(path: &BezPath, rule: FillRule, p: Point) -> bool {
    let w = closed(path).winding(p);
    match rule {
        FillRule::NonZero => w != 0,
        FillRule::EvenOdd => w % 2 != 0,
    }
}

/// `path` with each open subpath closed by a straight line back to its start.
fn closed(path: &BezPath) -> Cow<'_, BezPath> {
    let mut open = false;
    let mut needs = false;
    for el in path.elements() {
        match el {
            PathEl::MoveTo(_) => {
                needs |= open;
                open = true;
            }
            PathEl::ClosePath => open = false,
            _ => {}
        }
    }
    if !(needs || open) {
        return Cow::Borrowed(path);
    }
    let mut out = BezPath::new();
    let mut open = false;
    for &el in path.elements() {
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
    Cow::Owned(out)
}

/// Distance from `p` to the nearest point on the path outline.
pub fn distance_to_outline(path: &BezPath, p: Point) -> f64 {
    path.segments().map(|s| s.nearest(p, 1e-9).distance_sq).fold(f64::INFINITY, f64::min).sqrt()
}

/// Is `p` within `tol` of the path's stroke of width `width`?
pub fn stroke_contains(path: &BezPath, width: f64, tol: f64, p: Point) -> bool {
    let quick = path.bounding_box().inflate(width / 2.0 + tol, width / 2.0 + tol);
    quick.contains(p) && distance_to_outline(path, p) <= width / 2.0 + tol
}

/// Does the path intersect (or lie inside) the rect?
pub fn intersects_rect(path: &PathData, r: Rect) -> bool {
    let Some(b) = path.bounds() else { return false };
    if b.intersect(r).area() <= 0.0 && !(b.width() == 0.0 || b.height() == 0.0) {
        return false;
    }
    if path.anchors().any(|(_, _, a)| r.contains(a.p)) {
        return true;
    }
    let edges = [
        kurbo::Line::new((r.x0, r.y0), (r.x1, r.y0)),
        kurbo::Line::new((r.x1, r.y0), (r.x1, r.y1)),
        kurbo::Line::new((r.x1, r.y1), (r.x0, r.y1)),
        kurbo::Line::new((r.x0, r.y1), (r.x0, r.y0)),
    ];
    let bp = path.to_bezpath();
    for seg in bp.segments() {
        for e in &edges {
            if !seg.intersect_line(*e).is_empty() {
                return true;
            }
        }
    }
    path.is_closed() && fill_contains(&bp, FillRule::NonZero, r.center())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes;

    #[test]
    fn fill_hit() {
        let sq = shapes::rectangle(Rect::new(0.0, 0.0, 100.0, 100.0)).to_bezpath();
        assert!(fill_contains(&sq, FillRule::NonZero, Point::new(50.0, 50.0)));
        assert!(!fill_contains(&sq, FillRule::NonZero, Point::new(150.0, 50.0)));
    }

    #[test]
    fn stroke_hit() {
        let line = shapes::line(Point::new(0.0, 0.0), Point::new(100.0, 0.0)).to_bezpath();
        assert!(stroke_contains(&line, 4.0, 1.0, Point::new(50.0, 2.0)));
        assert!(!stroke_contains(&line, 4.0, 1.0, Point::new(50.0, 5.0)));
    }

    #[test]
    fn rect_intersection() {
        let sq = shapes::rectangle(Rect::new(0.0, 0.0, 100.0, 100.0));
        assert!(intersects_rect(&sq, Rect::new(50.0, 50.0, 150.0, 150.0)));
        assert!(!intersects_rect(&sq, Rect::new(200.0, 200.0, 300.0, 300.0)));
    }
}

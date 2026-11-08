//! Categorical locus vector geometry kernel.
//!
//! The editable path model is anchor based: a [`PathData`] is a list of [`SubPath`]s,
//! each a list of [`Anchor`]s with absolute in/out handle positions.
//!
//! Coordinates are document points (1/72 in), y pointing down.
#![forbid(unsafe_code)]

pub mod arc;
pub mod bez;
pub mod corners;
pub mod hit;
pub mod path;
pub mod perspective;
pub mod projective;
pub mod recognize;
pub mod shapes;
pub mod snap;
pub mod units;

pub use arc::ArcPath;
pub use kurbo;
pub use kurbo::{Affine, BezPath, CubicBez, Line, ParamCurve, PathEl, PathSeg, Point, Rect, Shape, Size, Vec2};
pub use path::{Anchor, AnchorKind, FillRule, PathData, SubPath};
pub use perspective::{Axis as PerspectiveAxis, PerspectiveGrid, Plane};
pub use projective::Homography;
pub use recognize::{Recognized, recognize, recognize_shape, simplify, simplify_freehand};
pub use shapes::{
    CornerKind, CornerKind as CornerStyle, KAPPA, arc, ellipse, ellipse_pie, fitted_corner_radius, line, max_corner_radius, polar_grid,
    polygon, rectangle, rectangle_with_corners, rectangular_grid, rounded_rectangle, spiral, star, straight_star_inner_radius,
};
pub use snap::{Axis as SnapAxis, closest, snap_point, snap_point_to_grid, snap_to_grid};
pub use units::Unit;

/// Type alias for [`PathData`].
pub type Path = PathData;

/// Type alias for [`SubPath`].
pub type Subpath = SubPath;

/// Live Corners alias for [`corners::cut_corners`].
pub use corners::{Corner, cut_corners, cut_corners as apply_corners, path_corners};

/// Tolerance used when comparing handle positions to anchor positions.
pub const EPS: f64 = 1e-9;

/// Union of two optional rects.
pub fn union_opt(a: Option<Rect>, b: Option<Rect>) -> Option<Rect> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.union(b)),
        (a, None) => a,
        (None, b) => b,
    }
}

/// Normalize an angle in degrees to (-180, 180].
pub fn normalize_deg(mut a: f64) -> f64 {
    a %= 360.0;
    if a <= -180.0 {
        a += 360.0;
    } else if a > 180.0 {
        a -= 360.0;
    }
    a
}

/// Snap a vector to the nearest multiple of `step_deg`.
pub fn constrain_angle(v: Vec2, step_deg: f64) -> Vec2 {
    constrain_angle_from(v, step_deg, 0.0)
}

/// [`constrain_angle`] in steps counted from `base_deg`.
pub fn constrain_angle_from(v: Vec2, step_deg: f64, base_deg: f64) -> Vec2 {
    let len = v.hypot();
    if len < EPS {
        return v;
    }
    let (step, base) = (step_deg.to_radians(), -base_deg.to_radians());
    let a = base + ((v.y.atan2(v.x) - base) / step).round() * step;
    Vec2::new(a.cos() * len, a.sin() * len)
}

/// Rect from two corner points in any order.
pub fn rect_from_points(a: Point, b: Point) -> Rect {
    Rect::from_points(a, b)
}

/// The 9 reference points of a rect (row-major from top-left).
pub fn reference_point(r: Rect, index: usize) -> Point {
    let xs = [r.x0, r.center().x, r.x1];
    let ys = [r.y0, r.center().y, r.y1];
    Point::new(xs[index % 3], ys[(index / 3).min(2)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_normalization() {
        assert_eq!(normalize_deg(190.0), -170.0);
        assert_eq!(normalize_deg(-190.0), 170.0);
        assert_eq!(normalize_deg(180.0), 180.0);
        assert_eq!(normalize_deg(720.0), 0.0);
    }

    #[test]
    fn constrain_45() {
        let v = constrain_angle(Vec2::new(10.0, 1.0), 45.0);
        assert!((v.y).abs() < 1e-9);
        let v = constrain_angle(Vec2::new(10.0, 9.0), 45.0);
        assert!((v.x - v.y).abs() < 1e-9);
    }

    #[test]
    fn reference_points() {
        let r = Rect::new(0.0, 0.0, 10.0, 20.0);
        assert_eq!(reference_point(r, 0), Point::new(0.0, 0.0));
        assert_eq!(reference_point(r, 4), Point::new(5.0, 10.0));
        assert_eq!(reference_point(r, 8), Point::new(10.0, 20.0));
    }

    #[test]
    fn union_options() {
        let a = Rect::new(0.0, 0.0, 1.0, 1.0);
        let b = Rect::new(2.0, 2.0, 3.0, 3.0);
        assert_eq!(union_opt(Some(a), Some(b)), Some(Rect::new(0.0, 0.0, 3.0, 3.0)));
        assert_eq!(union_opt(None, Some(b)), Some(b));
        assert_eq!(union_opt(None, None), None);
    }

    #[test]
    fn type_aliases_work() {
        let mut p = Path::default();
        assert!(p.is_empty());
        let sp = Subpath::polyline(&[Point::ZERO, Point::new(1.0, 1.0)], false);
        p.subpaths.push(sp);
        assert_eq!(p.anchor_count(), 2);
    }
}

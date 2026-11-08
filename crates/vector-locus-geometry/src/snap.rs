//! Snapping math for grids, guides, and points.

use kurbo::Point;

/// Snap a value to a grid with the given spacing and origin.
pub fn snap_to_grid(v: f64, spacing: f64, origin: f64) -> f64 {
    if spacing <= 0.0 {
        return v;
    }
    ((v - origin) / spacing).round() * spacing + origin
}

/// Snap a 2D point to an orthogonal grid with uniform spacing.
pub fn snap_point_to_grid(p: Point, spacing: f64) -> Point {
    Point::new(snap_to_grid(p.x, spacing, 0.0), snap_to_grid(p.y, spacing, 0.0))
}

/// A candidate alignment axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
}

/// Find the closest target within `tol` to `v`. Returns `(target, delta)`.
pub fn closest(v: f64, targets: impl IntoIterator<Item = f64>, tol: f64) -> Option<(f64, f64)> {
    targets.into_iter().map(|t| (t, t - v)).filter(|(_, d)| d.abs() <= tol).min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
}

/// Snap a point `p` against a collection of candidate points within tolerance `tol`.
/// Returns `(snapped_point, did_snap)`.
pub fn snap_point(p: Point, candidates: &[Point], tol: f64) -> (Point, bool) {
    let mut best: Option<(Point, f64)> = None;
    for &cand in candidates {
        let d = p.distance(cand);
        if d <= tol && best.as_ref().is_none_or(|b| d < b.1) {
            best = Some((cand, d));
        }
    }
    if let Some((cand, _)) = best {
        (cand, true)
    } else {
        (p, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        assert_eq!(snap_to_grid(13.0, 10.0, 0.0), 10.0);
        assert_eq!(snap_to_grid(16.0, 10.0, 0.0), 20.0);
        assert_eq!(snap_to_grid(16.0, 10.0, 3.0), 13.0);
        assert_eq!(snap_to_grid(16.0, 0.0, 0.0), 16.0);
        assert_eq!(snap_point_to_grid(Point::new(13.0, 16.0), 10.0), Point::new(10.0, 20.0));
    }

    #[test]
    fn closest_target() {
        assert_eq!(closest(10.0, [0.0, 11.0, 9.5], 2.0), Some((9.5, -0.5)));
        assert_eq!(closest(10.0, [0.0, 20.0], 2.0), None);
    }

    #[test]
    fn snap_point_candidates() {
        let targets = [Point::new(0.0, 0.0), Point::new(100.0, 100.0)];
        let (p, snapped) = snap_point(Point::new(99.0, 101.0), &targets, 3.0);
        assert!(snapped);
        assert_eq!(p, Point::new(100.0, 100.0));

        let (p2, snapped2) = snap_point(Point::new(50.0, 50.0), &targets, 3.0);
        assert!(!snapped2);
        assert_eq!(p2, Point::new(50.0, 50.0));
    }
}

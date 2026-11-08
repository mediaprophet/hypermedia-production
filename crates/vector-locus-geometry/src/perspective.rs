//! Perspective Grid: 1, 2, and 3-point perspective grid models, vanishing points,
//! and plane homographies.

use kurbo::{Point, Rect};
use serde::{Deserialize, Serialize};

use crate::projective::Homography;

/// Maximum plane depth offset in points.
pub const MAX_DEPTH: f64 = 100_000.0;

/// Grid planes in perspective.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Plane {
    #[default]
    Left,
    Right,
    Ground,
    None,
}

impl Plane {
    pub const ALL: [Plane; 3] = [Plane::Left, Plane::Right, Plane::Ground];

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "left" => Self::Left,
            "right" => Self::Right,
            "ground" | "horizontal" => Self::Ground,
            "none" => Self::None,
            _ => return None,
        })
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Ground => "ground",
            Self::None => "none",
        }
    }
}

/// Perspective coordinate axes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

fn two() -> u8 {
    2
}
fn yes() -> bool {
    true
}
fn is_zero(v: &f64) -> bool {
    *v == 0.0
}

/// Perspective grid definition and homography projection state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerspectiveGrid {
    /// 1, 2 or 3-point perspective.
    #[serde(default = "two")]
    pub kind: u8,
    /// Ground-level origin (where the planes meet), page coordinates.
    pub origin: [f64; 2],
    /// Horizon height (page y).
    pub horizon: f64,
    /// Left vanishing point x (the single vanishing point in 1-point perspective).
    pub vp_left: f64,
    /// Right vanishing point x.
    pub vp_right: f64,
    /// Third (vertical) vanishing point, 3-point only.
    pub vp_vertical: [f64; 2],
    /// Viewing distance (points): how fast receding axes foreshorten.
    pub distance: f64,
    /// Gridline every (points, in plane units).
    pub cell: f64,
    /// Horizontal extent of the planes (points, plane units).
    pub extent: f64,
    /// Vertical extent of the wall planes (points, plane units).
    pub height: f64,
    #[serde(default = "yes")]
    pub visible: bool,
    /// Active plane.
    #[serde(default)]
    pub plane: Plane,
    /// Viewing angle in degrees (two/three-point).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// Where the left, right and horizontal planes were moved along their normals.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub left_offset: f64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub right_offset: f64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub ground_offset: f64,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub locked: bool,
    #[serde(default = "yes")]
    pub snap: bool,
    /// Horizontal extent of the right plane; None: `extent`, as the left.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extent_right: Option<f64>,
}

impl PerspectiveGrid {
    /// Preset for `kind` (1, 2, or 3) fitted to a bounding box.
    pub fn preset(kind: u8, bounds: Rect) -> Self {
        let (w, h) = (bounds.width().max(1.0), bounds.height().max(1.0));
        let kind = kind.clamp(1, 3);
        let horizon = bounds.y0 + 0.42 * h;
        let ground = bounds.y0 + 0.78 * h;
        let ox = if kind == 1 { bounds.x0 + 0.3 * w } else { bounds.x0 + 0.5 * w };
        let (vl, vr) = if kind == 1 { (bounds.x0 + 0.55 * w, bounds.x1) } else { (bounds.x0 + 0.02 * w, bounds.x1 - 0.02 * w) };
        Self {
            kind,
            origin: [ox, ground],
            horizon,
            vp_left: vl,
            vp_right: vr,
            vp_vertical: [ox, horizon - 3.0 * (ground - horizon)],
            distance: 0.5 * w,
            cell: (w / 30.0).max(1.0).round(),
            extent: 0.6 * w,
            height: (ground - bounds.y0) * 0.8,
            visible: true,
            plane: Plane::Left,
            angle: None,
            left_offset: 0.0,
            right_offset: 0.0,
            ground_offset: 0.0,
            locked: false,
            snap: true,
            extent_right: None,
        }
    }

    /// Where `plane` is along its normal (points; 0: default position).
    pub fn offset(&self, plane: Plane) -> f64 {
        match plane {
            Plane::Left => self.left_offset,
            Plane::Right => self.right_offset,
            Plane::Ground => self.ground_offset,
            Plane::None => 0.0,
        }
    }

    /// Move `plane` to `offset` along its normal.
    pub fn set_offset(&mut self, plane: Plane, offset: f64) {
        match plane {
            Plane::Left => self.left_offset = offset,
            Plane::Right => self.right_offset = offset,
            Plane::Ground => self.ground_offset = offset,
            Plane::None => {}
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(1..=3).contains(&self.kind) {
            return Err("kind must be 1, 2 or 3".into());
        }
        let nums = [
            self.origin[0],
            self.origin[1],
            self.horizon,
            self.vp_left,
            self.vp_right,
            self.vp_vertical[0],
            self.vp_vertical[1],
            self.distance,
            self.cell,
            self.extent,
            self.height,
        ];
        if nums.iter().any(|v| !v.is_finite() || v.abs() > 4.0e6) {
            return Err("grid values must be finite".into());
        }
        if self.distance <= 0.0 || self.cell <= 0.0 || self.extent <= 0.0 || self.height <= 0.0 {
            return Err("distance, cell, extent and height must be positive".into());
        }
        if (self.origin[1] - self.horizon).abs() < 1e-6 {
            return Err("ground level must differ from the horizon".into());
        }
        let behind = |p: Plane| self.axes(p).is_some_and(|[.., n]| 1.0 + self.offset(p) * self.axis_col(n)[2] <= 1e-9);
        if Plane::ALL.iter().any(|p| !self.offset(*p).is_finite() || self.offset(*p).abs() > MAX_DEPTH || behind(*p)) {
            return Err("a plane is moved too far (behind the viewer)".into());
        }
        Ok(())
    }

    /// Foreshortening ratios along receding axes: left, right, vertical.
    pub fn foreshortening(&self) -> (f64, f64, f64) {
        let d = self.distance.max(1.0);
        let ll = 1.0 / d;
        let lr = 1.0 / d;
        let lu = if self.kind == 3 { 1.0 / d } else { 0.0 };
        (ll, lr, lu)
    }

    /// The image of `axis`'s point at infinity.
    pub fn axis_col(&self, axis: Axis) -> [f64; 3] {
        let (ll, lr, lu) = self.foreshortening();
        match axis {
            Axis::X if self.kind == 1 => [1.0, 0.0, 0.0],
            Axis::X => [self.vp_right * lr, self.horizon * lr, lr],
            Axis::Y if self.kind == 3 => [self.vp_vertical[0] * lu, self.vp_vertical[1] * lu, lu],
            Axis::Y => [0.0, -1.0, 0.0],
            Axis::Z => [self.vp_left * ll, self.horizon * ll, ll],
        }
    }

    /// A plane's axes: `[u, v, normal]`.
    pub fn axes(&self, plane: Plane) -> Option<[Axis; 3]> {
        Some(match (self.kind, plane) {
            (_, Plane::None) => return None,
            (1, Plane::Left) => [Axis::X, Axis::Y, Axis::Z],
            (1, Plane::Right) => [Axis::Z, Axis::Y, Axis::X],
            (_, Plane::Left) => [Axis::Z, Axis::Y, Axis::X],
            (_, Plane::Right) => [Axis::X, Axis::Y, Axis::Z],
            (_, Plane::Ground) => [Axis::X, Axis::Z, Axis::Y],
        })
    }

    /// The homography of `plane` at its current offset.
    pub fn homography(&self, plane: Plane) -> Option<Homography> {
        self.homography_at(plane, self.offset(plane))
    }

    /// The homography of the plane parallel to `plane` that lies `depth` points along its normal.
    pub fn homography_at(&self, plane: Plane, depth: f64) -> Option<Homography> {
        let [u, v, n] = self.axes(plane)?;
        self.frame_homography(u, v, (n, depth))
    }

    /// The plane spanned by axes `u` and `v` through the point `at.1` along axis `at.0`.
    pub fn frame_homography(&self, u: Axis, v: Axis, at: (Axis, f64)) -> Option<Homography> {
        if !at.1.is_finite() {
            return None;
        }
        let c = self.axis_col(at.0);
        let o = [self.origin[0] + at.1 * c[0], self.origin[1] + at.1 * c[1], 1.0 + at.1 * c[2]];
        if o[2] <= 1e-9 {
            return None;
        }
        let (cu, cv) = (self.axis_col(u), self.axis_col(v));
        let h = Homography::from_cols(cu, cv, o);
        (h.det().abs() > 1e-15).then_some(h)
    }

    /// Project a 2D point on `plane` (in plane coordinates, points) to page coordinates.
    pub fn to_page(&self, plane: Plane, p: Point) -> Option<Point> {
        self.homography(plane)?.apply(p)
    }

    /// Map a page coordinate point to plane coordinates.
    pub fn from_page(&self, plane: Plane, p: Point) -> Option<Point> {
        self.homography(plane)?.inverse()?.apply(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perspective_grid_preset_and_projections() {
        let r = Rect::new(0.0, 0.0, 800.0, 600.0);
        for kind in [1, 2, 3] {
            let grid = PerspectiveGrid::preset(kind, r);
            assert!(grid.validate().is_ok());

            let hl = grid.homography(Plane::Left).unwrap();
            let hr = grid.homography(Plane::Right).unwrap();
            let hg = grid.homography(Plane::Ground).unwrap();

            let origin_left = hl.apply(Point::ZERO).unwrap();
            let origin_right = hr.apply(Point::ZERO).unwrap();
            assert!(origin_left.distance(origin_right) < 1e-9);

            let pt = Point::new(50.0, 50.0);
            let page_pt = grid.to_page(Plane::Left, pt).unwrap();
            let back = grid.from_page(Plane::Left, page_pt).unwrap();
            assert!(back.distance(pt) < 1e-6);

            let ground_pt = grid.to_page(Plane::Ground, pt).unwrap();
            let back_g = grid.from_page(Plane::Ground, ground_pt).unwrap();
            assert!(back_g.distance(pt) < 1e-6);
        }
    }

    #[test]
    fn plane_offsets() {
        let mut grid = PerspectiveGrid::preset(2, Rect::new(0.0, 0.0, 500.0, 500.0));
        assert_eq!(grid.offset(Plane::Left), 0.0);
        grid.set_offset(Plane::Left, 25.0);
        assert_eq!(grid.offset(Plane::Left), 25.0);
        let h_offset = grid.homography(Plane::Left).unwrap();
        let h_zero = grid.homography_at(Plane::Left, 0.0).unwrap();
        assert_ne!(h_offset.to_array(), h_zero.to_array());
    }
}

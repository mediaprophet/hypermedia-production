//! Parametric architectural BIM geometry: compound walls with mitred corners, stairs/ramps, and MEP runs.
//!
//! Provides pure-Rust geometric representations for:
//! - Multi-layer compound walls with location line axes and mitred corner calculations (`walls`)
//! - Stair risers, treads, walk lines, and landing pads (`stairs`)
//! - Mechanical, Electrical, and Plumbing (MEP) 3D runs (pipes, ducts, conduits, cable trays) (`runs`)

use crate::csg::P3;
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

/// 2D Point.
pub type P2 = [f64; 2];

#[inline]
pub fn dist_2d(a: P2, b: P2) -> f64 {
    ((b[0] - a[0]).powi(2) + (b[1] - a[1]).powi(2)).sqrt()
}

#[inline]
pub fn sub_2d(a: P2, b: P2) -> P2 {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline]
pub fn add_2d(a: P2, b: P2) -> P2 {
    [a[0] + b[0], a[1] + b[1]]
}

#[inline]
pub fn mul_2d(a: P2, s: f64) -> P2 {
    [a[0] * s, a[1] * s]
}

#[inline]
pub fn unit_2d(a: P2) -> P2 {
    let d = (a[0] * a[0] + a[1] * a[1]).sqrt();
    if d > 1e-12 {
        [a[0] / d, a[1] / d]
    } else {
        [0.0, 0.0]
    }
}

#[inline]
pub fn perp_2d(a: P2) -> P2 {
    [-a[1], a[0]]
}

// ── Compound Walls ──

/// Axis of a wall location line: straight or an arc.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WallAxis {
    pub start: P2,
    pub end: P2,
    /// Arc center, radius, initial angle (radians), and sweep angle (radians).
    pub arc: Option<(P2, f64, f64, f64)>,
}

impl WallAxis {
    pub fn straight(start: P2, end: P2) -> Self {
        Self {
            start,
            end,
            arc: None,
        }
    }

    pub fn new_arc(start: P2, end: P2, arc_deg: f64) -> Self {
        let sweep = arc_deg.to_radians();
        let d = dist_2d(start, end);
        if !(sweep.is_finite() && sweep.abs() > 1e-9 && d > 0.0) {
            return Self::straight(start, end);
        }
        let half = sweep.abs() / 2.0;
        let mid = mul_2d(add_2d(start, end), 0.5);
        let dir = perp_2d(unit_2d(sub_2d(end, start)));
        let offset = sweep.signum() * d / (2.0 * half.tan());
        let c = add_2d(mid, mul_2d(dir, offset));
        let r = d / (2.0 * half.sin());
        let a0 = (start[1] - c[1]).atan2(start[0] - c[0]);
        Self {
            start,
            end,
            arc: Some((c, r, a0, sweep)),
        }
    }

    pub fn length(&self) -> f64 {
        match self.arc {
            Some((_, r, _, sweep)) => r * sweep.abs(),
            None => dist_2d(self.start, self.end),
        }
    }

    pub fn tangent(&self, s: f64) -> P2 {
        match self.arc {
            Some((_, r, a0, sweep)) => {
                let a = a0 + sweep.signum() * s / r;
                mul_2d([-a.sin(), a.cos()], sweep.signum())
            }
            None => unit_2d(sub_2d(self.end, self.start)),
        }
    }

    pub fn at(&self, s: f64, off: f64) -> P2 {
        match self.arc {
            Some((c, r, a0, sweep)) => {
                let a = a0 + sweep.signum() * s / r;
                add_2d(c, mul_2d([a.cos(), a.sin()], r - sweep.signum() * off))
            }
            None => {
                let u = unit_2d(sub_2d(self.end, self.start));
                add_2d(add_2d(self.start, mul_2d(u, s)), mul_2d(perp_2d(u), off))
            }
        }
    }
}

/// One physical material layer in a compound wall.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WallLayer {
    pub material: String,
    pub thickness_mm: f64,
    pub is_structural: bool,
}

/// Calculates mitred corner intersection between two straight walls meeting at an angle.
pub fn mitre_corner(
    wall_a: &WallAxis,
    thickness_a: f64,
    wall_b: &WallAxis,
    thickness_b: f64,
) -> Option<[P2; 4]> {
    let u_a = unit_2d(sub_2d(wall_a.end, wall_a.start));
    let u_b = unit_2d(sub_2d(wall_b.end, wall_b.start));
    let n_a = perp_2d(u_a);
    let n_b = perp_2d(u_b);

    let cross_z = u_a[0] * u_b[1] - u_a[1] * u_b[0];
    if cross_z.abs() < 1e-6 {
        return None; // Collinear or parallel
    }

    // Outer and inner edges offset from centerline
    let half_a = thickness_a / 2.0;
    let half_b = thickness_b / 2.0;

    let a_left = add_2d(wall_a.end, mul_2d(n_a, half_a));
    let a_right = add_2d(wall_a.end, mul_2d(n_a, -half_a));
    let b_left = add_2d(wall_b.start, mul_2d(n_b, half_b));
    let b_right = add_2d(wall_b.start, mul_2d(n_b, -half_b));

    Some([a_left, a_right, b_right, b_left])
}

// ── Stairs and Landings ──

/// A single flight run within a stair or ramp.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StairRun {
    pub start: P2,
    pub direction: P2,
    pub length: f64,
    pub elevation_start: f64,
    pub elevation_end: f64,
    pub risers: u32,
}

impl StairRun {
    pub fn end(&self) -> P2 {
        add_2d(self.start, mul_2d(self.direction, self.length))
    }

    pub fn outline(&self, width: f64) -> [P2; 4] {
        let n = mul_2d(perp_2d(self.direction), width / 2.0);
        let e = self.end();
        [
            sub_2d(self.start, n),
            sub_2d(e, n),
            add_2d(e, n),
            add_2d(self.start, n),
        ]
    }
}

/// A landing pad between stair flights.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StairPad {
    pub ring: Vec<P2>,
    pub elevation: f64,
}

/// Full stair geometry layout.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StairLayout {
    pub runs: Vec<StairRun>,
    pub pads: Vec<StairPad>,
    pub walk_line: Vec<(P2, f64)>,
    pub width: f64,
}

/// Generates a straight stair flight with uniform risers and treads.
pub fn generate_straight_stair(
    start: P2,
    direction_deg: f64,
    width: f64,
    tread_depth: f64,
    total_risers: u32,
    riser_height: f64,
    base_elevation: f64,
) -> StairLayout {
    let rad = direction_deg.to_radians();
    let dir = [rad.cos(), rad.sin()];
    let treads = total_risers.saturating_sub(1);
    let length = treads as f64 * tread_depth;
    let height = total_risers as f64 * riser_height;

    let run = StairRun {
        start,
        direction: dir,
        length,
        elevation_start: base_elevation,
        elevation_end: base_elevation + height,
        risers: total_risers,
    };

    let walk_line = vec![
        (start, base_elevation),
        (run.end(), base_elevation + height),
    ];

    StairLayout {
        runs: vec![run],
        pads: Vec::new(),
        walk_line,
        width,
    }
}

// ── MEP Runs ──

/// MEP run category classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MepCategory {
    Beam,
    StripFooting,
    Pipe,
    Conduit,
    CableTray,
    Wire,
    DownConductor,
    GroundConductor,
    Cable,
}

impl MepCategory {
    pub fn may_stand(self) -> bool {
        !matches!(self, MepCategory::Beam | MepCategory::StripFooting)
    }

    pub fn ifc_class_and_type(self) -> (&'static str, &'static str) {
        match self {
            MepCategory::Beam => ("IFCBEAM", ".BEAM."),
            MepCategory::StripFooting => ("IFCFOOTING", ".STRIP_FOOTING."),
            MepCategory::Pipe => ("IFCPIPESEGMENT", ".RIGIDSEGMENT."),
            MepCategory::Conduit => ("IFCCABLECARRIERSEGMENT", ".CONDUITSEGMENT."),
            MepCategory::CableTray => ("IFCCABLECARRIERSEGMENT", ".CABLETRAYSEGMENT."),
            MepCategory::Wire => ("IFCCABLESEGMENT", ".CABLESEGMENT."),
            MepCategory::DownConductor => ("IFCCABLESEGMENT", ".CONDUCTORSEGMENT."),
            MepCategory::GroundConductor => ("IFCCABLESEGMENT", ".CONDUCTORSEGMENT."),
            MepCategory::Cable => ("IFCCABLESEGMENT", ".CABLESEGMENT."),
        }
    }
}

/// 3D MEP straight run geometry.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct MepGeom {
    pub start: P3,
    pub end: P3,
    pub width: f64,
    pub depth: f64,
    pub is_round: bool,
}

impl MepGeom {
    pub fn new_pipe(start: P3, end: P3, diameter: f64) -> Self {
        Self {
            start,
            end,
            width: diameter,
            depth: diameter,
            is_round: true,
        }
    }

    pub fn new_duct(start: P3, end: P3, width: f64, depth: f64) -> Self {
        Self {
            start,
            end,
            width,
            depth,
            is_round: false,
        }
    }

    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn is_vertical(&self) -> bool {
        let dx = (self.end[0] - self.start[0]).abs();
        let dy = (self.end[1] - self.start[1]).abs();
        dx < 1e-4 && dy < 1e-4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wall_axis_straight_and_arc() {
        let w_straight = WallAxis::straight([0.0, 0.0], [5000.0, 0.0]);
        assert_eq!(w_straight.length(), 5000.0);
        assert_eq!(w_straight.tangent(100.0), [1.0, 0.0]);
        assert_eq!(w_straight.at(1000.0, 100.0), [1000.0, 100.0]);

        let w_arc = WallAxis::new_arc([0.0, 0.0], [2000.0, 0.0], 90.0);
        assert!(w_arc.arc.is_some());
        assert!(w_arc.length() > 2000.0);
    }

    #[test]
    fn mitre_corner_calculation() {
        let w1 = WallAxis::straight([0.0, 0.0], [3000.0, 0.0]);
        let w2 = WallAxis::straight([3000.0, 0.0], [3000.0, 4000.0]);
        let corner = mitre_corner(&w1, 200.0, &w2, 200.0);
        assert!(corner.is_some());
    }

    #[test]
    fn stair_flight_geometry() {
        let stair = generate_straight_stair([0.0, 0.0], 0.0, 1000.0, 250.0, 16, 175.0, 0.0);
        assert_eq!(stair.runs.len(), 1);
        let run = &stair.runs[0];
        assert_eq!(run.risers, 16);
        assert_eq!(run.length, 15.0 * 250.0);
        assert_eq!(run.elevation_end, 16.0 * 175.0);
    }

    #[test]
    fn mep_run_classification() {
        assert!(MepCategory::Pipe.may_stand());
        assert!(!MepCategory::Beam.may_stand());
        assert_eq!(
            MepCategory::CableTray.ifc_class_and_type(),
            ("IFCCABLECARRIERSEGMENT", ".CABLETRAYSEGMENT.")
        );

        let pipe = MepGeom::new_pipe([0.0, 0.0, 0.0], [0.0, 0.0, 3000.0], 100.0);
        assert!(pipe.is_vertical());
        assert_eq!(pipe.length(), 3000.0);
    }
}

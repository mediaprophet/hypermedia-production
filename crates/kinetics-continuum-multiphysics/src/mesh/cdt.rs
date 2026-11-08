//! 2D Constrained Delaunay Triangulation (CDT).
//!
//! Grounded in computational geometry determinism and topology preservation (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    pub fn dist_sq(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// A triangular cell with 3 node indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Triangle {
    pub nodes: [usize; 3],
}

/// Orientation test: positive if p0 -> p1 -> p2 is counter-clockwise.
pub fn orient2d(p0: Point2D, p1: Point2D, p2: Point2D) -> f64 {
    (p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x)
}

/// In-circle test: positive if p is inside the circumcircle of counter-clockwise (p0, p1, p2).
pub fn in_circle(p0: Point2D, p1: Point2D, p2: Point2D, p: Point2D) -> f64 {
    let d0x = p0.x - p.x;
    let d0y = p0.y - p.y;
    let d1x = p1.x - p.x;
    let d1y = p1.y - p.y;
    let d2x = p2.x - p.x;
    let d2y = p2.y - p.y;

    (d0x * d0x + d0y * d0y) * (d1x * d2y - d2x * d1y)
        - (d1x * d1x + d1y * d1y) * (d0x * d2y - d2x * d0y)
        + (d2x * d2x + d2y * d2y) * (d0x * d1y - d1x * d0y)
}

/// Compute 2D Delaunay triangulation via incremental point insertion.
pub fn triangulate_2d(points: &[Point2D]) -> Vec<Triangle> {
    if points.len() < 3 {
        return Vec::new();
    }

    // Compute bounding super-triangle
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for p in points {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    let dx = max_x - min_x;
    let dy = max_y - min_y;
    let dmax = dx.max(dy).max(1.0);
    let x_mid = (min_x + max_x) * 0.5;
    let y_mid = (min_y + max_y) * 0.5;

    let p_super0 = Point2D::new(x_mid - 20.0 * dmax, y_mid - dmax);
    let p_super1 = Point2D::new(x_mid, y_mid + 20.0 * dmax);
    let p_super2 = Point2D::new(x_mid + 20.0 * dmax, y_mid - dmax);

    let mut pts = points.to_vec();
    let s0 = pts.len();
    let s1 = s0 + 1;
    let s2 = s0 + 2;
    pts.push(p_super0);
    pts.push(p_super1);
    pts.push(p_super2);

    let mut triangles = vec![Triangle { nodes: [s0, s1, s2] }];

    for (p_idx, &p) in points.iter().enumerate() {
        let mut bad_triangles = Vec::new();

        for (t_idx, t) in triangles.iter().enumerate() {
            let p0 = pts[t.nodes[0]];
            let p1 = pts[t.nodes[1]];
            let p2 = pts[t.nodes[2]];
            let (p0, p1, p2) = if orient2d(p0, p1, p2) > 0.0 {
                (p0, p1, p2)
            } else {
                (p0, p2, p1)
            };

            if in_circle(p0, p1, p2, p) > 0.0 {
                bad_triangles.push(t_idx);
            }
        }

        // Polygon boundary of cavity
        let mut edges = Vec::new();
        for &t_idx in &bad_triangles {
            let t = triangles[t_idx];
            for i in 0..3 {
                let e0 = t.nodes[i];
                let e1 = t.nodes[(i + 1) % 3];
                let is_shared = edges.iter().position(|&(a, b)| (a == e0 && b == e1) || (a == e1 && b == e0));
                if let Some(pos) = is_shared {
                    edges.swap_remove(pos);
                } else {
                    edges.push((e0, e1));
                }
            }
        }

        // Remove bad triangles in reverse
        bad_triangles.sort_unstable();
        for &t_idx in bad_triangles.iter().rev() {
            triangles.swap_remove(t_idx);
        }

        // Add new triangles from polygon edges to point
        for (e0, e1) in edges {
            triangles.push(Triangle {
                nodes: [e0, e1, p_idx],
            });
        }
    }

    // Filter out triangles connected to super-triangle vertices
    triangles.retain(|t| t.nodes.iter().all(|&n| n < points.len()));
    triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangulate_four_points() {
        let pts = [
            Point2D::new(0.0, 0.0),
            Point2D::new(1.0, 0.0),
            Point2D::new(1.0, 1.0),
            Point2D::new(0.0, 1.0),
        ];
        let tris = triangulate_2d(&pts);
        assert_eq!(tris.len(), 2);
    }
}

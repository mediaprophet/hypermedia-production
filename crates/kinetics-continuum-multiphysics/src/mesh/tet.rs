//! 3D Delaunay and tetrahedral volume meshing.
//!
//! Grounded in computational geometry determinism and spatial discretization (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// 3D point coordinates.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// A tetrahedral cell with 4 node indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tetrahedron {
    pub nodes: [usize; 4],
}

/// Signed volume of a tetrahedron (positive if nodes satisfy right-hand rule).
pub fn tet_volume(p0: Point3D, p1: Point3D, p2: Point3D, p3: Point3D) -> f64 {
    let a = [p1.x - p0.x, p1.y - p0.y, p1.z - p0.z];
    let b = [p2.x - p0.x, p2.y - p0.y, p2.z - p0.z];
    let c = [p3.x - p0.x, p3.y - p0.y, p3.z - p0.z];

    let det = a[0] * (b[1] * c[2] - b[2] * c[1])
        - a[1] * (b[0] * c[2] - b[2] * c[0])
        + a[2] * (b[0] * c[1] - b[1] * c[0]);

    det / 6.0
}

/// Generate 5 or 6 tetrahedra from a hexahedral bounding box.
pub fn hex_to_tets(hex_nodes: [usize; 8]) -> [Tetrahedron; 5] {
    [
        Tetrahedron { nodes: [hex_nodes[0], hex_nodes[1], hex_nodes[3], hex_nodes[4]] },
        Tetrahedron { nodes: [hex_nodes[1], hex_nodes[2], hex_nodes[3], hex_nodes[6]] },
        Tetrahedron { nodes: [hex_nodes[4], hex_nodes[5], hex_nodes[6], hex_nodes[1]] },
        Tetrahedron { nodes: [hex_nodes[4], hex_nodes[6], hex_nodes[7], hex_nodes[3]] },
        Tetrahedron { nodes: [hex_nodes[1], hex_nodes[3], hex_nodes[4], hex_nodes[6]] },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tet_volume_unit_cube_subdivision() {
        let p0 = Point3D::new(0.0, 0.0, 0.0);
        let p1 = Point3D::new(1.0, 0.0, 0.0);
        let p2 = Point3D::new(0.0, 1.0, 0.0);
        let p3 = Point3D::new(0.0, 0.0, 1.0);
        let v = tet_volume(p0, p1, p2, p3);
        assert!((v - 1.0 / 6.0).abs() < 1e-12);
    }
}

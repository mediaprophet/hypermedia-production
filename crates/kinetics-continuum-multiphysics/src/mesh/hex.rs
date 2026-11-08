//! Swept and mapped hexahedral volume meshing.
//!
//! Grounded in structured spatial discretization and topology determinism (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};
use crate::mesh::tet::Point3D;

/// Hexahedral volume cell with 8 node indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hexahedron {
    pub nodes: [usize; 8],
}

/// Mesh structure containing 3D nodes and hexahedral elements.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HexMesh {
    pub nodes: Vec<Point3D>,
    pub elements: Vec<Hexahedron>,
}

/// Generate regular mapped hexahedral grid of nx x ny x nz cells.
pub fn generate_box_hex_mesh(
    nx: usize,
    ny: usize,
    nz: usize,
    min_corner: Point3D,
    max_corner: Point3D,
) -> HexMesh {
    assert!(nx >= 1 && ny >= 1 && nz >= 1);
    let mut nodes = Vec::with_capacity((nx + 1) * (ny + 1) * (nz + 1));

    let dx = (max_corner.x - min_corner.x) / nx as f64;
    let dy = (max_corner.y - min_corner.y) / ny as f64;
    let dz = (max_corner.z - min_corner.z) / nz as f64;

    for k in 0..=nz {
        let z = min_corner.z + k as f64 * dz;
        for j in 0..=ny {
            let y = min_corner.y + j as f64 * dy;
            for i in 0..=nx {
                let x = min_corner.x + i as f64 * dx;
                nodes.push(Point3D::new(x, y, z));
            }
        }
    }

    let node_idx = |i: usize, j: usize, k: usize| -> usize {
        (k * (ny + 1) + j) * (nx + 1) + i
    };

    let mut elements = Vec::with_capacity(nx * ny * nz);
    for k in 0..nz {
        for j in 0..ny {
            for i in 0..nx {
                elements.push(Hexahedron {
                    nodes: [
                        node_idx(i, j, k),
                        node_idx(i + 1, j, k),
                        node_idx(i + 1, j + 1, k),
                        node_idx(i, j + 1, k),
                        node_idx(i, j, k + 1),
                        node_idx(i + 1, j, k + 1),
                        node_idx(i + 1, j + 1, k + 1),
                        node_idx(i, j + 1, k + 1),
                    ],
                });
            }
        }
    }

    HexMesh { nodes, elements }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_mesh_node_and_element_counts() {
        let mesh = generate_box_hex_mesh(
            2,
            2,
            2,
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 1.0),
        );
        assert_eq!(mesh.elements.len(), 8);
        assert_eq!(mesh.nodes.len(), 27);
    }
}

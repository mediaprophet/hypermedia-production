//! Mesh element quality metrics and Laplacian smoothing relaxation.
//!
//! Grounded in computational mesh quality preservation (UDHR Art. 19, 27).

use crate::mesh::tet::Point3D;

/// Scaled Jacobian metric for a hexahedral element: in [-1, 1], where 1 is an ideal cube.
pub fn hex_scaled_jacobian(nodes: &[Point3D; 8]) -> f64 {
    let edges = [
        // Node 0
        ([1, 0], [3, 0], [4, 0]),
        // Node 1
        ([2, 1], [0, 1], [5, 1]),
        // Node 2
        ([3, 2], [1, 2], [6, 2]),
        // Node 3
        ([0, 3], [2, 3], [7, 3]),
        // Node 4
        ([5, 4], [7, 4], [0, 4]),
        // Node 5
        ([6, 5], [4, 5], [1, 5]),
        // Node 6
        ([7, 6], [5, 6], [2, 6]),
        // Node 7
        ([4, 7], [6, 7], [3, 7]),
    ];

    let mut min_sj = 1.0f64;

    for (e1, e2, e3) in edges {
        let v1 = vec_sub(nodes[e1[0]], nodes[e1[1]]);
        let v2 = vec_sub(nodes[e2[0]], nodes[e2[1]]);
        let v3 = vec_sub(nodes[e3[0]], nodes[e3[1]]);

        let l1 = vec_len(v1).max(1e-12);
        let l2 = vec_len(v2).max(1e-12);
        let l3 = vec_len(v3).max(1e-12);

        let det = v1[0] * (v2[1] * v3[2] - v2[2] * v3[1])
            - v1[1] * (v2[0] * v3[2] - v2[2] * v3[0])
            + v1[2] * (v2[0] * v3[1] - v2[1] * v3[0]);

        let sj = det / (l1 * l2 * l3);
        min_sj = min_sj.min(sj);
    }

    min_sj
}

/// Laplacian smoothing step for interior node positions given neighbor adjacency.
pub fn laplacian_smooth_step(
    nodes: &mut [Point3D],
    neighbors: &[Vec<usize>],
    is_boundary: &[bool],
    relaxation_factor: f64,
) {
    let original = nodes.to_vec();
    for (i, p) in nodes.iter_mut().enumerate() {
        if is_boundary[i] || neighbors[i].is_empty() {
            continue;
        }

        let mut avg = [0.0; 3];
        let count = neighbors[i].len() as f64;
        for &nbr in &neighbors[i] {
            avg[0] += original[nbr].x;
            avg[1] += original[nbr].y;
            avg[2] += original[nbr].z;
        }
        avg[0] /= count;
        avg[1] /= count;
        avg[2] /= count;

        p.x += relaxation_factor * (avg[0] - p.x);
        p.y += relaxation_factor * (avg[1] - p.y);
        p.z += relaxation_factor * (avg[2] - p.z);
    }
}

fn vec_sub(a: Point3D, b: Point3D) -> [f64; 3] {
    [a.x - b.x, a.y - b.y, a.z - b.z]
}

fn vec_len(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_cube_scaled_jacobian_is_one() {
        let nodes = [
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(1.0, 1.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
            Point3D::new(0.0, 0.0, 1.0),
            Point3D::new(1.0, 0.0, 1.0),
            Point3D::new(1.0, 1.0, 1.0),
            Point3D::new(0.0, 1.0, 1.0),
        ];
        let sj = hex_scaled_jacobian(&nodes);
        assert!((sj - 1.0).abs() < 1e-10);
    }
}

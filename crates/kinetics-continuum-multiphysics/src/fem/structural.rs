//! Structural and multiphysics finite elements: MITC4 shells, Timoshenko beams, and Thermal conduction.
//!
//! Grounded in structural integrity and thermal continuum conservation (UDHR Art. 19, 27; ICESCR Art. 15).

use crate::fem::constitutive::ElasticIsotropic;
use crate::fem::shape::{gauss_quad_2x2, shape_deriv_quad4, shape_quad4};

/// 3D Timoshenko beam element stiffness matrix (12x12 for 2 nodes with 6 DOFs per node).
pub fn timoshenko_beam_3d(
    length: f64,
    area: f64,
    iy: f64,
    iz: f64,
    j_torsion: f64,
    mat: &ElasticIsotropic,
) -> [[f64; 12]; 12] {
    let e = mat.youngs_modulus;
    let g = mat.shear_modulus();
    let l = length.max(1e-9);
    let l2 = l * l;
    let l3 = l2 * l;

    let mut k = [[0.0; 12]; 12];

    // Axial stiffness
    let k_ax = (e * area) / l;
    k[0][0] = k_ax;
    k[0][6] = -k_ax;
    k[6][0] = -k_ax;
    k[6][6] = k_ax;

    // Torsional stiffness
    let k_tor = (g * j_torsion) / l;
    k[3][3] = k_tor;
    k[3][9] = -k_tor;
    k[9][3] = -k_tor;
    k[9][9] = k_tor;

    // Bending in xy plane (iz)
    let k_b_z = e * iz;
    k[1][1] = 12.0 * k_b_z / l3;
    k[1][5] = 6.0 * k_b_z / l2;
    k[1][7] = -12.0 * k_b_z / l3;
    k[1][11] = 6.0 * k_b_z / l2;

    k[5][1] = 6.0 * k_b_z / l2;
    k[5][5] = 4.0 * k_b_z / l;
    k[5][7] = -6.0 * k_b_z / l2;
    k[5][11] = 2.0 * k_b_z / l;

    k[7][1] = -12.0 * k_b_z / l3;
    k[7][5] = -6.0 * k_b_z / l2;
    k[7][7] = 12.0 * k_b_z / l3;
    k[7][11] = -6.0 * k_b_z / l2;

    k[11][1] = 6.0 * k_b_z / l2;
    k[11][5] = 2.0 * k_b_z / l;
    k[11][7] = -6.0 * k_b_z / l2;
    k[11][11] = 4.0 * k_b_z / l;

    // Bending in xz plane (iy)
    let k_b_y = e * iy;
    k[2][2] = 12.0 * k_b_y / l3;
    k[2][4] = -6.0 * k_b_y / l2;
    k[2][8] = -12.0 * k_b_y / l3;
    k[2][10] = -6.0 * k_b_y / l2;

    k[4][2] = -6.0 * k_b_y / l2;
    k[4][4] = 4.0 * k_b_y / l;
    k[4][8] = 6.0 * k_b_y / l2;
    k[4][10] = 2.0 * k_b_y / l;

    k[8][2] = -12.0 * k_b_y / l3;
    k[8][4] = 6.0 * k_b_y / l2;
    k[8][8] = 12.0 * k_b_y / l3;
    k[8][10] = 6.0 * k_b_y / l2;

    k[10][2] = -6.0 * k_b_y / l2;
    k[10][4] = 2.0 * k_b_y / l;
    k[10][8] = 6.0 * k_b_y / l2;
    k[10][10] = 4.0 * k_b_y / l;

    k
}

/// MITC4 (Mixed Interpolation of Tensorial Components) 4-node shell element.
/// Overcomes shear locking by evaluating transverse shear strains at tying points.
pub fn mitc4_shear_strains(
    xi: f64,
    eta: f64,
    w_nodes: [f64; 4],
    theta_x: [f64; 4],
    theta_y: [f64; 4],
) -> (f64, f64) {
    // Tying point evaluations
    let gamma_xz = 0.5 * (1.0 - eta) * (w_nodes[1] - w_nodes[0] - 0.5 * (theta_y[0] + theta_y[1]))
        + 0.5 * (1.0 + eta) * (w_nodes[2] - w_nodes[3] - 0.5 * (theta_y[2] + theta_y[3]));

    let gamma_yz = 0.5 * (1.0 - xi) * (w_nodes[3] - w_nodes[0] + 0.5 * (theta_x[0] + theta_x[3]))
        + 0.5 * (1.0 + xi) * (w_nodes[2] - w_nodes[1] + 0.5 * (theta_x[1] + theta_x[2]));

    (gamma_xz, gamma_yz)
}

/// 4-node 2D thermal conduction element conductivity matrix (4x4).
pub fn thermal_conduction_quad4(
    coords: [[f64; 2]; 4],
    thermal_conductivity: f64,
    thickness: f64,
) -> [[f64; 4]; 4] {
    let mut k = [[0.0; 4]; 4];
    let gp = gauss_quad_2x2();

    for pt in &gp {
        let deriv = shape_deriv_quad4(pt.coords[0], pt.coords[1]);

        // Jacobian matrix J = dN/dxi * coords
        let mut j_mat = [[0.0; 2]; 2];
        for i in 0..4 {
            j_mat[0][0] += deriv[i][0] * coords[i][0];
            j_mat[0][1] += deriv[i][0] * coords[i][1];
            j_mat[1][0] += deriv[i][1] * coords[i][0];
            j_mat[1][1] += deriv[i][1] * coords[i][1];
        }

        let det_j = j_mat[0][0] * j_mat[1][1] - j_mat[0][1] * j_mat[1][0];
        let inv_det = 1.0 / det_j.max(1e-12);

        // Inverse Jacobian
        let inv_j = [
            [j_mat[1][1] * inv_det, -j_mat[0][1] * inv_det],
            [-j_mat[1][0] * inv_det, j_mat[0][0] * inv_det],
        ];

        // Global derivatives dN/dx = inv_J * dN/dxi
        let mut d_glob = [[0.0; 2]; 4];
        for i in 0..4 {
            d_glob[i][0] = inv_j[0][0] * deriv[i][0] + inv_j[0][1] * deriv[i][1];
            d_glob[i][1] = inv_j[1][0] * deriv[i][0] + inv_j[1][1] * deriv[i][1];
        }

        let factor = thermal_conductivity * thickness * det_j * pt.weight;

        for i in 0..4 {
            for j in 0..4 {
                let grad_dot = d_glob[i][0] * d_glob[j][0] + d_glob[i][1] * d_glob[j][1];
                k[i][j] += factor * grad_dot;
            }
        }
    }

    k
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timoshenko_beam_symmetry() {
        let mat = ElasticIsotropic::new(210e9, 0.3, 7850.0);
        let k = timoshenko_beam_3d(2.0, 0.01, 1e-4, 1e-4, 2e-4, &mat);
        for i in 0..12 {
            for j in 0..12 {
                assert!((k[i][j] - k[j][i]).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn thermal_conduction_matrix_symmetry() {
        let coords = [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        let k = thermal_conduction_quad4(coords, 50.0, 1.0);
        for i in 0..4 {
            for j in 0..4 {
                assert!((k[i][j] - k[j][i]).abs() < 1e-6);
            }
        }
    }
}

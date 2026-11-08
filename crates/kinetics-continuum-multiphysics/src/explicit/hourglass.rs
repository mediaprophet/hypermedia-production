//! Flanagan-Belytschko hourglass stabilization for reduced-integration elements.
//!
//! Grounded in numerical rank-sufficiency and volumetric preservation (UDHR Art. 19, 27).

/// Standard 4 orthogonal hourglass vectors for 8-node hexahedron: gamma_alpha (alpha = 1..4).
pub const GAMMA_HOURGLASS: [[f64; 8]; 4] = [
    [1.0, 1.0, -1.0, -1.0, -1.0, -1.0, 1.0, 1.0],
    [1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0],
    [1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0],
    [-1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, -1.0],
];

/// Compute Flanagan-Belytschko hourglass resisting nodal forces (8 nodes x 3 DOFs).
pub fn flanagan_belytschko_hex8(
    nodal_velocities: &[[f64; 3]; 8],
    element_volume: f64,
    density: f64,
    sound_speed: f64,
    hourglass_coefficient: f64,
) -> [[f64; 3]; 8] {
    let mut f_hg = [[0.0; 3]; 8];
    let vol = element_volume.max(1e-12);
    let char_len = vol.cbrt();
    let stiffness = hourglass_coefficient * density * sound_speed * char_len * char_len;

    for alpha in 0..4 {
        let gamma = &GAMMA_HOURGLASS[alpha];
        let mut q = [0.0; 3];

        // Generalized hourglass velocity q_alpha = gamma_alpha^T * v
        for i in 0..8 {
            q[0] += gamma[i] * nodal_velocities[i][0];
            q[1] += gamma[i] * nodal_velocities[i][1];
            q[2] += gamma[i] * nodal_velocities[i][2];
        }

        // Resisting nodal forces
        for i in 0..8 {
            f_hg[i][0] += stiffness * gamma[i] * q[0];
            f_hg[i][1] += stiffness * gamma[i] * q[1];
            f_hg[i][2] += stiffness * gamma[i] * q[2];
        }
    }

    f_hg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hourglass_zero_for_rigid_translation() {
        let v = [[1.0, 2.0, 3.0]; 8];
        let f_hg = flanagan_belytschko_hex8(&v, 1.0, 7800.0, 5000.0, 0.1);
        for node in &f_hg {
            assert!(node[0].abs() < 1e-10);
            assert!(node[1].abs() < 1e-10);
            assert!(node[2].abs() < 1e-10);
        }
    }
}

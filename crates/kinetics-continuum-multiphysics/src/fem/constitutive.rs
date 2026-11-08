//! Constitutive models: Linear Elasticity, St. Venant-Kirchhoff, Neo-Hookean, and J2 Plasticity.
//!
//! Grounded in material invariance and continuum thermodynamics (UDHR Art. 19, 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};

/// Linear isotropic elastic material parameters.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticIsotropic {
    pub youngs_modulus: f64,
    pub poissons_ratio: f64,
    pub density: f64,
}

impl ElasticIsotropic {
    pub fn new(youngs_modulus: f64, poissons_ratio: f64, density: f64) -> Self {
        Self {
            youngs_modulus,
            poissons_ratio,
            density,
        }
    }

    /// First Lamé parameter lambda = E * nu / ((1 + nu) * (1 - 2*nu)).
    pub fn lame_lambda(&self) -> f64 {
        let e = self.youngs_modulus;
        let nu = self.poissons_ratio;
        (e * nu) / ((1.0 + nu) * (1.0 - 2.0 * nu))
    }

    /// Second Lamé parameter (shear modulus) mu = E / (2 * (1 + nu)).
    pub fn shear_modulus(&self) -> f64 {
        self.youngs_modulus / (2.0 * (1.0 + self.poissons_ratio))
    }

    /// Bulk modulus K = E / (3 * (1 - 2*nu)).
    pub fn bulk_modulus(&self) -> f64 {
        self.youngs_modulus / (3.0 * (1.0 - 2.0 * self.poissons_ratio))
    }

    /// 6x6 elasticity matrix (Voigt notation: [eps_xx, eps_yy, eps_zz, gamma_yz, gamma_xz, gamma_xy]).
    pub fn elasticity_matrix_3d(&self) -> [[f64; 6]; 6] {
        let l = self.lame_lambda();
        let m = self.shear_modulus();
        let mut d = [[0.0; 6]; 6];
        d[0][0] = l + 2.0 * m;
        d[0][1] = l;
        d[0][2] = l;
        d[1][0] = l;
        d[1][1] = l + 2.0 * m;
        d[1][2] = l;
        d[2][0] = l;
        d[2][1] = l;
        d[2][2] = l + 2.0 * m;
        d[3][3] = m;
        d[4][4] = m;
        d[5][5] = m;
        d
    }

    /// 3x3 plane stress elasticity matrix [eps_xx, eps_yy, gamma_xy].
    pub fn plane_stress_matrix(&self) -> [[f64; 3]; 3] {
        let e = self.youngs_modulus;
        let nu = self.poissons_ratio;
        let factor = e / (1.0 - nu * nu);
        [
            [factor, factor * nu, 0.0],
            [factor * nu, factor, 0.0],
            [0.0, 0.0, factor * (1.0 - nu) * 0.5],
        ]
    }

    /// 3x3 plane strain elasticity matrix [eps_xx, eps_yy, gamma_xy].
    pub fn plane_strain_matrix(&self) -> [[f64; 3]; 3] {
        let e = self.youngs_modulus;
        let nu = self.poissons_ratio;
        let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));
        [
            [factor * (1.0 - nu), factor * nu, 0.0],
            [factor * nu, factor * (1.0 - nu), 0.0],
            [0.0, 0.0, factor * (0.5 - nu)],
        ]
    }
}

/// St. Venant-Kirchhoff hyperelastic material model: S = lambda * tr(E)*I + 2*mu*E.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaintVenantKirchhoff {
    pub elastic: ElasticIsotropic,
}

impl SaintVenantKirchhoff {
    pub fn new(elastic: ElasticIsotropic) -> Self {
        Self { elastic }
    }

    /// Compute Second Piola-Kirchhoff stress tensor S from Green-Lagrange strain tensor E (3x3).
    pub fn compute_pk2_stress(&self, green_lagrange: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let l = self.elastic.lame_lambda();
        let m = self.elastic.shear_modulus();
        let tr_e = green_lagrange[0][0] + green_lagrange[1][1] + green_lagrange[2][2];
        let mut s = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                s[i][j] = 2.0 * m * green_lagrange[i][j];
                if i == j {
                    s[i][j] += l * tr_e;
                }
            }
        }
        s
    }
}

/// Compressible Neo-Hookean hyperelastic material model.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeoHookean {
    pub elastic: ElasticIsotropic,
}

impl NeoHookean {
    pub fn new(elastic: ElasticIsotropic) -> Self {
        Self { elastic }
    }

    /// Compute Cauchy stress sigma from deformation gradient F (3x3).
    pub fn compute_cauchy_stress(&self, f: [[f64; 3]; 3]) -> Result<[[f64; 3]; 3], &'static str> {
        let det_f = det3x3(f);
        if det_f <= 0.0 {
            return Err("Non-positive Jacobian determinant in Neo-Hookean model");
        }
        let m = self.elastic.shear_modulus();
        let l = self.elastic.lame_lambda();
        let ln_j = det_f.ln();

        // Left Cauchy-Green tensor B = F * F^T
        let mut b = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    b[i][j] += f[i][k] * f[j][k];
                }
            }
        }

        // Cauchy stress sigma = (mu/J)*(B - I) + (lambda/J)*ln(J)*I
        let mut sigma = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                sigma[i][j] = (m / det_f) * b[i][j];
                if i == j {
                    sigma[i][j] += (l * ln_j - m) / det_f;
                }
            }
        }
        Ok(sigma)
    }
}

/// J2 Plasticity with isotropic hardening.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct J2Plasticity {
    pub elastic: ElasticIsotropic,
    pub yield_stress: f64,
    pub hardening_modulus: f64,
}

impl J2Plasticity {
    pub fn new(elastic: ElasticIsotropic, yield_stress: f64, hardening_modulus: f64) -> Self {
        Self {
            elastic,
            yield_stress,
            hardening_modulus,
        }
    }

    /// Classical radial return algorithm: updates stress, backstress, and equivalent plastic strain.
    pub fn radial_return(
        &self,
        trial_strain: [f64; 6],
        eq_plastic_strain_old: f64,
    ) -> ([f64; 6], f64, bool) {
        let d = self.elastic.elasticity_matrix_3d();
        let mut trial_stress = [0.0; 6];
        for i in 0..6 {
            for j in 0..6 {
                trial_stress[i] += d[i][j] * trial_strain[j];
            }
        }

        // Hydrostatic stress p = tr(sigma) / 3
        let p = (trial_stress[0] + trial_stress[1] + trial_stress[2]) / 3.0;

        // Deviatoric trial stress s = sigma - p * I
        let mut s = trial_stress;
        s[0] -= p;
        s[1] -= p;
        s[2] -= p;

        // Von Mises equivalent stress q = sqrt(3/2 * s : s)
        let j2 = 0.5 * (s[0] * s[0] + s[1] * s[1] + s[2] * s[2])
            + s[3] * s[3]
            + s[4] * s[4]
            + s[5] * s[5];
        let q_trial = (3.0 * j2).sqrt();

        // Current yield strength
        let sigma_y = self.yield_stress + self.hardening_modulus * eq_plastic_strain_old;
        let yield_func = q_trial - sigma_y;

        if yield_func <= 0.0 {
            // Elastic step
            (trial_stress, eq_plastic_strain_old, false)
        } else {
            // Plastic step
            let mu = self.elastic.shear_modulus();
            let delta_gamma = yield_func / (3.0 * mu + self.hardening_modulus);
            let factor = 1.0 - (3.0 * mu * delta_gamma) / q_trial.max(1e-12);

            let mut final_stress = trial_stress;
            final_stress[0] = p + s[0] * factor;
            final_stress[1] = p + s[1] * factor;
            final_stress[2] = p + s[2] * factor;
            final_stress[3] = s[3] * factor;
            final_stress[4] = s[4] * factor;
            final_stress[5] = s[5] * factor;

            (final_stress, eq_plastic_strain_old + delta_gamma, true)
        }
    }
}

fn det3x3(m: [[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elasticity_matrix_symmetry() {
        let mat = ElasticIsotropic::new(210e9, 0.3, 7850.0);
        let d = mat.elasticity_matrix_3d();
        for i in 0..6 {
            for j in 0..6 {
                assert!((d[i][j] - d[j][i]).abs() < 1e-4);
            }
        }
    }

    #[test]
    fn j2_plasticity_yields_when_exceeding_threshold() {
        let mat = ElasticIsotropic::new(200e9, 0.3, 7800.0);
        let j2 = J2Plasticity::new(mat, 250e6, 1e9);

        // Sub-yield strain
        let (stress1, p_strain1, plastic1) = j2.radial_return([1e-4, 0.0, 0.0, 0.0, 0.0, 0.0], 0.0);
        assert!(!plastic1);
        assert_eq!(p_strain1, 0.0);

        // Plastic strain
        let (stress2, p_strain2, plastic2) = j2.radial_return([5e-3, 0.0, 0.0, 0.0, 0.0, 0.0], 0.0);
        assert!(plastic2);
        assert!(p_strain2 > 0.0);
    }
}

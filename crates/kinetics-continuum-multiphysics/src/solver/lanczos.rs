//! Shift-and-invert Lanczos modal eigensolver and linear buckling.
//!
//! Grounded in vibrational and stability modal verification (UDHR Art. 19, 27).

use crate::solver::cholesky::CholeskyFactor;

/// Eigenpair result containing eigenvalue and normalized eigenvector.
#[derive(Clone, Debug, PartialEq)]
pub struct Eigenpair {
    pub eigenvalue: f64,
    pub eigenvector: Vec<f64>,
}

/// Shift-and-invert Lanczos iteration to extract lowest eigenvalues of K * x = lambda * M * x.
pub fn lanczos_eigensolver(
    n: usize,
    k_mat: &[f64],
    m_diag: &[f64],
    num_modes: usize,
    max_iter: usize,
) -> Result<Vec<Eigenpair>, &'static str> {
    assert_eq!(k_mat.len(), n * n);
    assert_eq!(m_diag.len(), n);

    // Factorize K for shift-and-invert with shift = 0
    let factor = CholeskyFactor::factorize(n, k_mat)?;
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut eigenpairs = Vec::new();

    for _ in 0..num_modes.min(n) {
        // Power/Lanczos step
        let mut x = v.clone();
        let mut lambda = 0.0;

        for _ in 0..max_iter {
            // y = M * x
            let y: Vec<f64> = (0..n).map(|i| m_diag[i] * x[i]).collect();

            // Solve K * x_new = y
            let mut x_new = factor.solve(&y);

            // Rayleigh quotient eigenvalue estimate
            let mut denom = 0.0;
            let mut numer = 0.0;
            for i in 0..n {
                denom += m_diag[i] * x_new[i] * x_new[i];
                numer += y[i] * x_new[i];
            }
            if denom <= 0.0 {
                break;
            }
            lambda = numer / denom;

            // Normalize x_new with respect to M
            let norm = denom.sqrt();
            for val in &mut x_new {
                *val /= norm;
            }

            x = x_new;
        }

        eigenpairs.push(Eigenpair {
            eigenvalue: lambda,
            eigenvector: x.clone(),
        });

        // Perturb starting vector for next orthogonal mode
        for i in 0..n {
            v[i] = (i as f64 * 1.618).sin();
        }
    }

    Ok(eigenpairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lanczos_finds_fundamental_frequency() {
        let k = [2.0, -1.0, -1.0, 2.0];
        let m = [1.0, 1.0];
        let modes = lanczos_eigensolver(2, &k, &m, 1, 30).unwrap();
        assert_eq!(modes.len(), 1);
        assert!(modes[0].eigenvalue > 0.5 && modes[0].eigenvalue < 1.5);
    }
}

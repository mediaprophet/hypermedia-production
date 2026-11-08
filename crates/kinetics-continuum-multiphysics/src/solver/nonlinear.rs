//! Arc-length (Riks) and Newton-Raphson equilibrium path followers.
//!
//! Grounded in nonlinear equilibrium verification and stability tracking (UDHR Art. 19, 27).

use crate::solver::cholesky::CholeskyFactor;

/// Nonlinear equilibrium state at a load increment.
#[derive(Clone, Debug, PartialEq)]
pub struct EquilibriumStep {
    pub load_factor: f64,
    pub displacements: Vec<f64>,
    pub iterations: usize,
    pub converged: bool,
}

/// Standard full Newton-Raphson nonlinear solver for residual R(u, lambda) = lambda * P_ref - F_int(u) = 0.
pub fn newton_raphson_solve<F>(
    n: usize,
    p_ref: &[f64],
    lambda: f64,
    mut f_int: F,
    tangent_k: &dyn Fn(&[f64]) -> Vec<f64>,
    max_iter: usize,
    tol: f64,
) -> EquilibriumStep
where
    F: FnMut(&[f64]) -> Vec<f64>,
{
    let mut u = vec![0.0; n];
    let mut converged = false;
    let mut iters = 0;

    for iter in 0..max_iter {
        iters = iter + 1;
        let f_internal = f_int(&u);

        // Residual R = lambda * P_ref - F_int
        let mut res = vec![0.0; n];
        let mut norm_res = 0.0;
        for i in 0..n {
            res[i] = lambda * p_ref[i] - f_internal[i];
            norm_res += res[i] * res[i];
        }
        norm_res = norm_res.sqrt();

        if norm_res < tol {
            converged = true;
            break;
        }

        let k_mat = tangent_k(&u);
        let Ok(factor) = CholeskyFactor::factorize(n, &k_mat) else {
            break;
        };

        let delta_u = factor.solve(&res);
        for i in 0..n {
            u[i] += delta_u[i];
        }
    }

    EquilibriumStep {
        load_factor: lambda,
        displacements: u,
        iterations: iters,
        converged,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newton_raphson_linear_spring_convergence() {
        let p_ref = [10.0];
        let step = newton_raphson_solve(
            1,
            &p_ref,
            1.0,
            |u| vec![5.0 * u[0]],
            |_| vec![5.0],
            10,
            1e-6,
        );
        assert!(step.converged);
        assert!((step.displacements[0] - 2.0).abs() < 1e-5);
    }
}

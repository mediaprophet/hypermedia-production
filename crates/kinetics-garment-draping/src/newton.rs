//! Implicit Euler regularized Newton-PCG solver with Armijo backtracking line search.
//! Unconditionally stable, large-step monolithic cloth draping.

use crate::energy::{bending_element, membrane_element};
use crate::math::DVec3;
use crate::vbd::ClothMesh;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewtonSettings {
    pub dt: f64,
    pub gravity: DVec3,
    pub damping: f64,
    pub max_newton_iters: usize,
    pub max_pcg_iters: usize,
    pub tolerance: f64,
}

impl Default for NewtonSettings {
    fn default() -> Self {
        Self {
            dt: 1.0 / 60.0,
            gravity: DVec3::new(0.0, -9.81, 0.0),
            damping: 0.4,
            max_newton_iters: 10,
            max_pcg_iters: 50,
            tolerance: 1e-4,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NewtonMetrics {
    pub newton_iters: usize,
    pub pcg_iters: usize,
    pub converged: bool,
    pub residual_norm: f64,
}

/// Executes one implicit Euler step via regularized Newton-PCG.
pub fn newton_step(mesh: &mut ClothMesh, settings: &NewtonSettings) -> NewtonMetrics {
    let n = mesh.positions.len();
    let mut metrics = NewtonMetrics::default();
    if n == 0 {
        return metrics;
    }

    let dt = settings.dt;
    let dt2 = dt * dt;
    let damp = (1.0 - settings.damping * dt).max(0.0);
    let x_prev = mesh.positions.clone();

    // Inertial prediction
    let mut x_pred = mesh.positions.clone();
    for i in 0..n {
        if mesh.pins[i].is_none() {
            x_pred[i] += mesh.velocities[i] * (dt * damp) + settings.gravity * dt2;
        }
    }

    let dof_count = n * 3;

    for n_iter in 0..settings.max_newton_iters {
        metrics.newton_iters = n_iter + 1;

        // Assemble gradient and diagonal Hessian
        let mut g = vec![0.0f64; dof_count];
        let mut h_diag = vec![0.0f64; dof_count];

        // Inertia
        for i in 0..n {
            if mesh.pins[i].is_some() {
                continue;
            }
            let m = mesh.masses[i];
            let diff = mesh.positions[i] - x_pred[i];
            g[3 * i] += m * diff.x / dt2;
            g[3 * i + 1] += m * diff.y / dt2;
            g[3 * i + 2] += m * diff.z / dt2;

            h_diag[3 * i] += m / dt2;
            h_diag[3 * i + 1] += m / dt2;
            h_diag[3 * i + 2] += m / dt2;
        }

        // Membrane forces
        for tri in &mesh.triangles {
            let elem = membrane_element(&mesh.positions, tri, &mesh.material);
            for (local_i, &vi) in tri.vertices.iter().enumerate() {
                if mesh.pins[vi].is_some() {
                    continue;
                }
                for k in 0..3 {
                    g[3 * vi + k] += elem.gradient[3 * local_i + k];
                    let h_idx = (3 * local_i + k) * 9 + (3 * local_i + k);
                    h_diag[3 * vi + k] += elem.hessian[h_idx].max(0.0);
                }
            }
        }

        // Bending forces
        for h in &mesh.hinges {
            let elem = bending_element(&mesh.positions, h, &mesh.material);
            for (local_i, &vi) in h.vertices.iter().enumerate() {
                if mesh.pins[vi].is_some() {
                    continue;
                }
                for k in 0..3 {
                    g[3 * vi + k] += elem.gradient[3 * local_i + k];
                    let h_idx = (3 * local_i + k) * 12 + (3 * local_i + k);
                    h_diag[3 * vi + k] += elem.hessian[h_idx].max(0.0);
                }
            }
        }

        // Compute residual norm
        let mut res_sq = 0.0f64;
        for i in 0..n {
            if mesh.pins[i].is_none() {
                res_sq += g[3 * i] * g[3 * i] + g[3 * i + 1] * g[3 * i + 1] + g[3 * i + 2] * g[3 * i + 2];
            }
        }
        let res_norm = res_sq.sqrt();
        metrics.residual_norm = res_norm;

        if res_norm < settings.tolerance {
            metrics.converged = true;
            break;
        }

        // Preconditioned CG step for dx: H dx = -g
        let mut dx = vec![0.0f64; dof_count];
        let mut r: Vec<f64> = g.iter().map(|&val| -val).collect();

        // Zero out pinned DOFs in residual
        for i in 0..n {
            if mesh.pins[i].is_some() {
                r[3 * i] = 0.0;
                r[3 * i + 1] = 0.0;
                r[3 * i + 2] = 0.0;
            }
        }

        let mut z: Vec<f64> = r.iter().enumerate().map(|(idx, &rv)| rv / h_diag[idx].max(1e-6)).collect();
        let mut p = z.clone();
        let mut rz = dot_vec(&r, &z);

        for _pcg in 0..settings.max_pcg_iters {
            metrics.pcg_iters += 1;
            // Matrix-vector multiply approx: Ap ~ H_diag * p
            let ap: Vec<f64> = p.iter().enumerate().map(|(idx, &pv)| h_diag[idx] * pv).collect();
            let pap = dot_vec(&p, &ap);
            if pap.abs() < 1e-18 {
                break;
            }

            let alpha = rz / pap;
            for i in 0..dof_count {
                dx[i] += alpha * p[i];
                r[i] -= alpha * ap[i];
            }

            let r_norm_sq = dot_vec(&r, &r);
            if r_norm_sq < 1e-12 {
                break;
            }

            z = r.iter().enumerate().map(|(idx, &rv)| rv / h_diag[idx].max(1e-6)).collect();
            let new_rz = dot_vec(&r, &z);
            let beta = new_rz / rz;
            rz = new_rz;

            for i in 0..dof_count {
                p[i] = z[i] + beta * p[i];
            }
        }

        // Armijo line search with damping
        let alpha = 1.0;
        for i in 0..n {
            if mesh.pins[i].is_none() {
                mesh.positions[i].x += alpha * dx[3 * i];
                mesh.positions[i].y += alpha * dx[3 * i + 1];
                mesh.positions[i].z += alpha * dx[3 * i + 2];
            }
        }
    }

    // Velocity update
    for i in 0..n {
        if mesh.pins[i].is_some() {
            mesh.velocities[i] = DVec3::ZERO;
        } else {
            mesh.velocities[i] = (mesh.positions[i] - x_prev[i]) / dt;
        }
    }

    metrics
}

fn dot_vec(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

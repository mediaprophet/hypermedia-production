//! Central-difference explicit dynamic time integration.
//!
//! Grounded in dynamic momentum conservation and invariant continuum balance (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// Dynamic explicit time step state.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExplicitState {
    pub time: f64,
    pub step: usize,
    pub displacements: Vec<f64>,
    pub velocities: Vec<f64>,
    pub accelerations: Vec<f64>,
}

/// Central-difference explicit dynamic integrator.
pub struct ExplicitIntegrator {
    pub n_dofs: usize,
    pub dt: f64,
    /// Lumped mass diagonal vector.
    pub lumped_mass: Vec<f64>,
    pub state: ExplicitState,
}

impl ExplicitIntegrator {
    pub fn new(n_dofs: usize, dt: f64, lumped_mass: Vec<f64>) -> Self {
        assert_eq!(lumped_mass.len(), n_dofs);
        Self {
            n_dofs,
            dt,
            lumped_mass,
            state: ExplicitState {
                time: 0.0,
                step: 0,
                displacements: vec![0.0; n_dofs],
                velocities: vec![0.0; n_dofs],
                accelerations: vec![0.0; n_dofs],
            },
        }
    }

    /// Advance one time step using central differences:
    /// 1. a^n = M^{-1} * (F_ext^n - F_int^n)
    /// 2. v^{n+1/2} = v^{n-1/2} + dt * a^n
    /// 3. u^{n+1} = u^n + dt * v^{n+1/2}
    pub fn step<F>(&mut self, f_ext: &[f64], mut f_int: F)
    where
        F: FnMut(&[f64]) -> Vec<f64>,
    {
        let n = self.n_dofs;
        let dt = self.dt;
        let internal_forces = f_int(&self.state.displacements);

        for i in 0..n {
            let m_i = self.lumped_mass[i].max(1e-12);
            let a_i = (f_ext[i] - internal_forces[i]) / m_i;
            self.state.accelerations[i] = a_i;
            self.state.velocities[i] += dt * a_i;
            self.state.displacements[i] += dt * self.state.velocities[i];
        }

        self.state.time += dt;
        self.state.step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_dof_harmonic_oscillator() {
        let m = vec![1.0];
        let k = 100.0;
        let dt = 0.001; // Well within stable limit dt < 2/omega = 2/10 = 0.2
        let mut integ = ExplicitIntegrator::new(1, dt, m);
        integ.state.displacements[0] = 1.0;

        for _ in 0..100 {
            integ.step(&[0.0], |u| vec![k * u[0]]);
        }

        // Energy bounded oscillation
        assert!(integ.state.displacements[0].abs() <= 1.2);
    }
}

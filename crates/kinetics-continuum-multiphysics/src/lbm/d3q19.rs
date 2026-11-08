//! Incompressible D3Q19 two-relaxation-time (TRT) Lattice Boltzmann fluid dynamics.
//!
//! Grounded in hydrodynamic continuum conservation and Navier-Stokes invariance (UDHR Art. 19, 27).

use serde::{Deserialize, Serialize};

/// Discrete velocity vectors for D3Q19 lattice: (x, y, z) in {-1, 0, 1}.
pub const C_DIRS: [[i32; 3]; 19] = [
    [0, 0, 0],   // 0: center
    [1, 0, 0],   // 1
    [-1, 0, 0],  // 2
    [0, 1, 0],   // 3
    [0, -1, 0],  // 4
    [0, 0, 1],   // 5
    [0, 0, -1],  // 6
    [1, 1, 0],   // 7
    [-1, -1, 0], // 8
    [1, -1, 0],  // 9
    [-1, 1, 0],  // 10
    [1, 0, 1],   // 11
    [-1, 0, -1], // 12
    [1, 0, -1],  // 13
    [-1, 0, 1],  // 14
    [0, 1, 1],   // 15
    [0, -1, -1], // 16
    [0, 1, -1],  // 17
    [0, -1, 1],  // 18
];

/// Opposite direction index for bounce-back: OPPOSITE[i] satisfies C_DIRS[i] = -C_DIRS[OPPOSITE[i]].
pub const OPPOSITE: [usize; 19] = [
    0, 2, 1, 4, 3, 6, 5, 8, 7, 10, 9, 12, 11, 14, 13, 16, 15, 18, 17,
];

/// Lattice weights for D3Q19.
pub const WEIGHTS: [f64; 19] = [
    1.0 / 3.0,
    1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0, 1.0 / 18.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
];

/// Equilibrium distribution for incompressible Navier-Stokes.
pub fn equilibrium_d3q19(rho: f64, u: [f64; 3]) -> [f64; 19] {
    let u_sq = u[0] * u[0] + u[1] * u[1] + u[2] * u[2];
    let mut feq = [0.0; 19];

    for i in 0..19 {
        let ci = C_DIRS[i];
        let c_dot_u = ci[0] as f64 * u[0] + ci[1] as f64 * u[1] + ci[2] as f64 * u[2];
        feq[i] = WEIGHTS[i] * (rho + 3.0 * c_dot_u + 4.5 * c_dot_u * c_dot_u - 1.5 * u_sq);
    }

    feq
}

/// D3Q19 Lattice Boltzmann grid.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LbmGrid3D {
    pub nx: usize,
    pub ny: usize,
    pub nz: usize,
    pub kinematic_viscosity: f64,
    /// Populations f[x, y, z, 19].
    pub f: Vec<f64>,
    pub f_coll: Vec<f64>,
    /// Solid boundary voxel mask (true = bounce-back wall).
    pub solid: Vec<bool>,
}

impl LbmGrid3D {
    pub fn new(nx: usize, ny: usize, nz: usize, kinematic_viscosity: f64) -> Self {
        let total_cells = nx * ny * nz;
        let feq0 = equilibrium_d3q19(1.0, [0.0, 0.0, 0.0]);
        let mut f = Vec::with_capacity(total_cells * 19);
        for _ in 0..total_cells {
            f.extend_from_slice(&feq0);
        }
        let f_coll = f.clone();
        let solid = vec![false; total_cells];

        Self {
            nx,
            ny,
            nz,
            kinematic_viscosity,
            f,
            f_coll,
            solid,
        }
    }

    pub fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        (z * self.ny + y) * self.nx + x
    }

    /// Two-relaxation-time (TRT) collision operator with magic parameter Lambda = 1/4.
    pub fn collide(&mut self) {
        let tau_plus = 3.0 * self.kinematic_viscosity + 0.5;
        let omega_plus = 1.0 / tau_plus;
        // Magic parameter Lambda = 1/4 gives exact boundary location
        let tau_minus = 0.5 + 0.25 / (tau_plus - 0.5).max(1e-6);
        let omega_minus = 1.0 / tau_minus;

        for z in 0..self.nz {
            for y in 0..self.ny {
                for x in 0..self.nx {
                    let c_idx = self.idx(x, y, z);
                    if self.solid[c_idx] {
                        continue;
                    }
                    let cell_start = c_idx * 19;
                    let f_cell = &self.f[cell_start..cell_start + 19];

                    // Compute macroscopic density and velocity
                    let mut rho = 0.0;
                    let mut u = [0.0; 3];
                    for i in 0..19 {
                        rho += f_cell[i];
                        let ci = C_DIRS[i];
                        u[0] += f_cell[i] * ci[0] as f64;
                        u[1] += f_cell[i] * ci[1] as f64;
                        u[2] += f_cell[i] * ci[2] as f64;
                    }
                    if rho > 1e-12 {
                        u[0] /= rho;
                        u[1] /= rho;
                        u[2] /= rho;
                    }

                    let feq = equilibrium_d3q19(rho, u);

                    // Symmetric and anti-symmetric decomposition
                    for i in 0..19 {
                        let opp = OPPOSITE[i];
                        let f_sym = 0.5 * (f_cell[i] + f_cell[opp]);
                        let f_asym = 0.5 * (f_cell[i] - f_cell[opp]);
                        let feq_sym = 0.5 * (feq[i] + feq[opp]);
                        let feq_asym = 0.5 * (feq[i] - feq[opp]);

                        self.f_coll[cell_start + i] =
                            f_cell[i] - omega_plus * (f_sym - feq_sym) - omega_minus * (f_asym - feq_asym);
                    }
                }
            }
        }
    }

    /// Stream step with half-way bounce-back on solid boundaries.
    pub fn stream(&mut self) {
        for z in 0..self.nz {
            for y in 0..self.ny {
                for x in 0..self.nx {
                    let c_idx = self.idx(x, y, z);
                    if self.solid[c_idx] {
                        continue;
                    }

                    for i in 0..19 {
                        let ci = C_DIRS[i];
                        let src_x = (x as i32 - ci[0]).rem_euclid(self.nx as i32) as usize;
                        let src_y = (y as i32 - ci[1]).rem_euclid(self.ny as i32) as usize;
                        let src_z = (z as i32 - ci[2]).rem_euclid(self.nz as i32) as usize;

                        let src_idx = self.idx(src_x, src_y, src_z);
                        if self.solid[src_idx] {
                            // Halfway bounce-back
                            let opp = OPPOSITE[i];
                            self.f[c_idx * 19 + i] = self.f_coll[c_idx * 19 + opp];
                        } else {
                            self.f[c_idx * 19 + i] = self.f_coll[src_idx * 19 + i];
                        }
                    }
                }
            }
        }
    }

    /// Step simulation forward one time increment.
    pub fn step(&mut self) {
        self.collide();
        self.stream();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lbm_mass_conservation() {
        let mut grid = LbmGrid3D::new(8, 8, 8, 0.02);
        let initial_mass: f64 = grid.f.iter().sum();
        for _ in 0..10 {
            grid.step();
        }
        let final_mass: f64 = grid.f.iter().sum();
        assert!((final_mass - initial_mass).abs() < 1e-6);
    }
}

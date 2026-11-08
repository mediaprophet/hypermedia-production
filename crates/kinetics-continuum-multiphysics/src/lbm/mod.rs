//! Lattice Boltzmann fluid dynamics: D3Q19 two-relaxation-time (TRT) incompressible solver.

pub mod d3q19;

pub use d3q19::{equilibrium_d3q19, LbmGrid3D, C_DIRS, OPPOSITE, WEIGHTS};

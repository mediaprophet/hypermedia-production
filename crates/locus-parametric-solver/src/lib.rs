//! `locus-parametric-solver` — Categorical 2D sketch constraint solver and CAD typography.
//!
//! Provides:
//! - Weighted minimum-norm damped Gauss-Newton (Levenberg-Marquardt) 2D parametric sketch constraint solver (`solver`)
//! - Conflict detection and minimal conflicting subset reporting
//! - Over-constraint / redundancy check via Jacobian rank
//! - Parametric formula expression evaluator with dependency environment and cycle detection
//! - Heuristic autoconstrain inference across geometric primitives
//! - Single-stroke CAM/plotter stick font polyline evaluator (`typography::stroke`)
//! - AutoCAD MTEXT formatting parser, control code decoder, and word wrap layout engine (`typography::mtext`)

#![forbid(unsafe_code)]

pub mod solver;
pub mod typography;

pub use solver::*;
pub use typography::*;

#[cfg(test)]
mod tests;

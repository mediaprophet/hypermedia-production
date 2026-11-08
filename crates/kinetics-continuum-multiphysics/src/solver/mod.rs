//! Numerical solvers: sparse matrices, Cholesky, LDLT, Lanczos, and nonlinear path following.

pub mod cholesky;
pub mod lanczos;
pub mod ldlt;
pub mod nonlinear;
pub mod sparse;

pub use cholesky::CholeskyFactor;
pub use lanczos::{lanczos_eigensolver, Eigenpair};
pub use ldlt::{Inertia, LdltFactor};
pub use nonlinear::{newton_raphson_solve, EquilibriumStep};
pub use sparse::CsrMatrix;

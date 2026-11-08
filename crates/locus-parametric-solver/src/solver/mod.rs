//! 2D parametric sketch constraint solver.

#![forbid(unsafe_code)]

pub mod auto;
pub mod conflict;
pub mod expr;
pub mod lm;
pub mod model;
pub mod residual;

pub use auto::{InferOptions, infer};
pub use conflict::{SolveError, SolveOptions, SolveReport, check_redundancy, solve_system};
pub use expr::{Env, Expr, ExprError, parse as parse_expr, valid_name};
pub use lm::{Outcome, jacobian, rank, solve, solve_dense};
pub use model::{
    EntityKind, GeomRef, Handle, Model, Shape, SketchArc, SketchCircle, SketchLine, SketchPoint, SketchPolyline,
    Slot, Sub,
};
pub use residual::{Built, Constraint, ConstraintKind, DistAxis, Form};

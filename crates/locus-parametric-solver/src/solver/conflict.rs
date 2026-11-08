//! Conflict detection and redundancy analysis for parametric sketch constraints.

#![forbid(unsafe_code)]

use std::fmt;

use super::expr::{Env, ExprError};
use super::lm::{Outcome, rank, solve};
use super::model::{EntityKind, Handle, Model};
use super::residual::{Built, Constraint, ConstraintKind};

#[derive(Clone, Debug, PartialEq)]
pub enum SolveError {
    Expr { name: String, err: ExprError },
    Invalid { id: u32, msg: String },
    Conflict { ids: Vec<u32> },
    Redundant { ids: Vec<u32> },
    TooLarge,
    Other(String),
}

impl fmt::Display for SolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SolveError::Expr { name, err } => write!(f, "parameter `{name}`: {err}"),
            SolveError::Invalid { id, msg } => write!(f, "constraint {id}: {msg}"),
            SolveError::Conflict { ids } => write!(f, "constraints conflict: {}", ids_text(ids)),
            SolveError::Redundant { ids } => write!(f, "redundant constraint with {}", ids_text(ids)),
            SolveError::TooLarge => write!(f, "constraint system too large to solve"),
            SolveError::Other(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for SolveError {}

fn ids_text(ids: &[u32]) -> String {
    if ids.is_empty() {
        return "existing constraints".into();
    }
    ids.iter().map(|i| format!("#{i}")).collect::<Vec<_>>().join(", ")
}

#[derive(Clone, Debug, Default)]
pub struct SolveOptions {
    pub keep: Vec<Handle>,
    pub prefer_move: Vec<Handle>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SolveReport {
    pub moved: Vec<Handle>,
    pub skipped: Vec<(u32, String)>,
}

/// Evaluates target value for dimensional constraints from environment.
pub fn eval_target(c: &Constraint, env: &Env) -> Result<f64, SolveError> {
    if !c.kind.is_dimensional() {
        return Ok(0.0);
    }
    let expr = if !c.expr.is_empty() {
        &c.expr
    } else if !c.name.is_empty() {
        &c.name
    } else {
        "0"
    };
    env.eval(expr).map_err(|err| SolveError::Expr {
        name: c.name.clone(),
        err,
    })
}

/// Check if a candidate constraint is redundant with existing active constraints.
pub fn check_redundancy(
    m: &Model,
    active: &[Built],
    cand: &Built,
    free: &[usize],
) -> bool {
    let old_rows: usize = active.iter().map(|b| b.rows).sum();
    let new_rows = old_rows + cand.rows;
    if cand.rows == 0 {
        return false;
    }

    let f_old = |x: &[f64], out: &mut Vec<f64>| {
        for b in active {
            b.eval(m, x, out);
        }
    };
    let jac_old = super::lm::jacobian(&f_old, &m.x0, free, old_rows);
    let rk_old = rank(&jac_old, old_rows, free.len());

    let f_new = |x: &[f64], out: &mut Vec<f64>| {
        for b in active {
            b.eval(m, x, out);
        }
        cand.eval(m, x, out);
    };
    let jac_new = super::lm::jacobian(&f_new, &m.x0, free, new_rows);
    let rk_new = rank(&jac_new, new_rows, free.len());

    // If adding non-zero equations does not increase rank, it provides no new independent constraint.
    rk_new <= rk_old
}

/// Solve the constraint system on the given model entities.
pub fn solve_system(
    entities: &[(Handle, EntityKind)],
    constraints: &[Constraint],
    env: &Env,
    opts: &SolveOptions,
) -> Result<(Vec<(Handle, EntityKind)>, SolveReport), SolveError> {
    let m = Model::build(entities);
    if m.slots.is_empty() {
        return Ok((entities.to_vec(), SolveReport::default()));
    }

    let mut report = SolveReport::default();
    let mut built = Vec::new();
    let mut locked_params = Vec::new();

    for c in constraints {
        let target = match eval_target(c, env) {
            Ok(v) => v,
            Err(e) => {
                report.skipped.push((c.id, e.to_string()));
                continue;
            }
        };
        match Built::new(c, &m, &m.x0, target) {
            Ok(b) => {
                if c.kind == ConstraintKind::Fix && b.rows == 0 {
                    // Lock entire entity parameters
                    if let Some(s) = m.slot(c.refs[0].handle) {
                        for p in s.off..s.off + s.len {
                            locked_params.push(p);
                        }
                    }
                }
                built.push(b);
            }
            Err(msg) => {
                report.skipped.push((c.id, msg));
            }
        }
    }

    let free: Vec<usize> = (0..m.x0.len())
        .filter(|p| !locked_params.contains(p))
        .collect();

    let mut weights = vec![1.0; m.x0.len()];
    for &h in &opts.keep {
        if let Some(s) = m.slot(h) {
            for p in s.off..s.off + s.len {
                if let Some(w) = weights.get_mut(p) {
                    *w = 1e6;
                }
            }
        }
    }
    for &h in &opts.prefer_move {
        if let Some(s) = m.slot(h) {
            for p in s.off..s.off + s.len {
                if let Some(w) = weights.get_mut(p) {
                    *w = 0.01;
                }
            }
        }
    }

    let total_rows: usize = built.iter().map(|b| b.rows).sum();
    let f = |x: &[f64], out: &mut Vec<f64>| {
        for b in &built {
            b.eval(&m, x, out);
        }
    };

    let outcome: Outcome = solve(&f, &m.x0, &free, &weights, 1e-8, 200);

    if !outcome.converged && total_rows > 0 {
        // Find conflicting constraints by probing removal
        let mut conflict_ids = Vec::new();
        for i in 0..built.len() {
            let probe_built: Vec<_> = built.iter().enumerate().filter(|&(idx, _)| idx != i).map(|(_, b)| b.clone()).collect();
            let f_probe = |x: &[f64], out: &mut Vec<f64>| {
                for b in &probe_built {
                    b.eval(&m, x, out);
                }
            };
            let o = solve(&f_probe, &m.x0, &free, &weights, 1e-8, 100);
            if o.converged {
                conflict_ids.push(built[i].id);
            }
        }
        if conflict_ids.is_empty() {
            conflict_ids = built.iter().map(|b| b.id).collect();
        }
        return Err(SolveError::Conflict { ids: conflict_ids });
    }

    let mut out_entities = Vec::with_capacity(entities.len());
    for &(h, ref orig) in entities {
        if let Some(updated) = m.extract(&outcome.x, h) {
            if &updated != orig {
                report.moved.push(h);
            }
            out_entities.push((h, updated));
        } else {
            out_entities.push((h, orig.clone()));
        }
    }

    Ok((out_entities, report))
}

//! Bounded algebra and formal-reasoning morphisms.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod boolean_attractors;
mod fourier_optics;
mod grammar;
mod hodgkin_huxley;
mod optics;
mod port_hamiltonian;
mod proof;
mod proposition;
mod sae;
mod sheaf;
mod toy_models;

use crate::{integer, Args, Value};

const SENTINEL: usize = 42 * 1024 * 1024;

fn budget(args: &Args, scratch_bytes: usize) -> Result<usize, String> {
    let requested = args
        .get("arena_budget_bytes")
        .map(|v| integer(v, "arena_budget_bytes"))
        .transpose()?
        .unwrap_or(SENTINEL);
    if requested > SENTINEL || scratch_bytes > requested {
        return Err("Formal solver scratch exceeds the admitted arena budget".into());
    }
    Ok(requested)
}

fn count_value(value: usize) -> Value {
    Value::U64(value as u64)
}

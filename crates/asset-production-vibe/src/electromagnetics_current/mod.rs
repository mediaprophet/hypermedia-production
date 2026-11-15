//! Canonical EMF production, admitted observations and explicit display/audio derivatives.
//! No legacy cell parser, RGB/sigma-as-physics or implicit constitutive inference.
mod clock;
mod compile_host;
mod document;
mod model;
mod projection;
mod raymarch_host;
mod read_host;
mod sonification;
mod sound_host;
pub(crate) use document::{read_memory_bound, read_work_bound, CurrentDocument, ReadLimits};
pub(crate) use model::{compile_generated, CompileLimits, Emitter, GenerationSpec};
pub(crate) use projection::{render_raymarch, render_slice, GridAtTime, ProjectionLimits};
pub(crate) use read_host::time_slice;
#[cfg(test)]
mod host_tests;
#[cfg(test)]
mod tests;

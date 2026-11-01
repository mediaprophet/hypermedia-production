//! Application-neutral editorial time and transition editing.
//!
//! The selected FilmCraft operations are adapted to a small public surface with
//! no application model, codec, renderer, file, GPU, or runtime dependencies.
//! `Tick` uses FilmCraft's exact media scale. Simulation time remains owned by
//! simulation systems; this crate describes editorial positions and frame
//! boundaries.

mod timebase;
mod transition;

pub use timebase::{FrameRate, Tick, TimeRange, TICKS_PER_SECOND};
pub use transition::{
    ClipSpan, TimelineBounds, TransitionAlignment, TransitionContext, TransitionEdge,
    TransitionEditError, TransitionSpan, ValidatedTransitionSpan, drag_transition_span,
    set_transition_span, start_for_alignment, transition_alignment, transition_bounds,
};

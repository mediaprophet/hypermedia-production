//! `layout-box-autolayout` — UI-agnostic categorical Flexbox and CSS Grid auto-layout engine.
//!
//! Provides:
//! - Full Flexbox auto-layout (`LayoutDirection::{Horizontal, Vertical}`)
//! - CSS Grid auto-layout with fractional `fr` tracks (`LayoutDirection::Grid`, `GridTrack`)
//! - Sizing modes: `Fixed`, `Hug` (fit content), `Fill` (share free space)
//! - Free space distribution algorithm (`distribute`) respecting min/max bounds
//! - Multi-line wrapping with content alignment (`ContentAlign`) and cross alignment (`CrossAlign`)
//! - Text baseline alignment (`CrossAlign::Baseline`)
//! - Constraint propagation on frame resize (`HConstraint`, `VConstraint`, `apply_constraints`)
//! - Top-level `solve_autolayout` orchestrator with nested container support

#![forbid(unsafe_code)]

pub mod constraints;
pub mod flex;
pub mod frame;
pub mod grid;
pub mod solver;
pub mod types;

pub use constraints::apply_constraints;
pub use flex::{distribute, main_offsets, solve_flex};
pub use frame::{AutoLayoutFrame, LayoutNode};
pub use grid::{GridCell, grid_place, resolve_tracks, solve_grid};
pub use solver::solve_autolayout;
pub use types::{
    CellAlign, ContentAlign, CrossAlign, GridTrack, HConstraint, LayoutDirection, MainAlign, Padding, Rect,
    SizingMode, VConstraint, clamp_size,
};

#[cfg(test)]
mod tests;

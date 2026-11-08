//! Hyphenation and line-breaking rules.

pub mod kinsoku;
pub mod liang;

pub use kinsoku::{NO_END, NO_START, is_east_asian, line_break_opportunities};
pub use liang::Patterns;

//! Resource limits and caps for academic TeX parsing.

#![forbid(unsafe_code)]

/// Longest command/identifier name kept; longer names are truncated.
pub const MAX_NAME: usize = 256;
/// Longest captured argument text (title, filename, bib value).
pub const MAX_ARG: usize = 64 * 1024;
/// Deepest brace/environment nesting tracked; deeper input is reported as an issue and skipped.
pub const MAX_DEPTH: usize = 512;

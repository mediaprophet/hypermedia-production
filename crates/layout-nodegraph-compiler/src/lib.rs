//! `layout-nodegraph-compiler` — Typed dataflow node graph compiler and unit-aware math expression parser.
//!
//! Provides:
//! - Pure-Rust typed dataflow compiler: `NodeNetwork` -> flattened `ProtoNetwork`
//! - Composite subnetwork inlining (`flatten_network`)
//! - Common Subexpression Elimination (`cse_deduplicate`) via deterministic Merkle hash trees
//! - Dead-node pruning (`prune_dead_nodes`)
//! - Topological sorting and dependency cycle detection
//! - Unit-aware algebraic expression parser (`math_parser`): implicit multiplication, scientific notation, constants (`pi`, `e`, `tau`, `phi`), unit conversions (`px`, `pt`, `mm`, `in`, `deg`, `rad`, `s`, `ms`, `Hz`).

#![forbid(unsafe_code)]

pub mod compiler;
pub mod math_parser;

pub use compiler::*;
pub use math_parser::*;

#[cfg(test)]
mod tests;

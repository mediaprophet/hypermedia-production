//! Preflight compliance evaluator for PDF/A (archiving) and PDF/X (prepress print).

#![forbid(unsafe_code)]

pub mod evaluator;

pub use evaluator::{
    PreflightIssue, PreflightReport, PreflightSnapshot, StandardProfile, evaluate_preflight,
};

//! Fail-closed semantic redaction and metadata sanitization engine.

#![forbid(unsafe_code)]

pub mod sanitizer;
pub mod scanner;
pub mod verifier;

pub use sanitizer::{HiddenCategory, SanitizeManifest, SanitizeReport, sanitize_document};
pub use scanner::{
    PiiKind, PiiMatch, luhn_check, scan_all_pii, scan_credit_cards, scan_emails, scan_phones,
    scan_ssns, scan_tax_ids,
};
pub use verifier::{
    RedactionTarget, VerificationReport, apply_semantic_redaction, verify_zero_residue,
};

//! Academic bibliography utilities: identifier parsing, format conversion, and audit.

#![forbid(unsafe_code)]

pub mod audit;
pub mod identifiers;
pub mod import;

pub use audit::{audit_bib, AuditFinding, AuditReport, Severity};
pub use identifiers::{parse_identifier, Ident};
pub use import::{convert, emit, norm_title, sniff, Format};

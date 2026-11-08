//! `discourse-pdf-compliance` — PDF/UA accessibility, semantic redaction, PAdES signatures, AcroForms, and PDF/A-PDF/X preflight.
//!
//! Grounded in international human rights instruments and informatics fiduciary standards:
//! - Universal Declaration of Human Rights (UDHR) Article 12: Protection of natural persons from arbitrary interference with privacy and correspondence through fail-closed redaction and zero-residue verification.
//! - UDHR Article 19 & ICCPR Article 19: Universal right to impart and receive accessible digital discourse regardless of frontiers (PDF/UA and WCAG 32-rule verification).
//! - International Covenant on Economic, Social and Cultural Rights (ICESCR) Article 15: Protection of moral and material interests resulting from scientific, literary, or artistic production through authentic digital signatures (ETSI PAdES B-B).
//! - Informatics Fiduciary Duties: Strict duty of loyalty, confidentiality, care, and fail-closed security for document handling.

#![forbid(unsafe_code)]

pub mod a11y;
pub mod forms;
pub mod preflight;
pub mod redact;
pub mod sign;

pub use a11y::*;
pub use forms::*;
pub use preflight::*;
pub use redact::*;
pub use sign::*;

#[cfg(test)]
mod tests;

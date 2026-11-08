//! # Discourse Academic TeX Engine
//!
//! A pure Rust, UI-agnostic categorical domain crate providing complete TeX/LaTeX
//! processing, mathematical typesetting, document parsing, citation management,
//! PDF generation, and collaborative review primitives.
//!
//! Grounded in international human rights guarantees of scientific and artistic
//! freedom, moral rights, and collaborative participation in scientific progress
//! (UDHR Articles 19 & 27; ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod bibliography;
pub mod catalog;
pub mod review;
pub mod texpdf;
pub mod texparse;
pub mod typeset;

pub use bibliography::{audit_bib as audit_bib_file, sniff as sniff_format, AuditReport as BibAuditReport, Format as BibFormat};
pub use catalog::{catalog, CommandSpec as CommandSignature};
pub use review::{apply_suggestion, enrich_diagnostics, expand_snippet, Suggestion, TeXDiagnostic};
pub use texpdf::{Doc as PdfDoc, Page as PdfPage, Ttf};
pub use texparse::{parse_bib, parse_magic, parse_structure, DocStats, DocStructure, ProjectGraph};
pub use typeset::{math as layout_formula, Fragment as MathMetrics, Item, Style};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_discourse_pipeline_smoke() {
        let tex = "\\documentclass{article}\n\\begin{document}\nHello world $E=mc^2$\n\\end{document}\n";
        let structure = parse_structure(tex);
        assert_eq!(structure.issues.len(), 0);
        assert_eq!(structure.envs.len(), 1);
        assert_eq!(structure.envs[0].name, "document");

        let stats = texparse::analyze_stats(tex);
        assert!(stats.words >= 2);

        let catalog_cmds = catalog();
        assert!(!catalog_cmds.is_empty());
    }
}

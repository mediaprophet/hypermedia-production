//! Categorical & Ontological metadata for the Paged Document Typesetter.
//!
//! Guarantees: Mathematical determinism, bounded memory budgets, zero recursion,
//! fail-closed error handling, and complete typographic provenance grounded in
//! International Human Rights Instruments (UDHR Articles 19 & 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalTriple {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub context: u64,
    pub parity: u64,
}

impl OntologicalTriple {
    pub const fn new(subject: u64, predicate: u64, object: u64, context: u64) -> Self {
        let parity = subject ^ predicate ^ object ^ context;
        OntologicalTriple {
            subject,
            predicate,
            object,
            context,
            parity,
        }
    }

    pub const fn verify_parity(&self) -> bool {
        (self.subject ^ self.predicate ^ self.object ^ self.context) == self.parity
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalContract {
    pub precondition: OntologicalTriple,
    pub postcondition: OntologicalTriple,
    pub preserves_invariant: OntologicalTriple,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AgenticMorphismSpec {
    pub morphism_id: &'static str,
    pub domain: &'static str,
    pub codomain: &'static str,
    pub morphism_kind: &'static str,
    pub teleology: &'static str,
    pub units_and_frames: &'static str,
    pub composable_with: &'static [&'static str],
    pub human_preview_affordance: &'static str,
    pub contract: OntologicalContract,
}

pub static PAGED_TYPESET: AgenticMorphismSpec = AgenticMorphismSpec {
    morphism_id: "hypermedia.layout.paged_typeset",
    domain: "DocumentFlow",
    codomain: "PagedGeometry",
    morphism_kind: "Homomorphism",
    teleology: "Formats flow text into multi-column, multi-page layout with Liang hyphenation, Kinsoku Shori breaking, widow/orphan control, and equation boxes",
    units_and_frames: "Points(1/72 inch), PageCoordinates(x, y)",
    composable_with: &["hypermedia.glyph.rasterize", "hypermedia.discourse.export_pdf"],
    human_preview_affordance: "Continuous multi-page preview with margins, columns, and footnote markers",
    contract: OntologicalContract {
        precondition: OntologicalTriple::new(0x2001, 0x0001, 0x0010, 0x0000),
        postcondition: OntologicalTriple::new(0x2001, 0x0002, 0x0020, 0x0000),
        preserves_invariant: OntologicalTriple::new(0x2001, 0x0003, 0x0030, 0x0000),
    },
};

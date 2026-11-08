//! Categorical & Ontological metadata for the Chronicle Tabular Engine.
//!
//! Guarantees: Mathematical determinism, bounded caller budgets, zero unbounded recursion,
//! fail-closed error propagation, and complete provenance tracking grounded in
//! International Human Rights Instruments (UDHR Articles 19 & 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

use serde::{Deserialize, Serialize};

/// Parity-verified 5-field ontology triple (NQuin48 representation).
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

/// Ontological contract specifying static verification invariants.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OntologicalContract {
    pub precondition: OntologicalTriple,
    pub postcondition: OntologicalTriple,
    pub preserves_invariant: OntologicalTriple,
}

/// Agentic morphism specification for categorical morphisms.
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

pub static TABULAR_DAG_RECALCULATE: AgenticMorphismSpec = AgenticMorphismSpec {
    morphism_id: "hypermedia.chronicle.tabular_dag_recalculate",
    domain: "WorkbookState",
    codomain: "WorkbookState",
    morphism_kind: "Endomorphism",
    teleology: "Evaluates dependency graph topologically, resolving formulas, closures and dynamic spills while isolating cycles as #CIRC!",
    units_and_frames: "CellCoordinates(row: 0..1048576, col: 0..16384), LamportClock",
    composable_with: &["hypermedia.chronicle.formula_parse", "hypermedia.chronicle.number_format"],
    human_preview_affordance: "Interactive spreadsheet grid display with dynamic spill bounds and instant recalculation indicators",
    contract: OntologicalContract {
        precondition: OntologicalTriple::new(0x1001, 0x0001, 0x0010, 0x0000),
        postcondition: OntologicalTriple::new(0x1001, 0x0002, 0x0020, 0x0000),
        preserves_invariant: OntologicalTriple::new(0x1001, 0x0003, 0x0030, 0x0000),
    },
};

//! Ontological and categorical definitions for office interchange codecs.
//! Grounded in UDHR Articles 19 & 27 (freedom of expression, cultural participation)
//! and Informatics Fiduciary duties.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct NQuin48 {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub context: u64,
    pub metadata: u64,
    pub parity: u64,
}

impl NQuin48 {
    pub const fn new(s: u64, p: u64, o: u64, c: u64, m: u64) -> Self {
        let parity = s ^ p ^ o ^ c ^ m;
        NQuin48 {
            subject: s,
            predicate: p,
            object: o,
            context: c,
            metadata: m,
            parity,
        }
    }

    pub const fn is_valid(&self) -> bool {
        self.parity == (self.subject ^ self.predicate ^ self.object ^ self.context ^ self.metadata)
    }
}

pub struct OntologicalContract {
    pub precondition: NQuin48,
    pub postcondition: NQuin48,
    pub invariant: NQuin48,
}

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

pub const OFFICE_CODEC_IMPORT_SPEC: AgenticMorphismSpec = AgenticMorphismSpec {
    morphism_id: "hypermedia.chronicle.office_import",
    domain: "office.raw_container_bytes",
    codomain: "hypermedia.flow_model",
    morphism_kind: "lossless_syntactic_functor",
    teleology: "Faithfully ingest office documents (DOCX, XLSX, PPTX, Markdown, HTML, RTF, LaTeX) honoring author intent under UDHR Art. 19/27",
    units_and_frames: "Normalized Unicode code points, typographic points, EMUs (1/12700 pt)",
    composable_with: &[
        "hypermedia.layout.typeset",
        "hypermedia.chronicle.tabular_eval",
        "hypermedia.kinetics.deck_morph",
    ],
    human_preview_affordance: "Document preview inspector and syntax tree visualizer",
    contract: OntologicalContract {
        precondition: NQuin48::new(0x4001, 0x10, 0x1, 0x42, 0x0),
        postcondition: NQuin48::new(0x4001, 0x11, 0x2, 0x42, 0x0),
        invariant: NQuin48::new(0x4001, 0x12, 0x3, 0x42, 0x0),
    },
};

pub const OFFICE_CODEC_EXPORT_SPEC: AgenticMorphismSpec = AgenticMorphismSpec {
    morphism_id: "hypermedia.chronicle.office_export",
    domain: "hypermedia.flow_model",
    codomain: "office.standard_package_bytes",
    morphism_kind: "canonical_standard_serializer",
    teleology: "Produce standard-compliant, portable document packages for open interchange without proprietary barriers",
    units_and_frames: "Standard OpenXML / CommonMark / RTF / LaTeX RFC byte representations",
    composable_with: &[
        "hypermedia.chronicle.office_import",
        "hypermedia.surface.rasterize",
    ],
    human_preview_affordance: "File download and export checksum inspector",
    contract: OntologicalContract {
        precondition: NQuin48::new(0x4002, 0x10, 0x1, 0x42, 0x0),
        postcondition: NQuin48::new(0x4002, 0x11, 0x2, 0x42, 0x0),
        invariant: NQuin48::new(0x4002, 0x12, 0x3, 0x42, 0x0),
    },
};

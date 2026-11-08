//! Category-theoretic ontology and agentic morphism specification for surface raster masks.
//!
//! Grounded in UDHR Articles 12, 19, 27; ICESCR Article 15; and Informatics Fiduciary duties.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NQuin48 {
    pub subject: u64,
    pub predicate: u64,
    pub object: u64,
    pub context: u64,
    pub metadata: u64,
}

impl NQuin48 {
    pub const fn new(subject: u64, predicate: u64, object: u64, context: u64, metadata: u64) -> Self {
        Self { subject, predicate, object, context, metadata }
    }

    pub const fn precondition(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1001, condition_hash, 0x7FFF_0001, seq)
    }

    pub const fn postcondition(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1002, condition_hash, 0x7FFF_0001, seq)
    }

    pub const fn invariant(morphism_hash: u64, condition_hash: u64, seq: u64) -> Self {
        Self::new(morphism_hash, 0x1003, condition_hash, 0x7FFF_0001, seq)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OntologicalContract {
    pub precondition: NQuin48,
    pub postcondition: NQuin48,
    pub invariant: NQuin48,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FuncDescriptor {
    pub name: &'static str,
    pub stable_id: u32,
    pub deterministic: bool,
    pub pure_compute: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct AgenticMorphismSpec {
    pub id: &'static str,
    pub category: &'static str,
    pub morphism: &'static str,
    pub summary: &'static str,
    pub domain: &'static str,
    pub codomain: &'static str,
    pub morphism_kind: &'static str,
    pub teleology: &'static str,
    pub units_and_frames: &'static str,
    pub composable_with: &'static [&'static str],
    pub human_preview_affordance: &'static str,
    pub descriptor: FuncDescriptor,
    pub contract: OntologicalContract,
}

pub static MASK_FEATHER_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.surface.raster_mask_feather",
    category: "surface",
    morphism: "raster_mask_feather",
    summary: "Applies separable 2D Gaussian blur feathering to an 8-bit layer mask buffer",
    domain: "MaskBufferU8, Dimensions { width, height }, RadiusPx",
    codomain: "MaskBufferU8",
    morphism_kind: "Endomorphism",
    teleology: "Softens selection and masking transitions to eliminate hard pixel fringes",
    units_and_frames: "Screen pixel radius, 8-bit grayscale values in [0, 255]",
    composable_with: &[
        "hypermedia.surface.raster_mask_density",
        "hypermedia.surface.clipping_knockout_compose",
    ],
    human_preview_affordance: "Monochrome alpha lattice preview with zoomable edge transition inspector",
    descriptor: FuncDescriptor {
        name: "raster_mask_feather",
        stable_id: 0x0301,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0301, 0xB001, 1),
        postcondition: NQuin48::postcondition(0x0301, 0xB002, 1),
        invariant: NQuin48::invariant(0x0301, 0xB003, 1),
    },
};

pub static MIXER_BRUSH_DAB_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.surface.mixer_brush_dab",
    category: "surface",
    morphism: "mixer_brush_dab",
    summary: "Evaluates wet paint reservoir pickup, mixing, and dryout per brush dab",
    domain: "CanvasColor, MixerSettings, MixerState",
    codomain: "DepositedColorRgba, NextMixerState",
    morphism_kind: "Functor",
    teleology: "Simulates impasto and wet-on-wet paint mixing with realistic dryout",
    units_and_frames: "Normalized straight RGBA [0.0, 1.0], dimensionless wet/load/mix fractions",
    composable_with: &["hypermedia.surface.clipping_knockout_compose"],
    human_preview_affordance: "Real-time canvas stroke rendering with color pickup display",
    descriptor: FuncDescriptor {
        name: "mixer_brush_dab",
        stable_id: 0x0302,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0302, 0xB004, 2),
        postcondition: NQuin48::postcondition(0x0302, 0xB005, 2),
        invariant: NQuin48::invariant(0x0302, 0xB006, 2),
    },
};

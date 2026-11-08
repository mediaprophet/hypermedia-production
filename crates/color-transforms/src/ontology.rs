//! Category-theoretic ontology and agentic morphism specification for radiance color transforms.
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

pub static ICC_TRANSFORM_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.radiance.icc_transform",
    category: "radiance",
    morphism: "icc_transform",
    summary: "Converts pixel buffers between ICC v2/v4 color profiles via PCS D50 with optional BPC",
    domain: "ImageBuffer { channels: N, profile: IccProfile }",
    codomain: "ImageBuffer { channels: M, profile: IccProfile }",
    morphism_kind: "Endomorphism",
    teleology: "Provides colorimetric fidelity across disparate display and print color spaces without visual clipping",
    units_and_frames: "PCS XYZ/Lab D50 illuminant, IEEE-754 f32 [0.0, 1.0] or 8-bit quantized [0, 255]",
    composable_with: &[
        "hypermedia.radiance.gamut_check",
        "hypermedia.surface.raster_mask_compose",
    ],
    human_preview_affordance: "Colorimetric split-view comparison against reference display profile",
    descriptor: FuncDescriptor {
        name: "icc_transform",
        stable_id: 0x0401,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0401, 0xC001, 1),
        postcondition: NQuin48::postcondition(0x0401, 0xC002, 1),
        invariant: NQuin48::invariant(0x0401, 0xC003, 1),
    },
};

pub static GAMUT_CHECK_MORPHISM: AgenticMorphismSpec = AgenticMorphismSpec {
    id: "hypermedia.radiance.gamut_check",
    category: "radiance",
    morphism: "gamut_check",
    summary: "Evaluates Delta-E76 out-of-gamut masks for proofing against target output profiles",
    domain: "ImageBuffer, ProofProfile, ThresholdDeltaE",
    codomain: "MaskBufferU8",
    morphism_kind: "Functor",
    teleology: "Identifies out-of-gamut reproduction hazards before physical press or display emission",
    units_and_frames: "CIE Delta-E76 distance, binary U8 mask (0 = in-gamut, 255 = out-of-gamut)",
    composable_with: &["hypermedia.radiance.icc_transform"],
    human_preview_affordance: "High-contrast warning overlay highlighting out-of-gamut regions",
    descriptor: FuncDescriptor {
        name: "gamut_check",
        stable_id: 0x0402,
        deterministic: true,
        pure_compute: true,
    },
    contract: OntologicalContract {
        precondition: NQuin48::precondition(0x0402, 0xC004, 2),
        postcondition: NQuin48::postcondition(0x0402, 0xC005, 2),
        invariant: NQuin48::invariant(0x0402, 0xC006, 2),
    },
};

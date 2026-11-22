//! WP-32 scientific and studio interchange descriptors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId};

pub(super) const START: usize = super::BASE_MORPHISM_COUNT
    + super::morphisms_10::BATCH.len()
    + super::morphisms_21::BATCH.len()
    + super::morphisms_22::BATCH.len()
    + super::morphisms_23::BATCH.len()
    + super::morphisms_11::BATCH.len()
    + super::morphisms_24::BATCH.len()
    + super::morphisms_12::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "qualia.interchange.transcode_scientific_cae_bio_and_qudt",
        aliases: &[],
        category: "interchange",
        morphism: "transcode_scientific_cae_bio_and_qudt",
        owner: "QualiaDB and Hypermedia Production",
        summary: "Normalize admitted QUDT v3.5.3 unit declarations and seal their verified identities in Q42.",
        domain: "DeclaredQudtUnitDimensionAndMultiplier",
        codomain: "CanonicalQudtDeclarationsAndQ42Receipt",
        morphism_kind: "Functor",
        teleology: "Preserve dimensional and numeric unit meaning when authoring scientific data and Vibe arguments.",
        units_and_frames: "QUDT 3.5.3 dimension vector IRIs and binary64 conversion multipliers.",
        arguments: "mode=normalize_qudt_unit, units=[{unit,dimension,conversion_multiplier}].",
        composable_with: &[],
        human_preview_affordance: "Inspect canonical unit and dimension IRIs and the corresponding Q42 receipt.",
        abi_descriptor: morphism_func!(
            "transcode_scientific_cae_bio_and_qudt",
            "https://qualiadb.org/ext/qualia/interchange#transcode_scientific_cae_bio_and_qudt",
            44, 322, "(DeclaredQudtUnitDimensionAndMultiplier) -> CanonicalQudtDeclarationsAndQ42Receipt", START + 0,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicExact, ReversibilityKind::PureNoState
        ),
    },
];

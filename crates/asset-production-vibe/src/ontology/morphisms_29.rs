//! WP-35 inclusion, trust and accessibility descriptors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId};

const START: usize = super::morphisms_25::START + super::morphisms_25::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "qualia.accessibility.synthesize_iso17049_tactile_braille",
        aliases: &[],
        category: "accessibility",
        morphism: "synthesize_iso17049_tactile_braille",
        owner: "QualiaDB and Hypermedia Production",
        summary: "Author bounded spherical-cap tactile relief from pretranscribed six-dot Braille and seal the plate with accessible CBLD labels.",
        domain: "AuthoredBrailleCellsAndPhysicalPlateParameters",
        codomain: "SdfbTactilePlateAndAccessibleSourceReceipt",
        morphism_kind: "Functor",
        teleology: "Maintain exact authored Braille spacing and source text through physical field encoding.",
        units_and_frames: "Authored millimetres, encoded metres, Unicode Braille pattern order and explicit language tags.",
        arguments: "braille_lines, accessible_text, entity_iri, dot_diameter_mm, dot_height_mm, plate_thickness_mm, and optional language, transcription system, spacing and voxel resolution.",
        composable_with: &[],
        human_preview_affordance: "Inspect tactile geometry, packed distances and accessible text before physical assessment.",
        abi_descriptor: morphism_func!(
            "synthesize_iso17049_tactile_braille",
            "https://qualiadb.org/ext/qualia/accessibility#synthesize_iso17049_tactile_braille",
            46, 318, "(AuthoredBrailleCellsAndPhysicalPlateParameters) -> SdfbTactilePlateAndAccessibleSourceReceipt", START + 0,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
];

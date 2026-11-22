//! WP-38 production descriptors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId};

pub(super) const START: usize = super::morphisms_26::START + super::morphisms_26::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.synthesize_laban_choreography",
        aliases: &[],
        category: "kinetics",
        morphism: "synthesize_laban_choreography",
        owner: "QualiaDB and Hypermedia Production",
        summary: "Retime an admitted SKL1 clip against authored beats and apply bounded Laban Effort path and time styling on a matching SKL2 rig.",
        domain: "AdmittedSkeletalClipAndAuthoredBeatPhrases",
        codomain: "RetimedSkeletalClipAndQ42EffortReceipt",
        morphism_kind: "Functor",
        teleology: "Retain source joint identity, authored beat endpoints, and the permitted spatial deviation.",
        units_and_frames: "Seconds, metres and exact PGA motor conventions from the mounted rig.",
        arguments: "source, rig, clip_iri, beat_times_s, phrases, optional samples_per_phrase and max_root_path_delta_m.",
        composable_with: &[],
        human_preview_affordance: "Inspect retimed motors and their authored effort phrases and beat bindings.",
        abi_descriptor: morphism_func!(
            "synthesize_laban_choreography",
            "https://qualiadb.org/ext/hypermedia/kinetics#synthesize_laban_choreography",
            7, 309, "(AdmittedSkeletalClipAndAuthoredBeatPhrases) -> RetimedSkeletalClipAndQ42EffortReceipt", START + 0,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
];

//! WP-31 categorical and formal kernel descriptors.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId};

pub(super) const START: usize = super::morphisms_33::START + super::morphisms_33::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "qualia.category.compute_pushout_pullback_and_kan_extension",
        aliases: &[],
        category: "category",
        morphism: "compute_pushout_pullback_and_kan_extension",
        owner: "QualiaDB and Hypermedia Production",
        summary: "Compute a finite directed attributed multigraph pullback or pushout and verify the projection or quotient witness.",
        domain: "FiniteAttributedMultigraphDiagram",
        codomain: "UniversalConstructionAndQ42MapWitness",
        morphism_kind: "Functor",
        teleology: "Preserve authored incidence and attributes and retain the exact structural map from each leg.",
        units_and_frames: "Zero-based vertex and edge slots with canonical RDF IRI attributes.",
        arguments: "operation=pullback|pushout, base, left, right, left_map, right_map, optional arena_budget_bytes.",
        composable_with: &[],
        human_preview_affordance: "Inspect the composed graph and its admitted projection or injection maps.",
        abi_descriptor: morphism_func!(
            "compute_pushout_pullback_and_kan_extension",
            "https://qualiadb.org/ext/qualia/category#compute_pushout_pullback_and_kan_extension",
            45, 281, "(FiniteAttributedMultigraphDiagram) -> UniversalConstructionAndQ42MapWitness", START + 0,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicExact, ReversibilityKind::PureNoState
        ),
    },
];

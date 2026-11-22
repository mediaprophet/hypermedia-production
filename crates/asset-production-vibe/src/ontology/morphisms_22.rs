//! WP-28 production morphism descriptors.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind,
    ReversibilityKind, StableFuncId};

const START: usize = super::BASE_MORPHISM_COUNT + super::morphisms_10::BATCH.len() + super::morphisms_21::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "qualia.governance.evaluate_semantic_instruments_and_tk_bc",
        aliases: &[],
        category: "governance",
        morphism: "evaluate_semantic_instruments_and_tk_bc",
        owner: "QualiaDB/semantic_instruments + governance",
        summary: "Evaluate semantic instruments, TK/BC labels, consent snapshots and geospatial rules, or prepare a collectable.",
        domain: "InstrumentOrPolicyParameters | HmcBundle",
        codomain: "InstrumentOrPolicyResult",
        morphism_kind: "Functor",
        teleology: "Evaluate semantic instruments, TK/BC labels, consent snapshots and geospatial rules, or prepare a collectable.",
        units_and_frames: "Consent and steward decisions use explicitly supplied evidence and time.",
        arguments: "op, parameters. Ops: evaluate, rpl, tk_bc, collectable_build, collectable_open, consent_snapshot, steward_unlock, location_triggers.",
        composable_with: &[],
        human_preview_affordance: "Inspect the result and verify the input evidence or artifact.",
        abi_descriptor: morphism_func!(
            "evaluate_semantic_instruments_and_tk_bc", "https://qualiadb.org/ext/qualia/governance#evaluate_semantic_instruments_and_tk_bc", 24, 256,
            "(InstrumentOrPolicyParameters | HmcBundle) -> InstrumentOrPolicyResult", START
        ),
    },
    AgenticMorphismSpec {
        id: "qualia.cooperative.evaluate_nine_stage_lifecycle_and_abac",
        aliases: &[],
        category: "cooperative",
        morphism: "evaluate_nine_stage_lifecycle_and_abac",
        owner: "QualiaDB/qualia-cooperative-core",
        summary: "Evaluate contribution lifecycle, obligation readiness, ledger replay, delegation and harm traces.",
        domain: "ContributionLedgerOrDelegationParameters",
        codomain: "CooperativeEvaluation",
        morphism_kind: "Functor",
        teleology: "Evaluate contribution lifecycle, obligation readiness, ledger replay, delegation and harm traces.",
        units_and_frames: "Exact integer accounting, dated state transitions, and explicit delegation context.",
        arguments: "op, parameters. Ops: compile_stages, derive_obligation, transition_readiness, merge_ledger, agency_permits, harm_trace, bankers_round.",
        composable_with: &[],
        human_preview_affordance: "Inspect the result and verify the input evidence or artifact.",
        abi_descriptor: morphism_func!(
            "evaluate_nine_stage_lifecycle_and_abac", "https://qualiadb.org/ext/qualia/cooperative#evaluate_nine_stage_lifecycle_and_abac", 25, 257,
            "(ContributionLedgerOrDelegationParameters) -> CooperativeEvaluation", START + 1
        ),
    },
    AgenticMorphismSpec {
        id: "qualia.anatomy.seal_cardiac_m64_emf_and_work_graph",
        aliases: &[],
        category: "anatomy",
        morphism: "seal_cardiac_m64_emf_and_work_graph",
        owner: "QualiaDB/qualia-seal-10d + work-graph",
        summary: "Seal retained cardiac geometry, produce matter companion volumes, or index present work-graph evidence.",
        domain: "RetainedCardiacAsset | OccupancyLattice | RetainedSources",
        codomain: "CardiacBundle | MatterAndSemanticAssets | PresentWorkGraph",
        morphism_kind: "Functor",
        teleology: "Preserve authored skeletal and semantic identities when packaging anatomical assets.",
        units_and_frames: "Anatomical geometry uses meters and retained occupancy bytes. Evidence comes from explicit source handles.",
        arguments: "op, parameters. Ops: cardiac_seal, heart_m64, bone_m64, work_graph_present.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the sealed companion resources or the observed-work report.",
        abi_descriptor: morphism_func!(
            "seal_cardiac_m64_emf_and_work_graph", "https://qualiadb.org/ext/qualia/anatomy#seal_cardiac_m64_emf_and_work_graph",
            27, 260, "(CardiacOrWorkGraphOperation, Parameters) -> AnatomyResult", START + 2
        ),
    },
    AgenticMorphismSpec {
        id: "qualia.wellfair.evaluate_personal_health_and_guardianship",
        aliases: &[],
        category: "wellfair",
        morphism: "evaluate_personal_health_and_guardianship",
        owner: "WellFare core and QualiaDB organic chemistry",
        summary: "Evaluate bounded health and guardianship fixtures and molecular or green-chemistry metrics.",
        domain: "AuthoredWellFareFixture | MoleculeOrMassInputs",
        codomain: "FixtureProjection | ChemistryMetrics",
        morphism_kind: "Functor",
        teleology: "Evaluate synthetic case histories and deterministic chemistry quantities.",
        units_and_frames: "Fixture timestamps use epoch seconds and durations use minutes. Chemistry uses declared mass and molar mass units.",
        arguments: "op, parameters. Ops: sleep_debt_fixture, clinical_claim_fixture, therapy_note_fixture, guardianship_veto_fixture, organic_descriptors, green_mass_metrics.",
        composable_with: &[],
        human_preview_affordance: "Inspect the fixture projection or computed chemistry quantities.",
        abi_descriptor: morphism_func!(
            "evaluate_personal_health_and_guardianship",
            "https://qualiadb.org/ext/qualia/wellfair#evaluate_personal_health_and_guardianship",
            34, 258, "(FixtureOrChemistryOperation, Parameters) -> FixtureOrChemistryResult", START + 3
        ),
    },
    AgenticMorphismSpec {
        id: "qualia.network.dispatch_qdnf_ilp_ebpf_and_os_services",
        aliases: &[],
        category: "network",
        morphism: "dispatch_qdnf_ilp_ebpf_and_os_services",
        owner: "QualiaDB/QDNF policies and geospatial raster codecs",
        summary: "Evaluate supplied QDNF policy scenarios or seal a retained projected GeoTIFF raster.",
        domain: "QdnfScenario | RetainedProjectedGeoTiff",
        codomain: "PolicyDecision | Encoded10d",
        morphism_kind: "Functor",
        teleology: "Preserve explicit consent boundaries and authored raster identity.",
        units_and_frames: "Projected rasters use metres and QDNF records use authored domain and policy identities.",
        arguments: "op, parameters. Ops: geotiff_to_10d_fiel, qdnf_dns_front_door, qdnf_contact_gate, identifier_fabric_guard, identifier_referent_guard, identifier_locator_guard.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the policy verdict or verify the sealed raster asset.",
        abi_descriptor: morphism_func!(
            "dispatch_qdnf_ilp_ebpf_and_os_services",
            "https://qualiadb.org/ext/qualia/network#dispatch_qdnf_ilp_ebpf_and_os_services",
            21, 259, "(ScenarioOrRaster, Parameters) -> DecisionOrAsset", START + 4
        ),
    },
];

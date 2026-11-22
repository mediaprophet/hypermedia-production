//! Production descriptors for section and semantic motion authoring.

use super::{AgenticMorphismSpec, DEFAULT_ENVELOPE, DEFAULT_PARAMS};
use vibescript_abi::{
    DeterminismGuarantee, EffectSet, FuncDescriptor, PrimitiveKind, ReversibilityKind, StableFuncId,
};

const START: usize = super::BASE_MORPHISM_COUNT
    + super::morphisms_10::BATCH.len()
    + super::morphisms_21::BATCH.len()
    + super::morphisms_22::BATCH.len()
    + super::morphisms_23::BATCH.len();

pub(super) const BATCH: &[AgenticMorphismSpec] = &[
    AgenticMorphismSpec {
        id: "hypermedia.manifold.configure_interactive_cutaway",
        aliases: &[],
        category: "manifold",
        morphism: "configure_interactive_cutaway",
        owner: "QualiaDB/SDFB + Hypermedia",
        summary: "Configure a section plane on sealed SDFB geometry and return pick groups with distinct tissue and lumen identities.",
        domain: "Sealed10dOrHmcAssembly",
        codomain: "CutawaySectionDescriptor",
        morphism_kind: "Functor",
        teleology: "Configure a section plane on sealed SDFB geometry and return pick groups with distinct tissue and lumen identities.",
        units_and_frames: "Plane coefficients use the authored metre frame. Slab thickness is in metres.",
        arguments: "source, plane_abcd, slab_thickness_m?, assembly_key?, leaf_slots?. The source is an encoded or retained 10D or HMC, a sealed HMC handle, or a 10D field lattice.",
        composable_with: &[],
        human_preview_affordance: "Move the plane and inspect the resolved tissue or lumen pick.",
        abi_descriptor: morphism_func!(
            "configure_interactive_cutaway",
            "https://qualiadb.org/ext/hypermedia/manifold#configure_interactive_cutaway",
            2, 182, "(Sealed10dOrHmcAssembly) -> CutawaySectionDescriptor", START + 0,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.simulate_vehicle_aero_hydro_6dof",
        aliases: &[],
        category: "kinetics",
        morphism: "simulate_vehicle_aero_hydro_6dof",
        owner: "QualiaDB semantic motion and Etalon physics",
        summary: "Sample a Q42-authorized six-DOF marine vehicle and emit leaf-local PGA keys and telemetry.",
        domain: "SealedHmcAssembly, Q42Action, M64Hull, Controls",
        codomain: "AnimationClip, TabularCube",
        morphism_kind: "Functor",
        teleology: "Compute authored vessel response under independent water and air forces.",
        units_and_frames: "Use an M64 local metre frame, seconds, and signed unitless throttle and rudder inputs.",
        arguments: "bundle, assembly_key, material_key, action_iri, context_iri, step_s, controls. Each control row is [throttle, rudder].",
        composable_with: &["hypermedia.manifold.seal_leaf_assembly_10d", "qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the vessel pose keys and the force and power telemetry.",
        abi_descriptor: morphism_func!(
            "simulate_vehicle_aero_hydro_6dof",
            "https://qualiadb.org/ext/hypermedia/kinetics#simulate_vehicle_aero_hydro_6dof",
            7, 228, "(HmcAssembly, Controls) -> (AnimationClip, TabularCube)", START + 1,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.solve_multibody_dca_ik",
        aliases: &[],
        category: "kinetics",
        morphism: "solve_multibody_dca_ik",
        owner: "Q42 skeletal motion and Etalon DCA",
        summary: "Solve admitted serial SKL2 hinge chains towards authored effector targets and return local motors and joint torque reports.",
        domain: "HmcRig, M64Tissue, Q42Action, EffectorTargets",
        codomain: "AnimationClip, JointTorqueReport",
        morphism_kind: "Functor",
        teleology: "Solve admitted serial SKL2 hinge chains towards authored effector targets and return local motors and joint torque reports.",
        units_and_frames: "Primary and secondary targets use metres in the chain-base frame. The Q42 secondaryTip fiducial is expressed in the terminal-link frame. Steps use seconds and torques newton-metres.",
        arguments: "bundle, asset_key, material_key, action_iri, context_iri, joint_iris, targets_m, step_s, optional secondary_targets_m.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the semantic motor outputs and scientific telemetry.",
        abi_descriptor: morphism_func!(
            "solve_multibody_dca_ik",
            "https://qualiadb.org/ext/hypermedia/kinetics#solve_multibody_dca_ik",
            7, 122, "(HmcRig, M64Tissue, Q42Action, EffectorTargets) -> (AnimationClip, JointTorqueReport)", START + 2,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.solve_homeostatic_graph_flux",
        aliases: &[],
        category: "kinetics",
        morphism: "solve_homeostatic_graph_flux",
        owner: "Q42 hydraulic motion and M64 matter",
        summary: "Advance a bounded pressure-and-flow graph and emit wall shear and semantically addressed motors.",
        domain: "HmcHydraulicModel, Q42Action, M64Tissue, SourceFlows",
        codomain: "PressureFlowAndMotorResult",
        morphism_kind: "Functor",
        teleology: "Advance a bounded pressure-and-flow graph and emit wall shear and semantically addressed motors.",
        units_and_frames: "Pressure is in pascals and liquid source flow in cubic metres per second.",
        arguments: "bundle, asset_key, material_key, action_iri, context_iri, node_iris, edge_iris, source_m3_s, step_s, steps?.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the semantic motor outputs and scientific telemetry.",
        abi_descriptor: morphism_func!(
            "solve_homeostatic_graph_flux",
            "https://qualiadb.org/ext/hypermedia/kinetics#solve_homeostatic_graph_flux",
            7, 166, "(HmcHydraulicModel, Q42Action, M64Tissue, SourceFlows) -> (PressureFlowAndMotorResult)", START + 3,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.bake_aeroelastic_wind",
        aliases: &[],
        category: "kinetics",
        morphism: "bake_aeroelastic_wind",
        owner: "QualiaDB semantic motion and Etalon vegetation",
        summary: "Sample an admitted aeroelastic branch into leaf-local PGA motion keys.",
        domain: "HmcBranchAssembly, Q42Action, M64Tissue",
        codomain: "AnimationClip",
        morphism_kind: "Functor",
        teleology: "Bake a branch response from the authored atmospheric profile and inertia.",
        units_and_frames: "The wind and branch dimensions use SI units. Motion keys use authored leaf-local metres.",
        arguments: "bundle, asset_key, material_key, action_iri, context_iri, step_s, steps?.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the branch pose over the sampled aeroelastic response.",
        abi_descriptor: morphism_func!(
            "bake_aeroelastic_wind",
            "https://qualiadb.org/ext/hypermedia/kinetics#bake_aeroelastic_wind",
            7, 161, "(HmcBranchAssembly, Atmosphere) -> AnimationClip", START + 4,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.kinetics.fit_and_verify_articulation_sweep",
        aliases: &[],
        category: "kinetics",
        morphism: "fit_and_verify_articulation_sweep",
        owner: "Hypermedia articulation and QualiaDB geometry",
        summary: "Fit a single-axis hinge from paired authored facets and verify a bounded convex assembly sweep.",
        domain: "SealedHmcConvexAssembly, PairedLandmarks, PoseSamples, ProtectedVoids",
        codomain: "FitReceiptQ42, EngineeringReport",
        morphism_kind: "Functor",
        teleology: "Measure authored articulation, preserve protected voids and supply a mechanical motion envelope.",
        units_and_frames: "Use metres in geometry, radians in motor samples, and millimetres for receipt clearance.",
        arguments: "source, assembly_key, receipt_iri, context_iri, fixed_leaf_iri, moving_leaf_iri, hinge_pairs, max_landmark_residual_mm, minimum_clearance_mm, poses, allowed_contact_zones?, protected_voids?.",
        composable_with: &["hypermedia.manifold.seal_leaf_assembly_10d", "qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect engineering status, allowed zones, protected clearance and sampled motion.",
        abi_descriptor: morphism_func!(
            "fit_and_verify_articulation_sweep",
            "https://qualiadb.org/ext/hypermedia/kinetics#fit_and_verify_articulation_sweep",
            7, 181, "(ConvexAssembly, LandmarkPairs, PoseSamples) -> EngineeringAndSemanticReceipts", START + 5,
            DEFAULT_PARAMS, EffectSet::GRAPH_READ,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
    AgenticMorphismSpec {
        id: "hypermedia.manifold.bake_gaussian_surfels_10d",
        aliases: &[],
        category: "manifold",
        morphism: "bake_gaussian_surfels_10d",
        owner: "QualiaDB SURF codec and Hypermedia authoring",
        summary: "Fit surfels from authored mesh triangle clusters and seal them into a verified HOLO 10D leaf.",
        domain: "Sealed10dMeshLeaf, TriangleClusters",
        codomain: "Encoded10dSurfels, RadianceLattice",
        morphism_kind: "Functor",
        teleology: "Provide a compact far-field surface representation from authored geometry.",
        units_and_frames: "Surfel positions and radii are in the authored local metre frame.",
        arguments: "source_10d, clusters, target_surfel_count. Each cluster supplies triangle_indices, spectral_coeffs, roughness, and optional flags.",
        composable_with: &["qualia.container.pack_and_diff_hmc"],
        human_preview_affordance: "Inspect the eight-view orthographic geometry diagnostic.",
        abi_descriptor: morphism_func!(
            "bake_gaussian_surfels_10d",
            "https://qualiadb.org/ext/hypermedia/manifold#bake_gaussian_surfels_10d",
            2, 198, "(Sealed10dLeaf, TriangleClusters) -> (Encoded10dSurfels, RadianceLattice)", START + 6,
            DEFAULT_PARAMS, EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicTolerance, ReversibilityKind::PureNoState
        ),
    },
];

//! Category-theoretic & ontological taxonomy, `vibescript-abi` provider descriptors,
//! 5-field parity-sealed `NQuin48` ontology triples, and `AgenticUtility` metadata.
//!
//! Every asset production operation is modeled as a pure, bounded morphism
//! `f: Domain -> Codomain` within a mathematical/physical category rather than a
//! legacy desktop-application "craft" namespace.

use serde_json::{json, Value};
use vibescript_abi::{
    EffectSet, FuncDescriptor, ParamQudtBoundsSpec, ParamSpec, PrimitiveKind, ResourceEnvelope,
};

/// Top-level mathematical/physical categories of the Hypermedia & QualiaDB asset ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CategoryDescriptor {
    pub id: &'static str,
    pub sub_id: u16,
    pub iri: &'static str,
    pub object_space: &'static str,
    pubmorphism_purpose: &'static str,
    pub replaces_legacy_terms: &'static [&'static str],
}

pub const CATEGORIES: &[CategoryDescriptor] = &[
    CategoryDescriptor {
        id: "locus",
        sub_id: 1,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Locus",
        object_space: "Locus1D, Locus2D, PathComplex, ConstraintSystem",
        pubmorphism_purpose: "1D/2D parametric and planar geometric loci, Bézier splines, path booleans, offsets, sketch constraint solving, and SVG/DXF projection",
        replaces_legacy_terms: &["vectorcraft", "profile", "drafting"],
    },
    CategoryDescriptor {
        id: "manifold",
        sub_id: 2,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Manifold",
        object_space: "SurfaceManifold3D, SolidManifold3D, Manifold10D",
        pubmorphism_purpose: "3D/10D spatial & solid manifolds, extrusion, revolution, lofting, CSG mesh booleans, UV atlas parameterization, and .10d container sealing",
        replaces_legacy_terms: &["cadcraft", "mesh"],
    },
    CategoryDescriptor {
        id: "surface",
        sub_id: 3,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Surface",
        object_space: "TangentFrameField, MaterialSheaf, ProceduralField",
        pubmorphism_purpose: "Intrinsic surface tangent frames, procedural material node graphs, PBR albedo/normal/ORM map synthesis, and 3D mesh map baking",
        replaces_legacy_terms: &["materialcraft", "texturecraft"],
    },
    CategoryDescriptor {
        id: "radiance",
        sub_id: 4,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Radiance",
        object_space: "RadianceLattice2D, LayerStack, MaskLattice, ColorSpace",
        pubmorphism_purpose: "2D spectral, linear-light, and straight-alpha RGBA lattices, layer composition, morphological masks, 3D LUTs, RAW development, and PNG/PSD codecs",
        replaces_legacy_terms: &["photocraft", "lightcraft", "image", "rgba"],
    },
    CategoryDescriptor {
        id: "glyph",
        sub_id: 5,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Glyph",
        object_space: "TypefaceManifold, ShapedGlyphRun, TextMetrics",
        pubmorphism_purpose: "OpenType font table parsing, HarfBuzz/HarfRust glyph shaping, outline extraction, kerning/metrics, and glyph run rendering",
        replaces_legacy_terms: &["font-kit", "typography"],
    },
    CategoryDescriptor {
        id: "layout",
        sub_id: 6,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Layout",
        object_space: "BoxTree, ConstraintLayout, ScopedThemeTokens",
        pubmorphism_purpose: "Flex and grid spatial box-tree layout solving, responsive viewport constraints, and Webizen scoped theme/state sheet synthesis",
        replaces_legacy_terms: &["figcraft", "ui-layout"],
    },
    CategoryDescriptor {
        id: "kinetics",
        sub_id: 7,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Kinetics",
        object_space: "TimebaseRational, TrajectoryCurve,KinematicChain, SceneGraph",
        pubmorphism_purpose: "Rational frame-time lattices, keyframe/spring trajectories, skeletal rigs, two-bone IK, XPBD islands, DirectSpace queries, funnel+ORCA navigation, NLA channelbags, and Lottie/pose compilation",
        replaces_legacy_terms: &["effectcraft", "timeline", "animation"],
    },
    CategoryDescriptor {
        id: "acoustics",
        sub_id: 8,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Acoustics",
        object_space: "PressureWave1D, SpectralFrame, MixBusGraph",
        pubmorphism_purpose: "Acoustic wave synthesis, biquad/compressor/reverb DSP, multi-track bus mixing, and PCM WAV/AAC/Opus codecs",
        replaces_legacy_terms: &["soundcraft", "audio"],
    },
    CategoryDescriptor {
        id: "electromagnetics",
        sub_id: 9,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Electromagnetics",
        object_space: "EmfVolume5D, StokesPolarization, SpectralDistribution",
        pubmorphism_purpose: "4D/5D wave-optical and electromagnetic field lattices, Poynting flux, Stokes polarization, Q42EMF64 (.emf) sealing, and spectral slice previews",
        replaces_legacy_terms: &["emf-sim", "optical-field"],
    },
    CategoryDescriptor {
        id: "geodesy",
        sub_id: 10,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Geodesy",
        object_space: "HeightfieldTile, GeodesicPatch, MatterBrickM64",
        pubmorphism_purpose: "Planetary/terrain elevation lattices, reversible sculpt/stamp/smooth brush morphisms, .m64 matter brick compilation, and GIS bake (WGS84 / OSM vector transcode; DEM/STAC ingest later)",
        replaces_legacy_terms: &["terrain-craft", "world-sculpt"],
    },
    CategoryDescriptor {
        id: "discourse",
        sub_id: 11,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Discourse",
        object_space: "FoliatedDocument, ProseTree, SlideDeck, RelationalGrid, ReviewGallery",
        pubmorphism_purpose: "Structured rhetorical, tabular, and archival projections: PDF foliation, DOCX prose, PPTX slide decks, XLSX/SVG charts, and self-contained HTML review galleries",
        replaces_legacy_terms: &["wordcraft", "deckcraft", "gridcraft", "poet-pdf", "preview"],
    },
    CategoryDescriptor {
        id: "chronicle",
        sub_id: 12,
        iri: "https://qualiadb.org/ontology/hypermedia/category#Chronicle",
        object_space: "TemporalPacketStream, IsobmffContainer",
        pubmorphism_purpose: "Synchronized audio-visual packet streams, H.264/VP9/AV1 video encoding, and ISOBMFF (.mp4) container muxing",
        replaces_legacy_terms: &["filmcraft", "video-mux"],
    },
    CategoryDescriptor {
        id: "container",
        sub_id: 13,
        iri: "https://qualiadb.org/ontology/hmc#Package",
        object_space: "HmcBundle, SemanticModel, PackageDelta",
        pubmorphism_purpose: "Pack resource ranges, semantic models and version deltas.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "math",
        sub_id: 14,
        iri: "https://qualiadb.org/ontology/math#SymbolicAlgebra",
        object_space: "CasExpression, Equation, NQuinAst",
        pubmorphism_purpose: "Solve bounded symbolic problems and cite expression trees in Q42.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "statistics",
        sub_id: 15,
        iri: "https://qualiadb.org/ontology/production#StatisticalInference",
        object_space: "NumericTable, ProbabilitySimplex, EconomicModel",
        pubmorphism_purpose: "Evaluate statistical and economic models over tabular or graph data.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "knowledge",
        sub_id: 16,
        iri: "https://qualiadb.org/ontology/production#Modality",
        object_space: "NamedQ42Graph, ModalityVerdict",
        pubmorphism_purpose: "Evaluate evidence, argumentation and agency constraints against a selected graph.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "geospatial",
        sub_id: 17,
        iri: "https://qualiadb.org/ontology/production#GeospatialModel",
        object_space: "M64GeoRaster, Corridor, Isogloss, Toponym",
        pubmorphism_purpose: "Author terrain corridors and geographic semantic assets.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "inference",
        sub_id: 18,
        iri: "https://qualiadb.org/ontology/production#InferenceAuthoring",
        object_space: "Q42Journal, ConditioningProfile",
        pubmorphism_purpose: "Stage and verify graph transactions and conditioning plans.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "epistemics",
        sub_id: 19,
        iri: "https://qualiadb.org/ontology/production#EpistemicAudit",
        object_space: "ResearchEvidence, NamedQ42Graph, AuditReport",
        pubmorphism_purpose: "Ground research and forensic analysis in admitted evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "live",
        sub_id: 20,
        iri: "https://qualiadb.org/ontology/production#LiveAuthoring",
        object_space: "CueStack, CompanionApp, PortalWorld, XrBinding",
        pubmorphism_purpose: "Author retained live-stage and world manifests.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "network",
        sub_id: 21,
        iri: "https://qualiadb.org/ontology/production#NetworkArtifact",
        object_space: "SolidTurtle, QdnfFrame, CorpusBundle",
        pubmorphism_purpose: "Prepare offline exchange artifacts and corpus receipts.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "physics",
        sub_id: 22,
        iri: "https://qualiadb.org/ontology/production#ContinuumPhysics",
        object_space: "MultiphaseMixture, CartoonKinematicState",
        pubmorphism_purpose: "Evaluate multiphase material properties and stylized deformation.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "topology",
        sub_id: 23,
        iri: "https://qualiadb.org/ontology/production#HarbouredTopology",
        object_space: "GeometryComplex, AnchoredCsrGraph",
        pubmorphism_purpose: "Induce and query multiple anchored graphs within a geometry complex.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "governance",
        sub_id: 24,
        iri: "https://qualiadb.org/ontology/production#SemanticInstrument",
        object_space: "Instrument, Policy, ConsentSnapshot",
        pubmorphism_purpose: "Evaluate admitted policies, instruments and consent evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "cooperative",
        sub_id: 25,
        iri: "https://qualiadb.org/ontology/production#ContributionLifecycle",
        object_space: "Contribution, LedgerEntry, Delegation",
        pubmorphism_purpose: "Evaluate staged contributions and accountable delegation.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "geometry",
        sub_id: 26,
        iri: "https://qualiadb.org/ontology/production#ComputationalGeometry",
        object_space: "PlanarProfile, SurfaceMesh, TetMesh, SolidArrangement",
        pubmorphism_purpose: "Refine meshes, construct arrangements and certify finite element boundaries.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "anatomy",
        sub_id: 27,
        iri: "https://qualiadb.org/ontology/production#AnatomicalAsset",
        object_space: "CardiacContainer, MatterVolume, EvidenceManifest",
        pubmorphism_purpose: "Seal retained anatomical assets and index observed work graph evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "twin",
        sub_id: 28,
        iri: "https://qualiadb.org/ontology/production#EntityAffordance",
        object_space: "EntityOntology, Ethogram, ObserverProjection",
        pubmorphism_purpose: "Evaluate authored behavior and project an observer-relative entity view.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "science",
        sub_id: 29,
        iri: "https://qualiadb.org/ontology/production#ScientificValidation",
        object_space: "OracleParameters, Measurement, ValidationReceipt",
        pubmorphism_purpose: "Evaluate physical and numerical invariants.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "q42",
        sub_id: 30,
        iri: "https://qualiadb.org/ontology/production#SpatialSemanticResources",
        object_space: "SpatialPredicate, SituatedPlaceTime, ModelSubstrate",
        pubmorphism_purpose: "Encode situated facts and verify addressed resources.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "solvers",
        sub_id: 31,
        iri: "https://qualiadb.org/ontology/production#NumericalSolvers",
        object_space: "Graph, Matrix, NumericCube, TensorCommitment",
        pubmorphism_purpose: "Evaluate bounded numerical and integrity operators.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "economics",
        sub_id: 32,
        iri: "https://qualiadb.org/ontology/production#ComputationalEconomics",
        object_space: "PreferenceMatrix, MarketScenario, EngineeringLoadHistory",
        pubmorphism_purpose: "Evaluate bounded market and engineering scenarios.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "vision",
        sub_id: 33,
        iri: "https://qualiadb.org/ontology/production#VisionObservations",
        object_space: "Image, VoxelVolume, ObservedSpectrum, OpticalDensity",
        pubmorphism_purpose: "Evaluate bounded image and optical observations.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "wellfair",
        sub_id: 34,
        iri: "https://qualiadb.org/ontology/production#WellFairSafeguards",
        object_space: "CohortScope, ConsentGrant, BranchingTimeModel",
        pubmorphism_purpose: "Evaluate authored access and temporal policy scenarios.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "vibe",
        sub_id: 35,
        iri: "https://qualiadb.org/ontology/production#VibeTooling",
        object_space: "SourceProgram, Bytecode, Texture, OrbitalFunction",
        pubmorphism_purpose: "Inspect retained source programs, rendering resources and orbital functions.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "graph",
        sub_id: 36,
        iri: "https://qualiadb.org/ontology/production#TemporalAndQuotedGraphs",
        object_space: "RetainedRdf, Q42Volume, TemporalCommits, RdfStarStatement",
        pubmorphism_purpose: "Verify graph equivalence, transform graph history and annotate statements.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "sensing",
        sub_id: 37,
        iri: "https://qualiadb.org/ontology/production#SensingGeometry",
        object_space: "RadioLink, ObstructionSample, ClearanceObservation",
        pubmorphism_purpose: "Evaluate physical sensing geometry and derive explicit observations.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "vector",
        sub_id: 38,
        iri: "https://qualiadb.org/ontology/production#VectorProfiles",
        object_space: "BezierContour, PlanarProfile, WindingMoments",
        pubmorphism_purpose: "Evaluate and transform authored vector contours.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "image",
        sub_id: 39,
        iri: "https://qualiadb.org/ontology/production#RadianceSegmentation",
        object_space: "RadianceLattice, SuperpixelLabel, RefinedMask",
        pubmorphism_purpose: "Derive explicit image regions and masks.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "rights",
        sub_id: 40,
        iri: "https://qualiadb.org/ontology/production#RightsAndUsageEvidence",
        object_space: "UsageAccession, WorkAttribution, CollectivePool",
        pubmorphism_purpose: "Evaluate admitted usage rights and associated accounting evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "cosmos",
        sub_id: 41,
        iri: "https://qualiadb.org/ontology/production#OrbitalEphemerides",
        object_space: "OrbitSource, ReferenceFrame, Trajectory",
        pubmorphism_purpose: "Evaluate celestial trajectories in a declared coordinate frame.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "compute",
        sub_id: 42,
        iri: "https://qualiadb.org/ontology/production#CalibratedComputeCartridges",
        object_space: "TrainingDataset, ComputeCartridge, CalibrationReceipt",
        pubmorphism_purpose: "Seal and evaluate calibrated native compute cartridges.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "linguistics",
        sub_id: 43,
        iri: "https://qualiadb.org/ontology/production#SpatiotemporalSemiotics",
        object_space: "HistoricalObservation, ArchiveIdentity, SemanticDriftGraph",
        pubmorphism_purpose: "Resolve historical observations in their authored spatial and temporal contexts.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "interchange",
        sub_id: 44,
        iri: "https://qualiadb.org/ontology/production#ScientificStudioInterchange",
        object_space: "ScientificAsset, PhysicalUnitDeclaration, InterchangeReceipt",
        pubmorphism_purpose: "Preserve canonical physical declarations across scientific and studio assets.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "category",
        sub_id: 45,
        iri: "https://qualiadb.org/ontology/production#CompositionalFormalStructures",
        object_space: "AttributedGraph, StructurePreservingMap, UniversalConstructionWitness",
        pubmorphism_purpose: "Derive finite composed structures and retain the corresponding map witnesses.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "accessibility",
        sub_id: 46,
        iri: "https://qualiadb.org/ontology/production#AccessibleMediaAndTactileAssets",
        object_space: "AccessibleText, AuthoredBraille, TactileManifold",
        pubmorphism_purpose: "Preserve accessible authoring semantics in admitted tangible and digital outputs.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "quantum",
        sub_id: 47,
        iri: "https://qualiadb.org/ontology/production#UnitaryCircuitSimulation",
        object_space: "BoundedCircuit, ComplexStateVector, BasisMeasurement",
        pubmorphism_purpose: "Simulate admitted offline circuits and retain basis measurement evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "narrative",
        sub_id: 48,
        iri: "https://qualiadb.org/ontology/production#LinearTokenWorldGrammar",
        object_space: "NarrativeAtom, LinearRule, WorldStateWitness",
        pubmorphism_purpose: "Apply authored token rules and validate finite narrative reachability traces.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "logic",
        sub_id: 49,
        iri: "https://qualiadb.org/ontology/production#BoundedFormalLogic",
        object_space: "CnfProblem, ProofTerm, FormalEvidence",
        pubmorphism_purpose: "Check bounded formal problems and expose the admitted evidence and resource limits.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "fabrication",
        sub_id: 50,
        iri: "https://qualiadb.org/ontology/production#CivilianFabrication",
        object_space: "ManufacturingMesh, SheetProfile, ModelPackage",
        pubmorphism_purpose: "Evaluate and prepare admitted civilian manufacturing geometry.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "privacy",
        sub_id: 51,
        iri: "https://qualiadb.org/ontology/production#ScopedRasterPrivacy",
        object_space: "RasterRedactionRegion, RedactedImage, PrivacyEvidence",
        pubmorphism_purpose: "Apply declared privacy operations to admitted raster data and report their coverage.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "infrastructure",
        sub_id: 52,
        iri: "https://qualiadb.org/ontology/production#CivilInfrastructure",
        object_space: "Microgrid, VehicleTrajectory, HydraulicNetwork",
        pubmorphism_purpose: "Evaluate admitted offline civil infrastructure models.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "control",
        sub_id: 53,
        iri: "https://qualiadb.org/ontology/production#OfflineControlScan",
        object_space: "BooleanInstruction, RegisterTrace, ScanCycle",
        pubmorphism_purpose: "Evaluate bounded authored control scans and expose their traces.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "environment",
        sub_id: 54,
        iri: "https://qualiadb.org/ontology/production#EnvironmentalSimulation",
        object_space: "WaterColumn, Ecotope, CoverField",
        pubmorphism_purpose: "Evaluate bounded civilian environmental inputs and report model scope.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "botany",
        sub_id: 55,
        iri: "https://qualiadb.org/ext/qualia/botany#",
        object_space: "Plant topology and environmental inputs",
        pubmorphism_purpose: "Model plant phenology and geometry.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "anvil",
        sub_id: 56,
        iri: "https://qualiadb.org/ext/qualia/anvil#",
        object_space: "Declared compute capability and routing models",
        pubmorphism_purpose: "Plan offline compute placement within capability constraints.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "storage",
        sub_id: 57,
        iri: "https://qualiadb.org/ext/qualia/storage#",
        object_space: "Storage zone models and placement plans",
        pubmorphism_purpose: "Plan bounded logical storage placement.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "provenance",
        sub_id: 58,
        iri: "https://qualiadb.org/ext/qualia/provenance#",
        object_space: "Creation lineage anchors and evidence",
        pubmorphism_purpose: "Seal and verify detached creation provenance.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "medical",
        sub_id: 59,
        iri: "https://qualiadb.org/ext/qualia/medical#",
        object_space: "Synthetic medical imaging fixtures",
        pubmorphism_purpose: "Reconstruct explicitly synthetic projection data.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "matter",
        sub_id: 60,
        iri: "https://qualiadb.org/ext/qualia/matter#",
        object_space: "Material stratum and thermal state",
        pubmorphism_purpose: "Evaluate bounded material response.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "ai",
        sub_id: 61,
        iri: "https://qualiadb.org/ext/qualia/ai#",
        object_space: "Local model interpretation diagnostics",
        pubmorphism_purpose: "Inspect aggregate local model behavior.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "crypto",
        sub_id: 62,
        iri: "https://qualiadb.org/ext/qualia/crypto#",
        object_space: "Offline cryptographic evidence and verification",
        pubmorphism_purpose: "Verify bounded retained cryptographic evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "chemistry",
        sub_id: 63,
        iri: "https://qualiadb.org/ext/qualia/chemistry#",
        object_space: "Bounded numerical kinetics and incidence systems",
        pubmorphism_purpose: "Evaluate declared bounded kinetic and flux models.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "bio",
        sub_id: 64,
        iri: "https://qualiadb.org/ext/qualia/bio#",
        object_space: "Bounded teaching models for compartment transport",
        pubmorphism_purpose: "Evaluate normalized transport fixtures.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "biosphere",
        sub_id: 65,
        iri: "https://qualiadb.org/ext/qualia/biosphere#",
        object_space: "Bounded ecological exchange models and provenance",
        pubmorphism_purpose: "Evaluate authored compartment exchange.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "neuro",
        sub_id: 66,
        iri: "https://qualiadb.org/ext/qualia/neuro#",
        object_space: "Published offline numerical membrane benchmarks",
        pubmorphism_purpose: "Evaluate fixed illustrative membrane trajectories.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "wellfare",
        sub_id: 67,
        iri: "https://qualiadb.org/ext/qualia/wellfare#",
        object_space: "Fixed educational wellbeing model fixtures",
        pubmorphism_purpose: "Inspect native system-weight behavior on fictional fixtures.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "poet",
        sub_id: 68,
        iri: "https://qualiadb.org/ext/qualia/poet#",
        object_space: "Signed event grants and offline action bindings",
        pubmorphism_purpose: "Verify detached event grants against admitted action and Aura evidence.",
        replaces_legacy_terms: &[],
    },
    CategoryDescriptor {
        id: "biology",
        sub_id: 69,
        iri: "https://qualiadb.org/ext/qualia/biology#",
        object_space: "Authored static inert geometry and probe-shell measurements",
        pubmorphism_purpose: "Measure exposed area in an explicitly inert geometric chain.",
        replaces_legacy_terms: &[],
    },
];

/// Agentic utility and ontological grounding for a single asset production morphism.
#[derive(Debug, Clone, Copy)]
pub struct AgenticMorphismSpec {
    /// Canonical categorical identifier (`hypermedia.<category>.<morphism>`).
    pub id: &'static str,
    /// Historical `hypermedia.*` or `synth.*` compatibility aliases that resolve to `id`.
    pub aliases: &'static [&'static str],
    /// Category identifier (`locus`, `manifold`, `surface`, `radiance`, etc.).
    pub category: &'static str,
    /// Morphism short name within the category.
    pub morphism: &'static str,
    /// Underlying owner crate/module in `hypermedia-production`, `qualia-core-db`, or `etalon`.
    pub owner: &'static str,
    /// Concise human & agent summary.
    pub summary: &'static str,
    /// Formal domain object signature.
    pub domain: &'static str,
    /// Formal codomain object signature.
    pub codomain: &'static str,
    /// Category-theoretic morphism classification (`Constructor`, `Endomorphism`, `Functor`, `NaturalTransformation`, `CoalgebricProjection`, `AlgebraicFold`).
    pub morphism_kind: &'static str,
    /// Teleological grounding: why an agent invokes this morphism and what real-world/UI problem it solves.
    pub teleology: &'static str,
    /// Physical units, coordinate conventions, and color/numerical invariants.
    pub units_and_frames: &'static str,
    /// Argument schema specification.
    pub arguments: &'static str,
    /// Upstream/downstream canonical morphism IDs that naturally compose with this morphism.
    pub composable_with: &'static [&'static str],
    /// How human reviewers and agents visually/auditorily inspect the codomain.
    pub human_preview_affordance: &'static str,
    /// VibeScript ABI function descriptor with 5-field parity `NQuin48` spans.
    pub abi_descriptor: FuncDescriptor,
}

pub(super) const DEFAULT_PARAMS: &[ParamSpec] = &[ParamSpec::slice("args_record", PrimitiveKind::U8, 0, u32::MAX)];
pub(super) const DEFAULT_ENVELOPE: ResourceEnvelope = ResourceEnvelope {
    max_ticks: 50_000_000,
    max_scratch_bytes: 64 * 1024 * 1024,
    max_vram_bytes: 0,
    max_output_bytes: 16 * 1024 * 1024,
    max_spawned_tasks: 0,
    flags: 0,
};

macro_rules! morphism_func {
    ($name:expr, $iri:expr, $sub_id:expr, $func_id:expr, $sig:expr, $span_idx:expr $(,)?) => {
        morphism_func!(
            $name,
            $iri,
            $sub_id,
            $func_id,
            $sig,
            $span_idx,
            DEFAULT_PARAMS,
            EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicExact,
            ReversibilityKind::PureNoState,
        )
    };
    (
        $name:expr, $iri:expr, $sub_id:expr, $func_id:expr, $sig:expr, $span_idx:expr,
        $params:expr, $effects:expr, $determinism:expr, $reversibility:expr $(,)?
    ) => {
        FuncDescriptor::const_new(
            $name,
            $iri,
            StableFuncId::new($sub_id, $func_id),
            $sig,
            $params,
            PrimitiveKind::U64,
            1,
            $effects,
            $determinism,
            $reversibility,
            DEFAULT_ENVELOPE,
            (($span_idx as u32) * 3, 1),
            (($span_idx as u32) * 3 + 1, 1),
            (($span_idx as u32) * 3 + 2, 1),
        )
    };
}


mod morphisms_01;
mod morphisms_02;
mod morphisms_03;
mod morphisms_04;
mod morphisms_05;
mod morphisms_06;
mod morphisms_07;
mod morphisms_08;
mod morphisms_09;
mod morphisms_10;
mod morphisms_11;
mod morphisms_12;
mod morphisms_21;
mod morphisms_22;
mod morphisms_23;
mod morphisms_24;
mod morphisms_25;
mod morphisms_26;
mod morphisms_29;
mod morphisms_32;
mod morphisms_33;
mod morphisms_34;
mod morphisms_swarm;
mod nquins;
mod vibe_descriptors;

#[cfg(test)]
mod tests;

pub use nquins::ONTOLOGY_NQUINS;
pub use vibe_descriptors::{HypermediaOntologyProvider, HYPERMEDIA_PROVIDER_DESCRIPTOR};

pub(crate) const EXPANSION: &[AgenticMorphismSpec] = &concat_morphisms::<{
    morphisms_10::BATCH.len()
        + morphisms_21::BATCH.len()
        + morphisms_22::BATCH.len()
        + morphisms_23::BATCH.len()
        + morphisms_11::BATCH.len()
        + morphisms_24::BATCH.len()
        + morphisms_12::BATCH.len()
        + morphisms_26::BATCH.len()
        + morphisms_32::BATCH.len()
        + morphisms_33::BATCH.len()
        + morphisms_25::BATCH.len()
        + morphisms_29::BATCH.len()
        + morphisms_swarm::BATCH.len()
        + morphisms_34::BATCH.len()
}>(&[
    morphisms_10::BATCH,
    morphisms_21::BATCH,
    morphisms_22::BATCH,
    morphisms_23::BATCH,
    morphisms_11::BATCH,
    morphisms_24::BATCH,
    morphisms_12::BATCH,
    morphisms_26::BATCH,
    morphisms_32::BATCH,
    morphisms_33::BATCH,
    morphisms_25::BATCH,
    morphisms_29::BATCH,
    morphisms_swarm::BATCH,
    morphisms_34::BATCH,
]);

const fn concat_morphisms<const N: usize>(batches: &[&[AgenticMorphismSpec]]) -> [AgenticMorphismSpec; N] {
    let mut out = [morphisms_01::BATCH[0]; N];
    let mut offset = 0;
    let mut batch = 0;
    while batch < batches.len() {
        let mut index = 0;
        while index < batches[batch].len() {
            out[offset] = batches[batch][index];
            offset += 1;
            index += 1;
        }
        batch += 1;
    }
    assert!(offset == N);
    out
}

const BASE_MORPHISM_COUNT: usize = 150;
const ALL_MORPHISMS: [AgenticMorphismSpec; BASE_MORPHISM_COUNT + EXPANSION.len()] = {
    let mut out = [morphisms_01::BATCH[0]; BASE_MORPHISM_COUNT + EXPANSION.len()];
    let batches: [&[AgenticMorphismSpec]; 10] = [
        morphisms_01::BATCH,
        morphisms_02::BATCH,
        morphisms_03::BATCH,
        morphisms_04::BATCH,
        morphisms_05::BATCH,
        morphisms_06::BATCH,
        morphisms_07::BATCH,
        morphisms_08::BATCH,
        morphisms_09::BATCH,
        EXPANSION,
    ];
    let mut idx = 0;
    let mut b = 0;
    while b < batches.len() {
        let slice = batches[b];
        let mut i = 0;
        while i < slice.len() {
            out[idx] = slice[i];
            idx += 1;
            i += 1;
        }
        b += 1;
    }
    assert!(idx == out.len());
    out
};

/// Complete ontological & agentic utility specification for all admitted morphisms.
pub static MORPHISMS: &[AgenticMorphismSpec] = &ALL_MORPHISMS;

/// Resolves any canonical categorical ID or historical `hypermedia.*`/`synth.*` alias
/// to its `AgenticMorphismSpec`.
pub fn resolve_morphism(id: &str) -> Option<&'static AgenticMorphismSpec> {
    MORPHISMS
        .iter()
        .find(|m| m.id == id || m.aliases.contains(&id))
}

/// Resolves any operation identifier to its canonical categorical ID (`hypermedia.<category>.<morphism>`).
pub fn canonical_morphism_id(id: &str) -> Option<&'static str> {
    resolve_morphism(id).map(|m| m.id)
}

/// Physical units and bounds for named morphism arguments, including fields of
/// a legacy ABI argument record.
pub fn morphism_param_qudt_bounds(id: &str) -> &'static [ParamQudtBoundsSpec] {
    const SDF_BRICK_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64(
            "resolution",
            "http://qudt.org/vocab/unit/UNITLESS",
            8.0,
            128.0,
        ),
        ParamQudtBoundsSpec::bounded_f64("padding", "http://qudt.org/vocab/unit/M", 0.01, 10.0),
    ];
    const SDFB_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("resolution", "http://qudt.org/vocab/unit/UNITLESS", 16.0, 128.0),
        ParamQudtBoundsSpec::bounded_f64("padding", "http://qudt.org/vocab/unit/M", 0.000_001_f32 as f64, 10.0),
        ParamQudtBoundsSpec::bounded_f64("narrow_band_voxels", "http://qudt.org/vocab/unit/UNITLESS", 1.0, 16.0),
    ];
    const CUTAWAY_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("slab_thickness_m", "http://qudt.org/vocab/unit/M", 0.0, f64::MAX),
    ];
    const MARINE_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("step_s", "http://qudt.org/vocab/unit/SEC", f64::from_bits(1), 0.25),
    ];
    const DCA_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("step_s", "http://qudt.org/vocab/unit/SEC", f64::from_bits(1), 0.05),
    ];
    const GARMENT_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("step_s", "http://qudt.org/vocab/unit/SEC", 1e-4, 1.0 / 30.0),
        ParamQudtBoundsSpec::bounded_f64("steps", "http://qudt.org/vocab/unit/UNITLESS", 1.0, 128.0),
        ParamQudtBoundsSpec::bounded_f64("iterations", "http://qudt.org/vocab/unit/UNITLESS", 1.0, 32.0),
        ParamQudtBoundsSpec::bounded_f64("max_initial_gap_m", "http://qudt.org/vocab/unit/M", f64::from_bits(1), f64::MAX),
    ];
    const WAVE_SPECTRUM_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("depth_m", "http://qudt.org/vocab/unit/M", 0.05, 12_000.0),
        ParamQudtBoundsSpec::bounded_f64("period_m", "http://qudt.org/vocab/unit/M", 1.0, 100_000.0),
        ParamQudtBoundsSpec::bounded_f64("time_s", "http://qudt.org/vocab/unit/SEC", -1e8, 1e8),
        ParamQudtBoundsSpec::bounded_f64("choppiness", "http://qudt.org/vocab/unit/UNITLESS", 0.0, 2.0),
        ParamQudtBoundsSpec::bounded_f64("whitecap_jacobian", "http://qudt.org/vocab/unit/UNITLESS", 0.05, 0.999),
    ];
    const SURROGATE_BOUNDS: &[ParamQudtBoundsSpec] = &[
        ParamQudtBoundsSpec::bounded_f64("degree", "http://qudt.org/vocab/unit/UNITLESS", 1.0, 6.0),
    ];

    match canonical_morphism_id(id) {
        Some("hypermedia.manifold.mesh_to_sdf_brick") => SDF_BRICK_BOUNDS,
        Some("hypermedia.manifold.seal_sdf_brick_10d") => SDFB_BOUNDS,
        Some("hypermedia.manifold.configure_interactive_cutaway") => CUTAWAY_BOUNDS,
        Some("hypermedia.kinetics.simulate_vehicle_aero_hydro_6dof") => MARINE_BOUNDS,
        Some("hypermedia.kinetics.solve_multibody_dca_ik") => DCA_BOUNDS,
        Some("hypermedia.kinetics.sew_and_drape_garment") => GARMENT_BOUNDS,
        Some("hypermedia.geodesy.synthesize_ocean_spectrum") => WAVE_SPECTRUM_BOUNDS,
        Some("qualia.compute.compile_p64_surrogate_cartridge") => SURROGATE_BOUNDS,
        _ => &[],
    }
}

fn effect_names(effects: EffectSet) -> String {
    if effects.is_pure() {
        return "PURE_COMPUTE".into();
    }

    let known_effects = [
        (EffectSet::GRAPH_READ, "GRAPH_READ"),
        (EffectSet::GRAPH_MUTATE, "GRAPH_MUTATE"),
        (EffectSet::HARDWARE_IO, "HARDWARE_IO"),
        (EffectSet::EXTERNAL_NET, "EXTERNAL_NET"),
        (EffectSet::CRYPTO_VAULT, "CRYPTO_VAULT"),
        (EffectSet::TASK_SPAWN, "TASK_SPAWN"),
    ];
    let mut names = Vec::new();
    let mut remaining = effects.0;
    for (flag, name) in known_effects {
        if effects.contains_all(flag) {
            names.push(name.into());
            remaining &= !flag.0;
        }
    }
    if remaining != 0 {
        names.push(format!("{remaining:#010x}"));
    }
    names.join(" | ")
}

fn hex_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

/// Serializes the category-theoretic ontology taxonomy for agentic discovery (`synth.ontology.categories`).
pub fn categories_json() -> Value {
    json!({
        "schema": "hypermedia.categorical-ontology/v1",
        "provider_iri": HYPERMEDIA_PROVIDER_DESCRIPTOR.canonical_iri.iri,
        "provider_alias": HYPERMEDIA_PROVIDER_DESCRIPTOR.short_alias,
        "naming_paradigm": "category-theoretic (Objects + Morphisms f: Domain -> Codomain; zero -craft/legacy product names)",
        "total_ontology_nquins": ONTOLOGY_NQUINS.len(),
        "categories": CATEGORIES.iter().map(|c| {
            let count = MORPHISMS.iter().filter(|m| m.category == c.id).count();
            json!({
                "id": c.id,
                "sub_namespace_id": c.sub_id,
                "iri": c.iri,
                "object_space": c.object_space,
                "morphism_purpose": c.pubmorphism_purpose,
                "replaces_legacy_terms": c.replaces_legacy_terms,
                "admitted_morphism_count": count,
            })
        }).collect::<Vec<_>>(),
    })
}

/// Serializes a single `AgenticMorphismSpec` including its `OntologicalContract` and `NQuin48` parity hashes.
pub fn morphism_to_json(m: &AgenticMorphismSpec) -> Value {
    let f = &m.abi_descriptor;
    let c = &f.contract;
    let pre_q = &ONTOLOGY_NQUINS[c.precondition_quin_span.0 as usize];
    let post_q = &ONTOLOGY_NQUINS[c.postcondition_quin_span.0 as usize];
    let inv_q = &ONTOLOGY_NQUINS[c.invariant_quin_span.0 as usize];
    let params = f.params.iter().map(|param| {
        json!({
            "name": param.name,
            "kind": format!("{:?}", param.kind),
            "min_elems": param.min_elems,
            "max_elems": param.max_elems,
        })
    }).collect::<Vec<_>>();
    let param_qudt_bounds = morphism_param_qudt_bounds(m.id)
        .iter()
        .map(|param| {
            let bounded = param.has_numeric_bounds == 1;
            let unit = vibescript_abi::qudt_units::normalize_qudt_unit(param.qudt_unit_iri);
            json!({
                "name": param.name,
                "qudt_unit_iri": unit.map_or(param.qudt_unit_iri, |unit| unit.iri),
                "qudt_dimension_iri": unit.map(|unit| unit.dimension_iri),
                "qudt_conversion_multiplier": unit.map(|unit| unit.conversion_multiplier()),
                "has_numeric_bounds": bounded,
                "min_inclusive": bounded.then(|| param.min_f64()),
                "max_inclusive": bounded.then(|| param.max_f64()),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "id": m.id,
        "aliases": m.aliases,
        "category": m.category,
        "morphism": m.morphism,
        "owner": m.owner,
        "summary": m.summary,
        "arguments": m.arguments,
        "agentic_utility": {
            "domain": m.domain,
            "codomain": m.codomain,
            "morphism_kind": m.morphism_kind,
            "teleology": m.teleology,
            "units_and_frames": m.units_and_frames,
            "composable_with": m.composable_with,
            "human_preview_affordance": m.human_preview_affordance,
        },
        "ontological_contract": {
            "canonical_iri": f.canonical_iri.iri,
            "iri_fnv1a_hash": format!("{:016x}", f.canonical_iri.fnv1a_hash),
            "stable_id": {
                "sub_namespace_id": f.stable_id.sub_namespace_id,
                "func_id": f.stable_id.func_id,
            },
            "contract_digest_sha256": hex_bytes(&c.contract_digest),
            "schema_hash": format!("{:016x}", c.schema_hash),
            "effects": effect_names(f.effects),
            "effect_mask": f.effects.0,
            "determinism": format!("{:?}", f.determinism),
            "reversibility": format!("{:?}", f.reversibility),
            "params": params,
            "param_qudt_bounds": param_qudt_bounds,
            "envelope": {
                "max_ticks": f.envelope.max_ticks,
                "max_scratch_bytes": f.envelope.max_scratch_bytes,
                "max_output_bytes": f.envelope.max_output_bytes,
            },
            "nquin48_triples": {
                "precondition": {
                    "span": [c.precondition_quin_span.0, c.precondition_quin_span.1],
                    "parity_verified": pre_q.verify_parity(),
                    "parity_word": format!("{:016x}", pre_q.parity()),
                },
                "postcondition": {
                    "span": [c.postcondition_quin_span.0, c.postcondition_quin_span.1],
                    "parity_verified": post_q.verify_parity(),
                    "parity_word": format!("{:016x}", post_q.parity()),
                },
                "invariant": {
                    "span": [c.invariant_quin_span.0, c.invariant_quin_span.1],
                    "parity_verified": inv_q.verify_parity(),
                    "parity_word": format!("{:016x}", inv_q.parity()),
                },
            }
        }
    })
}

#[cfg(test)]
mod descriptor_metadata_tests {
    use super::*;
    use vibescript_abi::{DeterminismGuarantee, ReversibilityKind, StableFuncId};

    #[test]
    fn catalog_json_reports_custom_abi_metadata() {
        const PARAMS: &[ParamSpec] = &[ParamSpec::scalar("padding", PrimitiveKind::F64)];
        const EFFECTS: EffectSet = EffectSet::GRAPH_READ.union(EffectSet::HARDWARE_IO);
        let mut sample = *resolve_morphism("hypermedia.manifold.mesh_to_sdf_brick").unwrap();
        let original = sample.abi_descriptor;
        let span_index = original.contract.precondition_quin_span.0 / 3;
        sample.abi_descriptor = morphism_func!(
            original.name,
            original.canonical_iri.iri,
            original.stable_id.sub_namespace_id,
            original.stable_id.func_id,
            sample.domain,
            span_index,
            PARAMS,
            EFFECTS,
            DeterminismGuarantee::DeterministicTolerance,
            ReversibilityKind::StagedReversible,
        );

        let json = morphism_to_json(&sample);
        let contract = &json["ontological_contract"];
        assert_eq!(contract["effects"], "GRAPH_READ | HARDWARE_IO");
        assert_eq!(contract["effect_mask"], EFFECTS.0);
        assert_eq!(contract["determinism"], "DeterministicTolerance");
        assert_eq!(contract["reversibility"], "StagedReversible");
        assert_eq!(
            contract["params"],
            json!([{"name": "padding", "kind": "F64", "min_elems": 1, "max_elems": 1}])
        );
        assert_eq!(
            contract["param_qudt_bounds"][1],
            json!({
                "name": "padding",
                "qudt_unit_iri": "http://qudt.org/vocab/unit/M",
                "qudt_dimension_iri": "http://qudt.org/vocab/dimensionvector/A0E0L1I0M0H0T0D0",
                "qudt_conversion_multiplier": 1.0,
                "has_numeric_bounds": true,
                "min_inclusive": 0.01,
                "max_inclusive": 10.0,
            })
        );
        let alias = sample.aliases.first().unwrap();
        assert_eq!(
            morphism_param_qudt_bounds(alias),
            morphism_param_qudt_bounds(sample.id),
        );
        assert!(morphism_param_qudt_bounds("unknown").is_empty());
    }
}

/// Serializes morphisms filtered by optional `category` (`synth.ontology.morphisms`).
pub fn morphisms_json(category: Option<&str>) -> Value {
    let filtered: Vec<_> = MORPHISMS
        .iter()
        .filter(|m| category.is_none_or(|cat| m.category == cat))
        .map(morphism_to_json)
        .collect();
    json!({
        "schema": "hypermedia.categorical-morphisms/v1",
        "filter_category": category,
        "count": filtered.len(),
        "morphisms": filtered,
    })
}

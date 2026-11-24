//! # `biosphere-authoring-forge`
//!
//! Unified cross-kingdom (**Flora, Fauna, Funga, Slime Molds, Lichens**) and multi-physics
//! (**Water, Sky/Solar, 3D Wind Pressure FSI, Hydrometeors/Cryosphere, Sub-Surface Pedology/Thixotropy,
//! Australian Pyro-Botany, Bio-Acoustics, Capillary Surface Tension, Atmospheric Electricity/Aeolian,
//! Ecological Persistence, and Quantum/Bio-Optical Horizons**) authoring and simulation domain engine
//! for **Webizen AI Synth**, **Poet**, and **Ecopunk**.
//!
//! ## 21 Zero-Heap Domain Modules (`#![forbid(unsafe_code)]`)
//!
//! ### Core & First-Wave Biosphere Modules (11 Modules)
//! 1. [`arthropod`] — Zero-heap 6–16 leg arthropod gaits, exoskeleton joint limits, and 64-byte (`16 × f32`)
//!    physical-metre `Q42EMF64` cuticle thin-film interference & Bouligand circular polarization synthesis.
//! 2. [`ik_contact`] — Analytical two-bone Law-of-Cosines terrain foot-contact IK and coupled batch multi-leg solver.
//! 3. [`skeletal_admission`] — `SKL1` & `SKL2 (0x0304)` 6-DoF Projective Geometric Algebra (`Cl(3,0,1)`) bone motors,
//!    barycentric UV tendon entheses, polyaxial retinacular pulleys, and non-identity vertex lineage weight remapping.
//! 4. [`botanical_junction`] — Arbitrary-$N$ organ 3D space colonization, Da Vinci pipe-model radii, multi-branch
//!    SDF collar welding with split-UV seam pairs, and Discrete Differential Geometry (`DDG`) smooth vertex normals.
//! 5. [`containers_10d_m64`] — Holographic 10D v2 (`HOLO 0x0306`) checked section editing and millimetre-exact
//!    `M64 v3` (`GRM3`) terrain brush elevation strokes with the `0.0005 m` half-millimetre narrowing guard.
//! 6. [`funga_mycelium`] — 3D hyphal foraging & lateral anastomosis networks ($b_1 > 0$ Betti loops), negative-gravitropic
//!    fruiting-body morphogenesis, Turing-bifurcating hymenophore gills (`lamellae`), Buller's drop spore plumes,
//!    525 nm bioluminescent `Q42EMF64` cells, and enzymatic wood-decay modulus softening.
//! 7. [`hydrometeor_cryosphere`] — Marshall–Palmer rain DSD & impact impulses, fog LWC/Koschmieder visibility &
//!    occult fog-drip harvesting, Nakaya snow crystal habit & slope accumulation, radiative hoar frost vs. windward
//!    supercooled rime icing, conformal glaze ice + stalactitic icicles, black-ice friction collapse, and Stefan lake ice.
//! 8. [`wind_pressure_fsi`] — 3D orographic ridge speed-up & lee recirculation eddy field, dynamic pressure tensor
//!    $q = \frac{1}{2}\rho U^2$ & surface $C_p$, 3D canopy wake shelter cones, 4-level hierarchical tree Vogel crown
//!    streamlining & abscission, Kelvin–Helmholtz `honami` meadow waves, and fauna wind-lean / arthropod wind-crouch reflex.
//! 9. [`hydro_surface_coupling`] — 2D Saint-Venant / Manning shallow-water velocity, Froude supercritical rapids,
//!    obstacle bow wakes, Karman vortex streets, hydrodynamic drag/buoyancy, and Washburn porous capillary wetting
//!    with Lekner–Dorf internal-reflection optical darkening hysteresis.
//! 10. [`sky_microclimate`] — 6-band Rayleigh/Mie/Ozone spectral sky dome with Stokes `[Q,U,V]` arthropod sky-compass
//!     polarization for `Q42EMF64`, Beer–Lambert canopy light extinction, Penman–Monteith evaporative cooling, and
//!     diurnal anabatic/katabatic cold-air drainage & valley frost-hollow pooling.
//! 11. [`fauna_flora_extensions`] — Tetrapod, bipedal saltatorial (macropod), undulatory, and avian CPG gaits,
//!     isochoric volume-preserving muscle belly bulging along `SKL2` entheses, pelage/plumage wet-clumping/piloerection/frost,
//!     and subterranean root anchorage, fluted buttress flare, and geodesic bryophyte/lichen/epiphyte colonization.
//!
//! ### Second-Wave Multi-Physics & Cross-Kingdom Gap-Audit Horizons (10 Modules, 40 Exact `#[repr(C)]` PODs)
//! 12. [`pedology_thixotropy`] (**Vector 1**) — Coussot thixotropic mud rheology, cyclic liquefaction, cavitation-bounded
//!     fauna IK suction mire pullout forces, Bishop–van Genuchten desiccation mud-cracking ($90^\circ$ T- to $120^\circ$ Y-junctions),
//!     Konrad–Morgenstern frost heave, and wombat/woylie/magnetic-termite bioturbation.
//! 13. [`pyro_botany`] (**Vector 2**) — Australian bushfire bark thermal insulation & charring recession, Van Wagner
//!     crown scorch height vs. active crown combustion, *Banksia*/*Hakea* serotinous follicle dehiscence, deep
//!     epicormic "fuzzy tree" & lignotuber resprouting, Modified Byram pyrocumulus (`pyroCu`) blow-up, and DeBano hydrophobic ash.
//! 14. [`bio_acoustics`] (**Vector 3**) — *Casuarina*/pine Aeolian needle Strouhal whistling vs. broadleaf rustle,
//!     raindrop underwater bubble Minnaert resonance, cicada 4-rib tymbal buckling, Orthoptera stridulation,
//!     anuran vocal-sac impedance matching, and nighttime inversion acoustic ducting vs. daytime lapse shadow zones.
//! 15. [`botanical_morph_hydraulics`] (**Vector 4**) — Vogel golden-angle spiral phyllotaxis, fern circinate vernation
//!     uncoiling, Pammenter–Vander Willigen ($P_{50}$) xylem cavitation & VPD leaf-wilting tensors, osmotic pulvinus
//!     motors (*Mimosa* thigmonasty / nyctinasty), 6-band PROSPECT-MP leaf senescence & 700 nm red-edge shift, and bark rhytidome peeling.
//! 16. [`capillary_intertidal`] (**Vector 5**) — Gerridae water-strider meniscus support & dipolar vortex thrust,
//!     Lotus Cassie–Baxter superhydrophobicity & splash crowns/Worthington jets, viscid spider silk Plateau–Rayleigh
//!     beading & electrostatic capture, intertidal *Avicennia* pneumatophores & Langmuir windrows, and shear-triggered
//!     dinoflagellate bioluminescent wakes.
//! 17. [`atmo_electricity_aeolian`] (**Vector 6**) — Atmospheric electric fields, Peek's law corona inception,
//!     spider electrostatic dragline ballooning, bumblebee floral electro-reception, Shao–Lu aeolian sand saltation
//!     & migrating impact ripples, Rankine dust devils, Edlén–Stull mirages, and 6-band rainbows & $22^\circ$ ice halos.
//! 18. [`fauna_physio_sensory`] (**Vector 7**) — Allometric mammalian/avian/insect DGC respiration cycles &
//!     Clausius–Clapeyron cold-air breath condensation plumes, vascular counter-current heat exchange & Band-5 LWIR
//!     radiators (macropod saliva-licked forearms, bilby ears), Cosserat muscular hydrostats, gecko Kendall dry-vdW
//!     peel & tree-frog capillary toe pads, and `L6` Cry4 quantum radical-pair magnetoreception, platypus electro-sense & pit-viper IR.
//! 19. [`funga_slime_lichen`] (**Vector 8**) — Tero–Alim *Physarum polycephalum* peristaltic tube network optimization,
//!     *Cyathus* splash-cup peridiole ballistics & funicular tetherball stem wrapping, Gray–Scott fairy rings,
//!     Seminara edge-limited lichen growth & oxalic acid rock etching, stinkhorn VOC fly recruitment, and *Coprinus* deliquescence.
//! 20. [`ecological_persistence`] (**Vector 9**) — Cole elasto-plastic vegetation trampling & `.m64` game-trail compaction,
//!     Couvreur epizoochory burr dispersal & pelage mud/snow loading, Australian bloodwood kino resin exudation,
//!     and larval IAA/cytokinin gall & green-island morphogenesis.
//! 21. [`biosphere_horizons`] (**Vector 10**) — Timoshenko–Reyssat hygromorph bilayer actuation (cones, *Banksia* separators,
//!     self-burying awns), Guenther MEGAN *Eucalyptus* BVOC blue-haze SOA, underwater CDOM tannin blackwater absorption
//!     & chromatic Snell's Window, Brewster-angle aquatic insect polarotaxis & Krisciunas–Schaefer nocturnal sky/airglow,
//!     wet beach-sand Reynolds dilatancy footprint halos, wildfire/frost rock spalling, and plant bio-electrical AP/VP + albatross dynamic soaring.

pub mod arthropod;
pub mod atmo_electricity_aeolian;
pub mod bio_acoustics;
pub mod biosphere_horizons;
pub mod botanical_junction;
pub mod botanical_morph_hydraulics;
pub mod capillary_intertidal;
pub mod containers_10d_m64;
pub mod ecological_persistence;
pub mod fauna_flora_extensions;
pub mod fauna_physio_sensory;
pub mod funga_mycelium;
pub mod funga_slime_lichen;
pub mod hydro_surface_coupling;
pub mod hydrometeor_cryosphere;
pub mod ik_contact;
pub mod pedology_thixotropy;
pub mod pyro_botany;
pub mod skeletal_admission;
pub mod sky_microclimate;
pub mod wind_pressure_fsi;

pub use arthropod::{
    evaluate_arthropod_gait_zero_heap, synthesize_cuticle_emf_field_zero_heap, ArthropodBodyPlan,
    ArthropodGaitError, CuticleEmfSpectralCell, CuticleOpticalError, CuticleOpticalProfile,
    ExoskeletonJointKind, ExoskeletonJointLimit, ExoskeletonSegment,
};
pub use atmo_electricity_aeolian::{
    eval_atmo_electro_field_cell, eval_atmo_optics_wavefront, sample_dust_devil_wind_and_dp,
    step_aeolian_saltation_field, AeolianSaltationCell, AtmoElectroFieldCell, AtmoOpticsWavefront,
    DustDevilVortex,
};
pub use bio_acoustics::{
    eval_acoustic_refraction_loss_db, step_bio_acoustic_emitters, AcousticRayProfile,
    BioAcousticEmitter,
};
pub use biosphere_horizons::{
    eval_brewster_polarization_and_nocturnal_sky, eval_underwater_cdom_and_snell_window,
    step_eucalypt_bvoc_blue_haze, step_hygromorph_bilayers, step_wet_sand_dilatancy,
    BrewsterNocturnalSky, BvocBlueHazeCell, HygromorphBilayerOrgan, PlantElectrophysNode,
    ThermalFrostRockCell, UnderwaterBioOpticsCell, WetSandDilatancyCell,
};
pub use botanical_junction::{
    compute_pipe_model_radii_zero_heap, evaluate_botanical_growth, weld_multi_junction,
    BotanicalGrowthResult, BotanicalLimits, BotanicalOrgan, BotanicalRecipe, BotanicalSolvedNode,
    ChildBranchAttachment, MultiJunctionBudget, MultiJunctionError, MultiJunctionSpec,
    WeldedMultiJunctionMesh,
};
pub use botanical_morph_hydraulics::{
    eval_prospect_leaf_emf_bands, evaluate_vogel_phyllotaxis_and_vernation_zero_heap,
    step_xylem_cavitation_and_wilt, BarkRhytidomePatch, LeafSpectralPigments,
    PhyllotaxisPrimordium, PulvinusMotorJoint, XylemHydraulicNode, GOLDEN_ANGLE_RAD,
};
pub use capillary_intertidal::{
    step_dinoflagellate_bioluminescence, step_water_strider_meniscus, CapillaryMeniscusLeg,
    DinoflagellateWakeCell, IntertidalZonationCell, SplashCrownEvent, ViscidSilkCaptureStrand,
};
pub use containers_10d_m64::{
    inspect_holographic_10d_v2, project_metres_f32_checked,
    replace_10d_section_preserving_holo_v2, Holographic10dEditLimits, Holographic10dInspection,
    SectionSpan10d, TerrainFalloff, TerrainOperation, TerrainStrokeDraft,
};
pub use ecological_persistence::{
    step_pelage_debris_and_epizoochory, step_plant_wound_kino_and_galls,
    step_vegetation_trampling, PelageDebrisAttachment, PlantWoundKinoGall,
    TramplingPersistenceCell,
};
pub use fauna_flora_extensions::{
    evaluate_fauna_cpg_locomotion_zero_heap, evaluate_isochoric_muscle_belly_deformation,
    evaluate_pelage_plumage_hydro_thermal_response, evaluate_root_buttress_and_epiphyte_ecology,
    evaluate_undulatory_spine_curvature, CoatSurfaceResponse, FaunaCpgKinematicFrame,
    FaunaGaitArchetype, IsochoricMuscleBellyState, RootButtressEpiphyteReport,
};
pub use fauna_physio_sensory::{
    eval_bio_adhesive_pad_capacity, eval_quantum_and_emf_receptors, step_fauna_respiration,
    step_vascular_thermoregulation_lwir, BioAdhesivePadContact, FaunaRespirationState,
    HydrostatSegment, QuantumSensoryReceptor, VascularThermoZone,
};
pub use funga_mycelium::{
    evaluate_fungal_wood_decay_modulus, step_mycelium_foraging_and_anastomosis_zero_heap,
    FungalEcologicalGuild, FungalFruitingBodySpec, HymenophoreMorphology, HyphalEdge, HyphalNode,
    MyceliumNetworkReceipt, MycorrhizalAttractor, MAX_HYPHAL_EDGES, MAX_HYPHAL_NODES,
    MAX_MYCORRHIZAL_ATTRACTORS,
};
pub use funga_slime_lichen::{
    step_fairy_ring_and_lichen_grid, step_physarum_network, step_splash_cup_peridioles,
    FungalVocLichenCell, PhysarumVeinEdge, SplashCupPeridiole,
};
pub use hydro_surface_coupling::{
    evaluate_shallow_water_obstacle_interaction_zero_heap,
    evaluate_washburn_porous_wetting_and_optics, PorousMaterialClass, PorousWettingOpticalResult,
    ShallowWaterObstacleReport,
};
pub use hydrometeor_cryosphere::{
    evaluate_hydrometeor_cryosphere_effects_zero_heap, CryosphereAssetEffects, HydrometeorState,
    NakayaCrystalHabit,
};
pub use ik_contact::{
    solve_arthropod_terrain_ik_batch_zero_heap, solve_terrain_foot_contact_zero_heap,
    FootContactTarget, IkContactError, IkContactResult, LimbChainSpec,
};
pub use pedology_thixotropy::{
    step_desiccation_fracture_network, step_thixotropic_mire_contact, BioturbationAgent,
    DesiccationCrackNode, ThixotropicSoilCell,
};
pub use pyro_botany::{
    evaluate_pyrocumulus_column_zero_heap, step_pyro_botany_stems, step_serotinous_follicles,
    PyroBotanyStemState, PyroPlumeColumn, SerotinousFolliclePod,
};
pub use skeletal_admission::{
    normalize_skin_weights_checked, remap_skin_weights_by_lineage_zero_heap,
    validate_skin_weights, AdmittedSkl2Rig, BarycentricUvEnthesis, JointLimitSpec,
    LimitEnforcementMode, MorphCategory, MorphTargetDescriptor, PgaMotor6Dof,
    RetinacularPulleySpec, SkeletalAdmissionError, SkeletalHierarchyValidator, SkinWeight4,
    MAX_SKL_JOINTS,
};
pub use sky_microclimate::{
    evaluate_spectral_sky_dome_and_polarization_zero_heap,
    evaluate_topographic_microclimate_zero_heap, SpectralSkySample, TopographicMicroclimateReport,
};
pub use wind_pressure_fsi::{
    evaluate_3d_wind_pressure_field_zero_heap, evaluate_fauna_wind_posture_compensation,
    evaluate_hierarchical_flora_wind_response, evaluate_honami_meadow_wave_at,
    evaluate_surface_pressure_coefficient_cp, HierarchicalFloraWindResponse,
    LocalWindPressureField, WindWakeObstacle,
};

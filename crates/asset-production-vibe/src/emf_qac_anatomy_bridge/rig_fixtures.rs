//! Canonical evidenced `SpeciesCorpus` and anatomical/botanical rig fixtures
//! for *Wallabia bicolor* (Swamp Wallaby) and *Corymbia aparrerinja* (Ghost Gum).

use crate::emf_qac_anatomy_bridge::rig_metamer::{
    ArticulatedVertebrateSegment, BioelectricDipoleSource, BioelectricSourceKind,
    BotanicalMetamerKind, BotanicalMetamerRig, BotanicalMetamerSegment,
    VertebrateAcousticCueSpec, VertebrateAnatomicalRig, VertebrateSegmentKind,
};
use qualia_core_db::q42::species_corpus::SpeciesCorpus;

fn corpus_assertion_json(
    id: &str,
    taxon_iri: &str,
    scope_domain: &str,
    property_iri: &str,
    part_iri: &str,
    min: f64,
    max: f64,
    unit_iri: &str,
) -> serde_json::Value {
    serde_json::json!({
        "assertion": {
            "id": id,
            "subject_uri": taxon_iri,
            "statement_type": "observation",
            "assertion_status": "active",
            "verification_status": "verified",
            "claim_summary": id,
            "author_did": "did:web:qualia.org.au:biophysics-curator",
            "root_source_id": "src-primary",
            "scope_domain": scope_domain,
            "sensitivity": "Public",
            "asserted_at_unix": 1_700_000_000
        },
        "evidence": {
            "property_iri": property_iri,
            "basis": "measured",
            "method_iri": "urn:qualia:method:biophysical-metrology-v1",
            "scope": {
                "taxon_concept_iri": taxon_iri,
                "part_iri": part_iri
            },
            "value": {
                "kind": "range",
                "min": min,
                "max": max,
                "unit_iri": unit_iri
            }
        }
    })
}

/// Builds a canonical evidenced `SpeciesCorpus` and articulated `VertebrateAnatomicalRig`
/// for *Wallabia bicolor* (Swamp Wallaby).
pub fn canonical_wallaby_rig_and_corpus(
) -> Result<(SpeciesCorpus, VertebrateAnatomicalRig), String> {
    let taxon = "https://biodiversity.org.au/afd/taxa/Wallabia_bicolor";
    let corpus_iri = "urn:corpus:wallabia-bicolor:emf-qac-r1";
    let doc = serde_json::json!({
        "@context": "https://webizen.org/ns/qualia/species-corpus/v1",
        "@type": "QualiaSpeciesCorpus",
        "format": "qualia.q42.species-corpus.v1",
        "corpus_iri": corpus_iri,
        "taxon_concept_iri": taxon,
        "species_id": "wallabia-bicolor",
        "revision": "2026-11-14-r1",
        "sources": [{
            "id": "src-primary",
            "title": "Wallabia bicolor articulated osteology, bioelectric and vocalization dataset",
            "access_policy": "public-not-for-redistribution",
            "source_sha256": "a1b2c3d4e5f60718293a4b5c6d7e8f90a1b2c3d4e5f60718293a4b5c6d7e8f90",
            "mapping_version": "1.0"
        }],
        "assertions": [
            corpus_assertion_json("ea-wb-spine", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#lumbarSpineSpan", "UBERON:0001130", 0.24, 0.32, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-thorax", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#thoracicCageSpan", "UBERON:0000915", 0.18, 0.24, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-cranium", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#cranialSpan", "UBERON:0003128", 0.11, 0.15, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-forelimb", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#forelimbSpan", "UBERON:0002102", 0.16, 0.22, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-hindlimb", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#saltatorialCrusSpan", "UBERON:0002103", 0.34, 0.44, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-tail", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/anatomy#caudalCounterbalanceSpan", "UBERON:0002415", 0.62, 0.76, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-wb-cardiac-dipole", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/physiology#ventricularDipoleMoment", "UBERON:0000948", 0.0008, 0.0022, "http://qudt.org/vocab/unit/A-M"),
            corpus_assertion_json("ea-wb-neural-dipole", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/physiology#motorCortexDipoleMoment", "UBERON:0000955", 1.0e-7, 8.0e-7, "http://qudt.org/vocab/unit/A-M"),
            corpus_assertion_json("ea-wb-myo-dipole", taxon, "species:wallabia-bicolor", "https://ns.webizen.org/eco/physiology#gastrocnemiusMotorDipole", "UBERON:0001388", 2.0e-5, 9.0e-5, "http://qudt.org/vocab/unit/A-M")
        ],
        "gaps": []
    });
    let corpus = SpeciesCorpus::from_json_value(doc)?;
    let wavelengths_m = vec![
        420e-9, 480e-9, 540e-9, 600e-9, 660e-9, 720e-9, 800e-9, 880e-9,
    ];
    let dorsal_pelage = vec![0.045, 0.052, 0.064, 0.088, 0.112, 0.145, 0.195, 0.230];
    let rufous_flank = vec![0.055, 0.068, 0.095, 0.165, 0.220, 0.265, 0.310, 0.340];

    let segments = vec![
        ArticulatedVertebrateSegment {
            segment_id: "axial_lumbar_pelvis".into(),
            kind: VertebrateSegmentKind::AxialSpine,
            uberon_iri: "UBERON:0001130".into(),
            parent_index: None,
            proximal_joint_m: [0.0, 0.42, -0.18],
            distal_joint_m: [0.0, 0.48, 0.08],
            cross_section_radii_m: [0.095, 0.110],
            muscular_activation_01: 0.62,
            metabolic_temperature_k: 309.65,
            dermal_reflectance_bins: dorsal_pelage.clone(),
            structural_modal_freq_hz: 68.0,
            structural_damping_s1: 14.5,
            assertion_id: "ea-wb-spine".into(),
        },
        ArticulatedVertebrateSegment {
            segment_id: "thoracic_ribcage".into(),
            kind: VertebrateSegmentKind::Thorax,
            uberon_iri: "UBERON:0000915".into(),
            parent_index: Some(0),
            proximal_joint_m: [0.0, 0.48, 0.08],
            distal_joint_m: [0.0, 0.56, 0.28],
            cross_section_radii_m: [0.082, 0.096],
            muscular_activation_01: 0.48,
            metabolic_temperature_k: 310.05,
            dermal_reflectance_bins: rufous_flank.clone(),
            structural_modal_freq_hz: 84.0,
            structural_damping_s1: 18.0,
            assertion_id: "ea-wb-thorax".into(),
        },
        ArticulatedVertebrateSegment {
            segment_id: "cervical_cranium".into(),
            kind: VertebrateSegmentKind::Cranium,
            uberon_iri: "UBERON:0003128".into(),
            parent_index: Some(1),
            proximal_joint_m: [0.0, 0.56, 0.28],
            distal_joint_m: [0.0, 0.66, 0.38],
            cross_section_radii_m: [0.046, 0.052],
            muscular_activation_01: 0.35,
            metabolic_temperature_k: 309.45,
            dermal_reflectance_bins: dorsal_pelage.clone(),
            structural_modal_freq_hz: 142.0,
            structural_damping_s1: 24.0,
            assertion_id: "ea-wb-cranium".into(),
        },
        ArticulatedVertebrateSegment {
            segment_id: "forelimb_brachium_l".into(),
            kind: VertebrateSegmentKind::Forelimb,
            uberon_iri: "UBERON:0002102".into(),
            parent_index: Some(1),
            proximal_joint_m: [0.075, 0.50, 0.22],
            distal_joint_m: [0.085, 0.33, 0.26],
            cross_section_radii_m: [0.028, 0.032],
            muscular_activation_01: 0.40,
            metabolic_temperature_k: 308.15,
            dermal_reflectance_bins: rufous_flank,
            structural_modal_freq_hz: 118.0,
            structural_damping_s1: 21.0,
            assertion_id: "ea-wb-forelimb".into(),
        },
        ArticulatedVertebrateSegment {
            segment_id: "hindlimb_crus_pes_l".into(),
            kind: VertebrateSegmentKind::Hindlimb,
            uberon_iri: "UBERON:0002103".into(),
            parent_index: Some(0),
            proximal_joint_m: [0.090, 0.41, -0.14],
            distal_joint_m: [0.095, 0.04, -0.02],
            cross_section_radii_m: [0.054, 0.068],
            muscular_activation_01: 0.88,
            metabolic_temperature_k: 310.35,
            dermal_reflectance_bins: dorsal_pelage.clone(),
            structural_modal_freq_hz: 96.0,
            structural_damping_s1: 16.0,
            assertion_id: "ea-wb-hindlimb".into(),
        },
        ArticulatedVertebrateSegment {
            segment_id: "caudal_counterbalance_tail".into(),
            kind: VertebrateSegmentKind::Tail,
            uberon_iri: "UBERON:0002415".into(),
            parent_index: Some(0),
            proximal_joint_m: [0.0, 0.40, -0.18],
            distal_joint_m: [0.0, 0.14, -0.82],
            cross_section_radii_m: [0.042, 0.045],
            muscular_activation_01: 0.55,
            metabolic_temperature_k: 307.65,
            dermal_reflectance_bins: dorsal_pelage,
            structural_modal_freq_hz: 52.0,
            structural_damping_s1: 11.5,
            assertion_id: "ea-wb-tail".into(),
        },
    ];

    let dipoles = vec![
        BioelectricDipoleSource {
            source_id: "dipole_cardiac_qrs".into(),
            kind: BioelectricSourceKind::CardiacVentricularSyncytium,
            anatomy_ontology: "UBERON".into(),
            anatomy_identifier: "UBERON:0000948".into(),
            observation_type: "current_dipole".into(),
            segment_index: 1,
            local_offset_m: [-0.012, -0.020, 0.015],
            current_dipole_moment_a_m: [0.00045, -0.00125, 0.00060],
            extracellular_potential_v: 0.00145,
            carrier_hz: 2.1,
            uncertainty_a_m: 2.5e-5,
            assertion_id: "ea-wb-cardiac-dipole".into(),
        },
        BioelectricDipoleSource {
            source_id: "dipole_cranial_motor_cortex".into(),
            kind: BioelectricSourceKind::NeuralCorticalColumn,
            anatomy_ontology: "UBERON".into(),
            anatomy_identifier: "UBERON:0000955".into(),
            observation_type: "current_dipole".into(),
            segment_index: 2,
            local_offset_m: [0.0, 0.018, 0.010],
            current_dipole_moment_a_m: [1.2e-7, 3.8e-7, 1.5e-7],
            extracellular_potential_v: 4.5e-5,
            carrier_hz: 24.0,
            uncertainty_a_m: 1.0e-8,
            assertion_id: "ea-wb-neural-dipole".into(),
        },
        BioelectricDipoleSource {
            source_id: "dipole_hindlimb_gastrocnemius".into(),
            kind: BioelectricSourceKind::MyoelectricMotorUnit,
            anatomy_ontology: "UBERON".into(),
            anatomy_identifier: "UBERON:0001388".into(),
            observation_type: "current_dipole".into(),
            segment_index: 4,
            local_offset_m: [0.008, 0.045, -0.010],
            current_dipole_moment_a_m: [1.5e-5, 4.8e-5, -1.2e-5],
            extracellular_potential_v: 3.2e-4,
            carrier_hz: 85.0,
            uncertainty_a_m: 1.5e-6,
            assertion_id: "ea-wb-myo-dipole".into(),
        },
    ];

    let rig = VertebrateAnatomicalRig {
        rig_id: "wallabia-bicolor-articulated-r1".into(),
        species_id: "wallabia-bicolor".into(),
        taxon_concept_iri: taxon.into(),
        corpus_iri: corpus_iri.into(),
        epoch_seconds: 1_700_000_000,
        spectral_wavelengths_m: wavelengths_m,
        segments,
        dipoles,
        acoustic_cue: VertebrateAcousticCueSpec {
            vocalization_fundamental_hz: 165.0,
            vocalization_formants_hz: [330.0, 720.0, 1480.0],
            footfall_impulse_pa: 0.42,
            hoa_azimuth_rad: 0.35,
            hoa_elevation_rad: 0.12,
            sample_rate_hz: 48_000,
            frame_count: 256,
        },
    };
    Ok((corpus, rig))
}

/// Builds a canonical evidenced `SpeciesCorpus` and Murray-law `BotanicalMetamerRig`
/// for *Corymbia aparrerinja* (Ghost Gum).
pub fn canonical_ghost_gum_metamer_rig_and_corpus(
) -> Result<(SpeciesCorpus, BotanicalMetamerRig), String> {
    let taxon = "https://biodiversity.org.au/afd/taxa/Corymbia_aparrerinja";
    let corpus_iri = "urn:corpus:corymbia-aparrerinja:emf-qac-r1";
    let doc = serde_json::json!({
        "@context": "https://webizen.org/ns/qualia/species-corpus/v1",
        "@type": "QualiaSpeciesCorpus",
        "format": "qualia.q42.species-corpus.v1",
        "corpus_iri": corpus_iri,
        "taxon_concept_iri": taxon,
        "species_id": "corymbia-aparrerinja",
        "revision": "2026-11-14-r1",
        "sources": [{
            "id": "src-primary",
            "title": "Corymbia aparrerinja hydraulic Murray-law and foliar red-edge dataset",
            "access_policy": "public-not-for-redistribution",
            "source_sha256": "f0e1d2c3b4a5968778695a4b3c2d1e0ff0e1d2c3b4a5968778695a4b3c2d1e0f",
            "mapping_version": "1.0"
        }],
        "assertions": [
            corpus_assertion_json("ea-ca-internode", taxon, "species:corymbia-aparrerinja", "https://ns.webizen.org/eco/botany#primaryInternodeXylemRadius", "PO:0005005", 0.008, 0.016, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-ca-petiole", taxon, "species:corymbia-aparrerinja", "https://ns.webizen.org/eco/botany#petioleVascularRadius", "PO:0020038", 0.004, 0.012, "http://qudt.org/vocab/unit/M"),
            corpus_assertion_json("ea-ca-lamina", taxon, "species:corymbia-aparrerinja", "https://ns.webizen.org/eco/botany#foliarRedEdgeReflectance", "PO:0025060", 0.04, 0.55, "http://qudt.org/vocab/unit/UNITLESS")
        ],
        "gaps": []
    });
    let corpus = SpeciesCorpus::from_json_value(doc)?;

    let inv_cbrt2 = 2.0f64.powf(-1.0 / 3.0);
    let r0_xylem = 0.0120;
    let r1_xylem = r0_xylem * inv_cbrt2;
    let r2_xylem = r1_xylem * inv_cbrt2;

    let r0_phloem = 0.0048;
    let r1_phloem = r0_phloem * inv_cbrt2;
    let r2_phloem = r1_phloem * inv_cbrt2;

    let wavelengths_m = vec![
        440e-9, 550e-9, 640e-9, 680e-9, 705e-9, 730e-9, 780e-9, 850e-9,
    ];
    let bark_reflectance = vec![0.68, 0.72, 0.75, 0.76, 0.77, 0.78, 0.80, 0.81];
    let petiole_reflectance = vec![0.10, 0.18, 0.12, 0.09, 0.24, 0.38, 0.42, 0.44];
    let lamina_red_edge = vec![0.048, 0.135, 0.058, 0.042, 0.265, 0.485, 0.525, 0.540];

    let metamers = vec![
        BotanicalMetamerSegment {
            metamer_id: "internode_primary_0".into(),
            kind: BotanicalMetamerKind::Internode,
            plant_ontology_iri: "PO:0005005".into(),
            parent_index: None,
            proximal_node_m: [0.0, 1.20, 0.0],
            distal_node_m: [0.0, 1.55, 0.04],
            xylem_radius_m: r0_xylem,
            phloem_radius_m: r0_phloem,
            lamina_area_m2: 0.0,
            leaf_temperature_k: 298.15,
            red_edge_reflectance_bins: bark_reflectance,
            rustle_modal_freq_hz: 44.0,
            rustle_damping_s1: 8.5,
            rustle_amplitude_pa: 0.08,
            assertion_id: "ea-ca-internode".into(),
        },
        BotanicalMetamerSegment {
            metamer_id: "petiole_branch_1a".into(),
            kind: BotanicalMetamerKind::Petiole,
            plant_ontology_iri: "PO:0020038".into(),
            parent_index: Some(0),
            proximal_node_m: [0.0, 1.55, 0.04],
            distal_node_m: [0.14, 1.72, 0.12],
            xylem_radius_m: r1_xylem,
            phloem_radius_m: r1_phloem,
            lamina_area_m2: 0.0012,
            leaf_temperature_k: 299.05,
            red_edge_reflectance_bins: petiole_reflectance.clone(),
            rustle_modal_freq_hz: 112.0,
            rustle_damping_s1: 15.0,
            rustle_amplitude_pa: 0.14,
            assertion_id: "ea-ca-petiole".into(),
        },
        BotanicalMetamerSegment {
            metamer_id: "petiole_branch_1b".into(),
            kind: BotanicalMetamerKind::Petiole,
            plant_ontology_iri: "PO:0020038".into(),
            parent_index: Some(0),
            proximal_node_m: [0.0, 1.55, 0.04],
            distal_node_m: [-0.13, 1.70, -0.09],
            xylem_radius_m: r1_xylem,
            phloem_radius_m: r1_phloem,
            lamina_area_m2: 0.0012,
            leaf_temperature_k: 298.85,
            red_edge_reflectance_bins: petiole_reflectance,
            rustle_modal_freq_hz: 118.0,
            rustle_damping_s1: 15.5,
            rustle_amplitude_pa: 0.13,
            assertion_id: "ea-ca-petiole".into(),
        },
        BotanicalMetamerSegment {
            metamer_id: "lamina_falcate_2a".into(),
            kind: BotanicalMetamerKind::Lamina,
            plant_ontology_iri: "PO:0025060".into(),
            parent_index: Some(1),
            proximal_node_m: [0.14, 1.72, 0.12],
            distal_node_m: [0.22, 1.60, 0.18],
            xylem_radius_m: r2_xylem,
            phloem_radius_m: r2_phloem,
            lamina_area_m2: 0.0048,
            leaf_temperature_k: 300.25,
            red_edge_reflectance_bins: lamina_red_edge.clone(),
            rustle_modal_freq_hz: 285.0,
            rustle_damping_s1: 28.0,
            rustle_amplitude_pa: 0.22,
            assertion_id: "ea-ca-lamina".into(),
        },
        BotanicalMetamerSegment {
            metamer_id: "lamina_falcate_2b".into(),
            kind: BotanicalMetamerKind::Lamina,
            plant_ontology_iri: "PO:0025060".into(),
            parent_index: Some(1),
            proximal_node_m: [0.14, 1.72, 0.12],
            distal_node_m: [0.19, 1.58, 0.05],
            xylem_radius_m: r2_xylem,
            phloem_radius_m: r2_phloem,
            lamina_area_m2: 0.0046,
            leaf_temperature_k: 300.15,
            red_edge_reflectance_bins: lamina_red_edge,
            rustle_modal_freq_hz: 310.0,
            rustle_damping_s1: 30.0,
            rustle_amplitude_pa: 0.20,
            assertion_id: "ea-ca-lamina".into(),
        },
    ];

    let rig = BotanicalMetamerRig {
        plant_id: "corymbia-aparrerinja-fspm-r1".into(),
        species_id: "corymbia-aparrerinja".into(),
        taxon_concept_iri: taxon.into(),
        corpus_iri: corpus_iri.into(),
        epoch_seconds: 1_700_000_000,
        spectral_wavelengths_m: wavelengths_m,
        metamers,
        wind_velocity_m_s: [3.4, 0.2, -1.1],
        sample_rate_hz: 48_000,
        frame_count: 256,
    };
    Ok((corpus, rig))
}

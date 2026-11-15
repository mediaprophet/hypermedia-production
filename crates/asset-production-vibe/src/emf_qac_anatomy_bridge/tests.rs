//! Comprehensive unit tests for deterministic EMF v4 + QAC v1 anatomical/botanical production.

use crate::emf_qac_anatomy_bridge::{
    canonical_ghost_gum_metamer_rig_and_corpus, canonical_wallaby_rig_and_corpus,
    emf_inspection_to_vibe_value, inspect_and_verify_emf_v4_bundle,
    inspect_and_verify_qac_v1_bundle, produce_botanical_emf_qac_vibe_bundle,
    produce_botanical_emf_v4, produce_botanical_qac_v1, produce_vertebrate_emf_qac_vibe_bundle,
    produce_vertebrate_emf_v4, produce_vertebrate_qac_v1, qac_inspection_to_vibe_value,
    MODE_BIOELECTRIC_DIPOLE_V1, MODE_INSTANTANEOUS_E_B_V1, MODE_POLARIMETRIC_RAY_V1,
    MODE_SAMPLED_SPECTRUM_V1, QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_MODAL_RESONANCE,
    QAC_MODE_PCM_PRESERVATION, QAC_MODE_STFT_PRESERVATION,
};
use crate::{AssetHost, AssetWorkspace, ToolLimits};
use std::sync::atomic::AtomicBool;
use vibe::Value;

#[test]
fn vertebrate_emf_v4_and_qac_v1_double_produce_byte_and_digest_equality() {
    let (corpus, rig) = canonical_wallaby_rig_and_corpus().expect("wallaby rig and corpus");
    let summary = rig
        .validate(&corpus)
        .expect("canonical wallaby rig must validate against SpeciesCorpus");
    assert_eq!(summary.segment_count, 6);
    assert_eq!(summary.dipole_count, 3);
    assert_eq!(summary.cited_assertion_ids.len(), 9);

    let emf_a = produce_vertebrate_emf_v4(&rig, &corpus, "wallaby_a")
        .expect("first vertebrate EMF v4 produce must succeed");
    let emf_b = produce_vertebrate_emf_v4(&rig, &corpus, "wallaby_a")
        .expect("second vertebrate EMF v4 produce must succeed");

    assert_eq!(
        emf_a.canonical_resource.bytes,
        emf_b.canonical_resource.bytes,
        "vertebrate canonical EMF v4 bytes must be byte-for-byte deterministic"
    );
    assert_eq!(emf_a.canonical_sha256_hex, emf_b.canonical_sha256_hex);
    assert_eq!(emf_a.canonical_blake3_hex, emf_b.canonical_blake3_hex);
    assert_eq!(emf_a.profile_resource.bytes, emf_b.profile_resource.bytes);
    assert_eq!(emf_a.profile_sha256_hex, emf_b.profile_sha256_hex);
    assert_eq!(emf_a.profile_blake3_hex, emf_b.profile_blake3_hex);

    let qac_a = produce_vertebrate_qac_v1(&rig, &corpus, "wallaby_a")
        .expect("first vertebrate QAC v1 produce must succeed");
    let qac_b = produce_vertebrate_qac_v1(&rig, &corpus, "wallaby_a")
        .expect("second vertebrate QAC v1 produce must succeed");

    assert_eq!(
        qac_a.canonical_resource.bytes,
        qac_b.canonical_resource.bytes,
        "vertebrate canonical QAC v1 bytes must be byte-for-byte deterministic"
    );
    assert_eq!(qac_a.canonical_sha256_hex, qac_b.canonical_sha256_hex);
    assert_eq!(qac_a.canonical_blake3_hex, qac_b.canonical_blake3_hex);
    assert_eq!(
        qac_a.preservation_resource.bytes,
        qac_b.preservation_resource.bytes
    );
    assert_eq!(qac_a.preservation_sha256_hex, qac_b.preservation_sha256_hex);
    assert_eq!(qac_a.preservation_blake3_hex, qac_b.preservation_blake3_hex);
}

#[test]
fn botanical_emf_v4_and_qac_v1_double_produce_byte_and_digest_equality() {
    let (corpus, rig) =
        canonical_ghost_gum_metamer_rig_and_corpus().expect("ghost gum rig and corpus");
    let summary = rig
        .validate(&corpus)
        .expect("canonical ghost gum metamer rig must validate against SpeciesCorpus");
    assert_eq!(summary.metamer_count, 5);
    assert_eq!(summary.branching_junction_count, 2);
    assert!(summary.max_murray_xylem_relative_error <= 1e-3);

    let emf_a = produce_botanical_emf_v4(&rig, &corpus, "ghost_gum_a")
        .expect("first botanical EMF v4 produce must succeed");
    let emf_b = produce_botanical_emf_v4(&rig, &corpus, "ghost_gum_a")
        .expect("second botanical EMF v4 produce must succeed");

    assert_eq!(
        emf_a.canonical_resource.bytes,
        emf_b.canonical_resource.bytes,
        "botanical canonical EMF v4 bytes must be byte-for-byte deterministic"
    );
    assert_eq!(emf_a.canonical_sha256_hex, emf_b.canonical_sha256_hex);
    assert_eq!(emf_a.canonical_blake3_hex, emf_b.canonical_blake3_hex);
    assert_eq!(emf_a.profile_resource.bytes, emf_b.profile_resource.bytes);

    let qac_a = produce_botanical_qac_v1(&rig, &corpus, "ghost_gum_a")
        .expect("first botanical QAC v1 produce must succeed");
    let qac_b = produce_botanical_qac_v1(&rig, &corpus, "ghost_gum_a")
        .expect("second botanical QAC v1 produce must succeed");

    assert_eq!(
        qac_a.canonical_resource.bytes,
        qac_b.canonical_resource.bytes,
        "botanical canonical QAC v1 bytes must be byte-for-byte deterministic"
    );
    assert_eq!(qac_a.canonical_sha256_hex, qac_b.canonical_sha256_hex);
    assert_eq!(qac_a.canonical_blake3_hex, qac_b.canonical_blake3_hex);
    assert_eq!(
        qac_a.preservation_resource.bytes,
        qac_b.preservation_resource.bytes
    );
}

#[test]
fn round_trip_structural_and_semantic_inspection_and_vibe_adapter() {
    let (v_corpus, v_rig) = canonical_wallaby_rig_and_corpus().expect("wallaby rig and corpus");
    let v_emf = produce_vertebrate_emf_v4(&v_rig, &v_corpus, "wallaby_rt")
        .expect("vertebrate EMF v4 produce");
    let v_emf_report = inspect_and_verify_emf_v4_bundle(&v_emf)
        .expect("vertebrate EMF v4 round-trip inspection must pass");
    assert_eq!(v_emf_report.entry_count, 4);
    assert_eq!(
        v_emf_report.mode_record_counts.get(&MODE_INSTANTANEOUS_E_B_V1),
        Some(&6)
    );
    assert_eq!(
        v_emf_report.mode_record_counts.get(&MODE_POLARIMETRIC_RAY_V1),
        Some(&6)
    );
    assert_eq!(
        v_emf_report.mode_record_counts.get(&MODE_SAMPLED_SPECTRUM_V1),
        Some(&6)
    );
    assert_eq!(
        v_emf_report.mode_record_counts.get(&MODE_BIOELECTRIC_DIPOLE_V1),
        Some(&3)
    );
    assert_eq!(v_emf_report.verified_biofield_crc32c_count, 3);
    assert_eq!(v_emf_report.profile_reader_admitted_entries, 7);

    let v_qac = produce_vertebrate_qac_v1(&v_rig, &v_corpus, "wallaby_rt")
        .expect("vertebrate QAC v1 produce");
    let v_qac_report = inspect_and_verify_qac_v1_bundle(&v_qac)
        .expect("vertebrate QAC v1 round-trip inspection must pass");
    assert_eq!(v_qac_report.entry_count, 4);
    assert!(v_qac_report.signed_zero_preserved);
    assert_eq!(
        v_qac_report.first_two_pcm_bits,
        [0x0000_0000, 0x8000_0000]
    );
    assert!(
        v_qac_report
            .mode_record_counts
            .get(&QAC_MODE_PCM_PRESERVATION)
            .copied()
            .unwrap_or(0)
            >= 64
    );
    assert!(
        v_qac_report
            .mode_record_counts
            .get(&QAC_MODE_STFT_PRESERVATION)
            .copied()
            .unwrap_or(0)
            >= 1
    );
    assert!(
        v_qac_report
            .mode_record_counts
            .get(&QAC_MODE_HOA_SPATIAL_AUDIO)
            .copied()
            .unwrap_or(0)
            >= 64
    );
    assert_eq!(
        v_qac_report.mode_record_counts.get(&QAC_MODE_MODAL_RESONANCE),
        Some(&6)
    );

    if let Value::Record(rec) = emf_inspection_to_vibe_value(&v_emf_report) {
        assert_eq!(
            rec.get("format"),
            Some(&Value::String("Q42EMF64_v4".into()))
        );
    } else {
        panic!("expected Value::Record from emf_inspection_to_vibe_value");
    }
    if let Value::Record(rec) = qac_inspection_to_vibe_value(&v_qac_report) {
        assert_eq!(rec.get("format"), Some(&Value::String("qac_v1".into())));
    } else {
        panic!("expected Value::Record from qac_inspection_to_vibe_value");
    }

    let limits = ToolLimits::default();
    let cancel = AtomicBool::new(false);
    let mut host = AssetHost {
        workspace: AssetWorkspace::default(),
        inputs: &[],
        limits: &limits,
        cancel: &cancel,
        work: 0,
    };
    let v_val = produce_vertebrate_emf_qac_vibe_bundle(&mut host, &v_rig, &v_corpus, "wallaby_host")
        .expect("host vertebrate vibe bundle must succeed");
    if let Value::Record(top) = v_val {
        assert!(top.contains_key("emf_v4"));
        assert!(top.contains_key("qac_v1"));
        assert!(top.contains_key("conversion_lineage"));
    } else {
        panic!("expected Value::Record from produce_vertebrate_emf_qac_vibe_bundle");
    }

    let (b_corpus, b_rig) =
        canonical_ghost_gum_metamer_rig_and_corpus().expect("ghost gum rig and corpus");
    let b_val = produce_botanical_emf_qac_vibe_bundle(&mut host, &b_rig, &b_corpus, "ghost_gum_host")
        .expect("host botanical vibe bundle must succeed");
    if let Value::Record(top) = b_val {
        assert!(top.contains_key("emf_v4"));
        assert!(top.contains_key("qac_v1"));
        assert!(top.contains_key("conversion_lineage"));
    } else {
        panic!("expected Value::Record from produce_botanical_emf_qac_vibe_bundle");
    }
}

#[test]
fn signed_zero_pcm_preservation_round_trip() {
    let (b_corpus, b_rig) =
        canonical_ghost_gum_metamer_rig_and_corpus().expect("ghost gum rig and corpus");
    let qac_bundle = produce_botanical_qac_v1(&b_rig, &b_corpus, "ghost_gum_sz")
        .expect("botanical QAC v1 produce must succeed");
    assert_eq!(
        qac_bundle.signed_zero_anchor_bits,
        [0x0000_0000, 0x8000_0000]
    );
    let report = inspect_and_verify_qac_v1_bundle(&qac_bundle)
        .expect("botanical QAC v1 inspection must succeed");
    assert!(report.signed_zero_preserved);
    assert_eq!(report.first_two_pcm_bits[0], 0x0000_0000);
    assert_eq!(report.first_two_pcm_bits[1], 0x8000_0000);
}

#[test]
fn rejects_ellipsoid_placeholders_and_invalid_anatomical_or_botanical_inputs() {
    let (v_corpus, mut v_rig) =
        canonical_wallaby_rig_and_corpus().expect("wallaby rig and corpus");
    v_rig.segments[0].segment_id = "comparative_ellipsoid_scaffold".into();
    let err = v_rig
        .validate(&v_corpus)
        .expect_err("comparative ellipsoid placeholder must be rejected");
    assert!(err.contains("ellipsoid placeholder"));

    let (v_corpus2, mut v_rig2) =
        canonical_wallaby_rig_and_corpus().expect("wallaby rig and corpus");
    v_rig2.dipoles[0].assertion_id = "non-existent-assertion-id".into();
    let err2 = v_rig2
        .validate(&v_corpus2)
        .expect_err("missing SpeciesCorpus assertion must be rejected");
    assert!(err2.contains("unknown or inactive corpus assertion"));

    let (b_corpus, mut b_rig) =
        canonical_ghost_gum_metamer_rig_and_corpus().expect("ghost gum rig and corpus");
    // Break Murray's law on child xylem radius
    b_rig.metamers[1].xylem_radius_m = b_rig.metamers[0].xylem_radius_m * 0.99;
    let err3 = b_rig
        .validate(&b_corpus)
        .expect_err("violated Murray's law must be rejected");
    assert!(err3.contains("Murray's law"));

    let (b_corpus2, mut b_rig2) =
        canonical_ghost_gum_metamer_rig_and_corpus().expect("ghost gum rig and corpus");
    // Flatten chlorophyll red-edge reflectance so NIR <= red
    for r in &mut b_rig2.metamers[3].red_edge_reflectance_bins {
        *r = 0.05;
    }
    let err4 = b_rig2
        .validate(&b_corpus2)
        .expect_err("flat red-edge reflectance must be rejected");
    assert!(err4.contains("red-edge"));
}

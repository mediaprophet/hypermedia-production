//! Packet T02 (`producer_engineer` | Gates: `G-PRODUCE`, `G-CUTOVER`):
//! Deterministic Current-Format EMF v4 (`Q42EMF64` version 4) and QAC v1 (`qac\0\0\0\0\0`
//! version 1) Anatomical Rig & Botanical Metamer Production Bridge.
//!
//! Submodules (all strictly <= 500–800 lines):
//! - [`rig_metamer`]: Articulated vertebrate anatomical rig & botanical Murray-law FSPM metamer
//!   descriptors backed by `SpeciesCorpus` evidenced assertions (zero comparative ellipsoid
//!   placeholders).
//! - [`rig_fixtures`]: Canonical evidenced *Wallabia bicolor* (Swamp Wallaby) and *Corymbia
//!   aparrerinja* (Ghost Gum) rig + `SpeciesCorpus` constructors.
//! - [`emf_v4_producer`]: Deterministic EMF v4 (`Q42EMF64` version 4) producer for Modes
//!   `0x0100` (`instantaneous_E_B_v1`), `0x0101` (`polarimetric_ray_v1`), `0x0103`
//!   (`sampled_spectrum_v1`), and `0x0107` (`bioelectric_dipole_v1`).
//! - [`emf_v4_envelope`]: Canonical 128B header + 128B directory + 8B TLV metadata + 64B payload
//!   envelope assembler for EMF v4.
//! - [`qac_v1_producer`]: Deterministic QAC v1 (`qac\0\0\0\0\0` version 1) acoustic cue producer
//!   for Modes `0x0100` (`pcm_preservation`), `0x0101` (`stft_preservation`), `0x0102`
//!   (`hoa_spatial_audio`), and `0x0104` (`modal_resonance`).
//! - [`qac_v1_envelope`]: Canonical 4-mode QAC v1 envelope assembler.
//! - [`inspection`]: Round-trip structural & semantic verification (`StructuralView::open`,
//!   `emf_profiles::ProfileReader`, `QacStorageView::open`, `audio::qac::signal::admit`) and
//!   `vibe::Value` record adapter.

pub mod emf_v4_envelope;
pub mod emf_v4_producer;
pub mod inspection;
pub mod qac_v1_envelope;
pub mod qac_v1_producer;
pub mod rig_fixtures;
pub mod rig_metamer;

#[cfg(test)]
mod tests;

pub use emf_v4_producer::{
    encode_bioelectric_dipole_record, encode_instantaneous_e_b_record,
    encode_polarimetric_ray_record, encode_sampled_spectrum_record, produce_botanical_emf_v4,
    produce_vertebrate_emf_v4, sampled_spectrum_stride, sha256_hex, EmfV4AnatomyRegistry,
    EmfV4ProducedBundle, BIOELECTRIC_MODEL_URI, BIOELECTRIC_PROFILE, CANDIDATE_MODE_E_B,
    CANDIDATE_MODE_RAY, CANDIDATE_MODE_SPECTRUM, MODE_BIOELECTRIC_DIPOLE_V1,
    MODE_INSTANTANEOUS_E_B_V1, MODE_POLARIMETRIC_RAY_V1, MODE_SAMPLED_SPECTRUM_V1,
};
pub use inspection::{
    emf_inspection_to_vibe_value, inspect_and_verify_emf_v4_bundle,
    inspect_and_verify_qac_v1_bundle, qac_inspection_to_vibe_value, EmfV4InspectionReport,
    QacV1InspectionReport,
};
pub use qac_v1_producer::{
    encode_hoa_order1_sn3d_frame, encode_modal_resonance_record, produce_botanical_qac_v1,
    produce_vertebrate_qac_v1, QacV1CueRegistry, QacV1ProducedBundle, HOA_MODEL_URI,
    MODAL_MODEL_URI, QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_MODAL_RESONANCE,
    QAC_MODE_PCM_PRESERVATION, QAC_MODE_STFT_PRESERVATION,
};
pub use rig_metamer::{
    canonical_ghost_gum_metamer_rig_and_corpus, canonical_wallaby_rig_and_corpus,
    planck_spectral_radiance_w_sr_m3, ArticulatedVertebrateSegment, BioelectricDipoleSource,
    BioelectricSourceKind, BotanicalMetamerKind, BotanicalMetamerRig, BotanicalMetamerSegment,
    BotanicalRigValidationSummary, VertebrateAcousticCueSpec, VertebrateAnatomicalRig,
    VertebrateRigValidationSummary, VertebrateSegmentKind,
};

use crate::AssetHost;
use qualia_core_db::q42::species_corpus::SpeciesCorpus;
use std::collections::BTreeMap;
use vibe::Value;

/// Produce, round-trip verify, and admit a vertebrate EMF v4 + QAC v1 bundle into `AssetHost`,
/// returning a deterministic `vibe::Value::Record` with resource handles, inspection summaries,
/// and explicit conversion lineage metadata.
pub fn produce_vertebrate_emf_qac_vibe_bundle(
    host: &mut AssetHost<'_>,
    rig: &VertebrateAnatomicalRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<Value, String> {
    let emf_bundle = produce_vertebrate_emf_v4(rig, corpus, key_prefix)?;
    let emf_report = inspect_and_verify_emf_v4_bundle(&emf_bundle)?;
    let qac_bundle = produce_vertebrate_qac_v1(rig, corpus, key_prefix)?;
    let qac_report = inspect_and_verify_qac_v1_bundle(&qac_bundle)?;

    let emf_canonical_handle =
        host.admit_encoded("emf", emf_bundle.canonical_resource.bytes.clone())?;
    let emf_profile_handle =
        host.admit_encoded("emf", emf_bundle.profile_resource.bytes.clone())?;
    let qac_canonical_handle =
        host.admit_encoded("qac", qac_bundle.canonical_resource.bytes.clone())?;
    let qac_preservation_handle =
        host.admit_encoded("qac", qac_bundle.preservation_resource.bytes.clone())?;

    let mut lineage = BTreeMap::new();
    lineage.insert(
        "domain".into(),
        Value::String("vertebrate_anatomical_rig".into()),
    );
    lineage.insert("rig_id".into(), Value::String(rig.rig_id.clone()));
    lineage.insert("species_id".into(), Value::String(corpus.species_id.clone()));
    lineage.insert("corpus_iri".into(), Value::String(corpus.corpus_iri.clone()));
    lineage.insert(
        "ellipsoid_placeholders_used".into(),
        Value::Bool(false),
    );
    lineage.insert(
        "emf_wire_freeze_version".into(),
        Value::String("Q42EMF64_v4".into()),
    );
    lineage.insert(
        "qac_wire_freeze_version".into(),
        Value::String("qac_v1".into()),
    );

    let mut out = BTreeMap::new();
    out.insert("emf_canonical_handle".into(), emf_canonical_handle);
    out.insert("emf_profile_handle".into(), emf_profile_handle);
    out.insert("qac_canonical_handle".into(), qac_canonical_handle);
    out.insert("qac_preservation_handle".into(), qac_preservation_handle);
    out.insert("emf_v4".into(), emf_inspection_to_vibe_value(&emf_report));
    out.insert("qac_v1".into(), qac_inspection_to_vibe_value(&qac_report));
    out.insert("conversion_lineage".into(), Value::Record(lineage));
    Ok(Value::Record(out))
}

/// Produce, round-trip verify, and admit a botanical FSPM metamer EMF v4 + QAC v1 bundle into
/// `AssetHost`, returning a deterministic `vibe::Value::Record` with resource handles, inspection
/// summaries, and explicit conversion lineage metadata.
pub fn produce_botanical_emf_qac_vibe_bundle(
    host: &mut AssetHost<'_>,
    rig: &BotanicalMetamerRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<Value, String> {
    let emf_bundle = produce_botanical_emf_v4(rig, corpus, key_prefix)?;
    let emf_report = inspect_and_verify_emf_v4_bundle(&emf_bundle)?;
    let qac_bundle = produce_botanical_qac_v1(rig, corpus, key_prefix)?;
    let qac_report = inspect_and_verify_qac_v1_bundle(&qac_bundle)?;

    let emf_canonical_handle =
        host.admit_encoded("emf", emf_bundle.canonical_resource.bytes.clone())?;
    let emf_profile_handle =
        host.admit_encoded("emf", emf_bundle.profile_resource.bytes.clone())?;
    let qac_canonical_handle =
        host.admit_encoded("qac", qac_bundle.canonical_resource.bytes.clone())?;
    let qac_preservation_handle =
        host.admit_encoded("qac", qac_bundle.preservation_resource.bytes.clone())?;

    let mut lineage = BTreeMap::new();
    lineage.insert(
        "domain".into(),
        Value::String("botanical_fspm_metamer_rig".into()),
    );
    lineage.insert("plant_id".into(), Value::String(rig.plant_id.clone()));
    lineage.insert("species_id".into(), Value::String(corpus.species_id.clone()));
    lineage.insert("corpus_iri".into(), Value::String(corpus.corpus_iri.clone()));
    lineage.insert(
        "murray_law_enforced".into(),
        Value::Bool(true),
    );
    lineage.insert(
        "ellipsoid_placeholders_used".into(),
        Value::Bool(false),
    );
    lineage.insert(
        "emf_wire_freeze_version".into(),
        Value::String("Q42EMF64_v4".into()),
    );
    lineage.insert(
        "qac_wire_freeze_version".into(),
        Value::String("qac_v1".into()),
    );

    let mut out = BTreeMap::new();
    out.insert("emf_canonical_handle".into(), emf_canonical_handle);
    out.insert("emf_profile_handle".into(), emf_profile_handle);
    out.insert("qac_canonical_handle".into(), qac_canonical_handle);
    out.insert("qac_preservation_handle".into(), qac_preservation_handle);
    out.insert("emf_v4".into(), emf_inspection_to_vibe_value(&emf_report));
    out.insert("qac_v1".into(), qac_inspection_to_vibe_value(&qac_report));
    out.insert("conversion_lineage".into(), Value::Record(lineage));
    Ok(Value::Record(out))
}

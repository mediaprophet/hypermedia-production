//! Round-Trip Inspection, Verification & Vibe Value Adapter (`inspection.rs`).
//!
//! Verifies and inspects produced EMF v4 (`Q42EMF64` version 4) and QAC v1 (`qac\0\0\0\0\0`
//! version 1) `Resource` containers using `StructuralView::open`, `emf_profiles::ProfileReader`,
//! `QacStorageView::open`, and `audio::qac::signal::admit`, returning structured `vibe::Value`
//! records with mode summaries, stream counts, exact clock/frame metadata, BLAKE3/SHA-256
//! digests, and explicit conversion lineage metadata.

use crate::emf_qac_anatomy_bridge::emf_v4_producer::{
    sampled_spectrum_stride, EmfV4AnatomyRegistry, EmfV4ProducedBundle, MODE_BIOELECTRIC_DIPOLE_V1,
    MODE_INSTANTANEOUS_E_B_V1, MODE_POLARIMETRIC_RAY_V1, MODE_SAMPLED_SPECTRUM_V1,
};
use crate::emf_qac_anatomy_bridge::qac_v1_producer::{
    QacV1CueRegistry, QacV1ProducedBundle, QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_MODAL_RESONANCE,
    QAC_MODE_PCM_PRESERVATION, QAC_MODE_STFT_PRESERVATION,
};
use qualia_core_db::audio::qac::metadata::{OriginBasis, Transform};
use qualia_core_db::audio::qac::signal::{
    admit as admit_qac_signal, decode_sample, Limits as QacSignalLimits, Sample, Scalar,
};
use qualia_core_db::audio::qac::{QacStorageOptions, QacStorageView};
use qualia_core_db::container_10d::crc32c::crc32c;
use qualia_core_db::q42::emf_profiles::{Limits as EmfProfileLimits, ProfileReader};
use qualia_core_db::q42::field_envelope::{Family, Limits as StorageLimits, StructuralView};
use qualia_core_db::q42::field_semantics::{
    self as semantics, Limits as SharedLimits, NodeState, Scratch,
};
use std::collections::BTreeMap;
use vibe::Value;

/// Structured round-trip inspection report for a produced EMF v4 bundle.
#[derive(Clone, Debug, PartialEq)]
pub struct EmfV4InspectionReport {
    pub species_id: String,
    pub primary_anatomy_iri: String,
    pub canonical_sha256_hex: String,
    pub canonical_blake3_hex: String,
    pub profile_sha256_hex: String,
    pub profile_blake3_hex: String,
    pub entry_count: u32,
    pub mode_record_counts: BTreeMap<u16, u64>,
    pub spectral_bin_count: u32,
    pub bioelectric_dipole_count: u64,
    pub verified_biofield_crc32c_count: u64,
    pub clock_rate_hz_num: u64,
    pub clock_rate_hz_den: u64,
    pub frame_handedness: String,
    pub profile_reader_admitted_entries: u32,
}

/// Structured round-trip inspection report for a produced QAC v1 bundle.
#[derive(Clone, Debug, PartialEq)]
pub struct QacV1InspectionReport {
    pub canonical_sha256_hex: String,
    pub canonical_blake3_hex: String,
    pub preservation_sha256_hex: String,
    pub preservation_blake3_hex: String,
    pub entry_count: u32,
    pub mode_record_counts: BTreeMap<u16, u64>,
    pub sample_rate_hz: u32,
    pub stft_fft_size: u32,
    pub stft_hop_size: u32,
    pub stft_window_len: u32,
    pub signed_zero_preserved: bool,
    pub first_two_pcm_bits: [u32; 2],
    pub hoa_channel_count: u32,
    pub modal_mode_count: u64,
    pub signal_admitted_entries: u32,
}

/// Inspect and verify both the canonical 4-mode EMF v4 container (`0x0100, 0x0101, 0x0103, 0x0107`)
/// and the companion 3-mode `emf_profiles` container (`0, 1, 3`).
pub fn inspect_and_verify_emf_v4_bundle(
    bundle: &EmfV4ProducedBundle,
) -> Result<EmfV4InspectionReport, String> {
    let bytes = &bundle.canonical_resource.bytes;
    let view = StructuralView::open(bytes, Family::EmfV4Candidate, StorageLimits::default())
        .map_err(|e| format!("EMF v4 canonical StructuralView::open failed: {e:?}"))?;
    if view.entry_count() != 4 {
        return Err(format!(
            "EMF v4 canonical container must have 4 entries, got {}",
            view.entry_count()
        ));
    }

    let mut states = [NodeState::default(); 64];
    let mut visited = [NodeState::default(); 64];
    let mut scratch = Scratch {
        metadata: &mut states,
        entries: &mut visited,
    };
    let shared = semantics::admit_shared(
        &view,
        &EmfV4AnatomyRegistry,
        &mut scratch,
        SharedLimits::default(),
    )
    .map_err(|e| format!("EMF v4 canonical shared semantic admission failed: {e:?}"))?;

    let clock_block = shared
        .context
        .require_schema(1, 1)
        .map_err(|e| format!("EMF v4 clock block 1 missing: {e:?}"))?;
    let clock_rate_hz_num = u64_field(clock_block, 4)?;
    let clock_rate_hz_den = u64_field(clock_block, 5)?;

    let frame_block = shared
        .context
        .require_schema(2, 2)
        .map_err(|e| format!("EMF v4 frame block 2 missing: {e:?}"))?;
    let frame_handedness = utf8_field(frame_block, 8)?.to_string();

    // Verify all 4 chunks through ProfileReader as well as explicit byte-level checks.
    let mut prof_states = [NodeState::default(); 64];
    let mut prof_visited = [NodeState::default(); 64];
    let mut prof_scratch = Scratch {
        metadata: &mut prof_states,
        entries: &mut prof_visited,
    };
    let mut reader = ProfileReader::open(
        &view,
        &mut prof_scratch,
        EmfProfileLimits::default(),
        64 << 20,
    )
    .map_err(|e| format!("EMF v4 canonical ProfileReader::open failed: {e:?}"))?;

    let chunk1 = reader
        .chunk(1)
        .map_err(|e| format!("EMF v4 canonical entry 1 (0x0100) admission failed: {e:?}"))?;
    if chunk1.mode() != MODE_INSTANTANEOUS_E_B_V1 || chunk1.record_count() == 0 {
        return Err("EMF v4 entry 1 mode/count mismatch".into());
    }
    let first_field = chunk1
        .field(0)
        .map_err(|e| format!("EMF v4 entry 1 field(0) failed: {e:?}"))?
        .ok_or_else(|| "EMF v4 entry 1 field(0) unexpectedly masked".to_string())?;
    if first_field.carrier_hz().unwrap_or(0.0) <= 0.0
        || first_field.stored_poynting_w_per_m2().is_none()
        || first_field.confidence().is_none()
    {
        return Err("EMF v4 entry 1 field(0) missing required carrier/poynting/confidence".into());
    }

    let chunk2 = reader
        .chunk(2)
        .map_err(|e| format!("EMF v4 canonical entry 2 (0x0101) admission failed: {e:?}"))?;
    if chunk2.mode() != MODE_POLARIMETRIC_RAY_V1 || chunk2.record_count() == 0 {
        return Err("EMF v4 entry 2 mode/count mismatch".into());
    }
    let first_ray = chunk2
        .ray(0)
        .map_err(|e| format!("EMF v4 entry 2 ray(0) failed: {e:?}"))?
        .ok_or_else(|| "EMF v4 entry 2 ray(0) unexpectedly masked".to_string())?;
    let stokes = first_ray.stokes();
    if stokes[0] <= 0.0 {
        return Err("EMF v4 entry 2 ray(0) Stokes I must be > 0".into());
    }

    let chunk3 = reader
        .chunk(3)
        .map_err(|e| format!("EMF v4 canonical entry 3 (0x0103) admission failed: {e:?}"))?;
    if chunk3.mode() != MODE_SAMPLED_SPECTRUM_V1 || chunk3.record_count() == 0 {
        return Err("EMF v4 entry 3 mode/count mismatch".into());
    }
    let expected_spec_stride = sampled_spectrum_stride(bundle.spectral_wavelengths_m.len()) as u32;
    if view.entry(3).map_err(|e| format!("{e:?}"))?.stride() != expected_spec_stride {
        return Err("EMF v4 entry 3 sampled_spectrum_v1 stride mismatch".into());
    }
    let first_spec = chunk3
        .spectrum(0)
        .map_err(|e| format!("EMF v4 entry 3 spectrum(0) failed: {e:?}"))?
        .ok_or_else(|| "EMF v4 entry 3 spectrum(0) unexpectedly masked".to_string())?;
    if first_spec.temperature_kelvin().unwrap_or(0.0) <= 0.0
        || first_spec.values().count() != bundle.spectral_wavelengths_m.len()
    {
        return Err("EMF v4 entry 3 spectrum(0) temperature/bin count mismatch".into());
    }

    let chunk4 = reader
        .chunk(4)
        .map_err(|e| format!("EMF v4 canonical entry 4 (0x0107) admission failed: {e:?}"))?;
    if chunk4.mode() != MODE_BIOELECTRIC_DIPOLE_V1 || chunk4.record_count() == 0 {
        return Err("EMF v4 entry 4 mode/count mismatch".into());
    }
    let first_bio = chunk4
        .biofield(0)
        .map_err(|e| format!("EMF v4 entry 4 biofield(0) failed: {e:?}"))?
        .ok_or_else(|| "EMF v4 entry 4 biofield(0) unexpectedly masked".to_string())?;
    if first_bio.anatomy_metadata_ref != 9
        || first_bio.model_ref != 8
        || first_bio.calibration_ref != 5
        || first_bio.consent_provenance_ref != 4
    {
        return Err("EMF v4 entry 4 biofield(0) metadata reference mismatch".into());
    }

    let raw_bio_bytes = view
        .verify_chunk(4)
        .map_err(|e| format!("EMF v4 verify_chunk(4) failed: {e:?}"))?;
    let mut verified_crc_count = 0u64;
    for rec in raw_bio_bytes.chunks_exact(128) {
        let expected_crc = crc32c(&rec[0..124]);
        let stored_crc = u32::from_le_bytes(rec[124..128].try_into().unwrap());
        if expected_crc != stored_crc {
            return Err("EMF v4 entry 4 bioelectric_dipole_v1 CRC-32C mismatch".into());
        }
        verified_crc_count += 1;
    }

    // Also verify the companion 3-mode candidate container (`0, 1, 3`).
    let prof_view = StructuralView::open(
        &bundle.profile_resource.bytes,
        Family::EmfV4Candidate,
        StorageLimits::default(),
    )
    .map_err(|e| format!("EMF v4 profile StructuralView::open failed: {e:?}"))?;
    let mut p_states = [NodeState::default(); 64];
    let mut p_visited = [NodeState::default(); 64];
    let mut p_scratch = Scratch {
        metadata: &mut p_states,
        entries: &mut p_visited,
    };
    let mut p_reader = ProfileReader::open(
        &prof_view,
        &mut p_scratch,
        EmfProfileLimits::default(),
        64 << 20,
    )
    .map_err(|e| format!("EMF v4 companion ProfileReader::open failed: {e:?}"))?;
    for idx in 1..=prof_view.entry_count() {
        let _ = p_reader
            .chunk(idx)
            .map_err(|e| format!("EMF v4 companion chunk({idx}) failed: {e:?}"))?;
    }

    let mut mode_record_counts = BTreeMap::new();
    for idx in 1..=view.entry_count() {
        let e = view.entry(idx).map_err(|err| format!("{err:?}"))?;
        mode_record_counts.insert(e.mode(), e.record_count());
    }

    Ok(EmfV4InspectionReport {
        species_id: bundle.species_id.clone(),
        primary_anatomy_iri: bundle.primary_anatomy_iri.clone(),
        canonical_sha256_hex: bundle.canonical_sha256_hex.clone(),
        canonical_blake3_hex: bundle.canonical_blake3_hex.clone(),
        profile_sha256_hex: bundle.profile_sha256_hex.clone(),
        profile_blake3_hex: bundle.profile_blake3_hex.clone(),
        entry_count: view.entry_count(),
        mode_record_counts,
        spectral_bin_count: bundle.spectral_wavelengths_m.len() as u32,
        bioelectric_dipole_count: chunk4.record_count(),
        verified_biofield_crc32c_count: verified_crc_count,
        clock_rate_hz_num,
        clock_rate_hz_den,
        frame_handedness,
        profile_reader_admitted_entries: view.entry_count() + prof_view.entry_count(),
    })
}

/// Inspect and verify both the canonical 4-mode QAC v1 container (`0x0100, 0x0101, 0x0102, 0x0104`)
/// and the companion 2-mode `audio::qac::signal::admit` preservation container (`0x0100, 0x0101`).
pub fn inspect_and_verify_qac_v1_bundle(
    bundle: &QacV1ProducedBundle,
) -> Result<QacV1InspectionReport, String> {
    let qac_view = QacStorageView::open(&bundle.canonical_resource.bytes, QacStorageOptions::default())
        .map_err(|e| format!("QAC v1 canonical QacStorageView::open failed: {e:?}"))?;
    let view = qac_view.structure();
    if view.entry_count() != 4 {
        return Err(format!(
            "QAC v1 canonical container must have 4 entries, got {}",
            view.entry_count()
        ));
    }

    let mut states = [NodeState::default(); 64];
    let mut visited = [NodeState::default(); 64];
    let mut scratch = Scratch {
        metadata: &mut states,
        entries: &mut visited,
    };
    let shared = semantics::admit_shared(
        view,
        &QacV1CueRegistry,
        &mut scratch,
        SharedLimits::default(),
    )
    .map_err(|e| format!("QAC v1 canonical shared semantic admission failed: {e:?}"))?;

    // Entry 1: Mode 0x0100 (`pcm_preservation`) with signed-zero anchor verification
    let e1 = view.entry(1).map_err(|e| format!("{e:?}"))?;
    if e1.mode() != QAC_MODE_PCM_PRESERVATION || e1.stride() != 4 {
        return Err("QAC v1 entry 1 mode/stride mismatch".into());
    }
    let pcm_bytes = view
        .verify_chunk(1)
        .map_err(|e| format!("QAC v1 verify_chunk(1) failed: {e:?}"))?;
    if pcm_bytes.len() < 8 {
        return Err("QAC v1 entry 1 must contain at least 2 f32 samples".into());
    }
    let s0 = match decode_sample(Scalar::F32, &pcm_bytes[0..4])
        .map_err(|e| format!("QAC v1 sample 0 decode failed: {e:?}"))?
    {
        Sample::F32(v) => v,
        _ => return Err("QAC v1 sample 0 not F32".to_string()),
    };
    let s1 = match decode_sample(Scalar::F32, &pcm_bytes[4..8])
        .map_err(|e| format!("QAC v1 sample 1 decode failed: {e:?}"))?
    {
        Sample::F32(v) => v,
        _ => return Err("QAC v1 sample 1 not F32".to_string()),
    };
    let bits0 = s0.to_bits();
    let bits1 = s1.to_bits();
    let signed_zero_preserved = bits0 == 0x0000_0000 && bits1 == 0x8000_0000;
    if !signed_zero_preserved {
        return Err(format!(
            "QAC v1 signed-zero anchor bits not preserved: got [0x{bits0:08x}, 0x{bits1:08x}]"
        ));
    }

    // Entry 2: Mode 0x0101 (`stft_preservation`, Schema 201 rev 1)
    let e2 = view.entry(2).map_err(|e| format!("{e:?}"))?;
    if e2.mode() != QAC_MODE_STFT_PRESERVATION {
        return Err("QAC v1 entry 2 mode mismatch".into());
    }
    let _stft_bytes = view
        .verify_chunk(2)
        .map_err(|e| format!("QAC v1 verify_chunk(2) failed: {e:?}"))?;
    let transform_block = shared
        .context
        .require_schema(11, 201)
        .map_err(|e| format!("QAC v1 Schema 201 block 11 missing: {e:?}"))?;
    let transform = Transform::from_block(transform_block)
        .map_err(|e| format!("QAC v1 Transform::from_block failed: {e:?}"))?;
    if transform.origin_basis != OriginBasis::SourceEntryFirstSample {
        return Err("QAC v1 STFT origin_basis must be SourceEntryFirstSample".into());
    }
    let window_bytes = semantics::field(transform_block, 16)
        .ok_or_else(|| "QAC v1 Schema 201 inline Hann window key 16 missing".to_string())?
        .value();
    let stft_window_len = (window_bytes.len() / 4) as u32;
    if stft_window_len != transform.fft_size {
        return Err("QAC v1 inline Hann window length != fft_size".into());
    }

    // Entry 3: Mode 0x0102 (`hoa_spatial_audio`, 16B stride, Schema 204)
    let e3 = view.entry(3).map_err(|e| format!("{e:?}"))?;
    if e3.mode() != QAC_MODE_HOA_SPATIAL_AUDIO || e3.stride() != 16 {
        return Err("QAC v1 entry 3 (HOA) mode/stride mismatch".into());
    }
    let hoa_bytes = view
        .verify_chunk(3)
        .map_err(|e| format!("QAC v1 verify_chunk(3) failed: {e:?}"))?;
    for ch_bytes in hoa_bytes.chunks_exact(4) {
        let v = f32::from_le_bytes(ch_bytes.try_into().unwrap());
        if !v.is_finite() {
            return Err("QAC v1 entry 3 (HOA) contains non-finite sample".into());
        }
    }
    let _hoa_block = shared
        .context
        .require_schema(13, 204)
        .map_err(|e| format!("QAC v1 Schema 204 block 13 missing: {e:?}"))?;

    // Entry 4: Mode 0x0104 (`modal_resonance`, 32B stride, Schema 206)
    let e4 = view.entry(4).map_err(|e| format!("{e:?}"))?;
    if e4.mode() != QAC_MODE_MODAL_RESONANCE || e4.stride() != 32 {
        return Err("QAC v1 entry 4 (modal_resonance) mode/stride mismatch".into());
    }
    let modal_bytes = view
        .verify_chunk(4)
        .map_err(|e| format!("QAC v1 verify_chunk(4) failed: {e:?}"))?;
    for rec in modal_bytes.chunks_exact(32) {
        let freq = f32::from_le_bytes(rec[0..4].try_into().unwrap());
        let amp = f32::from_le_bytes(rec[4..8].try_into().unwrap());
        let decay = f32::from_le_bytes(rec[8..12].try_into().unwrap());
        let reserved = u32::from_le_bytes(rec[28..32].try_into().unwrap());
        if !freq.is_finite() || freq <= 0.0 || !amp.is_finite() || amp <= 0.0 || !decay.is_finite() || decay <= 0.0 || reserved != 0 {
            return Err("QAC v1 entry 4 modal_resonance record invalid".into());
        }
    }
    let _modal_block = shared
        .context
        .require_schema(14, 206)
        .map_err(|e| format!("QAC v1 Schema 206 block 14 missing: {e:?}"))?;

    // Also admit the 2-mode preservation container through `qualia_core_db::audio::qac::signal::admit`.
    let pres_view = QacStorageView::open(
        &bundle.preservation_resource.bytes,
        QacStorageOptions::default(),
    )
    .map_err(|e| format!("QAC v1 preservation QacStorageView::open failed: {e:?}"))?;
    let mut s1_states = [NodeState::default(); 64];
    let mut s1_visited = [NodeState::default(); 64];
    let mut s1_scratch = Scratch {
        metadata: &mut s1_states,
        entries: &mut s1_visited,
    };
    let sig_pcm = admit_qac_signal(
        pres_view.structure(),
        1,
        &mut s1_scratch,
        SharedLimits::default(),
        QacSignalLimits::default(),
    )
    .map_err(|e| format!("QAC v1 signal::admit entry 1 (PCM) failed: {e:?}"))?;
    if sig_pcm.mode() != QAC_MODE_PCM_PRESERVATION {
        return Err("QAC v1 signal::admit entry 1 mode mismatch".into());
    }

    let mut s2_states = [NodeState::default(); 64];
    let mut s2_visited = [NodeState::default(); 64];
    let mut s2_scratch = Scratch {
        metadata: &mut s2_states,
        entries: &mut s2_visited,
    };
    let sig_stft = admit_qac_signal(
        pres_view.structure(),
        2,
        &mut s2_scratch,
        SharedLimits::default(),
        QacSignalLimits::default(),
    )
    .map_err(|e| format!("QAC v1 signal::admit entry 2 (STFT) failed: {e:?}"))?;
    if sig_stft.mode() != QAC_MODE_STFT_PRESERVATION {
        return Err("QAC v1 signal::admit entry 2 mode mismatch".into());
    }

    let mut mode_record_counts = BTreeMap::new();
    for idx in 1..=view.entry_count() {
        let e = view.entry(idx).map_err(|err| format!("{err:?}"))?;
        mode_record_counts.insert(e.mode(), e.record_count());
    }

    Ok(QacV1InspectionReport {
        canonical_sha256_hex: bundle.canonical_sha256_hex.clone(),
        canonical_blake3_hex: bundle.canonical_blake3_hex.clone(),
        preservation_sha256_hex: bundle.preservation_sha256_hex.clone(),
        preservation_blake3_hex: bundle.preservation_blake3_hex.clone(),
        entry_count: view.entry_count(),
        mode_record_counts,
        sample_rate_hz: bundle.sample_rate_hz,
        stft_fft_size: transform.fft_size,
        stft_hop_size: transform.hop,
        stft_window_len,
        signed_zero_preserved,
        first_two_pcm_bits: [bits0, bits1],
        hoa_channel_count: 4,
        modal_mode_count: e4.record_count(),
        signal_admitted_entries: 2,
    })
}

/// Convert an `EmfV4InspectionReport` into a deterministic `vibe::Value::Record`.
pub fn emf_inspection_to_vibe_value(report: &EmfV4InspectionReport) -> Value {
    let mut map = BTreeMap::new();
    map.insert("format".into(), Value::String("Q42EMF64_v4".into()));
    map.insert("species_id".into(), Value::String(report.species_id.clone()));
    map.insert(
        "primary_anatomy_iri".into(),
        Value::String(report.primary_anatomy_iri.clone()),
    );
    map.insert(
        "canonical_sha256".into(),
        Value::String(report.canonical_sha256_hex.clone()),
    );
    map.insert(
        "canonical_blake3".into(),
        Value::String(report.canonical_blake3_hex.clone()),
    );
    map.insert(
        "profile_sha256".into(),
        Value::String(report.profile_sha256_hex.clone()),
    );
    map.insert(
        "profile_blake3".into(),
        Value::String(report.profile_blake3_hex.clone()),
    );
    map.insert("entry_count".into(), Value::U64(report.entry_count as u64));
    map.insert(
        "spectral_bin_count".into(),
        Value::U64(report.spectral_bin_count as u64),
    );
    map.insert(
        "bioelectric_dipole_count".into(),
        Value::U64(report.bioelectric_dipole_count),
    );
    map.insert(
        "verified_biofield_crc32c_count".into(),
        Value::U64(report.verified_biofield_crc32c_count),
    );
    map.insert(
        "clock_rate_hz_num".into(),
        Value::U64(report.clock_rate_hz_num),
    );
    map.insert(
        "clock_rate_hz_den".into(),
        Value::U64(report.clock_rate_hz_den),
    );
    map.insert(
        "frame_handedness".into(),
        Value::String(report.frame_handedness.clone()),
    );
    map.insert(
        "profile_reader_admitted_entries".into(),
        Value::U64(report.profile_reader_admitted_entries as u64),
    );
    let mut modes = BTreeMap::new();
    for (&mode, &count) in &report.mode_record_counts {
        modes.insert(format!("0x{mode:04x}"), Value::U64(count));
    }
    map.insert("mode_record_counts".into(), Value::Record(modes));
    Value::Record(map)
}

/// Convert a `QacV1InspectionReport` into a deterministic `vibe::Value::Record`.
pub fn qac_inspection_to_vibe_value(report: &QacV1InspectionReport) -> Value {
    let mut map = BTreeMap::new();
    map.insert("format".into(), Value::String("qac_v1".into()));
    map.insert(
        "canonical_sha256".into(),
        Value::String(report.canonical_sha256_hex.clone()),
    );
    map.insert(
        "canonical_blake3".into(),
        Value::String(report.canonical_blake3_hex.clone()),
    );
    map.insert(
        "preservation_sha256".into(),
        Value::String(report.preservation_sha256_hex.clone()),
    );
    map.insert(
        "preservation_blake3".into(),
        Value::String(report.preservation_blake3_hex.clone()),
    );
    map.insert("entry_count".into(), Value::U64(report.entry_count as u64));
    map.insert(
        "sample_rate_hz".into(),
        Value::U64(report.sample_rate_hz as u64),
    );
    map.insert(
        "stft_fft_size".into(),
        Value::U64(report.stft_fft_size as u64),
    );
    map.insert(
        "stft_hop_size".into(),
        Value::U64(report.stft_hop_size as u64),
    );
    map.insert(
        "stft_window_len".into(),
        Value::U64(report.stft_window_len as u64),
    );
    map.insert(
        "signed_zero_preserved".into(),
        Value::Bool(report.signed_zero_preserved),
    );
    map.insert(
        "first_two_pcm_bits".into(),
        Value::List(vec![
            Value::U64(report.first_two_pcm_bits[0] as u64),
            Value::U64(report.first_two_pcm_bits[1] as u64),
        ]),
    );
    map.insert(
        "hoa_channel_count".into(),
        Value::U64(report.hoa_channel_count as u64),
    );
    map.insert(
        "modal_mode_count".into(),
        Value::U64(report.modal_mode_count),
    );
    map.insert(
        "signal_admitted_entries".into(),
        Value::U64(report.signal_admitted_entries as u64),
    );
    let mut modes = BTreeMap::new();
    for (&mode, &count) in &report.mode_record_counts {
        modes.insert(format!("0x{mode:04x}"), Value::U64(count));
    }
    map.insert("mode_record_counts".into(), Value::Record(modes));
    Value::Record(map)
}

fn u64_field(block: qualia_core_db::q42::field_envelope::Block<'_>, key: u16) -> Result<u64, String> {
    let f = semantics::field(block, key).ok_or_else(|| format!("missing key {key}"))?;
    if f.value().len() < 8 {
        return Err(format!("key {key} shorter than 8 bytes"));
    }
    Ok(u64::from_le_bytes(f.value()[0..8].try_into().unwrap()))
}

fn utf8_field<'a>(
    block: qualia_core_db::q42::field_envelope::Block<'a>,
    key: u16,
) -> Result<&'a str, String> {
    let f = semantics::field(block, key).ok_or_else(|| format!("missing key {key}"))?;
    std::str::from_utf8(f.value()).map_err(|e| format!("invalid UTF-8 at key {key}: {e}"))
}

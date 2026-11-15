//! Deterministic Current-Format QAC v1 (`qac\0\0\0\0\0` version 1) Acoustic Cue Producer.
//!
//! Assembles canonical QAC v1 containers from anatomical/botanical vocalization, locomotion
//! footfall/contact, and foliage aeroacoustic sources:
//! - Mode `0x0100` (`pcm_preservation`, preserving exact finite `f32` samples including signed
//!   zero `+0.0` vs `-0.0` with Schema 205 & Schema 210 `SignalEncoding`)
//! - Mode `0x0101` (`stft_preservation`, Schema 201 rev 1 with inline symmetric Hann window key 16
//!   and `origin_basis` key 17 via `qualia_core_db::audio::qac::signal::stft::StftDocument`)
//! - Mode `0x0102` (`hoa_spatial_audio`, 1st-order ACN/SN3D 16B stride with Schema 204 `SpatialHrir`)
//! - Mode `0x0104` (`modal_resonance`, 32B stride Schema 206 damped modes from anatomical/botanical
//!   structural segments)

use crate::emf_qac_anatomy_bridge::emf_v4_producer::sha256_hex;
use crate::emf_qac_anatomy_bridge::rig_metamer::{BotanicalMetamerRig, VertebrateAnatomicalRig};
use crate::Resource;
use qualia_core_db::audio::qac::metadata::Transform;
use qualia_core_db::audio::qac::signal::stft::{
    StftDocument, StftQuantity, StftWriteBuffers, StftWriteOptions,
};
use qualia_core_db::audio::qac::signal::writer::{
    PcmSamples, Provenance, ProvenanceRole, SignalRelation,
};
use qualia_core_db::audio::qac::signal::{Rate, Scalar, Timing};
use qualia_core_db::q42::field_envelope::{
    Block, BlockInput, EntryInput, EnvelopeInput, Family, FieldInput, FieldValue as V,
    Limits as StorageLimits, MetadataInput, StructuralView,
};
use qualia_core_db::q42::field_semantics::{
    self as semantics, field, validate_fields, Context, DomainRegistry, FieldRule, NodeState,
    Scratch,
};
use qualia_core_db::q42::species_corpus::SpeciesCorpus;
use std::collections::BTreeMap;
use std::f64::consts::TAU;

/// QAC v1 Mode `0x0100`: `pcm_preservation` (4B stride for 1-channel f32).
pub const QAC_MODE_PCM_PRESERVATION: u16 = 0x0100;
/// QAC v1 Mode `0x0101`: `stft_preservation` (`channels * fft_size * 8` stride).
pub const QAC_MODE_STFT_PRESERVATION: u16 = 0x0101;
/// QAC v1 Mode `0x0102`: `hoa_spatial_audio` (16B stride for 1st-order ACN/SN3D `f32[4]`).
pub const QAC_MODE_HOA_SPATIAL_AUDIO: u16 = 0x0102;
/// QAC v1 Mode `0x0104`: `modal_resonance` (32B stride damped structural modes).
pub const QAC_MODE_MODAL_RESONANCE: u16 = 0x0104;

pub const HOA_MODEL_URI: &str = "urn:qualia:qac:hoa-spatial-audio:1";
pub const MODAL_MODEL_URI: &str = "urn:qualia:qac:modal-resonance:1";

const fn r(key: u16, type_tag: u8, count: Option<u32>, required: bool) -> FieldRule {
    FieldRule {
        key,
        type_tag,
        count,
        required,
    }
}

const SIGNAL_210_RULES: &[FieldRule] = &[
    r(1, 6, None, true),
    r(2, 5, Some(1), true),
    r(3, 5, Some(1), true),
    r(4, 6, None, true),
    r(5, 6, None, true),
];

const SHAPE_205_RULES: &[FieldRule] = &[
    r(1, 6, None, true),
    r(2, 6, None, true),
    r(14, 2, None, true),
    r(15, 6, None, true),
];

const TRANSFORM_201_RULES: &[FieldRule] = &[
    r(1, 6, None, true),
    r(2, 1, Some(1), true),
    r(3, 1, Some(1), true),
    r(4, 6, None, true),
    r(5, 9, Some(1), false),
    r(6, 6, None, true),
    r(7, 1, Some(1), true),
    r(8, 9, Some(1), false),
    r(9, 6, None, true),
    r(10, 3, Some(1), true),
    r(11, 6, None, true),
    r(12, 6, None, true),
    r(13, 1, Some(1), true),
    r(14, 2, Some(1), true),
    r(15, 2, Some(1), true),
    r(16, 4, None, false),
    r(17, 6, None, false),
];

const SPATIAL_HRIR_204_RULES: &[FieldRule] = &[
    r(1, 6, None, true),
    r(2, 1, Some(1), true),
    r(3, 6, None, true),
    r(4, 6, None, true),
    r(5, 6, None, true),
];

const RAY_MODAL_206_RULES: &[FieldRule] = &[
    r(1, 6, None, true),
    r(2, 6, None, true),
    r(3, 5, Some(1), true),
    r(4, 6, None, true),
    r(5, 1, Some(1), true),
];

/// Domain registry admitting full 4-mode QAC v1 metadata schemas (`201, 204, 205, 206, 210`).
#[derive(Clone, Copy, Debug, Default)]
pub struct QacV1CueRegistry;

impl DomainRegistry for QacV1CueRegistry {
    fn rules(&self, schema: u16, revision: u16) -> Option<&'static [FieldRule]> {
        if revision != 1 {
            return None;
        }
        match schema {
            201 => Some(TRANSFORM_201_RULES),
            204 => Some(SPATIAL_HRIR_204_RULES),
            205 => Some(SHAPE_205_RULES),
            206 => Some(RAY_MODAL_206_RULES),
            210 => Some(SIGNAL_210_RULES),
            _ => None,
        }
    }

    fn descriptor_target_schema(&self, key: u16) -> Option<u16> {
        match key {
            5 => Some(201),
            _ => None,
        }
    }

    fn validate_block(
        &self,
        block: Block<'_>,
        ctx: &Context<'_, '_>,
    ) -> Result<(), semantics::Error> {
        ctx.charge_work(
            (block.fields().count() as u64)
                .checked_mul(32)
                .ok_or(semantics::Error::Overflow)?,
        )?;
        match block.schema_type() {
            201 => {
                if let Some(w) = field(block, 16) {
                    ctx.charge_work(w.value().len() as u64)?;
                }
                Transform::from_block(block).map_err(|_| semantics::Error::Value)?;
                Ok(())
            }
            204 => {
                validate_fields(block, SPATIAL_HRIR_204_RULES)?;
                let rep = utf8_field(block, 1)?;
                let order = u32_field(block, 2)?;
                let ordering = utf8_field(block, 3)?;
                let norm = utf8_field(block, 4)?;
                let delay_ref = utf8_field(block, 5)?;
                if rep != "HOA"
                    || order > 3
                    || ordering != "ACN"
                    || !["SN3D", "N3D"].contains(&norm)
                    || delay_ref != "relative-to-excitation-origin"
                {
                    return Err(semantics::Error::Value);
                }
                Ok(())
            }
            205 => {
                validate_fields(block, SHAPE_205_RULES)?;
                let uri = utf8_field(block, 1)?;
                if ![
                    "urn:qualia:qac:pcm-preservation:1",
                    "urn:qualia:qac:stft-preservation:1",
                    "urn:qualia:qac:opaque-original:1",
                    HOA_MODEL_URI,
                    MODAL_MODEL_URI,
                ]
                .contains(&uri)
                {
                    return Err(semantics::Error::UnsupportedDomain);
                }
                if utf8_field(block, 2)? != "1" {
                    return Err(semantics::Error::Value);
                }
                let shape = field(block, 14).ok_or(semantics::Error::MissingField)?;
                if shape.element_count() == 0 || shape.element_count() > 3 {
                    return Err(semantics::Error::Value);
                }
                for raw in shape.value().chunks_exact(8) {
                    if u64::from_le_bytes(raw.try_into().unwrap()) == 0 {
                        return Err(semantics::Error::Value);
                    }
                }
                let _ = utf8_field(block, 15)?;
                Ok(())
            }
            206 => {
                validate_fields(block, RAY_MODAL_206_RULES)?;
                let kind = utf8_field(block, 1)?;
                let gain = utf8_field(block, 2)?;
                let ref_dist = f64_field(block, 3)?;
                let excitation = utf8_field(block, 4)?;
                let count = u32_field(block, 5)?;
                if kind != "damped_modal_bank_v1"
                    || !["energy-ratio", "pressure-amplitude-ratio"].contains(&gain)
                    || !ref_dist.is_finite()
                    || ref_dist <= 0.0
                    || excitation.is_empty()
                    || count == 0
                {
                    return Err(semantics::Error::Value);
                }
                Ok(())
            }
            210 => {
                validate_fields(block, SIGNAL_210_RULES)?;
                let _ = utf8_field(block, 1)?;
                let scale = f64_field(block, 2)?;
                let offset = f64_field(block, 3)?;
                let _ = utf8_field(block, 4)?;
                let rel = utf8_field(block, 5)?;
                if !scale.is_finite() || scale <= 0.0 || !offset.is_finite() {
                    return Err(semantics::Error::Value);
                }
                match rel {
                    "physical-base-units" | "raw-counts" => {
                        if scale != 1.0 || offset != 0.0 {
                            return Err(semantics::Error::Value);
                        }
                    }
                    "digital-unrelated-to-pressure" => {}
                    _ => return Err(semantics::Error::Value),
                }
                Ok(())
            }
            _ => Err(semantics::Error::UnsupportedDomain),
        }
    }
}

fn u32_field(block: Block<'_>, key: u16) -> Result<u32, semantics::Error> {
    let f = field(block, key).ok_or(semantics::Error::MissingField)?;
    Ok(u32::from_le_bytes(f.value()[0..4].try_into().unwrap()))
}

fn f64_field(block: Block<'_>, key: u16) -> Result<f64, semantics::Error> {
    let f = field(block, key).ok_or(semantics::Error::MissingField)?;
    Ok(f64::from_bits(u64::from_le_bytes(
        f.value()[0..8].try_into().unwrap(),
    )))
}

fn utf8_field<'a>(block: Block<'a>, key: u16) -> Result<&'a str, semantics::Error> {
    let f = field(block, key).ok_or(semantics::Error::MissingField)?;
    std::str::from_utf8(f.value()).map_err(|_| semantics::Error::Value)
}

/// Output bundle from deterministic QAC v1 production.
#[derive(Clone, Debug, PartialEq)]
pub struct QacV1ProducedBundle {
    /// Canonical 4-mode `qac\0\0\0\0\0` v1 container (`0x0100, 0x0101, 0x0102, 0x0104`).
    pub canonical_resource: Resource,
    pub canonical_sha256_hex: String,
    pub canonical_blake3_hex: String,
    /// Companion `audio::qac::signal::admit`-certified preservation container (`0x0100 + 0x0101`).
    pub preservation_resource: Resource,
    pub preservation_sha256_hex: String,
    pub preservation_blake3_hex: String,
    /// Per-mode record counts in the canonical 4-mode container.
    pub mode_record_counts: BTreeMap<u16, u64>,
    /// Sample rate in Hz.
    pub sample_rate_hz: u32,
    /// STFT FFT size in samples.
    pub stft_fft_size: u32,
    /// STFT hop size in samples.
    pub stft_hop_size: u32,
    /// First two PCM sample raw IEEE-754 bits (`[0x0000_0000 (+0.0), 0x8000_0000 (-0.0)]`).
    pub signed_zero_anchor_bits: [u32; 2],
}

/// Encode one 16-byte Mode `0x0102` (`hoa_spatial_audio`, 1st-order ACN/SN3D `[W, Y, Z, X]`) frame.
pub fn encode_hoa_order1_sn3d_frame(pressure_pa: f32, azimuth_rad: f64, elevation_rad: f64) -> [u8; 16] {
    let cos_el = elevation_rad.cos();
    let w = pressure_pa;
    let y = (pressure_pa as f64 * azimuth_rad.sin() * cos_el) as f32;
    let z = (pressure_pa as f64 * elevation_rad.sin()) as f32;
    let x = (pressure_pa as f64 * azimuth_rad.cos() * cos_el) as f32;
    let mut rec = [0u8; 16];
    for (i, ch) in [w, y, z, x].into_iter().enumerate() {
        rec[i * 4..(i + 1) * 4].copy_from_slice(&ch.to_bits().to_le_bytes());
    }
    rec
}

/// Encode one 32-byte Mode `0x0104` (`modal_resonance`) record per wire-freeze §5.5:
/// `frequency_hz:f32@0`, `amplitude_pa:f32@4`, `decay_rate_s1:f32@8`, `phase_rad:f32@12`,
/// `mode_shape_index:u32@16`, `material_ref:u32@20`, `excitation_index:u32@24`, `reserved:u32@28`.
pub fn encode_modal_resonance_record(
    frequency_hz: f32,
    amplitude_pa: f32,
    decay_rate_s1: f32,
    phase_rad: f32,
    mode_shape_index: u32,
    material_ref: u32,
    excitation_index: u32,
) -> [u8; 32] {
    let mut rec = [0u8; 32];
    rec[0..4].copy_from_slice(&frequency_hz.to_bits().to_le_bytes());
    rec[4..8].copy_from_slice(&amplitude_pa.to_bits().to_le_bytes());
    rec[8..12].copy_from_slice(&decay_rate_s1.to_bits().to_le_bytes());
    rec[12..16].copy_from_slice(&phase_rad.to_bits().to_le_bytes());
    rec[16..20].copy_from_slice(&mode_shape_index.to_le_bytes());
    rec[20..24].copy_from_slice(&material_ref.to_le_bytes());
    rec[24..28].copy_from_slice(&excitation_index.to_le_bytes());
    // 28..32 reserved zero
    rec
}

/// Produce deterministic QAC v1 containers (`0x0100, 0x0101, 0x0102, 0x0104`) from an
/// articulated vertebrate anatomical rig (*Wallabia bicolor* vocalization + footfall + modes).
pub fn produce_vertebrate_qac_v1(
    rig: &VertebrateAnatomicalRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<QacV1ProducedBundle, String> {
    rig.validate(corpus)?;

    let cue = &rig.acoustic_cue;
    let n_samples = cue.frame_count;
    let sr = cue.sample_rate_hz as f64;
    let mut pcm_samples = Vec::with_capacity(n_samples);
    // Explicitly anchor +0.0f32 at sample 0 and -0.0f32 at sample 1 for signed-zero preservation
    pcm_samples.push(0.0f32);
    pcm_samples.push(-0.0f32);
    for i in 2..n_samples {
        let t = (i as f64) / sr;
        let env_vocal = (-6.5 * t).exp();
        let f0 = (TAU * cue.vocalization_fundamental_hz * t).sin();
        let f1 = 0.55 * (TAU * cue.vocalization_formants_hz[0] * t).sin();
        let f2 = 0.28 * (TAU * cue.vocalization_formants_hz[1] * t).sin();
        let f3 = 0.14 * (TAU * cue.vocalization_formants_hz[2] * t).sin();
        let footfall_t = (t - 0.0015).max(0.0);
        let footfall = if t >= 0.0015 {
            cue.footfall_impulse_pa
                * (-45.0 * footfall_t).exp()
                * (TAU * rig.segments[4].structural_modal_freq_hz * footfall_t).sin()
        } else {
            0.0
        };
        let val = (0.25 * env_vocal * (f0 + f1 + f2 + f3) + footfall) as f32;
        pcm_samples.push(val);
    }

    let mut hoa_payload = Vec::with_capacity(n_samples * 16);
    for &p in &pcm_samples {
        hoa_payload.extend_from_slice(&encode_hoa_order1_sn3d_frame(
            p,
            cue.hoa_azimuth_rad,
            cue.hoa_elevation_rad,
        ));
    }

    let mut modal_payload = Vec::with_capacity(rig.segments.len() * 32);
    for (idx, seg) in rig.segments.iter().enumerate() {
        let amp = (0.08 + 0.32 * seg.muscular_activation_01) as f32;
        let phase = (0.15 * (idx as f64)) as f32;
        modal_payload.extend_from_slice(&encode_modal_resonance_record(
            seg.structural_modal_freq_hz as f32,
            amp,
            seg.structural_damping_s1 as f32,
            phase,
            idx as u32,
            0,
            1,
        ));
    }

    assemble_qac_v1_bundles(
        &pcm_samples,
        cue.sample_rate_hz,
        rig.epoch_seconds,
        &hoa_payload,
        &modal_payload,
        rig.segments.len() as u64,
        "vertebrate_vocalization_and_footfall",
        &rig.rig_id,
        key_prefix,
    )
}

/// Produce deterministic QAC v1 containers (`0x0100, 0x0101, 0x0102, 0x0104`) from a
/// botanical FSPM metamer rig (*Corymbia aparrerinja* aeroacoustic foliage rustle + modes).
pub fn produce_botanical_qac_v1(
    rig: &BotanicalMetamerRig,
    corpus: &SpeciesCorpus,
    key_prefix: &str,
) -> Result<QacV1ProducedBundle, String> {
    rig.validate(corpus)?;

    let n_samples = rig.frame_count;
    let sr = rig.sample_rate_hz as f64;
    let wind_speed = rig.wind_velocity_m_s[0]
        .hypot(rig.wind_velocity_m_s[1])
        .hypot(rig.wind_velocity_m_s[2])
        .max(0.1);
    let az = rig.wind_velocity_m_s[1].atan2(rig.wind_velocity_m_s[0]);
    let el = (rig.wind_velocity_m_s[2] / wind_speed).clamp(-1.0, 1.0).asin();

    let mut pcm_samples = Vec::with_capacity(n_samples);
    // Explicitly anchor +0.0f32 at sample 0 and -0.0f32 at sample 1 for signed-zero preservation
    pcm_samples.push(0.0f32);
    pcm_samples.push(-0.0f32);
    for i in 2..n_samples {
        let t = (i as f64) / sr;
        let mut sum_pa = 0.0f64;
        for (idx, m) in rig.metamers.iter().enumerate() {
            let phase = 0.25 * (idx as f64);
            sum_pa += m.rustle_amplitude_pa
                * (-m.rustle_damping_s1 * t).exp()
                * (TAU * m.rustle_modal_freq_hz * t + phase).sin();
        }
        pcm_samples.push(sum_pa as f32);
    }

    let mut hoa_payload = Vec::with_capacity(n_samples * 16);
    for &p in &pcm_samples {
        hoa_payload.extend_from_slice(&encode_hoa_order1_sn3d_frame(p, az, el));
    }

    let mut modal_payload = Vec::with_capacity(rig.metamers.len() * 32);
    for (idx, m) in rig.metamers.iter().enumerate() {
        let phase = (0.25 * (idx as f64)) as f32;
        modal_payload.extend_from_slice(&encode_modal_resonance_record(
            m.rustle_modal_freq_hz as f32,
            m.rustle_amplitude_pa as f32,
            m.rustle_damping_s1 as f32,
            phase,
            idx as u32,
            0,
            1,
        ));
    }

    assemble_qac_v1_bundles(
        &pcm_samples,
        rig.sample_rate_hz,
        rig.epoch_seconds,
        &hoa_payload,
        &modal_payload,
        rig.metamers.len() as u64,
        "botanical_foliage_aeroacoustic_rustle",
        &rig.plant_id,
        key_prefix,
    )
}

#[allow(clippy::too_many_arguments)]
fn assemble_qac_v1_bundles(
    pcm_samples: &[f32],
    sample_rate_hz: u32,
    epoch_seconds: i64,
    hoa_payload: &[u8],
    modal_payload: &[u8],
    modal_count: u64,
    excitation_label: &str,
    lineage_id: &str,
    key_prefix: &str,
) -> Result<QacV1ProducedBundle, String> {
    let fft_size = 64u32;
    let hop_size = 32u32;
    let timing = Timing {
        scale: 1,
        epoch_seconds,
        epoch_nanoseconds: 0,
        ticks: Rate {
            numerator: u64::from(sample_rate_hz),
            denominator: 1,
        },
        samples: Rate {
            numerator: u64::from(sample_rate_hz),
            denominator: 1,
        },
        start_tick: 0,
    };

    let doc = StftDocument {
        samples: PcmSamples::F32(pcm_samples),
        timing,
        stream_id: 1,
        channels: 1,
        channel_order: "mono",
        fft_size,
        hop: hop_size,
        pcm_purpose: excitation_label,
        stft_purpose: "hann_stft_complex_preservation",
        quantity: StftQuantity::Pressure,
        relation: SignalRelation::PhysicalBaseUnits,
        pcm_provenance: Provenance {
            role: ProvenanceRole::Model,
            algorithm: Some("synth.emf_qac_anatomy_bridge.pcm"),
            algorithm_version: Some("1.0.0"),
            instrument_uri: None,
            lineage: Some(lineage_id),
        },
        stft_provenance: Provenance {
            role: ProvenanceRole::Derived,
            algorithm: Some("etalon_signal.stft_hann_symmetric"),
            algorithm_version: Some("1.0.0"),
            instrument_uri: None,
            lineage: Some(lineage_id),
        },
        original: None,
    };

    let mut mono_buf = vec![0.0f32; pcm_samples.len()];
    let mut fft_buf = vec![0.0f32; (fft_size as usize) * 2];
    let mut win_buf = vec![0.0f32; fft_size as usize];
    let stft_frames = (pcm_samples.len() - (fft_size as usize)) / (hop_size as usize) + 1;
    let mut spectra_buf = vec![0u8; stft_frames * (fft_size as usize) * 8];
    let mut meta_buf = vec![0u8; 16_384];
    let mut payload_buf = vec![0u8; pcm_samples.len() * 4];
    let mut staging_file = vec![0u8; 131_072];
    let mut preservation_out = vec![0u8; 131_072];
    let mut meta_nodes = [NodeState::default(); 32];
    let mut entry_nodes = [NodeState::default(); 8];
    let mut scratch = Scratch {
        metadata: &mut meta_nodes,
        entries: &mut entry_nodes,
    };

    let written_len = doc
        .encode_into(
            StftWriteBuffers {
                mono: &mut mono_buf,
                fft: &mut fft_buf,
                window: &mut win_buf,
                spectra: &mut spectra_buf,
                metadata: &mut meta_buf,
                payload: &mut payload_buf,
                file: &mut staging_file,
            },
            &mut scratch,
            &mut preservation_out,
            StftWriteOptions::default(),
        )
        .map_err(|e| format!("StftDocument encode failed: {e:?}"))?;
    preservation_out.truncate(written_len);

    // Extract verified PCM and STFT chunks from the admitted preservation container
    let storage_limits = StorageLimits::default();
    let pres_view = StructuralView::open(&preservation_out, Family::QacV1Candidate, storage_limits)
        .map_err(|e| format!("Preservation view open failed: {e:?}"))?;
    let pcm_bytes = pres_view
        .verify_chunk(1)
        .map_err(|e| format!("PCM chunk verify failed: {e:?}"))?;
    let stft_bytes = pres_view
        .verify_chunk(2)
        .map_err(|e| format!("STFT chunk verify failed: {e:?}"))?;
    let pcm_digest = *blake3::hash(pcm_bytes).as_bytes();

    let canonical_bytes = assemble_canonical_4mode_qac(
        epoch_seconds,
        u64::from(sample_rate_hz),
        pcm_samples.len() as u64,
        fft_size,
        hop_size,
        stft_frames as u64,
        &win_buf,
        pcm_bytes,
        &pcm_digest,
        stft_bytes,
        hoa_payload,
        modal_payload,
        modal_count,
        excitation_label,
        lineage_id,
    )?;

    let mut mode_record_counts = BTreeMap::new();
    mode_record_counts.insert(QAC_MODE_PCM_PRESERVATION, pcm_samples.len() as u64);
    mode_record_counts.insert(QAC_MODE_STFT_PRESERVATION, stft_frames as u64);
    mode_record_counts.insert(QAC_MODE_HOA_SPATIAL_AUDIO, pcm_samples.len() as u64);
    mode_record_counts.insert(QAC_MODE_MODAL_RESONANCE, modal_count);

    Ok(QacV1ProducedBundle {
        canonical_sha256_hex: sha256_hex(&canonical_bytes),
        canonical_blake3_hex: blake3::hash(&canonical_bytes).to_hex().to_string(),
        canonical_resource: Resource {
            key: format!("{key_prefix}_canonical_v1.qac"),
            kind: "qac".to_string(),
            bytes: canonical_bytes,
        },
        preservation_sha256_hex: sha256_hex(&preservation_out),
        preservation_blake3_hex: blake3::hash(&preservation_out).to_hex().to_string(),
        preservation_resource: Resource {
            key: format!("{key_prefix}_preservation_v1.qac"),
            kind: "qac".to_string(),
            bytes: preservation_out,
        },
        mode_record_counts,
        sample_rate_hz,
        stft_fft_size: fft_size,
        stft_hop_size: hop_size,
        signed_zero_anchor_bits: [pcm_samples[0].to_bits(), pcm_samples[1].to_bits()],
    })
}

use crate::emf_qac_anatomy_bridge::qac_v1_envelope::assemble_canonical_4mode_qac;


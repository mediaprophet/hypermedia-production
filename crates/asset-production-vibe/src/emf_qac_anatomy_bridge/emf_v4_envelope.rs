//! Canonical 128B header + 128B directory + 8B TLV metadata + 64B payload envelope
//! builder for EMF v4 (`Q42EMF64` version 4).

use crate::emf_qac_anatomy_bridge::emf_v4_producer::{
    sampled_spectrum_stride, BIOELECTRIC_MODEL_URI, BIOELECTRIC_PROFILE, CANDIDATE_MODE_E_B,
    CANDIDATE_MODE_RAY, CANDIDATE_MODE_SPECTRUM, MODE_BIOELECTRIC_DIPOLE_V1,
    MODE_INSTANTANEOUS_E_B_V1, MODE_POLARIMETRIC_RAY_V1, MODE_SAMPLED_SPECTRUM_V1,
};
use qualia_core_db::q42::emf_profiles::{
    FIELD_MODEL_URI, FIELD_PROFILE, RAY_MODEL_URI, RAY_PROFILE, SPECTRUM_MODEL_URI,
    SPECTRUM_PROFILE,
};
use qualia_core_db::q42::field_envelope::{
    BlockInput, EntryInput, EnvelopeInput, Family, FieldInput, FieldValue as V,
    Limits as StorageLimits, MetadataInput,
};
use sha2::{Digest, Sha256};

fn f(key: u16, value: V<'_>) -> FieldInput<'_> {
    FieldInput {
        key,
        flags: 1,
        value,
    }
}

fn blk<'a>(id: u32, schema_type: u16, fields: &'a [FieldInput<'a>]) -> BlockInput<'a> {
    BlockInput {
        id,
        schema_type,
        revision: 1,
        flags: 1,
        fields,
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn assemble_emf_v4_container(
    epoch_seconds: i64,
    wavelengths_m: &[f64],
    eb_payload: &[u8],
    eb_count: u64,
    ray_payload: &[u8],
    ray_count: u64,
    spec_payload: &[u8],
    spec_count: u64,
    bio_payload: Option<(&[u8], u64)>,
    authority_digest: &[u8; 32],
    anatomy_ontology: &str,
    anatomy_iri: &str,
    observation_type: &str,
    species_id: &str,
    rig_id: &str,
    use_wire_freeze_modes: bool,
) -> Result<Vec<u8>, String> {
    let clock_scale = [1u32]; // TAI
    let clock_epoch_s = [epoch_seconds];
    let clock_epoch_ns = [0u32];
    let clock_rate_num = [1_000u64];
    let clock_rate_den = [1u64];
    let clock_fields = [
        f(1, V::U32(&clock_scale)),
        f(2, V::I64(&clock_epoch_s)),
        f(3, V::U32(&clock_epoch_ns)),
        f(4, V::U64(&clock_rate_num)),
        f(5, V::U64(&clock_rate_den)),
    ];

    let frame_kind = [0u32];
    let frame_origin = [0.0f64, 0.0, 0.0];
    let frame_rot = [0.0f64, 0.0, 0.0, 1.0];
    let frame_fields = [
        f(1, V::U32(&frame_kind)),
        f(3, V::F64(&frame_origin)),
        f(4, V::F64(&frame_rot)),
        f(8, V::Utf8("right-handed-X-forward-Y-left-Z-up")),
    ];

    let prov_role = [1u32];
    let prov_fields = [
        f(1, V::U32(&prov_role)),
        f(3, V::Utf8("synth.emf_qac_anatomy_bridge")),
        f(4, V::Utf8("1.0.0")),
        f(7, V::Utf8(rig_id)),
    ];

    let consent_prov_fields = [
        f(1, V::U32(&prov_role)),
        f(3, V::Utf8("qualia.species_corpus.authority")),
        f(4, V::Utf8("1.0.0")),
        FieldInput {
            key: 6,
            flags: 0,
            value: V::Bytes(authority_digest),
        },
        f(7, V::Utf8(species_id)),
    ];

    let cal_scale = [1.0f64];
    let cal_offset = [0.0f64];
    let cal_uncertainty = [1.5e-11f64];
    let cal_fields = [
        f(1, V::Utf8("urn:qualia:calibration:bioelectric_reference_v1")),
        FieldInput {
            key: 2,
            flags: 1,
            value: V::Bytes(authority_digest),
        },
        f(3, V::Utf8("NIST-traceable-differential-biopotential")),
        f(4, V::F64(&cal_scale)),
        f(5, V::F64(&cal_offset)),
        f(6, V::F64(&cal_uncertainty)),
        f(7, V::Utf8("k=2 expanded combined uncertainty")),
    ];

    let spec_fields = [
        f(1, V::Utf8("vacuum_wavelength_m")),
        f(2, V::Utf8("point_density")),
        f(3, V::Utf8("dimensionless_reflectance")),
        f(4, V::Utf8("1")),
        f(5, V::F64(wavelengths_m)),
        f(6, V::Utf8("explicit")),
        f(8, V::Utf8("none")),
    ];

    let pol_fields = [
        f(1, V::Utf8("deterministic_transverse_v1")),
        f(2, V::Utf8("I_Q_U_V_v1")),
        f(3, V::Utf8("W sr^-1 m^-2")),
        f(4, V::Utf8("time_averaged")),
    ];

    let five_u32 = [5u32];
    let eight_u32 = [8u32];
    let model_fields = [
        f(1, V::Utf8("quasi-static-volume-conductor-dipole-v1")),
        f(2, V::Utf8("species-corpus-anatomical-solver")),
        FieldInput {
            key: 3,
            flags: 1,
            value: V::Bytes(authority_digest),
        },
        f(4, V::Utf8(rig_id)),
        f(5, V::Utf8("first-order-dipole-perturbation-bound")),
        f(8, V::Utf8("model_estimate")),
    ];

    let four_u32 = [4u32];
    let bio_ctx_fields = [
        f(1, V::Utf8(anatomy_ontology)),
        f(2, V::Utf8(anatomy_iri)),
        f(3, V::Utf8(observation_type)),
        f(4, V::MetadataRefs(&four_u32)),
        f(5, V::MetadataRefs(&five_u32)),
        f(6, V::MetadataRefs(&eight_u32)),
        f(7, V::MetadataRefs(&four_u32)),
    ];

    let mode_eb = [if use_wire_freeze_modes {
        u32::from(MODE_INSTANTANEOUS_E_B_V1)
    } else {
        u32::from(CANDIDATE_MODE_E_B)
    }];
    let mode_ray = [if use_wire_freeze_modes {
        u32::from(MODE_POLARIMETRIC_RAY_V1)
    } else {
        u32::from(CANDIDATE_MODE_RAY)
    }];
    let mode_spec = [if use_wire_freeze_modes {
        u32::from(MODE_SAMPLED_SPECTRUM_V1)
    } else {
        u32::from(CANDIDATE_MODE_SPECTRUM)
    }];
    let mode_bio = [u32::from(MODE_BIOELECTRIC_DIPOLE_V1)];

    let one_u32 = [1u32];
    let two_u32 = [2u32];
    let three_u32 = [3u32];
    let six_u32 = [6u32];
    let seven_u32 = [7u32];
    let nine_u32 = [9u32];
    let zero_u32 = [0u32];
    let dims_1 = [1u64, 1, 1];
    let bricks_1 = [1u32, 1, 1];
    let step_1 = [1i64];
    let bins_u32 = [wavelengths_m.len() as u32];

    let ext_eb_fields = [
        f(1, V::U32(&mode_eb)),
        f(2, V::U32(&one_u32)),
        f(3, V::MetadataRefs(&one_u32)),
        f(4, V::MetadataRefs(&two_u32)),
        f(5, V::MetadataRefs(&three_u32)),
        f(6, V::Utf8(FIELD_PROFILE)),
        f(7, V::Utf8("linear")),
        f(8, V::U64(&dims_1)),
        f(9, V::U32(&bricks_1)),
        f(10, V::U32(&zero_u32)),
        f(11, V::I64(&step_1)),
    ];

    let ext_ray_fields = [
        f(1, V::U32(&mode_ray)),
        f(2, V::U32(&one_u32)),
        f(3, V::MetadataRefs(&one_u32)),
        f(4, V::MetadataRefs(&two_u32)),
        f(5, V::MetadataRefs(&three_u32)),
        f(6, V::Utf8(RAY_PROFILE)),
        f(7, V::Utf8("linear")),
        f(8, V::U64(&dims_1)),
        f(9, V::U32(&bricks_1)),
        f(10, V::U32(&zero_u32)),
        f(11, V::I64(&step_1)),
        f(16, V::MetadataRefs(&seven_u32)),
    ];

    let ext_spec_fields = [
        f(1, V::U32(&mode_spec)),
        f(2, V::U32(&one_u32)),
        f(3, V::MetadataRefs(&one_u32)),
        f(4, V::MetadataRefs(&two_u32)),
        f(5, V::MetadataRefs(&three_u32)),
        f(6, V::Utf8(SPECTRUM_PROFILE)),
        f(7, V::Utf8("linear")),
        f(8, V::U64(&dims_1)),
        f(9, V::U32(&bricks_1)),
        f(10, V::U32(&zero_u32)),
        f(11, V::I64(&step_1)),
        f(12, V::MetadataRefs(&six_u32)),
        f(19, V::U32(&bins_u32)),
    ];

    let ext_bio_fields = [
        f(1, V::U32(&mode_bio)),
        f(2, V::U32(&one_u32)),
        f(3, V::MetadataRefs(&one_u32)),
        f(4, V::MetadataRefs(&two_u32)),
        f(5, V::MetadataRefs(&four_u32)),
        f(6, V::Utf8(BIOELECTRIC_PROFILE)),
        f(7, V::Utf8("linear")),
        f(8, V::U64(&dims_1)),
        f(9, V::U32(&bricks_1)),
        f(10, V::U32(&zero_u32)),
        f(11, V::I64(&step_1)),
        f(17, V::MetadataRefs(&eight_u32)),
        f(22, V::MetadataRefs(&nine_u32)),
        f(30, V::MetadataRefs(&five_u32)),
    ];

    let ref_10 = [10u32];
    let ref_11 = [11u32];
    let ref_12 = [12u32];
    let ref_13 = [13u32];

    let desc_eb_fields = [
        f(1, V::MetadataRefs(&one_u32)),
        f(2, V::MetadataRefs(&two_u32)),
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("electromagnetic_field_snapshot")),
        f(8, V::Utf8("mixed:mode-registry")),
        f(12, V::Utf8(FIELD_MODEL_URI)),
        f(13, V::U64(&clock_rate_num)),
        f(14, V::U64(&clock_rate_den)),
        f(18, V::MetadataRefs(&ref_10)),
    ];

    let desc_ray_fields = [
        f(1, V::MetadataRefs(&one_u32)),
        f(2, V::MetadataRefs(&two_u32)),
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("polarimetric_ray")),
        f(8, V::Utf8("W sr^-1 m^-2")),
        f(12, V::Utf8(RAY_MODEL_URI)),
        f(13, V::U64(&clock_rate_num)),
        f(14, V::U64(&clock_rate_den)),
        f(18, V::MetadataRefs(&ref_11)),
    ];

    let desc_spec_fields = [
        f(1, V::MetadataRefs(&one_u32)),
        f(2, V::MetadataRefs(&two_u32)),
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("dimensionless_reflectance")),
        f(8, V::Utf8("1")),
        f(12, V::Utf8(SPECTRUM_MODEL_URI)),
        f(13, V::U64(&clock_rate_num)),
        f(14, V::U64(&clock_rate_den)),
        f(18, V::MetadataRefs(&ref_12)),
    ];

    let desc_bio_fields = [
        f(1, V::MetadataRefs(&one_u32)),
        f(2, V::MetadataRefs(&two_u32)),
        f(3, V::MetadataRefs(&four_u32)),
        f(6, V::MetadataRefs(&five_u32)),
        f(7, V::Utf8("bioelectric_current_dipole")),
        f(8, V::Utf8("A m")),
        f(12, V::Utf8(BIOELECTRIC_MODEL_URI)),
        f(13, V::U64(&clock_rate_num)),
        f(14, V::U64(&clock_rate_den)),
        f(18, V::MetadataRefs(&ref_13)),
    ];

    let mut blocks = Vec::with_capacity(17);
    blocks.push(blk(1, 1, &clock_fields));
    blocks.push(blk(2, 2, &frame_fields));
    blocks.push(blk(3, 4, &prov_fields));
    if use_wire_freeze_modes {
        blocks.push(blk(4, 4, &consent_prov_fields));
        blocks.push(blk(5, 107, &cal_fields));
    }
    blocks.push(blk(6, 132, &spec_fields));
    blocks.push(blk(7, 135, &pol_fields));
    if use_wire_freeze_modes {
        blocks.push(blk(8, 134, &model_fields));
        blocks.push(blk(9, 137, &bio_ctx_fields));
    }
    blocks.push(blk(10, 103, &ext_eb_fields));
    blocks.push(blk(11, 103, &ext_ray_fields));
    blocks.push(blk(12, 103, &ext_spec_fields));
    if use_wire_freeze_modes {
        blocks.push(blk(13, 103, &ext_bio_fields));
    }
    blocks.push(blk(14, 3, &desc_eb_fields));
    blocks.push(blk(15, 3, &desc_ray_fields));
    blocks.push(blk(16, 3, &desc_spec_fields));
    if use_wire_freeze_modes {
        blocks.push(blk(17, 3, &desc_bio_fields));
    }

    let storage_limits = StorageLimits::default();
    let meta_input = MetadataInput { blocks: &blocks };
    let meta_len = meta_input
        .encoded_len(storage_limits)
        .map_err(|e| format!("EMF v4 metadata length error: {e:?}"))?;
    let mut meta_buf = vec![0u8; meta_len];
    meta_input
        .encode_into(&mut meta_buf, storage_limits)
        .map_err(|e| format!("EMF v4 metadata encode error: {e:?}"))?;

    let spec_stride = sampled_spectrum_stride(wavelengths_m.len()) as u32;
    let mk_entry = |mode, stream_id, count, stride, descriptor_id, payload| EntryInput {
        mode,
        revision: 1,
        flags: 2,
        stream_id,
        frame_id: 2,
        start_tick: 0,
        time_count: count,
        spatial_key: 0,
        record_count: count,
        stride,
        codec: 0,
        descriptor_id,
        payload,
    };
    let mut entries = Vec::with_capacity(4);
    entries.push(mk_entry(mode_eb[0] as u16, 1, eb_count, 128, 14, eb_payload));
    entries.push(mk_entry(mode_ray[0] as u16, 2, ray_count, 128, 15, ray_payload));
    entries.push(mk_entry(mode_spec[0] as u16, 3, spec_count, spec_stride, 16, spec_payload));
    if let Some((bio_bytes, bio_count)) = bio_payload {
        entries.push(mk_entry(MODE_BIOELECTRIC_DIPOLE_V1, 4, bio_count, 128, 17, bio_bytes));
    }

    let env = EnvelopeInput {
        family: Family::EmfV4Candidate,
        flags: 0,
        metadata: &meta_buf,
        entries: &entries,
    };
    let total_len = env
        .encoded_len(storage_limits)
        .map_err(|e| format!("EMF v4 envelope length error: {e:?}"))?;
    let mut output = vec![0u8; total_len];
    env.encode_into(&mut output, storage_limits)
        .map_err(|e| format!("EMF v4 envelope encode error: {e:?}"))?;
    Ok(output)
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut s = String::with_capacity(64);
    for b in digest {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

//! Canonical 128B header + 128B directory + 8B TLV metadata + 64B payload envelope
//! builder for 4-mode QAC v1 (`qac\0\0\0\0\0` version 1).

use crate::emf_qac_anatomy_bridge::qac_v1_producer::{
    HOA_MODEL_URI, MODAL_MODEL_URI, QAC_MODE_HOA_SPATIAL_AUDIO, QAC_MODE_MODAL_RESONANCE,
    QAC_MODE_PCM_PRESERVATION, QAC_MODE_STFT_PRESERVATION,
};
use qualia_core_db::audio::qac::signal::Scalar;
use qualia_core_db::q42::field_envelope::{
    BlockInput, EntryInput, EnvelopeInput, Family, FieldInput, FieldValue as V,
    Limits as StorageLimits, MetadataInput,
};

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
pub(crate) fn assemble_canonical_4mode_qac(
    epoch_seconds: i64,
    sample_rate_hz: u64,
    sample_frames: u64,
    fft_size: u32,
    hop_size: u32,
    stft_frames: u64,
    window: &[f32],
    pcm_bytes: &[u8],
    pcm_digest: &[u8; 32],
    stft_bytes: &[u8],
    hoa_bytes: &[u8],
    modal_bytes: &[u8],
    modal_count: u64,
    excitation_label: &str,
    lineage_id: &str,
) -> Result<Vec<u8>, String> {
    let scale = [1u32];
    let epoch = [epoch_seconds];
    let ns = [0u32];
    let rate_n = [sample_rate_hz];
    let rate_d = [1u64];
    let clock_fields = [
        f(1, V::U32(&scale)),
        f(2, V::I64(&epoch)),
        f(3, V::U32(&ns)),
        f(4, V::U64(&rate_n)),
        f(5, V::U64(&rate_d)),
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

    let role_model = [1u32];
    let role_derived = [2u32];
    let pcm_prov = [
        f(1, V::U32(&role_model)),
        f(3, V::Utf8("synth.emf_qac_anatomy_bridge.pcm")),
        f(4, V::Utf8("1.0.0")),
        f(7, V::Utf8(lineage_id)),
    ];
    let stft_prov = [
        f(1, V::U32(&role_derived)),
        f(2, V::Bytes(pcm_digest)),
        f(3, V::Utf8("etalon_signal.stft_hann_symmetric")),
        f(4, V::Utf8("1.0.0")),
        f(7, V::Utf8(lineage_id)),
    ];

    let pcm_dims = [sample_frames, 1];
    let pcm_shape = [
        f(1, V::Utf8("urn:qualia:qac:pcm-preservation:1")),
        f(2, V::Utf8("1")),
        f(14, V::U64(&pcm_dims)),
        f(15, V::Utf8("sample")),
    ];

    let stft_dims = [stft_frames, 1, u64::from(fft_size)];
    let stft_shape = [
        f(1, V::Utf8("urn:qualia:qac:stft-preservation:1")),
        f(2, V::Utf8("1")),
        f(14, V::U64(&stft_dims)),
        f(15, V::Utf8("real,imag")),
    ];

    let hoa_dims = [sample_frames, 4];
    let hoa_shape = [
        f(1, V::Utf8(HOA_MODEL_URI)),
        f(2, V::Utf8("1")),
        f(14, V::U64(&hoa_dims)),
        f(15, V::Utf8("time,acn_channel")),
    ];

    let modal_dims = [modal_count, 1];
    let modal_shape = [
        f(1, V::Utf8(MODAL_MODEL_URI)),
        f(2, V::Utf8("1")),
        f(14, V::U64(&modal_dims)),
        f(15, V::Utf8("mode,component")),
    ];

    let gain_1 = [1.0f64];
    let offset_0 = [0.0f64];
    let pcm_enc = [
        f(1, V::Utf8("mono")),
        f(2, V::F64(&gain_1)),
        f(3, V::F64(&offset_0)),
        f(4, V::Utf8(excitation_label)),
        f(5, V::Utf8("physical-base-units")),
    ];
    let stft_enc = [
        f(1, V::Utf8("mono")),
        f(2, V::F64(&gain_1)),
        f(3, V::F64(&offset_0)),
        f(4, V::Utf8("hann_stft_complex_preservation")),
        f(5, V::Utf8("physical-base-units")),
    ];
    let hoa_enc = [
        f(1, V::Utf8("ACN0,ACN1,ACN2,ACN3")),
        f(2, V::F64(&gain_1)),
        f(3, V::F64(&offset_0)),
        f(4, V::Utf8("hoa_order1_sn3d_spatial_field")),
        f(5, V::Utf8("physical-base-units")),
    ];

    let fft_u32 = [fft_size];
    let hop_u32 = [hop_size];
    let zero_u32 = [0u32];
    let zero_i64 = [0i64];
    let transform_201 = [
        f(1, V::Utf8("STFT-complex")),
        f(2, V::U32(&fft_u32)),
        f(3, V::U32(&hop_u32)),
        f(4, V::Utf8("hann-symmetric-authored-f32")),
        f(6, V::Utf8("forward-unscaled-inverse-1/N")),
        f(7, V::U32(&zero_u32)),
        f(9, V::Utf8("forward-negative-exponent")),
        f(10, V::I64(&zero_i64)),
        f(11, V::Utf8("none")),
        f(12, V::Utf8("mono")),
        f(13, V::U32(&fft_u32)),
        f(14, V::U64(&rate_n)),
        f(15, V::U64(&rate_d)),
        f(16, V::F32(window)),
        f(17, V::Utf8("source-entry-first-sample")),
    ];

    let hoa_order = [1u32];
    let spatial_204 = [
        f(1, V::Utf8("HOA")),
        f(2, V::U32(&hoa_order)),
        f(3, V::Utf8("ACN")),
        f(4, V::Utf8("SN3D")),
        f(5, V::Utf8("relative-to-excitation-origin")),
    ];

    let ref_dist = [1.0f64];
    let modal_count_u32 = [modal_count as u32];
    let modal_206 = [
        f(1, V::Utf8("damped_modal_bank_v1")),
        f(2, V::Utf8("pressure-amplitude-ratio")),
        f(3, V::F64(&ref_dist)),
        f(4, V::Utf8(excitation_label)),
        f(5, V::U32(&modal_count_u32)),
    ];

    let one_u32 = [1u32];
    let two_u32 = [2u32];
    let three_u32 = [3u32];
    let four_u32 = [4u32];
    let eleven_u32 = [11u32];
    let f32_scalar = [Scalar::F32 as u32];
    let cf32_scalar = [Scalar::ComplexF32 as u32];
    let pcm_ext = [5u32, 9];
    let stft_ext = [6u32, 10];
    let hoa_ext = [7u32, 12, 13];
    let modal_ext = [8u32, 14];

    let gcd_val = gcd(sample_rate_hz, u64::from(hop_size));
    let stft_rate_n = [sample_rate_hz / gcd_val];
    let stft_rate_d = [u64::from(hop_size) / gcd_val];

    let desc_pcm = [
        f(1, V::MetadataRefs(&one_u32)),
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("pressure")),
        f(8, V::Utf8("Pa")),
        f(9, V::U32(&one_u32)),
        f(10, V::Utf8("time-channel")),
        f(11, V::U32(&f32_scalar)),
        f(12, V::Utf8("urn:qualia:qac:pcm-preservation:1")),
        f(13, V::U64(&rate_n)),
        f(14, V::U64(&rate_d)),
        f(18, V::MetadataRefs(&pcm_ext)),
    ];

    let desc_stft = [
        f(1, V::MetadataRefs(&one_u32)),
        f(3, V::MetadataRefs(&four_u32)),
        f(5, V::MetadataRefs(&eleven_u32)),
        f(7, V::Utf8("complex_pressure_spectrum")),
        f(8, V::Utf8("Pa")),
        f(9, V::U32(&one_u32)),
        f(10, V::Utf8("frame-channel-bin")),
        f(11, V::U32(&cf32_scalar)),
        f(12, V::Utf8("urn:qualia:qac:stft-preservation:1")),
        f(13, V::U64(&stft_rate_n)),
        f(14, V::U64(&stft_rate_d)),
        f(17, V::EntryRefs(&one_u32)),
        f(18, V::MetadataRefs(&stft_ext)),
    ];

    let desc_hoa = [
        f(1, V::MetadataRefs(&one_u32)),
        f(2, V::MetadataRefs(&two_u32)),
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("hoa_ambisonic_pressure")),
        f(8, V::Utf8("Pa")),
        f(9, V::U32(&four_u32)),
        f(10, V::Utf8("time-channel")),
        f(11, V::U32(&f32_scalar)),
        f(12, V::Utf8(HOA_MODEL_URI)),
        f(13, V::U64(&rate_n)),
        f(14, V::U64(&rate_d)),
        f(18, V::MetadataRefs(&hoa_ext)),
    ];

    let desc_modal = [
        f(3, V::MetadataRefs(&three_u32)),
        f(7, V::Utf8("damped_modal_resonances")),
        f(8, V::Utf8("Pa")),
        f(9, V::U32(&one_u32)),
        f(10, V::Utf8("mode-record")),
        f(11, V::U32(&f32_scalar)),
        f(12, V::Utf8(MODAL_MODEL_URI)),
        f(18, V::MetadataRefs(&modal_ext)),
    ];

    let blocks = [
        blk(1, 1, &clock_fields),
        blk(2, 2, &frame_fields),
        blk(3, 4, &pcm_prov),
        blk(4, 4, &stft_prov),
        blk(5, 205, &pcm_shape),
        blk(6, 205, &stft_shape),
        blk(7, 205, &hoa_shape),
        blk(8, 205, &modal_shape),
        blk(9, 210, &pcm_enc),
        blk(10, 210, &stft_enc),
        blk(11, 201, &transform_201),
        blk(12, 210, &hoa_enc),
        blk(13, 204, &spatial_204),
        blk(14, 206, &modal_206),
        blk(15, 3, &desc_pcm),
        blk(16, 3, &desc_stft),
        blk(17, 3, &desc_hoa),
        blk(18, 3, &desc_modal),
    ];

    let storage_limits = StorageLimits::default();
    let meta_input = MetadataInput { blocks: &blocks };
    let meta_len = meta_input
        .encoded_len(storage_limits)
        .map_err(|e| format!("QAC v1 metadata length error: {e:?}"))?;
    let mut meta_buf = vec![0u8; meta_len];
    meta_input
        .encode_into(&mut meta_buf, storage_limits)
        .map_err(|e| format!("QAC v1 metadata encode error: {e:?}"))?;

    let mk_entry = |mode, flags, stream_id, frame_id, time_count, record_count, stride, descriptor_id, payload| {
        EntryInput {
            mode,
            revision: 1,
            flags,
            stream_id,
            frame_id,
            start_tick: 0,
            time_count,
            spatial_key: 0,
            record_count,
            stride,
            codec: 0,
            descriptor_id,
            payload,
        }
    };
    let entries = [
        mk_entry(QAC_MODE_PCM_PRESERVATION, 2, 1, 0, sample_frames, sample_frames, 4, 15, pcm_bytes),
        mk_entry(QAC_MODE_STFT_PRESERVATION, 2, 2, 0, stft_frames, stft_frames, fft_size * 8, 16, stft_bytes),
        mk_entry(QAC_MODE_HOA_SPATIAL_AUDIO, 2, 3, 2, sample_frames, sample_frames, 16, 17, hoa_bytes),
        mk_entry(QAC_MODE_MODAL_RESONANCE, 0, 4, 0, 0, modal_count, 32, 18, modal_bytes),
    ];

    let env = EnvelopeInput {
        family: Family::QacV1Candidate,
        flags: 0,
        metadata: &meta_buf,
        entries: &entries,
    };
    let total_len = env
        .encoded_len(storage_limits)
        .map_err(|e| format!("QAC v1 envelope length error: {e:?}"))?;
    let mut output = vec![0u8; total_len];
    env.encode_into(&mut output, storage_limits)
        .map_err(|e| format!("QAC v1 envelope encode error: {e:?}"))?;
    Ok(output)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

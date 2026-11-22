//! Passive QAC profile conversion, acoustic analysis, and tensor sonification.

use crate::{fields, list, num, record, required, text, Args, AssetHost, Resource, Value};
use qualia_core_db::{
    audio::{
        bioacoustics::{
            compile_vocalizations_to_sparql_mm, detect_bioacoustic_vocalizations,
            BioacousticVocalization, MA_HAS_END_TIME, MA_HAS_START_TIME, MA_IS_FRAGMENT_OF,
            QUALIA_ACOUSTIC_OBSERVATION, QUALIA_SPECIES_CONFIDENCE, QUALIA_TAXON_ID,
        },
        dsp_kernel::{configure_voice_from_tensor, parametric_sample},
        qac::{
            numerical::{
                solve_biot_poroelastic_dispersion, BiotPoroelasticParams,
                ZWICKER_24_BARK_CENTER_HZ, ZWICKER_BARK_BANDS,
            },
            profiles::{
                bridge_perceptual_view_zwicker_sones, find_ultrasound_summary_entry_index,
                migrate_legacy_q4au_v3_hrir, read_haptic_qac, read_perceptual_features_qac,
                read_ultrasound_qac, write_haptic_qac, write_ultrasound_qac, AdmittedEnvelope,
                ClockSpec, FrameSpec, HapticContactSpec, HapticRecord, HapticRole, PerceptualScale,
                ProfileLimits, ProvenanceSpec, QuaternionXyzw, RationalRate, RawUltrasoundPayload,
                UltrasoundAcquisitionSpec, UltrasoundBeamSummaryRecord, UltrasoundRole,
                HAPTIC_RECORD_STRIDE, REFERENCE_PRESSURE_AIR_PA, ULTRASOUND_SUMMARY_STRIDE,
            },
        },
        tf_surface::TfSurface,
        ParametricVoiceState, SPECTRAL_PREVIEW_BINS,
    },
    bundle::semantic_model::SemanticModelBuilder,
    q_hash, NQuin,
};
use std::{collections::BTreeMap, mem::size_of};

const MAX_ROWS: usize = 65_536;
const MAX_RASTER_CELLS: usize = 1_048_576;
const MAX_VOCALIZATIONS: usize = 4096;
const DETECTION_CONFIDENCE: &str = "qualia:vocalizationDetectionConfidence";

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "ultrasound_write" => ultrasound_write(host, params),
        "ultrasound_read" => ultrasound_read(host, params),
        "haptic_measurement_write" => haptic_measurement_write(host, params),
        "haptic_read" => haptic_read(host, params),
        "bioacoustic_vocalizations" => vocalizations(host, params),
        "migrate_q4au_v3_hrir" => migrate_hrir(host, params),
        "zwicker_sones" => zwicker_sones(host, params),
        "biot_dispersion" => biot_dispersion(params),
        "epistemic_tensor_voice" => tensor_voice(host, params),
        _ => Err(format!("Unsupported acoustic bridge operation: {op}")),
    }
}

fn resource<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
    kind: &str,
) -> Result<&'a Resource, String> {
    let resource = match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|entry| &entry.key == key)
            .ok_or_else(|| format!("Unknown retained acoustic resource {key}"))?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != kind || resource.bytes.len() > host.limits.resource_bytes {
        return Err(format!("Expected a bounded {kind} resource"));
    }
    Ok(resource)
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected unsigned integer".into()),
        _ => Err("Expected unsigned integer".into()),
    }
}

fn u32_value(value: &Value) -> Result<u32, String> {
    u32::try_from(unsigned(value)?).map_err(|_| "Acoustic integer exceeds u32".into())
}

fn f32_value(value: &Value) -> Result<f32, String> {
    let result = num(value)? as f32;
    if !result.is_finite() {
        return Err("Acoustic scalar exceeds f32 range".into());
    }
    Ok(result)
}

fn identifier(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    if value.is_empty() || value.len() > 2048 || value.chars().any(char::is_control) {
        return Err("Acoustic identifier is empty or exceeds its UTF-8 limit".into());
    }
    Ok(value)
}

fn rational(value: &Value) -> Result<RationalRate, String> {
    let [numerator, denominator] = list(value)? else {
        return Err("Acoustic rate requires [numerator, denominator]".into());
    };
    RationalRate::new(unsigned(numerator)?, unsigned(denominator)?)
        .map_err(|error| error.to_string())
}

fn limits(host: &AssetHost<'_>) -> ProfileLimits {
    let mut limits = ProfileLimits::default();
    let bytes = host.limits.resource_bytes as u64;
    limits.max_records_per_chunk = limits.max_records_per_chunk.min(MAX_ROWS as u64);
    limits.envelope.file_bytes = limits.envelope.file_bytes.min(bytes);
    limits.envelope.chunk_bytes = limits.envelope.chunk_bytes.min(bytes);
    limits.envelope.aggregate_decoded_bytes = limits
        .envelope
        .aggregate_decoded_bytes
        .min(host.limits.total_resource_bytes as u64);
    limits
}

fn clock(value: &Value) -> Result<ClockSpec, String> {
    let row = record(value)?;
    fields(
        row,
        &[
            "scale",
            "epoch_seconds",
            "epoch_nanoseconds",
            "ticks_per_second",
        ],
    )?;
    let seconds = match required(row, "epoch_seconds")? {
        Value::I64(value) => *value,
        Value::U64(value) => i64::try_from(*value).map_err(|_| "Epoch clock exceeds i64")?,
        _ => return Err("Epoch clock requires integer seconds".into()),
    };
    let spec = ClockSpec {
        scale: u32_value(required(row, "scale")?)?,
        epoch_seconds: seconds,
        epoch_nanoseconds: u32_value(required(row, "epoch_nanoseconds")?)?,
        ticks_per_second: rational(required(row, "ticks_per_second")?)?,
    };
    spec.validate().map_err(|error| error.to_string())?;
    Ok(spec)
}

enum Samples {
    Rf(Vec<f32>),
    Iq(Vec<[f32; 2]>),
}

struct RawCapture {
    samples: Samples,
    channels: u32,
    rate: RationalRate,
    digest: [u8; 32],
    bytes: usize,
}

impl RawCapture {
    fn borrow(&self) -> RawUltrasoundPayload<'_> {
        match &self.samples {
            Samples::Rf(values) => RawUltrasoundPayload::RawRfF32 {
                channels: self.channels,
                sample_rate: self.rate,
                samples_interleaved: values,
            },
            Samples::Iq(values) => RawUltrasoundPayload::BasebandIqComplexF32 {
                channels: self.channels,
                sample_rate: self.rate,
                iq_interleaved: values,
            },
        }
    }
}

fn floats(bytes: &[u8]) -> Result<Vec<f32>, String> {
    if bytes.is_empty() || bytes.len() % 4 != 0 {
        return Err("Little-endian f32 acoustic input has an invalid extent".into());
    }
    bytes
        .chunks_exact(4)
        .map(|chunk| {
            let value = f32::from_le_bytes(chunk.try_into().expect("four-byte f32 lane"));
            if value.is_finite() {
                Ok(value)
            } else {
                Err("Acoustic input contains non-finite samples".into())
            }
        })
        .collect()
}

fn raw_capture(host: &AssetHost<'_>, value: &Value) -> Result<RawCapture, String> {
    let row = record(value)?;
    fields(row, &["source", "format", "channels", "rate"])?;
    let bytes = &resource(host, required(row, "source")?, "bin")?.bytes;
    let channels = u32_value(required(row, "channels")?)?;
    if channels == 0 || channels > limits(host).max_channels {
        return Err("Raw ultrasound channel count exceeds the native profile".into());
    }
    let rate = rational(required(row, "rate")?)?;
    let format = text(required(row, "format")?)?;
    let lane_count = match format {
        "rf_f32le" => 1,
        "iq_f32le" => 2,
        _ => return Err("Unknown passive ultrasound capture representation".into()),
    };
    let frame_bytes = channels as usize * lane_count * 4;
    if bytes.is_empty() || bytes.len() % frame_bytes != 0 || bytes.len() / frame_bytes > MAX_ROWS {
        return Err("Raw ultrasound capture has a partial frame or exceeds record limits".into());
    }
    let decoded = floats(bytes)?;
    let samples = if lane_count == 1 {
        Samples::Rf(decoded)
    } else {
        Samples::Iq(
            decoded
                .chunks_exact(2)
                .map(|pair| [pair[0], pair[1]])
                .collect(),
        )
    };
    Ok(RawCapture {
        samples,
        channels,
        rate,
        digest: *blake3::hash(bytes).as_bytes(),
        bytes: bytes.len(),
    })
}

fn ultrasound_write(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "summary",
            "validity_mask",
            "raw",
            "clock",
            "sample_rate",
            "instrument_uri",
            "center_frequency_hz",
            "thermal_model_uri",
            "exposure_context",
        ],
    )?;
    let summaries = required(params, "summary")?;
    let masks = params.get("validity_mask");
    let input = &resource(host, summaries, "q07-summary")?.bytes;
    let count = input.len() / ULTRASOUND_SUMMARY_STRIDE as usize;
    if count == 0 || count > MAX_ROWS || count * ULTRASOUND_SUMMARY_STRIDE as usize != input.len() {
        return Err("Q07 summary source must contain complete bounded 64-byte records".into());
    }
    let digest = *blake3::hash(input).as_bytes();
    let mask = masks
        .map(|value| resource(host, value, "bin"))
        .transpose()?
        .map(|res| res.bytes.as_slice());
    if mask.is_some_and(|mask| mask.len() != count || mask.iter().any(|&sample| sample > 1)) {
        return Err("Q07 external validity mask must match summary record count".into());
    }
    let mut source_digests = vec![digest];
    if let Some(mask) = mask {
        source_digests.push(*blake3::hash(mask).as_bytes());
    }
    let records = input
        .chunks_exact(ULTRASOUND_SUMMARY_STRIDE as usize)
        .enumerate()
        .map(|(index, row)| {
            UltrasoundBeamSummaryRecord::decode(row, mask.is_none_or(|mask| mask[index] == 1))
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let input_bytes = input.len();
    let raw = params
        .get("raw")
        .map(|row| raw_capture(host, row))
        .transpose()?;
    if let Some(raw) = &raw {
        source_digests.push(raw.digest);
    }
    let bytes = input_bytes.saturating_add(raw.as_ref().map_or(0, |raw| raw.bytes));
    host.charge(bytes, bytes.saturating_mul(3))?;
    let mask = masks
        .map(|value| resource(host, value, "bin"))
        .transpose()?
        .map(|res| res.bytes.as_slice());
    let acquisition = UltrasoundAcquisitionSpec {
        role: UltrasoundRole::BeamSummary,
        center_frequency_hz: params.get("center_frequency_hz").map(num).transpose()?,
        element_positions_m: None,
        channel_map: None,
        transmit_delays_s: None,
        demodulation_phase_rad: None,
        prf: None,
        derating_model_uri: None,
        exposure_context: params.get("exposure_context").map(identifier).transpose()?,
        thermal_model_uri: params
            .get("thermal_model_uri")
            .map(identifier)
            .transpose()?,
        instrument_digest: None,
        authority_receipt_digest: None,
        target_frame: None,
        calibration: None,
    };
    let provenance = ProvenanceSpec {
        role: 2,
        source_digests: &source_digests,
        algorithm: Some("urn:qualia:audio:q07:passive-pack"),
        algorithm_version: Some("1"),
        instrument_uri: params.get("instrument_uri").map(identifier).transpose()?,
        authority_receipt_digest: None,
        lineage: None,
    };
    let raw_view = raw.as_ref().map(RawCapture::borrow);
    let qac = write_ultrasound_qac(
        &records,
        &acquisition,
        raw_view.as_ref(),
        &clock(required(params, "clock")?)?,
        rational(required(params, "sample_rate")?)?,
        &FrameSpec::local_default(),
        &provenance,
        mask,
        limits(host),
    )
    .map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("qac".into(), host.encode("qac", qac)?),
        ("record_count".into(), Value::U64(count as u64)),
    ])))
}

fn ultrasound_read(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["qac", "entry"])?;
    let input = required(params, "qac")?;
    let size = resource(host, input, "qac")?.bytes.len();
    host.charge(size, size.saturating_mul(3))?;
    let bytes = &resource(host, input, "qac")?.bytes;
    let limits = limits(host);
    let entry = if let Some(entry) = params.get("entry") {
        u32_value(entry)?
    } else {
        find_ultrasound_summary_entry_index(bytes, limits).map_err(|error| error.to_string())?
    };
    let view = read_ultrasound_qac(bytes, entry, limits).map_err(|error| error.to_string())?;
    let count = view.records.len();
    let role = view.role.as_str();
    let mut raw_summary = vec![0; count * ULTRASOUND_SUMMARY_STRIDE as usize];
    let mut mask = Vec::with_capacity(count);
    for (index, output) in raw_summary
        .chunks_exact_mut(ULTRASOUND_SUMMARY_STRIDE as usize)
        .enumerate()
    {
        let row = view.records.get(index).map_err(|error| error.to_string())?;
        let valid = row.center_frequency_hz > 0.0;
        mask.push(u8::from(valid));
        row.encode_into(output, valid)
            .map_err(|error| error.to_string())?;
    }
    let mut result = BTreeMap::from([
        ("mode".into(), Value::U64(0x0107)),
        ("entry".into(), Value::U64(u64::from(entry))),
        ("role".into(), Value::String(role.into())),
        ("record_count".into(), Value::U64(count as u64)),
    ]);
    for (name, uri) in [
        ("thermal_model_uri", view.thermal_model_uri),
        ("exposure_context", view.exposure_context),
    ] {
        if let Some(uri) = uri {
            result.insert(name.into(), Value::String(uri.into()));
        }
    }
    if let Some(frequency) = view.center_frequency_hz {
        result.insert("center_frequency_hz".into(), Value::F64(frequency));
    }
    if let Some(phase) = view.demodulation_phase_rad {
        result.insert("demodulation_phase_rad".into(), Value::F64(phase));
    }
    let max_channels = limits.max_channels as usize;
    if let Some(positions) = view.element_positions_m {
        if positions.len() > max_channels {
            return Err("Ultrasound element positions exceed the native channel limit".into());
        }
        result.insert(
            "element_positions_m".into(),
            Value::List(
                positions
                    .into_iter()
                    .map(|xyz| {
                        Value::List(
                            xyz.into_iter()
                                .map(|value| Value::F64(f64::from(value)))
                                .collect(),
                        )
                    })
                    .collect(),
            ),
        );
    }
    if let Some(channels) = view.channel_map {
        if channels.len() > max_channels {
            return Err("Ultrasound channel map exceeds the native channel limit".into());
        }
        result.insert(
            "channel_map".into(),
            Value::List(
                channels
                    .into_iter()
                    .map(|channel| Value::U64(u64::from(channel)))
                    .collect(),
            ),
        );
    }
    let raw_source = view.raw_source_bytes.map(<[u8]>::to_vec);
    result.insert("summary".into(), host.encode("q07-summary", raw_summary)?);
    result.insert("validity_mask".into(), host.encode("bin", mask)?);
    if let Some(raw) = raw_source {
        result.insert("raw_source".into(), host.encode("bin", raw)?);
    }
    Ok(Value::Record(result))
}

fn haptic_measurement_write(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "samples",
            "validity_mask",
            "clock",
            "sample_rate",
            "contact_area_m2",
            "rotation_xyzw",
        ],
    )?;
    let samples = required(params, "samples")?;
    let len = resource(host, samples, "q07-haptic")?.bytes.len();
    let stride = HAPTIC_RECORD_STRIDE as usize;
    let count = len / stride;
    if count == 0 || count > MAX_ROWS || len % stride != 0 {
        return Err("Q07 haptic source must contain complete bounded 64-byte records".into());
    }
    host.charge(len, len.saturating_mul(3))?;
    let input = &resource(host, samples, "q07-haptic")?.bytes;
    let mask = params
        .get("validity_mask")
        .map(|value| resource(host, value, "bin"))
        .transpose()?
        .map(|res| res.bytes.as_slice());
    if mask.is_some_and(|mask| mask.len() != count || mask.iter().any(|&sample| sample > 1)) {
        return Err("Q07 haptic validity mask must match the record count".into());
    }
    let records = input
        .chunks_exact(stride)
        .enumerate()
        .map(|(index, row)| {
            HapticRecord::decode(row, mask.is_none_or(|mask| mask[index] == 1))
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut source_digests = vec![*blake3::hash(input).as_bytes()];
    if let Some(mask) = mask {
        source_digests.push(*blake3::hash(mask).as_bytes());
    }
    let mut contact_frame = FrameSpec::local_default();
    if let Some(value) = params.get("rotation_xyzw") {
        let [x, y, z, w] = list(value)? else {
            return Err("Haptic rotation requires [x, y, z, w]".into());
        };
        contact_frame.rotation = QuaternionXyzw::from_xyzw([num(x)?, num(y)?, num(z)?, num(w)?])
            .map_err(|error| error.to_string())?;
    }
    let contact = HapticContactSpec {
        role: HapticRole::Measurement {
            receptor_model_uri: None,
        },
        contact_frame,
        contact_area_m2: params.get("contact_area_m2").map(num).transpose()?,
    };
    let provenance = ProvenanceSpec {
        role: 2,
        source_digests: &source_digests,
        algorithm: Some("urn:qualia:audio:q07:haptic-measurement"),
        algorithm_version: Some("1"),
        instrument_uri: None,
        authority_receipt_digest: None,
        lineage: None,
    };
    let qac = write_haptic_qac(
        &records,
        &contact,
        None,
        &clock(required(params, "clock")?)?,
        rational(required(params, "sample_rate")?)?,
        &FrameSpec::local_default(),
        &provenance,
        mask,
        limits(host),
    )
    .map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("qac".into(), host.encode("qac", qac)?),
        ("record_count".into(), Value::U64(count as u64)),
    ])))
}

fn haptic_read(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["qac", "entry"])?;
    let input = required(params, "qac")?;
    let entry = u32_value(required(params, "entry")?)?;
    let size = resource(host, input, "qac")?.bytes.len();
    host.charge(size, size.saturating_mul(3))?;
    let bytes = &resource(host, input, "qac")?.bytes;
    let limits = limits(host);
    let view = read_haptic_qac(bytes, entry, limits).map_err(|error| error.to_string())?;
    let receptor = match &view.role {
        HapticRole::Measurement { receptor_model_uri } => receptor_model_uri.clone(),
        HapticRole::ReceptorModel { receptor_model_uri } => Some(receptor_model_uri.clone()),
        HapticRole::ActuatorCommand { .. } => {
            return Err("Haptic bridge accepts passive entries only".into())
        }
    };
    let envelope =
        AdmittedEnvelope::open_alloc(bytes, limits).map_err(|error| error.to_string())?;
    let selected = envelope.entry(entry).map_err(|error| error.to_string())?;
    let descriptor = envelope
        .descriptor_for_entry(selected)
        .map_err(|error| error.to_string())?;
    let count = view.records.len();
    let mask = envelope
        .read_validity_mask(descriptor, selected.record_count())
        .map_err(|error| error.to_string())?
        .map_or_else(|| vec![1; count], <[u8]>::to_vec);
    if mask.iter().any(|&value| value > 1) {
        return Err("Q07 haptic entry contains a non-binary validity mask".into());
    }
    let mut samples = vec![0; count * HAPTIC_RECORD_STRIDE as usize];
    for (index, output) in samples
        .chunks_exact_mut(HAPTIC_RECORD_STRIDE as usize)
        .enumerate()
    {
        view.records
            .get(index)
            .and_then(|row| row.encode_into(output, mask[index] == 1))
            .map_err(|error| error.to_string())?;
    }
    let rotation = view.contact_frame_rotation;
    let mut result = BTreeMap::from([
        ("entry".into(), Value::U64(u64::from(entry))),
        ("role".into(), Value::String(view.role.as_str().into())),
        ("record_count".into(), Value::U64(count as u64)),
        (
            "rotation_xyzw".into(),
            Value::List(
                [rotation.x, rotation.y, rotation.z, rotation.w]
                    .into_iter()
                    .map(Value::F64)
                    .collect(),
            ),
        ),
    ]);
    if let Some(receptor) = receptor {
        result.insert("receptor_model_uri".into(), Value::String(receptor));
    }
    if let Some(area) = view.contact_area_m2 {
        result.insert("contact_area_m2".into(), Value::F64(area));
    }
    result.insert("samples".into(), host.encode("q07-haptic", samples)?);
    result.insert("validity_mask".into(), host.encode("bin", mask)?);
    Ok(Value::Record(result))
}

fn migrate_hrir(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["legacy"])?;
    let legacy = required(params, "legacy")?;
    let source_size = resource(host, legacy, "q4au")?.bytes.len();
    let bounded_extent = source_size.saturating_mul(3).saturating_add(16 * 1024);
    if bounded_extent > host.limits.resource_bytes {
        return Err("Legacy SOFA preservation and import would exceed the output budget".into());
    }
    host.charge(source_size, bounded_extent)?;
    let bytes = migrate_legacy_q4au_v3_hrir(&resource(host, legacy, "q4au")?.bytes, limits(host))
        .map_err(|error| error.to_string())?;
    host.encode("qac", bytes)
}

fn zwicker_sones(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["qac", "entry", "frame"])?;
    let input = required(params, "qac")?;
    let entry = u32_value(required(params, "entry")?)?;
    let frame = u32_value(required(params, "frame")?)? as usize;
    let size = resource(host, input, "qac")?.bytes.len();
    host.charge(size, size.saturating_mul(3))?;
    let bytes = &resource(host, input, "qac")?.bytes;
    let limits = limits(host);
    let envelope =
        AdmittedEnvelope::open_alloc(bytes, limits).map_err(|error| error.to_string())?;
    let selected = envelope.entry(entry).map_err(|error| error.to_string())?;
    if selected.stride() as usize != ZWICKER_BARK_BANDS * size_of::<f32>()
        || frame as u64 >= selected.record_count()
    {
        return Err("Zwicker analysis requires one in-range 24-band frame".into());
    }
    let descriptor = envelope
        .descriptor_for_entry(selected)
        .map_err(|error| error.to_string())?;
    let mask = envelope
        .read_validity_mask(descriptor, selected.record_count())
        .map_err(|error| error.to_string())?;
    if mask.is_some_and(|mask| mask[frame] != 1) {
        return Err("Selected Zwicker frame is masked".into());
    }
    let view =
        read_perceptual_features_qac(bytes, entry, limits).map_err(|error| error.to_string())?;
    if view.scale != PerceptualScale::DbSpl
        || !view
            .reference_pressure_pa
            .is_some_and(|p| (p - REFERENCE_PRESSURE_AIR_PA).abs() <= 1e-12)
        || !view.centers_hz.as_ref().is_some_and(|centers| {
            centers.len() == ZWICKER_BARK_BANDS
                && centers
                    .iter()
                    .zip(ZWICKER_24_BARK_CENTER_HZ)
                    .all(|(&declared, canonical)| f64::from(declared) == canonical)
        })
    {
        return Err(
            "Zwicker analysis requires standard Bark centers and dB-SPL referenced to 20 µPa"
                .into(),
        );
    }
    let (report, specific) =
        bridge_perceptual_view_zwicker_sones(&view, frame).map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        (
            "status".into(),
            Value::String(format!("{:?}", report.status)),
        ),
        (
            "total_sones".into(),
            Value::F64(report.total_loudness_sones),
        ),
        (
            "loudness_phons".into(),
            Value::F64(report.loudness_level_phons),
        ),
        (
            "peak_sones_per_bark".into(),
            Value::F64(report.peak_specific_loudness_sones_per_bark),
        ),
        (
            "specific_sones_per_bark".into(),
            Value::List(specific.into_iter().map(Value::F64).collect()),
        ),
    ])))
}

fn biot_dispersion(params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "omega_rad_s",
            "porosity",
            "rho_fluid_kg_m3",
            "rho_solid_kg_m3",
            "tortuosity_inf",
            "permeability_m2",
            "viscosity_pa_s",
            "biot_willis_alpha",
            "biot_modulus_m_pa",
            "frame_bulk_modulus_pa",
            "frame_shear_modulus_pa",
        ],
    )?;
    let scalar = |name| num(required(params, name)?);
    let material = BiotPoroelasticParams {
        porosity: scalar("porosity")?,
        rho_fluid_kg_m3: scalar("rho_fluid_kg_m3")?,
        rho_solid_kg_m3: scalar("rho_solid_kg_m3")?,
        tortuosity_inf: scalar("tortuosity_inf")?,
        permeability_m2: scalar("permeability_m2")?,
        viscosity_pa_s: scalar("viscosity_pa_s")?,
        biot_willis_alpha: scalar("biot_willis_alpha")?,
        biot_modulus_m_pa: scalar("biot_modulus_m_pa")?,
        frame_bulk_modulus_pa: scalar("frame_bulk_modulus_pa")?,
        frame_shear_modulus_pa: scalar("frame_shear_modulus_pa")?,
    };
    let result = solve_biot_poroelastic_dispersion(&material, scalar("omega_rad_s")?)
        .map_err(|error| format!("Biot poroelastic dispersion failed: {error:?}"))?;
    let values = [
        ("omega_rad_s", result.omega_rad_s),
        ("c_p1_m_s", result.c_p1_m_s),
        ("alpha_p1_np_m", result.alpha_p1_np_m),
        ("c_p2_m_s", result.c_p2_m_s),
        ("alpha_p2_np_m", result.alpha_p2_np_m),
        ("c_s_m_s", result.c_s_m_s),
        ("alpha_s_np_m", result.alpha_s_np_m),
    ];
    if values.iter().any(|(_, value)| !value.is_finite()) {
        return Err("Biot poroelastic dispersion produced a non-finite result".into());
    }
    Ok(Value::Record(
        values
            .into_iter()
            .map(|(name, value)| (name.into(), Value::F64(value)))
            .collect(),
    ))
}

fn tensor_voice(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "q",
            "mu",
            "alpha",
            "bins",
            "sample_rate",
            "sample_count",
            "phase",
        ],
    )?;
    let bins = list(required(params, "bins")?)?;
    if bins.len() != SPECTRAL_PREVIEW_BINS {
        return Err(format!(
            "Tensor sonification expects {SPECTRAL_PREVIEW_BINS} spectral bins"
        ));
    }
    let mut spectrum = [0.0; SPECTRAL_PREVIEW_BINS];
    for (output, input) in spectrum.iter_mut().zip(bins) {
        *output = f32_value(input)?;
    }
    let rate = u32_value(required(params, "sample_rate")?)?;
    let count = u32_value(required(params, "sample_count")?)? as usize;
    if !(8_000..=192_000).contains(&rate) || !(1..=MAX_ROWS).contains(&count) {
        return Err("Tensor voice rate or sample count exceeds the production range".into());
    }
    let phase = params
        .get("phase")
        .map(f32_value)
        .transpose()?
        .unwrap_or_default();
    if !(0.0..=1.0).contains(&phase) {
        return Err("Tensor voice phase must be within [0, 1]".into());
    }
    let mut voice = ParametricVoiceState {
        phase,
        ..ParametricVoiceState::default()
    };
    configure_voice_from_tensor(
        &mut voice,
        f32_value(required(params, "q")?)?,
        f32_value(required(params, "mu")?)?,
        f32_value(required(params, "alpha")?)?,
        &spectrum,
    );
    if 2.0 * voice.frequency_hz * (1.0 + 0.01 * voice.fm_index) >= rate as f32 {
        return Err(
            "Tensor voice effective frequency exceeds the sample-rate Nyquist limit".into(),
        );
    }
    host.charge(count, count * size_of::<f32>())?;
    let mut bytes = Vec::with_capacity(count * size_of::<f32>());
    for _ in 0..count {
        bytes.extend_from_slice(&parametric_sample(&mut voice, rate as f32).to_le_bytes());
    }
    Ok(Value::Record(BTreeMap::from([
        ("pcm_f32le".into(), host.encode("f32le", bytes)?),
        ("sample_rate".into(), Value::U64(u64::from(rate))),
        ("sample_count".into(), Value::U64(count as u64)),
        (
            "frequency_hz".into(),
            Value::F64(f64::from(voice.frequency_hz)),
        ),
        ("gain".into(), Value::F64(f64::from(voice.gain))),
        ("fm_index".into(), Value::F64(f64::from(voice.fm_index))),
        ("next_phase".into(), Value::F64(f64::from(voice.phase))),
    ])))
}

fn vocalizations(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "raster",
            "frames",
            "bins",
            "sample_rate",
            "hop_size",
            "threshold_db",
            "min_duration_s",
            "recording_iri",
            "taxon_iri",
        ],
    )?;
    let frames = u32_value(required(params, "frames")?)? as usize;
    let bins = u32_value(required(params, "bins")?)? as usize;
    let cells = frames
        .checked_mul(bins)
        .filter(|&cells| cells > 0 && cells <= MAX_RASTER_CELLS)
        .ok_or("Bioacoustic time-frequency grid exceeds production dimensions")?;
    let rate = u32_value(required(params, "sample_rate")?)?;
    let hop = u32_value(required(params, "hop_size")?)? as usize;
    let threshold = f32_value(required(params, "threshold_db")?)?;
    let minimum = f32_value(required(params, "min_duration_s")?)?;
    if rate == 0
        || hop == 0
        || minimum <= 0.0
        || (frames as f64 * hop as f64 / f64::from(rate)) > 86_400.0
    {
        return Err("Bioacoustic time-frequency raster has an invalid duration or timebase".into());
    }
    let recording_iri = identifier(required(params, "recording_iri")?)?;
    let taxon_iri = params.get("taxon_iri").map(identifier).transpose()?;
    let raster = required(params, "raster")?;
    let source = &resource(host, raster, "f32le")?.bytes;
    if source.len() != cells * 4 {
        return Err("Bioacoustic raster bytes do not match the frame-by-bin grid".into());
    }
    host.charge(cells, cells * 4 + MAX_VOCALIZATIONS * 448)?;
    let source = &resource(host, raster, "f32le")?.bytes;
    let magnitudes = floats(source)?;
    let surface = TfSurface::new(&magnitudes, frames, bins, rate, hop);
    let mut calls = vec![BioacousticVocalization::default(); MAX_VOCALIZATIONS + 1];
    let mut graph = SemanticModelBuilder::new(host.limits.resource_bytes.min(42 * 1024 * 1024));
    let recording = graph
        .bind_iri(recording_iri)
        .map_err(|error| error.to_string())?;
    let taxon = taxon_iri
        .map(|iri| graph.bind_iri(iri))
        .transpose()
        .map_err(|error| error.to_string())?
        .unwrap_or(0);
    let count = detect_bioacoustic_vocalizations(&surface, threshold, minimum, taxon, &mut calls);
    if count > MAX_VOCALIZATIONS {
        return Err("Bioacoustic event count exceeds the Q42 observation budget".into());
    }
    let calls = &calls[..count];
    let mut quins = vec![NQuin::default(); count * 6];
    let written = compile_vocalizations_to_sparql_mm(recording, calls, &mut quins);
    for term in [
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        QUALIA_ACOUSTIC_OBSERVATION,
        MA_HAS_START_TIME,
        MA_HAS_END_TIME,
        MA_IS_FRAGMENT_OF,
        QUALIA_TAXON_ID,
        DETECTION_CONFIDENCE,
    ] {
        graph.bind_iri(term).map_err(|error| error.to_string())?;
    }
    let digest = blake3::hash(source).to_hex().to_string();
    for (index, group) in quins[..written].chunks_exact_mut(6).enumerate() {
        let fragment = format!("urn:qualia:acoustic:{recording:015x}:{digest}:{frames}:{bins}:{rate}:{hop}:{:08x}:{:08x}:{index}",
            threshold.to_bits(), minimum.to_bits());
        let subject = graph
            .bind_iri(&fragment)
            .map_err(|error| error.to_string())?;
        for row in group {
            row.subject = subject;
            if row.predicate == q_hash(QUALIA_SPECIES_CONFIDENCE) {
                row.predicate = q_hash(DETECTION_CONFIDENCE);
            }
            row.recalculate_parity();
        }
    }
    quins.truncate(written);
    if taxon_iri.is_none() {
        quins.retain(|row| row.predicate != q_hash(QUALIA_TAXON_ID));
    }
    let q42 = graph
        .seal_projection(quins)
        .map_err(|error| error.to_string())?;
    let observations = calls
        .iter()
        .map(|row| {
            Value::Record(BTreeMap::from([
                ("start_s".into(), Value::F64(f64::from(row.start_time_s))),
                ("end_s".into(), Value::F64(f64::from(row.end_time_s))),
                ("low_hz".into(), Value::F64(f64::from(row.low_freq_hz))),
                ("high_hz".into(), Value::F64(f64::from(row.high_freq_hz))),
                (
                    "peak_db".into(),
                    Value::F64(f64::from(row.peak_magnitude_db)),
                ),
                (
                    "detection_confidence".into(),
                    Value::F64(f64::from(row.confidence)),
                ),
            ]))
        })
        .collect();
    Ok(Value::Record(BTreeMap::from([
        ("q42".into(), host.encode("q42", q42)?),
        ("observations".into(), Value::List(observations)),
    ])))
}

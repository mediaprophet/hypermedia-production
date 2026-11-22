//! Measured directional room responses, ACN/SN3D HOA, and retained HRIR rendering.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::{atomic::Ordering, Arc}};

use acoustics_daw_session::codecs::{decode_wav, AudioBuffer};
use qualia_core_db::{
    audio::qac::{
        numerical::room_acoustics::{analyze_room_impulse_response, RoomGeometry, SurfacePatch},
        profiles::{self, read_hrir_qac, read_hoa_temporal_qac, write_hoa_temporal_qac, ClockSpec,
            FrameSpec, HoaDirectionConvention, HoaOrdering, HoaProfileSpec, ProfileLimits,
            ProvenanceSpec, QuaternionXyzw, RationalRate, BASE_DELAY_REFERENCE},
        spatial_dsp::{convolve_fir_into, convolve_stereo_hrir_with_additional_delays_into,
            encode_mono_to_hoa_accum_into, fft_radix2_interleaved_f32, HoaNormalization,
            HoaOrder, HoaWignerRotation},
    },
    etalon_render::skeletal::Motor,
};
use sha2::{Digest, Sha256};

use crate::{fields, integer, list, num, number, record, required, Args, AssetHost, Resource, TypedArtifact, Value};
use crate::{artifact::FieldLattice, primal_replay::COUNTER_ALGORITHM};

const MAX_PATHS: usize = 8;
const MAX_CHANNELS: usize = 16;
const MAX_FRAMES: usize = 1_048_576;
const MAX_HRIR_TAPS: u32 = 4096;
const MAX_AUDIO_BYTES: usize = 16 * 1024 * 1024;
const DIRECT_FIR_WORK: usize = 1_000_000;

struct AudioInput {
    audio: Arc<AudioBuffer>,
    digest: String,
}

struct EarPair {
    left: Vec<f32>,
    right: Vec<f32>,
    delays: [usize; 2],
}

struct MeasuredPath {
    dry_channel: usize,
    rir: AudioInput,
    rir_channel: usize,
    onset: usize,
    gain: f32,
    direction: [f32; 3],
    measurement: Option<usize>,
    hrir: Option<EarPair>,
    wet_len: usize,
}

impl AssetHost<'_> {
    pub(crate) fn room_rir_and_hoa_render(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["audio", "room", "paths", "listener_motor", "hoa_order", "hrir", "replay_seed"])?;
        let replay_seed = match args.get("replay_seed") {
            None => 0,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            _ => return Err("Room replay_seed requires an unsigned 64-bit integer".into()),
        };
        let paths = list(required(args, "paths")?)?;
        if !(1..=MAX_PATHS).contains(&paths.len()) {
            return Err("Room rendering requires one to eight measured directional RIR paths".into());
        }
        let order = integer(required(args, "hoa_order")?, "hoa_order")?;
        let native_order = hoa_order(order)?;
        let channels = native_order.channel_count();
        let (position, rotation) = listener_pose(required(args, "listener_motor")?)?;
        let room = room(record(required(args, "room")?)?)?;
        let dry = audio_input(self, required(args, "audio")?)?;
        let hrir_arg = args.get("hrir").map(record).transpose()?;
        let mut paths: Vec<MeasuredPath> = paths.iter().map(|item| {
            measured_path(self, record(item)?, &dry.audio, position)
        }).collect::<Result<_, _>>()?;
        let hrir_digest = if let Some(args) = hrir_arg {
            Some(bind_measured_hrir(self, args, &mut paths, dry.audio.sample_rate)?)
        } else {
            if paths.iter().any(|path| path.measurement.is_some()) {
                return Err("A binaural measurement index requires an admitted HRIR document".into());
            }
            None
        };
        let (frames, binaural_frames, work, fft_scratch) = render_budget(&paths, &dry.audio, channels)?;
        let scalar_count = frames * channels;
        let hoa_bytes = scalar_count * 4;
        let native_limit = self.limits.resource_bytes.min(MAX_AUDIO_BYTES);
        let etc_frames = paths.iter().map(|path| path.rir.audio.num_frames()).max().unwrap_or(0);
        let etc_count = paths.len() * etc_frames;
        if hoa_bytes > native_limit.saturating_sub(4096)
            || binaural_frames.saturating_mul(8) > self.limits.resource_bytes
            || etc_count.saturating_mul(4) > self.limits.resource_bytes
        {
            return Err("Room render exceeds the HOA or binaural output resource budget".into());
        }
        let scratch = paths.iter().map(|path| path.rir.audio.num_frames() * 12).sum::<usize>()
            .saturating_add(hoa_bytes.saturating_mul(3))
            .saturating_add(binaural_frames.saturating_mul(16))
            .saturating_add(fft_scratch)
            .saturating_add(etc_count.saturating_mul(4));
        self.charge(work.saturating_add(etc_count), scratch)?;
        let mut world_hoa = vec![0.0f32; scalar_count];
        let mut binaural = (binaural_frames != 0)
            .then(|| AudioBuffer::new(dry.audio.sample_rate, 2, binaural_frames));
        let mut reports = Vec::with_capacity(paths.len());
        let mut etc = vec![0.0_f32; etc_count];
        for (index, path) in paths.iter().enumerate() {
            if self.cancel.load(Ordering::Acquire) {
                return Err("Production cancelled".into());
            }
            let rir = &path.rir.audio.channels[path.rir_channel];
            reports.push(analyze(path, rir, dry.audio.sample_rate)?);
            for (energy, &value) in etc[index * etc_frames..][..rir.len()].iter_mut().zip(rir) {
                let weighted = f64::from(value) * f64::from(path.gain);
                *energy = (weighted * weighted) as f32;
                if !energy.is_finite() {
                    return Err("Measured energy-time checkpoint exceeds finite f32 range".into());
                }
            }
            let mut wet = vec![0.0; path.wet_len];
            let rendered = convolve_measured(&dry.audio.channels[path.dry_channel], rir, &mut wet)?;
            if rendered != path.wet_len {
                return Err("Native room FIR returned an inconsistent response length".into());
            }
            let extent = path.wet_len * channels;
            encode_mono_to_hoa_accum_into(&wet, native_order, HoaNormalization::Sn3d,
                path.direction, path.gain, &mut world_hoa[..extent])
                .map_err(|error| format!("Native HOA plane-wave projection: {error:?}"))?;
            if let (Some(pair), Some(output)) = (&path.hrir, binaural.as_mut()) {
                render_binaural(&wet, pair, path.gain, output)?;
            }
        }
        let mut listener_hoa = vec![0.0; scalar_count];
        let inverse = HoaWignerRotation::from_quaternion_xyzw(rotation)
            .map_err(|error| format!("Native HOA listener orientation: {error:?}"))?
            .transpose();
        let transformed = inverse.apply_stream_into_f32(native_order, &world_hoa, &mut listener_hoa)
            .map_err(|error| format!("Native HOA Wigner rotation: {error:?}"))?;
        if transformed != frames || listener_hoa.iter().any(|value| !value.is_finite()) {
            return Err("Native HOA rotation returned invalid dimensions or samples".into());
        }
        let encoded = seal_hoa(&listener_hoa, order, channels, dry.audio.sample_rate, position, rotation,
            &dry.digest, &paths, hrir_digest.as_deref())?;
        let primal = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(FieldLattice::DenseF32 {
            axes: vec![paths.len() as u32, etc_frames as u32], samples: etc,
        })))?;
        let mut output = BTreeMap::from([
            ("hoa".into(), self.encode("qac", encoded)?),
            ("hoa_order".into(), Value::U64(order as u64)),
            ("hoa_channels".into(), Value::U64(channels as u64)),
            ("hoa_frames".into(), Value::U64(frames as u64)),
            ("sample_rate".into(), Value::U64(u64::from(dry.audio.sample_rate))),
            ("rir_measurements".into(), Value::List(reports)),
            ("room_rt60_estimates".into(), room_estimates(&room)),
            ("primal_etc".into(), primal),
            ("primal_kind".into(), Value::String("measured_rir_squared_gain_weighted".into())),
            ("replay_seed".into(), Value::U64(replay_seed)),
            ("replay_rng".into(), Value::String(COUNTER_ALGORITHM.into())),
        ]);
        if let Some(audio) = binaural {
            output.insert("binaural".into(), self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(audio)))?);
        }
        if let Some(digest) = hrir_digest {
            output.insert("hrir_sha256".into(), Value::String(digest));
        }
        Ok(Value::Record(output))
    }
}

fn hoa_order(order: usize) -> Result<HoaOrder, String> {
    match order {
        0 => Ok(HoaOrder::Order0),
        1 => Ok(HoaOrder::Order1),
        2 => Ok(HoaOrder::Order2),
        3 => Ok(HoaOrder::Order3),
        _ => Err("Native HOA supports orders zero through three".into()),
    }
}

fn room(args: &Args) -> Result<RoomGeometry, String> {
    fields(args, &["volume_m3", "sound_speed_m_s", "air_attenuation_m_inv", "surfaces"])?;
    let surfaces = list(required(args, "surfaces")?)?;
    if !(1..=64).contains(&surfaces.len()) {
        return Err("Room geometry requires one to 64 authored absorbing surfaces".into());
    }
    let surfaces = surfaces.iter().map(|value| {
        let row = record(value)?;
        fields(row, &["area_m2", "absorption_coeff"])?;
        Ok(SurfacePatch { area_m2: num(required(row, "area_m2")?)?,
            absorption_coeff: num(required(row, "absorption_coeff")?)? })
    }).collect::<Result<_, String>>()?;
    let room = RoomGeometry {
        volume_m3: num(required(args, "volume_m3")?)?,
        sound_speed_m_s: num(required(args, "sound_speed_m_s")?)?,
        air_attenuation_m_inv: num(required(args, "air_attenuation_m_inv")?)?,
        surfaces,
    };
    if !room.validate_base().map_err(|error| format!("Native room geometry: {error:?}"))?.is_finite() {
        return Err("Native room geometry exceeds a finite aggregate surface area".into());
    }
    Ok(room)
}

fn room_estimates(room: &RoomGeometry) -> Value {
    let fit = |value: Result<f64, _>| value.ok().filter(|seconds| f64::is_finite(*seconds))
        .map_or(Value::Null, Value::F64);
    Value::Record(BTreeMap::from([
        ("sabine_s".into(), fit(room.sabine_rt60_s())),
        ("eyring_s".into(), fit(room.eyring_rt60_s())),
        ("millington_sette_s".into(), fit(room.millington_rt60_s())),
    ]))
}

fn listener_pose(value: &Value) -> Result<([f64; 3], [f64; 4]), String> {
    let [rw, rx, ry, rz, dw, dx, dy, dz] = vector(value)?;
    let rotor = [rw, rx, ry, rz];
    let dual = [dw, dx, dy, dz];
    let rotor_sq = rotor.iter().map(|v| v * v).sum::<f64>();
    let study = rotor.iter().zip(dual).map(|(a, b)| a * b).sum::<f64>();
    if (rotor_sq - 1.0).abs() > 1e-6 || study.abs() > 1e-6 {
        return Err("Listener motor requires near-unit rotor and Study orthogonality".into());
    }
    let motor = Motor { r_w: rw, r_x: rx, r_y: ry, r_z: rz, d_w: dw, d_x: dx, d_y: dy, d_z: dz };
    let (wxyz, position) = motor.normalize().to_rotation_translation();
    if position.iter().any(|p| !p.is_finite() || p.abs() > 1e6) {
        return Err("Listener motor translation exceeds the finite local room frame".into());
    }
    let [w, x, y, z] = wxyz;
    Ok((position, [x, y, z, w]))
}

fn vector<const N: usize>(value: &Value) -> Result<[f64; N], String> {
    let values = list(value)?;
    if values.len() != N {
        return Err(format!("Expected {N} finite spatial coefficients"));
    }
    let mut out = [0.0; N];
    for (slot, value) in out.iter_mut().zip(values) {
        *slot = num(value)?;
        if !slot.is_finite() || slot.abs() > 1e6 {
            return Err("Spatial coefficient exceeds the finite local room frame".into());
        }
    }
    Ok(out)
}

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host.inputs.iter().find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained acoustics resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn audio_input(host: &AssetHost<'_>, value: &Value) -> Result<AudioInput, String> {
    if let Ok(TypedArtifact::AudioMultiChannel(audio)) = host.workspace.typed(value) {
        validate_audio(audio)?;
        let mut sha = Sha256::new();
        sha.update(b"qualia:planar-f32-audio:v1");
        sha.update(audio.sample_rate.to_le_bytes());
        sha.update((audio.num_channels() as u64).to_le_bytes());
        sha.update((audio.num_frames() as u64).to_le_bytes());
        for sample in audio.channels.iter().flatten() {
            sha.update(sample.to_le_bytes());
        }
        return Ok(AudioInput { audio: Arc::clone(audio), digest: format!("{:x}", sha.finalize()) });
    }
    let source = retained(host, value)?;
    if source.kind != "wav" || source.bytes.len() > host.limits.resource_bytes.min(MAX_AUDIO_BYTES) {
        return Err("Room audio input must be retained WAV or typed AudioMultiChannel".into());
    }
    let (format, frames) = check_wav(&source.bytes)?;
    // Feed the scalar decoder only the admitted format and frames. Original
    // auxiliary chunks stay with the retained source covered by its digest.
    let mut canonical = Vec::with_capacity(44 + frames.len());
    canonical.extend_from_slice(b"RIFF");
    canonical.extend_from_slice(&(36 + frames.len() as u32).to_le_bytes());
    canonical.extend_from_slice(b"WAVEfmt ");
    canonical.extend_from_slice(&16u32.to_le_bytes());
    canonical.extend_from_slice(format);
    canonical.extend_from_slice(b"data");
    canonical.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    canonical.extend_from_slice(frames);
    let audio = decode_wav(&canonical).ok_or("Native room WAV decoder refused the admitted source")?;
    validate_audio(&audio)?;
    Ok(AudioInput { audio: Arc::new(audio), digest: format!("{:x}", Sha256::digest(&source.bytes)) })
}

fn validate_audio(audio: &AudioBuffer) -> Result<(), String> {
    let frames = audio.num_frames();
    if !(8_000..=192_000).contains(&audio.sample_rate) || !(1..=MAX_CHANNELS).contains(&audio.num_channels())
        || frames == 0 || frames > MAX_FRAMES
        || audio.channels.iter().any(|channel| channel.len() != frames
            || channel.iter().any(|x| !x.is_finite()))
    {
        return Err("Audio must have finite, equal-length channels at an admitted sample rate".into());
    }
    Ok(())
}

fn check_wav(bytes: &[u8]) -> Result<(&[u8], &[u8]), String> {
    // The native decoder accepts partial tails, so verify exact retained RIFF
    // extents, unique format/data chunks, and native sample widths first.
    if bytes.len() < 44 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("Expected retained RIFF/WAVE audio".into());
    }
    let word = |raw: &[u8]| u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    if (word(&bytes[4..8]) as usize).checked_add(8) != Some(bytes.len()) {
        return Err("WAV RIFF extent disagrees with the retained file".into());
    }
    let mut rest = &bytes[12..];
    let mut frame_bytes = None;
    let mut format = None;
    let mut samples = None;
    for _ in 0..1024 {
        if rest.is_empty() {
            break;
        }
        let header = rest.get(..8).ok_or("Truncated WAV chunk header")?;
        let length = word(&header[4..8]) as usize;
        let end = length.checked_add(8).ok_or("WAV chunk length overflow")?;
        let payload = rest.get(8..end).ok_or("Truncated WAV chunk payload")?;
        match &header[..4] {
            b"fmt " => {
                if frame_bytes.is_some() || payload.len() < 16 {
                    return Err("WAV requires a single complete native format chunk".into());
                }
                let width = u16::from_le_bytes([payload[14], payload[15]]);
                let code = u16::from_le_bytes([payload[0], payload[1]]);
                let channels = u16::from_le_bytes([payload[2], payload[3]]) as usize;
                let rate = word(&payload[4..8]);
                let stride = u16::from_le_bytes([payload[12], payload[13]]) as usize;
                if !matches!((code, width), (1, 16) | (3, 32)) || !(1..=MAX_CHANNELS).contains(&channels)
                    || !(8_000..=192_000).contains(&rate) || stride != channels * usize::from(width / 8)
                    || word(&payload[8..12]) != rate * stride as u32
                {
                    return Err("WAV requires valid bounded PCM16 or IEEE float32 framing".into());
                }
                frame_bytes = Some(stride);
                format = Some(&payload[..16]);
            }
            b"data" => {
                if frame_bytes.is_none() || samples.is_some() {
                    return Err("WAV requires exactly one data chunk after its format chunk".into());
                }
                samples = Some(payload);
            }
            _ => {}
        }
        let next = end.checked_add(length % 2).ok_or("WAV aligned extent overflow")?;
        rest = rest.get(next..).ok_or("Missing WAV alignment byte")?;
    }
    if !rest.is_empty() {
        return Err("WAV exceeds the bounded chunk count".into());
    }
    let stride = frame_bytes.ok_or("WAV has no admitted format")?;
    let data = samples.ok_or("WAV has no data chunk")?;
    if data.is_empty() || data.len() % stride != 0 || data.len() / stride > MAX_FRAMES {
        return Err("WAV frames are incomplete or exceed the admitted extent".into());
    }
    Ok((format.ok_or("WAV has no admitted native format payload")?, data))
}

fn measured_path(
    host: &AssetHost<'_>, args: &Args, dry: &AudioBuffer, listener: [f64; 3],
) -> Result<MeasuredPath, String> {
    fields(args, &["rir", "dry_channel", "rir_channel", "direct_onset_sample",
        "source_position_m", "gain", "hrir_measurement"])?;
    let dry_channel = integer(required(args, "dry_channel")?, "dry_channel")?;
    let rir_channel = integer(required(args, "rir_channel")?, "rir_channel")?;
    let onset = integer(required(args, "direct_onset_sample")?, "direct_onset_sample")?;
    let rir = audio_input(host, required(args, "rir")?)?;
    if dry.sample_rate != rir.audio.sample_rate || dry_channel >= dry.num_channels()
        || rir_channel >= rir.audio.num_channels() || onset >= rir.audio.num_frames()
    {
        return Err("Directional RIR requires matching sample rates and valid authored channel/onset indices".into());
    }
    let source: [f64; 3] = vector(required(args, "source_position_m")?)?;
    let displacement: [f64; 3] = std::array::from_fn(|i| source[i] - listener[i]);
    let distance = displacement.iter().map(|d| d * d).sum::<f64>().sqrt();
    if distance <= 1e-9 {
        return Err("RIR directional projection needs a source distinct from the listener origin".into());
    }
    let direction = displacement.map(|v| (v / distance) as f32);
    let gain = number(args, "gain", 1.0)?;
    if !gain.is_finite() || gain.abs() > 8.0 {
        return Err("RIR rendering gain must be finite and within [-8, 8]".into());
    }
    let measurement = args.get("hrir_measurement")
        .map(|value| integer(value, "hrir_measurement")).transpose()?;
    let wet_len = convolution_len(dry.num_frames(), rir.audio.num_frames(), 0)?;
    Ok(MeasuredPath { dry_channel, rir, rir_channel, onset, gain: gain as f32,
        direction, measurement, hrir: None, wet_len })
}

fn bind_measured_hrir(
    host: &AssetHost<'_>, args: &Args, paths: &mut [MeasuredPath], sample_rate: u32,
) -> Result<String, String> {
    fields(args, &["resource", "entry", "sha256"])?;
    let source = retained(host, required(args, "resource")?)?;
    if source.kind != "qac" || source.bytes.len() > host.limits.resource_bytes.min(MAX_AUDIO_BYTES) {
        return Err("Binaural rendering requires a bounded retained QAC HRIR document".into());
    }
    let sha = format!("{:x}", Sha256::digest(&source.bytes));
    if crate::text(required(args, "sha256")?)? != sha {
        return Err("Retained HRIR document does not match its pinned SHA-256".into());
    }
    let entry = integer(required(args, "entry")?, "entry")?;
    let entry = u32::try_from(entry).map_err(|_| "HRIR QAC entry does not fit its native index")?;
    let limits = ProfileLimits { max_taps: MAX_HRIR_TAPS, max_channels: 2, ..ProfileLimits::default() };
    let doc = read_hrir_qac(&source.bytes, entry, limits)
        .map_err(|error| format!("Native QAC HRIR admission: {error}"))?;
    if doc.representation != "HRIR" || doc.chunk.receivers() != 2
        || doc.impulse_sample_rate.num != u64::from(sample_rate) || doc.impulse_sample_rate.den != 1
        || doc.delay_reference != BASE_DELAY_REFERENCE
    {
        return Err("Binaural rendering needs matched-rate, exact left/right HRIR with excitation-relative delay".into());
    }
    for path in paths {
        let measurement = path.measurement.ok_or(
            "Every directional RIR needs a measured HRIR index when binaural rendering is requested",
        )?;
        if measurement >= doc.chunk.measurements() {
            return Err("Selected QAC HRIR measurement is outside its native directory".into());
        }
        let mut left = vec![0.0; doc.chunk.taps()];
        let mut right = vec![0.0; doc.chunk.taps()];
        doc.chunk.read_impulse(measurement, 0, &mut left).map_err(|error| error.to_string())?;
        doc.chunk.read_impulse(measurement, 1, &mut right).map_err(|error| error.to_string())?;
        let mut delays = [0; 2];
        if let Some(samples) = doc.delays_samples.as_deref() {
            for (ear, delay) in delays.iter_mut().enumerate() {
                let sample = *samples.get(measurement * 2 + ear).ok_or("Missing native HRIR delay")?;
                if !(0.0..=f64::from(MAX_HRIR_TAPS)).contains(&sample) || sample.fract() != 0.0 {
                    return Err("Native measured-HRIR branch requires nonnegative integer additional delays".into());
                }
                *delay = sample as usize;
            }
        }
        path.hrir = Some(EarPair { left, right, delays });
    }
    Ok(sha)
}

fn convolution_len(signal: usize, taps: usize, delay: usize) -> Result<usize, String> {
    signal.checked_add(taps).and_then(|length| length.checked_sub(1))
        .and_then(|length| length.checked_add(delay))
        .filter(|length| *length <= MAX_FRAMES)
        .ok_or("Acoustic convolution exceeds the native fixed frame budget".into())
}

fn render_budget(
    paths: &[MeasuredPath], dry: &AudioBuffer, hoa_channels: usize,
) -> Result<(usize, usize, usize, usize), String> {
    let mut frames = 0usize;
    let mut binaural = 0usize;
    let mut work = 0usize;
    let mut scratch = 0usize;
    for path in paths {
        frames = frames.max(path.wet_len);
        let (units, temporary) = convolution_cost(dry.num_frames(), path.rir.audio.num_frames())?;
        work = work.saturating_add(units)
            .saturating_add(path.wet_len.saturating_mul(hoa_channels));
        scratch = scratch.max(temporary);
        if let Some(pair) = &path.hrir {
            let l = convolution_len(path.wet_len, pair.left.len(), pair.delays[0])?;
            let r = convolution_len(path.wet_len, pair.right.len(), pair.delays[1])?;
            binaural = binaural.max(l).max(r);
            for taps in [pair.left.len(), pair.right.len()] {
                let (units, temporary) = convolution_cost(path.wet_len, taps)?;
                work = work.saturating_add(units);
                scratch = scratch.max(temporary);
            }
        }
    }
    work = work.saturating_add(frames.saturating_mul(hoa_channels).saturating_mul(hoa_channels));
    Ok((frames, binaural, work, scratch))
}

fn convolution_cost(signal: usize, taps: usize) -> Result<(usize, usize), String> {
    let direct = signal.saturating_mul(taps);
    if direct <= DIRECT_FIR_WORK {
        return Ok((direct, 0));
    }
    let fft = convolution_len(signal, taps, 0)?.next_power_of_two();
    let work = fft.saturating_mul(fft.ilog2() as usize)
        .saturating_mul(18).saturating_add(fft.saturating_mul(8));
    Ok((work, fft * 4 * std::mem::size_of::<f32>()))
}

fn convolve_measured(signal: &[f32], kernel: &[f32], out: &mut [f32]) -> Result<usize, String> {
    if signal.len().saturating_mul(kernel.len()) <= DIRECT_FIR_WORK {
        return convolve_fir_into(signal, kernel, out)
            .map_err(|error| format!("Native measured FIR: {error:?}"));
    }
    let extent = convolution_len(signal.len(), kernel.len(), 0)?;
    if out.len() < extent {
        return Err("Measured FFT convolution output is too short".into());
    }
    let bins = extent.next_power_of_two();
    let mut left = vec![0.0f32; bins * 2];
    let mut right = vec![0.0f32; bins * 2];
    for (bin, &sample) in left.chunks_exact_mut(2).zip(signal) {
        bin[0] = sample;
    }
    for (bin, &sample) in right.chunks_exact_mut(2).zip(kernel) {
        bin[0] = sample;
    }
    fft_radix2_interleaved_f32(&mut left, false).map_err(|error| format!("Native RIR FFT: {error:?}"))?;
    fft_radix2_interleaved_f32(&mut right, false).map_err(|error| format!("Native RIR FFT: {error:?}"))?;
    for (bin, response) in left.chunks_exact_mut(2).zip(right.chunks_exact(2)) {
        let real = f64::from(bin[0]);
        let imag = f64::from(bin[1]);
        let rr = f64::from(response[0]);
        let ri = f64::from(response[1]);
        bin[0] = (real * rr - imag * ri) as f32;
        bin[1] = (real * ri + imag * rr) as f32;
        if !bin[0].is_finite() || !bin[1].is_finite() {
            return Err("Measured RIR frequency product exceeded the finite F32 range".into());
        }
    }
    fft_radix2_interleaved_f32(&mut left, true).map_err(|error| format!("Native RIR inverse FFT: {error:?}"))?;
    for (target, bin) in out[..extent].iter_mut().zip(left.chunks_exact(2)) {
        if !bin[0].is_finite() {
            return Err("Measured RIR inverse FFT returned a nonfinite sample".into());
        }
        *target = bin[0];
    }
    Ok(extent)
}

fn analyze(path: &MeasuredPath, samples: &[f32], sample_rate: u32) -> Result<Value, String> {
    let response: Vec<f64> = samples[path.onset..].iter().copied().map(f64::from).collect();
    let report = analyze_room_impulse_response(&response, f64::from(sample_rate))
        .map_err(|error| format!("Measured RIR does not support a complete ISO 3382-1 decay fit: {error:?}"))?;
    let names = [
        ("edt_s", report.edt_s), ("t20_s", report.t20_s), ("t30_s", report.t30_s),
        ("c50_db", report.c50_db), ("c80_db", report.c80_db), ("d50_ratio", report.d50_ratio),
        ("centre_time_s", report.centre_time_s),
    ];
    if names.iter().any(|(_, value)| !value.is_finite()) {
        return Err("Measured RIR produced a nonfinite clarity or decay result".into());
    }
    let mut receipt: BTreeMap<_, _> = names.into_iter()
        .map(|(name, value)| (name.to_owned(), Value::F64(value))).collect();
    receipt.insert("rir_sha256".into(), Value::String(path.rir.digest.clone()));
    receipt.insert("rir_channel".into(), Value::U64(path.rir_channel as u64));
    receipt.insert("direct_onset_sample".into(), Value::U64(path.onset as u64));
    receipt.insert("analysis_band".into(), Value::String("broadband".into()));
    if let Some(measurement) = path.measurement {
        receipt.insert("hrir_measurement".into(), Value::U64(measurement as u64));
    }
    Ok(Value::Record(receipt))
}

fn render_binaural(wet: &[f32], pair: &EarPair, gain: f32, output: &mut AudioBuffer) -> Result<(), String> {
    let mut left = vec![0.0; convolution_len(wet.len(), pair.left.len(), pair.delays[0])?];
    let mut right = vec![0.0; convolution_len(wet.len(), pair.right.len(), pair.delays[1])?];
    let result = if wet.len().saturating_mul(pair.left.len().max(pair.right.len())) <= DIRECT_FIR_WORK {
        convolve_stereo_hrir_with_additional_delays_into(
            wet, &pair.left, &pair.right, pair.delays, &mut left, &mut right,
        ).map_err(|error| format!("Native measured HRIR convolution: {error:?}"))?
    } else {
        let l = convolve_measured(wet, &pair.left, &mut left[pair.delays[0]..])?;
        let r = convolve_measured(wet, &pair.right, &mut right[pair.delays[1]..])?;
        (l + pair.delays[0], r + pair.delays[1])
    };
    if result != (left.len(), right.len()) {
        return Err("Native binaural convolution returned inconsistent transfer lengths".into());
    }
    for (output, rendered) in output.channels.iter_mut().zip([left, right]) {
        for (target, sample) in output.iter_mut().zip(rendered) {
            *target += gain * sample;
            if !target.is_finite() {
                return Err("Measured binaural result exceeds its finite sample range".into());
            }
        }
    }
    Ok(())
}

fn seal_hoa(
    data: &[f32], order: usize, channels: usize, sample_rate: u32, position: [f64; 3],
    orientation: [f64; 4], dry_digest: &str, paths: &[MeasuredPath], hrir_digest: Option<&str>,
) -> Result<Vec<u8>, String> {
    let rate = RationalRate::new(u64::from(sample_rate), 1).map_err(|error| error.to_string())?;
    let clock = ClockSpec { scale: 0, epoch_seconds: 0, epoch_nanoseconds: 0, ticks_per_second: rate };
    let mut frame = FrameSpec::local_default();
    frame.origin_m = position;
    frame.rotation = QuaternionXyzw::from_xyzw(orientation).map_err(|error| error.to_string())?;
    let spec = HoaProfileSpec {
        order: order as u8,
        ordering: HoaOrdering::Acn,
        normalization: profiles::HoaNormalization::Sn3d,
        direction_convention: HoaDirectionConvention::ListenerToSource,
        units: "1",
        sample_rate: rate,
    };
    let lineage = serde_json::to_string(&serde_json::json!({
        "dry": dry_digest,
        "directional_rirs": paths.iter().map(|path| serde_json::json!({
            "sha256": path.rir.digest,
            "dry_channel": path.dry_channel, "rir_channel": path.rir_channel,
            "direct_onset_sample": path.onset, "world_direction_xyz": path.direction,
            "gain": path.gain, "hrir_measurement": path.measurement,
        })).collect::<Vec<_>>(),
        "hrir": hrir_digest,
    })).map_err(|error| error.to_string())?;
    let provenance = ProvenanceSpec {
        role: 0,
        source_digests: &[],
        algorithm: Some("urn:qualia:acoustics:measured-rir-hoa"),
        algorithm_version: Some("1"),
        instrument_uri: None,
        authority_receipt_digest: None,
        lineage: Some(&lineage),
    };
    let rows: Vec<&[f32]> = data.chunks_exact(channels).collect();
    let limits = ProfileLimits::default();
    let bytes = write_hoa_temporal_qac(&rows, &spec, &clock, &frame, &provenance, None, limits)
        .map_err(|error| format!("Native temporal HOA QAC seal: {error}"))?;
    let admitted = read_hoa_temporal_qac(&bytes, 1, limits)
        .map_err(|error| format!("Native temporal HOA QAC read-back: {error}"))?;
    if admitted.order != order as u8 || admitted.ordering != HoaOrdering::Acn
        || admitted.normalization != profiles::HoaNormalization::Sn3d
        || admitted.chunk.frame_count() != rows.len()
    {
        return Err("Native temporal QAC disagrees with its HOA frame contract".into());
    }
    Ok(bytes)
}

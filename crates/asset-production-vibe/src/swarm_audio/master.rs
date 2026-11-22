//! PCM16 verbatim FLAC archival and native BS.1770 metering and WSOLA.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use acoustics_daw_session::{AudioBuffer, FlexWarpMode, WsolaTimeStretch};
use qualia_core_db::audio::qac::spatial_dsp::{
    measure_bs1770_integrated_loudness, measure_true_peak_4x_f32, Bs1770ChannelRole,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    boolean, fields, integer, num, required, text, Args, AssetHost, TypedArtifact, Value,
};

const BLOCK_SIZE: usize = 4096;
const MAX_FRAMES: usize = 1_048_576;

impl AssetHost<'_> {
    pub(crate) fn master_opus_flac_and_stems(&mut self, args: &Args) -> Result<Value, String> {
        let op = text(required(args, "op")?)?;
        match op {
            "encode_flac" => fields(
                args,
                &[
                    "op",
                    "audio",
                    "quantize_pcm16",
                    "max_integrated_lufs",
                    "max_true_peak_dbtp",
                ],
            ),
            "time_stretch_wsola" => fields(
                args,
                &[
                    "op",
                    "audio",
                    "target_frames",
                    "max_integrated_lufs",
                    "max_true_peak_dbtp",
                ],
            ),
            _ => return Err(format!("Unsupported audio mastering operation: {op}")),
        }?;
        let TypedArtifact::AudioMultiChannel(source) =
            self.workspace.typed(required(args, "audio")?)?
        else {
            return Err("Mastering requires typed AudioMultiChannel input".into());
        };
        let source = Arc::clone(source);
        let frames = source.num_frames();
        let channels = source.num_channels();
        if !(1..=MAX_FRAMES).contains(&frames)
            || !(1..=2).contains(&channels)
            || !(8_000..=192_000).contains(&source.sample_rate)
            || source.sample_rate % 10 != 0
            || source
                .channels
                .iter()
                .any(|channel| channel.len() != frames)
        {
            return Err(
                "Mastering needs bounded equal-length mono or stereo audio at a supported rate"
                    .into(),
            );
        }
        for sample in source.channels.iter().flatten() {
            if !sample.is_finite() || !(-1.0..=1.0).contains(sample) {
                return Err("Mastering input samples must be finite and within [-1, 1]".into());
            }
        }
        let loudness_ceiling = optional_gate(args, "max_integrated_lufs", -70.0, 12.0)?;
        let peak_ceiling = optional_gate(args, "max_true_peak_dbtp", -80.0, 12.0)?;

        if op == "encode_flac" {
            let scalar_count = frames * channels;
            self.charge(scalar_count * 112, scalar_count * 12 + 65536)?;
            let quantize = boolean(args, "quantize_pcm16", false)?;
            let mut pcm = Vec::with_capacity(scalar_count);
            let mut encoded_samples = Vec::with_capacity(scalar_count);
            let mut pcm_hash = Sha256::new();
            let mut maximum_error = 0.0f32;
            for frame in 0..frames {
                for channel in &source.channels {
                    let value = channel[frame];
                    let scaled = value * 32768.0;
                    let grid_value = scaled.round().clamp(-32768.0, 32767.0);
                    if !quantize && scaled != grid_value {
                        return Err("FLAC without quantization requires samples exactly on the PCM16 lattice".into());
                    }
                    let integer = grid_value as i16;
                    let recovered = f32::from(integer) / 32768.0;
                    maximum_error = maximum_error.max((value - recovered).abs());
                    pcm_hash.update(integer.to_le_bytes());
                    pcm.push(integer);
                    encoded_samples.push(recovered);
                }
            }
            let metrics = measure(
                &encoded_samples,
                channels,
                source.sample_rate,
                loudness_ceiling,
                peak_ceiling,
            )?;
            let flac = encode_flac(&pcm, channels, source.sample_rate)?;
            let receipt = json!({
                "operation": op,
                "encoding": "flac_verbatim_pcm16",
                "pcm_sha256_interleaved_le": format!("{:x}", pcm_hash.finalize()),
                "resource_sha256": format!("{:x}", Sha256::digest(&flac)),
                "frames": frames,
                "channels": channels,
                "sample_rate": source.sample_rate,
                "maximum_quantization_error": maximum_error,
                "metrics": metrics,
            });
            let audio = self.encode("flac", flac)?;
            let receipt = self.encode(
                "json",
                serde_json::to_vec(&receipt).map_err(|e| e.to_string())?,
            )?;
            return Ok(Value::Record(Args::from([
                ("audio".into(), audio),
                ("receipt".into(), receipt),
            ])));
        }

        let target = integer(required(args, "target_frames")?, "target_frames")?;
        // A shared multichannel correlation search is needed to preserve stereo phase.
        if channels != 1
            || !(8_000..=48_000).contains(&source.sample_rate)
            || target > MAX_FRAMES
            || target < frames.div_ceil(2)
            || target > frames * 2
        {
            return Err(
                "WSOLA accepts bounded mono audio and a duration factor in [0.5, 2]".into(),
            );
        }
        let window = (f64::from(source.sample_rate) * 0.025).round() as usize;
        if frames < window || target < window {
            return Err("WSOLA needs at least one full analysis window at both durations".into());
        }
        let shift = window.div_ceil(4);
        let work = target.div_ceil(window / 2) * window * (shift * 2 + 1);
        self.charge(work + target * 96, (target + window) * 16)?;
        let stretched = WsolaTimeStretch::new(FlexWarpMode::Monophonic, source.sample_rate as f32)
            .process(&source.channels[0], target);
        if stretched.len() != target || stretched.iter().any(|sample| !sample.is_finite()) {
            return Err("Native WSOLA returned invalid samples or extent".into());
        }
        let metrics = measure(
            &stretched,
            1,
            source.sample_rate,
            loudness_ceiling,
            peak_ceiling,
        )?;
        let receipt = json!({
            "operation": op,
            "input_frames": frames,
            "frames": target,
            "channels": 1,
            "sample_rate": source.sample_rate,
            "metrics": metrics,
        });
        let audio =
            self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(AudioBuffer {
                sample_rate: source.sample_rate,
                channels: vec![stretched],
            })))?;
        let receipt = self.encode(
            "json",
            serde_json::to_vec(&receipt).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("audio".into(), audio),
            ("receipt".into(), receipt),
        ])))
    }
}

fn optional_gate(args: &Args, field: &str, low: f64, high: f64) -> Result<Option<f64>, String> {
    let value = args.get(field).map(num).transpose()?;
    if value.is_some_and(|v| !(low..=high).contains(&v)) {
        return Err(format!("{field} must be in [{low}, {high}]"));
    }
    Ok(value)
}

fn measure(
    samples: &[f32],
    channels: usize,
    sample_rate: u32,
    loudness_gate: Option<f64>,
    peak_gate: Option<f64>,
) -> Result<serde_json::Value, String> {
    let roles = [Bs1770ChannelRole::FrontOrCenter; 2];
    let hop = (sample_rate / 10) as usize;
    let frames = samples.len() / channels;
    let reading = if frames < 4 * hop {
        None
    } else {
        let mut scratch = vec![0.0; 1 + (frames - 4 * hop) / hop];
        Some(
            measure_bs1770_integrated_loudness(
                samples,
                &roles[..channels],
                sample_rate,
                &mut scratch,
            )
            .map_err(|error| format!("Native BS.1770 measurement failed: {error:?}"))?,
        )
    };
    let integrated_lufs = reading.as_ref().and_then(|reading| reading.integrated_lufs);
    if let Some(ceiling) = loudness_gate {
        let level = integrated_lufs
            .ok_or("The requested loudness gate requires a measurable programme level")?;
        if level > ceiling {
            return Err(format!(
                "Measured {level} LUFS exceeds the specified ceiling {ceiling}"
            ));
        }
    }
    let mut true_peak = None::<f64>;
    for channel in 0..channels {
        let planar;
        let lane = if channels == 1 {
            samples
        } else {
            planar = samples
                .iter()
                .skip(channel)
                .step_by(channels)
                .copied()
                .collect::<Vec<_>>();
            &planar
        };
        let reading = measure_true_peak_4x_f32(lane)
            .map_err(|error| format!("Native true-peak measurement failed: {error:?}"))?;
        if let Some(value) = reading.dbtp {
            true_peak = Some(true_peak.map_or(value, |peak| peak.max(value)));
        }
    }
    if let Some(ceiling) = peak_gate {
        if true_peak.is_some_and(|peak| peak > ceiling) {
            return Err(format!(
                "True-peak level exceeds the specified ceiling {ceiling} dBTP"
            ));
        }
    }
    Ok(json!({
        "loudness_standard": "ITU-R BS.1770-5",
        "integrated_lufs": integrated_lufs,
        "loudness_blocks": reading.map(|reading| reading.total_blocks).unwrap_or(0),
        "true_peak_dbtp": true_peak,
        "max_integrated_lufs": loudness_gate,
        "max_true_peak_dbtp": peak_gate,
    }))
}

// RFC 9639 §§8.2, 9.1, 9.2.4 and 9.3. Each native audio frame carries one
// byte-aligned signed verbatim block per channel and standard FLAC CRCs.
fn encode_flac(pcm: &[i16], channels: usize, sample_rate: u32) -> Result<Vec<u8>, String> {
    let frames = pcm.len() / channels;
    let frame_count = frames.div_ceil(BLOCK_SIZE);
    if frame_count > 2048 || frames == 0 || pcm.len() % channels != 0 {
        return Err("Unsupported FLAC PCM extent".into());
    }
    let mut out = Vec::with_capacity(42 + pcm.len() * 2 + frame_count * (16 + channels));
    out.extend_from_slice(b"fLaC\x80\0\0\x22");
    let block = BLOCK_SIZE as u16;
    out.extend(block.to_be_bytes());
    out.extend(block.to_be_bytes());
    out.extend([0; 6]); // Optional minimum and maximum frame sizes.
    let packed = (u64::from(sample_rate) << 44)
        | (((channels - 1) as u64) << 41)
        | (15 << 36)
        | frames as u64;
    out.extend(packed.to_be_bytes());
    out.extend([0; 16]); // Optional STREAMINFO MD5. The receipt records exact PCM SHA-256.

    for (index, data) in pcm.chunks(BLOCK_SIZE * channels).enumerate() {
        let start = out.len();
        let size = data.len() / channels;
        out.extend([0xff, 0xf8, 0x70, (((channels - 1) as u8) << 4) | 0x08]);
        if index < 128 {
            out.push(index as u8);
        } else {
            out.extend([(0xc0 | index >> 6) as u8, (0x80 | index & 0x3f) as u8]);
        }
        out.extend(((size - 1) as u16).to_be_bytes());
        let crc_header = crc8(&out[start..]);
        out.push(crc_header);
        for channel in 0..channels {
            out.push(2); // Verbatim with no wasted bits.
            for sample in data.iter().skip(channel).step_by(channels) {
                out.extend(sample.to_be_bytes());
            }
        }
        let crc_frame = crc16(&out[start..]);
        out.extend(crc_frame.to_be_bytes());
    }
    Ok(out)
}

fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x80 == 0 { 0 } else { 7 };
        }
    }
    crc
}

fn crc16(bytes: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &byte in bytes {
        crc ^= u16::from(byte) << 8;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x8000 == 0 { 0 } else { 0x8005 };
        }
    }
    crc
}

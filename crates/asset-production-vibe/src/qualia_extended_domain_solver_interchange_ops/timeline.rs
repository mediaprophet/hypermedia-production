//! Native timeline interchange inspection and monophonic MIDI transcription.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    count, fields, integer, number, required, Args, Artifact, AssetHost,
    TypedArtifact, Value,
};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use acoustics_daw_session::{codecs, dsp::{detect_pitch_yin, hz_to_midi}};
use chronicle_broadcast_containers::mxf::{ber_length, demux_mxf};
use std::collections::BTreeMap;

const PPQ: u16 = 960;
const MAX_SAMPLES: usize = 2_097_152;
const MAX_WINDOWS: usize = 4096;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "yin_midi" => yin_midi(host, args),
        "inspect_mxf" => inspect_mxf(host, args),
        _ => Err(format!("Unsupported timeline interchange operation: {op}")),
    }
}

#[derive(Clone, Copy)]
struct Note {
    start: usize,
    end: usize,
    pitch: u8,
    energy: f64,
    frequency: f64,
    windows: usize,
}

fn yin_midi(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["audio", "channel", "window_samples", "hop_samples", "minimum_hz",
        "maximum_hz", "threshold", "minimum_note_ms", "tempo_bpm"])?;
    let input = required(args, "audio")?;
    let channel = args.get("channel").map(|value| integer(value, "channel")).transpose()?.unwrap_or(0);
    let window = count(args, "window_samples", 2048, 4096)?;
    let hop = count(args, "hop_samples", window / 4, window)?;
    let threshold = number(args, "threshold", 0.15)?;
    let low = number(args, "minimum_hz", 60.0)?;
    let high = number(args, "maximum_hz", 1400.0)?;
    let duration_ms = number(args, "minimum_note_ms", 65.0)?;
    let bpm = number(args, "tempo_bpm", 120.0)?;
    if window < 64 || !(20.0..high).contains(&low) || high > 10_000.0
        || !(0.01..=0.5).contains(&threshold) || !(1.0..=1000.0).contains(&duration_ms)
        || !(20.0..=400.0).contains(&bpm)
    {
        return Err("Invalid YIN frequency, threshold, window, minimum note duration, or MIDI tempo".into());
    }
    let tempo_us = (60_000_000.0 / bpm).round() as u32;
    let (sample_rate, frames, work) = audio_shape(host, input, channel, window, hop, low)?;
    if high >= f64::from(sample_rate) / 2.0 {
        return Err("YIN maximum frequency must be below the selected audio channel's Nyquist rate".into());
    }
    host.charge(work, frames * 4 + 16 * 1024 * 1024 + MAX_WINDOWS * std::mem::size_of::<Note>())?;
    let samples = read_channel(host, input, channel, sample_rate, frames)?;
    if samples.iter().any(|sample| !sample.is_finite() || sample.abs() > 1.0) {
        return Err("Audio transcription requires finite normalized samples".into());
    }
    let mut notes: Vec<Note> = Vec::new();
    for (index, frame) in samples.windows(window).step_by(hop).enumerate() {
        let start = index * hop;
        let energy = frame.iter().map(|sample| f64::from(*sample).powi(2)).sum::<f64>() / frame.len() as f64;
        if energy <= 1e-8 {
            continue;
        }
        let Some(frequency) = detect_pitch_yin(frame, sample_rate as f32, low as f32, high as f32, threshold as f32)
            .filter(|value| (low..=high).contains(&f64::from(*value))) else { continue; };
        let pitch = hz_to_midi(frequency);
        let end = start + hop;
        if let Some(note) = notes.last_mut().filter(|note| note.end == start && note.pitch == pitch) {
            note.end = end;
            note.energy += energy;
            note.frequency += f64::from(frequency);
            note.windows += 1;
        } else {
            notes.push(Note { start, end, pitch, energy, frequency: f64::from(frequency), windows: 1 });
        }
    }
    let min_samples = (duration_ms * f64::from(sample_rate) / 1000.0).ceil() as usize;
    notes.retain(|note| note.end - note.start >= min_samples);
    let mut input_hash = blake3::Hasher::new();
    input_hash.update(&sample_rate.to_le_bytes());
    for sample in &samples {
        input_hash.update(&sample.to_le_bytes());
    }
    let source = format!("urn:qualia:audio:blake3:{}", input_hash.finalize().to_hex());
    let midi = smf(&notes, tempo_us, sample_rate)?;
    let mut receipt = ReceiptGraph::new("urn:qualia:timeline:midi-transcription:v1");
    let output = format!("urn:qualia:midi:blake3:{}", blake3::hash(&midi).to_hex());
    receipt.fact(&output, "urn:qualia:timeline:derivedFromAudio", &source)?;
    let bytes = receipt.seal(host.limits.resource_bytes.min(1024 * 1024))?;
    let observations = notes.iter().map(|note| Value::Record(BTreeMap::from([
        ("start_sample".into(), Value::U64(note.start as u64)),
        ("end_sample".into(), Value::U64(note.end as u64)),
        ("midi_pitch".into(), Value::U64(u64::from(note.pitch))),
        ("frequency_hz".into(), Value::F64(note.frequency / note.windows as f64)),
    ]))).collect();
    Ok(Value::Record(BTreeMap::from([
        ("midi".into(), host.encode("mid", midi)?),
        ("receipt".into(), host.encode("q42", bytes)?),
        ("notes".into(), Value::List(observations)),
    ])))
}

fn audio_shape(host: &AssetHost<'_>, input: &Value, channel: usize, window: usize, hop: usize, low: f64) -> Result<(u32, usize, usize), String> {
    let (rate, frames) = audio_metadata(host, input, channel)?;
    if !(8000..=192_000).contains(&rate) || !(window..=MAX_SAMPLES).contains(&frames) {
        return Err("Audio bounds do not permit the requested YIN window".into());
    }
    let windows = 1 + (frames - window) / hop;
    if windows > MAX_WINDOWS {
        return Err("Audio exceeds the YIN window count budget".into());
    }
    let tau = ((f64::from(rate) / low) as usize).min(window / 2);
    Ok((rate, frames, windows.saturating_mul(window).saturating_mul(tau)))
}

fn audio_metadata(host: &AssetHost<'_>, input: &Value, channel: usize) -> Result<(u32, usize), String> {
    if let Ok(TypedArtifact::AudioMultiChannel(audio)) = host.workspace.typed(input) {
        let samples = audio.channels.get(channel).ok_or("Transcription channel is missing")?;
        return Ok((audio.sample_rate, samples.len()));
    }
    if let Ok(Artifact::Audio { rate, samples }) = host.workspace.get(input) {
        if channel > 0 {
            return Err("This audio handle has only one channel".into());
        }
        return Ok((*rate, samples.len()));
    }
    wave_info(wave_bytes(host, input)?, channel)
}

fn wave_bytes<'a>(host: &'a AssetHost<'_>, input: &Value) -> Result<&'a [u8], String> {
    let resource = encoded_resource(host, input)?;
    if resource.kind != "wav" || resource.bytes.len() > 8 * 1024 * 1024 {
        return Err("Transcription requires an audio handle or bounded retained WAV".into());
    }
    Ok(&resource.bytes)
}

fn wave_info(bytes: &[u8], channel: usize) -> Result<(u32, usize), String> {
    let Some(header) = bytes.get(..12) else {
        return Err("Truncated retained WAV header".into());
    };
    let extent = word32(&header[4..8]) as usize;
    if &header[..4] != b"RIFF" || &header[8..] != b"WAVE" || extent.checked_add(8) != Some(bytes.len()) {
        return Err("Invalid retained RIFF/WAVE extent".into());
    }
    let mut chunks = &bytes[12..];
    let mut format = None;
    while !chunks.is_empty() {
        let chunk = chunks.get(..8).ok_or("Truncated WAV chunk header")?;
        let size = word32(&chunk[4..]) as usize;
        if size % 2 != 0 {
            return Err("The PCM WAV decoder requires even-sized RIFF chunks".into());
        }
        let tail = &chunks[8..];
        let body = tail.get(..size).ok_or("Truncated WAV chunk payload")?;
        match &chunk[..4] {
            b"fmt " if format.is_none() && body.len() >= 16 => {
                let tag = u16::from_le_bytes([body[0], body[1]]);
                let channels = usize::from(u16::from_le_bytes([body[2], body[3]]));
                let rate = word32(&body[4..8]);
                let alignment = usize::from(u16::from_le_bytes([body[12], body[13]]));
                let bits = usize::from(u16::from_le_bytes([body[14], body[15]]));
                if !matches!((tag, bits), (1, 16) | (3, 32)) || !(1..=16).contains(&channels)
                    || alignment != channels * bits / 8 || channel >= channels
                {
                    return Err("Expected admitted PCM16 or float32 WAV with the requested channel".into());
                }
                format = Some((rate, alignment));
            }
            b"fmt " => return Err("Malformed or repeated WAV format chunk".into()),
            b"data" => {
                let (rate, alignment) = format.ok_or("WAV data is missing its format header")?;
                if body.len() % alignment != 0 {
                    return Err("WAV data ends within a sample frame".into());
                }
                return Ok((rate, body.len() / alignment));
            }
            _ => {}
        }
        chunks = &tail[size..];
    }
    Err("WAV contains no sample data".into())
}

fn word32(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}

fn read_channel(host: &AssetHost<'_>, input: &Value, channel: usize, rate: u32, count: usize) -> Result<Vec<f32>, String> {
    let samples = match host.workspace.typed(input) {
        Ok(TypedArtifact::AudioMultiChannel(audio)) => audio.channels[channel].clone(),
        _ => match host.workspace.get(input) {
            Ok(Artifact::Audio { samples, .. }) => samples.clone(),
            _ => {
                let mut audio = codecs::decode_wav(wave_bytes(host, input)?)
                    .ok_or("Invalid retained PCM WAV")?;
                if audio.sample_rate != rate || channel >= audio.channels.len() {
                    return Err("The selected WAV channel changed during admission".into());
                }
                audio.channels.swap_remove(channel)
            }
        },
    };
    if samples.len() != count {
        return Err("Audio sample geometry changed during admission".into());
    }
    Ok(samples)
}

fn smf(notes: &[Note], tempo_us: u32, sample_rate: u32) -> Result<Vec<u8>, String> {
    let mut output = b"MThd\0\0\0\x06\0\x01\0\x02".to_vec();
    output.extend_from_slice(&PPQ.to_be_bytes());
    let [_, h, m, l] = tempo_us.to_be_bytes();
    append_track(&mut output, &[0, 0xff, 0x51, 3, h, m, l, 0, 0xff, 0x2f, 0])?;
    let mut track = Vec::with_capacity(notes.len() * 14 + 4);
    let ticks = |sample: usize| {
        let denominator = u64::from(sample_rate) * u64::from(tempo_us);
        (sample as u64 * 1_000_000 * u64::from(PPQ) + denominator / 2) / denominator
    };
    let mut last = 0;
    for note in notes {
        let start = ticks(note.start).max(last);
        let end = ticks(note.end).max(start + 1);
        let velocity = (127.0 * (note.energy / note.windows as f64).sqrt()).round().clamp(1.0, 127.0) as u8;
        delta(&mut track, start - last)?;
        track.extend_from_slice(&[0x90, note.pitch, velocity]);
        delta(&mut track, end - start)?;
        track.extend_from_slice(&[0x80, note.pitch, 0]);
        last = end;
    }
    track.extend_from_slice(&[0, 0xff, 0x2f, 0]);
    append_track(&mut output, &track)?;
    Ok(output)
}

fn delta(track: &mut Vec<u8>, value: u64) -> Result<(), String> {
    if value > 0x0fff_ffff {
        return Err("MIDI event delta exceeds the SMF variable-length range".into());
    }
    for shift in [21, 14, 7, 0] {
        if shift == 0 {
            track.push((value & 0x7f) as u8);
        } else if value >> shift != 0 {
            track.push((value >> shift & 0x7f) as u8 | 0x80);
        }
    }
    Ok(())
}

fn append_track(output: &mut Vec<u8>, bytes: &[u8]) -> Result<(), String> {
    let len = u32::try_from(bytes.len()).map_err(|_| "MIDI track exceeds the SMF extent")?;
    output.extend_from_slice(b"MTrk");
    output.extend_from_slice(&len.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn inspect_mxf(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["source"])?;
    let input = required(args, "source")?;
    let resource = encoded_resource(host, input)?;
    let size = resource.bytes.len();
    if resource.kind != "mxf" || size > 16 * 1024 * 1024 {
        return Err("MXF inspection requires a bounded retained MXF resource".into());
    }
    host.charge(size, size * 2)?;
    let bytes = &encoded_resource(host, input)?.bytes;
    let mut rest = bytes.as_slice();
    let mut klv_count = 0;
    while !rest.is_empty() {
        let body = rest.get(16..).ok_or("Truncated MXF key")?;
        let (len, width) = ber_length(body).ok_or("Malformed MXF BER extent")?;
        let len = usize::try_from(len).map_err(|_| "MXF KLV length exceeds host addressing")?;
        let end = width.checked_add(len).ok_or("MXF KLV extent overflow")?;
        rest = body.get(end..).ok_or("Truncated MXF KLV payload")?;
        klv_count += 1;
        if klv_count > 65_536 {
            return Err("MXF KLV count exceeds inspection limit".into());
        }
    }
    let parsed = demux_mxf(bytes)?;
    if parsed.partitions.is_empty() {
        return Err("Verified MXF KLV stream contains no partitions".into());
    }
    let packets = parsed.packets.iter().map(|packet| Value::Record(BTreeMap::from([
        ("offset".into(), Value::U64(packet.offset)),
        ("bytes".into(), Value::U64(packet.payload.len() as u64)),
    ]))).collect();
    Ok(Value::Record(BTreeMap::from([
        ("operational_pattern".into(), Value::String(format!("{:?}", parsed.operational_pattern).into())),
        ("partition_count".into(), Value::U64(parsed.partitions.len() as u64)),
        ("essence_packets".into(), Value::List(packets)),
    ])))
}

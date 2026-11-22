//! Articulatory tube model and focused speaker-array drive authoring.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::f64::consts::PI;
use std::sync::Arc;

use acoustics_daw_session::{AudioBuffer, FdnReverb};
use qualia_core_db::audio::qac::signal::{
    writer::{
        PcmDocument, PcmQuantity, PcmSamples, Provenance, ProvenanceRole, SignalRelation,
        WriteBuffers, WriteOptions,
    },
    Rate, Timing,
};
use qualia_core_db::q42::field_semantics::{NodeState, Scratch};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{
    count, fields, list, num, number, record, required, vector, Args, AssetHost, TypedArtifact,
    Value,
};

const SOUND_SPEED: f64 = 343.0;
const MAX_SECTIONS: usize = 64;
const MAX_SPEAKERS: usize = 16;
const MAX_FRAMES: usize = 384_000;

struct Keyframe {
    time_s: f64,
    f0_hz: f64,
    level: f64,
    open_quotient: f64,
    areas_cm2: Vec<f64>,
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_articulatory_speech_and_holosonic(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "tract_keyframes",
                "speakers_m",
                "focus_m",
                "sample_rate",
                "fdn",
            ],
        )?;
        let sample_rate = count(args, "sample_rate", 48_000, 96_000)? as u32;
        if sample_rate < 24_000 {
            return Err("Articulatory model requires a rate of at least 24000 Hz".into());
        }
        let keyframes = read_keyframes(required(args, "tract_keyframes")?, sample_rate)?;
        let sections = keyframes[0].areas_cm2.len();
        let tract_length_m = sections as f64 * SOUND_SPEED / f64::from(sample_rate);
        if !(0.08..=0.28).contains(&tract_length_m) {
            return Err(
                "The tract section count and rate imply an out-of-range model length".into(),
            );
        }
        let focus = position(required(args, "focus_m")?)?;
        let speakers = list(required(args, "speakers_m")?)?;
        if !(2..=MAX_SPEAKERS).contains(&speakers.len()) {
            return Err("Focused rendering requires 2 to 16 speaker coordinates".into());
        }
        let speakers = speakers
            .iter()
            .map(position)
            .collect::<Result<Vec<_>, _>>()?;
        let distance = |a: [f64; 3], b: [f64; 3]| {
            a.into_iter()
                .zip(b)
                .map(|(x, y)| (x - y).powi(2))
                .sum::<f64>()
                .sqrt()
        };
        for (index, &speaker) in speakers.iter().enumerate() {
            if speakers[..index]
                .iter()
                .any(|&other| distance(speaker, other) < 0.001)
            {
                return Err("Focused rendering speaker coordinates must be distinct".into());
            }
        }
        let distances = speakers
            .iter()
            .map(|&point| distance(point, focus))
            .collect::<Vec<_>>();
        if distances.iter().any(|&d| !(0.1..=200.0).contains(&d)) {
            return Err("Speaker-to-focus distances must be within 0.1 to 200 metres".into());
        }
        let furthest = distances.iter().copied().fold(0.0, f64::max);
        let delays = distances
            .iter()
            .map(|distance| (furthest - distance) * f64::from(sample_rate) / SOUND_SPEED)
            .collect::<Vec<_>>();
        let norm = distances.iter().map(|d| d * d).sum::<f64>().sqrt();
        let gains = distances
            .iter()
            .map(|d| (d / norm) as f32)
            .collect::<Vec<_>>();
        let fdn = args
            .get("fdn")
            .map(record)
            .transpose()?
            .map(|spec| {
                fields(spec, &["feedback", "wet_fraction", "tail_s"])?;
                let feedback = number(spec, "feedback", 0.75)?;
                let wet = number(spec, "wet_fraction", 0.2)?;
                let tail = number(spec, "tail_s", 0.5)?;
                if !(0.0..=0.9).contains(&feedback)
                    || !(0.0..=1.0).contains(&wet)
                    || !(0.0..=2.0).contains(&tail)
                {
                    return Err(
                        "FDN feedback, wet fraction or duration exceeds the model limits".into(),
                    );
                }
                Ok::<_, String>((
                    feedback as f32,
                    wet as f32,
                    (tail * f64::from(sample_rate)).ceil() as usize,
                ))
            })
            .transpose()?;
        let voice_frames =
            (keyframes.last().unwrap().time_s * f64::from(sample_rate)).ceil() as usize;
        let tail = fdn.map_or(0, |(_, _, frames)| frames);
        let mono_frames = voice_frames + tail;
        let frames = mono_frames + delays.iter().copied().fold(0.0, f64::max).ceil() as usize + 1;
        let scalar_count = frames * speakers.len();
        let pcm_bytes = scalar_count * 4;
        let file_capacity = pcm_bytes + 32_768;
        if frames > MAX_FRAMES || pcm_bytes > 16 << 20 || file_capacity > self.limits.resource_bytes
        {
            return Err("Articulatory array output exceeds its QAC or timeline budget".into());
        }
        self.charge(
            voice_frames * sections * 28
                + mono_frames * (if fdn.is_some() { 220 } else { 2 })
                + scalar_count * 24,
            5 * file_capacity + mono_frames * 4 + 512 * 1024,
        )?;
        let mut voice = render_tube(&keyframes, sample_rate, voice_frames, mono_frames)?;
        if let Some((feedback, wet, _)) = fdn {
            let mut effect = FdnReverb::new(sample_rate as f32);
            effect.room_size = feedback;
            for sample in &mut voice {
                let (left, right) = effect.process_sample(*sample);
                *sample = *sample * (1.0 - wet) + 0.5 * (left + right) * wet;
            }
        }
        if voice.iter().any(|sample| !sample.is_finite()) {
            return Err("Articulatory model produced a non-finite signal".into());
        }
        let max_vol = voice.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        // Preserve relative drive level and reserve common headroom around all channels.
        let headroom = 0.95 / max_vol.max(0.95);
        let mut audio = AudioBuffer::new(sample_rate, speakers.len(), frames);
        for ((channel, &delay), &gain) in audio.channels.iter_mut().zip(&delays).zip(&gains) {
            let integer = delay.floor() as usize;
            let phase = (delay - integer as f64) as f32;
            for (i, &sample) in voice.iter().enumerate() {
                let sample = sample * gain * headroom;
                channel[i + integer] += sample * (1.0 - phase);
                channel[i + integer + 1] += sample * phase;
            }
        }
        let mut interleaved = Vec::with_capacity(scalar_count);
        for frame in 0..frames {
            for channel in &audio.channels {
                interleaved.push(channel[frame]);
            }
        }
        let lineage = json!({
            "model": "lossy-oral-tube-with-fractional-delay-array",
            "tract_length_m": tract_length_m,
            "sample_rate": sample_rate,
            "keyframes": keyframes.iter().map(|key| json!({"time_s": key.time_s, "f0_hz": key.f0_hz,
                "level": key.level, "open_quotient": key.open_quotient, "areas_cm2": key.areas_cm2})).collect::<Vec<_>>(),
            "focus_m": focus, "speakers_m": speakers,
            "speaker_delay_samples": delays, "speaker_gain": gains,
            "sound_speed_m_s": SOUND_SPEED, "shared_headroom": headroom,
            "fdn_feedback_wet_fraction_tail_samples": fdn,
        });
        let lineage_bytes = serde_json::to_vec(&lineage).map_err(|e| e.to_string())?;
        let lineage_hash = format!("urn:sha256:{:x}", Sha256::digest(&lineage_bytes));
        let channel_order = (0..speakers.len())
            .map(|index| format!("speaker-{}", index + 1))
            .collect::<Vec<_>>()
            .join(",");
        let qac = seal_pcm(
            &interleaved,
            sample_rate,
            speakers.len(),
            file_capacity,
            PcmModel {
                channel_order: &channel_order,
                purpose: "articulatory-focused-speaker-drive",
                algorithm: "urn:qualia:audio:time-varying-oral-tube-array",
                lineage: &lineage_hash,
            },
        )?;
        let mut lineage = lineage;
        lineage["lineage_iri"] = json!(lineage_hash);
        lineage["qac_sha256"] = json!(format!("{:x}", Sha256::digest(&qac)));
        lineage["sample_quantity"] = json!("digital_sample");
        let receipt = serde_json::to_vec(&lineage).map_err(|e| e.to_string())?;
        let audio = self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(audio)))?;
        let qac = self.encode("qac", qac)?;
        let receipt = self.encode("json", receipt)?;
        Ok(Value::Record(Args::from([
            ("audio".into(), audio),
            ("qac".into(), qac),
            ("receipt".into(), receipt),
        ])))
    }
}

fn read_keyframes(value: &Value, sample_rate: u32) -> Result<Vec<Keyframe>, String> {
    let rows = list(value)?;
    if !(2..=128).contains(&rows.len()) {
        return Err("Expected 2 to 128 articulatory keyframes".into());
    }
    let mut keyframes = Vec::<Keyframe>::with_capacity(rows.len());
    for row in rows {
        let row = record(row)?;
        fields(
            row,
            &["time_s", "f0_hz", "level", "open_quotient", "areas_cm2"],
        )?;
        let time_s = num(required(row, "time_s")?)?;
        let f0_hz = num(required(row, "f0_hz")?)?;
        let level = num(required(row, "level")?)?;
        let open_quotient = num(required(row, "open_quotient")?)?;
        let areas = list(required(row, "areas_cm2")?)?;
        if !(0.0..=4.0).contains(&time_s)
            || !(55.0..=550.0).contains(&f0_hz)
            || !(0.0..=1.0).contains(&level)
            || !(0.25..=0.85).contains(&open_quotient)
            || !(8..=MAX_SECTIONS).contains(&areas.len())
        {
            return Err("An articulatory keyframe exceeds its physical or signal bounds".into());
        }
        if let Some(previous) = keyframes.last() {
            if time_s - previous.time_s < 1.0 / f64::from(sample_rate)
                || areas.len() != previous.areas_cm2.len()
            {
                return Err(
                    "Articulatory keyframe times must ascend and tube topology must stay fixed"
                        .into(),
                );
            }
        } else if time_s != 0.0 {
            return Err("Articulatory trajectory must begin at zero".into());
        }
        let areas_cm2 = areas
            .iter()
            .map(|v| {
                let area = num(v)?;
                if !(0.02..=12.0).contains(&area) {
                    return Err("Each tract area must be in [0.02, 12] square centimetres".into());
                }
                Ok(area)
            })
            .collect::<Result<Vec<_>, String>>()?;
        keyframes.push(Keyframe {
            time_s,
            f0_hz,
            level,
            open_quotient,
            areas_cm2,
        });
    }
    Ok(keyframes)
}

fn position(value: &Value) -> Result<[f64; 3], String> {
    let vector = vector::<3>(value)?;
    if vector.iter().any(|x| x.abs() > 500.0) {
        return Err("Array coordinates must be within the bounded metric frame".into());
    }
    Ok(vector)
}

fn render_tube(
    keyframes: &[Keyframe],
    rate: u32,
    voice_frames: usize,
    total: usize,
) -> Result<Vec<f32>, String> {
    let sections = keyframes[0].areas_cm2.len();
    let mut right = [0.0f64; MAX_SECTIONS];
    let mut left = [0.0f64; MAX_SECTIONS];
    let mut next_right = right;
    let mut next_left = left;
    let mut output = vec![0.0f32; total];
    let mut index = 0;
    let mut phase = 0.0f64;
    let mut previous_lips = 0.0f64;
    for (i, sample) in output[..voice_frames].iter_mut().enumerate() {
        let t = i as f64 / f64::from(rate);
        while index + 2 < keyframes.len() && t >= keyframes[index + 1].time_s {
            index += 1;
        }
        let a = &keyframes[index];
        let b = &keyframes[index + 1];
        let fraction = ((t - a.time_s) / (b.time_s - a.time_s)).clamp(0.0, 1.0);
        let mix = |a: f64, b: f64| a + (b - a) * fraction;
        let period = mix(a.open_quotient, b.open_quotient);
        let pulse = if phase < period {
            0.5 * (1.0 - (2.0 * PI * phase / period).cos())
        } else {
            0.0
        };
        phase = (phase + mix(a.f0_hz, b.f0_hz) / f64::from(rate)).fract();
        next_right[0] = 0.88 * left[0] + pulse * mix(a.level, b.level);
        for j in 0..sections - 1 {
            let first = mix(a.areas_cm2[j], b.areas_cm2[j]);
            let second = mix(a.areas_cm2[j + 1], b.areas_cm2[j + 1]);
            let reflection = (first - second) / (first + second);
            next_left[j] = 0.992 * (reflection * right[j] + (1.0 - reflection) * left[j + 1]);
            next_right[j + 1] = 0.992 * ((1.0 + reflection) * right[j] - reflection * left[j + 1]);
        }
        let lips = right[sections - 1];
        next_left[sections - 1] = -0.82 * lips;
        // A differentiator models first-order radiation from the opening.
        let value = (lips - previous_lips) as f32;
        if !value.is_finite() {
            return Err("Articulatory tube propagation mode is unstable".into());
        }
        *sample = value;
        previous_lips = lips;
        std::mem::swap(&mut right, &mut next_right);
        std::mem::swap(&mut left, &mut next_left);
    }
    Ok(output)
}

pub(super) struct PcmModel<'a> {
    pub(super) channel_order: &'a str,
    pub(super) purpose: &'a str,
    pub(super) algorithm: &'a str,
    pub(super) lineage: &'a str,
}

pub(super) fn seal_pcm(
    samples: &[f32],
    sample_rate: u32,
    channels: usize,
    capacity: usize,
    model: PcmModel<'_>,
) -> Result<Vec<u8>, String> {
    let rate = Rate {
        numerator: u64::from(sample_rate),
        denominator: 1,
    };
    let doc = PcmDocument {
        samples: PcmSamples::F32(samples),
        timing: Timing {
            scale: 0,
            epoch_seconds: 0,
            epoch_nanoseconds: 0,
            ticks: rate,
            samples: rate,
            start_tick: 0,
        },
        stream_id: 1,
        channels: channels as u32,
        channel_order: model.channel_order,
        purpose: model.purpose,
        quantity: PcmQuantity::DigitalSample,
        relation: SignalRelation::DigitalUnrelatedToPressure,
        units_per_stored_sample: 1.0,
        stored_offset: 0.0,
        provenance: Provenance {
            role: ProvenanceRole::Model,
            algorithm: Some(model.algorithm),
            algorithm_version: Some("1"),
            instrument_uri: None,
            lineage: Some(model.lineage),
        },
        original: None,
    };
    let mut metadata = [0u8; 16_384];
    let mut payload = vec![0u8; 4 * samples.len()];
    let mut staging = vec![0u8; capacity];
    let mut output = vec![0u8; capacity];
    let mut meta_nodes = [NodeState::default(); 32];
    let mut entry_nodes = [NodeState::default(); 8];
    let size = doc
        .encode_into(
            WriteBuffers {
                metadata: &mut metadata,
                payload: &mut payload,
                file: &mut staging,
            },
            &mut Scratch {
                metadata: &mut meta_nodes,
                entries: &mut entry_nodes,
            },
            &mut output,
            WriteOptions::default(),
        )
        .map_err(|e| format!("Native modeled audio QAC seal failed: {e:?}"))?;
    output.truncate(size);
    Ok(output)
}

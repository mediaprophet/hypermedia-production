//! WP-38 native studio adapters for authored motion.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::BTreeMap,
    sync::{atomic::Ordering, Arc},
};

use qualia_core_db::{
    bundle::semantic_model::{resource_identity_meta, SemanticModelBuilder},
    container_10d::{self as ten, skeletal_section::SKL2_NONE_INDEX},
    etalon_render::skeletal::{sample_pose_into, valid_motor, Clip, Key, Motor, Track, MAX_KEYS},
    q_hash,
    resolver::pack_typed_literal,
    NQuin,
};
use sha2::{Digest, Sha256};

use crate::{
    artifact::AnimClip, count, fields, integer, list, num, number,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text, Args,
    AssetHost, TypedArtifact, Value,
};

const MAX_PHRASES: usize = 64;
const MAX_BEATS: usize = 512;
const MAX_RECEIPT_BYTES: usize = 2 * 1024 * 1024;
const MAX_SOURCE_CBLD_BYTES: usize = 4 * 1024 * 1024;
const MAX_TIME_S: f64 = 1200.0;

macro_rules! laban {
    ($term:literal) => {
        concat!("https://qualiadb.org/ontology/choreography#", $term)
    };
}

#[derive(Clone, Copy)]
struct Effort {
    name: &'static str,
    strong: bool,
    sudden: bool,
    direct: bool,
}

impl Effort {
    fn read(name: &str) -> Result<Self, String> {
        let (name, strong, sudden, direct) = match name {
            "punch" => ("punch", true, true, true),
            "slash" => ("slash", true, true, false),
            "dab" => ("dab", false, true, true),
            "flick" => ("flick", false, true, false),
            "press" => ("press", true, false, true),
            "wring" => ("wring", true, false, false),
            "glide" => ("glide", false, false, true),
            "float" => ("float", false, false, false),
            _ => return Err("Unknown Laban basic effort action".into()),
        };
        Ok(Self {
            name,
            strong,
            sudden,
            direct,
        })
    }

    fn phase(self, u: f64, flow: f64) -> f64 {
        let free = (1.0 + flow) * 0.5;
        let eased = (1.0 - free) * u * u * (3.0 - 2.0 * u) + free * u;
        let accent = if self.sudden { 0.6 } else { -0.15 } + if self.strong { 0.15 } else { -0.1 };
        eased + accent * eased * (1.0 - eased)
    }
}

struct Phrase {
    source: [f64; 2],
    beats: [usize; 2],
    effort: Effort,
    flow: f64,
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_laban_choreography(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "rig",
                "clip_iri",
                "beat_times_s",
                "phrases",
                "samples_per_phrase",
                "max_root_path_delta_m",
            ],
        )?;
        let source = required(args, "source")?;
        let rig = required(args, "rig")?;
        let clip_iri = text(required(args, "clip_iri")?)?;
        let clip_id = iri_hash(clip_iri)?;
        let beats = beat_times(required(args, "beat_times_s")?)?;
        let phrases = phrases(required(args, "phrases")?, beats.len())?;
        let intervals = count(args, "samples_per_phrase", 8, 32)?;
        let frame_count = 1 + intervals * phrases.len();
        let max_delta = number(args, "max_root_path_delta_m", 0.5)?;
        if !(0.0..=5.0).contains(&max_delta) {
            return Err("Laban max_root_path_delta_m must be in [0, 5]".into());
        }
        let size = source_bytes(self, source)?.len();
        self.charge(
            size.saturating_mul(3)
                .saturating_add(frame_count * MAX_KEYS),
            MAX_RECEIPT_BYTES + MAX_SOURCE_CBLD_BYTES,
        )?;
        let (clip, digest, realized) = {
            let bytes = source_bytes(self, source)?;
            let digest = format!("{:x}", Sha256::digest(bytes));
            verify_clip_iri(bytes, clip_iri, clip_id)?;
            let section =
                ten::skeletal_container::read_skeletal_section(bytes).map_err(|error| {
                    format!("Laban source requires native SKL1 animation: {error:?}")
                })?;
            let TypedArtifact::SkeletalRig(rig) = self.workspace.typed(rig)? else {
                return Err("Laban choreography requires an authored typed SKL2 rig".into());
            };
            rig.validate()
                .map_err(|error| format!("Laban SKL2 rig: {error:?}"))?;
            let counts = section.counts();
            if counts.joints == 0
                || counts.joints != rig.joints.len()
                || counts.joints * frame_count > MAX_KEYS
            {
                return Err("Laban source or result exceeds its native joint/key bounds".into());
            }
            let joints = (0..counts.joints)
                .map(|i| section.joint(i).ok_or("Invalid Laban SKL1 joint"))
                .collect::<Result<Vec<_>, _>>()?;
            for (slot, (joint, descriptor)) in joints.iter().zip(&rig.joints).enumerate() {
                if section.joint_id(slot) != Some(descriptor.joint_id)
                    || joint.parent.unwrap_or(SKL2_NONE_INDEX) != descriptor.parent_index
                {
                    return Err("Laban source and SKL2 rig disagree on ordered joint identities or topology".into());
                }
            }
            let mut roots = joints
                .iter()
                .enumerate()
                .filter(|(_, joint)| joint.parent.is_none());
            let root = roots.next().ok_or("Laban source has no root joint")?.0;
            if roots.next().is_some() {
                return Err("Laban source must have one authored actor root".into());
            }
            let mut clips = (0..counts.clips)
                .filter_map(|i| section.clip(i))
                .filter(|clip| clip.id == clip_id);
            let selected = clips
                .next()
                .ok_or("Laban clip IRI is absent from the sealed SKL1 clip bank")?;
            let last = phrases
                .last()
                .ok_or("Laban requires at least one authored phrase")?;
            if clips.next().is_some()
                || selected.looping
                || (selected.duration - last.source[1]).abs() > 1e-9 * selected.duration
            {
                return Err(
                    "Laban phrases must cover one unique nonlooping source clip exactly".into(),
                );
            }
            let keys = (0..counts.keys)
                .map(|index| section.key(index).ok_or("Invalid source animation key"))
                .collect::<Result<Vec<_>, _>>()?;
            let tracks = (0..counts.tracks)
                .map(|index| section.track(index).ok_or("Invalid source animation track"))
                .collect::<Result<Vec<_>, _>>()?;
            let first = selected.first_track as usize;
            let end = first
                .checked_add(selected.track_count as usize)
                .ok_or("Laban clip track extent overflow")?;
            let source_clip = Clip {
                duration: selected.duration,
                looping: false,
                keys: &keys,
                tracks: tracks
                    .get(first..end)
                    .ok_or("Laban clip track extent is invalid")?,
            };
            let mut pose = vec![Motor::identity(); joints.len()];
            let mut staged = vec![Vec::with_capacity(frame_count); joints.len()];
            let mut realized = 0.0_f64;
            for (index, phrase) in phrases.iter().enumerate() {
                if self.cancel.load(Ordering::Acquire) {
                    return Err("Production cancelled".into());
                }
                let source_end = if index + 1 == phrases.len() {
                    selected.duration
                } else {
                    phrase.source[1]
                };
                let read = |time, output: &mut [Motor]| {
                    sample_pose_into(&joints, source_clip, time, None, 1.0, output)
                        .map_err(|error| format!("Native Laban pose sample: {error:?}"))
                };
                read(phrase.source[0], &mut pose)?;
                let start_root = pose[root];
                read(source_end, &mut pose)?;
                let end_root = pose[root];
                let [t0, t1] = phrase.beats.map(|beat| beats[beat] - beats[0]);
                let first_step = if index == 0 { 0 } else { 1 };
                for step in first_step..=intervals {
                    let u = step as f64 / intervals as f64;
                    let progress = phrase.effort.phase(u, phrase.flow);
                    let source_time = if step == intervals {
                        source_end
                    } else {
                        phrase.source[0] + progress * (source_end - phrase.source[0])
                    };
                    read(source_time, &mut pose)?;
                    if step > 0 && step < intervals {
                        let (rotation, authored) = pose[root].to_rotation_translation();
                        let (_, geodesic) = Motor::sclerp(&start_root, &end_root, progress)
                            .to_rotation_translation();
                        let shape = if phrase.effort.direct { -0.55 } else { 0.2 };
                        let factor = shape * (u * std::f64::consts::PI).sin();
                        let offset: [f64; 3] =
                            std::array::from_fn(|i| (authored[i] - geodesic[i]) * factor);
                        let residual = offset[0].hypot(offset[1]).hypot(offset[2]);
                        if residual > max_delta {
                            return Err(format!("Laban phrase {index} root displacement {residual:e} m exceeds {max_delta:e} m"));
                        }
                        realized = realized.max(residual);
                        pose[root] = Motor::from_rotation_translation(
                            rotation,
                            std::array::from_fn(|i| authored[i] + offset[i]),
                        );
                        if !valid_motor(&pose[root]) {
                            return Err("Invalid styled Laban root motor".into());
                        }
                    }
                    let time = if step == intervals {
                        t1
                    } else {
                        t0 + u * (t1 - t0)
                    };
                    for (track, &local) in staged.iter_mut().zip(&pose) {
                        track.push(Key { time, local });
                    }
                }
            }
            let mut motor_keys = Vec::with_capacity(frame_count * joints.len());
            let mut tracks = Vec::with_capacity(joints.len());
            for (joint, keys) in staged.into_iter().enumerate() {
                tracks.push(Track {
                    joint: joint as u16,
                    first_key: motor_keys.len() as u32,
                    key_count: keys.len() as u16,
                });
                motor_keys.extend(keys);
            }
            let clip = AnimClip {
                duration_seconds: beats[last.beats[1]] - beats[0],
                looping: false,
                tracks,
                motor_keys,
                morph_keys: Vec::new(),
            };
            (clip, digest, realized)
        };
        let receipt = receipt(clip_iri, &digest, &beats, &phrases, intervals, realized)?;
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(clip)))?;
        let receipt = self.admit_encoded("q42", receipt)?;
        Ok(Value::Record(Args::from([
            ("clip".into(), clip),
            ("receipt".into(), receipt),
            ("max_root_path_delta_m".into(), Value::F64(realized)),
            ("frame_count".into(), Value::U64(frame_count as u64)),
        ])))
    }
}

fn source_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    let source = encoded_resource(host, value)?;
    if source.kind != "10d"
        || source.bytes.len()
            > host
                .limits
                .resource_bytes
                .min(ten::GRAPHICS_ASSET_RING_BYTES)
    {
        return Err("Laban requires a bounded sealed 10D clip bank".into());
    }
    let header =
        ten::Container10dHeader::parse(&source.bytes).map_err(|error| error.to_string())?;
    if ten::compute_whole_file_crc32c(&source.bytes) != header.header_crc32c {
        return Err("Laban source checksum mismatch".into());
    }
    Ok(&source.bytes)
}

fn verify_clip_iri(bytes: &[u8], iri: &str, hash: u64) -> Result<(), String> {
    let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let frame = ten::TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    let mut matched = false;
    let mut examined = 0usize;
    for entry in frame.payloads_of_kind(ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        let entry = entry.map_err(|error| error.to_string())?;
        examined = examined
            .checked_add(entry.bytes.len())
            .filter(|&n| n <= MAX_SOURCE_CBLD_BYTES)
            .ok_or("Laban source local CBLD exceeds the lexical budget")?;
        let payload =
            ten::decode_symbolic_rdf_cbor_ld(entry.bytes).map_err(|error| error.to_string())?;
        for term in payload.lexicon {
            if term.iri_utf8 == iri || term.term_hash == hash {
                if term.iri_utf8 != iri || term.term_hash != hash {
                    return Err("Laban source contains a conflicting clip IRI binding".into());
                }
                matched = true;
            }
        }
    }
    if !matched {
        return Err("Laban clip IRI has no canonical binding in the source CBLD".into());
    }
    Ok(())
}

fn beat_times(value: &Value) -> Result<Vec<f64>, String> {
    let source = list(value)?;
    if !(2..=MAX_BEATS).contains(&source.len()) {
        return Err("Laban requires 2–512 authored beat markers".into());
    }
    let times = source.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    if times[0] < 0.0
        || times[times.len() - 1] > MAX_TIME_S
        || times
            .windows(2)
            .any(|pair| !(60.0 / 320.0..=60.0 / 20.0).contains(&(pair[1] - pair[0])))
    {
        return Err(
            "Laban beat markers must be increasing in [0, 1200] seconds at 20–320 BPM".into(),
        );
    }
    Ok(times)
}

fn phrases(value: &Value, beat_count: usize) -> Result<Vec<Phrase>, String> {
    let source = list(value)?;
    if !(1..=MAX_PHRASES).contains(&source.len()) {
        return Err("Laban requires 1–64 authored phrases".into());
    }
    let (mut last_source, mut last_beat) = (0.0, 0);
    let mut phrases = Vec::with_capacity(source.len());
    for value in source {
        let row = record(value)?;
        fields(row, &["source_end_s", "end_beat", "effort", "flow"])?;
        let source_end = num(required(row, "source_end_s")?)?;
        let end_beat = integer(required(row, "end_beat")?, "end_beat")?;
        let effort = Effort::read(text(required(row, "effort")?)?)?;
        let flow = number(row, "flow", 0.0)?;
        if source_end <= last_source
            || source_end > MAX_TIME_S
            || end_beat <= last_beat
            || end_beat >= beat_count
            || !(-1.0..=1.0).contains(&flow)
        {
            return Err("Laban phrase endpoints and beat indices must increase, and flow must be in [-1, 1]".into());
        }
        phrases.push(Phrase {
            source: [last_source, source_end],
            beats: [last_beat, end_beat],
            effort,
            flow,
        });
        last_source = source_end;
        last_beat = end_beat;
    }
    Ok(phrases)
}

fn iri_hash(iri: &str) -> Result<u64, String> {
    if iri.len() > 2048 {
        return Err("Laban IRI exceeds its lexical limit".into());
    }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    let hash = q_hash(iri);
    if hash == 0 {
        return Err("Laban IRI has a zero semantic identity".into());
    }
    Ok(hash)
}

fn receipt(
    clip: &str,
    digest: &str,
    beats: &[f64],
    phrases: &[Phrase],
    intervals: usize,
    realized: f64,
) -> Result<Vec<u8>, String> {
    let mut derived = Sha256::new();
    derived.update(digest.as_bytes());
    derived.update((clip.len() as u64).to_le_bytes());
    derived.update(clip.as_bytes());
    derived.update((beats.len() as u64).to_le_bytes());
    for beat in beats {
        derived.update(beat.to_bits().to_le_bytes());
    }
    for phrase in phrases {
        for time in phrase.source {
            derived.update(time.to_bits().to_le_bytes());
        }
        for beat in phrase.beats {
            derived.update((beat as u64).to_le_bytes());
        }
        derived.update(phrase.flow.to_bits().to_le_bytes());
        derived.update([
            u8::from(phrase.effort.sudden),
            u8::from(phrase.effort.strong),
            u8::from(phrase.effort.direct),
        ]);
    }
    derived.update((intervals as u64).to_le_bytes());
    derived.update(realized.to_bits().to_le_bytes());
    let context = format!("urn:qualia:laban:{:x}", derived.finalize());
    let source = format!("urn:sha256:{digest}");
    let mut terms = BTreeMap::new();
    let mut facts = Vec::new();
    let mut term = |iri: &str| -> Result<u64, String> {
        let hash = iri_hash(iri)?;
        if terms.get(&hash).is_some_and(|value: &String| value != iri) {
            return Err("Laban receipt lexical collision".into());
        }
        terms.entry(hash).or_insert_with(|| iri.into());
        Ok(hash)
    };
    let c = term(&context)?;
    let mut fact = |s: &str, p: &str, iri: Option<&str>, decimal: f64| -> Result<(), String> {
        let o = match iri {
            Some(iri) => term(iri)?,
            None => pack_typed_literal(&decimal.to_string(), "xsd:decimal")
                .ok_or("Laban decimal out of range")?,
        };
        let (s, p) = (term(s)?, term(p)?);
        facts.push([s, p, o, c, 0, NQuin::calculate_parity(s, p, o, c, 0)]);
        Ok(())
    };
    fact(&context, laban!("sourceDigest"), Some(&source), 0.0)?;
    fact(&context, laban!("sourceClip"), Some(clip), 0.0)?;
    fact(
        &context,
        laban!("sampleIntervalsPerPhrase"),
        None,
        intervals as f64,
    )?;
    fact(&context, laban!("maximumRootPathDeltaM"), None, realized)?;
    for (index, phrase) in phrases.iter().enumerate() {
        let id = format!("{context}:phrase:{index}");
        let effort = format!("{}{}", laban!("effort/"), phrase.effort.name);
        fact(&context, laban!("hasPhrase"), Some(&id), 0.0)?;
        fact(&id, laban!("basicEffortAction"), Some(&effort), 0.0)?;
        for (predicate, number) in [
            (laban!("sourceStartS"), phrase.source[0]),
            (laban!("sourceEndS"), phrase.source[1]),
            (laban!("firstBeatTimeS"), beats[phrase.beats[0]]),
            (laban!("lastBeatTimeS"), beats[phrase.beats[1]]),
            (laban!("flow"), phrase.flow),
        ] {
            fact(&id, predicate, None, number)?;
        }
    }
    let payload = ten::SymbolicRdfCborLdPayload {
        context_iri: context,
        lexicon: terms
            .into_iter()
            .map(|(term_hash, iri_utf8)| ten::MultilingualLexicalTerm {
                term_hash,
                iri_utf8,
                lang_bcp47: None,
                label_utf8: String::new(),
            })
            .collect(),
        nquins: facts,
        ..Default::default()
    };
    let bytes = ten::encode_symbolic_rdf_cbor_ld(&payload).map_err(|error| error.to_string())?;
    SemanticModelBuilder::new(MAX_RECEIPT_BYTES)
        .merge_cbld(&bytes)
        .and_then(|model| model.finish())
        .map_err(|error| error.to_string())
}

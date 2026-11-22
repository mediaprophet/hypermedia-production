//! Compile authored synthetic landmark geometry to native rigid translation tracks.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeSet, mem::size_of, sync::Arc};

use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    q_hash,
    render::skeletal::{valid_motor, Key, Motor, Track, MAX_JOINTS, MAX_KEYS},
};
use sha2::{Digest, Sha256};

use crate::{
    artifact::AnimClip, fields, integer, list, num, record, required, text, vector, Args,
    AssetHost, TypedArtifact, Value,
};

use super::{digest_iri, review_fact, review_graph, review_number};

const MAX_LANDMARKS: usize = 468;
const MAX_FRAMES: usize = 512;
const POSE_GRAPH: &str = "https://qualiadb.org/ontology/authored-pose#";

struct Binding<'args> {
    iri: &'args str,
    landmark: usize,
    parent: Option<usize>,
}

impl AssetHost<'_> {
    pub(crate) fn extract_biometrics_and_pose468(&mut self, args: &Args) -> Result<Value, String> {
        if text(required(args, "mode")?)? != "compile_authored_synthetic_pose" {
            return Err(
                "Select compile_authored_synthetic_pose for authored synthetic geometry".into(),
            );
        }
        fields(args, &["mode", "joints", "frames"])?;
        let joints = list(required(args, "joints")?)?;
        let frames = list(required(args, "frames")?)?;
        if !(1..=MAX_JOINTS).contains(&joints.len())
            || !(2..=MAX_FRAMES).contains(&frames.len())
            || joints.len() * frames.len() > MAX_KEYS
        {
            return Err("Authored pose must fit 128 native joint tracks, 2 to 512 frames, and 4,096 motor keys".into());
        }
        let first_frame = record(&frames[0])?;
        let landmark_count = list(required(first_frame, "points")?)?.len();
        if !(1..=MAX_LANDMARKS).contains(&landmark_count) {
            return Err("Authored pose needs 1 to 468 synthetic landmarks per frame".into());
        }
        let sample_count = joints.len() * frames.len();
        let scratch = sample_count * size_of::<Key>() + joints.len() * 8192;
        self.charge(
            frames.len() * landmark_count * 24 + sample_count * 72,
            scratch,
        )?;
        let bindings = read_bindings(joints, landmark_count)?;
        let mut keys = vec![
            Key {
                time: 0.0,
                local: Motor::identity()
            };
            sample_count
        ];
        let mut previous_time = 0.0;
        let mut points = vec![[0.0; 3]; landmark_count];
        let mut digest = pose_digest(&bindings, frames.len(), landmark_count);
        for (frame_index, frame) in frames.iter().enumerate() {
            self.charge(0, 0)?;
            let frame = record(frame)?;
            fields(frame, &["time_s", "points"])?;
            let time = num(required(frame, "time_s")?)?;
            if !(0.0..=3600.0).contains(&time)
                || (frame_index == 0 && time != 0.0)
                || (frame_index != 0 && time <= previous_time)
            {
                return Err(
                    "Authored pose times must start at zero and increase strictly within one hour"
                        .into(),
                );
            }
            previous_time = time;
            let samples = list(required(frame, "points")?)?;
            if samples.len() != points.len() {
                return Err("Authored pose frames disagree on landmark count".into());
            }
            for (point, value) in points.iter_mut().zip(samples) {
                *point = vector::<3>(value)?;
                if point.iter().any(|component| component.abs() > 1_000.0) {
                    return Err(
                        "Authored world-metre landmark coordinates must be within ±1,000".into(),
                    );
                }
            }
            digest.update(canonical_f64(time));
            for (slot, binding) in bindings.iter().enumerate() {
                let world = points[binding.landmark];
                let origin = binding
                    .parent
                    .map_or([0.0; 3], |parent| points[bindings[parent].landmark]);
                let [x, y, z] = std::array::from_fn(|axis| world[axis] - origin[axis]);
                let local = Motor::from_translation(x, y, z);
                if !valid_motor(&local) {
                    return Err(
                        "Authored landmark translation produced an invalid native motor".into(),
                    );
                }
                keys[slot * frames.len() + frame_index] = Key { time, local };
                for component in [x, y, z] {
                    digest.update(canonical_f64(component));
                }
            }
        }
        let digest = digest_iri(&digest.finalize().into());
        let tracks: Vec<Track> = (0..bindings.len())
            .map(|slot| Track {
                joint: slot as u16,
                first_key: (slot * frames.len()) as u32,
                key_count: frames.len() as u16,
            })
            .collect();
        let clip = AnimClip {
            duration_seconds: previous_time,
            looping: false,
            tracks,
            motor_keys: keys,
            morph_keys: Vec::new(),
        };

        let mut receipt = review_graph("AuthoredSyntheticPoseTopology")?;
        review_fact(
            &mut receipt,
            "declaredInputScope",
            &format!("{POSE_GRAPH}SyntheticAuthoredGeometry"),
        )?;
        review_fact(
            &mut receipt,
            "trackingTransform",
            &format!("{POSE_GRAPH}ParentRelativeTranslation"),
        )?;
        review_fact(&mut receipt, "syntheticPoseDigest", &digest)?;
        review_number(&mut receipt, "frameCount", frames.len())?;
        review_number(&mut receipt, "landmarkCount", landmark_count)?;
        for (slot, binding) in bindings.iter().enumerate() {
            receipt.fact(
                binding.iri,
                &format!("{POSE_GRAPH}trackIndex"),
                &format!("urn:qualia:index:{slot}"),
            )?;
            if let Some(parent) = binding.parent {
                receipt.fact(
                    binding.iri,
                    &format!("{POSE_GRAPH}parentJoint"),
                    bindings[parent].iri,
                )?;
            }
        }
        Ok(Value::Record(Args::from([
            (
                "clip".into(),
                self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(clip)))?,
            ),
            (
                "assessment_scope".into(),
                Value::String("authored_synthetic_pose_topology".into()),
            ),
            (
                "playback_transform".into(),
                Value::String("parent_relative_translation".into()),
            ),
            ("pose_digest_iri".into(), Value::String(digest)),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }
}

fn read_bindings(joints: &[Value], landmark_count: usize) -> Result<Vec<Binding<'_>>, String> {
    let mut identities = BTreeSet::new();
    let mut markers = BTreeSet::new();
    joints
        .iter()
        .enumerate()
        .map(|(slot, joint)| {
            let joint = record(joint)?;
            fields(joint, &["iri", "landmark", "parent"])?;
            let iri = text(required(joint, "iri")?)?;
            if iri.len() > 512
                || resource_identity_meta(iri).is_err()
                || !identities.insert(q_hash(iri))
            {
                return Err("Authored joints need distinct bounded RDF identities".into());
            }
            let landmark = integer(required(joint, "landmark")?, "landmark")?;
            if landmark >= landmark_count || !markers.insert(landmark) {
                return Err("Authored joint landmark slots must be distinct and in range".into());
            }
            let parent = joint
                .get("parent")
                .map(|value| integer(value, "parent"))
                .transpose()?;
            if !matches!((slot, parent), (0, None)) && !parent.is_some_and(|parent| parent < slot) {
                return Err(
                    "The root is first and each other authored joint needs a preceding parent slot"
                        .into(),
                );
            }
            Ok(Binding {
                iri,
                landmark,
                parent,
            })
        })
        .collect()
}

fn pose_digest(bindings: &[Binding<'_>], frame_count: usize, landmark_count: usize) -> Sha256 {
    let mut digest = Sha256::new();
    digest.update(b"QUALIA-AUTHORED-SYNTHETIC-POSE-V1\0");
    digest.update((frame_count as u32).to_le_bytes());
    digest.update((landmark_count as u32).to_le_bytes());
    digest.update((bindings.len() as u32).to_le_bytes());
    for binding in bindings {
        digest.update((binding.iri.len() as u32).to_le_bytes());
        digest.update(binding.iri.as_bytes());
        digest.update((binding.landmark as u32).to_le_bytes());
        digest.update(
            binding
                .parent
                .map_or(u32::MAX, |parent| parent as u32)
                .to_le_bytes(),
        );
    }
    digest
}

fn canonical_f64(value: f64) -> [u8; 8] {
    if value == 0.0 {
        0.0f64.to_le_bytes()
    } else {
        value.to_le_bytes()
    }
}

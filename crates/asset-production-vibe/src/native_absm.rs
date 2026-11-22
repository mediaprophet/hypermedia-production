//! Sample authored native SKL1 clips through one Etalon ABSM blend state.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use qualia_core_db::{
    container_10d::{
        GRAPHICS_ASSET_RING_BYTES,
        skeletal_container::{read_skeletal_section, read_skl2_section},
        skeletal_section::SKL2_NONE_INDEX,
    },
    etalon_render::skeletal::{
        AbsmLayer, AbsmMachine, AbsmParameter, AbsmStateNode, AbsmStateSource, BlendSpace2D, Clip,
        Key, MAX_JOINTS, MAX_KEYS, MAX_SIGNALS_PER_STATE, Motor, RootMotionExtractor, SignalQueue,
        SignalTrigger, Track, sample_pose_into, valid_motor,
    },
};

use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::AnimClip, boolean, fields, integer, list, num,
    record, required,
};

const MAX_POINTS: usize = 16;
const MAX_FRAMES: usize = 256;

impl AssetHost<'_> {
    pub(crate) fn evaluate_absm_blendspace(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "rig_skl2",
                "points",
                "coordinates",
                "step_s",
                "signals",
                "lock_horizontal_root",
                "lock_vertical_root",
            ],
        )?;
        let source = required(args, "source")?;
        let optional_rig = args.get("rig_skl2");
        let points = list(required(args, "points")?)?;
        let coordinates = list(required(args, "coordinates")?)?;
        if !(3..=MAX_POINTS).contains(&points.len())
            || !(2..=MAX_FRAMES).contains(&coordinates.len())
        {
            return Err("ABSM requires 3–16 points and 2–256 sample coordinates".into());
        }
        let dt = num(required(args, "step_s")?)?;
        if !(1e-4..=1.0).contains(&dt) {
            return Err("ABSM step_s must be in [0.0001, 1]".into());
        }
        let source_bytes = sealed_skeletal_resource(self, source)?.len();
        let companion_bytes = optional_rig
            .map(|value| sealed_skeletal_resource(self, value).map(<[u8]>::len))
            .transpose()?
            .unwrap_or(0);
        self.charge(
            source_bytes
                .saturating_add(companion_bytes)
                .saturating_add(8 * coordinates.len() * points.len() * MAX_KEYS),
            2 * 1024 * 1024,
        )?;

        let section =
            read_skeletal_section(sealed_skeletal_resource(self, source)?).map_err(|error| {
                format!("ABSM requires native SKL1 rest and animation data: {error:?}")
            })?;
        let counts = section.counts();
        if counts.joints == 0
            || counts.joints > MAX_JOINTS
            || counts.joints * coordinates.len() > MAX_KEYS
        {
            return Err("ABSM sampled clip exceeds the native joint/key bounds".into());
        }
        if let Some(value) = optional_rig {
            let rig = read_skl2_section(sealed_skeletal_resource(self, value)?)
                .map_err(|error| format!("ABSM SKL2 companion: {error:?}"))?;
            if rig.joints.len() != counts.joints
                || rig.joints.iter().enumerate().any(|(slot, row)| {
                    section.joint_id(slot) != Some(row.joint_id)
                        || section.joint(slot).is_none_or(|joint| {
                            joint.parent.unwrap_or(SKL2_NONE_INDEX) != row.parent_index
                        })
                })
            {
                return Err("ABSM SKL1 and SKL2 joint identities or parent order differ".into());
            }
        }
        let joints = (0..counts.joints)
            .map(|i| section.joint(i).ok_or("Missing admitted SKL1 joint"))
            .collect::<Result<Vec<_>, _>>()?;
        let mut roots = joints
            .iter()
            .enumerate()
            .filter(|(_, joint)| joint.parent.is_none());
        let root = roots.next().ok_or("ABSM source has no skeletal root")?.0;
        if roots.next().is_some() {
            return Err("ABSM root motion requires one skeletal root".into());
        }
        let joint_ids = (0..counts.joints)
            .map(|i| {
                section
                    .joint_id(i)
                    .map(Value::U64)
                    .ok_or("Missing joint identity")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let rest = joints
            .iter()
            .map(|joint| joint.rest_local)
            .collect::<Vec<_>>();
        let keys = (0..counts.keys)
            .map(|i| section.key(i).ok_or("Missing admitted SKL1 key"))
            .collect::<Result<Vec<_>, _>>()?;
        let tracks = (0..counts.tracks)
            .map(|i| section.track(i).ok_or("Missing admitted SKL1 track"))
            .collect::<Result<Vec<_>, _>>()?;
        let mut space = BlendSpace2D::<MAX_POINTS, 32>::new();
        let mut clips = Vec::with_capacity(points.len());
        let mut common_duration = None;
        for (slot, point) in points.iter().enumerate() {
            let point = record(point)?;
            fields(point, &["x", "y", "clip_id"])?;
            let x = blend_coordinate(required(point, "x")?)?;
            let y = blend_coordinate(required(point, "y")?)?;
            let clip_id = exact_id(required(point, "clip_id")?)?;
            if space.points[..slot].iter().any(|prior| prior.pos == [x, y]) {
                return Err("ABSM blend points must be distinct".into());
            }
            let clip = (0..counts.clips)
                .filter_map(|i| section.clip(i))
                .find(|clip| clip.id == clip_id)
                .ok_or("ABSM blend point references a clip absent from the sealed SKL1")?;
            if clip.looping
                || clip.duration < 1e-4
                || common_duration.is_some_and(|duration| duration != clip.duration)
            {
                return Err(
                    "ABSM source clips must share a nonlooping timeline of at least 0.0001 seconds"
                        .into(),
                );
            }
            common_duration = Some(clip.duration);
            let end = clip.first_track as usize + clip.track_count as usize;
            clips.push(Clip {
                duration: clip.duration,
                looping: false,
                tracks: tracks
                    .get(clip.first_track as usize..end)
                    .ok_or("Invalid source clip track span")?,
                keys: &keys,
            });
            if !space.add_point(x, y, slot as u16) {
                return Err("Native ABSM blend point admission failed".into());
            }
        }
        let triangle_count = space.build_triangulation();
        let mut used = [false; MAX_POINTS];
        for triangle in &space.triangles[..triangle_count] {
            for vertex in triangle.indices {
                used[usize::from(vertex)] = true;
            }
        }
        if used[..points.len()].iter().any(|&used| !used) {
            return Err(
                "ABSM blend sites do not form a complete nondegenerate triangulation".into(),
            );
        }
        let duration = common_duration.ok_or("ABSM clip bank is empty")?;
        let sample_duration = (coordinates.len() - 1) as f64 * dt;
        if sample_duration > duration {
            return Err("ABSM sample interval exceeds the sealed clip timeline".into());
        }
        let mut layer = AbsmLayer::<1, 1>::default();
        layer
            .add_state(AbsmStateNode {
                active: true,
                source: AbsmStateSource::Blend2D {
                    space_index: 0,
                    param_idx: 0,
                },
                duration: duration as f32,
                speed: 1.0,
                looping: false,
                triggers: authored_signals(args)?,
            })
            .ok_or("Native ABSM rejected its blend state")?;
        let mut machine = AbsmMachine::<1, 1, 1>::new();
        machine.blend_spaces_2d[0] = space;
        machine
            .add_layer(layer)
            .ok_or("Native ABSM rejected its blend layer")?;
        let lock_horizontal = boolean(args, "lock_horizontal_root", true)?;
        let lock_vertical = boolean(args, "lock_vertical_root", false)?;
        if lock_vertical && !lock_horizontal {
            return Err("Native vertical root lock requires horizontal lock".into());
        }
        let mut clip_poses = [[Motor::identity(); MAX_JOINTS]; MAX_POINTS];
        let mut pose = [Motor::identity(); MAX_JOINTS];
        let mut output_keys = vec![
            Key {
                time: 0.0,
                local: Motor::identity()
            };
            coordinates.len() * counts.joints
        ];
        let mut root_deltas = Vec::with_capacity(coordinates.len());
        let mut timeline_events = Vec::new();
        let mut signal_queue = SignalQueue::<MAX_SIGNALS_PER_STATE>::new();
        for (ordinal, raw) in coordinates.iter().enumerate() {
            let [x, y] = list(raw)? else {
                return Err("ABSM coordinate requires [x,y]".into());
            };
            let coordinate = [blend_coordinate(x)?, blend_coordinate(y)?];
            let time = (ordinal as f64 * dt).min(duration);
            for (clip, output) in clips.iter().zip(&mut clip_poses) {
                sample_pose_into(
                    &joints,
                    *clip,
                    time,
                    None,
                    1.0,
                    &mut output[..counts.joints],
                )
                .map_err(|error| format!("ABSM native clip sample {ordinal}: {error:?}"))?;
            }
            let views: [&[Motor]; MAX_POINTS] =
                std::array::from_fn(|i| &clip_poses[i][..counts.joints]);
            machine.set_parameter(0, AbsmParameter::Vec2(coordinate));
            let native_dt = (time as f32 - machine.layers[0].state_time).max(0.0);
            let mut delta = machine
                .step_and_evaluate_zero_heap(
                    native_dt,
                    &rest,
                    &views[..clips.len()],
                    &mut pose[..counts.joints],
                    &mut signal_queue,
                )
                .map_err(|error| format!("ABSM native layer {ordinal}: {error:?}"))?;
            if ordinal == 0 {
                let mut extractor = RootMotionExtractor::new(root, pose[root], pose[root]);
                extractor.lock_horizontal = lock_horizontal;
                extractor.lock_vertical = lock_vertical;
                delta = extractor.extract_and_lock(&mut pose[..counts.joints], false);
                machine.root_motion = Some(extractor);
            }
            if pose[..counts.joints]
                .iter()
                .any(|motor| !valid_motor(motor))
                || delta
                    .delta_translation
                    .iter()
                    .chain(&delta.delta_rotor)
                    .any(|v| !v.is_finite())
            {
                return Err(format!(
                    "ABSM produced a nonfinite or invalid motor at frame {ordinal}"
                ));
            }
            for (joint, &local) in pose[..counts.joints].iter().enumerate() {
                output_keys[joint * coordinates.len() + ordinal] = Key { time, local };
            }
            root_deltas.push(Value::Record(Args::from([
                ("time_s".into(), Value::F64(time)),
                ("translation_m".into(), values(delta.delta_translation)),
                ("rotor_wxyz".into(), values(delta.delta_rotor)),
            ])));
            while let Some(signal) = signal_queue.pop() {
                timeline_events.push(Value::Record(Args::from([
                    (
                        "time_s".into(),
                        Value::F64(f64::from(signal.normalized_time) * duration),
                    ),
                    ("observed_at_s".into(), Value::F64(time)),
                    ("signal_id".into(), Value::U64(u64::from(signal.signal_id))),
                    (
                        "sonic_archetype".into(),
                        Value::U64(u64::from(signal.sonic_archetype)),
                    ),
                    ("payload".into(), Value::F64(f64::from(signal.payload_f32))),
                ])));
            }
        }
        let output_tracks = (0..counts.joints)
            .map(|joint| Track {
                joint: joint as u16,
                first_key: (joint * coordinates.len()) as u32,
                key_count: coordinates.len() as u16,
            })
            .collect();
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(AnimClip {
            duration_seconds: sample_duration,
            looping: false,
            tracks: output_tracks,
            motor_keys: output_keys,
            morph_keys: Vec::new(),
        })))?;
        Ok(Value::Record(Args::from([
            ("clip".into(), clip),
            ("joint_ids".into(), Value::List(joint_ids)),
            ("root_joint_slot".into(), Value::U64(root as u64)),
            ("root_motion".into(), Value::List(root_deltas)),
            ("events".into(), Value::List(timeline_events)),
        ])))
    }
}

fn sealed_skeletal_resource<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
) -> Result<&'a [u8], String> {
    let resource = encoded_resource(host, value)?;
    if resource.kind != "10d"
        || resource.bytes.len() > host.limits.resource_bytes.min(GRAPHICS_ASSET_RING_BYTES)
    {
        return Err("ABSM rig requires an admitted .10d resource".into());
    }
    Ok(&resource.bytes)
}

fn blend_coordinate(value: &Value) -> Result<f32, String> {
    let value = num(value)?;
    if !(-1e6..=1e6).contains(&value) {
        return Err("ABSM coordinate exceeds its finite numeric range".into());
    }
    Ok(value as f32)
}

fn exact_id(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(id) => Ok(*id),
        Value::I64(id) if *id >= 0 => Ok(*id as u64),
        _ => Err("ABSM identity must be an exact unsigned integer".into()),
    }
}

fn authored_signals(args: &Args) -> Result<[SignalTrigger; MAX_SIGNALS_PER_STATE], String> {
    let mut output = [SignalTrigger::default(); MAX_SIGNALS_PER_STATE];
    let Some(raw) = args.get("signals") else {
        return Ok(output);
    };
    let rows = list(raw)?;
    if rows.len() > output.len() {
        return Err("ABSM signal count exceeds native capacity".into());
    }
    for (row, entry) in rows.iter().zip(&mut output) {
        let row = record(row)?;
        fields(
            row,
            &["signal_id", "normalized_time", "sonic_archetype", "payload"],
        )?;
        let signal_id = u32::try_from(integer(required(row, "signal_id")?, "signal_id")?)
            .map_err(|_| "ABSM signal_id exceeds u32")?;
        let archetype = row
            .get("sonic_archetype")
            .map(|value| integer(value, "sonic_archetype"))
            .transpose()?
            .unwrap_or(0);
        let sonic_archetype =
            u8::try_from(archetype).map_err(|_| "ABSM sonic_archetype exceeds u8")?;
        let time = num(required(row, "normalized_time")?)?;
        let payload_f32 = row.get("payload").map(num).transpose()?.unwrap_or(0.0) as f32;
        if time <= 0.0 || time > 1.0 || time as f32 == 0.0 || !payload_f32.is_finite() {
            return Err("ABSM authored signal time must be in (0,1] with finite payload".into());
        }
        *entry = SignalTrigger {
            active: true,
            signal_id,
            normalized_time: time as f32,
            sonic_archetype,
            payload_f32,
        };
    }
    output[..rows.len()].sort_by(|a, b| a.normalized_time.total_cmp(&b.normalized_time));
    Ok(output)
}

fn values<const N: usize>(values: [f32; N]) -> Value {
    Value::List(
        values
            .into_iter()
            .map(|value| Value::F64(f64::from(value)))
            .collect(),
    )
}

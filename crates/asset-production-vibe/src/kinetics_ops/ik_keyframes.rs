//! Analytical PGA Motor two-bone IK solver and keyframe interpolation.

use super::string;
use crate::*;
use qualia_core_db::container_10d::skeletal_container::attach_skeletal_section_into;
use qualia_core_db::container_10d::skeletal_section::{
    encode_skeletal_section_into, ClipRecord, Counts as SklCounts, SkeletalData,
};
use qualia_core_db::hypermedia_authoring::world3d::{
    AnimatedProperty, AnimationClip, AnimationTrack, Bone, Keyframe, Rig, Transform3D,
};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::compile_10d::compile_mesh_to_10d;
use qualia_core_db::render::skeletal::{
    solve_two_bone_motor_ik_zero_heap, Joint, Key, Motor, SkinWeight, Track,
};
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn solve_two_bone_ik(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["root", "target", "pole", "upper_length", "lower_length"])?;

        let root = point3(a, "root")?;
        let target = point3(a, "target")?;
        let pole = optional_point3(a, "pole", [0.0, 1.0, 0.0])?;

        let upper_length = number(a, "upper_length", 1.0)?;
        if !(0.001..=1000.0).contains(&upper_length) {
            return Err("upper_length outside 0.001..=1000.0".into());
        }

        let lower_length = number(a, "lower_length", 1.0)?;
        if !(0.001..=1000.0).contains(&lower_length) {
            return Err("lower_length outside 0.001..=1000.0".into());
        }

        self.charge(256, 512)?;

        let rt = [
            target[0] - root[0],
            target[1] - root[1],
            target[2] - root[2],
        ];
        let d = (rt[0] * rt[0] + rt[1] * rt[1] + rt[2] * rt[2]).sqrt();

        let min_reach = (upper_length - lower_length).abs();
        let max_reach = upper_length + lower_length;
        let reachable = d <= max_reach && d >= min_reach;

        let min_clamp = min_reach.max(1e-6);
        let max_clamp = (max_reach - 1e-6).max(1e-6);
        let d_clamped = d.clamp(min_clamp, max_clamp);

        let cos_alpha = ((upper_length * upper_length + d_clamped * d_clamped
            - lower_length * lower_length)
            / (2.0 * upper_length * d_clamped))
            .clamp(-1.0, 1.0);
        let _sin_alpha = (1.0 - cos_alpha * cos_alpha).max(0.0).sqrt();

        let u_dir = if d > 1e-9 {
            [rt[0] / d, rt[1] / d, rt[2] / d]
        } else {
            [0.0, 0.0, 1.0]
        };

        let p_rel = [pole[0] - root[0], pole[1] - root[1], pole[2] - root[2]];
        let p_dot = dot_prod(p_rel, u_dir);
        let p_proj = [
            p_rel[0] - p_dot * u_dir[0],
            p_rel[1] - p_dot * u_dir[1],
            p_rel[2] - p_dot * u_dir[2],
        ];
        let proj_len = len_sq(p_proj).sqrt();

        let _u_bend = if proj_len > 1e-6 {
            [p_proj[0] / proj_len, p_proj[1] / proj_len, p_proj[2] / proj_len]
        } else {
            let ref_axis = if u_dir[0].abs() < 0.9 {
                [1.0, 0.0, 0.0]
            } else {
                [0.0, 1.0, 0.0]
            };
            normalize(cross(u_dir, ref_axis))
        };

        let root_m = Motor::from_rotation_translation([1.0, 0.0, 0.0, 0.0], root);
        let mid_m = Motor::from_rotation_translation(
            [1.0, 0.0, 0.0, 0.0],
            [root[0], root[1] + upper_length, root[2]],
        );
        let end_m = Motor::from_rotation_translation(
            [1.0, 0.0, 0.0, 0.0],
            [root[0], root[1] + upper_length + lower_length, root[2]],
        );
        let ik_res = solve_two_bone_motor_ik_zero_heap(
            root_m,
            mid_m,
            end_m,
            upper_length as f32,
            lower_length as f32,
            [target[0] as f32, target[1] as f32, target[2] as f32],
            [pole[0] as f32, pole[1] as f32, pole[2] as f32],
            0.001,
            std::f32::consts::PI,
        )
        .map_err(|e| format!("Etalon PGA Motor IK failed: {e:?}"))?;
        let (r_upper, _) = ik_res.world_motors[0].to_rotation_translation();
        let (r_lower, mid_position) = ik_res.world_motors[1].to_rotation_translation();
        let (_, end_position) = ik_res.world_motors[2].to_rotation_translation();
        let upper_rotation = [r_upper[1], r_upper[2], r_upper[3], r_upper[0]];
        let lower_rotation = [r_lower[1], r_lower[2], r_lower[3], r_lower[0]];

        let mut rig = Rig::new("ik_chain", "TwoBoneIK");
        rig.add_bone(Bone {
            id: "root".into(),
            name: "root".into(),
            parent: None,
            children: vec!["mid".into()],
            rest_transform: Transform3D {
                position: [root[0] as f32, root[1] as f32, root[2] as f32],
                rotation: [
                    upper_rotation[0] as f32,
                    upper_rotation[1] as f32,
                    upper_rotation[2] as f32,
                    upper_rotation[3] as f32,
                ],
                scale: [1.0, 1.0, 1.0],
            },
            inverse_bind: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        });
        rig.add_bone(Bone {
            id: "mid".into(),
            name: "mid".into(),
            parent: Some("root".into()),
            children: vec!["end".into()],
            rest_transform: Transform3D {
                position: [
                    mid_position[0] as f32,
                    mid_position[1] as f32,
                    mid_position[2] as f32,
                ],
                rotation: [
                    lower_rotation[0] as f32,
                    lower_rotation[1] as f32,
                    lower_rotation[2] as f32,
                    lower_rotation[3] as f32,
                ],
                scale: [1.0, 1.0, 1.0],
            },
            inverse_bind: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        });
        rig.add_bone(Bone {
            id: "end".into(),
            name: "end".into(),
            parent: Some("mid".into()),
            children: vec![],
            rest_transform: Transform3D {
                position: [
                    end_position[0] as f32,
                    end_position[1] as f32,
                    end_position[2] as f32,
                ],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            },
            inverse_bind: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        });

        let mut out = BTreeMap::new();
        out.insert(
            "root_position".into(),
            Value::List(vec![
                Value::F64(root[0]),
                Value::F64(root[1]),
                Value::F64(root[2]),
            ]),
        );
        out.insert(
            "mid_position".into(),
            Value::List(vec![
                Value::F64(mid_position[0]),
                Value::F64(mid_position[1]),
                Value::F64(mid_position[2]),
            ]),
        );
        out.insert(
            "end_position".into(),
            Value::List(vec![
                Value::F64(end_position[0]),
                Value::F64(end_position[1]),
                Value::F64(end_position[2]),
            ]),
        );
        out.insert(
            "upper_rotation".into(),
            Value::List(vec![
                Value::F64(upper_rotation[0]),
                Value::F64(upper_rotation[1]),
                Value::F64(upper_rotation[2]),
                Value::F64(upper_rotation[3]),
            ]),
        );
        out.insert(
            "lower_rotation".into(),
            Value::List(vec![
                Value::F64(lower_rotation[0]),
                Value::F64(lower_rotation[1]),
                Value::F64(lower_rotation[2]),
                Value::F64(lower_rotation[3]),
            ]),
        );
        out.insert("reachable".into(), Value::Bool(reachable));
        out.insert("bone_count".into(), Value::I64(rig.bone_count() as i64));

        let (container_bytes, skl1_byte_len) = seal_ik_pose_skl1_10d(
            root,
            upper_length,
            lower_length,
            &ik_res.local_motors,
        )?;
        self.charge(256, container_bytes.len().max(1024))?;
        out.insert("skl1_byte_len".into(), Value::I64(skl1_byte_len as i64));
        out.insert("skl1_joint_count".into(), Value::I64(3));
        out.insert("container_10d".into(), self.encode("10d", container_bytes)?);

        Ok(Value::Record(out))
    }

    pub(super) fn interpolate_keyframes(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["keyframes", "fps", "duration", "easing", "property"])?;

        let keyframes_val = required(a, "keyframes")?;
        let kf_list = list(keyframes_val)?;
        if kf_list.is_empty() || kf_list.len() > 256 {
            return Err("keyframes must contain 1..=256 items".into());
        }

        let mut parsed_keyframes = Vec::with_capacity(kf_list.len());
        for (idx, item) in kf_list.iter().enumerate() {
            let rec = record(item)?;
            let time = num(required(rec, "time")?)?;
            if time < 0.0 {
                return Err(format!("keyframe[{idx}].time must be non-negative"));
            }
            let val = vector::<3>(required(rec, "value")?)?;
            parsed_keyframes.push(ParsedKeyframe { time, value: val });
        }
        parsed_keyframes.sort_by(|x, y| {
            x.time
                .partial_cmp(&y.time)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let fps = count(a, "fps", 24, 240)?;

        let duration = number(a, "duration", 1.0)?;
        if !(0.01..=60.0).contains(&duration) {
            return Err("duration outside 0.01..=60.0".into());
        }

        let easing = string(a, "easing", "linear")?;
        match easing {
            "linear" | "ease_in" | "ease_out" | "ease_in_out" | "step" | "smoothstep" => {}
            _ => {
                return Err(format!(
                    "Unknown easing curve: {easing}; expected 'linear', 'ease_in', 'ease_out', 'ease_in_out', 'step', or 'smoothstep'"
                ))
            }
        }

        let property_str = string(a, "property", "position")?;
        let anim_prop = match property_str {
            "position" => AnimatedProperty::Position,
            "rotation" => AnimatedProperty::Rotation,
            "scale" => AnimatedProperty::Scale,
            _ => {
                return Err(format!(
                    "Unknown property: {property_str}; expected 'position', 'rotation', or 'scale'"
                ))
            }
        };

        self.charge(fps * 16, fps * 64)?;

        let mut clip = AnimationClip::new("kinetics_clip", "InterpolatedTrack", duration);
        let track = AnimationTrack {
            bone_id: "root".into(),
            property: anim_prop,
            keyframes: parsed_keyframes
                .iter()
                .map(|k| Keyframe {
                    time: k.time,
                    value: [k.value[0] as f32, k.value[1] as f32, k.value[2] as f32],
                })
                .collect(),
        };
        clip.add_track(track);

        let frame_count = ((duration * fps as f64).round() as usize).clamp(1, 1024);

        let mut samples = Vec::with_capacity(frame_count);
        for frame in 0..frame_count {
            let time = (frame as f64 / fps as f64).min(duration);
            let val = interpolate_at(time, &parsed_keyframes, easing);

            let mut sample_rec = BTreeMap::new();
            sample_rec.insert("frame".into(), Value::I64(frame as i64));
            sample_rec.insert("time".into(), Value::F64(time));
            sample_rec.insert(
                "value".into(),
                Value::List(vec![
                    Value::F64(val[0]),
                    Value::F64(val[1]),
                    Value::F64(val[2]),
                ]),
            );
            samples.push(Value::Record(sample_rec));
        }

        let mut out = BTreeMap::new();
        out.insert("samples".into(), Value::List(samples));
        out.insert("frame_count".into(), Value::I64(frame_count as i64));
        out.insert("track_count".into(), Value::I64(clip.track_count() as i64));
        out.insert("duration".into(), Value::F64(duration));

        Ok(Value::Record(out))
    }
}

struct ParsedKeyframe {
    time: f64,
    value: [f64; 3],
}

fn point3(a: &Args, k: &str) -> Result<[f64; 3], String> {
    let val = required(a, k)?;
    vector::<3>(val)
}

fn optional_point3(a: &Args, k: &str, default: [f64; 3]) -> Result<[f64; 3], String> {
    match a.get(k) {
        Some(v) => vector::<3>(v),
        None => Ok(default),
    }
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot_prod(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn len_sq(a: [f64; 3]) -> f64 {
    dot_prod(a, a)
}

fn normalize(a: [f64; 3]) -> [f64; 3] {
    let l = len_sq(a).sqrt();
    if l > 1e-12 {
        [a[0] / l, a[1] / l, a[2] / l]
    } else {
        [0.0, 0.0, 0.0]
    }
}

#[allow(dead_code)]
fn quat_from_to(from: [f64; 3], to: [f64; 3]) -> [f64; 4] {
    let u_from = normalize(from);
    let u_to = normalize(to);
    let dot = dot_prod(u_from, u_to);
    if dot >= 0.99999999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if dot <= -0.99999999 {
        let mut axis = cross(u_from, [1.0, 0.0, 0.0]);
        if len_sq(axis) < 1e-6 {
            axis = cross(u_from, [0.0, 0.0, 1.0]);
        }
        let axis = normalize(axis);
        return [axis[0], axis[1], axis[2], 0.0];
    }
    let c = cross(u_from, u_to);
    let w = 1.0 + dot;
    let inv_len = 1.0 / (c[0] * c[0] + c[1] * c[1] + c[2] * c[2] + w * w).sqrt();
    [c[0] * inv_len, c[1] * inv_len, c[2] * inv_len, w * inv_len]
}

fn interpolate_at(time: f64, kfs: &[ParsedKeyframe], easing: &str) -> [f64; 3] {
    if kfs.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    if kfs.len() == 1 || time <= kfs[0].time {
        return kfs[0].value;
    }
    if time >= kfs.last().unwrap().time {
        return kfs.last().unwrap().value;
    }

    let mut idx = 0;
    while idx + 1 < kfs.len() && kfs[idx + 1].time <= time {
        idx += 1;
    }

    let k0 = &kfs[idx];
    let k1 = &kfs[idx + 1];
    let dt = k1.time - k0.time;
    let t_norm = if dt > 1e-9 {
        ((time - k0.time) / dt).clamp(0.0, 1.0)
    } else {
        1.0
    };

    let e = apply_easing(t_norm, easing);
    [
        k0.value[0] + e * (k1.value[0] - k0.value[0]),
        k0.value[1] + e * (k1.value[1] - k0.value[1]),
        k0.value[2] + e * (k1.value[2] - k0.value[2]),
    ]
}

fn apply_easing(t: f64, easing: &str) -> f64 {
    let t = t.clamp(0.0, 1.0);
    match easing {
        "ease_in" => t * t,
        "ease_out" => t * (2.0 - t),
        "ease_in_out" => {
            if t < 0.5 {
                2.0 * t * t
            } else {
                -1.0 + (4.0 - 2.0 * t) * t
            }
        }
        "step" => {
            if t < 1.0 {
                0.0
            } else {
                1.0
            }
        }
        "smoothstep" => t * t * (3.0 - 2.0 * t),
        _ => t,
    }
}

/// Bind-pose triangle + 3-joint SKL1 attached into a sealed `.10d`.
fn seal_ik_pose_skl1_10d(
    root: [f64; 3],
    upper_length: f64,
    lower_length: f64,
    local_pose: &[Motor; 3],
) -> Result<(Vec<u8>, usize), String> {
    let rest_root = Motor::from_translation(root[0], root[1], root[2]);
    let rest_mid_local = Motor::from_translation(0.0, upper_length, 0.0);
    let rest_end_local = Motor::from_translation(0.0, lower_length, 0.0);
    let mid_world = [
        root[0],
        root[1] + upper_length,
        root[2],
    ];
    let end_world = [
        root[0],
        root[1] + upper_length + lower_length,
        root[2],
    ];

    let joints = [
        Joint {
            parent: None,
            rest_local: rest_root,
            inverse_bind: Motor::from_translation(-root[0], -root[1], -root[2]),
        },
        Joint {
            parent: Some(0),
            rest_local: rest_mid_local,
            inverse_bind: Motor::from_translation(-mid_world[0], -mid_world[1], -mid_world[2]),
        },
        Joint {
            parent: Some(1),
            rest_local: rest_end_local,
            inverse_bind: Motor::from_translation(-end_world[0], -end_world[1], -end_world[2]),
        },
    ];
    let joint_ids = [
        q_hash("urn:hypermedia:kinetics:ik:root"),
        q_hash("urn:hypermedia:kinetics:ik:mid"),
        q_hash("urn:hypermedia:kinetics:ik:end"),
    ];
    let skin = [
        SkinWeight {
            joints: [0, 0, 0, 0],
            weights: [1.0, 0.0, 0.0, 0.0],
        },
        SkinWeight {
            joints: [1, 0, 0, 0],
            weights: [1.0, 0.0, 0.0, 0.0],
        },
        SkinWeight {
            joints: [2, 0, 0, 0],
            weights: [1.0, 0.0, 0.0, 0.0],
        },
    ];
    let duration = 1.0;
    let clips = [ClipRecord {
        id: q_hash("urn:hypermedia:kinetics:ik:clip"),
        duration,
        first_track: 0,
        track_count: 3,
        looping: false,
    }];
    let tracks = [
        Track {
            joint: 0,
            first_key: 0,
            key_count: 2,
        },
        Track {
            joint: 1,
            first_key: 2,
            key_count: 2,
        },
        Track {
            joint: 2,
            first_key: 4,
            key_count: 2,
        },
    ];
    let keys = [
        Key {
            time: 0.0,
            local: rest_root,
        },
        Key {
            time: duration,
            local: local_pose[0],
        },
        Key {
            time: 0.0,
            local: rest_mid_local,
        },
        Key {
            time: duration,
            local: local_pose[1],
        },
        Key {
            time: 0.0,
            local: rest_end_local,
        },
        Key {
            time: duration,
            local: local_pose[2],
        },
    ];
    let counts = SklCounts {
        joints: joints.len(),
        vertices: skin.len(),
        clips: clips.len(),
        tracks: tracks.len(),
        keys: keys.len(),
    };
    let encoded_len = counts
        .encoded_len()
        .ok_or_else(|| "SKL1 count limit exceeded".to_string())?;
    let mut skl1_bytes = vec![0u8; encoded_len];
    let skl1_data = SkeletalData {
        joints: &joints,
        joint_ids: &joint_ids,
        skin: &skin,
        clips: &clips,
        tracks: &tracks,
        keys: &keys,
    };
    let written = encode_skeletal_section_into(&skl1_data, &mut skl1_bytes)
        .map_err(|e| format!("Failed to encode SKL1 section: {e:?}"))?;

    let positions = [
        [root[0] as f32, root[1] as f32, root[2] as f32],
        [mid_world[0] as f32, mid_world[1] as f32, mid_world[2] as f32],
        [
            end_world[0] as f32 + 0.05,
            end_world[1] as f32,
            end_world[2] as f32,
        ],
    ];
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in &positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    let mesh = Mesh {
        positions: positions.to_vec(),
        triangles: vec![[0, 1, 2]],
        min,
        max,
    };
    let compiled = compile_mesh_to_10d(&mesh).map_err(|e| format!("IK .10d compile failed: {e:?}"))?;
    let container = attach_skl1_into_10d(&compiled, &skl1_bytes[..written])?;
    Ok((container, written))
}

fn attach_skl1_into_10d(compiled: &[u8], payload: &[u8]) -> Result<Vec<u8>, String> {
    let mut out = vec![0u8; compiled.len().saturating_add(payload.len()).saturating_add(8192)];
    match attach_skeletal_section_into(compiled, payload, &mut out) {
        Ok(n) => {
            out.truncate(n);
            Ok(out)
        }
        Err(_) => {
            let mut larger = vec![
                0u8;
                compiled
                    .len()
                    .saturating_add(payload.len())
                    .saturating_mul(2)
                    .saturating_add(65536)
            ];
            let n = attach_skeletal_section_into(compiled, payload, &mut larger)
                .map_err(|e| format!("SKL1 attach failed: {e:?}"))?;
            larger.truncate(n);
            Ok(larger)
        }
    }
}

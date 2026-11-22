//! Production entry points for bounded Q42/M64 kinematic controllers.

use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::AnimClip, count, fields, list, num, required,
    text, vector,
};
use qualia_core_db::{
    container_10d::skeletal_container::read_skl2_section,
    q_hash,
    render::skeletal::{
        Key, Track,
        semantic_motion::{
            DcaChainState, HydraulicEdgeStep, SemanticRotation, prepare_aeroelastic_rotation,
            prepare_dca_motor_chain, prepare_hydraulic_motion,
        },
    },
};
use std::{collections::BTreeMap, sync::Arc};

use crate::kinetics_marine_vehicle::{bundle_bytes, with_semantic_motion};

impl AssetHost<'_> {
    pub(crate) fn bake_aeroelastic_wind(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "bundle",
                "asset_key",
                "material_key",
                "action_iri",
                "context_iri",
                "step_s",
                "steps",
            ],
        )?;
        let asset_key = text(required(args, "asset_key")?)?;
        let dt = timestep(args, 1.0)?;
        let steps = count(args, "steps", 128, 4096)?;
        self.charge(
            bundle_bytes(self, required(args, "bundle")?)?
                .len()
                .saturating_add(steps * 4096),
            2 * 1024 * 1024 + steps * 128,
        )?;
        let (mut controller, slot) = with_semantic_motion(
            self,
            args,
            asset_key,
            |bundle, _, graph, action, iri, matter| {
                let controller = prepare_aeroelastic_rotation(graph, action, iri, matter)
                    .map_err(|error| format!("Aeroelastic branch controller: {error:?}"))?;
                let admitted = qualia_core_db::render::portal_load_10d::admit_leaf_assembly(
                    bundle, asset_key,
                )?;
                let target = admitted
                    .iter()
                    .find(|leaf| leaf.semantic_id == controller.semantic_id())
                    .ok_or("Authored branch motion target is absent from the admitted assembly")?;
                let slot = u16::try_from(target.slot)
                    .map_err(|_| "Aeroelastic leaf exceeds track capacity")?;
                Ok((controller, slot))
            },
        )?;
        let mut motor_keys = Vec::with_capacity(steps + 1);
        for sample in 0..=steps {
            let rotation = controller
                .advance(if sample == 0 { 0.0 } else { dt })
                .map_err(|error| format!("Aeroelastic sample {sample}: {error:?}"))?;
            motor_keys.push(Key {
                time: sample as f64 * dt,
                local: rotation.motor,
            });
        }
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(AnimClip {
            duration_seconds: steps as f64 * dt,
            looping: false,
            tracks: vec![Track {
                joint: slot,
                first_key: 0,
                key_count: motor_keys.len() as u16,
            }],
            motor_keys,
            morph_keys: Vec::new(),
        })))?;
        Ok(Value::Record(BTreeMap::from([
            ("clip".into(), clip),
            ("semantic_id".into(), Value::U64(controller.semantic_id())),
            ("instance_slot".into(), Value::U64(u64::from(slot))),
        ])))
    }

    pub(crate) fn solve_multibody_dca_ik(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "bundle",
                "asset_key",
                "material_key",
                "action_iri",
                "context_iri",
                "joint_iris",
                "targets_m",
                "secondary_targets_m",
                "step_s",
            ],
        )?;
        let keys = iri_list(required(args, "joint_iris")?, 2, 16)?;
        let source = required(args, "bundle")?;
        let asset_key = text(required(args, "asset_key")?)?;
        let dt = timestep(args, 0.05)?;
        let targets = list(required(args, "targets_m")?)?;
        if !(1..=128).contains(&targets.len()) {
            return Err("DCA requires 1–128 target samples".into());
        }
        let secondary_targets = args.get("secondary_targets_m").map(list).transpose()?;
        if secondary_targets.is_some_and(|secondary| secondary.len() != targets.len()) {
            return Err("Secondary DCA targets must pair one-for-one with targets_m".into());
        }
        let work_per_frame = if secondary_targets.is_some() {
            131_072
        } else {
            65_536
        };
        self.charge(
            bundle_bytes(self, source)?
                .len()
                .saturating_add(targets.len() * work_per_frame),
            4 * 1024 * 1024,
        )?;
        let (controller, track_indices) = with_semantic_motion(
            self,
            args,
            asset_key,
            |_, asset, graph, action, iri, matter| {
                let rig = read_skl2_section(asset)
                    .map_err(|error| format!("DCA SKL2 admission: {error:?}"))?;
                if !rig.loop_closures.is_empty() {
                    return Err("The serial DCA solver requires an open hinge chain".into());
                }
                let mut indices = Vec::with_capacity(keys.len());
                let mut expected_parent = u16::MAX;
                for key in &keys {
                    let hash = q_hash(key);
                    let (i, joint) = rig
                        .joints
                        .iter()
                        .enumerate()
                        .find(|(_, joint)| joint.joint_id == hash)
                        .ok_or_else(|| format!("DCA chain joint is absent from SKL2: {key}"))?;
                    if joint.parent_index != expected_parent {
                        return Err("DCA ordered joints must start at the root and follow SKL2 parent links".into());
                    }
                    expected_parent =
                        u16::try_from(i).map_err(|_| "DCA joint exceeds track capacity")?;
                    indices.push(expected_parent);
                }
                let controller = prepare_dca_motor_chain(graph, action, &keys, iri, matter)
                    .map_err(|error| format!("DCA semantic controller: {error:?}"))?;
                Ok((controller, indices))
            },
        )?;
        let mut state = vec![DcaChainState::default(); controller.len()];
        controller
            .initialize_rest_state(&mut state)
            .map_err(|error| format!("DCA rest state: {error:?}"))?;
        let mut motors = vec![None; state.len()];
        let mut torques = vec![0.0; state.len()];
        let mut tracks = Vec::with_capacity(state.len());
        for (slot, joint) in state.iter().enumerate() {
            if joint.angle_radians != 0.0 {
                return Err("DCA authored rest pose lies outside its contact-safe interval".into());
            }
            let bind = controller
                .rest_motor(slot)
                .ok_or("DCA joint is missing its authored bind pose")?;
            let mut track = Vec::with_capacity(targets.len() + 1);
            track.push(Key {
                time: 0.0,
                local: bind,
            });
            tracks.push(track);
        }
        let mut report = Vec::with_capacity(targets.len());
        for (sample, target) in targets.iter().enumerate() {
            let target = vector(target)?;
            let (result, secondary) = if let Some(secondary) = secondary_targets {
                let targets = [target, vector(&secondary[sample])?];
                let [primary, secondary] = controller
                    .advance_two_points(targets, dt, &mut state, &mut motors, &mut torques)
                    .map_err(|error| format!("DCA sample {sample}: {error:?}"))?;
                (primary, Some(secondary))
            } else {
                let primary = controller
                    .advance(target, dt, &mut state, &mut motors, &mut torques)
                    .map_err(|error| format!("DCA sample {sample}: {error:?}"))?;
                (primary, None)
            };
            let time = dt * (sample + 1) as f64;
            for (i, motor) in motors.iter().enumerate() {
                let local = motor.ok_or("DCA solver omitted a joint motor")?;
                let bind = controller
                    .rest_motor(i)
                    .ok_or("DCA joint is missing its authored bind pose")?;
                tracks[i].push(Key {
                    time,
                    local: bind.mul(&local.motor),
                });
            }
            let mut row = BTreeMap::from([
                ("time_s".into(), Value::F64(time)),
                (
                    "tracking_residual_m".into(),
                    Value::F64(result.tracking_residual_m),
                ),
                ("ik_residual_m".into(), Value::F64(result.ik_residual_m)),
                ("torques_nm".into(), real_list(torques.iter().copied())),
            ]);
            if let Some(secondary) = secondary {
                row.insert(
                    "secondary_tracking_residual_m".into(),
                    Value::F64(secondary.tracking_residual_m),
                );
                row.insert(
                    "secondary_ik_residual_m".into(),
                    Value::F64(secondary.ik_residual_m),
                );
            }
            report.push(Value::Record(row));
        }
        let mut motor_keys = Vec::with_capacity(keys.len() * (targets.len() + 1));
        let tracks = track_indices
            .into_iter()
            .zip(tracks)
            .map(|(joint, keys)| {
                let track = Track {
                    joint,
                    first_key: motor_keys.len() as u32,
                    key_count: keys.len() as u16,
                };
                motor_keys.extend(keys);
                track
            })
            .collect();
        let clip = self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(AnimClip {
            duration_seconds: dt * targets.len() as f64,
            looping: false,
            tracks,
            motor_keys,
            morph_keys: Vec::new(),
        })))?;
        Ok(Value::Record(BTreeMap::from([
            ("clip".into(), clip),
            (
                "joint_iris".into(),
                Value::List(keys.into_iter().map(|s| Value::String(s.into())).collect()),
            ),
            ("torque_report".into(), Value::List(report)),
        ])))
    }

    pub(crate) fn solve_homeostatic_graph_flux(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "bundle",
                "asset_key",
                "material_key",
                "action_iri",
                "context_iri",
                "node_iris",
                "edge_iris",
                "source_m3_s",
                "step_s",
                "steps",
            ],
        )?;
        let node_iris = iri_list(required(args, "node_iris")?, 2, 32)?;
        let edge_iris = iri_list(required(args, "edge_iris")?, 1, 96)?;
        let source = real_values(required(args, "source_m3_s")?, node_iris.len())?;
        let asset_key = text(required(args, "asset_key")?)?;
        let dt = timestep(args, 1.0)?;
        let steps = count(args, "steps", 1, 4096)?;
        self.charge(
            bundle_bytes(self, required(args, "bundle")?)?
                .len()
                .saturating_add(steps * 65_536),
            4 * 1024 * 1024,
        )?;
        let controller =
            with_semantic_motion(self, args, asset_key, |_, _, graph, action, iri, matter| {
                prepare_hydraulic_motion(graph, action, &node_iris, &edge_iris, iri, matter)
                    .map_err(|error| format!("Hydraulic semantic controller: {error:?}"))
            })?;
        let mut pressure = controller.reference_pressures().collect::<Vec<_>>();
        let mut edges = vec![HydraulicEdgeStep::default(); controller.edge_count()];
        let mut motors = vec![None; controller.node_count()];
        let mut storage = 0.0;
        let mut supply = 0.0;
        let mut max_residual = 0.0f64;
        for sample in 0..steps {
            let result = controller
                .advance(dt, &mut pressure, &source, &mut edges, &mut motors)
                .map_err(|error| format!("Hydraulic sample {sample}: {error:?}"))?;
            storage += result.storage_change_m3;
            supply += result.supplied_volume_m3;
            max_residual = max_residual.max(result.mass_residual_m3.abs());
        }
        if !storage.is_finite() || !supply.is_finite() {
            return Err("Hydraulic accumulated volumes exceed finite range".into());
        }
        let edge_results = edges
            .into_iter()
            .zip(edge_iris)
            .map(|(edge, iri)| {
                Value::Record(BTreeMap::from([
                    ("iri".into(), Value::String(iri.into())),
                    ("semantic_id".into(), Value::U64(edge.semantic_id)),
                    ("flow_m3_s".into(), Value::F64(edge.flow_m3_s)),
                    ("wall_shear_pa".into(), Value::F64(edge.wall_shear_pa)),
                    (
                        "conductance_fraction".into(),
                        Value::F64(edge.conductance_fraction),
                    ),
                ]))
            })
            .collect();
        let node_results = node_iris
            .into_iter()
            .zip(pressure)
            .map(|(iri, pressure)| {
                Value::Record(BTreeMap::from([
                    ("iri".into(), Value::String(iri.into())),
                    ("pressure_pa".into(), Value::F64(pressure)),
                ]))
            })
            .collect();
        Ok(Value::Record(BTreeMap::from([
            ("nodes".into(), Value::List(node_results)),
            ("edges".into(), Value::List(edge_results)),
            (
                "motors".into(),
                Value::List(motors.into_iter().flatten().map(motor_value).collect()),
            ),
            ("storage_change_m3".into(), Value::F64(storage)),
            ("supplied_volume_m3".into(), Value::F64(supply)),
            ("max_step_mass_residual_m3".into(), Value::F64(max_residual)),
        ])))
    }
}

fn iri_list(value: &Value, min: usize, max: usize) -> Result<Vec<&str>, String> {
    let rows = list(value)?;
    if !(min..=max).contains(&rows.len()) {
        return Err(format!("Semantic ordering requires {min}–{max} IRIs"));
    }
    rows.iter()
        .map(|value| {
            let iri = text(value)?;
            if iri.is_empty() || iri.len() > 4096 {
                return Err("Invalid sized semantic selector".into());
            }
            Ok(iri)
        })
        .collect()
}

fn timestep(args: &Args, max: f64) -> Result<f64, String> {
    let dt = num(required(args, "step_s")?)?;
    if dt <= 0.0 || dt > max {
        return Err(format!("Motion step must be in (0, {max}] seconds"));
    }
    Ok(dt)
}

fn real_values(value: &Value, count: usize) -> Result<Vec<f64>, String> {
    let values = list(value)?;
    if values.len() != count {
        return Err("Source flow requires one value per authored node".into());
    }
    values.iter().map(num).collect()
}

fn real_list(values: impl IntoIterator<Item = f64>) -> Value {
    Value::List(values.into_iter().map(Value::F64).collect())
}

fn motor_value(rotation: SemanticRotation) -> Value {
    let motor = rotation.motor;
    Value::Record(BTreeMap::from([
        ("semantic_id".into(), Value::U64(rotation.semantic_id)),
        ("angle_radians".into(), Value::F64(rotation.angle_radians)),
        (
            "motor".into(),
            real_list([
                motor.r_w, motor.r_x, motor.r_y, motor.r_z, motor.d_w, motor.d_x, motor.d_y,
                motor.d_z,
            ]),
        ),
    ]))
}

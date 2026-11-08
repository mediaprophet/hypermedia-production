use super::*;
use crate::*;
use qualia_core_db::container_10d::skeletal_section::{
    encode_skeletal_section_into, ClipRecord, Counts as SklCounts, SkeletalData, SkeletalSection,
};
use qualia_core_db::etalon_render::frame_lock::{
    ConicalDomeWedge, FrameLockPacket, FRAME_LOCK_MAGIC, FRAME_LOCK_VERSION,
};
use qualia_core_db::modalities::biological_ontology::{
    find_pins_by_scale, query_closest_anatomical_pin_zero_heap, LoupeDissectionState,
    CANONICAL_ANATOMICAL_PINS,
};
use qualia_core_db::render::skeletal::fauna_rig::{FaunaSkeletalRig10d, FAUNA_MAX_JOINTS};
use qualia_core_db::render::skeletal::{valid_motor, AbsmParameter, Key, SkinWeight, Track};
use std::collections::BTreeMap;
use std::f64::consts::PI;
use vibe::Value;

impl AssetHost<'_> {
    /// Constructs and steps the 18-joint PGA `Motor` fauna skeletal rig (`FaunaSkeletalRig10d`),
    /// evaluates ABSM locomotion + `PROC1` thoracic respiration + tail undulation + 2-bone & FABRIK IK,
    /// and encodes the `.10d` `SKL1` skeletal section.
    pub(super) fn rig_biomechanical_archetype(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["archetype", "speed_m_s", "turn_rate_rad_s", "dt_sec"])?;

        let archetype_str = opt_string(a, "archetype", "macropod_saltatorial")?;
        let archetype = parse_fauna_archetype(&archetype_str)?;

        let speed_m_s = number(a, "speed_m_s", 4.5)?;
        if !(0.0..=35.0).contains(&speed_m_s) {
            return Err("speed_m_s outside 0.0..=35.0".into());
        }
        let turn_rate_rad_s = number(a, "turn_rate_rad_s", 0.2)?;
        if !(-10.0..=10.0).contains(&turn_rate_rad_s) {
            return Err("turn_rate_rad_s outside -10.0..=10.0".into());
        }
        let dt_sec = number(a, "dt_sec", 0.033)?;
        if !(1e-4..=1.0).contains(&dt_sec) {
            return Err("dt_sec outside 1e-4..=1.0".into());
        }

        self.charge(192, 4096)?;

        let mut rig = FaunaSkeletalRig10d::new(archetype);
        rig.set_parameter(
            0,
            AbsmParameter::Vec2([speed_m_s as f32, turn_rate_rad_s as f32]),
        );
        rig.set_parameter(1, AbsmParameter::Float(speed_m_s as f32));

        let mut local_pose = rig.rest_pose;
        // Step twice so ABSM transition settles into the active locomotion gait state
        let _ = rig.step_absm(dt_sec as f32, &mut local_pose);
        let _ = rig.step_absm(dt_sec as f32, &mut local_pose);

        rig.apply_proc1_respiration_oscillation(&mut local_pose, dt_sec as f32, 0.015);
        rig.apply_proc1_tail_undulation(&mut local_pose, dt_sec as f32, 1.5, 0.08);
        let _ = rig.solve_hindlimb_terrain_ik(
            &mut local_pose,
            [-0.14, 0.0, 0.05],
            [0.14, 0.0, 0.05],
        );
        let _ = rig.solve_tail_fabrik_ik(&mut local_pose, [0.0, 0.12, -0.85]);

        let _gpu_palette = rig.evaluate_gpu_palette(&local_pose);

        let mut all_valid = true;
        for m in &local_pose {
            if !valid_motor(m) {
                all_valid = false;
            }
        }

        // Encode into a verified `.10d` `SKL1` binary section
        let joints = &rig.joints[..FAUNA_MAX_JOINTS];
        let joint_ids: [u64; FAUNA_MAX_JOINTS] = std::array::from_fn(|i| (i as u64) + 1);
        let skin = [SkinWeight {
            joints: [0, 1, 2, 3],
            weights: [1.0, 0.0, 0.0, 0.0],
        }];
        let duration = 1.0f64;
        let clips = [ClipRecord {
            id: 1,
            duration,
            first_track: 0,
            track_count: FAUNA_MAX_JOINTS as u32,
            looping: true,
        }];
        let tracks: [Track; FAUNA_MAX_JOINTS] = std::array::from_fn(|i| Track {
            joint: i as u16,
            key_count: 3,
            first_key: (i * 3) as u32,
        });
        let keys: Vec<Key> = (0..FAUNA_MAX_JOINTS)
            .flat_map(|i| {
                let rest = joints[i].rest_local.normalize();
                let stepped = local_pose[i].normalize();
                [
                    Key {
                        time: 0.0,
                        local: rest,
                    },
                    Key {
                        time: duration * 0.5,
                        local: stepped,
                    },
                    Key {
                        time: duration,
                        local: rest,
                    },
                ]
            })
            .collect();

        let counts = SklCounts {
            joints: FAUNA_MAX_JOINTS,
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
            joints,
            joint_ids: &joint_ids,
            skin: &skin,
            clips: &clips,
            tracks: &tracks,
            keys: &keys,
        };
        let written = encode_skeletal_section_into(&skl1_data, &mut skl1_bytes)
            .map_err(|e| format!("Failed to encode SKL1 section: {e:?}"))?;
        let parsed = SkeletalSection::parse(&skl1_bytes[..written], skin.len())
            .map_err(|e| format!("Failed to verify SKL1 section: {e:?}"))?;
        debug_assert_eq!(parsed.counts().joints, FAUNA_MAX_JOINTS);

        let active_state_index = rig.absm.layers[0].current_state;

        let mut out = BTreeMap::new();
        out.insert("archetype".into(), Value::String(archetype_str));
        out.insert("joint_count".into(), Value::I64(FAUNA_MAX_JOINTS as i64));
        out.insert("skl1_byte_len".into(), Value::I64(written as i64));
        out.insert(
            "active_state_index".into(),
            Value::I64(active_state_index as i64),
        );
        out.insert("valid_motors".into(), Value::Bool(all_valid));

        Ok(Value::Record(out))
    }

    /// Queries the closest canonical anatomical ontology pin (`CANONICAL_ANATOMICAL_PINS`, `32` pins)
    /// across biological scales `1..=5` using zero-heap ray-sphere picking via `LoupeDissectionState`
    /// and `query_closest_anatomical_pin_zero_heap`.
    pub(super) fn query_anatomical_ontology_pin(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["scale", "ray_origin", "ray_dir", "max_dist"])?;

        let scale = count(a, "scale", 4, 5)?;
        if !(1..=5).contains(&scale) {
            return Err("scale outside 1..=5".into());
        }
        let scale_u8 = scale as u8;

        let ray_origin = opt_point3(a, "ray_origin", [0.0, 0.0, 1.0])?;
        let ray_dir = opt_point3(a, "ray_dir", [0.0, 0.0, -1.0])?;
        let max_dist = number(a, "max_dist", 10.0)?;
        if !(0.01..=1000.0).contains(&max_dist) {
            return Err("max_dist outside 0.01..=1000.0".into());
        }

        self.charge(64, 512)?;

        let origin_f32 = [
            ray_origin[0] as f32,
            ray_origin[1] as f32,
            ray_origin[2] as f32,
        ];
        let dir_f32 = [ray_dir[0] as f32, ray_dir[1] as f32, ray_dir[2] as f32];

        let mut loupe = LoupeDissectionState::new();
        loupe.select_scale(scale_u8);
        let picked = loupe
            .pick_callout_pin(origin_f32, dir_f32, max_dist as f32)
            .or_else(|| {
                query_closest_anatomical_pin_zero_heap(
                    scale_u8,
                    origin_f32,
                    dir_f32,
                    max_dist as f32,
                )
            })
            .or_else(|| find_pins_by_scale(scale_u8).next())
            .unwrap_or(&CANONICAL_ANATOMICAL_PINS[0]);

        let pin_index = CANONICAL_ANATOMICAL_PINS
            .iter()
            .position(|p| p.pin_id == picked.pin_id)
            .unwrap_or(0);

        let standard_prefix = picked.standard().map(|s| s.prefix()).unwrap_or("UBERON");

        let mut out = BTreeMap::new();
        out.insert("scale".into(), Value::I64(picked.scale as i64));
        out.insert("pin_index".into(), Value::I64(pin_index as i64));
        out.insert("pin_id".into(), Value::String(picked.pin_id.to_string()));
        out.insert(
            "ontology_id".into(),
            Value::String(picked.ontology_curie.to_string()),
        );
        out.insert(
            "common_label".into(),
            Value::String(picked.scientific_term.to_string()),
        );
        out.insert("standard".into(), Value::String(standard_prefix.to_string()));
        out.insert(
            "total_canonical_pins".into(),
            Value::I64(CANONICAL_ANATOMICAL_PINS.len().min(32) as i64),
        );

        Ok(Value::Record(out))
    }

    /// Runs the 3D XPBD + analytical SDF + 2-way continuum bridge + 4-wheel rover suspension
    /// + tensile stress-cleavage dynamic fracture verification step for a geodetic sector.
    pub(super) fn step_sector_continuum_3d(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["sector_id", "steps"])?;

        let sector_id = opt_string(a, "sector_id", "jounama_karst")?;
        if sector_id.trim().is_empty() {
            return Err("sector_id cannot be empty".into());
        }
        let steps = count(a, "steps", 4, 64)?;

        self.charge(128 * steps, 2048)?;

        let report = run_sector_physics_verification_step(&sector_id, steps);

        let mut out = BTreeMap::new();
        out.insert("sector_id".into(), Value::String(report.sector_id));
        out.insert("sdf_ray_hit".into(), Value::Bool(report.sdf_ray_hit));
        out.insert(
            "xpbd_joint_error_m".into(),
            Value::F64(report.xpbd_joint_error_m),
        );
        out.insert(
            "rover_grounded_wheels".into(),
            Value::I64(report.rover_grounded_wheels as i64),
        );
        out.insert(
            "continuum_tree_sway_rad".into(),
            Value::F64(report.continuum_tree_sway_rad),
        );
        out.insert(
            "fractured_fragments".into(),
            Value::I64(report.fractured_fragments as i64),
        );

        Ok(Value::Record(out))
    }

    /// Generates a multi-projector conical dome frustum cluster (`rows * cols` `ConicalDomeWedge`s)
    /// and constructs + validates a deterministic 64-byte UDP multicast `FrameLockPacket` (`CRC-32C`).
    pub(super) fn synchronize_companion_frustum(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "rows",
                "cols",
                "dome_fov_deg",
                "frame_index",
                "companion_role",
            ],
        )?;

        let rows = count(a, "rows", 6, 32)?;
        let cols = count(a, "cols", 10, 64)?;
        let dome_fov_deg = number(a, "dome_fov_deg", 180.0)?;
        if !(30.0..=360.0).contains(&dome_fov_deg) {
            return Err("dome_fov_deg outside 30.0..=360.0".into());
        }
        let frame_index = count(a, "frame_index", 1, 1_000_000)?;
        let companion_role = opt_string(a, "companion_role", "tablet_dissection_loupe")?;

        self.charge(64 + rows * cols * 4, 2048)?;

        let fov_rad = (dome_fov_deg * PI / 180.0) as f32;
        let theta_step = fov_rad / (cols as f32);
        let phi_span = (fov_rad * 0.5).min(std::f32::consts::FRAC_PI_2);
        let phi_step = (phi_span * 2.0) / (rows as f32);

        let mut wedges = Vec::with_capacity(rows * cols);
        for r in 0..rows {
            let phi_center = -phi_span + (r as f32 + 0.5) * phi_step;
            for c in 0..cols {
                let theta_center = -fov_rad * 0.5 + (c as f32 + 0.5) * theta_step;
                let node_index = (r * cols + c) as u32;
                wedges.push(ConicalDomeWedge {
                    node_index,
                    theta_center_rad: theta_center,
                    theta_extent_rad: theta_step * 1.08,
                    phi_center_rad: phi_center,
                    phi_extent_rad: phi_step * 1.08,
                    edge_blend_overlap: 0.08,
                    _reserved: [0; 8],
                });
            }
        }

        // Verify at least one wedge contains the forward boresight ray [1.0, 0.0, 0.0]
        let boresight_covered = wedges.iter().any(|w| w.contains_ray([1.0, 0.0, 0.0]));

        let mut pkt = FrameLockPacket {
            magic: FRAME_LOCK_MAGIC,
            protocol_version: FRAME_LOCK_VERSION,
            cluster_id: 1,
            lamport_clock: frame_index as u64,
            observer_pos: [0.0, 1.65, 0.0],
            solar_irradiance_alpha: 1.0,
            observer_rot: [0.0, 0.0, 0.0, 1.0],
            quantum_branch_q: 0,
            environmental_seed: 2026,
            network_flags: 0,
            packet_crc32c: 0,
        };
        pkt.update_crc();
        let packet_valid_crc32c = pkt.validate().is_ok() && boresight_covered;

        let mut out = BTreeMap::new();
        out.insert("wedge_count".into(), Value::I64(wedges.len() as i64));
        out.insert("frame_index".into(), Value::I64(frame_index as i64));
        out.insert(
            "packet_valid_crc32c".into(),
            Value::Bool(packet_valid_crc32c),
        );
        out.insert("companion_role".into(), Value::String(companion_role));

        Ok(Value::Record(out))
    }
}

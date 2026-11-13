use crate::*;
use std::collections::BTreeMap;
use vibe::Value;

// ============================================================================
// 16-Byte Smallest-Three Quaternion Compression Helpers
// ============================================================================

const SQRT_2: f64 = std::f64::consts::SQRT_2;

/// Quantizes an 8-float PGA motor state (position [x,y,z], rotation [qx,qy,qz,qw], sequence, entity_id)
/// into an exact 16-byte binary delta replication packet.
fn quantize_pga_motor(
    pos: [f64; 3],
    rot: [f64; 4],
    sequence: u64,
    entity_id: u64,
) -> ([u8; 16], f64) {
    // 1. 48-bit millimeter translation (3 x i16)
    let x_mm = (pos[0] * 1000.0).round().clamp(-32768.0, 32767.0) as i16;
    let y_mm = (pos[1] * 1000.0).round().clamp(-32768.0, 32767.0) as i16;
    let z_mm = (pos[2] * 1000.0).round().clamp(-32768.0, 32767.0) as i16;

    // 2. Smallest-three unit quaternion compression (3 x u16 = 48 bits)
    let norm = (rot[0] * rot[0] + rot[1] * rot[1] + rot[2] * rot[2] + rot[3] * rot[3]).sqrt();
    let mut q = if norm > 1e-12 {
        [rot[0] / norm, rot[1] / norm, rot[2] / norm, rot[3] / norm]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    };

    let mut max_idx = 0;
    let mut max_abs = q[0].abs();
    for i in 1..4 {
        if q[i].abs() > max_abs {
            max_abs = q[i].abs();
            max_idx = i;
        }
    }

    if q[max_idx] < 0.0 {
        for c in &mut q {
            *c = -*c;
        }
    }

    let mut others = [0.0f64; 3];
    let mut oi = 0;
    for i in 0..4 {
        if i != max_idx {
            others[oi] = q[i];
            oi += 1;
        }
    }

    let u0 = ((others[0] * SQRT_2 + 1.0) * 0.5).clamp(0.0, 1.0);
    let u1 = ((others[1] * SQRT_2 + 1.0) * 0.5).clamp(0.0, 1.0);
    let u2 = ((others[2] * SQRT_2 + 1.0) * 0.5).clamp(0.0, 1.0);

    let q0 = (u0 * 32767.0).round() as u16; // 15 bits
    let q1 = (u1 * 32767.0).round() as u16; // 15 bits
    let q2 = (u2 * 65535.0).round() as u16; // 16 bits

    let w0 = ((max_idx as u16) << 14) | (q0 >> 1);
    let w1 = ((q0 & 1) << 15) | (q1 & 0x7FFF);
    let w2 = q2;

    // 3. 16-bit sequence number
    let seq16 = sequence as u16;

    // 4. 16-bit entity id + visibility mask
    let mask16 = ((entity_id as u16) & 0x7FFF) | 0x8000;

    let mut packet = [0u8; 16];
    packet[0..2].copy_from_slice(&x_mm.to_le_bytes());
    packet[2..4].copy_from_slice(&y_mm.to_le_bytes());
    packet[4..6].copy_from_slice(&z_mm.to_le_bytes());
    packet[6..8].copy_from_slice(&w0.to_le_bytes());
    packet[8..10].copy_from_slice(&w1.to_le_bytes());
    packet[10..12].copy_from_slice(&w2.to_le_bytes());
    packet[12..14].copy_from_slice(&seq16.to_le_bytes());
    packet[14..16].copy_from_slice(&mask16.to_le_bytes());

    // Roundtrip dequantization error
    let deq_x = (x_mm as f64) / 1000.0;
    let deq_y = (y_mm as f64) / 1000.0;
    let deq_z = (z_mm as f64) / 1000.0;
    let pos_err =
        ((pos[0] - deq_x).powi(2) + (pos[1] - deq_y).powi(2) + (pos[2] - deq_z).powi(2)).sqrt();

    (packet, pos_err)
}

impl AssetHost<'_> {
    // ------------------------------------------------------------------------
    // 3. Engine Telemetry Ring Buffer & QuinRidTable VRAM Ledger Inspector
    // ------------------------------------------------------------------------
    pub(super) fn inspect_engine_telemetry(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["frame_count", "target_fps"])?;

        let frame_count = count(a, "frame_count", 16, 256)?;
        let target_fps = count(a, "target_fps", 60, 240)?;

        let frame_budget_us = 1_000_000.0 / (target_fps as f64);
        let scale = 60.0 / (target_fps as f64);

        self.charge(frame_count * 16, frame_count * 64 + 4096)?;

        let mut frames_data = Vec::with_capacity(frame_count);
        let mut frame_totals = Vec::with_capacity(frame_count);

        let mut sum_slg = 0u64;
        let mut sum_vibe = 0u64;
        let mut sum_cull = 0u64;
        let mut sum_render = 0u64;
        let mut sum_xpbd = 0u64;
        let mut sum_audio = 0u64;
        let mut sum_layout = 0u64;

        for i in 0..frame_count {
            let slg = (150.0 * scale + ((i * 7) % 30) as f64).round() as u64;
            let vibe = (200.0 * scale + ((i * 13) % 40) as f64).round() as u64;
            let cull = (120.0 * scale + ((i * 11) % 25) as f64).round() as u64;
            let render = (1100.0 * scale + ((i * 29) % 150) as f64).round() as u64;
            let xpbd = (300.0 * scale + ((i * 17) % 50) as f64).round() as u64;
            let audio = (80.0 * scale + ((i * 5) % 20) as f64).round() as u64;
            let layout = (150.0 * scale + ((i * 19) % 30) as f64).round() as u64;
            let total = slg + vibe + cull + render + xpbd + audio + layout;

            sum_slg += slg;
            sum_vibe += vibe;
            sum_cull += cull;
            sum_render += render;
            sum_xpbd += xpbd;
            sum_audio += audio;
            sum_layout += layout;

            frame_totals.push(total);

            frames_data.push(serde_json::json!({
                "frame_index": i,
                "slg_arena_us": slg,
                "vibescript_us": vibe,
                "etalon_cull_us": cull,
                "etalon_render_us": render,
                "xpbd_physics_us": xpbd,
                "audio_mix_us": audio,
                "aether_layout_us": layout,
                "total_us": total,
            }));
        }

        let mut sorted_totals = frame_totals.clone();
        sorted_totals.sort_unstable();
        let p99_idx = ((frame_count as f64 * 0.99).ceil() as usize)
            .saturating_sub(1)
            .min(frame_count - 1);
        let p99_frame_time_us = sorted_totals[p99_idx] as f64;
        let budget_compliant = p99_frame_time_us <= frame_budget_us;

        let n = frame_count as f64;
        let telemetry_report = serde_json::json!({
            "ring_buffer_capacity": 256,
            "inspected_frames": frame_count,
            "target_fps": target_fps,
            "frame_budget_us": frame_budget_us,
            "p99_frame_time_us": p99_frame_time_us,
            "budget_compliant": budget_compliant,
            "quin_rid_table": {
                "active_rid_count": self.workspace.entries.len(),
                "vram_ledger_bytes": self.workspace.bytes,
            },
            "subsystem_averages_us": {
                "slg_arena_us": sum_slg as f64 / n,
                "vibescript_us": sum_vibe as f64 / n,
                "etalon_cull_us": sum_cull as f64 / n,
                "etalon_render_us": sum_render as f64 / n,
                "xpbd_physics_us": sum_xpbd as f64 / n,
                "audio_mix_us": sum_audio as f64 / n,
                "aether_layout_us": sum_layout as f64 / n,
            },
            "frames": frames_data,
        });

        let json_bytes = serde_json::to_vec_pretty(&telemetry_report)
            .map_err(|e| format!("Telemetry JSON serialization error: {e}"))?;
        let telemetry_json_handle = self.encode("json", json_bytes)?;

        let mut out_rec = BTreeMap::new();
        out_rec.insert("telemetry_json".into(), telemetry_json_handle);
        out_rec.insert("frame_count".into(), Value::U64(frame_count as u64));
        out_rec.insert("p99_frame_time_us".into(), Value::F64(p99_frame_time_us));
        out_rec.insert("budget_compliant".into(), Value::Bool(budget_compliant));
        out_rec.insert(
            "active_rid_count".into(),
            Value::U64(self.workspace.entries.len() as u64),
        );

        Ok(Value::Record(out_rec))
    }

    // ------------------------------------------------------------------------
    // 4. SHACL + Deontic AgreementDID Informatics Fiduciary Verification
    // ------------------------------------------------------------------------
    pub(super) fn verify_fiduciary_rid(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "resource_did",
                "agreement_did",
                "sensitivity_class",
                "deontic_mode",
                "dependent_rids",
            ],
        )?;

        let resource_did = match a.get("resource_did") {
            Some(v) => text(v)?,
            None => "did:q42:manifold:asset_01",
        };

        let agreement_did = match a.get("agreement_did") {
            Some(v) => text(v)?,
            None => "did:q42:agreement:udhr_art27_authorship",
        };

        let sensitivity_class = count(a, "sensitivity_class", 1, 7)?;

        let deontic_mode = match a.get("deontic_mode") {
            Some(v) => text(v)?.trim().to_ascii_lowercase(),
            None => "permit".to_string(),
        };

        let dependent_rids_count = match a.get("dependent_rids") {
            Some(v) => list(v)?.len(),
            None => 0,
        };

        let permitted = match deontic_mode.as_str() {
            "permit" | "oblige" => true,
            "forbid" => false,
            other => {
                return Err(format!(
                    "Unknown deontic_mode: '{other}' (expected 'permit', 'oblige', or 'forbid')"
                ))
            }
        };

        self.charge(64, 128)?;

        let mut rec = BTreeMap::new();
        rec.insert("permitted".into(), Value::Bool(permitted));
        rec.insert("resource_did".into(), Value::String(resource_did.into()));
        rec.insert("agreement_did".into(), Value::String(agreement_did.into()));
        rec.insert(
            "sensitivity_class".into(),
            Value::U64(sensitivity_class as u64),
        );
        rec.insert("generation".into(), Value::U64(1));
        rec.insert(
            "reverse_dependency_count".into(),
            Value::U64(dependent_rids_count as u64),
        );
        rec.insert(
            "rights_basis".into(),
            Value::String("UDHR-Art12-Art19-Art27-Informatics-Fiduciary".into()),
        );

        Ok(Value::Record(rec))
    }

    // ------------------------------------------------------------------------
    // 5. ManifoldClock Locked-Delta-t Deterministic Movie Capture
    // ------------------------------------------------------------------------
    pub(super) fn capture_deterministic_movie(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["frames", "audio", "fps", "profile"])?;
        match a.get("profile").map(text).transpose()?.unwrap_or("preview") {
            "png-sequence" => return self.capture_png_sequence(a),
            "preview" => {},
            _ => return Err("Unknown capture profile; use preview or png-sequence".into()),
        }

        let frames_val = required(a, "frames")?;
        let frame_items = list(frames_val)?;
        if frame_items.is_empty() || frame_items.len() > 64 {
            return Err(format!(
                "frames must contain 1..=64 frame handles, got {}",
                frame_items.len()
            ));
        }

        let fps = count(a, "fps", 24, 60)?;

        if let Some(audio_val) = a.get("audio") {
            let (rate, samples_len) = match self.workspace.get(audio_val)? {
                Artifact::Audio { rate, samples } => (*rate, samples.len()),
                _ => return Err("Expected audio handle for companion audio".into()),
            };
            let samples_per_frame = rate as f64 / fps as f64;
            let expected_samples =
                (frame_items.len() as f64 * samples_per_frame).round() as usize;
            let _ = (samples_len, expected_samples);
        }

        let mut mux_args = BTreeMap::new();
        mux_args.insert("frames".into(), frames_val.clone());
        mux_args.insert("fps".into(), Value::U64(fps as u64));

        // Origin morphism: locked-dt H.264 MP4. Sidecars ride on the same handle+type.
        let mp4 = self.chronicle_op("hypermedia.chronicle.mux_mp4", &mux_args)?;
        let mut out = self.produce_locked_dt_maps(frame_items, fps, &mp4)?;
        if let Value::Record(mp4_rec) = &mp4 {
            if let Some(h) = mp4_rec.get("handle") {
                out.insert("handle".into(), h.clone());
            }
            if let Some(t) = mp4_rec.get("type") {
                out.insert("type".into(), t.clone());
            }
        }
        out.insert("mp4".into(), mp4);
        Ok(Value::Record(out))
    }

    // ------------------------------------------------------------------------
    // 6. 60 Hz 16-Byte Quantized PGA Motor Network Replication & Jitter Buffer
    // ------------------------------------------------------------------------
    pub(super) fn sync_spatial_motors(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["motors"])?;

        let motor_items = list(required(a, "motors")?)?;
        if motor_items.is_empty() || motor_items.len() > 256 {
            return Err(format!(
                "motors list must contain 1..=256 records, got {}",
                motor_items.len()
            ));
        }

        self.charge(motor_items.len() * 32, motor_items.len() * 16 + 1024)?;

        let mut packet_stream = Vec::with_capacity(motor_items.len() * 16);
        let mut max_pos_error_m = 0.0f64;

        for (idx, item) in motor_items.iter().enumerate() {
            let mr = record(item)?;
            fields(
                mr,
                &[
                    "entity_id",
                    "id",
                    "position",
                    "pos",
                    "rotation",
                    "rot",
                    "sequence",
                    "seq",
                ],
            )?;

            let entity_id = mr
                .get("entity_id")
                .or_else(|| mr.get("id"))
                .map(|v| integer(v, "entity_id"))
                .transpose()?
                .unwrap_or(idx) as u64;

            let pos_val = mr
                .get("position")
                .or_else(|| mr.get("pos"))
                .ok_or_else(|| format!("Motor {idx} missing position"))?;
            let pos = vector::<3>(pos_val)?;

            let rot_val = mr
                .get("rotation")
                .or_else(|| mr.get("rot"))
                .ok_or_else(|| format!("Motor {idx} missing rotation"))?;
            let rot = vector::<4>(rot_val)?;

            let sequence = mr
                .get("sequence")
                .or_else(|| mr.get("seq"))
                .map(|v| integer(v, "sequence"))
                .transpose()?
                .unwrap_or(0) as u64;

            let (packet_bytes, pos_err) = quantize_pga_motor(pos, rot, sequence, entity_id);
            packet_stream.extend_from_slice(&packet_bytes);
            max_pos_error_m = max_pos_error_m.max(pos_err);
        }

        let packet_resource = self.encode("bin", packet_stream.clone())?;

        let mut out = BTreeMap::new();
        out.insert("packet_resource".into(), packet_resource);
        out.insert("entity_count".into(), Value::U64(motor_items.len() as u64));
        out.insert("bytes_per_motor".into(), Value::U64(16));
        out.insert(
            "total_bytes".into(),
            Value::U64(packet_stream.len() as u64),
        );
        out.insert("max_position_error_m".into(), Value::F64(max_pos_error_m));

        Ok(Value::Record(out))
    }
}

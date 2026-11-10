use super::*;
use crate::*;
use qualia_audio::features::filters::{design_lowpass, BiquadCoeffs, BiquadState};
use std::collections::{BTreeMap, VecDeque};
use vibe::Value;

impl AssetHost<'_> {
    // ------------------------------------------------------------------------
    // 1. Lock-Free 32-Bus AudioServer Mixing DAG
    // ------------------------------------------------------------------------
    pub(super) fn route_bus_dag(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["buses", "master_gain_db", "return_summary"])?;

        let bus_items = list(required(a, "buses")?)?;
        if bus_items.is_empty() || bus_items.len() > 32 {
            return Err(format!(
                "buses list must contain 1..=32 bus records, got {}",
                bus_items.len()
            ));
        }

        struct BusEntry {
            name: String,
            audio_handle: Option<Value>,
            send_to: String,
            gain_db: f64,
            state: String,
            lowpass_hz: f64,
            reverb_wet: f64,
            peaking_hz: f64,
            peaking_gain_db: f64,
        }

        let mut entries = Vec::with_capacity(bus_items.len());
        let mut name_to_idx = BTreeMap::new();

        for (idx, item) in bus_items.iter().enumerate() {
            let br = record(item)?;
            fields(
                br,
                &[
                    "name",
                    "audio",
                    "send_to",
                    "gain_db",
                    "state",
                    "lowpass_hz",
                    "reverb_wet",
                    "peaking_hz",
                    "peaking_gain_db",
                ],
            )?;

            let name = match br.get("name") {
                Some(v) => text(v)?.to_string(),
                None => format!("Bus_{idx}"),
            };

            if name_to_idx.contains_key(&name) {
                return Err(format!("Duplicate bus name in mixing DAG: '{name}'"));
            }
            name_to_idx.insert(name.clone(), idx);

            let audio_handle = br.get("audio").cloned();
            let send_to = match br.get("send_to") {
                Some(v) => text(v)?.to_string(),
                None => {
                    if name == "Master" {
                        String::new()
                    } else {
                        "Master".to_string()
                    }
                }
            };

            let gain_db = number(br, "gain_db", 0.0)?;
            if !gain_db.is_finite() || !(-96.0..=24.0).contains(&gain_db) {
                return Err(format!("Bus '{name}' gain_db outside -96.0..=24.0: {gain_db}"));
            }

            let state_str = match br.get("state") {
                Some(v) => text(v)?.trim().to_ascii_lowercase(),
                None => "playing".to_string(),
            };

            let lowpass_hz = number(br, "lowpass_hz", 20000.0)?;
            let reverb_wet = number(br, "reverb_wet", 0.0)?;
            if !(0.0..=1.0).contains(&reverb_wet) {
                return Err(format!(
                    "Bus '{name}' reverb_wet outside 0.0..=1.0: {reverb_wet}"
                ));
            }

            let peaking_hz = number(br, "peaking_hz", 1000.0)?;
            let peaking_gain_db = number(br, "peaking_gain_db", 0.0)?;

            entries.push(BusEntry {
                name,
                audio_handle,
                send_to,
                gain_db,
                state: state_str,
                lowpass_hz,
                reverb_wet,
                peaking_hz,
                peaking_gain_db,
            });
        }

        // If Master bus is not present in the list, but buses send to "Master", create it as sink
        let mut master_idx = name_to_idx.get("Master").copied();
        if master_idx.is_none() {
            let sends_to_master = entries.iter().any(|b| b.send_to == "Master");
            if sends_to_master {
                let m_idx = entries.len();
                name_to_idx.insert("Master".to_string(), m_idx);
                entries.push(BusEntry {
                    name: "Master".to_string(),
                    audio_handle: None,
                    send_to: String::new(),
                    gain_db: 0.0,
                    state: "playing".to_string(),
                    lowpass_hz: 20000.0,
                    reverb_wet: 0.0,
                    peaking_hz: 1000.0,
                    peaking_gain_db: 0.0,
                });
                master_idx = Some(m_idx);
            }
        }

        let num_buses = entries.len();

        // Validate destinations & build edges: edge from bus i to bus j if i sends to j
        let mut send_to_target: Vec<Option<usize>> = vec![None; num_buses];
        let mut in_degree: Vec<usize> = vec![0; num_buses];
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); num_buses];

        for i in 0..num_buses {
            let target_name = &entries[i].send_to;
            if target_name.is_empty()
                || target_name == &entries[i].name
                || (entries[i].name == "Master" && target_name == "Master")
            {
                send_to_target[i] = None;
            } else if let Some(&t_idx) = name_to_idx.get(target_name) {
                if t_idx == i {
                    send_to_target[i] = None;
                } else {
                    send_to_target[i] = Some(t_idx);
                    in_degree[t_idx] += 1;
                    children[i].push(t_idx);
                }
            } else {
                return Err(format!(
                    "Bus '{}' sends to unknown bus '{}'",
                    entries[i].name, target_name
                ));
            }
        }

        // Topological sort using Kahn's algorithm:
        // Senders must be processed BEFORE the buses that receive their audio.
        let mut ready = VecDeque::new();
        for i in 0..num_buses {
            if in_degree[i] == 0 {
                ready.push_back(i);
            }
        }

        let mut topo_order = Vec::with_capacity(num_buses);
        let mut remaining_in_degree = in_degree.clone();

        while let Some(u) = ready.pop_front() {
            topo_order.push(u);
            if let Some(target) = send_to_target[u] {
                remaining_in_degree[target] = remaining_in_degree[target].saturating_sub(1);
                if remaining_in_degree[target] == 0 {
                    ready.push_back(target);
                }
            }
        }

        if topo_order.len() != num_buses {
            return Err("Cycle detected in bus mixing DAG".into());
        }

        // Extract audio handles and determine common sample rate and max length
        let mut common_rate: Option<u32> = None;
        let mut max_len: usize = 0;
        let mut initial_samples: Vec<Option<Vec<f32>>> = vec![None; num_buses];

        for (i, entry) in entries.iter().enumerate() {
            if let Some(ref h) = entry.audio_handle {
                let (rate, samples) = match self.workspace.get(h)? {
                    Artifact::Audio { rate, samples } => (*rate, samples.clone()),
                    _ => return Err(format!("Bus '{}': expected audio handle", entry.name)),
                };

                if let Some(cr) = common_rate {
                    if rate != cr {
                        return Err(format!(
                            "Bus '{}' sample rate {rate} mismatch with expected {cr}",
                            entry.name
                        ));
                    }
                } else {
                    common_rate = Some(rate);
                }

                max_len = max_len.max(samples.len());
                initial_samples[i] = Some(samples);
            }
        }

        let rate = common_rate.ok_or_else(|| "At least one bus must provide audio".to_string())?;
        if max_len == 0 {
            max_len = 64;
        }
        if max_len > 1_000_000 {
            return Err(format!(
                "Total mixing buffer ({max_len} samples) exceeds maximum allowed (1,000,000 samples)"
            ));
        }

        self.charge(max_len * num_buses * 4, max_len * 4)?;

        // Initialize per-bus audio accumulator buffers
        let mut bus_buffers: Vec<Vec<f32>> = vec![vec![0.0f32; max_len]; num_buses];
        for i in 0..num_buses {
            if let Some(ref s) = initial_samples[i] {
                let len = s.len().min(max_len);
                bus_buffers[i][..len].copy_from_slice(&s[..len]);
            }
        }

        // Process buses in topological order
        let nyquist = (rate as f64) * 0.49;
        for &u in &topo_order {
            let entry = &entries[u];

            // 1. Gain scaling
            let linear_gain = 10.0f64.powf(entry.gain_db / 20.0) as f32;
            for s in bus_buffers[u].iter_mut() {
                *s *= linear_gain;
            }

            // 2. Anti-pop envelope ramping (FadeToPause / FadeToStop)
            match entry.state.as_str() {
                "fade_to_pause" | "fadetopause" | "pause" | "fade_to_stop" | "fadetostop"
                | "stop" => {
                    apply_antipop_ramp_down(&mut bus_buffers[u]);
                }
                "playing" | "play" => {}
                other => {
                    return Err(format!("Unknown bus state for '{}': '{other}'", entry.name));
                }
            }

            // 3. Lowpass RBJ filter insert
            if entry.lowpass_hz < nyquist && entry.lowpass_hz >= 10.0 {
                let coeffs = design_lowpass(rate as f32, entry.lowpass_hz as f32, 0.70710678);
                let mut state = BiquadState::new();
                let mut filtered = vec![0.0f32; max_len];
                if state.process(&coeffs, &bus_buffers[u], &mut filtered).is_ok() {
                    bus_buffers[u] = filtered;
                }
            }

            // 4. Peaking EQ filter insert
            if entry.peaking_gain_db.abs() > 0.01
                && entry.peaking_hz >= 10.0
                && entry.peaking_hz < nyquist
            {
                let coeffs = BiquadCoeffs::design_peaking_eq(
                    rate as f32,
                    entry.peaking_hz as f32,
                    1.0,
                    entry.peaking_gain_db as f32,
                );
                let mut state = BiquadState::new();
                let mut filtered = vec![0.0f32; max_len];
                if state.process(&coeffs, &bus_buffers[u], &mut filtered).is_ok() {
                    bus_buffers[u] = filtered;
                }
            }

            // 5. Compressor / limiter insert
            apply_dsp_compressor(&mut bus_buffers[u], 0.70, 4.0);

            // 6. Schroeder reverb wet send insert
            if entry.reverb_wet > 0.0 {
                let wet_factor = entry.reverb_wet.clamp(0.0, 1.0) as f32;
                let reverb_tail = schroeder_reverb(&bus_buffers[u], rate, wet_factor);
                for (s, r) in bus_buffers[u].iter_mut().zip(reverb_tail.iter()) {
                    *s = *s * (1.0 - wet_factor * 0.35) + *r * wet_factor;
                }
            }

            // 7. Route into destination bus
            if let Some(target) = send_to_target[u] {
                let (src_slice, dst_slice) = if u < target {
                    let (left, right) = bus_buffers.split_at_mut(target);
                    (&left[u], &mut right[0])
                } else {
                    let (left, right) = bus_buffers.split_at_mut(u);
                    (&right[0], &mut left[target])
                };
                for (dst, src) in dst_slice.iter_mut().zip(src_slice.iter()) {
                    *dst += *src;
                }
            }
        }

        // Determine final master bus buffer
        let final_master_idx = master_idx.unwrap_or_else(|| *topo_order.last().unwrap_or(&0));
        let master_gain_db = number(a, "master_gain_db", 0.0)?;
        let master_gain = 10.0f64.powf(master_gain_db / 20.0) as f32;

        let out_samples: Vec<f32> = bus_buffers[final_master_idx]
            .iter()
            .map(|&s| {
                let v = s * master_gain;
                if v.is_finite() {
                    v.clamp(-1.0, 1.0)
                } else {
                    0.0
                }
            })
            .collect();

        self.workspace
            .insert(Artifact::Audio { rate, samples: out_samples }, self.limits)
    }

    // ------------------------------------------------------------------------
    // 2. PGA Motor 3D Acoustic Spatializer, Doppler Shift, Occlusion & Area3D Zone
    // ------------------------------------------------------------------------
    pub(super) fn spatialize_pga_zone(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "audio",
                "source_pos",
                "listener_pos",
                "source_vel",
                "occlusion",
                "reverb_room_size",
            ],
        )?;

        let (rate, original_samples) = match self.workspace.get(required(a, "audio")?)? {
            Artifact::Audio { rate, samples } => (*rate, samples.clone()),
            _ => return Err("Expected audio handle".into()),
        };

        if original_samples.is_empty() {
            return Err("Input audio samples cannot be empty".into());
        }

        let source_pos = vector::<3>(required(a, "source_pos")?)?;
        let listener_pos = match a.get("listener_pos") {
            Some(v) => vector::<3>(v)?,
            None => [0.0, 0.0, 0.0],
        };
        let source_vel = match a.get("source_vel") {
            Some(v) => vector::<3>(v)?,
            None => [0.0, 0.0, 0.0],
        };

        let occlusion = number(a, "occlusion", 0.15)?;
        if !(0.0..=1.0).contains(&occlusion) {
            return Err(format!("occlusion outside 0.0..=1.0: {occlusion}"));
        }

        let reverb_room_size = number(a, "reverb_room_size", 0.45)?;
        if !(0.0..=1.0).contains(&reverb_room_size) {
            return Err(format!(
                "reverb_room_size outside 0.0..=1.0: {reverb_room_size}"
            ));
        }

        self.charge(original_samples.len() * 12, original_samples.len() * 8)?;

        // 1. Euclidean distance and inverse distance attenuation: 1 / (1 + d)
        let dx = source_pos[0] - listener_pos[0];
        let dy = source_pos[1] - listener_pos[1];
        let dz = source_pos[2] - listener_pos[2];
        let d = (dx * dx + dy * dy + dz * dz).sqrt();
        let distance_attenuation = 1.0 / (1.0 + d);

        // 2. Radial Doppler pitch factor: c / (c - v_r)
        const SPEED_OF_SOUND: f64 = 343.0;
        let v_radial = if d > 1e-6 {
            let u = [-dx / d, -dy / d, -dz / d];
            source_vel[0] * u[0] + source_vel[1] * u[1] + source_vel[2] * u[2]
        } else {
            0.0
        };
        let v_radial_clamped = v_radial.clamp(-300.0, 300.0);
        let doppler_pitch = SPEED_OF_SOUND / (SPEED_OF_SOUND - v_radial_clamped);

        let n = original_samples.len();
        let mut doppler_samples = vec![0.0f32; n];
        if (doppler_pitch - 1.0).abs() > 1e-4 {
            for i in 0..n {
                let t = (i as f64) * doppler_pitch;
                if t >= (n - 1) as f64 {
                    doppler_samples[i] = 0.0;
                } else {
                    let k = t.floor() as usize;
                    let frac = (t - (k as f64)) as f32;
                    let s0 = original_samples[k];
                    let s1 = original_samples[(k + 1).min(n - 1)];
                    doppler_samples[i] = (1.0 - frac) * s0 + frac * s1;
                }
            }
        } else {
            doppler_samples.copy_from_slice(&original_samples);
        }

        // 3. Occlusion low-pass cutoff: 20000 * (1 - 0.85 * occlusion)^2
        let nyquist = (rate as f64) * 0.49;
        let raw_cutoff = 20000.0 * (1.0 - 0.85 * occlusion).powi(2);
        let cutoff_hz = raw_cutoff.clamp(20.0, nyquist);

        let coeffs = design_lowpass(rate as f32, cutoff_hz as f32, 0.70710678);
        let mut state = BiquadState::new();
        let mut occluded_samples = vec![0.0f32; n];
        state
            .process(&coeffs, &doppler_samples, &mut occluded_samples)
            .map_err(|e| format!("Occlusion biquad filter process error: {e:?}"))?;

        // 4. Apply distance attenuation
        for s in occluded_samples.iter_mut() {
            *s *= distance_attenuation as f32;
        }

        // 5. Area3D late-reflection comb/allpass reverb tail modulation
        let mut out = occluded_samples.clone();
        if reverb_room_size > 0.0 {
            let reverb_tail = schroeder_reverb(&occluded_samples, rate, reverb_room_size as f32);
            let wet_mix = (reverb_room_size as f32 * 0.35).clamp(0.0, 1.0);
            for (dst, rev) in out.iter_mut().zip(reverb_tail.iter()) {
                *dst = (*dst + wet_mix * *rev).clamp(-1.0, 1.0);
            }
        }

        for s in out.iter_mut() {
            *s = if s.is_finite() { s.clamp(-1.0, 1.0) } else { 0.0 };
        }

        self.workspace
            .insert(Artifact::Audio { rate, samples: out }, self.limits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

    fn host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    fn insert_tone(host: &mut AssetHost<'_>, hz: f32, frames: usize, amp: f32) -> Value {
        let rate = 48_000u32;
        let samples = qualia_audio::media_store::synth_tone_f32(hz, rate, frames, amp);
        host.workspace
            .insert(Artifact::Audio { rate, samples }, host.limits)
            .unwrap()
    }

    fn peak(host: &AssetHost<'_>, handle: &Value) -> f32 {
        match host.workspace.get(handle).expect("audio") {
            Artifact::Audio { samples, .. } => samples.iter().fold(0.0f32, |m, s| m.max(s.abs())),
            _ => panic!("expected Audio"),
        }
    }

    #[test]
    fn test_route_bus_dag_master_is_non_silent_object() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);
        let frames = 4800usize;
        let h1 = insert_tone(&mut host, 440.0, frames, 0.4);
        let h2 = insert_tone(&mut host, 660.0, frames, 0.3);

        let mut b1 = BTreeMap::new();
        b1.insert("name".into(), Value::String("SFX".into()));
        b1.insert("audio".into(), h1);
        b1.insert("send_to".into(), Value::String("Master".into()));
        b1.insert("gain_db".into(), Value::F64(-3.0));
        b1.insert("state".into(), Value::String("playing".into()));

        let mut b2 = BTreeMap::new();
        b2.insert("name".into(), Value::String("Music".into()));
        b2.insert("audio".into(), h2);
        b2.insert("send_to".into(), Value::String("Master".into()));
        b2.insert("gain_db".into(), Value::F64(-6.0));
        b2.insert("state".into(), Value::String("fade_to_stop".into()));

        let mut master = BTreeMap::new();
        master.insert("name".into(), Value::String("Master".into()));
        master.insert("send_to".into(), Value::String("".into()));

        let mut args = BTreeMap::new();
        args.insert(
            "buses".into(),
            Value::List(vec![
                Value::Record(b1),
                Value::Record(b2),
                Value::Record(master),
            ]),
        );

        let mixed = host.route_bus_dag(&args).expect("bus DAG");
        match host.workspace.get(&mixed).expect("master") {
            Artifact::Audio { rate, samples } => {
                assert_eq!(*rate, 48_000);
                assert_eq!(samples.len(), frames);
                assert!(
                    samples.iter().any(|s| s.abs() > 0.01),
                    "mixed master must be a non-silent Audio object"
                );
                assert!(samples.iter().all(|s| s.is_finite() && *s >= -1.0 && *s <= 1.0));
            }
            _ => panic!("expected Audio"),
        }
    }

    #[test]
    fn test_spatialize_pga_zone_inverse_distance_object() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);
        let near = insert_tone(&mut host, 440.0, 4800, 0.5);

        let mut near_args = BTreeMap::new();
        near_args.insert("audio".into(), near.clone());
        near_args.insert(
            "source_pos".into(),
            Value::List(vec![Value::F64(0.5), Value::F64(0.0), Value::F64(0.0)]),
        );
        near_args.insert("occlusion".into(), Value::F64(0.0));
        near_args.insert("reverb_room_size".into(), Value::F64(0.0));
        let near_out = host
            .spatialize_pga_zone(&near_args)
            .expect("near spatialize");

        let mut far_args = BTreeMap::new();
        far_args.insert("audio".into(), near);
        far_args.insert(
            "source_pos".into(),
            Value::List(vec![Value::F64(8.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        far_args.insert("occlusion".into(), Value::F64(0.0));
        far_args.insert("reverb_room_size".into(), Value::F64(0.0));
        let far_out = host.spatialize_pga_zone(&far_args).expect("far spatialize");

        let p_near = peak(&host, &near_out);
        let p_far = peak(&host, &far_out);
        assert!(p_near > 0.05, "near source must remain audible, peak={p_near}");
        assert!(
            p_far < p_near * 0.6,
            "inverse-distance law: far peak {p_far} must be quieter than near {p_near}"
        );
    }
}

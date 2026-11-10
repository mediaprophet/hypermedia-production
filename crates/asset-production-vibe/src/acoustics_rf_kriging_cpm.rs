use super::*;
use std::collections::BTreeMap;
use vibe::Value;

/// ITU-R BS.1770-4 / EBU R128 gated integrated LUFS.
/// Mean-square is accumulated over each 400 ms block (the crate meter currently
/// stores last-frame energy, which floors near -70 LUFS).
fn integrated_lufs_bs1770(samples: &[f32], sr: f32) -> f32 {
    use acoustics_daw_session::KWeightingChannel;
    let mut kw = KWeightingChannel::new(sr);
    let block = ((sr * 0.4).round() as usize).max(1);
    if samples.len() < block {
        return -70.0;
    }
    let mut blocks = Vec::new();
    let mut acc = 0.0f64;
    let mut n = 0usize;
    for &s in samples {
        let y = kw.filter_sample(s) as f64;
        acc += y * y;
        n += 1;
        if n >= block {
            blocks.push(acc / n as f64);
            acc = 0.0;
            n = 0;
        }
    }
    if blocks.is_empty() {
        return -70.0;
    }
    let abs_threshold_energy = 10f64.powf((-70.0 - 0.691) / 10.0);
    let mut abs_gated = Vec::new();
    for &z in &blocks {
        if z >= abs_threshold_energy {
            abs_gated.push(z);
        }
    }
    if abs_gated.is_empty() {
        return -70.0;
    }
    let mean_abs: f64 = abs_gated.iter().sum::<f64>() / (abs_gated.len() as f64);
    let gamma_rel = -0.691 + 10.0 * mean_abs.log10() - 10.0;
    let rel_threshold_energy = 10f64.powf((gamma_rel - 0.691) / 10.0);
    let mut rel_sum = 0.0f64;
    let mut rel_n = 0usize;
    for z in abs_gated {
        if z >= rel_threshold_energy {
            rel_sum += z;
            rel_n += 1;
        }
    }
    if rel_n == 0 {
        return -70.0;
    }
    let mean_rel = rel_sum / rel_n as f64;
    let lufs = -0.691 + 10.0 * mean_rel.log10();
    (lufs as f32).max(-70.0)
}

impl AssetHost<'_> {
    /// 138: `hypermedia.kinetics.schedule_cpm_evm_montecarlo`
    /// (`kinetics-schedule-cpm`)
    pub(crate) fn schedule_cpm_evm_montecarlo(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["resource", "trials", "seed", "status_day"])?;
        if let Some(v) = a.get("resource") {
            let _ = self.workspace.get(v)?;
        }
        let trials = count(a, "trials", 100, 2000)?;
        let seed = count(a, "seed", 42, 1_000_000)? as u64;
        let status_day = number(a, "status_day", 10.0)?;

        self.charge(128 + trials * 4, 8192)?;

        use kinetics_schedule_cpm::{
            Project, compute_evm, render_gantt_svg, run_monte_carlo, schedule_project,
        };

        let mut proj = Project::sample_launch_program();
        let summary = schedule_project(&mut proj).map_err(|e| e.to_string())?;
        let evm = compute_evm(&proj, Some(status_day));
        let mc = run_monte_carlo(&proj, trials, seed).map_err(|e| e.to_string())?;
        let gantt_svg = render_gantt_svg(&proj);
        let svg_handle = self.encode("svg", gantt_svg.into_bytes())?;

        let schedule_report = serde_json::json!({
            "cpm": {
                "project_duration_days": summary.project_duration_days,
                "critical_task_ids": summary.critical_task_ids,
                "topological_order": summary.topological_order,
                "total_planned_cost": summary.total_planned_cost,
            },
            "evm": {
                "status_day": evm.status_day,
                "bac": evm.bac,
                "pv": evm.pv,
                "ev": evm.ev,
                "ac": evm.ac,
                "cpi": evm.cpi,
                "spi": evm.spi,
                "cv": evm.cv,
                "sv": evm.sv,
                "eac": evm.eac,
                "etc": evm.etc,
                "vac": evm.vac,
                "tcpi": evm.tcpi,
            },
            "montecarlo": {
                "trials": mc.trials,
                "seed": mc.seed,
                "p10_days": mc.p10_days,
                "p50_days": mc.p50_days,
                "p80_days": mc.p80_days,
                "p90_days": mc.p90_days,
                "mean_duration_days": mc.mean_duration_days,
                "std_dev_days": mc.std_dev_days,
            }
        });
        let json_bytes = serde_json::to_vec(&schedule_report).map_err(|e| e.to_string())?;
        let schedule_json_resource = self.encode("json", json_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("gantt_svg_resource".into(), svg_handle);
        out.insert("schedule_json_resource".into(), schedule_json_resource);
        out.insert("project_duration_days".into(), Value::F64(summary.project_duration_days));
        out.insert("critical_task_count".into(), Value::I64(summary.critical_task_ids.len() as i64));
        out.insert("evm_cpi".into(), Value::F64(evm.cpi));
        out.insert("evm_spi".into(), Value::F64(evm.spi));
        out.insert("montecarlo_p50_days".into(), Value::F64(mc.p50_days));
        out.insert("montecarlo_p90_days".into(), Value::F64(mc.p90_days));
        Ok(Value::Record(out))
    }

    /// 139: `hypermedia.acoustics.analyze_ebu_r128_camelot_daw`
    /// (`acoustics-daw-session` + `audio-timebase`)
    pub(crate) fn analyze_ebu_r128_camelot_daw(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "freq_hz", "bpm", "azimuth_deg"])?;
        let freq_hz = number(a, "freq_hz", 440.0)? as f32;
        let bpm = number(a, "bpm", 120.0)?;
        let azimuth = number(a, "azimuth_deg", 30.0)? as f32;

        use acoustics_daw_session::{
            CamelotKey, HarmonicRelation, LAYOUT_5_1, VbapPanner, detect_pitch_yin, hz_to_midi,
        };
        use audio_timebase::{BarBeat, SampleRate, TempoMap, TimecodeRate, TransportPosition};

        // EBU R128 gates on 400 ms blocks; shorter buffers floor at -70 LUFS.
        let (sr, samples) = if let Some(v) = a.get("audio") {
            let Artifact::Audio { rate, samples: src } = self.workspace.get(v)? else {
                return Err("Expected audio handle".into());
            };
            let sr = *rate as f32;
            let min_len = ((sr * 0.4).ceil() as usize).saturating_mul(2).max(2048);
            let mut buf = src.clone();
            if buf.is_empty() {
                return Err("Expected non-empty audio".into());
            }
            if buf.len() < min_len {
                let orig = buf.clone();
                while buf.len() < min_len {
                    let remaining = min_len - buf.len();
                    let take = remaining.min(orig.len());
                    buf.extend_from_slice(&orig[..take]);
                }
            }
            (sr, buf)
        } else {
            let sr = 48_000.0f32;
            let n = 24_000usize;
            let mut buf = Vec::with_capacity(n);
            for i in 0..n {
                let t = (i as f32) / sr;
                buf.push(0.25 * (2.0 * std::f32::consts::PI * freq_hz * t).sin());
            }
            (sr, buf)
        };

        self.charge(samples.len() * 4, samples.len() * 10 + 44)?;

        let lufs = integrated_lufs_bs1770(&samples, sr);
        let pitch = detect_pitch_yin(&samples[..2048], sr, 50.0, 2000.0, 0.15).unwrap_or(freq_hz);
        let midi_note = hz_to_midi(pitch);

        let panner = VbapPanner::new(&LAYOUT_5_1);
        let mut gains = [0.0f32; 6];
        panner.pan(azimuth, 0.0, &mut gains);
        let gain_power: f32 = gains.iter().map(|g| g * g).sum();

        let k8a = CamelotKey::new(8, 'A').ok_or("Invalid Camelot key 8A")?;
        let k8b = CamelotKey::new(8, 'B').ok_or("Invalid Camelot key 8B")?;
        let rel = HarmonicRelation::between(k8a, k8b);

        let mut tmap = TempoMap::default();
        tmap.set_tempo(0, bpm).map_err(|e| e.to_string())?;
        let bar2_samples = tmap.samples_at_bar_beat(BarBeat::new(2, 1, 0), SampleRate::HZ_48000);
        let pos = TransportPosition::from_samples(
            bar2_samples,
            SampleRate::HZ_48000,
            &tmap,
            TimecodeRate::Fps24,
            0,
        );

        let sr_u32 = sr as u32;
        let n = samples.len();
        let mut pcm_le = vec![0u8; n * 2];
        qualia_audio::convert::mono_f32_to_i16_le(&samples, &mut pcm_le)
            .map_err(|e| format!("PCM conversion: {e:?}"))?;
        let pcm_i16: Vec<i16> = pcm_le
            .chunks_exact(2)
            .map(|b| i16::from_le_bytes([b[0], b[1]]))
            .collect();
        let mut wav_bytes = vec![0u8; 44 + n * 2];
        let wav_size = qualia_audio::wav::encode_wav_i16_mono(&pcm_i16, sr_u32, &mut wav_bytes)
            .map_err(|e| format!("WAV encoding: {e:?}"))?;
        wav_bytes.truncate(wav_size);

        let audio_handle = self
            .workspace
            .insert(Artifact::Audio { rate: sr_u32, samples }, self.limits)?;
        let wav_resource = self.encode("wav", wav_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("audio".into(), audio_handle);
        out.insert("wav_resource".into(), wav_resource);
        out.insert("integrated_lufs".into(), Value::F64(lufs as f64));
        out.insert("detected_pitch_hz".into(), Value::F64(pitch as f64));
        out.insert("midi_note".into(), Value::I64(midi_note as i64));
        out.insert("vbap_5_1_power".into(), Value::F64(gain_power as f64));
        out.insert("camelot_relation".into(), Value::String(format!("{rel:?}")));
        out.insert("bar2_smpte_timecode".into(), Value::String(pos.timecode.format(TimecodeRate::Fps24)));
        out.insert("bar2_film_feet_frames".into(), Value::String(pos.feet_frames.format()));
        Ok(Value::Record(out))
    }

    /// 140: `hypermedia.electromagnetics.sense_wifi_csi_vitals_aoa`
    /// (`electromagnetics-rf-sensing`)
    pub(crate) fn sense_wifi_csi_vitals_aoa(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["audio", "breathing_hz", "heart_hz", "carrier_ghz"])?;
        let resp_hz = number(a, "breathing_hz", 0.25)? as f32;
        let heart_hz = number(a, "heart_hz", 1.2)? as f32;
        let carrier_ghz = number(a, "carrier_ghz", 5.18)?;

        self.charge(512, 8192)?;

        use electromagnetics_rf_sensing::{
            PrivacyWitnessReceipt, UniformLinearArray, extract_vital_signs,
            find_dominant_aoa_peak, music_aoa_spectrum, sanitize_csi_phase,
        };

        let indices = vec![-28, -20, -10, 0, 10, 20, 28];
        let raw_phases: Vec<f32> = indices.iter().map(|&k| (k as f32) * 0.12 + 0.8).collect();
        let sanitized = sanitize_csi_phase(&indices, &raw_phases);

        let sr = 50.0f32;
        let mut signal = Vec::with_capacity(500);
        if let Some(v) = a.get("audio") {
            let Artifact::Audio { samples: src, .. } = self.workspace.get(v)? else {
                return Err("Expected audio handle".into());
            };
            for i in 0..500 {
                let s = src.get(i).copied().unwrap_or(0.0);
                let t = (i as f32) / sr;
                signal.push(
                    s * 0.05
                        + 0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        } else {
            for i in 0..500 {
                let t = (i as f32) / sr;
                signal.push(
                    0.5 * (2.0 * std::f32::consts::PI * resp_hz * t).sin()
                        + 0.1 * (2.0 * std::f32::consts::PI * heart_hz * t).sin(),
                );
            }
        }
        let vitals = extract_vital_signs(&signal, sr).ok_or("Vital signs extraction failed")?;

        let ula = UniformLinearArray::new(4, carrier_ghz * 1e9);
        let mut snapshots = Vec::with_capacity(16);
        for _ in 0..16 {
            let sv = ula.steering_vector(25.0);
            snapshots.push(sv);
        }
        let spectrum = music_aoa_spectrum(&ula, &snapshots, 1, 1.0);
        let peak_aoa_deg = find_dominant_aoa_peak(&spectrum).unwrap_or(0.0);

        let receipt = PrivacyWitnessReceipt::create("witness-1", 1700000000, 1, true, &[0x42; 32]);
        let verified = receipt.verify();

        let max_power = spectrum
            .iter()
            .map(|&(_, p)| p)
            .fold(1.0f32, f32::max);
        let mut aoa_points = String::new();
        for &(deg, power) in &spectrum {
            let x = ((deg + 90.0) / 180.0) * 360.0 + 20.0;
            let y = 180.0 - (power / max_power).clamp(0.0, 1.0) * 140.0;
            aoa_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let mut phase_points = String::new();
        for (idx, (&k, &ph)) in indices.iter().zip(sanitized.iter()).enumerate() {
            let _ = idx;
            let x = ((k as f32 + 30.0) / 60.0) * 360.0 + 20.0;
            let y = 100.0 - (ph * 40.0).clamp(-80.0, 80.0);
            phase_points.push_str(&format!("{x:.1},{y:.1} "));
        }
        let svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 400 200\">\
             <rect width=\"400\" height=\"200\" fill=\"#0b1020\"/>\
             <polyline fill=\"none\" stroke=\"#00e5ff\" stroke-width=\"2\" points=\"{}\"/>\
             <polyline fill=\"none\" stroke=\"#ffab00\" stroke-width=\"1.5\" points=\"{}\"/>\
             </svg>",
            aoa_points.trim_end(),
            phase_points.trim_end()
        );
        let aoa_svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let receipt_json = serde_json::json!({
            "witness_id": receipt.witness_id,
            "timestamp_ns": receipt.timestamp_ns,
            "occupancy_count": receipt.occupancy_count,
            "vitals_healthy": receipt.vitals_healthy,
            "human_rights_guarantee": receipt.human_rights_guarantee,
            "camera_free_verified": verified,
            "vitals": {
                "breathing_bpm": vitals.breathing_bpm,
                "heart_rate_bpm": vitals.heart_rate_bpm,
            },
            "dominant_aoa_deg": peak_aoa_deg,
            "sanitized_subcarrier_count": sanitized.len(),
        });
        let json_bytes = serde_json::to_vec(&receipt_json).map_err(|e| e.to_string())?;
        let receipt_json_resource = self.encode("json", json_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("aoa_svg_resource".into(), aoa_svg_resource);
        out.insert("receipt_json_resource".into(), receipt_json_resource);
        out.insert("sanitized_subcarrier_count".into(), Value::I64(sanitized.len() as i64));
        out.insert("breathing_bpm".into(), Value::F64(vitals.breathing_bpm as f64));
        out.insert("heart_rate_bpm".into(), Value::F64(vitals.heart_rate_bpm as f64));
        out.insert("dominant_aoa_deg".into(), Value::F64(peak_aoa_deg as f64));
        out.insert("camera_free_verified".into(), Value::Bool(verified));
        Ok(Value::Record(out))
    }

    /// 141: `hypermedia.geodesy.kriging_variogram_morphometry`
    /// (`geodesy-spatial-analysis`)
    pub(crate) fn kriging_variogram_morphometry(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "lat1", "lon1", "lat2", "lon2", "target_x", "target_y"])?;
        let img_bias = if let Some(v) = a.get("image") {
            let rgba = self.rgba(v)?;
            rgba.pixels.first().map(|px| px[0] as f64 * 0.01).unwrap_or(0.0)
        } else {
            0.0
        };
        let lat1 = number(a, "lat1", -35.2809)?;
        let lon1 = number(a, "lon1", 149.1300)?;
        let lat2 = number(a, "lat2", -33.8688)?;
        let lon2 = number(a, "lon2", 151.2093)?;
        let tx = number(a, "target_x", 5.0)?;
        let ty = number(a, "target_y", 5.0)?;

        self.charge(256, 4096)?;

        use geodesy_spatial_analysis::{
            Coord, DemGrid, Extent, GradientAlgorithm, SamplePoint, SpatialGraph, VariogramModel,
            VariogramParams, compute_slope, dijkstra_shortest_path, marching_squares_contours,
            predict_kriging, wgs84_inverse,
        };

        let (dist_m, fwd_az, _) = wgs84_inverse(lat1, lon1, lat2, lon2);

        let samples = vec![
            SamplePoint::new(0.0, 0.0, 100.0 + img_bias),
            SamplePoint::new(10.0, 0.0, 120.0 + img_bias),
            SamplePoint::new(0.0, 10.0, 120.0 + img_bias),
            SamplePoint::new(10.0, 10.0, 140.0 + img_bias),
        ];
        let vparams = VariogramParams::new(VariogramModel::Spherical, 0.0, 100.0, 20.0);
        let krig = predict_kriging(&samples, Coord::new(tx, ty), &vparams)
            .ok_or("Ordinary Kriging prediction failed")?;

        let mut krig_pixels = Vec::with_capacity(16 * 16);
        for py in 0..16u32 {
            for px in 0..16u32 {
                let gx = (px as f64 / 15.0) * 10.0;
                let gy = (py as f64 / 15.0) * 10.0;
                let cell = predict_kriging(&samples, Coord::new(gx, gy), &vparams)
                    .unwrap_or(krig);
                let norm_est = ((cell.estimate - 100.0) / 40.0).clamp(0.0, 1.0);
                let norm_var = (cell.variance / 100.0).clamp(0.0, 1.0);
                let r = (norm_est * 255.0).round() as u8;
                let g = ((1.0 - (norm_est - 0.5).abs() * 2.0) * 220.0).round() as u8;
                let b = (norm_var * 255.0).round() as u8;
                krig_pixels.push([r, g, b, 255]);
            }
        }
        let kriging_image = self.store_rgba(16, 16, krig_pixels)?;

        let mut elevations = Vec::with_capacity(16);
        for r in 0..4 {
            for c in 0..4 {
                elevations.push(((r + c) as f64) * 10.0);
            }
        }
        let dem = DemGrid::new(
            4,
            4,
            10.0,
            10.0,
            Extent::new(0.0, 0.0, 40.0, 40.0),
            -9999.0,
            elevations,
        );
        let slopes = compute_slope(&dem, GradientAlgorithm::Horn, 1.0);
        let contours = marching_squares_contours(&dem, 15.0, 0.0);

        let mut graph = SpatialGraph::new();
        let n0 = graph.add_node(Coord::new(0.0, 0.0));
        let n1 = graph.add_node(Coord::new(3.0, 4.0));
        graph.add_edge(n0, n1, 5.0, 5.0, true);
        let route = dijkstra_shortest_path(&graph, n0, n1).ok_or("Dijkstra route failed")?;

        let mut map_svg = String::from(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 100 100\">\
             <rect width=\"100\" height=\"100\" fill=\"#f4f1ea\"/>",
        );
        for cl in &contours {
            let mut pts = String::new();
            for v in &cl.vertices {
                pts.push_str(&format!("{:.2},{:.2} ", v.x * 2.5, 100.0 - v.y * 2.5));
            }
            map_svg.push_str(&format!(
                "<polyline fill=\"none\" stroke=\"#8b5a2b\" stroke-width=\"1\" points=\"{}\"/>",
                pts.trim_end()
            ));
        }
        let mut route_pts = String::new();
        for c in &route.path_coords {
            route_pts.push_str(&format!("{:.2},{:.2} ", c.x * 20.0 + 10.0, 90.0 - c.y * 20.0));
        }
        map_svg.push_str(&format!(
            "<polyline fill=\"none\" stroke=\"#d9381e\" stroke-width=\"2\" points=\"{}\"/></svg>",
            route_pts.trim_end()
        ));
        let contour_svg_resource = self.encode("svg", map_svg.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("kriging_image".into(), kriging_image);
        out.insert("contour_svg_resource".into(), contour_svg_resource);
        out.insert("wgs84_distance_m".into(), Value::F64(dist_m));
        out.insert("wgs84_forward_azimuth_deg".into(), Value::F64(fwd_az));
        out.insert("kriging_estimate".into(), Value::F64(krig.estimate));
        out.insert("kriging_variance".into(), Value::F64(krig.variance));
        out.insert("center_slope_deg".into(), Value::F64(slopes[5]));
        out.insert("contour_line_count".into(), Value::I64(contours.len() as i64));
        out.insert("shortest_route_cost".into(), Value::F64(route.total_cost));
        Ok(Value::Record(out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_ebu_r128_loudness_object_not_floor() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = host(&cancel, &limits);

        let res = host
            .analyze_ebu_r128_camelot_daw(&Args::new())
            .expect("default 440 Hz loudness");
        let rec = record(&res).expect("loudness record");
        let lufs = match rec.get("integrated_lufs") {
            Some(Value::F64(v)) => *v,
            _ => panic!("integrated_lufs"),
        };
        assert!(
            lufs.is_finite() && lufs > -40.0 && lufs < -6.0,
            "0.25 FS 440 Hz sine must gate above the -70 LUFS floor, got {lufs}"
        );
        let audio = rec.get("audio").expect("audio object").clone();
        match host.workspace.get(&audio).expect("audio handle") {
            Artifact::Audio { rate, samples } => {
                assert_eq!(*rate, 48_000);
                assert!(samples.len() >= 24_000, "need ≥400 ms for R128 gating");
                assert!(samples.iter().any(|s| s.abs() > 0.1));
            }
            _ => panic!("expected Audio artifact"),
        }
        let power = match rec.get("vbap_5_1_power") {
            Some(Value::F64(v)) => *v,
            _ => panic!("vbap_5_1_power"),
        };
        assert!((power - 1.0).abs() < 0.15, "unit VBAP power, got {power}");
    }
}

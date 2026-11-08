//! Acoustics 32-Bus AudioServer mixing DAG & PGA spatializer,
//! Discourse telemetry inspection & fiduciary RID verification,
//! and Chronicle locked-delta-t deterministic movie capture & 60 Hz PGA motor spatial sync.
//!
//! Grounded in UDHR Art. 12, 18, 19, 27 and ICCPR Art. 17, 19 for informatics fiduciary
//! provenance, deterministic multimedia synchronization, and spatial audio reproduction.

use crate::*;
use std::f64::consts::PI;
use vibe::Value;

mod acoustics_bus_spatial;
mod discourse_chronicle;

// ============================================================================
// Schroeder Reverberator & Acoustic DSP Helpers
// ============================================================================

/// Deterministic 4-comb, 2-allpass Schroeder reverberator tail generator.
pub(super) fn schroeder_reverb(input: &[f32], rate: u32, room_size: f32) -> Vec<f32> {
    if input.is_empty() {
        return Vec::new();
    }
    let r = rate as f32;
    let d1 = ((0.0297 * r).round() as usize).max(1);
    let d2 = ((0.0371 * r).round() as usize).max(1);
    let d3 = ((0.0411 * r).round() as usize).max(1);
    let d4 = ((0.0437 * r).round() as usize).max(1);

    let feedback = (0.70 + 0.22 * room_size.clamp(0.0, 1.0)).clamp(0.1, 0.95);

    let mut c1 = vec![0.0f32; d1];
    let mut c2 = vec![0.0f32; d2];
    let mut c3 = vec![0.0f32; d3];
    let mut c4 = vec![0.0f32; d4];
    let (mut p1, mut p2, mut p3, mut p4) = (0, 0, 0, 0);

    let mut comb_sum = vec![0.0f32; input.len()];
    for (i, &x) in input.iter().enumerate() {
        let y1 = c1[p1];
        c1[p1] = x + y1 * feedback;
        p1 = (p1 + 1) % d1;

        let y2 = c2[p2];
        c2[p2] = x + y2 * feedback;
        p2 = (p2 + 1) % d2;

        let y3 = c3[p3];
        c3[p3] = x + y3 * feedback;
        p3 = (p3 + 1) % d3;

        let y4 = c4[p4];
        c4[p4] = x + y4 * feedback;
        p4 = (p4 + 1) % d4;

        comb_sum[i] = (y1 + y2 + y3 + y4) * 0.25;
    }

    let da1 = ((0.0050 * r).round() as usize).max(1);
    let da2 = ((0.0017 * r).round() as usize).max(1);
    let ap_gain = 0.5f32;

    let mut a1_buf = vec![0.0f32; da1];
    let mut a2_buf = vec![0.0f32; da2];
    let mut pa1 = 0;
    let mut pa2 = 0;

    let mut out = vec![0.0f32; input.len()];
    for (i, &s) in comb_sum.iter().enumerate() {
        let buf_val1 = a1_buf[pa1];
        let y1 = -ap_gain * s + buf_val1;
        a1_buf[pa1] = s + ap_gain * y1;
        pa1 = (pa1 + 1) % da1;

        let buf_val2 = a2_buf[pa2];
        let y2 = -ap_gain * y1 + buf_val2;
        a2_buf[pa2] = y1 + ap_gain * y2;
        pa2 = (pa2 + 1) % da2;

        out[i] = y2;
    }
    out
}

/// Applies a 64-sample anti-pop lookahead cosine ramp down to zero.
pub(super) fn apply_antipop_ramp_down(samples: &mut [f32]) {
    let ramp_len = 64.min(samples.len());
    for k in 0..samples.len() {
        if k < ramp_len {
            let frac = k as f64 / 64.0;
            let ramp = 0.5 * (1.0 + (PI * frac).cos());
            samples[k] *= ramp as f32;
        } else {
            samples[k] = 0.0;
        }
    }
}

/// Soft-knee compressor / limiter insert.
pub(super) fn apply_dsp_compressor(samples: &mut [f32], threshold: f32, ratio: f32) {
    if ratio <= 1.0 || threshold <= 0.0 {
        return;
    }
    for s in samples.iter_mut() {
        let abs_s = s.abs();
        if abs_s > threshold {
            let excess = abs_s - threshold;
            let compressed = threshold + excess / ratio;
            *s = s.signum() * compressed;
        }
    }
}

// ============================================================================
// AssetHost Extension Operations
// ============================================================================

impl AssetHost<'_> {
    /// Categorical operations dispatch for acoustics, discourse, and chronicle extensions.
    pub(crate) fn acoustics_discourse_chronicle_ext_op(
        &mut self,
        id: &str,
        a: &Args,
    ) -> Result<Value, String> {
        match id {
            "hypermedia.acoustics.route_bus_dag"
            | "acoustics.route_bus_dag"
            | "synth.acoustics.route_bus_dag"
            | "route_bus_dag" => self.route_bus_dag(a),

            "hypermedia.acoustics.spatialize_pga_zone"
            | "acoustics.spatialize_pga_zone"
            | "synth.acoustics.spatialize_pga_zone"
            | "spatialize_pga_zone" => self.spatialize_pga_zone(a),

            "hypermedia.discourse.inspect_engine_telemetry"
            | "discourse.inspect_engine_telemetry"
            | "synth.discourse.inspect_engine_telemetry"
            | "inspect_engine_telemetry" => self.inspect_engine_telemetry(a),

            "hypermedia.discourse.verify_fiduciary_rid"
            | "discourse.verify_fiduciary_rid"
            | "synth.discourse.verify_fiduciary_rid"
            | "verify_fiduciary_rid" => self.verify_fiduciary_rid(a),

            "hypermedia.chronicle.capture_deterministic_movie"
            | "chronicle.capture_deterministic_movie"
            | "synth.chronicle.capture_deterministic_movie"
            | "capture_deterministic_movie" => self.capture_deterministic_movie(a),

            "hypermedia.chronicle.sync_spatial_motors"
            | "chronicle.sync_spatial_motors"
            | "synth.chronicle.sync_spatial_motors"
            | "sync_spatial_motors" => self.sync_spatial_motors(a),

            _ => Err(format!(
                "Unknown acoustics/discourse/chronicle extension operation: {id}"
            )),
        }
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

    fn create_test_host<'a>(
        workspace: AssetWorkspace,
        inputs: &'a [Resource],
        cancel: &'a AtomicBool,
        limits: &'a ToolLimits,
    ) -> AssetHost<'a> {
        AssetHost {
            workspace,
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn test_route_bus_dag_3_buses() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        // Synthesize 2 test audio tracks (440 Hz and 880 Hz)
        let rate = 48000u32;
        let frames = 4800; // 0.1s
        let samples1 = qualia_audio::media_store::synth_tone_f32(440.0, rate, frames, 0.25);
        let samples2 = qualia_audio::media_store::synth_tone_f32(880.0, rate, frames, 0.25);

        let h1 = host
            .workspace
            .insert(
                Artifact::Audio {
                    rate,
                    samples: samples1,
                },
                host.limits,
            )
            .unwrap();
        let h2 = host
            .workspace
            .insert(
                Artifact::Audio {
                    rate,
                    samples: samples2,
                },
                host.limits,
            )
            .unwrap();

        let mut b1 = BTreeMap::new();
        b1.insert("name".into(), Value::String("SFX".into()));
        b1.insert("audio".into(), h1);
        b1.insert("send_to".into(), Value::String("Master".into()));
        b1.insert("gain_db".into(), Value::F64(-3.0));
        b1.insert("state".into(), Value::String("playing".into()));
        b1.insert("lowpass_hz".into(), Value::F64(8000.0));
        b1.insert("reverb_wet".into(), Value::F64(0.15));

        let mut b2 = BTreeMap::new();
        b2.insert("name".into(), Value::String("Music".into()));
        b2.insert("audio".into(), h2);
        b2.insert("send_to".into(), Value::String("Master".into()));
        b2.insert("gain_db".into(), Value::F64(-6.0));
        b2.insert("state".into(), Value::String("fade_to_stop".into()));
        b2.insert("lowpass_hz".into(), Value::F64(20000.0));
        b2.insert("reverb_wet".into(), Value::F64(0.0));

        let mut b_master = BTreeMap::new();
        b_master.insert("name".into(), Value::String("Master".into()));
        b_master.insert("send_to".into(), Value::String("".into()));
        b_master.insert("gain_db".into(), Value::F64(0.0));

        let mut args = BTreeMap::new();
        args.insert(
            "buses".into(),
            Value::List(vec![
                Value::Record(b1),
                Value::Record(b2),
                Value::Record(b_master),
            ]),
        );

        let res = host
            .acoustics_discourse_chronicle_ext_op("hypermedia.acoustics.route_bus_dag", &args)
            .expect("route_bus_dag should succeed");

        let artifact = host.workspace.get(&res).expect("Should find mixed audio");
        match artifact {
            Artifact::Audio {
                rate: out_rate,
                samples,
            } => {
                assert_eq!(*out_rate, rate);
                assert_eq!(samples.len(), frames);
                assert!(samples
                    .iter()
                    .all(|s| s.is_finite() && *s >= -1.0 && *s <= 1.0));
            }
            _ => panic!("Expected Audio artifact"),
        }
    }

    #[test]
    fn test_route_bus_dag_cycle_detection() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let rate = 48000u32;
        let samples = qualia_audio::media_store::synth_tone_f32(440.0, rate, 1000, 0.25);
        let h = host
            .workspace
            .insert(Artifact::Audio { rate, samples }, host.limits)
            .unwrap();

        let mut b1 = BTreeMap::new();
        b1.insert("name".into(), Value::String("BusA".into()));
        b1.insert("audio".into(), h.clone());
        b1.insert("send_to".into(), Value::String("BusB".into()));

        let mut b2 = BTreeMap::new();
        b2.insert("name".into(), Value::String("BusB".into()));
        b2.insert("send_to".into(), Value::String("BusA".into()));

        let mut args = BTreeMap::new();
        args.insert(
            "buses".into(),
            Value::List(vec![Value::Record(b1), Value::Record(b2)]),
        );

        let err = host
            .acoustics_discourse_chronicle_ext_op("acoustics.route_bus_dag", &args)
            .unwrap_err();
        assert!(
            err.contains("Cycle detected"),
            "Expected cycle error, got: {err}"
        );
    }

    #[test]
    fn test_spatialize_pga_zone() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let rate = 48000u32;
        let samples = qualia_audio::media_store::synth_tone_f32(440.0, rate, 4800, 0.5);
        let h = host
            .workspace
            .insert(Artifact::Audio { rate, samples }, host.limits)
            .unwrap();

        let mut args = BTreeMap::new();
        args.insert("audio".into(), h);
        args.insert(
            "source_pos".into(),
            Value::List(vec![Value::F64(5.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "listener_pos".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "source_vel".into(),
            Value::List(vec![Value::F64(-15.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert("occlusion".into(), Value::F64(0.2));
        args.insert("reverb_room_size".into(), Value::F64(0.5));

        let res = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.acoustics.spatialize_pga_zone",
                &args,
            )
            .expect("spatialize_pga_zone should succeed");

        let artifact = host
            .workspace
            .get(&res)
            .expect("Should find spatialized audio");
        match artifact {
            Artifact::Audio {
                rate: out_rate,
                samples: out_samples,
            } => {
                assert_eq!(*out_rate, rate);
                assert_eq!(out_samples.len(), 4800);
                assert!(out_samples.iter().all(|s| s.is_finite()));
            }
            _ => panic!("Expected Audio artifact"),
        }
    }

    #[test]
    fn test_inspect_engine_telemetry() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("frame_count".into(), Value::U64(32));
        args.insert("target_fps".into(), Value::U64(60));

        let val = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.discourse.inspect_engine_telemetry",
                &args,
            )
            .expect("inspect_engine_telemetry should succeed");

        let rec = record(&val).unwrap();
        assert_eq!(rec.get("frame_count"), Some(&Value::U64(32)));
        assert_eq!(rec.get("budget_compliant"), Some(&Value::Bool(true)));
        assert!(rec.contains_key("telemetry_json"));
        assert!(rec.contains_key("p99_frame_time_us"));
    }

    #[test]
    fn test_verify_fiduciary_rid() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        // Permit mode
        let mut args_permit = BTreeMap::new();
        args_permit.insert(
            "resource_did".into(),
            Value::String("did:q42:manifold:asset_42".into()),
        );
        args_permit.insert("deontic_mode".into(), Value::String("permit".into()));
        args_permit.insert("sensitivity_class".into(), Value::U64(3));
        args_permit.insert(
            "dependent_rids".into(),
            Value::List(vec![Value::String("did:q42:dep:1".into())]),
        );

        let val_permit = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.discourse.verify_fiduciary_rid",
                &args_permit,
            )
            .unwrap();
        let rec_permit = record(&val_permit).unwrap();
        assert_eq!(rec_permit.get("permitted"), Some(&Value::Bool(true)));
        assert_eq!(
            rec_permit.get("reverse_dependency_count"),
            Some(&Value::U64(1))
        );

        // Forbid mode
        let mut args_forbid = BTreeMap::new();
        args_forbid.insert("deontic_mode".into(), Value::String("forbid".into()));
        let val_forbid = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.discourse.verify_fiduciary_rid",
                &args_forbid,
            )
            .unwrap();
        let rec_forbid = record(&val_forbid).unwrap();
        assert_eq!(rec_forbid.get("permitted"), Some(&Value::Bool(false)));
    }

    #[test]
    fn test_capture_deterministic_movie() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        // 2 frames of 16x16 RGBA
        let img = graphics::Rgba {
            width: 16,
            height: 16,
            pixels: vec![[200, 100, 50, 255]; 256],
        };
        let f1 = host
            .workspace
            .insert(Artifact::Rgba(img.clone()), host.limits)
            .unwrap();
        let f2 = host
            .workspace
            .insert(Artifact::Rgba(img), host.limits)
            .unwrap();

        let mut args = BTreeMap::new();
        args.insert("frames".into(), Value::List(vec![f1, f2]));
        args.insert("fps".into(), Value::U64(24));

        let res = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.chronicle.capture_deterministic_movie",
                &args,
            )
            .expect("capture_deterministic_movie should succeed");

        let resource = host
            .workspace
            .encoded(&res)
            .expect("Should find encoded MP4");
        assert_eq!(resource.kind, "mp4");
        assert!(resource.bytes.len() > 100);
    }

    #[test]
    fn test_sync_spatial_motors() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut m1 = BTreeMap::new();
        m1.insert("entity_id".into(), Value::U64(1));
        m1.insert(
            "position".into(),
            Value::List(vec![
                Value::F64(1.234),
                Value::F64(-2.345),
                Value::F64(3.456),
            ]),
        );
        m1.insert(
            "rotation".into(),
            Value::List(vec![
                Value::F64(0.0),
                Value::F64(0.0),
                Value::F64(0.0),
                Value::F64(1.0),
            ]),
        );
        m1.insert("sequence".into(), Value::U64(100));

        let mut m2 = BTreeMap::new();
        m2.insert("entity_id".into(), Value::U64(2));
        m2.insert(
            "position".into(),
            Value::List(vec![Value::F64(-5.0), Value::F64(10.0), Value::F64(0.5)]),
        );
        m2.insert(
            "rotation".into(),
            Value::List(vec![
                Value::F64(0.5),
                Value::F64(0.5),
                Value::F64(0.5),
                Value::F64(0.5),
            ]),
        );
        m2.insert("sequence".into(), Value::U64(101));

        let mut args = BTreeMap::new();
        args.insert(
            "motors".into(),
            Value::List(vec![Value::Record(m1), Value::Record(m2)]),
        );

        let val = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.chronicle.sync_spatial_motors",
                &args,
            )
            .expect("sync_spatial_motors should succeed");

        let rec = record(&val).unwrap();
        assert_eq!(rec.get("entity_count"), Some(&Value::U64(2)));
        assert_eq!(rec.get("bytes_per_motor"), Some(&Value::U64(16)));
        assert_eq!(rec.get("total_bytes"), Some(&Value::U64(32)));

        if let Some(Value::F64(err)) = rec.get("max_position_error_m") {
            assert!(*err < 0.001, "Position error must be < 1mm, got {err}");
        } else {
            panic!("Expected max_position_error_m float");
        }
    }
}

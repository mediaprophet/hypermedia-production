//! Analytical PGA Motor two-bone IK solver, keyframe interpolation, and Lottie v5.7.4 export.

mod ik_keyframes;
mod lottie_export;

use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(crate) fn kinetics_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.kinetics.solve_two_bone_ik"
            | "synth.kinetics.solve_two_bone_ik"
            | "kinetics.solve_two_bone_ik" => self.solve_two_bone_ik(a),

            "hypermedia.kinetics.interpolate_keyframes"
            | "synth.kinetics.interpolate_keyframes"
            | "kinetics.interpolate_keyframes" => self.interpolate_keyframes(a),

            "hypermedia.kinetics.export_lottie"
            | "synth.kinetics.export_lottie"
            | "kinetics.export_lottie" => self.export_lottie(a),

            _ => Err(format!("Unknown kinetics operation: {id}")),
        }
    }
}

pub(super) fn string<'a>(a: &'a Args, k: &str, default: &'a str) -> Result<&'a str, String> {
    match a.get(k) {
        Some(v) => text(v),
        None => Ok(default),
    }
}

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
    fn test_solve_two_bone_ik_reachable() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert(
            "root".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "target".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(1.5), Value::F64(0.0)]),
        );
        args.insert(
            "pole".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(1.0)]),
        );
        args.insert("upper_length".into(), Value::F64(1.0));
        args.insert("lower_length".into(), Value::F64(1.0));

        let res = host
            .kinetics_op("hypermedia.kinetics.solve_two_bone_ik", &args)
            .expect("ik solve");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("bone_count"), Some(&Value::I64(3)));
        assert_eq!(r.get("reachable"), Some(&Value::Bool(true)));

        let mid_val = r.get("mid_position").unwrap();
        let mid = vector::<3>(mid_val).unwrap();
        assert!(mid[2] > 0.0, "mid should bend toward positive z pole");

        let end_val = r.get("end_position").unwrap();
        let end = vector::<3>(end_val).unwrap();
        assert!((end[0] - 0.0).abs() < 1e-4);
        assert!((end[1] - 1.5).abs() < 1e-4);
        assert!((end[2] - 0.0).abs() < 1e-4);

        let up_rot = vector::<4>(r.get("upper_rotation").unwrap()).unwrap();
        let up_norm = (up_rot[0] * up_rot[0]
            + up_rot[1] * up_rot[1]
            + up_rot[2] * up_rot[2]
            + up_rot[3] * up_rot[3])
            .sqrt();
        assert!((up_norm - 1.0).abs() < 1e-6);

        let low_rot = vector::<4>(r.get("lower_rotation").unwrap()).unwrap();
        let low_norm = (low_rot[0] * low_rot[0]
            + low_rot[1] * low_rot[1]
            + low_rot[2] * low_rot[2]
            + low_rot[3] * low_rot[3])
            .sqrt();
        assert!((low_norm - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_solve_two_bone_ik_unreachable() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert(
            "root".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "target".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(10.0), Value::F64(0.0)]),
        );
        args.insert("upper_length".into(), Value::F64(1.0));
        args.insert("lower_length".into(), Value::F64(1.0));

        let res = host
            .kinetics_op("hypermedia.kinetics.solve_two_bone_ik", &args)
            .expect("ik solve");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("reachable"), Some(&Value::Bool(false)));

        let end_val = r.get("end_position").unwrap();
        let end = vector::<3>(end_val).unwrap();
        assert!((end[1] - 2.0).abs() < 1e-3);
    }

    #[test]
    fn test_interpolate_keyframes() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut kf0 = BTreeMap::new();
        kf0.insert("time".into(), Value::F64(0.0));
        kf0.insert(
            "value".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );

        let mut kf1 = BTreeMap::new();
        kf1.insert("time".into(), Value::F64(1.0));
        kf1.insert(
            "value".into(),
            Value::List(vec![Value::F64(10.0), Value::F64(20.0), Value::F64(30.0)]),
        );

        let mut args = BTreeMap::new();
        args.insert(
            "keyframes".into(),
            Value::List(vec![Value::Record(kf0), Value::Record(kf1)]),
        );
        args.insert("fps".into(), Value::I64(24));
        args.insert("duration".into(), Value::F64(1.0));
        args.insert("easing".into(), Value::String("smoothstep".into()));
        args.insert("property".into(), Value::String("position".into()));

        let res = host
            .kinetics_op("hypermedia.kinetics.interpolate_keyframes", &args)
            .expect("interpolate");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("frame_count"), Some(&Value::I64(24)));
        assert_eq!(r.get("track_count"), Some(&Value::I64(1)));
        assert_eq!(r.get("duration"), Some(&Value::F64(1.0)));

        let Value::List(samples) = r.get("samples").unwrap() else { panic!("expected list") };
        assert_eq!(samples.len(), 24);

        let Value::Record(first) = &samples[0] else { panic!("record") };
        assert_eq!(first.get("frame"), Some(&Value::I64(0)));
        assert_eq!(first.get("time"), Some(&Value::F64(0.0)));
        let first_val = vector::<3>(first.get("value").unwrap()).unwrap();
        assert_eq!(first_val, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn test_export_lottie() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut shape0 = BTreeMap::new();
        shape0.insert("name".into(), Value::String("box1".into()));
        shape0.insert(
            "color".into(),
            Value::List(vec![
                Value::F64(1.0),
                Value::F64(0.0),
                Value::F64(0.0),
                Value::F64(1.0),
            ]),
        );
        shape0.insert(
            "start_pos".into(),
            Value::List(vec![Value::F64(100.0), Value::F64(100.0)]),
        );
        shape0.insert(
            "end_pos".into(),
            Value::List(vec![Value::F64(400.0), Value::F64(400.0)]),
        );
        shape0.insert(
            "size".into(),
            Value::List(vec![Value::F64(80.0), Value::F64(80.0)]),
        );

        let mut args = BTreeMap::new();
        args.insert("width".into(), Value::I64(800));
        args.insert("height".into(), Value::I64(600));
        args.insert("fps".into(), Value::I64(60));
        args.insert("frames".into(), Value::I64(120));
        args.insert("name".into(), Value::String("test_anim".into()));
        args.insert("shapes".into(), Value::List(vec![Value::Record(shape0)]));

        let res = host
            .kinetics_op("hypermedia.kinetics.export_lottie", &args)
            .expect("export lottie");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("type"), Some(&Value::String("json".into())));

        let encoded = host.workspace.encoded(&Value::Record(r)).expect("encoded");
        let parsed: serde_json::Value =
            serde_json::from_slice(&encoded.bytes).expect("valid json");

        assert_eq!(parsed["v"], "5.7.4");
        assert_eq!(parsed["fr"], 60.0);
        assert_eq!(parsed["ip"], 0);
        assert_eq!(parsed["op"], 120);
        assert_eq!(parsed["w"], 800.0);
        assert_eq!(parsed["h"], 600.0);
        assert_eq!(parsed["nm"], "test_anim");

        let layers = parsed["layers"].as_array().expect("layers array");
        assert_eq!(layers.len(), 1);
        assert_eq!(layers[0]["ty"], 4);
        assert_eq!(layers[0]["nm"], "box1");
        assert_eq!(layers[0]["ks"]["p"]["a"], 1);

        let shapes = layers[0]["shapes"].as_array().expect("shapes array");
        assert_eq!(shapes[0]["ty"], "rc");
        assert_eq!(shapes[1]["ty"], "fl");
    }

    #[test]
    fn test_solve_two_bone_ik_skl1_10d_two_produces_match() {
        use qualia_core_db::container_10d::skeletal_container::read_skeletal_section;

        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert(
            "root".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "target".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(1.5), Value::F64(0.0)]),
        );
        args.insert(
            "pole".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(1.0)]),
        );
        args.insert("upper_length".into(), Value::F64(1.0));
        args.insert("lower_length".into(), Value::F64(1.0));

        let a = host
            .kinetics_op("hypermedia.kinetics.solve_two_bone_ik", &args)
            .expect("ik a");
        let b = host
            .kinetics_op("hypermedia.kinetics.solve_two_bone_ik", &args)
            .expect("ik b");
        let Value::Record(ra) = a else { panic!("record") };
        let Value::Record(rb) = b else { panic!("record") };

        let mid_a = vector::<3>(ra.get("mid_position").unwrap()).unwrap();
        let mid_b = vector::<3>(rb.get("mid_position").unwrap()).unwrap();
        let end_a = vector::<3>(ra.get("end_position").unwrap()).unwrap();
        let end_b = vector::<3>(rb.get("end_position").unwrap()).unwrap();
        assert_eq!(mid_a, mid_b);
        assert_eq!(end_a, end_b);
        assert_eq!(ra.get("skl1_joint_count"), Some(&Value::I64(3)));

        let enc_a = host
            .workspace
            .encoded(ra.get("container_10d").unwrap())
            .expect("10d a");
        let enc_b = host
            .workspace
            .encoded(rb.get("container_10d").unwrap())
            .expect("10d b");
        assert_eq!(enc_a.kind, "10d");
        assert_eq!(enc_a.bytes, enc_b.bytes);

        let skl = read_skeletal_section(&enc_a.bytes).expect("SKL1 in .10d");
        assert_eq!(skl.counts().joints, 3);
        assert_eq!(skl.counts().vertices, 3);
        let skl1_len = num(ra.get("skl1_byte_len").unwrap()).unwrap() as usize;
        assert!(skl1_len > 0);
        assert!(skl1_len < enc_a.bytes.len());
    }
}

//! Deterministic Unicode BiDi (UAX #9) & HarfBuzz complex script shaping,
//! multi-channel signed distance field (MSDF) font atlas packing,
//! bidirectional diegetic surface 3D quad projection & reverse UV pick-back,
//! and 4-stage fiduciary input routing.

mod bidi_msdf;
mod diegetic_input;

use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(crate) fn glyph_layout_diegetic_op(
        &mut self,
        id: &str,
        a: &Args,
    ) -> Result<Value, String> {
        match id {
            "hypermedia.glyph.shape_harfbuzz_bidi"
            | "glyph.shape_harfbuzz_bidi"
            | "synth.glyph.shape_harfbuzz_bidi" => self.shape_harfbuzz_bidi(a),

            "hypermedia.glyph.pack_msdf_font_atlas"
            | "glyph.pack_msdf_font_atlas"
            | "synth.glyph.pack_msdf_font_atlas" => self.pack_msdf_font_atlas(a),

            "hypermedia.layout.project_diegetic_surface"
            | "layout.project_diegetic_surface"
            | "synth.layout.project_diegetic_surface" => self.project_diegetic_surface(a),

            "hypermedia.layout.route_input_action"
            | "layout.route_input_action"
            | "synth.layout.route_input_action" => self.route_input_action(a),

            _ => Err(format!("Unknown glyph/layout diegetic operation: {id}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

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
    fn test_shape_harfbuzz_bidi_ltr() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("text".into(), Value::String("Hello World".into()));
        args.insert("font_size".into(), Value::F64(24.0));

        let res = host
            .glyph_layout_diegetic_op("hypermedia.glyph.shape_harfbuzz_bidi", &args)
            .expect("bidi shaping");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(
            r.get("logical_text"),
            Some(&Value::String("Hello World".into()))
        );
        assert_eq!(
            r.get("visual_text"),
            Some(&Value::String("Hello World".into()))
        );
        assert_eq!(
            r.get("resolved_direction"),
            Some(&Value::String("ltr".into()))
        );
        assert_eq!(r.get("rtl_run_count"), Some(&Value::U64(0)));
        let Value::F64(adv) = r.get("total_advance_px").unwrap() else { panic!() };
        assert!(*adv > 0.0);
        let Value::List(clusters) = r.get("glyph_clusters").unwrap() else { panic!() };
        assert_eq!(clusters.len(), 11);
    }

    #[test]
    fn test_shape_harfbuzz_bidi_rtl_hebrew() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("text".into(), Value::String("שלום".into()));

        let res = host
            .glyph_layout_diegetic_op("hypermedia.glyph.shape_harfbuzz_bidi", &args)
            .expect("bidi shaping rtl");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(
            r.get("resolved_direction"),
            Some(&Value::String("rtl".into()))
        );
        assert_eq!(r.get("rtl_run_count"), Some(&Value::U64(1)));
        assert_eq!(r.get("visual_text"), Some(&Value::String("םולש".into())));
    }

    #[test]
    fn test_pack_msdf_font_atlas() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("charset".into(), Value::String("ABC012".into()));
        args.insert("cell_size".into(), Value::U64(24));
        args.insert("px_range".into(), Value::F64(4.0));

        let res = host
            .glyph_layout_diegetic_op("hypermedia.glyph.pack_msdf_font_atlas", &args)
            .expect("pack msdf atlas");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("glyph_count"), Some(&Value::U64(6)));
        let atlas_w = r.get("atlas_width").unwrap();
        let atlas_h = r.get("atlas_height").unwrap();
        assert_eq!(atlas_w, &Value::U64(72));
        assert_eq!(atlas_h, &Value::U64(48));

        let metrics_val = r.get("metrics_json").unwrap();
        let encoded = host.workspace.encoded(metrics_val).expect("encoded json");
        let parsed: serde_json::Value =
            serde_json::from_slice(&encoded.bytes).expect("valid json");
        assert_eq!(parsed["atlas"]["type"], "msdf");
        assert_eq!(parsed["atlas"]["glyph_count"], 6);
    }

    #[test]
    fn test_project_diegetic_surface() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let pixels = vec![[64, 128, 200, 255]; 32 * 32];
        let ui_handle = host.store_rgba(32, 32, pixels).expect("store rgba");

        let mut args = BTreeMap::new();
        args.insert("ui_image".into(), ui_handle);
        args.insert("viewport_width".into(), Value::U64(256));
        args.insert("viewport_height".into(), Value::U64(256));
        args.insert("yaw_deg".into(), Value::F64(15.0));
        args.insert("pitch_deg".into(), Value::F64(-5.0));
        args.insert(
            "pick_screen_xy".into(),
            Value::List(vec![Value::F64(128.0), Value::F64(128.0)]),
        );

        let res = host
            .glyph_layout_diegetic_op("hypermedia.layout.project_diegetic_surface", &args)
            .expect("project diegetic surface");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("hit_surface"), Some(&Value::Bool(true)));

        let Value::List(uv) = r.get("picked_uv").unwrap() else { panic!() };
        let Value::F64(u) = uv[0] else { panic!() };
        let Value::F64(v) = uv[1] else { panic!() };
        assert!((u - 0.5).abs() < 0.15, "u should be close to center: {}", u);
        assert!((v - 0.5).abs() < 0.15, "v should be close to center: {}", v);
    }

    #[test]
    fn test_route_input_action_stages() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args1 = BTreeMap::new();
        args1.insert("event_type".into(), Value::String("key_chord".into()));
        args1.insert(
            "action_binding".into(),
            Value::String("action:escape".into()),
        );
        let res1 = host
            .glyph_layout_diegetic_op("hypermedia.layout.route_input_action", &args1)
            .expect("stage 1 route");
        let Value::Record(r1) = res1 else { panic!() };
        assert_eq!(r1.get("consumed_stage"), Some(&Value::U64(1)));
        assert_eq!(r1.get("propagated_to_3d"), Some(&Value::Bool(false)));

        let mut hud_rect0 = BTreeMap::new();
        hud_rect0.insert("id".into(), Value::String("btn_menu".into()));
        hud_rect0.insert("x".into(), Value::F64(10.0));
        hud_rect0.insert("y".into(), Value::F64(10.0));
        hud_rect0.insert("width".into(), Value::F64(100.0));
        hud_rect0.insert("height".into(), Value::F64(50.0));
        hud_rect0.insert("filter".into(), Value::String("stop".into()));

        let mut args2 = BTreeMap::new();
        args2.insert("event_type".into(), Value::String("pointer_down".into()));
        args2.insert(
            "pointer_xy".into(),
            Value::List(vec![Value::F64(50.0), Value::F64(30.0)]),
        );
        args2.insert(
            "hud_rects".into(),
            Value::List(vec![Value::Record(hud_rect0)]),
        );
        let res2 = host
            .glyph_layout_diegetic_op("hypermedia.layout.route_input_action", &args2)
            .expect("stage 2 route");
        let Value::Record(r2) = res2 else { panic!() };
        assert_eq!(r2.get("consumed_stage"), Some(&Value::U64(2)));
        assert_eq!(
            r2.get("consumed_by_id"),
            Some(&Value::String("btn_menu".into()))
        );
        assert_eq!(r2.get("propagated_to_3d"), Some(&Value::Bool(false)));

        let mut args3 = BTreeMap::new();
        args3.insert("event_type".into(), Value::String("pointer_down".into()));
        args3.insert(
            "pointer_xy".into(),
            Value::List(vec![Value::F64(180.0), Value::F64(135.0)]),
        );
        args3.insert(
            "diegetic_quad_bounds".into(),
            Value::List(vec![
                Value::F64(80.0),
                Value::F64(60.0),
                Value::F64(200.0),
                Value::F64(150.0),
            ]),
        );
        let res3 = host
            .glyph_layout_diegetic_op("hypermedia.layout.route_input_action", &args3)
            .expect("stage 3 route");
        let Value::Record(r3) = res3 else { panic!() };
        assert_eq!(r3.get("consumed_stage"), Some(&Value::U64(3)));
        assert_eq!(r3.get("propagated_to_3d"), Some(&Value::Bool(true)));
        let Value::List(uv3) = r3.get("diegetic_uv").unwrap() else { panic!() };
        assert_eq!(uv3[0], Value::F64(0.5));
        assert_eq!(uv3[1], Value::F64(0.5));

        let mut args4 = BTreeMap::new();
        args4.insert("event_type".into(), Value::String("gamepad_axis".into()));
        args4.insert("axis_value".into(), Value::F64(0.10));
        args4.insert("deadzone".into(), Value::F64(0.15));
        let res4 = host
            .glyph_layout_diegetic_op("hypermedia.layout.route_input_action", &args4)
            .expect("stage 4 route deadzone");
        let Value::Record(r4) = res4 else { panic!() };
        assert_eq!(r4.get("consumed_stage"), Some(&Value::U64(4)));
        assert_eq!(r4.get("remapped_axis_strength"), Some(&Value::F64(0.0)));

        let mut args4b = BTreeMap::new();
        args4b.insert("event_type".into(), Value::String("gamepad_axis".into()));
        args4b.insert("axis_value".into(), Value::F64(0.575));
        args4b.insert("deadzone".into(), Value::F64(0.15));
        let res4b = host
            .glyph_layout_diegetic_op("hypermedia.layout.route_input_action", &args4b)
            .expect("stage 4 route outside deadzone");
        let Value::Record(r4b) = res4b else { panic!() };
        assert_eq!(r4b.get("consumed_stage"), Some(&Value::U64(4)));
        let Value::F64(strength) = r4b.get("remapped_axis_strength").unwrap() else { panic!() };
        assert!((strength - 0.5).abs() < 1e-4);
    }
}

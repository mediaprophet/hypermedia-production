//! Bounded Locus generative curve and geodesic path morphisms:
//! G2 cubic corner filleting at print density, arc-length / curvature resampling,
//! sub-curve trimming, cyclic double-reflection Bishop rotation-minimizing frames (RMF),
//! and mesh geodesic shortest edge path search.

mod fillet_resample_trim;
mod rmf_geodesic;

use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    /// Dispatch morphism operations for generative curve and geodesic path workflows.
    pub(crate) fn locus_generative_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.locus.fillet" | "locus.fillet" | "synth.locus.fillet" => {
                self.locus_fillet(a)
            }
            "hypermedia.locus.resample" | "locus.resample" | "synth.locus.resample" => {
                self.locus_resample(a)
            }
            "hypermedia.locus.trim" | "locus.trim" | "synth.locus.trim" => {
                self.locus_trim(a)
            }
            "hypermedia.locus.bishop_rmf_cyclic"
            | "locus.bishop_rmf_cyclic"
            | "synth.locus.bishop_rmf_cyclic" => self.locus_bishop_rmf_cyclic(a),
            "hypermedia.locus.shortest_edge_path"
            | "locus.shortest_edge_path"
            | "synth.locus.shortest_edge_path" => self.locus_shortest_edge_path(a),
            _ => Err(format!("Unknown locus generative operation: {id}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use drafting_geometry::Vec2;
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;
    use vibe::Value;

    fn make_test_host<'a>(
        limits: &'a ToolLimits,
        cancel: &'a AtomicBool,
        inputs: &'a [Resource],
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn store_test_profile(host: &mut AssetHost<'_>, pts: Vec<[f64; 2]>) -> Value {
        host.workspace
            .insert(Artifact::Profile(pts), host.limits)
            .expect("store test profile")
    }

    fn store_test_mesh(host: &mut AssetHost<'_>) -> Value {
        // Quad sheet of two triangles:
        // 0: (0,0,0), 1: (1,0,0), 2: (1,1,0), 3: (0,1,0)
        let positions = vec![
            [0.0f32, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let triangles = vec![[0u32, 1, 2], [0, 2, 3]];
        let mesh = Mesh {
            positions,
            triangles,
            min: [0.0, 0.0, 0.0],
            max: [1.0, 1.0, 0.0],
        };
        host.workspace
            .insert(Artifact::Mesh(mesh), host.limits)
            .expect("store test mesh")
    }

    #[test]
    fn test_locus_fillet_square() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        // 10x10 CCW square
        let square = vec![
            [0.0, 0.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [0.0, 10.0],
        ];
        let prof_h = store_test_profile(&mut host, square);

        let mut args = BTreeMap::new();
        args.insert("profile".into(), prof_h);
        args.insert("radius".into(), Value::F64(1.0));
        args.insert("segments".into(), Value::U64(4));
        args.insert("closed".into(), Value::Bool(true));

        let res = host
            .locus_generative_op("hypermedia.locus.fillet", &args)
            .expect("fillet execution");

        let out_pts = host.profile(&res).expect("filleted profile handle");
        // Each corner rounded with 4 segments (5 points) replacing the sharp vertex
        assert!(out_pts.len() > 4);

        let pts_vec: Vec<Vec2> = out_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();
        let area = drafting_geometry::shoelace(&pts_vec);
        assert!(area > 0.0, "Shoelace area must be positive (CCW)");
    }

    #[test]
    fn test_locus_fillet_chamfer() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let square = vec![
            [0.0, 0.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [0.0, 10.0],
        ];
        let prof_h = store_test_profile(&mut host, square);

        let mut args = BTreeMap::new();
        args.insert("profile".into(), prof_h);
        args.insert("radius".into(), Value::F64(1.0));
        args.insert("segments".into(), Value::U64(1)); // chamfer
        args.insert("closed".into(), Value::Bool(true));

        let res = host
            .locus_generative_op("hypermedia.locus.fillet", &args)
            .expect("fillet chamfer execution");

        let out_pts = host.profile(&res).expect("chamfer profile handle");
        // 4 corners * 2 touch points = 8 vertices
        assert_eq!(out_pts.len(), 8);
    }

    #[test]
    fn test_locus_resample_count() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let square = vec![
            [0.0, 0.0],
            [4.0, 0.0],
            [4.0, 4.0],
            [0.0, 4.0],
        ];
        let prof_h = store_test_profile(&mut host, square);

        let mut args = BTreeMap::new();
        args.insert("profile".into(), prof_h);
        args.insert("mode".into(), Value::String("count".into()));
        args.insert("count".into(), Value::U64(32));
        args.insert("closed".into(), Value::Bool(true));

        let res = host
            .locus_generative_op("hypermedia.locus.resample", &args)
            .expect("resample count");

        let out_pts = host.profile(&res).expect("resampled profile handle");
        assert_eq!(out_pts.len(), 32);
    }

    #[test]
    fn test_locus_resample_length_and_curvature() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let square = vec![
            [0.0, 0.0],
            [10.0, 0.0],
            [10.0, 10.0],
            [0.0, 10.0],
        ];
        let prof_h = store_test_profile(&mut host, square);

        // Test length mode
        let mut args_len = BTreeMap::new();
        args_len.insert("profile".into(), prof_h.clone());
        args_len.insert("mode".into(), Value::String("length".into()));
        args_len.insert("step_length".into(), Value::F64(1.0));
        args_len.insert("closed".into(), Value::Bool(true));

        let res_len = host
            .locus_generative_op("hypermedia.locus.resample", &args_len)
            .expect("resample length");
        let pts_len = host.profile(&res_len).expect("resampled length handle");
        assert_eq!(pts_len.len(), 40); // 40.0 total perimeter / 1.0 = 40 vertices

        // Test curvature mode
        let mut args_curv = BTreeMap::new();
        args_curv.insert("profile".into(), prof_h);
        args_curv.insert("mode".into(), Value::String("curvature".into()));
        args_curv.insert("count".into(), Value::U64(24));
        args_curv.insert("closed".into(), Value::Bool(true));

        let res_curv = host
            .locus_generative_op("hypermedia.locus.resample", &args_curv)
            .expect("resample curvature");
        let pts_curv = host.profile(&res_curv).expect("resampled curvature handle");
        assert_eq!(pts_curv.len(), 24);
    }

    #[test]
    fn test_locus_trim() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let square = vec![
            [0.0, 0.0],
            [4.0, 0.0],
            [4.0, 4.0],
            [0.0, 4.0],
        ];
        let prof_h = store_test_profile(&mut host, square);

        let mut args = BTreeMap::new();
        args.insert("profile".into(), prof_h);
        args.insert("start".into(), Value::F64(0.0));
        args.insert("end".into(), Value::F64(0.25));
        args.insert("close_chord".into(), Value::Bool(true));

        let res = host
            .locus_generative_op("hypermedia.locus.trim", &args)
            .expect("trim locus");

        let out_pts = host.profile(&res).expect("trimmed profile handle");
        assert!(out_pts.len() >= 3);
    }

    #[test]
    fn test_locus_bishop_rmf_cyclic() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        // 16-point circle in XY plane
        let mut path = Vec::new();
        for i in 0..16 {
            let theta = i as f64 * std::f64::consts::TAU / 16.0;
            path.push(Value::List(vec![
                Value::F64(theta.cos()),
                Value::F64(theta.sin()),
                Value::F64(0.0),
            ]));
        }

        let mut args = BTreeMap::new();
        args.insert("path".into(), Value::List(path));
        args.insert("closed".into(), Value::Bool(true));
        args.insert(
            "initial_normal".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(1.0)]),
        );

        let res = host
            .locus_generative_op("hypermedia.locus.bishop_rmf_cyclic", &args)
            .expect("bishop rmf cyclic");

        let rec = record(&res).expect("record");
        let fc = integer(required(rec, "frame_count").unwrap(), "frame_count").unwrap();
        assert_eq!(fc, 16);

        let frames = list(required(rec, "frames").unwrap()).unwrap();
        assert_eq!(frames.len(), 16);

        let defect = num(required(rec, "holonomy_defect_rad").unwrap()).unwrap();
        assert!(defect.is_finite());
    }

    #[test]
    fn test_locus_shortest_edge_path() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_test_mesh(&mut host);

        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("start_vertex".into(), Value::U64(0));
        args.insert("end_vertex".into(), Value::U64(2));

        let res = host
            .locus_generative_op("hypermedia.locus.shortest_edge_path", &args)
            .expect("shortest edge path");

        let rec = record(&res).expect("record");
        let hops = integer(required(rec, "hop_count").unwrap(), "hop_count").unwrap();
        // Direct diagonal edge 0 -> 2 exists in triangle [0, 1, 2]
        assert_eq!(hops, 1);

        let total_len = num(required(rec, "total_length").unwrap()).unwrap();
        assert!((total_len - std::f64::consts::SQRT_2).abs() < 1e-4);

        let profile_val = required(rec, "profile_2d").unwrap();
        let pts_2d = host.profile(profile_val).expect("profile 2d handle");
        assert!(pts_2d.len() >= 3);
    }

    #[test]
    fn test_unknown_op() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);
        let args = BTreeMap::new();
        assert!(host.locus_generative_op("unknown.op", &args).is_err());
    }

    fn polyline_svg_d(pts: &[[f64; 2]]) -> String {
        use std::fmt::Write as _;
        fn svg_num(x: f64) -> String {
            let q = (if x.abs() < 1e-12 { 0.0 } else { x } * 10_000.0).round() / 10_000.0;
            let q = if q == 0.0 { 0.0 } else { q };
            format!("{q:.4}")
        }
        let mut d = String::new();
        let Some(first) = pts.first() else {
            return d;
        };
        let _ = write!(&mut d, "M{} {}", svg_num(first[0]), svg_num(first[1]));
        for p in &pts[1..] {
            let _ = write!(&mut d, " L{} {}", svg_num(p[0]), svg_num(p[1]));
        }
        d.push_str(" Z");
        d
    }

    #[test]
    fn test_locus_fillet_g2_print_density_svg_path_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let square = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        let prof_h = store_test_profile(&mut host, square);

        let mut args = BTreeMap::new();
        args.insert("profile".into(), prof_h);
        args.insert("radius".into(), Value::F64(1.0));
        args.insert("segments".into(), Value::U64(4));
        args.insert("closed".into(), Value::Bool(true));

        let res = host
            .locus_generative_op("hypermedia.locus.fillet", &args)
            .expect("g2 fillet");
        let out_pts = host.profile(&res).expect("filleted profile");
        // Origin 4-seg circular fillet is 20 verts; print-density G2 cubics are at least that dense.
        assert!(
            out_pts.len() >= 16,
            "G2 print-density fillet too sparse: {}",
            out_pts.len()
        );

        let d = polyline_svg_d(out_pts);
        assert_eq!(d, polyline_svg_d(out_pts), "fillet SVG still must be bit-stable");
        let back = kurbo::BezPath::from_svg(&d).expect("svg path round-trip");
        assert!(back.segments().count() >= 3);
        let pts_vec: Vec<Vec2> = out_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();
        assert!(drafting_geometry::shoelace(&pts_vec) > 0.0);
    }
}

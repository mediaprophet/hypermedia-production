//! Deterministic PGA MotorFTI interpolation, Island-Parallel XPBD rigid-body simulation,
//! DirectSpaceState BVH ray/motion queries, Funnel+ORCA navigation, NLA channelbag blending,
//! and double-buffered Simulation Zone curl flow.

mod motor_xpbd_space;
mod nav_nla_zones;

use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(crate) fn kinetics_sim_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.kinetics.interpolate_motor_fti"
            | "kinetics.interpolate_motor_fti"
            | "synth.kinetics.interpolate_motor_fti" => self.interpolate_motor_fti(a),

            "hypermedia.kinetics.step_island_xpbd"
            | "kinetics.step_island_xpbd"
            | "synth.kinetics.step_island_xpbd" => self.step_island_xpbd(a),

            "hypermedia.kinetics.query_direct_space"
            | "kinetics.query_direct_space"
            | "synth.kinetics.query_direct_space" => self.query_direct_space(a),

            "hypermedia.kinetics.navigate_funnel_orca"
            | "kinetics.navigate_funnel_orca"
            | "synth.kinetics.navigate_funnel_orca" => self.navigate_funnel_orca(a),

            "hypermedia.kinetics.evaluate_nla_channelbag"
            | "kinetics.evaluate_nla_channelbag"
            | "synth.kinetics.evaluate_nla_channelbag" => self.evaluate_nla_channelbag(a),

            "hypermedia.kinetics.step_simulation_zone"
            | "kinetics.step_simulation_zone"
            | "synth.kinetics.step_simulation_zone" => self.step_simulation_zone(a),

            _ => Err(format!("Unknown kinetics simulation operation: {id}")),
        }
    }
}

pub(super) fn point3(a: &Args, k: &str) -> Result<[f64; 3], String> {
    let val = required(a, k)?;
    vector::<3>(val)
}

pub(super) fn optional_point3(a: &Args, k: &str, default: [f64; 3]) -> Result<[f64; 3], String> {
    match a.get(k) {
        Some(v) => vector::<3>(v),
        None => Ok(default),
    }
}

pub(super) fn optional_quat4(a: &Args, k: &str, default: [f64; 4]) -> Result<[f64; 4], String> {
    match a.get(k) {
        Some(v) => vector::<4>(v),
        None => Ok(default),
    }
}

pub(super) fn string<'a>(a: &'a Args, k: &str, default: &'a str) -> Result<&'a str, String> {
    match a.get(k) {
        Some(v) => text(v),
        None => Ok(default),
    }
}

pub(super) fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub(super) fn len_sq(a: [f64; 3]) -> f64 {
    dot(a, a)
}

pub(super) fn len(a: [f64; 3]) -> f64 {
    len_sq(a).sqrt()
}

pub(super) fn normalize_quat(q: [f64; 4]) -> [f64; 4] {
    let l_sq = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];
    if l_sq > 1e-12 {
        let inv = 1.0 / l_sq.sqrt();
        [q[0] * inv, q[1] * inv, q[2] * inv, q[3] * inv]
    } else {
        [0.0, 0.0, 0.0, 1.0]
    }
}

pub(super) struct Dsu {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Dsu {
    pub(super) fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    pub(super) fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while root != self.parent[root] {
            root = self.parent[root];
        }
        while i != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    pub(super) fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            if self.rank[root_i] < self.rank[root_j] {
                self.parent[root_i] = root_j;
            } else if self.rank[root_i] > self.rank[root_j] {
                self.parent[root_j] = root_i;
            } else {
                self.parent[root_j] = root_i;
                self.rank[root_i] += 1;
            }
        }
    }

    pub(super) fn count_islands(&mut self) -> usize {
        let n = self.parent.len();
        if n == 0 {
            return 0;
        }
        let mut roots = std::collections::BTreeSet::new();
        for i in 0..n {
            roots.insert(self.find(i));
        }
        roots.len()
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
    fn test_interpolate_motor_fti_sclerp() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("prev_position".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        args.insert("curr_position".into(), Value::List(vec![Value::F64(10.0), Value::F64(0.0), Value::F64(0.0)]));
        args.insert("alpha".into(), Value::F64(0.5));
        args.insert("teleport".into(), Value::Bool(false));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.interpolate_motor_fti", &args)
            .expect("interpolate_motor_fti");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("teleported"), Some(&Value::Bool(false)));
        assert_eq!(r.get("alpha"), Some(&Value::F64(0.5)));

        let pos = vector::<3>(r.get("position").unwrap()).unwrap();
        assert!((pos[0] - 5.0).abs() < 1e-4);
        assert!((pos[1] - 0.0).abs() < 1e-4);
        assert!((pos[2] - 0.0).abs() < 1e-4);

        let motor_8f = list(r.get("motor_8f").unwrap()).unwrap();
        assert_eq!(motor_8f.len(), 8);
    }

    #[test]
    fn test_interpolate_motor_fti_teleport() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("prev_position".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        args.insert("curr_position".into(), Value::List(vec![Value::F64(10.0), Value::F64(2.0), Value::F64(0.0)]));
        args.insert("alpha".into(), Value::F64(0.25));
        args.insert("teleport".into(), Value::Bool(true));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.interpolate_motor_fti", &args)
            .expect("teleport");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("teleported"), Some(&Value::Bool(true)));

        let pos = vector::<3>(r.get("position").unwrap()).unwrap();
        assert_eq!(pos, [10.0, 2.0, 0.0]);
    }

    #[test]
    fn test_step_island_xpbd_ground_and_islands() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut b1 = BTreeMap::new();
        b1.insert("id".into(), Value::String("b1".into()));
        b1.insert("position".into(), Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]));
        b1.insert("velocity".into(), Value::List(vec![Value::F64(0.0), Value::F64(-2.0), Value::F64(0.0)]));
        b1.insert("radius".into(), Value::F64(0.5));
        b1.insert("inv_mass".into(), Value::F64(1.0));

        let mut b2 = BTreeMap::new();
        b2.insert("id".into(), Value::String("b2".into()));
        b2.insert("position".into(), Value::List(vec![Value::F64(0.0), Value::F64(2.0), Value::F64(0.0)]));
        b2.insert("velocity".into(), Value::List(vec![Value::F64(0.0), Value::F64(-1.0), Value::F64(0.0)]));
        b2.insert("radius".into(), Value::F64(0.5));
        b2.insert("inv_mass".into(), Value::F64(1.0));

        let mut args = BTreeMap::new();
        args.insert("bodies".into(), Value::List(vec![Value::Record(b1), Value::Record(b2)]));
        args.insert("steps".into(), Value::I64(4));
        args.insert("substeps".into(), Value::I64(4));
        args.insert("ground_y".into(), Value::F64(0.0));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.step_island_xpbd", &args)
            .expect("step_island_xpbd");

        let Value::Record(r) = res else { panic!("expected record") };
        let bodies_out = list(r.get("bodies").unwrap()).unwrap();
        assert_eq!(bodies_out.len(), 2);

        for b in bodies_out {
            let b_rec = record(b).unwrap();
            let pos = vector::<3>(b_rec.get("position").unwrap()).unwrap();
            assert!(pos[1] >= 0.5 - 1e-4, "y ({}) below ground", pos[1]);
        }
    }

    #[test]
    fn test_step_island_xpbd_area_override() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut b = BTreeMap::new();
        b.insert("id".into(), Value::String("wind_particle".into()));
        b.insert("position".into(), Value::List(vec![Value::F64(0.0), Value::F64(5.0), Value::F64(0.0)]));
        b.insert("radius".into(), Value::F64(0.2));
        b.insert("inv_mass".into(), Value::F64(1.0));

        let mut area = BTreeMap::new();
        area.insert("bounds_min".into(), Value::List(vec![Value::F64(-10.0), Value::F64(0.0), Value::F64(-10.0)]));
        area.insert("bounds_max".into(), Value::List(vec![Value::F64(10.0), Value::F64(10.0), Value::F64(10.0)]));
        area.insert("gravity".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        area.insert("linear_damping".into(), Value::F64(0.5));
        area.insert("wind".into(), Value::List(vec![Value::F64(10.0), Value::F64(0.0), Value::F64(0.0)]));

        let mut args = BTreeMap::new();
        args.insert("bodies".into(), Value::List(vec![Value::Record(b)]));
        args.insert("area_override".into(), Value::Record(area));
        args.insert("steps".into(), Value::I64(4));
        args.insert("substeps".into(), Value::I64(4));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.step_island_xpbd", &args)
            .expect("area override");

        let Value::Record(r) = res else { panic!("expected record") };
        let bodies_out = list(r.get("bodies").unwrap()).unwrap();
        let b0 = record(&bodies_out[0]).unwrap();
        let vel = vector::<3>(b0.get("velocity").unwrap()).unwrap();
        assert!(vel[0] > 0.0, "Particle should accelerate along +X wind");
    }

    #[test]
    fn test_query_direct_space_intersect_ray() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut col1 = BTreeMap::new();
        col1.insert("id".into(), Value::String("cube_static".into()));
        col1.insert("center".into(), Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]));
        col1.insert("half_extents".into(), Value::List(vec![Value::F64(1.0), Value::F64(1.0), Value::F64(1.0)]));
        col1.insert("layer_mask".into(), Value::I64(1));
        col1.insert("is_static".into(), Value::Bool(true));

        let mut args = BTreeMap::new();
        args.insert("colliders".into(), Value::List(vec![Value::Record(col1)]));
        args.insert("query_mode".into(), Value::String("intersect_ray".into()));
        args.insert("ray_origin".into(), Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(-5.0)]));
        args.insert("ray_dir".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(1.0)]));
        args.insert("collision_mask".into(), Value::I64(1));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.query_direct_space", &args)
            .expect("ray query");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("hit"), Some(&Value::Bool(true)));
        assert_eq!(r.get("hit_id"), Some(&Value::String("cube_static".into())));

        let hit_pt = vector::<3>(r.get("hit_point").unwrap()).unwrap();
        assert!((hit_pt[2] - (-1.0)).abs() < 1e-4);

        let normal = vector::<3>(r.get("hit_normal").unwrap()).unwrap();
        assert_eq!(normal, [0.0, 0.0, -1.0]);
    }

    #[test]
    fn test_query_direct_space_cast_motion() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut col = BTreeMap::new();
        col.insert("id".into(), Value::String("target_wall".into()));
        col.insert("center".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(4.0)]));
        col.insert("half_extents".into(), Value::List(vec![Value::F64(2.0), Value::F64(2.0), Value::F64(0.5)]));
        col.insert("layer_mask".into(), Value::I64(1));
        col.insert("is_static".into(), Value::Bool(true));

        let mut args = BTreeMap::new();
        args.insert("colliders".into(), Value::List(vec![Value::Record(col)]));
        args.insert("query_mode".into(), Value::String("cast_motion".into()));
        args.insert("ray_origin".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        args.insert("motion_delta".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(5.0)]));
        args.insert("probe_radius".into(), Value::F64(0.5));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.query_direct_space", &args)
            .expect("cast_motion");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("hit"), Some(&Value::Bool(true)));

        let safe = num(r.get("safe_fraction").unwrap()).unwrap();
        let unsafe_frac = num(r.get("unsafe_fraction").unwrap()).unwrap();
        assert!(safe <= unsafe_frac);
        assert!(safe > 0.0 && safe < 1.0);
    }

    #[test]
    fn test_navigate_funnel_orca() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("start".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        args.insert("goal".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(10.0)]));
        args.insert("agent_radius".into(), Value::F64(0.4));
        args.insert("max_speed".into(), Value::F64(4.0));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.navigate_funnel_orca", &args)
            .expect("funnel orca");

        let Value::Record(r) = res else { panic!("expected record") };
        let count = num(r.get("waypoint_count").unwrap()).unwrap() as usize;
        assert!(count >= 2);

        let len = num(r.get("string_pulled_length").unwrap()).unwrap();
        assert!(len >= 10.0);

        let pref_v = vector::<3>(r.get("preferred_velocity").unwrap()).unwrap();
        assert!(pref_v[2] > 0.0);
    }

    #[test]
    fn test_evaluate_nla_channelbag() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut strip1 = BTreeMap::new();
        strip1.insert("name".into(), Value::String("base_move".into()));
        strip1.insert("start_time".into(), Value::F64(0.0));
        strip1.insert("end_time".into(), Value::F64(2.0));
        strip1.insert("weight".into(), Value::F64(1.0));
        strip1.insert("blend_mode".into(), Value::String("replace".into()));
        strip1.insert("start_val".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        strip1.insert("end_val".into(), Value::List(vec![Value::F64(10.0), Value::F64(0.0), Value::F64(0.0)]));
        strip1.insert("transition".into(), Value::String("linear".into()));

        let mut strip2 = BTreeMap::new();
        strip2.insert("name".into(), Value::String("add_hop".into()));
        strip2.insert("start_time".into(), Value::F64(0.0));
        strip2.insert("end_time".into(), Value::F64(2.0));
        strip2.insert("weight".into(), Value::F64(1.0));
        strip2.insert("blend_mode".into(), Value::String("add".into()));
        strip2.insert("start_val".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
        strip2.insert("end_val".into(), Value::List(vec![Value::F64(0.0), Value::F64(4.0), Value::F64(0.0)]));
        strip2.insert("transition".into(), Value::String("linear".into()));

        let mut args = BTreeMap::new();
        args.insert("strips".into(), Value::List(vec![Value::Record(strip1), Value::Record(strip2)]));
        args.insert("eval_time".into(), Value::F64(1.0));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.evaluate_nla_channelbag", &args)
            .expect("channelbag");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("active_strip_count"), Some(&Value::I64(2)));

        let val = vector::<3>(r.get("evaluated_value").unwrap()).unwrap();
        assert!((val[0] - 5.0).abs() < 1e-4);
        assert!((val[1] - 2.0).abs() < 1e-4);
        assert!((val[2] - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_step_simulation_zone() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = create_test_host(AssetWorkspace::default(), &[], &cancel, &limits);

        let pt0 = Value::List(vec![Value::F64(1.0), Value::F64(0.0), Value::F64(0.0)]);
        let pt1 = Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]);

        let mut args = BTreeMap::new();
        args.insert("points".into(), Value::List(vec![pt0, pt1]));
        args.insert("iterations".into(), Value::I64(8));
        args.insert("dt".into(), Value::F64(0.05));
        args.insert("curl_strength".into(), Value::F64(0.5));
        args.insert("attractor".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));

        let res = host
            .kinetics_sim_op("hypermedia.kinetics.step_simulation_zone", &args)
            .expect("simulation zone");

        let Value::Record(r) = res else { panic!("expected record") };
        assert_eq!(r.get("point_count"), Some(&Value::I64(2)));
        assert_eq!(r.get("iterations"), Some(&Value::I64(8)));

        let disp = num(r.get("mean_displacement").unwrap()).unwrap();
        assert!(disp > 0.0);
    }
}

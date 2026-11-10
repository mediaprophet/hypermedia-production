//! Bounded manifold topology operations: Radial-Edge B-Rep with Euler operators,
//! chamfer/bevel, face inset with depth extrusion, two-sided shell solidification,
//! planar bisection with cap triangulation, and Catmull-Clark / Loop subdivision.

mod radial_bevel_inset;
mod solidify_bisect_subdivide;

use crate::*;
use qualia_core_db::q42::m64_matter::{M64ShellRecord, M64_MODE_SHELL};
use qualia_core_db::render::assets::Mesh;
use vibe::Value;

pub use qualia_core_db::q42::m64_matter::{M64ShellRecord as _, M64_MODE_SHELL as _};

// ── 3D Vector Math Helpers ───────────────────────────────────────────────────

#[inline]
pub(super) fn v_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
pub(super) fn v_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
pub(super) fn v_scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
pub(super) fn v_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
pub(super) fn v_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
pub(super) fn v_len_sq(a: [f32; 3]) -> f32 {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

#[inline]
pub(super) fn v_normalize(a: [f32; 3]) -> [f32; 3] {
    let l_sq = v_len_sq(a);
    if l_sq > 1e-12 {
        let inv = 1.0 / l_sq.sqrt();
        [a[0] * inv, a[1] * inv, a[2] * inv]
    } else {
        [0.0, 1.0, 0.0]
    }
}

/// Compute area-weighted smooth unit vertex normals for a triangle mesh.
pub(crate) fn compute_smooth_normals(
    positions: &[[f32; 3]],
    triangles: &[[u32; 3]],
) -> Vec<[f32; 3]> {
    let mut normals = vec![[0.0f32; 3]; positions.len()];
    for tri in triangles {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= positions.len() || i1 >= positions.len() || i2 >= positions.len() {
            continue;
        }
        let p0 = positions[i0];
        let p1 = positions[i1];
        let p2 = positions[i2];
        let cross = v_cross(v_sub(p1, p0), v_sub(p2, p0));
        for &idx in &[i0, i1, i2] {
            normals[idx][0] += cross[0];
            normals[idx][1] += cross[1];
            normals[idx][2] += cross[2];
        }
    }
    for n in &mut normals {
        *n = v_normalize(*n);
    }
    normals
}

// ── AssetHost Extension Methods ──────────────────────────────────────────────

impl AssetHost<'_> {
    /// Retrieve a reference to a `Mesh` artifact from a production handle.
    pub(crate) fn mesh<'a>(&'a self, val: &Value) -> Result<&'a Mesh, String> {
        match self.workspace.get(val)? {
            Artifact::Mesh(m) => Ok(m),
            _ => Err("Expected mesh handle".into()),
        }
    }

    /// Store a new `Mesh` artifact after bounding coordinates and computing bounding box.
    pub(crate) fn store_mesh_artifact(
        &mut self,
        positions: Vec<[f32; 3]>,
        triangles: Vec<[u32; 3]>,
    ) -> Result<Value, String> {
        self.mesh_budget(positions.len(), triangles.len())?;
        if positions
            .iter()
            .flatten()
            .any(|v| !v.is_finite() || v.abs() > 1e6)
        {
            return Err("Mesh coordinates outside finite +/-1000000 range".into());
        }
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &positions {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        if min[0].is_infinite() {
            min = [0.0; 3];
            max = [0.0; 3];
        }
        self.workspace.insert(
            Artifact::Mesh(Mesh {
                positions,
                triangles,
                min,
                max,
            }),
            self.limits,
        )
    }

    /// Dispatch manifold topology morphisms.
    pub(crate) fn manifold_topology_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.manifold.radial_euler"
            | "manifold.radial_euler"
            | "synth.manifold.radial_euler" => self.manifold_radial_euler(a),

            "hypermedia.manifold.bevel" | "manifold.bevel" | "synth.manifold.bevel" => {
                self.manifold_bevel(a)
            }

            "hypermedia.manifold.inset" | "manifold.inset" | "synth.manifold.inset" => {
                self.manifold_inset(a)
            }

            "hypermedia.manifold.solidify" | "manifold.solidify" | "synth.manifold.solidify" => {
                self.manifold_solidify(a)
            }

            "hypermedia.manifold.bisect_plane"
            | "manifold.bisect_plane"
            | "synth.manifold.bisect_plane" => self.manifold_bisect_plane(a),

            "hypermedia.manifold.subdivide_catmull_clark"
            | "manifold.subdivide_catmull_clark"
            | "synth.manifold.subdivide_catmull_clark" => self.manifold_subdivide_catmull_clark(a),

            _ => Err(format!("Unknown manifold topology operation: {id}")),
        }
    }
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use crate::*;
    use qualia_core_db::q42::m64_matter::M64Header;
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

    fn store_tetrahedron(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [0.0, 0.0, 1.0],
            [1.0, 0.0, -0.5],
            [-0.5, 0.866, -0.5],
            [-0.5, -0.866, -0.5],
        ];
        let triangles = vec![[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]];
        host.store_mesh_artifact(positions, triangles)
            .expect("store tetrahedron")
    }

    fn store_quad_sheet(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let triangles = vec![[0, 1, 2], [0, 2, 3]];
        host.store_mesh_artifact(positions, triangles)
            .expect("store quad sheet")
    }

    #[test]
    fn test_radial_euler_default_refinement_and_m64_serialization() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("crease_sharpness".into(), Value::F64(0.5));

        let res = host
            .manifold_topology_op("hypermedia.manifold.radial_euler", &args)
            .expect("radial_euler default");

        let rec = record(&res).expect("record");
        let v_count = integer(required(rec, "vertex_count").unwrap(), "vertex_count").unwrap();
        let e_count = integer(required(rec, "edge_count").unwrap(), "edge_count").unwrap();
        let f_count = integer(required(rec, "face_count").unwrap(), "face_count").unwrap();
        assert_eq!(v_count, 5); // 4 + 1 new vertex from SEMV
        assert_eq!(f_count, 5); // 4 original - 1 split + 2 new = 5 faces
        assert!(e_count >= 6);

        // Verify .m64 binary container
        let shell_val = required(rec, "shell_m64").unwrap();
        let res_obj = host.workspace.get(shell_val).expect("shell m64 resource");
        let Artifact::Encoded(resource) = res_obj else {
            panic!("Expected encoded resource");
        };
        assert_eq!(resource.kind, "m64");
        assert!(resource.bytes.len() >= 128 + 5 * 64);
        assert_eq!(&resource.bytes[..4], b"m64\0");

        // Verify M64Header CRC
        let header: M64Header = bytemuck::pod_read_unaligned(&resource.bytes[..64]);
        assert_eq!(header.header_crc32c, header.compute_crc32c());
    }

    #[test]
    fn test_radial_euler_explicit_ops() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron(&mut host);
        let mut op1 = BTreeMap::new();
        op1.insert("op".into(), Value::String("SEMV".into()));
        op1.insert("edge_index".into(), Value::U64(1));
        op1.insert("factor".into(), Value::F64(0.4));

        let mut op2 = BTreeMap::new();
        op2.insert("op".into(), Value::String("SFME".into()));
        op2.insert("face_index".into(), Value::U64(0));

        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert(
            "ops".into(),
            Value::List(vec![Value::Record(op1), Value::Record(op2)]),
        );

        let res = host
            .manifold_topology_op("hypermedia.manifold.radial_euler", &args)
            .expect("radial_euler with explicit ops");
        let rec = record(&res).expect("record");
        let v_count = integer(required(rec, "vertex_count").unwrap(), "vertex_count").unwrap();
        assert_eq!(v_count, 5);
    }

    #[test]
    fn test_manifold_bevel() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("width".into(), Value::F64(0.05));
        args.insert("segments".into(), Value::U64(1));

        let res = host
            .manifold_topology_op("hypermedia.manifold.bevel", &args)
            .expect("bevel segments 1");
        let out_mesh = host.mesh(&res).expect("mesh handle");
        assert!(out_mesh.triangles.len() >= 4);
        for p in &out_mesh.positions {
            assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
        }
    }

    #[test]
    fn test_manifold_inset() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_quad_sheet(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("thickness".into(), Value::F64(0.2));
        args.insert("depth".into(), Value::F64(0.05));

        let res = host
            .manifold_topology_op("hypermedia.manifold.inset", &args)
            .expect("inset");
        let out_mesh = host.mesh(&res).expect("mesh handle");
        // 2 input triangles * 7 = 14 output triangles
        assert_eq!(out_mesh.triangles.len(), 14);
        // 4 input vertices + 2 * 3 new inset vertices = 10 positions
        assert_eq!(out_mesh.positions.len(), 10);
    }

    #[test]
    fn test_manifold_solidify() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_quad_sheet(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("thickness".into(), Value::F64(0.1));
        args.insert("offset".into(), Value::F64(-1.0));

        let res = host
            .manifold_topology_op("hypermedia.manifold.solidify", &args)
            .expect("solidify");
        let out_mesh = host.mesh(&res).expect("mesh handle");
        // 4 outer + 4 inner = 8 vertices
        assert_eq!(out_mesh.positions.len(), 8);
        // 2 outer + 2 inner + 4 boundary rim quads (8 triangles) = 12 triangles
        assert_eq!(out_mesh.triangles.len(), 12);
    }

    #[test]
    fn test_manifold_bisect_plane() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert(
            "plane_origin".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
        );
        args.insert(
            "plane_normal".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(1.0)]),
        );
        args.insert("clear_outer".into(), Value::Bool(true));
        args.insert("clear_inner".into(), Value::Bool(false));
        args.insert("fill_cap".into(), Value::Bool(true));

        let res = host
            .manifold_topology_op("hypermedia.manifold.bisect_plane", &args)
            .expect("bisect_plane");
        let out_mesh = host.mesh(&res).expect("mesh handle");
        assert!(!out_mesh.triangles.is_empty());
        for p in &out_mesh.positions {
            assert!(p[2] <= 0.05); // Kept in negative half-space
        }
    }

    #[test]
    fn test_manifold_subdivide_catmull_clark() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("levels".into(), Value::U64(1));
        args.insert("crease".into(), Value::F64(0.0));

        let res1 = host
            .manifold_topology_op("hypermedia.manifold.subdivide_catmull_clark", &args)
            .expect("subdivide level 1");
        let mesh1 = host.mesh(&res1).expect("mesh handle");
        // 4 * 4 = 16 triangles
        assert_eq!(mesh1.triangles.len(), 16);

        args.insert("levels".into(), Value::U64(2));
        args.insert("crease".into(), Value::F64(0.8));
        let res2 = host
            .manifold_topology_op("hypermedia.manifold.subdivide_catmull_clark", &args)
            .expect("subdivide level 2");
        let mesh2 = host.mesh(&res2).expect("mesh handle");
        // 4 * 16 = 64 triangles
        assert_eq!(mesh2.triangles.len(), 64);
    }
}

//! Manifold Lazy Fields, SDF Brick Dual Contouring, Instancing & FSPM Botany.
//!
//! Provides deterministic capability bindings for:
//! 1. Zero-heap chunked Lazy Field Evaluator (`VFieldEvaluator`) across points, edges, faces, and instances.
//! 2. Exact narrow-band Signed Distance Field rasterizer into Morton/dense SDF bricks (`Q42SDF16`).
//! 3. Dual Contouring / Surface Nets mesh extraction with gradient-guided cell vertex placement.
//! 4. Prototype point instancing with preview mesh and transform metadata.
//! 5. Welded instance realization flattening instanced hierarchies into single solid meshes.
//! 6. Biophysical Functional-Structural Plant Model (FSPM) vascular branching tree generation
//!    governed by Murray's Law and Hagen-Poiseuille hydraulic conductance.

mod fields_instances;
mod sdf_botany;

use crate::*;
use qualia_core_db::render::assets::Mesh;
use vibe::Value;

/// Magic header bytes for deterministic 16^3 / variable resolution SDF bricks.
pub const SDF_BRICK_MAGIC: &[u8; 8] = b"Q42SDF16";

/// Deterministic chunk evaluation size for lazy vector fields.
pub const CHUNK_SIZE: usize = 256;

// ── 3D Geometry Utilities ───────────────────────────────────────────────────

#[inline]
pub(super) fn p_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
pub(super) fn p_add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
pub(super) fn p_scale(a: [f32; 3], s: f32) -> [f32; 3] {
    [a[0] * s, a[1] * s, a[2] * s]
}

#[inline]
pub(super) fn p_dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
pub(super) fn p_cross(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
pub(super) fn p_len_sq(a: [f32; 3]) -> f32 {
    a[0] * a[0] + a[1] * a[1] + a[2] * a[2]
}

#[inline]
pub(super) fn p_normalize(a: [f32; 3]) -> [f32; 3] {
    let l_sq = p_len_sq(a);
    if l_sq > 1e-12 {
        let inv = 1.0 / l_sq.sqrt();
        [a[0] * inv, a[1] * inv, a[2] * inv]
    } else {
        [0.0, 1.0, 0.0]
    }
}

/// Computes the exact closest point on a 3D triangle to a query point P
/// using Voronoi feature regions (Ericson RTCD algorithm).
pub(super) fn closest_point_on_triangle(
    p: [f32; 3],
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
) -> [f32; 3] {
    let ab = p_sub(b, a);
    let ac = p_sub(c, a);
    let ap = p_sub(p, a);

    let d1 = p_dot(ab, ap);
    let d2 = p_dot(ac, ap);
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }

    let bp = p_sub(p, b);
    let d3 = p_dot(ab, bp);
    let d4 = p_dot(ac, bp);
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }

    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        let v = d1 / (d1 - d3);
        return p_add(a, p_scale(ab, v));
    }

    let cp = p_sub(p, c);
    let d5 = p_dot(ab, cp);
    let d6 = p_dot(ac, cp);
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }

    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        let w = d2 / (d2 - d6);
        return p_add(a, p_scale(ac, w));
    }

    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        let w = (d4 - d3) / ((d4 - d3) + (d5 - d6));
        return p_add(b, p_scale(p_sub(c, b), w));
    }

    let denom = 1.0 / (va + vb + vc);
    let v = vb * denom;
    let w = vc * denom;
    p_add(a, p_add(p_scale(ab, v), p_scale(ac, w)))
}

/// Möller-Trumbore ray-triangle intersection test returning true if ray (origin, dir) intersects.
pub(super) fn ray_intersects_triangle(
    orig: [f32; 3],
    dir: [f32; 3],
    a: [f32; 3],
    b: [f32; 3],
    c: [f32; 3],
) -> bool {
    let edge1 = p_sub(b, a);
    let edge2 = p_sub(c, a);
    let h = p_cross(dir, edge2);
    let a_det = p_dot(edge1, h);

    if a_det.abs() < 1e-8 {
        return false;
    }

    let f = 1.0 / a_det;
    let s = p_sub(orig, a);
    let u = f * p_dot(s, h);
    if !(0.0..=1.0).contains(&u) {
        return false;
    }

    let q = p_cross(s, edge1);
    let v = f * p_dot(dir, q);
    if v < 0.0 || u + v > 1.0 {
        return false;
    }

    let t = f * p_dot(edge2, q);
    t > 1e-5
}

// ── AssetHost Extension Implementation ──────────────────────────────────────

impl AssetHost<'_> {
    /// Dispatch entrypoint for Wave 7 Manifold Lazy Fields, SDF Bricks, Instancing & Botany.
    pub(crate) fn manifold_fields_sdf_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.manifold.evaluate_field"
            | "manifold.evaluate_field"
            | "synth.manifold.evaluate_field" => self.manifold_evaluate_field(a),

            "hypermedia.manifold.mesh_to_sdf_brick"
            | "manifold.mesh_to_sdf_brick"
            | "synth.manifold.mesh_to_sdf_brick" => self.manifold_mesh_to_sdf_brick(a),

            "hypermedia.manifold.sdf_brick_to_mesh"
            | "manifold.sdf_brick_to_mesh"
            | "synth.manifold.sdf_brick_to_mesh" => self.manifold_sdf_brick_to_mesh(a),

            "hypermedia.manifold.instance_on_points"
            | "manifold.instance_on_points"
            | "synth.manifold.instance_on_points" => self.manifold_instance_on_points(a),

            "hypermedia.manifold.realize_instances"
            | "manifold.realize_instances"
            | "synth.manifold.realize_instances" => self.manifold_realize_instances(a),

            "hypermedia.manifold.fspm_vascular_grow"
            | "manifold.fspm_vascular_grow"
            | "synth.manifold.fspm_vascular_grow" => self.manifold_fspm_vascular_grow(a),

            _ => Err(format!("Unknown manifold fields/SDF operation: {id}")),
        }
    }

    /// Stores a 3D Mesh artifact validating budgets and finite bounds.
    pub(super) fn store_mesh_positions(
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
        if min[0] > max[0] {
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

    // ── Morphism 2: Mesh to SDF Brick ────────────────────────────────────────

    pub(super) fn manifold_mesh_to_sdf_brick(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "resolution", "padding"])?;

        let mesh = {
            let Artifact::Mesh(m) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            m.clone()
        };

        let resolution = count(a, "resolution", 16, 32)?;
        if resolution < 8 {
            return Err("resolution must be at least 8".into());
        }

        let padding = number(a, "padding", 0.2)?;
        if !padding.is_finite() || !(0.01..=10.0).contains(&padding) {
            return Err("padding outside 0.01..=10.0 range".into());
        }
        let pad_f = padding as f32;

        let total_voxels = resolution * resolution * resolution;
        self.charge(
            total_voxels * (mesh.triangles.len().max(1) * 2 + 10),
            total_voxels * 4 + 4096,
        )?;

        let origin = [
            mesh.min[0] - pad_f,
            mesh.min[1] - pad_f,
            mesh.min[2] - pad_f,
        ];
        let max_pt = [
            mesh.max[0] + pad_f,
            mesh.max[1] + pad_f,
            mesh.max[2] + pad_f,
        ];
        let span = [
            (max_pt[0] - origin[0]).max(1e-4),
            (max_pt[1] - origin[1]).max(1e-4),
            (max_pt[2] - origin[2]).max(1e-4),
        ];

        let res_denom = (resolution - 1).max(1) as f32;
        let voxel_size = [
            span[0] / res_denom,
            span[1] / res_denom,
            span[2] / res_denom,
        ];

        let ray_dir = p_normalize([1.0, 1.41421356e-4, 2.71828182e-4]);

        let mut bytes = Vec::with_capacity(36 + total_voxels * 4);
        bytes.extend_from_slice(SDF_BRICK_MAGIC);
        bytes.extend_from_slice(&(resolution as u32).to_le_bytes());
        bytes.extend_from_slice(&origin[0].to_le_bytes());
        bytes.extend_from_slice(&origin[1].to_le_bytes());
        bytes.extend_from_slice(&origin[2].to_le_bytes());
        bytes.extend_from_slice(&voxel_size[0].to_le_bytes());
        bytes.extend_from_slice(&voxel_size[1].to_le_bytes());
        bytes.extend_from_slice(&voxel_size[2].to_le_bytes());

        // Evaluate signed distance for each grid node in Morton/row-major ordering (iz, iy, ix)
        for iz in 0..resolution {
            let pz = origin[2] + (iz as f32) * voxel_size[2];
            for iy in 0..resolution {
                let py = origin[1] + (iy as f32) * voxel_size[1];
                for ix in 0..resolution {
                    let px = origin[0] + (ix as f32) * voxel_size[0];
                    let p = [px, py, pz];

                    if mesh.triangles.is_empty() {
                        bytes.extend_from_slice(&1.0f32.to_le_bytes());
                        continue;
                    }

                    let mut min_dist_sq = f32::INFINITY;
                    let mut intersections = 0usize;

                    for tri in &mesh.triangles {
                        let i0 = tri[0] as usize;
                        let i1 = tri[1] as usize;
                        let i2 = tri[2] as usize;
                        if i0 < mesh.positions.len()
                            && i1 < mesh.positions.len()
                            && i2 < mesh.positions.len()
                        {
                            let a_pt = mesh.positions[i0];
                            let b_pt = mesh.positions[i1];
                            let c_pt = mesh.positions[i2];

                            let cp = closest_point_on_triangle(p, a_pt, b_pt, c_pt);
                            let d2 = p_len_sq(p_sub(p, cp));
                            if d2 < min_dist_sq {
                                min_dist_sq = d2;
                            }

                            if ray_intersects_triangle(p, ray_dir, a_pt, b_pt, c_pt) {
                                intersections += 1;
                            }
                        }
                    }

                    let dist = min_dist_sq.sqrt();
                    let signed_dist = if intersections % 2 == 1 { -dist } else { dist };
                    bytes.extend_from_slice(&signed_dist.to_le_bytes());
                }
            }
        }

        self.encode("sdf", bytes)
    }
}

// ── Unit Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::SDF_BRICK_MAGIC;
    use crate::*;
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

    fn store_test_cube(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ];
        let triangles = vec![
            [0, 3, 2],
            [0, 2, 1],
            [4, 5, 6],
            [4, 6, 7],
            [0, 1, 5],
            [0, 5, 4],
            [3, 7, 6],
            [3, 6, 2],
            [0, 4, 7],
            [0, 7, 3],
            [1, 2, 6],
            [1, 6, 5],
        ];
        host.store_mesh_positions(positions, triangles)
            .expect("store test cube")
    }

    #[test]
    fn test_evaluate_field_lazy_chunking() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let cube_h = store_test_cube(&mut host);

        let mut args = BTreeMap::new();
        args.insert("mesh".into(), cube_h);
        args.insert("domain".into(), Value::String("point".into()));
        args.insert("expression".into(), Value::String("radial_wave".into()));
        args.insert("amplitude".into(), Value::F64(0.2));
        args.insert("frequency".into(), Value::F64(1.5));
        args.insert("apply_displacement".into(), Value::Bool(true));

        let res = host
            .manifold_fields_sdf_op("hypermedia.manifold.evaluate_field", &args)
            .expect("evaluate_field point");
        let rec = record(&res).expect("record");

        assert_eq!(rec.get("domain"), Some(&Value::String("point".into())));
        assert_eq!(rec.get("element_count"), Some(&Value::U64(8)));
        assert_eq!(rec.get("chunk_count"), Some(&Value::U64(1)));

        let out_mesh_h = rec.get("mesh").expect("mesh handle");
        let Artifact::Mesh(displaced) = host.workspace.get(out_mesh_h).expect("mesh artifact")
        else {
            panic!("expected mesh");
        };
        assert_eq!(displaced.positions.len(), 8);
        assert_eq!(displaced.triangles.len(), 12);
    }

    #[test]
    fn test_mesh_to_sdf_brick_and_dual_contouring_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let cube_h = store_test_cube(&mut host);

        // 1. Mesh to SDF Brick
        let mut sdf_args = BTreeMap::new();
        sdf_args.insert("mesh".into(), cube_h);
        sdf_args.insert("resolution".into(), Value::U64(16));
        sdf_args.insert("padding".into(), Value::F64(0.25));

        let sdf_res = host
            .manifold_fields_sdf_op("hypermedia.manifold.mesh_to_sdf_brick", &sdf_args)
            .expect("mesh_to_sdf_brick");

        let encoded = host.workspace.encoded(&sdf_res).expect("encoded sdf");
        assert_eq!(encoded.kind, "sdf");
        assert_eq!(&encoded.bytes[0..8], SDF_BRICK_MAGIC);
        // Header 36 bytes + 16^3 * 4 = 36 + 16384 = 16420 bytes
        assert_eq!(encoded.bytes.len(), 36 + 16 * 16 * 16 * 4);

        // 2. SDF Brick to Mesh (Dual Contouring)
        let mut dc_args = BTreeMap::new();
        dc_args.insert("sdf".into(), sdf_res);
        dc_args.insert("iso_level".into(), Value::F64(0.0));

        let mesh_res = host
            .manifold_fields_sdf_op("hypermedia.manifold.sdf_brick_to_mesh", &dc_args)
            .expect("sdf_brick_to_mesh");

        let Artifact::Mesh(extracted) = host.workspace.get(&mesh_res).expect("extracted mesh")
        else {
            panic!("expected mesh artifact");
        };
        assert!(!extracted.positions.is_empty());
        assert!(!extracted.triangles.is_empty());
        // Must be a reconstructed cube surface, not the empty-brick proxy box.
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &extracted.positions {
            assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        for i in 0..3 {
            assert!(
                min[i] < -0.5 && max[i] > 0.5,
                "SDF round-trip AABB must overlap the unit cube, axis {i} [{}, {}]",
                min[i],
                max[i]
            );
            assert!(
                min[i] > -1.5 && max[i] < 1.5,
                "SDF round-trip AABB must stay near the padded cube, axis {i} [{}, {}]",
                min[i],
                max[i]
            );
        }
        let span = (max[0] - min[0]).min(max[1] - min[1]).min(max[2] - min[2]);
        assert!(span > 1.0, "extracted solid too small to be the source cube, span={span}");
    }

    #[test]
    fn test_instance_on_points_and_realize() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let proto_h = store_test_cube(&mut host);

        let mut inst_args = BTreeMap::new();
        inst_args.insert("prototype".into(), proto_h);
        inst_args.insert(
            "points".into(),
            Value::List(vec![
                Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]),
                Value::List(vec![Value::F64(3.0), Value::F64(0.0), Value::F64(0.0)]),
            ]),
        );
        inst_args.insert("scale".into(), Value::F64(0.5));

        let inst_res = host
            .manifold_fields_sdf_op("hypermedia.manifold.instance_on_points", &inst_args)
            .expect("instance_on_points");
        let rec = record(&inst_res).expect("instance record");
        assert_eq!(rec.get("instance_count"), Some(&Value::U64(2)));

        let mut realize_args = BTreeMap::new();
        realize_args.insert("instances".into(), inst_res);

        let flat_mesh_h = host
            .manifold_fields_sdf_op("hypermedia.manifold.realize_instances", &realize_args)
            .expect("realize_instances");

        let Artifact::Mesh(flat_mesh) = host.workspace.get(&flat_mesh_h).expect("realized mesh")
        else {
            panic!("expected mesh artifact");
        };
        // 2 non-overlapping cubes of 8 vertices and 12 triangles = 16 vertices, 24 triangles
        assert_eq!(flat_mesh.positions.len(), 16);
        assert_eq!(flat_mesh.triangles.len(), 24);
    }

    #[test]
    fn test_fspm_vascular_grow_botany() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut args = BTreeMap::new();
        args.insert("generations".into(), Value::U64(3));
        args.insert("base_radius".into(), Value::F64(0.2));
        args.insert("base_length".into(), Value::F64(1.2));
        args.insert("branch_angle_deg".into(), Value::F64(30.0));
        args.insert("turgor_kpa".into(), Value::F64(900.0));
        args.insert("gdd_accumulated".into(), Value::F64(450.0));

        let res = host
            .manifold_fields_sdf_op("hypermedia.manifold.fspm_vascular_grow", &args)
            .expect("fspm_vascular_grow");
        let rec = record(&res).expect("botany record");

        // 3 generations dichotomous: 2^3 - 1 = 7 metamers
        assert_eq!(rec.get("metamer_count"), Some(&Value::U64(7)));
        assert_eq!(rec.get("murray_exponent"), Some(&Value::F64(3.0)));
        assert_eq!(
            rec.get("phenology_stage"),
            Some(&Value::String("VegetativeGrowth".into()))
        );

        let out_mesh_h = rec.get("mesh").expect("mesh handle");
        let Artifact::Mesh(plant_mesh) = host.workspace.get(out_mesh_h).expect("plant mesh")
        else {
            panic!("expected mesh artifact");
        };
        assert!(!plant_mesh.positions.is_empty());
        assert!(!plant_mesh.triangles.is_empty());
    }
}

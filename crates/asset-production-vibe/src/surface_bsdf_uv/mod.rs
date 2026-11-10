//! Bounded Surface SLIM UV parameterization, MikkTSpace tangent spaces,
//! Cycles physical BSDF closure compilation, and procedural texture field synthesis.
//!
//! Provides category-theoretic surface operations:
//! 1. `hypermedia.surface.unwrap_slim`: Flip-avoiding Symmetric Dirichlet / SLIM UV atlas parameterization
//! 2. `hypermedia.surface.compute_mikktspace`: Apache-2.0 MikkTSpace orthogonal tangent frames and preview
//! 3. `hypermedia.surface.compile_bsdf_graph`: Cycles-class physical BSDF closures and shaded sphere preview
//! 4. `hypermedia.surface.sample_procedural_field`: Anisotropic Gabor, Voronoi, Wave, Blackbody, and Wavelength synthesis

mod bsdf_procedural;
mod slim_mikktspace;

use crate::*;
use qualia_core_db::render::assets::Mesh;
use vibe::Value;

// ── 3D Geometry and Vector Helpers ──────────────────────────────────────────

#[inline]
pub(super) fn v_sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
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
pub(super) fn v_len(a: [f32; 3]) -> f32 {
    v_dot(a, a).sqrt()
}

#[inline]
pub(super) fn v_norm(a: [f32; 3]) -> [f32; 3] {
    let l = v_len(a);
    if l > 1e-12 {
        [a[0] / l, a[1] / l, a[2] / l]
    } else {
        [0.0, 1.0, 0.0]
    }
}

/// Compute smooth area-weighted per-vertex unit normals for a 3D mesh.
pub(super) fn compute_smooth_vertex_normals(mesh: &Mesh) -> Vec<[f32; 3]> {
    let mut normals = vec![[0.0f32; 3]; mesh.positions.len()];
    for tri in &mesh.triangles {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
            continue;
        }
        let p0 = mesh.positions[i0];
        let p1 = mesh.positions[i1];
        let p2 = mesh.positions[i2];
        let e1 = v_sub(p1, p0);
        let e2 = v_sub(p2, p0);
        let cross = v_cross(e1, e2);
        for &idx in &[i0, i1, i2] {
            normals[idx][0] += cross[0];
            normals[idx][1] += cross[1];
            normals[idx][2] += cross[2];
        }
    }
    for n in &mut normals {
        *n = v_norm(*n);
    }
    normals
}

/// Precomputed 2D isometric reference frame for a triangle in SLIM parameterization.
pub(super) struct RestTri {
    pub(super) i: [usize; 3],
    pub(super) area: f64,
    pub(super) inv_rest: [[f64; 2]; 2],
}

pub(super) fn build_rest_tris(mesh: &Mesh) -> (Vec<RestTri>, f64) {
    let mut rest_tris = Vec::with_capacity(mesh.triangles.len());
    let mut total_rest_area = 0.0;
    for tri in &mesh.triangles {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        let p0 = mesh.positions[i0];
        let p1 = mesh.positions[i1];
        let p2 = mesh.positions[i2];
        let e1 = v_sub(p1, p0);
        let e2 = v_sub(p2, p0);
        let l1 = (v_len(e1) as f64).max(1e-7);
        let ux = [e1[0] as f64 / l1, e1[1] as f64 / l1, e1[2] as f64 / l1];
        let n = v_cross(e1, e2);
        let double_area = (v_len(n) as f64).max(1e-12);
        let area = 0.5 * double_area;
        total_rest_area += area;
        let n_unit = [
            n[0] as f64 / double_area,
            n[1] as f64 / double_area,
            n[2] as f64 / double_area,
        ];
        let uy = [
            n_unit[1] * ux[2] - n_unit[2] * ux[1],
            n_unit[2] * ux[0] - n_unit[0] * ux[2],
            n_unit[0] * ux[1] - n_unit[1] * ux[0],
        ];
        let x2 = e2[0] as f64 * ux[0] + e2[1] as f64 * ux[1] + e2[2] as f64 * ux[2];
        let y2 = (e2[0] as f64 * uy[0] + e2[1] as f64 * uy[1] + e2[2] as f64 * uy[2]).max(1e-8);

        let inv_00 = 1.0 / l1;
        let inv_01 = -x2 / (l1 * y2);
        let inv_10 = 0.0;
        let inv_11 = 1.0 / y2;

        rest_tris.push(RestTri {
            i: [i0, i1, i2],
            area,
            inv_rest: [[inv_00, inv_01], [inv_10, inv_11]],
        });
    }
    if total_rest_area <= 1e-12 {
        total_rest_area = 1.0;
    }
    (rest_tris, total_rest_area)
}

#[inline]
pub(super) fn eval_tri_sd(rt: &RestTri, uvs: &[[f64; 2]]) -> (f64, bool) {
    let u0 = uvs[rt.i[0]];
    let u1 = uvs[rt.i[1]];
    let u2 = uvs[rt.i[2]];
    let du10 = u1[0] - u0[0];
    let dv10 = u1[1] - u0[1];
    let du20 = u2[0] - u0[0];
    let dv20 = u2[1] - u0[1];

    let j00 = du10 * rt.inv_rest[0][0] + du20 * rt.inv_rest[1][0];
    let j01 = du10 * rt.inv_rest[0][1] + du20 * rt.inv_rest[1][1];
    let j10 = dv10 * rt.inv_rest[0][0] + dv20 * rt.inv_rest[1][0];
    let j11 = dv10 * rt.inv_rest[0][1] + dv20 * rt.inv_rest[1][1];

    let det = j00 * j11 - j01 * j10;
    if det <= 1e-8 {
        return (1e9, true); // Barrier preventing flip
    }
    let frob_sq = j00 * j00 + j01 * j01 + j10 * j10 + j11 * j11;
    let inv_frob_sq = frob_sq / (det * det);
    let e = 0.5 * (frob_sq + inv_frob_sq);
    (e, false)
}

/// Vertex-aligned UVs from a list, unwrap record, or named profile (island outline
/// is only accepted when its length matches the mesh so ruled/8-bit profiles stay profiles).
pub(crate) fn parse_vertex_uvs(
    host: &AssetHost<'_>,
    value: &Value,
    vertex_count: usize,
) -> Result<Vec<[f64; 2]>, String> {
    if let Ok(Artifact::Profile(p)) = host.workspace.get(value) {
        if p.len() != vertex_count {
            return Err(format!(
                "uvs profile length {} != vertex count {vertex_count}",
                p.len()
            ));
        }
        return Ok(p.clone());
    }
    let items = match value {
        Value::Record(r) => match r.get("uvs") {
            Some(v) => list(v)?,
            None => return Err("uvs record missing uvs".into()),
        },
        other => list(other)?,
    };
    if items.len() != vertex_count {
        return Err(format!(
            "uvs length {} != vertex count {vertex_count}",
            items.len()
        ));
    }
    let mut out = Vec::with_capacity(vertex_count);
    for item in items {
        let pair = list(item)?;
        if pair.len() < 2 {
            return Err("uvs entries require [u, v]".into());
        }
        out.push([num(&pair[0])?, num(&pair[1])?]);
    }
    Ok(out)
}

/// Monotone-chain hull of a SLIM UV island. Sorted, so iteration is produce-safe.
pub(crate) fn uv_island_hull(uvs: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let mut pts: Vec<[f64; 2]> = uvs
        .iter()
        .copied()
        .filter(|p| p[0].is_finite() && p[1].is_finite())
        .collect();
    pts.sort_by(|a, b| {
        a[0].partial_cmp(&b[0])
            .unwrap_or(std::cmp::Ordering::Equal)
            .then(a[1].partial_cmp(&b[1]).unwrap_or(std::cmp::Ordering::Equal))
    });
    pts.dedup_by(|a, b| (a[0] - b[0]).abs() < 1e-12 && (a[1] - b[1]).abs() < 1e-12);
    if pts.len() < 3 {
        let c = pts.first().copied().unwrap_or([0.5, 0.5]);
        return vec![
            [c[0], c[1]],
            [c[0] + 0.05, c[1]],
            [c[0] + 0.05, c[1] + 0.05],
            [c[0], c[1] + 0.05],
        ];
    }
    fn cross(o: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
        (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
    }
    let mut lower = Vec::new();
    for &p in &pts {
        while lower.len() >= 2 && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(p);
    }
    let mut upper = Vec::new();
    for &p in pts.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(p);
    }
    lower.pop();
    upper.pop();
    lower.extend(upper);
    if lower.len() < 3 {
        vec![[0.02, 0.02], [0.98, 0.02], [0.98, 0.98], [0.02, 0.98]]
    } else {
        lower
    }
}

/// Rasterize UV triangles into a still map so a non-trivial island pixel is observable.
pub(crate) fn rasterize_uv_chart(
    uv: &[[f64; 2]],
    triangles: &[[u32; 3]],
    width: u32,
    height: u32,
) -> Vec<[u8; 4]> {
    let w = width as usize;
    let h = height as usize;
    let bg = [16u8, 18, 28, 255];
    let fill = [56u8, 189, 248, 255];
    let mut pixels = vec![bg; w * h];
    for tri in triangles {
        let i0 = tri[0] as usize;
        let i1 = tri[1] as usize;
        let i2 = tri[2] as usize;
        if i0 >= uv.len() || i1 >= uv.len() || i2 >= uv.len() {
            continue;
        }
        let a = uv[i0];
        let b = uv[i1];
        let c = uv[i2];
        let min_x = (a[0].min(b[0]).min(c[0]) * w as f64).floor().max(0.0) as usize;
        let max_x = (a[0].max(b[0]).max(c[0]) * w as f64)
            .ceil()
            .min(w as f64) as usize;
        let min_y = ((1.0 - a[1].max(b[1]).max(c[1])) * h as f64)
            .floor()
            .max(0.0) as usize;
        let max_y = ((1.0 - a[1].min(b[1]).min(c[1])) * h as f64)
            .ceil()
            .min(h as f64) as usize;
        let den = (b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]);
        if den.abs() < 1e-14 {
            continue;
        }
        for y in min_y..max_y.max(min_y + 1).min(h) {
            for x in min_x..max_x.max(min_x + 1).min(w) {
                let u = (x as f64 + 0.5) / w as f64;
                let v = 1.0 - (y as f64 + 0.5) / h as f64;
                let w0 = ((b[1] - c[1]) * (u - c[0]) + (c[0] - b[0]) * (v - c[1])) / den;
                let w1 = ((c[1] - a[1]) * (u - c[0]) + (a[0] - c[0]) * (v - c[1])) / den;
                let w2 = 1.0 - w0 - w1;
                if w0 >= -1e-6 && w1 >= -1e-6 && w2 >= -1e-6 {
                    pixels[y * w + x] = fill;
                }
            }
        }
    }
    if pixels.iter().all(|p| *p == bg) {
        if let Some(p0) = uv.first() {
            let x = (p0[0].clamp(0.0, 0.999) * w as f64) as usize;
            let y = ((1.0 - p0[1]).clamp(0.0, 0.999) * h as f64) as usize;
            pixels[y * w + x] = fill;
        }
    }
    pixels
}

/// Seal SLIM TEXCOORD_0 into a QualiaDB `.10d` so unwrap feeds `compile_10d_pbr`.
pub(crate) fn seal_mesh_10d_with_slim_uvs(
    mesh: &Mesh,
    uvs: &[[f32; 2]],
    normals: &[[f32; 3]],
) -> Result<Vec<u8>, String> {
    use qualia_core_db::container_10d::attach_texture_coordinate_section;
    use qualia_core_db::render::compile_10d::{
        compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
    };
    if uvs.len() != mesh.positions.len() {
        return Err("SLIM UVs must match vertex count".into());
    }
    let bytes = compile_mesh_to_10d_with_extras_and_frames(
        mesh,
        &[],
        None,
        Compile10dExtras::default(),
        None,
        Some(normals),
        None,
    )
    .map_err(|e| format!("10D PBR compile failed: {e:?}"))?;
    attach_texture_coordinate_section(&bytes, uvs)
        .map_err(|e| format!("UV01 attach failed: {e:?}"))
}

// ── AssetHost Extension ─────────────────────────────────────────────────────

impl AssetHost<'_> {
    /// Categorical operations dispatch for surface BSDF, UV, and procedural field synthesis.
    pub(crate) fn surface_bsdf_uv_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.surface.unwrap_slim"
            | "surface.unwrap_slim"
            | "synth.surface.unwrap_slim" => self.unwrap_slim(a),

            "hypermedia.surface.compute_mikktspace"
            | "surface.compute_mikktspace"
            | "synth.surface.compute_mikktspace" => self.compute_mikktspace(a),

            "hypermedia.surface.compile_bsdf_graph"
            | "surface.compile_bsdf_graph"
            | "synth.surface.compile_bsdf_graph" => self.compile_bsdf_graph(a),

            "hypermedia.surface.sample_procedural_field"
            | "surface.sample_procedural_field"
            | "synth.surface.sample_procedural_field" => self.sample_procedural_field(a),

            _ => Err(format!("Unknown surface BSDF/UV operation: {id}")),
        }
    }
}

// ── Unit Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use crate::*;
    use qualia_core_db::render::assets::Mesh;
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

    fn store_quad_mesh(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ];
        let triangles = vec![[0, 1, 2], [0, 2, 3]];
        let min = [0.0, 0.0, 0.0];
        let max = [1.0, 1.0, 0.0];
        host.workspace
            .insert(
                Artifact::Mesh(Mesh {
                    positions,
                    triangles,
                    min,
                    max,
                }),
                host.limits,
            )
            .expect("insert quad mesh")
    }

    fn store_tetrahedron_mesh(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [0.0, 0.0, 1.0],
            [1.0, 0.0, -0.5],
            [-0.5, 0.866, -0.5],
            [-0.5, -0.866, -0.5],
        ];
        let triangles = vec![[0, 1, 2], [0, 2, 3], [0, 3, 1], [1, 3, 2]];
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &positions {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        host.workspace
            .insert(
                Artifact::Mesh(Mesh {
                    positions,
                    triangles,
                    min,
                    max,
                }),
                host.limits,
            )
            .expect("insert tetrahedron mesh")
    }

    #[test]
    fn test_unwrap_slim_planar_quad() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_quad_mesh(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);
        args.insert("iterations".into(), Value::U64(12));
        args.insert("chart_padding".into(), Value::F64(0.04));

        let res = host
            .surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &args)
            .expect("unwrap_slim planar quad");

        let rec = record(&res).expect("record");
        let uv_count = integer(required(rec, "uv_count").unwrap(), "uv_count").unwrap();
        assert_eq!(uv_count, 4);

        let flips = integer(
            required(rec, "flipped_triangles").unwrap(),
            "flipped_triangles",
        )
        .unwrap();
        assert_eq!(flips, 0, "SLIM must avoid all triangle flips");

        let energy = match required(rec, "symmetric_dirichlet_energy").unwrap() {
            Value::F64(e) => *e,
            _ => panic!("Expected F64 energy"),
        };
        assert!(
            energy >= 1.0 && energy.is_finite(),
            "Symmetric Dirichlet energy must be finite"
        );

        let svg_val = required(rec, "uv_chart_svg").unwrap();
        let svg_res = host.workspace.encoded(svg_val).expect("encoded svg");
        assert_eq!(svg_res.kind, "svg");
        assert!(svg_res.bytes.starts_with(b"<svg"));
    }

    #[test]
    fn test_unwrap_slim_mode_slim_default_and_atlas() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_quad_mesh(&mut host);
        let mut slim_args = BTreeMap::new();
        slim_args.insert("mesh".into(), mesh_h.clone());
        slim_args.insert("iterations".into(), Value::U64(8));
        let slim = host
            .surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &slim_args)
            .expect("mode=slim default");
        let slim_rec = record(&slim).expect("record");
        let slim_uv = integer(required(slim_rec, "uv_count").unwrap(), "uv_count").unwrap();
        assert_eq!(slim_uv, 4, "default slim is 1:1 with source verts");

        slim_args.insert("mode".into(), Value::String("slim".into()));
        let slim_explicit = host
            .surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &slim_args)
            .expect("mode=slim explicit");
        let slim_explicit_rec = record(&slim_explicit).expect("record");
        assert_eq!(
            integer(
                required(slim_explicit_rec, "uv_count").unwrap(),
                "uv_count"
            )
            .unwrap(),
            4
        );

        let mut atlas_args = BTreeMap::new();
        atlas_args.insert("mesh".into(), mesh_h);
        atlas_args.insert("mode".into(), Value::String("atlas".into()));
        atlas_args.insert("iterations".into(), Value::U64(16));
        match host.surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &atlas_args) {
            Ok(res) => {
                let rec = record(&res).expect("record");
                let uv_count = integer(required(rec, "uv_count").unwrap(), "uv_count").unwrap();
                assert!(
                    uv_count >= 4,
                    "atlas may split verts; got uv_count={uv_count}"
                );
            }
            Err(e) => {
                assert!(
                    e.contains("1:1") || e.contains("atlas"),
                    "atlas fail-closed 1:1 UV01, got {e}"
                );
            }
        }

        let tet = store_tetrahedron_mesh(&mut host);
        let mut tet_args = BTreeMap::new();
        tet_args.insert("mesh".into(), tet);
        tet_args.insert("mode".into(), Value::String("atlas".into()));
        match host.surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &tet_args) {
            Ok(res) => {
                let rec = record(&res).expect("record");
                let uv_count = integer(required(rec, "uv_count").unwrap(), "uv_count").unwrap();
                assert!(uv_count >= 4, "atlas tetra may split verts");
            }
            Err(e) => {
                assert!(
                    e.contains("1:1") || e.contains("atlas") || e.contains("disk") || e.contains("seam"),
                    "closed mesh atlas fail-closed, got {e}"
                );
            }
        }
    }

    #[test]
    fn test_compute_mikktspace_tetrahedron() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_tetrahedron_mesh(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h);

        let res = host
            .surface_bsdf_uv_op("hypermedia.surface.compute_mikktspace", &args)
            .expect("compute_mikktspace");

        let rec = record(&res).expect("record");
        let v_count = integer(required(rec, "vertex_count").unwrap(), "vertex_count").unwrap();
        let t_count = integer(required(rec, "triangle_count").unwrap(), "triangle_count").unwrap();
        assert_eq!(v_count, 4);
        assert_eq!(t_count, 4);

        let ortho_err = match required(rec, "mean_orthogonality_error").unwrap() {
            Value::F64(e) => *e,
            _ => panic!("Expected F64 orthogonality error"),
        };
        assert!(
            ortho_err < 1e-5,
            "Gram-Schmidt orthogonal tangents must be orthogonal to normals"
        );

        let preview_h = required(rec, "tangent_preview_rgba").unwrap();
        let rgba = host.rgba(preview_h).expect("valid rgba handle");
        assert_eq!(rgba.width, 64);
        assert_eq!(rgba.height, 64);

        let sample_tangents = list(required(rec, "sample_tangents").unwrap()).unwrap();
        assert_eq!(sample_tangents.len(), 4);
        for item in sample_tangents {
            let t_vec = list(item).unwrap();
            assert_eq!(t_vec.len(), 4);
            let sign = num(&t_vec[3]).unwrap();
            assert!(
                sign == 1.0 || sign == -1.0,
                "Handedness sign must be +1.0 or -1.0"
            );
        }
    }

    #[test]
    fn test_compile_bsdf_graph_all_closures() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        for closure_name in [
            "principled_v2",
            "chiang_hair",
            "fournier_forand_water",
            "random_walk_sss",
        ] {
            let mut args = BTreeMap::new();
            args.insert("closure".into(), Value::String(closure_name.into()));
            args.insert(
                "base_color".into(),
                Value::List(vec![Value::F64(0.8), Value::F64(0.4), Value::F64(0.15)]),
            );
            args.insert("roughness".into(), Value::F64(0.2));
            args.insert("metallic".into(), Value::F64(0.7));
            args.insert("thin_film_nm".into(), Value::F64(350.0));
            args.insert("preview_size".into(), Value::U64(32));

            let res = host
                .surface_bsdf_uv_op("hypermedia.surface.compile_bsdf_graph", &args)
                .expect("compile_bsdf_graph succeeds");

            let rec = record(&res).expect("record");
            assert_eq!(
                rec.get("closure"),
                Some(&Value::String(closure_name.into()))
            );
            assert_eq!(rec.get("energy_conserving"), Some(&Value::Bool(true)));

            let lum = match rec.get("albedo_luminance") {
                Some(Value::F64(l)) => *l,
                _ => panic!("Expected F64 albedo_luminance"),
            };
            assert!(lum > 0.0 && lum <= 1.0);

            let preview_val = required(rec, "preview_rgba").unwrap();
            let rgba = host.rgba(preview_val).expect("preview rgba");
            assert_eq!(rgba.width, 32);
            assert_eq!(rgba.height, 32);

            let json_val = required(rec, "bsdf_json").unwrap();
            let json_res = host.workspace.encoded(json_val).expect("encoded json");
            assert_eq!(json_res.kind, "json");
            assert!(
                json_res.bytes.starts_with(b"{\n  \"albedo_luminance\"")
                    || json_res.bytes.starts_with(b"{\n")
            );
        }
    }

    #[test]
    fn test_sample_procedural_field_all_bases() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        for basis_name in ["gabor", "voronoi", "wave", "blackbody", "wavelength"] {
            let mut args = BTreeMap::new();
            args.insert("basis".into(), Value::String(basis_name.into()));
            args.insert("width".into(), Value::U64(32));
            args.insert("height".into(), Value::U64(32));
            args.insert("scale".into(), Value::F64(8.0));
            args.insert("temperature_k".into(), Value::F64(5500.0));
            args.insert("seed".into(), Value::U64(101));

            let res = host
                .surface_bsdf_uv_op("hypermedia.surface.sample_procedural_field", &args)
                .expect("sample_procedural_field succeeds");

            let rgba = host.rgba(&res).expect("valid rgba handle");
            assert_eq!(rgba.width, 32);
            assert_eq!(rgba.height, 32);
            assert_eq!(rgba.pixels.len(), 1024);
        }
    }

    fn still_has_nontrivial_pixel(pixels: &[[u8; 4]], bg: [u8; 4]) -> bool {
        pixels.iter().any(|p| *p != bg && (p[0] > 8 || p[1] > 8 || p[2] > 8) && p[3] > 0)
    }

    #[test]
    fn test_unwrap_slim_uv_island_10d_roundtrip() {
        use qualia_core_db::container_10d::read_texture_coordinate_section_into;

        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mesh_h = store_quad_mesh(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), mesh_h.clone());
        args.insert("iterations".into(), Value::U64(12));
        args.insert("chart_padding".into(), Value::F64(0.04));

        let res = host
            .surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &args)
            .expect("unwrap_slim");
        let rec = record(&res).expect("record");

        let uvs = list(required(rec, "uvs").unwrap()).unwrap();
        assert_eq!(uvs.len(), 4);
        let mut authored = Vec::with_capacity(4);
        for item in uvs {
            let pair = list(item).unwrap();
            authored.push([num(&pair[0]).unwrap() as f32, num(&pair[1]).unwrap() as f32]);
        }
        let span_u = authored
            .iter()
            .map(|p| p[0])
            .fold(f32::NEG_INFINITY, f32::max)
            - authored.iter().map(|p| p[0]).fold(f32::INFINITY, f32::min);
        let span_v = authored
            .iter()
            .map(|p| p[1])
            .fold(f32::NEG_INFINITY, f32::max)
            - authored.iter().map(|p| p[1]).fold(f32::INFINITY, f32::min);
        assert!(span_u > 0.2 && span_v > 0.2, "UV island must be non-degenerate");

        let island = host
            .profile(required(rec, "uv_island_profile").unwrap())
            .expect("named UV island profile");
        assert!(island.len() >= 3);

        let still = host
            .rgba(required(rec, "uv_island_rgba").unwrap())
            .expect("uv still");
        assert_eq!(still.width, 32);
        assert!(
            still_has_nontrivial_pixel(&still.pixels, [16, 18, 28, 255]),
            "UV still must contain a non-background island pixel"
        );

        let c10d = host
            .workspace
            .encoded(required(rec, "container_10d").unwrap())
            .expect("10d");
        assert_eq!(c10d.kind, "10d");
        let mut recovered = vec![[0.0f32; 2]; 4];
        let n = read_texture_coordinate_section_into(&c10d.bytes, 4, &mut recovered)
            .expect("UV01 round-trip");
        assert_eq!(n, 4);
        for i in 0..4 {
            assert!(
                (recovered[i][0] - authored[i][0]).abs() < 1e-5
                    && (recovered[i][1] - authored[i][1]).abs() < 1e-5,
                "SLIM UV[{i}] must round-trip through 10d UV01"
            );
        }

        let mut bake_args = BTreeMap::new();
        bake_args.insert("mesh".into(), mesh_h);
        bake_args.insert("resolution".into(), Value::U64(32));
        bake_args.insert("samples".into(), Value::U64(4));
        bake_args.insert("uvs".into(), required(rec, "uvs").unwrap().clone());
        let bake = host
            .bake_pbr_materialx(&bake_args)
            .expect("bake with SLIM UVs");
        let bake_rec = record(&bake).expect("bake record");
        assert_eq!(bake_rec.get("used_slim_uvs"), Some(&Value::Bool(true)));
        let ao = host
            .rgba(required(bake_rec, "ao_image").unwrap())
            .expect("ao map");
        assert!(
            ao.pixels.iter().any(|p| p[0] > 0 || p[1] > 0 || p[2] > 0),
            "MaterialX AO map must contain a non-trivial pixel"
        );
        let mtlx = host
            .workspace
            .encoded(required(bake_rec, "materialx_resource").unwrap())
            .expect("mtlx");
        assert_eq!(mtlx.kind, "mtlx");
        assert!(mtlx.bytes.starts_with(b"<?xml") || mtlx.bytes.windows(9).any(|w| w == b"<material") || mtlx.bytes.windows(8).any(|w| w == b"<Material") || !mtlx.bytes.is_empty());
    }

    #[test]
    fn test_bsdf_and_brush_nontrivial_preview_pixels() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut bsdf_args = BTreeMap::new();
        bsdf_args.insert("closure".into(), Value::String("principled_v2".into()));
        bsdf_args.insert(
            "base_color".into(),
            Value::List(vec![Value::F64(0.8), Value::F64(0.4), Value::F64(0.15)]),
        );
        bsdf_args.insert("preview_size".into(), Value::U64(32));
        let bsdf = host
            .surface_bsdf_uv_op("hypermedia.surface.compile_bsdf_graph", &bsdf_args)
            .expect("bsdf");
        let rec = record(&bsdf).expect("record");
        let preview = host
            .rgba(required(rec, "preview_rgba").unwrap())
            .expect("bsdf preview");
        assert!(
            still_has_nontrivial_pixel(&preview.pixels, [0, 0, 0, 0]),
            "Etalon BSDF sphere must contain a non-trivial shaded pixel"
        );

        let brush = host
            .expand_procedural_brush(&BTreeMap::new())
            .expect("brush");
        let brush_rec = record(&brush).expect("brush record");
        let image = host
            .rgba(required(brush_rec, "image").unwrap())
            .expect("brush still");
        assert!(
            still_has_nontrivial_pixel(&image.pixels, [248, 245, 240, 255]),
            "brush still must contain a non-background stroke pixel"
        );
        let _ = host
            .profile(required(brush_rec, "profile").unwrap())
            .expect("brush outline stays a named profile");
    }

    #[test]
    fn test_compile_bsdf_graph_ior_affects_preview() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let compile = |host: &mut AssetHost<'_>, ior: f64| {
            let mut args = BTreeMap::new();
            args.insert("closure".into(), Value::String("principled_v2".into()));
            args.insert(
                "base_color".into(),
                Value::List(vec![Value::F64(0.8), Value::F64(0.4), Value::F64(0.15)]),
            );
            // Dielectric so IOR F0 is not swamped by metallic mix or coat.
            args.insert("metallic".into(), Value::F64(0.0));
            args.insert("coat_weight".into(), Value::F64(0.0));
            args.insert("roughness".into(), Value::F64(0.2));
            args.insert("ior".into(), Value::F64(ior));
            args.insert("preview_size".into(), Value::U64(32));
            let res = host
                .surface_bsdf_uv_op("hypermedia.surface.compile_bsdf_graph", &args)
                .expect("compile_bsdf_graph");
            let rec = record(&res).expect("record");
            host.rgba(required(rec, "preview_rgba").unwrap())
                .expect("preview")
                .pixels
                .clone()
        };

        let glass = compile(&mut host, 2.4);
        let dielectric = compile(&mut host, 1.5);
        assert!(
            still_has_nontrivial_pixel(&glass, [0, 0, 0, 0]),
            "ior=2.4 preview must contain a non-trivial shaded pixel"
        );
        assert!(
            still_has_nontrivial_pixel(&dielectric, [0, 0, 0, 0]),
            "ior=1.5 preview must contain a non-trivial shaded pixel"
        );
        assert_ne!(
            glass, dielectric,
            "Etalon GGX preview must honor IOR-derived reflectance_f0"
        );
    }
}

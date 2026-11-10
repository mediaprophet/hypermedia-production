//! Flip-avoiding Symmetric Dirichlet SLIM UV parameterization and
//! Apache-2.0 MikkTSpace tangent space computation.

use super::{
    build_rest_tris, compute_smooth_vertex_normals, eval_tri_sd, parse_vertex_uvs,
    rasterize_uv_chart, seal_mesh_10d_with_slim_uvs, uv_island_hull, v_dot, v_len, v_norm, v_sub,
};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;
use vibe::Value;

impl AssetHost<'_> {
    // ── 1. Flip-avoiding Symmetric Dirichlet SLIM UV Parameterization ────────

    pub(super) fn unwrap_slim(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "iterations", "chart_padding"])?;

        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let nv = mesh.positions.len();
        let nt = mesh.triangles.len();
        if nv < 3 || nt == 0 {
            return Err("unwrap_slim requires at least 3 vertices and 1 triangle".into());
        }

        let iterations = count(a, "iterations", 8, 64)?;
        let chart_padding = number(a, "chart_padding", 0.02)?;
        if !(0.0..=0.25).contains(&chart_padding) || !chart_padding.is_finite() {
            return Err("chart_padding outside 0.0..=0.25".into());
        }

        let work_units = nv * 15 + nt * 25 + iterations * (nv + nt) * 6 + nv * 50 + 2048;
        self.charge(work_units, work_units * 16 + 65536)?;

        // Build vertex adjacency and edge frequency
        let mut adj: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); nv];
        let mut edge_count: BTreeMap<[u32; 2], usize> = BTreeMap::new();
        for tri in &mesh.triangles {
            for k in 0..3 {
                let v0 = tri[k] as usize;
                let v1 = tri[(k + 1) % 3] as usize;
                if v0 < nv && v1 < nv {
                    adj[v0].insert(v1);
                    adj[v1].insert(v0);
                    let ek = if tri[k] < tri[(k + 1) % 3] {
                        [tri[k], tri[(k + 1) % 3]]
                    } else {
                        [tri[(k + 1) % 3], tri[k]]
                    };
                    *edge_count.entry(ek).or_insert(0) += 1;
                }
            }
        }

        // Boundary vertices are those incident on edges with frequency == 1
        let mut is_boundary = vec![false; nv];
        let mut boundary_neighbors: Vec<Vec<usize>> = vec![Vec::new(); nv];
        for (&[v0, v1], &cnt) in &edge_count {
            if cnt == 1 {
                let i0 = v0 as usize;
                let i1 = v1 as usize;
                is_boundary[i0] = true;
                is_boundary[i1] = true;
                boundary_neighbors[i0].push(i1);
                boundary_neighbors[i1].push(i0);
            }
        }

        // Find primary boundary loop
        let mut boundary_loop = Vec::new();
        if let Some(start_v) = is_boundary.iter().position(|&b| b) {
            let mut visited = BTreeSet::new();
            let mut curr = start_v;
            while !visited.contains(&curr) {
                visited.insert(curr);
                boundary_loop.push(curr);
                let next_opt = boundary_neighbors[curr]
                    .iter()
                    .copied()
                    .find(|n| !visited.contains(n));
                match next_opt {
                    Some(nxt) => curr = nxt,
                    None => break,
                }
            }
        }

        let mut uv = vec![[0.5f64, 0.5f64]; nv];

        if boundary_loop.len() >= 3 {
            // Map boundary loop to a circle centered in [0, 1]^2
            let mut cum_len = vec![0.0f64; boundary_loop.len() + 1];
            for i in 0..boundary_loop.len() {
                let p0 = mesh.positions[boundary_loop[i]];
                let p1 = mesh.positions[boundary_loop[(i + 1) % boundary_loop.len()]];
                let d = (v_len(v_sub(p1, p0)) as f64).max(1e-7);
                cum_len[i + 1] = cum_len[i] + d;
            }
            let total_len = cum_len[boundary_loop.len()].max(1e-6);
            let radius = 0.5 - chart_padding;
            for (i, &v) in boundary_loop.iter().enumerate() {
                let theta = 2.0 * PI * (cum_len[i] / total_len);
                uv[v][0] = 0.5 + radius * theta.cos();
                uv[v][1] = 0.5 + radius * theta.sin();
            }

            // Tutte harmonic relaxation for interior vertices
            for _ in 0..40 {
                for i in 0..nv {
                    if is_boundary[i] || adj[i].is_empty() {
                        continue;
                    }
                    let deg = adj[i].len() as f64;
                    let mut sum_u = 0.0;
                    let mut sum_v = 0.0;
                    for &nb in &adj[i] {
                        sum_u += uv[nb][0];
                        sum_v += uv[nb][1];
                    }
                    uv[i][0] = sum_u / deg;
                    uv[i][1] = sum_v / deg;
                }
            }
        } else {
            // Closed mesh: spherical/conformal initial mapping
            let c = mesh.centroid();
            for i in 0..nv {
                let p = mesh.positions[i];
                let d = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
                let r = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt().max(1e-6);
                let phi = (d[2] as f64).atan2(d[0] as f64);
                let theta = ((d[1] / r) as f64).clamp(-1.0, 1.0).asin();
                uv[i][0] = (phi + PI) / (2.0 * PI);
                uv[i][1] = (theta + PI * 0.5) / PI;
            }
        }

        let (rest_tris, total_rest_area) = build_rest_tris(&mesh);

        // Map vertex to incident triangles for local energy evaluation
        let mut vert_tris: Vec<Vec<usize>> = vec![Vec::new(); nv];
        for (t_idx, rt) in rest_tris.iter().enumerate() {
            vert_tris[rt.i[0]].push(t_idx);
            vert_tris[rt.i[1]].push(t_idx);
            vert_tris[rt.i[2]].push(t_idx);
        }

        // Ensure initial boundary loop winding matches triangle orientation
        let initial_flips = rest_tris.iter().filter(|rt| eval_tri_sd(rt, &uv).1).count();
        if initial_flips * 2 > rest_tris.len() {
            for u in &mut uv {
                u[1] = 1.0 - u[1];
            }
        }

        // SLIM Optimization: Local Coordinate Descent with Backtracking Line Search
        let eval_local = |v: usize, uvs: &[[f64; 2]]| -> (f64, bool) {
            let mut sum = 0.0;
            for &t_idx in &vert_tris[v] {
                let (e, flipped) = eval_tri_sd(&rest_tris[t_idx], uvs);
                if flipped {
                    return (sum, true);
                }
                sum += rest_tris[t_idx].area * e;
            }
            (sum, false)
        };

        for _ in 0..iterations {
            for v in 0..nv {
                if is_boundary[v] || vert_tris[v].is_empty() {
                    continue;
                }

                let (current_energy, flipped) = eval_local(v, &uv);
                if flipped {
                    continue;
                }

                let eps = 1e-5;
                let old_pos = uv[v];

                uv[v] = [old_pos[0] + eps, old_pos[1]];
                let e_u_plus = eval_local(v, &uv).0;

                uv[v] = [old_pos[0] - eps, old_pos[1]];
                let e_u_minus = eval_local(v, &uv).0;

                uv[v] = [old_pos[0], old_pos[1] + eps];
                let e_v_plus = eval_local(v, &uv).0;

                uv[v] = [old_pos[0], old_pos[1] - eps];
                let e_v_minus = eval_local(v, &uv).0;

                uv[v] = old_pos;

                let grad_u = (e_u_plus - e_u_minus) / (2.0 * eps);
                let grad_v = (e_v_plus - e_v_minus) / (2.0 * eps);
                let grad_len = (grad_u * grad_u + grad_v * grad_v).sqrt();
                if grad_len < 1e-8 || !grad_len.is_finite() {
                    continue;
                }

                let dir_u = -grad_u / grad_len;
                let dir_v = -grad_v / grad_len;

                let mut step = 0.015;
                let mut best_uv = old_pos;
                for _ in 0..5 {
                    let cand_uv = [old_pos[0] + step * dir_u, old_pos[1] + step * dir_v];
                    uv[v] = cand_uv;
                    let (cand_energy, is_flip) = eval_local(v, &uv);
                    if !is_flip && cand_energy < current_energy {
                        best_uv = cand_uv;
                        break;
                    }
                    step *= 0.5;
                }
                uv[v] = best_uv;
            }
        }

        // Normalize UV coordinates into [padding, 1 - padding]^2 while preserving aspect ratio
        let mut min_u = f64::INFINITY;
        let mut min_v = f64::INFINITY;
        let mut max_u = f64::NEG_INFINITY;
        let mut max_v = f64::NEG_INFINITY;
        for coord in &uv {
            min_u = min_u.min(coord[0]);
            min_v = min_v.min(coord[1]);
            max_u = max_u.max(coord[0]);
            max_v = max_v.max(coord[1]);
        }
        let span_u = (max_u - min_u).max(1e-8);
        let span_v = (max_v - min_v).max(1e-8);
        let span_max = span_u.max(span_v);
        let target_size = 1.0 - 2.0 * chart_padding;
        let scale = target_size / span_max;
        let offset_u = chart_padding + 0.5 * (target_size - span_u * scale);
        let offset_v = chart_padding + 0.5 * (target_size - span_v * scale);

        for coord in &mut uv {
            coord[0] = offset_u + (coord[0] - min_u) * scale;
            coord[1] = offset_v + (coord[1] - min_v) * scale;
        }

        // Compute final total energy and verify flip-free property
        let mut total_weighted_energy = 0.0;
        let mut flipped_count = 0u64;
        for rt in &rest_tris {
            let (e, flip) = eval_tri_sd(rt, &uv);
            if flip {
                flipped_count += 1;
            }
            total_weighted_energy += rt.area * e.min(1e6);
        }
        let mean_sd_energy = total_weighted_energy / total_rest_area;

        // Render standalone SVG UV wireframe chart
        let mut svg = String::with_capacity(nt * 96 + 512);
        svg.push_str(r##"<svg viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">"##);
        svg.push_str(r##"<rect width="1024" height="1024" fill="#0f172a"/>"##);
        let pad_px = chart_padding * 1024.0;
        let size_px = target_size * 1024.0;
        svg.push_str(&format!(
            r##"<rect x="{pad_px:.1}" y="{pad_px:.1}" width="{size_px:.1}" height="{size_px:.1}" fill="none" stroke="#f59e0b" stroke-dasharray="4,4" stroke-width="1"/>"##
        ));
        for tri in &mesh.triangles {
            let p0 = uv[tri[0] as usize];
            let p1 = uv[tri[1] as usize];
            let p2 = uv[tri[2] as usize];
            let (x0, y0) = (p0[0] * 1024.0, (1.0 - p0[1]) * 1024.0);
            let (x1, y1) = (p1[0] * 1024.0, (1.0 - p1[1]) * 1024.0);
            let (x2, y2) = (p2[0] * 1024.0, (1.0 - p2[1]) * 1024.0);
            svg.push_str(&format!(
                r##"<polygon points="{x0:.1},{y0:.1} {x1:.1},{y1:.1} {x2:.1},{y2:.1}" fill="rgba(56,189,248,0.12)" stroke="#38bdf8" stroke-width="0.75"/>"##
            ));
        }
        svg.push_str("</svg>");

        let svg_handle = self.encode("svg", svg.into_bytes())?;

        let uv_f32: Vec<[f32; 2]> = uv.iter().map(|c| [c[0] as f32, c[1] as f32]).collect();
        let normals = compute_smooth_vertex_normals(&mesh);
        let container_10d =
            self.encode("10d", seal_mesh_10d_with_slim_uvs(&mesh, &uv_f32, &normals)?)?;

        let island_pts = uv_island_hull(&uv);
        let uv_island_profile = match self.store_profile(island_pts) {
            Ok(h) => h,
            Err(_) => {
                let pad = chart_padding.max(0.02);
                self.store_profile(vec![
                    [pad, pad],
                    [1.0 - pad, pad],
                    [1.0 - pad, 1.0 - pad],
                    [pad, 1.0 - pad],
                ])?
            }
        };

        let uv_island_rgba = self.store_rgba(32, 32, rasterize_uv_chart(&uv, &mesh.triangles, 32, 32))?;

        let sample_uvs: Vec<Value> = uv
            .iter()
            .map(|coord| Value::List(vec![Value::F64(coord[0]), Value::F64(coord[1])]))
            .collect();

        Ok(Value::Record(BTreeMap::from([
            ("uv_count".into(), Value::U64(uv.len() as u64)),
            (
                "symmetric_dirichlet_energy".into(),
                Value::F64(mean_sd_energy),
            ),
            ("flipped_triangles".into(), Value::U64(flipped_count)),
            ("uv_chart_svg".into(), svg_handle),
            ("uvs".into(), Value::List(sample_uvs)),
            ("uv_island_profile".into(), uv_island_profile),
            ("container_10d".into(), container_10d),
            ("uv_island_rgba".into(), uv_island_rgba),
        ])))
    }

    // ── 2. Apache-2.0 MikkTSpace Tangent Space Computation ──────────────────

    pub(super) fn compute_mikktspace(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "uvs"])?;

        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let nv = mesh.positions.len();
        let nt = mesh.triangles.len();
        if nv == 0 || nt == 0 {
            return Err("compute_mikktspace requires non-empty mesh".into());
        }

        self.charge(nv * 12 + nt * 18, (nv + nt) * 32 + 65536)?;

        let normals = compute_smooth_vertex_normals(&mesh);

        // SLIM UVs when unwrap was invoked; spherical TEXCOORD_0 otherwise.
        let uvs = if let Some(v) = a.get("uvs") {
            parse_vertex_uvs(self, v, nv)?
        } else {
            let c = mesh.centroid();
            let mut generated = Vec::with_capacity(nv);
            for p in &mesh.positions {
                let d = [p[0] - c[0], p[1] - c[1], p[2] - c[2]];
                let r = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                if r > 1e-6 {
                    let phi = (d[2] as f64).atan2(d[0] as f64);
                    let theta = ((d[1] / r) as f64).clamp(-1.0, 1.0).asin();
                    generated.push([(phi + PI) / (2.0 * PI), (theta + PI * 0.5) / PI]);
                } else {
                    generated.push([0.5, 0.5]);
                }
            }
            generated
        };

        let uv_f32: Vec<[f32; 2]> = uvs.iter().map(|u| [u[0] as f32, u[1] as f32]).collect();
        let indices: Vec<u32> = mesh.triangles.iter().flatten().copied().collect();
        let mut tangents = vec![[0.0f32; 4]; nv];
        let mut scratch = vec![[0.0f32; 6]; nv];
        qualia_core_db::etalon_render::surface_frames::tangents_from_uvs_into(
            &mesh.positions,
            &uv_f32,
            &normals,
            &indices,
            &mut tangents,
            &mut scratch,
        )
        .map_err(|e| format!("Etalon tangent frames: {e:?}"))?;

        let mut total_ortho_err = 0.0f64;
        for i in 0..nv {
            total_ortho_err += v_dot(normals[i], [tangents[i][0], tangents[i][1], tangents[i][2]])
                .abs() as f64;
        }
        let mean_ortho_err = total_ortho_err / nv as f64;

        // Render 64x64 normal/tangent preview RGBA map
        let mut preview_pixels = Vec::with_capacity(64 * 64);
        for y in 0..64 {
            for x in 0..64 {
                let u = (x as f64 + 0.5) / 64.0 * 2.0 - 1.0;
                let v = 1.0 - (y as f64 + 0.5) / 64.0 * 2.0;
                let r2 = u * u + v * v;
                if r2 <= 1.0 {
                    let z = (1.0 - r2).sqrt();
                    let n = [u as f32, v as f32, z as f32];
                    let r_xy = (u * u + v * v).sqrt();
                    let t_raw = if r_xy > 1e-6 {
                        [-v as f32 / r_xy as f32, u as f32 / r_xy as f32, 0.0]
                    } else {
                        [1.0, 0.0, 0.0]
                    };
                    let nt = v_dot(n, t_raw);
                    let t_ortho = v_norm([
                        t_raw[0] - nt * n[0],
                        t_raw[1] - nt * n[1],
                        t_raw[2] - nt * n[2],
                    ]);
                    let r = ((t_ortho[0] * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                    let g = ((t_ortho[1] * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                    let b = ((t_ortho[2] * 0.5 + 0.5) * 255.0).clamp(0.0, 255.0) as u8;
                    preview_pixels.push([r, g, b, 255]);
                } else {
                    preview_pixels.push([0, 0, 0, 0]);
                }
            }
        }

        let preview_handle = self.store_rgba(64, 64, preview_pixels)?;

        let sample_count = nv.min(64);
        let sample_tangents: Vec<Value> = tangents[..sample_count]
            .iter()
            .map(|t| {
                Value::List(vec![
                    Value::F64(t[0] as f64),
                    Value::F64(t[1] as f64),
                    Value::F64(t[2] as f64),
                    Value::F64(t[3] as f64),
                ])
            })
            .collect();

        Ok(Value::Record(BTreeMap::from([
            ("vertex_count".into(), Value::U64(nv as u64)),
            ("triangle_count".into(), Value::U64(nt as u64)),
            (
                "mean_orthogonality_error".into(),
                Value::F64(mean_ortho_err),
            ),
            ("tangent_preview_rgba".into(), preview_handle),
            ("sample_tangents".into(), Value::List(sample_tangents)),
        ])))
    }
}

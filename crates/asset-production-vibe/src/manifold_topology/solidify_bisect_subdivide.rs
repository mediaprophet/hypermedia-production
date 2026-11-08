//! Two-sided shell solidification, planar bisection with cap triangulation,
//! and Catmull-Clark / Loop subdivision with semi-sharp creases.

use super::{compute_smooth_normals, v_add, v_cross, v_dot, v_len_sq, v_normalize, v_scale, v_sub};
use crate::*;
use std::collections::{HashMap, HashSet};
use vibe::Value;

impl AssetHost<'_> {
    // ─────────────────────────────────────────────────────────────────────────
    // 4. Two-Sided Shell Solidification with Boundary Rim Stitching
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_solidify(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "thickness", "offset"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let thickness = number(a, "thickness", 0.1)?;
        if !(-10.0..=10.0).contains(&thickness) || thickness.abs() < 1e-6 {
            return Err("thickness outside non-zero -10.0..=10.0 range".into());
        }
        let offset = number(a, "offset", -1.0)?;
        if !(-1.0..=1.0).contains(&offset) {
            return Err("offset outside -1.0..=1.0 range".into());
        }

        let d_outer = (thickness * (1.0 + offset) * 0.5) as f32;
        let d_inner = (thickness * (offset - 1.0) * 0.5) as f32;

        let normals = compute_smooth_normals(&mesh.positions, &mesh.triangles);
        let nv = mesh.positions.len();

        let mut out_positions: Vec<[f32; 3]> = Vec::with_capacity(nv * 2);
        // Outer surface vertices
        for i in 0..nv {
            out_positions.push(v_add(mesh.positions[i], v_scale(normals[i], d_outer)));
        }
        // Inner surface vertices
        for i in 0..nv {
            out_positions.push(v_add(mesh.positions[i], v_scale(normals[i], d_inner)));
        }

        let mut out_triangles: Vec<[u32; 3]> = Vec::with_capacity(mesh.triangles.len() * 2 + 32);

        // Outer surface triangles (original winding)
        for tri in &mesh.triangles {
            out_triangles.push(*tri);
        }

        // Inner surface triangles (reversed winding)
        let nv_u32 = nv as u32;
        for tri in &mesh.triangles {
            out_triangles.push([nv_u32 + tri[0], nv_u32 + tri[2], nv_u32 + tri[1]]);
        }

        // Identify boundary edges (incident to exactly 1 triangle)
        let mut edge_counts: HashMap<(u32, u32), (usize, u32, u32)> = HashMap::new();
        for tri in &mesh.triangles {
            for i in 0..3 {
                let u = tri[i];
                let v = tri[(i + 1) % 3];
                let key = if u < v { (u, v) } else { (v, u) };
                let entry = edge_counts.entry(key).or_insert((0, u, v));
                entry.0 += 1;
            }
        }

        // Stitch boundary rim quads
        for (_, (cnt, u, v)) in edge_counts {
            if cnt == 1 {
                let u_inner = nv_u32 + u;
                let v_inner = nv_u32 + v;
                // Quad bridging outer edge to inner edge with outward normal
                out_triangles.push([v, u, u_inner]);
                out_triangles.push([v, u_inner, v_inner]);
            }
        }

        self.store_mesh_artifact(out_positions, out_triangles)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 5. Planar Bisection with Cross-Section Cap Triangulation
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_bisect_plane(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "plane_origin",
                "plane_normal",
                "clear_inner",
                "clear_outer",
                "fill_cap",
            ],
        )?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let plane_origin = a
            .get("plane_origin")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0, 0.0, 0.0]);
        let plane_normal = a
            .get("plane_normal")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0, 0.0, 1.0]);

        let clear_inner = boolean(a, "clear_inner", false)?;
        let clear_outer = boolean(a, "clear_outer", true)?;
        let fill_cap = boolean(a, "fill_cap", true)?;

        let n = [
            plane_normal[0] as f32,
            plane_normal[1] as f32,
            plane_normal[2] as f32,
        ];
        let n_len_sq = v_len_sq(n);
        if n_len_sq < 1e-12 {
            return Err("plane_normal must be non-zero".into());
        }
        let unit_n = v_normalize(n);
        let p0 = [
            plane_origin[0] as f32,
            plane_origin[1] as f32,
            plane_origin[2] as f32,
        ];

        let mut out_positions: Vec<[f32; 3]> = mesh.positions.clone();
        let mut out_triangles: Vec<[u32; 3]> = Vec::new();
        let mut cut_points: Vec<[f32; 3]> = Vec::new();

        let dist = |p: [f32; 3]| -> f32 { v_dot(v_sub(p, p0), unit_n) };

        for tri in &mesh.triangles {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p_a = mesh.positions[i0];
            let p_b = mesh.positions[i1];
            let p_c = mesh.positions[i2];
            let d_a = dist(p_a);
            let d_b = dist(p_b);
            let d_c = dist(p_c);

            let eps = 1e-6f32;
            let is_kept = |d: f32| -> bool {
                let on_pos = d > eps;
                let on_neg = d < -eps;
                if clear_outer && on_pos {
                    false
                } else if clear_inner && on_neg {
                    false
                } else {
                    true
                }
            };

            let k_a = is_kept(d_a);
            let k_b = is_kept(d_b);
            let k_c = is_kept(d_c);

            if k_a && k_b && k_c {
                out_triangles.push(*tri);
            } else if !k_a && !k_b && !k_c {
                // Completely culled
            } else {
                // Polygon clipping against planar half-space
                let verts = [(tri[0], p_a, d_a), (tri[1], p_b, d_b), (tri[2], p_c, d_c)];
                let mut kept_loop: Vec<u32> = Vec::new();

                for i in 0..3 {
                    let cur = verts[i];
                    let next = verts[(i + 1) % 3];
                    let cur_kept = is_kept(cur.2);
                    let next_kept = is_kept(next.2);

                    if cur_kept {
                        kept_loop.push(cur.0);
                    }

                    if cur_kept != next_kept {
                        // Edge crosses cut plane
                        let denom = next.2 - cur.2;
                        let t = if denom.abs() > 1e-12 {
                            (-cur.2 / denom).clamp(0.0, 1.0)
                        } else {
                            0.5
                        };
                        let cut_p = v_add(cur.1, v_scale(v_sub(next.1, cur.1), t));
                        let cut_idx = out_positions.len() as u32;
                        out_positions.push(cut_p);
                        cut_points.push(cut_p);
                        kept_loop.push(cut_idx);
                    }
                }

                if kept_loop.len() == 3 {
                    out_triangles.push([kept_loop[0], kept_loop[1], kept_loop[2]]);
                } else if kept_loop.len() == 4 {
                    out_triangles.push([kept_loop[0], kept_loop[1], kept_loop[2]]);
                    out_triangles.push([kept_loop[0], kept_loop[2], kept_loop[3]]);
                }
            }
        }

        // Triangulate planar cap if requested
        if fill_cap && cut_points.len() >= 3 {
            // Deduplicate cut points within spatial tolerance
            let mut unique_cut: Vec<[f32; 3]> = Vec::new();
            for pt in &cut_points {
                if !unique_cut.iter().any(|q| v_len_sq(v_sub(*pt, *q)) < 1e-8) {
                    unique_cut.push(*pt);
                }
            }

            if unique_cut.len() >= 3 {
                let mut centroid = [0.0f32; 3];
                for pt in &unique_cut {
                    centroid = v_add(centroid, *pt);
                }
                let inv = 1.0 / unique_cut.len() as f32;
                centroid = v_scale(centroid, inv);

                // Build 2D orthonormal basis on the cutting plane
                let up = if unit_n[2].abs() < 0.9 {
                    [0.0, 0.0, 1.0]
                } else {
                    [1.0, 0.0, 0.0]
                };
                let u_axis = v_normalize(v_cross(up, unit_n));
                let v_axis = v_cross(unit_n, u_axis);

                let mut angles: Vec<(f32, u32)> = unique_cut
                    .iter()
                    .map(|pt| {
                        let rel = v_sub(*pt, centroid);
                        let u_coord = v_dot(rel, u_axis);
                        let v_coord = v_dot(rel, v_axis);
                        let theta = v_coord.atan2(u_coord);
                        let idx = out_positions.len() as u32;
                        out_positions.push(*pt);
                        (theta, idx)
                    })
                    .collect();

                angles.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

                let c_idx = out_positions.len() as u32;
                out_positions.push(centroid);

                let m = angles.len();
                for i in 0..m {
                    let a_idx = angles[i].1;
                    let b_idx = angles[(i + 1) % m].1;
                    let pa = out_positions[a_idx as usize];
                    let pb = out_positions[b_idx as usize];
                    let fan_n = v_cross(v_sub(pa, centroid), v_sub(pb, centroid));

                    let desired_n = if clear_outer {
                        unit_n
                    } else {
                        v_scale(unit_n, -1.0)
                    };
                    if v_dot(fan_n, desired_n) >= 0.0 {
                        out_triangles.push([c_idx, a_idx, b_idx]);
                    } else {
                        out_triangles.push([c_idx, b_idx, a_idx]);
                    }
                }
            }
        }

        // Compact unreferenced vertices from culled half-space
        let mut remap: HashMap<u32, u32> = HashMap::new();
        let mut compacted_positions: Vec<[f32; 3]> = Vec::new();
        let mut compacted_triangles: Vec<[u32; 3]> = Vec::with_capacity(out_triangles.len());
        for tri in out_triangles {
            let mut new_tri = [0u32; 3];
            for k in 0..3 {
                let old_idx = tri[k];
                let new_idx = *remap.entry(old_idx).or_insert_with(|| {
                    let idx = compacted_positions.len() as u32;
                    compacted_positions.push(out_positions[old_idx as usize]);
                    idx
                });
                new_tri[k] = new_idx;
            }
            compacted_triangles.push(new_tri);
        }

        self.store_mesh_artifact(compacted_positions, compacted_triangles)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 6. Catmull-Clark / Loop Subdivision with Semi-Sharp Creases
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_subdivide_catmull_clark(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "levels", "crease"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let levels = count(a, "levels", 1, 3)?;
        let crease = number(a, "crease", 0.0)?;
        if !(0.0..=1.0).contains(&crease) {
            return Err("crease outside 0.0..=1.0 range".into());
        }

        let blend_sharp = crease as f32;

        let mut current_positions: Vec<[f32; 3]> = mesh.positions.clone();
        let mut current_triangles: Vec<[u32; 3]> = mesh.triangles.clone();

        for _lvl in 0..levels {
            if current_triangles.len() * 4 > 200_000 {
                return Err("Subdivided mesh exceeds 200,000 triangle limit".into());
            }

            struct EdgeInfo {
                u: usize,
                v: usize,
                opposites: Vec<usize>,
            }

            let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();
            let mut edge_list: Vec<EdgeInfo> = Vec::new();

            for tri in &current_triangles {
                let indices = [tri[0] as usize, tri[1] as usize, tri[2] as usize];
                for i in 0..3 {
                    let u = indices[i];
                    let v = indices[(i + 1) % 3];
                    let opp = indices[(i + 2) % 3];
                    let key = if u < v { (u, v) } else { (v, u) };
                    if let Some(&e_idx) = edge_map.get(&key) {
                        edge_list[e_idx].opposites.push(opp);
                    } else {
                        let e_idx = edge_list.len();
                        edge_map.insert(key, e_idx);
                        edge_list.push(EdgeInfo {
                            u: key.0,
                            v: key.1,
                            opposites: vec![opp],
                        });
                    }
                }
            }

            let orig_nv = current_positions.len();

            // Compute new edge vertices
            let mut edge_vertex_indices: Vec<u32> = Vec::with_capacity(edge_list.len());
            for edge in &edge_list {
                let pa = current_positions[edge.u];
                let pb = current_positions[edge.v];
                let p_sharp = v_scale(v_add(pa, pb), 0.5);

                let p_smooth = if edge.opposites.len() >= 2 {
                    let pc = current_positions[edge.opposites[0]];
                    let pd = current_positions[edge.opposites[1]];
                    v_add(
                        v_scale(v_add(pa, pb), 0.375),
                        v_scale(v_add(pc, pd), 0.125),
                    )
                } else {
                    p_sharp
                };

                let p_blended = v_add(
                    v_scale(p_smooth, 1.0 - blend_sharp),
                    v_scale(p_sharp, blend_sharp),
                );
                let new_idx = (orig_nv + edge_vertex_indices.len()) as u32;
                edge_vertex_indices.push(new_idx);
                current_positions.push(p_blended);
            }

            // Update existing vertices with Loop stencil
            let mut vertex_neighbors: Vec<HashSet<usize>> = vec![HashSet::new(); orig_nv];
            let mut boundary_neighbors: Vec<Vec<usize>> = vec![Vec::new(); orig_nv];

            for edge in &edge_list {
                vertex_neighbors[edge.u].insert(edge.v);
                vertex_neighbors[edge.v].insert(edge.u);
                if edge.opposites.len() == 1 {
                    boundary_neighbors[edge.u].push(edge.v);
                    boundary_neighbors[edge.v].push(edge.u);
                }
            }

            for i in 0..orig_nv {
                let p_sharp = current_positions[i];
                let p_smooth = if boundary_neighbors[i].len() >= 2 {
                    let b1 = current_positions[boundary_neighbors[i][0]];
                    let b2 = current_positions[boundary_neighbors[i][1]];
                    v_add(v_scale(p_sharp, 0.75), v_scale(v_add(b1, b2), 0.125))
                } else {
                    let k = vertex_neighbors[i].len();
                    if k >= 3 {
                        let beta = if k == 3 {
                            3.0 / 16.0
                        } else {
                            3.0 / (8.0 * k as f32)
                        };
                        let mut sum_nbr = [0.0f32; 3];
                        for &nbr in &vertex_neighbors[i] {
                            sum_nbr = v_add(sum_nbr, current_positions[nbr]);
                        }
                        v_add(
                            v_scale(p_sharp, 1.0 - k as f32 * beta),
                            v_scale(sum_nbr, beta),
                        )
                    } else {
                        p_sharp
                    }
                };

                current_positions[i] = v_add(
                    v_scale(p_smooth, 1.0 - blend_sharp),
                    v_scale(p_sharp, blend_sharp),
                );
            }

            // Subdivide each triangle into 4 triangles
            let mut next_triangles: Vec<[u32; 3]> = Vec::with_capacity(current_triangles.len() * 4);
            for tri in &current_triangles {
                let v0 = tri[0] as usize;
                let v1 = tri[1] as usize;
                let v2 = tri[2] as usize;

                let e01 = edge_vertex_indices[edge_map[&(v0.min(v1), v0.max(v1))]];
                let e12 = edge_vertex_indices[edge_map[&(v1.min(v2), v1.max(v2))]];
                let e20 = edge_vertex_indices[edge_map[&(v2.min(v0), v2.max(v0))]];

                next_triangles.push([tri[0], e01, e20]);
                next_triangles.push([tri[1], e12, e01]);
                next_triangles.push([tri[2], e20, e12]);
                next_triangles.push([e01, e12, e20]);
            }

            current_triangles = next_triangles;
        }

        self.store_mesh_artifact(current_positions, current_triangles)
    }
}

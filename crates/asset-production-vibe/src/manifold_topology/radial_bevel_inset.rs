//! Radial-Edge B-Rep Euler operators, chamfer/bevel, and face inset with depth extrusion.

use super::{v_add, v_cross, v_normalize, v_scale, v_sub, M64ShellRecord, M64_MODE_SHELL};
use crate::*;
use qualia_core_db::q42::m64_matter::{
    M64ChunkDirectoryEntry, M64Header, M64PayloadInterval, M64ReadLimits, M64Reader, M64_MAGIC,
};
use std::collections::BTreeMap;
use vibe::Value;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct RadialEdge {
    v0: usize,
    v1: usize,
    /// Radial cycle of faces incident to this edge.
    incident_faces: Vec<usize>,
    crease: f32,
}

#[derive(Clone, Debug)]
struct RadialFace {
    /// Ordered cycle of vertex indices around the face loop.
    vertices: Vec<usize>,
    active: bool,
}

fn undirected_edge_slot(cycle: &[usize], v0: usize, v1: usize) -> Option<usize> {
    let n = cycle.len();
    (0..n).find(|&i| {
        let u = cycle[i];
        let v = cycle[(i + 1) % n];
        (u == v0 && v == v1) || (u == v1 && v == v0)
    })
}

fn directed_edge_slot(cycle: &[usize], from: usize, to: usize) -> Option<usize> {
    let n = cycle.len();
    (0..n).find(|&i| cycle[i] == from && cycle[(i + 1) % n] == to)
}

/// Vertices strictly between `from` and `to` walking the cycle forward.
fn cycle_interior(cycle: &[usize], from: usize, to: usize) -> Result<Vec<usize>, String> {
    let n = cycle.len();
    let start = cycle
        .iter()
        .position(|&v| v == from)
        .ok_or_else(|| "JFKE: shared-edge vertices are not adjacent in both cycles".to_string())?;
    let mut interior = Vec::new();
    let mut i = (start + 1) % n;
    let mut seen = 0usize;
    while cycle[i] != to {
        interior.push(cycle[i]);
        i = (i + 1) % n;
        seen += 1;
        if seen >= n {
            return Err("JFKE: shared-edge vertices are not adjacent in both cycles".into());
        }
    }
    Ok(interior)
}

/// Join two face cycles at the shared oriented edge (v0, v1).
/// Walks `loop2` from the far shared vertex to the near one, excluding that edge,
/// and inserts the interior path into `loop1`'s shared-edge slot.
fn splice_face_cycles(
    loop1: &[usize],
    loop2: &[usize],
    v0: usize,
    v1: usize,
) -> Result<Vec<usize>, String> {
    let slot1 = undirected_edge_slot(loop1, v0, v1)
        .ok_or_else(|| "JFKE: shared-edge vertices are not adjacent in both cycles".to_string())?;
    let a = loop1[slot1];
    let b = loop1[(slot1 + 1) % loop1.len()];
    if directed_edge_slot(loop2, b, a).is_none() {
        return Err("JFKE: shared-edge vertices are not adjacent in both cycles".into());
    }
    let interior = cycle_interior(loop2, a, b)?;
    let mut joined = Vec::with_capacity(loop1.len() + interior.len());
    joined.push(a);
    joined.extend(interior);
    let mut i = (slot1 + 1) % loop1.len();
    while i != slot1 {
        joined.push(loop1[i]);
        i = (i + 1) % loop1.len();
    }
    if joined.len() < 3 {
        return Err("JFKE: joined face cycle has fewer than 3 vertices".into());
    }
    Ok(joined)
}

impl AssetHost<'_> {
    // ─────────────────────────────────────────────────────────────────────────
    // 1. Radial-Edge B-Rep Topological Kernel + .m64 Mode 0x04 Serializer
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_radial_euler(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "ops", "crease_sharpness"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let crease_sharpness = number(a, "crease_sharpness", 0.5)?;
        if !(0.0..=1.0).contains(&crease_sharpness) {
            return Err("crease_sharpness outside 0.0..=1.0 range".into());
        }

        let mut vertices: Vec<[f32; 3]> = mesh.positions.clone();
        let mut faces: Vec<RadialFace> = mesh
            .triangles
            .iter()
            .map(|t| RadialFace {
                vertices: vec![t[0] as usize, t[1] as usize, t[2] as usize],
                active: true,
            })
            .collect();

        // Helper to reconstruct edge table from active face loops
        let rebuild_edges = |faces: &[RadialFace], crease: f32| -> Vec<RadialEdge> {
            let mut edge_map: BTreeMap<(usize, usize), usize> = BTreeMap::new();
            let mut edges: Vec<RadialEdge> = Vec::new();
            for (f_idx, face) in faces.iter().enumerate() {
                if !face.active || face.vertices.len() < 3 {
                    continue;
                }
                let n = face.vertices.len();
                for i in 0..n {
                    let u = face.vertices[i];
                    let v = face.vertices[(i + 1) % n];
                    let key = if u < v { (u, v) } else { (v, u) };
                    if let Some(&e_idx) = edge_map.get(&key) {
                        edges[e_idx].incident_faces.push(f_idx);
                    } else {
                        let e_idx = edges.len();
                        edge_map.insert(key, e_idx);
                        edges.push(RadialEdge {
                            v0: key.0,
                            v1: key.1,
                            incident_faces: vec![f_idx],
                            crease,
                        });
                    }
                }
            }
            edges
        };

        let mut edges = rebuild_edges(&faces, crease_sharpness as f32);

        // Process Euler operators or run default refinement
        if let Some(ops_val) = a.get("ops") {
            let ops_list = list(ops_val)?;
            for op_item in ops_list {
                let rec = record(op_item)?;
                let op = text(required(rec, "op")?)?;
                let edge_index = rec
                    .get("edge_index")
                    .map(|v| integer(v, "edge_index"))
                    .transpose()?
                    .unwrap_or(0);
                let face_index = rec
                    .get("face_index")
                    .map(|v| integer(v, "face_index"))
                    .transpose()?
                    .unwrap_or(0);
                let factor = number(rec, "factor", 0.5)?.clamp(0.01, 0.99);

                match op {
                    "SEMV" => {
                        if !edges.is_empty() {
                            let e_idx = edge_index % edges.len();
                            let v0 = edges[e_idx].v0;
                            let v1 = edges[e_idx].v1;
                            let t = factor as f32;
                            let p_new =
                                v_add(v_scale(vertices[v0], 1.0 - t), v_scale(vertices[v1], t));
                            let v_new = vertices.len();
                            vertices.push(p_new);

                            for &f_idx in &edges[e_idx].incident_faces {
                                let f = &mut faces[f_idx];
                                let n = f.vertices.len();
                                let mut new_loop = Vec::with_capacity(n + 1);
                                for i in 0..n {
                                    let curr = f.vertices[i];
                                    let next = f.vertices[(i + 1) % n];
                                    new_loop.push(curr);
                                    if (curr == v0 && next == v1) || (curr == v1 && next == v0) {
                                        new_loop.push(v_new);
                                    }
                                }
                                f.vertices = new_loop;
                            }
                            edges = rebuild_edges(&faces, crease_sharpness as f32);
                        }
                    }
                    "SFME" => {
                        let active_indices: Vec<usize> = faces
                            .iter()
                            .enumerate()
                            .filter(|(_, f)| f.active && f.vertices.len() >= 3)
                            .map(|(i, _)| i)
                            .collect();
                        if !active_indices.is_empty() {
                            let f_idx = active_indices[face_index % active_indices.len()];
                            let v = faces[f_idx].vertices.clone();
                            if v.len() >= 4 {
                                let f1 = vec![v[0], v[1], v[2]];
                                let mut f2 = vec![v[0], v[2]];
                                f2.extend_from_slice(&v[3..]);
                                faces[f_idx].vertices = f1;
                                faces.push(RadialFace {
                                    vertices: f2,
                                    active: true,
                                });
                            } else {
                                let v_mid = vertices.len();
                                vertices.push(v_scale(v_add(vertices[v[0]], vertices[v[1]]), 0.5));
                                faces[f_idx].vertices = vec![v[0], v_mid, v[2]];
                                faces.push(RadialFace {
                                    vertices: vec![v_mid, v[1], v[2]],
                                    active: true,
                                });
                            }
                            edges = rebuild_edges(&faces, crease_sharpness as f32);
                        }
                    }
                    "JEKV" | "JVKE" => {
                        if !edges.is_empty() {
                            let e_idx = edge_index % edges.len();
                            let v0 = edges[e_idx].v0;
                            let v1 = edges[e_idx].v1;
                            for f in &mut faces {
                                if !f.active {
                                    continue;
                                }
                                for v in &mut f.vertices {
                                    if *v == v1 {
                                        *v = v0;
                                    }
                                }
                                f.vertices.dedup();
                                if f.vertices.len() >= 2 && f.vertices.first() == f.vertices.last()
                                {
                                    f.vertices.pop();
                                }
                                if f.vertices.len() < 3 {
                                    f.active = false;
                                }
                            }
                            edges = rebuild_edges(&faces, crease_sharpness as f32);
                        }
                    }
                    "JFKE" => {
                        if edges.is_empty() {
                            return Err("JFKE: mesh has no edges".into());
                        }
                        let e_idx = edge_index % edges.len();
                        // Join requires a 2-manifold edge; extra or missing radials are not a splice.
                        if edges[e_idx].incident_faces.len() != 2 {
                            return Err("JFKE: edge is not 2-manifold".into());
                        }
                        let f1 = edges[e_idx].incident_faces[0];
                        let f2 = edges[e_idx].incident_faces[1];
                        if f1 == f2 || !faces[f1].active || !faces[f2].active {
                            return Err("JFKE: edge is not 2-manifold".into());
                        }
                        let v0 = edges[e_idx].v0;
                        let v1 = edges[e_idx].v1;
                        let joined =
                            splice_face_cycles(&faces[f1].vertices, &faces[f2].vertices, v0, v1)?;
                        faces[f1].vertices = joined;
                        faces[f2].active = false;
                        edges = rebuild_edges(&faces, crease_sharpness as f32);
                    }
                    _ => return Err(format!("Unknown Euler operator: {op}")),
                }
            }
        } else if !faces.is_empty() && faces[0].vertices.len() == 3 {
            // Default refinement: SEMV on edge 0 + SFME on face 0
            let v0 = faces[0].vertices[0];
            let v1 = faces[0].vertices[1];
            let v2 = faces[0].vertices[2];
            let p_mid = v_scale(v_add(vertices[v0], vertices[v1]), 0.5);
            let v_mid = vertices.len();
            vertices.push(p_mid);

            // SEMV: Face 0 becomes 4-gon [v0, v_mid, v1, v2]
            // SFME: Split 4-gon across v_mid to v2 into two triangles
            faces[0].vertices = vec![v0, v_mid, v2];
            faces.push(RadialFace {
                vertices: vec![v_mid, v1, v2],
                active: true,
            });
        }

        // Triangulate resulting B-Rep faces back into triangles
        let mut out_triangles: Vec<[u32; 3]> = Vec::new();
        for face in &faces {
            if !face.active || face.vertices.len() < 3 {
                continue;
            }
            let v = &face.vertices;
            if v.len() == 3 {
                out_triangles.push([v[0] as u32, v[1] as u32, v[2] as u32]);
            } else {
                for i in 1..v.len() - 1 {
                    out_triangles.push([v[0] as u32, v[i] as u32, v[i + 1] as u32]);
                }
            }
        }

        if out_triangles.is_empty() {
            return Err("Radial-Edge Euler operation produced empty mesh".into());
        }

        // Fan diagonals are not in the pre-triangulation radial table. Rebuild from
        // the emitted triangles so shell edge_indices cannot collapse to 0.
        let tri_faces: Vec<RadialFace> = out_triangles
            .iter()
            .map(|t| RadialFace {
                vertices: vec![t[0] as usize, t[1] as usize, t[2] as usize],
                active: true,
            })
            .collect();
        edges = rebuild_edges(&tri_faces, crease_sharpness as f32);

        let non_manifold_count = edges.iter().filter(|e| e.incident_faces.len() >= 3).count();

        let edge_lookup: BTreeMap<(usize, usize), usize> = edges
            .iter()
            .enumerate()
            .map(|(i, e)| ((e.v0.min(e.v1), e.v0.max(e.v1)), i))
            .collect();
        let lookup_edge = |a: usize, b: usize| -> Result<usize, String> {
            edge_lookup
                .get(&(a.min(b), a.max(b)))
                .copied()
                .ok_or_else(|| format!("fan diagonal ({a},{b}) missing from edge table"))
        };

        // Serialize .m64 Mode 0x04 Shell Records
        let record_count = out_triangles.len();
        let payload_bytes = record_count * 64;
        let total_file_size = 128 + payload_bytes;

        let mut header = M64Header::default();
        header.total_file_size = total_file_size as u64;
        header.chunk_count = 1;
        header.chunk_table_offset = 64;
        header.spatial_origin = [0.0, 0.0];
        header.elevation_datum = [0.0, 1.0];
        header.update_crc();

        let mut entry = M64ChunkDirectoryEntry {
            chunk_uuid: [0x64; 16],
            spatial_hilbert_index: 0,
            byte_offset: 128,
            byte_length: payload_bytes as u32,
            record_count: record_count as u32,
            srid_geodetic_code: 0,
            lut_bank_epoch_id: 1,
            chunk_record_mode: M64_MODE_SHELL,
            compression_flags: 0,
            bao_root_hash_prefix: [0u8; 12],
            entry_crc32c: 0,
        };
        entry.update_crc();

        let mut buffer = vec![0u8; total_file_size];
        buffer[..64].copy_from_slice(bytemuck::bytes_of(&header));
        buffer[64..128].copy_from_slice(bytemuck::bytes_of(&entry));

        for (i, tri) in out_triangles.iter().enumerate() {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = vertices[i0];
            let p1 = vertices[i1];
            let p2 = vertices[i2];
            let normal = v_normalize(v_cross(v_sub(p1, p0), v_sub(p2, p0)));
            let centroid = [
                (p0[0] + p1[0] + p2[0]) / 3.0,
                (p0[1] + p1[1] + p2[1]) / 3.0,
                (p0[2] + p1[2] + p2[2]) / 3.0,
            ];

            let e0 = lookup_edge(i0, i1)?;
            let e1 = lookup_edge(i1, i2)?;
            let e2 = lookup_edge(i2, i0)?;

            let is_nm = [e0, e1, e2].iter().any(|&idx| {
                edges
                    .get(idx)
                    .map_or(false, |e| e.incident_faces.len() >= 3)
            });

            let mut rec = M64ShellRecord {
                face_id: i as u32,
                vertex_indices: *tri,
                edge_indices: [e0 as u32, e1 as u32, e2 as u32],
                normal,
                centroid,
                crease: crease_sharpness as f32,
                flags: if is_nm { 1 } else { 0 },
                record_crc32c: 0,
            };
            rec.update_crc();

            let offset = 128 + i * 64;
            buffer[offset..offset + 64].copy_from_slice(bytemuck::bytes_of(&rec));
        }

        // Fail closed: Mode 0x04 shell records carry CRC and must parse as SHELL.
        let limits = M64ReadLimits {
            max_file_bytes: total_file_size + 1024,
            max_chunks: 4,
            max_records: record_count as u64 + 16,
        };
        let mut scratch = [M64PayloadInterval::default(); 4];
        let reader = M64Reader::parse(&buffer, limits, &mut scratch)
            .map_err(|e| format!("M64 self-verification failed: {e}"))?;
        if reader.header().magic != M64_MAGIC
            || reader
                .chunk(0)
                .map(|c| c.directory_entry().chunk_record_mode)
                != Some(M64_MODE_SHELL)
            || reader.verification().record_crc_checked != record_count as u64
        {
            return Err("M64 self-verification failed: shell mode/CRC mismatch".into());
        }

        let shell_m64 = self.encode("m64", buffer)?;
        let mesh_handle = self.store_mesh_artifact(vertices.clone(), out_triangles)?;

        let mut out = BTreeMap::new();
        out.insert("mesh".into(), mesh_handle);
        out.insert("shell_m64".into(), shell_m64);
        out.insert("vertex_count".into(), Value::U64(vertices.len() as u64));
        out.insert("edge_count".into(), Value::U64(edges.len() as u64));
        out.insert(
            "face_count".into(),
            Value::U64(faces.iter().filter(|f| f.active).count() as u64),
        );
        out.insert(
            "non_manifold_edges".into(),
            Value::U64(non_manifold_count as u64),
        );
        Ok(Value::Record(out))
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 2. Bevel / Chamfer across Mesh Faces, Edges & Corners
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_bevel(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "width", "segments"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let width = number(a, "width", 0.08)?;
        if !(1e-4..=10.0).contains(&width) {
            return Err("width outside 1e-4..=10.0 range".into());
        }
        let segments = count(a, "segments", 1, 8)?;

        let fraction = (width as f32).clamp(0.005, 0.45);
        let num_faces = mesh.triangles.len();
        let mut out_positions: Vec<[f32; 3]> =
            Vec::with_capacity(num_faces * 3 + mesh.positions.len());
        let mut out_triangles: Vec<[u32; 3]> = Vec::new();

        // 1. Shrink each triangle face toward its centroid
        let mut face_insets: Vec<[u32; 3]> = Vec::with_capacity(num_faces);
        for tri in &mesh.triangles {
            let p0 = mesh.positions[tri[0] as usize];
            let p1 = mesh.positions[tri[1] as usize];
            let p2 = mesh.positions[tri[2] as usize];
            let c = [
                (p0[0] + p1[0] + p2[0]) / 3.0,
                (p0[1] + p1[1] + p2[1]) / 3.0,
                (p0[2] + p1[2] + p2[2]) / 3.0,
            ];

            let a_prime = v_add(p0, v_scale(v_sub(c, p0), fraction));
            let b_prime = v_add(p1, v_scale(v_sub(c, p1), fraction));
            let c_prime = v_add(p2, v_scale(v_sub(c, p2), fraction));

            let base_idx = out_positions.len() as u32;
            out_positions.push(a_prime);
            out_positions.push(b_prime);
            out_positions.push(c_prime);

            face_insets.push([base_idx, base_idx + 1, base_idx + 2]);
            out_triangles.push([base_idx, base_idx + 1, base_idx + 2]);
        }

        // 2. Build edge connectivity map to bridge adjacent beveled face edges
        #[allow(dead_code)]
        struct EdgeFaceRef {
            face_idx: usize,
            v_start: u32,
            v_end: u32,
            inset_start: u32,
            inset_end: u32,
        }

        let mut edge_map: BTreeMap<(u32, u32), Vec<EdgeFaceRef>> = BTreeMap::new();
        for (f_idx, tri) in mesh.triangles.iter().enumerate() {
            let insets = face_insets[f_idx];
            for i in 0..3 {
                let u = tri[i];
                let v = tri[(i + 1) % 3];
                let iu = insets[i];
                let iv = insets[(i + 1) % 3];
                let key = if u < v { (u, v) } else { (v, u) };
                edge_map.entry(key).or_default().push(EdgeFaceRef {
                    face_idx: f_idx,
                    v_start: u,
                    v_end: v,
                    inset_start: iu,
                    inset_end: iv,
                });
            }
        }

        // Connect adjacent face edges with quad / triangle strips
        for refs in edge_map.values() {
            if refs.len() == 2 {
                let r1 = &refs[0];
                let r2 = &refs[1];

                // Orient r2 so endpoints match r1
                let (r2_start, r2_end) = if r1.v_start == r2.v_start {
                    (r2.inset_start, r2.inset_end)
                } else {
                    (r2.inset_end, r2.inset_start)
                };

                let mut prev_a = r1.inset_start;
                let mut prev_b = r1.inset_end;

                for s in 1..=segments {
                    let t = s as f32 / segments as f32;
                    let next_a = if s == segments {
                        r2_start
                    } else {
                        let idx = out_positions.len() as u32;
                        out_positions.push(v_add(
                            v_scale(out_positions[r1.inset_start as usize], 1.0 - t),
                            v_scale(out_positions[r2_start as usize], t),
                        ));
                        idx
                    };

                    let next_b = if s == segments {
                        r2_end
                    } else {
                        let idx = out_positions.len() as u32;
                        out_positions.push(v_add(
                            v_scale(out_positions[r1.inset_end as usize], 1.0 - t),
                            v_scale(out_positions[r2_end as usize], t),
                        ));
                        idx
                    };

                    out_triangles.push([prev_a, prev_b, next_b]);
                    out_triangles.push([prev_a, next_b, next_a]);

                    prev_a = next_a;
                    prev_b = next_b;
                }
            }
        }

        // 3. Connect corner fan triangles at each original vertex
        let mut vertex_insets: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for (f_idx, tri) in mesh.triangles.iter().enumerate() {
            let insets = face_insets[f_idx];
            for i in 0..3 {
                vertex_insets.entry(tri[i]).or_default().push(insets[i]);
            }
        }

        for (orig_v, insets) in vertex_insets {
            if insets.len() >= 3 {
                let center_idx = out_positions.len() as u32;
                out_positions.push(mesh.positions[orig_v as usize]);
                let m = insets.len();
                for i in 0..m {
                    let next = (i + 1) % m;
                    out_triangles.push([center_idx, insets[i], insets[next]]);
                }
            }
        }

        self.store_mesh_artifact(out_positions, out_triangles)
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 3. Inset Faces with Thickness & Normal Depth Extrusion
    // ─────────────────────────────────────────────────────────────────────────
    pub(super) fn manifold_inset(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "thickness", "depth"])?;
        let mesh = self.mesh(required(a, "mesh")?)?.clone();
        let thickness = number(a, "thickness", 0.15)?;
        if !(0.01..=0.95).contains(&thickness) {
            return Err("thickness outside 0.01..=0.95 range".into());
        }
        let depth = number(a, "depth", 0.05)?;
        if !(-10.0..=10.0).contains(&depth) {
            return Err("depth outside -10.0..=10.0 range".into());
        }

        let t_factor = thickness as f32;
        let d_factor = depth as f32;

        let mut out_positions: Vec<[f32; 3]> = mesh.positions.clone();
        let mut out_triangles: Vec<[u32; 3]> = Vec::with_capacity(mesh.triangles.len() * 7);

        for tri in &mesh.triangles {
            let v0 = tri[0];
            let v1 = tri[1];
            let v2 = tri[2];

            let p0 = mesh.positions[v0 as usize];
            let p1 = mesh.positions[v1 as usize];
            let p2 = mesh.positions[v2 as usize];

            let centroid = [
                (p0[0] + p1[0] + p2[0]) / 3.0,
                (p0[1] + p1[1] + p2[1]) / 3.0,
                (p0[2] + p1[2] + p2[2]) / 3.0,
            ];
            let normal = v_normalize(v_cross(v_sub(p1, p0), v_sub(p2, p0)));
            let normal_offset = v_scale(normal, d_factor);

            // Inner 3 inset vertices
            let q0 = v_add(
                v_add(p0, v_scale(v_sub(centroid, p0), t_factor)),
                normal_offset,
            );
            let q1 = v_add(
                v_add(p1, v_scale(v_sub(centroid, p1), t_factor)),
                normal_offset,
            );
            let q2 = v_add(
                v_add(p2, v_scale(v_sub(centroid, p2), t_factor)),
                normal_offset,
            );

            let q0_idx = out_positions.len() as u32;
            let q1_idx = q0_idx + 1;
            let q2_idx = q0_idx + 2;
            out_positions.push(q0);
            out_positions.push(q1);
            out_positions.push(q2);

            // 3 side wall quads (6 triangles)
            out_triangles.push([v0, v1, q1_idx]);
            out_triangles.push([v0, q1_idx, q0_idx]);

            out_triangles.push([v1, v2, q2_idx]);
            out_triangles.push([v1, q2_idx, q1_idx]);

            out_triangles.push([v2, v0, q0_idx]);
            out_triangles.push([v2, q0_idx, q2_idx]);

            // 1 inner inset triangle
            out_triangles.push([q0_idx, q1_idx, q2_idx]);
        }

        self.store_mesh_artifact(out_positions, out_triangles)
    }
}

#[cfg(test)]
mod splice_tests {
    use super::splice_face_cycles;

    fn cycle_eq_rotated(got: &[usize], expected: &[usize]) -> bool {
        if got.len() != expected.len() || got.is_empty() {
            return got == expected;
        }
        (0..got.len()).any(|k| {
            got.iter()
                .cycle()
                .skip(k)
                .take(got.len())
                .eq(expected.iter())
        })
    }

    #[test]
    fn test_jfke_tetrahedron_splice_is_quad_not_set_union() {
        // F0 [0,1,2] and F1 [0,2,3] along 0-2: splice, not retain+append set union.
        let joined = splice_face_cycles(&[0, 1, 2], &[0, 2, 3], 0, 2).expect("2-manifold splice");
        assert!(
            cycle_eq_rotated(&joined, &[0, 1, 2, 3]),
            "expected quad [0,1,2,3] (up to rotation), got {joined:?}"
        );
        assert!(
            !cycle_eq_rotated(&joined, &[1, 0, 2, 3]),
            "set-union bag [1,0,2,3] is not the join cycle, got {joined:?}"
        );
    }

    #[test]
    fn test_jfke_fails_closed_when_vertices_not_adjacent() {
        let err = splice_face_cycles(&[0, 1, 2], &[0, 2, 3], 0, 3).expect_err("not adjacent");
        assert!(
            err.contains("not adjacent"),
            "expected adjacency failure, got {err}"
        );
    }

    #[test]
    fn test_jfke_fails_closed_when_cycles_share_same_orientation() {
        // Both faces traverse 2→0; no reverse edge on loop2.
        let err = splice_face_cycles(&[0, 1, 2], &[0, 3, 2], 0, 2).expect_err("same orientation");
        assert!(
            err.contains("not adjacent"),
            "expected reverse-edge failure, got {err}"
        );
    }
}

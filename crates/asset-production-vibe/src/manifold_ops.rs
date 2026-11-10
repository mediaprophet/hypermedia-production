use super::*;
use std::collections::BTreeMap;
use qualia_core_db::hypermedia_authoring::Camera3D;
use qualia_core_db::container_10d::header::Container10dHeader;
use qualia_core_db::container_10d::section::{parse_section_table, SectionType};
use qualia_core_db::container_10d::surface_reading::decode_surface_reading;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, decode_10d_mesh, Compile10dExtras,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg, boolean_3::Boolean3Op, AnisotropicRemeshOptions, AnisotropyField,
    DecimateOptions, MetricTensor, Point3, RemeshOptions,
};
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn manifold_ext(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.manifold.boolean_3d"
            | "manifold.boolean_3d"
            | "hypermedia.mesh.boolean_3d" => self.manifold_boolean_3d(a),
            "hypermedia.manifold.simplify"
            | "manifold.simplify"
            | "hypermedia.mesh.simplify" => self.manifold_simplify(a),
            "hypermedia.manifold.import_mesh"
            | "manifold.import_mesh"
            | "hypermedia.mesh.import" => self.manifold_import_mesh(a),
            "hypermedia.manifold.project_dxf"
            | "manifold.project_dxf"
            | "hypermedia.mesh.dxf" => self.manifold_project_dxf(a),
            "hypermedia.manifold.compile_10d_pbr"
            | "manifold.compile_10d_pbr"
            | "hypermedia.mesh.compile_10d_pbr" => self.manifold_compile_10d_pbr(a),
            "hypermedia.manifold.loft" | "manifold.loft" | "hypermedia.mesh.loft" => {
                self.manifold_loft(a)
            }
            _ => Err(format!("Unknown manifold operation: {id}")),
        }
    }

    fn manifold_boolean_3d(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["a", "b", "op"])?;
        let op_str = text(required(a, "op")?)?;
        let op = match op_str {
            "union" => Boolean3Op::Union,
            "intersection" => Boolean3Op::Intersection,
            "difference" => Boolean3Op::Difference,
            _ => {
                return Err(format!(
                    "Unknown boolean operation: {op_str}; expected 'union', 'intersection', or 'difference'"
                ))
            }
        };

        let (va, ta) = {
            let Artifact::Mesh(mesh_a) = self.workspace.get(required(a, "a")?)? else {
                return Err("Expected mesh handle for 'a'".into());
            };
            let va: Vec<Point3> = mesh_a
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let ta = mesh_a.triangles.clone();
            (va, ta)
        };

        let (vb, tb) = {
            let Artifact::Mesh(mesh_b) = self.workspace.get(required(a, "b")?)? else {
                return Err("Expected mesh handle for 'b'".into());
            };
            let vb: Vec<Point3> = mesh_b
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let tb = mesh_b.triangles.clone();
            (vb, tb)
        };

        // Production envelope matches mesh_budget (200k/400k), not the old 50k/100k authoring cap.
        let req_v = cg::required_vertices_3(va.len(), vb.len(), ta.len(), tb.len());
        let req_t = cg::required_triangles_3(ta.len(), tb.len());
        let max_v = req_v.min(200_000);
        let max_t = req_t.min(400_000);
        self.mesh_budget(max_v, max_t)?;

        let mut out_v = vec![Point3::default(); max_v];
        let mut out_t = vec![[0u32; 3]; max_t];

        let (nv, nt) = cg::boolean_3(
            &va,
            &ta,
            &vb,
            &tb,
            op,
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Boolean 3D failed: {e:?}"))?;

        out_v.truncate(nv);
        out_t.truncate(nt);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_simplify(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "target_triangles",
                "max_error",
                "mode",
                "iterations",
                "target_edge_length",
            ],
        )?;
        let mode = match a.get("mode") {
            Some(v) => text(v)?,
            None => "decimate",
        };
        match mode {
            "decimate" => self.manifold_simplify_decimate(a),
            "remesh" => self.manifold_simplify_remesh(a),
            "anisotropic" => self.manifold_simplify_anisotropic(a),
            _ => Err(
                "Unknown simplify mode: expected 'decimate' (QEM), 'remesh' (isotropic), or 'anisotropic'"
                    .into(),
            ),
        }
    }

    fn simplify_mesh_buffers(&self, a: &Args) -> Result<(Vec<Point3>, Vec<[u32; 3]>), String> {
        let Artifact::Mesh(mesh) = self.workspace.get(required(a, "mesh")?)? else {
            return Err("Expected mesh handle".into());
        };
        let v: Vec<Point3> = mesh
            .positions
            .iter()
            .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        let t = mesh.triangles.clone();
        Ok((v, t))
    }

    fn manifold_simplify_decimate(&mut self, a: &Args) -> Result<Value, String> {
        let target_triangles = integer(required(a, "target_triangles")?, "target_triangles")?;
        if target_triangles < 4 || target_triangles > 200_000 {
            return Err("target_triangles outside 4..=200000 range".into());
        }
        let max_error = number(a, "max_error", 0.05)?;
        if !max_error.is_finite() || max_error < 1e-6 || max_error > 1e4 {
            return Err("max_error outside 1e-6..=1e4 range".into());
        }

        let (vertices, triangles) = self.simplify_mesh_buffers(a)?;
        let nv = vertices.len();
        let nt = triangles.len();
        if nt < 4 || nv < 3 {
            return Err("Mesh has too few triangles or vertices for simplification".into());
        }

        self.charge((nv + nt) * 20, nv * 64 + nt * 32)?;

        let target_faces = target_triangles.min(nt);
        let mut out_v = vec![Point3::default(); nv];
        let mut out_t = vec![[0u32; 3]; nt];

        let report = cg::decimate_qem(
            &vertices,
            &triangles,
            DecimateOptions {
                target_faces,
                max_error: Some(max_error),
            },
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Simplify decimation failed: {e:?}"))?;

        out_v.truncate(report.vertices);
        out_t.truncate(report.faces);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_simplify_remesh(&mut self, a: &Args) -> Result<Value, String> {
        let (vertices, triangles) = self.simplify_mesh_buffers(a)?;
        let nv = vertices.len();
        let nt = triangles.len();
        if nt < 1 || nv < 3 {
            return Err("Mesh has too few triangles or vertices for remesh".into());
        }
        let iterations = count(a, "iterations", 3, 8)? as u32;
        let target_edge_length = if a.get("target_edge_length").is_some() {
            let l = number(a, "target_edge_length", 1.0)?;
            if !l.is_finite() || l <= 0.0 {
                return Err("target_edge_length must be finite and > 0".into());
            }
            l
        } else {
            let mean = mean_triangle_edge_length(&vertices, &triangles);
            let target_triangles = match a.get("target_triangles") {
                Some(v) => integer(v, "target_triangles")?.max(1),
                None => nt.max(1),
            };
            (mean * (nt as f64 / target_triangles as f64).sqrt()).max(1e-6)
        };
        let (cap_v, cap_t) = cg::required_output_capacity(nv, nt, iterations);
        let cap_v = cap_v.min(200_000);
        let cap_t = cap_t.min(400_000);
        self.mesh_budget(cap_v, cap_t)?;
        let mut out_v = vec![Point3::default(); cap_v];
        let mut out_t = vec![[0u32; 3]; cap_t];
        let report = cg::isotropic_remesh(
            &vertices,
            &triangles,
            RemeshOptions {
                target_edge_length,
                iterations,
            },
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Simplify remesh failed: {e:?}"))?;
        out_v.truncate(report.vertex_count);
        out_t.truncate(report.triangle_count);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_simplify_anisotropic(&mut self, a: &Args) -> Result<Value, String> {
        let (vertices, triangles) = self.simplify_mesh_buffers(a)?;
        let nv = vertices.len();
        let nt = triangles.len();
        if nt < 1 || nv < 3 {
            return Err("Mesh has too few triangles or vertices for anisotropic remesh".into());
        }
        let iterations = count(a, "iterations", 2, 8)? as u32;
        let field = AnisotropyField::Uniform {
            metric: MetricTensor::IDENTITY,
        };
        let (cap_v, cap_t) = cg::required_anisotropic_output_capacity(nv, nt, iterations);
        let cap_v = cap_v.min(200_000);
        let cap_t = cap_t.min(400_000);
        self.mesh_budget(cap_v, cap_t)?;
        let mut out_v = vec![Point3::default(); cap_v];
        let mut out_t = vec![[0u32; 3]; cap_t];
        let report = cg::anisotropic_remesh(
            &vertices,
            &triangles,
            &field,
            &[],
            &[],
            AnisotropicRemeshOptions {
                iterations,
                ..AnisotropicRemeshOptions::default()
            },
            &mut out_v,
            &mut out_t,
        )
        .map_err(|e| format!("Simplify anisotropic remesh failed: {e:?}"))?;
        out_v.truncate(report.vertex_count);
        out_t.truncate(report.triangle_count);
        self.store_mesh(out_v, out_t)
    }

    fn manifold_import_mesh(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["input", "format"])?;
        let key = text(required(a, "input")?)?;
        let explicit_format = if let Some(f_val) = a.get("format") {
            let f = text(f_val)?;
            match f {
                "obj" | "stl" | "glb" => Some(f.to_string()),
                _ => return Err(format!("Unsupported format: {f}; expected 'obj', 'stl', or 'glb'")),
            }
        } else {
            None
        };

        let (bytes, key_ext) = {
            let res = self
                .inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or_else(|| format!("Missing retained input: {key}"))?;
            let ext = std::path::Path::new(&res.key)
                .extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_ascii_lowercase());
            (res.bytes.clone(), ext)
        };

        let n = bytes.len();
        self.charge(n * 4, n * 8)?;

        let hint = explicit_format.as_deref().or(key_ext.as_deref());
        let imported = qualia_core_db::render::assets::import_asset_with_normals(&bytes, hint)
            .map_err(|e| format!("Import mesh failed: {e}"))?;
        let mesh = imported.mesh;

        self.mesh_budget(mesh.positions.len(), mesh.triangles.len())?;
        self.workspace
            .insert(Artifact::Mesh(mesh), self.limits)
    }

    fn manifold_project_dxf(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "mesh", "layer", "dxf", "input"])?;
        let layer = match a.get("layer") {
            Some(v) => text(v)?,
            None => "0",
        };

        if a.get("mesh").is_none() && a.get("profile").is_none() {
            if let Some(dxf_val) = a.get("dxf").or_else(|| a.get("input")) {
                return self.manifold_parse_dxf(dxf_val);
            }
        }

        let profile_opt = if let Some(pv) = a.get("profile") {
            let Artifact::Profile(pts) = self.workspace.get(pv)? else {
                return Err("Expected profile handle for 'profile'".into());
            };
            Some(pts.clone())
        } else {
            None
        };

        let mesh_opt = if let Some(mv) = a.get("mesh") {
            let Artifact::Mesh(m) = self.workspace.get(mv)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            Some(m.clone())
        } else {
            None
        };

        if profile_opt.is_none() && mesh_opt.is_none() {
            return Err("project_dxf requires at least one of 'profile' or 'mesh'".into());
        }

        let n_profile = profile_opt.as_ref().map(|p| p.len()).unwrap_or(0);
        let n_mesh = mesh_opt.as_ref().map(|m| m.triangles.len()).unwrap_or(0);
        let total_entities = n_profile + n_mesh;
        self.charge(total_entities * 10 + 1024, total_entities * 256 + 4096)?;

        use std::fmt::Write;
        let mut dxf = String::with_capacity(total_entities * 128 + 256);
        dxf.push_str("0\nSECTION\n2\nHEADER\n9\n$ACADVER\n1\nAC1015\n0\nENDSEC\n0\nSECTION\n2\nENTITIES\n");

        if let Some(profile) = profile_opt {
            dxf.push_str("0\nLWPOLYLINE\n8\n");
            dxf.push_str(layer);
            dxf.push_str("\n90\n");
            let _ = write!(&mut dxf, "{}\n", profile.len());
            dxf.push_str("70\n1\n");
            for p in &profile {
                dxf.push_str("10\n");
                let _ = write!(&mut dxf, "{}\n", p[0]);
                dxf.push_str("20\n");
                let _ = write!(&mut dxf, "{}\n", p[1]);
            }
        }

        if let Some(mesh) = mesh_opt {
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

                dxf.push_str("0\n3DFACE\n8\n");
                dxf.push_str(layer);
                dxf.push('\n');

                dxf.push_str("10\n");
                let _ = write!(&mut dxf, "{}\n", p0[0]);
                dxf.push_str("20\n");
                let _ = write!(&mut dxf, "{}\n", p0[1]);
                dxf.push_str("30\n");
                let _ = write!(&mut dxf, "{}\n", p0[2]);

                dxf.push_str("11\n");
                let _ = write!(&mut dxf, "{}\n", p1[0]);
                dxf.push_str("21\n");
                let _ = write!(&mut dxf, "{}\n", p1[1]);
                dxf.push_str("31\n");
                let _ = write!(&mut dxf, "{}\n", p1[2]);

                dxf.push_str("12\n");
                let _ = write!(&mut dxf, "{}\n", p2[0]);
                dxf.push_str("22\n");
                let _ = write!(&mut dxf, "{}\n", p2[1]);
                dxf.push_str("32\n");
                let _ = write!(&mut dxf, "{}\n", p2[2]);

                // AutoCAD 3DFACE repeats vertex 3 as vertex 4 for triangles
                dxf.push_str("13\n");
                let _ = write!(&mut dxf, "{}\n", p2[0]);
                dxf.push_str("23\n");
                let _ = write!(&mut dxf, "{}\n", p2[1]);
                dxf.push_str("33\n");
                let _ = write!(&mut dxf, "{}\n", p2[2]);
                dxf.push('\n');
            }
        }

        dxf.push_str("0\nENDSEC\n0\nEOF\n");
        self.encode("dxf", dxf.into_bytes())
    }

    fn manifold_parse_dxf(&mut self, dxf_val: &Value) -> Result<Value, String> {
        let resource = self.workspace.encoded(dxf_val)?;
        if resource.kind != "dxf" {
            return Err("Expected encoded 'dxf' resource handle".into());
        }
        let parsed = parse_ascii_dxf(&resource.bytes)?;
        self.charge(
            parsed.positions.len() * 8 + parsed.profile.len() * 4 + 256,
            parsed.positions.len() * 48 + parsed.triangles.len() * 12 + 1024,
        )?;

        let mut out = BTreeMap::new();
        out.insert(
            "lwpolyline_count".into(),
            Value::U64(parsed.lwpolyline_count as u64),
        );
        out.insert("face_count".into(), Value::U64(parsed.triangles.len() as u64));
        if !parsed.profile.is_empty() {
            out.insert("profile".into(), self.store_profile(parsed.profile)?);
        }
        if !parsed.positions.is_empty() && !parsed.triangles.is_empty() {
            let vertices: Vec<Point3> = parsed
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            out.insert("mesh".into(), self.store_mesh(vertices, parsed.triangles)?);
        }
        Ok(Value::Record(out))
    }

    fn manifold_loft(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["a", "b", "sections", "caps", "interpolation", "samples"])?;
        let interpolation = match a.get("interpolation") {
            Some(v) => text(v)?,
            None => "linear",
        };
        if interpolation != "linear" && interpolation != "cubic" && interpolation != "nurbs" {
            return Err(
                "Unknown interpolation: expected 'linear' (ruled), 'cubic' (Catmull-Rom), or 'nurbs'"
                    .into(),
            );
        }
        let samples = count(a, "samples", 4, 16)?;
        let caps = boolean(a, "caps", true)?;

        let mut rings = if let Some(sections_val) = a.get("sections") {
            let ring_vals = list(sections_val)?;
            if ring_vals.len() < 2 || ring_vals.len() > 256 {
                return Err("Multi-section loft requires 2..=256 sections".into());
            }
            let mut rings = Vec::with_capacity(ring_vals.len());
            let mut ring_len = 0usize;
            for (idx, rv) in ring_vals.iter().enumerate() {
                let pts = loft_points3(rv)?;
                if idx == 0 {
                    ring_len = pts.len();
                } else if pts.len() != ring_len {
                    return Err("All loft sections must have identical point counts".into());
                }
                rings.push(pts);
            }
            rings
        } else {
            let first = loft_points3(required(a, "a")?)?;
            let second = loft_points3(required(a, "b")?)?;
            if first.len() != second.len() {
                return Err("Loft curves need corresponding point counts".into());
            }
            vec![first, second]
        };

        if interpolation == "cubic" && rings.len() >= 3 {
            rings = densify_loft_rings_cubic(&rings, samples);
        } else if interpolation == "nurbs" {
            rings = densify_loft_rings_nurbs(&rings, samples)?;
        }

        let ring_len = rings[0].len();
        let n_sec = rings.len();
        let nv = n_sec * ring_len + if caps { 2 } else { 0 };
        let nt = (n_sec - 1) * ring_len * 2 + if caps { 2 * ring_len } else { 0 };
        self.mesh_budget(nv, nt)?;

        let mut vertices = Vec::with_capacity(nv);
        let mut triangles = Vec::with_capacity(nt);
        for ring in &rings {
            vertices.extend_from_slice(ring);
        }
        for s in 0..(n_sec - 1) {
            let base_a = (s * ring_len) as u32;
            let base_b = ((s + 1) * ring_len) as u32;
            for i in 0..ring_len {
                let next_i = (i + 1) % ring_len;
                let i00 = base_a + i as u32;
                let i01 = base_a + next_i as u32;
                let i10 = base_b + i as u32;
                let i11 = base_b + next_i as u32;
                triangles.push([i00, i10, i11]);
                triangles.push([i00, i11, i01]);
            }
        }
        if caps {
            for (ring_idx, flip) in [(0usize, true), (n_sec - 1, false)] {
                let ring = &rings[ring_idx];
                let mut cx = 0.0;
                let mut cy = 0.0;
                let mut cz = 0.0;
                for p in ring {
                    cx += p.x;
                    cy += p.y;
                    cz += p.z;
                }
                let inv = 1.0 / (ring_len as f64);
                let c_idx = vertices.len() as u32;
                vertices.push(Point3::new(cx * inv, cy * inv, cz * inv));
                let base = (ring_idx * ring_len) as u32;
                for i in 0..ring_len {
                    let next_i = (i + 1) % ring_len;
                    if flip {
                        triangles.push([c_idx, base + next_i as u32, base + i as u32]);
                    } else {
                        triangles.push([c_idx, base + i as u32, base + next_i as u32]);
                    }
                }
            }
        }
        self.store_mesh(vertices, triangles)
    }

    fn manifold_compile_10d_pbr(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "albedo",
                "source_10d",
                "topology",
                "spatial_index",
                "camera_position",
                "camera_target",
                "camera_up",
                "camera_fov",
                "camera_near",
            ],
        )?;
        let topology = boolean(a, "topology", true)?;
        let spatial_index = boolean(a, "spatial_index", true)?;

        let mesh = {
            let Artifact::Mesh(m) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle for 'mesh'".into());
            };
            m.clone()
        };

        let albedo = if let Some(av) = a.get("albedo") {
            let Artifact::Rgba(rgba) = self.workspace.get(av)? else {
                return Err("Expected rgba handle for 'albedo'".into());
            };
            Some(rgba.clone())
        } else {
            None
        };

        let source_10d_bytes = if let Some(sv) = a.get("source_10d") {
            Some(self.workspace.encoded(sv)?.bytes.clone())
        } else {
            None
        };
        let source_10d_colors = source_10d_bytes
            .as_deref()
            .and_then(decode_source_10d_vertex_colors);

        let nv = mesh.positions.len();
        let mut atlas_uvs = source_10d_bytes
            .as_deref()
            .and_then(|b| read_uv01_atlas(b, nv));
        // Auto-unwrap only when the author did not pass source_10d. A present
        // source_10d UV01 count must match the mesh (fail-closed in consume).
        if atlas_uvs.is_none()
            && a.get("source_10d").is_none()
            && albedo.is_some()
            && nv >= 3
            && !mesh.triangles.is_empty()
        {
            let mut unwrap_args = BTreeMap::new();
            unwrap_args.insert("mesh".into(), required(a, "mesh")?.clone());
            unwrap_args.insert("iterations".into(), Value::U64(8));
            if let Ok(Value::Record(rec)) =
                self.surface_bsdf_uv_op("hypermedia.surface.unwrap_slim", &unwrap_args)
            {
                if let Some(uv_val) = rec.get("uvs") {
                    if let Ok(parsed) = crate::surface_bsdf_uv::parse_vertex_uvs(self, uv_val, nv) {
                        atlas_uvs = Some(
                            parsed
                                .into_iter()
                                .map(|uv| [uv[0] as f32, uv[1] as f32])
                                .collect(),
                        );
                    }
                }
                if atlas_uvs.is_none() {
                    if let Some(c10d) = rec.get("container_10d") {
                        if let Ok(res) = self.workspace.encoded(c10d) {
                            atlas_uvs = read_uv01_atlas(&res.bytes, nv);
                        }
                    }
                }
            }
        }

        let n = mesh.positions.len() + mesh.triangles.len();
        self.charge(n * 50 + 2048, n * 256 + 16384)?;

        // Compute smooth area-weighted per-vertex unit normals of length mesh.vertex_count()
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
            let e1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let cross = [
                e1[1] * e2[2] - e1[2] * e2[1],
                e1[2] * e2[0] - e1[0] * e2[2],
                e1[0] * e2[1] - e1[1] * e2[0],
            ];
            for &idx in &[i0, i1, i2] {
                normals[idx][0] += cross[0];
                normals[idx][1] += cross[1];
                normals[idx][2] += cross[2];
            }
        }
        for n in &mut normals {
            let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
            if len_sq > 1e-12 {
                let inv = 1.0 / len_sq.sqrt();
                n[0] *= inv;
                n[1] *= inv;
                n[2] *= inv;
            } else {
                *n = [0.0, 1.0, 0.0];
            }
        }

        // Sample albedo: SLIM UV01 atlas, then camera look-at projection, then
        // spherical only when no atlas exists.
        let surface_reading: Option<Vec<[f32; 4]>> = if let Some(rgba) = albedo {
            if rgba.width == 0 || rgba.height == 0 || rgba.pixels.is_empty() {
                return Err("Albedo image has empty dimensions".into());
            }
            let centroid = mesh.centroid();
            let camera_eye = match a.get("camera_position") {
                Some(v) => Some(vector::<3>(v)?),
                None => None,
            };
            let camera_target = match a.get("camera_target") {
                Some(v) => vector::<3>(v)?,
                None => [centroid[0] as f64, centroid[1] as f64, centroid[2] as f64],
            };
            let camera_up = match a.get("camera_up") {
                Some(v) => vector::<3>(v)?,
                None => [0.0, 1.0, 0.0],
            };
            let camera_fov = number(a, "camera_fov", 45.0)?;
            let camera_near = number(a, "camera_near", 0.1)?;
            let mut readings = Vec::with_capacity(mesh.positions.len());
            let pi = std::f64::consts::PI;
            let two_pi = 2.0 * pi;
            let max_x = (rgba.width - 1) as f64;
            let max_y = (rgba.height - 1) as f64;
            let vw = rgba.width as f64;
            let vh = rgba.height as f64;

            let sample_uv = |u: f64, v: f64| -> [f32; 4] {
                let px = ((u * max_x).round() as usize).min(rgba.width as usize - 1);
                let py = ((v * max_y).round() as usize).min(rgba.height as usize - 1);
                let idx = py * rgba.width as usize + px;
                let pix = rgba.pixels.get(idx).copied().unwrap_or([255, 255, 255, 255]);
                [
                    pix[0] as f32 / 255.0,
                    pix[1] as f32 / 255.0,
                    pix[2] as f32 / 255.0,
                    pix[3] as f32 / 255.0,
                ]
            };

            for (i, p) in mesh.positions.iter().enumerate() {
                let dx = (p[0] - centroid[0]) as f64;
                let dy = (p[1] - centroid[1]) as f64;
                let dz = (p[2] - centroid[2]) as f64;
                let r = (dx * dx + dy * dy + dz * dz).sqrt();
                let atlas = atlas_uvs.as_ref().and_then(|uvs| {
                    uvs.get(i).map(|uv| {
                        sample_uv(
                            (uv[0] as f64).clamp(0.0, 1.0),
                            (uv[1] as f64).clamp(0.0, 1.0),
                        )
                    })
                });
                // Camera look-at means albedo is a photograph: do not resample it
                // through auto-unwrap UVs or spherical smear. Authored source_10d
                // UVs remain a valid fallback. Spherical only when no camera and
                // no atlas.
                let use_atlas_albedo =
                    atlas.is_some() && (camera_eye.is_none() || source_10d_bytes.is_some());
                let spherical = if camera_eye.is_some() || atlas.is_some() {
                    None
                } else if r < 1e-9 {
                    Some(sample_uv(0.5, 0.5))
                } else {
                    let phi = dz.atan2(dx);
                    let u = (phi / two_pi + 0.5).clamp(0.0, 1.0);
                    let lat = (dy / r).clamp(-1.0, 1.0).asin();
                    let v = (lat / pi + 0.5).clamp(0.0, 1.0);
                    Some(sample_uv(u, v))
                };
                let atlas = if use_atlas_albedo { atlas } else { None };

                let projected = if let Some(eye) = camera_eye {
                    let world = [p[0] as f64, p[1] as f64, p[2] as f64];
                    let n = normals[i];
                    let view = [eye[0] - world[0], eye[1] - world[1], eye[2] - world[2]];
                    let facing = n[0] as f64 * view[0] + n[1] as f64 * view[1] + n[2] as f64 * view[2];
                    if facing > 0.0 {
                        Camera3D::unproject_position_look_at(
                            eye,
                            camera_target,
                            camera_up,
                            camera_fov,
                            world,
                            (vw, vh),
                            camera_near,
                        )
                        .and_then(|px| {
                            if px[0] < 0.0 || px[1] < 0.0 || px[0] >= vw || px[1] >= vh {
                                return None;
                            }
                            let color = sample_uv(px[0] / vw, px[1] / vh);
                            // Key near-black studio backdrop so it does not stain the mesh.
                            if color[0].max(color[1]).max(color[2]) < 0.05 {
                                None
                            } else {
                                Some(color)
                            }
                        })
                    } else {
                        None
                    }
                } else {
                    None
                };

                readings.push(
                    projected
                        .or(atlas)
                        .or(spherical)
                        .unwrap_or([0.5, 0.5, 0.5, 1.0]),
                );
            }
            Some(readings)
        } else if let Some((ref_pos, ref_cols)) = source_10d_colors {
            Some(transfer_nearest_vertex_colors(
                &mesh.positions,
                &ref_pos,
                &ref_cols,
            ))
        } else {
            None
        };

        let bytes = match compile_mesh_to_10d_with_extras_and_frames(
            &mesh,
            &[],
            None,
            Compile10dExtras {
                topology,
                spatial_index,
            },
            surface_reading.as_deref(),
            Some(&normals),
            None,
        ) {
            Ok(b) => b,
            Err(_) if topology => compile_mesh_to_10d_with_extras_and_frames(
                &mesh,
                &[],
                None,
                Compile10dExtras {
                    topology: false,
                    spatial_index,
                },
                surface_reading.as_deref(),
                Some(&normals),
                None,
            )
            .map_err(|e| format!("10D PBR compile failed: {e:?}"))?,
            Err(e) => return Err(format!("10D PBR compile failed: {e:?}")),
        };

        let sealed = if let Some(uvs) = atlas_uvs.as_ref() {
            qualia_core_db::container_10d::attach_texture_coordinate_section(&bytes, uvs)
                .map_err(|e| format!("UV01 attach failed: {e:?}"))?
        } else {
            bytes
        };
        self.encode("10d", sealed)
    }
}

fn read_uv01_atlas(bytes: &[u8], nv: usize) -> Option<Vec<[f32; 2]>> {
    if nv == 0 {
        return None;
    }
    let mut uvs = vec![[0.0f32; 2]; nv];
    let n = qualia_core_db::container_10d::read_texture_coordinate_section_into(bytes, nv, &mut uvs)
        .ok()?;
    if n == nv {
        Some(uvs)
    } else {
        None
    }
}

fn decode_source_10d_vertex_colors(bytes: &[u8]) -> Option<(Vec<[f32; 3]>, Vec<[f32; 4]>)> {
    let header = Container10dHeader::parse(bytes).ok()?;
    let descs = parse_section_table(bytes, &header).ok()?;
    let ref_mesh = decode_10d_mesh(bytes).ok()?;
    for d in descs {
        if d.typ() == Some(SectionType::SurfaceReading) {
            let start = d.byte_offset as usize;
            let end = start.saturating_add(d.byte_length as usize);
            let slice = bytes.get(start..end)?;
            let cols = decode_surface_reading(slice)?;
            if cols.len() == ref_mesh.positions.len() && !cols.is_empty() {
                return Some((ref_mesh.positions, cols));
            }
        }
    }
    None
}

fn transfer_nearest_vertex_colors(
    target_positions: &[[f32; 3]],
    ref_positions: &[[f32; 3]],
    ref_colors: &[[f32; 4]],
) -> Vec<[f32; 4]> {
    let mut out = Vec::with_capacity(target_positions.len());
    for p in target_positions {
        let mut best_d2 = f32::INFINITY;
        let mut best_col = [0.65, 0.72, 0.68, 1.0];
        for (rp, rc) in ref_positions.iter().zip(ref_colors.iter()) {
            let dx = p[0] - rp[0];
            let dy = p[1] - rp[1];
            let dz = p[2] - rp[2];
            let d2 = dx * dx + dy * dy + dz * dz;
            if d2 < best_d2 {
                best_d2 = d2;
                best_col = *rc;
            }
        }
        out.push(best_col);
    }
    out
}

fn loft_points3(v: &Value) -> Result<Vec<Point3>, String> {
    let values = list(v)?;
    if values.len() < 3 || values.len() > 4096 {
        return Err("Loft section point count outside 3..=4096".into());
    }
    values
        .iter()
        .map(|item| {
            let p = vector::<3>(item)?;
            if p.iter().any(|n| !n.is_finite() || n.abs() > 1e6) {
                return Err("Coordinate outside +/-1000000 range".into());
            }
            Ok(Point3::new(p[0], p[1], p[2]))
        })
        .collect()
}

fn catmull_rom_scalar(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

fn catmull_rom_point3(p0: Point3, p1: Point3, p2: Point3, p3: Point3, t: f64) -> Point3 {
    Point3::new(
        catmull_rom_scalar(p0.x, p1.x, p2.x, p3.x, t),
        catmull_rom_scalar(p0.y, p1.y, p2.y, p3.y, t),
        catmull_rom_scalar(p0.z, p1.z, p2.z, p3.z, t),
    )
}

fn mean_triangle_edge_length(vertices: &[Point3], triangles: &[[u32; 3]]) -> f64 {
    let mut sum = 0.0;
    let mut n = 0.0;
    for tri in triangles {
        for k in 0..3 {
            let a = vertices[tri[k] as usize];
            let b = vertices[tri[(k + 1) % 3] as usize];
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            let dz = a.z - b.z;
            sum += (dx * dx + dy * dy + dz * dz).sqrt();
            n += 1.0;
        }
    }
    if n > 0.0 {
        sum / n
    } else {
        1.0
    }
}

/// Cubic Catmull-Rom densify of loft stations. Linear/ruled skinning is applied after.
fn densify_loft_rings_cubic(rings: &[Vec<Point3>], samples: usize) -> Vec<Vec<Point3>> {
    let n = rings.len();
    let ring_len = rings[0].len();
    let steps = samples.max(1);
    let mut out = Vec::with_capacity((n - 1) * steps + 1);
    out.push(rings[0].clone());
    for seg in 0..(n - 1) {
        let i0 = if seg == 0 { 0 } else { seg - 1 };
        let i1 = seg;
        let i2 = seg + 1;
        let i3 = (seg + 2).min(n - 1);
        for s in 1..=steps {
            if s == steps {
                out.push(rings[i2].clone());
                continue;
            }
            let t = s as f64 / steps as f64;
            let mut ring = Vec::with_capacity(ring_len);
            for k in 0..ring_len {
                ring.push(catmull_rom_point3(
                    rings[i0][k],
                    rings[i1][k],
                    rings[i2][k],
                    rings[i3][k],
                    t,
                ));
            }
            out.push(ring);
        }
    }
    out
}

/// NURBS densify of loft stations via `parametric_cad::nurbs_eval` (unit weights).
fn densify_loft_rings_nurbs(
    rings: &[Vec<Point3>],
    samples: usize,
) -> Result<Vec<Vec<Point3>>, String> {
    let n = rings.len();
    if n < 2 {
        return Err("NURBS loft requires at least 2 sections".into());
    }
    let ring_len = rings[0].len();
    let degree = (n - 1).min(3).max(1);
    if degree >= n {
        return Ok(rings.to_vec());
    }
    let steps = samples.max(1);
    let n_samples = (n - 1) * steps + 1;
    let denom = (n_samples - 1).max(1) as f64;
    let mut out = Vec::with_capacity(n_samples);
    for s in 0..n_samples {
        let t = s as f64 / denom;
        let mut ring = Vec::with_capacity(ring_len);
        for k in 0..ring_len {
            let control: Vec<cg::NurbsControlPoint> = rings
                .iter()
                .map(|r| cg::NurbsControlPoint {
                    point: r[k],
                    weight: 1.0,
                })
                .collect();
            let p = cg::nurbs_eval(&control, degree, t)
                .map_err(|e| format!("NURBS loft eval: {e:?}"))?;
            ring.push(p);
        }
        out.push(ring);
    }
    Ok(out)
}

struct ParsedDxf {
    profile: Vec<[f64; 2]>,
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
    lwpolyline_count: usize,
}

fn parse_ascii_dxf(bytes: &[u8]) -> Result<ParsedDxf, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "DXF is not valid UTF-8")?;
    let lines: Vec<&str> = text
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if lines.len() < 4 {
        return Err("DXF payload too short".into());
    }
    let mut weld: BTreeMap<(i64, i64, i64), u32> = BTreeMap::new();
    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut triangles: Vec<[u32; 3]> = Vec::new();
    let mut profile: Vec<[f64; 2]> = Vec::new();
    let mut lwpolyline_count = 0usize;

    let weld_vertex = |weld: &mut BTreeMap<(i64, i64, i64), u32>,
                       positions: &mut Vec<[f32; 3]>,
                       p: [f32; 3]|
     -> u32 {
        let key = (
            (p[0] as f64 * 1e5).round() as i64,
            (p[1] as f64 * 1e5).round() as i64,
            (p[2] as f64 * 1e5).round() as i64,
        );
        if let Some(&idx) = weld.get(&key) {
            idx
        } else {
            let idx = positions.len() as u32;
            weld.insert(key, idx);
            positions.push(p);
            idx
        }
    };

    let mut i = 0usize;
    let mut entity = "";
    let mut face = [[0.0f32; 3]; 4];
    let mut poly: Vec<[f64; 2]> = Vec::new();
    let mut x = 0.0f64;
    let mut y = 0.0f64;
    let mut z = 0.0f64;
    while i + 1 < lines.len() {
        let code: i32 = lines[i].parse().unwrap_or(i32::MIN);
        let val = lines[i + 1];
        i += 2;
        if code == i32::MIN {
            continue;
        }
        match code {
            0 => {
                if entity == "3DFACE" {
                    let i0 = weld_vertex(&mut weld, &mut positions, face[0]);
                    let i1 = weld_vertex(&mut weld, &mut positions, face[1]);
                    let i2 = weld_vertex(&mut weld, &mut positions, face[2]);
                    let i3 = weld_vertex(&mut weld, &mut positions, face[3]);
                    if i0 != i1 && i1 != i2 && i2 != i0 {
                        triangles.push([i0, i1, i2]);
                    }
                    if i3 != i2 && i3 != i0 && i0 != i2 {
                        triangles.push([i0, i2, i3]);
                    }
                    face = [[0.0; 3]; 4];
                }
                if entity == "LWPOLYLINE" && !poly.is_empty() {
                    if profile.is_empty() {
                        profile = poly.clone();
                    }
                    lwpolyline_count += 1;
                    poly.clear();
                }
                entity = val;
            }
            10 => x = val.parse().unwrap_or(0.0),
            20 => {
                y = val.parse().unwrap_or(0.0);
                if entity == "LWPOLYLINE" {
                    poly.push([x, y]);
                }
            }
            30 => z = val.parse().unwrap_or(0.0),
            11 | 12 | 13 | 21 | 22 | 23 | 31 | 32 | 33 => {
                let n = val.parse().unwrap_or(0.0) as f32;
                match code {
                    11 => face[1][0] = n,
                    21 => face[1][1] = n,
                    31 => face[1][2] = n,
                    12 => face[2][0] = n,
                    22 => face[2][1] = n,
                    32 => face[2][2] = n,
                    13 => face[3][0] = n,
                    23 => face[3][1] = n,
                    33 => face[3][2] = n,
                    _ => {}
                }
            }
            _ => {}
        }
        if entity == "3DFACE" {
            match code {
                10 => face[0][0] = x as f32,
                20 => face[0][1] = y as f32,
                30 => face[0][2] = z as f32,
                _ => {}
            }
        }
    }
    if entity == "3DFACE" {
        let i0 = weld_vertex(&mut weld, &mut positions, face[0]);
        let i1 = weld_vertex(&mut weld, &mut positions, face[1]);
        let i2 = weld_vertex(&mut weld, &mut positions, face[2]);
        let i3 = weld_vertex(&mut weld, &mut positions, face[3]);
        if i0 != i1 && i1 != i2 && i2 != i0 {
            triangles.push([i0, i1, i2]);
        }
        if i3 != i2 && i3 != i0 && i0 != i2 {
            triangles.push([i0, i2, i3]);
        }
    }
    if entity == "LWPOLYLINE" && !poly.is_empty() {
        if profile.is_empty() {
            profile = poly;
        }
        lwpolyline_count += 1;
    }
    if positions.is_empty() && profile.is_empty() {
        return Err("DXF contained no LWPOLYLINE or 3DFACE entities".into());
    }
    Ok(ParsedDxf {
        profile,
        positions,
        triangles,
        lwpolyline_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn vec3(x: f64, y: f64, z: f64) -> Value {
        Value::List(vec![Value::F64(x), Value::F64(y), Value::F64(z)])
    }

    fn unit_cube_mesh(host: &mut AssetHost<'_>) -> Value {
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
        let vertices: Vec<Point3> = positions
            .iter()
            .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        host.store_mesh(vertices, triangles).expect("store cube")
    }

    fn translated_cube(host: &mut AssetHost<'_>, dx: f64, dy: f64, dz: f64) -> Value {
        let positions = vec![
            [-1.0 + dx, -1.0 + dy, -1.0 + dz],
            [1.0 + dx, -1.0 + dy, -1.0 + dz],
            [1.0 + dx, 1.0 + dy, -1.0 + dz],
            [-1.0 + dx, 1.0 + dy, -1.0 + dz],
            [-1.0 + dx, -1.0 + dy, 1.0 + dz],
            [1.0 + dx, -1.0 + dy, 1.0 + dz],
            [1.0 + dx, 1.0 + dy, 1.0 + dz],
            [-1.0 + dx, 1.0 + dy, 1.0 + dz],
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
        let vertices: Vec<Point3> = positions
            .iter()
            .map(|p| Point3::new(p[0], p[1], p[2]))
            .collect();
        host.store_mesh(vertices, triangles).expect("store translated cube")
    }

    fn square_ring(z: f64, s: f64) -> Value {
        Value::List(vec![
            vec3(-s, -s, z),
            vec3(s, -s, z),
            vec3(s, s, z),
            vec3(-s, s, z),
        ])
    }

    #[test]
    fn test_manifold_loft_linear_remains_ruled_cubic_bends() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let mut linear_args = BTreeMap::new();
        linear_args.insert(
            "sections".into(),
            Value::List(vec![
                square_ring(0.0, 1.0),
                square_ring(1.0, 2.0),
                square_ring(2.0, 1.0),
            ]),
        );
        linear_args.insert("caps".into(), Value::Bool(false));
        linear_args.insert("interpolation".into(), Value::String("linear".into()));

        let linear_h = host
            .manifold_ext("hypermedia.manifold.loft", &linear_args)
            .expect("linear loft");
        let (linear_n, linear_t, mid) = {
            let Artifact::Mesh(linear) = host.workspace.get(&linear_h).expect("linear mesh") else {
                panic!("expected mesh");
            };
            // 3 rings * 4 verts, 2 spans * 4 quads * 2 tris
            (linear.positions.len(), linear.triangles.len(), linear.positions[4])
        };
        assert_eq!(linear_n, 12);
        assert_eq!(linear_t, 16);
        // Ruled: first vertex of the middle ring is the middle station (-2, -2, 1).
        assert!((mid[0] + 2.0).abs() < 1e-5 && (mid[2] - 1.0).abs() < 1e-5);

        let origin_linear = host
            .invoke(
                "hypermedia.manifold.loft",
                &Value::Record({
                    let mut a = BTreeMap::new();
                    a.insert("a".into(), square_ring(0.0, 1.0));
                    a.insert("b".into(), square_ring(1.0, 1.0));
                    a
                }),
            )
            .expect("origin loft");
        {
            let Artifact::Mesh(origin) = host.workspace.get(&origin_linear).expect("origin mesh")
            else {
                panic!("expected mesh");
            };
            assert!(origin.triangles.len() >= 4);
        }

        let mut cubic_args = linear_args.clone();
        cubic_args.insert("interpolation".into(), Value::String("cubic".into()));
        cubic_args.insert("samples".into(), Value::U64(4));
        let cubic_h = host
            .manifold_ext("hypermedia.manifold.loft", &cubic_args)
            .expect("cubic loft");
        let Artifact::Mesh(cubic) = host.workspace.get(&cubic_h).expect("cubic mesh") else {
            panic!("expected mesh");
        };
        assert!(cubic.positions.len() > linear_n);
        // First densified ring after station 0 is Catmull-Rom, not the linear chord of size 1.25.
        let densified = cubic.positions[4];
        let linear_chord = 1.0 + 0.25 * (2.0 - 1.0);
        assert!(
            (densified[0].abs() - linear_chord as f32).abs() > 1e-4,
            "cubic loft must leave the ruled chord, got {}",
            densified[0]
        );
        for p in &cubic.positions {
            assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
        }
    }

    #[test]
    fn test_manifold_boolean_3d_production_union_difference() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let a = unit_cube_mesh(&mut host);
        let b = translated_cube(&mut host, 0.6, 0.4, 0.5);

        let mut union_args = BTreeMap::new();
        union_args.insert("a".into(), a.clone());
        union_args.insert("b".into(), b.clone());
        union_args.insert("op".into(), Value::String("union".into()));
        let union_h = host
            .invoke("hypermedia.manifold.boolean_3d", &Value::Record(union_args))
            .expect("boolean union");
        {
            let Artifact::Mesh(union_mesh) = host.workspace.get(&union_h).expect("union mesh")
            else {
                panic!("expected mesh");
            };
            assert!(
                union_mesh.triangles.len() >= 12,
                "union must keep a solid, got {} tris",
                union_mesh.triangles.len()
            );
            for p in &union_mesh.positions {
                assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
            }
        }

        let mut diff_args = BTreeMap::new();
        diff_args.insert("a".into(), a);
        diff_args.insert("b".into(), b);
        diff_args.insert("op".into(), Value::String("difference".into()));
        let diff_h = host
            .invoke(
                "hypermedia.manifold.boolean_3d",
                &Value::Record(diff_args),
            )
            .expect("boolean difference");
        let Artifact::Mesh(diff_mesh) = host.workspace.get(&diff_h).expect("diff mesh") else {
            panic!("expected mesh");
        };
        assert!(!diff_mesh.triangles.is_empty());
        assert!(!diff_mesh.positions.is_empty());
    }

    #[test]
    fn test_manifold_project_dxf_mesh_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);

        let cube = unit_cube_mesh(&mut host);
        let mut export_args = BTreeMap::new();
        export_args.insert("mesh".into(), cube);
        export_args.insert("layer".into(), Value::String("SOLID".into()));
        let dxf_h = host
            .invoke(
                "hypermedia.manifold.project_dxf",
                &Value::Record(export_args),
            )
            .expect("dxf export");
        {
            let encoded = host.workspace.encoded(&dxf_h).expect("dxf bytes");
            assert_eq!(encoded.kind, "dxf");
            let text = std::str::from_utf8(&encoded.bytes).unwrap();
            assert!(text.contains("3DFACE"));
            assert!(text.contains("AC1015"));
        }

        let mut import_args = BTreeMap::new();
        import_args.insert("dxf".into(), dxf_h);
        let parsed = host
            .manifold_ext("hypermedia.manifold.project_dxf", &import_args)
            .expect("dxf import");
        let rec = record(&parsed).expect("record");
        let face_count = integer(required(rec, "face_count").unwrap(), "face_count").unwrap();
        assert_eq!(face_count, 12);
        let mesh_h = rec.get("mesh").expect("mesh handle");
        let Artifact::Mesh(roundtrip) = host.workspace.get(mesh_h).expect("rt mesh") else {
            panic!("expected mesh");
        };
        assert_eq!(roundtrip.triangles.len(), 12);
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in &roundtrip.positions {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        assert!((min[0] + 1.0).abs() < 1e-3 && (max[0] - 1.0).abs() < 1e-3);
        assert!((min[2] + 1.0).abs() < 1e-3 && (max[2] - 1.0).abs() < 1e-3);
    }

    fn unit_icosphere_mesh(host: &mut AssetHost<'_>) -> Value {
        let positions = vec![
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        let triangles = vec![
            [0, 2, 4],
            [2, 1, 4],
            [1, 3, 4],
            [3, 0, 4],
            [0, 5, 2],
            [2, 5, 1],
            [1, 5, 3],
            [3, 5, 0],
        ];
        let vertices: Vec<Point3> = positions
            .iter()
            .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
            .collect();
        host.store_mesh(vertices, triangles).expect("store icosphere")
    }

    fn mesh_golden_checksum(positions: &[[f32; 3]], triangles: &[[u32; 3]]) -> String {
        let mut quantized: Vec<(i64, i64, i64)> = positions
            .iter()
            .map(|p| {
                (
                    (p[0] as f64 * 1e5).round() as i64,
                    (p[1] as f64 * 1e5).round() as i64,
                    (p[2] as f64 * 1e5).round() as i64,
                )
            })
            .collect();
        quantized.sort_unstable();
        let mut hasher = blake3::Hasher::new();
        hasher.update(&(positions.len() as u64).to_le_bytes());
        hasher.update(&(triangles.len() as u64).to_le_bytes());
        for (x, y, z) in &quantized {
            hasher.update(&x.to_le_bytes());
            hasher.update(&y.to_le_bytes());
            hasher.update(&z.to_le_bytes());
        }
        hasher.finalize().to_hex().to_string()
    }

    #[test]
    fn test_manifold_simplify_decimate_default_qem() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);
        let cube = unit_cube_mesh(&mut host);
        let mut args = BTreeMap::new();
        args.insert("mesh".into(), cube);
        args.insert("target_triangles".into(), Value::U64(8));
        args.insert("max_error".into(), Value::F64(1.0));
        let decimated = host
            .manifold_ext("hypermedia.manifold.simplify", &args)
            .expect("default simplify is QEM decimate");
        let (decimated_t, decimated_v) = {
            let Artifact::Mesh(mesh) = host.workspace.get(&decimated).expect("decimated mesh")
            else {
                panic!("expected mesh");
            };
            assert!(mesh.triangles.len() <= 12);
            assert!(!mesh.triangles.is_empty());
            assert!(!mesh.positions.is_empty());
            (mesh.triangles.len(), mesh.positions.len())
        };

        args.insert("mode".into(), Value::String("decimate".into()));
        let explicit = host
            .manifold_ext("hypermedia.manifold.simplify", &args)
            .expect("mode=decimate");
        let Artifact::Mesh(explicit_mesh) = host.workspace.get(&explicit).expect("explicit mesh")
        else {
            panic!("expected mesh");
        };
        assert_eq!(explicit_mesh.triangles.len(), decimated_t);
        assert_eq!(explicit_mesh.positions.len(), decimated_v);
    }

    #[test]
    fn test_manifold_simplify_remesh_and_anisotropic_small_mesh() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);
        let cube = unit_cube_mesh(&mut host);
        let ico = unit_icosphere_mesh(&mut host);

        let mut remesh_args = BTreeMap::new();
        remesh_args.insert("mesh".into(), cube.clone());
        remesh_args.insert("target_triangles".into(), Value::U64(48));
        remesh_args.insert("mode".into(), Value::String("remesh".into()));
        remesh_args.insert("iterations".into(), Value::U64(2));
        remesh_args.insert("target_edge_length".into(), Value::F64(1.0));
        let remeshed = host
            .manifold_ext("hypermedia.manifold.simplify", &remesh_args)
            .expect("mode=remesh on cube");
        {
            let Artifact::Mesh(mesh) = host.workspace.get(&remeshed).expect("remesh mesh") else {
                panic!("expected mesh");
            };
            assert!(!mesh.triangles.is_empty());
            assert!(!mesh.positions.is_empty());
            for p in &mesh.positions {
                assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
            }
        }

        remesh_args.insert("mesh".into(), ico.clone());
        remesh_args.insert("target_edge_length".into(), Value::F64(0.75));
        let ico_remesh = host
            .manifold_ext("hypermedia.manifold.simplify", &remesh_args)
            .expect("mode=remesh on icosphere");
        {
            let Artifact::Mesh(mesh) = host.workspace.get(&ico_remesh).expect("ico remesh") else {
                panic!("expected mesh");
            };
            assert!(mesh.triangles.len() >= 8);
        }

        let mut aniso_args = BTreeMap::new();
        aniso_args.insert("mesh".into(), cube);
        aniso_args.insert("target_triangles".into(), Value::U64(24));
        aniso_args.insert("mode".into(), Value::String("anisotropic".into()));
        aniso_args.insert("iterations".into(), Value::U64(2));
        let aniso = host
            .manifold_ext("hypermedia.manifold.simplify", &aniso_args)
            .expect("mode=anisotropic on cube");
        let Artifact::Mesh(aniso_mesh) = host.workspace.get(&aniso).expect("aniso mesh") else {
            panic!("expected mesh");
        };
        assert!(!aniso_mesh.triangles.is_empty());
        assert!(!aniso_mesh.positions.is_empty());
    }

    #[test]
    fn test_manifold_boolean_3d_golden_checksum() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);
        let a = unit_cube_mesh(&mut host);
        let b = translated_cube(&mut host, 0.6, 0.4, 0.5);

        let mut union_args = BTreeMap::new();
        union_args.insert("a".into(), a.clone());
        union_args.insert("b".into(), b.clone());
        union_args.insert("op".into(), Value::String("union".into()));
        let union_h = host
            .invoke("hypermedia.manifold.boolean_3d", &Value::Record(union_args))
            .expect("boolean union");
        let union_hash = {
            let Artifact::Mesh(union_mesh) = host.workspace.get(&union_h).expect("union mesh")
            else {
                panic!("expected mesh");
            };
            mesh_golden_checksum(&union_mesh.positions, &union_mesh.triangles)
        };
        assert_eq!(
            union_hash,
            "1588a8ccb80d7ba41a163e13e8b02bf4ff9a9f07aef7bf1b72c2cfa4e6f6ae00",
            "union golden checksum (counts + quantized positions)"
        );

        let mut diff_args = BTreeMap::new();
        diff_args.insert("a".into(), a);
        diff_args.insert("b".into(), b);
        diff_args.insert("op".into(), Value::String("difference".into()));
        let diff_h = host
            .invoke(
                "hypermedia.manifold.boolean_3d",
                &Value::Record(diff_args),
            )
            .expect("boolean difference");
        let Artifact::Mesh(diff_mesh) = host.workspace.get(&diff_h).expect("diff mesh") else {
            panic!("expected mesh");
        };
        let diff_hash = mesh_golden_checksum(&diff_mesh.positions, &diff_mesh.triangles);
        let again = mesh_golden_checksum(&diff_mesh.positions, &diff_mesh.triangles);
        assert_eq!(diff_hash, again, "boolean checksum is stable");
        assert_ne!(union_hash, diff_hash);
    }

    #[test]
    fn test_manifold_loft_nurbs_optional_keeps_linear_cubic() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_test_host(&limits, &cancel, &[]);
        let mut args = BTreeMap::new();
        args.insert(
            "sections".into(),
            Value::List(vec![
                square_ring(0.0, 1.0),
                square_ring(1.0, 2.0),
                square_ring(2.0, 1.0),
            ]),
        );
        args.insert("caps".into(), Value::Bool(false));
        args.insert("interpolation".into(), Value::String("nurbs".into()));
        args.insert("samples".into(), Value::U64(4));
        let nurbs_h = host
            .manifold_ext("hypermedia.manifold.loft", &args)
            .expect("nurbs loft");
        let Artifact::Mesh(nurbs) = host.workspace.get(&nurbs_h).expect("nurbs mesh") else {
            panic!("expected mesh");
        };
        assert!(nurbs.positions.len() > 12);
        for p in &nurbs.positions {
            assert!(p[0].is_finite() && p[1].is_finite() && p[2].is_finite());
        }
    }
}

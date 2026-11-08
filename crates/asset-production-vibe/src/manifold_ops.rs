use super::*;
use qualia_core_db::render::compile_10d::{
    compile_mesh_to_10d_with_extras_and_frames, Compile10dExtras,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    self as cg, boolean_3::Boolean3Op, DecimateOptions, Point3,
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

        let base = (va.len() + vb.len()) * 8 + 1024;
        let max_v = base.min(50_000);
        let max_t = base.min(100_000);
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
        fields(a, &["mesh", "target_triangles", "max_error"])?;
        let target_triangles = integer(required(a, "target_triangles")?, "target_triangles")?;
        if target_triangles < 4 || target_triangles > 200_000 {
            return Err("target_triangles outside 4..=200000 range".into());
        }
        let max_error = number(a, "max_error", 0.05)?;
        if !max_error.is_finite() || max_error < 1e-6 || max_error > 1e4 {
            return Err("max_error outside 1e-6..=1e4 range".into());
        }

        let (vertices, triangles) = {
            let Artifact::Mesh(mesh) = self.workspace.get(required(a, "mesh")?)? else {
                return Err("Expected mesh handle".into());
            };
            let v: Vec<Point3> = mesh
                .positions
                .iter()
                .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                .collect();
            let t = mesh.triangles.clone();
            (v, t)
        };

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
        fields(a, &["profile", "mesh", "layer"])?;
        let layer = match a.get("layer") {
            Some(v) => text(v)?,
            None => "0",
        };

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

    fn manifold_compile_10d_pbr(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "albedo", "topology", "spatial_index"])?;
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

        // Sample albedo pixels at each vertex spherical projection
        let surface_reading: Option<Vec<[f32; 4]>> = if let Some(rgba) = albedo {
            if rgba.width == 0 || rgba.height == 0 || rgba.pixels.is_empty() {
                return Err("Albedo image has empty dimensions".into());
            }
            let centroid = mesh.centroid();
            let mut readings = Vec::with_capacity(mesh.positions.len());
            let pi = std::f64::consts::PI;
            let two_pi = 2.0 * pi;

            for p in &mesh.positions {
                let dx = (p[0] - centroid[0]) as f64;
                let dy = (p[1] - centroid[1]) as f64;
                let dz = (p[2] - centroid[2]) as f64;
                let r = (dx * dx + dy * dy + dz * dz).sqrt();

                let (u, v) = if r < 1e-9 {
                    (0.5, 0.5)
                } else {
                    let phi = dz.atan2(dx);
                    let u = (phi / two_pi + 0.5).clamp(0.0, 1.0);
                    let lat = (dy / r).clamp(-1.0, 1.0).asin();
                    let v = (lat / pi + 0.5).clamp(0.0, 1.0);
                    (u, v)
                };

                let max_x = (rgba.width - 1) as f64;
                let max_y = (rgba.height - 1) as f64;
                let px = ((u * max_x).round() as usize).min(rgba.width as usize - 1);
                let py = ((v * max_y).round() as usize).min(rgba.height as usize - 1);
                let idx = py * rgba.width as usize + px;
                let pix = rgba.pixels.get(idx).copied().unwrap_or([255, 255, 255, 255]);
                readings.push([
                    pix[0] as f32 / 255.0,
                    pix[1] as f32 / 255.0,
                    pix[2] as f32 / 255.0,
                    pix[3] as f32 / 255.0,
                ]);
            }
            Some(readings)
        } else {
            None
        };

        let bytes = compile_mesh_to_10d_with_extras_and_frames(
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
        )
        .map_err(|e| format!("10D PBR compile failed: {e:?}"))?;

        self.encode("10d", bytes)
    }
}

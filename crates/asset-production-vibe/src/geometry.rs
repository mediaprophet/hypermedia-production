use super::*;
use drafting_geometry::{Mat3, Vec2};
use profile_authoring::{Profile, ProfileLimits};
use qualia_core_db::specialized_libs::computational_geometry::{self as cg, Point2, Point3};

impl AssetHost<'_> {
    pub(crate) fn geometry(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.locus.construct" | "hypermedia.profile.create" => {
                let shape = text(required(a, "shape")?)?;
                let segments = count(a, "segments", 32, 256)?;
                let profile = match shape {
                    "circle" => {
                        fields(a, &["shape", "radius", "segments"])?;
                        Profile::Circle {
                            radius: number(a, "radius", 1.)?,
                            segments,
                        }
                    }
                    "rectangle" => {
                        fields(a, &["shape", "width", "height", "radius", "segments"])?;
                        Profile::RoundedRectangle {
                            width: number(a, "width", 1.)?,
                            height: number(a, "height", 1.)?,
                            radius: number(a, "radius", 0.)?,
                            segments,
                        }
                    }
                    "channel" => {
                        fields(
                            a,
                            &["shape", "width", "height", "wall", "radius", "segments"],
                        )?;
                        Profile::OpenChannel {
                            width: number(a, "width", 1.)?,
                            height: number(a, "height", 1.)?,
                            wall: number(a, "wall", 0.1)?,
                            corner_radius: number(a, "radius", 0.)?,
                            segments,
                        }
                    }
                    "polygon" => {
                        fields(a, &["shape", "points"])?;
                        Profile::Polyline {
                            points: points2(required(a, "points")?, 3)?,
                        }
                    }
                    _ => return Err("Unknown profile shape".into()),
                };
                self.charge(4096, 4096 * 64)?;
                let points = profile
                    .points(ProfileLimits {
                        points: 4096,
                        scratch_bytes: 4096 * 64,
                    })
                    .map_err(|e| e.to_string())?;
                self.store_profile(points)
            }
            "hypermedia.locus.transform" | "hypermedia.profile.transform" => {
                fields(a, &["profile", "matrix"])?;
                let [ma, b, c, d, e, f] = vector::<6>(required(a, "matrix")?)?;
                let matrix = Mat3 {
                    a: ma,
                    b,
                    c,
                    d,
                    e,
                    f,
                };
                let points = self.profile(required(a, "profile")?)?;
                let n = points.len();
                self.charge(n * 8, n * 16)?;
                let points = self
                    .profile(required(a, "profile")?)?
                    .iter()
                    .map(|p| {
                        let p = matrix.apply(Vec2::new(p[0], p[1]));
                        [p.x, p.y]
                    })
                    .collect();
                self.store_profile(points)
            }
            "hypermedia.locus.project_svg" | "hypermedia.profile.svg" => {
                fields(a, &["profile", "fill", "padding"])?;
                let points = self.profile(required(a, "profile")?)?;
                let n = points.len();
                self.charge(n, n * 768 + 512)?;
                let points = self.profile(required(a, "profile")?)?;
                let fill = a
                    .get("fill")
                    .map(color)
                    .transpose()?
                    .unwrap_or([40, 120, 220, 255]);
                let pad = number(a, "padding", 0.)?;
                if pad < 0. || pad > 1e6 {
                    return Err("Invalid SVG padding".into());
                }
                let (min, max) = bounds2(points);
                let mut svg=format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\"><polygon fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{}\" points=\"", min[0]-pad,min[1]-pad,max[0]-min[0]+2.*pad,max[1]-min[1]+2.*pad,fill[0],fill[1],fill[2],fill[3] as f64/255.);
                use std::fmt::Write;
                for p in points {
                    write!(&mut svg, "{},{} ", p[0], p[1]).unwrap();
                }
                svg.push_str("\"/></svg>\n");
                self.encode("svg", svg.into_bytes())
            }
            "hypermedia.manifold.extrude" | "hypermedia.mesh.extrude" => {
                fields(a, &["profile", "path", "caps", "stations"])?;
                let section = self.profile(required(a, "profile")?)?;
                let caps = boolean(a, "caps", true)?;
                let section_len = section.len();
                self.charge(if caps { section_len * section_len } else { 0 }, 0)?;
                let section = self.profile(required(a, "profile")?)?;
                if caps && !convex(section) {
                    return Err("Capped extrusion requires a convex profile; use caps:false for an open surface".into());
                }
                let path = points3(required(a, "path")?, 2)?;
                if path.windows(2).any(|p| p[0] == p[1]) {
                    return Err("Extrusion path has a repeated station".into());
                }
                let n = path.len();
                let stations_opt = if let Some(st_val) = a.get("stations") {
                    let st_list = list(st_val)?;
                    if st_list.len() != n {
                        return Err(format!(
                            "Extrusion stations count ({}) must match path length ({n})",
                            st_list.len()
                        ));
                    }
                    let mut parsed = Vec::with_capacity(n);
                    for item in st_list {
                        let st = match item {
                            Value::Record(r) => {
                                fields(r, &["scale_x", "scale_y", "twist_rad"])?;
                                cg::sweep_extrusion::ExtrusionStation {
                                    scale_x: number(r, "scale_x", 1.0)?,
                                    scale_y: number(r, "scale_y", 1.0)?,
                                    twist_rad: number(r, "twist_rad", 0.0)?,
                                }
                            }
                            Value::List(coords) if coords.len() == 2 => {
                                cg::sweep_extrusion::ExtrusionStation {
                                    scale_x: num(&coords[0])?,
                                    scale_y: num(&coords[1])?,
                                    twist_rad: 0.0,
                                }
                            }
                            Value::List(coords) if coords.len() == 3 => {
                                cg::sweep_extrusion::ExtrusionStation {
                                    scale_x: num(&coords[0])?,
                                    scale_y: num(&coords[1])?,
                                    twist_rad: num(&coords[2])?,
                                }
                            }
                            other => {
                                let s = num(other)?;
                                cg::sweep_extrusion::ExtrusionStation {
                                    scale_x: s,
                                    scale_y: s,
                                    twist_rad: 0.0,
                                }
                            }
                        };
                        if st.scale_x <= 0.0 || st.scale_y <= 0.0 {
                            return Err("Extrusion station scales must be positive".into());
                        }
                        parsed.push(st);
                    }
                    Some(parsed)
                } else {
                    None
                };
                let m = section.len();
                let nv = n * m + if caps { 2 } else { 0 };
                let nt = (n - 1) * m * 2 + if caps { 2 * m } else { 0 };
                self.mesh_budget(nv, nt)?;
                let section: Vec<_> = self
                    .profile(required(a, "profile")?)?
                    .iter()
                    .map(|p| Point2::new(p[0], p[1]))
                    .collect();
                let mut frames = vec![
                    cg::math_geometry::AffineFrame3 {
                        origin: Point3::default(),
                        e0: Point3::default(),
                        e1: Point3::default(),
                        e2: Point3::default()
                    };
                    n
                ];
                let mut vertices = vec![Point3::default(); nv];
                let mut triangles = vec![[0; 3]; nt];
                let (nv, nt) = cg::extrude_profile_along_path(
                    &path,
                    &section,
                    stations_opt.as_deref(),
                    true,
                    caps,
                    &mut frames,
                    &mut vertices,
                    &mut triangles,
                )
                .map_err(|e| format!("Extrusion: {e:?}"))?;
                vertices.truncate(nv);
                triangles.truncate(nt);
                self.store_mesh(vertices, triangles)
            }
            "hypermedia.manifold.revolve" | "hypermedia.mesh.revolve" => {
                fields(a, &["points", "segments"])?;
                let points = points2(required(a, "points")?, 2)?;
                if points.iter().any(|p| p[0] <= 0.) {
                    return Err("Revolve radii must be positive (surface only)".into());
                }
                let segments = count(a, "segments", 32, 256)?;
                if segments < 3 {
                    return Err("Revolve needs at least 3 segments".into());
                }
                let nv = points.len() * segments;
                let nt = (points.len() - 1) * segments * 2;
                self.mesh_budget(nv, nt)?;
                let points: Vec<_> = points.iter().map(|p| Point3::new(p[0], p[1], 0.)).collect();
                let mut v = vec![Point3::default(); nv];
                let mut t = vec![[0; 3]; nt];
                cg::parametric_cad::revolve_profile(&points, segments, &mut v, &mut t)
                    .map_err(|e| format!("Revolve: {e:?}"))?;
                self.store_mesh(v, t)
            }
            "hypermedia.manifold.loft" | "hypermedia.mesh.loft" => {
                let interpolation = match a.get("interpolation") {
                    Some(v) => text(v)?,
                    None => "linear",
                };
                if interpolation == "cubic" {
                    return self.manifold_ext("hypermedia.manifold.loft", a);
                }
                if interpolation != "linear" {
                    return Err(
                        "Unknown interpolation: expected 'linear' (ruled) or 'cubic' (Catmull-Rom)"
                            .into(),
                    );
                }
                fields(a, &["a", "b", "sections", "caps", "interpolation", "samples"])?;
                if let Some(sections_val) = a.get("sections") {
                    let ring_vals = list(sections_val)?;
                    if ring_vals.len() < 2 || ring_vals.len() > 256 {
                        return Err("Multi-section loft requires 2..=256 sections".into());
                    }
                    let mut rings = Vec::with_capacity(ring_vals.len());
                    let mut ring_len = 0usize;
                    for (idx, rv) in ring_vals.iter().enumerate() {
                        let pts = points3(rv, 3)?;
                        if idx == 0 {
                            ring_len = pts.len();
                        } else if pts.len() != ring_len {
                            return Err("All loft sections must have identical point counts".into());
                        }
                        rings.push(pts);
                    }
                    let caps = boolean(a, "caps", true)?;
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
                } else {
                    let first = points3(required(a, "a")?, 2)?;
                    let second = points3(required(a, "b")?, 2)?;
                    if first.len() != second.len() {
                        return Err("Loft curves need corresponding point counts".into());
                    }
                    let nv = first.len() * 2;
                    let nt = (first.len() - 1) * 2;
                    self.mesh_budget(nv, nt)?;
                    let mut v = vec![Point3::default(); nv];
                    let mut t = vec![[0; 3]; nt];
                    cg::parametric_cad::loft_profiles(&first, &second, &mut v, &mut t)
                        .map_err(|e| format!("Loft: {e:?}"))?;
                    self.store_mesh(v, t)
                }
            }
            "hypermedia.manifold.compile_10d" | "hypermedia.mesh.encode" => {
                fields(a, &["mesh"])?;
                let Artifact::Mesh(mesh) = self.workspace.get(required(a, "mesh")?)? else {
                    return Err("Expected mesh handle".into());
                };
                let n = mesh.positions.len() + mesh.triangles.len();
                self.charge(n * 20, n * 128)?;
                let Artifact::Mesh(mesh) = self.workspace.get(required(a, "mesh")?)? else {
                    unreachable!()
                };
                let bytes = qualia_core_db::render::compile_10d::compile_mesh_to_10d(mesh)
                    .map_err(|e| format!("10D encoding: {e:?}"))?;
                self.encode("10d", bytes)
            }
            _ => Err("Unknown geometry operation".into()),
        }
    }
    pub(crate) fn profile(&self, v: &Value) -> Result<&[[f64; 2]], String> {
        match self.workspace.get(v)? {
            Artifact::Profile(p) => Ok(p),
            _ => Err("Expected profile handle".into()),
        }
    }
    pub(crate) fn store_profile(&mut self, mut points: Vec<[f64; 2]>) -> Result<Value, String> {
        if points
            .iter()
            .flatten()
            .any(|n| !n.is_finite() || n.abs() > 1e6)
        {
            return Err("Profile coordinates outside finite +/-1000000 range".into());
        }
        let pts: Vec<_> = points.iter().map(|p| Vec2::new(p[0], p[1])).collect();
        let area = drafting_geometry::shoelace(&pts);
        if area.abs() < 1e-12 {
            return Err("Degenerate profile".into());
        }
        if area < 0. {
            points.reverse();
        }
        self.workspace
            .insert(Artifact::Profile(points), self.limits)
    }
    pub(crate) fn mesh_budget(&mut self, nv: usize, nt: usize) -> Result<(), String> {
        if nv > 200_000 || nt > 400_000 {
            return Err("Mesh size exceeds limit".into());
        }
        self.charge((nv + nt) * 20, nv * 48 + nt * 12)
    }
    pub(crate) fn store_mesh(
        &mut self,
        vertices: Vec<Point3>,
        triangles: Vec<[u32; 3]>,
    ) -> Result<Value, String> {
        let positions: Vec<[f32; 3]> = vertices
            .iter()
            .map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect();
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
}
fn points2(v: &Value, min: usize) -> Result<Vec<[f64; 2]>, String> {
    let values = list(v)?;
    if values.len() < min || values.len() > 4096 {
        return Err("Point count outside bounds".into());
    }
    let p: Vec<_> = values.iter().map(vector::<2>).collect::<Result<_, _>>()?;
    if p.iter().flatten().any(|v| v.abs() > 1e6) {
        return Err("Coordinate outside +/-1000000 range".into());
    }
    Ok(p)
}
fn points3(v: &Value, min: usize) -> Result<Vec<Point3>, String> {
    let values = list(v)?;
    if values.len() < min || values.len() > 4096 {
        return Err("Point count outside bounds".into());
    }
    values
        .iter()
        .map(|v| {
            let p = vector::<3>(v)?;
            if p.iter().any(|v| v.abs() > 1e6) {
                return Err("Coordinate outside +/-1000000 range".into());
            }
            Ok(Point3::new(p[0], p[1], p[2]))
        })
        .collect()
}
fn bounds2(points: &[[f64; 2]]) -> ([f64; 2], [f64; 2]) {
    let mut min = [f64::INFINITY; 2];
    let mut max = [f64::NEG_INFINITY; 2];
    for p in points {
        for i in 0..2 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    (min, max)
}
// Require every vertex on the same side of every edge: also rejects star polygons.
fn convex(p: &[[f64; 2]]) -> bool {
    let mut sign = 0.;
    for i in 0..p.len() {
        let a = p[i];
        let b = p[(i + 1) % p.len()];
        for c in p {
            let cross = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
            if cross.abs() > 1e-10 {
                if sign == 0. {
                    sign = cross.signum();
                } else if sign * cross < 0. {
                    return false;
                }
            }
        }
    }
    sign != 0.
}
pub(crate) fn color(v: &Value) -> Result<[u8; 4], String> {
    let p = list(v)?;
    if p.len() != 4 {
        return Err("Color needs four RGBA bytes".into());
    }
    let mut c = [0; 4];
    for (i, v) in p.iter().enumerate() {
        c[i] = u8::try_from(integer(v, "color")?).map_err(|_| "Color outside 0..255")?;
    }
    Ok(c)
}

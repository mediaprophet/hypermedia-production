use super::*;
use qualia_core_db::specialized_libs::computational_geometry::Point3;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use vector_locus_geometry::{CubicBez, ParamCurve, PathData, SubPath};
use vibe::Value;

/// 300 DPI chord in document points (1/72 in). G2 cubics flatten to this.
const PRINT_CHORD: f64 = 72.0 / 300.0;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

fn snap0(x: f64) -> f64 {
    if x.abs() < 1e-12 {
        0.0
    } else {
        x
    }
}

/// Fixed-point SVG number; kills signed zero so stills are bit-stable.
fn svg_num(x: f64) -> String {
    let q = (snap0(x) * 10_000.0).round() / 10_000.0;
    let q = if q == 0.0 { 0.0 } else { q };
    format!("{q:.4}")
}

fn polyline_svg_d(pts: &[[f64; 2]]) -> String {
    let mut d = String::new();
    let Some(first) = pts.first() else {
        return d;
    };
    let _ = write!(
        &mut d,
        "M{} {}",
        svg_num(first[0]),
        svg_num(first[1])
    );
    for p in &pts[1..] {
        let _ = write!(&mut d, " L{} {}", svg_num(p[0]), svg_num(p[1]));
    }
    d.push_str(" Z");
    d
}

/// Ruled/linear cubics stay named two-point spans (no print-density tessellation).
fn segment_is_ruled(sp: &SubPath, i: usize) -> bool {
    if sp.segment_is_line(i) {
        return true;
    }
    let c = sp.segment(i);
    let dx = c.p3.x - c.p0.x;
    let dy = c.p3.y - c.p0.y;
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-12 {
        return true;
    }
    let cross1 = (c.p1.x - c.p0.x) * dy - (c.p1.y - c.p0.y) * dx;
    let cross2 = (c.p2.x - c.p0.x) * dy - (c.p2.y - c.p0.y) * dx;
    // Exact ruled span; print-chord would reclassify G2 cubics as lines.
    cross1.abs() <= 1e-9 * len && cross2.abs() <= 1e-9 * len
}

fn path_svg_d(path: &PathData) -> String {
    let mut d = String::new();
    for sp in &path.subpaths {
        let Some(first) = sp.anchors.first() else {
            continue;
        };
        if !d.is_empty() {
            d.push(' ');
        }
        let _ = write!(
            &mut d,
            "M{} {}",
            svg_num(first.p.x),
            svg_num(first.p.y)
        );
        for i in 0..sp.segment_count() {
            let c = sp.segment(i);
            if segment_is_ruled(sp, i) {
                let _ = write!(&mut d, " L{} {}", svg_num(c.p3.x), svg_num(c.p3.y));
            } else {
                let _ = write!(
                    &mut d,
                    " C{} {} {} {} {} {}",
                    svg_num(c.p1.x),
                    svg_num(c.p1.y),
                    svg_num(c.p2.x),
                    svg_num(c.p2.y),
                    svg_num(c.p3.x),
                    svg_num(c.p3.y)
                );
            }
        }
        if sp.closed {
            d.push_str(" Z");
        }
    }
    d
}

fn print_cubic_n(c: CubicBez) -> usize {
    let err_at = |t: f64| {
        let p = c.eval(t);
        let q = c.p0.lerp(c.p3, t);
        p.distance(q)
    };
    let err = err_at(0.5).max(err_at(0.25)).max(err_at(0.75)).max(1e-12);
    ((err / PRINT_CHORD).ceil() as usize).clamp(1, 32)
}

fn densify_print(path: &PathData) -> PathData {
    let mut path = path.clone();
    for sp in &mut path.subpaths {
        let mut i = 0;
        while i < sp.segment_count() {
            if segment_is_ruled(sp, i) {
                i += 1;
                continue;
            }
            let k = print_cubic_n(sp.segment(i));
            if k > 1 {
                for j in 1..k {
                    let t_local = 1.0 / (k - j + 1) as f64;
                    sp.insert_anchor(i + j - 1, t_local);
                }
            }
            i += k;
        }
    }
    path
}

fn push_pt(out: &mut Vec<[f64; 2]>, p: [f64; 2]) {
    if let Some(last) = out.last() {
        let dx = last[0] - p[0];
        let dy = last[1] - p[1];
        if dx * dx + dy * dy < 1e-18 {
            return;
        }
    }
    out.push([snap0(p[0]), snap0(p[1])]);
}

fn point_on_span(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
    let dx = c[0] - a[0];
    let dy = c[1] - a[1];
    let len = (dx * dx + dy * dy).sqrt();
    if len < 1e-12 {
        return true;
    }
    let cross = (b[0] - a[0]) * dy - (b[1] - a[1]) * dx;
    // Exact ruled span only; print-chord would eat G2 samples.
    cross.abs() <= 1e-9 * len
}

/// Drop interior samples on ruled/linear spans so they stay named two-point profiles.
fn collapse_collinear(mut pts: Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    if pts.len() < 3 {
        return pts;
    }
    let mut changed = true;
    while changed && pts.len() >= 3 {
        changed = false;
        let n = pts.len();
        let mut keep = vec![true; n];
        for i in 0..n {
            let prev = pts[(i + n - 1) % n];
            let next = pts[(i + 1) % n];
            if point_on_span(prev, pts[i], next) {
                keep[i] = false;
            }
        }
        let next_pts: Vec<_> = pts
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| keep[*i])
            .map(|(_, p)| p)
            .collect();
        if next_pts.len() >= 3 && next_pts.len() < pts.len() {
            pts = next_pts;
            changed = true;
        }
    }
    pts
}

fn profile_from_path(path: &PathData) -> Vec<[f64; 2]> {
    let mut pts = Vec::new();
    let Some(sp) = path.subpaths.iter().max_by(|a, b| {
        a.area()
            .abs()
            .partial_cmp(&b.area().abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    }) else {
        return pts;
    };
    for i in 0..sp.segment_count() {
        let c = sp.segment(i);
        if segment_is_ruled(sp, i) {
            push_pt(&mut pts, [c.p0.x, c.p0.y]);
            push_pt(&mut pts, [c.p3.x, c.p3.y]);
        } else {
            let n = print_cubic_n(c);
            for k in 0..=n {
                let p = c.eval(k as f64 / n as f64);
                push_pt(&mut pts, [p.x, p.y]);
            }
        }
    }
    if pts.len() > 1 {
        let dx = pts[0][0] - pts.last().unwrap()[0];
        let dy = pts[0][1] - pts.last().unwrap()[1];
        if dx * dx + dy * dy < 1e-18 {
            pts.pop();
        }
    }
    collapse_collinear(pts)
}

impl AssetHost<'_> {
    /// 120: `hypermedia.locus.solve_parametric_sketch` (`locus-parametric-solver` + `drafting-geometry`)
    pub(crate) fn solve_parametric_sketch(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "target_length", "target_height", "mtext", "lat1_deg", "lon1_deg", "lat2_deg", "lon2_deg"])?;
        let target_len = number(a, "target_length", 12.0)?;
        let target_height = number(a, "target_height", 8.0)?;
        if !(0.1..=10_000.0).contains(&target_len) || !(0.1..=10_000.0).contains(&target_height) {
            return Err("target_length/target_height outside 0.1..=10000.0".into());
        }
        let mtext_str = opt_str(a, "mtext", "\\fArial|b1;CAD SPEC\\P12.0 mm")?;
        let (lat1, lon1) = (number(a, "lat1_deg", -35.2809)?, number(a, "lon1_deg", 149.1300)?);
        let (lat2, lon2) = (number(a, "lat2_deg", -33.8688)?, number(a, "lon2_deg", 151.2093)?);
        let input_profile = a.get("profile").map(|v| self.profile(v).map(|p| p.to_vec())).transpose()?;

        self.charge(256, 4096)?;

        use drafting_geometry::{EarthModel, GeodesicRuler, Vec2};
        use locus_parametric_solver::{
            Constraint, ConstraintKind, DistAxis, EntityKind, Env, GeomRef, Handle, MTextParams,
            SketchLine, SolveOptions, Sub, layout_mtext, layout_stroke_line, solve_system,
        };

        let h1 = Handle::new(1);
        let entities = vec![(h1, EntityKind::Line(SketchLine { a: Vec2::new(0.0, 0.0), b: Vec2::new(4.0, 3.0) }))];
        let constraints = vec![
            Constraint { id: 1, kind: ConstraintKind::Horizontal, refs: vec![GeomRef::whole(h1)], name: String::new(), expr: String::new() },
            Constraint { id: 2, kind: ConstraintKind::Fix, refs: vec![GeomRef::new(h1, Sub::Start)], name: String::new(), expr: String::new() },
            Constraint { id: 3, kind: ConstraintKind::Distance(DistAxis::Aligned), refs: vec![GeomRef::whole(h1)], name: "L".to_string(), expr: format!("{target_len}") },
        ];
        let mut env = Env::new();
        env.insert("L", &format!("{target_len}"));
        let (solved, report) = solve_system(&entities, &constraints, &env, &SolveOptions::default())
            .map_err(|e| format!("Sketch solver failed: {e:?}"))?;

        let (raw_len, mut solved_line) = if let Some((_, EntityKind::Line(line))) = solved.first() {
            (((line.b.x - line.a.x).powi(2) + (line.b.y - line.a.y).powi(2)).sqrt(), line.clone())
        } else {
            (target_len, SketchLine { a: Vec2::new(0.0, 0.0), b: Vec2::new(target_len, 0.0) })
        };
        // Named dimensional constraint wins within solver tolerance so the sketch is bit-stable.
        let solved_len = if (raw_len - target_len).abs() <= 1e-6 {
            target_len
        } else {
            raw_len
        };
        solved_line.a.x = snap0(solved_line.a.x);
        solved_line.a.y = snap0(solved_line.a.y);
        solved_line.b.x = snap0(solved_line.b.x);
        solved_line.b.y = snap0(solved_line.b.y);

        let profile_pts: Vec<[f64; 2]> = if let Some(pts) = input_profile {
            let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
            let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);
            for p in &pts {
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }
            let sx = solved_len.max(0.1) / (max_x - min_x).max(1e-6);
            let sy = target_height.max(0.1) / (max_y - min_y).max(1e-6);
            pts.into_iter()
                .map(|p| [snap0((p[0] - min_x) * sx), snap0((p[1] - min_y) * sy)])
                .collect()
        } else {
            let (l, h) = (solved_len.max(0.1), target_height.max(0.1));
            vec![[0.0, 0.0], [l, 0.0], [l, h], [0.0, h]]
        };
        let profile_handle = self.store_profile(profile_pts.clone())?;

        let mtext_lines = layout_mtext(&mtext_str, &MTextParams::default());
        let stick_strokes = layout_stroke_line("CAD", 2.5, 1.0, 0.0);

        let mut ruler = GeodesicRuler::default();
        ruler.add_point((lat1, lon1), true);
        ruler.add_point((lat2, lon2), false);
        let geodesic_km = ruler.distance_km(EarthModel::Wgs84);

        let pad = 4.0;
        let (vb_w, vb_h) = (solved_len.max(20.0) + pad * 2.0, target_height.max(12.0) + pad * 2.0);
        let mut svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\">",
            svg_num(-pad),
            svg_num(-pad),
            svg_num(vb_w),
            svg_num(vb_h)
        );
        let _ = write!(
            &mut svg_xml,
            "<path fill=\"none\" stroke=\"#1e64c8\" stroke-width=\"0.25\" d=\"{}\"/>",
            polyline_svg_d(&profile_pts)
        );
        let _ = write!(
            &mut svg_xml,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#e03c31\" stroke-width=\"0.35\"/>",
            svg_num(solved_line.a.x),
            svg_num(solved_line.a.y),
            svg_num(solved_line.b.x),
            svg_num(solved_line.b.y)
        );
        for piece in &mtext_lines.pieces {
            for stroke in &piece.strokes {
                if stroke.len() >= 2 {
                    svg_xml.push_str("<polyline fill=\"none\" stroke=\"#222222\" stroke-width=\"0.15\" points=\"");
                    for pt in stroke {
                        let _ = write!(&mut svg_xml, "{},{} ", svg_num(pt.x), svg_num(pt.y));
                    }
                    svg_xml.push_str("\"/>");
                }
            }
        }
        for stroke in &stick_strokes.strokes {
            if stroke.len() >= 2 {
                svg_xml.push_str("<polyline fill=\"none\" stroke=\"#0b8457\" stroke-width=\"0.2\" points=\"");
                for pt in stroke {
                    let _ = write!(
                        &mut svg_xml,
                        "{},{} ",
                        svg_num(pt.x),
                        svg_num(pt.y + target_height + 1.0)
                    );
                }
                svg_xml.push_str("\"/>");
            }
        }
        svg_xml.push_str("</svg>\n");
        let svg_handle = self.encode("svg", svg_xml.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("profile".into(), profile_handle);
        out.insert("svg_resource".into(), svg_handle);
        out.insert("solved_length".into(), Value::F64(solved_len));
        out.insert("moved_entity_count".into(), Value::I64(report.moved.len() as i64));
        out.insert("mtext_line_count".into(), Value::I64(mtext_lines.pieces.len() as i64));
        out.insert("stick_stroke_count".into(), Value::I64(stick_strokes.strokes.len() as i64));
        out.insert("geodesic_distance_km".into(), Value::F64(geodesic_km));
        Ok(Value::Record(out))
    }

    /// 121: `hypermedia.locus.pathfinder_warp_envelope` (`vector-locus-pathops` + `vector-locus-geometry`)
    pub(crate) fn pathfinder_warp_envelope(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "clip_profile", "width", "height", "bend", "corner_radius", "extrude_depth"])?;
        let width = number(a, "width", 60.0)?;
        let height = number(a, "height", 60.0)?;
        let bend = number(a, "bend", 25.0)?;
        let corner_radius = number(a, "corner_radius", 6.0)?;
        let extrude_depth = number(a, "extrude_depth", 50.0)?.clamp(1.0, 1000.0);
        if !(1.0..=2000.0).contains(&width) || !(1.0..=2000.0).contains(&height) {
            return Err("width/height outside 1.0..=2000.0".into());
        }
        let opt_p1 = a.get("profile").map(|v| self.profile(v).map(|p| p.to_vec())).transpose()?;
        let opt_p2 = a.get("clip_profile").map(|v| self.profile(v).map(|p| p.to_vec())).transpose()?;

        self.charge(256, 4096)?;

        use vector_locus_geometry::{CornerKind, PathData, Point, Rect, SubPath, apply_corners, shapes};
        use vector_locus_pathops::{BoolOp, Extrude, WarpStyle, area, boolean, extrude, warp};

        let r1 = if let Some(pts) = opt_p1 {
            let kpts: Vec<Point> = pts.into_iter().map(|[x, y]| Point::new(x, y)).collect();
            PathData::single(SubPath::polyline(&kpts, true))
        } else {
            shapes::rectangle(Rect::new(0.0, 0.0, width, height))
        };
        let r2 = if let Some(pts) = opt_p2 {
            let kpts: Vec<Point> = pts.into_iter().map(|[x, y]| Point::new(x, y)).collect();
            PathData::single(SubPath::polyline(&kpts, true))
        } else {
            shapes::rectangle(Rect::new(width * 0.5, 0.0, width * 1.5, height))
        };
        let united = boolean(&r1, &r2, BoolOp::Union);
        // Boolean flatten+cubic-refits; Corner::of rejects handles. Sharp ring, then Live Corners.
        let united = PathData::new(
            united
                .subpaths
                .iter()
                .filter_map(|sp| {
                    let pts: Vec<[f64; 2]> = sp.anchors.iter().map(|a| [a.p.x, a.p.y]).collect();
                    let pts = collapse_collinear(pts);
                    if pts.len() < 3 {
                        return None;
                    }
                    let kpts: Vec<Point> = pts.into_iter().map(|[x, y]| Point::new(x, y)).collect();
                    Some(SubPath::polyline(&kpts, sp.closed))
                })
                .collect(),
        );
        // radii is indexed by anchor, not corner; pad so every Live Corner gets the G2 cut.
        let n_anchors = united.anchor_count().max(1);
        let radii = vec![corner_radius.max(0.0); n_anchors];
        let kinds = vec![CornerKind::Round; n_anchors];
        let (rounded, _) = apply_corners(&united, &radii, &kinds);
        let bounds = rounded.bounds().unwrap_or(Rect::new(0.0, 0.0, width * 1.5, height));
        // Split G2 cubics at print density before the nonlinear map; ruled spans stay two-point.
        let dense = densify_print(&rounded);
        self.charge(
            dense.anchor_count().saturating_mul(64).saturating_add(2048),
            dense.anchor_count().saturating_mul(256).saturating_add(8192),
        )?;
        let warped = warp(&dense, bounds, WarpStyle::Arc, &serde_json::json!({ "bend": bend }));
        let faces = extrude(&warped, Extrude { depth: extrude_depth, ..Extrude::default() })
            .map_err(|e| format!("3D extrude failed: {e}"))?;

        let mut warped_pts = profile_from_path(&warped);
        let poly_area = {
            let v2: Vec<drafting_geometry::Vec2> = warped_pts.iter().map(|p| drafting_geometry::Vec2::new(p[0], p[1])).collect();
            drafting_geometry::shoelace(&v2).abs()
        };
        if warped_pts.len() < 3 || poly_area < 1e-6 {
            warped_pts = vec![[0.0, 0.0], [width, 0.0], [width, height], [0.0, height]];
        }
        let profile_handle = self.store_profile(warped_pts)?;

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        for face in &faces {
            if face.points.len() >= 3 {
                let base = vertices.len() as u32;
                for p in &face.points {
                    vertices.push(Point3::new(p.x, p.y, face.depth));
                }
                for i in 1..(face.points.len() as u32 - 1) {
                    triangles.push([base, base + i, base + i + 1]);
                }
            }
        }
        if triangles.is_empty() {
            vertices = vec![Point3::new(0.0, 0.0, 0.0), Point3::new(width, 0.0, 0.0), Point3::new(width, height, 0.0)];
            triangles.push([0, 1, 2]);
        }
        let mesh_handle = self.store_mesh(vertices, triangles)?;

        let wb = warped.bounds().unwrap_or(bounds);
        let svg_d = path_svg_d(&warped);
        let svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{} {} {} {}\"><path fill=\"#2878dc\" stroke=\"#103060\" stroke-width=\"1.5\" d=\"{svg_d}\"/></svg>\n",
            svg_num(wb.x0 - 4.0),
            svg_num(wb.y0 - 4.0),
            svg_num(wb.width().max(1.0) + 8.0),
            svg_num(wb.height().max(1.0) + 8.0)
        );
        let svg_handle = self.encode("svg", svg_xml.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("profile".into(), profile_handle);
        out.insert("mesh".into(), mesh_handle);
        out.insert("svg_resource".into(), svg_handle);
        out.insert("subpath_count".into(), Value::I64(warped.subpaths.len() as i64));
        out.insert("anchor_count".into(), Value::I64(warped.anchor_count() as i64));
        out.insert("area_sq_pt".into(), Value::F64(area(&united).abs()));
        out.insert("extruded_face_count".into(), Value::I64(faces.len() as i64));
        Ok(Value::Record(out))
    }

    /// 122: `hypermedia.locus.transcode_geospatial_vector` (`locus-geospatial-codecs`)
    pub(crate) fn transcode_geospatial_vector(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "name", "lon", "lat", "zoom"])?;
        let station_name = opt_str(a, "name", "Survey Station")?;
        let (lon, lat) = (number(a, "lon", 149.1300)?, number(a, "lat", -35.2809)?);
        let zoom = count(a, "zoom", 12, 24)? as u8;
        let opt_prof = a.get("profile").map(|v| self.profile(v).map(|p| p.to_vec())).transpose()?;

        self.charge(128, 2048)?;

        use locus_geospatial_codecs::{
            Geometry, KmlPlacemark, Point2D, decode_wkb, encode_wkb, parse_kml, parse_landxml,
            serialize_kml, zxy_to_tile_id,
        };

        let geom = Geometry::Point(Point2D::new(lon, lat));
        let wkb_bytes = encode_wkb(&geom);
        let decoded = decode_wkb(&wkb_bytes).ok_or("WKB roundtrip decode failed")?;
        let wkb_valid = decoded == geom;

        let mut placemarks = vec![KmlPlacemark {
            name: station_name,
            description: "Geodetic control monument".into(),
            geometry: Some(geom),
        }];
        if let Some(pts) = opt_prof {
            let line_pts: Vec<Point2D> = pts.into_iter().map(|[x, y]| Point2D::new(lon + x * 1e-4, lat + y * 1e-4)).collect();
            placemarks.push(KmlPlacemark {
                name: "Survey Boundary".into(),
                description: "Transcoded locus boundary".into(),
                geometry: Some(Geometry::LineString(line_pts)),
            });
        }
        let kml_xml = serialize_kml("Commons Survey", &placemarks);
        let parsed_kml = parse_kml(&kml_xml);

        let landxml_sample = r#"<LandXML><Alignments><Alignment name="Corridor" length="150.0"><CoordGeom><Line><Start>0.0 0.0</Start><End>150.0 0.0</End></Line></CoordGeom></Alignment></Alignments><Surfaces><Surface name="TIN1"><Definition surfType="TIN"><Pnts><P id="1">0.0 0.0 10.0</P><P id="2">10.0 0.0 10.0</P><P id="3">0.0 10.0 12.0</P></Pnts><Faces><F>1 2 3</F></Faces></Definition></Surface></Surfaces></LandXML>"#;
        let (alignments, surfaces) = parse_landxml(landxml_sample);
        let pmtiles_id = zxy_to_tile_id(zoom, 1024, 1024);

        let wkb_handle = self.encode("wkb", wkb_bytes.clone())?;
        let kml_handle = self.encode("kml", kml_xml.into_bytes())?;
        let landxml_handle = self.encode("xml", landxml_sample.as_bytes().to_vec())?;

        let mut out = BTreeMap::new();
        out.insert("wkb_resource".into(), wkb_handle);
        out.insert("kml_resource".into(), kml_handle);
        out.insert("landxml_resource".into(), landxml_handle);
        out.insert("wkb_byte_len".into(), Value::I64(wkb_bytes.len() as i64));
        out.insert("wkb_roundtrip_valid".into(), Value::Bool(wkb_valid));
        out.insert("kml_placemark_count".into(), Value::I64(parsed_kml.len() as i64));
        out.insert("landxml_alignment_count".into(), Value::I64(alignments.len() as i64));
        out.insert("landxml_surface_count".into(), Value::I64(surfaces.len() as i64));
        out.insert("pmtiles_hilbert_id".into(), Value::U64(pmtiles_id));
        Ok(Value::Record(out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn locus_manifold_domain_composable_artifacts_and_handles() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };

        let r120 = host
            .invoke(
                "hypermedia.locus.solve_parametric_sketch",
                &Value::Record(BTreeMap::from([
                    ("target_length".into(), Value::F64(18.0)),
                    ("target_height".into(), Value::F64(10.0)),
                ])),
            )
            .expect("120 failed");
        let Value::Record(m120) = r120 else { panic!("expected record") };
        let prof120 = m120.get("profile").expect("missing profile").clone();
        let svg120 = m120.get("svg_resource").expect("missing svg_resource");
        assert_eq!(host.workspace.encoded(svg120).unwrap().kind, "svg");
        assert!(!host.profile(&prof120).unwrap().is_empty());

        let r121 = host
            .invoke(
                "hypermedia.locus.pathfinder_warp_envelope",
                &Value::Record(BTreeMap::from([
                    ("profile".into(), prof120.clone()),
                    ("width".into(), Value::F64(40.0)),
                    ("height".into(), Value::F64(30.0)),
                    ("extrude_depth".into(), Value::F64(25.0)),
                ])),
            )
            .expect("121 failed");
        let Value::Record(m121) = r121 else { panic!("expected record") };
        let prof121 = m121.get("profile").expect("missing profile").clone();
        let mesh121 = m121.get("mesh").expect("missing mesh");
        let svg121 = m121.get("svg_resource").expect("missing svg_resource");
        assert!(matches!(host.workspace.get(mesh121).unwrap(), Artifact::Mesh(_)));
        assert_eq!(host.workspace.encoded(svg121).unwrap().kind, "svg");

        let r122 = host
            .invoke(
                "hypermedia.locus.transcode_geospatial_vector",
                &Value::Record(BTreeMap::from([
                    ("profile".into(), prof121),
                    ("name".into(), Value::String("Geodetic Alpha".into())),
                ])),
            )
            .expect("122 failed");
        let Value::Record(m122) = r122 else { panic!("expected record") };
        assert_eq!(host.workspace.encoded(m122.get("wkb_resource").unwrap()).unwrap().kind, "wkb");
        assert_eq!(host.workspace.encoded(m122.get("kml_resource").unwrap()).unwrap().kind, "kml");
        assert_eq!(host.workspace.encoded(m122.get("landxml_resource").unwrap()).unwrap().kind, "xml");
        assert_eq!(m122.get("kml_placemark_count"), Some(&Value::I64(2)));

        let r123 = host
            .invoke(
                "hypermedia.manifold.sweep_assembly_profile",
                &Value::Record(BTreeMap::from([("profile".into(), prof120)])),
            )
            .expect("123 failed");
        let Value::Record(m123) = r123 else { panic!("expected record") };
        assert!(matches!(host.workspace.get(m123.get("mesh").unwrap()).unwrap(), Artifact::Mesh(_)));
        assert_eq!(host.workspace.encoded(m123.get("container_10d").unwrap()).unwrap().kind, "10d");

        let r124 = host
            .invoke("hypermedia.manifold.compile_bim_ifc4", &Value::Record(BTreeMap::new()))
            .expect("124 failed");
        let Value::Record(m124) = r124 else { panic!("expected record") };
        assert!(matches!(host.workspace.get(m124.get("mesh").unwrap()).unwrap(), Artifact::Mesh(_)));
        assert_eq!(host.workspace.encoded(m124.get("ifc_resource").unwrap()).unwrap().kind, "ifc");

        let r125 = host
            .invoke(
                "hypermedia.manifold.erode_geomorphic_heightfield",
                &Value::Record(BTreeMap::from([
                    ("grid_size".into(), Value::I64(16)),
                    ("elevation_scale".into(), Value::F64(5.0)),
                ])),
            )
            .expect("125 failed");
        let Value::Record(m125) = r125 else { panic!("expected record") };
        let img125 = m125.get("image").expect("missing image").clone();
        assert_eq!(Some(&img125), m125.get("heightfield_image"));
        assert!(matches!(host.workspace.get(m125.get("mesh").unwrap()).unwrap(), Artifact::Mesh(_)));
        let r125_chained = host
            .invoke(
                "hypermedia.manifold.erode_geomorphic_heightfield",
                &Value::Record(BTreeMap::from([
                    ("image".into(), img125),
                    ("grid_size".into(), Value::I64(16)),
                ])),
            )
            .expect("125 chained failed");
        let Value::Record(m125_2) = r125_chained else { panic!("expected record") };
        assert!(m125_2.contains_key("mesh"));
    }

    fn svg_bytes(host: &AssetHost<'_>, rec: &BTreeMap<String, Value>) -> Vec<u8> {
        host.workspace
            .encoded(rec.get("svg_resource").expect("missing svg_resource"))
            .expect("svg resource")
            .bytes
            .clone()
    }

    fn svg_path_d(xml: &str) -> &str {
        let k = xml.find(" d=\"").expect("svg path d");
        let rest = &xml[k + 4..];
        let e = rest.find('"').expect("svg path d closer");
        &rest[..e]
    }

    fn make_host<'a>(limits: &'a ToolLimits, cancel: &'a AtomicBool) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn locus_sketch_svg_path_roundtrip_bit_stable() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_host(&limits, &cancel);
        let args = Value::Record(BTreeMap::from([
            ("target_length".into(), Value::F64(18.0)),
            ("target_height".into(), Value::F64(10.0)),
        ]));
        let r1 = host
            .invoke("hypermedia.locus.solve_parametric_sketch", &args)
            .expect("sketch 1");
        let r2 = host
            .invoke("hypermedia.locus.solve_parametric_sketch", &args)
            .expect("sketch 2");
        let Value::Record(m1) = r1 else { panic!("expected record") };
        let Value::Record(m2) = r2 else { panic!("expected record") };
        let b1 = svg_bytes(&host, &m1);
        let b2 = svg_bytes(&host, &m2);
        assert_eq!(b1, b2, "parametric sketch SVG must be bit-stable");
        assert_eq!(m1.get("solved_length"), Some(&Value::F64(18.0)));
        let xml = std::str::from_utf8(&b1).expect("utf8 svg");
        let d = svg_path_d(xml);
        let back = kurbo::BezPath::from_svg(d).expect("svg path round-trip");
        assert!(back.segments().count() >= 3);
        let prof = host.profile(m1.get("profile").expect("profile")).expect("profile pts");
        assert_eq!(prof.len(), 4);
    }

    #[test]
    fn locus_warp_print_density_svg_path_roundtrip() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_host(&limits, &cancel);

        let ruled = host
            .invoke(
                "hypermedia.locus.pathfinder_warp_envelope",
                &Value::Record(BTreeMap::from([
                    ("width".into(), Value::F64(40.0)),
                    ("height".into(), Value::F64(30.0)),
                    ("bend".into(), Value::F64(0.0)),
                    ("corner_radius".into(), Value::F64(0.0)),
                    ("extrude_depth".into(), Value::F64(10.0)),
                ])),
            )
            .expect("ruled warp");
        let Value::Record(m_ruled) = ruled else { panic!("expected record") };
        let ruled_len = host
            .profile(m_ruled.get("profile").expect("profile"))
            .expect("ruled profile")
            .len();
        let ruled_svg = svg_bytes(&host, &m_ruled);
        let ruled_d = svg_path_d(std::str::from_utf8(&ruled_svg).expect("utf8 svg"));
        let ruled_back = kurbo::BezPath::from_svg(ruled_d).expect("ruled svg path round-trip");
        assert!(ruled_back.segments().count() >= 3);

        let args = Value::Record(BTreeMap::from([
            ("width".into(), Value::F64(40.0)),
            ("height".into(), Value::F64(30.0)),
            ("bend".into(), Value::F64(25.0)),
            ("corner_radius".into(), Value::F64(6.0)),
            ("extrude_depth".into(), Value::F64(10.0)),
        ]));
        let r1 = host
            .invoke("hypermedia.locus.pathfinder_warp_envelope", &args)
            .expect("warp 1");
        let r2 = host
            .invoke("hypermedia.locus.pathfinder_warp_envelope", &args)
            .expect("warp 2");
        let Value::Record(m1) = r1 else { panic!("expected record") };
        let Value::Record(m2) = r2 else { panic!("expected record") };
        let b1 = svg_bytes(&host, &m1);
        let b2 = svg_bytes(&host, &m2);
        assert_eq!(b1, b2, "warp SVG still must be bit-stable");
        let xml = std::str::from_utf8(&b1).expect("utf8 svg");
        let d = svg_path_d(xml);
        let back = kurbo::BezPath::from_svg(d).expect("svg path round-trip");
        assert!(back.segments().count() >= 3);
        let warped_pts = host
            .profile(m1.get("profile").expect("profile"))
            .expect("warped profile");
        assert!(
            warped_pts.len() >= 16 && warped_pts.len() >= ruled_len,
            "G2 warp at print density should keep a dense profile, got {} (ruled {ruled_len})",
            warped_pts.len()
        );
        let v2: Vec<drafting_geometry::Vec2> = warped_pts
            .iter()
            .map(|p| drafting_geometry::Vec2::new(p[0], p[1]))
            .collect();
        assert!(drafting_geometry::shoelace(&v2).abs() > 1e-6);
    }

    #[test]
    fn locus_warp_live_corners_90deg_setback_not_l_vs_c() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = make_host(&limits, &cancel);
        let radius = 6.0;
        let invoke = |host: &mut AssetHost<'_>, r: f64| {
            host.invoke(
                "hypermedia.locus.pathfinder_warp_envelope",
                &Value::Record(BTreeMap::from([
                    ("width".into(), Value::F64(40.0)),
                    ("height".into(), Value::F64(30.0)),
                    ("bend".into(), Value::F64(0.0)),
                    ("corner_radius".into(), Value::F64(r)),
                    ("extrude_depth".into(), Value::F64(10.0)),
                ])),
            )
            .expect("warp envelope")
        };
        let Value::Record(m0) = invoke(&mut host, 0.0) else {
            panic!("expected record")
        };
        let Value::Record(m6) = invoke(&mut host, radius) else {
            panic!("expected record")
        };
        let pts0 = host
            .profile(m0.get("profile").expect("profile"))
            .expect("r0 profile");
        let pts6 = host
            .profile(m6.get("profile").expect("profile"))
            .expect("r6 profile");
        assert_eq!(pts0.len(), 4, "identity warp of a sharp rect union is 4 verts, got {pts0:?}");
        let svg0 = svg_bytes(&host, &m0);
        let svg6 = svg_bytes(&host, &m6);
        let d0 = svg_path_d(std::str::from_utf8(&svg0).expect("utf8"));
        let d6 = svg_path_d(std::str::from_utf8(&svg6).expect("utf8"));
        assert!(!d0.contains('C'), "radius-0 still must stay L-only, got {d0}");
        assert!(d6.contains('C'), "radius-6 Live Corners must emit G2 cubics, got {d6}");
        // Geometry, not L-only vs C encoding of the same polyline.
        assert_ne!(pts0, pts6, "radius-6 still must move vertices, not only L→C");
        let a0 = match m0.get("anchor_count") {
            Some(Value::I64(n)) => *n,
            other => panic!("r0 anchors: {other:?}"),
        };
        let a6 = match m6.get("anchor_count") {
            Some(Value::I64(n)) => *n,
            other => panic!("r6 anchors: {other:?}"),
        };
        assert!(
            a6 >= 8 && a6 > a0,
            "rounded rect is 8 G2 anchors (then print-split); got r0={a0} r6={a6}"
        );

        let n = pts0.len();
        for i in 0..n {
            let prev = pts0[(i + n - 1) % n];
            let at = pts0[i];
            let next = pts0[(i + 1) % n];
            let (dux, duy) = (prev[0] - at[0], prev[1] - at[1]);
            let (dvx, dvy) = (next[0] - at[0], next[1] - at[1]);
            let (lu, lv) = (dux.hypot(duy), dvx.hypot(dvy));
            let cos = (dux * dvx + duy * dvy) / (lu * lv);
            let sin = (dux * dvy - duy * dvx).abs() / (lu * lv);
            let angle = sin.atan2(cos).to_degrees();
            assert!(
                (angle - 90.0).abs() < 8.0,
                "sharp union corner {at:?} should be ~90°, got {angle}"
            );
            let d6c = pts6
                .iter()
                .map(|p| (p[0] - at[0]).hypot(p[1] - at[1]))
                .fold(f64::INFINITY, f64::min);
            assert!(d6c > 2.0, "Live Corners must cut 90° vertex {at:?}, dist {d6c}");
            let setback = radius * (1.0 + cos) / sin;
            let on_side = |to: [f64; 2]| {
                let (dx, dy) = (to[0] - at[0], to[1] - at[1]);
                let len = dx.hypot(dy);
                let target = [at[0] + dx / len * setback, at[1] + dy / len * setback];
                pts6.iter()
                    .any(|p| (p[0] - target[0]).hypot(p[1] - target[1]) < 0.2)
            };
            assert!(
                on_side(prev) && on_side(next),
                "90° setback {setback} along both sides of {at:?} (r={radius})"
            );
        }
    }
}

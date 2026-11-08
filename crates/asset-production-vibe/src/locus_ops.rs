use super::*;
use drafting_geometry::{Spline, Vec2};
use qualia_core_db::specialized_libs::computational_geometry::{self as cg, Point2};
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn locus_ext(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.locus.spline" | "locus.spline" => self.locus_spline(a),
            "hypermedia.locus.offset" | "locus.offset" => self.locus_offset(a),
            "hypermedia.locus.boolean" | "locus.boolean" => self.locus_boolean(a),
            "hypermedia.locus.stroke_to_locus" | "locus.stroke_to_locus" => {
                self.locus_stroke_to_locus(a)
            }
            _ => Err(format!("Unknown locus operation: {id}")),
        }
    }

    fn locus_spline(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["points", "closed", "tolerance"])?;
        let closed = boolean(a, "closed", true)?;
        let tolerance = number(a, "tolerance", 0.25)?;
        if !tolerance.is_finite() || tolerance < 0.01 || tolerance > 100.0 {
            return Err("Spline tolerance outside 0.01..100.0 range".into());
        }
        let fit_pts = parse_locus_points(required(a, "points")?, 3, 1024)?;
        let n = fit_pts.len();
        self.charge(n * n * 2 + 4096, 4096 * 64)?;

        let spline = if closed {
            Spline::from_fit_points_closed(&fit_pts)
        } else {
            Spline::from_fit_points(&fit_pts)
        };

        let tess = spline.tessellate(tolerance);
        let mut poly: Vec<Vec2> = Vec::with_capacity(tess.len());
        for p in tess {
            if let Some(last) = poly.last() {
                if last.near(p, 1e-9) {
                    continue;
                }
            }
            poly.push(p);
        }
        if poly.len() > 1
            && poly
                .first()
                .zip(poly.last())
                .is_some_and(|(a, b)| a.near(*b, 1e-9))
        {
            poly.pop();
        }

        if poly.len() < 3 {
            return Err("Degenerate spline locus: fewer than 3 distinct vertices".into());
        }

        let area = drafting_geometry::shoelace(&poly);
        if area.abs() < 1e-12 {
            return Err("Degenerate spline locus: zero area".into());
        }
        if area < 0.0 {
            poly.reverse();
        }

        let profile_pts: Vec<[f64; 2]> = poly.into_iter().map(|p| [p.x, p.y]).collect();
        self.store_locus_profile(profile_pts)
    }

    fn locus_offset(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "distance", "miter_limit"])?;
        let distance = number(a, "distance", 0.0)?;
        if !distance.is_finite() || distance.abs() < 1e-9 || distance.abs() > 1e5 {
            return Err("Offset distance must be non-zero and within +/-100000 range".into());
        }
        let miter_limit = number(a, "miter_limit", 4.0)?;
        if !miter_limit.is_finite() || miter_limit < 1.0 || miter_limit > 16.0 {
            return Err("Miter limit outside 1.0..16.0 range".into());
        }

        let n = {
            let pts = self.profile(required(a, "profile")?)?;
            pts.len()
        };
        self.charge(n * 32 + 2048, n * 256 + 4096)?;

        let raw_pts = self.profile(required(a, "profile")?)?.to_vec();
        let pts: Vec<Vec2> = raw_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();

        let mut clean_pts: Vec<Vec2> = Vec::with_capacity(pts.len());
        for p in pts {
            if let Some(last) = clean_pts.last() {
                if last.near(p, 1e-9) {
                    continue;
                }
            }
            clean_pts.push(p);
        }
        if clean_pts.len() > 1
            && clean_pts
                .first()
                .zip(clean_pts.last())
                .is_some_and(|(a, b)| a.near(*b, 1e-9))
        {
            clean_pts.pop();
        }

        if clean_pts.len() < 3 {
            return Err("Degenerate profile handle: fewer than 3 vertices".into());
        }

        let in_area = drafting_geometry::shoelace(&clean_pts);
        if in_area.abs() < 1e-12 {
            return Err("Degenerate profile handle: zero area".into());
        }
        if in_area < 0.0 {
            clean_pts.reverse();
        }

        let clean = offset_polygon(&clean_pts, distance, miter_limit)?;
        let area = drafting_geometry::shoelace(&clean);
        if distance < 0.0 && area <= 1e-12 {
            return Err("Inward offset collapsed or inverted profile".into());
        }
        if area.abs() < 1e-12 {
            return Err("Degenerate offset profile".into());
        }
        let mut out_pts = clean;
        if area < 0.0 {
            out_pts.reverse();
        }

        let profile_pts: Vec<[f64; 2]> = out_pts.into_iter().map(|p| [p.x, p.y]).collect();
        self.store_locus_profile(profile_pts)
    }

    fn locus_boolean(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["a", "b", "op"])?;
        let op_str = text(required(a, "op")?)?;
        let boolean_op = match op_str {
            "union" => cg::dcel_overlay::BooleanOp::Union,
            "intersection" => cg::dcel_overlay::BooleanOp::Intersection,
            "difference" => cg::dcel_overlay::BooleanOp::Difference,
            "xor" => cg::dcel_overlay::BooleanOp::Xor,
            _ => {
                return Err(
                    "Invalid boolean op: expected 'union', 'intersection', 'difference', or 'xor'"
                        .into(),
                )
            }
        };

        let (na, nb) = {
            let pa = self.profile(required(a, "a")?)?;
            let pb = self.profile(required(a, "b")?)?;
            (pa.len(), pb.len())
        };
        let total_n = na + nb;
        self.charge(total_n * total_n * 2 + 8192, total_n * 512 + 16384)?;

        let raw_a = self.profile(required(a, "a")?)?.to_vec();
        let raw_b = self.profile(required(a, "b")?)?.to_vec();

        let mut pts_a = to_clean_points(&raw_a)?;
        let mut pts_b = to_clean_points(&raw_b)?;

        // Ensure CCW orientation for inputs
        if cg::boolean_2::polygon_signed_area(&pts_a) < 0.0 {
            pts_a.reverse();
        }
        if cg::boolean_2::polygon_signed_area(&pts_b) < 0.0 {
            pts_b.reverse();
        }

        let res = cg::dcel_overlay::overlay_boolean(&pts_a, &pts_b, boolean_op)
            .map_err(|e| format!("DCEL boolean overlay error: {e:?}"))?;

        if res.components.is_empty() {
            return Err("Boolean operation produced empty result".into());
        }

        let area = cg::dcel_overlay::total_area(&res.components);
        let loop_count: usize = res.components.iter().map(|c| 1 + c.holes.len()).sum();

        let primary = res
            .components
            .iter()
            .max_by(|x, y| {
                cg::boolean_2::polygon_signed_area(&x.outer)
                    .abs()
                    .partial_cmp(&cg::boolean_2::polygon_signed_area(&y.outer).abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| "Boolean operation produced no valid components".to_string())?;

        let primary_pts: Vec<[f64; 2]> = primary.outer.iter().map(|p| [p.x, p.y]).collect();
        let profile_val = self.store_locus_profile(primary_pts)?;

        let mut record = BTreeMap::new();
        record.insert("profile".into(), profile_val);
        record.insert("area".into(), Value::F64(area));
        record.insert("loop_count".into(), Value::U64(loop_count as u64));
        Ok(Value::Record(record))
    }

    fn locus_stroke_to_locus(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["points", "width", "closed"])?;
        let width = number(a, "width", 1.0)?;
        if !width.is_finite() || width < 0.1 || width > 1e5 {
            return Err("Stroke width outside 0.1..100000 range".into());
        }
        let closed = boolean(a, "closed", false)?;
        let pts = parse_locus_points(required(a, "points")?, if closed { 3 } else { 2 }, 1024)?;
        let n = pts.len();
        self.charge(n * 32 + 2048, n * 256 + 4096)?;

        let out = if closed {
            stroke_closed(&pts, width)?
        } else {
            stroke_open(&pts, width)?
        };

        let profile_pts: Vec<[f64; 2]> = out.into_iter().map(|p| [p.x, p.y]).collect();
        self.store_locus_profile(profile_pts)
    }

    pub(super) fn store_locus_profile(&mut self, mut points: Vec<[f64; 2]>) -> Result<Value, String> {
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
        if area < 0.0 {
            points.reverse();
        }
        self.workspace
            .insert(Artifact::Profile(points), self.limits)
    }
}

fn parse_locus_points(v: &Value, min: usize, max: usize) -> Result<Vec<Vec2>, String> {
    let values = list(v)?;
    if values.len() < min || values.len() > max {
        return Err(format!("Point count outside {min}..{max} bounds"));
    }
    let mut out = Vec::with_capacity(values.len());
    for val in values {
        let [x, y] = vector::<2>(val)?;
        if !x.is_finite() || !y.is_finite() || x.abs() > 1e6 || y.abs() > 1e6 {
            return Err("Coordinate outside finite +/-1000000 range".into());
        }
        out.push(Vec2::new(x, y));
    }
    Ok(out)
}

fn to_clean_points(raw: &[[f64; 2]]) -> Result<Vec<Point2>, String> {
    let mut out: Vec<Point2> = Vec::with_capacity(raw.len());
    for p in raw {
        let pt = Point2::new(p[0], p[1]);
        if let Some(last) = out.last() {
            if (last.x - pt.x).abs() < 1e-9 && (last.y - pt.y).abs() < 1e-9 {
                continue;
            }
        }
        out.push(pt);
    }
    if out.len() > 1 {
        let first = out[0];
        let last = out[out.len() - 1];
        if (first.x - last.x).abs() < 1e-9 && (first.y - last.y).abs() < 1e-9 {
            out.pop();
        }
    }
    if out.len() < 3 {
        return Err("Profile requires at least 3 distinct vertices".into());
    }
    Ok(out)
}

fn offset_polygon(pts: &[Vec2], distance: f64, miter_limit: f64) -> Result<Vec<Vec2>, String> {
    let n = pts.len();
    if n < 3 {
        return Err("Offset requires at least 3 vertices".into());
    }
    let mut out = Vec::with_capacity(n * 2);

    for i in 0..n {
        let prev = pts[(i + n - 1) % n];
        let curr = pts[i];
        let next = pts[(i + 1) % n];

        let d0 = curr - prev;
        let d1 = next - curr;
        let l0 = d0.len();
        let l1 = d1.len();
        if l0 < 1e-12 || l1 < 1e-12 {
            continue;
        }
        let t0 = d0 / l0;
        let t1 = d1 / l1;

        // CCW outward normals
        let n0 = Vec2::new(t0.y, -t0.x);
        let n1 = Vec2::new(t1.y, -t1.x);

        let p0 = curr + n0 * distance;
        let p1 = curr + n1 * distance;

        let det = t0.x * t1.y - t0.y * t1.x;
        let dot = t0.x * t1.x + t0.y * t1.y;

        if det.abs() < 1e-8 {
            if dot > 0.0 {
                out.push(p0);
            } else {
                out.push(p0);
                out.push(p1);
            }
        } else {
            let miter_ratio_sq = 2.0 / (1.0 + dot).max(1e-12);
            if miter_ratio_sq <= miter_limit * miter_limit && (1.0 + dot) > 1e-12 {
                let s = distance * (1.0 - dot) / det;
                let pmiter = p0 + t0 * s;
                out.push(pmiter);
            } else {
                out.push(p0);
                out.push(p1);
            }
        }
    }

    // Clean duplicate consecutive vertices
    let mut clean: Vec<Vec2> = Vec::with_capacity(out.len());
    for p in out {
        if let Some(last) = clean.last() {
            if last.near(p, 1e-9) {
                continue;
            }
        }
        clean.push(p);
    }
    if clean.len() > 1 && clean.first().zip(clean.last()).is_some_and(|(a, b)| a.near(*b, 1e-9)) {
        clean.pop();
    }

    if clean.len() < 3 {
        return Err("Degenerate offset profile".into());
    }

    for p in &clean {
        if !p.x.is_finite() || !p.y.is_finite() || p.x.abs() > 1e6 || p.y.abs() > 1e6 {
            return Err("Offset coordinate outside finite +/-1000000 range".into());
        }
    }

    Ok(clean)
}

fn stroke_open(pts: &[Vec2], width: f64) -> Result<Vec<Vec2>, String> {
    let n = pts.len();
    if n < 2 {
        return Err("Stroke requires at least 2 points".into());
    }
    let h = width * 0.5;
    let mut left_pts = Vec::with_capacity(n * 2);
    let mut right_pts = Vec::with_capacity(n * 2);

    let mut tangents = Vec::with_capacity(n - 1);
    let mut normals = Vec::with_capacity(n - 1);
    for w in pts.windows(2) {
        let d = w[1] - w[0];
        let l = d.len();
        if l < 1e-12 {
            continue;
        }
        let t = d / l;
        let norm = Vec2::new(-t.y, t.x);
        tangents.push(t);
        normals.push(norm);
    }
    if tangents.is_empty() {
        return Err("Degenerate stroke path: all stations coincide".into());
    }

    let num_segs = tangents.len();

    // Start point: butt cap
    left_pts.push(pts[0] + normals[0] * h);
    right_pts.push(pts[0] - normals[0] * h);

    // Internal joins
    for i in 1..num_segs {
        let t0 = tangents[i - 1];
        let t1 = tangents[i];
        let n0 = normals[i - 1];
        let n1 = normals[i];
        let p_station = pts[i];

        let det = t0.x * t1.y - t0.y * t1.x;
        let dot = t0.x * t1.x + t0.y * t1.y;

        if det.abs() < 1e-8 {
            if dot > 0.0 {
                left_pts.push(p_station + n0 * h);
                right_pts.push(p_station - n0 * h);
            } else {
                left_pts.push(p_station + n0 * h);
                left_pts.push(p_station + n1 * h);
                right_pts.push(p_station - n0 * h);
                right_pts.push(p_station - n1 * h);
            }
        } else {
            let miter_sq = 2.0 / (1.0 + dot).max(1e-12);
            if miter_sq <= 16.0 && (1.0 + dot) > 1e-12 {
                let s_l = -h * (1.0 - dot) / det;
                left_pts.push(p_station + n0 * h + t0 * s_l);
                let s_r = h * (1.0 - dot) / det;
                right_pts.push(p_station - n0 * h + t0 * s_r);
            } else {
                left_pts.push(p_station + n0 * h);
                left_pts.push(p_station + n1 * h);
                right_pts.push(p_station - n0 * h);
                right_pts.push(p_station - n1 * h);
            }
        }
    }

    // End point: butt cap
    let last_p = *pts.last().unwrap();
    let last_norm = *normals.last().unwrap();
    left_pts.push(last_p + last_norm * h);
    right_pts.push(last_p - last_norm * h);

    // Assemble loop: left side forward, right side backward
    let mut poly = Vec::with_capacity(left_pts.len() + right_pts.len());
    poly.extend(left_pts);
    for r in right_pts.into_iter().rev() {
        poly.push(r);
    }

    // Dedup consecutive
    let mut clean: Vec<Vec2> = Vec::with_capacity(poly.len());
    for p in poly {
        if let Some(last) = clean.last() {
            if last.near(p, 1e-9) {
                continue;
            }
        }
        clean.push(p);
    }
    if clean.len() > 1 && clean.first().zip(clean.last()).is_some_and(|(a, b)| a.near(*b, 1e-9)) {
        clean.pop();
    }

    if clean.len() < 3 {
        return Err("Degenerate stroked polyline".into());
    }

    let area = drafting_geometry::shoelace(&clean);
    if area.abs() < 1e-12 {
        return Err("Degenerate stroked polyline: zero area".into());
    }
    if area < 0.0 {
        clean.reverse();
    }

    Ok(clean)
}

fn stroke_closed(pts: &[Vec2], width: f64) -> Result<Vec<Vec2>, String> {
    let mut poly = pts.to_vec();
    if poly.len() > 1 && poly.first().zip(poly.last()).is_some_and(|(a, b)| a.near(*b, 1e-9)) {
        poly.pop();
    }
    if poly.len() < 3 {
        return Err("Closed stroke requires at least 3 points".into());
    }
    let area = drafting_geometry::shoelace(&poly);
    if area.abs() < 1e-12 {
        return Err("Degenerate closed contour".into());
    }
    if area < 0.0 {
        poly.reverse();
    }

    let h = width * 0.5;
    let outer = offset_polygon(&poly, h, 4.0)?;
    let inner = offset_polygon(&poly, -h, 4.0)?;

    let area_outer = drafting_geometry::shoelace(&outer);
    let area_inner = drafting_geometry::shoelace(&inner);
    if area_inner <= 1e-12 || area_inner >= area_outer {
        return Err("Stroke width too large for closed contour".into());
    }

    let mut result = Vec::with_capacity(outer.len() + inner.len() + 2);
    result.extend_from_slice(&outer);
    result.push(outer[0]);
    result.push(inner[0]);
    for p in inner.iter().rev() {
        result.push(*p);
    }

    // Clean consecutive duplicates
    let mut clean: Vec<Vec2> = Vec::with_capacity(result.len());
    for p in result {
        if let Some(last) = clean.last() {
            if last.near(p, 1e-9) {
                continue;
            }
        }
        clean.push(p);
    }
    if clean.len() > 1 && clean.first().zip(clean.last()).is_some_and(|(a, b)| a.near(*b, 1e-9)) {
        clean.pop();
    }

    if clean.len() < 3 {
        return Err("Degenerate closed stroked profile".into());
    }

    let total_area = drafting_geometry::shoelace(&clean);
    if total_area <= 1e-12 {
        return Err("Degenerate closed stroked profile: non-positive area".into());
    }

    Ok(clean)
}

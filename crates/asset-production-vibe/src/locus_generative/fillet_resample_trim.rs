//! Corner filleting, arc-length / curvature resampling, and sub-curve trimming on 2D loci.

use crate::*;
use drafting_geometry::Vec2;
use vibe::Value;

/// 300 DPI chord in document points (1/72 in). G2 cubics flatten to this.
const PRINT_CHORD: f64 = 72.0 / 300.0;

fn dist2(a: [f64; 2], b: [f64; 2]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    dx * dx + dy * dy
}

fn cubic_eval(p0: [f64; 2], p1: [f64; 2], p2: [f64; 2], p3: [f64; 2], t: f64) -> [f64; 2] {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;
    [
        uu * u * p0[0] + 3.0 * uu * t * p1[0] + 3.0 * u * tt * p2[0] + tt * t * p3[0],
        uu * u * p0[1] + 3.0 * uu * t * p1[1] + 3.0 * u * tt * p2[1] + tt * t * p3[1],
    ]
}

fn push_cubic(
    out: &mut Vec<[f64; 2]>,
    p0: [f64; 2],
    p1: [f64; 2],
    p2: [f64; 2],
    p3: [f64; 2],
    n: usize,
) {
    let n = n.max(1);
    for k in 0..=n {
        let pt = cubic_eval(p0, p1, p2, p3, k as f64 / n as f64);
        if k == 0 && out.last().is_some_and(|p| dist2(*p, pt) < 1e-18) {
            continue;
        }
        out.push(pt);
    }
}

/// Print-density sample count for a circular sweep; `min_seg` is the origin floor.
fn print_arc_segments(radius: f64, sweep: f64, min_seg: usize) -> usize {
    let chord = PRINT_CHORD.min(radius.abs() * 0.01).max(1e-6);
    drafting_geometry::arc_segments(radius, sweep, chord)
        .max(min_seg)
        .clamp(1, 64)
}

impl AssetHost<'_> {
    /// Corner filleting on a 2D polyline or closed polygon.
    /// Clamps tangent distance d = (radius / tan(half_angle)).min(0.45 * L_in).min(0.45 * L_out)
    /// and replaces sharp corners with G2 cubic (kappa) fillets or linear chamfers.
    pub(super) fn locus_fillet(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "radius", "segments", "closed"])?;
        let radius = number(a, "radius", 0.15)?;
        if !radius.is_finite() || radius < 1e-4 || radius > 1000.0 {
            return Err("Radius outside 1e-4..=1000.0 range".into());
        }
        let segments = count(a, "segments", 6, 64)?;
        let closed = boolean(a, "closed", true)?;

        let raw_pts = self.profile(required(a, "profile")?)?.to_vec();
        if raw_pts.len() < 3 {
            return Err("Degenerate profile handle: fewer than 3 vertices".into());
        }

        let n = raw_pts.len();
        self.charge(n * 64 * 4 + 1024, n * 64 * 64 + 4096)?;

        // Clean consecutive duplicates
        let mut clean_pts: Vec<[f64; 2]> = Vec::with_capacity(n);
        for p in raw_pts {
            if let Some(last) = clean_pts.last() {
                let dx = p[0] - last[0];
                let dy = p[1] - last[1];
                if (dx * dx + dy * dy).sqrt() < 1e-9 {
                    continue;
                }
            }
            clean_pts.push(p);
        }
        if clean_pts.len() > 1 && closed {
            let dx = clean_pts[0][0] - clean_pts.last().unwrap()[0];
            let dy = clean_pts[0][1] - clean_pts.last().unwrap()[1];
            if (dx * dx + dy * dy).sqrt() < 1e-9 {
                clean_pts.pop();
            }
        }
        let m = clean_pts.len();
        if m < 3 {
            return Err("Degenerate profile handle: fewer than 3 distinct vertices".into());
        }

        let mut out_pts: Vec<[f64; 2]> = Vec::with_capacity(m * (segments + 1));

        for i in 0..m {
            if !closed && (i == 0 || i == m - 1) {
                // Endpoints of open polyline are not filleted
                out_pts.push(clean_pts[i]);
                continue;
            }

            let prev_idx = if i == 0 { m - 1 } else { i - 1 };
            let next_idx = if i + 1 == m { 0 } else { i + 1 };

            let p_prev = clean_pts[prev_idx];
            let p_curr = clean_pts[i];
            let p_next = clean_pts[next_idx];

            let vin = [p_curr[0] - p_prev[0], p_curr[1] - p_prev[1]];
            let vout = [p_next[0] - p_curr[0], p_next[1] - p_curr[1]];

            let lin = (vin[0] * vin[0] + vin[1] * vin[1]).sqrt();
            let lout = (vout[0] * vout[0] + vout[1] * vout[1]).sqrt();

            if lin < 1e-9 || lout < 1e-9 {
                out_pts.push(p_curr);
                continue;
            }

            let uin = [vin[0] / lin, vin[1] / lin];
            let uout = [vout[0] / lout, vout[1] / lout];

            // Ray directions radiating away from corner p_curr:
            let a_ray = [-uin[0], -uin[1]];
            let b_ray = [uout[0], uout[1]];

            let dot = (a_ray[0] * b_ray[0] + a_ray[1] * b_ray[1]).clamp(-1.0, 1.0);
            let cross = a_ray[0] * b_ray[1] - a_ray[1] * b_ray[0];
            let theta = cross.abs().atan2(dot);

            if theta < 1e-4 || theta > std::f64::consts::PI - 1e-4 || cross.abs() < 1e-6 {
                // Nearly straight line or spike; emit sharp corner
                out_pts.push(p_curr);
                continue;
            }

            let half_angle = theta * 0.5;
            let tan_half = half_angle.tan();
            if tan_half <= 1e-6 {
                out_pts.push(p_curr);
                continue;
            }

            let d = (radius / tan_half).min(0.45 * lin).min(0.45 * lout);
            let r_eff = d * tan_half;

            let t1 = [p_curr[0] + d * a_ray[0], p_curr[1] + d * a_ray[1]];
            let t2 = [p_curr[0] + d * b_ray[0], p_curr[1] + d * b_ray[1]];

            let bis_vec = [a_ray[0] + b_ray[0], a_ray[1] + b_ray[1]];
            let bis_len = (bis_vec[0] * bis_vec[0] + bis_vec[1] * bis_vec[1]).sqrt();
            if bis_len < 1e-9 {
                out_pts.push(p_curr);
                continue;
            }
            let bis_u = [bis_vec[0] / bis_len, bis_vec[1] / bis_len];
            let dist_center = d / half_angle.cos();
            let center = [
                p_curr[0] + dist_center * bis_u[0],
                p_curr[1] + dist_center * bis_u[1],
            ];

            let w1 = [t1[0] - center[0], t1[1] - center[1]];
            let w2 = [t2[0] - center[0], t2[1] - center[1]];

            let phi1 = w1[1].atan2(w1[0]);
            let phi2 = w2[1].atan2(w2[0]);

            let mut dphi = phi2 - phi1;
            while dphi > std::f64::consts::PI {
                dphi -= std::f64::consts::TAU;
            }
            while dphi < -std::f64::consts::PI {
                dphi += std::f64::consts::TAU;
            }

            if segments == 1 {
                // Ruled/linear chamfer stays a named two-point profile span.
                out_pts.push(t1);
                out_pts.push(t2);
            } else {
                // G2 cubic (4/3 tan(sweep/4) kappa) split at 90°, flattened at print density.
                let splits = ((dphi.abs() / std::f64::consts::FRAC_PI_2).ceil() as usize).clamp(1, 4);
                let dir = if dphi >= 0.0 { 1.0 } else { -1.0 };
                let min_each = (segments / splits).max(2);
                for s in 0..splits {
                    let a0 = phi1 + dphi * (s as f64 / splits as f64);
                    let a1 = phi1 + dphi * ((s + 1) as f64 / splits as f64);
                    let sweep = a1 - a0;
                    let p0 = [center[0] + r_eff * a0.cos(), center[1] + r_eff * a0.sin()];
                    let p3 = [center[0] + r_eff * a1.cos(), center[1] + r_eff * a1.sin()];
                    let tan0 = [-a0.sin() * dir, a0.cos() * dir];
                    let tan1 = [-a1.sin() * dir, a1.cos() * dir];
                    let h = r_eff * (4.0 / 3.0) * (sweep.abs() * 0.25).tan();
                    let c1 = [p0[0] + tan0[0] * h, p0[1] + tan0[1] * h];
                    let c2 = [p3[0] - tan1[0] * h, p3[1] - tan1[1] * h];
                    let n_tess = print_arc_segments(r_eff, sweep.abs(), min_each);
                    push_cubic(&mut out_pts, p0, c1, c2, p3, n_tess);
                }
            }
        }

        // Deduplicate adjacent points
        let mut final_pts: Vec<[f64; 2]> = Vec::with_capacity(out_pts.len());
        for p in out_pts {
            if let Some(last) = final_pts.last() {
                let dx = p[0] - last[0];
                let dy = p[1] - last[1];
                if (dx * dx + dy * dy).sqrt() < 1e-9 {
                    continue;
                }
            }
            final_pts.push(p);
        }
        if final_pts.len() > 1 && closed {
            let dx = final_pts[0][0] - final_pts.last().unwrap()[0];
            let dy = final_pts[0][1] - final_pts.last().unwrap()[1];
            if (dx * dx + dy * dy).sqrt() < 1e-9 {
                final_pts.pop();
            }
        }

        if final_pts.len() < 3 {
            return Err("Filleted locus resulted in fewer than 3 vertices".into());
        }

        if closed {
            let pts_vec: Vec<Vec2> = final_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();
            let area = drafting_geometry::shoelace(&pts_vec);
            if area.abs() < 1e-12 {
                return Err("Degenerate filleted profile: zero area".into());
            }
            if area < 0.0 {
                final_pts.reverse();
            }
        }

        self.workspace
            .insert(Artifact::Profile(final_pts), self.limits)
    }

    /// Resample a 2D locus by uniform arc length count, fixed step length, or curvature weighting.
    pub(super) fn locus_resample(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "mode", "count", "step_length", "closed"])?;
        let mode = a.get("mode").map(text).transpose()?.unwrap_or("count");
        let count_val = count(a, "count", 32, 4096)?;
        let step_length = number(a, "step_length", 0.1)?;
        if !step_length.is_finite() || step_length < 1e-4 || step_length > 1000.0 {
            return Err("step_length outside 1e-4..=1000.0 range".into());
        }
        let closed = boolean(a, "closed", true)?;

        let raw_pts = self.profile(required(a, "profile")?)?.to_vec();
        if raw_pts.len() < 3 {
            return Err("Degenerate profile handle: fewer than 3 vertices".into());
        }

        // Clean duplicates
        let mut clean_pts: Vec<[f64; 2]> = Vec::with_capacity(raw_pts.len());
        for p in raw_pts {
            if let Some(last) = clean_pts.last() {
                let dx = p[0] - last[0];
                let dy = p[1] - last[1];
                if (dx * dx + dy * dy).sqrt() < 1e-9 {
                    continue;
                }
            }
            clean_pts.push(p);
        }
        if clean_pts.len() > 1 && closed {
            let dx = clean_pts[0][0] - clean_pts.last().unwrap()[0];
            let dy = clean_pts[0][1] - clean_pts.last().unwrap()[1];
            if (dx * dx + dy * dy).sqrt() < 1e-9 {
                clean_pts.pop();
            }
        }
        let m = clean_pts.len();
        if m < 3 {
            return Err("Degenerate profile handle: fewer than 3 distinct vertices".into());
        }

        let seg_count = if closed { m } else { m - 1 };
        let mut seg_lengths = Vec::with_capacity(seg_count);
        let mut cum_s = Vec::with_capacity(seg_count + 1);
        cum_s.push(0.0);
        let mut total_len = 0.0;

        for j in 0..seg_count {
            let p0 = clean_pts[j];
            let p1 = clean_pts[(j + 1) % m];
            let dx = p1[0] - p0[0];
            let dy = p1[1] - p0[1];
            let l = (dx * dx + dy * dy).sqrt();
            seg_lengths.push(l);
            total_len += l;
            cum_s.push(total_len);
        }

        if total_len < 1e-12 {
            return Err("Degenerate profile handle: total arc length near zero".into());
        }

        let sample_at_s = |s: f64| -> [f64; 2] {
            let s_clamped = s.clamp(0.0, total_len);
            let idx = match cum_s.binary_search_by(|val| {
                val.partial_cmp(&s_clamped).unwrap_or(std::cmp::Ordering::Equal)
            }) {
                Ok(i) => i.min(seg_count.saturating_sub(1)),
                Err(i) => (i.saturating_sub(1)).min(seg_count.saturating_sub(1)),
            };
            let seg_len = seg_lengths[idx];
            if seg_len < 1e-12 {
                clean_pts[idx]
            } else {
                let t = ((s_clamped - cum_s[idx]) / seg_len).clamp(0.0, 1.0);
                let p0 = clean_pts[idx];
                let p1 = clean_pts[(idx + 1) % m];
                [p0[0] + t * (p1[0] - p0[0]), p0[1] + t * (p1[1] - p0[1])]
            }
        };

        let target_count = match mode {
            "count" => count_val,
            "length" => {
                let n_pts = (total_len / step_length).round() as usize;
                n_pts.clamp(3, 4096)
            }
            "curvature" => count_val,
            _ => {
                return Err(format!(
                    "Unknown resample mode: {mode}; expected 'count', 'length', or 'curvature'"
                ))
            }
        };

        self.charge(m * 16 + target_count * 8 + 1024, (m + target_count) * 64 + 4096)?;

        let mut out_pts = Vec::with_capacity(target_count);

        if mode == "curvature" {
            // Turning angle at each vertex
            let mut turning_angles = vec![0.0f64; m];
            for j in 0..m {
                if !closed && (j == 0 || j == m - 1) {
                    turning_angles[j] = 0.0;
                    continue;
                }
                let prev_j = if j == 0 { m - 1 } else { j - 1 };
                let next_j = if j + 1 == m { 0 } else { j + 1 };
                let p_prev = clean_pts[prev_j];
                let p_curr = clean_pts[j];
                let p_next = clean_pts[next_j];

                let vin = [p_curr[0] - p_prev[0], p_curr[1] - p_prev[1]];
                let vout = [p_next[0] - p_curr[0], p_next[1] - p_curr[1]];
                let lin = (vin[0] * vin[0] + vin[1] * vin[1]).sqrt();
                let lout = (vout[0] * vout[0] + vout[1] * vout[1]).sqrt();
                if lin > 1e-9 && lout > 1e-9 {
                    let dot = (vin[0] * vout[0] + vin[1] * vout[1]) / (lin * lout);
                    turning_angles[j] = dot.clamp(-1.0, 1.0).acos();
                }
            }

            // Weighted density across segments
            let mut seg_weights = Vec::with_capacity(seg_count);
            let mut cum_w = Vec::with_capacity(seg_count + 1);
            cum_w.push(0.0);
            let mut total_w = 0.0;

            for j in 0..seg_count {
                let kappa = (turning_angles[j] + turning_angles[(j + 1) % m]) / std::f64::consts::PI;
                let w = seg_lengths[j] * (1.0 + 3.0 * kappa);
                seg_weights.push(w);
                total_w += w;
                cum_w.push(total_w);
            }

            let denom = if closed {
                target_count as f64
            } else {
                (target_count - 1).max(1) as f64
            };

            for k in 0..target_count {
                let target_w = (k as f64 * total_w / denom).clamp(0.0, total_w);
                let idx = match cum_w.binary_search_by(|val| {
                    val.partial_cmp(&target_w).unwrap_or(std::cmp::Ordering::Equal)
                }) {
                    Ok(i) => i.min(seg_count.saturating_sub(1)),
                    Err(i) => (i.saturating_sub(1)).min(seg_count.saturating_sub(1)),
                };
                let w_span = seg_weights[idx];
                let frac = if w_span > 1e-12 {
                    ((target_w - cum_w[idx]) / w_span).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                let s = cum_s[idx] + frac * seg_lengths[idx];
                out_pts.push(sample_at_s(s));
            }
        } else {
            // Uniform arc length ("count" or "length")
            let denom = if closed {
                target_count as f64
            } else {
                (target_count - 1).max(1) as f64
            };

            for k in 0..target_count {
                let s = (k as f64 * total_len / denom).clamp(0.0, total_len);
                out_pts.push(sample_at_s(s));
            }
        }

        // Clean duplicates
        let mut final_pts: Vec<[f64; 2]> = Vec::with_capacity(out_pts.len());
        for p in out_pts {
            if let Some(last) = final_pts.last() {
                let dx = p[0] - last[0];
                let dy = p[1] - last[1];
                if (dx * dx + dy * dy).sqrt() < 1e-9 {
                    continue;
                }
            }
            final_pts.push(p);
        }
        if final_pts.len() > 1 && closed {
            let dx = final_pts[0][0] - final_pts.last().unwrap()[0];
            let dy = final_pts[0][1] - final_pts.last().unwrap()[1];
            if (dx * dx + dy * dy).sqrt() < 1e-9 {
                final_pts.pop();
            }
        }

        if final_pts.len() < 3 {
            return Err("Resampled profile resulted in fewer than 3 vertices".into());
        }

        if closed {
            let pts_vec: Vec<Vec2> = final_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();
            let area = drafting_geometry::shoelace(&pts_vec);
            if area.abs() < 1e-12 {
                return Err("Degenerate resampled profile: zero area".into());
            }
            if area < 0.0 {
                final_pts.reverse();
            }
        }

        self.workspace
            .insert(Artifact::Profile(final_pts), self.limits)
    }

    /// Extract a sub-curve from normalized arc length [start, end] with exact linear interpolation.
    pub(super) fn locus_trim(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "start", "end", "close_chord"])?;
        let start = number(a, "start", 0.0)?;
        let end = number(a, "end", 0.5)?;
        let close_chord = boolean(a, "close_chord", true)?;

        if !start.is_finite() || !end.is_finite() || start < 0.0 || end > 1.0 || start >= end {
            return Err(
                "Invalid trim range: start and end must be in 0.0..=1.0 with end > start".into(),
            );
        }

        let raw_pts = self.profile(required(a, "profile")?)?.to_vec();
        if raw_pts.len() < 3 {
            return Err("Degenerate profile handle: fewer than 3 vertices".into());
        }

        // Clean duplicates
        let mut clean_pts: Vec<[f64; 2]> = Vec::with_capacity(raw_pts.len());
        for p in raw_pts {
            if let Some(last) = clean_pts.last() {
                let dx = p[0] - last[0];
                let dy = p[1] - last[1];
                if (dx * dx + dy * dy).sqrt() < 1e-9 {
                    continue;
                }
            }
            clean_pts.push(p);
        }
        let dx = clean_pts[0][0] - clean_pts.last().unwrap()[0];
        let dy = clean_pts[0][1] - clean_pts.last().unwrap()[1];
        if (dx * dx + dy * dy).sqrt() < 1e-9 {
            clean_pts.pop();
        }
        let m = clean_pts.len();
        if m < 3 {
            return Err("Degenerate profile handle: fewer than 3 distinct vertices".into());
        }

        self.charge(m * 8 + 512, m * 32 + 2048)?;

        // Cumulative arc length around closed loop
        let mut seg_lengths = Vec::with_capacity(m);
        let mut cum_s = Vec::with_capacity(m + 1);
        cum_s.push(0.0);
        let mut total_len = 0.0;

        for j in 0..m {
            let p0 = clean_pts[j];
            let p1 = clean_pts[(j + 1) % m];
            let seg_dx = p1[0] - p0[0];
            let seg_dy = p1[1] - p0[1];
            let l = (seg_dx * seg_dx + seg_dy * seg_dy).sqrt();
            seg_lengths.push(l);
            total_len += l;
            cum_s.push(total_len);
        }

        if total_len < 1e-12 {
            return Err("Degenerate profile handle: total arc length near zero".into());
        }

        let sample_at_s = |s: f64| -> [f64; 2] {
            let s_clamped = s.clamp(0.0, total_len);
            let idx = match cum_s.binary_search_by(|val| {
                val.partial_cmp(&s_clamped).unwrap_or(std::cmp::Ordering::Equal)
            }) {
                Ok(i) => i.min(m.saturating_sub(1)),
                Err(i) => (i.saturating_sub(1)).min(m.saturating_sub(1)),
            };
            let seg_len = seg_lengths[idx];
            if seg_len < 1e-12 {
                clean_pts[idx]
            } else {
                let t = ((s_clamped - cum_s[idx]) / seg_len).clamp(0.0, 1.0);
                let p0 = clean_pts[idx];
                let p1 = clean_pts[(idx + 1) % m];
                [p0[0] + t * (p1[0] - p0[0]), p0[1] + t * (p1[1] - p0[1])]
            }
        };

        let s_start = start * total_len;
        let s_end = end * total_len;

        let p_start = sample_at_s(s_start);
        let p_end = sample_at_s(s_end);

        let mut out_pts = Vec::with_capacity(m + 2);
        out_pts.push(p_start);

        for j in 0..=m {
            let sj = cum_s[j];
            if sj > s_start + 1e-7 && sj < s_end - 1e-7 {
                out_pts.push(clean_pts[j % m]);
            }
        }
        out_pts.push(p_end);

        // Deduplicate adjacent
        let mut final_pts: Vec<[f64; 2]> = Vec::with_capacity(out_pts.len());
        for p in out_pts {
            if let Some(last) = final_pts.last() {
                let d_x = p[0] - last[0];
                let d_y = p[1] - last[1];
                if (d_x * d_x + d_y * d_y).sqrt() < 1e-9 {
                    continue;
                }
            }
            final_pts.push(p);
        }

        // Ensure at least 3 points so downstream extrude / render ops work seamlessly
        if final_pts.len() == 2 {
            let a_pt = final_pts[0];
            let b_pt = final_pts[1];
            let chord_dx = b_pt[0] - a_pt[0];
            let chord_dy = b_pt[1] - a_pt[1];
            let chord_len = (chord_dx * chord_dx + chord_dy * chord_dy).sqrt();
            let (nx, ny) = if chord_len > 1e-9 {
                (-chord_dy / chord_len, chord_dx / chord_len)
            } else {
                (0.0, 1.0)
            };
            let offset = (chord_len * 0.05).max(0.001);
            let mid = [
                (a_pt[0] + b_pt[0]) * 0.5 + nx * offset,
                (a_pt[1] + b_pt[1]) * 0.5 + ny * offset,
            ];
            final_pts.insert(1, mid);
        }

        if close_chord {
            let pts_vec: Vec<Vec2> = final_pts.iter().map(|p| Vec2::new(p[0], p[1])).collect();
            let area = drafting_geometry::shoelace(&pts_vec);
            if area.abs() < 1e-12 {
                let mid_i = final_pts.len() / 2;
                final_pts[mid_i][1] += 0.001;
            } else if area < 0.0 {
                final_pts.reverse();
            }
        }

        self.workspace
            .insert(Artifact::Profile(final_pts), self.limits)
    }
}

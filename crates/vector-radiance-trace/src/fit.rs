//! Loop -> polygon -> Bézier curves.
//!
//! Includes RDP polyline simplification, staircase dejagging, Schneider least-squares
//! cubic Bézier fitting, and curve straightening.

use kurbo::{CubicBez, ParamCurve, ParamCurveArclen, ParamCurveDeriv, Point, Vec2};
use vector_locus_geometry::{Anchor, AnchorKind, PathData, SubPath};

use crate::contour::Loop;

#[derive(Clone, Copy, Debug)]
pub(crate) struct FitOptions {
    pub polygon_tol: f64,
    pub fit_tol: f64,
    pub corner_angle: f64,
    pub snap_lines: bool,
}

fn seg_dist(p: Point, a: Point, b: Point) -> f64 {
    let ab = b - a;
    let l2 = ab.hypot2();
    if l2 < 1e-18 {
        return p.distance(a);
    }
    let t = ((p - a).dot(ab) / l2).clamp(0.0, 1.0);
    p.distance(a + ab * t)
}

/// Douglas-Peucker on `pts[a..=b]`, marking kept indices.
fn rdp_slice(pts: &[Point], a: usize, b: usize, tol: f64, keep: &mut [bool]) {
    let mut stack = vec![(a, b)];
    while let Some((a, b)) = stack.pop() {
        if b <= a + 1 {
            continue;
        }
        let (pa, pb) = (pts[a], pts[b % pts.len()]);
        let mut best = (0.0, a);
        for (i, &q) in pts.iter().enumerate().take(b).skip(a + 1) {
            let d = seg_dist(q, pa, pb);
            if d > best.0 {
                best = (d, i);
            }
        }
        if best.0 > tol {
            keep[best.1] = true;
            stack.push((a, best.1));
            stack.push((best.1, b));
        }
    }
}

/// The polygon approximating a closed loop of pixel-corner vertices within `tol`.
pub(crate) fn polygon(loop_pts: &[(i32, i32)], tol: f64) -> Vec<Point> {
    let raw: Vec<Point> = loop_pts.iter().map(|&(x, y)| Point::new(x as f64, y as f64)).collect();
    if raw.len() < 4 {
        return raw;
    }
    let pts = dejag(loop_pts);
    let pts = if pts.len() < 3 { raw.clone() } else { pts };
    let n = pts.len();
    if n < 4 {
        return pts;
    }
    let far = (1..n)
        .max_by(|&i, &j| pts[i].distance_squared(pts[0]).total_cmp(&pts[j].distance_squared(pts[0])))
        .unwrap_or(n / 2);
    let mut ext = pts.clone();
    ext.push(pts[0]);
    let mut keep = vec![false; n + 1];
    keep[0] = true;
    keep[far] = true;
    rdp_slice(&ext, 0, far, tol, &mut keep);
    rdp_slice(&ext, far, n, tol, &mut keep);
    let mut out: Vec<Point> = (0..n).filter(|&i| keep[i]).map(|i| pts[i]).collect();
    if out.len() > 3 {
        let (p, a, b) = (out[0], out[out.len() - 1], out[1]);
        if seg_dist(p, a, b) <= tol * 0.5 {
            out.remove(0);
        }
    }
    if out.len() < 3 { raw } else { out }
}

/// Replace pixel-staircase jogs by their midpoints.
fn dejag(v: &[(i32, i32)]) -> Vec<Point> {
    let n = v.len();
    let edge = |i: usize| {
        let (a, b) = (v[i % n], v[(i + 1) % n]);
        (b.0 - a.0, b.1 - a.1)
    };
    let turn = |i: usize| {
        let (a, b) = (edge((i + n - 1) % n), edge(i));
        (a.0 as i64 * b.1 as i64 - a.1 as i64 * b.0 as i64).signum()
    };
    let jog: Vec<bool> = (0..n)
        .map(|i| {
            let (dx, dy) = edge(i);
            dx.abs() + dy.abs() == 1 && turn(i) * turn(i + 1) < 0
        })
        .collect();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        if !jog[i] && !jog[(i + n - 1) % n] {
            out.push(Point::new(v[i].0 as f64, v[i].1 as f64));
        }
        if jog[i] {
            let (a, b) = (v[i], v[(i + 1) % n]);
            out.push(Point::new((a.0 + b.0) as f64 / 2.0, (a.1 + b.1) as f64 / 2.0));
        }
    }
    out
}

/// Fit one loop into a closed Bézier subpath (pixel coordinates).
pub(crate) fn fit_loop(l: &Loop, o: &FitOptions) -> Option<SubPath> {
    let poly = polygon(&l.pts, o.polygon_tol);
    if poly.len() < 3 {
        return None;
    }
    let sp = SubPath::polyline(&poly, true);
    let fitted = simplify_with(
        &PathData::single(sp),
        &SimplifyOptions {
            tolerance: o.fit_tol,
            corner_angle_deg: o.corner_angle,
            straight_lines: false,
        },
    );
    let mut sp = fitted.subpaths.into_iter().next()?;
    if sp.anchors.len() < 2 {
        return None;
    }
    if o.snap_lines {
        snap_to_lines(&mut sp, (o.fit_tol * 2.0).max(1.5));
    }
    Some(sp)
}

/// Fit a centre line through `pts` into a Bézier subpath.
pub(crate) fn fit_polyline(pts: &[Point], closed: bool, o: &FitOptions) -> Option<SubPath> {
    let n = pts.len();
    let first = *pts.first()?;
    if n < 2 {
        return None;
    }
    let mut ext = pts.to_vec();
    let mut keep = vec![false; n + 1];
    keep[0] = true;
    if closed {
        ext.push(first);
        let far = (1..n)
            .max_by(|&i, &j| pts[i].distance_squared(first).total_cmp(&pts[j].distance_squared(first)))
            .unwrap_or(n / 2);
        keep[far] = true;
        rdp_slice(&ext, 0, far, o.polygon_tol, &mut keep);
        rdp_slice(&ext, far, n, o.polygon_tol, &mut keep);
    } else {
        keep[n - 1] = true;
        rdp_slice(&ext, 0, n - 1, o.polygon_tol, &mut keep);
    }
    let mut poly: Vec<Point> = (0..n).filter(|&i| keep[i]).map(|i| pts[i]).collect();
    if closed && poly.len() < 3 {
        poly = pts.to_vec();
    }
    if poly.len() < if closed { 3 } else { 2 } {
        return None;
    }
    let sp = SubPath::polyline(&poly, closed);
    let fitted = simplify_with(
        &PathData::single(sp),
        &SimplifyOptions {
            tolerance: o.fit_tol,
            corner_angle_deg: o.corner_angle,
            straight_lines: false,
        },
    );
    let mut sp = fitted.subpaths.into_iter().next()?;
    if sp.anchors.len() < 2 {
        return None;
    }
    if o.snap_lines {
        snap_to_lines(&mut sp, (o.fit_tol * 2.0).max(1.5));
    }
    Some(sp)
}

/// Retract the handles of curves that stay within `tol` of their chord.
fn snap_to_lines(sp: &mut SubPath, tol: f64) {
    let n = sp.anchors.len();
    for i in 0..sp.segment_count() {
        let j = (i + 1) % n;
        let (a, b) = (sp.anchors[i], sp.anchors[j]);
        if !a.has_out() && !b.has_in() {
            continue;
        }
        let c = CubicBez::new(a.p, a.h_out, b.h_in, b.p);
        if (1..8).all(|k| seg_dist(c.eval(k as f64 / 8.0), a.p, b.p) <= tol) {
            sp.anchors[i].h_out = a.p;
            sp.anchors[j].h_in = b.p;
        }
    }
    for a in &mut sp.anchors {
        if !a.has_in() || !a.has_out() {
            a.kind = AnchorKind::Corner;
        }
    }
}

// ---------------------------------------------------------------------------
// Curve fitting and simplification routines
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SimplifyOptions {
    pub tolerance: f64,
    pub corner_angle_deg: f64,
    pub straight_lines: bool,
}

impl Default for SimplifyOptions {
    fn default() -> Self {
        Self {
            tolerance: 0.5,
            corner_angle_deg: 30.0,
            straight_lines: false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Seg {
    pub(crate) c: CubicBez,
    pub(crate) line: bool,
}

impl Seg {
    pub(crate) fn line(a: Point, b: Point) -> Self {
        Self {
            c: CubicBez::new(a, a, b, b),
            line: true,
        }
    }
}

pub(crate) fn segs_to_subpath(segs: &[Seg], closed: bool) -> Option<SubPath> {
    if segs.is_empty() {
        return None;
    }
    let n = segs.len();
    let mut anchors = Vec::with_capacity(n + 1);
    for i in 0..n {
        let s = segs[i];
        let p = s.c.p0;
        let h_out = if s.line { p } else { s.c.p1 };
        let h_in = if i == 0 && !closed {
            p
        } else {
            let prev = segs[(i + n - 1) % n];
            if prev.line { p } else { prev.c.p2 }
        };
        anchors.push(Anchor::with_handles(p, h_in, h_out));
    }
    if !closed {
        let last = segs[n - 1];
        let p = last.c.p3;
        let h_in = if last.line { p } else { last.c.p2 };
        anchors.push(Anchor::with_handles(p, h_in, p));
    }
    if closed && anchors.len() < 2 {
        return None;
    }
    Some(SubPath::new(anchors, closed))
}

fn turn_deg(a: &Seg, b: &Seg) -> f64 {
    let d = end_tangent(&a.c).dot(start_tangent(&b.c)).clamp(-1.0, 1.0);
    d.acos().to_degrees()
}

fn no_worse(original: &SubPath, fitted: SubPath) -> SubPath {
    if fitted.anchors.len() > original.anchors.len() {
        original.clone()
    } else {
        fitted
    }
}

fn subpath_segs(sp: &SubPath) -> Vec<Seg> {
    (0..sp.segment_count())
        .map(|i| Seg {
            c: sp.segment(i),
            line: sp.segment_is_line(i),
        })
        .collect()
}

pub fn simplify_with(path: &PathData, opts: &SimplifyOptions) -> PathData {
    let tol = opts.tolerance.max(1e-6);
    let mut out = Vec::new();
    for sp in &path.subpaths {
        let mut segs = subpath_segs(sp);
        let n = segs.len();
        if n == 0 {
            out.push(sp.clone());
            continue;
        }
        let corner_at = |segs: &[Seg], i: usize| -> bool {
            if !sp.closed && (i == 0 || i == n) {
                return true;
            }
            turn_deg(&segs[(i + n - 1) % n], &segs[i % n]) > opts.corner_angle_deg
        };
        let mut corners: Vec<usize> = (0..n).filter(|&i| corner_at(&segs, i)).collect();
        if sp.closed {
            if let Some(&k) = corners.first() {
                segs.rotate_left(k);
                corners = corners.iter().map(|&c| c - k).collect();
            } else {
                corners = vec![0];
            }
            corners.push(n);
        } else if corners.last() != Some(&n) {
            corners.push(n);
        }
        let mut result: Vec<Seg> = Vec::new();
        for w in corners.windows(2) {
            let run = &segs[w[0]..w[1]];
            if run.is_empty() {
                continue;
            }
            let mut pts = Vec::new();
            for (k, s) in run.iter().enumerate() {
                let len = s.c.arclen(1e-6);
                let m = ((len / (tol * 0.5)).ceil() as usize).clamp(4, 256);
                sample(&s.c, m, &mut pts, k == 0);
            }
            if opts.straight_lines {
                let keep = rdp_points(&pts, tol);
                result.extend(keep.windows(2).map(|p| Seg::line(p[0], p[1])));
                continue;
            }
            let t0 = start_tangent(&run[0].c);
            let t1 = end_tangent(&run[run.len() - 1].c);
            for c in fit_cubics(&pts, t0, t1, tol) {
                let line = is_straight(&c, tol * 0.02);
                result.push(if line {
                    Seg::line(c.p0, c.p3)
                } else {
                    Seg { c, line }
                });
            }
        }
        if let Some(s) = segs_to_subpath(&result, sp.closed) {
            out.push(if opts.straight_lines { s } else { no_worse(sp, s) });
        }
    }
    PathData::new(out)
}

fn rdp_points(pts: &[Point], tol: f64) -> Vec<Point> {
    if pts.len() < 3 {
        return pts.to_vec();
    }
    let mut keep = vec![false; pts.len()];
    keep[0] = true;
    keep[pts.len() - 1] = true;
    let mut stack = vec![(0, pts.len() - 1)];
    while let Some((a, b)) = stack.pop() {
        let (pa, pb) = (pts[a], pts[b]);
        let chord = pb - pa;
        let len = chord.hypot();
        let mut best = (0.0, a);
        for (i, &p) in pts.iter().enumerate().take(b).skip(a + 1) {
            let d = if len < 1e-12 {
                p.distance(pa)
            } else {
                (p - pa).cross(chord).abs() / len
            };
            if d > best.0 {
                best = (d, i);
            }
        }
        if best.0 > tol {
            keep[best.1] = true;
            stack.push((a, best.1));
            stack.push((best.1, b));
        }
    }
    pts.iter().zip(keep).filter(|(_, k)| *k).map(|(p, _)| *p).collect()
}

// ---------------------------------------------------------------------------
// Philip J. Schneider cubic curve fitting implementation
// ---------------------------------------------------------------------------

const MAX_REPARAM: usize = 6;

pub(crate) fn unit(v: Vec2) -> Option<Vec2> {
    let l = v.hypot();
    if l > 1e-12 && l.is_finite() {
        Some(v / l)
    } else {
        None
    }
}

pub(crate) fn start_tangent(c: &CubicBez) -> Vec2 {
    unit(c.p1 - c.p0)
        .or_else(|| unit(c.p2 - c.p0))
        .or_else(|| unit(c.p3 - c.p0))
        .unwrap_or(Vec2::new(1.0, 0.0))
}

pub(crate) fn end_tangent(c: &CubicBez) -> Vec2 {
    unit(c.p3 - c.p2)
        .or_else(|| unit(c.p3 - c.p1))
        .or_else(|| unit(c.p3 - c.p0))
        .unwrap_or(Vec2::new(1.0, 0.0))
}

pub(crate) fn fit_cubics(pts: &[Point], t0: Vec2, t1: Vec2, tol: f64) -> Vec<CubicBez> {
    let mut out = Vec::new();
    let pts = dedup(pts);
    if pts.len() < 2 {
        return out;
    }
    fit_rec(&pts, t0, t1, tol.max(1e-9), &mut out, 0);
    out
}

fn dedup(pts: &[Point]) -> Vec<Point> {
    let mut v: Vec<Point> = Vec::with_capacity(pts.len());
    for &p in pts {
        if v.last().is_none_or(|q: &Point| q.distance(p) > 1e-12) {
            v.push(p);
        }
    }
    v
}

fn fit_rec(pts: &[Point], t0: Vec2, t1: Vec2, tol: f64, out: &mut Vec<CubicBez>, depth: usize) {
    let n = pts.len();
    if n == 2 {
        let d = pts[0].distance(pts[1]) / 3.0;
        out.push(CubicBez::new(pts[0], pts[0] + t0 * d, pts[1] - t1 * d, pts[1]));
        return;
    }
    let (c, err, split) = fit_single(pts, t0, t1);
    if err <= tol || depth > 40 {
        out.push(c);
        return;
    }
    let split = split.clamp(1, n - 2);
    let tc = unit(pts[split + 1] - pts[split - 1]).unwrap_or(t0);
    fit_rec(&pts[..=split], t0, tc, tol, out, depth + 1);
    fit_rec(&pts[split..], tc, t1, tol, out, depth + 1);
}

pub(crate) fn fit_single(pts: &[Point], t0: Vec2, t1: Vec2) -> (CubicBez, f64, usize) {
    fit_single_from(pts, chord_params(pts), t0, t1)
}

pub(crate) fn fit_single_from(
    pts: &[Point],
    mut u: Vec<f64>,
    t0: Vec2,
    t1: Vec2,
) -> (CubicBez, f64, usize) {
    let mut best = generate(pts, &u, t0, t1);
    let (mut best_err, mut best_split) = max_error(pts, &best, &u);
    for _ in 0..MAX_REPARAM {
        for (i, ui) in u.iter_mut().enumerate() {
            *ui = newton(&best, pts[i], *ui);
        }
        let c = generate(pts, &u, t0, t1);
        let (e, s) = max_error(pts, &c, &u);
        if e < best_err {
            best = c;
            best_err = e;
            best_split = s;
        } else {
            break;
        }
    }
    (best, best_err, best_split)
}

fn chord_params(pts: &[Point]) -> Vec<f64> {
    let mut u = Vec::with_capacity(pts.len());
    let mut acc = 0.0;
    u.push(0.0);
    for w in pts.windows(2) {
        acc += w[0].distance(w[1]);
        u.push(acc);
    }
    if acc > 0.0 {
        for x in &mut u {
            *x /= acc;
        }
    }
    u
}

fn bern(t: f64) -> [f64; 4] {
    let s = 1.0 - t;
    [s * s * s, 3.0 * s * s * t, 3.0 * s * t * t, t * t * t]
}

fn generate(pts: &[Point], u: &[f64], t0: Vec2, t1: Vec2) -> CubicBez {
    let (Some(&p0), Some(&p3)) = (pts.first(), pts.last()) else {
        return CubicBez::new(Point::ZERO, Point::ZERO, Point::ZERO, Point::ZERO);
    };
    let dir_in = -t1;
    let (mut c00, mut c01, mut c11, mut x0, mut x1) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for (p, &t) in pts.iter().zip(u) {
        let b = bern(t);
        let a0 = t0 * b[1];
        let a1 = dir_in * b[2];
        c00 += a0.dot(a0);
        c01 += a0.dot(a1);
        c11 += a1.dot(a1);
        let tmp = *p - (p0.to_vec2() * (b[0] + b[1]) + p3.to_vec2() * (b[2] + b[3])).to_point();
        x0 += a0.dot(tmp);
        x1 += a1.dot(tmp);
    }
    let det = c00 * c11 - c01 * c01;
    let seg = p0.distance(p3);
    let eps = 1e-6 * seg;
    let (mut al, mut ar) = if det.abs() > 1e-12 {
        ((x0 * c11 - x1 * c01) / det, (c00 * x1 - c01 * x0) / det)
    } else {
        (seg / 3.0, seg / 3.0)
    };
    if !al.is_finite() || !ar.is_finite() || al < eps || ar < eps {
        al = seg / 3.0;
        ar = seg / 3.0;
    }
    let cap = seg * 4.0 + 1e-9;
    al = al.min(cap);
    ar = ar.min(cap);
    CubicBez::new(p0, p0 + t0 * al, p3 + dir_in * ar, p3)
}

fn max_error(pts: &[Point], c: &CubicBez, u: &[f64]) -> (f64, usize) {
    let mut e = 0.0;
    let mut idx = pts.len() / 2;
    for (i, (p, &t)) in pts.iter().zip(u).enumerate() {
        let d = c.eval(t).distance(*p);
        if d > e {
            e = d;
            idx = i;
        }
    }
    (e, idx)
}

fn newton(c: &CubicBez, p: Point, t: f64) -> f64 {
    let d1 = c.deriv();
    let d2 = d1.deriv();
    let q = c.eval(t) - p;
    let q1 = d1.eval(t).to_vec2();
    let q2 = d2.eval(t).to_vec2();
    let num = q.dot(q1);
    let den = q1.dot(q1) + q.dot(q2);
    if den.abs() < 1e-12 {
        return t;
    }
    let nt = t - num / den;
    if nt.is_finite() {
        nt.clamp(0.0, 1.0)
    } else {
        t
    }
}

pub(crate) fn is_straight(c: &CubicBez, tol: f64) -> bool {
    let chord = c.p3 - c.p0;
    let len = chord.hypot();
    if len < 1e-12 {
        return c.p1.distance(c.p0) <= tol && c.p2.distance(c.p0) <= tol;
    }
    let dir = chord / len;
    [c.p1, c.p2].iter().all(|&h| {
        let v = h - c.p0;
        let along = v.dot(dir);
        v.cross(dir).abs() <= tol && along >= -tol && along <= len + tol
    })
}

pub(crate) fn sample(c: &CubicBez, n: usize, out: &mut Vec<Point>, include_start: bool) {
    let start = if include_start { 0 } else { 1 };
    for i in start..=n {
        out.push(c.eval(i as f64 / n as f64));
    }
}

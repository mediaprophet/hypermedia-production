//! Curve-preserving boolean operations on editable vector paths.
//!
//! Grounded in authoring rights and creative expression (UDHR Art. 19, 27).
//!
//! Evaluates binary and N-ary boolean operations (Union, Intersect, Difference, Xor)
//! over filled vector paths with curve refitting.

use kurbo::{BezPath, CubicBez, ParamCurve, PathEl, Point, Rect, Shape as _, Vec2};
use vector_locus_geometry::{Anchor, FillRule, PathData, SubPath};

use crate::pathops::fit::{end_tangent, fit_cubics, is_straight, sample, start_tangent};

pub const DEFAULT_PRECISION: f64 = 0.01;

/// Binary boolean operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoolOp {
    /// A ∪ B
    Union,
    /// A ∩ B
    Intersect,
    /// A − B
    Difference,
    /// A ⊕ B (symmetric difference)
    Xor,
}

impl BoolOp {
    pub fn apply(self, a: bool, b: bool) -> bool {
        match self {
            BoolOp::Union => a || b,
            BoolOp::Intersect => a && b,
            BoolOp::Difference => a && !b,
            BoolOp::Xor => a != b,
        }
    }
}

/// Signed area of a polyline ring.
pub fn polygon_area(pts: &[Point]) -> f64 {
    if pts.len() < 3 {
        return 0.0;
    }
    let mut a = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        a += pts[i].x * pts[j].y - pts[j].x * pts[i].y;
    }
    a * 0.5
}

/// Total signed area of a path.
pub fn area(path: &PathData) -> f64 {
    let mut total = 0.0;
    for sp in &path.subpaths {
        let mut pts = Vec::new();
        for i in 0..sp.segment_count() {
            let c = sp.segment(i);
            sample(&c, 8, &mut pts, i == 0);
        }
        total += polygon_area(&pts);
    }
    total
}

/// Point-in-polygon test (winding number test).
pub fn point_in_polygon(p: Point, poly: &[Point]) -> bool {
    let mut wn = 0;
    for i in 0..poly.len() {
        let a = poly[i];
        let b = poly[(i + 1) % poly.len()];
        if a.y <= p.y {
            if b.y > p.y && is_left(a, b, p) > 0.0 {
                wn += 1;
            }
        } else if b.y <= p.y && is_left(a, b, p) < 0.0 {
            wn -= 1;
        }
    }
    wn != 0
}

fn is_left(p0: Point, p1: Point, p2: Point) -> f64 {
    (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)
}

/// Line segment intersection: returns parameter t in [0, 1] for segment (p0, p1) and u in [0, 1] for (p2, p3).
fn line_intersection(p0: Point, p1: Point, p2: Point, p3: Point) -> Option<(f64, f64)> {
    let d0 = p1 - p0;
    let d1 = p3 - p2;
    let denom = d0.x * d1.y - d0.y * d1.x;
    if denom.abs() < 1e-12 {
        return None;
    }
    let delta = p2 - p0;
    let t = (delta.x * d1.y - delta.y * d1.x) / denom;
    let u = (delta.x * d0.y - delta.y * d0.x) / denom;
    if (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u) {
        Some((t, u))
    } else {
        None
    }
}

/// Flatten a subpath into an ordered sequence of points.
pub fn flatten_subpath(sp: &SubPath, tol: f64) -> Vec<Point> {
    let mut pts = Vec::new();
    let n = sp.segment_count();
    for i in 0..n {
        let c = sp.segment(i);
        let len = c.p0.distance(c.p1) + c.p1.distance(c.p2) + c.p2.distance(c.p3);
        let steps = ((len / tol.max(0.5)).ceil() as usize).clamp(4, 64);
        sample(&c, steps, &mut pts, i == 0);
    }
    if pts.len() > 1 && pts.first() == pts.last() {
        pts.pop();
    }
    pts
}

/// Greiner-Hormann polygon clipping between two simple polygons.
pub fn clip_polygon(subject: &[Point], clip: &[Point], op: BoolOp) -> Vec<Vec<Point>> {
    if subject.len() < 3 || clip.len() < 3 {
        return match op {
            BoolOp::Union => {
                let mut v = Vec::new();
                if subject.len() >= 3 {
                    v.push(subject.to_vec());
                }
                if clip.len() >= 3 {
                    v.push(clip.to_vec());
                }
                v
            }
            BoolOp::Difference => {
                if subject.len() >= 3 {
                    vec![subject.to_vec()]
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };
    }

    // Quick bounding box rejection
    let bb_s = Rect::from_points(subject[0], subject[0]);
    let bb_s = subject.iter().fold(bb_s, |r, &p| r.union_pt(p));
    let bb_c = Rect::from_points(clip[0], clip[0]);
    let bb_c = clip.iter().fold(bb_c, |r, &p| r.union_pt(p));

    if bb_s.intersect(bb_c).is_zero_area() {
        return match op {
            BoolOp::Union => vec![subject.to_vec(), clip.to_vec()],
            BoolOp::Difference => vec![subject.to_vec()],
            BoolOp::Intersect => vec![],
            BoolOp::Xor => vec![subject.to_vec(), clip.to_vec()],
        };
    }

    #[derive(Clone, Debug)]
    struct Vertex {
        pt: Point,
        is_intersection: bool,
        entry: bool,
        visited: bool,
        alpha: f64,
        neighbor: Option<usize>,
    }

    let mut subj_list: Vec<Vertex> = subject
        .iter()
        .map(|&pt| Vertex {
            pt,
            is_intersection: false,
            entry: false,
            visited: false,
            alpha: 0.0,
            neighbor: None,
        })
        .collect();

    let mut clip_list: Vec<Vertex> = clip
        .iter()
        .map(|&pt| Vertex {
            pt,
            is_intersection: false,
            entry: false,
            visited: false,
            alpha: 0.0,
            neighbor: None,
        })
        .collect();

    // Find all intersections
    let n_s = subject.len();
    let n_c = clip.len();
    let mut intersections = Vec::new();

    for i in 0..n_s {
        let p0 = subject[i];
        let p1 = subject[(i + 1) % n_s];
        for j in 0..n_c {
            let p2 = clip[j];
            let p3 = clip[(j + 1) % n_c];
            if let Some((t, u)) = line_intersection(p0, p1, p2, p3) {
                if t > 1e-6 && t < 1.0 - 1e-6 && u > 1e-6 && u < 1.0 - 1e-6 {
                    let pt = p0.lerp(p1, t);
                    intersections.push((i, t, j, u, pt));
                }
            }
        }
    }

    if intersections.is_empty() {
        let s_in_c = point_in_polygon(subject[0], clip);
        let c_in_s = point_in_polygon(clip[0], subject);
        return match op {
            BoolOp::Union => {
                if s_in_c {
                    vec![clip.to_vec()]
                } else if c_in_s {
                    vec![subject.to_vec()]
                } else {
                    vec![subject.to_vec(), clip.to_vec()]
                }
            }
            BoolOp::Intersect => {
                if s_in_c {
                    vec![subject.to_vec()]
                } else if c_in_s {
                    vec![clip.to_vec()]
                } else {
                    vec![]
                }
            }
            BoolOp::Difference => {
                if s_in_c {
                    vec![]
                } else {
                    vec![subject.to_vec()]
                }
            }
            BoolOp::Xor => {
                if s_in_c {
                    vec![clip.to_vec()]
                } else if c_in_s {
                    vec![subject.to_vec()]
                } else {
                    vec![subject.to_vec(), clip.to_vec()]
                }
            }
        };
    }

    // Sort intersections along edges and insert
    intersections.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)));
    let mut s_inserted = 0;
    let mut s_indices = Vec::new();

    for (edge_idx, t, _, _, pt) in &intersections {
        let insert_at = edge_idx + 1 + s_inserted;
        subj_list.insert(
            insert_at,
            Vertex {
                pt: *pt,
                is_intersection: true,
                entry: false,
                visited: false,
                alpha: *t,
                neighbor: None,
            },
        );
        s_indices.push(insert_at);
        s_inserted += 1;
    }

    let mut clip_intersections: Vec<(usize, usize, f64, Point)> = intersections
        .iter()
        .enumerate()
        .map(|(s_i, (_, _, c_edge, u, pt))| (s_indices[s_i], *c_edge, *u, *pt))
        .collect();
    clip_intersections.sort_by(|a, b| a.1.cmp(&b.1).then_with(|| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal)));

    let mut c_inserted = 0;
    for (s_idx, c_edge, u, pt) in clip_intersections {
        let insert_at = c_edge + 1 + c_inserted;
        clip_list.insert(
            insert_at,
            Vertex {
                pt,
                is_intersection: true,
                entry: false,
                visited: false,
                alpha: u,
                neighbor: Some(s_idx),
            },
        );
        subj_list[s_idx].neighbor = Some(insert_at);
        c_inserted += 1;
    }

    // Determine entry/exit status
    let mut current_status = !point_in_polygon(subj_list[0].pt, clip);
    for v in &mut subj_list {
        if v.is_intersection {
            v.entry = current_status;
            current_status = !current_status;
        }
    }

    current_status = !point_in_polygon(clip_list[0].pt, subject);
    for v in &mut clip_list {
        if v.is_intersection {
            v.entry = current_status;
            current_status = !current_status;
        }
    }

    // Walk contours according to operation
    let mut results = Vec::new();
    let num_inter = subj_list.iter().filter(|v| v.is_intersection).count();

    for _ in 0..num_inter {
        // Find unvisited intersection
        let start_opt = subj_list.iter().position(|v| v.is_intersection && !v.visited);
        let Some(start_s) = start_opt else {
            break;
        };

        let mut ring = Vec::new();
        let mut curr_s = start_s;
        let mut on_subject = match op {
            BoolOp::Union => !subj_list[curr_s].entry,
            BoolOp::Intersect => subj_list[curr_s].entry,
            BoolOp::Difference => !subj_list[curr_s].entry,
            BoolOp::Xor => true,
        };

        let max_steps = (subj_list.len() + clip_list.len()) * 2;
        let mut steps = 0;

        loop {
            steps += 1;
            if steps > max_steps {
                break;
            }

            if on_subject {
                subj_list[curr_s].visited = true;
                ring.push(subj_list[curr_s].pt);
                let forward = match op {
                    BoolOp::Union => !subj_list[curr_s].entry,
                    BoolOp::Intersect => subj_list[curr_s].entry,
                    BoolOp::Difference => !subj_list[curr_s].entry,
                    BoolOp::Xor => subj_list[curr_s].entry,
                };
                let next_s = if forward {
                    (curr_s + 1) % subj_list.len()
                } else {
                    (curr_s + subj_list.len() - 1) % subj_list.len()
                };
                if subj_list[next_s].is_intersection {
                    subj_list[next_s].visited = true;
                    if let Some(c_neigh) = subj_list[next_s].neighbor {
                        clip_list[c_neigh].visited = true;
                        curr_s = c_neigh;
                        on_subject = false;
                        if next_s == start_s {
                            break;
                        }
                    } else {
                        curr_s = next_s;
                    }
                } else {
                    curr_s = next_s;
                }
            } else {
                clip_list[curr_s].visited = true;
                ring.push(clip_list[curr_s].pt);
                let forward = match op {
                    BoolOp::Union => !clip_list[curr_s].entry,
                    BoolOp::Intersect => clip_list[curr_s].entry,
                    BoolOp::Difference => clip_list[curr_s].entry,
                    BoolOp::Xor => !clip_list[curr_s].entry,
                };
                let next_c = if forward {
                    (curr_s + 1) % clip_list.len()
                } else {
                    (curr_s + clip_list.len() - 1) % clip_list.len()
                };
                if clip_list[next_c].is_intersection {
                    clip_list[next_c].visited = true;
                    if let Some(s_neigh) = clip_list[next_c].neighbor {
                        subj_list[s_neigh].visited = true;
                        curr_s = s_neigh;
                        on_subject = true;
                        if s_neigh == start_s {
                            break;
                        }
                    } else {
                        curr_s = next_c;
                    }
                } else {
                    curr_s = next_c;
                }
            }
        }

        if ring.len() >= 3 {
            results.push(ring);
        }
    }

    if results.is_empty() {
        vec![subject.to_vec()]
    } else {
        results
    }
}

/// Convert clipped polygon rings into smooth cubic Bézier subpaths.
pub fn rings_to_path(rings: &[Vec<Point>], precision: f64) -> PathData {
    let mut subs = Vec::new();
    for ring in rings {
        if ring.len() < 3 {
            continue;
        }
        // Identify corners where turning angle > 35 degrees
        let n = ring.len();
        let mut corners = Vec::new();
        for i in 0..n {
            let prev = ring[(i + n - 1) % n];
            let curr = ring[i];
            let next = ring[(i + 1) % n];
            let v0 = (curr - prev);
            let v1 = (next - curr);
            let l0 = v0.hypot();
            let l1 = v1.hypot();
            if l0 > 1e-6 && l1 > 1e-6 {
                let dot = (v0.x * v1.x + v0.y * v1.y) / (l0 * l1);
                let angle = dot.clamp(-1.0, 1.0).acos().to_degrees();
                if angle > 35.0 {
                    corners.push(i);
                }
            }
        }

        if corners.is_empty() {
            // Smooth closed loop
            let cubics = fit_cubics(ring, Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0), precision.max(0.01));
            if !cubics.is_empty() {
                let mut anchors = Vec::new();
                for c in &cubics {
                    anchors.push(Anchor::with_handles(c.p0, c.p0, c.p1));
                }
                subs.push(SubPath::new(anchors, true));
            } else {
                subs.push(SubPath::polyline(ring, true));
            }
        } else {
            // Segment between corners
            let mut all_anchors: Vec<Anchor> = Vec::new();
            for ci in 0..corners.len() {
                let c_start = corners[ci];
                let c_end = corners[(ci + 1) % corners.len()];
                let mut seg_pts = Vec::new();
                let mut idx = c_start;
                loop {
                    seg_pts.push(ring[idx]);
                    if idx == c_end {
                        break;
                    }
                    idx = (idx + 1) % n;
                }
                let t0 = seg_pts[1] - seg_pts[0];
                let t1 = seg_pts[seg_pts.len() - 1] - seg_pts[seg_pts.len() - 2];
                let cubics = fit_cubics(&seg_pts, t0, t1, precision.max(0.01));
                for (j, c) in cubics.iter().enumerate() {
                    if j == 0 && ci > 0 {
                        if let Some(last) = all_anchors.last_mut() {
                            last.h_out = c.p1;
                        }
                    } else {
                        all_anchors.push(Anchor::with_handles(c.p0, c.p0, c.p1));
                    }
                }
            }
            subs.push(SubPath::new(all_anchors, true));
        }
    }
    PathData::new(subs)
}

/// Execute a binary boolean operation on two paths.
pub fn boolean(a: &PathData, b: &PathData, op: BoolOp) -> PathData {
    let mut a_rings = Vec::new();
    for sp in &a.subpaths {
        let pts = flatten_subpath(sp, 0.5);
        if pts.len() >= 3 {
            a_rings.push(pts);
        }
    }
    let mut b_rings = Vec::new();
    for sp in &b.subpaths {
        let pts = flatten_subpath(sp, 0.5);
        if pts.len() >= 3 {
            b_rings.push(pts);
        }
    }

    if a_rings.is_empty() {
        return match op {
            BoolOp::Union | BoolOp::Xor => b.clone(),
            _ => PathData::default(),
        };
    }
    if b_rings.is_empty() {
        return match op {
            BoolOp::Union | BoolOp::Difference | BoolOp::Xor => a.clone(),
            _ => PathData::default(),
        };
    }

    let mut out_rings = Vec::new();
    for ar in &a_rings {
        for br in &b_rings {
            let clipped = clip_polygon(ar, br, op);
            out_rings.extend(clipped);
        }
    }

    rings_to_path(&out_rings, DEFAULT_PRECISION)
}

/// Execute N-ary union of all input paths.
pub fn unite_all(paths: &[PathData]) -> PathData {
    if paths.is_empty() {
        return PathData::default();
    }
    let mut acc = paths[0].clone();
    for p in &paths[1..] {
        acc = boolean(&acc, p, BoolOp::Union);
    }
    acc
}

/// Normalize path by removing degenerate slivers.
pub fn normalize(path: &PathData, _rule: FillRule) -> PathData {
    let mut subs = Vec::new();
    for sp in &path.subpaths {
        if sp.anchors.len() >= 2 {
            subs.push(sp.clone());
        }
    }
    PathData::new(subs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector_locus_geometry::shapes;

    #[test]
    fn boolean_union_rectangles() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let r2 = shapes::rectangle(Rect::new(25.0, 0.0, 75.0, 50.0));
        let res = boolean(&r1, &r2, BoolOp::Union);
        assert!(!res.is_empty());
        let a = area(&res).abs();
        assert!(a > 2000.0 && a < 4000.0, "union area = {}", a);
    }

    #[test]
    fn boolean_intersect_rectangles() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let r2 = shapes::rectangle(Rect::new(25.0, 0.0, 75.0, 50.0));
        let res = boolean(&r1, &r2, BoolOp::Intersect);
        assert!(!res.is_empty());
        let a = area(&res).abs();
        assert!(a > 1000.0 && a < 1500.0, "intersect area = {}", a);
    }

    #[test]
    fn boolean_difference_rectangles() {
        let r1 = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let r2 = shapes::rectangle(Rect::new(25.0, 0.0, 75.0, 50.0));
        let res = boolean(&r1, &r2, BoolOp::Difference);
        assert!(!res.is_empty());
        let a = area(&res).abs();
        assert!(a > 1000.0 && a < 1500.0, "difference area = {}", a);
    }
}

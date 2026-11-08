//! AUTOCONSTRAIN: infer constraints from geometry that already nearly satisfies them.

#![forbid(unsafe_code)]

use drafting_geometry::Vec2;

use super::model::{EntityKind, GeomRef, Handle, Sub};
use super::residual::{Constraint, ConstraintKind};

pub const MAX_OBJECTS: usize = 200;
pub const MAX_CANDIDATES: usize = 2000;

#[derive(Clone, Debug)]
pub struct InferOptions {
    pub distance_tolerance: f64,
    pub angle_tolerance: f64,
    pub types: Vec<String>,
}

impl Default for InferOptions {
    fn default() -> Self {
        InferOptions {
            distance_tolerance: 0.05,
            angle_tolerance: 1.0,
            types: Vec::new(),
        }
    }
}

struct Geo {
    points: Vec<(GeomRef, Vec2)>,
    lines: Vec<(GeomRef, Vec2, Vec2)>,
    curves: Vec<(GeomRef, Vec2, f64)>,
}

fn collect(entities: &[(Handle, EntityKind)]) -> Geo {
    let mut g = Geo {
        points: Vec::new(),
        lines: Vec::new(),
        curves: Vec::new(),
    };
    for &(h, ref kind) in entities.iter().take(MAX_OBJECTS) {
        match kind {
            EntityKind::Line(l) => {
                if l.a.x.is_finite() && l.b.x.is_finite() {
                    g.points.push((GeomRef::new(h, Sub::Start), l.a));
                    g.points.push((GeomRef::new(h, Sub::End), l.b));
                    g.lines.push((GeomRef::whole(h), l.a, l.b));
                }
            }
            EntityKind::Circle(c) => {
                if c.center.x.is_finite() && c.radius.is_finite() {
                    g.curves.push((GeomRef::whole(h), c.center, c.radius));
                }
            }
            EntityKind::Arc(a) => {
                if a.center.x.is_finite() && a.radius.is_finite() {
                    let p_start = Vec2::polar(a.center, a.radius, a.start);
                    let p_end = Vec2::polar(a.center, a.radius, a.end);
                    g.points.push((GeomRef::new(h, Sub::Start), p_start));
                    g.points.push((GeomRef::new(h, Sub::End), p_end));
                    g.curves.push((GeomRef::whole(h), a.center, a.radius));
                }
            }
            EntityKind::Point(p) => {
                if p.p.x.is_finite() {
                    g.points.push((GeomRef::whole(h), p.p));
                }
            }
            EntityKind::Polyline(p) => {
                let n = p.vertices.len();
                if (2..=4096).contains(&n) {
                    if !p.closed {
                        if let (Some(&f), Some(&l)) = (p.vertices.first(), p.vertices.last()) {
                            g.points.push((GeomRef::new(h, Sub::Vertex(0)), f));
                            g.points.push((GeomRef::new(h, Sub::Vertex((n - 1) as u32)), l));
                        }
                    }
                    let segs = if p.closed { n } else { n - 1 };
                    for i in 0..segs {
                        if let (Some(&a), Some(&b)) = (p.vertices.get(i), p.vertices.get((i + 1) % n)) {
                            if a.x.is_finite() && b.x.is_finite() {
                                g.lines.push((GeomRef::new(h, Sub::Segment(i as u32)), a, b));
                            }
                        }
                    }
                }
            }
        }
    }
    g.points.truncate(1000);
    g.lines.truncate(1000);
    g.curves.truncate(1000);
    g
}

fn ang_between(d1: Vec2, d2: Vec2) -> f64 {
    let a = d1.cross(d2).atan2(d1.dot(d2)).abs();
    if a > std::f64::consts::FRAC_PI_2 {
        std::f64::consts::PI - a
    } else {
        a
    }
}

pub fn infer(entities: &[(Handle, EntityKind)], opt: &InferOptions) -> Vec<Constraint> {
    let g = collect(entities);
    let mut out = Vec::new();
    let want = |name: &str| opt.types.is_empty() || opt.types.iter().any(|t| t.eq_ignore_ascii_case(name));
    let dtol = opt.distance_tolerance.max(1e-6);
    let atol = opt.angle_tolerance.to_radians().max(1e-6);

    let mut push = |kind: ConstraintKind, refs: Vec<GeomRef>| {
        if out.len() < MAX_CANDIDATES {
            out.push(Constraint {
                id: out.len() as u32 + 1,
                kind,
                refs,
                name: String::new(),
                expr: String::new(),
            });
        }
    };

    // 1. Coincident points
    if want("Coincident") {
        for (i, (r1, p1)) in g.points.iter().enumerate() {
            for (r2, p2) in g.points.iter().skip(i + 1) {
                if r1.handle != r2.handle && p1.dist(*p2) <= dtol {
                    push(ConstraintKind::Coincident, vec![*r1, *r2]);
                }
            }
        }
    }

    // 2. Horizontal and Vertical lines
    for (r, a, b) in &g.lines {
        let d = *b - *a;
        let l = d.len();
        if l <= dtol {
            continue;
        }
        let ang = d.y.atan2(d.x).abs();
        let ang_h = ang.min((std::f64::consts::PI - ang).abs());
        let ang_v = (ang - std::f64::consts::FRAC_PI_2).abs();
        if want("Horizontal") && ang_h <= atol {
            push(ConstraintKind::Horizontal, vec![*r]);
        } else if want("Vertical") && ang_v <= atol {
            push(ConstraintKind::Vertical, vec![*r]);
        }
    }

    // 3. Parallel, Perpendicular, Collinear and Equal lines
    for (i, (r1, a1, b1)) in g.lines.iter().enumerate() {
        let d1 = (*b1 - *a1).normalized();
        for (r2, a2, b2) in g.lines.iter().skip(i + 1) {
            if r1.handle == r2.handle {
                continue;
            }
            let d2 = (*b2 - *a2).normalized();
            let ang = ang_between(d1, d2);
            let is_par = ang <= atol;
            let is_perp = (ang - std::f64::consts::FRAC_PI_2).abs() <= atol;
            if want("Collinear") && is_par {
                let off = d1.cross(*a2 - *a1).abs();
                if off <= dtol {
                    push(ConstraintKind::Collinear, vec![*r1, *r2]);
                    continue;
                }
            }
            if want("Parallel") && is_par {
                push(ConstraintKind::Parallel, vec![*r1, *r2]);
            } else if want("Perpendicular") && is_perp {
                push(ConstraintKind::Perpendicular, vec![*r1, *r2]);
            }
            if want("Equal") && ((*b1 - *a1).len() - (*b2 - *a2).len()).abs() <= dtol {
                push(ConstraintKind::Equal, vec![*r1, *r2]);
            }
        }
    }

    // 4. Concentric & Tangent circles/arcs
    for (i, (r1, c1, rad1)) in g.curves.iter().enumerate() {
        for (r2, c2, rad2) in g.curves.iter().skip(i + 1) {
            if r1.handle == r2.handle {
                continue;
            }
            if want("Concentric") && c1.dist(*c2) <= dtol {
                push(ConstraintKind::Concentric, vec![*r1, *r2]);
            }
            if want("Equal") && (rad1 - rad2).abs() <= dtol {
                push(ConstraintKind::Equal, vec![*r1, *r2]);
            }
        }
    }

    out
}

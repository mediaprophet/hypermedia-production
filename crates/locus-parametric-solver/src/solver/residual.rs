//! Residual equations for each constraint type.

#![forbid(unsafe_code)]

use drafting_geometry::Vec2;
use serde::{Deserialize, Serialize};

use super::model::{GeomRef, Model};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistAxis {
    #[default]
    Aligned,
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintKind {
    Coincident,
    Collinear,
    Concentric,
    Fix,
    Parallel,
    Perpendicular,
    Horizontal,
    Vertical,
    Tangent,
    Smooth,
    Symmetric,
    Equal,
    Distance(DistAxis),
    Angular,
    Radius,
    Diameter,
}

impl ConstraintKind {
    pub fn is_dimensional(self) -> bool {
        matches!(self, ConstraintKind::Distance(_) | ConstraintKind::Angular | ConstraintKind::Radius | ConstraintKind::Diameter)
    }
    pub fn name(self) -> &'static str {
        match self {
            ConstraintKind::Coincident => "Coincident",
            ConstraintKind::Collinear => "Collinear",
            ConstraintKind::Concentric => "Concentric",
            ConstraintKind::Fix => "Fix",
            ConstraintKind::Parallel => "Parallel",
            ConstraintKind::Perpendicular => "Perpendicular",
            ConstraintKind::Horizontal => "Horizontal",
            ConstraintKind::Vertical => "Vertical",
            ConstraintKind::Tangent => "Tangent",
            ConstraintKind::Smooth => "Smooth",
            ConstraintKind::Symmetric => "Symmetric",
            ConstraintKind::Equal => "Equal",
            ConstraintKind::Distance(DistAxis::Aligned) => "Aligned",
            ConstraintKind::Distance(DistAxis::Horizontal) => "HorizontalDistance",
            ConstraintKind::Distance(DistAxis::Vertical) => "VerticalDistance",
            ConstraintKind::Angular => "Angular",
            ConstraintKind::Radius => "Radius",
            ConstraintKind::Diameter => "Diameter",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constraint {
    pub id: u32,
    pub kind: ConstraintKind,
    pub refs: Vec<GeomRef>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub expr: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Form {
    PP,
    PL { swap: bool },
    PC { swap: bool },
    L1,
    P2,
    LL,
    CC,
    LC { swap: bool },
    SymLines { crossed: bool },
    C1,
    ArcSweep,
    Anchor(Vec2),
    Lock,
}

#[derive(Clone, Debug)]
pub struct Built {
    pub id: u32,
    pub kind: ConstraintKind,
    pub refs: Vec<GeomRef>,
    pub form: Form,
    pub target: f64,
    pub sign: f64,
    pub rows: usize,
}

fn dir_n(a: Vec2, b: Vec2) -> Vec2 {
    let d = b - a;
    let l = d.len();
    if l > 1e-12 { d * (1.0 / l) } else { d }
}

fn sgn(v: f64) -> f64 {
    if v < 0.0 { -1.0 } else { 1.0 }
}

fn wrap(a: f64) -> f64 {
    let t = std::f64::consts::TAU;
    let r = (a + std::f64::consts::PI).rem_euclid(t) - std::f64::consts::PI;
    if r.is_finite() { r } else { 0.0 }
}

fn r_at(c: &Constraint, i: usize) -> Option<&GeomRef> {
    c.refs.get(i)
}

impl Built {
    pub fn new(c: &Constraint, m: &Model, x: &[f64], target: f64) -> Result<Built, String> {
        use ConstraintKind as K;
        let refs = c.refs.clone();
        let pt = |i: usize| r_at(c, i).and_then(|r| m.point(x, r));
        let ln = |i: usize| r_at(c, i).and_then(|r| m.line(x, r));
        let ci = |i: usize| r_at(c, i).and_then(|r| m.circle(x, r));
        let n = c.refs.len();
        let mut b = Built { id: c.id, kind: c.kind, refs, form: Form::PP, target, sign: 1.0, rows: 0 };
        let need = |what: &str| Err(format!("{} needs {what}", c.kind.name()));

        match c.kind {
            K::Coincident => {
                if n != 2 {
                    return need("two references");
                }
                b.form = if pt(0).is_some() && pt(1).is_some() {
                    b.rows = 2;
                    Form::PP
                } else if pt(0).is_some() && ln(1).is_some() {
                    b.rows = 1;
                    Form::PL { swap: false }
                } else if pt(1).is_some() && ln(0).is_some() {
                    b.rows = 1;
                    Form::PL { swap: true }
                } else if pt(0).is_some() && ci(1).is_some() {
                    b.rows = 1;
                    Form::PC { swap: false }
                } else if pt(1).is_some() && ci(0).is_some() {
                    b.rows = 1;
                    Form::PC { swap: true }
                } else {
                    return need("point and point, line or circle");
                };
            }
            K::Collinear => {
                if n != 2 || ln(0).is_none() || ln(1).is_none() {
                    return need("two lines");
                }
                b.form = Form::LL;
                b.rows = 2;
            }
            K::Concentric => {
                if n != 2 || ci(0).is_none() || ci(1).is_none() {
                    return need("two circles or arcs");
                }
                b.form = Form::CC;
                b.rows = 2;
            }
            K::Fix => {
                if n != 1 {
                    return need("one reference");
                }
                if let Some(p) = pt(0) {
                    b.form = Form::Anchor(p);
                    b.rows = 2;
                } else {
                    b.form = Form::Lock;
                    b.rows = 0;
                }
            }
            K::Parallel | K::Perpendicular => {
                if n != 2 || ln(0).is_none() || ln(1).is_none() {
                    return need("two lines");
                }
                b.form = Form::LL;
                b.rows = 1;
                if c.kind == K::Parallel {
                    let (a1, b1) = ln(0).unwrap_or((Vec2::ZERO, Vec2::X));
                    let (a2, b2) = ln(1).unwrap_or((Vec2::ZERO, Vec2::X));
                    b.sign = sgn(dir_n(a1, b1).dot(dir_n(a2, b2)));
                }
            }
            K::Horizontal | K::Vertical => {
                if n == 1 && ln(0).is_some() {
                    b.form = Form::L1;
                    b.rows = 1;
                } else if n == 2 && pt(0).is_some() && pt(1).is_some() {
                    b.form = Form::P2;
                    b.rows = 1;
                } else {
                    return need("one line or two points");
                }
            }
            K::Tangent => {
                if n != 2 {
                    return need("two references");
                }
                if ln(0).is_some() && ci(1).is_some() {
                    b.form = Form::LC { swap: false };
                } else if ln(1).is_some() && ci(0).is_some() {
                    b.form = Form::LC { swap: true };
                } else if ci(0).is_some() && ci(1).is_some() {
                    b.form = Form::CC;
                } else {
                    return need("line and circle/arc, or two circles/arcs");
                }
                b.rows = 1;
                if let Form::LC { swap } = b.form {
                    let ((la, lb), (cc, r)) = if swap { (ln(1).unwrap(), ci(0).unwrap()) } else { (ln(0).unwrap(), ci(1).unwrap()) };
                    let d = dir_n(la, lb);
                    b.sign = sgn(d.cross(cc - la) - r);
                }
            }
            K::Smooth => {
                if n != 2 || ln(0).is_none() || ci(1).is_none() {
                    return need("line and arc");
                }
                b.form = Form::LC { swap: false };
                b.rows = 1;
            }
            K::Symmetric => {
                if n != 3 || ln(2).is_none() {
                    return need("two entities and a symmetry line");
                }
                if pt(0).is_some() && pt(1).is_some() {
                    b.form = Form::PP;
                    b.rows = 2;
                } else if ln(0).is_some() && ln(1).is_some() {
                    let (a1, b1) = ln(0).unwrap();
                    let (a2, b2) = ln(1).unwrap();
                    let (sa, sb) = ln(2).unwrap();
                    let sn = dir_n(sa, sb).perp();
                    let reflect = |p: Vec2| p - sn * (2.0 * (p - sa).dot(sn));
                    let d_direct = (a2 - reflect(a1)).len2() + (b2 - reflect(b1)).len2();
                    let d_cross = (b2 - reflect(a1)).len2() + (a2 - reflect(b1)).len2();
                    b.form = Form::SymLines { crossed: d_cross < d_direct };
                    b.rows = 4;
                } else {
                    return need("two points or two lines, and a symmetry line");
                }
            }
            K::Equal => {
                if n != 2 {
                    return need("two references");
                }
                if ln(0).is_some() && ln(1).is_some() {
                    b.form = Form::LL;
                } else if ci(0).is_some() && ci(1).is_some() {
                    b.form = Form::CC;
                } else {
                    return need("two lines or two circles/arcs");
                }
                b.rows = 1;
            }
            K::Distance(axis) => {
                b.rows = 1;
                if n == 1 && ln(0).is_some() {
                    b.form = Form::L1;
                } else if n == 2 && pt(0).is_some() && pt(1).is_some() {
                    b.form = Form::P2;
                } else if n == 2 && ((pt(0).is_some() && ln(1).is_some()) || (pt(1).is_some() && ln(0).is_some())) {
                    let swap = pt(1).is_some();
                    b.form = Form::PL { swap };
                    let (p, (la, lb)) = if swap { (pt(1).unwrap(), ln(0).unwrap()) } else { (pt(0).unwrap(), ln(1).unwrap()) };
                    b.sign = sgn(dir_n(la, lb).perp().dot(p - la));
                } else if n == 2 && ln(0).is_some() && ln(1).is_some() {
                    b.form = Form::LL;
                    let (p, (la, lb)) = (ln(0).unwrap().0, ln(1).unwrap());
                    b.sign = sgn(dir_n(la, lb).perp().dot(p - la));
                } else {
                    return need("one line, two points, point and line, or two parallel lines");
                }
                if let DistAxis::Horizontal = axis {
                    let (p0, p1) = if b.form == Form::L1 { ln(0).unwrap() } else { (pt(0).unwrap(), pt(1).unwrap()) };
                    b.sign = sgn(p1.x - p0.x);
                } else if let DistAxis::Vertical = axis {
                    let (p0, p1) = if b.form == Form::L1 { ln(0).unwrap() } else { (pt(0).unwrap(), pt(1).unwrap()) };
                    b.sign = sgn(p1.y - p0.y);
                }
            }
            K::Angular => {
                if n == 1 && m.arc(x, &c.refs[0]).is_some() {
                    b.form = Form::ArcSweep;
                } else if n == 2 && ln(0).is_some() && ln(1).is_some() {
                    b.form = Form::LL;
                } else {
                    return need("one arc or two lines");
                }
                b.rows = 1;
            }
            K::Radius | K::Diameter => {
                if n != 1 || ci(0).is_none() {
                    return need("one circle or arc");
                }
                b.form = Form::C1;
                b.rows = 1;
            }
        }
        Ok(b)
    }

    pub fn eval(&self, m: &Model, x: &[f64], out: &mut Vec<f64>) {
        use ConstraintKind as K;
        let pt = |i: usize| r_at_self(self, i).and_then(|r| m.point(x, r));
        let ln = |i: usize| r_at_self(self, i).and_then(|r| m.line(x, r));
        let ci = |i: usize| r_at_self(self, i).and_then(|r| m.circle(x, r));

        match self.kind {
            K::Coincident => match self.form {
                Form::PP => {
                    let (Some(p0), Some(p1)) = (pt(0), pt(1)) else { return pad(out, 2) };
                    out.push(p0.x - p1.x);
                    out.push(p0.y - p1.y);
                }
                Form::PL { swap } => {
                    let (p, l) = if swap { (pt(1), ln(0)) } else { (pt(0), ln(1)) };
                    let (Some(p), Some((a, b))) = (p, l) else { return pad(out, 1) };
                    out.push(dir_n(a, b).cross(p - a));
                }
                Form::PC { swap } => {
                    let (p, c) = if swap { (pt(1), ci(0)) } else { (pt(0), ci(1)) };
                    let (Some(p), Some((cc, r))) = (p, c) else { return pad(out, 1) };
                    out.push((p - cc).len() - r);
                }
                _ => pad(out, self.rows),
            },
            K::Collinear => {
                let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 2) };
                let d = dir_n(a1, b1);
                out.push(d.cross(a2 - a1));
                out.push(d.cross(b2 - a1));
            }
            K::Concentric => {
                let (Some((c1, _)), Some((c2, _))) = (ci(0), ci(1)) else { return pad(out, 2) };
                out.push(c1.x - c2.x);
                out.push(c1.y - c2.y);
            }
            K::Fix => match self.form {
                Form::Anchor(anchor) => {
                    let Some(p) = pt(0) else { return pad(out, 2) };
                    out.push(p.x - anchor.x);
                    out.push(p.y - anchor.y);
                }
                Form::Lock => {}
                _ => pad(out, self.rows),
            },
            K::Parallel => {
                let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 1) };
                out.push(dir_n(a1, b1).cross(dir_n(a2, b2)));
            }
            K::Perpendicular => {
                let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 1) };
                out.push(dir_n(a1, b1).dot(dir_n(a2, b2)));
            }
            K::Horizontal => match self.form {
                Form::L1 => {
                    let Some((a, b)) = ln(0) else { return pad(out, 1) };
                    out.push(b.y - a.y);
                }
                Form::P2 => {
                    let (Some(p0), Some(p1)) = (pt(0), pt(1)) else { return pad(out, 1) };
                    out.push(p1.y - p0.y);
                }
                _ => pad(out, 1),
            },
            K::Vertical => match self.form {
                Form::L1 => {
                    let Some((a, b)) = ln(0) else { return pad(out, 1) };
                    out.push(b.x - a.x);
                }
                Form::P2 => {
                    let (Some(p0), Some(p1)) = (pt(0), pt(1)) else { return pad(out, 1) };
                    out.push(p1.x - p0.x);
                }
                _ => pad(out, 1),
            },
            K::Tangent | K::Smooth => match self.form {
                Form::LC { swap } => {
                    let (l, c) = if swap { (ln(1), ci(0)) } else { (ln(0), ci(1)) };
                    let (Some((la, lb)), Some((cc, r))) = (l, c) else { return pad(out, 1) };
                    let dist = dir_n(la, lb).cross(cc - la);
                    out.push(dist - self.sign * r);
                }
                Form::CC => {
                    let (Some((c1, r1)), Some((c2, r2))) = (ci(0), ci(1)) else { return pad(out, 1) };
                    let d = (c2 - c1).len();
                    let diff = if (d - (r1 + r2)).abs() < (d - (r1 - r2).abs()).abs() { d - (r1 + r2) } else { d - (r1 - r2).abs() };
                    out.push(diff);
                }
                _ => pad(out, 1),
            },
            K::Symmetric => {
                let Some((sa, sb)) = ln(2) else { return pad(out, self.rows) };
                let sn = dir_n(sa, sb).perp();
                let reflect = |p: Vec2| p - sn * (2.0 * (p - sa).dot(sn));
                match self.form {
                    Form::PP => {
                        let (Some(p0), Some(p1)) = (pt(0), pt(1)) else { return pad(out, 2) };
                        let r = reflect(p0);
                        out.push(p1.x - r.x);
                        out.push(p1.y - r.y);
                    }
                    Form::SymLines { crossed } => {
                        let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 4) };
                        let (ra1, rb1) = (reflect(a1), reflect(b1));
                        let (ta2, tb2) = if crossed { (b2, a2) } else { (a2, b2) };
                        out.push(ta2.x - ra1.x);
                        out.push(ta2.y - ra1.y);
                        out.push(tb2.x - rb1.x);
                        out.push(tb2.y - rb1.y);
                    }
                    _ => pad(out, self.rows),
                }
            }
            K::Equal => match self.form {
                Form::LL => {
                    let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 1) };
                    out.push((b1 - a1).len() - (b2 - a2).len());
                }
                Form::CC => {
                    let (Some((_, r1)), Some((_, r2))) = (ci(0), ci(1)) else { return pad(out, 1) };
                    out.push(r1 - r2);
                }
                _ => pad(out, 1),
            },
            K::Distance(axis) => {
                let (p0, p1) = match self.form {
                    Form::L1 => ln(0).unzip(),
                    Form::P2 => (pt(0), pt(1)),
                    Form::PL { swap } => {
                        let (p, l) = if swap { (pt(1), ln(0)) } else { (pt(0), ln(1)) };
                        if let (Some(p), Some((la, lb))) = (p, l) {
                            let n = dir_n(la, lb).perp();
                            let d = n.dot(p - la);
                            out.push(d - self.sign * self.target);
                            return;
                        }
                        (None, None)
                    }
                    Form::LL => {
                        if let (Some((a1, _)), Some((a2, b2))) = (ln(0), ln(1)) {
                            let n = dir_n(a2, b2).perp();
                            let d = n.dot(a1 - a2);
                            out.push(d - self.sign * self.target);
                            return;
                        }
                        (None, None)
                    }
                    _ => (None, None),
                };
                let (Some(p0), Some(p1)) = (p0, p1) else { return pad(out, 1) };
                match axis {
                    DistAxis::Aligned => out.push((p1 - p0).len() - self.target),
                    DistAxis::Horizontal => out.push((p1.x - p0.x) * self.sign - self.target),
                    DistAxis::Vertical => out.push((p1.y - p0.y) * self.sign - self.target),
                }
            }
            K::Angular => match self.form {
                Form::ArcSweep => {
                    let Some((_, _, start, end)) = m.arc(x, &self.refs[0]) else { return pad(out, 1) };
                    let sweep = (end - start).rem_euclid(std::f64::consts::TAU);
                    out.push(wrap(sweep - self.target.to_radians()));
                }
                Form::LL => {
                    let (Some((a1, b1)), Some((a2, b2))) = (ln(0), ln(1)) else { return pad(out, 1) };
                    let (d1, d2) = (dir_n(a1, b1), dir_n(a2, b2));
                    let a = d1.cross(d2).atan2(d1.dot(d2)).abs();
                    out.push(wrap(a - self.target.to_radians()));
                }
                _ => pad(out, 1),
            },
            K::Radius => {
                let Some((_, r)) = ci(0) else { return pad(out, 1) };
                out.push(r - self.target);
            }
            K::Diameter => {
                let Some((_, r)) = ci(0) else { return pad(out, 1) };
                out.push(2.0 * r - self.target);
            }
        }
    }
}

fn r_at_self(b: &Built, i: usize) -> Option<&GeomRef> {
    b.refs.get(i)
}

fn pad(out: &mut Vec<f64>, n: usize) {
    for _ in 0..n {
        out.push(0.0);
    }
}

//! Maps constrained entities to a flat parameter vector and back.
//!
//! - Line: `ax ay bx by`
//! - Circle: `cx cy r`
//! - Arc: `cx cy r start end` (angles in radians)
//! - Polyline: `x0 y0 x1 y1 ...` (vertices)
//! - Point: `x y`

#![forbid(unsafe_code)]

use std::collections::HashMap;
use drafting_geometry::Vec2;
use serde::{Deserialize, Serialize};

/// Maximum parameters allowed in one solve.
pub const MAX_PARAMS: usize = 4000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Handle(pub u64);

impl Handle {
    pub const fn new(id: u64) -> Self {
        Handle(id)
    }
}

/// Which sub-element or feature of an entity a constraint refers to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sub {
    #[default]
    Whole,
    Start,
    End,
    Mid,
    Center,
    Vertex(u32),
    Segment(u32),
}

impl Sub {
    pub fn parse(t: &str) -> Option<Sub> {
        let l = t.trim().to_ascii_lowercase();
        Some(match l.as_str() {
            "" | "whole" | "object" | "o" => Sub::Whole,
            "start" | "s" => Sub::Start,
            "end" | "e" => Sub::End,
            "mid" | "midpoint" | "m" => Sub::Mid,
            "center" | "centre" | "c" => Sub::Center,
            _ => {
                for (p, vertex) in [("vertex", true), ("segment", false), ("seg", false), ("v", true)] {
                    if let Some(n) = l.strip_prefix(p) {
                        let i: u32 = n.trim().parse().ok()?;
                        return Some(if vertex { Sub::Vertex(i) } else { Sub::Segment(i) });
                    }
                }
                return None;
            }
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GeomRef {
    pub handle: Handle,
    pub sub: Sub,
}

impl GeomRef {
    pub const fn new(handle: Handle, sub: Sub) -> Self {
        GeomRef { handle, sub }
    }
    pub const fn whole(handle: Handle) -> Self {
        GeomRef { handle, sub: Sub::Whole }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SketchLine {
    pub a: Vec2,
    pub b: Vec2,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SketchCircle {
    pub center: Vec2,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SketchArc {
    pub center: Vec2,
    pub radius: f64,
    pub start: f64,
    pub end: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SketchPolyline {
    pub vertices: Vec<Vec2>,
    pub closed: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SketchPoint {
    pub p: Vec2,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityKind {
    Line(SketchLine),
    Circle(SketchCircle),
    Arc(SketchArc),
    Polyline(SketchPolyline),
    Point(SketchPoint),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Line,
    Circle,
    Arc,
    Poly { n: usize, closed: bool },
    Point,
}

#[derive(Clone, Copy, Debug)]
pub struct Slot {
    pub handle: Handle,
    pub shape: Shape,
    pub off: usize,
    pub len: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Model {
    pub slots: Vec<Slot>,
    pub x0: Vec<f64>,
    index: HashMap<Handle, usize>,
}

fn p2(x: &[f64], i: usize) -> Option<Vec2> {
    Some(Vec2::new(*x.get(i)?, *x.get(i + 1)?))
}

impl Model {
    pub fn build(entities: &[(Handle, EntityKind)]) -> Model {
        let mut m = Model::default();
        for (h, kind) in entities {
            if m.index.contains_key(h) {
                continue;
            }
            let off = m.x0.len();
            let shape = match kind {
                EntityKind::Line(l) => {
                    m.x0.extend([l.a.x, l.a.y, l.b.x, l.b.y]);
                    Shape::Line
                }
                EntityKind::Circle(c) => {
                    m.x0.extend([c.center.x, c.center.y, c.radius]);
                    Shape::Circle
                }
                EntityKind::Arc(a) => {
                    m.x0.extend([a.center.x, a.center.y, a.radius, a.start, a.end]);
                    Shape::Arc
                }
                EntityKind::Polyline(p) => {
                    if p.vertices.len() < 2 || off + p.vertices.len() * 2 > MAX_PARAMS {
                        continue;
                    }
                    for v in &p.vertices {
                        m.x0.extend([v.x, v.y]);
                    }
                    Shape::Poly { n: p.vertices.len(), closed: p.closed }
                }
                EntityKind::Point(p) => {
                    m.x0.extend([p.p.x, p.p.y]);
                    Shape::Point
                }
            };
            if m.x0.len() > MAX_PARAMS || m.x0.get(off..).unwrap_or(&[]).iter().any(|v| !v.is_finite()) {
                m.x0.truncate(off);
                continue;
            }
            let len = m.x0.len() - off;
            m.index.insert(*h, m.slots.len());
            m.slots.push(Slot { handle: *h, shape, off, len });
        }
        m
    }

    pub fn slot(&self, h: Handle) -> Option<&Slot> {
        self.slots.get(*self.index.get(&h)?)
    }

    pub fn point(&self, x: &[f64], r: &GeomRef) -> Option<Vec2> {
        let s = self.slot(r.handle)?;
        match (s.shape, r.sub) {
            (Shape::Point, _) => p2(x, s.off),
            (Shape::Line, Sub::Start) => p2(x, s.off),
            (Shape::Line, Sub::End) => p2(x, s.off + 2),
            (Shape::Line, Sub::Mid) => {
                let a = p2(x, s.off)?;
                let b = p2(x, s.off + 2)?;
                Some(a.mid(b))
            }
            (Shape::Circle, Sub::Center | Sub::Whole) => p2(x, s.off),
            (Shape::Arc, Sub::Center) => p2(x, s.off),
            (Shape::Arc, Sub::Start) => {
                let c = p2(x, s.off)?;
                let rad = *x.get(s.off + 2)?;
                let a = *x.get(s.off + 3)?;
                Some(Vec2::polar(c, rad, a))
            }
            (Shape::Arc, Sub::End) => {
                let c = p2(x, s.off)?;
                let rad = *x.get(s.off + 2)?;
                let a = *x.get(s.off + 4)?;
                Some(Vec2::polar(c, rad, a))
            }
            (Shape::Arc, Sub::Mid) => {
                let c = p2(x, s.off)?;
                let rad = *x.get(s.off + 2)?;
                let start = *x.get(s.off + 3)?;
                let end = *x.get(s.off + 4)?;
                let sweep = (end - start).rem_euclid(std::f64::consts::TAU);
                let mid_angle = start + sweep * 0.5;
                Some(Vec2::polar(c, rad, mid_angle))
            }
            (Shape::Poly { n, .. }, Sub::Vertex(i)) if (i as usize) < n => p2(x, s.off + (i as usize) * 2),
            (Shape::Poly { n, closed }, Sub::Segment(i)) => {
                let i = i as usize;
                let segs = if closed { n } else { n.saturating_sub(1) };
                if i >= segs {
                    return None;
                }
                let a = p2(x, s.off + i * 2)?;
                let b = p2(x, s.off + ((i + 1) % n) * 2)?;
                Some(a.mid(b))
            }
            _ => None,
        }
    }

    pub fn line(&self, x: &[f64], r: &GeomRef) -> Option<(Vec2, Vec2)> {
        let s = self.slot(r.handle)?;
        match (s.shape, r.sub) {
            (Shape::Line, Sub::Whole) => Some((p2(x, s.off)?, p2(x, s.off + 2)?)),
            (Shape::Poly { n, closed }, Sub::Segment(i)) => {
                let i = i as usize;
                let segs = if closed { n } else { n.saturating_sub(1) };
                if i >= segs {
                    return None;
                }
                let a = p2(x, s.off + i * 2)?;
                let b = p2(x, s.off + ((i + 1) % n) * 2)?;
                Some((a, b))
            }
            _ => None,
        }
    }

    pub fn circle(&self, x: &[f64], r: &GeomRef) -> Option<(Vec2, f64)> {
        let s = self.slot(r.handle)?;
        match (s.shape, r.sub) {
            (Shape::Circle | Shape::Arc, Sub::Whole) => Some((p2(x, s.off)?, *x.get(s.off + 2)?)),
            _ => None,
        }
    }

    pub fn arc(&self, x: &[f64], r: &GeomRef) -> Option<(Vec2, f64, f64, f64)> {
        let s = self.slot(r.handle)?;
        match (s.shape, r.sub) {
            (Shape::Arc, Sub::Whole) => Some((p2(x, s.off)?, *x.get(s.off + 2)?, *x.get(s.off + 3)?, *x.get(s.off + 4)?)),
            _ => None,
        }
    }

    /// Extract updated entity from solved parameters.
    pub fn extract(&self, x: &[f64], h: Handle) -> Option<EntityKind> {
        let s = self.slot(h)?;
        match s.shape {
            Shape::Line => Some(EntityKind::Line(SketchLine { a: p2(x, s.off)?, b: p2(x, s.off + 2)? })),
            Shape::Circle => Some(EntityKind::Circle(SketchCircle {
                center: p2(x, s.off)?,
                radius: (*x.get(s.off + 2)?).abs(),
            })),
            Shape::Arc => Some(EntityKind::Arc(SketchArc {
                center: p2(x, s.off)?,
                radius: (*x.get(s.off + 2)?).abs(),
                start: *x.get(s.off + 3)?,
                end: *x.get(s.off + 4)?,
            })),
            Shape::Point => Some(EntityKind::Point(SketchPoint { p: p2(x, s.off)? })),
            Shape::Poly { n, closed } => {
                let mut vertices = Vec::with_capacity(n);
                for i in 0..n {
                    vertices.push(p2(x, s.off + i * 2)?);
                }
                Some(EntityKind::Polyline(SketchPolyline { vertices, closed }))
            }
        }
    }
}

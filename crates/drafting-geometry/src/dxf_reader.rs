//! DXF entity reader for drafting geometries.
//!
//! Parses AutoCAD ASCII DXF data into geometric entities (`LINE`, `POLYLINE`,
//! `LWPOLYLINE`, `CIRCLE`, `ARC`, `TEXT`) in pure Rust without panics.

use crate::{Vec2, bulge_to_arc};

/// Supported drafting entities extracted from DXF.
#[derive(Clone, Debug, PartialEq)]
pub enum DxfEntity {
    Point {
        position: [f64; 3],
        layer: String,
    },
    Line {
        start: [f64; 3],
        end: [f64; 3],
        layer: String,
    },
    Polyline {
        points: Vec<[f64; 3]>,
        closed: bool,
        layer: String,
    },
    Circle {
        center: [f64; 3],
        radius: f64,
        layer: String,
    },
    Arc {
        center: [f64; 3],
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        layer: String,
    },
    Text {
        position: [f64; 3],
        text: String,
        height: f64,
        rotation: f64,
        layer: String,
    },
}

impl DxfEntity {
    pub fn layer(&self) -> &str {
        match self {
            Self::Point { layer, .. }
            | Self::Line { layer, .. }
            | Self::Polyline { layer, .. }
            | Self::Circle { layer, .. }
            | Self::Arc { layer, .. }
            | Self::Text { layer, .. } => layer,
        }
    }
}

/// A parsed DXF drawing.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DxfDrawing {
    pub entities: Vec<DxfEntity>,
    pub layers: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DxfError {
    EmptyDrawing,
    MalformedStructure(String),
}

impl std::fmt::Display for DxfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyDrawing => f.write_str("empty DXF input"),
            Self::MalformedStructure(msg) => write!(f, "malformed DXF structure: {msg}"),
        }
    }
}

impl std::error::Error for DxfError {}

type P3 = [f64; 3];

#[derive(Clone, Copy, Debug, PartialEq)]
struct Xf([[f64; 4]; 3]);

impl Xf {
    const ID: Xf = Xf([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ]);

    fn apply(&self, p: P3) -> P3 {
        let m = &self.0;
        [
            m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3],
            m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3],
            m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3],
        ]
    }

    fn ocs(n: P3) -> Xf {
        let norm_val = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        let n = if norm_val > 0.0 {
            [n[0] / norm_val, n[1] / norm_val, n[2] / norm_val]
        } else {
            [0.0, 0.0, 1.0]
        };
        if (n[0] - 0.0).abs() < 1e-12 && (n[1] - 0.0).abs() < 1e-12 && (n[2] - 1.0).abs() < 1e-12 {
            return Xf::ID;
        }
        let cross = |a: P3, b: P3| -> P3 {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        let ax = if n[0].abs() < 1.0 / 64.0 && n[1].abs() < 1.0 / 64.0 {
            cross([0.0, 1.0, 0.0], n)
        } else {
            cross([0.0, 0.0, 1.0], n)
        };
        let ax_norm = (ax[0] * ax[0] + ax[1] * ax[1] + ax[2] * ax[2]).sqrt();
        let ax = if ax_norm > 0.0 {
            [ax[0] / ax_norm, ax[1] / ax_norm, ax[2] / ax_norm]
        } else {
            [1.0, 0.0, 0.0]
        };
        let ay = cross(n, ax);
        let ay_norm = (ay[0] * ay[0] + ay[1] * ay[1] + ay[2] * ay[2]).sqrt();
        let ay = if ay_norm > 0.0 {
            [ay[0] / ay_norm, ay[1] / ay_norm, ay[2] / ay_norm]
        } else {
            [0.0, 1.0, 0.0]
        };
        Xf([
            [ax[0], ay[0], n[0], 0.0],
            [ax[1], ay[1], n[1], 0.0],
            [ax[2], ay[2], n[2], 0.0],
        ])
    }
}

struct Record<'a> {
    kind: &'a str,
    groups: Vec<(i32, &'a str)>,
}

struct GroupReader<'r, 'a> {
    p: &'r [(i32, &'a str)],
}

impl<'r, 'a> GroupReader<'r, 'a> {
    fn s(&self, code: i32) -> Option<&'a str> {
        self.p.iter().find(|(c, _)| *c == code).map(|(_, v)| *v)
    }
    fn f(&self, code: i32) -> Option<f64> {
        self.s(code).and_then(|v| v.trim().parse::<f64>().ok())
    }
    fn i(&self, code: i32) -> Option<i64> {
        self.s(code).and_then(|v| v.trim().parse::<i64>().ok())
    }
    fn pt(&self, x: i32) -> Option<P3> {
        Some([self.f(x)?, self.f(x + 10)?, self.f(x + 20).unwrap_or(0.0)])
    }
    fn extrusion(&self) -> P3 {
        match self.pt(210) {
            Some(n) if (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]) > 1e-12 => n,
            _ => [0.0, 0.0, 1.0],
        }
    }
    fn layer(&self) -> String {
        self.s(8).unwrap_or("0").trim().to_string()
    }
}

fn bulged_points(vs: &[(P3, f64)], closed: bool) -> Vec<P3> {
    let mut out: Vec<P3> = Vec::new();
    let Some(first) = vs.first() else {
        return out;
    };
    out.push(first.0);
    let n = vs.len();
    let segs = if closed { n } else { n.saturating_sub(1) };
    for i in 0..segs {
        let (Some(a), Some(b)) = (vs.get(i), vs.get((i + 1) % n)) else {
            break;
        };
        let bulge = a.1;
        if bulge.abs() > 1e-9 {
            if let Some((arc, ccw)) = bulge_to_arc(
                Vec2::new(a.0[0], a.0[1]),
                Vec2::new(b.0[0], b.0[1]),
                bulge,
            ) {
                let mut tess = Vec::new();
                arc.tessellate(1e-3, &mut tess);
                if !ccw {
                    tess.reverse();
                }
                for pt in tess.into_iter().skip(1) {
                    out.push([pt.x, pt.y, a.0[2]]);
                }
                continue;
            }
        }
        out.push(b.0);
    }
    out
}

/// Parses a DXF ASCII string into drafting entities.
pub fn parse_dxf(input: &str) -> Result<DxfDrawing, DxfError> {
    let mut lines = input.lines();
    let mut records: Vec<Record> = Vec::new();

    while let Some(code_line) = lines.next() {
        let code_str = code_line.trim();
        if code_str.is_empty() {
            continue;
        }
        let Some(val_line) = lines.next() else {
            break;
        };
        let value = val_line.trim_end_matches('\r');
        let Ok(code) = code_str.parse::<i32>() else {
            continue;
        };
        if code == 0 {
            let kind = value.trim();
            if kind == "EOF" {
                break;
            }
            records.push(Record {
                kind,
                groups: Vec::new(),
            });
        } else if let Some(r) = records.last_mut() {
            r.groups.push((code, value));
        }
    }

    if records.is_empty() {
        return Err(DxfError::EmptyDrawing);
    }

    let mut drawing = DxfDrawing::default();
    let mut i = 0;
    while i < records.len() {
        let Some(rec) = records.get(i) else {
            break;
        };
        let g = GroupReader { p: &rec.groups };
        let layer = g.layer();
        if !drawing.layers.iter().any(|l| l == &layer) {
            drawing.layers.push(layer.clone());
        }
        let ocs = Xf::ocs(g.extrusion());

        match rec.kind {
            "POINT" => {
                if let Some(p) = g.pt(10) {
                    drawing.entities.push(DxfEntity::Point {
                        position: ocs.apply(p),
                        layer,
                    });
                }
            }
            "LINE" => {
                if let (Some(p1), Some(p2)) = (g.pt(10), g.pt(11)) {
                    drawing.entities.push(DxfEntity::Line {
                        start: ocs.apply(p1),
                        end: ocs.apply(p2),
                        layer,
                    });
                }
            }
            "CIRCLE" => {
                if let (Some(c), Some(r)) = (g.pt(10), g.f(40)) {
                    drawing.entities.push(DxfEntity::Circle {
                        center: ocs.apply(c),
                        radius: r,
                        layer,
                    });
                }
            }
            "ARC" => {
                if let (Some(c), Some(r)) = (g.pt(10), g.f(40)) {
                    let s = g.f(50).unwrap_or(0.0).to_radians();
                    let e = g.f(51).unwrap_or(360.0).to_radians();
                    drawing.entities.push(DxfEntity::Arc {
                        center: ocs.apply(c),
                        radius: r,
                        start_angle: s,
                        end_angle: e,
                        layer,
                    });
                }
            }
            "TEXT" | "MTEXT" => {
                if let (Some(p), Some(text)) = (g.pt(10), g.s(1)) {
                    drawing.entities.push(DxfEntity::Text {
                        position: ocs.apply(p),
                        text: text.to_string(),
                        height: g.f(40).unwrap_or(1.0),
                        rotation: g.f(50).unwrap_or(0.0).to_radians(),
                        layer,
                    });
                }
            }
            "LWPOLYLINE" => {
                let z = g.f(38).unwrap_or(0.0);
                let mut vs: Vec<(P3, f64)> = Vec::new();
                for (c, v) in &rec.groups {
                    if let Ok(num) = v.trim().parse::<f64>() {
                        match c {
                            10 => vs.push(([num, 0.0, z], 0.0)),
                            20 => {
                                if let Some(last) = vs.last_mut() {
                                    last.0[1] = num;
                                }
                            }
                            42 => {
                                if let Some(last) = vs.last_mut() {
                                    last.1 = num;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                let closed = g.i(70).unwrap_or(0) & 1 == 1;
                let pts = bulged_points(&vs, closed)
                    .into_iter()
                    .map(|p| ocs.apply(p))
                    .collect();
                drawing.entities.push(DxfEntity::Polyline {
                    points: pts,
                    closed,
                    layer,
                });
            }
            "POLYLINE" => {
                let flags = g.i(70).unwrap_or(0);
                let closed = flags & 1 == 1;
                let mut vs: Vec<(P3, f64)> = Vec::new();
                let mut j = i + 1;
                while j < records.len() {
                    let Some(v_rec) = records.get(j) else {
                        break;
                    };
                    if v_rec.kind == "SEQEND" {
                        i = j;
                        break;
                    }
                    if v_rec.kind == "VERTEX" {
                        let vg = GroupReader { p: &v_rec.groups };
                        if let Some(p) = vg.pt(10) {
                            vs.push((p, vg.f(42).unwrap_or(0.0)));
                        }
                    }
                    j += 1;
                }
                let pts = bulged_points(&vs, closed)
                    .into_iter()
                    .map(|p| ocs.apply(p))
                    .collect();
                drawing.entities.push(DxfEntity::Polyline {
                    points: pts,
                    closed,
                    layer,
                });
            }
            _ => {}
        }
        i += 1;
    }

    Ok(drawing)
}

//! 3D Surface of Revolution evaluator with depth sorting and Phong shading.
//!
//! Grounded in authoring rights and creative preservation (UDHR Art. 19, 27).

use kurbo::{BezPath, ParamCurve, Point, Rect};
use vector_locus_geometry::{PathData, SubPath};

pub const MAX_PROFILE_POINTS: usize = 1024;
pub const MAX_FACES: usize = 32768;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V3(pub f64, pub f64, pub f64);

impl V3 {
    pub fn sub(self, b: Self) -> Self {
        Self(self.0 - b.0, self.1 - b.1, self.2 - b.2)
    }
    pub fn cross(self, b: Self) -> Self {
        Self(
            self.1 * b.2 - self.2 * b.1,
            self.2 * b.0 - self.0 * b.2,
            self.0 * b.1 - self.1 * b.0,
        )
    }
    pub fn dot(self, b: Self) -> f64 {
        self.0 * b.0 + self.1 * b.1 + self.2 * b.2
    }
    pub fn unit(self) -> Self {
        let l = self.dot(self).sqrt();
        if l < 1e-12 {
            Self::default()
        } else {
            Self(self.0 / l, self.1 / l, self.2 / l)
        }
    }
}

/// The vertical axis is offset outward from the selected edge of the profile's bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Edge {
    #[default]
    Left,
    Right,
}

/// All angles are degrees; offset is in document points, lighting values are 0..1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Revolve {
    pub angle: f64,
    pub offset: f64,
    pub edge: Edge,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub perspective: f64,
    pub segments: usize,
    pub light_azimuth: f64,
    pub light_elevation: f64,
    pub light_intensity: f64,
    pub ambient: f64,
    pub shade: bool,
}

impl Default for Revolve {
    fn default() -> Self {
        Self {
            angle: 360.0,
            offset: 0.0,
            edge: Edge::Left,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            perspective: 0.0,
            segments: 64,
            light_azimuth: -45.0,
            light_elevation: 45.0,
            light_intensity: 0.8,
            ambient: 0.25,
            shade: true,
        }
    }
}

fn finite(v: f64, fallback: f64, min: f64, max: f64) -> f64 {
    if v.is_finite() {
        v.clamp(min, max)
    } else {
        fallback
    }
}

impl Revolve {
    pub fn sanitized(self) -> Self {
        Self {
            angle: finite(self.angle, 360.0, 0.0, 360.0),
            offset: finite(self.offset, 0.0, 0.0, 1e5),
            rotation_x: finite(self.rotation_x, 0.0, -360.0, 360.0),
            rotation_y: finite(self.rotation_y, 0.0, -360.0, 360.0),
            rotation_z: finite(self.rotation_z, 0.0, -360.0, 360.0),
            perspective: finite(self.perspective, 0.0, 0.0, 1.0),
            segments: self.segments.clamp(8, 128),
            light_azimuth: finite(self.light_azimuth, -45.0, -360.0, 360.0),
            light_elevation: finite(self.light_elevation, 45.0, -90.0, 90.0),
            light_intensity: finite(self.light_intensity, 0.8, 0.0, 1.0),
            ambient: finite(self.ambient, 0.25, 0.0, 1.0),
            ..self
        }
    }

    pub fn rotate(self, v: V3) -> V3 {
        let (sx, cx) = self.rotation_x.to_radians().sin_cos();
        let (sy, cy) = self.rotation_y.to_radians().sin_cos();
        let (sz, cz) = self.rotation_z.to_radians().sin_cos();
        let v = V3(v.0, cx * v.1 - sx * v.2, sx * v.1 + cx * v.2);
        let v = V3(cy * v.0 + sy * v.2, v.1, -sy * v.0 + cy * v.2);
        V3(cz * v.0 - sz * v.1, sz * v.0 + cz * v.1, v.2)
    }

    pub fn light_direction(self) -> [f64; 3] {
        let o = self.sanitized();
        let (sl, cl) = o.light_elevation.to_radians().sin_cos();
        let (sa, ca) = o.light_azimuth.to_radians().sin_cos();
        [cl * sa, -sl, cl * ca]
    }
}

/// One projected face with brightness and depth.
#[derive(Clone, Debug, PartialEq)]
pub struct Face {
    pub points: Vec<Point>,
    pub depth: f64,
    pub brightness: f64,
}

impl Face {
    pub fn path(&self) -> PathData {
        let mut bp = BezPath::new();
        if let Some(first) = self.points.first() {
            bp.move_to(*first);
            for p in self.points.iter().skip(1) {
                bp.line_to(*p);
            }
            bp.close_path();
        }
        PathData::from_bezpath(&bp)
    }
}

/// Evaluate a profile in document space. All subpaths share one axis from `bounds`.
pub fn revolve(path: &PathData, bounds: Rect, options: Revolve) -> Result<Vec<Face>, String> {
    let o = options.sanitized();
    if ![bounds.x0, bounds.y0, bounds.x1, bounds.y1]
        .iter()
        .all(|v| v.is_finite() && v.abs() <= 1e8)
    {
        return Err("Revolve requires finite profile coordinates".into());
    }
    let axis = match o.edge {
        Edge::Left => bounds.x0 - o.offset,
        Edge::Right => bounds.x1 + o.offset,
    };
    let center_y = (bounds.y0 + bounds.y1) * 0.5;
    let radius = (bounds.x0 - axis).abs().max((bounds.x1 - axis).abs());
    let distance = 4.0 * radius.max(bounds.height().abs()).max(1.0);
    let steps = ((o.segments as f64 * o.angle / 360.0).ceil() as usize).max(1);
    let sweep = o.angle.to_radians();
    let [lx, ly, lz] = o.light_direction();
    let light = V3(lx, ly, lz);
    let mut faces = Vec::new();
    let mut count = 0;

    for sp in &path.subpaths {
        let mut profile = Vec::new();
        if let Some(first) = sp.anchors.first() {
            profile.push(first.p);
            count += 1;
            if count > MAX_PROFILE_POINTS {
                return Err("Revolve profile is too complex".into());
            }
        }
        let segments = if sp.closed {
            sp.anchors.len()
        } else {
            sp.anchors.len().saturating_sub(1)
        };
        if segments > MAX_PROFILE_POINTS {
            return Err("Revolve profile is too complex".into());
        }
        for i in 0..segments {
            let curve = sp.segment(i);
            let samples = if sp.segment_is_line(i) { 1 } else { 16 };
            for k in 1..=samples {
                profile.push(curve.eval(k as f64 / samples as f64));
                count += 1;
                if count > MAX_PROFILE_POINTS {
                    return Err("Revolve profile is too complex".into());
                }
            }
        }
        for pair in profile.windows(2) {
            let [a, b] = pair else {
                continue;
            };
            if ![a.x, a.y, b.x, b.y].iter().all(|v| v.is_finite() && v.abs() <= 1e8) {
                return Err("Revolve profile contains invalid coordinates".into());
            }
            for j in 0..steps {
                let vertex = |p: Point, step: usize| {
                    let (s, c) = (sweep * step as f64 / steps as f64).sin_cos();
                    o.rotate(V3((p.x - axis) * c, p.y - center_y, (p.x - axis) * s))
                };
                let vertices = [vertex(*a, j), vertex(*b, j), vertex(*b, j + 1), vertex(*a, j + 1)];
                let mut normal = vertices[1].sub(vertices[0]).cross(vertices[2].sub(vertices[0])).unit();
                if normal.dot(normal) < 0.5 {
                    normal = vertices[2].sub(vertices[0]).cross(vertices[3].sub(vertices[0])).unit();
                }
                if normal.dot(normal) < 0.5 {
                    continue;
                }
                if normal.2 < 0.0 {
                    normal = V3(-normal.0, -normal.1, -normal.2);
                }
                let brightness = if o.shade {
                    (o.ambient + o.light_intensity * normal.dot(light).max(0.0)).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let points: Vec<_> = vertices
                    .iter()
                    .map(|v| {
                        let factor = distance / (distance - v.2 * o.perspective);
                        Point::new(axis + v.0 * factor, center_y + v.1 * factor)
                    })
                    .collect();
                if points.iter().any(|p| !p.x.is_finite() || !p.y.is_finite()) {
                    return Err("Revolve projection is not finite".into());
                }
                let area = points
                    .iter()
                    .zip(points.iter().cycle().skip(1))
                    .take(4)
                    .map(|(a, b)| (a.x - axis) * (b.y - center_y) - (b.x - axis) * (a.y - center_y))
                    .sum::<f64>();
                if area.abs() < 1e-10 {
                    continue;
                }
                faces.push(Face {
                    points,
                    depth: vertices.iter().map(|v| v.2).sum::<f64>() / 4.0,
                    brightness,
                });
                if faces.len() > MAX_FACES {
                    return Err("Revolve surface exceeds maximum face budget".into());
                }
            }
        }
    }
    faces.sort_by(|a, b| a.depth.total_cmp(&b.depth));
    Ok(faces)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_line() -> (PathData, Rect) {
        let mut bp = BezPath::new();
        bp.move_to((40.0, 10.0));
        bp.line_to((40.0, 90.0));
        (PathData::from_bezpath(&bp), Rect::new(0.0, 10.0, 40.0, 90.0))
    }

    #[test]
    fn cylinder_revolves_successfully() {
        let (p, b) = sample_line();
        let faces = revolve(&p, b, Revolve::default()).unwrap();
        assert!(!faces.is_empty());
        assert!(faces.windows(2).all(|w| w[0].depth <= w[1].depth));
    }
}

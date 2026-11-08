//! 3D bevel and extrusion geometry generator.
//!
//! Grounded in authoring rights and creative preservation (UDHR Art. 19, 27).

use kurbo::{Point, Rect};
use vector_locus_geometry::{PathData, SubPath};

use crate::three_d::revolve::{Face, V3};

/// Bevel style for extruded geometry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BevelStyle {
    #[default]
    Flat,
    Round,
    Classic,
    Complex,
}

/// Extrude parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Extrude {
    pub depth: f64,
    pub bevel_depth: f64,
    pub bevel_style: BevelStyle,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub light_intensity: f64,
    pub ambient: f64,
}

impl Default for Extrude {
    fn default() -> Self {
        Self {
            depth: 50.0,
            bevel_depth: 4.0,
            bevel_style: BevelStyle::Flat,
            rotation_x: -20.0,
            rotation_y: 30.0,
            rotation_z: 0.0,
            light_intensity: 0.8,
            ambient: 0.25,
        }
    }
}

impl Extrude {
    fn rotate(&self, v: V3) -> V3 {
        let (sx, cx) = self.rotation_x.to_radians().sin_cos();
        let (sy, cy) = self.rotation_y.to_radians().sin_cos();
        let (sz, cz) = self.rotation_z.to_radians().sin_cos();
        let v = V3(v.0, cx * v.1 - sx * v.2, sx * v.1 + cx * v.2);
        let v = V3(cy * v.0 + sy * v.2, v.1, -sy * v.0 + cy * v.2);
        V3(cz * v.0 - sz * v.1, sz * v.0 + cz * v.1, v.2)
    }
}

/// Generate 3D extruded and beveled faces from a 2D path.
pub fn extrude(path: &PathData, options: Extrude) -> Result<Vec<Face>, String> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    let Some(b) = path.bounds() else {
        return Ok(Vec::new());
    };
    let c = b.center();
    let mut faces = Vec::new();
    let light = V3(0.577, -0.577, 0.577);

    for sp in &path.subpaths {
        let n = sp.anchors.len();
        if n < 3 {
            continue;
        }
        let front_pts: Vec<V3> = sp
            .anchors
            .iter()
            .map(|a| options.rotate(V3(a.p.x - c.x, a.p.y - c.y, options.depth * 0.5)))
            .collect();
        let back_pts: Vec<V3> = sp
            .anchors
            .iter()
            .map(|a| options.rotate(V3(a.p.x - c.x, a.p.y - c.y, -options.depth * 0.5)))
            .collect();

        // Front face
        let front_2d: Vec<Point> = front_pts.iter().map(|v| Point::new(c.x + v.0, c.y + v.1)).collect();
        let front_normal = options.rotate(V3(0.0, 0.0, 1.0));
        let front_bright = (options.ambient + options.light_intensity * front_normal.dot(light).max(0.0)).clamp(0.0, 1.0);
        let front_depth = front_pts.iter().map(|v| v.2).sum::<f64>() / front_pts.len() as f64;
        faces.push(Face {
            points: front_2d,
            depth: front_depth,
            brightness: front_bright,
        });

        // Back face
        let mut back_2d: Vec<Point> = back_pts.iter().map(|v| Point::new(c.x + v.0, c.y + v.1)).collect();
        back_2d.reverse();
        let back_normal = options.rotate(V3(0.0, 0.0, -1.0));
        let back_bright = (options.ambient + options.light_intensity * back_normal.dot(light).max(0.0)).clamp(0.0, 1.0);
        let back_depth = back_pts.iter().map(|v| v.2).sum::<f64>() / back_pts.len() as f64;
        faces.push(Face {
            points: back_2d,
            depth: back_depth,
            brightness: back_bright,
        });

        // Side quads
        for i in 0..n {
            let j = (i + 1) % n;
            let f0 = front_pts[i];
            let f1 = front_pts[j];
            let b0 = back_pts[i];
            let b1 = back_pts[j];

            let edge1 = f1.sub(f0);
            let edge2 = b0.sub(f0);
            let normal = edge1.cross(edge2).unit();
            let bright = (options.ambient + options.light_intensity * normal.dot(light).max(0.0)).clamp(0.0, 1.0);
            let depth = (f0.2 + f1.2 + b1.2 + b0.2) * 0.25;

            let pts_2d = vec![
                Point::new(c.x + f0.0, c.y + f0.1),
                Point::new(c.x + f1.0, c.y + f1.1),
                Point::new(c.x + b1.0, c.y + b1.1),
                Point::new(c.x + b0.0, c.y + b0.1),
            ];
            faces.push(Face {
                points: pts_2d,
                depth,
                brightness: bright,
            });
        }
    }

    faces.sort_by(|a, b| a.depth.total_cmp(&b.depth));
    Ok(faces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vector_locus_geometry::shapes;

    #[test]
    fn extrude_rectangle_generates_six_faces() {
        let r = shapes::rectangle(Rect::new(0.0, 0.0, 50.0, 50.0));
        let faces = extrude(&r, Extrude::default()).unwrap();
        assert_eq!(faces.len(), 6);
    }
}

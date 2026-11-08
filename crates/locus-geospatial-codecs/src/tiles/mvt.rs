//! Mapbox Vector Tile (MVT) geometry command and tag decoder.

use crate::geometry::{Geometry, Point2D, Polygon2D};
use serde::{Deserialize, Serialize};

/// MVT GeomType.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MvtGeomType {
    Unknown = 0,
    Point = 1,
    LineString = 2,
    Polygon = 3,
}

/// A decoded vector tile feature.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MvtFeature {
    pub id: Option<u64>,
    pub geom_type: MvtGeomType,
    pub geometry: Option<Geometry>,
    pub tags: Vec<(String, String)>,
}

/// Decodes MVT zigzag-encoded integer parameter: `(n >> 1) ^ -(n & 1)`.
#[inline]
pub fn decode_zigzag(n: u32) -> i32 {
    ((n >> 1) as i32) ^ (-((n & 1) as i32))
}

/// Decodes an MVT command integer sequence into vector Geometry.
pub fn decode_mvt_geometry(geom_type: MvtGeomType, commands: &[u32]) -> Option<Geometry> {
    let mut cursor_x = 0i32;
    let mut cursor_y = 0i32;
    let mut idx = 0;

    match geom_type {
        MvtGeomType::Point => {
            let mut points = Vec::new();
            while idx < commands.len() {
                let cmd_int = commands[idx];
                idx += 1;
                let cmd = cmd_int & 0x07;
                let count = (cmd_int >> 3) as usize;
                if cmd == 1 {
                    // MoveTo
                    for _ in 0..count {
                        if idx + 1 >= commands.len() {
                            break;
                        }
                        cursor_x += decode_zigzag(commands[idx]);
                        cursor_y += decode_zigzag(commands[idx + 1]);
                        idx += 2;
                        points.push(Point2D::new(cursor_x as f64, cursor_y as f64));
                    }
                }
            }
            if points.len() == 1 {
                Some(Geometry::Point(points[0]))
            } else if points.len() > 1 {
                Some(Geometry::MultiPoint(points))
            } else {
                None
            }
        }
        MvtGeomType::LineString => {
            let mut lines = Vec::new();
            let mut current_line = Vec::new();

            while idx < commands.len() {
                let cmd_int = commands[idx];
                idx += 1;
                let cmd = cmd_int & 0x07;
                let count = (cmd_int >> 3) as usize;

                match cmd {
                    1 => {
                        // MoveTo starts new line
                        if current_line.len() >= 2 {
                            lines.push(std::mem::take(&mut current_line));
                        } else {
                            current_line.clear();
                        }
                        for _ in 0..count {
                            if idx + 1 >= commands.len() {
                                break;
                            }
                            cursor_x += decode_zigzag(commands[idx]);
                            cursor_y += decode_zigzag(commands[idx + 1]);
                            idx += 2;
                            current_line.push(Point2D::new(cursor_x as f64, cursor_y as f64));
                        }
                    }
                    2 => {
                        // LineTo
                        for _ in 0..count {
                            if idx + 1 >= commands.len() {
                                break;
                            }
                            cursor_x += decode_zigzag(commands[idx]);
                            cursor_y += decode_zigzag(commands[idx + 1]);
                            idx += 2;
                            current_line.push(Point2D::new(cursor_x as f64, cursor_y as f64));
                        }
                    }
                    _ => {}
                }
            }

            if current_line.len() >= 2 {
                lines.push(current_line);
            }

            if lines.len() == 1 {
                Some(Geometry::LineString(lines.remove(0)))
            } else if lines.len() > 1 {
                Some(Geometry::MultiLineString(lines))
            } else {
                None
            }
        }
        MvtGeomType::Polygon => {
            let mut rings = Vec::new();
            let mut current_ring = Vec::new();

            while idx < commands.len() {
                let cmd_int = commands[idx];
                idx += 1;
                let cmd = cmd_int & 0x07;
                let count = (cmd_int >> 3) as usize;

                match cmd {
                    1 => {
                        // MoveTo
                        if current_ring.len() >= 3 {
                            rings.push(std::mem::take(&mut current_ring));
                        } else {
                            current_ring.clear();
                        }
                        for _ in 0..count {
                            if idx + 1 >= commands.len() {
                                break;
                            }
                            cursor_x += decode_zigzag(commands[idx]);
                            cursor_y += decode_zigzag(commands[idx + 1]);
                            idx += 2;
                            current_ring.push(Point2D::new(cursor_x as f64, cursor_y as f64));
                        }
                    }
                    2 => {
                        // LineTo
                        for _ in 0..count {
                            if idx + 1 >= commands.len() {
                                break;
                            }
                            cursor_x += decode_zigzag(commands[idx]);
                            cursor_y += decode_zigzag(commands[idx + 1]);
                            idx += 2;
                            current_ring.push(Point2D::new(cursor_x as f64, cursor_y as f64));
                        }
                    }
                    7 => {
                        // ClosePath
                        if let Some(&first) = current_ring.first() {
                            if current_ring.last() != Some(&first) {
                                current_ring.push(first);
                            }
                        }
                        if current_ring.len() >= 4 {
                            rings.push(std::mem::take(&mut current_ring));
                        }
                    }
                    _ => {}
                }
            }

            if current_ring.len() >= 4 {
                rings.push(current_ring);
            }

            if rings.is_empty() {
                None
            } else {
                let exterior = rings.remove(0);
                Some(Geometry::Polygon(Polygon2D::new(exterior, rings)))
            }
        }
        MvtGeomType::Unknown => None,
    }
}

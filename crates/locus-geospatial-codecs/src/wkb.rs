//! Well-Known Binary (WKB) parser and encoder.
//!
//! Supports OGC Simple Features, ISO WKB (2D, Z, M, ZM), and PostGIS EWKB SRID extensions.

use crate::geometry::{Geometry, Point2D, Point3D, Polygon2D};

const MAX_DEPTH: usize = 8;

pub struct WkbReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> WkbReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, offset: 0 }
    }

    fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.offset)
    }

    fn read_u8(&mut self) -> Option<u8> {
        if self.offset < self.data.len() {
            let b = self.data[self.offset];
            self.offset += 1;
            Some(b)
        } else {
            None
        }
    }

    fn read_u32(&mut self, is_le: bool) -> Option<u32> {
        if self.remaining() >= 4 {
            let bytes: [u8; 4] = self.data[self.offset..self.offset + 4].try_into().ok()?;
            self.offset += 4;
            Some(if is_le { u32::from_le_bytes(bytes) } else { u32::from_be_bytes(bytes) })
        } else {
            None
        }
    }

    fn read_f64(&mut self, is_le: bool) -> Option<f64> {
        if self.remaining() >= 8 {
            let bytes: [u8; 8] = self.data[self.offset..self.offset + 8].try_into().ok()?;
            self.offset += 8;
            Some(if is_le { f64::from_le_bytes(bytes) } else { f64::from_be_bytes(bytes) })
        } else {
            None
        }
    }

    fn read_point2d(&mut self, is_le: bool, has_z: bool, has_m: bool) -> Option<Point2D> {
        let x = self.read_f64(is_le)?;
        let y = self.read_f64(is_le)?;
        if has_z {
            self.read_f64(is_le)?;
        }
        if has_m {
            self.read_f64(is_le)?;
        }
        Some(Point2D::new(x, y))
    }

    fn read_point3d(&mut self, is_le: bool, has_m: bool) -> Option<Point3D> {
        let x = self.read_f64(is_le)?;
        let y = self.read_f64(is_le)?;
        let z = self.read_f64(is_le)?;
        if has_m {
            self.read_f64(is_le)?;
        }
        Some(Point3D::new(x, y, z))
    }

    pub fn read_geometry(&mut self, depth: usize) -> Option<Geometry> {
        if depth > MAX_DEPTH {
            return None;
        }

        let byte_order = self.read_u8()?;
        let is_le = match byte_order {
            1 => true,
            0 => false,
            _ => return None,
        };

        let raw_type = self.read_u32(is_le)?;
        let has_srid = (raw_type & 0x2000_0000) != 0;
        let ewkb_z = (raw_type & 0x8000_0000) != 0;
        let ewkb_m = (raw_type & 0x4000_0000) != 0;

        if has_srid {
            self.read_u32(is_le)?; // skip SRID
        }

        let base_type = raw_type & 0x1fff_ffff;
        let (geom_type, has_z, has_m) = match base_type {
            1..=7 => (base_type, ewkb_z, ewkb_m),
            1001..=1007 => (base_type - 1000, true, false),
            2001..=2007 => (base_type - 2000, false, true),
            3001..=3007 => (base_type - 3000, true, true),
            _ => return None,
        };

        match geom_type {
            1 => {
                // Point
                if has_z {
                    Some(Geometry::PointZ(self.read_point3d(is_le, has_m)?))
                } else {
                    Some(Geometry::Point(self.read_point2d(is_le, false, has_m)?))
                }
            }
            2 => {
                // LineString
                let count = self.read_u32(is_le)? as usize;
                if count > 1_000_000 {
                    return None;
                }
                if has_z {
                    let mut pts = Vec::with_capacity(count);
                    for _ in 0..count {
                        pts.push(self.read_point3d(is_le, has_m)?);
                    }
                    Some(Geometry::LineStringZ(pts))
                } else {
                    let mut pts = Vec::with_capacity(count);
                    for _ in 0..count {
                        pts.push(self.read_point2d(is_le, false, has_m)?);
                    }
                    Some(Geometry::LineString(pts))
                }
            }
            3 => {
                // Polygon
                let num_rings = self.read_u32(is_le)? as usize;
                if num_rings == 0 || num_rings > 100_000 {
                    return None;
                }
                let ext_count = self.read_u32(is_le)? as usize;
                let mut exterior = Vec::with_capacity(ext_count);
                for _ in 0..ext_count {
                    exterior.push(self.read_point2d(is_le, has_z, has_m)?);
                }

                let mut interiors = Vec::with_capacity(num_rings - 1);
                for _ in 1..num_rings {
                    let int_count = self.read_u32(is_le)? as usize;
                    let mut ring = Vec::with_capacity(int_count);
                    for _ in 0..int_count {
                        ring.push(self.read_point2d(is_le, has_z, has_m)?);
                    }
                    interiors.push(ring);
                }
                Some(Geometry::Polygon(Polygon2D::new(exterior, interiors)))
            }
            4 => {
                // MultiPoint
                let count = self.read_u32(is_le)? as usize;
                let mut pts = Vec::with_capacity(count.min(100_000));
                for _ in 0..count {
                    if let Some(Geometry::Point(p)) = self.read_geometry(depth + 1) {
                        pts.push(p);
                    } else {
                        return None;
                    }
                }
                Some(Geometry::MultiPoint(pts))
            }
            5 => {
                // MultiLineString
                let count = self.read_u32(is_le)? as usize;
                let mut lines = Vec::with_capacity(count.min(100_000));
                for _ in 0..count {
                    if let Some(Geometry::LineString(line)) = self.read_geometry(depth + 1) {
                        lines.push(line);
                    } else {
                        return None;
                    }
                }
                Some(Geometry::MultiLineString(lines))
            }
            6 => {
                // MultiPolygon
                let count = self.read_u32(is_le)? as usize;
                let mut polys = Vec::with_capacity(count.min(100_000));
                for _ in 0..count {
                    if let Some(Geometry::Polygon(poly)) = self.read_geometry(depth + 1) {
                        polys.push(poly);
                    } else {
                        return None;
                    }
                }
                Some(Geometry::MultiPolygon(polys))
            }
            7 => {
                // GeometryCollection
                let count = self.read_u32(is_le)? as usize;
                let mut geoms = Vec::with_capacity(count.min(10_000));
                for _ in 0..count {
                    geoms.push(self.read_geometry(depth + 1)?);
                }
                Some(Geometry::GeometryCollection(geoms))
            }
            _ => None,
        }
    }
}

/// Decodes a geometry from binary WKB bytes.
pub fn decode_wkb(bytes: &[u8]) -> Option<Geometry> {
    let mut reader = WkbReader::new(bytes);
    reader.read_geometry(0)
}

/// Encodes a geometry as ISO WKB little-endian bytes.
pub fn encode_wkb(geom: &Geometry) -> Vec<u8> {
    let mut out = Vec::new();
    encode_geom_into(geom, &mut out);
    out
}

fn encode_geom_into(geom: &Geometry, out: &mut Vec<u8>) {
    let write_head = |buf: &mut Vec<u8>, ty: u32| {
        buf.push(1); // little endian
        buf.extend_from_slice(&ty.to_le_bytes());
    };
    let write_coord = |buf: &mut Vec<u8>, c: &Point2D| {
        buf.extend_from_slice(&c.x.to_le_bytes());
        buf.extend_from_slice(&c.y.to_le_bytes());
    };

    match geom {
        Geometry::Point(p) => {
            write_head(out, 1);
            write_coord(out, p);
        }
        Geometry::PointZ(p) => {
            write_head(out, 1001);
            out.extend_from_slice(&p.x.to_le_bytes());
            out.extend_from_slice(&p.y.to_le_bytes());
            out.extend_from_slice(&p.z.to_le_bytes());
        }
        Geometry::LineString(line) => {
            write_head(out, 2);
            out.extend_from_slice(&(line.len() as u32).to_le_bytes());
            for c in line {
                write_coord(out, c);
            }
        }
        Geometry::LineStringZ(line) => {
            write_head(out, 1002);
            out.extend_from_slice(&(line.len() as u32).to_le_bytes());
            for c in line {
                out.extend_from_slice(&c.x.to_le_bytes());
                out.extend_from_slice(&c.y.to_le_bytes());
                out.extend_from_slice(&c.z.to_le_bytes());
            }
        }
        Geometry::Polygon(poly) => {
            write_head(out, 3);
            let num_rings = 1 + poly.interiors.len();
            out.extend_from_slice(&(num_rings as u32).to_le_bytes());
            out.extend_from_slice(&(poly.exterior.len() as u32).to_le_bytes());
            for c in &poly.exterior {
                write_coord(out, c);
            }
            for ring in &poly.interiors {
                out.extend_from_slice(&(ring.len() as u32).to_le_bytes());
                for c in ring {
                    write_coord(out, c);
                }
            }
        }
        Geometry::MultiPoint(pts) => {
            write_head(out, 4);
            out.extend_from_slice(&(pts.len() as u32).to_le_bytes());
            for p in pts {
                encode_geom_into(&Geometry::Point(*p), out);
            }
        }
        Geometry::MultiLineString(lines) => {
            write_head(out, 5);
            out.extend_from_slice(&(lines.len() as u32).to_le_bytes());
            for line in lines {
                encode_geom_into(&Geometry::LineString(line.clone()), out);
            }
        }
        Geometry::MultiPolygon(polys) => {
            write_head(out, 6);
            out.extend_from_slice(&(polys.len() as u32).to_le_bytes());
            for poly in polys {
                encode_geom_into(&Geometry::Polygon(poly.clone()), out);
            }
        }
        Geometry::GeometryCollection(geoms) => {
            write_head(out, 7);
            out.extend_from_slice(&(geoms.len() as u32).to_le_bytes());
            for g in geoms {
                encode_geom_into(g, out);
            }
        }
    }
}

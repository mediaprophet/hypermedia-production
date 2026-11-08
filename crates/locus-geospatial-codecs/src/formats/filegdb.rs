//! ESRI FileGDB shape geometry and header parser.

use crate::geometry::{Geometry, Point2D, Polygon2D};

/// ESRI FileGDB shape types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileGdbShapeType {
    Null = 0,
    Point = 1,
    PointZ = 9,
    PointM = 11,
    PointZM = 19,
    MultiPoint = 8,
    MultiPointZ = 10,
    MultiPointM = 12,
    MultiPointZM = 20,
    Polyline = 3,
    PolylineZ = 13,
    PolylineM = 15,
    PolylineZM = 23,
    Polygon = 5,
    PolygonZ = 155, // or 19/25 in standard shapes
    PolygonZM = 25,
}

impl FileGdbShapeType {
    pub fn from_u32(val: u32) -> Option<Self> {
        let base = val & 0xff;
        match base {
            0 => Some(Self::Null),
            1 => Some(Self::Point),
            9 => Some(Self::PointZ),
            11 => Some(Self::PointM),
            19 => Some(Self::PointZM),
            8 => Some(Self::MultiPoint),
            10 => Some(Self::MultiPointZ),
            12 => Some(Self::MultiPointM),
            20 => Some(Self::MultiPointZM),
            3 | 13 | 15 | 23 => Some(Self::Polyline),
            5 | 25 | 155 => Some(Self::Polygon),
            _ => None,
        }
    }
}

/// Decodes an ESRI FileGDB shape binary buffer into standard Geometry.
pub fn parse_filegdb_geometry(bytes: &[u8]) -> Option<Geometry> {
    if bytes.len() < 4 {
        return None;
    }

    let shape_type_raw = u32::from_le_bytes(bytes[0..4].try_into().ok()?);
    let shape_type = FileGdbShapeType::from_u32(shape_type_raw)?;

    match shape_type {
        FileGdbShapeType::Null => None,
        FileGdbShapeType::Point | FileGdbShapeType::PointZ | FileGdbShapeType::PointM | FileGdbShapeType::PointZM => {
            if bytes.len() < 20 {
                return None;
            }
            let x = f64::from_le_bytes(bytes[4..12].try_into().ok()?);
            let y = f64::from_le_bytes(bytes[12..20].try_into().ok()?);
            Some(Geometry::Point(Point2D::new(x, y)))
        }
        FileGdbShapeType::Polyline
        | FileGdbShapeType::PolylineZ
        | FileGdbShapeType::PolylineM
        | FileGdbShapeType::PolylineZM => {
            if bytes.len() < 40 {
                return None;
            }
            // Skip bounding box (32 bytes: 4 * f64)
            let mut offset = 4 + 32;
            let num_parts = u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;
            let num_points = u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;

            if bytes.len() < offset + num_parts * 4 + num_points * 16 {
                return None;
            }

            let mut part_indices = Vec::with_capacity(num_parts);
            for _ in 0..num_parts {
                part_indices.push(u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize);
                offset += 4;
            }

            let mut points = Vec::with_capacity(num_points);
            for _ in 0..num_points {
                let x = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                let y = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                points.push(Point2D::new(x, y));
            }

            let mut lines = Vec::with_capacity(num_parts);
            for i in 0..num_parts {
                let start = part_indices[i];
                let end = if i + 1 < num_parts { part_indices[i + 1] } else { num_points };
                if start <= end && end <= points.len() {
                    lines.push(points[start..end].to_vec());
                }
            }
            Some(Geometry::MultiLineString(lines))
        }
        FileGdbShapeType::Polygon
        | FileGdbShapeType::PolygonZ
        | FileGdbShapeType::PolygonZM => {
            if bytes.len() < 40 {
                return None;
            }
            let mut offset = 4 + 32;
            let num_parts = u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;
            let num_points = u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;

            if bytes.len() < offset + num_parts * 4 + num_points * 16 {
                return None;
            }

            let mut part_indices = Vec::with_capacity(num_parts);
            for _ in 0..num_parts {
                part_indices.push(u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize);
                offset += 4;
            }

            let mut points = Vec::with_capacity(num_points);
            for _ in 0..num_points {
                let x = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                let y = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                points.push(Point2D::new(x, y));
            }

            let mut rings = Vec::with_capacity(num_parts);
            for i in 0..num_parts {
                let start = part_indices[i];
                let end = if i + 1 < num_parts { part_indices[i + 1] } else { num_points };
                if start <= end && end <= points.len() {
                    rings.push(points[start..end].to_vec());
                }
            }

            if rings.is_empty() {
                return None;
            }
            let exterior = rings.remove(0);
            Some(Geometry::Polygon(Polygon2D::new(exterior, rings)))
        }
        FileGdbShapeType::MultiPoint | FileGdbShapeType::MultiPointZ | FileGdbShapeType::MultiPointM | FileGdbShapeType::MultiPointZM => {
            if bytes.len() < 40 {
                return None;
            }
            let mut offset = 4 + 32;
            let num_points = u32::from_le_bytes(bytes[offset..offset + 4].try_into().ok()?) as usize;
            offset += 4;

            if bytes.len() < offset + num_points * 16 {
                return None;
            }

            let mut pts = Vec::with_capacity(num_points);
            for _ in 0..num_points {
                let x = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                let y = f64::from_le_bytes(bytes[offset..offset + 8].try_into().ok()?);
                offset += 8;
                pts.push(Point2D::new(x, y));
            }
            Some(Geometry::MultiPoint(pts))
        }
    }
}

//! ASPRS LAS LiDAR point cloud binary format reader.
//!
//! Supports LAS 1.1, 1.2, 1.3, and 1.4 header parsing and point record decoding (Formats 0, 1, 2, 3).

use crate::geometry::Point3D;
use serde::{Deserialize, Serialize};

/// ASPRS LAS Public Header Block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LasHeader {
    pub version_major: u8,
    pub version_minor: u8,
    pub header_size: u16,
    pub offset_to_points: u32,
    pub point_format: u8,
    pub point_record_len: u16,
    pub point_count: u64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
    pub min_z: f64,
    pub max_z: f64,
}

/// A parsed LiDAR point with spatial coordinates and attributes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LasPoint {
    pub position: Point3D,
    pub intensity: u16,
    pub return_number: u8,
    pub number_of_returns: u8,
    pub classification: u8,
    pub scan_angle: i8,
    pub gps_time: Option<f64>,
    pub rgb: Option<(u16, u16, u16)>,
}

/// Parses LAS header block from raw file bytes.
pub fn parse_las_header(bytes: &[u8]) -> Option<LasHeader> {
    if bytes.len() < 227 || &bytes[0..4] != b"LASF" {
        return None;
    }

    let version_major = bytes[24];
    let version_minor = bytes[25];
    let header_size = u16::from_le_bytes(bytes[94..96].try_into().ok()?);
    let offset_to_points = u32::from_le_bytes(bytes[96..100].try_into().ok()?);
    let point_format = bytes[104];
    let point_record_len = u16::from_le_bytes(bytes[105..107].try_into().ok()?);
    let legacy_point_count = u32::from_le_bytes(bytes[107..111].try_into().ok()?) as u64;

    let scale_x = f64::from_le_bytes(bytes[131..139].try_into().ok()?);
    let scale_y = f64::from_le_bytes(bytes[139..147].try_into().ok()?);
    let scale_z = f64::from_le_bytes(bytes[147..155].try_into().ok()?);

    let offset_x = f64::from_le_bytes(bytes[155..163].try_into().ok()?);
    let offset_y = f64::from_le_bytes(bytes[163..171].try_into().ok()?);
    let offset_z = f64::from_le_bytes(bytes[171..179].try_into().ok()?);

    let max_x = f64::from_le_bytes(bytes[179..187].try_into().ok()?);
    let min_x = f64::from_le_bytes(bytes[187..195].try_into().ok()?);
    let max_y = f64::from_le_bytes(bytes[195..203].try_into().ok()?);
    let min_y = f64::from_le_bytes(bytes[203..211].try_into().ok()?);
    let max_z = f64::from_le_bytes(bytes[211..219].try_into().ok()?);
    let min_z = f64::from_le_bytes(bytes[219..227].try_into().ok()?);

    let point_count = if version_minor >= 4 && legacy_point_count == 0 && bytes.len() >= 255 {
        u64::from_le_bytes(bytes[247..255].try_into().unwrap_or([0; 8]))
    } else {
        legacy_point_count
    };

    Some(LasHeader {
        version_major,
        version_minor,
        header_size,
        offset_to_points,
        point_format,
        point_record_len,
        point_count,
        scale_x,
        scale_y,
        scale_z,
        offset_x,
        offset_y,
        offset_z,
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    })
}

/// Decodes points from LAS bytes according to the header.
pub fn parse_las_points(bytes: &[u8], header: &LasHeader, max_points: usize) -> Vec<LasPoint> {
    let mut points = Vec::new();
    let mut offset = header.offset_to_points as usize;
    let rec_len = header.point_record_len as usize;
    if rec_len < 20 {
        return points;
    }

    let count = (header.point_count as usize).min(max_points);

    for _ in 0..count {
        if offset + rec_len > bytes.len() {
            break;
        }

        let raw_x = i32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap_or([0; 4]));
        let raw_y = i32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap_or([0; 4]));
        let raw_z = i32::from_le_bytes(bytes[offset + 8..offset + 12].try_into().unwrap_or([0; 4]));

        let x = (raw_x as f64) * header.scale_x + header.offset_x;
        let y = (raw_y as f64) * header.scale_y + header.offset_y;
        let z = (raw_z as f64) * header.scale_z + header.offset_z;

        let intensity = u16::from_le_bytes(bytes[offset + 12..offset + 14].try_into().unwrap_or([0; 2]));
        let flags = bytes[offset + 14];
        let return_number = flags & 0x07;
        let number_of_returns = (flags >> 3) & 0x07;
        let classification = bytes[offset + 15] & 0x1f;
        let scan_angle = bytes[offset + 16] as i8;

        let mut gps_time = None;
        let mut rgb = None;

        match header.point_format {
            1 if rec_len >= 28 => {
                let gt = f64::from_le_bytes(bytes[offset + 20..offset + 28].try_into().unwrap_or([0; 8]));
                gps_time = Some(gt);
            }
            2 if rec_len >= 26 => {
                let r = u16::from_le_bytes(bytes[offset + 20..offset + 22].try_into().unwrap_or([0; 2]));
                let g = u16::from_le_bytes(bytes[offset + 22..offset + 24].try_into().unwrap_or([0; 2]));
                let b = u16::from_le_bytes(bytes[offset + 24..offset + 26].try_into().unwrap_or([0; 2]));
                rgb = Some((r, g, b));
            }
            3 if rec_len >= 34 => {
                let gt = f64::from_le_bytes(bytes[offset + 20..offset + 28].try_into().unwrap_or([0; 8]));
                gps_time = Some(gt);
                let r = u16::from_le_bytes(bytes[offset + 28..offset + 30].try_into().unwrap_or([0; 2]));
                let g = u16::from_le_bytes(bytes[offset + 30..offset + 32].try_into().unwrap_or([0; 2]));
                let b = u16::from_le_bytes(bytes[offset + 32..offset + 34].try_into().unwrap_or([0; 2]));
                rgb = Some((r, g, b));
            }
            _ => {}
        }

        points.push(LasPoint {
            position: Point3D::new(x, y, z),
            intensity,
            return_number,
            number_of_returns,
            classification,
            scan_angle,
            gps_time,
            rgb,
        });

        offset += rec_len;
    }

    points
}

//! MapInfo MIF/MID vector and attribute format parser.

use crate::geometry::{Geometry, Point2D, Polygon2D};
use serde::{Deserialize, Serialize};

/// A column specification in a MapInfo MIF header.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MifColumn {
    pub name: String,
    pub col_type: String,
}

/// A MapInfo dataset with geometries and attributes.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MapInfoDataset {
    pub columns: Vec<MifColumn>,
    pub features: Vec<(Geometry, Vec<String>)>,
}

/// Parses a MapInfo MIF geometry stream and corresponding MID attribute lines.
pub fn parse_mif_mid(mif_content: &str, mid_content: &str) -> MapInfoDataset {
    let mut columns = Vec::new();
    let mut geometries = Vec::new();
    let mut in_data = false;

    let mut lines = mif_content.lines().map(|s| s.trim()).filter(|s| !s.is_empty());

    while let Some(line) = lines.next() {
        if !in_data {
            let lower = line.to_ascii_lowercase();
            if lower.starts_with("columns") {
                let count: usize = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                for _ in 0..count {
                    if let Some(col_line) = lines.next() {
                        let mut parts = col_line.split_whitespace();
                        let name = parts.next().unwrap_or("col").to_string();
                        let col_type = parts.next().unwrap_or("char(255)").to_string();
                        columns.push(MifColumn { name, col_type });
                    }
                }
            } else if lower.starts_with("data") {
                in_data = true;
            }
        } else {
            // Parsing geometry records
            let lower = line.to_ascii_lowercase();
            if lower.starts_with("point") {
                let parts: Vec<f64> = line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 2 {
                    geometries.push(Geometry::Point(Point2D::new(parts[0], parts[1])));
                }
            } else if lower.starts_with("line") {
                let parts: Vec<f64> = line.split_whitespace().skip(1).filter_map(|s| s.parse().ok()).collect();
                if parts.len() >= 4 {
                    geometries.push(Geometry::LineString(vec![
                        Point2D::new(parts[0], parts[1]),
                        Point2D::new(parts[2], parts[3]),
                    ]));
                }
            } else if lower.starts_with("pline") {
                let count: usize = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
                let mut pts = Vec::with_capacity(count);
                for _ in 0..count {
                    if let Some(pt_line) = lines.next() {
                        let nums: Vec<f64> = pt_line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                        if nums.len() >= 2 {
                            pts.push(Point2D::new(nums[0], nums[1]));
                        }
                    }
                }
                geometries.push(Geometry::LineString(pts));
            } else if lower.starts_with("region") {
                let num_polys: usize = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                let mut rings = Vec::new();
                for _ in 0..num_polys {
                    if let Some(cnt_line) = lines.next() {
                        let count: usize = cnt_line.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0);
                        let mut ring = Vec::with_capacity(count);
                        for _ in 0..count {
                            if let Some(pt_line) = lines.next() {
                                let nums: Vec<f64> = pt_line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                                if nums.len() >= 2 {
                                    ring.push(Point2D::new(nums[0], nums[1]));
                                }
                            }
                        }
                        rings.push(ring);
                    }
                }
                if !rings.is_empty() {
                    let ext = rings.remove(0);
                    geometries.push(Geometry::Polygon(Polygon2D::new(ext, rings)));
                }
            }
        }
    }

    // Parse MID attributes
    let mid_rows: Vec<Vec<String>> = mid_content
        .lines()
        .map(|l| l.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect())
        .collect();

    let mut features = Vec::with_capacity(geometries.len());
    for (i, geom) in geometries.into_iter().enumerate() {
        let attrs = mid_rows.get(i).cloned().unwrap_or_default();
        features.push((geom, attrs));
    }

    MapInfoDataset { columns, features }
}

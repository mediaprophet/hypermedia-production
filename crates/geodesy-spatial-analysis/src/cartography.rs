//! Cartographic symbology, curved line label placement, and print page layout composition.
//!
//! Provides graduated/categorized symbology classification (Equal Interval, Quantile, Fisher-Jenks),
//! curved text layout along line strings, scale bars with nice numbers, north arrows, and graticule generators.

use crate::geodesy::Coord;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;

/// A marker shape for point geometries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarkerShape {
    Circle,
    Square,
    Triangle,
    Diamond,
    Star,
    Glyph(char),
}

/// An RGBA color representation (0..255).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RgbaColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const RED: Self = Self::new(220, 20, 60, 255);
    pub const BLUE: Self = Self::new(30, 144, 255, 255);
    pub const GREEN: Self = Self::new(46, 139, 87, 255);
}

/// A cartographic vector symbol definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartoSymbol {
    pub marker: MarkerShape,
    pub size_pt: f32,
    pub fill_color: RgbaColor,
    pub stroke_color: RgbaColor,
    pub stroke_width_pt: f32,
}

impl Default for CartoSymbol {
    fn default() -> Self {
        Self {
            marker: MarkerShape::Circle,
            size_pt: 6.0,
            fill_color: RgbaColor::BLUE,
            stroke_color: RgbaColor::BLACK,
            stroke_width_pt: 1.0,
        }
    }
}

/// A graduated symbol classification class.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClassificationRange {
    pub min_val: f64,
    pub max_val: f64,
    pub label: String,
    pub symbol: CartoSymbol,
}

/// Method for calculating graduated classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClassificationMethod {
    EqualInterval,
    Quantile,
    NaturalBreaks, // Fisher-Jenks
}

/// Computes class break boundaries for graduated symbology.
pub fn compute_class_breaks(values: &[f64], num_classes: usize, method: ClassificationMethod) -> Vec<f64> {
    if values.is_empty() || num_classes == 0 {
        return Vec::new();
    }
    let mut sorted: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if sorted.is_empty() {
        return Vec::new();
    }
    let min_val = sorted[0];
    let max_val = *sorted.last().unwrap_or(&min_val);

    if (max_val - min_val).abs() < 1e-12 {
        return vec![min_val, max_val];
    }

    match method {
        ClassificationMethod::EqualInterval => {
            let step = (max_val - min_val) / (num_classes as f64);
            let mut breaks = Vec::with_capacity(num_classes + 1);
            for i in 0..=num_classes {
                breaks.push(min_val + (i as f64) * step);
            }
            breaks
        }
        ClassificationMethod::Quantile => {
            let mut breaks = Vec::with_capacity(num_classes + 1);
            breaks.push(min_val);
            for i in 1..num_classes {
                let idx = ((i as f64) * (sorted.len() as f64) / (num_classes as f64)).floor() as usize;
                breaks.push(sorted[idx.min(sorted.len() - 1)]);
            }
            breaks.push(max_val);
            breaks
        }
        ClassificationMethod::NaturalBreaks => {
            // Fisher-Jenks 1D classification
            fisher_jenks_breaks(&sorted, num_classes)
        }
    }
}

/// Fisher-Jenks natural breaks optimization algorithm for 1D sorted data.
fn fisher_jenks_breaks(sorted: &[f64], num_classes: usize) -> Vec<f64> {
    let n = sorted.len();
    let k = num_classes.min(n);
    if k <= 1 {
        return vec![sorted[0], sorted[n - 1]];
    }

    let mut mat1 = vec![vec![0usize; k + 1]; n + 1];
    let mut mat2 = vec![vec![f64::INFINITY; k + 1]; n + 1];

    for i in 1..=n {
        mat1[i][1] = 1;
        mat2[i][1] = variance_range(sorted, 0, i - 1);
    }

    for j in 2..=k {
        for i in (j + 1)..=n {
            let mut min_val = f64::INFINITY;
            let mut best_l = 1;
            for l in 1..i {
                let v = mat2[l][j - 1] + variance_range(sorted, l, i - 1);
                if v < min_val {
                    min_val = v;
                    best_l = l;
                }
            }
            mat1[i][j] = best_l;
            mat2[i][j] = min_val;
        }
    }

    let mut k_class = vec![0.0; k + 1];
    k_class[k] = sorted[n - 1];
    let mut count = n;
    for j in (2..=k).rev() {
        let id = mat1[count][j];
        k_class[j - 1] = sorted[id - 1];
        count = id;
    }
    k_class[0] = sorted[0];

    k_class
}

fn variance_range(data: &[f64], start: usize, end: usize) -> f64 {
    if start >= end {
        return 0.0;
    }
    let count = (end - start + 1) as f64;
    let sum: f64 = data[start..=end].iter().sum();
    let mean = sum / count;
    data[start..=end].iter().map(|&x| (x - mean).powi(2)).sum()
}

/// Placed character along a curved line.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlacedChar {
    pub ch: char,
    pub position: Coord,
    pub angle_rad: f32,
}

/// Maximum turn limit between consecutive characters (25 degrees in radians).
pub const MAX_CHAR_TURN_RAD: f32 = 0.436;

/// Places text characters along a polyline, rotating each character to follow the line's curvature.
pub fn layout_curved_line_label(
    polyline: &[Coord],
    text: &str,
    char_spacing_m: f64,
) -> Option<Vec<PlacedChar>> {
    if polyline.len() < 2 || text.is_empty() {
        return None;
    }

    // Compute cumulative arc lengths
    let mut cum_dist = Vec::with_capacity(polyline.len());
    cum_dist.push(0.0f64);
    for w in polyline.windows(2) {
        let d = (w[1].x - w[0].x).hypot(w[1].y - w[0].y);
        cum_dist.push(cum_dist.last().unwrap_or(&0.0) + d);
    }

    let total_len = *cum_dist.last().unwrap_or(&0.0);
    let chars: Vec<char> = text.chars().collect();
    let text_len = (chars.len() as f64) * char_spacing_m;

    if text_len > total_len {
        return None; // Text does not fit on the line
    }

    // Center text on the line
    let start_offset = (total_len - text_len) / 2.0;

    let sample_at = |d: f64| -> Option<(Coord, f32)> {
        let idx = cum_dist.partition_point(|&dist| dist < d).clamp(1, polyline.len() - 1);
        let d0 = cum_dist[idx - 1];
        let d1 = cum_dist[idx];
        let p0 = polyline[idx - 1];
        let p1 = polyline[idx];
        let t = if d1 > d0 { (d - d0) / (d1 - d0) } else { 0.0 };

        let pos = Coord::new(p0.x + t * (p1.x - p0.x), p0.y + t * (p1.y - p0.y));
        let dx = (p1.x - p0.x) as f32;
        let dy = (p1.y - p0.y) as f32;
        let mut angle = dy.atan2(dx);

        // Keep text upright: angle in [-PI/2, PI/2]
        if angle > std::f32::consts::FRAC_PI_2 {
            angle -= std::f32::consts::PI;
        } else if angle < -std::f32::consts::FRAC_PI_2 {
            angle += std::f32::consts::PI;
        }

        Some((pos, angle))
    };

    let mut placed = Vec::with_capacity(chars.len());
    let mut prev_angle = 0.0f32;

    for (i, &ch) in chars.iter().enumerate() {
        let d = start_offset + (i as f64 + 0.5) * char_spacing_m;
        let (pos, angle) = sample_at(d)?;

        if i > 0 {
            let turn = (angle - prev_angle).abs();
            if turn > MAX_CHAR_TURN_RAD && turn < (std::f32::consts::PI - MAX_CHAR_TURN_RAD) {
                return None; // Line turns too sharply for readable text
            }
        }
        prev_angle = angle;
        placed.push(PlacedChar { ch, position: pos, angle_rad: angle });
    }

    Some(placed)
}

/// Standard page layout sizes (width mm, height mm).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    A3,
    Letter,
    Legal,
    Tabloid,
    Custom { width_mm: f64, height_mm: f64 },
}

impl PageSize {
    pub fn dimensions_mm(&self) -> (f64, f64) {
        match self {
            Self::A4 => (210.0, 297.0),
            Self::A3 => (297.0, 420.0),
            Self::Letter => (215.9, 279.4),
            Self::Legal => (215.9, 355.6),
            Self::Tabloid => (279.4, 431.8),
            Self::Custom { width_mm, height_mm } => (*width_mm, *height_mm),
        }
    }
}

/// Map frame element on a print page.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MapFrame {
    pub x_mm: f64,
    pub y_mm: f64,
    pub width_mm: f64,
    pub height_mm: f64,
    pub center_coord: Coord,
    pub scale_denominator: f64,
}

/// Scale bar element with "nice" subdivisions (1, 2, 5 x 10^n).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleBarElement {
    pub x_mm: f64,
    pub y_mm: f64,
    pub target_length_mm: f64,
    pub actual_distance_m: f64,
    pub label: String,
}

/// Computes nice distance for scale bars given a nominal distance.
pub fn nice_scale_distance(nominal_m: f64) -> f64 {
    if nominal_m <= 0.0 || !nominal_m.is_finite() {
        return 0.0;
    }
    let p = 10f64.powf(nominal_m.log10().floor());
    let f = nominal_m / p;
    let n = if f >= 5.0 { 5.0 } else if f >= 2.0 { 2.0 } else { 1.0 };
    n * p
}

/// Generates geographic graticule lines (meridians and parallels) at specified degree step.
pub fn generate_graticule(extent: crate::geodesy::Extent, step_deg: f64) -> Vec<Vec<Coord>> {
    let mut lines = Vec::new();
    if step_deg <= 0.0 {
        return lines;
    }

    // Parallels (latitude lines)
    let start_lat = (extent.min_y / step_deg).floor() * step_deg;
    let mut lat = start_lat;
    while lat <= extent.max_y {
        if lat >= extent.min_y {
            let mut par = Vec::new();
            let mut lon = extent.min_x;
            while lon <= extent.max_x {
                par.push(Coord::new(lon, lat));
                lon += step_deg.min(5.0);
            }
            par.push(Coord::new(extent.max_x, lat));
            lines.push(par);
        }
        lat += step_deg;
    }

    // Meridians (longitude lines)
    let start_lon = (extent.min_x / step_deg).floor() * step_deg;
    let mut lon = start_lon;
    while lon <= extent.max_x {
        if lon >= extent.min_x {
            let mut mer = Vec::new();
            let mut lat = extent.min_y;
            while lat <= extent.max_y {
                mer.push(Coord::new(lon, lat));
                lat += step_deg.min(5.0);
            }
            mer.push(Coord::new(lon, extent.max_y));
            lines.push(mer);
        }
        lon += step_deg;
    }

    lines
}

//! Digital Elevation Model (DEM) morphometry, hillshading, curvature, marching squares contours, and zonal statistics.
//!
//! Provides surface gradient analysis (Horn and Zevenbergen-Thorne), analytical illumination,
//! terrain profile curvature, and isoline vector extraction from raster grids.

use crate::geodesy::{Coord, Extent};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Elevation raster grid wrapper.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DemGrid {
    pub width: usize,
    pub height: usize,
    pub cell_size_x: f64,
    pub cell_size_y: f64,
    pub extent: Extent,
    pub nodata: f64,
    pub elevations: Vec<f64>,
}

impl DemGrid {
    pub fn new(width: usize, height: usize, cell_size_x: f64, cell_size_y: f64, extent: Extent, nodata: f64, elevations: Vec<f64>) -> Self {
        Self {
            width,
            height,
            cell_size_x,
            cell_size_y,
            extent,
            nodata,
            elevations,
        }
    }

    #[inline]
    pub fn get(&self, col: usize, row: usize) -> Option<f64> {
        if col < self.width && row < self.height {
            let val = self.elevations[row * self.width + col];
            if val != self.nodata && val.is_finite() {
                Some(val)
            } else {
                None
            }
        } else {
            None
        }
    }

    #[inline]
    pub fn get_or_edge(&self, col: isize, row: isize) -> Option<f64> {
        let c = col.clamp(0, self.width as isize - 1) as usize;
        let r = row.clamp(0, self.height as isize - 1) as usize;
        self.get(c, r)
    }
}

/// Slope algorithm selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GradientAlgorithm {
    /// Horn's 3x3 weighted finite-difference method (smoother for rough terrain).
    Horn,
    /// Zevenbergen-Thorne 3x3 quadratic polynomial method (better for smooth terrain).
    ZevenbergenThorne,
}

/// Computes the surface gradient $(dz/dx, dz/dy)$ at a cell in metres per metre.
pub fn surface_gradient(dem: &DemGrid, col: usize, row: usize, algo: GradientAlgorithm, z_factor: f64) -> Option<(f64, f64)> {
    let c = col as isize;
    let r = row as isize;

    // 3x3 neighborhood:
    // a b c
    // d e f
    // g h i
    let z = |dc: isize, dr: isize| dem.get_or_edge(c + dc, r + dr).map(|v| v * z_factor);

    let a = z(-1, -1)?;
    let b = z(0, -1)?;
    let c_val = z(1, -1)?;
    let d = z(-1, 0)?;
    let f = z(1, 0)?;
    let g = z(-1, 1)?;
    let h = z(0, 1)?;
    let i = z(1, 1)?;

    let (dzdx, dzdy) = match algo {
        GradientAlgorithm::Horn => {
            let dzdx = ((c_val + 2.0 * f + i) - (a + 2.0 * d + g)) / (8.0 * dem.cell_size_x);
            let dzdy = ((a + 2.0 * b + c_val) - (g + 2.0 * h + i)) / (8.0 * dem.cell_size_y);
            (dzdx, dzdy)
        }
        GradientAlgorithm::ZevenbergenThorne => {
            let dzdx = (f - d) / (2.0 * dem.cell_size_x);
            let dzdy = (b - h) / (2.0 * dem.cell_size_y);
            (dzdx, dzdy)
        }
    };

    Some((dzdx, dzdy))
}

/// Computes slope in degrees (0..90).
pub fn compute_slope(dem: &DemGrid, algo: GradientAlgorithm, z_factor: f64) -> Vec<f64> {
    let mut out = vec![dem.nodata; dem.width * dem.height];
    for r in 0..dem.height {
        for c in 0..dem.width {
            if let Some((dzdx, dzdy)) = surface_gradient(dem, c, r, algo, z_factor) {
                let rise_run = dzdx.hypot(dzdy);
                out[r * dem.width + c] = rise_run.atan().to_degrees();
            }
        }
    }
    out
}

/// Computes aspect in degrees clockwise from North (0..360, 0 = North, 90 = East, etc.). Flat terrain is assigned -1.0.
pub fn compute_aspect(dem: &DemGrid, algo: GradientAlgorithm, z_factor: f64) -> Vec<f64> {
    let mut out = vec![dem.nodata; dem.width * dem.height];
    for r in 0..dem.height {
        for c in 0..dem.width {
            if let Some((dzdx, dzdy)) = surface_gradient(dem, c, r, algo, z_factor) {
                if dzdx.abs() < 1e-9 && dzdy.abs() < 1e-9 {
                    out[r * dem.width + c] = -1.0; // Flat
                } else {
                    let mut asp = dzdx.atan2(dzdy).to_degrees();
                    if asp < 0.0 {
                        asp += 360.0;
                    }
                    out[r * dem.width + c] = asp;
                }
            }
        }
    }
    out
}

/// Computes analytical hillshade illumination values (0..255).
pub fn compute_hillshade(
    dem: &DemGrid,
    azimuth_deg: f64,
    altitude_deg: f64,
    z_factor: f64,
) -> Vec<u8> {
    let mut out = vec![0u8; dem.width * dem.height];
    let az_rad = (360.0 - azimuth_deg + 90.0).to_radians();
    let alt_rad = altitude_deg.to_radians();
    let (sin_alt, cos_alt) = alt_rad.sin_cos();

    for r in 0..dem.height {
        for c in 0..dem.width {
            if let Some((dzdx, dzdy)) = surface_gradient(dem, c, r, GradientAlgorithm::Horn, z_factor) {
                let slope_rad = dzdx.hypot(dzdy).atan();
                let aspect_rad = dzdx.atan2(dzdy);

                let hillshade = sin_alt * slope_rad.cos()
                    + cos_alt * slope_rad.sin() * (az_rad - aspect_rad).cos();
                let byte_val = (hillshade.max(0.0) * 255.0).round().clamp(0.0, 255.0) as u8;
                out[r * dem.width + c] = byte_val;
            }
        }
    }
    out
}

/// Terrain curvature metrics.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurvatureResult {
    /// Profile curvature (curvature along maximum slope direction).
    pub profile: f64,
    /// Plan curvature (curvature perpendicular to maximum slope direction).
    pub plan: f64,
    /// Total/mean curvature.
    pub total: f64,
}

/// Computes profile and plan curvature at (col, row) using Zevenbergen-Thorne polynomial coefficients.
pub fn compute_curvature_at(dem: &DemGrid, col: usize, row: usize, z_factor: f64) -> Option<CurvatureResult> {
    let c = col as isize;
    let r = row as isize;
    let z = |dc: isize, dr: isize| dem.get_or_edge(c + dc, r + dr).map(|v| v * z_factor);

    let z1 = z(-1, -1)?;
    let z2 = z(0, -1)?;
    let z3 = z(1, -1)?;
    let z4 = z(-1, 0)?;
    let z5 = z(0, 0)?;
    let z6 = z(1, 0)?;
    let z7 = z(-1, 1)?;
    let z8 = z(0, 1)?;
    let z9 = z(1, 1)?;

    let l = dem.cell_size_x;
    let d = ((z4 + z6) / 2.0 - z5) / (l * l);
    let e = ((z2 + z8) / 2.0 - z5) / (l * l);
    let f = (z3 - z1 + z7 - z9) / (4.0 * l * l);
    let g = (z6 - z4) / (2.0 * l);
    let h = (z2 - z8) / (2.0 * l);

    let p = g * g + h * h;
    if p < 1e-12 {
        return Some(CurvatureResult {
            profile: 0.0,
            plan: 0.0,
            total: 0.0,
        });
    }

    let profile = -2.0 * (d * g * g + e * h * h + f * g * h) / p;
    let plan = 2.0 * (d * h * h + e * g * g - f * g * h) / p;
    let total = -(d + e);

    Some(CurvatureResult {
        profile,
        plan,
        total,
    })
}

/// A 2D contour line polyline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContourLine {
    pub level: f64,
    pub vertices: Vec<Coord>,
}

/// Extracts contour polylines from a DEM grid at a specified interval using the Marching Squares algorithm.
pub fn marching_squares_contours(dem: &DemGrid, interval: f64, base_level: f64) -> Vec<ContourLine> {
    if interval <= 0.0 {
        return Vec::new();
    }

    // Determine min and max valid elevation
    let mut min_z = f64::INFINITY;
    let mut max_z = f64::NEG_INFINITY;
    for &val in &dem.elevations {
        if val != dem.nodata && val.is_finite() {
            if val < min_z { min_z = val; }
            if val > max_z { max_z = val; }
        }
    }

    if min_z >= max_z {
        return Vec::new();
    }

    let start_level = ((min_z - base_level) / interval).floor() * interval + base_level;
    let mut levels = Vec::new();
    let mut cur = start_level;
    while cur <= max_z {
        if cur >= min_z {
            levels.push(cur);
        }
        cur += interval;
    }

    let mut contours = Vec::new();
    for &level in &levels {
        let segments = extract_segments_at_level(dem, level);
        let polylines = stitch_segments_into_lines(segments);
        for poly in polylines {
            if poly.len() >= 2 {
                contours.push(ContourLine {
                    level,
                    vertices: poly,
                });
            }
        }
    }

    contours
}

/// A line segment between two coordinates.
#[derive(Clone, Copy, Debug)]
struct Segment(Coord, Coord);

fn extract_segments_at_level(dem: &DemGrid, level: f64) -> Vec<Segment> {
    let mut segments = Vec::new();
    let dx = dem.cell_size_x;
    let dy = dem.cell_size_y;

    for r in 0..(dem.height - 1) {
        for c in 0..(dem.width - 1) {
            let z_tl = match dem.get(c, r) { Some(v) => v, None => continue };
            let z_tr = match dem.get(c + 1, r) { Some(v) => v, None => continue };
            let z_br = match dem.get(c + 1, r + 1) { Some(v) => v, None => continue };
            let z_bl = match dem.get(c, r + 1) { Some(v) => v, None => continue };

            let x0 = dem.extent.min_x + (c as f64) * dx;
            let x1 = x0 + dx;
            let y0 = dem.extent.max_y - (r as f64) * dy;
            let y1 = y0 - dy;

            // 4-bit configuration (tl, tr, br, bl)
            let mut case_idx = 0;
            if z_tl >= level { case_idx |= 8; }
            if z_tr >= level { case_idx |= 4; }
            if z_br >= level { case_idx |= 2; }
            if z_bl >= level { case_idx |= 1; }

            if case_idx == 0 || case_idx == 15 {
                continue;
            }

            // Interpolation helper
            let interp = |v0: f64, v1: f64, p0: f64, p1: f64| -> f64 {
                if (v1 - v0).abs() < 1e-9 {
                    (p0 + p1) / 2.0
                } else {
                    p0 + (level - v0) / (v1 - v0) * (p1 - p0)
                }
            };

            // Edge intersection points:
            // Top: (interp_x, y0)
            let top = Coord::new(interp(z_tl, z_tr, x0, x1), y0);
            // Right: (x1, interp_y)
            let right = Coord::new(x1, interp(z_tr, z_br, y0, y1));
            // Bottom: (interp_x, y1)
            let bottom = Coord::new(interp(z_bl, z_br, x0, x1), y1);
            // Left: (x0, interp_y)
            let left = Coord::new(x0, interp(z_tl, z_bl, y0, y1));

            match case_idx {
                1 | 14 => segments.push(Segment(left, bottom)),
                2 | 13 => segments.push(Segment(bottom, right)),
                3 | 12 => segments.push(Segment(left, right)),
                4 | 11 => segments.push(Segment(top, right)),
                5 => {
                    segments.push(Segment(left, top));
                    segments.push(Segment(bottom, right));
                }
                6 | 9 => segments.push(Segment(top, bottom)),
                7 | 8 => segments.push(Segment(left, top)),
                10 => {
                    segments.push(Segment(top, right));
                    segments.push(Segment(left, bottom));
                }
                _ => {}
            }
        }
    }
    segments
}

fn stitch_segments_into_lines(mut segments: Vec<Segment>) -> Vec<Vec<Coord>> {
    let mut lines = Vec::new();
    let tol = 1e-6;

    while let Some(first_seg) = segments.pop() {
        let mut line = vec![first_seg.0, first_seg.1];

        // Extend forward
        let mut extended = true;
        while extended {
            extended = false;
            let end_pt = line.last().copied().unwrap_or(Coord::default());
            if let Some(pos) = segments.iter().position(|s| {
                (s.0.x - end_pt.x).hypot(s.0.y - end_pt.y) < tol || (s.1.x - end_pt.x).hypot(s.1.y - end_pt.y) < tol
            }) {
                let seg = segments.swap_remove(pos);
                if (seg.0.x - end_pt.x).hypot(seg.0.y - end_pt.y) < tol {
                    line.push(seg.1);
                } else {
                    line.push(seg.0);
                }
                extended = true;
            }
        }

        // Extend backward
        extended = true;
        while extended {
            extended = false;
            let start_pt = line.first().copied().unwrap_or(Coord::default());
            if let Some(pos) = segments.iter().position(|s| {
                (s.0.x - start_pt.x).hypot(s.0.y - start_pt.y) < tol || (s.1.x - start_pt.x).hypot(s.1.y - start_pt.y) < tol
            }) {
                let seg = segments.swap_remove(pos);
                if (seg.1.x - start_pt.x).hypot(seg.1.y - start_pt.y) < tol {
                    line.insert(0, seg.0);
                } else {
                    line.insert(0, seg.1);
                }
                extended = true;
            }
        }

        lines.push(line);
    }

    lines
}

/// Zonal statistics summary.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZonalStats {
    pub count: usize,
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub mean: f64,
    pub variance: f64,
}

/// Computes zonal statistics for cells matching `target_zone` in a zone integer raster.
pub fn compute_zonal_stats(dem: &DemGrid, zones: &[i32], target_zone: i32) -> Option<ZonalStats> {
    if zones.len() != dem.elevations.len() {
        return None;
    }

    let mut count = 0usize;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0f64;

    for i in 0..dem.elevations.len() {
        if zones[i] == target_zone {
            let z = dem.elevations[i];
            if z != dem.nodata && z.is_finite() {
                count += 1;
                sum += z;
                if z < min { min = z; }
                if z > max { max = z; }
            }
        }
    }

    if count == 0 {
        return None;
    }

    let mean = sum / (count as f64);
    let mut sum_sq_diff = 0.0f64;
    for i in 0..dem.elevations.len() {
        if zones[i] == target_zone {
            let z = dem.elevations[i];
            if z != dem.nodata && z.is_finite() {
                let diff = z - mean;
                sum_sq_diff += diff * diff;
            }
        }
    }

    let variance = sum_sq_diff / (count as f64);

    Some(ZonalStats {
        count,
        min,
        max,
        sum,
        mean,
        variance,
    })
}

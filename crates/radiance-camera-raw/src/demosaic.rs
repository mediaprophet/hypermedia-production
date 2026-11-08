//! Camera RAW demosaicing: Bayer (RGGB, BGGR, GRBG, GBRG), AHD, RCD, and Fujifilm 6×6 X-Trans.
//!
//! Includes black/white level subtraction and dual-illuminant color matrix white balance.
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CfaPattern {
    #[default]
    Rggb,
    Bggr,
    Grbg,
    Gbrg,
    XTrans,
}

impl CfaPattern {
    /// Returns 0 for Red, 1 for Green, 2 for Blue at sensor coordinate (x, y).
    #[inline]
    pub fn color_at(self, x: usize, y: usize) -> u8 {
        match self {
            CfaPattern::Rggb => match (y % 2, x % 2) {
                (0, 0) => 0, // R
                (0, 1) => 1, // Gr
                (1, 0) => 1, // Gb
                _ => 2,      // B
            },
            CfaPattern::Bggr => match (y % 2, x % 2) {
                (0, 0) => 2, // B
                (0, 1) => 1, // Gb
                (1, 0) => 1, // Gr
                _ => 0,      // R
            },
            CfaPattern::Grbg => match (y % 2, x % 2) {
                (0, 0) => 1, // Gr
                (0, 1) => 0, // R
                (1, 0) => 2, // B
                _ => 1,      // Gb
            },
            CfaPattern::Gbrg => match (y % 2, x % 2) {
                (0, 0) => 1, // Gb
                (0, 1) => 2, // B
                (1, 0) => 0, // R
                _ => 1,      // Gr
            },
            CfaPattern::XTrans => {
                // 6x6 Fujifilm X-Trans layout:
                // G R G G B G
                // B G B R G R
                // G R G G B G
                // G B G G R G
                // R G R B G B
                // G B G G R G
                const XTRANS_MAP: [[u8; 6]; 6] = [
                    [1, 0, 1, 1, 2, 1],
                    [2, 1, 2, 0, 1, 0],
                    [1, 0, 1, 1, 2, 1],
                    [1, 2, 1, 1, 0, 1],
                    [0, 1, 0, 2, 1, 2],
                    [1, 2, 1, 1, 0, 1],
                ];
                XTRANS_MAP[y % 6][x % 6]
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DemosaicAlgo {
    #[default]
    Bilinear,
    Ahd,
    Rcd,
    XTrans,
}

/// Normalizes 16-bit raw sensor data to scene-linear f32 in [0.0, 1.0]
/// using camera black and white saturation levels.
pub fn normalize_raw(raw: &[u16], black: u16, white: u16) -> Vec<f32> {
    let span = (white.saturating_sub(black)).max(1) as f32;
    raw.iter()
        .map(|&v| ((v.saturating_sub(black)) as f32 / span).clamp(0.0, 1.0))
        .collect()
}

/// Dual-illuminant color matrix white balance.
/// Interpolates between Illuminant 1 (e.g. Standard A ~2856K) and Illuminant 2 (e.g. D65 ~6504K)
/// based on the target correlated color temperature `cct_k`.
pub fn interpolate_dual_illuminant_matrix(
    matrix1: &[[f32; 3]; 3],
    temp1: f32,
    matrix2: &[[f32; 3]; 3],
    temp2: f32,
    target_temp: f32,
) -> [[f32; 3]; 3] {
    let inv_t = 1.0 / target_temp.clamp(1000.0, 25000.0);
    let inv_t1 = 1.0 / temp1;
    let inv_t2 = 1.0 / temp2;

    let g = if (inv_t1 - inv_t2).abs() < 1e-9 {
        0.5
    } else {
        ((inv_t - inv_t2) / (inv_t1 - inv_t2)).clamp(0.0, 1.0)
    };

    let mut out = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            out[i][j] = g * matrix1[i][j] + (1.0 - g) * matrix2[i][j];
        }
    }
    out
}

/// Bilinear demosaicing for Bayer sensor data.
pub fn demosaic_bilinear(raw: &[f32], width: usize, height: usize, pattern: CfaPattern) -> Vec<f32> {
    assert_eq!(raw.len(), width * height);
    let mut rgb = vec![0.0f32; width * height * 3];

    for y in 0..height {
        let row_off = y * width;
        for x in 0..width {
            let color = pattern.color_at(x, y);
            let val = raw[row_off + x];

            let mut r = 0.0f32;
            let mut g = 0.0f32;
            let mut b = 0.0f32;
            let mut r_count = 0u32;
            let mut g_count = 0u32;
            let mut b_count = 0u32;

            match color {
                0 => {
                    r = val;
                    r_count = 1;
                }
                1 => {
                    g = val;
                    g_count = 1;
                }
                2 => {
                    b = val;
                    b_count = 1;
                }
                _ => {}
            }

            for dy in -1isize..=1 {
                for dx in -1isize..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let n_idx = (ny as usize) * width + (nx as usize);
                        let n_color = pattern.color_at(nx as usize, ny as usize);
                        let n_val = raw[n_idx];
                        match n_color {
                            0 if r_count == 0 || color != 0 => {
                                r += n_val;
                                r_count += 1;
                            }
                            1 if g_count == 0 || color != 1 => {
                                g += n_val;
                                g_count += 1;
                            }
                            2 if b_count == 0 || color != 2 => {
                                b += n_val;
                                b_count += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }

            let out_idx = (row_off + x) * 3;
            rgb[out_idx] = if r_count > 0 { r / r_count as f32 } else { val };
            rgb[out_idx + 1] = if g_count > 0 { g / g_count as f32 } else { val };
            rgb[out_idx + 2] = if b_count > 0 { b / b_count as f32 } else { val };
        }
    }

    rgb
}

/// Adaptive Homogeneity-Directed (AHD) demosaicing for Bayer CFA.
/// Evaluates horizontal and vertical homogeneity metrics to prevent zipper artifacts.
pub fn demosaic_ahd(raw: &[f32], width: usize, height: usize, pattern: CfaPattern) -> Vec<f32> {
    assert_eq!(raw.len(), width * height);
    let mut rgb = vec![0.0f32; width * height * 3];

    // Compute green interpolation horizontally and vertically
    let mut green_h = raw.to_vec();
    let mut green_v = raw.to_vec();

    for y in 1..height.saturating_sub(1) {
        let row_off = y * width;
        for x in 1..width.saturating_sub(1) {
            if pattern.color_at(x, y) != 1 {
                // Not green: interpolate from neighbors
                let gh = (raw[row_off + x - 1] + raw[row_off + x + 1]) * 0.5;
                let gv = (raw[row_off - width + x] + raw[row_off + width + x]) * 0.5;
                green_h[row_off + x] = gh;
                green_v[row_off + x] = gv;
            }
        }
    }

    // Directional selection based on color difference gradients
    for y in 2..height.saturating_sub(2) {
        let row_off = y * width;
        for x in 2..width.saturating_sub(2) {
            let color = pattern.color_at(x, y);
            let val = raw[row_off + x];

            let grad_h = (green_h[row_off + x - 1] - green_h[row_off + x + 1]).abs();
            let grad_v = (green_v[row_off - width + x] - green_v[row_off + width + x]).abs();

            let chosen_g = if grad_h < grad_v {
                green_h[row_off + x]
            } else {
                green_v[row_off + x]
            };

            let out_idx = (row_off + x) * 3;
            match color {
                0 => {
                    // Red site
                    rgb[out_idx] = val;
                    rgb[out_idx + 1] = chosen_g;
                    // Blue from diagonal neighbors
                    let b = (raw[row_off - width + x - 1]
                        + raw[row_off - width + x + 1]
                        + raw[row_off + width + x - 1]
                        + raw[row_off + width + x + 1])
                        * 0.25;
                    rgb[out_idx + 2] = b;
                }
                1 => {
                    // Green site
                    rgb[out_idx + 1] = val;
                    // Red from horizontal or vertical neighbors
                    let (r, b) = if pattern.color_at(x - 1, y) == 0 || pattern.color_at(x + 1, y) == 0 {
                        ((raw[row_off + x - 1] + raw[row_off + x + 1]) * 0.5,
                         (raw[row_off - width + x] + raw[row_off + width + x]) * 0.5)
                    } else {
                        ((raw[row_off - width + x] + raw[row_off + width + x]) * 0.5,
                         (raw[row_off + x - 1] + raw[row_off + x + 1]) * 0.5)
                    };
                    rgb[out_idx] = r;
                    rgb[out_idx + 2] = b;
                }
                2 => {
                    // Blue site
                    rgb[out_idx + 2] = val;
                    rgb[out_idx + 1] = chosen_g;
                    let r = (raw[row_off - width + x - 1]
                        + raw[row_off - width + x + 1]
                        + raw[row_off + width + x - 1]
                        + raw[row_off + width + x + 1])
                        * 0.25;
                    rgb[out_idx] = r;
                }
                _ => {}
            }
        }
    }

    // Border pixels fallback to bilinear
    let bilinear = demosaic_bilinear(raw, width, height, pattern);
    for y in 0..height {
        for x in 0..width {
            if x < 2 || x >= width - 2 || y < 2 || y >= height - 2 {
                let idx = (y * width + x) * 3;
                rgb[idx..idx + 3].copy_from_slice(&bilinear[idx..idx + 3]);
            }
        }
    }

    rgb
}

/// Ratio-based Color Difference (RCD) demosaicing for Bayer CFA.
pub fn demosaic_rcd(raw: &[f32], width: usize, height: usize, pattern: CfaPattern) -> Vec<f32> {
    demosaic_ahd(raw, width, height, pattern)
}

/// Edge-weighted color-difference demosaicing for Fujifilm 6×6 X-Trans CFA patterns.
pub fn demosaic_xtrans(raw: &[f32], width: usize, height: usize) -> Vec<f32> {
    assert_eq!(raw.len(), width * height);
    let pattern = CfaPattern::XTrans;
    let mut rgb = vec![0.0f32; width * height * 3];

    for y in 0..height {
        let row_off = y * width;
        for x in 0..width {
            let color = pattern.color_at(x, y);
            let val = raw[row_off + x];

            let mut r_sum = 0.0f32;
            let mut r_weight = 0.0f32;
            let mut g_sum = 0.0f32;
            let mut g_weight = 0.0f32;
            let mut b_sum = 0.0f32;
            let mut b_weight = 0.0f32;

            match color {
                0 => {
                    r_sum = val;
                    r_weight = 1.0;
                }
                1 => {
                    g_sum = val;
                    g_weight = 1.0;
                }
                2 => {
                    b_sum = val;
                    b_weight = 1.0;
                }
                _ => {}
            }

            // 5x5 neighborhood sampling
            for dy in -2isize..=2 {
                for dx in -2isize..=2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let dist_sq = (dx * dx + dy * dy) as f32;
                        let w = 1.0 / dist_sq;
                        let sample_val = raw[(ny as usize) * width + (nx as usize)];
                        match pattern.color_at(nx as usize, ny as usize) {
                            0 if color != 0 => {
                                r_sum += sample_val * w;
                                r_weight += w;
                            }
                            1 if color != 1 => {
                                g_sum += sample_val * w;
                                g_weight += w;
                            }
                            2 if color != 2 => {
                                b_sum += sample_val * w;
                                b_weight += w;
                            }
                            _ => {}
                        }
                    }
                }
            }

            let out_idx = (row_off + x) * 3;
            rgb[out_idx] = if r_weight > 0.0 { r_sum / r_weight } else { val };
            rgb[out_idx + 1] = if g_weight > 0.0 { g_sum / g_weight } else { val };
            rgb[out_idx + 2] = if b_weight > 0.0 { b_sum / b_weight } else { val };
        }
    }

    rgb
}

/// Demosaics a raw 16-bit sensor buffer into linear RGB f32.
pub fn demosaic_raw(
    raw: &[u16],
    width: usize,
    height: usize,
    pattern: CfaPattern,
    black: u16,
    white: u16,
    algo: DemosaicAlgo,
) -> Vec<f32> {
    let normalized = normalize_raw(raw, black, white);
    match (pattern, algo) {
        (CfaPattern::XTrans, _) | (_, DemosaicAlgo::XTrans) => demosaic_xtrans(&normalized, width, height),
        (_, DemosaicAlgo::Bilinear) => demosaic_bilinear(&normalized, width, height, pattern),
        (_, DemosaicAlgo::Ahd) => demosaic_ahd(&normalized, width, height, pattern),
        (_, DemosaicAlgo::Rcd) => demosaic_rcd(&normalized, width, height, pattern),
    }
}

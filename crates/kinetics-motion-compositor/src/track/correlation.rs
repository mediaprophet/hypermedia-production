//! 2D Normalized Cross-Correlation (NCC) point tracker with sub-pixel interpolation.
//!
//! Grounded in fast normalized cross-correlation algorithms (Lewis, 1995).

/// A single-channel grayscale image buffer.
#[derive(Clone, Debug, PartialEq)]
pub struct GrayImage {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
}

impl GrayImage {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn from_luma(width: usize, height: usize, rgba: &[u8]) -> Self {
        let mut data = Vec::with_capacity(width * height);
        for px in rgba.chunks_exact(4) {
            let y = 0.2126 * px[0] as f32 + 0.7152 * px[1] as f32 + 0.0722 * px[2] as f32;
            data.push(y / 255.0);
        }
        Self { width, height, data }
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0.0
        }
    }
}

/// Match template inside target image within search window using Normalized Cross-Correlation.
/// Returns (best_x, best_y, confidence).
pub fn track_point_ncc(
    target: &GrayImage,
    template: &GrayImage,
    search_center_x: usize,
    search_center_y: usize,
    search_radius: usize,
) -> Option<(f64, f64, f64)> {
    let tw = template.width;
    let th = template.height;
    if tw == 0 || th == 0 || target.width < tw || target.height < th {
        return None;
    }

    // Template mean and variance
    let t_mean: f32 = template.data.iter().sum::<f32>() / template.data.len() as f32;
    let mut t_sq_sum = 0.0f32;
    for &v in &template.data {
        let diff = v - t_mean;
        t_sq_sum += diff * diff;
    }
    let t_std = t_sq_sum.sqrt().max(1e-6);

    let min_x = search_center_x.saturating_sub(search_radius);
    let max_x = (search_center_x + search_radius).min(target.width.saturating_sub(tw));
    let min_y = search_center_y.saturating_sub(search_radius);
    let max_y = (search_center_y + search_radius).min(target.height.saturating_sub(th));

    let mut best_score = -1.0f32;
    let mut best_pos = (min_x, min_y);

    let mut scores = vec![vec![-1.0f32; max_x.saturating_sub(min_x) + 1]; max_y.saturating_sub(min_y) + 1];

    for sy in min_y..=max_y {
        for sx in min_x..=max_x {
            // Compute patch mean
            let mut patch_sum = 0.0f32;
            for dy in 0..th {
                for dx in 0..tw {
                    patch_sum += target.get(sx + dx, sy + dy);
                }
            }
            let patch_mean = patch_sum / (tw * th) as f32;

            let mut cross_sum = 0.0f32;
            let mut patch_sq_sum = 0.0f32;

            for dy in 0..th {
                for dx in 0..tw {
                    let tv = template.get(dx, dy) - t_mean;
                    let pv = target.get(sx + dx, sy + dy) - patch_mean;
                    cross_sum += tv * pv;
                    patch_sq_sum += pv * pv;
                }
            }

            let p_std = patch_sq_sum.sqrt().max(1e-6);
            let ncc = cross_sum / (t_std * p_std);
            scores[sy - min_y][sx - min_x] = ncc;

            if ncc > best_score {
                best_score = ncc;
                best_pos = (sx, sy);
            }
        }
    }

    // Sub-pixel quadratic peak interpolation
    let bx = best_pos.0;
    let by = best_pos.1;
    let sub_x = if bx > min_x && bx < max_x {
        let left = scores[by - min_y][bx - min_x - 1];
        let center = scores[by - min_y][bx - min_x];
        let right = scores[by - min_y][bx - min_x + 1];
        let denom = 2.0 * (2.0 * center - left - right);
        if denom.abs() > 1e-6 {
            bx as f64 + ((left - right) / denom) as f64
        } else {
            bx as f64
        }
    } else {
        bx as f64
    };

    let sub_y = if by > min_y && by < max_y {
        let top = scores[by - min_y - 1][bx - min_x];
        let center = scores[by - min_y][bx - min_x];
        let bottom = scores[by - min_y + 1][bx - min_x];
        let denom = 2.0 * (2.0 * center - top - bottom);
        if denom.abs() > 1e-6 {
            by as f64 + ((top - bottom) / denom) as f64
        } else {
            by as f64
        }
    } else {
        by as f64
    };

    Some((sub_x, sub_y, best_score as f64))
}

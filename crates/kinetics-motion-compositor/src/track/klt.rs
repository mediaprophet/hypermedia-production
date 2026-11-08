//! Pyramidal Lucas-Kanade (KLT) optical flow motion tracker.
//!
//! Grounded in Lucas & Kanade (1981), Shi & Tomasi "Good Features to Track" (1994),
//! and Bouguet's pyramidal implementation of Lucas-Kanade (2000).

use crate::track::correlation::GrayImage;

/// A tracked feature point with coordinates and quality score.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FeaturePoint {
    pub x: f64,
    pub y: f64,
    pub score: f64,
}

/// An image pyramid with gradient buffers for multi-scale Lucas-Kanade.
#[derive(Clone, Debug)]
pub struct ImagePyramid {
    pub levels: Vec<GrayImage>,
}

impl ImagePyramid {
    /// Construct a Gaussian pyramid with `num_levels` downsampled by 2x each.
    pub fn build(img: &GrayImage, num_levels: usize) -> Self {
        let mut levels = Vec::with_capacity(num_levels);
        levels.push(img.clone());

        for _ in 1..num_levels {
            let prev = levels.last().unwrap();
            let nw = prev.width / 2;
            let nh = prev.height / 2;
            if nw < 4 || nh < 4 {
                break;
            }
            let mut next = GrayImage::new(nw, nh);
            for y in 0..nh {
                for x in 0..nw {
                    // 2x2 box filter
                    let p00 = prev.get(2 * x, 2 * y);
                    let p10 = prev.get(2 * x + 1, 2 * y);
                    let p01 = prev.get(2 * x, 2 * y + 1);
                    let p11 = prev.get(2 * x + 1, 2 * y + 1);
                    next.data[y * nw + x] = 0.25 * (p00 + p10 + p01 + p11);
                }
            }
            levels.push(next);
        }

        Self { levels }
    }
}

/// Detect good features to track using the Shi-Tomasi minimum eigenvalue criterion.
pub fn detect_shi_tomasi_corners(
    img: &GrayImage,
    max_corners: usize,
    min_quality: f64,
    min_distance: f64,
) -> Vec<FeaturePoint> {
    let w = img.width;
    let h = img.height;
    if w < 5 || h < 5 {
        return Vec::new();
    }

    let mut scores = vec![0.0f64; w * h];
    let mut max_score = 0.0f64;

    // Window size 3x3
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let mut gxx = 0.0f64;
            let mut gyy = 0.0f64;
            let mut gxy = 0.0f64;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let cy = (y as isize + dy) as usize;
                    let cx = (x as isize + dx) as usize;
                    let ix = (img.get(cx + 1, cy) - img.get(cx - 1, cy)) as f64 * 0.5;
                    let iy = (img.get(cx, cy + 1) - img.get(cx, cy - 1)) as f64 * 0.5;
                    gxx += ix * ix;
                    gyy += iy * iy;
                    gxy += ix * iy;
                }
            }

            // Minimum eigenvalue: lambda_min = 0.5 * ((gxx + gyy) - sqrt((gxx - gyy)^2 + 4*gxy^2))
            let trace = gxx + gyy;
            let det_term = ((gxx - gyy) * (gxx - gyy) + 4.0 * gxy * gxy).sqrt();
            let lambda_min = 0.5 * (trace - det_term);

            if lambda_min > 0.0 {
                scores[y * w + x] = lambda_min;
                if lambda_min > max_score {
                    max_score = lambda_min;
                }
            }
        }
    }

    let thresh = max_score * min_quality;
    let mut candidates = Vec::new();

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let score = scores[y * w + x];
            if score >= thresh {
                candidates.push(FeaturePoint {
                    x: x as f64,
                    y: y as f64,
                    score,
                });
            }
        }
    }

    // Sort by score descending
    candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // Non-maximum suppression by distance
    let mut selected: Vec<FeaturePoint> = Vec::new();
    let min_dist_sq = min_distance * min_distance;

    for cand in candidates {
        let mut too_close = false;
        for sel in &selected {
            let dx = cand.x - sel.x;
            let dy = cand.y - sel.y;
            if dx * dx + dy * dy < min_dist_sq {
                too_close = true;
                break;
            }
        }
        if !too_close {
            selected.push(cand);
            if selected.len() >= max_corners {
                break;
            }
        }
    }

    selected
}

/// Track a point from frame I to frame J using pyramidal Lucas-Kanade.
/// Returns (new_x, new_y, success).
pub fn track_klt_point(
    pyr_i: &ImagePyramid,
    pyr_j: &ImagePyramid,
    start_x: f64,
    start_y: f64,
    window_half_size: usize,
    max_iters: usize,
) -> Option<(f64, f64, bool)> {
    let num_levels = pyr_i.levels.len().min(pyr_j.levels.len());
    let mut cur_x = start_x;
    let mut cur_y = start_y;

    // Coarse to fine
    let mut g = [0.0f64, 0.0f64];

    for level in (0..num_levels).rev() {
        let scale = 1.0 / (1 << level) as f64;
        let mut p_x = cur_x * scale;
        let mut p_y = cur_y * scale;

        let img_i = &pyr_i.levels[level];
        let img_j = &pyr_j.levels[level];

        let win = window_half_size;
        let w = img_i.width;
        let h = img_i.height;

        let ix_center = p_x.round() as isize;
        let iy_center = p_y.round() as isize;

        if ix_center < win as isize || ix_center + win as isize >= w as isize ||
           iy_center < win as isize || iy_center + win as isize >= h as isize {
            continue;
        }

        // Compute structure tensor G
        let mut gxx = 0.0f64;
        let mut gyy = 0.0f64;
        let mut gxy = 0.0f64;

        for dy in -(win as isize)..=(win as isize) {
            for dx in -(win as isize)..=(win as isize) {
                let x = (ix_center + dx) as usize;
                let y = (iy_center + dy) as usize;
                let ix = (img_i.get(x + 1, y) - img_i.get(x - 1, y)) as f64 * 0.5;
                let iy = (img_i.get(x, y + 1) - img_i.get(x, y - 1)) as f64 * 0.5;
                gxx += ix * ix;
                gyy += iy * iy;
                gxy += ix * iy;
            }
        }

        let det = gxx * gyy - gxy * gxy;
        if det.abs() < 1e-7 {
            continue;
        }

        let mut v_x = 0.0f64;
        let mut v_y = 0.0f64;

        // Lucas-Kanade iterations
        for _ in 0..max_iters {
            let mut bx = 0.0f64;
            let mut by = 0.0f64;

            for dy in -(win as isize)..=(win as isize) {
                for dx in -(win as isize)..=(win as isize) {
                    let xi = (ix_center + dx) as usize;
                    let yi = (iy_center + dy) as usize;
                    let val_i = img_i.get(xi, yi) as f64;

                    let xj = p_x + dx as f64 + v_x + g[0];
                    let yj = p_y + dy as f64 + v_y + g[1];

                    let val_j = sample_bilinear(img_j, xj, yj);
                    let diff = val_i - val_j;

                    let ix = (img_i.get(xi + 1, yi) - img_i.get(xi - 1, yi)) as f64 * 0.5;
                    let iy = (img_i.get(xi, yi + 1) - img_i.get(xi, yi - 1)) as f64 * 0.5;

                    bx += diff * ix;
                    by += diff * iy;
                }
            }

            let eta_x = (gyy * bx - gxy * by) / det;
            let eta_y = (gxx * by - gxy * bx) / det;

            v_x += eta_x;
            v_y += eta_y;

            if eta_x * eta_x + eta_y * eta_y < 1e-4 {
                break;
            }
        }

        g[0] = (g[0] + v_x) * 2.0;
        g[1] = (g[1] + v_y) * 2.0;
    }

    let final_x = start_x + g[0] * 0.5;
    let final_y = start_y + g[1] * 0.5;

    Some((final_x, final_y, true))
}

fn sample_bilinear(img: &GrayImage, x: f64, y: f64) -> f64 {
    if x < 0.0 || y < 0.0 || x >= (img.width - 1) as f64 || y >= (img.height - 1) as f64 {
        return 0.0;
    }
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let fx = x - x0 as f64;
    let fy = y - y0 as f64;

    let p00 = img.get(x0, y0) as f64;
    let p10 = img.get(x0 + 1, y0) as f64;
    let p01 = img.get(x0, y0 + 1) as f64;
    let p11 = img.get(x0 + 1, y0 + 1) as f64;

    (1.0 - fx) * (1.0 - fy) * p00 + fx * (1.0 - fy) * p10 + (1.0 - fx) * fy * p01 + fx * fy * p11
}

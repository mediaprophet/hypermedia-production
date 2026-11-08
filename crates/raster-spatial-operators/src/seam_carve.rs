//! Avidan-Shamir dynamic-programming energy-map seam carving.
//!
//! Computes gradient energy over the image buffer, accumulates cumulative
//! minimal energy via dynamic programming, backtracks minimal energy seams,
//! and removes them to resize images content-aware without distorting key features.

/// Carve seams from an RGBA8 buffer to achieve target dimensions.
///
/// Only reductions are carved. If a target dimension is greater than or equal
/// to the source, that dimension is preserved.
///
/// Returns `(carved_rgba, final_width, final_height)`.
pub fn seam_carve_rgba8(
    src: &[u8],
    width: usize,
    height: usize,
    target_width: usize,
    target_height: usize,
) -> (Vec<u8>, usize, usize) {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return (Vec::new(), 0, 0);
    }
    let tw = target_width.clamp(1, width);
    let th = target_height.clamp(1, height);

    if tw == width && th == height {
        return (src.to_vec(), width, height);
    }

    let mut cur_w = width;
    let mut cur_h = height;
    let mut data = src.to_vec();

    // Carve vertical seams (reduce width)
    while cur_w > tw {
        data = carve_one_vertical_seam_rgba8(&data, cur_w, cur_h);
        cur_w -= 1;
    }

    // Carve horizontal seams (reduce height)
    while cur_h > th {
        data = carve_one_horizontal_seam_rgba8(&data, cur_w, cur_h);
        cur_h -= 1;
    }

    (data, cur_w, cur_h)
}

/// Carve seams from an RGBA32F buffer (`[f32]`, 4 values per pixel).
pub fn seam_carve_rgba32f(
    src: &[f32],
    width: usize,
    height: usize,
    target_width: usize,
    target_height: usize,
) -> (Vec<f32>, usize, usize) {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return (Vec::new(), 0, 0);
    }
    let tw = target_width.clamp(1, width);
    let th = target_height.clamp(1, height);

    if tw == width && th == height {
        return (src.to_vec(), width, height);
    }

    let mut cur_w = width;
    let mut cur_h = height;
    let mut data = src.to_vec();

    while cur_w > tw {
        data = carve_one_vertical_seam_rgba32f(&data, cur_w, cur_h);
        cur_w -= 1;
    }

    while cur_h > th {
        data = carve_one_horizontal_seam_rgba32f(&data, cur_w, cur_h);
        cur_h -= 1;
    }

    (data, cur_w, cur_h)
}

fn carve_one_vertical_seam_rgba8(src: &[u8], w: usize, h: usize) -> Vec<u8> {
    assert_eq!(src.len(), w * h * 4);
    assert!(w > 1);

    // 1. Calculate luminance map
    let mut luma = Vec::with_capacity(w * h);
    for chunk in src.chunks_exact(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        let a = chunk[3] as f32 / 255.0;
        let y = (0.299 * r + 0.587 * g + 0.114 * b) * a;
        luma.push(y);
    }

    // 2. Dual-gradient energy map E(x, y) = |dx| + |dy|
    let mut energy = vec![0.0f32; w * h];
    for y in 0..h {
        let y_prev = y.saturating_sub(1);
        let y_next = (y + 1).min(h - 1);
        let row_offset = y * w;
        for x in 0..w {
            let x_prev = x.saturating_sub(1);
            let x_next = (x + 1).min(w - 1);

            let dx = (luma[row_offset + x_next] - luma[row_offset + x_prev]).abs();
            let dy = (luma[y_next * w + x] - luma[y_prev * w + x]).abs();
            energy[row_offset + x] = dx + dy;
        }
    }

    // 3. Dynamic programming cumulative energy table M
    let mut dp = vec![0.0f32; w * h];
    // Copy top row
    dp[..w].copy_from_slice(&energy[..w]);

    for y in 1..h {
        let prev_row = (y - 1) * w;
        let cur_row = y * w;
        for x in 0..w {
            let left = if x > 0 { dp[prev_row + x - 1] } else { f32::INFINITY };
            let center = dp[prev_row + x];
            let right = if x + 1 < w { dp[prev_row + x + 1] } else { f32::INFINITY };
            let min_parent = center.min(left.min(right));
            dp[cur_row + x] = energy[cur_row + x] + min_parent;
        }
    }

    // 4. Backtrack seam from bottom row
    let last_row = (h - 1) * w;
    let mut min_x = 0;
    let mut min_val = dp[last_row];
    for x in 1..w {
        let val = dp[last_row + x];
        if val < min_val {
            min_val = val;
            min_x = x;
        }
    }

    let mut seam = vec![0usize; h];
    seam[h - 1] = min_x;
    for y in (0..h - 1).rev() {
        let prev_row = y * w;
        let x = seam[y + 1];
        let mut best_x = x;
        let mut best_val = dp[prev_row + x];

        if x > 0 && dp[prev_row + x - 1] < best_val {
            best_val = dp[prev_row + x - 1];
            best_x = x - 1;
        }
        if x + 1 < w && dp[prev_row + x + 1] < best_val {
            best_x = x + 1;
        }
        seam[y] = best_x;
    }

    // 5. Construct new buffer without the seam
    let new_w = w - 1;
    let mut out = Vec::with_capacity(new_w * h * 4);
    for y in 0..h {
        let remove_x = seam[y];
        let src_row = y * w * 4;
        // Copy before seam
        out.extend_from_slice(&src[src_row..src_row + remove_x * 4]);
        // Copy after seam
        out.extend_from_slice(&src[src_row + (remove_x + 1) * 4..src_row + w * 4]);
    }
    out
}

fn carve_one_horizontal_seam_rgba8(src: &[u8], w: usize, h: usize) -> Vec<u8> {
    assert_eq!(src.len(), w * h * 4);
    assert!(h > 1);

    // Transpose into (h, w), carve vertical seam, transpose back
    let mut transposed = vec![0u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let s_idx = (y * w + x) * 4;
            let t_idx = (x * h + y) * 4;
            transposed[t_idx..t_idx + 4].copy_from_slice(&src[s_idx..s_idx + 4]);
        }
    }

    let carved_transposed = carve_one_vertical_seam_rgba8(&transposed, h, w);
    let new_h = h - 1;
    let mut out = vec![0u8; w * new_h * 4];
    for x in 0..w {
        for y in 0..new_h {
            let t_idx = (x * new_h + y) * 4;
            let s_idx = (y * w + x) * 4;
            out[s_idx..s_idx + 4].copy_from_slice(&carved_transposed[t_idx..t_idx + 4]);
        }
    }
    out
}

fn carve_one_vertical_seam_rgba32f(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    assert_eq!(src.len(), w * h * 4);
    assert!(w > 1);

    let mut luma = Vec::with_capacity(w * h);
    for chunk in src.chunks_exact(4) {
        let r = chunk[0];
        let g = chunk[1];
        let b = chunk[2];
        let a = chunk[3].clamp(0.0, 1.0);
        let y = (0.299 * r + 0.587 * g + 0.114 * b) * a;
        luma.push(y);
    }

    let mut energy = vec![0.0f32; w * h];
    for y in 0..h {
        let y_prev = y.saturating_sub(1);
        let y_next = (y + 1).min(h - 1);
        let row_offset = y * w;
        for x in 0..w {
            let x_prev = x.saturating_sub(1);
            let x_next = (x + 1).min(w - 1);

            let dx = (luma[row_offset + x_next] - luma[row_offset + x_prev]).abs();
            let dy = (luma[y_next * w + x] - luma[y_prev * w + x]).abs();
            energy[row_offset + x] = dx + dy;
        }
    }

    let mut dp = vec![0.0f32; w * h];
    dp[..w].copy_from_slice(&energy[..w]);

    for y in 1..h {
        let prev_row = (y - 1) * w;
        let cur_row = y * w;
        for x in 0..w {
            let left = if x > 0 { dp[prev_row + x - 1] } else { f32::INFINITY };
            let center = dp[prev_row + x];
            let right = if x + 1 < w { dp[prev_row + x + 1] } else { f32::INFINITY };
            let min_parent = center.min(left.min(right));
            dp[cur_row + x] = energy[cur_row + x] + min_parent;
        }
    }

    let last_row = (h - 1) * w;
    let mut min_x = 0;
    let mut min_val = dp[last_row];
    for x in 1..w {
        let val = dp[last_row + x];
        if val < min_val {
            min_val = val;
            min_x = x;
        }
    }

    let mut seam = vec![0usize; h];
    seam[h - 1] = min_x;
    for y in (0..h - 1).rev() {
        let prev_row = y * w;
        let x = seam[y + 1];
        let mut best_x = x;
        let mut best_val = dp[prev_row + x];

        if x > 0 && dp[prev_row + x - 1] < best_val {
            best_val = dp[prev_row + x - 1];
            best_x = x - 1;
        }
        if x + 1 < w && dp[prev_row + x + 1] < best_val {
            best_x = x + 1;
        }
        seam[y] = best_x;
    }

    let new_w = w - 1;
    let mut out = Vec::with_capacity(new_w * h * 4);
    for y in 0..h {
        let remove_x = seam[y];
        let src_row = y * w * 4;
        out.extend_from_slice(&src[src_row..src_row + remove_x * 4]);
        out.extend_from_slice(&src[src_row + (remove_x + 1) * 4..src_row + w * 4]);
    }
    out
}

fn carve_one_horizontal_seam_rgba32f(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    assert_eq!(src.len(), w * h * 4);
    assert!(h > 1);

    let mut transposed = vec![0.0f32; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let s_idx = (y * w + x) * 4;
            let t_idx = (x * h + y) * 4;
            transposed[t_idx..t_idx + 4].copy_from_slice(&src[s_idx..s_idx + 4]);
        }
    }

    let carved_transposed = carve_one_vertical_seam_rgba32f(&transposed, h, w);
    let new_h = h - 1;
    let mut out = vec![0.0f32; w * new_h * 4];
    for x in 0..w {
        for y in 0..new_h {
            let t_idx = (x * new_h + y) * 4;
            let s_idx = (y * w + x) * 4;
            out[s_idx..s_idx + 4].copy_from_slice(&carved_transposed[t_idx..t_idx + 4]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seam_carve_flat_region_dropped_first() {
        // 8x4 image: high-contrast column in the center (x=4), flat on sides
        let w = 8;
        let h = 4;
        let mut src = vec![0u8; w * h * 4];
        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) * 4;
                let val = if x == 4 { 255 } else { 0 };
                src[idx] = val;
                src[idx + 1] = val;
                src[idx + 2] = val;
                src[idx + 3] = 255;
            }
        }

        let (carved, new_w, new_h) = seam_carve_rgba8(&src, w, h, 6, 4);
        assert_eq!(new_w, 6);
        assert_eq!(new_h, 4);
        // The high energy column at x=4 must NOT have been carved away
        let mut has_high_val = false;
        for x in 0..new_w {
            let idx = (0 * new_w + x) * 4;
            if carved[idx] == 255 {
                has_high_val = true;
                break;
            }
        }
        assert!(has_high_val, "high energy feature preserved");
    }

    #[test]
    fn test_seam_carve_identity() {
        let w = 4;
        let h = 4;
        let src = vec![128u8; w * h * 4];
        let (carved, new_w, new_h) = seam_carve_rgba8(&src, w, h, 4, 4);
        assert_eq!(new_w, 4);
        assert_eq!(new_h, 4);
        assert_eq!(carved, src);
    }

    #[test]
    fn test_seam_carve_both_axes() {
        let w = 6;
        let h = 6;
        let src = vec![200u8; w * h * 4];
        let (carved, new_w, new_h) = seam_carve_rgba8(&src, w, h, 4, 3);
        assert_eq!(new_w, 4);
        assert_eq!(new_h, 3);
        assert_eq!(carved.len(), 4 * 3 * 4);
    }
}

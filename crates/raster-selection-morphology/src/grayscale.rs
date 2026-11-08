//! Grayscale floating-point mathematical morphology, distance transforms, and skeletonization.
//!
//! Evaluates operations on caller-provided `&[f32]` row-major rasters.
//! Bounds and virtual border pixels use edge clamping.

use crate::distance::edt;

/// Operations for structuring-element morphology.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MorphologyMethod {
    Dilate,
    Erode,
    Open,
    Close,
    TopHat,
    BlackHat,
    Gradient,
    EdgeIn,
    EdgeOut,
}

/// A 2D structuring element with anchor/center offset.
#[derive(Clone, Debug, PartialEq)]
pub struct MorphologyKernel {
    pub width: usize,
    pub height: usize,
    pub center_x: isize,
    pub center_y: isize,
    /// Row-major weights. `f32::NAN` marks elements excluded from the neighborhood.
    pub values: Vec<f32>,
}

impl MorphologyKernel {
    /// Square structuring element of width `2 * radius + 1`.
    pub fn square(radius: usize) -> Self {
        let size = radius * 2 + 1;
        Self {
            width: size,
            height: size,
            center_x: radius as isize,
            center_y: radius as isize,
            values: vec![0.0; size * size],
        }
    }

    /// Circular disk structuring element of given floating-point radius.
    pub fn disk(radius: f32) -> Self {
        let r_ceil = radius.ceil().max(1.0) as usize;
        let size = r_ceil * 2 + 1;
        let center = r_ceil as isize;
        let r2 = radius * radius;
        let mut values = Vec::with_capacity(size * size);
        for y in 0..size {
            let dy = y as isize - center;
            for x in 0..size {
                let dx = x as isize - center;
                if (dx * dx + dy * dy) as f32 <= r2 {
                    values.push(0.0);
                } else {
                    values.push(f32::NAN);
                }
            }
        }
        Self {
            width: size,
            height: size,
            center_x: center,
            center_y: center,
            values,
        }
    }

    /// Diamond structuring element with Manhattan radius `|dx| + |dy| <= radius`.
    pub fn diamond(radius: usize) -> Self {
        let size = radius * 2 + 1;
        let center = radius as isize;
        let mut values = Vec::with_capacity(size * size);
        for y in 0..size {
            let dy = (y as isize - center).abs();
            for x in 0..size {
                let dx = (x as isize - center).abs();
                if (dx + dy) as usize <= radius {
                    values.push(0.0);
                } else {
                    values.push(f32::NAN);
                }
            }
        }
        Self {
            width: size,
            height: size,
            center_x: center,
            center_y: center,
            values,
        }
    }

    /// Cross structuring element (`dx == 0 || dy == 0`).
    pub fn cross(radius: usize) -> Self {
        let size = radius * 2 + 1;
        let center = radius as isize;
        let mut values = Vec::with_capacity(size * size);
        for y in 0..size {
            let dy = y as isize - center;
            for x in 0..size {
                let dx = x as isize - center;
                if dx == 0 || dy == 0 {
                    values.push(0.0);
                } else {
                    values.push(f32::NAN);
                }
            }
        }
        Self {
            width: size,
            height: size,
            center_x: center,
            center_y: center,
            values,
        }
    }

    /// Flat rectangular structuring element.
    pub fn rectangle(width: usize, height: usize) -> Self {
        let w = width.max(1);
        let h = height.max(1);
        Self {
            width: w,
            height: h,
            center_x: (w as isize - 1) / 2,
            center_y: (h as isize - 1) / 2,
            values: vec![0.0; w * h],
        }
    }

    /// Custom structuring element from row-major slice.
    pub fn from_slice(width: usize, height: usize, values: &[f32]) -> Option<Self> {
        if width == 0 || height == 0 || values.len() != width * height {
            return None;
        }
        Some(Self {
            width,
            height,
            center_x: (width as isize - 1) / 2,
            center_y: (height as isize - 1) / 2,
            values: values.to_vec(),
        })
    }
}

#[inline(always)]
fn sample_clamped(data: &[f32], width: usize, height: usize, x: isize, y: isize) -> f32 {
    let cx = x.clamp(0, width as isize - 1) as usize;
    let cy = y.clamp(0, height as isize - 1) as usize;
    data[cy * width + cx]
}

/// Grayscale morphological dilation: `out(x, y) = max (input + kernel)`.
pub fn grayscale_dilate(
    input: &[f32],
    width: usize,
    height: usize,
    kernel: &MorphologyKernel,
) -> Vec<f32> {
    if input.len() != width * height || width == 0 || height == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(input.len());
    for y in 0..height {
        for x in 0..width {
            let mut max_val = f32::NEG_INFINITY;
            for ky in 0..kernel.height {
                let sy = y as isize + ky as isize - kernel.center_y;
                for kx in 0..kernel.width {
                    let weight = kernel.values[ky * kernel.width + kx];
                    if weight.is_nan() {
                        continue;
                    }
                    let sx = x as isize + kx as isize - kernel.center_x;
                    let val = sample_clamped(input, width, height, sx, sy) + weight;
                    if val > max_val {
                        max_val = val;
                    }
                }
            }
            out.push(if max_val.is_finite() { max_val } else { input[y * width + x] });
        }
    }
    out
}

/// Grayscale morphological erosion: `out(x, y) = min (input - kernel)`.
pub fn grayscale_erode(
    input: &[f32],
    width: usize,
    height: usize,
    kernel: &MorphologyKernel,
) -> Vec<f32> {
    if input.len() != width * height || width == 0 || height == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(input.len());
    for y in 0..height {
        for x in 0..width {
            let mut min_val = f32::INFINITY;
            for ky in 0..kernel.height {
                let sy = y as isize + ky as isize - kernel.center_y;
                for kx in 0..kernel.width {
                    let weight = kernel.values[ky * kernel.width + kx];
                    if weight.is_nan() {
                        continue;
                    }
                    let sx = x as isize + kx as isize - kernel.center_x;
                    let val = sample_clamped(input, width, height, sx, sy) - weight;
                    if val < min_val {
                        min_val = val;
                    }
                }
            }
            out.push(if min_val.is_finite() { min_val } else { input[y * width + x] });
        }
    }
    out
}

/// Applies a morphological method to a 2D grayscale raster.
pub fn apply_morphology(
    input: &[f32],
    width: usize,
    height: usize,
    method: MorphologyMethod,
    kernel: &MorphologyKernel,
) -> Vec<f32> {
    match method {
        MorphologyMethod::Dilate => grayscale_dilate(input, width, height, kernel),
        MorphologyMethod::Erode => grayscale_erode(input, width, height, kernel),
        MorphologyMethod::Open => {
            let eroded = grayscale_erode(input, width, height, kernel);
            grayscale_dilate(&eroded, width, height, kernel)
        }
        MorphologyMethod::Close => {
            let dilated = grayscale_dilate(input, width, height, kernel);
            grayscale_erode(&dilated, width, height, kernel)
        }
        MorphologyMethod::TopHat => {
            let opened = apply_morphology(input, width, height, MorphologyMethod::Open, kernel);
            input.iter().zip(&opened).map(|(a, b)| (a - b).max(0.0)).collect()
        }
        MorphologyMethod::BlackHat => {
            let closed = apply_morphology(input, width, height, MorphologyMethod::Close, kernel);
            closed.iter().zip(input).map(|(c, a)| (c - a).max(0.0)).collect()
        }
        MorphologyMethod::Gradient => {
            let d = grayscale_dilate(input, width, height, kernel);
            let e = grayscale_erode(input, width, height, kernel);
            d.iter().zip(&e).map(|(a, b)| (a - b).max(0.0)).collect()
        }
        MorphologyMethod::EdgeIn => {
            let e = grayscale_erode(input, width, height, kernel);
            input.iter().zip(&e).map(|(a, b)| (a - b).max(0.0)).collect()
        }
        MorphologyMethod::EdgeOut => {
            let d = grayscale_dilate(input, width, height, kernel);
            d.iter().zip(input).map(|(a, b)| (a - b).max(0.0)).collect()
        }
    }
}

/// Euclidean distance transform on a thresholded raster mask (`>= 0.5` inside).
pub fn distance_transform_f32(mask: &[f32], width: usize, height: usize) -> Vec<f32> {
    if mask.len() != width * height || width == 0 || height == 0 {
        return Vec::new();
    }
    let inside: Vec<bool> = mask.iter().map(|&v| v >= 0.5).collect();
    edt(&inside, width, height)
}

/// Topological skeletonization via the Zhang-Suen thinning algorithm.
///
/// Returns a single-pixel wide binary skeleton mask (`1.0` for skeleton, `0.0` otherwise).
pub fn skeletonize_mask(mask: &[f32], width: usize, height: usize) -> Vec<f32> {
    if mask.len() != width * height || width == 0 || height == 0 {
        return Vec::new();
    }
    let mut binary: Vec<u8> = mask.iter().map(|&v| if v >= 0.5 { 1 } else { 0 }).collect();
    let mut to_delete = Vec::new();
    let mut changed = true;

    while changed {
        changed = false;

        // Sub-iteration 1
        for y in 1..height.saturating_sub(1) {
            for x in 1..width.saturating_sub(1) {
                let idx = y * width + x;
                if binary[idx] == 0 {
                    continue;
                }

                // Clockwise neighborhood:
                // P2 P3 P4
                // P9 P1 P5
                // P8 P7 P6
                let p2 = binary[(y - 1) * width + x];
                let p3 = binary[(y - 1) * width + (x + 1)];
                let p4 = binary[y * width + (x + 1)];
                let p5 = binary[(y + 1) * width + (x + 1)];
                let p6 = binary[(y + 1) * width + x];
                let p7 = binary[(y + 1) * width + (x - 1)];
                let p8 = binary[y * width + (x - 1)];
                let p9 = binary[(y - 1) * width + (x - 1)];

                let b = p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9;
                if !(2..=6).contains(&b) {
                    continue;
                }

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9, p2];
                let mut transitions = 0;
                for i in 0..8 {
                    if neighbors[i] == 0 && neighbors[i + 1] == 1 {
                        transitions += 1;
                    }
                }
                if transitions != 1 {
                    continue;
                }

                if p2 * p4 * p6 == 0 && p4 * p6 * p8 == 0 {
                    to_delete.push(idx);
                }
            }
        }

        if !to_delete.is_empty() {
            changed = true;
            for &idx in &to_delete {
                binary[idx] = 0;
            }
            to_delete.clear();
        }

        // Sub-iteration 2
        for y in 1..height.saturating_sub(1) {
            for x in 1..width.saturating_sub(1) {
                let idx = y * width + x;
                if binary[idx] == 0 {
                    continue;
                }

                let p2 = binary[(y - 1) * width + x];
                let p3 = binary[(y - 1) * width + (x + 1)];
                let p4 = binary[y * width + (x + 1)];
                let p5 = binary[(y + 1) * width + (x + 1)];
                let p6 = binary[(y + 1) * width + x];
                let p7 = binary[(y + 1) * width + (x - 1)];
                let p8 = binary[y * width + (x - 1)];
                let p9 = binary[(y - 1) * width + (x - 1)];

                let b = p2 + p3 + p4 + p5 + p6 + p7 + p8 + p9;
                if !(2..=6).contains(&b) {
                    continue;
                }

                let neighbors = [p2, p3, p4, p5, p6, p7, p8, p9, p2];
                let mut transitions = 0;
                for i in 0..8 {
                    if neighbors[i] == 0 && neighbors[i + 1] == 1 {
                        transitions += 1;
                    }
                }
                if transitions != 1 {
                    continue;
                }

                if p2 * p4 * p8 == 0 && p2 * p6 * p8 == 0 {
                    to_delete.push(idx);
                }
            }
        }

        if !to_delete.is_empty() {
            changed = true;
            for &idx in &to_delete {
                binary[idx] = 0;
            }
            to_delete.clear();
        }
    }

    binary.into_iter().map(|v| v as f32).collect()
}

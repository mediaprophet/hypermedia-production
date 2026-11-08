//! Convolve: Separable 2D Gaussian blur, unsharp mask, Sobel/Laplacian edge detection,
//! and general 2D kernel convolution.

use std::f64::consts::PI;

/// A 2D convolution kernel with width, height, anchor origin, and row-major weights.
#[derive(Clone, Debug, PartialEq)]
pub struct Kernel2D {
    pub width: usize,
    pub height: usize,
    pub x_origin: usize,
    pub y_origin: usize,
    pub weights: Vec<f64>,
}

impl Kernel2D {
    pub fn new(width: usize, height: usize, weights: Vec<f64>) -> Self {
        assert_eq!(weights.len(), width * height);
        Self {
            width,
            height,
            x_origin: width / 2,
            y_origin: height / 2,
            weights,
        }
    }

    pub fn with_origin(
        width: usize,
        height: usize,
        x_origin: usize,
        y_origin: usize,
        weights: Vec<f64>,
    ) -> Self {
        assert_eq!(weights.len(), width * height);
        Self {
            width,
            height,
            x_origin,
            y_origin,
            weights,
        }
    }

    /// Normalize kernel weights so their sum is 1.0 (if sum is non-zero).
    pub fn normalized(&self) -> Self {
        let sum: f64 = self.weights.iter().sum();
        if sum.abs() > 1e-12 {
            let weights = self.weights.iter().map(|&w| w / sum).collect();
            Self {
                width: self.width,
                height: self.height,
                x_origin: self.x_origin,
                y_origin: self.y_origin,
                weights,
            }
        } else {
            self.clone()
        }
    }
}

/// Create a 1D Gaussian kernel with radius and standard deviation sigma.
pub fn gaussian_kernel_1d(radius: usize, sigma: f64) -> Vec<f64> {
    let s = sigma.abs().max(1e-4);
    let size = 2 * radius + 1;
    let mut kernel = Vec::with_capacity(size);
    let factor = 1.0 / ((2.0 * PI).sqrt() * s);
    let denom = 2.0 * s * s;

    let mut sum = 0.0;
    for i in 0..size {
        let x = (i as isize - radius as isize) as f64;
        let v = factor * (-x * x / denom).exp();
        kernel.push(v);
        sum += v;
    }
    if sum > 0.0 {
        for v in &mut kernel {
            *v /= sum;
        }
    }
    kernel
}

/// General 2D convolution for RGBA8 image buffer.
pub fn convolve_rgba8(src: &[u8], width: usize, height: usize, kernel: &Kernel2D) -> Vec<u8> {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    let mut out = vec![0u8; width * height * 4];
    let kw = kernel.width;
    let kh = kernel.height;
    let ox = kernel.x_origin as isize;
    let oy = kernel.y_origin as isize;

    for y in 0..height {
        let out_row = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;

            for ky in 0..kh {
                let sy = (y as isize + ky as isize - oy).clamp(0, height as isize - 1) as usize;
                let k_row = ky * kw;
                let s_row = sy * width * 4;
                for kx in 0..kw {
                    let sx = (x as isize + kx as isize - ox).clamp(0, width as isize - 1) as usize;
                    let weight = kernel.weights[k_row + kx];
                    let idx = s_row + sx * 4;

                    sum_r += src[idx] as f64 * weight;
                    sum_g += src[idx + 1] as f64 * weight;
                    sum_b += src[idx + 2] as f64 * weight;
                    sum_a += src[idx + 3] as f64 * weight;
                }
            }

            let idx = out_row + x * 4;
            out[idx] = sum_r.round().clamp(0.0, 255.0) as u8;
            out[idx + 1] = sum_g.round().clamp(0.0, 255.0) as u8;
            out[idx + 2] = sum_b.round().clamp(0.0, 255.0) as u8;
            out[idx + 3] = sum_a.round().clamp(0.0, 255.0) as u8;
        }
    }
    out
}

/// General 2D convolution for RGBA32F image buffer.
pub fn convolve_rgba32f(src: &[f32], width: usize, height: usize, kernel: &Kernel2D) -> Vec<f32> {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    let mut out = vec![0.0f32; width * height * 4];
    let kw = kernel.width;
    let kh = kernel.height;
    let ox = kernel.x_origin as isize;
    let oy = kernel.y_origin as isize;

    for y in 0..height {
        let out_row = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;

            for ky in 0..kh {
                let sy = (y as isize + ky as isize - oy).clamp(0, height as isize - 1) as usize;
                let k_row = ky * kw;
                let s_row = sy * width * 4;
                for kx in 0..kw {
                    let sx = (x as isize + kx as isize - ox).clamp(0, width as isize - 1) as usize;
                    let weight = kernel.weights[k_row + kx];
                    let idx = s_row + sx * 4;

                    sum_r += src[idx] as f64 * weight;
                    sum_g += src[idx + 1] as f64 * weight;
                    sum_b += src[idx + 2] as f64 * weight;
                    sum_a += src[idx + 3] as f64 * weight;
                }
            }

            let idx = out_row + x * 4;
            out[idx] = sum_r as f32;
            out[idx + 1] = sum_g as f32;
            out[idx + 2] = sum_b as f32;
            out[idx + 3] = sum_a as f32;
        }
    }
    out
}

/// Separable 2D Gaussian blur for RGBA8 buffer.
/// Runs in O(2*radius) per pixel rather than O(radius^2).
pub fn gaussian_blur_rgba8(src: &[u8], width: usize, height: usize, radius: f64, sigma: f64) -> Vec<u8> {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    let r = radius.round().max(1.0) as usize;
    let kernel = gaussian_kernel_1d(r, sigma);
    let k_size = kernel.len();

    // 1. Horizontal pass
    let mut horiz = vec![0.0f64; width * height * 4];
    for y in 0..height {
        let row_idx = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;
            for (k, &w) in kernel.iter().enumerate() {
                let sx = (x as isize + k as isize - r as isize).clamp(0, width as isize - 1) as usize;
                let idx = row_idx + sx * 4;
                sum_r += src[idx] as f64 * w;
                sum_g += src[idx + 1] as f64 * w;
                sum_b += src[idx + 2] as f64 * w;
                sum_a += src[idx + 3] as f64 * w;
            }
            let out_idx = row_idx + x * 4;
            horiz[out_idx] = sum_r;
            horiz[out_idx + 1] = sum_g;
            horiz[out_idx + 2] = sum_b;
            horiz[out_idx + 3] = sum_a;
        }
    }

    // 2. Vertical pass
    let mut out = vec![0u8; width * height * 4];
    for y in 0..height {
        let out_row = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;
            for (k, &w) in kernel.iter().enumerate() {
                let sy = (y as isize + k as isize - r as isize).clamp(0, height as isize - 1) as usize;
                let idx = sy * width * 4 + x * 4;
                sum_r += horiz[idx] * w;
                sum_g += horiz[idx + 1] * w;
                sum_b += horiz[idx + 2] * w;
                sum_a += horiz[idx + 3] * w;
            }
            let idx = out_row + x * 4;
            out[idx] = sum_r.round().clamp(0.0, 255.0) as u8;
            out[idx + 1] = sum_g.round().clamp(0.0, 255.0) as u8;
            out[idx + 2] = sum_b.round().clamp(0.0, 255.0) as u8;
            out[idx + 3] = sum_a.round().clamp(0.0, 255.0) as u8;
        }
    }
    out
}

/// Separable 2D Gaussian blur for RGBA32F buffer.
pub fn gaussian_blur_rgba32f(src: &[f32], width: usize, height: usize, radius: f64, sigma: f64) -> Vec<f32> {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    let r = radius.round().max(1.0) as usize;
    let kernel = gaussian_kernel_1d(r, sigma);

    let mut horiz = vec![0.0f32; width * height * 4];
    for y in 0..height {
        let row_idx = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;
            for (k, &w) in kernel.iter().enumerate() {
                let sx = (x as isize + k as isize - r as isize).clamp(0, width as isize - 1) as usize;
                let idx = row_idx + sx * 4;
                let wf = w as f32;
                sum_r += src[idx] * wf;
                sum_g += src[idx + 1] * wf;
                sum_b += src[idx + 2] * wf;
                sum_a += src[idx + 3] * wf;
            }
            let out_idx = row_idx + x * 4;
            horiz[out_idx] = sum_r;
            horiz[out_idx + 1] = sum_g;
            horiz[out_idx + 2] = sum_b;
            horiz[out_idx + 3] = sum_a;
        }
    }

    let mut out = vec![0.0f32; width * height * 4];
    for y in 0..height {
        let out_row = y * width * 4;
        for x in 0..width {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;
            for (k, &w) in kernel.iter().enumerate() {
                let sy = (y as isize + k as isize - r as isize).clamp(0, height as isize - 1) as usize;
                let idx = sy * width * 4 + x * 4;
                let wf = w as f32;
                sum_r += horiz[idx] * wf;
                sum_g += horiz[idx + 1] * wf;
                sum_b += horiz[idx + 2] * wf;
                sum_a += horiz[idx + 3] * wf;
            }
            let idx = out_row + x * 4;
            out[idx] = sum_r;
            out[idx + 1] = sum_g;
            out[idx + 2] = sum_b;
            out[idx + 3] = sum_a;
        }
    }
    out
}

/// Unsharp mask sharpening for RGBA8 buffer.
/// `sharp = original + gain * (original - blurred)` for differences >= `threshold`.
pub fn unsharp_mask_rgba8(
    src: &[u8],
    width: usize,
    height: usize,
    radius: f64,
    sigma: f64,
    gain: f64,
    threshold: f64,
) -> Vec<u8> {
    let blurred = gaussian_blur_rgba8(src, width, height, radius, sigma);
    let mut out = vec![0u8; src.len()];

    for i in (0..src.len()).step_by(4) {
        for c in 0..3 {
            let orig = src[i + c] as f64;
            let blur = blurred[i + c] as f64;
            let diff = orig - blur;
            if diff.abs() >= threshold {
                let sharp = orig + gain * diff;
                out[i + c] = sharp.round().clamp(0.0, 255.0) as u8;
            } else {
                out[i + c] = src[i + c];
            }
        }
        out[i + 3] = src[i + 3];
    }
    out
}

/// Sobel gradient magnitude edge detection for RGBA8 buffer.
pub fn sobel_edge_rgba8(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let kx = Kernel2D::new(
        3,
        3,
        vec![
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0,
        ],
    );
    let ky = Kernel2D::new(
        3,
        3,
        vec![
            -1.0, -2.0, -1.0,
             0.0,  0.0,  0.0,
             1.0,  2.0,  1.0,
        ],
    );

    let mut out = vec![0u8; width * height * 4];
    for y in 0..height {
        let row_idx = y * width * 4;
        for x in 0..width {
            let mut gx = 0.0;
            let mut gy = 0.0;
            for dy in 0..3 {
                let sy = (y as isize + dy as isize - 1).clamp(0, height as isize - 1) as usize;
                for dx in 0..3 {
                    let sx = (x as isize + dx as isize - 1).clamp(0, width as isize - 1) as usize;
                    let idx = (sy * width + sx) * 4;
                    let luma = 0.299 * src[idx] as f64 + 0.587 * src[idx + 1] as f64 + 0.114 * src[idx + 2] as f64;
                    gx += luma * kx.weights[dy * 3 + dx];
                    gy += luma * ky.weights[dy * 3 + dx];
                }
            }
            let mag = gx.hypot(gy).round().clamp(0.0, 255.0) as u8;
            let idx = row_idx + x * 4;
            out[idx] = mag;
            out[idx + 1] = mag;
            out[idx + 2] = mag;
            out[idx + 3] = 255;
        }
    }
    out
}

/// Laplacian 2D second-derivative edge detector for RGBA8 buffer.
pub fn laplacian_edge_rgba8(src: &[u8], width: usize, height: usize) -> Vec<u8> {
    let kernel = Kernel2D::new(
        3,
        3,
        vec![
            0.0,  1.0, 0.0,
            1.0, -4.0, 1.0,
            0.0,  1.0, 0.0,
        ],
    );
    let mut out = vec![0u8; width * height * 4];
    for y in 0..height {
        let row_idx = y * width * 4;
        for x in 0..width {
            let mut sum = 0.0;
            for dy in 0..3 {
                let sy = (y as isize + dy as isize - 1).clamp(0, height as isize - 1) as usize;
                for dx in 0..3 {
                    let sx = (x as isize + dx as isize - 1).clamp(0, width as isize - 1) as usize;
                    let idx = (sy * width + sx) * 4;
                    let luma = 0.299 * src[idx] as f64 + 0.587 * src[idx + 1] as f64 + 0.114 * src[idx + 2] as f64;
                    sum += luma * kernel.weights[dy * 3 + dx];
                }
            }
            let val = sum.abs().round().clamp(0.0, 255.0) as u8;
            let idx = row_idx + x * 4;
            out[idx] = val;
            out[idx + 1] = val;
            out[idx + 2] = val;
            out[idx + 3] = 255;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_convolution() {
        let w = 4;
        let h = 4;
        let src = vec![100u8; w * h * 4];
        let id_kernel = Kernel2D::new(1, 1, vec![1.0]);
        let res = convolve_rgba8(&src, w, h, &id_kernel);
        assert_eq!(res, src);
    }

    #[test]
    fn test_gaussian_blur_conservation() {
        let w = 8;
        let h = 8;
        let src = vec![200u8; w * h * 4];
        let blurred = gaussian_blur_rgba8(&src, w, h, 2.0, 1.0);
        assert_eq!(blurred.len(), src.len());
        assert!(blurred.iter().all(|&v| v == 200));
    }

    #[test]
    fn test_sobel_edge_detection() {
        // Step edge: left half black, right half white
        let w = 8;
        let h = 8;
        let mut src = vec![0u8; w * h * 4];
        for y in 0..h {
            for x in 4..w {
                let idx = (y * w + x) * 4;
                src[idx] = 255;
                src[idx + 1] = 255;
                src[idx + 2] = 255;
                src[idx + 3] = 255;
            }
        }
        let edges = sobel_edge_rgba8(&src, w, h);
        // Pixels near x=3 and x=4 should show strong edge gradient
        let edge_val = edges[(4 * w + 3) * 4];
        assert!(edge_val > 100, "edge detected at step: {edge_val}");
    }
}

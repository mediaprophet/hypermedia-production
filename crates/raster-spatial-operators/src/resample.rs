//! Filter-bank image resampling.
//!
//! Provides separable two-pass filter-bank resampling supporting:
//! `MitchellNetravali`, `CatmullRom`, `Lanczos3`, `Kaiser`, `Blackman`, `Bessel`,
//! plus `Box` and `Triangle`.

use std::f64::consts::PI;

/// Resampling filter kernel types.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResampleFilter {
    Box,
    Triangle,
    #[default]
    MitchellNetravali,
    CatmullRom,
    Lanczos3,
    Kaiser,
    Blackman,
    Bessel,
}

impl ResampleFilter {
    /// Spatial support radius in pixels.
    pub fn support(self) -> f64 {
        match self {
            Self::Box => 0.5,
            Self::Triangle => 1.0,
            Self::MitchellNetravali | Self::CatmullRom => 2.0,
            Self::Lanczos3 | Self::Kaiser | Self::Blackman => 3.0,
            Self::Bessel => 3.2383,
        }
    }

    /// Continuous weighting function at distance `x`.
    pub fn weight(self, x: f64) -> f64 {
        let x = x.abs();
        let s = self.support();
        if x >= s {
            return 0.0;
        }
        match self {
            Self::Box => 1.0,
            Self::Triangle => 1.0 - x,
            Self::MitchellNetravali => cubic_bc(x, 1.0 / 3.0, 1.0 / 3.0),
            Self::CatmullRom => cubic_bc(x, 0.0, 0.5),
            Self::Lanczos3 => sinc(x) * sinc(x / 3.0),
            Self::Kaiser => {
                const ALPHA: f64 = 6.5;
                let t = x / 3.0;
                let kaiser_w = if t < 1.0 {
                    bessel_i0(ALPHA * (1.0 - t * t).sqrt()) / bessel_i0(ALPHA)
                } else {
                    0.0
                };
                sinc(x) * kaiser_w
            }
            Self::Blackman => {
                let w = 0.42 + 0.5 * (PI * x / 3.0).cos() + 0.08 * (2.0 * PI * x / 3.0).cos();
                sinc(x) * w
            }
            Self::Bessel => {
                if x < 1e-6 {
                    PI / 4.0
                } else {
                    bessel_j1(PI * x) / (2.0 * x)
                }
            }
        }
    }
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-6 {
        1.0
    } else {
        let px = PI * x;
        px.sin() / px
    }
}

fn cubic_bc(x: f64, b: f64, c: f64) -> f64 {
    let two_b = b + b;
    let k0 = 1.0 - (1.0 / 3.0) * b;
    let k1 = -3.0 + two_b + c;
    let k2 = 2.0 - 1.5 * b - c;
    let k3 = (4.0 / 3.0) * b + 4.0 * c;
    let k4 = -8.0 * c - two_b;
    let k5 = b + 5.0 * c;
    let k6 = (-1.0 / 6.0) * b - c;

    if x < 1.0 {
        k0 + x * (x * (k1 + x * k2))
    } else if x < 2.0 {
        k3 + x * (k4 + x * (k5 + x * k6))
    } else {
        0.0
    }
}

/// Zeroth-order modified Bessel function of the first kind I_0(x).
fn bessel_i0(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    let half_x = x / 2.0;
    for k in 1..25 {
        term *= (half_x / k as f64) * (half_x / k as f64);
        sum += term;
        if term < 1e-12 * sum {
            break;
        }
    }
    sum
}

/// First-order Bessel function of the first kind J_1(x).
fn bessel_j1(x: f64) -> f64 {
    let mut sum = 0.0;
    let half_x = x / 2.0;
    let mut term = half_x;
    sum += term;
    for k in 1..25 {
        term *= -1.0 * (half_x / k as f64) * (half_x / (k + 1) as f64);
        sum += term;
        if term.abs() < 1e-12 * sum.abs() {
            break;
        }
    }
    sum
}

#[derive(Clone)]
struct Contribution {
    start: usize,
    weights: Vec<f64>,
}

fn contributions(source_len: usize, target_len: usize, filter: ResampleFilter) -> Vec<Contribution> {
    let factor = target_len as f64 / source_len as f64;
    let (scale, support) = if factor < 1.0 {
        let s = 1.0 / factor;
        (s, s * filter.support())
    } else {
        (1.0, filter.support())
    };

    let mut result = Vec::with_capacity(target_len);
    for idx in 0..target_len {
        let center = (idx as f64 + 0.5) / factor;
        let start = (center - support).floor().max(0.0) as usize;
        let stop = ((center + support).ceil() as usize).min(source_len);

        let mut weights = Vec::with_capacity(stop.saturating_sub(start));
        let mut density = 0.0;

        for p in start..stop {
            let dist = (p as f64 + 0.5 - center) / scale;
            let w = filter.weight(dist);
            weights.push(w);
            density += w;
        }

        if density.abs() > 1e-12 {
            let inv = 1.0 / density;
            for w in &mut weights {
                *w *= inv;
            }
        }

        result.push(Contribution { start, weights });
    }
    result
}

/// Resample an RGBA8 buffer to new target dimensions.
pub fn resample_rgba8(
    src: &[u8],
    width: usize,
    height: usize,
    target_width: usize,
    target_height: usize,
    filter: ResampleFilter,
) -> Vec<u8> {
    if width == 0 || height == 0 || target_width == 0 || target_height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    if width == target_width && height == target_height {
        return src.to_vec();
    }

    // Pass 1: Horizontal resize
    let h_contrib = contributions(width, target_width, filter);
    let mut intermediate = vec![0.0f64; target_width * height * 4];

    for y in 0..height {
        let in_row = y * width * 4;
        let out_row = y * target_width * 4;
        for (x, c) in h_contrib.iter().enumerate() {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;

            for (k, &w) in c.weights.iter().enumerate() {
                let px = in_row + (c.start + k) * 4;
                sum_r += src[px] as f64 * w;
                sum_g += src[px + 1] as f64 * w;
                sum_b += src[px + 2] as f64 * w;
                sum_a += src[px + 3] as f64 * w;
            }

            let idx = out_row + x * 4;
            intermediate[idx] = sum_r;
            intermediate[idx + 1] = sum_g;
            intermediate[idx + 2] = sum_b;
            intermediate[idx + 3] = sum_a;
        }
    }

    // Pass 2: Vertical resize
    let v_contrib = contributions(height, target_height, filter);
    let mut out = vec![0u8; target_width * target_height * 4];

    for y in 0..target_height {
        let c = &v_contrib[y];
        let out_row = y * target_width * 4;
        for x in 0..target_width {
            let mut sum_r = 0.0;
            let mut sum_g = 0.0;
            let mut sum_b = 0.0;
            let mut sum_a = 0.0;

            for (k, &w) in c.weights.iter().enumerate() {
                let py = (c.start + k) * target_width * 4 + x * 4;
                sum_r += intermediate[py] * w;
                sum_g += intermediate[py + 1] * w;
                sum_b += intermediate[py + 2] * w;
                sum_a += intermediate[py + 3] * w;
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

/// Resample an RGBA32F buffer to new target dimensions.
pub fn resample_rgba32f(
    src: &[f32],
    width: usize,
    height: usize,
    target_width: usize,
    target_height: usize,
    filter: ResampleFilter,
) -> Vec<f32> {
    if width == 0 || height == 0 || target_width == 0 || target_height == 0 || src.len() != width * height * 4 {
        return Vec::new();
    }
    if width == target_width && height == target_height {
        return src.to_vec();
    }

    let h_contrib = contributions(width, target_width, filter);
    let mut intermediate = vec![0.0f32; target_width * height * 4];

    for y in 0..height {
        let in_row = y * width * 4;
        let out_row = y * target_width * 4;
        for (x, c) in h_contrib.iter().enumerate() {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;

            for (k, &w) in c.weights.iter().enumerate() {
                let px = in_row + (c.start + k) * 4;
                let wf = w as f32;
                sum_r += src[px] * wf;
                sum_g += src[px + 1] * wf;
                sum_b += src[px + 2] * wf;
                sum_a += src[px + 3] * wf;
            }

            let idx = out_row + x * 4;
            intermediate[idx] = sum_r;
            intermediate[idx + 1] = sum_g;
            intermediate[idx + 2] = sum_b;
            intermediate[idx + 3] = sum_a;
        }
    }

    let v_contrib = contributions(height, target_height, filter);
    let mut out = vec![0.0f32; target_width * target_height * 4];

    for y in 0..target_height {
        let c = &v_contrib[y];
        let out_row = y * target_width * 4;
        for x in 0..target_width {
            let mut sum_r = 0.0f32;
            let mut sum_g = 0.0f32;
            let mut sum_b = 0.0f32;
            let mut sum_a = 0.0f32;

            for (k, &w) in c.weights.iter().enumerate() {
                let py = (c.start + k) * target_width * 4 + x * 4;
                let wf = w as f32;
                sum_r += intermediate[py] * wf;
                sum_g += intermediate[py + 1] * wf;
                sum_b += intermediate[py + 2] * wf;
                sum_a += intermediate[py + 3] * wf;
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

/// Compute target dimensions to fit aspect ratio inside bounding box.
pub fn fit_size(width: usize, height: usize, box_width: usize, box_height: usize) -> (usize, usize) {
    if width == 0 || height == 0 || box_width == 0 || box_height == 0 {
        return (0, 0);
    }
    let sx = box_width as f64 / width as f64;
    let sy = box_height as f64 / height as f64;
    let scale = sx.min(sy);
    let fw = ((scale * width as f64).round() as usize).max(1);
    let fh = ((scale * height as f64).round() as usize).max(1);
    (fw, fh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resample_uniform_conservation() {
        let filters = [
            ResampleFilter::MitchellNetravali,
            ResampleFilter::CatmullRom,
            ResampleFilter::Lanczos3,
            ResampleFilter::Kaiser,
            ResampleFilter::Blackman,
            ResampleFilter::Bessel,
        ];
        let w = 8;
        let h = 8;
        let src = vec![180u8; w * h * 4];

        for &f in &filters {
            let res = resample_rgba8(&src, w, h, 4, 12, f);
            assert_eq!(res.len(), 4 * 12 * 4);
            assert!(
                res.iter().all(|&v| (v as i32 - 180).abs() <= 1),
                "filter {:?} violated energy conservation: got {:?}",
                f,
                &res[..4]
            );
        }
    }

    #[test]
    fn test_fit_size() {
        let (w, h) = fit_size(800, 600, 400, 400);
        assert_eq!((w, h), (400, 300));
    }
}

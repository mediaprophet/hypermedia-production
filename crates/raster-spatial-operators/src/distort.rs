//! Geometric image distortions: Affine, Perspective, Arc, Polar/DePolar, Barrel/Pincushion.
//!
//! Provides backward/inverse mapping from destination pixels into the source
//! image with Bilinear or Bicubic sampling and configurable virtual pixel policies.

use std::f64::consts::{PI, TAU};

/// Virtual pixel border handling policy when sampling outside the source image bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VirtualPixelMode {
    #[default]
    Edge,
    Transparent,
    Tile,
    Mirror,
}

/// Image interpolation algorithm for non-integer sample positions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Interpolation {
    #[default]
    Bilinear,
    Bicubic,
}

/// Geometric distortion mapping transform.
#[derive(Clone, Debug, PartialEq)]
pub enum DistortTransform {
    /// 2x3 Affine backward mapping matrix:
    /// `u = c[0]*x + c[1]*y + c[2]`
    /// `v = c[3]*x + c[4]*y + c[5]`
    Affine([f64; 6]),

    /// 8-DOF Projective / Perspective homography matrix:
    /// `u = (c[0]*x + c[1]*y + c[2]) / (c[6]*x + c[7]*y + 1.0)`
    /// `v = (c[3]*x + c[4]*y + c[5]) / (c[6]*x + c[7]*y + 1.0)`
    Perspective([f64; 8]),

    /// Arc distortion: bends source image around a circular ring sector.
    Arc {
        radius_inner: f64,
        radius_outer: f64,
        angle_deg: f64,
        rotation_deg: f64,
    },

    /// Polar: Maps Cartesian coordinates to Polar (angle, radius).
    Polar {
        center: (f64, f64),
        radius: f64,
    },

    /// DePolar: Maps Polar (angle, radius) back to Cartesian coordinates.
    DePolar {
        center: (f64, f64),
        radius: f64,
    },

    /// Barrel / Pincushion radial optical lens distortion:
    /// `r_src = r_dst * (a*r^3 + b*r^2 + c*r + d)`
    Barrel {
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        center: Option<(f64, f64)>,
    },
}

impl DistortTransform {
    /// Maps destination coordinate `(dst_x, dst_y)` back to source coordinate `Option<(src_x, src_y)>`.
    pub fn map_inverse(
        &self,
        dst_x: f64,
        dst_y: f64,
        src_w: f64,
        src_h: f64,
        dst_w: f64,
        dst_h: f64,
    ) -> Option<(f64, f64)> {
        match self {
            Self::Affine(c) => {
                let u = c[0] * dst_x + c[1] * dst_y + c[2];
                let v = c[3] * dst_x + c[4] * dst_y + c[5];
                Some((u, v))
            }
            Self::Perspective(c) => {
                let denom = c[6] * dst_x + c[7] * dst_y + 1.0;
                if denom.abs() < 1e-12 {
                    return None;
                }
                let u = (c[0] * dst_x + c[1] * dst_y + c[2]) / denom;
                let v = (c[3] * dst_x + c[4] * dst_y + c[5]) / denom;
                Some((u, v))
            }
            Self::Polar { center, radius } => {
                let dx = dst_x - center.0;
                let dy = dst_y - center.1;
                let r = dx.hypot(dy);
                let mut theta = dy.atan2(dx);
                if theta < 0.0 {
                    theta += TAU;
                }
                let u = (theta / TAU) * src_w;
                let v = (r / radius.max(1e-6)) * src_h;
                Some((u, v))
            }
            Self::DePolar { center, radius } => {
                let norm_x = (dst_x / dst_w.max(1.0)).clamp(0.0, 1.0);
                let norm_y = (dst_y / dst_h.max(1.0)).clamp(0.0, 1.0);
                let theta = norm_x * TAU;
                let r = norm_y * radius;
                let u = center.0 + r * theta.cos();
                let v = center.1 + r * theta.sin();
                Some((u, v))
            }
            Self::Arc {
                radius_inner,
                radius_outer,
                angle_deg,
                rotation_deg,
            } => {
                let cx = dst_w / 2.0;
                let cy = dst_h / 2.0;
                let dx = dst_x - cx;
                let dy = dst_y - cy;
                let r = dx.hypot(dy);
                let mut angle = dy.atan2(dx) - rotation_deg.to_radians();
                while angle < -PI {
                    angle += TAU;
                }
                while angle > PI {
                    angle -= TAU;
                }
                let span = angle_deg.to_radians().abs().max(1e-6);
                let half_span = span / 2.0;
                if angle < -half_span || angle > half_span {
                    return None;
                }
                let u = ((angle + half_span) / span) * src_w;
                let dr = (radius_outer - radius_inner).abs().max(1e-6);
                let v = ((r - radius_inner) / dr) * src_h;
                Some((u, v))
            }
            Self::Barrel {
                a,
                b,
                c,
                d,
                center,
            } => {
                let (cx, cy) = center.unwrap_or((dst_w / 2.0, dst_h / 2.0));
                let scale = cx.min(cy).max(1.0);
                let dx = (dst_x - cx) / scale;
                let dy = (dst_y - cy) / scale;
                let r = dx.hypot(dy);
                let r2 = r * r;
                let r3 = r2 * r;
                let poly = a * r3 + b * r2 + c * r + d;
                let u = cx + dx * poly * scale;
                let v = cy + dy * poly * scale;
                Some((u, v))
            }
        }
    }
}

/// Distort an RGBA8 buffer.
pub fn distort_rgba8(
    src: &[u8],
    width: usize,
    height: usize,
    transform: &DistortTransform,
    out_width: usize,
    out_height: usize,
    virtual_pixel: VirtualPixelMode,
    interp: Interpolation,
) -> Vec<u8> {
    if width == 0 || height == 0 || out_width == 0 || out_height == 0 {
        return Vec::new();
    }
    let mut out = vec![0u8; out_width * out_height * 4];
    let sw = width as f64;
    let sh = height as f64;
    let dw = out_width as f64;
    let dh = out_height as f64;

    for y in 0..out_height {
        let dy = y as f64 + 0.5;
        let row_idx = y * out_width * 4;
        for x in 0..out_width {
            let dx = x as f64 + 0.5;
            let pixel_idx = row_idx + x * 4;

            let sample = match transform.map_inverse(dx, dy, sw, sh, dw, dh) {
                Some((sx, sy)) => sample_rgba8(src, width, height, sx - 0.5, sy - 0.5, virtual_pixel, interp),
                None => sample_virtual_rgba8(src, width, height, -1, -1, virtual_pixel),
            };

            out[pixel_idx..pixel_idx + 4].copy_from_slice(&sample);
        }
    }
    out
}

/// Distort an RGBA32F buffer.
pub fn distort_rgba32f(
    src: &[f32],
    width: usize,
    height: usize,
    transform: &DistortTransform,
    out_width: usize,
    out_height: usize,
    virtual_pixel: VirtualPixelMode,
    interp: Interpolation,
) -> Vec<f32> {
    if width == 0 || height == 0 || out_width == 0 || out_height == 0 {
        return Vec::new();
    }
    let mut out = vec![0.0f32; out_width * out_height * 4];
    let sw = width as f64;
    let sh = height as f64;
    let dw = out_width as f64;
    let dh = out_height as f64;

    for y in 0..out_height {
        let dy = y as f64 + 0.5;
        let row_idx = y * out_width * 4;
        for x in 0..out_width {
            let dx = x as f64 + 0.5;
            let pixel_idx = row_idx + x * 4;

            let sample = match transform.map_inverse(dx, dy, sw, sh, dw, dh) {
                Some((sx, sy)) => sample_rgba32f(src, width, height, sx - 0.5, sy - 0.5, virtual_pixel, interp),
                None => sample_virtual_rgba32f(src, width, height, -1, -1, virtual_pixel),
            };

            out[pixel_idx..pixel_idx + 4].copy_from_slice(&sample);
        }
    }
    out
}

fn sample_virtual_rgba8(
    src: &[u8],
    w: usize,
    h: usize,
    x: isize,
    y: isize,
    vp: VirtualPixelMode,
) -> [u8; 4] {
    if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
        let idx = (y as usize * w + x as usize) * 4;
        return [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]];
    }
    match vp {
        VirtualPixelMode::Transparent => [0, 0, 0, 0],
        VirtualPixelMode::Edge => {
            let cx = x.clamp(0, w as isize - 1) as usize;
            let cy = y.clamp(0, h as isize - 1) as usize;
            let idx = (cy * w + cx) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
        VirtualPixelMode::Tile => {
            let cx = x.rem_euclid(w as isize) as usize;
            let cy = y.rem_euclid(h as isize) as usize;
            let idx = (cy * w + cx) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
        VirtualPixelMode::Mirror => {
            let mut mx = x.abs();
            let mut my = y.abs();
            let w2 = 2 * w as isize;
            let h2 = 2 * h as isize;
            mx %= w2;
            my %= h2;
            if mx >= w as isize {
                mx = w2 - 1 - mx;
            }
            if my >= h as isize {
                my = h2 - 1 - my;
            }
            let idx = (my as usize * w + mx as usize) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
    }
}

fn sample_virtual_rgba32f(
    src: &[f32],
    w: usize,
    h: usize,
    x: isize,
    y: isize,
    vp: VirtualPixelMode,
) -> [f32; 4] {
    if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
        let idx = (y as usize * w + x as usize) * 4;
        return [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]];
    }
    match vp {
        VirtualPixelMode::Transparent => [0.0, 0.0, 0.0, 0.0],
        VirtualPixelMode::Edge => {
            let cx = x.clamp(0, w as isize - 1) as usize;
            let cy = y.clamp(0, h as isize - 1) as usize;
            let idx = (cy * w + cx) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
        VirtualPixelMode::Tile => {
            let cx = x.rem_euclid(w as isize) as usize;
            let cy = y.rem_euclid(h as isize) as usize;
            let idx = (cy * w + cx) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
        VirtualPixelMode::Mirror => {
            let mut mx = x.abs();
            let mut my = y.abs();
            let w2 = 2 * w as isize;
            let h2 = 2 * h as isize;
            mx %= w2;
            my %= h2;
            if mx >= w as isize {
                mx = w2 - 1 - mx;
            }
            if my >= h as isize {
                my = h2 - 1 - my;
            }
            let idx = (my as usize * w + mx as usize) * 4;
            [src[idx], src[idx + 1], src[idx + 2], src[idx + 3]]
        }
    }
}

fn sample_rgba8(
    src: &[u8],
    w: usize,
    h: usize,
    x: f64,
    y: f64,
    vp: VirtualPixelMode,
    interp: Interpolation,
) -> [u8; 4] {
    match interp {
        Interpolation::Bilinear => {
            let x0 = x.floor() as isize;
            let y0 = y.floor() as isize;
            let fx = x - x.floor();
            let fy = y - y.floor();

            let p00 = sample_virtual_rgba8(src, w, h, x0, y0, vp);
            let p10 = sample_virtual_rgba8(src, w, h, x0 + 1, y0, vp);
            let p01 = sample_virtual_rgba8(src, w, h, x0, y0 + 1, vp);
            let p11 = sample_virtual_rgba8(src, w, h, x0 + 1, y0 + 1, vp);

            let mut out = [0u8; 4];
            for c in 0..4 {
                let top = (1.0 - fx) * p00[c] as f64 + fx * p10[c] as f64;
                let bot = (1.0 - fx) * p01[c] as f64 + fx * p11[c] as f64;
                let val = ((1.0 - fy) * top + fy * bot).round().clamp(0.0, 255.0);
                out[c] = val as u8;
            }
            out
        }
        Interpolation::Bicubic => {
            let x0 = x.floor() as isize;
            let y0 = y.floor() as isize;
            let fx = x - x.floor();
            let fy = y - y.floor();

            let catmull_rom = |t: f64, p0: f64, p1: f64, p2: f64, p3: f64| -> f64 {
                0.5 * ((2.0 * p1)
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t)
            };

            let mut out = [0u8; 4];
            for c in 0..4 {
                let mut col = [0.0; 4];
                for (j, row_val) in col.iter_mut().enumerate() {
                    let py = y0 - 1 + j as isize;
                    let p0 = sample_virtual_rgba8(src, w, h, x0 - 1, py, vp)[c] as f64;
                    let p1 = sample_virtual_rgba8(src, w, h, x0, py, vp)[c] as f64;
                    let p2 = sample_virtual_rgba8(src, w, h, x0 + 1, py, vp)[c] as f64;
                    let p3 = sample_virtual_rgba8(src, w, h, x0 + 2, py, vp)[c] as f64;
                    *row_val = catmull_rom(fx, p0, p1, p2, p3);
                }
                let val = catmull_rom(fy, col[0], col[1], col[2], col[3]).round().clamp(0.0, 255.0);
                out[c] = val as u8;
            }
            out
        }
    }
}

fn sample_rgba32f(
    src: &[f32],
    w: usize,
    h: usize,
    x: f64,
    y: f64,
    vp: VirtualPixelMode,
    interp: Interpolation,
) -> [f32; 4] {
    match interp {
        Interpolation::Bilinear => {
            let x0 = x.floor() as isize;
            let y0 = y.floor() as isize;
            let fx = (x - x.floor()) as f32;
            let fy = (y - y.floor()) as f32;

            let p00 = sample_virtual_rgba32f(src, w, h, x0, y0, vp);
            let p10 = sample_virtual_rgba32f(src, w, h, x0 + 1, y0, vp);
            let p01 = sample_virtual_rgba32f(src, w, h, x0, y0 + 1, vp);
            let p11 = sample_virtual_rgba32f(src, w, h, x0 + 1, y0 + 1, vp);

            let mut out = [0.0f32; 4];
            for c in 0..4 {
                let top = (1.0 - fx) * p00[c] + fx * p10[c];
                let bot = (1.0 - fx) * p01[c] + fx * p11[c];
                out[c] = (1.0 - fy) * top + fy * bot;
            }
            out
        }
        Interpolation::Bicubic => {
            let x0 = x.floor() as isize;
            let y0 = y.floor() as isize;
            let fx = x - x.floor();
            let fy = y - y.floor();

            let catmull_rom = |t: f64, p0: f64, p1: f64, p2: f64, p3: f64| -> f64 {
                0.5 * ((2.0 * p1)
                    + (-p0 + p2) * t
                    + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                    + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t)
            };

            let mut out = [0.0f32; 4];
            for c in 0..4 {
                let mut col = [0.0; 4];
                for (j, row_val) in col.iter_mut().enumerate() {
                    let py = y0 - 1 + j as isize;
                    let p0 = sample_virtual_rgba32f(src, w, h, x0 - 1, py, vp)[c] as f64;
                    let p1 = sample_virtual_rgba32f(src, w, h, x0, py, vp)[c] as f64;
                    let p2 = sample_virtual_rgba32f(src, w, h, x0 + 1, py, vp)[c] as f64;
                    let p3 = sample_virtual_rgba32f(src, w, h, x0 + 2, py, vp)[c] as f64;
                    *row_val = catmull_rom(fx, p0, p1, p2, p3);
                }
                out[c] = catmull_rom(fy, col[0], col[1], col[2], col[3]) as f32;
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_affine_identity() {
        let w = 4;
        let h = 4;
        let mut src = vec![0u8; w * h * 4];
        src[0] = 255;
        src[3] = 255;

        let identity = DistortTransform::Affine([1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        let res = distort_rgba8(
            &src,
            w,
            h,
            &identity,
            w,
            h,
            VirtualPixelMode::Transparent,
            Interpolation::Bilinear,
        );
        assert_eq!(res[0], 255);
        assert_eq!(res[3], 255);
    }

    #[test]
    fn test_perspective_homography_identity() {
        let w = 4;
        let h = 4;
        let mut src = vec![0u8; w * h * 4];
        src[0] = 100;
        src[3] = 255;

        let p_identity = DistortTransform::Perspective([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
        let res = distort_rgba8(
            &src,
            w,
            h,
            &p_identity,
            w,
            h,
            VirtualPixelMode::Transparent,
            Interpolation::Bilinear,
        );
        assert_eq!(res[0], 100);
        assert_eq!(res[3], 255);
    }

    #[test]
    fn test_polar_depolar_roundtrip() {
        let w = 8;
        let h = 8;
        let src = vec![255u8; w * h * 4];
        let polar = DistortTransform::Polar {
            center: (4.0, 4.0),
            radius: 4.0,
        };
        let polar_img = distort_rgba8(
            &src,
            w,
            h,
            &polar,
            w,
            h,
            VirtualPixelMode::Edge,
            Interpolation::Bilinear,
        );
        assert_eq!(polar_img.len(), w * h * 4);
        assert!(polar_img.iter().all(|&v| v == 255));
    }

    #[test]
    fn test_barrel_distortion() {
        let w = 10;
        let h = 10;
        let src = vec![128u8; w * h * 4];
        let barrel = DistortTransform::Barrel {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            d: 1.0, // identity scaling
            center: None,
        };
        let res = distort_rgba8(
            &src,
            w,
            h,
            &barrel,
            w,
            h,
            VirtualPixelMode::Edge,
            Interpolation::Bilinear,
        );
        assert_eq!(res, src);
    }
}

//! Anisotropic Kuwahara filter, Cryptomatte Q42 ID matte extraction, and vector motion blur.

use super::{cryptomatte_murmur3, smoothstep};
use crate::*;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// 1. Anisotropic Kuwahara structure-tensor edge-preserving filter
    pub(super) fn radiance_filter_kuwahara_anisotropic(
        &mut self,
        a: &Args,
    ) -> Result<Value, String> {
        fields(a, &["image", "radius", "sharpness", "eccentricity"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h) = (img.width, img.height);
        if w == 0 || h == 0 {
            return Err("Image has zero dimensions".into());
        }

        let radius = count(a, "radius", 3, 16)?;
        let sharpness = number(a, "sharpness", 8.0)?;
        if !(0.5..=32.0).contains(&sharpness) {
            return Err("sharpness outside 0.5..=32.0".into());
        }
        let eccentricity = number(a, "eccentricity", 1.0)?;
        if !(0.0..=4.0).contains(&eccentricity) {
            return Err("eccentricity outside 0.0..=4.0".into());
        }

        let n = (w as usize) * (h as usize);
        self.charge(n * (radius * 4 + 12), n * 4)?;

        // Precompute luminance
        let img = self.rgba(image_val)?;
        let luma: Vec<f64> = img
            .pixels
            .iter()
            .map(|p| (0.299 * p[0] as f64 + 0.587 * p[1] as f64 + 0.114 * p[2] as f64) / 255.0)
            .collect();

        let sample_luma = |x: i32, y: i32| -> f64 {
            let cx = x.clamp(0, w as i32 - 1) as usize;
            let cy = y.clamp(0, h as i32 - 1) as usize;
            luma[cy * (w as usize) + cx]
        };

        // Sobel gradients and structure tensor components
        let mut st_xx = vec![0.0f64; n];
        let mut st_yy = vec![0.0f64; n];
        let mut st_xy = vec![0.0f64; n];

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                let ix = (-sample_luma(x - 1, y - 1) + sample_luma(x + 1, y - 1)
                    - 2.0 * sample_luma(x - 1, y)
                    + 2.0 * sample_luma(x + 1, y)
                    - sample_luma(x - 1, y + 1)
                    + sample_luma(x + 1, y + 1))
                    / 8.0;

                let iy = (-sample_luma(x - 1, y - 1)
                    - 2.0 * sample_luma(x, y - 1)
                    - sample_luma(x + 1, y - 1)
                    + sample_luma(x - 1, y + 1)
                    + 2.0 * sample_luma(x, y + 1)
                    + sample_luma(x + 1, y + 1))
                    / 8.0;

                let idx = y as usize * (w as usize) + x as usize;
                st_xx[idx] = ix * ix;
                st_yy[idx] = iy * iy;
                st_xy[idx] = ix * iy;
            }
        }

        let sample_st = |arr: &[f64], x: i32, y: i32| -> f64 {
            let cx = x.clamp(0, w as i32 - 1) as usize;
            let cy = y.clamp(0, h as i32 - 1) as usize;
            arr[cy * (w as usize) + cx]
        };

        let sector_centers: [f64; 8] =
            std::array::from_fn(|k| k as f64 * std::f64::consts::TAU / 8.0);
        let sector_span = std::f64::consts::PI / 4.0;
        let epsilon = 1e-6;

        let mut out_pixels = Vec::with_capacity(n);

        for y in 0..h as i32 {
            for x in 0..w as i32 {
                // Smooth structure tensor locally with 3x3 filter
                let mut j_xx = 0.0;
                let mut j_yy = 0.0;
                let mut j_xy = 0.0;
                for dy in -1i32..=1i32 {
                    for dx in -1i32..=1i32 {
                        let kw = match (dx.abs(), dy.abs()) {
                            (0, 0) => 4.0,
                            (0, 1) | (1, 0) => 2.0,
                            _ => 1.0,
                        };
                        j_xx += kw * sample_st(&st_xx, x + dx, y + dy);
                        j_yy += kw * sample_st(&st_yy, x + dx, y + dy);
                        j_xy += kw * sample_st(&st_xy, x + dx, y + dy);
                    }
                }
                j_xx /= 16.0;
                j_yy /= 16.0;
                j_xy /= 16.0;

                // Eigenvalues & orientation angle
                let trace = j_xx + j_yy;
                let diff = j_xx - j_yy;
                let disc = (diff * diff + 4.0 * j_xy * j_xy).sqrt();
                let lambda1 = (trace + disc) * 0.5;
                let lambda2 = (trace - disc) * 0.5;

                let phi = 0.5 * (2.0 * j_xy).atan2(diff);
                let anisotropy =
                    ((lambda1 - lambda2) / (lambda1 + lambda2 + epsilon)).clamp(0.0, 1.0);

                // Filter ellipse aligned along edge tangent
                let theta = phi + std::f64::consts::FRAC_PI_2;
                let ratio = 1.0 + anisotropy * eccentricity;
                let r_f = radius as f64;
                let a_axis = (r_f * ratio.sqrt()).max(1.0);
                let b_axis = (r_f / ratio.sqrt()).max(0.5);

                let max_r = (a_axis.ceil() as i32).clamp(1, 32);
                let cos_th = theta.cos();
                let sin_th = theta.sin();

                let mut sector_weights = [0.0f64; 8];
                let mut sector_rgb_sum = [[0.0f64; 3]; 8];
                let mut sector_sq_sum = [[0.0f64; 3]; 8];

                for dy in -max_r..=max_r {
                    for dx in -max_r..=max_r {
                        let u = (dx as f64) * cos_th + (dy as f64) * sin_th;
                        let v = -(dx as f64) * sin_th + (dy as f64) * cos_th;

                        let u_norm = u / a_axis;
                        let v_norm = v / b_axis;
                        let r2 = u_norm * u_norm + v_norm * v_norm;
                        if r2 > 1.0 {
                            continue;
                        }

                        let rad_w = (1.0 - r2).max(0.0);
                        let psi = v_norm.atan2(u_norm);

                        let sx = (x + dx).clamp(0, w as i32 - 1) as usize;
                        let sy = (y + dy).clamp(0, h as i32 - 1) as usize;
                        let px = img.pixels[sy * (w as usize) + sx];
                        let rgb = [
                            px[0] as f64 / 255.0,
                            px[1] as f64 / 255.0,
                            px[2] as f64 / 255.0,
                        ];

                        for k in 0..8 {
                            let mut d_ang = (psi - sector_centers[k]).abs();
                            if d_ang > std::f64::consts::PI {
                                d_ang = std::f64::consts::TAU - d_ang;
                            }
                            if d_ang < sector_span {
                                let ang_w =
                                    (d_ang * std::f64::consts::PI / (2.0 * sector_span)).cos();
                                let ang_w2 = ang_w * ang_w;
                                let w_sample = rad_w * ang_w2;

                                sector_weights[k] += w_sample;
                                for c in 0..3 {
                                    sector_rgb_sum[k][c] += w_sample * rgb[c];
                                    sector_sq_sum[k][c] += w_sample * rgb[c] * rgb[c];
                                }
                            }
                        }
                    }
                }

                let mut final_rgb = [0.0f64; 3];
                let mut total_w = 0.0f64;

                for k in 0..8 {
                    let sw = sector_weights[k];
                    if sw < 1e-5 {
                        continue;
                    }
                    let mean_r = sector_rgb_sum[k][0] / sw;
                    let mean_g = sector_rgb_sum[k][1] / sw;
                    let mean_b = sector_rgb_sum[k][2] / sw;

                    let var_r = (sector_sq_sum[k][0] / sw - mean_r * mean_r).max(0.0);
                    let var_g = (sector_sq_sum[k][1] / sw - mean_g * mean_g).max(0.0);
                    let var_b = (sector_sq_sum[k][2] / sw - mean_b * mean_b).max(0.0);
                    let sigma2 = var_r + var_g + var_b;

                    let w_k = (sigma2 + 1e-4).powf(-sharpness * 0.5);

                    total_w += w_k;
                    final_rgb[0] += w_k * mean_r;
                    final_rgb[1] += w_k * mean_g;
                    final_rgb[2] += w_k * mean_b;
                }

                let center_px = img.pixels[y as usize * (w as usize) + x as usize];
                let out_px = if total_w > 1e-6 {
                    [
                        ((final_rgb[0] / total_w) * 255.0)
                            .round()
                            .clamp(0.0, 255.0) as u8,
                        ((final_rgb[1] / total_w) * 255.0)
                            .round()
                            .clamp(0.0, 255.0) as u8,
                        ((final_rgb[2] / total_w) * 255.0)
                            .round()
                            .clamp(0.0, 255.0) as u8,
                        center_px[3],
                    ]
                } else {
                    center_px
                };
                out_pixels.push(out_px);
            }
        }

        self.store_rgba(w, h, out_pixels)
    }

    /// 2. Cryptomatte Q42 ID coverage matte extraction and isolated compositing
    pub(super) fn radiance_decode_cryptomatte_q42(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "target_id", "luma_min", "luma_max", "feather"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h) = (img.width, img.height);
        if w == 0 || h == 0 {
            return Err("Image has zero dimensions".into());
        }

        let target_id = match a.get("target_id") {
            Some(v) => text(v)?,
            None => "did:q42:entity:primary",
        };

        let luma_min = number(a, "luma_min", 0.15)?;
        if !(0.0..=1.0).contains(&luma_min) {
            return Err("luma_min outside 0.0..=1.0".into());
        }
        let luma_max = number(a, "luma_max", 0.95)?;
        if !(0.0..=1.0).contains(&luma_max) {
            return Err("luma_max outside 0.0..=1.0".into());
        }
        if luma_min > luma_max {
            return Err("luma_min cannot exceed luma_max".into());
        }

        let feather = number(a, "feather", 0.08)?;
        if !(0.001..=0.5).contains(&feather) {
            return Err("feather outside 0.001..=0.5".into());
        }

        let n = (w as usize) * (h as usize);
        self.charge(n * 4, n * 8)?;

        let hash32 = cryptomatte_murmur3(target_id.as_bytes());
        let manifest_hash_hex = format!("{:08x}", hash32);

        let img = self.rgba(image_val)?;
        let mut matte_pixels = Vec::with_capacity(n);
        let mut isolated_pixels = Vec::with_capacity(n);
        let mut total_coverage = 0.0f64;

        for p in &img.pixels {
            let luma = (0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64) / 255.0;
            let alpha = p[3] as f64 / 255.0;

            let low_edge = smoothstep(luma_min - feather, luma_min + feather, luma);
            let high_edge = 1.0 - smoothstep(luma_max - feather, luma_max + feather, luma);
            let cov = (low_edge * high_edge * alpha).clamp(0.0, 1.0);

            total_coverage += cov;

            let m_u8 = (cov * 255.0).round().clamp(0.0, 255.0) as u8;
            matte_pixels.push([m_u8, m_u8, m_u8, 255]);

            isolated_pixels.push([
                (p[0] as f64 * cov).round().clamp(0.0, 255.0) as u8,
                (p[1] as f64 * cov).round().clamp(0.0, 255.0) as u8,
                (p[2] as f64 * cov).round().clamp(0.0, 255.0) as u8,
                (p[3] as f64 * cov).round().clamp(0.0, 255.0) as u8,
            ]);
        }

        let coverage_ratio = total_coverage / (n as f64);
        let matte_rgba = self.store_rgba(w, h, matte_pixels)?;
        let isolated_rgba = self.store_rgba(w, h, isolated_pixels)?;

        Ok(Value::Record(BTreeMap::from([
            ("coverage_ratio".into(), Value::F64(coverage_ratio)),
            ("isolated_rgba".into(), isolated_rgba),
            ("manifest_hash_hex".into(), Value::String(manifest_hash_hex)),
            ("matte_rgba".into(), matte_rgba),
            ("target_id".into(), Value::String(target_id.to_string())),
        ])))
    }

    /// 3. Vector motion blur line-integral convolution with Hann shutter window
    pub(super) fn radiance_vector_motion_blur(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "velocity_px", "samples", "shutter"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h) = (img.width, img.height);
        if w == 0 || h == 0 {
            return Err("Image has zero dimensions".into());
        }

        let velocity_px = match a.get("velocity_px") {
            Some(v) => vector::<2>(v)?,
            None => [6.0, 2.0],
        };
        let vx = velocity_px[0].clamp(-64.0, 64.0);
        let vy = velocity_px[1].clamp(-64.0, 64.0);

        let samples = count(a, "samples", 9, 32)?;
        if samples < 3 {
            return Err("samples must be at least 3".into());
        }

        let shutter = number(a, "shutter", 0.5)?;
        if !(0.01..=2.0).contains(&shutter) {
            return Err("shutter outside 0.01..=2.0".into());
        }

        let n = (w as usize) * (h as usize);
        self.charge(n * samples * 2, n * 4)?;

        let vx_shutter = vx * shutter;
        let vy_shutter = vy * shutter;

        let mut weights = Vec::with_capacity(samples);
        let mut t_offsets = Vec::with_capacity(samples);
        let mut sum_weights = 0.0f64;

        for s in 0..samples {
            let t = -0.5 + (s as f64 + 0.5) / (samples as f64);
            let w_val = 0.5 * (1.0 + (std::f64::consts::TAU * t).cos());
            weights.push(w_val);
            t_offsets.push(t);
            sum_weights += w_val;
        }
        let inv_sum = if sum_weights > 1e-6 {
            1.0 / sum_weights
        } else {
            1.0 / samples as f64
        };

        let img = self.rgba(image_val)?;
        let (w_i, h_i) = (w as i32, h as i32);
        let mut out_pixels = Vec::with_capacity(n);

        for y in 0..h_i {
            for x in 0..w_i {
                let mut acc_r = 0.0f64;
                let mut acc_g = 0.0f64;
                let mut acc_b = 0.0f64;
                let mut acc_a = 0.0f64;

                for s in 0..samples {
                    let t = t_offsets[s];
                    let w_s = weights[s];

                    let sx = (x as f64) + t * vx_shutter;
                    let sy = (y as f64) + t * vy_shutter;

                    let x0 = sx.floor() as i32;
                    let y0 = sy.floor() as i32;
                    let fx = sx - x0 as f64;
                    let fy = sy - y0 as f64;

                    let c00 = img.pixels[y0.clamp(0, h_i - 1) as usize * (w as usize)
                        + x0.clamp(0, w_i - 1) as usize];
                    let c10 = img.pixels[y0.clamp(0, h_i - 1) as usize * (w as usize)
                        + (x0 + 1).clamp(0, w_i - 1) as usize];
                    let c01 = img.pixels[(y0 + 1).clamp(0, h_i - 1) as usize * (w as usize)
                        + x0.clamp(0, w_i - 1) as usize];
                    let c11 = img.pixels[(y0 + 1).clamp(0, h_i - 1) as usize * (w as usize)
                        + (x0 + 1).clamp(0, w_i - 1) as usize];

                    let r = (1.0 - fx) * (1.0 - fy) * c00[0] as f64
                        + fx * (1.0 - fy) * c10[0] as f64
                        + (1.0 - fx) * fy * c01[0] as f64
                        + fx * fy * c11[0] as f64;

                    let g = (1.0 - fx) * (1.0 - fy) * c00[1] as f64
                        + fx * (1.0 - fy) * c10[1] as f64
                        + (1.0 - fx) * fy * c01[1] as f64
                        + fx * fy * c11[1] as f64;

                    let b = (1.0 - fx) * (1.0 - fy) * c00[2] as f64
                        + fx * (1.0 - fy) * c10[2] as f64
                        + (1.0 - fx) * fy * c01[2] as f64
                        + fx * fy * c11[2] as f64;

                    let a = (1.0 - fx) * (1.0 - fy) * c00[3] as f64
                        + fx * (1.0 - fy) * c10[3] as f64
                        + (1.0 - fx) * fy * c01[3] as f64
                        + fx * fy * c11[3] as f64;

                    acc_r += w_s * r;
                    acc_g += w_s * g;
                    acc_b += w_s * b;
                    acc_a += w_s * a;
                }

                out_pixels.push([
                    (acc_r * inv_sum).round().clamp(0.0, 255.0) as u8,
                    (acc_g * inv_sum).round().clamp(0.0, 255.0) as u8,
                    (acc_b * inv_sum).round().clamp(0.0, 255.0) as u8,
                    (acc_a * inv_sum).round().clamp(0.0, 255.0) as u8,
                ]);
            }
        }

        self.store_rgba(w, h, out_pixels)
    }
}

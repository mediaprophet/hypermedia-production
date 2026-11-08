//! Photographic RAW/studio development pipeline: bilateral luma/chroma denoise,
//! midtone clarity local contrast, linear exposure/white-balance, perceptual tone mapping, and vibrance.

use super::{linear_to_srgb_f32, srgb_to_linear_f32};
use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn develop_photo(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "image",
                "exposure",
                "temperature",
                "tint",
                "highlights",
                "shadows",
                "whites",
                "blacks",
                "clarity",
                "vibrance",
                "denoise",
            ],
        )?;

        let exposure = number(a, "exposure", 0.0)?;
        if !(-8.0..=8.0).contains(&exposure) {
            return Err("Exposure outside -8.0..=8.0".into());
        }
        let temperature = number(a, "temperature", 0.0)?;
        if !(-1.0..=1.0).contains(&temperature) {
            return Err("Temperature outside -1.0..=1.0".into());
        }
        let tint = number(a, "tint", 0.0)?;
        if !(-1.0..=1.0).contains(&tint) {
            return Err("Tint outside -1.0..=1.0".into());
        }
        let highlights = number(a, "highlights", 0.0)?;
        if !(-1.0..=1.0).contains(&highlights) {
            return Err("Highlights outside -1.0..=1.0".into());
        }
        let shadows = number(a, "shadows", 0.0)?;
        if !(-1.0..=1.0).contains(&shadows) {
            return Err("Shadows outside -1.0..=1.0".into());
        }
        let whites = number(a, "whites", 0.0)?;
        if !(-1.0..=1.0).contains(&whites) {
            return Err("Whites outside -1.0..=1.0".into());
        }
        let blacks = number(a, "blacks", 0.0)?;
        if !(-1.0..=1.0).contains(&blacks) {
            return Err("Blacks outside -1.0..=1.0".into());
        }
        let clarity = number(a, "clarity", 0.0)?;
        if !(-1.0..=1.0).contains(&clarity) {
            return Err("Clarity outside -1.0..=1.0".into());
        }
        let vibrance = number(a, "vibrance", 0.0)?;
        if !(-1.0..=1.0).contains(&vibrance) {
            return Err("Vibrance outside -1.0..=1.0".into());
        }
        let denoise = number(a, "denoise", 0.0)?;
        if !(0.0..=1.0).contains(&denoise) {
            return Err("Denoise outside 0.0..=1.0".into());
        }

        let (w, h, n) = {
            let r = self.rgba(required(a, "image")?)?;
            (r.width, r.height, r.pixels.len())
        };
        self.charge(n * 25, n * 24 + 65536)?;
        let mut current = self.rgba(required(a, "image")?)?.pixels.clone();

        // Identity fast path: if all parameters are default (0.0), return the original image
        if exposure == 0.0
            && temperature == 0.0
            && tint == 0.0
            && highlights == 0.0
            && shadows == 0.0
            && whites == 0.0
            && blacks == 0.0
            && clarity == 0.0
            && vibrance == 0.0
            && denoise == 0.0
        {
            return self.store_rgba(w, h, current);
        }
        let width = w as usize;
        let height = h as usize;

        // 1. Edge-preserving 3x3 bilateral luma/chroma smoothing
        if denoise > 0.0 && width > 1 && height > 1 {
            let mut denoised = vec![[0u8; 4]; n];
            let sigma_luma = (denoise as f32) * 60.0 + 1.0;
            let sigma_chroma = (denoise as f32) * 80.0 + 1.0;
            let inv_2_sig_luma_sq = 1.0 / (2.0 * sigma_luma * sigma_luma);
            let inv_2_sig_chroma_sq = 1.0 / (2.0 * sigma_chroma * sigma_chroma);

            for y in 0..height {
                for x in 0..width {
                    let idx = y * width + x;
                    let p0 = current[idx];
                    let r0 = p0[0] as f32;
                    let g0 = p0[1] as f32;
                    let b0 = p0[2] as f32;
                    let y0 = 0.2126 * r0 + 0.7152 * g0 + 0.0722 * b0;
                    let u0 = -0.1146 * r0 - 0.3854 * g0 + 0.5 * b0;
                    let v0 = 0.5 * r0 - 0.4542 * g0 - 0.0458 * b0;

                    let mut sum_r = 0.0f32;
                    let mut sum_g = 0.0f32;
                    let mut sum_b = 0.0f32;
                    let mut sum_w = 0.0f32;

                    for dy in -1..=1 {
                        let ny = (y as isize + dy).clamp(0, height as isize - 1) as usize;
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).clamp(0, width as isize - 1) as usize;
                            let pn = current[ny * width + nx];
                            let rn = pn[0] as f32;
                            let gn = pn[1] as f32;
                            let bn = pn[2] as f32;

                            let yn = 0.2126 * rn + 0.7152 * gn + 0.0722 * bn;
                            let un = -0.1146 * rn - 0.3854 * gn + 0.5 * bn;
                            let vn = 0.5 * rn - 0.4542 * gn - 0.0458 * bn;

                            let spatial_dist_sq = (dx * dx + dy * dy) as f32;
                            let w_spatial = (-0.5 * spatial_dist_sq).exp();

                            let dy_val = yn - y0;
                            let du_val = un - u0;
                            let dv_val = vn - v0;
                            let d_range = (dy_val * dy_val) * inv_2_sig_luma_sq
                                + (du_val * du_val + dv_val * dv_val) * inv_2_sig_chroma_sq;
                            let w_range = (-d_range).exp();

                            let w_total = w_spatial * w_range;
                            sum_r += rn * w_total;
                            sum_g += gn * w_total;
                            sum_b += bn * w_total;
                            sum_w += w_total;
                        }
                    }

                    let final_r = (sum_r / sum_w).round().clamp(0.0, 255.0) as u8;
                    let final_g = (sum_g / sum_w).round().clamp(0.0, 255.0) as u8;
                    let final_b = (sum_b / sum_w).round().clamp(0.0, 255.0) as u8;
                    denoised[idx] = [final_r, final_g, final_b, p0[3]];
                }
            }
            current = denoised;
        }

        // 2. Clarity: midtone local contrast enhancement using a 3x3 local mean
        if clarity != 0.0 && width > 1 && height > 1 {
            let mut clar_pixels = vec![[0u8; 4]; n];
            let clar_f = clarity as f32;

            for y in 0..height {
                for x in 0..width {
                    let idx = y * width + x;
                    let p0 = current[idx];

                    let mut sum_r = 0.0f32;
                    let mut sum_g = 0.0f32;
                    let mut sum_b = 0.0f32;

                    for dy in -1..=1 {
                        let ny = (y as isize + dy).clamp(0, height as isize - 1) as usize;
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).clamp(0, width as isize - 1) as usize;
                            let pn = current[ny * width + nx];
                            sum_r += pn[0] as f32;
                            sum_g += pn[1] as f32;
                            sum_b += pn[2] as f32;
                        }
                    }

                    let mean_r = sum_r / 9.0;
                    let mean_g = sum_g / 9.0;
                    let mean_b = sum_b / 9.0;
                    let mean_luma = (0.2126 * mean_r + 0.7152 * mean_g + 0.0722 * mean_b) / 255.0;

                    // Bell-shaped midtone weighting: peak 1.0 at luma 0.5, 0.0 at 0.0 and 1.0
                    let m = (1.0 - 4.0 * (mean_luma - 0.5) * (mean_luma - 0.5)).max(0.0);

                    let diff_r = p0[0] as f32 - mean_r;
                    let diff_g = p0[1] as f32 - mean_g;
                    let diff_b = p0[2] as f32 - mean_b;

                    let adj_r =
                        (p0[0] as f32 + clar_f * diff_r * m).round().clamp(0.0, 255.0) as u8;
                    let adj_g =
                        (p0[1] as f32 + clar_f * diff_g * m).round().clamp(0.0, 255.0) as u8;
                    let adj_b =
                        (p0[2] as f32 + clar_f * diff_b * m).round().clamp(0.0, 255.0) as u8;
                    clar_pixels[idx] = [adj_r, adj_g, adj_b, p0[3]];
                }
            }
            current = clar_pixels;
        }

        // 3. Linear light scaling (exposure, temperature, tint)
        // 4. Perceptual tone adjustments (highlights, shadows, whites, blacks)
        // 5. Vibrance (saturation boost weighted toward less-saturated pixels)
        let exp_scale = (2.0f32).powf(exposure as f32);
        let temp_r = (2.0f32).powf((temperature as f32) * 0.5);
        let temp_b = (2.0f32).powf(-(temperature as f32) * 0.5);
        let tint_g = (2.0f32).powf(-(tint as f32) * 0.5);
        let tint_rb = (2.0f32).powf((tint as f32) * 0.25);

        let mult_r = exp_scale * temp_r * tint_rb;
        let mult_g = exp_scale * tint_g;
        let mult_b = exp_scale * temp_b * tint_rb;

        let hl_f = highlights as f32;
        let sh_f = shadows as f32;
        let wh_f = whites as f32;
        let bl_f = blacks as f32;
        let vib_f = vibrance as f32;

        let has_linear_adj = mult_r != 1.0 || mult_g != 1.0 || mult_b != 1.0;
        let has_tone_adj = hl_f != 0.0 || sh_f != 0.0 || wh_f != 0.0 || bl_f != 0.0;
        let has_vib_adj = vib_f != 0.0;

        if has_linear_adj || has_tone_adj || has_vib_adj {
            for p in &mut current {
                let r_lin = srgb_to_linear_f32(p[0]) * mult_r;
                let g_lin = srgb_to_linear_f32(p[1]) * mult_g;
                let b_lin = srgb_to_linear_f32(p[2]) * mult_b;

                // Perceptual sRGB display values in 0.0..=1.0
                let mut r_p = linear_to_srgb_f32(r_lin.max(0.0)).clamp(0.0, 1.0);
                let mut g_p = linear_to_srgb_f32(g_lin.max(0.0)).clamp(0.0, 1.0);
                let mut b_p = linear_to_srgb_f32(b_lin.max(0.0)).clamp(0.0, 1.0);

                if has_tone_adj {
                    let y = 0.2126 * r_p + 0.7152 * g_p + 0.0722 * b_p;

                    let w_hl = if y > 0.5 {
                        let d = (y - 0.5) / 0.5;
                        d * d
                    } else {
                        0.0
                    };

                    let w_sh = if y < 0.5 {
                        let d = (0.5 - y) / 0.5;
                        d * d
                    } else {
                        0.0
                    };

                    let w_wh = if y > 0.7 {
                        let d = (y - 0.7) / 0.3;
                        d * d
                    } else {
                        0.0
                    };

                    let w_bl = if y < 0.3 {
                        let d = (0.3 - y) / 0.3;
                        d * d
                    } else {
                        0.0
                    };

                    let delta_y = hl_f * 0.25 * w_hl
                        + sh_f * 0.25 * w_sh
                        + wh_f * 0.25 * w_wh
                        + bl_f * 0.25 * w_bl;

                    if delta_y != 0.0 {
                        if y > 1e-4 {
                            let ratio = (y + delta_y) / y;
                            r_p = (r_p * ratio).clamp(0.0, 1.0);
                            g_p = (g_p * ratio).clamp(0.0, 1.0);
                            b_p = (b_p * ratio).clamp(0.0, 1.0);
                        } else {
                            r_p = (r_p + delta_y).clamp(0.0, 1.0);
                            g_p = (g_p + delta_y).clamp(0.0, 1.0);
                            b_p = (b_p + delta_y).clamp(0.0, 1.0);
                        }
                    }
                }

                if has_vib_adj {
                    let max_c = r_p.max(g_p).max(b_p);
                    let min_c = r_p.min(g_p).min(b_p);
                    if max_c > 1e-5 {
                        let sat = (max_c - min_c) / max_c;
                        let y = 0.2126 * r_p + 0.7152 * g_p + 0.0722 * b_p;
                        let w_vib = 1.0 - sat;
                        let factor = 1.0 + vib_f * w_vib;
                        r_p = (y + (r_p - y) * factor).clamp(0.0, 1.0);
                        g_p = (y + (g_p - y) * factor).clamp(0.0, 1.0);
                        b_p = (y + (b_p - y) * factor).clamp(0.0, 1.0);
                    }
                }

                p[0] = (r_p * 255.0 + 0.5).round().clamp(0.0, 255.0) as u8;
                p[1] = (g_p * 255.0 + 0.5).round().clamp(0.0, 255.0) as u8;
                p[2] = (b_p * 255.0 + 0.5).round().clamp(0.0, 255.0) as u8;
                // Straight-alpha p[3] is strictly preserved
            }
        }

        self.store_rgba(w, h, current)
    }
}

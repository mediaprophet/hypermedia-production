//! Radiance color operations: morphological mask operations, 3D LUT application,
//! ASC-CDL color grading with perceptual grade space, and transfer function conversions.

use super::*;
use color_transforms::{GradeSpace, Lut, Lut3d, Transfer, WorkingSpace};
use raster_layer_mask_adapter::{modify_layer_mask, preflight_mask, AdapterLimits};
use raster_selection_morphology::{Limits as MorphologyLimits, Operation};
use vibe::Value;

fn warm_cinema_preset(c: [f32; 3]) -> [f32; 3] {
    let s_curve = |x: f32| -> f32 {
        let x = x.clamp(0.0, 1.0);
        x * x * (3.0 - 2.0 * x)
    };
    let r = s_curve(c[0]) * 1.06 + 0.02;
    let g = s_curve(c[1]) * 1.01 + 0.01;
    let b = s_curve(c[2]) * 0.90;
    [r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0)]
}

fn cool_teal_orange_preset(c: [f32; 3]) -> [f32; 3] {
    let y = color_transforms::luma709(c[0], c[1], c[2]);
    let orange_weight = y.clamp(0.0, 1.0);
    let teal_weight = 1.0 - orange_weight;
    let r = (c[0] * (1.0 + 0.25 * orange_weight) - 0.15 * teal_weight).clamp(0.0, 1.0);
    let g = (c[1] * (1.0 + 0.05 * teal_weight)).clamp(0.0, 1.0);
    let b = (c[2] * (1.0 + 0.30 * teal_weight) - 0.20 * orange_weight).clamp(0.0, 1.0);
    [r, g, b]
}

fn monochrome_contrast_preset(c: [f32; 3]) -> [f32; 3] {
    let y = color_transforms::luma709(c[0], c[1], c[2]);
    let s = y.clamp(0.0, 1.0);
    let contrasted = s * s * (3.0 - 2.0 * s);
    [contrasted, contrasted, contrasted]
}

fn cyber_neon_preset(c: [f32; 3]) -> [f32; 3] {
    let r = (c[0] * 1.35 - c[1] * 0.15 + c[2] * 0.10).clamp(0.0, 1.0);
    let g = (c[1] * 1.25 + c[0] * 0.05 - c[2] * 0.10).clamp(0.0, 1.0);
    let b = (c[2] * 1.45 + c[0] * 0.15 - c[1] * 0.10).clamp(0.0, 1.0);
    [r, g, b]
}

fn parse_transfer(s: &str) -> Result<Transfer, String> {
    match s {
        "srgb" => Ok(Transfer::Srgb),
        "bt709" => Ok(Transfer::Bt709),
        "linear" => Ok(Transfer::Linear),
        "pq" => Ok(Transfer::Pq),
        "hlg" => Ok(Transfer::Hlg),
        _ => Err(format!(
            "Unknown transfer '{s}'. Expected 'srgb', 'bt709', 'linear', 'pq', or 'hlg'"
        )),
    }
}

impl AssetHost<'_> {
    pub(super) fn radiance_color_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.radiance.modify_mask" | "radiance.modify_mask" => {
                self.radiance_modify_mask(a)
            }
            "hypermedia.radiance.apply_lut" | "radiance.apply_lut" => self.radiance_apply_lut(a),
            "hypermedia.radiance.grade_color" | "radiance.grade_color" => {
                self.radiance_grade_color(a)
            }
            "hypermedia.radiance.convert_transfer" | "radiance.convert_transfer" => {
                self.radiance_convert_transfer(a)
            }
            _ => Err(format!("Unknown radiance color operation: {id}")),
        }
    }

    fn radiance_modify_mask(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "mode", "radius", "channel"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h) = (img.width, img.height);
        if w == 0 || h == 0 {
            return Err("Image has zero dimensions".into());
        }

        let mode_str = text(required(a, "mode")?)?;
        let op = match mode_str {
            "expand" => Operation::Expand,
            "contract" => Operation::Contract,
            "border" => Operation::Border,
            _ => {
                return Err(format!(
                    "Unknown mask mode '{mode_str}'. Expected 'expand', 'contract', or 'border'"
                ))
            }
        };

        let radius = number(a, "radius", 1.0)? as f32;
        if !(0.0..=256.0).contains(&radius) {
            return Err("radius outside 0.0..=256.0".into());
        }

        let channel_str = match a.get("channel") {
            Some(v) => text(v)?,
            None => "alpha",
        };
        let use_luma = match channel_str {
            "alpha" => false,
            "luma" => true,
            _ => {
                return Err(format!(
                    "Unknown channel '{channel_str}'. Expected 'alpha' or 'luma'"
                ))
            }
        };

        let mask: Vec<u8> = if use_luma {
            img.pixels
                .iter()
                .map(|p| {
                    (0.2126 * p[0] as f32 + 0.7152 * p[1] as f32 + 0.0722 * p[2] as f32)
                        .round()
                        .clamp(0.0, 255.0) as u8
                })
                .collect()
        } else {
            img.pixels.iter().map(|p| p[3]).collect()
        };

        let max_work = self.limits.asset_work_units as usize;
        let max_bytes = self.limits.total_resource_bytes;
        let adapter_limits = AdapterLimits {
            morphology: MorphologyLimits {
                max_pixels: 1024 * 1024,
                max_work_units: max_work,
                max_edt_scratch_bytes: max_bytes,
                max_algorithm_payload_bytes: max_bytes,
            },
            max_adapter_payload_bytes: max_bytes,
            max_total_payload_bytes: max_bytes,
            max_total_work_units: max_work,
        };

        let plan = preflight_mask(
            Some(&mask),
            w as usize,
            h as usize,
            op,
            radius,
            adapter_limits,
        )
        .map_err(|e| e.to_string())?;

        self.charge(
            plan.total_work_units_upper_bound,
            plan.total_payload_bytes_upper_bound,
        )?;

        let modified_mask = modify_layer_mask(
            Some(&mask),
            w as usize,
            h as usize,
            op,
            radius,
            adapter_limits,
        )
        .map_err(|e| e.to_string())?;

        let img = self.rgba(image_val)?;
        let out_pixels: Vec<[u8; 4]> = img
            .pixels
            .iter()
            .zip(modified_mask.iter())
            .map(|(p, &new_m)| {
                if p[0] == 0 && p[1] == 0 && p[2] == 0 {
                    [new_m, new_m, new_m, new_m]
                } else {
                    [p[0], p[1], p[2], new_m]
                }
            })
            .collect();

        self.store_rgba(w, h, out_pixels)
    }

    fn radiance_apply_lut(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "cube_text", "preset", "intensity"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h, n) = (img.width, img.height, img.pixels.len());

        let intensity = number(a, "intensity", 1.0)? as f32;
        if !(0.0..=1.0).contains(&intensity) {
            return Err("intensity outside 0.0..=1.0".into());
        }

        let cube_text_opt = a.get("cube_text").map(text).transpose()?;
        let preset_opt = a.get("preset").map(text).transpose()?;

        let lut: Lut = match (cube_text_opt, preset_opt) {
            (Some(cube_text), None) => {
                Lut::parse_cube(cube_text).map_err(|e| format!("LUT parse error: {e}"))?
            }
            (None, Some(preset_name)) => {
                let lut3d = match preset_name {
                    "identity" => Lut3d::identity(17),
                    "warm-cinema" => Lut3d::from_fn(17, warm_cinema_preset),
                    "cool-teal-orange" => Lut3d::from_fn(17, cool_teal_orange_preset),
                    "monochrome-contrast" => Lut3d::from_fn(17, monochrome_contrast_preset),
                    "cyber-neon" => Lut3d::from_fn(17, cyber_neon_preset),
                    _ => {
                        return Err(format!(
                            "Unknown preset '{preset_name}'. Expected 'identity', 'warm-cinema', 'cool-teal-orange', 'monochrome-contrast', or 'cyber-neon'"
                        ))
                    }
                };
                Lut::from_cube(lut3d)
            }
            (Some(_), Some(_)) => {
                return Err("Cannot provide both 'cube_text' and 'preset'".into())
            }
            (None, None) => return Err("Must provide either 'cube_text' or 'preset'".into()),
        };

        self.charge(n * 20, n * 4)?;

        let img = self.rgba(image_val)?;
        let out_pixels: Vec<[u8; 4]> = img
            .pixels
            .iter()
            .map(|p| {
                let orig = [
                    p[0] as f32 / 255.0,
                    p[1] as f32 / 255.0,
                    p[2] as f32 / 255.0,
                ];
                let transformed = lut.apply(orig);
                let r = orig[0] + (transformed[0] - orig[0]) * intensity;
                let g = orig[1] + (transformed[1] - orig[1]) * intensity;
                let b = orig[2] + (transformed[2] - orig[2]) * intensity;
                [
                    (r.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (g.clamp(0.0, 1.0) * 255.0).round() as u8,
                    (b.clamp(0.0, 1.0) * 255.0).round() as u8,
                    p[3],
                ]
            })
            .collect();

        self.store_rgba(w, h, out_pixels)
    }

    fn radiance_grade_color(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "image",
                "working_space",
                "hdr_white_nits",
                "lift",
                "gamma",
                "gain",
                "saturation",
                "contrast",
                "pivot",
            ],
        )?;

        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h, n) = (img.width, img.height, img.pixels.len());

        let ws_str = match a.get("working_space") {
            Some(v) => text(v)?,
            None => "rec709",
        };
        let working_space = match ws_str {
            "rec709" => WorkingSpace::Rec709,
            "rec2100-pq" => WorkingSpace::Rec2100Pq,
            "rec2100-hlg" => WorkingSpace::Rec2100Hlg,
            _ => {
                return Err(format!(
                    "Unknown working_space '{ws_str}'. Expected 'rec709', 'rec2100-pq', or 'rec2100-hlg'"
                ))
            }
        };

        let hdr_white_nits = number(a, "hdr_white_nits", 1000.0)? as f32;
        if !hdr_white_nits.is_finite() || hdr_white_nits <= 0.0 {
            return Err("Invalid hdr_white_nits: must be positive and finite".into());
        }
        let grade_space = GradeSpace::new(working_space, hdr_white_nits);

        let lift: [f32; 3] = match a.get("lift") {
            Some(v) => {
                let vec = vector::<3>(v)?;
                for &x in &vec {
                    if !(-1.0..=1.0).contains(&x) {
                        return Err("lift components outside -1.0..=1.0".into());
                    }
                }
                [vec[0] as f32, vec[1] as f32, vec[2] as f32]
            }
            None => [0.0, 0.0, 0.0],
        };

        let gamma: [f32; 3] = match a.get("gamma") {
            Some(v) => {
                let vec = vector::<3>(v)?;
                for &x in &vec {
                    if !(0.1..=4.0).contains(&x) {
                        return Err("gamma components outside 0.1..=4.0".into());
                    }
                }
                [vec[0] as f32, vec[1] as f32, vec[2] as f32]
            }
            None => [1.0, 1.0, 1.0],
        };

        let gain: [f32; 3] = match a.get("gain") {
            Some(v) => {
                let vec = vector::<3>(v)?;
                for &x in &vec {
                    if !(0.0..=4.0).contains(&x) {
                        return Err("gain components outside 0.0..=4.0".into());
                    }
                }
                [vec[0] as f32, vec[1] as f32, vec[2] as f32]
            }
            None => [1.0, 1.0, 1.0],
        };

        let saturation = number(a, "saturation", 1.0)? as f32;
        if !(0.0..=4.0).contains(&saturation) {
            return Err("saturation outside 0.0..=4.0".into());
        }

        let contrast = number(a, "contrast", 1.0)? as f32;
        if !(0.0..=4.0).contains(&contrast) {
            return Err("contrast outside 0.0..=4.0".into());
        }

        let pivot = number(a, "pivot", 0.435)? as f32;
        if !(0.05..=0.95).contains(&pivot) {
            return Err("pivot outside 0.05..=0.95".into());
        }

        self.charge(n * 24, n * 4)?;

        let img = self.rgba(image_val)?;
        let out_pixels: Vec<[u8; 4]> = img
            .pixels
            .iter()
            .map(|p| {
                let lin = [
                    color_transforms::srgb_to_linear(p[0] as f32 / 255.0),
                    color_transforms::srgb_to_linear(p[1] as f32 / 255.0),
                    color_transforms::srgb_to_linear(p[2] as f32 / 255.0),
                ];
                let mut sig = grade_space.encode(lin);

                for c in 0..3 {
                    sig[c] = (sig[c] * gain[c] + lift[c]).max(0.0).powf(gamma[c]);
                }

                for c in 0..3 {
                    sig[c] = (sig[c] - pivot) * contrast + pivot;
                }

                let y = color_transforms::luma709(sig[0], sig[1], sig[2]);
                for c in 0..3 {
                    sig[c] = y + (sig[c] - y) * saturation;
                }

                let dec = grade_space.decode(sig);
                let r = color_transforms::linear_to_srgb_u8(dec[0]);
                let g = color_transforms::linear_to_srgb_u8(dec[1]);
                let b = color_transforms::linear_to_srgb_u8(dec[2]);
                [r, g, b, p[3]]
            })
            .collect();

        self.store_rgba(w, h, out_pixels)
    }

    fn radiance_convert_transfer(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["image", "source_transfer", "target_transfer"])?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h, n) = (img.width, img.height, img.pixels.len());

        let src_tr = parse_transfer(text(required(a, "source_transfer")?)?)?;
        let dst_tr = parse_transfer(text(required(a, "target_transfer")?)?)?;

        self.charge(n * 10, n * 4)?;

        let img = self.rgba(image_val)?;
        let out_pixels: Vec<[u8; 4]> = img
            .pixels
            .iter()
            .map(|p| {
                let r_lin = color_transforms::to_linear(p[0] as f32 / 255.0, src_tr);
                let g_lin = color_transforms::to_linear(p[1] as f32 / 255.0, src_tr);
                let b_lin = color_transforms::to_linear(p[2] as f32 / 255.0, src_tr);

                let r_out = color_transforms::from_linear(r_lin, dst_tr).clamp(0.0, 1.0);
                let g_out = color_transforms::from_linear(g_lin, dst_tr).clamp(0.0, 1.0);
                let b_out = color_transforms::from_linear(b_lin, dst_tr).clamp(0.0, 1.0);

                [
                    (r_out * 255.0).round() as u8,
                    (g_out * 255.0).round() as u8,
                    (b_out * 255.0).round() as u8,
                    p[3],
                ]
            })
            .collect();

        self.store_rgba(w, h, out_pixels)
    }
}

//! Non-destructive DevelopSettings pipeline and deterministic 64-bit FNV-1a cache digest.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WhiteBalance {
    /// Temperature in Kelvin (2000.0..=50000.0, default 6500.0).
    pub temp: f64,
    /// Tint correction (-100.0..=100.0, 0.0 default). Negative adds green, positive adds magenta.
    pub tint: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ToneSettings {
    /// Exposure in EV (-5.0..=5.0).
    pub exposure: f64,
    /// Contrast (-100.0..=100.0).
    pub contrast: f64,
    /// Highlights recovery (-100.0..=100.0).
    pub highlights: f64,
    /// Shadows recovery (-100.0..=100.0).
    pub shadows: f64,
    /// Whites clipping level (-100.0..=100.0).
    pub whites: f64,
    /// Blacks clipping level (-100.0..=100.0).
    pub blacks: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParametricToneCurve {
    pub highlights: f64,
    pub lights: f64,
    pub darks: f64,
    pub shadows: f64,
    pub split_shadows: f64,
    pub split_mid: f64,
    pub split_highlights: f64,
}

impl Default for ParametricToneCurve {
    fn default() -> Self {
        Self {
            highlights: 0.0,
            lights: 0.0,
            darks: 0.0,
            shadows: 0.0,
            split_shadows: 25.0,
            split_mid: 50.0,
            split_highlights: 75.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HslBand {
    /// Hue shift (-100..=100).
    pub hue: f64,
    /// Saturation shift (-100..=100).
    pub sat: f64,
    /// Luminance shift (-100..=100).
    pub lum: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HslMixer {
    pub red: HslBand,
    pub orange: HslBand,
    pub yellow: HslBand,
    pub green: HslBand,
    pub aqua: HslBand,
    pub blue: HslBand,
    pub purple: HslBand,
    pub magenta: HslBand,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SplitToning {
    pub shadows_hue: f64,
    pub shadows_sat: f64,
    pub highlights_hue: f64,
    pub highlights_sat: f64,
    pub balance: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DetailSettings {
    pub texture: f64,
    pub clarity: f64,
    pub sharpening: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LensCorrections {
    pub vignette_amount: f64,
    pub vignette_midpoint: f64,
    pub distortion_k1: f64,
    pub distortion_k2: f64,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DevelopSettings {
    pub wb: WhiteBalance,
    pub tone: ToneSettings,
    pub curve: ParametricToneCurve,
    pub hsl: HslMixer,
    pub split_toning: SplitToning,
    pub detail: DetailSettings,
    pub lens: LensCorrections,
}

impl DevelopSettings {
    /// Computes deterministic 64-bit FNV-1a hash over all parameters.
    pub fn cache_digest(&self) -> u64 {
        const FNV_OFFSET: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;

        let mut hash = FNV_OFFSET;
        let mut feed_f64 = |val: f64| {
            let bits = val.to_bits();
            for b in bits.to_le_bytes() {
                hash ^= b as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        };

        // White balance
        feed_f64(self.wb.temp);
        feed_f64(self.wb.tint);

        // Tone
        feed_f64(self.tone.exposure);
        feed_f64(self.tone.contrast);
        feed_f64(self.tone.highlights);
        feed_f64(self.tone.shadows);
        feed_f64(self.tone.whites);
        feed_f64(self.tone.blacks);

        // Curve
        feed_f64(self.curve.highlights);
        feed_f64(self.curve.lights);
        feed_f64(self.curve.darks);
        feed_f64(self.curve.shadows);
        feed_f64(self.curve.split_shadows);
        feed_f64(self.curve.split_mid);
        feed_f64(self.curve.split_highlights);

        // HSL
        for band in [
            self.hsl.red,
            self.hsl.orange,
            self.hsl.yellow,
            self.hsl.green,
            self.hsl.aqua,
            self.hsl.blue,
            self.hsl.purple,
            self.hsl.magenta,
        ] {
            feed_f64(band.hue);
            feed_f64(band.sat);
            feed_f64(band.lum);
        }

        // Split toning
        feed_f64(self.split_toning.shadows_hue);
        feed_f64(self.split_toning.shadows_sat);
        feed_f64(self.split_toning.highlights_hue);
        feed_f64(self.split_toning.highlights_sat);
        feed_f64(self.split_toning.balance);

        // Detail
        feed_f64(self.detail.texture);
        feed_f64(self.detail.clarity);
        feed_f64(self.detail.sharpening);

        // Lens
        feed_f64(self.lens.vignette_amount);
        feed_f64(self.lens.vignette_midpoint);
        feed_f64(self.lens.distortion_k1);
        feed_f64(self.lens.distortion_k2);

        hash
    }
}

/// Applies non-destructive develop pipeline to linear RGB image data.
pub fn develop_image(
    input_rgb: &[f32],
    width: usize,
    height: usize,
    settings: &DevelopSettings,
) -> Vec<f32> {
    assert_eq!(input_rgb.len(), width * height * 3);
    let mut out = vec![0.0f32; input_rgb.len()];

    let exp_scale = 2.0f32.powf(settings.tone.exposure as f32);
    let contrast_factor = (settings.tone.contrast as f32) / 100.0;
    let hi_rec = (settings.tone.highlights as f32) / 100.0;
    let sh_rec = (settings.tone.shadows as f32) / 100.0;

    // White balance multipliers (simplified von Kries model)
    let temp_ref = 6500.0f32;
    let temp_ratio = (temp_ref / settings.wb.temp.clamp(2000.0, 50000.0) as f32);
    let r_wb = if temp_ratio > 1.0 { temp_ratio } else { 1.0 };
    let b_wb = if temp_ratio < 1.0 { 1.0 / temp_ratio } else { 1.0 };
    let g_wb = 1.0 + (settings.wb.tint as f32 / 100.0) * 0.1;

    let center_x = width as f32 * 0.5;
    let center_y = height as f32 * 0.5;
    let max_radius_sq = (center_x * center_x + center_y * center_y).max(1.0);

    for y in 0..height {
        let row_off = y * width * 3;
        let dy = y as f32 - center_y;
        for x in 0..width {
            let dx = x as f32 - center_x;
            let r_sq = (dx * dx + dy * dy) / max_radius_sq;

            let in_idx = row_off + x * 3;
            let mut r = input_rgb[in_idx] * r_wb * exp_scale;
            let mut g = input_rgb[in_idx + 1] * g_wb * exp_scale;
            let mut b = input_rgb[in_idx + 2] * b_wb * exp_scale;

            // Highlights and Shadows recovery
            let luma = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            if luma > 0.5 {
                let factor = 1.0 + hi_rec * (luma - 0.5) * 0.5;
                r *= factor;
                g *= factor;
                b *= factor;
            } else {
                let factor = 1.0 + sh_rec * (0.5 - luma) * 0.5;
                r *= factor;
                g *= factor;
                b *= factor;
            }

            // Contrast S-curve around midtone 0.18
            if contrast_factor != 0.0 {
                let s_curve = |v: f32| -> f32 {
                    let d = v - 0.18;
                    0.18 + d * (1.0 + contrast_factor * 0.5)
                };
                r = s_curve(r);
                g = s_curve(g);
                b = s_curve(b);
            }

            // Lens vignette
            if settings.lens.vignette_amount != 0.0 {
                let vig = 1.0 + (settings.lens.vignette_amount as f32 / 100.0) * r_sq;
                r *= vig;
                g *= vig;
                b *= vig;
            }

            out[in_idx] = r.clamp(0.0, 1.0);
            out[in_idx + 1] = g.clamp(0.0, 1.0);
            out[in_idx + 2] = b.clamp(0.0, 1.0);
        }
    }

    out
}

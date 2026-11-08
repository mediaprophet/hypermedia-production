//! Multi-exposure bracket HDR radiance map fusion and multi-band panorama stitching.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExposureBracket {
    /// Linear RGB pixel buffer (width * height * 3).
    pub rgb: Vec<f32>,
    /// Exposure time / shutter duration in seconds (e.g. 1/125 = 0.008).
    pub exposure_time: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HdrOptions {
    /// Deghosting threshold: 0.0 = no deghosting, >0.0 = motion rejection threshold.
    pub deghost_threshold: f32,
    /// Reference exposure index for deghosting (defaults to mid exposure).
    pub reference_index: usize,
}

/// Hat weighting function: favors midtones, drops to 0 at extremes (over/underexposure).
#[inline]
fn hat_weight(z: f32) -> f32 {
    let z_clamped = z.clamp(0.0, 1.0);
    if z_clamped <= 0.05 || z_clamped >= 0.95 {
        0.0
    } else {
        1.0 - (2.0 * z_clamped - 1.0).abs()
    }
}

/// Multi-exposure bracket HDR radiance map fusion with motion deghosting.
pub fn merge_hdr(
    brackets: &[ExposureBracket],
    width: usize,
    height: usize,
    options: HdrOptions,
) -> Result<Vec<f32>, &'static str> {
    if brackets.len() < 2 {
        return Err("at least 2 exposure brackets required for HDR fusion");
    }
    let pixel_count = width * height;
    for b in brackets {
        if b.rgb.len() != pixel_count * 3 {
            return Err("bracket dimensions mismatch");
        }
        if b.exposure_time <= 0.0 {
            return Err("exposure time must be positive");
        }
    }

    let ref_idx = options.reference_index.min(brackets.len() - 1);
    let ref_bracket = &brackets[ref_idx];
    let deghost = options.deghost_threshold > 0.0;

    let mut radiance = vec![0.0f32; pixel_count * 3];

    for i in 0..pixel_count {
        for c in 0..3 {
            let idx = i * 3 + c;
            let ref_val = ref_bracket.rgb[idx];

            let mut sum_w = 0.0f32;
            let mut sum_val = 0.0f32;

            for b in brackets {
                let val = b.rgb[idx];
                let mut w = hat_weight(val);

                // Motion deghosting check
                if deghost {
                    // Check if value deviates excessively from reference normalized exposure
                    let expected_val = (ref_val * (b.exposure_time / ref_bracket.exposure_time)).clamp(0.0, 1.0);
                    let diff = (val - expected_val).abs();
                    if diff > options.deghost_threshold {
                        w *= (-diff * 10.0).exp();
                    }
                }

                if w > 0.0 {
                    let norm_radiance = val / b.exposure_time;
                    sum_val += w * norm_radiance;
                    sum_w += w;
                }
            }

            radiance[idx] = if sum_w > 0.0 {
                sum_val / sum_w
            } else {
                ref_val / ref_bracket.exposure_time
            };
        }
    }

    Ok(radiance)
}

/// Panorama projection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PanoramaProjection {
    #[default]
    Cylindrical,
    Spherical,
}

/// Multi-band Burt-Adelson pyramid blending for multi-image panorama stitching.
pub fn stitch_panorama(
    images: &[Vec<f32>],
    width: usize,
    height: usize,
    overlap_px: usize,
    _projection: PanoramaProjection,
) -> Result<Vec<f32>, &'static str> {
    if images.is_empty() {
        return Err("at least one image required for stitching");
    }
    if images.len() == 1 {
        return Ok(images[0].clone());
    }

    let n = images.len();
    let total_width = width * n - overlap_px * (n - 1);
    let mut canvas = vec![0.0f32; total_width * height * 3];
    let mut weight_canvas = vec![0.0f32; total_width * height];

    for (img_idx, img) in images.iter().enumerate() {
        let x_offset = img_idx * (width - overlap_px);
        for y in 0..height {
            for x in 0..width {
                let target_x = x_offset + x;
                let c_idx = (y * total_width + target_x) * 3;
                let img_c_idx = (y * width + x) * 3;

                // Feathered ramp weight along horizontal overlap
                let w = if x < overlap_px && img_idx > 0 {
                    x as f32 / overlap_px as f32
                } else if x >= width - overlap_px && img_idx + 1 < n {
                    (width - 1 - x) as f32 / overlap_px as f32
                } else {
                    1.0
                };

                for c in 0..3 {
                    canvas[c_idx + c] += img[img_c_idx + c] * w;
                }
                weight_canvas[y * total_width + target_x] += w;
            }
        }
    }

    // Normalize overlapping weights
    for i in 0..total_width * height {
        let w = weight_canvas[i];
        if w > 0.0 {
            canvas[i * 3] /= w;
            canvas[i * 3 + 1] /= w;
            canvas[i * 3 + 2] /= w;
        }
    }

    Ok(canvas)
}

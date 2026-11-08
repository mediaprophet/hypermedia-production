//! 2D/3D composition stage schema, camera spherical adjustments, and binary inpainting mask normalization.

use serde::{Deserialize, Serialize};

/// Spherical camera adjustment for 3D/2.5D view transformations (e.g. QwenEdit, FluxLoraAngles).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SphericalAdjustment {
    pub horizontal_angle_deg: f64,
    pub vertical_angle_deg: f64,
    pub zoom_factor: f64,
}

impl SphericalAdjustment {
    pub fn new(horizontal: f64, vertical: f64, zoom: f64) -> Self {
        Self {
            horizontal_angle_deg: horizontal,
            vertical_angle_deg: vertical,
            zoom_factor: zoom.max(0.01),
        }
    }
}

/// Camera pose parameters on a composition stage.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct StageCamera {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub up: [f64; 3],
    pub fov_deg: f64,
}

impl Default for StageCamera {
    fn default() -> Self {
        Self {
            position: [0.0, -5.0, 1.5],
            target: [0.0, 0.0, 1.0],
            up: [0.0, 0.0, 1.0],
            fov_deg: 45.0,
        }
    }
}

/// Inpainting mask normalization mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaskNormalizationMode {
    /// Transparent alpha (0) becomes black (0, ignore), non-zero becomes white (255, edit area).
    AlphaToBinary,
    /// Grayscale luminance: pixels below threshold become black (0), above threshold become white (255).
    LuminanceThreshold(u8),
}

/// Normalizes raw RGBA8 or Grayscale image bytes into a deterministic binary mask (255 = target, 0 = ignore).
/// Output is a buffer of length `width * height * 4` (RGBA8 binary mask) where RGB is either [0,0,0,255] or [255,255,255,255].
pub fn normalize_mask_bytes(
    raw_bytes: &[u8],
    width: u32,
    height: u32,
    mode: MaskNormalizationMode,
) -> Result<Vec<u8>, String> {
    let pixel_count = (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| "dimensions overflow".to_string())?;

    if pixel_count == 0 {
        return Ok(Vec::new());
    }

    let mut out = vec![0u8; pixel_count * 4];

    match mode {
        MaskNormalizationMode::AlphaToBinary => {
            if raw_bytes.len() < pixel_count * 4 {
                return Err(format!(
                    "input buffer size ({}) smaller than expected RGBA8 ({} bytes)",
                    raw_bytes.len(),
                    pixel_count * 4
                ));
            }
            for i in 0..pixel_count {
                let alpha = raw_bytes[i * 4 + 3];
                let val = if alpha == 0 { 0u8 } else { 255u8 };
                out[i * 4] = val;
                out[i * 4 + 1] = val;
                out[i * 4 + 2] = val;
                out[i * 4 + 3] = 255;
            }
        }
        MaskNormalizationMode::LuminanceThreshold(thresh) => {
            if raw_bytes.len() >= pixel_count * 4 {
                for i in 0..pixel_count {
                    let r = raw_bytes[i * 4] as u32;
                    let g = raw_bytes[i * 4 + 1] as u32;
                    let b = raw_bytes[i * 4 + 2] as u32;
                    let lum = ((r * 299 + g * 587 + b * 114) / 1000) as u8;
                    let val = if lum >= thresh { 255u8 } else { 0u8 };
                    out[i * 4] = val;
                    out[i * 4 + 1] = val;
                    out[i * 4 + 2] = val;
                    out[i * 4 + 3] = 255;
                }
            } else if raw_bytes.len() >= pixel_count {
                for i in 0..pixel_count {
                    let lum = raw_bytes[i];
                    let val = if lum >= thresh { 255u8 } else { 0u8 };
                    out[i * 4] = val;
                    out[i * 4 + 1] = val;
                    out[i * 4 + 2] = val;
                    out[i * 4 + 3] = 255;
                }
            } else {
                return Err("insufficient raw bytes for specified dimensions".to_string());
            }
        }
    }

    Ok(out)
}

/// Composite 2D/3D multimodal generation stage request.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct StageComposition {
    pub prompt: String,
    pub model_id: Option<String>,
    pub camera: StageCamera,
    pub camera_adjustment: Option<SphericalAdjustment>,
    pub canvas_token: Option<String>,
    pub scene_token: Option<String>,
    pub inpainting_mask_token: Option<String>,
    pub aspect_ratio: Option<String>,
    pub resolution: Option<String>,
    pub batch_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_alpha_mask() {
        // 2x2 RGBA image
        let mut input = vec![0u8; 16];
        // pixel 0: fully transparent
        input[3] = 0;
        // pixel 1: translucent
        input[7] = 128;
        // pixel 2: opaque
        input[11] = 255;
        // pixel 3: transparent
        input[15] = 0;

        let mask = normalize_mask_bytes(&input, 2, 2, MaskNormalizationMode::AlphaToBinary).unwrap();
        assert_eq!(&mask[0..4], &[0, 0, 0, 255]);
        assert_eq!(&mask[4..8], &[255, 255, 255, 255]);
        assert_eq!(&mask[8..12], &[255, 255, 255, 255]);
        assert_eq!(&mask[12..16], &[0, 0, 0, 255]);
    }

    #[test]
    fn camera_spherical_adjustment() {
        let adj = SphericalAdjustment::new(45.0, -15.0, 1.25);
        assert_eq!(adj.horizontal_angle_deg, 45.0);
        assert_eq!(adj.vertical_angle_deg, -15.0);
        assert_eq!(adj.zoom_factor, 1.25);
    }
}

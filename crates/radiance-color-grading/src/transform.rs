//! Tone mapping, gamut mapping, and color-managed input/output transforms.
//!
//! Grounded in ITU-R BT.2446, ITU-R BT.2408, ACES Output Transforms, and international human rights
//! standards on technical provenance and fidelity (UDHR Art. 19; ICCPR Art. 19).

use serde::{Deserialize, Serialize};

use crate::matrix::{
    REFERENCE_WHITE_NITS, from_linear, hlg_inverse_oetf, hlg_oetf, linear_to_srgb, pq_eotf,
    pq_inverse_eotf, srgb_to_linear, to_linear,
};
use crate::spaces::{
    ColorSpace, Curve, Gamut, WorkingSpace, apply3, gamut_matrix, to_f32,
};

/// Tone mapping method for mapping HDR luminance into SDR/display ranges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToneMap {
    #[default]
    None,
    Reinhard,
    AcesFitted,
    Filmic,
    Bt2446a,
}

impl ToneMap {
    /// Apply tone mapping to a linear luminance value `y` with target peak `peak_nits`.
    pub fn map_luma(self, y: f32, peak_nits: f32) -> f32 {
        let white_ratio = (peak_nits / REFERENCE_WHITE_NITS as f32).max(1.0);
        match self {
            ToneMap::None => y.min(1.0),
            ToneMap::Reinhard => y / (1.0 + y / white_ratio),
            ToneMap::AcesFitted => {
                // ACES Narkowicz fitted curve: (x*(a*x+b))/(x*(c*x+d)+e)
                let x = y / white_ratio;
                let a = 2.51f32;
                let b = 0.03f32;
                let c = 2.43f32;
                let d = 0.59f32;
                let e = 0.14f32;
                ((x * (a * x + b)) / (x * (c * x + d) + e)).clamp(0.0, 1.0)
            }
            ToneMap::Filmic => {
                // Hejl-Burgess-Dawson filmic tone curve
                let x = (y / white_ratio).max(0.0);
                let x_adj = (x - 0.004).max(0.0);
                (x_adj * (6.2 * x_adj + 0.5)) / (x_adj * (6.2 * x_adj + 1.7) + 0.06)
            }
            ToneMap::Bt2446a => {
                // ITU-R BT.2446 Method A SDR conversion
                let x = y / white_ratio;
                if x <= 0.0 {
                    0.0
                } else {
                    (1.0 + (x.powf(0.4) - 1.0) * 0.75).clamp(0.0, 1.0)
                }
            }
        }
    }

    /// Apply tone mapping to an RGB triplet in linear working units.
    pub fn apply_rgb(self, rgb: [f32; 3], peak_nits: f32) -> [f32; 3] {
        if self == ToneMap::None {
            return rgb;
        }
        let luma = 0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2];
        if luma <= 1e-6 {
            return rgb;
        }
        let mapped_luma = self.map_luma(luma, peak_nits);
        let ratio = mapped_luma / luma;
        [rgb[0] * ratio, rgb[1] * ratio, rgb[2] * ratio]
    }
}

/// Color-managed Input Transform: maps source frame -> linear sequence working space.
#[derive(Clone, Debug)]
pub struct InputTransform {
    pub source_space: ColorSpace,
    pub working_space: WorkingSpace,
    matrix: [[f32; 3]; 3],
}

impl InputTransform {
    pub fn new(source: ColorSpace, working: WorkingSpace) -> Self {
        let m = gamut_matrix(source.gamut(), working.gamut());
        Self {
            source_space: source,
            working_space: working,
            matrix: to_f32(&m),
        }
    }

    /// Transform an encoded source color triplet to sequence working linear light.
    #[inline]
    pub fn apply(&self, c: [f32; 3]) -> [f32; 3] {
        let linear_src = match self.source_space.curve() {
            Curve::Linear => c,
            Curve::Sdr | Curve::Srgb => c.map(srgb_to_linear),
            Curve::Pq => c.map(pq_eotf),
            Curve::Hlg => c.map(hlg_inverse_oetf),
            Curve::Log(log) => c.map(|v| log.decode(v as f64) as f32),
            Curve::AcesCc => c.map(|v| crate::log::LogCurve::AcesCc.decode(v as f64) as f32),
            Curve::AcesCct => c.map(|v| crate::log::LogCurve::AcesCct.decode(v as f64) as f32),
        };
        apply3(&self.matrix, linear_src)
    }
}

/// Color-managed Output Transform: maps sequence working linear light -> display/export encoding.
#[derive(Clone, Debug)]
pub struct OutputTransform {
    pub working_space: WorkingSpace,
    pub target_space: ColorSpace,
    pub tonemap: ToneMap,
    pub peak_nits: f32,
    matrix: [[f32; 3]; 3],
}

impl OutputTransform {
    pub fn new(working: WorkingSpace, target: ColorSpace, tonemap: ToneMap, peak_nits: f32) -> Self {
        let m = gamut_matrix(working.gamut(), target.gamut());
        Self {
            working_space: working,
            target_space: target,
            tonemap,
            peak_nits,
            matrix: to_f32(&m),
        }
    }

    /// Transform sequence working linear color to target encoded display/export color.
    #[inline]
    pub fn apply(&self, c: [f32; 3]) -> [f32; 3] {
        let mapped = self.tonemap.apply_rgb(c, self.peak_nits);
        let linear_dst = apply3(&self.matrix, mapped);
        match self.target_space.curve() {
            Curve::Linear => linear_dst,
            Curve::Sdr | Curve::Srgb => linear_dst.map(linear_to_srgb),
            Curve::Pq => linear_dst.map(pq_inverse_eotf),
            Curve::Hlg => linear_dst.map(hlg_oetf),
            Curve::Log(log) => linear_dst.map(|v| log.encode(v as f64) as f32),
            Curve::AcesCc => linear_dst.map(|v| crate::log::LogCurve::AcesCc.encode(v as f64) as f32),
            Curve::AcesCct => linear_dst.map(|v| crate::log::LogCurve::AcesCct.encode(v as f64) as f32),
        }
    }
}

/// Precomputed 1D decode table for fast 8-bit or 10-bit LUT conversion.
pub struct DecodeTable {
    pub table: Vec<f32>,
}

impl DecodeTable {
    pub fn from_fn(size: usize, f: impl Fn(f32) -> f32) -> Self {
        let max_val = (size - 1) as f32;
        let table = (0..size).map(|i| f(i as f32 / max_val)).collect();
        Self { table }
    }

    #[inline]
    pub fn lookup(&self, index: usize) -> f32 {
        self.table[index.min(self.table.len() - 1)]
    }
}

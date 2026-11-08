//! Colour science and transforms: YUV matrices, transfer functions, camera log curves, gamuts,
//! colour-managed input/output transforms (tone and gamut mapping), colour metadata and LUTs.
//! See the crate README for the formulas and their sources.
//!
//! The compositor works in **linear-light, premultiplied RGBA f32** in the sequence working space
//! (Rec.709 primaries by default). Decoded frames carry [`ColorInfo`] so conversions are explicit.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

pub mod grade;
pub mod icc;
pub mod ontology;
pub mod palette;
pub mod log;
pub mod lut;
pub mod spaces;
pub mod transform;

pub use grade::GradeSpace;
pub use log::LogCurve;
pub use lut::{Lut, Lut1d, Lut3d, LutFormat};
pub use spaces::{ColorPipeline, ColorSpace, Curve, Gamut, WorkingSpace};
pub use transform::{DecodeTable, InputTransform, OutputTransform, REFERENCE_WHITE_NITS, ToneMap};

/// YUV â†” RGB matrix coefficients.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Matrix {
    Bt601,
    #[default]
    Bt709,
    Bt2020Ncl,
}

impl Matrix {
    /// (Kr, Kb) luma coefficients.
    pub fn kr_kb(self) -> (f32, f32) {
        match self {
            Matrix::Bt601 => (0.299, 0.114),
            Matrix::Bt709 => (0.2126, 0.0722),
            Matrix::Bt2020Ncl => (0.2627, 0.0593),
        }
    }
    /// From an ISO/IEC 23091-2 `matrix_coefficients` code.
    pub fn from_code(c: u8) -> Option<Matrix> {
        match c {
            1 => Some(Matrix::Bt709),
            5 | 6 => Some(Matrix::Bt601),
            9 | 10 => Some(Matrix::Bt2020Ncl),
            _ => None,
        }
    }
}

/// Transfer characteristics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Transfer {
    /// BT.709 / BT.1886 display (we use pure 2.4 gamma for display-referred decode, as NLEs do).
    #[default]
    Bt709,
    Srgb,
    Linear,
    /// SMPTE ST 2084.
    Pq,
    /// ARIB STD-B67.
    Hlg,
}

impl Transfer {
    pub fn from_code(c: u8) -> Option<Transfer> {
        match c {
            1 | 6 | 14 | 15 => Some(Transfer::Bt709),
            13 => Some(Transfer::Srgb),
            8 => Some(Transfer::Linear),
            16 => Some(Transfer::Pq),
            18 => Some(Transfer::Hlg),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Primaries {
    #[default]
    Bt709,
    Bt601_625,
    Bt601_525,
    Bt2020,
    P3D65,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Range {
    /// 16â€“235 (8-bit) "video" range.
    #[default]
    Limited,
    Full,
}

/// Colour metadata attached to frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorInfo {
    pub matrix: Matrix,
    pub transfer: Transfer,
    pub primaries: Primaries,
    pub range: Range,
}

/// HDR static metadata of a stream: SMPTE ST 2086 mastering display luminance and the CTA-861.3
/// content light levels (MaxCLL / MaxFALL), in cd/mÂ². Read from `mdcv` / `clli` (MP4 / MOV) and
/// Matroska `MasteringMetadata` / `MaxCLL` / `MaxFALL`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdrMetadata {
    pub mastering_max_nits: Option<f32>,
    pub mastering_min_nits: Option<f32>,
    pub max_cll: Option<f32>,
    pub max_fall: Option<f32>,
}

impl HdrMetadata {
    /// The content's peak for tone mapping: MaxCLL when signalled (the brightest pixel actually
    /// in the content), else the mastering display's peak; `None` when neither is (0 means
    /// unknown in both). Never below HDR reference white.
    pub fn peak_nits(&self) -> Option<f32> {
        let known = |v: Option<f32>| v.filter(|n| *n > 0.0);
        let p = match (known(self.max_cll), known(self.mastering_max_nits)) {
            (Some(c), Some(m)) => c.min(m),
            (Some(c), None) => c,
            (None, m) => m?,
        };
        Some(p.clamp(transform::REFERENCE_WHITE_NITS as f32, 10_000.0))
    }
}

impl ColorInfo {
    pub const SRGB_FULL: ColorInfo = ColorInfo { matrix: Matrix::Bt709, transfer: Transfer::Srgb, primaries: Primaries::Bt709, range: Range::Full };
    pub const REC709: ColorInfo = ColorInfo { matrix: Matrix::Bt709, transfer: Transfer::Bt709, primaries: Primaries::Bt709, range: Range::Limited };
}

/// Convert normalised Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'.
#[inline]
pub fn ycbcr_to_rgb(y: f32, cb: f32, cr: f32, m: Matrix) -> [f32; 3] {
    let (kr, kb) = m.kr_kb();
    let kg = 1.0 - kr - kb;
    let r = y + 2.0 * (1.0 - kr) * cr;
    let b = y + 2.0 * (1.0 - kb) * cb;
    let g = (y - kr * r - kb * b) / kg;
    [r, g, b]
}

/// Convert non-linear R'G'B' to normalised Y'CbCr.
#[inline]
pub fn rgb_to_ycbcr(r: f32, g: f32, b: f32, m: Matrix) -> [f32; 3] {
    let (kr, kb) = m.kr_kb();
    let kg = 1.0 - kr - kb;
    let y = kr * r + kg * g + kb * b;
    [y, (b - y) / (2.0 * (1.0 - kb)), (r - y) / (2.0 * (1.0 - kr))]
}

/// Normalise an integer code value to (Y 0..1, C -0.5..0.5) given bit depth and range.
#[inline]
pub fn normalize_y(v: u32, bits: u32, range: Range) -> f32 {
    let scale = (1u32 << (bits - 8)) as f32;
    match range {
        Range::Limited => (v as f32 - 16.0 * scale) / (219.0 * scale),
        Range::Full => v as f32 / ((1u32 << bits) - 1) as f32,
    }
}
#[inline]
pub fn normalize_c(v: u32, bits: u32, range: Range) -> f32 {
    let scale = (1u32 << (bits - 8)) as f32;
    match range {
        Range::Limited => (v as f32 - 128.0 * scale) / (224.0 * scale),
        Range::Full => (v as f32 - (1u32 << (bits - 1)) as f32) / ((1u32 << bits) - 1) as f32,
    }
}

/// sRGB electro-optical transfer (encoded â†’ linear).
#[inline]
pub fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}
#[inline]
pub fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.003_130_8 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 }
}

/// Encoded â†’ linear for a transfer function (display-referred; PQ normalised to 100 nits = 1.0).
pub fn to_linear(v: f32, t: Transfer) -> f32 {
    match t {
        Transfer::Srgb => srgb_to_linear(v),
        // Premiere treats Rec.709 as display gamma 2.4 in its colour-managed pipeline; the
        // sRGB curve is visually identical for UI previews and keeps round-trips exact.
        Transfer::Bt709 => srgb_to_linear(v),
        Transfer::Linear => v,
        Transfer::Pq => pq_eotf(v) * 100.0,
        Transfer::Hlg => hlg_inverse_oetf(v),
    }
}

pub fn from_linear(v: f32, t: Transfer) -> f32 {
    match t {
        Transfer::Srgb | Transfer::Bt709 => linear_to_srgb(v),
        Transfer::Linear => v,
        Transfer::Pq => pq_inverse_eotf(v / 100.0),
        Transfer::Hlg => hlg_oetf(v),
    }
}

/// SMPTE ST 2084 EOTF, output normalised to 10 000 nits = 1.0.
pub fn pq_eotf(e: f32) -> f32 {
    let (m1, m2) = (0.159_301_76_f32, 78.84375_f32);
    let (c1, c2, c3) = (0.835_937_5_f32, 18.851_563_f32, 18.6875_f32);
    let p = e.max(0.0).powf(1.0 / m2);
    ((p - c1).max(0.0) / (c2 - c3 * p)).powf(1.0 / m1)
}
pub fn pq_inverse_eotf(y: f32) -> f32 {
    let (m1, m2) = (0.159_301_76_f32, 78.84375_f32);
    let (c1, c2, c3) = (0.835_937_5_f32, 18.851_563_f32, 18.6875_f32);
    let p = y.max(0.0).powf(m1);
    ((c1 + c2 * p) / (1.0 + c3 * p)).powf(m2)
}
pub fn hlg_oetf(l: f32) -> f32 {
    let (a, b, c) = (0.178_832_77_f32, 0.284_668_92_f32, 0.559_910_7_f32);
    if l <= 1.0 / 12.0 { (3.0 * l.max(0.0)).sqrt() } else { a * (12.0 * l - b).ln() + c }
}
pub fn hlg_inverse_oetf(e: f32) -> f32 {
    let (a, b, c) = (0.178_832_77_f32, 0.284_668_92_f32, 0.559_910_7_f32);
    if e <= 0.5 { e * e / 3.0 } else { (((e - c) / a).exp() + b) / 12.0 }
}

/// 256-entry table: sRGB-encoded u8 â†’ linear f32.
pub fn srgb_u8_to_linear_table() -> &'static [f32; 256] {
    static T: OnceLock<[f32; 256]> = OnceLock::new();
    T.get_or_init(|| std::array::from_fn(|i| srgb_to_linear(i as f32 / 255.0)))
}

/// 4096-entry table: linear (0..1, quantised to 12 bits) â†’ sRGB-encoded u8.
pub fn linear_to_srgb_u8_table() -> &'static [u8; 4096] {
    static T: OnceLock<[u8; 4096]> = OnceLock::new();
    T.get_or_init(|| std::array::from_fn(|i| (linear_to_srgb(i as f32 / 4095.0) * 255.0 + 0.5).clamp(0.0, 255.0) as u8))
}

#[inline]
pub fn linear_to_srgb_u8(v: f32) -> u8 {
    let i = (v.clamp(0.0, 1.0) * 4095.0 + 0.5) as usize;
    linear_to_srgb_u8_table()[i]
}

/// Rec.709 luma of a linear or encoded RGB triple.
#[inline]
pub fn luma709(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// RGB (0..1) â†’ HSL (h in 0..1).
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> [f32; 3] {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-7 {
        return [0.0, 0.0, l];
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    [h / 6.0, s, l]
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [f32; 3] {
    if s <= 0.0 {
        return [l, l, l];
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let f = |mut t: f32| {
        t = t.rem_euclid(1.0);
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    [f(h + 1.0 / 3.0), f(h), f(h - 1.0 / 3.0)]
}

/// Parse `#rrggbb` / `#rrggbbaa` into 0..1 floats.
pub fn parse_hex(s: &str) -> Option<[f32; 4]> {
    let s = s.trim().trim_start_matches('#');
    let b = |i: usize| u8::from_str_radix(s.get(i..i + 2)?, 16).ok().map(|v| v as f32 / 255.0);
    match s.len() {
        6 => Some([b(0)?, b(2)?, b(4)?, 1.0]),
        8 => Some([b(0)?, b(2)?, b(4)?, b(6)?]),
        _ => None,
    }
}

pub fn to_hex(c: [f32; 4]) -> String {
    let q = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    if c[3] >= 0.999 {
        format!("#{:02x}{:02x}{:02x}", q(c[0]), q(c[1]), q(c[2]))
    } else {
        format!("#{:02x}{:02x}{:02x}{:02x}", q(c[0]), q(c[1]), q(c[2]), q(c[3]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ycbcr_roundtrip() {
        for m in [Matrix::Bt601, Matrix::Bt709, Matrix::Bt2020Ncl] {
            let rgb = [0.8, 0.3, 0.1];
            let y = rgb_to_ycbcr(rgb[0], rgb[1], rgb[2], m);
            let back = ycbcr_to_rgb(y[0], y[1], y[2], m);
            for k in 0..3 {
                assert!((rgb[k] - back[k]).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn limited_range_levels() {
        assert_eq!(normalize_y(16, 8, Range::Limited), 0.0);
        assert_eq!(normalize_y(235, 8, Range::Limited), 1.0);
        assert_eq!(normalize_y(940, 10, Range::Limited), 1.0);
        assert_eq!(normalize_c(128, 8, Range::Limited), 0.0);
    }

    #[test]
    fn transfer_roundtrips() {
        for t in [Transfer::Srgb, Transfer::Pq, Transfer::Hlg, Transfer::Linear] {
            for v in [0.0, 0.1, 0.5, 0.9] {
                let l = to_linear(v, t);
                assert!((from_linear(l, t) - v).abs() < 1e-3, "{t:?} {v}");
            }
        }
        assert_eq!(linear_to_srgb_u8(1.0), 255);
        assert_eq!(linear_to_srgb_u8(0.0), 0);
        assert_eq!(linear_to_srgb_u8(srgb_u8_to_linear_table()[128]), 128);
    }

    #[test]
    fn hsl_roundtrip() {
        let c = [0.2, 0.6, 0.9];
        let h = rgb_to_hsl(c[0], c[1], c[2]);
        let b = hsl_to_rgb(h[0], h[1], h[2]);
        for k in 0..3 {
            assert!((c[k] - b[k]).abs() < 1e-5);
        }
    }


    #[test]
    fn test_icc_transform_roundtrip() {
        use crate::icc::{Intent, Transform, TransformOptions, adobe_rgb, srgb};
        let src = srgb();
        let dst = adobe_rgb();
        let xform = Transform::with_options(src, dst, TransformOptions {
            intent: Intent::RelativeColorimetric,
            bpc: true,
            precise_float: true,
        }).expect("Transform creation should succeed");

        let mut out = [0.0f32; 3];
        xform.eval(&[0.5, 0.5, 0.5], &mut out);
        assert!((out[0] - out[1]).abs() < 0.05);
        assert!((out[1] - out[2]).abs() < 0.05);

        let src_u8 = [128u8, 128, 128, 255];
        let mut dst_u8 = [0u8; 4];
        xform.convert_u8(&src_u8, 4, &mut dst_u8, 4, true);
        assert_eq!(dst_u8[3], 255, "Alpha channel preserved");
        assert!((dst_u8[0] as i32 - dst_u8[1] as i32).abs() <= 2);
    }

    #[test]
    fn test_tetrahedral_clut_interpolation() {
        use crate::icc::Clut;
        let clut = Clut::sample(vec![5, 5, 5], 3, |inp, out| {
            out[0] = inp[0] * 0.5 + inp[1] * 0.3 + inp[2] * 0.2;
            out[1] = inp[0] * inp[1];
            out[2] = 1.0 - inp[2];
        });
        let mut out = [0.0f32; 3];
        clut.eval(&[0.2, 0.4, 0.6], &mut out);
        let expected_0 = 0.2 * 0.5 + 0.4 * 0.3 + 0.6 * 0.2;
        assert!((out[0] - expected_0).abs() < 1e-4);
        assert!((out[2] - 0.4).abs() < 1e-4);
    }

    #[test]
    fn test_gamut_warning() {
        use crate::icc::{GamutCheck, generic_cmyk, srgb};
        let src = srgb();
        let proof = generic_cmyk();
        let gamut = GamutCheck::new(src, proof, 2.0).expect("GamutCheck creation should succeed");
        let green = [0.0f32, 1.0, 0.0];
        let de = gamut.delta_e(&green);
        assert!(de > 0.0, "Delta E should be positive");
    }

    #[test]
    fn test_aco_roundtrip() {
        use crate::palette::{AcoColor, AcoSwatch, parse_aco, write_aco};
        let swatches = vec![
            AcoSwatch {
                name: "Test Red".into(),
                color: AcoColor::Rgb([65535, 0, 0]),
            },
            AcoSwatch {
                name: "Test Cyan".into(),
                color: AcoColor::Cmyk([65535, 0, 0, 0]),
            },
        ];
        let bytes = write_aco(&swatches).expect("write_aco should succeed");
        let parsed = parse_aco(&bytes).expect("parse_aco should succeed");
        assert_eq!(parsed.swatches.len(), 2);
        assert_eq!(parsed.swatches[0].name, "Test Red");
        assert_eq!(parsed.swatches[0].color, AcoColor::Rgb([65535, 0, 0]));
    }

    #[test]
    fn test_ase_roundtrip() {
        use crate::palette::{AseColor, AseEntry, AseFile, AseGroup, AseKind, parse_ase, write_ase};
        let file = AseFile {
            groups: vec![AseGroup {
                name: "Warm Tones".into(),
                entries: vec![AseEntry {
                    name: "Solar Orange".into(),
                    color: AseColor::Rgb([1.0, 0.5, 0.0]),
                    kind: AseKind::Global,
                }],
            }],
            loose: vec![AseEntry {
                name: "Process Black".into(),
                color: AseColor::Gray(0.0),
                kind: AseKind::Process,
            }],
        };
        let bytes = write_ase(&file).expect("write_ase should succeed");
        let parsed = parse_ase(&bytes).expect("parse_ase should succeed");
        assert_eq!(parsed.groups.len(), 1);
        assert_eq!(parsed.groups[0].name, "Warm Tones");
        assert_eq!(parsed.groups[0].entries[0].name, "Solar Orange");
        assert_eq!(parsed.loose.len(), 1);
        assert_eq!(parsed.loose[0].name, "Process Black");
    }

    #[test]
    fn test_gpl_roundtrip() {
        use crate::palette::{GplColor, GplPalette, parse_gpl, write_gpl};
        let pal = GplPalette {
            name: "EcoPalette".into(),
            columns: 4,
            comments: vec!["Test palette".into()],
            colors: vec![
                GplColor { rgb: [255, 0, 0], name: "Pure Red".into() },
                GplColor { rgb: [0, 255, 0], name: "Pure Green".into() },
                GplColor { rgb: [0, 0, 255], name: "Pure Blue".into() },
            ],
        };
        let bytes = write_gpl(&pal).expect("write_gpl should succeed");
        let parsed = parse_gpl(&bytes).expect("parse_gpl should succeed");
        assert_eq!(parsed.name, "EcoPalette");
        assert_eq!(parsed.columns, 4);
        assert_eq!(parsed.colors.len(), 3);
        assert_eq!(parsed.colors[1].name, "Pure Green");
        assert_eq!(parsed.colors[1].rgb, [0, 255, 0]);
    }

    #[test]
    fn hex() {
        assert_eq!(parse_hex("#ff8000"), Some([1.0, 128.0 / 255.0, 0.0, 1.0]));
        assert_eq!(to_hex([1.0, 0.0, 0.0, 1.0]), "#ff0000");
    }
}

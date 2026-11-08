//! YUV/YCbCr matrices, transfer characteristics, primaries, range, and color encoding functions.
//!
//! Grounded in international standards: ITU-R BT.709, BT.2020, BT.2100, SMPTE ST 2084 (PQ),
//! ARIB STD-B67 (HLG), and international human rights instruments protecting creative expression
//! and cultural participation (UDHR Art. 19, 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Standard HDR reference white luminance (cd/m²), matching ITU-R BT.2408.
pub const REFERENCE_WHITE_NITS: f64 = 203.0;

/// YUV <-> RGB matrix coefficients.
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
    /// BT.709 / BT.1886 display (display gamma 2.4).
    #[default]
    Bt709,
    Srgb,
    Linear,
    /// SMPTE ST 2084.
    Pq,
    /// ARIB STD-B67 (HLG).
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

/// Color primaries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Primaries {
    #[default]
    Bt709,
    Bt601_625,
    Bt601_525,
    Bt2020,
    P3D65,
}

/// Luma quantization range.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Range {
    /// 16..235 (8-bit) "video" range.
    #[default]
    Limited,
    Full,
}

/// Color metadata attached to frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorInfo {
    pub matrix: Matrix,
    pub transfer: Transfer,
    pub primaries: Primaries,
    pub range: Range,
}

impl ColorInfo {
    pub const SRGB_FULL: ColorInfo = ColorInfo {
        matrix: Matrix::Bt709,
        transfer: Transfer::Srgb,
        primaries: Primaries::Bt709,
        range: Range::Full,
    };

    pub const REC709: ColorInfo = ColorInfo {
        matrix: Matrix::Bt709,
        transfer: Transfer::Bt709,
        primaries: Primaries::Bt709,
        range: Range::Limited,
    };
}

/// HDR static metadata of a stream.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HdrMetadata {
    pub mastering_max_nits: Option<f32>,
    pub mastering_min_nits: Option<f32>,
    pub max_cll: Option<f32>,
    pub max_fall: Option<f32>,
}

impl HdrMetadata {
    /// The content peak for tone mapping.
    pub fn peak_nits(&self) -> Option<f32> {
        let known = |v: Option<f32>| v.filter(|n| *n > 0.0);
        let p = match (known(self.max_cll), known(self.mastering_max_nits)) {
            (Some(c), Some(m)) => c.min(m),
            (Some(c), None) => c,
            (None, m) => m?,
        };
        Some(p.clamp(REFERENCE_WHITE_NITS as f32, 10_000.0))
    }
}

/// Convert normalized Y'CbCr (Y in 0..1, Cb/Cr in -0.5..0.5) to non-linear R'G'B'.
#[inline]
pub fn ycbcr_to_rgb(y: f32, cb: f32, cr: f32, m: Matrix) -> [f32; 3] {
    let (kr, kb) = m.kr_kb();
    let kg = 1.0 - kr - kb;
    let r = y + 2.0 * (1.0 - kr) * cr;
    let b = y + 2.0 * (1.0 - kb) * cb;
    let g = y - (2.0 * kr * (1.0 - kr) * cr + 2.0 * kb * (1.0 - kb) * cb) / kg;
    [r, g, b]
}

/// Convert non-linear R'G'B' (nominal 0..1) to normalized Y'CbCr.
#[inline]
pub fn rgb_to_ycbcr(r: f32, g: f32, b: f32, m: Matrix) -> [f32; 3] {
    let (kr, kb) = m.kr_kb();
    let kg = 1.0 - kr - kb;
    let y = kr * r + kg * g + kb * b;
    let cb = (b - y) / (2.0 * (1.0 - kb));
    let cr = (r - y) / (2.0 * (1.0 - kr));
    [y, cb, cr]
}

/// Quantized integer sample -> normalized Y float (0..1 nominal).
#[inline]
pub fn normalize_y(v: u32, bit_depth: u32, range: Range) -> f32 {
    let max = ((1u32 << bit_depth) - 1) as f32;
    match range {
        Range::Full => v as f32 / max,
        Range::Limited => {
            let shift = bit_depth - 8;
            let black = (16u32 << shift) as f32;
            let white = (235u32 << shift) as f32;
            (v as f32 - black) / (white - black)
        }
    }
}

/// Quantized integer sample -> normalized Cb/Cr float (-0.5..0.5 nominal).
#[inline]
pub fn normalize_c(v: u32, bit_depth: u32, range: Range) -> f32 {
    let max = ((1u32 << bit_depth) - 1) as f32;
    match range {
        Range::Full => v as f32 / max - 0.5,
        Range::Limited => {
            let shift = bit_depth - 8;
            let mid = (128u32 << shift) as f32;
            let span = (112u32 << shift) as f32 * 2.0;
            (v as f32 - mid) / span
        }
    }
}

/// Normalized Y float (0..1 nominal) -> integer sample clamped to range.
#[inline]
pub fn denormalize_y(v: f32, bit_depth: u32, range: Range) -> u32 {
    let max = (1u32 << bit_depth) - 1;
    match range {
        Range::Full => (v * max as f32 + 0.5).clamp(0.0, max as f32) as u32,
        Range::Limited => {
            let shift = bit_depth - 8;
            let black = 16u32 << shift;
            let white = 235u32 << shift;
            let scaled = v * (white - black) as f32 + black as f32 + 0.5;
            scaled.clamp(1.0, (max - 1) as f32) as u32
        }
    }
}

/// Normalized Cb/Cr float (-0.5..0.5 nominal) -> integer sample clamped to range.
#[inline]
pub fn denormalize_c(v: f32, bit_depth: u32, range: Range) -> u32 {
    let max = (1u32 << bit_depth) - 1;
    match range {
        Range::Full => ((v + 0.5) * max as f32 + 0.5).clamp(0.0, max as f32) as u32,
        Range::Limited => {
            let shift = bit_depth - 8;
            let mid = 128u32 << shift;
            let span = (112u32 << shift) as f32 * 2.0;
            let scaled = v * span + mid as f32 + 0.5;
            scaled.clamp(1.0, (max - 1) as f32) as u32
        }
    }
}

/// Convert encoded display value to linear light.
#[inline]
pub fn to_linear(v: f32, t: Transfer) -> f32 {
    match t {
        Transfer::Linear => v,
        Transfer::Srgb => srgb_to_linear(v),
        Transfer::Bt709 => {
            if v <= 0.0 {
                0.0
            } else {
                v.powf(2.4)
            }
        }
        Transfer::Pq => pq_eotf(v),
        Transfer::Hlg => hlg_inverse_oetf(v),
    }
}

/// Convert linear light to encoded display value.
#[inline]
pub fn from_linear(v: f32, t: Transfer) -> f32 {
    match t {
        Transfer::Linear => v,
        Transfer::Srgb => linear_to_srgb(v),
        Transfer::Bt709 => {
            if v <= 0.0 {
                0.0
            } else {
                v.powf(1.0 / 2.4)
            }
        }
        Transfer::Pq => pq_inverse_eotf(v),
        Transfer::Hlg => hlg_oetf(v),
    }
}

#[inline]
pub fn srgb_to_linear(v: f32) -> f32 {
    let s = v.abs();
    let l = if s <= 0.04045 {
        s / 12.92
    } else {
        ((s + 0.055) / 1.055).powf(2.4)
    };
    l.copysign(v)
}

#[inline]
pub fn linear_to_srgb(v: f32) -> f32 {
    let s = v.abs();
    let e = if s <= 0.0031308 {
        s * 12.92
    } else {
        1.055 * s.powf(1.0 / 2.4) - 0.055
    };
    e.copysign(v)
}

pub fn pq_eotf(e: f32) -> f32 {
    let (m1, m2, c1, c2, c3) = (
        0.1593017578125_f32,
        78.84375_f32,
        0.8359375_f32,
        18.8515625_f32,
        18.6875_f32,
    );
    let p = e.max(0.0).powf(1.0 / m2);
    let num = (p - c1).max(0.0);
    let den = c2 - c3 * p;
    if den <= 0.0 {
        return 0.0;
    }
    (num / den).powf(1.0 / m1)
}

pub fn pq_inverse_eotf(y: f32) -> f32 {
    let (m1, m2, c1, c2, c3) = (
        0.1593017578125_f32,
        78.84375_f32,
        0.8359375_f32,
        18.8515625_f32,
        18.6875_f32,
    );
    let p = y.max(0.0).powf(m1);
    ((c1 + c2 * p) / (1.0 + c3 * p)).powf(m2)
}

pub fn hlg_oetf(l: f32) -> f32 {
    let (a, b, c) = (0.17883277_f32, 0.28466892_f32, 0.5599107_f32);
    if l <= 1.0 / 12.0 {
        (3.0 * l.max(0.0)).sqrt()
    } else {
        a * (12.0 * l - b).ln() + c
    }
}

pub fn hlg_inverse_oetf(e: f32) -> f32 {
    let (a, b, c) = (0.17883277_f32, 0.28466892_f32, 0.5599107_f32);
    if e <= 0.5 {
        e * e / 3.0
    } else {
        (((e - c) / a).exp() + b) / 12.0
    }
}

/// 256-entry table: sRGB-encoded u8 -> linear f32.
pub fn srgb_u8_to_linear_table() -> &'static [f32; 256] {
    static T: OnceLock<[f32; 256]> = OnceLock::new();
    T.get_or_init(|| std::array::from_fn(|i| srgb_to_linear(i as f32 / 255.0)))
}

/// 4096-entry table: linear (0..1 quantised to 12 bits) -> sRGB u8.
pub fn linear_to_srgb_u8_table() -> &'static [u8; 4096] {
    static T: OnceLock<[u8; 4096]> = OnceLock::new();
    T.get_or_init(|| {
        std::array::from_fn(|i| {
            (linear_to_srgb(i as f32 / 4095.0) * 255.0 + 0.5).clamp(0.0, 255.0) as u8
        })
    })
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

/// RGB (0..1) -> HSL (h in 0..1).
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> [f32; 3] {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-7 {
        return [0.0, 0.0, l];
    }
    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
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
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
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

//! Built-in standard ICC profiles.
//!
//! Generated from published colorimetric definitions (primaries, white points, transfer functions):
//! sRGB, Display P3, Adobe RGB 1998, ProPhoto RGB, Linear sRGB, Rec. 2020, Gray Gamma 2.2, sGray,
//! Lab D50, and Synthetic Coated CMYK.
//!
//! Grounded in UDHR Art. 27 and ICESCR Art. 15.

use std::sync::OnceLock;

use crate::icc::curve::{Curve, srgb_trc};
use crate::icc::math::{self, Mat3};
use crate::icc::pipeline::Stage;
use crate::icc::profile::{ColorSpace, Lut, LutKind, Pcs, Profile, ProfileClass};
use crate::icc::synth;

pub const COPYRIGHT: &str = "No copyright: CC0-1.0 public domain dedication.";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Builtin {
    Srgb,
    DisplayP3,
    AdobeRgbCompat,
    ProPhotoCompat,
    LinearSrgb,
    Rec2020,
    GrayGamma22,
    SGray,
    LabD50,
    CoatedCmyk,
}

impl Builtin {
    pub const ALL: [Builtin; 10] = [
        Builtin::Srgb,
        Builtin::DisplayP3,
        Builtin::AdobeRgbCompat,
        Builtin::ProPhotoCompat,
        Builtin::LinearSrgb,
        Builtin::Rec2020,
        Builtin::GrayGamma22,
        Builtin::SGray,
        Builtin::LabD50,
        Builtin::CoatedCmyk,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Builtin::Srgb => "srgb",
            Builtin::DisplayP3 => "display-p3",
            Builtin::AdobeRgbCompat => "adobe-rgb-compat",
            Builtin::ProPhotoCompat => "prophoto-compat",
            Builtin::LinearSrgb => "linear-srgb",
            Builtin::Rec2020 => "rec2020",
            Builtin::GrayGamma22 => "gray-gamma22",
            Builtin::SGray => "sgray",
            Builtin::LabD50 => "lab-d50",
            Builtin::CoatedCmyk => "coated-cmyk",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Builtin::Srgb => "sRGB IEC 61966-2.1",
            Builtin::DisplayP3 => "Display P3",
            Builtin::AdobeRgbCompat => "Adobe RGB (1998) Compatible",
            Builtin::ProPhotoCompat => "ProPhoto RGB Compatible (ROMM RGB)",
            Builtin::LinearSrgb => "Linear sRGB",
            Builtin::Rec2020 => "ITU-R BT.2020",
            Builtin::GrayGamma22 => "Gray Gamma 2.2",
            Builtin::SGray => "sGray (sRGB Tone Curve)",
            Builtin::LabD50 => "CIE L*a*b* D50 Identity",
            Builtin::CoatedCmyk => synth::DESCRIPTION,
        }
    }

    pub fn profile(self) -> &'static Profile {
        static CELLS: [OnceLock<Profile>; 10] = [const { OnceLock::new() }; 10];
        CELLS[self as usize].get_or_init(|| build(self))
    }
}

pub fn srgb() -> &'static Profile {
    Builtin::Srgb.profile()
}

pub fn display_p3() -> &'static Profile {
    Builtin::DisplayP3.profile()
}

pub fn adobe_rgb() -> &'static Profile {
    Builtin::AdobeRgbCompat.profile()
}

pub fn prophoto_rgb() -> &'static Profile {
    Builtin::ProPhotoCompat.profile()
}

pub fn rec2020() -> &'static Profile {
    Builtin::Rec2020.profile()
}

pub fn generic_cmyk() -> &'static Profile {
    Builtin::CoatedCmyk.profile()
}

fn d65() -> [f64; 3] {
    math::xy_to_xyz(math::D65_XY)
}

fn colorants(r: [f64; 2], g: [f64; 2], b: [f64; 2], white: [f64; 3]) -> Mat3 {
    let m = math::rgb_to_xyz_matrix(r, g, b, white).unwrap_or(math::IDENTITY);
    let d50 = math::d50_quantized();
    let adapt = if (white[0] - math::D50[0]).abs() < 1e-3 && (white[2] - math::D50[2]).abs() < 1e-3 {
        None
    } else {
        Some(math::bradford(white, math::D50))
    };
    let m50 = match &adapt {
        Some(a) => math::mul(a, &m),
        None => {
            let s = math::apply(&m, [1.0, 1.0, 1.0]);
            let mut o = m;
            for (i, row) in o.iter_mut().enumerate() {
                for v in row.iter_mut() {
                    *v *= math::D50[i] / s[i];
                }
            }
            o
        }
    };
    let mut q = m50.map(|row| row.map(math::s15f16_round));
    for (i, row) in q.iter_mut().enumerate() {
        let err = d50[i] - row.iter().sum::<f64>();
        let k = (0..3).max_by(|a, b| row[*a].abs().total_cmp(&row[*b].abs())).unwrap_or(0);
        row[k] = math::s15f16_round(row[k] + err);
    }
    q
}

fn rgb(desc: &str, prim: [[f64; 2]; 3], white: [f64; 3], trc: Curve) -> Profile {
    let m = colorants(prim[0], prim[1], prim[2], white);
    Profile {
        version: (4, 0x30),
        class: ProfileClass::Display,
        color_space: ColorSpace::Rgb,
        pcs: Pcs::Xyz,
        description: desc.into(),
        white_point: math::d50_quantized(),
        matrix: Some(m),
        trc: Some([trc.clone(), trc.clone(), trc]),
        gray_trc: None,
        a2b: [None, None, None],
        b2a: [None, None, None],
        raw_bytes: None,
    }
}

fn gray(desc: &str, trc: Curve) -> Profile {
    Profile {
        version: (4, 0x30),
        class: ProfileClass::Display,
        color_space: ColorSpace::Gray,
        pcs: Pcs::Xyz,
        description: desc.into(),
        white_point: math::d50_quantized(),
        matrix: None,
        trc: None,
        gray_trc: Some(trc),
        a2b: [None, None, None],
        b2a: [None, None, None],
        raw_bytes: None,
    }
}

fn lab() -> Profile {
    let ident = || Lut {
        kind: LutKind::Ab,
        inputs: 3,
        outputs: 3,
        stages: vec![Stage::Curves(vec![Curve::Identity; 3])],
    };
    Profile {
        version: (4, 0x30),
        class: ProfileClass::ColorSpace,
        color_space: ColorSpace::Lab,
        pcs: Pcs::Lab,
        description: Builtin::LabD50.description().into(),
        white_point: math::d50_quantized(),
        matrix: None,
        trc: None,
        gray_trc: None,
        a2b: [Some(ident()), None, None],
        b2a: [Some(ident()), None, None],
        raw_bytes: None,
    }
}

fn build(b: Builtin) -> Profile {
    let (srgb_p, p3, adobe, romm, bt2020) = (
        [[0.64, 0.33], [0.30, 0.60], [0.15, 0.06]],
        [[0.680, 0.320], [0.265, 0.690], [0.150, 0.060]],
        [[0.64, 0.33], [0.21, 0.71], [0.15, 0.06]],
        [[0.7347, 0.2653], [0.1596, 0.8404], [0.0366, 0.0001]],
        [[0.708, 0.292], [0.170, 0.797], [0.131, 0.046]],
    );
    match b {
        Builtin::Srgb => rgb(b.description(), srgb_p, d65(), srgb_trc()),
        Builtin::DisplayP3 => rgb(b.description(), p3, d65(), srgb_trc()),
        Builtin::AdobeRgbCompat => rgb(b.description(), adobe, d65(), Curve::Gamma(563.0 / 256.0)),
        Builtin::ProPhotoCompat => rgb(b.description(), romm, math::D50, Curve::Gamma(1.8)),
        Builtin::LinearSrgb => rgb(b.description(), srgb_p, d65(), Curve::Gamma(1.0)),
        Builtin::Rec2020 => {
            let a = 1.099;
            let trc = Curve::Parametric {
                kind: 3,
                p: [1.0 / 0.45, 1.0 / a, (a - 1.0) / a, 1.0 / 4.5, 0.081, 0.0, 0.0],
            };
            rgb(b.description(), bt2020, d65(), trc)
        }
        Builtin::GrayGamma22 => gray(b.description(), Curve::Gamma(2.2)),
        Builtin::SGray => gray(b.description(), srgb_trc()),
        Builtin::LabD50 => lab(),
        Builtin::CoatedCmyk => synth::coated_cmyk(),
    }
}

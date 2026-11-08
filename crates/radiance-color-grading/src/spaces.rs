//! Color spaces, RGB primaries, chromatic adaptation, and working space management.
//!
//! Grounded in SMPTE RP 177, SMPTE ST 2065-1 (ACES2065-1 AP0), Academy S-2014-004 (ACEScg AP1),
//! Academy S-2014-003 (ACEScc), and Academy S-2016-001 (ACEScct).

use serde::{Deserialize, Serialize};

use crate::log::LogCurve;
use crate::matrix::{ColorInfo, Primaries, Transfer};

pub type Mat3 = [[f64; 3]; 3];

pub const D65: [f64; 2] = [0.3127, 0.3290];
pub const ACES_WHITE: [f64; 2] = [0.32168, 0.33767];

/// RGB primaries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gamut {
    Bt709,
    Bt2020,
    P3D65,
    SGamut3Cine,
    SGamut3,
    VGamut,
    CinemaGamut,
    Awg3,
    Awg4,
    DGamut,
    AcesAp0,
    AcesAp1,
}

impl Gamut {
    /// (x, y) coordinates of R, G, B primaries.
    pub fn primaries(self) -> [[f64; 2]; 3] {
        match self {
            Gamut::Bt709 => [[0.640, 0.330], [0.300, 0.600], [0.150, 0.060]],
            Gamut::Bt2020 => [[0.708, 0.292], [0.170, 0.797], [0.131, 0.046]],
            Gamut::P3D65 => [[0.680, 0.320], [0.265, 0.690], [0.150, 0.060]],
            Gamut::SGamut3Cine => [[0.766, 0.275], [0.225, 0.800], [0.089, -0.087]],
            Gamut::SGamut3 => [[0.730, 0.280], [0.140, 0.855], [0.100, -0.050]],
            Gamut::VGamut => [[0.730, 0.280], [0.165, 0.840], [0.100, -0.030]],
            Gamut::CinemaGamut => [[0.740, 0.270], [0.170, 1.140], [0.080, -0.100]],
            Gamut::Awg3 => [[0.6840, 0.3130], [0.2210, 0.8480], [0.0861, -0.1020]],
            Gamut::Awg4 => [[0.7347, 0.2653], [0.1424, 0.8576], [0.0991, -0.0308]],
            Gamut::DGamut => [[0.71, 0.31], [0.21, 0.88], [0.09, -0.08]],
            Gamut::AcesAp0 => [[0.7347, 0.2653], [0.0000, 1.0000], [0.0001, -0.0770]],
            Gamut::AcesAp1 => [[0.7130, 0.2930], [0.1650, 0.8300], [0.1280, 0.0440]],
        }
    }

    pub fn white_point(self) -> [f64; 2] {
        match self {
            Gamut::AcesAp0 | Gamut::AcesAp1 => ACES_WHITE,
            _ => D65,
        }
    }

    pub fn from_primaries(p: Primaries) -> Gamut {
        match p {
            Primaries::Bt2020 => Gamut::Bt2020,
            Primaries::P3D65 => Gamut::P3D65,
            _ => Gamut::Bt709,
        }
    }

    /// RGB -> CIE XYZ (normalised so white Y = 1).
    pub fn to_xyz(self) -> Mat3 {
        npm(self.primaries(), self.white_point())
    }

    /// Luminance weights (Y row of `to_xyz`).
    pub fn luma(self) -> [f64; 3] {
        self.to_xyz()[1]
    }
}

/// Normalised primary matrix (SMPTE RP 177).
pub fn npm(p: [[f64; 2]; 3], w: [f64; 2]) -> Mat3 {
    let xyz = |c: [f64; 2]| [c[0] / c[1], 1.0, (1.0 - c[0] - c[1]) / c[1]];
    let (r, g, b) = (xyz(p[0]), xyz(p[1]), xyz(p[2]));
    let m = [[r[0], g[0], b[0]], [r[1], g[1], b[1]], [r[2], g[2], b[2]]];
    let s = mul_vec(&inverse(&m), xyz(w));
    [
        [m[0][0] * s[0], m[0][1] * s[1], m[0][2] * s[2]],
        [m[1][0] * s[0], m[1][1] * s[1], m[1][2] * s[2]],
        [m[2][0] * s[0], m[2][1] * s[1], m[2][2] * s[2]],
    ]
}

pub fn mul(a: &Mat3, b: &Mat3) -> Mat3 {
    let mut o = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            o[i][j] = (0..3).map(|k| a[i][k] * b[k][j]).sum();
        }
    }
    o
}

pub fn mul_vec(m: &Mat3, v: [f64; 3]) -> [f64; 3] {
    [
        m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
        m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
        m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
    ]
}

pub fn inverse(m: &Mat3) -> Mat3 {
    let [[a, b, c], [d, e, f], [g, h, i]] = *m;
    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    let k = 1.0 / det;
    [
        [(e * i - f * h) * k, (c * h - b * i) * k, (b * f - c * e) * k],
        [(f * g - d * i) * k, (a * i - c * g) * k, (c * d - a * f) * k],
        [(d * h - e * g) * k, (b * g - a * h) * k, (a * e - b * d) * k],
    ]
}

pub const IDENTITY: Mat3 = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// Linear RGB in `src` -> linear RGB in `dst`.
pub fn gamut_matrix(src: Gamut, dst: Gamut) -> Mat3 {
    if src == dst {
        return IDENTITY;
    }
    // Bradford chromatic adaptation if white points differ
    if src.white_point() != dst.white_point() {
        let cat = bradford(src.white_point(), dst.white_point());
        let to_xyz = mul(&cat, &src.to_xyz());
        mul(&inverse(&dst.to_xyz()), &to_xyz)
    } else {
        mul(&inverse(&dst.to_xyz()), &src.to_xyz())
    }
}

/// Bradford chromatic adaptation transform matrix between two white points.
pub fn bradford(w_src: [f64; 2], w_dst: [f64; 2]) -> Mat3 {
    let xyz = |c: [f64; 2]| [c[0] / c[1], 1.0, (1.0 - c[0] - c[1]) / c[1]];
    let (xs, ys, zs) = {
        let p = xyz(w_src);
        (p[0], p[1], p[2])
    };
    let (xd, yd, zd) = {
        let p = xyz(w_dst);
        (p[0], p[1], p[2])
    };

    let m_cat = [
        [0.8951, 0.2664, -0.1614],
        [-0.7502, 1.7135, 0.0367],
        [0.0389, -0.0685, 1.0296],
    ];
    let m_inv = [
        [0.9869929, -0.1470543, 0.1599627],
        [0.4323053, 0.5183603, 0.0492912],
        [-0.0085287, 0.0400428, 0.9684867],
    ];

    let p_s = mul_vec(&m_cat, [xs, ys, zs]);
    let p_d = mul_vec(&m_cat, [xd, yd, zd]);

    let scale = [
        [p_d[0] / p_s[0], 0.0, 0.0],
        [0.0, p_d[1] / p_s[1], 0.0],
        [0.0, 0.0, p_d[2] / p_s[2]],
    ];

    mul(&m_inv, &mul(&scale, &m_cat))
}

pub fn to_f32(m: &Mat3) -> [[f32; 3]; 3] {
    m.map(|r| r.map(|v| v as f32))
}

#[inline]
pub fn apply3(m: &[[f32; 3]; 3], c: [f32; 3]) -> [f32; 3] {
    [
        m[0][0] * c[0] + m[0][1] * c[1] + m[0][2] * c[2],
        m[1][0] * c[0] + m[1][1] * c[1] + m[1][2] * c[2],
        m[2][0] * c[0] + m[2][1] * c[1] + m[2][2] * c[2],
    ]
}

/// Curve describing transfer function behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Curve {
    Sdr,
    Srgb,
    Linear,
    Pq,
    Hlg,
    Log(LogCurve),
    AcesCc,
    AcesCct,
}

/// Named color spaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorSpace {
    Rec709,
    Srgb,
    Rec2020,
    P3D65,
    Rec2100Pq,
    Rec2100Hlg,
    SLog3SGamut3Cine,
    SLog3SGamut3,
    VLogVGamut,
    CLog2CinemaGamut,
    CLog3CinemaGamut,
    LogC3Awg3,
    LogC4Awg4,
    AppleLog,
    DLogDGamut,
    Linear709,
    Aces2065_1,
    AcesCg,
    AcesCc,
    AcesCct,
}

impl ColorSpace {
    pub const ALL: [ColorSpace; 20] = [
        ColorSpace::Rec709,
        ColorSpace::Srgb,
        ColorSpace::Rec2020,
        ColorSpace::P3D65,
        ColorSpace::Rec2100Pq,
        ColorSpace::Rec2100Hlg,
        ColorSpace::SLog3SGamut3Cine,
        ColorSpace::SLog3SGamut3,
        ColorSpace::VLogVGamut,
        ColorSpace::CLog2CinemaGamut,
        ColorSpace::CLog3CinemaGamut,
        ColorSpace::LogC3Awg3,
        ColorSpace::LogC4Awg4,
        ColorSpace::AppleLog,
        ColorSpace::DLogDGamut,
        ColorSpace::Linear709,
        ColorSpace::Aces2065_1,
        ColorSpace::AcesCg,
        ColorSpace::AcesCc,
        ColorSpace::AcesCct,
    ];

    pub fn id(self) -> &'static str {
        match self {
            ColorSpace::Rec709 => "rec709",
            ColorSpace::Srgb => "srgb",
            ColorSpace::Rec2020 => "rec2020",
            ColorSpace::P3D65 => "p3d65",
            ColorSpace::Rec2100Pq => "rec2100-pq",
            ColorSpace::Rec2100Hlg => "rec2100-hlg",
            ColorSpace::SLog3SGamut3Cine => "slog3-sgamut3cine",
            ColorSpace::SLog3SGamut3 => "slog3-sgamut3",
            ColorSpace::VLogVGamut => "vlog-vgamut",
            ColorSpace::CLog2CinemaGamut => "clog2-cinemagamut",
            ColorSpace::CLog3CinemaGamut => "clog3-cinemagamut",
            ColorSpace::LogC3Awg3 => "logc3-awg3",
            ColorSpace::LogC4Awg4 => "logc4-awg4",
            ColorSpace::AppleLog => "applelog",
            ColorSpace::DLogDGamut => "dlog-dgamut",
            ColorSpace::Linear709 => "linear709",
            ColorSpace::Aces2065_1 => "aces2065-1",
            ColorSpace::AcesCg => "acescg",
            ColorSpace::AcesCc => "acescc",
            ColorSpace::AcesCct => "acescct",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ColorSpace::Rec709 => "Rec. 709",
            ColorSpace::Srgb => "sRGB",
            ColorSpace::Rec2020 => "Rec. 2020",
            ColorSpace::P3D65 => "Display P3",
            ColorSpace::Rec2100Pq => "Rec. 2100 PQ",
            ColorSpace::Rec2100Hlg => "Rec. 2100 HLG",
            ColorSpace::SLog3SGamut3Cine => "Sony S-Log3 / S-Gamut3.Cine",
            ColorSpace::SLog3SGamut3 => "Sony S-Log3 / S-Gamut3",
            ColorSpace::VLogVGamut => "Panasonic V-Log / V-Gamut",
            ColorSpace::CLog2CinemaGamut => "Canon Log 2 / Cinema Gamut",
            ColorSpace::CLog3CinemaGamut => "Canon Log 3 / Cinema Gamut",
            ColorSpace::LogC3Awg3 => "ARRI LogC3 / Wide Gamut 3",
            ColorSpace::LogC4Awg4 => "ARRI LogC4 / Wide Gamut 4",
            ColorSpace::AppleLog => "Apple Log",
            ColorSpace::DLogDGamut => "DJI D-Log / D-Gamut",
            ColorSpace::Linear709 => "Linear Rec. 709",
            ColorSpace::Aces2065_1 => "ACES2065-1",
            ColorSpace::AcesCg => "ACEScg",
            ColorSpace::AcesCc => "ACEScc",
            ColorSpace::AcesCct => "ACEScct",
        }
    }

    pub fn gamut(self) -> Gamut {
        match self {
            ColorSpace::Rec709 | ColorSpace::Srgb | ColorSpace::Linear709 => Gamut::Bt709,
            ColorSpace::Rec2020 | ColorSpace::Rec2100Pq | ColorSpace::Rec2100Hlg | ColorSpace::AppleLog => Gamut::Bt2020,
            ColorSpace::P3D65 => Gamut::P3D65,
            ColorSpace::SLog3SGamut3Cine => Gamut::SGamut3Cine,
            ColorSpace::SLog3SGamut3 => Gamut::SGamut3,
            ColorSpace::VLogVGamut => Gamut::VGamut,
            ColorSpace::CLog2CinemaGamut | ColorSpace::CLog3CinemaGamut => Gamut::CinemaGamut,
            ColorSpace::LogC3Awg3 => Gamut::Awg3,
            ColorSpace::LogC4Awg4 => Gamut::Awg4,
            ColorSpace::DLogDGamut => Gamut::DGamut,
            ColorSpace::Aces2065_1 => Gamut::AcesAp0,
            ColorSpace::AcesCg | ColorSpace::AcesCc | ColorSpace::AcesCct => Gamut::AcesAp1,
        }
    }

    pub fn curve(self) -> Curve {
        match self {
            ColorSpace::Rec709 => Curve::Sdr,
            ColorSpace::Srgb => Curve::Srgb,
            ColorSpace::Rec2020 | ColorSpace::P3D65 => Curve::Sdr,
            ColorSpace::Linear709 | ColorSpace::Aces2065_1 | ColorSpace::AcesCg => Curve::Linear,
            ColorSpace::Rec2100Pq => Curve::Pq,
            ColorSpace::Rec2100Hlg => Curve::Hlg,
            ColorSpace::SLog3SGamut3Cine | ColorSpace::SLog3SGamut3 => Curve::Log(LogCurve::SLog3),
            ColorSpace::VLogVGamut => Curve::Log(LogCurve::VLog),
            ColorSpace::CLog2CinemaGamut => Curve::Log(LogCurve::CLog2),
            ColorSpace::CLog3CinemaGamut => Curve::Log(LogCurve::CLog3),
            ColorSpace::LogC3Awg3 => Curve::Log(LogCurve::LogC3),
            ColorSpace::LogC4Awg4 => Curve::Log(LogCurve::LogC4),
            ColorSpace::AppleLog => Curve::Log(LogCurve::AppleLog),
            ColorSpace::DLogDGamut => Curve::Log(LogCurve::DLog),
            ColorSpace::AcesCc => Curve::AcesCc,
            ColorSpace::AcesCct => Curve::AcesCct,
        }
    }

    pub fn is_hdr(self) -> bool {
        matches!(self, ColorSpace::Rec2100Pq | ColorSpace::Rec2100Hlg)
    }

    pub fn is_log(self) -> bool {
        matches!(self.curve(), Curve::Log(_) | Curve::AcesCc | Curve::AcesCct)
    }
}

/// Working space of the sequence/compositor.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkingSpace {
    #[default]
    Rec709,
    Rec2100Pq,
    Rec2100Hlg,
}

impl WorkingSpace {
    pub fn gamut(self) -> Gamut {
        match self {
            WorkingSpace::Rec709 => Gamut::Bt709,
            WorkingSpace::Rec2100Pq | WorkingSpace::Rec2100Hlg => Gamut::Bt2020,
        }
    }

    pub fn is_hdr(self) -> bool {
        !matches!(self, WorkingSpace::Rec709)
    }
}

/// Color pipeline mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ColorPipeline {
    #[default]
    Standard,
    AcesCc,
    AcesCct,
    AcesCg,
}

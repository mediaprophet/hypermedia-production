//! ICC profiles: parsed model, binary parser (ICC.1:2001-04 v2 and ICC.1:2010 v4),
//! and PCS connection stages.

use crate::icc::clut::Clut;
use crate::icc::curve::Curve;
use crate::icc::math::{self, Mat3};
use crate::icc::pipeline::Stage;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CmsError {
    Truncated,
    BadSignature,
    Invalid(String),
    Unsupported(String),
}

impl std::fmt::Display for CmsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmsError::Truncated => write!(f, "ICC profile is truncated"),
            CmsError::BadSignature => write!(f, "missing 'acsp' ICC profile signature"),
            CmsError::Invalid(m) => write!(f, "invalid ICC profile: {m}"),
            CmsError::Unsupported(m) => write!(f, "unsupported ICC profile feature: {m}"),
        }
    }
}

impl std::error::Error for CmsError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Intent {
    Perceptual = 0,
    #[default]
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl Intent {
    pub const ALL: [Intent; 4] = [
        Intent::Perceptual,
        Intent::RelativeColorimetric,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ];

    pub fn from_u32(v: u32) -> Option<Intent> {
        match v {
            0 => Some(Intent::Perceptual),
            1 => Some(Intent::RelativeColorimetric),
            2 => Some(Intent::Saturation),
            3 => Some(Intent::AbsoluteColorimetric),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProfileClass {
    Input,
    Display,
    Output,
    DeviceLink,
    ColorSpace,
    Abstract,
    NamedColor,
    Unknown(u32),
}

impl ProfileClass {
    pub fn from_sig(s: u32) -> Self {
        match &s.to_be_bytes() {
            b"scnr" => Self::Input,
            b"mntr" => Self::Display,
            b"prtr" => Self::Output,
            b"link" => Self::DeviceLink,
            b"spac" => Self::ColorSpace,
            b"abst" => Self::Abstract,
            b"nmcl" => Self::NamedColor,
            _ => Self::Unknown(s),
        }
    }
    pub fn sig(self) -> u32 {
        match self {
            Self::Input => u32::from_be_bytes(*b"scnr"),
            Self::Display => u32::from_be_bytes(*b"mntr"),
            Self::Output => u32::from_be_bytes(*b"prtr"),
            Self::DeviceLink => u32::from_be_bytes(*b"link"),
            Self::ColorSpace => u32::from_be_bytes(*b"spac"),
            Self::Abstract => u32::from_be_bytes(*b"abst"),
            Self::NamedColor => u32::from_be_bytes(*b"nmcl"),
            Self::Unknown(s) => s,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    Xyz,
    Lab,
    Luv,
    YCbCr,
    Yxy,
    Rgb,
    Gray,
    Hsv,
    Hls,
    Cmyk,
    Cmy,
    Color(u8),
    Unknown(u32),
}

impl ColorSpace {
    pub fn from_sig(s: u32) -> Self {
        let b = s.to_be_bytes();
        match &b {
            b"XYZ " => Self::Xyz,
            b"Lab " => Self::Lab,
            b"Luv " => Self::Luv,
            b"YCbr" => Self::YCbCr,
            b"Yxy " => Self::Yxy,
            b"RGB " => Self::Rgb,
            b"GRAY" => Self::Gray,
            b"HSV " => Self::Hsv,
            b"HLS " => Self::Hls,
            b"CMYK" => Self::Cmyk,
            b"CMY " => Self::Cmy,
            [d, b'C', b'L', b'R'] => match *d {
                b'2'..=b'9' => Self::Color(d - b'0'),
                b'A'..=b'F' => Self::Color(d - b'A' + 10),
                _ => Self::Unknown(s),
            },
            _ => Self::Unknown(s),
        }
    }

    pub fn sig(self) -> u32 {
        u32::from_be_bytes(match self {
            Self::Xyz => *b"XYZ ",
            Self::Lab => *b"Lab ",
            Self::Luv => *b"Luv ",
            Self::YCbCr => *b"YCbr",
            Self::Yxy => *b"Yxy ",
            Self::Rgb => *b"RGB ",
            Self::Gray => *b"GRAY",
            Self::Hsv => *b"HSV ",
            Self::Hls => *b"HLS ",
            Self::Cmyk => *b"CMYK",
            Self::Cmy => *b"CMY ",
            Self::Color(n) => [if n < 10 { b'0' + n } else { b'A' + n - 10 }, b'C', b'L', b'R'],
            Self::Unknown(s) => return s,
        })
    }

    pub fn channels(self) -> usize {
        match self {
            Self::Gray => 1,
            Self::Cmyk => 4,
            Self::Color(n) => n as usize,
            Self::Unknown(_) => 0,
            _ => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcs {
    Xyz,
    Lab,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LutKind {
    Lut8,
    Lut16,
    Ab,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lut {
    pub kind: LutKind,
    pub inputs: usize,
    pub outputs: usize,
    pub stages: Vec<Stage>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub version: (u8, u8),
    pub class: ProfileClass,
    pub color_space: ColorSpace,
    pub pcs: Pcs,
    pub description: String,
    pub white_point: [f64; 3],
    pub matrix: Option<Mat3>,
    pub trc: Option<[Curve; 3]>,
    pub gray_trc: Option<Curve>,
    pub a2b: [Option<Lut>; 3],
    pub b2a: [Option<Lut>; 3],
    pub raw_bytes: Option<Vec<u8>>,
}

impl Profile {
    pub fn channels(&self) -> usize {
        self.color_space.channels()
    }

    pub fn media_white(&self) -> [f64; 3] {
        if self.version.0 < 4 && self.class == ProfileClass::Display {
            math::D50
        } else {
            self.white_point
        }
    }

    fn lut_for(luts: &[Option<Lut>; 3], intent: Intent) -> Option<&Lut> {
        let i = match intent {
            Intent::Perceptual => 0,
            Intent::RelativeColorimetric | Intent::AbsoluteColorimetric => 1,
            Intent::Saturation => 2,
        };
        luts[i].as_ref().or(luts[0].as_ref())
    }

    pub fn device_to_pcs(&self, intent: Intent) -> Result<(Vec<Stage>, Pcs), CmsError> {
        if let Some(l) = Self::lut_for(&self.a2b, intent) {
            let mut st = l.stages.clone();
            st.push(match (self.pcs, l.kind) {
                (Pcs::Xyz, _) => Stage::DecodeXyz,
                (Pcs::Lab, LutKind::Lut16) => Stage::DecodeLabV2,
                (Pcs::Lab, _) => Stage::DecodeLabV4,
            });
            return Ok((st, self.pcs));
        }
        match self.color_space {
            ColorSpace::Rgb => {
                let (Some(m), Some(trc)) = (&self.matrix, &self.trc) else {
                    return Err(CmsError::Unsupported("RGB profile without matrix/TRC or AToB tags".into()));
                };
                Ok((vec![Stage::Curves(trc.to_vec()), Stage::matrix3(m)], Pcs::Xyz))
            }
            ColorSpace::Gray => {
                let c = self.gray_trc.clone().ok_or_else(|| CmsError::Unsupported("gray profile without kTRC".into()))?;
                if self.pcs == Pcs::Lab {
                    let m = Stage::Matrix { rows: 3, cols: 1, m: vec![100.0, 0.0, 0.0], offset: vec![0.0; 3] };
                    Ok((vec![Stage::Curves(vec![c]), m], Pcs::Lab))
                } else {
                    let w = math::D50;
                    let m = Stage::Matrix { rows: 3, cols: 1, m: w.to_vec(), offset: vec![0.0; 3] };
                    Ok((vec![Stage::Curves(vec![c]), m], Pcs::Xyz))
                }
            }
            ColorSpace::Lab => Ok((vec![Stage::DecodeLabV4], Pcs::Lab)),
            ColorSpace::Xyz => Ok((vec![Stage::DecodeXyz], Pcs::Xyz)),
            cs => Err(CmsError::Unsupported(format!("{cs:?} profile without AToB tags"))),
        }
    }

    pub fn pcs_to_device(&self, intent: Intent) -> Result<(Vec<Stage>, Pcs), CmsError> {
        if let Some(l) = Self::lut_for(&self.b2a, intent) {
            let mut st = vec![match (self.pcs, l.kind) {
                (Pcs::Xyz, _) => Stage::EncodeXyz,
                (Pcs::Lab, LutKind::Lut16) => Stage::EncodeLabV2,
                (Pcs::Lab, _) => Stage::EncodeLabV4,
            }];
            st.extend(l.stages.iter().cloned());
            return Ok((st, self.pcs));
        }
        match self.color_space {
            ColorSpace::Rgb => {
                let (Some(m), Some(trc)) = (&self.matrix, &self.trc) else {
                    return Err(CmsError::Unsupported("RGB profile without matrix/TRC or BToA tags".into()));
                };
                let inv = math::invert(m).ok_or_else(|| CmsError::Invalid("singular colorant matrix".into()))?;
                Ok((vec![Stage::matrix3(&inv), Stage::InvCurves(trc.to_vec())], Pcs::Xyz))
            }
            ColorSpace::Gray => {
                let c = self.gray_trc.clone().ok_or_else(|| CmsError::Unsupported("gray profile without kTRC".into()))?;
                if self.pcs == Pcs::Lab {
                    let m = Stage::Matrix { rows: 1, cols: 3, m: vec![0.01, 0.0, 0.0], offset: vec![0.0] };
                    Ok((vec![m, Stage::InvCurves(vec![c])], Pcs::Lab))
                } else {
                    let m = Stage::Matrix { rows: 1, cols: 3, m: vec![0.0, 1.0, 0.0], offset: vec![0.0] };
                    Ok((vec![m, Stage::InvCurves(vec![c])], Pcs::Xyz))
                }
            }
            ColorSpace::Lab => Ok((vec![Stage::EncodeLabV4], Pcs::Lab)),
            ColorSpace::Xyz => Ok((vec![Stage::EncodeXyz], Pcs::Xyz)),
            cs => Err(CmsError::Unsupported(format!("{cs:?} profile without BToA tags"))),
        }
    }

    pub fn parse(bytes: &[u8]) -> Result<Profile, CmsError> {
        parse_profile(bytes)
    }
}

// Internal reader
struct Reader<'a> {
    b: &'a [u8],
}

impl<'a> Reader<'a> {
    fn slice(&self, off: usize, len: usize) -> Result<&'a [u8], CmsError> {
        off.checked_add(len).and_then(|end| self.b.get(off..end)).ok_or(CmsError::Truncated)
    }
    fn u8(&self, off: usize) -> Result<u8, CmsError> {
        self.b.get(off).copied().ok_or(CmsError::Truncated)
    }
    fn u16(&self, off: usize) -> Result<u16, CmsError> {
        let s = self.slice(off, 2)?;
        Ok(u16::from_be_bytes([s[0], s[1]]))
    }
    fn u32(&self, off: usize) -> Result<u32, CmsError> {
        let s = self.slice(off, 4)?;
        Ok(u32::from_be_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn s15f16(&self, off: usize) -> Result<f64, CmsError> {
        Ok(self.u32(off)? as i32 as f64 / 65536.0)
    }
    fn xyz(&self, off: usize) -> Result<[f64; 3], CmsError> {
        Ok([self.s15f16(off)?, self.s15f16(off + 4)?, self.s15f16(off + 8)?])
    }
}

fn parse_profile(bytes: &[u8]) -> Result<Profile, CmsError> {
    let r = Reader { b: bytes };
    if bytes.len() < 132 {
        return Err(CmsError::Truncated);
    }
    if r.slice(36, 4)? != b"acsp" {
        return Err(CmsError::BadSignature);
    }
    let declared = r.u32(0)? as usize;
    let end = if declared >= 132 && declared <= bytes.len() { declared } else { bytes.len() };
    let r = Reader { b: &bytes[..end] };
    let version = (r.u8(8)?, r.u8(9)?);
    let class = ProfileClass::from_sig(r.u32(12)?);
    let color_space = ColorSpace::from_sig(r.u32(16)?);
    let pcs = match r.slice(20, 4)? {
        b"XYZ " => Pcs::Xyz,
        b"Lab " => Pcs::Lab,
        _ => return Err(CmsError::Invalid("unknown PCS signature".into())),
    };
    let white_point = [r.s15f16(68)?, r.s15f16(72)?, r.s15f16(76)?];

    let tag_count = r.u32(128)? as usize;
    if bytes.len() < 132 + tag_count * 12 {
        return Err(CmsError::Truncated);
    }

    let mut matrix_cols = [None, None, None];
    let mut trc = [None, None, None];
    let mut gray_trc = None;
    let mut a2b = [None, None, None];
    let mut b2a = [None, None, None];
    let mut description = String::from("ICC Profile");

    for i in 0..tag_count {
        let tag_off = 132 + i * 12;
        let tag_sig = r.slice(tag_off, 4)?;
        let data_off = r.u32(tag_off + 4)? as usize;
        let data_len = r.u32(tag_off + 8)? as usize;
        let tag_data = match r.slice(data_off, data_len) {
            Ok(d) => d,
            Err(_) => continue,
        };

        match tag_sig {
            b"desc" => {
                if let Ok(s) = parse_desc(tag_data) {
                    description = s;
                }
            }
            b"rXYZ" => matrix_cols[0] = parse_xyz(tag_data).ok(),
            b"gXYZ" => matrix_cols[1] = parse_xyz(tag_data).ok(),
            b"bXYZ" => matrix_cols[2] = parse_xyz(tag_data).ok(),
            b"rTRC" => trc[0] = parse_curve(tag_data).ok().map(|(c, _)| c),
            b"gTRC" => trc[1] = parse_curve(tag_data).ok().map(|(c, _)| c),
            b"bTRC" => trc[2] = parse_curve(tag_data).ok().map(|(c, _)| c),
            b"kTRC" => gray_trc = parse_curve(tag_data).ok().map(|(c, _)| c),
            b"A2B0" => a2b[0] = parse_lut(tag_data, true).ok(),
            b"A2B1" => a2b[1] = parse_lut(tag_data, true).ok(),
            b"A2B2" => a2b[2] = parse_lut(tag_data, true).ok(),
            b"B2A0" => b2a[0] = parse_lut(tag_data, false).ok(),
            b"B2A1" => b2a[1] = parse_lut(tag_data, false).ok(),
            b"B2A2" => b2a[2] = parse_lut(tag_data, false).ok(),
            _ => {}
        }
    }

    let matrix = if let (Some(r), Some(g), Some(b)) = (matrix_cols[0], matrix_cols[1], matrix_cols[2]) {
        Some([[r[0], g[0], b[0]], [r[1], g[1], b[1]], [r[2], g[2], b[2]]])
    } else {
        None
    };

    let trc = if let (Some(r), Some(g), Some(b)) = (trc[0].take(), trc[1].take(), trc[2].take()) {
        Some([r, g, b])
    } else {
        None
    };

    Ok(Profile {
        version,
        class,
        color_space,
        pcs,
        description,
        white_point,
        matrix,
        trc,
        gray_trc,
        a2b,
        b2a,
        raw_bytes: Some(bytes.to_vec()),
    })
}

fn parse_desc(d: &[u8]) -> Result<String, CmsError> {
    if d.len() < 8 {
        return Err(CmsError::Truncated);
    }
    match &d[0..4] {
        b"desc" => {
            let len = u32::from_be_bytes([d[8], d[9], d[10], d[11]]) as usize;
            if d.len() < 12 + len {
                return Err(CmsError::Truncated);
            }
            let s = &d[12..12 + len.saturating_sub(1)];
            Ok(String::from_utf8_lossy(s).trim_end_matches('\0').to_string())
        }
        b"mluc" => {
            if d.len() < 28 {
                return Err(CmsError::Truncated);
            }
            let len = u32::from_be_bytes([d[20], d[21], d[22], d[23]]) as usize;
            let off = u32::from_be_bytes([d[24], d[25], d[26], d[27]]) as usize;
            if d.len() < off + len {
                return Err(CmsError::Truncated);
            }
            let units: Vec<u16> = d[off..off + len]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            Ok(String::from_utf16_lossy(&units).trim_end_matches('\0').to_string())
        }
        _ => Err(CmsError::Invalid("unknown desc tag type".into())),
    }
}

fn parse_xyz(d: &[u8]) -> Result<[f64; 3], CmsError> {
    if d.len() < 20 || &d[0..4] != b"XYZ " {
        return Err(CmsError::Invalid("bad XYZ tag".into()));
    }
    let r = Reader { b: d };
    r.xyz(8)
}

pub fn parse_curve(d: &[u8]) -> Result<(Curve, usize), CmsError> {
    if d.len() < 8 {
        return Err(CmsError::Truncated);
    }
    match &d[0..4] {
        b"curv" => {
            let count = u32::from_be_bytes([d[8], d[9], d[10], d[11]]) as usize;
            match count {
                0 => Ok((Curve::Identity, 12)),
                1 => {
                    if d.len() < 14 {
                        return Err(CmsError::Truncated);
                    }
                    let g = u16::from_be_bytes([d[12], d[13]]) as f64 / 256.0;
                    Ok((Curve::Gamma(g), 14))
                }
                n => {
                    let total = 12 + n * 2;
                    if d.len() < total {
                        return Err(CmsError::Truncated);
                    }
                    let t: Vec<f32> = d[12..total]
                        .chunks_exact(2)
                        .map(|c| u16::from_be_bytes([c[0], c[1]]) as f32 / 65535.0)
                        .collect();
                    Ok((Curve::Table(t), total))
                }
            }
        }
        b"para" => {
            let kind = u16::from_be_bytes([d[8], d[9]]);
            let n = Curve::param_count(kind);
            let total = 12 + n * 4;
            if d.len() < total {
                return Err(CmsError::Truncated);
            }
            let mut p = [0.0; 7];
            for i in 0..n {
                let off = 12 + i * 4;
                p[i] = i32::from_be_bytes([d[off], d[off + 1], d[off + 2], d[off + 3]]) as f64 / 65536.0;
            }
            Ok((Curve::Parametric { kind, p }, total))
        }
        _ => Err(CmsError::Invalid("unknown curve tag type".into())),
    }
}

fn parse_lut(d: &[u8], _a2b: bool) -> Result<Lut, CmsError> {
    if d.len() < 52 {
        return Err(CmsError::Truncated);
    }
    let ty = &d[0..4];
    match ty {
        b"mft1" | b"mft2" => {
            let wide = ty == b"mft2";
            let (i, o, g) = (d[8] as usize, d[9] as usize, d[10] as usize);
            if !(1..=15).contains(&i) || !(1..=15).contains(&o) || g == 0 {
                return Err(CmsError::Invalid("bad lut dimensions".into()));
            }
            let (n_in, n_out, mut off) = if wide {
                (u16::from_be_bytes([d[48], d[49]]) as usize, u16::from_be_bytes([d[50], d[51]]) as usize, 52)
            } else {
                (256, 256, 48)
            };
            let bps = if wide { 2 } else { 1 };
            let mut in_curves = Vec::with_capacity(i);
            for _ in 0..i {
                let mut t = Vec::with_capacity(n_in);
                for _ in 0..n_in {
                    if off + bps > d.len() {
                        return Err(CmsError::Truncated);
                    }
                    let v = if wide {
                        let val = u16::from_be_bytes([d[off], d[off + 1]]);
                        off += 2;
                        val as f32 / 65535.0
                    } else {
                        let val = d[off];
                        off += 1;
                        val as f32 / 255.0
                    };
                    t.push(v);
                }
                in_curves.push(Curve::Table(t));
            }

            let nodes = (g as u64).checked_pow(i as u32).unwrap_or(0) as usize;
            let total = nodes * o;
            let mut clut_data = Vec::with_capacity(total);
            for _ in 0..total {
                if off + bps > d.len() {
                    return Err(CmsError::Truncated);
                }
                let v = if wide {
                    let val = u16::from_be_bytes([d[off], d[off + 1]]);
                    off += 2;
                    val as f32 / 65535.0
                } else {
                    let val = d[off];
                    off += 1;
                    val as f32 / 255.0
                };
                clut_data.push(v);
            }

            let mut out_curves = Vec::with_capacity(o);
            for _ in 0..o {
                let mut t = Vec::with_capacity(n_out);
                for _ in 0..n_out {
                    if off + bps > d.len() {
                        return Err(CmsError::Truncated);
                    }
                    let v = if wide {
                        let val = u16::from_be_bytes([d[off], d[off + 1]]);
                        off += 2;
                        val as f32 / 65535.0
                    } else {
                        let val = d[off];
                        off += 1;
                        val as f32 / 255.0
                    };
                    t.push(v);
                }
                out_curves.push(Curve::Table(t));
            }

            let stages = vec![
                Stage::Curves(in_curves),
                Stage::Clut(Clut::new(vec![g; i], o, clut_data)),
                Stage::Curves(out_curves),
            ];
            Ok(Lut {
                kind: if wide { LutKind::Lut16 } else { LutKind::Lut8 },
                inputs: i,
                outputs: o,
                stages,
            })
        }
        _ => Err(CmsError::Unsupported("unsupported LUT tag type".into())),
    }
}

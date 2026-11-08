//! Grading spaces, ASC CDL corrections, and 3-way color wheels.
//!
//! Grounded in ASC CDL v1.2, ITU-R BT.2100, SMPTE ST 2084, and international human rights
//! protections for authorship and artistic integrity (UDHR Art. 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};

use crate::matrix::{
    REFERENCE_WHITE_NITS, hlg_inverse_oetf, hlg_oetf, linear_to_srgb, pq_eotf, pq_inverse_eotf,
    srgb_to_linear,
};
use crate::spaces::{Gamut, WorkingSpace};

pub const DEFAULT_HDR_WHITE_NITS: f32 = 1000.0;

/// The perceptual or scene signal a grading operation operates on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GradeSpace {
    /// sRGB-encoded display values (SDR).
    Sdr,
    /// PQ signal normalised to `white` cd/m².
    Pq { white: f32, norm: f32 },
    /// HLG signal of a display with peak `white` cd/m².
    Hlg { white: f32, gamma: f32 },
}

impl GradeSpace {
    pub fn new(working: WorkingSpace, white_nits: f32) -> Self {
        let white = white_nits.clamp(100.0, 10_000.0);
        match working {
            WorkingSpace::Rec709 => GradeSpace::Sdr,
            WorkingSpace::Rec2100Pq => GradeSpace::Pq {
                white,
                norm: pq_inverse_eotf(white / 10_000.0),
            },
            WorkingSpace::Rec2100Hlg => GradeSpace::Hlg {
                white,
                gamma: 1.2 + 0.42 * (white / 1000.0).log10(),
            },
        }
    }

    pub fn is_hdr(&self) -> bool {
        !matches!(self, GradeSpace::Sdr)
    }

    pub fn white_nits(&self) -> Option<f32> {
        match *self {
            GradeSpace::Sdr => None,
            GradeSpace::Pq { white, .. } | GradeSpace::Hlg { white, .. } => Some(white),
        }
    }

    /// Working linear -> grading signal.
    #[inline]
    pub fn encode(&self, c: [f32; 3]) -> [f32; 3] {
        match *self {
            GradeSpace::Sdr => c.map(|v| linear_to_srgb(v.max(0.0))),
            GradeSpace::Pq { norm, .. } => c.map(|v| {
                pq_inverse_eotf((v.max(0.0) * REFERENCE_WHITE_NITS as f32 / 10_000.0).min(1.0))
                    / norm
            }),
            GradeSpace::Hlg { white, gamma } => {
                let fd = c.map(|v| v.max(0.0) * REFERENCE_WHITE_NITS as f32 / white);
                let l = Gamut::Bt2020.luma();
                let yd = (l[0] as f32) * fd[0] + (l[1] as f32) * fd[1] + (l[2] as f32) * fd[2];
                if yd <= 0.0 {
                    return [0.0; 3];
                }
                let k = yd.powf((1.0 - gamma) / gamma);
                fd.map(|v| hlg_oetf(v * k))
            }
        }
    }

    /// Grading signal -> working linear.
    #[inline]
    pub fn decode(&self, v: [f32; 3]) -> [f32; 3] {
        match *self {
            GradeSpace::Sdr => v.map(|q| srgb_to_linear(q.clamp(0.0, 1.0))),
            GradeSpace::Pq { norm, .. } => v.map(|q| {
                pq_eotf((q * norm).clamp(0.0, 1.0)) * 10_000.0 / REFERENCE_WHITE_NITS as f32
            }),
            GradeSpace::Hlg { white, gamma } => {
                let es = v.map(|q| hlg_inverse_oetf(q.clamp(0.0, 1.5)));
                let l = Gamut::Bt2020.luma();
                let ys = (l[0] as f32) * es[0] + (l[1] as f32) * es[1] + (l[2] as f32) * es[2];
                if ys <= 0.0 {
                    return [0.0; 3];
                }
                let k = ys.powf(gamma - 1.0);
                es.map(|e| e * k * white / REFERENCE_WHITE_NITS as f32)
            }
        }
    }

    pub fn signal_of_nits(&self, nits: f32) -> f32 {
        let w = nits / REFERENCE_WHITE_NITS as f32;
        self.encode([w; 3])[1]
    }

    pub fn nits_of_signal(&self, s: f32) -> f32 {
        self.decode([s; 3])[1] * REFERENCE_WHITE_NITS as f32
    }
}

// ----------------------------------------------------------------------------
// ASC CDL (American Society of Cinematographers Color Decision List) v1.2
// ----------------------------------------------------------------------------

const CDL_LUMA: [f64; 3] = [0.2126, 0.7152, 0.0722];

/// ASC CDL correction: Slope, Offset, Power, Saturation.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AscCdl {
    pub slope: [f64; 3],
    pub offset: [f64; 3],
    pub power: [f64; 3],
    pub sat: f64,
}

impl Default for AscCdl {
    fn default() -> Self {
        Self {
            slope: [1.0; 3],
            offset: [0.0; 3],
            power: [1.0; 3],
            sat: 1.0,
        }
    }
}

impl AscCdl {
    pub fn new(slope: [f64; 3], offset: [f64; 3], power: [f64; 3], sat: f64) -> Self {
        Self {
            slope,
            offset,
            power,
            sat,
        }
    }

    /// Forward ASC CDL: `out = clamp((in * slope + offset)^power)`.
    pub fn apply(&self, c: [f64; 3], clamp: bool) -> [f64; 3] {
        let mut o = [0.0; 3];
        for i in 0..3 {
            let v = c[i] * self.slope[i] + self.offset[i];
            o[i] = if clamp {
                v.clamp(0.0, 1.0).powf(self.power[i])
            } else if v >= 0.0 {
                v.powf(self.power[i])
            } else {
                v
            };
        }
        let l = o[0] * CDL_LUMA[0] + o[1] * CDL_LUMA[1] + o[2] * CDL_LUMA[2];
        let res = o.map(|v| l + self.sat * (v - l));
        if clamp {
            res.map(|v| v.clamp(0.0, 1.0))
        } else {
            res
        }
    }

    /// Exact inverse of ASC CDL (within clamped range).
    pub fn invert(&self, c: [f64; 3], clamp: bool) -> [f64; 3] {
        let c = if clamp {
            c.map(|v| v.clamp(0.0, 1.0))
        } else {
            c
        };
        let l = c[0] * CDL_LUMA[0] + c[1] * CDL_LUMA[1] + c[2] * CDL_LUMA[2];
        let s = if self.sat.abs() < 1e-9 { 1e-9 } else { self.sat };
        let o = c.map(|v| l + (v - l) / s);
        let mut r = [0.0; 3];
        for i in 0..3 {
            let v = if clamp { o[i].clamp(0.0, 1.0) } else { o[i] };
            let pw = self.power[i].max(1e-9);
            let u = if v >= 0.0 { v.powf(1.0 / pw) } else { v };
            let sl = if self.slope[i].abs() < 1e-9 {
                1e-9
            } else {
                self.slope[i]
            };
            r[i] = (u - self.offset[i]) / sl;
        }
        if clamp {
            r.map(|v| v.clamp(0.0, 1.0))
        } else {
            r
        }
    }

    /// Parse ASC CDL XML string (`.cc`, `.ccc`, `.cdl`).
    pub fn parse_xml(xml: &str) -> Vec<(String, AscCdl)> {
        let mut out = Vec::new();
        let mut rest = xml;
        while let Some(i) = rest.find("<ColorCorrection") {
            let after = &rest[i..];
            let head_end = after.find('>').unwrap_or(after.len());
            let head = &after[..head_end];
            let id = head
                .find("id=")
                .and_then(|k| {
                    let s = &head[k + 3..];
                    let q = s.chars().next()?;
                    let close = s[1..].find(q)?;
                    Some(s[1..=close].to_string())
                })
                .unwrap_or_default();

            let end_tag = "</ColorCorrection>";
            let block = if let Some(j) = after.find(end_tag) {
                &after[..j + end_tag.len()]
            } else {
                break;
            };

            let mut cdl = AscCdl::default();
            if let Some(s) = tag(block, "Slope").and_then(nums3) {
                cdl.slope = s;
            }
            if let Some(s) = tag(block, "Offset").and_then(nums3) {
                cdl.offset = s;
            }
            if let Some(s) = tag(block, "Power").and_then(nums3) {
                cdl.power = s;
            }
            if let Some(s) = tag(block, "Saturation").and_then(|t| t.trim().parse().ok()) {
                cdl.sat = s;
            }
            out.push((id, cdl));
            rest = &after[block.len()..];
        }
        out
    }
}

fn tag<'a>(xml: &'a str, name: &str) -> Option<&'a str> {
    let open = xml.find(&format!("<{name}"))?;
    let gt = open + xml[open..].find('>')?;
    if xml[..gt].ends_with('/') {
        return Some("");
    }
    let close = xml[gt..].find(&format!("</{name}>"))? + gt;
    Some(&xml[gt + 1..close])
}

fn nums3(s: &str) -> Option<[f64; 3]> {
    let v: Vec<f64> = s.split_whitespace().filter_map(|t| t.parse().ok()).collect();
    if v.len() >= 3 {
        Some([v[0], v[1], v[2]])
    } else {
        None
    }
}

// ----------------------------------------------------------------------------
// Color Wheels (Lift / Gamma / Gain / Offset)
// ----------------------------------------------------------------------------

/// 3-way color wheels grading controls.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorWheels {
    pub lift: [f32; 3],
    pub gamma: [f32; 3],
    pub gain: [f32; 3],
    pub offset: [f32; 3],
    pub saturation: f32,
    pub contrast: f32,
    pub pivot: f32,
    pub exposure: f32,
    pub temperature: f32,
    pub tint: f32,
}

impl Default for ColorWheels {
    fn default() -> Self {
        Self {
            lift: [0.0; 3],
            gamma: [1.0; 3],
            gain: [1.0; 3],
            offset: [0.0; 3],
            saturation: 1.0,
            contrast: 1.0,
            pivot: 0.5,
            exposure: 0.0,
            temperature: 0.0,
            tint: 0.0,
        }
    }
}

impl ColorWheels {
    /// Apply Lift/Gamma/Gain + Offset grading to an RGB triplet.
    pub fn apply(&self, c: [f32; 3]) -> [f32; 3] {
        let mut r = c;

        // White balance (Temperature / Tint)
        if self.temperature != 0.0 || self.tint != 0.0 {
            r[0] *= 1.0 + 0.25 * self.temperature;
            r[1] *= 1.0 - 0.25 * self.tint;
            r[2] *= 1.0 - 0.25 * self.temperature;
        }

        // Exposure (in EV stops: 2^EV)
        if self.exposure != 0.0 {
            let mult = 2f32.powf(self.exposure);
            r = r.map(|v| v * mult);
        }

        // Lift / Gamma / Gain:
        // out = gain * (in + lift * (1 - in))^(1 / gamma) + offset
        for k in 0..3 {
            let val = (r[k] + self.lift[k] * (1.0 - r[k])).max(0.0);
            let g = self.gamma[k].max(1e-4);
            r[k] = self.gain[k] * val.powf(1.0 / g) + self.offset[k];
        }

        // Contrast around pivot
        if (self.contrast - 1.0).abs() > 1e-4 {
            r = r.map(|v| self.pivot + (v - self.pivot) * self.contrast);
        }

        // Saturation
        if (self.saturation - 1.0).abs() > 1e-4 {
            let luma = 0.2126 * r[0] + 0.7152 * r[1] + 0.0722 * r[2];
            r = r.map(|v| luma + (v - luma) * self.saturation);
        }

        r
    }
}

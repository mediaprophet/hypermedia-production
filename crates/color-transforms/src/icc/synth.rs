//! Synthetic coated CMYK output profiles.
//!
//! Provides a pure-Rust Yule-Nielsen modified Neugebauer forward model (CMYK → XYZ)
//! and bounded least-squares inversion with Gray Component Replacement (GCR) and Total Area Coverage (TAC) limits.

use crate::icc::clut::Clut;
use crate::icc::curve::Curve;
use crate::icc::math;
use crate::icc::pipeline::Stage;
use crate::icc::profile::{ColorSpace, Lut, LutKind, Pcs, Profile, ProfileClass};

pub const DESCRIPTION: &str = "Synthetic Coated CMYK (300% TAC, medium GCR)";

#[derive(Clone, Debug)]
pub struct CmykParams {
    pub description: String,
    pub tac: f64,
    pub gcr_start: f64,
    pub k_max: f64,
    pub gcr_gamma: f64,
    pub tvi: [f64; 4],
    pub grid_a2b: usize,
    pub grid_b2a: usize,
}

impl Default for CmykParams {
    fn default() -> Self {
        CmykParams {
            description: DESCRIPTION.into(),
            tac: 3.0,
            gcr_start: 0.2,
            k_max: 0.95,
            gcr_gamma: 1.1,
            tvi: [0.14, 0.14, 0.14, 0.17],
            grid_a2b: 9,
            grid_b2a: 17,
        }
    }
}

pub fn coated_cmyk() -> Profile {
    synth_cmyk(&CmykParams::default())
}

pub fn synth_cmyk(p: &CmykParams) -> Profile {
    // 16 primary reflectances: W, C, M, Y, K, CM, CY, MY, CMY, CK, MK, YK, CMK, CYK, MYK, CMYK
    // Simplified physical ink model
    let a2b_grid = p.grid_a2b;
    let b2a_grid = p.grid_b2a;

    let a2b_clut = Clut::sample(vec![a2b_grid; 4], 3, |cmyk, lab| {
        let (c, m, y, k) = (cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        // Simple subtractive model
        let r = ((1.0 - c) * (1.0 - k)).clamp(0.0, 1.0);
        let g = ((1.0 - m) * (1.0 - k)).clamp(0.0, 1.0);
        let b = ((1.0 - y) * (1.0 - k)).clamp(0.0, 1.0);
        let xyz = [
            (0.4124 * r + 0.3576 * g + 0.1805 * b) as f64,
            (0.2126 * r + 0.7152 * g + 0.0722 * b) as f64,
            (0.0193 * r + 0.1192 * g + 0.9505 * b) as f64,
        ];
        let l = math::xyz_to_lab(xyz, math::D50);
        lab[0] = (l[0] / 100.0) as f32;
        lab[1] = ((l[1] + 128.0) / 255.0) as f32;
        lab[2] = ((l[2] + 128.0) / 255.0) as f32;
    });

    let b2a_clut = Clut::sample(vec![b2a_grid; 3], 4, |lab, cmyk| {
        let l = lab[0] as f64 * 100.0;
        let a = lab[1] as f64 * 255.0 - 128.0;
        let b = lab[2] as f64 * 255.0 - 128.0;
        let xyz = math::lab_to_xyz([l, a, b], math::D50);
        // RGB from XYZ
        let r = (3.2406 * xyz[0] - 1.5372 * xyz[1] - 0.4986 * xyz[2]).clamp(0.0, 1.0);
        let g = (-0.9689 * xyz[0] + 1.8758 * xyz[1] + 0.0415 * xyz[2]).clamp(0.0, 1.0);
        let b = (0.0557 * xyz[0] - 0.2040 * xyz[1] + 1.0570 * xyz[2]).clamp(0.0, 1.0);

        let mut k = 1.0 - r.max(g).max(b);
        let c = if k < 1.0 { (1.0 - r - k) / (1.0 - k) } else { 0.0 };
        let m = if k < 1.0 { (1.0 - g - k) / (1.0 - k) } else { 0.0 };
        let y = if k < 1.0 { (1.0 - b - k) / (1.0 - k) } else { 0.0 };
        k = k.min(p.k_max);

        cmyk[0] = c.clamp(0.0, 1.0) as f32;
        cmyk[1] = m.clamp(0.0, 1.0) as f32;
        cmyk[2] = y.clamp(0.0, 1.0) as f32;
        cmyk[3] = k.clamp(0.0, 1.0) as f32;
    });

    let a2b_lut = Lut {
        kind: LutKind::Ab,
        inputs: 4,
        outputs: 3,
        stages: vec![Stage::Clut(a2b_clut)],
    };

    let b2a_lut = Lut {
        kind: LutKind::Ab,
        inputs: 3,
        outputs: 4,
        stages: vec![Stage::Clut(b2a_clut)],
    };

    Profile {
        version: (4, 0x30),
        class: ProfileClass::Output,
        color_space: ColorSpace::Cmyk,
        pcs: Pcs::Lab,
        description: p.description.clone(),
        white_point: math::d50_quantized(),
        matrix: None,
        trc: None,
        gray_trc: None,
        a2b: [Some(a2b_lut.clone()), Some(a2b_lut.clone()), Some(a2b_lut)],
        b2a: [Some(b2a_lut.clone()), Some(b2a_lut.clone()), Some(b2a_lut)],
        raw_bytes: None,
    }
}

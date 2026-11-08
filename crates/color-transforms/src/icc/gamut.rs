//! Gamut checks against a proof (output) profile: out-of-gamut mask detection.

use crate::icc::clut::Clut;
use crate::icc::pipeline::{Pipeline, Stage};
use crate::icc::profile::{CmsError, Intent, Profile};
use crate::icc::transform::link_stages;

pub const DEFAULT_THRESHOLD: f32 = 4.0;

#[derive(Debug)]
pub struct GamutCheck {
    inputs: usize,
    grid: Option<Clut>,
    exact: (Pipeline, Pipeline),
    pub threshold: f32,
}

impl GamutCheck {
    pub fn new(src: &Profile, proof: &Profile, threshold: f32) -> Result<GamutCheck, CmsError> {
        let lab = crate::icc::builtin::Builtin::LabD50.profile();
        let direct = Pipeline::new(src.channels(), link_stages(src, lab, Intent::RelativeColorimetric, false)?);
        let mut via = link_stages(src, proof, Intent::RelativeColorimetric, false)?;
        via.push(Stage::Clamp01);
        via.extend(link_stages(proof, lab, Intent::RelativeColorimetric, false)?);
        let via = Pipeline::new(src.channels(), via);

        let de = |p: &[f32], o: &mut [f32]| {
            let mut a = [0.0f32; 16];
            let mut b = [0.0f32; 16];
            direct.eval(p, &mut a);
            via.eval(p, &mut b);
            let d0 = (a[0] - b[0]) * 100.0;
            let d1 = (a[1] - b[1]) * 255.0;
            let d2 = (a[2] - b[2]) * 255.0;
            o[0] = (d0 * d0 + d1 * d1 + d2 * d2).sqrt();
        };

        let n = src.channels();
        let grid = match n {
            3 => Some(Clut::sample(vec![33; 3], 1, de)),
            4 => Some(Clut::sample(vec![17; 4], 1, de)),
            _ => None,
        };

        Ok(GamutCheck {
            inputs: n,
            grid,
            exact: (direct, via),
            threshold,
        })
    }

    pub fn inputs(&self) -> usize {
        self.inputs
    }

    pub fn delta_e(&self, px: &[f32]) -> f32 {
        let mut o = [0.0f32; 1];
        match &self.grid {
            Some(g) => {
                let mut c = [0.0f32; 16];
                for (d, s) in c.iter_mut().zip(&px[..self.inputs]) {
                    *d = s.clamp(0.0, 1.0);
                }
                g.eval(&c[..self.inputs], &mut o);
            }
            None => {
                let mut a = [0.0f32; 16];
                let mut b = [0.0f32; 16];
                self.exact.0.eval(px, &mut a);
                self.exact.1.eval(px, &mut b);
                let d0 = (a[0] - b[0]) * 100.0;
                let d1 = (a[1] - b[1]) * 255.0;
                let d2 = (a[2] - b[2]) * 255.0;
                o[0] = (d0 * d0 + d1 * d1 + d2 * d2).sqrt();
            }
        }
        o[0]
    }

    pub fn out_of_gamut(&self, px: &[f32]) -> bool {
        self.delta_e(px) > self.threshold
    }

    /// Evaluates a mask buffer: writes 255 for out-of-gamut pixels, 0 for in-gamut pixels.
    pub fn generate_mask(&self, pixels: &[f32], stride: usize, mask_out: &mut [u8]) {
        for (px, m) in pixels.chunks_exact(stride).zip(mask_out.iter_mut()) {
            *m = if self.out_of_gamut(px) { 255 } else { 0 };
        }
    }
}

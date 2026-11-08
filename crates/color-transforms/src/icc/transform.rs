//! Colour transforms between profiles through PCS XYZ/Lab D50.
//!
//! Provides 4 rendering intents (Perceptual, RelativeColorimetric, Saturation, AbsoluteColorimetric),
//! Adobe-style Black Point Compensation (BPC), tetrahedral CLUT lookup acceleration,
//! and accurate float/8-bit buffer conversions.

use crate::icc::clut::Clut;
use crate::icc::math;
use crate::icc::pipeline::{Pipeline, Stage};
use crate::icc::profile::{CmsError, ColorSpace, Intent, Pcs, Profile, ProfileClass};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TransformOptions {
    pub intent: Intent,
    pub bpc: bool,
    pub precise_float: bool,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            intent: Intent::RelativeColorimetric,
            bpc: true,
            precise_float: true,
        }
    }
}

pub const GRID_3D: usize = 33;
pub const GRID_4D: usize = 17;
const PERCEPTUAL_BLACK: [f64; 3] = [0.00336, 0.0034731, 0.00287];

#[derive(Debug)]
enum Core {
    Grid(Clut),
    Exact,
}

#[derive(Debug)]
pub struct Transform {
    inputs: usize,
    outputs: usize,
    opts: TransformOptions,
    pipeline: Pipeline,
    core: Core,
}

impl Transform {
    pub fn new(src: &Profile, dst: &Profile, intent: Intent, bpc: bool) -> Result<Transform, CmsError> {
        Self::with_options(src, dst, TransformOptions { intent, bpc, ..Default::default() })
    }

    pub fn with_options(src: &Profile, dst: &Profile, opts: TransformOptions) -> Result<Transform, CmsError> {
        let stages = link_stages(src, dst, opts.intent, opts.bpc)?;
        let mut pipe = Pipeline::new(src.channels(), stages);
        pipe.optimize();

        let inputs = pipe.inputs;
        let outputs = pipe.outputs;

        let core = match inputs {
            3 => {
                let p = pipe.clone();
                Core::Grid(Clut::sample(vec![GRID_3D; 3], outputs, move |i, o| p.eval(i, o)))
            }
            4 => {
                let p = pipe.clone();
                Core::Grid(Clut::sample(vec![GRID_4D; 4], outputs, move |i, o| p.eval(i, o)))
            }
            _ => Core::Exact,
        };

        Ok(Transform {
            inputs,
            outputs,
            opts,
            pipeline: pipe,
            core,
        })
    }

    pub fn inputs(&self) -> usize {
        self.inputs
    }

    pub fn outputs(&self) -> usize {
        self.outputs
    }

    pub fn options(&self) -> TransformOptions {
        self.opts
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    #[inline]
    pub fn eval(&self, input: &[f32], out: &mut [f32]) {
        self.pipeline.eval(input, out);
    }

    #[inline]
    pub fn eval_fast(&self, input: &[f32], out: &mut [f32]) {
        match &self.core {
            Core::Grid(c) => c.eval(input, out),
            Core::Exact => self.pipeline.eval(input, out),
        }
    }

    /// Converts interleaved float pixels.
    pub fn convert_f32(&self, src: &[f32], src_stride: usize, dst: &mut [f32], dst_stride: usize, copy_extra: bool) {
        assert!(src_stride >= self.inputs && dst_stride >= self.outputs);
        let extra = if copy_extra {
            (src_stride - self.inputs).min(dst_stride - self.outputs)
        } else {
            0
        };

        let precise = self.opts.precise_float;
        for (sp, dp) in src.chunks_exact(src_stride).zip(dst.chunks_exact_mut(dst_stride)) {
            if precise {
                self.eval(sp, dp);
            } else {
                self.eval_fast(sp, dp);
            }
            for e in 0..extra {
                dp[self.outputs + e] = sp[self.inputs + e];
            }
        }
    }

    /// Converts interleaved 8-bit pixels.
    pub fn convert_u8(&self, src: &[u8], src_stride: usize, dst: &mut [u8], dst_stride: usize, copy_extra: bool) {
        assert!(src_stride >= self.inputs && dst_stride >= self.outputs);
        let extra = if copy_extra {
            (src_stride - self.inputs).min(dst_stride - self.outputs)
        } else {
            0
        };

        let mut in_f = [0.0f32; 16];
        let mut out_f = [0.0f32; 16];

        for (sp, dp) in src.chunks_exact(src_stride).zip(dst.chunks_exact_mut(dst_stride)) {
            for i in 0..self.inputs {
                in_f[i] = sp[i] as f32 / 255.0;
            }
            self.eval_fast(&in_f[..self.inputs], &mut out_f[..self.outputs]);
            for i in 0..self.outputs {
                dp[i] = (out_f[i].clamp(0.0, 1.0) * 255.0 + 0.5) as u8;
            }
            for e in 0..extra {
                dp[self.outputs + e] = sp[self.inputs + e];
            }
        }
    }
}

pub(crate) fn link_stages(src: &Profile, dst: &Profile, intent: Intent, bpc: bool) -> Result<Vec<Stage>, CmsError> {
    for p in [src, dst] {
        if matches!(p.class, ProfileClass::DeviceLink | ProfileClass::NamedColor | ProfileClass::Abstract) {
            return Err(CmsError::Unsupported(format!("{:?} profile cannot be used as source/destination", p.class)));
        }
    }
    let (mut stages, s_pcs) = src.device_to_pcs(intent)?;
    let (d_stages, d_pcs) = dst.pcs_to_device(intent)?;
    let adjust = connection(src, dst, intent, bpc)?;

    match adjust {
        Some((scale, offset)) => {
            if s_pcs == Pcs::Lab {
                stages.push(Stage::LabToXyz);
            }
            stages.push(Stage::Matrix {
                rows: 3,
                cols: 3,
                m: vec![scale[0], 0.0, 0.0, 0.0, scale[1], 0.0, 0.0, 0.0, scale[2]],
                offset: offset.to_vec(),
            });
            if d_pcs == Pcs::Lab {
                stages.push(Stage::XyzToLab);
            }
        }
        None => match (s_pcs, d_pcs) {
            (Pcs::Lab, Pcs::Xyz) => stages.push(Stage::LabToXyz),
            (Pcs::Xyz, Pcs::Lab) => stages.push(Stage::XyzToLab),
            _ => {}
        },
    }
    stages.extend(d_stages);
    Ok(stages)
}

type Affine3 = ([f64; 3], [f64; 3]);

fn connection(src: &Profile, dst: &Profile, intent: Intent, bpc: bool) -> Result<Option<Affine3>, CmsError> {
    if intent == Intent::AbsoluteColorimetric {
        let (ws, wd) = (src.media_white(), dst.media_white());
        let usable = |w: &[f64; 3]| w.iter().all(|v| v.is_finite() && *v > 0.0);
        if !usable(&ws) || !usable(&wd) {
            return Err(CmsError::Invalid("absolute colorimetric requires positive media white point".into()));
        }
        let s = [ws[0] / wd[0], ws[1] / wd[1], ws[2] / wd[2]];
        if s.iter().all(|v| (v - 1.0).abs() < 1e-6) {
            return Ok(None);
        }
        return Ok(Some((s, [0.0; 3])));
    }
    if !bpc {
        return Ok(None);
    }
    let bs = black_point(src, intent)?;
    let bd = black_point(dst, intent)?;
    if (bs - bd).abs() < 1e-6 {
        return Ok(None);
    }
    let w = math::D50;
    let mut scale = [1.0; 3];
    let mut offset = [0.0; 3];
    for i in 0..3 {
        let (s, d) = (bs * w[i], bd * w[i]);
        let a = (w[i] - d) / (w[i] - s);
        scale[i] = a;
        offset[i] = w[i] * (1.0 - a);
    }
    Ok(Some((scale, offset)))
}

pub fn black_point(p: &Profile, intent: Intent) -> Result<f64, CmsError> {
    if matches!(p.color_space, ColorSpace::Lab | ColorSpace::Xyz) {
        return Ok(0.0);
    }
    let lut_based = p.a2b.iter().any(Option::is_some);
    if lut_based && p.version.0 >= 4 && matches!(intent, Intent::Perceptual | Intent::Saturation) {
        return Ok(PERCEPTUAL_BLACK[1]);
    }
    // For matrix/TRC profiles, black is at (0, 0, 0)
    let (st, pcs) = p.device_to_pcs(intent)?;
    let pipe = Pipeline::new(p.channels(), st);
    let dev = vec![0.0f32; p.channels()];
    let mut o = [0.0f32; 16];
    pipe.eval(&dev, &mut o);
    let y = match pcs {
        Pcs::Xyz => o[1] as f64,
        Pcs::Lab => math::lab_to_xyz([o[0] as f64, 0.0, 0.0], math::D50)[1],
    };
    Ok(y.clamp(0.0, 0.5))
}

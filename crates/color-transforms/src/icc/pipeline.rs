//! Float evaluation pipelines: stages mapping normalised device values to PCS (or back).

use crate::icc::clut::Clut;
use crate::icc::curve::Curve;
use crate::icc::math;

pub const MAX: usize = 16;

#[derive(Clone, Debug, PartialEq)]
pub enum Stage {
    Curves(Vec<Curve>),
    InvCurves(Vec<Curve>),
    Matrix {
        rows: usize,
        cols: usize,
        m: Vec<f64>,
        offset: Vec<f64>,
    },
    Clut(Clut),
    LabToXyz,
    XyzToLab,
    DecodeLabV4,
    EncodeLabV4,
    DecodeLabV2,
    EncodeLabV2,
    DecodeXyz,
    EncodeXyz,
    Clamp01,
}

const XYZ_SCALE: f32 = 65535.0 / 32768.0;
const V2_L: f32 = 100.0 * 65535.0 / 65280.0;
const V2_AB: f32 = 255.0 * 65535.0 / 65280.0;

impl Stage {
    pub fn outputs(&self, n: usize) -> usize {
        match self {
            Stage::Matrix { rows, .. } => *rows,
            Stage::Clut(c) => c.outputs,
            _ => n,
        }
    }

    pub fn matrix3(m: &math::Mat3) -> Stage {
        Stage::Matrix {
            rows: 3,
            cols: 3,
            m: m.iter().flatten().copied().collect(),
            offset: vec![0.0; 3],
        }
    }

    pub fn eval(&self, input: &[f32], out: &mut [f32]) {
        match self {
            Stage::Curves(cs) => {
                for (i, c) in cs.iter().enumerate() {
                    out[i] = c.eval32(input[i]);
                }
            }
            Stage::InvCurves(cs) => {
                for (i, c) in cs.iter().enumerate() {
                    out[i] = c.eval_inverse64(input[i] as f64) as f32;
                }
            }
            Stage::Matrix { rows, cols, m, offset } => {
                for r in 0..*rows {
                    let mut s = offset.get(r).copied().unwrap_or(0.0);
                    let row = &m[r * cols..(r + 1) * cols];
                    for (c, &w) in row.iter().enumerate().take(input.len()) {
                        s += w * input[c] as f64;
                    }
                    out[r] = s as f32;
                }
            }
            Stage::Clut(c) => c.eval(input, out),
            Stage::LabToXyz => {
                let lab = [input[0] as f64, input[1] as f64, input[2] as f64];
                let xyz = math::lab_to_xyz(lab, math::D50);
                out[0] = xyz[0] as f32;
                out[1] = xyz[1] as f32;
                out[2] = xyz[2] as f32;
            }
            Stage::XyzToLab => {
                let xyz = [input[0] as f64, input[1] as f64, input[2] as f64];
                let lab = math::xyz_to_lab(xyz, math::D50);
                out[0] = lab[0] as f32;
                out[1] = lab[1] as f32;
                out[2] = lab[2] as f32;
            }
            Stage::DecodeLabV4 => {
                out[0] = input[0] * 100.0;
                out[1] = input[1] * 255.0 - 128.0;
                out[2] = input[2] * 255.0 - 128.0;
            }
            Stage::EncodeLabV4 => {
                out[0] = input[0] / 100.0;
                out[1] = (input[1] + 128.0) / 255.0;
                out[2] = (input[2] + 128.0) / 255.0;
            }
            Stage::DecodeLabV2 => {
                out[0] = input[0] * V2_L;
                out[1] = input[1] * V2_AB - 128.0;
                out[2] = input[2] * V2_AB - 128.0;
            }
            Stage::EncodeLabV2 => {
                out[0] = input[0] / V2_L;
                out[1] = (input[1] + 128.0) / V2_AB;
                out[2] = (input[2] + 128.0) / V2_AB;
            }
            Stage::DecodeXyz => {
                out[0] = input[0] * XYZ_SCALE;
                out[1] = input[1] * XYZ_SCALE;
                out[2] = input[2] * XYZ_SCALE;
            }
            Stage::EncodeXyz => {
                out[0] = input[0] / XYZ_SCALE;
                out[1] = input[1] / XYZ_SCALE;
                out[2] = input[2] / XYZ_SCALE;
            }
            Stage::Clamp01 => {
                for (i, v) in input.iter().enumerate() {
                    out[i] = v.clamp(0.0, 1.0);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pipeline {
    pub inputs: usize,
    pub outputs: usize,
    pub stages: Vec<Stage>,
}

impl Pipeline {
    pub fn new(inputs: usize, stages: Vec<Stage>) -> Self {
        let mut n = inputs;
        for s in &stages {
            n = s.outputs(n);
        }
        Pipeline { inputs, outputs: n, stages }
    }

    pub fn eval(&self, input: &[f32], out: &mut [f32]) {
        let mut buf_a = [0.0f32; MAX];
        let mut buf_b = [0.0f32; MAX];
        let mut current_len = self.inputs;
        buf_a[..current_len].copy_from_slice(&input[..current_len]);

        let mut in_a = true;
        for s in &self.stages {
            let next_len = s.outputs(current_len);
            if in_a {
                s.eval(&buf_a[..current_len], &mut buf_b[..next_len]);
            } else {
                s.eval(&buf_b[..current_len], &mut buf_a[..next_len]);
            }
            current_len = next_len;
            in_a = !in_a;
        }

        if in_a {
            out[..self.outputs].copy_from_slice(&buf_a[..self.outputs]);
        } else {
            out[..self.outputs].copy_from_slice(&buf_b[..self.outputs]);
        }
    }

    pub fn optimize(&mut self) {
        // Strip identity curves
        self.stages.retain(|s| match s {
            Stage::Curves(cs) | Stage::InvCurves(cs) => !cs.iter().all(Curve::is_identity),
            _ => true,
        });
    }
}

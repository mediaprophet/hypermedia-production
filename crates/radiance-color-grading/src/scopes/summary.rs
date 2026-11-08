//! Numeric statistics and metrics from scopes analysis.

use serde::Serialize;

use crate::matrix::{Matrix, rgb_to_ycbcr};
use crate::scopes::{Grid, Signal, Vectorscope, Waveform, angle_deg};

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Level {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

#[derive(Default)]
struct Acc {
    min: f32,
    max: f32,
    sum: f64,
    n: u64,
}

impl Acc {
    fn add(&mut self, v: f32, count: u64) {
        if self.n == 0 {
            self.min = v;
            self.max = v;
        } else {
            self.min = self.min.min(v);
            self.max = self.max.max(v);
        }
        self.sum += v as f64 * count as f64;
        self.n += count;
    }

    fn level(&self) -> Option<Level> {
        (self.n > 0).then(|| Level {
            min: round2(self.min),
            max: round2(self.max),
            mean: round2(self.sum / self.n as f64),
        })
    }
}

fn round2(v: impl Into<f64>) -> f64 {
    (v.into() * 100.0).round() / 100.0
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChannelStats {
    pub r: Level,
    pub g: Level,
    pub b: Level,
    pub y: Level,
    pub chroma_mean: [f64; 2],
    pub hue_deg: f64,
    pub saturation: f64,
}

pub fn channel_stats(s: &Signal, m: Matrix) -> Option<ChannelStats> {
    if s.is_empty() {
        return None;
    }
    let mut acc: [Acc; 4] = Default::default();
    let (mut cb_sum, mut cr_sum) = (0f64, 0f64);
    for &[r, g, b] in &s.rgb {
        let [y, cb, cr] = rgb_to_ycbcr(r, g, b, m);
        for (a, v) in acc.iter_mut().zip([r, g, b, y]) {
            a.add(v * 100.0, 1);
        }
        cb_sum += cb as f64;
        cr_sum += cr as f64;
    }
    let n = s.len() as f64;
    let (cb, cr) = ((cb_sum / n) as f32, (cr_sum / n) as f32);
    Some(ChannelStats {
        r: acc[0].level()?,
        g: acc[1].level()?,
        b: acc[2].level()?,
        y: acc[3].level()?,
        chroma_mean: [round2(cb * 100.0), round2(cr * 100.0)],
        hue_deg: round2(angle_deg(cb, cr)),
        saturation: round2((cb * cb + cr * cr).sqrt() / 0.5 * 100.0),
    })
}

pub fn peaks(v: &Vectorscope, count: usize, min_fraction: f32) -> Vec<(usize, usize, u32)> {
    let mut out: Vec<(usize, usize, u32)> = Vec::new();
    let thresh = ((v.samples as f32) * min_fraction) as u32;
    for y in 0..v.size {
        for x in 0..v.size {
            let c = v.at(x, y);
            if c >= thresh {
                out.push((x, y, c));
            }
        }
    }
    out.sort_by(|a, b| b.2.cmp(&a.2));
    out.truncate(count);
    out
}

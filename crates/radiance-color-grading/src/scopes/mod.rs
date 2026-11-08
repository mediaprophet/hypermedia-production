//! Video scopes maths (Waveform, Vectorscope, RGB Parade, and Histogram).
//!
//! Grounded in SMPTE RP 219, ITU-R BT.709, BT.2020, and international standards for video signal
//! monitoring and perceptual validation.

pub mod paint;
pub mod summary;
#[cfg(test)]
pub mod tests;

use serde::{Deserialize, Serialize};

use crate::matrix::{Matrix, rgb_to_ycbcr};

pub const MAX_W: usize = 480;
pub const MAX_H: usize = 270;
pub const VECTOR_EXTENT: f32 = 0.6;
pub const SKIN_TONE_DEG: f32 = 123.0;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScopeKind {
    VectorscopeYuv,
    VectorscopeHls,
    Histogram,
    Parade,
    #[default]
    Waveform,
}

impl ScopeKind {
    pub const ALL: [ScopeKind; 5] = [
        ScopeKind::VectorscopeYuv,
        ScopeKind::VectorscopeHls,
        ScopeKind::Histogram,
        ScopeKind::Parade,
        ScopeKind::Waveform,
    ];

    pub fn name(self) -> &'static str {
        match self {
            ScopeKind::VectorscopeYuv => "vectorscopeYuv",
            ScopeKind::VectorscopeHls => "vectorscopeHls",
            ScopeKind::Histogram => "histogram",
            ScopeKind::Parade => "parade",
            ScopeKind::Waveform => "waveform",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            ScopeKind::VectorscopeYuv => "Vectorscope YUV",
            ScopeKind::VectorscopeHls => "Vectorscope HLS",
            ScopeKind::Histogram => "Histogram",
            ScopeKind::Parade => "Parade",
            ScopeKind::Waveform => "Waveform",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WaveformType {
    #[default]
    Rgb,
    Luma,
    Yc,
    YcNoChroma,
}

impl WaveformType {
    pub const ALL: [WaveformType; 4] = [
        WaveformType::Rgb,
        WaveformType::Luma,
        WaveformType::Yc,
        WaveformType::YcNoChroma,
    ];

    pub fn label(self) -> &'static str {
        match self {
            WaveformType::Rgb => "RGB",
            WaveformType::Luma => "Luma",
            WaveformType::Yc => "YC",
            WaveformType::YcNoChroma => "YC no Chroma",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParadeType {
    #[default]
    Rgb,
    Yuv,
    RgbWhite,
}

impl ParadeType {
    pub const ALL: [ParadeType; 3] = [ParadeType::Rgb, ParadeType::Yuv, ParadeType::RgbWhite];

    pub fn label(self) -> &'static str {
        match self {
            ParadeType::Rgb => "RGB",
            ParadeType::Yuv => "YUV",
            ParadeType::RgbWhite => "RGB-White",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Targets {
    #[default]
    #[serde(rename = "75")]
    Percent75,
    #[serde(rename = "100")]
    Percent100,
}

impl Targets {
    pub fn amplitude(self) -> f32 {
        match self {
            Targets::Percent75 => 0.75,
            Targets::Percent100 => 1.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Targets::Percent75 => "75%",
            Targets::Percent100 => "100%",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub matrix: Matrix,
    pub clamp: bool,
    pub rows: usize,
    pub vector_size: usize,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            matrix: Matrix::Bt709,
            clamp: true,
            rows: 256,
            vector_size: 255,
        }
    }
}

impl Params {
    pub fn range(&self) -> (f32, f32) {
        if self.clamp {
            (0.0, 1.0)
        } else {
            (-0.1, 1.1)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Signal {
    pub w: usize,
    pub h: usize,
    pub rgb: Vec<[f32; 3]>,
}

fn steps(w: usize, h: usize, max_w: usize, max_h: usize) -> (usize, usize) {
    (w.div_ceil(max_w.max(1)).max(1), h.div_ceil(max_h.max(1)).max(1))
}

impl Signal {
    pub fn from_rgba8(w: usize, h: usize, px: &[u8]) -> Signal {
        Self::from_rgba8_max(w, h, px, MAX_W, MAX_H)
    }

    pub fn from_rgba8_max(w: usize, h: usize, px: &[u8], max_w: usize, max_h: usize) -> Signal {
        if w == 0 || h == 0 || px.len() < w * h * 4 {
            return Signal::default();
        }
        let lut: [f32; 256] = std::array::from_fn(|i| i as f32 / 255.0);
        let (sx, sy) = steps(w, h, max_w, max_h);
        let (ow, oh) = (w.div_ceil(sx), h.div_ceil(sy));
        let mut rgb = Vec::with_capacity(ow * oh);
        for y in (0..h).step_by(sy) {
            let row = &px[y * w * 4..(y + 1) * w * 4];
            for x in (0..w).step_by(sx) {
                let p = &row[x * 4..x * 4 + 3];
                rgb.push([lut[p[0] as usize], lut[p[1] as usize], lut[p[2] as usize]]);
            }
        }
        Signal { w: ow, h: oh, rgb }
    }

    pub fn from_fn(w: usize, h: usize, f: impl Fn(usize, usize) -> [f32; 3]) -> Signal {
        let mut rgb = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                rgb.push(f(x, y));
            }
        }
        Signal { w, h, rgb }
    }

    pub fn len(&self) -> usize {
        self.rgb.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rgb.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Grid {
    pub name: &'static str,
    pub cols: usize,
    pub rows: usize,
    pub data: Vec<u32>,
}

impl Grid {
    pub fn new(name: &'static str, cols: usize, rows: usize) -> Grid {
        Grid {
            name,
            cols,
            rows,
            data: vec![0; cols * rows],
        }
    }

    pub fn at(&self, col: usize, row: usize) -> u32 {
        self.data[row * self.cols + col]
    }

    pub fn total(&self) -> u64 {
        self.data.iter().map(|&c| c as u64).sum()
    }

    pub fn max(&self) -> u32 {
        self.data.iter().copied().max().unwrap_or(0)
    }

    pub fn rows_in(&self, col: usize) -> Vec<usize> {
        (0..self.rows).filter(|&r| self.at(col, r) > 0).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Waveform {
    pub lo: f32,
    pub hi: f32,
    pub per_column: usize,
    pub traces: Vec<Grid>,
}

impl Waveform {
    pub fn trace(&self, name: &str) -> Option<&Grid> {
        self.traces.iter().find(|t| t.name == name)
    }

    pub fn level(&self, row: usize, rows: usize) -> f32 {
        self.lo + (self.hi - self.lo) * row as f32 / (rows.max(2) - 1) as f32
    }
}

#[inline]
pub fn row_of(v: f32, lo: f32, hi: f32, rows: usize) -> Option<usize> {
    let t = (v - lo) / (hi - lo);
    if !(0.0..=1.0).contains(&t) {
        return None;
    }
    Some(((t * (rows - 1) as f32).round() as usize).min(rows - 1))
}

type Hits = ([(u8, f32); 4], usize);

fn build(s: &Signal, p: &Params, names: &[&'static str], sample: impl Fn([f32; 3]) -> Hits) -> Waveform {
    let (lo, hi) = p.range();
    let rows = p.rows.max(2);
    let mut traces: Vec<Grid> = names.iter().map(|n| Grid::new(n, s.w, rows)).collect();
    for y in 0..s.h {
        for x in 0..s.w {
            let mut c = s.rgb[y * s.w + x];
            if p.clamp {
                c = c.map(|v| v.clamp(0.0, 1.0));
            }
            let (hits, n) = sample(c);
            for &(t, v) in &hits[..n] {
                let v = if p.clamp { v.clamp(0.0, 1.0) } else { v };
                if let Some(r) = row_of(v, lo, hi, rows) {
                    let g = &mut traces[t as usize];
                    g.data[r * g.cols + x] += 1;
                }
            }
        }
    }
    Waveform {
        lo,
        hi,
        per_column: s.h,
        traces,
    }
}

pub fn waveform(s: &Signal, kind: WaveformType, p: &Params) -> Waveform {
    let m = p.matrix;
    let z = (0u8, 0.0f32);
    match kind {
        WaveformType::Rgb => build(s, p, &["R", "G", "B"], |[r, g, b]| {
            ([(0, r), (1, g), (2, b), z], 3)
        }),
        WaveformType::Luma | WaveformType::YcNoChroma => build(s, p, &["Y"], |[r, g, b]| {
            ([(0, rgb_to_ycbcr(r, g, b, m)[0]), z, z, z], 1)
        }),
        WaveformType::Yc => build(s, p, &["Y", "C"], |[r, g, b]| {
            let [y, cb, cr] = rgb_to_ycbcr(r, g, b, m);
            let c = (cb * cb + cr * cr).sqrt();
            ([(0, y), (1, y + c), (1, y - c), z], if c > 1e-6 { 3 } else { 2 })
        }),
    }
}

pub fn parade(s: &Signal, kind: ParadeType, p: &Params) -> Waveform {
    let m = p.matrix;
    let z = (0u8, 0.0f32);
    match kind {
        ParadeType::Rgb => build(s, p, &["R", "G", "B"], |[r, g, b]| {
            ([(0, r), (1, g), (2, b), z], 3)
        }),
        ParadeType::RgbWhite => build(s, p, &["R", "G", "B", "Y"], |[r, g, b]| {
            (
                [(0, r), (1, g), (2, b), (3, rgb_to_ycbcr(r, g, b, m)[0])],
                4,
            )
        }),
        ParadeType::Yuv => build(s, p, &["Y", "Cb", "Cr"], |[r, g, b]| {
            let [y, cb, cr] = rgb_to_ycbcr(r, g, b, m);
            ([(0, y), (1, cb + 0.5), (2, cr + 0.5), z], 3)
        }),
    }
}

pub const BINS: usize = 256;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Histogram {
    pub r: Vec<u32>,
    pub g: Vec<u32>,
    pub b: Vec<u32>,
    pub y: Vec<u32>,
    pub samples: u32,
    pub below: [u32; 4],
    pub above: [u32; 4],
}

#[inline]
pub fn bin_of(v: f32) -> Option<usize> {
    if (0.0..=1.0 + 1e-6).contains(&v) {
        Some(((v * 255.0).round() as usize).min(BINS - 1))
    } else {
        None
    }
}

pub fn histogram(s: &Signal, p: &Params) -> Histogram {
    let mut h = Histogram {
        r: vec![0; BINS],
        g: vec![0; BINS],
        b: vec![0; BINS],
        y: vec![0; BINS],
        samples: s.len() as u32,
        below: [0; 4],
        above: [0; 4],
    };
    for &[r, g, b] in &s.rgb {
        let y = rgb_to_ycbcr(r, g, b, p.matrix)[0];
        for (i, v) in [r, g, b, y].into_iter().enumerate() {
            let bins = match i {
                0 => &mut h.r,
                1 => &mut h.g,
                2 => &mut h.b,
                _ => &mut h.y,
            };
            match bin_of(v) {
                Some(k) => bins[k] += 1,
                None if v.is_nan() => {}
                None => {
                    let low = v < 0.0;
                    if low {
                        h.below[i] += 1;
                    } else {
                        h.above[i] += 1;
                    }
                    if p.clamp {
                        bins[if low { 0 } else { BINS - 1 }] += 1;
                    }
                }
            }
        }
    }
    h
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Vectorscope {
    pub size: usize,
    pub extent: f32,
    pub data: Vec<u32>,
    pub samples: u32,
    pub mean: [f32; 2],
}

#[inline]
pub fn cell_of(cb: f32, cr: f32, size: usize, extent: f32) -> Option<(usize, usize)> {
    let n = (size - 1) as f32;
    let fx = (cb / extent * 0.5 + 0.5) * n;
    let fy = (0.5 - cr / extent * 0.5) * n;
    if !(0.0..=n).contains(&fx) || !(0.0..=n).contains(&fy) {
        return None;
    }
    Some((fx.round() as usize, fy.round() as usize))
}

impl Vectorscope {
    fn new(size: usize) -> Self {
        Vectorscope {
            size,
            extent: VECTOR_EXTENT,
            data: vec![0; size * size],
            samples: 0,
            mean: [0.0; 2],
        }
    }

    pub fn at(&self, x: usize, y: usize) -> u32 {
        self.data[y * self.size + x]
    }

    pub fn cell(&self, cb: f32, cr: f32) -> Option<(usize, usize)> {
        cell_of(cb, cr, self.size, self.extent)
    }

    pub fn point(&self, x: usize, y: usize) -> [f32; 2] {
        let n = (self.size - 1) as f32;
        [
            (x as f32 / n - 0.5) * 2.0 * self.extent,
            (0.5 - y as f32 / n) * 2.0 * self.extent,
        ]
    }

    fn plot(&mut self, cb: f32, cr: f32, sum: &mut [f64; 2]) {
        if let Some((x, y)) = self.cell(cb, cr) {
            self.data[y * self.size + x] += 1;
            self.samples += 1;
            sum[0] += cb as f64;
            sum[1] += cr as f64;
        }
    }

    fn finish(mut self, sum: [f64; 2]) -> Self {
        if self.samples > 0 {
            self.mean = [
                (sum[0] / self.samples as f64) as f32,
                (sum[1] / self.samples as f64) as f32,
            ];
        }
        self
    }
}

pub fn vectorscope_yuv(s: &Signal, p: &Params) -> Vectorscope {
    let mut v = Vectorscope::new(p.vector_size.max(2));
    let mut sum = [0f64; 2];
    for &c in &s.rgb {
        let [r, g, b] = if p.clamp {
            c.map(|x| x.clamp(0.0, 1.0))
        } else {
            c
        };
        let [_, cb, cr] = rgb_to_ycbcr(r, g, b, p.matrix);
        v.plot(cb, cr, &mut sum);
    }
    v.finish(sum)
}

pub fn angle_deg(cb: f32, cr: f32) -> f32 {
    cr.atan2(cb).to_degrees().rem_euclid(360.0)
}

pub fn red_angle(m: Matrix) -> f32 {
    let [_, cb, cr] = rgb_to_ycbcr(1.0, 0.0, 0.0, m);
    angle_deg(cb, cr)
}

pub fn vectorscope_hls(s: &Signal, p: &Params) -> Vectorscope {
    let mut v = Vectorscope::new(p.vector_size.max(2));
    let mut sum = [0f64; 2];
    let red = red_angle(p.matrix);
    let dirs: Vec<[f32; 2]> = (0..=3600)
        .map(|i| (red + i as f32 / 10.0).to_radians())
        .map(|a| [a.cos(), a.sin()])
        .collect();
    for &c in &s.rgb {
        let [r, g, b] = c.map(|x| x.clamp(0.0, 1.0));
        let [h, sat, _] = crate::matrix::rgb_to_hsl(r, g, b);
        let [dx, dy] = dirs[((h * 3600.0).round() as usize).min(3600)];
        let rad = sat.clamp(0.0, 1.0) * 0.5;
        v.plot(rad * dx, rad * dy, &mut sum);
    }
    v.finish(sum)
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Target {
    pub name: &'static str,
    pub rgb: [f32; 3],
    pub cb: f32,
    pub cr: f32,
}

pub fn targets(m: Matrix, amplitude: f32) -> [Target; 6] {
    let a = amplitude;
    let t = |name, rgb: [f32; 3]| {
        let [_, cb, cr] = rgb_to_ycbcr(rgb[0], rgb[1], rgb[2], m);
        Target { name, rgb, cb, cr }
    };
    [
        t("R", [a, 0.0, 0.0]),
        t("Mg", [a, 0.0, a]),
        t("B", [0.0, 0.0, a]),
        t("Cy", [0.0, a, a]),
        t("G", [0.0, a, 0.0]),
        t("Yl", [a, a, 0.0]),
    ]
}

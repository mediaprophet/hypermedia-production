//! Vector Radiance Trace: raster -> vector image tracing.
//!
//! A clean-room image tracer:
//! 1. **Quantise**: Black and White threshold, Grayscale k-means, or Color median-cut/k-means.
//! 2. **Denoise**: 4-connected component filtering and pinhole closing.
//! 3. **Follow boundaries**: pixel-crack contour walking into closed loops.
//! 4. **Polygon**: Douglas-Peucker reduction removing staircases.
//! 5. **Bézier fit**: Philip J. Schneider least-squares cubic Bézier fitting with corner preservation.
//! 6. **Centerline**: Zhang-Suen morphological thinning and skeleton graph extraction for stroked paths.

#![forbid(unsafe_code)]

mod centerline;
mod contour;
mod fit;
mod mosaic;
mod quantize;

#[cfg(test)]
mod tests;

use std::sync::atomic::{AtomicUsize, Ordering};

use serde::{Deserialize, Serialize};
use vector_locus_geometry::PathData;

pub use contour::{Component, Loop, trace_mask};
pub use mosaic::mosaic;
pub use quantize::{FULL_TONE_MAX, Quantized, TRANSPARENT, denoise, quantize};

/// Errors encountered while tracing an image.
#[derive(Debug, thiserror::Error)]
pub enum TraceError {
    #[error("image buffer is empty or invalid dimensions")]
    Empty,
    #[error("the image is {width} x {height} pixels, more than the {max} megapixel limit")]
    TooLarge { width: u32, height: u32, max: u64 },
    #[error("\"{0}\" is not a valid colour: use #rrggbb")]
    BadPalette(String),
    #[error("trace exceeded maximum complexity budget of {max} anchors")]
    TooComplex { max: usize },
}

/// The maximum number of pixels an image may contain (64 megapixels).
pub const MAX_PIXELS: u64 = 64 << 20;

/// An RGBA8 raster (row-major, top row first).
#[derive(Clone, Debug, PartialEq)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

impl Raster {
    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Self {
        assert_eq!(
            rgba.len(),
            width as usize * height as usize * 4,
            "rgba length must be width * height * 4"
        );
        Self { width, height, rgba }
    }

    pub fn from_fn(width: u32, height: u32, f: impl Fn(u32, u32) -> [u8; 4]) -> Self {
        let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
        for y in 0..height {
            for x in 0..width {
                rgba.extend_from_slice(&f(x, y));
            }
        }
        Self { width, height, rgba }
    }

    pub fn pixel(&self, x: u32, y: u32) -> [u8; 4] {
        let i = (y as usize * self.width as usize + x as usize) * 4;
        [
            self.rgba[i],
            self.rgba[i + 1],
            self.rgba[i + 2],
            self.rgba[i + 3],
        ]
    }
}

/// Tracing mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    #[default]
    BlackAndWhite,
    Grayscale,
    Color,
    Logo,
}

pub type TraceMode = Mode;

/// Layer arrangement method.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Method {
    #[default]
    Abutting,
    Overlapping,
}

pub type TraceMethod = Method;

/// Where Color mode palette colours originate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Palette {
    #[default]
    Limited,
    FullTone,
    Automatic,
    DocumentLibrary,
}

pub type TracePalette = Palette;

/// Image Trace parameters and options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct TraceParams {
    pub mode: Mode,
    pub threshold: u8,
    pub palette: Palette,
    pub colors: u32,
    pub color_detail: f64,
    pub library: String,
    #[serde(skip)]
    pub swatches: Vec<[u8; 3]>,
    pub paths: f64,
    pub corners: f64,
    pub noise: u32,
    pub method: Method,
    pub ignore_white: bool,
    pub snap_curves_to_lines: bool,
    pub fills: bool,
    pub strokes: bool,
    pub stroke_width: f64,
    pub logo_colors: Vec<String>,
}

pub type TraceOptions = TraceParams;

impl Default for TraceParams {
    fn default() -> Self {
        Self {
            mode: Mode::BlackAndWhite,
            threshold: 128,
            palette: Palette::Limited,
            colors: 6,
            color_detail: 50.0,
            library: "document".into(),
            swatches: vec![],
            paths: 50.0,
            corners: 75.0,
            noise: 25,
            method: Method::Abutting,
            ignore_white: false,
            snap_curves_to_lines: false,
            fills: true,
            strokes: false,
            stroke_width: 10.0,
            logo_colors: Vec::new(),
        }
    }
}

impl TraceParams {
    fn polygon_tolerance(&self) -> f64 {
        let f = (self.paths / 100.0).clamp(0.0, 1.0);
        0.55 + 0.75 * (1.0 - f)
    }

    fn fit_tolerance(&self) -> f64 {
        let f = (self.paths / 100.0).clamp(0.0, 1.0);
        0.2 + 1.6 * (1.0 - f)
    }

    fn corner_angle(&self) -> f64 {
        let c = (self.corners / 100.0).clamp(0.0, 1.0);
        150.0 - 115.0 * c
    }
}

/// Trace Presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TracePreset {
    Default,
    HighFidelityPhoto,
    LowFidelityPhoto,
    ThreeColors,
    SixColors,
    SixteenColors,
    ShadesOfGray,
    BlackAndWhiteLogo,
    SketchedArt,
    Silhouettes,
    LineArt,
    TechnicalDrawing,
}

impl TracePreset {
    pub fn to_options(self) -> TraceOptions {
        match self {
            Self::Default => preset("default").unwrap(),
            Self::HighFidelityPhoto => preset("high fidelity photo").unwrap(),
            Self::LowFidelityPhoto => preset("low fidelity photo").unwrap(),
            Self::ThreeColors => preset("3 colors").unwrap(),
            Self::SixColors => preset("6 colors").unwrap(),
            Self::SixteenColors => preset("16 colors").unwrap(),
            Self::ShadesOfGray => preset("shades of gray").unwrap(),
            Self::BlackAndWhiteLogo => preset("black and white logo").unwrap(),
            Self::SketchedArt => preset("sketched art").unwrap(),
            Self::Silhouettes => preset("silhouettes").unwrap(),
            Self::LineArt => preset("line art").unwrap(),
            Self::TechnicalDrawing => preset("technical drawing").unwrap(),
        }
    }
}

pub const PRESET_NAMES: &[&str] = &[
    "Default",
    "High Fidelity Photo",
    "Low Fidelity Photo",
    "3 Colors",
    "6 Colors",
    "16 Colors",
    "Shades of Gray",
    "Black and White Logo",
    "Sketched Art",
    "Silhouettes",
    "Line Art",
    "Technical Drawing",
];

pub fn preset(name: &str) -> Option<TraceParams> {
    let key = name
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_ascii_lowercase();
    let d = TraceParams::default();
    let color = |colors: u32, paths: f64, corners: f64, noise: u32, method: Method| TraceParams {
        mode: Mode::Color,
        colors,
        paths,
        corners,
        noise,
        method,
        ..TraceParams::default()
    };
    let bw = |threshold: u8, paths: f64, corners: f64, noise: u32, snap: bool| TraceParams {
        mode: Mode::BlackAndWhite,
        threshold,
        paths,
        corners,
        noise,
        ignore_white: true,
        snap_curves_to_lines: snap,
        ..TraceParams::default()
    };
    Some(match key.as_str() {
        "default" => d,
        "high fidelity photo" => color(64, 90.0, 25.0, 4, Method::Overlapping),
        "low fidelity photo" => color(20, 60.0, 50.0, 12, Method::Overlapping),
        "3 colors" => color(3, 60.0, 60.0, 20, Method::Abutting),
        "6 colors" => color(6, 65.0, 60.0, 16, Method::Abutting),
        "16 colors" => color(16, 70.0, 55.0, 10, Method::Abutting),
        "shades of gray" => TraceParams {
            mode: Mode::Grayscale,
            colors: 8,
            paths: 60.0,
            corners: 50.0,
            noise: 10,
            method: Method::Overlapping,
            ..d
        },
        "black and white logo" => bw(128, 95.0, 80.0, 8, true),
        "sketched art" => bw(150, 50.0, 50.0, 100, false),
        "silhouettes" => bw(200, 40.0, 60.0, 30, false),
        "line art" => bw(128, 80.0, 70.0, 10, false),
        "technical drawing" => bw(128, 95.0, 90.0, 4, true),
        _ => return None,
    })
}

pub fn presets() -> Vec<(&'static str, TraceParams)> {
    PRESET_NAMES
        .iter()
        .filter_map(|n| Some((*n, preset(n)?)))
        .collect()
}

/// One traced vector shape.
#[derive(Clone, Debug, PartialEq)]
pub struct TracedPath {
    pub path: PathData,
    pub color: [u8; 3],
    pub pixels: usize,
    pub stroke: Option<f64>,
}

/// The complete trace result.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TraceResult {
    pub paths: Vec<TracedPath>,
    pub palette: Vec<[u8; 3]>,
}

impl TraceResult {
    pub fn anchor_count(&self) -> usize {
        self.paths.iter().map(|p| p.path.anchor_count()).sum()
    }
}

fn is_white(c: [u8; 3]) -> bool {
    c.iter().all(|&v| v >= 245)
}

/// High-level function tracing raw RGBA8 slice buffer into vector geometry.
pub fn trace_rgba(
    width: u32,
    height: u32,
    rgba: &[u8],
    options: &TraceOptions,
) -> Result<TraceResult, TraceError> {
    if width == 0 || height == 0 || rgba.len() != width as usize * height as usize * 4 {
        return Err(TraceError::Empty);
    }
    if u64::from(width) * u64::from(height) > MAX_PIXELS {
        return Err(TraceError::TooLarge {
            width,
            height,
            max: MAX_PIXELS >> 20,
        });
    }
    let raster = Raster::new(width, height, rgba.to_vec());
    trace_within(&raster, options, usize::MAX)
}

/// Trace `img` with `params`.
pub fn trace(img: &Raster, params: &TraceParams) -> TraceResult {
    trace_within(img, params, usize::MAX).unwrap_or_default()
}

/// Trace `img` with `params`, bounded by `max_anchors`.
pub fn trace_within(
    img: &Raster,
    params: &TraceParams,
    max_anchors: usize,
) -> Result<TraceResult, TraceError> {
    let (w, h) = (img.width as usize, img.height as usize);
    if w == 0 || h == 0 || img.rgba.len() != w * h * 4 {
        return Ok(TraceResult::default());
    }
    let mut q = quantize(img, params);
    denoise(&mut q.labels, w, h, params.noise as usize);
    let mut counts = vec![0usize; q.palette.len()];
    for &l in &q.labels {
        if let Some(c) = counts.get_mut(l as usize) {
            *c += 1;
        }
    }
    let mut order: Vec<usize> = (0..q.palette.len()).filter(|&i| counts[i] > 0).collect();
    order.sort_by(|a, b| counts[*b].cmp(&counts[*a]).then(a.cmp(b)));
    let mut rank = vec![usize::MAX; q.palette.len()];
    for (r, &i) in order.iter().enumerate() {
        rank[i] = r;
    }
    let layers: Vec<(usize, usize)> = order
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, ci)| !(params.ignore_white && is_white(q.palette[ci])))
        .collect();
    let job = Layers {
        q: &q,
        rank: &rank,
        params,
        opts: fit::FitOptions {
            polygon_tol: params.polygon_tolerance(),
            fit_tol: params.fit_tolerance(),
            corner_angle: params.corner_angle(),
            snap_lines: params.snap_curves_to_lines,
        },
        w,
        h,
        anchors: AtomicUsize::new(0),
        max_anchors,
    };
    let mut out = TraceResult::default();
    for (paths, &(_, ci)) in parallel_map(&layers, |&(r, ci)| job.layer(r, ci))
        .into_iter()
        .zip(&layers)
    {
        out.paths
            .extend(paths.ok_or(TraceError::TooComplex { max: max_anchors })?);
        let color = q.palette[ci];
        if !out.palette.contains(&color) {
            out.palette.push(color);
        }
    }
    Ok(out)
}

struct Layers<'a> {
    q: &'a Quantized,
    rank: &'a [usize],
    params: &'a TraceParams,
    opts: fit::FitOptions,
    w: usize,
    h: usize,
    anchors: AtomicUsize,
    max_anchors: usize,
}

impl Layers<'_> {
    fn layer(&self, r: usize, ci: usize) -> Option<Vec<TracedPath>> {
        let (params, opts) = (self.params, &self.opts);
        let color = *self.q.palette.get(ci)?;
        let overlapping = params.method == Method::Overlapping;
        let (w, h) = (self.w, self.h);
        let (thin, lines) = if params.strokes {
            centerline::lines(
                &self.q.labels,
                u16::try_from(ci).unwrap_or(TRANSPARENT),
                w,
                h,
                params.stroke_width,
                opts,
            )
        } else {
            (vec![], vec![])
        };
        let mask: Vec<bool> = self
            .q
            .labels
            .iter()
            .enumerate()
            .map(|(i, &l)| {
                l != TRANSPARENT
                    && !thin.get(i).copied().unwrap_or(false)
                    && if overlapping {
                        self.rank.get(l as usize).is_some_and(|&lr| lr >= r)
                    } else {
                        l as usize == ci
                    }
            })
            .collect();
        let area_stroke = (params.strokes && !params.fills).then_some(1.0);
        let min_hole = params.noise.max(1) as i64;
        let mut paths = vec![];
        let mut add = |path: PathData, pixels: usize, stroke: Option<f64>| {
            let n = path.anchor_count();
            if self
                .anchors
                .fetch_add(n, Ordering::Relaxed)
                .saturating_add(n)
                > self.max_anchors
            {
                return false;
            }
            paths.push(TracedPath {
                path,
                color,
                pixels,
                stroke,
            });
            true
        };
        for comp in trace_mask(&mask, w, h) {
            let Some(outer) = fit::fit_loop(&comp.outer, opts) else {
                continue;
            };
            let mut subs = vec![outer];
            for hole in &comp.holes {
                if hole.area2.abs() / 2 < min_hole && params.noise > 0 {
                    continue;
                }
                subs.extend(fit::fit_loop(hole, opts));
            }
            if !add(PathData::new(subs), comp.pixels, area_stroke) {
                return None;
            }
        }
        for line in lines {
            if !add(line.path, line.pixels, Some(line.width)) {
                return None;
            }
        }
        Some(paths)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn parallel_map<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let threads = std::thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(items.len());
    if threads <= 1 {
        return items.iter().map(f).collect();
    }
    let next = AtomicUsize::new(0);
    let mut done: Vec<(usize, R)> = std::thread::scope(|s| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                s.spawn(|| {
                    let mut mine = vec![];
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(item) = items.get(i) else { break };
                        mine.push((i, f(item)));
                    }
                    mine
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().unwrap_or_else(|e| std::panic::resume_unwind(e)))
            .collect()
    });
    done.sort_by_key(|(i, _)| *i);
    done.into_iter().map(|(_, r)| r).collect()
}

#[cfg(target_arch = "wasm32")]
fn parallel_map<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    items.iter().map(f).collect()
}

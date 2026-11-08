//! Layer mask density scaling, Gaussian mask feathering, clipping mask alpha propagation,
//! knockout (Shallow/Deep), and BlendIf luminosity range ramp evaluation.
//!
//! Grounded in UDHR Articles 12, 19, 27 and ICESCR Article 15.

use serde::{Deserialize, Serialize};

/// Scales layer mask density in-place or returns scaled copy.
/// `density` in `[0.0, 1.0]`:
/// - 1.0: mask applies normally (no change).
/// - 0.0: mask is fully revealed (all 255, no masking effect).
pub fn scale_mask_density(mask: &[u8], density: f32) -> Vec<u8> {
    let d = density.clamp(0.0, 1.0);
    if (d - 1.0).abs() < 1e-6 {
        return mask.to_vec();
    }
    mask.iter()
        .map(|&m| {
            // White (255) remains white; black (0) scales towards 255
            let v = f32::from(m);
            let scaled = 255.0 - (255.0 - v) * d;
            (scaled + 0.5).clamp(0.0, 255.0) as u8
        })
        .collect()
}

/// Applies separable 2D Gaussian feathering to an 8-bit mask buffer.
/// `radius` is the Gaussian standard deviation (sigma) in pixels.
pub fn feather_mask(mask: &[u8], width: usize, height: usize, radius: f32) -> Vec<u8> {
    if radius <= 0.0 || width == 0 || height == 0 || mask.len() != width * height {
        return mask.to_vec();
    }

    let k_radius = (radius * 3.0).ceil() as usize;
    if k_radius == 0 {
        return mask.to_vec();
    }

    // Precompute 1D Gaussian kernel
    let mut kernel = Vec::with_capacity(k_radius * 2 + 1);
    let mut k_sum = 0.0f32;
    let two_sigma_sq = 2.0 * radius * radius;
    for i in -(k_radius as isize)..=(k_radius as isize) {
        let w = (-((i * i) as f32) / two_sigma_sq).exp();
        kernel.push(w);
        k_sum += w;
    }
    for w in &mut kernel {
        *w /= k_sum;
    }

    // Horizontal pass: u8 -> f32
    let mut temp = vec![0.0f32; width * height];
    for y in 0..height {
        let row_off = y * width;
        for x in 0..width {
            let mut acc = 0.0f32;
            for (ki, &w) in kernel.iter().enumerate() {
                let offset = ki as isize - k_radius as isize;
                let sample_x = (x as isize + offset).clamp(0, width as isize - 1) as usize;
                acc += f32::from(mask[row_off + sample_x]) * w;
            }
            temp[row_off + x] = acc;
        }
    }

    // Vertical pass: f32 -> u8
    let mut output = vec![0u8; width * height];
    for x in 0..width {
        for y in 0..height {
            let mut acc = 0.0f32;
            for (ki, &w) in kernel.iter().enumerate() {
                let offset = ki as isize - k_radius as isize;
                let sample_y = (y as isize + offset).clamp(0, height as isize - 1) as usize;
                acc += temp[sample_y * width + x] * w;
            }
            output[y * width + x] = (acc + 0.5).clamp(0.0, 255.0) as u8;
        }
    }

    output
}

/// Propagates clipping mask base alpha onto a clipped layer's alpha buffer.
/// `clipped_alpha[i] = (clipped_alpha[i] * base_alpha[i]) / 255`.
pub fn propagate_clipping_mask_alpha(base_alpha: &[u8], clipped_alpha: &mut [u8]) {
    assert_eq!(base_alpha.len(), clipped_alpha.len());
    for (c, &b) in clipped_alpha.iter_mut().zip(base_alpha) {
        *c = ((u32::from(*c) * u32::from(b) + 127) / 255) as u8;
    }
}

/// Knockout modes in layer styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum KnockoutMode {
    #[default]
    None,
    /// Reveals the bottom of the enclosing group or clipping base.
    Shallow,
    /// Reveals the background layer or canvas transparency.
    Deep,
}

/// Evaluates knockout composition for a single straight-alpha RGBA pixel.
/// Premultiplied linear mix between backdrop, knockout target, and layer draw.
pub fn evaluate_knockout(
    mode: KnockoutMode,
    shape_coverage: f32,
    backdrop: [f32; 4],
    target: [f32; 4],
    fill_opacity: f32,
    layer_opacity: f32,
    layer_draw: [f32; 4],
) -> [f32; 4] {
    if mode == KnockoutMode::None || shape_coverage <= 0.0 {
        // Standard alpha over
        let k = layer_draw[3] * layer_opacity;
        return [
            layer_draw[0] * k + backdrop[0] * (1.0 - k),
            layer_draw[1] * k + backdrop[1] * (1.0 - k),
            layer_draw[2] * k + backdrop[2] * (1.0 - k),
            k + backdrop[3] * (1.0 - k),
        ];
    }

    let cov = shape_coverage.clamp(0.0, 1.0);
    // 1. Inside knockout shape, backdrop blends towards target
    let knocked_backdrop = [
        backdrop[0] * (1.0 - cov) + target[0] * cov,
        backdrop[1] * (1.0 - cov) + target[1] * cov,
        backdrop[2] * (1.0 - cov) + target[2] * cov,
        backdrop[3] * (1.0 - cov) + target[3] * cov,
    ];

    // 2. Layer draws onto knocked backdrop at fill opacity
    let fill = fill_opacity.clamp(0.0, 1.0) * layer_draw[3];
    let painted = [
        layer_draw[0] * fill + knocked_backdrop[0] * (1.0 - fill),
        layer_draw[1] * fill + knocked_backdrop[1] * (1.0 - fill),
        layer_draw[2] * fill + knocked_backdrop[2] * (1.0 - fill),
        fill + knocked_backdrop[3] * (1.0 - fill),
    ];

    // 3. Layer opacity blends between untouched backdrop and painted result
    let op = layer_opacity.clamp(0.0, 1.0);
    [
        painted[0] * op + backdrop[0] * (1.0 - op),
        painted[1] * op + backdrop[1] * (1.0 - op),
        painted[2] * op + backdrop[2] * (1.0 - op),
        painted[3] * op + backdrop[3] * (1.0 - op),
    ]
}

/// One Blend If slider pair: black = [low, high], white = [low, high] on 0..=255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlendRange {
    pub black: [u8; 2],
    pub white: [u8; 2],
}

impl Default for BlendRange {
    fn default() -> Self {
        Self::FULL
    }
}

impl BlendRange {
    pub const FULL: BlendRange = BlendRange {
        black: [0, 0],
        white: [255, 255],
    };

    pub fn is_full(&self) -> bool {
        *self == Self::FULL
    }

    pub fn from_bytes(b: [u8; 4]) -> Self {
        BlendRange {
            black: [b[0], b[1]],
            white: [b[2], b[3]],
        }
    }

    pub fn to_bytes(self) -> [u8; 4] {
        [self.black[0], self.black[1], self.white[0], self.white[1]]
    }

    /// Evaluates visibility weight (0.0..=1.0) for a sample `v` in `0.0..=255.0`.
    pub fn weight(&self, v: f32) -> f32 {
        let ramp = |lo: u8, hi: u8, x: f32| -> f32 {
            let (lo, hi) = (f32::from(lo), f32::from(hi));
            if hi <= lo {
                if x >= lo { 1.0 } else { 0.0 }
            } else {
                ((x - lo) / (hi - lo)).clamp(0.0, 1.0)
            }
        };
        let lo = ramp(self.black[0], self.black[1], v);
        let hi = ramp(255 - self.white[1], 255 - self.white[0], 255.0 - v);
        lo * hi
    }
}

/// Blend If settings: `ranges[0]` is composite gray, followed by color channel ranges.
/// Each entry is `[this_layer, underlying_layer]`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlendIf {
    pub ranges: Vec<[BlendRange; 2]>,
}

impl BlendIf {
    pub fn is_default(&self) -> bool {
        self.ranges.iter().flatten().all(BlendRange::is_full)
    }

    pub fn get(&self, i: usize) -> [BlendRange; 2] {
        self.ranges.get(i).copied().unwrap_or([BlendRange::FULL; 2])
    }

    pub fn set(&mut self, i: usize, pair: [BlendRange; 2]) {
        if self.ranges.len() <= i {
            self.ranges.resize(i + 1, [BlendRange::FULL; 2]);
        }
        self.ranges[i] = pair;
        while self.ranges.last().is_some_and(|p| p.iter().all(BlendRange::is_full)) {
            self.ranges.pop();
        }
    }

    /// Evaluates effective Blend If multiplier for a pixel.
    pub fn evaluate(
        &self,
        this_gray: f32,
        underlying_gray: f32,
        this_channels: &[f32],
        underlying_channels: &[f32],
    ) -> f32 {
        if self.is_default() {
            return 1.0;
        }

        let mut weight = 1.0f32;

        // Composite Gray
        let gray_pair = self.get(0);
        weight *= gray_pair[0].weight(this_gray * 255.0);
        weight *= gray_pair[1].weight(underlying_gray * 255.0);

        // Per-channel
        let num_ch = this_channels.len().min(underlying_channels.len());
        for c in 0..num_ch {
            let pair = self.get(c + 1);
            if !pair[0].is_full() {
                weight *= pair[0].weight(this_channels[c] * 255.0);
            }
            if !pair[1].is_full() {
                weight *= pair[1].weight(underlying_channels[c] * 255.0);
            }
        }

        weight.clamp(0.0, 1.0)
    }
}

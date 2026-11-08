//! Pure channel blend operators, HDRI operators, and W3C/PDF blend modes.
//!
//! Inputs are expected in the normalized channel range `[0, 1]` or HDR floating-point ranges.
//! This crate does not own persistent image formats, document state, or display pipelines.

/// Four operators with specific edge-tolerance behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlendOperator {
    ColorBurn,
    ColorDodge,
    VividLight,
    HardMix,
}

/// Near-zero/near-one tolerance used by burn, dodge, and generic vivid-light
/// edge handling.
pub const EDGE: f32 = 1e-4;

/// Evaluates one operator for one backdrop/source channel pair.
pub fn blend_channel(operator: BlendOperator, backdrop: f32, source: f32) -> f32 {
    match operator {
        BlendOperator::ColorBurn => color_burn(backdrop, source),
        BlendOperator::ColorDodge => color_dodge(backdrop, source),
        BlendOperator::VividLight => vivid_light(backdrop, source),
        BlendOperator::HardMix => {
            if vivid_light_generic(backdrop, source) >= 0.5 - 1e-6 {
                1.0
            } else {
                0.0
            }
        }
    }
}

/// Byte-quantized Color Burn operator in `0..=255`.
pub fn color_burn_u8(backdrop: u8, source: u8) -> u8 {
    let b = backdrop as f32 / 255.0;
    let s = source as f32 / 255.0;
    (color_burn(b, s) * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Byte-quantized Color Dodge operator in `0..=255`.
pub fn color_dodge_u8(backdrop: u8, source: u8) -> u8 {
    let b = backdrop as f32 / 255.0;
    let s = source as f32 / 255.0;
    (color_dodge(b, s) * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Byte-quantized Vivid Light operator in `0..=255`.
pub fn vivid_light_u8(backdrop: u8, source: u8) -> u8 {
    let b = backdrop as f32 / 255.0;
    let s = source as f32 / 255.0;
    (vivid_light(b, s) * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Byte-quantized Hard Mix operator in `0..=255`.
pub fn hard_mix_u8(backdrop: u8, source: u8) -> u8 {
    let b = backdrop as f32 / 255.0;
    let s = source as f32 / 255.0;
    if vivid_light_generic(b, s) >= 0.5 - 1e-6 {
        255
    } else {
        0
    }
}

/// Evaluates a `BlendOperator` independently for three RGB channels.
pub fn blend_operator_rgb(
    operator: BlendOperator,
    backdrop: [f32; 3],
    source: [f32; 3],
) -> [f32; 3] {
    std::array::from_fn(|index| blend_channel(operator, backdrop[index], source[index]))
}

// ---------------------------------------------------------------------------
// 16 W3C / PDF Blend Modes
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum BlendMode {
    #[default]
    Normal,
    Darken,
    Multiply,
    ColorBurn,
    Lighten,
    Screen,
    ColorDodge,
    Overlay,
    SoftLight,
    HardLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl BlendMode {
    /// In Transparency-panel order (grouped into 6 standard categories).
    pub const ALL: [BlendMode; 16] = [
        BlendMode::Normal,
        BlendMode::Darken,
        BlendMode::Multiply,
        BlendMode::ColorBurn,
        BlendMode::Lighten,
        BlendMode::Screen,
        BlendMode::ColorDodge,
        BlendMode::Overlay,
        BlendMode::SoftLight,
        BlendMode::HardLight,
        BlendMode::Difference,
        BlendMode::Exclusion,
        BlendMode::Hue,
        BlendMode::Saturation,
        BlendMode::Color,
        BlendMode::Luminosity,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Darken => "Darken",
            BlendMode::Multiply => "Multiply",
            BlendMode::ColorBurn => "Color Burn",
            BlendMode::Lighten => "Lighten",
            BlendMode::Screen => "Screen",
            BlendMode::ColorDodge => "Color Dodge",
            BlendMode::Overlay => "Overlay",
            BlendMode::SoftLight => "Soft Light",
            BlendMode::HardLight => "Hard Light",
            BlendMode::Difference => "Difference",
            BlendMode::Exclusion => "Exclusion",
            BlendMode::Hue => "Hue",
            BlendMode::Saturation => "Saturation",
            BlendMode::Color => "Color",
            BlendMode::Luminosity => "Luminosity",
        }
    }

    /// Index of the separator group (0..=5).
    pub fn group(self) -> u8 {
        match self {
            BlendMode::Normal => 0,
            BlendMode::Darken | BlendMode::Multiply | BlendMode::ColorBurn => 1,
            BlendMode::Lighten | BlendMode::Screen | BlendMode::ColorDodge => 2,
            BlendMode::Overlay | BlendMode::SoftLight | BlendMode::HardLight => 3,
            BlendMode::Difference | BlendMode::Exclusion => 4,
            _ => 5,
        }
    }

    /// Whether the mode works on each colour channel separately (all but Hue, Saturation, Color, Luminosity).
    pub fn is_separable(self) -> bool {
        self.group() != 5
    }

    /// Parse a label or identifier, case/space-insensitively.
    pub fn parse(s: &str) -> Option<Self> {
        let norm = |x: &str| x.to_ascii_lowercase().replace([' ', '_', '-'], "");
        let want = norm(s);
        Self::ALL.into_iter().find(|m| norm(m.label()) == want)
    }
}

/// Luminance weights of the non-separable modes (Hue, Saturation, Color, Luminosity).
pub const LUM: [f32; 3] = [0.3, 0.59, 0.11];

/// Luminance weights of opacity masks (Rec. 709).
pub const MASK_LUM: [f32; 3] = [0.2126, 0.7152, 0.0722];

/// Evaluates a `BlendMode` on straight (not premultiplied) RGB in `[0, 1]`.
pub fn blend_mode_rgb(mode: BlendMode, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    use BlendMode as B;
    let separable = |f: fn(f32, f32) -> f32| {
        [0, 1, 2].map(|i| f(backdrop[i].clamp(0.0, 1.0), source[i].clamp(0.0, 1.0)))
    };
    match mode {
        B::Normal => separable(|_, s| s),
        B::Multiply => separable(|b, s| b * s),
        B::Screen => separable(screen),
        B::Overlay => separable(|b, s| hard_light(s, b)),
        B::Darken => separable(f32::min),
        B::Lighten => separable(f32::max),
        B::ColorDodge => separable(|b, s| {
            if b <= 0.0 {
                0.0
            } else if s >= 1.0 {
                1.0
            } else {
                (b / (1.0 - s)).min(1.0)
            }
        }),
        B::ColorBurn => separable(|b, s| {
            if b >= 1.0 {
                1.0
            } else if s <= 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - b) / s).min(1.0)
            }
        }),
        B::HardLight => separable(hard_light),
        B::SoftLight => separable(|b, s| {
            if s <= 0.5 {
                b - (1.0 - 2.0 * s) * b * (1.0 - b)
            } else {
                let d = if b <= 0.25 {
                    ((16.0 * b - 12.0) * b + 4.0) * b
                } else {
                    b.sqrt()
                };
                b + (2.0 * s - 1.0) * (d - b)
            }
        }),
        B::Difference => separable(|b, s| (b - s).abs()),
        B::Exclusion => separable(|b, s| b + s - 2.0 * b * s),
        B::Hue => set_lum(set_sat(source, sat(backdrop)), lum(backdrop)),
        B::Saturation => set_lum(set_sat(backdrop, sat(source)), lum(backdrop)),
        B::Color => set_lum(source, lum(backdrop)),
        B::Luminosity => set_lum(backdrop, lum(source)),
    }
}

/// Unified trait allowing `blend_rgb` to accept either `BlendOperator` or `BlendMode`.
pub trait BlendTarget {
    fn apply_rgb(&self, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3];
}

impl BlendTarget for BlendOperator {
    fn apply_rgb(&self, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
        blend_operator_rgb(*self, backdrop, source)
    }
}

impl BlendTarget for BlendMode {
    fn apply_rgb(&self, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
        blend_mode_rgb(*self, backdrop, source)
    }
}

/// Evaluates an operator or blend mode independently across RGB channels.
pub fn blend_rgb<T: BlendTarget>(target: T, backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    target.apply_rgb(backdrop, source)
}

/// Source-over compositing of `source` onto `backdrop` with `mode` (straight RGBA in `0..=1`).
pub fn composite(mode: BlendMode, backdrop: [f32; 4], source: [f32; 4]) -> [f32; 4] {
    let (ab, as_) = (backdrop[3].clamp(0.0, 1.0), source[3].clamp(0.0, 1.0));
    let ao = as_ + ab * (1.0 - as_);
    if ao <= 0.0 {
        return [0.0; 4];
    }
    let mixed = blend_mode_rgb(
        mode,
        [backdrop[0], backdrop[1], backdrop[2]],
        [source[0], source[1], source[2]],
    );
    let c = |i: usize| {
        ((1.0 - ab) * source[i] + ab * mixed[i]) * as_ + (1.0 - as_) * ab * backdrop[i]
    };
    [c(0) / ao, c(1) / ao, c(2) / ao, ao]
}

/// Splits a CMYK colour into two additive planes for blending.
pub fn cmyk_planes([c, m, y, k]: [f32; 4]) -> [[f32; 3]; 2] {
    [[1.0 - c, 1.0 - m, 1.0 - y], [1.0 - k; 3]]
}

/// Combines blended CMY and K planes back into CMYK.
pub fn planes_cmyk(cmy: [f32; 3], k: f32) -> [f32; 4] {
    [1.0 - cmy[0], 1.0 - cmy[1], 1.0 - cmy[2], 1.0 - k].map(|v| v.clamp(0.0, 1.0))
}

// ---------------------------------------------------------------------------
// HDRI Floating-Point Channel and Pixel Operators
// ---------------------------------------------------------------------------

/// HDRI Linear Dodge (Additive): `backdrop + source`.
#[inline(always)]
pub fn linear_dodge_f32(backdrop: f32, source: f32) -> f32 {
    backdrop + source
}

/// HDRI Linear Burn: `backdrop + source - 1.0`.
#[inline(always)]
pub fn linear_burn_f32(backdrop: f32, source: f32) -> f32 {
    backdrop + source - 1.0
}

/// HDRI Linear Light: `backdrop + 2.0 * source - 1.0`.
#[inline(always)]
pub fn linear_light_f32(backdrop: f32, source: f32) -> f32 {
    backdrop + 2.0 * source - 1.0
}

/// HDRI Pin Light: conditional min/max against thresholded source.
#[inline(always)]
pub fn pin_light_f32(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        backdrop.min(2.0 * source)
    } else {
        backdrop.max(2.0 * source - 1.0)
    }
}

/// HDRI Pegtop Soft Light approximation.
pub fn pegtop_light_f32(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        backdrop - (1.0 - 2.0 * source) * backdrop * (1.0 - backdrop)
    } else {
        let root = if backdrop <= 0.0 {
            0.0
        } else {
            backdrop.sqrt()
        };
        backdrop + (2.0 * source - 1.0) * (root - backdrop)
    }
}

/// W3C/PDF Soft Light channel blend function.
pub fn soft_light_f32(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        backdrop - (1.0 - 2.0 * source) * backdrop * (1.0 - backdrop)
    } else {
        let d = if backdrop <= 0.25 {
            ((16.0 * backdrop - 12.0) * backdrop + 4.0) * backdrop
        } else if backdrop <= 0.0 {
            0.0
        } else {
            backdrop.sqrt()
        };
        backdrop + (2.0 * source - 1.0) * (d - backdrop)
    }
}

/// HDRI Exclusion: `backdrop + source - 2.0 * backdrop * source`.
#[inline(always)]
pub fn exclusion_f32(backdrop: f32, source: f32) -> f32 {
    backdrop + source - 2.0 * backdrop * source
}

/// HDRI Difference: `|backdrop - source|`.
#[inline(always)]
pub fn difference_f32(backdrop: f32, source: f32) -> f32 {
    (backdrop - source).abs()
}

/// HDRI Divide: `backdrop / source` (safeguarded against division by zero).
#[inline(always)]
pub fn divide_f32(backdrop: f32, source: f32) -> f32 {
    if source.abs() < 1e-12 {
        1.0
    } else {
        backdrop / source
    }
}

// HDRI RGB Pixel Helpers

pub fn linear_dodge_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| linear_dodge_f32(backdrop[i], source[i]))
}

pub fn linear_burn_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| linear_burn_f32(backdrop[i], source[i]))
}

pub fn linear_light_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| linear_light_f32(backdrop[i], source[i]))
}

pub fn pin_light_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| pin_light_f32(backdrop[i], source[i]))
}

pub fn pegtop_light_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| pegtop_light_f32(backdrop[i], source[i]))
}

pub fn soft_light_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| soft_light_f32(backdrop[i], source[i]))
}

pub fn exclusion_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| exclusion_f32(backdrop[i], source[i]))
}

pub fn difference_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| difference_f32(backdrop[i], source[i]))
}

pub fn divide_rgb(backdrop: [f32; 3], source: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| divide_f32(backdrop[i], source[i]))
}

// ---------------------------------------------------------------------------
// Internal Helper Functions
// ---------------------------------------------------------------------------

fn vivid_light(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        if source <= 0.0 {
            0.0
        } else {
            1.0 - ((1.0 - backdrop) / (2.0 * source)).min(1.0)
        }
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (2.0 * (1.0 - source))).min(1.0)
    }
}

fn color_burn(backdrop: f32, source: f32) -> f32 {
    if backdrop >= 1.0 - EDGE {
        1.0
    } else if source <= 0.0 {
        0.0
    } else {
        1.0 - ((1.0 - backdrop) / source).min(1.0)
    }
}

fn color_dodge(backdrop: f32, source: f32) -> f32 {
    if backdrop <= EDGE {
        0.0
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (1.0 - source)).min(1.0)
    }
}

fn vivid_light_generic(backdrop: f32, source: f32) -> f32 {
    if source <= 0.5 {
        if backdrop >= 1.0 - EDGE {
            1.0
        } else if source <= 0.0 {
            0.0
        } else {
            1.0 - ((1.0 - backdrop) / (2.0 * source)).min(1.0)
        }
    } else if backdrop <= EDGE {
        0.0
    } else if source >= 1.0 {
        1.0
    } else {
        (backdrop / (2.0 * (1.0 - source))).min(1.0)
    }
}

fn screen(b: f32, s: f32) -> f32 {
    b + s - b * s
}

fn hard_light(b: f32, s: f32) -> f32 {
    if s <= 0.5 {
        b * 2.0 * s
    } else {
        screen(b, 2.0 * s - 1.0)
    }
}

fn lum(c: [f32; 3]) -> f32 {
    LUM[0] * c[0] + LUM[1] * c[1] + LUM[2] * c[2]
}

fn sat(c: [f32; 3]) -> f32 {
    c[0].max(c[1]).max(c[2]) - c[0].min(c[1]).min(c[2])
}

fn clip_color(c: [f32; 3]) -> [f32; 3] {
    let l = lum(c);
    let (n, x) = (c[0].min(c[1]).min(c[2]), c[0].max(c[1]).max(c[2]));
    let mut c = c;
    if n < 0.0 {
        c = if l - n > f32::EPSILON {
            c.map(|v| l + (v - l) * l / (l - n))
        } else {
            c.map(|v| v.max(0.0))
        };
    }
    if x > 1.0 {
        c = if x - l > f32::EPSILON {
            c.map(|v| l + (v - l) * (1.0 - l) / (x - l))
        } else {
            c.map(|v| v.min(1.0))
        };
    }
    c
}

fn set_lum(c: [f32; 3], l: f32) -> [f32; 3] {
    let d = l - lum(c);
    clip_color(c.map(|v| v + d))
}

fn set_sat(c: [f32; 3], s: f32) -> [f32; 3] {
    let (n, x) = (c[0].min(c[1]).min(c[2]), c[0].max(c[1]).max(c[2]));
    if x <= n {
        return [0.0; 3];
    }
    c.map(|v| (v - n) * s / (x - n))
}

#[cfg(test)]
mod migration_oracle;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vivid_light_source_extremes_win() {
        assert_eq!(blend_channel(BlendOperator::VividLight, 0.0, 1.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::VividLight, 1.0, 0.0), 0.0);
        assert!((blend_channel(BlendOperator::VividLight, 0.5, 0.75) - 1.0).abs() < 1e-6);
        assert!((blend_channel(BlendOperator::VividLight, 0.5, 0.25) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn hard_mix_uses_generic_vivid_light_edge_rules() {
        assert_eq!(blend_channel(BlendOperator::HardMix, 1.0, 0.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.0, 1.0), 0.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.6, 0.6), 1.0);
        assert_eq!(blend_channel(BlendOperator::HardMix, 0.3, 0.3), 0.0);
    }

    #[test]
    fn burn_and_dodge_treat_near_extremes_as_exact() {
        assert_eq!(blend_channel(BlendOperator::ColorBurn, 1.0 - 1e-6, 0.0), 1.0);
        assert_eq!(blend_channel(BlendOperator::ColorDodge, 1e-6, 1.0), 0.0);
        assert!((blend_channel(BlendOperator::ColorBurn, 0.5, 0.5)).abs() < 1e-6);
        assert!((blend_channel(BlendOperator::ColorDodge, 0.25, 0.5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn u8_operators_behave_consistently() {
        assert_eq!(color_burn_u8(255, 0), 255);
        assert_eq!(color_burn_u8(128, 128), 0);
        assert_eq!(color_dodge_u8(0, 255), 0);
        assert_eq!(vivid_light_u8(0, 255), 255);
        assert_eq!(vivid_light_u8(255, 0), 0);
        assert_eq!(hard_mix_u8(255, 0), 255);
        assert_eq!(hard_mix_u8(0, 255), 0);
    }

    #[test]
    fn hdri_operators_produce_expected_values() {
        assert_eq!(linear_dodge_f32(0.5, 0.7), 1.2);
        assert_eq!(linear_burn_f32(0.8, 0.5), 0.3);
        assert_eq!(linear_light_f32(0.4, 0.6), 0.6);
        assert_eq!(pin_light_f32(0.3, 0.2), 0.3);
        assert_eq!(exclusion_f32(0.4, 0.6), 0.4 + 0.6 - 2.0 * 0.24);
        assert_eq!(difference_f32(0.2, 0.7), 0.5);
        assert!((divide_f32(0.5, 2.0) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn separable_modes_follow_the_reference_formulas() {
        let (b, s) = ([0.2, 0.5, 0.9], [0.6, 0.25, 0.5]);
        let near = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5);
        let cases: [(BlendMode, [f32; 3]); 12] = [
            (BlendMode::Normal, s),
            (BlendMode::Multiply, [0.12, 0.125, 0.45]),
            (BlendMode::Screen, [0.68, 0.625, 0.95]),
            (BlendMode::Darken, [0.2, 0.25, 0.5]),
            (BlendMode::Lighten, [0.6, 0.5, 0.9]),
            (BlendMode::Difference, [0.4, 0.25, 0.4]),
            (BlendMode::Exclusion, [0.56, 0.5, 0.5]),
            (BlendMode::Overlay, [0.24, 0.25, 0.9]),
            (BlendMode::HardLight, [0.36, 0.25, 0.9]),
            (BlendMode::ColorDodge, [0.5, 0.6666667, 1.0]),
            (BlendMode::ColorBurn, [0.0, 0.0, 0.8]),
            (
                BlendMode::SoftLight,
                [
                    0.2 + 0.2 * (((16.0 * 0.2 - 12.0) * 0.2 + 4.0) * 0.2 - 0.2),
                    0.375,
                    0.9,
                ],
            ),
        ];
        for (m, want) in cases {
            assert!(
                near(blend_mode_rgb(m, b, s), want),
                "{m:?}: {:?} ≠ {want:?}",
                blend_mode_rgb(m, b, s)
            );
            assert!(
                near(blend_rgb(m, b, s), want),
                "generic blend_rgb {m:?}: {:?} ≠ {want:?}",
                blend_rgb(m, b, s)
            );
        }
    }

    #[test]
    fn parse_labels_round_trip() {
        assert_eq!(BlendMode::parse("color burn"), Some(BlendMode::ColorBurn));
        assert_eq!(BlendMode::parse("ColorBurn"), Some(BlendMode::ColorBurn));
        assert_eq!(BlendMode::parse("soft_light"), Some(BlendMode::SoftLight));
        assert_eq!(BlendMode::parse("nope"), None);
        for m in BlendMode::ALL {
            assert_eq!(BlendMode::parse(m.label()), Some(m));
        }
    }
}

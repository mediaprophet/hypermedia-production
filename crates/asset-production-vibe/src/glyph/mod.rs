//! Deterministic, bounded, zero-external-font-file vector glyph outline & MSDF metric shaping.
//! Provides proportional typography, pair kerning, greedy word wrapping, SVG path emission,
//! standalone SVG projection, and straight-alpha RGBA rasterization.

mod shaping_svg;
mod stroke_table;

pub use shaping_svg::shape_glyph_run;

use std::fmt::Write;

/// Retained font metric ratios normalized to the 1.0 em-box.
#[derive(Clone, Copy, Debug)]
pub struct FontMetrics {
    pub em_size: f64,
    pub ascender: f64,
    pub cap_height: f64,
    pub x_height: f64,
    pub baseline: f64,
    pub descender: f64,
    pub line_height: f64,
}

impl Default for FontMetrics {
    fn default() -> Self {
        Self {
            em_size: 1.0,
            ascender: 0.12,
            cap_height: 0.16,
            x_height: 0.38,
            baseline: 0.78,
            descender: 0.96,
            line_height: 1.25,
        }
    }
}

/// A fully shaped, measured, and positioned vector glyph run.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ShapedGlyphRun {
    pub text: String,
    pub font_size: f64,
    pub line_height: f64,
    pub tracking: f64,
    pub width: f64,
    pub height: f64,
    pub line_count: usize,
    pub glyph_count: usize,
    pub svg_path_d: String,
}

/// Proportional advance width in normalized em units (1.0 = em height).
#[inline]
pub fn glyph_advance(ch: char) -> f64 {
    match ch {
        ' ' => 0.30,
        '.' | ',' | ':' | ';' | '\'' | '`' | '|' => 0.25,
        '!' => 0.28,
        'i' | 'l' => 0.25,
        'j' => 0.28,
        'f' | 't' => 0.35,
        'r' => 0.38,
        '(' | ')' | '[' | ']' => 0.32,
        '{' | '}' => 0.35,
        '-' => 0.36,
        '"' => 0.38,
        '*' => 0.40,
        '/' | '\\' => 0.45,
        '^' => 0.45,
        '<' | '>' => 0.48,
        'c' | 'k' | 'v' | 'x' | 'y' => 0.48,
        's' | 'z' => 0.46,
        'a' | 'e' => 0.50,
        'b' | 'd' | 'g' | 'h' | 'n' | 'o' | 'p' | 'q' | 'u' => 0.52,
        '~' | '_' => 0.50,
        '?' => 0.48,
        '0'..='9' => 0.55,
        '+' | '=' => 0.55,
        'I' => 0.28,
        'J' => 0.40,
        'L' => 0.48,
        'E' | 'F' | 'S' | 'Z' => 0.56,
        'B' | 'K' | 'P' | 'R' | 'X' | 'Y' => 0.60,
        'C' => 0.62,
        'D' | 'G' | 'H' | 'N' | 'U' | 'V' => 0.64,
        'A' => 0.65,
        'O' | 'Q' => 0.66,
        'T' => 0.58,
        'm' => 0.78,
        'w' => 0.72,
        'M' => 0.80,
        'W' => 0.85,
        '#' | '$' => 0.58,
        '%' => 0.70,
        '&' => 0.65,
        '@' => 0.75,
        'α' => 0.52,
        'μ' => 0.54,
        'σ' => 0.52,
        'Δ' => 0.65,
        'Ω' => 0.66,
        'π' => 0.54,
        '±' => 0.55,
        '°' => 0.35,
        '→' => 0.70,
        '↔' => 0.75,
        '≤' | '≥' => 0.55,
        '×' => 0.50,
        '·' => 0.28,
        '—' => 0.75,
        _ => 0.55,
    }
}

/// Pair kerning adjustment in normalized em units.
#[inline]
pub fn pair_kerning(prev: char, curr: char) -> f64 {
    match (prev, curr) {
        ('A', 'V') => -0.10,
        ('A', 'W') => -0.08,
        ('A', 'Y') => -0.10,
        ('A', 'T') => -0.09,
        ('V', 'A') => -0.10,
        ('W', 'A') => -0.08,
        ('Y', 'A') => -0.10,
        ('T', 'A') => -0.09,
        ('T', 'o') => -0.08,
        ('T', 'e') => -0.08,
        ('T', 'a') => -0.07,
        ('V', 'o') => -0.07,
        ('V', 'e') => -0.07,
        ('L', 'T') => -0.09,
        ('L', 'V') => -0.08,
        _ => 0.0,
    }
}

pub(super) struct SubpathBuilder<'a> {
    out: &'a mut String,
    x: f64,
    y: f64,
    scale: f64,
}

impl<'a> SubpathBuilder<'a> {
    #[inline]
    pub(super) fn new(out: &'a mut String, x: f64, y: f64, scale: f64) -> Self {
        Self { out, x, y, scale }
    }

    #[inline]
    fn pt(&self, gx: f64, gy: f64) -> (f64, f64) {
        (self.x + gx * self.scale, self.y + gy * self.scale)
    }

    pub(super) fn stroke(&mut self, pts: &[(f64, f64)]) {
        if pts.is_empty() {
            return;
        }
        let (p0x, p0y) = self.pt(pts[0].0, pts[0].1);
        let _ = write!(self.out, "M{:.2} {:.2}", p0x, p0y);
        for pt in &pts[1..] {
            let (px, py) = self.pt(pt.0, pt.1);
            let _ = write!(self.out, " L{:.2} {:.2}", px, py);
        }
        self.out.push(' ');
    }

    pub(super) fn closed(&mut self, pts: &[(f64, f64)]) {
        if pts.is_empty() {
            return;
        }
        let (p0x, p0y) = self.pt(pts[0].0, pts[0].1);
        let _ = write!(self.out, "M{:.2} {:.2}", p0x, p0y);
        for pt in &pts[1..] {
            let (px, py) = self.pt(pt.0, pt.1);
            let _ = write!(self.out, " L{:.2} {:.2}", px, py);
        }
        self.out.push_str(" Z ");
    }
}

pub(super) const Y_TOP: f64 = 0.12;
pub(super) const Y_CAP: f64 = 0.16;
pub(super) const Y_X: f64 = 0.38;
pub(super) const Y_MID: f64 = 0.47;
pub(super) const Y_XMID: f64 = 0.58;
pub(super) const Y_BASE: f64 = 0.78;
pub(super) const Y_DESC: f64 = 0.96;

/// Emits the clean vector strokes for `ch` into `out` as SVG path instructions `M ... L ...`.
pub fn char_to_svg_subpath(ch: char, x: f64, y: f64, scale: f64, out: &mut String) {
    if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
        return;
    }

    let adv = glyph_advance(ch);
    let xl = adv * 0.12;
    let xr = adv * 0.88;
    let xm = (xl + xr) * 0.5;
    let w = xr - xl;

    let mut b = SubpathBuilder::new(out, x, y, scale);

    match ch {
        'A' => {
            b.stroke(&[(xl, Y_BASE), (xm, Y_CAP), (xr, Y_BASE)]);
            b.stroke(&[(xl + w * 0.22, Y_MID + 0.08), (xr - w * 0.22, Y_MID + 0.08)]);
        }
        'B' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[
                (xl, Y_CAP),
                (xr - w * 0.18, Y_CAP),
                (xr, Y_CAP + (Y_MID - Y_CAP) * 0.5),
                (xr - w * 0.18, Y_MID),
                (xl, Y_MID),
            ]);
            b.stroke(&[
                (xl, Y_MID),
                (xr - w * 0.12, Y_MID),
                (xr, Y_MID + (Y_BASE - Y_MID) * 0.5),
                (xr - w * 0.12, Y_BASE),
                (xl, Y_BASE),
            ]);
        }
        'C' => {
            b.stroke(&[
                (xr, Y_CAP + 0.12),
                (xr - w * 0.2, Y_CAP),
                (xl + w * 0.2, Y_CAP),
                (xl, Y_CAP + 0.15),
                (xl, Y_BASE - 0.15),
                (xl + w * 0.2, Y_BASE),
                (xr - w * 0.2, Y_BASE),
                (xr, Y_BASE - 0.12),
            ]);
        }
        'D' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[
                (xl, Y_CAP),
                (xr - w * 0.2, Y_CAP),
                (xr, Y_CAP + 0.18),
                (xr, Y_BASE - 0.18),
                (xr - w * 0.2, Y_BASE),
                (xl, Y_BASE),
            ]);
        }
        'E' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP)]);
            b.stroke(&[(xl, Y_MID), (xr - w * 0.2, Y_MID)]);
            b.stroke(&[(xl, Y_BASE), (xr, Y_BASE)]);
        }
        'F' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP)]);
            b.stroke(&[(xl, Y_MID), (xr - w * 0.25, Y_MID)]);
        }
        'G' => {
            b.stroke(&[
                (xr, Y_CAP + 0.12),
                (xr - w * 0.2, Y_CAP),
                (xl + w * 0.2, Y_CAP),
                (xl, Y_CAP + 0.15),
                (xl, Y_BASE - 0.15),
                (xl + w * 0.2, Y_BASE),
                (xr, Y_BASE),
                (xr, Y_MID),
                (xm, Y_MID),
            ]);
        }
        'H' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[(xr, Y_CAP), (xr, Y_BASE)]);
            b.stroke(&[(xl, Y_MID), (xr, Y_MID)]);
        }
        'I' => {
            b.stroke(&[(xm, Y_CAP), (xm, Y_BASE)]);
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP)]);
            b.stroke(&[(xl, Y_BASE), (xr, Y_BASE)]);
        }
        'J' => {
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP)]);
            b.stroke(&[
                (xr - w * 0.2, Y_CAP),
                (xr - w * 0.2, Y_BASE - 0.12),
                (xm, Y_BASE),
                (xl, Y_BASE - 0.08),
            ]);
        }
        'K' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[(xr, Y_CAP), (xl, Y_MID)]);
            b.stroke(&[(xl + w * 0.3, Y_MID - 0.04), (xr, Y_BASE)]);
        }
        'L' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE), (xr, Y_BASE)]);
        }
        'M' => {
            b.stroke(&[
                (xl, Y_BASE),
                (xl, Y_CAP),
                (xm, Y_BASE - 0.08),
                (xr, Y_CAP),
                (xr, Y_BASE),
            ]);
        }
        'N' => {
            b.stroke(&[(xl, Y_BASE), (xl, Y_CAP), (xr, Y_BASE), (xr, Y_CAP)]);
        }
        'O' => {
            b.closed(&[
                (xm, Y_CAP),
                (xl + w * 0.15, Y_CAP + 0.05),
                (xl, Y_CAP + 0.2),
                (xl, Y_BASE - 0.2),
                (xl + w * 0.15, Y_BASE - 0.05),
                (xm, Y_BASE),
                (xr - w * 0.15, Y_BASE - 0.05),
                (xr, Y_BASE - 0.2),
                (xr, Y_CAP + 0.2),
                (xr - w * 0.15, Y_CAP + 0.05),
            ]);
        }
        'P' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[
                (xl, Y_CAP),
                (xr - w * 0.15, Y_CAP),
                (xr, Y_CAP + (Y_MID - Y_CAP) * 0.5),
                (xr - w * 0.15, Y_MID),
                (xl, Y_MID),
            ]);
        }
        'Q' => {
            b.closed(&[
                (xm, Y_CAP),
                (xl + w * 0.15, Y_CAP + 0.05),
                (xl, Y_CAP + 0.2),
                (xl, Y_BASE - 0.2),
                (xl + w * 0.15, Y_BASE - 0.05),
                (xm, Y_BASE),
                (xr - w * 0.15, Y_BASE - 0.05),
                (xr, Y_BASE - 0.2),
                (xr, Y_CAP + 0.2),
                (xr - w * 0.15, Y_CAP + 0.05),
            ]);
            b.stroke(&[(xm + w * 0.05, Y_BASE - 0.14), (xr + w * 0.1, Y_BASE + 0.08)]);
        }
        'R' => {
            b.stroke(&[(xl, Y_CAP), (xl, Y_BASE)]);
            b.stroke(&[
                (xl, Y_CAP),
                (xr - w * 0.15, Y_CAP),
                (xr, Y_CAP + (Y_MID - Y_CAP) * 0.5),
                (xr - w * 0.15, Y_MID),
                (xl, Y_MID),
            ]);
            b.stroke(&[(xl + w * 0.35, Y_MID), (xr, Y_BASE)]);
        }
        'S' => {
            b.stroke(&[
                (xr, Y_CAP + 0.1),
                (xr - w * 0.2, Y_CAP),
                (xl + w * 0.2, Y_CAP),
                (xl, Y_CAP + 0.12),
                (xm, Y_MID),
                (xr, Y_MID + 0.12),
                (xr, Y_BASE - 0.12),
                (xr - w * 0.2, Y_BASE),
                (xl + w * 0.2, Y_BASE),
                (xl, Y_BASE - 0.1),
            ]);
        }
        'T' => {
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP)]);
            b.stroke(&[(xm, Y_CAP), (xm, Y_BASE)]);
        }
        'U' => {
            b.stroke(&[
                (xl, Y_CAP),
                (xl, Y_BASE - 0.18),
                (xl + w * 0.2, Y_BASE),
                (xr - w * 0.2, Y_BASE),
                (xr, Y_BASE - 0.18),
                (xr, Y_CAP),
            ]);
        }
        'V' => {
            b.stroke(&[(xl, Y_CAP), (xm, Y_BASE), (xr, Y_CAP)]);
        }
        'W' => {
            b.stroke(&[
                (xl, Y_CAP),
                (xl + w * 0.25, Y_BASE),
                (xm, Y_CAP + 0.2),
                (xr - w * 0.25, Y_BASE),
                (xr, Y_CAP),
            ]);
        }
        'X' => {
            b.stroke(&[(xl, Y_CAP), (xr, Y_BASE)]);
            b.stroke(&[(xr, Y_CAP), (xl, Y_BASE)]);
        }
        'Y' => {
            b.stroke(&[(xl, Y_CAP), (xm, Y_MID)]);
            b.stroke(&[(xr, Y_CAP), (xm, Y_MID)]);
            b.stroke(&[(xm, Y_MID), (xm, Y_BASE)]);
        }
        'Z' => {
            b.stroke(&[(xl, Y_CAP), (xr, Y_CAP), (xl, Y_BASE), (xr, Y_BASE)]);
        }
        _ => stroke_table::char_to_svg_subpath_ext(ch, &mut b, xl, xm, xr, w),
    }
}

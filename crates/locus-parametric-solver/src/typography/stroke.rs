//! Single-stroke CAM/plotter stick font polyline evaluator.
//!
//! Each glyph is `(width, strokes)`. A stroke is a run of points; each point is two characters:
//! an x digit `0..=6` and a y letter where `a` = -2, `c` = 0 (baseline), `g` = 4 (x-height),
//! `i` = 6 (cap height). Strokes are separated by spaces.

#![forbid(unsafe_code)]

use drafting_geometry::Vec2;

pub const CAP: f64 = 6.0;
pub const GAP: f64 = 1.6;

pub fn glyph(c: char) -> Option<(f64, &'static str)> {
    let g: (f64, &str) = match c {
        ' ' => (2.4, ""),
        'A' => (4.0, "0c2i4c 1f3f"),
        'B' => (4.0, "0c0i3i4h4g3f0f 3f4e4d3c0c"),
        'C' => (4.0, "4h3i1i0h0d1c3c4d"),
        'D' => (4.0, "0c0i3i4h4d3c0c"),
        'E' => (4.0, "4i0i0c4c 0f3f"),
        'F' => (4.0, "4i0i0c 0f3f"),
        'G' => (4.0, "4h3i1i0h0d1c3c4d4f2f"),
        'H' => (4.0, "0c0i 4c4i 0f4f"),
        'I' => (2.0, "0i2i 1i1c 0c2c"),
        'J' => (4.0, "4i4d3c1c0d0e"),
        'K' => (4.0, "0c0i 4i0e 1f4c"),
        'L' => (4.0, "0i0c4c"),
        'M' => (4.0, "0c0i2f4i4c"),
        'N' => (4.0, "0c0i4c4i"),
        'O' => (4.0, "1c0d0h1i3i4h4d3c1c"),
        'P' => (4.0, "0c0i3i4h4g3f0f"),
        'Q' => (4.0, "1c0d0h1i3i4h4d3c1c 2e4b"),
        'R' => (4.0, "0c0i3i4h4g3f0f 2f4c"),
        'S' => (4.0, "4h3i1i0h0g1f3f4e4d3c1c0d"),
        'T' => (4.0, "0i4i 2i2c"),
        'U' => (4.0, "0i0d1c3c4d4i"),
        'V' => (4.0, "0i2c4i"),
        'W' => (4.0, "0i1c2f3c4i"),
        'X' => (4.0, "0i4c 0c4i"),
        'Y' => (4.0, "0i2f4i 2f2c"),
        'Z' => (4.0, "0i4i0c4c"),
        '0' => (4.0, "1c0d0h1i3i4h4d3c1c"),
        '1' => (4.0, "1h2i2c 1c3c"),
        '2' => (4.0, "0h1i3i4h4g0c4c"),
        '3' => (4.0, "0h1i3i4h4g3f2f 3f4e4d3c1c0d"),
        '4' => (4.0, "3c3i0e4e"),
        '5' => (4.0, "4i0i0f3f4e4d3c1c0d"),
        '6' => (4.0, "4h3i1i0h0d1c3c4d4e3f1f0e"),
        '7' => (4.0, "0i4i1c"),
        '8' => (4.0, "1f0g0h1i3i4h4g3f1f0e0d1c3c4d4e3f"),
        '9' => (4.0, "4f1f0g0h1i3i4h4d3c1c0d"),
        'a' => (4.0, "4g4c 4f3g1g0f0d1c3c4d"),
        'b' => (4.0, "0i0c 0d1c3c4d4f3g1g0f"),
        'c' => (4.0, "4f3g1g0f0d1c3c4d"),
        'd' => (4.0, "4i4c 4d3c1c0d0f1g3g4f"),
        'e' => (4.0, "0e4e4f3g1g0f0d1c3c4d"),
        'f' => (3.0, "3i2i1h1c 0g3g"),
        'g' => (4.0, "4g4b3a1a0b 4f3g1g0f0d1c3c4d"),
        'h' => (4.0, "0i0c 0f1g3g4f4c"),
        'i' => (1.0, "0c0g 0h0i"),
        'j' => (2.0, "2g2b1a0a 2h2i"),
        'k' => (4.0, "0i0c 3g0d 1e3c"),
        'l' => (2.0, "1i1d2c"),
        'm' => (4.0, "0c0g 0f1g2f2c 2f3g4f4c"),
        'n' => (4.0, "0c0g 0f1g3g4f4c"),
        'o' => (4.0, "1c0d0f1g3g4f4d3c1c"),
        'p' => (4.0, "0g0a 0f1g3g4f4d3c1c0d"),
        'q' => (4.0, "4g4a 4f3g1g0f0d1c3c4d"),
        'r' => (3.0, "0c0g 0e2g3g"),
        's' => (4.0, "4f3g1g0f1e3e4d3c1c0d"),
        't' => (3.0, "1i1d2c3c 0g3g"),
        'u' => (4.0, "0g0d1c3c4d 4g4c"),
        'v' => (4.0, "0g2c4g"),
        'w' => (4.0, "0g1c2f3c4g"),
        'x' => (4.0, "0g4c 0c4g"),
        'y' => (4.0, "0g2c 4g1a0a"),
        'z' => (4.0, "0g4g0c4c"),
        '!' => (1.0, "0i0e 0d0c"),
        '"' => (2.0, "0i0g 2i2g"),
        '#' => (4.0, "1c1i 3c3i 0e4e 0g4g"),
        '$' => (4.0, "4h3i1i0h0g1f3f4e4d3c1c0d 2i2c"),
        '%' => (4.0, "0c4i 0i0h1h1i0i 3c3d4d4c3c"),
        '&' => (4.0, "4c1h1i2i3h0e0d1c2c4e"),
        '\'' => (1.0, "0i0g"),
        '(' => (2.0, "2i1h0f0e1d2c"),
        ')' => (2.0, "0i1h2f2e1d0c"),
        '*' => (4.0, "2h2d 0g4e 0e4g"),
        '+' => (4.0, "2d2h 0f4f"),
        ',' => (1.0, "1d1c0b"),
        '-' => (4.0, "0f4f"),
        '.' => (1.0, "0c0d"),
        '/' => (4.0, "0c4i"),
        ':' => (1.0, "0c0d 0g0h"),
        ';' => (1.0, "1d1c0b 1g1h"),
        '<' => (4.0, "4h0f4d"),
        '=' => (4.0, "0e4e 0g4g"),
        '>' => (4.0, "0h4f0d"),
        '?' => (4.0, "0h1i3i4h4g2f2e 2d2c"),
        '@' => (4.0, "3e3g1g1e3e4f4h3i1i0h0d1c4c"),
        '[' => (2.0, "2i0i0c2c"),
        '\\' => (4.0, "0i4c"),
        ']' => (2.0, "0i2i2c0c"),
        '^' => (4.0, "0g2i4g"),
        '_' => (4.0, "0c4c"),
        '`' => (1.0, "0i1h"),
        '{' => (2.0, "2i1h1f0e1d1c2c"),
        '|' => (1.0, "0i0c"),
        '}' => (2.0, "0i1h1f2e1d1c0c"),
        '~' => (4.0, "0f1g3e4f"),
        '°' => (2.0, "0h0i1i1h0h"),
        '±' => (4.0, "2d2h 0f4f 0b4b"),
        '⌀' => (4.0, "1c0d0h1i3i4h4d3c1c 0b4i"),
        _ => return None,
    };
    Some(g)
}

pub fn char_advance(c: char) -> f64 {
    match glyph(c) {
        Some((w, _)) => (w + GAP) / CAP,
        None if c.is_whitespace() => (2.4 + GAP) / CAP,
        None => (4.0 + GAP) / CAP,
    }
}

pub fn decode_stroke(def: &str) -> Vec<Vec<Vec2>> {
    let mut polylines = Vec::new();
    for run in def.split_whitespace() {
        let mut pts = Vec::new();
        let bytes = run.as_bytes();
        let mut i = 0;
        while i + 1 < bytes.len() {
            let x_code = bytes[i];
            let y_code = bytes[i + 1];
            let x = if x_code.is_ascii_digit() { (x_code - b'0') as f64 } else { 0.0 };
            let y = if y_code >= b'a' && y_code <= b'z' {
                (y_code - b'c') as f64
            } else {
                0.0
            };
            pts.push(Vec2::new(x, y));
            i += 2;
        }
        if !pts.is_empty() {
            polylines.push(pts);
        }
    }
    polylines
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StrokeLayout {
    pub strokes: Vec<Vec<Vec2>>,
    pub width: f64,
}

pub fn layout_stroke_line(
    s: &str,
    height: f64,
    width_factor: f64,
    oblique_rad: f64,
) -> StrokeLayout {
    let h = if height.is_finite() && height > 0.0 { height } else { 1.0 };
    let wf = if width_factor.is_finite() && width_factor.abs() > 1e-6 { width_factor } else { 1.0 };
    let sc = h / CAP;
    let shear = oblique_rad.tan().clamp(-10.0, 10.0);
    let mut x_cursor = 0.0;
    let mut all_strokes = Vec::new();

    for c in s.chars() {
        let (gw, def) = match glyph(c) {
            Some(pair) => pair,
            None if c.is_whitespace() => {
                x_cursor += char_advance(c) * h * wf;
                continue;
            }
            None => (4.0, "0c0i4i4c0c"), // box fallback
        };

        let raw_strokes = decode_stroke(def);
        for stroke in raw_strokes {
            let transformed: Vec<Vec2> = stroke
                .into_iter()
                .map(|p| {
                    let px = (p.x * sc) * wf;
                    let py = p.y * sc;
                    Vec2::new(x_cursor + px + py * shear, py)
                })
                .collect();
            all_strokes.push(transformed);
        }
        x_cursor += ((gw + GAP) / CAP) * h * wf;
    }

    StrokeLayout {
        strokes: all_strokes,
        width: x_cursor,
    }
}

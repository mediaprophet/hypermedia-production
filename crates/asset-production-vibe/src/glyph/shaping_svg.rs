//! Word wrapping, glyph run shaping, SVG projection, and straight-alpha RGBA rasterization.

use super::{char_to_svg_subpath, glyph_advance, pair_kerning, ShapedGlyphRun};
use crate::geometry::color;
use crate::*;
use anyrender::{ImageRenderer, PaintScene};
use kurbo::{Affine, BezPath};
use std::fmt::Write;

/// Greedy word-wrapping supporting manual newlines and bounded per-character breaking.
fn wrap_paragraph(para: &str, font_size: f64, tracking: f64, max_width: f64) -> Vec<String> {
    if max_width <= 0.0 || para.is_empty() {
        return vec![para.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0.0;
    let mut prev_char: Option<char> = None;

    let words: Vec<&str> = para.split(' ').collect();
    for &word in &words {
        if word.is_empty() {
            let sp_adv = (glyph_advance(' ') + tracking) * font_size;
            if !current_line.is_empty() {
                if current_width + sp_adv <= max_width {
                    current_line.push(' ');
                    current_width += sp_adv;
                    prev_char = Some(' ');
                } else {
                    lines.push(current_line);
                    current_line = String::new();
                    current_width = 0.0;
                    prev_char = None;
                }
            }
            continue;
        }

        let mut word_width = 0.0;
        let mut w_prev = None;
        for ch in word.chars() {
            let k = w_prev.map(|p| pair_kerning(p, ch)).unwrap_or(0.0);
            word_width += (glyph_advance(ch) + k + tracking) * font_size;
            w_prev = Some(ch);
        }

        let space_adv = if current_line.is_empty() {
            0.0
        } else {
            let k = prev_char.map(|p| pair_kerning(p, ' ')).unwrap_or(0.0);
            (glyph_advance(' ') + k + tracking) * font_size
        };

        if current_line.is_empty() {
            if word_width <= max_width {
                current_line.push_str(word);
                current_width = word_width;
                prev_char = word.chars().last();
            } else {
                for ch in word.chars() {
                    let k = prev_char.map(|p| pair_kerning(p, ch)).unwrap_or(0.0);
                    let ch_adv = (glyph_advance(ch) + k + tracking) * font_size;
                    if !current_line.is_empty() && current_width + ch_adv > max_width {
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = 0.0;
                    }
                    current_line.push(ch);
                    current_width += (glyph_advance(ch) + tracking) * font_size;
                    prev_char = Some(ch);
                }
            }
        } else if current_width + space_adv + word_width <= max_width {
            current_line.push(' ');
            current_line.push_str(word);
            current_width += space_adv + word_width;
            prev_char = word.chars().last();
        } else {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0.0;
            prev_char = None;

            if word_width <= max_width {
                current_line.push_str(word);
                current_width = word_width;
                prev_char = word.chars().last();
            } else {
                for ch in word.chars() {
                    let k = prev_char.map(|p| pair_kerning(p, ch)).unwrap_or(0.0);
                    let ch_adv = (glyph_advance(ch) + k + tracking) * font_size;
                    if !current_line.is_empty() && current_width + ch_adv > max_width {
                        lines.push(current_line);
                        current_line = String::new();
                        current_width = 0.0;
                    }
                    current_line.push(ch);
                    current_width += (glyph_advance(ch) + tracking) * font_size;
                    prev_char = Some(ch);
                }
            }
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Shapes, measures, and positions vector glyph runs into SVG path instructions.
pub fn shape_glyph_run(
    text: &str,
    font_size: f64,
    line_height: f64,
    tracking: f64,
    max_width: f64,
    origin_x: f64,
    origin_y: f64,
) -> ShapedGlyphRun {
    if text.is_empty() {
        return ShapedGlyphRun {
            text: String::new(),
            font_size,
            line_height,
            tracking,
            width: 0.0,
            height: 0.0,
            line_count: 0,
            glyph_count: 0,
            svg_path_d: String::new(),
        };
    }

    let mut lines = Vec::new();
    for raw_para in text.split('\n') {
        let para = raw_para.strip_suffix('\r').unwrap_or(raw_para);
        lines.extend(wrap_paragraph(para, font_size, tracking, max_width));
    }

    let line_step = font_size * line_height;
    let mut max_line_width: f64 = 0.0;
    let mut glyph_count = 0;
    let mut svg_path_d = String::new();

    for (line_idx, line_str) in lines.iter().enumerate() {
        let line_y = origin_y + line_idx as f64 * line_step;
        let mut pen_x = origin_x;
        let mut prev_char: Option<char> = None;

        for ch in line_str.chars() {
            let k = prev_char.map(|p| pair_kerning(p, ch)).unwrap_or(0.0);
            pen_x += k * font_size;

            char_to_svg_subpath(ch, pen_x, line_y, font_size, &mut svg_path_d);
            if ch != ' ' && ch != '\t' {
                glyph_count += 1;
            }

            pen_x += (glyph_advance(ch) + tracking) * font_size;
            prev_char = Some(ch);
        }

        let line_w = pen_x - origin_x;
        if line_w > max_line_width {
            max_line_width = line_w;
        }
    }

    let line_count = lines.len();
    let height = line_count as f64 * line_step;

    ShapedGlyphRun {
        text: text.to_string(),
        font_size,
        line_height,
        tracking,
        width: max_line_width,
        height,
        line_count,
        glyph_count,
        svg_path_d,
    }
}

impl AssetHost<'_> {
    /// Dispatches glyph operations admitted by the category-theoretic ontology.
    pub(crate) fn glyph_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.glyph.measure_bounds" | "hypermedia.glyph.measure" => {
                fields(
                    a,
                    &["text", "font_size", "line_height", "tracking", "max_width"],
                )?;
                let t = text(required(a, "text")?)?;
                if t.len() > 16384 {
                    return Err("text exceeds 16384 bytes".into());
                }
                let font_size = number(a, "font_size", 16.0)?;
                if !(4.0..=512.0).contains(&font_size) {
                    return Err("font_size outside 4..512".into());
                }
                let line_height = number(a, "line_height", 1.25)?;
                if !(0.8..=4.0).contains(&line_height) {
                    return Err("line_height outside 0.8..4.0".into());
                }
                let tracking = number(a, "tracking", 0.0)?;
                if !(-0.5..=2.0).contains(&tracking) {
                    return Err("tracking outside -0.5..2.0".into());
                }
                let max_width = number(a, "max_width", 0.0)?;
                if !(0.0..=4096.0).contains(&max_width) {
                    return Err("max_width outside 0..4096".into());
                }

                self.charge(t.len().max(1) * 8, t.len() * 16)?;
                let run =
                    shape_glyph_run(t, font_size, line_height, tracking, max_width, 0.0, 0.0);

                Ok(Value::Record(BTreeMap::from([
                    ("width".into(), Value::F64(run.width)),
                    ("height".into(), Value::F64(run.height)),
                    ("line_count".into(), Value::U64(run.line_count as u64)),
                    ("glyph_count".into(), Value::U64(run.glyph_count as u64)),
                ])))
            }
            "hypermedia.glyph.shape_to_svg_path" | "hypermedia.glyph.shape" => {
                fields(
                    a,
                    &[
                        "text",
                        "font_size",
                        "line_height",
                        "tracking",
                        "max_width",
                        "origin_x",
                        "origin_y",
                    ],
                )?;
                let t = text(required(a, "text")?)?;
                if t.len() > 16384 {
                    return Err("text exceeds 16384 bytes".into());
                }
                let font_size = number(a, "font_size", 16.0)?;
                if !(4.0..=512.0).contains(&font_size) {
                    return Err("font_size outside 4..512".into());
                }
                let line_height = number(a, "line_height", 1.25)?;
                if !(0.8..=4.0).contains(&line_height) {
                    return Err("line_height outside 0.8..4.0".into());
                }
                let tracking = number(a, "tracking", 0.0)?;
                if !(-0.5..=2.0).contains(&tracking) {
                    return Err("tracking outside -0.5..2.0".into());
                }
                let max_width = number(a, "max_width", 0.0)?;
                if !(0.0..=4096.0).contains(&max_width) {
                    return Err("max_width outside 0..4096".into());
                }
                let origin_x = number(a, "origin_x", 0.0)?;
                let origin_y = number(a, "origin_y", 0.0)?;
                if origin_x.abs() > 1e6 || origin_y.abs() > 1e6 {
                    return Err("origin coordinates outside bounds".into());
                }

                self.charge(t.len().max(1) * 16, t.len() * 64 + 1024)?;
                let run = shape_glyph_run(
                    t,
                    font_size,
                    line_height,
                    tracking,
                    max_width,
                    origin_x,
                    origin_y,
                );

                Ok(Value::Record(BTreeMap::from([
                    ("path".into(), Value::String(run.svg_path_d)),
                    ("width".into(), Value::F64(run.width)),
                    ("height".into(), Value::F64(run.height)),
                    ("line_count".into(), Value::U64(run.line_count as u64)),
                    ("glyph_count".into(), Value::U64(run.glyph_count as u64)),
                ])))
            }
            "hypermedia.glyph.project_svg" | "hypermedia.glyph.svg" => {
                fields(
                    a,
                    &[
                        "text",
                        "font_size",
                        "line_height",
                        "tracking",
                        "max_width",
                        "origin_x",
                        "origin_y",
                        "color",
                        "background",
                        "padding",
                        "stroke_width",
                    ],
                )?;
                let t = text(required(a, "text")?)?;
                if t.len() > 16384 {
                    return Err("text exceeds 16384 bytes".into());
                }
                let font_size = number(a, "font_size", 16.0)?;
                if !(4.0..=512.0).contains(&font_size) {
                    return Err("font_size outside 4..512".into());
                }
                let line_height = number(a, "line_height", 1.25)?;
                if !(0.8..=4.0).contains(&line_height) {
                    return Err("line_height outside 0.8..4.0".into());
                }
                let tracking = number(a, "tracking", 0.0)?;
                if !(-0.5..=2.0).contains(&tracking) {
                    return Err("tracking outside -0.5..2.0".into());
                }
                let max_width = number(a, "max_width", 0.0)?;
                if !(0.0..=4096.0).contains(&max_width) {
                    return Err("max_width outside 0..4096".into());
                }
                let origin_x = number(a, "origin_x", 0.0)?;
                let origin_y = number(a, "origin_y", 0.0)?;
                if origin_x.abs() > 1e6 || origin_y.abs() > 1e6 {
                    return Err("origin coordinates outside bounds".into());
                }
                let pad = number(a, "padding", 8.0)?;
                if !(0.0..=512.0).contains(&pad) {
                    return Err("padding outside 0..512".into());
                }
                let stroke_width = number(a, "stroke_width", 1.8)?;
                if !(0.5..=32.0).contains(&stroke_width) {
                    return Err("stroke_width outside 0.5..32.0".into());
                }
                let fg = a
                    .get("color")
                    .map(color)
                    .transpose()?
                    .unwrap_or([234, 241, 248, 255]);
                let bg = a.get("background").map(color).transpose()?;

                self.charge(t.len().max(1) * 32, t.len() * 128 + 4096)?;
                let run = shape_glyph_run(
                    t,
                    font_size,
                    line_height,
                    tracking,
                    max_width,
                    origin_x,
                    origin_y,
                );

                let min_x = origin_x - pad;
                let min_y = origin_y - pad;
                let vb_w = (run.width + 2.0 * pad).max(1.0);
                let vb_h = (run.height + 2.0 * pad).max(1.0);

                let mut svg = format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"{:.2} {:.2} {:.2} {:.2}\">\n",
                    min_x, min_y, vb_w, vb_h
                );
                if let Some(b) = bg {
                    let _ = writeln!(
                        &mut svg,
                        "  <rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#{:02x}{:02x}{:02x}\" fill-opacity=\"{:.3}\"/>",
                        min_x,
                        min_y,
                        vb_w,
                        vb_h,
                        b[0],
                        b[1],
                        b[2],
                        b[3] as f64 / 255.0
                    );
                }
                let _ = writeln!(
                    &mut svg,
                    "  <path d=\"{}\" fill=\"none\" stroke=\"#{:02x}{:02x}{:02x}\" stroke-opacity=\"{:.3}\" stroke-width=\"{:.2}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
                    run.svg_path_d,
                    fg[0],
                    fg[1],
                    fg[2],
                    fg[3] as f64 / 255.0,
                    stroke_width
                );
                svg.push_str("</svg>\n");

                self.encode("svg", svg.into_bytes())
            }
            "hypermedia.glyph.rasterize_rgba" | "hypermedia.glyph.rasterize" => {
                fields(
                    a,
                    &[
                        "text",
                        "width",
                        "height",
                        "origin_x",
                        "origin_y",
                        "font_size",
                        "line_height",
                        "tracking",
                        "max_width",
                        "color",
                        "background",
                        "stroke_width",
                    ],
                )?;
                let t = text(required(a, "text")?)?;
                if t.len() > 16384 {
                    return Err("text exceeds 16384 bytes".into());
                }
                let w = count(a, "width", 256, 1024)? as u32;
                let h = count(a, "height", 96, 1024)? as u32;
                let origin_x = number(a, "origin_x", 12.0)?;
                let origin_y = number(a, "origin_y", 12.0)?;
                if origin_x.abs() > 1e6 || origin_y.abs() > 1e6 {
                    return Err("origin coordinates outside bounds".into());
                }
                let font_size = number(a, "font_size", 20.0)?;
                if !(4.0..=512.0).contains(&font_size) {
                    return Err("font_size outside 4..512".into());
                }
                let line_height = number(a, "line_height", 1.25)?;
                if !(0.8..=4.0).contains(&line_height) {
                    return Err("line_height outside 0.8..4.0".into());
                }
                let tracking = number(a, "tracking", 0.0)?;
                if !(-0.5..=2.0).contains(&tracking) {
                    return Err("tracking outside -0.5..2.0".into());
                }
                let max_width = number(a, "max_width", 0.0)?;
                if !(0.0..=4096.0).contains(&max_width) {
                    return Err("max_width outside 0..4096".into());
                }
                let fg = a
                    .get("color")
                    .map(color)
                    .transpose()?
                    .unwrap_or([234, 241, 248, 255]);
                let bg = a
                    .get("background")
                    .map(color)
                    .transpose()?
                    .unwrap_or([0, 0, 0, 0]);
                let stroke_width = number(a, "stroke_width", 2.0)?;
                if !(0.0..=64.0).contains(&stroke_width) {
                    return Err("stroke_width outside 0..64".into());
                }

                let n = w as usize * h as usize;
                self.charge(n * 4 + t.len() * 32, n * 16 + t.len() * 64)?;

                let run = shape_glyph_run(
                    t,
                    font_size,
                    line_height,
                    tracking,
                    max_width,
                    origin_x,
                    origin_y,
                );

                let path = if run.svg_path_d.trim().is_empty() {
                    BezPath::new()
                } else {
                    BezPath::from_svg(&run.svg_path_d)
                        .map_err(|e| format!("Invalid SVG path: {e:?}"))?
                };

                let mut bytes = vec![0u8; n * 4];
                let mut renderer = vector_render::VelloCpuImageRenderer::new(w, h);
                renderer.render(
                    |scene| {
                        if bg[3] > 0 {
                            let bg_brush = peniko::Color::from_rgba8(bg[0], bg[1], bg[2], bg[3]);
                            scene.fill(
                                peniko::Fill::NonZero,
                                Affine::IDENTITY,
                                bg_brush,
                                None,
                                &kurbo::Rect::new(0.0, 0.0, w as f64, h as f64),
                            );
                        }
                        if !path.elements().is_empty() && stroke_width > 0.0 && fg[3] > 0 {
                            let stroke = kurbo::Stroke::new(stroke_width)
                                .with_caps(kurbo::Cap::Round)
                                .with_join(kurbo::Join::Round);
                            let fg_brush = peniko::Color::from_rgba8(fg[0], fg[1], fg[2], fg[3]);
                            scene.stroke(&stroke, Affine::IDENTITY, fg_brush, None, &path);
                        }
                    },
                    &mut bytes,
                );

                let pixels: Vec<[u8; 4]> = bytes
                    .chunks_exact(4)
                    .map(|p| {
                        let alpha = p[3] as u32;
                        if alpha == 0 {
                            [0; 4]
                        } else {
                            [
                                (p[0] as u32 * 255 + alpha / 2)
                                    .checked_div(alpha)
                                    .unwrap_or(0)
                                    .min(255) as u8,
                                (p[1] as u32 * 255 + alpha / 2)
                                    .checked_div(alpha)
                                    .unwrap_or(0)
                                    .min(255) as u8,
                                (p[2] as u32 * 255 + alpha / 2)
                                    .checked_div(alpha)
                                    .unwrap_or(0)
                                    .min(255) as u8,
                                p[3],
                            ]
                        }
                    })
                    .collect();

                self.workspace.insert(
                    Artifact::Rgba(crate::graphics::Rgba {
                        width: w,
                        height: h,
                        pixels,
                    }),
                    self.limits,
                )
            }
            _ => Err(format!("Unsupported glyph operation: {id}")),
        }
    }
}

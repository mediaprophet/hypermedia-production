//! Shared, deterministic glyph outlines for raster and SVG UI labels.
use super::{LayoutBox, ThemeTokens};
use kurbo::{Affine, BezPath, Rect, Shape};
use parley::{Alignment, AlignmentOptions, FontContext, Layout, LayoutContext,
    PositionedLayoutItem, StyleProperty};
use std::{borrow::Cow, sync::Arc};

pub(super) struct LabelOutline {
    pub path: BezPath,
    pub clip: Rect,
    pub color: [u8; 4],
    pub stroke_width: f64,
}

pub(super) fn shape_labels(
    boxes: &[LayoutBox], theme: &ThemeTokens, font: Option<&[u8]>,
) -> Result<Vec<Option<LabelOutline>>, String> {
    let mut font_context = FontContext {
        collection: parley::fontique::Collection::new(parley::fontique::CollectionOptions {
            shared: false, system_fonts: false,
        }),
        source_cache: Default::default(),
    };
    let family = if let Some(bytes) = font {
        if bytes.is_empty() || bytes.len() > 4 * 1024 * 1024 {
            return Err("UI font must contain 1..=4 MiB of retained OpenType data".into());
        }
        let face = ttf_parser::Face::parse(bytes, 0).map_err(|e| format!("UI font: {e:?}"))?;
        if face.is_variable() {
            return Err("UI outline export requires a static font instance; variable font axes are not yet retained".into());
        }
        let families = font_context.collection.register_fonts(
            peniko::Blob::new(Arc::new(bytes.to_vec())), None);
        let id = families.first().ok_or("UI font has no usable family")?.0;
        Some(font_context.collection.family_name(id).ok_or("UI font family missing")?.to_string())
    } else { None };
    let mut layout_context = LayoutContext::<()>::new();
    boxes.iter().map(|b| {
        if b.label.is_empty() || b.width <= 0.0 || b.height <= 0.0 { return Ok(None); }
        let font_size = match b.role.as_str() { "header" => 16.0, "badge" => 11.0,
            "button" | "input" => 13.0, _ => 14.0 };
        let pad = if b.role == "input" { 12.0 } else { 16.0 };
        let width = (b.width - 2.0 * pad).max(1.0);
        let (mut path, text_width, text_height) = if let Some(family) = &family {
            let mut builder = layout_context.ranged_builder(&mut font_context, &b.label, 1.0, true);
            builder.push_default(StyleProperty::FontSize(font_size as f32));
            builder.push_default(StyleProperty::FontFamily(parley::FontFamily::Source(
                Cow::Owned(format!("\"{}\"", family.replace('"', ""))))));
            let mut layout: Layout<()> = builder.build(&b.label);
            layout.break_all_lines(Some(width as f32));
            layout.align(Alignment::Start, AlignmentOptions::default());
            let mut path = BezPath::new();
            for line in layout.lines() {
                for item in line.items() {
                    if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                        let run = glyph_run.run();
                        let face = ttf_parser::Face::parse(run.font().data.as_ref(), run.font().index)
                            .map_err(|e| format!("shaped UI font: {e:?}"))?;
                        let scale = run.font_size() as f64 / face.units_per_em() as f64;
                        for glyph in glyph_run.positioned_glyphs() {
                            if glyph.id == 0 { return Err(format!("retained UI font cannot shape label {:?}", b.label)); }
                            let mut outline = Outline { path: &mut path, x: glyph.x as f64,
                                y: glyph.y as f64, scale };
                            let glyph_id = ttf_parser::GlyphId(glyph.id as u16);
                            let visible = face.outline_glyph(glyph_id, &mut outline).is_some();
                            // Bitmap/color-only glyphs must not silently disappear from
                            // SVG/raster parity. Blank whitespace and joiners are legitimate.
                            let blank = [' ', '\t', '\r', '\n', '\u{00a0}', '\u{200b}', '\u{200c}', '\u{200d}', '\u{feff}']
                                .iter().any(|character| face.glyph_index(*character) == Some(glyph_id));
                            if !visible && !blank {
                                return Err("Retained UI font contains a glyph without a supported vector outline".into());
                            }
                        }
                    }
                }
            }
            (path, layout.width() as f64, layout.height() as f64)
        } else {
            if b.label.chars().any(|c| !c.is_ascii()) {
                return Err("Non-ASCII UI labels require a retained `font` input with the required glyphs".into());
            }
            let run = crate::glyph::shape_glyph_run(&b.label, font_size, 1.25, 0.0, width, 0.0, 0.0);
            (BezPath::from_svg(&run.svg_path_d).map_err(|e| format!("UI label outline: {e}"))?, run.width, run.height)
        };
        let centered = matches!(b.role.as_str(), "button" | "badge");
        let x = b.x + if centered { ((b.width - text_width) / 2.0).max(0.0) } else { pad };
        let y = b.y + if centered || matches!(b.role.as_str(), "header" | "input") {
            ((b.height - text_height) / 2.0).max(0.0)
        } else { 12.0 };
        path.apply_affine(Affine::translate((x, y)));
        let mut color = match b.role.as_str() {
            "badge" => theme.accent, "input" => theme.text_muted, _ => theme.text_primary,
        };
        if b.state == "disabled" { color[3] = 100; }
        Ok(Some(LabelOutline { path, clip: Rect::new(b.x, b.y, b.x+b.width, b.y+b.height), color,
            stroke_width: if font.is_some() { 0.0 } else { font_size * 0.07 } }))
    }).collect()
}

struct Outline<'a> { path: &'a mut BezPath, x: f64, y: f64, scale: f64 }
impl Outline<'_> {
    fn point(&self, x: f32, y: f32) -> (f64, f64) {
        (self.x + x as f64*self.scale, self.y - y as f64*self.scale)
    }
}
impl ttf_parser::OutlineBuilder for Outline<'_> {
    fn move_to(&mut self, x:f32, y:f32) { self.path.move_to(self.point(x,y)); }
    fn line_to(&mut self, x:f32, y:f32) { self.path.line_to(self.point(x,y)); }
    fn quad_to(&mut self, x1:f32,y1:f32,x:f32,y:f32) {
        self.path.quad_to(self.point(x1,y1),self.point(x,y));
    }
    fn curve_to(&mut self,x1:f32,y1:f32,x2:f32,y2:f32,x:f32,y:f32) {
        self.path.curve_to(self.point(x1,y1),self.point(x2,y2),self.point(x,y));
    }
    fn close(&mut self) { self.path.close_path(); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_is_glyph_geometry_and_respects_its_box() {
        let theme = super::super::build_theme_tokens("test", "holosonic-dark", [0,216,242,255], 10.0);
        let b = LayoutBox { id:"label".into(), role:"button".into(),label:"EXPORT".into(),
            state:"default".into(),x:10.0,y:20.0,width:140.0,height:40.0 };
        let labels = shape_labels(&[b], &theme, None).unwrap();
        let label = labels[0].as_ref().unwrap();
        assert!(label.path.elements().len() > 20);
        assert!(label.clip.contains(label.path.bounding_box().center()));
    }
}

//! Unicode shaping and precise OpenType glyph outlines from an admitted font.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{borrow::Cow, sync::Arc};

use crate::layout_ops::label_outline::sfnt_outline;
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use crate::*;
use kurbo::Shape;
use parley::{
    Alignment, AlignmentOptions, FontContext, FontFamily, Layout, LayoutContext,
    PositionedLayoutItem, StyleProperty, style::FontFeatures,
};
use serde_json::json;
use sha2::{Digest, Sha256};

impl AssetHost<'_> {
    pub(crate) fn shape_opentype_sfnt(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["font", "text", "language", "features", "size_pt", "wrap_pt"],
        )?;
        let font = required(args, "font")?;
        let text_value = text(required(args, "text")?)?;
        if text_value.is_empty() || text_value.len() > 16_384 || text_value.chars().count() > 4096 {
            return Err("OpenType shaping requires 1 to 4096 Unicode characters".into());
        }
        let resource = encoded_resource(self, font)?;
        if !matches!(resource.kind.as_str(), "ttf" | "otf")
            || !(12..=4 * 1024 * 1024).contains(&resource.bytes.len())
        {
            return Err("OpenType shaping requires a bounded standalone TTF or OTF font".into());
        }
        let size = num(required(args, "size_pt")?)?;
        let wrap = args.get("wrap_pt").map(num).transpose()?;
        if !(1.0..=1000.0).contains(&size)
            || wrap.is_some_and(|width| !(1.0..=65536.0).contains(&width))
        {
            return Err(
                "OpenType font size or line width is outside the admitted typographic range".into(),
            );
        }
        let language = args
            .get("language")
            .map(text)
            .transpose()?
            .map(|lang| {
                lang.parse()
                    .map_err(|_| "Language must be a valid locale tag")
            })
            .transpose()?;
        let mut features = Vec::<String>::new();
        if let Some(value) = args.get("features") {
            let entries = record(value)?;
            if entries.len() > 32 {
                return Err("Supply at most 32 OpenType feature settings".into());
            }
            for (tag, value) in entries {
                if tag.len() != 4 || !tag.bytes().all(|ch| ch.is_ascii_alphanumeric()) {
                    return Err("OpenType feature tags require four letters or digits".into());
                }
                let setting = integer(value, "OpenType feature value")?;
                if setting > 65535 {
                    return Err("OpenType feature value exceeds u16".into());
                }
                features.push(format!("'{tag}' {setting}"));
            }
        }
        let source_bytes = resource.bytes.len();
        self.charge(
            source_bytes
                .saturating_mul(16)
                .saturating_add(text_value.len() * 1024),
            source_bytes
                .saturating_mul(8)
                .saturating_add(text_value.len() * 128),
        )?;
        let bytes = encoded_resource(self, font)?.bytes.as_slice();
        let digest = format!("{:x}", Sha256::digest(bytes));
        let source_face =
            ttf_parser::Face::parse(bytes, 0).map_err(|err| format!("OpenType font: {err:?}"))?;
        if source_face.is_variable() || bytes.starts_with(b"ttcf") {
            return Err(
                "Choose a static single-face OpenType font for deterministic outline export".into(),
            );
        }
        let mut fonts = FontContext {
            collection: parley::fontique::Collection::new(parley::fontique::CollectionOptions {
                shared: false,
                system_fonts: false,
            }),
            source_cache: Default::default(),
        };
        let families = fonts
            .collection
            .register_fonts(peniko::Blob::new(Arc::new(bytes.to_vec())), None);
        let family = families
            .first()
            .and_then(|value| fonts.collection.family_name(value.0))
            .ok_or("Admitted font has no usable family")?
            .replace('"', "");
        let mut ctx = LayoutContext::<()>::new();
        let mut builder = ctx.ranged_builder(&mut fonts, text_value, 1.0, true);
        builder.push_default(StyleProperty::FontSize(size as f32));
        builder.push_default(StyleProperty::FontFamily(FontFamily::Source(Cow::Owned(
            format!("\"{family}\""),
        ))));
        if let Some(language) = language {
            builder.push_default(StyleProperty::Locale(Some(language)));
        }
        if !features.is_empty() {
            builder.push_default(StyleProperty::FontFeatures(FontFeatures::Source(
                Cow::Owned(features.join(", ")),
            )));
        }
        let mut layout: Layout<()> = builder.build(text_value);
        layout.break_all_lines(wrap.map(|v| v as f32));
        layout.align(Alignment::Start, AlignmentOptions::default());

        let mut lines = Vec::new();
        let mut path = kurbo::BezPath::new();
        let max_path = self.limits.resource_bytes.min(16 * 1024 * 1024) / 100;
        let mut glyph_count = 0usize;
        for line in layout.lines() {
            let mut glyphs = Vec::new();
            for item in line.items() {
                let PositionedLayoutItem::GlyphRun(run) = item else {
                    continue;
                };
                let source = run.run().font();
                if source.index != 0 || source.data.as_ref() != bytes {
                    return Err("Shaper selected a fallback font outside the retained face".into());
                }
                let scale =
                    f64::from(run.run().font_size()) / f64::from(source_face.units_per_em());
                for glyph in run.positioned_glyphs() {
                    glyph_count += 1;
                    if glyph_count > 8192 || glyph.id == 0 {
                        return Err(
                            "Text includes a missing glyph or exceeds the outline glyph budget"
                                .into(),
                        );
                    }
                    let glyph_id = ttf_parser::GlyphId(glyph.id as u16);
                    let outline = sfnt_outline(
                        &source_face,
                        glyph_id,
                        f64::from(glyph.x),
                        f64::from(glyph.y),
                        scale,
                    );
                    let has_outline = outline.is_some();
                    if let Some(outline) = outline {
                        if path
                            .elements()
                            .len()
                            .saturating_add(outline.elements().len())
                            > max_path
                        {
                            return Err("OpenType outline exceeds its path budget".into());
                        }
                        path.extend(outline.elements().iter().copied());
                    } else if ![
                        ' ', '\t', '\r', '\n', '\u{00a0}', '\u{200b}', '\u{200c}', '\u{200d}',
                        '\u{feff}',
                    ]
                    .iter()
                    .any(|&ch| source_face.glyph_index(ch) == Some(glyph_id))
                    {
                        return Err("Shaped glyph has no supported vector contour".into());
                    }
                    glyphs.push(json!({
                        "id": glyph.id, "x_pt": glyph.x, "y_pt": glyph.y, "advance_pt": glyph.advance,
                        "horizontal_advance_units": source_face.glyph_hor_advance(glyph_id),
                        "vector_outline": has_outline
                    }));
                }
            }
            let range = line.text_range();
            lines.push(json!({
                "utf8_start": range.start, "utf8_end": range.end, "glyphs": glyphs
            }));
        }
        let metrics = json!({
            "schema": "qualia-opentype-layout/1", "font_sha256": digest,
            "units_per_em": source_face.units_per_em(), "ascender_units": source_face.ascender(),
            "descender_units": source_face.descender(), "width_pt": layout.width(), "height_pt": layout.height(),
            "glyph_count": glyph_count, "lines": lines
        });
        let bounds = path.bounding_box().union(kurbo::Rect::new(
            0.0, 0.0, f64::from(layout.width().max(1.0)), f64::from(layout.height().max(1.0)),
        ));
        let width = bounds.width();
        let height = bounds.height();
        let (left, top) = (bounds.x0, bounds.y0);
        let svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}pt\" height=\"{height}pt\" viewBox=\"{left} {top} {width} {height}\"><path d=\"{}\"/></svg>",
            path.to_svg()
        );
        let metrics = self.encode(
            "json",
            serde_json::to_vec(&metrics).map_err(|err| err.to_string())?,
        )?;
        let outlines = self.encode("svg", svg.into_bytes())?;
        Ok(Value::Record(Args::from([
            ("outlines".into(), outlines),
            ("metrics".into(), metrics),
        ])))
    }
}

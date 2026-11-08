use super::*;
use std::collections::BTreeMap;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

impl AssetHost<'_> {
    /// 131: `hypermedia.glyph.compose_knuth_plass_paragraph`
    /// (`vector-glyph-shaping`)
    pub(crate) fn compose_knuth_plass_paragraph(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["text", "frame_width", "frame_height"])?;
        let text_str = opt_str(
            a,
            "text",
            "Typography and international human rights protect creative authorship and universal linguistic diversity.",
        )?;
        let frame_w = number(a, "frame_width", 220.0)?;
        let frame_h = number(a, "frame_height", 120.0)?;

        self.charge(256, 4096)?;

        use std::fmt::Write as _;
        use vector_glyph_shaping::{
            AreaTextOptions, ComposerParams, OtFeatures, SGlyph, SfntTable, build_sfnt_font,
            compose, hyphen_points, layout_area_text, parse_sfnt_directory, resolve_bidi_runs,
        };
        use vector_locus_geometry::Rect;

        let w = frame_w.max(40.0);
        let h = frame_h.max(20.0);
        let (_, bidi_runs) = resolve_bidi_runs(&text_str, None);
        let hyph = hyphen_points("typography");
        let ot = OtFeatures::from_tags(["liga", "kern", "smcp"]);
        let area_res = layout_area_text(
            Rect::new(0.0, 0.0, w, h),
            &text_str,
            &AreaTextOptions::default(),
        );

        let glyphs: Vec<SGlyph> = (0..12).map(|_| SGlyph::char_glyph(8.0)).collect();
        let kp_breaks = compose(&glyphs, &|_| frame_w.max(60.0), &|_| 0.0, &[], &ComposerParams::default());

        let mut svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w:.1} {h:.1}\">\n  <rect width=\"{w:.1}\" height=\"{h:.1}\" fill=\"#fdfbf7\"/>\n"
        );
        for line in &area_res.lines {
            let escaped = line
                .text
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            let _ = writeln!(
                svg_xml,
                "  <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"serif\" font-size=\"12\" fill=\"#1a1a1a\">{escaped}</text>",
                line.origin.x, line.baseline_y
            );
        }
        svg_xml.push_str("</svg>\n");
        let svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let sfnt_bytes = build_sfnt_font(
            false,
            &[SfntTable {
                tag: *b"head",
                checksum: 0,
                offset: 0,
                length: 54,
                data: vec![0u8; 54],
            }],
        );
        let sfnt_dir = parse_sfnt_directory(&sfnt_bytes)
            .map_err(|e| format!("SFNT directory parse failed: {e}"))?;
        let font_resource = self.encode("ttf", sfnt_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("svg_resource".into(), svg_resource);
        out.insert("font_resource".into(), font_resource);
        out.insert("bidi_run_count".into(), Value::I64(bidi_runs.len() as i64));
        out.insert("hyphen_split_count".into(), Value::I64(hyph.len() as i64));
        out.insert("small_caps_enabled".into(), Value::Bool(ot.small_caps));
        out.insert("laid_out_line_count".into(), Value::I64(area_res.lines.len() as i64));
        out.insert("knuth_plass_converged".into(), Value::Bool(kp_breaks.is_some()));
        out.insert("sfnt_table_count".into(), Value::I64(sfnt_dir.len() as i64));
        Ok(Value::Record(out))
    }
}

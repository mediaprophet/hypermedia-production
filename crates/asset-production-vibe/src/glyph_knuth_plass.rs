use super::*;
use crate::glyph::{glyph_advance, pair_kerning};
use std::collections::BTreeMap;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// UAX #9 L2 visual order for a single laid-out line (char-level levels).
fn visual_reorder_line(line: &str) -> String {
    use vector_glyph_shaping::{bidi_class, reorder_visual, resolve_bidi_runs, BidiClass, BidiDirection};

    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    let (dir, _) = resolve_bidi_runs(line, None);
    let base: u8 = if dir == BidiDirection::Rtl { 1 } else { 0 };
    let mut levels = Vec::with_capacity(chars.len());
    for &ch in &chars {
        let lvl = match bidi_class(ch) {
            BidiClass::AL | BidiClass::R => 1,
            BidiClass::L => 0,
            BidiClass::EN | BidiClass::AN => {
                if base == 1 {
                    2
                } else {
                    0
                }
            }
            _ => base,
        };
        levels.push(lvl);
    }
    reorder_visual(&levels).into_iter().map(|i| chars[i]).collect()
}

struct GlyphTok {
    ch: char,
    glyph: vector_glyph_shaping::SGlyph,
}

/// Shape `text` into composer glyphs, word/hyphen breakpoints, and hyphen-point count.
fn paragraph_glyphs(text: &str, em_px: f64) -> (Vec<GlyphTok>, Vec<vector_glyph_shaping::Breakpoint>, i64) {
    use vector_glyph_shaping::{hyphen_points, Breakpoint, SGlyph};

    let chars: Vec<char> = text.chars().collect();
    let mut toks: Vec<GlyphTok> = Vec::new();
    let mut cands: Vec<Breakpoint> = Vec::new();
    let mut hyphen_split_count = 0i64;
    let hyphen_w = glyph_advance('-') * em_px;
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        if ch == '\n' || ch == '\r' {
            if !toks.is_empty() {
                cands.push(Breakpoint {
                    end: toks.len(),
                    hyphen: 0.0,
                });
            }
            i += 1;
            continue;
        }
        if ch.is_whitespace() {
            let k = toks.last().map(|t| pair_kerning(t.ch, ch)).unwrap_or(0.0);
            toks.push(GlyphTok {
                ch,
                glyph: SGlyph::space_glyph((glyph_advance(ch) + k) * em_px),
            });
            cands.push(Breakpoint {
                end: toks.len(),
                hyphen: 0.0,
            });
            i += 1;
            continue;
        }
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '\n' && chars[i] != '\r' {
            i += 1;
        }
        let word: String = chars[start..i].iter().collect();
        let hyph = hyphen_points(&word);
        hyphen_split_count += hyph.len() as i64;
        let word_glyph_start = toks.len();
        for (wi, &wch) in chars[start..i].iter().enumerate() {
            let k = toks.last().map(|t| pair_kerning(t.ch, wch)).unwrap_or(0.0);
            toks.push(GlyphTok {
                ch: wch,
                glyph: SGlyph::char_glyph((glyph_advance(wch) + k) * em_px),
            });
            if wi > 0 && hyph.contains(&wi) {
                cands.push(Breakpoint {
                    end: word_glyph_start + wi,
                    hyphen: hyphen_w,
                });
            }
        }
    }
    (toks, cands, hyphen_split_count)
}

fn lines_from_breaks(toks: &[GlyphTok], breaks: &[(usize, bool)]) -> Vec<String> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    for &(end, hyphenated) in breaks {
        if end < start {
            continue;
        }
        let mut chars: Vec<char> = toks[start..end.min(toks.len())].iter().map(|t| t.ch).collect();
        while chars.last().map(|c| c.is_whitespace()).unwrap_or(false) {
            chars.pop();
        }
        let mut line: String = chars.iter().collect();
        if hyphenated {
            line.push('-');
        }
        if !line.is_empty() {
            lines.push(visual_reorder_line(&line));
        }
        start = end;
    }
    lines
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
        if text_str.len() > 65536 {
            return Err("text exceeds 65536 bytes".into());
        }
        let frame_w = number(a, "frame_width", 220.0)?;
        let frame_h = number(a, "frame_height", 120.0)?;

        self.charge(
            text_str.len().max(1) * 16 + 256,
            text_str.len() * 64 + 4096,
        )?;

        use std::fmt::Write as _;
        use vector_glyph_shaping::{
            AreaTextOptions, ComposerParams, OtFeatures, SfntTable, build_sfnt_font,
            calc_table_checksum, compose, layout_area_text, parse_sfnt_directory, resolve_bidi_runs,
        };
        use vector_locus_geometry::Rect;

        let w = frame_w.max(40.0);
        let h = frame_h.max(20.0);
        let em_px = 12.0;
        let line_height = 14.0;

        let (_, bidi_runs) = resolve_bidi_runs(&text_str, None);
        let ot = OtFeatures::from_tags(["liga", "kern", "smcp"]);

        let (toks, cands, hyphen_split_count) = paragraph_glyphs(&text_str, em_px);
        let glyphs: Vec<_> = toks.iter().map(|t| t.glyph).collect();
        let measure_w = w.max(60.0);
        let kp_breaks = compose(
            &glyphs,
            &|_| measure_w,
            &|_| 0.0,
            &cands,
            &ComposerParams::default(),
        );

        let lines = if let Some(ref br) = kp_breaks {
            lines_from_breaks(&toks, br)
        } else {
            let area_res = layout_area_text(
                Rect::new(0.0, 0.0, w, h),
                &text_str,
                &AreaTextOptions::default(),
            );
            area_res.lines.into_iter().map(|l| l.text).collect()
        };

        let mut svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {w:.1} {h:.1}\">\n  <rect width=\"{w:.1}\" height=\"{h:.1}\" fill=\"#fdfbf7\"/>\n"
        );
        let mut y = line_height * 0.8;
        for line in &lines {
            if y > h {
                break;
            }
            let escaped = escape_xml(line);
            let _ = writeln!(
                svg_xml,
                "  <text x=\"0.0\" y=\"{y:.1}\" font-family=\"serif\" font-size=\"{em_px:.0}\" fill=\"#1a1a1a\">{escaped}</text>"
            );
            y += line_height;
        }
        svg_xml.push_str("</svg>\n");
        let svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let head_data = vec![0u8; 54];
        let checksum = calc_table_checksum(&head_data);
        let sfnt_bytes = build_sfnt_font(
            false,
            &[SfntTable {
                tag: *b"head",
                checksum,
                offset: 0,
                length: head_data.len() as u32,
                data: head_data,
            }],
        );
        let sfnt_dir = parse_sfnt_directory(&sfnt_bytes)
            .map_err(|e| format!("SFNT directory parse failed: {e}"))?;
        let font_resource = self.encode("ttf", sfnt_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("svg_resource".into(), svg_resource);
        out.insert("font_resource".into(), font_resource);
        out.insert("bidi_run_count".into(), Value::I64(bidi_runs.len() as i64));
        out.insert("hyphen_split_count".into(), Value::I64(hyphen_split_count));
        out.insert("small_caps_enabled".into(), Value::Bool(ot.small_caps));
        out.insert("laid_out_line_count".into(), Value::I64(lines.len() as i64));
        out.insert("knuth_plass_converged".into(), Value::Bool(kp_breaks.is_some()));
        out.insert("sfnt_table_count".into(), Value::I64(sfnt_dir.len() as i64));
        Ok(Value::Record(out))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    fn test_host<'a>(cancel: &'a AtomicBool, limits: &'a ToolLimits) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits,
            cancel,
            work: 0,
        }
    }

    #[test]
    fn compose_knuth_plass_paragraph_converges_and_two_produces_match() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert(
            "text".into(),
            Value::String(
                "Typography and international human rights protect creative authorship and universal linguistic diversity.".into(),
            ),
        );
        args.insert("frame_width".into(), Value::F64(220.0));
        args.insert("frame_height".into(), Value::F64(120.0));

        let a = host
            .compose_knuth_plass_paragraph(&args)
            .expect("knuth-plass a");
        let b = host
            .compose_knuth_plass_paragraph(&args)
            .expect("knuth-plass b");
        let Value::Record(ra) = &a else { panic!("expected record") };
        let Value::Record(rb) = &b else { panic!("expected record") };
        assert_eq!(ra.get("knuth_plass_converged"), Some(&Value::Bool(true)));
        assert_eq!(ra.get("small_caps_enabled"), Some(&Value::Bool(true)));
        let Value::I64(lines) = ra.get("laid_out_line_count").unwrap() else { panic!() };
        assert!(*lines >= 2, "paragraph must wrap, got {lines} lines");
        let Value::I64(hyph) = ra.get("hyphen_split_count").unwrap() else { panic!() };
        assert!(*hyph >= 2, "Liang points for typography-class words, got {hyph}");
        let Value::I64(runs) = ra.get("bidi_run_count").unwrap() else { panic!() };
        assert!(*runs >= 1);
        assert_eq!(ra.get("sfnt_table_count"), Some(&Value::I64(1)));

        let svg_a = host.workspace.encoded(ra.get("svg_resource").unwrap()).expect("svg a");
        let svg_b = host.workspace.encoded(rb.get("svg_resource").unwrap()).expect("svg b");
        assert_eq!(svg_a.bytes, svg_b.bytes, "two Knuth-Plass SVG produces must match");
        let xml = String::from_utf8_lossy(&svg_a.bytes);
        assert!(xml.contains("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(xml.contains("Typography") || xml.contains("Typog"));
        let font_a = host.workspace.encoded(ra.get("font_resource").unwrap()).expect("font a");
        let font_b = host.workspace.encoded(rb.get("font_resource").unwrap()).expect("font b");
        assert_eq!(font_a.bytes, font_b.bytes, "two SFNT produces must match");
        assert_eq!(font_a.kind, "ttf");
    }

    #[test]
    fn compose_knuth_plass_mixed_bidi_is_produce_safe() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut host = test_host(&cancel, &limits);

        let mut args = BTreeMap::new();
        args.insert("text".into(), Value::String("Hello שלום world".into()));
        args.insert("frame_width".into(), Value::F64(180.0));
        args.insert("frame_height".into(), Value::F64(80.0));

        let a = host
            .compose_knuth_plass_paragraph(&args)
            .expect("mixed bidi a");
        let b = host
            .compose_knuth_plass_paragraph(&args)
            .expect("mixed bidi b");
        let Value::Record(ra) = &a else { panic!() };
        let Value::Record(rb) = &b else { panic!() };
        assert_eq!(ra.get("knuth_plass_converged"), Some(&Value::Bool(true)));
        let Value::I64(runs) = ra.get("bidi_run_count").unwrap() else { panic!() };
        assert!(*runs >= 2, "mixed LTR/RTL must yield multiple BiDi runs, got {runs}");
        let svg_a = host.workspace.encoded(ra.get("svg_resource").unwrap()).expect("svg a");
        let svg_b = host.workspace.encoded(rb.get("svg_resource").unwrap()).expect("svg b");
        assert_eq!(svg_a.bytes, svg_b.bytes);
        let xml = String::from_utf8_lossy(&svg_a.bytes);
        assert!(xml.contains("םולש"), "RTL run must be L2-reversed in the SVG");
    }
}

use super::*;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use vibe::Value;

fn opt_str(a: &Args, k: &str, default: &str) -> Result<String, String> {
    match a.get(k) {
        None => Ok(default.to_string()),
        Some(v) => Ok(text(v)?.to_string()),
    }
}

fn xml_escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

impl AssetHost<'_> {
    /// 132: `hypermedia.layout.solve_flexbox_grid_constraints` (`layout-box-autolayout`)
    pub(crate) fn solve_flexbox_grid_constraints(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "container_width", "container_height", "gap", "padding"])?;
        let profile_dims = if let Some(pv) = a.get("profile") {
            let pts = self.profile(pv)?;
            let (mut min_x, mut max_x) = (f64::INFINITY, f64::NEG_INFINITY);
            let (mut min_y, mut max_y) = (f64::INFINITY, f64::NEG_INFINITY);
            for p in pts {
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }
            Some(((max_x - min_x).max(120.0), (max_y - min_y).max(80.0)))
        } else {
            None
        };
        let cw = number(a, "container_width", profile_dims.map(|d| d.0).unwrap_or(320.0))?;
        let ch = number(a, "container_height", profile_dims.map(|d| d.1).unwrap_or(180.0))?;
        let gap = number(a, "gap", 8.0)?;
        let pad = number(a, "padding", 12.0)?;

        self.charge(128, 2048)?;

        use layout_box_autolayout::{
            AutoLayoutFrame, GridTrack, LayoutDirection, LayoutNode, Padding, SizingMode,
            solve_autolayout,
        };

        let mut flex_frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
            .with_padding(Padding::uniform(pad))
            .with_gap(gap);
        flex_frame.add_child(LayoutNode::new(1, 80.0, 40.0));
        flex_frame.add_child(LayoutNode::new(2, 0.0, 40.0).with_sizing(SizingMode::Fill, SizingMode::Fixed));
        let (solved_w, solved_h) = solve_autolayout(&mut flex_frame, cw, ch);
        let fill_child_w = flex_frame.children[1].rect.width;

        let mut grid_frame = AutoLayoutFrame::new(LayoutDirection::Grid)
            .with_padding(Padding::uniform(pad))
            .with_gap(gap);
        grid_frame.grid_columns = vec![GridTrack::Fr(1.0), GridTrack::Fr(2.0)];
        grid_frame.grid_rows = vec![GridTrack::Fixed(60.0)];
        grid_frame.add_child(LayoutNode::new(10, 0.0, 0.0).with_sizing(SizingMode::Fill, SizingMode::Fill));
        grid_frame.add_child(LayoutNode::new(11, 0.0, 0.0).with_sizing(SizingMode::Fill, SizingMode::Fill));
        solve_autolayout(&mut grid_frame, cw, ch);

        let svg_w = solved_w.max(1.0);
        let svg_h = (solved_h * 2.0 + 16.0).max(1.0);
        let mut svg_xml = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {svg_w:.2} {svg_h:.2}\" width=\"{svg_w:.2}\" height=\"{svg_h:.2}\">\n\
             <rect x=\"0\" y=\"0\" width=\"{solved_w:.2}\" height=\"{solved_h:.2}\" fill=\"#12161f\" stroke=\"#3b82f6\" stroke-width=\"1.5\"/>\n"
        );
        for child in &flex_frame.children {
            let r = child.rect;
            let _ = writeln!(
                svg_xml,
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#1e3a8a\" fill-opacity=\"0.65\" stroke=\"#60a5fa\" stroke-width=\"1\"/>",
                r.x, r.y, r.width, r.height
            );
        }
        let grid_y_off = solved_h + 16.0;
        let _ = writeln!(
            svg_xml,
            "<rect x=\"0\" y=\"{grid_y_off:.2}\" width=\"{solved_w:.2}\" height=\"{solved_h:.2}\" fill=\"#12161f\" stroke=\"#10b981\" stroke-width=\"1.5\"/>"
        );
        for child in &grid_frame.children {
            let r = child.rect;
            let _ = writeln!(
                svg_xml,
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#065f46\" fill-opacity=\"0.65\" stroke=\"#34d399\" stroke-width=\"1\"/>",
                r.x, r.y + grid_y_off, r.width, r.height
            );
        }
        svg_xml.push_str("</svg>\n");
        let svg_resource = self.encode("svg", svg_xml.into_bytes())?;

        let (pw, ph) = (64u32, 36u32);
        let mut pixels = vec![[18u8, 22, 31, 255]; (pw * ph) as usize];
        for py in 0..ph {
            for px in 0..pw {
                let lx = (px as f64 + 0.5) / (pw as f64) * solved_w.max(1.0);
                let is_top_half = py < ph / 2;
                let local_y = if is_top_half {
                    (py as f64 + 0.5) / ((ph / 2) as f64) * solved_h.max(1.0)
                } else {
                    ((py - ph / 2) as f64 + 0.5) / ((ph - ph / 2) as f64) * solved_h.max(1.0)
                };
                let children = if is_top_half { &flex_frame.children } else { &grid_frame.children };
                for (idx, child) in children.iter().enumerate() {
                    let r = child.rect;
                    if lx >= r.x && lx <= r.right() && local_y >= r.y && local_y <= r.bottom() {
                        pixels[(py * pw + px) as usize] = match (is_top_half, idx % 2) {
                            (true, 0) => [59, 130, 246, 255],
                            (true, _) => [96, 165, 250, 255],
                            (false, 0) => [16, 185, 129, 255],
                            (false, _) => [52, 211, 153, 255],
                        };
                    }
                }
            }
        }
        let image_handle = self.store_rgba(pw, ph, pixels)?;

        let mut out = BTreeMap::new();
        out.insert("svg_resource".into(), svg_resource);
        out.insert("image".into(), image_handle);
        out.insert("solved_width".into(), Value::F64(solved_w));
        out.insert("solved_height".into(), Value::F64(solved_h));
        out.insert("fill_child_width".into(), Value::F64(fill_child_w));
        out.insert("grid_col1_width".into(), Value::F64(grid_frame.children[1].rect.width));
        Ok(Value::Record(out))
    }

    /// 133: `hypermedia.layout.typeset_paged_foliation` (`layout-paged-typesetter`)
    pub(crate) fn typeset_paged_foliation(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "columns", "latex_formula", "paragraph_count"])?;
        if let Some(pv) = a.get("profile") {
            let _ = self.profile(pv)?;
        }
        let columns = count(a, "columns", 2, 4)?;
        let formula = opt_str(a, "latex_formula", "\\frac{a+b}{c}")?;
        let para_count = count(a, "paragraph_count", 6, 64)?;

        self.charge(256, 8192)?;

        use layout_paged_typesetter::{
            Bibliography, Citation, CitationAuthor, CitationItem, CitationStyle, MItem, ParaDoc,
            PlacedItem, SectionProps, Typesetter, layout_equation, parse_latex,
        };

        let mut section = SectionProps::default();
        section.num_columns = columns;
        let (page_w, page_h) = (section.page_w, section.page_h);
        let typesetter = Typesetter::new(section);

        let mut paras = Vec::with_capacity(para_count);
        paras.push(ParaDoc::heading("1. Constitutional & Scientific Foundations", 1));
        for i in 1..para_count {
            let mut p = ParaDoc::new(format!(
                "Paragraph {i}: Multi-column foliation with deterministic widow and orphan control preserves scholarly readability across paginated surfaces."
            ));
            if i == 1 {
                p.footnotes.push("UDHR Article 27 & ICESCR Article 15.".into());
            }
            paras.push(p);
        }
        let pages = typesetter.paginate(&paras, &[]);

        let math_ast = parse_latex(&formula);
        let math_layout = layout_equation(&math_ast, 12.0);

        let item = CitationItem {
            id: "udhr1948".into(),
            title: "Universal Declaration of Human Rights".into(),
            authors: vec![CitationAuthor { family: "United Nations".into(), given: "General Assembly".into() }],
            year: Some(1948),
            locator: Some("Art. 27".into()),
            journal: Some("UN Doc A/810".into()),
            volume: None,
            pages: None,
            doi: None,
        };
        let _bib = Bibliography::generate(std::slice::from_ref(&item), CitationStyle::AuthorDate);
        let cite = Citation { citation_id: "c1".into(), items: vec![item] };
        let inline_cite = cite.format(CitationStyle::AuthorDate, 1);

        let total_pages = pages.len().max(1);
        let sheet_h = (page_h + 24.0) * (total_pages as f32);
        let mut page_svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {page_w:.2} {sheet_h:.2}\" width=\"{page_w:.2}\" height=\"{sheet_h:.2}\">\n"
        );
        for (p_idx, page) in pages.iter().enumerate() {
            let y_off = (p_idx as f32) * (page_h + 24.0);
            let _ = writeln!(
                page_svg,
                "<rect x=\"0\" y=\"{y_off:.2}\" width=\"{page_w:.2}\" height=\"{page_h:.2}\" fill=\"#ffffff\" stroke=\"#94a3b8\" stroke-width=\"1\"/>"
            );
            for placed in page.items.iter().chain(page.footnotes.iter()) {
                match placed {
                    PlacedItem::Line { rect, text: line_txt, font_size } => {
                        let _ = writeln!(
                            page_svg,
                            "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"serif\" font-size=\"{:.2}\" fill=\"#0f172a\">{}</text>",
                            rect.x, y_off + rect.y + font_size, font_size, xml_escape(line_txt)
                        );
                    }
                    PlacedItem::Figure { rect, alt } => {
                        let _ = writeln!(
                            page_svg,
                            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#f1f5f9\" stroke=\"#64748b\"/><title>{}</title>",
                            rect.x, y_off + rect.y, rect.w, rect.h, xml_escape(alt)
                        );
                    }
                    PlacedItem::Footnote { rect, number: fn_num, text: fn_txt } => {
                        let _ = writeln!(
                            page_svg,
                            "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"serif\" font-size=\"9\" fill=\"#334155\">[{fn_num}] {}</text>",
                            rect.x, y_off + rect.y + 9.0, xml_escape(fn_txt)
                        );
                    }
                }
            }
        }
        let (eq_x, eq_y) = (72.0f32, page_h - 48.0);
        for mitem in &math_layout.items {
            match mitem {
                MItem::Text { x, y, text: t, font_size } => {
                    let _ = writeln!(
                        page_svg,
                        "<text x=\"{:.2}\" y=\"{:.2}\" font-family=\"serif\" font-style=\"italic\" font-size=\"{:.2}\" fill=\"#1e293b\">{}</text>",
                        eq_x + x, eq_y + y, font_size, xml_escape(t)
                    );
                }
                MItem::Line { x0, y0, x1, y1, thickness } => {
                    let _ = writeln!(
                        page_svg,
                        "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"#1e293b\" stroke-width=\"{:.2}\"/>",
                        eq_x + x0, eq_y + y0, eq_x + x1, eq_y + y1, thickness
                    );
                }
                MItem::Rect { x, y, w, h } => {
                    let _ = writeln!(
                        page_svg,
                        "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"#1e293b\" stroke-width=\"0.75\"/>",
                        eq_x + x, eq_y + y, w, h
                    );
                }
            }
        }
        let _ = writeln!(
            page_svg,
            "<text x=\"72.00\" y=\"{:.2}\" font-family=\"serif\" font-size=\"10\" fill=\"#475569\">{}</text>",
            page_h - 20.0, xml_escape(&inline_cite)
        );
        page_svg.push_str("</svg>\n");
        let svg_resource = self.encode("svg", page_svg.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("svg_resource".into(), svg_resource);
        out.insert("page_count".into(), Value::I64(pages.len() as i64));
        out.insert("placed_item_count".into(), Value::I64(pages.iter().map(|p| p.items.len() as i64).sum()));
        out.insert("equation_width_pt".into(), Value::F64(math_layout.width as f64));
        out.insert("equation_height_pt".into(), Value::F64(math_layout.height() as f64));
        out.insert("formatted_citation".into(), Value::String(inline_cite));
        Ok(Value::Record(out))
    }

    /// 134: `hypermedia.layout.compile_dataflow_nodegraph` (`layout-nodegraph-compiler` + `layout-presentation-deck`)
    pub(crate) fn compile_dataflow_nodegraph(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["profile", "expression", "morph_progress"])?;
        if let Some(pv) = a.get("profile") {
            let _ = self.profile(pv)?;
        }
        let expr_str = opt_str(a, "expression", "2 * pi * 10")?;
        let morph_t = number(a, "morph_progress", 0.5)?;

        self.charge(256, 4096)?;

        use layout_nodegraph_compiler::{
            DocumentNode, MathContext, NodeGraphCompiler, NodeId, NodeImplementation, NodeInput,
            NodeNetwork, TaggedValue,
        };
        use layout_presentation_deck::{
            LayoutId, Shape, ShapeId, Slide, Xfrm, build_morph_frame, model::types::SlideId,
        };

        let mut net = NodeNetwork::new();
        let (n1, n2, n3) = (NodeId::new(1), NodeId::new(2), NodeId::new(3));
        net.add_node(
            n1,
            DocumentNode {
                name: "ConstA".into(),
                inputs: vec![NodeInput::Value { tagged_value: TaggedValue::F64(10.0), exposed: false }],
                implementation: NodeImplementation::ProtoNode("ConstF64".into()),
                metadata: BTreeMap::new(),
            },
        );
        net.add_node(
            n2,
            DocumentNode {
                name: "ConstA_Dup".into(),
                inputs: vec![NodeInput::Value { tagged_value: TaggedValue::F64(10.0), exposed: false }],
                implementation: NodeImplementation::ProtoNode("ConstF64".into()),
                metadata: BTreeMap::new(),
            },
        );
        net.add_node(
            n3,
            DocumentNode {
                name: "Add".into(),
                inputs: vec![
                    NodeInput::Node { node_id: n1, output_index: 0, lambda: false },
                    NodeInput::Node { node_id: n2, output_index: 0, lambda: false },
                ],
                implementation: NodeImplementation::ProtoNode("AddF64".into()),
                metadata: BTreeMap::new(),
            },
        );
        net.exports = vec![n3];
        let proto = NodeGraphCompiler::default()
            .compile(net)
            .map_err(|e| format!("NodeGraph compile failed: {e}"))?;

        let math_ctx = MathContext::new();
        let math_val = math_ctx.eval(&expr_str).map_err(|e| format!("Math eval failed: {e}"))?;

        let mut s0 = Slide::new(SlideId(1), LayoutId(1));
        s0.shapes.push(Shape::new(ShapeId(1), "!!hero_card", Xfrm::new(0.0, 0.0, 100.0, 50.0)));

        let mut s1 = Slide::new(SlideId(2), LayoutId(1));
        s1.shapes.push(Shape::new(ShapeId(2), "!!hero_card", Xfrm::new(200.0, 100.0, 100.0, 50.0)));

        let clamped_t = morph_t.clamp(0.0, 1.0);
        let morph = build_morph_frame(&s0, &s1, clamped_t);

        let json_payload = serde_json::json!({
            "proto_network": &proto,
            "expression": expr_str,
            "math_result": math_val,
            "morph_progress": clamped_t,
        });
        let json_bytes = serde_json::to_vec_pretty(&json_payload).map_err(|e| e.to_string())?;
        let nodegraph_json_resource = self.encode("json", json_bytes)?;

        let mut morph_svg = String::from(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 360 220\" width=\"360\" height=\"220\">\n\
             <rect x=\"0\" y=\"0\" width=\"360\" height=\"220\" fill=\"#0f172a\"/>\n",
        );
        for (shape, _is_old) in &morph.shapes {
            let op = morph.opacity_of(shape.id);
            let (x, y) = (shape.xfrm.x + 20.0, shape.xfrm.y + 20.0);
            let _ = writeln!(
                morph_svg,
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" rx=\"6\" fill=\"#38bdf8\" fill-opacity=\"{:.3}\" stroke=\"#e0f2fe\" stroke-width=\"1.5\"/>\n\
                 <text x=\"{:.2}\" y=\"{:.2}\" font-family=\"sans-serif\" font-size=\"11\" fill=\"#0f172a\">{}</text>",
                x, y, shape.xfrm.w, shape.xfrm.h, op, x + 8.0, y + shape.xfrm.h * 0.55, xml_escape(&shape.name)
            );
        }
        morph_svg.push_str("</svg>\n");
        let morph_svg_resource = self.encode("svg", morph_svg.into_bytes())?;

        let mut out = BTreeMap::new();
        out.insert("nodegraph_json_resource".into(), nodegraph_json_resource);
        out.insert("morph_svg_resource".into(), morph_svg_resource);
        out.insert("compiled_proto_nodes".into(), Value::I64(proto.nodes.len() as i64));
        out.insert("math_result".into(), Value::F64(math_val));
        out.insert("morph_interpolated_shapes".into(), Value::I64(morph.shapes.len() as i64));
        Ok(Value::Record(out))
    }
}

//! Semantic HTML5 / XHTML importer and exporter for FlowDoc.

use crate::flow::model::{
    FlowBlock, FlowCell, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowListInfo, FlowPara, FlowRow,
    FlowTable,
};
use crate::xml::dom::{escape_xml, parse_xml, XEl, XNode};

/// Serialize a `FlowDoc` to clean, semantic HTML5.
pub fn to_html(doc: &FlowDoc) -> String {
    let mut out = String::new();
    out.push_str("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n");
    if let Some(title) = &doc.title {
        out.push_str("<title>");
        escape_xml(title, &mut out);
        out.push_str("</title>\n");
    }
    out.push_str("</head>\n<body>\n");

    for block in &doc.blocks {
        match block {
            FlowBlock::Para(p) => {
                let tag = match p.kind {
                    FlowKind::Heading(1) => "h1",
                    FlowKind::Heading(2) => "h2",
                    FlowKind::Heading(3) => "h3",
                    FlowKind::Heading(4) => "h4",
                    FlowKind::Heading(5) => "h5",
                    FlowKind::Heading(_) => "h6",
                    FlowKind::Title => "h1",
                    FlowKind::Quote => "blockquote",
                    FlowKind::Code => "pre",
                    FlowKind::Rule => {
                        out.push_str("<hr>\n");
                        continue;
                    }
                    FlowKind::Normal => {
                        if p.list.is_some() {
                            "li"
                        } else {
                            "p"
                        }
                    }
                };

                out.push('<');
                out.push_str(tag);
                out.push('>');
                write_html_inlines(&p.inlines, &mut out);
                out.push_str("</");
                out.push_str(tag);
                out.push_str(">\n");
            }
            FlowBlock::Table(table) => {
                out.push_str("<table>\n");
                for (r_idx, row) in table.rows.iter().enumerate() {
                    out.push_str("  <tr>\n");
                    let cell_tag = if r_idx == 0 { "th" } else { "td" };
                    for cell in &row.cells {
                        out.push_str("    <");
                        out.push_str(cell_tag);
                        if cell.col_span > 1 {
                            out.push_str(&format!(" colspan=\"{}\"", cell.col_span));
                        }
                        if cell.row_span > 1 {
                            out.push_str(&format!(" rowspan=\"{}\"", cell.row_span));
                        }
                        out.push('>');
                        for b in &cell.blocks {
                            if let FlowBlock::Para(cp) = b {
                                write_html_inlines(&cp.inlines, &mut out);
                            }
                        }
                        out.push_str("</");
                        out.push_str(cell_tag);
                        out.push_str(">\n");
                    }
                    out.push_str("  </tr>\n");
                }
                out.push_str("</table>\n");
            }
        }
    }

    out.push_str("</body>\n</html>\n");
    out
}

fn write_html_inlines(inlines: &[FlowInline], out: &mut String) {
    for inline in inlines {
        match inline {
            FlowInline::Text(t, fmt) => {
                if let Some(link) = &fmt.link {
                    out.push_str("<a href=\"");
                    escape_xml(link, out);
                    out.push_str("\">");
                }
                if fmt.bold {
                    out.push_str("<strong>");
                }
                if fmt.italic {
                    out.push_str("<em>");
                }
                if fmt.code {
                    out.push_str("<code>");
                }
                if fmt.underline {
                    out.push_str("<u>");
                }
                if fmt.strike {
                    out.push_str("<s>");
                }
                escape_xml(t, out);
                if fmt.strike {
                    out.push_str("</s>");
                }
                if fmt.underline {
                    out.push_str("</u>");
                }
                if fmt.code {
                    out.push_str("</code>");
                }
                if fmt.italic {
                    out.push_str("</em>");
                }
                if fmt.bold {
                    out.push_str("</strong>");
                }
                if fmt.link.is_some() {
                    out.push_str("</a>");
                }
            }
            FlowInline::Math { expr, display } => {
                if *display {
                    out.push_str("<div class=\"math display\">\\[");
                    escape_xml(expr, out);
                    out.push_str("\\]</div>");
                } else {
                    out.push_str("<span class=\"math inline\">\\(");
                    escape_xml(expr, out);
                    out.push_str("\\)</span>");
                }
            }
            FlowInline::Image { src, alt, .. } => {
                out.push_str("<img src=\"");
                escape_xml(src, out);
                out.push_str("\" alt=\"");
                escape_xml(alt, out);
                out.push_str("\">");
            }
            FlowInline::Anchor(name) => {
                out.push_str("<a id=\"");
                escape_xml(name, out);
                out.push_str("\"></a>");
            }
        }
    }
}

/// Parse HTML string into a `FlowDoc`.
pub fn parse_html(input: &str) -> Result<FlowDoc, String> {
    // Wrap with root if fragment
    let wrapped = if !input.contains("<html") {
        format!("<html><body>{}</body></html>", input)
    } else {
        input.to_string()
    };

    let root = parse_xml(&wrapped).map_err(|e| format!("HTML parse error: {}", e))?;
    let mut doc = FlowDoc::new();

    // Find body
    let body = find_tag_recursive(&root, "body").unwrap_or(&root);
    extract_flow_blocks(body, &mut doc);

    Ok(doc)
}

fn find_tag_recursive<'a>(el: &'a XEl, tag: &str) -> Option<&'a XEl> {
    if el.local().eq_ignore_ascii_case(tag) {
        return Some(el);
    }
    for child in &el.children {
        if let XNode::Element(cel) = child {
            if let Some(found) = find_tag_recursive(cel, tag) {
                return Some(found);
            }
        }
    }
    None
}

fn extract_flow_blocks(parent: &XEl, doc: &mut FlowDoc) {
    for node in &parent.children {
        if let XNode::Element(el) = node {
            let tag = el.local().to_ascii_lowercase();
            match tag.as_str() {
                "h1" => doc.push_para(FlowPara::with_text(FlowKind::Heading(1), el.text_content())),
                "h2" => doc.push_para(FlowPara::with_text(FlowKind::Heading(2), el.text_content())),
                "h3" => doc.push_para(FlowPara::with_text(FlowKind::Heading(3), el.text_content())),
                "h4" => doc.push_para(FlowPara::with_text(FlowKind::Heading(4), el.text_content())),
                "h5" => doc.push_para(FlowPara::with_text(FlowKind::Heading(5), el.text_content())),
                "h6" => doc.push_para(FlowPara::with_text(FlowKind::Heading(6), el.text_content())),
                "p" => {
                    let mut p = FlowPara::new(FlowKind::Normal);
                    extract_inlines_from_html(el, &mut p.inlines, FlowFmt::default());
                    doc.push_para(p);
                }
                "blockquote" => {
                    let mut p = FlowPara::new(FlowKind::Quote);
                    extract_inlines_from_html(el, &mut p.inlines, FlowFmt::default());
                    doc.push_para(p);
                }
                "pre" => {
                    let mut p = FlowPara::new(FlowKind::Code);
                    p.inlines.push(FlowInline::Text(
                        el.text_content(),
                        FlowFmt {
                            code: true,
                            ..Default::default()
                        },
                    ));
                    doc.push_para(p);
                }
                "hr" => doc.push_para(FlowPara::new(FlowKind::Rule)),
                "table" => {
                    let mut rows = Vec::new();
                    for tr in el.find_children("tr") {
                        let mut cells = Vec::new();
                        for child in &tr.children {
                            if let XNode::Element(td) = child {
                                if td.local().eq_ignore_ascii_case("td") || td.local().eq_ignore_ascii_case("th") {
                                    cells.push(FlowCell::with_text(td.text_content()));
                                }
                            }
                        }
                        if !cells.is_empty() {
                            rows.push(FlowRow { cells });
                        }
                    }
                    if !rows.is_empty() {
                        let cols = rows[0].cells.len();
                        doc.push_table(FlowTable {
                            col_widths: vec![100.0; cols],
                            rows,
                        });
                    }
                }
                _ => {
                    // Recurse for containers like div, main, section
                    extract_flow_blocks(el, doc);
                }
            }
        }
    }
}

fn extract_inlines_from_html(el: &XEl, out: &mut Vec<FlowInline>, mut fmt: FlowFmt) {
    let tag = el.local().to_ascii_lowercase();
    match tag.as_str() {
        "b" | "strong" => fmt.bold = true,
        "i" | "em" => fmt.italic = true,
        "u" => fmt.underline = true,
        "s" | "strike" => fmt.strike = true,
        "code" => fmt.code = true,
        "a" => {
            if let Some(href) = el.attr("href") {
                fmt.link = Some(href.to_string());
            }
        }
        _ => {}
    }

    for child in &el.children {
        match child {
            XNode::Text(t) => {
                if !t.is_empty() {
                    out.push(FlowInline::Text(t.clone(), fmt.clone()));
                }
            }
            XNode::Element(cel) => {
                extract_inlines_from_html(cel, out, fmt.clone());
            }
        }
    }
}

//! CommonMark and GitHub-Flavored Markdown (GFM) parser and serializer.
//! Supports headings, lists, blockquotes, code blocks, GFM tables, and LaTeX math ($x$, $$...$$).

use crate::flow::model::{
    FlowBlock, FlowCell, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowListInfo, FlowPara, FlowRow,
    FlowTable,
};

/// Parse a Markdown string into a `FlowDoc`.
pub fn parse_markdown(input: &str) -> FlowDoc {
    let mut doc = FlowDoc::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
            continue;
        }

        // 1. Math block ($$...$$)
        if trimmed.starts_with("$$") {
            let mut math_content = String::new();
            if trimmed.len() > 2 && trimmed.ends_with("$$") && trimmed.len() > 4 {
                math_content = trimmed[2..trimmed.len() - 2].trim().to_string();
                i += 1;
            } else {
                math_content.push_str(trimmed.trim_start_matches('$'));
                i += 1;
                while i < lines.len() {
                    let cur = lines[i];
                    if cur.trim().ends_with("$$") {
                        math_content.push('\n');
                        math_content.push_str(cur.trim().trim_end_matches('$'));
                        i += 1;
                        break;
                    } else {
                        math_content.push('\n');
                        math_content.push_str(cur);
                        i += 1;
                    }
                }
            }
            let mut p = FlowPara::new(FlowKind::Normal);
            p.inlines.push(FlowInline::Math {
                expr: math_content.trim().to_string(),
                display: true,
            });
            doc.push_para(p);
            continue;
        }

        // 2. Fenced code block (``` ... ```)
        if trimmed.starts_with("```") {
            let _lang = trimmed.trim_start_matches('`').trim();
            let mut code_body = String::new();
            i += 1;
            while i < lines.len() {
                let cur = lines[i];
                if cur.trim().starts_with("```") {
                    i += 1;
                    break;
                }
                if !code_body.is_empty() {
                    code_body.push('\n');
                }
                code_body.push_str(cur);
                i += 1;
            }
            let mut p = FlowPara::new(FlowKind::Code);
            p.inlines.push(FlowInline::Text(
                code_body,
                FlowFmt {
                    code: true,
                    ..Default::default()
                },
            ));
            doc.push_para(p);
            continue;
        }

        // 3. Headings (# Heading)
        if trimmed.starts_with('#') {
            let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
            let heading_text = trimmed[hash_count..].trim();
            let level = (hash_count as u8).min(6);
            let mut p = FlowPara::new(FlowKind::Heading(level));
            p.inlines = parse_inlines(heading_text);
            doc.push_para(p);
            i += 1;
            continue;
        }

        // 4. Horizontal rule (--- or ***)
        if (trimmed.starts_with("---") || trimmed.starts_with("***")) && trimmed.chars().all(|c| c == '-' || c == '*' || c.is_whitespace()) {
            doc.push_para(FlowPara::new(FlowKind::Rule));
            i += 1;
            continue;
        }

        // 5. Blockquote (> Quote)
        if trimmed.starts_with('>') {
            let quote_text = trimmed.trim_start_matches('>').trim();
            let mut p = FlowPara::new(FlowKind::Quote);
            p.inlines = parse_inlines(quote_text);
            doc.push_para(p);
            i += 1;
            continue;
        }

        // 6. GFM Table (| Col 1 | Col 2 |)
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            let mut table_rows = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('|') && lines[i].trim().ends_with('|') {
                let row_str = lines[i].trim();
                // Check if delimiter row (| --- | --- |)
                let inner = &row_str[1..row_str.len() - 1];
                let is_sep = inner.chars().all(|c| c == '-' || c == ':' || c == '|' || c.is_whitespace());
                if !is_sep {
                    let cell_texts: Vec<&str> = inner.split('|').map(|s| s.trim()).collect();
                    let cells = cell_texts
                        .into_iter()
                        .map(|ct| FlowCell::with_text(ct))
                        .collect();
                    table_rows.push(FlowRow { cells });
                }
                i += 1;
            }
            if !table_rows.is_empty() {
                let col_count = table_rows[0].cells.len();
                let col_widths = vec![100.0; col_count];
                doc.push_table(FlowTable {
                    col_widths,
                    rows: table_rows,
                });
            }
            continue;
        }

        // 7. Unordered / Ordered list
        let is_ul = trimmed.starts_with("- ") || trimmed.starts_with("* ");
        let is_ol = trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) && trimmed.contains(". ");

        if is_ul || is_ol {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();
            let level = (indent / 2).min(8) as u8;
            let item_text = if is_ul {
                trimmed[2..].trim()
            } else {
                let dot_idx = trimmed.find(". ").unwrap_or(1);
                trimmed[dot_idx + 2..].trim()
            };
            let mut p = FlowPara::new(FlowKind::Normal);
            p.list = Some(FlowListInfo {
                ordered: is_ol,
                level,
            });
            p.inlines = parse_inlines(item_text);
            doc.push_para(p);
            i += 1;
            continue;
        }

        // 8. Normal paragraph
        let mut p = FlowPara::new(FlowKind::Normal);
        p.inlines = parse_inlines(trimmed);
        doc.push_para(p);
        i += 1;
    }

    doc
}

/// Parse inline formatting: bold, italic, code, link, inline math.
pub fn parse_inlines(input: &str) -> Vec<FlowInline> {
    let mut out = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut cur_text = String::new();

    while i < chars.len() {
        // Inline math $...$
        if chars[i] == '$' && (i + 1 < chars.len() && chars[i + 1] != '$') {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '$') {
                if !cur_text.is_empty() {
                    out.push(FlowInline::plain_text(std::mem::take(&mut cur_text)));
                }
                let math_expr: String = chars[i + 1..i + 1 + end].iter().collect();
                out.push(FlowInline::Math {
                    expr: math_expr,
                    display: false,
                });
                i = i + 1 + end + 1;
                continue;
            }
        }

        // Inline code `...`
        if chars[i] == '`' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                if !cur_text.is_empty() {
                    out.push(FlowInline::plain_text(std::mem::take(&mut cur_text)));
                }
                let code_text: String = chars[i + 1..i + 1 + end].iter().collect();
                out.push(FlowInline::Text(
                    code_text,
                    FlowFmt {
                        code: true,
                        ..Default::default()
                    },
                ));
                i = i + 1 + end + 1;
                continue;
            }
        }

        // Bold **...**
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            let rest = &chars[i + 2..];
            if let Some(end) = rest.windows(2).position(|w| w[0] == '*' && w[1] == '*') {
                if !cur_text.is_empty() {
                    out.push(FlowInline::plain_text(std::mem::take(&mut cur_text)));
                }
                let bold_text: String = rest[..end].iter().collect();
                out.push(FlowInline::Text(
                    bold_text,
                    FlowFmt {
                        bold: true,
                        ..Default::default()
                    },
                ));
                i = i + 2 + end + 2;
                continue;
            }
        }

        // Italic *...*
        if chars[i] == '*' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '*') {
                if !cur_text.is_empty() {
                    out.push(FlowInline::plain_text(std::mem::take(&mut cur_text)));
                }
                let it_text: String = chars[i + 1..i + 1 + end].iter().collect();
                out.push(FlowInline::Text(
                    it_text,
                    FlowFmt {
                        italic: true,
                        ..Default::default()
                    },
                ));
                i = i + 1 + end + 1;
                continue;
            }
        }

        // Markdown Link [text](url)
        if chars[i] == '[' {
            if let Some(close_bracket) = chars[i + 1..].iter().position(|&c| c == ']') {
                let link_text_end = i + 1 + close_bracket;
                if link_text_end + 1 < chars.len() && chars[link_text_end + 1] == '(' {
                    if let Some(close_paren) = chars[link_text_end + 2..].iter().position(|&c| c == ')') {
                        let link_url_end = link_text_end + 2 + close_paren;
                        if !cur_text.is_empty() {
                            out.push(FlowInline::plain_text(std::mem::take(&mut cur_text)));
                        }
                        let text_val: String = chars[i + 1..link_text_end].iter().collect();
                        let url_val: String = chars[link_text_end + 2..link_url_end].iter().collect();
                        out.push(FlowInline::Text(
                            text_val,
                            FlowFmt {
                                link: Some(url_val),
                                ..Default::default()
                            },
                        ));
                        i = link_url_end + 1;
                        continue;
                    }
                }
            }
        }

        cur_text.push(chars[i]);
        i += 1;
    }

    if !cur_text.is_empty() {
        out.push(FlowInline::plain_text(cur_text));
    }

    out
}

/// Serialize a `FlowDoc` to CommonMark / GFM Markdown.
pub fn to_markdown(doc: &FlowDoc) -> String {
    let mut out = String::new();

    for block in &doc.blocks {
        match block {
            FlowBlock::Para(para) => {
                match para.kind {
                    FlowKind::Heading(level) => {
                        for _ in 0..level.clamp(1, 6) {
                            out.push('#');
                        }
                        out.push(' ');
                        write_inlines(&para.inlines, &mut out);
                        out.push_str("\n\n");
                    }
                    FlowKind::Title => {
                        out.push_str("# ");
                        write_inlines(&para.inlines, &mut out);
                        out.push_str("\n\n");
                    }
                    FlowKind::Quote => {
                        out.push_str("> ");
                        write_inlines(&para.inlines, &mut out);
                        out.push_str("\n\n");
                    }
                    FlowKind::Code => {
                        out.push_str("```\n");
                        write_inlines(&para.inlines, &mut out);
                        out.push_str("\n```\n\n");
                    }
                    FlowKind::Rule => {
                        out.push_str("---\n\n");
                    }
                    FlowKind::Normal => {
                        if let Some(list) = &para.list {
                            for _ in 0..list.level {
                                out.push_str("  ");
                            }
                            if list.ordered {
                                out.push_str("1. ");
                            } else {
                                out.push_str("- ");
                            }
                            write_inlines(&para.inlines, &mut out);
                            out.push('\n');
                        } else {
                            write_inlines(&para.inlines, &mut out);
                            out.push_str("\n\n");
                        }
                    }
                }
            }
            FlowBlock::Table(table) => {
                if table.rows.is_empty() {
                    continue;
                }
                // Header row
                out.push('|');
                for cell in &table.rows[0].cells {
                    out.push(' ');
                    out.push_str(&cell_text(cell));
                    out.push_str(" |");
                }
                out.push('\n');

                // Separator row
                out.push('|');
                for _ in &table.rows[0].cells {
                    out.push_str(" --- |");
                }
                out.push('\n');

                // Data rows
                for row in table.rows.iter().skip(1) {
                    out.push('|');
                    for cell in &row.cells {
                        out.push(' ');
                        out.push_str(&cell_text(cell));
                        out.push_str(" |");
                    }
                    out.push('\n');
                }
                out.push('\n');
            }
        }
    }

    out.trim_end().to_string()
}

fn cell_text(cell: &FlowCell) -> String {
    let mut s = String::new();
    for block in &cell.blocks {
        if let FlowBlock::Para(p) = block {
            if !s.is_empty() {
                s.push(' ');
            }
            s.push_str(&p.plain_text());
        }
    }
    s
}

fn write_inlines(inlines: &[FlowInline], out: &mut String) {
    for inline in inlines {
        match inline {
            FlowInline::Text(t, fmt) => {
                if let Some(url) = &fmt.link {
                    out.push('[');
                }
                if fmt.bold {
                    out.push_str("**");
                }
                if fmt.italic {
                    out.push('*');
                }
                if fmt.code {
                    out.push('`');
                }
                out.push_str(t);
                if fmt.code {
                    out.push('`');
                }
                if fmt.italic {
                    out.push('*');
                }
                if fmt.bold {
                    out.push_str("**");
                }
                if let Some(url) = &fmt.link {
                    out.push_str("](");
                    out.push_str(url);
                    out.push(')');
                }
            }
            FlowInline::Math { expr, display } => {
                if *display {
                    out.push_str("$$\n");
                    out.push_str(expr);
                    out.push_str("\n$$");
                } else {
                    out.push('$');
                    out.push_str(expr);
                    out.push('$');
                }
            }
            FlowInline::Image { src, alt, .. } => {
                out.push_str("![");
                out.push_str(alt);
                out.push_str("](");
                out.push_str(src);
                out.push(')');
            }
            FlowInline::Anchor(name) => {
                out.push_str("<a name=\"");
                out.push_str(name);
                out.push_str("\"></a>");
            }
        }
    }
}

//! LaTeX (.tex) document importer and exporter for FlowDoc.

use crate::flow::model::{
    FlowBlock, FlowCell, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowListInfo, FlowPara, FlowRow,
    FlowTable,
};

/// Export a `FlowDoc` to standard LaTeX source.
pub fn to_latex(doc: &FlowDoc) -> String {
    let mut out = String::new();
    out.push_str("\\documentclass{article}\n\\usepackage[utf8]{inputenc}\n\\usepackage{amsmath,amssymb}\n\\usepackage{graphicx}\n\\usepackage{hyperref}\n\n");
    if let Some(title) = &doc.title {
        out.push_str(&format!("\\title{{{}}}\n", escape_latex(title)));
    }
    if let Some(author) = &doc.author {
        out.push_str(&format!("\\author{{{}}}\n", escape_latex(author)));
    }
    out.push_str("\\begin{document}\n");
    if doc.title.is_some() {
        out.push_str("\\maketitle\n\n");
    }

    for block in &doc.blocks {
        match block {
            FlowBlock::Para(p) => match p.kind {
                FlowKind::Heading(1) => {
                    out.push_str("\\section{");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("}\n\n");
                }
                FlowKind::Heading(2) => {
                    out.push_str("\\subsection{");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("}\n\n");
                }
                FlowKind::Heading(3) => {
                    out.push_str("\\subsubsection{");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("}\n\n");
                }
                FlowKind::Heading(_) => {
                    out.push_str("\\paragraph{");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("}\n\n");
                }
                FlowKind::Title => {
                    out.push_str("{\\Huge\\bfseries ");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("}\n\n");
                }
                FlowKind::Quote => {
                    out.push_str("\\begin{quote}\n");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("\n\\end{quote}\n\n");
                }
                FlowKind::Code => {
                    out.push_str("\\begin{verbatim}\n");
                    write_latex_inlines(&p.inlines, &mut out);
                    out.push_str("\n\\end{verbatim}\n\n");
                }
                FlowKind::Rule => {
                    out.push_str("\\hrulefill\n\n");
                }
                FlowKind::Normal => {
                    if let Some(list) = &p.list {
                        let env = if list.ordered { "enumerate" } else { "itemize" };
                        out.push_str(&format!("\\begin{{{}}}\n\\item ", env));
                        write_latex_inlines(&p.inlines, &mut out);
                        out.push_str(&format!("\n\\end{{{}}}\n\n", env));
                    } else {
                        write_latex_inlines(&p.inlines, &mut out);
                        out.push_str("\n\n");
                    }
                }
            },
            FlowBlock::Table(table) => {
                if table.rows.is_empty() {
                    continue;
                }
                let cols = table.rows[0].cells.len();
                let col_spec = "c".repeat(cols);
                out.push_str(&format!("\\begin{{tabular}}{{|{}:|}}\n\\hline\n", col_spec.chars().map(|c| format!("{}|", c)).collect::<String>()));
                for row in &table.rows {
                    for (c_idx, cell) in row.cells.iter().enumerate() {
                        if c_idx > 0 {
                            out.push_str(" & ");
                        }
                        for b in &cell.blocks {
                            if let FlowBlock::Para(cp) = b {
                                write_latex_inlines(&cp.inlines, &mut out);
                            }
                        }
                    }
                    out.push_str(" \\\\\n\\hline\n");
                }
                out.push_str("\\end{tabular}\n\n");
            }
        }
    }

    out.push_str("\\end{document}\n");
    out
}

fn write_latex_inlines(inlines: &[FlowInline], out: &mut String) {
    for inline in inlines {
        match inline {
            FlowInline::Text(t, fmt) => {
                if let Some(url) = &fmt.link {
                    out.push_str(&format!("\\href{{{}}}{{", url));
                }
                if fmt.bold {
                    out.push_str("\\textbf{");
                }
                if fmt.italic {
                    out.push_str("\\textit{");
                }
                if fmt.code {
                    out.push_str("\\texttt{");
                }
                out.push_str(&escape_latex(t));
                if fmt.code {
                    out.push('}');
                }
                if fmt.italic {
                    out.push('}');
                }
                if fmt.bold {
                    out.push('}');
                }
                if fmt.link.is_some() {
                    out.push('}');
                }
            }
            FlowInline::Math { expr, display } => {
                if *display {
                    out.push_str("\n\\begin{equation}\n");
                    out.push_str(expr);
                    out.push_str("\n\\end{equation}\n");
                } else {
                    out.push('$');
                    out.push_str(expr);
                    out.push('$');
                }
            }
            FlowInline::Image { src, alt, .. } => {
                out.push_str(&format!("\\includegraphics[alt={{{}}}]{{{}}}", alt, src));
            }
            FlowInline::Anchor(name) => {
                out.push_str(&format!("\\label{{{}}}", name));
            }
        }
    }
}

pub fn escape_latex(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("\\&"),
            '%' => out.push_str("\\%"),
            '$' => out.push_str("\\$"),
            '#' => out.push_str("\\#"),
            '_' => out.push_str("\\_"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '~' => out.push_str("\\textasciitilde{}"),
            '^' => out.push_str("\\textasciicircum{}"),
            '\\' => out.push_str("\\textbackslash{}"),
            _ => out.push(c),
        }
    }
    out
}

/// Simple parser for LaTeX `.tex` documents into `FlowDoc`.
pub fn parse_latex(input: &str) -> FlowDoc {
    let mut doc = FlowDoc::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('%') || line.starts_with("\\documentclass") || line.starts_with("\\usepackage") || line.starts_with("\\begin{document}") || line.starts_with("\\end{document}") {
            i += 1;
            continue;
        }

        if line.starts_with("\\section{") {
            let title = line.trim_start_matches("\\section{").trim_end_matches('}');
            doc.push_para(FlowPara::with_text(FlowKind::Heading(1), title));
            i += 1;
            continue;
        }
        if line.starts_with("\\subsection{") {
            let title = line.trim_start_matches("\\subsection{").trim_end_matches('}');
            doc.push_para(FlowPara::with_text(FlowKind::Heading(2), title));
            i += 1;
            continue;
        }
        if line.starts_with("\\subsubsection{") {
            let title = line.trim_start_matches("\\subsubsection{").trim_end_matches('}');
            doc.push_para(FlowPara::with_text(FlowKind::Heading(3), title));
            i += 1;
            continue;
        }

        // Equation environment
        if line.starts_with("\\begin{equation}") {
            let mut math_content = String::new();
            i += 1;
            while i < lines.len() {
                let cur = lines[i].trim();
                if cur.starts_with("\\end{equation}") {
                    i += 1;
                    break;
                }
                if !math_content.is_empty() {
                    math_content.push('\n');
                }
                math_content.push_str(cur);
                i += 1;
            }
            let mut p = FlowPara::new(FlowKind::Normal);
            p.inlines.push(FlowInline::Math {
                expr: math_content,
                display: true,
            });
            doc.push_para(p);
            continue;
        }

        // Normal paragraph
        let mut p = FlowPara::new(FlowKind::Normal);
        p.inlines.push(FlowInline::plain_text(line));
        doc.push_para(p);
        i += 1;
    }

    doc
}

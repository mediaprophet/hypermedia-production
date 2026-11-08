//! Rich Text Format (RTF 1.5) reader and writer for FlowDoc.

use crate::flow::model::{
    FlowBlock, FlowDoc, FlowFmt, FlowInline, FlowKind, FlowPara, FlowRow, FlowTable,
};

/// Serialize a `FlowDoc` to standard RTF 1.5.
pub fn to_rtf(doc: &FlowDoc) -> String {
    let mut out = String::new();
    out.push_str("{\\rtf1\\ansi\\ansicpg1252\\deff0\\nouicompat\\deflang1033\n");
    out.push_str("{\\fonttbl{\\f0\\fnil\\fcharset0 Calibri;}{\\f1\\fnil\\fcharset0 Courier New;}}\n");
    out.push_str("{\\colortbl ;\\red0\\green0\\blue0;\\red0\\green0\\blue255;}\n");
    out.push_str("\\viewkind4\\uc1\\pard\\sa200\\sl276\\slmult1\\f0\\fs22\\lang9\n");

    for block in &doc.blocks {
        match block {
            FlowBlock::Para(p) => {
                out.push_str("\\pard ");
                match p.kind {
                    FlowKind::Heading(1) => out.push_str("\\b\\fs36 "),
                    FlowKind::Heading(2) => out.push_str("\\b\\fs28 "),
                    FlowKind::Heading(3) => out.push_str("\\b\\fs24 "),
                    FlowKind::Heading(_) => out.push_str("\\b\\fs22 "),
                    FlowKind::Title => out.push_str("\\b\\fs40 "),
                    FlowKind::Code => out.push_str("\\f1\\fs20 "),
                    _ => out.push_str("\\fs22 "),
                }
                write_rtf_inlines(&p.inlines, &mut out);
                out.push_str("\\par\n");
            }
            FlowBlock::Table(table) => {
                for row in &table.rows {
                    out.push_str("\\trowd\\trgaph108\\trleft-108");
                    let mut cur_x = 0;
                    for _ in &row.cells {
                        cur_x += 2000;
                        out.push_str(&format!("\\cellx{}", cur_x));
                    }
                    out.push('\n');
                    for cell in &row.cells {
                        out.push_str("\\pard\\intbl ");
                        for b in &cell.blocks {
                            if let FlowBlock::Para(cp) = b {
                                write_rtf_inlines(&cp.inlines, &mut out);
                            }
                        }
                        out.push_str("\\cell\n");
                    }
                    out.push_str("\\row\n");
                }
            }
        }
    }

    out.push('}');
    out
}

fn write_rtf_inlines(inlines: &[FlowInline], out: &mut String) {
    for inline in inlines {
        match inline {
            FlowInline::Text(t, fmt) => {
                if fmt.bold {
                    out.push_str("\\b ");
                }
                if fmt.italic {
                    out.push_str("\\i ");
                }
                if fmt.underline {
                    out.push_str("\\ul ");
                }
                if fmt.strike {
                    out.push_str("\\strike ");
                }
                if fmt.code {
                    out.push_str("\\f1 ");
                }

                escape_rtf(t, out);

                if fmt.code {
                    out.push_str("\\f0 ");
                }
                if fmt.strike {
                    out.push_str("\\strike0 ");
                }
                if fmt.underline {
                    out.push_str("\\ulnone ");
                }
                if fmt.italic {
                    out.push_str("\\i0 ");
                }
                if fmt.bold {
                    out.push_str("\\b0 ");
                }
            }
            FlowInline::Math { expr, .. } => {
                out.push_str("\\i ");
                escape_rtf(expr, out);
                out.push_str("\\i0 ");
            }
            FlowInline::Image { alt, .. } => {
                escape_rtf(alt, out);
            }
            FlowInline::Anchor(_) => {}
        }
    }
}

fn escape_rtf(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '\n' => out.push_str("\\line "),
            _ => {
                if (c as u32) < 128 {
                    out.push(c);
                } else {
                    out.push_str(&format!("\\u{}?", c as i32));
                }
            }
        }
    }
}

/// Simple parser for RTF 1.5 files into `FlowDoc`.
pub fn parse_rtf(input: &str) -> Result<FlowDoc, String> {
    if !input.starts_with("{\\rtf") {
        return Err("Not an RTF file".into());
    }

    let mut doc = FlowDoc::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut cur_para = FlowPara::new(FlowKind::Normal);
    let mut cur_text = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut underline = false;

    while i < chars.len() {
        let c = chars[i];
        if c == '{' || c == '}' {
            i += 1;
            continue;
        }
        if c == '\\' {
            i += 1;
            if i >= chars.len() {
                break;
            }
            if chars[i] == '\\' || chars[i] == '{' || chars[i] == '}' {
                cur_text.push(chars[i]);
                i += 1;
                continue;
            }

            // Read control word
            let mut word = String::new();
            while i < chars.len() && chars[i].is_ascii_alphabetic() {
                word.push(chars[i]);
                i += 1;
            }
            // Read optional numeric argument
            let mut num_str = String::new();
            let mut is_neg = false;
            if i < chars.len() && chars[i] == '-' {
                is_neg = true;
                i += 1;
            }
            while i < chars.len() && chars[i].is_ascii_digit() {
                num_str.push(chars[i]);
                i += 1;
            }
            let _num: Option<i32> = if !num_str.is_empty() {
                let v = num_str.parse::<i32>().unwrap_or(0);
                Some(if is_neg { -v } else { v })
            } else {
                None
            };

            // Optional space delimiter after control word
            if i < chars.len() && chars[i] == ' ' {
                i += 1;
            }

            match word.as_str() {
                "par" => {
                    if !cur_text.is_empty() {
                        cur_para.inlines.push(FlowInline::Text(
                            std::mem::take(&mut cur_text),
                            FlowFmt {
                                bold,
                                italic,
                                underline,
                                ..Default::default()
                            },
                        ));
                    }
                    if !cur_para.inlines.is_empty() {
                        doc.push_para(std::mem::take(&mut cur_para));
                    }
                }
                "b" => bold = _num != Some(0),
                "i" => italic = _num != Some(0),
                "ul" => underline = true,
                "ulnone" => underline = false,
                "u" => {
                    if let Some(code) = _num {
                        if let Some(ch) = char::from_u32(code as u32) {
                            cur_text.push(ch);
                        }
                    }
                    // Skip replacement char
                    if i < chars.len() && chars[i] == '?' {
                        i += 1;
                    }
                }
                _ => {}
            }
            continue;
        }

        if c != '\r' && c != '\n' {
            cur_text.push(c);
        }
        i += 1;
    }

    if !cur_text.is_empty() {
        cur_para.inlines.push(FlowInline::Text(
            cur_text,
            FlowFmt {
                bold,
                italic,
                underline,
                ..Default::default()
            },
        ));
    }
    if !cur_para.inlines.is_empty() {
        doc.push_para(cur_para);
    }

    Ok(doc)
}

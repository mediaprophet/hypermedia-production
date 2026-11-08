//! UnicodeMath / OMML linear format converter.

use crate::equation::ast::*;

pub fn parse_linear(src: &str) -> Arg {
    let mut nodes = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c == '(' {
            let mut depth = 1usize;
            let mut j = i + 1;
            while j < chars.len() && depth > 0 {
                if chars[j] == '(' { depth += 1; }
                else if chars[j] == ')' { depth -= 1; }
                j += 1;
            }
            let inner_str: String = chars[i + 1..j - 1].iter().collect();
            let inner = parse_linear(&inner_str);

            if j < chars.len() && chars[j] == '/' {
                // Fraction: (num)/(den)
                j += 1;
                if j < chars.len() && chars[j] == '(' {
                    let mut d_depth = 1usize;
                    let mut k = j + 1;
                    while k < chars.len() && d_depth > 0 {
                        if chars[k] == '(' { d_depth += 1; }
                        else if chars[k] == ')' { d_depth -= 1; }
                        k += 1;
                    }
                    let den_str: String = chars[j + 1..k - 1].iter().collect();
                    let den = parse_linear(&den_str);
                    nodes.push(MNode::Frac { kind: FracKind::Bar, num: inner, den });
                    i = k;
                    continue;
                }
            }

            nodes.push(MNode::Delim {
                beg: Some('('),
                end: Some(')'),
                elems: vec![inner],
            });
            i = j;
        } else if c == '^' || c == '_' {
            let is_sup = c == '^';
            i += 1;
            let mut script_str = String::new();
            if i < chars.len() && chars[i] == '(' {
                i += 1;
                while i < chars.len() && chars[i] != ')' {
                    script_str.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() { i += 1; }
            } else if i < chars.len() {
                script_str.push(chars[i]);
                i += 1;
            }
            let script_nodes = parse_linear(&script_str);
            let prev = nodes.pop().unwrap_or_else(|| MNode::Run(MRun::new("")));
            if is_sup {
                nodes.push(MNode::Script {
                    kind: ScriptKind::Sup,
                    base: vec![prev],
                    sub: vec![],
                    sup: script_nodes,
                });
            } else {
                nodes.push(MNode::Script {
                    kind: ScriptKind::Sub,
                    base: vec![prev],
                    sub: script_nodes,
                    sup: vec![],
                });
            }
        } else {
            let mut text = String::new();
            while i < chars.len() && chars[i] != '(' && chars[i] != '^' && chars[i] != '_' {
                text.push(chars[i]);
                i += 1;
            }
            nodes.push(MNode::Run(MRun::new(text)));
        }
    }
    nodes
}

pub fn to_linear(nodes: &[MNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        match node {
            MNode::Run(r) => out.push_str(&r.text),
            MNode::Frac { num, den, .. } => {
                out.push('(');
                out.push_str(&to_linear(num));
                out.push_str(")/(");
                out.push_str(&to_linear(den));
                out.push(')');
            }
            MNode::Script { kind, base, sub, sup } => {
                out.push_str(&to_linear(base));
                match kind {
                    ScriptKind::Sup => {
                        out.push('^');
                        out.push_str(&to_linear(sup));
                    }
                    ScriptKind::Sub => {
                        out.push('_');
                        out.push_str(&to_linear(sub));
                    }
                    ScriptKind::SubSup | ScriptKind::Pre => {
                        out.push('_');
                        out.push_str(&to_linear(sub));
                        out.push('^');
                        out.push_str(&to_linear(sup));
                    }
                }
            }
            MNode::Rad { deg, deg_hide, e } => {
                out.push_str("√(");
                if !*deg_hide && !deg.is_empty() {
                    out.push_str(&to_linear(deg));
                    out.push('&');
                }
                out.push_str(&to_linear(e));
                out.push(')');
            }
            MNode::Delim { beg, end, elems } => {
                if let Some(b) = beg { out.push(*b); }
                for (idx, elem) in elems.iter().enumerate() {
                    if idx > 0 { out.push(','); }
                    out.push_str(&to_linear(elem));
                }
                if let Some(e) = end { out.push(*e); }
            }
            MNode::Nary { chr, sub, sup, e, .. } => {
                out.push(*chr);
                if !sub.is_empty() {
                    out.push('_');
                    out.push_str(&to_linear(sub));
                }
                if !sup.is_empty() {
                    out.push('^');
                    out.push_str(&to_linear(sup));
                }
                out.push(' ');
                out.push_str(&to_linear(e));
            }
            MNode::Matrix { rows, .. } => {
                out.push_str("\\matrix(");
                for (ri, r) in rows.iter().enumerate() {
                    if ri > 0 { out.push('@'); }
                    for (ci, c) in r.iter().enumerate() {
                        if ci > 0 { out.push('&'); }
                        out.push_str(&to_linear(c));
                    }
                }
                out.push(')');
            }
            MNode::Acc { chr, e } => {
                out.push_str(&to_linear(e));
                out.push(*chr);
            }
        }
    }
    out
}

//! Bidirectional LaTeX mathematical equation converter.

use crate::equation::ast::*;

pub fn parse_latex(s: &str) -> Arg {
    let mut p = Parser {
        src: s.chars().collect(),
        i: 0,
    };
    p.parse_arg()
}

pub fn to_latex(nodes: &[MNode]) -> String {
    let mut out = String::new();
    for node in nodes {
        match node {
            MNode::Run(r) => out.push_str(&r.text),
            MNode::Frac { num, den, .. } => {
                out.push_str("\\frac{");
                out.push_str(&to_latex(num));
                out.push_str("}{");
                out.push_str(&to_latex(den));
                out.push('}');
            }
            MNode::Script { kind, base, sub, sup } => {
                out.push_str(&to_latex(base));
                match kind {
                    ScriptKind::Sup => {
                        out.push_str("^{");
                        out.push_str(&to_latex(sup));
                        out.push('}');
                    }
                    ScriptKind::Sub => {
                        out.push_str("_{");
                        out.push_str(&to_latex(sub));
                        out.push('}');
                    }
                    ScriptKind::SubSup | ScriptKind::Pre => {
                        out.push_str("_{");
                        out.push_str(&to_latex(sub));
                        out.push_str("}^{");
                        out.push_str(&to_latex(sup));
                        out.push('}');
                    }
                }
            }
            MNode::Rad { deg, deg_hide, e } => {
                out.push_str("\\sqrt");
                if !*deg_hide && !deg.is_empty() {
                    out.push('[');
                    out.push_str(&to_latex(deg));
                    out.push(']');
                }
                out.push('{');
                out.push_str(&to_latex(e));
                out.push('}');
            }
            MNode::Nary { chr, sub, sup, e, .. } => {
                let cmd = match *chr {
                    '∑' => "\\sum",
                    '∫' => "\\int",
                    '∏' => "\\prod",
                    _ => "\\sum",
                };
                out.push_str(cmd);
                if !sub.is_empty() {
                    out.push_str("_{");
                    out.push_str(&to_latex(sub));
                    out.push('}');
                }
                if !sup.is_empty() {
                    out.push_str("^{");
                    out.push_str(&to_latex(sup));
                    out.push('}');
                }
                out.push(' ');
                out.push_str(&to_latex(e));
            }
            MNode::Delim { beg, end, elems } => {
                let b = beg.map_or(".".to_string(), |c| c.to_string());
                let e = end.map_or(".".to_string(), |c| c.to_string());
                out.push_str("\\left");
                out.push_str(&b);
                for (idx, elem) in elems.iter().enumerate() {
                    if idx > 0 {
                        out.push(',');
                    }
                    out.push_str(&to_latex(elem));
                }
                out.push_str("\\right");
                out.push_str(&e);
            }
            MNode::Matrix { rows, .. } => {
                out.push_str("\\begin{matrix}");
                for (ri, row) in rows.iter().enumerate() {
                    if ri > 0 {
                        out.push_str(" \\\\ ");
                    }
                    for (ci, cell) in row.iter().enumerate() {
                        if ci > 0 {
                            out.push_str(" & ");
                        }
                        out.push_str(&to_latex(cell));
                    }
                }
                out.push_str("\\end{matrix}");
            }
            MNode::Acc { chr, e } => {
                let cmd = match *chr {
                    '^' | '̂' => "\\hat",
                    '¯' | '̄' => "\\bar",
                    '→' | '⃗' => "\\vec",
                    '.' | '̇' => "\\dot",
                    _ => "\\tilde",
                };
                out.push_str(cmd);
                out.push('{');
                out.push_str(&to_latex(e));
                out.push('}');
            }
        }
    }
    out
}

struct Parser {
    src: Vec<char>,
    i: usize,
}

impl Parser {
    fn peek(&self) -> Option<char> {
        self.src.get(self.i).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.i += 1;
        Some(c)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.i += 1;
            } else {
                break;
            }
        }
    }

    fn parse_arg(&mut self) -> Arg {
        let mut nodes = Vec::new();
        while self.i < self.src.len() {
            self.skip_whitespace();
            let Some(c) = self.peek() else { break };
            if c == '}' || c == '&' {
                break;
            }
            if c == '\\' {
                self.next();
                let cmd = self.parse_command();
                if cmd == "frac" {
                    let num = self.parse_group();
                    let den = self.parse_group();
                    nodes.push(MNode::Frac {
                        kind: FracKind::Bar,
                        num,
                        den,
                    });
                } else if cmd == "sqrt" {
                    let mut deg = Vec::new();
                    let mut deg_hide = true;
                    self.skip_whitespace();
                    if self.peek() == Some('[') {
                        self.next();
                        deg = self.parse_until(']');
                        deg_hide = false;
                    }
                    let e = self.parse_group();
                    nodes.push(MNode::Rad { deg, deg_hide, e });
                } else if cmd == "sum" {
                    nodes.push(self.parse_nary('∑'));
                } else if cmd == "int" {
                    nodes.push(self.parse_nary('∫'));
                } else if cmd == "prod" {
                    nodes.push(self.parse_nary('∏'));
                } else if cmd == "left" {
                    let beg = self.next();
                    let inner = self.parse_until_right();
                    let end = self.next();
                    nodes.push(MNode::Delim {
                        beg: beg.filter(|&c| c != '.'),
                        end: end.filter(|&c| c != '.'),
                        elems: vec![inner],
                    });
                } else if cmd == "alpha" {
                    nodes.push(MNode::Run(MRun::new("α")));
                } else if cmd == "beta" {
                    nodes.push(MNode::Run(MRun::new("β")));
                } else if cmd == "gamma" {
                    nodes.push(MNode::Run(MRun::new("γ")));
                } else if cmd == "pi" {
                    nodes.push(MNode::Run(MRun::new("π")));
                } else {
                    nodes.push(MNode::Run(MRun::new(format!("\\{cmd}"))));
                }
            } else if c == '{' {
                let grp = self.parse_group();
                nodes.extend(grp);
            } else if c == '^' || c == '_' {
                let is_sup = c == '^';
                self.next();
                let script_arg = self.parse_script_target();
                if let Some(prev) = nodes.pop() {
                    let base = vec![prev];
                    if is_sup {
                        nodes.push(MNode::Script {
                            kind: ScriptKind::Sup,
                            base,
                            sub: vec![],
                            sup: script_arg,
                        });
                    } else {
                        nodes.push(MNode::Script {
                            kind: ScriptKind::Sub,
                            base,
                            sub: script_arg,
                            sup: vec![],
                        });
                    }
                } else {
                    let base = vec![];
                    if is_sup {
                        nodes.push(MNode::Script {
                            kind: ScriptKind::Sup,
                            base,
                            sub: vec![],
                            sup: script_arg,
                        });
                    } else {
                        nodes.push(MNode::Script {
                            kind: ScriptKind::Sub,
                            base,
                            sub: script_arg,
                            sup: vec![],
                        });
                    }
                }
            } else {
                let mut text = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_whitespace() || ch == '\\' || ch == '{' || ch == '}' || ch == '^' || ch == '_' || ch == '&' {
                        break;
                    }
                    text.push(ch);
                    self.i += 1;
                }
                nodes.push(MNode::Run(MRun::new(text)));
            }
        }
        nodes
    }

    fn parse_command(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_alphabetic() {
                s.push(c);
                self.i += 1;
            } else {
                break;
            }
        }
        s
    }

    fn parse_group(&mut self) -> Arg {
        self.skip_whitespace();
        if self.peek() == Some('{') {
            self.next();
            let arg = self.parse_arg();
            if self.peek() == Some('}') {
                self.next();
            }
            arg
        } else if let Some(c) = self.next() {
            vec![MNode::Run(MRun::new(c.to_string()))]
        } else {
            vec![]
        }
    }

    fn parse_script_target(&mut self) -> Arg {
        self.parse_group()
    }

    fn parse_until(&mut self, term: char) -> Arg {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek() {
            if c == term {
                self.next();
                break;
            }
            nodes.extend(self.parse_arg());
        }
        nodes
    }

    fn parse_until_right(&mut self) -> Arg {
        let mut nodes = Vec::new();
        while self.i < self.src.len() {
            if self.src[self.i..].starts_with(&['\\', 'r', 'i', 'g', 'h', 't']) {
                self.i += 6;
                break;
            }
            nodes.extend(self.parse_arg());
        }
        nodes
    }

    fn parse_nary(&mut self, chr: char) -> MNode {
        let mut sub = Vec::new();
        let mut sup = Vec::new();
        self.skip_whitespace();
        while let Some(c) = self.peek() {
            if c == '_' {
                self.next();
                sub = self.parse_group();
            } else if c == '^' {
                self.next();
                sup = self.parse_group();
            } else {
                break;
            }
        }
        let e = self.parse_arg();
        MNode::Nary {
            chr,
            lim_loc: LimLoc::UndOvr,
            sub,
            sup,
            e,
        }
    }
}

//! Document statistics for academic TeX text analysis.

#![forbid(unsafe_code)]

use crate::texparse::lex::{lex, Tok};
use crate::texparse::structure::{group_text, Issue};

/// Document statistics for analysis.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DocStats {
    /// Whitespace-separated runs inside Text tokens (non-comment, non-math).
    pub words: usize,
    pub chars: usize,
    pub chars_no_space: usize,
    /// Physical source lines (`\n` + 1 for non-empty src).
    pub lines: usize,
    /// `\cmd` tokens.
    pub commands: usize,
    /// `$…$` / `\(…\)` inline math spans.
    pub math_inline: usize,
    /// `$$…$$` / `\[…\]` / equation-family display environments.
    pub math_display: usize,
    /// `\begin{…}` count.
    pub envs: usize,
    /// Heading commands (section/subsection/…/part/chapter/paragraph).
    pub sections: usize,
    /// `\label{…}` count.
    pub labels: usize,
    /// `\cite`-family occurrences (cite, citep, citet, parencite, …).
    pub cites: usize,
}

enum Math {
    Dollar,
    Paren,
    Bracket,
    Env(String),
}

/// Compute stats for `.tex` source `src` in single O(n) pass.
pub fn analyze(src: &str) -> DocStats {
    let mut d = DocStats {
        chars: src.chars().count(),
        chars_no_space: src.chars().filter(|c| !c.is_whitespace()).count(),
        lines: if src.is_empty() { 0 } else { src.matches('\n').count() + 1 },
        ..DocStats::default()
    };

    let toks = lex(src);
    let mut math: Option<Math> = None;
    let mut issues: Vec<Issue> = Vec::new();
    let mut i = 0usize;

    while let Some(tok) = toks.get(i) {
        i += 1;
        match &tok.kind {
            Tok::Text(s) => {
                if math.is_none() {
                    d.words += s.split_whitespace().count();
                }
            }
            Tok::MathShift => match math {
                None => {
                    if matches!(toks.get(i).map(|t| &t.kind), Some(Tok::MathShift)) {
                        i += 1;
                        d.math_display += 1;
                    } else {
                        d.math_inline += 1;
                    }
                    math = Some(Math::Dollar);
                }
                Some(Math::Dollar) => {
                    if matches!(toks.get(i).map(|t| &t.kind), Some(Tok::MathShift)) {
                        i += 1;
                    }
                    math = None;
                }
                _ => {}
            },
            Tok::CmdSym('(') => {
                if math.is_none() {
                    math = Some(Math::Paren);
                    d.math_inline += 1;
                }
            }
            Tok::CmdSym(')') => {
                if matches!(math, Some(Math::Paren)) {
                    math = None;
                }
            }
            Tok::CmdSym('[') => {
                if math.is_none() {
                    math = Some(Math::Bracket);
                    d.math_display += 1;
                }
            }
            Tok::CmdSym(']') => {
                if matches!(math, Some(Math::Bracket)) {
                    math = None;
                }
            }
            Tok::Cmd(name) => {
                d.commands += 1;
                match name.as_str() {
                    "begin" => {
                        d.envs += 1;
                        if let Some((env, ni)) = group_text(&toks, i, name, &mut issues) {
                            i = ni;
                            if math.is_none() && is_math_env(&env) {
                                math = Some(Math::Env(env));
                                d.math_display += 1;
                            }
                        }
                    }
                    "end" => {
                        if let Some((env, ni)) = group_text(&toks, i, name, &mut issues) {
                            i = ni;
                            if matches!(&math, Some(Math::Env(open)) if open == &env) {
                                math = None;
                            }
                        }
                    }
                    "label" => d.labels += 1,
                    c if is_section(c) => d.sections += 1,
                    c if c.contains("cite") => d.cites += 1,
                    _ => {}
                }
            }
            _ => {}
        }
    }
    d
}

fn is_section(name: &str) -> bool {
    matches!(name, "part" | "chapter" | "section" | "subsection" | "subsubsection" | "paragraph" | "subparagraph")
}

fn is_math_env(name: &str) -> bool {
    let base = name.strip_suffix('*').unwrap_or(name);
    matches!(base, "equation" | "align" | "gather" | "multline" | "eqnarray")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_words_chars_lines() {
        let s = analyze("Hello world.\nSecond line.");
        assert_eq!(s.words, 4);
        assert_eq!(s.chars, 25);
        assert_eq!(s.chars_no_space, 22);
        assert_eq!(s.lines, 2);
    }

    #[test]
    fn inline_math_skipped_but_counted() {
        let s = analyze("a $x + y$ b \\( z \\) c");
        assert_eq!(s.words, 3);
        assert_eq!(s.math_inline, 2);
        assert_eq!(s.math_display, 0);
    }
}

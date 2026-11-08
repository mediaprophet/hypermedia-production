//! TextMate-style snippet source, loader, and expansion engine.
//!
//! Grounded in authoring rights and scientific communication freedom (UDHR Art. 19, 27; ICESCR Art. 15).
//!
//! Three layers merge, later overriding by trigger:
//! 1. Built-in standard TeX starters
//! 2. Global snippets (`$ACADEMIC_TEX_HOME/snippets.json` or `~/.academic-tex/snippets.json`)
//! 3. Project-level `.academic-tex/snippets.json` (git-shareable)
//!
//! Bodies carry `$1`..`$9` tabstops, `${1:default}` placeholders, and `$0`
//! for final caret position. `expand` renders insertion text and the first
//! stop's byte offset.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

/// One snippet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snippet {
    pub trigger: String,
    pub body: String,
    pub description: String,
    /// `math` expands only inside math mode; `text` only outside; `any` everywhere.
    pub context: String,
    /// `builtin` | `global` | `project`.
    pub source: String,
}

/// Built-in starters.
pub const BUILTINS: &[(&str, &str, &str, &str)] = &[
    ("fig", "\\begin{figure}\n    \\centering\n    \\includegraphics[width=${1:0.8}\\textwidth]{${2:image}}\n    \\caption{${3:caption}}\n    \\label{fig:${4:label}}\n\\end{figure}\n$0", "figure + includegraphics", "any"),
    ("tab", "\\begin{table}\n    \\centering\n    \\begin{tabular}{${1:cc}}\n        $0\n    \\end{tabular}\n    \\caption{${2:caption}}\n    \\label{tab:${3:label}}\n\\end{table}", "table + tabular", "any"),
    ("eq", "\\begin{equation}\n    $0\n\\end{equation}", "numbered equation", "any"),
    ("al", "\\begin{align}\n    ${1:lhs} &= ${2:rhs}\n\\end{align}", "align environment", "math"),
    ("itm", "\\begin{itemize}\n    \\item $0\n\\end{itemize}", "itemize list", "any"),
    ("enu", "\\begin{enumerate}\n    \\item $0\n\\end{enumerate}", "enumerate list", "any"),
    ("fra", "\\begin{frame}{${1:title}}\n    $0\n\\end{frame}", "beamer frame", "any"),
    ("frac", "\\frac{${1:a}}{${2:b}}$0", "\\frac{}{}", "math"),
    ("sec", "\\section{${1:title}}\n$0", "section", "any"),
    ("sub", "\\subsection{${1:title}}\n$0", "subsection", "any"),
    ("bf", "\\textbf{${1:text}}$0", "bold", "any"),
    ("it", "\\textit{${1:text}}$0", "italic", "any"),
];

/// Max bytes read from a snippet file.
pub const MAX_SNIPPET_FILE: u64 = 256 * 1024;
/// Max snippets per source.
pub const MAX_SNIPPETS: usize = 512;

/// Global snippets path: `$ACADEMIC_TEX_HOME/snippets.json`, else `~/.academic-tex/snippets.json`.
pub fn global_snippets_path() -> Option<PathBuf> {
    if let Ok(h) = std::env::var("ACADEMIC_TEX_HOME") {
        if !h.is_empty() {
            return Some(Path::new(&h).join("snippets.json"));
        }
    }
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .filter(|h| !h.is_empty())
        .map(|h| Path::new(&h).join(".academic-tex").join("snippets.json"))
}

/// Parse a snippet JSON string: accepts `[{trigger,body,...}]` or `{trigger: body}` / `{trigger: {body,...}}`.
pub fn parse_snippets(text: &str, source: &str) -> Vec<Snippet> {
    let Ok(v) = serde_json::from_str::<Value>(text) else { return Vec::new() };
    let mut out = Vec::new();
    let mut push = |trigger: &str, body: &str, desc: &str, context: &str| {
        if out.len() < MAX_SNIPPETS && !trigger.is_empty() && !body.is_empty() {
            out.push(Snippet {
                trigger: trigger.to_string(),
                body: body.to_string(),
                description: desc.to_string(),
                context: if matches!(context, "math" | "text") { context } else { "any" }.to_string(),
                source: source.to_string(),
            });
        }
    };
    match &v {
        Value::Array(items) => {
            for it in items {
                let t = it.get("trigger").and_then(Value::as_str).unwrap_or("");
                let b = it.get("body").and_then(Value::as_str).unwrap_or("");
                let d = it.get("description").and_then(Value::as_str).unwrap_or("");
                let c = it.get("context").and_then(Value::as_str).unwrap_or("any");
                push(t, b, d, c);
            }
        }
        Value::Object(map) => {
            for (k, val) in map.iter().take(MAX_SNIPPETS) {
                match val {
                    Value::String(b) => push(k, b, "", "any"),
                    Value::Object(_) => {
                        let b = val.get("body").and_then(Value::as_str).unwrap_or("");
                        let d = val.get("description").and_then(Value::as_str).unwrap_or("");
                        let c = val.get("context").and_then(Value::as_str).unwrap_or("any");
                        push(k, b, d, c);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
    out
}

/// Read a snippet file bounded; missing or invalid returns empty vec.
pub fn read_snippets_file(path: &Path, source: &str) -> Vec<Snippet> {
    use std::io::Read;
    let mut buf = String::new();
    let Ok(f) = std::fs::File::open(path) else { return Vec::new() };
    if f.take(MAX_SNIPPET_FILE).read_to_string(&mut buf).is_err() {
        return Vec::new();
    }
    parse_snippets(&buf, source)
}

/// Load and merge all snippets given an optional project directory root.
pub fn load_snippets(project_dir: Option<&Path>) -> Vec<Snippet> {
    let mut merged: Vec<Snippet> = BUILTINS
        .iter()
        .map(|(t, b, d, c)| Snippet {
            trigger: (*t).to_string(),
            body: (*b).to_string(),
            description: (*d).to_string(),
            context: (*c).to_string(),
            source: "builtin".to_string(),
        })
        .collect();

    let global_p = global_snippets_path();
    let project_p = project_dir.map(|d| d.join(".academic-tex").join("snippets.json"));

    for (path, src) in [(global_p, "global"), (project_p, "project")] {
        if let Some(p) = path {
            for snip in read_snippets_file(&p, src) {
                if let Some(existing) = merged.iter_mut().find(|s| s.trigger == snip.trigger) {
                    *existing = snip;
                } else {
                    merged.push(snip);
                }
            }
        }
    }
    merged
}

/// Render `body`: `${n:default}` -> `default`, `$n` -> removed; caret lands
/// on `$1`, else `$0`, else end. Returns (rendered_text, caret_byte_offset).
pub fn expand(body: &str) -> (String, usize) {
    let mut out = String::with_capacity(body.len());
    let mut caret: Option<usize> = None;
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'$' {
            if bytes.get(i + 1) == Some(&b'{') {
                if let Some(close) = body.get(i + 2..).and_then(|r| r.find('}')) {
                    let inner = body.get(i + 2..i + 2 + close).unwrap_or("");
                    let (num, def) = match inner.split_once(':') {
                        Some((n, d)) => (n.chars().next().unwrap_or('9'), d),
                        None => (inner.chars().next().unwrap_or('9'), ""),
                    };
                    if num.is_ascii_digit() {
                        if num == '1' || (num == '0' && caret.is_none()) {
                            caret = Some(out.len());
                        }
                        out.push_str(def);
                        i += 2 + close + 1;
                        continue;
                    }
                }
            }
            if let Some(d) = bytes.get(i + 1) {
                if d.is_ascii_digit() {
                    if *d == b'1' || (*d == b'0' && caret.is_none()) {
                        caret = Some(out.len());
                    }
                    i += 2;
                    continue;
                }
            }
        }
        let ch_len = body.get(i..).and_then(|r| r.chars().next()).map(char::len_utf8).unwrap_or(1);
        out.push_str(body.get(i..i + ch_len).unwrap_or(""));
        i += ch_len;
    }
    let caret = caret.unwrap_or(out.len());
    (out, caret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_defaults_and_caret() {
        let (body, caret) = expand("\\section{${1:title}}\n$0");
        assert_eq!(body, "\\section{title}\n");
        assert_eq!(caret, "\\section{".len());
    }

    #[test]
    fn dollar_one_wins_over_zero() {
        let (body, caret) = expand("$0$1x");
        assert_eq!(body, "x");
        assert_eq!(caret, 0);
    }

    #[test]
    fn bare_tabstops_drop() {
        let (body, caret) = expand("a$2b$0");
        assert_eq!(body, "ab");
        assert_eq!(caret, 2);
    }

    #[test]
    fn malformed_files_are_empty_not_fatal() {
        assert!(parse_snippets("not json", "project").is_empty());
        assert_eq!(parse_snippets(r#"{"x":"$0"}"#, "project").len(), 1);
        assert!(parse_snippets("[]", "project").is_empty());
    }

    #[test]
    fn builtins_are_available() {
        let all = load_snippets(None);
        assert!(all.len() >= 12);
        let fig = all.iter().find(|s| s.trigger == "fig").unwrap();
        assert!(fig.body.contains("\\begin{figure}"));
    }
}

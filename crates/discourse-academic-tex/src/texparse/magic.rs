//! Editor "magic comment" header directives.
//!
//! TeXstudio/TeXShop-style `key = value` settings embedded in file headers:
//! `% !TeX program = xelatex`, `% !TeX root = main.tex`, `% !BIB program = biber`.

#![forbid(unsafe_code)]

const MAX_LINES: usize = 100;
const MAX_BYTES: usize = 8 * 1024;

/// Engine/root/encoding hints editors read from magic comments.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Magic {
    /// `% !TeX program` / `% !TEX ts-program` — the build tool.
    pub program: Option<String>,
    /// `% !TeX root` — master file of a multi-file project.
    pub root: Option<String>,
    /// `% !TeX encoding` — e.g. `UTF-8`.
    pub encoding: Option<String>,
    /// `% !BIB program` / `% !TeX bibliography` — e.g. `biber`, `bibtex`.
    pub bibliography: Option<String>,
    /// `% !TeX spellcheck` — e.g. `en_US`.
    pub spellcheck: Option<String>,
    /// `% !TeX jobname` — output name override.
    pub jobname: Option<String>,
    /// Extra `(key, value)` pairs in order of appearance.
    pub extras: Vec<(String, String)>,
}

impl Magic {
    /// Normalised build engine hinted by `program`/`ts-program`, falling back to `% !BIB program`.
    pub fn engine_hint(&self) -> Option<&str> {
        let raw = self.program.as_deref().or(self.bibliography.as_deref())?.trim();
        if raw.is_empty() {
            return None;
        }
        let mut name = raw.split_whitespace().next().unwrap_or(raw).to_ascii_lowercase();
        if name.ends_with(".exe") {
            name.truncate(name.len() - ".exe".len());
        }
        let engine = match name.as_str() {
            "pdflatex" | "pdftex" => "pdflatex",
            "xelatex" | "xetex" => "xelatex",
            "lualatex" | "luatex" => "lualatex",
            "latex" => "latex",
            "latexmk" => "latexmk",
            "bibtex" | "bibtex8" | "bibtexu" => "bibtex",
            "biber" => "biber",
            _ => return Some(raw),
        };
        Some(engine)
    }
}

/// Read magic-comment header of a `.tex` source.
pub fn parse(src: &str) -> Magic {
    let mut magic = Magic::default();
    let mut bytes = 0usize;
    for (i, raw) in src.strip_prefix('\u{feff}').unwrap_or(src).lines().enumerate() {
        if i >= MAX_LINES || bytes >= MAX_BYTES {
            break;
        }
        let end = floor_boundary(raw, MAX_BYTES - bytes);
        let line = raw.get(..end).unwrap_or(raw);
        bytes += line.len() + 1;
        let body = line.trim_start();
        if body.is_empty() {
            continue;
        }
        if !body.starts_with('%') {
            break;
        }
        if let Some((key, value, is_bib)) = directive(body) {
            assign(&mut magic, &key, &value, is_bib);
        }
    }
    magic
}

fn directive(line: &str) -> Option<(String, String, bool)> {
    let rest = line.strip_prefix('%')?.trim_start();
    let rest = rest.strip_prefix('!')?.trim_start();
    let name_len = rest.find(|c: char| !c.is_ascii_alphabetic()).unwrap_or(rest.len());
    let (name, kv) = rest.split_at(name_len);
    let is_bib = match name.to_ascii_lowercase().as_str() {
        "tex" => false,
        "bib" => true,
        _ => return None,
    };
    let (key, value) = kv.trim_start().split_once('=')?;
    let key = key.trim().to_ascii_lowercase();
    if key.is_empty() {
        return None;
    }
    Some((key, value.trim().to_string(), is_bib))
}

fn assign(m: &mut Magic, key: &str, value: &str, is_bib: bool) {
    let slot = match key {
        "program" => {
            if is_bib {
                &mut m.bibliography
            } else {
                &mut m.program
            }
        }
        "ts-program" => &mut m.program,
        "bibliography" | "bibtex" => &mut m.bibliography,
        "root" => &mut m.root,
        "encoding" => &mut m.encoding,
        "spellcheck" | "spellcheck-language" => &mut m.spellcheck,
        "jobname" => &mut m.jobname,
        _ => {
            m.extras.push((key.to_string(), value.to_string()));
            return;
        }
    };
    *slot = if value.is_empty() { None } else { Some(value.to_string()) };
}

fn floor_boundary(s: &str, limit: usize) -> usize {
    let mut n = limit.min(s.len());
    while n > 0 && !s.is_char_boundary(n) {
        n -= 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tex_directives() {
        let m = parse("% !TeX program = xelatex\n% !TeX root = main.tex\n% !TeX encoding = UTF-8\n% !TeX jobname = draft\n% !TeX spellcheck = en_US\n\\documentclass{article}\n");
        assert_eq!(m.program.as_deref(), Some("xelatex"));
        assert_eq!(m.root.as_deref(), Some("main.tex"));
        assert_eq!(m.encoding.as_deref(), Some("UTF-8"));
        assert_eq!(m.jobname.as_deref(), Some("draft"));
        assert_eq!(m.spellcheck.as_deref(), Some("en_US"));
        assert!(m.extras.is_empty());
        assert_eq!(m.engine_hint(), Some("xelatex"));
    }

    #[test]
    fn bib_directives() {
        let m = parse("% !BIB program = biber\n");
        assert_eq!(m.bibliography.as_deref(), Some("biber"));
        assert_eq!(m.engine_hint(), Some("biber"));
    }
}

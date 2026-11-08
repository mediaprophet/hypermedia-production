//! `.bib` (BibTeX) file parser.

#![forbid(unsafe_code)]

use crate::texparse::caps;
use crate::texparse::lex::Pos;
use crate::texparse::structure::Issue;

/// One bibliography entry: `@kind{key, field = value, …}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibEntry {
    /// Lowercased entry type (`article`, `book`, `misc`, …).
    pub kind: String,
    pub key: String,
    /// `(field, value)` pairs; field names are lowercased.
    pub fields: Vec<(String, String)>,
    pub pos: Pos,
}

impl BibEntry {
    /// Case-insensitive field lookup.
    pub fn field(&self, name: &str) -> Option<&str> {
        let lname = name.to_ascii_lowercase();
        self.fields.iter().find(|(n, _)| *n == lname).map(|(_, v)| v.as_str())
    }
}

/// A parsed `.bib` file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BibFile {
    /// `@string` abbreviations, name -> value (names lowercased).
    pub strings: Vec<(String, String)>,
    /// `@preamble` contents.
    pub preambles: Vec<String>,
    /// Real entries (`@string`/`@preamble`/`@comment` are not entries).
    pub entries: Vec<BibEntry>,
    pub issues: Vec<Issue>,
}

impl BibFile {
    /// Entry lookup by key (exact match).
    pub fn get(&self, key: &str) -> Option<&BibEntry> {
        self.entries.iter().find(|e| e.key == key)
    }
}

/// Parse a `.bib` source string. Always returns a file object; problems land in `issues`.
pub fn parse_bib(src: &str) -> BibFile {
    let mut p = Parser { chars: src.char_indices().collect(), i: 0, file: BibFile::default(), depth: 0, lc: (0, 1, 1) };
    p.run();
    p.file
}

struct Parser {
    chars: Vec<(usize, char)>,
    i: usize,
    file: BibFile,
    depth: usize,
    lc: (usize, usize, usize),
}

impl Parser {
    fn run(&mut self) {
        while self.i < self.chars.len() {
            if self.cur().1 != '@' {
                self.i += 1;
                continue;
            }
            let at_pos = self.pos();
            self.i += 1;
            let kind = self.ident().to_ascii_lowercase();
            self.skip_ws();
            match kind.as_str() {
                "comment" => self.comment_entry(at_pos),
                "string" => self.preamble_body(at_pos, true),
                "preamble" => self.preamble_body(at_pos, false),
                other => self.bib_entry(at_pos, other),
            }
            if self.depth > caps::MAX_DEPTH {
                self.issue_at(at_pos, "nesting too deep — aborting parse".into());
                return;
            }
        }
    }

    fn cur(&self) -> (usize, char) {
        self.chars.get(self.i).copied().unwrap_or((usize::MAX, '\0'))
    }

    fn pos(&mut self) -> Pos {
        while self.lc.0 < self.i {
            let (_, c) = self.chars.get(self.lc.0).copied().unwrap_or((usize::MAX, '\0'));
            if c == '\n' {
                self.lc.1 += 1;
                self.lc.2 = 1;
            } else {
                self.lc.2 += 1;
            }
            self.lc.0 += 1;
        }
        let off = self.chars.get(self.i).map(|&(o, _)| o).unwrap_or(usize::MAX);
        Pos { offset: off, line: self.lc.1, col: self.lc.2 }
    }

    fn issue(&mut self, msg: String) {
        let pos = self.pos();
        self.file.issues.push(Issue { message: msg, pos });
    }

    fn issue_at(&mut self, pos: Pos, msg: String) {
        self.file.issues.push(Issue { message: msg, pos });
    }

    fn skip_ws(&mut self) {
        while self.i < self.chars.len() && self.cur().1.is_whitespace() {
            self.i += 1;
        }
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            self.skip_ws();
            if self.cur().1 == '%' {
                while self.i < self.chars.len() && self.cur().1 != '\n' {
                    self.i += 1;
                }
            } else {
                return;
            }
        }
    }

    fn ident(&mut self) -> String {
        let mut s = String::new();
        while self.i < self.chars.len() {
            let c = self.cur().1;
            if c.is_whitespace() || matches!(c, '{' | '}' | '(' | ')' | '=' | ',' | '"' | '#' | '@' | '%') {
                break;
            }
            if s.len() < caps::MAX_NAME {
                s.push(c);
            }
            self.i += 1;
        }
        s
    }

    fn open_delim(&mut self) -> Option<char> {
        self.skip_ws();
        match self.cur().1 {
            '{' => {
                self.i += 1;
                Some('}')
            }
            '(' => {
                self.i += 1;
                Some(')')
            }
            _ => None,
        }
    }

    fn comment_entry(&mut self, at: Pos) {
        self.skip_ws();
        match self.cur().1 {
            '{' | '(' => {
                let close = if self.cur().1 == '{' { '}' } else { ')' };
                self.i += 1;
                self.skip_balanced(close, at);
            }
            _ => {
                while self.i < self.chars.len() && self.cur().1 != '\n' {
                    self.i += 1;
                }
            }
        }
    }

    fn preamble_body(&mut self, at: Pos, is_string: bool) {
        let Some(close) = self.open_delim() else {
            self.issue_at(at, "expected { after @string/@preamble".into());
            return;
        };
        if is_string {
            self.skip_ws_and_comments();
            let name = self.ident().to_ascii_lowercase();
            self.skip_ws();
            if self.cur().1 == '=' {
                self.i += 1;
            } else {
                self.issue(format!("@string '{name}': expected ="));
                self.skip_balanced(close, at);
                return;
            }
            let val = self.value();
            self.file.strings.push((name, val));
        } else {
            let val = self.value();
            if !val.is_empty() {
                self.file.preambles.push(val);
            }
        }
        self.skip_ws_and_comments();
        if self.cur().1 == close {
            self.i += 1;
        } else {
            self.issue(format!("expected {close} to close entry"));
        }
    }

    fn bib_entry(&mut self, at: Pos, kind: &str) {
        let kind = kind.to_string();
        let Some(close) = self.open_delim() else {
            self.issue_at(at, format!("expected {{ after @{kind}"));
            return;
        };
        self.skip_ws_and_comments();
        if self.cur().1 == close {
            self.i += 1;
            self.file.entries.push(BibEntry { kind, key: String::new(), fields: Vec::new(), pos: at });
            return;
        }
        let key = self.ident();
        self.skip_ws_and_comments();
        if self.cur().1 == ',' {
            self.i += 1;
        } else if self.cur().1 != close {
            self.issue(format!("entry '{key}': expected , after key"));
        }
        let mut fields = Vec::new();
        loop {
            self.skip_ws_and_comments();
            match self.cur().1 {
                _ if self.i >= self.chars.len() => {
                    self.issue(format!("entry '{key}': unexpected end of file"));
                    break;
                }
                c if c == close => {
                    self.i += 1;
                    break;
                }
                '}' if close == ')' => {
                    self.issue(format!("entry '{key}': mismatched }} inside (…) entry"));
                    break;
                }
                ',' | '@' => {
                    if self.cur().1 == '@' {
                        self.issue(format!("entry '{key}': unclosed — resynchronising at @"));
                        break;
                    }
                    self.i += 1;
                }
                _ => {
                    let name = self.ident().to_ascii_lowercase();
                    if name.is_empty() {
                        self.issue(format!("entry '{key}': skipped '{}'", self.cur().1));
                        self.i += 1;
                        continue;
                    }
                    self.skip_ws();
                    if self.cur().1 == '=' {
                        self.i += 1;
                        let val = self.value();
                        if fields.iter().any(|(n, _): &(String, String)| *n == name) {
                            self.issue(format!("entry '{key}': duplicate field '{name}'"));
                        }
                        fields.push((name, val));
                    } else {
                        self.issue(format!("entry '{key}': expected = after '{name}'"));
                    }
                    self.skip_ws_and_comments();
                    if self.cur().1 == ',' {
                        self.i += 1;
                    }
                }
            }
            if self.depth > caps::MAX_DEPTH {
                self.issue_at(at, "nesting too deep — aborting parse".into());
                return;
            }
        }
        self.file.entries.push(BibEntry { kind, key, fields, pos: at });
    }

    fn value(&mut self) -> String {
        let mut out = String::new();
        loop {
            self.skip_ws_and_comments();
            match self.cur().1 {
                '"' => {
                    self.i += 1;
                    self.quoted(&mut out);
                }
                '{' => {
                    self.i += 1;
                    self.braced(&mut out);
                }
                c if c.is_ascii_digit() => {
                    while self.i < self.chars.len() && self.cur().1.is_ascii_digit() {
                        out.push(self.cur().1);
                        self.i += 1;
                    }
                }
                c if c.is_whitespace() || matches!(c, ',' | '}' | ')' | '#') => break,
                _ => {
                    let id = self.ident().to_ascii_lowercase();
                    match self.file.strings.iter().rev().find(|(n, _)| *n == id).map(|(_, v)| v.clone()) {
                        Some(v) => out.push_str(&v),
                        None => {
                            if !id.is_empty() {
                                self.issue(format!("undefined @string '{id}'"));
                                out.push_str(&id);
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
            self.skip_ws_and_comments();
            if self.cur().1 == '#' {
                self.i += 1;
            } else {
                break;
            }
            if out.len() > caps::MAX_ARG {
                break;
            }
        }
        out.truncate(caps::MAX_ARG);
        out
    }

    fn quoted(&mut self, out: &mut String) {
        let mut depth = 0usize;
        while self.i < self.chars.len() {
            let c = self.cur().1;
            self.i += 1;
            match c {
                '{' => {
                    depth += 1;
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
                '}' if depth > 0 => {
                    depth -= 1;
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
                '"' if depth == 0 => return,
                _ => {
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
            }
            self.depth = self.depth.max(depth);
        }
        self.issue("unterminated \"…\" value".into());
    }

    fn braced(&mut self, out: &mut String) {
        let mut depth = 1usize;
        while self.i < self.chars.len() {
            let c = self.cur().1;
            self.i += 1;
            match c {
                '{' => {
                    depth += 1;
                    if depth > caps::MAX_DEPTH {
                        self.depth = depth;
                        self.issue("brace nesting too deep".into());
                        return;
                    }
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return;
                    }
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
                _ => {
                    if out.len() < caps::MAX_ARG {
                        out.push(c);
                    }
                }
            }
            self.depth = self.depth.max(depth);
        }
        self.issue("unterminated {…} value".into());
    }

    fn skip_balanced(&mut self, close: char, at: Pos) {
        let mut depth = 1usize;
        while self.i < self.chars.len() {
            let c = self.cur().1;
            self.i += 1;
            if c == '{' || c == '(' {
                depth += 1;
            } else if c == '}' || c == ')' {
                depth = depth.saturating_sub(1);
                if depth == 0 || (c == close && depth == 0) {
                    return;
                }
            }
            if depth > caps::MAX_DEPTH {
                self.issue_at(at, "nesting too deep".into());
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_entry() {
        let f = parse_bib(
            r#"
@article{knuth84,
  author = {Knuth, Donald E.},
  title  = {The {TeX}book},
  year   = 1984,
}
"#,
        );
        assert!(f.issues.is_empty(), "issues: {:?}", f.issues);
        let e = f.get("knuth84").unwrap();
        assert_eq!(e.kind, "article");
        assert_eq!(e.field("author"), Some("Knuth, Donald E."));
        assert_eq!(e.field("title"), Some("The {TeX}book"));
        assert_eq!(e.field("YEAR"), Some("1984"));
    }

    #[test]
    fn strings_and_concat() {
        let f = parse_bib(
            r#"
@string{j = {Journal of }}
@preamble{"\newcommand{\R}{\mathbb{R}}"}
@misc{x, title = j # " Tests" # 2020}
"#,
        );
        assert!(f.issues.is_empty(), "issues: {:?}", f.issues);
        assert_eq!(f.strings, vec![("j".to_string(), "Journal of ".to_string())]);
        assert_eq!(f.preambles.len(), 1);
        assert_eq!(f.get("x").unwrap().field("title"), Some("Journal of  Tests2020"));
    }

    #[test]
    fn quoted_values_and_comments() {
        let f = parse_bib("junk before\n@comment{ignored {nested}}\n@book{b, title = \"A {B} title\"}\n% line comment\n");
        assert_eq!(f.entries.len(), 1);
        assert_eq!(f.get("b").unwrap().field("title"), Some("A {B} title"));
        assert!(f.issues.is_empty(), "issues: {:?}", f.issues);
    }

    #[test]
    fn parens_delimiters() {
        let f = parse_bib("@misc(k, title = {t})");
        assert!(f.issues.is_empty(), "issues: {:?}", f.issues);
        assert_eq!(f.get("k").unwrap().field("title"), Some("t"));
    }

    #[test]
    fn malformed_recovers() {
        let f = parse_bib("@article{a, title = {ok} @book{b, title = {t2}}");
        assert!(!f.issues.is_empty());
        assert_eq!(f.get("a").unwrap().field("title"), Some("ok"));
        assert_eq!(f.get("b").unwrap().field("title"), Some("t2"));
    }

    #[test]
    fn hostile_never_panics() {
        for src in ["", "@", "@{", "@misc{", "@misc{,", "@misc{k,", "@misc{k,t=", "@misc{k,t={", "@misc{k,t=\"", "{{{", "@string"] {
            let _ = parse_bib(src);
        }
    }
}

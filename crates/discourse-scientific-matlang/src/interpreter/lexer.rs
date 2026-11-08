//! Tokenizer for M-code grammar.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct LangError {
    pub message: String,
    pub line: usize,
}

impl fmt::Display for LangError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for LangError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Num(f64),
    Ident(String),
    Str(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    DotStar,
    DotSlash,
    DotCaret,
    Apostrophe,
    Tilde,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Eq,
    EqEq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    Amp,
    Pipe,
    Newline,
    Eof,
}

pub fn lex(source: &str) -> Result<Vec<Tok>, LangError> {
    let mut toks = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut line = 1;

    while i < len {
        let c = chars[i];

        if c == '\n' {
            toks.push(Tok::Newline);
            line += 1;
            i += 1;
            continue;
        }

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Comments: '%' or '//'
        if c == '%' || (c == '/' && i + 1 < len && chars[i + 1] == '/') {
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Numbers
        if c.is_ascii_digit() || (c == '.' && i + 1 < len && chars[i + 1].is_ascii_digit()) {
            let start = i;
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            if i < len && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < len && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
                while i < len && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let s: String = chars[start..i].iter().collect();
            let val: f64 = s.parse().map_err(|_| LangError {
                message: format!("invalid numeric literal `{s}`"),
                line,
            })?;
            toks.push(Tok::Num(val));
            continue;
        }

        // Identifiers
        if c.is_ascii_alphabetic() || c == '_' {
            let start = i;
            while i < len && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let id: String = chars[start..i].iter().collect();
            toks.push(Tok::Ident(id));
            continue;
        }

        // Strings
        if c == '\'' || c == '"' {
            let quote = c;
            i += 1;
            let start = i;
            while i < len && chars[i] != quote {
                if chars[i] == '\n' {
                    line += 1;
                }
                i += 1;
            }
            let s: String = chars[start..i].iter().collect();
            if i < len && chars[i] == quote {
                i += 1;
            }
            toks.push(Tok::Str(s));
            continue;
        }

        // Multi-char operators
        if c == '.' && i + 1 < len {
            if chars[i + 1] == '*' {
                toks.push(Tok::DotStar);
                i += 2;
                continue;
            }
            if chars[i + 1] == '/' {
                toks.push(Tok::DotSlash);
                i += 2;
                continue;
            }
            if chars[i + 1] == '^' {
                toks.push(Tok::DotCaret);
                i += 2;
                continue;
            }
        }

        if c == '=' && i + 1 < len && chars[i + 1] == '=' {
            toks.push(Tok::EqEq);
            i += 2;
            continue;
        }
        if c == '~' && i + 1 < len && chars[i + 1] == '=' {
            toks.push(Tok::Neq);
            i += 2;
            continue;
        }
        if c == '<' && i + 1 < len && chars[i + 1] == '=' {
            toks.push(Tok::Le);
            i += 2;
            continue;
        }
        if c == '>' && i + 1 < len && chars[i + 1] == '=' {
            toks.push(Tok::Ge);
            i += 2;
            continue;
        }

        // Single-char tokens
        match c {
            '+' => toks.push(Tok::Plus),
            '-' => toks.push(Tok::Minus),
            '*' => toks.push(Tok::Star),
            '/' => toks.push(Tok::Slash),
            '^' => toks.push(Tok::Caret),
            '\'' => toks.push(Tok::Apostrophe),
            '~' => toks.push(Tok::Tilde),
            '(' => toks.push(Tok::LParen),
            ')' => toks.push(Tok::RParen),
            '[' => toks.push(Tok::LBracket),
            ']' => toks.push(Tok::RBracket),
            ',' => toks.push(Tok::Comma),
            ';' => toks.push(Tok::Semicolon),
            ':' => toks.push(Tok::Colon),
            '=' => toks.push(Tok::Eq),
            '<' => toks.push(Tok::Lt),
            '>' => toks.push(Tok::Gt),
            '&' => toks.push(Tok::Amp),
            '|' => toks.push(Tok::Pipe),
            _ => {
                return Err(LangError {
                    message: format!("unexpected character `{c}`"),
                    line,
                });
            }
        }
        i += 1;
    }

    toks.push(Tok::Eof);
    Ok(toks)
}

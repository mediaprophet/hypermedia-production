//! Tokenizer for spreadsheet formulas.

use std::fmt;
use crate::core::value::CellError;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Number(f64),
    Text(String),
    Bool(bool),
    Error(CellError),
    Word(String),
    Sheet(String, Option<String>),
    Func(String),
    Struct(String),
    Op(&'static str),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Colon,
    Space,
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub tok: Tok,
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LexError {
    pub msg: String,
    pub pos: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.msg, self.pos)
    }
}

impl std::error::Error for LexError {}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '.' | '$' | '\\' | '?')
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
    let chars: Vec<(usize, char)> = src.char_indices().collect();
    let n = chars.len();
    let pos_of = |i: usize| chars.get(i).map(|c| c.0).unwrap_or(src.len());
    let ch = |i: usize| chars.get(i).map(|c| c.1);
    let mut out = Vec::new();
    let mut i = 0;

    while i < n {
        let start = pos_of(i);
        let Some(c) = ch(i) else { break };
        let push = |out: &mut Vec<Token>, tok: Tok, end_i: usize| {
            out.push(Token { tok, start, end: pos_of(end_i) });
        };

        match c {
            ' ' | '\t' | '\n' | '\r' => {
                let mut j = i;
                while matches!(ch(j), Some(' ' | '\t' | '\n' | '\r')) {
                    j += 1;
                }
                push(&mut out, Tok::Space, j);
                i = j;
            }
            '"' => {
                let mut s = String::new();
                let mut j = i + 1;
                loop {
                    match ch(j) {
                        None => return Err(LexError { msg: "unterminated string".into(), pos: start }),
                        Some('"') if ch(j + 1) == Some('"') => {
                            s.push('"');
                            j += 2;
                        }
                        Some('"') => {
                            j += 1;
                            break;
                        }
                        Some(x) => {
                            s.push(x);
                            j += 1;
                        }
                    }
                }
                push(&mut out, Tok::Text(s), j);
                i = j;
            }
            '\'' => {
                let mut s = String::new();
                let mut j = i + 1;
                loop {
                    match ch(j) {
                        None => return Err(LexError { msg: "unterminated quoted sheet name".into(), pos: start }),
                        Some('\'') if ch(j + 1) == Some('\'') => {
                            s.push('\'');
                            j += 2;
                        }
                        Some('\'') => {
                            j += 1;
                            break;
                        }
                        Some(x) => {
                            s.push(x);
                            j += 1;
                        }
                    }
                }
                if ch(j) == Some('!') {
                    j += 1;
                    if let Some((a, b)) = s.split_once(':') {
                        push(&mut out, Tok::Sheet(a.to_string(), Some(b.to_string())), j);
                    } else {
                        push(&mut out, Tok::Sheet(s, None), j);
                    }
                    i = j;
                } else {
                    return Err(LexError { msg: "expected '!' after quoted sheet name".into(), pos: pos_of(j) });
                }
            }
            '[' => {
                let mut depth = 1usize;
                let mut j = i + 1;
                let mut s = String::new();
                while j < n && depth > 0 {
                    match ch(j) {
                        Some('[') => { depth += 1; s.push('['); }
                        Some(']') => {
                            depth -= 1;
                            if depth > 0 { s.push(']'); }
                        }
                        Some(x) => s.push(x),
                        None => break,
                    }
                    j += 1;
                }
                if depth > 0 {
                    return Err(LexError { msg: "unterminated table reference [...]".into(), pos: start });
                }
                push(&mut out, Tok::Struct(s), j);
                i = j;
            }
            '(' => { push(&mut out, Tok::LParen, i + 1); i += 1; }
            ')' => { push(&mut out, Tok::RParen, i + 1); i += 1; }
            '{' => { push(&mut out, Tok::LBrace, i + 1); i += 1; }
            '}' => { push(&mut out, Tok::RBrace, i + 1); i += 1; }
            ',' => { push(&mut out, Tok::Comma, i + 1); i += 1; }
            ';' => { push(&mut out, Tok::Semicolon, i + 1); i += 1; }
            ':' => { push(&mut out, Tok::Colon, i + 1); i += 1; }
            '+' => { push(&mut out, Tok::Op("+"), i + 1); i += 1; }
            '-' => { push(&mut out, Tok::Op("-"), i + 1); i += 1; }
            '*' => { push(&mut out, Tok::Op("*"), i + 1); i += 1; }
            '/' => { push(&mut out, Tok::Op("/"), i + 1); i += 1; }
            '^' => { push(&mut out, Tok::Op("^"), i + 1); i += 1; }
            '&' => { push(&mut out, Tok::Op("&"), i + 1); i += 1; }
            '%' => { push(&mut out, Tok::Op("%"), i + 1); i += 1; }
            '#' => {
                let mut j = i + 1;
                while matches!(ch(j), Some(c) if c.is_ascii_alphanumeric() || c == '/' || c == '?' || c == '!') {
                    let is_term = matches!(ch(j), Some('?' | '!'));
                    j += 1;
                    if is_term { break; }
                }
                let slice = &src[start..pos_of(j)];
                if let Some(err) = CellError::parse(slice) {
                    push(&mut out, Tok::Error(err), j);
                    i = j;
                } else {
                    push(&mut out, Tok::Op("#"), i + 1);
                    i += 1;
                }
            }
            '@' => { push(&mut out, Tok::Op("@"), i + 1); i += 1; }
            '=' => { push(&mut out, Tok::Op("="), i + 1); i += 1; }
            '<' => {
                if ch(i + 1) == Some('>') {
                    push(&mut out, Tok::Op("<>"), i + 2);
                    i += 2;
                } else if ch(i + 1) == Some('=') {
                    push(&mut out, Tok::Op("<="), i + 2);
                    i += 2;
                } else {
                    push(&mut out, Tok::Op("<"), i + 1);
                    i += 1;
                }
            }
            '>' => {
                if ch(i + 1) == Some('=') {
                    push(&mut out, Tok::Op(">="), i + 2);
                    i += 2;
                } else {
                    push(&mut out, Tok::Op(">"), i + 1);
                    i += 1;
                }
            }
            '0'..='9' | '.' => {
                let mut j = i;
                let mut has_dot = c == '.';
                let mut has_exp = false;
                while let Some(ch_j) = ch(j) {
                    if ch_j.is_ascii_digit() {
                        j += 1;
                    } else if ch_j == '.' && !has_dot && !has_exp {
                        has_dot = true;
                        j += 1;
                    } else if (ch_j == 'e' || ch_j == 'E') && !has_exp {
                        has_exp = true;
                        j += 1;
                        if matches!(ch(j), Some('+' | '-')) {
                            j += 1;
                        }
                    } else {
                        break;
                    }
                }
                let slice = &src[start..pos_of(j)];
                if slice == "." {
                    push(&mut out, Tok::Op("."), i + 1);
                    i += 1;
                } else if let Ok(n) = slice.parse::<f64>() {
                    push(&mut out, Tok::Number(n), j);
                    i = j;
                } else {
                    return Err(LexError { msg: "invalid number".into(), pos: start });
                }
            }
            _ if is_word_char(c) => {
                let mut j = i;
                while matches!(ch(j), Some(x) if is_word_char(x)) {
                    j += 1;
                }
                let word = src[start..pos_of(j)].to_string();
                if ch(j) == Some('!') {
                    let sheet_name = word;
                    j += 1;
                    push(&mut out, Tok::Sheet(sheet_name, None), j);
                    i = j;
                } else if ch(j) == Some('(') {
                    push(&mut out, Tok::Func(word), j + 1);
                    i = j + 1;
                } else if word.eq_ignore_ascii_case("TRUE") {
                    push(&mut out, Tok::Bool(true), j);
                    i = j;
                } else if word.eq_ignore_ascii_case("FALSE") {
                    push(&mut out, Tok::Bool(false), j);
                    i = j;
                } else {
                    push(&mut out, Tok::Word(word), j);
                    i = j;
                }
            }
            _ => {
                return Err(LexError { msg: format!("unexpected character '{c}'"), pos: start });
            }
        }
    }
    out.push(Token { tok: Tok::Eof, start: src.len(), end: src.len() });
    Ok(out)
}

//! Pratt parser for spreadsheet formulas.

use std::fmt;
use std::sync::Arc;

use crate::core::addr::{letters_to_col, parse_a1_prefix};
use crate::core::value::CellError;
use crate::formula::ast::*;
use crate::formula::lexer::{Tok, Token, tokenize};

#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    pub msg: String,
    pub pos: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} at {}", self.msg, self.pos)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(src: &str) -> Result<Expr, ParseError> {
    let body = src.strip_prefix('=').unwrap_or(src);
    let toks = tokenize(body).map_err(|e| ParseError { msg: e.msg, pos: e.pos })?;
    let toks = filter_spaces(toks);
    let mut p = Parser { toks, i: 0 };
    let e = p.expr(0)?;
    match p.peek() {
        Tok::Eof => Ok(e),
        _ => Err(p.err("unexpected token after formula")),
    }
}

fn filter_spaces(toks: Vec<Token>) -> Vec<Token> {
    toks.into_iter().filter(|t| t.tok != Tok::Space).collect()
}

struct Parser {
    toks: Vec<Token>,
    i: usize,
}

impl Parser {
    fn peek(&self) -> &Tok {
        self.toks.get(self.i).map(|t| &t.tok).unwrap_or(&Tok::Eof)
    }

    fn pos(&self) -> usize {
        self.toks.get(self.i).map(|t| t.start).unwrap_or(0)
    }

    fn next(&mut self) -> Tok {
        if self.i < self.toks.len() {
            let tok = self.toks[self.i].tok.clone();
            self.i += 1;
            tok
        } else {
            Tok::Eof
        }
    }

    fn err(&self, msg: impl Into<String>) -> ParseError {
        ParseError {
            msg: msg.into(),
            pos: self.pos(),
        }
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.primary()?;

        loop {
            // Postfix operators: `%`, `#`
            if self.peek() == &Tok::Op("%") {
                self.next();
                lhs = Expr::Unary(UnOp::Percent, Box::new(lhs));
                continue;
            }
            if self.peek() == &Tok::Op("#") {
                self.next();
                lhs = Expr::Unary(UnOp::Spill, Box::new(lhs));
                continue;
            }

            // Binary operators
            let (op, l_bp, r_bp) = match self.peek() {
                Tok::Colon => (BinOp::Range, 90, 91),
                Tok::Op("^") => (BinOp::Pow, 50, 49),
                Tok::Op("*") => (BinOp::Mul, 40, 41),
                Tok::Op("/") => (BinOp::Div, 40, 41),
                Tok::Op("+") => (BinOp::Add, 30, 31),
                Tok::Op("-") => (BinOp::Sub, 30, 31),
                Tok::Op("&") => (BinOp::Concat, 20, 21),
                Tok::Op("=") => (BinOp::Eq, 10, 11),
                Tok::Op("<>") => (BinOp::Ne, 10, 11),
                Tok::Op("<") => (BinOp::Lt, 10, 11),
                Tok::Op(">") => (BinOp::Gt, 10, 11),
                Tok::Op("<=") => (BinOp::Le, 10, 11),
                Tok::Op(">=") => (BinOp::Ge, 10, 11),
                _ => break,
            };

            if l_bp < min_bp {
                break;
            }

            self.next();
            let rhs = self.expr(r_bp)?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }

        Ok(lhs)
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        match self.next() {
            Tok::Number(n) => Ok(Expr::Number(n)),
            Tok::Text(s) => Ok(Expr::Text(Arc::from(s.as_str()))),
            Tok::Bool(b) => Ok(Expr::Bool(b)),
            Tok::Error(e) => Ok(Expr::Error(e)),
            Tok::Op("+") => {
                let e = self.expr(60)?;
                Ok(Expr::Unary(UnOp::Plus, Box::new(e)))
            }
            Tok::Op("-") => {
                let e = self.expr(60)?;
                Ok(Expr::Unary(UnOp::Neg, Box::new(e)))
            }
            Tok::Op("@") => {
                let e = self.expr(70)?;
                Ok(Expr::Unary(UnOp::At, Box::new(e)))
            }
            Tok::LParen => {
                let e = self.expr(0)?;
                if self.peek() != &Tok::RParen {
                    return Err(self.err("expected ')'"));
                }
                self.next();
                Ok(Expr::Paren(Box::new(e)))
            }
            Tok::LBrace => {
                // Array constant {1, 2; 3, 4}
                let mut rows = Vec::new();
                let mut curr_row = Vec::new();
                while self.peek() != &Tok::RBrace && self.peek() != &Tok::Eof {
                    let elem = self.expr(0)?;
                    curr_row.push(elem);
                    if self.peek() == &Tok::Comma {
                        self.next();
                    } else if self.peek() == &Tok::Semicolon {
                        self.next();
                        rows.push(std::mem::take(&mut curr_row));
                    }
                }
                if !curr_row.is_empty() {
                    rows.push(curr_row);
                }
                if self.peek() != &Tok::RBrace {
                    return Err(self.err("expected '}'"));
                }
                self.next();
                Ok(Expr::Array(rows))
            }
            Tok::Func(name) => {
                let upper = normalize_function_name(&name);
                let mut args = Vec::new();
                if self.peek() != &Tok::RParen {
                    loop {
                        if self.peek() == &Tok::Comma {
                            // Missing argument
                            args.push(Expr::Missing);
                            self.next();
                            continue;
                        }
                        if self.peek() == &Tok::RParen {
                            break;
                        }
                        let arg = self.expr(0)?;
                        args.push(arg);
                        if self.peek() == &Tok::Comma {
                            self.next();
                        } else {
                            break;
                        }
                    }
                }
                if self.peek() != &Tok::RParen {
                    return Err(self.err("expected ')' after function arguments"));
                }
                self.next();
                Ok(Expr::Call(upper, args))
            }
            Tok::Sheet(first, second) => {
                let sheet = match second {
                    Some(last) => SheetSel::Span(first, last),
                    None => SheetSel::Named(first),
                };
                let target = self.primary()?;
                match target {
                    Expr::Ref(mut r) => {
                        r.sheet = sheet;
                        Ok(Expr::Ref(r))
                    }
                    _ => Err(self.err("expected cell reference after sheet prefix")),
                }
            }
            Tok::Word(w) => {
                if let Some(cell) = parse_cell_reference(&w) {
                    Ok(Expr::Ref(Reference {
                        sheet: SheetSel::Current,
                        kind: RefKind::Cell(cell),
                    }))
                } else if let Some((c0, abs)) = parse_col_reference(&w) {
                    Ok(Expr::Ref(Reference {
                        sheet: SheetSel::Current,
                        kind: RefKind::Cols(c0, abs, c0, abs),
                    }))
                } else {
                    Ok(Expr::Name(w))
                }
            }
            Tok::Struct(body) => {
                Ok(Expr::Struct(StructRef {
                    table: String::new(),
                    specifiers: vec![],
                    col_start: Some(body),
                    col_end: None,
                }))
            }
            _ => Err(self.err("unexpected token")),
        }
    }
}

pub fn normalize_function_name(name: &str) -> String {
    let u = name.to_ascii_uppercase();
    for p in ["_XLFN._XLWS.", "_XLFN.", "_XLWS.", "_XLUDF."] {
        if let Some(r) = u.strip_prefix(p) {
            return r.to_string();
        }
    }
    u
}

fn parse_cell_reference(w: &str) -> Option<Anchor> {
    let (col, row, col_abs, row_abs, rest) = parse_a1_prefix(w)?;
    if !rest.is_empty() {
        return None;
    }
    Some(Anchor { row, col, row_abs, col_abs })
}

fn parse_col_reference(w: &str) -> Option<(u32, bool)> {
    let abs = w.starts_with('$');
    let letters = w.trim_start_matches('$');
    let col = letters_to_col(letters)?;
    Some((col, abs))
}

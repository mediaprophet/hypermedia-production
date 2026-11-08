//! Expression language for dimensional constraints and parametric variables.
//!
//! Evaluates numbers, variable identifiers, `+ - * /`, unary minus, parentheses and the constant `pi`.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::fmt;

pub const MAX_LEN: usize = 1024;
pub const MAX_DEPTH: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub enum ExprError {
    Syntax(String),
    Unknown(String),
    Cycle(String),
    DivZero,
    TooDeep,
    NotFinite,
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprError::Syntax(s) => write!(f, "syntax error: {s}"),
            ExprError::Unknown(s) => write!(f, "unknown parameter `{s}`"),
            ExprError::Cycle(s) => write!(f, "parameter `{s}` refers to itself"),
            ExprError::DivZero => write!(f, "division by zero"),
            ExprError::TooDeep => write!(f, "expression too long or too deeply nested"),
            ExprError::NotFinite => write!(f, "result is not a finite number"),
        }
    }
}

impl std::error::Error for ExprError {}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    Neg(Box<Expr>),
    Bin(char, Box<Expr>, Box<Expr>),
}

pub fn valid_name(s: &str) -> bool {
    let mut c = s.chars();
    matches!(c.next(), Some(ch) if ch.is_ascii_alphabetic() || ch == '_')
        && c.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        && s.len() <= 64
        && !s.eq_ignore_ascii_case("pi")
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
    depth: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.s.get(self.i).is_some_and(|c| c.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
    fn peek(&mut self) -> Option<u8> {
        self.ws();
        self.s.get(self.i).copied()
    }
    fn enter(&mut self) -> Result<(), ExprError> {
        self.depth += 1;
        if self.depth > MAX_DEPTH { Err(ExprError::TooDeep) } else { Ok(()) }
    }
    fn sum(&mut self) -> Result<Expr, ExprError> {
        self.enter()?;
        let mut e = self.product()?;
        while let Some(c @ (b'+' | b'-')) = self.peek() {
            self.i += 1;
            let r = self.product()?;
            e = Expr::Bin(c as char, Box::new(e), Box::new(r));
        }
        self.depth -= 1;
        Ok(e)
    }
    fn product(&mut self) -> Result<Expr, ExprError> {
        let mut e = self.unary()?;
        while let Some(c @ (b'*' | b'/')) = self.peek() {
            self.i += 1;
            let r = self.unary()?;
            e = Expr::Bin(c as char, Box::new(e), Box::new(r));
        }
        Ok(e)
    }
    fn unary(&mut self) -> Result<Expr, ExprError> {
        match self.peek() {
            Some(b'-') => {
                self.i += 1;
                self.enter()?;
                let e = self.unary()?;
                self.depth -= 1;
                Ok(Expr::Neg(Box::new(e)))
            }
            Some(b'+') => {
                self.i += 1;
                self.enter()?;
                let e = self.unary()?;
                self.depth -= 1;
                Ok(e)
            }
            _ => self.atom(),
        }
    }
    fn atom(&mut self) -> Result<Expr, ExprError> {
        self.ws();
        match self.peek() {
            Some(b'(') => {
                self.i += 1;
                self.enter()?;
                let e = self.sum()?;
                self.depth -= 1;
                if self.peek() != Some(b')') {
                    return Err(ExprError::Syntax("expected `)`".into()));
                }
                self.i += 1;
                Ok(e)
            }
            Some(c) if c.is_ascii_digit() || c == b'.' => self.number(),
            Some(c) if c.is_ascii_alphabetic() || c == b'_' => self.name(),
            Some(other) => Err(ExprError::Syntax(format!("unexpected `{}`", other as char))),
            None => Err(ExprError::Syntax("unexpected end of expression".into())),
        }
    }
    fn number(&mut self) -> Result<Expr, ExprError> {
        let start = self.i;
        while self.s.get(self.i).is_some_and(|c| c.is_ascii_digit() || *c == b'.') {
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).map_err(|_| ExprError::Syntax("invalid utf-8".into()))?;
        let v: f64 = text.parse().map_err(|_| ExprError::Syntax(format!("bad number `{text}`")))?;
        Ok(Expr::Num(v))
    }
    fn name(&mut self) -> Result<Expr, ExprError> {
        let start = self.i;
        while self.s.get(self.i).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_') {
            self.i += 1;
        }
        let text = std::str::from_utf8(&self.s[start..self.i]).map_err(|_| ExprError::Syntax("invalid utf-8".into()))?;
        if text.eq_ignore_ascii_case("pi") {
            return Ok(Expr::Num(std::f64::consts::PI));
        }
        Ok(Expr::Var(text.to_string()))
    }
}

pub fn parse(s: &str) -> Result<Expr, ExprError> {
    let t = s.trim();
    if t.is_empty() {
        return Err(ExprError::Syntax("empty expression".into()));
    }
    if t.len() > MAX_LEN {
        return Err(ExprError::TooDeep);
    }
    let mut p = Parser { s: t.as_bytes(), i: 0, depth: 0 };
    let e = p.sum()?;
    p.ws();
    if p.i != p.s.len() {
        return Err(ExprError::Syntax("trailing characters".into()));
    }
    Ok(e)
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Env {
    pub vars: BTreeMap<String, String>,
}

impl Env {
    pub fn new() -> Self {
        Env::default()
    }
    pub fn insert(&mut self, name: &str, expr: &str) {
        self.vars.insert(name.to_string(), expr.to_string());
    }
    pub fn eval(&self, expr_str: &str) -> Result<f64, ExprError> {
        let e = parse(expr_str)?;
        let mut visiting = Vec::new();
        self.eval_ast(&e, &mut visiting)
    }
    fn eval_ast(&self, e: &Expr, visiting: &mut Vec<String>) -> Result<f64, ExprError> {
        if visiting.len() > MAX_DEPTH {
            return Err(ExprError::TooDeep);
        }
        match e {
            Expr::Num(n) => {
                if n.is_finite() { Ok(*n) } else { Err(ExprError::NotFinite) }
            }
            Expr::Var(v) => {
                if visiting.iter().any(|name| name == v) {
                    return Err(ExprError::Cycle(v.clone()));
                }
                let Some(defn) = self.vars.get(v) else {
                    return Err(ExprError::Unknown(v.clone()));
                };
                let parsed = parse(defn)?;
                visiting.push(v.clone());
                let res = self.eval_ast(&parsed, visiting);
                visiting.pop();
                res
            }
            Expr::Neg(inner) => {
                let v = self.eval_ast(inner, visiting)?;
                let res = -v;
                if res.is_finite() { Ok(res) } else { Err(ExprError::NotFinite) }
            }
            Expr::Bin(op, l, r) => {
                let lv = self.eval_ast(l, visiting)?;
                let rv = self.eval_ast(r, visiting)?;
                let res = match op {
                    '+' => lv + rv,
                    '-' => lv - rv,
                    '*' => lv * rv,
                    '/' => {
                        if rv.abs() < 1e-15 {
                            return Err(ExprError::DivZero);
                        }
                        lv / rv
                    }
                    _ => return Err(ExprError::Syntax(format!("unsupported operator `{op}`"))),
                };
                if res.is_finite() { Ok(res) } else { Err(ExprError::NotFinite) }
            }
        }
    }
}

//! Formula syntax tree for spreadsheet expressions.

use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::core::addr::{CellRef, MAX_COLS, MAX_ROWS, RangeRef};
use crate::core::value::CellError;

/// One end of a reference, with `$` anchors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Anchor {
    pub row: u32,
    pub col: u32,
    pub row_abs: bool,
    pub col_abs: bool,
}

impl Anchor {
    pub fn cell(&self) -> CellRef {
        CellRef::new(self.row, self.col)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RefKind {
    Cell(Anchor),
    Range(Anchor, Anchor),
    /// Whole rows `3:5` (row, abs) pairs.
    Rows(u32, bool, u32, bool),
    /// Whole columns `A:C`.
    Cols(u32, bool, u32, bool),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SheetSel {
    /// Unqualified: the formula's own sheet.
    Current,
    Named(String),
    /// 3-D span `Sheet1:Sheet3`.
    Span(String, String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Reference {
    pub sheet: SheetSel,
    pub kind: RefKind,
}

impl Reference {
    pub fn range(&self) -> RangeRef {
        match &self.kind {
            RefKind::Cell(a) => RangeRef::cell(a.cell()),
            RefKind::Range(a, b) => RangeRef::new(a.cell(), b.cell()),
            RefKind::Rows(r0, _, r1, _) => RangeRef::rows((*r0).min(*r1), (*r0).max(*r1)),
            RefKind::Cols(c0, _, c1, _) => RangeRef::cols((*c0).min(*c1), (*c0).max(*c1)),
        }
    }

    pub fn sheet_name(&self) -> Option<&str> {
        match &self.sheet {
            SheetSel::Named(s) => Some(s),
            _ => None,
        }
    }
}

/// Structured reference `Table1[[#Headers],[Col1]:[Col3]]`, `[@Col]`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructRef {
    pub table: String,
    pub specifiers: Vec<StructItem>,
    pub col_start: Option<String>,
    pub col_end: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StructItem {
    All,
    Data,
    Headers,
    Totals,
    ThisRow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnOp {
    Neg,
    Plus,
    Percent,
    /// Implicit intersection `@`.
    At,
    /// Spill range operator `A1#`.
    Spill,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Concat,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    /// `:` between two references.
    Range,
    /// `,` inside parentheses.
    Union,
    /// Space between two references.
    Intersect,
}

impl BinOp {
    pub fn symbol(&self) -> &'static str {
        match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Pow => "^",
            BinOp::Concat => "&",
            BinOp::Eq => "=",
            BinOp::Ne => "<>",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::Range => ":",
            BinOp::Union => ",",
            BinOp::Intersect => " ",
        }
    }

    pub fn precedence(&self) -> u8 {
        match self {
            BinOp::Range => 90,
            BinOp::Intersect => 85,
            BinOp::Union => 80,
            BinOp::Pow => 50,
            BinOp::Mul | BinOp::Div => 40,
            BinOp::Add | BinOp::Sub => 30,
            BinOp::Concat => 20,
            BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => 10,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Number(f64),
    Text(Arc<str>),
    Bool(bool),
    Error(CellError),
    Array(Vec<Vec<Expr>>),
    Ref(Reference),
    Name(String),
    Struct(StructRef),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Invoke(Box<Expr>, Vec<Expr>),
    Missing,
    Paren(Box<Expr>),
}

impl Expr {
    pub fn walk<'a>(&'a self, f: &mut dyn FnMut(&'a Expr)) {
        f(self);
        match self {
            Expr::Array(rows) => rows.iter().flatten().for_each(|e| e.walk(f)),
            Expr::Unary(_, e) | Expr::Paren(e) => e.walk(f),
            Expr::Binary(_, a, b) => {
                a.walk(f);
                b.walk(f);
            }
            Expr::Call(_, args) => args.iter().for_each(|e| e.walk(f)),
            Expr::Invoke(c, args) => {
                c.walk(f);
                args.iter().for_each(|e| e.walk(f));
            }
            _ => {}
        }
    }

    pub fn map(self, f: &mut dyn FnMut(Expr) -> Expr) -> Expr {
        let e = match self {
            Expr::Array(rows) => {
                Expr::Array(rows.into_iter().map(|r| r.into_iter().map(|e| e.map(f)).collect()).collect())
            }
            Expr::Unary(op, e) => Expr::Unary(op, Box::new(e.map(f))),
            Expr::Paren(e) => Expr::Paren(Box::new(e.map(f))),
            Expr::Binary(op, a, b) => Expr::Binary(op, Box::new(a.map(f)), Box::new(b.map(f))),
            Expr::Call(name, args) => Expr::Call(name, args.into_iter().map(|e| e.map(f)).collect()),
            Expr::Invoke(c, args) => Expr::Invoke(Box::new(c.map(f)), args.into_iter().map(|e| e.map(f)).collect()),
            other => other,
        };
        f(e)
    }
}

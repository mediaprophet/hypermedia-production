//! Formatting `Expr` AST back into formula text.

use std::fmt::Write;
use crate::core::addr::{CellRef, col_to_letters};
use crate::core::value::number_to_text;
use crate::formula::ast::*;

pub fn print(e: &Expr) -> String {
    let mut out = String::new();
    write_expr(&mut out, e, 0);
    out
}

pub fn needs_quotes(name: &str) -> bool {
    if name.is_empty() {
        return true;
    }
    let first = name.chars().next().unwrap_or('_');
    if first.is_ascii_digit() || name.chars().any(|c| !(c.is_alphanumeric() || c == '_' || c == '.')) {
        return true;
    }
    CellRef::parse(name).is_some() || name.eq_ignore_ascii_case("TRUE") || name.eq_ignore_ascii_case("FALSE")
}

pub fn quote_sheet(name: &str) -> String {
    if needs_quotes(name) {
        format!("'{}'", name.replace('\'', "''"))
    } else {
        name.to_string()
    }
}

fn sheet_prefix(s: &SheetSel) -> String {
    match s {
        SheetSel::Current => String::new(),
        SheetSel::Named(n) => format!("{}!", quote_sheet(n)),
        SheetSel::Span(a, b) => {
            let joined = format!("{a}:{b}");
            if needs_quotes(a) || needs_quotes(b) {
                format!("'{}'!", joined.replace('\'', "''"))
            } else {
                format!("{joined}!")
            }
        }
    }
}

fn anchor_a1(a: &Anchor) -> String {
    format!(
        "{}{}{}{}",
        if a.col_abs { "$" } else { "" },
        col_to_letters(a.col),
        if a.row_abs { "$" } else { "" },
        a.row as u64 + 1
    )
}

pub fn reference_a1(r: &Reference) -> String {
    let body = match &r.kind {
        RefKind::Cell(a) => anchor_a1(a),
        RefKind::Range(a, b) => format!("{}:{}", anchor_a1(a), anchor_a1(b)),
        RefKind::Rows(r0, a0, r1, a1) => {
            format!("{}{}:{}{}", if *a0 { "$" } else { "" }, r0 + 1, if *a1 { "$" } else { "" }, r1 + 1)
        }
        RefKind::Cols(c0, a0, c1, a1) => {
            format!(
                "{}{}:{}{}",
                if *a0 { "$" } else { "" },
                col_to_letters(*c0),
                if *a1 { "$" } else { "" },
                col_to_letters(*c1)
            )
        }
    };
    format!("{}{}", sheet_prefix(&r.sheet), body)
}

fn write_expr(out: &mut String, e: &Expr, parent_bp: u8) {
    match e {
        Expr::Number(n) => out.push_str(&number_to_text(*n)),
        Expr::Text(s) => {
            out.push('"');
            out.push_str(&s.replace('"', "\"\""));
            out.push('"');
        }
        Expr::Bool(b) => out.push_str(if *b { "TRUE" } else { "FALSE" }),
        Expr::Error(err) => out.push_str(err.as_str()),
        Expr::Ref(r) => out.push_str(&reference_a1(r)),
        Expr::Name(n) => out.push_str(n),
        Expr::Struct(s) => {
            out.push_str(&s.table);
            out.push('[');
            if let Some(col) = &s.col_start {
                out.push_str(col);
            }
            out.push(']');
        }
        Expr::Unary(UnOp::Neg, inner) => {
            out.push('-');
            write_expr(out, inner, 60);
        }
        Expr::Unary(UnOp::Plus, inner) => {
            out.push('+');
            write_expr(out, inner, 60);
        }
        Expr::Unary(UnOp::Percent, inner) => {
            write_expr(out, inner, 70);
            out.push('%');
        }
        Expr::Unary(UnOp::At, inner) => {
            out.push('@');
            write_expr(out, inner, 70);
        }
        Expr::Unary(UnOp::Spill, inner) => {
            write_expr(out, inner, 70);
            out.push('#');
        }
        Expr::Binary(op, left, right) => {
            let bp = op.precedence();
            let need_paren = bp < parent_bp;
            if need_paren {
                out.push('(');
            }
            write_expr(out, left, bp);
            out.push_str(op.symbol());
            write_expr(out, right, bp + 1);
            if need_paren {
                out.push(')');
            }
        }
        Expr::Call(name, args) => {
            out.push_str(name);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_expr(out, arg, 0);
            }
            out.push(')');
        }
        Expr::Invoke(c, args) => {
            write_expr(out, c, 0);
            out.push('(');
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_expr(out, arg, 0);
            }
            out.push(')');
        }
        Expr::Array(rows) => {
            out.push('{');
            for (r_idx, row) in rows.iter().enumerate() {
                if r_idx > 0 {
                    out.push(';');
                }
                for (c_idx, cell) in row.iter().enumerate() {
                    if c_idx > 0 {
                        out.push(',');
                    }
                    write_expr(out, cell, 0);
                }
            }
            out.push('}');
        }
        Expr::Paren(inner) => {
            out.push('(');
            write_expr(out, inner, 0);
            out.push(')');
        }
        Expr::Missing => {}
    }
}

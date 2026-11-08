//! Reference adjustment on row/col insert, delete, and copy.

use crate::core::addr::{MAX_COLS, MAX_ROWS, RangeRef};
use crate::core::value::CellError;
use crate::formula::ast::*;

pub fn shift_relative(e: Expr, dr: i64, dc: i64) -> Expr {
    e.map(&mut |x| match x {
        Expr::Ref(r) => match shift_ref(&r, dr, dc) {
            Some(r) => Expr::Ref(r),
            None => Expr::Error(CellError::Ref),
        },
        other => other,
    })
}

fn shift_anchor(a: &Anchor, dr: i64, dc: i64) -> Option<Anchor> {
    let row = if a.row_abs { a.row as i64 } else { a.row as i64 + dr };
    let col = if a.col_abs { a.col as i64 } else { a.col as i64 + dc };
    if row < 0 || row >= MAX_ROWS as i64 || col < 0 || col >= MAX_COLS as i64 {
        return None;
    }
    Some(Anchor {
        row: row as u32,
        col: col as u32,
        ..*a
    })
}

fn shift_line(v: u32, abs: bool, d: i64, max: u32) -> Option<u32> {
    if abs {
        return Some(v);
    }
    let n = v as i64 + d;
    if (0..max as i64).contains(&n) {
        Some(n as u32)
    } else {
        None
    }
}

fn shift_ref(r: &Reference, dr: i64, dc: i64) -> Option<Reference> {
    let kind = match &r.kind {
        RefKind::Cell(a) => RefKind::Cell(shift_anchor(a, dr, dc)?),
        RefKind::Range(a, b) => RefKind::Range(shift_anchor(a, dr, dc)?, shift_anchor(b, dr, dc)?),
        RefKind::Rows(r0, a0, r1, a1) => {
            RefKind::Rows(shift_line(*r0, *a0, dr, MAX_ROWS)?, *a0, shift_line(*r1, *a1, dr, MAX_ROWS)?, *a1)
        }
        RefKind::Cols(c0, a0, c1, a1) => {
            RefKind::Cols(shift_line(*c0, *a0, dc, MAX_COLS)?, *a0, shift_line(*c1, *a1, dc, MAX_COLS)?, *a1)
        }
    };
    Some(Reference {
        sheet: r.sheet.clone(),
        kind,
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    Rows,
    Cols,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Edit {
    Insert { axis: Axis, at: u32, count: u32 },
    Delete { axis: Axis, at: u32, count: u32 },
    Move { from: RangeRef, to_row: u32, to_col: u32 },
}

pub fn adjust(e: Expr, host: &str, target: &str, edit: &Edit) -> Expr {
    e.map(&mut |x| match x {
        Expr::Ref(r) => {
            let on_target = match &r.sheet {
                SheetSel::Current => host.eq_ignore_ascii_case(target),
                SheetSel::Named(n) => n.eq_ignore_ascii_case(target),
                SheetSel::Span(..) => false,
            };
            if !on_target {
                return Expr::Ref(r);
            }
            match adjust_ref(&r, edit) {
                Some(r) => Expr::Ref(r),
                None => Expr::Error(CellError::Ref),
            }
        }
        other => other,
    })
}

fn adjust_ref(r: &Reference, edit: &Edit) -> Option<Reference> {
    match edit {
        Edit::Insert { axis, at, count } => match (axis, &r.kind) {
            (Axis::Rows, RefKind::Cell(a)) => {
                let row = if a.row >= *at { a.row.checked_add(*count)? } else { a.row };
                if row >= MAX_ROWS { return None; }
                Some(Reference { sheet: r.sheet.clone(), kind: RefKind::Cell(Anchor { row, ..*a }) })
            }
            (Axis::Cols, RefKind::Cell(a)) => {
                let col = if a.col >= *at { a.col.checked_add(*count)? } else { a.col };
                if col >= MAX_COLS { return None; }
                Some(Reference { sheet: r.sheet.clone(), kind: RefKind::Cell(Anchor { col, ..*a }) })
            }
            _ => Some(r.clone()),
        },
        Edit::Delete { axis, at, count } => match (axis, &r.kind) {
            (Axis::Rows, RefKind::Cell(a)) => {
                if a.row >= *at && a.row < *at + *count {
                    return None;
                }
                let row = if a.row >= *at + *count { a.row - *count } else { a.row };
                Some(Reference { sheet: r.sheet.clone(), kind: RefKind::Cell(Anchor { row, ..*a }) })
            }
            (Axis::Cols, RefKind::Cell(a)) => {
                if a.col >= *at && a.col < *at + *count {
                    return None;
                }
                let col = if a.col >= *at + *count { a.col - *count } else { a.col };
                Some(Reference { sheet: r.sheet.clone(), kind: RefKind::Cell(Anchor { col, ..*a }) })
            }
            _ => Some(r.clone()),
        },
        _ => Some(r.clone()),
    }
}

//! Formula evaluator: expressions -> values and references, LET/LAMBDA closures, dynamic arrays.

use std::sync::Arc;

use crate::core::addr::{CellRef, RangeRef};
use crate::core::value::{Array, CellError, Value, compare};
use crate::formula::ast::{BinOp, Expr, RefKind, Reference, SheetSel, UnOp};
use crate::calc::model::Workbook;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    pub sheet: usize,
    pub range: RangeRef,
}

#[derive(Clone, Debug)]
pub struct Lambda {
    pub params: Vec<String>,
    pub body: Expr,
    pub env: Vec<(String, Ev)>,
}

#[derive(Clone, Debug)]
pub enum Ev {
    V(Value),
    R(Vec<Area>),
    L(Arc<Lambda>),
}

impl From<Value> for Ev {
    fn from(v: Value) -> Self {
        Ev::V(v)
    }
}

pub trait Host {
    fn workbook(&self) -> &Workbook;
    fn cell_value(&mut self, sheet: usize, c: CellRef) -> Value;
    fn now_serial(&self) -> f64;
    fn random(&mut self) -> f64;
    fn note_volatile(&mut self) {}
    fn spill_range(&mut self, sheet: usize, anchor: CellRef) -> Option<RangeRef> {
        self.workbook().sheet(sheet)?.spill_ranges.get(&anchor).copied()
    }
    fn range_values(&mut self, sheet: usize, range: RangeRef) -> Vec<Value> {
        range.iter().map(|c| self.cell_value(sheet, c)).collect()
    }
}

pub struct Evaluator<'h> {
    pub host: &'h mut dyn Host,
    pub sheet: usize,
    pub at: CellRef,
    pub env: Vec<(String, Ev)>,
    pub depth: usize,
}

impl<'h> Evaluator<'h> {
    pub fn new(host: &'h mut dyn Host, sheet: usize, at: CellRef) -> Self {
        Evaluator {
            host,
            sheet,
            at,
            env: Vec::new(),
            depth: 0,
        }
    }

    pub fn eval(&mut self, e: &Expr) -> Ev {
        if self.depth > 256 {
            return Ev::V(Value::Error(CellError::Calc));
        }
        self.depth += 1;
        let res = self.eval_inner(e);
        self.depth -= 1;
        res
    }

    fn eval_inner(&mut self, e: &Expr) -> Ev {
        match e {
            Expr::Number(n) => Ev::V(Value::Number(*n)),
            Expr::Text(s) => Ev::V(Value::Text(s.to_string())),
            Expr::Bool(b) => Ev::V(Value::Bool(*b)),
            Expr::Error(err) => Ev::V(Value::Error(*err)),
            Expr::Name(name) => {
                let u = name.to_ascii_uppercase();
                for (k, v) in self.env.iter().rev() {
                    if k.eq_ignore_ascii_case(&u) {
                        return v.clone();
                    }
                }
                Ev::V(Value::Error(CellError::Name))
            }
            Expr::Paren(inner) => self.eval(inner),
            Expr::Ref(r) => self.eval_ref(r),
            Expr::Unary(op, inner) => self.eval_unary(*op, inner),
            Expr::Binary(op, left, right) => self.eval_binary(*op, left, right),
            Expr::Call(name, args) => self.eval_call(name, args),
            Expr::Invoke(target, args) => self.eval_invoke(target, args),
            Expr::Array(rows) => {
                let mut data = Vec::new();
                let r_count = rows.len();
                let c_count = rows.first().map_or(0, |r| r.len());
                for r in rows {
                    for cell in r {
                        let ev = self.eval(cell);
                        data.push(self.to_value(ev));
                    }
                }
                if let Some(arr) = Array::new(r_count, c_count, data) {
                    Ev::V(Value::Array(Arc::new(arr)))
                } else {
                    Ev::V(Value::Error(CellError::Value))
                }
            }
            Expr::Struct(_) => Ev::V(Value::Error(CellError::Ref)),
            Expr::Missing => Ev::V(Value::Empty),
        }
    }

    fn eval_ref(&mut self, r: &Reference) -> Ev {
        let sheet_idx = match &r.sheet {
            SheetSel::Current => self.sheet,
            SheetSel::Named(name) => {
                match self.host.workbook().sheets.iter().position(|s| s.name.eq_ignore_ascii_case(name)) {
                    Some(idx) => idx,
                    None => return Ev::V(Value::Error(CellError::Ref)),
                }
            }
            SheetSel::Span(..) => return Ev::V(Value::Error(CellError::Ref)),
        };

        match &r.kind {
            RefKind::Cell(a) => {
                let c = a.cell();
                Ev::R(vec![Area { sheet: sheet_idx, range: RangeRef::cell(c) }])
            }
            RefKind::Range(a, b) => {
                let range = RangeRef::new(a.cell(), b.cell());
                Ev::R(vec![Area { sheet: sheet_idx, range }])
            }
            RefKind::Rows(r0, _, r1, _) => {
                let range = RangeRef::rows(*r0, *r1);
                Ev::R(vec![Area { sheet: sheet_idx, range }])
            }
            RefKind::Cols(c0, _, c1, _) => {
                let range = RangeRef::cols(*c0, *c1);
                Ev::R(vec![Area { sheet: sheet_idx, range }])
            }
        }
    }

    pub fn to_value(&mut self, ev: Ev) -> Value {
        match ev {
            Ev::V(v) => v,
            Ev::R(areas) => {
                if let Some(first) = areas.first() {
                    if first.range.is_single() {
                        self.host.cell_value(first.sheet, first.range.start)
                    } else {
                        let vals = self.host.range_values(first.sheet, first.range);
                        if let Some(arr) = Array::new(first.range.height() as usize, first.range.width() as usize, vals) {
                            Value::Array(Arc::new(arr))
                        } else {
                            Value::Error(CellError::Value)
                        }
                    }
                } else {
                    Value::Error(CellError::Ref)
                }
            }
            Ev::L(_) => Value::Error(CellError::Calc),
        }
    }

    fn eval_unary(&mut self, op: UnOp, inner: &Expr) -> Ev {
        let ev = self.eval(inner);
        match op {
            UnOp::Neg => {
                let v = self.to_value(ev);
                if let Some(n) = v.as_f64() {
                    Ev::V(Value::Number(-n))
                } else {
                    Ev::V(Value::Error(CellError::Value))
                }
            }
            UnOp::Plus => {
                let v = self.to_value(ev);
                if let Some(n) = v.as_f64() {
                    Ev::V(Value::Number(n))
                } else {
                    Ev::V(Value::Error(CellError::Value))
                }
            }
            UnOp::Percent => {
                let v = self.to_value(ev);
                if let Some(n) = v.as_f64() {
                    Ev::V(Value::Number(n / 100.0))
                } else {
                    Ev::V(Value::Error(CellError::Value))
                }
            }
            UnOp::At => {
                let v = self.to_value(ev);
                match v {
                    Value::Array(arr) => Ev::V(arr.get(0, 0).cloned().unwrap_or(Value::Empty)),
                    other => Ev::V(other),
                }
            }
            UnOp::Spill => {
                if let Ev::R(areas) = ev {
                    if let Some(area) = areas.first() {
                        if let Some(spill_r) = self.host.spill_range(area.sheet, area.range.start) {
                            return Ev::R(vec![Area { sheet: area.sheet, range: spill_r }]);
                        }
                    }
                }
                Ev::V(Value::Error(CellError::Ref))
            }
        }
    }

    fn eval_binary(&mut self, op: BinOp, left: &Expr, right: &Expr) -> Ev {
        if op == BinOp::Range {
            let el = self.eval(left);
            let er = self.eval(right);
            if let (Ev::R(al), Ev::R(ar)) = (el, er) {
                if let (Some(a), Some(b)) = (al.first(), ar.first()) {
                    if a.sheet == b.sheet {
                        return Ev::R(vec![Area { sheet: a.sheet, range: a.range.union(&b.range) }]);
                    }
                }
            }
            return Ev::V(Value::Error(CellError::Value));
        }

        let el = self.eval(left);
        let vl = self.to_value(el);
        let er = self.eval(right);
        let vr = self.to_value(er);

        if vl.is_error() { return Ev::V(vl); }
        if vr.is_error() { return Ev::V(vr); }

        match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Pow => {
                let (Some(a), Some(b)) = (vl.as_f64(), vr.as_f64()) else {
                    return Ev::V(Value::Error(CellError::Value));
                };
                let res = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    BinOp::Div => if b == 0.0 { return Ev::V(Value::Error(CellError::Div0)); } else { a / b },
                    BinOp::Pow => a.powf(b),
                    _ => 0.0,
                };
                Ev::V(Value::Number(res))
            }
            BinOp::Concat => {
                let s1 = vl.to_display_string();
                let s2 = vr.to_display_string();
                Ev::V(Value::Text(format!("{s1}{s2}")))
            }
            BinOp::Eq => Ev::V(Value::Bool(compare(&vl, &vr).is_eq())),
            BinOp::Ne => Ev::V(Value::Bool(!compare(&vl, &vr).is_eq())),
            BinOp::Lt => Ev::V(Value::Bool(compare(&vl, &vr).is_lt())),
            BinOp::Gt => Ev::V(Value::Bool(compare(&vl, &vr).is_gt())),
            BinOp::Le => Ev::V(Value::Bool(compare(&vl, &vr).is_le())),
            BinOp::Ge => Ev::V(Value::Bool(compare(&vl, &vr).is_ge())),
            _ => Ev::V(Value::Error(CellError::Value)),
        }
    }

    fn eval_call(&mut self, name: &str, args: &[Expr]) -> Ev {
        let u = name.to_ascii_uppercase();
        if u == "LET" {
            return self.eval_let(args);
        }
        if u == "LAMBDA" {
            return self.eval_lambda(args);
        }
        if u == "IF" && (2..=3).contains(&args.len()) {
            let cond_ev = self.eval(&args[0]);
            let cond_val = self.to_value(cond_ev);
            let is_true = match cond_val {
                Value::Bool(b) => b,
                Value::Number(n) => n != 0.0,
                Value::Text(s) => s.eq_ignore_ascii_case("TRUE"),
                _ => false,
            };
            return if is_true {
                self.eval(&args[1])
            } else if args.len() == 3 {
                self.eval(&args[2])
            } else {
                Ev::V(Value::Bool(false))
            };
        }

        // Standard function library dispatch
        let mut evaluated_args = Vec::new();
        for arg in args {
            let ev = self.eval(arg);
            let val = self.to_value(ev);
            evaluated_args.push(val);
        }

        let res = crate::functions::dispatch(&u, &evaluated_args, self.host.workbook().date_system);
        Ev::V(res)
    }

    fn eval_let(&mut self, args: &[Expr]) -> Ev {
        if args.len() < 3 || args.len() % 2 == 0 {
            return Ev::V(Value::Error(CellError::Value));
        }
        let pairs = (args.len() - 1) / 2;
        let prev_len = self.env.len();
        for k in 0..pairs {
            let name_expr = &args[k * 2];
            let val_expr = &args[k * 2 + 1];
            let name = match name_expr {
                Expr::Name(n) => n.clone(),
                _ => {
                    self.env.truncate(prev_len);
                    return Ev::V(Value::Error(CellError::Value));
                }
            };
            let ev = self.eval(val_expr);
            self.env.push((name, ev));
        }
        let final_expr = &args[args.len() - 1];
        let res = self.eval(final_expr);
        self.env.truncate(prev_len);
        res
    }

    fn eval_lambda(&mut self, args: &[Expr]) -> Ev {
        if args.is_empty() {
            return Ev::V(Value::Error(CellError::Value));
        }
        let body = args.last().cloned().unwrap_or(Expr::Missing);
        let mut params = Vec::new();
        for arg in &args[..args.len() - 1] {
            if let Expr::Name(p) = arg {
                params.push(p.clone());
            } else {
                return Ev::V(Value::Error(CellError::Value));
            }
        }
        Ev::L(Arc::new(Lambda {
            params,
            body,
            env: self.env.clone(),
        }))
    }

    fn eval_invoke(&mut self, target: &Expr, args: &[Expr]) -> Ev {
        let ev = self.eval(target);
        match ev {
            Ev::L(lambda) => {
                if args.len() != lambda.params.len() {
                    return Ev::V(Value::Error(CellError::Value));
                }
                let mut arg_vals = Vec::new();
                for arg in args {
                    arg_vals.push(self.eval(arg));
                }
                let prev_env = std::mem::replace(&mut self.env, lambda.env.clone());
                for (p, v) in lambda.params.iter().zip(arg_vals) {
                    self.env.push((p.clone(), v));
                }
                let res = self.eval(&lambda.body);
                self.env = prev_env;
                res
            }
            _ => Ev::V(Value::Error(CellError::Calc)),
        }
    }
}

pub fn precedents(wb: &Workbook, sheet: usize, e: &Expr) -> (Vec<Area>, bool) {
    let mut areas = Vec::new();
    let mut dynamic = false;

    e.walk(&mut |node| match node {
        Expr::Ref(r) => {
            let si = match &r.sheet {
                SheetSel::Current => sheet,
                SheetSel::Named(name) => {
                    wb.sheets.iter().position(|s| s.name.eq_ignore_ascii_case(name)).unwrap_or(sheet)
                }
                SheetSel::Span(..) => sheet,
            };
            areas.push(Area { sheet: si, range: r.range() });
        }
        Expr::Call(name, _) => {
            let u = name.to_ascii_uppercase();
            if matches!(u.as_str(), "OFFSET" | "INDIRECT" | "RAND" | "RANDBETWEEN" | "NOW" | "TODAY") {
                dynamic = true;
            }
        }
        _ => {}
    });

    (areas, dynamic)
}

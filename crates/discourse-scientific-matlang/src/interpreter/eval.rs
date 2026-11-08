//! Workspace evaluation engine and standard numerical math builtins.

use crate::interpreter::ast::{BinOp, Expr, Stmt, UnOp};
use crate::interpreter::lexer::LangError;
use crate::matrix::{chol, eig_symmetric, lu, qr, svd, Matrix};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Num(f64),
    Mat(Matrix),
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn as_matrix(&self) -> Matrix {
        match self {
            Value::Mat(m) => m.clone(),
            Value::Num(n) => Matrix::scalar(*n),
            Value::Bool(b) => Matrix::scalar(if *b { 1.0 } else { 0.0 }),
            Value::Str(_) => Matrix::zeros(0, 0),
        }
    }

    pub fn as_num(&self) -> Option<f64> {
        match self {
            Value::Num(n) => Some(*n),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Mat(m) => m.as_scalar(),
            _ => None,
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Num(n) => *n != 0.0 && !n.is_nan(),
            Value::Mat(m) => m.data.iter().any(|&v| v != 0.0),
            Value::Str(s) => !s.is_empty(),
        }
    }
}

pub struct Workspace {
    pub vars: HashMap<String, Value>,
    pub output: Vec<String>,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    pub fn new() -> Self {
        let mut ws = Self {
            vars: HashMap::new(),
            output: Vec::new(),
        };
        ws.vars.insert("pi".to_string(), Value::Num(std::f64::consts::PI));
        ws.vars.insert("eps".to_string(), Value::Num(f64::EPSILON));
        ws
    }

    pub fn exec(&mut self, stmts: &[Stmt]) -> Result<(), LangError> {
        for stmt in stmts {
            match stmt {
                Stmt::Assign { name, value, suppress } => {
                    let v = self.eval_expr(value)?;
                    if !suppress {
                        self.output.push(format!("{name} = {:?}", v));
                    }
                    self.vars.insert(name.clone(), v);
                }
                Stmt::Expr(expr, suppress) => {
                    let v = self.eval_expr(expr)?;
                    if !suppress {
                        self.output.push(format!("ans = {:?}", v));
                    }
                    self.vars.insert("ans".to_string(), v);
                }
                Stmt::If { cond, then_body, else_body } => {
                    let c = self.eval_expr(cond)?;
                    if c.is_truthy() {
                        self.exec(then_body)?;
                    } else if let Some(eb) = else_body {
                        self.exec(eb)?;
                    }
                }
                Stmt::For { var, iter, body } => {
                    let iter_val = self.eval_expr(iter)?;
                    let mat = iter_val.as_matrix();
                    for col in 0..mat.cols {
                        let col_val = if mat.rows == 1 {
                            Value::Num(mat.data[col])
                        } else {
                            Value::Mat(mat.slice(0, mat.rows, col, col + 1).unwrap())
                        };
                        self.vars.insert(var.clone(), col_val);
                        self.exec(body)?;
                    }
                }
                Stmt::While { cond, body } => {
                    let mut iters = 0;
                    while self.eval_expr(cond)?.is_truthy() && iters < 100_000 {
                        self.exec(body)?;
                        iters += 1;
                    }
                }
                Stmt::Return => break,
            }
        }
        Ok(())
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, LangError> {
        match expr {
            Expr::Num(n) => Ok(Value::Num(*n)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Ident(id) => self
                .vars
                .get(id)
                .cloned()
                .ok_or_else(|| LangError {
                    message: format!("undefined variable `{id}`"),
                    line: 0,
                }),
            Expr::Range(start_e, step_e, end_e) => {
                let start = self.eval_expr(start_e)?.as_num().unwrap_or(0.0);
                let step = if let Some(se) = step_e {
                    self.eval_expr(se)?.as_num().unwrap_or(1.0)
                } else {
                    1.0
                };
                let end = self.eval_expr(end_e)?.as_num().unwrap_or(0.0);

                let mut vals = Vec::new();
                let mut curr = start;
                if step > 0.0 {
                    while curr <= end + 1e-12 {
                        vals.push(curr);
                        curr += step;
                    }
                } else if step < 0.0 {
                    while curr >= end - 1e-12 {
                        vals.push(curr);
                        curr += step;
                    }
                }
                Ok(Value::Mat(Matrix::row_vector(vals)))
            }
            Expr::Matrix(rows) => {
                let mut data = Vec::new();
                let num_rows = rows.len();
                let mut num_cols = 0;

                for (r_idx, row) in rows.iter().enumerate() {
                    let mut row_vals = Vec::new();
                    for elem_e in row {
                        let ev = self.eval_expr(elem_e)?;
                        let m = ev.as_matrix();
                        row_vals.extend_from_slice(&m.data);
                    }
                    if r_idx == 0 {
                        num_cols = row_vals.len();
                    }
                    data.extend_from_slice(&row_vals);
                }
                if num_rows == 0 || num_cols == 0 {
                    return Ok(Value::Mat(Matrix::zeros(0, 0)));
                }
                let mat = Matrix::from_row_major(num_rows, num_cols, &data).map_err(|e| LangError {
                    message: format!("matrix literal error: {e}"),
                    line: 0,
                })?;
                Ok(Value::Mat(mat))
            }
            Expr::Transpose(e) => {
                let v = self.eval_expr(e)?;
                let m = v.as_matrix();
                Ok(Value::Mat(m.transpose()))
            }
            Expr::Unary(op, e) => {
                let v = self.eval_expr(e)?;
                match op {
                    UnOp::Neg => {
                        let m = v.as_matrix();
                        Ok(Value::Mat(m.scale(-1.0)))
                    }
                    UnOp::Not => Ok(Value::Bool(!v.is_truthy())),
                }
            }
            Expr::Binary(op, left_e, right_e) => {
                let left = self.eval_expr(left_e)?;
                let right = self.eval_expr(right_e)?;
                self.eval_binop(*op, &left, &right)
            }
            Expr::CallOrIndex(func_or_mat, args) => {
                let mut arg_vals = Vec::new();
                for a in args {
                    arg_vals.push(self.eval_expr(a)?);
                }
                self.eval_call(func_or_mat, &arg_vals)
            }
        }
    }

    fn eval_binop(&self, op: BinOp, left: &Value, right: &Value) -> Result<Value, LangError> {
        let err = |msg: String| LangError { message: msg, line: 0 };
        match (left, right) {
            (Value::Num(a), Value::Num(b)) => match op {
                BinOp::Add => Ok(Value::Num(a + b)),
                BinOp::Sub => Ok(Value::Num(a - b)),
                BinOp::Mul | BinOp::ElemMul => Ok(Value::Num(a * b)),
                BinOp::Div | BinOp::ElemDiv => Ok(Value::Num(a / b)),
                BinOp::Pow | BinOp::ElemPow => Ok(Value::Num(a.powf(*b))),
                BinOp::Eq => Ok(Value::Bool((a - b).abs() < 1e-12)),
                BinOp::Neq => Ok(Value::Bool((a - b).abs() >= 1e-12)),
                BinOp::Lt => Ok(Value::Bool(a < b)),
                BinOp::Le => Ok(Value::Bool(a <= b)),
                BinOp::Gt => Ok(Value::Bool(a > b)),
                BinOp::Ge => Ok(Value::Bool(a >= b)),
                BinOp::And => Ok(Value::Bool(*a != 0.0 && *b != 0.0)),
                BinOp::Or => Ok(Value::Bool(*a != 0.0 || *b != 0.0)),
            },
            _ => {
                let ma = left.as_matrix();
                let mb = right.as_matrix();
                match op {
                    BinOp::Add => Ok(Value::Mat(ma.elem_add(&mb).map_err(|e| err(e.to_string()))?)),
                    BinOp::Sub => Ok(Value::Mat(ma.elem_sub(&mb).map_err(|e| err(e.to_string()))?)),
                    BinOp::Mul => Ok(Value::Mat(ma.matmul(&mb).map_err(|e| err(e.to_string()))?)),
                    BinOp::ElemMul => Ok(Value::Mat(ma.elem_mul(&mb).map_err(|e| err(e.to_string()))?)),
                    BinOp::Div | BinOp::ElemDiv => {
                        let mut out = ma.clone();
                        for (a, &b) in out.data.iter_mut().zip(&mb.data) {
                            *a /= b;
                        }
                        Ok(Value::Mat(out))
                    }
                    BinOp::Eq => Ok(Value::Bool(ma == mb)),
                    BinOp::Neq => Ok(Value::Bool(ma != mb)),
                    _ => Err(err(format!("unsupported binary operation on matrices: {:?}", op))),
                }
            }
        }
    }

    fn eval_call(&mut self, name: &str, args: &[Value]) -> Result<Value, LangError> {
        let err = |msg: String| LangError { message: msg, line: 0 };

        // 1. Check if name is in workspace (1-based Matrix indexing)
        if let Some(val) = self.vars.get(name) {
            if let Value::Mat(m) = val {
                if args.len() == 1 {
                    let idx = args[0].as_num().unwrap_or(1.0) as usize;
                    if idx >= 1 && idx <= m.data.len() {
                        return Ok(Value::Num(m.data[idx - 1]));
                    }
                } else if args.len() == 2 {
                    let r = args[0].as_num().unwrap_or(1.0) as usize;
                    let c = args[1].as_num().unwrap_or(1.0) as usize;
                    if r >= 1 && r <= m.rows && c >= 1 && c <= m.cols {
                        return Ok(Value::Num(m.get(r - 1, c - 1).unwrap()));
                    }
                }
            }
        }

        // 2. Standard math builtins
        match name {
            "zeros" => {
                let r = args.first().and_then(Value::as_num).unwrap_or(1.0) as usize;
                let c = args.get(1).and_then(Value::as_num).unwrap_or(r as f64) as usize;
                Ok(Value::Mat(Matrix::zeros(r, c)))
            }
            "ones" => {
                let r = args.first().and_then(Value::as_num).unwrap_or(1.0) as usize;
                let c = args.get(1).and_then(Value::as_num).unwrap_or(r as f64) as usize;
                Ok(Value::Mat(Matrix::ones(r, c)))
            }
            "eye" => {
                let r = args.first().and_then(Value::as_num).unwrap_or(1.0) as usize;
                let c = args.get(1).and_then(Value::as_num).unwrap_or(r as f64) as usize;
                Ok(Value::Mat(Matrix::eye(r, c)))
            }
            "det" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("det needs 1 arg".into()))?;
                let d = m.det().map_err(|e| err(e.to_string()))?;
                Ok(Value::Num(d))
            }
            "inv" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("inv needs 1 arg".into()))?;
                let inv_m = m.inverse().map_err(|e| err(e.to_string()))?;
                Ok(Value::Mat(inv_m))
            }
            "lu" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("lu needs 1 arg".into()))?;
                let (l, u, _p) = lu(&m).map_err(|e| err(e.to_string()))?;
                Ok(Value::Mat(l.elem_add(&u).unwrap()))
            }
            "qr" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("qr needs 1 arg".into()))?;
                let (_q, r) = qr(&m).map_err(|e| err(e.to_string()))?;
                Ok(Value::Mat(r))
            }
            "chol" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("chol needs 1 arg".into()))?;
                let r = chol(&m).map_err(|e| err(e.to_string()))?;
                Ok(Value::Mat(r))
            }
            "svd" => {
                let m = args.first().map(Value::as_matrix).ok_or_else(|| err("svd needs 1 arg".into()))?;
                let (_u, s, _v) = svd(&m).map_err(|e| err(e.to_string()))?;
                Ok(Value::Mat(s))
            }
            "sin" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(0.0));
                Ok(Value::Mat(m.map(|x| x.sin())))
            }
            "cos" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(0.0));
                Ok(Value::Mat(m.map(|x| x.cos())))
            }
            "exp" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(0.0));
                Ok(Value::Mat(m.map(|x| x.exp())))
            }
            "log" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(1.0));
                Ok(Value::Mat(m.map(|x| x.ln())))
            }
            "sqrt" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(0.0));
                Ok(Value::Mat(m.map(|x| x.sqrt())))
            }
            "sum" => {
                let m = args.first().map(Value::as_matrix).unwrap_or_else(|| Matrix::scalar(0.0));
                let total: f64 = m.data.iter().sum();
                Ok(Value::Num(total))
            }
            _ => Err(err(format!("unknown function `{name}`"))),
        }
    }
}

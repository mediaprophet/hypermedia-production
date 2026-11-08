//! Utility helpers for tabular functions.

use std::sync::Arc;
use crate::core::value::{Array, CellError, Value};

pub type R<T> = Result<T, CellError>;

pub fn num(args: &[Value], idx: usize) -> R<f64> {
    let v = args.get(idx).ok_or(CellError::Value)?;
    to_f64(v)
}

pub fn opt_num(args: &[Value], idx: usize, default: f64) -> R<f64> {
    match args.get(idx) {
        Some(Value::Empty) | None => Ok(default),
        Some(v) => to_f64(v),
    }
}

pub fn to_f64(v: &Value) -> R<f64> {
    match v {
        Value::Number(n) => Ok(*n),
        Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
        Value::Text(s) => s.trim().parse::<f64>().map_err(|_| CellError::Value),
        Value::Empty => Ok(0.0),
        Value::Error(e) => Err(*e),
        Value::Array(a) => {
            if let Some(first) = a.get(0, 0) {
                to_f64(first)
            } else {
                Err(CellError::Value)
            }
        }
    }
}

pub fn flatten_numbers(args: &[Value]) -> R<Vec<f64>> {
    let mut out = Vec::new();
    for arg in args {
        match arg {
            Value::Number(n) => out.push(*n),
            Value::Array(arr) => {
                for v in &arr.data {
                    if let Value::Number(n) = v {
                        out.push(*n);
                    }
                }
            }
            Value::Error(e) => return Err(*e),
            _ => {}
        }
    }
    Ok(out)
}

pub fn as_array(v: &Value) -> Arc<Array> {
    match v {
        Value::Array(a) => a.clone(),
        single => Arc::new(Array::single(single.clone())),
    }
}

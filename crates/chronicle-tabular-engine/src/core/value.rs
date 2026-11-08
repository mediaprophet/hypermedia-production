//! Cell values and tabular calculation coercion/comparison rules.

use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// Excel-compatible calculation error values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellError {
    Null,
    Div0,
    Value,
    Ref,
    Name,
    Num,
    NA,
    GettingData,
    Spill,
    Calc,
    Circ,
}

impl CellError {
    pub const ALL: [CellError; 11] = [
        CellError::Null,
        CellError::Div0,
        CellError::Value,
        CellError::Ref,
        CellError::Name,
        CellError::Num,
        CellError::NA,
        CellError::GettingData,
        CellError::Spill,
        CellError::Calc,
        CellError::Circ,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            CellError::Null => "#NULL!",
            CellError::Div0 => "#DIV/0!",
            CellError::Value => "#VALUE!",
            CellError::Ref => "#REF!",
            CellError::Name => "#NAME?",
            CellError::Num => "#NUM!",
            CellError::NA => "#N/A",
            CellError::GettingData => "#GETTING_DATA",
            CellError::Spill => "#SPILL!",
            CellError::Calc => "#CALC!",
            CellError::Circ => "#CIRC!",
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            CellError::Null => 1,
            CellError::Div0 => 2,
            CellError::Value => 3,
            CellError::Ref => 4,
            CellError::Name => 5,
            CellError::Num => 6,
            CellError::NA => 7,
            CellError::GettingData => 8,
            CellError::Spill => 9,
            CellError::Calc => 14,
            CellError::Circ => 15,
        }
    }

    pub fn parse(s: &str) -> Option<CellError> {
        let u = s.to_ascii_uppercase();
        Self::ALL.iter().find(|e| e.as_str() == u).copied()
    }
}

impl fmt::Display for CellError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A 2-D array of values (row-major). Always at least 1x1.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Array {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Value>,
}

impl Array {
    pub fn new(rows: usize, cols: usize, data: Vec<Value>) -> Option<Array> {
        if rows == 0 || cols == 0 || rows.checked_mul(cols)? != data.len() {
            return None;
        }
        Some(Array { rows, cols, data })
    }

    pub fn filled(rows: usize, cols: usize, v: Value) -> Array {
        let (rows, cols) = (rows.max(1), cols.max(1));
        Array {
            rows,
            cols,
            data: vec![v; rows.saturating_mul(cols)],
        }
    }

    pub fn single(v: Value) -> Array {
        Array {
            rows: 1,
            cols: 1,
            data: vec![v],
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&Value> {
        if r < self.rows && c < self.cols {
            self.data.get(r * self.cols + c)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Value> {
        if r < self.rows && c < self.cols {
            let i = r * self.cols + c;
            self.data.get_mut(i)
        } else {
            None
        }
    }
}

/// A cell's evaluated value.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum Value {
    #[default]
    Empty,
    Number(f64),
    Text(String),
    Bool(bool),
    Error(CellError),
    Array(Arc<Array>),
}

impl Value {
    pub fn number(n: f64) -> Value {
        Value::Number(n)
    }

    pub fn text(s: impl Into<String>) -> Value {
        Value::Text(s.into())
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, Value::Error(_))
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::Empty => Some(0.0),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Empty => String::new(),
            Value::Number(n) => number_to_text(*n),
            Value::Text(s) => s.clone(),
            Value::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            Value::Error(e) => e.as_str().to_string(),
            Value::Array(a) => {
                if let Some(v) = a.get(0, 0) {
                    v.to_display_string()
                } else {
                    String::new()
                }
            }
        }
    }
}

pub fn number_to_text(n: f64) -> String {
    if n.is_nan() {
        return "#NUM!".to_string();
    }
    if n.is_infinite() {
        return if n > 0.0 { "#NUM!" } else { "-#NUM!" }.to_string();
    }
    if n == 0.0 {
        return "0".to_string();
    }
    let s = format!("{n}");
    if s.contains('e') || s.contains('E') {
        s
    } else {
        s
    }
}

pub fn compare_numbers(a: f64, b: f64) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

pub fn compare_text(a: &str, b: &str) -> Ordering {
    a.to_lowercase().cmp(&b.to_lowercase())
}

pub fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Empty, Value::Empty) => Ordering::Equal,
        (Value::Empty, _) => Ordering::Less,
        (_, Value::Empty) => Ordering::Greater,
        (Value::Number(x), Value::Number(y)) => compare_numbers(*x, *y),
        (Value::Text(x), Value::Text(y)) => compare_text(x, y),
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        (Value::Error(x), Value::Error(y)) => x.code().cmp(&y.code()),
        (Value::Number(_), Value::Text(_)) => Ordering::Less,
        (Value::Text(_), Value::Number(_)) => Ordering::Greater,
        (Value::Bool(_), _) => Ordering::Greater,
        (_, Value::Bool(_)) => Ordering::Less,
        (Value::Error(_), _) => Ordering::Greater,
        (_, Value::Error(_)) => Ordering::Less,
        (Value::Array(ax), Value::Array(ay)) => {
            let vx = ax.get(0, 0).unwrap_or(&Value::Empty);
            let vy = ay.get(0, 0).unwrap_or(&Value::Empty);
            compare(vx, vy)
        }
        (Value::Array(ax), other) => {
            let vx = ax.get(0, 0).unwrap_or(&Value::Empty);
            compare(vx, other)
        }
        (other, Value::Array(ay)) => {
            let vy = ay.get(0, 0).unwrap_or(&Value::Empty);
            compare(other, vy)
        }
    }
}

pub fn sort_compare(a: &Value, b: &Value, ascending: bool) -> Ordering {
    let ord = compare(a, b);
    if ascending {
        ord
    } else {
        ord.reverse()
    }
}

//! Dynamic array operators: FILTER, SORT, UNIQUE, SEQUENCE, TAKE, DROP, etc.

use std::sync::Arc;
use crate::core::value::{Array, CellError, Value, sort_compare};
use crate::functions::util::{R, as_array, num, opt_num};

pub fn sequence(args: &[Value]) -> R<Value> {
    let rows = num(args, 0)?.trunc() as i64;
    let cols = opt_num(args, 1, 1.0)?.trunc() as i64;
    let start = opt_num(args, 2, 1.0)?;
    let step = opt_num(args, 3, 1.0)?;

    if rows < 1 || cols < 1 || rows > 100_000 || cols > 16_384 {
        return Err(CellError::Value);
    }
    let r = rows as usize;
    let c = cols as usize;
    let mut data = Vec::with_capacity(r * c);
    let mut val = start;
    for _ in 0..r {
        for _ in 0..c {
            data.push(Value::Number(val));
            val += step;
        }
    }
    let arr = Array::new(r, c, data).ok_or(CellError::Value)?;
    Ok(Value::Array(Arc::new(arr)))
}

pub fn filter(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let inc = as_array(args.get(1).ok_or(CellError::Value)?);
    let if_empty = args.get(2).cloned().unwrap_or(Value::Error(CellError::Calc));

    if inc.rows != arr.rows && inc.cols != arr.cols {
        return Err(CellError::Value);
    }

    if inc.rows == arr.rows {
        // Filter rows
        let mut matched_rows = Vec::new();
        for r in 0..arr.rows {
            let is_match = match inc.get(r, 0) {
                Some(Value::Bool(b)) => *b,
                Some(Value::Number(n)) => *n != 0.0,
                _ => false,
            };
            if is_match {
                matched_rows.push(r);
            }
        }
        if matched_rows.is_empty() {
            return Ok(if_empty);
        }
        let out_rows = matched_rows.len();
        let out_cols = arr.cols;
        let mut data = Vec::with_capacity(out_rows * out_cols);
        for &r in &matched_rows {
            for c in 0..out_cols {
                data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
            }
        }
        let out = Array::new(out_rows, out_cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    } else {
        // Filter columns
        let mut matched_cols = Vec::new();
        for c in 0..arr.cols {
            let is_match = match inc.get(0, c) {
                Some(Value::Bool(b)) => *b,
                Some(Value::Number(n)) => *n != 0.0,
                _ => false,
            };
            if is_match {
                matched_cols.push(c);
            }
        }
        if matched_cols.is_empty() {
            return Ok(if_empty);
        }
        let out_rows = arr.rows;
        let out_cols = matched_cols.len();
        let mut data = Vec::with_capacity(out_rows * out_cols);
        for r in 0..out_rows {
            for &c in &matched_cols {
                data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
            }
        }
        let out = Array::new(out_rows, out_cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    }
}

pub fn unique(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let by_col = opt_num(args, 1, 0.0)? != 0.0;
    let exactly_once = opt_num(args, 2, 0.0)? != 0.0;

    if !by_col {
        // Unique rows
        let mut rows: Vec<Vec<Value>> = Vec::new();
        for r in 0..arr.rows {
            let row_vals: Vec<Value> = (0..arr.cols).map(|c| arr.get(r, c).cloned().unwrap_or(Value::Empty)).collect();
            rows.push(row_vals);
        }
        let mut filtered = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let count = rows.iter().filter(|&x| x == r).count();
            if exactly_once {
                if count == 1 {
                    filtered.push(r.clone());
                }
            } else if !filtered.contains(r) {
                filtered.push(r.clone());
            }
        }
        if filtered.is_empty() {
            return Ok(Value::Error(CellError::Calc));
        }
        let out_rows = filtered.len();
        let out_cols = arr.cols;
        let data: Vec<Value> = filtered.into_iter().flatten().collect();
        let out = Array::new(out_rows, out_cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    } else {
        // Unique columns
        let mut cols: Vec<Vec<Value>> = Vec::new();
        for c in 0..arr.cols {
            let col_vals: Vec<Value> = (0..arr.rows).map(|r| arr.get(r, c).cloned().unwrap_or(Value::Empty)).collect();
            cols.push(col_vals);
        }
        let mut filtered = Vec::new();
        for c in &cols {
            let count = cols.iter().filter(|&x| x == c).count();
            if exactly_once {
                if count == 1 {
                    filtered.push(c.clone());
                }
            } else if !filtered.contains(c) {
                filtered.push(c.clone());
            }
        }
        if filtered.is_empty() {
            return Ok(Value::Error(CellError::Calc));
        }
        let out_rows = arr.rows;
        let out_cols = filtered.len();
        let mut data = Vec::with_capacity(out_rows * out_cols);
        for r in 0..out_rows {
            for c_vec in &filtered {
                data.push(c_vec.get(r).cloned().unwrap_or(Value::Empty));
            }
        }
        let out = Array::new(out_rows, out_cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    }
}

pub fn sort(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let sort_idx = (opt_num(args, 1, 1.0)?.trunc() as i64).max(1) as usize - 1;
    let sort_order = opt_num(args, 2, 1.0)? >= 0.0;
    let by_col = opt_num(args, 3, 0.0)? != 0.0;

    if !by_col {
        if sort_idx >= arr.cols {
            return Err(CellError::Value);
        }
        let mut row_indices: Vec<usize> = (0..arr.rows).collect();
        row_indices.sort_by(|&a, &b| {
            let va = arr.get(a, sort_idx).unwrap_or(&Value::Empty);
            let vb = arr.get(b, sort_idx).unwrap_or(&Value::Empty);
            sort_compare(va, vb, sort_order)
        });
        let mut data = Vec::with_capacity(arr.rows * arr.cols);
        for &r in &row_indices {
            for c in 0..arr.cols {
                data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
            }
        }
        let out = Array::new(arr.rows, arr.cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    } else {
        if sort_idx >= arr.rows {
            return Err(CellError::Value);
        }
        let mut col_indices: Vec<usize> = (0..arr.cols).collect();
        col_indices.sort_by(|&a, &b| {
            let va = arr.get(sort_idx, a).unwrap_or(&Value::Empty);
            let vb = arr.get(sort_idx, b).unwrap_or(&Value::Empty);
            sort_compare(va, vb, sort_order)
        });
        let mut data = Vec::with_capacity(arr.rows * arr.cols);
        for r in 0..arr.rows {
            for &c in &col_indices {
                data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
            }
        }
        let out = Array::new(arr.rows, arr.cols, data).ok_or(CellError::Value)?;
        Ok(Value::Array(Arc::new(out)))
    }
}

pub fn take(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let rows_arg = opt_num(args, 1, arr.rows as f64)?.trunc() as i64;
    let cols_arg = opt_num(args, 2, arr.cols as f64)?.trunc() as i64;

    let r_take = rows_arg.abs() as usize;
    let c_take = cols_arg.abs() as usize;

    let (r_start, r_count) = if rows_arg >= 0 {
        (0, r_take.min(arr.rows))
    } else {
        (arr.rows.saturating_sub(r_take), r_take.min(arr.rows))
    };

    let (c_start, c_count) = if cols_arg >= 0 {
        (0, c_take.min(arr.cols))
    } else {
        (arr.cols.saturating_sub(c_take), c_take.min(arr.cols))
    };

    if r_count == 0 || c_count == 0 {
        return Err(CellError::Calc);
    }

    let mut data = Vec::with_capacity(r_count * c_count);
    for r in r_start..r_start + r_count {
        for c in c_start..c_start + c_count {
            data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
        }
    }
    let out = Array::new(r_count, c_count, data).ok_or(CellError::Value)?;
    Ok(Value::Array(Arc::new(out)))
}

pub fn drop(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let rows_arg = opt_num(args, 1, 0.0)?.trunc() as i64;
    let cols_arg = opt_num(args, 2, 0.0)?.trunc() as i64;

    let r_drop = rows_arg.abs() as usize;
    let c_drop = cols_arg.abs() as usize;

    let (r_start, r_count) = if rows_arg >= 0 {
        (r_drop.min(arr.rows), arr.rows.saturating_sub(r_drop))
    } else {
        (0, arr.rows.saturating_sub(r_drop))
    };

    let (c_start, c_count) = if cols_arg >= 0 {
        (c_drop.min(arr.cols), arr.cols.saturating_sub(c_drop))
    } else {
        (0, arr.cols.saturating_sub(c_drop))
    };

    if r_count == 0 || c_count == 0 {
        return Err(CellError::Calc);
    }

    let mut data = Vec::with_capacity(r_count * c_count);
    for r in r_start..r_start + r_count {
        for c in c_start..c_start + c_count {
            data.push(arr.get(r, c).cloned().unwrap_or(Value::Empty));
        }
    }
    let out = Array::new(r_count, c_count, data).ok_or(CellError::Value)?;
    Ok(Value::Array(Arc::new(out)))
}

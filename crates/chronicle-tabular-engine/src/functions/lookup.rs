//! Lookup and reference functions: XLOOKUP, INDEX, MATCH, XMATCH, VLOOKUP, HLOOKUP.

use crate::core::value::{CellError, Value, compare};
use crate::functions::util::{R, as_array, num, opt_num};

pub fn match_fn(args: &[Value]) -> R<Value> {
    let lookup_val = args.get(0).ok_or(CellError::Value)?;
    let arr = as_array(args.get(1).ok_or(CellError::Value)?);
    let match_type = opt_num(args, 2, 1.0)?.trunc() as i64;

    let items: Vec<&Value> = arr.data.iter().collect();

    if match_type == 0 {
        // Exact match
        for (i, v) in items.iter().enumerate() {
            if compare(lookup_val, v).is_eq() {
                return Ok(Value::Number(i as f64 + 1.0));
            }
        }
        return Err(CellError::NA);
    } else if match_type > 0 {
        // Largest value less than or equal to (assumes ascending order)
        let mut best = None;
        for (i, v) in items.iter().enumerate() {
            if compare(v, lookup_val).is_le() {
                best = Some(i as f64 + 1.0);
            } else {
                break;
            }
        }
        best.map(Value::Number).ok_or(CellError::NA)
    } else {
        // Smallest value greater than or equal to (assumes descending order)
        let mut best = None;
        for (i, v) in items.iter().enumerate() {
            if compare(v, lookup_val).is_ge() {
                best = Some(i as f64 + 1.0);
            } else {
                break;
            }
        }
        best.map(Value::Number).ok_or(CellError::NA)
    }
}

pub fn index_fn(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let row_num = (opt_num(args, 1, 1.0)?.trunc() as i64).max(0) as usize;
    let col_num = (opt_num(args, 2, 1.0)?.trunc() as i64).max(0) as usize;

    if row_num == 0 && col_num == 0 {
        return Ok(Value::Array(arr));
    }

    if row_num > arr.rows || col_num > arr.cols {
        return Err(CellError::Ref);
    }

    let r_idx = if row_num == 0 { 0 } else { row_num - 1 };
    let c_idx = if col_num == 0 { 0 } else { col_num - 1 };

    Ok(arr.get(r_idx, c_idx).cloned().unwrap_or(Value::Empty))
}

pub fn xlookup(args: &[Value]) -> R<Value> {
    let lookup_val = args.get(0).ok_or(CellError::Value)?;
    let lookup_arr = as_array(args.get(1).ok_or(CellError::Value)?);
    let return_arr = as_array(args.get(2).ok_or(CellError::Value)?);
    let if_not_found = args.get(3).cloned().unwrap_or(Value::Error(CellError::NA));
    let match_mode = opt_num(args, 4, 0.0)?.trunc() as i64;

    let n = lookup_arr.data.len();
    let mut found_idx = None;

    if match_mode == 0 {
        // Exact match
        for i in 0..n {
            if let Some(v) = lookup_arr.data.get(i) {
                if compare(lookup_val, v).is_eq() {
                    found_idx = Some(i);
                    break;
                }
            }
        }
    } else if match_mode == -1 {
        // Exact match or next smaller
        let mut best_smaller = None;
        for i in 0..n {
            if let Some(v) = lookup_arr.data.get(i) {
                if compare(lookup_val, v).is_eq() {
                    found_idx = Some(i);
                    break;
                } else if compare(v, lookup_val).is_lt() {
                    best_smaller = Some(i);
                }
            }
        }
        if found_idx.is_none() {
            found_idx = best_smaller;
        }
    } else if match_mode == 1 {
        // Exact match or next larger
        let mut best_larger = None;
        for i in 0..n {
            if let Some(v) = lookup_arr.data.get(i) {
                if compare(lookup_val, v).is_eq() {
                    found_idx = Some(i);
                    break;
                } else if compare(v, lookup_val).is_gt() && best_larger.is_none() {
                    best_larger = Some(i);
                }
            }
        }
        if found_idx.is_none() {
            found_idx = best_larger;
        }
    }

    match found_idx {
        Some(idx) => {
            if return_arr.rows == 1 || return_arr.cols == 1 {
                Ok(return_arr.data.get(idx).cloned().unwrap_or(Value::Empty))
            } else if return_arr.rows == lookup_arr.rows {
                let r = idx;
                let mut row_vals = Vec::with_capacity(return_arr.cols);
                for c in 0..return_arr.cols {
                    row_vals.push(return_arr.get(r, c).cloned().unwrap_or(Value::Empty));
                }
                let arr = crate::core::value::Array::new(1, return_arr.cols, row_vals).ok_or(CellError::Value)?;
                Ok(Value::Array(std::sync::Arc::new(arr)))
            } else {
                Ok(return_arr.data.get(idx).cloned().unwrap_or(Value::Empty))
            }
        }
        None => Ok(if_not_found),
    }
}

pub fn vlookup(args: &[Value]) -> R<Value> {
    let lookup_val = args.get(0).ok_or(CellError::Value)?;
    let table = as_array(args.get(1).ok_or(CellError::Value)?);
    let col_idx = num(args, 2)?.trunc() as usize;
    let exact = opt_num(args, 3, 1.0)? == 0.0;

    if col_idx < 1 || col_idx > table.cols {
        return Err(CellError::Ref);
    }
    let target_col = col_idx - 1;

    let mut match_row = None;
    for r in 0..table.rows {
        let v = table.get(r, 0).unwrap_or(&Value::Empty);
        if exact {
            if compare(lookup_val, v).is_eq() {
                match_row = Some(r);
                break;
            }
        } else {
            if compare(v, lookup_val).is_le() {
                match_row = Some(r);
            } else {
                break;
            }
        }
    }

    match match_row {
        Some(r) => Ok(table.get(r, target_col).cloned().unwrap_or(Value::Empty)),
        None => Err(CellError::NA),
    }
}

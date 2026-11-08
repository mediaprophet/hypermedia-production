//! Text manipulation functions: CONCAT, TEXTJOIN, LEFT, RIGHT, MID, etc.

use crate::core::value::{CellError, Value};
use crate::functions::util::{R, as_array, num, opt_num};

pub fn concat(args: &[Value]) -> R<Value> {
    let mut s = String::new();
    for arg in args {
        s.push_str(&arg.to_display_string());
    }
    Ok(Value::Text(s))
}

pub fn textjoin(args: &[Value]) -> R<Value> {
    let delim = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let ignore_empty = opt_num(args, 1, 1.0)? != 0.0;
    let mut parts = Vec::new();

    for arg in &args[2..] {
        match arg {
            Value::Array(arr) => {
                for v in &arr.data {
                    let text = v.to_display_string();
                    if !ignore_empty || !text.is_empty() {
                        parts.push(text);
                    }
                }
            }
            v => {
                let text = v.to_display_string();
                if !ignore_empty || !text.is_empty() {
                    parts.push(text);
                }
            }
        }
    }
    Ok(Value::Text(parts.join(&delim)))
}

pub fn left(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let count = opt_num(args, 1, 1.0)?.trunc() as usize;
    let chars: String = text.chars().take(count).collect();
    Ok(Value::Text(chars))
}

pub fn right(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let count = opt_num(args, 1, 1.0)?.trunc() as usize;
    let total = text.chars().count();
    let skip = total.saturating_sub(count);
    let chars: String = text.chars().skip(skip).collect();
    Ok(Value::Text(chars))
}

pub fn mid(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let start = num(args, 1)?.trunc() as usize;
    let count = num(args, 2)?.trunc() as usize;
    if start == 0 {
        return Err(CellError::Value);
    }
    let chars: String = text.chars().skip(start - 1).take(count).collect();
    Ok(Value::Text(chars))
}

pub fn len(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    Ok(Value::Number(text.chars().count() as f64))
}

pub fn trim(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let words: Vec<&str> = text.split_whitespace().collect();
    Ok(Value::Text(words.join(" ")))
}

pub fn upper(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    Ok(Value::Text(text.to_uppercase()))
}

pub fn lower(args: &[Value]) -> R<Value> {
    let text = args.get(0).ok_or(CellError::Value)?.to_display_string();
    Ok(Value::Text(text.to_lowercase()))
}

pub fn exact(args: &[Value]) -> R<Value> {
    let s1 = args.get(0).ok_or(CellError::Value)?.to_display_string();
    let s2 = args.get(1).ok_or(CellError::Value)?.to_display_string();
    Ok(Value::Bool(s1 == s2))
}

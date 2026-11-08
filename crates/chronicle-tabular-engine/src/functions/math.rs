//! Standard mathematical functions: SUM, PRODUCT, ROUND, ABS, etc.

use crate::core::value::{CellError, Value};
use crate::functions::util::{R, flatten_numbers, num, opt_num};

pub fn sum(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    let s: f64 = nums.iter().sum();
    Ok(Value::Number(s))
}

pub fn product(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    if nums.is_empty() {
        return Ok(Value::Number(0.0));
    }
    let p: f64 = nums.iter().product();
    Ok(Value::Number(p))
}

pub fn abs(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    Ok(Value::Number(n.abs()))
}

pub fn round(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    let digits = opt_num(args, 1, 0.0)?.trunc() as i32;
    let factor = 10.0_f64.powi(digits);
    Ok(Value::Number((n * factor).round() / factor))
}

pub fn floor(args: &[Value]) -> R<Value> {
    let (n, sig) = (num(args, 0)?, num(args, 1)?);
    if sig == 0.0 {
        return Err(CellError::Div0);
    }
    Ok(Value::Number((n / sig).floor() * sig))
}

pub fn ceiling(args: &[Value]) -> R<Value> {
    let (n, sig) = (num(args, 0)?, num(args, 1)?);
    if sig == 0.0 {
        return Err(CellError::Div0);
    }
    Ok(Value::Number((n / sig).ceil() * sig))
}

pub fn mod_fn(args: &[Value]) -> R<Value> {
    let (n, d) = (num(args, 0)?, num(args, 1)?);
    if d == 0.0 {
        return Err(CellError::Div0);
    }
    Ok(Value::Number(n.rem_euclid(d)))
}

pub fn sqrt(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    if n < 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(n.sqrt()))
}

pub fn power(args: &[Value]) -> R<Value> {
    let (base, exp) = (num(args, 0)?, num(args, 1)?);
    Ok(Value::Number(base.powf(exp)))
}

pub fn ln(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    if n <= 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(n.ln()))
}

pub fn log10(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    if n <= 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(n.log10()))
}

pub fn exp_fn(args: &[Value]) -> R<Value> {
    let n = num(args, 0)?;
    Ok(Value::Number(n.exp()))
}

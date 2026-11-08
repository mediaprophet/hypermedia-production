//! Statistical distribution functions and aggregations.

use std::f64::consts::PI;
use crate::core::value::{CellError, Value};
use crate::functions::util::{R, flatten_numbers, num, opt_num};

// ---------------------------------------------------------------------------------------------
// Normal Distribution
// ---------------------------------------------------------------------------------------------

/// Standard normal PDF: phi(x)
pub fn std_norm_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * PI).sqrt()
}

/// Error function erf(x) via Chebyshev approximation
pub fn erf(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t * (-x * x - 1.26551223
        + t * (1.00002368
            + t * (0.37409196
                + t * (0.09678418
                    + t * (-0.18628806
                        + t * (0.27886807
                            + t * (-1.13520398
                                + t * (1.48851587
                                    + t * (-0.82215223
                                        + t * 0.17087277))))))))).exp();
    if x >= 0.0 { 1.0 - tau } else { tau - 1.0 }
}

/// Standard normal CDF: Phi(x)
pub fn std_norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / std::f64::consts::SQRT_2))
}

/// Inverse standard normal CDF (Acklam approximation)
pub fn std_norm_inv(p: f64) -> Option<f64> {
    if !(0.0..=1.0).contains(&p) || p == 0.0 || p == 1.0 {
        return None;
    }
    const A: [f64; 6] = [
        -3.969683028665376e+01,
        2.209460984245205e+02,
        -2.759285104469687e+02,
        1.383577518672690e+02,
        -3.066479806614716e+01,
        2.506628277459239e+00,
    ];
    const B: [f64; 5] = [
        -5.447609879822406e+01,
        1.615858368580409e+02,
        -1.556989798598866e+02,
        6.680131188771972e+01,
        -1.328068155288572e+01,
    ];
    const C: [f64; 6] = [
        -7.784894002430293e-03,
        -3.223964580411365e-01,
        -2.400758277161838e+00,
        -2.549732539343734e+00,
        4.374664141464968e+00,
        2.938163982698783e+00,
    ];
    const D: [f64; 4] = [
        7.784695709041462e-03,
        3.224671290700398e-01,
        2.445134137142996e+00,
        3.754408661907416e+00,
    ];

    let p_low = 0.02425;
    let p_high = 1.0 - p_low;

    if p < p_low {
        let q = (-2.0 * p.ln()).sqrt();
        Some(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0))
    } else if p <= p_high {
        let q = p - 0.5;
        let r = q * q;
        Some((((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q
            / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0))
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        Some(-(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]
            / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)))
    }
}

pub fn norm_s_dist(args: &[Value]) -> R<Value> {
    let z = num(args, 0)?;
    let cumulative = opt_num(args, 1, 1.0)? != 0.0;
    if cumulative {
        Ok(Value::Number(std_norm_cdf(z)))
    } else {
        Ok(Value::Number(std_norm_pdf(z)))
    }
}

pub fn norm_dist(args: &[Value]) -> R<Value> {
    let (x, mean, stdev) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let cumulative = opt_num(args, 3, 1.0)? != 0.0;
    if stdev <= 0.0 {
        return Err(CellError::Num);
    }
    let z = (x - mean) / stdev;
    if cumulative {
        Ok(Value::Number(std_norm_cdf(z)))
    } else {
        Ok(Value::Number(std_norm_pdf(z) / stdev))
    }
}

pub fn norm_inv(args: &[Value]) -> R<Value> {
    let (p, mean, stdev) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    if stdev <= 0.0 || p <= 0.0 || p >= 1.0 {
        return Err(CellError::Num);
    }
    let z = std_norm_inv(p).ok_or(CellError::Num)?;
    Ok(Value::Number(mean + z * stdev))
}

// ---------------------------------------------------------------------------------------------
// Poisson Distribution
// ---------------------------------------------------------------------------------------------

pub fn poisson_dist(args: &[Value]) -> R<Value> {
    let (x, mean) = (num(args, 0)?, num(args, 1)?);
    let cumulative = opt_num(args, 2, 1.0)? != 0.0;
    if x < 0.0 || mean <= 0.0 {
        return Err(CellError::Num);
    }
    let k = x.floor() as u64;
    let mut prob = (-mean).exp();
    if cumulative {
        let mut sum = prob;
        let mut term = prob;
        for i in 1..=k {
            term *= mean / i as f64;
            sum += term;
        }
        Ok(Value::Number(sum))
    } else {
        for i in 1..=k {
            prob *= mean / i as f64;
        }
        Ok(Value::Number(prob))
    }
}

// ---------------------------------------------------------------------------------------------
// Gamma and Other Distributions
// ---------------------------------------------------------------------------------------------

pub fn gamma_ln(xx: f64) -> f64 {
    let mut x = xx;
    let mut y = xx;
    let mut tmp = x + 5.5;
    tmp -= (x + 0.5) * tmp.ln();
    let mut ser = 1.000000000190015;
    const COEFF: [f64; 6] = [
        76.18009172947146,
        -86.50532032941677,
        24.01409824083091,
        -1.231739572450155,
        0.1208650973866179e-2,
        -0.5395239384953e-5,
    ];
    for c in &COEFF {
        y += 1.0;
        ser += c / y;
    }
    -tmp + (2.5066282746310005 * ser / x).ln()
}

pub fn gamma_dist(args: &[Value]) -> R<Value> {
    let (x, alpha, beta) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let cumulative = opt_num(args, 3, 1.0)? != 0.0;
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
        return Err(CellError::Num);
    }
    if !cumulative {
        let pdf = (alpha - 1.0) * x.ln() - (x / beta) - alpha * beta.ln() - gamma_ln(alpha);
        return Ok(Value::Number(pdf.exp()));
    }
    // Simple series integration for cumulative
    let mut sum = 0.0;
    let steps = 1000;
    let dx = x / steps as f64;
    for i in 0..steps {
        let mid = (i as f64 + 0.5) * dx;
        let pdf = ((alpha - 1.0) * mid.ln() - (mid / beta) - alpha * beta.ln() - gamma_ln(alpha)).exp();
        sum += pdf * dx;
    }
    Ok(Value::Number(sum.min(1.0)))
}

pub fn weibull_dist(args: &[Value]) -> R<Value> {
    let (x, alpha, beta) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let cumulative = opt_num(args, 3, 1.0)? != 0.0;
    if x < 0.0 || alpha <= 0.0 || beta <= 0.0 {
        return Err(CellError::Num);
    }
    let z = x / beta;
    if cumulative {
        Ok(Value::Number(1.0 - (-z.powf(alpha)).exp()))
    } else {
        Ok(Value::Number((alpha / beta) * z.powf(alpha - 1.0) * (-z.powf(alpha)).exp()))
    }
}

pub fn chisq_dist(args: &[Value]) -> R<Value> {
    let (x, deg_f) = (num(args, 0)?, num(args, 1)?);
    let cumulative = opt_num(args, 2, 1.0)? != 0.0;
    if x < 0.0 || deg_f < 1.0 {
        return Err(CellError::Num);
    }
    gamma_dist(&[Value::Number(x), Value::Number(deg_f / 2.0), Value::Number(2.0), Value::Number(if cumulative { 1.0 } else { 0.0 })])
}

pub fn t_dist(args: &[Value]) -> R<Value> {
    let (x, deg_f) = (num(args, 0)?, num(args, 1)?);
    let cumulative = opt_num(args, 2, 1.0)? != 0.0;
    if deg_f < 1.0 {
        return Err(CellError::Num);
    }
    let pdf = (gamma_ln((deg_f + 1.0) / 2.0) - gamma_ln(deg_f / 2.0)).exp()
        / ((deg_f * PI).sqrt() * (1.0 + x * x / deg_f).powf((deg_f + 1.0) / 2.0));
    if !cumulative {
        return Ok(Value::Number(pdf));
    }
    // Trapezoidal numerical integration
    let z_bound = 10.0f64;
    let lo = -z_bound;
    let hi = x.clamp(-z_bound, z_bound);
    let steps = 1000;
    let dx = (hi - lo) / steps as f64;
    let mut sum = 0.0;
    for i in 0..steps {
        let t = lo + (i as f64 + 0.5) * dx;
        let p = (gamma_ln((deg_f + 1.0) / 2.0) - gamma_ln(deg_f / 2.0)).exp()
            / ((deg_f * PI).sqrt() * (1.0 + t * t / deg_f).powf((deg_f + 1.0) / 2.0));
        sum += p * dx;
    }
    Ok(Value::Number(sum.clamp(0.0, 1.0)))
}

// ---------------------------------------------------------------------------------------------
// Aggregations: AVERAGE, COUNT, MAX, MIN, STDEV, VAR
// ---------------------------------------------------------------------------------------------

pub fn average(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    if nums.is_empty() {
        return Err(CellError::Div0);
    }
    let sum: f64 = nums.iter().sum();
    Ok(Value::Number(sum / nums.len() as f64))
}

pub fn count(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    Ok(Value::Number(nums.len() as f64))
}

pub fn max(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    if nums.is_empty() {
        return Ok(Value::Number(0.0));
    }
    let m = nums.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    Ok(Value::Number(m))
}

pub fn min(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    if nums.is_empty() {
        return Ok(Value::Number(0.0));
    }
    let m = nums.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    Ok(Value::Number(m))
}

pub fn var_s(args: &[Value]) -> R<Value> {
    let nums = flatten_numbers(args)?;
    if nums.len() < 2 {
        return Err(CellError::Div0);
    }
    let avg: f64 = nums.iter().sum::<f64>() / nums.len() as f64;
    let sum_sq: f64 = nums.iter().map(|&x| (x - avg) * (x - avg)).sum();
    Ok(Value::Number(sum_sq / (nums.len() - 1) as f64))
}

pub fn stdev_s(args: &[Value]) -> R<Value> {
    let v = var_s(args)?;
    if let Value::Number(n) = v {
        Ok(Value::Number(n.sqrt()))
    } else {
        Err(CellError::Value)
    }
}

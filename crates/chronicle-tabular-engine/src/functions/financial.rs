//! Financial functions: time value of money, cash-flow analysis, depreciation and securities.
//!
//! Sign convention follows standard financial calculations: money paid out is negative,
//! money received is positive.

use crate::core::value::{CellError, Value};
use crate::functions::util::{R, as_array, num, opt_num, to_f64};

// ---------------------------------------------------------------------------------------------
// Time Value of Money
// ---------------------------------------------------------------------------------------------

pub fn pmt_calc(rate: f64, nper: f64, pv: f64, fv: f64, typ: f64) -> f64 {
    if rate == 0.0 {
        return -(pv + fv) / nper;
    }
    let p = (1.0 + rate).powf(nper);
    -(rate * (fv + pv * p)) / ((1.0 + rate * typ) * (p - 1.0))
}

pub fn fv_calc(rate: f64, nper: f64, pmt: f64, pv: f64, typ: f64) -> f64 {
    if rate == 0.0 {
        return -(pv + pmt * nper);
    }
    let p = (1.0 + rate).powf(nper);
    -(pv * p + pmt * (1.0 + rate * typ) * (p - 1.0) / rate)
}

pub fn pv_calc(rate: f64, nper: f64, pmt: f64, fv: f64, typ: f64) -> f64 {
    if rate == 0.0 {
        return -(fv + pmt * nper);
    }
    let p = (1.0 + rate).powf(nper);
    -(fv + pmt * (1.0 + rate * typ) * (p - 1.0) / rate) / p
}

pub fn ipmt_calc(rate: f64, per: f64, nper: f64, pv: f64, fv: f64, typ: f64) -> f64 {
    let p = pmt_calc(rate, nper, pv, fv, typ);
    if typ == 1.0 && per == 1.0 {
        return 0.0;
    }
    let bal = fv_calc(rate, per - 1.0, p, pv, typ);
    let i = bal * rate;
    if typ == 1.0 { i / (1.0 + rate) } else { i }
}

pub fn pmt(args: &[Value]) -> R<Value> {
    let (rate, nper, pv) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let fv = opt_num(args, 3, 0.0)?;
    let typ = opt_num(args, 4, 0.0)?;
    if nper == 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(pmt_calc(rate, nper, pv, fv, typ)))
}

pub fn fv(args: &[Value]) -> R<Value> {
    let (rate, nper, pmt_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let pv_val = opt_num(args, 3, 0.0)?;
    let typ = opt_num(args, 4, 0.0)?;
    Ok(Value::Number(fv_calc(rate, nper, pmt_val, pv_val, typ)))
}

pub fn pv(args: &[Value]) -> R<Value> {
    let (rate, nper, pmt_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let fv_val = opt_num(args, 3, 0.0)?;
    let typ = opt_num(args, 4, 0.0)?;
    Ok(Value::Number(pv_calc(rate, nper, pmt_val, fv_val, typ)))
}

pub fn nper(args: &[Value]) -> R<Value> {
    let (rate, pmt_val, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let fv_val = opt_num(args, 3, 0.0)?;
    let typ = opt_num(args, 4, 0.0)?;
    if rate == 0.0 {
        if pmt_val == 0.0 {
            return Err(CellError::Num);
        }
        return Ok(Value::Number(-(pv_val + fv_val) / pmt_val));
    }
    let numer = pmt_val * (1.0 + rate * typ) - fv_val * rate;
    let denom = pv_val * rate + pmt_val * (1.0 + rate * typ);
    if denom == 0.0 {
        return Err(CellError::Num);
    }
    let q = numer / denom;
    if q <= 0.0 || rate <= -1.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(q.ln() / (1.0 + rate).ln()))
}

pub fn ipmt(args: &[Value]) -> R<Value> {
    let (rate, per, np, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let fv_val = opt_num(args, 4, 0.0)?;
    let typ = opt_num(args, 5, 0.0)?;
    if per < 1.0 || per > np || np == 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(ipmt_calc(rate, per, np, pv_val, fv_val, typ)))
}

pub fn ppmt(args: &[Value]) -> R<Value> {
    let (rate, per, np, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let fv_val = opt_num(args, 4, 0.0)?;
    let typ = opt_num(args, 5, 0.0)?;
    if per < 1.0 || per > np || np == 0.0 {
        return Err(CellError::Num);
    }
    let total_pmt = pmt_calc(rate, np, pv_val, fv_val, typ);
    let interest = ipmt_calc(rate, per, np, pv_val, fv_val, typ);
    Ok(Value::Number(total_pmt - interest))
}

pub fn cumipmt(args: &[Value]) -> R<Value> {
    let (rate, np, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let (start, end, typ) = (num(args, 3)?.trunc(), num(args, 4)?.trunc(), num(args, 5)?);
    if rate <= 0.0 || np <= 0.0 || pv_val <= 0.0 || start < 1.0 || end < start || end > np || (typ != 0.0 && typ != 1.0) {
        return Err(CellError::Num);
    }
    let mut sum = 0.0;
    let mut per = start;
    while per <= end {
        sum += ipmt_calc(rate, per, np, pv_val, 0.0, typ);
        per += 1.0;
    }
    Ok(Value::Number(sum))
}

pub fn cumprinc(args: &[Value]) -> R<Value> {
    let (rate, np, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let (start, end, typ) = (num(args, 3)?.trunc(), num(args, 4)?.trunc(), num(args, 5)?);
    if rate <= 0.0 || np <= 0.0 || pv_val <= 0.0 || start < 1.0 || end < start || end > np || (typ != 0.0 && typ != 1.0) {
        return Err(CellError::Num);
    }
    let p = pmt_calc(rate, np, pv_val, 0.0, typ);
    let mut sum = 0.0;
    let mut per = start;
    while per <= end {
        let i = ipmt_calc(rate, per, np, pv_val, 0.0, typ);
        sum += p - i;
        per += 1.0;
    }
    Ok(Value::Number(sum))
}

// ---------------------------------------------------------------------------------------------
// Rate / Cash-flow solvers
// ---------------------------------------------------------------------------------------------

fn solve_rate(f: impl Fn(f64) -> f64, guess: f64) -> R<f64> {
    let mut x = guess;
    for _ in 0..200 {
        let y = f(x);
        if !y.is_finite() {
            break;
        }
        let h = 1e-6 * x.abs().max(1e-3);
        let d = (f(x + h) - f(x - h)) / (2.0 * h);
        if d == 0.0 || !d.is_finite() {
            break;
        }
        let mut nx = x - y / d;
        if nx <= -1.0 {
            nx = (x - 1.0) / 2.0;
            if nx <= -1.0 {
                nx = -0.999_999;
            }
        }
        if (nx - x).abs() < 1e-12 * nx.abs().max(1.0) {
            return Ok(nx);
        }
        x = nx;
    }

    // Bisection fallback
    let grid = [-0.99, -0.5, -0.1, 0.0, 0.1, 0.5, 1.0, 5.0, 10.0];
    for w in grid.windows(2) {
        let (mut lo, mut hi) = (w[0], w[1]);
        let (flo, fhi) = (f(lo), f(hi));
        if !(flo.is_finite() && fhi.is_finite()) || flo * fhi > 0.0 {
            continue;
        }
        let lo_neg = flo < 0.0;
        for _ in 0..100 {
            let mid = 0.5 * (lo + hi);
            let fm = f(mid);
            if (fm < 0.0) == lo_neg {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        return Ok(0.5 * (lo + hi));
    }
    Err(CellError::Num)
}

pub fn rate(args: &[Value]) -> R<Value> {
    let (np, pmt_val, pv_val) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let fv_val = opt_num(args, 3, 0.0)?;
    let typ = opt_num(args, 4, 0.0)?;
    let guess = opt_num(args, 5, 0.1)?;
    if np <= 0.0 {
        return Err(CellError::Num);
    }
    let f = |r: f64| {
        if r.abs() < 1e-12 {
            pv_val + pmt_val * np + fv_val
        } else {
            let p = (1.0 + r).powf(np);
            pv_val * p + pmt_val * (1.0 + r * typ) * (p - 1.0) / r + fv_val
        }
    };
    Ok(Value::Number(solve_rate(f, guess)?))
}

pub fn npv(args: &[Value]) -> R<Value> {
    let rate_val = num(args, 0)?;
    if rate_val == -1.0 {
        return Err(CellError::Div0);
    }
    let mut sum = 0.0;
    let mut d = 1.0;
    for arg in &args[1..] {
        match arg {
            Value::Number(n) => {
                d *= 1.0 + rate_val;
                sum += n / d;
            }
            Value::Array(arr) => {
                for v in &arr.data {
                    if let Value::Number(n) = v {
                        d *= 1.0 + rate_val;
                        sum += n / d;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(Value::Number(sum))
}

fn dated_flows(val_arg: &Value, date_arg: &Value) -> R<(Vec<f64>, Vec<f64>)> {
    let va = as_array(val_arg);
    let da = as_array(date_arg);
    if va.data.len() != da.data.len() || va.data.is_empty() {
        return Err(CellError::Num);
    }
    let mut v = Vec::with_capacity(va.data.len());
    let mut d = Vec::with_capacity(da.data.len());
    for (x, y) in va.data.iter().zip(da.data.iter()) {
        let vx = to_f64(x)?;
        let dy = to_f64(y)?.trunc();
        v.push(vx);
        d.push(dy);
    }
    let d0 = *d.first().ok_or(CellError::Num)?;
    if d.iter().any(|&x| x < d0) {
        return Err(CellError::Num);
    }
    Ok((v, d))
}

pub fn xnpv_calc(rate_val: f64, v: &[f64], d: &[f64]) -> f64 {
    let d0 = d.first().copied().unwrap_or(0.0);
    v.iter().zip(d).map(|(x, t)| x / (1.0 + rate_val).powf((t - d0) / 365.0)).sum()
}

pub fn xnpv(args: &[Value]) -> R<Value> {
    let rate_val = num(args, 0)?;
    if rate_val <= -1.0 {
        return Err(CellError::Num);
    }
    let (v, d) = dated_flows(args.get(1).ok_or(CellError::Value)?, args.get(2).ok_or(CellError::Value)?)?;
    Ok(Value::Number(xnpv_calc(rate_val, &v, &d)))
}

pub fn xirr(args: &[Value]) -> R<Value> {
    let (v, d) = dated_flows(args.get(0).ok_or(CellError::Value)?, args.get(1).ok_or(CellError::Value)?)?;
    let guess = opt_num(args, 2, 0.1)?;
    if !v.iter().any(|x| *x > 0.0) || !v.iter().any(|x| *x < 0.0) {
        return Err(CellError::Num);
    }
    let r = solve_rate(|r| xnpv_calc(r, &v, &d), guess)?;
    Ok(Value::Number(r))
}

pub fn irr(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let guess = opt_num(args, 1, 0.1)?;
    let mut v = Vec::new();
    for item in &arr.data {
        if let Ok(n) = to_f64(item) {
            v.push(n);
        }
    }
    if !v.iter().any(|x| *x > 0.0) || !v.iter().any(|x| *x < 0.0) {
        return Err(CellError::Num);
    }
    let f = |r: f64| {
        let mut s = 0.0;
        let mut d = 1.0;
        for x in &v {
            s += x / d;
            d *= 1.0 + r;
        }
        s
    };
    Ok(Value::Number(solve_rate(f, guess)?))
}

pub fn mirr(args: &[Value]) -> R<Value> {
    let arr = as_array(args.get(0).ok_or(CellError::Value)?);
    let finance_rate = num(args, 1)?;
    let reinvest_rate = num(args, 2)?;
    let mut v = Vec::new();
    for item in &arr.data {
        v.push(to_f64(item)?);
    }
    let n = v.len() as f64 - 1.0;
    if n <= 0.0 {
        return Err(CellError::Div0);
    }
    let mut npv_pos = 0.0;
    let mut npv_neg = 0.0;
    for (i, &val) in v.iter().enumerate() {
        if val >= 0.0 {
            npv_pos += val * (1.0 + reinvest_rate).powf(n - i as f64);
        } else {
            npv_neg += val / (1.0 + finance_rate).powf(i as f64);
        }
    }
    if npv_pos == 0.0 || npv_neg == 0.0 {
        return Err(CellError::Div0);
    }
    let res = (-npv_pos / npv_neg).powf(1.0 / n) - 1.0;
    Ok(Value::Number(res))
}

// ---------------------------------------------------------------------------------------------
// Effective/Nominal Rates & Conversions
// ---------------------------------------------------------------------------------------------

pub fn effect(args: &[Value]) -> R<Value> {
    let (nom, npery) = (num(args, 0)?, num(args, 1)?);
    let n = npery.trunc();
    if nom <= 0.0 || n < 1.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number((1.0 + nom / n).powf(n) - 1.0))
}

pub fn nominal(args: &[Value]) -> R<Value> {
    let (eff, npery) = (num(args, 0)?, num(args, 1)?);
    let n = npery.trunc();
    if eff <= 0.0 || n < 1.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(n * ((1.0 + eff).powf(1.0 / n) - 1.0)))
}

pub fn dollarde(args: &[Value]) -> R<Value> {
    let (fractional, fraction) = (num(args, 0)?, num(args, 1)?);
    let f = fraction.trunc();
    if f < 0.0 { return Err(CellError::Num); }
    if f == 0.0 { return Err(CellError::Div0); }
    let int_part = fractional.trunc();
    let frac_part = fractional - int_part;
    Ok(Value::Number(int_part + frac_part * 10.0_f64.powf((f.log10().ceil())) / f))
}

pub fn dollarfr(args: &[Value]) -> R<Value> {
    let (decimal, fraction) = (num(args, 0)?, num(args, 1)?);
    let f = fraction.trunc();
    if f < 0.0 { return Err(CellError::Num); }
    if f == 0.0 { return Err(CellError::Div0); }
    let int_part = decimal.trunc();
    let frac_part = decimal - int_part;
    Ok(Value::Number(int_part + frac_part * f / 10.0_f64.powf((f.log10().ceil()))))
}

// ---------------------------------------------------------------------------------------------
// Depreciation
// ---------------------------------------------------------------------------------------------

pub fn sln(args: &[Value]) -> R<Value> {
    let (cost, salvage, life) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    if life == 0.0 {
        return Err(CellError::Div0);
    }
    Ok(Value::Number((cost - salvage) / life))
}

pub fn syd(args: &[Value]) -> R<Value> {
    let (cost, salvage, life, per) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    if life <= 0.0 || per < 1.0 || per > life {
        return Err(CellError::Num);
    }
    Ok(Value::Number((cost - salvage) * (life - per + 1.0) * 2.0 / (life * (life + 1.0))))
}

pub fn db(args: &[Value]) -> R<Value> {
    let (cost, salvage, life, per) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let month = opt_num(args, 4, 12.0)?;
    if cost < 0.0 || salvage < 0.0 || life <= 0.0 || per < 1.0 || per > life + 1.0 || month < 1.0 || month > 12.0 {
        return Err(CellError::Num);
    }
    let rate_val = (1.0 - (salvage / cost).powf(1.0 / life) * 1000.0).round() / 1000.0;
    let mut total = 0.0;
    let mut dep = 0.0;
    for p in 1..=(per as usize) {
        dep = if p == 1 {
            cost * rate_val * month / 12.0
        } else if p == (life as usize) + 1 {
            (cost - total) * rate_val * (12.0 - month) / 12.0
        } else {
            (cost - total) * rate_val
        };
        total += dep;
    }
    Ok(Value::Number(dep))
}

pub fn ddb(args: &[Value]) -> R<Value> {
    let (cost, salvage, life, per) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let factor = opt_num(args, 4, 2.0)?;
    if cost < 0.0 || salvage < 0.0 || life <= 0.0 || per < 1.0 || per > life || factor <= 0.0 {
        return Err(CellError::Num);
    }
    let rate_val = factor / life;
    let mut total = 0.0;
    let mut dep = 0.0;
    for _ in 1..=(per as usize) {
        dep = ((cost - total) * rate_val).min(cost - salvage - total).max(0.0);
        total += dep;
    }
    Ok(Value::Number(dep))
}

pub fn vdb(args: &[Value]) -> R<Value> {
    let (cost, salvage, life, start_per, end_per) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?, num(args, 4)?);
    let factor = opt_num(args, 5, 2.0)?;
    if cost < 0.0 || salvage < 0.0 || life <= 0.0 || start_per < 0.0 || end_per < start_per || end_per > life || factor <= 0.0 {
        return Err(CellError::Num);
    }
    let mut total_dep = 0.0;
    for p in (start_per.floor() as usize + 1)..=(end_per.ceil() as usize) {
        let v = ddb(&[Value::Number(cost), Value::Number(salvage), Value::Number(life), Value::Number(p as f64), Value::Number(factor)])?;
        if let Value::Number(n) = v {
            total_dep += n;
        }
    }
    Ok(Value::Number(total_dep))
}

// ---------------------------------------------------------------------------------------------
// Securities & Bond Metrics
// ---------------------------------------------------------------------------------------------

pub fn duration(args: &[Value]) -> R<Value> {
    let (settlement, maturity, coupon, yld, frequency) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?, num(args, 4)?);
    let f = frequency.trunc();
    if coupon < 0.0 || yld < 0.0 || settlement >= maturity || (f != 1.0 && f != 2.0 && f != 4.0) {
        return Err(CellError::Num);
    }
    let n = ((maturity - settlement) / (365.0 / f)).ceil();
    let c = coupon / f;
    let y = yld / f;
    let mut mac_dur = 0.0;
    let mut pv_sum = 0.0;
    for t in 1..=(n as usize) {
        let cash = if t == (n as usize) { 100.0 * (1.0 + c) } else { 100.0 * c };
        let pv_cash = cash / (1.0 + y).powf(t as f64);
        mac_dur += (t as f64) * pv_cash;
        pv_sum += pv_cash;
    }
    if pv_sum == 0.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(mac_dur / (pv_sum * f)))
}

pub fn mduration(args: &[Value]) -> R<Value> {
    let mac = duration(args)?;
    let yld = num(args, 3)?;
    let frequency = num(args, 4)?.trunc();
    if let Value::Number(d) = mac {
        Ok(Value::Number(d / (1.0 + yld / frequency)))
    } else {
        Err(CellError::Num)
    }
}

pub fn price_bond(args: &[Value]) -> R<Value> {
    let (settlement, maturity, rate_val, yld, redemption, frequency) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?, num(args, 4)?, num(args, 5)?);
    let f = frequency.trunc();
    if rate_val < 0.0 || yld < 0.0 || redemption <= 0.0 || settlement >= maturity || (f != 1.0 && f != 2.0 && f != 4.0) {
        return Err(CellError::Num);
    }
    let n = ((maturity - settlement) / (365.0 / f)).ceil();
    let c = 100.0 * rate_val / f;
    let y = yld / f;
    let mut pv_coupons = 0.0;
    for t in 1..=(n as usize) {
        pv_coupons += c / (1.0 + y).powf(t as f64);
    }
    let pv_redemption = redemption / (1.0 + y).powf(n);
    Ok(Value::Number(pv_coupons + pv_redemption))
}

pub fn yield_maturity(args: &[Value]) -> R<Value> {
    let (settlement, maturity, rate_val, pr, redemption, frequency) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?, num(args, 4)?, num(args, 5)?);
    if pr <= 0.0 || redemption <= 0.0 || settlement >= maturity {
        return Err(CellError::Num);
    }
    let f = |y: f64| {
        let p_res = price_bond(&[
            Value::Number(settlement),
            Value::Number(maturity),
            Value::Number(rate_val),
            Value::Number(y),
            Value::Number(redemption),
            Value::Number(frequency),
        ]);
        match p_res {
            Ok(Value::Number(p)) => p - pr,
            _ => f64::NAN,
        }
    };
    Ok(Value::Number(solve_rate(f, 0.05)?))
}

pub fn disc(args: &[Value]) -> R<Value> {
    let (settlement, maturity, pr, redemption) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let basis = opt_num(args, 4, 0.0)?;
    if pr <= 0.0 || redemption <= 0.0 || settlement >= maturity {
        return Err(CellError::Num);
    }
    let b = if basis == 1.0 || basis == 3.0 { 365.0 } else { 360.0 };
    let d = maturity - settlement;
    Ok(Value::Number(((redemption - pr) / redemption) * (b / d)))
}

pub fn tbilleq(args: &[Value]) -> R<Value> {
    let (settlement, maturity, discount) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let d = maturity - settlement;
    if discount <= 0.0 || d <= 0.0 || d > 366.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number((365.0 * discount) / (360.0 - discount * d)))
}

pub fn tbillprice(args: &[Value]) -> R<Value> {
    let (settlement, maturity, discount) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let d = maturity - settlement;
    if discount <= 0.0 || d <= 0.0 || d > 366.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(100.0 * (1.0 - discount * d / 360.0)))
}

pub fn tbillyield(args: &[Value]) -> R<Value> {
    let (settlement, maturity, pr) = (num(args, 0)?, num(args, 1)?, num(args, 2)?);
    let d = maturity - settlement;
    if pr <= 0.0 || d <= 0.0 || d > 366.0 {
        return Err(CellError::Num);
    }
    Ok(Value::Number(((100.0 - pr) / pr) * (360.0 / d)))
}

pub fn received(args: &[Value]) -> R<Value> {
    let (settlement, maturity, investment, discount) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let basis = opt_num(args, 4, 0.0)?;
    let d = maturity - settlement;
    if investment <= 0.0 || discount <= 0.0 || d <= 0.0 {
        return Err(CellError::Num);
    }
    let b = if basis == 1.0 || basis == 3.0 { 365.0 } else { 360.0 };
    Ok(Value::Number(investment / (1.0 - discount * d / b)))
}

pub fn intrate(args: &[Value]) -> R<Value> {
    let (settlement, maturity, investment, redemption) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let basis = opt_num(args, 4, 0.0)?;
    let d = maturity - settlement;
    if investment <= 0.0 || redemption <= 0.0 || d <= 0.0 {
        return Err(CellError::Num);
    }
    let b = if basis == 1.0 || basis == 3.0 { 365.0 } else { 360.0 };
    Ok(Value::Number(((redemption - investment) / investment) * (b / d)))
}

pub fn pricedisc(args: &[Value]) -> R<Value> {
    let (settlement, maturity, discount, redemption) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let basis = opt_num(args, 4, 0.0)?;
    let d = maturity - settlement;
    if discount <= 0.0 || redemption <= 0.0 || d <= 0.0 {
        return Err(CellError::Num);
    }
    let b = if basis == 1.0 || basis == 3.0 { 365.0 } else { 360.0 };
    Ok(Value::Number(redemption - discount * redemption * d / b))
}

pub fn yielddisc(args: &[Value]) -> R<Value> {
    let (settlement, maturity, pr, redemption) = (num(args, 0)?, num(args, 1)?, num(args, 2)?, num(args, 3)?);
    let basis = opt_num(args, 4, 0.0)?;
    let d = maturity - settlement;
    if pr <= 0.0 || redemption <= 0.0 || d <= 0.0 {
        return Err(CellError::Num);
    }
    let b = if basis == 1.0 || basis == 3.0 { 365.0 } else { 360.0 };
    Ok(Value::Number(((redemption - pr) / pr) * (b / d)))
}

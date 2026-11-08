//! Parsing what users type into a cell and numeric text in formulas.

use crate::core::date::{DateSystem, MONTHS, serial_from_ymd, time_fraction};
use crate::core::value::{CellError, Value};

#[derive(Clone, Debug, PartialEq)]
pub struct Parsed {
    pub value: Value,
    pub format: Option<&'static str>,
}

pub fn parse_input(s: &str, sys: DateSystem) -> Parsed {
    if s.is_empty() {
        return Parsed {
            value: Value::Empty,
            format: None,
        };
    }
    if let Some(rest) = s.strip_prefix('\'') {
        return Parsed {
            value: Value::text(rest),
            format: None,
        };
    }
    let t = s.trim();
    if t.eq_ignore_ascii_case("TRUE") {
        return Parsed {
            value: Value::Bool(true),
            format: None,
        };
    }
    if t.eq_ignore_ascii_case("FALSE") {
        return Parsed {
            value: Value::Bool(false),
            format: None,
        };
    }
    if let Some(e) = CellError::parse(t) {
        return Parsed {
            value: Value::Error(e),
            format: None,
        };
    }
    if let Some((n, f)) = parse_numeric(t, sys) {
        return Parsed {
            value: Value::number(n),
            format: f,
        };
    }
    Parsed {
        value: Value::text(s),
        format: None,
    }
}

pub fn parse_number_text(s: &str) -> Option<f64> {
    parse_numeric(s.trim(), DateSystem::D1900).map(|(n, _)| n)
}

fn parse_numeric(t: &str, sys: DateSystem) -> Option<(f64, Option<&'static str>)> {
    if t.is_empty() {
        return None;
    }
    if let Some(inner) = t.strip_prefix('(').and_then(|x| x.strip_suffix(')')) {
        return parse_numeric(inner.trim(), sys).map(|(n, f)| (-n, f));
    }
    if let Some(r) = parse_plain(t) {
        return Some(r);
    }
    if let Some(r) = parse_fraction(t) {
        return Some((r, Some("# ?/?")));
    }
    if let Some(r) = parse_datetime(t, sys) {
        return Some(r);
    }
    None
}

fn parse_plain(t: &str) -> Option<(f64, Option<&'static str>)> {
    let mut s = t;
    let mut neg = false;
    let mut currency = false;
    let mut percent = false;
    if let Some(r) = s.strip_prefix('-') {
        neg = true;
        s = r.trim_start();
    } else if let Some(r) = s.strip_prefix('+') {
        s = r.trim_start();
    }
    if let Some(r) = s.strip_prefix('$') {
        currency = true;
        s = r.trim_start();
        if !neg && let Some(r2) = s.strip_prefix('-') {
            neg = true;
            s = r2;
        }
    }
    if let Some(r) = s.strip_suffix('%') {
        percent = true;
        s = r.trim_end();
    }
    if s.is_empty() {
        return None;
    }
    let cleaned: String = s.chars().filter(|c| *c != ',').collect();
    let val: f64 = cleaned.parse().ok()?;
    let mut n = if neg { -val } else { val };
    if percent {
        n /= 100.0;
    }
    let fmt = if currency {
        Some("$#,##0.00")
    } else if percent {
        Some("0.00%")
    } else {
        None
    };
    Some((n, fmt))
}

fn parse_fraction(t: &str) -> Option<f64> {
    let parts: Vec<&str> = t.split_whitespace().collect();
    match parts.as_slice() {
        [frac] => {
            let (num, den) = frac.split_once('/')?;
            let n: f64 = num.parse().ok()?;
            let d: f64 = den.parse().ok()?;
            if d == 0.0 { None } else { Some(n / d) }
        }
        [whole, frac] => {
            let w: f64 = whole.parse().ok()?;
            let (num, den) = frac.split_once('/')?;
            let n: f64 = num.parse().ok()?;
            let d: f64 = den.parse().ok()?;
            if d == 0.0 {
                None
            } else {
                let sign = if w < 0.0 { -1.0 } else { 1.0 };
                Some(w + sign * (n / d))
            }
        }
        _ => None,
    }
}

fn parse_datetime(t: &str, sys: DateSystem) -> Option<(f64, Option<&'static str>)> {
    if let Some((y, m, d)) = parse_iso_date(t) {
        let serial = serial_from_ymd(sys, y, m, d)?;
        return Some((serial, Some("yyyy-mm-dd")));
    }
    if let Some((m, d, y)) = parse_us_date(t) {
        let serial = serial_from_ymd(sys, y, m, d)?;
        return Some((serial, Some("m/d/yyyy")));
    }
    if let Some(frac) = parse_time_only(t) {
        return Some((frac, Some("h:mm:ss")));
    }
    None
}

fn parse_iso_date(t: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = t.split('-').collect();
    if parts.len() == 3 {
        let y: i64 = parts[0].parse().ok()?;
        let m: i64 = parts[1].parse().ok()?;
        let d: i64 = parts[2].parse().ok()?;
        if (1..=12).contains(&m) && (1..=31).contains(&d) {
            return Some((y, m, d));
        }
    }
    None
}

fn parse_us_date(t: &str) -> Option<(i64, i64, i64)> {
    let parts: Vec<&str> = t.split('/').collect();
    if parts.len() == 3 {
        let m: i64 = parts[0].parse().ok()?;
        let d: i64 = parts[1].parse().ok()?;
        let mut y: i64 = parts[2].parse().ok()?;
        if y < 100 {
            y = if y < 30 { 2000 + y } else { 1900 + y };
        }
        if (1..=12).contains(&m) && (1..=31).contains(&d) {
            return Some((m, d, y));
        }
    }
    None
}

fn parse_time_only(t: &str) -> Option<f64> {
    let (time_str, pm) = if let Some(r) = t.strip_suffix("PM").or_else(|| t.strip_suffix("pm")) {
        (r.trim(), Some(true))
    } else if let Some(r) = t.strip_suffix("AM").or_else(|| t.strip_suffix("am")) {
        (r.trim(), Some(false))
    } else {
        (t, None)
    };
    let parts: Vec<&str> = time_str.split(':').collect();
    let mut h: f64 = parts.first()?.parse().ok()?;
    let m: f64 = parts.get(1)?.parse().ok()?;
    let s: f64 = parts.get(2).map(|x| x.parse().ok()).flatten().unwrap_or(0.0);
    if let Some(is_pm) = pm {
        if is_pm && h < 12.0 {
            h += 12.0;
        } else if !is_pm && h == 12.0 {
            h = 0.0;
        }
    }
    Some(time_fraction(h, m, s))
}

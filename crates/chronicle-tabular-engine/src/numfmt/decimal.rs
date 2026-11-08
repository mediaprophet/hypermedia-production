//! Decimal-string rounding and the General format.

pub(crate) const MAX_DECIMALS: usize = 400;

pub(crate) fn decimal_digits(a: f64) -> ([u8; 15], i32) {
    let mut digits = [0u8; 15];
    if !a.is_finite() || a <= 0.0 {
        return (digits, 0);
    }
    let s = format!("{:.14e}", a);
    let (mant, exp) = s.split_once('e').unwrap_or((s.as_str(), "0"));
    for (slot, b) in digits.iter_mut().zip(mant.bytes().filter(u8::is_ascii_digit)) {
        *slot = b - b'0';
    }
    (digits, exp.parse().unwrap_or(0))
}

pub(crate) fn fixed_from_digits(digits: &[u8; 15], exp: i32, decimals: usize) -> (String, String) {
    let decimals_calc = decimals.min(MAX_DECIMALS);
    let keep = exp as i64 + decimals_calc as i64 + 1;
    let mut m: Vec<u8> = if keep <= 0 {
        if keep == 0 && digits.first().copied().unwrap_or(0) >= 5 { vec![1] } else { Vec::new() }
    } else if keep as usize >= digits.len() {
        let mut v = digits.to_vec();
        v.resize(keep as usize, 0);
        v
    } else {
        let keep = keep as usize;
        let mut v = digits.get(..keep).map(<[u8]>::to_vec).unwrap_or_default();
        if digits.get(keep).copied().unwrap_or(0) >= 5 {
            let mut carry = true;
            for d in v.iter_mut().rev() {
                if *d == 9 {
                    *d = 0;
                } else {
                    *d += 1;
                    carry = false;
                    break;
                }
            }
            if carry {
                v.insert(0, 1);
            }
        }
        v
    };
    let lead = m.iter().take_while(|&&d| d == 0).count();
    m.drain(..lead);
    let to_s = |ds: &[u8]| ds.iter().map(|d| char::from(b'0' + d)).collect::<String>();
    let (int, mut frac) = if m.len() <= decimals_calc {
        let mut f = "0".repeat(decimals_calc - m.len());
        f.push_str(&to_s(&m));
        (String::new(), f)
    } else {
        let split = m.len() - decimals_calc;
        (to_s(m.get(..split).unwrap_or(&[])), to_s(m.get(split..).unwrap_or(&[])))
    };
    if decimals > decimals_calc {
        frac.push_str(&"0".repeat(decimals - decimals_calc));
    }
    (int, frac)
}

pub(crate) fn fixed(a: f64, decimals: usize) -> (String, String) {
    if !a.is_finite() || a <= 0.0 {
        return (String::new(), "0".repeat(decimals.min(MAX_DECIMALS * 4)));
    }
    let (d, e) = decimal_digits(a);
    fixed_from_digits(&d, e, decimals)
}

pub fn format_general_fit(n: f64, max_chars: usize) -> Option<String> {
    if !n.is_finite() {
        return None;
    }
    if n == 0.0 {
        return (max_chars >= 1).then(|| "0".to_string());
    }
    let sign = if n < 0.0 { "-" } else { "" };
    let (digits, x) = decimal_digits(n.abs());
    let int_len = sign.len() + (x.max(0) as usize) + 1;
    if x >= -4 && int_len <= max_chars {
        let avail = max_chars - int_len;
        let mut decimals = if avail >= 2 { avail - 1 } else { 0 };
        loop {
            let (i, f) = fixed_from_digits(&digits, x, decimals);
            let int_part = if i.is_empty() { "0" } else { &i };
            let s = if decimals > 0 {
                let trimmed = f.trim_end_matches('0');
                if trimmed.is_empty() {
                    format!("{sign}{int_part}")
                } else {
                    format!("{sign}{int_part}.{trimmed}")
                }
            } else {
                format!("{sign}{int_part}")
            };
            if s.len() <= max_chars {
                return Some(s);
            }
            if decimals == 0 {
                break;
            }
            decimals -= 1;
        }
    }
    // Scientific notation fallback
    let s = format!("{sign}{:.2e}", n.abs());
    if s.len() <= max_chars {
        Some(s)
    } else {
        None
    }
}

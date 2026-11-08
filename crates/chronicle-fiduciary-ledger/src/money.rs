//! Exact fixed-point multi-currency money arithmetic.
//!
//! Money amounts are integer counts of minor units (cents) represented as `i64`.
//! Quantities are fixed-point with 4 decimal places (`qty_e4`), unit prices have
//! 2 extra decimal places (`unit_e2`, i.e. 4dp of the major currency unit), and
//! percentages/rates are represented as basis points (`bp`, 10,000 = 100%).
//! Foreign exchange rates are fixed-point with 8 decimal places (`rate_e8`).

pub type Cents = i64;

/// Commercial rounding: divide rounding half away from zero.
pub fn div_round(n: i128, d: i128) -> i128 {
    assert!(d != 0, "division by zero");
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    if n >= 0 {
        (n + d / 2) / d
    } else {
        -((-n + d / 2) / d)
    }
}

/// Banker's rounding: divide rounding half to even.
pub fn div_round_bankers(n: i128, d: i128) -> i128 {
    assert!(d != 0, "division by zero");
    let (n, d) = if d < 0 { (-n, -d) } else { (n, d) };
    let q = n.div_euclid(d);
    let r = n.rem_euclid(d);
    let twice = 2 * r;
    if twice > d || (twice == d && q % 2 != 0) {
        q + 1
    } else {
        q
    }
}

/// Calculates line net amount (before tax) = qty * unit price * (1 - discount).
pub fn line_amount(qty_e4: i64, unit_e2: i64, discount_bp: i64) -> Cents {
    let gross = qty_e4 as i128 * unit_e2 as i128; // scaled 1e6
    let net = gross * (10_000 - discount_bp as i128); // scaled 1e10
    div_round(net, 10_000_000_000) as i64
}

/// Converts an amount in foreign currency to base currency (1 base = rate foreign).
pub fn to_base(foreign: Cents, rate_e8: i64, foreign_decimals: u32, base_decimals: u32) -> Cents {
    if rate_e8 == 0 {
        return foreign;
    }
    let mut n = foreign as i128 * 100_000_000;
    let mut d = rate_e8 as i128;
    if base_decimals > foreign_decimals {
        n *= 10i128.pow(base_decimals - foreign_decimals);
    }
    if foreign_decimals > base_decimals {
        d *= 10i128.pow(foreign_decimals - base_decimals);
    }
    div_round(n, d) as i64
}

/// Parse a decimal string like "-1,234.50" into a fixed-point integer with `dp` decimals.
pub fn parse_fixed(s: &str, dp: u32) -> Option<i64> {
    let s: String = s
        .trim()
        .chars()
        .filter(|c| *c != ',' && *c != '$' && *c != ' ')
        .collect();
    if s.is_empty() {
        return None;
    }
    let (neg, s) = if let Some(r) = s.strip_prefix('-') {
        (true, r.to_string())
    } else if s.starts_with('(') && s.ends_with(')') {
        (true, s[1..s.len() - 1].to_string())
    } else {
        (false, s)
    };
    let mut parts = s.splitn(2, '.');
    let whole = parts.next()?;
    let frac = parts.next().unwrap_or("");
    if !whole.chars().all(|c| c.is_ascii_digit()) || !frac.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let whole: i128 = if whole.is_empty() {
        0
    } else {
        whole.parse().ok()?
    };
    let mut frac_digits: String = frac.chars().take(dp as usize + 1).collect();
    while frac_digits.len() < dp as usize + 1 {
        frac_digits.push('0');
    }
    let frac_v: i128 = frac_digits.parse().ok()?;
    let scaled = whole * 10i128.pow(dp + 1) + frac_v;
    let v = div_round(scaled, 10) as i64;
    Some(if neg { -v } else { v })
}

pub fn fmt_fixed(v: i64, dp: u32) -> String {
    let neg = v < 0;
    let a = (v as i128).abs();
    let p = 10i128.pow(dp);
    let s = if dp == 0 {
        format!("{}", a)
    } else {
        format!("{}.{:0width$}", a / p, a % p, width = dp as usize)
    };
    if neg {
        format!("-{s}")
    } else {
        s
    }
}

pub fn fmt_money(v: Cents) -> String {
    fmt_fixed(v, 2)
}

/// Formats money with thousands separators: e.g. `12,345.60` or `-1,234.56`.
pub fn fmt_money_grouped(v: Cents) -> String {
    let s = fmt_money(v.abs());
    let (whole, frac) = s.split_once('.').unwrap_or((&s, "00"));
    let mut g = String::new();
    for (i, ch) in whole.chars().enumerate() {
        if i > 0 && (whole.len() - i) % 3 == 0 {
            g.push(',');
        }
        g.push(ch);
    }
    format!("{}{g}.{frac}", if v < 0 { "-" } else { "" })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_grouped_money() {
        assert_eq!(fmt_money_grouped(0), "0.00");
        assert_eq!(fmt_money_grouped(99), "0.99");
        assert_eq!(fmt_money_grouped(123456), "1,234.56");
        assert_eq!(fmt_money_grouped(-123456789), "-1,234,567.89");
    }

    #[test]
    fn rounding_logic() {
        assert_eq!(div_round(5, 2), 3);
        assert_eq!(div_round(-5, 2), -3);
        assert_eq!(div_round(4, 2), 2);

        // Banker's rounding
        assert_eq!(div_round_bankers(5, 2), 2); // 2.5 -> 2 (even)
        assert_eq!(div_round_bankers(7, 2), 4); // 3.5 -> 4 (even)
    }

    #[test]
    fn line_amount_calculation() {
        // 2 items (20000 e4) @ $15.50 (155000 e2) with 10% discount (1000 bp)
        // Gross = $31.00 -> 10% off = $27.90 = 2790 cents
        let net = line_amount(20000, 155000, 1000);
        assert_eq!(net, 2790);
    }

    #[test]
    fn parse_fixed_values() {
        assert_eq!(parse_fixed("123.45", 2), Some(12345));
        assert_eq!(parse_fixed("-1,234.56", 2), Some(-123456));
        assert_eq!(parse_fixed("(500.00)", 2), Some(-50000));
    }
}

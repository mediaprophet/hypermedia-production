//! Physical measurement units, parsing, formatting, conversions, and ruler steps.

use serde::{Deserialize, Serialize};

/// Measurement units (geometry coordinates are internally points: 1/72 inch).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[default]
    Points,
    Picas,
    Inches,
    Millimeters,
    Centimeters,
    Pixels,
    FeetInches,
    Meters,
    Yards,
    Feet,
}

impl Unit {
    pub const ALL: [Unit; 10] = [
        Unit::Points,
        Unit::Picas,
        Unit::Inches,
        Unit::Millimeters,
        Unit::Centimeters,
        Unit::Pixels,
        Unit::FeetInches,
        Unit::Meters,
        Unit::Yards,
        Unit::Feet,
    ];

    /// Points per unit.
    pub fn points(self) -> f64 {
        match self {
            Unit::Points | Unit::Pixels => 1.0,
            Unit::Picas => 12.0,
            Unit::Inches => 72.0,
            Unit::Millimeters => 72.0 / 25.4,
            Unit::Centimeters => 72.0 / 2.54,
            Unit::Meters => 72.0 / 0.0254,
            Unit::Yards => 72.0 * 36.0,
            Unit::Feet | Unit::FeetInches => 72.0 * 12.0,
        }
    }

    pub fn suffix(self) -> &'static str {
        match self {
            Unit::Points => "pt",
            Unit::Picas => "p",
            Unit::Inches => "in",
            Unit::Millimeters => "mm",
            Unit::Centimeters => "cm",
            Unit::Pixels => "px",
            Unit::FeetInches | Unit::Feet => "ft",
            Unit::Meters => "m",
            Unit::Yards => "yd",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Unit::Points => "Points",
            Unit::Picas => "Picas",
            Unit::Inches => "Inches",
            Unit::Millimeters => "Millimeters",
            Unit::Centimeters => "Centimeters",
            Unit::Pixels => "Pixels",
            Unit::FeetInches => "Feet & Inches",
            Unit::Meters => "Meters",
            Unit::Yards => "Yards",
            Unit::Feet => "Feet",
        }
    }

    /// The rulers' label step at `zoom` (screen pixels per point), in this unit.
    pub fn ruler_step(self, zoom: f64) -> f64 {
        const STEPS: [f64; 19] = [
            0.01, 0.02, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 25.0, 50.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 5000.0, 10000.0,
        ];
        STEPS.iter().copied().find(|s| s * self.points() * zoom >= 50.0).unwrap_or(10000.0)
    }

    /// Snap `v` (points) to the nearest ruler tick at `zoom`.
    pub fn snap_to_ruler_tick(self, v: f64, zoom: f64) -> f64 {
        let tick = self.ruler_step(zoom) / 10.0 * self.points();
        if tick <= 0.0 {
            return v;
        }
        (v / tick).round() * tick
    }

    pub fn from_pt(self, v: f64) -> f64 {
        v / self.points()
    }

    pub fn to_pt(self, v: f64) -> f64 {
        v * self.points()
    }

    /// Format a point value in this unit (e.g. `12.5 pt`).
    pub fn format(self, pt: f64) -> String {
        format!("{} {}", self.number(pt), self.suffix())
    }

    /// Format without suffix.
    pub fn number(self, pt: f64) -> String {
        let decimals = if matches!(self, Unit::Points | Unit::Pixels | Unit::Picas) { 3 } else { 4 };
        let s = format!("{:.*}", decimals, self.from_pt(pt));
        let s = s.trim_end_matches('0').trim_end_matches('.');
        if s == "-0" { "0".into() } else { s.into() }
    }

    /// Parse a length string in this unit with optional arithmetic (`+ - * /`).
    pub fn parse(self, s: &str) -> Option<f64> {
        parse_measure(s, self).filter(|v| v.is_finite())
    }

    /// Canonical key name.
    pub fn key(self) -> &'static str {
        match self {
            Unit::Points => "points",
            Unit::Picas => "picas",
            Unit::Inches => "inches",
            Unit::Millimeters => "millimeters",
            Unit::Centimeters => "centimeters",
            Unit::Pixels => "pixels",
            Unit::FeetInches => "feetInches",
            Unit::Meters => "meters",
            Unit::Yards => "yards",
            Unit::Feet => "feet",
        }
    }

    /// Find a unit by label, suffix, or key.
    pub fn named(name: &str) -> Option<Unit> {
        let letters = |s: &'static str| s.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase());
        let given = || name.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase());
        Unit::ALL.into_iter().find(|u| letters(u.label()).eq(given()) || u.suffix().chars().eq(given()) || letters(u.key()).eq(given()))
    }

    pub fn readout(self, pt: f64) -> String {
        format!("{:.2} {}", self.from_pt(pt), self.suffix())
    }
}

fn parse_measure(s: &str, default: Unit) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    // Check arithmetic: + - * /
    for op in ['+', '-', '*', '/'] {
        let skip = s.chars().next().map_or(0, char::len_utf8);
        if let Some(i) = s[skip..].rfind(op).map(|i| i + skip) {
            let (l, r) = (&s[..i], &s[i + 1..]);
            if l.trim().is_empty() || r.trim().is_empty() {
                continue;
            }
            let (lv, rv) = (parse_measure(l, default)?, parse_measure(r, default)?);
            let res = match op {
                '+' => lv + rv,
                '-' => lv - rv,
                '*' => lv * (rv / default.points()),
                '/' => if rv.abs() > 1e-12 { lv / (rv / default.points()) } else { return None },
                _ => return None,
            };
            return Some(res);
        }
    }

    // Check for unit suffix in the token
    let num_end = s.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+')).unwrap_or(s.len());
    let (num_str, suffix_str) = (&s[..num_end], s[num_end..].trim());
    let val: f64 = num_str.parse().ok()?;

    if suffix_str.is_empty() {
        Some(default.to_pt(val))
    } else if let Some(unit) = Unit::named(suffix_str) {
        Some(unit.to_pt(val))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_conversion() {
        assert_eq!(Unit::Points.points(), 1.0);
        assert_eq!(Unit::Inches.points(), 72.0);
        assert_eq!(Unit::Picas.points(), 12.0);
        assert!((Unit::Millimeters.points() - 72.0 / 25.4).abs() < 1e-9);
    }

    #[test]
    fn parse_with_units_and_arithmetic() {
        assert_eq!(Unit::Points.parse("72"), Some(72.0));
        assert_eq!(Unit::Points.parse("1 in"), Some(72.0));
        assert_eq!(Unit::Points.parse("2.54 cm"), Some(72.0));
        assert_eq!(Unit::Points.parse("10 + 5"), Some(15.0));
        assert_eq!(Unit::Points.parse("100 / 2"), Some(50.0));
        assert_eq!(Unit::Inches.parse("1 + 1"), Some(144.0));
    }

    #[test]
    fn formatting() {
        assert_eq!(Unit::Points.format(72.0), "72 pt");
        assert_eq!(Unit::Inches.format(72.0), "1 in");
        assert_eq!(Unit::Millimeters.format(72.0), "25.4 mm");
    }

    #[test]
    fn unit_named() {
        assert_eq!(Unit::named("pt"), Some(Unit::Points));
        assert_eq!(Unit::named("in"), Some(Unit::Inches));
        assert_eq!(Unit::named("mm"), Some(Unit::Millimeters));
        assert_eq!(Unit::named("cm"), Some(Unit::Centimeters));
        assert_eq!(Unit::named("picas"), Some(Unit::Picas));
    }
}

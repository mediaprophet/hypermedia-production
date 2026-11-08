//! Format rendering engine.

use crate::core::date::{DateSystem, MONTHS, WEEKDAYS, datetime_from_serial};
use crate::numfmt::decimal::{fixed, format_general_fit};
use crate::numfmt::parse::{DatePart, Section, Tok};

#[derive(Default)]
pub struct Out {
    pub text: String,
    pub fill: Option<(char, usize)>,
}

impl Out {
    pub fn push(&mut self, s: &str) {
        self.text.push_str(s);
    }
    pub fn prepend(&mut self, s: &str) {
        self.text.insert_str(0, s);
    }
}

pub fn render_text(sec: &Section, text: &str) -> Out {
    let mut out = Out::default();
    for t in &sec.toks {
        match t {
            Tok::At => out.push(text),
            Tok::Fill(c) => out.fill = Some((*c, out.text.len())),
            other => out.push(&other.literal_text()),
        }
    }
    out
}

pub fn render_general(sec: &Section, v: f64, width: usize) -> Out {
    let mut out = Out::default();
    for t in &sec.toks {
        match t {
            Tok::General => out.push(&format_general_fit(v, width).unwrap_or_else(|| "0".into())),
            Tok::Fill(c) => out.fill = Some((*c, out.text.len())),
            other => out.push(&other.literal_text()),
        }
    }
    out
}

pub fn render_date(sec: &Section, v: f64, sys: DateSystem) -> Option<Out> {
    if !v.is_finite() || v < 0.0 {
        return None;
    }
    let dt = datetime_from_serial(sys, v)?;
    let mut out = Out::default();
    let month_name = MONTHS.get((dt.month as usize).saturating_sub(1)).copied().unwrap_or("");
    let day_name = WEEKDAYS.get(dt.weekday as usize).copied().unwrap_or("");

    for t in &sec.toks {
        match t {
            Tok::Date(d) => match d {
                DatePart::Year(n) => {
                    if *n <= 2 {
                        out.push(&format!("{:02}", dt.year.rem_euclid(100)));
                    } else {
                        out.push(&format!("{:04}", dt.year));
                    }
                }
                DatePart::Month(n) => match n {
                    1 => out.push(&dt.month.to_string()),
                    2 => out.push(&format!("{:02}", dt.month)),
                    3 => out.push(&month_name.chars().take(3).collect::<String>()),
                    _ => out.push(month_name),
                },
                DatePart::Day(n) => match n {
                    1 => out.push(&dt.day.to_string()),
                    2 => out.push(&format!("{:02}", dt.day)),
                    3 => out.push(&day_name.chars().take(3).collect::<String>()),
                    _ => out.push(day_name),
                },
                DatePart::Hour(n) => {
                    if *n <= 1 {
                        out.push(&dt.hour.to_string());
                    } else {
                        out.push(&format!("{:02}", dt.hour));
                    }
                }
                DatePart::Minute(n) => {
                    if *n <= 1 {
                        out.push(&dt.minute.to_string());
                    } else {
                        out.push(&format!("{:02}", dt.minute));
                    }
                }
                DatePart::Second(n) => {
                    if *n <= 1 {
                        out.push(&dt.second.to_string());
                    } else {
                        out.push(&format!("{:02}", dt.second));
                    }
                }
                DatePart::SubSec(_) => out.push(&format!("{:03}", dt.milli)),
                DatePart::AmPm => {
                    out.push(if dt.hour >= 12 { "PM" } else { "AM" });
                }
            },
            other => out.push(&other.literal_text()),
        }
    }
    Some(out)
}

pub fn render_number(sec: &Section, v: f64) -> Out {
    let mut out = Out::default();
    let mut val = v;
    if sec.percent > 0 {
        val *= 100.0_f64.powi(sec.percent as i32);
    }
    let (mut int_str, frac_str) = fixed(val, sec.decimals);
    if int_str.is_empty() {
        int_str = "0".to_string();
    }
    if sec.thousands {
        let mut with_commas = String::new();
        let bytes = int_str.as_bytes();
        let len = bytes.len();
        for (idx, &b) in bytes.iter().enumerate() {
            if idx > 0 && (len - idx) % 3 == 0 {
                with_commas.push(',');
            }
            with_commas.push(b as char);
        }
        int_str = with_commas;
    }

    let mut emitted_num = false;
    for t in &sec.toks {
        match t {
            Tok::Digit(_) | Tok::Point if !emitted_num => {
                emitted_num = true;
                out.push(&int_str);
                if sec.decimals > 0 {
                    out.push(".");
                    out.push(&frac_str);
                }
            }
            Tok::Digit(_) | Tok::Point => {}
            Tok::Percent => out.push("%"),
            Tok::Fill(c) => out.fill = Some((*c, out.text.len())),
            other => out.push(&other.literal_text()),
        }
    }
    if !emitted_num {
        out.push(&int_str);
        if sec.decimals > 0 {
            out.push(".");
            out.push(&frac_str);
        }
    }
    out
}

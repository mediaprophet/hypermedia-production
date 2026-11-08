//! Excel-compatible number format engine.

pub mod builtin;
pub mod decimal;
pub mod parse;
pub mod render;

use std::sync::Arc;
use crate::core::date::DateSystem;
use crate::core::value::Value;

pub use builtin::{builtin_format, builtin_id};
pub use decimal::format_general_fit;
pub use parse::{FormatColor, SecKind, Section};
pub use render::Out;

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormat {
    pub code: String,
    pub sections: Vec<Section>,
}

impl Default for NumberFormat {
    fn default() -> Self {
        NumberFormat::general()
    }
}

impl NumberFormat {
    pub fn parse(code: &str) -> Self {
        let sections = parse::parse_sections(code);
        NumberFormat {
            code: code.to_string(),
            sections,
        }
    }

    pub fn general() -> Self {
        Self::parse("General")
    }

    pub fn format(&self, v: &Value, sys: DateSystem) -> String {
        match v {
            Value::Empty => String::new(),
            Value::Text(s) => {
                if let Some(sec) = self.sections.iter().find(|s| s.kind == SecKind::Text) {
                    render::render_text(sec, s).text
                } else {
                    s.clone()
                }
            }
            Value::Bool(b) => if *b { "TRUE".into() } else { "FALSE".into() },
            Value::Error(e) => e.as_str().into(),
            Value::Number(n) => {
                let sec = if *n < 0.0 && self.sections.len() >= 2 {
                    &self.sections[1]
                } else if *n == 0.0 && self.sections.len() >= 3 {
                    &self.sections[2]
                } else {
                    &self.sections[0]
                };

                match sec.kind {
                    SecKind::General => render::render_general(sec, *n, 11).text,
                    SecKind::Date => render::render_date(sec, *n, sys).map_or_else(|| "########".into(), |o| o.text),
                    SecKind::Number => {
                        let mut out = render::render_number(sec, n.abs());
                        if *n < 0.0 && self.sections.len() < 2 {
                            out.prepend("-");
                        }
                        out.text
                    }
                    SecKind::Text => render::render_text(sec, &n.to_string()).text,
                }
            }
            Value::Array(a) => {
                if let Some(first) = a.get(0, 0) {
                    self.format(first, sys)
                } else {
                    String::new()
                }
            }
        }
    }
}

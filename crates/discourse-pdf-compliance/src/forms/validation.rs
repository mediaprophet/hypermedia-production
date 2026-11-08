//! AcroForm native formatting, validation, and calculation rules.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use super::model::AcroForm;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum FormatRule {
    #[default]
    None,
    Number { decimals: u8, currency_prefix: String },
    Percent { decimals: u8 },
    SpecialZip,
    SpecialPhone,
    SpecialSsn,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum ValidateRule {
    #[default]
    None,
    Range { min: Option<f64>, max: Option<f64> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalcOp {
    Sum,
    Product,
    Average,
    Minimum,
    Maximum,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum CalculateRule {
    #[default]
    None,
    Simple { op: CalcOp, field_names: Vec<String> },
}

pub fn format_field(value: &str, rule: &FormatRule) -> String {
    match rule {
        FormatRule::None => value.to_string(),
        FormatRule::Number { decimals, currency_prefix } => {
            if let Ok(v) = value.trim().parse::<f64>() {
                format!("{}{:.prec$}", currency_prefix, v, prec = *decimals as usize)
            } else {
                value.to_string()
            }
        }
        FormatRule::Percent { decimals } => {
            if let Ok(v) = value.trim().parse::<f64>() {
                format!("{:.prec$}%", v * 100.0, prec = *decimals as usize)
            } else {
                value.to_string()
            }
        }
        FormatRule::SpecialZip => {
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() == 9 {
                format!("{}-{}", &digits[0..5], &digits[5..9])
            } else {
                value.to_string()
            }
        }
        FormatRule::SpecialPhone => {
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() == 10 {
                format!("({}) {}-{}", &digits[0..3], &digits[3..6], &digits[6..10])
            } else {
                value.to_string()
            }
        }
        FormatRule::SpecialSsn => {
            let digits: String = value.chars().filter(|c| c.is_ascii_digit()).collect();
            if digits.len() == 9 {
                format!("{}-{}-{}", &digits[0..3], &digits[3..5], &digits[5..9])
            } else {
                value.to_string()
            }
        }
    }
}

pub fn validate_field(value: &str, rule: &ValidateRule) -> Result<(), String> {
    match rule {
        ValidateRule::None => Ok(()),
        ValidateRule::Range { min, max } => {
            let v: f64 = value.trim().parse().map_err(|_| format!("`{value}` is not a valid number"))?;
            if let Some(min_v) = min {
                if v < *min_v {
                    return Err(format!("Value {v} is less than minimum {min_v}"));
                }
            }
            if let Some(max_v) = max {
                if v > *max_v {
                    return Err(format!("Value {v} exceeds maximum {max_v}"));
                }
            }
            Ok(())
        }
    }
}

pub fn execute_calculation(form: &AcroForm, rule: &CalculateRule) -> Option<f64> {
    match rule {
        CalculateRule::None => None,
        CalculateRule::Simple { op, field_names } => {
            let values: Vec<f64> = field_names
                .iter()
                .filter_map(|name| form.field(name))
                .filter_map(|f| f.value.trim().parse::<f64>().ok())
                .collect();

            if values.is_empty() {
                return None;
            }

            let result = match op {
                CalcOp::Sum => values.iter().sum(),
                CalcOp::Product => values.iter().product(),
                CalcOp::Average => values.iter().sum::<f64>() / values.len() as f64,
                CalcOp::Minimum => values.into_iter().fold(f64::INFINITY, f64::min),
                CalcOp::Maximum => values.into_iter().fold(-f64::INFINITY, f64::max),
            };
            Some(result)
        }
    }
}

//! SQL-like attribute filter expression evaluator for geospatial and telemetry streams.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Literal values in attribute queries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl AttributeValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Text(s) => s.trim().parse().ok(),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Text(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

/// Comparison operators for attribute queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Like,
}

/// A filter predicate expression.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FilterExpr {
    Compare {
        field: String,
        op: ComparisonOp,
        value: AttributeValue,
    },
    In {
        field: String,
        values: Vec<AttributeValue>,
    },
    And(Vec<FilterExpr>),
    Or(Vec<FilterExpr>),
    Not(Box<FilterExpr>),
    AlwaysTrue,
}

impl FilterExpr {
    /// Evaluates the expression against an attribute dictionary.
    pub fn evaluate(&self, record: &HashMap<String, AttributeValue>) -> bool {
        match self {
            Self::AlwaysTrue => true,
            Self::Compare { field, op, value } => {
                let actual = record.get(field).unwrap_or(&AttributeValue::Null);
                compare_values(actual, *op, value)
            }
            Self::In { field, values } => {
                let actual = record.get(field).unwrap_or(&AttributeValue::Null);
                values.iter().any(|v| compare_values(actual, ComparisonOp::Eq, v))
            }
            Self::And(children) => children.iter().all(|c| c.evaluate(record)),
            Self::Or(children) => children.iter().any(|c| c.evaluate(record)),
            Self::Not(child) => !child.evaluate(record),
        }
    }
}

fn compare_values(actual: &AttributeValue, op: ComparisonOp, expected: &AttributeValue) -> bool {
    match op {
        ComparisonOp::Eq => actual == expected,
        ComparisonOp::NotEq => actual != expected,
        ComparisonOp::Lt => match (actual.as_f64(), expected.as_f64()) {
            (Some(a), Some(b)) => a < b,
            _ => false,
        },
        ComparisonOp::LtEq => match (actual.as_f64(), expected.as_f64()) {
            (Some(a), Some(b)) => a <= b,
            _ => false,
        },
        ComparisonOp::Gt => match (actual.as_f64(), expected.as_f64()) {
            (Some(a), Some(b)) => a > b,
            _ => false,
        },
        ComparisonOp::GtEq => match (actual.as_f64(), expected.as_f64()) {
            (Some(a), Some(b)) => a >= b,
            _ => false,
        },
        ComparisonOp::Like => match (actual.as_str(), expected.as_str()) {
            (Some(text), Some(pattern)) => match_sql_like(text, pattern),
            _ => false,
        },
    }
}

/// Simple SQL LIKE pattern matcher (% matches any sequence, _ matches single character).
fn match_sql_like(text: &str, pattern: &str) -> bool {
    let t_chars: Vec<char> = text.chars().collect();
    let p_chars: Vec<char> = pattern.chars().collect();
    let mut dp = vec![vec![false; p_chars.len() + 1]; t_chars.len() + 1];

    dp[0][0] = true;
    for j in 1..=p_chars.len() {
        if p_chars[j - 1] == '%' {
            dp[0][j] = dp[0][j - 1];
        }
    }

    for i in 1..=t_chars.len() {
        for j in 1..=p_chars.len() {
            if p_chars[j - 1] == '%' {
                dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
            } else if p_chars[j - 1] == '_' || p_chars[j - 1] == t_chars[i - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            }
        }
    }

    dp[t_chars.len()][p_chars.len()]
}

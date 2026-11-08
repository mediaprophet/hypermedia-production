//! AST representation for the unit-aware algebraic mathematical expression parser.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Unit {
    Px,
    Pt,
    Mm,
    Cm,
    In,
    Rad,
    Deg,
    S,
    Ms,
    Hz,
    Percent,
}

impl Unit {
    pub fn parse(s: &str) -> Option<Unit> {
        match s.to_ascii_lowercase().as_str() {
            "px" => Some(Unit::Px),
            "pt" => Some(Unit::Pt),
            "mm" => Some(Unit::Mm),
            "cm" => Some(Unit::Cm),
            "in" => Some(Unit::In),
            "rad" => Some(Unit::Rad),
            "deg" | "°" => Some(Unit::Deg),
            "s" | "sec" => Some(Unit::S),
            "ms" => Some(Unit::Ms),
            "hz" => Some(Unit::Hz),
            "%" => Some(Unit::Percent),
            _ => None,
        }
    }

    /// Converts standard unit to its base canonical dimension:
    /// Length -> pixels (at 96 DPI: 1 in = 96 px, 1 pt = 96/72 px, 1 mm = 96/25.4 px)
    /// Angle -> radians
    /// Time -> seconds
    pub fn to_base_scale(self) -> f64 {
        match self {
            Unit::Px => 1.0,
            Unit::In => 96.0,
            Unit::Pt => 96.0 / 72.0,
            Unit::Mm => 96.0 / 25.4,
            Unit::Cm => 96.0 / 2.54,
            Unit::Rad => 1.0,
            Unit::Deg => std::f64::consts::PI / 180.0,
            Unit::S => 1.0,
            Unit::Ms => 0.001,
            Unit::Hz => 1.0,
            Unit::Percent => 0.01,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Neg,
    Pos,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MathNode {
    Number(f64),
    UnitValue(f64, Unit),
    Variable(String),
    Unary(UnaryOp, Box<MathNode>),
    Binary(BinaryOp, Box<MathNode>, Box<MathNode>),
    FunctionCall(String, Vec<MathNode>),
    Vector(Vec<f64>),
}

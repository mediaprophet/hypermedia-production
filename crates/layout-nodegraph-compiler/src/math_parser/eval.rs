//! Evaluation of mathematical expressions with units and context variables.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use super::ast::{BinaryOp, MathNode, UnaryOp};
use super::parser::parse;

#[derive(Clone, Debug, Default)]
pub struct MathContext {
    pub variables: BTreeMap<String, f64>,
}

impl MathContext {
    pub fn new() -> Self {
        let mut ctx = MathContext::default();
        ctx.set_var("pi", std::f64::consts::PI);
        ctx.set_var("e", std::f64::consts::E);
        ctx.set_var("tau", std::f64::consts::TAU);
        ctx.set_var("phi", 1.618033988749895);
        ctx
    }

    pub fn set_var(&mut self, name: &str, val: f64) {
        self.variables.insert(name.to_lowercase(), val);
    }

    pub fn eval(&self, expr_str: &str) -> Result<f64, String> {
        let ast = parse(expr_str)?;
        self.eval_node(&ast)
    }

    pub fn eval_node(&self, node: &MathNode) -> Result<f64, String> {
        match node {
            MathNode::Number(n) => Ok(*n),
            MathNode::UnitValue(n, u) => Ok(n * u.to_base_scale()),
            MathNode::Variable(name) => {
                let lower = name.to_lowercase();
                self.variables.get(&lower).copied().ok_or_else(|| format!("Unknown variable `{name}`"))
            }
            MathNode::Unary(op, inner) => {
                let v = self.eval_node(inner)?;
                match op {
                    UnaryOp::Pos => Ok(v),
                    UnaryOp::Neg => Ok(-v),
                }
            }
            MathNode::Binary(op, left, right) => {
                let lv = self.eval_node(left)?;
                let rv = self.eval_node(right)?;
                match op {
                    BinaryOp::Add => Ok(lv + rv),
                    BinaryOp::Sub => Ok(lv - rv),
                    BinaryOp::Mul => Ok(lv * rv),
                    BinaryOp::Div => {
                        if rv.abs() < 1e-15 {
                            return Err("Division by zero in math expression".into());
                        }
                        Ok(lv / rv)
                    }
                    BinaryOp::Mod => Ok(lv % rv),
                    BinaryOp::Pow => Ok(lv.powf(rv)),
                }
            }
            MathNode::FunctionCall(name, args) => {
                let lower = name.to_lowercase();
                let eval_args: Result<Vec<f64>, String> = args.iter().map(|a| self.eval_node(a)).collect();
                let a = eval_args?;
                match lower.as_str() {
                    "sin" if a.len() == 1 => Ok(a[0].sin()),
                    "cos" if a.len() == 1 => Ok(a[0].cos()),
                    "tan" if a.len() == 1 => Ok(a[0].tan()),
                    "sqrt" if a.len() == 1 => {
                        if a[0] < 0.0 {
                            return Err("Square root of negative number".into());
                        }
                        Ok(a[0].sqrt())
                    }
                    "abs" if a.len() == 1 => Ok(a[0].abs()),
                    "exp" if a.len() == 1 => Ok(a[0].exp()),
                    "ln" if a.len() == 1 => {
                        if a[0] <= 0.0 {
                            return Err("Logarithm of non-positive number".into());
                        }
                        Ok(a[0].ln())
                    }
                    "min" if a.len() == 2 => Ok(a[0].min(a[1])),
                    "max" if a.len() == 2 => Ok(a[0].max(a[1])),
                    "clamp" if a.len() == 3 => Ok(a[0].clamp(a[1], a[2])),
                    "round" if a.len() == 1 => Ok(a[0].round()),
                    "floor" if a.len() == 1 => Ok(a[0].floor()),
                    "ceil" if a.len() == 1 => Ok(a[0].ceil()),
                    _ => Err(format!("Unknown or mismatched function `{name}` with {} args", a.len())),
                }
            }
            MathNode::Vector(v) => {
                v.first().copied().ok_or("Empty vector".into())
            }
        }
    }
}

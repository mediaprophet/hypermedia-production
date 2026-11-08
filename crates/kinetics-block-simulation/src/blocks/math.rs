//! Math and routing blocks: Gain, Sum, Product, Saturation, DeadZone, Switch, Relational, LogicalOperator, Abs.

use crate::block::{Block, Params};
use crate::blocks::params;
use crate::signal::{in_bool, in_real, PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

pub struct Gain {
    k: Vec<f64>,
    reqs: [PortReq; 1],
}

impl Gain {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["gain"])?;
        let k = params::vec(p, "gain", vec![1.0])?;
        Ok(Self {
            k,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Gain {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let m = ins.first().copied().flatten().unwrap_or(SigMeta::real(1));
        Ok(m)
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        if let Some(SigVal::Real(v)) = y.get_mut(0) {
            for (i, o) in v.iter_mut().enumerate() {
                let k = self.k.get(i).copied().unwrap_or_else(|| self.k[0]);
                *o = k * uu.get(i).copied().unwrap_or(0.0);
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Sum {
    signs: Vec<bool>, // true = '+', false = '-'
    reqs: Vec<PortReq>,
}

impl Sum {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["signs"])?;
        let s = params::string(p, "signs", "++")?;
        let signs: Vec<bool> = s
            .chars()
            .map(|c| match c {
                '+' => Ok(true),
                '-' => Ok(false),
                _ => Err(SimError::Msg(format!("sum signs must be '+' or '-', got {c}"))),
            })
            .collect::<Result<_, _>>()?;
        let n = signs.len().max(1);
        Ok(Self {
            signs,
            reqs: vec![PortReq::REAL_ANY; n],
        })
    }
}

impl Block for Sum {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.iter().filter_map(|m| *m).map(|m| m.width).max().unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.fill(0.0);
            for (p_idx, &sign) in self.signs.iter().enumerate() {
                if let Ok(uu) = in_real(u, p_idx) {
                    for (i, o) in out.iter_mut().enumerate() {
                        let val = uu.get(i).copied().unwrap_or(0.0);
                        if sign {
                            *o += val;
                        } else {
                            *o -= val;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Product {
    ops: Vec<bool>, // true = '*', false = '/'
    reqs: Vec<PortReq>,
}

impl Product {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["ops"])?;
        let s = params::string(p, "ops", "**")?;
        let ops: Vec<bool> = s
            .chars()
            .map(|c| match c {
                '*' => Ok(true),
                '/' => Ok(false),
                _ => Err(SimError::Msg(format!("product ops must be '*' or '/', got {c}"))),
            })
            .collect::<Result<_, _>>()?;
        let n = ops.len().max(1);
        Ok(Self {
            ops,
            reqs: vec![PortReq::REAL_ANY; n],
        })
    }
}

impl Block for Product {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.iter().filter_map(|m| *m).map(|m| m.width).max().unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.fill(1.0);
            for (p_idx, &is_mul) in self.ops.iter().enumerate() {
                if let Ok(uu) = in_real(u, p_idx) {
                    for (i, o) in out.iter_mut().enumerate() {
                        let val = uu.get(i).copied().unwrap_or(1.0);
                        if is_mul {
                            *o *= val;
                        } else if val.abs() > 1e-12 {
                            *o /= val;
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Saturation {
    upper: f64,
    lower: f64,
    reqs: [PortReq; 1],
}

impl Saturation {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["upper", "lower"])?;
        Ok(Self {
            upper: params::num(p, "upper", 1.0)?,
            lower: params::num(p, "lower", -1.0)?,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Saturation {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            for (i, o) in out.iter_mut().enumerate() {
                let v = uu.get(i).copied().unwrap_or(0.0);
                *o = v.clamp(self.lower, self.upper);
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct DeadZone {
    start: f64,
    end: f64,
    reqs: [PortReq; 1],
}

impl DeadZone {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["start", "end"])?;
        Ok(Self {
            start: params::num(p, "start", -0.5)?,
            end: params::num(p, "end", 0.5)?,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for DeadZone {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            for (i, o) in out.iter_mut().enumerate() {
                let v = uu.get(i).copied().unwrap_or(0.0);
                if v > self.end {
                    *o = v - self.end;
                } else if v < self.start {
                    *o = v - self.start;
                } else {
                    *o = 0.0;
                }
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Switch {
    threshold: f64,
    reqs: [PortReq; 3], // [in1, ctrl, in2]
}

impl Switch {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["threshold"])?;
        Ok(Self {
            threshold: params::num(p, "threshold", 0.0)?,
            reqs: [PortReq::REAL_ANY, PortReq::ANY, PortReq::REAL_ANY],
        })
    }
}

impl Block for Switch {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins[0].map(|m| m.width).or_else(|| ins[2].map(|m| m.width)).unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let in1 = in_real(u, 0)?;
        let in2 = in_real(u, 2)?;

        let pass1 = match u.get(1) {
            Some(SigVal::Bool(b)) => b.first().copied().unwrap_or(false),
            Some(SigVal::Real(r)) => r.first().copied().unwrap_or(0.0) >= self.threshold,
            _ => false,
        };

        let selected = if pass1 { in1 } else { in2 };
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.copy_from_slice(selected);
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Relational {
    operator: String,
    reqs: [PortReq; 2],
}

impl Relational {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["operator"])?;
        Ok(Self {
            operator: params::string(p, "operator", ">=")?,
            reqs: [PortReq::REAL_ANY, PortReq::REAL_ANY],
        })
    }
}

impl Block for Relational {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.iter().filter_map(|m| *m).map(|m| m.width).max().unwrap_or(1);
        Ok(SigMeta::boolean(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let in1 = in_real(u, 0)?;
        let in2 = in_real(u, 1)?;
        if let Some(SigVal::Bool(out)) = y.get_mut(0) {
            for (i, o) in out.iter_mut().enumerate() {
                let a = in1.get(i).copied().unwrap_or(0.0);
                let b = in2.get(i).copied().unwrap_or(0.0);
                *o = match self.operator.as_str() {
                    "==" => (a - b).abs() < 1e-12,
                    "!=" => (a - b).abs() >= 1e-12,
                    "<" => a < b,
                    "<=" => a <= b,
                    ">" => a > b,
                    ">=" => a >= b,
                    _ => false,
                };
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct LogicalOperator {
    operator: String,
    reqs: Vec<PortReq>,
}

impl LogicalOperator {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["operator", "inputs"])?;
        let op = params::string(p, "operator", "AND")?;
        let n = if op.eq_ignore_ascii_case("NOT") {
            1
        } else {
            params::count(p, "inputs", 2)?
        };
        Ok(Self {
            operator: op,
            reqs: vec![PortReq::BOOL_ANY; n],
        })
    }
}

impl Block for LogicalOperator {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.iter().filter_map(|m| *m).map(|m| m.width).max().unwrap_or(1);
        Ok(SigMeta::boolean(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Bool(out)) = y.get_mut(0) {
            if self.operator.eq_ignore_ascii_case("NOT") {
                let b = in_bool(u, 0)?;
                for (i, o) in out.iter_mut().enumerate() {
                    *o = !b.get(i).copied().unwrap_or(false);
                }
            } else {
                let is_and = self.operator.eq_ignore_ascii_case("AND");
                for (i, o) in out.iter_mut().enumerate() {
                    let mut res = is_and;
                    for p in 0..self.reqs.len() {
                        if let Ok(b) = in_bool(u, p) {
                            let val = b.get(i).copied().unwrap_or(false);
                            if is_and {
                                res = res && val;
                            } else {
                                res = res || val;
                            }
                        }
                    }
                    *o = res;
                }
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Abs {
    reqs: [PortReq; 1],
}

impl Abs {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &[])?;
        Ok(Self {
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Abs {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(1);
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            for (i, o) in out.iter_mut().enumerate() {
                *o = uu.get(i).copied().unwrap_or(0.0).abs();
            }
        }
        Ok(())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

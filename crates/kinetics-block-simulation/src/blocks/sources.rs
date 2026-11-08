//! Signal source blocks: Constant, Step, Sine, Ramp, Clock.

use crate::block::{Block, Params};
use crate::blocks::params;
use crate::signal::{PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

pub struct Constant {
    v: Vec<f64>,
    reqs: [PortReq; 0],
}

impl Constant {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["value"])?;
        let v = params::vec(p, "value", vec![1.0])?;
        Ok(Self { v, reqs: [] })
    }
}

impl Block for Constant {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _port: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(self.v.len()))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.clone_from(&self.v);
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

pub struct Step {
    time: f64,
    before: f64,
    after: f64,
    reqs: [PortReq; 0],
}

impl Step {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["time", "before", "after"])?;
        Ok(Self {
            time: params::num(p, "time", 0.0)?,
            before: params::num(p, "before", 0.0)?,
            after: params::num(p, "after", 1.0)?,
            reqs: [],
        })
    }
}

impl Block for Step {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _port: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let val = if t < self.time { self.before } else { self.after };
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = val;
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

pub struct Sine {
    amplitude: f64,
    frequency: f64,
    phase: f64,
    bias: f64,
    reqs: [PortReq; 0],
}

impl Sine {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["amplitude", "frequency", "phase", "bias"])?;
        Ok(Self {
            amplitude: params::num(p, "amplitude", 1.0)?,
            frequency: params::num(p, "frequency", 1.0)?,
            phase: params::num(p, "phase", 0.0)?,
            bias: params::num(p, "bias", 0.0)?,
            reqs: [],
        })
    }
}

impl Block for Sine {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _port: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let val = self.bias + self.amplitude * (self.frequency * t + self.phase).sin();
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = val;
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

pub struct Ramp {
    slope: f64,
    start_time: f64,
    initial_output: f64,
    reqs: [PortReq; 0],
}

impl Ramp {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["slope", "start_time", "initial_output"])?;
        Ok(Self {
            slope: params::num(p, "slope", 1.0)?,
            start_time: params::num(p, "start_time", 0.0)?,
            initial_output: params::num(p, "initial_output", 0.0)?,
            reqs: [],
        })
    }
}

impl Block for Ramp {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _port: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let val = if t < self.start_time {
            self.initial_output
        } else {
            self.initial_output + self.slope * (t - self.start_time)
        };
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = val;
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

pub struct Clock {
    reqs: [PortReq; 0],
}

impl Clock {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &[])?;
        Ok(Self { reqs: [] })
    }
}

impl Block for Clock {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _port: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = t;
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

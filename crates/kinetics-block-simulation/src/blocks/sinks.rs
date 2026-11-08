//! Sink blocks: Scope, ToWorkspace, and Trace logging.

use crate::block::{Block, Params};
use crate::blocks::params;
use crate::signal::{in_real, PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

#[derive(Debug, Clone, Default)]
pub struct Trace {
    pub t: Vec<f64>,
    pub y: Vec<Vec<f64>>,
}

impl Trace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, t: f64, v: &[f64]) {
        self.t.push(t);
        self.y.push(v.to_vec());
    }
}

pub struct Scope {
    pub trace: Trace,
    reqs: [PortReq; 1],
}

impl Scope {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &[])?;
        Ok(Self {
            trace: Trace::new(),
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Scope {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        0
    }
    fn resolve_out(&mut self, _p: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Err(SimError::Msg("scope has no output ports".into()))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], _y: &mut Vec<SigVal>) -> Result<(), SimError> {
        Ok(())
    }
    fn log(&mut self, t: f64, u: &[&SigVal]) -> Result<(), SimError> {
        let v = in_real(u, 0)?;
        self.trace.push(t, v);
        Ok(())
    }
    fn has_feedthrough(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ToWorkspace {
    pub var: String,
    pub trace: Trace,
    reqs: [PortReq; 1],
}

impl ToWorkspace {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["var"])?;
        let var = params::string(p, "var", "simout")?;
        Ok(Self {
            var,
            trace: Trace::new(),
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for ToWorkspace {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        0
    }
    fn resolve_out(&mut self, _p: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Err(SimError::Msg("to_workspace has no output ports".into()))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], _y: &mut Vec<SigVal>) -> Result<(), SimError> {
        Ok(())
    }
    fn log(&mut self, t: f64, u: &[&SigVal]) -> Result<(), SimError> {
        let v = in_real(u, 0)?;
        self.trace.push(t, v);
        Ok(())
    }
    fn has_feedthrough(&self) -> bool {
        false
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

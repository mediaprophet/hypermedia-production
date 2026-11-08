//! Discrete-time blocks: UnitDelay, ZeroOrderHold.

use crate::block::{Block, Params, SampleTime};
use crate::blocks::params;
use crate::signal::{in_real, PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

pub struct UnitDelay {
    ts: f64,
    held: Vec<f64>,
    reqs: [PortReq; 1],
}

impl UnitDelay {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["sample_time", "x0"])?;
        let ts = params::num(p, "sample_time", 1.0)?;
        let held = params::vec(p, "x0", vec![0.0])?;
        Ok(Self {
            ts,
            held,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for UnitDelay {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(self.held.len());
        if self.held.len() == 1 && w > 1 {
            self.held = vec![self.held[0]; w];
        }
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.clone_from(&self.held);
        }
        Ok(())
    }
    fn update(&mut self, _t: f64, u: &[&SigVal]) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        self.held.copy_from_slice(uu);
        Ok(())
    }
    fn sample_time(&self) -> SampleTime {
        SampleTime::Discrete(self.ts)
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

pub struct ZeroOrderHold {
    ts: f64,
    held: Vec<f64>,
    reqs: [PortReq; 1],
}

impl ZeroOrderHold {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["sample_time"])?;
        let ts = params::num(p, "sample_time", 1.0)?;
        Ok(Self {
            ts,
            held: Vec::new(),
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for ZeroOrderHold {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(1);
        self.held = vec![0.0; w];
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.clone_from(&self.held);
        }
        Ok(())
    }
    fn update(&mut self, _t: f64, u: &[&SigVal]) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        self.held.copy_from_slice(uu);
        Ok(())
    }
    fn sample_time(&self) -> SampleTime {
        SampleTime::Discrete(self.ts)
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

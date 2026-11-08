//! Routing blocks: Mux and Demux.

use crate::block::{Block, Params};
use crate::blocks::params;
use crate::signal::{in_real, PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

pub struct Mux {
    reqs: Vec<PortReq>,
}

impl Mux {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["inputs"])?;
        let n = params::count(p, "inputs", 2)?;
        Ok(Self {
            reqs: vec![PortReq::REAL_ANY; n],
        })
    }
}

impl Block for Mux {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w: usize = ins.iter().filter_map(|m| *m).map(|m| m.width).sum();
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let mut v = Vec::new();
        for i in 0..self.reqs.len() {
            v.extend_from_slice(in_real(u, i)?);
        }
        if let Some(slot) = y.get_mut(0) {
            *slot = SigVal::Real(v);
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

pub struct Demux {
    n: usize,
    reqs: [PortReq; 1],
}

impl Demux {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["outputs"])?;
        let n = params::count(p, "outputs", 2)?;
        Ok(Self {
            n,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Demux {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        self.n
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(0);
        if w != self.n && w != 0 {
            return Err(SimError::Msg(format!(
                "demux splits vector of width {w} into {} scalar ports",
                self.n
            )));
        }
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let v = in_real(u, 0)?;
        for i in 0..self.n {
            if let (Some(slot), Some(&x)) = (y.get_mut(i), v.get(i)) {
                *slot = SigVal::scalar(x);
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

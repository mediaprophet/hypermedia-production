//! Continuous-time blocks: Integrator, Derivative, PidController, TransferFcn, TransportDelay.

use crate::block::{Block, Params};
use crate::blocks::params;
use crate::signal::{in_real, PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;

pub struct Integrator {
    x0: Vec<f64>,
    reqs: [PortReq; 1],
}

impl Integrator {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["x0"])?;
        let x0 = params::vec(p, "x0", vec![0.0])?;
        Ok(Self {
            x0,
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Integrator {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        let w = ins.first().copied().flatten().map(|m| m.width).unwrap_or(self.x0.len());
        if self.x0.len() == 1 && w > 1 {
            self.x0 = vec![self.x0[0]; w];
        }
        Ok(SigMeta::real(w))
    }
    fn outputs(&mut self, _t: f64, x: &[f64], _u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out.copy_from_slice(x);
        }
        Ok(())
    }
    fn derivatives(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], dx: &mut [f64]) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        for (d, &v) in dx.iter_mut().zip(uu.iter()) {
            *d = v;
        }
        Ok(())
    }
    fn n_states(&self) -> usize {
        self.x0.len()
    }
    fn get_x(&self, x: &mut [f64]) -> Result<(), SimError> {
        x.copy_from_slice(&self.x0);
        Ok(())
    }
    fn set_x(&mut self, x: &[f64]) -> Result<(), SimError> {
        self.x0.copy_from_slice(x);
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

pub struct Derivative {
    last_t: f64,
    last_u: Vec<f64>,
    reqs: [PortReq; 1],
}

impl Derivative {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &[])?;
        Ok(Self {
            last_t: 0.0,
            last_u: Vec::new(),
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for Derivative {
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
    fn outputs(&mut self, t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            let dt = t - self.last_t;
            if dt > 1e-12 && !self.last_u.is_empty() {
                for i in 0..out.len().min(uu.len()) {
                    let prev = self.last_u.get(i).copied().unwrap_or(0.0);
                    out[i] = (uu[i] - prev) / dt;
                }
            } else {
                out.fill(0.0);
            }
        }
        self.last_t = t;
        self.last_u = uu.to_vec();
        Ok(())
    }
    fn has_feedthrough(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct PidController {
    kp: f64,
    ki: f64,
    kd: f64,
    last_t: f64,
    last_e: f64,
    x0: [f64; 1],
    reqs: [PortReq; 1],
}

impl PidController {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["kp", "ki", "kd"])?;
        Ok(Self {
            kp: params::num(p, "kp", 1.0)?,
            ki: params::num(p, "ki", 0.0)?,
            kd: params::num(p, "kd", 0.0)?,
            last_t: 0.0,
            last_e: 0.0,
            x0: [0.0],
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for PidController {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, t: f64, x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let e = in_real(u, 0)?.first().copied().unwrap_or(0.0);
        let dt = t - self.last_t;
        let de = if dt > 1e-12 { (e - self.last_e) / dt } else { 0.0 };
        self.last_t = t;
        self.last_e = e;

        let integral = x.first().copied().unwrap_or(0.0);
        let u_out = self.kp * e + self.ki * integral + self.kd * de;

        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = u_out;
        }
        Ok(())
    }
    fn derivatives(&mut self, _t: f64, _x: &[f64], u: &[&SigVal], dx: &mut [f64]) -> Result<(), SimError> {
        let e = in_real(u, 0)?.first().copied().unwrap_or(0.0);
        if let Some(d) = dx.get_mut(0) {
            *d = e;
        }
        Ok(())
    }
    fn n_states(&self) -> usize {
        1
    }
    fn get_x(&self, x: &mut [f64]) -> Result<(), SimError> {
        x[0] = self.x0[0];
        Ok(())
    }
    fn set_x(&mut self, x: &[f64]) -> Result<(), SimError> {
        self.x0[0] = x[0];
        Ok(())
    }
    fn has_feedthrough(&self) -> bool {
        true
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct TransferFcn {
    num: Vec<f64>,
    den: Vec<f64>,
    x0: Vec<f64>,
    reqs: [PortReq; 1],
}

impl TransferFcn {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["num", "den"])?;
        let num = params::req_vec(p, "num")?;
        let den = params::req_vec(p, "den")?;
        let n_states = den.len().saturating_sub(1);
        Ok(Self {
            num,
            den,
            x0: vec![0.0; n_states],
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for TransferFcn {
    fn inputs(&self) -> &[PortReq] {
        &self.reqs
    }
    fn n_outputs(&self) -> usize {
        1
    }
    fn resolve_out(&mut self, _p: usize, _ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError> {
        Ok(SigMeta::real(1))
    }
    fn outputs(&mut self, _t: f64, x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let u_val = in_real(u, 0)?.first().copied().unwrap_or(0.0);
        let n = self.x0.len();
        let a0 = self.den.first().copied().unwrap_or(1.0);

        let mut out_val = 0.0;
        let num_len = self.num.len();
        for (i, &xi) in x.iter().enumerate() {
            if i < num_len {
                out_val += (self.num[num_len - 1 - i] / a0) * xi;
            }
        }
        if num_len > n {
            out_val += (self.num[0] / a0) * u_val;
        }

        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            out[0] = out_val;
        }
        Ok(())
    }
    fn derivatives(&mut self, _t: f64, x: &[f64], u: &[&SigVal], dx: &mut [f64]) -> Result<(), SimError> {
        let u_val = in_real(u, 0)?.first().copied().unwrap_or(0.0);
        let n = self.x0.len();
        if n == 0 {
            return Ok(());
        }
        let a0 = self.den[0];

        // x'_i = x_{i+1} for i < n-1
        for i in 0..n - 1 {
            dx[i] = x[i + 1];
        }
        // x'_last = u - sum(a_k * x_k)
        let mut sum_a = 0.0;
        for i in 0..n {
            sum_a += (self.den[self.den.len() - 1 - i] / a0) * x[i];
        }
        dx[n - 1] = u_val / a0 - sum_a;
        Ok(())
    }
    fn n_states(&self) -> usize {
        self.x0.len()
    }
    fn get_x(&self, x: &mut [f64]) -> Result<(), SimError> {
        x.copy_from_slice(&self.x0);
        Ok(())
    }
    fn set_x(&mut self, x: &[f64]) -> Result<(), SimError> {
        self.x0.copy_from_slice(x);
        Ok(())
    }
    fn has_feedthrough(&self) -> bool {
        self.num.len() >= self.den.len()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct TransportDelay {
    delay: f64,
    history: Vec<(f64, Vec<f64>)>,
    reqs: [PortReq; 1],
}

impl TransportDelay {
    pub fn new(p: &Params) -> Result<Self, SimError> {
        params::check_keys(p, &["delay"])?;
        let delay = params::num(p, "delay", 1.0)?;
        Ok(Self {
            delay,
            history: Vec::new(),
            reqs: [PortReq::REAL_ANY],
        })
    }
}

impl Block for TransportDelay {
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
    fn outputs(&mut self, t: f64, _x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError> {
        let uu = in_real(u, 0)?;
        self.history.push((t, uu.to_vec()));

        let target_t = t - self.delay;
        if let Some(SigVal::Real(out)) = y.get_mut(0) {
            if target_t <= 0.0 || self.history.is_empty() {
                out.fill(0.0);
            } else {
                // Find closest in history
                let mut best_val = &self.history[0].1;
                for (ht, val) in &self.history {
                    if *ht <= target_t {
                        best_val = val;
                    } else {
                        break;
                    }
                }
                out.copy_from_slice(best_val);
            }
        }
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

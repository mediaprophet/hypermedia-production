//! Block trait and parameter abstractions.

use crate::signal::{PortReq, SigMeta, SigVal};
use crate::SimError;
use std::any::Any;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SampleTime {
    Continuous,
    Discrete(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Param {
    Num(f64),
    Vec(Vec<f64>),
    Str(String),
    Bool(bool),
}

impl Param {
    pub fn as_num(&self) -> Option<f64> {
        match self {
            Param::Num(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_vec(&self) -> Option<&[f64]> {
        match self {
            Param::Vec(v) => Some(v),
            Param::Num(n) => Some(std::slice::from_ref(n)),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Param::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Param::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

pub type Params = BTreeMap<String, Param>;

pub trait Block: Send + Sync {
    fn inputs(&self) -> &[PortReq];
    fn n_outputs(&self) -> usize;
    fn resolve_out(&mut self, port: usize, ins: &[Option<SigMeta>]) -> Result<SigMeta, SimError>;
    fn outputs(&mut self, t: f64, x: &[f64], u: &[&SigVal], y: &mut Vec<SigVal>) -> Result<(), SimError>;

    fn derivatives(&mut self, _t: f64, _x: &[f64], _u: &[&SigVal], _dx: &mut [f64]) -> Result<(), SimError> {
        Ok(())
    }

    fn update(&mut self, _t: f64, _u: &[&SigVal]) -> Result<(), SimError> {
        Ok(())
    }

    fn log(&mut self, _t: f64, _u: &[&SigVal]) -> Result<(), SimError> {
        Ok(())
    }

    fn sample_time(&self) -> SampleTime {
        SampleTime::Continuous
    }

    fn has_feedthrough(&self) -> bool {
        true
    }

    fn n_states(&self) -> usize {
        0
    }

    fn get_x(&self, _x: &mut [f64]) -> Result<(), SimError> {
        Ok(())
    }

    fn set_x(&mut self, _x: &[f64]) -> Result<(), SimError> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait BlockFactory {
    fn make(&self, type_name: &str, params: &Params, path: &str) -> Result<Box<dyn Block>, SimError>;
    fn known_types(&self) -> Vec<&'static str>;
}

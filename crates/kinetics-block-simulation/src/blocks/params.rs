//! Parameter reading helpers for blocks.

use crate::block::{Param, Params};
use crate::SimError;

pub fn check_keys(p: &Params, known: &[&str]) -> Result<(), SimError> {
    for k in p.keys() {
        if !known.contains(&k.as_str()) {
            return Err(SimError::Msg(format!("unknown parameter `{k}`")));
        }
    }
    Ok(())
}

pub fn num(p: &Params, key: &str, default: f64) -> Result<f64, SimError> {
    match p.get(key) {
        None => Ok(default),
        Some(v) => v
            .as_num()
            .filter(|n| n.is_finite())
            .ok_or_else(|| SimError::Msg(format!("parameter `{key}` must be a finite number"))),
    }
}

pub fn vec(p: &Params, key: &str, default: Vec<f64>) -> Result<Vec<f64>, SimError> {
    match p.get(key) {
        None => Ok(default),
        Some(v) => match v.as_vec() {
            Some(vv) if vv.iter().all(|x| x.is_finite()) => Ok(vv.to_vec()),
            _ => Err(SimError::Msg(format!("parameter `{key}` must be a number or number array"))),
        },
    }
}

pub fn req_vec(p: &Params, key: &str) -> Result<Vec<f64>, SimError> {
    match p.get(key) {
        Some(v) => match v.as_vec() {
            Some(vv) if !vv.is_empty() && vv.iter().all(|x| x.is_finite()) => Ok(vv.to_vec()),
            _ => Err(SimError::Msg(format!("parameter `{key}` must be a non-empty number array"))),
        },
        None => Err(SimError::Msg(format!("missing parameter `{key}`"))),
    }
}

pub fn string(p: &Params, key: &str, default: &str) -> Result<String, SimError> {
    match p.get(key) {
        None => Ok(default.to_string()),
        Some(v) => v
            .as_str()
            .map(String::from)
            .ok_or_else(|| SimError::Msg(format!("parameter `{key}` must be a string"))),
    }
}

pub fn count(p: &Params, key: &str, default: usize) -> Result<usize, SimError> {
    match p.get(key) {
        None => Ok(default),
        Some(Param::Num(n)) if *n >= 1.0 && n.fract() == 0.0 => Ok(*n as usize),
        _ => Err(SimError::Msg(format!("parameter `{key}` must be a positive integer"))),
    }
}

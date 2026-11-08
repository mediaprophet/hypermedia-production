//! Signal types, port requirements, and runtime data representations.

use crate::SimError;

/// Element type of a signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DType {
    Real,
    Bool,
}

/// Resolved port signature: width (number of elements) x dtype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigMeta {
    pub width: usize,
    pub dtype: DType,
}

impl SigMeta {
    pub fn real(width: usize) -> Self {
        Self {
            width,
            dtype: DType::Real,
        }
    }

    pub fn boolean(width: usize) -> Self {
        Self {
            width,
            dtype: DType::Bool,
        }
    }
}

/// Input port width requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthReq {
    Any,
    Fixed(usize),
}

/// Input port data type requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DTypeReq {
    Real,
    Bool,
    Any,
}

/// Port requirement declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortReq {
    pub width: WidthReq,
    pub dtype: DTypeReq,
}

impl PortReq {
    pub const REAL_ANY: PortReq = PortReq {
        width: WidthReq::Any,
        dtype: DTypeReq::Real,
    };
    pub const BOOL_ANY: PortReq = PortReq {
        width: WidthReq::Any,
        dtype: DTypeReq::Bool,
    };
    pub const ANY: PortReq = PortReq {
        width: WidthReq::Any,
        dtype: DTypeReq::Any,
    };

    pub fn fixed(width: usize) -> Self {
        PortReq {
            width: WidthReq::Fixed(width),
            dtype: DTypeReq::Real,
        }
    }

    pub fn accepts(&self, meta: SigMeta) -> bool {
        let w_ok = match self.width {
            WidthReq::Any => true,
            WidthReq::Fixed(n) => meta.width == n,
        };
        let d_ok = match self.dtype {
            DTypeReq::Any => true,
            DTypeReq::Real => meta.dtype == DType::Real,
            DTypeReq::Bool => meta.dtype == DType::Bool,
        };
        w_ok && d_ok
    }
}

/// Runtime signal value carried along a wire.
#[derive(Debug, Clone, PartialEq)]
pub enum SigVal {
    Real(Vec<f64>),
    Bool(Vec<bool>),
}

impl SigVal {
    pub fn scalar(v: f64) -> Self {
        SigVal::Real(vec![v])
    }

    pub fn boolean(b: bool) -> Self {
        SigVal::Bool(vec![b])
    }

    pub fn width(&self) -> usize {
        match self {
            SigVal::Real(v) => v.len(),
            SigVal::Bool(v) => v.len(),
        }
    }

    pub fn dtype(&self) -> DType {
        match self {
            SigVal::Real(_) => DType::Real,
            SigVal::Bool(_) => DType::Bool,
        }
    }

    pub fn meta(&self) -> SigMeta {
        SigMeta {
            width: self.width(),
            dtype: self.dtype(),
        }
    }
}

pub fn in_real<'a>(u: &'a [&'a SigVal], port: usize) -> Result<&'a [f64], SimError> {
    match u.get(port) {
        Some(SigVal::Real(v)) => Ok(v),
        Some(SigVal::Bool(_)) => Err(SimError::Msg(format!("input port {port} expected real, got bool"))),
        None => Err(SimError::Msg(format!("input port {port} out of range"))),
    }
}

pub fn in_bool<'a>(u: &'a [&'a SigVal], port: usize) -> Result<&'a [bool], SimError> {
    match u.get(port) {
        Some(SigVal::Bool(v)) => Ok(v),
        Some(SigVal::Real(_)) => Err(SimError::Msg(format!("input port {port} expected bool, got real"))),
        None => Err(SimError::Msg(format!("input port {port} out of range"))),
    }
}

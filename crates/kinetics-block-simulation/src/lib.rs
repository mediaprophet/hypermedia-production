//! kinetics-block-simulation: UI-agnostic categorical block diagram simulation kernel.
//!
//! Provides topological compilation, algebraic loop isolation, static signal type
//! and dimension inference, hybrid continuous/discrete integration (Euler, RK4, RK45),
//! a comprehensive block library, and JSON model persistence.
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod block;
pub mod blocks;
pub mod document;
pub mod model;
pub mod signal;
pub mod sim;

pub use block::{Block, BlockFactory, Param, Params, SampleTime};
pub use blocks::StandardBlockFactory;
pub use document::Document;
pub use model::{CompiledModel, Model, Wire};
pub use signal::{in_bool, in_real, DType, DTypeReq, PortReq, SigMeta, SigVal, WidthReq};
pub use sim::{Sim, SimConfig, SimOut, Solver};

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum SimError {
    AlgebraicLoop(Vec<String>),
    DuplicateBlock(String),
    BadWire(String),
    UnconnectedInput { block: String, port: usize },
    MultipleDrivers { block: String, port: usize },
    PortMismatch { src: String, dst: String, reason: String },
    BadParam { block: String, msg: String },
    TooLarge(String),
    Msg(String),
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimError::AlgebraicLoop(names) => write!(f, "algebraic loop: {}", names.join(" -> ")),
            SimError::DuplicateBlock(n) => write!(f, "duplicate block `{n}`"),
            SimError::BadWire(m) => write!(f, "bad wire: {m}"),
            SimError::UnconnectedInput { block, port } => write!(f, "block `{block}` port {port} unconnected"),
            SimError::MultipleDrivers { block, port } => write!(f, "block `{block}` port {port} has multiple drivers"),
            SimError::PortMismatch { src, dst, reason } => write!(f, "mismatch {src} -> {dst}: {reason}"),
            SimError::BadParam { block, msg } => write!(f, "block `{block}`: {msg}"),
            SimError::TooLarge(m) => write!(f, "size limit exceeded: {m}"),
            SimError::Msg(m) => write!(f, "{m}"),
        }
    }
}

impl std::error::Error for SimError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::sinks::Scope;
    use std::collections::BTreeMap;

    #[test]
    fn test_integrator_simulation() {
        let factory = StandardBlockFactory;
        let mut model = Model::new();

        // constant (value=2.0) -> integrator (x0=0.0) -> scope
        let mut p_const = BTreeMap::new();
        p_const.insert("value".to_string(), Param::Num(2.0));
        let c_idx = model.add_block("src", factory.make("constant", &p_const, "src").unwrap()).unwrap();

        let p_int = BTreeMap::new();
        let int_idx = model.add_block("int", factory.make("integrator", &p_int, "int").unwrap()).unwrap();

        let p_scope = BTreeMap::new();
        let sc_idx = model.add_block("scope", factory.make("scope", &p_scope, "scope").unwrap()).unwrap();

        model.add_wire(c_idx, 0, int_idx, 0).unwrap();
        model.add_wire(int_idx, 0, sc_idx, 0).unwrap();

        let compiled = model.compile().unwrap();
        let config = SimConfig {
            solver: Solver::Rk4,
            dt: 0.01,
            tstop: 1.0,
            ..Default::default()
        };

        let mut sim = Sim::new(compiled, config).unwrap();
        let out = sim.run().unwrap();

        // integral of 2.0 dt from 0 to 1 should equal 2.0
        let final_x = *out.x.last().unwrap();
        assert!((final_x - 2.0).abs() < 1e-3);
    }

    #[test]
    fn test_algebraic_loop_detection() {
        let factory = StandardBlockFactory;
        let mut model = Model::new();

        let mut p_gain = BTreeMap::new();
        p_gain.insert("gain".to_string(), Param::Num(2.0));

        let g1 = model.add_block("g1", factory.make("gain", &p_gain, "g1").unwrap()).unwrap();
        let g2 = model.add_block("g2", factory.make("gain", &p_gain, "g2").unwrap()).unwrap();

        // Direct feedthrough cycle
        model.add_wire(g1, 0, g2, 0).unwrap();
        model.add_wire(g2, 0, g1, 0).unwrap();

        let err = model.compile().unwrap_err();
        match err {
            SimError::AlgebraicLoop(_) => {}
            other => panic!("expected AlgebraicLoop error, got {:?}", other),
        }
    }

    #[test]
    fn test_json_document_parse_and_run() {
        let json_text = r#"{
            "format": "block-simulation",
            "version": 1,
            "solver": { "solver": "rk4", "dt": 0.05, "tstop": 1.0 },
            "blocks": [
                { "path": "c", "type": "constant", "params": { "value": 5.0 } },
                { "path": "g", "type": "gain", "params": { "gain": 3.0 } },
                { "path": "sc", "type": "scope" }
            ],
            "wires": [
                { "from": "c/1", "to": "g/1" },
                { "from": "g/1", "to": "sc/1" }
            ]
        }"#;

        let factory = StandardBlockFactory;
        let doc = Document::parse(json_text).unwrap();
        let mut model = doc.to_model(&factory).unwrap();
        let compiled = model.compile().unwrap();

        let mut sim = Sim::new(compiled, doc.solver).unwrap();
        let out = sim.run().unwrap();
        assert_eq!(out.solver, Solver::Rk4);
        assert!((out.t.last().copied().unwrap() - 1.0).abs() < 1e-6);
    }
}

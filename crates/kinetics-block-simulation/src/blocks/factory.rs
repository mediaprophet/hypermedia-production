//! Block factory and catalog.

use crate::block::{Block, BlockFactory, Params};
use crate::blocks::continuous::{Derivative, Integrator, PidController, TransferFcn, TransportDelay};
use crate::blocks::discrete::{UnitDelay, ZeroOrderHold};
use crate::blocks::math::{Abs, DeadZone, Gain, LogicalOperator, Product, Relational, Saturation, Sum, Switch};
use crate::blocks::routing::{Demux, Mux};
use crate::blocks::sinks::{Scope, ToWorkspace};
use crate::blocks::sources::{Clock, Constant, Ramp, Sine, Step};
use crate::SimError;

pub struct BlockInfo {
    pub type_name: &'static str,
    pub label: &'static str,
}

pub struct StandardBlockFactory;

const CATALOG: &[BlockInfo] = &[
    BlockInfo { type_name: "constant", label: "Constant" },
    BlockInfo { type_name: "step", label: "Step" },
    BlockInfo { type_name: "sine", label: "Sine Wave" },
    BlockInfo { type_name: "ramp", label: "Ramp" },
    BlockInfo { type_name: "clock", label: "Clock" },
    BlockInfo { type_name: "gain", label: "Gain" },
    BlockInfo { type_name: "sum", label: "Sum" },
    BlockInfo { type_name: "product", label: "Product" },
    BlockInfo { type_name: "abs", label: "Abs" },
    BlockInfo { type_name: "saturation", label: "Saturation" },
    BlockInfo { type_name: "dead_zone", label: "Dead Zone" },
    BlockInfo { type_name: "switch", label: "Switch" },
    BlockInfo { type_name: "relational", label: "Relational Operator" },
    BlockInfo { type_name: "logical_operator", label: "Logical Operator" },
    BlockInfo { type_name: "integrator", label: "Integrator" },
    BlockInfo { type_name: "derivative", label: "Derivative" },
    BlockInfo { type_name: "transfer_fcn", label: "Transfer Fcn" },
    BlockInfo { type_name: "pid", label: "PID Controller" },
    BlockInfo { type_name: "transport_delay", label: "Transport Delay" },
    BlockInfo { type_name: "unit_delay", label: "Unit Delay" },
    BlockInfo { type_name: "zero_order_hold", label: "Zero-Order Hold" },
    BlockInfo { type_name: "mux", label: "Mux" },
    BlockInfo { type_name: "demux", label: "Demux" },
    BlockInfo { type_name: "scope", label: "Scope" },
    BlockInfo { type_name: "to_workspace", label: "To Workspace" },
];

impl BlockFactory for StandardBlockFactory {
    fn make(&self, type_name: &str, params: &Params, path: &str) -> Result<Box<dyn Block>, SimError> {
        let err = |e: SimError| SimError::Msg(format!("block `{path}` ({type_name}): {e}"));
        let b: Box<dyn Block> = match type_name {
            "constant" => Box::new(Constant::new(params).map_err(err)?),
            "step" => Box::new(Step::new(params).map_err(err)?),
            "sine" => Box::new(Sine::new(params).map_err(err)?),
            "ramp" => Box::new(Ramp::new(params).map_err(err)?),
            "clock" => Box::new(Clock::new(params).map_err(err)?),
            "gain" => Box::new(Gain::new(params).map_err(err)?),
            "sum" => Box::new(Sum::new(params).map_err(err)?),
            "product" => Box::new(Product::new(params).map_err(err)?),
            "abs" => Box::new(Abs::new(params).map_err(err)?),
            "saturation" => Box::new(Saturation::new(params).map_err(err)?),
            "dead_zone" => Box::new(DeadZone::new(params).map_err(err)?),
            "switch" => Box::new(Switch::new(params).map_err(err)?),
            "relational" => Box::new(Relational::new(params).map_err(err)?),
            "logical_operator" => Box::new(LogicalOperator::new(params).map_err(err)?),
            "integrator" => Box::new(Integrator::new(params).map_err(err)?),
            "derivative" => Box::new(Derivative::new(params).map_err(err)?),
            "transfer_fcn" => Box::new(TransferFcn::new(params).map_err(err)?),
            "pid" => Box::new(PidController::new(params).map_err(err)?),
            "transport_delay" => Box::new(TransportDelay::new(params).map_err(err)?),
            "unit_delay" => Box::new(UnitDelay::new(params).map_err(err)?),
            "zero_order_hold" => Box::new(ZeroOrderHold::new(params).map_err(err)?),
            "mux" => Box::new(Mux::new(params).map_err(err)?),
            "demux" => Box::new(Demux::new(params).map_err(err)?),
            "scope" => Box::new(Scope::new(params).map_err(err)?),
            "to_workspace" => Box::new(ToWorkspace::new(params).map_err(err)?),
            other => {
                return Err(SimError::Msg(format!("unknown block type `{other}` for `{path}`")));
            }
        };
        Ok(b)
    }

    fn known_types(&self) -> Vec<&'static str> {
        CATALOG.iter().map(|i| i.type_name).collect()
    }
}

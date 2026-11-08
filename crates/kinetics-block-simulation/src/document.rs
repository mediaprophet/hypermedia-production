//! JSON block-diagram document format serialization and deserialization.

use crate::block::{BlockFactory, Param, Params};
use crate::model::Model;
use crate::sim::{SimConfig, Solver};
use crate::SimError;
use std::collections::BTreeMap;

pub struct Document {
    pub solver: SimConfig,
    blocks: Vec<BlockRecord>,
    wires: Vec<WireRecord>,
}

struct BlockRecord {
    path: String,
    type_name: String,
    params: Params,
}

struct WireRecord {
    src: String,
    dst: String,
}

fn split_endpoint(s: &str) -> Result<(String, usize), SimError> {
    let (p, port) = s
        .rsplit_once('/')
        .ok_or_else(|| SimError::Msg(format!("endpoint `{s}` needs a /port suffix")))?;
    let n: usize = port
        .parse()
        .map_err(|_| SimError::Msg(format!("endpoint `{s}` has a non-numeric port")))?;
    if n == 0 {
        return Err(SimError::Msg(format!("endpoint `{s}`: ports are 1-based")));
    }
    Ok((p.to_string(), n - 1))
}

impl Document {
    pub fn parse(text: &str) -> Result<Document, SimError> {
        let v: serde_json::Value =
            serde_json::from_str(text).map_err(|e| SimError::Msg(format!("invalid JSON: {e}")))?;
        let obj = v
            .as_object()
            .ok_or_else(|| SimError::Msg("model must be a JSON object".into()))?;

        match obj.get("format").and_then(|f| f.as_str()) {
            Some("block-simulation") => {}
            Some(other) => return Err(SimError::Msg(format!("unsupported format tag: {other}"))),
            None => return Err(SimError::Msg("missing format tag".into())),
        }

        let mut solver = SimConfig::default();
        if let Some(s) = obj.get("solver").and_then(|s| s.as_object()) {
            if let Some(name) = s.get("solver").and_then(|x| x.as_str()) {
                solver.solver = Solver::parse(name)
                    .ok_or_else(|| SimError::Msg(format!("unknown solver `{name}`")))?;
            }
            if let Some(dt) = s.get("dt").and_then(|x| x.as_f64()) {
                solver.dt = dt;
            }
            if let Some(tstop) = s.get("tstop").and_then(|x| x.as_f64()) {
                solver.tstop = tstop;
            }
        }

        let mut blocks = Vec::new();
        if let Some(b_arr) = obj.get("blocks").and_then(|b| b.as_array()) {
            for b in b_arr {
                let b_obj = b.as_object().ok_or_else(|| SimError::Msg("block must be an object".into()))?;
                let path = b_obj.get("path").and_then(|p| p.as_str()).ok_or_else(|| SimError::Msg("missing block path".into()))?.to_string();
                let type_name = b_obj.get("type").and_then(|t| t.as_str()).ok_or_else(|| SimError::Msg("missing block type".into()))?.to_string();

                let mut params = BTreeMap::new();
                if let Some(p_obj) = b_obj.get("params").and_then(|p| p.as_object()) {
                    for (k, val) in p_obj {
                        let param = match val {
                            serde_json::Value::Number(n) => Param::Num(n.as_f64().unwrap_or(0.0)),
                            serde_json::Value::String(s) => Param::Str(s.clone()),
                            serde_json::Value::Bool(b) => Param::Bool(*b),
                            serde_json::Value::Array(a) => {
                                let nums: Vec<f64> = a.iter().filter_map(|x| x.as_f64()).collect();
                                Param::Vec(nums)
                            }
                            _ => continue,
                        };
                        params.insert(k.clone(), param);
                    }
                }

                blocks.push(BlockRecord { path, type_name, params });
            }
        }

        let mut wires = Vec::new();
        if let Some(w_arr) = obj.get("wires").and_then(|w| w.as_array()) {
            for w in w_arr {
                let w_obj = w.as_object().ok_or_else(|| SimError::Msg("wire must be an object".into()))?;
                let src = w_obj.get("from").and_then(|f| f.as_str()).ok_or_else(|| SimError::Msg("missing wire from".into()))?.to_string();
                let dst = w_obj.get("to").and_then(|t| t.as_str()).ok_or_else(|| SimError::Msg("missing wire to".into()))?.to_string();
                wires.push(WireRecord { src, dst });
            }
        }

        Ok(Document { solver, blocks, wires })
    }

    pub fn to_model(&self, factory: &dyn BlockFactory) -> Result<Model, SimError> {
        let mut model = Model::new();
        for b in &self.blocks {
            let blk = factory.make(&b.type_name, &b.params, &b.path)?;
            model.add_block(&b.path, blk)?;
        }
        for w in &self.wires {
            let (src_path, src_port) = split_endpoint(&w.src)?;
            let (dst_path, dst_port) = split_endpoint(&w.dst)?;
            let src_b = model.index_of(&src_path).ok_or_else(|| SimError::BadWire(format!("unknown block `{src_path}`")))?;
            let dst_b = model.index_of(&dst_path).ok_or_else(|| SimError::BadWire(format!("unknown block `{dst_path}`")))?;
            model.add_wire(src_b, src_port, dst_b, dst_port)?;
        }
        Ok(model)
    }
}

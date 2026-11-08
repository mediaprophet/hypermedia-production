//! Model graph representation, signal type propagation, and topological compiler.

use crate::block::{Block, SampleTime};
use crate::signal::{SigMeta, SigVal};
use crate::SimError;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wire {
    pub src_block: usize,
    pub src_port: usize,
    pub dst_block: usize,
    pub dst_port: usize,
}

pub struct Model {
    names: Vec<String>,
    index: HashMap<String, usize>,
    blocks: Vec<Box<dyn Block>>,
    wires: Vec<Wire>,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            index: HashMap::new(),
            blocks: Vec::new(),
            wires: Vec::new(),
        }
    }

    pub fn add_block(&mut self, path: &str, block: Box<dyn Block>) -> Result<usize, SimError> {
        if path.is_empty() {
            return Err(SimError::Msg("empty block path".into()));
        }
        if self.index.contains_key(path) {
            return Err(SimError::DuplicateBlock(path.to_string()));
        }
        let idx = self.blocks.len();
        self.blocks.push(block);
        self.names.push(path.to_string());
        self.index.insert(path.to_string(), idx);
        Ok(idx)
    }

    pub fn add_wire(
        &mut self,
        src_block: usize,
        src_port: usize,
        dst_block: usize,
        dst_port: usize,
    ) -> Result<(), SimError> {
        let sb = self.blocks.get(src_block).ok_or_else(|| {
            SimError::BadWire(format!("src block index {src_block} out of range"))
        })?;
        if src_port >= sb.n_outputs() {
            return Err(SimError::BadWire(format!(
                "src block has no output port {src_port}"
            )));
        }
        let db = self.blocks.get(dst_block).ok_or_else(|| {
            SimError::BadWire(format!("dst block index {dst_block} out of range"))
        })?;
        if dst_port >= db.inputs().len() {
            return Err(SimError::BadWire(format!(
                "dst block has no input port {dst_port}"
            )));
        }

        self.wires.push(Wire {
            src_block,
            src_port,
            dst_block,
            dst_port,
        });
        Ok(())
    }

    pub fn index_of(&self, path: &str) -> Option<usize> {
        self.index.get(path).copied()
    }

    pub fn block_names(&self) -> &[String] {
        &self.names
    }

    pub fn block<T: 'static>(&self, path: &str) -> Option<&T> {
        let idx = self.index_of(path)?;
        self.blocks.get(idx)?.as_any().downcast_ref::<T>()
    }

    pub fn block_mut<T: 'static>(&mut self, path: &str) -> Option<&mut T> {
        let idx = self.index_of(path)?;
        self.blocks.get_mut(idx)?.as_any_mut().downcast_mut::<T>()
    }

    pub fn compile(mut self) -> Result<CompiledModel, SimError> {
        let n_blocks = self.blocks.len();

        // 1. Check unconnected inputs and multiple drivers
        let mut drivers: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
        for wire in &self.wires {
            let key = (wire.dst_block, wire.dst_port);
            if drivers.insert(key, (wire.src_block, wire.src_port)).is_some() {
                return Err(SimError::MultipleDrivers {
                    block: self.names[wire.dst_block].clone(),
                    port: wire.dst_port,
                });
            }
        }

        for (bi, b) in self.blocks.iter().enumerate() {
            for port in 0..b.inputs().len() {
                if !drivers.contains_key(&(bi, port)) {
                    return Err(SimError::UnconnectedInput {
                        block: self.names[bi].clone(),
                        port,
                    });
                }
            }
        }

        // 2. Resolve port signatures and signal dimensions
        let mut out_metas: Vec<Vec<SigMeta>> = vec![Vec::new(); n_blocks];
        for (bi, b) in self.blocks.iter().enumerate() {
            out_metas[bi] = vec![SigMeta::real(0); b.n_outputs()];
        }

        let mut in_metas: Vec<Vec<Option<SigMeta>>> = self
            .blocks
            .iter()
            .map(|b| vec![None; b.inputs().len()])
            .collect();

        // Propagate signals iteratively
        let mut changed = true;
        let mut iters = 0;
        while changed && iters < n_blocks + 10 {
            changed = false;
            iters += 1;

            // Update in_metas from drivers
            for (&(dst_b, dst_p), &(src_b, src_p)) in &drivers {
                let meta = out_metas[src_b].get(src_p).copied();
                if meta.is_some() && in_metas[dst_b][dst_p] != meta && meta.unwrap().width > 0 {
                    in_metas[dst_b][dst_p] = meta;
                    changed = true;
                }
            }

            // Call resolve_out on each block
            for bi in 0..n_blocks {
                let n_out = self.blocks[bi].n_outputs();
                for p in 0..n_out {
                    if let Ok(resolved) = self.blocks[bi].resolve_out(p, &in_metas[bi]) {
                        if out_metas[bi][p] != resolved {
                            out_metas[bi][p] = resolved;
                            changed = true;
                        }
                    }
                }
            }
        }

        // Validate all port requirements
        for (bi, b) in self.blocks.iter().enumerate() {
            for (p, req) in b.inputs().iter().enumerate() {
                let m = in_metas[bi][p].ok_or_else(|| SimError::UnconnectedInput {
                    block: self.names[bi].clone(),
                    port: p,
                })?;
                if !req.accepts(m) {
                    let &(src_b, src_p) = &drivers[&(bi, p)];
                    return Err(SimError::PortMismatch {
                        src: format!("{}/{}", self.names[src_b], src_p + 1),
                        dst: format!("{}/{}", self.names[bi], p + 1),
                        reason: format!("accepted {:?}, got {:?}", req, m),
                    });
                }
            }
        }

        // 3. Topological sorting with feedthrough cycle (algebraic loop) detection
        // Adjacency graph on blocks where edge u -> v exists if u feeds v through direct feedthrough
        let mut adj = vec![Vec::new(); n_blocks];
        let mut in_degree = vec![0usize; n_blocks];

        for (&(dst_b, _), &(src_b, _)) in &drivers {
            // If dst_b has feedthrough, execution order must place src_b before dst_b
            if self.blocks[dst_b].has_feedthrough() {
                adj[src_b].push(dst_b);
                in_degree[dst_b] += 1;
            }
        }

        let mut queue: Vec<usize> = (0..n_blocks).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::with_capacity(n_blocks);

        while let Some(u) = queue.pop() {
            order.push(u);
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push(v);
                }
            }
        }

        if order.len() < n_blocks {
            // Algebraic loop detected
            let in_loop: Vec<String> = (0..n_blocks)
                .filter(|&i| in_degree[i] > 0)
                .map(|i| self.names[i].clone())
                .collect();
            return Err(SimError::AlgebraicLoop(in_loop));
        }

        // 4. State allocation
        let mut state_offsets = vec![0usize; n_blocks];
        let mut total_states = 0usize;
        for (bi, b) in self.blocks.iter().enumerate() {
            state_offsets[bi] = total_states;
            total_states += b.n_states();
        }

        let mut x0 = vec![0.0f64; total_states];
        for (bi, b) in self.blocks.iter().enumerate() {
            let ns = b.n_states();
            if ns > 0 {
                let off = state_offsets[bi];
                b.get_x(&mut x0[off..off + ns])?;
            }
        }

        // 5. Pre-allocate outputs
        let mut initial_outputs = Vec::with_capacity(n_blocks);
        for metas in &out_metas {
            let mut outs = Vec::with_capacity(metas.len());
            for m in metas {
                match m.dtype {
                    crate::signal::DType::Real => outs.push(SigVal::Real(vec![0.0; m.width])),
                    crate::signal::DType::Bool => outs.push(SigVal::Bool(vec![false; m.width])),
                }
            }
            initial_outputs.push(outs);
        }

        Ok(CompiledModel {
            names: self.names,
            blocks: self.blocks,
            drivers,
            order,
            state_offsets,
            total_states,
            initial_outputs,
        })
    }
}

pub struct CompiledModel {
    pub names: Vec<String>,
    pub blocks: Vec<Box<dyn Block>>,
    pub drivers: HashMap<(usize, usize), (usize, usize)>,
    pub order: Vec<usize>,
    pub state_offsets: Vec<usize>,
    pub total_states: usize,
    pub initial_outputs: Vec<Vec<SigVal>>,
}

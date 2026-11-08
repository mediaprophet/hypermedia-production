//! Top-level pipeline for dataflow graph compilation into executable ProtoNetwork.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use super::cse::{cse_deduplicate, prune_dead_nodes};
use super::flatten::flatten_network;
use super::node::{NodeId, NodeImplementation, NodeInput, NodeNetwork, TaggedValue};
use super::proto::{ConstructionArgs, ProtoNode, ProtoNetwork};

#[derive(Clone, Debug, Default)]
pub struct CompilerOptions {
    pub enable_inlining: bool,
    pub enable_cse: bool,
    pub enable_dead_code_elimination: bool,
}

impl CompilerOptions {
    pub fn all_optimizations() -> Self {
        CompilerOptions {
            enable_inlining: true,
            enable_cse: true,
            enable_dead_code_elimination: true,
        }
    }
}

pub struct NodeGraphCompiler {
    pub options: CompilerOptions,
}

impl Default for NodeGraphCompiler {
    fn default() -> Self {
        NodeGraphCompiler {
            options: CompilerOptions::all_optimizations(),
        }
    }
}

impl NodeGraphCompiler {
    pub fn new(options: CompilerOptions) -> Self {
        NodeGraphCompiler { options }
    }

    pub fn compile(&self, mut network: NodeNetwork) -> Result<ProtoNetwork, String> {
        let output_id = network.output_node_id().ok_or("Node graph has no export or output node")?;

        // 1. Inlining of composite subnetworks
        if self.options.enable_inlining {
            network = flatten_network(&network);
        }

        // 2. Common Subexpression Elimination (CSE)
        if self.options.enable_cse {
            cse_deduplicate(&mut network);
        }

        // 3. Dead-node elimination
        if self.options.enable_dead_code_elimination {
            prune_dead_nodes(&mut network);
        }

        // 4. Stable topological sorting
        let sorted_ids = topological_sort(&network)?;

        // 5. Build ProtoNetwork
        let mut proto = ProtoNetwork::new(output_id);
        proto.inputs = network.inputs.clone();

        for id in sorted_ids {
            let node = &network.nodes[&id];
            let ident = match &node.implementation {
                NodeImplementation::ProtoNode(ident) => ident.clone(),
                NodeImplementation::Network(_) => "CompositeNetwork".into(),
            };

            let mut node_deps = Vec::new();
            let mut val_arg = TaggedValue::None;

            for inp in &node.inputs {
                match inp {
                    NodeInput::Node { node_id, .. } => {
                        node_deps.push(*node_id);
                    }
                    NodeInput::Value { tagged_value, .. } => {
                        val_arg = tagged_value.clone();
                    }
                    NodeInput::NetworkInput { .. } => {}
                }
            }

            let construction = if !node_deps.is_empty() {
                ConstructionArgs::Nodes(node_deps.clone())
            } else {
                ConstructionArgs::Value(val_arg)
            };

            proto.add_node(id, ProtoNode {
                identifier: ident,
                construction_args: construction,
                input_args: node_deps,
            });
        }

        Ok(proto)
    }
}

fn topological_sort(network: &NodeNetwork) -> Result<Vec<NodeId>, String> {
    let mut visited = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    let mut order = Vec::new();

    for &id in network.nodes.keys() {
        if !visited.contains(&id) {
            dfs(id, network, &mut visited, &mut visiting, &mut order)?;
        }
    }

    Ok(order)
}

fn dfs(
    id: NodeId,
    network: &NodeNetwork,
    visited: &mut BTreeSet<NodeId>,
    visiting: &mut BTreeSet<NodeId>,
    order: &mut Vec<NodeId>,
) -> Result<(), String> {
    if visiting.contains(&id) {
        return Err(format!("Cycle detected in dataflow graph at node {}", id.0));
    }
    if visited.contains(&id) {
        return Ok(());
    }

    visiting.insert(id);

    if let Some(node) = network.nodes.get(&id) {
        for inp in &node.inputs {
            if let NodeInput::Node { node_id, .. } = inp {
                if network.nodes.contains_key(node_id) {
                    dfs(*node_id, network, visited, visiting, order)?;
                }
            }
        }
    }

    visiting.remove(&id);
    visited.insert(id);
    order.push(id);
    Ok(())
}

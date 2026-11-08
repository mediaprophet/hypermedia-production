//! Composite subnetwork inlining and flattening pass.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use super::node::{DocumentNode, NodeId, NodeImplementation, NodeInput, NodeNetwork};

pub fn flatten_network(network: &NodeNetwork) -> NodeNetwork {
    let mut flat = NodeNetwork::new();
    flat.inputs = network.inputs.clone();
    flat.exports = network.exports.clone();

    let mut next_fresh_id: u64 = network.nodes.keys().map(|k| k.0).max().unwrap_or(0) + 1;

    for (&node_id, node) in &network.nodes {
        match &node.implementation {
            NodeImplementation::ProtoNode(_) => {
                flat.nodes.insert(node_id, node.clone());
            }
            NodeImplementation::Network(subnetwork) => {
                // Recursively flatten subnetwork first
                let flat_sub = flatten_network(subnetwork);

                // Map inner NodeIds to fresh outer NodeIds
                let mut id_map: BTreeMap<NodeId, NodeId> = BTreeMap::new();
                for &inner_id in flat_sub.nodes.keys() {
                    id_map.insert(inner_id, NodeId::new(next_fresh_id));
                    next_fresh_id += 1;
                }

                // If subnetwork has an export, map the composite node_id to the export's mapped id
                let sub_output = flat_sub.output_node_id();

                for (&inner_id, inner_node) in &flat_sub.nodes {
                    let mapped_id = id_map[&inner_id];
                    let mut remapped_node = inner_node.clone();

                    // Remap inputs of the inlined node
                    for input in &mut remapped_node.inputs {
                        match input {
                            NodeInput::Node { node_id, .. } => {
                                if let Some(&fresh_id) = id_map.get(node_id) {
                                    *node_id = fresh_id;
                                }
                            }
                            NodeInput::NetworkInput { input_index } => {
                                // Connect to parent caller node's input at input_index
                                if let Some(parent_input) = node.inputs.get(*input_index) {
                                    *input = parent_input.clone();
                                }
                            }
                            _ => {}
                        }
                    }

                    flat.nodes.insert(mapped_id, remapped_node);
                }

                // Replace references to `node_id` with `id_map[&sub_output]` across existing flat nodes
                if let Some(out_id) = sub_output {
                    if let Some(&mapped_out) = id_map.get(&out_id) {
                        for existing_node in flat.nodes.values_mut() {
                            for input in &mut existing_node.inputs {
                                if let NodeInput::Node { node_id: target, .. } = input {
                                    if *target == node_id {
                                        *target = mapped_out;
                                    }
                                }
                            }
                        }
                        for exp in &mut flat.exports {
                            if *exp == node_id {
                                *exp = mapped_out;
                            }
                        }
                    }
                }
            }
        }
    }

    flat
}

//! Common Subexpression Elimination (CSE) and dead-node elimination.

#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::node::{NodeId, NodeImplementation, NodeInput, NodeNetwork, TaggedValue};

fn fnv1a_hash_str(s: &str, mut h: u64) -> u64 {
    for &b in s.as_bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn fnv1a_hash_tagged_value(tv: &TaggedValue, mut h: u64) -> u64 {
    match tv {
        TaggedValue::None => h = fnv1a_hash_str("None", h),
        TaggedValue::Bool(b) => h = fnv1a_hash_str(if *b { "true" } else { "false" }, h),
        TaggedValue::I64(v) => h = fnv1a_hash_str(&v.to_string(), h),
        TaggedValue::U64(v) => h = fnv1a_hash_str(&v.to_string(), h),
        TaggedValue::F64(v) => h = fnv1a_hash_str(&v.to_bits().to_string(), h),
        TaggedValue::String(s) => h = fnv1a_hash_str(s, h),
        TaggedValue::Vec2([x, y]) => {
            h = fnv1a_hash_str(&x.to_bits().to_string(), h);
            h = fnv1a_hash_str(&y.to_bits().to_string(), h);
        }
        TaggedValue::Vec3([x, y, z]) => {
            h = fnv1a_hash_str(&x.to_bits().to_string(), h);
            h = fnv1a_hash_str(&y.to_bits().to_string(), h);
            h = fnv1a_hash_str(&z.to_bits().to_string(), h);
        }
        TaggedValue::Color([r, g, b, a]) => {
            h = fnv1a_hash_str(&r.to_bits().to_string(), h);
            h = fnv1a_hash_str(&g.to_bits().to_string(), h);
            h = fnv1a_hash_str(&b.to_bits().to_string(), h);
            h = fnv1a_hash_str(&a.to_bits().to_string(), h);
        }
        TaggedValue::Custom(ty, data) => {
            h = fnv1a_hash_str(ty, h);
            for &b in data {
                h ^= b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
    }
    h
}

/// Compute deterministic hash trees for all nodes in topological order.
pub fn compute_node_hashes(network: &NodeNetwork) -> BTreeMap<NodeId, u64> {
    let mut hashes: BTreeMap<NodeId, u64> = BTreeMap::new();

    // Loop until all node hashes are stably resolved
    let mut changed = true;
    while changed {
        changed = false;
        for (&id, node) in &network.nodes {
            if hashes.contains_key(&id) {
                continue;
            }

            // Check if all input dependency nodes already have resolved hashes
            let inputs_ready = node.inputs.iter().all(|inp| match inp {
                NodeInput::Node { node_id, .. } => hashes.contains_key(node_id),
                _ => true,
            });

            if inputs_ready {
                let mut h: u64 = 0xcbf29ce484222325;
                h = fnv1a_hash_str(&node.name, h);
                if let NodeImplementation::ProtoNode(ident) = &node.implementation {
                    h = fnv1a_hash_str(ident, h);
                }

                for inp in &node.inputs {
                    match inp {
                        NodeInput::Node { node_id, output_index, lambda } => {
                            let dep_hash = hashes[node_id];
                            h = fnv1a_hash_str(&dep_hash.to_string(), h);
                            h = fnv1a_hash_str(&output_index.to_string(), h);
                            h = fnv1a_hash_str(if *lambda { "L1" } else { "L0" }, h);
                        }
                        NodeInput::Value { tagged_value, exposed } => {
                            h = fnv1a_hash_tagged_value(tagged_value, h);
                            h = fnv1a_hash_str(if *exposed { "E1" } else { "E0" }, h);
                        }
                        NodeInput::NetworkInput { input_index } => {
                            h = fnv1a_hash_str(&format!("NETIN_{input_index}"), h);
                        }
                    }
                }

                hashes.insert(id, h);
                changed = true;
            }
        }
    }

    hashes
}

/// Deduplicate identical subexpressions:
/// If multiple nodes have identical hash trees, the first node is retained as canonical,
/// and all subsequent nodes are rewired to the canonical node.
pub fn cse_deduplicate(network: &mut NodeNetwork) -> usize {
    let hashes = compute_node_hashes(network);
    let mut canonical_by_hash: BTreeMap<u64, NodeId> = BTreeMap::new();
    let mut remappings: BTreeMap<NodeId, NodeId> = BTreeMap::new();
    let mut deduplicated_count = 0;

    for (&id, &h) in &hashes {
        if let Some(&canonical_id) = canonical_by_hash.get(&h) {
            remappings.insert(id, canonical_id);
            deduplicated_count += 1;
        } else {
            canonical_by_hash.insert(h, id);
        }
    }

    if deduplicated_count == 0 {
        return 0;
    }

    // Rewire all references
    for node in network.nodes.values_mut() {
        for input in &mut node.inputs {
            if let NodeInput::Node { node_id, .. } = input {
                if let Some(&canonical) = remappings.get(node_id) {
                    *node_id = canonical;
                }
            }
        }
    }

    for exp in &mut network.exports {
        if let Some(&canonical) = remappings.get(exp) {
            *exp = canonical;
        }
    }

    // Remove duplicate nodes
    for redundant_id in remappings.keys() {
        network.nodes.remove(redundant_id);
    }

    deduplicated_count
}

/// Dead-code elimination:
/// Prunes away all nodes that are not reachable from the exports/output nodes.
pub fn prune_dead_nodes(network: &mut NodeNetwork) -> usize {
    let mut reachable = BTreeSet::new();
    let mut queue = VecDeque::new();

    for &exp in &network.exports {
        queue.push_back(exp);
        reachable.insert(exp);
    }

    while let Some(curr) = queue.pop_front() {
        if let Some(node) = network.nodes.get(&curr) {
            for input in &node.inputs {
                if let NodeInput::Node { node_id, .. } = input {
                    if reachable.insert(*node_id) {
                        queue.push_back(*node_id);
                    }
                }
            }
        }
    }

    let initial_count = network.nodes.len();
    network.nodes.retain(|id, _| reachable.contains(id));
    initial_count - network.nodes.len()
}

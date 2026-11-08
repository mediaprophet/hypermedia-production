//! Typed dataflow document node graph definitions.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(pub u64);

impl NodeId {
    pub const fn new(id: u64) -> Self {
        NodeId(id)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TaggedValue {
    None,
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Vec2([f64; 2]),
    Vec3([f64; 3]),
    Color([f32; 4]),
    Custom(String, Vec<u8>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeInput {
    Node {
        node_id: NodeId,
        output_index: usize,
        lambda: bool,
    },
    Value {
        tagged_value: TaggedValue,
        exposed: bool,
    },
    NetworkInput {
        input_index: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NodeImplementation {
    ProtoNode(String),
    Network(NodeNetwork),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DocumentNode {
    pub name: String,
    pub inputs: Vec<NodeInput>,
    pub implementation: NodeImplementation,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeNetwork {
    pub nodes: BTreeMap<NodeId, DocumentNode>,
    pub inputs: Vec<NodeId>,
    pub exports: Vec<NodeId>,
}

impl NodeNetwork {
    pub fn new() -> Self {
        NodeNetwork::default()
    }

    pub fn add_node(&mut self, id: NodeId, node: DocumentNode) {
        self.nodes.insert(id, node);
    }

    pub fn output_node_id(&self) -> Option<NodeId> {
        self.exports.first().copied().or_else(|| self.nodes.keys().last().copied())
    }
}

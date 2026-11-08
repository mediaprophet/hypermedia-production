//! ProtoNetwork: flattened, intermediate representation of executable nodes.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use super::node::{NodeId, TaggedValue};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConstructionArgs {
    Value(TaggedValue),
    Nodes(Vec<NodeId>),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProtoNode {
    pub identifier: String,
    pub construction_args: ConstructionArgs,
    pub input_args: Vec<NodeId>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ProtoNetwork {
    pub inputs: Vec<NodeId>,
    pub output: NodeId,
    pub nodes: Vec<(NodeId, ProtoNode)>,
}

impl ProtoNetwork {
    pub fn new(output: NodeId) -> Self {
        ProtoNetwork {
            inputs: Vec::new(),
            output,
            nodes: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: NodeId, node: ProtoNode) {
        self.nodes.push((id, node));
    }

    pub fn node(&self, id: NodeId) -> Option<&ProtoNode> {
        self.nodes.iter().find(|(nid, _)| *nid == id).map(|(_, n)| n)
    }

    pub fn node_mut(&mut self, id: NodeId) -> Option<&mut ProtoNode> {
        self.nodes.iter_mut().find(|(nid, _)| *nid == id).map(|(_, n)| n)
    }
}

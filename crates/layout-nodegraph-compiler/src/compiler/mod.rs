//! Pure-Rust typed dataflow node graph compiler.

#![forbid(unsafe_code)]

pub mod cse;
pub mod flatten;
pub mod node;
pub mod pipeline;
pub mod proto;

pub use cse::{compute_node_hashes, cse_deduplicate, prune_dead_nodes};
pub use flatten::flatten_network;
pub use node::{
    DocumentNode, NodeId, NodeImplementation, NodeInput, NodeNetwork, TaggedValue,
};
pub use pipeline::{CompilerOptions, NodeGraphCompiler};
pub use proto::{ConstructionArgs, ProtoNetwork, ProtoNode};

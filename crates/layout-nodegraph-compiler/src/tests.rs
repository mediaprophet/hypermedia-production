//! Comprehensive unit tests for layout-nodegraph-compiler.

#![forbid(unsafe_code)]

use std::collections::BTreeMap;

use super::compiler::cse::{compute_node_hashes, cse_deduplicate, prune_dead_nodes};
use super::compiler::flatten::flatten_network;
use super::compiler::node::{
    DocumentNode, NodeId, NodeImplementation, NodeInput, NodeNetwork, TaggedValue,
};
use super::compiler::pipeline::{CompilerOptions, NodeGraphCompiler};
use super::math_parser::ast::Unit;
use super::math_parser::eval::MathContext;

#[test]
fn test_nodegraph_pipeline_basic() {
    let mut network = NodeNetwork::new();
    let id_a = NodeId::new(1);
    let id_b = NodeId::new(2);
    let id_add = NodeId::new(3);

    // Node 1: Value 10
    network.add_node(
        id_a,
        DocumentNode {
            name: "ValA".into(),
            inputs: vec![NodeInput::Value {
                tagged_value: TaggedValue::F64(10.0),
                exposed: false,
            }],
            implementation: NodeImplementation::ProtoNode("ConstF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    // Node 2: Value 20
    network.add_node(
        id_b,
        DocumentNode {
            name: "ValB".into(),
            inputs: vec![NodeInput::Value {
                tagged_value: TaggedValue::F64(20.0),
                exposed: false,
            }],
            implementation: NodeImplementation::ProtoNode("ConstF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    // Node 3: Add(A, B)
    network.add_node(
        id_add,
        DocumentNode {
            name: "Add".into(),
            inputs: vec![
                NodeInput::Node { node_id: id_a, output_index: 0, lambda: false },
                NodeInput::Node { node_id: id_b, output_index: 0, lambda: false },
            ],
            implementation: NodeImplementation::ProtoNode("AddF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    network.exports = vec![id_add];

    let compiler = NodeGraphCompiler::default();
    let proto = compiler.compile(network).expect("compile successful");
    assert_eq!(proto.output, id_add);
    assert_eq!(proto.nodes.len(), 3);
}

#[test]
fn test_composite_subnetwork_inlining() {
    let mut sub = NodeNetwork::new();
    let sub_in = NodeId::new(10);
    let sub_mult = NodeId::new(11);

    sub.add_node(
        sub_in,
        DocumentNode {
            name: "Input".into(),
            inputs: vec![NodeInput::NetworkInput { input_index: 0 }],
            implementation: NodeImplementation::ProtoNode("Identity".into()),
            metadata: BTreeMap::new(),
        },
    );
    sub.add_node(
        sub_mult,
        DocumentNode {
            name: "MultiplyBy2".into(),
            inputs: vec![
                NodeInput::Node { node_id: sub_in, output_index: 0, lambda: false },
                NodeInput::Value { tagged_value: TaggedValue::F64(2.0), exposed: false },
            ],
            implementation: NodeImplementation::ProtoNode("MulF64".into()),
            metadata: BTreeMap::new(),
        },
    );
    sub.exports = vec![sub_mult];

    let mut parent = NodeNetwork::new();
    let parent_val = NodeId::new(1);
    let parent_composite = NodeId::new(2);

    parent.add_node(
        parent_val,
        DocumentNode {
            name: "Val".into(),
            inputs: vec![NodeInput::Value { tagged_value: TaggedValue::F64(5.0), exposed: false }],
            implementation: NodeImplementation::ProtoNode("ConstF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    parent.add_node(
        parent_composite,
        DocumentNode {
            name: "Doubler".into(),
            inputs: vec![NodeInput::Node { node_id: parent_val, output_index: 0, lambda: false }],
            implementation: NodeImplementation::Network(sub),
            metadata: BTreeMap::new(),
        },
    );
    parent.exports = vec![parent_composite];

    let flattened = flatten_network(&parent);
    // After flattening, composite node is inlined into atomic proto nodes
    assert!(flattened.nodes.values().all(|n| matches!(n.implementation, NodeImplementation::ProtoNode(_))));
    assert_eq!(flattened.nodes.len(), 3); // Val + inlined Input + inlined Multiply
}

#[test]
fn test_common_subexpression_elimination() {
    let mut network = NodeNetwork::new();
    let id_a1 = NodeId::new(1);
    let id_a2 = NodeId::new(2); // identical to A1
    let id_sum = NodeId::new(3);

    network.add_node(
        id_a1,
        DocumentNode {
            name: "Const42".into(),
            inputs: vec![NodeInput::Value { tagged_value: TaggedValue::F64(42.0), exposed: false }],
            implementation: NodeImplementation::ProtoNode("ConstF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    network.add_node(
        id_a2,
        DocumentNode {
            name: "Const42".into(),
            inputs: vec![NodeInput::Value { tagged_value: TaggedValue::F64(42.0), exposed: false }],
            implementation: NodeImplementation::ProtoNode("ConstF64".into()),
            metadata: BTreeMap::new(),
        },
    );

    network.add_node(
        id_sum,
        DocumentNode {
            name: "Add".into(),
            inputs: vec![
                NodeInput::Node { node_id: id_a1, output_index: 0, lambda: false },
                NodeInput::Node { node_id: id_a2, output_index: 0, lambda: false },
            ],
            implementation: NodeImplementation::ProtoNode("AddF64".into()),
            metadata: BTreeMap::new(),
        },
    );
    network.exports = vec![id_sum];

    let dedup_count = cse_deduplicate(&mut network);
    assert_eq!(dedup_count, 1);
    assert_eq!(network.nodes.len(), 2); // 1 and 3 remain, 2 deduplicated!

    // Check that Add inputs now point both to id_a1
    let add_node = &network.nodes[&id_sum];
    assert_eq!(add_node.inputs[0], NodeInput::Node { node_id: id_a1, output_index: 0, lambda: false });
    assert_eq!(add_node.inputs[1], NodeInput::Node { node_id: id_a1, output_index: 0, lambda: false });
}

#[test]
fn test_dead_node_pruning() {
    let mut network = NodeNetwork::new();
    let id_used = NodeId::new(1);
    let id_dead = NodeId::new(2);

    network.add_node(
        id_used,
        DocumentNode {
            name: "Used".into(),
            inputs: vec![],
            implementation: NodeImplementation::ProtoNode("Const".into()),
            metadata: BTreeMap::new(),
        },
    );

    network.add_node(
        id_dead,
        DocumentNode {
            name: "Dead".into(),
            inputs: vec![],
            implementation: NodeImplementation::ProtoNode("Const".into()),
            metadata: BTreeMap::new(),
        },
    );

    network.exports = vec![id_used];

    let pruned = prune_dead_nodes(&mut network);
    assert_eq!(pruned, 1);
    assert_eq!(network.nodes.len(), 1);
    assert!(network.nodes.contains_key(&id_used));
    assert!(!network.nodes.contains_key(&id_dead));
}

#[test]
fn test_math_parser_algebraic_and_units() {
    let ctx = MathContext::new();

    // Standard precedence
    assert_eq!(ctx.eval("2 + 3 * 4").unwrap(), 14.0);
    assert_eq!(ctx.eval("(2 + 3) * 4").unwrap(), 20.0);
    assert_eq!(ctx.eval("2 ^ 3").unwrap(), 8.0);

    // Implicit multiplication
    let pi = std::f64::consts::PI;
    assert!((ctx.eval("2pi").unwrap() - 2.0 * pi).abs() < 1e-10);
    assert_eq!(ctx.eval("5(2 + 3)").unwrap(), 25.0);

    // Scientific notation
    assert_eq!(ctx.eval("2.5e3").unwrap(), 2500.0);
    assert_eq!(ctx.eval("2e-1").unwrap(), 0.2);

    // Functions
    assert_eq!(ctx.eval("sqrt(16)").unwrap(), 4.0);
    assert_eq!(ctx.eval("abs(-42)").unwrap(), 42.0);
    assert_eq!(ctx.eval("min(10, 20)").unwrap(), 10.0);
    assert_eq!(ctx.eval("clamp(150, 0, 100)").unwrap(), 100.0);

    // Unit conversions (canonical: 96 px = 1 inch, 25.4 mm = 1 inch)
    assert_eq!(ctx.eval("1in").unwrap(), 96.0);
    assert!((ctx.eval("25.4mm").unwrap() - 96.0).abs() < 1e-6);
    assert_eq!(ctx.eval("72pt").unwrap(), 96.0);
    assert!((ctx.eval("180deg").unwrap() - pi).abs() < 1e-10);
    assert_eq!(ctx.eval("50%").unwrap(), 0.5);
}

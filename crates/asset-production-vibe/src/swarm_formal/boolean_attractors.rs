//! Terminal strongly connected components of finite asynchronous Boolean systems.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::BTreeMap,
    mem::size_of,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, Args, AssetHost, TypedArtifact, Value,
};

use super::{budget, count_value};

const CONTEXT: &str = "urn:qualia:math:finite-asynchronous-boolean-attractors-v1";
const MAX_NODES: usize = 12;
const MAX_INPUTS: usize = 6;
const MAX_STATES: usize = 1 << MAX_NODES;
const WORKSPACE_BYTES: usize = size_of::<Workspace>() + size_of::<[Rule; MAX_NODES]>();
const UNSEEN: u16 = u16::MAX;
const UNASSIGNED: u16 = MAX_STATES as u16;
const COLUMNS: [&str; 3] = ["state", "component", "terminal_attractor"];

#[derive(Clone, Copy)]
struct Rule {
    input_indices: [u8; MAX_INPUTS],
    input_count: usize,
    table: u64,
}

impl Rule {
    const EMPTY: Self = Self {
        input_indices: [0; MAX_INPUTS],
        input_count: 0,
        table: 0,
    };

    fn parse(value: &Value, nodes: usize) -> Result<Self, String> {
        let rule = record(value)?;
        fields(rule, &["inputs", "table"])?;
        let inputs = list(required(rule, "inputs")?)?;
        let table = list(required(rule, "table")?)?;
        if inputs.len() > MAX_INPUTS || table.len() != 1 << inputs.len() {
            return Err(
                "Each rule requires at most six inputs and exactly 2^inputs Boolean entries".into(),
            );
        }
        let mut parsed = Self {
            input_count: inputs.len(),
            ..Self::EMPTY
        };
        let mut used = 0u16;
        for (index, input) in parsed.input_indices.iter_mut().zip(inputs) {
            let node = integer(input, "input node")?;
            if node >= nodes || used & (1 << node) != 0 {
                return Err(
                    "Each rule requires distinct input nodes within the zero-based node range"
                        .into(),
                );
            }
            *index = node as u8;
            used |= 1 << node;
        }
        for (index, output) in table.iter().enumerate() {
            let Value::Bool(output) = output else {
                return Err("A truth table may contain only Boolean entries".into());
            };
            parsed.table |= (*output as u64) << index;
        }
        Ok(parsed)
    }

    fn changes(&self, state: u16, node: usize) -> bool {
        // inputs[0] is the low bit of the truth-table index.
        let mut index = 0u16;
        for (offset, &source) in self.input_indices[..self.input_count].iter().enumerate() {
            index |= ((state >> source) & 1) << offset;
        }
        ((self.table >> index) & 1) != u64::from((state >> node) & 1)
    }
}

struct Workspace {
    finish_order: [u16; MAX_STATES],
    stack: [u16; MAX_STATES],
    next_node: [u8; MAX_STATES],
    component: [u16; MAX_STATES],
    terminal: [bool; MAX_STATES],
}

struct Summary {
    components: usize,
    transitions: usize,
    fixed_points: usize,
    attractors: usize,
}

impl Workspace {
    fn new() -> Self {
        Self {
            finish_order: [0; MAX_STATES],
            stack: [0; MAX_STATES],
            next_node: [0; MAX_STATES],
            component: [UNSEEN; MAX_STATES],
            terminal: [true; MAX_STATES],
        }
    }

    fn solve(&mut self, rules: &[Rule], cancel: &AtomicBool) -> Result<Summary, &'static str> {
        let states = 1u16 << rules.len();
        self.component[..usize::from(states)].fill(UNSEEN);
        self.finish_forward(rules, states, cancel)?;
        let components = self.assign_reverse(rules, states, cancel)?;
        if self.component[..usize::from(states)]
            .iter()
            .any(|&id| usize::from(id) >= components)
        {
            return Err("Boolean SCC traversal did not assign every state");
        }
        self.terminal[..components].fill(true);
        let mut transitions = 0;
        let mut fixed_points = 0;
        for state in 0..states {
            check_cancel(cancel)?;
            let source = self.component[usize::from(state)];
            let previous_count = transitions;
            for (node, rule) in rules.iter().enumerate() {
                if rule.changes(state, node) {
                    let successor = state ^ (1 << node);
                    let target = self.component[usize::from(successor)];
                    if source != target {
                        // The reverse traversal numbers components in forward
                        // topological order, so every crossing edge points up.
                        if target < source {
                            return Err(
                                "Boolean SCC traversal produced an invalid component order",
                            );
                        }
                        self.terminal[usize::from(source)] = false;
                    }
                    transitions += 1;
                }
            }
            if transitions == previous_count {
                fixed_points += 1;
            }
        }
        let attractors = self.terminal[..components]
            .iter()
            .filter(|&&terminal| terminal)
            .count();
        Ok(Summary {
            components,
            transitions,
            fixed_points,
            attractors,
        })
    }

    fn finish_forward(
        &mut self,
        rules: &[Rule],
        states: u16,
        cancel: &AtomicBool,
    ) -> Result<(), &'static str> {
        let mut finished = 0;
        for root in 0..states {
            if self.component[usize::from(root)] != UNSEEN {
                continue;
            }
            self.component[usize::from(root)] = UNASSIGNED;
            self.stack[0] = root;
            self.next_node[0] = 0;
            let mut depth = 1;
            while depth > 0 {
                let frame = depth - 1;
                let state = self.stack[frame];
                let node = usize::from(self.next_node[frame]);
                check_cancel(cancel)?;
                if node == rules.len() {
                    self.finish_order[finished] = state;
                    finished += 1;
                    depth -= 1;
                    continue;
                }
                self.next_node[frame] += 1;
                let successor = state ^ (1 << node);
                if self.component[usize::from(successor)] == UNSEEN
                    && rules[node].changes(state, node)
                {
                    self.component[usize::from(successor)] = UNASSIGNED;
                    self.stack[depth] = successor;
                    self.next_node[depth] = 0;
                    depth += 1;
                }
            }
        }
        Ok(())
    }

    fn assign_reverse(
        &mut self,
        rules: &[Rule],
        states: u16,
        cancel: &AtomicBool,
    ) -> Result<usize, &'static str> {
        let mut component = 0u16;
        for &root in self.finish_order[..usize::from(states)].iter().rev() {
            if self.component[usize::from(root)] != UNASSIGNED {
                continue;
            }
            self.component[usize::from(root)] = component;
            self.stack[0] = root;
            let mut depth = 1;
            while depth > 0 {
                check_cancel(cancel)?;
                depth -= 1;
                let state = self.stack[depth];
                for (node, rule) in rules.iter().enumerate() {
                    // The only possible predecessor along this node's update
                    // differs at this bit, and must enable the update itself.
                    let predecessor = state ^ (1 << node);
                    if self.component[usize::from(predecessor)] == UNASSIGNED
                        && rule.changes(predecessor, node)
                    {
                        self.component[usize::from(predecessor)] = component;
                        self.stack[depth] = predecessor;
                        depth += 1;
                    }
                }
            }
            component += 1;
        }
        Ok(usize::from(component))
    }
}

fn check_cancel(cancel: &AtomicBool) -> Result<(), &'static str> {
    if cancel.load(Ordering::Acquire) {
        Err("Production cancelled")
    } else {
        Ok(())
    }
}

impl AssetHost<'_> {
    pub(crate) fn solve_grn_epigenetic_morphogenesis(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["rules", "arena_budget_bytes"])?;
        let authored = list(required(args, "rules")?)?;
        let nodes = authored.len();
        if !(1..=MAX_NODES).contains(&nodes) {
            return Err("The finite Boolean solver requires 1 through 12 numbered nodes".into());
        }
        let states = 1usize << nodes;
        budget(args, WORKSPACE_BYTES)?;
        let table_bytes = (states + 1) * COLUMNS.len() * 64;
        self.charge(
            states * nodes * 3 * (MAX_INPUTS + 1) * 8,
            WORKSPACE_BYTES + table_bytes,
        )?;
        let mut rules = [Rule::EMPTY; MAX_NODES];
        for (rule, value) in rules.iter_mut().zip(authored) {
            *rule = Rule::parse(value, nodes)?;
        }
        let rules = &rules[..nodes];
        let mut workspace = Workspace::new();
        let summary = workspace.solve(rules, self.cancel)?;

        let mut digest = Sha256::new();
        digest.update(CONTEXT.as_bytes());
        digest.update([nodes as u8]);
        for rule in rules {
            digest.update([rule.input_count as u8]);
            digest.update(&rule.input_indices[..rule.input_count]);
            digest.update(rule.table.to_le_bytes());
        }
        let mut book = Workbook::new();
        let sheet_id = book.add_sheet("Asynchronous Boolean states");
        let sheet = book
            .sheet_mut(sheet_id)
            .ok_or("Cannot create the Boolean-state worksheet")?;
        for (column, name) in COLUMNS.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::Text(name.into()));
        }
        for state in 0..states {
            check_cancel(self.cancel)?;
            let component = workspace.component[state];
            let terminal = workspace.terminal[usize::from(component)];
            digest.update(component.to_le_bytes());
            digest.update([u8::from(terminal)]);
            let row = state as u32 + 1;
            sheet.set_value(CellRef::new(row, 0), CellValue::Number(state as f64));
            sheet.set_value(
                CellRef::new(row, 1),
                CellValue::Number(f64::from(component)),
            );
            sheet.set_value(CellRef::new(row, 2), CellValue::Bool(terminal));
        }

        let graph_iri = format!("urn:sha256:{:x}", digest.finalize());
        let mut graph = ReceiptGraph::new(CONTEXT);
        graph.fact(&graph_iri, "urn:qualia:math:solver-model", CONTEXT)?;
        for (predicate, count) in [
            ("urn:qualia:math:state-count", states),
            ("urn:qualia:math:transition-count", summary.transitions),
            ("urn:qualia:math:component-count", summary.components),
            ("urn:qualia:math:attractor-count", summary.attractors),
            ("urn:qualia:math:fixed-point-count", summary.fixed_points),
            ("urn:qualia:math:fixed-workspace-bytes", WORKSPACE_BYTES),
        ] {
            graph.fact(
                &graph_iri,
                predicate,
                &format!("urn:qualia:integer:{count}"),
            )?;
        }
        let sealed = graph.seal(self.limits.resource_bytes)?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let receipt = self.encode("q42", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("node_count".into(), count_value(nodes)),
            ("state_count".into(), count_value(states)),
            ("transition_count".into(), count_value(summary.transitions)),
            ("component_count".into(), count_value(summary.components)),
            ("attractor_count".into(), count_value(summary.attractors)),
            (
                "fixed_point_count".into(),
                count_value(summary.fixed_points),
            ),
        ])))
    }
}

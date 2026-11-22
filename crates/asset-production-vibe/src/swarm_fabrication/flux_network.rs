//! Exact integral steady-state flow on a bounded directed incidence network.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod exchange;

use crate::{
    fields, integer, list, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    record, required, text, Args, AssetHost, TypedArtifact, Value,
};
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::domains::geospatial::gis_gp::{gp_max_flow_edmonds_karp, NetworkEdge};
use sha2::{Digest, Sha256};
use std::sync::Arc;

const MAX_NODES: usize = 48;
const MAX_EDGES: usize = 96;
const MAX_CAPACITY: usize = 1_000_000_000;
const MODEL_CONTEXT: &str = "urn:qualia:contexts:directed-incidence-network-flow-v1";

impl AssetHost<'_> {
    pub(crate) fn invert_flux_balance_and_rates(&mut self, args: &Args) -> Result<Value, String> {
        match text(required(args, "op")?)? {
            "directed_incidence_max_flow" => self.invert_directed_incidence_flow(args),
            "inverse_normalized_exchange" => self.inverse_normalized_exchange(args),
            _ => {
                Err("Expected op directed_incidence_max_flow or inverse_normalized_exchange".into())
            }
        }
    }

    fn invert_directed_incidence_flow(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "node_count", "source", "sink", "edges"])?;
        let nodes = integer(required(args, "node_count")?, "node_count")?;
        let source = integer(required(args, "source")?, "source")?;
        let sink = integer(required(args, "sink")?, "sink")?;
        if !(2..=MAX_NODES).contains(&nodes) || source >= nodes || sink >= nodes || source == sink {
            return Err(format!(
                "Flow requires 2 through {MAX_NODES} nodes and distinct source and sink indices"
            ));
        }
        let edges = list(required(args, "edges")?)?;
        if edges.len() > MAX_EDGES {
            return Err(format!("Flow supports at most {MAX_EDGES} directed edges"));
        }
        let size = edges.len().max(1);
        let scratch = 32_768 + (nodes + size) * 256;
        let receipt_budget = self.limits.resource_bytes.min(1024 * 1024);
        // Each edge can require an additional native solve. Edmonds–Karp's
        // augment count is bounded by O(VE), with O(V+E) work per augment.
        self.charge(
            (edges.len() + 1) * (nodes + 2) * size * (nodes + 2 * size),
            scratch.max(receipt_budget),
        )?;
        let mut native = Vec::with_capacity(edges.len());
        let mut upper = Vec::with_capacity(edges.len());
        let mut evidence = Sha256::new();
        evidence.update(b"qualia.directed-incidence-network-flow/v1");
        for value in [nodes, source, sink, edges.len()] {
            evidence.update((value as u32).to_le_bytes());
        }
        for (i, row) in edges.iter().enumerate() {
            let row = record(row)?;
            fields(row, &["from", "to", "capacity"])?;
            let from = integer(required(row, "from")?, "from")?;
            let to = integer(required(row, "to")?, "to")?;
            let capacity = integer(required(row, "capacity")?, "capacity")?;
            if from >= nodes || to >= nodes || from == to || capacity > MAX_CAPACITY {
                return Err(format!(
                    "Invalid endpoints or capacity at directed edge {i}"
                ));
            }
            for value in [from, to, capacity] {
                evidence.update((value as u32).to_le_bytes());
            }
            native.push(NetworkEdge {
                id: i as u32,
                from,
                to,
                base_cost: 0.0,
                capacity: capacity as f64,
                one_way: true,
            });
            upper.push(capacity as u64);
        }
        let evidence = format!("urn:qualia:network:sha256:{:x}", evidence.finalize());
        let solve = |edges: &[NetworkEdge]| {
            gp_max_flow_edmonds_karp(nodes, edges, source, sink, scratch)
                .map_err(|e| format!("Native network-flow solve: {e}"))
        };
        let total_capacity: u64 = upper.iter().sum();
        let (objective, cut) = solve(&native)?;
        let objective = exact_flow(objective, total_capacity)?;
        let mut source_side = vec![false; nodes];
        if !cut.contains(&source) || cut.contains(&sink) {
            return Err("Native max-flow result has an invalid source–sink cut".into());
        }
        for &node in &cut {
            let side = source_side
                .get_mut(node)
                .ok_or("Native cut node is out of range")?;
            *side = true;
        }
        let mut cut_capacity = 0;
        for (edge, &capacity) in native.iter().zip(&upper) {
            if source_side[edge.from] && !source_side[edge.to] {
                cut_capacity += capacity;
            }
        }
        if cut_capacity != objective {
            return Err("Native max-flow objective and cut capacity do not agree".into());
        }

        let mut flux = vec![0; native.len()];
        let mut balance = vec![0i64; nodes];
        for (i, &capacity) in upper.iter().enumerate() {
            if capacity == 0 || objective == 0 {
                continue;
            }
            self.charge(0, 0)?;
            native[i].capacity = 0.0;
            let without = exact_flow(solve(&native)?.0, objective)?;
            // For this single directed edge, every cut capacity either stays
            // fixed or increases by the edge's capacity. Its least admissible
            // capacity preserving the maximum is therefore objective - without.
            // Once minimized, all remaining maximum flows saturate this edge.
            let flow = objective - without;
            if flow > capacity {
                return Err(format!(
                    "Recovered flux exceeds the capacity at directed edge {i}"
                ));
            }
            native[i].capacity = flow as f64;
            flux[i] = flow;
            balance[native[i].from] += flow as i64;
            balance[native[i].to] -= flow as i64;
        }
        for (node, net_outflow) in balance.into_iter().enumerate() {
            let expected = if node == source {
                objective as i64
            } else if node == sink {
                -(objective as i64)
            } else {
                0
            };
            if net_outflow != expected {
                return Err(format!(
                    "Recovered flux violates conservation at node {node}"
                ));
            }
        }

        let mut table = Workbook::new();
        let id = table.add_sheet("Directed flux");
        let sheet = table.sheet_mut(id).ok_or("Missing directed flux sheet")?;
        for (column, label) in ["Edge index", "From", "To", "Capacity", "Flux"]
            .into_iter()
            .enumerate()
        {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::text(label));
        }
        let mut receipt = ReceiptGraph::new(MODEL_CONTEXT);
        for (predicate, value) in [
            ("nodeCount", nodes as u64),
            ("totalFlux", objective),
            ("cutCapacity", cut_capacity),
        ] {
            integer_fact(&mut receipt, &evidence, predicate, value)?;
        }
        for (predicate, node) in [("source", source), ("sink", sink)] {
            receipt.fact(
                &evidence,
                &relation(predicate),
                &format!("{evidence}:node:{node}"),
            )?;
        }
        for &node in &cut {
            receipt.fact(
                &evidence,
                &relation("sourceSide"),
                &format!("{evidence}:node:{node}"),
            )?;
        }
        for (i, edge) in native.iter().enumerate() {
            let row = i as u32 + 1;
            for (column, value) in [
                i as u64,
                edge.from as u64,
                edge.to as u64,
                upper[i],
                flux[i],
            ]
            .into_iter()
            .enumerate()
            {
                sheet.set_value(
                    CellRef::new(row, column as u32),
                    CellValue::Number(value as f64),
                );
            }
            let edge_iri = format!("{evidence}:edge:{i}");
            receipt.fact(&evidence, &relation("edge"), &edge_iri)?;
            for (predicate, node) in [("from", edge.from), ("to", edge.to)] {
                receipt.fact(
                    &edge_iri,
                    &relation(predicate),
                    &format!("{evidence}:node:{node}"),
                )?;
            }
            integer_fact(&mut receipt, &edge_iri, "capacity", upper[i])?;
            integer_fact(&mut receipt, &edge_iri, "flux", flux[i])?;
        }
        let facts = receipt.seal(receipt_budget)?;
        let cube = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(table)))?;
        let receipt = self.admit_encoded("q42", facts)?;
        Ok(Value::Record(
            [
                ("cube".into(), cube),
                ("receipt".into(), receipt),
                ("max_flow".into(), Value::U64(objective)),
                ("cut_capacity".into(), Value::U64(cut_capacity)),
                (
                    "source_side".into(),
                    Value::List(
                        cut.into_iter()
                            .map(|node| Value::U64(node as u64))
                            .collect(),
                    ),
                ),
            ]
            .into(),
        ))
    }
}

fn exact_flow(value: f64, upper: u64) -> Result<u64, String> {
    // Total admitted capacity is below 2^53, so integral f64 flow is exact.
    if !(0.0..=upper as f64).contains(&value) || value.fract() != 0.0 {
        return Err("Native max-flow result violates the integral capacity bounds".into());
    }
    Ok(value as u64)
}

fn relation(name: &str) -> String {
    format!("urn:qualia:incidence-flow:{name}")
}

fn integer_fact(
    graph: &mut ReceiptGraph,
    subject: &str,
    predicate: &str,
    value: u64,
) -> Result<(), String> {
    graph.fact(
        subject,
        &relation(predicate),
        &format!("urn:qualia:integer:{value}"),
    )
}

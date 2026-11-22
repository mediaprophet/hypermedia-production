//! Closed, abstract compartment transport using the native conservative exchange kernel.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{
    NQuin,
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::multiscale_strata_section::{
        MultilingualLexicalTerm, SymbolicRdfCborLdPayload, encode_symbolic_rdf_cbor_ld,
    },
    domains::biosphere::funga_network::{
        HyphalEdge, HyphalNode, METADATA_PROVENANCE_SIMULATION,
        step_mycelial_nutrient_transport_zero_heap,
    },
    foundation::frame_layout::pack_float_object,
    q_hash,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Value, fields, integer, list, record, required, text, vector};

use super::{bounded, profile_result};

const CONSERVATION_RTOL: f64 = 2.0e-6;
const CHANNELS: [&str; 3] = ["pool_0", "pool_1", "pool_2"];

struct Link {
    endpoints: [usize; 2],
    fraction: f32,
    net_flow: [f64; 3],
}

impl AssetHost<'_> {
    pub(crate) fn step_funga_and_trophic_web(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "compartments", "links", "rounds"])?;
        if text(required(args, "mode")?)? != "closed_compartment_exchange" {
            return Err("Expected closed_compartment_exchange mode".into());
        }
        let inputs = list(required(args, "compartments")?)?;
        let links = list(required(args, "links")?)?;
        let rounds = integer(required(args, "rounds")?, "rounds")?;
        if !(2..=128).contains(&inputs.len())
            || !(1..=512).contains(&links.len())
            || !(1..=128).contains(&rounds)
        {
            return Err(
                "Closed exchange requires 2..=128 compartments, 1..=512 links and 1..=128 rounds"
                    .into(),
            );
        }
        self.charge(
            rounds * (inputs.len() * 16 + links.len() * 96),
            inputs.len() * 384 + links.len() * 2560,
        )?;
        let mut hash = Sha256::new();
        hash.update(b"closed_compartment_exchange:v1");
        hash.update((inputs.len() as u32).to_le_bytes());
        hash.update((links.len() as u32).to_le_bytes());
        hash.update((rounds as u32).to_le_bytes());
        let mut nodes = Vec::with_capacity(inputs.len());
        for (i, input) in inputs.iter().enumerate() {
            let pools = vector::<3>(input)?;
            if pools.iter().any(|v| !(0.0..=1.0).contains(v)) {
                return Err("Each pool must be a dimensionless quantity within [0, 1]".into());
            }
            let [carbon_mol_m3, phosphorus_mol_m3, nitrogen_mol_m3] = pools.map(|v| v as f32);
            for v in pools {
                hash.update(v.to_le_bytes());
            }
            // Only the kernel's exchange arithmetic is used. Pool labels and
            // the timestep represent anonymous unitless graph state.
            nodes.push(HyphalNode {
                node_id: i as u64 + 1,
                substrate_moisture: 0.0,
                carbon_mol_m3,
                phosphorus_mol_m3,
                nitrogen_mol_m3,
            });
        }
        let mut unique = BTreeSet::new();
        let mut edges = Vec::with_capacity(links.len());
        for item in links {
            let row = record(item)?;
            fields(row, &["a", "b", "mixing_fraction"])?;
            let mut endpoints = [
                integer(required(row, "a")?, "a")?,
                integer(required(row, "b")?, "b")?,
            ];
            endpoints.sort_unstable();
            if endpoints[0] == endpoints[1]
                || endpoints[1] >= nodes.len()
                || !unique.insert(endpoints)
            {
                return Err(
                    "Each link must be a unique pair of distinct compartment indices".into(),
                );
            }
            let fraction = bounded(row, "mixing_fraction", 0.0, 0.5)? as f32;
            edges.push(Link {
                endpoints,
                fraction,
                net_flow: [0.0; 3],
            });
        }
        edges.sort_unstable_by_key(|edge| edge.endpoints);
        for link in &edges {
            for index in link.endpoints {
                hash.update((index as u32).to_le_bytes());
            }
            hash.update(link.fraction.to_le_bytes());
        }
        let digest = format!("{:x}", hash.finalize());
        let initial = totals(&nodes);
        let mut max_residual = [0.0_f64; 3];
        for _ in 0..rounds {
            self.charge(0, 0)?;
            for link in &mut edges {
                let [a, b] = link.endpoints;
                let mut pair = [nodes[a], nodes[b]];
                let from_before = pools(&pair[0]);
                let to_before = pools(&pair[1]);
                step_mycelial_nutrient_transport_zero_heap(
                    &mut pair,
                    &[HyphalEdge {
                        from_idx: 0,
                        to_idx: 1,
                        conductance: link.fraction,
                        anastomosis_fused: false,
                    }],
                    1.0,
                )
                .map_err(|error| format!("Native compartment exchange: {error}"))?;
                let from_after = pools(&pair[0]);
                let to_after = pools(&pair[1]);
                for i in 0..3 {
                    let debit = f64::from(from_before[i]) - f64::from(from_after[i]);
                    let credit = f64::from(to_after[i]) - f64::from(to_before[i]);
                    link.net_flow[i] += 0.5 * (debit + credit);
                }
                [nodes[a], nodes[b]] = pair;
            }
            let current = totals(&nodes);
            for i in 0..3 {
                max_residual[i] = max_residual[i].max((current[i] - initial[i]).abs());
                if !current[i].is_finite()
                    || max_residual[i] > CONSERVATION_RTOL * initial[i].max(1.0)
                {
                    return Err(format!(
                        "Closed exchange conservation check failed for pool {i}"
                    ));
                }
            }
        }
        let context = format!("urn:qualia:earth:compartment-exchange:{digest}");
        let graph_bytes = flux_graph(self, &context, &edges)?;
        let graph_hash = format!("{:x}", Sha256::digest(&graph_bytes));
        let samples = nodes.iter().flat_map(pools).collect();
        let final_totals = totals(&nodes);
        let report = json!({
            "mode": "closed_compartment_exchange", "unit": "UNITLESS", "axes": ["compartment", "pool"],
            "channels": CHANNELS, "rounds": rounds, "compartment_count": nodes.len(), "link_count": edges.len(),
            "initial_totals": initial, "final_totals": final_totals, "maximum_absolute_conservation_residual": max_residual,
            "relative_conservation_tolerance": CONSERVATION_RTOL, "input_sha256": digest,
            "flux_graph_iri": context, "flux_graph_sha256": graph_hash,
            "edge_update_order": "ascending canonical endpoint pair", "flux_sign_convention": "positive from lower to higher index",
        });
        let result = profile_result(
            self,
            "closed-compartment-exchange",
            vec![nodes.len() as u32, 3],
            samples,
            report,
        )?;
        let mut output = record(&result)?.clone();
        output.insert("flux_q42".into(), self.encode("q42", graph_bytes)?);
        Ok(Value::Record(output))
    }
}

fn pools(node: &HyphalNode) -> [f32; 3] {
    [
        node.carbon_mol_m3,
        node.phosphorus_mol_m3,
        node.nitrogen_mol_m3,
    ]
}

fn totals(nodes: &[HyphalNode]) -> [f64; 3] {
    nodes.iter().fold([0.0; 3], |sum, node| {
        let quantities = pools(node);
        std::array::from_fn(|i| sum[i] + f64::from(quantities[i]))
    })
}

fn flux_graph(host: &AssetHost<'_>, context: &str, links: &[Link]) -> Result<Vec<u8>, String> {
    const FLOW: [&str; 3] = [
        "urn:qualia:earth:conservation:net-pool-0",
        "urn:qualia:earth:conservation:net-pool-1",
        "urn:qualia:earth:conservation:net-pool-2",
    ];
    const FIRST: &str = "urn:qualia:earth:conservation:first-compartment";
    const SECOND: &str = "urn:qualia:earth:conservation:second-compartment";
    const METADATA: u64 = (1 << 61) | METADATA_PROVENANCE_SIMULATION;
    let mut dict = BTreeMap::new();
    let mut register = |iri: &str| -> Result<u64, String> {
        let hash = q_hash(iri);
        if hash == 0 {
            return Err("Compartment Q42 term has a zero hash".into());
        }
        match dict.entry(hash) {
            std::collections::btree_map::Entry::Occupied(entry) if entry.get() == iri => {}
            std::collections::btree_map::Entry::Occupied(_) => {
                return Err("Compartment Q42 term hash collision".into());
            }
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(iri.to_owned());
            }
        }
        Ok(hash)
    };
    let graph = register(context)?;
    let p_first = register(FIRST)?;
    let p_second = register(SECOND)?;
    let p_flow = [register(FLOW[0])?, register(FLOW[1])?, register(FLOW[2])?];
    let mut quins = Vec::with_capacity(links.len() * 5);
    for link in links {
        let [a, b] = link.endpoints;
        let edge = register(&format!("{context}:link:{a}:{b}"))?;
        let first = register(&format!("{context}:compartment:{a}"))?;
        let second = register(&format!("{context}:compartment:{b}"))?;
        let mut append = |predicate, object| {
            quins.push([
                edge,
                predicate,
                object,
                graph,
                METADATA,
                NQuin::calculate_parity(edge, predicate, object, graph, METADATA),
            ]);
        };
        append(p_first, first);
        append(p_second, second);
        for (predicate, flow) in p_flow.into_iter().zip(link.net_flow) {
            append(predicate, pack_float_object(flow as f32));
        }
    }
    let data = SymbolicRdfCborLdPayload {
        context_iri: context.to_owned(),
        nquins: quins,
        lexicon: dict
            .into_iter()
            .map(|(term_hash, iri_utf8)| MultilingualLexicalTerm {
                term_hash,
                iri_utf8,
                lang_bcp47: None,
                label_utf8: String::new(),
            })
            .collect(),
        ..Default::default()
    };
    let cbld = encode_symbolic_rdf_cbor_ld(&data).map_err(|error| error.to_string())?;
    SemanticModelBuilder::new(host.limits.resource_bytes.min(4 * 1024 * 1024))
        .merge_cbld(&cbld)
        .and_then(SemanticModelBuilder::finish)
        .map_err(|error| error.to_string())
}

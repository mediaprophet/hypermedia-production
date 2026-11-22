//! Fixed closed directed exchange with the native parity-protected flux encoder.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use bytemuck::Zeroable;
use qualia_core_db::{
    NQuin,
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::multiscale_strata_section::{
        MultilingualLexicalTerm, SymbolicRdfCborLdPayload, encode_symbolic_rdf_cbor_ld,
    },
    domains::biosphere::ecological_coupling::{
        EcologicalCouplingSpec, METADATA_PROVENANCE_SIMULATION, PRED_TROPHIC_EXCHANGE_FLUX,
        TrophicCouplingKind, evaluate_ecological_coupling_zero_heap,
    },
    foundation::frame_layout::{INLINE_TAG_FLOAT, object_tag, unpack_float_object},
    q_hash,
};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Value, fields, record, required, text};

use super::profile_result;

const FIXTURE_MODE: &str = "synthetic_directed_flux_receipt";
const CONTEXT: &str = "urn:qualia:earth:synthetic-directed-flux:v1";
const PREDICATE: &str = "qualia:biosphere#trophicExchangeFlux";
const COMPARTMENTS: [&str; 4] = [
    "urn:qualia:earth:synthetic-directed-flux:v1:a",
    "urn:qualia:earth:synthetic-directed-flux:v1:b",
    "urn:qualia:earth:synthetic-directed-flux:v1:c",
    "urn:qualia:earth:synthetic-directed-flux:v1:d",
];
const LINKS: [[usize; 2]; 4] = [[0, 1], [1, 2], [2, 3], [3, 0]];

impl AssetHost<'_> {
    pub(crate) fn step_funga_microbial_trophic_coupling(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["mode"])?;
        if text(required(args, "mode")?)? != FIXTURE_MODE {
            return Err("Expected the fixed synthetic_directed_flux_receipt fixture".into());
        }
        self.charge(262_144, 512 * 1024)?;
        let hashes = COMPARTMENTS.map(q_hash);
        let initial: [f64; 4] = [1.0, 0.4, 0.2, 0.6];
        let coupling = |at: usize, base: f32, window: [f32; 2], active| {
            let [source, target] = LINKS[at];
            EcologicalCouplingSpec {
                source_entity_did_hash: hashes[source],
                target_entity_did_hash: hashes[target],
                // The native variant supplies a compatibility tag. Its model
                // label is not emitted as a claim about the abstract nodes.
                coupling_kind: TrophicCouplingKind::SoilMoistureCanopyTranspiration,
                flux_rate: base,
                min_soil_moisture: window[0],
                max_soil_moisture: window[1],
                temperature_optimum_c: 0.0,
                is_simulation: true,
                active,
            }
        };
        let specs = [
            coupling(0, 0.18, [0.25, 1.0], true),
            coupling(1, 0.15, [0.25, 0.75], true),
            coupling(2, 0.10, [0.0, 1.0], false),
            coupling(3, 0.10, [0.65, 0.90], true),
        ];
        let mut native = [NQuin::zeroed(); LINKS.len() * 2];
        let count = evaluate_ecological_coupling_zero_heap(&specs, 0.5, &mut native)
            .map_err(|error| format!("Native synthetic directed exchange: {error}"))?;
        if count != native.len() || q_hash(PREDICATE) != PRED_TROPHIC_EXCHANGE_FLUX {
            return Err("Native exchange failed its canonical receipt layout".into());
        }

        let mut debits = [0.0; COMPARTMENTS.len()];
        let mut credits = debits;
        let mut output_rows = Vec::with_capacity(LINKS.len());
        let mut report = Vec::with_capacity(LINKS.len());
        for (lane, ([source, target], pair)) in
            LINKS.into_iter().zip(native.chunks_exact(2)).enumerate()
        {
            if pair.iter().any(|row| {
                !row.verify_ecc_parity() || row.metadata & METADATA_PROVENANCE_SIMULATION == 0
            }) {
                return Err(
                    "Native exchange receipt is missing parity or simulation provenance".into(),
                );
            }
            let row = pair[1];
            if row.subject != hashes[source]
                || row.context != hashes[target]
                || row.predicate != PRED_TROPHIC_EXCHANGE_FLUX
                || object_tag(row.object) != INLINE_TAG_FLOAT
            {
                return Err(
                    "Native exchange receipt does not identify the expected directed link".into(),
                );
            }
            let flux = f64::from(unpack_float_object(row.object));
            if !flux.is_finite() || flux < 0.0 || (flux > 0.0) != (lane < 2) {
                return Err(
                    "Native exchange transfers do not match the fixture's active and gated links"
                        .into(),
                );
            }
            debits[source] += flux;
            credits[target] += flux;
            report.push(json!({"source": COMPARTMENTS[source], "destination": COMPARTMENTS[target], "transfer": flux}));
            output_rows.push(columns(&row));
        }
        if debits
            .iter()
            .zip(initial)
            .any(|(&debit, available)| debit > available)
        {
            return Err("Native exchange would overdraw an abstract compartment".into());
        }
        let final_pools: [f64; COMPARTMENTS.len()] =
            std::array::from_fn(|i| initial[i] - debits[i] + credits[i]);
        let residual = (initial.iter().sum::<f64>() - final_pools.iter().sum::<f64>()).abs();
        if residual > 1e-12
            || final_pools
                .iter()
                .any(|&value| value < 0.0 || !value.is_finite())
        {
            return Err("Synthetic directed exchange failed conservation".into());
        }
        let budget = self.limits.resource_bytes.min(1024 * 1024);
        let graph = flux_graph(output_rows, budget)?;
        let graph_hash = format!("{:x}", Sha256::digest(&graph));
        let samples = initial
            .into_iter()
            .zip(final_pools)
            .flat_map(|(before, after)| [before as f32, after as f32])
            .collect();
        let response = profile_result(
            self,
            "synthetic-directed-flux",
            vec![COMPARTMENTS.len() as u32, 2],
            samples,
            json!({
                "mode": FIXTURE_MODE, "unit": "UNITLESS", "axes": ["abstract_compartment", "state"],
                "channels": ["initial_pool", "final_pool"], "compartments": COMPARTMENTS, "transfers": report,
                "conservation_residual": residual, "source_engine": "evaluate_ecological_coupling_zero_heap",
                "simulation_provenance_bit": 28, "flux_graph_sha256": graph_hash, "flux_graph_iri": CONTEXT,
            }),
        )?;
        let mut output = record(&response)?.clone();
        output.insert("flux_q42".into(), self.encode("q42", graph)?);
        Ok(Value::Record(output))
    }
}

fn columns(row: &NQuin) -> [u64; 6] {
    [
        row.subject,
        row.predicate,
        row.object,
        row.context,
        row.metadata,
        row.parity,
    ]
}

fn flux_graph(rows: Vec<[u64; 6]>, budget: usize) -> Result<Vec<u8>, String> {
    let mut dict = BTreeMap::new();
    for iri in COMPARTMENTS.into_iter().chain([CONTEXT, PREDICATE]) {
        let hash = q_hash(iri);
        if hash == 0 || dict.insert(hash, iri).is_some() {
            return Err("Synthetic directed exchange has a nonbijective lexical binding".into());
        }
    }
    let mut expected = rows.clone();
    let payload = SymbolicRdfCborLdPayload {
        context_iri: CONTEXT.to_owned(),
        nquins: rows,
        lexicon: dict
            .into_iter()
            .map(|(term_hash, iri)| MultilingualLexicalTerm {
                term_hash,
                iri_utf8: iri.to_owned(),
                lang_bcp47: None,
                label_utf8: String::new(),
            })
            .collect(),
        ..Default::default()
    };
    let cbld = encode_symbolic_rdf_cbor_ld(&payload).map_err(|error| error.to_string())?;
    let graph = SemanticModelBuilder::new(budget)
        .merge_cbld(&cbld)
        .and_then(SemanticModelBuilder::finish)
        .map_err(|error| format!("Sealing native directed flux Q42: {error}"))?;
    let admitted = SemanticModelBuilder::new(budget)
        .merge_q42(&graph)
        .map_err(|error| format!("Re-admitting native directed flux Q42: {error}"))?;
    let mut actual = admitted
        .snapshot_quins()
        .iter()
        .map(columns)
        .collect::<Vec<_>>();
    actual.sort_unstable();
    expected.sort_unstable();
    if actual != expected {
        return Err("Native directed flux rows did not round-trip through Q42".into());
    }
    Ok(graph)
}

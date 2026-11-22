//! Explicit redaction, integrity verification, and accessibility assessment adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod accessibility;
mod credential;
mod lineage;
mod optical;
mod pose;
mod quorum;
mod redaction;
mod timestamp;
mod wellfare_fixture;

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{AssetHost, Resource, Value};
use sha2::{Digest, Sha256};

const REVIEW_CONTEXT: &str = "urn:qualia:privacy-provenance-review:v1";
const REVIEW_NS: &str = "https://qualiadb.org/ontology/production-review#";

fn review_graph(mode: &str) -> Result<ReceiptGraph, String> {
    let mut graph = ReceiptGraph::new(REVIEW_CONTEXT);
    review_fact(&mut graph, "assessment", &format!("{REVIEW_NS}{mode}"))?;
    Ok(graph)
}

fn review_fact(graph: &mut ReceiptGraph, predicate: &str, object: &str) -> Result<(), String> {
    graph.fact(REVIEW_CONTEXT, &format!("{REVIEW_NS}{predicate}"), object)
}

fn review_number(graph: &mut ReceiptGraph, predicate: &str, value: usize) -> Result<(), String> {
    review_fact(graph, predicate, &format!("urn:qualia:count:{value}"))
}

fn retained<'host>(host: &'host AssetHost<'_>, value: &Value) -> Result<&'host Resource, String> {
    match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|input| input.key == *key)
            .ok_or_else(|| "Unknown retained review input".to_owned()),
        _ => host.workspace.encoded(value),
    }
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn digest_iri(digest: &[u8; 32]) -> String {
    format!("urn:sha256:{}", encode_hex(digest))
}

fn encode_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        encoded.push(char::from(HEX[(byte >> 4) as usize]));
        encoded.push(char::from(HEX[(byte & 15) as usize]));
    }
    encoded
}

fn parse_hex<const N: usize>(value: &str, name: &str) -> Result<[u8; N], String> {
    if value.len() != N * 2 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{name} requires {} hexadecimal characters", N * 2));
    }
    let mut bytes = [0; N];
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&value[index * 2..index * 2 + 2], 16)
            .map_err(|_| format!("Invalid hexadecimal character in {name}"))?;
    }
    Ok(bytes)
}

impl AssetHost<'_> {
    fn admit_review(&mut self, graph: ReceiptGraph) -> Result<Value, String> {
        self.admit_encoded(
            "q42",
            graph.seal(self.limits.resource_bytes.min(1024 * 1024))?,
        )
    }
}

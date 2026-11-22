//! Offline Solid LDP, QDNF transition-envelope, and semantic corpus operations.

use super::{external_resource, fields, required, text, Args, AssetHost, ReceiptGraph, Value, SYMBOLIC_LIMIT};
use crate::record;
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    net::qdnf::{
        bearer::{decapsulate_nym, encapsulate_nym, NYM_HEADER_LEN, NYM_SPHINX_MTU},
        frame::decode_frame,
        types::ObservedLocator,
    },
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

mod corpus;

const GRAPH: &str = "https://qualiadb.org/ontology/q42#OfflineAdmissionReceipt";
const TURTLE_SOURCE_LIMIT: usize = 1024 * 1024;
const ENVELOPE_LIMIT: usize = NYM_SPHINX_MTU as usize;

impl AssetHost<'_> {
    pub(crate) fn sync_solid_pod_and_qdnf_peer(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let params = record(required(args, "parameters")?)?;
        match op {
            "solid_turtle_import" => self.import_turtle(params),
            "qdnf_nym_envelope" | "qdnf_nym_unwrap" => self.nym_packet(op, params),
            "corpus_rank" => corpus::rank(self, params),
            _ => Err(format!("Unsupported offline network operation: {op}")),
        }
    }

    fn import_turtle(&mut self, params: &Args) -> Result<Value, String> {
        fields(params, &["source", "source_iri"])?;
        let input = required(params, "source")?;
        let source_iri = text(required(params, "source_iri")?)?;
        if source_iri.len() > 2048 {
            return Err("Solid source IRI exceeds the input byte budget".into());
        }
        let source = external_resource(self, input)?;
        if !matches!(source.kind.as_str(), "ttl" | "turtle" | "text/turtle")
            || source.bytes.len() > TURTLE_SOURCE_LIMIT
        {
            return Err("Solid LDP input requires bounded Turtle bytes".into());
        }
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        let source_bytes = source.bytes.len();
        self.charge(source_bytes, budget)?;
        let source = external_resource(self, input)?;
        let turtle = std::str::from_utf8(&source.bytes).map_err(|_| "Solid LDP input must be UTF-8")?;
        let (q42, count) = SemanticModelBuilder::new(budget)
            .seal_solid_turtle(source_iri, turtle, self.cancel).map_err(|error| error.to_string())?;
        let mut graph = ReceiptGraph::new(GRAPH);
        let iri = record_admission(&mut graph, "solid-turtle-import", &q42)?;
        graph.fact(&iri, "https://qualiadb.org/ontology/q42#solidSource", source_iri)?;
        graph.fact(&iri, "https://qualiadb.org/ontology/q42#sourceDigest",
            &format!("urn:sha256:{:x}", Sha256::digest(&source.bytes)))?;
        let receipt = graph.seal(budget)?;
        self.charge(0, q42.len().saturating_add(receipt.len()))?;
        Ok(Value::Record(BTreeMap::from([
            ("q42".into(), self.encode("q42", q42)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("quin_count".into(), Value::U64(count as u64)),
        ])))
    }

    fn nym_packet(&mut self, op: &str, params: &Args) -> Result<Value, String> {
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        let (kind, payload, sender) = if op == "qdnf_nym_envelope" {
            fields(params, &["frame", "sender"])?;
            let frame = external_resource(self, required(params, "frame")?)?;
            if frame.kind != "qframe" {
                return Err("A Nym envelope requires a QDNF QFrame".into());
            }
            let sender = text(required(params, "sender")?)?;
            let sealed = wrap_nym(&frame.bytes, sender.as_bytes())?;
            ("qnym", sealed, sender.to_owned())
        } else {
            fields(params, &["envelope"])?;
            let resource = external_resource(self, required(params, "envelope")?)?;
            if resource.kind != "qnym" {
                return Err("A Nym payload envelope must have kind qnym".into());
            }
            let (decoded, sender) = unwrap_nym(&resource.bytes)?;
            let sender = String::from_utf8(sender.as_slice().to_vec())
                .map_err(|_| "Nym fixture sender is not valid UTF-8")?;
            ("qframe", decoded, sender)
        };
        self.charge(payload.len(), ENVELOPE_LIMIT)?;
        let mut graph = ReceiptGraph::new(GRAPH);
        record_admission(&mut graph, op, &payload)?;
        let receipt = graph.seal(budget)?;
        Ok(Value::Record(BTreeMap::from([
            ("artifact".into(), self.encode(kind, payload)?),
            ("sender".into(), Value::String(sender)),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }
}

fn validate_qframe(bytes: &[u8]) -> Result<(), String> {
    let (_, offset, length) = decode_frame(bytes).map_err(|error| error.to_string())?;
    if offset + length != bytes.len() {
        return Err("QDNF QFrame contains bytes beyond its declared payload".into());
    }
    Ok(())
}

fn wrap_nym(frame: &[u8], sender: &[u8]) -> Result<Vec<u8>, String> {
    if frame.len() > ENVELOPE_LIMIT.saturating_sub(NYM_HEADER_LEN) || sender.is_empty() {
        return Err("QDNF frame does not fit the Nym payload MTU or has no sender locator".into());
    }
    validate_qframe(frame)?;
    let sender = ObservedLocator::from_slice(sender).map_err(|error| error.to_string())?;
    let mut out = vec![0; NYM_HEADER_LEN + frame.len()];
    let length = encapsulate_nym(&sender, frame, &mut out).map_err(|error| error.to_string())?;
    out.truncate(length);
    Ok(out)
}

fn unwrap_nym(bytes: &[u8]) -> Result<(Vec<u8>, ObservedLocator), String> {
    if !(NYM_HEADER_LEN..=ENVELOPE_LIMIT).contains(&bytes.len()) {
        return Err("Nym payload envelope is outside the frame MTU".into());
    }
    let mut sender = ObservedLocator::EMPTY;
    let mut out = vec![0; bytes.len() - NYM_HEADER_LEN];
    let length = decapsulate_nym(bytes, &mut sender, &mut out).map_err(|error| error.to_string())?;
    if bytes.len() != NYM_HEADER_LEN + length || sender.len == 0
        || sender.bytes[usize::from(sender.len)..].iter().any(|&byte| byte != 0)
    {
        return Err("Nym payload envelope has noncanonical framing".into());
    }
    out.truncate(length);
    validate_qframe(&out)?;
    Ok((out, sender))
}

fn record_admission(graph: &mut ReceiptGraph, operation: &str, bytes: &[u8]) -> Result<String, String> {
    let iri = format!("urn:sha256:{:x}", Sha256::digest(bytes));
    graph.fact(&iri, "https://qualiadb.org/ontology/q42#admissionOperation",
        &format!("urn:qualia:offline:{operation}"))?;
    Ok(iri)
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::net::qdnf::{
        frame::{encode_frame, FrameHeader}, registries::{FrameType, NextProtocol},
    };

    #[test]
    fn wraps_and_unwraps_native_qdnf_frame_with_exact_bounds() {
        let mut header = FrameHeader::new(FrameType::DiscoveryBeacon, NextProtocol::QLink);
        header.payload_len = 3;
        let mut buffer = [0; 128];
        let written = encode_frame(&header, &[4, 5, 6], &mut buffer).unwrap();
        let mut envelope = wrap_nym(&buffer[..written], b"peer-fixture").unwrap();
        let (result, locator) = unwrap_nym(&envelope).unwrap();
        assert_eq!(result, buffer[..written]);
        assert_eq!(locator.as_slice(), b"peer-fixture");
        envelope.push(0);
        assert!(unwrap_nym(&envelope).is_err());
    }
}

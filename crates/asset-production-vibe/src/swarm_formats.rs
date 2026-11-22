//! Bounded production adapters for native media interchange.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

#[cfg(not(target_arch = "wasm32"))]
mod dicom_seg;
mod document;
mod image;
mod layered;
mod morphology;
#[cfg(not(target_arch = "wasm32"))]
mod phantom;
mod segmentation;
mod video;

use crate::{
    Args, AssetHost, Value, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
};
use serde_json::Value as Json;
use sha2::{Digest, Sha256};

#[cfg(target_arch = "wasm32")]
impl AssetHost<'_> {
    pub(crate) fn ingest_and_reconstruct_dicom(&mut self, _args: &Args) -> Result<Value, String> {
        Err("CapabilityUnavailable: synthetic filtered backprojection requires a native host".into())
    }

    pub(crate) fn export_dicom_seg_rtstruct(&mut self, _args: &Args) -> Result<Value, String> {
        Err("CapabilityUnavailable: synthetic DICOM SEG requires a native host".into())
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn encode_json(host: &mut AssetHost<'_>, value: &Json) -> Result<Value, String> {
    host.encode(
        "json",
        serde_json::to_vec(value).map_err(|error| error.to_string())?,
    )
}

fn receipt(
    host: &mut AssetHost<'_>,
    format: &str,
    source: &str,
    output: &str,
) -> Result<Value, String> {
    let source = format!("urn:sha256:{source}");
    let output = format!("urn:sha256:{output}");
    receipt_iris(host, format, &source, &output)
}

fn receipt_iris(
    host: &mut AssetHost<'_>,
    format: &str,
    source: &str,
    output: &str,
) -> Result<Value, String> {
    let mut graph = ReceiptGraph::new(format);
    graph.fact(output, "http://www.w3.org/ns/prov#wasDerivedFrom", source)?;
    graph.fact(
        output,
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
        format,
    )?;
    host.encode(
        "q42",
        graph.seal(host.limits.resource_bytes.min(1024 * 1024))?,
    )
}

fn result(fields: &[(&str, Value)]) -> Value {
    Value::Record(Args::from_iter(
        fields
            .iter()
            .map(|(key, value)| ((*key).into(), value.clone())),
    ))
}

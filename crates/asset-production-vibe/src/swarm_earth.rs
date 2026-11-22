//! Bounded geoscience and retained GIS production operations.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;
use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::artifact::FieldLattice;
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{Args, AssetHost, TypedArtifact, Value, num, required};

mod ecology;
mod fwi;
mod gis;
mod kriging;
mod native_tissue_compartment_receipt;
mod native_trophic_receipt;
mod profiles;
mod rf_classroom;
mod trophic;
mod tsunami;
mod zarr;

fn bounded(args: &Args, key: &str, low: f64, high: f64) -> Result<f64, String> {
    let value = num(required(args, key)?)?;
    if (low..=high).contains(&value) {
        Ok(value)
    } else {
        Err(format!("{key} must be within [{low}, {high}]"))
    }
}

fn lattice(host: &mut AssetHost<'_>, axes: Vec<u32>, samples: Vec<f32>) -> Result<Value, String> {
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 { axes, samples },
    )))
}

fn receipt(
    host: &mut AssetHost<'_>,
    context: &str,
    facts: &[(&str, &str, &str)],
) -> Result<Value, String> {
    let context = if context.starts_with("urn:") {
        context.to_owned()
    } else {
        format!("urn:qualia:earth:receipt:{context}")
    };
    let mut graph = ReceiptGraph::new(&context);
    for &(subject, predicate, object) in facts {
        graph.fact(subject, predicate, object)?;
    }
    let bytes = graph.seal(host.limits.resource_bytes.min(1024 * 1024))?;
    host.encode("q42", bytes)
}

fn profile_result(
    host: &mut AssetHost<'_>,
    model: &str,
    shape: Vec<u32>,
    samples: Vec<f32>,
    report: serde_json::Value,
) -> Result<Value, String> {
    if samples.iter().any(|v| !v.is_finite()) {
        return Err(format!("{model} generated a nonfinite field"));
    }
    let report = serde_json::to_vec(&report).map_err(|err| err.to_string())?;
    let report_id = format!("urn:sha256:{:x}", Sha256::digest(&report));
    let mut hasher = Sha256::new();
    hasher.update((shape.len() as u32).to_le_bytes());
    for axis in &shape {
        hasher.update(axis.to_le_bytes());
    }
    for sample in &samples {
        hasher.update(sample.to_le_bytes());
    }
    let field_id = format!("urn:sha256:{:x}", hasher.finalize());
    let model_iri = format!("urn:qualia:earth:{model}");
    Ok(Value::Record(BTreeMap::from([
        ("field".into(), lattice(host, shape, samples)?),
        ("report".into(), host.encode("json", report)?),
        (
            "receipt".into(),
            receipt(
                host,
                model,
                &[
                    (
                        &report_id,
                        "http://www.w3.org/ns/prov#wasGeneratedBy",
                        &model_iri,
                    ),
                    (&report_id, "http://www.w3.org/ns/prov#hadMember", &field_id),
                ],
            )?,
        ),
    ])))
}

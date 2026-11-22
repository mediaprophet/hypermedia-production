//! Bounded production bridges for native rendering and offline optics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod cartoon;
mod caustics;
mod diffraction;
mod hybrid;
mod lens;
mod polarimetry;

use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{AssetHost, Value};

pub(super) fn receipt(
    host: &mut AssetHost<'_>,
    method: &str,
    samples: &[f32],
    facts: &[(&str, f64)],
) -> Result<Value, String> {
    let mut digest = Sha256::new();
    digest.update(method.as_bytes());
    digest.update((samples.len() as u64).to_le_bytes());
    for &v in samples {
        digest.update(v.to_le_bytes());
    }
    for &(k, v) in facts {
        digest.update(k.as_bytes());
        digest.update(v.to_le_bytes());
    }
    let id = format!("urn:qualia:offline-optics:{:x}", digest.finalize());
    let mut graph = ReceiptGraph::new("urn:qualia:offline-optics:v1");
    graph.fact(
        &id,
        "urn:qualia:offline-optics:method",
        &format!("urn:qualia:offline-optics:{method}"),
    )?;
    for &(k, v) in facts {
        graph.fact(
            &id,
            &format!("urn:qualia:offline-optics:{k}"),
            &format!("urn:ieee754:binary64:{:016x}", v.to_bits()),
        )?;
    }
    host.admit_encoded("q42", graph.seal(host.limits.resource_bytes)?)
}

//! Bounded collectable manufacture and verification.

use super::super::{fields, from_json, json, required, resource, text, Args, AssetHost, Value};
use qualia_core_db::{
    bundle::{semantic_model::{open_semantic_model, pack_semantic_hmc, HmcPackAsset, HmcPackLimits,
        SEMANTIC_MODEL_KEY}, BundleReadLimits, BundleReader},
    semantic_instruments::{assert_can_publish, build_collectable, inspect_for_publish,
        open_collectable, CollectableParts, InstrumentManifest},
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_COLLECTABLE: usize = 8 * 1024 * 1024;

fn limits(host: &AssetHost<'_>) -> BundleReadLimits {
    BundleReadLimits {
        file_bytes: host.limits.resource_bytes.min(MAX_COLLECTABLE),
        entries: 16,
        ..BundleReadLimits::default()
    }
}

pub(super) fn build(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["manifest", "definition_n3", "visual", "graph"])?;
    let mut bytes = 0;
    let manifest: InstrumentManifest = serde_json::from_value(
        json(required(params, "manifest")?, &mut bytes, 0)?,
    ).map_err(|error| format!("Invalid collectable manifest: {error}"))?;
    let definition = text(required(params, "definition_n3")?)?;
    let visual = required(params, "visual")?;
    let graph = params.get("graph");
    let visual_len = resource(host, visual, "10d")?.bytes.len();
    let graph_len = graph.map(|input| resource(host, input, "q42")).transpose()?
        .map_or(0, |source| source.bytes.len());
    let bundle_limits = limits(host);
    let size = bytes.saturating_add(definition.len()).saturating_add(visual_len).saturating_add(graph_len);
    if size > bundle_limits.file_bytes {
        return Err("Collectable source exceeds authoring byte budget".into());
    }
    host.charge(size, bundle_limits.file_bytes)?;
    let native = build_collectable(CollectableParts {
        manifest,
        definition_n3: definition.as_bytes().to_vec(),
        hcf: Vec::new(),
        visual_10d: resource(host, visual, "10d")?.bytes.clone(),
        small_q42: graph.map(|input| resource(host, input, "q42")).transpose()?
            .map(|resource| resource.bytes.clone()),
    }).map_err(|error| error.to_string())?;
    assert_can_publish(&inspect_for_publish(&native).map_err(|error| error.to_string())?)
        .map_err(|error| error.to_string())?;
    let bundle = BundleReader::parse_with_limits(&native, bundle_limits)
        .map_err(|error| error.to_string())?;
    let assets = bundle.entries().iter().map(|entry| {
        Ok(HmcPackAsset {
            key: &entry.key, kind: &entry.kind, metadata: entry.meta.as_deref(), salience: None,
            bytes: bundle.get(&entry.key).ok_or("Collectable member extent missing")?,
        })
    }).collect::<Result<Vec<_>, String>>()?;
    let packed = pack_semantic_hmc(&assets, None, HmcPackLimits {
        bundle: bundle_limits,
        resource_bytes: bundle_limits.file_bytes,
        total_resource_bytes: host.limits.total_resource_bytes,
        semantic_bytes: bundle_limits.file_bytes,
    }, host.cancel).map_err(|error| error.to_string())?;
    let mut result = inspect(&packed.hmc, bundle_limits)?;
    result.insert("hmc".into(), host.encode("hmc", packed.hmc)?);
    Ok(Value::Record(result))
}

pub(super) fn open(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["hmc"])?;
    let input = required(params, "hmc")?;
    let len = resource(host, input, "hmc")?.bytes.len();
    let bundle_limits = limits(host);
    if len > bundle_limits.file_bytes {
        return Err("Collectable exceeds byte budget".into());
    }
    host.charge(len, len)?;
    let bytes = &resource(host, input, "hmc")?.bytes;
    inspect(bytes, bundle_limits).map(Value::Record)
}

fn inspect(bytes: &[u8], limits: BundleReadLimits) -> Result<BTreeMap<String, Value>, String> {
    let model = BundleReader::parse_with_limits(bytes, limits).map_err(|error| error.to_string())?;
    let has_model = model.entry(SEMANTIC_MODEL_KEY).is_some() || model.entry("manifest.q42").is_some();
    if has_model {
        open_semantic_model(&model).map_err(|error| error.to_string())?;
    }
    let opened = open_collectable(bytes).map_err(|error| error.to_string())?;
    let mut payload = Sha256::new();
    for member in [&opened.hcf, &opened.definition_n3, &opened.visual_10d] {
        payload.update(member);
    }
    if let Some(q42) = &opened.small_q42 {
        payload.update(q42);
    }
    if opened.manifest.content_digest != format!("sha256:{:x}", payload.finalize()) {
        return Err("Collectable manifest digest does not match its authored payload".into());
    }
    let manifest = serde_json::to_value(&opened.manifest).map_err(|error| error.to_string())?;
    Ok(BTreeMap::from([
        ("manifest".into(), from_json(manifest)),
        ("bundle_digest".into(), Value::String(opened.bundle_digest)),
        ("has_semantic_model".into(), Value::Bool(has_model)),
        ("has_compiled_instrument_graph".into(), Value::Bool(opened.small_q42.is_some())),
    ]))
}

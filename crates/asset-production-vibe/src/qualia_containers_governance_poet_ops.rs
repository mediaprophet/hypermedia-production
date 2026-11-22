//! Vibe adapter for the bounded semantic HMC packer.

use super::{artifact::HmcWorkspaceView, fields, record, required, text, Args, AssetHost, Resource,
    TypedArtifact, Value};
use qualia_core_db::bundle::{
    semantic_model::{pack_semantic_hmc, HmcPackAsset, HmcPackLimits, HmcSalience, SEMANTIC_MODEL_KEY},
    BundleReadLimits, BundleReader,
};
use std::collections::BTreeMap;

impl AssetHost<'_> {
    pub(crate) fn pack_and_diff_hmc(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["bundle", "merit_order", "baseline"])?;
        let bundle = required(args, "bundle")?;
        let mut order = BTreeMap::new();
        if let Some(entries) = args.get("merit_order") {
            let entries = super::list(entries)?;
            if entries.len() > self.limits.resources {
                return Err("HMC merit order exceeds resource count limit".into());
            }
            for value in entries {
                let row = record(value)?;
                fields(row, &["key", "salience"])?;
                let key = text(required(row, "key")?)?;
                let priority = HmcSalience::parse(text(required(row, "salience")?)?)
                    .ok_or("HMC salience must be A, B, C, or D")?;
                if order.insert(key, priority).is_some() {
                    return Err(format!("Duplicate HMC merit order key {key}"));
                }
            }
        }
        let file_budget = self.limits.resource_bytes
            .min(self.limits.total_resource_bytes.saturating_sub(self.workspace.bytes));
        let bundle_limits = BundleReadLimits {
            file_bytes: file_budget,
            entries: self.limits.resources,
            ..BundleReadLimits::default()
        };
        let limits = HmcPackLimits {
            bundle: bundle_limits,
            resource_bytes: self.limits.resource_bytes,
            total_resource_bytes: self.limits.total_resource_bytes,
            semantic_bytes: 42 * 1024 * 1024,
        };
        self.charge(0, 0)?;
        let (output, work) = {
            let input_limits = BundleReadLimits {
                file_bytes: self.limits.resource_bytes,
                ..bundle_limits
            };
            let baseline_bytes = args.get("baseline")
                .map(|value| hmc_bytes(self, value)).transpose()?;
            let baseline = baseline_bytes.map(|bytes| verified_bundle(bytes, input_limits))
                .transpose().map_err(|error| format!("HMC baseline admission failed: {error}"))?;
            let parsed;
            let mut assets = if let Value::List(resources) = bundle {
                if resources.len() > self.limits.resources {
                    return Err("HMC asset list exceeds resource count limit".into());
                }
                let mut assets = Vec::with_capacity(resources.len());
                for value in resources {
                    let row = record(value)?;
                    fields(row, &["key", "resource"])?;
                    let key = text(required(row, "key")?)?;
                    let resource = encoded_resource(self, required(row, "resource")?)?;
                    assets.push(HmcPackAsset {
                        key, kind: &resource.kind, bytes: &resource.bytes, metadata: None, salience: None,
                    });
                }
                assets
            } else if let Ok(TypedArtifact::HmcBundle(view)) = self.workspace.typed(bundle) {
                match view.as_ref() {
                    HmcWorkspaceView::Staged(resources) => {
                        if resources.len() > self.limits.resources {
                            return Err("Staged HMC exceeds resource count limit".into());
                        }
                        resources.iter().map(resource_asset).collect()
                    }
                    HmcWorkspaceView::Sealed(bytes) => {
                        parsed = verified_bundle(bytes, input_limits)
                            .map_err(|error| format!("HMC input admission failed: {error}"))?;
                        sealed_assets(&parsed)?
                    }
                }
            } else {
                parsed = verified_bundle(hmc_bytes(self, bundle)?, input_limits)
                    .map_err(|error| format!("HMC input admission failed: {error}"))?;
                sealed_assets(&parsed)?
            };
            for asset in &mut assets {
                asset.salience = order.remove(asset.key);
            }
            if let Some((key, _)) = order.first_key_value() {
                return Err(format!("HMC merit order references unknown resource {key}"));
            }
            let work = assets.iter().map(|asset| asset.bytes.len())
                .chain(baseline_bytes.map(<[u8]>::len))
                .try_fold(0usize, |total, bytes| total.checked_add(bytes))
                .ok_or("HMC packing work overflows")?;
            if self.work.saturating_add(work as u64) > self.limits.asset_work_units {
                return Err("HMC packing exceeds production work budget".into());
            }
            let output = pack_semantic_hmc(&assets, baseline.as_ref(), limits, self.cancel)
                .map_err(|error| format!("HMC packing failed: {error}"))?;
            (output, work)
        };
        let total = output.hmc.len().checked_add(output.delta.as_ref().map_or(0, Vec::len))
            .ok_or("HMC output bytes overflow")?;
        let handles = 1 + usize::from(output.delta.is_some());
        if total > self.limits.total_resource_bytes.saturating_sub(self.workspace.bytes)
            || output.delta.as_ref().is_some_and(|bytes| bytes.len() > self.limits.resource_bytes)
            || handles > self.limits.resources.saturating_sub(self.workspace.entries.len())
        {
            return Err("HMC output and delta exceed workspace budget".into());
        }
        self.charge(work, total)?;
        let hmc = self.encode("hmc", output.hmc)?;
        let ranges = Value::List(output.ranges.into_iter().map(|range| {
            Value::Record(BTreeMap::from([
                ("key".into(), Value::String(range.key)),
                ("kind".into(), Value::String(range.kind)),
                ("offset".into(), Value::U64(range.offset)),
                ("length".into(), Value::U64(u64::from(range.length))),
                ("sha256".into(), Value::String(range.sha256_hex)),
                ("crc32c".into(), Value::U64(u64::from(range.crc32c))),
                ("salience".into(), Value::String(range.salience.as_str().into())),
            ]))
        }).collect());
        let mut result = BTreeMap::from([
            ("hmc".into(), hmc),
            ("semantic_model_key".into(), Value::String(SEMANTIC_MODEL_KEY.into())),
            ("ranges".into(), ranges),
        ]);
        if let Some(delta) = output.delta {
            result.insert("delta".into(), self.encode("q42", delta)?);
        }
        Ok(Value::Record(result))
    }
}

fn resource_asset(resource: &Resource) -> HmcPackAsset<'_> {
    HmcPackAsset {
        key: &resource.key, kind: &resource.kind, bytes: &resource.bytes,
        metadata: None, salience: None,
    }
}

fn verified_bundle(bytes: &[u8], limits: BundleReadLimits) -> Result<BundleReader<'_>, String> {
    let reader = BundleReader::parse_with_limits(bytes, limits).map_err(|error| error.to_string())?;
    for entry in reader.entries() {
        if !reader.verify_entry(&entry.key) {
            return Err(format!("HMC payload digest mismatch: {}", entry.key));
        }
    }
    Ok(reader)
}

fn sealed_assets<'a>(reader: &'a BundleReader<'_>) -> Result<Vec<HmcPackAsset<'a>>, String> {
    reader.entries().iter().map(|entry| {
        let bytes = reader.get(&entry.key).ok_or("Cannot read verified HMC entry")?;
        Ok(HmcPackAsset {
            key: &entry.key, kind: &entry.kind, bytes,
            metadata: entry.meta.as_deref(), salience: None,
        })
    }).collect()
}

pub(super) fn encoded_resource<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host.inputs.iter().find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn hmc_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    if let Ok(TypedArtifact::HmcBundle(view)) = host.workspace.typed(value) {
        return match view.as_ref() {
            HmcWorkspaceView::Staged(_) => Err("HMC baseline requires a sealed bundle".into()),
            HmcWorkspaceView::Sealed(bytes) => Ok(bytes),
        };
    }
    let resource = encoded_resource(host, value)?;
    if resource.kind != "hmc" {
        return Err("Expected a sealed HMC resource".into());
    }
    Ok(&resource.bytes)
}

//! Offline NP64 capability declaration admission and roofline routing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, *};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

impl AssetHost<'_> {
    pub(crate) fn attest_and_route_np64(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mode", "work", "targets", "isolation"])?;
        if text(required(a, "mode")?)? != "declared_roofline" {
            return Err(
                "Expected declared_roofline with caller-declared target capability evidence".into(),
            );
        }
        let work = record(required(a, "work")?)?;
        fields(
            work,
            &[
                "operations",
                "device_bytes",
                "working_set_bytes",
                "upload_bytes",
                "download_bytes",
                "precision",
            ],
        )?;
        let operation_count = bounded(work, "operations", 1.0, 1e18)?;
        let device_bytes = bounded(work, "device_bytes", 1.0, 1e16)?;
        let working_set = bounded(work, "working_set_bytes", 1.0, 1e16)?;
        let upload = bounded(work, "upload_bytes", 0.0, 1e16)?;
        let download = bounded(work, "download_bytes", 0.0, 1e16)?;
        let precision = text(required(work, "precision")?)?;
        if !matches!(precision, "f16" | "f32" | "f64" | "int8")
            || upload > working_set
            || download > working_set
        {
            return Err(
                "Unsupported precision or transfer totals exceeding the declared working set"
                    .into(),
            );
        }
        let isolation = text(required(a, "isolation")?)?;
        if !matches!(isolation, "wasm_sandbox" | "avf_microvm" | "bare_metal") {
            return Err("Unknown isolation envelope".into());
        }
        let targets = list(required(a, "targets")?)?;
        if !(1..=32).contains(&targets.len()) {
            return Err("Supply 1 to 32 explicitly declared targets".into());
        }
        self.charge(targets.len() * 2_048, targets.len() * 8_192)?;
        let mut seen = BTreeSet::new();
        let mut best: Option<(f64, &str)> = None;
        let mut rows = Vec::new();
        for value in targets {
            let target = record(value)?;
            fields(
                target,
                &[
                    "name",
                    "backend",
                    "evidence_sha256",
                    "precision",
                    "isolation",
                    "ops_per_s",
                    "device_bytes_per_s",
                    "transfer_bytes_per_s",
                    "usable_bytes",
                    "launch_s",
                ],
            )?;
            let name = text(required(target, "name")?)?;
            let backend = text(required(target, "backend")?)?;
            let evidence = text(required(target, "evidence_sha256")?)?;
            let target_precision = text(required(target, "precision")?)?;
            let target_isolation = text(required(target, "isolation")?)?;
            if name.is_empty()
                || name.len() > 80
                || name.chars().any(char::is_control)
                || !seen.insert(name)
                || !matches!(backend, "cpu" | "gpu" | "npu")
                || evidence.len() != 64
                || !evidence.bytes().all(|b| b.is_ascii_hexdigit())
                || !matches!(target_precision, "f16" | "f32" | "f64" | "int8")
                || !matches!(
                    target_isolation,
                    "wasm_sandbox" | "avf_microvm" | "bare_metal"
                )
            {
                return Err("Invalid or duplicate declared target identity, evidence, precision or envelope".into());
            }
            let compute = bounded(target, "ops_per_s", 1.0, 1e20)?;
            let bandwidth = bounded(target, "device_bytes_per_s", 1.0, 1e18)?;
            let transfer = bounded(target, "transfer_bytes_per_s", 1.0, 1e18)?;
            let memory = bounded(target, "usable_bytes", 1.0, 1e16)?;
            let launch = bounded(target, "launch_s", 0.0, 10.0)?;
            let denial = if isolation != target_isolation {
                Some("isolation")
            } else if precision != target_precision {
                Some("precision")
            } else if working_set > memory {
                Some("working_set")
            } else {
                None
            };
            let seconds = (operation_count / compute).max(device_bytes / bandwidth)
                + (upload + download) / transfer
                + launch;
            if denial.is_none() && best.is_none_or(|(old, label)| (seconds, name) < (old, label)) {
                best = Some((seconds, name));
            }
            rows.push(json!({"name":name,"backend":backend,"evidence_sha256":evidence.to_ascii_lowercase(),
                "estimated_seconds":seconds,"denied_by":denial,"precision":target_precision,"isolation":target_isolation,
                "ops_per_s":compute,"device_bytes_per_s":bandwidth,"transfer_bytes_per_s":transfer,"usable_bytes":memory,"launch_s":launch}));
        }
        let (seconds, name) = best.ok_or(
            "No declared target admits the requested isolation, precision and working set",
        )?;
        rows.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
        let plan = serde_json::to_vec(&json!({
            "schema":"qualia.declared-roofline/v1", "evidence_status":"caller_declared", "selected_target":name,
            "requested":{"operations":operation_count,"device_bytes":device_bytes,"working_set_bytes":working_set,
                "upload_bytes":upload,"download_bytes":download,"precision":precision,"isolation":isolation},
            "targets":rows,
        })).map_err(|e| e.to_string())?;
        let digest = format!("{:x}", Sha256::digest(&plan));
        let subject = format!("urn:qualia:declared-roofline:sha256:{digest}");
        let target = rows
            .iter()
            .find(|value| value["name"].as_str() == Some(name))
            .ok_or("Selected target missing")?;
        let target_digest = target["evidence_sha256"]
            .as_str()
            .ok_or("Selected target missing evidence")?;
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "urn:qualia:compute:inputEvidence",
            &format!("urn:sha256:{target_digest}"),
        )?;
        graph.fact(
            &subject,
            "urn:qualia:compute:evidenceStatus",
            "urn:qualia:compute:caller-declared",
        )?;
        let receipt = graph.seal(self.limits.resource_bytes)?;
        Ok(Value::Record(Args::from([
            ("plan".into(), self.encode("json", plan)?),
            ("receipt".into(), self.encode("q42", receipt)?),
            ("selected_target".into(), Value::String(name.into())),
            ("estimated_seconds".into(), Value::F64(seconds)),
        ])))
    }
}

fn bounded(a: &Args, field: &str, min: f64, max: f64) -> Result<f64, String> {
    let n = num(required(a, field)?)?;
    if (min..=max).contains(&n) {
        Ok(n)
    } else {
        Err(format!("{field} outside its admitted range"))
    }
}

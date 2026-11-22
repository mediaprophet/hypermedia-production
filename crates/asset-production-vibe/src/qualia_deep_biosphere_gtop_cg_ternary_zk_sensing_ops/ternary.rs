//! Offline P64 tensor quantization and lossless GPU-layout rebaking.

use std::{collections::BTreeMap, io::{self, Write}};

use qualia_core_db::{
    inference::neuro_symbolic_sieve::{NeuroSymbolicSieve, SieveSlot, MAX_SIEVE_ALLOW},
    inference::ternary::{self, GGML_TYPE_TERNARY_158},
    q42::p64_weight::{transcode_safetensor_to_p64_policy, IntegrityMode, P64TensorIndex},
    q42_volume::WasmSliceQ42Reader,
    q_hash,
};
use sha2::{Digest, Sha256};

use crate::{artifact::CartridgeKind, fields, integer, list, num, record, required, text,
    Args, AssetHost, TypedArtifact, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;

const MAX_RAW_WEIGHTS: usize = 65_536;
const MAX_TENSOR_WEIGHTS: usize = 32 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn transcode_p64_ternary_and_sieve(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let args = record(required(args, "parameters")?)?;
        match op {
            "quantize_ternary" => self.quantize_ternary_weights(args),
            "transcode_safetensors" => self.transcode_ternary_safetensors(args),
            "rebake_tensor_2bit" => self.rebake_ternary_tensor(args),
            "sieve_nquin" => self.sieve_nquin(args),
            _ => Err(format!("Unknown P64 ternary operation {op}")),
        }
    }

    fn quantize_ternary_weights(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["weights"])?;
        let weights = list(required(args, "weights")?)?;
        if weights.is_empty() || weights.len() > MAX_RAW_WEIGHTS {
            return Err("Provide 1–65536 inline tensor weights".into());
        }
        self.charge(weights.len() * 4, weights.len() * 8)?;
        let weights = weights.iter().map(|value| {
            let value = num(value)? as f32;
            if !value.is_finite() { return Err("Weight cannot be represented as f32".into()); }
            Ok::<_, String>(value)
        }).collect::<Result<Vec<_>, _>>()?;
        let packed = ternary::ternary_blob(&weights);
        let scale = validate_blob(&packed, weights.len())?;
        let receipt = receipt("quantize_ternary", &packed, self.limits.resource_bytes)?;
        Ok(Value::Record(BTreeMap::from([
            ("tensor".into(), self.encode("ternary158", packed)?),
            ("element_count".into(), Value::U64(weights.len() as u64)),
            ("scale".into(), Value::F64(f64::from(scale))),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }

    fn transcode_ternary_safetensors(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source"])?;
        let source = required(args, "source")?;
        let budget = self.limits.resource_bytes;
        let input = resource_bytes(self, source, "safetensors")?;
        if input.len() > budget { return Err("Safetensors source exceeds the resource budget".into()); }
        let work = input.len().saturating_mul(4);
        self.charge(work, budget.saturating_mul(2))?;
        let input = resource_bytes(self, source, "safetensors")?;
        let mut writer = BoundedWriter { bytes: Vec::new(), budget };
        transcode_safetensor_to_p64_policy(input, 12, &mut writer)
            .map_err(|error| format!("P64 transcode: {error}"))?;
        let index = P64TensorIndex::from_p64_with_integrity(&writer.bytes, IntegrityMode::Full)?;
        let tensors = index.entries.iter().filter(|entry| u32::from(entry.dtype) == GGML_TYPE_TERNARY_158).count();
        if tensors == 0 { return Err("Safetensors source has no supported FFN tensor to quantize".into()); }
        let receipt = receipt("transcode_safetensors", &writer.bytes, budget)?;
        Ok(Value::Record(BTreeMap::from([
            ("p64".into(), self.encode("p64", writer.bytes)?),
            ("ternary_tensors".into(), Value::U64(tensors as u64)),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }

    fn rebake_ternary_tensor(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "tensor_index", "element_count"])?;
        let source = required(args, "source")?;
        let tensor_index = args.get("tensor_index").map(|value| integer(value, "tensor_index")).transpose()?;
        let count = args.get("element_count").map(|value| integer(value, "element_count")).transpose()?;
        if tensor_index.is_some() == count.is_some() {
            return Err("Supply a P64 tensor_index or a raw ternary element_count".into());
        }
        let kind = if tensor_index.is_some() { "p64" } else { "ternary158" };
        let input = resource_bytes(self, source, kind)?;
        if input.len() > self.limits.resource_bytes { return Err("Ternary source exceeds the resource budget".into()); }
        self.charge(input.len(), 0)?;
        let input = resource_bytes(self, source, kind)?;
        let (blob, count) = if let Some(slot) = tensor_index {
            let index = P64TensorIndex::from_p64_with_integrity(input, IntegrityMode::Full)?;
            let entry = index.entries.get(slot).ok_or("P64 tensor index is out of range")?;
            if u32::from(entry.dtype) != GGML_TYPE_TERNARY_158 {
                return Err("Selected P64 tensor is not ternary".into());
            }
            let rank = usize::try_from(entry.rank).map_err(|_| "Invalid P64 tensor rank")?;
            let dims = entry.dimensions.get(..rank).filter(|dims| !dims.is_empty())
                .ok_or("Invalid P64 tensor dimensions")?;
            let count = dims.iter().try_fold(1usize, |product, &size| product.checked_mul(size as usize))
                .ok_or("P64 element count overflows")?;
            (index.blob(input, entry), count)
        } else {
            (input, count.ok_or("Missing raw ternary element_count")?)
        };
        let scale = validate_blob(blob, count)?;
        let needed = count.saturating_add(ternary::packed_trit_len_2bit(count) + 4);
        let work = count.saturating_mul(2);
        if needed > self.limits.resource_bytes || self.work.saturating_add(work as u64) > self.limits.asset_work_units {
            return Err("Ternary rebake exceeds its workspace budget".into());
        }
        let (_, gpu_bytes) = ternary::rebake_ternary_blob_to_2bit(blob, count);
        self.charge(work, needed)?;
        let mut bytes = Vec::with_capacity(gpu_bytes.len() + 4);
        bytes.extend_from_slice(&scale.to_le_bytes());
        bytes.extend_from_slice(&gpu_bytes);
        let receipt = receipt("rebake_tensor_2bit", &bytes, self.limits.resource_bytes)?;
        Ok(Value::Record(BTreeMap::from([
            ("tensor".into(), self.encode("ternary2bit", bytes)?),
            ("element_count".into(), Value::U64(count as u64)),
            ("scale".into(), Value::F64(f64::from(scale))),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }

    fn sieve_nquin(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "subjects", "predicates", "objects", "tokens", "context_iri"])?;
        let model = required(args, "model")?;
        let tokens = list(required(args, "tokens")?)?;
        if tokens.len() != 3 { return Err("A complete sieve emission requires three token IDs".into()); }
        let size = resource_bytes(self, model, "q42")?.len();
        if size > 40 * 1024 * 1024 { return Err("Sieve model exceeds the vocabulary read budget".into()); }
        self.charge(size, 1024 * 1024 + 4096)?;
        let reader = WasmSliceQ42Reader::open(resource_bytes(self, model, "q42")?)
            .map_err(|error| error.to_string())?;
        if !reader.is_unified_volume() { return Err("Sieve requires a Q42 volume with Q42LEX".into()); }
        let mut page = vec![0; 1024 * 1024];
        let mut lexical = [0; 4096];
        let mut admitted = |iri: &str| -> Result<u64, String> {
            if iri.len() > lexical.len() { return Err("Sieve term exceeds the lexical budget".into()); }
            qualia_core_db::bundle::semantic_model::resource_identity_meta(iri).map_err(|error| error.to_string())?;
            let hash = q_hash(iri);
            let len = reader.lookup_lexicon_hash_into(hash, &mut page, &mut lexical)
                .map_err(|error| error.to_string())?;
            if len.and_then(|len| lexical.get(..len)) != Some(iri.as_bytes()) {
                return Err("Sieve term lacks its exact Q42LEX binding".into());
            }
            Ok(hash)
        };
        let context = text(required(args, "context_iri")?)?;
        let context_hash = admitted(context)?;
        let mut dictionary = BTreeMap::new();
        let mut resolved = BTreeMap::new();
        let mut masks = [Vec::new(), Vec::new(), Vec::new()];
        for (name, mask) in ["subjects", "predicates", "objects"].into_iter().zip(&mut masks) {
            let items = list(required(args, name)?)?;
            if !(1..=MAX_SIEVE_ALLOW).contains(&items.len()) { return Err("Sieve slot count is outside the grammar bound".into()); }
            for item in items {
                let row = record(item)?;
                fields(row, &["token_id", "iri"])?;
                let token_id = token(required(row, "token_id")?)?;
                let iri = text(required(row, "iri")?)?;
                let lexicon_hash = admitted(iri)?;
                if dictionary.insert(token_id, lexicon_hash).is_some_and(|known| known != lexicon_hash) {
                    return Err("Sieve token maps to different IRIs across grammar states".into());
                }
                resolved.insert(lexicon_hash, iri);
                mask.push(SieveSlot { token_id, lexicon_hash });
            }
        }
        let mut sieve = NeuroSymbolicSieve::from_admitted_slots(masks.each_ref().map(Vec::as_slice))
            .map_err(|error| format!("Invalid sieve grammar: {error:?}"))?;
        for value in tokens { sieve.apply_token(token(value)?).map_err(|error| format!("Sieve rejected token: {error:?}"))?; }
        if !sieve.is_complete() { return Err("Sieve did not yield a complete triple".into()); }
        let quin = sieve.assemble_quin(context_hash);
        let mut receipt = ReceiptGraph::new(context);
        receipt.fact(resolved[&quin.subject], resolved[&quin.predicate], resolved[&quin.object])?;
        let receipt = receipt.seal(self.limits.resource_bytes.min(4 * 1024 * 1024))?;
        Ok(Value::Record(BTreeMap::from([
            ("nquin".into(), Value::List([quin.subject, quin.predicate, quin.object, quin.context,
                quin.metadata, quin.parity].into_iter().map(Value::U64).collect())),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }
}

fn token(value: &Value) -> Result<u32, String> {
    u32::try_from(integer(value, "token_id")?).map_err(|_| "Token ID exceeds u32".into())
}

fn resource_bytes<'a>(host: &'a AssetHost<'_>, value: &Value, kind: &str) -> Result<&'a [u8], String> {
    if kind == "p64" {
        if let Ok(TypedArtifact::ComputeCartridge { kind: CartridgeKind::P64, bytes }) = host.workspace.typed(value) {
            return Ok(bytes);
        }
    }
    let resource = match value {
        Value::String(key) => host.inputs.iter().find(|item| &item.key == key).ok_or("Unknown retained weight resource")?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != kind { return Err(format!("Expected a {kind} weight resource")); }
    Ok(&resource.bytes)
}

fn validate_blob(bytes: &[u8], count: usize) -> Result<f32, String> {
    if !(1..=MAX_TENSOR_WEIGHTS).contains(&count) || bytes.len() != ternary::ternary_blob_len(count) {
        return Err("Ternary tensor byte length does not match its bounded element count".into());
    }
    let scale = f32::from_le_bytes(bytes[..4].try_into().map_err(|_| "Missing ternary scale")?);
    if !scale.is_finite() || scale.is_sign_negative() || bytes[4..].iter().any(|&byte| byte >= 243) {
        return Err("Invalid ternary scale or base-3 byte".into());
    }
    let remainder = count % ternary::TRITS_PER_BYTE;
    if remainder != 0 && u32::from(bytes[bytes.len() - 1]) >= 3u32.pow(remainder as u32) {
        return Err("Noncanonical ternary padding".into());
    }
    Ok(scale)
}

fn receipt(operation: &str, artifact: &[u8], budget: usize) -> Result<Vec<u8>, String> {
    let mut graph = ReceiptGraph::new("urn:qualia:p64:ternaryReceipt");
    graph.fact(&format!("urn:sha256:{:x}", Sha256::digest(artifact)),
        "urn:qualia:p64:operation", &format!("urn:qualia:p64:{operation}"))?;
    graph.seal(budget.min(4 * 1024 * 1024))
}

struct BoundedWriter {
    bytes: Vec<u8>,
    budget: usize,
}

impl Write for BoundedWriter {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if data.len() > self.budget.saturating_sub(self.bytes.len()) {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "P64 output exceeds resource budget"));
        }
        self.bytes.extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

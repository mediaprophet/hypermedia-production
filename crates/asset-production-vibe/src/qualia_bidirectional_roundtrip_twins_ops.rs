//! Bounded P64 inspection, dequantization and SafeTensors export.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod roundtrip_tensor_exports;

use std::sync::Arc;

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    inference::ternary::{dequantize_blob, GGML_TYPE_TERNARY_158},
    modalities::evidential_etau::f16_to_f32,
    q42::p64_weight::{IntegrityMode, P64TensorEntry, P64TensorIndex, P64WeightHeader},
};
use sha2::{Digest, Sha256};

use crate::{fields, integer, list, required, text, Args, AssetHost, TypedArtifact, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::qualia_containers_governance_poet_ops::encoded_resource;

const MAX_BYTES: usize = 32 * 1024 * 1024;
const MAX_TENSORS: usize = 8192;
const MAX_DECODED_VALUES: usize = 1_048_576;

impl AssetHost<'_> {
    pub(crate) fn inspect_and_dequantize_p64(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "source_sha256", "mode", "tensor_ordinals"])?;
        let mode = args.get("mode").map(text).transpose()?.unwrap_or("inspect");
        if !matches!(mode, "inspect" | "dequantize") {
            return Err("P64 inspection mode must be inspect or dequantize".into());
        }
        let handle = required(args, "source")?;
        let size = encoded_resource(self, handle)?.bytes.len();
        if !(64..=self.limits.resource_bytes.min(MAX_BYTES)).contains(&size) {
            return Err("P64 inspection requires a bounded retained container".into());
        }
        self.charge(size * 6, 16 * 1024 * 1024)?;
        let resource = encoded_resource(self, handle)?;
        let digest = format!("{:x}", Sha256::digest(&resource.bytes));
        if resource.kind != "p64" || !digest.eq_ignore_ascii_case(text(required(args, "source_sha256")?)?) {
            return Err("P64 inspection requires exact source SHA-256 and format".into());
        }
        let header = P64WeightHeader::read_le(&resource.bytes)?;
        if !(1..=MAX_TENSORS as u32).contains(&header.tensor_count) {
            return Err("P64 tensor directory exceeds the inspection limit".into());
        }
        let index = P64TensorIndex::from_p64_with_integrity(&resource.bytes, IntegrityMode::Full)?;
        let ordinals = if let Some(values) = args.get("tensor_ordinals") {
            let values = list(values)?;
            if values.is_empty() || values.len() > 64 {
                return Err("Select 1 to 64 explicit P64 tensor ordinals".into());
            }
            let mut ordinals = values.iter().map(|value| integer(value, "tensor ordinal"))
                .collect::<Result<Vec<_>, _>>()?;
            ordinals.sort_unstable();
            if ordinals.windows(2).any(|pair| pair[0] == pair[1]) || ordinals.last().is_some_and(|&n| n >= index.entries.len()) {
                return Err("P64 ordinals must be unique and within the admitted directory".into());
            }
            ordinals
        } else if mode == "inspect" {
            (0..index.entries.len()).collect()
        } else {
            return Err("Dequantization requires explicit tensor_ordinals".into());
        };
        let mut book = Workbook::new();
        let sheet_id = book.add_sheet("P64 tensor directory");
        let sheet = book.sheet_mut(sheet_id).ok_or("Failed to allocate P64 inspection sheet")?;
        for (column, label) in ["ordinal", "source_name_hash", "dtype", "role_id", "dimensions", "blob_bytes"].into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::Text(label.into()));
        }
        let mut safetensors = serde_json::Map::new();
        let mut decoded = Vec::new();
        let mut receipt = ReceiptGraph::new("urn:qualia:p64:inspection:v1");
        let source = format!("urn:sha256:{digest}");
        receipt.fact(&source, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type", "urn:qualia:p64:IntegrityCheckedContainer")?;
        for (row, &ordinal) in ordinals.iter().enumerate() {
            let entry = &index.entries[ordinal];
            let dims = entry.dimensions.get(..entry.rank as usize).filter(|shape| !shape.is_empty())
                .ok_or("Invalid P64 tensor rank")?;
            let suffix = format!("tensor_{ordinal:05}_{:016x}", entry.source_name_hash);
            let tensor = format!("{source}:{suffix}");
            receipt.fact(&source, "urn:qualia:p64:inspectedTensor", &tensor)?;
            let shape = dims.iter().map(u32::to_string).collect::<Vec<_>>().join(" × ");
            let metadata = [CellValue::Number(ordinal as f64), CellValue::Text(format!("{:016x}", entry.source_name_hash)),
                CellValue::Number(f64::from(entry.dtype)), CellValue::Number(f64::from(entry.role_id)),
                CellValue::Text(shape), CellValue::Number(f64::from(entry.blob_size))];
            for (column, value) in metadata.into_iter().enumerate() {
                sheet.set_value(CellRef::new(row as u32 + 1, column as u32), value);
            }
            if mode == "dequantize" {
                let count = dims.iter().try_fold(1usize, |count, &length| count.checked_mul(length as usize))
                    .filter(|&count| count != 0 && count <= MAX_DECODED_VALUES - decoded.len() / 4)
                    .ok_or("P64 decoded tensor exceeds the admitted sample budget")?;
                let start = decoded.len();
                let blob = entry.blob_slice(&resource.bytes).ok_or("P64 tensor blob is outside its verified extent")?;
                decode_tensor(entry, blob, count, &mut decoded)?;
                safetensors.insert(suffix, serde_json::json!({"dtype":"F32", "shape":[count], "data_offsets":[start,decoded.len()]}));
            }
        }
        let archive = if mode == "dequantize" {
            let mut header = serde_json::to_vec(&safetensors).map_err(|error| error.to_string())?;
            header.resize(header.len().next_multiple_of(8), b' ');
            let mut bytes = Vec::with_capacity(8 + header.len() + decoded.len());
            bytes.extend((header.len() as u64).to_le_bytes());
            bytes.append(&mut header);
            bytes.append(&mut decoded);
            let archive = format!("urn:sha256:{:x}", Sha256::digest(&bytes));
            receipt.fact(&archive, "http://www.w3.org/ns/prov#wasDerivedFrom", &source)?;
            Some(bytes)
        } else { None };
        let graph = receipt.seal(self.limits.resource_bytes.min(4 * 1024 * 1024))?;
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let graph = self.encode("q42", graph)?;
        let mut output = Args::from([
            ("table".into(), table), ("receipt".into(), graph), ("source_sha256".into(), Value::String(digest)),
        ]);
        if let Some(archive) = archive {
            output.insert("safetensors".into(), self.encode("safetensors", archive)?);
        }
        Ok(Value::Record(output))
    }
}

fn decode_tensor(entry: &P64TensorEntry, bytes: &[u8], count: usize, output: &mut Vec<u8>) -> Result<(), String> {
    let mut append = |value: f32| {
        if !value.is_finite() {
            return Err("Dequantized P64 tensor is non-finite".into());
        }
        output.extend(value.to_le_bytes());
        Ok::<(), String>(())
    };
    match u32::from(entry.dtype) {
        0 if bytes.len() == count * 4 => {
            for word in bytes.chunks_exact(4) {
                append(f32::from_le_bytes(word.try_into().unwrap()))?;
            }
        }
        1 if bytes.len() == count * 2 => {
            for word in bytes.chunks_exact(2) {
                append(f16_to_f32(u16::from_le_bytes(word.try_into().unwrap())))?;
            }
        }
        GGML_TYPE_TERNARY_158 if bytes.len() == 4 + count.div_ceil(5) => {
            let scale = f32::from_le_bytes(bytes[..4].try_into().unwrap());
            if !scale.is_finite() || scale < 0.0 || bytes[4..].iter().any(|&digit| digit >= 243) {
                return Err("P64 ternary tensor has an invalid scale or packed trit".into());
            }
            let mut decoded = vec![0.0; count];
            dequantize_blob(bytes, &mut decoded);
            for value in decoded { append(value)?; }
        }
        _ => return Err("P64 dequantization requires a complete F32, F16 or native base-3 ternary tensor".into()),
    }
    Ok(())
}

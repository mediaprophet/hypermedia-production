//! Local aggregate diagnostics on synthetic probes for P64 ternary dictionaries.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, mem::size_of, sync::Arc};

use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use qualia_core_db::{
    inference::ternary::{ternary_blob_len, ternary_gemm_cpu, trit_at, GGML_TYPE_TERNARY_158},
    q42::p64_weight::{IntegrityMode, P64TensorEntry, P64TensorIndex, P64WeightHeader},
};
use sha2::{Digest, Sha256};

use crate::{
    fields, integer, num, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, required, text, Args, AssetHost,
    TypedArtifact, Value,
};

use super::{budget, count_value};

const CONTEXT: &str = "urn:qualia:ai:p64-synthetic-sparse-dictionary-audit-v1";
const MAX_BYTES: usize = 4 * 1024 * 1024;
const MAX_TENSORS: usize = 1024;
const MAX_WIDTH: usize = 128;
const MAX_FEATURES: usize = 256;
const MAX_ELEMENTS: usize = 16_384;
const MAX_PROBES: usize = 128;

#[derive(Clone, Copy)]
struct Matrix<'a> {
    packed: &'a [u8],
    scale: f32,
    inputs: usize,
    outputs: usize,
}

#[derive(Clone, Copy)]
enum Activation {
    TopK(usize),
    JumpRelu(f32),
}

#[derive(Clone, Copy, Default)]
struct Feature {
    count: [u32; 2],
    total: f64,
    removal_energy: f64,
    decoder_norm_squared: f64,
}

struct Report {
    features: [Feature; MAX_FEATURES],
    mean_l0: f64,
    reconstruction_mse: f64,
}

impl AssetHost<'_> {
    pub(crate) fn interpret_p64_sae_and_renyi_dp_federation(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "source_sha256",
                "encoder_ordinal",
                "decoder_ordinal",
                "dimension_order",
                "activation",
                "sae_l0_sparsity",
                "jump_threshold",
                "probe_count",
                "arena_budget_bytes",
            ],
        )?;
        let handle = required(args, "source")?;
        let source_size = encoded_resource(self, handle)?.bytes.len();
        if !(64..=self.limits.resource_bytes.min(MAX_BYTES)).contains(&source_size) {
            return Err(
                "The local sparse dictionary audit requires a retained P64 of at most 4 MiB".into(),
            );
        }
        let probes = integer(required(args, "probe_count")?, "probe_count")?;
        if !(4..=MAX_PROBES).contains(&probes) || probes % 4 != 0 {
            return Err(
                "probe_count must be a multiple of 4 in 4..=128 for paired split halves".into(),
            );
        }
        let encoder_ordinal = integer(required(args, "encoder_ordinal")?, "encoder_ordinal")?;
        let decoder_ordinal = integer(required(args, "decoder_ordinal")?, "decoder_ordinal")?;
        if encoder_ordinal == decoder_ordinal {
            return Err(
                "The sparse dictionary audit requires distinct encoder and decoder tensors".into(),
            );
        }
        let rows_first = match text(required(args, "dimension_order")?)? {
            "rows_columns" => true,
            "columns_rows" => false,
            _ => return Err(
                "Specify explicitly whether the P64 dimensions are rows_columns or columns_rows"
                    .into(),
            ),
        };
        let activation = match text(required(args, "activation")?)? {
            "top_k" if !args.contains_key("jump_threshold") => Activation::TopK(integer(
                required(args, "sae_l0_sparsity")?,
                "sae_l0_sparsity",
            )?),
            "jump_relu" if !args.contains_key("sae_l0_sparsity") => {
                let threshold = num(required(args, "jump_threshold")?)?;
                if !(1e-6..=1e4).contains(&threshold) {
                    return Err("jump_threshold must be in 1e-6..=1e4".into());
                }
                Activation::JumpRelu(threshold as f32)
            }
            _ => {
                return Err(
                    "Choose top_k with sae_l0_sparsity or jump_relu with jump_threshold".into(),
                )
            }
        };
        let scratch = 4 * 1024 * 1024
            + MAX_FEATURES * (size_of::<Feature>() + size_of::<usize>() + size_of::<f32>())
            + MAX_WIDTH * 2 * size_of::<f32>();
        budget(args, scratch)?;
        self.charge(source_size * 6 + MAX_ELEMENTS * probes * 6, scratch)?;

        let resource = encoded_resource(self, handle)?;
        let digest = format!("{:x}", Sha256::digest(&resource.bytes));
        if resource.kind != "p64"
            || !digest.eq_ignore_ascii_case(text(required(args, "source_sha256")?)?)
        {
            return Err(
                "The sparse dictionary source does not match its declared P64 digest".into(),
            );
        }
        let header = P64WeightHeader::read_le(&resource.bytes)?;
        if !(2..=MAX_TENSORS as u32).contains(&header.tensor_count) {
            return Err("The sparse dictionary directory exceeds its tensor-count cap".into());
        }
        let index = P64TensorIndex::from_p64_with_integrity(&resource.bytes, IntegrityMode::Full)?;
        let encoder = matrix(&resource.bytes, entry(&index, encoder_ordinal)?, rows_first)?;
        let decoder = matrix(&resource.bytes, entry(&index, decoder_ordinal)?, rows_first)?;
        if encoder.inputs != decoder.outputs
            || encoder.outputs != decoder.inputs
            || encoder.inputs > MAX_WIDTH
            || encoder.outputs > MAX_FEATURES
        {
            return Err("Encoder and decoder must have compatible bounded activation and feature dimensions".into());
        }
        if matches!(activation, Activation::TopK(k) if !(1..=encoder.outputs.min(32)).contains(&k))
        {
            return Err("sae_l0_sparsity must be in 1..=min(32, feature_count)".into());
        }
        let feature_count = encoder.outputs;
        let report = audit(encoder, decoder, &activation, probes)?;
        let source_iri = format!("urn:sha256:{digest}");
        let identity = audit_identity(
            &source_iri,
            encoder_ordinal,
            decoder_ordinal,
            rows_first,
            &activation,
            probes,
        );
        let mut receipt = ReceiptGraph::new(CONTEXT);
        receipt.fact(
            &identity,
            "http://www.w3.org/ns/prov#wasDerivedFrom",
            &source_iri,
        )?;
        receipt.fact(&identity, "urn:qualia:ai:synthetic-probe-protocol", CONTEXT)?;
        receipt.fact(
            &identity,
            "urn:qualia:ai:mean-l0-bits",
            &format!("urn:qualia:binary64:{:016x}", report.mean_l0.to_bits()),
        )?;
        receipt.fact(
            &identity,
            "urn:qualia:ai:mean-reconstruction-error-bits",
            &format!(
                "urn:qualia:binary64:{:016x}",
                report.reconstruction_mse.to_bits()
            ),
        )?;
        let graph = receipt.seal(self.limits.resource_bytes)?;

        let mut book = Workbook::new();
        let id = book.add_sheet("Aggregate synthetic SAE features");
        let sheet = book
            .sheet_mut(id)
            .ok_or("Cannot create sparse dictionary diagnostic sheet")?;
        for (column, title) in [
            "feature",
            "fraction_active",
            "mean_nonnegative_activation",
            "split_half_rate_gap",
            "mean_decoder_removal_delta_squared",
        ]
        .into_iter()
        .enumerate()
        {
            sheet.set_value(
                CellRef::new(0, column as u32),
                CellValue::Text(title.into()),
            );
        }
        for (feature, stats) in report.features[..feature_count].iter().enumerate() {
            let [first, second] = stats.count;
            let probe_count = probes as f64;
            let values = [
                feature as f64,
                f64::from(first + second) / probe_count,
                stats.total / probe_count,
                f64::from(first.abs_diff(second)) / (0.5 * probe_count),
                stats.removal_energy / probe_count,
            ];
            for (column, value) in values.into_iter().enumerate() {
                sheet.set_value(
                    CellRef::new(feature as u32 + 1, column as u32),
                    CellValue::Number(value),
                );
            }
        }
        let table = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(book)))?;
        let receipt = self.encode("q42", graph)?;
        Ok(Value::Record(BTreeMap::from([
            ("table".into(), table),
            ("receipt".into(), receipt),
            ("probe_count".into(), count_value(probes)),
            ("feature_count".into(), count_value(feature_count)),
            ("mean_l0".into(), Value::F64(report.mean_l0)),
            (
                "mean_reconstruction_mse".into(),
                Value::F64(report.reconstruction_mse),
            ),
        ])))
    }
}

fn entry(index: &P64TensorIndex, ordinal: usize) -> Result<&P64TensorEntry, String> {
    index
        .entries
        .get(ordinal)
        .ok_or_else(|| "Selected dictionary tensor ordinal is outside the P64".into())
}

fn matrix<'a>(
    bytes: &'a [u8],
    entry: &P64TensorEntry,
    rows_first: bool,
) -> Result<Matrix<'a>, String> {
    if entry.rank != 2 || u32::from(entry.dtype) != GGML_TYPE_TERNARY_158 {
        return Err(
            "The sparse dictionary audit requires rank-two native ternary P64 tensors".into(),
        );
    }
    let columns = usize::from(rows_first);
    let inputs = entry.dimensions[columns] as usize;
    let outputs = entry.dimensions[1 - columns] as usize;
    let count = inputs
        .checked_mul(outputs)
        .filter(|&n| (1..=MAX_ELEMENTS).contains(&n))
        .ok_or("Sparse dictionary matrix shape exceeds its element cap")?;
    let blob = entry
        .blob_slice(bytes)
        .ok_or("Missing sparse dictionary tensor blob")?;
    if blob.len() != ternary_blob_len(count) {
        return Err("Ternary dictionary blob length disagrees with its dimensions".into());
    }
    let mut scale_bytes = [0; 4];
    scale_bytes.copy_from_slice(&blob[..4]);
    let scale = f32::from_le_bytes(scale_bytes);
    let packed = &blob[4..];
    if !scale.is_finite()
        || !(0.0..=64.0).contains(&scale)
        || packed.iter().any(|&trits| trits >= 243)
    {
        return Err("Sparse dictionary tensor has invalid scale or base-three digits".into());
    }
    Ok(Matrix {
        packed,
        scale,
        inputs,
        outputs,
    })
}

fn audit(
    encoder: Matrix<'_>,
    decoder: Matrix<'_>,
    activation: &Activation,
    probes: usize,
) -> Result<Report, String> {
    let mut features = [Feature::default(); MAX_FEATURES];
    let mut sample = [0.0; MAX_WIDTH];
    let mut sparse = [0.0; MAX_FEATURES];
    let mut reconstruction = [0.0; MAX_WIDTH];
    let mut ordering = [0; MAX_FEATURES];
    let scale = f64::from(decoder.scale).powi(2);
    for row in 0..decoder.outputs {
        for (column, stats) in features[..encoder.outputs].iter_mut().enumerate() {
            let value = trit_at(decoder.packed, row * decoder.inputs + column);
            stats.decoder_norm_squared += scale * f64::from(value * value);
        }
    }
    let (mut total_l0, mut squared_error) = (0usize, 0.0_f64);
    let normalizer = 1.0 / (encoder.inputs as f32).sqrt();
    for probe in 0..probes {
        for (coordinate, value) in sample[..encoder.inputs].iter_mut().enumerate() {
            *value = synthetic_sign(probe, coordinate) * normalizer;
        }
        ternary_gemm_cpu(
            &sample,
            encoder.packed,
            encoder.scale,
            encoder.inputs,
            encoder.outputs,
            1,
            0,
            0,
            &mut sparse,
        );
        let active = &mut sparse[..encoder.outputs];
        match activation {
            Activation::TopK(k) => {
                let indices = &mut ordering[..encoder.outputs];
                for (index, slot) in indices.iter_mut().enumerate() {
                    *slot = index;
                }
                indices.sort_unstable_by(|&a, &b| {
                    active[b].total_cmp(&active[a]).then_with(|| a.cmp(&b))
                });
                for &feature in &indices[*k..] {
                    active[feature] = 0.0;
                }
                for value in active.iter_mut() {
                    *value = value.max(0.0);
                }
            }
            Activation::JumpRelu(threshold) => {
                for value in active.iter_mut() {
                    if *value <= *threshold {
                        *value = 0.0;
                    }
                }
            }
        }
        for (&value, stats) in active.iter().zip(&mut features) {
            if value > 0.0 {
                total_l0 += 1;
                stats.count[usize::from(probe >= probes / 2)] += 1;
                stats.total += f64::from(value);
                stats.removal_energy += f64::from(value).powi(2) * stats.decoder_norm_squared;
            }
        }
        ternary_gemm_cpu(
            &sparse,
            decoder.packed,
            decoder.scale,
            decoder.inputs,
            decoder.outputs,
            1,
            0,
            0,
            &mut reconstruction,
        );
        squared_error += sample[..encoder.inputs]
            .iter()
            .zip(&reconstruction)
            .map(|(&expected, &actual)| (f64::from(expected) - f64::from(actual)).powi(2))
            .sum::<f64>();
    }
    let reconstruction_mse = squared_error / (probes * encoder.inputs) as f64;
    if !reconstruction_mse.is_finite() {
        return Err("Nonfinite reconstruction in sparse dictionary audit".into());
    }
    Ok(Report {
        features,
        mean_l0: total_l0 as f64 / probes as f64,
        reconstruction_mse,
    })
}

fn synthetic_sign(probe: usize, coordinate: usize) -> f32 {
    // A fixed public paired-sign protocol supplies probes, never retained activations.
    let mut word = ((probe / 2) as u64).wrapping_mul(0x9E3779B97F4A7C15)
        ^ (coordinate as u64).wrapping_add(0xD1B54A32D192ED03);
    word = (word ^ (word >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    word = (word ^ (word >> 27)).wrapping_mul(0x94D049BB133111EB);
    word ^= word >> 31;
    if ((word & 1) as usize) ^ (probe & 1) == 0 {
        1.0
    } else {
        -1.0
    }
}

fn audit_identity(
    source: &str,
    enc: usize,
    dec: usize,
    rows_first: bool,
    activation: &Activation,
    probes: usize,
) -> String {
    let mut sha = Sha256::new();
    sha.update(CONTEXT.as_bytes());
    sha.update(source.as_bytes());
    for integer in [enc, dec, probes] {
        sha.update((integer as u64).to_le_bytes());
    }
    sha.update([u8::from(rows_first)]);
    match activation {
        Activation::TopK(k) => {
            sha.update([0]);
            sha.update((*k as u64).to_le_bytes());
        }
        Activation::JumpRelu(threshold) => {
            sha.update([1]);
            sha.update(threshold.to_le_bytes());
        }
    }
    format!("urn:sha256:{:x}", sha.finalize())
}

//! Matrix-free offline RIR identification and audio deconvolution.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use acoustics_daw_session::AudioBuffer;
use serde_json::json;
use sha2::{Digest, Sha256};

use super::tract::{seal_pcm, PcmModel};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{count, fields, number, required, text, Args, AssetHost, TypedArtifact, Value};

const MAX_FRAMES: usize = 131_072;
const MAX_RESPONSE_FRAMES: usize = 4096;
const ALGORITHM: &str = "urn:qualia:audio:regularized-full-convolution-cg";

impl AssetHost<'_> {
    pub(crate) fn solve_inverse_rir_prb_and_bss(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "op",
                "observed",
                "reference",
                "lambda_relative",
                "max_iterations",
                "max_relative_residual",
            ],
        )?;
        let op = text(required(args, "op")?)?;
        if !matches!(op, "identify_rir" | "deconvolve") {
            return Err("Inverse acoustics supports identify_rir and deconvolve".into());
        }
        let known = typed_mono(self, required(args, "reference")?)?;
        let observed = typed_mono(self, required(args, "observed")?)?;
        if known.sample_rate != observed.sample_rate {
            return Err(
                "Inverse acoustics requires matching reference and observed sample rates".into(),
            );
        }
        let count_known = known.num_frames();
        let count_observed = observed.num_frames();
        if count_known > count_observed {
            return Err(
                "Observed audio must include the full linear convolution including its tail".into(),
            );
        }
        let count_unknown = count_observed - count_known + 1;
        let response_frames = if op == "identify_rir" {
            count_unknown
        } else {
            count_known
        };
        if response_frames > MAX_RESPONSE_FRAMES {
            return Err("Inverse acoustics response exceeds 4096 taps".into());
        }
        let lambda = number(args, "lambda_relative", 1e-4)?;
        let iterations = count(args, "max_iterations", 32, 128)?;
        let residual_ceiling = args
            .get("max_relative_residual")
            .map(crate::num)
            .transpose()?;
        if !(1e-8..=1.0).contains(&lambda)
            || iterations == 0
            || residual_ceiling.is_some_and(|value| !(0.0..=1.0).contains(&value))
        {
            return Err("Inverse acoustics requires bounded positive regularization, iterations and residual ceiling".into());
        }
        let per_convolution = count_known
            .checked_mul(count_unknown)
            .ok_or("Inverse convolution work overflow")?;
        let work = per_convolution
            .checked_mul(16 + iterations * 8)
            .and_then(|work| work.checked_add((count_observed + count_unknown) * iterations * 12))
            .ok_or("Inverse convolution work overflow")?;
        self.charge(work, 200_000 + count_observed * 24 + count_unknown * 72)?;

        let kernel = known.channels[0]
            .iter()
            .copied()
            .map(f64::from)
            .collect::<Vec<_>>();
        let target = observed.channels[0]
            .iter()
            .copied()
            .map(f64::from)
            .collect::<Vec<_>>();
        let kernel_energy = dot(&kernel, &kernel);
        let target_energy = dot(&target, &target);
        if kernel_energy <= 1e-24 || target_energy <= 1e-24 {
            return Err(
                "Inverse acoustics requires non-silent reference and observed signals".into(),
            );
        }
        let operator = Convolution {
            kernel: &kernel,
            regularization: lambda * kernel_energy,
        };
        let (solution, used_iterations, stationarity) =
            operator.fit(&target, count_unknown, iterations)?;
        let samples = solution.into_iter().map(|x| x as f32).collect::<Vec<_>>();
        if samples.iter().any(|x| !x.is_finite()) {
            return Err("Inverse acoustics output cannot be represented as finite PCM".into());
        }
        let reconstructed = samples.iter().copied().map(f64::from).collect::<Vec<_>>();
        let mut forward = vec![0.0; count_observed];
        operator.forward(&reconstructed, &mut forward);
        let error_energy = forward
            .iter()
            .zip(&target)
            .map(|(p, o)| (p - o).powi(2))
            .sum::<f64>();
        let residual = (error_energy / target_energy).sqrt();
        if residual_ceiling.is_some_and(|ceiling| residual > ceiling) {
            return Err(format!("Inverse acoustics relative reconstruction residual {residual} exceeds the requested ceiling"));
        }
        let purpose = if op == "identify_rir" {
            "identified-causal-room-response"
        } else {
            "recovered-dry-audio"
        };
        let known_sha = pcm_hash(known.sample_rate, &known.channels[0]);
        let observed_sha = pcm_hash(observed.sample_rate, &observed.channels[0]);
        let output_sha = pcm_hash(observed.sample_rate, &samples);
        let mut receipt = json!({
            "operation": op,
            "algorithm": ALGORITHM,
            "model": "single channel, linear time-invariant, synchronized, full linear convolution with no missing tail",
            "known_pcm_sha256": known_sha,
            "observed_pcm_sha256": observed_sha,
            "output_pcm_sha256": output_sha,
            "lambda_relative": lambda,
            "regularization": operator.regularization,
            "max_iterations": iterations,
            "iterations": used_iterations,
            "relative_stationarity": stationarity,
            "relative_reconstruction_residual": residual,
            "sample_rate": observed.sample_rate,
            "response_frames": response_frames,
            "observed_frames": count_observed,
            "output_frames": count_unknown,
        });
        let lineage_sha = format!(
            "{:x}",
            Sha256::digest(serde_json::to_vec(&receipt).map_err(|e| e.to_string())?)
        );
        let lineage = format!("urn:sha256:{lineage_sha}");
        let qac = seal_pcm(
            &samples,
            observed.sample_rate,
            1,
            samples.len() * 4 + 32_768,
            PcmModel {
                channel_order: "mono",
                purpose,
                algorithm: ALGORITHM,
                lineage: &lineage,
            },
        )?;
        let qac_sha = format!("{:x}", Sha256::digest(&qac));
        let qac_iri = format!("urn:sha256:{qac_sha}");
        let mut graph = ReceiptGraph::new("urn:qualia:acoustics:inverse-convolution-receipt");
        graph.fact(
            &qac_iri,
            "http://www.w3.org/ns/prov#wasGeneratedBy",
            &lineage,
        )?;
        graph.fact(
            &lineage,
            "urn:qualia:acoustics:reference-pcm",
            &format!("urn:sha256:{known_sha}"),
        )?;
        graph.fact(
            &lineage,
            "urn:qualia:acoustics:observed-pcm",
            &format!("urn:sha256:{observed_sha}"),
        )?;
        graph.fact(&lineage, "http://purl.org/dc/terms/conformsTo", ALGORITHM)?;
        let q42 = graph.seal(self.limits.resource_bytes)?;
        receipt["lineage_iri"] = json!(lineage);
        receipt["qac_sha256"] = json!(qac_sha);
        let audio =
            self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(AudioBuffer {
                sample_rate: observed.sample_rate,
                channels: vec![samples],
            })))?;
        let qac = self.encode("qac", qac)?;
        let q42 = self.encode("q42", q42)?;
        let receipt = self.encode(
            "json",
            serde_json::to_vec(&receipt).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(Args::from([
            ("audio".into(), audio),
            ("qac".into(), qac),
            ("q42".into(), q42),
            ("receipt".into(), receipt),
        ])))
    }
}

fn typed_mono(host: &AssetHost<'_>, value: &Value) -> Result<Arc<AudioBuffer>, String> {
    let TypedArtifact::AudioMultiChannel(buffer) = host.workspace.typed(value)? else {
        return Err("Inverse acoustics inputs must be typed audio".into());
    };
    if buffer.num_channels() != 1
        || !(1..=MAX_FRAMES).contains(&buffer.num_frames())
        || !(8000..=384_000).contains(&buffer.sample_rate)
        || buffer.channels[0]
            .iter()
            .any(|&sample| !sample.is_finite() || sample.abs() > 128.0)
    {
        return Err("Inverse acoustics requires bounded finite mono PCM at 8–384 kHz".into());
    }
    Ok(Arc::clone(buffer))
}

fn pcm_hash(rate: u32, samples: &[f32]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"qualia:mono-pcm-f32le:v1\0");
    digest.update(rate.to_le_bytes());
    digest.update((samples.len() as u64).to_le_bytes());
    for sample in samples {
        digest.update(sample.to_le_bytes());
    }
    format!("{:x}", digest.finalize())
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}

struct Convolution<'a> {
    kernel: &'a [f64],
    regularization: f64,
}

impl Convolution<'_> {
    fn forward(&self, unknown: &[f64], output: &mut [f64]) {
        output.fill(0.0);
        for (shift, &taps) in unknown.iter().enumerate() {
            for (out, known) in output[shift..][..self.kernel.len()]
                .iter_mut()
                .zip(self.kernel)
            {
                *out += taps * known;
            }
        }
    }

    fn adjoint(&self, observed: &[f64], output: &mut [f64]) {
        for (shift, target) in output.iter_mut().enumerate() {
            *target = dot(self.kernel, &observed[shift..][..self.kernel.len()]);
        }
    }

    fn fit(
        &self,
        observed: &[f64],
        unknown_count: usize,
        max_iter: usize,
    ) -> Result<(Vec<f64>, usize, f64), String> {
        let mut solution = vec![0.0; unknown_count];
        let mut rhs = vec![0.0; unknown_count];
        self.adjoint(observed, &mut rhs);
        let initial = dot(&rhs, &rhs);
        if initial <= 1e-32 {
            return Err("Observed audio has insufficient correlation with the reference".into());
        }
        let mut residual = rhs.clone();
        let mut direction = rhs.clone();
        let mut transformed = vec![0.0; unknown_count];
        let mut forward = vec![0.0; observed.len()];
        let mut squared = initial;
        let mut used = 0;
        // The ridge term makes the normal operator symmetric positive definite.
        for iteration in 1..=max_iter {
            self.forward(&direction, &mut forward);
            self.adjoint(&forward, &mut transformed);
            for (value, &step) in transformed.iter_mut().zip(&direction) {
                *value += self.regularization * step;
            }
            let denominator = dot(&direction, &transformed);
            if !denominator.is_finite() || denominator <= 0.0 {
                return Err(
                    "Inverse acoustics lost positive definiteness in conjugate gradients".into(),
                );
            }
            let step = squared / denominator;
            used = iteration;
            for i in 0..unknown_count {
                solution[i] += step * direction[i];
                residual[i] -= step * transformed[i];
            }
            let next = dot(&residual, &residual);
            if !next.is_finite() {
                return Err("Inverse acoustics encountered a non-finite normal residual".into());
            }
            if next <= initial * 1e-16 {
                break;
            }
            let beta = next / squared;
            for (direction, residual) in direction.iter_mut().zip(&residual) {
                *direction = residual + beta * *direction;
            }
            squared = next;
        }
        self.forward(&solution, &mut forward);
        self.adjoint(&forward, &mut transformed);
        let defect = transformed
            .iter()
            .zip(&solution)
            .zip(&rhs)
            .map(|((a, x), b)| (a + self.regularization * x - b).powi(2))
            .sum::<f64>();
        let stationarity = (defect / initial).sqrt();
        if !stationarity.is_finite() || stationarity > 1e-6 {
            return Err("Inverse acoustics did not meet the normal residual tolerance within its iteration budget".into());
        }
        Ok((solution, used, stationarity))
    }
}

//! Pairwise contrast comparison of registered optical object captures.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::{fields, integer, list, num, required, text, Args, AssetHost, Value};

use super::accessibility::{lattice, LinearPixels};
use super::{digest_iri, encode_hex, parse_hex, review_fact, review_graph, review_number};

const MIN_PAIRS: usize = 128;
const MAX_PAIRS: usize = 4096;
const MAX_CAPTURE_PIXELS: usize = 16 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn verify_microstructure_puf_fuzzy(&mut self, args: &Args) -> Result<Value, String> {
        let mode = text(required(args, "mode")?)?;
        let verifying = match mode {
            "commit_registered_optical_enrollment" => false,
            "compare_registered_optical_capture" => true,
            _ => return Err("Select registered optical object enrollment or comparison".into()),
        };
        fields(
            args,
            &[
                "mode",
                "enrollment_captures",
                "probe_capture",
                "sample_pairs",
                "minimum_contrast",
                "maximum_mismatch_fraction",
                "trusted_enrollment_sha256",
            ],
        )?;
        let handles = list(required(args, "enrollment_captures")?)?;
        let pairs = list(required(args, "sample_pairs")?)?;
        if !(2..=8).contains(&handles.len()) || !(MIN_PAIRS..=MAX_PAIRS).contains(&pairs.len()) {
            return Err("Object comparison requires 2 to 8 registered enrollment captures and 128 to 4,096 disjoint sample pairs".into());
        }
        let min_contrast = num(required(args, "minimum_contrast")?)?;
        let max_mismatch = num(required(args, "maximum_mismatch_fraction")?)?;
        if !(0.0001..=1.0).contains(&min_contrast) || !(0.0..=0.3).contains(&max_mismatch) {
            return Err(
                "Optical contrast must be in [0.0001, 1] and maximum mismatch in [0, 0.3]".into(),
            );
        }
        let first = lattice(self, &handles[0])?;
        let dimensions = [first.width, first.height];
        let n_captures = handles.len() + usize::from(verifying);
        let work = LinearPixels::new(first)?
            .len()
            .checked_mul(n_captures)
            .filter(|&pixels| pixels <= MAX_CAPTURE_PIXELS)
            .ok_or("Registered optical capture exceeds its aggregate pixel budget")?;
        self.charge(work * 40 + pairs.len() * n_captures * 12, pairs.len() * 192)?;
        let indices = sample_indices(pairs, dimensions)?;
        let mut commitment = Sha256::new();
        commitment.update(b"QUALIA-REGISTERED-OPTICAL-OBJECT-V1\0");
        commitment.update(dimensions[0].to_le_bytes());
        commitment.update(dimensions[1].to_le_bytes());
        commitment.update([handles.len() as u8]);
        commitment.update((indices.len() as u32).to_le_bytes());
        commitment.update(min_contrast.to_le_bytes());
        commitment.update(max_mismatch.to_le_bytes());
        for &[a, b] in &indices {
            commitment.update((a as u64).to_le_bytes());
            commitment.update((b as u64).to_le_bytes());
        }
        let mut stable = vec![None; pairs.len()];
        let mut capture_digests = BTreeSet::new();
        for (number, handle) in handles.iter().enumerate() {
            self.charge(0, 0)?;
            let pixels = registered_capture(self, handle, dimensions)?;
            let digest = capture_sha256(&pixels)?;
            if !capture_digests.insert(digest) {
                return Err("Enrollment captures must be distinct decoded optical samples".into());
            }
            commitment.update(digest);
            for (sample, &[a, b]) in stable.iter_mut().zip(&indices) {
                let sign = contrast_sign(&pixels, a, b, min_contrast)?;
                if number == 0 {
                    *sample = sign;
                } else if *sample != sign {
                    *sample = None;
                }
            }
        }
        let stable_pairs = stable.iter().filter(|sample| sample.is_some()).count();
        if stable_pairs < MIN_PAIRS || stable_pairs * 4 < pairs.len() * 3 {
            return Err("Enrollment needs at least 128 consistently contrasted pairs covering 75% of its sample layout".into());
        }
        let enrollment_digest: [u8; 32] = commitment.finalize().into();
        if !verifying {
            if args.contains_key("probe_capture") || args.contains_key("trusted_enrollment_sha256")
            {
                return Err("Probe and trusted enrollment digest apply to comparison mode".into());
            }
            return Ok(Value::Record(Args::from([
                (
                    "enrollment_sha256".into(),
                    Value::String(encode_hex(&enrollment_digest)),
                ),
                ("stable_pairs".into(), Value::U64(stable_pairs as u64)),
            ])));
        }
        let trusted_digest = parse_hex::<32>(
            text(required(args, "trusted_enrollment_sha256")?)?,
            "trusted_enrollment_sha256",
        )?;
        if trusted_digest != enrollment_digest {
            return Err(
                "Optical enrollment and challenge do not match the trusted commitment".into(),
            );
        }
        let probe = registered_capture(self, required(args, "probe_capture")?, dimensions)?;
        if capture_digests.contains(&capture_sha256(&probe)?) {
            return Err("The decoded probe exactly duplicates an enrollment capture".into());
        }
        let mut mismatches = 0;
        let mut unresolved = 0;
        for (sample, &[a, b]) in stable.into_iter().zip(&indices) {
            let Some(enrolled) = sample else { continue };
            match contrast_sign(&probe, a, b, min_contrast)? {
                Some(probed) => mismatches += usize::from(enrolled != probed),
                None => unresolved += 1,
            }
        }
        let decoded = stable_pairs - unresolved;
        if decoded == 0 {
            return Err("The probe provides no distinguishable enrolled sample pairs".into());
        }
        let worst_mismatch = (mismatches + unresolved) as f64 / stable_pairs as f64;
        let matches = worst_mismatch <= max_mismatch;
        let mut receipt = review_graph("RegisteredOpticalObjectComparison")?;
        review_fact(
            &mut receipt,
            "enrollmentDigest",
            &digest_iri(&enrollment_digest),
        )?;
        review_number(&mut receipt, "stablePairs", stable_pairs)?;
        review_number(&mut receipt, "decodedProbePairs", decoded)?;
        review_number(&mut receipt, "observedMismatches", mismatches)?;
        review_number(&mut receipt, "unresolvedProbePairs", unresolved)?;
        review_fact(
            &mut receipt,
            "referenceMatch",
            if matches {
                "urn:qualia:boolean:true"
            } else {
                "urn:qualia:boolean:false"
            },
        )?;
        Ok(Value::Record(Args::from([
            ("matches_reference".into(), Value::Bool(matches)),
            ("stable_pairs".into(), Value::U64(stable_pairs as u64)),
            (
                "unresolved_probe_pairs".into(),
                Value::U64(unresolved as u64),
            ),
            (
                "fractional_hamming_distance".into(),
                Value::F64(mismatches as f64 / decoded as f64),
            ),
            (
                "worst_case_mismatch_fraction".into(),
                Value::F64(worst_mismatch),
            ),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }
}

fn sample_indices(items: &[Value], [width, height]: [u32; 2]) -> Result<Vec<[usize; 2]>, String> {
    let mut used = BTreeSet::new();
    items
        .iter()
        .map(|item| {
            let [x0, y0, x1, y1] = list(item)? else {
                return Err("Sample pair requires [x0, y0, x1, y1]".into());
            };
            let mut indices = [0; 2];
            for (index, [x, y]) in indices.iter_mut().zip([[x0, y0], [x1, y1]]) {
                let x = integer(x, "sample x")?;
                let y = integer(y, "sample y")?;
                if x >= width as usize || y >= height as usize {
                    return Err("Optical sample point is outside the registered geometry".into());
                }
                *index = y * width as usize + x;
                if !used.insert(*index) {
                    return Err("Optical sample pairs must use disjoint points".into());
                }
            }
            Ok(indices)
        })
        .collect()
}

fn registered_capture<'host>(
    host: &'host AssetHost<'_>,
    input: &Value,
    dimensions: [u32; 2],
) -> Result<LinearPixels<'host>, String> {
    let image = lattice(host, input)?;
    if [image.width, image.height] != dimensions {
        return Err("All optical captures must share the same registered geometry".into());
    }
    LinearPixels::new(image)
}

fn capture_sha256(pixels: &LinearPixels<'_>) -> Result<[u8; 32], String> {
    let mut hash = Sha256::new();
    for pixel in pixels.iter() {
        for component in pixel? {
            // RGB is admitted F32 and alpha is always opaque. Canonicalize
            // signed zero so equivalent decoded samples share a commitment.
            let value = if component == 0.0 {
                0.0
            } else {
                component as f32
            };
            hash.update(value.to_le_bytes());
        }
    }
    Ok(hash.finalize().into())
}

fn contrast_sign(
    pixels: &LinearPixels<'_>,
    a: usize,
    b: usize,
    min_gap: f64,
) -> Result<Option<bool>, String> {
    let luminance = |[r, g, b]: [f64; 3]| 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let contrast = luminance(pixels.pixel(a)?) - luminance(pixels.pixel(b)?);
    Ok((contrast.abs() >= min_gap).then_some(contrast > 0.0))
}

//! Verification of native paired QBDL publication proofs against offline public pins.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    fields, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, required, text, Args,
    AssetHost, Value,
};
use qualia_core_db::{
    bundle::stream::{
        descriptor::{decode_body, domain_digest, DigestDomain, HASH_SCRATCH_BYTES, MAX_ENCODED},
        policy::PUBLICATION_PURPOSE,
    },
    crypto::network::{
        cose::{self, Binding, PairedPublicKeys},
        types::{ED25519_PK_LEN, ML_DSA_65_PK_LEN},
    },
};
use sha2::{Digest, Sha256};
use std::fmt::Write;

impl AssetHost<'_> {
    pub(crate) fn prove_zk_deontic_pq_and_sanctuary(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "descriptor_key",
                "proof_key",
                "ml_dsa_65_key",
                "ed25519_key",
                "expected_descriptor_id",
                "pinned_ml_dsa_65_id",
                "pinned_ed25519_id",
                "controller_epoch",
                "audience",
            ],
        )?;
        if text(required(args, "mode")?)? != "verify_pinned_qbdl_dual_proof" {
            return Err(
                "Expected verify_pinned_qbdl_dual_proof with retained public inputs".into(),
            );
        }
        let controller_epoch = match required(args, "controller_epoch")? {
            Value::U64(n) if *n > 0 => *n,
            Value::I64(n) if *n > 0 => *n as u64,
            _ => return Err("controller_epoch requires an exact positive integer".into()),
        };
        let audience = text(required(args, "audience")?)?;
        if !(1..=cose::MAX_AUDIENCE_BYTES).contains(&audience.len())
            || audience.chars().any(char::is_control)
        {
            return Err("Provide a bounded, nonempty UTF-8 publication audience".into());
        }
        self.charge(
            100_000,
            HASH_SCRATCH_BYTES + core::mem::size_of::<cose::Workspace>(),
        )?;
        let descriptor = retained(self, args, "descriptor_key", 1, MAX_ENCODED)?;
        let proof = retained(self, args, "proof_key", 1, cose::MAX_ENVELOPE_BYTES)?;
        let ml_dsa_65 = retained(
            self,
            args,
            "ml_dsa_65_key",
            ML_DSA_65_PK_LEN,
            ML_DSA_65_PK_LEN,
        )?;
        let ed25519 = retained(self, args, "ed25519_key", ED25519_PK_LEN, ED25519_PK_LEN)?;
        let keys = PairedPublicKeys {
            ml_dsa_65: ml_dsa_65
                .try_into()
                .map_err(|_| "Invalid ML-DSA-65 public-key extent")?,
            ed25519: ed25519
                .try_into()
                .map_err(|_| "Invalid Ed25519 public-key extent")?,
        };
        let key_ids = keys.key_ids();
        let ml_dsa_65_id = lowercase_hex(&key_ids.ml_dsa_65);
        let ed25519_id = lowercase_hex(&key_ids.ed25519);
        if ml_dsa_65_id != text(required(args, "pinned_ml_dsa_65_id")?)?
            || ed25519_id != text(required(args, "pinned_ed25519_id")?)?
        {
            return Err("Both canonical QBDL public-key IDs must match the caller pins".into());
        }

        let body = decode_body(descriptor)
            .map_err(|err| format!("Invalid QASD publication descriptor: {err}"))?;
        let mut framing = vec![0; HASH_SCRATCH_BYTES];
        let descriptor_id =
            domain_digest(DigestDomain::Body, body.encoded_bytes(), &mut framing)
                .map_err(|err| format!("Unable to commit the publication descriptor: {err}"))?;
        let native_id = lowercase_hex(descriptor_id.as_bytes());
        if native_id != text(required(args, "expected_descriptor_id")?)? {
            return Err("QASD descriptor ID differs from the caller's expected publication".into());
        }
        let context_id = domain_digest(
            DigestDomain::Context,
            body.context().encoded_bytes(),
            &mut framing,
        )
        .map_err(|err| format!("Unable to commit the publication context: {err}"))?;
        if context_id != body.context_id() {
            return Err("QASD publication descriptor has an invalid context commitment".into());
        }
        cose::verify_pair(
            proof,
            &Binding {
                descriptor_digest: descriptor_id.as_bytes(),
                key_ids,
                controller_epoch,
                purpose: PUBLICATION_PURPOSE,
                audience: audience.as_bytes(),
            },
            &keys,
            &mut cose::Workspace::new(),
        )
        .map_err(|_| "Paired ML-DSA-65 and Ed25519 signatures failed QBDL binding verification")?;

        let proof_sha256 = format!("{:x}", Sha256::digest(proof));
        let context_id = lowercase_hex(context_id.as_bytes());
        let subject = format!("urn:qualia:qbdl-dual-verification:sha256:{proof_sha256}");
        let mut receipt = ReceiptGraph::new(&subject);
        for (predicate, object) in [
            (
                "verificationProfile",
                format!("urn:qualia:crypto:{}", cose::PROFILE),
            ),
            ("verifiedDescriptor", format!("urn:sha384:{native_id}")),
            ("verifiedContext", format!("urn:sha384:{context_id}")),
            ("verifiedProof", format!("urn:sha256:{proof_sha256}")),
            (
                "pinnedMlDsa65KeyId",
                format!("urn:qualia:qbdl-key:sha384:{ml_dsa_65_id}"),
            ),
            (
                "pinnedEd25519KeyId",
                format!("urn:qualia:qbdl-key:sha384:{ed25519_id}"),
            ),
            (
                "expectedControllerEpoch",
                format!("urn:qualia:controller-epoch:{controller_epoch}"),
            ),
            (
                "expectedAudienceDigest",
                format!("urn:sha256:{:x}", Sha256::digest(audience.as_bytes())),
            ),
            (
                "verificationScope",
                "urn:qualia:crypto:qbdl-publication-signature-binding".into(),
            ),
        ] {
            receipt.fact(&subject, &format!("urn:qualia:crypto:{predicate}"), &object)?;
        }
        let receipt = receipt.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        Ok(Value::Record(Args::from([
            ("verified".into(), Value::Bool(true)),
            ("descriptor_id".into(), Value::String(native_id)),
            ("context_id".into(), Value::String(context_id)),
            ("proof_sha256".into(), Value::String(proof_sha256)),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }
}

fn retained<'a>(
    host: &'a AssetHost<'_>,
    args: &Args,
    arg: &str,
    lower: usize,
    upper: usize,
) -> Result<&'a [u8], String> {
    let key = text(required(args, arg)?)?;
    let input = host
        .inputs
        .iter()
        .find(|input| input.key == key)
        .ok_or_else(|| format!("Unknown retained public input for {arg}"))?;
    if input.kind != "bin" || !(lower..=upper).contains(&input.bytes.len()) {
        return Err(format!(
            "{arg} requires a retained bin input with {lower} to {upper} bytes"
        ));
    }
    Ok(&input.bytes)
}

pub(super) fn lowercase_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(out, "{byte:02x}").expect("writing to a String cannot fail");
    }
    out
}

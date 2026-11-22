//! Detached, time-bounded ML-DSA-65 work graph co-attestation.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::fiduciary_crypto::{
    CryptoContext, MlDsaSignature, MlDsaSigner, ML_DSA_PUBLIC_KEY_SIZE, ML_DSA_SIGNATURE_SIZE,
};
use qualia_core_db::net::qdnf::fabric::instruments::{
    CoAttestationBundle, ProvenanceMonomial, MAX_ATTESTATION_MEMBERS, MAX_MONOMIAL_GENERATORS,
};
use qualia_core_db::q42_volume::WasmSliceQ42Reader;
use qualia_core_db::q_hash;

use crate::{fields, integer, list, record, required, text, Args, AssetHost, Value};

use super::credential::exact_u64;
use super::{
    digest_iri, encode_hex, parse_hex, retained, review_fact, review_graph, review_number, sha256,
};

const DOMAIN: &str = "urn:qualia:production:work-graph";
const PURPOSE: &str = "detached-quorum-v1";
const MAX_WORK_GRAPH: usize = 16 * 1024 * 1024;

struct Member {
    key: [u8; ML_DSA_PUBLIC_KEY_SIZE],
    key_digest: [u8; 32],
    factors: Vec<[u8; 32]>,
    signature: Option<MlDsaSignature>,
}

impl AssetHost<'_> {
    pub(crate) fn seal_qecp_and_work_graph(&mut self, args: &Args) -> Result<Value, String> {
        let mode = text(required(args, "mode")?)?;
        let verifying = match mode {
            "prepare_detached_work_graph_quorum" => false,
            "verify_detached_work_graph_quorum" => true,
            _ => return Err("Select a detached work graph quorum mode".into()),
        };
        fields(
            args,
            &[
                "mode",
                "asset",
                "work_graph",
                "members",
                "required_signatures",
                "not_before_epoch",
                "expires_at_epoch",
                "trusted_policy_sha256",
            ],
        )?;
        let asset_input = required(args, "asset")?;
        let graph_input = required(args, "work_graph")?;
        let sized = {
            let asset = retained(self, asset_input)?;
            let graph = retained(self, graph_input)?;
            if !matches!(asset.kind.as_str(), "pdf" | "hmc" | "10d")
                || asset.bytes.is_empty()
                || graph.kind != "q42"
                || !(1..=MAX_WORK_GRAPH).contains(&graph.bytes.len())
            {
                return Err(
                    "Provide a retained PDF, HMC, or 10d and a Q42 work graph up to 16 MiB".into(),
                );
            }
            asset.bytes.len().saturating_add(graph.bytes.len())
        };
        let members = list(required(args, "members")?)?;
        let threshold = integer(
            required(args, "required_signatures")?,
            "required_signatures",
        )?;
        if !(2..=MAX_ATTESTATION_MEMBERS).contains(&members.len())
            || !(2..=members.len()).contains(&threshold)
        {
            return Err(
                "A work graph quorum requires 2 to 8 pinned members and M in [2, N]".into(),
            );
        }
        let not_before = exact_u64(required(args, "not_before_epoch")?)?;
        let expires_at = exact_u64(required(args, "expires_at_epoch")?)?;
        if not_before >= expires_at {
            return Err("The work graph quorum needs an increasing validity interval".into());
        }
        self.charge(
            sized
                .saturating_mul(4)
                .saturating_add(members.len() * 200_000),
            256 * 1024,
        )?;
        let graph_bytes = &retained(self, graph_input)?.bytes;
        let reader = WasmSliceQ42Reader::open(graph_bytes).map_err(|_| "Invalid Q42 work graph")?;
        if !reader.is_unified_volume() || reader.block_count() == 0 {
            return Err("The work graph must be a headered, nonempty Q42 volume".into());
        }
        let graph_digest = sha256(graph_bytes);
        let asset_digest = sha256(&retained(self, asset_input)?.bytes);
        let members = parse_members(members, verifying)?;
        let policy = policy_digest(&members, threshold);
        let mut statement = b"QUALIA-WORK-GRAPH-QUORUM-V1\0".to_vec();
        statement.extend(asset_digest);
        statement.extend(graph_digest);
        statement.extend(policy);
        statement.extend(not_before.to_le_bytes());
        statement.extend(expires_at.to_le_bytes());
        if !verifying {
            if args.contains_key("trusted_policy_sha256") {
                return Err("trusted_policy_sha256 applies to verification mode".into());
            }
            return Ok(Value::Record(Args::from([
                (
                    "statement_hex".into(),
                    Value::String(encode_hex(&statement)),
                ),
                ("policy_sha256".into(), Value::String(encode_hex(&policy))),
                ("signature_domain".into(), Value::String(DOMAIN.into())),
                ("signature_purpose".into(), Value::String(PURPOSE.into())),
                ("signature_timestamp".into(), Value::U64(not_before)),
                (
                    "signature_nonce".into(),
                    Value::String(encode_hex(&graph_digest)),
                ),
            ])));
        }
        let trusted_policy = parse_hex::<32>(
            text(required(args, "trusted_policy_sha256")?)?,
            "trusted_policy_sha256",
        )?;
        if trusted_policy != policy {
            return Err("Work graph roster does not match the trusted policy digest".into());
        }
        let now = trusted_host_epoch()?;
        if !(not_before..expires_at).contains(&now) {
            return Err("Work graph quorum is outside its validity interval".into());
        }
        let context = CryptoContext {
            domain: DOMAIN.into(),
            purpose: PURPOSE.into(),
            timestamp: not_before,
            nonce: graph_digest,
        };
        let mut co_attestation = CoAttestationBundle {
            claim_digest_hash: q_hash(&digest_iri(&sha256(&statement))),
            not_before,
            expires_at,
            monomials: [ProvenanceMonomial::EMPTY; MAX_ATTESTATION_MEMBERS],
            member_count: 0,
        };
        for member in members {
            let Some(signature) = member.signature else {
                continue;
            };
            if !MlDsaSigner::verify_with_public(&member.key, &statement, &signature, &context)
                .map_err(|_| "Invalid ML-DSA-65 co-attestation")?
            {
                return Err("Work graph co-attestation signature verification failed".into());
            }
            let mut generators = [0; MAX_MONOMIAL_GENERATORS];
            generators[0] = q_hash(&digest_iri(&member.key_digest));
            for (slot, factor) in generators[1..].iter_mut().zip(&member.factors) {
                *slot = q_hash(&digest_iri(factor));
            }
            co_attestation.monomials[usize::from(co_attestation.member_count)] =
                ProvenanceMonomial::from_generators(&generators[..1 + member.factors.len()])
                    .map_err(|_| "Invalid co-attestation provenance factor")?;
            co_attestation.member_count += 1;
        }
        let verified = co_attestation.verify_independent_quorum(now, threshold)
            .map_err(|_| "Work graph signatures did not establish an independent quorum under the pinned roster")?;
        let mut receipt = review_graph("DetachedWorkGraphQuorum")?;
        review_fact(
            &mut receipt,
            "retainedAssetDigest",
            &digest_iri(&asset_digest),
        )?;
        review_fact(&mut receipt, "workGraphDigest", &digest_iri(&graph_digest))?;
        review_fact(&mut receipt, "trustedPolicyDigest", &digest_iri(&policy))?;
        review_fact(
            &mut receipt,
            "quorumStatementDigest",
            &digest_iri(&sha256(&statement)),
        )?;
        review_number(&mut receipt, "verifiedSignatures", verified)?;
        Ok(Value::Record(Args::from([
            ("verified_signatures".into(), Value::U64(verified as u64)),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn trusted_host_epoch() -> Result<u64, String> {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|_| "Host wall clock predates the Unix epoch".into())
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn trusted_host_epoch() -> Result<u64, String> {
    Err("Detached work graph quorum verification requires a native host clock".into())
}

fn parse_members(items: &[Value], verifying: bool) -> Result<Vec<Member>, String> {
    let mut members: Vec<Member> = Vec::with_capacity(items.len());
    for item in items {
        let item = record(item)?;
        fields(
            item,
            &[
                "public_key_hex",
                "independence_factor_sha256",
                "signature_hex",
            ],
        )?;
        let key = parse_hex::<ML_DSA_PUBLIC_KEY_SIZE>(
            text(required(item, "public_key_hex")?)?,
            "public_key_hex",
        )?;
        let key_digest = sha256(&key);
        let factors = list(required(item, "independence_factor_sha256")?)?;
        if !(1..MAX_MONOMIAL_GENERATORS).contains(&factors.len()) {
            return Err(
                "Declare one to three trusted independence factor digests per quorum member".into(),
            );
        }
        let mut factors = factors
            .iter()
            .map(|value| parse_hex::<32>(text(value)?, "independence_factor_sha256"))
            .collect::<Result<Vec<_>, _>>()?;
        factors.sort_unstable();
        if factors.windows(2).any(|pair| pair[0] == pair[1])
            || factors.contains(&[0; 32])
            || factors.contains(&key_digest)
        {
            return Err(
                "Quorum factor digests must be distinct, nonzero, and separate from the key digest"
                    .into(),
            );
        }
        let signature = match item.get("signature_hex") {
            Some(value) if verifying => Some(MlDsaSignature {
                sig_bytes: parse_hex::<ML_DSA_SIGNATURE_SIZE>(text(value)?, "signature_hex")?
                    .to_vec(),
            }),
            Some(_) => return Err("The statement preparation route takes unsigned members".into()),
            None => None,
        };
        members.push(Member {
            key,
            key_digest,
            factors,
            signature,
        });
    }
    members.sort_unstable_by_key(|member| member.key_digest);
    if members
        .windows(2)
        .any(|pair| pair[0].key_digest == pair[1].key_digest)
    {
        return Err("Duplicate public keys cannot supply distinct co-attestations".into());
    }
    Ok(members)
}

fn policy_digest(members: &[Member], threshold: usize) -> [u8; 32] {
    let mut policy = b"QUALIA-WORK-GRAPH-ROSTER-V1\0".to_vec();
    policy.extend([members.len() as u8, threshold as u8]);
    for member in members {
        policy.extend(member.key_digest);
        policy.push(member.factors.len() as u8);
        for factor in &member.factors {
            policy.extend(factor);
        }
    }
    sha256(&policy)
}

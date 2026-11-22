//! Native ML-DSA-65 claim-graph preparation and pinned verification.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::fiduciary_crypto::{
    CryptoContext, MlDsaVcProof, ML_DSA_PUBLIC_KEY_SIZE, ML_DSA_SIGNATURE_SIZE,
};
use qualia_core_db::{q_hash, NQuin};

use crate::{fields, list, required, text, Args, AssetHost, Value};

use super::{
    digest_iri, encode_hex, parse_hex, retained, review_fact, review_graph, review_number, sha256,
};

const DOMAIN: &str = "urn:qualia:production:vc";
const PURPOSE: &str = "retained-asset-v1";
const FRAGMENTS: usize = ML_DSA_SIGNATURE_SIZE.div_ceil(8);
const HEAD_PREDICATE: u64 = q_hash("vc:proof/mldsa");
const FRAGMENT_PREDICATE: u64 = q_hash("vc:proof/mldsa/frag");
const DIGEST_PREDICATES: [u64; 4] = [
    q_hash("urn:qualia:vc:asset-sha256-le-word:0"),
    q_hash("urn:qualia:vc:asset-sha256-le-word:1"),
    q_hash("urn:qualia:vc:asset-sha256-le-word:2"),
    q_hash("urn:qualia:vc:asset-sha256-le-word:3"),
];
const MAX_CLAIMS: usize = 2048;

struct CredentialBinding<'args> {
    issuer: &'args str,
    issuer_hash: u64,
    asset_digest: [u8; 32],
    context: CryptoContext,
}

impl AssetHost<'_> {
    pub(crate) fn issue_pq_vc_and_agent_state(&mut self, args: &Args) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "prepare_mldsa65_asset_claims" => self.prepare_credential(args),
            "verify_mldsa65_asset_claims" => self.verify_credential(args),
            _ => Err(
                "Select ML-DSA-65 retained asset claim preparation or pinned verification".into(),
            ),
        }
    }

    fn prepare_credential(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "asset",
                "issuer",
                "additional_claims",
                "signature_timestamp",
                "signature_nonce",
            ],
        )?;
        let supplied = args
            .get("additional_claims")
            .map(list)
            .transpose()?
            .unwrap_or_default();
        if supplied.len() + DIGEST_PREDICATES.len() > MAX_CLAIMS {
            return Err("The native credential exceeds its 2,048-claim bound".into());
        }
        let CredentialBinding {
            issuer_hash,
            asset_digest,
            context,
            ..
        } = self.credential_binding(args, supplied.len() * 160, 8192)?;
        let mut claims = Vec::with_capacity(DIGEST_PREDICATES.len() + supplied.len());
        for (predicate, word) in DIGEST_PREDICATES
            .into_iter()
            .zip(asset_digest.chunks_exact(8))
        {
            let object = u64::from_le_bytes(word.try_into().map_err(|_| "Invalid SHA-256 word")?);
            claims.push(NQuin {
                subject: issuer_hash,
                predicate,
                object,
                context: issuer_hash,
                metadata: 0,
                parity: NQuin::calculate_parity(issuer_hash, predicate, object, issuer_hash, 0),
            });
        }
        for value in supplied {
            let claim = parse_quin(value)?;
            if DIGEST_PREDICATES.contains(&claim.predicate) {
                return Err(
                    "The asset binding predicates are reserved for the retained asset digest"
                        .into(),
                );
            }
            claims.push(claim);
        }
        let claims = claims
            .into_iter()
            .map(|quin| {
                Value::List(
                    [
                        quin.subject,
                        quin.predicate,
                        quin.object,
                        quin.context,
                        quin.metadata,
                        quin.parity,
                    ]
                    .into_iter()
                    .map(Value::U64)
                    .collect(),
                )
            })
            .collect();
        Ok(Value::Record(Args::from([
            ("claims".into(), Value::List(claims)),
            ("issuer_q_hash".into(), Value::U64(issuer_hash)),
            (
                "asset_digest_iri".into(),
                Value::String(digest_iri(&asset_digest)),
            ),
            ("signature_domain".into(), Value::String(context.domain)),
            ("signature_purpose".into(), Value::String(context.purpose)),
            ("signature_timestamp".into(), Value::U64(context.timestamp)),
            (
                "signature_nonce".into(),
                Value::String(encode_hex(&context.nonce)),
            ),
        ])))
    }

    fn verify_credential(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "asset",
                "issuer",
                "public_key_hex",
                "issuer_key_sha256",
                "claims",
                "proof_head",
                "proof_fragments",
                "signature_timestamp",
                "signature_nonce",
            ],
        )?;
        let claims = list(required(args, "claims")?)?;
        let fragments = list(required(args, "proof_fragments")?)?;
        if !(DIGEST_PREDICATES.len()..=MAX_CLAIMS).contains(&claims.len())
            || fragments.len() != FRAGMENTS
        {
            return Err("Provide 4 to 2,048 claims and exactly 414 ML-DSA-65 fragments".into());
        }
        let CredentialBinding {
            issuer,
            issuer_hash,
            asset_digest,
            context,
        } = self.credential_binding(args, (claims.len() + FRAGMENTS) * 160, 400_000)?;

        let pk = parse_hex::<ML_DSA_PUBLIC_KEY_SIZE>(
            text(required(args, "public_key_hex")?)?,
            "public_key_hex",
        )?;
        let key_pin = parse_hex::<32>(
            text(required(args, "issuer_key_sha256")?)?,
            "issuer_key_sha256",
        )?;
        if sha256(&pk) != key_pin {
            return Err("ML-DSA public key does not match the issuer key pin".into());
        }
        let claims = claims
            .iter()
            .map(parse_quin)
            .collect::<Result<Vec<_>, _>>()?;
        match_asset_digest(&claims, issuer_hash, &asset_digest)?;
        let head = parse_quin(required(args, "proof_head")?)?;
        let proof = read_proof(issuer_hash, head, fragments)?;
        if !proof
            .verify_vc_mldsa(&claims, &pk, &context)
            .map_err(|_| "Unable to verify native ML-DSA-65 claim graph")?
        {
            return Err("ML-DSA-65 claim graph signature verification failed".into());
        }

        let mut graph = review_graph("PinnedNativeMlDsa65ClaimGraph")?;
        review_fact(&mut graph, "issuer", issuer)?;
        review_fact(&mut graph, "issuerKeyDigest", &digest_iri(&key_pin))?;
        review_fact(
            &mut graph,
            "retainedAssetDigest",
            &digest_iri(&asset_digest),
        )?;
        review_number(&mut graph, "verifiedClaims", claims.len())?;
        Ok(Value::Record(Args::from([
            ("verified".into(), Value::Bool(true)),
            (
                "asset_digest_iri".into(),
                Value::String(digest_iri(&asset_digest)),
            ),
            ("receipt".into(), self.admit_review(graph)?),
        ])))
    }

    fn credential_binding<'args>(
        &mut self,
        args: &'args Args,
        scratch: usize,
        overhead: usize,
    ) -> Result<CredentialBinding<'args>, String> {
        let issuer = text(required(args, "issuer")?)?;
        if issuer.len() > 512
            || !issuer.strip_prefix("did:").is_some_and(|id| {
                id.split_once(':').is_some_and(|(method, value)| {
                    !method.is_empty()
                        && !value.is_empty()
                        && method
                            .bytes()
                            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
                }) && !id
                    .chars()
                    .any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
            })
            || q_hash(issuer) == 0
        {
            return Err("issuer must be a bounded DID".into());
        }
        let asset_handle = required(args, "asset")?;
        let asset_size = {
            let asset = retained(self, asset_handle)?;
            if !matches!(asset.kind.as_str(), "q42" | "hmc" | "10d") || asset.bytes.is_empty() {
                return Err("Provide a nonempty retained Q42, HMC, or 10d asset".into());
            }
            asset.bytes.len()
        };
        let timestamp = exact_u64(required(args, "signature_timestamp")?)?;
        let nonce = parse_hex::<32>(text(required(args, "signature_nonce")?)?, "signature_nonce")?;
        self.charge(
            asset_size.saturating_mul(4).saturating_add(overhead),
            scratch,
        )?;
        Ok(CredentialBinding {
            issuer,
            issuer_hash: q_hash(issuer),
            asset_digest: sha256(&retained(self, asset_handle)?.bytes),
            // The native proof envelope carries the compact issuer q_hash.
            // Bind the complete DID as well through the cryptographic context.
            context: CryptoContext {
                domain: format!("{DOMAIN}:{}", encode_hex(&sha256(issuer.as_bytes()))),
                purpose: PURPOSE.into(),
                timestamp,
                nonce,
            },
        })
    }
}

fn match_asset_digest(claims: &[NQuin], issuer: u64, digest: &[u8; 32]) -> Result<(), String> {
    for (predicate, word) in DIGEST_PREDICATES.into_iter().zip(digest.chunks_exact(8)) {
        let mut matches = claims.iter().filter(|claim| {
            claim.subject == issuer && claim.predicate == predicate && claim.context == issuer
        });
        let claim = matches
            .next()
            .ok_or("Signed asset SHA-256 word is missing")?;
        let expected = u64::from_le_bytes(word.try_into().map_err(|_| "Invalid SHA-256 word")?);
        if claim.object != expected || claim.metadata != 0 || matches.next().is_some() {
            return Err("The signed asset SHA-256 binding is ambiguous or does not match".into());
        }
    }
    Ok(())
}

fn read_proof(issuer: u64, head: NQuin, fragments: &[Value]) -> Result<MlDsaVcProof, String> {
    let expected_encoding = (ML_DSA_SIGNATURE_SIZE as u64) << 32 | FRAGMENTS as u64;
    if head.subject != issuer
        || head.context != issuer
        || head.predicate != HEAD_PREDICATE
        || head.object != expected_encoding
    {
        return Err("Invalid ML-DSA proof header".into());
    }
    let mut seen = [false; FRAGMENTS];
    let mut fragment_quins = Vec::with_capacity(FRAGMENTS);
    for item in fragments {
        let fragment = parse_quin(item)?;
        let index = (fragment.metadata >> 32) as usize;
        let count = fragment.metadata as u32 as usize;
        if index >= FRAGMENTS
            || count != FRAGMENTS
            || seen[index]
            || fragment.subject != issuer
            || fragment.context != issuer
            || fragment.predicate != FRAGMENT_PREDICATE
        {
            return Err("Invalid or duplicate ML-DSA proof fragment".into());
        }
        let width = (ML_DSA_SIGNATURE_SIZE - index * 8).min(8);
        if width < 8 && fragment.object >> (width * 8) != 0 {
            return Err("Nonzero padding in ML-DSA proof fragment".into());
        }
        seen[index] = true;
        fragment_quins.push(fragment);
    }
    Ok(MlDsaVcProof {
        head_quin: head,
        fragment_quins,
    })
}

fn parse_quin(value: &Value) -> Result<NQuin, String> {
    let [s, p, o, c, m, parity] = list(value)? else {
        return Err("Expected a six-column NQuin".into());
    };
    let quin = NQuin {
        subject: exact_u64(s)?,
        predicate: exact_u64(p)?,
        object: exact_u64(o)?,
        context: exact_u64(c)?,
        metadata: exact_u64(m)?,
        parity: exact_u64(parity)?,
    };
    if !quin.verify_ecc_parity() {
        return Err("Invalid NQuin parity in signed credential".into());
    }
    Ok(quin)
}

pub(super) fn exact_u64(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(word) => Ok(*word),
        Value::I64(word) => u64::try_from(*word).map_err(|_| "Expected an unsigned integer".into()),
        _ => Err("Expected an exact unsigned integer".into()),
    }
}

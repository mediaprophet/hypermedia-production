//! Offline timestamp imprint screening and pinned native time-witness verification.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use discourse_pdf_compliance::{verify_rfc3161_timestamp, TimestampToken};
use qualia_core_db::fiduciary_crypto::{
    CryptoContext, MlDsaSignature, MlDsaSigner, ML_DSA_PUBLIC_KEY_SIZE, ML_DSA_SIGNATURE_SIZE,
};

use crate::{fields, required, text, Args, AssetHost, Value};

use super::credential::exact_u64;
use super::{digest_iri, encode_hex, parse_hex, retained, review_fact, review_graph, sha256};

const SHA256_OID: &str = "2.16.840.1.101.3.4.2.1";
const WITNESS_DOMAIN: &str = "urn:qualia:production:time-witness";
const WITNESS_PURPOSE: &str = "offline-imprint-v1";

impl AssetHost<'_> {
    pub(crate) fn attest_vdf_roughtime_and_pq_threshold(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        match text(required(args, "mode")?)? {
            "check_rfc3161_sha256_imprint" => self.check_timestamp_imprint(args),
            "prepare_mldsa65_time_witness" => self.time_witness(args, false),
            "verify_pinned_mldsa65_time_witness" => self.time_witness(args, true),
            _ => Err(
                "Select RFC 3161 imprint screening or an offline ML-DSA-65 time-witness mode"
                    .into(),
            ),
        }
    }

    fn check_timestamp_imprint(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "asset",
                "token_metadata",
                "request_nonce",
                "expected_policy_oid",
            ],
        )?;
        let handle = required(args, "token_metadata")?;
        let expected_nonce = exact_u64(required(args, "request_nonce")?)?;
        let expected_policy = text(required(args, "expected_policy_oid")?)?;
        if expected_nonce == 0 || !canonical_oid(expected_policy) {
            return Err(
                "Provide a nonzero original request nonce and a canonical timestamp policy OID"
                    .into(),
            );
        }
        let digest = self.timestamp_asset_digest(required(args, "asset")?, 8192)?;
        let metadata = retained(self, handle)?;
        if metadata.kind != "json" || !(1..=4096).contains(&metadata.bytes.len()) {
            return Err(
                "Provide a retained decoded timestamp metadata JSON record up to 4 KiB".into(),
            );
        }
        let token: TimestampToken = serde_json::from_slice(&metadata.bytes)
            .map_err(|_| "Unable to decode the native timestamp metadata JSON record")?;
        if token.hash_algorithm_oid != SHA256_OID
            || token.nonce != Some(expected_nonce)
            || token.policy_oid != expected_policy
            || !(1..=32).contains(&token.serial_number.len())
            || !utc_generalized_time(&token.gen_time_utc)
        {
            return Err("Timestamp metadata disagrees with the SHA-256 request profile".into());
        }
        let check = verify_rfc3161_timestamp(&token, &digest);
        if !check.imprint_matches {
            return Err("Timestamp message imprint differs from the retained asset SHA-256".into());
        }
        let mut receipt = review_graph("DecodedTimestampImprintMatch")?;
        review_fact(&mut receipt, "retainedAssetDigest", &digest_iri(&digest))?;
        review_fact(
            &mut receipt,
            "sourceCriteria",
            "https://www.rfc-editor.org/rfc/rfc3161.html",
        )?;
        review_fact(
            &mut receipt,
            "claimedGenerationTime",
            &format!("urn:qualia:generalized-time:{}", check.gen_time),
        )?;
        Ok(Value::Record(Args::from([
            ("imprint_matches".into(), Value::Bool(true)),
            ("request_nonce_matches".into(), Value::Bool(true)),
            ("tsa_signature_verified".into(), Value::Bool(false)),
            (
                "claimed_generation_time".into(),
                Value::String(check.gen_time),
            ),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }

    fn time_witness(&mut self, args: &Args, verify: bool) -> Result<Value, String> {
        let common = [
            "mode",
            "asset",
            "challenge_hex",
            "witnessed_epoch_ms",
            "uncertainty_ms",
        ];
        if verify {
            let mut keys = common.to_vec();
            keys.extend([
                "public_key_hex",
                "trusted_key_sha256",
                "signature_hex",
                "earliest_epoch_ms",
                "latest_epoch_ms",
            ]);
            fields(args, &keys)?;
        } else {
            fields(args, &common)?;
        }
        let observed = exact_u64(required(args, "witnessed_epoch_ms")?)?;
        let uncertainty = exact_u64(required(args, "uncertainty_ms")?)?;
        if !(1..=3_600_000).contains(&uncertainty) {
            return Err("Witness uncertainty must be in [1, 3,600,000] ms".into());
        }
        let lower = observed
            .checked_sub(uncertainty)
            .ok_or("Invalid lower time bound")?;
        let upper = observed
            .checked_add(uncertainty)
            .ok_or("Invalid upper time bound")?;
        let challenge = parse_hex::<32>(text(required(args, "challenge_hex")?)?, "challenge_hex")?;
        if challenge == [0; 32] {
            return Err("Time witness requires a nonzero caller challenge".into());
        }
        let digest = self.timestamp_asset_digest(required(args, "asset")?, 200_000)?;
        let context = CryptoContext {
            domain: WITNESS_DOMAIN.into(),
            purpose: WITNESS_PURPOSE.into(),
            timestamp: observed / 1000,
            nonce: challenge,
        };
        let mut statement = b"QUALIA-NATIVE-TIME-WITNESS-V1\0".to_vec();
        statement.extend(digest);
        statement.extend(challenge);
        statement.extend(observed.to_le_bytes());
        statement.extend(uncertainty.to_le_bytes());
        if !verify {
            return Ok(Value::Record(Args::from([
                (
                    "statement_hex".into(),
                    Value::String(encode_hex(&statement)),
                ),
                ("signature_domain".into(), Value::String(context.domain)),
                ("signature_purpose".into(), Value::String(context.purpose)),
                ("signature_timestamp".into(), Value::U64(context.timestamp)),
                (
                    "signature_nonce".into(),
                    Value::String(encode_hex(&context.nonce)),
                ),
            ])));
        }
        let earliest = exact_u64(required(args, "earliest_epoch_ms")?)?;
        let latest = exact_u64(required(args, "latest_epoch_ms")?)?;
        if earliest >= latest || lower < earliest || upper > latest {
            return Err(
                "Witness uncertainty interval exceeds the caller's allowed clock window".into(),
            );
        }
        let pk = parse_hex::<ML_DSA_PUBLIC_KEY_SIZE>(
            text(required(args, "public_key_hex")?)?,
            "public_key_hex",
        )?;
        let pin = parse_hex::<32>(
            text(required(args, "trusted_key_sha256")?)?,
            "trusted_key_sha256",
        )?;
        if sha256(&pk) != pin {
            return Err("Time witness public key differs from the trusted key pin".into());
        }
        let signature = MlDsaSignature {
            sig_bytes: parse_hex::<ML_DSA_SIGNATURE_SIZE>(
                text(required(args, "signature_hex")?)?,
                "signature_hex",
            )?
            .to_vec(),
        };
        if !MlDsaSigner::verify_with_public(&pk, &statement, &signature, &context)
            .map_err(|_| "Invalid native ML-DSA-65 time witness signature")?
        {
            return Err("The native ML-DSA-65 time witness signature failed verification".into());
        }
        let mut receipt = review_graph("PinnedMlDsa65TimeWitness")?;
        review_fact(&mut receipt, "retainedAssetDigest", &digest_iri(&digest))?;
        review_fact(&mut receipt, "witnessKeyDigest", &digest_iri(&pin))?;
        review_fact(
            &mut receipt,
            "witnessStatementDigest",
            &digest_iri(&sha256(&statement)),
        )?;
        review_fact(
            &mut receipt,
            "witnessLowerBound",
            &format!("urn:qualia:epoch-milliseconds:{lower}"),
        )?;
        review_fact(
            &mut receipt,
            "witnessUpperBound",
            &format!("urn:qualia:epoch-milliseconds:{upper}"),
        )?;
        Ok(Value::Record(Args::from([
            ("witness_signature_verified".into(), Value::Bool(true)),
            ("witness_lower_epoch_ms".into(), Value::U64(lower)),
            ("witness_upper_epoch_ms".into(), Value::U64(upper)),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }

    fn timestamp_asset_digest(
        &mut self,
        asset: &Value,
        overhead: usize,
    ) -> Result<[u8; 32], String> {
        let size = retained(self, asset)?.bytes.len();
        if size == 0 {
            return Err("Time witness needs a nonempty retained resource".into());
        }
        self.charge(size.saturating_mul(4).saturating_add(overhead), 32 * 1024)?;
        Ok(sha256(&retained(self, asset)?.bytes))
    }
}

fn canonical_oid(value: &str) -> bool {
    if !(3..=128).contains(&value.len()) {
        return false;
    }
    let mut arcs = value.split('.');
    let Some(first) = arcs.next().and_then(oid_arc) else {
        return false;
    };
    let Some(second) = arcs.next().and_then(oid_arc) else {
        return false;
    };
    first <= 2 && (first == 2 || second < 40) && arcs.all(|arc| oid_arc(arc).is_some())
}

fn oid_arc(arc: &str) -> Option<u64> {
    if arc.is_empty()
        || (arc.len() > 1 && arc.starts_with('0'))
        || !arc.bytes().all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    arc.parse().ok()
}

fn utc_generalized_time(value: &str) -> bool {
    let Some(body) = value.strip_suffix('Z') else {
        return false;
    };
    let (base, fraction) = body
        .split_once('.')
        .map_or((body, None), |(base, frac)| (base, Some(frac)));
    if base.len() != 14
        || !base.bytes().all(|byte| byte.is_ascii_digit())
        || fraction.is_some_and(|fraction| {
            !(1..=9).contains(&fraction.len())
                || fraction.ends_with('0')
                || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        })
    {
        return false;
    }
    let digits = |start: usize, end: usize| -> u16 {
        base.as_bytes()[start..end]
            .iter()
            .fold(0, |value, byte| value * 10 + u16::from(byte - b'0'))
    };
    let year = digits(0, 4);
    let month = digits(4, 6);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => return false,
    };
    year != 0
        && (1..=max_day).contains(&digits(6, 8))
        && digits(8, 10) < 24
        && digits(10, 12) < 60
        && digits(12, 14) < 60
}

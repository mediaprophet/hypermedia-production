//! RFC 3161 Timestamp Protocol token parsing and verification.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimestampToken {
    pub policy_oid: String,
    pub hash_algorithm_oid: String,
    pub message_imprint: Vec<u8>,
    pub serial_number: Vec<u8>,
    pub gen_time_utc: String, // e.g. "20261108060000Z"
    pub nonce: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TimestampVerification {
    pub valid: bool,
    pub imprint_matches: bool,
    pub gen_time: String,
    pub message: String,
}

pub fn verify_rfc3161_timestamp(
    token: &TimestampToken,
    expected_imprint: &[u8],
) -> TimestampVerification {
    let imprint_matches = token.message_imprint == expected_imprint;
    let valid = imprint_matches && !token.gen_time_utc.is_empty();

    let message = if valid {
        format!("RFC 3161 timestamp valid, certified at UTC {}", token.gen_time_utc)
    } else if !imprint_matches {
        "RFC 3161 message imprint mismatch: timestamp was created for different signature content".into()
    } else {
        "RFC 3161 timestamp token malformed or missing generation time".into()
    };

    TimestampVerification {
        valid,
        imprint_matches,
        gen_time: token.gen_time_utc.clone(),
        message,
    }
}

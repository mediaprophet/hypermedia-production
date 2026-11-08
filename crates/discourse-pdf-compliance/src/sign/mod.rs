//! Digital signatures: ETSI PAdES B-B envelope, byte-range verifier, and RFC 3161 timestamps.

#![forbid(unsafe_code)]

pub mod digest;
pub mod pades;
pub mod timestamp;

pub use digest::{hex_encode, sha256_slices};
pub use pades::{
    ByteRange, SignatureEnvelope, SignatureVerificationResult, verify_pades_byte_range,
};
pub use timestamp::{TimestampToken, TimestampVerification, verify_rfc3161_timestamp};

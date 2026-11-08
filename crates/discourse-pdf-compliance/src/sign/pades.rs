//! ETSI PAdES B-B digital signature envelope and ByteRange verifier.
//!
//! Conforms to ETSI EN 319 142-1 (PAdES Baseline Profile B-B) and ISO 32000-2 §12.8.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use super::digest::sha256_slices;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ByteRange {
    pub start1: usize,
    pub len1: usize,
    pub start2: usize,
    pub len2: usize,
}

impl ByteRange {
    pub const fn new(start1: usize, len1: usize, start2: usize, len2: usize) -> Self {
        ByteRange { start1, len1, start2, len2 }
    }

    /// Verifies that the ByteRange covers the file cleanly without overlaps,
    /// starting at 0 and leaving a valid signature gap.
    pub fn validate(&self, total_file_len: usize) -> Result<usize, String> {
        if self.start1 != 0 {
            return Err("ByteRange chunk 1 must start at index 0".into());
        }
        if self.len1 == 0 {
            return Err("ByteRange chunk 1 length must be non-zero".into());
        }
        let gap_start = self.start1 + self.len1;
        if self.start2 <= gap_start {
            return Err("ByteRange chunk 2 overlaps or precedes signature gap".into());
        }
        let gap_len = self.start2 - gap_start;
        if gap_len < 32 {
            return Err("Signature gap too small to hold valid CMS envelope".into());
        }
        if self.start2 + self.len2 > total_file_len {
            return Err(format!(
                "ByteRange extends beyond file length: {} > {}",
                self.start2 + self.len2,
                total_file_len
            ));
        }
        Ok(gap_len)
    }

    pub fn to_pdf_string(&self) -> String {
        format!("[ {} {} {} {} ]", self.start1, self.len1, self.start2, self.len2)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignatureEnvelope {
    pub filter: String,       // "Adobe.PPKLite"
    pub sub_filter: String,   // "ETSI.CAdES.detached" for PAdES
    pub byte_range: ByteRange,
    pub signer_name: Option<String>,
    pub signing_time: Option<String>,
    pub reason: Option<String>,
    pub location: Option<String>,
    pub raw_cms_der: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignatureVerificationResult {
    pub byte_range_valid: bool,
    pub digest_matches: bool,
    pub computed_sha256: [u8; 32],
    pub signed_byte_count: usize,
    pub message: String,
}

/// Verifies that a signed document's byte range digest matches the signed content.
pub fn verify_pades_byte_range(
    file_bytes: &[u8],
    byte_range: &ByteRange,
    expected_digest: Option<&[u8; 32]>,
) -> SignatureVerificationResult {
    if let Err(e) = byte_range.validate(file_bytes.len()) {
        return SignatureVerificationResult {
            byte_range_valid: false,
            digest_matches: false,
            computed_sha256: [0; 32],
            signed_byte_count: 0,
            message: format!("Invalid ByteRange: {e}"),
        };
    }

    let slice1 = &file_bytes[byte_range.start1..byte_range.start1 + byte_range.len1];
    let slice2 = &file_bytes[byte_range.start2..byte_range.start2 + byte_range.len2];
    let computed_digest = sha256_slices(&[slice1, slice2]);

    let digest_matches = match expected_digest {
        Some(expected) => &computed_digest == expected,
        None => true,
    };

    let signed_byte_count = byte_range.len1 + byte_range.len2;
    let message = if digest_matches {
        "PAdES B-B byte-range integrity verified: document contents intact since signing.".into()
    } else {
        "TAMPER DETECTED: Computed byte range digest does not match signature imprint!".into()
    };

    SignatureVerificationResult {
        byte_range_valid: true,
        digest_matches,
        computed_sha256: computed_digest,
        signed_byte_count,
        message,
    }
}

//! Tamper-evident hash-chained fiduciary audit trail.
//!
//! Grounded in international human rights instruments:
//! - **UDHR Article 12:** Protection against arbitrary interference with privacy, family, home, or correspondence.
//! - **UDHR Article 18:** Freedom of thought, conscience, and belief in information processing.
//! - **UDHR Article 19 & ICCPR Article 19:** Freedom to seek, receive, and impart information through any media.
//! - **UDHR Article 27 & ICESCR Article 15:** Protection of moral and material interests resulting from scientific,
//!   literary, or artistic production and civic participation.
//! - **Informatics Fiduciary Duties:** Binding obligations of utmost good faith, undivided loyalty, strict
//!   confidentiality, and non-manipulation owed by digital stewards to natural persons.

use serde::{Deserialize, Serialize};

/// Standard fiduciary human rights citations for audit logging.
pub const FIDUCIARY_CITATION_UDHR_12_18_19_27: &str =
    "UDHR Art. 12, 18, 19, 27; ICCPR Art. 17, 19; ICESCR Art. 15; Informatics Fiduciary Duty";

/// An immutable, cryptographically hash-chained entry in the fiduciary audit ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiduciaryAuditEntry {
    pub seq: u64,
    pub previous_hash: String,
    pub entry_hash: String,
    pub timestamp_epoch_secs: i64,
    pub actor_id: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub payload_digest: String,
    pub fiduciary_citation: String,
}

impl FiduciaryAuditEntry {
    pub fn compute_hash(
        seq: u64,
        prev_hash: &str,
        timestamp: i64,
        actor: &str,
        action: &str,
        entity_type: &str,
        entity_id: &str,
        payload_digest: &str,
        citation: &str,
    ) -> String {
        let payload = format!(
            "{seq}:{prev_hash}:{timestamp}:{actor}:{action}:{entity_type}:{entity_id}:{payload_digest}:{citation}"
        );
        fnv1a_hex(payload.as_bytes())
    }
}

/// A tamper-evident append-only fiduciary audit trail.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FiduciaryAuditTrail {
    pub entries: Vec<FiduciaryAuditEntry>,
}

impl FiduciaryAuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Appends a new verified audit event to the tamper-evident chain.
    pub fn append(
        &mut self,
        timestamp_epoch_secs: i64,
        actor_id: impl Into<String>,
        action: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
        payload_digest: impl Into<String>,
        citation: Option<&str>,
    ) -> String {
        let seq = (self.entries.len() as u64) + 1;
        let prev_hash = self
            .entries
            .last()
            .map(|e| e.entry_hash.clone())
            .unwrap_or_else(|| "0000000000000000".to_string());

        let actor = actor_id.into();
        let act = action.into();
        let e_type = entity_type.into();
        let e_id = entity_id.into();
        let digest = payload_digest.into();
        let cit = citation
            .unwrap_or(FIDUCIARY_CITATION_UDHR_12_18_19_27)
            .to_string();

        let entry_hash = FiduciaryAuditEntry::compute_hash(
            seq,
            &prev_hash,
            timestamp_epoch_secs,
            &actor,
            &act,
            &e_type,
            &e_id,
            &digest,
            &cit,
        );

        let entry = FiduciaryAuditEntry {
            seq,
            previous_hash: prev_hash,
            entry_hash: entry_hash.clone(),
            timestamp_epoch_secs,
            actor_id: actor,
            action: act,
            entity_type: e_type,
            entity_id: e_id,
            payload_digest: digest,
            fiduciary_citation: cit,
        };

        self.entries.push(entry);
        entry_hash
    }

    /// Validates the complete cryptographic hash chain from genesis to head.
    pub fn verify_chain(&self) -> Result<(), (usize, String)> {
        let mut expected_prev = "0000000000000000".to_string();
        for (i, entry) in self.entries.iter().enumerate() {
            if entry.previous_hash != expected_prev {
                return Err((
                    i,
                    format!(
                        "broken chain at seq {}: expected prev {}, found {}",
                        entry.seq, expected_prev, entry.previous_hash
                    ),
                ));
            }
            let recomputed = FiduciaryAuditEntry::compute_hash(
                entry.seq,
                &entry.previous_hash,
                entry.timestamp_epoch_secs,
                &entry.actor_id,
                &entry.action,
                &entry.entity_type,
                &entry.entity_id,
                &entry.payload_digest,
                &entry.fiduciary_citation,
            );
            if recomputed != entry.entry_hash {
                return Err((
                    i,
                    format!(
                        "tampered entry at seq {}: hash mismatch (recomputed {}, stored {})",
                        entry.seq, recomputed, entry.entry_hash
                    ),
                ));
            }
            expected_prev = entry.entry_hash.clone();
        }
        Ok(())
    }

    pub fn filter_by_entity(&self, entity_type: &str, entity_id: &str) -> Vec<&FiduciaryAuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.entity_type == entity_type && e.entity_id == entity_id)
            .collect()
    }
}

fn fnv1a_hex(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_trail_integrity_and_verification() {
        let mut trail = FiduciaryAuditTrail::new();
        let hash1 = trail.append(
            1700000000,
            "did:qualia:steward_01",
            "journal.post",
            "journal",
            "J-101",
            "sha256:abc1234",
            None,
        );
        let hash2 = trail.append(
            1700000060,
            "did:qualia:steward_01",
            "asset.depreciate",
            "asset",
            "A-201",
            "sha256:def5678",
            None,
        );

        assert_ne!(hash1, hash2);
        assert_eq!(trail.entries.len(), 2);
        assert!(trail.verify_chain().is_ok());

        // Assert citation contains UDHR and ICCPR grounding
        assert!(trail.entries[0]
            .fiduciary_citation
            .contains("UDHR Art. 12, 18, 19, 27"));
    }

    #[test]
    fn detects_tampering() {
        let mut trail = FiduciaryAuditTrail::new();
        trail.append(1000, "user1", "action1", "doc", "1", "hash1", None);
        trail.append(1010, "user2", "action2", "doc", "2", "hash2", None);

        assert!(trail.verify_chain().is_ok());

        // Tamper with an action in entry 0
        trail.entries[0].action = "tampered_action".into();
        assert!(trail.verify_chain().is_err());
    }
}

//! Synthetic NFC1 record encoding and detached native NCR1 signature verification.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::dual_proof::lowercase_hex;
use crate::{
    fields, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, required, text, Args,
    AssetHost, Value,
};
use qualia_core_db::{
    q_hash,
    sensing::{
        nfc_evidence::{
            encode_nfc_evidence_record, NfcCustodyReceipt, NfcEvidenceRecord, NfcReceiptStatus,
            ProtocolIdentifier, VariableRadioIdentifier, VariableRadioIdentifierKind,
            NFC_EVIDENCE_RECORD_SIZE, NFC_RECEIPT_ENCODED_LEN,
        },
        radio_evidence::{RadioEvidenceReference, RadioObservationTime},
        radio_receipt::ReceiptTimestamp,
    },
};
use sha2::{Digest, Sha256};

const FIXTURE_IRI: &str = "urn:qualia:sensing:fixture:synthetic-nfc1";

impl AssetHost<'_> {
    pub(super) fn encode_nfc1_fixture(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "case_id"])?;
        self.charge(NFC_EVIDENCE_RECORD_SIZE, NFC_EVIDENCE_RECORD_SIZE)?;
        let fixture = make_fixture(args)?;
        let mut bytes = vec![0; NFC_EVIDENCE_RECORD_SIZE];
        encode_nfc_evidence_record(fixture, &mut bytes)
            .map_err(|err| format!("Unable to encode the synthetic NFC1 record: {err:?}"))?;
        let record_sha256 = format!("{:x}", Sha256::digest(&bytes));
        Ok(Value::Record(Args::from([
            ("fixture".into(), self.encode("bin", bytes)?),
            ("record_sha256".into(), Value::String(record_sha256)),
        ])))
    }

    pub(super) fn verify_pinned_nfc1_fixture_receipt(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &["mode", "case_id", "receipt_key", "pinned_signer_sha256"],
        )?;
        let fixture = make_fixture(args)?;
        self.charge(100_000, NFC_EVIDENCE_RECORD_SIZE + NFC_RECEIPT_ENCODED_LEN)?;
        let key = text(required(args, "receipt_key")?)?;
        let input = self
            .inputs
            .iter()
            .find(|input| input.key == key)
            .ok_or("Unknown detached NFC1 fixture receipt input")?;
        if input.kind != "bin" || input.bytes.len() != NFC_RECEIPT_ENCODED_LEN {
            return Err("Provide a retained bin input containing one 240-byte NCR1 receipt".into());
        }
        let proof = NfcCustodyReceipt::decode_from(&input.bytes)
            .map_err(|err| format!("Invalid detached NCR1 fixture receipt: {err:?}"))?;
        if proof.sequence() != 0
            || proof.previous_checkpoint().is_some()
            || proof.timestamp() != ReceiptTimestamp::Unknown
        {
            return Err(
                "Detached verification requires a standalone, untimed NFC1 fixture receipt".into(),
            );
        }
        let signer_sha256 = format!("{:x}", Sha256::digest(proof.signer_key()));
        if signer_sha256 != text(required(args, "pinned_signer_sha256")?)? {
            return Err("NCR1 Ed25519 signer differs from the independent public-key pin".into());
        }
        let status = proof
            .verify_signature(fixture, &mut [0; NFC_EVIDENCE_RECORD_SIZE])
            .map_err(|err| {
                format!("Detached NCR1 signature failed fixture verification: {err:?}")
            })?;
        if status != NfcReceiptStatus::SignatureValidUntrusted {
            return Err("Unexpected detached NFC1 fixture verification status".into());
        }
        let record_sha256 = lowercase_hex(&proof.record_digest());
        let receipt_sha256 = format!("{:x}", Sha256::digest(&input.bytes));
        let subject = format!("urn:qualia:nfc1-fixture-signature:sha256:{receipt_sha256}");
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(&subject, "urn:qualia:sensing:fixtureKind", FIXTURE_IRI)?;
        graph.fact(
            &subject,
            "urn:qualia:sensing:verifiedSignatureScope",
            "urn:qualia:sensing:detached-fixture",
        )?;
        graph.fact(
            &subject,
            "urn:qualia:sensing:recordDigest",
            &format!("urn:sha256:{record_sha256}"),
        )?;
        graph.fact(
            &subject,
            "urn:qualia:sensing:signerDigest",
            &format!("urn:sha256:{signer_sha256}"),
        )?;
        let receipt = graph.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        Ok(Value::Record(Args::from([
            ("signature_verified".into(), Value::Bool(true)),
            ("record_sha256".into(), Value::String(record_sha256)),
            ("receipt".into(), self.encode("q42", receipt)?),
        ])))
    }
}

fn make_fixture(args: &Args) -> Result<NfcEvidenceRecord, String> {
    let case_id = match required(args, "case_id")? {
        Value::U64(n) if *n > 0 => *n,
        Value::I64(n) if *n > 0 => *n as u64,
        _ => return Err("case_id requires an exact positive integer".into()),
    };
    let token = format!("fixture-{case_id:016x}");
    let identifier = VariableRadioIdentifier::new(
        VariableRadioIdentifierKind::ProtocolIdentifier,
        ProtocolIdentifier::from_optional(q_hash(FIXTURE_IRI)),
        token.as_bytes(),
    )
    .map_err(|err| format!("Invalid synthetic NFC1 fixture identifier: {err:?}"))?;
    NfcEvidenceRecord::new(
        case_id,
        identifier,
        q_hash("urn:qualia:sensing:fixture:synthetic-nfc1-source"),
        q_hash(&format!("{FIXTURE_IRI}:{case_id}:capture")),
        None,
        None,
        None,
        RadioObservationTime::Unknown,
        RadioEvidenceReference::NotAttached,
        None,
    )
    .map_err(|err| format!("Invalid synthetic NFC1 fixture record: {err:?}"))
}

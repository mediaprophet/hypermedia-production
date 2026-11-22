//! Bounded Q42 journal authoring and recovery.

use super::{exact_u64, external_resource, fields, required, text, Args, AssetHost, ReceiptGraph,
    Value, SYMBOLIC_LIMIT};
use crate::{list, record};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    q42::journal::{crc32c, FrameHeader, JournalApplicationProfile, JournalFrameType, JournalHeader,
        JournalPrivacyClass, JournalTransaction, MutableQ42Session, Q42Journal, FRAME_HEADER_SIZE,
        Q42J_HEADER_SIZE},
    q_hash, NQuin,
};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, HashSet}, io::Cursor, sync::atomic::{AtomicBool, Ordering}};

const GRAPH: &str = "https://qualiadb.org/ontology/q42#JournalReceipt";
const MAX_DELTA_ROWS: usize = 16_384;
const MAX_TOKEN_BYTES: usize = 4096;

#[path = "conditioning.rs"]
mod conditioning;

impl AssetHost<'_> {
    pub(crate) fn compile_conditioning_and_q42_journal(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let parameters = record(required(args, "parameters")?)?;
        if matches!(op, "conditioning_validate" | "conditioning_compile") {
            return conditioning::profile(self, op, parameters);
        }
        if matches!(op, "hybrid_graph_logit_bias" | "kv_dictionary_roundtrip") {
            return super::graph_decode::evaluate(self, op, parameters);
        }
        if !matches!(op, "journal_commit" | "journal_rollback" | "journal_recover") {
            return Err(format!("Unsupported conditioning/journal operation: {op}"));
        }
        if op == "journal_recover" {
            fields(parameters, &["base", "journal"])?;
        } else {
            fields(parameters, &[
                "base", "world_did", "actor_did", "command", "tick", "adds", "removes", "bindings",
            ])?;
        }
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        let base = required(parameters, "base")?;
        let journal = if op == "journal_recover" { Some(required(parameters, "journal")?) } else { None };
        let base_len = resource(self, base, "q42", budget)?.len();
        let journal_len = journal.map(|value| resource(self, value, "q42j", budget))
            .transpose()?.map_or(0, <[u8]>::len);
        let work = base_len.checked_add(journal_len).ok_or("Q42 journal input size overflows")?;
        let scratch = budget.min(self.limits.total_resource_bytes);
        self.charge(work, scratch)?;

        let output = {
            let base = resource(self, base, "q42", budget)?;
            let digest: [u8; 32] = Sha256::digest(base).into();
            let model = SemanticModelBuilder::new(scratch)
                .merge_q42(base)
                .map_err(|error| format!("Q42 journal baseline admission failed: {error}"))?;
            // The native mutable session owns a second copy of the row keys.
            if model.snapshot_quins().len() > scratch / (4 * std::mem::size_of::<NQuin>()) {
                return Err("Q42 journal baseline exceeds mutation scratch budget".into());
            }
            if let Some(journal) = journal {
                let bytes = resource(self, journal, "q42j", budget)?;
                recover(base, digest, bytes, model, budget, self.cancel)?
            } else {
                stage_and_apply(base, digest, model, parameters, op == "journal_commit", budget, self.cancel)?
            }
        };
        let sum = output.q42.len().checked_add(output.journal.len())
            .and_then(|bytes| bytes.checked_add(output.receipt.len()))
            .ok_or("Q42 journal output size overflows")?;
        if sum > self.limits.total_resource_bytes.saturating_sub(self.workspace.bytes)
            || self.limits.resources.saturating_sub(self.workspace.entries.len()) < 3
        {
            return Err("Q42 journal outputs exceed production workspace budget".into());
        }
        self.charge(output.operations, sum)?;
        let mut result = BTreeMap::from([
            ("q42".into(), self.encode("q42", output.q42)?),
            ("journal".into(), self.encode("q42j", output.journal)?),
            ("receipt".into(), self.encode("q42", output.receipt)?),
            ("committed".into(), Value::Bool(output.transaction.is_some())),
            ("quin_count".into(), Value::U64(output.rows as u64)),
        ]);
        if let Some(id) = output.transaction {
            result.insert("transaction".into(), Value::U64(u64::from(id)));
        }
        Ok(Value::Record(result))
    }
}

fn resource<'a>(
    host: &'a AssetHost<'_>, value: &Value, kind: &str, budget: usize,
) -> Result<&'a [u8], String> {
    let resource = external_resource(host, value)?;
    if resource.kind != kind || resource.bytes.len() > budget {
        return Err(format!("Expected a {kind} resource within journal byte budget"));
    }
    Ok(&resource.bytes)
}

fn check_cancelled(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Relaxed) {
        return Err("Production cancelled".into());
    }
    Ok(())
}

fn parse_rows(rows: &[Value], cancel: &AtomicBool) -> Result<Vec<NQuin>, String> {
    if rows.len() > MAX_DELTA_ROWS {
        return Err("Q42 journal row count exceeds transaction budget".into());
    }
    let mut parsed = Vec::with_capacity(rows.len());
    let mut seen = HashSet::with_capacity(rows.len());
    for value in rows {
        check_cancelled(cancel)?;
        let fields: &[Value; 6] = list(value)?.try_into()
            .map_err(|_| "A journal NQuin requires six integer columns")?;
        let [s, p, o, c, m, parity] = fields;
        let row = NQuin {
            subject: exact_u64(s)?, predicate: exact_u64(p)?, object: exact_u64(o)?,
            context: exact_u64(c)?, metadata: exact_u64(m)?, parity: exact_u64(parity)?,
        };
        if !row.verify_ecc_parity() || !seen.insert(row) {
            return Err("Q42 journal contains an invalid or duplicate NQuin".into());
        }
        parsed.push(row);
    }
    Ok(parsed)
}

fn items<'a>(parameters: &'a Args, name: &str) -> Result<&'a [Value], String> {
    parameters.get(name).map(list).unwrap_or(Ok(&[]))
}

struct JournalOutput {
    q42: Vec<u8>,
    journal: Vec<u8>,
    receipt: Vec<u8>,
    transaction: Option<u32>,
    rows: usize,
    operations: usize,
}

#[allow(clippy::too_many_arguments)]
fn stage_and_apply(
    base: &[u8],
    digest: [u8; 32],
    mut model: SemanticModelBuilder,
    params: &Args,
    commit: bool,
    budget: usize,
    cancel: &AtomicBool,
) -> Result<JournalOutput, String> {
    let world_did = text(required(params, "world_did")?)?;
    let actor_did = text(required(params, "actor_did")?)?;
    let command = text(required(params, "command")?)?;
    if !world_did.starts_with("did:") || !actor_did.starts_with("did:") {
        return Err("Q42 journal world and actor identities require DIDs".into());
    }
    let identity = |model: &mut SemanticModelBuilder, iri: &str| {
        if iri.len() > MAX_TOKEN_BYTES {
            return Err("Q42 journal identity exceeds token budget".to_owned());
        }
        model.bind_iri(iri).map_err(|error| error.to_string())
    };
    let world_hash = identity(&mut model, world_did)?;
    let actor_hash = identity(&mut model, actor_did)?;
    let command_hash = identity(&mut model, command)?;
    let tick = exact_u64(required(params, "tick")?)?;
    if tick == 0 {
        return Err("Q42 journal tick must be positive".into());
    }
    let mut transaction = JournalTransaction::new(1, tick, command_hash, actor_hash);
    transaction.adds = parse_rows(items(params, "adds")?, cancel)?;
    transaction.removes = parse_rows(items(params, "removes")?, cancel)?;
    let removals: HashSet<_> = transaction.removes.iter().copied().collect();
    if transaction.adds.iter().any(|row| removals.contains(row)) {
        return Err("A Q42 journal transaction cannot both add and remove the same row".into());
    }
    let bindings = items(params, "bindings")?;
    let operations = transaction.adds.len() + transaction.removes.len() + bindings.len();
    if operations > MAX_DELTA_ROWS {
        return Err("Q42 journal transaction exceeds operation limit".into());
    }
    let mut seen = HashSet::new();
    for iri in [world_did, actor_did, command].into_iter()
        .chain(bindings.iter().map(text).collect::<Result<Vec<_>, _>>()?)
    {
        check_cancelled(cancel)?;
        let hash = identity(&mut model, iri)?;
        if seen.insert(hash) {
            transaction.bind_lexicon(hash, iri.to_owned());
        }
    }
    for row in transaction.adds.iter().chain(&transaction.removes) {
        model.check_quin_tokens(row).map_err(|error| error.to_string())?;
    }
    let frame_bytes = transaction.lexicon_bindings.iter().try_fold(0usize, |size, (_, iri)| {
        size.checked_add(iri.len() + FRAME_HEADER_SIZE + 8)
    }).ok_or("Q42 lexical frames exceed address space")?;
    let journal_len = Q42J_HEADER_SIZE + 2 * FRAME_HEADER_SIZE + 20 + frame_bytes
        + (transaction.adds.len() + transaction.removes.len()) * (FRAME_HEADER_SIZE + 48);
    if journal_len > budget {
        return Err("Q42 journal frames exceed the output byte budget".into());
    }
    let header = JournalHeader::new(JournalApplicationProfile::GenericSemantic, world_hash, digest, 1);
    let storage = Cursor::new(Vec::with_capacity(if commit { journal_len } else { Q42J_HEADER_SIZE }));
    let mut journal = Q42Journal::create(storage, header).map_err(|error| error.to_string())?;
    check_cancelled(cancel)?;
    let (q42, journal, rows, committed) = if commit {
        model.restrict_to_sanctuary();
        journal.append_transaction(&transaction).map_err(|error| error.to_string())?;
        let session = MutableQ42Session::open(journal, model.snapshot_quins())
            .map_err(|error| error.to_string())?;
        let rows = session.active_state.len();
        let output = model.seal_projection(session.active_state).map_err(|error| error.to_string())?;
        (output, session.journal.storage.into_inner(), rows, Some(transaction.tx_id))
    } else {
        let mut session = MutableQ42Session::open(journal, model.snapshot_quins())
            .map_err(|error| error.to_string())?;
        for row in transaction.adds {
            session.stage_add(row);
        }
        for row in transaction.removes {
            session.stage_remove(row);
        }
        session.rollback_staged();
        (base.to_vec(), session.journal.storage.into_inner(), session.active_state.len(), None)
    };
    make_output(if commit { "commit" } else { "rollback" }, q42, journal, rows, committed,
        operations, budget, cancel)
}

fn recover(
    base: &[u8],
    digest: [u8; 32],
    bytes: &[u8],
    mut model: SemanticModelBuilder,
    budget: usize,
    cancel: &AtomicBool,
) -> Result<JournalOutput, String> {
    let admitted = committed_prefix(bytes, cancel)?;
    let mut journal = Q42Journal::open(Cursor::new(admitted.to_vec()), &digest)
        .map_err(|error| error.to_string())?;
    if journal.header.privacy != JournalPrivacyClass::Commons as u8 {
        model.restrict_to_sanctuary();
    }
    let transactions = journal.recover_transactions().map_err(|error| error.to_string())?;
    let mut operations = 0usize;
    for transaction in &transactions {
        check_cancelled(cancel)?;
        for (hash, iri) in &transaction.lexicon_bindings {
            if model.bind_iri(iri).map_err(|error| error.to_string())? != *hash {
                return Err("Journal lexical binding hash differs from its canonical IRI".into());
            }
        }
        for row in transaction.adds.iter().chain(&transaction.removes) {
            model.check_quin_tokens(row).map_err(|error| error.to_string())?;
        }
        operations += transaction.adds.len() + transaction.removes.len()
            + transaction.lexicon_bindings.len();
    }
    let committed = transactions.last().map(|transaction| transaction.tx_id);
    drop(transactions);
    let session = MutableQ42Session::open(journal, model.snapshot_quins())
        .map_err(|error| error.to_string())?;
    let rows = session.active_state.len();
    let q42 = if committed.is_some() {
        model.seal_projection(session.active_state).map_err(|error| error.to_string())?
    } else {
        base.to_vec()
    };
    make_output("recover", q42, session.journal.storage.into_inner(), rows, committed,
        operations, budget, cancel)
}

/// Check all lengths and transaction boundaries before native recovery reads payloads.
/// Return only complete committed frames, excluding an interrupted write at the tail.
fn committed_prefix<'a>(bytes: &'a [u8], cancel: &AtomicBool) -> Result<&'a [u8], String> {
    let header = JournalHeader::from_bytes(bytes).map_err(|error| error.to_string())?;
    if header.profile != JournalApplicationProfile::GenericSemantic as u16
        || header.schema_rule_version != 1 || header.flags != 0
        || header.privacy > JournalPrivacyClass::Commons as u8
        || header.required_pack_digest != [0; 32]
    {
        return Err("Unsupported Q42 journal profile or required rule pack".into());
    }
    let mut position = Q42J_HEADER_SIZE;
    let mut committed = position;
    let mut pending = None;
    let mut previous_tx = 0u32;
    let mut previous_tick = 0u64;
    let mut previous_sequence = 0u64;
    let mut frames = 0usize;
    while let Some(frame) = bytes.get(position..position + FRAME_HEADER_SIZE) {
        check_cancelled(cancel)?;
        frames += 1;
        if frames > MAX_DELTA_ROWS * 2 {
            return Err("Q42 journal recovery exceeds frame budget".into());
        }
        let frame = FrameHeader::from_bytes(frame).map_err(|error| error.to_string())?;
        let kind = JournalFrameType::try_from(frame.frame_type).map_err(|error| error.to_string())?;
        let len = frame.payload_len as usize;
        let valid_len = match kind {
            JournalFrameType::TxBegin => len == 8 || len == 16,
            JournalFrameType::QuinAdd | JournalFrameType::QuinRemove => len == 48,
            JournalFrameType::LexiconBind => (9..=MAX_TOKEN_BYTES + 8).contains(&len),
            JournalFrameType::Receipt => len == 32,
            JournalFrameType::TxCommit => len == 4,
            _ => false,
        };
        if !valid_len || frame.reserved != [0; 3] || frame.sequence_num <= previous_sequence {
            return Err("Q42 journal frame profile or sequence mismatch".into());
        }
        let start = position + FRAME_HEADER_SIZE;
        let end = start.checked_add(len).ok_or("Q42 journal frame extent overflows")?;
        let Some(payload) = bytes.get(start..end) else { break };
        if frame.crc32c != crc32c(payload) {
            return Err("Q42 journal frame checksum mismatch".into());
        }
        previous_sequence = frame.sequence_num;
        if kind == JournalFrameType::TxBegin {
            if pending.is_some() || frame.tx_id <= previous_tx || frame.tx_id == u32::MAX
                || frame.tick < previous_tick
            {
                return Err("Q42 journal transaction order is invalid".into());
            }
            pending = Some((frame.tx_id, frame.tick));
        } else if pending != Some((frame.tx_id, frame.tick)) {
            return Err("Q42 journal frame is outside its transaction".into());
        } else if kind == JournalFrameType::LexiconBind {
            let hash = u64::from_le_bytes(payload[..8].try_into().map_err(|_| "Invalid lexical frame")?);
            let iri = std::str::from_utf8(&payload[8..]).map_err(|_| "Invalid journal lexical UTF-8")?;
            if hash == 0 || q_hash(iri) != hash {
                return Err("Journal lexical frame hash differs from its canonical IRI".into());
            }
        } else if kind == JournalFrameType::TxCommit {
            if payload != [0xAA; 4] {
                return Err("Q42 journal commit marker is invalid".into());
            }
            committed = end;
            pending = None;
            previous_tx = frame.tx_id;
            previous_tick = frame.tick;
        }
        position = end;
    }
    Ok(&bytes[..committed])
}

#[allow(clippy::too_many_arguments)]
fn make_output(
    action: &str,
    q42: Vec<u8>,
    journal: Vec<u8>,
    rows: usize,
    transaction: Option<u32>,
    operations: usize,
    budget: usize,
    cancel: &AtomicBool,
) -> Result<JournalOutput, String> {
    check_cancelled(cancel)?;
    let subject = format!("urn:sha256:{:x}", Sha256::digest(&journal));
    let mut receipt = ReceiptGraph::new(GRAPH);
    receipt.fact(&subject, "https://qualiadb.org/ontology/q42#journalAction",
        &format!("urn:qualia:q42:journal:{action}"))?;
    receipt.fact(&subject, "https://qualiadb.org/ontology/q42#resultSnapshot",
        &format!("urn:sha256:{:x}", Sha256::digest(&q42)))?;
    let receipt = receipt.seal(budget)?;
    Ok(JournalOutput { q42, journal, receipt, transaction, rows, operations })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_recovery_uses_complete_commits_and_rejects_bad_crc() {
        let cancel = AtomicBool::new(false);
        let digest = [7; 32];
        let header = JournalHeader::new(JournalApplicationProfile::GenericSemantic, q_hash("did:q42:test"), digest, 1);
        let mut journal = Q42Journal::create(Cursor::new(Vec::new()), header).unwrap();
        journal.append_transaction(&JournalTransaction::new(1, 10, 4, 8)).unwrap();
        let first = journal.storage.get_ref().len();
        journal.append_transaction(&JournalTransaction::new(2, 11, 4, 8)).unwrap();
        let mut bytes = journal.storage.into_inner();
        bytes.pop();
        assert_eq!(committed_prefix(&bytes, &cancel).unwrap().len(), first);
        bytes[Q42J_HEADER_SIZE + FRAME_HEADER_SIZE] ^= 1;
        assert!(committed_prefix(&bytes, &cancel).is_err());
    }

    #[test]
    fn staged_rollback_keeps_exact_base_and_commit_has_recoverable_projection() {
        let cancel = AtomicBool::new(false);
        let budget = 1024 * 1024;
        let mut model = SemanticModelBuilder::new(budget);
        let names = ["urn:test:subject", "urn:test:predicate", "urn:test:object"];
        let [s, p, o] = names.map(|name| model.bind_iri(name).unwrap());
        let row = NQuin { subject:s, predicate:p, object:o, context:0, metadata:0, parity:s ^ p ^ o };
        let base = model.seal_projection(std::iter::once(row)).unwrap();
        let digest = Sha256::digest(&base).into();
        let params = BTreeMap::from([
            ("world_did".into(), Value::String("did:q42:world".into())),
            ("actor_did".into(), Value::String("did:q42:actor".into())),
            ("command".into(), Value::String("urn:qualia:remove".into())),
            ("tick".into(), Value::U64(1)),
            ("removes".into(), Value::List(vec![Value::List(
                [s, p, o, 0, 0, row.parity].into_iter().map(Value::U64).collect(),
            )])),
        ]);
        let load = || SemanticModelBuilder::new(budget).merge_q42(&base).unwrap();
        let rollback = stage_and_apply(&base, digest, load(), &params, false, budget, &cancel).unwrap();
        assert_eq!(rollback.q42, base);
        assert_eq!(rollback.journal.len(), Q42J_HEADER_SIZE);
        assert!(rollback.transaction.is_none());
        let commit = stage_and_apply(&base, digest, load(), &params, true, budget, &cancel).unwrap();
        assert_eq!(commit.rows, 0);
        let replay = recover(&base, digest, &commit.journal, load(), budget, &cancel).unwrap();
        assert_eq!(replay.q42, commit.q42);
        assert_eq!(replay.transaction, commit.transaction);
    }
}

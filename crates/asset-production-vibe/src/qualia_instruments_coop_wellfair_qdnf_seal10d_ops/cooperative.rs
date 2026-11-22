//! Cooperative lifecycle, ledger merge, and supported-agency evaluation.

use super::{fields, from_json, json, quin_value, required, text, timestamp, unsigned, Args,
    AssetHost, Value, SYMBOLIC_LIMIT};
use crate::{list, num, vector};
use qualia_cooperative_core::{
    agency_delegation::{delegation_permits, AccessDecision, AccessRequest, AgencyDelegation, ControlStage},
    agency_domain::agency_domain_taxonomy,
    finance::{div_round_bankers, merge_journal_entries, JournalEntryRecord},
    record::NQuin as CooperativeQuin,
    trigger::{evaluate as evaluate_trigger, TriggerContext},
};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    specialized_libs::computational_economics::forensic_economics::{
        accumulate_harm_trace, AccumulatedHarm, HealthWelfareState, NquinVector, MAX_TRAJECTORY_STEPS,
    },
    NQuin,
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_RECORDS: usize = 512;

macro_rules! read {
    ($params:expr, $size:expr, $name:literal, $ty:ty) => {
        serde_json::from_value::<$ty>(json(required($params, $name)?, $size, 0)?)
            .map_err(|error| format!("Invalid {}: {error}", $name))?
    };
}

mod lifecycle;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "compile_stages" => lifecycle::compile_stages(host, params),
        "derive_obligation" => lifecycle::derive(host, params),
        "transition_readiness" => lifecycle::transition(host, params),
        "merge_ledger" => ledger(host, params),
        "agency_permits" => agency(host, params),
        "harm_trace" => harm(host, params),
        "bankers_round" => bankers_round(host, params),
        _ => Err(format!("Unsupported cooperative operation: {op}")),
    }
}

fn core_row(row: &CooperativeQuin, object_is_term: bool) -> NQuin {
    // Cooperative's native layout hashes identifiers with full-width FNV-1a. Q42
    // uses the matching lower 60 bits for resource tokens. Numeric words stay intact.
    const TOKEN_MASK: u64 = 0x0FFF_FFFF_FFFF_FFFF;
    let mut quin = NQuin {
        subject: row.subject & TOKEN_MASK, predicate: row.predicate & TOKEN_MASK,
        object: if object_is_term { row.object & TOKEN_MASK } else { row.object },
        context: row.context & TOKEN_MASK, metadata: row.metadata, parity: 0,
    };
    quin.recalculate_parity();
    quin
}

fn seal(host: &mut AssetHost<'_>, rows: &[NQuin]) -> Result<Value, String> {
    let budget = host.limits.resource_bytes.min(SYMBOLIC_LIMIT);
    let mut graph = SemanticModelBuilder::new(budget);
    graph.restrict_to_sanctuary();
    let bytes = graph.seal_projection(rows.iter().copied()).map_err(|error| error.to_string())?;
    host.encode("q42", bytes)
}

fn signed(value: &Value) -> Result<i64, String> {
    match value {
        Value::I64(value) => Ok(*value),
        Value::U64(value) => i64::try_from(*value).map_err(|_| "Signed integer exceeds i64".into()),
        _ => Err("Expected signed integer".into()),
    }
}

fn bankers_round(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["numerator", "denominator"])?;
    let numerator = signed(required(params, "numerator")?)?;
    let denominator = signed(required(params, "denominator")?)?;
    if denominator == 0 {
        return Err("Integer division requires a nonzero denominator".into());
    }
    host.charge(1, 0)?;
    Ok(Value::I64(div_round_bankers(i128::from(numerator), i128::from(denominator))))
}

fn validates_entry(entry: &JournalEntryRecord) -> Result<(), String> {
    if entry.id.is_empty() || entry.lines.len() > 128 || entry.currency.len() != 3
        || !entry.currency.bytes().all(|byte| byte.is_ascii_uppercase())
        || entry.lines.iter().any(|line| line.account_code.trim().is_empty())
    {
        return Err("Ledger entry identity, currency, or line count is invalid".into());
    }
    entry.validate_balance()?;
    let (debit, credit) = entry.lines.iter().fold((0i128, 0i128), |(d, c), line|
        (d + i128::from(line.debit_cents), c + i128::from(line.credit_cents)));
    if debit != credit || debit > i128::from(i64::MAX) {
        return Err("Ledger entry totals differ or exceed the native cents range".into());
    }
    Ok(())
}

fn ledger(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["existing", "incoming"])?;
    let mut size = 0;
    let existing = read!(params, &mut size, "existing", Vec<JournalEntryRecord>);
    let incoming = read!(params, &mut size, "incoming", Vec<JournalEntryRecord>);
    let count = existing.len() + incoming.len();
    if count > MAX_RECORDS {
        return Err("Ledger merge exceeds entry count budget".into());
    }
    host.charge(count.saturating_mul(count).saturating_add(size), size)?;
    let mut seen = BTreeMap::<&str, &JournalEntryRecord>::new();
    for entry in existing.iter().chain(&incoming) {
        validates_entry(entry)?;
        if seen.insert(&entry.id, entry).is_some_and(|prior| prior != entry) {
            return Err(format!("Conflicting immutable journal entry {}", entry.id));
        }
    }
    let merged = merge_journal_entries(&existing, &incoming);
    let mut rows = Vec::with_capacity(merged.len() * 4);
    for entry in &merged {
        let mut bytes = [CooperativeQuin::default(); 4];
        let count = entry.compile_to_quins(&mut bytes);
        rows.extend(bytes[..count].iter().enumerate().map(|(index, row)| core_row(row, index < 2)));
    }
    let receipt = seal(host, &rows)?;
    Ok(Value::Record(BTreeMap::from([
        ("entries".into(), from_json(serde_json::to_value(&merged).map_err(|error| error.to_string())?)),
        ("receipt".into(), receipt),
        ("entry_count".into(), Value::U64(merged.len() as u64)),
    ])))
}

fn stage_name(stage: ControlStage) -> &'static str {
    match stage {
        ControlStage::GuardianSole => "guardian_sole",
        ControlStage::CoSigned => "co_signed",
        ControlStage::PrincipalSole => "principal_sole",
    }
}

fn agency(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["delegation", "request", "context", "actor", "cosigners"])?;
    let mut size = 0;
    let delegation = read!(params, &mut size, "delegation", AgencyDelegation);
    let request = read!(params, &mut size, "request", AccessRequest);
    let context = read!(params, &mut size, "context", TriggerContext);
    let actor = text(required(params, "actor")?)?;
    let cosigners = match params.get("cosigners") {
        Some(value) => list(value)?,
        None => &[],
    };
    if delegation.transfer_schedule.len() > MAX_RECORDS || cosigners.len() > 64 {
        return Err("Delegation proof exceeds participant or transfer budget".into());
    }
    let cosigners = cosigners.iter().map(text).collect::<Result<BTreeSet<_>, _>>()?;
    let mut stages = BTreeMap::new();
    let mut current = ControlStage::GuardianSole;
    let mut pending = false;
    host.charge(size, size)?;
    for transfer in &delegation.transfer_schedule {
        let prior = stages.insert(&transfer.domain, transfer.to_stage);
        if prior.is_some_and(|prior| transfer.to_stage <= prior) {
            return Err("Delegation transfer schedule must advance monotonically by domain".into());
        }
        if transfer.domain == request.domain && !pending {
            if evaluate_trigger(&transfer.trigger, &context) {
                current = transfer.to_stage;
            } else {
                pending = true;
            }
        }
    }
    let decision = delegation_permits(&delegation, &agency_domain_taxonomy(), &request, &context);
    let reason = match decision {
        AccessDecision::Deny(reason) => Some(reason),
        AccessDecision::Permit if current == ControlStage::PrincipalSole
            && actor != delegation.principal_did.as_str() => Some("principal controls this domain".into()),
        AccessDecision::Permit if current != ControlStage::PrincipalSole
            && !delegation.agent_dids.iter().any(|did| did == actor) => Some("actor is outside the delegation".into()),
        AccessDecision::Permit if current == ControlStage::CoSigned
            && !cosigners.contains(delegation.principal_did.as_str()) => Some("the principal's co-signature is required".into()),
        AccessDecision::Permit => None,
    };
    let mut result = BTreeMap::from([
        ("permitted".into(), Value::Bool(reason.is_none())),
        ("control_stage".into(), Value::String(stage_name(current).into())),
    ]);
    if let Some(reason) = reason {
        result.insert("reason".into(), Value::String(reason));
    }
    Ok(Value::Record(result))
}

fn harm(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["initial_state", "initial_vector", "support", "shocks", "seed"])?;
    let initial_state = match text(required(params, "initial_state")?)? {
        "stable" => HealthWelfareState::Stable,
        "stressed" => HealthWelfareState::Stressed,
        "chronic_unresolved" => HealthWelfareState::ChronicUnresolved,
        "acute_crisis" => HealthWelfareState::AcuteCrisis,
        "irrecoverable_debilitation" => HealthWelfareState::IrrecoverableDebilitation,
        _ => return Err("Unknown initial experiential state".into()),
    };
    let initial = NquinVector::from_array(vector::<5>(required(params, "initial_vector")?)?);
    let support = list(required(params, "support")?)?;
    let shocks = list(required(params, "shocks")?)?;
    let count = support.len();
    if count == 0 || count != shocks.len() || count > MAX_TRAJECTORY_STEPS {
        return Err("Experiential trace requires equal finite-length support and shock arrays".into());
    }
    let support = support.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    if support.iter().any(|amount| !(0.0..=1.0).contains(amount)) {
        return Err("Experiential support levels must be in [0, 1]".into());
    }
    let shocks = shocks.iter().map(num).collect::<Result<Vec<_>, _>>()?;
    let seed = unsigned(required(params, "seed")?)?;
    host.charge(count, count * 144)?;
    let mut states = vec![initial_state; count];
    let mut vectors = vec![NquinVector::ZERO; count];
    let mut accumulated = vec![AccumulatedHarm {
        current_load: NquinVector::ZERO, chronic_steps: 0, acute_spikes: 0,
        entered_absorbing: false, recovery_baseline_shift: 0.,
    }; count];
    let (written, absorbing) = accumulate_harm_trace(initial_state, &initial, &support,
        &shocks, seed, &mut states, &mut vectors, &mut accumulated)
        .map_err(|error| format!("Invalid experiential trace: {error:?}"))?;
    let values = |array: [f64; 5]| Value::List(array.into_iter().map(Value::F64).collect());
    let trace = (0..written).map(|index| {
        let harm = accumulated[index];
        Value::Record(BTreeMap::from([
            ("state".into(), Value::U64(states[index] as u64)),
            ("vector".into(), values(vectors[index].to_array())),
            ("chronic_steps".into(), Value::U64(u64::from(harm.chronic_steps))),
            ("acute_spikes".into(), Value::U64(u64::from(harm.acute_spikes))),
            ("baseline_shift".into(), Value::F64(harm.recovery_baseline_shift)),
        ]))
    }).collect();
    Ok(Value::Record(BTreeMap::from([
        ("trace".into(), Value::List(trace)),
        ("absorbing".into(), Value::Bool(absorbing)),
    ])))
}

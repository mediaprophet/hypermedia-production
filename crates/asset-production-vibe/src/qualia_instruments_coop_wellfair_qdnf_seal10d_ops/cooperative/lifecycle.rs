//! Bounded lifecycle compilation and linked obligation derivation.

use super::{fields, from_json, json, quin_value, required, text, timestamp, Args, AssetHost,
    CooperativeQuin, NQuin, Value, MAX_RECORDS, core_row, seal};
use qualia_cooperative_core::contribution_lifecycle::{
    apply_settlements_and_recognition, derive_obligation_from_lifecycle, evaluate_transition_readiness,
    AcceptanceDecision, ContributionLifecycleStage, LifecycleObligation, RequestAndAgreement,
    SettlementAllocation, SubmissionOrClaim, TransitionDecision, Valuation,
};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn compile_stages(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["stages"])?;
    let mut size = 0;
    let stages = read!(params, &mut size, "stages", Vec<ContributionLifecycleStage>);
    if stages.is_empty() || stages.len() > MAX_RECORDS {
        return Err("Lifecycle stage array is empty or exceeds its production count limit".into());
    }
    host.charge(size, size)?;
    let mut rows: Vec<NQuin> = Vec::with_capacity(stages.len() * 2);
    for stage in &stages {
        let mut output = [CooperativeQuin::default(); 2];
        let count = stage.compile_to_quins(&mut output);
        let object_is_term = matches!(stage, ContributionLifecycleStage::OutputOrService(_)
            | ContributionLifecycleStage::TransitionDecision(_));
        rows.extend(output[..count].iter().enumerate()
            .map(|(index, row)| core_row(row, index == 0 || object_is_term)));
    }
    let receipt = seal(host, &rows)?;
    Ok(Value::Record(BTreeMap::from([
        ("quins".into(), Value::List(rows.iter().map(quin_value).collect())),
        ("receipt".into(), receipt),
    ])))
}

pub(super) fn derive(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["id", "submission", "agreement", "acceptance", "valuation", "now_unix"])?;
    let id = text(required(params, "id")?)?;
    let now = timestamp(params)?;
    let mut size = 0;
    let submission = read!(params, &mut size, "submission", SubmissionOrClaim);
    let agreement = match params.get("agreement") {
        None => None,
        Some(_) => Some(read!(params, &mut size, "agreement", RequestAndAgreement)),
    };
    let acceptance = match params.get("acceptance") {
        None => None,
        Some(_) => Some(read!(params, &mut size, "acceptance", AcceptanceDecision)),
    };
    let valuation = match params.get("valuation") {
        None => None,
        Some(_) => Some(read!(params, &mut size, "valuation", Valuation)),
    };
    if id.is_empty() || id.len() > 1024 || submission.submitted_at_unix > now
        || agreement.as_ref().is_some_and(|row| row.project_id != submission.project_id
            || row.contributor_did != submission.contributor_did
            || submission.agreement_id.as_deref() != Some(row.id.as_str())
            || row.created_at_unix > submission.submitted_at_unix)
        || acceptance.as_ref().is_some_and(|row| row.project_id != submission.project_id
            || row.submission_id != submission.id || row.agreement_id != submission.agreement_id
            || row.decided_at_unix < submission.submitted_at_unix || row.decided_at_unix > now)
        || valuation.as_ref().is_some_and(|row| row.project_id != submission.project_id
            || row.submission_id != submission.id || acceptance.as_ref().is_none_or(|accept|
                accept.id != row.acceptance_id || row.valued_at_unix < accept.decided_at_unix)
            || row.valued_at_unix > now)
    {
        return Err("Lifecycle stage identities or times do not link to this submission".into());
    }
    host.charge(size, size)?;
    let obligation = derive_obligation_from_lifecycle(id, &submission, agreement.as_ref(),
        acceptance.as_ref(), valuation.as_ref(), now)?;
    let mut output = [CooperativeQuin::default(); 2];
    let count = obligation.compile_to_quins(&mut output);
    let rows: Vec<_> = output[..count].iter().enumerate()
        .map(|(index, row)| core_row(row, index == 0)).collect();
    let receipt = seal(host, &rows)?;
    let value = serde_json::to_value(obligation).map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("obligation".into(), from_json(value)),
        ("receipt".into(), receipt),
    ])))
}

pub(super) fn transition(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["decision", "obligations", "allocations"])?;
    let mut size = 0;
    let decision = read!(params, &mut size, "decision", TransitionDecision);
    let obligations = read!(params, &mut size, "obligations", Vec<LifecycleObligation>);
    let allocations = read!(params, &mut size, "allocations", Vec<SettlementAllocation>);
    if obligations.len() + allocations.len() > MAX_RECORDS {
        return Err("Transition assessment exceeds lifecycle record budget".into());
    }
    let mut linked = BTreeMap::new();
    for row in &obligations {
        if row.id.is_empty() || row.project_id != decision.project_id || row.principal_cents <= 0
            || row.created_at_unix > decision.decided_at_unix || linked.insert(&row.id, row).is_some()
        {
            return Err("Transition has a conflicting, foreign or invalid obligation".into());
        }
    }
    let mut ids = BTreeSet::new();
    let mut receipts = BTreeSet::new();
    for row in &allocations {
        let obligation = linked.get(&row.obligation_id);
        if row.project_id != decision.project_id || row.allocated_cents <= 0
            || row.settled_at_unix > decision.decided_at_unix || !ids.insert(&row.id)
            || obligation.is_none_or(|ob| row.beneficiary_did != ob.beneficiary_did
                || row.settled_at_unix < ob.created_at_unix)
            || row.beneficiary_receipt_hash.is_some_and(|receipt|
                receipt == 0 || !receipts.insert((&row.obligation_id, receipt)))
        {
            return Err("Transition has a conflicting or unlinked settlement allocation".into());
        }
    }
    host.charge(size.saturating_add(obligations.len() * allocations.len()), size)?;
    let outcome = evaluate_transition_readiness(&decision, &obligations, &allocations);
    let recomputed: Vec<_> = obligations.iter().map(|obligation|
        apply_settlements_and_recognition(obligation, &allocations, &[])).collect();
    let mut output = [CooperativeQuin::default(); 2];
    let count = decision.compile_to_quins(&mut output);
    let rows: Vec<_> = output[..count].iter().map(|row| core_row(row, true)).collect();
    let receipt = seal(host, &rows)?;
    Ok(Value::Record(BTreeMap::from([
        ("ready".into(), Value::Bool(outcome.is_ready())),
        ("outcome".into(), from_json(serde_json::to_value(outcome).map_err(|error| error.to_string())?)),
        ("obligations".into(), from_json(serde_json::to_value(recomputed).map_err(|error| error.to_string())?)),
        ("receipt".into(), receipt),
    ])))
}

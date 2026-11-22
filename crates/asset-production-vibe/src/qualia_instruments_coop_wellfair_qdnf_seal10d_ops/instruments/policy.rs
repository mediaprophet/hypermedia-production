//! Pure consent and geospatial policy evaluation over admitted snapshots.

use super::super::{fields, json, quin_value, required, resource, text, timestamp, unsigned, Args,
    AssetHost, Value, SYMBOLIC_LIMIT};
use super::row_receipt;
use crate::{list, record};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    domains::geospatial::{
        steward::{validate_steward_unlock, StewardContract, StewardVerdict},
        triggers::LocationTriggerEngine,
    },
    governance::consent_contract::{ConsentGrant, ConsentLedger, ConsentScope, RevocationReceipt,
        MAX_CONSENT_LEDGER},
    q_hash, NQuin,
};
use std::collections::{BTreeMap, BTreeSet};

const MAX_ROWS: usize = 8192;

pub(super) fn consent(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["grant", "revocations", "actor", "category", "purpose_hash", "now_unix"])?;
    let mut size = 0;
    let grant: ConsentGrant = serde_json::from_value(json(required(params, "grant")?, &mut size, 0)?)
        .map_err(|error| format!("Invalid consent grant: {error}"))?;
    let receipts = list(required(params, "revocations")?)?;
    if receipts.len() > MAX_CONSENT_LEDGER {
        return Err("Consent snapshot exceeds revocation ledger capacity".into());
    }
    let revocations: Vec<RevocationReceipt> = receipts.iter().map(|row| {
        let json = json(row, &mut size, 0)?;
        serde_json::from_value(json).map_err(|error| format!("Invalid revocation receipt: {error}"))
    }).collect::<Result<_, _>>()?;
    let actor: [u8; 32] = serde_json::from_value(json(required(params, "actor")?, &mut size, 0)?)
        .map_err(|error| format!("Invalid recipient key: {error}"))?;
    let category = ConsentScope::category_from_family(text(required(params, "category")?)?)
        .ok_or("Unknown health consent category")?;
    let now = unsigned(required(params, "now_unix")?)?;
    let purpose = unsigned(required(params, "purpose_hash")?)?;
    if grant.scope.0 & !ConsentScope::ALL != 0 || now < grant.created_at || purpose != grant.purpose_hash {
        return Err("Consent snapshot has an unsupported scope, mismatched purpose or future grant".into());
    }
    host.charge(size, size)?;
    let mut ledger = ConsentLedger::new();
    let decision = ledger.issue(&grant, now).and_then(|()| {
        for receipt in &revocations {
            ledger.revoke(&grant, receipt)?;
        }
        ledger.authorize_category_access(&grant, &[], category, &actor, now)
    });
    let mut values = BTreeMap::from([("authorised".into(), Value::Bool(decision.is_ok()))]);
    if let Err(error) = decision {
        let reason = match error {
            qualia_core_db::governance::consent_contract::ConsentError::Expired { .. } => "expired",
            qualia_core_db::governance::consent_contract::ConsentError::Revoked { .. } => "revoked",
            qualia_core_db::governance::consent_contract::ConsentError::RequesterMismatch => "requester_mismatch",
            qualia_core_db::governance::consent_contract::ConsentError::ScopeViolation { .. } => "scope_violation",
            _ => "invalid_consent_evidence",
        };
        values.insert("reason".into(), Value::String(reason.into()));
    } else {
        let mut row = grant.to_deontic_quin(category);
        row.recalculate_parity();
        values.insert("receipt".into(), row_receipt(host, &row)?);
    }
    Ok(Value::Record(values))
}

fn graph(host: &AssetHost<'_>, value: &Value) -> Result<SemanticModelBuilder, String> {
    let bytes = &resource(host, value, "q42")?.bytes;
    let graph = SemanticModelBuilder::new(host.limits.resource_bytes.min(SYMBOLIC_LIMIT))
        .merge_q42(bytes).map_err(|error| error.to_string())?;
    if graph.snapshot_quins().len() > MAX_ROWS {
        return Err("Policy snapshot exceeds evaluation row budget".into());
    }
    Ok(graph)
}

pub(super) fn steward(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["arena", "requester", "steward", "asset", "scope", "expiry_unix",
        "quorum", "now_unix"])?;
    let now = timestamp(params)?;
    let value = |name| unsigned(required(params, name)?);
    let requester = value("requester")?;
    let contract = StewardContract {
        steward_did_hash: value("steward")?, asset_hash: value("asset")?, scope: value("scope")?,
        expiry_unix: u32::try_from(value("expiry_unix")?).map_err(|_| "Invalid steward expiry")?,
        quorum: u8::try_from(value("quorum")?).map_err(|_| "Invalid steward quorum")?,
    };
    if [requester, contract.steward_did_hash, contract.asset_hash, contract.scope].contains(&0)
        || contract.quorum == 0
    {
        return Err("Steward evaluation requires nonzero identities and quorum".into());
    }
    let graph = graph(host, required(params, "arena")?)?;
    let p = q_hash("q42:stewards");
    let mut seen = BTreeSet::new();
    let admitted: Vec<_> = graph.snapshot_quins().iter().copied().filter(|row| {
        row.predicate != p || (row.context == contract.scope
            && seen.insert((row.subject, row.predicate, row.object, row.context)))
    }).collect();
    host.charge(admitted.len(), admitted.len() * std::mem::size_of::<NQuin>())?;
    let (outcome, reason) = match validate_steward_unlock(requester, &contract, &admitted, now) {
        StewardVerdict::Granted => ("granted", None),
        StewardVerdict::Denied(reason) => ("denied", Some(reason)),
        StewardVerdict::Expired => ("expired", None),
    };
    let mut result = BTreeMap::from([("outcome".into(), Value::String(outcome.into()))]);
    if let Some(reason) = reason {
        result.insert("reason".into(), Value::String(reason.into()));
    }
    Ok(Value::Record(result))
}

pub(super) fn location_triggers(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["arena", "location", "rules"])?;
    let graph = graph(host, required(params, "arena")?)?;
    let location = unsigned(required(params, "location")?)?;
    let rules = list(required(params, "rules")?)?;
    if location == 0 || rules.len() > 256 {
        return Err("Location trigger ID is empty or rule count exceeds budget".into());
    }
    host.charge(rules.len().saturating_mul(graph.snapshot_quins().len().saturating_add(1)),
        rules.len() * std::mem::size_of::<NQuin>())?;
    let mut engine = LocationTriggerEngine::new();
    for rule in rules {
        let rule = record(rule)?;
        fields(rule, &["region", "quin"])?;
        let region = unsigned(required(rule, "region")?)?;
        let [s, p, o, c, m, parity] = list(required(rule, "quin")?)? else {
            return Err("Location trigger requires the six NQuin columns".into());
        };
        let row = NQuin {
            subject: unsigned(s)?, predicate: unsigned(p)?, object: unsigned(o)?,
            context: unsigned(c)?, metadata: unsigned(m)?, parity: unsigned(parity)?,
        };
        if region == 0 || !row.verify_ecc_parity() {
            return Err("Location trigger contains an invalid region or row checksum".into());
        }
        engine.register_trigger(region, row);
    }
    let mut fired = engine.on_enter(location, graph.snapshot_quins());
    fired.sort_unstable_by_key(|row| (row.subject, row.predicate, row.object, row.context,
        row.metadata, row.parity));
    fired.dedup();
    Ok(Value::Record(BTreeMap::from([(
        "quins".into(), Value::List(fired.iter().map(quin_value).collect()),
    )])))
}

//! Semantic instruments, TK/BC and scoped policy snapshots.

use super::{fields, from_json, json, quin_value, required, text, timestamp, unsigned, Args,
    AssetHost, Value, INPUT_BYTES, SYMBOLIC_LIMIT};
use crate::{boolean, list, record};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    governance::tk_bc_labels::{evaluate_tk_bc_policy, TkAccessContext, TkBcLabelFlags,
        TkEvaluationVerdict},
    modalities::capability_gap::MAX_CAP_NODES,
    semantic_instruments::{evaluate_core_semantic_instrument, evaluate_rpl, CoreInstrumentPayload,
        EvidenceBasis, EvidenceItem},
};
use std::collections::BTreeMap;

mod collectable;
mod policy;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "evaluate" => instrument(host, params),
        "rpl" => rpl(host, params),
        "tk_bc" => tk_bc(host, params),
        "collectable_build" => collectable::build(host, params),
        "collectable_open" => collectable::open(host, params),
        "consent_snapshot" => policy::consent(host, params),
        "steward_unlock" => policy::steward(host, params),
        "location_triggers" => policy::location_triggers(host, params),
        _ => Err(format!("Unsupported semantic instrument operation: {op}")),
    }
}

fn instrument(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["payload", "now_unix"])?;
    let mut bytes = 0;
    let input = json(required(params, "payload")?, &mut bytes, 0)?;
    let payload: CoreInstrumentPayload = serde_json::from_value(input.clone())
        .map_err(|error| format!("Invalid core instrument payload: {error}"))?;
    let canonical = serde_json::to_value(&payload).map_err(|error| error.to_string())?;
    if let (Some(input), Some(canonical)) = (input.as_object(), canonical.as_object()) {
        if let Some(key) = input.keys().find(|key| !canonical.contains_key(*key)) {
            return Err(format!("Unknown core instrument payload field: {key}"));
        }
    }
    let now = timestamp(params)?;
    host.charge(bytes, bytes)?;
    let report = evaluate_core_semantic_instrument(&payload, now);
    if !report.receipt_quin.verify_ecc_parity() {
        return Err("Core instrument receipt parity failed".into());
    }
    let findings = serde_json::to_value(&report.findings).map_err(|error| error.to_string())?;
    let receipt = row_receipt(host, &report.receipt_quin)?;
    Ok(Value::Record(BTreeMap::from([
        ("instrument".into(), Value::String(report.instrument.slug().into())),
        ("outcome".into(), Value::String(report.outcome.as_str().into())),
        ("findings".into(), from_json(findings)),
        ("quin".into(), quin_value(&report.receipt_quin)),
        ("receipt".into(), receipt),
    ])))
}

fn row_receipt(host: &mut AssetHost<'_>, row: &qualia_core_db::NQuin) -> Result<Value, String> {
    let budget = host.limits.resource_bytes.min(SYMBOLIC_LIMIT);
    let mut graph = SemanticModelBuilder::new(budget);
    graph.restrict_to_sanctuary();
    let bytes = graph.seal_projection(std::iter::once(*row))
        .map_err(|error| error.to_string())?;
    host.encode("q42", bytes)
}

fn tk_bc(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    const KNOWN: u32 = TkBcLabelFlags::TK_ATTRIBUTION.0
        | TkBcLabelFlags::TK_NON_COMMERCIAL.0 | TkBcLabelFlags::TK_CULTURALLY_SENSITIVE.0
        | TkBcLabelFlags::TK_COMMUNITY_VERIFIED.0 | TkBcLabelFlags::BC_PROVENANCE.0
        | TkBcLabelFlags::BC_MULTIPLE_COMMUNITY.0;
    fields(params, &["labels", "is_commercial_use", "has_attribution", "community_consent_signed",
        "observer_is_verified_custodian"])?;
    let bits = u32::try_from(unsigned(required(params, "labels")?)?)
        .map_err(|_| "TK/BC labels exceed their native bitmask")?;
    if bits & !KNOWN != 0 {
        return Err("TK/BC label bitmask contains unknown flags".into());
    }
    let ctx = TkAccessContext {
        is_commercial_use: boolean(params, "is_commercial_use", true)?,
        has_attribution: boolean(params, "has_attribution", false)?,
        community_consent_signed: boolean(params, "community_consent_signed", false)?,
        observer_is_verified_custodian: boolean(params, "observer_is_verified_custodian", false)?,
    };
    host.charge(1, 0)?;
    let verdict = evaluate_tk_bc_policy(TkBcLabelFlags(bits), &ctx);
    let label = match verdict {
        TkEvaluationVerdict::Permitted => "permitted",
        TkEvaluationVerdict::ProhibitedCommercialWithoutConsent => "prohibited_commercial_without_consent",
        TkEvaluationVerdict::ProhibitedMissingAttribution => "prohibited_missing_attribution",
        TkEvaluationVerdict::RestrictedCulturalSensitivity => "restricted_cultural_sensitivity",
    };
    Ok(Value::Record(BTreeMap::from([
        ("outcome".into(), Value::String(label.into())),
        ("permitted".into(), Value::Bool(verdict == TkEvaluationVerdict::Permitted)),
        ("labels".into(), Value::U64(u64::from(bits))),
    ])))
}

fn bounded_list<'a>(params: &'a Args, name: &str) -> Result<&'a [Value], String> {
    let rows = params.get(name).map(list).transpose()?.unwrap_or(&[]);
    if rows.len() > MAX_CAP_NODES {
        return Err(format!("RPL {name} exceeds the bounded capability graph"));
    }
    Ok(rows)
}

fn rpl(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["required", "evidence", "authorised_equivalences",
        "unauthorised_equivalences", "prerequisites", "goal"])?;
    let budget = host.limits.resource_bytes.min(INPUT_BYTES);
    let mut terms = SemanticModelBuilder::new(budget);
    let mut nodes = std::collections::BTreeSet::new();
    let mut iri = |value: &Value| -> Result<String, String> {
        let value = text(value)?;
        if value.len() > 2048 {
            return Err("RPL IRI exceeds byte budget".into());
        }
        nodes.insert(terms.bind_iri(value).map_err(|error| error.to_string())?);
        if nodes.len() > MAX_CAP_NODES {
            return Err("RPL distinct capabilities exceed the bounded graph".into());
        }
        Ok(value.to_owned())
    };
    let required = bounded_list(params, "required")?.iter().map(&mut iri)
        .collect::<Result<Vec<_>, _>>()?;
    if required.is_empty() {
        return Err("RPL requires capability requirements".into());
    }
    let evidence = bounded_list(params, "evidence")?.iter().map(|item| {
        let row = record(item)?;
        fields(row, &["capability", "basis", "available", "private"])?;
        let basis = match text(super::required(row, "basis")?)? {
            "formal" => EvidenceBasis::Formal,
            "prior_learning" => EvidenceBasis::PriorLearning,
            "experiential" => EvidenceBasis::Experiential,
            "peer_attested" => EvidenceBasis::PeerAttested,
            _ => return Err("Unknown RPL evidence basis".into()),
        };
        Ok(EvidenceItem {
            capability: iri(super::required(row, "capability")?)?, basis,
            available: boolean(row, "available", false)?, private: boolean(row, "private", true)?,
        })
    }).collect::<Result<Vec<_>, String>>()?;
    let mut pairs = |name| -> Result<Vec<_>, String> {
        bounded_list(params, name)?.iter().map(|item| match list(item)? {
            [left, right] => Ok((iri(left)?, iri(right)?)),
            _ => Err("RPL equivalence requires exactly two IRIs".into()),
        }).collect()
    };
    let certified = pairs("authorised_equivalences")?;
    let proposed = pairs("unauthorised_equivalences")?;
    let prereqs = bounded_list(params, "prerequisites")?.iter().map(|item| match list(item)? {
        [from, to, cost] => Ok((iri(from)?, iri(to)?, u32::try_from(unsigned(cost)?)
            .map_err(|_| "RPL prerequisite cost exceeds u32")?)),
        _ => Err("RPL prerequisite requires source IRI, target IRI and cost".into()),
    }).collect::<Result<Vec<_>, String>>()?;
    let goal = params.get("goal").map(iri).transpose()?;
    host.charge(MAX_CAP_NODES * MAX_CAP_NODES, budget)?;
    let mut scratch = [0; MAX_CAP_NODES];
    let gap = evaluate_rpl(&required, &evidence, &certified, &proposed, &prereqs, goal.as_deref(),
        &mut scratch).map_err(|error| error.to_string())?;
    let names = |names: Vec<String>| Value::List(names.into_iter().map(Value::String).collect());
    Ok(Value::Record(BTreeMap::from([
        ("satisfied".into(), names(gap.satisfied)),
        ("equivalent".into(), names(gap.equivalent)),
        ("unresolved".into(), names(gap.unresolved)),
        ("unmet".into(), names(gap.unmet)),
        ("learning_path_cost".into(), gap.learning_path_cost
            .map_or(Value::Null, |value| Value::U64(u64::from(value)))),
    ])))
}

//! Request-scoped use of the native conditioning compiler.

use super::{exact_u64, fields, required, text, Args, AssetHost, ReceiptGraph, Value};
use crate::{list, record};
use qualia_core_db::inference::conditioning::{
    compile_into, encode_plan_cbor, validate_spec, AuthorityView, BackendCapabilities,
    CompileBuffers, ConditioningBudget, ConditioningSpec, EvidencePart, OutputContractRef,
    RequirementClass, RequirementDisposition, RequirementOutcome, RequirementRef,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const MAX_REQUIREMENTS: usize = 64;
const MAX_PROFILE_BYTES: usize = 64 * 1024;

fn require_bool(row: &Args, name: &str) -> Result<bool, String> {
    match required(row, name)? {
        Value::Bool(value) => Ok(*value),
        _ => Err(format!("{name} must be a boolean")),
    }
}

fn read_requirement(value: &Value) -> Result<RequirementRef<'_>, String> {
    let row = record(value)?;
    fields(row, &["id", "class", "rule", "validator", "required", "priority"])?;
    let class = match text(required(row, "class")?)? {
        "guidance" => RequirementClass::Guidance,
        "enforced" => RequirementClass::Enforced,
        "evidence_obligation" => RequirementClass::EvidenceObligation,
        _ => return Err("Unsupported conditioning requirement class".into()),
    };
    let priority = u8::try_from(exact_u64(required(row, "priority")?)?)
        .map_err(|_| "Conditioning priority must fit in one byte")?;
    Ok(RequirementRef {
        id: text(required(row, "id")?)?,
        class,
        rule: text(required(row, "rule")?)?,
        validator: row.get("validator").map(text).transpose()?,
        required: require_bool(row, "required")?,
        priority,
    })
}

fn disposition(value: RequirementDisposition) -> &'static str {
    match value {
        RequirementDisposition::Applied => "applied",
        RequirementDisposition::Enforced => "enforced",
        RequirementDisposition::Degraded => "degraded",
        RequirementDisposition::Rejected => "rejected",
        RequirementDisposition::NotApplicable => "not_applicable",
    }
}

pub(super) fn profile(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    fields(params, &["profile_id", "objective", "requirements", "input_tokens", "output_tokens"])?;
    let input = params.get("requirements").map(list).transpose()?.unwrap_or(&[]);
    if input.len() > MAX_REQUIREMENTS {
        return Err("Conditioning requirement count exceeds native compiler bounds".into());
    }
    let requirements = input.iter().map(read_requirement).collect::<Result<Vec<_>, _>>()?;
    let profile_id = text(required(params, "profile_id")?)?;
    let objective = text(required(params, "objective")?)?;
    let mut input_bytes = profile_id.len().saturating_add(objective.len());
    for req in &requirements {
        input_bytes = input_bytes.saturating_add(req.id.len()).saturating_add(req.rule.len())
            .saturating_add(req.validator.map_or(0, str::len));
    }
    let budget = host.limits.resource_bytes.min(MAX_PROFILE_BYTES);
    if input_bytes > budget {
        return Err("Conditioning source exceeds profile byte budget".into());
    }
    host.charge(input_bytes, budget)?;
    let get_tokens = |name| -> Result<u32, String> {
        let tokens = u32::try_from(exact_u64(required(params, name)?)?)
            .map_err(|_| format!("{name} exceeds token budget field"))?;
        if tokens == 0 {
            return Err(format!("{name} must be positive"));
        }
        Ok(tokens)
    };
    let spec = ConditioningSpec {
        schema_version: 1,
        profile_id,
        objective,
        requirements: &requirements,
        domain_refs: &[],
        output_contract: OutputContractRef { schema_hint: None, min_citations: 0 },
        budget: ConditioningBudget {
            input_tokens: get_tokens("input_tokens")?,
            output_tokens: get_tokens("output_tokens")?,
            tool_rounds: 0,
            max_bytes: budget as u32,
            thinking_token_budget: None,
        },
    };
    let authority = AuthorityView {
        principal_did_hash: 0, disclosure_ceiling: 0, allowed_graph_scopes: &[], tools_allowed: false,
    };
    validate_spec(&spec, &authority).map_err(|error| format!("Conditioning spec refused: {error:?}"))?;
    if op == "conditioning_validate" {
        return Ok(Value::Record(BTreeMap::from([
            ("valid".into(), Value::Bool(true)),
            ("profile_id".into(), Value::String(profile_id.into())),
        ])));
    }
    let mut outcomes = [RequirementOutcome::applied(""); MAX_REQUIREMENTS];
    let mut evidence: [EvidencePart<'_>; 0] = [];
    let mut buffers = CompileBuffers { outcomes: &mut outcomes, selected_evidence: &mut evidence };
    let summary = compile_into(&spec, &authority, &BackendCapabilities::native_gguf_baseline(),
        &[], &mut buffers).map_err(|error| format!("Conditioning compile refused: {error:?}"))?;
    if let Some(rejected) = outcomes[..summary.outcomes_count].iter()
        .find(|outcome| outcome.disposition == RequirementDisposition::Rejected)
    {
        return Err(format!("Required conditioning evidence unavailable for {}", rejected.id));
    }
    let mut plan = vec![0; budget.min(profile_id.len().saturating_add(objective.len()).saturating_add(64))];
    let len = encode_plan_cbor(&summary, &mut plan)
        .map_err(|error| format!("Conditioning plan encoding failed: {error:?}"))?;
    plan.truncate(len);
    let iri = format!("urn:sha256:{:x}", Sha256::digest(&plan));
    let mut receipt = ReceiptGraph::new("https://qualiadb.org/ontology/q42#ConditioningReceipt");
    receipt.fact(&iri, "https://qualiadb.org/ontology/q42#conditioningBackend", "urn:qualia:conditioning:native-gguf")?;
    let receipt = receipt.seal(budget)?;
    let rows = outcomes[..summary.outcomes_count].iter().map(|row| {
        let mut fields = BTreeMap::from([
            ("id".into(), Value::String(row.id.into())),
            ("disposition".into(), Value::String(disposition(row.disposition).into())),
        ]);
        if let Some(reason) = row.reason {
            fields.insert("reason".into(), Value::String(reason.into()));
        }
        Value::Record(fields)
    }).collect();
    Ok(Value::Record(BTreeMap::from([
        ("plan".into(), host.encode("cplan", plan)?),
        ("plan_id".into(), Value::U64(summary.plan_id)),
        ("receipt".into(), host.encode("q42", receipt)?),
        ("requirements".into(), Value::List(rows)),
    ])))
}

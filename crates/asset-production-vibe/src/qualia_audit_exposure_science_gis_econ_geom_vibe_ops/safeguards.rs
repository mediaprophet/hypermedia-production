//! Offline WellFair policy scenarios over bounded, caller-authored fixtures.

use crate::{boolean, fields, integer, list, record, required, text, Args, AssetHost, Value};
use qualia_core_db::{
    governance::cohort_access_gate::{
        authorize_derivative_or_explanation_export, filter_authorized_cohort_quins,
        pack_context_with_sensitivity, AccessGateError, PrincipalAndAgentGrant, CONTEXT_GRAPH_MASK,
        SENSITIVITY_CLASSIFIED, SENSITIVITY_PUBLIC, SENSITIVITY_RESTRICTED,
    },
    modalities::ctl::{self, MAX_CTL_STATES},
    q_hash, NQuin,
};
use std::{collections::BTreeMap, mem::size_of};

const MAX_COHORT_ROWS: usize = 256;
const MAX_CTL_ROWS: usize = 2048;
const MAX_CTL_PROPOSITIONS: usize = 256;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "cohort_access_gate" => cohort_access(host, params),
        "exists_next" | "always_next" | "exists_finally" | "all_finally" | "exists_globally"
        | "always_globally" | "exists_until" | "all_until" | "fair_globally" => {
            check_ctl(host, op, params)
        }
        _ => Err(format!("Unsupported offline WellFair operation: {op}")),
    }
}

fn uint(args: &Args, key: &str) -> Result<u64, String> {
    match required(args, key)? {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| format!("Invalid {key}")),
        _ => Err(format!("{key} requires an unsigned integer")),
    }
}

fn context(args: &Args) -> Result<u64, String> {
    let context = uint(args, "context")?;
    if context > CONTEXT_GRAPH_MASK {
        return Err("Context exceeds the native 56-bit graph field".into());
    }
    Ok(context)
}

fn sensitivity(args: &Args, key: &str) -> Result<u8, String> {
    match text(required(args, key)?)? {
        "public" => Ok(SENSITIVITY_PUBLIC),
        "restricted" => Ok(SENSITIVITY_RESTRICTED),
        "classified" => Ok(SENSITIVITY_CLASSIFIED),
        _ => Err(format!("{key} must be public, restricted, or classified")),
    }
}

fn sensitivity_name(class: u8) -> &'static str {
    match class {
        SENSITIVITY_PUBLIC => "public",
        SENSITIVITY_RESTRICTED => "restricted",
        SENSITIVITY_CLASSIFIED => "classified",
        _ => unreachable!("Fixture classes are validated before calling the native gate"),
    }
}

fn gate_code(error: AccessGateError) -> String {
    match error {
        AccessGateError::MissingHumanPrincipal => "missing_human_principal",
        AccessGateError::GrantRevoked => "grant_revoked",
        AccessGateError::GrantExpired { .. } => "grant_expired",
        AccessGateError::GrantNotYetValid { .. } => "grant_not_yet_valid",
        AccessGateError::InvalidEpochRange { .. } => "invalid_epoch_range",
        AccessGateError::SensitivityExceeded { .. } => "sensitivity_exceeded",
        AccessGateError::ContextUnauthorized { .. } => "context_unauthorized",
        AccessGateError::DeclassificationForbidden { .. } => "declassification_forbidden",
        AccessGateError::ExplanationTraceForbidden => "explanation_trace_forbidden",
        AccessGateError::DerivativeExportForbidden => "derivative_export_forbidden",
        AccessGateError::OutputBufferTooSmall { .. } => "output_buffer_too_small",
    }
    .into()
}

fn quin(subject: u64, predicate: u64, object: u64, context: u64) -> NQuin {
    let mut row = NQuin {
        subject,
        predicate,
        object,
        context,
        metadata: 0,
        parity: 0,
    };
    row.recalculate_parity();
    row
}

fn cohort_access(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "sources",
            "grant",
            "current_epoch",
            "target_sensitivity",
            "release_kind",
        ],
    )?;
    let sources = list(required(params, "sources")?)?;
    if sources.is_empty() || sources.len() > MAX_COHORT_ROWS {
        return Err("Cohort fixture requires 1..=256 source rows".into());
    }
    let fixture_grant = record(required(params, "grant")?)?;
    fields(
        fixture_grant,
        &[
            "has_human_principal",
            "delegated",
            "context",
            "max_sensitivity",
            "allow_derivative_export",
            "allow_explanation_trace",
            "issued_epoch",
            "expiry_epoch",
            "revoked",
        ],
    )?;
    // The synthetic principal and delegate are fixture-local roles. The caller only
    // supplies policy settings, epochs and graph scope indices.
    let grant = PrincipalAndAgentGrant {
        human_principal_did_hash: boolean(fixture_grant, "has_human_principal", false)? as u64,
        agent_did_hash: (boolean(fixture_grant, "delegated", false)? as u64) * 2,
        authorized_context_hash: context(fixture_grant)?,
        max_sensitivity_class: sensitivity(fixture_grant, "max_sensitivity")?,
        allow_derivative_export: boolean(fixture_grant, "allow_derivative_export", false)?,
        allow_explanation_trace: boolean(fixture_grant, "allow_explanation_trace", false)?,
        issued_epoch: uint(fixture_grant, "issued_epoch")?,
        expiry_epoch: uint(fixture_grant, "expiry_epoch")?,
        revoked: boolean(fixture_grant, "revoked", false)?,
    };
    let epoch = uint(params, "current_epoch")?;
    let target = sensitivity(params, "target_sensitivity")?;
    let is_explanation = match text(required(params, "release_kind")?)? {
        "derivative" => false,
        "explanation" => true,
        _ => return Err("release_kind must be derivative or explanation".into()),
    };
    host.charge(sources.len() * 8, 2 * MAX_COHORT_ROWS * size_of::<NQuin>())?;
    let mut rows = [NQuin::default(); MAX_COHORT_ROWS];
    let relation = q_hash("urn:qualia:wellfair:fixtureSensitivity");
    for (index, (row, input)) in rows.iter_mut().zip(sources).enumerate() {
        let input = record(input)?;
        fields(input, &["context", "sensitivity"])?;
        let class = sensitivity(input, "sensitivity")?;
        *row = quin(
            index as u64 + 1,
            relation,
            u64::from(class) + 1,
            pack_context_with_sensitivity(context(input)?, class),
        );
    }
    let rows = &rows[..sources.len()];
    let mut filtered = [NQuin::default(); MAX_COHORT_ROWS];
    let counts = filter_authorized_cohort_quins(rows, &grant, epoch, &mut filtered);
    // Check the whole modeled dependency set so reading a permitted subset cannot
    // lower a derivative's classification. Discard the native authorization token.
    let release =
        authorize_derivative_or_explanation_export(rows, &grant, target, is_explanation, epoch);
    let mut result = BTreeMap::from([
        ("filter_succeeded".into(), Value::Bool(counts.is_ok())),
        ("release_authorized".into(), Value::Bool(release.is_ok())),
    ]);
    match counts {
        Ok(counts) => {
            result.insert(
                "all_sources_admitted".into(),
                Value::Bool(counts.is_all_admitted()),
            );
            result.insert(
                "admitted_count".into(),
                Value::U64(counts.admitted_count as u64),
            );
            result.insert(
                "redacted_count".into(),
                Value::U64(counts.redacted_count as u64),
            );
            result.insert(
                "inherited_max_sensitivity".into(),
                Value::String(sensitivity_name(counts.inherited_max_sensitivity).into()),
            );
        }
        Err(error) => {
            result.insert("read_denial".into(), Value::String(gate_code(error)));
        }
    }
    if let Err(error) = release {
        result.insert("release_denial".into(), Value::String(gate_code(error)));
    }
    Ok(Value::Record(result))
}

fn bounded_index(value: &Value, limit: usize, name: &str) -> Result<usize, String> {
    let index = integer(value, name)?;
    if index >= limit {
        return Err(format!("{name} is outside 0..{limit}"));
    }
    Ok(index)
}

fn check_ctl(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    let until = matches!(op, "exists_until" | "all_until");
    let fair = op == "fair_globally";
    let mut keys = ["state_labels", "transitions", "start", "proposition", ""];
    keys[4] = if until { "until" } else { "fairness" };
    fields(params, &keys[..if fair || until { 5 } else { 4 }])?;
    let states = list(required(params, "state_labels")?)?;
    let transitions = list(required(params, "transitions")?)?;
    if states.is_empty() || states.len() > MAX_CTL_STATES {
        return Err(format!("CTL requires 1..={MAX_CTL_STATES} states"));
    }
    let start = bounded_index(required(params, "start")?, states.len(), "start")?;
    let proposition = bounded_index(
        required(params, "proposition")?,
        MAX_CTL_PROPOSITIONS,
        "proposition",
    )?;
    let additional = if fair || until {
        bounded_index(required(params, keys[4])?, MAX_CTL_PROPOSITIONS, keys[4])? as u64 + 1
    } else {
        0
    };
    let mut row_count = transitions.len();
    for state in states {
        row_count = row_count
            .checked_add(list(state)?.len())
            .ok_or("CTL row count overflow")?;
        if row_count > MAX_CTL_ROWS {
            return Err(format!(
                "CTL facts and transitions exceed {MAX_CTL_ROWS} rows"
            ));
        }
    }
    let state_work = states.len() + 1;
    host.charge(
        row_count.max(1) * state_work * state_work,
        MAX_CTL_ROWS * size_of::<NQuin>() + 8 * MAX_CTL_STATES * size_of::<u64>(),
    )?;

    let mut graph = [NQuin::default(); MAX_CTL_ROWS];
    let mut graph_len = 0;
    let next = q_hash("ctl:next");
    let holds = q_hash("ctl:holds");
    let mut invariant_states = [false; MAX_CTL_STATES];
    for (state, labels) in states.iter().enumerate() {
        for label in list(labels)? {
            let label = bounded_index(label, MAX_CTL_PROPOSITIONS, "state label")?;
            invariant_states[state] |= label == proposition;
            graph[graph_len] = quin(state as u64 + 1, holds, label as u64 + 1, 0);
            graph_len += 1;
        }
    }
    for edge in transitions {
        let [from, to] = list(edge)? else {
            return Err("Each transition must contain [from, to]".into());
        };
        let from = bounded_index(from, states.len(), "transition from")?;
        let to = bounded_index(to, states.len(), "transition to")?;
        // The native fairness traversal initially visits all reachable states.
        // Restrict it to the invariant subgraph for the whole path semantics of EG.
        if fair && !(invariant_states[from] && invariant_states[to]) {
            continue;
        }
        graph[graph_len] = quin(from as u64 + 1, next, to as u64 + 1, 0);
        graph_len += 1;
    }

    let graph = &graph[..graph_len];
    let initial = start as u64 + 1;
    let prop = proposition as u64 + 1;
    let satisfied = match op {
        "exists_next" => ctl::exists_next(graph, initial, prop, next, holds),
        "always_next" => ctl::always_next(graph, initial, prop, next, holds),
        "exists_finally" => ctl::exists_finally(graph, initial, prop, next, holds),
        "all_finally" => ctl::all_finally(graph, initial, prop, next, holds),
        "exists_globally" => ctl::exists_globally(graph, initial, prop, next, holds),
        "always_globally" => ctl::always_globally(graph, initial, prop, next, holds),
        "exists_until" => ctl::exists_until(graph, initial, prop, additional, next, holds),
        "all_until" => ctl::all_until(graph, initial, prop, additional, next, holds),
        "fair_globally" => {
            invariant_states[start]
                && ctl::fair_globally(graph, initial, prop, additional, next, holds)
        }
        _ => unreachable!("CTL dispatch validates the operator"),
    };
    Ok(Value::Record(BTreeMap::from([(
        "satisfied".into(),
        Value::Bool(satisfied),
    )])))
}

//! Deterministic, request-scoped cooperative ledger calculations.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{fields, list, record, required, text, Args, AssetHost, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use qualia_cooperative_core::finance::{div_round_bankers, JournalEntryRecord, JournalLine, LedgerBook};
use std::collections::{BTreeMap, BTreeSet};

const GRAPH: &str = "urn:qualia:cooperative:inventory-allocation:v1";

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "allocate_landed_cost" => landed_cost(host, args),
        "apply_basis_points" => basis_points(args),
        _ => Err(format!("Unsupported cooperative ledger operation: {op}")),
    }
}

struct Lot<'a> {
    iri: &'a str,
    quantity: u64,
    value: u64,
}

fn landed_cost(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["entry_iri", "currency", "project_iri", "occurred_at_unix",
        "method", "cost_cents", "lots", "inventory_account", "payable_account"])?;
    let iri = text(required(args, "entry_iri")?)?;
    let project = text(required(args, "project_iri")?)?;
    let inventory = account(required(args, "inventory_account")?)?;
    let payable = account(required(args, "payable_account")?)?;
    if inventory == payable {
        return Err("The inventory and payable accounts must be distinct".into());
    }
    let currency = text(required(args, "currency")?)?;
    if currency.len() != 3 || !currency.bytes().all(|byte| byte.is_ascii_uppercase()) {
        return Err("Currency requires a three-letter uppercase code".into());
    }
    let occurred = u32::try_from(unsigned(required(args, "occurred_at_unix")?)?)
        .map_err(|_| "Journal timestamp exceeds the native epoch representation")?;
    let by_value = match text(required(args, "method")?)? {
        "value" => true,
        "quantity" => false,
        _ => return Err("Select value or quantity as the allocation basis".into()),
    };
    let cost = unsigned(required(args, "cost_cents")?)?;
    if cost == 0 || cost > i64::MAX as u64 {
        return Err("Capitalized cost must be a positive amount in i64 minor units".into());
    }
    let inputs = list(required(args, "lots")?)?;
    if inputs.is_empty() || inputs.len() > 4096 {
        return Err("Landed cost requires 1–4096 received lots in FIFO order".into());
    }
    host.charge(inputs.len() * 128, inputs.len() * 1024)?;
    let mut lots = Vec::with_capacity(inputs.len());
    let mut used = BTreeSet::new();
    for row in inputs {
        let row = record(row)?;
        fields(row, &["iri", "quantity", "value_cents"])?;
        let lot = Lot {
            iri: text(required(row, "iri")?)?,
            quantity: unsigned(required(row, "quantity")?)?,
            value: unsigned(required(row, "value_cents")?)?,
        };
        if lot.iri.len() > 4096 || !used.insert(lot.iri) || lot.quantity == 0 || lot.value > i64::MAX as u64 {
            return Err("Each received lot requires a distinct IRI, a positive quantity and bounded value".into());
        }
        lots.push(lot);
    }
    let weight = |lot: &Lot<'_>| if by_value { lot.value } else { lot.quantity };
    let total = lots.iter().map(|lot| i128::from(weight(lot))).sum::<i128>();
    if total <= 0 || total > i128::from(i64::MAX) {
        return Err("Allocation basis requires a positive bounded total".into());
    }
    let mut graph = ReceiptGraph::new(GRAPH);
    graph.fact(iri, "urn:qualia:cooperative:project", project)?;
    graph.fact(iri, "urn:qualia:cooperative:currency", &format!("urn:iso:std:iso:4217:{currency}"))?;
    let mut allocations = Vec::with_capacity(lots.len());
    let mut lines = Vec::with_capacity(lots.len() + 1);
    let mut cumulative = 0i128;
    let mut previous = 0i64;
    for lot in lots {
        cumulative += i128::from(weight(&lot));
        let rounded = div_round_bankers(i128::from(cost) * cumulative, total);
        let adjustment = rounded - previous;
        previous = rounded;
        let capitalized = lot.value.checked_add(adjustment as u64)
            .filter(|value| *value <= i64::MAX as u64).ok_or("A capitalized lot exceeds i64 minor units")?;
        if adjustment > 0 {
            let mut line = JournalLine::debit(inventory, adjustment);
            line.dimension_project_id = Some(project.into());
            lines.push(line);
        }
        graph.fact(iri, "urn:qualia:cooperative:capitalizedLot", lot.iri)?;
        graph.fact(lot.iri, "urn:qualia:cooperative:landedCostMinor", &format!("urn:qualia:money:minor:{adjustment}"))?;
        allocations.push(Value::Record(BTreeMap::from([
            ("lot_iri".into(), Value::String(lot.iri.into())),
            ("allocated_cents".into(), Value::I64(adjustment)),
            ("capitalized_value_cents".into(), Value::U64(capitalized)),
        ])));
    }
    if previous != cost as i64 {
        return Err("Rounded landed costs fail minor-unit conservation".into());
    }
    lines.push(JournalLine::credit(payable, previous));
    let journal = JournalEntryRecord {
        id: iri.into(), book: LedgerBook::ProjectActual, currency: currency.into(),
        memo: "Landed cost capitalization".into(), lines,
        occurred_at_unix: occurred, occurred_at_instant: None,
        project_id: Some(project.into()), reversal_of_id: None,
    };
    journal.validate_balance()?;
    graph.fact(iri, "urn:qualia:cooperative:debitMinor", &format!("urn:qualia:money:minor:{}", journal.total_debits()))?;
    graph.fact(iri, "urn:qualia:cooperative:creditMinor", &format!("urn:qualia:money:minor:{}", journal.total_credits()))?;
    let receipt = graph.seal(host.limits.resource_bytes.min(8 * 1024 * 1024))?;
    let draft = serde_json::to_vec(&journal).map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("allocations".into(), Value::List(allocations)),
        ("journal_draft".into(), host.encode("json", draft)?),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn account(value: &Value) -> Result<&str, String> {
    let code = text(value)?;
    if code.is_empty() || code.len() > 64 || code.chars().any(|character| character.is_control()) {
        return Err("Invalid bounded journal account code".into());
    }
    Ok(code)
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected a nonnegative integer".into()),
        _ => Err("Expected an exact integer".into()),
    }
}

fn basis_points(args: &Args) -> Result<Value, String> {
    fields(args, &["amount_cents", "basis_points"])?;
    let signed = |value: &Value| match value {
        Value::I64(value) => Ok(*value),
        Value::U64(value) => i64::try_from(*value).map_err(|_| "Amount exceeds i64"),
        _ => Err("Expected exact integer amount or basis points"),
    };
    let amount = signed(required(args, "amount_cents")?)?;
    let basis_points = signed(required(args, "basis_points")?)?;
    let numerator = i128::from(amount) * i128::from(basis_points);
    if numerator < i128::from(i64::MIN) * 10_000 || numerator > i128::from(i64::MAX) * 10_000 {
        return Err("Basis-point result exceeds i64 minor units".into());
    }
    Ok(Value::I64(div_round_bankers(numerator, 10_000)))
}

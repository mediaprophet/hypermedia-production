//! Request-scoped graph logit adjustment and KV dictionary codecs.

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{inference::{kv_dict::KvDictionary, qualia_hybrid::GRAPH_LOGIT_BIAS},
    q42_volume::WasmSliceQ42Reader, q_hash, NQuin};

use super::{external_resource, modalities::{admitted_iri, input_iri, Model, MAX_FACTS, READ_BYTES},
    ReceiptGraph, SYMBOLIC_LIMIT};
use crate::{fields, integer, list, num, record, required, Args, AssetHost, Value};

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, values: &Args) -> Result<Value, String> {
    if op == "hybrid_graph_logit_bias" { bias(host, values) } else { dictionary(host, values) }
}

fn bias(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["model", "context_iri", "subject_iri", "predicate_iri", "logits", "candidates"])?;
    let handle = required(args, "model")?;
    let slots = list(required(args, "logits")?)?;
    let candidates = list(required(args, "candidates")?)?;
    if slots.is_empty() || slots.len() > 262_144 || candidates.is_empty() || candidates.len() > MAX_FACTS {
        return Err("Graph logit adjustment exceeds its vocabulary or candidate bound".into());
    }
    let source = external_resource(host, handle)?;
    if source.kind != "q42" || source.bytes.len() > SYMBOLIC_LIMIT.saturating_sub(READ_BYTES + slots.len() * 64) {
        return Err("Graph logit adjustment requires a bounded Q42 volume".into());
    }
    let work = source.bytes.len().saturating_add(slots.len() * 4 + candidates.len() * MAX_FACTS);
    host.charge(work, READ_BYTES + slots.len() * 64)?;
    let source = external_resource(host, handle)?;
    let context = input_iri(args, "context_iri")?;
    let reader = WasmSliceQ42Reader::open(&source.bytes).map_err(|error| error.to_string())?;
    let mut model = Model::new(reader)?;
    let s = q_hash(admitted_iri(args, "subject_iri", &mut model)?);
    let p = q_hash(admitted_iri(args, "predicate_iri", &mut model)?);
    let mut facts = [NQuin::default(); MAX_FACTS + 1];
    let rows = model.context(context, &mut facts)?;
    let mut logits = slots.iter().map(finite_f32).collect::<Result<Vec<_>, _>>()?;
    let mut tokens = BTreeSet::new();
    let mut boosted = Vec::new();
    let mut receipt = ReceiptGraph::new("urn:qualia:inference:graphLogitBias");
    let source_iri = input_iri(args, "subject_iri")?;
    receipt.fact(source_iri, "urn:qualia:inference:consideredContext", context)?;
    for row in candidates {
        let row = record(row)?;
        fields(row, &["object_iri", "token_id"])?;
        let object = admitted_iri(row, "object_iri", &mut model)?;
        let id = integer(required(row, "token_id")?, "token_id")?;
        if id >= logits.len() || !tokens.insert(id) { return Err("Invalid or duplicate candidate token".into()); }
        if facts[..rows].iter().any(|q| q.subject == s && q.predicate == p && q.object == q_hash(object)) {
            logits[id] += GRAPH_LOGIT_BIAS;
            boosted.push(Value::U64(id as u64));
            receipt.fact(source_iri, "urn:qualia:inference:groundedCandidate", object)?;
        }
    }
    if logits.iter().any(|logit| !logit.is_finite()) { return Err("Graph logit adjustment overflowed".into()); }
    drop(model);
    let receipt = receipt.seal(host.limits.resource_bytes.min(4 * 1024 * 1024))?;
    Ok(Value::Record(BTreeMap::from([
        ("logits".into(), floats(logits)), ("boosted_token_ids".into(), Value::List(boosted)),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn dictionary(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["atoms", "vector", "sparsity"])?;
    let atoms = list(required(args, "atoms")?)?;
    let vector = list(required(args, "vector")?)?;
    let sparsity = integer(required(args, "sparsity")?, "sparsity")?;
    if !(1..=1024).contains(&vector.len()) || !(1..=256).contains(&atoms.len())
        || !(1..=atoms.len().min(16)).contains(&sparsity)
    {
        return Err("KV dictionary shape exceeds supported bounds".into());
    }
    let dim = vector.len();
    let n_atoms = atoms.len();
    host.charge(dim * n_atoms * sparsity + dim * sparsity.pow(3), (dim * n_atoms + dim * 4) * 4)?;
    let mut weights = Vec::with_capacity(dim * n_atoms);
    for atom in atoms {
        let atom = list(atom)?;
        if atom.len() != dim { return Err("KV dictionary atoms must match the vector dimension".into()); }
        for value in atom { weights.push(finite_f32(value)?); }
    }
    if weights.iter().any(|value| value.abs() > 1e14) { return Err("KV atom exceeds stable numeric range".into()); }
    let mut dictionary = KvDictionary { dim, n_atoms, atoms: weights, sparsity };
    if !dictionary.normalize_atoms() { return Err("KV dictionary contains a degenerate atom".into()); }
    let vector = vector.iter().map(finite_f32).collect::<Result<Vec<_>, _>>()?;
    if vector.iter().any(|value| value.abs() > 1e14) { return Err("KV vector exceeds stable numeric range".into()); }
    let code = dictionary.encode(&vector, sparsity);
    let reconstructed = dictionary.reconstruct(&code);
    if code.coeffs.iter().chain(&reconstructed).any(|value| !value.is_finite()) {
        return Err("KV dictionary produced a non-finite result".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("indices".into(), Value::List(code.indices.into_iter().map(|i| Value::U64(u64::from(i))).collect())),
        ("coefficients".into(), floats(code.coeffs)), ("reconstructed".into(), floats(reconstructed)),
    ])))
}

fn finite_f32(value: &Value) -> Result<f32, String> {
    let value = num(value)? as f32;
    if value.is_finite() { Ok(value) } else { Err("Input does not fit finite f32".into()) }
}

fn floats(values: Vec<f32>) -> Value {
    Value::List(values.into_iter().map(|value| Value::F64(f64::from(value))).collect())
}

//! Request-scoped semantic instrument and cooperative evaluation adapters.

use super::{fields, record, required, text, Args, AssetHost, Resource, Value};
use qualia_core_db::NQuin;
use std::collections::BTreeMap;

mod instruments;
mod cooperative;
mod personal_health;
mod sealer;
mod shared_services;

const SYMBOLIC_LIMIT: usize = 42 * 1024 * 1024;
const INPUT_BYTES: usize = 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn dispatch_qdnf_ilp_ebpf_and_os_services(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        let (op, parameters) = operation(args)?;
        shared_services::evaluate(self, op, parameters)
    }

    pub(crate) fn evaluate_personal_health_and_guardianship(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        let (op, parameters) = operation(args)?;
        personal_health::evaluate(self, op, parameters)
    }

    pub(crate) fn evaluate_semantic_instruments_and_tk_bc(
        &mut self, args: &Args,
    ) -> Result<Value, String> {
        let (op, parameters) = operation(args)?;
        instruments::evaluate(self, op, parameters)
    }

    pub(crate) fn evaluate_nine_stage_lifecycle_and_abac(
        &mut self, args: &Args,
    ) -> Result<Value, String> {
        let (op, parameters) = operation(args)?;
        cooperative::evaluate(self, op, parameters)
    }

    pub(crate) fn seal_cardiac_m64_emf_and_work_graph(
        &mut self, args: &Args,
    ) -> Result<Value, String> {
        let (op, parameters) = operation(args)?;
        sealer::evaluate(self, op, parameters)
    }
}

fn operation(args: &Args) -> Result<(&str, &Args), String> {
    fields(args, &["op", "parameters"])?;
    Ok((text(required(args, "op")?)?, record(required(args, "parameters")?)?))
}

fn resource<'a>(host: &'a AssetHost<'_>, value: &Value, kind: &str) -> Result<&'a Resource, String> {
    let resource = match value {
        Value::String(key) => host.inputs.iter().find(|row| &row.key == key)
            .ok_or_else(|| format!("Unknown retained resource {key}"))?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != kind || resource.bytes.len() > host.limits.resource_bytes {
        return Err(format!("Expected a {kind} resource within the production byte budget"));
    }
    Ok(resource)
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected unsigned integer".into()),
        _ => Err("Expected unsigned integer".into()),
    }
}

fn timestamp(params: &Args) -> Result<u32, String> {
    u32::try_from(unsigned(required(params, "now_unix")?)?)
        .map_err(|_| "Instrument time does not fit a u32 Unix timestamp".into())
}

fn json(value: &Value, bytes: &mut usize, depth: usize) -> Result<serde_json::Value, String> {
    use serde_json::{Number, Value as Json};
    fn charge(bytes: &mut usize, required: usize) -> Result<(), String> {
        *bytes = bytes.checked_add(required).filter(|&total| total <= INPUT_BYTES)
            .ok_or("Semantic instrument input exceeds byte budget")?;
        Ok(())
    }
    if depth > 32 {
        return Err("Semantic instrument input exceeds nesting budget".into());
    }
    charge(bytes, 16)?;
    Ok(match value {
        Value::Null => Json::Null,
        Value::Bool(value) => Json::Bool(*value),
        Value::U64(value) => Json::Number(Number::from(*value)),
        Value::I64(value) => Json::Number(Number::from(*value)),
        Value::F64(value) => Json::Number(Number::from_f64(*value).ok_or("Expected finite number")?),
        Value::String(value) | Value::Iri(value) => {
            charge(bytes, value.len())?;
            Json::String(value.clone())
        }
        Value::List(values) => Json::Array(values.iter().map(|value| json(value, bytes, depth + 1))
            .collect::<Result<_, _>>()?),
        Value::Record(values) => Json::Object(values.iter().map(|(key, value)| {
            charge(bytes, key.len())?;
            Ok((key.clone(), json(value, bytes, depth + 1)?))
        }).collect::<Result<_, String>>()?),
        _ => return Err("Unsupported value in semantic instrument input".into()),
    })
}

fn from_json(value: serde_json::Value) -> Value {
    use serde_json::Value as Json;
    match value {
        Json::Null => Value::Null,
        Json::Bool(value) => Value::Bool(value),
        Json::Number(value) => value.as_u64().map(Value::U64)
            .or_else(|| value.as_i64().map(Value::I64))
            .unwrap_or_else(|| Value::F64(value.as_f64().expect("JSON number is representable"))),
        Json::String(value) => Value::String(value),
        Json::Array(values) => Value::List(values.into_iter().map(from_json).collect()),
        Json::Object(values) => Value::Record(values.into_iter().map(|(key, value)|
            (key, from_json(value))).collect::<BTreeMap<_, _>>()),
    }
}

fn quin_value(row: &NQuin) -> Value {
    Value::List([row.subject, row.predicate, row.object, row.context, row.metadata, row.parity]
        .into_iter().map(Value::U64).collect())
}

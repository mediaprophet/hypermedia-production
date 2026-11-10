//! Species spec and rule-pack argument resolution for anatomy compilers.
//!
//! A species spec uses [`SCAFFOLD_SCHEMA_V1`] and is supplied either as a VibeScript record or as
//! the key of a workspace CBOR-LD input (for example a payload taken from a `.q42`/`.hmc`
//! container). Both decode to the same data model. JSON text is rejected.
//!
//! Rules are either a VibeScript list of rule records or the key of a workspace `.q42` input.
//! Without rules, the archetype-class default rules are used. Rules derived from the spec's
//! `joints` ranges are appended to explicit or default rules.

use super::rule_pack::{
    archetype_default_rules, compile_rule_pack_q42, parse_rules_value, validate_q42_logic,
    Q42LogicSummary, RuleSpec, DOMAIN_FAUNA, DOMAIN_FLORA, FAUNA_CONTEXT_DID, FLORA_CONTEXT_DID,
};
use crate::{Args, Resource};
use serde_json::Value as JsonValue;
use vibe::Value;

pub const SCAFFOLD_SCHEMA_V1: &str = "qualia.portable-anatomy-scaffold/v1";
pub const MAX_CBOR_SPEC_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecSource {
    VibeScript,
    CborLd,
}

impl SpecSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::VibeScript => "vibescript",
            Self::CborLd => "cbor-ld",
        }
    }
}

fn decode_cbor_ld(key: &str, bytes: &[u8]) -> Result<JsonValue, String> {
    if bytes.len() > MAX_CBOR_SPEC_BYTES {
        return Err(format!("CBOR-LD spec '{key}' exceeds {MAX_CBOR_SPEC_BYTES} bytes"));
    }
    let v: JsonValue = ciborium::de::from_reader(bytes)
        .map_err(|e| format!("Spec input '{key}' is not valid CBOR-LD: {e}"))?;
    if !v.is_object() {
        return Err(format!("CBOR-LD spec '{key}' must decode to a map"));
    }
    Ok(v)
}

/// Returns the species spec from `spec` (or its legacy alias `scaffold`) and where it came from.
pub fn resolve_species_spec(
    inputs: &[Resource],
    a: &Args,
) -> Result<Option<(JsonValue, SpecSource)>, String> {
    let v = match (a.get("spec"), a.get("scaffold")) {
        (Some(_), Some(_)) => return Err("Pass either 'spec' or 'scaffold', not both".into()),
        (Some(v), None) | (None, Some(v)) => v,
        (None, None) => return Ok(None),
    };
    let (json, source) = match v {
        Value::Record(_) => (vibe_value_to_json(v), SpecSource::VibeScript),
        Value::String(key) => match inputs.iter().find(|r| r.key == *key) {
            Some(res) => (decode_cbor_ld(key, &res.bytes)?, SpecSource::CborLd),
            None => {
                return Err(
                    "Species spec must be a VibeScript record or the key of a CBOR-LD input; JSON text is not accepted"
                        .into(),
                )
            }
        },
        _ => {
            return Err(
                "Species spec must be a VibeScript record or the key of a CBOR-LD input".into(),
            )
        }
    };
    match json.get("schema").and_then(|s| s.as_str()) {
        Some(SCAFFOLD_SCHEMA_V1) => Ok(Some((json, source))),
        Some(other) => Err(format!(
            "Unsupported species spec schema '{other}' (expected {SCAFFOLD_SCHEMA_V1})"
        )),
        None => Err(format!("Species spec requires schema: \"{SCAFFOLD_SCHEMA_V1}\"")),
    }
}

/// The model id for rule packs: the spec's `species_id`, otherwise `fallback`.
pub fn species_model_id(spec: Option<&JsonValue>, fallback: &str) -> String {
    spec.and_then(|s| s.get("species_id"))
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(fallback)
        .to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RulesSource {
    Spec,
    Input,
    Default,
}

impl RulesSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Spec => "spec",
            Self::Input => "input",
            Self::Default => "default",
        }
    }
}

pub struct ResolvedRules {
    pub bytes: Vec<u8>,
    pub summary: Q42LogicSummary,
    pub source: RulesSource,
}

/// Resolves the `rules` argument into a validated `Q42LOGIC` container, appending `derived`
/// rules (for example from joint ranges) to explicit or default rules.
pub fn resolve_rules(
    inputs: &[Resource],
    a: &Args,
    model_id: &str,
    is_flora: bool,
    derived: &[RuleSpec],
) -> Result<ResolvedRules, String> {
    let (domain, ctx) = if is_flora {
        (DOMAIN_FLORA, FLORA_CONTEXT_DID)
    } else {
        (DOMAIN_FAUNA, FAUNA_CONTEXT_DID)
    };
    let (bytes, source) = match a.get("rules") {
        None => {
            let mut rules = archetype_default_rules(is_flora);
            rules.extend_from_slice(derived);
            (
                compile_rule_pack_q42(model_id, domain, ctx, &rules)?,
                RulesSource::Default,
            )
        }
        Some(v @ Value::List(_)) => {
            let mut rules = parse_rules_value(v)?;
            rules.extend_from_slice(derived);
            (
                compile_rule_pack_q42(model_id, domain, ctx, &rules)?,
                RulesSource::Spec,
            )
        }
        Some(Value::String(key)) => {
            if !derived.is_empty() {
                return Err(
                    "A prebuilt .q42 rules input cannot be combined with spec joint ranges; pass rules as a list"
                        .into(),
                );
            }
            let res = inputs
                .iter()
                .find(|r| r.key == *key)
                .ok_or_else(|| format!("Rules input '{key}' not found in workspace inputs"))?;
            (res.bytes.clone(), RulesSource::Input)
        }
        Some(_) => {
            return Err("rules must be a list of rule records or a .q42 input key".into());
        }
    };
    let summary = validate_q42_logic(&bytes)?;
    Ok(ResolvedRules {
        bytes,
        summary,
        source,
    })
}

pub fn vibe_value_to_json(v: &Value) -> JsonValue {
    match v {
        Value::Bool(b) => JsonValue::Bool(*b),
        Value::I64(n) => JsonValue::Number((*n).into()),
        Value::U64(n) => JsonValue::Number((*n).into()),
        Value::F64(f) => serde_json::Number::from_f64(*f)
            .map(JsonValue::Number)
            .unwrap_or(JsonValue::Null),
        Value::String(s) => JsonValue::String(s.clone()),
        Value::List(items) => JsonValue::Array(items.iter().map(vibe_value_to_json).collect()),
        Value::Record(map) => {
            let mut obj = serde_json::Map::new();
            for (k, val) in map {
                obj.insert(k.clone(), vibe_value_to_json(val));
            }
            JsonValue::Object(obj)
        }
        _ => JsonValue::Null,
    }
}

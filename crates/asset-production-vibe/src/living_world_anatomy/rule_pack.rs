//! Data-driven `.q42` rule packs for species specs.
//!
//! A rule pack is authored as a VibeScript list of rule records and compiled into the existing
//! `Q42LOGIC` container (64-byte header + parity-checked 48-byte `NQuin` records) produced by
//! [`super::behavioral_hooks::compile_model_logic_q42_bytes`]. Species rules therefore live in
//! `.vibe`/`.q42` data instead of Rust tables.
//!
//! Rule record fields:
//! - `kind`: `"permit"` (0x11), `"forbid"` (0x12), `"obligate"` (0x10) or `"ltl_globally"` (0x40)
//! - `property`: property IRI, for example `"q42:safeCervicalNeckPitchRad"`
//! - `value`: finite, non-negative number (stored as `xsd:decimal` x 10^6)
//! - `defeater` (optional): sets the predicate defeater bit; defaults to `true` for `forbid`
//! - `lane` (optional): `"passthrough"`, `"commons"`, `"bilateral"`, `"spatial"` or 0..=3
//! - `weight` (optional): u32 metadata payload

use super::behavioral_hooks::{fnv1a_60, push_nquin_48};
use std::collections::BTreeMap;
use vibe::Value;

pub const Q42_LOGIC_MAGIC: &[u8; 8] = b"Q42LOGIC";
pub const Q42_LOGIC_HEADER_LEN: usize = 64;
pub const NQUIN_LEN: usize = 48;
pub const MAX_RULES: usize = 1024;
pub const DOMAIN_FAUNA: u32 = 1;
pub const DOMAIN_FLORA: u32 = 2;
pub const FAUNA_CONTEXT_DID: &str = "did:q42:world-of-god:fauna-neuromotor-envelope";
pub const FLORA_CONTEXT_DID: &str = "did:q42:world-of-god:flora-mycelium-neural-network";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleKind {
    Permit,
    Forbid,
    Obligate,
    LtlGlobally,
}

impl RuleKind {
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim().to_ascii_lowercase().replace('-', "_").as_str() {
            "permit" => Ok(Self::Permit),
            "forbid" => Ok(Self::Forbid),
            "obligate" => Ok(Self::Obligate),
            "ltl_globally" | "globally" => Ok(Self::LtlGlobally),
            other => Err(format!("Unknown rule kind '{other}'")),
        }
    }

    pub fn opcode(self) -> u8 {
        match self {
            Self::Obligate => 0x10,
            Self::Permit => 0x11,
            Self::Forbid => 0x12,
            Self::LtlGlobally => 0x40,
        }
    }

    fn header_slot(self) -> usize {
        match self {
            Self::Permit => 0,
            Self::Forbid => 1,
            Self::Obligate => 2,
            Self::LtlGlobally => 3,
        }
    }

    fn defaults(self) -> (bool, u64, u32) {
        match self {
            Self::Permit => (false, 0b11, 1000),
            Self::Forbid => (true, 0b11, 2000),
            Self::Obligate => (false, 0b01, 1500),
            Self::LtlGlobally => (false, 0b11, 4096),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RuleSpec {
    pub kind: RuleKind,
    pub property: String,
    pub value: f64,
    /// `measured`, `described`, `derived`, `model_assumption`, or `artistic_choice`.
    /// A rule with no assertion is a model assumption.
    pub basis: String,
    pub assertion_ids: Vec<String>,
    pub defeater: bool,
    pub lane: u64,
    pub weight: u32,
}

impl RuleSpec {
    pub fn new(kind: RuleKind, property: &str, value: f64) -> Self {
        let (defeater, lane, weight) = kind.defaults();
        Self {
            kind,
            property: property.to_string(),
            value,
            basis: "model_assumption".to_string(),
            assertion_ids: Vec::new(),
            defeater,
            lane,
            weight,
        }
    }
}

pub fn parse_claim_basis(s: &str) -> Result<&'static str, String> {
    match s.trim().to_ascii_lowercase().replace('-', "_").as_str() {
        "measured" => Ok("measured"),
        "described" => Ok("described"),
        "derived" => Ok("derived"),
        "model_assumption" => Ok("model_assumption"),
        "artistic_choice" => Ok("artistic_choice"),
        other => Err(format!("unknown claim basis '{other}'")),
    }
}

fn basis_needs_assertion(basis: &str) -> bool {
    matches!(basis, "measured" | "described" | "derived")
}

pub fn validate_rule_citation(rule: &RuleSpec) -> Result<(), String> {
    if basis_needs_assertion(&rule.basis)
        && !rule.assertion_ids.iter().any(|id| !id.trim().is_empty())
    {
        return Err(format!(
            "rule '{}' is {} and needs an assertion id",
            rule.property, rule.basis
        ));
    }
    if rule.basis == "unknown" {
        return Err(format!("rule '{}' cannot be an unknown value", rule.property));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Q42LogicSummary {
    pub quin_count: u32,
    pub domain_code: u32,
    pub kind_counts: [u32; 4],
    pub model_hash: u64,
    pub payload_digest: u64,
}

fn number(v: &Value, ctx: &str) -> Result<f64, String> {
    match v {
        Value::I64(n) => Ok(*n as f64),
        Value::U64(n) => Ok(*n as f64),
        Value::F64(f) => Ok(*f),
        _ => Err(format!("{ctx} must be a number")),
    }
}

fn parse_lane(v: &Value, ctx: &str) -> Result<u64, String> {
    match v {
        Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "passthrough" => Ok(0b00),
            "commons" => Ok(0b01),
            "bilateral" => Ok(0b10),
            "spatial" => Ok(0b11),
            other => Err(format!("{ctx}: unknown lane '{other}'")),
        },
        Value::I64(n) if (0..=3).contains(n) => Ok(*n as u64),
        Value::U64(n) if *n <= 3 => Ok(*n),
        _ => Err(format!("{ctx}: lane must be a lane name or 0..=3")),
    }
}

fn parse_rule(idx: usize, rec: &BTreeMap<String, Value>) -> Result<RuleSpec, String> {
    let ctx = format!("rules[{idx}]");
    for key in rec.keys() {
        if !matches!(
            key.as_str(),
            "kind" | "property" | "value" | "defeater" | "lane" | "weight" | "basis" | "assertion_ids"
        ) {
            return Err(format!("{ctx}: unknown field '{key}'"));
        }
    }
    let kind = match rec.get("kind") {
        Some(Value::String(s)) => RuleKind::parse(s).map_err(|e| format!("{ctx}: {e}"))?,
        _ => return Err(format!("{ctx}: 'kind' string is required")),
    };
    let property = match rec.get("property") {
        Some(Value::String(s)) if !s.trim().is_empty() => s.trim(),
        _ => return Err(format!("{ctx}: 'property' string is required")),
    };
    let value = number(
        rec.get("value")
            .ok_or_else(|| format!("{ctx}: 'value' is required"))?,
        &format!("{ctx}.value"),
    )?;
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{ctx}: value must be finite and non-negative"));
    }
    let mut rule = RuleSpec::new(kind, property, value);
    if let Some(v) = rec.get("defeater") {
        rule.defeater = match v {
            Value::Bool(b) => *b,
            _ => return Err(format!("{ctx}: 'defeater' must be a boolean")),
        };
    }
    if let Some(v) = rec.get("lane") {
        rule.lane = parse_lane(v, &ctx)?;
    }
    if let Some(v) = rec.get("weight") {
        let w = number(v, &format!("{ctx}.weight"))?;
        if w.fract() != 0.0 || !(0.0..=u32::MAX as f64).contains(&w) {
            return Err(format!("{ctx}: weight must be an integer in u32 range"));
        }
        rule.weight = w as u32;
    }
    if let Some(v) = rec.get("basis") {
        let Value::String(s) = v else {
            return Err(format!("{ctx}: 'basis' must be a string"));
        };
        rule.basis = parse_claim_basis(s).map_err(|e| format!("{ctx}: {e}"))?.to_string();
    }
    if let Some(v) = rec.get("assertion_ids") {
        rule.assertion_ids = parse_id_list(v, &format!("{ctx}.assertion_ids"))?;
    }
    validate_rule_citation(&rule).map_err(|e| format!("{ctx}: {e}"))?;
    Ok(rule)
}

fn parse_id_list(v: &Value, ctx: &str) -> Result<Vec<String>, String> {
    match v {
        Value::String(s) if !s.trim().is_empty() => Ok(vec![s.trim().to_string()]),
        Value::List(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (i, item) in items.iter().enumerate() {
                match item {
                    Value::String(s) if !s.trim().is_empty() => out.push(s.trim().to_string()),
                    _ => return Err(format!("{ctx}[{i}] must be a non-empty string")),
                }
            }
            Ok(out)
        }
        _ => Err(format!("{ctx} must be a string or a list of strings")),
    }
}

/// Parses a VibeScript list of rule records.
pub fn parse_rules_value(v: &Value) -> Result<Vec<RuleSpec>, String> {
    let items = match v {
        Value::List(items) => items,
        _ => return Err("rules must be a VibeScript list of rule records".into()),
    };
    if items.is_empty() {
        return Err("rules list is empty".into());
    }
    if items.len() > MAX_RULES {
        return Err(format!("rules list exceeds {MAX_RULES} entries"));
    }
    items
        .iter()
        .enumerate()
        .map(|(i, item)| match item {
            Value::Record(rec) => parse_rule(i, rec),
            _ => Err(format!("rules[{i}] must be a record")),
        })
        .collect()
}

/// The archetype-class default rules, identical to the legacy
/// [`super::behavioral_hooks::compile_model_logic_q42_bytes`] tables.
pub fn archetype_default_rules(flora: bool) -> Vec<RuleSpec> {
    use RuleKind::*;
    let table: [(RuleKind, &str, f64); 16] = if flora {
        [
            (Permit, "q42:safeStemElasticDeflectionRad", 0.38),
            (Permit, "q42:safeModulusOfRuptureMPa", 65.0),
            (Permit, "q42:safeFoliarBrowseFraction01", 0.22),
            (Permit, "q42:safeXylemWaterPotentialMPa", 1.80),
            (Forbid, "q42:injuryThresholdHerbivoryBrowse01", 0.25),
            (Forbid, "q42:injuryThresholdCambiumBarkStrip01", 0.15),
            (Forbid, "q42:injuryThresholdBranchCleavageMPa", 65.0),
            (Forbid, "q42:injuryThresholdXylemCavitationMPa", 2.40),
            (Obligate, "q42:obligateMycelialJasmonateSpikeHz", 1.80),
            (Obligate, "q42:obligateStomatalEmergencyClosure01", 0.08),
            (Obligate, "q42:obligateCambiumKinoResinCallus01", 0.95),
            (Obligate, "q42:obligateMycorrhizalCarbonDumpMgHr", 18.4),
            (LtlGlobally, "q42:ltlGloballyEnforceJointConeLimits", 60.0),
            (LtlGlobally, "q42:ltlGloballyClampSmallLlmHooksToSafeRom", 4.0),
            (LtlGlobally, "q42:ltlUntilWoundHealedMaintainGuarding", 1.0),
            (LtlGlobally, "q42:ltlGloballyVerifyNQuinXorParity", 1.0),
        ]
    } else {
        [
            (Permit, "q42:safeAnkleAchillesFlexionRad", 0.95),
            (Permit, "q42:safeCervicalNeckPitchRad", 0.75),
            (Permit, "q42:safeMandibularJawOpen01", 0.78),
            (Permit, "q42:safeForelimbReachRad", 1.15),
            (Forbid, "q42:injuryThresholdAchillesStrain01", 1.00),
            (Forbid, "q42:injuryThresholdForepawSprain01", 1.00),
            (Forbid, "q42:injuryThresholdCervicalHyperext01", 1.00),
            (Forbid, "q42:injuryThresholdCaudalTorque01", 1.00),
            (Obligate, "q42:obligateInjuredAsymmetricLimpGait", 0.35),
            (Obligate, "q42:obligateForepawProtectiveGuarding", 0.90),
            (Obligate, "q42:obligateNociceptiveEarPinBack", 0.85),
            (Obligate, "q42:obligateHerbivoryToMyceliumCoupling", 1.00),
            (LtlGlobally, "q42:ltlGloballyEnforceJointConeLimits", 60.0),
            (LtlGlobally, "q42:ltlGloballyClampSmallLlmHooksToSafeRom", 4.0),
            (LtlGlobally, "q42:ltlUntilWoundHealedMaintainGuarding", 1.0),
            (LtlGlobally, "q42:ltlGloballyVerifyNQuinXorParity", 1.0),
        ]
    };
    table
        .iter()
        .map(|(k, p, v)| RuleSpec::new(*k, p, *v))
        .collect()
}

/// Compiles rules into a `Q42LOGIC` container. Lamport clocks follow rule order, starting at 1.
pub fn compile_rule_pack_q42(
    model_id: &str,
    domain_code: u32,
    context_did: &str,
    rules: &[RuleSpec],
) -> Result<Vec<u8>, String> {
    if model_id.trim().is_empty() {
        return Err("rule pack model id is empty".into());
    }
    if rules.is_empty() || rules.len() > MAX_RULES {
        return Err(format!("rule pack must hold 1..={MAX_RULES} rules"));
    }
    let subj_did = format!("did:q42:model:{model_id}");
    let mut quins = Vec::with_capacity(rules.len() * NQUIN_LEN);
    let mut kind_counts = [0u32; 4];
    for (idx, r) in rules.iter().enumerate() {
        if !r.value.is_finite() || r.value < 0.0 || r.lane > 3 {
            return Err(format!("rule {idx} has an invalid value or lane"));
        }
        validate_rule_citation(r).map_err(|e| format!("rule {idx}: {e}"))?;
        kind_counts[r.kind.header_slot()] += 1;
        push_nquin_48(
            &mut quins,
            &subj_did,
            &r.property,
            r.kind.opcode(),
            r.defeater,
            r.value,
            context_did,
            r.lane,
            (idx as u64) + 1,
            r.weight,
        );
    }
    let mut out = Vec::with_capacity(Q42_LOGIC_HEADER_LEN + quins.len());
    out.extend_from_slice(Q42_LOGIC_MAGIC);
    out.extend_from_slice(&1u32.to_le_bytes());
    out.extend_from_slice(&(rules.len() as u32).to_le_bytes());
    out.extend_from_slice(&domain_code.to_le_bytes());
    for c in kind_counts {
        out.extend_from_slice(&c.to_le_bytes());
    }
    let model_hash = (1u64 << 63) | fnv1a_60(subj_did.as_bytes());
    out.extend_from_slice(&model_hash.to_le_bytes());
    out.extend_from_slice(&fnv1a_60(&quins).to_le_bytes());
    out.extend_from_slice(&[0u8; 12]);
    out.extend_from_slice(&quins);
    Ok(out)
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

/// Validates a `Q42LOGIC` container: magic, version, length, per-kind counts, payload digest and
/// per-quin XOR parity.
pub fn validate_q42_logic(bytes: &[u8]) -> Result<Q42LogicSummary, String> {
    if bytes.len() < Q42_LOGIC_HEADER_LEN {
        return Err("q42 rule pack shorter than its 64-byte header".into());
    }
    if &bytes[0..8] != Q42_LOGIC_MAGIC {
        return Err("q42 rule pack magic is not Q42LOGIC".into());
    }
    if u32_at(bytes, 8) != 1 {
        return Err("q42 rule pack version is not 1".into());
    }
    let quin_count = u32_at(bytes, 12);
    if quin_count == 0 || quin_count as usize > MAX_RULES {
        return Err(format!("q42 rule pack must hold 1..={MAX_RULES} quins"));
    }
    let expected = Q42_LOGIC_HEADER_LEN + quin_count as usize * NQUIN_LEN;
    if bytes.len() != expected {
        return Err(format!(
            "q42 rule pack length {} does not match header ({expected})",
            bytes.len()
        ));
    }
    let kind_counts = [
        u32_at(bytes, 20),
        u32_at(bytes, 24),
        u32_at(bytes, 28),
        u32_at(bytes, 32),
    ];
    if kind_counts.iter().map(|&c| c as u64).sum::<u64>() != quin_count as u64 {
        return Err("q42 rule pack kind counts do not sum to quin count".into());
    }
    let payload = &bytes[Q42_LOGIC_HEADER_LEN..];
    let payload_digest = u64_at(bytes, 44);
    if fnv1a_60(payload) != payload_digest {
        return Err("q42 rule pack payload digest mismatch".into());
    }
    for (i, q) in payload.chunks_exact(NQUIN_LEN).enumerate() {
        let parity = u64_at(q, 0) ^ u64_at(q, 8) ^ u64_at(q, 16) ^ u64_at(q, 24);
        if parity != u64_at(q, 40) {
            return Err(format!("q42 rule pack quin {i} parity mismatch"));
        }
    }
    Ok(Q42LogicSummary {
        quin_count,
        domain_code: u32_at(bytes, 16),
        kind_counts,
        model_hash: u64_at(bytes, 36),
        payload_digest,
    })
}

//! Turns a species corpus into production values.
//!
//! The corpus stays in qualiaDB. This module reads joint ranges out of it and cites the
//! assertion that supplied each range. A spec range with no assertion is a model assumption.
//! `qualia.species.assemble_corpus` seals sources and assertions into the `.q42` volume.

use super::joint_rom::{canonical_joint, AngleRange, Axis, AxisRom, JointRom};
use super::rule_pack::parse_claim_basis;
use crate::*;
use qualia_core_db::q42::species_corpus::{volume_sha256, SpeciesCorpus};
use qualia_core_db::q42_volume::Q42_MAGIC;
use serde_json::{Map, Value as JsonValue};
use std::collections::BTreeMap;
use vibe::Value;

const RADIAN: &str = "http://qudt.org/vocab/unit/RAD";

struct CitedRange {
    range: AngleRange,
    basis: String,
    assertion_id: String,
}

pub fn resolve_corpus(
    inputs: &[Resource],
    a: &Args,
    spec: Option<&JsonValue>,
) -> Result<Option<SpeciesCorpus>, String> {
    let spec_iri = spec.and_then(|s| s.get("corpus_iri")).and_then(|v| v.as_str());
    let spec_sha = spec.and_then(|s| s.get("corpus_sha256")).and_then(|v| v.as_str());
    let (loaded, volume_sha) = match a.get("corpus") {
        None => {
            if spec_iri.is_some() || spec_sha.is_some() {
                return Err(
                    "spec names a species corpus; pass it as the 'corpus' input".into(),
                );
            }
            return Ok(None);
        }
        Some(Value::String(key)) => {
            let res = inputs
                .iter()
                .find(|r| r.key == *key)
                .ok_or_else(|| format!("Species corpus input '{key}' was not found"))?;
            let sha = if res.bytes.starts_with(&Q42_MAGIC) {
                Some(volume_sha256(&res.bytes))
            } else {
                None
            };
            (load_corpus_bytes(&res.bytes)?, sha)
        }
        Some(Value::Record(rec)) => {
            let json = vibe_record_to_json(rec);
            (SpeciesCorpus::from_json_value(json)?, None)
        }
        Some(_) => {
            return Err(
                "corpus must be a species-corpus record or the key of a .q42 or CBOR-LD input"
                    .into(),
            )
        }
    };
    if let Some(iri) = spec_iri {
        if iri != loaded.corpus_iri {
            return Err(format!(
                "spec corpus_iri '{iri}' does not match corpus '{}'",
                loaded.corpus_iri
            ));
        }
    }
    if let Some(expected) = spec_sha {
        let Some(actual) = volume_sha else {
            return Err("corpus_sha256 checks the supplied .q42 volume bytes".into());
        };
        if !expected.eq_ignore_ascii_case(&actual) {
            return Err(format!(
                "spec corpus_sha256 does not match the corpus volume {actual}"
            ));
        }
    }
    Ok(Some(loaded))
}

fn load_corpus_bytes(bytes: &[u8]) -> Result<SpeciesCorpus, String> {
    if bytes.starts_with(&Q42_MAGIC) {
        SpeciesCorpus::from_q42_bytes(bytes)
    } else {
        SpeciesCorpus::from_cbor_ld(bytes)
    }
}

fn vibe_record_to_json(rec: &BTreeMap<String, Value>) -> JsonValue {
    let mut obj = Map::new();
    for (k, v) in rec {
        obj.insert(k.clone(), super::species_spec::vibe_value_to_json(v));
    }
    JsonValue::Object(obj)
}

fn is_radian(unit: &str) -> bool {
    let u = unit.trim();
    u.eq_ignore_ascii_case(RADIAN) || u.ends_with("/RAD") || u.ends_with("#RAD")
}

fn parse_rom_property(iri: &str) -> Option<(&'static str, Axis, &'static str)> {
    let rest = iri.strip_prefix("q42:rom:")?;
    let mut parts = rest.split(':');
    let joint = canonical_joint(parts.next()?)?;
    let axis = Axis::parse(parts.next().unwrap_or("")).ok()?;
    let zone = match parts.next()? {
        "healthy" => "healthy",
        "injury" => "injury",
        "injured" => "injured",
        _ => return None,
    };
    if parts.next().is_some() {
        return None;
    }
    Some((joint, axis, zone))
}

/// Joint ranges taken from active corpus assertions. Each range cites its assertion.
pub fn joint_rom_from_corpus(corpus: &SpeciesCorpus) -> Result<Vec<JointRom>, String> {
    let mut grouped: BTreeMap<(&'static str, Axis), [Option<CitedRange>; 3]> = BTreeMap::new();
    let mut gaits: BTreeMap<&'static str, (String, String, String)> = BTreeMap::new();
    let mut guarding: BTreeMap<&'static str, (f64, String, String)> = BTreeMap::new();

    for fact in corpus.facts() {
        if !fact.active {
            continue;
        }
        if let Some(joint) = fact
            .property_iri
            .strip_prefix("q42:rom:")
            .and_then(|rest| rest.strip_suffix(":injuredGait"))
            .and_then(canonical_joint)
        {
            let text = fact
                .text
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| format!("assertion '{}' injured gait has no text", fact.assertion_id))?;
            gaits.insert(
                joint,
                (text.to_string(), fact.basis.to_string(), fact.assertion_id.to_string()),
            );
            continue;
        }
        if let Some(joint) = fact
            .property_iri
            .strip_prefix("q42:rom:")
            .and_then(|rest| rest.strip_suffix(":injuredGuarding"))
            .and_then(canonical_joint)
        {
            let value = fact.min.ok_or_else(|| {
                format!("assertion '{}' injured guarding has no number", fact.assertion_id)
            })?;
            if !(0.0..=1.0).contains(&value) {
                return Err(format!(
                    "assertion '{}' injured guarding must be in 0.0..=1.0",
                    fact.assertion_id
                ));
            }
            guarding.insert(
                joint,
                (value, fact.basis.to_string(), fact.assertion_id.to_string()),
            );
            continue;
        }
        let Some((joint, axis, zone)) = parse_rom_property(fact.property_iri) else {
            continue;
        };
        let (min, max) = match (fact.min, fact.max) {
            (Some(min), Some(max)) => (min, max),
            _ => {
                return Err(format!(
                    "assertion '{}' range of motion has no numeric range",
                    fact.assertion_id
                ))
            }
        };
        if let Some(unit) = fact.unit_iri {
            if !is_radian(unit) {
                return Err(format!(
                    "assertion '{}' angle unit '{unit}' is not a radian",
                    fact.assertion_id
                ));
            }
        }
        if !min.is_finite() || !max.is_finite() || min > 0.0 || max < 0.0 {
            return Err(format!(
                "assertion '{}' range must contain the neutral angle 0",
                fact.assertion_id
            ));
        }
        let cited = CitedRange {
            range: AngleRange { min, max },
            basis: fact.basis.to_string(),
            assertion_id: fact.assertion_id.to_string(),
        };
        let slot = grouped.entry((joint, axis)).or_default();
        let idx = match zone {
            "healthy" => 0,
            "injury" => 1,
            _ => 2,
        };
        if slot[idx].is_some() {
            return Err(format!(
                "corpus has two active assertions for q42:rom:{joint}:{}:{zone}",
                axis.as_str()
            ));
        }
        slot[idx] = Some(cited);
    }

    let mut by_joint: BTreeMap<&'static str, JointRom> = BTreeMap::new();
    for ((joint, axis), slots) in grouped {
        let [healthy, injury, injured] = slots;
        let healthy = healthy.ok_or_else(|| {
            format!("corpus is missing q42:rom:{joint}:{}:healthy", axis.as_str())
        })?;
        let injury = injury.ok_or_else(|| {
            format!("corpus is missing q42:rom:{joint}:{}:injury", axis.as_str())
        })?;
        if !injury.range.encloses(&healthy.range) {
            return Err(format!(
                "q42:rom:{joint}:{} injury limit must enclose the healthy range",
                axis.as_str()
            ));
        }
        let injured = injured.map(|c| {
            if !healthy.range.encloses(&c.range) {
                Err(format!(
                    "q42:rom:{joint}:{} injured range must lie within the healthy range",
                    axis.as_str()
                ))
            } else {
                Ok(c)
            }
        }).transpose()?;
        let entry = by_joint.entry(joint).or_insert_with(|| JointRom {
            joint,
            axes: Vec::new(),
            injured_gait: None,
            injured_guarding: 0.0,
            gait_basis: "model_assumption".into(),
            gait_assertion_ids: Vec::new(),
        });
        entry.axes.push(AxisRom {
            axis,
            healthy: healthy.range,
            injury: injury.range,
            injured_healthy: injured.as_ref().map(|c| c.range),
            healthy_basis: healthy.basis,
            injury_basis: injury.basis,
            injured_basis: injured
                .as_ref()
                .map(|c| c.basis.clone())
                .unwrap_or_else(|| "model_assumption".into()),
            healthy_assertion_ids: vec![healthy.assertion_id],
            injury_assertion_ids: vec![injury.assertion_id],
            injured_assertion_ids: injured.into_iter().map(|c| c.assertion_id).collect(),
        });
    }
    for (joint, rom) in &mut by_joint {
        rom.axes.sort_by_key(|a| a.axis);
        if let Some((gait, basis, id)) = gaits.remove(joint) {
            let _ = parse_claim_basis(&basis).map_err(|e| format!("joint {joint}: {e}"))?;
            rom.injured_gait = Some(gait);
            rom.gait_basis = basis;
            rom.gait_assertion_ids = vec![id];
        }
        if let Some((g, _, _)) = guarding.remove(joint) {
            rom.injured_guarding = g;
        }
    }
    Ok(by_joint.into_values().collect())
}

impl AssetHost<'_> {
    /// `qualia.species.assemble_corpus`: validate a species document and seal it as `.q42`.
    pub(super) fn assemble_species_corpus(&mut self, a: &Args) -> Result<Value, String> {
        crate::fields(a, &["corpus"])?;
        let supplied_q42 = match a.get("corpus") {
            Some(Value::String(key)) => self
                .inputs
                .iter()
                .find(|r| r.key == *key)
                .filter(|r| r.bytes.starts_with(&Q42_MAGIC))
                .map(|r| r.bytes.clone()),
            _ => None,
        };
        let corpus = resolve_corpus(self.inputs, a, None)?
            .ok_or("corpus record or input key is required")?;
        let (q42_bytes, q42_sha256, cbor_bytes, conflict_count, conflicts) =
            if let Some(bytes) = supplied_q42 {
                let cbor = corpus.to_cbor_ld()?;
                let conflicts = corpus.conflicts();
                let summary = conflict_summary(&conflicts);
                (bytes.clone(), volume_sha256(&bytes), cbor, conflicts.len(), summary)
            } else {
                let sealed = corpus.seal()?;
                let summary = conflict_summary(&sealed.conflicts);
                (
                    sealed.q42,
                    sealed.q42_sha256,
                    sealed.cbor_ld,
                    sealed.conflicts.len(),
                    summary,
                )
            };
        self.charge(
            q42_bytes.len().saturating_add(cbor_bytes.len()),
            q42_bytes.len(),
        )?;
        let q42 = self.encode("q42", q42_bytes)?;
        let cbor = self.encode("cbor-ld", cbor_bytes)?;
        Ok(Value::Record(BTreeMap::from([
            ("corpus_iri".into(), Value::String(corpus.corpus_iri)),
            ("species_id".into(), Value::String(corpus.species_id)),
            ("revision".into(), Value::String(corpus.revision)),
            ("source_count".into(), Value::I64(corpus.sources.len() as i64)),
            ("assertion_count".into(), Value::I64(corpus.assertions.len() as i64)),
            ("gap_count".into(), Value::I64(corpus.gaps.len() as i64)),
            ("conflict_count".into(), Value::I64(conflict_count as i64)),
            ("conflicts".into(), Value::String(conflicts)),
            ("q42_sha256".into(), Value::String(q42_sha256)),
            ("corpus_q42".into(), q42),
            ("corpus_cbor".into(), cbor),
        ])))
    }
}

fn conflict_summary(conflicts: &[qualia_core_db::q42::species_corpus::CorpusConflict]) -> String {
    conflicts
        .iter()
        .map(|c| format!("{} [{}]", c.property_iri, c.assertion_ids.join(", ")))
        .collect::<Vec<_>>()
        .join("; ")
}

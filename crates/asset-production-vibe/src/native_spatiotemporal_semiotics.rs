//! Digest-bound historical metrology observations and Q42 provenance.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::{hash_map::Entry, BTreeSet, HashMap};

use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    lexicon::generate_60bit_token,
    modalities::spatiotemporal_linguistics::{
        evaluate_metrology_area_sq_m, evaluate_metrology_volume_liters, MetrologyUnitKind,
    },
    q42_volume::{UnifiedVolumeBuilder, WasmSliceQ42Reader},
    q_hash,
    resolver::typed_double,
    NQuin, QUINS_PER_BLOCK,
};
use sha2::{Digest, Sha256};

use crate::{
    fields, qualia_containers_governance_poet_ops::encoded_resource, required, text, vector, Args,
    AssetHost, Value,
};

const MAX_SOURCE: usize = 256 * 1024;
const MAX_OBSERVATIONS: usize = 256;
const MAX_TERM: usize = 2048;
const MAX_RECEIPT: usize = 4 * 1024 * 1024;
const TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const DERIVED_FROM: &str = "http://www.w3.org/ns/prov#wasDerivedFrom";
const QUDT_VALUE: &str = "http://qudt.org/schema/qudt/numericValue";
const QUDT_UNIT: &str = "http://qudt.org/schema/qudt/unit";
const SQUARE_METRE: &str = "http://qudt.org/vocab/unit/SQ-M";
const LITRE: &str = "http://qudt.org/vocab/unit/L";

macro_rules! term {
    ($name:literal) => {
        concat!("urn:qualia:linguistics:historical-metrology:", $name)
    };
}

/// Each archive row stores observation IRI, place IRI, archive locator, year,
/// historical unit, quantity, and optional surveyor temperature in degrees Celsius.
type Observation = (String, String, String, f64, String, f64, Option<f64>);

impl AssetHost<'_> {
    pub(crate) fn analyze_spatiotemporal_semiotics(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &["source", "source_sha256", "source_iri", "year_horizon"],
        )?;
        let source = required(args, "source")?;
        let expected_hash = text(required(args, "source_sha256")?)?;
        let source_iri = text(required(args, "source_iri")?)?;
        let [first_year, last_year] = vector(required(args, "year_horizon")?)?;
        if first_year > last_year || !valid_year(first_year) || !valid_year(last_year) {
            return Err(
                "Historical metrology year_horizon requires ordered years in [-10000, 3000]".into(),
            );
        }
        bounded_iri(source_iri)?;
        let n = {
            let resource = encoded_resource(self, source)?;
            if resource.kind != "json" || !(1..=MAX_SOURCE).contains(&resource.bytes.len()) {
                return Err(
                    "Historical metrology requires a retained JSON archive of at most 256 KiB"
                        .into(),
                );
            }
            resource.bytes.len()
        };
        self.charge(n.saturating_mul(12), MAX_RECEIPT + n.saturating_mul(8))?;
        let (digest, observations) = {
            let resource = encoded_resource(self, source)?;
            let digest = format!("{:x}", Sha256::digest(&resource.bytes));
            if expected_hash.len() != digest.len() || !expected_hash.eq_ignore_ascii_case(&digest) {
                return Err("Historical metrology archive SHA-256 mismatch".into());
            }
            let observations = serde_json::from_slice::<Vec<Observation>>(&resource.bytes)
                .map_err(|error| format!("Invalid historical observation archive: {error}"))?;
            if !(1..=MAX_OBSERVATIONS).contains(&observations.len()) {
                return Err("Historical archive requires 1–256 observation rows".into());
            }
            (digest, observations)
        };
        let digest_iri = format!("urn:sha256:{digest}");
        let context = format!("urn:qualia:linguistics:metrology:{digest}");
        let mut graph = HistoricalGraph::new(&context);
        graph.iri(source_iri, term!("sourceDigest"), &digest_iri)?;
        let mut seen = BTreeSet::new();
        let mut matched = 0;
        for (index, (observation, place, locator, year, unit, quantity, temperature)) in
            observations.iter().enumerate()
        {
            self.charge(0, 0)?;
            bounded_iri(observation)?;
            bounded_iri(place)?;
            if observation == source_iri
                || !seen.insert(observation)
                || locator.is_empty()
                || locator.len() > MAX_TERM
                || locator.chars().any(char::is_control)
                || !valid_year(*year)
                || !(0.0..=1e8).contains(quantity)
            {
                return Err(format!(
                    "Invalid historical observation at archive row {index}"
                ));
            }
            let (resolved, unit_iri) = convert(unit, *quantity, *temperature)
                .map_err(|error| format!("Historical observation {index}: {error}"))?;
            if !(first_year..=last_year).contains(year) {
                continue;
            }
            matched += 1;
            let result = format!("{context}:row:{index}");
            graph.iri(observation, TYPE, term!("ArchiveObservation"))?;
            graph.iri(observation, DERIVED_FROM, source_iri)?;
            graph.iri(observation, term!("place"), place)?;
            graph.text(observation, term!("archiveLocator"), locator)?;
            graph.text(observation, term!("historicalUnit"), unit)?;
            graph.number(observation, term!("year"), *year)?;
            graph.number(observation, term!("historicalQuantity"), *quantity)?;
            if let Some(temperature) = temperature {
                graph.number(
                    observation,
                    term!("surveyorTemperatureCelsius"),
                    *temperature,
                )?;
            }
            graph.iri(&result, TYPE, term!("ConvertedObservation"))?;
            graph.iri(&result, DERIVED_FROM, observation)?;
            graph.iri(&result, QUDT_UNIT, unit_iri)?;
            graph.number(&result, QUDT_VALUE, resolved)?;
        }
        if matched == 0 {
            return Err("No historical observations fall within the specified year horizon".into());
        }
        let q42 = graph.seal(self.limits.resource_bytes.min(MAX_RECEIPT))?;
        let receipt = self.encode("q42", q42)?;
        Ok(Value::Record(Args::from([
            ("drift_graph".into(), receipt),
            ("archive_sha256".into(), Value::String(digest)),
            ("context_iri".into(), Value::String(context)),
            ("matched_observations".into(), Value::U64(matched as u64)),
            (
                "excluded_observations".into(),
                Value::U64((observations.len() - matched) as u64),
            ),
        ])))
    }
}

fn valid_year(year: f64) -> bool {
    (-10_000.0..=3000.0).contains(&year)
}

fn bounded_iri(iri: &str) -> Result<(), String> {
    if iri.len() > MAX_TERM {
        return Err("Historical observation IRI exceeds its term limit".into());
    }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    Ok(())
}

fn convert(
    unit: &str,
    quantity: f64,
    temperature: Option<f64>,
) -> Result<(f64, &'static str), String> {
    let (kind, area) = match unit {
        "acre_statute" => (MetrologyUnitKind::AcreStatute, true),
        "acre_irish_plantation" => (MetrologyUnitKind::AcreIrishPlantation, true),
        "acre_scottish" => (MetrologyUnitKind::AcreScottish, true),
        "gunters_chain_square_side" => (MetrologyUnitKind::GuntersChain, true),
        "square_perch" => (MetrologyUnitKind::Perch, true),
        "hoppus_ton" => (MetrologyUnitKind::HoppusTon, false),
        "bushel_winchester" => (MetrologyUnitKind::BushelWinchester, false),
        "bushel_imperial" => (MetrologyUnitKind::BushelImperial, false),
        _ => return Err("Unsupported historical unit".into()),
    };
    match (area, temperature) {
        (true, Some(temperature)) if (-60.0..=70.0).contains(&temperature) =>
            Ok((evaluate_metrology_area_sq_m(kind, quantity, temperature), SQUARE_METRE)),
        (false, None) => Ok((evaluate_metrology_volume_liters(kind, quantity), LITRE)),
        _ => Err("Supply temperature in [-60, 70] °C for area observations and null for commodity volume".into()),
    }
}

struct HistoricalGraph<'a> {
    context: &'a str,
    lexicon: HashMap<u64, String>,
    facts: Vec<NQuin>,
}

impl<'a> HistoricalGraph<'a> {
    fn new(context: &'a str) -> Self {
        Self {
            context,
            lexicon: HashMap::new(),
            facts: Vec::new(),
        }
    }

    fn iri(&mut self, s: &str, p: &str, o: &str) -> Result<(), String> {
        self.add(s, p, o, q_hash(o))
    }

    fn text(&mut self, s: &str, p: &str, literal: &str) -> Result<(), String> {
        self.add(s, p, literal, generate_60bit_token(literal.as_bytes()))
    }

    fn number(&mut self, s: &str, p: &str, value: f64) -> Result<(), String> {
        let literal = typed_double::from_f64(value)
            .ok_or("Historical observation requires a finite RDF double")?;
        self.add(s, p, &literal, typed_double::token(&literal))
    }

    fn add(&mut self, subject: &str, predicate: &str, object: &str, o: u64) -> Result<(), String> {
        let [s, p, c] = [subject, predicate, self.context].map(q_hash);
        for (hash, lexical) in [(s, subject), (p, predicate), (o, object), (c, self.context)] {
            if hash == 0 {
                return Err("Historical graph has a zero term hash".into());
            }
            match self.lexicon.entry(hash) {
                Entry::Vacant(slot) => {
                    slot.insert(lexical.into());
                }
                Entry::Occupied(slot) if slot.get() == lexical => {}
                Entry::Occupied(_) => {
                    return Err("Historical graph has conflicting lexical identities".into())
                }
            }
        }
        self.facts.push(NQuin {
            subject: s,
            predicate: p,
            object: o,
            context: c,
            metadata: 0,
            parity: NQuin::calculate_parity(s, p, o, c, 0),
        });
        Ok(())
    }

    fn seal(mut self, limit: usize) -> Result<Vec<u8>, String> {
        self.facts
            .sort_unstable_by_key(|q| (q.object, q.subject, q.predicate, q.context));
        let mut volume = UnifiedVolumeBuilder::with_lex_map(&self.lexicon)
            .map_err(|error| error.to_string())?
            .with_fixed_timestamp_ms(0);
        for (index, rows) in self.facts.chunks(QUINS_PER_BLOCK).enumerate() {
            volume
                .push_block(index as u64, rows)
                .map_err(|error| error.to_string())?;
        }
        let bytes = volume.finish_to_bytes();
        if bytes.len() > limit {
            return Err("Historical Q42 volume exceeded its receipt byte budget".into());
        }
        let admitted = WasmSliceQ42Reader::open(&bytes).map_err(|error| error.to_string())?;
        if !admitted.is_unified_volume() {
            return Err("Historical Q42 receipt failed unified-volume admission".into());
        }
        Ok(bytes)
    }
}

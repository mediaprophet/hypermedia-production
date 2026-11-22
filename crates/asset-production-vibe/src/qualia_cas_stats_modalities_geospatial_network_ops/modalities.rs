//! Cold, bounded asset adapters for the QualiaDB modality evaluators.

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{NQuin, q_hash};
use qualia_core_db::bundle::semantic_model::resource_identity_meta;
use qualia_core_db::modalities::{
    abductive, accrual, argumentation, blackboard::SemanticBlackboard, capacity, contract,
    delegation, deontic_compose, dl, epistemic_boundaries, epistemic_ontology, evidential_etau,
    fuzzy_type2, jural, logic::deontic, reconciliation, responsibility, stit,
};
use qualia_core_db::q42_volume::{
    MAX_COMPRESSED_SUPERBLOCK_SIZE, Q42RangeQueryCursor, Q42RangeQueryPattern,
    Q42RangeQueryPlan, SUPERBLOCK_SIZE, WasmSliceQ42Reader,
};
use qualia_core_db::query::resolver::{InlineLiteral, classify_inline_literal};

use super::{ReceiptGraph, SYMBOLIC_LIMIT, external_resource, exact_u64};
use crate::{Args, AssetHost, Value, fields, record, required, text};

pub(super) const MAX_FACTS: usize = 255;
const LEX_PAGE_BYTES: usize = 1024 * 1024;
const MAX_TERM_BYTES: usize = 4096;
pub(super) const READ_BYTES: usize = MAX_COMPRESSED_SUPERBLOCK_SIZE + SUPERBLOCK_SIZE + LEX_PAGE_BYTES
    + MAX_TERM_BYTES + 4 * (MAX_FACTS + 1) * core::mem::size_of::<NQuin>();
const RECEIPT: &str = "urn:qualia:modality:verdict";
const SOURCE_CONTEXT: &str = "urn:qualia:modality:sourceContext";
const VERDICT: &str = "urn:qualia:modality:verdictStatus";
const RDF_TYPE: u64 = q_hash("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
const SUBCLASS: u64 = q_hash("http://www.w3.org/2000/01/rdf-schema#subClassOf");
const ABBREV_SUBCLASS: u64 = q_hash("rdfs:subClassOf");
const TRUE: u64 = q_hash("urn:qualia:modality:True");

impl AssetHost<'_> {
    pub(crate) fn evaluate_epistemic_jural_and_blackboard(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "op", "context_iri", "parameters"])?;
        let model_arg = required(args, "model")?;
        let operation = text(required(args, "op")?)?;
        let context = input_iri(args, "context_iri")?;
        let parameters = record(required(args, "parameters")?)?;
        let source = external_resource(self, model_arg)?;
        if source.kind != "q42" || source.bytes.len() > SYMBOLIC_LIMIT.saturating_sub(READ_BYTES) {
            return Err("Modality evaluation requires a Q42 volume within its symbolic byte budget".into());
        }
        let work = source.bytes.len().saturating_mul(2).saturating_add(MAX_FACTS * MAX_FACTS);
        self.charge(work, READ_BYTES)?;
        let source = external_resource(self, model_arg)?;
        let reader = WasmSliceQ42Reader::open(&source.bytes).map_err(|error| error.to_string())?;
        let mut model = Model::new(reader)?;
        let mut facts = [NQuin::default(); MAX_FACTS + 1];
        let len = model.context(context, &mut facts)?;
        let mut receipt = ReceiptGraph::new("urn:qualia:modality:derivedGraph");
        receipt.fact(RECEIPT, SOURCE_CONTEXT, context)?;
        let (mut report, verdict) = evaluate(operation, parameters, &facts[..len], &mut model, &mut receipt)?;
        drop(model);
        receipt.fact(RECEIPT, VERDICT, verdict)?;
        report.insert("verdict".into(), Value::String(verdict.into()));
        let rows = receipt.data.nquins.iter().map(|row| {
            Value::List(row.iter().copied().map(Value::U64).collect())
        }).collect();
        report.insert("derived_nquins".into(), Value::List(rows));
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        report.insert("receipt".into(), self.admit_encoded("q42", receipt.seal(budget)?)?);
        Ok(Value::Record(report))
    }
}

pub(super) struct Model<'a> {
    reader: WasmSliceQ42Reader<'a>,
    compressed: Vec<u8>,
    decoded: Vec<u8>,
    lex_page: Vec<u8>,
    lexical: [u8; MAX_TERM_BYTES],
}

impl<'a> Model<'a> {
    pub(super) fn new(reader: WasmSliceQ42Reader<'a>) -> Result<Self, String> {
        if !reader.is_unified_volume() {
            return Err("Modality evaluation requires a unified Q42 volume".into());
        }
        Ok(Self {
            reader,
            compressed: vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE],
            decoded: vec![0; SUPERBLOCK_SIZE],
            lex_page: vec![0; LEX_PAGE_BYTES],
            lexical: [0; MAX_TERM_BYTES],
        })
    }

    pub(super) fn term(&mut self, hash: u64) -> Result<&str, String> {
        let length = self.reader.lookup_lexicon_hash_into(hash, &mut self.lex_page, &mut self.lexical)
            .map_err(|error| error.to_string())?.ok_or_else(|| format!("Unresolved modality term {hash:#018x}"))?;
        let term = std::str::from_utf8(&self.lexical[..length]).map_err(|error| error.to_string())?;
        if hash == 0 || hash != q_hash(term) {
            return Err(format!("Invalid Q42LEX spelling for {hash:#018x}"));
        }
        Ok(term)
    }

    pub(super) fn iri(&mut self, hash: u64) -> Result<&str, String> {
        let iri = self.term(hash)?;
        if resource_identity_meta(iri).is_err() {
            return Err(format!("Invalid Q42LEX IRI for {hash:#018x}"));
        }
        Ok(iri)
    }

    pub(super) fn context(&mut self, iri: &str, out: &mut [NQuin; MAX_FACTS + 1]) -> Result<usize, String> {
        let context = q_hash(iri);
        if self.iri(context)? != iri {
            return Err("Modality context has an ambiguous lexical binding".into());
        }
        let plan = Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern { context: Some(context), ..Default::default() });
        let page = self.reader.execute_query_page_into(
            plan, Q42RangeQueryCursor::default(), &mut self.compressed, &mut self.decoded, out,
        ).map_err(|error| error.to_string())?;
        if page.next_cursor.is_some() || page.returned > MAX_FACTS {
            return Err(format!("Modality context exceeds the {MAX_FACTS}-fact bound"));
        }
        if page.returned == 0 || out[..page.returned].iter().any(|q| !q.verify_ecc_parity()) {
            return Err("Modality context has missing or corrupt facts".into());
        }
        Ok(page.returned)
    }
}

pub(super) fn input_iri<'a>(args: &'a Args, key: &str) -> Result<&'a str, String> {
    let iri = text(required(args, key)?)?;
    if iri.len() > MAX_TERM_BYTES || resource_identity_meta(iri).is_err() {
        return Err(format!("Invalid modality IRI for {key}"));
    }
    Ok(iri)
}

pub(super) fn admitted_iri<'a>(args: &'a Args, key: &str, model: &mut Model<'_>) -> Result<&'a str, String> {
    let iri = input_iri(args, key)?;
    if model.iri(q_hash(iri))? != iri {
        return Err(format!("Q42LEX spelling differs for {key}"));
    }
    Ok(iri)
}

pub(super) fn yes_no(value: bool) -> &'static str {
    if value { "urn:qualia:modality:True" } else { "urn:qualia:modality:False" }
}

fn tbox(facts: &[NQuin]) -> impl Iterator<Item = NQuin> + '_ {
    facts.iter().filter(|q| matches!(q.predicate, SUBCLASS | ABBREV_SUBCLASS)).copied()
}

fn evaluate(
    op: &str, args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph,
) -> Result<(BTreeMap<String, Value>, &'static str), String> {
    match op {
        "dung_argumentation" => dung(args, facts, model, receipt),
        "abductive_evidential_etau" => abduct(args, facts, model, receipt),
        "dl_subsumption" => subsumption(args, facts, model, receipt),
        "type2_fuzzy" => fuzzy(args, facts, model, receipt),
        "wordnet_cili_epistemic_stance" => stance(args, facts, model, receipt),
        "socratic_degradation_gate" => socratic(args, facts, model, receipt),
        "hohfeldian_jural_correlatives" => hohfeld(args, facts, model, receipt),
        "stit_agency_mens_rea" => agency(args, facts, model, receipt),
        "delegation_contract_accrual" => contract_accrual(args, facts, model, receipt),
        "semantic_blackboard_reconcile" => reconcile(args, facts, model, receipt),
        _ => Err(format!("Unknown modality operation {op}")),
    }
}

type ResultFields = Result<(BTreeMap<String, Value>, &'static str), String>;

fn dung(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["goal_iri"])?;
    let goal = admitted_iri(args, "goal_iri", model)?;
    let class = q_hash("urn:qualia:modality:Argument");
    let attacks = q_hash("urn:qualia:modality:attacks");
    let vertices: BTreeSet<_> = facts.iter().filter(|q| q.predicate == RDF_TYPE && q.object == class)
        .map(|q| q.subject).collect();
    if vertices.len() > argumentation::MAX_GROUNDED_ARGS || !vertices.contains(&q_hash(goal)) {
        return Err("Missing Dung goal or argument count exceeds its bound".into());
    }
    let nodes: Vec<u64> = vertices.into_iter().collect();
    let edges: Vec<_> = facts.iter().filter(|q| q.predicate == attacks).map(|q| (q.subject, q.object)).collect();
    if edges.iter().any(|(a, b)| !nodes.contains(a) || !nodes.contains(b)) {
        return Err("Dung attack refers to an undeclared argument".into());
    }
    for &node in &nodes { model.iri(node)?; }
    let accepted = argumentation::grounded_contains(&nodes, &edges, q_hash(goal));
    receipt.fact(goal, "urn:qualia:modality:grounded", yes_no(accepted))?;
    Ok((BTreeMap::from([("grounded".into(), Value::Bool(accepted))]), yes_no(accepted)))
}

fn abduct(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["observation_iri", "explains_iri"])?;
    let observation = admitted_iri(args, "observation_iri", model)?;
    let explains = q_hash(admitted_iri(args, "explains_iri", model)?);
    let start = q_hash(observation);
    let mut current = start;
    let mut seen = BTreeSet::from([start]);
    let mut evidence = None;
    loop {
        let mut incoming = facts.iter().filter(|q| q.predicate == explains && q.object == current);
        let Some(edge) = incoming.next() else { break };
        if incoming.next().is_some() || !seen.insert(edge.subject) || seen.len() > abductive::MAX_ABDUCTION_DEPTH {
            return Err("Abductive chain is ambiguous, cyclic or exceeds its depth bound".into());
        }
        current = edge.subject;
        evidence = Some(edge);
    }
    let hypothesis = abductive::abductive_explanation(facts, start, explains);
    let mut out = BTreeMap::new();
    if let (Some(id), Some(evidence)) = (hypothesis, evidence) {
        let iri = model.iri(id)?;
        receipt.fact(observation, "urn:qualia:modality:explainedBy", iri)?;
        out.insert("hypothesis_iri".into(), Value::String(iri.into()));
        let score = evidential_etau::evidential_score(evidence);
        if !score.is_finite() { return Err("Abductive evidence score is non-finite".into()); }
        out.insert("evidential_score".into(), Value::F64(f64::from(score)));
    }
    Ok((out, yes_no(hypothesis.is_some())))
}

fn subsumption(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["subclass_iri", "superclass_iri"])?;
    let sub = admitted_iri(args, "subclass_iri", model)?;
    let sup = admitted_iri(args, "superclass_iri", model)?;
    let tbox: Vec<_> = tbox(facts).collect();
    let result = dl::check_subsumption_quin(q_hash(sub), q_hash(sup), &tbox);
    if result { receipt.fact(sub, "http://www.w3.org/2000/01/rdf-schema#subClassOf", sup)?; }
    Ok((BTreeMap::from([("subsumed".into(), Value::Bool(result))]), yes_no(result)))
}

fn literal(value: u64) -> Result<f32, String> {
    let number = match classify_inline_literal(value) {
        Some(InlineLiteral::Integer(n)) => n as f32,
        Some(InlineLiteral::Decimal(n)) => n as f32 / 1_000_000.0,
        Some(InlineLiteral::Float(n)) => n,
        _ => return Err("Expected a finite inline Q42 number".into()),
    };
    if number.is_finite() { Ok(number) } else { Err("Non-finite inline Q42 number".into()) }
}

fn object(facts: &[NQuin], subject: u64, predicate: u64) -> Result<u64, String> {
    let mut rows = facts.iter().filter(|q| q.subject == subject && q.predicate == predicate);
    let first = rows.next().ok_or_else(|| format!("Missing modality fact {subject:#018x}/{predicate:#018x}"))?;
    if rows.next().is_some() { return Err("Ambiguous single-valued modality fact".into()); }
    Ok(first.object)
}

fn fuzzy(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["set_iri"])?;
    let set = admitted_iri(args, "set_iri", model)?;
    let point = q_hash("urn:qualia:modality:membershipPoint");
    let mut samples = Vec::new();
    for id in facts.iter().filter(|q| q.subject == q_hash(set) && q.predicate == point).map(|q| q.object) {
        let x = literal(object(facts, id, q_hash("urn:qualia:modality:x"))?)?;
        let lo = literal(object(facts, id, q_hash("urn:qualia:modality:lower"))?)?;
        let hi = literal(object(facts, id, q_hash("urn:qualia:modality:upper"))?)?;
        if !(0.0..=1.0).contains(&lo) || !(lo..=1.0).contains(&hi) {
            return Err("Invalid interval type-2 membership bounds".into());
        }
        samples.push((x, lo, hi));
    }
    samples.sort_unstable_by(|a, b| a.0.total_cmp(&b.0));
    if samples.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err("Duplicate interval type-2 membership coordinate".into());
    }
    let x: Vec<_> = samples.iter().map(|v| v.0).collect();
    let lo: Vec<_> = samples.iter().map(|v| v.1).collect();
    let hi: Vec<_> = samples.iter().map(|v| v.2).collect();
    let (left, right) = fuzzy_type2::karnik_mendel(&x, &lo, &hi)
        .filter(|(a, b)| a.is_finite() && b.is_finite() && a <= b)
        .ok_or("No finite interval type-2 centroid")?;
    receipt.fact(set, "urn:qualia:modality:centroidLeftF32", &format!("urn:f32:{:08x}", left.to_bits()))?;
    receipt.fact(set, "urn:qualia:modality:centroidRightF32", &format!("urn:f32:{:08x}", right.to_bits()))?;
    Ok((BTreeMap::from([
        ("left".into(), Value::F64(f64::from(left))), ("right".into(), Value::F64(f64::from(right))),
    ]), yes_no(true)))
}

fn stance(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    use epistemic_ontology::{EpistemicStanceRecord, PRED_WN_HYPERNYM};
    fields(args, &["subject_iri", "expected_mode", "language"])?;
    let subject = admitted_iri(args, "subject_iri", model)?;
    let expected_mode = text(required(args, "expected_mode")?)?;
    let expected = epistemic_ontology::parse_epistemic_mode_to_synset(expected_mode)
        .ok_or("Unknown epistemic stance mode")?;
    let language = text(required(args, "language")?)?;
    if language.len() > 35 || !language.split('-').all(|part| {
        !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_alphanumeric())
    }) {
        return Err("Invalid BCP-47 language tag".into());
    }
    let tbox: Vec<_> = facts.iter().filter(|q| matches!(q.predicate, SUBCLASS | ABBREV_SUBCLASS | PRED_WN_HYPERNYM))
        .map(|q| NQuin { subject: q.subject & 0x0FFF_FFFF_FFFF_FFFF, object: q.object & 0x0FFF_FFFF_FFFF_FFFF, ..*q })
        .collect();
    let selected: Vec<_> = facts.iter().filter(|q| q.subject == q_hash(subject)).copied().collect();
    let empty = EpistemicStanceRecord { subject: 0, synset_hash: 0, cili_hash: 0, confidence: 0, is_subsumed: false };
    let mut out = [empty; MAX_FACTS];
    let count = epistemic_ontology::evaluate_epistemic_stances(&selected, expected, &tbox, &mut out)
        .map_err(|error| format!("Epistemic stance evaluation: {error:?}"))?;
    let mut results = Vec::with_capacity(count);
    for row in &out[..count] {
        let label = epistemic_ontology::synset_to_canonical_label(row.synset_hash);
        let lemma = epistemic_ontology::lookup_epistemic_lemma(row.cili_hash, language).unwrap_or(label);
        let token = format!("urn:qualia:wordnet:{:015x}", row.synset_hash);
        receipt.fact(subject, "urn:qualia:modality:matchedStance", &token)?;
        results.push(Value::Record(BTreeMap::from([
            ("synset_hash".into(), Value::U64(row.synset_hash)),
            ("cili_hash".into(), Value::U64(row.cili_hash)),
            ("confidence_q16".into(), Value::U64(u64::from(row.confidence))),
            ("lemma".into(), Value::String(lemma.into())),
        ])));
    }
    Ok((BTreeMap::from([("stances".into(), Value::List(results))]), yes_no(count != 0)))
}

fn socratic(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["subject_iri", "predicate_iri"])?;
    let subject = admitted_iri(args, "subject_iri", model)?;
    let predicate = admitted_iri(args, "predicate_iri", model)?;
    let subject_hash = q_hash(subject);
    let predicate_hash = q_hash(predicate);
    let _ = object(facts, subject_hash, predicate_hash)?;
    let quin = facts.iter().find(|q| q.subject == subject_hash && q.predicate == predicate_hash)
        .ok_or("Missing Socratic claim")?;
    let vector = epistemic_boundaries::identify_degradation_vector(quin);
    let degraded = epistemic_boundaries::degrade_claim_to_socratic(vector);
    let mut out = BTreeMap::new();
    if let Some(degraded) = degraded {
        out.insert("probabilistic_map".into(), Value::String(degraded.probabilistic_map.into()));
        out.insert("socratic_prompt".into(), Value::String(degraded.socratic_prompt.into()));
        out.insert("immutable_disclaimer".into(), Value::String(degraded.immutable_disclaimer.into()));
    }
    receipt.fact(subject, "urn:qualia:modality:degraded", yes_no(degraded.is_some()))?;
    Ok((out, yes_no(degraded.is_some())))
}

fn hohfeld(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    use jural::*;
    fields(args, &["holder_iri", "counterparty_iri", "content_iri", "position"])?;
    let holder = admitted_iri(args, "holder_iri", model)?;
    let counterparty = admitted_iri(args, "counterparty_iri", model)?;
    let content = admitted_iri(args, "content_iri", model)?;
    const POSITIONS: [&str; 8] = ["claim", "duty", "privilege", "no_right", "power", "liability", "immunity", "disability"];
    let requested = text(required(args, "position")?)?;
    let position = POSITIONS.iter().position(|&name| name == requested)
        .map(|index| JURAL_CLAIM + index as u8).ok_or("Invalid Hohfeldian position")?;
    let relation = compile_jural_quin(q_hash(holder), position, q_hash(content), q_hash(counterparty), facts[0].context);
    let named_relation = q_hash("urn:qualia:modality:juralRelation");
    let declared_position = q_hash("urn:qualia:modality:position");
    let declared_content = q_hash("urn:qualia:modality:content");
    let declared_counterparty = q_hash("urn:qualia:modality:counterparty");
    let mut compiled = Vec::with_capacity(facts.len());
    for row in facts.iter().filter(|q| q.predicate == named_relation) {
        let position_iri = object(facts, row.object, declared_position)?;
        let code = POSITIONS.iter().position(|name| q_hash(&format!("urn:qualia:jural:{name}")) == position_iri)
            .map(|index| JURAL_CLAIM + index as u8).ok_or("Unknown jural position in graph")?;
        let content_iri = object(facts, row.object, declared_content)?;
        let counterparty_iri = object(facts, row.object, declared_counterparty)?;
        model.iri(content_iri)?;
        model.iri(counterparty_iri)?;
        compiled.push(compile_jural_quin(row.subject, code, content_iri, counterparty_iri, row.context));
    }
    compiled.extend(facts.iter().filter(|q| {
        [relation.predicate, correlative_quin(&relation).predicate].contains(&q.predicate)
    }).copied());
    if !compiled.contains(&relation) && !compiled.iter().any(|q| {
        q.subject == relation.subject && q.object == relation.object && q.predicate == relation.predicate
    }) {
        return Err("Requested Hohfeldian relation is absent from the source graph".into());
    }
    let taxonomy: Vec<_> = tbox(facts).collect();
    let mut classes = facts.iter().filter(|q| q.subject == relation.subject && q.predicate == RDF_TYPE).peekable();
    let exclusive = content == "https://ns.webcivics.net/values/inherentDignity"
        || facts.iter().any(|q| q.subject == q_hash(content)
            && q.predicate == q_hash("urn:qualia:modality:naturalPersonOnly") && q.object == TRUE);
    if exclusive && matches!(position, JURAL_CLAIM | JURAL_PRIVILEGE | JURAL_IMMUNITY) && classes.peek().is_none() {
        return Err("Natural-person-only claim requires a declared holder class".into());
    }
    let invalid_personhood = exclusive && matches!(position, JURAL_CLAIM | JURAL_PRIVILEGE | JURAL_IMMUNITY)
        && classes.all(|class| personhood_category_error(class.object, exclusive, position, &taxonomy));
    let has_correlative = jural_correlativity_holds(&relation, &compiled);
    let expected = POSITIONS[usize::from(correlative(position) - JURAL_CLAIM)];
    receipt.fact(holder, "urn:qualia:modality:personhoodCategoryError", yes_no(invalid_personhood))?;
    receipt.fact(holder, "urn:qualia:modality:correlativePresent", yes_no(has_correlative))?;
    if !invalid_personhood && !has_correlative {
        receipt.fact(counterparty, "urn:qualia:modality:missingCorrelativePosition", &format!("urn:qualia:jural:{expected}"))?;
        receipt.fact(counterparty, "urn:qualia:modality:missingCorrelativeContent", content)?;
        receipt.fact(counterparty, "urn:qualia:modality:missingCorrelativeToward", holder)?;
    }
    Ok((BTreeMap::from([
        ("personhood_category_error".into(), Value::Bool(invalid_personhood)),
        ("correlative_present".into(), Value::Bool(has_correlative)),
        ("expected_correlative".into(), Value::String(expected.into())),
    ]), yes_no(!invalid_personhood && has_correlative)))
}

fn agency(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["agent_iri", "content_iri", "norm"])?;
    let agent = admitted_iri(args, "agent_iri", model)?;
    let content = admitted_iri(args, "content_iri", model)?;
    let (opcode, pred) = match text(required(args, "norm")?)? {
        "obligation" => (deontic::OP_OBLIGATE, q_hash("urn:qualia:modality:obligatedTo")),
        "prohibition" => (deontic::OP_FORBID, q_hash("urn:qualia:modality:prohibitedFrom")),
        _ => return Err("Unknown STIT norm".into()),
    };
    let agent_hash = q_hash(agent);
    let action = q_hash(content);
    let mut norms = facts.iter().filter(|q| q.subject == agent_hash && q.object == action
        && (q.predicate == pred || q.predicate == deontic::compile_norm_quin(agent_hash, opcode, action, action, q.context, 0, false).predicate));
    let source_norm = norms.next().ok_or("STIT norm must be present as a source Q42 fact")?;
    if norms.next().is_some() { return Err("Ambiguous STIT source norm".into()); }
    let norm = deontic::compile_norm_quin(agent_hash, opcode, action, action, source_norm.context, 0, false);
    let knows = q_hash("urn:qualia:modality:knows");
    let know_facts: Vec<_> = facts.iter().filter(|q| q.predicate == knows)
        .map(|q| NQuin { predicate: u64::from(qualia_core_db::modalities::epistemic::OP_KNOWS), ..*q }).collect();
    let duty = facts.iter().any(|q| q.subject == agent_hash && q.object == action && q.predicate == q_hash("urn:qualia:modality:dutyToKnow"));
    let mens_rea = deontic_compose::classify_mens_rea(&norm, facts, &know_facts, duty);
    let status = stit::agentive_status(&norm, facts);
    let adjudicated = |predicate| facts.iter().any(|q| q.subject == agent_hash && q.object == action && q.predicate == q_hash(predicate));
    let confirmed = adjudicated("urn:qualia:modality:confirmedResponsibility");
    let dismissed = adjudicated("urn:qualia:modality:dismissedResponsibility");
    if confirmed && dismissed { return Err("Conflicting responsibility adjudication in source graph".into()); }
    let outcome = responsibility::adjudicate(confirmed, dismissed);
    let supported = status != deontic::DeonticStatus::Violated;
    receipt.fact(agent, "urn:qualia:modality:agencyCompliance", yes_no(supported))?;
    receipt.fact(agent, "urn:qualia:modality:mensRea", &format!("urn:qualia:modality:{mens_rea:?}"))?;
    Ok((BTreeMap::from([
        ("norm_status".into(), Value::String(format!("{status:?}"))),
        ("mens_rea".into(), Value::String(format!("{mens_rea:?}"))),
        ("adjudication_status".into(), Value::String(format!("{outcome:?}"))),
    ]), yes_no(supported)))
}

fn capacity_status(hash: u64) -> Result<capacity::CapacityStatus, String> {
    match hash {
        n if n == q_hash("urn:qualia:modality:Intact") => Ok(capacity::CapacityStatus::Intact),
        n if n == q_hash("urn:qualia:modality:Impaired") => Ok(capacity::CapacityStatus::Impaired),
        n if n == q_hash("urn:qualia:modality:UnderDuress") => Ok(capacity::CapacityStatus::UnderDuress),
        _ => Err("Unknown declared capacity status".into()),
    }
}

fn contract_accrual(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["contract_iri", "stock_iri", "event_iri"])?;
    let agreement = admitted_iri(args, "contract_iri", model)?;
    let id = q_hash(agreement);
    let offeror = object(facts, id, q_hash("urn:qualia:modality:offeror"))?;
    let acceptor = object(facts, id, q_hash("urn:qualia:modality:acceptor"))?;
    let authority = object(facts, id, q_hash("urn:qualia:modality:authority"))?;
    let capacity = q_hash("urn:qualia:modality:capacity");
    let stipulated = object(facts, id, q_hash("urn:qualia:modality:stipulated"))? == TRUE;
    let accepted = object(facts, id, q_hash("urn:qualia:modality:accepted"))? == TRUE;
    let delegated = delegation::has_delegated_authority(facts, authority, offeror);
    let binding = delegated && contract::is_binding_contract(
        stipulated, accepted, capacity_status(object(facts, offeror, capacity)?)?,
        capacity_status(object(facts, acceptor, capacity)?)?,
    );
    let mut out = BTreeMap::from([("delegated".into(), Value::Bool(delegated)), ("binding".into(), Value::Bool(binding))]);
    receipt.fact(agreement, "urn:qualia:modality:delegatedAuthority", yes_no(delegated))?;
    receipt.fact(agreement, "urn:qualia:modality:bindingContract", yes_no(binding))?;
    if args.contains_key("stock_iri") || args.contains_key("event_iri") {
        let stock = admitted_iri(args, "stock_iri", model)?;
        let event = admitted_iri(args, "event_iri", model)?;
        let report = accrual_loss(facts, q_hash(stock), q_hash(event))?;
        receipt.fact(stock, "urn:qualia:modality:criticalStockBreached", yes_no(report.is_critical_stock_breached))?;
        receipt.fact(stock, "urn:qualia:modality:effectiveIrreversibility",
            &format!("urn:qualia:modality:{:?}", report.effective_irreversibility))?;
        out.insert("critical_stock_breached".into(), Value::Bool(report.is_critical_stock_breached));
        out.insert("effective_irreversibility".into(), Value::String(format!("{:?}", report.effective_irreversibility)));
    }
    Ok((out, yes_no(binding)))
}

fn inline_uint(facts: &[NQuin], subject: u64, predicate: &str) -> Result<u64, String> {
    let n = object(facts, subject, q_hash(predicate))?;
    if let Some(InlineLiteral::Integer(n)) = classify_inline_literal(n) {
        return u64::try_from(n).map_err(|_| format!("Negative {predicate}"));
    }
    Err(format!("Expected an inline unsigned integer for {predicate}"))
}

fn accrual_loss(facts: &[NQuin], stock_id: u64, event_id: u64) -> Result<accrual::AccrualLossReport, String> {
    let stock_u = |key| inline_uint(facts, stock_id, key);
    let stock_f = |key| literal(object(facts, stock_id, q_hash(key))?);
    let event_f = |key| literal(object(facts, event_id, q_hash(key))?);
    let maturity = stock_u("urn:qualia:modality:maturity")?;
    let irreversibility = stock_u("urn:qualia:modality:irreversibility")?;
    let age = stock_f("urn:qualia:modality:accrualAgeYears")?;
    let extent = event_f("urn:qualia:modality:extentLossPct")?;
    let biomass = stock_f("urn:qualia:modality:biomass")?;
    let structural_complexity = stock_f("urn:qualia:modality:structuralComplexity")?;
    let hydrological_function = stock_f("urn:qualia:modality:hydrologicalFunction")?;
    let genetic_distinctness = stock_f("urn:qualia:modality:geneticDistinctness")?;
    let custodial_depth = stock_f("urn:qualia:modality:custodialDepth")?;
    if maturity > 4 || irreversibility > 3 || !(0.0..=100.0).contains(&extent)
        || [age, biomass, structural_complexity, hydrological_function, genetic_distinctness, custodial_depth]
            .iter().any(|&v| v < 0.0)
        || object(facts, event_id, q_hash("urn:qualia:modality:impactedStock"))? != stock_id
    {
        return Err("Invalid accrual stock or loss data".into());
    }
    let stock = accrual::StockEntity {
        entity_id: stock_id, maturity: accrual::MaturityClass::from_u8(maturity as u8), accrual_age_years: age,
        formation_time_s: stock_u("urn:qualia:modality:formationSeconds")?,
        destruction_time_s: stock_u("urn:qualia:modality:destructionSeconds")?,
        profile: accrual::AccrualProfile { biomass, structural_complexity, hydrological_function, genetic_distinctness, custodial_depth },
        irreversibility: accrual::IrreversibilityClass::from_u8(irreversibility as u8),
    };
    let event = accrual::LossEvent {
        event_id, stock_id, cause_hash: object(facts, event_id, q_hash("urn:qualia:modality:cause"))?,
        timestamp_epoch: u32::try_from(inline_uint(facts, event_id, "urn:qualia:modality:eventEpoch")?)
            .map_err(|_| "Accrual event time outside u32 range")?,
        extent_loss_pct: extent, irreversibility_override: None,
    };
    Ok(accrual::eval_accrual_loss(&stock, &event))
}

fn reconcile(args: &Args, facts: &[NQuin], model: &mut Model<'_>, receipt: &mut ReceiptGraph) -> ResultFields {
    fields(args, &["subject_iri", "external_context_iri", "internal_vintage", "external_vintage", "now", "expected_lag_s"])?;
    let subject = admitted_iri(args, "subject_iri", model)?;
    let external = admitted_iri(args, "external_context_iri", model)?;
    let mut external_facts = [NQuin::default(); MAX_FACTS + 1];
    let external_len = model.context(external, &mut external_facts)?;
    let mut board = SemanticBlackboard::new();
    for (channel, rows) in [("internal", facts), ("external", &external_facts[..external_len])] {
        for &quin in rows.iter().filter(|q| q.subject == q_hash(subject)) {
            board.write(channel, quin)?;
        }
        board.freeze_channel(channel)?;
    }
    let fingerprint = |channel| -> Result<u64, String> {
        let state = board.read(channel).filter(|rows| !rows.is_empty()).ok_or("Missing blackboard subject channel")?;
        let mut facts: Vec<_> = state.iter().map(|q| (q.subject, q.predicate, q.object)).collect();
        facts.sort_unstable();
        facts.dedup();
        let mut hash = blake3::Hasher::new();
        for (s, p, o) in facts {
            for part in [s, p, o] { hash.update(&part.to_le_bytes()); }
        }
        let bytes = hash.finalize();
        Ok(u64::from_le_bytes(bytes.as_bytes()[..8].try_into().unwrap()))
    };
    let time = |key| -> Result<u32, String> {
        u32::try_from(exact_u64(required(args, key)?)?).map_err(|_| format!("Out of range {key}"))
    };
    let now = time("now")?;
    let report = reconciliation::compare_views(q_hash(subject), fingerprint("internal")?, fingerprint("external")?,
        time("internal_vintage")?, time("external_vintage")?, now, time("expected_lag_s")?);
    let status = format!("{:?}", report.status);
    let stale = report.is_stale(now);
    receipt.fact(subject, "urn:qualia:modality:reconciliationStatus", &format!("urn:qualia:modality:{status}"))?;
    receipt.fact(subject, "urn:qualia:modality:stale", yes_no(stale))?;
    Ok((BTreeMap::from([
        ("status".into(), Value::String(status)),
        ("stale".into(), Value::Bool(stale)),
        ("divergence_score".into(), Value::F64(f64::from(report.divergence_score))),
        ("blackboard_lamport".into(), Value::U64(board.lamport())),
    ]), yes_no(report.status == reconciliation::ReconciliationStatus::Converged && !stale)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_correlative_and_refuses_the_personhood_category_error() {
        const CONTEXT: &str = "urn:test:jural:context";
        const HOLDER: &str = "urn:test:jural:holder";
        const PARTY: &str = "urn:test:jural:party";
        const DIGNITY: &str = "https://ns.webcivics.net/values/inherentDignity";
        let mut source = ReceiptGraph::new(CONTEXT);
        source.fact(HOLDER, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type", "urn:test:jural:Corporation").unwrap();
        source.fact(HOLDER, "urn:qualia:modality:juralRelation", "urn:test:jural:assertion").unwrap();
        source.fact("urn:test:jural:assertion", "urn:qualia:modality:position", "urn:qualia:jural:claim").unwrap();
        source.fact("urn:test:jural:assertion", "urn:qualia:modality:counterparty", PARTY).unwrap();
        source.fact("urn:test:jural:assertion", "urn:qualia:modality:content", DIGNITY).unwrap();
        let mut host = AssetHost {
            workspace: Default::default(), inputs: &[], limits: &crate::ToolLimits::default(),
            cancel: &std::sync::atomic::AtomicBool::new(false), work: 0,
        };
        let model = host.admit_encoded("q42", source.seal(SYMBOLIC_LIMIT).unwrap()).unwrap();
        let args = BTreeMap::from([
            ("model".into(), model),
            ("op".into(), Value::String("hohfeldian_jural_correlatives".into())),
            ("context_iri".into(), Value::String(CONTEXT.into())),
            ("parameters".into(), Value::Record(BTreeMap::from([
                ("holder_iri".into(), Value::String(HOLDER.into())),
                ("counterparty_iri".into(), Value::String(PARTY.into())),
                ("content_iri".into(), Value::String(DIGNITY.into())),
                ("position".into(), Value::String("claim".into())),
            ]))),
        ]);
        let result = host.evaluate_epistemic_jural_and_blackboard(&args).unwrap();
        let report = record(&result).unwrap();
        assert!(matches!(report["personhood_category_error"], Value::Bool(true)));
        assert!(matches!(report["correlative_present"], Value::Bool(false)));
        assert!(host.workspace.encoded(&report["receipt"]).is_ok());
    }
}

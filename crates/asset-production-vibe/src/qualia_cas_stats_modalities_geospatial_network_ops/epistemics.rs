//! Evidence-scoped research audits for admitted Q42 and HMC content.

use std::collections::{BTreeMap, BTreeSet};

use qualia_core_db::{NQuin, q_hash};
use qualia_core_db::bundle::{BundleReader, semantic_model::{SEMANTIC_MODEL_KEY, open_semantic_model}};
use qualia_core_db::q42_volume::WasmSliceQ42Reader;
use qualia_core_db::research::{
    DarkLink, EpistemicAssessment, HypothesisGraph, HypothesisNode, RealityCategory, SentimentAssessment,
    detect_concealment_patterns, detect_provenance_gaps, reconcile_perspectives, register_perspective,
    hypothesis::{Evaluation, NodeStatus},
};
use qualia_core_db::specialized_libs::computational_economics::forensic_economics::{
    NquinVector, compute_malfeasance_delta, compute_narrative_divergence,
};

use super::{ReceiptGraph, SYMBOLIC_LIMIT, external_resource};
use super::modalities::{MAX_FACTS, Model, READ_BYTES, admitted_iri, input_iri, yes_no};
use crate::{Args, AssetHost, Value, boolean, fields, list, num, record, required, text, vector};

const AUDIT: &str = "urn:qualia:research:audit";
const RDF_TYPE: u64 = q_hash("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
const CONTENT: u64 = q_hash("http://purl.org/dc/terms/description");
const DERIVED_FROM: u64 = q_hash("http://www.w3.org/ns/prov#wasDerivedFrom");
const ATTRIBUTED_TO: u64 = q_hash("http://www.w3.org/ns/prov#wasAttributedTo");
const MAX_TRACE_POINTS: usize = 256;

struct Audit {
    output: BTreeMap<String, Value>,
    grounded: Vec<NQuin>,
    receipt: ReceiptGraph,
}

impl AssetHost<'_> {
    pub(crate) fn audit_research_and_grounding(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "task", "context_iri", "parameters"])?;
        let model_arg = required(args, "model")?;
        let context = input_iri(args, "context_iri")?;
        let task = text(required(args, "task")?)?;
        let params = record(required(args, "parameters")?)?;
        let source = external_resource(self, model_arg)?;
        if source.bytes.len() > self.limits.resource_bytes {
            return Err("Research source exceeds its retained resource budget".into());
        }
        let work = source.bytes.len().saturating_mul(2).saturating_add(MAX_FACTS * MAX_FACTS);
        self.charge(work, READ_BYTES + 8 * 1024 * 1024)?;
        let source = external_resource(self, model_arg)?;
        let reader = match source.kind.as_str() {
            "q42" => {
                check_model_size(source.bytes.len())?;
                WasmSliceQ42Reader::open(&source.bytes).map_err(|error| error.to_string())?
            }
            "hmc" => {
                let bundle = BundleReader::parse(&source.bytes).map_err(|error| format!("Research HMC: {error:?}"))?;
                let semantic = bundle.get(SEMANTIC_MODEL_KEY).or_else(|| bundle.get("manifest.q42"))
                    .ok_or("Research HMC semantic entry is missing")?;
                check_model_size(semantic.len())?;
                open_semantic_model(&bundle).map_err(|error| error.to_string())?
            }
            _ => return Err("Research audits require an encoded Q42 volume or HMC semantic model".into()),
        };
        let mut model = Model::new(reader)?;
        let mut facts = [NQuin::default(); MAX_FACTS + 1];
        let count = model.context(context, &mut facts)?;
        let facts = &facts[..count];
        let mut audit = Audit {
            output: BTreeMap::new(), grounded: Vec::new(),
            receipt: ReceiptGraph::new("urn:qualia:research:derivedReport"),
        };
        audit.receipt.fact(AUDIT, "urn:qualia:research:sourceContext", context)?;
        audit.receipt.fact(AUDIT, "urn:qualia:research:auditTask", &format!("urn:qualia:research:{task}"))?;
        match task {
            "hypothesis_graph" => hypothesis(params, facts, &mut model, &mut audit)?,
            "infer_dark_links" => dark_links(params, facts, &mut model, &mut audit)?,
            "detect_provenance_gaps_and_concealment" => provenance(params, facts, &mut model, &mut audit)?,
            "detect_deceptive_fiction_and_sentiment_manip" => sentiment(params, facts, &mut model, &mut audit)?,
            "reconcile_perspectives" => perspectives(params, facts, &mut model, &mut audit)?,
            "verify_hybrid_graph_grounding" => grounding(params, facts, &mut model, &mut audit)?,
            "forensic_malfeasance_narrative_divergence" => forensic(params, facts, &mut model, &mut audit)?,
            _ => return Err(format!("Unknown research audit task {task}")),
        }
        drop(model);
        audit.grounded.sort_unstable();
        audit.grounded.dedup();
        audit.output.insert("grounded_nquins".into(), Value::List(audit.grounded.iter().map(|q| {
            Value::List([q.subject, q.predicate, q.object, q.context, q.metadata, q.parity]
                .into_iter().map(Value::U64).collect())
        }).collect()));
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        audit.output.insert("report".into(), self.admit_encoded("q42", audit.receipt.seal(budget)?)?);
        Ok(Value::Record(audit.output))
    }
}

fn check_model_size(size: usize) -> Result<(), String> {
    if size > SYMBOLIC_LIMIT.saturating_sub(READ_BYTES + 8 * 1024 * 1024) {
        return Err("Research model and working storage exceed the symbolic byte budget".into());
    }
    Ok(())
}

fn related(facts: &[NQuin], subject: u64, predicate: u64) -> Result<Option<&NQuin>, String> {
    let mut rows = facts.iter().filter(|q| q.subject == subject && q.predicate == predicate);
    let first = rows.next();
    if rows.next().is_some() { return Err("Ambiguous single-valued research evidence".into()); }
    Ok(first)
}

fn rdf_text(model: &mut Model<'_>, hash: u64) -> Result<String, String> {
    let term = model.term(hash)?;
    let mut strings = serde_json::Deserializer::from_str(term).into_iter::<String>();
    let value = strings.next().transpose().map_err(|error| error.to_string())?.ok_or("Missing RDF text")?;
    let suffix = &term[strings.byte_offset()..];
    let language = suffix.strip_prefix('@').is_some_and(|language| {
        !language.is_empty() && language.split('-').all(|part| !part.is_empty()
            && part.bytes().all(|byte| byte.is_ascii_alphanumeric()))
    });
    if !suffix.is_empty() && !language && suffix != "^^<http://www.w3.org/2001/XMLSchema#string>" {
        return Err("Research text requires a plain, language-tagged or xsd:string RDF literal".into());
    }
    Ok(value)
}

fn content(facts: &[NQuin], model: &mut Model<'_>, subject: u64, grounded: &mut Vec<NQuin>) -> Result<String, String> {
    let row = related(facts, subject, CONTENT)?.ok_or("The selected research subject has no content literal")?;
    let value = rdf_text(model, row.object)?;
    grounded.push(*row);
    Ok(value)
}

fn hypothesis(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &[])?;
    let class = q_hash("urn:qualia:research:Hypothesis");
    let supports = q_hash("urn:qualia:research:supports");
    let contradicts = q_hash("urn:qualia:research:contradicts");
    let mut graph = HypothesisGraph::new();
    for row in facts.iter().filter(|q| q.predicate == RDF_TYPE && q.object == class) {
        let iri = model.iri(row.subject)?.to_owned();
        if graph.nodes.contains_key(&iri) { continue; }
        let statement = content(facts, model, row.subject, &mut audit.grounded)?;
        let author = related(facts, row.subject, ATTRIBUTED_TO)?.ok_or("Hypothesis lacks an attributed author")?;
        let agent_id = model.iri(author.object)?.to_owned();
        graph.create(HypothesisNode {
            id: iri.clone(), statement, agent_id, evaluations: Vec::new(),
            dark_links: Vec::new(), gaps: Vec::new(), status: NodeStatus::Open,
        });
        audit.grounded.extend([*row, *author]);
        for evidence in facts.iter().filter(|q| q.object == row.subject && matches!(q.predicate, p if p == supports || p == contradicts)) {
            let evidence_iri = model.iri(evidence.subject)?.to_owned();
            let assessor = related(facts, evidence.subject, ATTRIBUTED_TO)?.ok_or("Hypothesis evaluation lacks attribution")?;
            let author = model.iri(assessor.object)?.to_owned();
            let label = if evidence.predicate == supports { "supports" } else { "contradicts" };
            graph.contribute_evaluation(&iri, Evaluation {
                agent_id: author, assessment: label.into(), score: 0.0, timestamp: String::new(),
            });
            audit.receipt.fact(&evidence_iri, &format!("urn:qualia:research:{label}"), &iri)?;
            audit.grounded.extend([*evidence, *assessor]);
        }
        audit.receipt.fact(AUDIT, "urn:qualia:research:reviewedHypothesis", &iri)?;
    }
    if graph.nodes.is_empty() { return Err("No admitted research hypotheses in context".into()); }
    audit.output.insert("hypotheses".into(), Value::List(graph.nodes.into_values().map(|node| {
        Value::Record(BTreeMap::from([
            ("iri".into(), Value::String(node.id)), ("statement".into(), Value::String(node.statement)),
            ("evaluation_count".into(), Value::U64(node.evaluations.len() as u64)),
        ]))
    }).collect()));
    Ok(())
}

fn dark_links(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["source_iri", "target_iri", "relation_iri"])?;
    let source = admitted_iri(args, "source_iri", model)?;
    let target = admitted_iri(args, "target_iri", model)?;
    let relation = admitted_iri(args, "relation_iri", model)?;
    let (s, p, o) = (q_hash(source), q_hash(relation), q_hash(target));
    if s == o { return Err("Dark-link endpoints must differ".into()); }
    let direct: Vec<_> = facts.iter().filter(|q| q.subject == s && q.predicate == p && q.object == o).copied().collect();
    if !direct.is_empty() {
        audit.grounded.extend(direct);
        audit.receipt.fact(AUDIT, "urn:qualia:research:directLinkPresent", yes_no(true))?;
        audit.output.insert("direct_link_present".into(), Value::Bool(true));
        return Ok(());
    }
    let claim = format!("urn:qualia:research:dark-link:{s:016x}:{p:016x}:{o:016x}");
    let mut link = DarkLink::new(&claim, source, target, relation);
    let mut intermediates = BTreeSet::new();
    for first in facts.iter().filter(|q| q.subject == s && q.predicate == p && q.object != s && q.object != o) {
        if let Some(second) = facts.iter().find(|q| q.subject == first.object && q.predicate == p && q.object == o) {
            if intermediates.insert(first.object) {
                let bridge = model.iri(first.object)?;
                link.add_evidence(bridge);
                audit.grounded.extend([*first, *second]);
                audit.receipt.fact(&claim, "urn:qualia:research:possibleBridge", bridge)?;
            }
        }
    }
    let proposed = !link.evidence.is_empty();
    if proposed {
        audit.receipt.fact(&claim, "urn:qualia:research:proposedSource", source)?;
        audit.receipt.fact(&claim, "urn:qualia:research:proposedTarget", target)?;
        audit.receipt.fact(&claim, "urn:qualia:research:proposedRelation", relation)?;
        audit.receipt.fact(AUDIT, "urn:qualia:research:inferredDarkLink", &claim)?;
    }
    audit.output.insert("direct_link_present".into(), Value::Bool(false));
    audit.output.insert("proposed".into(), Value::Bool(proposed));
    audit.output.insert("evidence_paths".into(), Value::List(link.evidence.into_iter().map(Value::String).collect()));
    Ok(())
}

fn selected_texts(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<Vec<(String, String)>, String> {
    let input = list(required(args, "item_iris")?)?;
    if input.is_empty() || input.len() > 64 { return Err("Research audit requires 1..=64 text items".into()); }
    let mut ids = BTreeSet::new();
    let mut items = Vec::with_capacity(input.len());
    for item in input {
        let iri = text(item)?;
        let id = q_hash(iri);
        if !ids.insert(id) || model.iri(id)? != iri { return Err("Duplicate or unresolved research item".into()); }
        items.push((iri.to_owned(), content(facts, model, id, &mut audit.grounded)?));
    }
    Ok(items)
}

fn provenance(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["item_iris"])?;
    let items = selected_texts(args, facts, model, audit)?;
    let mut refs = Vec::with_capacity(items.len());
    for (iri, text) in &items {
        let sources: Vec<_> = facts.iter().filter(|q| q.subject == q_hash(iri) && q.predicate == DERIVED_FROM).collect();
        let mut source_iri = None;
        for source in sources {
            let reference = model.iri(source.object)?;
            if source_iri.is_none() { source_iri = Some(reference.to_owned()); }
            audit.grounded.push(*source);
            audit.receipt.fact(iri, "http://www.w3.org/ns/prov#wasDerivedFrom", reference)?;
        }
        if source_iri.is_none() { audit.receipt.fact(iri, "urn:qualia:research:missingSource", yes_no(true))?; }
        refs.push((iri.clone(), text.clone(), source_iri));
    }
    let gaps = detect_provenance_gaps(&refs);
    let indicators = detect_concealment_patterns(&items);
    audit.receipt.fact(AUDIT, "urn:qualia:research:lexicalConcealmentIndicators", yes_no(!indicators.is_empty()))?;
    audit.output.insert("provenance_gap_indicators".into(), strings(gaps));
    audit.output.insert("lexical_concealment_indicators".into(), strings(indicators));
    Ok(())
}

fn strings(values: Vec<String>) -> Value {
    Value::List(values.into_iter().map(Value::String).collect())
}

fn sentiment(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["item_iris"])?;
    let items = selected_texts(args, facts, model, audit)?;
    let declared_case = q_hash("urn:qualia:research:declaredRealityCategory");
    let mut results = Vec::with_capacity(items.len());
    for (iri, text) in &items {
        let declared = related(facts, q_hash(iri), declared_case)?;
        if let Some(fact) = declared { audit.grounded.push(*fact); }
        let factual = declared.is_some_and(|q| q.object == q_hash("urn:qualia:research:Factual"));
        let lexical = EpistemicAssessment::classify_reality(text);
        let mismatch = factual && EpistemicAssessment::detect_deceptive_fiction(text, RealityCategory::Factual);
        let sentiment = SentimentAssessment::assess_sentiment(text);
        audit.receipt.fact(iri, "urn:qualia:research:lexicalRealityCategory", &format!("urn:qualia:research:{}", lexical.as_str()))?;
        audit.receipt.fact(iri, "urn:qualia:research:lexicalFictionMismatch", yes_no(mismatch))?;
        results.push(Value::Record(BTreeMap::from([
            ("item_iri".into(), Value::String(iri.clone())),
            ("lexical_category".into(), Value::String(lexical.as_str().into())),
            ("lexical_fiction_mismatch".into(), Value::Bool(mismatch)),
            ("lexical_sentiment".into(), Value::F64(sentiment)),
        ])));
    }
    let texts = items.into_iter().map(|(_, text)| text).collect::<Vec<_>>();
    let indicators = SentimentAssessment::detect_sentiment_manipulation(&texts);
    audit.receipt.fact(AUDIT, "urn:qualia:research:lexicalSentimentIndicators", yes_no(!indicators.is_empty()))?;
    audit.output.insert("items".into(), Value::List(results));
    audit.output.insert("lexical_sentiment_indicators".into(), strings(indicators));
    Ok(())
}

fn perspectives(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["first_iri", "second_iri"])?;
    let a = admitted_iri(args, "first_iri", model)?;
    let b = admitted_iri(args, "second_iri", model)?;
    let [first, second] = [a, b].map(|iri| {
        let author = related(facts, q_hash(iri), ATTRIBUTED_TO)?.ok_or("Perspective lacks attribution")?;
        let author_iri = model.iri(author.object)?.to_owned();
        let text = content(facts, model, q_hash(iri), &mut audit.grounded)?;
        audit.grounded.push(*author);
        Ok::<_, String>(register_perspective(iri, &author_iri, &text))
    });
    let result = reconcile_perspectives(&first?, &second?);
    audit.receipt.fact(a, "urn:qualia:research:comparedPerspective", b)?;
    audit.receipt.fact(AUDIT, "urn:qualia:research:lexicallyReconcilable", yes_no(result.reconcilable))?;
    audit.output.extend([
        ("common_words".into(), Value::String(result.common_ground)),
        ("lexical_similarity".into(), Value::F64(result.similarity)),
        ("lexical_conflict_indicators".into(), Value::U64(result.conflict_count as u64)),
        ("lexically_reconcilable".into(), Value::Bool(result.reconcilable)),
    ]);
    Ok(())
}

fn grounding(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["claims"])?;
    let claims = list(required(args, "claims")?)?;
    if claims.is_empty() || claims.len() > MAX_FACTS { return Err("Invalid grounding claim count".into()); }
    let mut results = Vec::with_capacity(claims.len());
    let mut bound = 0;
    for claim in claims {
        let claim = record(claim)?;
        fields(claim, &["subject_iri", "predicate_iri", "object_iri"])?;
        let subject = input_iri(claim, "subject_iri")?;
        let predicate = input_iri(claim, "predicate_iri")?;
        let object = input_iri(claim, "object_iri")?;
        let terms = [subject, predicate, object];
        let [s, p, o] = terms.map(q_hash);
        let mut supported = false;
        for row in facts.iter().filter(|q| q.subject == s && q.predicate == p && q.object == o) {
            for term in terms {
                if model.iri(q_hash(term))? != term { return Err("Candidate grounding term disagrees with Q42LEX".into()); }
            }
            audit.grounded.push(*row);
            supported = true;
        }
        if supported {
            bound += 1;
            audit.receipt.fact(subject, predicate, object)?;
        }
        results.push(Value::Bool(supported));
    }
    audit.receipt.fact(AUDIT, "urn:qualia:research:allClaimsGraphSupported", yes_no(bound == claims.len()))?;
    audit.output.insert("graph_supported".into(), Value::List(results));
    audit.output.insert("graph_supported_count".into(), Value::U64(bound as u64));
    Ok(())
}

fn forensic(args: &Args, facts: &[NQuin], model: &mut Model<'_>, audit: &mut Audit) -> Result<(), String> {
    fields(args, &["evidence_iri", "capital_allocated", "delivered_utility", "inverted", "factual", "narrative"])?;
    let evidence = admitted_iri(args, "evidence_iri", model)?;
    let associations: Vec<_> = facts.iter().filter(|q| q.subject == q_hash(evidence) && q.predicate == DERIVED_FROM).collect();
    if associations.is_empty() { return Err("Forensic assessment requires source provenance in the admitted graph".into()); }
    for association in associations {
        let provenance = model.iri(association.object)?;
        audit.receipt.fact(evidence, "http://www.w3.org/ns/prov#wasDerivedFrom", provenance)?;
        audit.grounded.push(*association);
    }
    let trace = |key| -> Result<Vec<NquinVector>, String> {
        let input = list(required(args, key)?)?;
        if input.is_empty() || input.len() > MAX_TRACE_POINTS { return Err(format!("Invalid forensic {key} trace length")); }
        input.iter().map(|sample| vector(sample).map(NquinVector::from_array)).collect()
    };
    let factual = trace("factual")?;
    let narrative = trace("narrative")?;
    if factual.len() != narrative.len() { return Err("Forensic traces require paired samples".into()); }
    let divergence = compute_narrative_divergence(&factual, &narrative).map_err(|error| format!("Narrative divergence: {error:?}"))?;
    let delta = compute_malfeasance_delta(num(required(args, "capital_allocated")?)?,
        num(required(args, "delivered_utility")?)?, boolean(args, "inverted", false)?)
        .map_err(|error| format!("Malfeasance delta: {error:?}"))?;
    if !delta.delta.is_finite() || !divergence.propagated_cost.is_finite() || !divergence.true_vs_narrative.is_finite() {
        return Err("Forensic arithmetic exceeded finite numeric range".into());
    }
    audit.receipt.fact(AUDIT, "urn:qualia:research:forensicEvidence", evidence)?;
    audit.receipt.fact(AUDIT, "urn:qualia:research:computedMalfeasanceDelta", &format!("urn:f64:{:016x}", delta.delta.to_bits()))?;
    audit.receipt.fact(AUDIT, "urn:qualia:research:computedNarrativeCost", &format!("urn:f64:{:016x}", divergence.propagated_cost.to_bits()))?;
    audit.output.extend([
        ("calculated_delta".into(), Value::F64(delta.delta)),
        ("propagated_cost".into(), Value::F64(divergence.propagated_cost)),
        ("maintenance_indicator".into(), Value::Bool(divergence.maintenance_active)),
        ("narrative_divergence".into(), Value::List(divergence.true_vs_narrative.to_array().into_iter().map(Value::F64).collect())),
    ]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn grounding_reports_only_exact_claims_in_the_selected_graph() {
        const CONTEXT: &str = "urn:test:research:context";
        const S: &str = "urn:test:research:subject";
        const P: &str = "urn:test:research:predicate";
        const O: &str = "urn:test:research:object";
        let mut source = ReceiptGraph::new(CONTEXT);
        source.fact(S, P, O).unwrap();
        let limits = crate::ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost { workspace: Default::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let model = host.admit_encoded("q42", source.seal(SYMBOLIC_LIMIT).unwrap()).unwrap();
        let claim = |object: &str| Value::Record(Args::from([
            ("subject_iri".into(), Value::String(S.into())),
            ("predicate_iri".into(), Value::String(P.into())),
            ("object_iri".into(), Value::String(object.into())),
        ]));
        let args = Args::from([
            ("model".into(), model), ("context_iri".into(), Value::String(CONTEXT.into())),
            ("task".into(), Value::String("verify_hybrid_graph_grounding".into())),
            ("parameters".into(), Value::Record(Args::from([
                ("claims".into(), Value::List(vec![claim(O), claim("urn:test:research:missing")])),
            ]))),
        ]);
        let result = host.audit_research_and_grounding(&args).unwrap();
        let report = record(&result).unwrap();
        assert!(matches!(list(&report["graph_supported"]).unwrap(), [Value::Bool(true), Value::Bool(false)]));
        assert_eq!(list(&report["grounded_nquins"]).unwrap().len(), 1);
        assert!(host.workspace.encoded(&report["report"]).is_ok());
    }
}

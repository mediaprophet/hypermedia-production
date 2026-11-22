//! Bounded corpus novelty over QBDL package-root RDF and standalone Q42 graphs.

use super::{external_resource, fields, required, text, Args, AssetHost, ReceiptGraph, Value,
    GRAPH, SYMBOLIC_LIMIT};
use crate::{artifact::HmcWorkspaceView, list, record, TypedArtifact};
use qualia_core_db::{
    bundle::{BundleReadLimits, BundleReader, semantic_model::{
        open_semantic_model, SemanticModelBuilder, SEMANTIC_MODEL_KEY,
    }},
    q_hash,
};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, sync::atomic::Ordering};

const MAX_DOCUMENTS: usize = 64;
const MAX_DOCUMENT_FACTS: usize = 8192;
const MAX_CORPUS_FACTS: usize = 131_072;
const HAS_ENTRY: u64 = q_hash("https://qualiadb.org/ontology/hmc#hasEntry");
const RDF_TYPE: u64 = q_hash("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
const PACKAGE: u64 = q_hash("https://qualiadb.org/ontology/hmc#Package");
const HMC_GRAPH: u64 = q_hash("https://qualiadb.org/ontology/hmc#SemanticModel");

type Triple = [u64; 3];

struct Document {
    key: String,
    digest: [u8; 32],
    facts: BTreeSet<Triple>,
}

struct Ranking<'a> {
    key: &'a str,
    nearest: Option<&'a str>,
    duplicate_of: Option<&'a str>,
    novelty: f64,
}

pub(super) fn rank(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["documents", "known"])?;
    let docs = list(required(params, "documents")?)?;
    let known = params.get("known").map(list).transpose()?.unwrap_or(&[]);
    if docs.is_empty() || docs.len() + known.len() > MAX_DOCUMENTS {
        return Err("Corpus must contain documents and fit the semantic corpus count limit".into());
    }
    let limits = BundleReadLimits {
        file_bytes: host.limits.resource_bytes,
        entries: host.limits.resources,
        ..BundleReadLimits::default()
    };
    let budget = host.limits.resource_bytes.min(SYMBOLIC_LIMIT);
    let mut keys = BTreeSet::new();
    let mut source_bytes = 0usize;
    let mut corpus_facts = 0usize;
    let mut read_documents = |rows: &[Value]| -> Result<Vec<Document>, String> {
        let mut documents = Vec::with_capacity(rows.len());
        for value in rows {
            if host.cancel.load(Ordering::Relaxed) {
                return Err("Production cancelled".into());
            }
            let row = record(value)?;
            fields(row, &["key", "artifact"])?;
            let key = text(required(row, "key")?)?;
            if key.is_empty() || key.len() > 1024 || !keys.insert(key.to_string()) {
                return Err("Corpus keys must be bounded and unique".into());
            }
            let artifact = required(row, "artifact")?;
            let (kind, bytes) = artifact_bytes(host, artifact)?;
            source_bytes = source_bytes.checked_add(bytes.len()).ok_or("Corpus size overflows")?;
            if source_bytes > host.limits.total_resource_bytes
                || (host.work.saturating_add(source_bytes as u64) > host.limits.asset_work_units)
            {
                return Err("Corpus exceeds production input or work budget".into());
            }
            let facts = semantic_facts(kind, bytes, limits, budget)?;
            corpus_facts += facts.len();
            if corpus_facts > MAX_CORPUS_FACTS {
                return Err("Corpus fact count exceeds comparison budget".into());
            }
            documents.push(Document { key: key.to_owned(), digest: Sha256::digest(bytes).into(), facts });
        }
        Ok(documents)
    };
    let mut documents = read_documents(docs)?;
    let mut baselines = read_documents(known)?;
    baselines.sort_unstable_by(|a, b| a.key.cmp(&b.key));
    documents.sort_unstable_by(|a, b| a.key.cmp(&b.key));
    let comparison_work = corpus_facts.saturating_mul(documents.len() + baselines.len());
    host.charge(source_bytes.saturating_add(comparison_work), budget)?;
    let rows = compare(&documents, &baselines, host.cancel)?;
    let mut graph = ReceiptGraph::new(GRAPH);
    let mut report = Vec::with_capacity(rows.len());
    for row in rows {
        let subject = format!("urn:qualia:corpus:{}", hex_key(row.key));
        graph.fact(&subject, "https://qualiadb.org/ontology/q42#noveltyMetric",
            "urn:qualia:corpus:triple-jaccard")?;
        if let Some(key) = row.duplicate_of {
            graph.fact(&subject, "https://qualiadb.org/ontology/q42#duplicateOf",
                &format!("urn:qualia:corpus:{}", hex_key(key)))?;
        }
        let mut values = BTreeMap::from([
            ("key".into(), Value::String(row.key.into())),
            ("novelty".into(), Value::F64(row.novelty)),
        ]);
        if let Some(key) = row.nearest {
            values.insert("nearest".into(), Value::String(key.into()));
        }
        if let Some(key) = row.duplicate_of {
            values.insert("duplicate_of".into(), Value::String(key.into()));
        }
        report.push(Value::Record(values));
    }
    let receipt = graph.seal(budget)?;
    Ok(Value::Record(BTreeMap::from([
        ("ranking".into(), Value::List(report)),
        ("receipt".into(), host.encode("q42", receipt)?),
    ])))
}

fn hex_key(key: &str) -> String {
    format!("{:x}", Sha256::digest(key.as_bytes()))
}

fn artifact_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<(&'a str, &'a [u8]), String> {
    if let Ok(TypedArtifact::HmcBundle(view)) = host.workspace.typed(value) {
        return match view.as_ref() {
            HmcWorkspaceView::Sealed(bytes) => Ok(("hmc", bytes)),
            HmcWorkspaceView::Staged(_) => Err("Corpus HMC input must be an admitted sealed bundle".into()),
        };
    }
    let resource = external_resource(host, value)?;
    Ok((&resource.kind, &resource.bytes))
}

fn semantic_facts(
    kind: &str, bytes: &[u8], limits: BundleReadLimits, budget: usize,
) -> Result<BTreeSet<Triple>, String> {
    let view;
    let embedded = match kind {
        "q42" => bytes,
        "hmc" => {
            view = BundleReader::parse_with_limits(bytes, limits).map_err(|error| error.to_string())?;
            open_semantic_model(&view).map_err(|error| error.to_string())?;
            view.get(SEMANTIC_MODEL_KEY).or_else(|| view.get("manifest.q42"))
                .ok_or("Corpus HMC has no semantic model")?
        }
        _ => return Err("Corpus accepts sealed HMC or Q42 resources".into()),
    };
    let model = SemanticModelBuilder::new(budget).merge_q42(embedded).map_err(|error| error.to_string())?;
    let rows = model.snapshot_quins();
    if rows.len() > MAX_DOCUMENT_FACTS {
        return Err("Corpus document exceeds the per-document fact budget".into());
    }
    Ok(rows.iter().filter(|row| {
        !(row.context == HMC_GRAPH && (row.predicate == HAS_ENTRY
            || (row.predicate == RDF_TYPE && row.object == PACKAGE)))
    }).map(|row| [row.subject, row.predicate, row.object]).collect())
}

fn compare<'a>(
    documents: &'a [Document],
    known: &'a [Document],
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<Vec<Ranking<'a>>, String> {
    let mut report = Vec::with_capacity(documents.len());
    for (index, row) in documents.iter().enumerate() {
        if cancel.load(Ordering::Relaxed) {
            return Err("Production cancelled".into());
        }
        let mut similarity = 0.;
        let mut nearest = None;
        let mut duplicate_of = None;
        for other in known.iter().chain(&documents[..index]) {
            let key = other.key.as_str();
            let intersection = row.facts.intersection(&other.facts).count();
            let union = row.facts.len() + other.facts.len() - intersection;
            let exact = row.digest == other.digest || (union != 0 && intersection == union);
            let current = if exact { 1. } else if union == 0 { 0. } else {
                intersection as f64 / union as f64
            };
            if exact && duplicate_of.is_none_or(|old| key < old) {
                duplicate_of = Some(key);
            }
            if nearest.is_none_or(|old| current > similarity || (current == similarity && key < old)) {
                nearest = Some(key);
                similarity = current;
            }
        }
        report.push(Ranking { key: &row.key, nearest, duplicate_of, novelty: 1. - similarity });
    }
    report.sort_unstable_by(|a, b| b.novelty.total_cmp(&a.novelty).then_with(|| a.key.cmp(b.key)));
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;

    #[test]
    fn structural_comparison_exposes_duplicate_and_highest_novelty() {
        let documents = [
            Document { key:"a".into(), digest:[1; 32], facts:BTreeSet::from([[1, 2, 3]]) },
            Document { key:"b".into(), digest:[2; 32], facts:BTreeSet::from([[1, 2, 3]]) },
            Document { key:"c".into(), digest:[3; 32], facts:BTreeSet::from([[4, 5, 6]]) },
        ];
        let report = compare(&documents, &[], &AtomicBool::new(false)).unwrap();
        assert_eq!(report[0].key, "a");
        assert_eq!(report[1].key, "c");
        assert_eq!(report[2].duplicate_of, Some("a"));
        assert_eq!(report[2].novelty, 0.);
    }
}

//! Asset production adapters for the QualiaDB Category A expansion.

use super::{fields, required, text, Args, AssetHost, Resource, Value};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::{encode_symbolic_rdf_cbor_ld, MultilingualLexicalTerm, SymbolicRdfCborLdPayload},
    q_hash, NQuin,
};
use std::collections::BTreeMap;

mod cas;
mod epistemics;
mod geospatial;
mod graph_decode;
mod journal;
mod live;
mod modalities;
mod network;
mod statistics;

const SYMBOLIC_LIMIT: usize = 42 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn solve_symbolic_cas_and_ode(&mut self, args: &Args) -> Result<Value, String> {
        let operation = text(required(args, "op")?)?;
        let parameters = super::record(required(args, "parameters")?)?;
        fields(args, &["op", "parameters"])?;
        let result = if operation == "from_quins" {
            fields(parameters, &["model", "citation_hash"])?;
            let resource = external_resource(self, required(parameters, "model")?)?;
            if resource.bytes.len() > SYMBOLIC_LIMIT {
                return Err("CAS input exceeds the symbolic byte budget".into());
            }
            let size = resource.bytes.len();
            self.charge(size, cas::READ_SCRATCH)?;
            let resource = external_resource(self, required(parameters, "model")?)?;
            cas::load(resource, exact_u64(required(parameters, "citation_hash")?)?)?
        } else {
            self.charge(cas::work(operation), 0)?;
            cas::evaluate(operation, parameters)?
        };
        let budget = self.limits.resource_bytes.min(SYMBOLIC_LIMIT);
        let (mut values, model) = cas::output(result, budget)?;
        if let Some(model) = model {
            values.insert("model".into(), self.encode("q42", model)?);
        }
        Ok(Value::Record(values))
    }
}

fn external_resource<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host.inputs.iter().find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn exact_u64(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(n) => Ok(*n),
        Value::I64(n) => u64::try_from(*n).map_err(|_| "Expected unsigned integer".into()),
        _ => Err("Expected unsigned integer".into()),
    }
}

/// A cold authoring graph whose terms can be resolved in both directions in Q42LEX.
pub(crate) struct ReceiptGraph {
    terms: BTreeMap<u64, String>,
    data: SymbolicRdfCborLdPayload,
}

impl ReceiptGraph {
    pub(crate) fn new(context: &str) -> Self {
        Self {
            terms: BTreeMap::new(),
            data: SymbolicRdfCborLdPayload { context_iri: context.into(), ..Default::default() },
        }
    }

    pub(crate) fn fact(&mut self, subject: &str, predicate: &str, object: &str) -> Result<(), String> {
        let terms = [subject, predicate, object, self.data.context_iri.as_str()];
        let [s, p, o, c] = terms.map(q_hash);
        for (term, hash) in terms.into_iter().zip([s, p, o, c]) {
            match self.terms.entry(hash) {
                std::collections::btree_map::Entry::Vacant(entry) => { entry.insert(term.into()); }
                std::collections::btree_map::Entry::Occupied(entry) if entry.get() == term => {}
                _ => return Err("Q42 receipt contains colliding terms".into()),
            }
        }
        self.data.nquins.push([s, p, o, c, 0, NQuin::calculate_parity(s, p, o, c, 0)]);
        Ok(())
    }

    pub(crate) fn seal(mut self, budget: usize) -> Result<Vec<u8>, String> {
        self.data.lexicon = self.terms.into_iter().map(|(term_hash, iri_utf8)| MultilingualLexicalTerm {
            term_hash, iri_utf8, lang_bcp47: None, label_utf8: String::new(),
        }).collect();
        let cbld = encode_symbolic_rdf_cbor_ld(&self.data).map_err(|e| e.to_string())?;
        SemanticModelBuilder::new(budget.min(SYMBOLIC_LIMIT)).merge_cbld(&cbld)
            .and_then(SemanticModelBuilder::finish).map_err(|e| e.to_string())
    }
}

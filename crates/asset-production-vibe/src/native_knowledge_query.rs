//! Bounded production evaluation over retained Q42 and HMC semantic models.

use std::{collections::BTreeMap, sync::atomic::Ordering};

use qualia_core_db::{
    NQuin,
    bundle::{
        BundleReadLimits, BundleReader,
        semantic_model::{SEMANTIC_MODEL_KEY, open_semantic_model},
    },
    modalities::{
        asp, causal,
        logic::n3_parser::{N3Parser, N3ParserError, RuleType, StackEvent, Term},
        paraconsistent::Belnap,
        spatio_temporal::{self, Rcc8Relation},
        temporal_ltl::{self, LtlFormula},
    },
    q_hash,
    q42_volume::{
        MAX_COMPRESSED_SUPERBLOCK_SIZE, OpfsSliceRangeSource, Q42RangeQueryCursor,
        Q42RangeQueryPattern, Q42RangeQueryPlan, Q42RangeVolume, SUPERBLOCK_SIZE,
        WasmSliceQ42Reader,
    },
    shacl_compiler::{ShaclConstraint, validate_shacl_property},
    sparql_ast::{BindingRow, Pattern, SparqlQuery},
    sparql_executor::{
        Q42RangeTripleSelectCursor, Q42RangeTripleSelectPlan, execute_range_triple_select_page_into,
    },
    sparql_parser::parse_sparql,
    sparql_planner::QueryPlanner,
};

use crate::{Args, AssetHost, Resource, Value, fields, list, num, record, required, text};

const SENTINEL: usize = 42 * 1024 * 1024;
const MAX_FACTS: usize = 256;
const MAX_ROWS: usize = 256;
const LEX_PAGE_BYTES: usize = 256 * 1024;
const MAX_TERM_BYTES: usize = 16 * 1024;
const SCRATCH: usize = MAX_COMPRESSED_SUPERBLOCK_SIZE
    + SUPERBLOCK_SIZE
    + LEX_PAGE_BYTES
    + MAX_TERM_BYTES
    + 2 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn query_q42_sparql_and_n3(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "mode", "source", "parameters"])?;
        let model = required(args, "model")?;
        let mode = text(required(args, "mode")?)?;
        let params = record(required(args, "parameters")?)?;
        let input_bytes = retained(self, model)?.bytes.len();
        self.charge(
            input_bytes.saturating_mul(2).saturating_add(2_000_000),
            SCRATCH,
        )?;
        let bytes = model_bytes(self, model)?;
        match mode {
            "sparql" => {
                fields(params, &["max_rows"])?;
                let source = text(required(args, "source")?)?;
                let max_rows = params
                    .get("max_rows")
                    .map(cardinality)
                    .transpose()?
                    .unwrap_or(MAX_ROWS as u32) as usize;
                if !(1..=MAX_ROWS).contains(&max_rows) {
                    return Err("SPARQL max_rows must be in 1..=256".into());
                }
                sparql(bytes, source, max_rows, self.cancel)
            }
            "shacl_property" => {
                if args.contains_key("source") {
                    return Err("shacl_property takes constraints in parameters".into());
                }
                shacl_property(&mut Model::new(bytes)?, params)
            }
            "n3" => {
                let source = text(required(args, "source")?)?;
                n3(&mut Model::new(bytes)?, source, params)
            }
            _ => Err(format!("Unsupported bounded Q42 query mode {mode}")),
        }
    }

    pub(crate) fn solve_causal_temporal_rcc8(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "mode", "parameters"])?;
        let model = required(args, "model")?;
        let mode = text(required(args, "mode")?)?;
        let params = record(required(args, "parameters")?)?;
        let input_bytes = retained(self, model)?.bytes.len();
        let mut work = 2_000_000usize;
        if mode == "asp_paraconsistent" {
            let atoms = list(required(params, "atoms")?)?.len();
            let rules = list(required(params, "rules")?)?.len();
            if !(1..=asp::ASP_MAX_ATOMS).contains(&atoms) || rules > 64 {
                return Err("ASP admits 1–12 atoms and at most 64 authored rules".into());
            }
            work += (1usize << atoms)
                * (atoms + 1)
                * (rules + 2 * atoms)
                * (2 * asp::ASP_MAX_BODY + 1)
                * (atoms + 1);
        }
        self.charge(input_bytes.saturating_mul(2).saturating_add(work), SCRATCH)?;
        let mut model = Model::new(model_bytes(self, model)?)?;
        let result = match mode {
            "pearl_do_intervention" => causal_do(&mut model, params),
            "ltl_model_check" => ltl(&mut model, params),
            "rcc8_spatial_consistency" => rcc8(&mut model, params),
            "asp_paraconsistent" => asp_paraconsistent(&mut model, params),
            _ => return Err(format!("Unknown causal/temporal/spatial mode {mode}")),
        }?;
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        Ok(result)
    }
}

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn model_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    let source = retained(host, value)?;
    if source.bytes.len() > host.limits.resource_bytes {
        return Err("Q42/HMC input exceeds the retained resource budget".into());
    }
    let bytes = match source.kind.as_str() {
        "q42" => source.bytes.as_slice(),
        "hmc" => {
            let bundle = BundleReader::parse_with_limits(
                &source.bytes,
                BundleReadLimits {
                    file_bytes: host.limits.resource_bytes,
                    entries: host.limits.resources,
                    ..BundleReadLimits::default()
                },
            )
            .map_err(|error| format!("HMC admission: {error}"))?;
            open_semantic_model(&bundle).map_err(|error| error.to_string())?;
            let key = if bundle.entry(SEMANTIC_MODEL_KEY).is_some() {
                SEMANTIC_MODEL_KEY
            } else {
                "manifest.q42"
            };
            if !bundle.verify_entry(key) {
                return Err("HMC Q42 digest mismatch".into());
            }
            bundle.get(key).ok_or("HMC has no primary Q42 bytes")?
        }
        _ => return Err("Knowledge input must be a retained Q42 volume or sealed HMC".into()),
    };
    if bytes.len() > SENTINEL - SCRATCH {
        return Err("Q42 volume and bounded query storage exceed the 42 MiB Sentinel".into());
    }
    Ok(bytes)
}

struct Model<'a> {
    reader: WasmSliceQ42Reader<'a>,
    compressed: Vec<u8>,
    decoded: Vec<u8>,
    lex_page: Vec<u8>,
    lexical: Vec<u8>,
}

impl<'a> Model<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self, String> {
        let reader = WasmSliceQ42Reader::open(bytes).map_err(|error| error.to_string())?;
        if !reader.is_unified_volume() {
            return Err("Knowledge evaluation requires a unified Q42 volume".into());
        }
        Ok(Self {
            reader,
            compressed: vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE],
            decoded: vec![0; SUPERBLOCK_SIZE],
            lex_page: vec![0; LEX_PAGE_BYTES],
            lexical: vec![0; MAX_TERM_BYTES],
        })
    }

    fn term(&mut self, value: &Value) -> Result<u64, String> {
        self.iri(text(value)?)
    }

    fn iri(&mut self, iri: &str) -> Result<u64, String> {
        let (scheme, local) = iri
            .split_once(':')
            .ok_or("Expected a canonical absolute RDF IRI")?;
        if scheme.is_empty()
            || !scheme.as_bytes()[0].is_ascii_alphabetic()
            || !scheme
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || b"+-.".contains(&ch))
            || local.is_empty()
            || local.chars().any(|ch| {
                ch.is_whitespace()
                    || ch.is_control()
                    || "<>\"{}|\\^".contains(ch)
                    || ch == char::from(0x60)
            })
        {
            return Err("Expected a canonical absolute RDF IRI".into());
        }
        let hash = q_hash(iri);
        let len = self
            .reader
            .lookup_lexicon_hash_into(hash, &mut self.lex_page, &mut self.lexical)
            .map_err(|error| error.to_string())?
            .ok_or("IRI is absent from admitted Q42LEX")?;
        if hash == 0 || self.lexical.get(..len) != Some(iri.as_bytes()) {
            return Err("Q42LEX IRI/hash mismatch".into());
        }
        Ok(hash)
    }

    fn named(&mut self, args: &Args, key: &str) -> Result<u64, String> {
        self.term(required(args, key)?)
    }

    fn facts(&mut self, pattern: Q42RangeQueryPattern, out: &mut [NQuin]) -> Result<usize, String> {
        let page = self
            .reader
            .execute_query_page_into(
                Q42RangeQueryPlan::for_pattern(pattern),
                Q42RangeQueryCursor::default(),
                &mut self.compressed,
                &mut self.decoded,
                out,
            )
            .map_err(|error| error.to_string())?;
        if page.next_cursor.is_some() || page.returned == out.len() {
            return Err("Selected Q42 evidence exceeds the fixed fact budget".into());
        }
        if out[..page.returned]
            .iter()
            .any(|quin| !quin.verify_ecc_parity())
        {
            return Err("Q42 evidence failed NQuin parity".into());
        }
        Ok(page.returned)
    }

    fn require_rule_source(&mut self, scope: u64, source: &str) -> Result<(), String> {
        let literal = serde_json::to_string(source).map_err(|error| error.to_string())?;
        let hash = q_hash(&literal);
        let count = self
            .reader
            .lookup_lexicon_hash_into(hash, &mut self.lex_page, &mut self.lexical)
            .map_err(|error| error.to_string())?
            .ok_or("N3 source is absent from Q42LEX")?;
        if hash == 0 || self.lexical.get(..count) != Some(literal.as_bytes()) {
            return Err("Q42LEX N3 rule source hash mismatch".into());
        }
        let mut fact = [NQuin::default(); 2];
        if self.facts(
            Q42RangeQueryPattern {
                subject: Some(scope),
                predicate: Some(q_hash("https://qualiadb.org/ontology/hmc#n3RuleSource")),
                object: Some(hash),
                context: Some(q_hash("https://qualiadb.org/ontology/hmc#SemanticModel")),
            },
            &mut fact,
        )? != 1
        {
            return Err(
                "The designated Q42 scope does not contain exactly one matching N3 source".into(),
            );
        }
        Ok(())
    }
}

fn result(proof: Args, derived: &[NQuin]) -> Value {
    Value::Record(BTreeMap::from([
        ("proof".into(), Value::Record(proof)),
        (
            "derived_nquins".into(),
            Value::List(
                derived
                    .iter()
                    .map(|q| {
                        Value::List(
                            [
                                q.subject,
                                q.predicate,
                                q.object,
                                q.context,
                                q.metadata,
                                q.parity,
                            ]
                            .into_iter()
                            .map(Value::U64)
                            .collect(),
                        )
                    })
                    .collect(),
            ),
        ),
    ]))
}

fn sparql(
    bytes: &[u8],
    source: &str,
    max_rows: usize,
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<Value, String> {
    if source.len() > 4096 {
        return Err("SPARQL source exceeds 4096 bytes".into());
    }
    check_sparql_source(source)?;
    let (parsed, ctx) = parse_sparql(source)?;
    if ctx.patterns[..ctx.pattern_count]
        .iter()
        .any(|pattern| !matches!(pattern, Pattern::Triple { .. } | Pattern::Group { .. }))
    {
        return Err("Bounded SPARQL requires one ordinary triple pattern".into());
    }
    let ask = match &parsed {
        SparqlQuery::Ask(_) => true,
        SparqlQuery::Select(_) => false,
        _ => return Err("Bounded native SPARQL accepts SELECT and ASK".into()),
    };
    let physical = QueryPlanner::plan(&parsed, &ctx)?;
    let mut plan = Q42RangeTripleSelectPlan::from_execution_plan(&physical)?;
    if plan.filter_count != 0 {
        return Err(
            "Bounded native SPARQL supports one triple pattern with projection and LIMIT/OFFSET"
                .into(),
        );
    }
    plan.limit = plan.limit.min(if ask { 1 } else { max_rows as u64 + 1 });
    let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(bytes))
        .map_err(|error| error.to_string())?;
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decoded = vec![0; SUPERBLOCK_SIZE];
    let mut facts = [NQuin::default(); 32];
    let mut rows = [BindingRow::default(); 32];
    let mut cursor = Q42RangeTripleSelectCursor::default();
    let mut bindings = Vec::new();
    let mut found = false;
    loop {
        if cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        let page = execute_range_triple_select_page_into(
            &volume,
            plan,
            &ctx,
            cursor,
            &mut compressed,
            &mut decoded,
            &mut facts,
            &mut rows,
        )?;
        if ask {
            found |= page.returned != 0;
        } else {
            if bindings.len() + page.returned > max_rows {
                return Err("SPARQL bindings exceed max_rows; use a bounded LIMIT".into());
            }
            bindings.extend(rows[..page.returned].iter().map(|row| {
                Value::List(
                    row.slots
                        .iter()
                        .enumerate()
                        .filter_map(|(slot, value)| {
                            value.map(|hash| {
                                Value::Record(BTreeMap::from([
                                    ("variable".into(), Value::U64(ctx.variable_hashes[slot])),
                                    ("value".into(), Value::U64(hash)),
                                ]))
                            })
                        })
                        .collect(),
                )
            }));
        }
        match page.next_cursor {
            Some(next) if next != cursor => cursor = next,
            Some(_) => return Err("Native SPARQL cursor did not advance".into()),
            None => break,
        }
    }
    Ok(Value::Record(BTreeMap::from([
        ("bindings".into(), Value::List(bindings)),
        (
            "ask".into(),
            if ask { Value::Bool(found) } else { Value::Null },
        ),
        ("derived_nquins".into(), Value::List(Vec::new())),
    ])))
}

fn check_sparql_source(source: &str) -> Result<(), String> {
    let mut query = source.trim();
    while let Some(prefix) = query.strip_prefix("PREFIX ") {
        let (name, declaration) = prefix.split_once('<').ok_or("Expected a PREFIX IRI")?;
        let (_, rest) = declaration
            .split_once('>')
            .ok_or("Expected a closed PREFIX IRI")?;
        let name = name.trim();
        if !name.ends_with(':') || name.split_whitespace().count() != 1 {
            return Err("Invalid SPARQL PREFIX declaration".into());
        }
        query = rest.trim_start();
    }
    let (heading, clause) = query
        .split_once('{')
        .ok_or("SPARQL WHERE block is missing")?;
    let mut adornment = heading.split_whitespace();
    let form = adornment.next().ok_or("Expected SELECT or ASK")?;
    let where_tokens: Vec<_> = adornment.collect();
    let (where_token, projection) = where_tokens.split_last().ok_or("Expected WHERE")?;
    if *where_token != "WHERE" {
        return Err("Expected a WHERE clause with a single explicit triple pattern".into());
    }
    match form {
        "ASK" if projection.is_empty() => {}
        "SELECT"
            if projection == ["*"]
                || (!projection.is_empty()
                    && projection
                        .iter()
                        .all(|token| token.starts_with('?') && token.len() > 1)) => {}
        _ => {
            return Err("SPARQL supports SELECT variables or ASK over one admitted dataset".into());
        }
    }
    let (pattern, tail) = clause
        .rsplit_once('}')
        .ok_or("SPARQL WHERE block is not closed")?;
    if pattern.trim().is_empty() {
        return Err("SPARQL triple pattern is empty".into());
    }
    let mut options = tail.split_whitespace();
    let mut seen_limit = false;
    let mut seen_offset = false;
    while let Some(option) = options.next() {
        match option {
            "LIMIT" if form == "SELECT" && !seen_limit => seen_limit = true,
            "OFFSET" if form == "SELECT" && !seen_offset => seen_offset = true,
            _ => return Err("SPARQL only supports LIMIT and OFFSET after the WHERE block".into()),
        }
        options
            .next()
            .and_then(|n| n.parse::<u64>().ok())
            .ok_or("SPARQL LIMIT/OFFSET requires a nonnegative integer")?;
    }
    Ok(())
}

fn cardinality(value: &Value) -> Result<u32, String> {
    match value {
        Value::U64(n) => u32::try_from(*n),
        Value::I64(n) => u32::try_from(*n),
        _ => return Err("Expected an exact nonnegative integer".into()),
    }
    .map_err(|_| "Integer is outside the u32 range".into())
}

fn shacl_property(model: &mut Model<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &["context", "focus", "path", "min_count", "max_count"],
    )?;
    let context = model.named(params, "context")?;
    let focus = model.named(params, "focus")?;
    let path = model.named(params, "path")?;
    let min = params.get("min_count").map(cardinality).transpose()?;
    let max = params.get("max_count").map(cardinality).transpose()?;
    if min.is_none() && max.is_none() || min.zip(max).is_some_and(|(min, max)| min > max) {
        return Err("SHACL requires a nonempty consistent cardinality constraint".into());
    }
    let constraints = [
        ShaclConstraint::MinCount(min.unwrap_or(0)),
        ShaclConstraint::MaxCount(max.unwrap_or(u32::MAX)),
    ];
    let mut facts = [NQuin::default(); MAX_FACTS + 1];
    let count = model.facts(
        Q42RangeQueryPattern {
            subject: Some(focus),
            predicate: Some(path),
            context: Some(context),
            object: None,
        },
        &mut facts,
    )?;
    let set = &mut facts[..count];
    set.sort_unstable_by_key(|fact| fact.object);
    let mut unique = 0;
    for read in 0..set.len() {
        if unique == 0 || set[read].object != set[unique - 1].object {
            set[unique] = set[read];
            unique += 1;
        }
    }
    let conforms = validate_shacl_property(&set[..unique], focus, path, &constraints);
    Ok(Value::Record(BTreeMap::from([
        ("bindings".into(), Value::List(Vec::new())),
        ("derived_nquins".into(), Value::List(Vec::new())),
        (
            "shacl_report".into(),
            Value::Record(BTreeMap::from([
                ("conforms".into(), Value::Bool(conforms)),
                ("focus".into(), Value::U64(focus)),
                ("path".into(), Value::U64(path)),
                ("value_count".into(), Value::U64(unique as u64)),
            ])),
        ),
    ])))
}

#[derive(Clone, Copy)]
enum RuleSlot<'a> {
    Iri(u64),
    Variable(&'a str),
}

impl RuleSlot<'_> {
    fn constant(self) -> Option<u64> {
        match self {
            Self::Iri(hash) => Some(hash),
            Self::Variable(_) => None,
        }
    }
}

fn n3(model: &mut Model<'_>, source: &str, params: &Args) -> Result<Value, String> {
    use std::fmt::Write;

    fields(params, &["context", "rule_scope"])?;
    let context = model.named(params, "context")?;
    if source.len() > 4096 {
        return Err("N3 source exceeds 4096 bytes".into());
    }
    if let Some(scope) = params.get("rule_scope") {
        let scope = model.term(scope)?;
        model.require_rule_source(scope, source)?;
    }
    let strict = source
        .trim()
        .strip_suffix('.')
        .ok_or("N3 implication must end in a period")?;
    let (premise, conclusion) = strict
        .split_once("=>")
        .ok_or("Expected a strict N3 implication")?;
    let mut constants = [None; 6];
    let mut normalized = String::with_capacity(source.len().min(1024) + 192);
    for (side, formula) in [premise, conclusion].into_iter().enumerate() {
        let body = formula
            .trim()
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .ok_or("N3 implication requires explicitly braced formulae")?
            .trim();
        let body = body.strip_suffix('.').map_or(body, str::trim_end);
        if body.split_whitespace().count() != 3 {
            return Err(
                "N3 bounded implication requires exactly one ordinary triple on each side".into(),
            );
        }
        if side != 0 {
            normalized.push_str(" => ");
        }
        normalized.push_str("{ ");
        for (column, token) in body.split_whitespace().enumerate() {
            if let Some(name) = token.strip_prefix('?') {
                if name.is_empty() || !name.chars().all(|ch| ch.is_alphanumeric() || ch == '_') {
                    return Err("N3 bounded implication requires ordinary variable names".into());
                }
                normalized.push_str(token);
            } else {
                let iri = token
                    .strip_prefix('<')
                    .and_then(|s| s.strip_suffix('>'))
                    .unwrap_or(token);
                let slot = side * 3 + column;
                constants[slot] = Some(model.iri(iri)?);
                // The native rule tokenizer reserves periods outside numeric literals.
                // Preserve the original canonical IRI separately from its grammar token.
                write!(normalized, "urn:qualia:n3-slot:{slot}")
                    .map_err(|error| error.to_string())?;
            }
            normalized.push(' ');
        }
        normalized.push('}');
    }
    normalized.push_str(" .");
    let mut rule = None;
    N3Parser::new(&normalized)
        .parse_all_zero_heap(|event| match event {
            StackEvent::LogicRule(candidate)
                if rule.is_none()
                    && candidate.rule_type == RuleType::Strict
                    && candidate.premise.len == 1
                    && candidate.conclusion.len == 1 =>
            {
                rule = Some(candidate);
                Ok(())
            }
            _ => Err(N3ParserError(
                "Expected one strict N3 single-triple implication",
            )),
        })
        .map_err(|error| error.to_string())?;
    let rule = rule.ok_or("No strict N3 implication present")?;
    let mut slots = [[RuleSlot::Iri(0); 3]; 2];
    for (triple, out) in [rule.premise.triples[0], rule.conclusion.triples[0]]
        .into_iter()
        .zip(&mut slots)
    {
        for (term, out) in [triple.subject, triple.predicate, triple.object]
            .into_iter()
            .zip(out.iter_mut())
        {
            *out = match term {
                Term::Uri(iri) => {
                    let index = iri
                        .strip_prefix("urn:qualia:n3-slot:")
                        .and_then(|id| id.parse::<usize>().ok())
                        .ok_or("Invalid normalized N3 constant")?;
                    RuleSlot::Iri(
                        constants
                            .get(index)
                            .copied()
                            .flatten()
                            .ok_or("Missing N3 constant")?,
                    )
                }
                Term::Variable(name) if name.len() > 1 => RuleSlot::Variable(name),
                _ => return Err("N3 bounded implication accepts IRIs and variables only".into()),
            };
        }
        if !matches!(out[1], RuleSlot::Iri(_)) {
            return Err("N3 bounded implication requires constant predicate IRIs".into());
        }
    }
    let [body, head] = slots;
    if head.iter().any(|term| {
        matches!(term, RuleSlot::Variable(name)
        if !body.iter().any(|input| matches!(input, RuleSlot::Variable(bound) if name == bound)))
    }) {
        return Err("N3 conclusion has an unbound variable".into());
    }
    if matches!(head[0], RuleSlot::Variable(name)
        if !matches!(body[0], RuleSlot::Variable(subject) if name == subject))
    {
        return Err(
            "N3 bounded implication requires subject bindings from premise subjects".into(),
        );
    }
    let mut facts = [NQuin::default(); MAX_FACTS + 1];
    let count = model.facts(
        Q42RangeQueryPattern {
            subject: body[0].constant(),
            predicate: body[1].constant(),
            object: body[2].constant(),
            context: Some(context),
        },
        &mut facts,
    )?;
    let mut derived = [NQuin::default(); MAX_FACTS];
    let mut produced = 0;
    for fact in &facts[..count] {
        let values = [fact.subject, fact.predicate, fact.object];
        if matches!((body[0], body[2]), (RuleSlot::Variable(a), RuleSlot::Variable(b)) if a == b)
            && values[0] != values[2]
        {
            continue;
        }
        let bind = |term| match term {
            RuleSlot::Iri(hash) => Ok(hash),
            RuleSlot::Variable(name) => body
                .iter()
                .position(|term| matches!(term, RuleSlot::Variable(bound) if *bound == name))
                .map(|index| values[index])
                .ok_or("Unbound N3 conclusion variable"),
        };
        let [subject, predicate, object] = [bind(head[0])?, bind(head[1])?, bind(head[2])?];
        let quin = NQuin {
            subject,
            predicate,
            object,
            context,
            metadata: 0,
            parity: NQuin::calculate_parity(subject, predicate, object, context, 0),
        };
        if !derived[..produced].contains(&quin) {
            derived[produced] = quin;
            produced += 1;
        }
    }
    let Value::Record(mut output) = result(
        BTreeMap::from([
            ("matched_facts".into(), Value::U64(count as u64)),
            ("derived_count".into(), Value::U64(produced as u64)),
        ]),
        &derived[..produced],
    ) else {
        return Err("Cannot render N3 result".into());
    };
    output.insert("bindings".into(), Value::List(Vec::new()));
    Ok(Value::Record(output))
}

fn term_list<const N: usize>(
    model: &mut Model<'_>,
    value: Option<&Value>,
    out: &mut [u64; N],
) -> Result<usize, String> {
    let Some(value) = value else { return Ok(0) };
    let values = list(value)?;
    if values.len() > N {
        return Err(format!("Knowledge term list exceeds {N} positions"));
    }
    for (index, value) in values.iter().enumerate() {
        let term = model.term(value)?;
        if out[..index].contains(&term) {
            return Err("Duplicate IRI in a knowledge term list".into());
        }
        out[index] = term;
    }
    Ok(values.len())
}

fn causal_do(model: &mut Model<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &["context", "roots", "set_present", "set_absent", "effect"],
    )?;
    let context = model.named(params, "context")?;
    let effect = model.named(params, "effect")?;
    let mut roots = [0; causal::MAX_CAUSAL_NODES];
    let mut present = [0; causal::MAX_CAUSAL_NODES];
    let mut absent = [0; causal::MAX_CAUSAL_NODES];
    let root_count = term_list(model, Some(required(params, "roots")?), &mut roots)?;
    let present_count = term_list(model, params.get("set_present"), &mut present)?;
    let absent_count = term_list(model, params.get("set_absent"), &mut absent)?;
    let mut edges = [NQuin::default(); MAX_FACTS + 1];
    let count = model.facts(
        Q42RangeQueryPattern {
            predicate: Some(causal::cause_predicate()),
            context: Some(context),
            ..Q42RangeQueryPattern::default()
        },
        &mut edges,
    )?;
    let edges = &edges[..count];
    let (roots, present, absent) = (
        &roots[..root_count],
        &present[..present_count],
        &absent[..absent_count],
    );
    if present.iter().any(|term| absent.contains(term))
        || edges.len() + roots.len() + present.len() > causal::MAX_CAUSAL_NODES
    {
        return Err("Conflicting intervention or causal frontier exceeds the native bound".into());
    }
    check_causal_dag(edges)?;
    Ok(result(
        BTreeMap::from([
            ("context".into(), Value::U64(context)),
            ("effect".into(), Value::U64(effect)),
            (
                "factual".into(),
                Value::Bool(causal::caused(edges, roots, effect)),
            ),
            (
                "intervened".into(),
                Value::Bool(causal::do_intervene(edges, roots, present, absent, effect)),
            ),
            ("edge_count".into(), Value::U64(edges.len() as u64)),
        ]),
        &[],
    ))
}

fn check_causal_dag(edges: &[NQuin]) -> Result<(), String> {
    let mut nodes = [0u64; causal::MAX_CAUSAL_NODES * 2];
    let mut count = 0;
    for node in edges.iter().flat_map(|edge| [edge.subject, edge.object]) {
        if node == 0 {
            return Err("Causal DAG contains an absent node ID".into());
        }
        if !nodes[..count].contains(&node) {
            nodes[count] = node;
            count += 1;
        }
    }
    if count > causal::MAX_CAUSAL_NODES {
        return Err("Causal DAG exceeds the native node bound".into());
    }
    let nodes = &mut nodes[..count];
    nodes.sort_unstable();
    let locate = |id: u64| {
        nodes
            .binary_search(&id)
            .map_err(|_| "Invalid causal DAG endpoint".to_owned())
    };
    let mut indegrees = [0usize; causal::MAX_CAUSAL_NODES];
    for edge in edges {
        indegrees[locate(edge.object)?] += 1;
    }
    for _ in 0..count {
        let next = indegrees[..count]
            .iter()
            .position(|&degree| degree == 0)
            .ok_or("Q42 causeOf evidence contains a cycle")?;
        indegrees[next] = usize::MAX;
        for edge in edges.iter().filter(|edge| edge.subject == nodes[next]) {
            indegrees[locate(edge.object)?] -= 1;
        }
    }
    Ok(())
}

fn ltl(model: &mut Model<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["context", "trace", "formula"])?;
    let context = model.named(params, "context")?;
    let trace = model.named(params, "trace")?;
    let formula_args = record(required(params, "formula")?)?;
    let op = text(required(formula_args, "op")?)?;
    let formula = match op {
        "globally" | "finally" | "next" => {
            fields(formula_args, &["op", "predicate"])?;
            let predicate = model.named(formula_args, "predicate")?;
            match op {
                "globally" => LtlFormula::Globally(predicate),
                "finally" => LtlFormula::Finally(predicate),
                _ => LtlFormula::Next(predicate),
            }
        }
        "until" => {
            fields(formula_args, &["op", "antecedent", "consequent"])?;
            LtlFormula::Until {
                ante: model.named(formula_args, "antecedent")?,
                consequent: model.named(formula_args, "consequent")?,
            }
        }
        "release" => {
            fields(formula_args, &["op", "trigger", "invariant"])?;
            LtlFormula::Release {
                trigger: model.named(formula_args, "trigger")?,
                invariant: model.named(formula_args, "invariant")?,
            }
        }
        _ => return Err(format!("Unsupported finite-trace LTL operator {op}")),
    };
    let mut events = [NQuin::default(); MAX_FACTS + 1];
    let count = model.facts(
        Q42RangeQueryPattern {
            subject: Some(trace),
            context: Some(context),
            ..Q42RangeQueryPattern::default()
        },
        &mut events,
    )?;
    let events = &mut events[..count];
    events.sort_unstable_by_key(|event| event.metadata);
    if events.is_empty()
        || events
            .windows(2)
            .any(|pair| pair[0].metadata == pair[1].metadata)
    {
        return Err(
            "Finite LTL requires a nonempty trace with a unique metadata tick per event".into(),
        );
    }
    Ok(result(
        BTreeMap::from([
            ("trace".into(), Value::U64(trace)),
            ("operator".into(), Value::String(op.into())),
            (
                "holds".into(),
                Value::Bool(temporal_ltl::evaluate_ltl_trace(events, &formula)),
            ),
            ("event_count".into(), Value::U64(count as u64)),
        ]),
        &[],
    ))
}

fn bounds(value: &Value) -> Result<[f64; 4], String> {
    let values = list(value)?;
    let [x0, y0, x1, y1] = values else {
        return Err("RCC8 bounds must be [min_x, min_y, max_x, max_y]".into());
    };
    let bounds = [num(x0)?, num(y0)?, num(x1)?, num(y1)?];
    if bounds.iter().any(|x| !x.is_finite()) || bounds[2] <= bounds[0] || bounds[3] <= bounds[1] {
        return Err(
            "RCC8 bounds require finite positive-area rectangles in one coordinate frame".into(),
        );
    }
    Ok(bounds)
}

fn rcc8(model: &mut Model<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &["context", "region_a", "bounds_a", "region_b", "bounds_b"],
    )?;
    let context = model.named(params, "context")?;
    let a = model.named(params, "region_a")?;
    let b = model.named(params, "region_b")?;
    let ax = bounds(required(params, "bounds_a")?)?;
    let bx = bounds(required(params, "bounds_b")?)?;
    if a == b && ax != bx {
        return Err("One semantic region has incompatible RCC8 bounds".into());
    }
    // Joint ordinal coordinates preserve rectangle topology even when source
    // edges differ by less than the polygon kernel's floating-point tolerance.
    let xs = [ax[0], ax[2], bx[0], bx[2]];
    let ys = [ax[1], ax[3], bx[1], bx[3]];
    let rank =
        |value: f64, axis: &[f64; 4]| axis.iter().filter(|&&edge| edge < value).count() as f64;
    let polygon = |[x0, y0, x1, y1]: [f64; 4]| {
        let (x0, y0, x1, y1) = (rank(x0, &xs), rank(y0, &ys), rank(x1, &xs), rank(y1, &ys));
        [(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
    };
    let relation = spatio_temporal::evaluate_rcc8_points(a, &polygon(ax), b, &polygon(bx));
    let overlap = ax[0].max(bx[0]) < ax[2].min(bx[2]) && ax[1].max(bx[1]) < ax[3].min(bx[3]);
    let relation = match relation {
        Rcc8Relation::Disconnected => "rcc8dc",
        Rcc8Relation::ExternallyConnected if overlap => "rcc8po",
        Rcc8Relation::ExternallyConnected => "rcc8ec",
        Rcc8Relation::PartiallyOverlapping => "rcc8po",
        Rcc8Relation::TangentiallyProperPart => "rcc8tpp",
        Rcc8Relation::TangentiallyProperPartInverse => "rcc8tppi",
        Rcc8Relation::NonTangentialProperPart => "rcc8ntpp",
        Rcc8Relation::NonTangentialProperPartInverse => "rcc8ntppi",
        Rcc8Relation::Equal => "rcc8eq",
    };
    let iri = format!("http://www.opengis.net/ont/geosparql#{relation}");
    let predicate = q_hash(&iri);
    let quin = NQuin {
        subject: a,
        predicate,
        object: b,
        context,
        metadata: 0,
        parity: NQuin::calculate_parity(a, predicate, b, context, 0),
    };
    Ok(result(
        BTreeMap::from([
            ("relation".into(), Value::String(iri)),
            ("region_a".into(), Value::U64(a)),
            ("region_b".into(), Value::U64(b)),
        ]),
        &[quin],
    ))
}

fn asp_paraconsistent(model: &mut Model<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "context",
            "atoms",
            "rules",
            "evidence_subject",
            "positive_predicate",
            "negative_predicate",
        ],
    )?;
    let context = model.named(params, "context")?;
    let subject = model.named(params, "evidence_subject")?;
    let positive = model.named(params, "positive_predicate")?;
    let negative = model.named(params, "negative_predicate")?;
    if positive == negative {
        return Err("ASP requires distinct positive and negative evidence predicates".into());
    }
    let mut atoms = [0; asp::ASP_MAX_ATOMS];
    let count = term_list(model, Some(required(params, "atoms")?), &mut atoms)?;
    if count == 0 {
        return Err("ASP requires at least one declared atom".into());
    }
    let atoms = &atoms[..count];
    let spec = list(required(params, "rules")?)?;
    if spec.len() > 64 {
        return Err("ASP exceeds 64 authored rules".into());
    }
    let mut program = [asp::AspRule {
        head: 0,
        pos: [0; asp::ASP_MAX_BODY],
        pos_len: 0,
        neg: [0; asp::ASP_MAX_BODY],
        neg_len: 0,
    }; 64 + asp::ASP_MAX_ATOMS * 2];
    for (slot, row) in spec.iter().enumerate() {
        let row = record(row)?;
        fields(row, &["head", "positive", "negative"])?;
        let mut pos = [0; asp::ASP_MAX_BODY];
        let mut neg = [0; asp::ASP_MAX_BODY];
        let pc = term_list(model, row.get("positive"), &mut pos)?;
        let nc = term_list(model, row.get("negative"), &mut neg)?;
        let head = row
            .get("head")
            .map(|head| model.term(head))
            .transpose()?
            .unwrap_or(0);
        if (head != 0 && !atoms.contains(&head))
            || pos[..pc]
                .iter()
                .chain(&neg[..nc])
                .any(|term| !atoms.contains(term))
        {
            return Err("An ASP rule references an undeclared atom".into());
        }
        program[slot] = asp::AspRule::new(head, &pos[..pc], &neg[..nc]);
    }
    let mut evidence = [NQuin::default(); MAX_FACTS + 1];
    let len = model.facts(
        Q42RangeQueryPattern {
            subject: Some(subject),
            context: Some(context),
            ..Q42RangeQueryPattern::default()
        },
        &mut evidence,
    )?;
    let evidence = &evidence[..len];
    let observed = |atom, predicate| {
        evidence
            .iter()
            .any(|q| q.object == atom && q.predicate == predicate)
    };
    let mut rule_count = spec.len();
    for &atom in atoms {
        if observed(atom, positive) {
            program[rule_count] = asp::AspRule::fact(atom);
            rule_count += 1;
        }
        if observed(atom, negative) {
            program[rule_count] = asp::AspRule::constraint(&[atom], &[]);
            rule_count += 1;
        }
    }
    let mut answer_sets = [0; MAX_ROWS + 1];
    let outcome =
        asp::answer_sets_or_paraconsistent(atoms, &program[..rule_count], &mut answer_sets);
    let mut proof = BTreeMap::new();
    match outcome {
        asp::AspOutcome::Stable(count) => {
            if count > MAX_ROWS {
                return Err("ASP exceeds 256 stable answer sets".into());
            }
            proof.insert("status".into(), Value::String("stable".into()));
            proof.insert(
                "models".into(),
                Value::List(
                    answer_sets[..count]
                        .iter()
                        .map(|mask| {
                            Value::List(
                                atoms
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(index, term)| {
                                        (*mask & (1 << index) != 0).then_some(Value::U64(*term))
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                ),
            );
        }
        asp::AspOutcome::NoStableModel => {
            proof.insert("status".into(), Value::String("paraconsistent".into()));
            proof.insert(
                "evidence".into(),
                Value::List(
                    atoms
                        .iter()
                        .map(|&atom| {
                            let truth = Belnap::from_evidence(
                                observed(atom, positive),
                                observed(atom, negative),
                            );
                            Value::Record(BTreeMap::from([
                                ("atom".into(), Value::U64(atom)),
                                (
                                    "support".into(),
                                    Value::String(format!("{truth:?}").to_lowercase()),
                                ),
                            ]))
                        })
                        .collect(),
                ),
            );
        }
    }
    Ok(result(proof, &[]))
}

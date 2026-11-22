//! Bounded local Q42 verification, temporal replay, and RDF-Star projections.

use crate::{fields, list, record, required, text, Args, AssetHost, Value};
use qualia_core_db::{
    crdt::CrdtResolver,
    git_bridge::{quins_merkle, DagStore},
    lexicon::generate_60bit_token,
    mini_parser::{OP_END, OP_HALT_IF_FALSE, OP_MATCH_OBJECT, OP_MATCH_PREDICATE, OP_MATCH_SUBJECT},
    q42_lex::LexiconEntry,
    q42_volume::{quin_requires_sanctuary, UnifiedVolumeBuilder},
    query::temporal_scrub::{frame_diff_hints, replay_to_state},
    rdf_star::triple_from_components,
    webizen_bytecode::execute_program_simd,
    NQuin, QUINS_PER_BLOCK,
};
use std::collections::{hash_map::Entry, BTreeMap, HashMap};

const MAX_COMMITS: usize = 512;
const MAX_ROWS: usize = 8192;
const MAX_TERMS: usize = 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "prove_rdf_q42_equivalence" => proof::prove(host, params),
        "temporal_frame_diff" => temporal(host, params, true),
        "temporal_replay" => temporal(host, params, false),
        "rdf_star_annotate" => annotate(host, params),
        "resolve_nquin_crdt" => resolve_crdt(params),
        "filter_nquins" => filter(host, params),
        _ => Err(format!("Unsupported RDF or temporal operation: {op}")),
    }
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected an unsigned integer".into()),
        _ => Err("Expected an unsigned integer".into()),
    }
}

fn row(value: &Value) -> Result<NQuin, String> {
    let [s, p, o, c, m, parity] = list(value)? else {
        return Err("Expected the six exact NQuin columns [s, p, o, c, m, parity]".into());
    };
    let row = NQuin {
        subject: unsigned(s)?,
        predicate: unsigned(p)?,
        object: unsigned(o)?,
        context: unsigned(c)?,
        metadata: unsigned(m)?,
        parity: unsigned(parity)?,
    };
    if row.subject == 0 || row.predicate == 0 || !row.verify_ecc_parity() {
        return Err("NQuin has a zero subject or predicate, or failed its parity check".into());
    }
    Ok(row)
}

fn columns(row: NQuin) -> Value {
    Value::List([row.subject, row.predicate, row.object, row.context, row.metadata, row.parity]
        .into_iter().map(Value::U64).collect())
}

fn rows(values: Vec<NQuin>) -> Value {
    Value::List(values.into_iter().map(columns).collect())
}

fn temporal(host: &mut AssetHost<'_>, params: &Args, diff: bool) -> Result<Value, String> {
    let allowed = if diff { &["commits", "from_ms", "to_ms"][..] }
        else { &["commits", "as_of_ms"][..] };
    fields(params, allowed)?;
    let commits = list(required(params, "commits")?)?;
    if commits.len() > MAX_COMMITS {
        return Err("Temporal history exceeds its commit budget".into());
    }
    let mut total = 0;
    for commit in commits {
        let commit = record(commit)?;
        let count = list(required(commit, "rows")?)?.len();
        if count == 0 || count > MAX_ROWS - total {
            return Err("Each commit requires nonempty rows within the total history budget".into());
        }
        total += count;
    }
    host.charge(total * 16 + commits.len(), total * 512 + commits.len() * 1024)?;
    let mut dag = DagStore::new();
    let mut blocks = BTreeMap::<[u8; 32], Vec<NQuin>>::new();
    let mut tip = [0; 32];
    let mut last_time = 0;
    for (index, commit) in commits.iter().enumerate() {
        let commit = record(commit)?;
        fields(commit, &["timestamp_ms", "author_id", "rows", "message"])?;
        let time = unsigned(required(commit, "timestamp_ms")?)?;
        let author = unsigned(required(commit, "author_id")?)?;
        let message = commit.get("message").map(text).transpose()?.unwrap_or_default();
        if time < last_time || author == 0 || message.len() > 2048 {
            return Err("Temporal commits require ordered timestamps, a nonzero author and bounded messages".into());
        }
        last_time = time;
        let records = list(required(commit, "rows")?)?.iter().map(row).collect::<Result<Vec<_>, _>>()?;
        let key = quins_merkle(&records);
        if blocks.get(&key).is_some_and(|previous| previous != &records) {
            return Err("Temporal commits share a native four-column Merkle key with different metadata".into());
        }
        tip = if index == 0 {
            dag.genesis_node(&records, author, time, message)
        } else {
            dag.commit_node(tip, &records, author, time, message)
        };
        blocks.entry(key).or_insert(records);
    }
    let resolve = |digest| blocks.get(&digest).cloned();
    let mut result = if diff {
        let from = unsigned(required(params, "from_ms")?)?;
        let to = unsigned(required(params, "to_ms")?)?;
        let (added, removed) = frame_diff_hints(&dag, from, to, resolve);
        BTreeMap::from([("added".into(), rows(added)), ("removed".into(), rows(removed))])
    } else {
        let as_of = unsigned(required(params, "as_of_ms")?)?;
        BTreeMap::from([("rows".into(), rows(replay_to_state(&dag, as_of, resolve)))])
    };
    result.insert("append_only".into(), Value::Bool(true));
    result.insert("tip_sha256".into(), Value::String(hex(&tip)));
    Ok(Value::Record(result))
}

fn hex(source: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(source.len() * 2);
    for byte in source {
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn iri(value: &Value) -> Result<&str, String> {
    let value = match value {
        Value::String(value) | Value::Iri(value) => value.as_str(),
        _ => return Err("RDF-Star term requires an absolute IRI".into()),
    };
    let (scheme, rest) = value.split_once(':').ok_or("RDF-Star term requires an absolute IRI")?;
    let mut scheme = scheme.bytes();
    if value.len() > 1024 || !scheme.next().is_some_and(|first| first.is_ascii_alphabetic())
        || !scheme.all(|byte| byte.is_ascii_alphanumeric() || b"+-.".contains(&byte))
        || rest.is_empty()
        || rest.chars().any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
    {
        return Err("RDF-Star term requires a bounded, lexical absolute IRI".into());
    }
    Ok(value)
}

fn term(lex: &mut HashMap<u64, LexiconEntry>, value: &Value) -> Result<u64, String> {
    let spelling = iri(value)?;
    let hash = generate_60bit_token(spelling.as_bytes());
    if hash == 0 {
        return Err("RDF-Star term hashes to the reserved zero token".into());
    }
    match lex.entry(hash) {
        Entry::Occupied(entry) if !matches!(entry.get(), LexiconEntry::String(existing) if existing == spelling) =>
            return Err("RDF-Star IRI token collision".into()),
        Entry::Occupied(_) => {}
        Entry::Vacant(entry) => { entry.insert(LexiconEntry::String(spelling.to_owned())); }
    }
    Ok(hash)
}

fn annotate(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["annotations"])?;
    let annotations = list(required(params, "annotations")?)?;
    if annotations.is_empty() || annotations.len() > MAX_TERMS {
        return Err("Expected a bounded nonempty set of RDF-Star annotations".into());
    }
    host.charge(annotations.len() * 128, annotations.len() * 8192)?;
    let mut lex = HashMap::new();
    let mut quins = Vec::with_capacity(annotations.len());
    for entry in annotations {
        let entry = record(entry)?;
        fields(entry, &["subject", "predicate", "object", "annotation_predicate", "annotation_object", "context"])?;
        let triple = [
            term(&mut lex, required(entry, "subject")?)?,
            term(&mut lex, required(entry, "predicate")?)?,
            term(&mut lex, required(entry, "object")?)?,
        ];
        let subject = triple_from_components(triple[0], triple[1], triple[2]);
        match lex.entry(subject) {
            Entry::Occupied(entry) if !matches!(entry.get(), LexiconEntry::EmbeddedTriple(existing) if existing == &triple) =>
                return Err("RDF-Star embedded triple token collision".into()),
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => { entry.insert(LexiconEntry::EmbeddedTriple(triple)); }
        }
        let predicate = term(&mut lex, required(entry, "annotation_predicate")?)?;
        let object = term(&mut lex, required(entry, "annotation_object")?)?;
        let context = entry.get("context").map(|value| term(&mut lex, value)).transpose()?.unwrap_or(0);
        let parity = NQuin::calculate_parity(subject, predicate, object, context, 0);
        quins.push(NQuin { subject, predicate, object, context, metadata: 0, parity });
    }
    // Stable object order is required by Q42 block construction. Within equal object
    // keys the additional columns make the compiled volume independent of input ordering.
    quins.sort_unstable_by_key(|row| (row.object, row.subject, row.predicate, row.context));
    quins.dedup();
    let mut q42 = UnifiedVolumeBuilder::with_lex_entries(&lex).map_err(|error| error.to_string())?
        .with_fixed_timestamp_ms(0);
    for (index, chunk) in quins.chunks(QUINS_PER_BLOCK).enumerate() {
        q42.push_block(index as u64, chunk).map_err(|error| error.to_string())?;
    }
    let bytes = q42.finish_to_bytes();
    if bytes.len() > host.limits.resource_bytes.min(42 * 1024 * 1024) {
        return Err("RDF-Star annotation exceeded the bounded Q42 output budget".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("q42".into(), host.encode("q42", bytes)?),
        ("annotations".into(), Value::U64(quins.len() as u64)),
    ])))
}

fn resolve_crdt(params: &Args) -> Result<Value, String> {
    fields(params, &["local", "remote", "domain"])?;
    let local = row(required(params, "local")?)?;
    let remote = row(required(params, "remote")?)?;
    if (local.subject, local.predicate, local.context) != (remote.subject, remote.predicate, remote.context) {
        return Err("CRDT input rows must address the same subject, predicate and context".into());
    }
    let domain = text(required(params, "domain")?)?;
    let selfhood = match domain {
        "selfhood" => true,
        "shared_fixture" => quin_requires_sanctuary(&local) || quin_requires_sanctuary(&remote),
        _ => return Err("CRDT resolution requires selfhood or a shared fixture domain".into()),
    };
    let winner = CrdtResolver::resolve_lww(&local, &remote, selfhood);
    Ok(Value::Record(BTreeMap::from([
        ("row".into(), columns(winner)),
        ("local_retained".into(), Value::Bool(winner == local)),
        ("selfhood".into(), Value::Bool(selfhood)),
    ])))
}

fn filter(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["rows", "subject", "predicate", "object"])?;
    let records = list(required(params, "rows")?)?;
    if records.len() > MAX_ROWS {
        return Err("NQuin VM filter exceeds its bounded row count".into());
    }
    host.charge(records.len() * 8, records.len() * 512)?;
    let mut program = Vec::with_capacity(31);
    for (name, opcode) in [
        ("subject", OP_MATCH_SUBJECT), ("predicate", OP_MATCH_PREDICATE), ("object", OP_MATCH_OBJECT),
    ] {
        if let Some(value) = params.get(name) {
            program.push(opcode);
            program.extend_from_slice(&unsigned(value)?.to_le_bytes());
            program.push(OP_HALT_IF_FALSE);
        }
    }
    program.push(OP_END);
    let input = records.iter().map(row).collect::<Result<Vec<_>, _>>()?;
    let mut output = vec![NQuin::default(); input.len()];
    let (count, cycles) = execute_program_simd(&program, &input, &mut output)
        .map_err(|error| format!("NQuin filter VM failed: {error:?}"))?;
    output.truncate(count);
    Ok(Value::Record(BTreeMap::from([
        ("rows".into(), rows(output)),
        ("match_count".into(), Value::U64(count as u64)),
        ("vm_cycles".into(), Value::U64(cycles)),
    ])))
}

#[cfg(target_arch = "wasm32")]
mod proof {
    use super::*;

    pub(super) fn prove(_host: &mut AssetHost<'_>, _params: &Args) -> Result<Value, String> {
        Err("CapabilityUnavailable: external RDF proof sorting requires native local scratch".into())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod proof {
    use super::*;
    use crate::Resource;
    use qualia_core_db::{
        q42_volume::{OpfsSliceRangeSource, Q42RangeVolume, FLAG_SANCTUARY, FLAG_VOLUME_ROOT},
        query::graph_proof::{prove_import_rdf_q42_equivalence, EncoderProfile, ImportProofOptions, RdfIsomorphismStatus},
    };
    use std::{
        fs::{self, DirBuilder, OpenOptions},
        io::{ErrorKind, Write},
        path::{Path, PathBuf},
        sync::atomic::{AtomicU64, Ordering},
    };

    const MIB: usize = 1024 * 1024;
    // The native proof retains the source and Q42 sort vectors until it returns.
    const MAX_SORT_BYTES: usize = 14 * MIB;
    const MIN_SORT_BYTES: usize = MIB / 8;
    const MAX_TEMP_BYTES: u64 = 512 * MIB as u64;
    const MAX_INPUT_BYTES: usize = 64 * MIB;
    static NEXT_WORKSPACE: AtomicU64 = AtomicU64::new(0);

    struct Workspace {
        dir: PathBuf,
        source: PathBuf,
        volume: PathBuf,
    }

    impl Workspace {
        fn new(extension: &str) -> Result<Self, String> {
            let base = std::env::temp_dir();
            let builder = DirBuilder::new();
            #[cfg(unix)]
            let builder = {
                use std::os::unix::fs::DirBuilderExt;
                let mut builder = builder;
                builder.mode(0o700);
                builder
            };
            for _ in 0..32 {
                let index = NEXT_WORKSPACE.fetch_add(1, Ordering::Relaxed);
                let dir = base.join(format!("qualia-rdf-proof-{}-{index}", std::process::id()));
                match builder.create(&dir) {
                    Ok(()) => return Ok(Self {
                        source: dir.join(format!("source.{extension}")),
                        volume: dir.join("input.q42"),
                        dir,
                    }),
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => continue,
                    Err(error) => return Err(format!("Could not allocate RDF proof scratch: {error}")),
                }
            }
            Err("Could not allocate unique RDF proof scratch".into())
        }

        fn stage(path: &Path, bytes: &[u8]) -> Result<(), String> {
            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let mut file = options.open(path).map_err(|error| error.to_string())?;
            file.write_all(bytes).map_err(|error| error.to_string())
        }
    }

    impl Drop for Workspace {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.source);
            let _ = fs::remove_file(&self.volume);
            let _ = fs::remove_dir(&self.dir);
        }
    }

    fn resource<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
        match value {
            Value::String(key) => host.inputs.iter().find(|resource| &resource.key == key)
                .ok_or_else(|| "Unknown retained RDF proof input".into()),
            _ => host.workspace.encoded(value),
        }
    }

    pub(super) fn prove(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
        fields(params, &["source", "q42", "sort_memory_bytes", "temporary_byte_budget", "strip_literals"])?;
        let source_arg = required(params, "source")?;
        let volume_arg = required(params, "q42")?;
        let source = resource(host, source_arg)?;
        let volume = resource(host, volume_arg)?;
        let extension = match source.kind.as_str() {
            "nt" | "ntriples" => "nt",
            "nq" | "nquads" => "nq",
            "ttl" | "turtle" => "ttl",
            "trig" => "trig",
            _ => return Err("RDF equivalence requires a retained NT, NQ, Turtle or TriG source".into()),
        };
        let input_bytes = source.bytes.len().saturating_add(volume.bytes.len());
        let max_per_resource = host.limits.resource_bytes;
        if volume.kind != "q42" || source.bytes.len() > max_per_resource || volume.bytes.len() > max_per_resource
            || input_bytes > MAX_INPUT_BYTES
        {
            return Err("RDF proof inputs exceed the bounded retained resource extent".into());
        }
        let memory = params.get("sort_memory_bytes").map(unsigned).transpose()?.unwrap_or(8 * MIB as u64);
        let temp = params.get("temporary_byte_budget").map(unsigned).transpose()?.unwrap_or(256 * MIB as u64);
        if !(MIN_SORT_BYTES as u64..=MAX_SORT_BYTES as u64).contains(&memory) || temp > MAX_TEMP_BYTES
            || temp <= input_bytes as u64
        {
            return Err("Invalid RDF proof sort or temporary storage budget".into());
        }
        let strip = match params.get("strip_literals") {
            None => false,
            Some(Value::Bool(value)) => *value,
            _ => return Err("strip_literals requires a boolean".into()),
        };
        host.charge(input_bytes.saturating_mul(8), 2 * memory as usize + 4 * MIB)?;
        let volume = resource(host, volume_arg)?;
        let flags = Q42RangeVolume::open(OpfsSliceRangeSource::new(&volume.bytes))
            .map_err(|error| error.to_string())?.header().flags;
        if flags & (FLAG_SANCTUARY | FLAG_VOLUME_ROOT) != 0 {
            return Err("RDF proof requires a self-contained Q42 input admitted for disclosure".into());
        }
        let source = resource(host, source_arg)?;
        let text = std::str::from_utf8(&source.bytes).map_err(|_| "RDF proof source must be UTF-8")?;
        if text.lines().any(|line| line.len() > 64 * 1024)
            || text.as_bytes().windows(2).filter(|pair| *pair == b"<<").take(65).count() > 64
        {
            return Err("RDF source exceeds the proof lexical or quoted-triple budget".into());
        }
        let turtle = matches!(extension, "ttl" | "trig");
        if turtle && text.bytes().filter(|&byte| matches!(byte, b'[' | b'(')).take(257).count() > 256 {
            return Err("RDF source exceeds the proof collection and property-list budget".into());
        }
        // Native ingest does not retain blank graph labels in RawTriple.context.
        // Treat possible blank graph terms and anonymous Turtle nodes conservatively.
        let possible_blank_nodes = text.contains("_:") || (turtle && (text.contains('[') || text.contains('(')));
        let workspace = Workspace::new(extension)?;
        Workspace::stage(&workspace.source, &source.bytes)?;
        Workspace::stage(&workspace.volume, &volume.bytes)?;
        let report = prove_import_rdf_q42_equivalence(&workspace.source, &workspace.volume, ImportProofOptions {
            memory_limit_bytes: memory as usize,
            temporary_byte_budget: temp - input_bytes as u64,
            omit_predicate_hashes: Vec::new(),
            encoder_profile: if strip { EncoderProfile::ImportStripLiterals } else { EncoderProfile::ImportComplete },
        }).map_err(|error| error.to_string())?;
        let status = match report.rdf_isomorphism {
            RdfIsomorphismStatus::GroundGraphProven if possible_blank_nodes => "BlankNodeCanonicalizationRequired",
            RdfIsomorphismStatus::GroundGraphProven => "GroundGraphProven",
            RdfIsomorphismStatus::BlankNodeCanonicalizationRequired => "BlankNodeCanonicalizationRequired",
            RdfIsomorphismStatus::Different => "Different",
        };
        let first = |entry: Option<[u64; 4]>| entry.map_or(Value::Null, |row|
            Value::List(row.into_iter().map(Value::U64).collect()));
        Ok(Value::Record(BTreeMap::from([
            ("status".into(), Value::String(status.into())),
            ("scope".into(), Value::String("encoded_ingestion_quad_set".into())),
            ("encoded_sets_match".into(), Value::Bool(report.encoded_sets_match())),
            ("source_blake3".into(), Value::String(blake3::hash(&source.bytes).to_hex().to_string())),
            ("q42_blake3".into(), Value::String(blake3::hash(&volume.bytes).to_hex().to_string())),
            ("source_records".into(), Value::U64(report.source_records)),
            ("q42_records".into(), Value::U64(report.q42_records)),
            ("source_unique_records".into(), Value::U64(report.source_unique_records)),
            ("q42_unique_records".into(), Value::U64(report.q42_unique_records)),
            ("missing_from_q42".into(), Value::U64(report.missing_from_q42)),
            ("unexpected_in_q42".into(), Value::U64(report.unexpected_in_q42)),
            ("first_missing".into(), first(report.first_missing)),
            ("first_unexpected".into(), first(report.first_unexpected)),
            ("source_may_contain_blank_nodes".into(), Value::Bool(report.source_contains_blank_nodes || possible_blank_nodes)),
            ("oversize_literal_count".into(), Value::U64(report.oversize_literal_count)),
            ("lexicon_has_no_terms".into(), Value::Bool(report.lexicon_has_no_terms)),
            ("encoder_profile".into(), Value::String(if strip { "import_strip_literals" } else { "import_complete" }.into())),
            ("sort_memory_bytes".into(), Value::U64(memory)),
            ("combined_sort_buffer_bytes".into(), Value::U64(2 * memory)),
            ("temporary_byte_budget".into(), Value::U64(temp)),
        ])))
    }
}

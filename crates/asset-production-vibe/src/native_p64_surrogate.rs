//! Deterministic scalar statistical surrogates with a read-back F16 error gate.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::atomic::Ordering,
};

use qualia_core_db::{
    NQuin,
    bundle::semantic_model::SemanticModelBuilder,
    modalities::{
        evidential_etau::{f16_to_f32, f32_to_f16},
        p64_cartridge::{pack_p64_extension_cartridge_into, unpack_p64_extension_cartridge_into},
    },
    q_hash,
    q42::p64_weight::{IntegrityMode, P64TensorIndex, transcode_safetensor_to_p64},
    q42_volume::{
        FLAG_SANCTUARY, FLAG_VOLUME_ROOT, MAX_COMPRESSED_SUPERBLOCK_SIZE, OpfsSliceRangeSource,
        Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan, Q42RangeVolume,
        SUPERBLOCK_SIZE, WasmSliceQ42Reader,
    },
    resolver::{InlineLiteral, classify_inline_literal},
    solvers::learning::{metrics::regression::rmse, splines::polynomial_regression},
};
use sha2::{Digest, Sha256};

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{Args, AssetHost, Resource, Value, fields, integer, list, num, required, text};

const SURROGATE: &str = "urn:qualia:surrogate:v1:";
const RECEIPT: &str = "urn:qualia:surrogate:v1:calibration";
const COEFFICIENTS: &str = "qualia.surrogate.polynomial.coefficients";
const DOMAIN: &str = "qualia.surrogate.polynomial.domain";
const MIN_EVALUATION: usize = 8;
const MAX_SAMPLES: usize = 4096;
const MAX_DATASET_BYTES: usize = 8 * 1024 * 1024;
const MAX_MODEL_BYTES: usize = 64 * 1024;
const MAX_DEGREE: usize = 6;

macro_rules! term {
    ($name:literal) => {
        concat!("urn:qualia:surrogate:v1:", $name)
    };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Split {
    Training,
    Calibration,
}

struct Sample {
    id: String,
    x: f64,
    y: f64,
    split: Split,
}

struct TeacherTrace {
    teacher_iri: String,
    samples: Vec<Sample>,
}

impl AssetHost<'_> {
    pub(crate) fn compile_p64_surrogate_cartridge(&mut self, args: &Args) -> Result<Value, String> {
        let mode = args.get("mode").map(text).transpose()?.unwrap_or("compile");
        match mode {
            "compile" => self.compile_polynomial_p64(args),
            "predict" => self.predict_polynomial_p64(args),
            _ => Err(format!("Unknown P64 surrogate mode {mode}")),
        }
    }

    fn compile_polynomial_p64(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "source",
                "source_sha256",
                "dataset_context",
                "model_iri",
                "architecture",
                "quant",
                "degree",
                "domain",
                "max_rmse",
            ],
        )?;
        if text(required(args, "architecture")?)? != "stat_ml_polynomial"
            || text(required(args, "quant")?)? != "f16"
        {
            return Err("Statistical surrogate authoring supports the stat_ml_polynomial architecture and f16 weighting".into());
        }
        let handle = required(args, "source")?;
        let degree = integer(required(args, "degree")?, "degree")?;
        let domain = parse_domain(required(args, "domain")?)?;
        let ceiling = num(required(args, "max_rmse")?)?;
        let model_iri = text(required(args, "model_iri")?)?;
        if !(1..=MAX_DEGREE).contains(&degree)
            || !ceiling.is_finite()
            || ceiling < 0.0
            || !valid_iri(model_iri)
            || q_hash(model_iri) == 0
        {
            return Err("Surrogate requires degree 1–6, a canonical model IRI, and a finite nonnegative RMSE limit".into());
        }
        let size = retained(self, handle)?.bytes.len();
        if size == 0 || size > self.limits.resource_bytes.min(MAX_DATASET_BYTES) {
            return Err("Surrogate dataset exceeds the admitted retained byte budget".into());
        }
        self.charge(
            size.saturating_mul(4)
                .saturating_add(MAX_SAMPLES * (degree + 1).pow(2)),
            4 * 1024 * 1024,
        )?;
        let source = retained(self, handle)?;
        let source_sha = digest_matches(&source.bytes, required(args, "source_sha256")?)?;
        let (samples, provenance, sanctuary) = match source.kind.as_str() {
            "q42" => {
                let context = text(required(args, "dataset_context")?)?;
                let (samples, sanctuary) = read_q42(&source.bytes, context, self.cancel)?;
                (samples, context.to_owned(), sanctuary)
            }
            "json" => {
                if args.contains_key("dataset_context") {
                    return Err(
                        "A teacher trace carries its own teacher_iri accession identifier".into(),
                    );
                }
                let trace = read_teacher_trace(&source.bytes)?;
                (trace.samples, trace.teacher_iri, false)
            }
            _ => {
                return Err(
                    "Surrogate requires a retained Q42 or exact authored JSON teacher trace".into(),
                );
            }
        };
        let data = admit_samples(samples, domain, degree)?;
        let model = polynomial_regression(&data.train_x, &data.train_y, data.train_x.len(), degree)
            .map_err(|error| format!("Native polynomial fit: {error:?}"))?;
        let p64 = encode_weights(&model.coefficients, domain)?;
        let readback = Polynomial::read(&p64)?;
        let predictions = data
            .calibration_x
            .iter()
            .map(|&x| readback.predict(x))
            .collect::<Result<Vec<_>, _>>()?;
        let error = rmse(&data.calibration_y, &predictions)
            .filter(|error| error.is_finite())
            .ok_or("Native surrogate evaluation returned a nonfinite error")?;
        let max_error = data
            .calibration_y
            .iter()
            .zip(&predictions)
            .map(|(observed, predicted)| (observed - predicted).abs())
            .fold(0.0, f64::max);
        if error > ceiling {
            return Err(format!(
                "F16 calibration RMSE {error} exceeds the requested {ceiling}"
            ));
        }
        let p64_sha = format!("{:x}", Sha256::digest(&p64));
        let (receipt, extension) = calibration_receipt(
            model_iri,
            &provenance,
            &source_sha,
            &p64_sha,
            domain,
            &data,
            error,
            max_error,
            ceiling,
            self.limits.resource_bytes.min(1024 * 1024),
            sanctuary,
        )?;
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        Ok(Value::Record(BTreeMap::from([
            ("p64".into(), self.encode("p64", p64)?),
            ("p64_extension".into(), self.encode("p64ext", extension)?),
            ("calibration_receipt".into(), self.encode("q42", receipt)?),
            ("p64_sha256".into(), Value::String(p64_sha)),
            ("source_sha256".into(), Value::String(source_sha)),
            (
                "training_count".into(),
                Value::U64(data.train_x.len() as u64),
            ),
            (
                "calibration_count".into(),
                Value::U64(data.calibration_x.len() as u64),
            ),
            ("rmse".into(), Value::F64(error)),
            ("max_absolute_error".into(), Value::F64(max_error)),
        ])))
    }

    fn predict_polynomial_p64(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "cartridge", "p64_sha256", "inputs"])?;
        let handle = required(args, "cartridge")?;
        let values = list(required(args, "inputs")?)?;
        let size = retained(self, handle)?.bytes.len();
        if values.is_empty() || values.len() > MAX_SAMPLES || size > MAX_MODEL_BYTES {
            return Err(
                "Surrogate prediction input exceeds the admitted model or batch bound".into(),
            );
        }
        self.charge(
            size.saturating_mul(2)
                .saturating_add(values.len() * MAX_DEGREE),
            0,
        )?;
        let retained = retained(self, handle)?;
        if retained.kind != "p64" {
            return Err("Surrogate prediction requires its native P64 weight asset".into());
        }
        let digest = digest_matches(&retained.bytes, required(args, "p64_sha256")?)?;
        let polynomial = Polynomial::read(&retained.bytes)?;
        let mut predicted = Vec::with_capacity(values.len());
        for value in values {
            if self.cancel.load(Ordering::Relaxed) {
                return Err("Production cancelled".into());
            }
            predicted.push(Value::F64(polynomial.predict(num(value)?)?));
        }
        Ok(Value::Record(BTreeMap::from([
            ("p64_sha256".into(), Value::String(digest)),
            ("predictions".into(), Value::List(predicted)),
        ])))
    }
}

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|entry| &entry.key == key)
            .ok_or_else(|| format!("Unknown retained surrogate resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn digest_matches(bytes: &[u8], expected: &Value) -> Result<String, String> {
    let digest = format!("{:x}", Sha256::digest(bytes));
    if text(expected)? != digest {
        return Err("Surrogate input does not match its pinned lowercase SHA-256".into());
    }
    Ok(digest)
}

fn read_teacher_trace(bytes: &[u8]) -> Result<TeacherTrace, String> {
    let document: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|error| format!("Teacher trace: {error}"))?;
    let trace = document
        .as_object()
        .ok_or("Expected an authored teacher trace record")?;
    if trace.len() != 2 {
        return Err("Teacher trace requires exactly teacher_iri and samples".into());
    }
    let teacher_iri = trace
        .get("teacher_iri")
        .and_then(serde_json::Value::as_str)
        .filter(|iri| valid_iri(iri))
        .ok_or("Teacher trace requires a canonical teacher IRI")?;
    let rows = trace
        .get("samples")
        .and_then(serde_json::Value::as_array)
        .ok_or("Teacher trace requires an authored samples array")?;
    if rows.len() > MAX_SAMPLES {
        return Err("Teacher trace exceeds the fixed sample bound".into());
    }
    let mut samples = Vec::with_capacity(rows.len());
    for value in rows {
        let record = value
            .as_object()
            .ok_or("Teacher sample requires an authored record")?;
        if record.len() != 4 {
            return Err("Teacher sample requires exactly id, x, y and split".into());
        }
        let id = record
            .get("id")
            .and_then(serde_json::Value::as_str)
            .ok_or("Teacher sample needs a canonical id")?
            .to_owned();
        let x = record
            .get("x")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Teacher sample needs numeric x")?;
        let y = record
            .get("y")
            .and_then(serde_json::Value::as_f64)
            .ok_or("Teacher sample needs numeric y")?;
        let split = match record.get("split").and_then(serde_json::Value::as_str) {
            Some("training") => Split::Training,
            Some("calibration") => Split::Calibration,
            _ => {
                return Err(
                    "Teacher sample requires an explicit training or calibration split".into(),
                );
            }
        };
        samples.push(Sample { id, x, y, split });
    }
    Ok(TeacherTrace {
        teacher_iri: teacher_iri.to_owned(),
        samples,
    })
}

fn valid_iri(iri: &str) -> bool {
    let Some((scheme, local)) = iri.split_once(':') else {
        return false;
    };
    iri.len() <= 2048
        && !scheme.is_empty()
        && scheme.as_bytes()[0].is_ascii_alphabetic()
        && scheme
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"+-.".contains(&byte))
        && !local.is_empty()
        && !local.chars().any(|ch| {
            ch.is_whitespace()
                || ch.is_control()
                || "<>\"{}|\\^".contains(ch)
                || ch == char::from(0x60)
        })
}

#[derive(Clone, Copy)]
struct Domain {
    min: f32,
    max: f32,
}

impl Domain {
    fn admit(min: f32, max: f32) -> Result<Self, String> {
        if !min.is_finite() || !max.is_finite() || min >= max {
            return Err("Surrogate domain requires distinct finite native F32 endpoints".into());
        }
        Ok(Self { min, max })
    }

    fn scale(self, x: f64) -> Result<f64, String> {
        let (min, max) = (f64::from(self.min), f64::from(self.max));
        if !x.is_finite() || x < min || x > max {
            return Err("Sample or prediction lies outside the admitted surrogate domain".into());
        }
        Ok(2.0 * ((x - min) / (max - min)) - 1.0)
    }
}

fn parse_domain(value: &Value) -> Result<Domain, String> {
    let [min, max] = list(value)? else {
        return Err("domain requires [minimum, maximum]".into());
    };
    let (min, max) = (num(min)?, num(max)?);
    if f64::from(min as f32) != min || f64::from(max as f32) != max {
        return Err(
            "Author the surrogate domain endpoints as exactly representable F32 values".into(),
        );
    }
    Domain::admit(min as f32, max as f32)
}

struct AdmittedSamples {
    train_x: Vec<f64>,
    train_y: Vec<f64>,
    calibration_x: Vec<f64>,
    calibration_y: Vec<f64>,
}

fn admit_samples(
    mut samples: Vec<Sample>,
    domain: Domain,
    degree: usize,
) -> Result<AdmittedSamples, String> {
    if samples.len() > MAX_SAMPLES {
        return Err("Surrogate dataset exceeds 4096 samples".into());
    }
    samples.sort_unstable_by(|left, right| left.id.cmp(&right.id));
    if samples.windows(2).any(|pair| pair[0].id == pair[1].id) {
        return Err("Surrogate dataset repeats a sample identity across its split".into());
    }
    let mut samples_admitted = AdmittedSamples {
        train_x: Vec::new(),
        train_y: Vec::new(),
        calibration_x: Vec::new(),
        calibration_y: Vec::new(),
    };
    let mut training_points = BTreeSet::new();
    let mut calibration_points = BTreeSet::new();
    for sample in samples {
        if !valid_iri(&sample.id) || !sample.y.is_finite() || sample.y.abs() > 1.0e12 {
            return Err("Surrogate sample needs a canonical IRI and bounded finite target".into());
        }
        let normalized = domain.scale(sample.x)?;
        let key = if normalized == 0.0 {
            0
        } else {
            normalized.to_bits()
        };
        match sample.split {
            Split::Training => {
                samples_admitted.train_x.push(normalized);
                samples_admitted.train_y.push(sample.y);
                training_points.insert(key);
            }
            Split::Calibration => {
                samples_admitted.calibration_x.push(sample.x);
                samples_admitted.calibration_y.push(sample.y);
                calibration_points.insert(key);
            }
        }
    }
    if training_points.len() <= degree
        || samples_admitted.train_x.len() < degree + 2
        || calibration_points.len() < MIN_EVALUATION
        || !training_points.is_disjoint(&calibration_points)
    {
        return Err("Surrogate needs identifiable training data and at least eight disjoint calibration positions".into());
    }
    Ok(samples_admitted)
}

#[derive(Default)]
struct DatasetRow {
    x: Option<u64>,
    y: Option<u64>,
    split: Option<u64>,
}

fn read_q42(
    bytes: &[u8],
    context: &str,
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<(Vec<Sample>, bool), String> {
    let view = WasmSliceQ42Reader::open(bytes).map_err(|error| error.to_string())?;
    if !view.is_unified_volume() || !valid_iri(context) {
        return Err(
            "Surrogate dataset requires an admitted Q42 volume and explicit RDF context".into(),
        );
    }
    let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(bytes))
        .map_err(|error| error.to_string())?;
    if volume.header().flags & FLAG_VOLUME_ROOT != 0 {
        return Err("Surrogate dataset must be a self-contained Q42 volume".into());
    }
    let sanctuary = volume.header().flags & FLAG_SANCTUARY != 0;
    let mut page = vec![0; 256 * 1024];
    let mut lex = vec![0; 16 * 1024];
    let context_hash = q_hash(context);
    if lookup(&view, context_hash, &mut page, &mut lex)? != Some(context) {
        return Err("Surrogate dataset context failed bidirectional Q42LEX resolution".into());
    }
    for predicate in [term!("input"), term!("target"), term!("split")] {
        if lookup(&view, q_hash(predicate), &mut page, &mut lex)? != Some(predicate) {
            return Err(
                "Surrogate dataset predicate failed bidirectional Q42LEX resolution".into(),
            );
        }
    }
    let plan = Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern {
        context: Some(context_hash),
        ..Q42RangeQueryPattern::default()
    });
    let mut cursor = Q42RangeQueryCursor::default();
    let mut compressed = vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE];
    let mut decompressed = vec![0; SUPERBLOCK_SIZE];
    let mut facts = [NQuin::default(); 64];
    let mut rows = BTreeMap::<u64, DatasetRow>::new();
    let mut count = 0;
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err("Production cancelled".into());
        }
        let result = view
            .execute_query_page_into(plan, cursor, &mut compressed, &mut decompressed, &mut facts)
            .map_err(|error| error.to_string())?;
        count += result.returned;
        if count > MAX_SAMPLES * 3 {
            return Err("Q42 surrogate context exceeds the bounded sample statement count".into());
        }
        for row in &facts[..result.returned] {
            if row.subject == 0 || row.context != context_hash || !row.verify_ecc_parity() {
                return Err("Q42 surrogate sample failed context or parity admission".into());
            }
            let entry = rows.entry(row.subject).or_default();
            let column = match row.predicate {
                predicate if predicate == q_hash(term!("input")) => &mut entry.x,
                predicate if predicate == q_hash(term!("target")) => &mut entry.y,
                predicate if predicate == q_hash(term!("split")) => &mut entry.split,
                _ => {
                    return Err(
                        "Surrogate dataset context contains an unknown sample predicate".into(),
                    );
                }
            };
            if column.replace(row.object).is_some() {
                return Err("Surrogate dataset repeats a sample field".into());
            }
        }
        if rows.len() > MAX_SAMPLES {
            return Err("Surrogate dataset exceeds 4096 samples".into());
        }
        match result.next_cursor {
            Some(next) if next != cursor => cursor = next,
            Some(_) => return Err("Surrogate dataset Q42 cursor did not advance".into()),
            None => break,
        }
    }
    let mut samples = Vec::with_capacity(rows.len());
    for (id, row) in rows {
        let sample_iri = lookup(&view, id, &mut page, &mut lex)?
            .filter(|iri| valid_iri(iri))
            .ok_or("Sample identity is absent from the Q42 dictionary")?
            .to_owned();
        let split = match lookup(
            &view,
            row.split.ok_or("Q42 sample has no authored split")?,
            &mut page,
            &mut lex,
        )? {
            Some(term!("training")) => Split::Training,
            Some(term!("calibration")) => Split::Calibration,
            _ => return Err("Q42 sample uses an unsupported surrogate split IRI".into()),
        };
        samples.push(Sample {
            id: sample_iri,
            x: q42_number(
                &view,
                row.x.ok_or("Q42 sample has no input")?,
                &mut page,
                &mut lex,
            )?,
            y: q42_number(
                &view,
                row.y.ok_or("Q42 sample has no target")?,
                &mut page,
                &mut lex,
            )?,
            split,
        });
    }
    Ok((samples, sanctuary))
}

fn lookup<'a>(
    view: &WasmSliceQ42Reader<'_>,
    hash: u64,
    page: &mut [u8],
    lexical: &'a mut [u8],
) -> Result<Option<&'a str>, String> {
    let len = view
        .lookup_lexicon_hash_into(hash, page, lexical)
        .map_err(|error| error.to_string())?;
    let Some(len) = len else { return Ok(None) };
    let encoded = lexical
        .get(..len)
        .ok_or("Q42 lexical buffer exceeded its declared extent")?;
    let text = std::str::from_utf8(encoded).map_err(|error| error.to_string())?;
    if hash == 0 || q_hash(text) != hash {
        return Err("Q42 surrogate lexical hash disagrees with the exact authored term".into());
    }
    Ok(Some(text))
}

fn q42_number(
    view: &WasmSliceQ42Reader<'_>,
    object: u64,
    page: &mut [u8],
    lexical: &mut [u8],
) -> Result<f64, String> {
    if let Some(term) = lookup(view, object, page, lexical)? {
        let (value, datatype) = term
            .rsplit_once("^^<")
            .ok_or("Expected an exact typed Q42 numeric value")?;
        if !matches!(
            datatype,
            "http://www.w3.org/2001/XMLSchema#integer>"
                | "http://www.w3.org/2001/XMLSchema#decimal>"
                | "http://www.w3.org/2001/XMLSchema#double>"
                | "http://www.w3.org/2001/XMLSchema#float>"
        ) {
            return Err("Unsupported Q42 surrogate numeric datatype".into());
        }
        let quoted = serde_json::from_str::<String>(value).map_err(|error| error.to_string())?;
        return quoted
            .parse::<f64>()
            .map_err(|_| "Invalid Q42 surrogate numeric lexical token".into());
    }
    match classify_inline_literal(object) {
        Some(InlineLiteral::Integer(value)) => Ok(value as f64),
        Some(InlineLiteral::Decimal(value)) => Ok(value as f64 / 1_000_000.0),
        Some(InlineLiteral::Float(value)) => Ok(f64::from(value)),
        _ => Err("Q42 surrogate requires a native scalar numeric value".into()),
    }
}

fn encode_weights(coefficients: &[f64], domain: Domain) -> Result<Vec<u8>, String> {
    if !(2..=MAX_DEGREE + 1).contains(&coefficients.len()) {
        return Err("Native polynomial returned an unexpected coefficient count".into());
    }
    let mut blob = Vec::with_capacity(coefficients.len() * 2 + 8);
    for &weight in coefficients {
        let bits = f32_to_f16(weight as f32);
        if !weight.is_finite() || !f16_to_f32(bits).is_finite() {
            return Err("A fitted coefficient is outside the finite native F16 range".into());
        }
        blob.extend_from_slice(&bits.to_le_bytes());
    }
    let domain_offset = blob.len();
    blob.extend_from_slice(&domain.min.to_le_bytes());
    blob.extend_from_slice(&domain.max.to_le_bytes());
    let manifest = BTreeMap::from([
        (
            COEFFICIENTS,
            serde_json::json!({
                "dtype": "F16", "shape": [coefficients.len()], "data_offsets": [0, domain_offset],
            }),
        ),
        (
            DOMAIN,
            serde_json::json!({
                "dtype": "F32", "shape": [2], "data_offsets": [domain_offset, blob.len()],
            }),
        ),
    ]);
    let mut header = serde_json::to_vec(&manifest).map_err(|error| error.to_string())?;
    let padded = (header.len() + 7) & !7;
    header.resize(padded, b' ');
    let mut safetensors = Vec::with_capacity(8 + header.len() + blob.len());
    safetensors.extend_from_slice(&(header.len() as u64).to_le_bytes());
    safetensors.extend_from_slice(&header);
    safetensors.extend_from_slice(&blob);
    let mut out = Vec::new();
    transcode_safetensor_to_p64(&safetensors, 12, &mut out)
        .map_err(|error| format!("Native P64 weight seal: {error}"))?;
    if out.len() > MAX_MODEL_BYTES {
        return Err("Native polynomial P64 exceeds the expected bounded weight layout".into());
    }
    Ok(out)
}

struct Polynomial {
    domain: Domain,
    coefficients: [f64; MAX_DEGREE + 1],
    count: usize,
}

impl Polynomial {
    fn read(bytes: &[u8]) -> Result<Self, String> {
        let reader = P64TensorIndex::from_p64_with_integrity(bytes, IntegrityMode::Full)?;
        if reader.entries.len() != 2 {
            return Err(
                "Polynomial P64 must have exactly its coefficient and domain tensors".into(),
            );
        }
        let coefficients = reader
            .entries
            .iter()
            .find(|entry| entry.source_name_hash == q_hash(COEFFICIENTS))
            .ok_or("Missing surrogate coefficient tensor")?;
        let domain = reader
            .entries
            .iter()
            .find(|entry| entry.source_name_hash == q_hash(DOMAIN))
            .ok_or("Missing surrogate domain tensor")?;
        let count = coefficients.dimensions[0] as usize;
        if coefficients.dtype != 1
            || coefficients.rank != 1
            || !(2..=MAX_DEGREE + 1).contains(&count)
            || domain.dtype != 0
            || domain.rank != 1
            || domain.dimensions[0] != 2
        {
            return Err("Polynomial P64 weight/dtype contract failed".into());
        }
        let bounds = reader.blob(bytes, domain);
        let [a, b, c, d, e, f, g, h] = bounds else {
            return Err("Native P64 domain has an invalid F32 extent".into());
        };
        let domain = Domain::admit(
            f32::from_le_bytes([*a, *b, *c, *d]),
            f32::from_le_bytes([*e, *f, *g, *h]),
        )?;
        let weights = reader.blob(bytes, coefficients);
        if weights.len() != count * 2 {
            return Err("Native P64 coefficients have an invalid F16 extent".into());
        }
        let mut coefficients = [0.0; MAX_DEGREE + 1];
        for (lane, raw) in coefficients.iter_mut().zip(weights.chunks_exact(2)) {
            *lane = f64::from(f16_to_f32(u16::from_le_bytes([raw[0], raw[1]])));
            if !lane.is_finite() {
                return Err("Native P64 coefficient is nonfinite".into());
            }
        }
        Ok(Self {
            domain,
            coefficients,
            count,
        })
    }

    fn predict(&self, x: f64) -> Result<f64, String> {
        let normalized = self.domain.scale(x)?;
        let prediction = self.coefficients[..self.count]
            .iter()
            .rev()
            .fold(0.0, |accumulator, weight| accumulator * normalized + weight);
        if !prediction.is_finite() {
            return Err("Native surrogate prediction is nonfinite".into());
        }
        Ok(prediction)
    }
}

fn calibration_receipt(
    iri: &str,
    source: &str,
    source_sha: &str,
    p64_sha: &str,
    domain: Domain,
    data: &AdmittedSamples,
    rmse: f64,
    max_error: f64,
    ceiling: f64,
    budget: usize,
    sanctuary: bool,
) -> Result<(Vec<u8>, Vec<u8>), String> {
    let weight_iri = format!("urn:sha256:{p64_sha}");
    let source_iri = format!("urn:sha256:{source_sha}");
    let mut receipt = ReceiptGraph::new(RECEIPT);
    let mut guards = Vec::new();
    let mut fact = |path: &str, object: &str| -> Result<(), String> {
        receipt.fact(iri, path, object)?;
        let [subject, predicate, object, context] = [iri, path, object, RECEIPT].map(q_hash);
        guards.push(NQuin {
            subject,
            predicate,
            object,
            context,
            metadata: 0,
            parity: NQuin::calculate_parity(subject, predicate, object, context, 0),
        });
        Ok(())
    };
    fact(term!("calibrationSource"), source)?;
    fact(term!("sourceDigest"), &source_iri)?;
    fact(term!("weightDigest"), &weight_iri)?;
    fact(term!("architecture"), term!("StatisticalPolynomial"))?;
    fact(term!("weightQuantization"), term!("F16"))?;
    fact(term!("calibrationVerdict"), term!("WithinRequestedRmse"))?;
    for (path, value) in [
        (term!("calibrationRmse"), rmse),
        (term!("calibrationMaxAbsoluteError"), max_error),
        (term!("requestedRmseLimit"), ceiling),
        (term!("domainMinimum"), f64::from(domain.min)),
        (term!("domainMaximum"), f64::from(domain.max)),
    ] {
        fact(
            path,
            &format!("{SURROGATE}Float64Bits:{:016x}", value.to_bits()),
        )?;
    }
    for (path, value) in [
        (term!("trainingCount"), data.train_x.len()),
        (term!("calibrationCount"), data.calibration_x.len()),
    ] {
        fact(path, &format!("{SURROGATE}Count:{value}"))?;
    }
    let mut extension = vec![0; 4096];
    let written = pack_p64_extension_cartridge_into(q_hash(iri), &[], &guards, &[], &mut extension)
        .map_err(|error| format!("P64EXT01 seal: {error:?}"))?;
    extension.truncate(written);
    let mut recovered = vec![NQuin::default(); guards.len()];
    let index = unpack_p64_extension_cartridge_into(&extension, &mut [], &mut recovered, &mut [])
        .map_err(|error| format!("P64EXT01 admission: {error:?}"))?;
    if index.asset_did_hash != q_hash(iri) || index.tier1_slot_count != 0 || recovered != guards {
        return Err("P64EXT01 guardrail read-back disagrees with the calibration receipt".into());
    }
    let receipt = receipt.seal(budget)?;
    let receipt = if sanctuary {
        let mut sealed = SemanticModelBuilder::new(budget);
        sealed.restrict_to_sanctuary();
        sealed
            .merge_q42(&receipt)
            .and_then(SemanticModelBuilder::finish)
            .map_err(|error| error.to_string())?
    } else {
        receipt
    };
    Ok((receipt, extension))
}

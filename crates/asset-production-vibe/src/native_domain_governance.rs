//! Bounded, accession-scoped governance evaluation over retained source bytes.

use std::{borrow::Cow, collections::{BTreeMap, BTreeSet}, sync::atomic::Ordering};

use qualia_core_db::{
    bundle::{semantic_model::{open_semantic_model, SemanticModelBuilder, SEMANTIC_MODEL_KEY},
        BundleReadLimits, BundleReader},
    governance::{
        illocution::{effective_weight, resolve_conflict, Norm, Resolution},
        tk_bc_labels::{evaluate_tk_bc_policy, TkAccessContext, TkBcLabelFlags, TkEvaluationVerdict},
    },
    modalities::{
        logic::{computational_maths_shacl::get_computational_maths_shacl_ttl,
            core_modalities_shacl::get_core_modalities_shacl_ttl,
            infrastructure_shacl::get_infrastructure_shacl_ttl,
            logic_modalities_shacl::get_logic_modalities_shacl_ttl,
            shacl::{shapes_from_turtle, CompiledShape, ShaclConstraint, ShaclEngine, ShaclSeverity},
            specialized_libs_shacl::get_specialized_libs_shacl_ttl},
        socioeconomics::clear_pool_obligations_bps,
    },
    q42_volume::{FLAG_SANCTUARY, FLAG_VOLUME_ROOT, MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE,
        OpfsSliceRangeSource, Q42RangeQueryCursor, Q42RangeQueryPattern, Q42RangeQueryPlan,
        Q42RangeVolume, WasmSliceQ42Reader},
    resolver::{classify_inline_literal, InlineLiteral},
    semantic_instruments::{evaluate_licence_transition_readiness, LicenceTransitionReadiness},
    q_hash, NQuin,
};
use sha2::{Digest, Sha256};

use crate::{fields, list, record, required, text, Args, AssetHost, Resource, Value};
use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;

const SENTINEL: usize = 42 * 1024 * 1024;
const SCRATCH_BYTES: usize = 8 * 1024 * 1024;
const MAX_FACTS: usize = 256;
const MAX_TERM_BYTES: usize = 16 * 1024;
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const EXT: &str = "urn:qualia:production:governance:";
const RECEIPT_CONTEXT: &str = "urn:qualia:production:governance:receipt";
const XSD_INTEGER: &str = "^^<http://www.w3.org/2001/XMLSchema#integer>";
const XSD_BOOLEAN: &str = "^^<http://www.w3.org/2001/XMLSchema#boolean>";

macro_rules! gov {
    ($term:literal) => { concat!("urn:qualia:production:governance:", $term) };
}

macro_rules! commons {
    ($term:literal) => { concat!("https://ns.webcivics.net/permissive-commons/", $term) };
}

macro_rules! modal {
    ($term:literal) => { concat!("https://ns.webcivics.net/modal-juncture/", $term) };
}

impl AssetHost<'_> {
    pub(crate) fn evaluate_agreement_and_pool(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "q42_sha256", "context", "agreement", "usage_event", "credit_state"])?;
        let size = retained(self, required(args, "model")?)?.bytes.len();
        self.charge(size.saturating_mul(4).saturating_add(2_000_000), SCRATCH_BYTES)?;
        let budget = self.limits.resource_bytes.min(SENTINEL - SCRATCH_BYTES);
        let (mut result, receipt) = {
            let admitted = admit_source(self, required(args, "model")?, false)?;
            let mut evidence = Evidence::new(&admitted.q42, Some(required(args, "q42_sha256")?))?;
            let context = text(required(args, "context")?)?;
            let count = evidence.select_context(context)?;
            let (result, mut report) = evaluate_pool(&mut evidence, args, context, count)?;
            add_source_facts(&mut report, &admitted, &evidence, context)?;
            (result, seal_receipt(report, budget, evidence.sanctuary)?)
        };
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        result.insert("attestation_receipt".into(), self.encode("q42", receipt)?);
        Ok(Value::Record(result))
    }

    pub(crate) fn validate_domain_shacl_and_tk_bc(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["model", "source_sha256", "q42_sha256", "context", "domain_checks", "tk_bc", "illocution"])?;
        let model = required(args, "model")?;
        let size = retained(self, model)?.bytes.len();
        self.charge(size.saturating_mul(4).saturating_add(25_000_000), SCRATCH_BYTES)?;
        let budget = self.limits.resource_bytes.min(SENTINEL - SCRATCH_BYTES);
        let (mut result, receipt) = {
            let admitted = admit_source(self, model, true)?;
            if let Some(expected) = args.get("source_sha256") {
                if text(expected)? != admitted.sha256 {
                    return Err("Governance payload does not match source_sha256".into());
                }
            } else if !args.contains_key("q42_sha256") {
                return Err("Governance requires source_sha256 or q42_sha256".into());
            }
            let mut evidence = Evidence::new(&admitted.q42, args.get("q42_sha256"))?;
            let context = text(required(args, "context")?)?;
            let count = evidence.select_context(context)?;
            let checks = args.get("domain_checks").map(list).transpose()?.unwrap_or(&[]);
            if checks.is_empty() && !args.contains_key("tk_bc") && !args.contains_key("illocution") {
                return Err("Governance requires a declared domain, TK/BC, or illocution check".into());
            }
            let mut report = ReceiptGraph::new(RECEIPT_CONTEXT);
            let mut result = BTreeMap::from([
                ("evidence_facts".into(), Value::U64(count as u64)),
                ("q42_sha256".into(), Value::String(evidence.sha256.clone())),
                ("source_sha256".into(), Value::String(admitted.sha256.clone())),
            ]);
            if !checks.is_empty() {
                result.insert("domain_reports".into(), run_domains(&mut evidence, checks, &mut report)?);
            }
            if let Some(accession) = args.get("tk_bc") {
                result.insert("tk_bc".into(), evaluate_labels(&mut evidence, record(accession)?, &mut report)?);
            }
            if let Some(act) = args.get("illocution") {
                result.insert("illocution".into(), evaluate_illocution(&mut evidence, record(act)?, &mut report)?);
            }
            add_source_facts(&mut report, &admitted, &evidence, context)?;
            let report = seal_receipt(report, budget, evidence.sanctuary || args.contains_key("tk_bc"))?;
            (result, report)
        };
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        result.insert("provenance_receipt".into(), self.encode("q42", receipt)?);
        Ok(Value::Record(result))
    }
}

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host.inputs.iter().find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained governance resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

struct AdmittedSource<'a> {
    q42: Cow<'a, [u8]>,
    sha256: String,
}

fn admit_source<'a>(
    host: &'a AssetHost<'_>, input: &Value, allow_spatial: bool,
) -> Result<AdmittedSource<'a>, String> {
    let source = retained(host, input)?;
    if source.bytes.len() > host.limits.resource_bytes {
        return Err("Governance input exceeds the retained resource budget".into());
    }
    let q42 = match source.kind.as_str() {
        "q42" => Cow::Borrowed(source.bytes.as_slice()),
        "hmc" => {
            let bundle = BundleReader::parse_with_limits(&source.bytes, BundleReadLimits {
                file_bytes: host.limits.resource_bytes, entries: host.limits.resources,
                ..BundleReadLimits::default()
            }).map_err(|error| format!("Governance HMC admission: {error}"))?;
            open_semantic_model(&bundle).map_err(|error| error.to_string())?;
            let key = if bundle.entry(SEMANTIC_MODEL_KEY).is_some() {
                SEMANTIC_MODEL_KEY
            } else {
                "manifest.q42"
            };
            if !bundle.verify_entry(key) {
                return Err("Governance HMC Q42 entry digest mismatch".into());
            }
            Cow::Borrowed(bundle.get(key).ok_or("Missing governance HMC semantic model")?)
        }
        "10d" if allow_spatial => {
            let (builder, spatial) = SemanticModelBuilder::new(SENTINEL - SCRATCH_BYTES)
                .merge_10d(&source.bytes).map_err(|error| error.to_string())?;
            if spatial.is_none() {
                return Err("Governance needs a HOLO 10D asset with a complete semantic graph".into());
            }
            Cow::Owned(builder.finish().map_err(|error| error.to_string())?)
        }
        _ => return Err("Governance requires admitted Q42 or HMC semantic evidence".into()),
    };
    Ok(AdmittedSource { q42, sha256: format!("{:x}", Sha256::digest(&source.bytes)) })
}

struct Evidence<'a> {
    reader: WasmSliceQ42Reader<'a>,
    sha256: String,
    sanctuary: bool,
    compressed: Vec<u8>,
    decoded: Vec<u8>,
    lex_page: Vec<u8>,
    lex_term: Vec<u8>,
    facts: [NQuin; MAX_FACTS + 1],
    count: usize,
}

impl<'a> Evidence<'a> {
    fn new(bytes: &'a [u8], digest: Option<&Value>) -> Result<Self, String> {
        if bytes.len() > SENTINEL - SCRATCH_BYTES {
            return Err("Governance Q42 evidence exceeds the bounded symbolic store".into());
        }
        let sha256 = format!("{:x}", Sha256::digest(bytes));
        if let Some(digest) = digest {
            if text(digest)? != sha256 {
                return Err("Selected Q42 bytes do not match q42_sha256".into());
            }
        }
        let reader = WasmSliceQ42Reader::open(bytes).map_err(|error| error.to_string())?;
        if !reader.is_unified_volume() {
            return Err("Governance requires a unified Q42 volume".into());
        }
        let volume = Q42RangeVolume::open(OpfsSliceRangeSource::new(bytes))
            .map_err(|error| error.to_string())?;
        if volume.header().flags & FLAG_VOLUME_ROOT != 0 {
            return Err("Governance evidence requires a self-contained Q42 volume".into());
        }
        let sanctuary = volume.header().flags & FLAG_SANCTUARY != 0;
        Ok(Self {
            reader, sha256, sanctuary,
            compressed: vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE],
            decoded: vec![0; SUPERBLOCK_SIZE],
            lex_page: vec![0; 256 * 1024],
            lex_term: vec![0; MAX_TERM_BYTES],
            facts: [NQuin::default(); MAX_FACTS + 1],
            count: 0,
        })
    }

    fn select_context(&mut self, context: &str) -> Result<usize, String> {
        let context = self.resolve_iri(context)?;
        let page = self.reader.execute_query_page_into(
            Q42RangeQueryPlan::for_pattern(Q42RangeQueryPattern {
                context: Some(context), ..Q42RangeQueryPattern::default()
            }), Q42RangeQueryCursor::default(), &mut self.compressed, &mut self.decoded, &mut self.facts,
        ).map_err(|error| error.to_string())?;
        if page.returned == 0 || page.returned > MAX_FACTS || page.next_cursor.is_some() {
            return Err("Governance accession context requires 1–256 asserted facts".into());
        }
        self.count = page.returned;
        let facts = &self.facts[..self.count];
        if facts.iter().any(|q| q.context != context || !q.verify_ecc_parity()) {
            return Err("Governance evidence failed context or parity verification".into());
        }
        Ok(self.count)
    }

    fn lexical(&mut self, hash: u64) -> Result<Option<&str>, String> {
        let count = self.reader.lookup_lexicon_hash_into(hash, &mut self.lex_page, &mut self.lex_term)
            .map_err(|error| error.to_string())?;
        count.map(|count| std::str::from_utf8(&self.lex_term[..count]).map_err(|error| error.to_string()))
            .transpose()
    }

    fn resolve_iri(&mut self, iri: &str) -> Result<u64, String> {
        let hash = q_hash(iri);
        if !valid_iri(iri) || hash == 0 || self.lexical(hash)? != Some(iri) {
            return Err(format!("No exact admitted Q42 IRI {iri}"));
        }
        Ok(hash)
    }

    fn iri_at(&mut self, hash: u64) -> Result<String, String> {
        let iri = self.lexical(hash)?.ok_or("Governance evidence has no Q42 lexical IRI")?;
        if hash == 0 || !valid_iri(iri) || q_hash(iri) != hash {
            return Err("Governance evidence URI/hash mismatch".into());
        }
        Ok(iri.into())
    }

    fn values(&self, subject: u64, predicate: &str) -> impl Iterator<Item = u64> + '_ {
        let hash = q_hash(predicate);
        self.facts[..self.count].iter().filter(move |q| q.subject == subject && q.predicate == hash)
            .map(|q| q.object)
    }

    fn value(&self, subject: u64, predicate: &str) -> Result<u64, String> {
        let mut values = self.values(subject, predicate);
        let first = values.next().ok_or_else(|| format!("Missing governance evidence {predicate}"))?;
        if values.any(|next| next != first) {
            return Err(format!("Conflicting governance evidence {predicate}"));
        }
        Ok(first)
    }

    fn require_object(&mut self, subject: u64, predicate: &str, object: &str) -> Result<(), String> {
        let expected = self.resolve_iri(object)?;
        let asserted = if predicate == RDF_TYPE {
            self.values(subject, predicate).any(|value| value == expected)
        } else {
            self.value(subject, predicate)? == expected
        };
        if !asserted {
            return Err(format!("Governance evidence {predicate} does not establish {object}"));
        }
        Ok(())
    }

    fn integer(&mut self, subject: u64, predicate: &str) -> Result<i64, String> {
        let hash = self.value(subject, predicate)?;
        let value = match self.lexical(hash)? {
            Some(lexical) if q_hash(lexical) == hash => literal_text(lexical, XSD_INTEGER)?
                .parse::<i64>().map_err(|_| format!("Invalid integer at {predicate}"))?,
            None => match classify_inline_literal(hash) {
                Some(InlineLiteral::Integer(value)) => value,
                _ => return Err(format!("Expected an RDF integer at {predicate}")),
            },
            _ => return Err(format!("Invalid lexical commitment at {predicate}")),
        };
        if value < 0 {
            return Err(format!("Expected a nonnegative integer at {predicate}"));
        }
        Ok(value)
    }

    fn boolean(&mut self, subject: u64, predicate: &str) -> Result<bool, String> {
        let hash = self.value(subject, predicate)?;
        match self.lexical(hash)? {
            Some(lexical) if q_hash(lexical) == hash => match literal_text(lexical, XSD_BOOLEAN)? {
                "true" | "1" => Ok(true),
                "false" | "0" => Ok(false),
                _ => Err(format!("Invalid Boolean at {predicate}")),
            },
            None => match classify_inline_literal(hash) {
                Some(InlineLiteral::Boolean(value)) => Ok(value),
                _ => Err(format!("Expected an RDF Boolean at {predicate}")),
            },
            _ => Err(format!("Invalid lexical commitment at {predicate}")),
        }
    }
}

fn valid_iri(iri: &str) -> bool {
    let Some((scheme, local)) = iri.split_once(':') else { return false };
    scheme.as_bytes().first().is_some_and(u8::is_ascii_alphabetic)
        && scheme.bytes().all(|c| c.is_ascii_alphanumeric() || b"+-.".contains(&c))
        && !local.is_empty()
        && !local.chars().any(|c| c.is_control() || c.is_whitespace() || "<>\"{}|\\^".contains(c)
            || c == char::from(0x60))
}

fn literal_text<'a>(lexical: &'a str, datatype: &str) -> Result<&'a str, String> {
    lexical.strip_suffix(datatype).and_then(|s| s.strip_prefix('"'))
        .and_then(|s| s.strip_suffix('"')).ok_or("RDF literal has the wrong exact datatype".into())
}

fn add_source_facts(
    report: &mut ReceiptGraph, source: &AdmittedSource<'_>, evidence: &Evidence<'_>, context: &str,
) -> Result<(), String> {
    report.fact(RECEIPT_CONTEXT, "http://www.w3.org/ns/prov#wasDerivedFrom", context)?;
    report.fact(RECEIPT_CONTEXT, &format!("{EXT}sourceDigest"), &format!("urn:sha256:{}", source.sha256))?;
    report.fact(RECEIPT_CONTEXT, &format!("{EXT}q42Digest"), &format!("urn:sha256:{}", evidence.sha256))
}

fn seal_receipt(report: ReceiptGraph, budget: usize, sanctuary: bool) -> Result<Vec<u8>, String> {
    let bytes = report.seal(budget)?;
    if !sanctuary {
        return Ok(bytes);
    }
    let mut restricted = SemanticModelBuilder::new(budget);
    restricted.restrict_to_sanctuary();
    restricted.merge_q42(&bytes).and_then(SemanticModelBuilder::finish)
        .map_err(|error| error.to_string())
}

fn evaluate_pool(
    evidence: &mut Evidence<'_>, args: &Args, context: &str, count: usize,
) -> Result<(Args, ReceiptGraph), String> {
    let agreement_iri = text(required(args, "agreement")?)?;
    let event_iri = text(required(args, "usage_event")?)?;
    let state_iri = text(required(args, "credit_state")?)?;
    let agreement = evidence.resolve_iri(agreement_iri)?;
    let event = evidence.resolve_iri(event_iri)?;
    let state = evidence.resolve_iri(state_iri)?;
    evidence.require_object(agreement, RDF_TYPE, commons!("Work"))?;
    evidence.require_object(agreement, commons!("dischargeModel"), commons!("CollectivePool"))?;
    evidence.require_object(event, RDF_TYPE, gov!("UsageEvent"))?;
    evidence.require_object(state, RDF_TYPE, gov!("CreditAllocationState"))?;
    evidence.require_object(event, gov!("agreement"), agreement_iri)?;
    evidence.require_object(event, gov!("creditState"), state_iri)?;
    evidence.require_object(state, gov!("agreement"), agreement_iri)?;
    let author_hashes: BTreeSet<_> = evidence.values(agreement, commons!("attributedTo")).collect();
    if author_hashes.is_empty() {
        return Err("Permissive Commons work requires admitted attribution".into());
    }
    let mut attribution = author_hashes.into_iter().map(|author| evidence.iri_at(author))
        .collect::<Result<Vec<_>, _>>()?;
    attribution.sort_unstable();
    let unit = evidence.value(agreement, commons!("unitOfAccount"))?;
    let unit = evidence.iri_at(unit)?;
    let period = evidence.value(state, gov!("period"))?;
    let period = evidence.iri_at(period)?;
    evidence.require_object(event, gov!("period"), &period)?;
    evidence.require_object(event, commons!("unitOfAccount"), &unit)?;
    evidence.require_object(state, commons!("unitOfAccount"), &unit)?;
    let event_class = evidence.value(event, gov!("usageClass"))?;
    let event_class = evidence.iri_at(event_class)?;
    let gross = evidence.integer(event, gov!("grossReceiptCents"))?;
    let now_unix = bounded_u32(evidence, event, gov!("observedAtUnix"))?;
    let allowed = match event_class.as_str() {
        gov!("Commercial") => evidence.boolean(agreement, gov!("commercialUseAllowed"))?,
        gov!("HumanRightsNoCharge") => {
            let grant = evidence.value(event, gov!("humanRightsBasis"))?;
            let grant_iri = evidence.iri_at(grant)?;
            evidence.require_object(grant, RDF_TYPE, gov!("HumanRightsAccessAccession"))?;
            evidence.require_object(grant, gov!("agreement"), agreement_iri)?;
            evidence.require_object(grant, gov!("usageEvent"), event_iri)?;
            evidence.require_object(grant, gov!("period"), &period)?;
            evidence.require_object(event, gov!("humanRightsBasis"), &grant_iri)?;
            if gross != 0 {
                return Err("Human-rights no-charge evidence carries a nonzero gross receipt".into());
            }
            evidence.boolean(agreement, gov!("humanRightsNoChargeAllowed"))?
        }
        _ => return Err("Unsupported governance usage class".into()),
    };
    let prior = evidence.integer(state, gov!("poolFundedCents"))?;
    let target = evidence.integer(agreement, gov!("targetObligationCents"))?;
    let settled = evidence.integer(state, gov!("settledVerifiedCents"))?;
    let waived = evidence.integer(state, gov!("waivedObligationCents"))?;
    let reserve_bps = bounded_u32(evidence, agreement, gov!("operatingReserveBps"))?;
    let fiduciary_bps = bounded_u32(evidence, agreement, gov!("fiduciaryFeeBps"))?;
    if target == 0 || settled.checked_add(waived).is_none() || prior.checked_add(gross).is_none() {
        return Err("Pool requires a positive target and bounded exact integer credits".into());
    }
    let pool_gross = if allowed { gross } else { 0 };
    let clearance = clear_pool_obligations_bps(
        pool_gross, reserve_bps, fiduciary_bps, prior, target, settled, waived,
    ).ok_or("Pool clearing rejected the admitted basis-point configuration")?;
    let credits = clearance.operating_reserve_cents.checked_add(clearance.fiduciary_fee_cents)
        .and_then(|amount| amount.checked_add(clearance.obligation_pool_allocation_cents));
    if clearance.obligation_pool_allocation_cents < 0 || credits != Some(pool_gross)
        || prior.checked_add(clearance.obligation_pool_allocation_cents) != Some(clearance.pool_funded_cents)
    {
        return Err("Native rounded pool components do not conserve the exact gross receipt".into());
    }
    let steward = evidence.value(agreement, gov!("authorisedHumanSteward"))?;
    let steward = evidence.iri_at(steward)?;
    if !steward.starts_with("did:") {
        return Err("Licence transition steward requires an admitted DID".into());
    }
    let transition = evaluate_licence_transition_readiness(&LicenceTransitionReadiness {
        project_id: agreement_iri.into(),
        pool_funded_cents: clearance.pool_funded_cents,
        required_obligation_cents: target,
        settled_verified_cents: settled,
        waived_obligation_cents: waived,
        dependencies_compatible: evidence.boolean(agreement, gov!("dependenciesCompatible"))?,
        unresolved_material_challenges: bounded_u32(evidence, agreement, gov!("unresolvedMaterialChallenges"))?,
        authorised_human_steward_did: Some(steward),
    }, now_unix);
    if !transition.receipt_quin.verify_ecc_parity() {
        return Err("Native instrument output failed NQuin parity".into());
    }
    let outcome = if allowed { "permitted_by_accession" } else { "denied_by_accession" };
    let cents = [
        ("gross_receipt_cents", pool_gross),
        ("operating_reserve_cents", clearance.operating_reserve_cents),
        ("fiduciary_fee_cents", clearance.fiduciary_fee_cents),
        ("pool_allocation_cents", clearance.obligation_pool_allocation_cents),
        ("prior_pool_funded_cents", prior),
        ("pool_funded_cents", clearance.pool_funded_cents),
        ("settled_verified_cents", settled),
        ("waived_obligation_cents", waived),
        ("remaining_obligation_cents", clearance.remaining_obligation_cents),
    ];
    let mut receipt = ReceiptGraph::new(RECEIPT_CONTEXT);
    for (predicate, object) in [
        (gov!("agreement"), agreement_iri), (gov!("usageEvent"), event_iri),
        (gov!("creditState"), state_iri), (gov!("period"), &period),
        (commons!("unitOfAccount"), &unit),
        (gov!("declaredEvidenceContext"), context),
        (gov!("usageClass"), &event_class),
    ] {
        receipt.fact(RECEIPT_CONTEXT, predicate, object)?;
    }
    for author in &attribution {
        receipt.fact(RECEIPT_CONTEXT, commons!("attributedTo"), author)?;
    }
    receipt.fact(RECEIPT_CONTEXT, gov!("decision"), &format!("{EXT}{outcome}"))?;
    receipt.fact(RECEIPT_CONTEXT, gov!("transitionOutcome"), &format!("{EXT}{}", transition.outcome.as_str()))?;
    for (field, cents) in cents {
        receipt.fact(RECEIPT_CONTEXT, &format!("{EXT}{field}"), &format!("urn:qualia:cents:{cents}"))?;
    }
    let findings = transition.findings.into_iter().map(|finding| Value::Record(BTreeMap::from([
        ("code".into(), Value::String(finding.code)),
        ("outcome".into(), Value::String(finding.outcome.as_str().into())),
        ("message".into(), Value::String(finding.message)),
    ]))).collect();
    let result = BTreeMap::from([
        ("outcome".into(), Value::String(outcome.into())),
        ("usage_class".into(), Value::String(event_class)),
        ("period".into(), Value::String(period)),
        ("unit".into(), Value::String(unit)),
        ("attribution".into(), Value::List(attribution.into_iter().map(Value::String).collect())),
        ("evidence_facts".into(), Value::U64(count as u64)),
        ("q42_sha256".into(), Value::String(evidence.sha256.clone())),
        ("contributor_obligations_discharged".into(), Value::Bool(clearance.contributor_obligations_discharged)),
        ("transition_outcome".into(), Value::String(transition.outcome.as_str().into())),
        ("transition_findings".into(), Value::List(findings)),
        ("pool".into(), Value::Record(cents.into_iter()
            .map(|(name, amount)| (name.to_owned(), Value::I64(amount))).collect())),
    ]);
    Ok((result, receipt))
}

fn bounded_u32(evidence: &mut Evidence<'_>, subject: u64, path: &str) -> Result<u32, String> {
    u32::try_from(evidence.integer(subject, path)?).map_err(|_| format!("Integer range exceeded at {path}"))
}

fn evaluate_labels(
    evidence: &mut Evidence<'_>, args: &Args, receipt: &mut ReceiptGraph,
) -> Result<Value, String> {
    const KNOWN: u32 = TkBcLabelFlags::TK_ATTRIBUTION.0 | TkBcLabelFlags::TK_NON_COMMERCIAL.0
        | TkBcLabelFlags::TK_CULTURALLY_SENSITIVE.0 | TkBcLabelFlags::TK_COMMUNITY_VERIFIED.0
        | TkBcLabelFlags::BC_PROVENANCE.0 | TkBcLabelFlags::BC_MULTIPLE_COMMUNITY.0;
    fields(args, &["asset", "accession"])?;
    let asset_iri = text(required(args, "asset")?)?;
    let accession_iri = text(required(args, "accession")?)?;
    let asset = evidence.resolve_iri(asset_iri)?;
    let accession = evidence.resolve_iri(accession_iri)?;
    evidence.require_object(accession, RDF_TYPE, gov!("TkBcAccessAccession"))?;
    evidence.require_object(accession, gov!("governedAsset"), asset_iri)?;
    let bits = bounded_u32(evidence, asset, gov!("tkBcLabelMask"))?;
    if bits & !KNOWN != 0 {
        return Err("TK/BC label evidence includes an unknown native flag".into());
    }
    let community = evidence.value(accession, gov!("attributingCommunity"))?;
    let community = evidence.iri_at(community)?;
    let access = TkAccessContext {
        is_commercial_use: evidence.boolean(accession, gov!("commercialUse"))?,
        has_attribution: evidence.boolean(accession, gov!("communityAttributionRecorded"))?,
        community_consent_signed: evidence.boolean(accession, gov!("communityConsentAttested"))?,
        observer_is_verified_custodian: evidence.boolean(accession, gov!("verifiedCustodianAttested"))?,
    };
    let verdict = evaluate_tk_bc_policy(TkBcLabelFlags(bits), &access);
    let outcome = match verdict {
        TkEvaluationVerdict::Permitted => "permitted",
        TkEvaluationVerdict::ProhibitedCommercialWithoutConsent => "prohibited_commercial_without_consent",
        TkEvaluationVerdict::ProhibitedMissingAttribution => "prohibited_missing_attribution",
        TkEvaluationVerdict::RestrictedCulturalSensitivity => "restricted_cultural_sensitivity",
    };
    receipt.fact(RECEIPT_CONTEXT, gov!("tkBcAccession"), accession_iri)?;
    receipt.fact(accession_iri, gov!("governedAsset"), asset_iri)?;
    receipt.fact(accession_iri, gov!("attributingCommunity"), &community)?;
    receipt.fact(accession_iri, gov!("tkBcLabelMask"), &format!("urn:qualia:labels:{bits}"))?;
    receipt.fact(accession_iri, gov!("tkBcDecision"), &format!("{EXT}{outcome}"))?;
    Ok(Value::Record(BTreeMap::from([
        ("asset".into(), Value::String(asset_iri.into())),
        ("accession".into(), Value::String(accession_iri.into())),
        ("attributing_community".into(), Value::String(community)),
        ("labels".into(), Value::U64(u64::from(bits))),
        ("outcome".into(), Value::String(outcome.into())),
        ("permitted".into(), Value::Bool(verdict == TkEvaluationVerdict::Permitted)),
    ])))
}

fn evaluate_illocution(
    evidence: &mut Evidence<'_>, args: &Args, receipt: &mut ReceiptGraph,
) -> Result<Value, String> {
    fields(args, &["left", "right"])?;
    let left_iri = text(required(args, "left")?)?;
    let right_iri = text(required(args, "right")?)?;
    let left = evidence.resolve_iri(left_iri)?;
    let right = evidence.resolve_iri(right_iri)?;
    if left == right {
        return Err("Illocution conflict evaluation requires distinct authored acts".into());
    }
    for actor in [left, right] {
        evidence.require_object(actor, RDF_TYPE, gov!("IllocutionaryAct"))?;
    }
    for path in [gov!("party"), gov!("action")] {
        let bound = evidence.value(left, path)?;
        let bound_iri = evidence.iri_at(bound)?;
        evidence.require_object(right, path, &bound_iri)?;
        receipt.fact(RECEIPT_CONTEXT, path, &bound_iri)?;
    }
    let a = read_norm(evidence, left)?;
    let b = read_norm(evidence, right)?;
    let resolution = match resolve_conflict(a, b) {
        Resolution::AGoverns => "left_governs",
        Resolution::BGoverns => "right_governs",
        Resolution::GenuineConflict => "genuine_conflict",
    };
    receipt.fact(RECEIPT_CONTEXT, gov!("leftAct"), left_iri)?;
    receipt.fact(RECEIPT_CONTEXT, gov!("rightAct"), right_iri)?;
    receipt.fact(RECEIPT_CONTEXT, gov!("illocutionResolution"), &format!("{EXT}{resolution}"))?;
    Ok(Value::Record(BTreeMap::from([
        ("left".into(), Value::String(left_iri.into())),
        ("right".into(), Value::String(right_iri.into())),
        ("left_weight".into(), Value::U64(u64::from(a.weight))),
        ("right_weight".into(), Value::U64(u64::from(b.weight))),
        ("resolution".into(), Value::String(resolution.into())),
    ])))
}

fn read_norm(evidence: &mut Evidence<'_>, act: u64) -> Result<Norm, String> {
    let base = u8::try_from(evidence.integer(act, modal!("bindingWeight"))?)
        .map_err(|_| "Illocution binding weight must fit the native force band")?;
    let speaker = u8::try_from(evidence.integer(act, gov!("speakerAuthority"))?)
        .map_err(|_| "Illocution speaker authority must fit the native force band")?;
    let scaled = evidence.boolean(act, modal!("authorityScaled"))?;
    let immune = evidence.boolean(act, gov!("immuneNorm"))?;
    let state = evidence.value(act, modal!("engineState"))?;
    let state = evidence.iri_at(state)?;
    let exemptive = match state.as_str() {
        modal!("DefeaterUnless") => true,
        modal!("ObligateSelf") | modal!("ObligateSelfLiability") | modal!("ObligateTarget")
        | modal!("Recommend") | modal!("Permit") | modal!("Forbid") | modal!("ForbidTemporal") => false,
        _ => return Err("Illocution act is outside the supported deontic conflict states".into()),
    };
    if immune && exemptive {
        return Err("One illocution act cannot claim both immunity and exemption".into());
    }
    Ok(Norm { weight: effective_weight(base, scaled, speaker), exemptive, immune })
}

fn domain_source(domain: &str) -> Result<&'static str, String> {
    match domain {
        "logic" => Ok(get_logic_modalities_shacl_ttl()),
        "core" => Ok(get_core_modalities_shacl_ttl()),
        "math" => Ok(get_computational_maths_shacl_ttl()),
        "specialized_libs" => Ok(get_specialized_libs_shacl_ttl()),
        "geometry" => Ok(include_str!("../../../../qualiaDB/crates/qualia-core-db/shapes/geometry-asset.shacl.ttl")),
        "infrastructure" => Ok(get_infrastructure_shacl_ttl()),
        _ => Err(format!("Unknown native SHACL domain {domain}")),
    }
}

fn expand_term(source: &str, name: &str) -> String {
    if let Some(iri) = name.strip_prefix('<').and_then(|s| s.strip_suffix('>')) {
        return iri.into();
    }
    if let Some((prefix, local)) = name.split_once(':') {
        for line in source.lines().map(str::trim) {
            let Some(after) = line.strip_prefix("@prefix ") else { continue };
            let Some((head, namespace)) = after.split_once('<') else { continue };
            if head.trim().strip_suffix(':') == Some(prefix) {
                if let Some((namespace, _)) = namespace.split_once('>') {
                    return format!("{namespace}{local}");
                }
            }
        }
    }
    name.into()
}

fn supported_shape(source: &str, shape: &CompiledShape) -> Result<CompiledShape, String> {
    let mut shape = shape.clone();
    shape.shape_class = expand_term(source, &shape.shape_class);
    if !shape.property_path.is_empty() {
        shape.property_path = expand_term(source, &shape.property_path);
        if !valid_iri(&shape.property_path) {
            return Err("SHACL domain check requires a simple canonical predicate path".into());
        }
    }
    for constraint in &mut shape.constraints {
        match constraint {
            ShaclConstraint::Class(class) => *class = expand_term(source, class),
            ShaclConstraint::DataType(datatype) if matches!(datatype.as_str(),
                "xsd:string" | "xsd:integer" | "xsd:decimal" | "xsd:float" | "xsd:boolean") => {}
            ShaclConstraint::NodeKind(kind) if matches!(kind.as_str(), "sh:IRI") => {}
            ShaclConstraint::In(values) => {
                for value in values {
                    *value = expand_term(source, value);
                }
            }
            ShaclConstraint::MinInclusive(_) | ShaclConstraint::MaxInclusive(_)
            | ShaclConstraint::MinExclusive(_) | ShaclConstraint::MaxExclusive(_)
            | ShaclConstraint::MinCount(_) | ShaclConstraint::MaxCount(_)
            | ShaclConstraint::MinLength(_) | ShaclConstraint::MaxLength(_)
            | ShaclConstraint::Pattern(_) => {}
            _ => return Err("Native SHACL shape uses an unavailable bounded constraint".into()),
        }
    }
    Ok(shape)
}

fn selected_shape_source(document: &str, requested: &str) -> Result<(String, String), String> {
    let canonical = expand_term(document, requested);
    let (spelling, start) = document.split_inclusive('\n').scan(0, |offset, line| {
        let current = *offset;
        *offset += line.len();
        Some((current, line))
    }).find_map(|(offset, line)| {
        let mut terms = line.split_whitespace();
        let subject = terms.next()?;
        (terms.next() == Some("a") && terms.next() == Some("sh:NodeShape")
            && expand_term(document, subject) == canonical).then_some((subject, offset))
    }).ok_or_else(|| format!("No native declaration for shape {requested}"))?;
    let statement = &document[start..];
    let end = statement_end(statement)
        .ok_or("SHACL domain shape is missing its statement terminator")?;
    let statement = &statement[..end];
    if statement.len() > 16 * 1024 {
        return Err("SHACL domain declaration exceeds the bounded shape source".into());
    }
    let mut normalized = String::with_capacity(statement.len() + 1024);
    let mut properties = Vec::new();
    let mut remaining = statement;
    while let Some(open) = outside_quotes(remaining, '[') {
        let prefix = &remaining[..open];
        if !prefix.trim_end().ends_with("sh:property") || properties.len() == 64 {
            return Err("SHACL domain shape uses an unsupported anonymous structure".into());
        }
        let contents = &remaining[open + 1..];
        let close = outside_quotes(contents, ']').ok_or("Unclosed native SHACL property shape")?;
        let body = &contents[..close];
        if outside_quotes(body, '[').is_some() {
            return Err("Nested anonymous SHACL shape is unavailable in this bounded mode".into());
        }
        let name = format!("qualia-inline:property-{}", properties.len());
        normalized.push_str(prefix);
        normalized.push_str(&name);
        properties.push((name, body));
        remaining = &contents[close + 1..];
    }
    normalized.push_str(remaining);
    for (name, body) in properties {
        let body = body.trim().trim_end_matches(';').trim_end();
        normalized.push('\n');
        normalized.push_str(&name);
        normalized.push(' ');
        normalized.push_str(body);
        normalized.push_str(" .");
    }
    if ["sh:sparql", "sh:targetNode", "sh:targetObjectsOf", "sh:targetSubjectsOf", "sh:deactivated"]
        .iter().any(|term| normalized.contains(*term))
    {
        return Err("Selected native SHACL shape requires an unavailable target or execution mode".into());
    }
    let normalized = normalize_membership(&normalized)?;
    Ok((spelling.to_owned(), normalized))
}

fn statement_end(statement: &str) -> Option<usize> {
    visible_chars(statement).find_map(|(i, ch)| {
        if ch == '.' && i > 0 && statement.as_bytes()[i - 1].is_ascii_whitespace()
            && statement.get(i + 1..).is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace))
        {
            Some(i + 1)
        } else {
            None
        }
    })
}

fn visible_chars(text: &str) -> impl Iterator<Item = (usize, char)> + '_ {
    let mut quoted = false;
    let mut escaped = false;
    text.char_indices().filter(move |&(_, ch)| {
        if escaped {
            escaped = false;
            return false;
        }
        if quoted && ch == '\\' {
            escaped = true;
            return false;
        }
        if ch == '"' {
            quoted = !quoted;
            return false;
        }
        !quoted
    })
}

fn outside_quotes(text: &str, needle: char) -> Option<usize> {
    visible_chars(text).find_map(|(i, ch)| (ch == needle).then_some(i))
}

fn normalize_membership(source: &str) -> Result<String, String> {
    let mut normalized = String::with_capacity(source.len());
    let mut remaining = source;
    while let Some(start) = visible_chars(remaining).find_map(|(i, ch)|
        (ch == '(' && remaining[..i].trim_end().ends_with("sh:in")).then_some(i))
    {
        let list = &remaining[start + 1..];
        let end = outside_quotes(list, ')').ok_or("Unclosed native SHACL membership list")?;
        let list = &list[..end];
        let mut items = list.split_whitespace().peekable();
        if items.peek().is_none() {
            return Err("Empty native SHACL membership list is unsupported".into());
        }
        normalized.push_str(&remaining[..start]);
        for (index, token) in items.enumerate() {
            let quoted = token.starts_with('"') && token.ends_with('"') && token.len() > 1;
            if token.contains(['(', ')']) || (!quoted && !valid_iri(token)) {
                return Err("Native SHACL membership requires unambiguous string or IRI members".into());
            }
            if index != 0 {
                normalized.push_str(", ");
            }
            normalized.push_str(token);
        }
        remaining = &remaining[start + end + 2..];
    }
    normalized.push_str(remaining);
    Ok(normalized)
}

fn verify_compiled_shape_surface(source: &str, shapes: &[CompiledShape]) -> Result<(), String> {
    let unquoted: String = visible_chars(source).map(|(_, ch)| ch).collect();
    let mut constraints = 0usize;
    let mut paths = 0usize;
    for token in unquoted.split_whitespace() {
        let token = token.trim_end_matches([';', ',', '.']);
        match token {
            "sh:path" => paths += 1,
            "sh:minInclusive" | "sh:maxInclusive" | "sh:minExclusive" | "sh:maxExclusive"
            | "sh:minCount" | "sh:maxCount" | "sh:minLength" | "sh:maxLength" | "sh:pattern"
            | "sh:in" | "sh:class" | "sh:datatype" | "sh:nodeKind" => constraints += 1,
            "sh:NodeShape" | "sh:property" | "sh:targetClass" | "sh:message" | "sh:severity"
            | "sh:name" | "sh:description" | "sh:IRI" | "sh:Violation" => {}
            _ if token.starts_with("sh:") => {
                return Err(format!("Unadmitted constraint or shape term {token}"));
            }
            _ => {}
        }
    }
    if constraints != shapes.iter().map(|shape| shape.constraints.len()).sum::<usize>()
        || paths != shapes.iter().filter(|shape| !shape.property_path.is_empty()).count()
    {
        return Err("Native SHACL compiler did not preserve the full selected constraint surface".into());
    }
    Ok(())
}

struct DomainFacts {
    facts: Vec<NQuin>,
    lexical: BTreeMap<u64, String>,
    strings: BTreeSet<u64>,
}

fn domain_facts(evidence: &mut Evidence<'_>) -> Result<DomainFacts, String> {
    use qualia_core_db::resolver::pack_typed_literal;

    let mut facts = evidence.facts[..evidence.count].to_vec();
    let mut lexicon = BTreeMap::new();
    let mut strings = BTreeSet::new();
    let mut iris = BTreeSet::new();
    let mut lexical_bytes = 0usize;
    for fact in &mut facts {
        for hash in [fact.subject, fact.predicate, fact.object] {
            if lexicon.contains_key(&hash) {
                continue;
            }
            if let Some(spelling) = evidence.lexical(hash)? {
                if q_hash(spelling) != hash {
                    return Err("SHACL evidence term does not match Q42LEX".into());
                }
                lexical_bytes += spelling.len();
                if spelling.len() > 512 || lexical_bytes > 256 * 1024 {
                    return Err("SHACL lexical evidence exceeds the per-term or aggregate bound".into());
                }
                if valid_iri(spelling) {
                    iris.insert(hash);
                }
                lexicon.insert(hash, spelling.to_string());
            }
        }
        if let Some(spelling) = lexicon.get(&fact.object) {
            let mut normalized_string = None;
            if let Some((value, datatype)) = spelling.rsplit_once("^^<")
                .and_then(|(value, datatype)| Some((value, datatype.strip_suffix('>')?)))
            {
                let value = serde_json::from_str::<String>(value).map_err(|error| error.to_string())?;
                if datatype == "http://www.w3.org/2001/XMLSchema#integer"
                    || datatype == "http://www.w3.org/2001/XMLSchema#boolean"
                {
                    fact.object = pack_typed_literal(&value, datatype)
                        .ok_or("SHACL exact RDF integer or Boolean exceeds its native encoding")?;
                } else if datatype == "http://www.w3.org/2001/XMLSchema#string" {
                    normalized_string = Some(value);
                }
            } else if spelling.starts_with('"') && spelling.ends_with('"') {
                normalized_string = Some(serde_json::from_str::<String>(spelling).map_err(|error| error.to_string())?);
            }
            if let Some(value) = normalized_string {
                let hash = q_hash(&value);
                if let Some(other) = lexicon.insert(hash, value.clone()) {
                    if other != value {
                        return Err("SHACL lexical normalization collides with another Q42 term".into());
                    }
                }
                fact.object = hash;
                strings.insert(hash);
            }
        }
        fact.parity = NQuin::calculate_parity(fact.subject, fact.predicate, fact.object, fact.context, fact.metadata);
    }
    if strings.iter().any(|hash| iris.contains(hash)) {
        return Err("SHACL string normalization collides with an IRI in the selected evidence".into());
    }
    // ShaclEngine resolves class membership using its native rdf:type aliases.
    // Keep the canonical predicate as well for shapes that select it explicitly.
    let type_aliases: Vec<_> = facts.iter().filter(|fact| fact.predicate == q_hash(RDF_TYPE))
        .map(|fact| {
            let predicate = q_hash("rdf:type");
            NQuin { predicate, parity: NQuin::calculate_parity(
                fact.subject, predicate, fact.object, fact.context, fact.metadata,
            ), ..*fact }
        }).collect();
    facts.extend(type_aliases);
    facts.sort_unstable_by_key(|q| (q.subject, q.predicate, q.object));
    facts.dedup_by_key(|q| (q.subject, q.predicate, q.object));
    Ok(DomainFacts { facts, lexical: lexicon, strings })
}

fn run_domains(
    evidence: &mut Evidence<'_>, checks: &[Value], receipt: &mut ReceiptGraph,
) -> Result<Value, String> {
    if checks.len() > 12 {
        return Err("SHACL admits at most twelve requested domain checks".into());
    }
    let DomainFacts { facts, lexical, strings } = domain_facts(evidence)?;
    let resolve = |hash: u64| lexical.get(&hash).cloned();
    let mut reports = Vec::with_capacity(checks.len());
    for check in checks {
        let args = record(check)?;
        fields(args, &["domain", "shape", "focus"])?;
        let domain = text(required(args, "domain")?)?;
        let requested_shape = text(required(args, "shape")?)?;
        let focus_iri = text(required(args, "focus")?)?;
        let focus = evidence.resolve_iri(focus_iri)?;
        if !facts.iter().any(|q| q.subject == focus) {
            return Err("Requested SHACL focus has no assertions in the admitted context".into());
        }
        let source = domain_source(domain)?;
        let shape_document_sha = format!("{:x}", Sha256::digest(source.as_bytes()));
        let (source_name, normalized) = selected_shape_source(source, requested_shape)?;
        let shapes = shapes_from_turtle(&normalized)?;
        let canonical_shape = expand_term(source, requested_shape);
        let mut selected = shapes.iter().filter(|shape| shape.name.as_deref() == Some(source_name.as_str()))
            .map(|shape| supported_shape(source, shape)).collect::<Result<Vec<_>, _>>()?;
        if selected.is_empty() || !valid_iri(&canonical_shape) {
            return Err(format!("No compiled named shape {requested_shape} in domain {domain}"));
        }
        verify_compiled_shape_surface(&normalized, &selected)?;
        selected.sort_by(|left, right| left.property_path.cmp(&right.property_path));
        let mut findings = Vec::new();
        let engine = ShaclEngine::new(&facts, &selected);
        for shape in &selected {
            if shape.constraints.len() > 8 {
                return Err("Selected SHACL path exceeds the eight-constraint evaluation bound".into());
            }
            if shape.shape_class != canonical_shape
                && !facts.iter().any(|q| q.subject == focus && q.predicate == q_hash(RDF_TYPE)
                    && q.object == q_hash(&shape.shape_class))
            {
                return Err(format!("SHACL focus has no admitted target class {}", shape.shape_class));
            }
            engine.validate_focus(focus, shape, &resolve, &mut findings);
            verify_lexical_constraints(shape, focus, focus_iri, &facts, &lexical, &strings, &mut findings);
            if findings.len() > 256 {
                return Err("SHACL reporting exceeds 256 findings per check".into());
            }
        }
        let conforms = !findings.iter().any(|finding| finding.severity == ShaclSeverity::Violation);
        let rows = findings.into_iter().map(|finding| Value::Record(BTreeMap::from([
            ("severity".into(), Value::String(format!("{:?}", finding.severity))),
            ("focus".into(), Value::String(finding.focus_node)),
            ("path".into(), finding.result_path.map_or(Value::Null, Value::String)),
            ("constraint".into(), finding.source_constraint_component.map_or(Value::Null, Value::String)),
            ("message".into(), finding.message.map_or(Value::Null, Value::String)),
        ]))).collect();
        let mut digest = Sha256::new();
        for lexical in [
            evidence.sha256.as_str(), shape_document_sha.as_str(), focus_iri, canonical_shape.as_str(),
        ] {
            digest.update((lexical.len() as u64).to_le_bytes());
            digest.update(lexical.as_bytes());
        }
        let check_id = format!("urn:sha256:{:x}", digest.finalize());
        receipt.fact(RECEIPT_CONTEXT, gov!("shaclCheck"), &check_id)?;
        receipt.fact(&check_id, gov!("shaclFocus"), focus_iri)?;
        receipt.fact(&check_id, gov!("shaclShape"), &canonical_shape)?;
        receipt.fact(&check_id, gov!("shapeDocumentDigest"), &format!("urn:sha256:{shape_document_sha}"))?;
        receipt.fact(&check_id, gov!("shaclOutcome"), if conforms { gov!("Conforms") } else { gov!("Violates") })?;
        reports.push(Value::Record(BTreeMap::from([
            ("domain".into(), Value::String(domain.into())),
            ("shape".into(), Value::String(canonical_shape)),
            ("shape_document_sha256".into(), Value::String(shape_document_sha)),
            ("focus".into(), Value::String(focus_iri.into())),
            ("conforms".into(), Value::Bool(conforms)),
            ("findings".into(), Value::List(rows)),
        ])));
    }
    Ok(Value::List(reports))
}

fn verify_lexical_constraints(
    shape: &CompiledShape, focus: u64, focus_iri: &str, facts: &[NQuin],
    lexical: &BTreeMap<u64, String>, strings: &BTreeSet<u64>,
    findings: &mut Vec<qualia_core_db::modalities::logic::shacl::ValidationResult>,
) {
    let path = q_hash(&shape.property_path);
    for constraint in &shape.constraints {
        let checks_string = matches!(constraint, ShaclConstraint::DataType(datatype) if datatype == "xsd:string");
        let checks_iri = matches!(constraint, ShaclConstraint::NodeKind(kind) if kind == "sh:IRI");
        if !checks_string && !checks_iri {
            continue;
        }
        let matches = |term: u64| {
            if checks_string { strings.contains(&term) }
            else { !strings.contains(&term) && lexical.get(&term).is_some_and(|value| valid_iri(value)) }
        };
        let values = facts.iter().filter(|q| q.subject == focus && q.predicate == path).map(|q| q.object);
        let focus_value = shape.property_path.is_empty().then_some(focus);
        for term in values.chain(focus_value) {
            if !matches(term) {
                findings.push(qualia_core_db::modalities::logic::shacl::ValidationResult {
                    severity: shape.severity,
                    focus_node: focus_iri.into(),
                    result_path: (!shape.property_path.is_empty()).then_some(shape.property_path.clone()),
                    message: Some(if checks_string { "Q42 value lacks exact RDF string lexical evidence" }
                        else { "Q42 value lacks canonical RDF IRI lexical evidence" }.into()),
                    source_constraint: shape.name.clone(),
                    source_constraint_component: Some(if checks_string { "sh:DatatypeConstraintComponent" }
                        else { "sh:NodeKindConstraintComponent" }.into()),
                    value: lexical.get(&term).cloned(),
                });
            }
        }
    }
}

//! Offline Poet event-grant validation using a pinned HMC and public key.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.
//!
//! In the mounted `ActionAuraPolicy` named graph, the action links to each
//! required positive fact with `requiresAuraFact`. Each linked declaration
//! has exactly one `factSubject`, `factPredicate`, `factObject`, and
//! `factGraph` in that graph. The detached signed Aura volume must contain
//! exactly the declared fact set, with the same six-column rows as the HMC.
//! In the `EventGrantAuthority` named graph, the actor DID must have an
//! `actorEd25519KeySha256` assertion pointing to `urn:sha256:<key-digest>`
//! and an `actorGrantedCapability` assertion naming the exact EONT capability.
//! The signature authenticates the event binding. The credential gate uses
//! that binding as a scoped claim and does not issue or verify a separate VC.

use std::collections::BTreeSet;
use std::sync::atomic::Ordering;

use bytemuck::Zeroable;
use qualia_core_db::{
    bundle::{semantic_model::open_semantic_model, BundleReadLimits, BundleReader},
    container_10d::{
        decode_entity_ontology_section_into, integrity::check_whole_file_crc32c,
        AffordanceBinaryRecord, SectionType, TypedFieldRecord, TypedPayloadResolver,
        MANIFOLD_IDENTITY_V2_MAX_FIELDS, MAX_AFFORDANCES_PER_SECTION,
    },
    crypto::network::signing_profile::verify_ed25519_raw_strict,
    identifier::parse_did_q42,
    identity::{did_qi::parse_did_qi_uri, webizen_did::parse_did_webizen},
    modalities::logic::shacl_extensions::{credential_gates_target, CredentialGate},
    q42_volume::{
        Q42RangeQueryCursor, Q42RangeQueryPage, Q42RangeQueryPattern, Q42RangeQueryPlan,
        WasmSliceQ42Reader, MAX_COMPRESSED_SUPERBLOCK_SIZE, SUPERBLOCK_SIZE,
    },
    q_hash,
    query::resolver::classify_inline_literal,
    verifiable_credential::Credential,
    NQuin,
};
use sha2::{Digest, Sha256};

use crate::{
    fields, kinetics_marine_vehicle::bundle_bytes,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text, Args,
    AssetHost, Value,
};

const AUTHORITY_GRAPH: &str = "https://qualiadb.org/ontology/poet#EventGrantAuthority";
const AUTHORITY_KEY: &str = "https://qualiadb.org/ontology/poet#actorEd25519KeySha256";
const AUTHORITY_CAPABILITY: &str = "https://qualiadb.org/ontology/poet#actorGrantedCapability";
const CREDENTIAL_SHAPE: &str = "https://qualiadb.org/ontology/poet#PulseAuraGrantCredentialShape";
const GRANT_DOMAIN: &[u8] = b"Q42-PULSE-AURA-GRANT-V1";
const UNAUTHORIZED_GRANT: &str = "pulse_aura_unauthorized_grant";
const AURA_POLICY_GRAPH: &str = "https://qualiadb.org/ontology/poet#ActionAuraPolicy";
const REQUIRES_AURA_FACT: &str = "https://qualiadb.org/ontology/poet#requiresAuraFact";
const AURA_FACT_FIELDS: [&str; 4] = [
    "https://qualiadb.org/ontology/poet#factSubject",
    "https://qualiadb.org/ontology/poet#factPredicate",
    "https://qualiadb.org/ontology/poet#factObject",
    "https://qualiadb.org/ontology/poet#factGraph",
];
const HMC_GRAPH: &str = "https://qualiadb.org/ontology/hmc#SemanticModel";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const AFFORDANCE: &str = "https://qualiadb.org/ontology/hmc#Affordance";
const REQUIRED_CAPABILITY: &str = "https://qualiadb.org/ontology/hmc#requiredCapability";
const EMITTED_TOPIC: &str = "https://qualiadb.org/ontology/hmc#emittedTopic";
const MAX_AURA_BYTES: usize = 256 * 1024;
const MAX_AURA_FACTS: usize = 32;
const MAX_MODEL_BLOCKS: u64 = 256;
const LEX_PAGE_BYTES: usize = 1024 * 1024;
const TERM_BYTES: usize = 4096;
const SCRATCH_BYTES: usize =
    MAX_COMPRESSED_SUPERBLOCK_SIZE + SUPERBLOCK_SIZE + LEX_PAGE_BYTES + 2048 * 1024;

struct Grant {
    root: [u8; 32],
    actor: u64,
    action: u64,
    capability: u64,
    aura: [u8; 32],
    signature: [u8; 64],
}

impl Grant {
    /// Exact 32-byte Ed25519 message. Numeric identifiers are little-endian.
    fn digest(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        digest.update(GRANT_DOMAIN);
        digest.update(self.root);
        digest.update(self.actor.to_le_bytes());
        digest.update(self.action.to_le_bytes());
        digest.update(self.capability.to_le_bytes());
        digest.update(self.aura);
        digest.finalize().into()
    }

    fn parse(event: &Args) -> Result<Self, String> {
        fields(
            event,
            &[
                "hmc_root_digest",
                "actor_did_hash",
                "eont_action_id",
                "required_capability_hash",
                "aura_predicates_q42",
                "aura_graph_digest",
                "signature_hex",
            ],
        )?;
        let grant = Self {
            root: read_hex(event, "hmc_root_digest")?,
            actor: hash_field(event, "actor_did_hash")?,
            action: hash_field(event, "eont_action_id")?,
            capability: hash_field(event, "required_capability_hash")?,
            aura: read_hex(event, "aura_graph_digest")?,
            signature: read_hex(event, "signature_hex")?,
        };
        if grant.actor == 0 || grant.action == 0 || grant.capability == 0 {
            return Err(UNAUTHORIZED_GRANT.into());
        }
        Ok(grant)
    }
}

struct BoundSession {
    actor: String,
    action: String,
    capability: String,
    topic: Option<String>,
    asset_digest: String,
    aura_facts: usize,
}

impl AssetHost<'_> {
    pub(crate) fn bind_pulse_aura_hypermedia(&mut self, args: &Args) -> Result<Value, String> {
        if args.contains_key("signature_verified") {
            return Err(UNAUTHORIZED_GRANT.into());
        }
        let event = required(args, "event_grant")
            .and_then(record)
            .map_err(|_| UNAUTHORIZED_GRANT)?;
        let grant = Grant::parse(event).map_err(|_| UNAUTHORIZED_GRANT)?;
        fields(
            args,
            &[
                "bundle",
                "asset_key",
                "event_grant",
                "trusted_hmc_root_digest",
                "trusted_authority_key_digest",
                "authority_ed25519_public_key",
            ],
        )?;
        let bundle = required(args, "bundle")?;
        let asset_key = text(required(args, "asset_key")?)?;
        if asset_key.is_empty() || asset_key.len() > 4096 {
            return Err("Poet event requires a bounded 10D entry key".into());
        }
        let root_pin =
            read_hex::<32>(args, "trusted_hmc_root_digest").map_err(|_| UNAUTHORIZED_GRANT)?;
        let key_pin =
            read_hex::<32>(args, "trusted_authority_key_digest").map_err(|_| UNAUTHORIZED_GRANT)?;
        let key =
            read_hex::<32>(args, "authority_ed25519_public_key").map_err(|_| UNAUTHORIZED_GRANT)?;
        if grant.root != root_pin || sha256(&key) != key_pin {
            return Err(UNAUTHORIZED_GRANT.into());
        }
        let aura = required(event, "aura_predicates_q42").map_err(|_| UNAUTHORIZED_GRANT)?;
        let bundle_len = bundle_bytes(self, bundle)
            .map_err(|_| UNAUTHORIZED_GRANT)?
            .len();
        let aura_len = aura_bytes(self, aura)
            .map_err(|_| UNAUTHORIZED_GRANT)?
            .len();
        self.charge(
            bundle_len
                .saturating_mul(6)
                .saturating_add(aura_len * 4)
                .saturating_add(100_000),
            SCRATCH_BYTES,
        )?;
        let digest = grant.digest();
        verify_ed25519_raw_strict(&key, &digest, &grant.signature)
            .map_err(|_| UNAUTHORIZED_GRANT)?;
        let (bound, query_work) = verify_binding(self, bundle, asset_key, aura, &grant, &key_pin)
            .map_err(|_| UNAUTHORIZED_GRANT)?;
        self.charge(query_work, 0)?;
        let grant_iri = format!("urn:qualia:poet:event-grant:{}", hex(&digest));
        let root_iri = format!("urn:sha256:{}", hex(&grant.root));
        let aura_iri = format!("urn:sha256:{}", hex(&grant.aura));
        let key_iri = format!("urn:sha256:{}", hex(&key_pin));
        let mut graph = ReceiptGraph::new(&grant_iri);
        for (predicate, object) in [
            (
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
                "https://qualiadb.org/ontology/poet#OfflineVerifiedEventBinding",
            ),
            (
                "https://qualiadb.org/ontology/poet#boundActor",
                bound.actor.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#boundAction",
                bound.action.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#boundHmc",
                root_iri.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#boundAsset",
                bound.asset_digest.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#auraGraph",
                aura_iri.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#verifiedKey",
                key_iri.as_str(),
            ),
            (
                "https://qualiadb.org/ontology/poet#verificationScope",
                "https://qualiadb.org/ontology/poet#OfflineExternalPins",
            ),
            (
                "https://qualiadb.org/ontology/poet#actuationDisposition",
                "https://qualiadb.org/ontology/poet#OfflineOnly",
            ),
        ] {
            graph.fact(&grant_iri, predicate, object)?;
        }
        if let Some(topic) = &bound.topic {
            graph.fact(&grant_iri, EMITTED_TOPIC, topic)?;
        }
        graph.fact(&grant_iri, REQUIRED_CAPABILITY, &bound.capability)?;
        graph.fact(
            &grant_iri,
            "https://qualiadb.org/ontology/poet#evaluatedCredentialShape",
            CREDENTIAL_SHAPE,
        )?;
        let receipt = self.encode("q42", graph.seal(self.limits.resource_bytes)?)?;
        let session = Value::Record(Args::from([
            ("grant_iri".into(), Value::String(grant_iri)),
            ("hmc_root_digest".into(), Value::String(hex(&grant.root))),
            ("asset_key".into(), Value::String(asset_key.into())),
            ("asset_digest_iri".into(), Value::String(bound.asset_digest)),
            ("actor_did".into(), Value::String(bound.actor)),
            ("actor_did_hash".into(), Value::U64(grant.actor)),
            ("action_iri".into(), Value::String(bound.action)),
            ("eont_action_id".into(), Value::U64(grant.action)),
            (
                "required_capability_hash".into(),
                Value::U64(grant.capability),
            ),
            (
                "required_capability_iri".into(),
                Value::String(bound.capability),
            ),
            (
                "mounted_capability_policy_verified".into(),
                Value::Bool(true),
            ),
            ("capability_entitlement_verified".into(), Value::Bool(false)),
            (
                "pulse_topic_iri".into(),
                bound.topic.map_or(Value::Null, Value::String),
            ),
            ("aura_graph_digest".into(), Value::String(hex(&grant.aura))),
            (
                "aura_facts_verified".into(),
                Value::U64(bound.aura_facts as u64),
            ),
            ("signature_verified".into(), Value::Bool(true)),
            (
                "authority_scope".into(),
                Value::String("offline_external_pins".into()),
            ),
            ("authorizes_actuation".into(), Value::Bool(false)),
        ]));
        Ok(Value::Record(Args::from([
            ("bound_session".into(), session),
            ("motion_trigger_receipt".into(), receipt),
        ])))
    }
}

fn verify_binding(
    host: &AssetHost<'_>,
    bundle: &Value,
    asset_key: &str,
    aura: &Value,
    grant: &Grant,
    key_pin: &[u8; 32],
) -> Result<(BoundSession, usize), String> {
    let bundle_bytes = bundle_bytes(host, bundle)?;
    if sha256(bundle_bytes) != grant.root {
        return Err("Verified HMC bytes do not match the pinned event root".into());
    }
    let aura_bytes = aura_bytes(host, aura)?;
    if sha256(aura_bytes) != grant.aura {
        return Err("Aura predicates do not match the signed digest".into());
    }
    let bundle = BundleReader::parse_with_limits(
        bundle_bytes,
        BundleReadLimits {
            file_bytes: host.limits.resource_bytes,
            entries: host.limits.resources,
            ..BundleReadLimits::default()
        },
    )
    .map_err(|error| format!("Poet event HMC: {error}"))?;
    let model = open_semantic_model(&bundle).map_err(|error| error.to_string())?;
    let aura = WasmSliceQ42Reader::open(aura_bytes).map_err(|error| error.to_string())?;
    if !aura.is_unified_volume()
        || aura.block_count() == 0
        || aura.block_count() > 4
        || model.block_count() == 0
        || model.block_count() > MAX_MODEL_BLOCKS
    {
        return Err("Poet event requires bounded unified Q42 semantic and Aura graphs".into());
    }
    let mut scratch = QueryScratch::new();
    let mut aura_facts = [NQuin::default(); MAX_AURA_FACTS + 1];
    let page = scratch.page(&aura, Q42RangeQueryPattern::default(), &mut aura_facts)?;
    if page.returned == 0 || page.returned > MAX_AURA_FACTS || page.next_cursor.is_some() {
        return Err("Signed Aura graph must contain 1..=32 explicit ground facts".into());
    }
    let aura_facts = &aura_facts[..page.returned];
    let query_work = (model.block_count() as usize)
        .saturating_mul(2 * aura_facts.len() + 6)
        .saturating_mul(SUPERBLOCK_SIZE);
    if host.work.saturating_add(query_work as u64) > host.limits.asset_work_units {
        return Err("Poet event graph exceeds the admitted query work budget".into());
    }

    let (entry_digest, record) = find_affordance(host, &bundle, asset_key, grant)?;
    let actor = scratch.term(&model, grant.actor)?.to_owned();
    if !valid_actor_did(&actor) {
        return Err("Poet actor q_hash must resolve to a valid base DID in Q42LEX".into());
    }
    let action = scratch.term(&model, grant.action)?.to_owned();
    let key_iri = format!("urn:sha256:{}", hex(key_pin));
    scratch.expect_binding(
        &model,
        grant.actor,
        AUTHORITY_KEY,
        Some(&key_iri),
        AUTHORITY_GRAPH,
    )?;
    scratch.expect_fact(&model, grant.action, RDF_TYPE, AFFORDANCE, HMC_GRAPH)?;
    let capability = scratch
        .term(&model, record.required_capability_hash)?
        .to_owned();
    if !is_absolute_iri(&capability) {
        return Err("Poet event requires an absolute EONT capability IRI".into());
    }
    scratch.expect_fact(
        &model,
        grant.actor,
        AUTHORITY_CAPABILITY,
        &capability,
        AUTHORITY_GRAPH,
    )?;
    check_grant_credential_gate(grant, record.required_capability_hash)?;
    let topic = if record.emitted_topic_hash == 0 {
        None
    } else {
        Some(scratch.term(&model, record.emitted_topic_hash)?.to_owned())
    };
    for (predicate, iri) in [
        (REQUIRED_CAPABILITY, Some(capability.as_str())),
        (EMITTED_TOPIC, topic.as_deref()),
    ] {
        scratch.expect_binding(&model, grant.action, predicate, iri, HMC_GRAPH)?;
    }

    let mut declared =
        declared_aura_facts(host, &mut scratch, &model, grant.action, aura_facts.len())?;
    for row in aura_facts {
        if host.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        if !declared.remove(&[row.subject, row.predicate, row.object, row.context]) {
            return Err("Signed Aura graph contains a duplicate or undeclared precondition".into());
        }
        for (hash, is_object) in [
            (row.subject, false),
            (row.predicate, false),
            (row.object, true),
            (row.context, false),
        ] {
            if is_object && classify_inline_literal(hash).is_some() {
                continue;
            }
            let local = scratch.term(&aura, hash)?.to_owned();
            if (!is_object && !is_absolute_iri(&local)) || scratch.term(&model, hash)? != local {
                return Err(
                    "Signed Aura term spelling conflicts with the mounted Q42 graph".into(),
                );
            }
        }
        let mut matches = [NQuin::default(); 2];
        let page = scratch.page(
            &model,
            Q42RangeQueryPattern {
                subject: Some(row.subject),
                predicate: Some(row.predicate),
                object: Some(row.object),
                context: Some(row.context),
            },
            &mut matches,
        )?;
        if page.returned != 1 || page.next_cursor.is_some() || matches[0] != *row {
            return Err(
                "Signed Aura precondition is absent or ambiguous in the mounted Q42 graph".into(),
            );
        }
    }
    if !declared.is_empty() {
        return Err("Signed Aura graph omits an action-declared precondition".into());
    }
    Ok((
        BoundSession {
            actor,
            action,
            capability,
            topic,
            asset_digest: entry_digest,
            aura_facts: aura_facts.len(),
        },
        query_work,
    ))
}

/// Give the native SHACL gate the exact claim authenticated by the grant.
/// The mounted actor-key and actor-capability facts are checked separately.
/// This transient projection does not represent a separately verified VC.
fn check_grant_credential_gate(grant: &Grant, admitted_capability: u64) -> Result<(), String> {
    let predicate = q_hash(AUTHORITY_CAPABILITY);
    let context = q_hash(AUTHORITY_GRAPH);
    let claim = NQuin {
        subject: grant.actor,
        predicate,
        object: grant.capability,
        context,
        metadata: 0,
        parity: NQuin::calculate_parity(grant.actor, predicate, grant.capability, context, 0),
    };
    let projected = Credential {
        issuer: grant.actor,
        subject: grant.actor,
        issued_at: 0,
        valid_until: 0,
        claims: vec![claim],
    };
    let gate = CredentialGate {
        shape: q_hash(CREDENTIAL_SHAPE),
        required_claim_predicate: predicate,
        required_claim_object: admitted_capability,
        accepted_issuer: grant.actor,
    };
    if !credential_gates_target(&gate, grant.actor, &projected) {
        return Err("Poet event capability did not pass the mounted SHACL credential gate".into());
    }
    Ok(())
}

fn declared_aura_facts(
    host: &AssetHost<'_>,
    scratch: &mut QueryScratch,
    model: &WasmSliceQ42Reader<'_>,
    action: u64,
    signed_count: usize,
) -> Result<BTreeSet<[u64; 4]>, String> {
    let graph = q_hash(AURA_POLICY_GRAPH);
    let pattern = |subject| Q42RangeQueryPattern {
        subject: Some(subject),
        context: Some(graph),
        ..Default::default()
    };
    let mut links = [NQuin::default(); MAX_AURA_FACTS + 1];
    let page = scratch.page(model, pattern(action), &mut links)?;
    if page.returned != signed_count || page.next_cursor.is_some() {
        return Err("Signed Aura graph must match the mounted action's complete policy".into());
    }
    if scratch.term(model, graph)? != AURA_POLICY_GRAPH {
        return Err("Mounted action policy graph has a conflicting Q42LEX binding".into());
    }

    let mut nodes = BTreeSet::new();
    let mut facts = BTreeSet::new();
    for link in &links[..page.returned] {
        if host.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        if scratch.term(model, link.predicate)? != REQUIRES_AURA_FACT
            || link.object == action
            || !nodes.insert(link.object)
            || !is_absolute_iri(scratch.term(model, link.object)?)
        {
            return Err("Mounted action has an unsupported or ambiguous Aura declaration".into());
        }
        let mut rows = [NQuin::default(); AURA_FACT_FIELDS.len() + 1];
        let page = scratch.page(model, pattern(link.object), &mut rows)?;
        if page.returned != AURA_FACT_FIELDS.len() || page.next_cursor.is_some() {
            return Err("Aura fact declaration must have exactly four ground coordinates".into());
        }
        let mut fact = [0; AURA_FACT_FIELDS.len()];
        for row in &rows[..page.returned] {
            let predicate = scratch.term(model, row.predicate)?;
            let slot = AURA_FACT_FIELDS
                .iter()
                .position(|name| *name == predicate)
                .ok_or("Aura fact declaration has an unsupported coordinate predicate")?;
            if row.object == 0 || fact[slot] != 0 {
                return Err("Aura fact declaration has a missing or ambiguous coordinate".into());
            }
            fact[slot] = row.object;
        }
        if !facts.insert(fact) {
            return Err(
                "Mounted action repeats the same Aura fact through distinct declarations".into(),
            );
        }
    }
    Ok(facts)
}

fn find_affordance(
    host: &AssetHost<'_>,
    bundle: &BundleReader<'_>,
    asset_key: &str,
    grant: &Grant,
) -> Result<(String, AffordanceBinaryRecord), String> {
    let mut result = None;
    let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut temp = fields;
    let mut records = [AffordanceBinaryRecord::zeroed(); MAX_AFFORDANCES_PER_SECTION];
    for entry in bundle.entries().iter().filter(|entry| entry.kind == "10d") {
        if host.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        let bytes = bundle
            .get(&entry.key)
            .ok_or("Poet event could not read a 10D entry")?;
        if !bundle.verify_entry(&entry.key) {
            return Err("Poet event found an invalid 10D entry digest".into());
        }
        check_whole_file_crc32c(bytes).map_err(|error| error.to_string())?;
        let view = TypedPayloadResolver::new(bytes, &[], &mut temp, &mut fields)
            .map_err(|error| error.to_string())?;
        let Some(payload) = view.section_payload(SectionType::EntityOntology) else {
            continue;
        };
        let (header, count) = decode_entity_ontology_section_into(payload, &mut records)
            .map_err(|error| error.to_string())?;
        if header.entity_did_hash != grant.actor {
            continue;
        }
        for action in records[..count]
            .iter()
            .filter(|row| row.action_id == grant.action)
        {
            if result.is_some() || entry.key != asset_key {
                return Err(
                    "Poet actor and action are ambiguous or belong to a different 10D entry".into(),
                );
            }
            if action.required_capability_hash != grant.capability
                || action.min_observer_status != 0
                || header.energy_permill < action.min_energy_permill
                || header.damage_permill > action.max_damage_permill
                || header.hydration_permill < action.min_hydration_permill
                || header.wear_permill > action.max_wear_permill
                || [
                    header.energy_permill,
                    header.damage_permill,
                    header.hydration_permill,
                    header.wear_permill,
                ]
                .into_iter()
                .any(|value| value > 1000)
            {
                return Err(
                    "Poet event does not meet the exact EONT capability and state conditions"
                        .into(),
                );
            }
            result = Some((format!("urn:sha256:{}", hex(&entry.sha256)), *action));
        }
    }
    result.ok_or_else(|| "Poet event requires an exact EONT actor and action match".into())
}

struct QueryScratch {
    compressed: Vec<u8>,
    decoded: Vec<u8>,
    lex_page: Vec<u8>,
    spelling: [u8; TERM_BYTES],
}

impl QueryScratch {
    fn new() -> Self {
        Self {
            compressed: vec![0; MAX_COMPRESSED_SUPERBLOCK_SIZE],
            decoded: vec![0; SUPERBLOCK_SIZE],
            lex_page: vec![0; LEX_PAGE_BYTES],
            spelling: [0; TERM_BYTES],
        }
    }

    fn page(
        &mut self,
        model: &WasmSliceQ42Reader<'_>,
        pattern: Q42RangeQueryPattern,
        rows: &mut [NQuin],
    ) -> Result<Q42RangeQueryPage, String> {
        let page = model
            .execute_query_page_into(
                Q42RangeQueryPlan::for_pattern(pattern),
                Q42RangeQueryCursor::default(),
                &mut self.compressed,
                &mut self.decoded,
                rows,
            )
            .map_err(|error| error.to_string())?;
        if rows[..page.returned]
            .iter()
            .any(|row| !row.verify_ecc_parity())
        {
            return Err("Poet event encountered an invalid Q42 parity field".into());
        }
        Ok(page)
    }

    fn term(&mut self, model: &WasmSliceQ42Reader<'_>, hash: u64) -> Result<&str, String> {
        let len = model
            .lookup_lexicon_hash_into(hash, &mut self.lex_page, &mut self.spelling)
            .map_err(|error| error.to_string())?
            .ok_or("Poet event q_hash has no Q42LEX term")?;
        let term = std::str::from_utf8(&self.spelling[..len]).map_err(|error| error.to_string())?;
        if hash == 0 || q_hash(term) != hash {
            return Err("Poet event q_hash conflicts with its Q42LEX term".into());
        }
        Ok(term)
    }

    fn expect_binding(
        &mut self,
        model: &WasmSliceQ42Reader<'_>,
        subject: u64,
        predicate: &str,
        object: Option<&str>,
        graph: &str,
    ) -> Result<(), String> {
        let mut rows = [NQuin::default(); 2];
        let page = self.page(
            model,
            Q42RangeQueryPattern {
                subject: Some(subject),
                predicate: Some(q_hash(predicate)),
                object: None,
                context: Some(q_hash(graph)),
            },
            &mut rows,
        )?;
        if object.is_none() && page.returned == 0 && page.next_cursor.is_none() {
            return Ok(());
        }
        if page.returned != 1 || page.next_cursor.is_some() {
            return Err(format!(
                "Poet event has a missing or ambiguous {predicate} binding"
            ));
        }
        let object =
            object.ok_or_else(|| format!("Poet event has an unexpected {predicate} binding"))?;
        if self.term(model, rows[0].predicate)? != predicate
            || self.term(model, rows[0].context)? != graph
            || self.term(model, rows[0].object)? != object
        {
            return Err(format!(
                "Poet event {predicate} binding does not match its admitted value"
            ));
        }
        Ok(())
    }

    fn expect_fact(
        &mut self,
        model: &WasmSliceQ42Reader<'_>,
        subject: u64,
        predicate: &str,
        object: &str,
        graph: &str,
    ) -> Result<(), String> {
        let mut rows = [NQuin::default(); 2];
        let page = self.page(
            model,
            Q42RangeQueryPattern {
                subject: Some(subject),
                predicate: Some(q_hash(predicate)),
                object: Some(q_hash(object)),
                context: Some(q_hash(graph)),
            },
            &mut rows,
        )?;
        if page.returned != 1
            || page.next_cursor.is_some()
            || self.term(model, rows[0].predicate)? != predicate
            || self.term(model, rows[0].object)? != object
            || self.term(model, rows[0].context)? != graph
        {
            return Err("Poet event has no unique Q42 affordance classification".into());
        }
        Ok(())
    }
}

fn aura_bytes<'a>(host: &'a AssetHost<'_>, source: &Value) -> Result<&'a [u8], String> {
    let resource = encoded_resource(host, source)?;
    if resource.kind != "q42" || !(1..=MAX_AURA_BYTES).contains(&resource.bytes.len()) {
        return Err("Poet Aura predicates require a bounded retained Q42 resource".into());
    }
    Ok(&resource.bytes)
}

fn valid_actor_did(iri: &str) -> bool {
    let Some((method, identifier)) = iri.strip_prefix("did:").and_then(|did| did.split_once(':'))
    else {
        return false;
    };
    if iri.len() > 512
        || method.is_empty()
        || !method
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        || identifier.is_empty()
        || identifier.ends_with(':')
    {
        return false;
    }
    let mut bytes = identifier.bytes();
    while let Some(byte) = bytes.next() {
        match byte {
            b'%' => {
                if !(0..2).all(|_| bytes.next().is_some_and(|digit| digit.is_ascii_hexdigit())) {
                    return false;
                }
            }
            b'.' | b'-' | b'_' | b':' => {}
            byte if byte.is_ascii_alphanumeric() => {}
            _ => return false,
        }
    }
    match method {
        "q42" => parse_did_q42(iri.as_bytes()).is_ok(),
        "qi" => parse_did_qi_uri(iri.as_bytes()).is_ok(),
        "webizen" => parse_did_webizen(iri.as_bytes()).is_ok(),
        _ => true,
    }
}

fn is_absolute_iri(term: &str) -> bool {
    let Some((scheme, remainder)) = term.split_once(':') else {
        return false;
    };
    let mut scheme = scheme.bytes();
    scheme.next().is_some_and(|byte| byte.is_ascii_alphabetic())
        && scheme.all(|byte| byte.is_ascii_alphanumeric() || b"+-.".contains(&byte))
        && !remainder.is_empty()
        && !remainder
            .chars()
            .any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
}

fn hash_field(args: &Args, field: &str) -> Result<u64, String> {
    match required(args, field)? {
        Value::U64(value) => Ok(*value),
        Value::I64(value) if *value >= 0 => Ok(*value as u64),
        _ => Err(format!("{field} must be an exact unsigned 64-bit integer")),
    }
}

fn read_hex<const N: usize>(args: &Args, field: &str) -> Result<[u8; N], String> {
    let input = text(required(args, field)?)?.as_bytes();
    if input.len() != N * 2 {
        return Err(format!(
            "{field} must contain {} lowercase hexadecimal digits",
            N * 2
        ));
    }
    let mut output = [0; N];
    for (dst, pair) in output.iter_mut().zip(input.chunks_exact(2)) {
        let digit = |b| match b {
            b'0'..=b'9' => Ok(b - b'0'),
            b'a'..=b'f' => Ok(b - b'a' + 10),
            _ => Err(format!("{field} must contain lowercase hexadecimal digits")),
        };
        *dst = (digit(pair[0])? << 4) | digit(pair[1])?;
    }
    Ok(output)
}

fn hex(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(DIGITS[usize::from(byte >> 4)] as char);
        out.push(DIGITS[usize::from(byte & 15)] as char);
    }
    out
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

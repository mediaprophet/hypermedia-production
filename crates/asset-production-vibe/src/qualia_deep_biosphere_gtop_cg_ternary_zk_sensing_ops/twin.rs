//! Bounded entity-twin evaluation and observer-relative semantic projections.

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{
    artifact::AnimClip, boolean, fields, list, num, record, required, text, vector, Args,
    AssetHost, TypedArtifact, Value,
};
use bytemuck::Zeroable;
use qualia_core_db::{
    container_10d::{
        self as ten,
        entity_ontology_section::{
            decode_entity_ontology_section_into, AffordanceBinaryRecord, EntityOntologyMiniHeader,
            MAX_AFFORDANCES_PER_SECTION,
        },
        skeletal_container::{
            read_skeletal_section_payload, ContainerSkeletalSection, SkeletalContainerError,
        },
        SectionType, TypedFieldRecord, TypedPayloadResolver, FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    },
    entity_view::{
        bifurcation::{
            evaluate_quin_bifurcation, BifurcationContext64, FLAG_REQUIRE_PARITY,
            FLAG_STRICT_SUBJECT,
        },
        entity_id::{EntityId, EntityKind},
        entity_ontologies::{
            ActionAffordance, ActionEffect, ActionEvaluationVerdict, ActionIntent,
            ActionPrecondition, AffordanceCategory, EntityOntology, LifecycleStage,
            OntologicalOrigin,
        },
        ethogram::{
            EntityPhysiologicalState, EthogramBehaviorEntry, EthogramRegistry, StimulusCategory,
            StimulusEvent, MAX_ETHOGRAM_BEHAVIORS,
        },
        observer::{EntityViewMeta, ObserverStatus, RepresentationWing, SensitivityClass},
        package::BifurcatedPackage,
        projection::{layout_scene_nodes, LayoutInput, SceneNodeProj},
        rights_filter::{decide_view, ViewDecision},
    },
    q_hash, NQuin,
};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

const GRAPH: &str = "urn:qualia:twin:action-receipt:v1";
const PREFIX: &str = "urn:qualia:twin:";
const MAX_TEXT: usize = 4096;
const MAX_CLIP_KEYS: usize = 65_536;

macro_rules! enumeration {
    ($value:expr, $field:expr) => {
        serde_json::from_value(serde_json::Value::String(bounded_text($value)?.into()))
            .map_err(|_| format!("Invalid {}", $field))
    };
}

impl AssetHost<'_> {
    pub(crate) fn evaluate_ethogram_and_entity_affordance(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "entity",
                "event",
                "observer",
                "state",
                "scene",
                "behaviors",
                "export_ai_tools",
                "clip_source",
            ],
        )?;
        let entity = record(required(args, "entity")?)?;
        fields(
            entity,
            &[
                "iri",
                "taxon_or_system_iri",
                "origin",
                "kind",
                "sensitivity",
                "is_secret",
                "commons_visible",
                "peer_offered",
                "affordances",
                "location_lat_lon",
            ],
        )?;
        let entity_iri = iri(required(entity, "iri")?)?;
        let taxon = iri(required(entity, "taxon_or_system_iri")?)?;
        let id = EntityId::from_iri_q_hash(entity_iri);
        let viewer = record(required(args, "observer")?)?;
        fields(
            viewer,
            &[
                "status",
                "caller_iri",
                "capability_iris",
                "available_flags",
                "lamport_clock",
            ],
        )?;
        let status: ObserverStatus = enumeration!(required(viewer, "status")?, "observer status")?;
        let meta = EntityViewMeta {
            entity_id: id,
            kind: entity
                .get("kind")
                .map(|v| enumeration!(v, "entity kind"))
                .transpose()?
                .unwrap_or(EntityKind::Asset),
            sensitivity: entity
                .get("sensitivity")
                .map(|v| enumeration!(v, "sensitivity"))
                .transpose()?
                .unwrap_or(SensitivityClass::Public),
            is_secret: boolean(entity, "is_secret", false)?,
            commons_visible: boolean(entity, "commons_visible", false)?,
            peer_offered: boolean(entity, "peer_offered", false)?,
        };
        let decision = decide_view(status, &meta);
        if !decision.visible || !decision.affordances.can_open {
            return Ok(Value::Record(BTreeMap::from([(
                "visible".into(),
                Value::Bool(false),
            )])));
        }
        let observer = Observer::new(viewer, status)?;
        let scene = args.get("scene");
        let clip_source = args.get("clip_source").or(scene);
        let header_bytes = scene
            .map(|value| source_size(self, value))
            .transpose()?
            .unwrap_or(0);
        let clip_bytes = clip_source
            .map(|value| source_size(self, value))
            .transpose()?
            .unwrap_or(0);
        self.charge(
            header_bytes
                .saturating_add(clip_bytes)
                .saturating_add(MAX_AFFORDANCES_PER_SECTION * MAX_AFFORDANCES_PER_SECTION),
            4 * 1024 * 1024,
        )?;
        self_admit_bound(header_bytes, self.limits.resource_bytes)?;
        self_admit_bound(clip_bytes, self.limits.resource_bytes)?;

        let mut ontology = EntityOntology::new(id, taxon);
        let origin: OntologicalOrigin =
            enumeration!(required(entity, "origin")?, "ontological origin")?;
        ontology.origin = origin;
        let mut action_iris = BTreeMap::new();
        let mut by_effect_iri = BTreeMap::new();
        let mut state = if let Some(value) = scene {
            let bytes = source_10d(self, value)?;
            let (header, entries) = read_eont(bytes, entity_iri, taxon)?;
            let metadata = entity
                .get("affordances")
                .map(list)
                .transpose()?
                .unwrap_or(&[]);
            let mut authored = BTreeMap::new();
            for value in metadata {
                let row = record(value)?;
                fields(row, &["iri", "name", "description", "input_schema"])?;
                let action = iri(required(row, "iri")?)?;
                if authored.insert(q_hash(action), (action, row)).is_some() {
                    return Err("Repeated scene affordance metadata".into());
                }
            }
            for (wire, lexical, capability, emission) in entries {
                let authored = authored.remove(&wire.action_id);
                if authored.is_some_and(|(iri, _)| iri != lexical) {
                    return Err("Scene affordance metadata conflicts with CBLD".into());
                }
                let mut action = action_metadata(&lexical, authored.map(|(_, row)| row))?;
                apply_wire_action(&mut action, wire)?;
                insert_action(&mut ontology, &mut action_iris, action, lexical)?;
                if let Some(iri) = capability {
                    by_effect_iri.insert(wire.required_capability_hash, iri);
                }
                if let Some(iri) = emission {
                    by_effect_iri.insert(wire.emitted_topic_hash, iri);
                }
            }
            if !authored.is_empty() {
                return Err(
                    "Scene affordance metadata references an action absent from EONT".into(),
                );
            }
            state_from_header(header)?
        } else {
            let authored = list(required(entity, "affordances")?)?;
            if authored.len() > MAX_AFFORDANCES_PER_SECTION {
                return Err("Entity has too many authored affordances".into());
            }
            for value in authored {
                let row = record(value)?;
                fields(
                    row,
                    &[
                        "iri",
                        "name",
                        "description",
                        "category",
                        "input_schema",
                        "preconditions",
                        "effects",
                    ],
                )?;
                let lexical = iri(required(row, "iri")?)?;
                let mut action = action_metadata(lexical, Some(row))?;
                action.category = enumeration!(required(row, "category")?, "affordance category")?;
                if let Some(value) = row.get("preconditions") {
                    let p = record(value)?;
                    fields(
                        p,
                        &[
                            "min_energy_permill",
                            "max_damage_permill",
                            "min_hydration_permill",
                            "max_wear_permill",
                            "capability_iri",
                            "min_observer_status",
                        ],
                    )?;
                    action.preconditions = ActionPrecondition {
                        min_energy_permill: meter(p, "min_energy_permill", 0)?,
                        max_damage_permill: meter(p, "max_damage_permill", 1000)?,
                        min_hydration_permill: meter(p, "min_hydration_permill", 0)?,
                        max_wear_permill: meter(p, "max_wear_permill", 1000)?,
                        required_capability_hash: optional_binding(
                            p,
                            "capability_iri",
                            &mut by_effect_iri,
                        )?,
                        min_observer_status: p
                            .get("min_observer_status")
                            .map(|v| enumeration!(v, "minimum observer status"))
                            .transpose()?
                            .unwrap_or(ObserverStatus::Public),
                    };
                }
                if let Some(value) = row.get("effects") {
                    let e = record(value)?;
                    fields(
                        e,
                        &[
                            "energy_delta",
                            "damage_delta",
                            "hydration_delta",
                            "wear_delta",
                            "emitted_iri",
                            "animation_clip_id",
                        ],
                    )?;
                    action.effects = ActionEffect {
                        energy_delta: delta(e, "energy_delta")?,
                        damage_delta: delta(e, "damage_delta")?,
                        hydration_delta: delta(e, "hydration_delta")?,
                        wear_delta: delta(e, "wear_delta")?,
                        emitted_topic_hash: optional_binding(e, "emitted_iri", &mut by_effect_iri)?,
                        animation_clip_id: u32_value(e, "animation_clip_id", 0)?,
                    };
                }
                insert_action(&mut ontology, &mut action_iris, action, lexical.into())?;
            }
            EntityPhysiologicalState {
                energy_permill: 1000,
                hydration_permill: 1000,
                lifecycle_stage: LifecycleStage::Adult as u8,
                ..Default::default()
            }
        };
        if let Some(value) = args.get("state") {
            state = state_from_value(value, state)?;
        }
        ontology.stage = LifecycleStage::from_u8(state.lifecycle_stage);
        ontology.energy_permill = state.energy_permill;
        ontology.damage_permill = state.damage_permill;
        ontology.hydration_permill = state.hydration_permill;
        ontology.wear_permill = state.wear_permill;

        let event = record(required(args, "event")?)?;
        let kind = text(required(event, "kind")?)?;
        let fired = match kind {
            "inspect" => {
                fields(event, &["kind"])?;
                None
            }
            "intent" => {
                fields(event, &["kind", "action_iri", "parameter_iri"])?;
                let lexical = iri(required(event, "action_iri")?)?;
                let hash = q_hash(lexical);
                if action_iris.get(&hash).map(String::as_str) != Some(lexical) {
                    return Err("The requested action is absent from the entity ontology".into());
                }
                let parameter = event.get("parameter_iri").map(iri).transpose()?;
                let verdict = evaluate(&ontology, hash, parameter, &observer)?;
                apply_verdict(&mut state, &verdict);
                Some(Fired {
                    hash,
                    event_iri: parameter,
                    verdict,
                })
            }
            "stimulus" => {
                fields(
                    event,
                    &[
                        "kind",
                        "category",
                        "topic_iri",
                        "source_iri",
                        "intensity_permill",
                        "distance_m",
                    ],
                )?;
                let topic = iri(required(event, "topic_iri")?)?;
                let source = iri(required(event, "source_iri")?)?;
                let distance = num(required(event, "distance_m")?)?;
                if !(0.0..=1.0e9).contains(&distance) {
                    return Err("Stimulus distance outside supported physical bounds".into());
                }
                let stimulus = StimulusEvent {
                    topic_hash: q_hash(topic),
                    source_did_hash: q_hash(source),
                    intensity_permill: meter(event, "intensity_permill", 1000)?,
                    category: stimulus_category(text(required(event, "category")?)?)?,
                    reserved_u8: 0,
                    distance_meters: distance as f32,
                };
                trigger(
                    &ontology,
                    &observer,
                    &action_iris,
                    required(args, "behaviors")?,
                    &stimulus,
                    topic,
                    &mut state,
                )?
            }
            _ => return Err("Entity twin event kind must be inspect, intent, or stimulus".into()),
        };

        let projection = projection(entity, &meta, decision)?;
        let animation = match (fired.as_ref(), clip_source) {
            (Some(fired), Some(source)) if fired.verdict.animation_clip_id != 0 => {
                read_transition_clip(source_10d(self, source)?, fired.verdict.animation_clip_id)?
            }
            _ => None,
        };
        let export_tools = boolean(args, "export_ai_tools", false)?;
        let tools = if export_tools {
            ontology.energy_permill = state.energy_permill;
            ontology.damage_permill = state.damage_permill;
            ontology.hydration_permill = state.hydration_permill;
            ontology.wear_permill = state.wear_permill;
            let eligible: BTreeSet<_> = ontology
                .affordances
                .iter()
                .filter_map(|action| {
                    evaluate(&ontology, action.action_id, None, &observer)
                        .ok()
                        .map(|_| action.action_id)
                })
                .collect();
            ontology
                .affordances
                .retain(|action| eligible.contains(&action.action_id));
            let json = ontology.export_ai_tools_json();
            serde_json::from_str::<serde_json::Value>(&json)
                .map_err(|e| format!("Native AI tool manifest: {e}"))?;
            if json.len() > self.limits.resource_bytes {
                return Err("Entity AI tool manifest exceeds the resource budget".into());
            }
            Some(json.into_bytes())
        } else {
            None
        };

        let mut result = BTreeMap::from([
            ("visible".into(), Value::Bool(true)),
            ("projection".into(), projection),
            ("state".into(), state_value(state)),
            ("transitioned".into(), Value::Bool(fired.is_some())),
        ]);
        if let Some(fired) = fired {
            let action_iri = action_iris
                .get(&fired.hash)
                .ok_or("Action evaluated without a canonical IRI")?;
            let (quin, graph) = action_receipt(
                entity_iri,
                taxon,
                action_iri,
                &observer,
                &meta,
                decision,
                &fired,
                state,
                &by_effect_iri,
            )?;
            result.insert(
                "action_quin".into(),
                Value::List(quin.into_iter().map(Value::U64).collect()),
            );
            result.insert("action_iri".into(), Value::String(action_iri.clone()));
            result.insert(
                "animation_clip_id".into(),
                Value::U64(u64::from(fired.verdict.animation_clip_id)),
            );
            result.insert(
                "receipt".into(),
                self.encode("q42", graph.seal(self.limits.resource_bytes)?)?,
            );
        }
        if let Some(animation) = animation {
            result.insert(
                "animation".into(),
                self.admit_artifact(TypedArtifact::AnimationClip(Arc::new(animation)))?,
            );
        }
        if let Some(tools) = tools {
            result.insert("ai_tools".into(), self.encode("json", tools)?);
        }
        Ok(Value::Record(result))
    }
}

fn self_admit_bound(bytes: usize, budget: usize) -> Result<(), String> {
    if bytes > budget.min(128 * 1024 * 1024) {
        return Err("Entity source exceeds the 10D ring budget".into());
    }
    Ok(())
}

fn source_size(host: &AssetHost<'_>, value: &Value) -> Result<usize, String> {
    source_10d(host, value).map(<[u8]>::len)
}

fn source_10d<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    if let Ok(TypedArtifact::SceneGraph10d(scene)) = host.workspace.typed(value) {
        return Ok(scene.fields().as_10d_bytes());
    }
    let resource = match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|r| &r.key == key)
            .ok_or("Unknown retained 10D source")?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != "10d" {
        return Err("Entity source requires an encoded 10D or SceneGraph10d".into());
    }
    Ok(&resource.bytes)
}

fn bounded_text(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    if value.is_empty() || value.len() > MAX_TEXT || value.chars().any(char::is_control) {
        return Err("Expected nonempty bounded Unicode text".into());
    }
    Ok(value)
}

fn iri(value: &Value) -> Result<&str, String> {
    let value = bounded_text(value)?;
    if !absolute_iri(value) || q_hash(value) == 0 {
        return Err("Expected an absolute RDF IRI with a nonzero canonical q_hash".into());
    }
    Ok(value)
}

fn absolute_iri(value: &str) -> bool {
    let Some((scheme, tail)) = value.split_once(':') else {
        return false;
    };
    scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        && scheme
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"+.-".contains(&b))
        && !tail.is_empty()
        && !tail
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "<>{}|\\^\"".contains(c))
}

fn exact_unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => {
            u64::try_from(*value).map_err(|_| "Expected an unsigned integer".into())
        }
        _ => Err("Expected an exact unsigned integer".into()),
    }
}

fn u32_value(args: &Args, key: &str, default: u32) -> Result<u32, String> {
    let value = args
        .get(key)
        .map(exact_unsigned)
        .transpose()?
        .unwrap_or(u64::from(default));
    u32::try_from(value).map_err(|_| format!("{key} exceeds u32"))
}

fn u16_value(args: &Args, key: &str, default: u16) -> Result<u16, String> {
    let value = args
        .get(key)
        .map(exact_unsigned)
        .transpose()?
        .unwrap_or(u64::from(default));
    u16::try_from(value).map_err(|_| format!("{key} exceeds u16"))
}

fn meter(args: &Args, key: &str, default: u16) -> Result<u16, String> {
    let value = u16_value(args, key, default)?;
    if value > 1000 {
        return Err(format!("{key} must be in 0..=1000 permill"));
    }
    Ok(value)
}

fn delta(args: &Args, key: &str) -> Result<i16, String> {
    match args.get(key) {
        None => Ok(0),
        Some(Value::I64(value)) if (-1000..=1000).contains(value) => Ok(*value as i16),
        Some(Value::U64(value)) if *value <= 1000 => Ok(*value as i16),
        _ => Err(format!(
            "{key} requires an exact signed permill delta in -1000..=1000"
        )),
    }
}

fn optional_binding(
    args: &Args,
    key: &str,
    bindings: &mut BTreeMap<u64, String>,
) -> Result<u64, String> {
    let Some(value) = args.get(key) else {
        return Ok(0);
    };
    let iri = iri(value)?;
    let hash = q_hash(iri);
    if bindings.get(&hash).is_some_and(|existing| existing != iri) {
        return Err("Entity effect IRIs have a q_hash collision".into());
    }
    bindings.entry(hash).or_insert_with(|| iri.into());
    Ok(hash)
}

struct Observer<'a> {
    status: ObserverStatus,
    iri: &'a str,
    capabilities: BTreeSet<u64>,
    flags: u16,
    clock: u32,
}

impl<'a> Observer<'a> {
    fn new(viewer: &'a Args, status: ObserverStatus) -> Result<Self, String> {
        let iri = iri(required(viewer, "caller_iri")?)?;
        let capabilities = viewer
            .get("capability_iris")
            .map(list)
            .transpose()?
            .unwrap_or(&[]);
        if capabilities.len() > MAX_ETHOGRAM_BEHAVIORS {
            return Err("Observer has too many declared capability IRIs".into());
        }
        Ok(Self {
            status,
            iri,
            capabilities: capabilities
                .iter()
                .map(|v| self::iri(v).map(q_hash))
                .collect::<Result<_, _>>()?,
            flags: u16_value(viewer, "available_flags", 0)?,
            clock: u32_value(viewer, "lamport_clock", 0)?,
        })
    }
}

fn action_metadata(iri: &str, authored: Option<&Args>) -> Result<ActionAffordance, String> {
    let hash = q_hash(iri);
    let fallback = format!("action_{hash:016x}");
    let name = authored
        .and_then(|row| row.get("name"))
        .map(bounded_text)
        .transpose()?
        .unwrap_or(&fallback);
    if name.len() > 64
        || !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
    {
        return Err(
            "AI tool names must be stable function identifiers of up to 64 characters".into(),
        );
    }
    let description = authored
        .and_then(|row| row.get("description"))
        .map(bounded_text)
        .transpose()?
        .unwrap_or(iri);
    if description.contains('\\') {
        return Err("AI tool description contains an unsupported JSON escape".into());
    }
    let mut action = ActionAffordance::new(name, description, AffordanceCategory::Biological);
    action.action_id = hash;
    if let Some(schema) = authored.and_then(|row| row.get("input_schema")) {
        let schema = crate::living_world_anatomy::species_spec::vibe_value_to_json(schema);
        if schema.get("type").and_then(|v| v.as_str()) != Some("object") {
            return Err("AI tool parameter schema must declare an object".into());
        }
        let encoded = serde_json::to_string(&schema).map_err(|e| format!("AI tool schema: {e}"))?;
        if encoded.len() > MAX_TEXT {
            return Err("AI tool schema exceeds the authoring budget".into());
        }
        action.input_schema_json = encoded;
    }
    Ok(action)
}

fn insert_action(
    ontology: &mut EntityOntology,
    actions: &mut BTreeMap<u64, String>,
    action: ActionAffordance,
    iri: String,
) -> Result<(), String> {
    if actions.insert(action.action_id, iri).is_some()
        || ontology
            .affordances
            .iter()
            .any(|entry| entry.name == action.name)
    {
        return Err("Duplicate entity action IRI, hash, or tool name".into());
    }
    ontology.add_affordance(action);
    Ok(())
}

type LexicalAction = (
    AffordanceBinaryRecord,
    String,
    Option<String>,
    Option<String>,
);

fn read_eont(
    bytes: &[u8],
    entity: &str,
    taxon: &str,
) -> Result<(EntityOntologyMiniHeader, Vec<LexicalAction>), String> {
    let header = ten::Container10dHeader::parse(bytes).map_err(|e| e.to_string())?;
    if header.header_crc32c != ten::compute_whole_file_crc32c(bytes) {
        return Err("Entity 10D checksum mismatch".into());
    }
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|e| e.to_string())?;
    let payload = view
        .section_payload(SectionType::EntityOntology)
        .ok_or("Scene has no EONT1 section")?;
    let mut records = [AffordanceBinaryRecord::zeroed(); MAX_AFFORDANCES_PER_SECTION];
    let (header, count) =
        decode_entity_ontology_section_into(payload, &mut records).map_err(|e| e.to_string())?;
    if header.entity_did_hash != q_hash(entity)
        || header.taxon_iri_hash != q_hash(taxon)
        || header.reserved_u8 != 0
        || header.reserved_u32 != 0
    {
        return Err("Scene EONT identity differs from its declared entity or taxon IRI".into());
    }
    let mut wanted: BTreeMap<_, Option<String>> = BTreeMap::from([
        (header.entity_did_hash, None),
        (header.taxon_iri_hash, None),
    ]);
    for record in &records[..count] {
        if record.action_id == 0 {
            return Err("EONT action identity must be nonzero".into());
        }
        for hash in [
            record.action_id,
            record.required_capability_hash,
            record.emitted_topic_hash,
        ] {
            if hash != 0 {
                wanted.entry(hash).or_insert(None);
            }
        }
    }
    for payload in view.payloads_of_kind(FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        let decoded = ten::decode_symbolic_rdf_cbor_ld(payload.map_err(|e| e.to_string())?.bytes)
            .map_err(|e| e.to_string())?;
        for term in decoded.lexicon {
            let Some(slot) = wanted.get_mut(&term.term_hash) else {
                continue;
            };
            if term.iri_utf8.len() > MAX_TEXT
                || !absolute_iri(&term.iri_utf8)
                || q_hash(&term.iri_utf8) != term.term_hash
                || slot.as_ref().is_some_and(|old| old != &term.iri_utf8)
            {
                return Err("Invalid local CBLD binding for an EONT term".into());
            }
            *slot = Some(term.iri_utf8);
        }
    }
    if wanted.values().any(Option::is_none)
        || wanted[&header.entity_did_hash].as_deref() != Some(entity)
        || wanted[&header.taxon_iri_hash].as_deref() != Some(taxon)
    {
        return Err("EONT q_hashes require their bijective local CBLD IRIs".into());
    }
    let lookup = |hash| wanted.get(&hash).and_then(Clone::clone);
    let mut output = Vec::with_capacity(count);
    for &record in &records[..count] {
        let action = lookup(record.action_id).ok_or("Missing local CBLD action IRI")?;
        output.push((
            record,
            action,
            lookup(record.required_capability_hash),
            lookup(record.emitted_topic_hash),
        ));
    }
    Ok((header, output))
}

fn apply_wire_action(
    action: &mut ActionAffordance,
    record: AffordanceBinaryRecord,
) -> Result<(), String> {
    let status = match record.min_observer_status {
        0 => ObserverStatus::Principal,
        1 => ObserverStatus::Peer,
        2 => ObserverStatus::Guardian,
        3 => ObserverStatus::Steward,
        4 => ObserverStatus::Public,
        5 => ObserverStatus::Instrument,
        6 => ObserverStatus::Auditor,
        _ => return Err("EONT observer status is out of range".into()),
    };
    if record.category > AffordanceCategory::Governance as u8
        || record.reserved_u16 != 0
        || [
            record.min_energy_permill,
            record.max_damage_permill,
            record.min_hydration_permill,
            record.max_wear_permill,
        ]
        .into_iter()
        .any(|value| value > 1000)
        || [
            record.energy_delta,
            record.damage_delta,
            record.hydration_delta,
            record.wear_delta,
        ]
        .into_iter()
        .any(|value| !(-1000..=1000).contains(&value))
    {
        return Err("EONT action has an invalid category or physiological range".into());
    }
    action.category = AffordanceCategory::from_u8(record.category);
    action.preconditions = ActionPrecondition {
        min_energy_permill: record.min_energy_permill,
        max_damage_permill: record.max_damage_permill,
        min_hydration_permill: record.min_hydration_permill,
        max_wear_permill: record.max_wear_permill,
        required_capability_hash: record.required_capability_hash,
        min_observer_status: status,
    };
    action.effects = ActionEffect {
        energy_delta: record.energy_delta,
        damage_delta: record.damage_delta,
        hydration_delta: record.hydration_delta,
        wear_delta: record.wear_delta,
        emitted_topic_hash: record.emitted_topic_hash,
        animation_clip_id: record.animation_clip_id,
    };
    Ok(())
}

fn state_from_header(header: EntityOntologyMiniHeader) -> Result<EntityPhysiologicalState, String> {
    if header.stage > LifecycleStage::Decommissioned as u8
        || [
            header.energy_permill,
            header.damage_permill,
            header.hydration_permill,
            header.wear_permill,
        ]
        .into_iter()
        .any(|value| value > 1000)
    {
        return Err("Invalid EONT lifecycle or physiological state".into());
    }
    Ok(EntityPhysiologicalState {
        energy_permill: header.energy_permill,
        damage_permill: header.damage_permill,
        hydration_permill: header.hydration_permill,
        wear_permill: header.wear_permill,
        lifecycle_stage: header.stage,
        ..Default::default()
    })
}

fn state_from_value(
    value: &Value,
    default: EntityPhysiologicalState,
) -> Result<EntityPhysiologicalState, String> {
    let state = record(value)?;
    fields(
        state,
        &[
            "energy_permill",
            "damage_permill",
            "hydration_permill",
            "wear_permill",
            "active_state_id",
            "lifecycle_stage",
            "active_motor_clip_id",
        ],
    )?;
    let stage = match state.get("lifecycle_stage") {
        None => LifecycleStage::from_u8(default.lifecycle_stage),
        Some(value @ (Value::I64(_) | Value::U64(_))) => {
            let raw = exact_unsigned(value)?;
            if raw > LifecycleStage::Decommissioned as u64 {
                return Err("Lifecycle stage must be in 0..=8".into());
            }
            LifecycleStage::from_u8(raw as u8)
        }
        Some(value) => enumeration!(value, "lifecycle stage")?,
    };
    Ok(EntityPhysiologicalState {
        energy_permill: meter(state, "energy_permill", default.energy_permill)?,
        damage_permill: meter(state, "damage_permill", default.damage_permill)?,
        hydration_permill: meter(state, "hydration_permill", default.hydration_permill)?,
        wear_permill: meter(state, "wear_permill", default.wear_permill)?,
        active_state_id: u16_value(state, "active_state_id", default.active_state_id)?,
        lifecycle_stage: stage as u8,
        active_motor_clip_id: u32_value(
            state,
            "active_motor_clip_id",
            default.active_motor_clip_id,
        )?,
        ..Default::default()
    })
}

fn state_value(state: EntityPhysiologicalState) -> Value {
    Value::Record(BTreeMap::from([
        (
            "energy_permill".into(),
            Value::U64(u64::from(state.energy_permill)),
        ),
        (
            "damage_permill".into(),
            Value::U64(u64::from(state.damage_permill)),
        ),
        (
            "hydration_permill".into(),
            Value::U64(u64::from(state.hydration_permill)),
        ),
        (
            "wear_permill".into(),
            Value::U64(u64::from(state.wear_permill)),
        ),
        (
            "active_state_id".into(),
            Value::U64(u64::from(state.active_state_id)),
        ),
        (
            "lifecycle_stage".into(),
            Value::U64(u64::from(state.lifecycle_stage)),
        ),
        (
            "active_motor_clip_id".into(),
            Value::U64(u64::from(state.active_motor_clip_id)),
        ),
    ]))
}

fn evaluate(
    ontology: &EntityOntology,
    hash: u64,
    parameter: Option<&str>,
    observer: &Observer<'_>,
) -> Result<ActionEvaluationVerdict, String> {
    let action = ontology
        .affordances
        .iter()
        .find(|action| action.action_id == hash)
        .ok_or("Undeclared action IRI")?;
    let capability = action.preconditions.required_capability_hash;
    if capability != 0 && !observer.capabilities.contains(&capability) {
        return Err("Observer does not hold the action's declared capability".into());
    }
    ontology
        .evaluate_intent(
            &ActionIntent {
                caller_did_hash: q_hash(observer.iri),
                target_entity_hash: ontology.entity_id.raw(),
                action_id: hash,
                parameters_hash: parameter.map_or(0, q_hash),
                lamport_clock: observer.clock,
            },
            observer.status,
        )
        .map_err(|e| e.to_string())
}

fn apply_verdict(state: &mut EntityPhysiologicalState, verdict: &ActionEvaluationVerdict) {
    state.energy_permill = verdict.new_energy;
    state.damage_permill = verdict.new_damage;
    state.hydration_permill = verdict.new_hydration;
    state.wear_permill = verdict.new_wear;
    state.active_motor_clip_id = verdict.animation_clip_id;
}

fn stimulus_category(category: &str) -> Result<u8, String> {
    Ok(match category {
        "ambient_proximity" => StimulusCategory::AmbientProximity,
        "predator_alert" => StimulusCategory::PredatorAlert,
        "prey_or_food_detected" => StimulusCategory::PreyOrFoodDetected,
        "mating_call" => StimulusCategory::MatingCall,
        "weather_drought" => StimulusCategory::WeatherDrought,
        "weather_deluge" => StimulusCategory::WeatherDeluge,
        "wildfire_smoke" => StimulusCategory::WildfireSmoke,
        "physical_contact" => StimulusCategory::PhysicalContact,
        "human_observer_approaching" => StimulusCategory::HumanObserverApproaching,
        _ => return Err("Unknown ethogram stimulus category".into()),
    } as u8)
}

struct Fired<'a> {
    hash: u64,
    event_iri: Option<&'a str>,
    verdict: ActionEvaluationVerdict,
}

fn trigger<'a>(
    ontology: &EntityOntology,
    observer: &Observer<'_>,
    action_iris: &BTreeMap<u64, String>,
    behaviors: &Value,
    stimulus: &StimulusEvent,
    topic: &'a str,
    state: &mut EntityPhysiologicalState,
) -> Result<Option<Fired<'a>>, String> {
    let items = list(behaviors)?;
    if items.len() > MAX_ETHOGRAM_BEHAVIORS {
        return Err("Ethogram registry exceeds 32 authored behaviors".into());
    }
    let mut registry = EthogramRegistry::new(q_hash(&ontology.taxon_or_system_iri));
    let mut best: Option<(u8, Fired<'a>)> = None;
    for item in items {
        let row = record(item)?;
        fields(
            row,
            &[
                "action_iri",
                "topic_iri",
                "category",
                "target_state_id",
                "required_flags",
                "priority",
                "min_intensity_permill",
                "max_distance_m",
            ],
        )?;
        let action_iri = iri(required(row, "action_iri")?)?;
        let hash = q_hash(action_iri);
        if action_iris.get(&hash).map(String::as_str) != Some(action_iri) {
            return Err("Ethogram behavior names an unknown action IRI".into());
        }
        let action = ontology
            .affordances
            .iter()
            .find(|action| action.action_id == hash)
            .ok_or("Ethogram behavior has no action definition")?;
        let trigger_iri = iri(required(row, "topic_iri")?)?;
        let category = stimulus_category(text(required(row, "category")?)?)?;
        let threshold = meter(row, "min_intensity_permill", 0)?;
        let flags = u16_value(row, "required_flags", 0)?;
        let distance = row
            .get("max_distance_m")
            .map(num)
            .transpose()?
            .unwrap_or(1.0e9);
        if !(0.0..=1.0e9).contains(&distance) {
            return Err("Ethogram distance bound is invalid".into());
        }
        let priority = u8::try_from(u16_value(row, "priority", 0)?)
            .map_err(|_| "Ethogram priority exceeds u8")?;
        let target_state_id = u16_value(row, "target_state_id", 0)?;
        if trigger_iri != topic
            || category != stimulus.category
            || stimulus.intensity_permill < threshold
            || f64::from(stimulus.distance_meters) > distance
            || (observer.flags & flags) != flags
        {
            continue;
        }
        let Ok(verdict) = evaluate(ontology, hash, Some(topic), observer) else {
            continue;
        };
        let effects = &action.effects;
        if !registry.add_behavior(EthogramBehaviorEntry {
            action_id: hash,
            trigger_stimulus_hash: stimulus.topic_hash,
            target_state_id,
            precondition_mask: flags,
            animation_clip_id: effects.animation_clip_id,
            energy_delta_permill: effects.energy_delta,
            hydration_delta_permill: effects.hydration_delta,
            damage_delta_permill: effects.damage_delta,
            priority,
            category: action.category as u8,
        }) {
            return Err("Ethogram registry is full".into());
        }
        // Native evaluation breaks equal priorities in favor of the last match.
        if best.as_ref().is_none_or(|(old, _)| priority >= *old) {
            best = Some((
                priority,
                Fired {
                    hash,
                    event_iri: Some(topic),
                    verdict,
                },
            ));
        }
    }
    let transition = registry.evaluate_stimulus(stimulus, state);
    let Some((_, fired)) = best else {
        return Ok(None);
    };
    if !transition.triggered
        || transition.new_energy_permill != fired.verdict.new_energy
        || transition.new_hydration_permill != fired.verdict.new_hydration
        || transition.new_damage_permill != fired.verdict.new_damage
        || transition.animation_clip_id != fired.verdict.animation_clip_id
    {
        return Err("Ethogram transition diverged from the authorized entity action".into());
    }
    state.wear_permill = fired.verdict.new_wear;
    Ok(Some(fired))
}

fn wing_label(wing: RepresentationWing) -> &'static str {
    match wing {
        RepresentationWing::Private => "private",
        RepresentationWing::Offered => "offered",
        RepresentationWing::Commons => "commons",
    }
}

fn projection(
    entity: &Args,
    meta: &EntityViewMeta,
    decision: ViewDecision,
) -> Result<Value, String> {
    let [mut lat, mut lon] = [None; 2];
    if let Some(value) = entity.get("location_lat_lon") {
        let [y, x] = vector::<2>(value)?;
        if !(-90.0..=90.0).contains(&y) || !(-180.0..=180.0).contains(&x) {
            return Err("Projection geographic coordinates exceed valid latitude/longitude".into());
        }
        if decision.wing == RepresentationWing::Private {
            lat = Some(y as f32);
            lon = Some(x as f32);
        }
    }
    let identifier = [meta.entity_id];
    let (private, offered, commons): (&[EntityId], &[EntityId], &[EntityId]) = match decision.wing {
        RepresentationWing::Private => (&identifier, &[], &[]),
        RepresentationWing::Offered => (&[], &identifier, &[]),
        RepresentationWing::Commons => (&[], &[], &identifier),
    };
    let key = format!(
        "{:016x}:{}",
        meta.entity_id.raw(),
        wing_label(decision.wing)
    );
    let package = BifurcatedPackage::new(&key, private, offered, commons);
    let wing = package.wing_for(decision.wing);
    let mut nodes = [SceneNodeProj {
        entity_id: 0,
        id: String::new(),
        x: 0.0,
        y: 0.0,
        z: 0.0,
        color: String::new(),
        radius: 0.0,
        alpha: 0.0,
        affordance_bits: 0,
    }];
    layout_scene_nodes(
        &[LayoutInput {
            entity_id: meta.entity_id,
            lat,
            lon,
            affordances: decision.affordances,
            wing: decision.wing,
        }],
        &mut nodes,
    );
    let [node] = nodes;
    Ok(Value::Record(BTreeMap::from([
        (
            "wing".into(),
            Value::String(wing_label(decision.wing).into()),
        ),
        ("entity_id".into(), Value::U64(node.entity_id)),
        ("package_id".into(), Value::U64(package.package_id)),
        ("payload_digest".into(), Value::U64(wing.payload_digest)),
        (
            "affordance_bits".into(),
            Value::U64(u64::from(node.affordance_bits)),
        ),
        (
            "position".into(),
            Value::List(
                [node.x, node.y, node.z]
                    .into_iter()
                    .map(Value::F64)
                    .collect(),
            ),
        ),
        ("color".into(), Value::String(node.color)),
    ])))
}

fn action_receipt(
    entity: &str,
    taxon: &str,
    action: &str,
    observer: &Observer<'_>,
    meta: &EntityViewMeta,
    decision: ViewDecision,
    fired: &Fired<'_>,
    state: EntityPhysiologicalState,
    effects: &BTreeMap<u64, String>,
) -> Result<([u64; 6], ReceiptGraph), String> {
    let mut raw = fired.verdict.record_quin;
    if !raw.verify_ecc_parity() {
        return Err("Native entity action returned an invalid Quin parity".into());
    }
    // Preserve the full actor IRI hash in the real action receipt. The
    // bifurcation filter's sensitivity byte is checked on a view copy only.
    if decision.wing != RepresentationWing::Commons {
        raw.metadata &= !(0b11_u64 << 61);
        raw.parity = NQuin::calculate_parity(
            raw.subject,
            raw.predicate,
            raw.object,
            raw.context,
            raw.metadata,
        );
    }
    let sensitivity = if meta.is_secret {
        3
    } else {
        meta.sensitivity as u8
    };
    let clearance = match decision.wing {
        RepresentationWing::Private => sensitivity,
        RepresentationWing::Offered | RepresentationWing::Commons => 0,
    };
    let mut view = raw;
    view.set_sensitivity_byte(sensitivity);
    view.parity = NQuin::calculate_parity(
        view.subject,
        view.predicate,
        view.object,
        view.context,
        view.metadata,
    );
    let mut policy = BifurcationContext64 {
        observer_did_hash: q_hash(observer.iri),
        subject_did_hash: meta.entity_id.raw(),
        max_sensitivity_class: clearance,
        flags: FLAG_REQUIRE_PARITY | FLAG_STRICT_SUBJECT,
        ..Default::default()
    };
    policy.seal_crc32c();
    if !policy.verify_crc32c() || !evaluate_quin_bifurcation(&policy, &view) {
        return Err(
            "Entity action receipt is outside the selected representation's visibility".into(),
        );
    }
    let columns = [
        raw.subject,
        raw.predicate,
        raw.object,
        raw.context,
        raw.metadata,
        raw.parity,
    ];
    let mut hasher = blake3::Hasher::new();
    for field in columns {
        hasher.update(&field.to_le_bytes());
    }
    let subject = format!("{PREFIX}receipt:{}", hasher.finalize().to_hex());
    let mut graph = ReceiptGraph::new(&format!("{GRAPH}:{}", wing_label(decision.wing)));
    graph.fact(&subject, &format!("{PREFIX}entity"), entity)?;
    graph.fact(&subject, &format!("{PREFIX}taxon"), taxon)?;
    graph.fact(&subject, &format!("{PREFIX}action"), action)?;
    graph.fact(&subject, &format!("{PREFIX}actor"), observer.iri)?;
    graph.fact(
        &subject,
        &format!("{PREFIX}epistemicKind"),
        &format!("{PREFIX}modelEvaluation"),
    )?;
    if let Some(event) = fired.event_iri {
        graph.fact(&subject, &format!("{PREFIX}event"), event)?;
    }
    if let Some(iri) = effects.get(&fired.verdict.emitted_topic_hash) {
        graph.fact(&subject, &format!("{PREFIX}emittedTopic"), iri)?;
    }
    for (key, meter) in [
        ("energyPermill", state.energy_permill),
        ("hydrationPermill", state.hydration_permill),
        ("damagePermill", state.damage_permill),
        ("wearPermill", state.wear_permill),
        ("state", state.active_state_id),
    ] {
        graph.fact(
            &subject,
            &format!("{PREFIX}{key}"),
            &format!("{PREFIX}value:{meter}"),
        )?;
    }
    Ok((columns, graph))
}

fn read_transition_clip(bytes: &[u8], id: u32) -> Result<Option<AnimClip>, String> {
    let section = match read_skeletal_section_payload(bytes) {
        Ok(ContainerSkeletalSection::Skl1(section)) => section,
        Ok(ContainerSkeletalSection::Skl2(_)) | Err(SkeletalContainerError::Missing) => {
            return Ok(None)
        }
        Err(error) => return Err(format!("Entity transition skeletal source: {error:?}")),
    };
    let clip = (0..section.counts().clips)
        .filter_map(|i| section.clip(i))
        .find(|clip| clip.id == u64::from(id))
        .ok_or("The SKL1 source does not contain the selected clip ID")?;
    let mut tracks = Vec::with_capacity(clip.track_count as usize);
    let mut keys = Vec::new();
    for index in clip.first_track..clip.first_track + clip.track_count {
        let mut track = section
            .track(index as usize)
            .ok_or("SKL1 transition track out of range")?;
        if keys.len() + usize::from(track.key_count) > MAX_CLIP_KEYS {
            return Err("SKL1 transition clip exceeds its key budget".into());
        }
        let offset = keys.len() as u32;
        for index in track.first_key..track.first_key + u32::from(track.key_count) {
            keys.push(
                section
                    .key(index as usize)
                    .ok_or("SKL1 transition key out of range")?,
            );
        }
        track.first_key = offset;
        tracks.push(track);
    }
    Ok(Some(AnimClip {
        duration_seconds: clip.duration,
        looping: clip.looping,
        tracks,
        motor_keys: keys,
        morph_keys: Vec::new(),
    }))
}

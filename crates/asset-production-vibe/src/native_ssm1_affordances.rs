//! Bounded phenotype PCA and native SSM1/EONT authoring.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use qualia_cooperative_core::contextual_evidence::EpistemicTier;
use qualia_core_db::{
    bundle::semantic_model::{resource_identity_meta, SemanticModelBuilder},
    container_10d::{
        self as ten,
        skeletal_section::{EpistemicProvenanceWord, Ssm1BasisHeader, Ssm1ModeWeight, SSM1_MAGIC},
    },
    entity_view::{
        entity_id::EntityId,
        entity_ontologies::{ActionAffordance, AffordanceCategory, EntityOntology, LifecycleStage},
        observer::ObserverStatus,
    },
    q_hash,
    resolver::pack_typed_literal,
    specialized_libs::linear_algebra::eigen_symmetric,
    NQuin,
};

use crate::{
    artifact::FieldLattice, count, fields, integer, list, num, number,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text, vector, Args,
    AssetHost, TypedArtifact, Value,
};

const MAX_PHENOTYPES: usize = 16;
const MAX_MODES: usize = 8;
const MAX_VERTICES: usize = 16_384;
const SEMANTIC_BYTES: usize = 42 * 1024 * 1024;
const GRAPH: &str = "https://qualiadb.org/ontology/kinematics#OntogeneticStructure";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const EONT: &str = "https://qualiadb.org/ontology/hmc#Affordance";

macro_rules! kine {
    ($name:literal) => {
        concat!("https://qualiadb.org/ontology/kinematics#", $name)
    };
}

impl AssetHost<'_> {
    pub(crate) fn compile_ssm1_and_eont_affordances(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "taxon_iri",
                "envelope",
                "frame_mass_kg",
                "phenotypes",
                "mode_count",
                "affordances",
                "developmental_keyframes",
            ],
        )?;
        let source = required(args, "source")?;
        let limit = self
            .limits
            .resource_bytes
            .min(ten::GRAPHICS_ASSET_RING_BYTES);
        let taxon = text(required(args, "taxon_iri")?)?;
        let taxon_hash = iri(taxon)?;
        let envelope = envelope(required(args, "envelope")?)?;
        let frame_mass = admitted_mass(num(required(args, "frame_mass_kg")?)?, &envelope)
            .map_err(|error| format!("SSM1 frame {error}"))?;
        let phenotypes = list(required(args, "phenotypes")?)?;
        if !(2..=MAX_PHENOTYPES).contains(&phenotypes.len()) {
            return Err("SSM1 requires 2–16 aligned phenotype samples".into());
        }
        let modes = count(args, "mode_count", 1, MAX_MODES.min(phenotypes.len() - 1))?;
        let mut samples = Vec::with_capacity(phenotypes.len());
        for (index, value) in phenotypes.iter().enumerate() {
            let row = record(value)?;
            fields(row, &["mesh", "mass_kg"])?;
            let mass = admitted_mass(num(required(row, "mass_kg")?)?, &envelope)
                .map_err(|error| format!("Phenotype {index} {error}"))?;
            let TypedArtifact::AttributedMesh(mesh) =
                self.workspace.typed(required(row, "mesh")?)?
            else {
                return Err("Each SSM1 phenotype requires an attributed mesh".into());
            };
            samples.push((Arc::clone(mesh), mass));
        }
        let vertices = samples[0].0.surface.mesh.positions.len();
        if !(3..=MAX_VERTICES).contains(&vertices) || samples[0].0.surface.mesh.triangles.is_empty()
        {
            return Err("SSM1 requires a nonempty bounded phenotype surface".into());
        }
        let source_len = {
            let asset = encoded_resource(self, source)?;
            if asset.kind != "10d" || asset.bytes.len() > limit {
                return Err(
                    "SSM1 source must be a bounded sealed .10d with an authored SKL2 rig".into(),
                );
            }
            asset.bytes.len()
        };
        self.charge(
            source_len
                .saturating_mul(4)
                .saturating_add(vertices * phenotypes.len() * phenotypes.len() * 12),
            source_len
                .saturating_mul(3)
                .saturating_add(vertices * (modes + phenotypes.len()) * 40),
        )?;

        let bytes = &encoded_resource(self, source)?.bytes;
        let section = ten::skeletal_container::read_skl2_section(bytes)
            .map_err(|error| format!("SSM1 source rig: {error:?}"))?;
        let mut rig = section;
        if !rig.ssm_headers.is_empty() || !rig.ssm_modes.is_empty() {
            return Err("SSM1 source already supplies its own statistical basis".into());
        }
        let mut field_scratch =
            [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
        let mut field_records = field_scratch;
        let view =
            ten::TypedPayloadResolver::new(bytes, &[], &mut field_scratch, &mut field_records)
                .map_err(|error| error.to_string())?;
        if view
            .section_payload(ten::SectionType::EntityOntology)
            .is_some()
            || view
                .payloads_of_kind(ten::FIELD_KIND_ARCHETYPE_REF)
                .next()
                .is_some()
        {
            return Err(
                "SSM1 requires an independent leaf frame without an existing EONT section".into(),
            );
        }
        let frame = ten::decode_mesh_section(
            view.section_payload(ten::SectionType::QuantizedMesh)
                .ok_or("SSM1 source requires explicit quantized topology")?,
        )
        .map_err(|error| format!("{error:?}"))?;
        let (_, spatial) = SemanticModelBuilder::new(SEMANTIC_BYTES)
            .merge_10d(bytes)
            .map_err(|error| error.to_string())?;
        let spatial = spatial
            .filter(|entry| entry.stratum_mask != 0)
            .ok_or("SSM1 source requires addressed HOLO and MSTR")?;
        let root = &spatial.root_entity_iri;
        let mut report = Vec::with_capacity(samples.len() + 1);
        let mut aligned = Vec::with_capacity(samples.len());
        if frame.positions.len() != vertices
            || frame
                .triangles
                .iter()
                .flatten()
                .any(|&v| v as usize >= vertices)
        {
            return Err(
                "SSM1 frame topology does not match the supplied phenotype surfaces".into(),
            );
        }
        let frame_extents =
            ten::verify_metric_trait_bounds_zero_heap(&frame.positions, 1.0, &envelope)
                .map_err(|error| format!("SSM1 frame metric gate: {error}"))?;
        for (index, (mesh, mass)) in samples.iter().enumerate() {
            let source = &mesh.surface.mesh;
            if source.triangles != frame.triangles || source.positions.len() != vertices {
                return Err(format!(
                    "Phenotype {index} does not share the frame's indexed vertex correspondence"
                ));
            }
            let extents =
                ten::verify_metric_trait_bounds_zero_heap(&source.positions, 1.0, &envelope)
                    .map_err(|error| format!("Phenotype {index} metric gate: {error}"))?;
            aligned.push(source.positions.as_slice());
            report.push(Value::Record(Args::from([
                ("sample".into(), Value::U64(index as u64)),
                ("mass_kg".into(), Value::F64(*mass)),
                ("extents_m".into(), vector_value(extents)),
            ])));
        }
        let (basis, descriptors) = phenotype_pca(&aligned, modes)?;
        rig.ssm_headers.push(
            Ssm1BasisHeader {
                magic: SSM1_MAGIC,
                version: 1,
                mode_count: modes as u16,
                target_vertex_count: vertices as u32,
                target_joint_count: rig.joints.len() as u16,
                flags: 0,
                species_taxon_hash: taxon_hash,
                provenance: EpistemicProvenanceWord::pack(
                    EpistemicTier::AuthorialIntent.rank(),
                    0,
                    0,
                    2,
                    0,
                    0,
                ),
                record_crc32c: 0,
            }
            .with_sealed_crc(),
        );
        rig.ssm_modes = descriptors;
        let mut graph = Graph::new();
        graph.fact(root, kine!("phenotypeTaxon"), taxon)?;
        graph.fact(
            root,
            kine!("phenotypeEvidenceTier"),
            EpistemicTier::AuthorialIntent.as_iri(),
        )?;
        let mut ontology = EntityOntology::new(EntityId::from_u64(spatial.root_q_hash), taxon);
        let developmental = keyframes(args.get("developmental_keyframes"), root, &mut graph)?;
        if let Some(first) = developmental.first() {
            ontology.stage = LifecycleStage::from_u8(first.stage);
        }
        let actions = list(required(args, "affordances")?)?;
        if !(1..=ten::MAX_AFFORDANCES_PER_SECTION).contains(&actions.len()) {
            return Err("SSM1 requires a bounded nonempty EONT affordance list".into());
        }
        let joints = rig
            .joints
            .iter()
            .map(|joint| joint.joint_id)
            .collect::<BTreeSet<_>>();
        let mut action_ids = BTreeSet::new();
        for action in actions {
            let action = read_action(action, root, &joints, &mut graph)?;
            if !action_ids.insert(action.action_id) {
                return Err("Duplicate EONT action identity".into());
            }
            ontology.affordances.push(action);
        }
        let mut eont = vec![0; ten::entity_ontology_encoded_len(ontology.affordances.len())];
        ten::encode_entity_ontology_section(&ontology, taxon_hash, &mut eont)
            .map_err(|error| error.to_string())?;
        let cbld = graph.encode()?;
        let skl2 = rig.to_bytes_v2().map_err(|error| format!("{error:?}"))?;
        let sealed = reseal(bytes, &view, &skl2, &eont, &cbld, limit)?;
        let final_rig = ten::skeletal_container::read_skl2_section(&sealed)
            .map_err(|error| format!("{error:?}"))?;
        if final_rig.ssm_modes != rig.ssm_modes {
            return Err("SSM1 binary read-back differs from the fitted modes".into());
        }
        SemanticModelBuilder::new(SEMANTIC_BYTES)
            .merge_10d(&sealed)
            .and_then(|(model, _)| model.finish())
            .map_err(|error| format!("SSM1/EONT semantic admission: {error}"))?;
        let basis = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![(modes + 1) as u32, vertices as u32, 3],
                samples: basis,
            },
        )))?;
        let result = self.admit_encoded("10d", sealed)?;
        Ok(Value::Record(Args::from([
            ("container_10d".into(), result),
            ("basis".into(), basis),
            (
                "metric_gate_receipt".into(),
                Value::Record(Args::from([
                    ("frame_extents_m".into(), vector_value(frame_extents)),
                    ("frame_mass_kg".into(), Value::F64(frame_mass)),
                    ("phenotypes".into(), Value::List(report)),
                ])),
            ),
        ])))
    }
}

fn vector_value<const N: usize>(values: [f32; N]) -> Value {
    Value::List(
        values
            .into_iter()
            .map(|value| Value::F64(f64::from(value)))
            .collect(),
    )
}

fn envelope(value: &Value) -> Result<ten::MetricTraitEnvelope, String> {
    let row = record(value)?;
    fields(row, &["length_m", "width_m", "height_m", "mass_kg"])?;
    let range = |key| -> Result<[f32; 2], String> {
        let values = vector::<2>(required(row, key)?)?.map(|value| value as f32);
        if values[0] <= 0.0 || !values[1].is_finite() || values[0] > values[1] {
            return Err(format!("Invalid positive SSM1 envelope range {key}"));
        }
        Ok(values)
    };
    let [min_length_m, max_length_m] = range("length_m")?;
    let [min_width_m, max_width_m] = range("width_m")?;
    let [min_height_m, max_height_m] = range("height_m")?;
    let [min_mass_kg, max_mass_kg] = range("mass_kg")?;
    Ok(ten::MetricTraitEnvelope {
        entity_class_id: 0,
        min_length_m,
        max_length_m,
        min_width_m,
        max_width_m,
        min_height_m,
        max_height_m,
        min_mass_kg,
        max_mass_kg,
    })
}

fn admitted_mass(mass: f64, envelope: &ten::MetricTraitEnvelope) -> Result<f64, String> {
    let (min, max) = (
        f64::from(envelope.min_mass_kg),
        f64::from(envelope.max_mass_kg),
    );
    if !(min..=max).contains(&mass) {
        return Err(format!("mass {mass} kg outside [{min}, {max}] kg"));
    }
    Ok(mass)
}

fn phenotype_pca(
    samples: &[&[[f32; 3]]],
    modes: usize,
) -> Result<(Vec<f32>, Vec<Ssm1ModeWeight>), String> {
    // Diagonalize the sample Gram matrix rather than a vertex-by-vertex covariance.
    let n = samples.len();
    let dofs = samples[0].len() * 3;
    let mut mean = vec![0.0_f64; dofs];
    for sample in samples {
        for (center, &value) in mean.iter_mut().zip(sample.iter().flatten()) {
            *center += f64::from(value);
        }
    }
    for center in &mut mean {
        *center /= n as f64;
    }
    let centered = samples
        .iter()
        .map(|sample| {
            sample
                .iter()
                .flatten()
                .zip(&mean)
                .map(|(&value, &center)| f64::from(value) - center)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let divisor = (n - 1) as f64;
    let mut gram = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..=i {
            let dot = centered[i]
                .iter()
                .zip(&centered[j])
                .map(|(a, b)| a * b)
                .sum::<f64>()
                / divisor;
            gram[i * n + j] = dot;
            gram[j * n + i] = dot;
        }
    }
    let scale = gram.iter().map(|value| value.abs()).fold(0.0, f64::max);
    if scale == 0.0 || !scale.is_finite() {
        return Err("Phenotype samples have no finite shape variation".into());
    }
    for value in &mut gram {
        *value /= scale;
    }
    let (eigenvalues, eigenvectors) =
        eigen_symmetric(n, &gram).map_err(|error| format!("Native SSM1 PCA: {error:?}"))?;
    let mut order = (0..n).collect::<Vec<_>>();
    order.sort_by(|&a, &b| eigenvalues[b].total_cmp(&eigenvalues[a]));
    let mut basis = mean.into_iter().map(|x| x as f32).collect::<Vec<_>>();
    let mut descriptors = Vec::with_capacity(modes);
    for &column in &order[..modes] {
        let eigenvalue = (eigenvalues[column] * scale) as f32;
        if eigenvalues[column] <= 1e-9 * eigenvalues[order[0]]
            || eigenvalue <= 0.0
            || !eigenvalue.is_finite()
        {
            return Err("Requested SSM1 mode count exceeds the finite sample rank".into());
        }
        let displacement = (0..dofs)
            .map(|dof| {
                (0..n)
                    .map(|sample| centered[sample][dof] * eigenvectors[sample * n + column])
                    .sum::<f64>()
                    / divisor.sqrt()
            })
            .collect::<Vec<_>>();
        let pivot = displacement
            .iter()
            .max_by(|a, b| a.abs().total_cmp(&b.abs()))
            .copied()
            .unwrap_or(0.0);
        let sign = if pivot < 0.0 { -1.0 } else { 1.0 };
        let (min_sigma, max_sigma) = (0..n)
            .map(|sample| sign * divisor.sqrt() * eigenvectors[sample * n + column])
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), x| {
                (lo.min(x), hi.max(x))
            });
        basis.extend(displacement.into_iter().map(|value| (sign * value) as f32));
        descriptors.push(Ssm1ModeWeight {
            mode_id: descriptors.len() as u32 + 1,
            eigenvalue,
            min_sigma: min_sigma as f32,
            max_sigma: max_sigma as f32,
        });
    }
    if basis.iter().any(|value| !value.is_finite()) {
        return Err("Nonfinite SSM1 basis".into());
    }
    Ok((basis, descriptors))
}

fn iri(value: &str) -> Result<u64, String> {
    if value.len() > 4096 {
        return Err("SSM1/EONT term exceeds its IRI limit".into());
    }
    resource_identity_meta(value).map_err(|error| error.to_string())?;
    let hash = q_hash(value);
    if hash == 0 {
        return Err("SSM1/EONT term has a zero identity hash".into());
    }
    Ok(hash)
}

fn read_action(
    value: &Value,
    root: &str,
    joints: &BTreeSet<u64>,
    graph: &mut Graph,
) -> Result<ActionAffordance, String> {
    let row = record(value)?;
    fields(
        row,
        &[
            "action_iri",
            "anchor_joint_iri",
            "capability_iri",
            "topic_iri",
            "category",
        ],
    )?;
    let action_iri = text(required(row, "action_iri")?)?;
    let joint = text(required(row, "anchor_joint_iri")?)?;
    if !joints.contains(&iri(joint)?) {
        return Err("EONT anchor must address a joint of the sealed SKL2 frame".into());
    }
    let category = match row
        .get("category")
        .map(text)
        .transpose()?
        .unwrap_or("physical")
    {
        "physical" => AffordanceCategory::Physical,
        "biological" => AffordanceCategory::Biological,
        "sensing" => AffordanceCategory::Sensing,
        _ => return Err("Unsupported SSM1/EONT affordance category".into()),
    };
    let mut result = ActionAffordance::new(action_iri, "Authored EONT affordance", category);
    result.preconditions.min_observer_status = ObserverStatus::Public;
    graph.fact(action_iri, RDF_TYPE, EONT)?;
    graph.fact(root, kine!("affordance"), action_iri)?;
    graph.fact(action_iri, kine!("anchorJoint"), joint)?;
    if let Some(value) = row.get("capability_iri") {
        let cap = text(value)?;
        result.preconditions.required_capability_hash = iri(cap)?;
        graph.fact(
            action_iri,
            "https://qualiadb.org/ontology/hmc#requiredCapability",
            cap,
        )?;
    }
    if let Some(value) = row.get("topic_iri") {
        let topic = text(value)?;
        result.effects.emitted_topic_hash = iri(topic)?;
        graph.fact(
            action_iri,
            "https://qualiadb.org/ontology/hmc#emittedTopic",
            topic,
        )?;
    }
    Ok(result)
}

fn keyframes(
    input: Option<&Value>,
    root: &str,
    graph: &mut Graph,
) -> Result<Vec<ten::DevelopmentalKeyframeRecord>, String> {
    let Some(input) = input else {
        return Ok(Vec::new());
    };
    let entries = list(input)?;
    if entries.len() > ten::MAX_DEVELOPMENTAL_KEYFRAMES {
        return Err("SSM1 developmental keyframe count exceeds the native limit".into());
    }
    let mut result = Vec::<ten::DevelopmentalKeyframeRecord>::new();
    for (index, value) in entries.iter().enumerate() {
        let row = record(value)?;
        fields(
            row,
            &[
                "age_days",
                "stage",
                "crown_scale",
                "trunk_scale",
                "lai_multiplier",
            ],
        )?;
        let age = num(required(row, "age_days")?)? as f32;
        let stage = integer(required(row, "stage")?, "stage")?;
        if stage > 4 {
            return Err("Developmental stage must be a native lifecycle value in [0,4]".into());
        }
        let stage = stage as u8;
        let crown = number(row, "crown_scale", 1.0)? as f32;
        let trunk = number(row, "trunk_scale", 1.0)? as f32;
        let lai = number(row, "lai_multiplier", 1.0)? as f32;
        if !(0.0..=1e8).contains(&age)
            || !(0.001..=1000.0).contains(&crown)
            || !(0.001..=1000.0).contains(&trunk)
            || !(0.0..=1000.0).contains(&lai)
            || result
                .last()
                .is_some_and(|previous| age <= previous.age_days || stage < previous.stage)
        {
            return Err("Developmental keyframes require nondecreasing stages, increasing ages and bounded coefficients".into());
        }
        let id = format!("urn:qualia:ontogeny:{:016x}:keyframe:{index}", q_hash(root));
        graph.fact(root, kine!("developmentalKeyframe"), &id)?;
        for (predicate, value) in [
            (kine!("ageDays"), age),
            (kine!("lifecycleStage"), f32::from(stage)),
            (kine!("crownScale"), crown),
            (kine!("trunkScale"), trunk),
            (kine!("laiMultiplier"), lai),
        ] {
            graph.number(&id, predicate, value)?;
        }
        result.push(ten::DevelopmentalKeyframeRecord {
            age_days: age,
            stage,
            reserved_u8: 0,
            reserved_u16: 0,
            allometric_crown_scale: crown,
            allometric_trunk_scale: trunk,
            lai_multiplier: lai,
            reserved_u32: 0,
            mesh_asset_hash: 0,
        });
    }
    Ok(result)
}

struct Graph {
    terms: BTreeMap<u64, String>,
    facts: Vec<[u64; 6]>,
}

impl Graph {
    fn new() -> Self {
        Self {
            terms: BTreeMap::new(),
            facts: Vec::new(),
        }
    }
    fn term(&mut self, value: &str) -> Result<u64, String> {
        let hash = iri(value)?;
        if self
            .terms
            .get(&hash)
            .is_some_and(|previous| previous != value)
        {
            return Err("SSM1/EONT graph contains a conflicting IRI".into());
        }
        self.terms.entry(hash).or_insert_with(|| value.into());
        Ok(hash)
    }
    fn add(&mut self, s: &str, p: &str, o: u64) -> Result<(), String> {
        let (s, p, c) = (self.term(s)?, self.term(p)?, self.term(GRAPH)?);
        self.facts
            .push([s, p, o, c, 0, NQuin::calculate_parity(s, p, o, c, 0)]);
        Ok(())
    }
    fn fact(&mut self, s: &str, p: &str, o: &str) -> Result<(), String> {
        let o = self.term(o)?;
        self.add(s, p, o)
    }
    fn number(&mut self, s: &str, p: &str, value: f32) -> Result<(), String> {
        let literal =
            pack_typed_literal(&value.to_string(), "xsd:float").ok_or("Invalid SSM1 number")?;
        self.add(s, p, literal)
    }
    fn encode(mut self) -> Result<Vec<u8>, String> {
        self.facts.sort_unstable();
        self.facts.dedup();
        ten::encode_symbolic_rdf_cbor_ld(&ten::SymbolicRdfCborLdPayload {
            context_iri: GRAPH.into(),
            nquins: self.facts,
            lexicon: self
                .terms
                .into_iter()
                .map(|(term_hash, iri_utf8)| ten::MultilingualLexicalTerm {
                    term_hash,
                    iri_utf8,
                    lang_bcp47: None,
                    label_utf8: String::new(),
                })
                .collect(),
            ..Default::default()
        })
        .map_err(|error| error.to_string())
    }
}

fn reseal(
    bytes: &[u8],
    view: &ten::TypedPayloadResolver<'_, '_>,
    rig: &[u8],
    eont: &[u8],
    cbld: &[u8],
    limit: usize,
) -> Result<Vec<u8>, String> {
    let header = ten::Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    let identity = view.identity().ok_or("SSM1 frame lacks MID2")?;
    let mut fields = view.fields().to_vec();
    if fields
        .iter()
        .any(|field| field.payload_section_id == ten::SectionType::SkeletalAnimation as u32)
    {
        return Err("SSM1 frame carries typed payload ranges into its skeletal section".into());
    }
    let mut sidecar = view
        .section_payload(ten::SectionType::FieldSidecar)
        .unwrap_or_default()
        .to_vec();
    let field_id = fields
        .last()
        .map_or(0, |field| field.field_id)
        .checked_add(1)
        .ok_or("SSM1 field ID overflow")?;
    fields.push(ten::TypedFieldRecord {
        field_id,
        kind: ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        payload_section_id: ten::SectionType::FieldSidecar as u32,
        payload_offset: sidecar.len() as u64,
        payload_length: cbld.len() as u64,
        ..Default::default()
    });
    sidecar.extend_from_slice(cbld);
    let mut mid2 = vec![
        0;
        ten::manifold_identity_v2_encoded_len(fields.len())
            .map_err(|error| format!("{error:?}"))?
    ];
    ten::encode_manifold_identity_v2(identity, &fields, &mut mid2)
        .map_err(|error| format!("{error:?}"))?;
    let mut sections = Vec::new();
    let mut sidecar_tier = ten::AlignmentTier::Page;
    let mut mid2_tier = ten::AlignmentTier::Page;
    let mut skl_tier = ten::AlignmentTier::Word;
    for descriptor in ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())? {
        let section_type = descriptor.typ().ok_or("Unknown SSM1 source section")?;
        let tier = descriptor.tier().ok_or("Invalid SSM1 source alignment")?;
        match section_type {
            ten::SectionType::FieldSidecar => sidecar_tier = tier,
            ten::SectionType::ManifoldIdentityV2 => mid2_tier = tier,
            ten::SectionType::SkeletalAnimation => skl_tier = tier,
            _ => sections.push(ten::SectionInput {
                section_type,
                alignment_tier: tier,
                stride: descriptor.stride,
                element_count: descriptor.element_count,
                payload: view
                    .section_payload(section_type)
                    .ok_or("Missing SSM1 source payload")?,
            }),
        }
    }
    for (section_type, alignment_tier, payload) in [
        (
            ten::SectionType::ManifoldIdentityV2,
            mid2_tier,
            mid2.as_slice(),
        ),
        (
            ten::SectionType::FieldSidecar,
            sidecar_tier,
            sidecar.as_slice(),
        ),
        (ten::SectionType::SkeletalAnimation, skl_tier, rig),
        (
            ten::SectionType::EntityOntology,
            ten::AlignmentTier::Word,
            eont,
        ),
    ] {
        sections.push(ten::SectionInput {
            section_type,
            alignment_tier,
            stride: 0,
            element_count: 0,
            payload,
        });
    }
    let need = match ten::encode_container(&header, &sections, &mut []) {
        Err(ten::SectionTableError::OutputBufferTooSmall { needed, .. }) if needed <= limit => {
            needed
        }
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("SSM1 container size was not reported".into()),
    };
    let mut result = vec![0; need];
    let actual = ten::encode_container(&header, &sections, &mut result)
        .map_err(|error| error.to_string())?;
    result.truncate(actual);
    ten::seal_whole_file_crc32c(&mut result);
    Ok(result)
}

//! Bounded Vibe authoring adapter for `.10d` SKL2 and SSM1 descriptors.

use std::{collections::BTreeMap, sync::Arc};

use qualia_core_db::{container_10d as ten, q_hash};
use ten::skeletal_container::SkeletalContainerError;
use ten::skeletal_section::{
    EpistemicProvenanceWord, KinematicSolverRegime, LoopClosureConstraintRecord,
    SKL2_MAX_JOINTS, SKL2_NONE_INDEX, SSM1_MAGIC, SkeletalSectionPayload,
    Skl2BuildError, Skl2JointDescriptorV2, Ssm1BasisHeader, Ssm1ModeWeight,
    SwingTwistConeLimits, seal_authored_skl2_into,
};

use crate::{Args, AssetHost, TypedArtifact, Value, fields, integer, list, number, record, required, text, vector};

impl AssetHost<'_> {
    pub(crate) fn compile_skl2_rig(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "joints", "loop_closures", "ssm_basis"])?;
        let joints = list(required(args, "joints")?)?;
        if joints.is_empty() || joints.len() > SKL2_MAX_JOINTS {
            return Err(format!("SKL2 rig needs 1..={SKL2_MAX_JOINTS} joints"));
        }
        let mut rig = SkeletalSectionPayload::new(joints.iter().map(joint).collect::<Result<_, _>>()?);
        let joint_iris = joints.iter().map(joint_iri).collect::<Result<Vec<_>, _>>()?;
        let mut closure_iris = Vec::new();
        if let Some(value) = args.get("loop_closures") {
            let rows = list(value)?;
            if rows.len() > SKL2_MAX_JOINTS {
                return Err("SKL2 loop-closure count exceeded".into());
            }
            rig.loop_closures = rows.iter().map(closure).collect::<Result<_, _>>()?;
            closure_iris = rows.iter().map(joint_iri).collect::<Result<_, _>>()?;
        }
        let mut taxon_iri = None;
        if let Some(value) = args.get("ssm_basis") {
            let (header, modes) = ssm_basis(value, joints.len())?;
            taxon_iri = Some(text(required(record(value)?, "taxon_iri")?)?);
            rig.ssm_headers.push(header);
            rig.ssm_modes = modes;
        }
        rig.validate().map_err(|error| format!("SKL2 rig: {error:?}"))?;
        let source = self.workspace.encoded(required(args, "source")?)?;
        let limit = self.limits.resource_bytes.min(ten::GRAPHICS_ASSET_RING_BYTES);
        if source.kind != "10d" || source.bytes.len() > limit {
            return Err("SKL2 source must be an encoded .10d within its ring budget".into());
        }
        let enriched = seal_skl2_semantics(&source.bytes, &rig, &joint_iris, &closure_iris, taxon_iri, limit)?;
        let mut rig_scratch = vec![0; rig.encoded_v2_size()];
        let capacity = match seal_authored_skl2_into(&enriched, &rig, &mut rig_scratch, &mut []) {
            Err(Skl2BuildError::Enclosure(SkeletalContainerError::Sections(
                ten::SectionTableError::OutputBufferTooSmall { needed, .. },
            ))) if needed <= limit => needed,
            Err(error) => return Err(format!("SKL2 envelope admission: {error:?}")),
            Ok(_) => return Err("SKL2 enclosure did not report its output size".into()),
        };
        let mut out = vec![0; capacity];
        // The dry writer has already validated the graph and encoded `rig_scratch`.
        let size = ten::skeletal_container::attach_skeletal_section_into(&enriched, &rig_scratch, &mut out)
            .map_err(|error| format!("SKL2 enclosure: {error:?}"))?;
        out.truncate(size);
        self.charge(source.bytes.len().saturating_add(enriched.len()).saturating_add(size),
            size.saturating_add(enriched.len()).saturating_add(rig_scratch.len()))?;
        let rig = self.admit_artifact(TypedArtifact::SkeletalRig(Arc::new(rig)))?;
        let container = self.admit_encoded("10d", out)?;
        Ok(Value::Record(BTreeMap::from([
            ("rig".into(), rig),
            ("container_10d".into(), container),
        ])))
    }
}

fn joint_iri(value: &Value) -> Result<&str, String> {
    text(required(record(value)?, "iri")?)
}

fn seal_skl2_semantics(
    bytes: &[u8],
    rig: &SkeletalSectionPayload,
    joints: &[&str],
    closures: &[&str],
    taxon: Option<&str>,
    limit: usize,
) -> Result<Vec<u8>, String> {
    use qualia_core_db::{bundle::semantic_model::SemanticModelBuilder, NQuin};
    const GRAPH: &str = "https://qualiadb.org/ontology/kinematics#Skl2Structure";
    const JOINT: &str = "https://qualiadb.org/ontology/kinematics#Skl2Joint";
    const CLOSURE: &str = "https://qualiadb.org/ontology/kinematics#Skl2Closure";
    const RELATION: &str = "https://qualiadb.org/ontology/kinematics#parentJoint";
    const END_A: &str = "https://qualiadb.org/ontology/kinematics#closureEndpointA";
    const END_B: &str = "https://qualiadb.org/ontology/kinematics#closureEndpointB";
    const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

    let header = ten::Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    if ten::compute_whole_file_crc32c(bytes) != header.header_crc32c {
        return Err("Invalid SKL2 source checksum".into());
    }
    let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut entries = scratch;
    let view = ten::TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut entries)
        .map_err(|error| error.to_string())?;
    let identity = view.identity().ok_or("SKL2 semantic authoring requires MID2")?;
    let mut terms = BTreeMap::<u64, &str>::new();
    for iri in [GRAPH, JOINT, CLOSURE, RELATION, END_A, END_B, RDF_TYPE]
        .into_iter().chain(joints.iter().copied()).chain(closures.iter().copied()).chain(taxon)
    {
        let hash = q_hash(iri);
        if hash == 0 || terms.insert(hash, iri).is_some_and(|previous| previous != iri) {
            return Err("SKL2 structure contains conflicting semantic identities".into());
        }
    }
    let graph = q_hash(GRAPH);
    let triple = |s, p, o| [s, p, o, graph, 0, NQuin::calculate_parity(s, p, o, graph, 0)];
    let mut facts = Vec::with_capacity(joints.len() * 2 + closures.len() * 3);
    for joint in &rig.joints {
        facts.push(triple(joint.joint_id, q_hash(RDF_TYPE), q_hash(JOINT)));
        if joint.parent_index != SKL2_NONE_INDEX {
            let parent = rig.joints.get(joint.parent_index as usize).ok_or("Invalid semantic parent joint")?.joint_id;
            facts.push(triple(joint.joint_id, q_hash(RELATION), parent));
        }
    }
    for closure in &rig.loop_closures {
        facts.push(triple(closure.constraint_id, q_hash(RDF_TYPE), q_hash(CLOSURE)));
        let a = rig.joints.get(closure.joint_a as usize).ok_or("Invalid semantic closure endpoint A")?.joint_id;
        let b = rig.joints.get(closure.joint_b as usize).ok_or("Invalid semantic closure endpoint B")?.joint_id;
        facts.push(triple(closure.constraint_id, q_hash(END_A), a));
        facts.push(triple(closure.constraint_id, q_hash(END_B), b));
    }
    let cbld = ten::encode_symbolic_rdf_cbor_ld(&ten::SymbolicRdfCborLdPayload {
        context_iri: GRAPH.into(),
        lexicon: terms.into_iter().map(|(term_hash, iri)| ten::MultilingualLexicalTerm {
            term_hash, iri_utf8: iri.into(), lang_bcp47: None, label_utf8: String::new(),
        }).collect(),
        nquins: facts,
        ..Default::default()
    }).map_err(|error| error.to_string())?;
    let mut model = SemanticModelBuilder::new(42 * 1024 * 1024);
    for entry in view.payloads_of_kind(ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        model = model.merge_cbld(entry.map_err(|error| error.to_string())?.bytes)
            .map_err(|error| error.to_string())?;
    }
    model.merge_cbld(&cbld).map_err(|error| error.to_string())?;
    let source_sidecar = view.section_payload(ten::SectionType::FieldSidecar).unwrap_or_default();
    if bytes.len().saturating_add(cbld.len()) > limit {
        return Err("SKL2 semantic fields exceed the 10D budget".into());
    }
    let mut sidecar = source_sidecar.to_vec();
    sidecar.extend(&cbld);
    let mut fields = view.fields().to_vec();
    let field_id = fields.last().map_or(0, |field| field.field_id).checked_add(1)
        .ok_or("SKL2 semantic field IDs exhausted")?;
    fields.push(ten::TypedFieldRecord {
        field_id, kind: ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        payload_section_id: ten::SectionType::FieldSidecar as u32,
        payload_offset: source_sidecar.len() as u64, payload_length: cbld.len() as u64,
        ..Default::default()
    });
    let mid2_len = ten::manifold_identity_v2_encoded_len(fields.len()).map_err(|error| format!("{error:?}"))?;
    let mut mid2 = vec![0; mid2_len];
    ten::encode_manifold_identity_v2(identity, &fields, &mut mid2).map_err(|error| format!("{error:?}"))?;
    let mut sections = Vec::new();
    let mut identity_tier = ten::AlignmentTier::Page;
    let mut sidecar_tier = ten::AlignmentTier::Page;
    for descriptor in ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())? {
        let section_type = descriptor.typ().ok_or("Unknown SKL2 section")?;
        let tier = descriptor.tier().ok_or("Invalid SKL2 alignment")?;
        match section_type {
            ten::SectionType::ManifoldIdentityV2 => { identity_tier = tier; continue; }
            ten::SectionType::FieldSidecar => { sidecar_tier = tier; continue; }
            _ => {}
        }
        let payload = view.section_payload(section_type).ok_or("SKL2 section payload is missing")?;
        sections.push(ten::SectionInput {
            section_type, alignment_tier: tier,
            stride: descriptor.stride, element_count: descriptor.element_count, payload,
        });
    }
    sections.extend([
        ten::SectionInput { section_type: ten::SectionType::ManifoldIdentityV2, alignment_tier: identity_tier, stride: 0, element_count: 0, payload: &mid2 },
        ten::SectionInput { section_type: ten::SectionType::FieldSidecar, alignment_tier: sidecar_tier, stride: 0, element_count: 0, payload: &sidecar },
    ]);
    let capacity = match ten::encode_container(&header, &sections, &mut []) {
        Err(ten::SectionTableError::OutputBufferTooSmall { needed, .. }) if needed <= limit => needed,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("SKL2 semantic writer did not report its output capacity".into()),
    };
    let mut result = vec![0; capacity];
    let written = ten::encode_container(&header, &sections, &mut result).map_err(|error| error.to_string())?;
    result.truncate(written);
    ten::seal_whole_file_crc32c(&mut result);
    Ok(result)
}

fn unsigned<T>(args: &Args, key: &str, fallback: T) -> Result<T, String>
where T: TryFrom<usize>,
{
    match args.get(key) {
        Some(value) => T::try_from(integer(value, key)?).map_err(|_| format!("Out of range {key}")),
        None => Ok(fallback),
    }
}

fn scalar(args: &Args, key: &str, fallback: f64) -> Result<f32, String> {
    let value = number(args, key, fallback)? as f32;
    if value.is_finite() { Ok(value) } else { Err(format!("Invalid {key}")) }
}

fn identity(args: &Args, key: &str) -> Result<u64, String> {
    let iri = text(required(args, key)?)?;
    if iri.len() > 4096 {
        return Err(format!("Semantic IRI exceeds the lexical bound for {key}"));
    }
    qualia_core_db::bundle::semantic_model::resource_identity_meta(iri)
        .map_err(|error| format!("Invalid {key}: {error}"))?;
    Ok(q_hash(iri))
}

fn joint(value: &Value) -> Result<Skl2JointDescriptorV2, String> {
    let row = record(value)?;
    fields(row, &[
        "iri", "parent", "loop_closure_partner", "swing_twist_rad", "solver_regime",
        "cortical_bmd_mg_cm3", "trabecular_bv_tv", "bone_length_m", "segment_mass_kg",
        "epistemic_tier", "observation_method", "confidence_q8", "flags", "provenance_assertion_idx",
    ])?;
    let [swing_y, swing_z, twist_min, twist_max] = vector(required(row, "swing_twist_rad")?)?.map(|v| v as f32);
    let regime = KinematicSolverRegime::from_u8(unsigned(row, "solver_regime", 0)?)
        .map_err(|error| format!("Invalid SKL2 solver regime: {error:?}"))?;
    let mut joint = Skl2JointDescriptorV2::new(
        identity(row, "iri")?, unsigned(row, "parent", SKL2_NONE_INDEX)?,
        SwingTwistConeLimits::new(swing_y, swing_z, twist_min, twist_max), regime,
        scalar(row, "cortical_bmd_mg_cm3", 0.0)?, scalar(row, "trabecular_bv_tv", 0.0)?,
        scalar(row, "bone_length_m", 0.0)?, scalar(row, "segment_mass_kg", 0.0)?,
        unsigned(row, "epistemic_tier", 6)?, unsigned(row, "observation_method", 0)?,
        unsigned(row, "confidence_q8", 0)?,
    );
    joint.loop_closure_partner = unsigned(row, "loop_closure_partner", SKL2_NONE_INDEX)?;
    joint.flags = unsigned(row, "flags", 0)?;
    joint.provenance_assertion_idx = unsigned(row, "provenance_assertion_idx", 0)?;
    joint.seal_crc();
    Ok(joint)
}

fn closure(value: &Value) -> Result<LoopClosureConstraintRecord, String> {
    let row = record(value)?;
    fields(row, &[
        "iri", "joint_a", "joint_b", "closure_type", "flags", "anchor_a_m", "anchor_b_m",
        "stiffness_n_m", "damping_n_s_m", "rest_angle_rad", "tolerance_m", "provenance_word",
    ])?;
    Ok(LoopClosureConstraintRecord {
        constraint_id: identity(row, "iri")?,
        joint_a: unsigned(row, "joint_a", SKL2_NONE_INDEX)?,
        joint_b: unsigned(row, "joint_b", SKL2_NONE_INDEX)?,
        closure_type: unsigned(row, "closure_type", 0)?,
        flags: unsigned(row, "flags", 0)?,
        reserved: 0,
        anchor_a_local: vector(required(row, "anchor_a_m")?)?.map(|v| v as f32),
        anchor_b_local: vector(required(row, "anchor_b_m")?)?.map(|v| v as f32),
        stiffness_n_m: scalar(row, "stiffness_n_m", f64::NAN)?,
        damping_n_s_m: scalar(row, "damping_n_s_m", f64::NAN)?,
        rest_angle_rad: scalar(row, "rest_angle_rad", 0.0)?,
        tolerance_m: scalar(row, "tolerance_m", f64::NAN)?,
        provenance: EpistemicProvenanceWord(unsigned(row, "provenance_word", 0)?),
        record_crc32c: 0,
    }.with_sealed_crc())
}

fn ssm_basis(value: &Value, joints: usize) -> Result<(Ssm1BasisHeader, Vec<Ssm1ModeWeight>), String> {
    let row = record(value)?;
    fields(row, &["taxon_iri", "target_vertex_count", "flags", "provenance_word", "modes"])?;
    let mode_rows = list(required(row, "modes")?)?;
    if mode_rows.is_empty() || mode_rows.len() > ten::MAX_INSTANCE_SSM_COEFFS {
        return Err("SSM1 requires a bounded nonempty mode list".into());
    }
    let mut modes = Vec::with_capacity(mode_rows.len());
    for value in mode_rows {
        let mode = record(value)?;
        fields(mode, &["mode_id", "eigenvalue", "min_sigma", "max_sigma"])?;
        let mode_id = u32::try_from(integer(required(mode, "mode_id")?, "mode_id")?)
            .map_err(|_| "Invalid SSM1 mode id")?;
        modes.push(Ssm1ModeWeight {
            mode_id,
            eigenvalue: scalar(mode, "eigenvalue", f64::NAN)?,
            min_sigma: scalar(mode, "min_sigma", f64::NAN)?,
            max_sigma: scalar(mode, "max_sigma", f64::NAN)?,
        });
    }
    modes.sort_unstable_by_key(|mode| mode.mode_id);
    let header = Ssm1BasisHeader {
        magic: SSM1_MAGIC,
        version: 1,
        mode_count: modes.len() as u16,
        target_vertex_count: unsigned(row, "target_vertex_count", 0)?,
        target_joint_count: joints as u16,
        flags: unsigned(row, "flags", 0)?,
        species_taxon_hash: identity(row, "taxon_iri")?,
        provenance: EpistemicProvenanceWord(unsigned(row, "provenance_word", 0)?),
        record_crc32c: 0,
    }.with_sealed_crc();
    Ok((header, modes))
}

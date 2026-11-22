//! Source-side admission and sealing of ordered external `.10d` leaves.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use qualia_core_db::{NQuin, q_hash};
use qualia_core_db::bundle::semantic_model::{hmc_entry_key_iri, SemanticModelBuilder, resource_identity_meta};
use qualia_core_db::container_10d as ten;
use qualia_core_db::render::skeletal::{Motor, valid_motor};

use crate::{Args, AssetHost, Value, TypedArtifact, fields, list, number, record, required, text, vector};
use crate::artifact::{SceneGraph10d, TenDFields};
use crate::qualia_containers_governance_poet_ops::encoded_resource;
use sha2::{Digest, Sha256};

const RING_BYTES: usize = 128 * 1024 * 1024;
const PART_OF: &str = "http://purl.obolibrary.org/obo/BFO_0000050";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const ARCHETYPE: &str = "https://qualiadb.org/ontology/hmc#archetype";
const ENTRY_KEY: &str = "https://qualiadb.org/ontology/hmc#entryKey";
const MAX_LEAF_IRI_BYTES: usize = 4096;

#[derive(Debug)]
struct Leaf<'a> {
    key: &'a str,
    iri: &'a str,
    entity_iri: &'a str,
    class_iri: &'a str,
    digest: [u8; 32],
    archetype_id: [u8; 16],
    motor: [f32; 8],
    age_days: f32,
    scale: f32,
    coefficients: Vec<ten::SsmInstanceCoeff>,
}

struct AdmittedArchetype<'a> {
    key: &'a str,
    iri: String,
    class_iri: &'a str,
    digest: [u8; 32],
    entity_id: [u8; 16],
}

impl AssetHost<'_> {
    pub(crate) fn assembly_frame_leaf_capacity(frame: &[u8]) -> Result<usize, String> {
        let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
        let mut records = scratch;
        let view = ten::TypedPayloadResolver::new(frame, &[], &mut scratch, &mut records)
            .map_err(|error| error.to_string())?;
        let count = view.payloads_of_kind(ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD).count();
        let limit = leaf_capacity(count);
        if count == 0 || limit == 0 {
            return Err("Assembly frame needs CBLD and capacity for at least one leaf".into());
        }
        Ok(limit)
    }

    /// Compose referenced archetypes from bytes admitted by the host.
    pub(crate) fn compose_assembly_10d(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["assembly_frame", "archetypes", "instances"])?;
        let definitions = list(required(args, "archetypes")?)?;
        let placements = list(required(args, "instances")?)?;
        let limit = self.limits.resource_bytes.min(RING_BYTES);
        let max_leaves = (ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS - 4) / 2;
        if definitions.is_empty() || definitions.len() > max_leaves
            || placements.is_empty() || placements.len() > max_leaves
        {
            return Err(format!("Assembly requires 1..={max_leaves} archetypes and placements"));
        }
        let mut archetypes = BTreeMap::new();
        let mut graph = SemanticModelBuilder::new(limit);
        for definition in definitions {
            let row = record(definition)?;
            fields(row, &["leaf_key", "resource", "taxon_or_class_iri"])?;
            let key = text(required(row, "leaf_key")?)?;
            let class_iri = text(required(row, "taxon_or_class_iri")?)?;
            let value = required(row, "resource")?;
            let resource = encoded_resource(self, value)?;
            if resource.kind != "10d" || resource.bytes.len() > limit {
                return Err("Compose assembly expects bounded .10d archetypes".into());
            }
            self.charge(resource.bytes.len().saturating_mul(4), resource.bytes.len())?;
            let bytes = &encoded_resource(self, value)?.bytes;
            let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
            let mut records = scratch;
            let view = ten::TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut records)
                .map_err(|error| format!("Archetype {key}: {error}"))?;
            let entity_id = view.identity().ok_or("Archetype requires MID2")?.entity_id.0;
            if entity_id == [0; 16] || view.payloads_of_kind(ten::FIELD_KIND_ARCHETYPE_REF).next().is_some() {
                return Err(format!("Archetype {key} must be an independently identified leaf"));
            }
            let (merged, spatial) = graph.merge_10d(bytes)
                .map_err(|error| format!("Archetype {key}: {error}"))?;
            graph = merged;
            let spatial = spatial.ok_or("Archetype requires a locally addressed HOLO root")?;
            if spatial.stratum_mask == 0 {
                return Err("Archetype requires an addressed MSTR stratum".into());
            }
            let archetype = AdmittedArchetype {
                key,
                iri: spatial.root_entity_iri,
                class_iri,
                digest: Sha256::digest(bytes).into(),
                entity_id,
            };
            if archetypes.insert(key, archetype).is_some() {
                return Err(format!("Duplicate archetype key {key}"));
            }
        }
        let leaves = placements.iter().map(|value| {
            let row = record(value)?;
            fields(row, &[
                "leaf_key", "entity_iri", "local_pga_motor", "age_days", "uniform_scale",
                "ssm_coefficients",
            ])?;
            let key = text(required(row, "leaf_key")?)?;
            let archetype = archetypes.get(key).ok_or_else(|| format!("Unknown archetype {key}"))?;
            parse_placement(row, archetype.key, &archetype.iri, archetype.class_iri,
                archetype.digest, archetype.entity_id)
        }).collect::<Result<Vec<_>, _>>()?;
        self.admit_leaf_assembly(required(args, "assembly_frame")?, &leaves)
    }

    /// Seal an assembly around an authored HOLO/MSTR/CBLD root frame. The HMC
    /// admission step resolves each leaf's content digest and stable identity.
    pub(crate) fn seal_leaf_assembly_10d(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["assembly_frame", "leaves"])?;
        let values = list(required(args, "leaves")?)?;
        let max_leaves = (ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS - 4) / 2;
        if values.is_empty() || values.len() > max_leaves {
            return Err(format!("Assembly needs 1..={max_leaves} ordered leaves"));
        }
        let leaves = values.iter().map(parse_leaf).collect::<Result<Vec<_>, _>>()?;
        self.admit_leaf_assembly(required(args, "assembly_frame")?, &leaves)
    }

    fn admit_leaf_assembly(&mut self, frame: &Value, leaves: &[Leaf<'_>]) -> Result<Value, String> {
        let frame = encoded_resource(self, frame)?;
        let limit = self.limits.resource_bytes.min(RING_BYTES);
        if frame.kind != "10d" || frame.bytes.len() > limit {
            return Err("Assembly frame must be a .10d resource within the ring budget".into());
        }
        let sealed = seal_assembly(&frame.bytes, leaves, limit)?;
        self.charge(frame.bytes.len().saturating_add(sealed.len()), sealed.len())?;
        let scene = SceneGraph10d::from_fields(TenDFields::parse(Arc::from(sealed.as_slice()))?)?;
        let scene = self.admit_artifact(TypedArtifact::SceneGraph10d(Arc::new(scene)))?;
        let assembly = self.admit_encoded("10d", sealed)?;
        Ok(Value::Record(BTreeMap::from([
            ("scene".into(), scene),
            ("assembly".into(), assembly),
        ])))
    }
}

fn parse_hex<const N: usize>(value: &Value) -> Result<[u8; N], String> {
    let bytes = text(value)?.as_bytes();
    if bytes.len() != N * 2 || !bytes.iter().all(u8::is_ascii_hexdigit) {
        return Err(format!("Expected a {N}-byte hexadecimal identity"));
    }
    let mut result = [0; N];
    for (dst, pair) in result.iter_mut().zip(bytes.chunks_exact(2)) {
        let low = (pair[1] as char).to_digit(16).unwrap() as u8;
        let high = (pair[0] as char).to_digit(16).unwrap() as u8;
        *dst = (high << 4) | low;
    }
    Ok(result)
}

fn parse_leaf(value: &Value) -> Result<Leaf<'_>, String> {
    let row = record(value)?;
    fields(row, &[
        "leaf_key", "leaf_iri", "entity_iri", "taxon_or_class_iri", "sha256",
        "archetype_entity_id", "local_pga_motor", "age_days", "uniform_scale", "ssm_coefficients",
    ])?;
    parse_placement(
        row, text(required(row, "leaf_key")?)?, text(required(row, "leaf_iri")?)?,
        text(required(row, "taxon_or_class_iri")?)?, parse_hex(required(row, "sha256")?)?,
        parse_hex(required(row, "archetype_entity_id")?)?,
    )
}

fn parse_placement<'a>(
    row: &'a Args, key: &'a str, iri: &'a str, class_iri: &'a str,
    digest: [u8; 32], archetype_id: [u8; 16],
) -> Result<Leaf<'a>, String> {
    if key.is_empty() || key.len() > MAX_LEAF_IRI_BYTES || key.chars().any(char::is_control) {
        return Err("Invalid HMC leaf key".into());
    }
    let [s, e23, e31, e12, e01, e02, e03, e0123] = vector(required(row, "local_pga_motor")?)?;
    let motor = [s, e23, e31, e12, e01, e02, e03, e0123].map(|v| v as f32);
    let [s, e23, e31, e12, e01, e02, e03, e0123] = motor.map(f64::from);
    if !valid_motor(&Motor {
        r_w: s, r_x: e23, r_y: e31, r_z: e12,
        d_w: e0123, d_x: -e01, d_y: -e02, d_z: -e03,
    }) {
        return Err("Local PGA motor must be finite, unit-norm and Plücker-orthogonal".into());
    }
    let age_days = number(row, "age_days", 0.0)? as f32;
    let scale = number(row, "uniform_scale", 1.0)? as f32;
    if !age_days.is_finite() || age_days < 0.0 || !scale.is_finite() || scale <= 0.0 {
        return Err("Invalid assembly instance age or scale".into());
    }
    let mut coefficients = Vec::new();
    if let Some(value) = row.get("ssm_coefficients") {
        let weights = list(value)?;
        if weights.len() > ten::MAX_INSTANCE_SSM_COEFFS {
            return Err("Assembly leaf SSM coefficient limit exceeded".into());
        }
        for weight in weights {
            let weight = record(weight)?;
            fields(weight, &["mode_id", "sigma_weight"])?;
            let id = number(weight, "mode_id", -1.0)?;
            let sigma_weight = number(weight, "sigma_weight", f64::NAN)? as f32;
            if !(0.0..=f64::from(u32::MAX)).contains(&id) || id.fract() != 0.0
                || !sigma_weight.is_finite()
            {
                return Err("Invalid assembly leaf SSM coefficient".into());
            }
            coefficients.push(ten::SsmInstanceCoeff { mode_id: id as u32, sigma_weight });
        }
        coefficients.sort_unstable_by_key(|weight| weight.mode_id);
        if coefficients.windows(2).any(|pair| pair[0].mode_id == pair[1].mode_id) {
            return Err("Duplicate assembly leaf SSM mode".into());
        }
    }
    Ok(Leaf {
        key,
        iri,
        entity_iri: text(required(row, "entity_iri")?)?,
        class_iri,
        digest,
        archetype_id,
        motor,
        age_days,
        scale,
        coefficients,
    })
}

fn append_field(
    sidecar: &mut Vec<u8>, records: &mut Vec<ten::TypedFieldRecord>, kind: u16, bytes: &[u8],
) {
    let offset = sidecar.len();
    sidecar.extend_from_slice(bytes);
    records.push(ten::TypedFieldRecord {
        field_id: records.len() as u64 + 1,
        kind,
        payload_section_id: ten::SectionType::FieldSidecar as u32,
        payload_offset: offset as u64,
        payload_length: bytes.len() as u64,
        ..Default::default()
    });
}

fn required_payload<'a>(view: &ten::TypedPayloadResolver<'a, '_>, kind: u16) -> Result<ten::TypedPayload<'a>, String> {
    let mut found = view.payloads_of_kind(kind);
    let payload = found.next().ok_or_else(|| format!("Missing frame field {kind:#06x}"))?
        .map_err(|error| error.to_string())?;
    if found.next().is_some() {
        return Err(format!("Ambiguous frame field {kind:#06x}"));
    }
    Ok(payload)
}

fn preserve_frame_field(
    sidecar: &mut Vec<u8>, fields: &mut Vec<ten::TypedFieldRecord>,
    mut record: ten::TypedFieldRecord, bytes: &[u8],
) {
    record.payload_section_id = ten::SectionType::FieldSidecar as u32;
    record.payload_offset = sidecar.len() as u64;
    record.payload_length = bytes.len() as u64;
    sidecar.extend_from_slice(bytes);
    fields.push(record);
}

fn leaf_capacity(cbld_count: usize) -> usize {
    // HOLO, MSTR and INST each need one field. AREF and IOVR need two per leaf.
    ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS.saturating_sub(cbld_count + 3) / 2
}

fn lexical(terms: &mut BTreeMap<u64, String>, iri: &str) -> Result<u64, String> {
    if iri.len() > MAX_LEAF_IRI_BYTES {
        return Err("Assembly IRI exceeds bounded lexical storage".into());
    }
    resource_identity_meta(iri).map_err(|error| error.to_string())?;
    let hash = q_hash(iri);
    if terms.get(&hash).is_some_and(|prior| prior != iri) {
        return Err(format!("Conflicting assembly lexicon binding {hash:#018x}"));
    }
    terms.entry(hash).or_insert_with(|| iri.into());
    Ok(hash)
}

fn seal_assembly(frame: &[u8], leaves: &[Leaf<'_>], limit: usize) -> Result<Vec<u8>, String> {
    let header = ten::Container10dHeader::parse(frame).map_err(|error| error.to_string())?;
    if header.header_crc32c != ten::compute_whole_file_crc32c(frame) {
        return Err("Assembly frame CRC mismatch".into());
    }
    let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut records = scratch;
    let view = ten::TypedPayloadResolver::new(frame, &[], &mut scratch, &mut records)
        .map_err(|error| error.to_string())?;
    let mut identity = *view.identity().ok_or("Missing assembly frame identity")?;
    identity.external_digest = None;
    let holo = required_payload(&view, ten::FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE)?;
    let mstr = required_payload(&view, ten::FIELD_KIND_MULTISCALE_STRATA)?;
    if !mstr.bytes.starts_with(&ten::MULTISCALE_STRATA_TAG) {
        return Err("Invalid assembly MSTR signature".into());
    }
    let holo_header_bytes = holo.bytes.get(..ten::HOLOGRAPHIC_PHASE_HEADER_SIZE)
        .ok_or("Truncated assembly HOLO")?;
    let holo_header: ten::HolographicPhaseHeader = bytemuck::pod_read_unaligned(holo_header_bytes);
    let mut segments = vec![<qualia_core_db::q42::m64_matter::M64HoloRecord as bytemuck::Zeroable>::zeroed(); usize::from(holo_header.holo_segment_count)
        .min(ten::MAX_HOLO_SEGMENTS)];
    let mut wavefields = vec![<ten::EulerianWavefieldRecord as bytemuck::Zeroable>::zeroed(); usize::from(holo_header.wavefield_count)
        .min(ten::MAX_EULERIAN_WAVEFIELDS)];
    let (holo_header, _, _) = ten::decode_holographic_phase_section_into(holo.bytes, &mut segments, &mut wavefields)
        .map_err(|error| format!("Assembly HOLO: {error:?}"))?;
    let root = holo_header.nquin_subject_hash;
    let mut terms = BTreeMap::new();
    let mut cbl_base = None;
    let mut cbl_preserved = Vec::new();
    for field in view.payloads_of_kind(ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        let field = field.map_err(|error| error.to_string())?;
        let payload = ten::decode_symbolic_rdf_cbor_ld(field.bytes)
            .map_err(|error| format!("Assembly CBLD: {error:?}"))?;
        for term in &payload.lexicon {
            if lexical(&mut terms, &term.iri_utf8)? != term.term_hash {
                return Err(format!("Inconsistent assembly CBLD key {:#018x}", term.term_hash));
            }
        }
        if cbl_base.is_none()
            && payload.context_iri != qualia_core_db::bundle::semantic_model::HOLO_ADDRESS_CONTEXT
            && payload.lexicon.iter().any(|term| term.term_hash == root)
        {
            cbl_base = Some((field.record, payload));
        } else {
            cbl_preserved.push(field);
        }
    }
    let (cbl_record, mut cbl) = cbl_base.ok_or("Assembly HOLO root is absent from authored CBLD")?;
    let preserved_fields = cbl_preserved.len() + 3;
    if leaves.len() > leaf_capacity(cbl_preserved.len() + 1) {
        return Err("Assembly placements and frame metadata exceed the MID2 field limit".into());
    }
    let declared: BTreeSet<_> = terms.keys().copied().collect();
    let context = lexical(&mut terms, &cbl.context_iri)?;
    let part_of = lexical(&mut terms, PART_OF)?;
    let rdf_type = lexical(&mut terms, RDF_TYPE)?;
    let archetype = lexical(&mut terms, ARCHETYPE)?;
    let mut leaf_ids = BTreeSet::new();
    let mut selectors = BTreeMap::new();
    let mut instances = Vec::with_capacity(leaves.len());
    let mut sidecar = Vec::new();
    let mut fields = Vec::with_capacity(leaves.len() * 2 + preserved_fields + 1);
    preserve_frame_field(&mut sidecar, &mut fields, holo.record, holo.bytes);
    preserve_frame_field(&mut sidecar, &mut fields, mstr.record, mstr.bytes);
    let mut last_id = cbl_preserved.iter().map(|field| field.record.field_id)
        .chain(fields.iter().map(|field| field.field_id))
        .fold(cbl_record.field_id, u64::max);
    let new_fields_start = fields.len();
    let mut iovr = [0; ten::INSTANCE_OVERRIDE_HEADER_SIZE + ten::MAX_INSTANCE_SSM_COEFFS * ten::SSM_INSTANCE_COEFF_SIZE];
    for (slot, leaf) in leaves.iter().enumerate() {
        let entity = lexical(&mut terms, leaf.entity_iri)?;
        let class = lexical(&mut terms, leaf.class_iri)?;
        let iri = lexical(&mut terms, leaf.iri)?;
        let selector_iri = hmc_entry_key_iri(leaf.key);
        let selector = lexical(&mut terms, &selector_iri)?;
        if entity == root || !leaf_ids.insert(entity) {
            return Err(format!("Duplicate assembly instance identity at slot {slot}"));
        }
        if selector == 0 || selectors.insert(selector, (leaf.key, leaf.digest, leaf.archetype_id))
            .is_some_and(|previous| previous != (leaf.key, leaf.digest, leaf.archetype_id))
        {
            return Err(format!("Ambiguous HMC leaf selector at slot {slot}"));
        }
        let entry_key = lexical(&mut terms, ENTRY_KEY)?;
        for (s, p, o) in [(entity, part_of, root), (entity, rdf_type, class),
            (entity, archetype, iri), (entity, entry_key, selector)] {
            cbl.push_nquin(&NQuin {
                subject: s, predicate: p, object: o, context, metadata: 0,
                parity: NQuin::calculate_parity(s, p, o, context, 0),
            });
        }
        let reference = ten::ArchetypeRefRecord::new(
            ten::ArchetypeDigestAlg::Sha256, ten::ArchetypeTargetKind::HmcBundleEntry, 0,
            leaf.digest, leaf.archetype_id, class, selector,
        );
        let mut aref = [0; ten::ARCHETYPE_REF_RECORD_SIZE];
        ten::encode_archetype_ref_record(&reference, &mut aref).map_err(|error| error.to_string())?;
        let len = ten::encode_instance_overrides(leaf.age_days, leaf.scale, leaf.motor, &leaf.coefficients, &mut iovr)
            .map_err(|error| error.to_string())?;
        append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_ARCHETYPE_REF, &aref);
        append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_INSTANCE_OVERRIDES, &iovr[..len]);
        instances.push(ten::InstanceBatchRecord {
            entity_id_hash: entity,
            proc_seed: entity,
            age_days: leaf.age_days,
            uniform_scale: leaf.scale,
            pga_rotor: [leaf.motor[0], leaf.motor[1], leaf.motor[2], leaf.motor[3]],
            ..Default::default()
        });
    }
    let mut inst = vec![0; ten::instance_batch_encoded_len(instances.len()).ok_or("Assembly INST limit exceeded")?];
    ten::encode_instance_batch(&instances, &mut inst).map_err(|error| error.to_string())?;
    append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_INSTANCE_BATCH, &inst);
    cbl.lexicon.extend(terms.into_iter().filter(|(hash, _)| !declared.contains(hash)).map(|(term_hash, iri_utf8)| {
        ten::MultilingualLexicalTerm { term_hash, iri_utf8, lang_bcp47: None, label_utf8: String::new() }
    }));
    cbl.nquins.sort_unstable();
    cbl.nquins.dedup();
    let cbl = ten::encode_symbolic_rdf_cbor_ld(&cbl).map_err(|error| format!("Assembly CBLD: {error:?}"))?;
    for field in &mut fields[new_fields_start..] {
        last_id = last_id.checked_add(1).ok_or("Assembly frame field IDs are exhausted")?;
        field.field_id = last_id;
    }
    for field in cbl_preserved {
        preserve_frame_field(&mut sidecar, &mut fields, field.record, field.bytes);
    }
    preserve_frame_field(&mut sidecar, &mut fields, cbl_record, &cbl);
    fields.sort_unstable_by_key(|field| field.field_id);
    let section = blob(ten::SectionType::FieldSidecar, &sidecar);
    let mut sections = vec![section];
    for kind in [ten::SectionType::ProvenanceSidecar, ten::SectionType::EntityOntology] {
        if let Some(payload) = view.section_payload(kind) {
            sections.push(blob(kind, payload));
        }
    }
    let mut manifest = [0; ten::MANIFOLD_IDENTITY_V2_MAX_BYTES];
    let capacity = sections.iter().try_fold(manifest.len() + 4096, |sum, section| {
        sum.checked_add(section.payload.len())?.checked_add(4096)
    }).ok_or("Assembly length overflow")?.min(limit);
    let mut out = vec![0; capacity];
    let mut section_scratch = [section; 4];
    let length = ten::encode_container_with_manifold_identity(
        &header, &sections, &identity, &fields, &mut manifest, &mut section_scratch, &mut out,
    ).map_err(|error| format!("Assembly .10d envelope: {error:?}"))?;
    out.truncate(length);
    ten::seal_whole_file_crc32c(&mut out);
    SemanticModelBuilder::new(limit).merge_10d(&out)
        .map_err(|error| format!("Assembly semantic bindings: {error}"))?;
    Ok(out)
}

fn blob(section_type: ten::SectionType, payload: &[u8]) -> ten::SectionInput<'_> {
    ten::SectionInput { section_type, alignment_tier: ten::AlignmentTier::Page, stride: 0, element_count: 0, payload }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    fn frame() -> Vec<u8> {
        let root_iri = "urn:test:assembly:root";
        let root = q_hash(root_iri);
        let header = ten::HolographicPhaseHeader {
            max_raymarch_steps: 16, max_harmonic_order: 2, nquin_subject_hash: root,
            proxy_aabb_min: [-1.0; 3], proxy_aabb_max: [1.0; 3],
            ..ten::HolographicPhaseHeader::zeroed()
        };
        let mut holo = [0; ten::HOLOGRAPHIC_PHASE_HEADER_SIZE];
        ten::encode_holographic_phase_section(header, &[], &[], &mut holo).unwrap();
        let mut node = ten::NestedStratumNodeRecord::zeroed();
        node.host_anchor.entity_hash = root;
        node.parent_node_index = ten::STRATUM_NO_PARENT;
        node.taxon_or_ontology_iri_hash = root;
        let mut mstr = [0; ten::MULTISCALE_STRATA_HEADER_SIZE + ten::NESTED_STRATUM_NODE_SIZE];
        ten::encode_multiscale_strata_section(&[node], 0, &mut mstr).unwrap();
        let cbl = ten::encode_symbolic_rdf_cbor_ld(&ten::SymbolicRdfCborLdPayload {
            context_iri: "urn:test:assembly:context".into(),
            lexicon: vec![ten::MultilingualLexicalTerm {
                term_hash: root, iri_utf8: root_iri.into(), lang_bcp47: None, label_utf8: String::new(),
            }],
            ..Default::default()
        }).unwrap();
        let mut sidecar = Vec::new();
        let mut fields = Vec::new();
        append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE, &holo);
        append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_MULTISCALE_STRATA, &mstr);
        append_field(&mut sidecar, &mut fields, ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD, &cbl);
        let identity = ten::ManifoldIdentityV2 {
            entity_id: ten::StableEntityId([9; 16]), profile_id: 1, domain_id: 2,
            coordinate_convention_id: 3, coordinates: [0.0; 10], external_digest: None,
        };
        let section = blob(ten::SectionType::FieldSidecar, &sidecar);
        let mut manifest = [0; ten::MANIFOLD_IDENTITY_V2_MAX_BYTES];
        let mut out = vec![0; 16 * 1024];
        let len = ten::encode_container_with_manifold_identity(
            &ten::Container10dHeader::proposed(), &[section], &identity, &fields,
            &mut manifest, &mut [section; 2], &mut out,
        ).unwrap();
        out.truncate(len);
        ten::seal_whole_file_crc32c(&mut out);
        out
    }

    #[test]
    fn preserves_authored_order_and_lexical_identity_without_leaf_geometry() {
        let make_leaf = |key, iri, id, scale| Leaf {
            key, iri, entity_iri: iri, class_iri: "urn:test:assembly:class",
            digest: [id; 32], archetype_id: [id; 16], motor: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            age_days: 0.0, scale, coefficients: vec![],
        };
        let leaves = [
            make_leaf("z.10d", "urn:test:assembly:z", 1, 1.25),
            make_leaf("a.10d", "urn:test:assembly:a", 2, 1.5),
        ];
        let sealed = seal_assembly(&frame(), &leaves, 64 * 1024).unwrap();
        let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
        let mut fields = scratch;
        let view = ten::TypedPayloadResolver::new(&sealed, &[], &mut scratch, &mut fields).unwrap();
        assert!(view.section_payload(ten::SectionType::QuantizedMesh).is_none());
        assert_eq!(view.instance_override_count().unwrap(), 2);
        let cbld = ten::decode_symbolic_rdf_cbor_ld(required_payload(&view, ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD).unwrap().bytes).unwrap();
        let mut records = [ten::InstanceBatchRecord::default(); 2];
        let inst = required_payload(&view, ten::FIELD_KIND_INSTANCE_BATCH).unwrap().bytes;
        assert_eq!(ten::decode_instance_batch_into(inst, &mut records).unwrap().1, 2);
        for (slot, leaf) in leaves.iter().enumerate() {
            let override_record = view.instance_override_into(slot, &mut []).unwrap().unwrap();
            assert_eq!(override_record.header.uniform_scale, leaf.scale);
            assert_eq!(records[slot].entity_id_hash, q_hash(leaf.entity_iri));
            assert!(cbld.lexicon.iter().any(|entry| entry.iri_utf8 == leaf.entity_iri));
            assert_eq!(override_record.identity, ten::InstanceOverrideIdentity::Leaf {
                archetype_entity_id: ten::StableEntityId(leaf.archetype_id),
                taxon_or_class_iri_hash: q_hash(leaf.class_iri),
                subresource_key_hash: q_hash(&hmc_entry_key_iri(leaf.key)),
            });
        }
    }
}

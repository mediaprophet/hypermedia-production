//! Stable cardiac rig terms, source provenance and aspect graph.

use qualia_core_db::{
    container_10d::{
        decode_manifold_identity_v2, encode_manifold_identity_v2, encode_symbolic_rdf_cbor_ld,
        header::Container10dHeader, integrity::seal_whole_file_crc32c,
        manifold_identity_v2_encoded_len, MultilingualLexicalTerm, SymbolicRdfCborLdPayload,
        provenance_section::{encode_provenance_section, encoded_len, ProvenanceSidecar},
        section::{encode_container, parse_section_table, SectionInput, SectionTableError, SectionType},
        skeletal_container::read_skeletal_section, TypedFieldRecord, FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        MANIFOLD_IDENTITY_V2_MAX_FIELDS,
    },
    hypermedia::{container_to_nquins, content_digest, AssetRef, AssetRole, HypermediaContainer},
    q42_volume::UnifiedVolumeBuilder,
    q_hash, NQuin, QUINS_PER_BLOCK,
};

const JOINTS: [&str; 6] = [
    "urn:etalon:reconstruct:joint:base", "urn:etalon:reconstruct:joint:apex",
    "urn:etalon:reconstruct:joint:lv", "urn:etalon:reconstruct:joint:rv",
    "urn:etalon:reconstruct:joint:la", "urn:etalon:reconstruct:joint:ra",
];
const CLIP: &str = "urn:etalon:reconstruct:clip:beat";
const CONTEXT: &str = "https://qualiadb.org/ontology/hmc#CardiacRig";
const HAS_JOINT: &str = "https://qualiadb.org/ontology/hmc#hasRigJoint";
const HAS_CLIP: &str = "https://qualiadb.org/ontology/hmc#hasRigClip";

fn cardiac_dictionary(bytes: &[u8], iri: &str) -> Result<Vec<u8>, String> {
    let rig = read_skeletal_section(bytes).map_err(|error| format!("Cardiac SKL1 admission: {error:?}"))?;
    let counts = rig.counts();
    if counts.joints != JOINTS.len() || counts.keys != 42 || counts.clips != 1
        || rig.clip(0).is_none_or(|clip| !clip.looping || clip.id != q_hash(CLIP))
        || JOINTS.iter().enumerate().any(|(index, joint)| rig.joint_id(index) != Some(q_hash(joint)))
    {
        return Err("Native cardiac SKL1 clip or its six joint identities disagree with the authored dictionary".into());
    }
    let lexicon = [iri, CONTEXT, HAS_JOINT, HAS_CLIP, CLIP].into_iter().chain(JOINTS)
        .map(|term| MultilingualLexicalTerm {
            term_hash: q_hash(term), iri_utf8: term.into(), lang_bcp47: None, label_utf8: String::new(),
        }).collect();
    let subject = q_hash(iri);
    let context = q_hash(CONTEXT);
    let nquins = JOINTS.into_iter().map(|joint| (HAS_JOINT, joint)).chain([(HAS_CLIP, CLIP)])
        .map(|(predicate, object)| {
            let (p, o) = (q_hash(predicate), q_hash(object));
            [subject, p, o, context, 0, NQuin::calculate_parity(subject, p, o, context, 0)]
        }).collect();
    encode_symbolic_rdf_cbor_ld(&SymbolicRdfCborLdPayload {
        context_iri: CONTEXT.into(), lexicon, nquins, ..Default::default()
    }).map_err(|error| error.to_string())
}

pub(super) fn attach_cardiac_dictionary(
    bytes: &[u8], iri: &str, provenance: &ProvenanceSidecar, budget: usize,
) -> Result<Vec<u8>, String> {
    let header = Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    let table = parse_section_table(bytes, &header).map_err(|error| error.to_string())?;
    let payload = |kind| -> Result<&[u8], String> {
        let row = table.iter().find(|row| row.typ() == Some(kind))
            .ok_or("Missing native cardiac section")?;
        bytes.get(row.byte_offset as usize..(u64::from(row.byte_offset) + u64::from(row.byte_length)) as usize)
            .ok_or_else(|| "Cardiac section lies outside container".into())
    };
    let dictionary = cardiac_dictionary(bytes, iri)?;
    let mut sidecar = payload(SectionType::FieldSidecar)?.to_vec();
    let offset = u64::try_from(sidecar.len()).map_err(|_| "Cardiac CBLD offset exceeds u64")?;
    sidecar.extend_from_slice(&dictionary);
    let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let (identity, count) = decode_manifold_identity_v2(payload(SectionType::ManifoldIdentityV2)?,
        &[], &mut fields).map_err(|error| format!("Cardiac MID2 decode: {error:?}"))?;
    let id = fields[..count].last().map_or(0, |field| field.field_id).checked_add(1)
        .ok_or("Cardiac CBLD ID exceeds u64")?;
    let field = fields.get_mut(count).ok_or("Cardiac MID2 has no free field slot")?;
    *field = TypedFieldRecord {
        field_id: id, kind: FIELD_KIND_SYMBOLIC_RDF_CBOR_LD, flags: 0, unit_id: 0,
        payload_section_id: SectionType::FieldSidecar as u32, payload_offset: offset,
        payload_length: dictionary.len() as u64,
    };
    let mut mid2 = vec![0; manifold_identity_v2_encoded_len(count + 1)
        .map_err(|error| format!("Cardiac MID2 length: {error:?}"))?];
    encode_manifold_identity_v2(&identity, &fields[..count + 1], &mut mid2)
        .map_err(|error| format!("Cardiac MID2 encode: {error:?}"))?;
    let mut prv1 = vec![0; encoded_len(provenance)];
    encode_provenance_section(provenance, &mut prv1).map_err(|error| error.to_string())?;
    let sections = table.iter().map(|row| {
        let section_type = row.typ().ok_or("Unknown cardiac section type")?;
        let payload = match section_type {
            SectionType::FieldSidecar => sidecar.as_slice(),
            SectionType::ManifoldIdentityV2 => mid2.as_slice(),
            SectionType::ProvenanceSidecar => prv1.as_slice(),
            _ => payload(section_type)?,
        };
        Ok(SectionInput {
            section_type, payload, stride: row.stride, element_count: row.element_count,
            alignment_tier: row.tier().ok_or("Unknown cardiac section alignment")?,
        })
    }).collect::<Result<Vec<_>, String>>()?;
    let needed = match encode_container(&header, &sections, &mut []) {
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) | Ok(needed) => needed,
        Err(error) => return Err(error.to_string()),
    };
    if needed > budget {
        return Err("Cardiac 10d exceeds its sealed graphics ring budget".into());
    }
    let mut output = vec![0; needed];
    let len = encode_container(&header, &sections, &mut output).map_err(|error| error.to_string())?;
    output.truncate(len);
    seal_whole_file_crc32c(&mut output);
    Ok(output)
}

pub(super) fn aspects(
    iri: &str, source_iri: &str, licence: &str, geometry: &[u8], m64: &[u8], emf: &[u8],
) -> Result<Vec<u8>, String> {
    let primary = AssetRef::new(iri, content_digest(geometry), "model/vnd.qualia.10d", AssetRole::Primary)
        .derived_from(source_iri).with_licence(licence);
    let container_iri = format!("{iri}#cardiac-package");
    let container = HypermediaContainer::new(container_iri, primary)
        .with_related(AssetRef::new(format!("{iri}#matter"), content_digest(m64),
            "application/vnd.qualia.m64", AssetRole::Derivation).derived_from(iri))
        .with_related(AssetRef::new(format!("{iri}#electromagnetic"), content_digest(emf),
            "application/vnd.qualia.emf", AssetRole::Derivation).derived_from(iri));
    let (mut quins, mut lex) = container_to_nquins(&container);
    for term in [
        "urn:qualia:context:hypermedia", "urn:qualia:hypermedia:Container",
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#type", "urn:qualia:hypermedia:hasPrimary",
        "urn:qualia:hypermedia:bundledWith", "urn:qualia:hypermedia:hasRole",
        "urn:qualia:hypermedia:role:primary", "urn:qualia:hypermedia:role:derivation",
        "urn:qualia:hypermedia:mediaType", "urn:qualia:hypermedia:digest",
        "http://www.w3.org/ns/prov#wasDerivedFrom", "http://purl.org/dc/terms/license",
    ] {
        if lex.insert(q_hash(term), term.into()).is_some_and(|old| old != term) {
            return Err("Cardiac aspect graph contains a lexical collision".into());
        }
    }
    for quin in &mut quins {
        quin.recalculate_parity();
    }
    quins.sort_unstable_by_key(|row| (row.object, row.subject, row.predicate, row.context, row.metadata));
    let mut model = UnifiedVolumeBuilder::with_lex_map(&lex)
        .map_err(|error| error.to_string())?.with_fixed_timestamp_ms(0);
    for (index, block) in quins.chunks(QUINS_PER_BLOCK).enumerate() {
        model.push_block(index as u64, block).map_err(|error| error.to_string())?;
    }
    Ok(model.finish_to_bytes())
}

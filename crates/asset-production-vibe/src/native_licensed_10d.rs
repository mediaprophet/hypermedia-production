//! Digest-bound source licensing for a preserved 10D envelope or static mesh.

use std::{collections::BTreeMap, sync::atomic::Ordering};

use qualia_core_db::{
    NQuin,
    bundle::semantic_model::{holo_segment_iri, SemanticModelBuilder},
    container_10d::{
        self as ten, AlignmentTier, Container10dHeader, FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE,
        FIELD_KIND_SYMBOLIC_RDF_CBOR_LD, MANIFOLD_IDENTITY_V2_MAX_FIELDS, MultilingualLexicalTerm,
        SectionInput, SectionTableError, SectionType, SymbolicRdfCborLdPayload, TypedFieldRecord,
        TypedPayloadResolver,
        crc32c::crc32c,
        decode_symbolic_rdf_cbor_ld, encode_manifold_identity_v2, encode_symbolic_rdf_cbor_ld,
        integrity::{check_whole_file_crc32c, seal_whole_file_crc32c},
        manifold_identity_v2_encoded_len,
        provenance_section::{
            self as prv, ProvenanceSidecar, decode_provenance_section, validate_provenance,
        },
    },
    modalities::logic::shacl::{CompiledShape, ShaclConstraint, ShaclEngine, ShaclSeverity},
    q_hash,
    render::compile_10d::{
        SourceLicenseTier, compile_organ_asset, upgrade_legacy_10d_to_holographic_v2,
    },
    resolver::pack_typed_literal,
};
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Resource, Value, fields, list, num, record, required, text};

const RING: usize = 128 * 1024 * 1024;
const SENTINEL: usize = 42 * 1024 * 1024;
const LICENCE_CONTEXT: &str = "urn:qualia:10d:source-licence:v1";
const LICENCE_SHAPE: &str =
    "https://ns.webcivics.net/source-asset-licensing/SourceAssetProvenanceShape";
const LICENCE_ONTOLOGY: &[u8] =
    include_bytes!("../../../../qualiaDB/core-ontologies/source-asset-licensing.n3");
const XSD_INT: &str = "http://www.w3.org/2001/XMLSchema#integer";
const XSD_BOOL: &str = "http://www.w3.org/2001/XMLSchema#boolean";
const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
const ATTRIBUTED_TO: &str = "https://ns.webcivics.net/permissive-commons/attributedTo";
const WAS_DERIVED_FROM: &str = "http://www.w3.org/ns/prov#wasDerivedFrom";

macro_rules! source_lic {
    ($name:literal) => {
        concat!("https://ns.webcivics.net/source-asset-licensing/", $name)
    };
}

struct License<'a> {
    spdx: &'a str,
    creator: &'a str,
    notice: &'a str,
    origin: &'a str,
    format: &'a str,
    modifications: &'a str,
    commercial: bool,
    share_alike: bool,
    timestamp: u64,
    embed_source: bool,
}

impl AssetHost<'_> {
    pub(crate) fn seal_10d_with_cbld_licensing(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "source_sha256",
                "asset_10d",
                "asset_iri",
                "license",
                "coordinates_10d",
                "domain_id",
            ],
        )?;
        let source = required(args, "source")?;
        let source_size = retained(self, source)?.bytes.len();
        let prepared = args.get("asset_10d");
        let prepared_size = prepared
            .map(|arg| retained(self, arg).map(|r| r.bytes.len()))
            .transpose()?
            .unwrap_or(0);
        let limit = self.limits.resource_bytes.min(RING);
        let combined = source_size
            .checked_add(prepared_size)
            .ok_or("Licensing input size overflow")?;
        if source_size == 0 || source_size > limit || prepared_size > limit {
            return Err("Licensing sources exceed the admitted 10D ring or are empty".into());
        }
        self.charge(combined.saturating_mul(8), combined.saturating_mul(2))?;
        let licence = validate_licence(record(required(args, "license")?)?)?;
        let asset_iri = text(required(args, "asset_iri")?)?;
        if !valid_iri(asset_iri) || q_hash(asset_iri) == 0 {
            return Err("10D asset requires a canonical absolute RDF IRI".into());
        }
        let (asset, licence_model, manifest, summary) = {
            let source = retained(self, source)?;
            let digest = Sha256::digest(&source.bytes);
            let sha_hex = format!("{digest:x}");
            if text(required(args, "source_sha256")?)? != sha_hex {
                return Err("Original retained source bytes do not match source_sha256".into());
            }
            check_format(source, licence.format, prepared.is_some())?;
            let sha256: [u8; 32] = digest.into();
            let prepared = prepared.map(|value| retained(self, value)).transpose()?;
            if prepared.is_some_and(|payload| payload.kind != "10d") {
                return Err("asset_10d requires a retained or encoded 10D envelope".into());
            }
            let source_10d = prepared.or((licence.format == "10d").then_some(source));
            let prior_prv = source_10d
                .map(|asset| previous_provenance(&asset.bytes))
                .transpose()?
                .flatten();
            if let Some(prior) = prior_prv {
                validate_existing_provenance(prior, &source.bytes, &sha256, &licence)?;
            } else if prepared
                .is_some_and(|asset| asset.bytes.as_slice() != source.bytes.as_slice())
            {
                return Err(
                    "Prepared 10D requires an existing PRV1 bound to the supplied original bytes"
                        .into(),
                );
            }

            let carried = if licence.embed_source {
                if source_size > prv::MAX_PROVENANCE_FIELD {
                    return Err("Full original source exceeds the 16 MiB PRV1 field limit".into());
                }
                &source.bytes[..]
            } else {
                &sha256[..]
            };
            let crc = prior_prv.map_or_else(|| crc32c(carried), |prior| prior.source_digest());
            let (mut license_graph, source_iri) = licensing_graph(
                asset_iri,
                &licence,
                &sha_hex,
                source_size,
                crc,
                crc32c(&source.bytes),
            )?;
            validate_licensing_graph(&license_graph, &source_iri, crc)?;
            let cbld =
                encode_symbolic_rdf_cbor_ld(&license_graph).map_err(|error| error.to_string())?;
            let graft = if prior_prv.is_some() {
                None
            } else {
                Some(
                    ProvenanceSidecar::new(
                        carried.to_vec(),
                        media_type(licence.format),
                        licence.spdx,
                    )
                    .with_metadata(cbld.clone(), licence.timestamp, sha256),
                )
            };
            let raw;
            let native = if let Some(source_10d) = source_10d {
                &source_10d.bytes[..]
            } else {
                let compiled = compile_organ_asset(
                    &source.bytes,
                    Some(licence.format),
                    asset_iri,
                    licence.format,
                    None,
                    None,
                    graft.as_ref(),
                )
                .map_err(|error| format!("Native 10D source import: {error:?}"))?;
                raw = compiled.container_10d;
                &raw
            };
            let native_has_prv = previous_provenance(native)?.is_some();
            let encoded_prv = if native_has_prv {
                None
            } else {
                let prv = graft
                    .as_ref()
                    .ok_or("Missing validated PRV1 source provenance")?;
                let mut bytes = vec![0; prv::encoded_len(prv)];
                let len = prv::encode_provenance_section(prv, &mut bytes)
                    .map_err(|error| error.to_string())?;
                bytes.truncate(len);
                Some(bytes)
            };
            let staged;
            let working = if let Some(prv) = encoded_prv.as_deref() {
                staged = reseal(native, None, None, Some(prv), limit)?;
                &staged[..]
            } else {
                native
            };
            let geometry;
            let upgraded = if has_holo(working)? {
                if args.contains_key("coordinates_10d") || args.contains_key("domain_id") {
                    return Err(
                        "Existing HOLO geometry keeps its authored coordinates and domain".into(),
                    );
                }
                working
            } else {
                let coords = coordinates(required(args, "coordinates_10d")?)?;
                let domain = exact_u64(required(args, "domain_id")?)?;
                if domain == 0 {
                    return Err("10D legacy upgrade requires a nonzero declared domain".into());
                }
                geometry = upgrade_legacy_10d_to_holographic_v2(working, asset_iri, domain, coords)
                    .map_err(|error| format!("Native HOLO upgrade: {error:?}"))?;
                geometry.as_slice()
            };
            preserve_holo_addressability(upgraded, asset_iri, &mut license_graph)?;
            let local_licence =
                encode_symbolic_rdf_cbor_ld(&license_graph).map_err(|error| error.to_string())?;
            let asset = attach_licensing(upgraded, &local_licence, &license_graph, limit)?;
            let final_prv = previous_provenance(&asset)?.ok_or("Sealed 10D lost its PRV1")?;
            if final_prv.source_digest() != crc || final_prv.licence() != licence.spdx {
                return Err(
                    "Sealed 10D provenance disagrees with the validated source record".into(),
                );
            }
            let model = SemanticModelBuilder::new(limit.min(SENTINEL));
            let (model, info) = model.merge_10d(&asset).map_err(|error| error.to_string())?;
            let info = info.ok_or("Sealed 10D does not carry a valid HOLO semantic root")?;
            if info.root_entity_iri != asset_iri || info.stratum_mask == 0 {
                return Err(
                    "Sealed 10D requires its declared root and at least one authored MSTR level"
                        .into(),
                );
            }
            let compiled_crc = ten::integrity::compute_whole_file_crc32c(&asset);
            let receipt = format!(
                "<{asset_iri}> <{}> \"{compiled_crc}\"^^<{XSD_INT}> .\n\
                <{source_iri}> <{}> \"{sha_hex}\"^^<http://www.w3.org/2001/XMLSchema#hexBinary> .",
                source_lic!("compiled10dCrc32c"),
                source_lic!("sourceSha256")
            );
            let (q42, _) = model
                .seal_solid_turtle(LICENCE_CONTEXT, &receipt, self.cancel)
                .map_err(|error| error.to_string())?;
            let asset_sha256 = format!("{:x}", Sha256::digest(&asset));
            let manifest = make_manifest(asset_iri, &sha_hex, &asset_sha256, info.stratum_mask)?;
            let summary = BTreeMap::from([
                ("source_sha256".into(), Value::String(sha_hex)),
                ("asset_sha256".into(), Value::String(asset_sha256)),
                (
                    "strata_mask".into(),
                    Value::U64(u64::from(info.stratum_mask)),
                ),
                (
                    "licence_tier".into(),
                    Value::String(SourceLicenseTier::classify_spdx(licence.spdx).iri().into()),
                ),
                (
                    "preserved_source_provenance".into(),
                    Value::Bool(prior_prv.is_some()),
                ),
            ]);
            (asset, q42, manifest, summary)
        };
        if self.cancel.load(Ordering::Acquire) {
            return Err("Production cancelled".into());
        }
        let mut result = summary;
        result.insert("asset".into(), self.encode("10d", asset)?);
        result.insert("license_q42".into(), self.encode("q42", licence_model)?);
        result.insert(
            "manifest_q42_yaml".into(),
            self.encode("yaml", manifest.into_bytes())?,
        );
        Ok(Value::Record(result))
    }
}

fn retained<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|resource| &resource.key == key)
            .ok_or_else(|| format!("Unknown retained 10D licensing resource {key}")),
        _ => host.workspace.encoded(value),
    }
}

fn exact_u64(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(u) => Ok(*u),
        Value::I64(i) => u64::try_from(*i).map_err(|_| "Expected a nonnegative 10D integer".into()),
        _ => Err("Expected an exact 10D integer".into()),
    }
}

fn exact_bool(args: &Args, key: &str) -> Result<bool, String> {
    match required(args, key)? {
        Value::Bool(value) => Ok(*value),
        _ => Err(format!("Source license field {key} requires a Boolean")),
    }
}

fn valid_iri(iri: &str) -> bool {
    let Some((scheme, value)) = iri.split_once(':') else {
        return false;
    };
    scheme
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphabetic)
        && scheme
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"+-.".contains(&byte))
        && !value.is_empty()
        && !value.chars().any(|ch| {
            ch.is_control()
                || ch.is_whitespace()
                || "<>\"{}|\\^".contains(ch)
                || ch == char::from(0x60)
        })
}

fn validate_licence(args: &Args) -> Result<License<'_>, String> {
    fields(
        args,
        &[
            "spdx_license",
            "creator_iri",
            "attribution",
            "source_uri",
            "origin_format",
            "commercial_allowed",
            "derivatives_allowed",
            "share_alike_required",
            "modifications_summary",
            "timestamp_epoch_s",
            "embed_full_source",
        ],
    )?;
    let word = |key| text(required(args, key)?);
    let spdx = word("spdx_license")?;
    let creator = word("creator_iri")?;
    let notice = word("attribution")?;
    let origin = word("source_uri")?;
    let format = word("origin_format")?;
    let modifications = word("modifications_summary")?;
    let timestamp = exact_u64(required(args, "timestamp_epoch_s")?)?;
    if !valid_iri(creator)
        || !valid_iri(origin)
        || creator.len() > 2048
        || origin.len() > 2048
        || notice.trim().chars().count() < 3
        || notice.len() > 4096
        || modifications.trim().is_empty()
        || modifications.len() > 4096
        || timestamp == 0
    {
        return Err("Source licensing requires canonical origin and creator IRIs, UTF-8 attribution, a modification summary, and a declared timestamp".into());
    }
    let (commercial, share_alike) = match spdx {
        "CC0-1.0" | "CC-BY-4.0" | "MIT" | "Apache-2.0" | "BSD-2-Clause" | "BSD-3-Clause" => {
            (true, false)
        }
        "CC-BY-SA-4.0" => (true, true),
        "CC-BY-NC-4.0" => (false, false),
        "CC-BY-NC-SA-4.0" => (false, true),
        _ if spdx.contains("-ND-") => {
            return Err("No-derivatives terms do not authorize this conversion".into());
        }
        _ => {
            return Err(
                "Source SPDX license requires an unsupported authorization or expression".into(),
            );
        }
    };
    if exact_bool(args, "commercial_allowed")? != commercial
        || !exact_bool(args, "derivatives_allowed")?
        || exact_bool(args, "share_alike_required")? != share_alike
    {
        return Err(
            "Declared source use permissions conflict with the single supported SPDX license"
                .into(),
        );
    }
    let embed_source = args
        .get("embed_full_source")
        .map(|value| match value {
            Value::Bool(value) => Ok(*value),
            _ => Err("embed_full_source requires a Boolean"),
        })
        .transpose()?
        .unwrap_or(false);
    Ok(License {
        spdx,
        creator,
        notice,
        origin,
        format,
        modifications,
        commercial,
        share_alike,
        timestamp,
        embed_source,
    })
}

fn media_type(format: &str) -> &'static str {
    match format {
        "obj" => "model/obj",
        "stl" => "model/stl",
        "glb" => "model/gltf-binary",
        _ => "application/x-qualia-10d",
    }
}

fn check_format(source: &Resource, format: &str, prepared: bool) -> Result<(), String> {
    if !matches!(format, "10d" | "obj" | "stl" | "glb") || source.kind != format {
        return Err("Source origin_format must match retained OBJ, STL, GLB, or 10D bytes".into());
    }
    if format == "glb" && !prepared {
        return Err("GLB licensing requires a prepared native asset_10d to preserve scene and material layers".into());
    }
    if format == "obj" {
        let text = std::str::from_utf8(&source.bytes).map_err(|error| error.to_string())?;
        if !prepared
            && text
                .lines()
                .any(|line| line.split_whitespace().next() == Some("mtllib"))
        {
            return Err(
                "OBJ with external material dependencies requires a prepared native 10D".into(),
            );
        }
    }
    Ok(())
}

fn coordinates(value: &Value) -> Result<[f32; 10], String> {
    let values = list(value)?;
    if values.len() != 10 {
        return Err("coordinates_10d requires ten authored coordinates".into());
    }
    let mut coords = [0f32; 10];
    for (out, value) in coords.iter_mut().zip(values) {
        *out = num(value)? as f32;
        if !out.is_finite() {
            return Err("10D coordinates must be finite in the native F32 format".into());
        }
    }
    Ok(coords)
}

fn previous_provenance(bytes: &[u8]) -> Result<Option<prv::ProvenanceSidecarView<'_>>, String> {
    check_whole_file_crc32c(bytes).map_err(|error| error.to_string())?;
    let header = Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    let entries = ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())?;
    let payload = entries
        .iter()
        .find(|entry| entry.typ() == Some(SectionType::ProvenanceSidecar))
        .map(|entry| section_bytes(bytes, entry.byte_offset, entry.byte_length))
        .transpose()?;
    payload
        .map(|payload| {
            let view = decode_provenance_section(payload).map_err(|error| error.to_string())?;
            validate_provenance(&view).map_err(|error| error.to_string())?;
            Ok(view)
        })
        .transpose()
}

fn validate_existing_provenance(
    view: prv::ProvenanceSidecarView<'_>,
    original: &[u8],
    sha: &[u8; 32],
    licence: &License<'_>,
) -> Result<(), String> {
    let stored = view.source_bytes();
    if stored != original && (stored != sha || view.version_hash() != sha) {
        return Err("Existing 10D PRV1 does not bind the supplied original source bytes".into());
    }
    if view.licence() != licence.spdx || view.source_media_type() != media_type(licence.format) {
        return Err(
            "Existing 10D PRV1 license or format conflicts with the requested source license"
                .into(),
        );
    }
    if licence.embed_source && stored != original {
        return Err(
            "Existing digest-only PRV1 cannot be replaced by a full-source receipt in this path"
                .into(),
        );
    }
    Ok(())
}

fn has_holo(bytes: &[u8]) -> Result<bool, String> {
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    let result = view
        .payloads_of_kind(FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE)
        .next()
        .transpose()
        .map(|payload| payload.is_some())
        .map_err(|error| error.to_string());
    result
}

fn licensing_graph(
    asset: &str,
    license: &License<'_>,
    sha: &str,
    source_len: usize,
    carried_crc: u32,
    source_crc: u32,
) -> Result<(SymbolicRdfCborLdPayload, String), String> {
    let origin = format!("urn:qualia:3d-source:sha256:{sha}");
    let digest = format!("urn:sha256:{sha}");
    let spdx = format!("https://spdx.org/licenses/{}", license.spdx);
    let format = format!("urn:qualia:3d-media-format:{}", license.format);
    let modification = format!("urn:sha256:{:x}", Sha256::digest(license.modifications));
    let policy_digest = format!("urn:sha256:{:x}", Sha256::digest(LICENCE_ONTOLOGY));
    if asset == origin || asset == digest {
        return Err(
            "10D asset IRI must distinguish the embodiment from its original-source identity"
                .into(),
        );
    }
    let tier = SourceLicenseTier::classify_spdx(license.spdx);
    let mut graph = SourceGraph {
        lex: BTreeMap::new(),
        data: SymbolicRdfCborLdPayload {
            context_iri: LICENCE_CONTEXT.into(),
            shacl_shape_iris_utf8: vec![LICENCE_SHAPE.into()],
            ..Default::default()
        },
    };
    graph.iri(&origin, Some(license.notice))?;
    graph.iri(&spdx, Some(license.spdx))?;
    graph.iri(&format, Some(media_type(license.format)))?;
    graph.iri(&modification, Some(license.modifications))?;
    for (subject, predicate, object) in [
        (&*origin, RDF_TYPE, source_lic!("SourceAssetRecord")),
        (asset, RDF_TYPE, source_lic!("ConvertedEmbodiment")),
        (asset, WAS_DERIVED_FROM, &origin),
        (asset, ATTRIBUTED_TO, license.creator),
        (&origin, source_lic!("originalCreator"), license.creator),
        (&origin, source_lic!("originUri"), license.origin),
        (&origin, source_lic!("licenseTier"), tier.iri()),
        (asset, source_lic!("licenseTier"), tier.iri()),
        (&origin, "http://purl.org/dc/terms/license", &spdx),
        (asset, "http://purl.org/dc/terms/license", &spdx),
        (&origin, "urn:qualia:10d:sourceByteDigest", &digest),
        (asset, "urn:qualia:10d:sourceLicensingOntologyDigest", &policy_digest),
    ] {
        let object = graph.iri(object, None)?;
        graph.fact(subject, predicate, object)?;
    }
    for (predicate, label) in [
        (source_lic!("attributionNotice"), license.notice),
        (source_lic!("spdxIdentifier"), license.spdx),
        (source_lic!("sourceFormat"), media_type(license.format)),
        ("urn:qualia:10d:modificationsSummary", license.modifications),
    ] {
        let literal = serde_json::to_string(label).map_err(|error| error.to_string())?;
        graph.fact(&origin, predicate, q_hash(&literal))?;
    }
    for (predicate, number) in [
        (source_lic!("sourceByteLength"), source_len as u64),
        (source_lic!("sourceCrc32c"), u64::from(carried_crc)),
        ("urn:qualia:10d:originalSourceCrc32c", u64::from(source_crc)),
        ("urn:qualia:10d:attestedAtUnix", license.timestamp),
    ] {
        graph.fact(
            &origin,
            predicate,
            pack_typed_literal(&number.to_string(), XSD_INT)
                .ok_or("Licensing integer cannot be represented as an exact native literal")?,
        )?;
    }
    for (predicate, value) in [
        (
            source_lic!("commercialUsePermittedWithoutSettlement"),
            license.commercial,
        ),
        (source_lic!("derivativesPermittedOutsideEcosystem"), true),
        (source_lic!("requiresAttributionPreservation"), true),
        ("urn:qualia:10d:shareAlikeRequired", license.share_alike),
    ] {
        let object = pack_typed_literal(if value { "true" } else { "false" }, XSD_BOOL)
            .ok_or("Licensing Boolean cannot be represented as an exact native literal")?;
        graph.fact(asset, predicate, object)?;
        graph.fact(&origin, predicate, object)?;
    }
    graph.data.lexicon = graph.lex.into_values().collect();
    Ok((graph.data, origin))
}

fn validate_licensing_graph(
    graph: &SymbolicRdfCborLdPayload, source: &str, crc: u32,
) -> Result<(), String> {
    // SourceAssetProvenanceShape defines these three cardinality and length
    // checks. The CRC additionally has to equal the byte-backed PRV1 value.
    let source = q_hash(source);
    let rows = graph.decode_nquins();
    let expected = pack_typed_literal(&crc.to_string(), XSD_INT).ok_or("Invalid native CRC binding")?;
    if rows.iter().filter(|row| row.subject == source
        && row.predicate == q_hash(source_lic!("sourceCrc32c"))).map(|row| row.object).ne([expected])
    {
        return Err("Source-asset licence graph CRC does not equal the native PRV1 commitment".into());
    }
    let mut lexical = BTreeMap::new();
    for term in &graph.lexicon {
        lexical.insert(term.term_hash, term.iri_utf8.clone());
        if !term.label_utf8.is_empty() {
            let label = serde_json::to_string(&term.label_utf8).map_err(|error| error.to_string())?;
            lexical.insert(q_hash(&label), term.label_utf8.clone());
        }
    }
    let shapes: Vec<_> = [
        (source_lic!("spdxIdentifier"), Some(2)),
        (source_lic!("sourceCrc32c"), None),
        (source_lic!("attributionNotice"), Some(3)),
    ].into_iter().map(|(path, min_length)| {
        let mut constraints = vec![ShaclConstraint::MinCount(1)];
        if let Some(length) = min_length {
            constraints.push(ShaclConstraint::MinLength(length));
        }
        let mut shape = CompiledShape::new(source_lic!("SourceAssetRecord").into(),
            constraints, ShaclSeverity::Violation);
        shape.property_path = path.into();
        shape
    }).collect();
    let engine = ShaclEngine::new(&rows, &shapes);
    let mut findings = Vec::new();
    for shape in &shapes {
        engine.validate_focus(source, shape, &|hash| lexical.get(&hash).cloned(), &mut findings);
    }
    if !findings.is_empty() {
        return Err(format!("Native source-asset licence SHACL rejected {} constraint(s)", findings.len()));
    }
    Ok(())
}

fn preserve_holo_addressability(
    bytes: &[u8], root: &str, graph: &mut SymbolicRdfCborLdPayload,
) -> Result<(), String> {
    use bytemuck::Zeroable;
    use qualia_core_db::{
        container_10d::{decode_holographic_phase_section_into, holographic_phase_encoded_len,
            EulerianWavefieldRecord, HolographicPhaseHeader, HOLOGRAPHIC_PHASE_HEADER_SIZE},
        q42::m64_matter::M64HoloRecord,
    };
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let view = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| error.to_string())?;
    let root_hash = q_hash(root);
    let protected_predicates = [
        source_lic!("licenseTier"),
        source_lic!("commercialUsePermittedWithoutSettlement"),
        source_lic!("derivativesPermittedOutsideEcosystem"),
        source_lic!("requiresAttributionPreservation"),
    ].map(q_hash);
    let asserted: BTreeMap<_, _> = graph.nquins.iter().filter(|row|
        row[0] == root_hash && protected_predicates.contains(&row[1]))
        .map(|row| (row[1], row[2])).collect();
    let mut represented = std::collections::BTreeSet::new();
    for field in view.payloads_of_kind(FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        let field = field.map_err(|error| error.to_string())?;
        let existing = decode_symbolic_rdf_cbor_ld(field.bytes).map_err(|error| error.to_string())?;
        for row in &existing.nquins {
            if row[0] == root_hash && asserted.get(&row[1]).is_some_and(|&object| object != row[2]) {
                return Err("Source licensing would conflict with an authored 10D license constraint".into());
            }
        }
        if existing.context_iri == "urn:qualia:10d:holo:addressability:v1" {
            represented.extend(existing.lexicon.into_iter().map(|term| (term.term_hash, term.iri_utf8)));
        }
    }
    let mut projection = SourceGraph {
        lex: graph.lexicon.drain(..).map(|term| (term.term_hash, term)).collect(),
        data: std::mem::take(graph),
    };
    let mut inspected = 0;
    for field in view.payloads_of_kind(FIELD_KIND_HOLOGRAPHIC_PHASE_SPACE) {
        let field = field.map_err(|error| error.to_string())?;
        let header_bytes = field.bytes.get(..HOLOGRAPHIC_PHASE_HEADER_SIZE).ok_or("Missing HOLO header")?;
        let header: HolographicPhaseHeader = bytemuck::pod_read_unaligned(header_bytes);
        let count = usize::from(header.holo_segment_count);
        let waves = usize::from(header.wavefield_count);
        if header.nquin_subject_hash != root_hash
            || holographic_phase_encoded_len(count, waves) != Some(field.bytes.len())
        {
            return Err("Native HOLO root or segment extent disagrees with the licensed 10D accession".into());
        }
        let mut segments = vec![M64HoloRecord::default(); count];
        let mut wavefields = vec![EulerianWavefieldRecord::zeroed(); waves];
        decode_holographic_phase_section_into(field.bytes, &mut segments, &mut wavefields)
            .map_err(|error| format!("Licensed 10D HOLO verification: {error:?}"))?;
        inspected += 1;
        for ordinal in 0..header.holo_segment_count {
            let iri = holo_segment_iri(root_hash, field.record.field_id, ordinal);
            if represented.contains(&(q_hash(&iri), iri.clone())) {
                continue;
            }
            let segment = projection.iri(&iri, None)?;
            let class = projection.iri("https://qualiadb.org/ontology/hmc#HoloSegment", None)?;
            projection.fact(root, "https://qualiadb.org/ontology/hmc#hasHoloSegment", segment)?;
            projection.fact(&iri, RDF_TYPE, class)?;
        }
    }
    if inspected == 0 {
        return Err("Licensed 10D does not carry a HOLO field".into());
    }
    projection.data.lexicon = projection.lex.into_values().collect();
    *graph = projection.data;
    Ok(())
}

struct SourceGraph {
    lex: BTreeMap<u64, MultilingualLexicalTerm>,
    data: SymbolicRdfCborLdPayload,
}

impl SourceGraph {
    fn iri(&mut self, iri: &str, label: Option<&str>) -> Result<u64, String> {
        let hash = q_hash(iri);
        if hash == 0 || !valid_iri(iri) {
            return Err("Licensing graph requires canonical nonzero RDF IRIs".into());
        }
        match self.lex.entry(hash) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(MultilingualLexicalTerm {
                    term_hash: hash,
                    iri_utf8: iri.into(),
                    lang_bcp47: None,
                    label_utf8: label.unwrap_or_default().into(),
                });
            }
            std::collections::btree_map::Entry::Occupied(entry)
                if entry.get().iri_utf8 == iri
                    && label.is_none_or(|label| entry.get().label_utf8 == label) => {}
            _ => return Err("Source licensing graph has a conflicting Q42 lexical binding".into()),
        }
        Ok(hash)
    }

    fn fact(&mut self, subject: &str, predicate: &str, object: u64) -> Result<(), String> {
        if object == 0 {
            return Err("Source licensing graph object hashes to zero".into());
        }
        let subject = self.iri(subject, None)?;
        let predicate = self.iri(predicate, None)?;
        let context = self.iri(LICENCE_CONTEXT, None)?;
        self.data.nquins.push([
            subject,
            predicate,
            object,
            context,
            0,
            NQuin::calculate_parity(subject, predicate, object, context, 0),
        ]);
        Ok(())
    }
}

fn section_bytes(bytes: &[u8], offset: u32, length: u32) -> Result<&[u8], String> {
    let start = offset as usize;
    let end = start
        .checked_add(length as usize)
        .ok_or("10D section extent overflow")?;
    bytes
        .get(start..end)
        .ok_or("10D section extent outside input".into())
}

fn reseal(
    bytes: &[u8],
    sidecar: Option<&[u8]>,
    mid2: Option<&[u8]>,
    provenance: Option<&[u8]>,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let header = Container10dHeader::parse(bytes).map_err(|error| error.to_string())?;
    let table = ten::parse_section_table(bytes, &header).map_err(|error| error.to_string())?;
    let mut input = Vec::with_capacity(table.len() + 2);
    for descriptor in table {
        let section_type = descriptor
            .typ()
            .ok_or("10D contains an unknown native section")?;
        let original = section_bytes(bytes, descriptor.byte_offset, descriptor.byte_length)?;
        let replacement = match section_type {
            SectionType::FieldSidecar => sidecar,
            SectionType::ManifoldIdentityV2 => mid2,
            SectionType::ProvenanceSidecar if provenance.is_some() => {
                return Err("Refusing to replace a preserved native PRV1 section".into());
            }
            _ => None,
        };
        input.push(SectionInput {
            section_type,
            alignment_tier: AlignmentTier::from_u8(descriptor.alignment_tier)
                .ok_or("Invalid native 10D alignment tier")?,
            stride: replacement.map_or(descriptor.stride, |_| 0),
            element_count: replacement.map_or(descriptor.element_count, |_| 0),
            payload: replacement.unwrap_or(original),
        });
    }
    for (kind, extra) in [
        (SectionType::FieldSidecar, sidecar),
        (SectionType::ProvenanceSidecar, provenance),
    ] {
        if let Some(payload) = extra {
            if !input.iter().any(|section| section.section_type == kind) {
                input.push(SectionInput {
                    section_type: kind,
                    alignment_tier: AlignmentTier::Page,
                    stride: 0,
                    element_count: 0,
                    payload,
                });
            }
        }
    }
    let needed = match ten::encode_container(&header, &input, &mut []) {
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) => needed,
        Err(error) => return Err(error.to_string()),
        Ok(_) => return Err("Native 10D encoder returned no section extent".into()),
    };
    if needed > limit {
        return Err(format!(
            "Licensed 10D requires {needed} bytes, greater than the {limit}-byte admitted ring"
        ));
    }
    let mut out = vec![0; needed];
    let written =
        ten::encode_container(&header, &input, &mut out).map_err(|error| error.to_string())?;
    out.truncate(written);
    seal_whole_file_crc32c(&mut out);
    Ok(out)
}

fn attach_licensing(
    bytes: &[u8],
    cbld: &[u8],
    expected: &SymbolicRdfCborLdPayload,
    limit: usize,
) -> Result<Vec<u8>, String> {
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut native_fields = scratch;
    let view = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut native_fields)
        .map_err(|error| error.to_string())?;
    let identity = view.identity().ok_or("Licensed 10D requires MID2")?;
    let mut matches = false;
    for entry in view.payloads_of_kind(FIELD_KIND_SYMBOLIC_RDF_CBOR_LD) {
        let entry = entry.map_err(|error| error.to_string())?;
        let prior = decode_symbolic_rdf_cbor_ld(entry.bytes).map_err(|error| error.to_string())?;
        if prior.context_iri == LICENCE_CONTEXT {
            if matches || &prior != expected {
                return Err(
                    "Existing source-licensing CBLD conflicts with the supplied source accession"
                        .into(),
                );
            }
            matches = true;
        }
    }
    if matches {
        return Ok(bytes.to_vec());
    }
    let raw_mid2 = view
        .section_payload(SectionType::ManifoldIdentityV2)
        .ok_or("Missing native MID2")?;
    let mut fields = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let (_, count) = ten::decode_manifold_identity_v2(raw_mid2, &[], &mut fields)
        .map_err(|error| error.to_string())?;
    let mut fields = fields[..count].to_vec();
    let mut sidecar = view
        .section_payload(SectionType::FieldSidecar)
        .unwrap_or_default()
        .to_vec();
    let next = fields
        .last()
        .map_or(0, |field| field.field_id)
        .checked_add(1)
        .ok_or("MID2 field ID overflow")?;
    fields.push(TypedFieldRecord {
        field_id: next,
        kind: FIELD_KIND_SYMBOLIC_RDF_CBOR_LD,
        flags: 0,
        unit_id: 0,
        payload_section_id: SectionType::FieldSidecar as u32,
        payload_offset: sidecar.len() as u64,
        payload_length: cbld.len() as u64,
    });
    sidecar.extend_from_slice(cbld);
    let len = manifold_identity_v2_encoded_len(fields.len()).map_err(|error| error.to_string())?;
    let mut mid2 = vec![0; len];
    let written = encode_manifold_identity_v2(&identity, &fields, &mut mid2)
        .map_err(|error| error.to_string())?;
    mid2.truncate(written);
    reseal(bytes, Some(&sidecar), Some(&mid2), None, limit)
}

fn make_manifest(asset: &str, original: &str, sealed: &str, mask: u16) -> Result<String, String> {
    let id = serde_json::to_string(asset).map_err(|error| error.to_string())?;
    let mut manifest = format!(
        "schema: yaml-ld-q42/v2\nid: {id}\nsource_sha256: \"{original}\"\n\
        archetype_10d_sha256: \"{sealed}\"\nholographic_10d_v2:\n  field_kind_holo: \"0x0306\"\n\
          field_kind_cbld: \"0x0305\"\n  field_kind_mstr: \"0x0304\"\nmultiscale_strata:\n"
    );
    for (index, title) in [
        "Continuum and Classical Field Mechanics",
        "Rigid and Articulated Multi-Body Kinetics",
        "Soft-Body, Elastoplastic and Fluid Continuum",
        "Molecular and Chemical Kinetics",
        "Cellular, Tissue and Biological Morphogenesis",
        "Ecological, Geomorphic and Planetary Systems",
        "L6QuantumPhysics",
    ]
    .into_iter()
    .enumerate()
    {
        let present = mask & (1u16 << index) != 0;
        manifest.push_str(&format!(
            "  L{index}:\n    name: \"{title}\"\n    present: {present}\n"
        ));
    }
    Ok(manifest)
}

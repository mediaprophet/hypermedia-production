//! Request-scoped QDNF policy projections and retained GeoTIFF ingestion.

use super::{fields, record, required, resource, text, unsigned, Args, AssetHost, Value};
use crate::{artifact::FieldLattice, list, TypedArtifact};
use qualia_core_db::{
    domains::geospatial::decode_geotiff_raster_into,
    net::{
        dns::sdn::parse_front_door_txt,
        qdnf::{
            authority::{Plane, TemporalGrant},
            errors::QdnfError,
            fabric::{
                as_natural_agent, attempt_join, validate_evidence_relation, AudienceScope,
                EvidenceRelation, JoinAttempt, JoinKey, NetworkReferent, RelationScopedLocator,
            },
            session::contact::{
                admit_contact, admit_help, group_membership_is_private_consent,
                payment_bypasses_block, revoke_mandate, ContactTable,
            },
            types::{ProfileId, StrongDigest},
        },
    },
};
use std::{collections::BTreeMap, ops::Range, sync::Arc};

const MAX_ACTIONS: usize = 128;
const MAX_CELLS: usize = 65_536;
const MAX_IFD_ENTRIES: usize = 128;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, args: &Args) -> Result<Value, String> {
    match op {
        "geotiff_to_10d_fiel" => geotiff_to_field(host, args),
        "qdnf_dns_front_door" => qdp_dns(host, args),
        "qdnf_contact_gate" => qdnf_contact(host, args),
        "identifier_fabric_guard" => identifier_fabric(args),
        "identifier_referent_guard" => identifier_referent(args),
        "identifier_locator_guard" => identifier_locator(args),
        _ => Err(format!("Unsupported shared-service operation: {op}")),
    }
}

fn boolean(value: &Value) -> Result<bool, String> {
    match value {
        Value::Bool(value) => Ok(*value),
        _ => Err("Expected a boolean".into()),
    }
}

fn qdp_dns(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["domain", "txt", "ttl", "bilateral_friend"])?;
    let supplied_domain = text(required(args, "domain")?)?;
    let domain = supplied_domain.strip_suffix('.').unwrap_or(supplied_domain);
    let txt = text(required(args, "txt")?)?;
    let ttl = u32::try_from(unsigned(required(args, "ttl")?)?)
        .map_err(|_| "QDP DNS TTL exceeds u32")?;
    let bilateral = args.get("bilateral_friend").map(boolean).transpose()?.unwrap_or_default();
    if domain.is_empty() || domain.len() > 248 || domain.split('.').any(|label|
        label.is_empty() || label.len() > 63 || label.starts_with('-') || label.ends_with('-')
        || !label.bytes().all(|byte| byte.is_ascii_alphanumeric() || byte == b'-'))
        || txt.is_empty() || txt.len() > 4096 || txt.chars().any(char::is_control)
    {
        return Err("QDP front door requires a bounded DNS domain and TXT payload".into());
    }
    host.charge(txt.len(), txt.len())?;
    let domain = domain.to_ascii_lowercase();
    const KEYS: [&str; 8] = ["qdp:signer", "qdp:agentType", "qdp:identityKey", "qdp:wireguard",
        "qdp:overlay", "qdp:nym", "qdp:ecash", "qdp:profile"];
    let mut seen = [false; KEYS.len()];
    for clause in txt.split(';').map(str::trim).filter(|clause| !clause.is_empty()) {
        let (key, value) = clause.split_once(|c: char| c.is_whitespace() || c == '=')
            .ok_or("Malformed QDP TXT clause")?;
        let index = KEYS.iter().position(|&candidate| candidate == key)
            .ok_or("Unsupported QDP TXT clause")?;
        if seen[index] || value.trim().is_empty() {
            return Err("Duplicate or empty QDP TXT clause".into());
        }
        seen[index] = true;
    }
    let view = parse_front_door_txt(&domain, txt)?;
    let signer = view.front_door_did;
    let payload = signer.strip_prefix("did:q42:").ok_or("QDP signer requires a did:q42 coordinate")?;
    if payload.is_empty() || payload.chars()
        .any(|ch| ch.is_whitespace() || ch.is_control() || "<>\"{}|\\^`".contains(ch))
    {
        return Err("Malformed QDP signer coordinate".into());
    }
    let pointer = view.did_topological_pointer.ok_or("QDP signer pointer is unavailable")?;
    let quin = view.to_quin(ttl, bilateral).ok_or("QDP front door could not encode its quin")?;
    if !quin.verify_ecc_parity() {
        return Err("QDP front-door quin failed its parity check".into());
    }
    Ok(Value::Record(BTreeMap::from([
        ("dns_name".into(), Value::String(format!("_qdp.{domain}."))),
        ("signer".into(), Value::String(signer.into())),
        ("route_pointer".into(), Value::U64(pointer)),
        ("declared_agent_type".into(), Value::String(view.agent_type.into())),
        ("proposed_routing_lane".into(), Value::U64(view.determine_routing_lane(bilateral) as u64)),
        ("quin".into(), Value::List([quin.subject, quin.predicate, quin.object, quin.context,
            quin.metadata, quin.parity].into_iter().map(Value::U64).collect())),
        ("claim_verified".into(), Value::Bool(false)),
    ])))
}

fn digest(value: &Value) -> Result<StrongDigest, String> {
    fn nibble(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            _ => None,
        }
    }
    let input = text(value)?.as_bytes();
    let mut bytes = [0u8; 48];
    if input.len() != bytes.len() * 2 {
        return Err("QDNF fixture digest must contain 96 hex digits".into());
    }
    for (pair, byte) in input.chunks_exact(2).zip(&mut bytes) {
        let high = nibble(pair[0]).ok_or("Invalid QDNF digest hex digit")?;
        let low = nibble(pair[1]).ok_or("Invalid QDNF digest hex digit")?;
        *byte = (high << 4) | low;
    }
    let digest = StrongDigest(bytes);
    if digest.is_zero() {
        return Err("QDNF fixture digests must be nonzero".into());
    }
    Ok(digest)
}

fn outcome(result: Result<(), QdnfError>) -> Value {
    let mut value = BTreeMap::from([("allowed".into(), Value::Bool(result.is_ok()))]);
    if let Err(error) = result {
        value.insert("code".into(), Value::U64(error as u64));
        value.insert("diagnostic".into(), Value::String(error.to_string()));
    }
    Value::Record(value)
}

fn grant(value: &Value) -> Result<TemporalGrant, String> {
    let row = record(value)?;
    fields(row, &["purpose", "audience", "authority_generation", "not_before_unix", "expires_unix"])?;
    let result = TemporalGrant {
        purpose_digest: digest(required(row, "purpose")?)?,
        audience_digest: digest(required(row, "audience")?)?,
        authority_generation: unsigned(required(row, "authority_generation")?)?,
        not_before_unix: unsigned(required(row, "not_before_unix")?)?,
        expires_unix: unsigned(required(row, "expires_unix")?)?,
        profile: ProfileId::QDNF_CRYPTO_1,
    };
    if result.not_before_unix >= result.expires_unix || result.authority_generation == 0 {
        return Err("QDNF fixture grant requires a nonempty interval and current generation".into());
    }
    Ok(result)
}

fn contact_action(table: &mut ContactTable, op: &str, row: &Args, peer: StrongDigest)
    -> Result<Result<(), QdnfError>, String> {
    Ok(match op {
        "request" => {
            fields(row, &["op", "peer", "purpose", "paid", "mandate_generation"])?;
            table.insert_request(peer, digest(required(row, "purpose")?)?,
                boolean(required(row, "paid")?)?, unsigned(required(row, "mandate_generation")?)?)
        }
        "revoke" => {
            fields(row, &["op", "peer", "generation"])?;
            revoke_mandate(table, peer, unsigned(required(row, "generation")?)?)
        }
        "admit" => {
            fields(row, &["op", "peer", "purpose", "paid", "now_unix", "grant", "audience",
                "authority_generation"])?;
            let purpose = digest(required(row, "purpose")?)?;
            let audience = digest(required(row, "audience")?)?;
            let authority_generation = unsigned(required(row, "authority_generation")?)?;
            let grant = grant(required(row, "grant")?)?;
            let paid = boolean(required(row, "paid")?)?;
            let now = unsigned(required(row, "now_unix")?)?;
            admit_contact(table, peer, purpose, paid, now, &grant).and_then(|()| {
                if grant.audience_digest != audience {
                    return Err(QdnfError::Unauthorized);
                }
                if grant.authority_generation != authority_generation {
                    return Err(QdnfError::StaleGeneration);
                }
                Ok(())
            })
        }
        "consent" | "activate" | "suspend" | "block" | "help" => {
            fields(row, &["op", "peer"])?;
            match op {
                "consent" => table.consent(peer),
                "activate" => table.activate(peer),
                "suspend" => table.suspend(peer),
                "block" => table.block(peer),
                "help" => admit_help(table, peer),
                _ => unreachable!("contact operation validated"),
            }
        }
        _ => return Err("Unknown QDNF contact operation".into()),
    })
}

fn qdnf_contact(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["actions"])?;
    let actions = list(required(args, "actions")?)?;
    if actions.len() > MAX_ACTIONS {
        return Err("QDNF contact fixture exceeds the bounded action count".into());
    }
    host.charge(actions.len() * 16, actions.len() * 192)?;
    let mut table = ContactTable::new();
    let mut trace = Vec::with_capacity(actions.len());
    for action in actions {
        let row = record(action)?;
        let op = text(required(row, "op")?)?;
        let peer = digest(required(row, "peer")?)?;
        let result = contact_action(&mut table, op, row, peer)?;
        let state = table.get(peer).ok().map(|slot| Value::String(format!("{:?}", slot.state)))
            .unwrap_or(Value::Null);
        trace.push(Value::Record(BTreeMap::from([
            ("op".into(), Value::String(op.into())),
            ("contact_state".into(), state),
            ("decision".into(), outcome(result)),
        ])));
    }
    Ok(Value::Record(BTreeMap::from([
        ("fixture".into(), Value::Bool(true)),
        ("contact_occupied".into(), Value::U64(table.contact_occupied() as u64)),
        ("help_occupied".into(), Value::U64(table.help_occupied() as u64)),
        ("payment_bypasses_block".into(), Value::Bool(payment_bypasses_block())),
        ("group_membership_is_private_consent".into(), Value::Bool(group_membership_is_private_consent())),
        ("trace".into(), Value::List(trace)),
    ])))
}

fn plane(value: &Value) -> Result<Plane, String> {
    match text(value)? {
        "entity" => Ok(Plane::Entity),
        "claim" => Ok(Plane::Claim),
        "handle" => Ok(Plane::Handle),
        "instrument" => Ok(Plane::Instrument),
        _ => Err("Unknown identifier fabric plane".into()),
    }
}

fn identifier_fabric(args: &Args) -> Result<Value, String> {
    fields(args, &["left", "right", "join_key", "relation"])?;
    let left = plane(required(args, "left")?)?;
    let right = plane(required(args, "right")?)?;
    match (args.get("join_key"), args.get("relation")) {
        (Some(key), None) => {
            let key = match text(key)? {
                "did" => JoinKey::Did,
                "wallet" => JoinKey::Wallet,
                "location" => JoinKey::Location,
                "role" => JoinKey::Role,
                "alias" => JoinKey::Alias,
                "classifier_similarity" => JoinKey::ClassifierSimilarity,
                "same_as_assertion" => JoinKey::SameAsAssertion,
                "biometric_sample" => JoinKey::BiometricSample,
                "account_handle" => JoinKey::AccountHandle,
                "sensor_co_presence" => JoinKey::SensorCoPresence,
                "organization_representative" => JoinKey::OrgRepresentative,
                "reliance_retraction" => JoinKey::RelianceRetraction,
                _ => return Err("Unknown identifier join key".into()),
            };
            Ok(outcome(attempt_join(JoinAttempt { left, right, key })))
        }
        (None, Some(relation)) => {
            let relation = match text(relation)? {
                "supported_by_instrument" => EvidenceRelation::SupportedByInstrument,
                "asserted_about" => EvidenceRelation::AssertedAbout,
                "observed_at_handle" => EvidenceRelation::ObservedAtHandle,
                _ => return Err("Unknown identifier fabric relation".into()),
            };
            Ok(outcome(validate_evidence_relation(relation, left, right)))
        }
        _ => Err("Supply exactly one identifier join key or typed evidence relation".into()),
    }
}

fn identifier_referent(args: &Args) -> Result<Value, String> {
    fields(args, &["referent"])?;
    let referent = match text(required(args, "referent")?)? {
        "natural_agent" => NetworkReferent::NaturalAgent,
        "verified_route" => NetworkReferent::VerifiedRoute,
        "instrument" => NetworkReferent::Instrument,
        "ai_agent" => NetworkReferent::AiAgent,
        "machine_device" => NetworkReferent::MachineDevice,
        "organization" => NetworkReferent::Organization,
        "role_capacity" => NetworkReferent::RoleCapacity,
        _ => return Err("Unknown network referent".into()),
    };
    Ok(outcome(as_natural_agent(referent)))
}

fn identifier_locator(args: &Args) -> Result<Value, String> {
    fields(args, &["locator_hash", "direction_hash", "relation_context_hash", "purpose_bind_hash",
        "requested_purpose_hash", "audience_scope", "not_before_unix", "expires_unix", "withdrawn",
        "now_unix", "proxy_hop"])?;
    let audience_scope = match text(required(args, "audience_scope")?)? {
        "private_pairwise" => AudienceScope::PrivatePairwise,
        "closed_group" => AudienceScope::ClosedGroup,
        "community" => AudienceScope::Community,
        "public_service" => AudienceScope::PublicService,
        _ => return Err("Unknown relation-scoped locator audience".into()),
    };
    let read = |name| unsigned(required(args, name)?);
    let locator = RelationScopedLocator {
        locator_hash: read("locator_hash")?,
        direction_hash: read("direction_hash")?,
        relation_context_hash: read("relation_context_hash")?,
        purpose_bind_hash: read("purpose_bind_hash")?,
        audience_scope,
        not_before: read("not_before_unix")?,
        expires_at: read("expires_unix")?,
        withdrawn: boolean(required(args, "withdrawn")?)?,
    };
    let now = read("now_unix")?;
    let lifecycle = locator.validate_locator_use(
        read("requested_purpose_hash")?, now, boolean(required(args, "proxy_hop")?)?,
    );
    let publication = lifecycle.and_then(|()| locator.admit_public_qsr_publication());
    Ok(Value::Record(BTreeMap::from([
        ("fixture".into(), Value::Bool(true)),
        ("locator_use".into(), outcome(lifecycle)),
        ("public_qsr_publication".into(), outcome(publication)),
    ])))
}

#[derive(Clone, Copy)]
struct IfdEntry {
    tag: u16,
    ty: u16,
    count: u32,
    value: u32,
}

impl IfdEntry {
    fn scalar(self) -> Result<u32, String> {
        match (self.ty, self.count) {
            (3, 1) => Ok(self.value & 0xffff),
            (4, 1) => Ok(self.value),
            _ => Err(format!("Unexpected scalar TIFF field type for tag {}", self.tag)),
        }
    }

    fn extent(self, size: usize) -> Result<Range<usize>, String> {
        let start = self.value as usize;
        let end = usize::try_from(self.count).ok().and_then(|count| count.checked_mul(size))
            .and_then(|len| start.checked_add(len)).ok_or("GeoTIFF extent overflows address space")?;
        if start < 8 || start % 2 != 0 {
            return Err("GeoTIFF auxiliary block has an invalid offset".into());
        }
        Ok(start..end)
    }
}

struct GeoTiffLayout {
    width: usize,
    height: usize,
    epsg: u16,
}

fn u16_at(input: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes(input[offset..offset + 2].try_into().expect("checked TIFF field"))
}

fn u32_at(input: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(input[offset..offset + 4].try_into().expect("checked TIFF field"))
}

fn overlaps(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn geo_keys(bytes: &[u8], entry: IfdEntry) -> Result<(u16, Range<usize>), String> {
    if entry.ty != 3 || entry.count < 4 || entry.count > 516 {
        return Err("GeoTIFF requires a bounded GeoKey directory".into());
    }
    let extent = entry.extent(2)?;
    let data = bytes.get(extent.clone()).ok_or("Truncated GeoKey directory")?;
    let count = usize::from(u16_at(data, 6));
    if u16_at(data, 0) != 1 || u16_at(data, 2) != 1 || data.len() != 8 + count * 8 {
        return Err("Malformed or unsupported GeoKey directory revision".into());
    }
    let (mut model, mut pixels, mut epsg, mut units) = (None, None, None, None);
    for row in data[8..].chunks_exact(8) {
        let target = match u16_at(row, 0) {
            1024 => &mut model,
            1025 => &mut pixels,
            3072 => &mut epsg,
            3076 => &mut units,
            _ => continue,
        };
        if u16_at(row, 2) != 0 || u16_at(row, 4) != 1 || target.replace(u16_at(row, 6)).is_some() {
            return Err("Unsupported or repeated projected GeoTIFF GeoKey".into());
        }
    }
    let epsg = epsg.filter(|&epsg| (1024..32767).contains(&epsg))
        .ok_or("GeoTIFF requires a declared projected EPSG code")?;
    if model != Some(1) || pixels != Some(1) || units != Some(9001) {
        return Err("GeoTIFF requires projected metre units and pixel-is-area semantics".into());
    }
    Ok((epsg, extent))
}

fn raster_layout(bytes: &[u8]) -> Result<GeoTiffLayout, String> {
    if bytes.len() < 8 || bytes[..4] != [b'I', b'I', 42, 0] {
        return Err("GeoTIFF requires classic little-endian TIFF".into());
    }
    let start = u32_at(bytes, 4) as usize;
    let ifd = bytes.get(start..).filter(|ifd| start >= 8 && start % 2 == 0 && ifd.len() >= 2)
        .ok_or("Invalid GeoTIFF IFD offset")?;
    let count = usize::from(u16_at(ifd, 0));
    if count > MAX_IFD_ENTRIES || ifd.len() < count * 12 + 6 {
        return Err("Truncated or oversized GeoTIFF IFD".into());
    }
    let ifd_end = count * 12 + 2;
    if u32_at(ifd, ifd_end) != 0 {
        return Err("GeoTIFF conversion accepts one image directory".into());
    }
    let mut tags = [IfdEntry { tag: 0, ty: 0, count: 0, value: 0 }; MAX_IFD_ENTRIES];
    for (index, row) in ifd[2..ifd_end].chunks_exact(12).enumerate() {
        let tag = u16_at(row, 0);
        if tags[..index].iter().any(|entry| entry.tag == tag) {
            return Err("GeoTIFF contains a duplicate TIFF tag".into());
        }
        tags[index] = IfdEntry { tag, ty: u16_at(row, 2), count: u32_at(row, 4), value: u32_at(row, 8) };
    }
    let find = |tag| tags[..count].iter().find(|entry| entry.tag == tag).copied();
    let require = |tag| find(tag).ok_or_else(|| format!("GeoTIFF is missing TIFF tag {tag}"));
    let scalar = |tag: u16, default| find(tag).map(IfdEntry::scalar).transpose()
        .map(|value| value.unwrap_or(default));
    let width = require(256)?.scalar()? as usize;
    let height = require(257)?.scalar()? as usize;
    let cells = width.checked_mul(height).filter(|&cells| cells <= MAX_CELLS)
        .ok_or("GeoTIFF exceeds the bounded cell count")?;
    if !(2..=2048).contains(&width) || !(2..=2048).contains(&height)
        || scalar(259, 1)? != 1 || scalar(262, 1)? != 1 || scalar(274, 1)? != 1
        || scalar(277, 1)? != 1 || scalar(284, 1)? != 1 || scalar(317, 1)? != 1
        || scalar(278, height as u32)? < height as u32
        || [322, 323, 324, 325, 330, 338, 34264, 42113].into_iter().any(|tag| find(tag).is_some())
    {
        return Err("GeoTIFF requires one uncompressed north-up scalar strip with complete data".into());
    }
    let sample_bytes = match (require(339)?.scalar()?, require(258)?.scalar()?) {
        (3, 32) => 4,
        (1 | 2, 16) => 2,
        _ => return Err("Unsupported GeoTIFF scalar sample encoding".into()),
    };
    let strip = require(273)?;
    let strip_bytes = require(279)?.scalar()? as usize;
    if strip.ty != 4 || strip.count != 1 || strip_bytes != cells * sample_bytes {
        return Err("GeoTIFF scalar strip count or byte extent is invalid".into());
    }
    let start_of_strip = strip.scalar()? as usize;
    let strip_end = start_of_strip.checked_add(strip_bytes).ok_or("GeoTIFF strip extent overflow")?;
    let strip = start_of_strip..strip_end;
    let ifd = start..start + ifd_end + 4;
    if strip.start < 8 || strip.end > bytes.len() || overlaps(&strip, &ifd) {
        return Err("GeoTIFF scalar strip is outside its proper file extent".into());
    }
    let scale = require(33550)?;
    let tiepoint = require(33922)?;
    if (scale.ty, scale.count) != (12, 3) || (tiepoint.ty, tiepoint.count) != (12, 6) {
        return Err("GeoTIFF requires a three-axis pixel scale and a single tie point".into());
    }
    let (epsg, geo_extent) = geo_keys(bytes, require(34735)?)?;
    let scale = scale.extent(8)?;
    let tiepoint = tiepoint.extent(8)?;
    let metadata = [scale.clone(), tiepoint.clone(), geo_extent];
    for (index, extent) in metadata.iter().enumerate() {
        if extent.end > bytes.len() || overlaps(extent, &strip) || overlaps(extent, &ifd)
            || metadata[..index].iter().any(|prior| overlaps(extent, prior))
        {
            return Err("GeoTIFF georeference blocks overlap or exceed the file extent".into());
        }
    }
    let double = |offset| f64::from_le_bytes(bytes[offset..offset + 8].try_into().expect("checked GeoTIFF tag"));
    let sx = double(scale.start);
    let sy = double(scale.start + 8);
    if !(1e-6..=1e6).contains(&sx) || sx != sy
        || (0..3).any(|index| double(tiepoint.start + index * 8) != 0.0)
        || !(3..6).all(|index| double(tiepoint.start + index * 8).is_finite())
    {
        return Err("GeoTIFF requires square metre pixels and a finite zero-index tie point".into());
    }
    Ok(GeoTiffLayout { width, height, epsg })
}

fn geotiff_to_field(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
    fields(args, &["geotiff", "origin_m", "entity_iri", "source_iri", "name_iri", "unit_iri"])?;
    let input = required(args, "geotiff")?;
    let source = resource(host, input, "geotiff")?;
    let layout = raster_layout(&source.bytes)?;
    let cells = layout.width * layout.height;
    host.charge(source.bytes.len().saturating_add(cells), cells * 16)?;
    let source = &resource(host, input, "geotiff")?.bytes;
    let source_digest = blake3::hash(source).to_hex().to_string();
    let mut samples = vec![0.0; cells];
    let decoded = decode_geotiff_raster_into(source, &mut samples)
        .map_err(|error| format!("Native GeoTIFF decode failed: {error:?}"))?;
    let (west, south, east, north) = decoded.bbox;
    if decoded.width != layout.width || decoded.height != layout.height
        || decoded.sample_count != cells || [west, south, east, north].iter().any(|v| !v.is_finite())
        || west >= east || south >= north
        || samples.iter().any(|value| !(-1e18..=1e18).contains(value))
    {
        return Err("Decoded GeoTIFF raster disagrees with the admitted shape or finite sample domain".into());
    }
    // TIFF rows start at the northern edge. The local footprint origin identifies the southwest
    // sample center, so the FIEL rows advance northward while the reported bbox covers pixel edges.
    for row in 0..decoded.height / 2 {
        for column in 0..decoded.width {
            samples.swap(row * decoded.width + column, (decoded.height - row - 1) * decoded.width + column);
        }
    }
    let field = host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(FieldLattice::DenseF32 {
        axes: vec![decoded.height as u32, decoded.width as u32],
        samples,
    })))?;
    let mut parameters = BTreeMap::from([
        ("width".into(), Value::U64(decoded.width as u64)),
        ("height".into(), Value::U64(decoded.height as u64)),
        ("cell_size_m".into(), Value::F64(decoded.cell_size_m)),
        ("field".into(), field),
    ]);
    for name in ["origin_m", "entity_iri", "source_iri", "name_iri", "unit_iri"] {
        parameters.insert(name.into(), required(args, name)?.clone());
    }
    let command = BTreeMap::from([
        ("op".into(), Value::String("seal_datacube_raster".into())),
        ("parameters".into(), Value::Record(parameters)),
    ]);
    let Value::Record(mut result) = host.evaluate_gis_gp_terrain_and_datacubes(&command)? else {
        return Err("GIS raster sealer returned an invalid result".into());
    };
    result.insert("source_digest".into(), Value::String(source_digest));
    result.insert("epsg".into(), Value::U64(u64::from(layout.epsg)));
    result.insert("source_bbox_m".into(), Value::List(
        [west, south, east, north].into_iter().map(Value::F64).collect()));
    result.insert("sample_row_order".into(), Value::String("south_to_north".into()));
    Ok(Value::Record(result))
}

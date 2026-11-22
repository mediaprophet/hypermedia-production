//! Bounded Q42 spatial preflight, situated render decisions, and model substrates.

use crate::{
    artifact::CartridgeKind, fields, list, num, required, text, vector, Args, AssetHost, Resource,
    TypedArtifact, Value,
};
use qualia_core_db::{
    bundle::semantic_model::SemanticModelBuilder,
    container_10d::{
        integrity::check_whole_file_crc32c,
        parse_node_header, read_node,
        section::{parse_section_table, SectionType},
        Container10dHeader,
    },
    p64_weight::{IntegrityMode, P64TensorIndex},
    q42::spatial_predicates::*,
    q42_volume::{decode_and_verify_car, CidSha256, WasmSliceQ42Reader},
    render::{
        model_substrate::{compose_substrate, read_substrate, SUBSTRATE_HEADER_BYTES},
        place_time::{situate_artefact, situated_render_verdict, MAX_RENDER_NORMS},
    },
    tensor::buffer_export::{write_tensor_buffer, TensorBufferHeader},
    NQuin,
};
use std::collections::BTreeMap;

const GRAPH_BUDGET: usize = 42 * 1024 * 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "pack_coordinate" => coordinate(host, params),
        "pack_link" => link(host, params),
        "situated_render" => situated(host, params),
        "compose_substrate" => substrate(host, params),
        "verify_car_block" => car_block(host, params),
        _ => Err(format!("Unsupported Q42 spatial operation: {op}")),
    }
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => u64::try_from(*value).map_err(|_| "Expected unsigned integer".into()),
        _ => Err("Expected unsigned integer".into()),
    }
}

fn epoch(value: &Value) -> Result<i64, String> {
    let value = match value {
        Value::I64(value) => *value,
        Value::U64(value) => i64::try_from(*value).map_err(|_| "Epoch exceeds i64")?,
        _ => return Err("Expected integer epoch seconds".into()),
    };
    i32::try_from(value)
        .map(i64::from)
        .map_err(|_| "Epoch exceeds the situated 32-bit format".into())
}

fn graph(host: &AssetHost<'_>) -> SemanticModelBuilder {
    SemanticModelBuilder::new(host.limits.resource_bytes.min(GRAPH_BUDGET))
}

fn bind(graph: &mut SemanticModelBuilder, iri: &str) -> Result<u64, String> {
    if iri.len() > 2048 {
        return Err("Spatial IRI exceeds token budget".into());
    }
    graph.bind_iri(iri).map_err(|error| error.to_string())
}

fn encode(
    host: &mut AssetHost<'_>,
    graph: SemanticModelBuilder,
    row: NQuin,
) -> Result<Value, String> {
    let bytes = graph
        .seal_projection(std::iter::once(row))
        .map_err(|error| error.to_string())?;
    let quin = Value::List(
        [
            row.subject,
            row.predicate,
            row.object,
            row.context,
            row.metadata,
            row.parity,
        ]
        .into_iter()
        .map(Value::U64)
        .collect(),
    );
    Ok(Value::Record(BTreeMap::from([
        ("q42".into(), host.encode("q42", bytes)?),
        ("quin".into(), quin),
    ])))
}

fn coordinate(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "subject",
            "context",
            "predicate",
            "position_m",
            "scale_tier",
            "metadata",
        ],
    )?;
    let mut graph = graph(host);
    let subject = bind(&mut graph, text(required(params, "subject")?)?)?;
    let context = bind(&mut graph, text(required(params, "context")?)?)?;
    let predicate = match text(required(params, "predicate")?)? {
        iri @ ("geo:hasVertex"
        | "geo:hasPackedCentroid"
        | "geo:hasPackedBboxMin"
        | "geo:hasPackedBboxMax") => bind(&mut graph, iri)?,
        _ => return Err("Unknown packed spatial coordinate predicate".into()),
    };
    let tier = u8::try_from(unsigned(required(params, "scale_tier")?)?)
        .ok()
        .filter(|&tier| tier <= 7)
        .ok_or("Spatial scale tier must be in 0..=7")?;
    let point = vector::<3>(required(params, "position_m")?)?.map(|value| value as f32);
    let step = scale_tier_step_meters(tier);
    if point
        .iter()
        .any(|value| !(-524_288.0..=524_287.0).contains(&(value / step).round()))
    {
        return Err(
            "Spatial coordinate exceeds the signed 20-bit range at the requested scale".into(),
        );
    }
    let metadata = params
        .get("metadata")
        .map(unsigned)
        .transpose()?
        .unwrap_or(0);
    host.charge(1, 0)?;
    encode(
        host,
        graph,
        make_spatial_coord_quin(
            subject, predicate, point[0], point[1], point[2], tier, context, metadata,
        ),
    )
}

fn link(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "subject",
            "context",
            "predicate",
            "target",
            "slot",
            "chunk_records",
        ],
    )?;
    let mut graph = graph(host);
    let subject = bind(&mut graph, text(required(params, "subject")?)?)?;
    let context = bind(&mut graph, text(required(params, "context")?)?)?;
    let target = bind(&mut graph, text(required(params, "target")?)?)?;
    let name = text(required(params, "predicate")?)?;
    let predicate = bind(&mut graph, name)?;
    let slot = u32::try_from(unsigned(required(params, "slot")?)?)
        .map_err(|_| "Spatial slot exceeds u32")?;
    let row = if matches!(
        name,
        "q42:hasContainer10d"
            | "q42:hasM64Chunk"
            | "q42:hasEmfChunk"
            | "q42:hasSpectralSheetV3"
            | "q42:hasHmcBundle"
            | "q42:hasClusterDag"
    ) {
        let count = u16::try_from(unsigned(required(params, "chunk_records")?)?)
            .map_err(|_| "Spatial chunk count exceeds u16")?;
        make_container_link_quin(subject, predicate, target, slot, count, context)
    } else {
        if params.contains_key("chunk_records") {
            return Err("Chunk count belongs to a volume link".into());
        }
        match name {
            "kin:hasPgaMotor"
            | "kin:hasJointParent"
            | "bio:hasHillMuscle"
            | "bio:hasAllometricLaw"
            | "mech:hasFatigueLaw"
            | "phys:hasBondPort" => make_kinematic_quin(subject, predicate, target, slot, context),
            "sens:hasHoaOrder"
            | "sens:hasSofaHrtf"
            | "sens:hasStokesVector"
            | "sens:hasRefractiveIndex" => {
                make_sensory_quin(subject, predicate, target, slot, context)
            }
            "llm:hasKvSlot" | "llm:hasSparseAtomCode" => {
                make_llm_grounding_quin(subject, predicate, slot, target, context)
            }
            _ => return Err("Unknown spatial link predicate".into()),
        }
    };
    host.charge(1, 0)?;
    encode(host, graph, row)
}

fn load_resource<'a>(
    host: &'a AssetHost<'_>,
    value: &Value,
    kind: &str,
) -> Result<&'a Resource, String> {
    let resource = match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|entry| &entry.key == key)
            .ok_or_else(|| format!("Unknown retained resource {key}"))?,
        _ => host.workspace.encoded(value)?,
    };
    if resource.kind != kind || resource.bytes.len() > host.limits.resource_bytes {
        return Err(format!("Expected bounded {kind} resource"));
    }
    Ok(resource)
}

fn situated(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    use qualia_core_db::modalities::spatio_temporal::Rcc8Relation;
    fields(
        params,
        &[
            "subject",
            "jurisdiction",
            "position",
            "valid_from",
            "valid_to",
            "polygon",
            "radius",
            "window_start",
            "window_end",
            "now_unix",
            "norms",
        ],
    )?;
    let mut graph = graph(host);
    let subject = bind(&mut graph, text(required(params, "subject")?)?)?;
    let jurisdiction = bind(&mut graph, text(required(params, "jurisdiction")?)?)?;
    bind(&mut graph, "urn:qualia:place:situatedAt")?;
    let [x, y] = vector::<2>(required(params, "position")?)?;
    let valid_from = epoch(required(params, "valid_from")?)?;
    let valid_to = epoch(required(params, "valid_to")?)?;
    let window_start = epoch(required(params, "window_start")?)?;
    let window_end = epoch(required(params, "window_end")?)?;
    let radius = num(required(params, "radius")?)?;
    if x.abs() > 1.0e12
        || y.abs() > 1.0e12
        || radius <= 0.
        || radius > 1.0e12
        || valid_from >= valid_to
        || window_start >= window_end
    {
        return Err(
            "Situated footprint or temporal interval is outside the supported domain".into(),
        );
    }
    let points = list(required(params, "polygon")?)?;
    if !(3..=4096).contains(&points.len()) {
        return Err("Situated jurisdiction needs a bounded polygon".into());
    }
    let points = points
        .iter()
        .map(|value| {
            let [x, y] = vector::<2>(value)?;
            if x.abs() > 1.0e12 || y.abs() > 1.0e12 {
                return Err("Jurisdiction coordinate exceeds spatial bounds".into());
            }
            Ok((x, y))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let area: f64 = points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(&(a, b), &(c, d))| a * d - b * c)
        .sum();
    if area == 0. {
        return Err("Situated jurisdiction polygon has no area".into());
    }
    let now = u32::try_from(unsigned(required(params, "now_unix")?)?)
        .map_err(|_| "Norm evaluation time exceeds u32")?;
    let policy = params
        .get("norms")
        .map(|value| {
            let bytes = &load_resource(host, value, "q42")?.bytes;
            SemanticModelBuilder::new(host.limits.resource_bytes.min(GRAPH_BUDGET))
                .merge_q42(bytes)
                .map_err(|error| error.to_string())
        })
        .transpose()?;
    let norms = policy
        .as_ref()
        .map(SemanticModelBuilder::snapshot_quins)
        .unwrap_or(&[]);
    if norms.len() > MAX_RENDER_NORMS || norms.iter().any(|row| row.context != jurisdiction) {
        return Err(
            "Render norms exceed the bounded scope or belong to another jurisdiction".into(),
        );
    }
    host.charge(points.len().saturating_add(norms.len()), points.len() * 16)?;
    let fact = situate_artefact(subject, x, y, valid_from, valid_to, jurisdiction);
    let verdict = situated_render_verdict(
        &fact,
        jurisdiction,
        &points,
        radius,
        window_start,
        window_end,
        norms,
        now,
    );
    let place = match verdict.place {
        Rcc8Relation::Disconnected => "disconnected",
        Rcc8Relation::ExternallyConnected => "externally_connected",
        Rcc8Relation::PartiallyOverlapping => "partially_overlapping",
        Rcc8Relation::Equal => "equal",
        Rcc8Relation::TangentiallyProperPart => "tangentially_proper_part",
        Rcc8Relation::NonTangentialProperPart => "non_tangential_proper_part",
        Rcc8Relation::TangentiallyProperPartInverse => "tangentially_proper_part_inverse",
        Rcc8Relation::NonTangentialProperPartInverse => "non_tangential_proper_part_inverse",
    };
    let artifact = graph
        .seal_projection(std::iter::once(fact))
        .map_err(|error| error.to_string())?;
    Ok(Value::Record(BTreeMap::from([
        ("q42".into(), host.encode("q42", artifact)?),
        ("place".into(), Value::String(place.into())),
        ("within_place".into(), Value::Bool(verdict.within_place)),
        ("within_time".into(), Value::Bool(verdict.within_time)),
        (
            "deontic_permits".into(),
            Value::Bool(verdict.deontic_permits),
        ),
        ("admit".into(), Value::Bool(verdict.admit)),
    ])))
}

fn substrate(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["manifold", "weights"])?;
    let manifold = required(params, "manifold")?;
    let weights = required(params, "weights")?;
    let m_len = load_resource(host, manifold, "10d")?.bytes.len();
    let w_len = weights_bytes(host, weights)?.len();
    host.charge(m_len.saturating_add(w_len), m_len.saturating_add(w_len))?;
    let source = &load_resource(host, manifold, "10d")?.bytes;
    let weights = weights_bytes(host, weights)?;
    check_whole_file_crc32c(source).map_err(|error| format!("Manifold integrity: {error:?}"))?;
    let header = Container10dHeader::parse(source).map_err(|error| error.to_string())?;
    let table = parse_section_table(source, &header).map_err(|error| error.to_string())?;
    let row = table
        .iter()
        .find(|row| row.typ() == Some(SectionType::Tensor10DNodes))
        .ok_or("Substrate requires an authored Tensor10D node section")?;
    let start = row.byte_offset as usize;
    let nodes = source
        .get(start..start + row.byte_length as usize)
        .ok_or("Invalid node section extent")?;
    let (node_header, _) =
        parse_node_header(nodes).map_err(|error| format!("Invalid node section: {error:?}"))?;
    let count = node_header.node_count as usize;
    let tensor_bytes = TensorBufferHeader::total_bytes(count);
    if count == 0
        || tensor_bytes
            .saturating_add(w_len)
            .saturating_add(SUBSTRATE_HEADER_BYTES)
            > host.limits.resource_bytes
    {
        return Err(
            "Substrate extent exceeds production resource limit or the manifold is empty".into(),
        );
    }
    let tensors = (0..count)
        .map(|index| read_node(nodes, index))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Node decode: {error:?}"))?;
    if tensors.iter().any(|node| {
        [
            node.q, node.v, node.w, node.x, node.y, node.z, node.t, node.alpha, node.mu, node.sigma,
        ]
        .iter()
        .any(|lane| !lane.is_finite())
    }) {
        return Err("Substrate contains a non-finite manifold node".into());
    }
    let mut buffer = vec![0; tensor_bytes];
    write_tensor_buffer(&tensors, &mut buffer).map_err(|error| error.to_string())?;
    let combined = compose_substrate(&buffer, weights);
    let view = read_substrate(&combined)?;
    P64TensorIndex::from_p64_with_integrity(view.weights, IntegrityMode::Full)?;
    Ok(Value::Record(BTreeMap::from([
        ("substrate".into(), host.encode("subq", combined)?),
        ("node_count".into(), Value::U64(count as u64)),
    ])))
}

fn weights_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    if let Ok(TypedArtifact::ComputeCartridge {
        kind: CartridgeKind::P64,
        bytes,
    }) = host.workspace.typed(value)
    {
        if bytes.len() <= host.limits.resource_bytes {
            return Ok(bytes);
        }
        return Err("P64 cartridge exceeds production resource budget".into());
    }
    Ok(&load_resource(host, value, "p64")?.bytes)
}

fn car_block(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["car", "cid"])?;
    let input = required(params, "car")?;
    let cid =
        CidSha256::parse(text(required(params, "cid")?)?).map_err(|error| error.to_string())?;
    let size = load_resource(host, input, "car")?.bytes.len();
    if size > 4 * 1024 * 1024 {
        return Err("CAR proof input exceeds block verification budget".into());
    }
    host.charge(size, size.saturating_mul(3))?;
    let bytes = &load_resource(host, input, "car")?.bytes;
    validate_car_header(bytes)?;
    let mut blocks = decode_and_verify_car(bytes).map_err(|error| error.to_string())?;
    let index = blocks
        .iter()
        .position(|block| block.cid == cid)
        .ok_or("Requested CID absent from verified CAR blocks")?;
    let data = blocks.swap_remove(index).data;
    let kind = if WasmSliceQ42Reader::open(&data)
        .ok()
        .is_some_and(|reader| reader.is_unified_volume())
    {
        "q42"
    } else {
        "bin"
    };
    Ok(Value::Record(BTreeMap::from([
        ("artifact".into(), host.encode(kind, data)?),
        ("codec".into(), Value::U64(cid.codec)),
    ])))
}

fn validate_car_header(bytes: &[u8]) -> Result<(), String> {
    use ciborium::Value as Cbor;
    // This admission profile permits at most 64 KiB of CAR header before block proofs.
    let mut length = 0usize;
    for (index, &byte) in bytes.iter().take(3).enumerate() {
        length |= usize::from(byte & 0x7f) << (7 * index);
        if byte & 0x80 != 0 {
            continue;
        }
        if length == 0 || length > 64 * 1024 || (index > 0 && byte == 0) {
            return Err("CAR header length exceeds the canonical bounded profile".into());
        }
        let start = index + 1;
        let mut header = bytes
            .get(start..start + length)
            .ok_or("Truncated CAR header")?;
        let Cbor::Map(fields) = ciborium::from_reader::<Cbor, _>(&mut header)
            .map_err(|error| format!("Invalid CAR header: {error}"))?
        else {
            return Err("CAR header is not a CBOR map".into());
        };
        let version = fields
            .iter()
            .filter_map(|(key, value)| {
                matches!(key, Cbor::Text(key) if key == "version").then_some(value)
            })
            .collect::<Vec<_>>();
        let roots = fields
            .iter()
            .filter(|(key, value)| {
                matches!(key, Cbor::Text(key) if key == "roots") && matches!(value, Cbor::Array(_))
            })
            .count();
        if !header.is_empty()
            || fields.len() != 2
            || roots != 1
            || version.len() != 1
            || !matches!(version[0], Cbor::Integer(n) if u64::try_from(*n).ok() == Some(1))
        {
            return Err("Invalid CARv1 header fields".into());
        }
        return Ok(());
    }
    Err("Truncated or oversized CAR header length".into())
}

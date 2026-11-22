//! Zarr v3 ZipStore export of authored scalar tensor fields.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeSet, sync::{atomic::{AtomicBool, Ordering}, Arc}};

use chronicle_office_codecs::zip::ZipPackage;
use qualia_core_db::{
    bundle::semantic_model::resource_identity_meta,
    container_10d::{
        compute_whole_file_crc32c, crc32c, Container10dHeader, SectionType, TypedFieldRecord,
        TypedPayloadResolver, MANIFOLD_IDENTITY_V2_MAX_FIELDS,
        field_section::{inspect_field_section, FieldInspectionLimits, FIELD_DESCRIPTOR_SIZE,
            FIELD_SECTION_HEADER_SIZE, FIELD_SECTION_MAGIC, RASTER_GRID_METADATA_SIZE},
        manifold_identity_v2::FIELD_KIND_SCALAR,
    },
    domains::geospatial::gis_io::datacube_zarr_netcdf_grib::{format_zarr_chunk_key, ZarrArrayMeta},
};
use serde_json::{json, Value as Json};
use sha2::{Digest, Sha256};

use crate::{
    artifact::FieldLattice, fields, integer, list, num, record, required, text, Args, Artifact,
    AssetHost, Resource, TypedArtifact, Value, WorkspaceEntry,
    qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph,
};

const ARRAY_RANK_MAX: usize = 4;
const DATASET_BYTES_MAX: usize = 64 * 1024 * 1024;
const SOURCE_BYTES_MAX: usize = 128 * 1024 * 1024;
const CHUNK_BYTES_MIN: usize = 4096;
const CHUNK_BYTES_MAX: usize = 128 * 1024 * 1024;
const STORE_KEYS_MAX: usize = 8192;
const SHARD_CHUNKS_MAX: usize = 256;
const WORK_PER_ENCODED_BYTE: usize = 28;
const SEMANTIC_FORMAT: &str = "urn:qualia:interchange:zarr-v3:zipstore";

#[derive(Clone, Copy)]
enum Samples<'a> {
    Float32(&'a [f32]),
    LittleEndian(&'a [u8]),
}

impl Samples<'_> {
    fn word(self, index: usize) -> [u8; 4] {
        match self {
            Self::Float32(samples) => samples[index].to_bits().to_le_bytes(),
            Self::LittleEndian(samples) => samples[4 * index..4 * index + 4].try_into()
                .expect("validated scalar sample bounds"),
        }
    }
}

struct ArraySource<'a> {
    shape: Vec<usize>,
    samples: Samples<'a>,
    digest: [u8; 32],
    field: Option<FieldSource>,
}

enum SourceBacking<'a> {
    Field(Arc<FieldLattice>),
    Retained(&'a Resource),
    Encoded(Arc<Artifact>),
}

struct FieldSource {
    id: Option<u64>,
    index: usize,
    name_hash: u64,
    unit_hash: u64,
}

struct ChunkPlan {
    inner_shape: Vec<usize>,
    outer_shape: Vec<usize>,
    store_grid: Vec<usize>,
    chunks_per_shard: Option<Vec<usize>>,
    inner_cells: usize,
    keys: usize,
    zip_bytes_bound: usize,
}

impl<'host> AssetHost<'host> {
    pub(crate) fn export_cog_zarr_netcdf4(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "source_sha256", "format", "field_id", "field_index", "axes",
            "dense_axis_order", "coordinate_reference_iri", "chunk_size_bytes", "chunk_shape", "shard_shape"])?;
        let format = args.get("format").map(text).transpose()?.unwrap_or("zarr_v3");
        if format != "zarr_v3" {
            return Err("This datacube export supports the zarr_v3 format".into());
        }
        let source = required(args, "source")?;
        let field_index = args.get("field_index").map(|v| integer(v, "field_index")).transpose()?;
        let field_id = args.get("field_id").map(|v| match v {
            Value::U64(id) => Ok(*id),
            Value::I64(id) => u64::try_from(*id).map_err(|_| "FIEL field_id must be an unsigned integer"),
            _ => Err("FIEL field_id must be an unsigned integer"),
        }).transpose()?;
        let axis_order = args.get("dense_axis_order").map(text).transpose()?;
        let requested_digest = args.get("source_sha256").map(text).transpose()?;
        let byte_ceiling = self.limits.resource_bytes.min(DATASET_BYTES_MAX);
        let backing = source_backing(self, source)?;
        let source_bytes = backing.byte_len()?;
        if source_bytes == 0 || source_bytes > self.limits.resource_bytes.min(SOURCE_BYTES_MAX) {
            return Err("Zarr source exceeds its bounded resource extent".into());
        }
        self.charge(source_bytes.saturating_mul(6), 32 * 1024)?;

        let parsed = backing.read(field_id, field_index, axis_order, self.cancel)?;
        let source_digest = parsed.digest.iter().map(|b| format!("{b:02x}")).collect::<String>();
        if requested_digest.is_some_and(|expected| !expected.eq_ignore_ascii_case(&source_digest)) {
            return Err("Zarr source SHA-256 mismatch".into());
        }
        let (names, axes) = axis_metadata(required(args, "axes")?, &parsed.shape)?;
        let crs = args.get("coordinate_reference_iri").map(text).transpose()?;
        if let Some(iri) = crs {
            validate_iri(iri)?;
        }
        let plan = chunk_plan(args, &parsed.shape, byte_ceiling)?;
        let metadata = array_metadata(&parsed, &plan, &names, axes, crs, &source_digest)?;
        let zip_bound = plan.zip_bytes_bound + metadata.len();
        if zip_bound > byte_ceiling {
            return Err("Zarr source and metadata exceed the output archive budget".into());
        }
        self.charge(zip_bound.saturating_mul(WORK_PER_ENCODED_BYTE), zip_bound.saturating_mul(3))?;

        let array_shape = parsed.shape.clone();
        let archive = zip_store(parsed, &plan, metadata, self.cancel)?;
        if archive.len() > zip_bound {
            return Err("Zarr store exceeds its planned byte extent".into());
        }
        let archive_digest = format!("{:x}", Sha256::digest(&archive));
        let archive_iri = format!("urn:sha256:{archive_digest}");
        let mut graph = ReceiptGraph::new(SEMANTIC_FORMAT);
        graph.fact(&archive_iri, "http://www.w3.org/ns/prov#wasDerivedFrom", &format!("urn:sha256:{source_digest}"))?;
        graph.fact(&archive_iri, "http://www.w3.org/1999/02/22-rdf-syntax-ns#type", SEMANTIC_FORMAT)?;
        if let Some(iri) = crs {
            graph.fact(&archive_iri, "urn:qualia:geodesy:declaredCoordinateReference", iri)?;
        }
        let receipt = graph.seal(self.limits.resource_bytes.min(1024 * 1024))?;
        self.charge(0, 0)?;
        if self.workspace.entries.len().saturating_add(2) > self.limits.resources
            || self.workspace.bytes.saturating_add(archive.len()).saturating_add(receipt.len()) > self.limits.total_resource_bytes
        {
            return Err("Zarr store and semantic receipt exceed the remaining workspace budget".into());
        }
        let archive = self.encode("zarr.zip", archive)?;
        let receipt = self.encode("q42", receipt)?;
        Ok(Value::Record(Args::from([
            ("archive".into(), archive),
            ("receipt".into(), receipt),
            ("source_sha256".into(), Value::String(source_digest)),
            ("archive_sha256".into(), Value::String(archive_digest)),
            ("shape".into(), dimensions_value(&array_shape)),
            ("chunk_shape".into(), dimensions_value(&plan.inner_shape)),
            ("shard_shape".into(), plan.chunks_per_shard.as_ref().map_or(Value::Null, |_| dimensions_value(&plan.outer_shape))),
            ("data_objects".into(), Value::U64(plan.keys as u64)),
        ])))
    }
}

fn dimensions_value(shape: &[usize]) -> Value {
    Value::List(shape.iter().map(|&v| Value::U64(v as u64)).collect())
}

fn source_backing<'a>(host: &AssetHost<'a>, source: &Value) -> Result<SourceBacking<'a>, String> {
    if let Value::String(key) = source {
        let resource = host.inputs.iter().find(|resource| &resource.key == key && resource.kind == "10d")
            .ok_or("Zarr export requires a retained .10d FIEL raster")?;
        return Ok(SourceBacking::Retained(resource));
    }
    match host.workspace.get_entry(source)? {
        WorkspaceEntry::Typed(TypedArtifact::FieldLattice(lattice)) => Ok(SourceBacking::Field(lattice.clone())),
        WorkspaceEntry::Legacy(artifact) if matches!(artifact.as_ref(), Artifact::Encoded(r) if r.kind == "10d") => {
            Ok(SourceBacking::Encoded(artifact.clone()))
        }
        _ => Err("Zarr export requires a typed scalar FieldLattice or an encoded .10d FIEL raster".into()),
    }
}

impl SourceBacking<'_> {
    fn byte_len(&self) -> Result<usize, String> {
        if let Self::Field(field) = self {
            if let FieldLattice::DenseF32 { samples, .. } = field.as_ref() {
                return Ok(std::mem::size_of_val(samples.as_slice()));
            }
        }
        self.ten_d_bytes().map(<[u8]>::len)
    }

    fn ten_d_bytes(&self) -> Result<&[u8], String> {
        match self {
            Self::Retained(resource) => return Ok(&resource.bytes),
            Self::Encoded(artifact) => {
                if let Artifact::Encoded(resource) = artifact.as_ref() {
                    return Ok(&resource.bytes);
                }
            }
            Self::Field(field) => {
                if let FieldLattice::TenD(fields) = field.as_ref() {
                    return Ok(fields.as_10d_bytes());
                }
            }
        }
        Err("Zarr export requires a dense field or an admitted .10d FIEL raster".into())
    }

    fn read(&self, field_id: Option<u64>, field_index: Option<usize>, axis_order: Option<&str>,
        cancel: &AtomicBool) -> Result<ArraySource<'_>, String>
    {
        if let Self::Field(field) = self {
            if let FieldLattice::DenseF32 { axes, samples } = field.as_ref() {
                if field_id.is_some() || field_index.is_some() {
                    return Err("field_id and field_index require .10d FIEL input".into());
                }
                return read_dense(axes, samples, axis_order, cancel);
            }
        }
        if axis_order.is_some() {
            return Err("dense_axis_order requires a DenseF32 input".into());
        }
        read_fiel(self.ten_d_bytes()?, field_id, field_index, cancel)
    }
}

fn read_dense<'a>(axes: &[u32], samples: &'a [f32], axis_order: Option<&str>, cancel: &AtomicBool)
    -> Result<ArraySource<'a>, String>
{
    if !(1..=ARRAY_RANK_MAX).contains(&axes.len()) {
        return Err("Dense Zarr export supports rank 1–4".into());
    }
    let axis_order = axis_order.or((axes.len() == 1).then_some("slowest_first"));
    let mut shape: Vec<usize> = axes.iter().map(|&v| v as usize).collect();
    match axis_order {
        Some("fastest_first") => shape.reverse(),
        Some("slowest_first") => {}
        _ => return Err("Declare dense_axis_order as fastest_first or slowest_first for a multidimensional field".into()),
    }
    if shape.contains(&0) || product(&shape)? != samples.len() || samples.len() > DATASET_BYTES_MAX / 4 {
        return Err("Dense field axes disagree with its bounded sample extent".into());
    }
    let mut hash = Sha256::new();
    hash.update(b"qualia:dense-f32:c-order:v1\0");
    hash.update([shape.len() as u8]);
    for axis in &shape {
        hash.update((*axis as u32).to_le_bytes());
    }
    for group in samples.chunks(16_384) {
        check_cancelled(cancel)?;
        for value in group {
            hash.update(value.to_bits().to_le_bytes());
        }
    }
    Ok(ArraySource { shape, samples: Samples::Float32(samples), digest: hash.finalize().into(), field: None })
}

fn read_fiel<'a>(bytes: &'a [u8], field_id: Option<u64>, index: Option<usize>, cancel: &AtomicBool)
    -> Result<ArraySource<'a>, String>
{
    let header = Container10dHeader::parse(bytes).map_err(|error| format!("Zarr .10d header: {error}"))?;
    if compute_whole_file_crc32c(bytes) != header.header_crc32c {
        return Err("Invalid .10d checksum for Zarr source".into());
    }
    check_cancelled(cancel)?;
    let mut scratch = [TypedFieldRecord::default(); MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut fields = scratch;
    let resolver = TypedPayloadResolver::new(bytes, &[], &mut scratch, &mut fields)
        .map_err(|error| format!("Zarr .10d typed fields: {error}"))?;
    let magic = FIELD_SECTION_MAGIC.to_le_bytes();
    let (field, selected_field_id) = if let Some(id) = field_id {
        let selected = resolver.field(id).map_err(|error| error.to_string())?
            .filter(|payload| payload.record.kind == FIELD_KIND_SCALAR && payload.bytes.starts_with(&magic))
            .ok_or("The selected .10d field_id is not a FIEL scalar payload")?;
        (selected.bytes, Some(id))
    } else {
        let mut selected = None;
        for entry in resolver.payloads_of_kind(FIELD_KIND_SCALAR) {
            let entry = entry.map_err(|error| error.to_string())?;
            if entry.bytes.starts_with(&magic) && selected.replace(entry).is_some() {
                return Err("Multiple .10d FIEL payloads require an explicit field_id".into());
            }
        }
        if let Some(entry) = selected {
            (entry.bytes, Some(entry.record.field_id))
        } else {
            let sidecar = resolver.section_payload(SectionType::FieldSidecar).filter(|bytes| bytes.starts_with(&magic))
                .ok_or("The .10d source has no FIEL sidecar or declared scalar FIEL payload")?;
            (sidecar, None)
        }
    };
    let inspection = inspect_field_section(field, FieldInspectionLimits {
        max_payload_bytes: DATASET_BYTES_MAX, max_fields: 1024, max_samples: DATASET_BYTES_MAX / 4,
    }).map_err(|error| error.to_string())?;
    let index = index.or((inspection.fields == 1).then_some(0)).ok_or("Select a FIEL field_index")?;
    if index >= inspection.fields {
        return Err("FIEL field_index exceeds the declared field table".into());
    }
    let at = FIELD_SECTION_HEADER_SIZE + index * FIELD_DESCRIPTOR_SIZE;
    let descriptor = &field[at..at + FIELD_DESCRIPTOR_SIZE];
    let offset = usize::try_from(u64_at(descriptor, 24)).map_err(|_| "FIEL sample offset exceeds the address space")?;
    if offset == 0 {
        return Err("Selected FIEL entry has no continuous scalar raster".into());
    }
    // The native inspection above validates the fixed metadata and full sample extent.
    let width = u32_at(field, offset) as usize;
    let height = u32_at(field, offset + 4) as usize;
    let count = u32_at(field, offset + 8) as usize;
    let first_sample = offset + RASTER_GRID_METADATA_SIZE;
    check_cancelled(cancel)?;
    Ok(ArraySource {
        shape: vec![height, width], samples: Samples::LittleEndian(&field[first_sample..first_sample + count * 4]),
        digest: Sha256::digest(bytes).into(),
        field: Some(FieldSource { id: selected_field_id, index,
            name_hash: u64_at(descriptor, 0), unit_hash: u64_at(descriptor, 8) }),
    })
}

fn axis_metadata(value: &Value, shape: &[usize]) -> Result<(Vec<String>, Vec<Json>), String> {
    let axes = list(value)?;
    if axes.len() != shape.len() {
        return Err("Zarr axes must match rank in slowest-to-fastest order".into());
    }
    let mut unique = BTreeSet::new();
    let mut names = Vec::with_capacity(shape.len());
    let mut metadata = Vec::with_capacity(shape.len());
    for (axis, &length) in axes.iter().zip(shape) {
        let axis = record(axis)?;
        fields(axis, &["name", "origin", "spacing", "unit_iri"])?;
        let name = text(required(axis, "name")?)?;
        if name.is_empty() || name.len() > 128 || name.chars().any(char::is_control) || !unique.insert(name) {
            return Err("Zarr axis names must be distinct and bounded UTF-8 text".into());
        }
        let origin = axis.get("origin").map(num).transpose()?;
        let spacing = axis.get("spacing").map(num).transpose()?;
        if let (Some(origin), Some(spacing)) = (origin, spacing) {
            let final_coordinate = (length - 1) as f64 * spacing + origin;
            if spacing == 0.0 || !final_coordinate.is_finite() {
                return Err("Zarr regular axis coordinates must remain finite with nonzero spacing".into());
            }
        } else if origin.is_some() || spacing.is_some() {
            return Err("A calibrated Zarr axis requires both origin and spacing".into());
        }
        let unit = axis.get("unit_iri").map(text).transpose()?;
        if let Some(iri) = unit {
            validate_iri(iri)?;
        }
        names.push(name.into());
        metadata.push(json!({"name":name, "length":length, "origin":origin, "spacing":spacing, "unit_iri":unit}));
    }
    Ok((names, metadata))
}

fn validate_iri(value: &str) -> Result<(), String> {
    if value.len() > 2048 {
        return Err("Zarr geospatial IRI exceeds its lexical bound".into());
    }
    resource_identity_meta(value).map(|_| ()).map_err(|error| error.to_string())
}

fn product(shape: &[usize]) -> Result<usize, String> {
    shape.iter().try_fold(1usize, |n, &v| n.checked_mul(v).ok_or_else(|| "Zarr shape overflows".into()))
}

fn parse_shape(value: &Value, rank: usize, name: &str) -> Result<Vec<usize>, String> {
    let values = list(value)?;
    if values.len() != rank {
        return Err(format!("Zarr {name} must match the input rank"));
    }
    values.iter().map(|value| integer(value, name).and_then(|v|
        if v > 0 && v <= u32::MAX as usize { Ok(v) } else { Err(format!("Invalid Zarr {name} dimension")) }))
        .collect()
}

fn chunk_plan(args: &Args, shape: &[usize], budget: usize) -> Result<ChunkPlan, String> {
    let limit = args.get("chunk_size_bytes").map(|v| integer(v, "chunk_size_bytes"))
        .transpose()?.unwrap_or(1024 * 1024);
    if !(CHUNK_BYTES_MIN..=CHUNK_BYTES_MAX).contains(&limit) {
        return Err("Zarr chunk_size_bytes must lie in 4096..=134217728".into());
    }
    let inner_shape = if let Some(value) = args.get("chunk_shape") {
        parse_shape(value, shape.len(), "chunk_shape")?
    } else {
        let mut cells_left = limit / 4;
        let mut chunk = vec![1; shape.len()];
        for (chunk, &axis) in chunk.iter_mut().zip(shape).rev() {
            *chunk = axis.min(cells_left);
            cells_left /= *chunk;
        }
        chunk
    };
    let inner_cells = product(&inner_shape)?;
    if inner_cells > limit / 4 || inner_cells > budget / 4 {
        return Err("Zarr inner chunk exceeds its sample budget".into());
    }
    let outer_shape = args.get("shard_shape").map(|v| parse_shape(v, shape.len(), "shard_shape"))
        .transpose()?.unwrap_or_else(|| inner_shape.clone());
    let chunks_per_shard = if args.contains_key("shard_shape") {
        let counts: Vec<usize> = outer_shape.iter().zip(&inner_shape).map(|(&outer, &inner)| {
            if outer % inner != 0 { Err("Each Zarr shard axis must be a multiple of its chunk axis".into()) }
            else { Ok(outer / inner) }
        }).collect::<Result<_, String>>()?;
        if product(&counts)? > SHARD_CHUNKS_MAX {
            return Err("Zarr shard exceeds the 256 inner-chunk limit".into());
        }
        Some(counts)
    } else { None };
    let store_grid: Vec<usize> = shape.iter().zip(&outer_shape).map(|(&axis, &extent)| axis.div_ceil(extent)).collect();
    let keys = product(&store_grid)?;
    if keys >= STORE_KEYS_MAX {
        return Err("Zarr archive exceeds its 8192-object budget including metadata".into());
    }
    let positions = chunks_per_shard.as_deref().map(product).transpose()?.unwrap_or(1);
    let shard_index_bytes = if chunks_per_shard.is_some() { positions * 16 + 4 } else { 0 };
    let zip_bytes_bound = inner_cells.checked_mul(4 * positions)
        .and_then(|bytes| bytes.checked_add(shard_index_bytes + 256))
        .and_then(|bytes| bytes.checked_mul(keys))
        .and_then(|bytes| bytes.checked_add(256))
        .filter(|&bytes| bytes <= budget)
        .ok_or("Zarr padding, directory and shard indexes exceed the archive budget")?;
    Ok(ChunkPlan { inner_shape, outer_shape, store_grid, chunks_per_shard, inner_cells, keys, zip_bytes_bound })
}

fn array_metadata(source: &ArraySource<'_>, plan: &ChunkPlan, names: &[String], axes: Vec<Json>,
    crs: Option<&str>, source_digest: &str) -> Result<Vec<u8>, String>
{
    let bytes_codec = json!({"name":"bytes", "configuration":{"endian":"little"}});
    let codecs = if plan.chunks_per_shard.is_some() {
        json!([{"name":"sharding_indexed", "configuration":{
            "chunk_shape":plan.inner_shape, "codecs":[bytes_codec.clone()],
            "index_codecs":[bytes_codec, {"name":"crc32c"}], "index_location":"end"
        }}])
    } else { json!([bytes_codec]) };
    let field = source.field.as_ref().map(|field| json!({"field_index":field.index,
        "field_id_hex":field.id.map(|id| format!("{id:016x}")),
        "name_q_hash_hex":format!("{:016x}", field.name_hash), "unit_q_hash_hex":format!("{:016x}", field.unit_hash)}));
    serde_json::to_vec(&json!({
        "zarr_format":3, "node_type":"array", "shape":source.shape, "data_type":"float32",
        "chunk_grid":{"name":"regular", "configuration":{"chunk_shape":plan.outer_shape}},
        "chunk_key_encoding":{"name":"default", "configuration":{"separator":"/"}},
        "fill_value":0.0, "codecs":codecs, "dimension_names":names,
        "attributes":{"qualia_axes":axes, "qualia_declared_crs_iri":crs,
            "qualia_source_sha256":source_digest, "qualia_fiel":field}
    })).map_err(|error| error.to_string())
}

fn unpack_index(mut linear: usize, shape: &[usize]) -> [usize; ARRAY_RANK_MAX] {
    let mut coordinates = [0; ARRAY_RANK_MAX];
    for (i, &extent) in shape.iter().enumerate().rev() {
        coordinates[i] = linear % extent;
        linear /= extent;
    }
    coordinates
}

fn zip_store(source: ArraySource<'_>, plan: &ChunkPlan, metadata: Vec<u8>, cancel: &AtomicBool) -> Result<Vec<u8>, String> {
    let rank = source.shape.len();
    let mut store = ZipPackage::new();
    store.insert("zarr.json", metadata);
    let mut native_meta = ZarrArrayMeta { zarr_format: 3, shape: [0; 4], chunks: [0; 4], fill_value: 0.0, little_endian: true };
    native_meta.shape[..rank].copy_from_slice(&source.shape);
    native_meta.chunks[..rank].copy_from_slice(&plan.outer_shape);
    for key_index in 0..plan.keys {
        check_cancelled(cancel)?;
        let grid_pos = unpack_index(key_index, &plan.store_grid);
        let mut start = [0; ARRAY_RANK_MAX];
        for i in 0..rank {
            start[i] = grid_pos[i] * plan.outer_shape[i];
        }
        let payload = if let Some(inner_grid) = &plan.chunks_per_shard {
            let mut shard = Vec::new();
            let entries = product(inner_grid)?;
            let mut index_bytes = Vec::with_capacity(entries * 16);
            for linear in 0..entries {
                let inner_pos = unpack_index(linear, inner_grid);
                let mut inner_start = start;
                for i in 0..rank {
                    inner_start[i] += inner_pos[i] * plan.inner_shape[i];
                }
                let (offset, size) = if (0..rank).any(|i| inner_start[i] >= source.shape[i]) {
                    (u64::MAX, u64::MAX)
                } else {
                    let offset = shard.len() as u64;
                    append_chunk(&source, plan, inner_start, &mut shard, cancel)?;
                    (offset, shard.len() as u64 - offset)
                };
                index_bytes.extend_from_slice(&offset.to_le_bytes());
                index_bytes.extend_from_slice(&size.to_le_bytes());
            }
            let index_crc = crc32c(&index_bytes);
            shard.extend_from_slice(&index_bytes);
            shard.extend_from_slice(&index_crc.to_le_bytes());
            shard
        } else {
            let mut chunk = Vec::with_capacity(plan.inner_cells * 4);
            append_chunk(&source, plan, start, &mut chunk, cancel)?;
            chunk
        };
        let key = format_zarr_chunk_key(&native_meta, grid_pos, '/', 64)
            .map_err(|e| format!("Zarr chunk key: {e:?}"))?;
        store.insert(key, payload);
    }
    check_cancelled(cancel)?;
    Ok(store.write_to_bytes())
}

fn append_chunk(source: &ArraySource<'_>, plan: &ChunkPlan, start: [usize; ARRAY_RANK_MAX],
    bytes: &mut Vec<u8>, cancel: &AtomicBool) -> Result<(), String>
{
    for linear in 0..plan.inner_cells {
        if linear % 16_384 == 0 {
            check_cancelled(cancel)?;
        }
        let local = unpack_index(linear, &plan.inner_shape);
        let mut sample = Some(0usize);
        for (axis, &extent) in source.shape.iter().enumerate() {
            let coordinate = start[axis] + local[axis];
            if coordinate >= extent {
                sample = None;
                break;
            }
            sample = sample.map(|sample| sample * extent + coordinate);
        }
        bytes.extend_from_slice(&sample.map_or([0; 4], |sample| source.samples.word(sample)));
    }
    Ok(())
}

fn check_cancelled(cancel: &AtomicBool) -> Result<(), String> {
    if cancel.load(Ordering::Acquire) { Err("Production cancelled".into()) } else { Ok(()) }
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(bytes[at..at + 4].try_into().expect("validated FIEL header"))
}

fn u64_at(bytes: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(bytes[at..at + 8].try_into().expect("validated FIEL descriptor"))
}

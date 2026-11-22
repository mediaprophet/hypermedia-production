//! Bounded Zarr v3 ZipStore re-admission for the #233/#40/#163 round trip.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use chronicle_office_codecs::zip::minizip::crc32;
use qualia_core_db::{
    container_10d::crc32c,
    domains::geospatial::gis_io::datacube_zarr_netcdf_grib::{
        ZarrArrayMeta, format_zarr_chunk_key,
    },
};
use serde_json::Value as Json;
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Value, fields, required, text};

use super::{lattice, receipt};

pub(super) const MAX_ARCHIVE: usize = 64 * 1024 * 1024;
const MAX_OBJECTS: usize = 8192;
const MAX_METADATA: usize = 64 * 1024;
const MAX_SHARD_CHUNKS: usize = 256;

struct Array {
    shape: Vec<usize>,
    inner: Vec<usize>,
    outer: Vec<usize>,
    outer_grid: Vec<usize>,
    shard_grid: Option<Vec<usize>>,
    fill: f32,
    little: bool,
    separator: char,
}

impl AssetHost<'_> {
    pub(crate) fn decode_zarr_v3_zip(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["source", "source_sha256"])?;
        let source = required(args, "source")?;
        let expected = args.get("source_sha256").map(text).transpose()?;
        let (crs, mut out) = decode(self, source, expected)?;
        if let Some(iri) = crs {
            out.insert("crs_iri".into(), Value::String(iri));
        }
        Ok(Value::Record(out))
    }
}

pub(super) fn decode(
    host: &mut AssetHost<'_>,
    source: &Value,
    expected: Option<&str>,
) -> Result<(Option<String>, Args), String> {
    let input = super::gis::resource(host, source)?;
    let extent = input.bytes.len();
    if input.kind != "zarr.zip"
        || extent == 0
        || extent > host.limits.resource_bytes.min(MAX_ARCHIVE)
    {
        return Err("Expected a bounded retained zarr.zip resource".into());
    }
    host.charge(extent.saturating_mul(11), MAX_OBJECTS * 192)?;
    let metadata_bytes;
    let model;
    let archive_hash;
    let crs;
    let cells;
    {
        let bytes = &super::gis::resource(host, source)?.bytes;
        archive_hash = format!("{:x}", Sha256::digest(bytes));
        if expected.is_some_and(|expected| !expected.eq_ignore_ascii_case(&archive_hash)) {
            return Err("Zarr ZIP source SHA-256 mismatch".into());
        }
        let archive = members(bytes, host.cancel, false)?;
        let raw_metadata = metadata(&archive)?;
        metadata_bytes = archive["zarr.json"].to_vec();
        model = parse_array(&raw_metadata)?;
        crs = raw_metadata["attributes"]["qualia_declared_crs_iri"]
            .as_str()
            .map(str::to_owned);
        if let Some(ref iri) = crs {
            if iri.len() > 2048 {
                return Err("Zarr declared CRS exceeds its metadata limit".into());
            }
            qualia_core_db::bundle::semantic_model::resource_identity_meta(iri)
                .map_err(|e| e.to_string())?;
        }
        cells = product(&model.shape)?;
    }
    let field_bytes = cells
        .checked_mul(4)
        .filter(|&n| n <= host.limits.resource_bytes.min(MAX_ARCHIVE))
        .ok_or("Zarr decoded field exceeds the resource byte limit")?;
    host.charge(
        cells.saturating_mul(36),
        field_bytes.saturating_add(MAX_OBJECTS * 192),
    )?;
    let (samples, dense_hash) = {
        let bytes = &super::gis::resource(host, source)?.bytes;
        let archive = members(bytes, host.cancel, true)?;
        let raw_metadata = metadata(&archive)?;
        let samples = model.samples(&archive, cells, host.cancel)?;
        let dense_hash = dense_digest(&model.shape, &samples, host.cancel)?;
        let attributes = &raw_metadata["attributes"];
        if attributes.get("qualia_fiel").is_some_and(Json::is_null)
            && attributes["qualia_source_sha256"]
                .as_str()
                .is_some_and(|expected| !expected.eq_ignore_ascii_case(&dense_hash))
        {
            return Err("Zarr samples fail the original DenseF32 source digest".into());
        }
        (samples, dense_hash)
    };

    let archive_iri = format!("urn:sha256:{archive_hash}");
    let digest_iri = format!("urn:sha256:{dense_hash}");
    let mut facts = vec![(
        digest_iri.as_str(),
        "http://www.w3.org/ns/prov#wasDerivedFrom",
        archive_iri.as_str(),
    )];
    if let Some(ref iri) = crs {
        facts.push((
            &digest_iri,
            "urn:qualia:geodesy:declaredCoordinateReference",
            iri,
        ));
    }
    let shape = model.shape.iter().map(|&n| n as u32).collect::<Vec<_>>();
    let axes = Value::List(shape.iter().map(|&n| Value::U64(u64::from(n))).collect());
    let field = lattice(host, shape, samples)?;
    let graph = receipt(host, "zarr-v3-zipstore-reassembly", &facts)?;
    Ok((
        crs,
        BTreeMap::from([
            ("field".into(), field),
            ("axes".into(), axes),
            ("axis_order".into(), Value::String("slowest_first".into())),
            ("metadata".into(), host.encode("json", metadata_bytes)?),
            ("dense_sha256".into(), Value::String(dense_hash)),
            ("archive_sha256".into(), Value::String(archive_hash)),
            ("receipt".into(), graph),
        ]),
    ))
}

fn word<const N: usize>(bytes: &[u8], at: usize) -> Result<[u8; N], String> {
    let end = at.checked_add(N).ok_or("Storage word extent overflows")?;
    bytes
        .get(at..end)
        .and_then(|lane| lane.try_into().ok())
        .ok_or_else(|| "Truncated storage word".into())
}

fn u16_at(bytes: &[u8], at: usize) -> Result<usize, String> {
    Ok(usize::from(u16::from_le_bytes(word(bytes, at)?)))
}

fn u32_at(bytes: &[u8], at: usize) -> Result<usize, String> {
    Ok(u32::from_le_bytes(word(bytes, at)?) as usize)
}

fn members<'a>(
    bytes: &'a [u8],
    cancel: &std::sync::atomic::AtomicBool,
    verify_crc: bool,
) -> Result<BTreeMap<&'a str, &'a [u8]>, String> {
    let end = bytes.len().checked_sub(22).ok_or("Truncated Zarr ZIP")?;
    let eocd = &bytes[end..];
    let count = u16_at(eocd, 10)?;
    let directory_start = u32_at(eocd, 16)?;
    if !eocd.starts_with(b"PK\x05\x06")
        || u32_at(eocd, 4)? != 0
        || u16_at(eocd, 8)? != count
        || u16_at(eocd, 20)? != 0
        || !(1..=MAX_OBJECTS).contains(&count)
        || directory_start.checked_add(u32_at(eocd, 12)?) != Some(end)
    {
        return Err("Expected a bounded single-disk stored ZIP without a comment".into());
    }
    let mut directory = &bytes[directory_start..end];
    let mut out = BTreeMap::new();
    let mut spans = Vec::with_capacity(count);
    let mut checks = Vec::new();
    for _ in 0..count {
        cancelled(cancel)?;
        if directory.len() < 46 || !directory.starts_with(b"PK\x01\x02") {
            return Err("Invalid Zarr ZIP central entry".into());
        }
        let name_len = u16_at(directory, 28)?;
        let extent = 46 + name_len + u16_at(directory, 30)? + u16_at(directory, 32)?;
        let entry = directory
            .get(..extent)
            .ok_or("Truncated Zarr ZIP central entry")?;
        directory = &directory[extent..];
        let flags = u16_at(entry, 8)?;
        let length = u32_at(entry, 24)?;
        if !(1..=128).contains(&name_len)
            || flags & !0x0800 != 0
            || u16_at(entry, 10)? != 0
            || u32_at(entry, 20)? != length
            || u16_at(entry, 34)? != 0
        {
            return Err(
                "Zarr ZIP members must be uncompressed, single-disk and unencrypted".into(),
            );
        }
        let name = std::str::from_utf8(&entry[46..46 + name_len])
            .map_err(|_| "Zarr ZIP member key is not UTF-8")?;
        let local_at = u32_at(entry, 42)?;
        let local = bytes
            .get(local_at..directory_start)
            .ok_or("Zarr ZIP member offset outside data region")?;
        if !local.starts_with(b"PK\x03\x04")
            || u16_at(local, 6)? != flags
            || u16_at(local, 8)? != 0
            || u32_at(local, 14)? != u32_at(entry, 16)?
            || u32_at(local, 18)? != length
            || u32_at(local, 22)? != length
            || u16_at(local, 26)? != name_len
            || local.get(30..30 + name_len) != Some(name.as_bytes())
        {
            return Err("Zarr ZIP local entry disagrees with central data".into());
        }
        let start = 30 + name_len + u16_at(local, 28)?;
        let data_end = start
            .checked_add(length)
            .ok_or("Zarr ZIP member length overflows")?;
        let data = local
            .get(start..data_end)
            .ok_or("Truncated Zarr ZIP member")?;
        if out.insert(name, data).is_some() {
            return Err("Zarr ZIP has a duplicate member".into());
        }
        if verify_crc {
            checks.push((data, u32_at(entry, 16)? as u32));
        }
        spans.push((local_at, local_at + data_end));
    }
    spans.sort_unstable();
    if !directory.is_empty() || spans.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return Err("Zarr ZIP member spans overlap or directory length is inconsistent".into());
    }
    for (data, expected) in checks {
        cancelled(cancel)?;
        if crc32(data) != expected {
            return Err("Zarr ZIP member failed CRC-32 verification".into());
        }
    }
    Ok(out)
}

fn metadata(members: &BTreeMap<&str, &[u8]>) -> Result<Json, String> {
    let bytes = members
        .get("zarr.json")
        .ok_or("Zarr store has no root zarr.json")?;
    if bytes.len() > MAX_METADATA {
        return Err("Zarr array metadata exceeds 64 KiB".into());
    }
    serde_json::from_slice(bytes).map_err(|e| format!("Zarr metadata: {e}"))
}

fn sizes(value: &Json) -> Result<Vec<usize>, String> {
    let axes = value
        .as_array()
        .filter(|a| (1..=4).contains(&a.len()))
        .ok_or("Zarr shape requires rank 1..=4")?;
    axes.iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| u32::try_from(n).ok())
                .filter(|&n| n > 0)
                .map(|n| n as usize)
                .ok_or_else(|| "Zarr dimension must be a positive u32".into())
        })
        .collect()
}

fn product(shape: &[usize]) -> Result<usize, String> {
    shape.iter().try_fold(1usize, |product, n| {
        product
            .checked_mul(*n)
            .ok_or_else(|| "Zarr dimension product overflows".into())
    })
}

fn little(codec: &Json) -> Result<bool, String> {
    if codec["name"] != "bytes" {
        return Err("Expected Zarr bytes codec".into());
    }
    match codec["configuration"]["endian"].as_str() {
        Some("little") => Ok(true),
        Some("big") => Ok(false),
        _ => Err("Expected explicit Zarr bytes endianness".into()),
    }
}

fn parse_array(doc: &Json) -> Result<Array, String> {
    if doc["zarr_format"].as_u64() != Some(3)
        || doc["node_type"] != "array"
        || doc["data_type"] != "float32"
        || doc["chunk_grid"]["name"] != "regular"
        || doc["chunk_key_encoding"]["name"] != "default"
        || doc
            .get("storage_transformers")
            .is_some_and(|v| !v.as_array().is_some_and(Vec::is_empty))
    {
        return Err("Expected a regular Zarr v3 root float32 array with default chunk keys".into());
    }
    let separator = match doc["chunk_key_encoding"]["configuration"]["separator"]
        .as_str()
        .unwrap_or("/")
    {
        "/" => '/',
        "." => '.',
        _ => return Err("Unsupported Zarr chunk key separator".into()),
    };
    let shape = sizes(&doc["shape"])?;
    let outer = sizes(&doc["chunk_grid"]["configuration"]["chunk_shape"])?;
    if outer.len() != shape.len() {
        return Err("Zarr shape and regular chunk rank disagree".into());
    }
    let fill = doc["fill_value"]
        .as_f64()
        .filter(|n| n.abs() <= f32::MAX as f64)
        .ok_or("Expected a finite float32 Zarr fill value")? as f32;
    let codecs = doc["codecs"]
        .as_array()
        .ok_or("Zarr metadata requires codecs")?;
    let [codec] = codecs.as_slice() else {
        return Err("Expected one compatible Zarr array codec".into());
    };
    let (inner, shard_grid, little) = if codec["name"] == "sharding_indexed" {
        let config = &codec["configuration"];
        let index = config["index_codecs"]
            .as_array()
            .ok_or("Zarr shard index requires codecs")?;
        let inner_codecs = config["codecs"]
            .as_array()
            .ok_or("Zarr shard requires inner codecs")?;
        let [data_codec] = inner_codecs.as_slice() else {
            return Err("Expected one Zarr shard data codec".into());
        };
        let [index_codec, checksum] = index.as_slice() else {
            return Err("Expected bytes and CRC32C shard index codecs".into());
        };
        if config["index_location"] != "end"
            || !little(index_codec)?
            || checksum["name"] != "crc32c"
        {
            return Err("Expected a CRC32C-protected little-endian shard index at the end".into());
        }
        let inner = sizes(&config["chunk_shape"])?;
        if inner.len() != outer.len() || outer.iter().zip(&inner).any(|(&a, &b)| a % b != 0) {
            return Err(
                "Zarr shard axes must be positive multiples of the corresponding inner chunk"
                    .into(),
            );
        }
        let shard_grid: Vec<_> = outer.iter().zip(&inner).map(|(&a, &b)| a / b).collect();
        if product(&shard_grid)? > MAX_SHARD_CHUNKS {
            return Err("Zarr shard exceeds the 256-inner-chunk limit".into());
        }
        (inner, Some(shard_grid), little(data_codec)?)
    } else {
        (outer.clone(), None, little(codec)?)
    };
    let outer_grid: Vec<_> = shape
        .iter()
        .zip(&outer)
        .map(|(&n, &step)| n.div_ceil(step))
        .collect();
    if product(&outer_grid)? >= MAX_OBJECTS
        || product(&inner)?
            .checked_mul(4)
            .is_none_or(|n| n > MAX_ARCHIVE)
    {
        return Err("Zarr regular chunk grid exceeds the bounded reshape profile".into());
    }
    Ok(Array {
        shape,
        inner,
        outer,
        outer_grid,
        shard_grid,
        fill,
        little,
        separator,
    })
}

impl Array {
    fn samples(
        &self,
        members: &BTreeMap<&str, &[u8]>,
        count: usize,
        cancel: &std::sync::atomic::AtomicBool,
    ) -> Result<Vec<f32>, String> {
        let rank = self.shape.len();
        let mut native = ZarrArrayMeta {
            zarr_format: 3,
            shape: [0; 4],
            chunks: [0; 4],
            fill_value: self.fill,
            little_endian: self.little,
        };
        native.shape[..rank].copy_from_slice(&self.shape);
        native.chunks[..rank].copy_from_slice(&self.outer);
        let mut samples = vec![self.fill; count];
        let chunk_bytes = product(&self.inner)? * 4;
        let mut used = 1;
        for index in 0..product(&self.outer_grid)? {
            cancelled(cancel)?;
            let coord = unravel(index, &self.outer_grid);
            let key = format_zarr_chunk_key(&native, coord, self.separator, 128)
                .map_err(|e| format!("Zarr chunk key: {e:?}"))?;
            let Some(bytes) = members.get(key.as_str()) else {
                continue;
            };
            used += 1;
            let mut start = [0; 4];
            for i in 0..rank {
                start[i] = coord[i] * self.outer[i];
            }
            if let Some(grid) = &self.shard_grid {
                let lanes = shard_lanes(bytes, product(grid)?, chunk_bytes)?;
                for (index, lane) in lanes.into_iter().enumerate() {
                    let Some(data) = lane else {
                        continue;
                    };
                    let chunk_coord = unravel(index, grid);
                    let mut base = start;
                    for i in 0..rank {
                        base[i] += chunk_coord[i] * self.inner[i];
                    }
                    self.copy_chunk(data, base, &mut samples, cancel)?;
                }
            } else {
                if bytes.len() != chunk_bytes {
                    return Err("Zarr chunk byte count disagrees with its declared extent".into());
                }
                self.copy_chunk(bytes, start, &mut samples, cancel)?;
            }
        }
        if used != members.len() {
            return Err("Zarr ZIP contains keys outside the supported one-array store".into());
        }
        Ok(samples)
    }

    fn copy_chunk(
        &self,
        data: &[u8],
        start: [usize; 4],
        target: &mut [f32],
        cancel: &std::sync::atomic::AtomicBool,
    ) -> Result<(), String> {
        let rank = self.shape.len();
        let clipped: [usize; 4] = std::array::from_fn(|i| {
            if i < rank {
                self.inner[i].min(self.shape[i].saturating_sub(start[i]))
            } else {
                1
            }
        });
        for at in 0..product(&clipped[..rank])? {
            if at % 16_384 == 0 {
                cancelled(cancel)?;
            }
            let local = unravel(at, &clipped[..rank]);
            let mut source_index = 0;
            let mut target_index = 0;
            for (axis, &extent) in self.shape.iter().enumerate() {
                source_index = source_index * self.inner[axis] + local[axis];
                target_index = target_index * extent + start[axis] + local[axis];
            }
            let word = word(data, source_index * 4)?;
            target[target_index] = if self.little {
                f32::from_le_bytes(word)
            } else {
                f32::from_be_bytes(word)
            };
        }
        Ok(())
    }
}

fn shard_lanes(
    bytes: &[u8],
    entries: usize,
    chunk_bytes: usize,
) -> Result<Vec<Option<&[u8]>>, String> {
    let index_at = bytes
        .len()
        .checked_sub(entries * 16 + 4)
        .ok_or("Zarr shard index is truncated")?;
    let (payload, index) = bytes.split_at(index_at);
    let (index, checksum) = index.split_at(entries * 16);
    if crc32c(index) != u32::from_le_bytes(word(checksum, 0)?) {
        return Err("Zarr shard index CRC32C failed".into());
    }
    let mut lanes = Vec::with_capacity(entries);
    let mut extents = Vec::new();
    for row in index.chunks_exact(16) {
        let offset = u64::from_le_bytes(word(row, 0)?);
        let length = u64::from_le_bytes(word(row, 8)?);
        if (offset, length) == (u64::MAX, u64::MAX) {
            lanes.push(None);
            continue;
        }
        let start =
            usize::try_from(offset).map_err(|_| "Zarr shard offset exceeds address space")?;
        if length != chunk_bytes as u64 {
            return Err("Zarr shard inner chunk has an invalid length".into());
        }
        let end = start
            .checked_add(chunk_bytes)
            .ok_or("Zarr shard inner extent overflows")?;
        lanes.push(Some(payload.get(start..end).ok_or(
            "Zarr shard inner chunk overlaps the index or leaves its extent",
        )?));
        extents.push((start, end));
    }
    extents.sort_unstable();
    if extents.windows(2).any(|pair| pair[0].1 > pair[1].0) {
        return Err("Zarr shard inner byte ranges overlap".into());
    }
    Ok(lanes)
}

fn unravel(mut n: usize, shape: &[usize]) -> [usize; 4] {
    let mut coord = [0; 4];
    for (i, &axis) in shape.iter().enumerate().rev() {
        coord[i] = n % axis;
        n /= axis;
    }
    coord
}

fn dense_digest(
    shape: &[usize],
    samples: &[f32],
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<String, String> {
    let mut hash = Sha256::new();
    hash.update(b"qualia:dense-f32:c-order:v1\0");
    hash.update([shape.len() as u8]);
    for &axis in shape {
        hash.update((axis as u32).to_le_bytes());
    }
    for group in samples.chunks(16_384) {
        cancelled(cancel)?;
        for value in group {
            hash.update(value.to_bits().to_le_bytes());
        }
    }
    Ok(format!("{:x}", hash.finalize()))
}

fn cancelled(cancel: &std::sync::atomic::AtomicBool) -> Result<(), String> {
    if cancel.load(std::sync::atomic::Ordering::Acquire) {
        Err("Production cancelled".into())
    } else {
        Ok(())
    }
}

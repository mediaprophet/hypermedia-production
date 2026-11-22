//! Retained raster and LiDAR ingestion through Qualia's native readers.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use qualia_core_db::domains::geospatial::{
    decode_geotiff_raster_into,
    gis_io::lidar_copc_m64::{decode_las_point_format0, parse_las_header},
};
use serde_json::json;
use sha2::{Digest, Sha256};

use super::{lattice, receipt};
use crate::{Args, AssetHost, Resource, Value, fields, integer, required, text};

const RASTER_CELLS_MAX: usize = 1_048_576;
const POINTS_MAX: usize = 32_768;
const SOURCE_BYTES_MAX: usize = 32 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn ingest_gis_corpus(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "source",
                "source_crs_epsg",
                "target_crs_epsg",
                "source_sha256",
            ],
        )?;
        let source = required(args, "source")?;
        let target = epsg(required(args, "target_crs_epsg")?)?;
        let declared = args.get("source_crs_epsg").map(epsg).transpose()?;
        let input = resource(self, source)?;
        let size = input.bytes.len();
        let kind = input.kind.clone();
        let format_limit = if kind == "zarr.zip" {
            super::zarr::MAX_ARCHIVE
        } else {
            SOURCE_BYTES_MAX
        };
        if size == 0 || size > self.limits.resource_bytes.min(format_limit) {
            return Err("Retained GIS source exceeds the input byte budget or is empty".into());
        }
        self.charge(size.saturating_mul(4), 0)?;
        let digest = format!("{:x}", Sha256::digest(&resource(self, source)?.bytes));
        if args
            .get("source_sha256")
            .map(text)
            .transpose()?
            .is_some_and(|expected| !expected.eq_ignore_ascii_case(&digest))
        {
            return Err("Retained GIS source SHA-256 mismatch".into());
        }

        let (source_epsg, mut result) = match kind.as_str() {
            "geotiff" => geotiff(self, source, declared, target)?,
            "zarr.zip" => zarr(self, source, declared, target)?,
            "netcdf" => netcdf(self, source, declared, target)?,
            "las" => las(self, source, declared, target)?,
            _ => {
                return Err(
                    "GIS ingestion supports retained geotiff, zarr.zip, netcdf and las resources"
                        .into(),
                );
            }
        };
        let source_iri = format!("urn:sha256:{digest}");
        let dataset = format!("urn:qualia:gis:{kind}:{source_epsg}:{target}:{digest}");
        let source_crs = epsg_iri(source_epsg);
        let target_crs = epsg_iri(target);
        let receipt = receipt(
            self,
            "urn:qualia:geodesy:ingest-gis-corpus",
            &[
                (
                    &dataset,
                    "http://www.w3.org/ns/prov#wasDerivedFrom",
                    &source_iri,
                ),
                (
                    &dataset,
                    "urn:qualia:geodesy:sourceCoordinateReference",
                    &source_crs,
                ),
                (
                    &dataset,
                    "urn:qualia:geodesy:declaredCoordinateReference",
                    &target_crs,
                ),
            ],
        )?;
        result.extend([
            ("source_sha256".into(), Value::String(digest)),
            ("source_crs_epsg".into(), Value::U64(u64::from(source_epsg))),
            ("target_crs_epsg".into(), Value::U64(u64::from(target))),
            ("receipt".into(), receipt),
        ]);
        Ok(Value::Record(result))
    }
}

pub(super) fn resource<'a>(
    host: &'a AssetHost<'_>,
    source: &Value,
) -> Result<&'a Resource, String> {
    if let Value::String(key) = source {
        host.inputs
            .iter()
            .find(|input| &input.key == key)
            .ok_or("Missing retained GIS source".into())
    } else {
        host.workspace.encoded(source)
    }
}

fn zarr(
    host: &mut AssetHost<'_>,
    source: &Value,
    declared: Option<u32>,
    target: u32,
) -> Result<(u32, Args), String> {
    let (crs_iri, mut out) = super::zarr::decode(host, source, None)?;
    let crs = if let Some(iri) = crs_iri {
        let code = [
            "http://www.opengis.net/def/crs/EPSG/0/",
            "https://www.opengis.net/def/crs/EPSG/0/",
            "urn:ogc:def:crs:EPSG::",
        ]
        .iter()
        .find_map(|prefix| iri.strip_prefix(*prefix))
        .and_then(|tail| tail.parse::<u32>().ok())
        .filter(|code| (1..32_767).contains(code))
        .ok_or(
            "GIS Zarr metadata must declare an EPSG CRS IRI to compare against target_crs_epsg",
        )?;
        out.insert("crs_iri".into(), Value::String(iri));
        out.insert("crs_origin".into(), Value::String("zarr_metadata".into()));
        code
    } else {
        out.insert(
            "crs_origin".into(),
            Value::String("authored_declaration".into()),
        );
        declared.ok_or(
            "GIS Zarr ingestion requires a declared source EPSG code when metadata has no CRS",
        )?
    };
    same_raster_crs(crs, declared, target)?;
    if let Some(receipt) = out.remove("receipt") {
        out.insert("roundtrip_receipt".into(), receipt);
    }
    Ok((crs, out))
}

fn epsg(value: &Value) -> Result<u32, String> {
    let code = integer(value, "EPSG code")?;
    if !(1..32_767).contains(&code) {
        return Err("Invalid or user-defined EPSG code".into());
    }
    Ok(code as u32)
}

fn epsg_iri(code: u32) -> String {
    format!("http://www.opengis.net/def/crs/EPSG/0/{code}")
}

fn same_raster_crs(actual: u32, declared: Option<u32>, target: u32) -> Result<(), String> {
    if declared.is_some_and(|code| code != actual) {
        return Err("Declared source CRS disagrees with the native raster CRS".into());
    }
    if actual != target {
        return Err("Raster reprojection requires an explicit resampling grid and kernel".into());
    }
    Ok(())
}

struct TiffEntry {
    datatype: u16,
    count: u32,
    value: [u8; 4],
}

impl TiffEntry {
    fn scalar(&self) -> Option<u32> {
        if self.count != 1 {
            return None;
        }
        match self.datatype {
            3 => Some(u32::from(u16::from_le_bytes([
                self.value[0],
                self.value[1],
            ]))),
            4 => Some(u32::from_le_bytes(self.value)),
            _ => None,
        }
    }

    fn span<'a>(&self, bytes: &'a [u8], word_bytes: usize) -> Option<&'a [u8]> {
        let offset = u32::from_le_bytes(self.value) as usize;
        let length = (self.count as usize).checked_mul(word_bytes)?;
        if length <= self.value.len() {
            return None;
        }
        bytes.get(offset..offset.checked_add(length)?)
    }
}

fn tiff_entries(bytes: &[u8]) -> Result<BTreeMap<u16, TiffEntry>, String> {
    if !bytes.starts_with(b"II\x2a\x00") {
        return Err("GeoTIFF requires a little-endian classic TIFF header".into());
    }
    let offset = u32::from_le_bytes(
        bytes
            .get(4..8)
            .ok_or("Truncated TIFF header")?
            .try_into()
            .map_err(|_| "Truncated TIFF header")?,
    ) as usize;
    let remaining = bytes.get(offset..).ok_or("TIFF IFD offset outside file")?;
    let count = u16::from_le_bytes(
        remaining
            .get(..2)
            .ok_or("Truncated TIFF IFD")?
            .try_into()
            .map_err(|_| "Truncated TIFF IFD")?,
    ) as usize;
    if offset < 8 || count == 0 || count > 256 {
        return Err("Invalid or oversized TIFF IFD".into());
    }
    let mut out = BTreeMap::new();
    for entry in remaining
        .get(2..2 + 12 * count)
        .ok_or("Truncated TIFF IFD")?
        .chunks_exact(12)
    {
        let tag = u16::from_le_bytes([entry[0], entry[1]]);
        let datatype = u16::from_le_bytes([entry[2], entry[3]]);
        let count = u32::from_le_bytes(entry[4..8].try_into().map_err(|_| "TIFF count overflow")?);
        let value = entry[8..12]
            .try_into()
            .map_err(|_| "Truncated TIFF value")?;
        if out
            .insert(
                tag,
                TiffEntry {
                    datatype,
                    count,
                    value,
                },
            )
            .is_some()
        {
            return Err("Duplicate TIFF IFD tag".into());
        }
    }
    if remaining.get(2 + 12 * count..6 + 12 * count) != Some(&[0u8; 4][..]) {
        return Err("GeoTIFF ingestion supports one TIFF image directory".into());
    }
    Ok(out)
}

fn tiff_epsg(entries: &BTreeMap<u16, TiffEntry>, bytes: &[u8]) -> Result<u32, String> {
    let directory = entries
        .get(&34735)
        .filter(|entry| entry.datatype == 3)
        .and_then(|entry| entry.span(bytes, 2))
        .ok_or("GeoTIFF lacks a bounded GeoKeyDirectoryTag")?;
    let mut words = directory
        .chunks_exact(2)
        .map(|word| u16::from_le_bytes([word[0], word[1]]));
    let (Some(1), Some(1), Some(0 | 1), Some(keys)) =
        (words.next(), words.next(), words.next(), words.next())
    else {
        return Err("Unsupported or truncated GeoTIFF GeoKey directory version".into());
    };
    if usize::from(keys) > 128 || words.len() != usize::from(keys) * 4 {
        return Err("GeoTIFF GeoKey extent is inconsistent".into());
    }
    let mut codes = BTreeMap::new();
    for key in directory[8..].chunks_exact(8) {
        let id = u16::from_le_bytes([key[0], key[1]]);
        if matches!(id, 1024 | 1025 | 2048 | 3072) {
            if key[2..6] != [0, 0, 1, 0] {
                return Err("GeoTIFF CRS keys require inline scalar SHORT values".into());
            }
            if codes
                .insert(id, u16::from_le_bytes([key[6], key[7]]))
                .is_some()
            {
                return Err("Duplicate GeoTIFF CRS key".into());
            }
        }
    }
    if codes.get(&1025).copied().unwrap_or(1) != 1 {
        return Err("GeoTIFF ingestion supports PixelIsArea footprints".into());
    }
    let crs_key = match codes.get(&1024) {
        Some(1) => 3072,
        Some(2) => 2048,
        _ => return Err("GeoTIFF must identify its projected or geographic CRS model".into()),
    };
    codes
        .get(&crs_key)
        .copied()
        .filter(|code| (1..32_767).contains(code))
        .map(u32::from)
        .ok_or("GeoTIFF must identify an EPSG CRS".into())
}

fn geotiff(
    host: &mut AssetHost<'_>,
    source: &Value,
    declared: Option<u32>,
    target: u32,
) -> Result<(u32, Args), String> {
    let bytes = &resource(host, source)?.bytes;
    let entries = tiff_entries(bytes)?;
    let crs = tiff_epsg(&entries, bytes)?;
    same_raster_crs(crs, declared, target)?;
    let scalar = |tag: u16, default: Option<u32>| -> Result<u32, String> {
        match entries.get(&tag) {
            Some(entry) => entry
                .scalar()
                .ok_or_else(|| format!("Invalid TIFF scalar tag {tag}")),
            None => default.ok_or_else(|| format!("Missing TIFF tag {tag}")),
        }
    };
    let width = scalar(256, None)?;
    let height = scalar(257, None)?;
    let cells = (width as usize)
        .checked_mul(height as usize)
        .filter(|n| (1..=RASTER_CELLS_MAX).contains(n))
        .ok_or("GeoTIFF raster exceeds the bounded cell budget")?;
    if scalar(259, Some(1))? != 1
        || scalar(277, Some(1))? != 1
        || scalar(274, Some(1))? != 1
        || scalar(284, Some(1))? != 1
        || scalar(317, Some(1))? != 1
        || scalar(278, Some(height))? < height
        || scalar(262, Some(1))? != 1
        || [322, 323, 324, 325, 330, 338, 34264, 42112, 42113]
            .iter()
            .any(|tag| entries.contains_key(tag))
    {
        return Err("GeoTIFF requires one unmasked top-left single-band uncompressed strip".into());
    }
    let word_bytes = match (scalar(339, None)?, scalar(258, None)?) {
        (1 | 2, 16) => 2,
        (3, 32) => 4,
        _ => return Err("GeoTIFF band must be Float32, Int16 or UInt16".into()),
    };
    let offset = scalar(273, None)? as usize;
    let strip_entry = entries.get(&273).ok_or("Missing TIFF strip offset")?;
    if u32::from_le_bytes(strip_entry.value) as usize != offset {
        return Err("TIFF strip offset has nonzero inline padding".into());
    }
    if scalar(279, None)? as usize != cells * word_bytes
        || offset
            .checked_add(cells * word_bytes)
            .is_none_or(|end| end > bytes.len())
    {
        return Err("GeoTIFF strip extent disagrees with its sample format and shape".into());
    }
    let doubles = |tag, count| {
        entries
            .get(&tag)
            .filter(|entry| entry.datatype == 12 && entry.count == count)
            .and_then(|entry| entry.span(bytes, 8))
            .ok_or("Missing GeoTIFF scale or tie point")
    };
    let scale = doubles(33550, 3)?;
    let tie = doubles(33922, 6)?;
    let read =
        |raw: &[u8], at: usize| f64::from_le_bytes(raw[8 * at..8 * (at + 1)].try_into().unwrap());
    let (dx, dy) = (read(scale, 0), read(scale, 1));
    if ![dx, dy].iter().all(|v| (1e-12..=1e12).contains(v))
        || (0..3).any(|at| read(tie, at) != 0.0)
        || (3..6).any(|at| !read(tie, at).is_finite())
    {
        return Err("GeoTIFF must be a north-up raster with a zero-index finite tie point".into());
    }
    let (west, north) = (read(tie, 3), read(tie, 4));
    let (east, south) = (west + dx * f64::from(width), north - dy * f64::from(height));
    if !east.is_finite() || !south.is_finite() || west >= east || south >= north {
        return Err("GeoTIFF bounds are non-finite or degenerate".into());
    }
    host.charge(cells.saturating_mul(10), cells.saturating_mul(4))?;
    let mut samples = vec![0.0; cells];
    let info = decode_geotiff_raster_into(&resource(host, source)?.bytes, &mut samples)
        .map_err(|error| format!("Native GeoTIFF decode: {error:?}"))?;
    if info.width != width as usize
        || info.height != height as usize
        || info.sample_count != cells
        || samples.iter().any(|sample| !sample.is_finite())
    {
        return Err("Decoded GeoTIFF disagrees with its shape or finite sample domain".into());
    }
    let field = lattice(host, vec![height, width], samples)?;
    Ok((
        crs,
        BTreeMap::from([
            ("field".into(), field),
            ("bounds".into(), values([west, south, east, north])),
            (
                "sample_origin_xy".into(),
                values([west + dx * 0.5, north - dy * 0.5]),
            ),
            ("sample_step_xy".into(), values([dx, -dy])),
            ("row_order".into(), Value::String("north_to_south".into())),
        ]),
    ))
}

fn netcdf(
    host: &mut AssetHost<'_>,
    source: &Value,
    declared: Option<u32>,
    target: u32,
) -> Result<(u32, Args), String> {
    let crs =
        declared.ok_or("Classic NetCDF slab ingestion requires an authored source EPSG code")?;
    same_raster_crs(crs, declared, target)?;
    let result = host.evaluate_gis_gp_terrain_and_datacubes(&BTreeMap::from([
        ("op".into(), Value::String("decode_netcdf_f32".into())),
        (
            "parameters".into(),
            Value::Record(BTreeMap::from([("source".into(), source.clone())])),
        ),
    ]))?;
    let Value::Record(mut result) = result else {
        return Err("Classic NetCDF decoder did not return a field record".into());
    };
    result.insert(
        "crs_origin".into(),
        Value::String("authored_declaration".into()),
    );
    Ok((crs, result))
}

fn las(
    host: &mut AssetHost<'_>,
    source: &Value,
    declared: Option<u32>,
    target: u32,
) -> Result<(u32, Args), String> {
    let crs = declared.ok_or("LAS ingestion requires an authored source EPSG code")?;
    let bytes = &resource(host, source)?.bytes;
    let header =
        parse_las_header(bytes).map_err(|error| format!("Native LAS header: {error:?}"))?;
    let legacy_count = u64::from(u32::from_le_bytes(
        bytes[107..111]
            .try_into()
            .map_err(|_| "LAS legacy point count is truncated")?,
    ));
    let point_count = if header.version_minor == 4 {
        // ASPRS LAS 1.4 stores the extended point count immediately after
        // the EVLR count, at byte 247. The legacy count may be zero.
        let extended_count = u64::from_le_bytes(
            bytes
                .get(247..255)
                .ok_or("LAS 1.4 point count is truncated")?
                .try_into()
                .map_err(|_| "LAS 1.4 point count is truncated")?,
        );
        if legacy_count != 0 && extended_count != 0 && legacy_count != extended_count {
            return Err("LAS point count fields disagree".into());
        }
        extended_count.max(legacy_count)
    } else {
        legacy_count
    };
    let count = usize::try_from(point_count)
        .ok()
        .filter(|n| (1..=POINTS_MAX).contains(n))
        .ok_or("LAS exceeds the bounded point count or is empty")?;
    let offset = header.offset_to_point_data as usize;
    let record_bytes = header.point_data_record_length as usize;
    let header_bytes = u16::from_le_bytes([bytes[94], bytes[95]]) as usize;
    let expected_header = match header.version_minor {
        2 => 227,
        3 => 235,
        _ => 375,
    };
    if header.point_data_format != 0
        || record_bytes < 20
        || header_bytes < expected_header
        || offset < header_bytes
        || (0..3).any(|axis| {
            !header.scale[axis].is_finite()
                || header.scale[axis] <= 0.0
                || !header.offset[axis].is_finite()
                || !header.min_bounds[axis].is_finite()
                || !header.max_bounds[axis].is_finite()
                || header.max_bounds[axis] < header.min_bounds[axis]
        })
        || count
            .checked_mul(record_bytes)
            .and_then(|len| offset.checked_add(len))
            .is_none_or(|end| end > bytes.len())
    {
        return Err("LAS requires a complete uncompressed format-0 point extent".into());
    }
    host.charge(count.saturating_mul(96), count.saturating_mul(256))?;
    let bytes = &resource(host, source)?.bytes;
    let mut coordinates = Vec::with_capacity(count);
    let mut classifications = Vec::with_capacity(count);
    let mut intensities = Vec::with_capacity(count);
    for raw in bytes[offset..offset + count * record_bytes].chunks_exact(record_bytes) {
        let p = decode_las_point_format0(raw, &header).map_err(|e| format!("LAS point: {e:?}"))?;
        let [x, y] = project([p.x, p.y], crs, target)?;
        if !p.z.is_finite() {
            return Err("Non-finite LAS point elevation".into());
        }
        coordinates.push([x, y, p.z]);
        classifications.push(p.classification & 31);
        intensities.push(p.intensity);
    }
    let layer = json!({"crs_epsg": target, "geometry_type": "Point", "coordinates": coordinates,
        "classification": classifications, "intensity": intensities});
    let bytes = serde_json::to_vec(&layer).map_err(|e| e.to_string())?;
    let layer = host.encode("json", bytes)?;
    Ok((
        crs,
        BTreeMap::from([
            ("vector_layers".into(), layer),
            ("point_count".into(), Value::U64(count as u64)),
            (
                "crs_origin".into(),
                Value::String("authored_declaration".into()),
            ),
            (
                "z_reference".into(),
                Value::String("retained_source_vertical_axis".into()),
            ),
        ]),
    ))
}

fn project([x, y]: [f64; 2], source: u32, target: u32) -> Result<[f64; 2], String> {
    const A: f64 = 6_378_137.0;
    const LIMIT: f64 = A * std::f64::consts::PI;
    if !x.is_finite() || !y.is_finite() {
        return Err("Non-finite vector coordinates".into());
    }
    match (source, target) {
        (4326, 3857) if x.abs() <= 180.0 && y.abs() <= 85.051_128_779_806_6 => Ok([
            A * x.to_radians(),
            A * (std::f64::consts::FRAC_PI_4 + 0.5 * y.to_radians())
                .tan()
                .ln(),
        ]),
        (3857, 4326) if x.abs() <= LIMIT && y.abs() <= LIMIT => Ok([
            (x / A).to_degrees(),
            (2.0 * (y / A).exp().atan() - std::f64::consts::FRAC_PI_2).to_degrees(),
        ]),
        (4326, 4326) if x.abs() <= 180.0 && y.abs() <= 90.0 => Ok([x, y]),
        (a, b) if a == b && a != 4326 => Ok([x, y]),
        _ => {
            Err("Unsupported EPSG transformation or coordinate outside its supported domain".into())
        }
    }
}

fn values(v: impl IntoIterator<Item = f64>) -> Value {
    Value::List(v.into_iter().map(Value::F64).collect())
}

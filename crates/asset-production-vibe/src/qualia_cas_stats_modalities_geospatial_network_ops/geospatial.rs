//! Geospatial Category A adapters for retained terrain, lects, and sound recordings.

use super::{exact_u64, external_resource, ReceiptGraph};
use crate::{
    fields, list, num, record, required, text, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    q42::m64_matter::{
        M64DecodedRecord, M64PayloadInterval, M64RasterRows, M64ReadLimits, M64Reader,
    },
    render::{
        assets::Mesh,
        compile_10d::{compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2},
        portal_qac::{Limits as QacLimits, RetainedQac},
    },
};
use qualia_geospatial::{
    hilbert_to_xy, xy_to_hilbert, CorridorKind, DiachronicIsogloss, GeodeticCoord, GeodeticFrame,
    GeologicUnitRecord, GeologicVectorLayer, HermiteCorridorSegment, SonicToponymStreamer,
    SplineControlPoint, TimeFrequencyBox, SRID_GDA2020, SRID_WGS84, VERTICAL_DATUM_AHD,
    VERTICAL_DATUM_ELLIPSOIDAL,
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::atomic::Ordering,
};

const MAX_SEGMENTS: usize = 64;
const MAX_GEOLOGIC_UNITS: usize = 96;
const GEOSPATIAL_ONTOLOGY: &str = "https://qualiadb.org/ontology/geospatial#";

impl AssetHost<'_> {
    pub(crate) fn synthesize_corridor_and_isogloss(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let parameters = record(required(args, "parameters")?)?;
        match op {
            "hermite_corridor_cut_fill" => self.grade_hermite_corridor(parameters),
            "diachronic_isogloss_boundary" => self.evaluate_isogloss(parameters),
            "sonic_toponym_stream" => self.export_sonic_toponym(parameters),
            "hilbert_dggs_index" => hilbert_index(parameters),
            "geologic_wfs_layer" => self.map_geologic_layer(parameters),
            _ => Err(format!("Unknown geospatial operation {op}")),
        }
    }

    fn grade_hermite_corridor(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["terrain", "origin_lat_lon", "segments", "entity_iri"])?;
        let terrain = required(a, "terrain")?;
        let entity = iri(required(a, "entity_iri")?)?;
        let rows = list(required(a, "segments")?)?;
        if rows.is_empty() || rows.len() > MAX_SEGMENTS {
            return Err("A corridor requires 1..=64 Hermite segments".into());
        }
        let segments = rows.iter().map(segment).collect::<Result<Vec<_>, _>>()?;

        let compiled;
        let input = if matches!(terrain, Value::Record(fields) if fields.contains_key("samples")) {
            let origin = required(a, "origin_lat_lon")?;
            compiled = self.geodesy_op(
                "hypermedia.geodesy.compile_m64",
                &BTreeMap::from([
                    ("heightfield".into(), terrain.clone()),
                    ("origin_lat_lon".into(), origin.clone()),
                ]),
            )?;
            &compiled
        } else {
            if a.contains_key("origin_lat_lon") {
                return Err("Retained M64 already supplies the origin_lat_lon".into());
            }
            terrain
        };
        let size = terrain_bytes(self, input)?.len();
        let max_chunks = size / (64 * 4096);
        let scratch_bytes = max_chunks.saturating_mul(std::mem::size_of::<M64PayloadInterval>());
        let maximum = self.limits.resource_bytes;
        if size > maximum || max_chunks == 0 {
            return Err(
                "M64 terrain exceeds the corridor read budget or contains no raster chunks".into(),
            );
        }
        let work = size / 64 * (1 + 21 * segments.len());
        self.charge(work, size.saturating_add(scratch_bytes))?;
        let mut scratch = vec![M64PayloadInterval::default(); max_chunks];
        let read_limits = M64ReadLimits {
            max_file_bytes: maximum,
            max_chunks,
            max_records: (size / 64) as u64,
        };
        let input = terrain_bytes(self, input)?;
        let reader = M64Reader::parse(input, read_limits, &mut scratch)
            .map_err(|e| format!("M64 terrain: {e:?}"))?;
        let raster = *reader
            .georaster()
            .ok_or("Corridor grading requires a version 3 M64 georaster")?;
        let shape = raster
            .shape()
            .map_err(|e| format!("M64 raster shape: {e:?}"))?;
        let mut result = input.to_vec();
        let cell_mm = f64::from(raster.cell_size_mm);
        let dy = if raster.rows == M64RasterRows::North {
            cell_mm
        } else {
            -cell_mm
        };
        let mut changed = 0u64;
        let mut cut_mm = 0u64;
        let mut fill_mm = 0u64;
        for index in 0..shape.chunks {
            if self.cancel.load(Ordering::Relaxed) {
                return Err("M64 corridor grading cancelled".into());
            }
            let chunk = reader
                .chunk(index)
                .ok_or("M64 corridor lacks a declared chunk")?;
            let start = chunk.directory_entry().byte_offset as usize;
            let chunk_x = index % shape.chunks_x;
            let chunk_y = index / shape.chunks_x;
            for cell in 0..4096 {
                let raw_x = chunk_x * 64 + cell % 64;
                let raw_y = chunk_y * 64 + cell / 64;
                let x = raw_x.min(shape.width - 1);
                let y = raw_y.min(shape.height - 1);
                let Some(M64DecodedRecord::Strata(mut column)) = chunk
                    .decode_record(cell)
                    .map_err(|e| format!("M64 strata record: {e:?}"))?
                else {
                    return Err("M64 corridor expected a strata column".into());
                };
                let old = column.surface_elevation_mm;
                if raster.nodata_sentinel_mm == Some(old) {
                    continue;
                }
                for segment in &segments {
                    // Authored order is significant at segment overlaps.
                    segment.conform_strata_column(x as f64 * cell_mm, y as f64 * dy, &mut column);
                }
                let new = column.surface_elevation_mm;
                if new == old {
                    continue;
                }
                if raster.nodata_sentinel_mm == Some(new) {
                    return Err("Corridor grade would collide with the M64 nodata sentinel".into());
                }
                // GeoRaster's reserved non-elevation fields are checked zero.
                // Copy the source column and write only the authored elevation.
                let offset = start + cell * 64;
                let mut elevation_only: qualia_core_db::q42::m64_matter::M64StrataColumnRecord =
                    bytemuck::pod_read_unaligned(&input[offset..offset + 64]);
                elevation_only.surface_elevation_mm = new;
                elevation_only.update_crc();
                result[offset..offset + 64].copy_from_slice(bytemuck::bytes_of(&elevation_only));
                if (raw_x, raw_y) == (x, y) {
                    changed += 1;
                    let delta = i64::from(new) - i64::from(old);
                    if delta < 0 {
                        cut_mm += delta.unsigned_abs();
                    } else {
                        fill_mm += delta as u64;
                    }
                }
            }
        }
        M64Reader::parse(&result, read_limits, &mut scratch)
            .map_err(|e| format!("Graded M64 terrain: {e:?}"))?;
        let mut scene = SceneSurface::default();
        for segment in &segments {
            let half_width = corridor_width(segment.kind) / 2000.0;
            let profile_offset = match segment.kind {
                CorridorKind::Road {
                    crown_height_mm, ..
                } => f64::from(crown_height_mm) / 1000.0,
                CorridorKind::Creek { depth_mm, .. } => -f64::from(depth_mm) / 1000.0,
            };
            let steps = 64usize;
            let mut edges = Vec::with_capacity(steps + 1);
            for step in 0..=steps {
                let t = step as f64 / steps as f64;
                let (p, h) = segment.sample_centerline(t);
                let (before, _) = segment.sample_centerline((t - 1.0 / steps as f64).max(0.0));
                let (after, _) = segment.sample_centerline((t + 1.0 / steps as f64).min(1.0));
                let tangent = [after[0] - before[0], after[1] - before[1]];
                let length = tangent[0].hypot(tangent[1]);
                if length == 0.0 {
                    return Err("The Hermite corridor has an unresolved stationary tangent".into());
                }
                let across = [
                    -tangent[1] / length * half_width,
                    tangent[0] / length * half_width,
                ];
                let elevation = (h - f64::from(raster.origin_elevation_mm)) / 1000.0;
                let [x, y] = p.map(|v| v / 1000.0);
                edges.push([
                    [x - across[0], elevation, -y + across[1]],
                    [x, elevation + profile_offset, -y],
                    [x + across[0], elevation, -y - across[1]],
                ]);
            }
            scene.strip(edges)?;
        }
        let scene = scene.seal(entity)?;
        let m64 = self.encode("m64", result)?;
        let ten_d = self.encode("10d", scene)?;
        let mm_m2_to_m3 = (cell_mm / 1000.0).powi(2) / 1000.0;
        Ok(Value::Record(BTreeMap::from([
            ("m64".into(), m64),
            ("scene_10d".into(), ten_d),
            ("graded_cells".into(), Value::U64(changed)),
            ("cut_m3".into(), Value::F64(cut_mm as f64 * mm_m2_to_m3)),
            ("fill_m3".into(), Value::F64(fill_mm as f64 * mm_m2_to_m3)),
        ])))
    }

    fn evaluate_isogloss(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "lect_iri",
                "frame",
                "center",
                "radius_m",
                "era_years",
                "position",
                "year",
            ],
        )?;
        let lect_iri = iri(required(a, "lect_iri")?)?;
        let frame = geodetic_frame(required(a, "frame")?)?;
        let center = coordinate(required(a, "center")?)?;
        let position = coordinate(required(a, "position")?)?;
        let [start, end] = vector::<2>(required(a, "era_years")?)?;
        let radius = num(required(a, "radius_m")?)?;
        let year = num(required(a, "year")?)?;
        if !(-100_000.0..=100_000.0).contains(&start)
            || !(start..=100_000.0).contains(&end)
            || !(-100_000.0..=100_000.0).contains(&year)
            || !(0.01..=1_000_000.0).contains(&radius)
        {
            return Err("Isogloss radius or era is outside the local projection bounds".into());
        }
        let hash = qualia_core_db::q_hash(lect_iri);
        let isogloss = DiachronicIsogloss::new(hash, center, radius, start as f32, end as f32);
        let active = isogloss.contains_point(&position, year as f32, &frame);
        let attenuation = isogloss.evaluate_attenuation_weight(&position, year as f32, &frame);
        let (east, north, up) = frame.project_to_local_mm(&center);
        let center = [
            east as f64 / 1000.0,
            f64::from(up) / 1000.0,
            -north as f64 / 1000.0,
        ];
        let mut scene = SceneSurface::default();
        let width = (radius * 0.01).clamp(0.0025, 10.0);
        scene.strip((0..=128).map(|i| {
            let angle = if i == 128 {
                0.0
            } else {
                std::f64::consts::TAU * f64::from(i) / 128.0
            };
            let (sin, cos) = angle.sin_cos();
            [radius - width, radius + width]
                .map(|r| [center[0] + r * cos, center[1], center[2] - r * sin])
        }))?;
        self.charge(256, 16 * 1024)?;
        let scene = self.encode("10d", scene.seal(lect_iri)?)?;
        Ok(Value::Record(BTreeMap::from([
            ("scene_10d".into(), scene),
            ("lect_hash".into(), Value::U64(hash)),
            ("contains".into(), Value::Bool(active)),
            ("attenuation_weight".into(), Value::F64(attenuation)),
        ])))
    }

    fn export_sonic_toponym(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "audio",
                "entry",
                "site_iri",
                "custodial_scope_iri",
                "site",
                "frame",
                "listener",
                "listener_velocity_m_s",
                "base_spl_db",
                "time_ns",
                "frequency_hz",
                "license",
            ],
        )?;
        let license = record(required(a, "license")?)?;
        fields(
            license,
            &["spdx_id", "source_uri", "attribution", "export_authorized"],
        )?;
        if required(license, "export_authorized")? != &Value::Bool(true) {
            return Err(
                "Sonic export requires explicit custodial and source-license authorization".into(),
            );
        }
        let spdx = text(required(license, "spdx_id")?)?;
        if spdx.is_empty()
            || spdx.len() > 80
            || !spdx
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b".-+".contains(&b))
        {
            return Err("Sonic export requires a single SPDX license ID".into());
        }
        let attribution = text(required(license, "attribution")?)?;
        if attribution.trim().is_empty() || attribution.len() > 4096 {
            return Err("Sonic export requires a bounded attribution notice".into());
        }
        let toponym = iri(required(a, "site_iri")?)?;
        let scope = iri(required(a, "custodial_scope_iri")?)?;
        let source = iri(required(license, "source_uri")?)?;
        let frame = geodetic_frame(required(a, "frame")?)?;
        let site = coordinate(required(a, "site")?)?;
        let listener = coordinate(required(a, "listener")?)?;
        let velocity = vector::<3>(required(a, "listener_velocity_m_s")?)?;
        let spl = num(required(a, "base_spl_db")?)?;
        let [min_freq, max_freq] = vector::<2>(required(a, "frequency_hz")?)?;
        let [start, end] = list(required(a, "time_ns")?)? else {
            return Err("time_ns requires [start, end] exact integer nanoseconds".into());
        };
        let (start_nanos, end_nanos) = (exact_u64(start)?, exact_u64(end)?);
        if start_nanos >= end_nanos
            || min_freq < 0.0
            || !(min_freq..=96_000.0).contains(&max_freq)
            || min_freq == max_freq
            || !(0.0..=200.0).contains(&spl)
            || velocity.iter().map(|v| v * v).sum::<f64>() > 150.0_f64.powi(2)
        {
            return Err(
                "Invalid sonic interval, bandwidth, pressure level, or listener velocity".into(),
            );
        }
        let streamer = SonicToponymStreamer::new(
            site,
            TimeFrequencyBox {
                start_nanos,
                end_nanos,
                min_freq_hz: min_freq as f32,
                max_freq_hz: max_freq as f32,
                ..Default::default()
            },
            qualia_core_db::q_hash(scope),
            spl as f32,
        );
        let input = required(a, "audio")?;
        let size = external_resource(self, input)?.bytes.len();
        let entry = u32::try_from(exact_u64(required(a, "entry")?)?)
            .map_err(|_| "QAC entry outside u32")?;
        let memory = self.limits.total_resource_bytes.min(320 * 1024 * 1024);
        let mut limits = QacLimits {
            memory_bytes: memory,
            ..Default::default()
        };
        limits.envelope.file_bytes = limits
            .envelope
            .file_bytes
            .min(self.limits.resource_bytes as u64);
        if size as u64 > limits.envelope.file_bytes {
            return Err("Retained sonic QAC exceeds the export byte budget".into());
        }
        self.charge(size, size.saturating_mul(2))?;
        let audio = external_resource(self, input)?;
        if audio.kind != "qac" {
            return Err("Sonic toponym requires an admitted QAC PCM recording".into());
        }
        let retained = RetainedQac::open(audio.bytes.clone().into_boxed_slice(), entry, limits)
            .map_err(|e| format!("Sonic QAC admission: {e:?}"))?;
        let metadata = retained.metadata();
        if metadata.mode != 0x100 {
            return Err("Sonic toponym requires playable waveform PCM".into());
        }
        let records = metadata.records;
        let channels = metadata.channels;
        let rate = metadata.timing.samples;
        let duration_scaled = u128::from(records) * u128::from(rate.denominator) * 1_000_000_000;
        if u128::from(end_nanos) * u128::from(rate.numerator) > duration_scaled
            || max_freq > rate.numerator as f64 / (2.0 * rate.denominator as f64)
        {
            return Err(
                "Sonic time-frequency bounds exceed the recording extent or Nyquist limit".into(),
            );
        }
        drop(retained);

        let hash = format!("urn:sha256:{:x}", Sha256::digest(&audio.bytes));
        let notice = format!("urn:sha256:{:x}", Sha256::digest(attribution.as_bytes()));
        let mut provenance = ReceiptGraph::new(GEOSPATIAL_ONTOLOGY);
        for (predicate, object) in [
            ("audio", hash.as_str()),
            ("custodialScope", scope),
            ("source", source),
            ("attributionNoticeDigest", notice.as_str()),
        ] {
            provenance.fact(
                toponym,
                &format!("{GEOSPATIAL_ONTOLOGY}{predicate}"),
                object,
            )?;
        }
        let interval = format!("urn:geospatial:recording-window:{start_nanos}:{end_nanos}");
        provenance.fact(
            toponym,
            &format!("{GEOSPATIAL_ONTOLOGY}recordingWindow"),
            &interval,
        )?;
        provenance.fact(
            toponym,
            "http://purl.org/dc/terms/license",
            &format!("https://spdx.org/licenses/{spdx}.html"),
        )?;
        let model = provenance.seal(self.limits.resource_bytes)?;
        let bytes = audio.bytes.clone();
        let spl = streamer.evaluate_listener_spl_db(&listener, &frame);
        let doppler = streamer.evaluate_doppler_factor(velocity, &listener, &frame);
        Ok(Value::Record(BTreeMap::from([
            ("qac".into(), self.encode("qac", bytes)?),
            ("model".into(), self.encode("q42", model)?),
            ("entry".into(), Value::U64(u64::from(entry))),
            ("frames".into(), Value::U64(records)),
            ("channels".into(), Value::U64(u64::from(channels))),
            ("attribution".into(), Value::String(attribution.into())),
            ("spdx_id".into(), Value::String(spdx.into())),
            ("source_uri".into(), Value::String(source.into())),
            (
                "time_ns".into(),
                Value::List(vec![Value::U64(start_nanos), Value::U64(end_nanos)]),
            ),
            ("listener_spl_db".into(), Value::F64(f64::from(spl))),
            ("doppler_factor".into(), Value::F64(doppler)),
        ])))
    }

    fn map_geologic_layer(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["features"])?;
        let resource = required(a, "features")?;
        let size = external_resource(self, resource)?.bytes.len();
        if size > 2 * 1024 * 1024 {
            return Err("Geological WFS layer exceeds the GeoJSON byte budget".into());
        }
        self.charge(size, size.saturating_mul(8))?;
        let features = external_resource(self, resource)?;
        if !matches!(features.kind.as_str(), "geojson" | "json") {
            return Err("Geological WFS layer expects a retained GeoJSON FeatureCollection".into());
        }
        let doc: serde_json::Value = serde_json::from_slice(&features.bytes)
            .map_err(|e| format!("Invalid WFS GeoJSON: {e}"))?;
        if doc["type"] != "FeatureCollection" {
            return Err("Expected a WFS GeoJSON FeatureCollection".into());
        }
        let features = doc["features"]
            .as_array()
            .ok_or("GeoJSON is missing its features array")?;
        if features.is_empty() || features.len() > MAX_GEOLOGIC_UNITS {
            return Err("Geological WFS layer requires 1..=96 features".into());
        }
        let units = features
            .iter()
            .map(geologic_unit)
            .collect::<Result<Vec<_>, _>>()?;
        let mut codes = BTreeSet::new();
        let mut indices = BTreeSet::new();
        for unit in &units {
            if !codes.insert(unit.unit_code.to_lowercase()) || !indices.insert(unit.lut_index) {
                return Err("Geological WFS layer has duplicate unit codes or recipe slots".into());
            }
        }
        let layer = GeologicVectorLayer { units };
        let recipes = layer.compile_bank_1_recipes();
        let mut units = Vec::with_capacity(layer.units.len());
        for unit in &layer.units {
            let index = layer
                .resolve_unit_to_lut_index(&unit.unit_code)
                .ok_or("Unknown geological unit")?;
            let recipe = recipes[usize::from(index - 128)];
            units.push(Value::Record(BTreeMap::from([
                ("unit_code".into(), Value::String(unit.unit_code.clone())),
                ("lithology".into(), Value::String(unit.lithology.clone())),
                (
                    "stratigraphic_age_ma".into(),
                    Value::F64(unit.stratigraphic_age_ma),
                ),
                ("lut_index".into(), Value::U64(u64::from(index))),
                (
                    "bulk_density_kg_m3".into(),
                    Value::U64(u64::from(recipe.bulk_density_kg_m3)),
                ),
                (
                    "friction_angle_deg".into(),
                    Value::U64(u64::from(recipe.friction_angle_deg)),
                ),
                (
                    "cohesion_kpa".into(),
                    Value::U64(u64::from(recipe.cohesion_kpa)),
                ),
            ])));
        }
        Ok(Value::Record(BTreeMap::from([(
            "units".into(),
            Value::List(units),
        )])))
    }
}

fn terrain_bytes<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a [u8], String> {
    if let Ok(TypedArtifact::FieldLattice(lattice)) = host.workspace.typed(value) {
        if let crate::artifact::FieldLattice::M64(bytes) = lattice.as_ref() {
            return Ok(bytes);
        }
        return Err("Field lattice must carry a georaster M64 resource".into());
    }
    let resource = external_resource(host, value)?;
    if resource.kind != "m64" {
        return Err("Expected an encoded M64 terrain resource".into());
    }
    Ok(&resource.bytes)
}

fn hilbert_index(a: &Args) -> Result<Value, String> {
    fields(a, &["order", "xy", "key"])?;
    let order = exact_u64(required(a, "order")?)?;
    if !(1..=31).contains(&order) {
        return Err("Hilbert order must be in 1..=31".into());
    }
    let side = 1u64 << order;
    let (x, y, key) = match (a.get("xy"), a.get("key")) {
        (Some(xy), None) => {
            let [x, y] = list(xy)? else {
                return Err("xy requires two exact grid coordinates".into());
            };
            let (x, y) = (exact_u64(x)?, exact_u64(y)?);
            if x >= side || y >= side {
                return Err("Hilbert coordinate exceeds the declared order".into());
            }
            let (x, y) = (x as u32, y as u32);
            (x, y, xy_to_hilbert(x, y, order as usize))
        }
        (None, Some(key)) => {
            let key = exact_u64(key)?;
            if key >= side * side {
                return Err("Hilbert key exceeds the declared order".into());
            }
            let (x, y) = hilbert_to_xy(key, order as usize);
            (x, y, key)
        }
        _ => return Err("Hilbert requires exactly one of xy or key".into()),
    };
    Ok(Value::Record(BTreeMap::from([
        ("key".into(), Value::U64(key)),
        (
            "xy".into(),
            Value::List(vec![Value::U64(u64::from(x)), Value::U64(u64::from(y))]),
        ),
    ])))
}

fn iri(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    let Some((scheme, tail)) = value.split_once(':') else {
        return Err("An absolute Unicode RDF IRI is required".into());
    };
    if scheme.is_empty()
        || tail.is_empty()
        || value.len() > 4096
        || !scheme.as_bytes()[0].is_ascii_alphabetic()
        || !scheme
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"+.-".contains(&b))
        || tail
            .chars()
            .any(|c| c.is_whitespace() || c.is_control() || "<>{}|\\^\"".contains(c))
    {
        return Err("An absolute Unicode RDF IRI is required".into());
    }
    Ok(value)
}

fn signed_mm(meters: f64) -> Result<i32, String> {
    let mm = (meters * 1000.0).round();
    if !(f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&mm) {
        return Err("Elevation must fit signed millimetres".into());
    }
    Ok(mm as i32)
}

fn coordinate(input: &Value) -> Result<GeodeticCoord, String> {
    let [lat, lon, elevation] = vector::<3>(input)?;
    if !(-89.999..=89.999).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
        return Err("Invalid local geodetic latitude/longitude".into());
    }
    if !(-20_000.0..=20_000.0).contains(&elevation) {
        return Err("Geodetic elevation is outside the supported local vertical range".into());
    }
    Ok(GeodeticCoord::new(lat, lon, signed_mm(elevation)?))
}

fn geodetic_frame(input: &Value) -> Result<GeodeticFrame, String> {
    let frame = record(input)?;
    fields(frame, &["origin", "srid", "vertical_datum", "epoch_year"])?;
    let origin = coordinate(required(frame, "origin")?)?;
    let srid = exact_u64(required(frame, "srid")?)?;
    let datum = exact_u64(required(frame, "vertical_datum")?)?;
    let year = num(required(frame, "epoch_year")?)?;
    if ![u64::from(SRID_WGS84), u64::from(SRID_GDA2020)].contains(&srid)
        || ![
            u64::from(VERTICAL_DATUM_AHD),
            u64::from(VERTICAL_DATUM_ELLIPSOIDAL),
        ]
        .contains(&datum)
        || !(1800.0..=2500.0).contains(&year)
        || origin.elevation_ahd_mm.unsigned_abs() > 20_000_000
    {
        return Err(
            "Geospatial frame requires supported CRS, vertical datum, origin and epoch".into(),
        );
    }
    Ok(GeodeticFrame {
        srid: srid as u32,
        vertical_datum: datum as u16,
        epoch_year: year as f32,
        origin_lat_deg: origin.latitude_deg,
        origin_lon_deg: origin.longitude_deg,
        origin_elevation_mm: origin.elevation_ahd_mm,
    })
}

fn control_point(input: &Value) -> Result<SplineControlPoint, String> {
    let row = record(input)?;
    fields(row, &["position_m", "elevation_m", "tangent_m"])?;
    let position = vector::<2>(required(row, "position_m")?)?;
    let tangent = vector::<2>(required(row, "tangent_m")?)?;
    let elevation_mm = signed_mm(num(required(row, "elevation_m")?)?)?;
    if position
        .into_iter()
        .chain(tangent)
        .any(|v| v.abs() > 1_000_000.0)
        || elevation_mm.unsigned_abs() > 20_000_000
    {
        return Err("Corridor control points exceed bounded local metric coordinates".into());
    }
    Ok(SplineControlPoint {
        pos_xy_mm: position.map(|v| v * 1000.0),
        elevation_mm,
        tangent_xy: tangent.map(|v| v * 1000.0),
    })
}

fn corridor_width(kind: CorridorKind) -> f64 {
    match kind {
        CorridorKind::Road { width_mm, .. } | CorridorKind::Creek { width_mm, .. } => {
            f64::from(width_mm)
        }
    }
}

fn segment(input: &Value) -> Result<HermiteCorridorSegment, String> {
    let row = record(input)?;
    let kind = text(required(row, "kind")?)?;
    let p0 = control_point(required(row, "start")?)?;
    let p1 = control_point(required(row, "end")?)?;
    let width = num(required(row, "width_m")?)?;
    if p0.pos_xy_mm == p1.pos_xy_mm || !(0.05..=1000.0).contains(&width) {
        return Err(
            "A corridor requires distinct endpoints and width in 0.05..=1000 meters".into(),
        );
    }
    let width_mm = (width * 1000.0).round() as u32;
    let kind = match kind {
        "road" | "rail" => {
            fields(row, &["kind", "start", "end", "width_m", "crown_m"])?;
            let crown_height_mm = i16::try_from(signed_mm(num(required(row, "crown_m")?)?)?)
                .map_err(|_| "Road crown exceeds signed i16 millimetres")?;
            CorridorKind::Road {
                width_mm,
                crown_height_mm,
            }
        }
        "creek" | "stream" => {
            fields(
                row,
                &[
                    "kind",
                    "start",
                    "end",
                    "width_m",
                    "depth_m",
                    "flow_velocity_mm_s",
                ],
            )?;
            let depth_mm = u16::try_from(signed_mm(num(required(row, "depth_m")?)?)?)
                .map_err(|_| "Creek depth exceeds unsigned u16 millimetres")?;
            let [east, north] = vector::<2>(required(row, "flow_velocity_mm_s")?)?;
            let velocity = |value: f64| -> Result<i16, String> {
                if value.fract() != 0.0
                    || !(f64::from(i16::MIN)..=f64::from(i16::MAX)).contains(&value)
                {
                    return Err("Creek flow must fit exact signed millimetres per second".into());
                }
                Ok(value as i16)
            };
            CorridorKind::Creek {
                width_mm,
                depth_mm,
                flow_velocity_xy: [velocity(east)?, velocity(north)?],
            }
        }
        _ => return Err("Corridor kind requires road, rail, creek, or stream".into()),
    };
    Ok(HermiteCorridorSegment { p0, p1, kind })
}

fn geologic_unit(feature: &serde_json::Value) -> Result<GeologicUnitRecord, String> {
    if feature["type"] != "Feature" {
        return Err("Invalid WFS GeoJSON feature type".into());
    }
    let props = feature["properties"]
        .as_object()
        .ok_or("Geological feature lacks properties")?;
    let code = props
        .get("unit_code")
        .and_then(|v| v.as_str())
        .ok_or("Missing geological unit code")?;
    let lithology = props
        .get("lithology")
        .and_then(|v| v.as_str())
        .ok_or("Missing lithology")?;
    let age = props
        .get("stratigraphic_age_ma")
        .and_then(|v| v.as_f64())
        .ok_or("Missing geological age")?;
    let index = props
        .get("lut_index")
        .and_then(|v| v.as_u64())
        .ok_or("Missing geological LUT index")?;
    if code.is_empty()
        || code.len() > 64
        || lithology.is_empty()
        || lithology.len() > 128
        || !(0.0..=4600.0).contains(&age)
        || !(128..224).contains(&index)
    {
        return Err("Geological feature properties exceed valid age, text, or LUT bounds".into());
    }
    Ok(GeologicUnitRecord {
        unit_code: code.into(),
        lithology: lithology.into(),
        stratigraphic_age_ma: age,
        lut_index: index as u8,
    })
}

#[derive(Default)]
struct SceneSurface {
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
}

impl SceneSurface {
    fn strip<const N: usize>(
        &mut self,
        edges: impl IntoIterator<Item = [[f64; 3]; N]>,
    ) -> Result<(), String> {
        let start = self.positions.len();
        for edge in edges {
            let row = edge.map(|p| p.map(|v| v as f32));
            if row.windows(2).any(|pair| pair[0] == pair[1])
                || row.iter().flatten().any(|v| !v.is_finite())
            {
                return Err(
                    "Geospatial overlay strip is degenerate at 10D render precision".into(),
                );
            }
            if self.positions.len() != start {
                let next = self.positions.len() as u32;
                for band in 0..N - 1 {
                    let right = next + band as u32;
                    let left = right - N as u32;
                    self.triangles
                        .extend([[left, left + 1, right], [left + 1, right + 1, right]]);
                }
            }
            self.positions.extend(row);
        }
        Ok(())
    }

    fn seal(self, entity: &str) -> Result<Vec<u8>, String> {
        let (mut min, mut max) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
        for p in &self.positions {
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
        }
        let mesh = Mesh {
            positions: self.positions,
            triangles: self.triangles,
            min,
            max,
        };
        let bytes =
            compile_mesh_to_10d(&mesh).map_err(|e| format!("Geospatial 10D overlay: {e:?}"))?;
        let [x, y, z] = mesh.centroid();
        upgrade_legacy_10d_to_holographic_v2(
            &bytes,
            entity,
            1,
            [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
        )
        .map_err(|e| format!("Geospatial 10D overlay semantic sealing: {e:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{geodesy_ops::heightfield_record, AssetWorkspace, ToolLimits};
    use std::sync::atomic::AtomicBool;

    fn values<const N: usize>(values: [f64; N]) -> Value {
        Value::List(values.into_iter().map(Value::F64).collect())
    }

    #[test]
    fn grades_retained_raster_and_preserves_repeated_padding() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = AssetHost {
            workspace: AssetWorkspace::default(),
            inputs: &[],
            limits: &limits,
            cancel: &cancel,
            work: 0,
        };
        let point = |x| {
            Value::Record(BTreeMap::from([
                ("position_m".into(), values([x, 0.0])),
                ("elevation_m".into(), Value::F64(2.0)),
                ("tangent_m".into(), values([1.0, 0.0])),
            ]))
        };
        let segment = Value::Record(BTreeMap::from([
            ("kind".into(), Value::String("road".into())),
            ("start".into(), point(0.0)),
            ("end".into(), point(1.0)),
            ("width_m".into(), Value::F64(0.5)),
            ("crown_m".into(), Value::F64(0.0)),
        ]));
        let result = host
            .grade_hermite_corridor(&BTreeMap::from([
                (
                    "terrain".into(),
                    heightfield_record(2, 1.0, [0.0; 3], &[0.0; 4]),
                ),
                ("origin_lat_lon".into(), values([-35.0, 145.0])),
                ("segments".into(), Value::List(vec![segment])),
                ("entity_iri".into(), Value::String("urn:test:road".into())),
            ]))
            .unwrap();
        let result = record(&result).unwrap();
        assert_eq!(result["graded_cells"], Value::U64(2));
        assert_eq!(result["cut_m3"], Value::F64(0.0));
        assert_eq!(result["fill_m3"], Value::F64(4.0));
    }

    #[test]
    fn preserves_full_hilbert_key_at_high_order() {
        let xy = Value::List(vec![Value::U64(0), Value::U64((1 << 31) - 1)]);
        let indexed = hilbert_index(&BTreeMap::from([
            ("order".into(), Value::U64(31)),
            ("xy".into(), xy.clone()),
        ]))
        .unwrap();
        let key = record(&indexed).unwrap()["key"].clone();
        let position = hilbert_index(&BTreeMap::from([
            ("order".into(), Value::U64(31)),
            ("key".into(), key),
        ]))
        .unwrap();
        assert_eq!(record(&position).unwrap()["xy"], xy);
    }
}

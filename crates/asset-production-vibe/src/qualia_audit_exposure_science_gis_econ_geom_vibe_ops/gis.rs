//! Bounded geoprocessing, terrain qualification, and retained datacube ingestion.

use crate::qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph;
use crate::{
    artifact::FieldLattice, fields, list, num, required, text, vector, Args, AssetHost,
    Resource, TypedArtifact, Value,
};
use qualia_core_db::{
    container_10d::{self as ten, field_section as fiel},
    domains::geospatial::{
        gis_gp::{
            gp_hydro_cost_watershed as hydro, gp_kriging_spatial_stats as stats,
            gp_map_algebra_zonal as algebra, gp_network_topology as network,
            gp_viewshed_solar_morph as morph,
        },
        gis_io::datacube_zarr_netcdf_grib as cube,
        terrain::{
            geomorph_qa_report::audit_geomorph_realism, hydro_stream_power::compute_d8_flow,
        },
    },
    q_hash,
    render::{
        assets::Mesh,
        compile_10d::{compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2},
    },
    NQuin,
};
use serde_json::Value as Json;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

const GRAPH: &str = "urn:qualia:gis:operation:v1";
const MAX_CELLS: usize = 65_536;
const MAX_POINTS: usize = 512;
const MAX_CHUNK_BYTES: usize = 64 * 1024 * 1024;
const MAX_ZARR_METADATA_BYTES: usize = 16 * 1024;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    let mut result = match op {
        "geomorph_qa" => geomorph(host, params)?,
        "geomorphons" => geomorphons(host, params)?,
        "d8_basins" => d8(host, params)?,
        "rusle_soil_loss" => rusle(host, params)?,
        "moran_lisa_getis_ord" => moran(host, params)?,
        "gwr" => gwr(host, params)?,
        "ordinary_kriging" => kriging(host, params)?,
        "saaty_ahp" => ahp(host, params)?,
        "edmonds_karp" | "brandes" => network_stats(host, params, op)?,
        "decode_zarr_f32" | "decode_netcdf_f32" | "decode_grib2" => decode(host, params, op)?,
        "seal_datacube_raster" => seal_raster(host, params)?,
        _ => return Err(format!("Unsupported GIS operation {op}")),
    };
    let stable = result
        .iter()
        .filter(
            |(_, value)| !matches!(value, Value::Record(record) if record.contains_key("handle")),
        )
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    let canonical =
        crate::living_world_anatomy::species_spec::vibe_value_to_json(&Value::Record(stable));
    let materialized = serde_json::to_vec(&(op, canonical)).map_err(|error| error.to_string())?;
    let subject = format!(
        "urn:qualia:gis:result:{}",
        blake3::hash(&materialized).to_hex()
    );
    let mut graph = ReceiptGraph::new(GRAPH);
    graph.fact(
        &subject,
        "urn:qualia:gis:operation",
        &format!("urn:qualia:gis:op:{op}"),
    )?;
    for (name, value) in &result {
        let iri = match value {
            Value::Bool(value) => format!("urn:qualia:gis:boolean:{value}"),
            Value::U64(value) => format!("urn:qualia:gis:unsigned:{value}"),
            Value::F64(value) if value.is_finite() => {
                format!("urn:qualia:gis:double:{:016x}", value.to_bits())
            }
            Value::String(digest) if name.ends_with("_digest") => format!("urn:blake3:{digest}"),
            _ => continue,
        };
        graph.fact(&subject, &format!("urn:qualia:gis:{name}"), &iri)?;
    }
    result.insert(
        "receipt".into(),
        host.encode(
            "q42",
            graph.seal(host.limits.resource_bytes.min(4 * 1024 * 1024))?,
        )?,
    );
    Ok(Value::Record(result))
}

#[derive(Clone, Copy)]
struct Grid {
    width: usize,
    height: usize,
    spacing: f32,
}

impl Grid {
    fn parse(params: &Args) -> Result<Self, String> {
        let width = dimension(params, "width", 2048)?;
        let height = dimension(params, "height", 2048)?;
        if width * height > MAX_CELLS {
            return Err("GIS raster exceeds the bounded cell count".into());
        }
        let spacing = number(params, "cell_size_m", 1e-6, 1e6)? as f32;
        Ok(Self {
            width,
            height,
            spacing,
        })
    }

    fn len(self) -> usize {
        self.width * self.height
    }
    fn axes(self) -> Vec<u32> {
        vec![self.height as u32, self.width as u32]
    }
}

fn geomorph(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "cell_size_m",
            "dem",
            "channel_threshold_m2",
            "mass_before",
            "mass_after",
        ],
    )?;
    let grid = Grid::parse(params)?;
    let values = raster(host, required(params, "dem")?, grid, -2e6, 2e6)?;
    let threshold = number(params, "channel_threshold_m2", 0.0, 1e18)? as f32;
    let mass_before = number(params, "mass_before", 1e-7, 1e25)? as f32;
    let mass_after = number(params, "mass_after", 0.0, 1e25)? as f32;
    host.charge(grid.len() * 96, grid.len() * 40)?;
    let mut receiver = vec![0; grid.len()];
    let mut sorted = vec![0; grid.len()];
    let mut area = vec![0.0; grid.len()];
    let mut strahler = vec![0; grid.len()];
    compute_d8_flow(
        &values,
        grid.width,
        grid.height,
        grid.spacing,
        &mut sorted,
        &mut receiver,
        &mut area,
    )?;
    let report = audit_geomorph_realism(
        &values,
        &receiver,
        &sorted,
        &area,
        &mut strahler,
        grid.width,
        grid.height,
        grid.spacing,
        threshold,
        mass_before,
        mass_after,
    )?;
    let mut out = numeric([
        (
            "hypsometric_integral",
            f64::from(report.hypsometric_integral),
        ),
        (
            "drainage_density_m_per_m2",
            f64::from(report.drainage_density_m_per_m2),
        ),
        (
            "strahler_bifurcation_ratio",
            f64::from(report.strahler_bifurcation_ratio),
        ),
        (
            "hacks_law_exponent_h",
            f64::from(report.hacks_law_exponent_h),
        ),
        (
            "channel_concavity_theta",
            f64::from(report.channel_concavity_theta),
        ),
        ("slope_skewness", f64::from(report.slope_skewness)),
        (
            "mass_conservation_rel_err",
            f64::from(report.mass_conservation_rel_err),
        ),
    ])?;
    out.insert(
        "strahler_max_order".into(),
        Value::U64(u64::from(report.strahler_max_order)),
    );
    out.insert(
        "passes_natural_realism_gate".into(),
        Value::Bool(report.passes_natural_realism_gate),
    );
    out.insert(
        "hypsometric_curve_21".into(),
        decimals(report.hypsometric_curve_21.map(f64::from))?,
    );
    add_field(
        host,
        &mut out,
        "strahler_order",
        grid.axes(),
        strahler.into_iter().map(f32::from).collect(),
    )?;
    add_field(host, &mut out, "accumulation_area_m2", grid.axes(), area)?;
    Ok(out)
}

fn geomorphons(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "cell_size_m",
            "dem",
            "lookup_cells",
            "flatness_rad",
        ],
    )?;
    let grid = Grid::parse(params)?;
    let dem = raster(host, required(params, "dem")?, grid, -2e6, 2e6)?;
    let lookup = dimension(params, "lookup_cells", 128)?;
    let flatness = number(params, "flatness_rad", 0.0, std::f64::consts::FRAC_PI_2)? as f32;
    host.charge(grid.len() * lookup * 8, grid.len() * 9)?;
    let mut classes = vec![0; grid.len()];
    morph::gp_classify_geomorphons_into(
        &dem,
        grid.width,
        grid.height,
        grid.spacing,
        lookup,
        flatness,
        &mut classes,
    )
    .map_err(|e| format!("Geomorphons: {e:?}"))?;
    let mut counts = [0u64; 10];
    for &class in &classes {
        counts[usize::from(class - 1)] += 1;
    }
    let mut out = BTreeMap::from([("class_counts".into(), uvalues(counts))]);
    add_field(
        host,
        &mut out,
        "classes",
        grid.axes(),
        classes.into_iter().map(f32::from).collect(),
    )?;
    Ok(out)
}

fn d8(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &["width", "height", "cell_size_m", "dem", "pour_points"],
    )?;
    let grid = Grid::parse(params)?;
    let dem = raster(host, required(params, "dem")?, grid, -2e6, 2e6)?;
    let mut ids = BTreeSet::new();
    let mut points = BTreeSet::new();
    let mut pour = Vec::new();
    for point in bounded_list(required(params, "pour_points")?, 512)? {
        let [x, y, id] = list(point)? else {
            return Err("Pour point requires [x, y, positive basin id]".into());
        };
        let (x, y, id) = (index(x)?, index(y)?, exact(id)?);
        if x >= grid.width
            || y >= grid.height
            || !(1..=(1 << 24)).contains(&id)
            || !ids.insert(id)
            || !points.insert((x, y))
        {
            return Err(
                "D8 pour points need distinct coordinates and positive f32-exact basin IDs".into(),
            );
        }
        pour.push((x, y, id as u32));
    }
    let scratch = grid.len() * 64 + pour.len() * 64;
    host.charge(grid.len() * 96, scratch)?;
    let (direction, basins) =
        hydro::gp_d8_flow_dir_and_watershed_basins(&dem, grid.width, grid.height, &pour, scratch)
            .map_err(|e| format!("D8 basin routing: {e:?}"))?;
    let mut out = BTreeMap::from([("declared_basins".into(), Value::U64(pour.len() as u64))]);
    add_field(
        host,
        &mut out,
        "direction",
        grid.axes(),
        direction.into_iter().map(f32::from).collect(),
    )?;
    add_field(
        host,
        &mut out,
        "basin_id",
        grid.axes(),
        basins.into_iter().map(|v| v as f32).collect(),
    )?;
    Ok(out)
}

fn rusle(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "cell_size_m",
            "rainfall_r",
            "erodibility_k",
            "flow_accum_cells",
            "slope_rad",
            "cover_c",
            "practice_p",
        ],
    )?;
    let grid = Grid::parse(params)?;
    let rain = raster(host, required(params, "rainfall_r")?, grid, 0.0, 1e9)?;
    let erodibility = raster(host, required(params, "erodibility_k")?, grid, 0.0, 1e3)?;
    let flow = raster(host, required(params, "flow_accum_cells")?, grid, 0.0, 1e9)?;
    let slope = raster(
        host,
        required(params, "slope_rad")?,
        grid,
        0.0,
        std::f32::consts::FRAC_PI_2,
    )?;
    let cover = raster(host, required(params, "cover_c")?, grid, 0.0, 1.0)?;
    let practice = raster(host, required(params, "practice_p")?, grid, 0.0, 1.0)?;
    host.charge(grid.len() * 16, grid.len() * 32)?;
    let mut values = vec![0.0; grid.len()];
    hydro::gp_rusle_soil_loss_into(
        &rain,
        &erodibility,
        &flow,
        &slope,
        &cover,
        &practice,
        grid.spacing,
        &mut values,
    )
    .map_err(|e| format!("RUSLE soil loss: {e:?}"))?;
    let mut out = scalar_report(&values)?;
    add_field(host, &mut out, "soil_loss_tons_ha_yr", grid.axes(), values)?;
    Ok(out)
}

fn moran(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(params, &["samples", "bandwidth_m"])?;
    let points = points::<3>(params, "samples", MAX_POINTS)?;
    let bandwidth = number(params, "bandwidth_m", 1e-12, 1e9)?;
    let size = points.len();
    host.charge(size * size * 2, size * 64)?;
    let report = stats::gp_global_morans_i_and_gearys_c(&points, bandwidth)
        .map_err(|e| format!("Moran and Geary: {e:?}"))?;
    let (mut local, mut getis) = (vec![0.0; size], vec![0.0; size]);
    stats::gp_local_lisa_and_getis_ord_gi_star(&points, bandwidth, &mut local, &mut getis)
        .map_err(|e| format!("LISA and Getis-Ord: {e:?}"))?;
    let mut out = numeric([
        ("moran_i", report.moran_i),
        ("expected_i", report.expected_i),
        ("z_score", report.z_score),
        ("geary_c", report.geary_c),
    ])?;
    out.insert("local_lisa_i".into(), decimals(local)?);
    out.insert("getis_ord_gi_star_z".into(), decimals(getis)?);
    Ok(out)
}

fn gwr(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(params, &["samples", "bandwidth_m"])?;
    let points = points::<4>(params, "samples", MAX_POINTS)?;
    let bandwidth = number(params, "bandwidth_m", 1e-9, 1e9)?;
    let coords: Vec<_> = points.iter().map(|row| [row[0], row[1]]).collect();
    let x: Vec<_> = points.iter().map(|row| row[2]).collect();
    let y: Vec<_> = points.iter().map(|row| row[3]).collect();
    let scratch = points.len() * 96;
    host.charge(points.len() * points.len() * 2, scratch)?;
    let result = stats::gp_gwr_local_regression(&coords, &x, &y, bandwidth, scratch)
        .map_err(|e| format!("GWR: {e:?}"))?;
    let rows = result
        .into_iter()
        .map(|[intercept, slope, r2]| {
            numeric([
                ("intercept", intercept),
                ("slope", slope),
                ("r_squared", r2),
            ])
            .map(Value::Record)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BTreeMap::from([("rows".into(), Value::List(rows))]))
}

fn kriging(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &[
            "samples", "width", "height", "bbox_m", "model", "nugget", "sill", "range_m",
        ],
    )?;
    let points = points::<3>(params, "samples", 64)?;
    let width = dimension(params, "width", 64)?;
    let height = dimension(params, "height", 64)?;
    let bbox = vector::<4>(required(params, "bbox_m")?)?;
    if bbox[0] >= bbox[2] || bbox[1] >= bbox[3] || bbox.iter().any(|v| v.abs() > 1e9) {
        return Err("Kriging bbox requires increasing finite planar metre bounds".into());
    }
    let nugget = number(params, "nugget", 0.0, 1e15)?;
    let sill = number(params, "sill", 1e-12, 1e15)?;
    let range = number(params, "range_m", 1e-9, 1e9)?;
    let model = match text(required(params, "model")?)? {
        "spherical" => stats::VariogramModel::Spherical {
            nugget,
            sill,
            range,
        },
        "exponential" => stats::VariogramModel::Exponential {
            nugget,
            sill,
            range,
        },
        "gaussian" => stats::VariogramModel::Gaussian {
            nugget,
            sill,
            range,
        },
        _ => return Err("Unknown kriging variogram model".into()),
    };
    let scratch = (points.len() + 1).pow(2) * 32 + width * height * 32 + 4096;
    host.charge(width * height * (points.len() + 1).pow(3), scratch)?;
    let (means, variance) =
        stats::gp_ordinary_kriging_grid(&points, &model, bbox, width, height, scratch)
            .map_err(|e| format!("Ordinary kriging: {e:?}"))?;
    let mut out = numeric([("sill", sill), ("nugget", nugget), ("range_m", range)])?;
    let axes = vec![height as u32, width as u32];
    add_field(
        host,
        &mut out,
        "estimate",
        axes.clone(),
        means.into_iter().map(|v| v as f32).collect(),
    )?;
    add_field(
        host,
        &mut out,
        "variance",
        axes,
        variance.into_iter().map(|v| v as f32).collect(),
    )?;
    Ok(out)
}

fn ahp(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(params, &["pairwise"])?;
    let rows = bounded_list(required(params, "pairwise")?, 32)?;
    let size = rows.len();
    let mut matrix = Vec::with_capacity(size * size);
    for row in rows {
        let row = list(row)?;
        if row.len() != size {
            return Err("AHP matrix must be square".into());
        }
        for value in row {
            let value = num(value)?;
            if !(1.0 / 9.0..=9.0).contains(&value) {
                return Err("AHP comparisons must be in the Saaty scale 1/9..=9".into());
            }
            matrix.push(value);
        }
    }
    for i in 0..size {
        for j in i..size {
            if (matrix[i * size + j] * matrix[j * size + i] - 1.0).abs() > 1e-8 {
                return Err("AHP matrix must be reciprocal, with unit diagonal".into());
            }
        }
    }
    host.charge(size * size * 100, 16 * 1024)?;
    let result = algebra::gp_ahp_weights_and_consistency(&matrix, size, 16 * 1024)
        .map_err(|e| format!("Saaty AHP: {e:?}"))?;
    let mut out = numeric([
        ("lambda_max", result.lambda_max),
        ("consistency_index", result.consistency_index),
        ("consistency_ratio", result.consistency_ratio),
    ])?;
    out.insert("is_consistent".into(), Value::Bool(result.is_consistent));
    out.insert("weights".into(), decimals(result.weights)?);
    Ok(out)
}

fn network_stats(host: &mut AssetHost<'_>, params: &Args, op: &str) -> Result<Args, String> {
    if op == "edmonds_karp" {
        fields(params, &["node_count", "edges", "source", "sink"])?;
    } else {
        fields(params, &["node_count", "edges"])?;
    }
    let nodes = dimension(params, "node_count", 512)?;
    let rows = bounded_list(required(params, "edges")?, 4096)?;
    let mut edges = Vec::with_capacity(rows.len());
    for (id, row) in rows.iter().enumerate() {
        let [from, to, cost, capacity, one_way] = list(row)? else {
            return Err("Network edge requires [from, to, cost, capacity, one_way]".into());
        };
        let (from, to) = (index(from)?, index(to)?);
        let (cost, capacity) = (num(cost)?, num(capacity)?);
        let Value::Bool(one_way) = one_way else {
            return Err("Network one_way must be boolean".into());
        };
        if from >= nodes
            || to >= nodes
            || from == to
            || !(1e-9..=1e12).contains(&cost)
            || !(0.0..=1e12).contains(&capacity)
        {
            return Err("Invalid network endpoint, cost, or capacity".into());
        }
        edges.push(network::NetworkEdge {
            id: id as u32,
            from,
            to,
            base_cost: cost,
            capacity,
            one_way: *one_way,
        });
    }
    let scratch = nodes * 128 + edges.len() * 96 + 4096;
    if op == "edmonds_karp" {
        host.charge(nodes * edges.len() * edges.len(), scratch)?;
        let source = index(required(params, "source")?)?;
        let sink = index(required(params, "sink")?)?;
        let (flow, cut) = network::gp_max_flow_edmonds_karp(nodes, &edges, source, sink, scratch)
            .map_err(|e| format!("Edmonds-Karp: {e:?}"))?;
        let mut out = numeric([("max_flow", flow)])?;
        out.insert(
            "min_cut_source_nodes".into(),
            uvalues(cut.into_iter().map(|id| id as u64)),
        );
        Ok(out)
    } else {
        host.charge(nodes * edges.len() * 24, scratch)?;
        let (close, between) = network::gp_centrality_closeness_betweenness(nodes, &edges, scratch)
            .map_err(|e| format!("Brandes centrality: {e:?}"))?;
        Ok(BTreeMap::from([
            ("closeness".into(), decimals(close)?),
            ("betweenness".into(), decimals(between)?),
        ]))
    }
}

fn decode(host: &mut AssetHost<'_>, params: &Args, op: &str) -> Result<Args, String> {
    if op == "decode_zarr_f32" {
        fields(params, &["source", "metadata", "chunk_index"])?;
    } else {
        fields(params, &["source"])?;
    }
    let source = required(params, "source")?;
    let size = resource(host, source)?.bytes.len();
    if size == 0 || size > host.limits.resource_bytes.min(MAX_CHUNK_BYTES) {
        return Err("Datacube source exceeds its retained input byte budget or is empty".into());
    }
    host.charge(
        size + MAX_CELLS,
        size.min(MAX_ZARR_METADATA_BYTES) + MAX_CELLS * 8,
    )?;
    let bytes = &resource(host, source)?.bytes;
    let (axes, values, mut out) = match op {
        "decode_zarr_f32" => {
            let meta = &resource(host, required(params, "metadata")?)?.bytes;
            zarr(bytes, meta, required(params, "chunk_index")?)?
        }
        "decode_netcdf_f32" => netcdf(bytes)?,
        "decode_grib2" => grib(bytes)?,
        _ => return Err("Unknown datacube format".into()),
    };
    out.extend(scalar_report(&values)?);
    out.insert("axes".into(), uvalues(axes.iter().copied().map(u64::from)));
    add_field(host, &mut out, "field", axes, values)?;
    Ok(out)
}

type CubeSlab = (Vec<u32>, Vec<f32>, Args);

fn zarr(bytes: &[u8], metadata: &[u8], chunk: &Value) -> Result<CubeSlab, String> {
    if metadata.len() > MAX_ZARR_METADATA_BYTES {
        return Err("Zarr metadata exceeds its UTF-8 byte budget".into());
    }
    let metadata = std::str::from_utf8(metadata).map_err(|_| "Zarr metadata must be UTF-8")?;
    let doc: Json =
        serde_json::from_str(metadata).map_err(|error| format!("Zarr metadata: {error}"))?;
    let declared = cube::parse_zarr_array_meta(metadata, MAX_ZARR_METADATA_BYTES)
        .map_err(|e| format!("Zarr array metadata: {e:?}"))?;
    let version = doc["zarr_format"]
        .as_u64()
        .ok_or("Zarr metadata must declare a format version")?;
    if u64::from(declared.zarr_format) != version {
        return Err("Inconsistent Zarr version metadata".into());
    }
    let (chunks, little_endian, separator) = match version {
        2 => {
            if doc["order"] != "C"
                || !doc["compressor"].is_null()
                || doc["filters"]
                    .as_array()
                    .is_some_and(|values| !values.is_empty())
                || (!doc["filters"].is_null() && !doc["filters"].is_array())
            {
                return Err("Zarr 2 requires uncompressed, unfiltered C-order chunks".into());
            }
            let little = match doc["dtype"].as_str() {
                Some("<f4") => true,
                Some(">f4") => false,
                _ => return Err("Zarr 2 requires explicit little- or big-endian f32".into()),
            };
            let separator = match doc["dimension_separator"].as_str().unwrap_or(".") {
                "." => '.',
                "/" => '/',
                _ => return Err("Unsupported Zarr 2 chunk separator".into()),
            };
            (&doc["chunks"], little, separator)
        }
        3 => {
            if doc["node_type"] != "array"
                || doc["data_type"] != "float32"
                || doc["chunk_grid"]["name"] != "regular"
            {
                return Err("Zarr 3 requires a regular float32 array".into());
            }
            let codecs = doc["codecs"]
                .as_array()
                .ok_or("Zarr 3 requires an explicit bytes codec")?;
            if codecs.len() != 1
                || codecs[0]["name"] != "bytes"
                || doc["chunk_key_encoding"]["name"] != "default"
            {
                return Err(
                    "Zarr 3 supports the uncompressed bytes codec and default chunk keys".into(),
                );
            }
            let little = match codecs[0]["configuration"]["endian"].as_str() {
                Some("little") => true,
                Some("big") => false,
                _ => return Err("Unsupported Zarr 3 byte order".into()),
            };
            let separator = match doc["chunk_key_encoding"]["configuration"]["separator"]
                .as_str()
                .unwrap_or("/")
            {
                "." => '.',
                "/" => '/',
                _ => return Err("Unsupported Zarr 3 chunk separator".into()),
            };
            (
                &doc["chunk_grid"]["configuration"]["chunk_shape"],
                little,
                separator,
            )
        }
        _ => return Err("Unsupported Zarr array version".into()),
    };
    let shape = axes_json(&doc["shape"])?;
    let chunks = axes_json(chunks)?;
    let indices = list(chunk)?;
    if shape.len() != chunks.len() || shape.len() != indices.len() {
        return Err("Zarr chunk coordinates do not match array dimensions".into());
    }
    let nominal = cell_count(&chunks)?;
    if bytes.len() != nominal * 4 {
        return Err("Zarr chunk bytes do not match the declared uncompressed chunk extents".into());
    }
    let mut clipped = Vec::with_capacity(chunks.len());
    let mut coords = [0usize; 4];
    for axis in 0..shape.len() {
        let index = usize::try_from(exact(&indices[axis])?)
            .map_err(|_| "Zarr chunk index exceeds the platform")?;
        let full = chunks[axis] as usize;
        let count = shape[axis] as usize;
        if index >= count.div_ceil(full)
            || declared.shape[axis] != count
            || declared.chunks[axis] != full
        {
            return Err("Zarr chunk index is outside the array or metadata is inconsistent".into());
        }
        coords[axis] = index;
        clipped.push(full.min(count - index * full) as u32);
    }
    let key = cube::format_zarr_chunk_key(&declared, coords, separator, 128)
        .map_err(|e| format!("Zarr chunk key: {e:?}"))?;
    let mut values = vec![0.0; nominal];
    cube::decode_zarr_f32_chunk_into(bytes, little_endian, &mut values)
        .map_err(|e| format!("Zarr chunk decoding: {e:?}"))?;
    let values = if chunks == clipped {
        values
    } else {
        let active = cell_count(&clipped)?;
        (0..active)
            .map(|mut index| {
                let (mut source, mut stride) = (0, 1);
                for axis in (0..clipped.len()).rev() {
                    source += index % clipped[axis] as usize * stride;
                    index /= clipped[axis] as usize;
                    stride *= chunks[axis] as usize;
                }
                values[source]
            })
            .collect()
    };
    Ok((
        clipped,
        values,
        BTreeMap::from([
            ("chunk_key".into(), Value::String(key)),
            ("zarr_format".into(), Value::U64(version)),
        ]),
    ))
}

fn axes_json(value: &Json) -> Result<Vec<u32>, String> {
    let axes = value.as_array().ok_or("Datacube axes must be an array")?;
    if axes.is_empty() || axes.len() > 4 {
        return Err("Datacube axes require rank in 1..=4".into());
    }
    axes.iter()
        .map(|value| {
            value
                .as_u64()
                .and_then(|v| u32::try_from(v).ok())
                .filter(|&v| v > 0)
                .ok_or_else(|| "Datacube axis must be a positive exact u32".into())
        })
        .collect()
}

struct Nc<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Nc<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        let end = self
            .position
            .checked_add(count)
            .ok_or("NetCDF header extent overflow")?;
        let bytes = self
            .bytes
            .get(self.position..end)
            .ok_or("Truncated NetCDF header")?;
        self.position = end;
        Ok(bytes)
    }

    fn u32(&mut self) -> Result<u32, String> {
        self.take(4)
            .map(|v| u32::from_be_bytes(v.try_into().unwrap()))
    }

    fn count(&mut self, tag: u32, cap: u32) -> Result<u32, String> {
        let kind = self.u32()?;
        let count = self.u32()?;
        if (kind == 0 && count == 0) || (kind == tag && count <= cap) {
            Ok(count)
        } else {
            Err("Invalid or oversized NetCDF header table".into())
        }
    }

    fn aligned(&mut self, length: usize) -> Result<(), String> {
        let size = length
            .checked_add(3)
            .ok_or("NetCDF padded field length overflow")?
            & !3;
        self.take(size).map(|_| ())
    }

    fn name(&mut self) -> Result<(), String> {
        let length = self.u32()? as usize;
        if length == 0 || length > 4096 {
            return Err("NetCDF variable/dimension name is outside bounds".into());
        }
        self.aligned(length)
    }

    fn attributes(&mut self) -> Result<(), String> {
        for _ in 0..self.count(cube::NC_ATTRIBUTE, 512)? {
            self.name()?;
            let size = match self.u32()? {
                1 | 2 => 1usize,
                3 => 2,
                4 | 5 => 4,
                6 => 8,
                _ => return Err("Invalid NetCDF classic attribute scalar".into()),
            };
            let values = self.u32()? as usize;
            self.aligned(
                values
                    .checked_mul(size)
                    .ok_or("NetCDF attribute size overflow")?,
            )?;
        }
        Ok(())
    }
}

fn netcdf(bytes: &[u8]) -> Result<CubeSlab, String> {
    let mut scan = Nc { bytes, position: 0 };
    let magic = scan.take(4)?;
    if magic != cube::NETCDF_CLASSIC_MAGIC && magic != cube::NETCDF_64BIT_OFFSET_MAGIC {
        return Err("NetCDF requires classic CDF-1 or CDF-2".into());
    }
    let record_count = scan.u32()?;
    let mut dims = Vec::new();
    for _ in 0..scan.count(cube::NC_DIMENSION, 64)? {
        scan.name()?;
        dims.push(scan.u32()?);
    }
    scan.attributes()?;
    let mut axes = Vec::new();
    let mut offset = 0;
    let mut first_size = 0;
    for index in 0..scan.count(cube::NC_VARIABLE, 256)? {
        scan.name()?;
        let rank = scan.u32()?;
        if rank > 64 {
            return Err("NetCDF variable rank exceeds its header budget".into());
        }
        for _ in 0..rank {
            let dimension = *dims
                .get(scan.u32()? as usize)
                .ok_or("NetCDF variable references an unknown dimension")?;
            if index == 0 {
                axes.push(dimension);
            }
        }
        scan.attributes()?;
        let scalar = scan.u32()?;
        let size = scan.u32()?;
        let start = if magic == cube::NETCDF_CLASSIC_MAGIC {
            u64::from(scan.u32()?)
        } else {
            u64::from_be_bytes(scan.take(8)?.try_into().unwrap())
        };
        if index == 0 {
            if scalar != 5 || axes.is_empty() || axes.len() > 4 || axes.contains(&0) {
                return Err("NetCDF first variable must be a bounded fixed-size f32 slab".into());
            }
            offset =
                usize::try_from(start).map_err(|_| "NetCDF slab offset exceeds the platform")?;
            first_size = size as usize;
        }
    }
    let count = cell_count(&axes)?;
    if offset < scan.position || first_size != count * 4 {
        return Err("NetCDF f32 slab overlaps its header or has an inconsistent extent".into());
    }
    let summary = cube::parse_netcdf_classic_header(bytes)
        .map_err(|e| format!("NetCDF classic header: {e:?}"))?;
    if summary.first_var_offset != offset as u64
        || summary.first_var_vsize as usize != first_size
        || summary.num_recs != record_count
    {
        return Err("NetCDF summary differs from checked variable metadata".into());
    }
    let mut values = vec![0.0; count];
    cube::read_netcdf_f32_slab_into(bytes, offset, count, &mut values)
        .map_err(|e| format!("NetCDF f32 slab: {e:?}"))?;
    Ok((
        axes,
        values,
        BTreeMap::from([
            ("record_count".into(), Value::U64(u64::from(record_count))),
            (
                "variable_count".into(),
                Value::U64(u64::from(summary.var_count)),
            ),
        ]),
    ))
}

fn grib(bytes: &[u8]) -> Result<CubeSlab, String> {
    let summary = cube::parse_grib2_sections(bytes).map_err(|e| format!("GRIB2 header: {e:?}"))?;
    if bytes.len() < 20 || summary.total_length != bytes.len() as u64 {
        return Err("GRIB2 input must be exactly one complete message".into());
    }
    let count = cell_count(&[summary.ny, summary.nx])?;
    let tail = bytes.len() - 4;
    let mut sections: Vec<(u8, &[u8])> = Vec::new();
    let mut position = 16;
    while position < tail {
        let header = bytes
            .get(position..position + 5)
            .ok_or("Truncated GRIB2 section")?;
        let size = u32::from_be_bytes(header[..4].try_into().unwrap()) as usize;
        let end = position
            .checked_add(size)
            .ok_or("GRIB2 section extent overflow")?;
        if size < 5 || end > tail || sections.len() >= 7 {
            return Err("Invalid GRIB2 section layout".into());
        }
        sections.push((header[4], &bytes[position..end]));
        position = end;
    }
    let order: Vec<_> = sections.iter().map(|(id, _)| *id).collect();
    if !matches!(order.as_slice(), [1, 3, 4, 5, 6, 7] | [1, 2, 3, 4, 5, 6, 7]) {
        return Err("GRIB2 expects one regular grid and one field".into());
    }
    let get = |id| {
        sections
            .iter()
            .find(|(section, _)| *section == id)
            .map(|(_, bytes)| *bytes)
    };
    let grid = get(3).ok_or("Missing GRIB2 grid section")?;
    let packing = get(5).ok_or("Missing GRIB2 representation section")?;
    let bitmap = get(6).ok_or("Missing GRIB2 bitmap section")?;
    let values = get(7).ok_or("Missing GRIB2 data section")?;
    if grid.len() < 72
        || grid[12..14] != [0, 0]
        || grid[71] & 0x30 != 0
        || u32::from_be_bytes(grid[6..10].try_into().unwrap()) as usize != count
        || packing.len() < 21
        || packing[9..11] != [0, 0]
        || u32::from_be_bytes(packing[5..9].try_into().unwrap()) as usize != count
        || bitmap.len() != 6
        || bitmap[5] != 255
        || summary.bits_per_value > 32
        || (count * usize::from(summary.bits_per_value)).div_ceil(8) != values.len() - 5
    {
        return Err("GRIB2 requires regular row-wise latitude/longitude grid, simple packing, and no bitmap".into());
    }
    let mut output = vec![0.0; count];
    let written = cube::unpack_grib2_simple_packing_into(bytes, &summary, &mut output)
        .map_err(|e| format!("GRIB2 field decoding: {e:?}"))?;
    if written != count {
        return Err("GRIB2 decoder returned an incomplete slab".into());
    }
    Ok((
        vec![summary.ny, summary.nx],
        output,
        BTreeMap::from([
            ("year".into(), Value::U64(u64::from(summary.year))),
            ("month".into(), Value::U64(u64::from(summary.month))),
            ("day".into(), Value::U64(u64::from(summary.day))),
            ("hour".into(), Value::U64(u64::from(summary.hour))),
            ("scanning_mode".into(), Value::U64(u64::from(grid[71]))),
        ]),
    ))
}

fn seal_raster(host: &mut AssetHost<'_>, params: &Args) -> Result<Args, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "cell_size_m",
            "field",
            "origin_m",
            "entity_iri",
            "source_iri",
            "name_iri",
            "unit_iri",
        ],
    )?;
    let grid = Grid::parse(params)?;
    if grid.width < 2 || grid.height < 2 {
        return Err("A spatial datacube raster needs at least a 2×2 authored footprint".into());
    }
    let values = raster(host, required(params, "field")?, grid, -1e18, 1e18)?;
    let entity = iri(required(params, "entity_iri")?)?;
    let source_iri = iri(required(params, "source_iri")?)?;
    let name = iri(required(params, "name_iri")?)?;
    let unit = iri(required(params, "unit_iri")?)?;
    let origin = vector::<3>(required(params, "origin_m")?)?;
    let [east, up, north] = origin.map(|v| v as f32);
    let east_end = east + (grid.width - 1) as f32 * grid.spacing;
    let north_end = north + (grid.height - 1) as f32 * grid.spacing;
    if [east, up, north, east_end, north_end]
        .iter()
        .any(|v| !v.is_finite() || v.abs() > 1e6)
        || east >= east_end
        || north >= north_end
    {
        return Err("Raster footprint exceeds finite local metre precision".into());
    }
    host.charge(grid.len() * 16, values.len() * 16 + 128 * 1024)?;
    let render_mesh = Mesh {
        positions: vec![
            [east, up, north],
            [east_end, up, north],
            [east_end, up, north_end],
            [east, up, north_end],
        ],
        triangles: vec![[0, 2, 1], [0, 3, 2]],
        min: [east, up, north],
        max: [east_end, up, north_end],
    };
    let envelope =
        compile_mesh_to_10d(&render_mesh).map_err(|e| format!("10D raster footprint: {e:?}"))?;
    let [x, y, z] = render_mesh.centroid();
    let envelope = upgrade_legacy_10d_to_holographic_v2(
        &envelope,
        entity,
        1,
        [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
    )
    .map_err(|e| format!("HOLO raster footprint: {e:?}"))?;

    let mut payload = vec![
        0;
        fiel::FIELD_SECTION_HEADER_SIZE
            + fiel::FIELD_DESCRIPTOR_SIZE
            + fiel::RASTER_GRID_METADATA_SIZE
            + grid.len() * 4
    ];
    let written = fiel::encode_field_section_with_rasters(
        &[fiel::ContinuousRasterField {
            name_hash: q_hash(name),
            unit_hash: q_hash(unit),
            width: grid.width as u32,
            height: grid.height as u32,
            samples: &values,
        }],
        &mut payload,
    )
    .map_err(|e| format!("FIEL raster: {e:?}"))?;
    payload.truncate(written);
    let semantic = raster_metadata(entity, source_iri, name, unit)?;
    let asset = attach_raster(&envelope, &payload, &semantic, host.limits.resource_bytes)?;
    let mut out = scalar_report(&values)?;
    out.insert(
        "data_digest".into(),
        Value::String(blake3::hash(&asset).to_hex().to_string()),
    );
    out.insert("asset".into(), host.encode("10d", asset)?);
    Ok(out)
}

fn raster_metadata(entity: &str, source: &str, field: &str, unit: &str) -> Result<Vec<u8>, String> {
    const CONTEXT: &str = "urn:qualia:gis:raster-layer:v1";
    const HAS_FIELD: &str = "urn:qualia:gis:hasField";
    const UNIT: &str = "http://qudt.org/schema/qudt/unit";
    const SOURCE: &str = "http://purl.org/dc/terms/source";
    let mut terms = BTreeMap::new();
    for term in [
        entity, source, field, unit, CONTEXT, HAS_FIELD, UNIT, SOURCE,
    ] {
        let hash = q_hash(term);
        if terms
            .insert(hash, term)
            .is_some_and(|previous| previous != term)
        {
            return Err("Raster semantic IRIs collide in the q_hash mapping".into());
        }
    }
    let lexicon = terms
        .into_iter()
        .map(|(term_hash, iri)| ten::MultilingualLexicalTerm {
            term_hash,
            iri_utf8: iri.into(),
            lang_bcp47: None,
            label_utf8: String::new(),
        })
        .collect();
    let nquins = [
        [entity, HAS_FIELD, field],
        [field, UNIT, unit],
        [field, SOURCE, source],
    ]
    .into_iter()
    .map(|[s, p, o]| {
        let [s, p, o, c] = [s, p, o, CONTEXT].map(q_hash);
        [s, p, o, c, 0, NQuin::calculate_parity(s, p, o, c, 0)]
    })
    .collect();
    ten::encode_symbolic_rdf_cbor_ld(&ten::SymbolicRdfCborLdPayload {
        context_iri: CONTEXT.into(),
        lexicon,
        nquins,
        ..Default::default()
    })
    .map_err(|e| format!("Raster CBLD: {e:?}"))
}

fn attach_raster(
    base: &[u8],
    payload: &[u8],
    semantic: &[u8],
    budget: usize,
) -> Result<Vec<u8>, String> {
    let header = ten::Container10dHeader::parse(base).map_err(|e| e.to_string())?;
    let mut scratch = [ten::TypedFieldRecord::default(); ten::MANIFOLD_IDENTITY_V2_MAX_FIELDS];
    let mut list = scratch;
    let view = ten::TypedPayloadResolver::new(base, &[], &mut scratch, &mut list)
        .map_err(|e| e.to_string())?;
    let identity = view.identity().ok_or("Raster footprint requires MID2")?;
    let mut sidecar = view
        .section_payload(ten::SectionType::FieldSidecar)
        .ok_or("Raster footprint has no field sidecar")?
        .to_vec();
    let mut records = view.fields().to_vec();
    let mut next = records.last().map_or(0, |record| record.field_id);
    for (kind, bytes) in [
        (ten::manifold_identity_v2::FIELD_KIND_SCALAR, payload),
        (ten::FIELD_KIND_SYMBOLIC_RDF_CBOR_LD, semantic),
    ] {
        next = next.checked_add(1).ok_or("10D field ID overflow")?;
        records.push(ten::TypedFieldRecord {
            field_id: next,
            kind,
            payload_section_id: ten::SectionType::FieldSidecar as u32,
            payload_offset: sidecar.len() as u64,
            payload_length: bytes.len() as u64,
            ..Default::default()
        });
        sidecar.extend_from_slice(bytes);
    }
    let length = ten::manifold_identity_v2_encoded_len(records.len())
        .map_err(|e| format!("Raster MID2: {e:?}"))?;
    let mut manifest = vec![0; length];
    ten::encode_manifold_identity_v2(identity, &records, &mut manifest)
        .map_err(|e| format!("Raster MID2: {e:?}"))?;
    let table = ten::parse_section_table(base, &header).map_err(|e| e.to_string())?;
    let mut sections = Vec::with_capacity(table.len());
    for descriptor in table {
        let section_type = descriptor.typ().ok_or("Unknown base 10D section")?;
        let bytes: &[u8] = match section_type {
            ten::SectionType::FieldSidecar => &sidecar,
            ten::SectionType::ManifoldIdentityV2 => &manifest,
            _ => view
                .section_payload(section_type)
                .ok_or("Missing base 10D section bytes")?,
        };
        sections.push(ten::SectionInput {
            section_type,
            alignment_tier: ten::AlignmentTier::from_u8(descriptor.alignment_tier)
                .ok_or("Invalid raster section alignment")?,
            stride: descriptor.stride,
            element_count: descriptor.element_count,
            payload: bytes,
        });
    }
    let capacity = sections
        .iter()
        .try_fold((sections.len() + 1) * 4096, |size, section| {
            size.checked_add(section.payload.len())
        })
        .ok_or("Raster 10D capacity overflow")?;
    if capacity > budget.min(128 * 1024 * 1024) {
        return Err("Raster asset exceeds the 10D ring budget".into());
    }
    let mut out = vec![0; capacity];
    let written = ten::encode_container(&header, &sections, &mut out).map_err(|e| e.to_string())?;
    out.truncate(written);
    ten::seal_whole_file_crc32c(&mut out);
    Ok(out)
}

fn resource<'a>(host: &'a AssetHost<'_>, value: &Value) -> Result<&'a Resource, String> {
    match value {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|source| &source.key == key)
            .ok_or_else(|| "Unknown retained GIS resource".into()),
        _ => host.workspace.encoded(value),
    }
}

fn iri(value: &Value) -> Result<&str, String> {
    let value = text(value)?;
    let (scheme, rest) = value
        .split_once(':')
        .ok_or("Raster semantics require absolute RDF IRIs")?;
    if value.len() > 4096
        || !scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        || !scheme
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"+.-".contains(&byte))
        || rest.is_empty()
        || rest
            .chars()
            .any(|c| c.is_control() || c.is_whitespace() || "<>{}|\\^\"".contains(c))
        || q_hash(value) == 0
    {
        return Err("Invalid raster RDF IRI".into());
    }
    Ok(value)
}

fn exact(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) => {
            u64::try_from(*value).map_err(|_| "Expected nonnegative exact integer".into())
        }
        _ => Err("Expected nonnegative exact integer".into()),
    }
}

fn index(value: &Value) -> Result<usize, String> {
    usize::try_from(exact(value)?).map_err(|_| "GIS index exceeds the platform".into())
}

fn dimension(params: &Args, key: &str, max: usize) -> Result<usize, String> {
    let value = exact(required(params, key)?)?;
    if value == 0 || value > max as u64 {
        return Err(format!("GIS {key} must be in 1..={max}"));
    }
    Ok(value as usize)
}

fn number(params: &Args, key: &str, low: f64, high: f64) -> Result<f64, String> {
    let value = num(required(params, key)?)?;
    if !(low..=high).contains(&value) {
        return Err(format!("GIS {key} is outside {low}..={high}"));
    }
    Ok(value)
}

fn bounded_list(value: &Value, cap: usize) -> Result<&[Value], String> {
    let values = list(value)?;
    if values.is_empty() || values.len() > cap {
        return Err(format!("GIS list requires 1..={cap} entries"));
    }
    Ok(values)
}

fn cell_count(axes: &[u32]) -> Result<usize, String> {
    if axes.is_empty() || axes.contains(&0) {
        return Err("Datacube axes must be nonempty and positive".into());
    }
    axes.iter()
        .try_fold(1usize, |count, &axis| count.checked_mul(axis as usize))
        .filter(|&count| count <= MAX_CELLS)
        .ok_or_else(|| "Datacube slab exceeds its bounded sample count".into())
}

fn raster(
    host: &AssetHost<'_>,
    value: &Value,
    grid: Grid,
    low: f32,
    high: f32,
) -> Result<Vec<f32>, String> {
    let values = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => match field.as_ref() {
            FieldLattice::DenseF32 { axes, samples }
                if axes
                    .iter()
                    .copied()
                    .skip_while(|&size| size == 1)
                    .collect::<Vec<_>>()
                    == grid
                        .axes()
                        .into_iter()
                        .skip_while(|&size| size == 1)
                        .collect::<Vec<_>>()
                    && samples.len() == grid.len() =>
            {
                samples.clone()
            }
            _ => return Err("Typed GIS raster shape differs from the authored grid".into()),
        },
        _ => {
            let values = if let Value::Record(fields) = value {
                list(required(fields, "samples")?)?
            } else {
                list(value)?
            };
            if values.len() != grid.len() {
                return Err("GIS raster sample count differs from grid dimensions".into());
            }
            values
                .iter()
                .map(|sample| num(sample).map(|v| v as f32))
                .collect::<Result<Vec<_>, _>>()?
        }
    };
    if values.iter().any(|value| !(low..=high).contains(value)) {
        return Err("GIS raster contains non-finite or out-of-range values".into());
    }
    Ok(values)
}

fn points<const N: usize>(params: &Args, key: &str, cap: usize) -> Result<Vec<[f64; N]>, String> {
    bounded_list(required(params, key)?, cap)?
        .iter()
        .map(|value| {
            let point = vector::<N>(value)?;
            if point.iter().any(|v| v.abs() > 1e12) {
                return Err("GIS sample exceeds bounded planar/model coordinates".into());
            }
            Ok(point)
        })
        .collect()
}

fn numeric<const N: usize>(values: [(&str, f64); N]) -> Result<Args, String> {
    values
        .into_iter()
        .map(|(key, value)| {
            if !value.is_finite() {
                return Err(format!("GIS {key} produced a non-finite value"));
            }
            Ok((key.into(), Value::F64(value)))
        })
        .collect()
}

fn decimals(values: impl IntoIterator<Item = f64>) -> Result<Value, String> {
    Ok(Value::List(
        values
            .into_iter()
            .map(|value| {
                if !value.is_finite() {
                    return Err("GIS native statistic is non-finite".into());
                }
                Ok(Value::F64(value))
            })
            .collect::<Result<Vec<_>, String>>()?,
    ))
}

fn uvalues(values: impl IntoIterator<Item = u64>) -> Value {
    Value::List(values.into_iter().map(Value::U64).collect())
}

fn scalar_report(values: &[f32]) -> Result<Args, String> {
    if values.is_empty() || values.iter().any(|v| !v.is_finite()) {
        return Err("GIS raster returns non-finite or empty samples".into());
    }
    let (low, high, total) = values.iter().copied().map(f64::from).fold(
        (f64::INFINITY, f64::NEG_INFINITY, 0.0),
        |(low, high, total), value| (low.min(value), high.max(value), total + value),
    );
    let mut out = numeric([
        ("min", low),
        ("max", high),
        ("mean", total / values.len() as f64),
    ])?;
    out.insert("cells".into(), Value::U64(values.len() as u64));
    Ok(out)
}

fn add_field(
    host: &mut AssetHost<'_>,
    out: &mut Args,
    name: &str,
    axes: Vec<u32>,
    samples: Vec<f32>,
) -> Result<(), String> {
    if cell_count(&axes)? != samples.len() || samples.iter().any(|v| !v.is_finite()) {
        return Err("GIS output field is non-finite or mismatched to its dimensions".into());
    }
    let mut digest = blake3::Hasher::new();
    for axis in &axes {
        digest.update(&axis.to_le_bytes());
    }
    for sample in &samples {
        digest.update(&sample.to_le_bytes());
    }
    let digest = digest.finalize().to_hex().to_string();
    let field = host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 { axes, samples },
    )))?;
    out.insert(format!("{name}_digest"), Value::String(digest));
    out.insert(name.into(), field);
    Ok(())
}

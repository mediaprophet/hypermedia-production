//! Production access to bounded native raster and vector geoprocessing.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::sync::Arc;

use crate::artifact::FieldLattice;
use crate::geodesy_ops::{heightfield_record, parse_heightfield};
use crate::*;
use geodesy_spatial_analysis::cartography::{ClassificationMethod, compute_class_breaks};
use geodesy_spatial_analysis::{DemGrid, Extent, compute_hillshade};
use qualia_core_db::domains::geospatial::gis_gp::{
    BufferJoinStyle, NetworkEdge, NetworkNode, ObserverSpec, gp_buffer_polygon, gp_buffer_polyline,
    gp_d8_flow_dir_and_watershed_basins, gp_dissolve_by_key, gp_service_area_isochrone_nodes,
    gp_viewshed_radial_sweep, ring_area,
};

impl AssetHost<'_> {
    pub(crate) fn gp_spatial_analyze(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let options = record(required(args, "parameters")?)?;
        let result = match text(required(args, "op")?)? {
            "buffer" => {
                fields(options, &["points", "geometry", "distance_m", "join"])?;
                let geometry = text(required(options, "geometry")?)?;
                let vertices = points(required(options, "points")?, 4096)?;
                let distance = num(required(options, "distance_m")?)?;
                if distance <= 0.0 || distance > 100_000.0 {
                    return Err("Buffer distance must be positive and at most 100000 m".into());
                }
                let join = match options
                    .get("join")
                    .map(text)
                    .transpose()?
                    .unwrap_or("round")
                {
                    "round" => BufferJoinStyle::Round(16),
                    "bevel" => BufferJoinStyle::Bevel,
                    "miter" => BufferJoinStyle::Miter(4.0),
                    _ => return Err("Unknown buffer join style".into()),
                };
                self.charge(vertices.len() * 512, vertices.len() * 2048)?;
                let limit = self.limits.resource_bytes;
                let ring = match geometry {
                    "polygon" => gp_buffer_polygon(&vertices, distance, join, limit),
                    "polyline" => gp_buffer_polyline(&vertices, distance, join, limit),
                    _ => return Err("Buffer geometry must be polygon or polyline".into()),
                }
                .map_err(|error| format!("Native buffer: {error:?}"))?;
                let area = ring_area(&ring);
                Args::from([
                    ("ring".into(), ring_value(ring)),
                    ("area_m2".into(), Value::F64(area)),
                ])
            }
            "dissolve" => {
                fields(options, &["features", "snap_tolerance_m"])?;
                let values = list(required(options, "features")?)?;
                if !(1..=128).contains(&values.len()) {
                    return Err("Dissolve needs 1 to 128 polygon features".into());
                }
                let tolerance = num(required(options, "snap_tolerance_m")?)?;
                if !(1e-7..=100.0).contains(&tolerance) {
                    return Err(
                        "Dissolve snap tolerance is outside the admitted metre interval".into(),
                    );
                }
                let mut polygons = Vec::with_capacity(values.len());
                let mut total = 0usize;
                for value in values {
                    let feature = record(value)?;
                    fields(feature, &["key", "ring"])?;
                    let key = unsigned(required(feature, "key")?)?;
                    let ring = points(required(feature, "ring")?, 4096)?;
                    total += ring.len();
                    if ring.len() < 3 || total > 16_384 {
                        return Err("Dissolve polygon total falls outside its vertex budget".into());
                    }
                    polygons.push((key, ring));
                }
                self.charge(total.saturating_mul(total).saturating_mul(4), total * 256)?;
                let mut dissolved =
                    gp_dissolve_by_key(&polygons, tolerance, self.limits.resource_bytes)
                        .map_err(|error| format!("Native dissolve: {error:?}"))?;
                for (_, ring) in &mut dissolved {
                    canonical_ring(ring);
                }
                dissolved.sort_by(|(ak, a), (bk, b)| {
                    ak.cmp(bk).then_with(|| {
                        a.iter()
                            .flatten()
                            .zip(b.iter().flatten())
                            .map(|(a, b)| a.total_cmp(b))
                            .find(|&cmp| cmp != std::cmp::Ordering::Equal)
                            .unwrap_or_else(|| a.len().cmp(&b.len()))
                    })
                });
                let features = dissolved
                    .into_iter()
                    .map(|(key, ring)| {
                        Value::Record(Args::from([
                            ("key".into(), Value::U64(key)),
                            ("ring".into(), ring_value(ring)),
                        ]))
                    })
                    .collect();
                Args::from([("features".into(), Value::List(features))])
            }
            "hydro_d8" | "watershed" => {
                fields(options, &["heightfield", "pour_points"])?;
                let (side, _, _, dem) = parse_heightfield(required(options, "heightfield")?)?;
                let mut pour_points = Vec::new();
                let mut labels = std::collections::BTreeSet::new();
                let mut cells = std::collections::BTreeSet::new();
                for value in options
                    .get("pour_points")
                    .map(list)
                    .transpose()?
                    .unwrap_or(&[])
                {
                    let pour = record(value)?;
                    fields(pour, &["column", "row", "label"])?;
                    let col = integer(required(pour, "column")?, "column")?;
                    let row = integer(required(pour, "row")?, "row")?;
                    let label = u32::try_from(unsigned(required(pour, "label")?)?)
                        .map_err(|_| "Watershed label exceeds u32")?;
                    if col >= side
                        || row >= side
                        || !(1..=16_777_216).contains(&label)
                        || !labels.insert(label)
                        || !cells.insert((col, row))
                        || labels.len() > 256
                    {
                        return Err(
                            "Watershed pour points require distinct cells and nonzero exact labels"
                                .into(),
                        );
                    }
                    pour_points.push((col, row, label));
                }
                self.charge(
                    side * side * (64 + pour_points.len() * 16),
                    side * side * 96,
                )?;
                let (direction, basins) = gp_d8_flow_dir_and_watershed_basins(
                    &dem,
                    side,
                    side,
                    &pour_points,
                    self.limits.resource_bytes,
                )
                .map_err(|error| format!("Native watershed: {error:?}"))?;
                let dir = raster(self, side, direction.into_iter().map(f32::from).collect())?;
                let basins = raster(self, side, basins.into_iter().map(|v| v as f32).collect())?;
                Args::from([
                    ("directions".into(), dir),
                    ("basins".into(), basins),
                    ("sink_code".into(), Value::U64(8)),
                ])
            }
            "fill_lakes" => {
                fields(options, &["heightfield"])?;
                let (side, spacing, origin, dem) =
                    parse_heightfield(required(options, "heightfield")?)?;
                self.charge(side * side * 64, side * side * 96)?;
                let mut filled = dem.clone();
                terrain_erosion_solvers::priority_flood_fill(&mut filled, side, side);
                let depth = filled
                    .iter()
                    .zip(dem)
                    .map(|(&h, base)| (h - base).max(0.0))
                    .collect();
                Args::from([
                    (
                        "heightfield".into(),
                        heightfield_record(side, spacing, origin, &filled),
                    ),
                    ("depression_depth_m".into(), raster(self, side, depth)?),
                ])
            }
            "viewshed" => {
                fields(
                    options,
                    &[
                        "heightfield",
                        "column",
                        "row",
                        "observer_height_m",
                        "target_height_m",
                        "radius_m",
                        "refraction_k",
                    ],
                )?;
                let (side, spacing, _, dem) = parse_heightfield(required(options, "heightfield")?)?;
                let radius = num(required(options, "radius_m")?)?;
                let cols = (radius / spacing).ceil();
                let observer_offset = num(required(options, "observer_height_m")?)?;
                let target_offset = number(options, "target_height_m", 0.0)?;
                let refraction = number(options, "refraction_k", 0.0)?;
                if radius <= 0.0
                    || cols > 512.0
                    || !(0.0..=100_000.0).contains(&observer_offset)
                    || !(0.0..=100_000.0).contains(&target_offset)
                    || !(0.0..=1.0).contains(&refraction)
                {
                    return Err("Viewshed radius, height, or refraction exceeds its physical or work bounds".into());
                }
                let observer = ObserverSpec {
                    col: integer(required(options, "column")?, "column")?,
                    row: integer(required(options, "row")?, "row")?,
                    observer_offset_m: observer_offset as f32,
                    target_offset_m: target_offset as f32,
                    max_radius_m: radius as f32,
                    earth_curvature_refraction_k: refraction as f32,
                };
                let cols = cols as usize;
                self.charge((cols * 13).max(360) * (cols * 2 + 2) * 8, side * side * 24)?;
                let visibility = gp_viewshed_radial_sweep(
                    &dem,
                    side,
                    side,
                    spacing as f32,
                    &observer,
                    self.limits.resource_bytes,
                )
                .map_err(|error| format!("Native viewshed: {error:?}"))?;
                let visible = visibility.iter().filter(|&&v| v != 0).count();
                Args::from([
                    ("visible_cells".into(), Value::U64(visible as u64)),
                    (
                        "visibility".into(),
                        raster(self, side, visibility.into_iter().map(f32::from).collect())?,
                    ),
                ])
            }
            "hillshade" => {
                fields(options, &["heightfield", "azimuth_deg", "altitude_deg"])?;
                let (side, spacing, origin, terrain) =
                    parse_heightfield(required(options, "heightfield")?)?;
                let azimuth = num(required(options, "azimuth_deg")?)?;
                let altitude = num(required(options, "altitude_deg")?)?;
                if !(0.0..360.0).contains(&azimuth) || !(0.0..=90.0).contains(&altitude) {
                    return Err(
                        "Hillshade expects a compass azimuth and an altitude from 0 to 90 degrees"
                            .into(),
                    );
                }
                self.charge(side * side * 96, side * side * 32)?;
                let size = spacing * (side - 1) as f64;
                let dem = DemGrid::new(
                    side,
                    side,
                    spacing,
                    spacing,
                    Extent {
                        min_x: origin[0],
                        min_y: origin[1],
                        max_x: origin[0] + size,
                        max_y: origin[1] + size,
                    },
                    f64::NAN,
                    terrain.into_iter().map(f64::from).collect(),
                );
                let shade = compute_hillshade(&dem, azimuth, altitude, 1.0);
                Args::from([(
                    "hillshade".into(),
                    raster(
                        self,
                        side,
                        shade.into_iter().map(|v| f32::from(v) / 255.0).collect(),
                    )?,
                )])
            }
            "isochrone" => {
                fields(options, &["nodes", "edges", "origins", "max_cost"])?;
                let points = list(required(options, "nodes")?)?;
                let links = list(required(options, "edges")?)?;
                let starts = list(required(options, "origins")?)?;
                let max_cost = num(required(options, "max_cost")?)?;
                if !(1..=4096).contains(&points.len())
                    || links.len() > 16384
                    || !(1..=points.len()).contains(&starts.len())
                    || !(0.0..=1e12).contains(&max_cost)
                {
                    return Err(
                        "Service-area graph, origins or cost exceeds the admitted range".into(),
                    );
                }
                self.charge(
                    (points.len() + links.len()) * 128,
                    (points.len() + links.len()) * 192,
                )?;
                let nodes = points
                    .iter()
                    .enumerate()
                    .map(|(id, value)| {
                        let [x, y, z_m] = vector(value)?;
                        Ok(NetworkNode {
                            id: id as u32,
                            x,
                            y,
                            z_m,
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                let mut edges = Vec::with_capacity(links.len());
                for (id, value) in links.iter().enumerate() {
                    let row = record(value)?;
                    fields(row, &["from", "to", "cost", "one_way"])?;
                    let base_cost = num(required(row, "cost")?)?;
                    if !(0.0..=1e12).contains(&base_cost) {
                        return Err("Service-area costs must be finite and nonnegative".into());
                    }
                    edges.push(NetworkEdge {
                        id: id as u32,
                        base_cost,
                        capacity: 1.0,
                        from: integer(required(row, "from")?, "from")?,
                        to: integer(required(row, "to")?, "to")?,
                        one_way: boolean(row, "one_way", false)?,
                    });
                }
                let origins = starts
                    .iter()
                    .map(|value| integer(value, "origin"))
                    .collect::<Result<Vec<_>, _>>()?;
                let reachable = gp_service_area_isochrone_nodes(
                    &nodes,
                    &edges,
                    &origins,
                    max_cost,
                    self.limits.resource_bytes,
                )
                .map_err(|error| error.to_string())?;
                let reachable = reachable
                    .into_iter()
                    .map(|(node, cost)| {
                        Value::Record(Args::from([
                            ("node".into(), Value::U64(node as u64)),
                            ("cost".into(), Value::F64(cost)),
                        ]))
                    })
                    .collect();
                Args::from([("reachable".into(), Value::List(reachable))])
            }
            "fisher_jenks" => {
                fields(options, &["values", "classes"])?;
                let samples = list(required(options, "values")?)?;
                let classes = count(options, "classes", 5, 32)?;
                if !(classes..=2048).contains(&samples.len()) {
                    return Err("Fisher-Jenks requires at least one sample per class and at most 2048 samples".into());
                }
                let values = samples.iter().map(num).collect::<Result<Vec<_>, _>>()?;
                if values.iter().any(|v| v.abs() > 1e100) {
                    return Err("Classification samples exceed safe variance precision".into());
                }
                self.charge(
                    samples.len() * samples.len() * classes * 8,
                    samples.len() * classes * 24,
                )?;
                let breaks =
                    compute_class_breaks(&values, classes, ClassificationMethod::NaturalBreaks);
                Args::from([(
                    "breaks".into(),
                    Value::List(breaks.into_iter().map(Value::F64).collect()),
                )])
            }
            operator => return Err(format!("Unknown geoprocessing operation {operator}")),
        };
        Ok(Value::Record(result))
    }
}

fn points(value: &Value, max: usize) -> Result<Vec<[f64; 2]>, String> {
    let rows = list(value)?;
    if !(2..=max).contains(&rows.len()) {
        return Err("Invalid geometry vertex count".into());
    }
    let points = rows
        .iter()
        .map(vector::<2>)
        .collect::<Result<Vec<_>, _>>()?;
    if points.iter().flatten().any(|v| v.abs() > 10_000_000.0) {
        return Err("Geometry exceeds projected metre coordinate range".into());
    }
    Ok(points)
}

fn ring_value(ring: Vec<[f64; 2]>) -> Value {
    Value::List(
        ring.into_iter()
            .map(|point| Value::List(point.into_iter().map(Value::F64).collect()))
            .collect(),
    )
}

fn canonical_ring(ring: &mut Vec<[f64; 2]>) {
    let closed = ring.len() > 1 && ring.first() == ring.last();
    if closed {
        ring.pop();
    }
    if let Some((index, _)) = ring
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a[0].total_cmp(&b[0]).then(a[1].total_cmp(&b[1])))
    {
        ring.rotate_left(index);
    }
    if closed {
        ring.push(ring[0]);
    }
}

fn unsigned(value: &Value) -> Result<u64, String> {
    match value {
        Value::U64(value) => Ok(*value),
        Value::I64(value) if *value >= 0 => Ok(*value as u64),
        _ => Err("Expected unsigned integer".into()),
    }
}

fn raster(host: &mut AssetHost<'_>, side: usize, samples: Vec<f32>) -> Result<Value, String> {
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 {
            axes: vec![side as u32, side as u32],
            samples,
        },
    )))
}

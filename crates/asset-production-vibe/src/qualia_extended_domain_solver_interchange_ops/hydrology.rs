//! Bounded raster hydrology and probability-mask authoring.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::{AttributedMesh, FieldLattice},
    boolean, fields, integer, list, num, required, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::render::assets::{ImportedMesh, Mesh};
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc};
use terrain_erosion_solvers::{
    carve_channels, d8_flow_directions, flow_accumulation, hydraulic_erosion, priority_flood_fill,
    HydraulicConfig, D8_OFFSETS,
};

const MAX_CELLS: usize = 65_536;
const MAX_DIMENSION: usize = 1024;
const MAX_DROPLET_STEPS: usize = 16_000_000;

pub(super) fn evaluate(host: &mut AssetHost<'_>, op: &str, params: &Args) -> Result<Value, String> {
    match op {
        "rivers" => rivers(host, params),
        "run_droplet" => droplets(host, params),
        "seal_terrain_plane" => seal_terrain_plane(host, params),
        "meshify" | "outer_outline" => mask_geometry(host, params, op == "meshify"),
        _ => Err(format!("Unsupported terrain or mask operation {op}")),
    }
}

fn seal_terrain_plane(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(params, &["maps", "channel", "width", "height", "cell_size_m", "origin_m",
        "entity_iri", "source_iri", "name_iri", "unit_iri"])?;
    let grid = Grid::parse(params, 2)?;
    let index = integer(required(params, "channel")?, "channel")?;
    let TypedArtifact::FieldLattice(field) = host.workspace.typed(required(params, "maps")?)? else {
        return Err("Terrain sealing requires a typed raster stack".into());
    };
    let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
        return Err("Terrain sealing requires an f32 raster stack".into());
    };
    let [channels, height, width] = axes.as_slice() else {
        return Err("Terrain sealing requires [channel, height, width] axes".into());
    };
    if *width as usize != grid.width || *height as usize != grid.height || index >= *channels as usize {
        return Err("Terrain raster channel or declared geometry does not match the stack".into());
    }
    let start = index.checked_mul(grid.len()).ok_or("Terrain plane offset overflows")?;
    let end = start.checked_add(grid.len()).ok_or("Terrain plane extent overflows")?;
    let samples = samples.get(start..end).ok_or("Terrain plane is truncated")?.to_vec();
    let raster = host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(FieldLattice::DenseF32 {
        axes: vec![*height, *width],
        samples,
    })))?;
    let mut input = params.clone();
    input.remove("maps");
    input.remove("channel");
    input.insert("field".into(), raster);
    host.evaluate_gis_gp_terrain_and_datacubes(&BTreeMap::from([
        ("op".into(), Value::String("seal_datacube_raster".into())),
        ("parameters".into(), Value::Record(input)),
    ]))
}

#[derive(Clone, Copy)]
struct Grid {
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(params: &Args, minimum: usize) -> Result<Self, String> {
        let width = integer(required(params, "width")?, "width")?;
        let height = integer(required(params, "height")?, "height")?;
        if !(minimum..=MAX_DIMENSION).contains(&width)
            || !(minimum..=MAX_DIMENSION).contains(&height)
            || width * height > MAX_CELLS
        {
            return Err(format!("Grid requires dimensions {minimum}..={MAX_DIMENSION} and at most {MAX_CELLS} cells"));
        }
        Ok(Self { width, height })
    }

    fn len(self) -> usize {
        self.width * self.height
    }

    fn scale_down(self) -> Self {
        Self {
            width: self.width.div_ceil(2),
            height: self.height.div_ceil(2),
        }
    }
}

fn raster(
    host: &AssetHost<'_>,
    value: &Value,
    grid: Grid,
    limits: [f32; 2],
) -> Result<Vec<f32>, String> {
    let samples = if let Ok(TypedArtifact::FieldLattice(field)) = host.workspace.typed(value) {
        let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
            return Err("Expected a single-plane DenseF32 raster".into());
        };
        if axes.as_slice() != [grid.height as u32, grid.width as u32] || samples.len() != grid.len()
        {
            return Err("DenseF32 raster shape differs from [height, width]".into());
        }
        samples.clone()
    } else {
        let samples = list(value)?;
        if samples.len() != grid.len() {
            return Err("Sample count differs from width × height".into());
        }
        samples
            .iter()
            .map(|v| num(v).map(|v| v as f32))
            .collect::<Result<_, _>>()?
    };
    if samples.iter().any(|v| !(limits[0]..=limits[1]).contains(v)) {
        return Err("Raster contains non-finite or out-of-range samples".into());
    }
    Ok(samples)
}

fn parameter(params: &Args, key: &str, default: f64, bounds: [f64; 2]) -> Result<f32, String> {
    let value = params.get(key).map(num).transpose()?.unwrap_or(default);
    if !(bounds[0]..=bounds[1]).contains(&value) {
        return Err(format!("{key} is outside its admitted range"));
    }
    Ok(value as f32)
}

fn maps<const N: usize>(
    host: &mut AssetHost<'_>,
    grid: Grid,
    planes: [(&str, &str, Vec<f32>); N],
) -> Result<Args, String> {
    let mut names = Vec::with_capacity(N);
    let mut units = Vec::with_capacity(N);
    let mut data = Vec::with_capacity(grid.len() * N);
    for (name, unit, values) in planes {
        if values.len() != grid.len() || values.iter().any(|v| !v.is_finite()) {
            return Err(format!("Terrain solver produced an invalid {name} raster"));
        }
        names.push(Value::String(name.into()));
        units.push(Value::String(unit.into()));
        data.extend(values);
    }
    let field = host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 {
            axes: vec![N as u32, grid.height as u32, grid.width as u32],
            samples: data,
        },
    )))?;
    Ok(BTreeMap::from([
        ("maps".into(), field),
        ("map_names".into(), Value::List(names)),
        ("map_units".into(), Value::List(units)),
        ("width".into(), Value::U64(grid.width as u64)),
        ("height".into(), Value::U64(grid.height as u64)),
    ]))
}

fn gaussian_reduce(source: &[f32], grid: Grid) -> Vec<f32> {
    let next = grid.scale_down();
    let filter = [1.0, 4.0, 6.0, 4.0, 1.0];
    let mut reduced = Vec::with_capacity(next.len());
    for y in 0..next.height {
        for x in 0..next.width {
            let mut result = 0.0;
            for (j, &fy) in filter.iter().enumerate() {
                let row = (2 * y + j).saturating_sub(2).min(grid.height - 1);
                for (i, &fx) in filter.iter().enumerate() {
                    let column = (2 * x + i).saturating_sub(2).min(grid.width - 1);
                    result += fx * fy * f64::from(source[row * grid.width + column]);
                }
            }
            reduced.push((result / 256.0) as f32);
        }
    }
    reduced
}

fn expand(source: &[f32], coarse: Grid, fine: Grid) -> Vec<f32> {
    let mut expanded = Vec::with_capacity(fine.len());
    for y in 0..fine.height {
        let (y0, y1) = (y / 2, (y / 2 + 1).min(coarse.height - 1));
        let ty = (y % 2) as f32 * 0.5;
        for x in 0..fine.width {
            let (x0, x1) = (x / 2, (x / 2 + 1).min(coarse.width - 1));
            let tx = (x % 2) as f32 * 0.5;
            let top =
                (1.0 - tx) * source[y0 * coarse.width + x0] + tx * source[y0 * coarse.width + x1];
            let bottom =
                (1.0 - tx) * source[y1 * coarse.width + x0] + tx * source[y1 * coarse.width + x1];
            expanded.push((1.0 - ty) * top + ty * bottom);
        }
    }
    expanded
}

struct Drainage {
    conditioned: Vec<f32>,
    directions: Vec<u8>,
    accumulation: Vec<f32>,
    cut_depth: Vec<f32>,
}

fn drainage(terrain: &[f32], grid: Grid, downcut_m: f32, contributing_fraction: f32) -> Drainage {
    let mut conditioned = terrain.to_vec();
    priority_flood_fill(&mut conditioned, grid.width, grid.height);
    let directions = d8_flow_directions(&conditioned, grid.width, grid.height);
    let accumulation = flow_accumulation(&conditioned, &directions, grid.width, grid.height);
    let threshold = (grid.len() as f32 * contributing_fraction).max(2.0);
    // The native carver expects non-negative depths and limits cuts to its input
    // column. Use a common column exceeding any possible requested downcut so
    // extraction works without changing DEM elevations or the vertical datum.
    let column = 1.0001 * downcut_m * (grid.len() as f32 + 1.0).ln();
    let mut cut_depth = vec![column; grid.len()];
    carve_channels(
        &mut cut_depth,
        &accumulation,
        grid.width,
        grid.height,
        downcut_m * 200.0,
        threshold,
    );
    cut_depth
        .iter_mut()
        .for_each(|value| *value = column - *value);
    Drainage {
        conditioned,
        directions,
        accumulation,
        cut_depth,
    }
}

fn rivers(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "elevation_m",
            "downcut_m",
            "contributing_fraction",
            "detail_fade",
        ],
    )?;
    let fine = Grid::parse(params, 9)?;
    let mid = fine.scale_down();
    let coarse = mid.scale_down();
    let downcut: [f64; 3] = vector(required(params, "downcut_m")?)?;
    if downcut.iter().any(|v| !(0.0..=1000.0).contains(v)) {
        return Err(
            "Each of the fine, middle and coarse downcut coefficients must be 0..=1000 m".into(),
        );
    }
    let downcut = downcut.map(|v| v as f32);
    let fraction = parameter(params, "contributing_fraction", 0.02, [0.000001, 1.0])?;
    let fade = parameter(params, "detail_fade", 0.5, [0.0, 1.0])?;
    host.charge(fine.len() * 280, fine.len() * 104)?;
    let terrain = raster(host, required(params, "elevation_m")?, fine, [-2e6, 2e6])?;
    let middle = gaussian_reduce(&terrain, fine);
    let base = gaussian_reduce(&middle, mid);

    let low = drainage(&base, coarse, downcut[2], fraction);
    let low_delta = low.cut_depth.into_iter().map(|v| -v).collect::<Vec<_>>();
    let middle_detail = expand(&base, coarse, mid);
    let inherited_middle_delta = expand(&low_delta, coarse, mid);
    let middle_bed = middle
        .iter()
        .zip(&inherited_middle_delta)
        .map(|(&p, &d)| p + d)
        .collect::<Vec<_>>();
    let middle_drainage = drainage(&middle_bed, mid, downcut[1], fraction);
    let middle_delta = blend_delta(
        &middle,
        &middle_detail,
        inherited_middle_delta,
        &middle_drainage.cut_depth,
        fade,
        downcut[1],
    );

    let fine_detail = expand(&middle, mid, fine);
    let inherited_fine_delta = expand(&middle_delta, mid, fine);
    let fine_bed = terrain
        .iter()
        .zip(&inherited_fine_delta)
        .map(|(&p, &d)| p + d)
        .collect::<Vec<_>>();
    let fine_drainage = drainage(&fine_bed, fine, downcut[0], fraction);
    let delta = blend_delta(
        &terrain,
        &fine_detail,
        inherited_fine_delta,
        &fine_drainage.cut_depth,
        fade,
        downcut[0],
    );
    let carved = terrain
        .into_iter()
        .zip(&delta)
        .map(|(elevation, &change)| elevation + change)
        .collect();
    let mut output = maps(
        host,
        fine,
        [
            ("carved_elevation", "m", carved),
            ("conditioned_elevation", "m", fine_drainage.conditioned),
            ("upstream_cells", "count", fine_drainage.accumulation),
            ("elevation_delta", "m", delta),
            (
                "d8_direction",
                "index",
                fine_drainage
                    .directions
                    .into_iter()
                    .map(f32::from)
                    .collect(),
            ),
        ],
    )?;
    output.insert("d8_flat_or_sink".into(), Value::U64(255));
    output.insert(
        "d8_offsets_xy".into(),
        Value::List(
            D8_OFFSETS
                .into_iter()
                .map(|(x, y)| Value::List(vec![Value::I64(i64::from(x)), Value::I64(i64::from(y))]))
                .collect(),
        ),
    );
    Ok(Value::Record(output))
}

fn blend_delta(
    input: &[f32],
    detail_base: &[f32],
    inherited: Vec<f32>,
    cut: &[f32],
    fade: f32,
    downcut_m: f32,
) -> Vec<f32> {
    input
        .iter()
        .zip(detail_base)
        .zip(inherited)
        .zip(cut)
        .map(|(((&original, &base), previous), &cut)| {
            let mask = if downcut_m == 0.0 {
                0.0
            } else {
                (cut / downcut_m).clamp(0.0, 1.0)
            };
            previous - cut - fade * mask * (original - base)
        })
        .collect()
}

fn positive_count(params: &Args, name: &str, default: usize, max: usize) -> Result<usize, String> {
    let value = params
        .get(name)
        .map(|v| integer(v, name))
        .transpose()?
        .unwrap_or(default);
    (value != 0 && value <= max)
        .then_some(value)
        .ok_or_else(|| format!("{name} requires 1..={max}"))
}

fn droplets(host: &mut AssetHost<'_>, params: &Args) -> Result<Value, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "elevation_m",
            "seed",
            "droplet_count",
            "max_steps",
            "rock_softness",
            "fluvial_strength",
            "laminar_flow",
            "cell_size_m",
        ],
    )?;
    let grid = Grid::parse(params, 3)?;
    let seed = params
        .get("seed")
        .map(|v| integer(v, "seed"))
        .transpose()?
        .unwrap_or(1337);
    let seed = u32::try_from(seed).map_err(|_| "Droplet seed exceeds u32")?;
    let count = positive_count(params, "droplet_count", 50_000, 500_000)?;
    let steps = positive_count(params, "max_steps", 128, 512)?;
    if count * steps > MAX_DROPLET_STEPS {
        return Err("Droplet count × step limit exceeds the operation budget".into());
    }
    let cell_size = parameter(params, "cell_size_m", 1.0, [1e-6, 10_000.0])?;
    let config = HydraulicConfig {
        seed,
        droplet_count: count,
        max_steps: steps,
        height_range: cell_size * grid.width as f32,
        rock_softness: parameter(params, "rock_softness", 0.65, [0.0, 1.0])?,
        fluvial_strength: parameter(params, "fluvial_strength", 0.5, [0.0, 1.0])?,
        laminar_flow: boolean(params, "laminar_flow", false)?,
        ..HydraulicConfig::default()
    };
    host.charge(count * steps * 16 + grid.len() * 8, grid.len() * 48)?;
    let mut terrain = raster(host, required(params, "elevation_m")?, grid, [-2e6, 2e6])?;
    let output = hydraulic_erosion(&mut terrain, grid.width, grid.height, &config);
    let mut result = maps(
        host,
        grid,
        [
            ("eroded_elevation", "m", output.height),
            ("deposition", "m", output.deposit),
            ("wear", "m", output.wear),
            ("droplet_visits", "count", output.flow),
        ],
    )?;
    result.insert("seed".into(), Value::U64(u64::from(seed)));
    result.insert("cell_size_m".into(), Value::F64(f64::from(cell_size)));
    Ok(Value::Record(result))
}

fn mask_geometry(
    host: &mut AssetHost<'_>,
    params: &Args,
    export_mesh: bool,
) -> Result<Value, String> {
    fields(
        params,
        &[
            "width",
            "height",
            "probability",
            "threshold",
            "laplace_steps",
            "diffusivity",
            "cell_size_m",
        ],
    )?;
    let grid = Grid::parse(params, 1)?;
    let steps = params
        .get("laplace_steps")
        .map(|v| integer(v, "laplace_steps"))
        .transpose()?
        .unwrap_or(1);
    if steps > 32 {
        return Err("Laplacian mask smoothing permits at most 32 steps".into());
    }
    let diffusivity = parameter(params, "diffusivity", 0.2, [0.0, 0.25])?;
    let threshold = parameter(params, "threshold", 0.5, [0.0, 1.0])?;
    let spacing = parameter(params, "cell_size_m", 1.0, [1e-6, 10_000.0])?;
    host.charge(grid.len() * (64 + steps * 8), grid.len() * 240)?;
    let mut probability = raster(host, required(params, "probability")?, grid, [0.0, 1.0])?;
    let mut next = vec![0.0; grid.len()];
    for _ in 0..steps {
        for (index, value) in next.iter_mut().enumerate() {
            let x = index % grid.width;
            let y = index / grid.width;
            let north = y.saturating_sub(1) * grid.width + x;
            let south = (y + 1).min(grid.height - 1) * grid.width + x;
            let west = y * grid.width + x.saturating_sub(1);
            let east = y * grid.width + (x + 1).min(grid.width - 1);
            let neighbors =
                probability[north] + probability[south] + probability[west] + probability[east];
            *value = ((1.0 - 4.0 * diffusivity) * probability[index] + diffusivity * neighbors)
                .clamp(0.0, 1.0);
        }
        std::mem::swap(&mut probability, &mut next);
    }
    let mask: Vec<_> = probability
        .into_iter()
        .map(|value| value >= threshold)
        .collect();
    if export_mesh {
        let mesh = pixel_surface(&mask, grid, spacing)?;
        let triangle_count = mesh.triangles.len() as u64;
        let surface = ImportedMesh {
            mesh,
            normals: None,
            tangents: None,
            texture_coordinates_0: None,
            materials: Vec::new(),
            texture_dependencies: Vec::new(),
            submeshes: Vec::new(),
        };
        let encoded =
            host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
                surface,
                uv1: Vec::new(),
                vertex_colors: Vec::new(),
                skin_weights: Vec::new(),
                morph_targets: Arc::from(Vec::new()),
                protected_faces: Vec::new(),
            })))?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), encoded),
            ("triangle_count".into(), Value::U64(triangle_count)),
        ])))
    } else {
        let loops = vector_radiance_trace::trace_mask(&mask, grid.width, grid.height);
        let count = loops.len() as u64;
        let components: Vec<_> = loops.into_iter().map(|component| json!({
            "outer": component.outer.pts, "holes": component.holes.into_iter().map(|hole| hole.pts).collect::<Vec<_>>(),
            "occupied_cells": component.pixels,
        })).collect();
        let outline = json!({"schema": "qualia.raster-outline/v1", "width": grid.width, "height": grid.height,
            "cell_size_m": spacing, "coordinate_system": "pixel_corner_xy_y_down", "components": components});
        let bytes = serde_json::to_vec(&outline).map_err(|error| error.to_string())?;
        Ok(Value::Record(BTreeMap::from([
            ("outline".into(), host.encode("json", bytes)?),
            ("component_count".into(), Value::U64(count)),
        ])))
    }
}

fn pixel_surface(mask: &[bool], grid: Grid, spacing: f32) -> Result<Mesh, String> {
    let row_stride = grid.width + 1;
    let mut corners = vec![[u32::MAX; 2]; row_stride * (grid.height + 1)];
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    let mut vertex = |x: usize, y: usize, quadrant: usize| {
        let occupied = |x: Option<usize>, y: Option<usize>| match (x, y) {
            (Some(x), Some(y)) if x < grid.width && y < grid.height => mask[y * grid.width + x],
            _ => false,
        };
        let west = x.checked_sub(1);
        let north = y.checked_sub(1);
        let adjacent = [
            occupied(west, north),
            occupied(Some(x), north),
            occupied(Some(x), Some(y)),
            occupied(west, Some(y)),
        ];
        let split = !adjacent[(quadrant + 1) % 4]
            && !adjacent[(quadrant + 3) % 4]
            && adjacent[(quadrant + 2) % 4];
        let slot = if split && quadrant >= 2 { 1 } else { 0 };
        let corner = &mut corners[y * row_stride + x][slot];
        if *corner == u32::MAX {
            *corner = positions.len() as u32;
            positions.push([x as f32 * spacing, -(y as f32) * spacing, 0.0]);
        }
        *corner
    };
    for (index, &inside) in mask.iter().enumerate() {
        if !inside {
            continue;
        }
        let (x, y) = (index % grid.width, index / grid.width);
        let [a, b, c, d] = [
            vertex(x, y, 2),
            vertex(x + 1, y, 3),
            vertex(x + 1, y + 1, 0),
            vertex(x, y + 1, 1),
        ];
        triangles.extend([[a, c, b], [a, d, c]]);
    }
    let Some(&first) = positions.first() else {
        return Err("Probability mask produces an empty surface".into());
    };
    let (min, max) = positions.iter().fold((first, first), |(min, max), point| {
        (
            std::array::from_fn(|axis| min[axis].min(point[axis])),
            std::array::from_fn(|axis| max[axis].max(point[axis])),
        )
    });
    Ok(Mesh {
        positions,
        triangles,
        min,
        max,
    })
}

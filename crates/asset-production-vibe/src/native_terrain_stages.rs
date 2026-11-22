//! Independently callable terrain shaping, snow and lake stages.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::geodesy_ops::{heightfield_record, parse_heightfield};
use crate::*;
use terrain_erosion_solvers::{
    HydraulicConfig, LakeConfig, SnowConfig, TerraceConfig, ThermalShaperConfig, apply_terraces,
    fill_lakes, hydraulic_erosion, simulate_snow, thermal_shape_with,
};

impl AssetHost<'_> {
    pub(crate) fn solve_geomorph_stage(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["heightfield", "stage", "parameters"])?;
        let (side, spacing, origin, mut terrain) =
            parse_heightfield(required(args, "heightfield")?)?;
        let stage = text(required(args, "stage")?)?;
        let options = record(required(args, "parameters")?)?;
        let n = terrain.len();
        let mut diagnostics = Args::new();
        match stage {
            "terraces" => {
                fields(options, &["spacing_m", "intensity", "octaves", "seed"])?;
                let config = TerraceConfig {
                    spacing: bounded(options, "spacing_m", spacing, 0.01, 10_000.0)? as f32,
                    intensity: bounded(options, "intensity", 0.85, 0.0, 1.0)? as f32,
                    octaves: count(options, "octaves", 3, 8)? as i32,
                    seed: seed(options)? as i32,
                    ..Default::default()
                };
                self.charge(n * (config.octaves as usize + 1) * 16, n * 48)?;
                apply_terraces(&mut terrain, side, side, &config);
            }
            "thermal_talus" => {
                fields(options, &["strength", "feature_scale_m", "passes"])?;
                let passes = count(options, "passes", 1, 16)?;
                let feature_scale = bounded(
                    options,
                    "feature_scale_m",
                    spacing * 8.0,
                    spacing * 4.0,
                    spacing * 1024.0,
                )?;
                let config = ThermalShaperConfig {
                    shape: bounded(options, "strength", 0.25, -1.0, 1.0)?,
                    size: spacing * side as f64,
                    scale: feature_scale,
                };
                self.charge(n.saturating_mul(passes * 256 * 24), n * 64)?;
                let low = terrain.iter().copied().fold(f32::INFINITY, f32::min);
                let high = terrain.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let range = high - low;
                if range > 1e-6 && config.shape != 0.0 {
                    for h in &mut terrain {
                        *h = (*h - low) / range;
                    }
                    for _ in 0..passes {
                        terrain = thermal_shape_with(&terrain, side, &config);
                    }
                    for h in &mut terrain {
                        *h = h.mul_add(range, low);
                    }
                }
            }
            "hydraulic_droplets" => {
                fields(
                    options,
                    &["droplets", "steps", "seed", "softness", "flow_volume"],
                )?;
                let config = HydraulicConfig {
                    seed: seed(options)?,
                    droplet_count: count(options, "droplets", 4096, 100_000)?,
                    max_steps: count(options, "steps", 32, 512)?,
                    rock_softness: bounded(options, "softness", 0.65, 0.0, 1.0)? as f32,
                    flow_volume: bounded(options, "flow_volume", 0.1, 0.00001, 1.0)? as f32,
                    height_range: (spacing * side as f64) as f32,
                    ..Default::default()
                };
                self.charge(config.droplet_count * config.max_steps * 40, n * 128)?;
                let result = hydraulic_erosion(&mut terrain, side, side, &config);
                terrain = result.height;
                diagnostics.insert("deposited_m".into(), raster_values(&result.deposit)?);
                diagnostics.insert("wear_m".into(), raster_values(&result.wear)?);
                diagnostics.insert("flow".into(), raster_values(&result.flow)?);
            }
            "glacial_snow" => {
                fields(
                    options,
                    &["snow_line_m", "snowfall_m", "melt_m", "talus_deg", "steps"],
                )?;
                let result = snow_stage(self, side, spacing, &terrain, options)?;
                terrain = result.0;
                diagnostics.insert("snow_depth_m".into(), raster_values(&result.1)?);
            }
            _ => return Err(format!("Unknown geomorph stage {stage}")),
        }
        if terrain
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 2_000_000.0)
        {
            return Err("The conditioned terrain exceeds supported metre elevations".into());
        }
        diagnostics.insert(
            "heightfield".into(),
            heightfield_record(side, spacing, origin, &terrain),
        );
        Ok(Value::Record(diagnostics))
    }

    pub(crate) fn accumulate_snow_lakes(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "heightfield",
                "snow",
                "lake_floor_m",
                "lake_iterations",
                "shore_width_m",
            ],
        )?;
        let (side, spacing, origin, terrain) = parse_heightfield(required(args, "heightfield")?)?;
        let snow = record(required(args, "snow")?)?;
        fields(
            snow,
            &["snow_line_m", "snowfall_m", "melt_m", "talus_deg", "steps"],
        )?;
        let (combined, depth) = snow_stage(self, side, spacing, &terrain, snow)?;
        let floor = num(required(args, "lake_floor_m")?)?;
        if floor.abs() > 2_000_000.0 {
            return Err("Lake floor exceeds the supported metre range".into());
        }
        let shore = bounded(args, "shore_width_m", spacing * 2.0, 0.0, spacing * 16.0)? / spacing;
        let iterations = count(args, "lake_iterations", 24, 128)?;
        let window = 2 * shore.ceil() as usize + 1;
        self.charge(
            side * side * (iterations * 12 + window * window * 8),
            side * side * 112,
        )?;
        let result = fill_lakes(
            &combined,
            side,
            side,
            &LakeConfig {
                water_floor: floor as f32,
                precipitation: 0.0,
                shore_size: shore as f32,
                iterations,
            },
        );
        let mask = Value::List(result.depth.iter().map(|&v| Value::Bool(v > 0.0)).collect());
        Ok(Value::Record(Args::from([
            (
                "heightfield".into(),
                heightfield_record(side, spacing, origin, &combined),
            ),
            ("snow_depth_m".into(), raster_values(&depth)?),
            ("lake_depth_m".into(), raster_values(&result.depth)?),
            ("water_surface_m".into(), raster_values(&result.surface)?),
            ("shore_mask".into(), raster_values(&result.shore)?),
            ("lake_mask".into(), mask),
        ])))
    }
}

fn snow_stage(
    host: &mut AssetHost<'_>,
    side: usize,
    spacing: f64,
    terrain: &[f32],
    options: &Args,
) -> Result<(Vec<f32>, Vec<f32>), String> {
    let line = num(required(options, "snow_line_m")?)?;
    if line.abs() > 2_000_000.0 {
        return Err("Snow line exceeds the supported metre range".into());
    }
    let snowfall = bounded(options, "snowfall_m", 0.0, 0.0, 10.0)?;
    let melt = bounded(options, "melt_m", 0.0, 0.0, 10.0)?;
    let talus = bounded(options, "talus_deg", 38.0, 0.0, 80.0)?;
    let steps = count(options, "steps", 16, 128)?;
    let extent = terrain
        .iter()
        .map(|h| (f64::from(*h) - line).abs())
        .fold(spacing, f64::max);
    let scale = 2.0 * extent + (snowfall + melt) * steps as f64;
    let normalized: Vec<_> = terrain
        .iter()
        .map(|h| (0.5 + (f64::from(*h) - line) / scale) as f32)
        .collect();
    host.charge(terrain.len() * steps * 150, terrain.len() * 96)?;
    let snow = simulate_snow(
        &normalized,
        side,
        side,
        &SnowConfig {
            snow_line: 0.5,
            snowfall_intensity: (snowfall / scale) as f32,
            melting: (melt / scale) as f32,
            talus_angle: (talus.to_radians().tan() * spacing / scale)
                .atan()
                .to_degrees() as f32,
            clinging_volume: 0.0,
            iterations: steps,
        },
    );
    let depths: Vec<_> = snow
        .snow_depth
        .into_iter()
        .map(|value| (f64::from(value) * scale) as f32)
        .collect();
    if depths
        .iter()
        .any(|&value| !value.is_finite() || value < 0.0)
    {
        return Err("Snow solver returned an invalid depth".into());
    }
    let combined: Vec<_> = terrain
        .iter()
        .zip(&depths)
        .map(|(&rock, &snow)| rock + snow)
        .collect();
    if combined
        .iter()
        .any(|&value| !value.is_finite() || value.abs() > 2_000_000.0)
    {
        return Err("Snow cover exceeds the supported elevation range".into());
    }
    Ok((combined, depths))
}

fn seed(args: &Args) -> Result<u32, String> {
    let value = args
        .get("seed")
        .map(|v| integer(v, "seed"))
        .transpose()?
        .unwrap_or(0);
    u32::try_from(value).map_err(|_| "Stage seed must fit u32".into())
}

fn bounded(args: &Args, name: &str, default: f64, min: f64, max: f64) -> Result<f64, String> {
    let value = number(args, name, default)?;
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{name} outside the admitted range {min} to {max}"))
}

fn raster_values(values: &[f32]) -> Result<Value, String> {
    if values.iter().any(|v| !v.is_finite()) {
        return Err("Terrain stage returned a non-finite sample".into());
    }
    Ok(Value::List(
        values.iter().map(|&v| Value::F64(f64::from(v))).collect(),
    ))
}

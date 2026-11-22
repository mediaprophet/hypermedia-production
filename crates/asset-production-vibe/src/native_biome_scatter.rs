//! Native blue-noise biome scatter into a reference-only 10D scene.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::{
    container_10d::{GRAPHICS_ASSET_RING_BYTES, MANIFOLD_IDENTITY_V2_MAX_FIELDS},
    q_hash,
    specialized_libs::computational_geometry::{poisson_disk_2d, Point2},
};
use sha2::{Digest, Sha256};

use crate::{
    artifact::FieldLattice, count, fields, geodesy_ops::parse_heightfield, list, num, number,
    qualia_containers_governance_poet_ops::encoded_resource, record, required, text, vector, Args,
    AssetHost, TypedArtifact, Value,
};

const MAX_ARCHETYPES: usize = 32;
const MAX_CANDIDATES: usize = 32_768;
const MAX_GRID_CELLS: usize = 4_194_304;
const POISSON_ATTEMPTS: usize = 24;
const MAX_INSTANCES: usize = (MANIFOLD_IDENTITY_V2_MAX_FIELDS - 4) / 2;

struct Terrain {
    side: usize,
    spacing: f64,
    origin: [f64; 3],
    heights: Vec<f32>,
}

struct Habitat {
    moisture: Option<Vec<f32>>,
    canopy: Option<Vec<f32>>,
    moisture_range: [f64; 2],
    canopy_range: [f64; 2],
    max_gradient: f64,
}

struct Archetype<'a> {
    key: &'a str,
    weight: f64,
    uniform_scale: f64,
    age_days: f64,
    definition: Value,
}

impl AssetHost<'_> {
    pub(crate) fn scatter_biome_instances(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "heightfield",
                "cell_size_m",
                "origin_m",
                "biome",
                "archetypes",
                "assembly_frame",
                "radius_m",
                "seed",
                "max_candidates",
                "max_instances",
            ],
        )?;
        let terrain = Terrain::read(self, args)?;
        let habitat = Habitat::read(self, args.get("biome"), terrain.side)?;
        let archetypes = archetypes(required(args, "archetypes")?)?;
        let radius = num(required(args, "radius_m")?)?;
        if !(1e-3..=1e5).contains(&radius) {
            return Err("Biome radius_m must be in [0.001, 100000]".into());
        }
        let radius_sq = radius * radius;
        let seed = match required(args, "seed")? {
            Value::U64(seed) => *seed,
            Value::I64(seed) if *seed >= 0 => *seed as u64,
            _ => return Err("Biome seed needs an unsigned 64-bit integer".into()),
        };
        let candidate_limit = count(args, "max_candidates", 8192, MAX_CANDIDATES)?;
        let frame = required(args, "assembly_frame")?;
        let (frame_digest, frame_capacity) = {
            let source = encoded_resource(self, frame)?;
            if source.kind != "10d"
                || source.bytes.len() > self.limits.resource_bytes.min(GRAPHICS_ASSET_RING_BYTES)
            {
                return Err("Biome assembly_frame requires a bounded sealed .10d".into());
            }
            (format!("{:x}", Sha256::digest(&source.bytes)), Self::assembly_frame_leaf_capacity(&source.bytes)?)
        };
        let instance_limit = count(args, "max_instances", frame_capacity, MAX_INSTANCES)?.min(frame_capacity);
        if instance_limit == 0 {
            return Err("Biome scatter requires a positive instance limit".into());
        }
        let length = (terrain.side - 1) as f64 * terrain.spacing;
        let bins = (length / (radius / std::f64::consts::SQRT_2)).ceil() as usize;
        let grid_cells = bins
            .checked_mul(bins)
            .filter(|&cells| (1..=MAX_GRID_CELLS).contains(&cells))
            .ok_or("Biome Poisson acceleration grid exceeds its cell budget")?;
        self.charge(
            grid_cells.saturating_add(candidate_limit * (POISSON_ATTEMPTS * 25 + instance_limit)),
            grid_cells * 4 + candidate_limit * 96 + terrain.side * terrain.side * 12,
        )?;
        let mut grid = vec![0; grid_cells];
        let mut active = vec![0; candidate_limit];
        let mut points = vec![Point2::default(); candidate_limit];
        let count = poisson_disk_2d(
            Point2::new(0.0, 0.0),
            Point2::new(length, length),
            radius,
            seed,
            POISSON_ATTEMPTS,
            &mut grid,
            &mut active,
            &mut points,
        )
        .map_err(|error| format!("Native biome Poisson scatter: {error:?}"))?;
        drop(grid);
        drop(active);
        self.charge(0, 0)?;

        let mut eligible = Vec::new();
        for (slot, point) in points.into_iter().take(count).enumerate() {
            let x = point.x / terrain.spacing;
            let y = point.y / terrain.spacing;
            let x0 = (x as usize).min(terrain.side - 2);
            let y0 = (y as usize).min(terrain.side - 2);
            let tx = x - x0 as f64;
            let ty = y - y0 as f64;
            let quad = Quad {
                offset: y0 * terrain.side + x0,
                stride: terrain.side,
                tx,
                ty,
            };
            let [a, b, c, d] = quad.corners(&terrain.heights);
            let gradient = ((b - a) * (1.0 - ty) + (d - c) * ty)
                .hypot((c - a) * (1.0 - tx) + (d - b) * tx)
                / terrain.spacing;
            if gradient > habitat.max_gradient || !habitat.accepts(quad) {
                continue;
            }
            let height = quad.blend([a, b, c, d]);
            let position = [
                terrain.origin[0] + point.x,
                terrain.origin[1] + point.y,
                terrain.origin[2] + height,
            ]
            .map(|coordinate| coordinate as f32);
            if position
                .iter()
                .any(|value| !value.is_finite() || value.abs() >= 1e8)
            {
                return Err("Biome placement exceeds the native PGA coordinate envelope".into());
            }
            let priority = mix(seed ^ slot as u64);
            eligible.push((priority, slot, position));
        }
        eligible.sort_unstable_by_key(|&(priority, slot, _)| (priority, slot));
        let eligible_count = eligible.len();
        let weights: f64 = archetypes.iter().map(|entry| entry.weight).sum();
        let mut kept = Vec::with_capacity(instance_limit);
        let mut placements = Vec::with_capacity(instance_limit);
        for (priority, _, position) in eligible {
            if placements.len() == instance_limit {
                break;
            }
            let [x, y, z] = position.map(f64::from);
            if kept
                .iter()
                .any(|&[px, py]: &[f64; 2]| (px - x).powi(2) + (py - y).powi(2) < radius_sq)
            {
                continue;
            }
            kept.push([x, y]);
            let draw = (mix(priority) >> 11) as f64 * (weights / (1u64 << 53) as f64);
            let mut cumulative = 0.0;
            let entry = archetypes
                .iter()
                .find(|entry| {
                    cumulative += entry.weight;
                    draw < cumulative
                })
                .unwrap_or(&archetypes[archetypes.len() - 1]);
            let entity = format!(
                "urn:qualia:biome-instance:{frame_digest}:{seed:016x}:{:016x}:{:08x}{:08x}{:08x}",
                q_hash(entry.key),
                position[0].to_bits(),
                position[1].to_bits(),
                position[2].to_bits()
            );
            let motor = [1.0, 0.0, 0.0, 0.0, -0.5 * x, -0.5 * y, -0.5 * z, 0.0]
                .into_iter()
                .map(Value::F64)
                .collect();
            placements.push(Value::Record(Args::from([
                ("leaf_key".into(), Value::String(entry.key.into())),
                ("entity_iri".into(), Value::String(entity)),
                ("local_pga_motor".into(), Value::List(motor)),
                ("uniform_scale".into(), Value::F64(entry.uniform_scale)),
                ("age_days".into(), Value::F64(entry.age_days)),
            ])));
        }
        if placements.is_empty() {
            return Err(
                "No Poisson candidate passed the supplied biome mask and slope limits".into(),
            );
        }
        let placed = placements.len();
        let definitions = archetypes
            .into_iter()
            .map(|entry| entry.definition)
            .collect();
        let output = self.compose_assembly_10d(&Args::from([
            ("assembly_frame".into(), frame.clone()),
            ("archetypes".into(), Value::List(definitions)),
            ("instances".into(), Value::List(placements)),
        ]))?;
        let Value::Record(mut output) = output else {
            return Err("Native biome assembly did not return its typed scene".into());
        };
        output.insert("poisson_candidates".into(), Value::U64(count as u64));
        output.insert(
            "eligible_candidates".into(),
            Value::U64(eligible_count as u64),
        );
        output.insert("instance_count".into(), Value::U64(placed as u64));
        Ok(Value::Record(output))
    }
}

impl Terrain {
    fn read(host: &AssetHost<'_>, args: &Args) -> Result<Self, String> {
        let source = required(args, "heightfield")?;
        if matches!(source, Value::Record(fields) if !fields.contains_key("handle")) {
            if args.contains_key("cell_size_m") || args.contains_key("origin_m") {
                return Err(
                    "A calibrated heightfield record already supplies spacing and origin".into(),
                );
            }
            let (side, spacing, origin, heights) = parse_heightfield(source)?;
            return Ok(Self {
                side,
                spacing,
                origin,
                heights,
            });
        }
        let TypedArtifact::FieldLattice(lattice) = host.workspace.typed(source)? else {
            return Err(
                "Biome heightfield requires a calibrated record or a dense FieldLattice".into(),
            );
        };
        let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
            return Err("Biome FieldLattice heightfield must be dense f32".into());
        };
        let &[width, height] = axes.as_slice() else {
            return Err("Biome heightfield requires two axes".into());
        };
        let side = width as usize;
        let spacing = num(required(args, "cell_size_m")?)?;
        let origin = vector(required(args, "origin_m")?)?;
        if width != height
            || !(2..=256).contains(&side)
            || samples.len() != side * side
            || spacing <= 0.0
            || spacing > 1000.0
            || samples
                .iter()
                .any(|value| !value.is_finite() || value.abs() > 2e6)
        {
            return Err("Invalid dense, metrically calibrated biome heightfield".into());
        }
        Ok(Self {
            side,
            spacing,
            origin,
            heights: samples.clone(),
        })
    }
}

#[derive(Clone, Copy)]
struct Quad {
    offset: usize,
    stride: usize,
    tx: f64,
    ty: f64,
}

impl Quad {
    fn corners(self, samples: &[f32]) -> [f64; 4] {
        [0, 1, self.stride, self.stride + 1].map(|i| f64::from(samples[self.offset + i]))
    }

    fn blend(self, [a, b, c, d]: [f64; 4]) -> f64 {
        (a * (1.0 - self.tx) + b * self.tx) * (1.0 - self.ty)
            + (c * (1.0 - self.tx) + d * self.tx) * self.ty
    }
}

impl Habitat {
    fn read(host: &AssetHost<'_>, biome: Option<&Value>, side: usize) -> Result<Self, String> {
        let empty = Args::new();
        let biome = biome.map(record).transpose()?.unwrap_or(&empty);
        fields(
            biome,
            &[
                "moisture",
                "canopy",
                "moisture_range",
                "canopy_range",
                "max_slope_rad",
            ],
        )?;
        let mask = |name| {
            biome
                .get(name)
                .map(|input| mask_samples(host, input, side))
                .transpose()
        };
        let range = |name: &str, has_mask: bool| -> Result<[f64; 2], String> {
            let bounds = biome.get(name).map(vector::<2>).transpose()?;
            if bounds.is_some() && !has_mask {
                return Err(format!("{name} requires its co-registered mask"));
            }
            let [min, max] = bounds.unwrap_or([0.0, 1.0]);
            if min < 0.0 || max > 1.0 || min > max {
                return Err(format!("Invalid biome {name}"));
            }
            Ok([min, max])
        };
        let moisture = mask("moisture")?;
        let canopy = mask("canopy")?;
        let moisture_range = range("moisture_range", moisture.is_some())?;
        let canopy_range = range("canopy_range", canopy.is_some())?;
        let slope = number(biome, "max_slope_rad", std::f64::consts::FRAC_PI_2)?;
        if !(0.0..=std::f64::consts::FRAC_PI_2).contains(&slope) {
            return Err("Biome max_slope_rad must be in [0, π/2]".into());
        }
        Ok(Self {
            moisture,
            canopy,
            moisture_range,
            canopy_range,
            max_gradient: slope.tan(),
        })
    }

    fn accepts(&self, quad: Quad) -> bool {
        [
            (&self.moisture, self.moisture_range),
            (&self.canopy, self.canopy_range),
        ]
        .into_iter()
        .all(|(samples, [min, max])| {
            samples
                .as_deref()
                .is_none_or(|mask| (min..=max).contains(&quad.blend(quad.corners(mask))))
        })
    }
}

fn mask_samples(host: &AssetHost<'_>, source: &Value, side: usize) -> Result<Vec<f32>, String> {
    let samples = if let Value::List(values) = source {
        if values.len() != side * side {
            return Err("Biome mask size must match the heightfield".into());
        }
        values
            .iter()
            .map(|sample| {
                num(sample).and_then(|value| {
                    if (0.0..=1.0).contains(&value) {
                        Ok(value as f32)
                    } else {
                        Err("Biome mask sample must be in [0, 1]".into())
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let TypedArtifact::FieldLattice(lattice) = host.workspace.typed(source)? else {
            return Err(
                "Biome mask needs a co-registered dense FieldLattice or sample list".into(),
            );
        };
        let FieldLattice::DenseF32 { axes, samples } = lattice.as_ref() else {
            return Err("Biome FieldLattice mask must be dense f32".into());
        };
        if axes.as_slice() != [side as u32, side as u32]
            || samples.len() != side * side
            || samples.iter().any(|value| !(0.0..=1.0).contains(value))
        {
            return Err("Biome mask must be co-registered with finite samples in [0, 1]".into());
        }
        samples.clone()
    };
    Ok(samples)
}

fn archetypes(value: &Value) -> Result<Vec<Archetype<'_>>, String> {
    let values = list(value)?;
    if !(1..=MAX_ARCHETYPES).contains(&values.len()) {
        return Err("Biome scatter requires 1–32 authored archetypes".into());
    }
    values
        .iter()
        .map(|value| {
            let entry = record(value)?;
            fields(
                entry,
                &[
                    "leaf_key",
                    "resource",
                    "taxon_or_class_iri",
                    "weight",
                    "uniform_scale",
                    "age_days",
                ],
            )?;
            let key = text(required(entry, "leaf_key")?)?;
            let class = text(required(entry, "taxon_or_class_iri")?)?;
            let weight = number(entry, "weight", 1.0)?;
            let uniform_scale = number(entry, "uniform_scale", 1.0)?;
            let age_days = number(entry, "age_days", 0.0)?;
            if weight <= 0.0
                || weight > 1e6
                || !(0.001..=1000.0).contains(&uniform_scale)
                || !(0.0..=1e8).contains(&age_days)
            {
                return Err("Invalid biome archetype weight, age or scale".into());
            }
            let definition = Value::Record(Args::from([
                ("leaf_key".into(), Value::String(key.into())),
                ("taxon_or_class_iri".into(), Value::String(class.into())),
                ("resource".into(), required(entry, "resource")?.clone()),
            ]));
            Ok(Archetype {
                key,
                weight,
                uniform_scale,
                age_days,
                definition,
            })
        })
        .collect()
}

fn mix(mut word: u64) -> u64 {
    word = (word ^ (word >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    word = (word ^ (word >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    word ^ (word >> 31)
}

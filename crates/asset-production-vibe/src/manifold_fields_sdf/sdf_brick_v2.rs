//! SDFB v2 authoring and semantic sealing.

use super::triangle_field::{SamplingBudget, TriangleField, SCRATCH_PER_TRIANGLE};
use super::SDF_BRICK_MAGIC;
use crate::*;
use qualia_core_db::container_10d::sdf_brick_section::{
    encode_sdfb_into, sdfb_encoded_len, seal_10d_with_sdfb, SdfBrickGrid, SdfBrickKey,
    SdfBrickRecord, SdfSampleEncoding, SDFB_OCCUPANCY_BYTES, SDFB_SAMPLES,
};
use qualia_core_db::render::{
    assets::Mesh,
    compile_10d::{compile_mesh_to_10d, upgrade_legacy_10d_to_holographic_v2},
};
use std::sync::atomic::Ordering;

fn workspace_mesh<'a>(workspace: &'a AssetWorkspace, value: &Value) -> Result<&'a Mesh, String> {
    match workspace.get(value) {
        Ok(Artifact::Mesh(mesh)) => Ok(mesh),
        _ => match workspace.typed(value) {
            Ok(TypedArtifact::AttributedMesh(mesh)) => Ok(&mesh.surface.mesh),
            _ => Err("Expected a mesh or attributed mesh handle".into()),
        },
    }
}

fn fit_mesh(mesh: &Mesh, iri: Option<&Value>) -> Result<Vec<u8>, String> {
    let bytes =
        compile_mesh_to_10d(mesh).map_err(|error| format!("10D mesh compilation: {error:?}"))?;
    let iri = asset_entity_iri(iri, &bytes)?;
    let [x, y, z] = mesh.centroid();
    upgrade_legacy_10d_to_holographic_v2(
        &bytes,
        &iri,
        1,
        [0.0, 3.0, 2.0, x, y, z, 0.0, 1.0, 1.0, 0.5],
    )
    .map_err(|error| format!("HOLO v2 fitting: {error:?}"))
}

fn base_10d<'a>(workspace: &'a AssetWorkspace, handle: &Value) -> Result<&'a [u8], String> {
    let resource = workspace.encoded(handle)?;
    if resource.kind != "10d" {
        return Err("base_10d requires an encoded 10d handle".into());
    }
    Ok(&resource.bytes)
}

type Bounds = [[f32; 3]; 2];

fn refinement_bounds(value: &Value) -> Result<Bounds, String> {
    let [min, max] = list(value)? else {
        return Err("refinement_bounds requires [min, max] in meters".into());
    };
    let bounds = [point(min)?, point(max)?];
    if (0..3).any(|axis| bounds[0][axis] >= bounds[1][axis]) {
        return Err("refinement_bounds requires min < max on every axis".into());
    }
    Ok(bounds)
}

fn point(value: &Value) -> Result<[f32; 3], String> {
    let coords = vector::<3>(value)?.map(|coordinate| coordinate as f32);
    if coords.iter().any(|coordinate| !coordinate.is_finite()) {
        return Err("Expected finite coordinates in meters".into());
    }
    Ok(coords)
}

fn triplets(side: u32) -> impl Iterator<Item = [u32; 3]> {
    (0..side).flat_map(move |x| (0..side).flat_map(move |y| (0..side).map(move |z| [x, y, z])))
}

#[derive(Clone, Copy)]
struct BrickBake {
    grid: SdfBrickGrid,
    band: f32,
    wall: u64,
    lumen: u64,
}

impl BrickBake {
    fn new(
        origin: [f32; 3],
        spacing: [f32; 3],
        bricks: u32,
        band_voxels: f32,
        wall: u64,
        lumen: u64,
    ) -> Result<Self, String> {
        let band = spacing.into_iter().fold(0.0, f32::max) * band_voxels;
        let meters_per_i16 = band / f32::from(i16::MAX);
        if spacing.iter().any(|&step| !step.is_finite() || step <= 0.0)
            || !band.is_finite()
            || meters_per_i16 <= 0.0
        {
            return Err("SDFB requires positive finite voxel spacing and narrow band".into());
        }
        let grid = SdfBrickGrid {
            origin_m: origin,
            voxel_spacing_m: spacing,
            base_bricks: [bricks; 3],
            encoding: SdfSampleEncoding::ScaledI16,
            meters_per_i16,
        };
        Ok(Self {
            grid,
            band,
            wall,
            lumen,
        })
    }

    fn position(&self, key: SdfBrickKey, offset: [f32; 3]) -> [f32; 3] {
        let stride = (1u32 << key.refinement_level) as f32;
        std::array::from_fn(|axis| {
            let sample = (key.coordinate[axis] as f32 * 8.0 + offset[axis]) / stride;
            self.grid.origin_m[axis] + sample * self.grid.voxel_spacing_m[axis]
        })
    }

    fn empty_record(self, key: SdfBrickKey) -> SdfBrickRecord {
        SdfBrickRecord {
            key,
            wall_semantic_id: self.wall,
            lumen_semantic_id: self.lumen,
            samples: [i16::MAX as u16; SDFB_SAMPLES],
            occupancy: [0; SDFB_OCCUPANCY_BYTES],
        }
    }

    fn sample_record(
        self,
        key: SdfBrickKey,
        mut distance: impl FnMut([u32; 3]) -> Result<f32, String>,
    ) -> Result<Option<SdfBrickRecord>, String> {
        let mut brick = self.empty_record(key);
        let mut retained = false;
        for (index, sample) in brick.samples.iter_mut().enumerate() {
            let point = [
                (index % 8) as u32,
                ((index / 8) % 8) as u32,
                (index / 64) as u32,
            ];
            let distance = distance(point)?;
            if !distance.is_finite() {
                return Err("SDFB source contains a non-finite distance".into());
            }
            retained |= distance < self.band;
            let rounded =
                (distance.clamp(-self.band, self.band) / self.grid.meters_per_i16).round() as i16;
            *sample = if distance < 0.0 {
                brick.occupancy[index / 8] |= 1 << (index % 8);
                rounded.min(-1) as u16
            } else {
                rounded as u16
            };
        }
        Ok(retained.then_some(brick))
    }

    fn sample_mesh(
        self,
        key: SdfBrickKey,
        field: &TriangleField<'_>,
        budget: &mut SamplingBudget<'_>,
    ) -> Result<(Option<SdfBrickRecord>, bool), String> {
        let center = field.distance(self.position(key, [3.5; 3]), budget)?;
        let stride = (1u32 << key.refinement_level) as f32;
        let radius = self
            .grid
            .voxel_spacing_m
            .iter()
            .map(|v| v * v)
            .sum::<f32>()
            .sqrt()
            * 3.5
            / stride;
        if center > self.band + radius {
            return Ok((None, false));
        }
        if center < -self.band - radius {
            let mut brick = self.empty_record(key);
            brick.samples.fill((-i16::MAX) as u16);
            brick.occupancy.fill(u8::MAX);
            return Ok((Some(brick), false));
        }
        let brick = self.sample_record(key, |local| {
            field.distance(self.position(key, local.map(|x| x as f32)), budget)
        })?;
        Ok((brick, true))
    }

    fn touches(self, key: SdfBrickKey, [min, max]: Bounds) -> bool {
        let lo = self.position(key, [0.0; 3]);
        let hi = self.position(key, [8.0; 3]);
        (0..3).all(|axis| lo[axis] <= max[axis] && hi[axis] >= min[axis])
    }
}

struct DenseSdf<'a> {
    origin: [f32; 3],
    spacing: [f32; 3],
    resolution: u32,
    samples: &'a [u8],
}

impl<'a> DenseSdf<'a> {
    fn parse(bytes: &'a [u8]) -> Result<Self, String> {
        if bytes.len() < 36 || &bytes[..8] != SDF_BRICK_MAGIC {
            return Err("Expected Q42SDF16 input".into());
        }
        let resolution = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        if !(2..=128).contains(&resolution) || bytes.len() != 36 + resolution.pow(3) as usize * 4 {
            return Err("Invalid Q42SDF16 payload length or resolution".into());
        }
        let read = |offset| f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap());
        let origin = std::array::from_fn(|axis| read(12 + axis * 4));
        if origin.iter().any(|value| !value.is_finite()) {
            return Err("Non-finite Q42SDF16 origin".into());
        }
        Ok(Self {
            origin,
            spacing: std::array::from_fn(|axis| read(24 + axis * 4)),
            resolution,
            samples: &bytes[36..],
        })
    }

    fn distance(&self, point: [u32; 3], outside: f32) -> f32 {
        if point.into_iter().any(|index| index >= self.resolution) {
            return outside;
        }
        let [x, y, z] = point;
        let offset = ((z * self.resolution + y) * self.resolution + x) as usize * 4;
        f32::from_le_bytes(self.samples[offset..offset + 4].try_into().unwrap())
    }
}

impl AssetHost<'_> {
    pub(super) fn fit_holographic_v2(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mesh", "entity_iri"])?;
        let handle = required(a, "mesh")?;
        let mesh = workspace_mesh(&self.workspace, handle)?;
        let work = mesh.triangles.len().saturating_mul(32);
        self.charge(work, work)?;
        let encoded = fit_mesh(
            workspace_mesh(&self.workspace, handle)?,
            a.get("entity_iri"),
        )?;
        self.encode("10d", encoded)
    }

    pub(super) fn seal_sdf_brick_10d(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "mesh",
                "sdf",
                "base_10d",
                "entity_iri",
                "wall_iri",
                "lumen_iri",
                "resolution",
                "padding",
                "narrow_band_voxels",
                "refinement_bounds",
            ],
        )?;
        let wall_iri = text(required(a, "wall_iri")?)?;
        let lumen_iri = text(required(a, "lumen_iri")?)?;
        let wall = qualia_core_db::q_hash(wall_iri);
        let lumen = qualia_core_db::q_hash(lumen_iri);
        if wall == 0 || lumen == 0 || wall == lumen {
            return Err("SDFB requires distinct nonzero wall and lumen semantic IDs".into());
        }
        let band = number(a, "narrow_band_voxels", 3.0)?;
        if !(1.0..=16.0).contains(&band) {
            return Err("narrow_band_voxels must be in 1..=16".into());
        }
        let mesh = a.get("mesh");
        let dense = a.get("sdf");
        let source = a.get("base_10d");
        let (grid, mut records, fitted) = match (mesh, dense) {
            (Some(handle), None) => self.sample_mesh_bricks(handle, a, band as f32, wall, lumen)?,
            (None, Some(handle)) => {
                if source.is_none() {
                    return Err("SDFB dense input requires base_10d".into());
                }
                if ["resolution", "padding", "entity_iri", "refinement_bounds"]
                    .iter()
                    .any(|key| a.contains_key(*key))
                {
                    return Err(
                        "Dense SDFB uses its sealed grid and cannot add local refinement".into(),
                    );
                }
                let (grid, records) = self.sample_dense_bricks(handle, band as f32, wall, lumen)?;
                (grid, records, None)
            }
            _ => return Err("Supply exactly one of mesh or sdf".into()),
        };
        let base = match source {
            Some(handle) => base_10d(&self.workspace, handle)?,
            None => fitted.as_deref().ok_or("SDFB requires a base 10D asset")?,
        };
        records.sort_unstable_by_key(|record| record.key);
        let len = sdfb_encoded_len(records.len()).map_err(|error| error.to_string())?;
        let mut field = vec![0; len];
        encode_sdfb_into(grid, &records, &mut field).map_err(|error| error.to_string())?;
        drop(records);
        let sealed = seal_10d_with_sdfb(base, &field, wall_iri, lumen_iri)?;
        let asset = self.encode("10d", sealed)?;
        let field = self.encode("sdfb", field)?;
        Ok(Value::Record(BTreeMap::from([
            ("asset".into(), asset),
            ("field".into(), field),
        ])))
    }

    fn sample_mesh_bricks(
        &mut self,
        handle: &Value,
        a: &Args,
        band: f32,
        wall: u64,
        lumen: u64,
    ) -> Result<(SdfBrickGrid, Vec<SdfBrickRecord>, Option<Vec<u8>>), String> {
        let resolution = count(a, "resolution", 48, 128)?;
        let padding = number(a, "padding", 0.02)? as f32;
        if resolution < 16 || !(0.000_001..=10.0).contains(&padding) {
            return Err(
                "SDFB requires resolution in 16..=128 and positive padding in meters".into(),
            );
        }
        let refine = a
            .get("refinement_bounds")
            .map(refinement_bounds)
            .transpose()?;
        if a.contains_key("base_10d") && a.contains_key("entity_iri") {
            return Err("base_10d already provides the entity_iri".into());
        }
        let mesh = workspace_mesh(&self.workspace, handle)?;
        if mesh.triangles.is_empty() {
            return Err("SDFB mesh input requires a closed triangle surface".into());
        }
        let side = resolution.div_ceil(8) as u32;
        let origin = mesh.min.map(|min| min - padding);
        let spacing = std::array::from_fn(|axis| {
            (mesh.max[axis] + padding - origin[axis]) / (side * 8 - 1) as f32
        });
        let bake = BrickBake::new(origin, spacing, side, band, wall, lumen)?;
        let count = side.pow(3) as usize;
        let children = refine.map_or(0, |bounds| {
            triplets(side)
                .filter(|&coordinate| {
                    bake.touches(
                        SdfBrickKey {
                            refinement_level: 0,
                            coordinate,
                        },
                        bounds,
                    )
                })
                .count()
                * 8
        });
        let upper = sdfb_encoded_len(count + children).map_err(|error| error.to_string())?;
        let triangles = mesh.triangles.len();
        self.charge(
            triangles.saturating_mul(32),
            upper
                .saturating_mul(2)
                .saturating_add(triangles.saturating_mul(SCRATCH_PER_TRIANGLE)),
        )?;
        let mesh = workspace_mesh(&self.workspace, handle)?;
        let field = TriangleField::build(mesh)?;
        let before = usize::try_from(self.limits.asset_work_units.saturating_sub(self.work))
            .unwrap_or(usize::MAX);
        let mut budget = SamplingBudget {
            remaining: before,
            cancel: self.cancel,
        };
        let sampled: Result<_, String> = (|| {
            let mut records = Vec::new();
            for coordinate in triplets(side) {
                let key = SdfBrickKey {
                    refinement_level: 0,
                    coordinate,
                };
                let (record, near) = bake.sample_mesh(key, &field, &mut budget)?;
                records.extend(record);
                if near && refine.is_some_and(|bounds| bake.touches(key, bounds)) {
                    for child in triplets(2) {
                        let coordinate =
                            std::array::from_fn(|axis| coordinate[axis] * 2 + child[axis]);
                        let key = SdfBrickKey {
                            refinement_level: 1,
                            coordinate,
                        };
                        records.extend(bake.sample_mesh(key, &field, &mut budget)?.0);
                    }
                }
            }
            Ok(records)
        })();
        let work = before - budget.remaining;
        self.charge(work, 0)?;
        let records = sampled?;
        let fitted = if a.contains_key("base_10d") {
            None
        } else {
            Some(fit_mesh(
                workspace_mesh(&self.workspace, handle)?,
                a.get("entity_iri"),
            )?)
        };
        Ok((bake.grid, records, fitted))
    }

    fn sample_dense_bricks(
        &mut self,
        handle: &Value,
        band: f32,
        wall: u64,
        lumen: u64,
    ) -> Result<(SdfBrickGrid, Vec<SdfBrickRecord>), String> {
        let read = |workspace: &AssetWorkspace| -> Result<_, String> {
            let resource = workspace.encoded(handle)?;
            if resource.kind != "sdf" {
                return Err("Expected a legacy sdf handle".into());
            }
            let dense = DenseSdf::parse(&resource.bytes)?;
            Ok((dense.resolution, dense.origin, dense.spacing))
        };
        let (resolution, origin, spacing) = read(&self.workspace)?;
        let side = resolution.div_ceil(8);
        let bricks = side.pow(3) as usize;
        let upper = sdfb_encoded_len(bricks).map_err(|error| error.to_string())?;
        self.charge(bricks * SDFB_SAMPLES, upper.saturating_mul(2))?;
        let dense = DenseSdf::parse(&self.workspace.encoded(handle)?.bytes)?;
        let bake = BrickBake::new(origin, spacing, side, band, wall, lumen)?;
        let mut records = Vec::new();
        for coordinate in triplets(side) {
            if self.cancel.load(Ordering::Relaxed) {
                return Err("SDFB sampling cancelled".into());
            }
            let key = SdfBrickKey {
                refinement_level: 0,
                coordinate,
            };
            records.extend(bake.sample_record(key, |offset| {
                let point = std::array::from_fn(|axis| coordinate[axis] * 8 + offset[axis]);
                Ok(dense.distance(point, bake.band))
            })?);
        }
        Ok((bake.grid, records))
    }
}

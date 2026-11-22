//! Ordinary kriging in a three-dimensional local metric frame.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{collections::BTreeMap, sync::atomic::Ordering};

use qualia_core_db::q42::m64_matter::{M64RasterEdges, M64RasterRows};
use qualia_geospatial::{
    ElevationRaster, GeodeticFrame, SRID_WGS84, VERTICAL_DATUM_ELLIPSOIDAL,
    raster_m64::{RasterM64Limits, RasterM64Policy, compile_raster_to_m64_v3},
};
use sha2::{Digest, Sha256};

use super::{lattice, receipt};
use crate::{Args, AssetHost, Value, fields, integer, list, num, record, required, text, vector};

const MAX_SAMPLES: usize = 96;
const MAX_CELLS: usize = 1_048_576;

#[derive(Clone, Copy)]
enum Model {
    Spherical,
    Exponential,
    MaternThreeHalves,
}

struct Covariance {
    model: Model,
    range_m: f64,
    partial_sill: f64,
    nugget: f64,
}

impl Covariance {
    fn diagonal(&self) -> f64 {
        self.partial_sill + self.nugget
    }

    fn evaluate(&self, a: [f64; 3], b: [f64; 3]) -> f64 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        let dz = a[2] - b[2];
        let distance = dx.hypot(dy).hypot(dz);
        if distance == 0.0 {
            return self.diagonal();
        }
        let q = distance / self.range_m;
        let correlation = match self.model {
            Model::Spherical => {
                let t = q.min(1.0);
                (1.0 - t).powi(2) * (1.0 + 0.5 * t)
            }
            Model::Exponential => (-q).exp(),
            Model::MaternThreeHalves => {
                let t = 3.0f64.sqrt() * q;
                (1.0 + t) * (-t).exp()
            }
        };
        correlation * self.partial_sill
    }
}

struct Observation {
    xyz: [f64; 3],
    value: f64,
}

struct Grid {
    shape: [usize; 3],
    origin_m: [f64; 3],
    spacing_m: [f64; 3],
    cells: usize,
}

impl AssetHost<'_> {
    pub(crate) fn krige_subsurface_strata_m64(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &["borehole_samples", "grid", "variogram", "drift", "horizon"],
        )?;
        if args
            .get("drift")
            .map(text)
            .transpose()?
            .unwrap_or("ordinary")
            != "ordinary"
        {
            return Err("3D kriging supports the ordinary constant-unknown-mean drift".into());
        }
        let variogram = record(required(args, "variogram")?)?;
        fields(variogram, &["model", "range_m", "partial_sill", "nugget"])?;
        let model_name = text(required(variogram, "model")?)?;
        let covariance = Covariance {
            model: match model_name {
                "spherical" => Model::Spherical,
                "exponential" => Model::Exponential,
                "matern_3_2" => Model::MaternThreeHalves,
                _ => return Err("Expected spherical, exponential or matern_3_2 covariance".into()),
            },
            range_m: bounded(variogram, "range_m", 0.001, 1e8)?,
            partial_sill: bounded(variogram, "partial_sill", 1e-12, 1e20)?,
            nugget: bounded(variogram, "nugget", 0.0, 1e20)?,
        };
        let grid = parse_grid(record(required(args, "grid")?)?)?;
        let points = list(required(args, "borehole_samples")?)?;
        if !(2..=MAX_SAMPLES).contains(&points.len()) {
            return Err("3D kriging requires 2..=96 borehole sample rows".into());
        }
        let mut samples = Vec::with_capacity(points.len());
        let mut source_hash = Sha256::new();
        for point in points {
            let [x, y, z, value] = vector::<4>(point)?;
            let xyz = [x, y, z];
            if xyz.iter().any(|at| at.abs() > 1e8) || value.abs() > 1e18 {
                return Err(
                    "Borehole coordinates or scalar observations exceed their finite domain".into(),
                );
            }
            if samples
                .iter()
                .any(|previous: &Observation| previous.xyz == xyz)
            {
                return Err("Borehole sample positions must be unique".into());
            }
            for item in [x, y, z, value] {
                source_hash.update(item.to_le_bytes());
            }
            samples.push(Observation { xyz, value });
        }
        let source_iri = format!("urn:sha256:{:x}", source_hash.finalize());
        let n = samples.len();
        self.charge(
            (n * n * n).saturating_add(grid.cells.saturating_mul(n * n + 24 * n)),
            grid.cells * 8 + n * n * 8,
        )?;
        let lower = factor(&samples, &covariance)?;
        let mut inverse_one = vec![1.0; n];
        solve_cholesky(&lower, &mut inverse_one);
        let one_scale: f64 = inverse_one.iter().sum();
        if !one_scale.is_finite() || one_scale <= 0.0 {
            return Err("Kriging mean constraint is singular".into());
        }

        let [nx, ny, nz] = grid.shape;
        let mut weights = vec![0.0; n];
        let mut cross_covariance = vec![0.0; n];
        let mut estimate = Vec::with_capacity(grid.cells);
        let mut variance = Vec::with_capacity(grid.cells);
        let mut output_hash = Sha256::new();
        output_hash.update(source_iri.as_bytes());
        output_hash.update(model_name.as_bytes());
        for parameter in [
            covariance.range_m,
            covariance.partial_sill,
            covariance.nugget,
        ] {
            output_hash.update(parameter.to_le_bytes());
        }
        for axis in 0..3 {
            output_hash.update((grid.shape[axis] as u64).to_le_bytes());
            output_hash.update(grid.origin_m[axis].to_le_bytes());
            output_hash.update(grid.spacing_m[axis].to_le_bytes());
        }
        for i in 0..grid.cells {
            if i % 1024 == 0 && self.cancel.load(Ordering::Relaxed) {
                return Err("Subsurface kriging cancelled".into());
            }
            let [x, y, z] = [i % nx, (i / nx) % ny, i / (nx * ny)];
            let xyz = [x, y, z].map(|at| at as f64);
            let xyz =
                std::array::from_fn(|axis| grid.origin_m[axis] + grid.spacing_m[axis] * xyz[axis]);
            for (c, sample) in cross_covariance.iter_mut().zip(&samples) {
                *c = covariance.evaluate(sample.xyz, xyz);
            }
            weights.copy_from_slice(&cross_covariance);
            solve_cholesky(&lower, &mut weights);
            let multiplier = (weights.iter().sum::<f64>() - 1.0) / one_scale;
            let (mut mean, mut correlation, mut weight_sum) = (0.0, 0.0, 0.0);
            for j in 0..n {
                let weight = weights[j] - multiplier * inverse_one[j];
                weight_sum += weight;
                mean += weight * samples[j].value;
                correlation += weight * cross_covariance[j];
            }
            let sigma2 = covariance.diagonal() - correlation - multiplier;
            if (weight_sum - 1.0).abs() > 1e-8
                || !weight_sum.is_finite()
                || !mean.is_finite()
                || mean.abs() > f32::MAX as f64
                || !sigma2.is_finite()
                || sigma2 < -1e-8 * covariance.diagonal()
                || sigma2 > f32::MAX as f64
            {
                return Err(format!(
                    "Kriging variance or unbiased weight residual failed at cell {i}"
                ));
            }
            let mean = mean as f32;
            let sigma2 = sigma2.max(0.0) as f32;
            output_hash.update(mean.to_le_bytes());
            output_hash.update(sigma2.to_le_bytes());
            estimate.push(mean);
            variance.push(sigma2);
        }

        let dataset = format!("urn:sha256:{:x}", output_hash.finalize());
        let model = format!("urn:qualia:kriging:covariance:{model_name}");
        let mut facts = vec![
            (
                dataset.as_str(),
                "http://www.w3.org/ns/prov#wasDerivedFrom",
                source_iri.as_str(),
            ),
            (
                dataset.as_str(),
                "urn:qualia:kriging:covarianceModel",
                model.as_str(),
            ),
        ];
        let horizon = args
            .get("horizon")
            .map(record)
            .transpose()?
            .map(|options| seal_horizon(self, options, &grid, &estimate))
            .transpose()?;
        let horizon_iri = horizon
            .as_ref()
            .map(|(_, hash)| format!("urn:sha256:{hash}"));
        if let Some(iri) = &horizon_iri {
            facts.push((
                iri.as_str(),
                "http://www.w3.org/ns/prov#wasDerivedFrom",
                dataset.as_str(),
            ));
        }
        let q42 = receipt(self, "urn:qualia:geodesy:krige-subsurface-strata", &facts)?;
        let axes = vec![nz as u32, ny as u32, nx as u32];
        let estimate = lattice(self, axes.clone(), estimate)?;
        let variance = lattice(self, axes, variance)?;
        let mut result = BTreeMap::from([
            ("estimate".into(), estimate),
            ("variance".into(), variance),
            ("receipt".into(), q42),
            ("origin_xyz_m".into(), values(grid.origin_m)),
            ("spacing_xyz_m".into(), values(grid.spacing_m)),
            (
                "axis_order".into(),
                Value::String("z_y_x_slowest_to_fastest".into()),
            ),
        ]);
        if let Some((m64, digest)) = horizon {
            result.insert("m64".into(), m64);
            result.insert("m64_sha256".into(), Value::String(digest));
        }
        Ok(Value::Record(result))
    }
}

fn parse_grid(args: &Args) -> Result<Grid, String> {
    fields(args, &["shape_xyz", "origin_xyz_m", "spacing_xyz_m"])?;
    let shape = list(required(args, "shape_xyz")?)?;
    if shape.len() != 3 {
        return Err("The 3D grid requires three axis lengths".into());
    }
    let shape = [
        integer(&shape[0], "nx")?,
        integer(&shape[1], "ny")?,
        integer(&shape[2], "nz")?,
    ];
    let cells = shape
        .into_iter()
        .try_fold(1usize, |count, side| {
            (1..=256).contains(&side).then_some(())?;
            count.checked_mul(side)
        })
        .filter(|&count| count <= MAX_CELLS)
        .ok_or("3D grid exceeds the cell or axis budget")?;
    let origin_m = vector::<3>(required(args, "origin_xyz_m")?)?;
    let spacing_m = vector::<3>(required(args, "spacing_xyz_m")?)?;
    if (0..3).any(|axis| {
        !(0.001..=1e6).contains(&spacing_m[axis])
            || origin_m[axis].abs() > 1e8
            || (origin_m[axis] + (shape[axis] - 1) as f64 * spacing_m[axis]).abs() > 1e8
    }) {
        return Err("3D grid extent or spacing is outside the local metric domain".into());
    }
    Ok(Grid {
        shape,
        origin_m,
        spacing_m,
        cells,
    })
}

fn factor(samples: &[Observation], covariance: &Covariance) -> Result<Vec<f64>, String> {
    let n = samples.len();
    let mut lower = vec![0.0; n * n];
    for i in 0..n {
        for j in 0..=i {
            let mut value = covariance.evaluate(samples[i].xyz, samples[j].xyz);
            for k in 0..j {
                value -= lower[i * n + k] * lower[j * n + k];
            }
            if i == j {
                if value <= covariance.diagonal() * 1e-12 || !value.is_finite() {
                    return Err(format!(
                        "Kriging covariance is not numerically positive definite at pivot {i}"
                    ));
                }
                lower[i * n + j] = value.sqrt();
            } else {
                lower[i * n + j] = value / lower[j * n + j];
            }
        }
    }
    Ok(lower)
}

fn solve_cholesky(lower: &[f64], rhs: &mut [f64]) {
    let n = rhs.len();
    for i in 0..n {
        let entry = rhs[..i]
            .iter()
            .zip(&lower[i * n..i * n + i])
            .fold(rhs[i], |residual, (&x, &a)| residual - a * x);
        rhs[i] = entry / lower[i * n + i];
    }
    for i in (0..n).rev() {
        let entry = ((i + 1)..n).fold(rhs[i], |residual, j| residual - lower[j * n + i] * rhs[j]);
        rhs[i] = entry / lower[i * n + i];
    }
}

fn seal_horizon(
    host: &mut AssetHost<'_>,
    args: &Args,
    grid: &Grid,
    field: &[f32],
) -> Result<(Value, String), String> {
    fields(args, &["isovalue", "origin_lat_deg", "origin_lon_deg"])?;
    let isovalue = bounded(args, "isovalue", -1e18, 1e18)?;
    let [width, height, depth] = grid.shape;
    let [dx, dy, dz] = grid.spacing_m;
    let cell_mm = (dx * 1000.0).round();
    if depth < 2
        || dx != dy
        || (cell_mm / 1000.0 - dx).abs() > 1e-9
        || !(1.0..=u32::MAX as f64).contains(&cell_mm)
    {
        return Err("M64 horizon requires square cells at exact millimetre spacing and at least two z levels".into());
    }
    let origin_lat_deg = bounded(args, "origin_lat_deg", -89.999, 89.999)?;
    let origin_lon_deg = bounded(args, "origin_lon_deg", -180.0, 180.0)?;
    let plane = width * height;
    let mut elevations = Vec::with_capacity(plane);
    for i in 0..plane {
        let mut hit = None;
        for z in 0..depth {
            let current = f64::from(field[z * plane + i]);
            let root = if current == isovalue {
                Some(z as f64)
            } else if z + 1 < depth {
                let next = f64::from(field[(z + 1) * plane + i]);
                ((current < isovalue && next > isovalue) || (current > isovalue && next < isovalue))
                    .then(|| z as f64 + (isovalue - current) / (next - current))
            } else {
                None
            };
            if let Some(root) = root {
                if hit.replace(root).is_some() {
                    return Err(format!("Horizon has multiple intersections at column {i}"));
                }
            }
        }
        let root = hit.ok_or_else(|| format!("Horizon is absent at column {i}"))?;
        let millimetres = ((grid.origin_m[2] + root * dz) * 1000.0).round();
        if !(i32::MIN as f64..=i32::MAX as f64).contains(&millimetres) {
            return Err("Horizon elevation exceeds the M64 signed millimetre range".into());
        }
        elevations.push(millimetres as i32);
    }
    let frame = GeodeticFrame {
        srid: SRID_WGS84,
        vertical_datum: VERTICAL_DATUM_ELLIPSOIDAL,
        origin_lat_deg,
        origin_lon_deg,
        origin_elevation_mm: elevations[0],
        ..Default::default()
    };
    let raster = ElevationRaster {
        width,
        height,
        cell_size_mm: cell_mm as u32,
        elevations_mm: &elevations,
    };
    let policy = RasterM64Policy {
        rows: M64RasterRows::North,
        edges: M64RasterEdges::RepeatNearest,
        nodata_sentinel_mm: None,
    };
    let shape = policy
        .metadata(&raster, &frame)
        .shape()
        .map_err(|error| format!("M64 horizon shape: {error:?}"))?;
    if shape.file_bytes > host.limits.resource_bytes {
        return Err("M64 horizon exceeds the output byte budget".into());
    }
    host.charge(shape.records * 64, shape.file_bytes + field.len() * 8)?;
    let mut bytes = vec![0; shape.file_bytes];
    let length = compile_raster_to_m64_v3(
        &raster,
        &frame,
        1,
        policy,
        RasterM64Limits {
            max_output_bytes: host.limits.resource_bytes,
            max_chunks: shape.chunks,
            max_records: shape.records,
        },
        host.cancel,
        &mut bytes,
    )
    .map_err(|error| format!("Native M64 horizon seal: {error:?}"))?;
    bytes.truncate(length);
    let digest = format!("{:x}", Sha256::digest(&bytes));
    Ok((host.encode("m64", bytes)?, digest))
}

fn bounded(args: &Args, key: &str, lo: f64, hi: f64) -> Result<f64, String> {
    let value = num(required(args, key)?)?;
    if !(lo..=hi).contains(&value) {
        return Err(format!("{key} outside supported range {lo}..={hi}"));
    }
    Ok(value)
}

fn values(v: impl IntoIterator<Item = f64>) -> Value {
    Value::List(v.into_iter().map(Value::F64).collect())
}

//! Bounded authored cloud density volumes and a CPU single-scattering preview.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::{FieldLattice, LatticeSamples, RadianceLattice},
    fields, integer, list, num, record, required, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    domains::geospatial::terrain::noise_simd_lut::perlin_3d,
    simulation::environment::atmosphere::BarometricState,
};
use std::{collections::BTreeMap, f32::consts::PI, sync::Arc};

const MAX_AXIS: usize = 128;
const MAX_VOXELS: usize = 262_144;
const PHASE_SAMPLES: usize = 512;

#[derive(Clone, Copy)]
struct Grid {
    dims: [usize; 3],
    origin: [f32; 3],
    extent: [f32; 3],
    spacing: [f32; 3],
}

impl Grid {
    fn cell_index(self, x: usize, y: usize, z: usize) -> usize {
        (z * self.dims[1] + y) * self.dims[0] + x
    }

    fn sample(self, density: &[f32], point: [f32; 3]) -> f32 {
        let position: [f32; 3] = std::array::from_fn(|axis| {
            (point[axis] / self.spacing[axis] - 0.5).clamp(0.0, (self.dims[axis] - 1) as f32)
        });
        let [x, y, z] = position.map(|coordinate| coordinate.floor() as usize);
        let [nx, ny, nz] = self.dims;
        let [x1, y1, z1] = [
            (x + 1).min(nx - 1),
            (y + 1).min(ny - 1),
            (z + 1).min(nz - 1),
        ];
        let [tx, ty, tz] = [
            position[0] - x as f32,
            position[1] - y as f32,
            position[2] - z as f32,
        ];
        let fetch = |x, y, z| density[self.cell_index(x, y, z)];
        let lower = lerp(
            lerp(fetch(x, y, z), fetch(x1, y, z), tx),
            lerp(fetch(x, y1, z), fetch(x1, y1, z), tx),
            ty,
        );
        let upper = lerp(
            lerp(fetch(x, y, z1), fetch(x1, y, z1), tx),
            lerp(fetch(x, y1, z1), fetch(x1, y1, z1), tx),
            ty,
        );
        lerp(lower, upper, tz)
    }
}

struct Cloud {
    grid: Grid,
    atmosphere: BarometricState,
    base: f32,
    top: f32,
    feature_size: f32,
    coverage: f32,
    top_shift: [f32; 2],
    peak_water_density: f32,
    seed: u64,
}

struct Lighting {
    sun_dir: [f32; 3],
    sun: [f32; 3],
    sky: [f32; 3],
    extinction: f32,
    albedo: f32,
    g: f32,
    shadow_samples: usize,
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_volumetric_cloud(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "barometric_state",
                "ground_altitude_m",
                "lcl_altitude_m",
                "cloud_top_altitude_m",
                "domain_origin_m",
                "domain_extents_m",
                "resolution",
                "coverage",
                "feature_size_m",
                "anvil_offset_xz_m",
                "peak_density_kg_m3",
                "seed",
                "sun_direction_xyz",
                "sun_linear_rgb",
                "sky_linear_rgb",
                "droplet_radius_um",
                "phase_g",
                "single_scattering_albedo",
                "shadow_samples",
            ],
        )?;
        let cloud = Cloud::from_args(args)?;
        let lighting = Lighting::from_args(args)?;
        let grid = cloud.grid;
        let n = grid.dims.iter().product::<usize>();
        let [nx, ny, nz] = grid.dims;
        self.charge(
            n * (512 + lighting.shadow_samples * 48),
            n * 4 + nx * ny * 16 + 4096,
        )?;
        let density = generate_density(&cloud);
        let (preview, max_opacity) = render_preview(&density, grid, &lighting);
        if preview
            .iter()
            .chain(&density)
            .any(|value| !value.is_finite())
        {
            return Err("Cloud synthesis produced non-finite density or radiance".into());
        }
        let integral_kg = density.iter().map(|&d| f64::from(d)).sum::<f64>()
            * grid.spacing.map(f64::from).iter().product::<f64>();
        let nonzero_voxels = density.iter().filter(|&&d| d > 0.0).count();
        let density = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![nz as u32, ny as u32, nx as u32],
                samples: density,
            },
        )))?;
        let phase_lut = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![2, PHASE_SAMPLES as u32],
                samples: phase_lut(lighting.g),
            },
        )))?;
        let preview =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: nx as u32,
                height: ny as u32,
                channels: 4,
                primaries_iri: "http://www.w3.org/ns/ttml/parameter/rec709".into(),
                transfer_iri: "urn:qualia:color:transfer:linear".into(),
                samples: LatticeSamples::F32(preview),
            })))?;
        Ok(Value::Record(BTreeMap::from([
            ("density".into(), density),
            ("phase_lut".into(), phase_lut),
            ("preview".into(), preview),
            ("density_units".into(), Value::String("kg/m³".into())),
            (
                "phase_lut_channels".into(),
                Value::List(
                    vec![
                        Value::String("cos_theta".into()),
                        Value::String("phase_per_sr".into()),
                    ]
                    .into(),
                ),
            ),
            (
                "preview_view_axis".into(),
                Value::String("+Z, image rows from +Y".into()),
            ),
            ("origin_m".into(), numbers(grid.origin)),
            ("voxel_size_m".into(), numbers(grid.spacing)),
            ("lcl_altitude_m".into(), Value::F64(cloud.base.into())),
            ("cloud_top_altitude_m".into(), Value::F64(cloud.top.into())),
            ("liquid_water_mass_kg".into(), Value::F64(integral_kg)),
            ("nonzero_voxels".into(), Value::U64(nonzero_voxels as u64)),
            (
                "extinction_m2_per_kg".into(),
                Value::F64(lighting.extinction.into()),
            ),
            (
                "maximum_preview_opacity".into(),
                Value::F64(max_opacity.into()),
            ),
            (
                "scattering".into(),
                Value::String("single_henyey_greenstein".into()),
            ),
        ])))
    }
}

impl Cloud {
    fn from_args(args: &Args) -> Result<Self, String> {
        let weather = record(required(args, "barometric_state")?)?;
        fields(weather, &["mslp_hpa", "temperature_c", "dew_point_c"])?;
        let mslp_hpa = number(weather, "mslp_hpa", None, 780.0, 1_100.0)?;
        let temperature_c = number(weather, "temperature_c", None, -70.0, 50.0)?;
        let dew_point_c = number(weather, "dew_point_c", None, -100.0, temperature_c)?;
        let tetens = |temp: f32| (17.27 * temp / (temp + 237.3)).exp();
        let atmosphere = BarometricState {
            mslp_hpa,
            temperature_c,
            dew_point_c,
            relative_humidity: tetens(dew_point_c) / tetens(temperature_c),
            wind_vector_10m: [0.0; 2],
        };
        let ground = number(args, "ground_altitude_m", None, -500.0, 8_500.0)?;
        let base = number(
            args,
            "lcl_altitude_m",
            Some(ground + atmosphere.lifting_condensation_level_m()),
            ground,
            19_999.0,
        )?;
        let top = number(args, "cloud_top_altitude_m", None, base + 1.0, 20_000.0)?;
        let grid = grid(args)?;
        let domain_top = grid.origin[1] + grid.extent[1];
        if domain_top <= base || grid.origin[1] >= top {
            return Err("Cloud domain must intersect the vertical condensation layer".into());
        }
        let top_shift = args
            .get("anvil_offset_xz_m")
            .map(|v| checked_vector::<2>(v, "anvil_offset_xz_m", -50_000.0, 50_000.0))
            .transpose()?
            .unwrap_or([0.0; 2]);
        let seed = match required(args, "seed")? {
            Value::U64(value) => *value,
            Value::I64(value) => {
                u64::try_from(*value).map_err(|_| "Cloud seed must be non-negative")?
            }
            _ => return Err("Cloud seed requires a non-negative integer".into()),
        };
        Ok(Self {
            grid,
            atmosphere,
            base,
            top,
            top_shift,
            seed,
            feature_size: number(args, "feature_size_m", None, 10.0, 50_000.0)?,
            coverage: number(args, "coverage", None, 0.0, 1.0)?,
            peak_water_density: number(args, "peak_density_kg_m3", None, 1e-7, 0.02)?,
        })
    }
}

impl Lighting {
    fn from_args(args: &Args) -> Result<Self, String> {
        let direction = checked_vector::<3>(
            required(args, "sun_direction_xyz")?,
            "sun_direction_xyz",
            -1.0,
            1.0,
        )?;
        let length = direction[0].hypot(direction[1]).hypot(direction[2]);
        if length < 1e-6 {
            return Err("sun_direction_xyz must have nonzero length".into());
        }
        let radius = number(args, "droplet_radius_um", Some(10.0), 1.0, 100.0)?;
        let render_color = |key, default| {
            args.get(key)
                .map(|v| checked_vector::<3>(v, key, 0.0, 100.0))
                .transpose()
                .map(|color| color.unwrap_or(default))
        };
        let shadow_samples = args
            .get("shadow_samples")
            .map(|value| integer(value, "shadow_samples"))
            .transpose()?
            .unwrap_or(12);
        if !(4..=32).contains(&shadow_samples) {
            return Err("shadow_samples must be in 4..=32".into());
        }
        // Geometric optics approximation for liquid droplets, Qext = 2.
        let extinction = 3.0 * 2.0 / (4.0 * 1_000.0 * radius * 1e-6);
        Ok(Self {
            sun_dir: direction.map(|value| value / length),
            sun: render_color("sun_linear_rgb", [7.0, 6.7, 6.2])?,
            sky: render_color("sky_linear_rgb", [0.27, 0.48, 0.82])?,
            extinction,
            albedo: number(args, "single_scattering_albedo", Some(0.999), 0.0, 1.0)?,
            g: number(args, "phase_g", Some(0.84), -0.8, 0.95)?,
            shadow_samples,
        })
    }
}

fn grid(args: &Args) -> Result<Grid, String> {
    let [x, y, z] = list(required(args, "resolution")?)? else {
        return Err("resolution requires [nx, ny, nz]".into());
    };
    let dims = [integer(x, "nx")?, integer(y, "ny")?, integer(z, "nz")?];
    if dims.iter().any(|extent| !(2..=MAX_AXIS).contains(extent))
        || dims.iter().product::<usize>() > MAX_VOXELS
    {
        return Err(format!(
            "Cloud volume requires axes 2..={MAX_AXIS} and at most {MAX_VOXELS} voxels"
        ));
    }
    let origin = checked_vector::<3>(
        required(args, "domain_origin_m")?,
        "domain_origin_m",
        -1e6,
        1e6,
    )?;
    let extent = checked_vector::<3>(
        required(args, "domain_extents_m")?,
        "domain_extents_m",
        2.0,
        200_000.0,
    )?;
    if !(-500.0..20_000.0).contains(&origin[1]) || origin[1] + extent[1] > 20_000.0 {
        return Err("Cloud vertical domain must lie between -500 m and 20,000 m altitude".into());
    }
    let spacing = std::array::from_fn(|axis| extent[axis] / dims[axis] as f32);
    Ok(Grid {
        dims,
        origin,
        extent,
        spacing,
    })
}

fn generate_density(cloud: &Cloud) -> Vec<f32> {
    let Grid {
        dims: [nx, ny, nz],
        spacing: [dx, dy, dz],
        origin,
        ..
    } = cloud.grid;
    let mut density = vec![0.0; nx * ny * nz];
    if cloud.coverage == 0.0 {
        return density;
    }
    let rho_base = cloud.atmosphere.air_density(cloud.base);
    for iy in 0..ny {
        let lower = cloud.base.max(origin[1] + iy as f32 * dy);
        let upper = cloud.top.min(origin[1] + (iy + 1) as f32 * dy);
        if lower >= upper {
            continue;
        }
        let altitude = (lower + upper) * 0.5;
        let h = (altitude - cloud.base) / (cloud.top - cloud.base);
        let layer_fraction = ((upper - lower) / dy).min(1.0);
        let envelope = smooth_step(0.0, 0.13, h) * (1.0 - smooth_step(0.66, 1.0, h));
        let height_density =
            layer_fraction * envelope * cloud.atmosphere.air_density(altitude) / rho_base;
        let anvil = h * h;
        for iz in 0..nz {
            for ix in 0..nx {
                let position = [
                    (origin[0] + (ix as f32 + 0.5) * dx - anvil * cloud.top_shift[0])
                        / cloud.feature_size,
                    (altitude - cloud.base) / cloud.feature_size,
                    (origin[2] + (iz as f32 + 0.5) * dz - anvil * cloud.top_shift[1])
                        / cloud.feature_size,
                ];
                let [x, y, z] = position;
                let base = 0.5 + 0.5 * perlin_3d(x, y * 0.65, z, cloud.seed);
                let bands = 0.5
                    + 0.5
                        * perlin_3d(
                            x * 2.11,
                            y * 2.11,
                            z * 2.11,
                            cloud.seed ^ 0x517c_c1b7_2722_0a95,
                        );
                let cellular =
                    (1.0 - 1.08 * worley(position.map(|v| v * 1.57), cloud.seed)).clamp(0.0, 1.0);
                let score = base * 0.53 + bands * 0.19 + cellular * 0.28;
                let threshold = 1.0 - cloud.coverage;
                let fill = smooth_step(threshold - 0.04, threshold + 0.12, score);
                density[cloud.grid.cell_index(ix, iy, iz)] =
                    cloud.peak_water_density * height_density * fill;
            }
        }
    }
    density
}

fn worley(position: [f32; 3], seed: u64) -> f32 {
    let base = position.map(|x| x.floor() as i32);
    let local = std::array::from_fn::<_, 3, _>(|axis| position[axis] - base[axis] as f32);
    let mut nearest = f32::INFINITY;
    for z in -1_i32..=1 {
        for y in -1_i32..=1 {
            for x in -1_i32..=1 {
                let key = seed
                    ^ ((base[0] + x) as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
                    ^ ((base[1] + y) as u64).wrapping_mul(0xbf58_476d_1ce4_e5b9)
                    ^ ((base[2] + z) as u64).wrapping_mul(0x94d0_49bb_1331_11eb);
                let hash = mix(key);
                let offset = [x, y, z];
                // Restrict jitter to the center third so a feature outside
                // this 3×3×3 neighborhood cannot be the nearest.
                let delta = std::array::from_fn::<_, 3, _>(|axis| {
                    let jitter = ((hash >> (axis * 16)) & 0xffff) as f32 / 65_535.0;
                    offset[axis] as f32 + (1.0 + jitter) / 3.0 - local[axis]
                });
                nearest = nearest.min(delta.iter().map(|v| v * v).sum());
            }
        }
    }
    nearest.sqrt()
}

fn mix(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}

fn render_preview(density: &[f32], grid: Grid, lighting: &Lighting) -> (Vec<f32>, f32) {
    let [nx, ny, nz] = grid.dims;
    let [dx, dy, dz] = grid.spacing;
    let mut image = Vec::with_capacity(nx * ny * 4);
    let mut max_opacity = 0.0_f32;
    let phase = hg_phase(lighting.sun_dir[2], lighting.g);
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut view_transmittance = 1.0_f32;
            let mut radiance = [0.0; 3];
            for z in 0..nz {
                let water = density[grid.cell_index(x, y, z)];
                if water <= 0.0 {
                    continue;
                }
                let point = [
                    (x as f32 + 0.5) * dx,
                    (y as f32 + 0.5) * dy,
                    (z as f32 + 0.5) * dz,
                ];
                let sun_transmittance = shadow_transmittance(density, grid, point, lighting);
                let alpha = -(-water * dz * lighting.extinction).exp_m1();
                let scatter =
                    alpha * view_transmittance * sun_transmittance * phase * lighting.albedo;
                for (value, incident) in radiance.iter_mut().zip(lighting.sun) {
                    *value += scatter * incident;
                }
                view_transmittance *= 1.0 - alpha;
                if view_transmittance < 1e-6 {
                    break;
                }
            }
            for (value, background) in radiance.iter_mut().zip(lighting.sky) {
                *value += view_transmittance * background;
            }
            image.extend([radiance[0], radiance[1], radiance[2], 1.0]);
            max_opacity = max_opacity.max(1.0 - view_transmittance);
        }
    }
    (image, max_opacity)
}

fn shadow_transmittance(density: &[f32], grid: Grid, point: [f32; 3], lighting: &Lighting) -> f32 {
    let mut ray_length = f32::INFINITY;
    for (axis, &direction) in lighting.sun_dir.iter().enumerate() {
        if direction > 0.0 {
            ray_length = ray_length.min((grid.extent[axis] - point[axis]) / direction);
        } else if direction < 0.0 {
            ray_length = ray_length.min(-point[axis] / direction);
        }
    }
    let step = ray_length / lighting.shadow_samples as f32;
    let mut optical_depth = 0.0;
    for sample in 0..lighting.shadow_samples {
        let t = (sample as f32 + 0.5) * step;
        let position = std::array::from_fn(|axis| point[axis] + t * lighting.sun_dir[axis]);
        optical_depth += grid.sample(density, position) * step * lighting.extinction;
        if optical_depth > 20.0 {
            return 0.0;
        }
    }
    (-optical_depth).exp()
}

fn hg_phase(cosine: f32, g: f32) -> f32 {
    let angular = 1.0 + g * g - 2.0 * g * cosine.clamp(-1.0, 1.0);
    (1.0 - g * g) / (4.0 * PI * angular * angular.sqrt())
}

fn phase_lut(g: f32) -> Vec<f32> {
    let mut samples = Vec::with_capacity(PHASE_SAMPLES * 2);
    // Equal-probability samples resolve the narrow forward lobe at high g.
    let asymmetry = f64::from(g);
    samples.extend((0..PHASE_SAMPLES).map(|i| {
        let u = i as f64 / (PHASE_SAMPLES - 1) as f64;
        if asymmetry.abs() < 1e-5 {
            (2.0 * u - 1.0) as f32
        } else {
            let ratio = (1.0 - asymmetry * asymmetry) / (1.0 - asymmetry + 2.0 * asymmetry * u);
            ((1.0 + asymmetry * asymmetry - ratio * ratio) / (2.0 * asymmetry)).clamp(-1.0, 1.0)
                as f32
        }
    }));
    for index in 0..PHASE_SAMPLES {
        samples.push(hg_phase(samples[index], g));
    }
    samples
}

fn smooth_step(lo: f32, hi: f32, x: f32) -> f32 {
    let t = ((x - lo) / (hi - lo)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn checked_vector<const N: usize>(
    value: &Value,
    name: &str,
    lo: f64,
    hi: f64,
) -> Result<[f32; N], String> {
    let coordinates = vector::<N>(value)?;
    if coordinates.iter().any(|x| !(lo..=hi).contains(x)) {
        return Err(format!("{name} components must lie in {lo}..={hi}"));
    }
    Ok(coordinates.map(|x| x as f32))
}

fn number(args: &Args, key: &str, default: Option<f32>, lo: f32, hi: f32) -> Result<f32, String> {
    let value = match args.get(key) {
        Some(value) => num(value)? as f32,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (lo..=hi)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must lie in {lo}..={hi}"))
}

fn numbers<const N: usize>(values: [f32; N]) -> Value {
    Value::List(values.map(|value| Value::F64(value.into())).into())
}

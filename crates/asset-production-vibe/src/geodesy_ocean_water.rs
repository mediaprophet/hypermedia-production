//! CPU ocean snapshots with authored bathymetry, waves, ripple impulses, and water optics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::{AttributedMesh, FieldLattice},
    fields, integer, list, num, record, required, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::{
    etalon_render::water_surface::{
        BioGeoOpticalConstituents, DielectricVolumeIop, GerstnerWave, InteractiveRippleBuffer,
        SimdGerstnerSpectrum, MAX_GERSTNER_HARMONICS, RIPPLE_GRID_DIM, RIPPLE_TOTAL_CELLS,
        WAVELENGTH_BLUE_NM, WAVELENGTH_GREEN_NM, WAVELENGTH_RED_NM,
    },
    render::assets::{ImportedMesh, Mesh},
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_AXIS: usize = 128;
const MAX_RIPPLE_STEPS: usize = 4_096;
const MAX_COLLIDERS: usize = 128;
const CHANNELS: [&str; 4] = ["surface_y_m", "ripple_y_m", "depth_m", "wetness"];

#[derive(Clone, Copy)]
struct Grid {
    width: usize,
    height: usize,
    spacing: f32,
    origin: [f32; 2],
}

impl Grid {
    fn point(self, index: usize) -> [f32; 2] {
        [
            self.origin[0] + (index % self.width) as f32 * self.spacing,
            self.origin[1] + (index / self.width) as f32 * self.spacing,
        ]
    }

    fn sample(self, data: &[f32], point: [f32; 2]) -> Result<f32, String> {
        let end = self.point(self.width * self.height - 1);
        if (0..2).any(|axis| !(self.origin[axis]..=end[axis]).contains(&point[axis])) {
            return Err("Ocean collider lies outside the bathymetry grid".into());
        }
        let x = ((point[0] - self.origin[0]) / self.spacing).min((self.width - 1) as f32);
        let z = ((point[1] - self.origin[1]) / self.spacing).min((self.height - 1) as f32);
        let ix = (x.floor() as usize).min(self.width - 2);
        let iz = (z.floor() as usize).min(self.height - 2);
        let a = iz * self.width + ix;
        let x_fraction = x - ix as f32;
        let z_fraction = z - iz as f32;
        let top = data[a] + (data[a + 1] - data[a]) * x_fraction;
        let bottom =
            data[a + self.width] + (data[a + self.width + 1] - data[a + self.width]) * x_fraction;
        Ok(top + (bottom - top) * z_fraction)
    }
}

struct Impulse {
    time: f32,
    center: [f32; 2],
    elevation: f32,
    radius: f32,
}

impl AssetHost<'_> {
    pub(crate) fn simulate_ocean_water(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "width",
                "height",
                "cell_size_m",
                "origin_xz_m",
                "bathymetry_m",
                "sea_level_m",
                "time_s",
                "waves",
                "optics",
                "colliders",
                "shore_band_m",
                "wet_fringe_m",
                "ripple_speed_m_s",
                "ripple_damping_per_step",
            ],
        )?;
        let axis = |key| {
            let n = integer(required(args, key)?, key)?;
            (2..=MAX_AXIS)
                .contains(&n)
                .then_some(n)
                .ok_or_else(|| format!("{key} must be between 2 and {MAX_AXIS} samples"))
        };
        let grid = Grid {
            width: axis("width")?,
            height: axis("height")?,
            spacing: bound(args, "cell_size_m", None, 0.01, 10_000.0)?,
            origin: pair(required(args, "origin_xz_m")?, "origin_xz_m", 10_000.0)?,
        };
        let n = grid.width * grid.height;
        let sea = bound(args, "sea_level_m", None, -10_000.0, 10_000.0)?;
        let time = bound(args, "time_s", None, 0.0, 3_600.0)?;
        let shore_band = bound(
            args,
            "shore_band_m",
            Some(grid.spacing * 3.0),
            0.01,
            100_000.0,
        )?;
        let wet_fringe = bound(args, "wet_fringe_m", Some(0.05), 0.001, 10.0)?;
        let speed = bound(args, "ripple_speed_m_s", Some(2.5), 0.01, 20.0)?;
        let damping = bound(args, "ripple_damping_per_step", Some(0.985), 0.01, 1.0)?;
        let spectrum = spectrum(required(args, "waves")?, sea)?;
        let optics = optics(required(args, "optics")?)?;
        let end = grid.point(n - 1);
        let extents = [end[0] - grid.origin[0], end[1] - grid.origin[1]];
        let ripple_size = extents[0].max(extents[1]);
        let mut ripple = InteractiveRippleBuffer::new(
            grid.origin[0] + extents[0] * 0.5,
            grid.origin[1] + extents[1] * 0.5,
            ripple_size,
            speed,
            damping,
        );
        let scratch = n * 600 + (256 << 10);
        self.charge(n * 160, scratch)?;
        let bed = bathymetry(self, required(args, "bathymetry_m")?, grid)?;
        let impulses = impulses(args, grid, &bed, sea, time, &ripple)?;
        let ripple_steps = advance_ripples(self, &mut ripple, impulses, time, scratch)?;
        let still_depth = bed.iter().map(|y| sea - y).collect::<Vec<_>>();
        let shore = shore_distances(&still_depth, grid);
        let mut elevations = Vec::with_capacity(n);
        let mut ripples = Vec::with_capacity(n);
        let mut clearance = Vec::with_capacity(n);
        for (i, &shore_distance) in shore.iter().enumerate() {
            let [x, z] = grid.point(i);
            let (wave, _) =
                spectrum.sample_nearshore_surface(x, z, time, shore_distance, shore_band);
            let shore_weight = (shore_distance / grid.spacing).clamp(0.0, 1.0);
            let perturbation = ripple.sample_height(x, z) * shore_weight;
            let surface = wave + perturbation;
            elevations.push(surface);
            ripples.push(perturbation);
            clearance.push(surface - bed[i]);
        }
        let water = clipped_surface(grid, &elevations, &clearance)?;
        let samples = elevations
            .into_iter()
            .chain(ripples)
            .chain(clearance.iter().map(|&x| x.max(0.0)))
            .chain(
                clearance
                    .iter()
                    .map(|&x| (1.0 + x.min(0.0) / wet_fringe).max(0.0)),
            )
            .collect::<Vec<_>>();
        if !samples.iter().all(|x| x.is_finite()) {
            return Err("Ocean calculation produced non-finite samples".into());
        }
        let (mesh, triangle_count) = match water {
            Some(surface) => {
                let count = surface.mesh.triangles.len() as u64;
                (
                    self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
                        surface,
                        uv1: Vec::new(),
                        vertex_colors: Vec::new(),
                        skin_weights: Vec::new(),
                        morph_targets: Arc::from(Vec::new()),
                        protected_faces: Vec::new(),
                    })))?,
                    count,
                )
            }
            None => (Value::Null, 0),
        };
        let fields = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![CHANNELS.len() as u32, grid.height as u32, grid.width as u32],
                samples,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), mesh),
            ("fields".into(), fields),
            ("iop".into(), optical_record(optics)),
            (
                "channel_names".into(),
                Value::List(CHANNELS.map(|x| Value::String(x.into())).into()),
            ),
            ("origin_xz_m".into(), numbers(grid.origin)),
            ("cell_size_m".into(), Value::F64(grid.spacing.into())),
            ("time_s".into(), Value::F64(time.into())),
            ("ripple_steps".into(), Value::U64(ripple_steps as u64)),
            ("triangle_count".into(), Value::U64(triangle_count)),
        ])))
    }
}

fn spectrum(value: &Value, sea_level: f32) -> Result<SimdGerstnerSpectrum, String> {
    let harmonics = list(value)?;
    if harmonics.len() > MAX_GERSTNER_HARMONICS {
        return Err(format!(
            "At most {MAX_GERSTNER_HARMONICS} ocean harmonics are supported"
        ));
    }
    let mut spectrum = SimdGerstnerSpectrum {
        sea_level,
        active_waves: harmonics.len(),
        ..Default::default()
    };
    for (harmonic, output) in harmonics.iter().zip(&mut spectrum.waves) {
        let args = record(harmonic)?;
        fields(
            args,
            &["direction_xz", "amplitude_m", "wavelength_m", "phase_rad"],
        )?;
        let [x, z] = pair(required(args, "direction_xz")?, "direction_xz", 1.0)?;
        let length = x.hypot(z);
        if length < 1e-6 {
            return Err("Ocean harmonic direction must have positive length".into());
        }
        *output = GerstnerWave {
            direction: [x / length, z / length],
            amplitude: bound(args, "amplitude_m", None, 0.0, 100.0)?,
            wavelength: bound(args, "wavelength_m", None, 0.01, 100_000.0)?,
            steepness: 0.0,
            phase: bound(args, "phase_rad", Some(0.0), -1_000.0, 1_000.0)?,
        };
    }
    Ok(spectrum)
}

fn optics(value: &Value) -> Result<DielectricVolumeIop, String> {
    let args = record(value)?;
    fields(
        args,
        &[
            "cdom_g_m3",
            "tripton_g_m3",
            "chlorophyll_mg_m3",
            "salinity_psu",
            "temperature_c",
        ],
    )?;
    Ok(BioGeoOpticalConstituents {
        cdom_tannin_conc: bound(args, "cdom_g_m3", None, 0.0, 1_000.0)?,
        tripton_clay_silt_conc: bound(args, "tripton_g_m3", None, 0.0, 10_000.0)?,
        chlorophyll_conc: bound(args, "chlorophyll_mg_m3", None, 0.0, 1_000.0)?,
        salinity_psu: bound(args, "salinity_psu", None, 0.0, 50.0)?,
        water_temp_c: bound(args, "temperature_c", None, -3.0, 50.0)?,
    }
    .synthesize_iop())
}

fn optical_record(iop: DielectricVolumeIop) -> Value {
    Value::Record(BTreeMap::from([
        (
            "wavelength_nm".into(),
            numbers([WAVELENGTH_RED_NM, WAVELENGTH_GREEN_NM, WAVELENGTH_BLUE_NM]),
        ),
        ("ior".into(), numbers(iop.ior_spectrum)),
        ("absorption_per_m".into(), numbers(iop.absorption_coeff)),
        ("rayleigh_per_m".into(), numbers(iop.rayleigh_scattering)),
        (
            "mie_per_m".into(),
            numbers([
                iop.mie_scattering[0],
                iop.mie_scattering[1],
                iop.mie_scattering[2],
            ]),
        ),
        ("mie_g".into(), Value::F64(iop.mie_scattering[3].into())),
        ("attenuation_per_m".into(), numbers(iop.beam_attenuation())),
    ]))
}

fn bathymetry(host: &AssetHost<'_>, value: &Value, grid: Grid) -> Result<Vec<f32>, String> {
    let n = grid.width * grid.height;
    let samples = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Ocean bathymetry requires DenseF32 samples".into());
            };
            if axes.as_slice() != [grid.height as u32, grid.width as u32] || samples.len() != n {
                return Err("Ocean bathymetry shape must be [height, width]".into());
            }
            samples.clone()
        }
        Ok(_) => {
            return Err("Ocean bathymetry requires a DenseF32 field or flat sample list".into())
        }
        Err(_) => {
            let values = list(value)?;
            if values.len() != n {
                return Err("Ocean bathymetry requires width × height samples".into());
            }
            values
                .iter()
                .map(|value| num(value).map(|v| v as f32))
                .collect::<Result<_, _>>()?
        }
    };
    if samples.iter().any(|x| !(-20_000.0..=20_000.0).contains(x)) {
        return Err("Ocean bathymetry elevations must be finite and bounded by ±20,000 m".into());
    }
    Ok(samples)
}

fn impulses(
    args: &Args,
    grid: Grid,
    bed: &[f32],
    sea: f32,
    time: f32,
    ripple: &InteractiveRippleBuffer,
) -> Result<Vec<Impulse>, String> {
    let Some(colliders) = args.get("colliders") else {
        return Ok(Vec::new());
    };
    let colliders = list(colliders)?;
    if colliders.len() > MAX_COLLIDERS {
        return Err(format!(
            "Ocean collider list exceeds {MAX_COLLIDERS} timed impulses"
        ));
    }
    let mut result = Vec::with_capacity(colliders.len());
    let max_grid_index = (RIPPLE_GRID_DIM - 1) as f32;
    for collider in colliders {
        let args = record(collider)?;
        fields(args, &["time_s", "center_xz_m", "amplitude_m", "radius_m"])?;
        let center = pair(required(args, "center_xz_m")?, "center_xz_m", 2_000_000.0)?;
        let (gx, gz) = ripple.world_to_grid(center[0], center[1]);
        if gx <= 0.0
            || gz <= 0.0
            || gx >= max_grid_index
            || gz >= max_grid_index
            || grid.sample(bed, center)? >= sea
        {
            return Err(
                "Ocean collider must lie within the ripple domain on submerged bathymetry".into(),
            );
        }
        result.push(Impulse {
            time: bound(args, "time_s", None, 0.0, time)?,
            center,
            elevation: bound(args, "amplitude_m", None, -10.0, 10.0)?,
            radius: bound(
                args,
                "radius_m",
                None,
                ripple.cell_size(),
                ripple.world_size_m * 0.5,
            )?,
        });
    }
    result.sort_by(|a, b| a.time.total_cmp(&b.time));
    Ok(result)
}

fn advance_ripples(
    host: &mut AssetHost<'_>,
    ripple: &mut InteractiveRippleBuffer,
    impulses: Vec<Impulse>,
    end_time: f32,
    scratch: usize,
) -> Result<usize, String> {
    let max_dt = (ripple.cell_size() / ripple.wave_speed * 0.68).min(1.0 / 30.0);
    let mut elapsed = None;
    let mut total = 0;
    for (next, impulse) in impulses
        .into_iter()
        .map(|event| (event.time, Some(event)))
        .chain(std::iter::once((end_time, None)))
    {
        if let Some(previous) = elapsed {
            let duration: f32 = next - previous;
            let count = (duration / max_dt).ceil() as usize;
            total += count;
            if total > MAX_RIPPLE_STEPS {
                return Err(format!(
                    "Ocean ripple CFL step count exceeds {MAX_RIPPLE_STEPS}"
                ));
            }
            if count > 0 {
                host.charge(count * RIPPLE_TOTAL_CELLS * 16, scratch)?;
                for _ in 0..count {
                    ripple.step(duration / count as f32);
                }
            }
        }
        if let Some(event) = impulse {
            ripple.inject_impulse(
                event.center[0],
                event.center[1],
                event.elevation,
                event.radius,
            );
        }
        elapsed = Some(next);
    }
    Ok(total)
}

fn shore_distances(depth: &[f32], grid: Grid) -> Vec<f32> {
    let mut distance = vec![f32::INFINITY; depth.len()];
    for z in 0..grid.height {
        for x in 0..grid.width {
            let i = z * grid.width + x;
            for neighbour in [
                (x + 1 < grid.width).then_some(i + 1),
                (z + 1 < grid.height).then_some(i + grid.width),
            ]
            .into_iter()
            .flatten()
            {
                if (depth[i] > 0.0) != (depth[neighbour] > 0.0) {
                    let fraction = depth[i].abs() / (depth[i] - depth[neighbour]).abs();
                    distance[i] = distance[i].min(grid.spacing * fraction);
                    distance[neighbour] = distance[neighbour].min(grid.spacing * (1.0 - fraction));
                }
            }
        }
    }
    let diagonal = grid.spacing * std::f32::consts::SQRT_2;
    for (reverse, offsets) in [
        (false, [(-1, 0), (-1, -1), (0, -1), (1, -1)]),
        (true, [(1, 0), (-1, 1), (0, 1), (1, 1)]),
    ] {
        for ordinal in 0..depth.len() {
            let i = if reverse {
                depth.len() - ordinal - 1
            } else {
                ordinal
            };
            let x = i % grid.width;
            let z = i / grid.width;
            for (dx, dz) in offsets {
                let (Some(nx), Some(nz)) = (x.checked_add_signed(dx), z.checked_add_signed(dz))
                else {
                    continue;
                };
                if nx < grid.width && nz < grid.height {
                    let weight = if dx == 0 || dz == 0 {
                        grid.spacing
                    } else {
                        diagonal
                    };
                    distance[i] = distance[i].min(distance[nz * grid.width + nx] + weight);
                }
            }
        }
    }
    for (distance, &depth) in distance.iter_mut().zip(depth) {
        if depth <= 0.0 {
            *distance = -*distance;
        }
    }
    distance
}

fn clipped_surface(
    grid: Grid,
    elevations: &[f32],
    clearance: &[f32],
) -> Result<Option<ImportedMesh>, String> {
    let mut positions = Vec::new();
    let mut triangles = Vec::new();
    let mut keys = BTreeMap::new();
    let mut vertex = |mut a: usize, mut b: usize| {
        if clearance[a] == 0.0 {
            b = a;
        }
        if clearance[b] == 0.0 {
            a = b;
        }
        if b < a {
            (a, b) = (b, a);
        }
        *keys.entry((a, b)).or_insert_with(|| {
            let [x, z] = grid.point(a);
            let coord = if a == b {
                [x, elevations[a], z]
            } else {
                let [other_x, other_z] = grid.point(b);
                let t = clearance[a] / (clearance[a] - clearance[b]);
                [
                    x + t * (other_x - x),
                    elevations[a] + t * (elevations[b] - elevations[a]),
                    z + t * (other_z - z),
                ]
            };
            let next = positions.len() as u32;
            positions.push(coord);
            next
        })
    };
    for z in 0..grid.height - 1 {
        for x in 0..grid.width - 1 {
            let a = z * grid.width + x;
            let b = a + 1;
            let c = a + grid.width;
            let d = c + 1;
            for tri in [[a, c, b], [b, c, d]] {
                let mut polygon = [0; 4];
                let mut count = 0;
                for (start, end) in tri.into_iter().zip([tri[1], tri[2], tri[0]]) {
                    let inside = clearance[start] > 0.0;
                    if inside {
                        polygon[count] = vertex(start, start);
                        count += 1;
                    }
                    if inside != (clearance[end] > 0.0) {
                        let edge = vertex(start, end);
                        if count == 0 || polygon[count - 1] != edge {
                            polygon[count] = edge;
                            count += 1;
                        }
                    }
                }
                if count > 1 && polygon[0] == polygon[count - 1] {
                    count -= 1;
                }
                for i in 1..count.saturating_sub(1) {
                    let [a, b, c] = [polygon[0], polygon[i], polygon[i + 1]];
                    if a != b && b != c && c != a {
                        triangles.push([a, b, c]);
                    }
                }
            }
        }
    }
    if positions.iter().flatten().any(|x| !x.is_finite()) {
        return Err("Ocean surface contains non-finite vertices".into());
    }
    // Near-tangent intersections can round to an endpoint in the f32 mesh.
    // Remove the resulting zero-area faces and any unreferenced vertices.
    triangles.retain(|tri| face_normal(&positions, *tri)[1] > 0.0);
    if triangles.is_empty() {
        return Ok(None);
    }
    let mut normals = vec![[0.0_f64; 3]; positions.len()];
    for &triangle in &triangles {
        let normal = face_normal(&positions, triangle);
        for index in triangle {
            for (component, contribution) in normals[index as usize].iter_mut().zip(normal) {
                *component += contribution;
            }
        }
    }
    let mut compact = Vec::new();
    let mut compact_normals = Vec::new();
    let mut remap = vec![u32::MAX; positions.len()];
    for (index, (point, normal)) in positions.into_iter().zip(normals).enumerate() {
        if normal[1] > 0.0 {
            let length = normal[0].hypot(normal[1]).hypot(normal[2]);
            remap[index] = compact.len() as u32;
            compact.push(point);
            compact_normals.push(normal.map(|component| (component / length) as f32));
        }
    }
    for index in triangles.iter_mut().flatten() {
        *index = remap[*index as usize];
    }
    let positions = compact;
    let (min, max) = positions
        .iter()
        .fold((positions[0], positions[0]), |(min, max), p| {
            (
                std::array::from_fn(|axis| min[axis].min(p[axis])),
                std::array::from_fn(|axis| max[axis].max(p[axis])),
            )
        });
    Ok(Some(ImportedMesh {
        mesh: Mesh {
            positions,
            triangles,
            min,
            max,
        },
        normals: Some(compact_normals),
        tangents: None,
        texture_coordinates_0: None,
        materials: Vec::new(),
        texture_dependencies: Vec::new(),
        submeshes: Vec::new(),
    }))
}

fn face_normal(positions: &[[f32; 3]], [a, b, c]: [u32; 3]) -> [f64; 3] {
    let [a, b, c] = [a, b, c].map(|index| positions[index as usize].map(f64::from));
    let u: [f64; 3] = std::array::from_fn(|axis| b[axis] - a[axis]);
    let v: [f64; 3] = std::array::from_fn(|axis| c[axis] - a[axis]);
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn pair(value: &Value, name: &str, maximum: f32) -> Result<[f32; 2], String> {
    let [x, z] = vector::<2>(value)?;
    if x.abs() > f64::from(maximum) || z.abs() > f64::from(maximum) {
        return Err(format!("{name} components must be in ±{maximum}"));
    }
    Ok([x as f32, z as f32])
}

fn numbers<const N: usize>(values: [f32; N]) -> Value {
    Value::List(values.map(|x| Value::F64(x.into())).into())
}

fn bound(args: &Args, key: &str, default: Option<f32>, lo: f32, hi: f32) -> Result<f32, String> {
    let value = match args.get(key) {
        Some(value) => num(value)? as f32,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (lo..=hi)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must be in {lo}..={hi}"))
}

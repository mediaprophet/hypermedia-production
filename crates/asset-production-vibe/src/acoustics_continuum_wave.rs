//! Bounded native 3D fluid and elastic acoustic stepping, with authored time series probes.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::{AudioBuffer, FieldLattice},
    fields, integer, list, num, record, required, text, vector, Args, AssetHost, TypedArtifact,
    Value,
};
use qualia_core_db::audio::qac::numerical::{
    step_elastic_virieux_in_place, step_fluid_fdtd_in_place, ElasticBoundaryCondition,
    ElasticMedium, ElasticVirieuxConfig, ElasticVirieuxState, FluidBoundaryCondition,
    FluidFdtdConfig, FluidFdtdState, FluidMedium, NumericalError,
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_AXIS: usize = 128;
const MAX_CELLS: usize = 65_536;
const MAX_STEPS: usize = 16_384;
const MAX_SOURCES: usize = 32;
const MAX_PROBES: usize = 8;
const ELASTIC_PLANES: [&str; 7] = [
    "mean_compressive_stress_pa",
    "sigma_xx_pa",
    "sigma_yy_pa",
    "sigma_zz_pa",
    "sigma_xy_pa",
    "sigma_yz_pa",
    "sigma_xz_pa",
];

#[derive(Clone, Copy)]
struct Grid {
    dims: [usize; 3],
    spacing_m: [f64; 3],
}

impl Grid {
    fn cells(self) -> usize {
        self.dims.iter().product()
    }

    fn axes(self) -> [u32; 3] {
        let [nx, ny, nz] = self.dims;
        [nz as u32, ny as u32, nx as u32]
    }

    fn native_cell(self, value: &Value) -> Result<usize, String> {
        let [x, y, z] = list(value)? else {
            return Err("Acoustic cell must be [x, y, z] indices".into());
        };
        let [nx, ny, nz] = self.dims;
        let index = |value, extent| bounded_integer(value, "cell index", 0, extent - 1);
        Ok((index(x, nx)? * ny + index(y, ny)?) * nz + index(z, nz)?)
    }

    fn native_of_x_fast(self, index: usize) -> usize {
        let [nx, ny, nz] = self.dims;
        let (x, y, z) = (index % nx, (index / nx) % ny, index / (nx * ny));
        (x * ny + y) * nz + z
    }
}

struct Source {
    index: usize,
    start: usize,
    increments: Vec<f64>,
}

enum Solver {
    Fluid(FluidFdtdConfig, FluidFdtdState),
    Elastic(ElasticVirieuxConfig, ElasticVirieuxState),
}

impl Solver {
    fn step(&mut self) -> Result<(), NumericalError> {
        match self {
            Self::Fluid(cfg, state) => step_fluid_fdtd_in_place(cfg, state),
            Self::Elastic(cfg, state) => step_elastic_virieux_in_place(cfg, state),
        }
    }

    fn inject(&mut self, index: usize, compression_pa: f64) {
        match self {
            Self::Fluid(_, state) => state.p[index] += compression_pa,
            Self::Elastic(_, state) => {
                state.sxx[index] -= compression_pa;
                state.syy[index] -= compression_pa;
                state.szz[index] -= compression_pa;
            }
        }
    }

    fn pressure(&self, index: usize) -> f64 {
        match self {
            Self::Fluid(_, state) => state.p[index],
            Self::Elastic(_, state) => {
                -(state.sxx[index] + state.syy[index] + state.szz[index]) / 3.0
            }
        }
    }

    fn quantity(&self) -> &'static str {
        match self {
            Self::Fluid(_, _) => "pressure_pa",
            Self::Elastic(_, _) => "mean_compressive_stress_pa",
        }
    }

    fn planes(&self) -> (&[&str], &[[f64; 3]]) {
        match self {
            Self::Fluid(_, _) => (&["pressure_pa"], &[[0.0; 3]]),
            Self::Elastic(_, _) => (
                &ELASTIC_PLANES,
                &[
                    [0.0; 3],
                    [0.0; 3],
                    [0.0; 3],
                    [0.0; 3],
                    [0.5, 0.5, 0.0],
                    [0.0, 0.5, 0.5],
                    [0.5, 0.0, 0.5],
                ],
            ),
        }
    }

    fn speed(&self) -> f64 {
        match self {
            Self::Fluid(cfg, _) => cfg.medium.sound_speed_m_s(),
            Self::Elastic(cfg, _) => cfg.medium.p_wave_speed_m_s(),
        }
    }

    fn field(&self, grid: Grid) -> Result<FieldLattice, String> {
        let (names, _) = self.planes();
        let n = grid.cells();
        let mut samples = Vec::with_capacity(n * names.len());
        for index in 0..n {
            samples.push(self.pressure(grid.native_of_x_fast(index)) as f32);
        }
        if let Self::Elastic(_, state) = self {
            for lane in [
                &state.sxx, &state.syy, &state.szz, &state.sxy, &state.syz, &state.sxz,
            ] {
                samples.extend((0..n).map(|index| lane[grid.native_of_x_fast(index)] as f32));
            }
        }
        if samples.iter().any(|x| !x.is_finite()) {
            return Err("Acoustic continuum projection exceeds finite f32 output".into());
        }
        let [nz, ny, nx] = grid.axes();
        Ok(FieldLattice::DenseF32 {
            axes: vec![names.len() as u32, nz, ny, nx],
            samples,
        })
    }
}

impl AssetHost<'_> {
    pub(crate) fn solve_continuum_wave_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mode",
                "grid_dims",
                "spacing_m",
                "sample_rate_hz",
                "steps",
                "medium",
                "boundary",
                "cpml",
                "initial_pressure_pa",
                "initial_stress_pa",
                "sources",
                "probe_cells",
                "pa_per_full_scale",
            ],
        )?;
        let grid = grid(args)?;
        let sample_rate = bounded_integer(
            required(args, "sample_rate_hz")?,
            "sample_rate_hz",
            8_000,
            384_000,
        )?;
        let steps = bounded_integer(required(args, "steps")?, "steps", 1, MAX_STEPS)?;
        let scale = number(args, "pa_per_full_scale", None, 1e-12, 1e24)?;
        let mode = text(required(args, "mode")?)?;
        let boundary = text(required(args, "boundary")?)?;
        let dt_s = 1.0 / sample_rate as f64;
        let n = grid.cells();
        let scratch = n * 192 + steps * (MAX_SOURCES * 8 + MAX_PROBES * 4) + 64 * 1024;
        self.charge(n * 8, scratch)?;
        let probes = probe_indices(required(args, "probe_cells")?, grid)?;
        let sources = parse_sources(args, grid, steps)?;
        let (mut solver, courant) = create_solver(self, args, grid, mode, boundary, dt_s)?;
        let mut audio = AudioBuffer::new(sample_rate as u32, probes.len(), steps);
        for step in 0..steps {
            self.charge(n * 104 + sources.len(), scratch)?;
            for source in &sources {
                if let Some(increment) = step
                    .checked_sub(source.start)
                    .and_then(|i| source.increments.get(i))
                {
                    solver.inject(source.index, *increment);
                }
            }
            solver
                .step()
                .map_err(|err| format!("Acoustic step {step}: {err:?}"))?;
            for (probe, channel) in probes.iter().zip(&mut audio.channels) {
                let sample = solver.pressure(*probe) / scale;
                if !sample.is_finite() || sample.abs() > 1.0 {
                    return Err(format!(
                        "Probe output at step {} exceeds pa_per_full_scale",
                        step + 1
                    ));
                }
                channel[step] = sample as f32;
            }
        }
        let (names, offsets) = solver.planes();
        let fields =
            self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(solver.field(grid)?)))?;
        let audio = self.admit_artifact(TypedArtifact::AudioMultiChannel(Arc::new(audio)))?;
        Ok(Value::Record(BTreeMap::from([
            ("fields".into(), fields),
            ("audio".into(), audio),
            ("mode".into(), Value::String(mode.into())),
            ("boundary".into(), Value::String(boundary.into())),
            (
                "channel_names".into(),
                Value::List(
                    names
                        .iter()
                        .map(|&name| Value::String(name.into()))
                        .collect::<Vec<_>>()
                        .into(),
                ),
            ),
            (
                "channel_offsets_cells_xyz".into(),
                Value::List(
                    offsets
                        .iter()
                        .map(|offset| Value::List(offset.map(Value::F64).into()))
                        .collect::<Vec<_>>()
                        .into(),
                ),
            ),
            (
                "probe_quantity".into(),
                Value::String(solver.quantity().into()),
            ),
            ("pa_per_full_scale".into(), Value::F64(scale)),
            ("sample_rate_hz".into(), Value::U64(sample_rate as u64)),
            ("first_audio_sample_time_s".into(), Value::F64(dt_s)),
            ("end_time_s".into(), Value::F64(dt_s * steps as f64)),
            ("courant".into(), Value::F64(courant)),
            ("compressional_speed_m_s".into(), Value::F64(solver.speed())),
        ])))
    }
}

fn grid(args: &Args) -> Result<Grid, String> {
    let [x, y, z] = list(required(args, "grid_dims")?)? else {
        return Err("grid_dims must be [nx, ny, nz]".into());
    };
    let dim = |axis| bounded_integer(axis, "grid dimension", 2, MAX_AXIS);
    let dims = [dim(x)?, dim(y)?, dim(z)?];
    if dims.iter().product::<usize>() > MAX_CELLS {
        return Err(format!("Acoustic grid exceeds {MAX_CELLS} cells"));
    }
    let spacing_m = vector::<3>(required(args, "spacing_m")?)?;
    if spacing_m.iter().any(|x| !(1e-6..=1e6).contains(x)) {
        return Err("spacing_m components must lie between 1e-6 and 1e6 m".into());
    }
    Ok(Grid { dims, spacing_m })
}

fn create_solver(
    host: &AssetHost<'_>,
    args: &Args,
    grid: Grid,
    mode: &str,
    boundary: &str,
    dt_s: f64,
) -> Result<(Solver, f64), String> {
    let medium = record(required(args, "medium")?)?;
    let density = number(medium, "density_kg_m3", None, 1e-6, 1e6)?;
    let bulk = number(medium, "bulk_modulus_pa", None, 1e-6, 1e15)?;
    let error = |err| format!("Acoustic {mode}: {err:?}");
    match mode {
        "fluid" => {
            fields(
                medium,
                &["density_kg_m3", "bulk_modulus_pa", "damping_per_s"],
            )?;
            if args.contains_key("initial_stress_pa") {
                return Err("initial_stress_pa applies to elastic mode".into());
            }
            let damping = number(medium, "damping_per_s", Some(0.0), 0.0, 1e6)?;
            let cfg = FluidFdtdConfig {
                dims: grid.dims,
                spacing_m: grid.spacing_m,
                dt_s,
                medium: FluidMedium::new(density, bulk, damping, 0).map_err(error)?,
                boundary: fluid_boundary(args, boundary)?,
            };
            let courant = cfg.validate().map_err(error)?;
            let mut state = FluidFdtdState::new(&cfg).map_err(error)?;
            if let Some(p) = args.get("initial_pressure_pa") {
                read_field(host, p, grid, &mut state.p)?;
            }
            Ok((Solver::Fluid(cfg, state), courant))
        }
        "elastic" => {
            fields(
                medium,
                &["density_kg_m3", "bulk_modulus_pa", "shear_modulus_pa"],
            )?;
            if args.contains_key("initial_pressure_pa") || args.contains_key("cpml") {
                return Err("initial_pressure_pa and cpml apply to fluid mode".into());
            }
            let boundary = match boundary {
                "periodic" => ElasticBoundaryCondition::Periodic,
                "free_surface" => ElasticBoundaryCondition::FreeSurface,
                _ => return Err("Elastic boundary must be periodic or free_surface".into()),
            };
            let shear = number(medium, "shear_modulus_pa", None, 1e-6, 1e15)?;
            let cfg = ElasticVirieuxConfig {
                dims: grid.dims,
                spacing_m: grid.spacing_m,
                dt_s,
                medium: ElasticMedium::new_solid(density, bulk, shear, 0).map_err(error)?,
                boundary,
            };
            let courant = cfg.validate().map_err(error)?;
            let mut state = ElasticVirieuxState::new(&cfg).map_err(error)?;
            if let Some(stress) = args.get("initial_stress_pa") {
                let [xx, yy, zz, xy, yz, xz] = list(stress)? else {
                    return Err("initial_stress_pa must be [xx, yy, zz, xy, yz, xz]".into());
                };
                for (source, target) in [
                    (xx, &mut state.sxx),
                    (yy, &mut state.syy),
                    (zz, &mut state.szz),
                    (xy, &mut state.sxy),
                    (yz, &mut state.syz),
                    (xz, &mut state.sxz),
                ] {
                    read_field(host, source, grid, target)?;
                }
            }
            Ok((Solver::Elastic(cfg, state), courant))
        }
        _ => Err("Acoustic mode must be fluid or elastic".into()),
    }
}

fn fluid_boundary(args: &Args, name: &str) -> Result<FluidBoundaryCondition, String> {
    if name != "cpml" && args.contains_key("cpml") {
        return Err("cpml parameters require the cpml fluid boundary".into());
    }
    match name {
        "periodic" => Ok(FluidBoundaryCondition::Periodic),
        "rigid" => Ok(FluidBoundaryCondition::Rigid),
        "pressure_release" => Ok(FluidBoundaryCondition::PressureRelease),
        "engquist_majda" => Ok(FluidBoundaryCondition::EngquistMajdaFirstOrder),
        "cpml" => {
            let cpml = record(required(args, "cpml")?)?;
            fields(
                cpml,
                &["layers", "sigma_max_per_s", "kappa_max", "alpha_max_per_s"],
            )?;
            Ok(FluidBoundaryCondition::Cpml {
                layers: bounded_integer(required(cpml, "layers")?, "layers", 1, 16)?,
                sigma_max: number(cpml, "sigma_max_per_s", None, 0.0, 1e9)?,
                kappa_max: number(cpml, "kappa_max", None, 1.0, 1e4)?,
                alpha_max: number(cpml, "alpha_max_per_s", None, 0.0, 1e9)?,
            })
        }
        _ => Err(
            "Fluid boundary must be periodic, rigid, pressure_release, engquist_majda or cpml"
                .into(),
        ),
    }
}

fn probe_indices(value: &Value, grid: Grid) -> Result<Vec<usize>, String> {
    let probes = list(value)?;
    if !(1..=MAX_PROBES).contains(&probes.len()) {
        return Err(format!(
            "probe_cells requires 1..={MAX_PROBES} authored probes"
        ));
    }
    probes.iter().map(|probe| grid.native_cell(probe)).collect()
}

fn parse_sources(args: &Args, grid: Grid, steps: usize) -> Result<Vec<Source>, String> {
    let Some(sources) = args.get("sources") else {
        return Ok(Vec::new());
    };
    let sources = list(sources)?;
    if sources.len() > MAX_SOURCES {
        return Err(format!("Acoustic source count exceeds {MAX_SOURCES}"));
    }
    sources
        .iter()
        .map(|value| {
            let source = record(value)?;
            fields(source, &["cell", "start_step", "increments_pa"])?;
            let index = grid.native_cell(required(source, "cell")?)?;
            let start = source
                .get("start_step")
                .map(|v| bounded_integer(v, "start_step", 0, steps - 1))
                .transpose()?
                .unwrap_or(0);
            let values = list(required(source, "increments_pa")?)?;
            if values.is_empty() || values.len() > steps - start {
                return Err("Acoustic source increments must fit at its start_step".into());
            }
            let increments = values
                .iter()
                .map(|value| num(value).and_then(amplitude))
                .collect::<Result<_, _>>()?;
            Ok(Source {
                index,
                start,
                increments,
            })
        })
        .collect()
}

fn read_field(
    host: &AssetHost<'_>,
    value: &Value,
    grid: Grid,
    target: &mut [f64],
) -> Result<(), String> {
    match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Acoustic initial field must use DenseF32 samples".into());
            };
            if axes.as_slice() != grid.axes() || samples.len() != target.len() {
                return Err("Acoustic initial field requires shape [nz, ny, nx]".into());
            }
            for (index, &value) in samples.iter().enumerate() {
                target[grid.native_of_x_fast(index)] = amplitude(f64::from(value))?;
            }
        }
        Ok(_) => return Err("Acoustic initial field must be numeric or a DenseF32 lattice".into()),
        Err(_) => match value {
            Value::List(samples) if samples.len() == target.len() => {
                for (index, value) in samples.iter().enumerate() {
                    target[grid.native_of_x_fast(index)] = amplitude(num(value)?)?;
                }
            }
            Value::List(_) => {
                return Err("Acoustic initial sample count differs from nx×ny×nz".into())
            }
            uniform => target.fill(amplitude(num(uniform)?)?),
        },
    }
    Ok(())
}

fn amplitude(value: f64) -> Result<f64, String> {
    (-1e15..=1e15)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| "Acoustic input amplitudes must be finite and bounded by ±1e15 Pa".into())
}

fn number(args: &Args, key: &str, default: Option<f64>, lo: f64, hi: f64) -> Result<f64, String> {
    let value = match args.get(key) {
        Some(value) => num(value)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (lo..=hi)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must lie in {lo}..={hi}"))
}

fn bounded_integer(value: &Value, name: &str, lo: usize, hi: usize) -> Result<usize, String> {
    let value = integer(value, name)?;
    (lo..=hi)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{name} must lie in {lo}..={hi}"))
}

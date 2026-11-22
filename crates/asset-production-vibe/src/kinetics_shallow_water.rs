//! Bounded hydrostatic finite-volume authoring for a square-cell 2.5D water grid.

use crate::{
    artifact::FieldLattice, fields, integer, list, num, record, required, vector, Args, AssetHost,
    TypedArtifact, Value,
};
use std::{collections::BTreeMap, sync::Arc};

const MAX_DIM: usize = 512;
const MAX_CELLS: usize = 65_536;
const MAX_DEPTH_M: f64 = 1e6;
const MAX_FLOW_SPEED_M_S: f64 = 1e4;

#[derive(Clone, Copy)]
struct Grid {
    width: usize,
    height: usize,
    spacing: f64,
}

impl Grid {
    fn len(self) -> usize {
        self.width * self.height
    }
}

#[derive(Clone, Copy, Default)]
struct State {
    depth: f64,
    momentum: [f64; 2],
    tracer_mass: f64,
}

impl State {
    fn velocity(self, wet: f64) -> [f64; 2] {
        if self.depth > wet {
            self.momentum.map(|q| q / self.depth)
        } else {
            [0.0; 2]
        }
    }
    fn concentration(self) -> f64 {
        if self.depth > 0.0 {
            self.tracer_mass / self.depth
        } else {
            0.0
        }
    }
    fn add_scaled(&mut self, flux: Self, scale: f64) {
        self.depth += scale * flux.depth;
        self.tracer_mass += scale * flux.tracer_mass;
        for (q, f) in self.momentum.iter_mut().zip(flux.momentum) {
            *q += scale * f;
        }
    }
}

#[derive(Clone, Copy)]
struct Fluid {
    gravity: f64,
    wet_depth: f64,
    cfl: f64,
}

#[derive(Clone, Copy, Default)]
struct Ledger {
    rain: f64,
    inflow: f64,
    infiltration: f64,
    tracer_inflow: f64,
    tracer_infiltration: f64,
}

#[derive(Clone, Copy)]
struct Forcing {
    rainfall: f64,
    infiltration: f64,
}

impl AssetHost<'_> {
    pub(crate) fn solve_shallow_water_25d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "width",
                "height",
                "cell_size_m",
                "bathymetry_m",
                "initial_depth_m",
                "initial_surface_m",
                "initial_velocity_uv_m_s",
                "initial_tracer",
                "manning_n",
                "rain_rate_m_s",
                "infiltration_rate_m_s",
                "inflows",
                "dt_s",
                "steps",
                "gravity_m_s2",
                "wet_depth_m",
                "cfl",
                "max_substeps_per_step",
            ],
        )?;
        let width = bounded_integer(args, "width", None, 1, MAX_DIM)?;
        let height = bounded_integer(args, "height", None, 1, MAX_DIM)?;
        let grid = Grid {
            width,
            height,
            spacing: bounded_number(args, "cell_size_m", None, 1e-3, 100_000.0)?,
        };
        if grid.len() > MAX_CELLS {
            return Err("Shallow-water raster exceeds 65,536 cells".into());
        }
        let timestep = bounded_number(args, "dt_s", None, 1e-6, 3600.0)?;
        let steps = bounded_integer(args, "steps", None, 1, 1024)?;
        let substep_limit = bounded_integer(args, "max_substeps_per_step", Some(128), 1, 256)?;
        let fluid = Fluid {
            gravity: bounded_number(args, "gravity_m_s2", Some(9.80665), 0.01, 100.0)?,
            wet_depth: bounded_number(args, "wet_depth_m", Some(1e-4), 1e-8, 1.0)?,
            cfl: bounded_number(args, "cfl", Some(0.45), 0.01, 0.5)?,
        };
        let forcing = Forcing {
            rainfall: bounded_number(args, "rain_rate_m_s", None, 0.0, 10.0)?,
            infiltration: bounded_number(args, "infiltration_rate_m_s", Some(0.0), 0.0, 10.0)?,
        };
        let scratch = grid.len() * 272;
        self.charge(grid.len() * 32, scratch)?;
        let bathymetry = raster(self, required(args, "bathymetry_m")?, grid, [-2e6, 2e6])?;
        let roughness = raster(self, required(args, "manning_n")?, grid, [0.0, 1.0])?;
        let mut water = initial_state(self, args, grid, &bathymetry, fluid.wet_depth)?;
        let inflows = source_rates(args, grid)?;
        let mut next = vec![State::default(); grid.len()];
        let mut ledger = Ledger::default();
        let mut substeps = 0u64;
        let area = grid.spacing * grid.spacing;
        let initial_volume = water.iter().map(|state| state.depth).sum::<f64>() * area;
        let initial_tracer = water.iter().map(|state| state.tracer_mass).sum::<f64>() * area;
        for _ in 0..steps {
            let mut remaining = timestep;
            for substep in 0..substep_limit {
                self.charge(grid.len() * 120, scratch)?;
                let dt = remaining.min(cfl_step(grid, fluid, &water));
                if dt <= 0.0 || !dt.is_finite() {
                    return Err("Shallow-water CFL calculation failed to advance time".into());
                }
                advance(grid, fluid, &bathymetry, &water, &mut next, dt);
                apply_sources(
                    fluid,
                    forcing,
                    &mut next,
                    &inflows,
                    &roughness,
                    dt,
                    &mut ledger,
                )?;
                std::mem::swap(&mut water, &mut next);
                substeps += 1;
                if dt == remaining {
                    break;
                }
                remaining -= dt;
                if substep + 1 == substep_limit {
                    return Err(format!("Shallow-water CFL requires more than {substep_limit} substeps for an authored time step"));
                }
            }
        }
        let final_volume = water.iter().map(|state| state.depth).sum::<f64>() * area;
        let final_tracer = water.iter().map(|state| state.tracer_mass).sum::<f64>() * area;
        let rain = ledger.rain * area;
        let inflow = ledger.inflow * area;
        let infiltrated = ledger.infiltration * area;
        let expected = initial_volume + rain + inflow - infiltrated;
        let residual = final_volume - expected;
        let tracer_residual = final_tracer
            - (initial_tracer + area * (ledger.tracer_inflow - ledger.tracer_infiltration));
        let tolerance = (initial_volume + rain + inflow).max(1.0) * 1e-9;
        if !residual.is_finite()
            || !tracer_residual.is_finite()
            || residual.abs() > tolerance
            || tracer_residual.abs() > tolerance
        {
            return Err(format!("Closed shallow-water mass gate failed: water residual {residual:.12e} m3, tracer residual {tracer_residual:.12e} m3-equivalent, tolerance {tolerance:.12e}"));
        }
        let names = [
            "depth_m",
            "velocity_u_m_s",
            "velocity_v_m_s",
            "tracer_fraction",
            "wet_mask",
        ];
        let mut planes = Vec::with_capacity(grid.len() * names.len());
        planes.extend(water.iter().map(|state| state.depth as f32));
        for axis in 0..2 {
            planes.extend(
                water
                    .iter()
                    .map(|state| state.velocity(fluid.wet_depth)[axis] as f32),
            );
        }
        planes.extend(water.iter().map(|state| state.concentration() as f32));
        planes.extend(water.iter().map(|state| {
            if state.depth > fluid.wet_depth {
                1.0
            } else {
                0.0
            }
        }));
        let field = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![names.len() as u32, height as u32, width as u32],
                samples: planes,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("water".into(), field),
            (
                "plane_names".into(),
                Value::List(names.map(|name| Value::String(name.into())).into()),
            ),
            ("boundary".into(), Value::String("reflective".into())),
            ("cell_size_m".into(), Value::F64(grid.spacing)),
            ("gravity_m_s2".into(), Value::F64(fluid.gravity)),
            ("cfl".into(), Value::F64(fluid.cfl)),
            (
                "simulated_seconds".into(),
                Value::F64(timestep * steps as f64),
            ),
            ("substeps".into(), Value::U64(substeps)),
            ("wet_depth_m".into(), Value::F64(fluid.wet_depth)),
            ("initial_volume_m3".into(), Value::F64(initial_volume)),
            ("rain_volume_m3".into(), Value::F64(rain)),
            ("inflow_volume_m3".into(), Value::F64(inflow)),
            ("infiltrated_volume_m3".into(), Value::F64(infiltrated)),
            ("final_volume_m3".into(), Value::F64(final_volume)),
            ("water_balance_residual_m3".into(), Value::F64(residual)),
            (
                "tracer_balance_residual_m3_equivalent".into(),
                Value::F64(tracer_residual),
            ),
        ])))
    }
}

fn bounded_integer(
    args: &Args,
    key: &str,
    default: Option<usize>,
    min: usize,
    max: usize,
) -> Result<usize, String> {
    let value = match args.get(key) {
        Some(value) => integer(value, key)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} is outside {min}..={max}"))
}

fn bounded_number(
    args: &Args,
    key: &str,
    default: Option<f64>,
    min: f64,
    max: f64,
) -> Result<f64, String> {
    let value = match args.get(key) {
        Some(value) => num(value)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} is outside {min}..={max}"))
}

fn raster(
    host: &AssetHost<'_>,
    value: &Value,
    grid: Grid,
    limits: [f64; 2],
) -> Result<Vec<f64>, String> {
    let samples = match host.workspace.typed(value) {
        Ok(TypedArtifact::FieldLattice(field)) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Expected a DenseF32 scalar field".into());
            };
            if axes.as_slice() != [grid.height as u32, grid.width as u32]
                || samples.len() != grid.len()
            {
                return Err("Scalar field must have shape [height, width]".into());
            }
            samples.iter().copied().map(f64::from).collect::<Vec<_>>()
        }
        Ok(_) => return Err("Expected a scalar field, numeric list or uniform number".into()),
        Err(_) => match value {
            Value::List(samples) => {
                if samples.len() != grid.len() {
                    return Err("Scalar raster count differs from width × height".into());
                }
                samples.iter().map(num).collect::<Result<_, _>>()?
            }
            uniform => vec![num(uniform)?; grid.len()],
        },
    };
    if samples
        .iter()
        .any(|&sample| !(limits[0]..=limits[1]).contains(&sample))
    {
        return Err("Scalar raster contains non-finite or out-of-range values".into());
    }
    Ok(samples)
}

fn initial_state(
    host: &AssetHost<'_>,
    args: &Args,
    grid: Grid,
    bed: &[f64],
    wet: f64,
) -> Result<Vec<State>, String> {
    let depth = match (args.get("initial_depth_m"), args.get("initial_surface_m")) {
        (Some(depth), None) => raster(host, depth, grid, [0.0, MAX_DEPTH_M])?,
        (None, Some(surface)) => raster(host, surface, grid, [-2e6, 2e6])?
            .into_iter()
            .zip(bed)
            .map(|(surface, bed)| (surface - bed).max(0.0))
            .collect(),
        _ => return Err("Supply exactly one initial depth or free-surface elevation field".into()),
    };
    if depth.iter().any(|&depth| depth > MAX_DEPTH_M) {
        return Err("Initial water depth exceeds its supported metric range".into());
    }
    let velocity = args.get("initial_velocity_uv_m_s").map(list).transpose()?;
    let [u, v] = match velocity {
        Some([u, v]) => [
            raster(host, u, grid, [-1000.0, 1000.0])?,
            raster(host, v, grid, [-1000.0, 1000.0])?,
        ],
        None => [vec![0.0; grid.len()], vec![0.0; grid.len()]],
        _ => return Err("Initial velocity requires two scalar fields [u, v]".into()),
    };
    let tracer = args
        .get("initial_tracer")
        .map(|v| raster(host, v, grid, [0.0, 1.0]))
        .transpose()?
        .unwrap_or_else(|| vec![0.0; grid.len()]);
    Ok(depth
        .into_iter()
        .zip(u)
        .zip(v)
        .zip(tracer)
        .map(|(((depth, u), v), concentration)| State {
            depth,
            momentum: if depth > wet {
                [depth * u, depth * v]
            } else {
                [0.0; 2]
            },
            tracer_mass: depth * concentration,
        })
        .collect())
}

fn source_rates(args: &Args, grid: Grid) -> Result<Vec<State>, String> {
    let mut rates = vec![State::default(); grid.len()];
    let inflows = args.get("inflows").map(list).transpose()?.unwrap_or(&[]);
    if inflows.len() > 256 {
        return Err("Shallow-water authoring permits at most 256 inflows".into());
    }
    for value in inflows {
        let source = record(value)?;
        fields(
            source,
            &[
                "row",
                "column",
                "discharge_m3_s",
                "velocity_uv_m_s",
                "tracer",
            ],
        )?;
        let row = bounded_integer(source, "row", None, 0, grid.height - 1)?;
        let column = bounded_integer(source, "column", None, 0, grid.width - 1)?;
        let discharge = bounded_number(source, "discharge_m3_s", None, 0.0, 1e6)?
            / (grid.spacing * grid.spacing);
        let velocity = source
            .get("velocity_uv_m_s")
            .map(vector::<2>)
            .transpose()?
            .unwrap_or([0.0; 2]);
        if velocity.iter().any(|v| v.abs() > 1000.0) {
            return Err("Inflow velocity exceeds its supported metric range".into());
        }
        let tracer = bounded_number(source, "tracer", Some(0.0), 0.0, 1.0)?;
        rates[row * grid.width + column].add_scaled(
            State {
                depth: discharge,
                momentum: velocity.map(|v| discharge * v),
                tracer_mass: discharge * tracer,
            },
            1.0,
        );
    }
    Ok(rates)
}

fn cfl_step(grid: Grid, fluid: Fluid, water: &[State]) -> f64 {
    let [u, v] = water.iter().fold([0.0_f64; 2], |[u, v], &state| {
        let [x, y] = state.velocity(fluid.wet_depth);
        let wave = (fluid.gravity * state.depth).sqrt();
        [u.max(x.abs() + wave), v.max(y.abs() + wave)]
    });
    if u + v == 0.0 {
        f64::INFINITY
    } else {
        fluid.cfl * grid.spacing / (2.0 * (u + v))
    }
}

fn pressure(h: f64, gravity: f64) -> f64 {
    0.5 * gravity * h * h
}

fn physical_flux(depth: f64, velocity: [f64; 2], axis: usize, gravity: f64) -> State {
    let discharge = depth * velocity[axis];
    let mut momentum = velocity.map(|v| discharge * v);
    momentum[axis] += pressure(depth, gravity);
    State {
        depth: discharge,
        momentum,
        tracer_mass: 0.0,
    }
}

fn interface(left: State, right: State, bed: [f64; 2], axis: usize, fluid: Fluid) -> [State; 2] {
    let face_bed = bed[0].max(bed[1]);
    // Subtract the bed jump before combining with depth to avoid cancellation
    // from large absolute vertical datums.
    let a = (left.depth - (face_bed - bed[0])).max(0.0);
    let b = (right.depth - (face_bed - bed[1])).max(0.0);
    let va = left.velocity(fluid.wet_depth);
    let vb = right.velocity(fluid.wet_depth);
    let left_wave = (fluid.gravity * a).sqrt();
    let right_wave = (fluid.gravity * b).sqrt();
    let left_speed = if a > 0.0 { va[axis] } else { 0.0 };
    let right_speed = if b > 0.0 { vb[axis] } else { 0.0 };
    let plus = 0.0_f64
        .max(left_speed + left_wave)
        .max(right_speed + right_wave);
    let minus = 0.0_f64
        .min(left_speed - left_wave)
        .min(right_speed - right_wave);
    let mut flux = State::default();
    if plus > minus {
        let fa = physical_flux(a, va, axis, fluid.gravity);
        let fb = physical_flux(b, vb, axis, fluid.gravity);
        let wave_product = plus * minus;
        let denominator = plus - minus;
        flux.depth = (plus * fa.depth - minus * fb.depth + wave_product * (b - a)) / denominator;
        flux.momentum = std::array::from_fn(|lane| {
            (plus * fa.momentum[lane] - minus * fb.momentum[lane]
                + wave_product * (b * vb[lane] - a * va[lane]))
                / denominator
        });
        flux.tracer_mass = flux.depth
            * if flux.depth >= 0.0 {
                left.concentration()
            } else {
                right.concentration()
            };
    }
    let mut left_flux = flux;
    let mut right_flux = flux;
    // Use the cell-side hydrostatic pressure corrections at the same interface.
    left_flux.momentum[axis] += pressure(left.depth, fluid.gravity) - pressure(a, fluid.gravity);
    right_flux.momentum[axis] += pressure(right.depth, fluid.gravity) - pressure(b, fluid.gravity);
    [left_flux, right_flux]
}

fn wall(state: State, bed: f64, axis: usize, upper: bool, fluid: Fluid) -> State {
    let mut ghost = state;
    ghost.momentum[axis] = -ghost.momentum[axis];
    let mut flux = if upper {
        interface(state, ghost, [bed; 2], axis, fluid)[0]
    } else {
        interface(ghost, state, [bed; 2], axis, fluid)[1]
    };
    flux.depth = 0.0;
    flux.tracer_mass = 0.0;
    flux
}

fn advance(grid: Grid, fluid: Fluid, bed: &[f64], old: &[State], next: &mut [State], dt: f64) {
    next.copy_from_slice(old);
    let scale = dt / grid.spacing;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let i = y * grid.width + x;
            if x == 0 {
                next[i].add_scaled(wall(old[i], bed[i], 0, false, fluid), scale);
            }
            if y == 0 {
                next[i].add_scaled(wall(old[i], bed[i], 1, false, fluid), scale);
            }
            for (axis, outside, neighbor) in [
                (0, x + 1 == grid.width, i + 1),
                (1, y + 1 == grid.height, i + grid.width),
            ] {
                if outside {
                    next[i].add_scaled(wall(old[i], bed[i], axis, true, fluid), -scale);
                } else {
                    let [local, remote] =
                        interface(old[i], old[neighbor], [bed[i], bed[neighbor]], axis, fluid);
                    next[i].add_scaled(local, -scale);
                    next[neighbor].add_scaled(remote, scale);
                }
            }
        }
    }
}

fn apply_sources(
    fluid: Fluid,
    forcing: Forcing,
    water: &mut [State],
    rates: &[State],
    roughness: &[f64],
    dt: f64,
    ledger: &mut Ledger,
) -> Result<(), String> {
    let rain = forcing.rainfall * dt;
    ledger.rain += rain * water.len() as f64;
    for (index, ((cell, &rate), &n)) in water.iter_mut().zip(rates).zip(roughness).enumerate() {
        let tolerance = 128.0 * f64::EPSILON * cell.depth.abs().max(1.0);
        if !cell.depth.is_finite()
            || cell.depth < -tolerance
            || !cell.tracer_mass.is_finite()
            || cell.tracer_mass < -tolerance
            || cell.tracer_mass > cell.depth.max(0.0) + tolerance
            || cell.momentum.iter().any(|value| !value.is_finite())
        {
            return Err(format!("Shallow-water positivity gate at cell {index}: depth {:.12e} m, tracer {:.12e} m-equivalent", cell.depth, cell.tracer_mass));
        }
        cell.depth = cell.depth.max(0.0);
        cell.tracer_mass = cell.tracer_mass.clamp(0.0, cell.depth);
        cell.depth += rain;
        cell.add_scaled(rate, dt);
        ledger.inflow += rate.depth * dt;
        ledger.tracer_inflow += rate.tracer_mass * dt;
        let infiltration = cell.depth.min(forcing.infiltration * dt);
        let retained = if cell.depth > 0.0 {
            (cell.depth - infiltration) / cell.depth
        } else {
            0.0
        };
        let tracer_before = cell.tracer_mass;
        cell.depth -= infiltration;
        cell.momentum = cell.momentum.map(|q| q * retained);
        cell.tracer_mass = (tracer_before * retained).min(cell.depth);
        ledger.infiltration += infiltration;
        ledger.tracer_infiltration += tracer_before - cell.tracer_mass;
        if cell.depth > fluid.wet_depth {
            let speed = cell.momentum[0].hypot(cell.momentum[1]) / cell.depth;
            let damping = 1.0 + dt * fluid.gravity * n * n * speed / cell.depth.powf(4.0 / 3.0);
            cell.momentum = cell.momentum.map(|q| q / damping);
        } else {
            cell.momentum = [0.0; 2];
        }
        let speed = cell.velocity(fluid.wet_depth);
        if !cell.depth.is_finite()
            || cell.depth > MAX_DEPTH_M
            || speed
                .iter()
                .any(|v| !v.is_finite() || v.abs() > MAX_FLOW_SPEED_M_S)
        {
            return Err(format!(
                "Shallow-water state at cell {index} exceeds its finite depth or velocity envelope"
            ));
        }
    }
    Ok(())
}

//! D3Q19 low-Mach flow with stationary halfway bounce-back.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{dot, input_plane, int_arg, number_arg, output_plane, read_dimensions};
use crate::{Args, AssetHost, Value, fields, integer, list, record, required, text, vector};
use kinetics_continuum_multiphysics::{C_DIRS, LbmGrid3D, OPPOSITE, equilibrium_d3q19};

const MAX_LATTICE_SPEED: f64 = 0.16;
const MAX_DENSITY: f64 = 10.0;

struct Source {
    cell: usize,
    density: f64,
    velocity: [f64; 3],
}

impl AssetHost<'_> {
    pub(crate) fn solve_cfd_flip_smoke(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "dimensions",
                "obstacle_sdf",
                "initial_density_lu",
                "initial_velocity_lu",
                "kinematic_viscosity_lu",
                "emitters",
                "steps",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "LBM_D3Q19" {
                return Err("Supported solver for this route is LBM_D3Q19".into());
            }
        }
        let [nx, ny, nz] = read_dimensions(args, 128, 131_072)?;
        let len = nx * ny * nz;
        let shape = [nz as u32, ny as u32, nx as u32];
        let viscosity = number_arg(args, "kinematic_viscosity_lu", Some(0.08), 0.004..=0.4)?;
        let steps = int_arg(args, "steps", None, 1..=4096)?;
        let scratch = len * 424;
        self.charge(len * 64, scratch)?;
        let mut grid = LbmGrid3D::new(nx, ny, nz, viscosity);
        if let Some(obstacle) = args.get("obstacle_sdf") {
            let sdf = input_plane(self, obstacle, &shape, -1e9..=1e9, "obstacle_sdf")?;
            for (solid, distance) in grid.solid.iter_mut().zip(sdf) {
                *solid = distance <= 0.0;
            }
        }
        let fluid_cells = grid.solid.iter().filter(|&&solid| !solid).count();
        if fluid_cells == 0 {
            return Err("Obstacle field leaves no fluid cells".into());
        }
        let velocity = args
            .get("initial_velocity_lu")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        check_speed(velocity, "initial velocity")?;
        let density = args
            .get("initial_density_lu")
            .map(|value| input_plane(self, value, &shape, 0.8..=1.2, "initial_density_lu"))
            .transpose()?
            .unwrap_or_else(|| vec![1.0; len]);
        let mut initial_mass = 0.0;
        for (i, rho) in density.into_iter().enumerate() {
            if !grid.solid[i] {
                initial_mass += rho;
                grid.f[i * 19..(i + 1) * 19].copy_from_slice(&equilibrium(rho, velocity));
            }
        }
        let sources = args
            .get("emitters")
            .map(|value| parse_sources(value, &grid))
            .transpose()?
            .unwrap_or_default();
        let mut injected_mass = 0.0;
        let mut peak_speed: f64 = 0.0;
        for step in 0..steps {
            self.charge(len * 220 + sources.len() * 19, scratch)?;
            for source in &sources {
                let feq = equilibrium(source.density, source.velocity);
                for (population, add) in grid.f[source.cell * 19..(source.cell + 1) * 19]
                    .iter_mut()
                    .zip(feq)
                {
                    *population += add;
                }
                injected_mass += source.density;
            }
            peak_speed = peak_speed.max(collide(&mut grid, step)?);
            grid.stream();
        }
        let mut rho = vec![0.0; len];
        let mut pressure = vec![0.0; len];
        let mut velocity = [vec![0.0; len], vec![0.0; len], vec![0.0; len]];
        let mut mass = 0.0;
        for i in 0..len {
            if grid.solid[i] {
                continue;
            }
            let (density, u) = moments(&grid.f[i * 19..(i + 1) * 19])
                .map_err(|error| format!("Final LBM cell {i}: {error}"))?;
            peak_speed = peak_speed.max(check_speed(u, "final velocity")?);
            rho[i] = density;
            pressure[i] = (density - 1.0) / 3.0;
            for (component, value) in velocity.iter_mut().zip(u) {
                component[i] = value;
            }
            mass += density;
        }
        let residual = mass - initial_mass - injected_mass;
        let mass_tolerance = (initial_mass + injected_mass) * 1e-9;
        if !residual.is_finite() || residual.abs() > mass_tolerance {
            return Err(format!(
                "D3Q19 mass residual {residual:.12e} exceeds {mass_tolerance:.12e}"
            ));
        }
        let field = output_plane(
            self,
            shape.to_vec(),
            &[&velocity[0], &velocity[1], &velocity[2], &pressure, &rho],
        )?;
        Ok(Value::Record(Args::from([
            ("field".into(), field),
            (
                "plane_names".into(),
                Value::List(
                    ["vx_lu", "vy_lu", "vz_lu", "gauge_pressure_lu", "density_lu"]
                        .map(|name| Value::String(name.into()))
                        .into(),
                ),
            ),
            ("solver".into(), Value::String("LBM_D3Q19_TRT".into())),
            (
                "boundary".into(),
                Value::String("periodic_with_bounce_back".into()),
            ),
            ("fluid_cells".into(), Value::U64(fluid_cells as u64)),
            ("initial_mass_lu".into(), Value::F64(initial_mass)),
            ("injected_mass_lu".into(), Value::F64(injected_mass)),
            ("final_mass_lu".into(), Value::F64(mass)),
            ("mass_residual_lu".into(), Value::F64(residual)),
            ("peak_mach".into(), Value::F64(peak_speed * 3.0_f64.sqrt())),
        ])))
    }
}

fn parse_sources(value: &Value, grid: &LbmGrid3D) -> Result<Vec<Source>, String> {
    let list = list(value)?;
    if list.len() > 128 {
        return Err("At most 128 density emitters are supported".into());
    }
    list.iter()
        .map(|value| {
            let args = record(value)?;
            fields(args, &["cell", "density_per_step_lu", "velocity_lu"])?;
            let values = crate::list(required(args, "cell")?)?;
            let [x, y, z] = values else {
                return Err("Emitter cell requires x, y, z indices".into());
            };
            let x = integer(x, "x")?;
            let y = integer(y, "y")?;
            let z = integer(z, "z")?;
            if x >= grid.nx || y >= grid.ny || z >= grid.nz {
                return Err("Emitter cell lies outside the grid".into());
            }
            let cell = grid.idx(x, y, z);
            if grid.solid[cell] {
                return Err("Emitter cell intersects a solid obstacle".into());
            }
            let velocity = args
                .get("velocity_lu")
                .map(vector::<3>)
                .transpose()?
                .unwrap_or([0.0; 3]);
            check_speed(velocity, "emitter velocity")?;
            Ok(Source {
                cell,
                density: number_arg(args, "density_per_step_lu", None, 0.0..=0.1)?,
                velocity,
            })
        })
        .collect()
}

fn equilibrium(density: f64, velocity: [f64; 3]) -> [f64; 19] {
    // The native polynomial with unit density has the correct normalized
    // moments. Scaling the whole polynomial conserves rho * velocity.
    equilibrium_d3q19(1.0, velocity).map(|population| population * density)
}

fn moments(populations: &[f64]) -> Result<(f64, [f64; 3]), String> {
    let mut rho = 0.0;
    let mut momentum = [0.0; 3];
    for (&population, direction) in populations.iter().zip(C_DIRS) {
        rho += population;
        for (entry, component) in momentum.iter_mut().zip(direction) {
            *entry += population * f64::from(component);
        }
    }
    if !(0.02..=MAX_DENSITY).contains(&rho) {
        return Err(format!(
            "density {rho:.12e} is outside the admitted interval"
        ));
    }
    Ok((rho, momentum.map(|entry| entry / rho)))
}

fn check_speed(velocity: [f64; 3], location: &str) -> Result<f64, String> {
    let speed = dot(velocity, velocity).sqrt();
    if !(0.0..=MAX_LATTICE_SPEED).contains(&speed) {
        return Err(format!(
            "D3Q19 {location} speed {speed:.12e} exceeds low-Mach bound {MAX_LATTICE_SPEED}"
        ));
    }
    Ok(speed)
}

fn collide(grid: &mut LbmGrid3D, step: usize) -> Result<f64, String> {
    let bulk = 3.0 * grid.kinematic_viscosity;
    let omega_even = 1.0 / (0.5 + bulk);
    let omega_odd = 1.0 / (0.5 + 0.25 / bulk);
    let mut peak_speed: f64 = 0.0;
    for (i, &solid) in grid.solid.iter().enumerate() {
        if solid {
            continue;
        }
        let old = &grid.f[i * 19..(i + 1) * 19];
        let (rho, velocity) =
            moments(old).map_err(|error| format!("LBM step {step}, cell {i}: {error}"))?;
        peak_speed = peak_speed.max(check_speed(velocity, "collision")?);
        let feq = equilibrium(rho, velocity);
        for (direction, opposite) in OPPOSITE.into_iter().enumerate() {
            let delta = old[direction] - feq[direction];
            let reverse_delta = old[opposite] - feq[opposite];
            let result = old[direction]
                - 0.5
                    * (omega_even * (delta + reverse_delta) + omega_odd * (delta - reverse_delta));
            if !result.is_finite() || result < 0.0 {
                return Err(format!(
                    "D3Q19 step {step} cell {i} has inadmissible population {result:.12e}"
                ));
            }
            grid.f_coll[i * 19 + direction] = result;
        }
    }
    Ok(peak_speed)
}

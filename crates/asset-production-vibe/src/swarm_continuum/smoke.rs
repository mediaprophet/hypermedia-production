//! Low-Mach buoyant reactive smoke on a staggered periodic grid.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{input_plane, int_arg, number_arg, output_plane, read_dimensions};
use crate::{Args, AssetHost, Value, fields, text};

struct Grid {
    dimensions: [usize; 3],
    spacing: f64,
}

struct Smoke {
    velocity: [Vec<f64>; 3],
    temperature: Vec<f64>,
    fuel: Vec<f64>,
    soot: Vec<f64>,
}

struct Model {
    ambient: f64,
    ignition: f64,
    buoyancy: f64,
    rate: f64,
    heat_release: f64,
    soot_yield: f64,
    cooling: f64,
    confinement: f64,
}

impl AssetHost<'_> {
    pub(crate) fn solve_pyro_buoyancy_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "dimensions",
                "cell_size_m",
                "steps",
                "dt_s",
                "ambient_temp_k",
                "initial_temperature_k",
                "ignition_temperature_k",
                "initial_fuel_fraction",
                "initial_soot_fraction",
                "fuel_emitter_per_s",
                "buoyancy_m_s2",
                "reaction_rate_s_inv",
                "heat_release_k",
                "soot_yield",
                "cooling_rate_s_inv",
                "vorticity_confinement_m_s",
                "projection_iterations",
                "projection_tolerance_s_inv",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "periodic_boussinesq_mac" {
                return Err("Supported solver is periodic_boussinesq_mac".into());
            }
        }
        let dimensions = read_dimensions(args, 128, 131_072)?;
        let [nx, ny, nz] = dimensions;
        let shape = [nz as u32, ny as u32, nx as u32];
        let n = nx * ny * nz;
        let steps = int_arg(args, "steps", None, 1..=2048)?;
        let projection_iterations = int_arg(args, "projection_iterations", Some(256), 1..=2048)?;
        let dt = number_arg(args, "dt_s", None, 1e-7..=10.0)?;
        let projection_tolerance =
            number_arg(args, "projection_tolerance_s_inv", Some(1e-6), 1e-10..=0.01)?;
        let ambient = number_arg(args, "ambient_temp_k", Some(293.15), 180.0..=600.0)?;
        let model = Model {
            ambient,
            ignition: number_arg(
                args,
                "ignition_temperature_k",
                Some(650.0),
                ambient..=3000.0,
            )?,
            buoyancy: number_arg(args, "buoyancy_m_s2", Some(9.80665), 0.0..=100.0)?,
            rate: number_arg(args, "reaction_rate_s_inv", Some(2.0), 0.0..=1000.0)?,
            heat_release: number_arg(args, "heat_release_k", Some(1200.0), 0.0..=4000.0)?,
            soot_yield: number_arg(args, "soot_yield", Some(0.15), 0.0..=1.0)?,
            cooling: number_arg(args, "cooling_rate_s_inv", Some(0.15), 0.0..=50.0)?,
            confinement: number_arg(args, "vorticity_confinement_m_s", Some(0.0), 0.0..=20.0)?,
        };
        let grid = Grid {
            dimensions,
            spacing: number_arg(args, "cell_size_m", None, 1e-4..=1000.0)?,
        };
        let scratch = n * 256;
        self.charge(n * 64, scratch)?;
        let mut smoke = Smoke {
            velocity: [vec![0.0; n], vec![0.0; n], vec![0.0; n]],
            temperature: optional(
                self,
                args,
                "initial_temperature_k",
                &shape,
                ambient,
                180.0..=5000.0,
            )?,
            fuel: optional(self, args, "initial_fuel_fraction", &shape, 0.0, 0.0..=1.0)?,
            soot: optional(self, args, "initial_soot_fraction", &shape, 0.0, 0.0..=1.0)?,
        };
        let emitter = optional(self, args, "fuel_emitter_per_s", &shape, 0.0, 0.0..=10.0)?;
        let mut emitted = 0.0;
        let mut burned = 0.0;
        let mut projection_work = 0u64;
        let mut worst_divergence: f64 = 0.0;
        let mut substeps = 0u64;
        for _ in 0..steps {
            let mut time_left = dt;
            for attempt in 0..256 {
                self.charge(n * 450, scratch)?;
                let max_speed = smoke
                    .velocity
                    .iter()
                    .map(|plane| plane.iter().map(|value| value.abs()).fold(0.0, f64::max))
                    .sum::<f64>();
                let max_temperature = smoke.temperature.iter().copied().fold(ambient, f64::max);
                let acceleration = model.buoyancy * (max_temperature / ambient - 1.0).max(0.0);
                let cfl_dt = 0.5 * grid.spacing / max_speed.max(1e-12);
                let force_dt = (0.15 * grid.spacing / acceleration.max(1e-12)).sqrt();
                let reaction_dt = 0.25 / model.rate.max(1e-12);
                let confinement_dt = 0.25 * grid.spacing / model.confinement.max(1e-12);
                let substep = time_left
                    .min(cfl_dt)
                    .min(force_dt)
                    .min(reaction_dt)
                    .min(confinement_dt);
                if !substep.is_finite()
                    || substep <= 0.0
                    || (substep < time_left && time_left - substep == time_left)
                {
                    return Err("Buoyant smoke substep could not advance time".into());
                }
                smoke.advect(&grid, substep);
                for i in 0..n {
                    let injected =
                        (substep * emitter[i] * (1.0 - smoke.fuel[i])).min(1.0 - smoke.fuel[i]);
                    smoke.fuel[i] += injected;
                    emitted += injected;
                    if smoke.temperature[i] >= model.ignition {
                        let consumed = smoke.fuel[i] * -(-model.rate * substep).exp_m1();
                        smoke.fuel[i] -= consumed;
                        smoke.soot[i] += model.soot_yield * consumed;
                        smoke.temperature[i] += model.heat_release * consumed;
                        burned += consumed;
                    }
                    smoke.temperature[i] = ambient
                        + (smoke.temperature[i] - ambient) * (-model.cooling * substep).exp();
                    if !(180.0..=5000.0).contains(&smoke.temperature[i])
                        || !(0.0..=1.0).contains(&smoke.fuel[i])
                        || !(0.0..=100.0).contains(&smoke.soot[i])
                    {
                        return Err(format!(
                            "Buoyant smoke state at cell {i} exceeds admitted limits"
                        ));
                    }
                }
                smoke.apply_buoyancy(&grid, &model, substep);
                let (iterations, divergence) = grid.project(
                    self,
                    &mut smoke.velocity,
                    projection_iterations,
                    projection_tolerance,
                    scratch,
                )?;
                worst_divergence = worst_divergence.max(divergence);
                projection_work += iterations as u64;
                substeps += 1;
                if substep == time_left {
                    break;
                }
                time_left -= substep;
                if attempt == 255 {
                    return Err(
                        "Buoyant smoke requires more than 256 substeps per authored step".into(),
                    );
                }
            }
        }
        let center = smoke.cell_velocities(&grid);
        let velocity: [Vec<f64>; 3] =
            std::array::from_fn(|k| center.iter().map(|u| u[k]).collect());
        let exitance: Vec<_> = smoke
            .temperature
            .iter()
            .zip(&smoke.soot)
            .map(|(&temperature, &soot)| {
                let emittance = 1.0 - (-soot).exp();
                5.670374419e-8 * emittance * (temperature.powi(4) - ambient.powi(4)).max(0.0)
            })
            .collect();
        let field = output_plane(
            self,
            shape.to_vec(),
            &[
                &smoke.temperature,
                &smoke.soot,
                &smoke.fuel,
                &velocity[0],
                &velocity[1],
                &velocity[2],
                &exitance,
            ],
        )?;
        Ok(Value::Record(Args::from([
            ("field".into(), field),
            (
                "plane_names".into(),
                Value::List(
                    [
                        "temperature_k",
                        "relative_soot_density",
                        "fuel_fraction",
                        "vx_m_s",
                        "vy_m_s",
                        "vz_m_s",
                        "greybody_exitance_w_m2_proxy",
                    ]
                    .map(|name| Value::String(name.into()))
                    .into(),
                ),
            ),
            (
                "solver".into(),
                Value::String("periodic_boussinesq_mac".into()),
            ),
            ("substeps".into(), Value::U64(substeps)),
            ("projection_iterations".into(), Value::U64(projection_work)),
            (
                "max_projected_divergence_s_inv".into(),
                Value::F64(worst_divergence),
            ),
            (
                "fuel_source_integral_m3_equivalent".into(),
                Value::F64(emitted * grid.spacing.powi(3)),
            ),
            (
                "fuel_reaction_integral_m3_equivalent".into(),
                Value::F64(burned * grid.spacing.powi(3)),
            ),
        ])))
    }
}

fn optional(
    host: &AssetHost<'_>,
    args: &Args,
    key: &str,
    shape: &[u32; 3],
    default: f64,
    bounds: std::ops::RangeInclusive<f64>,
) -> Result<Vec<f64>, String> {
    args.get(key)
        .map(|value| input_plane(host, value, shape, bounds, key))
        .transpose()
        .map(|output| {
            output.unwrap_or_else(|| vec![default; shape.iter().map(|d| *d as usize).product()])
        })
}

impl Grid {
    fn index(&self, xyz: [usize; 3]) -> usize {
        let [x, y, z] = xyz;
        let [nx, ny, _] = self.dimensions;
        (z * ny + y) * nx + x
    }

    fn coord(&self, i: usize) -> [usize; 3] {
        let [nx, ny, _] = self.dimensions;
        [i % nx, (i / nx) % ny, i / (nx * ny)]
    }

    fn neighbor(&self, i: usize, axis: usize, step: isize) -> usize {
        let mut xyz = self.coord(i);
        xyz[axis] = (xyz[axis] as isize + step).rem_euclid(self.dimensions[axis] as isize) as usize;
        self.index(xyz)
    }

    fn sample(&self, values: &[f64], query: [f64; 3]) -> f64 {
        let lower = query.map(f64::floor);
        let frac: [f64; 3] = std::array::from_fn(|axis| query[axis] - lower[axis]);
        let mut sample = 0.0;
        let mut minimum = f64::INFINITY;
        let mut maximum = f64::NEG_INFINITY;
        for corner in 0..8 {
            let mut weight = 1.0;
            let mut point = [0; 3];
            for axis in 0..3 {
                let next = (corner >> axis) & 1;
                weight *= if next == 0 {
                    1.0 - frac[axis]
                } else {
                    frac[axis]
                };
                point[axis] = (lower[axis] as isize + next as isize)
                    .rem_euclid(self.dimensions[axis] as isize)
                    as usize;
            }
            let value = values[self.index(point)];
            minimum = minimum.min(value);
            maximum = maximum.max(value);
            sample += weight * value;
        }
        // Keep interpolation in its donor range even at representable endpoints.
        sample.clamp(minimum, maximum)
    }

    fn divergence(&self, field: &[Vec<f64>; 3], i: usize) -> f64 {
        field
            .iter()
            .enumerate()
            .map(|(axis, lane)| lane[i] - lane[self.neighbor(i, axis, -1)])
            .sum::<f64>()
            / self.spacing
    }

    fn project(
        &self,
        host: &mut AssetHost<'_>,
        field: &mut [Vec<f64>; 3],
        limit: usize,
        tolerance: f64,
        scratch: usize,
    ) -> Result<(usize, f64), String> {
        let n = field[0].len();
        let mut rhs: Vec<_> = (0..n)
            .map(|i| -self.divergence(field, i) * self.spacing.powi(2))
            .collect();
        let mean = rhs.iter().sum::<f64>() / n as f64;
        rhs.iter_mut().for_each(|x| *x -= mean);
        let mut directions = rhs.clone();
        let mut ap = vec![0.0; n];
        let mut potential = vec![0.0; n];
        let mut rr = rhs.iter().map(|x| x * x).sum::<f64>();
        let threshold = tolerance * self.spacing.powi(2) * 0.5;
        let mut kept = 0;
        while rhs.iter().any(|r| r.abs() > threshold) && kept < limit {
            host.charge(n * 90, scratch)?;
            for i in 0..n {
                let mut v = 6.0 * directions[i];
                for axis in 0..3 {
                    v -= directions[self.neighbor(i, axis, -1)]
                        + directions[self.neighbor(i, axis, 1)];
                }
                ap[i] = v;
            }
            let curvature = directions.iter().zip(&ap).map(|(a, b)| a * b).sum::<f64>();
            if curvature <= 0.0 || !curvature.is_finite() {
                return Err("MAC pressure projection lost positive curvature".into());
            }
            let step = rr / curvature;
            for i in 0..n {
                potential[i] += step * directions[i];
                rhs[i] -= step * ap[i];
            }
            let next = rhs.iter().map(|x| x * x).sum::<f64>();
            let beta = next / rr;
            rr = next;
            for i in 0..n {
                directions[i] = rhs[i] + beta * directions[i];
            }
            kept += 1;
        }
        for (axis, plane) in field.iter_mut().enumerate() {
            for (i, value) in plane.iter_mut().enumerate() {
                *value -= (potential[self.neighbor(i, axis, 1)] - potential[i]) / self.spacing;
                if !value.is_finite() || value.abs() > 100_000.0 {
                    return Err(format!(
                        "MAC correction produced inadmissible velocity {value:.12e}"
                    ));
                }
            }
        }
        let maximum = (0..n)
            .map(|i| self.divergence(field, i).abs())
            .fold(0.0, f64::max);
        if !maximum.is_finite() || maximum > tolerance {
            return Err(format!(
                "MAC projection residual {maximum:.12e} s^-1 exceeds {tolerance:.12e} after {kept} iterations"
            ));
        }
        Ok((kept, maximum))
    }
}

impl Smoke {
    fn advect(&mut self, grid: &Grid, dt: f64) {
        let flow = |query: [f64; 3]| -> [f64; 3] {
            std::array::from_fn(|axis| {
                let mut query = query;
                query[axis] -= 0.5;
                grid.sample(&self.velocity[axis], query)
            })
        };
        let backtrace = |point: [f64; 3]| {
            let first = flow(point);
            let midpoint = std::array::from_fn(|i| point[i] - 0.5 * dt * first[i] / grid.spacing);
            let middle = flow(midpoint);
            std::array::from_fn(|i| point[i] - dt * middle[i] / grid.spacing)
        };
        let n = self.temperature.len();
        let mut temperature = Vec::with_capacity(n);
        let mut fuel = Vec::with_capacity(n);
        let mut soot = Vec::with_capacity(n);
        let mut velocity = std::array::from_fn(|_| Vec::with_capacity(n));
        for i in 0..n {
            let center = grid.coord(i).map(|v| v as f64);
            let from = backtrace(center);
            temperature.push(grid.sample(&self.temperature, from));
            fuel.push(grid.sample(&self.fuel, from));
            soot.push(grid.sample(&self.soot, from));
            for (axis, plane) in velocity.iter_mut().enumerate() {
                let mut face = center;
                face[axis] += 0.5;
                let mut from = backtrace(face);
                from[axis] -= 0.5;
                plane.push(grid.sample(&self.velocity[axis], from));
            }
        }
        *self = Self {
            velocity,
            temperature,
            fuel,
            soot,
        };
    }

    fn cell_velocities(&self, grid: &Grid) -> Vec<[f64; 3]> {
        (0..self.temperature.len())
            .map(|i| {
                std::array::from_fn(|axis| {
                    0.5 * (self.velocity[axis][i] + self.velocity[axis][grid.neighbor(i, axis, -1)])
                })
            })
            .collect()
    }

    fn apply_buoyancy(&mut self, grid: &Grid, model: &Model, dt: f64) {
        let n = self.temperature.len();
        let mut force: Vec<_> = self
            .temperature
            .iter()
            .map(|temperature| {
                [
                    0.0,
                    model.buoyancy * (temperature / model.ambient - 1.0),
                    0.0,
                ]
            })
            .collect();
        if model.confinement > 0.0 {
            let center = self.cell_velocities(grid);
            let curl: Vec<[f64; 3]> = (0..n)
                .map(|i| {
                    let derivative = |axis, component| {
                        let next = center[grid.neighbor(i, axis, 1)][component];
                        let previous = center[grid.neighbor(i, axis, -1)][component];
                        (next - previous) / (2.0 * grid.spacing)
                    };
                    [
                        derivative(1, 2) - derivative(2, 1),
                        derivative(2, 0) - derivative(0, 2),
                        derivative(0, 1) - derivative(1, 0),
                    ]
                })
                .collect();
            let magnitude: Vec<_> = curl.iter().map(|&c| super::dot(c, c).sqrt()).collect();
            for (i, f) in force.iter_mut().enumerate() {
                let gradient = std::array::from_fn(|axis| {
                    magnitude[grid.neighbor(i, axis, 1)] - magnitude[grid.neighbor(i, axis, -1)]
                });
                let length = super::dot(gradient, gradient).sqrt();
                if length > 1e-12 {
                    let [x, y, z] = gradient.map(|g| model.confinement * g / length);
                    let [a, b, c] = curl[i];
                    f[0] += y * c - z * b;
                    f[1] += z * a - x * c;
                    f[2] += x * b - y * a;
                }
            }
        }
        for (axis, plane) in self.velocity.iter_mut().enumerate() {
            for (i, v) in plane.iter_mut().enumerate() {
                *v += 0.5 * dt * (force[i][axis] + force[grid.neighbor(i, axis, 1)][axis]);
            }
        }
    }
}

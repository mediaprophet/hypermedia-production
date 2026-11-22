//! Dry Drucker-Prager material points on a trilinear 3D grid.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{add3, dot, int_arg, number_arg, output_plane, read_dimensions, sub3};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::ParticleSet, fields, list, required, text,
    vector,
};
use std::sync::Arc;

type V3 = [f64; 3];
type M3 = [[f64; 3]; 3];

struct Material {
    mass: f64,
    reference_volume: f64,
    shear: f64,
    lame: f64,
    friction_slope: f64,
    cohesion_intercept: f64,
    wave_speed: f64,
}

struct Grid {
    axes: [usize; 3],
    origin: V3,
    dx: f64,
}

struct Particle {
    position: V3,
    velocity: V3,
    stress: M3,
    volume_ratio: f64,
}

#[derive(Clone, Copy, Default)]
struct Node {
    mass: f64,
    momentum: V3,
    internal_force: V3,
    before: V3,
    after: V3,
}

#[derive(Clone, Copy)]
struct Weight {
    index: usize,
    value: f64,
    gradient: V3,
}

impl AssetHost<'_> {
    pub(crate) fn solve_granular_mpm(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "dimensions",
                "grid_origin_m",
                "cell_size_m",
                "initial_positions_m",
                "initial_velocity_m_s",
                "particle_mass_kg",
                "density_kg_m3",
                "young_modulus_pa",
                "poisson_ratio",
                "friction_angle_deg",
                "cohesion_pa",
                "boundary_friction",
                "flip_fraction",
                "gravity_m_s2",
                "dt_s",
                "steps",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "dry_drucker_prager_mpm" {
                return Err("Supported solver is dry_drucker_prager_mpm".into());
            }
        }
        let axes = read_dimensions(args, 128, 131_072)?;
        let grid = Grid {
            axes,
            origin: vector::<3>(required(args, "grid_origin_m")?)?,
            dx: number_arg(args, "cell_size_m", None, 1e-5..=100.0)?,
        };
        if grid.origin.iter().any(|x| x.abs() > 1e5) {
            return Err("MPM origin exceeds the admitted coordinate range".into());
        }
        if grid.origin.iter().enumerate().any(|(k, x)| {
            let largest = x.abs() + grid.dx * (axes[k] - 1) as f64;
            grid.dx <= 4.0 * f64::from(f32::EPSILON) * largest
        }) {
            return Err(
                "MPM cell spacing cannot be represented precisely in the authored frame".into(),
            );
        }
        let steps = int_arg(args, "steps", None, 1..=2048)?;
        let dt = number_arg(args, "dt_s", None, 1e-8..=1.0)?;
        let density = number_arg(args, "density_kg_m3", Some(1600.0), 10.0..=1e5)?;
        let mass = number_arg(args, "particle_mass_kg", None, 1e-15..=1e9)?;
        let stiffness = number_arg(args, "young_modulus_pa", None, 10.0..=1e10)?;
        let poisson = number_arg(args, "poisson_ratio", Some(0.25), 0.0..=0.45)?;
        let angle = number_arg(args, "friction_angle_deg", Some(35.0), 0.0..=60.0)?.to_radians();
        let cohesion = number_arg(args, "cohesion_pa", Some(0.0), 0.0..=1e8)?;
        let shear = stiffness / (2.0 * (1.0 + poisson));
        let lame = stiffness * poisson / ((1.0 + poisson) * (1.0 - 2.0 * poisson));
        let friction = number_arg(args, "boundary_friction", Some(0.4), 0.0..=2.0)?;
        let flip = number_arg(args, "flip_fraction", Some(0.75), 0.0..=0.95)?;
        let gravity = args
            .get("gravity_m_s2")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0, 0.0, -9.80665]);
        if gravity.iter().any(|g| g.abs() > 1000.0) {
            return Err("MPM gravity exceeds the admitted component range".into());
        }
        let material = Material {
            mass,
            reference_volume: mass / density,
            shear,
            lame,
            friction_slope: 6.0 * angle.sin() / (3.0 - angle.sin()),
            cohesion_intercept: 6.0 * cohesion * angle.cos() / (3.0 - angle.sin()),
            // Volume may expand to three reference volumes. Use this density
            // floor in the elastic wave speed bound.
            wave_speed: ((lame + 2.0 * shear) * 3.0 / density).sqrt(),
        };
        let volume = material.reference_volume;
        if !(1e-9..=1.0).contains(&(volume / grid.dx.powi(3))) {
            return Err("MPM reference particle volume must be 1e-9..=1 grid-cell volumes".into());
        }
        let initial = list(required(args, "initial_positions_m")?)?;
        if !(2..=4096).contains(&initial.len()) {
            return Err("MPM needs 2..=4096 material points".into());
        }
        let initial_velocity = args
            .get("initial_velocity_m_s")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if dot(initial_velocity, initial_velocity) > 100_000_000.0 {
            return Err("Initial MPM speed exceeds 10,000 m/s".into());
        }
        let n = axes.iter().product::<usize>();
        let scratch = n * 128 + initial.len() * 160;
        self.charge(n * 16 + initial.len() * 32, scratch)?;
        let mut particles = Vec::with_capacity(initial.len());
        for value in initial {
            let position = vector::<3>(value)?;
            if !grid.interior(position) {
                return Err("MPM initial position touches or exceeds the grid boundary".into());
            }
            particles.push(Particle {
                position,
                velocity: initial_velocity,
                stress: [[0.0; 3]; 3],
                volume_ratio: 1.0,
            });
        }
        let mut nodes = vec![Node::default(); n];
        let mut yielded_updates = 0u64;
        let mut advanced = 0u64;
        let mut peak_stress_pa: f64 = 0.0;
        for _ in 0..steps {
            let mut time_left = dt;
            for attempt in 0..256 {
                self.charge(n * 135 + particles.len() * 1120, scratch)?;
                let speed = particles
                    .iter()
                    .map(|p| dot(p.velocity, p.velocity).sqrt())
                    .fold(0.0, f64::max);
                let substep = time_left.min(0.25 * grid.dx / (material.wave_speed + speed));
                if !substep.is_finite()
                    || substep <= 0.0
                    || (substep < time_left && time_left - substep == time_left)
                {
                    return Err("Granular MPM Courant step could not advance time".into());
                }
                nodes.fill(Node::default());
                for p in &particles {
                    for lane in grid.stencil(p.position)? {
                        let node = &mut nodes[lane.index];
                        node.mass += lane.value * material.mass;
                        node.momentum = add3(
                            node.momentum,
                            p.velocity.map(|v| lane.value * material.mass * v),
                        );
                        let volume = material.reference_volume * p.volume_ratio;
                        let force = p.stress.map(|row| -volume * dot(row, lane.gradient));
                        node.internal_force = add3(node.internal_force, force);
                    }
                }
                for (index, node) in nodes.iter_mut().enumerate() {
                    if node.mass <= material.mass * 1e-14 {
                        continue;
                    }
                    node.before = node.momentum.map(|u| u / node.mass);
                    node.after = add3(
                        node.before,
                        std::array::from_fn(|i| {
                            substep * (node.internal_force[i] / node.mass + gravity[i])
                        }),
                    );
                    grid.wall_node(index, friction, &mut node.after);
                }
                for particle in &mut particles {
                    let mut pic = [0.0; 3];
                    let mut delta = [0.0; 3];
                    let mut velocity_gradient = [[0.0; 3]; 3];
                    for lane in grid.stencil(particle.position)? {
                        let node = nodes[lane.index];
                        pic = add3(pic, node.after.map(|v| v * lane.value));
                        delta = add3(delta, sub3(node.after, node.before).map(|v| v * lane.value));
                        for (k, row) in velocity_gradient.iter_mut().enumerate() {
                            for (d, gradient) in row.iter_mut().enumerate() {
                                *gradient += node.after[k] * lane.gradient[d];
                            }
                        }
                    }
                    if material.update(particle, &velocity_gradient, substep)? {
                        yielded_updates += 1;
                    }
                    let velocity = add3(particle.velocity, delta);
                    particle.velocity =
                        std::array::from_fn(|i| (1.0 - flip) * pic[i] + flip * velocity[i]);
                    particle.position = add3(particle.position, pic.map(|u| substep * u));
                    grid.confine(&mut particle.position, &mut particle.velocity, friction)?;
                    for &stress in particle.stress.iter().flatten() {
                        peak_stress_pa = peak_stress_pa.max(stress.abs());
                    }
                }
                advanced += 1;
                if substep == time_left {
                    break;
                }
                time_left -= substep;
                if attempt == 255 {
                    return Err(
                        "Granular MPM exceeded 256 stable substeps per authored step".into(),
                    );
                }
            }
        }
        let [nx, ny, _] = axes;
        let mut heights = vec![0.0_f64; nx * ny];
        let mut footprint = vec![0.0; nx * ny];
        let mut pressure_sum = vec![0.0; n];
        let mut weight_sum = vec![0.0; n];
        let mut radii = Vec::with_capacity(particles.len());
        for particle in &particles {
            let r = (3.0 * volume * particle.volume_ratio / (4.0 * std::f64::consts::PI)).cbrt();
            radii.push(r as f32);
            let [x, y, _] = grid.relative(particle.position).map(|w| w.round() as usize);
            let column = y * nx + x;
            heights[column] = heights[column].max(particle.position[2] + r - grid.origin[2]);
            footprint[column] = 1.0;
            let p = -(0..3).map(|i| particle.stress[i][i]).sum::<f64>() / 3.0;
            for weight in grid.stencil(particle.position)? {
                pressure_sum[weight.index] += weight.value * p;
                weight_sum[weight.index] += weight.value;
            }
        }
        for (sum, weight) in pressure_sum.iter_mut().zip(weight_sum) {
            if weight > 0.0 {
                *sum /= weight;
            }
        }
        let heightfield = output_plane(self, vec![ny as u32, nx as u32], &[&heights, &footprint])?;
        let pressure = output_plane(
            self,
            vec![axes[2] as u32, ny as u32, nx as u32],
            &[&pressure_sum],
        )?;
        let count = particles.len();
        let output = self.admit_artifact(TypedArtifact::ParticleSet(Arc::new(ParticleSet {
            positions: particles
                .iter()
                .map(|p| p.position.map(|x| x as f32))
                .collect(),
            velocities: particles
                .iter()
                .map(|p| p.velocity.map(|x| x as f32))
                .collect(),
            mass_kg: vec![mass as f32; count],
            radii_m: radii,
        })))?;
        Ok(Value::Record(Args::from([
            ("heightfield".into(), heightfield),
            ("mean_stress_pressure_pa".into(), pressure),
            ("particles".into(), output),
            ("total_mass_kg".into(), Value::F64(count as f64 * mass)),
            (
                "heightfield_plane_names".into(),
                Value::List(
                    [
                        "particle_surface_height_above_origin_m",
                        "particle_occupancy",
                    ]
                    .map(|x| Value::String(x.into()))
                    .into(),
                ),
            ),
            (
                "grid_origin_m".into(),
                Value::List(grid.origin.map(Value::F64).into()),
            ),
            ("cell_size_m".into(), Value::F64(grid.dx)),
            ("substeps".into(), Value::U64(advanced)),
            (
                "plastic_particle_updates".into(),
                Value::U64(yielded_updates),
            ),
            ("peak_abs_stress_pa".into(), Value::F64(peak_stress_pa)),
        ])))
    }
}

impl Grid {
    fn relative(&self, position: V3) -> V3 {
        std::array::from_fn(|k| (position[k] - self.origin[k]) / self.dx)
    }

    fn interior(&self, position: V3) -> bool {
        self.relative(position)
            .into_iter()
            .zip(self.axes)
            .all(|(x, axis)| x > 0.0 && x < (axis - 1) as f64)
    }

    fn stencil(&self, position: V3) -> Result<[Weight; 8], String> {
        if !self.interior(position) {
            return Err("Material point left the trilinear grid interior".into());
        }
        let x = self.relative(position);
        let lower = x.map(|x| x.floor() as usize);
        let t: [f64; 3] = std::array::from_fn(|k| x[k] - lower[k] as f64);
        let [nx, ny, _] = self.axes;
        Ok(std::array::from_fn(|corner| {
            let bits: [usize; 3] = std::array::from_fn(|k| corner >> k & 1);
            let raw: [f64; 3] =
                std::array::from_fn(|k| if bits[k] == 0 { 1.0 - t[k] } else { t[k] });
            let [x, y, z] = std::array::from_fn(|k| lower[k] + bits[k]);
            let gradient = std::array::from_fn(|k| {
                let sign = if bits[k] == 0 { -1.0 } else { 1.0 };
                sign * raw[(k + 1) % 3] * raw[(k + 2) % 3] / self.dx
            });
            Weight {
                index: (z * ny + y) * nx + x,
                value: raw.into_iter().product(),
                gradient,
            }
        }))
    }

    fn wall_node(&self, index: usize, friction: f64, v: &mut V3) {
        let [nx, ny, _] = self.axes;
        let coordinate = [index % nx, (index / nx) % ny, index / (nx * ny)];
        for (k, cell) in coordinate.into_iter().enumerate() {
            if (cell == 0 && v[k] < 0.0) || (cell == self.axes[k] - 1 && v[k] > 0.0) {
                self.wall_velocity(k, friction, v);
            }
        }
    }

    fn wall_velocity(&self, k: usize, friction: f64, v: &mut V3) {
        let normal = v[k].abs();
        v[k] = 0.0;
        let speed = dot(*v, *v).sqrt();
        if speed > 0.0 {
            *v = v.map(|x| x * (1.0 - friction * normal / speed).max(0.0));
        }
    }

    fn confine(&self, position: &mut V3, velocity: &mut V3, friction: f64) -> Result<(), String> {
        if position
            .iter()
            .chain(velocity.iter())
            .any(|x| !x.is_finite())
            || dot(*velocity, *velocity) > 100_000_000.0
        {
            return Err("MPM particle left the finite position or velocity domain".into());
        }
        for (k, x) in position.iter_mut().enumerate() {
            let offset = self.dx * 1e-6;
            let lower = self.origin[k] + offset;
            let upper = self.origin[k] + self.dx * (self.axes[k] - 1) as f64 - offset;
            if *x < lower {
                *x = lower;
                if velocity[k] < 0.0 {
                    self.wall_velocity(k, friction, velocity);
                }
            } else if *x > upper {
                *x = upper;
                if velocity[k] > 0.0 {
                    self.wall_velocity(k, friction, velocity);
                }
            }
        }
        Ok(())
    }
}

impl Material {
    fn update(&self, p: &mut Particle, grad: &M3, dt: f64) -> Result<bool, String> {
        let divergence = (0..3).map(|i| grad[i][i]).sum::<f64>();
        let increment = dt * divergence;
        if !(-0.2..=0.2).contains(&increment) {
            return Err(format!(
                "MPM particle volume strain increment {increment:.12e} exceeds small elastic step limit"
            ));
        }
        p.volume_ratio *= increment.exp();
        if !(0.3..=3.0).contains(&p.volume_ratio) {
            return Err(format!(
                "MPM volume ratio {:.12e} is outside [0.3, 3.0]",
                p.volume_ratio
            ));
        }
        let spin = |i: usize, j: usize| 0.5 * (grad[i][j] - grad[j][i]);
        let mut trial = [[0.0; 3]; 3];
        for (i, row) in trial.iter_mut().enumerate() {
            for (j, entry) in row.iter_mut().enumerate() {
                let rotation = (0..3)
                    .map(|k| spin(i, k) * p.stress[k][j] - p.stress[i][k] * spin(k, j))
                    .sum::<f64>();
                let dilation = if i == j { self.lame * divergence } else { 0.0 };
                *entry = p.stress[i][j]
                    + dt * (self.shear * (grad[i][j] + grad[j][i]) + dilation + rotation);
            }
        }
        let mean = (0..3).map(|i| trial[i][i]).sum::<f64>() / 3.0;
        for (i, row) in trial.iter_mut().enumerate() {
            row[i] -= mean;
        }
        let q = (1.5 * trial.iter().flatten().map(|x| x * x).sum::<f64>()).sqrt();
        if !mean.is_finite() || !q.is_finite() || q > 1e12 || mean.abs() > 1e12 {
            return Err(format!(
                "MPM stress trial is inadmissible: q={q:.12e}, mean={mean:.12e} Pa"
            ));
        }
        let pressure = (-mean).max(0.0);
        let limit = self.cohesion_intercept + self.friction_slope * pressure;
        let yielded = q > limit;
        let scale = if yielded { limit / q } else { 1.0 };
        for (i, row) in p.stress.iter_mut().enumerate() {
            for (j, target) in row.iter_mut().enumerate() {
                *target = scale * trial[i][j] - if i == j { pressure } else { 0.0 };
            }
        }
        Ok(yielded || mean > 0.0)
    }
}

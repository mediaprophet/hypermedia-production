//! Weakly compressible 3D SPH in a bounded tank with a reconstructed free surface.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{add3, dot, int_arg, mesh_artifact, number_arg, output_plane, read_dimensions, sub3};
use crate::{
    Args, AssetHost, TypedArtifact, Value, artifact::ParticleSet, fields, list, required, text,
    vector,
};
use qualia_core_db::specialized_libs::computational_geometry::{
    Point3,
    indexed_tetra_surface::{TetraEdgeSlot, indexed_tetra_surface_capped_into},
};
use std::{f64::consts::PI, sync::Arc};

#[derive(Clone, Copy)]
struct Particle {
    position: [f64; 3],
    velocity: [f64; 3],
}

struct Tank {
    min: [f64; 3],
    max: [f64; 3],
    margin: f64,
    restitution: f64,
}

struct Fluid {
    support: f64,
    mass: f64,
    rest_density: f64,
    sound_speed: f64,
    viscosity: f64,
    tension: f64,
    gravity: [f64; 3],
}

impl AssetHost<'_> {
    pub(crate) fn solve_free_surface_3d(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "dimensions",
                "domain_min_m",
                "domain_max_m",
                "initial_positions_m",
                "initial_velocity_m_s",
                "rest_density_kg_m3",
                "particle_mass_kg",
                "support_radius_m",
                "surface_radius_m",
                "sound_speed_m_s",
                "viscosity_m2_s",
                "surface_tension_n_m",
                "gravity_m_s2",
                "restitution",
                "dt_s",
                "steps",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "SPH_SurfaceTension" {
                return Err("Supported solver for this route is SPH_SurfaceTension".into());
            }
        }
        let dimensions = read_dimensions(args, 128, 131_072)?;
        let steps = int_arg(args, "steps", None, 1..=2048)?;
        let dt = number_arg(args, "dt_s", None, 1e-8..=1.0)?;
        let support = number_arg(args, "support_radius_m", None, 1e-5..=100.0)?;
        let surface_radius = number_arg(
            args,
            "surface_radius_m",
            Some(0.5 * support),
            0.1 * support..=support,
        )?;
        let min = vector::<3>(required(args, "domain_min_m")?)?;
        let max = vector::<3>(required(args, "domain_max_m")?)?;
        if min.iter().chain(&max).any(|x| x.abs() > 1e5)
            || (0..3).any(|k| max[k] - min[k] <= 2.0 * surface_radius)
        {
            return Err("SPH tank bounds cannot contain the authored particles".into());
        }
        let tank = Tank {
            min,
            max,
            margin: surface_radius,
            restitution: number_arg(args, "restitution", Some(0.0), 0.0..=0.95)?,
        };
        let gravity = args
            .get("gravity_m_s2")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0, -9.80665, 0.0]);
        if gravity.iter().any(|g| g.abs() > 1000.0) {
            return Err("SPH gravity exceeds the admitted component range".into());
        }
        let fluid = Fluid {
            support,
            mass: number_arg(args, "particle_mass_kg", None, 1e-15..=1e9)?,
            rest_density: number_arg(args, "rest_density_kg_m3", Some(1000.0), 10.0..=1e5)?,
            sound_speed: number_arg(args, "sound_speed_m_s", Some(25.0), 0.1..=1000.0)?,
            viscosity: number_arg(args, "viscosity_m2_s", Some(1e-5), 0.0..=10.0)?,
            tension: number_arg(args, "surface_tension_n_m", Some(0.072), 0.0..=100.0)?,
            gravity,
        };
        let points = list(required(args, "initial_positions_m")?)?;
        if !(2..=1024).contains(&points.len()) {
            return Err("SPH requires 2..=1024 authored particle centers".into());
        }
        let velocity = args
            .get("initial_velocity_m_s")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        check_flow_speed(velocity, fluid.sound_speed)?;
        let mut particles = Vec::with_capacity(points.len());
        for point in points {
            let position = vector::<3>(point)?;
            if (0..3).any(|k| {
                !(min[k] + surface_radius..=max[k] - surface_radius).contains(&position[k])
            }) {
                return Err("Initial particle is outside the tank inset".into());
            }
            if particles.iter().any(|prior: &Particle| {
                let d = sub3(position, prior.position);
                dot(d, d) <= (1e-6 * support).powi(2)
            }) {
                return Err("Initial SPH particle centers overlap numerically".into());
            }
            particles.push(Particle { position, velocity });
        }
        let len = particles.len();
        let mut rho = vec![0.0; len];
        let mut pressure = vec![0.0; len];
        let mut acceleration = vec![[0.0; 3]; len];
        let mut substeps = 0u64;
        let mut peak_density_ratio: f64 = 0.0;
        let mut max_speed: f64 = 0.0;
        for _ in 0..steps {
            let mut remaining = dt;
            for attempt in 0..256 {
                self.charge(len * len * 100, len * 128)?;
                fluid.forces(&particles, &mut rho, &mut pressure, &mut acceleration)?;
                let density = rho.iter().copied().fold(0.0, f64::max) / fluid.rest_density;
                peak_density_ratio = peak_density_ratio.max(density);
                let max_acceleration = acceleration
                    .iter()
                    .map(|&a| dot(a, a).sqrt())
                    .fold(0.0, f64::max);
                if !max_acceleration.is_finite() {
                    return Err("SPH acceleration is nonfinite".into());
                }
                let mut instantaneous: f64 = 0.0;
                for particle in &particles {
                    instantaneous =
                        instantaneous.max(check_flow_speed(particle.velocity, fluid.sound_speed)?);
                }
                max_speed = max_speed.max(instantaneous);
                let step = remaining.min(fluid.bound(instantaneous, max_acceleration));
                if !step.is_finite()
                    || step <= 0.0
                    || (step < remaining && remaining - step == remaining)
                {
                    return Err("SPH stable substep could not advance time".into());
                }
                for (particle, acceleration) in particles.iter_mut().zip(&acceleration) {
                    particle.velocity = add3(particle.velocity, acceleration.map(|a| step * a));
                    max_speed =
                        max_speed.max(check_flow_speed(particle.velocity, fluid.sound_speed)?);
                    particle.position =
                        add3(particle.position, particle.velocity.map(|v| step * v));
                    tank.confine(particle)?;
                }
                substeps += 1;
                if step == remaining {
                    break;
                }
                remaining -= step;
                if attempt == 255 {
                    return Err(
                        "SPH requires more than 256 stable substeps per authored step".into(),
                    );
                }
            }
        }
        self.charge(len * len * 100, len * 128)?;
        fluid.forces(&particles, &mut rho, &mut pressure, &mut acceleration)?;
        peak_density_ratio =
            peak_density_ratio.max(rho.iter().copied().fold(0.0, f64::max) / fluid.rest_density);
        let origin = min.map(|x| x - surface_radius);
        let spacing = std::array::from_fn(|k| {
            (max[k] - min[k] + 2.0 * surface_radius) / (dimensions[k] - 1) as f64
        });
        if spacing.iter().any(|d| *d > 1.5 * surface_radius) {
            return Err(
                "SPH surface grid spacing exceeds 1.5 times the reconstruction radius".into(),
            );
        }
        let (field, mesh) = reconstruct(
            self,
            &particles,
            dimensions,
            origin,
            spacing,
            surface_radius,
        )?;
        let particles = self.admit_artifact(TypedArtifact::ParticleSet(Arc::new(ParticleSet {
            positions: particles
                .iter()
                .map(|p| p.position.map(|x| x as f32))
                .collect(),
            velocities: particles
                .iter()
                .map(|p| p.velocity.map(|x| x as f32))
                .collect(),
            mass_kg: vec![fluid.mass as f32; len],
            radii_m: vec![surface_radius as f32; len],
        })))?;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("surface_field".into(), field),
            ("particles".into(), particles),
            (
                "origin_m".into(),
                Value::List(origin.map(Value::F64).into()),
            ),
            (
                "spacing_m".into(),
                Value::List(spacing.map(Value::F64).into()),
            ),
            (
                "surface_field_kind".into(),
                Value::String("sphere_union_implicit_m".into()),
            ),
            ("substeps".into(), Value::U64(substeps)),
            ("mass_kg".into(), Value::F64(fluid.mass * len as f64)),
            ("simulated_seconds".into(), Value::F64(dt * steps as f64)),
            ("peak_density_ratio".into(), Value::F64(peak_density_ratio)),
            (
                "peak_mach".into(),
                Value::F64(max_speed / fluid.sound_speed),
            ),
        ])))
    }
}

fn check_flow_speed(velocity: [f64; 3], sound_speed: f64) -> Result<f64, String> {
    let speed = dot(velocity, velocity).sqrt();
    if !(0.0..=0.25 * sound_speed).contains(&speed) {
        return Err(format!(
            "SPH speed {speed:.9e} m/s exceeds low-Mach limit {:.9e} m/s",
            0.25 * sound_speed
        ));
    }
    Ok(speed)
}

impl Tank {
    fn confine(&self, particle: &mut Particle) -> Result<(), String> {
        for k in 0..3 {
            let lower = self.min[k] + self.margin;
            let upper = self.max[k] - self.margin;
            let position = particle.position[k];
            if !position.is_finite() {
                return Err("SPH particle left the finite simulation domain".into());
            }
            if position < lower {
                particle.position[k] = lower;
                particle.velocity[k] = particle.velocity[k].abs() * self.restitution;
            } else if position > upper {
                particle.position[k] = upper;
                particle.velocity[k] = -particle.velocity[k].abs() * self.restitution;
            }
        }
        Ok(())
    }
}

impl Fluid {
    fn bound(&self, velocity: f64, acceleration: f64) -> f64 {
        let wave = 0.2 * self.support / (self.sound_speed + velocity);
        let viscosity = 0.1 * self.support.powi(2) / self.viscosity.max(f64::MIN_POSITIVE);
        let capillary = 0.2
            * (self.rest_density * self.support.powi(3) / self.tension.max(f64::MIN_POSITIVE))
                .sqrt();
        let force = 0.2 * (self.support / acceleration.max(f64::MIN_POSITIVE)).sqrt();
        wave.min(viscosity).min(capillary).min(force)
    }

    fn forces(
        &self,
        particles: &[Particle],
        rho: &mut [f64],
        pressure: &mut [f64],
        acceleration: &mut [[f64; 3]],
    ) -> Result<(), String> {
        let support_sq = self.support * self.support;
        let poly6 = (315.0 / (64.0 * PI)) / self.support.powi(3);
        rho.fill(self.mass * poly6);
        for (i, a) in particles.iter().enumerate() {
            for (j, b) in particles.iter().enumerate().skip(i + 1) {
                let gap = sub3(a.position, b.position);
                let r_sq = dot(gap, gap);
                if r_sq < support_sq {
                    let contribution = self.mass * poly6 * (1.0 - r_sq / support_sq).powi(3);
                    rho[i] += contribution;
                    rho[j] += contribution;
                }
            }
        }
        for (pressure, &density) in pressure.iter_mut().zip(rho.iter()) {
            let ratio = density / self.rest_density;
            if !(1e-8..=1.5).contains(&ratio) {
                return Err(format!(
                    "SPH density ratio {ratio:.12e} is outside [1e-8, 1.5]"
                ));
            }
            *pressure = (self.sound_speed.powi(2) * self.rest_density / 7.0)
                * (ratio.powi(7) - 1.0).max(0.0);
        }
        acceleration.fill(self.gravity);
        let gradient = 45.0 / (PI * self.support.powi(4));
        let viscosity = 45.0 / (PI * self.support.powi(5));
        for (i, a) in particles.iter().enumerate() {
            for (j, b) in particles.iter().enumerate().skip(i + 1) {
                let difference = sub3(a.position, b.position);
                let distance = dot(difference, difference).sqrt();
                if distance >= self.support {
                    continue;
                }
                if distance <= 1e-8 * self.support {
                    return Err(format!("SPH particle centers {i}, {j} collide numerically"));
                }
                let q = distance / self.support;
                let w = 1.0 - q;
                let p = self.mass
                    * (pressure[i] / rho[i].powi(2) + pressure[j] / rho[j].powi(2))
                    * gradient
                    * w.powi(2);
                let v = self.mass
                    * self.viscosity
                    * 0.5
                    * (rho[i].recip() + rho[j].recip())
                    * viscosity
                    * w;
                let cohesion = self.tension * self.support / self.mass * q * w.powi(2);
                let delta = add3(
                    difference.map(|d| d * (p - cohesion) / distance),
                    sub3(b.velocity, a.velocity).map(|u| u * v),
                );
                acceleration[i] = add3(acceleration[i], delta);
                acceleration[j] = sub3(acceleration[j], delta);
            }
        }
        Ok(())
    }
}

fn reconstruct(
    host: &mut AssetHost<'_>,
    particles: &[Particle],
    dimensions: [usize; 3],
    origin: [f64; 3],
    spacing: [f64; 3],
    radius: f64,
) -> Result<(Value, Value), String> {
    const VERTICES: usize = 65_536;
    const TRIANGLES: usize = 130_000;
    const SLOTS: usize = 1 << 18;
    const PROBES: usize = 8_000_000;
    let [nx, ny, nz] = dimensions;
    let len = nx * ny * nz;
    host.charge(
        len * (particles.len() * 16 + 96) + PROBES,
        len * 16 + VERTICES * 56 + TRIANGLES * 24 + SLOTS * 24,
    )?;
    let mut grid = Vec::with_capacity(len);
    for z in 0..nz {
        for y in 0..ny {
            for x in 0..nx {
                let point = std::array::from_fn(|k| origin[k] + [x, y, z][k] as f64 * spacing[k]);
                let sq = particles
                    .iter()
                    .map(|particle| {
                        let d = sub3(particle.position, point);
                        dot(d, d)
                    })
                    .fold(f64::INFINITY, f64::min);
                grid.push(sq.sqrt() - radius);
            }
        }
    }
    let mut vertices = vec![Point3::default(); VERTICES];
    let mut triangles = vec![[0u32; 3]; TRIANGLES];
    let mut edge_sources = vec![[0usize; 2]; VERTICES];
    let mut slots = vec![TetraEdgeSlot::default(); SLOTS];
    let (nv, nt) = indexed_tetra_surface_capped_into(
        &grid,
        dimensions,
        Point3::new(origin[0], origin[1], origin[2]),
        Point3::new(spacing[0], spacing[1], spacing[2]),
        0.0,
        131_072,
        PROBES,
        0.0,
        &mut vertices,
        &mut triangles,
        &mut edge_sources,
        &mut slots,
    )
    .map_err(|error| format!("SPH indexed surface extraction: {error:?}"))?;
    triangles.truncate(nt);
    let mesh = mesh_artifact(
        host,
        vertices[..nv]
            .iter()
            .map(|p| [p.x as f32, p.y as f32, p.z as f32])
            .collect(),
        triangles,
    )?;
    let field = output_plane(host, vec![nz as u32, ny as u32, nx as u32], &[&grid])?;
    Ok((field, mesh))
}

//! Dimensionless NVE dynamics with compactly supported soft-particle repulsion.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{add3, dot, int_arg, number_arg, output_plane, seal_diagnostics, sub3};
use crate::{fields, list, record, required, text, vector, Args, AssetHost, Value};

#[derive(Clone, Copy)]
struct Particle {
    position: [f64; 3],
    velocity: [f64; 3],
    mass: f64,
}

struct ParticleSystem {
    particles: Vec<Particle>,
    box_length: f64,
    radius: f64,
    total_mass: f64,
}

#[derive(Clone, Copy)]
struct Thermodynamics {
    kinetic: f64,
    potential: f64,
    internal: f64,
    momentum: [f64; 3],
    pressure: f64,
    temperature: f64,
}

impl AssetHost<'_> {
    pub(crate) fn step_molecular_dynamics(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "solver",
                "particles",
                "steps",
                "box_length",
                "interaction_radius",
                "step_size",
                "sample_every",
                "energy_drift_tolerance",
            ],
        )?;
        if let Some(solver) = args.get("solver") {
            if text(solver)? != "dimensionless_periodic_soft_particles" {
                return Err("Supported solver is dimensionless_periodic_soft_particles".into());
            }
        }
        let box_length = number_arg(args, "box_length", Some(8.0), 0.5..=100.0)?;
        let radius = number_arg(args, "interaction_radius", Some(1.0), 0.05..=25.0)?;
        if radius >= 0.5 * box_length {
            return Err("interaction_radius must be smaller than half the periodic box".into());
        }
        let dt = number_arg(args, "step_size", Some(0.002), 1e-5..=0.1)?;
        let steps = int_arg(args, "steps", None, 1..=8192)?;
        let stride = int_arg(args, "sample_every", Some(steps.min(10)), 1..=steps)?;
        let tolerance = number_arg(args, "energy_drift_tolerance", Some(1e-3), 1e-8..=0.05)?;
        let mut system = ParticleSystem::read(args, box_length, radius)?;
        let count = system.particles.len();
        let min_mass = system
            .particles
            .iter()
            .map(|p| p.mass)
            .fold(f64::INFINITY, f64::min);
        // Bound the mass-weighted Hessian with the complete graph Laplacian
        // and the maximum pair curvature 8/radius^2.
        let frequency_bound = (8.0 * count as f64 / min_mass).sqrt() / radius;
        if dt * frequency_bound > 0.35 {
            return Err(
                "step_size must resolve the maximum soft-particle interaction frequency".into(),
            );
        }
        let frames = steps.div_ceil(stride) + 1;
        let samples = 3 * count * frames;
        let pair_count = count * (count - 1) / 2;
        let work_per_step = 70 * pair_count + 60 * count;
        let scratch = 24 * samples + 192 * count;
        self.charge(work_per_step, scratch)?;
        let hash = system.digest(dt, steps, stride, tolerance);
        let mut force = vec![[0.0; 3]; count];
        let (potential, virial) = system.pair_forces(&mut force);
        let initial = system.thermodynamics(potential, virial);
        let energy_reference = initial.internal.max(1e-10);
        let mut positions = Vec::with_capacity(samples);
        let mut velocities = Vec::with_capacity(samples);
        let mut times = Vec::with_capacity(frames);
        let mut final_stats = initial;
        let mut peak_energy_drift = 0.0_f64;
        let mut peak_momentum_drift = 0.0_f64;
        system.sample(&mut positions, &mut velocities);
        times.push(Value::F64(0.0));
        for step in 1..=steps {
            const STEPS_PER_CHARGE: usize = 64;
            if (step - 1) % STEPS_PER_CHARGE == 0 {
                self.charge(
                    work_per_step * (steps - step + 1).min(STEPS_PER_CHARGE),
                    scratch,
                )?;
            }
            system.half_kick(&force, dt);
            system.drift(dt)?;
            let (potential, virial) = system.pair_forces(&mut force);
            system.half_kick(&force, dt);
            final_stats = system.thermodynamics(potential, virial);
            let energy_drift = (final_stats.internal - initial.internal).abs() / energy_reference;
            let momentum_delta = sub3(final_stats.momentum, initial.momentum);
            let momentum_drift = dot(momentum_delta, momentum_delta).sqrt();
            if !energy_drift.is_finite() || !momentum_drift.is_finite() || energy_drift > tolerance
            {
                return Err(format!("Particle step {step} exceeded normalized energy drift {tolerance:.3e}: {energy_drift:.3e}"));
            }
            peak_energy_drift = peak_energy_drift.max(energy_drift);
            peak_momentum_drift = peak_momentum_drift.max(momentum_drift);
            if step % stride == 0 || step == steps {
                system.sample(&mut positions, &mut velocities);
                times.push(Value::F64(step as f64 * dt));
            }
        }
        let metrics = [
            ("kinetic_energy", final_stats.kinetic),
            ("potential_energy", final_stats.potential),
            ("internal_energy", final_stats.internal),
            ("total_energy", final_stats.kinetic + final_stats.potential),
            ("initial_internal_energy", initial.internal),
            ("virial_pressure", final_stats.pressure),
            ("kinetic_temperature", final_stats.temperature),
            ("max_normalized_internal_energy_drift", peak_energy_drift),
            ("max_absolute_momentum_drift", peak_momentum_drift),
        ];
        let receipt = seal_diagnostics(
            self,
            "dimensionless-periodic-soft-particle-nve",
            &hash,
            &metrics,
        )?;
        let trajectory = output_plane(
            self,
            vec![frames as u32, count as u32, 3],
            &[&positions, &velocities],
        )?;
        let stats = metrics
            .into_iter()
            .map(|(name, value)| (name.into(), Value::F64(value)))
            .collect();
        Ok(Value::Record(Args::from([
            ("trajectory".into(), trajectory),
            ("sample_times".into(), Value::List(times.into())),
            ("box_length".into(), Value::F64(box_length)),
            ("thermodynamics".into(), Value::Record(stats)),
            ("receipt".into(), receipt),
        ])))
    }
}

impl ParticleSystem {
    fn read(args: &Args, box_length: f64, radius: f64) -> Result<Self, String> {
        let authored = list(required(args, "particles")?)?;
        if !(2..=128).contains(&authored.len()) {
            return Err("Provide 2..=128 dimensionless particles".into());
        }
        let mut particles = Vec::with_capacity(authored.len());
        for value in authored {
            let row = record(value)?;
            fields(row, &["position", "velocity", "mass"])?;
            let position = vector::<3>(required(row, "position")?)?;
            let velocity = vector::<3>(required(row, "velocity")?)?;
            let mass = number_arg(row, "mass", Some(1.0), 0.1..=10.0)?;
            if position.iter().any(|x| !(0.0..box_length).contains(x))
                || velocity.iter().any(|v| !(-25.0..=25.0).contains(v))
            {
                return Err(
                    "Particle positions must lie in [0, box_length), with velocities in [-25, 25]"
                        .into(),
                );
            }
            particles.push(Particle {
                position,
                velocity,
                mass,
            });
        }
        let total_mass = particles.iter().map(|particle| particle.mass).sum();
        Ok(Self {
            particles,
            box_length,
            radius,
            total_mass,
        })
    }

    fn pair_forces(&self, force: &mut [[f64; 3]]) -> (f64, f64) {
        force.fill([0.0; 3]);
        let mut potential = 0.0;
        let mut virial = 0.0;
        let inv_radius2 = 1.0 / self.radius.powi(2);
        for (i, p) in self.particles.iter().enumerate() {
            for (j, q) in self.particles.iter().enumerate().skip(i + 1) {
                let delta = sub3(p.position, q.position)
                    .map(|d| d - self.box_length * (d / self.box_length).round());
                let distance2 = dot(delta, delta);
                let taper = 1.0 - distance2 * inv_radius2;
                if taper > 0.0 {
                    let scale = 8.0 * inv_radius2 * taper.powi(3);
                    let pair = delta.map(|d| d * scale);
                    force[i] = add3(force[i], pair);
                    force[j] = sub3(force[j], pair);
                    potential += taper.powi(4);
                    virial += scale * distance2;
                }
            }
        }
        (potential, virial)
    }

    fn half_kick(&mut self, force: &[[f64; 3]], dt: f64) {
        for (particle, force) in self.particles.iter_mut().zip(force) {
            let scale = 0.5 * dt / particle.mass;
            particle.velocity = add3(particle.velocity, force.map(|f| scale * f));
        }
    }

    fn drift(&mut self, dt: f64) -> Result<(), String> {
        let momentum = self.momentum();
        let center = momentum.map(|p| p / self.total_mass);
        for velocity in self.particles.iter().map(|p| sub3(p.velocity, center)) {
            if dt * dot(velocity, velocity).sqrt() > 0.25 * self.radius {
                return Err(
                    "Particle relative step exceeds one quarter of the interaction radius".into(),
                );
            }
        }
        for particle in &mut self.particles {
            particle.position =
                add3(particle.position, particle.velocity.map(|v| dt * v)).map(|x| {
                    let wrapped = x.rem_euclid(self.box_length);
                    if wrapped == self.box_length {
                        0.0
                    } else {
                        wrapped
                    }
                });
        }
        Ok(())
    }

    fn momentum(&self) -> [f64; 3] {
        self.particles
            .iter()
            .fold([0.0; 3], |sum, p| add3(sum, p.velocity.map(|v| p.mass * v)))
    }

    fn thermodynamics(&self, potential: f64, virial: f64) -> Thermodynamics {
        let momentum = self.momentum();
        let center = momentum.map(|p| p / self.total_mass);
        let peculiar: f64 = self
            .particles
            .iter()
            .map(|particle| {
                let velocity = sub3(particle.velocity, center);
                0.5 * particle.mass * dot(velocity, velocity)
            })
            .sum();
        Thermodynamics {
            kinetic: peculiar + 0.5 * self.total_mass * dot(center, center),
            potential,
            internal: peculiar + potential,
            momentum,
            pressure: (2.0 * peculiar + virial) / (3.0 * self.box_length.powi(3)),
            temperature: 2.0 * peculiar / (3.0 * (self.particles.len() - 1) as f64),
        }
    }

    fn sample(&self, positions: &mut Vec<f64>, velocities: &mut Vec<f64>) {
        for particle in &self.particles {
            positions.extend(particle.position.map(|x| {
                let quantized = f64::from(x as f32);
                if quantized >= self.box_length {
                    0.0
                } else {
                    quantized
                }
            }));
            velocities.extend(particle.velocity);
        }
    }

    fn digest(&self, dt: f64, steps: usize, stride: usize, tolerance: f64) -> String {
        let mut hasher =
            blake3::Hasher::new_derive_key("qualia dimensionless periodic soft particles v1");
        for count in [self.particles.len(), steps, stride] {
            hasher.update(&(count as u64).to_le_bytes());
        }
        for value in [dt, self.box_length, self.radius, tolerance]
            .into_iter()
            .chain(
                self.particles
                    .iter()
                    .flat_map(|p| p.position.into_iter().chain(p.velocity).chain([p.mass])),
            )
        {
            hasher.update(&value.to_bits().to_le_bytes());
        }
        hasher.finalize().to_hex().to_string()
    }
}

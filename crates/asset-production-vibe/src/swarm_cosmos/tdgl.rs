//! Gauge-covariant 2D time-dependent Ginzburg–Landau flow for a film.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{count, dimensions, names, scalar};
use crate::{artifact::FieldLattice, fields, list, num, Args, AssetHost, TypedArtifact, Value};
use std::{
    collections::BTreeMap,
    f64::consts::{PI, TAU},
    sync::Arc,
};

const MAX_SOLVE_ITERATIONS: usize = 32;
const FLUX_QUANTUM_WB: f64 = 6.626_070_15e-34 / (2.0 * 1.602_176_634e-19);

struct FilmState {
    order: Vec<[f64; 2]>,
    horizontal: Vec<f64>,
    vertical: Vec<f64>,
}

struct Film {
    nx: usize,
    ny: usize,
    spacing: f64,
    kappa_squared: f64,
    applied_circulation: f64,
    alpha: Vec<f64>,
}

impl AssetHost<'_> {
    pub(crate) fn solve_tdgl_cryogenic_superconductivity(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "grid_size",
                "grid_spacing_xi",
                "steps",
                "dt_relaxation",
                "temperature_k",
                "critical_temperature_k",
                "gl_kappa",
                "applied_flux_phase_per_cell",
                "coherence_length_m",
                "pinning_suppression",
                "initial_amplitude",
                "initial_phase_rad",
                "initial_link_x_phase_rad",
                "initial_link_y_phase_rad",
            ],
        )?;
        let [nx, ny] = dimensions(args, "grid_size", [24, 24], 128)?;
        let n = nx * ny;
        let spacing = scalar(args, "grid_spacing_xi", Some(0.5), 0.1, 2.0)?;
        let kappa = scalar(args, "gl_kappa", Some(2.0), 0.01, 500.0)?;
        let temperature = scalar(args, "temperature_k", Some(4.0), 0.001, 300.0)?;
        let critical = scalar(args, "critical_temperature_k", Some(8.0), 0.001, 300.0)?;
        let coherence = scalar(args, "coherence_length_m", Some(1e-8), 1e-11, 1e-3)?;
        let applied_circulation = scalar(
            args,
            "applied_flux_phase_per_cell",
            Some(0.0),
            -PI / 2.0,
            PI / 2.0,
        )?;
        let steps = count(args, "steps", 100, 1, 4_096)?;
        self.charge(n * 500, n * 384 + 65_536)?;
        let pinning =
            lane(args, "pinning_suppression", n, 0.0, 2.0)?.unwrap_or_else(|| vec![0.0; n]);
        let alpha = pinning
            .into_iter()
            .map(|s| 1.0 - temperature / critical - s)
            .collect::<Vec<_>>();
        let largest_alpha = alpha.iter().fold(0.0_f64, |peak, x| peak.max(x.abs()));
        let kappa_squared = kappa * kappa;
        let rate_scale = 8.0 * (1.0 + kappa_squared) / (spacing * spacing) + largest_alpha + 32.0;
        let dt_cap = 0.6 / rate_scale;
        let dt = scalar(args, "dt_relaxation", Some(dt_cap), 1e-12, dt_cap)?;
        let film = Film {
            nx,
            ny,
            spacing,
            kappa_squared,
            applied_circulation,
            alpha,
        };
        let amplitude = lane(args, "initial_amplitude", n, 0.0, 2.0)?
            .unwrap_or_else(|| film.alpha.iter().map(|x| x.max(0.0).sqrt()).collect());
        let phase = lane(args, "initial_phase_rad", n, -1e5, 1e5)?.unwrap_or_else(|| vec![0.0; n]);
        let order = amplitude
            .into_iter()
            .zip(phase)
            .map(|(amplitude, phase)| {
                let (sin, cos) = phase.sin_cos();
                [amplitude * cos, amplitude * sin]
            })
            .collect();
        let horizontal = lane(args, "initial_link_x_phase_rad", (nx - 1) * ny, -1e5, 1e5)?;
        let vertical = lane(args, "initial_link_y_phase_rad", nx * (ny - 1), -1e5, 1e5)?;
        let (horizontal, vertical) = match (horizontal, vertical) {
            (Some(horizontal), Some(vertical)) => (horizontal, vertical),
            (None, None) => (
                vec![0.0; (nx - 1) * ny],
                (0..nx * (ny - 1))
                    .map(|i| (i as f64 % nx as f64 - (nx - 1) as f64 * 0.5) * applied_circulation)
                    .collect(),
            ),
            _ => {
                return Err(
                    "Provide both initial link lanes or use the applied-field initialization"
                        .into(),
                )
            }
        };
        let mut state = FilmState {
            order,
            horizontal,
            vertical,
        };
        if film.max_plaquette_circulation(&state) > PI {
            return Err(
                "Initial plaquette circulation exceeds the resolved half-flux-quantum limit".into(),
            );
        }
        let mut base = FilmState::blank(nx, ny);
        let mut trial = FilmState::blank(nx, ny);
        let mut rates = FilmState::blank(nx, ny);
        let init_energy = film.free_energy(&state);
        let mut energy = init_energy;
        let mut max_iterations = 0;
        for _ in 0..steps {
            self.charge(n * 176, 0)?;
            film.rates(&state, &mut rates);
            base.set_sum(&state, &rates, 0.5 * dt);
            trial.set_sum(&state, &rates, dt);
            let mut converged = false;
            for iteration in 1..=MAX_SOLVE_ITERATIONS {
                self.charge(n * 200, 0)?;
                film.rates(&trial, &mut rates);
                let change = trial.set_sum(&base, &rates, 0.5 * dt);
                if change <= 1e-11 {
                    max_iterations = max_iterations.max(iteration);
                    converged = true;
                    break;
                }
            }
            if !converged {
                return Err("TDGL Crank–Nicolson iteration failed to converge".into());
            }
            let next_energy = film.free_energy(&trial);
            if !next_energy.is_finite() || next_energy > energy + 1e-10 * (1.0 + energy.abs()) {
                return Err("TDGL flow left its finite, dissipative energy domain".into());
            }
            energy = next_energy;
            std::mem::swap(&mut state, &mut trial);
        }
        if film.max_plaquette_circulation(&state) > PI {
            return Err(
                "Final plaquette circulation is too coarse to resolve vortex winding".into(),
            );
        }
        let mut order_samples = Vec::with_capacity(3 * n);
        order_samples.extend(state.order.iter().map(|p| p[0] as f32));
        order_samples.extend(state.order.iter().map(|p| p[1] as f32));
        order_samples.extend(state.order.iter().map(|p| squared_norm(*p) as f32));
        film.rates(&state, &mut rates);
        let rate_peak = rates
            .order
            .iter()
            .map(|p| squared_norm(*p).sqrt())
            .chain(
                rates
                    .horizontal
                    .iter()
                    .chain(&rates.vertical)
                    .map(|x| x.abs()),
            )
            .fold(0.0_f64, f64::max);
        let mut flux_samples = Vec::with_capacity(2 * (nx - 1) * (ny - 1));
        let mut windings = Vec::with_capacity((nx - 1) * (ny - 1));
        let mut field_sum = 0.0;
        let mut net_winding = 0_i64;
        let mut unresolved = 0_u64;
        for y in 0..ny - 1 {
            for x in 0..nx - 1 {
                let circulation = film.circulation(&state, x, y);
                let flux_density = circulation / (spacing * spacing);
                flux_samples.push(flux_density as f32);
                field_sum += flux_density;
                let winding = film.winding(&state, x, y, circulation);
                net_winding += winding.unwrap_or_default();
                unresolved += u64::from(u8::from(winding.is_none()));
                windings.push(winding);
            }
        }
        let average_field = field_sum / flux_samples.len() as f64;
        // The resolved mask accompanies the winding lane at sites where the phase is undefined.
        flux_samples.extend(windings.iter().map(|v| v.unwrap_or_default() as f32));
        flux_samples.extend(windings.iter().map(|v| f32::from(u8::from(v.is_some()))));
        let current_x = (0..ny)
            .flat_map(|y| (0..nx - 1).map(move |x| (x, y)))
            .map(|(x, y)| {
                let i = y * nx + x;
                (current(
                    state.order[i],
                    state.order[i + 1],
                    state.horizontal[y * (nx - 1) + x],
                ) / spacing) as f32
            })
            .collect();
        let current_y = (0..ny - 1)
            .flat_map(|y| (0..nx).map(move |x| (x, y)))
            .map(|(x, y)| {
                let i = y * nx + x;
                (current(state.order[i], state.order[i + nx], state.vertical[i]) / spacing) as f32
            })
            .collect();
        let order_parameter =
            admit_film_lattice(self, vec![3, ny as u32, nx as u32], order_samples)?;
        let flux_and_vortices =
            admit_film_lattice(self, vec![3, ny as u32 - 1, nx as u32 - 1], flux_samples)?;
        let current_x = admit_film_lattice(self, vec![ny as u32, nx as u32 - 1], current_x)?;
        let current_y = admit_film_lattice(self, vec![ny as u32 - 1, nx as u32], current_y)?;
        let link_x = admit_film_lattice(
            self,
            vec![ny as u32, nx as u32 - 1],
            state.horizontal.iter().map(|v| *v as f32).collect(),
        )?;
        let link_y = admit_film_lattice(
            self,
            vec![ny as u32 - 1, nx as u32],
            state.vertical.iter().map(|v| *v as f32).collect(),
        )?;
        let field_scale_t = FLUX_QUANTUM_WB / (TAU * coherence * coherence);
        Ok(Value::Record(BTreeMap::from([
            ("order_parameter".into(), order_parameter),
            (
                "order_channels".into(),
                names(["psi_re", "psi_im", "condensate_density"]),
            ),
            ("flux_and_vortices".into(), flux_and_vortices),
            (
                "flux_channels".into(),
                names(["b_z_dimensionless", "winding", "winding_resolved"]),
            ),
            ("current_x".into(), current_x),
            ("current_y".into(), current_y),
            ("link_x_phase_rad".into(), link_x),
            ("link_y_phase_rad".into(), link_y),
            ("time_relaxation".into(), Value::F64(steps as f64 * dt)),
            ("dt_relaxation".into(), Value::F64(dt)),
            ("grid_spacing_xi".into(), Value::F64(spacing)),
            ("field_scale_t".into(), Value::F64(field_scale_t)),
            (
                "average_b_z_t".into(),
                Value::F64(average_field * field_scale_t),
            ),
            (
                "applied_b_z_t".into(),
                Value::F64(applied_circulation / (spacing * spacing) * field_scale_t),
            ),
            (
                "initial_free_energy_dimensionless".into(),
                Value::F64(init_energy),
            ),
            ("final_free_energy_dimensionless".into(), Value::F64(energy)),
            ("final_peak_rate".into(), Value::F64(rate_peak)),
            (
                "max_nonlinear_iterations".into(),
                Value::U64(max_iterations as u64),
            ),
            ("net_resolved_winding".into(), Value::I64(net_winding)),
            ("unresolved_plaquettes".into(), Value::U64(unresolved)),
            (
                "model".into(),
                Value::String("tdgl_2d_link_crank_nicolson_temporal_gauge".into()),
            ),
        ])))
    }
}

impl FilmState {
    fn blank(nx: usize, ny: usize) -> Self {
        Self {
            order: vec![[0.0; 2]; nx * ny],
            horizontal: vec![0.0; (nx - 1) * ny],
            vertical: vec![0.0; nx * (ny - 1)],
        }
    }

    fn set_sum(&mut self, base: &Self, increment: &Self, dt: f64) -> f64 {
        let mut max_change: f64 = 0.0;
        for ((dst, &base), &increment) in
            self.order.iter_mut().zip(&base.order).zip(&increment.order)
        {
            let next = std::array::from_fn(|i| base[i] + dt * increment[i]);
            max_change = max_change.max(squared_norm([next[0] - dst[0], next[1] - dst[1]]).sqrt());
            *dst = next;
        }
        let dst = self.horizontal.iter_mut().chain(&mut self.vertical);
        let base = base.horizontal.iter().chain(&base.vertical);
        let increment = increment.horizontal.iter().chain(&increment.vertical);
        for ((dst, &base), &increment) in dst.zip(base).zip(increment) {
            let next = base + dt * increment;
            max_change = max_change.max((next - *dst).abs());
            *dst = next;
        }
        max_change
    }
}

impl Film {
    fn rates(&self, state: &FilmState, rate: &mut FilmState) {
        for ((rate, &order), alpha) in rate.order.iter_mut().zip(&state.order).zip(&self.alpha) {
            *rate = order.map(|v| v * (alpha - squared_norm(order)));
        }
        let inverse_h2 = 1.0 / (self.spacing * self.spacing);
        for y in 0..self.ny {
            for x in 0..self.nx {
                let i = y * self.nx + x;
                if x + 1 < self.nx {
                    let edge = y * (self.nx - 1) + x;
                    rate.horizontal[edge] = link_flow(
                        state,
                        &mut rate.order,
                        state.horizontal[edge],
                        i,
                        i + 1,
                        inverse_h2,
                    );
                }
                if y + 1 < self.ny {
                    rate.vertical[i] = link_flow(
                        state,
                        &mut rate.order,
                        state.vertical[i],
                        i,
                        i + self.nx,
                        inverse_h2,
                    );
                }
            }
        }
        let magnetic_weight = self.kappa_squared * inverse_h2;
        for y in 0..self.ny - 1 {
            for x in 0..self.nx - 1 {
                let drive =
                    magnetic_weight * (self.circulation(state, x, y) - self.applied_circulation);
                let h = y * (self.nx - 1) + x;
                let v = y * self.nx + x;
                rate.horizontal[h] -= drive;
                rate.horizontal[h + self.nx - 1] += drive;
                rate.vertical[v] += drive;
                rate.vertical[v + 1] -= drive;
            }
        }
    }

    fn circulation(&self, state: &FilmState, x: usize, y: usize) -> f64 {
        let h = y * (self.nx - 1) + x;
        let v = y * self.nx + x;
        state.horizontal[h] + state.vertical[v + 1]
            - state.horizontal[h + self.nx - 1]
            - state.vertical[v]
    }

    fn max_plaquette_circulation(&self, state: &FilmState) -> f64 {
        (0..self.ny - 1)
            .flat_map(|y| (0..self.nx - 1).map(move |x| self.circulation(state, x, y).abs()))
            .fold(0.0_f64, f64::max)
    }

    fn free_energy(&self, state: &FilmState) -> f64 {
        let h2 = self.spacing * self.spacing;
        let mut energy = 0.0;
        for y in 0..self.ny {
            for x in 0..self.nx {
                let i = y * self.nx + x;
                let order = state.order[i];
                let density = squared_norm(order);
                energy += h2 * (0.5 * density * density - self.alpha[i] * density);
                if x + 1 < self.nx {
                    let theta = state.horizontal[y * (self.nx - 1) + x];
                    energy += link_energy(order, state.order[i + 1], theta);
                }
                if y + 1 < self.ny {
                    energy += link_energy(order, state.order[i + self.nx], state.vertical[i]);
                }
                if x + 1 < self.nx && y + 1 < self.ny {
                    energy += (self.kappa_squared / h2)
                        * (self.circulation(state, x, y) - self.applied_circulation).powi(2);
                }
            }
        }
        energy
    }

    fn winding(&self, state: &FilmState, x: usize, y: usize, circulation: f64) -> Option<i64> {
        let a = y * self.nx + x;
        let b = a + 1;
        let d = a + self.nx;
        let c = d + 1;
        let h = y * (self.nx - 1) + x;
        let corners = [
            state.order[a],
            state.order[b],
            state.order[c],
            state.order[d],
        ];
        if corners.iter().any(|&p| squared_norm(p) < 1e-12) {
            return None;
        }
        let phase = corners.map(|p| p[1].atan2(p[0]));
        let links = [
            state.horizontal[h],
            state.vertical[b],
            -state.horizontal[h + self.nx - 1],
            -state.vertical[a],
        ];
        let mut winding = circulation;
        for i in 0..4 {
            let diff = (phase[(i + 1) % 4] - phase[i] - links[i] + PI).rem_euclid(TAU) - PI;
            if (diff.abs() - PI).abs() < 1e-7 {
                return None;
            }
            winding += diff;
        }
        Some((winding / TAU).round() as i64)
    }
}

fn link_flow(
    state: &FilmState,
    rate: &mut [[f64; 2]],
    theta: f64,
    a: usize,
    b: usize,
    inverse_h2: f64,
) -> f64 {
    let u = state.order[a];
    let v = state.order[b];
    let (sin, cos) = theta.sin_cos();
    let forward = [cos * v[0] + sin * v[1], cos * v[1] - sin * v[0]];
    let backward = [cos * u[0] - sin * u[1], cos * u[1] + sin * u[0]];
    for lane in 0..2 {
        rate[a][lane] += inverse_h2 * (forward[lane] - u[lane]);
        rate[b][lane] += inverse_h2 * (backward[lane] - v[lane]);
    }
    u[0] * forward[1] - u[1] * forward[0]
}

fn squared_norm(v: [f64; 2]) -> f64 {
    v[0] * v[0] + v[1] * v[1]
}

fn link_energy(u: [f64; 2], v: [f64; 2], theta: f64) -> f64 {
    let (sin, cos) = theta.sin_cos();
    squared_norm([
        cos * v[0] + sin * v[1] - u[0],
        cos * v[1] - sin * v[0] - u[1],
    ])
}

fn current(u: [f64; 2], v: [f64; 2], theta: f64) -> f64 {
    let (sin, cos) = theta.sin_cos();
    cos * (u[0] * v[1] - u[1] * v[0]) - sin * (u[0] * v[0] + u[1] * v[1])
}

fn lane(
    args: &Args,
    key: &str,
    count: usize,
    low: f64,
    high: f64,
) -> Result<Option<Vec<f64>>, String> {
    let Some(value) = args.get(key) else {
        return Ok(None);
    };
    let values = list(value)?;
    if values.len() != count {
        return Err(format!("{key} must have {count} samples"));
    }
    values
        .iter()
        .map(|v| {
            let value = num(v)?;
            (low..=high)
                .contains(&value)
                .then_some(value)
                .ok_or_else(|| format!("{key} samples must be in [{low}, {high}]"))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}

fn admit_film_lattice(
    host: &mut AssetHost<'_>,
    axes: Vec<u32>,
    samples: Vec<f32>,
) -> Result<Value, String> {
    if samples.iter().any(|x| !x.is_finite()) {
        return Err("TDGL fields exceed finite DenseF32 output".into());
    }
    host.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
        FieldLattice::DenseF32 { axes, samples },
    )))
}

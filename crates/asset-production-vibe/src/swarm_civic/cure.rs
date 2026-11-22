//! Lumped thermoset cure and boundary heat exchange for authored material data.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, fields, list, record, required, vector};

const R: f64 = 8.314_462_618_153_24;
const MAX_ATTEMPTS: usize = 32_768;
const MIN_TEMP_K: f64 = 250.0;
const MAX_TEMP_K: f64 = 673.15;

struct Material {
    prefactor_per_s: [f64; 2],
    activation_j_mol: [f64; 2],
    autocatalytic_power: f64,
    depletion_power: f64,
    heat_capacity_j_kg_k: f64,
    reaction_heat_j_kg: f64,
    boundary_conductance_w_kg_k: f64,
}

impl Material {
    fn read(args: &Args) -> Result<Self, String> {
        fields(
            args,
            &[
                "prefactor_per_s",
                "activation_j_mol",
                "autocatalytic_power",
                "depletion_power",
                "heat_capacity_j_kg_k",
                "reaction_heat_j_kg",
                "boundary_conductance_w_kg_k",
            ],
        )?;
        let prefactor_per_s = vector::<2>(required(args, "prefactor_per_s")?)?;
        let activation_j_mol = vector::<2>(required(args, "activation_j_mol")?)?;
        if prefactor_per_s.iter().any(|a| !(0.0..=1e16).contains(a))
            || activation_j_mol
                .iter()
                .any(|e| !(0.0..=300_000.0).contains(e))
        {
            return Err(
                "Authored Arrhenius coefficients are outside the admitted model range".into(),
            );
        }
        Ok(Self {
            prefactor_per_s,
            activation_j_mol,
            autocatalytic_power: bounded(args, "autocatalytic_power", None, 0.01, 8.0)?,
            depletion_power: bounded(args, "depletion_power", None, 1.0, 8.0)?,
            heat_capacity_j_kg_k: bounded(args, "heat_capacity_j_kg_k", None, 100.0, 10_000.0)?,
            reaction_heat_j_kg: bounded(args, "reaction_heat_j_kg", None, 0.0, 1e6)?,
            boundary_conductance_w_kg_k: bounded(
                args,
                "boundary_conductance_w_kg_k",
                None,
                0.0,
                1e5,
            )?,
        })
    }

    fn rates(&self, state: State, environment_k: f64) -> [f64; 2] {
        // Kamal–Sourour autocatalytic model, doi:10.1002/pen.760130110.
        let [k1, k2] = std::array::from_fn(|i| {
            self.prefactor_per_s[i] * (-self.activation_j_mol[i] / (R * state.temperature_k)).exp()
        });
        let degree = state.degree;
        let rate = (k1 + k2 * degree.powf(self.autocatalytic_power))
            * (1.0 - degree).powf(self.depletion_power);
        let boundary_heat_rate =
            self.boundary_conductance_w_kg_k * (environment_k - state.temperature_k);
        [rate, boundary_heat_rate]
    }

    fn advance(&self, initial: State, environment_k: f64, dt: f64) -> Option<(State, f64)> {
        let [a1, q1] = self.rates(initial, environment_k);
        let euler = initial.advance(self, a1 * dt, q1 * dt)?;
        let [a2, q2] = self.rates(euler, environment_k);
        let heat = dt * 0.5 * (q1 + q2);
        let state = initial.advance(self, dt * 0.5 * (a1 + a2), heat)?;
        Some((state, heat))
    }
}

#[derive(Clone, Copy)]
struct State {
    degree: f64,
    temperature_k: f64,
}

impl State {
    fn advance(self, material: &Material, conversion: f64, boundary_heat: f64) -> Option<Self> {
        let degree = self.degree + conversion;
        let temperature_k = self.temperature_k
            + ((degree - self.degree) * material.reaction_heat_j_kg + boundary_heat)
                / material.heat_capacity_j_kg_k;
        ((self.degree..=1.0).contains(&degree)
            && (MIN_TEMP_K..=MAX_TEMP_K).contains(&temperature_k))
        .then_some(Self {
            degree,
            temperature_k,
        })
    }
}

struct Stage {
    environment_k: f64,
    duration_s: f64,
}

impl AssetHost<'_> {
    pub(crate) fn simulate_molding_casting_and_composites(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "material",
                "temperature_schedule",
                "initial_degree",
                "initial_temperature_k",
                "max_step_s",
                "temperature_error_k",
                "conversion_error",
            ],
        )?;
        let input = list(required(args, "temperature_schedule")?)?;
        if !(1..=512).contains(&input.len()) {
            return Err("Cure model requires 1–512 piecewise-constant thermal stages".into());
        }
        self.charge(MAX_ATTEMPTS * 256, 4096 * (input.len() + 1))?;
        let material = Material::read(record(required(args, "material")?)?)?;
        let max_step = bounded(args, "max_step_s", Some(60.0), 1e-3, 3600.0)?;
        let thermal_tol = bounded(args, "temperature_error_k", Some(0.005), 1e-5, 0.5)?;
        let conversion_tol = bounded(args, "conversion_error", Some(1e-6), 1e-9, 1e-3)?;
        let initial = State {
            degree: bounded(args, "initial_degree", None, 0.0, 1.0)?,
            temperature_k: bounded(args, "initial_temperature_k", None, MIN_TEMP_K, MAX_TEMP_K)?,
        };
        let stages = input
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(row, &["environment_temperature_k", "duration_s"])?;
                Ok(Stage {
                    environment_k: bounded(
                        row,
                        "environment_temperature_k",
                        None,
                        MIN_TEMP_K,
                        MAX_TEMP_K,
                    )?,
                    duration_s: bounded(row, "duration_s", None, 1e-3, 86_400.0)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut state = initial;
        let mut attempts = 0;
        let mut accepted = 0;
        let mut time = 0.0;
        let mut cumulative_boundary_j_kg = 0.0;
        let mut peak = initial.temperature_k;
        let mut worst_balance = 0.0_f64;
        let mut worst_estimate = 0.0_f64;
        let mut rows = Vec::with_capacity(stages.len() + 1);
        rows.push(numeric_row([
            time,
            state.temperature_k,
            state.degree,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0,
        ]));
        for stage in stages {
            let mut elapsed = 0.0;
            let mut proposed_dt = max_step;
            while elapsed < stage.duration_s {
                if attempts >= MAX_ATTEMPTS {
                    return Err(format!(
                        "Cure solver exhausted {MAX_ATTEMPTS} admitted attempts at t={:.9e} s",
                        time + elapsed
                    ));
                }
                if attempts % 256 == 0 && self.cancel.load(std::sync::atomic::Ordering::Relaxed) {
                    return Err("Cure simulation cancelled".into());
                }
                let remaining = stage.duration_s - elapsed;
                let dt = remaining.min(proposed_dt);
                if dt < remaining && (dt < 1e-7 || elapsed + dt == elapsed) {
                    return Err(format!(
                        "Cure integration cannot keep the admitted state and error bounds at t={:.9e} s",
                        time + elapsed
                    ));
                }
                attempts += 1;
                // Second-order Heun with a step-doubling local error estimate.
                let trial =
                    material
                        .advance(state, stage.environment_k, dt)
                        .and_then(|(coarse, _)| {
                            let (mid, input1) =
                                material.advance(state, stage.environment_k, dt * 0.5)?;
                            let (fine, input2) =
                                material.advance(mid, stage.environment_k, dt * 0.5)?;
                            let error = ((fine.temperature_k - coarse.temperature_k).abs()
                                / (3.0 * thermal_tol))
                                .max((fine.degree - coarse.degree).abs() / (3.0 * conversion_tol));
                            (error <= 1.0 && error.is_finite()).then_some((
                                fine,
                                input1 + input2,
                                error,
                                mid.temperature_k,
                            ))
                        });
                let Some((next, boundary_heat, error, midpoint_temp)) = trial else {
                    proposed_dt = dt * 0.5;
                    continue;
                };
                state = next;
                accepted += 1;
                cumulative_boundary_j_kg += boundary_heat;
                peak = peak.max(midpoint_temp).max(state.temperature_k);
                worst_estimate = worst_estimate.max(error);
                if dt == remaining {
                    elapsed = stage.duration_s;
                } else {
                    elapsed += dt;
                }
                proposed_dt = if error < 0.125 {
                    (dt * 2.0).min(max_step)
                } else {
                    dt
                };
            }
            time += stage.duration_s;
            let released = material.reaction_heat_j_kg * (state.degree - initial.degree);
            let sensible =
                material.heat_capacity_j_kg_k * (state.temperature_k - initial.temperature_k);
            let residual = (sensible - cumulative_boundary_j_kg - released).abs();
            let scale = released
                .abs()
                .max(cumulative_boundary_j_kg.abs())
                .max(sensible.abs())
                .max(1.0);
            if !residual.is_finite() || residual > 1e-8 + 1e-9 * scale {
                return Err(format!(
                    "Cure energy balance at t={time:.9e} s leaves {residual:.9e} J/kg"
                ));
            }
            worst_balance = worst_balance.max(residual);
            rows.push(numeric_row([
                time,
                state.temperature_k,
                state.degree,
                cumulative_boundary_j_kg,
                released,
                sensible,
                residual,
                accepted as f64,
            ]));
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:lumped-nonisothermal-kamal-sourour-heun",
            &[
                "time_s",
                "material_temperature_k",
                "cure_degree",
                "cumulative_boundary_j_kg",
                "cumulative_reaction_j_kg",
                "sensible_energy_change_j_kg",
                "energy_residual_j_kg",
                "accepted_step_count",
            ],
            &rows,
        )?;
        out.insert("final_cure_degree".into(), Value::F64(state.degree));
        out.insert(
            "final_temperature_k".into(),
            Value::F64(state.temperature_k),
        );
        out.insert("peak_sampled_temperature_k".into(), Value::F64(peak));
        out.insert(
            "max_accepted_local_error_ratio".into(),
            Value::F64(worst_estimate),
        );
        out.insert("max_energy_residual_j_kg".into(), Value::F64(worst_balance));
        Ok(Value::Record(out))
    }
}

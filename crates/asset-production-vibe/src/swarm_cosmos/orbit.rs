//! Offline gravitational trajectories and deep-space propagation diagnostics.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{count, numeric_list, scalar, unit};
use crate::{fields, list, record, required, text, vector, Args, AssetHost, TypedArtifact, Value};
use chronicle_tabular_engine::{CellRef, Value as CellValue, Workbook};
use std::{collections::BTreeMap, sync::Arc};
use vibe::{
    cosmic::celestial::{
        pulsar_dispersion_delay_s, triangulate_from_pulsars, GravitationalField, PulsarAnchor,
        C0_VACUUM_M_S,
    },
    physics::nbody_3d::{
        step_nbody_3d_verlet, total_angular_momentum, total_mechanical_energy, CelestialBody3D,
        GRAVITATIONAL_CONSTANT,
    },
};

const MAX_BODIES: usize = 32;
const MAX_STEPS: usize = 32_768;
const MAX_STATE_ROWS: usize = 16_384;

impl AssetHost<'_> {
    pub(crate) fn eval_nbody_transfer(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "bodies",
                "duration_s",
                "steps",
                "sample_every_steps",
                "softening_m",
                "integrator",
            ],
        )?;
        if args
            .get("integrator")
            .map(text)
            .transpose()?
            .unwrap_or("symplectic_verlet")
            != "symplectic_verlet"
        {
            return Err("Select symplectic_verlet for Newtonian trajectory sampling".into());
        }
        let bodies = list(required(args, "bodies")?)?;
        if !(2..=MAX_BODIES).contains(&bodies.len()) {
            return Err(format!(
                "N-body propagation requires 2 to {MAX_BODIES} bodies"
            ));
        }
        let mut states = Vec::with_capacity(bodies.len());
        for item in bodies {
            let body = record(item)?;
            fields(body, &["position_m", "velocity_m_s", "mass_kg", "radius_m"])?;
            let position = vector::<3>(required(body, "position_m")?)?;
            let velocity = vector::<3>(required(body, "velocity_m_s")?)?;
            if magnitude(position) > 1e18 || magnitude(velocity) > C0_VACUUM_M_S * 0.01 {
                return Err(
                    "Initial position or velocity lies outside the admitted Newtonian domain"
                        .into(),
                );
            }
            states.push(CelestialBody3D::new(
                position,
                velocity,
                scalar(body, "mass_kg", None, 1e-6, 1e36)?,
                scalar(body, "radius_m", Some(0.0), 0.0, 1e15)?,
            ));
        }
        let duration = scalar(args, "duration_s", None, 1e-6, 1e12)?;
        let steps = count(args, "steps", 256, 1, MAX_STEPS)?;
        let dt = duration / steps as f64;
        let cadence = count(args, "sample_every_steps", 1, 1, steps)?;
        let softening = scalar(args, "softening_m", Some(0.0), 0.0, 1e15)?;
        let sample_count = 1 + steps.div_ceil(cadence);
        let rows = sample_count * bodies.len();
        if rows > MAX_STATE_ROWS {
            return Err(
                "N-body trajectory exceeds 16384 body samples; increase sample_every_steps".into(),
            );
        }
        self.charge(
            steps * states.len() * states.len() * 144 + rows * 64,
            rows * 2048,
        )?;
        check_newtonian_step(&states, dt, softening)?;
        let initial_energy = total_mechanical_energy(&states, softening);
        let initial_momentum = total_angular_momentum(&states);
        let mut table = Workbook::new();
        let sheet = table
            .sheet_mut(0)
            .ok_or("N-body trajectory worksheet unavailable")?;
        sheet.name = "N-body Newtonian trajectory".into();
        let columns = [
            "time_s",
            "body_index",
            "x_m",
            "y_m",
            "z_m",
            "vx_m_s",
            "vy_m_s",
            "vz_m_s",
        ];
        for (column, name) in columns.into_iter().enumerate() {
            sheet.set_value(CellRef::new(0, column as u32), CellValue::text(name));
        }
        let mut row = 1;
        for step in 0..=steps {
            if step % 64 == 0 {
                self.charge(0, 0)?;
            }
            if step % cadence == 0 || step == steps {
                for (index, state) in states.iter().enumerate() {
                    let time = if step == steps {
                        duration
                    } else {
                        (step as f64) * dt
                    };
                    let [x, y, z] = state.position_m;
                    let [vx, vy, vz] = state.velocity_m_s;
                    for (column, value) in [time, index as f64, x, y, z, vx, vy, vz]
                        .into_iter()
                        .enumerate()
                    {
                        sheet.set_value(CellRef::new(row, column as u32), CellValue::Number(value));
                    }
                    row += 1;
                }
            }
            if step < steps {
                step_nbody_3d_verlet(&mut states, dt, softening, false);
                check_newtonian_step(&states, dt, softening)?;
            }
        }
        let final_energy = total_mechanical_energy(&states, softening);
        let final_momentum = total_angular_momentum(&states);
        let energy_change = final_energy - initial_energy;
        let momentum_change = subtract(final_momentum, initial_momentum);
        let report = Value::Record(BTreeMap::from([
            ("initial_energy_j".into(), Value::F64(initial_energy)),
            ("final_energy_j".into(), Value::F64(final_energy)),
            ("energy_change_j".into(), Value::F64(energy_change)),
            (
                "initial_angular_momentum_kg_m2_s".into(),
                numeric_list(initial_momentum),
            ),
            (
                "final_angular_momentum_kg_m2_s".into(),
                numeric_list(final_momentum),
            ),
            (
                "angular_momentum_change_kg_m2_s".into(),
                numeric_list(momentum_change),
            ),
            ("time_step_s".into(), Value::F64(dt)),
        ]));
        let trajectory = self.admit_artifact(TypedArtifact::TabularCube(Arc::new(table)))?;
        Ok(Value::Record(BTreeMap::from([
            ("trajectory".into(), trajectory),
            ("diagnostics".into(), report),
            (
                "integrator".into(),
                Value::String("symplectic_verlet_newtonian".into()),
            ),
        ])))
    }

    pub(crate) fn deep_space_rf_link(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "tx_position_m",
                "rx_position_m",
                "tx_velocity_m_s",
                "rx_velocity_m_s",
                "carrier_frequency_hz",
                "dispersion_measure_pc_cm3",
                "gravitating_bodies",
                "event_interval_s",
                "pulsar_anchors",
            ],
        )?;
        let tx = vector::<3>(required(args, "tx_position_m")?)?;
        let rx = vector::<3>(required(args, "rx_position_m")?)?;
        let displacement = subtract(rx, tx);
        let distance_m = magnitude(displacement);
        if magnitude(tx) > 1e23 || magnitude(rx) > 1e23 || distance_m < 1.0 {
            return Err("Deep-space link endpoints require a noncoincident Cartesian baseline of at least one metre".into());
        }
        let ray = unit(displacement, "link baseline")?;
        let carrier = scalar(args, "carrier_frequency_hz", None, 1e6, 1e15)?;
        let dm = scalar(args, "dispersion_measure_pc_cm3", Some(0.0), 0.0, 1e5)?;
        let tx_velocity = args
            .get("tx_velocity_m_s")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        let rx_velocity = args
            .get("rx_velocity_m_s")
            .map(vector::<3>)
            .transpose()?
            .unwrap_or([0.0; 3]);
        if magnitude(tx_velocity) > C0_VACUUM_M_S * 0.1
            || magnitude(rx_velocity) > C0_VACUUM_M_S * 0.1
        {
            return Err(
                "Endpoint speed must be at most 0.1 c for the admitted link diagnostics".into(),
            );
        }
        let bodies = args
            .get("gravitating_bodies")
            .map(list)
            .transpose()?
            .unwrap_or(&[]);
        if bodies.len() > MAX_BODIES {
            return Err("At most 32 static monopoles may contribute to a deep-space link".into());
        }
        self.charge(1024 + bodies.len() * 384, 4096)?;
        let mut shapiro = 0.0;
        for value in bodies {
            let body = record(value)?;
            fields(body, &["position_m", "mu_m3_s2", "radius_m"])?;
            let center = vector::<3>(required(body, "position_m")?)?;
            if magnitude(center) > 1e23 {
                return Err("Monopole origin exceeds the admitted Cartesian extent".into());
            }
            let mu = scalar(body, "mu_m3_s2", None, 1e-8, 1e27)?;
            let radius = scalar(body, "radius_m", None, 1e-6, 1e15)?;
            let relative_tx = subtract(tx, center);
            let relative_rx = subtract(rx, center);
            let along_ray = (-dot(relative_tx, ray)).clamp(0.0, distance_m);
            let closest = magnitude(std::array::from_fn(|i| relative_tx[i] + along_ray * ray[i]));
            let schw_radius = 2.0 * mu / C0_VACUUM_M_S.powi(2);
            let r_sum = magnitude(relative_tx) + magnitude(relative_rx);
            if closest <= radius.max(1000.0 * schw_radius) || r_sum - distance_m <= r_sum * 1e-14 {
                return Err("Link intersects a body, leaves the weak-field regime, or has unresolved conjunction geometry".into());
            }
            shapiro +=
                GravitationalField::newtonian(mu, radius).shapiro_delay_s(relative_tx, relative_rx);
        }
        let vacuum_s = distance_m / C0_VACUUM_M_S;
        let plasma_s = pulsar_dispersion_delay_s(dm, carrier).map_err(str::to_owned)?;
        let causal_s = vacuum_s + shapiro;
        let arrival_s = causal_s + plasma_s;
        let event_s = scalar(args, "event_interval_s", Some(arrival_s), -1e16, 1e16)?;
        let cone = if event_s.abs() < causal_s {
            "spacelike_elsewhere"
        } else if event_s < 0.0 {
            "past_causal_envelope"
        } else if plasma_s > 0.0 && event_s >= arrival_s {
            "dispersed_medium_arrival"
        } else {
            "future_causal_envelope"
        };
        let received_hz =
            carrier * frequency_factor(rx_velocity, ray) / frequency_factor(tx_velocity, ray);
        let pulsar_position = args.get("pulsar_anchors").map(pulsar_fix).transpose()?;
        Ok(Value::Record(BTreeMap::from([
            ("distance_m".into(), Value::F64(distance_m)),
            ("vacuum_light_time_s".into(), Value::F64(vacuum_s)),
            ("shapiro_delay_s".into(), Value::F64(shapiro)),
            ("cold_plasma_delay_s".into(), Value::F64(plasma_s)),
            ("causal_envelope_s".into(), Value::F64(causal_s)),
            ("signal_arrival_s".into(), Value::F64(arrival_s)),
            ("event_interval_s".into(), Value::F64(event_s)),
            ("received_frequency_hz".into(), Value::F64(received_hz)),
            (
                "kinematic_doppler_hz".into(),
                Value::F64(received_hz - carrier),
            ),
            ("causal_relation".into(), Value::String(cone.into())),
            (
                "pulsar_position_m".into(),
                pulsar_position.map(numeric_list).unwrap_or(Value::Null),
            ),
        ])))
    }
}

fn check_newtonian_step(states: &[CelestialBody3D], dt: f64, softening: f64) -> Result<(), String> {
    for (i, body) in states.iter().enumerate() {
        let speed = magnitude(body.velocity_m_s);
        let radius = magnitude(body.position_m);
        if !(radius <= 1e20 && speed <= C0_VACUUM_M_S * 0.01) {
            return Err(
                "N-body trajectory left the admitted Newtonian position or velocity domain".into(),
            );
        }
        for other in &states[i + 1..] {
            let separation = magnitude(subtract(body.position_m, other.position_m));
            if separation <= (body.radius_m + other.radius_m).max(1e-6) {
                return Err("N-body surfaces intersect at an integration sample".into());
            }
            let effective_radius = separation.hypot(softening);
            let mutual_mu = GRAVITATIONAL_CONSTANT * (body.mass_kg + other.mass_kg);
            let inverse_time = (mutual_mu / effective_radius.powi(3)).sqrt();
            if mutual_mu / effective_radius > (C0_VACUUM_M_S * 0.01).powi(2)
                || dt * inverse_time > 0.25
            {
                return Err(
                    "N-body encounter requires a weaker Newtonian field or finer integration steps"
                        .into(),
                );
            }
        }
    }
    Ok(())
}

fn pulsar_fix(value: &Value) -> Result<[f64; 3], String> {
    let items = list(value)?;
    if items.len() != 4 {
        return Err("Pulsar timing fix requires four unwrapped observations".into());
    }
    let mut anchors = [PulsarAnchor::x_ray(0, [1.0, 0.0, 0.0], 1.0, 0.0, 0.0); 4];
    let mut phase = [0.0; 4];
    for (index, value) in items.iter().enumerate() {
        let row = record(value)?;
        fields(
            row,
            &[
                "direction",
                "period_s",
                "carrier_frequency_hz",
                "dispersion_measure_pc_cm3",
                "unwrapped_phase_cycles",
            ],
        )?;
        let direction = unit(
            vector::<3>(required(row, "direction")?)?,
            "pulsar direction",
        )?;
        let period = scalar(row, "period_s", None, 1e-5, 100.0)?;
        let dm = scalar(row, "dispersion_measure_pc_cm3", Some(0.0), 0.0, 1e5)?;
        let frequency = if row.contains_key("carrier_frequency_hz") {
            scalar(row, "carrier_frequency_hz", None, 1e6, 1e15)?
        } else if dm == 0.0 {
            0.0
        } else {
            return Err("A dispersive pulsar observation requires a carrier frequency".into());
        };
        anchors[index] = PulsarAnchor {
            carrier_freq_hz: frequency,
            dispersion_measure_pc_cm3: dm,
            ..PulsarAnchor::x_ray(index as u64 + 1, direction, period, 0.0, 0.0)
        };
        phase[index] = scalar(row, "unwrapped_phase_cycles", None, -1e12, 1e12)?;
    }
    let position = triangulate_from_pulsars(&anchors, &phase).map_err(str::to_owned)?;
    if !position.iter().all(|v| v.is_finite()) {
        return Err("Pulsar timing geometry returned a nonfinite fix".into());
    }
    Ok(position)
}

fn magnitude(vector: [f64; 3]) -> f64 {
    vector[0].hypot(vector[1]).hypot(vector[2])
}

fn subtract(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}

fn frequency_factor(velocity: [f64; 3], ray: [f64; 3]) -> f64 {
    let beta = velocity.map(|v| v / C0_VACUUM_M_S);
    (1.0 - dot(beta, ray)) / (1.0 - dot(beta, beta)).sqrt()
}

//! Single-lane intelligent-driver-model trajectories.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded, numeric_row, table_receipt};
use crate::{Args, AssetHost, Value, count, fields, list, record, required};

#[derive(Clone, Copy)]
struct Vehicle {
    front: f64,
    speed: f64,
    length: f64,
    desired_speed: f64,
}

impl Vehicle {
    fn stop_time(self, acceleration: f64, dt: f64) -> f64 {
        if acceleration < 0.0 {
            (-self.speed / acceleration).min(dt)
        } else {
            dt
        }
    }

    fn position(self, acceleration: f64, time: f64) -> f64 {
        let active = self.stop_time(acceleration, time);
        self.front + (self.speed + 0.5 * acceleration * active) * active
    }
}

fn minimum_gap(leader: Vehicle, follower: Vehicle, accelerations: [f64; 2], dt: f64) -> f64 {
    let [a, b] = accelerations;
    let [stop_l, stop_f] = [leader.stop_time(a, dt), follower.stop_time(b, dt)];
    let mut breaks = [0.0, stop_l, stop_f, dt];
    breaks.sort_by(f64::total_cmp);
    let gap = |t| leader.position(a, t) - follower.position(b, t) - leader.length;
    let mut minimum = gap(0.0).min(gap(dt));
    for pair in breaks.windows(2) {
        let [u, v] = [pair[0], pair[1]];
        minimum = minimum.min(gap(v));
        if u < v {
            let acl = if u < stop_l { a } else { 0.0 };
            let acf = if u < stop_f { b } else { 0.0 };
            let relative_velocity = (leader.speed + a * u.min(stop_l)).max(0.0)
                - (follower.speed + b * u.min(stop_f)).max(0.0);
            if acl - acf > 0.0 {
                let t = u - relative_velocity / (acl - acf);
                if (u..v).contains(&t) {
                    minimum = minimum.min(gap(t));
                }
            }
        }
    }
    minimum
}

impl AssetHost<'_> {
    pub(crate) fn solve_multimodal_transit_and_crowd(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "vehicles",
                "steps",
                "dt_s",
                "time_headway_s",
                "minimum_gap_m",
                "acceleration_m_s2",
                "comfortable_braking_m_s2",
            ],
        )?;
        let incoming = list(required(args, "vehicles")?)?;
        let steps = count(args, "steps", 100, 1024)?;
        if !(1..=128).contains(&incoming.len()) || incoming.len() * (steps + 1) > 8192 {
            return Err("Traffic scenario exceeds admitted vehicle-samples".into());
        }
        let samples = incoming.len() * (steps + 1);
        self.charge(samples * 128, samples * 3000)?;
        let dt = bounded(args, "dt_s", Some(0.1), 1e-4, 0.25)?;
        let headway = bounded(args, "time_headway_s", Some(1.5), 0.2, 30.0)?;
        let gap = bounded(args, "minimum_gap_m", Some(2.0), 0.01, 50.0)?;
        let acceleration = bounded(args, "acceleration_m_s2", Some(1.4), 0.1, 10.0)?;
        let braking = bounded(args, "comfortable_braking_m_s2", Some(2.0), 0.1, 10.0)?;
        let mut vehicles = incoming
            .iter()
            .map(|value| {
                let row = record(value)?;
                fields(
                    row,
                    &["front_m", "speed_m_s", "length_m", "desired_speed_m_s"],
                )?;
                Ok(Vehicle {
                    front: bounded(row, "front_m", None, -1e7, 1e7)?,
                    speed: bounded(row, "speed_m_s", None, 0.0, 140.0)?,
                    length: bounded(row, "length_m", None, 0.1, 100.0)?,
                    desired_speed: bounded(row, "desired_speed_m_s", None, 0.1, 140.0)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut minimum = f64::INFINITY;
        for pair in vehicles.windows(2) {
            let spacing = pair[0].front - pair[0].length - pair[1].front;
            if spacing <= 0.0 {
                return Err("Vehicles must be supplied front-to-back with positive gaps".into());
            }
            minimum = minimum.min(spacing);
        }
        let mut rows = Vec::with_capacity(samples);
        let mut rates = vec![0.0; vehicles.len()];
        for step in 0..=steps {
            for (index, vehicle) in vehicles.iter().enumerate() {
                rows.push(numeric_row([
                    step as f64 * dt,
                    index as f64,
                    vehicle.front,
                    vehicle.speed,
                ]));
            }
            if step == steps {
                break;
            }
            for (index, vehicle) in vehicles.iter().enumerate() {
                let interaction = if index == 0 {
                    0.0
                } else {
                    let leader = vehicles[index - 1];
                    let current_gap = leader.front - leader.length - vehicle.front;
                    let approach = vehicle.speed - leader.speed;
                    let demand = gap
                        + (vehicle.speed * headway
                            + vehicle.speed * approach / (2.0 * (acceleration * braking).sqrt()))
                        .max(0.0);
                    (demand / current_gap).powi(2)
                };
                rates[index] = acceleration
                    * (1.0 - (vehicle.speed / vehicle.desired_speed).powi(4) - interaction);
            }
            for (index, pair) in vehicles.windows(2).enumerate() {
                let spacing = minimum_gap(pair[0], pair[1], [rates[index], rates[index + 1]], dt);
                if spacing <= 0.0 || !spacing.is_finite() {
                    return Err(format!(
                        "IDM trajectory overlap at step {step}, pair {index}, gap {spacing} m"
                    ));
                }
                minimum = minimum.min(spacing);
            }
            for (vehicle, &rate) in vehicles.iter_mut().zip(&rates) {
                vehicle.front = vehicle.position(rate, dt);
                vehicle.speed = (vehicle.speed + rate * dt).max(0.0);
            }
        }
        let mut out = table_receipt(
            self,
            "urn:qualia:civic:idm-single-lane",
            &["time_s", "vehicle_index", "front_m", "speed_m_s"],
            &rows,
        )?;
        out.insert(
            "minimum_observed_gap_m".into(),
            if minimum.is_finite() {
                Value::F64(minimum)
            } else {
                Value::Null
            },
        );
        Ok(Value::Record(out))
    }
}

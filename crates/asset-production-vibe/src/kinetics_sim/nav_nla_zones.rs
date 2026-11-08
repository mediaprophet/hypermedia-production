//! 3D NavMesh Funnel + ORCA navigation, Blender ActionChannelbag NLA strip blending,
//! and double-buffered Simulation Zone divergence-free curl flow.

use super::{dot, len, optional_point3, point3, string};
use crate::*;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// 3D NavMesh Portal Pathfinding + Simple Stupid Funnel + 3D ORCA Avoidance.
    pub(super) fn navigate_funnel_orca(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["start", "goal", "portals", "neighbors", "agent_radius", "max_speed"])?;

        let start = point3(a, "start")?;
        let goal = point3(a, "goal")?;
        let agent_radius = number(a, "agent_radius", 0.4)?;
        if !(0.05..=10.0).contains(&agent_radius) {
            return Err("agent_radius outside 0.05..=10.0".into());
        }
        let max_speed = number(a, "max_speed", 4.0)?;
        if !(0.1..=100.0).contains(&max_speed) {
            return Err("max_speed outside 0.1..=100.0".into());
        }

        self.charge(256, 1024)?;

        let portal_list = if let Some(p_val) = a.get("portals") {
            let p_raw = list(p_val)?;
            let mut list_out = Vec::with_capacity(p_raw.len());
            for item in p_raw {
                let rec = record(item)?;
                fields(rec, &["left", "right"])?;
                let left = point3(rec, "left")?;
                let right = point3(rec, "right")?;
                list_out.push(NavPortal { left, right });
            }
            list_out
        } else {
            let dx = goal[0] - start[0];
            let dy = goal[1] - start[1];
            let dz = goal[2] - start[2];
            let d_len = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-6);
            let dir = [dx / d_len, dy / d_len, dz / d_len];

            let perp = [-dir[2], 0.0, dir[0]];
            let perp_len = len(perp);
            let perp = if perp_len < 1e-6 {
                [1.0, 0.0, 0.0]
            } else {
                [perp[0] / perp_len, 0.0, perp[2] / perp_len]
            };

            let mut list_out = Vec::with_capacity(4);
            for k in 1..=4 {
                let t = k as f64 / 5.0;
                let curve = (std::f64::consts::PI * t).sin() * 1.5;
                let center = [
                    start[0] + dx * t + perp[0] * curve,
                    start[1] + dy * t + perp[1] * curve,
                    start[2] + dz * t + perp[2] * curve,
                ];
                let half_w = 2.0;
                let left = [center[0] + perp[0] * half_w, center[1], center[2] + perp[2] * half_w];
                let right = [center[0] - perp[0] * half_w, center[1], center[2] - perp[2] * half_w];
                list_out.push(NavPortal { left, right });
            }
            list_out
        };

        let mut waypoints = vec![start];
        if !portal_list.is_empty() {
            let mut apex = start;
            let mut left_pt = portal_list[0].left;
            let mut right_pt = portal_list[0].right;
            let mut left_idx = 0;
            let mut right_idx = 0;

            let mut all_portals = portal_list;
            all_portals.push(NavPortal { left: goal, right: goal });

            let mut i = 1;
            while i < all_portals.len() {
                let new_left = all_portals[i].left;
                let new_right = all_portals[i].right;

                if tri_area_2d(apex, right_pt, new_right) <= 0.0 {
                    if apex == right_pt || tri_area_2d(apex, left_pt, new_right) > 0.0 {
                        right_pt = new_right;
                        right_idx = i;
                    } else {
                        waypoints.push(left_pt);
                        apex = left_pt;
                        left_pt = apex;
                        right_pt = apex;
                        i = left_idx + 1;
                        left_idx = i;
                        right_idx = i;
                        continue;
                    }
                }

                if tri_area_2d(apex, left_pt, new_left) >= 0.0 {
                    if apex == left_pt || tri_area_2d(apex, right_pt, new_left) < 0.0 {
                        left_pt = new_left;
                        left_idx = i;
                    } else {
                        waypoints.push(right_pt);
                        apex = right_pt;
                        left_pt = apex;
                        right_pt = apex;
                        i = right_idx + 1;
                        left_idx = i;
                        right_idx = i;
                        continue;
                    }
                }

                i += 1;
            }
        }

        if waypoints.last() != Some(&goal) {
            waypoints.push(goal);
        }

        let mut string_pulled_length = 0.0;
        for w in waypoints.windows(2) {
            let dx = w[1][0] - w[0][0];
            let dy = w[1][1] - w[0][1];
            let dz = w[1][2] - w[0][2];
            string_pulled_length += (dx * dx + dy * dy + dz * dz).sqrt();
        }

        let target = if waypoints.len() > 1 { waypoints[1] } else { goal };
        let to_target = [target[0] - start[0], target[1] - start[1], target[2] - start[2]];
        let dist_target = len(to_target);
        let dir_target = if dist_target > 1e-6 {
            [to_target[0] / dist_target, to_target[1] / dist_target, to_target[2] / dist_target]
        } else {
            [0.0, 0.0, 0.0]
        };

        let pref_speed = max_speed.min(dist_target);
        let preferred_velocity = [dir_target[0] * pref_speed, dir_target[1] * pref_speed, dir_target[2] * pref_speed];

        let mut orca_safe_velocity = preferred_velocity;
        if let Some(neigh_val) = a.get("neighbors") {
            let neigh_list = list(neigh_val)?;
            let time_horizon = 2.0;

            for item in neigh_list {
                let rec = record(item)?;
                fields(rec, &["position", "velocity", "radius"])?;
                let n_pos = point3(rec, "position")?;
                let n_vel = optional_point3(rec, "velocity", [0.0, 0.0, 0.0])?;
                let n_rad = number(rec, "radius", agent_radius)?;

                let rel_pos = [n_pos[0] - start[0], n_pos[1] - start[1], n_pos[2] - start[2]];
                let rel_vel = [
                    orca_safe_velocity[0] - n_vel[0],
                    orca_safe_velocity[1] - n_vel[1],
                    orca_safe_velocity[2] - n_vel[2],
                ];

                let combined_rad = agent_radius + n_rad;
                let cutoff_center = [rel_pos[0] / time_horizon, rel_pos[1] / time_horizon, rel_pos[2] / time_horizon];
                let cutoff_rad = combined_rad / time_horizon;

                let w = [
                    rel_vel[0] - cutoff_center[0],
                    rel_vel[1] - cutoff_center[1],
                    rel_vel[2] - cutoff_center[2],
                ];
                let w_len = len(w);

                if w_len < cutoff_rad && dot(rel_vel, rel_pos) > 0.0 {
                    let u = if w_len > 1e-9 {
                        let factor = (cutoff_rad - w_len) / w_len;
                        [w[0] * factor, w[1] * factor, w[2] * factor]
                    } else {
                        [0.0, cutoff_rad, 0.0]
                    };

                    orca_safe_velocity[0] += u[0] * 0.5;
                    orca_safe_velocity[1] += u[1] * 0.5;
                    orca_safe_velocity[2] += u[2] * 0.5;
                }
            }

            let cur_speed = len(orca_safe_velocity);
            if cur_speed > max_speed {
                let factor = max_speed / cur_speed;
                orca_safe_velocity[0] *= factor;
                orca_safe_velocity[1] *= factor;
                orca_safe_velocity[2] *= factor;
            }
        }

        let waypoints_val: Vec<Value> = waypoints
            .iter()
            .map(|p| Value::List(vec![Value::F64(p[0]), Value::F64(p[1]), Value::F64(p[2])]))
            .collect();

        let mut out = BTreeMap::new();
        out.insert("waypoints".into(), Value::List(waypoints_val));
        out.insert("string_pulled_length".into(), Value::F64(string_pulled_length));
        out.insert(
            "preferred_velocity".into(),
            Value::List(vec![
                Value::F64(preferred_velocity[0]),
                Value::F64(preferred_velocity[1]),
                Value::F64(preferred_velocity[2]),
            ]),
        );
        out.insert(
            "orca_safe_velocity".into(),
            Value::List(vec![
                Value::F64(orca_safe_velocity[0]),
                Value::F64(orca_safe_velocity[1]),
                Value::F64(orca_safe_velocity[2]),
            ]),
        );
        out.insert("waypoint_count".into(), Value::I64(waypoints.len() as i64));

        Ok(Value::Record(out))
    }

    /// Blender ActionChannelbag NLA Strip Blending + Godot LensTweenTable evaluation.
    pub(super) fn evaluate_nla_channelbag(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["strips", "eval_time"])?;

        let raw_strips = list(required(a, "strips")?)?;
        let eval_time = num(required(a, "eval_time")?)?;

        self.charge(raw_strips.len() * 16, 512)?;

        let mut acc = [0.0, 0.0, 0.0];
        let mut active_strip_count: usize = 0;
        let mut is_first_active = true;

        for s_val in raw_strips {
            let rec = record(s_val)?;
            fields(
                rec,
                &["name", "start_time", "end_time", "weight", "blend_mode", "start_val", "end_val", "transition"],
            )?;

            let start_time = number(rec, "start_time", 0.0)?;
            let end_time = number(rec, "end_time", 1.0)?;
            let weight = number(rec, "weight", 1.0)?.clamp(0.0, 1.0);
            let blend_mode = string(rec, "blend_mode", "replace")?;
            let start_val = point3(rec, "start_val")?;
            let end_val = point3(rec, "end_val")?;
            let transition = string(rec, "transition", "linear")?;

            if eval_time >= start_time && eval_time <= end_time {
                active_strip_count += 1;
                let duration = end_time - start_time;
                let t = if duration > 1e-9 {
                    ((eval_time - start_time) / duration).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                let eased = evaluate_tween(t, transition);
                let v_strip = [
                    start_val[0] + (end_val[0] - start_val[0]) * eased,
                    start_val[1] + (end_val[1] - start_val[1]) * eased,
                    start_val[2] + (end_val[2] - start_val[2]) * eased,
                ];

                if is_first_active {
                    acc = [v_strip[0] * weight, v_strip[1] * weight, v_strip[2] * weight];
                    is_first_active = false;
                } else {
                    match blend_mode {
                        "replace" => {
                            acc[0] = acc[0] * (1.0 - weight) + v_strip[0] * weight;
                            acc[1] = acc[1] * (1.0 - weight) + v_strip[1] * weight;
                            acc[2] = acc[2] * (1.0 - weight) + v_strip[2] * weight;
                        }
                        "add" => {
                            acc[0] += v_strip[0] * weight;
                            acc[1] += v_strip[1] * weight;
                            acc[2] += v_strip[2] * weight;
                        }
                        "multiply" => {
                            acc[0] = acc[0] * (1.0 - weight) + (acc[0] * v_strip[0]) * weight;
                            acc[1] = acc[1] * (1.0 - weight) + (acc[1] * v_strip[1]) * weight;
                            acc[2] = acc[2] * (1.0 - weight) + (acc[2] * v_strip[2]) * weight;
                        }
                        _ => return Err(format!("Unknown blend_mode: {blend_mode}")),
                    }
                }
            }
        }

        let mut out = BTreeMap::new();
        out.insert("evaluated_value".into(), Value::List(vec![Value::F64(acc[0]), Value::F64(acc[1]), Value::F64(acc[2])]));
        out.insert("active_strip_count".into(), Value::I64(active_strip_count as i64));
        out.insert("eval_time".into(), Value::F64(eval_time));

        Ok(Value::Record(out))
    }

    /// Double-buffered Simulation / Repeat Zone iterative divergence-free curl flow.
    pub(super) fn step_simulation_zone(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["points", "iterations", "dt", "curl_strength", "attractor"])?;

        let raw_pts = list(required(a, "points")?)?;
        let iterations = count(a, "iterations", 8, 256)?;
        let dt = number(a, "dt", 0.05)?;
        if !(0.001..=1.0).contains(&dt) {
            return Err("dt outside 0.001..=1.0".into());
        }

        let curl_strength = number(a, "curl_strength", 0.5)?;
        if !(-10.0..=10.0).contains(&curl_strength) {
            return Err("curl_strength outside -10.0..=10.0".into());
        }

        let attractor = optional_point3(a, "attractor", [0.0, 0.0, 0.0])?;

        self.charge(raw_pts.len() * iterations * 8, 4096)?;

        let initial_pts: Vec<[f64; 3]> = raw_pts.iter().map(vector::<3>).collect::<Result<Vec<_>, _>>()?;

        let n = initial_pts.len();
        let mut state_a = initial_pts.clone();
        let mut state_b = vec![[0.0, 0.0, 0.0]; n];
        let mut a_is_src = true;

        for _iter in 0..iterations {
            let (src, dst) = if a_is_src {
                (&state_a, &mut state_b)
            } else {
                (&state_b, &mut state_a)
            };

            for i in 0..n {
                let p = src[i];
                let vx_curl = -p[1].sin() - p[2].cos();
                let vy_curl = -p[2].sin() - p[0].cos();
                let vz_curl = -p[0].sin() - p[1].cos();

                let ax = attractor[0] - p[0];
                let ay = attractor[1] - p[1];
                let az = attractor[2] - p[2];
                let d = (ax * ax + ay * ay + az * az).sqrt();
                let (vx_attr, vy_attr, vz_attr) = if d > 1e-6 {
                    let speed = d.min(2.0);
                    (ax / d * speed, ay / d * speed, az / d * speed)
                } else {
                    (0.0, 0.0, 0.0)
                };

                let vx = vx_curl * curl_strength + vx_attr * 0.5;
                let vy = vy_curl * curl_strength + vy_attr * 0.5;
                let vz = vz_curl * curl_strength + vz_attr * 0.5;

                dst[i] = [p[0] + vx * dt, p[1] + vy * dt, p[2] + vz * dt];
            }
            a_is_src = !a_is_src;
        }

        let final_pts = if a_is_src { &state_a } else { &state_b };

        let mut total_disp = 0.0;
        for i in 0..n {
            let dx = final_pts[i][0] - initial_pts[i][0];
            let dy = final_pts[i][1] - initial_pts[i][1];
            let dz = final_pts[i][2] - initial_pts[i][2];
            total_disp += (dx * dx + dy * dy + dz * dz).sqrt();
        }

        let mean_displacement = if n > 0 { total_disp / n as f64 } else { 0.0 };

        let pts_val: Vec<Value> = final_pts
            .iter()
            .map(|p| Value::List(vec![Value::F64(p[0]), Value::F64(p[1]), Value::F64(p[2])]))
            .collect();

        let mut out = BTreeMap::new();
        out.insert("points".into(), Value::List(pts_val));
        out.insert("iterations".into(), Value::I64(iterations as i64));
        out.insert("mean_displacement".into(), Value::F64(mean_displacement));
        out.insert("point_count".into(), Value::I64(n as i64));

        Ok(Value::Record(out))
    }
}

struct NavPortal {
    left: [f64; 3],
    right: [f64; 3],
}

fn tri_area_2d(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> f64 {
    (b[0] - a[0]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[0] - a[0])
}

fn evaluate_tween(t: f64, trans: &str) -> f64 {
    let t = t.clamp(0.0, 1.0);
    const PI: f64 = std::f64::consts::PI;
    match trans {
        "sine" => (t * PI * 0.5).sin(),
        "quad" => t * t,
        "cubic" => t * t * t,
        "quart" => t * t * t * t,
        "quint" => t * t * t * t * t,
        "expo" => {
            if t <= 0.0 {
                0.0
            } else {
                (2.0f64).powf(10.0 * (t - 1.0))
            }
        }
        "circ" => 1.0 - (1.0 - t * t).max(0.0).sqrt(),
        "back" => {
            let s = 1.70158;
            t * t * ((s + 1.0) * t - s)
        }
        "spring" | "elastic" => {
            if t <= 0.0 {
                0.0
            } else if t >= 1.0 {
                1.0
            } else {
                let p = 0.3;
                let s = p / 4.0;
                -(2.0f64).powf(10.0 * (t - 1.0)) * ((t - 1.0 - s) * (2.0 * PI) / p).sin()
            }
        }
        "bounce" => {
            let t = 1.0 - t;
            let b = if t < 1.0 / 2.75 {
                7.5625 * t * t
            } else if t < 2.0 / 2.75 {
                let t_adj = t - 1.5 / 2.75;
                7.5625 * t_adj * t_adj + 0.75
            } else if t < 2.5 / 2.75 {
                let t_adj = t - 2.25 / 2.75;
                7.5625 * t_adj * t_adj + 0.9375
            } else {
                let t_adj = t - 2.625 / 2.75;
                7.5625 * t_adj * t_adj + 0.984375
            };
            1.0 - b
        }
        _ => t,
    }
}

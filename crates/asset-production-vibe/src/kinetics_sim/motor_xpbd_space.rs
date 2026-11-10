//! PGA MotorFTI ScLERP interpolation, Island-Parallel XPBD rigid-body solver,
//! and DirectSpaceState BVH ray/motion queries.

use super::{len, normalize_quat, optional_point3, optional_quat4, point3, string, Dsu};
use crate::*;
use qualia_core_db::etalon_render::skeletal::Motor as PgaMotor;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    /// PGA MotorFTI Screw-Linear Interpolation (ScLERP) between physics ticks and render frames.
    pub(super) fn interpolate_motor_fti(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &["prev_position", "prev_rotation", "curr_position", "curr_rotation", "alpha", "teleport"],
        )?;

        let prev_pos = optional_point3(a, "prev_position", [0.0, 0.0, 0.0])?;
        let prev_rot = optional_quat4(a, "prev_rotation", [0.0, 0.0, 0.0, 1.0])?;
        let curr_pos = point3(a, "curr_position")?;
        let curr_rot = optional_quat4(a, "curr_rotation", [0.0, 0.0, 0.0, 1.0])?;

        let alpha = number(a, "alpha", 0.5)?;
        if !(0.0..=1.0).contains(&alpha) {
            return Err("alpha outside 0.0..=1.0".into());
        }

        let teleport = boolean(a, "teleport", false)?;
        self.charge(64, 128)?;

        let norm_prev_rot = normalize_quat(prev_rot);
        let norm_curr_rot = normalize_quat(curr_rot);

        let rot_prev_wxyz = [norm_prev_rot[3], norm_prev_rot[0], norm_prev_rot[1], norm_prev_rot[2]];
        let rot_curr_wxyz = [norm_curr_rot[3], norm_curr_rot[0], norm_curr_rot[1], norm_curr_rot[2]];

        let m_prev = PgaMotor::from_rotation_translation(rot_prev_wxyz, prev_pos);
        let m_curr = PgaMotor::from_rotation_translation(rot_curr_wxyz, curr_pos);

        let (res_motor, res_pos, res_rot) = if teleport {
            let m_snap = m_curr.normalize();
            let (rot_wxyz, trans) = m_snap.to_rotation_translation();
            let rot_xyzw = [rot_wxyz[1], rot_wxyz[2], rot_wxyz[3], rot_wxyz[0]];
            (m_snap, trans, rot_xyzw)
        } else {
            let m_interp = PgaMotor::sclerp(&m_prev, &m_curr, alpha).normalize();
            let (rot_wxyz, trans) = m_interp.to_rotation_translation();
            let rot_xyzw = [rot_wxyz[1], rot_wxyz[2], rot_wxyz[3], rot_wxyz[0]];
            (m_interp, trans, rot_xyzw)
        };

        let mut out = BTreeMap::new();
        out.insert("position".into(), Value::List(vec![Value::F64(res_pos[0]), Value::F64(res_pos[1]), Value::F64(res_pos[2])]));
        out.insert(
            "rotation".into(),
            Value::List(vec![Value::F64(res_rot[0]), Value::F64(res_rot[1]), Value::F64(res_rot[2]), Value::F64(res_rot[3])]),
        );
        out.insert(
            "motor_8f".into(),
            Value::List(vec![
                Value::F64(res_motor.r_w),
                Value::F64(res_motor.r_x),
                Value::F64(res_motor.r_y),
                Value::F64(res_motor.r_z),
                Value::F64(res_motor.d_w),
                Value::F64(res_motor.d_x),
                Value::F64(res_motor.d_y),
                Value::F64(res_motor.d_z),
            ]),
        );
        out.insert("alpha".into(), Value::F64(alpha));
        out.insert("teleported".into(), Value::Bool(teleport));

        Ok(Value::Record(out))
    }

    /// ID-sorted deterministic Island-Parallel XPBD rigid-body simulation.
    pub(super) fn step_island_xpbd(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["bodies", "area_override", "dt", "steps", "substeps", "ground_y"])?;

        let raw_bodies = list(required(a, "bodies")?)?;
        if raw_bodies.is_empty() || raw_bodies.len() > 128 {
            return Err("bodies count outside 1..128".into());
        }

        let dt = number(a, "dt", 0.015625)?;
        if !(1e-4..=0.25).contains(&dt) {
            return Err("dt outside 1e-4..=0.25".into());
        }

        let steps = count(a, "steps", 4, 120)?;
        let substeps = count(a, "substeps", 4, 16)?;
        let ground_y = number(a, "ground_y", 0.0)?;

        let area_override = if let Some(area_val) = a.get("area_override") {
            let area_rec = record(area_val)?;
            fields(area_rec, &["bounds_min", "bounds_max", "gravity", "linear_damping", "wind"])?;
            let bounds_min = point3(area_rec, "bounds_min")?;
            let bounds_max = point3(area_rec, "bounds_max")?;
            let gravity = optional_point3(area_rec, "gravity", [0.0, -9.81, 0.0])?;
            let linear_damping = number(area_rec, "linear_damping", 0.01)?.clamp(0.0, 1.0);
            let wind = optional_point3(area_rec, "wind", [0.0, 0.0, 0.0])?;
            Some(AreaOverride { bounds_min, bounds_max, gravity, linear_damping, wind })
        } else {
            None
        };

        self.charge(128 * steps * substeps, 2048)?;

        let mut bodies = Vec::with_capacity(raw_bodies.len());
        for b_val in raw_bodies {
            let b_rec = record(b_val)?;
            fields(b_rec, &["id", "position", "velocity", "radius", "inv_mass"])?;
            let id = text(required(b_rec, "id")?)?.to_string();
            let position = point3(b_rec, "position")?;
            let velocity = optional_point3(b_rec, "velocity", [0.0, 0.0, 0.0])?;
            let radius = number(b_rec, "radius", 0.5)?;
            if radius <= 0.0 {
                return Err("radius must be positive".into());
            }
            let inv_mass = number(b_rec, "inv_mass", 1.0)?;
            if inv_mass < 0.0 {
                return Err("inv_mass must be non-negative".into());
            }
            bodies.push(XpbdBody { id, position, velocity, radius, inv_mass });
        }

        bodies.sort_by(|a, b| a.id.cmp(&b.id));

        let n = bodies.len();
        let mut dsu = Dsu::new(n);
        let mut contact_count: usize = 0;
        let mut prev_positions = vec![[0.0, 0.0, 0.0]; n];
        let h = dt / substeps as f64;

        for _step in 0..steps {
            for _sub in 0..substeps {
                for (i, b) in bodies.iter_mut().enumerate() {
                    if b.inv_mass > 0.0 {
                        let in_area = if let Some(area) = &area_override {
                            b.position[0] >= area.bounds_min[0]
                                && b.position[0] <= area.bounds_max[0]
                                && b.position[1] >= area.bounds_min[1]
                                && b.position[1] <= area.bounds_max[1]
                                && b.position[2] >= area.bounds_min[2]
                                && b.position[2] <= area.bounds_max[2]
                        } else {
                            false
                        };

                        let (grav, damp, wind) = if in_area {
                            let a = area_override.as_ref().unwrap();
                            (a.gravity, a.linear_damping, a.wind)
                        } else {
                            ([0.0, -9.81, 0.0], 0.01, [0.0, 0.0, 0.0])
                        };

                        b.velocity[0] += grav[0] * h;
                        b.velocity[1] += grav[1] * h;
                        b.velocity[2] += grav[2] * h;

                        let drag = (damp * h).clamp(0.0, 1.0);
                        b.velocity[0] += (wind[0] - b.velocity[0]) * drag;
                        b.velocity[1] += (wind[1] - b.velocity[1]) * drag;
                        b.velocity[2] += (wind[2] - b.velocity[2]) * drag;

                        prev_positions[i] = b.position;
                        b.position[0] += b.velocity[0] * h;
                        b.position[1] += b.velocity[1] * h;
                        b.position[2] += b.velocity[2] * h;
                    } else {
                        prev_positions[i] = b.position;
                    }
                }

                for b in bodies.iter_mut() {
                    if b.inv_mass > 0.0 {
                        let min_y = ground_y + b.radius;
                        if b.position[1] < min_y {
                            b.position[1] = min_y;
                            contact_count += 1;
                        }
                    }
                }

                for i in 0..n {
                    for j in (i + 1)..n {
                        let dx = bodies[j].position[0] - bodies[i].position[0];
                        let dy = bodies[j].position[1] - bodies[i].position[1];
                        let dz = bodies[j].position[2] - bodies[i].position[2];
                        let dist_sq = dx * dx + dy * dy + dz * dz;
                        let r_sum = bodies[i].radius + bodies[j].radius;

                        if dist_sq <= (r_sum + 0.05) * (r_sum + 0.05) {
                            dsu.union(i, j);
                        }

                        if dist_sq < r_sum * r_sum {
                            contact_count += 1;
                            let dist = dist_sq.sqrt();
                            let (nx, ny, nz) = if dist > 1e-9 {
                                (dx / dist, dy / dist, dz / dist)
                            } else {
                                (0.0, 1.0, 0.0)
                            };

                            let penetration = r_sum - dist;
                            let w = bodies[i].inv_mass + bodies[j].inv_mass;
                            if w > 0.0 {
                                let delta_lambda = penetration / w;
                                if bodies[i].inv_mass > 0.0 {
                                    let factor = delta_lambda * bodies[i].inv_mass;
                                    bodies[i].position[0] -= nx * factor;
                                    bodies[i].position[1] -= ny * factor;
                                    bodies[i].position[2] -= nz * factor;
                                }
                                if bodies[j].inv_mass > 0.0 {
                                    let factor = delta_lambda * bodies[j].inv_mass;
                                    bodies[j].position[0] += nx * factor;
                                    bodies[j].position[1] += ny * factor;
                                    bodies[j].position[2] += nz * factor;
                                }
                            }
                        }
                    }
                }

                for (i, b) in bodies.iter_mut().enumerate() {
                    if b.inv_mass > 0.0 {
                        b.velocity[0] = (b.position[0] - prev_positions[i][0]) / h;
                        b.velocity[1] = (b.position[1] - prev_positions[i][1]) / h;
                        b.velocity[2] = (b.position[2] - prev_positions[i][2]) / h;

                        if (b.position[1] - (ground_y + b.radius)).abs() < 1e-4 && b.velocity[1] < 0.0 {
                            b.velocity[1] = 0.0;
                            b.velocity[0] *= 0.95;
                            b.velocity[2] *= 0.95;
                        }
                    }
                }
            }
        }

        let mut sleeping_count: usize = 0;
        for b in &bodies {
            if b.inv_mass > 0.0 {
                let speed_sq = b.velocity[0] * b.velocity[0] + b.velocity[1] * b.velocity[1] + b.velocity[2] * b.velocity[2];
                if speed_sq < 0.04 && (b.position[1] - (ground_y + b.radius)).abs() < 0.05 {
                    sleeping_count += 1;
                }
            }
        }

        let body_values: Vec<Value> = bodies
            .iter()
            .map(|b| {
                let mut rec = BTreeMap::new();
                rec.insert("id".into(), Value::String(b.id.clone()));
                rec.insert("position".into(), Value::List(vec![Value::F64(b.position[0]), Value::F64(b.position[1]), Value::F64(b.position[2])]));
                rec.insert("velocity".into(), Value::List(vec![Value::F64(b.velocity[0]), Value::F64(b.velocity[1]), Value::F64(b.velocity[2])]));
                rec.insert("radius".into(), Value::F64(b.radius));
                rec.insert("inv_mass".into(), Value::F64(b.inv_mass));
                Value::Record(rec)
            })
            .collect();

        let mut out = BTreeMap::new();
        out.insert("island_count".into(), Value::I64(dsu.count_islands() as i64));
        out.insert("contact_count".into(), Value::I64(contact_count as i64));
        out.insert("sleeping_count".into(), Value::I64(sleeping_count as i64));
        out.insert("bodies".into(), Value::List(body_values));

        Ok(Value::Record(out))
    }

    /// Synchronous DirectSpaceState spatial queries over split Static/Dynamic BVH.
    pub(super) fn query_direct_space(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &["colliders", "query_mode", "ray_origin", "ray_dir", "motion_delta", "probe_radius", "collision_mask"],
        )?;

        let raw_colliders = list(required(a, "colliders")?)?;
        let query_mode = string(a, "query_mode", "intersect_ray")?;
        let ray_origin = optional_point3(a, "ray_origin", [0.0, 1.0, -5.0])?;
        let ray_dir = optional_point3(a, "ray_dir", [0.0, 0.0, 1.0])?;
        let motion_delta = optional_point3(a, "motion_delta", [0.0, 0.0, 4.0])?;
        let probe_radius = number(a, "probe_radius", 0.25)?;
        if !(0.0..=50.0).contains(&probe_radius) {
            return Err("probe_radius outside 0.0..=50.0".into());
        }

        let collision_mask = match a.get("collision_mask") {
            Some(v) => integer(v, "collision_mask")? as u64,
            None => 0xFFFF_FFFFu64,
        };

        self.charge(raw_colliders.len() * 32, 1024)?;

        let mut static_colliders = Vec::new();
        let mut dynamic_colliders = Vec::new();

        for c_val in raw_colliders {
            let c_rec = record(c_val)?;
            fields(c_rec, &["id", "center", "half_extents", "layer_mask", "is_static"])?;
            let id = text(required(c_rec, "id")?)?.to_string();
            let center = point3(c_rec, "center")?;
            let half_extents = point3(c_rec, "half_extents")?;
            let layer_mask = match c_rec.get("layer_mask") {
                Some(v) => integer(v, "layer_mask")? as u64,
                None => 1u64,
            };
            let is_static = boolean(c_rec, "is_static", true)?;

            if (layer_mask & collision_mask) != 0 {
                let col = SpatialCollider { id, center, half_extents };
                if is_static {
                    static_colliders.push(col);
                } else {
                    dynamic_colliders.push(col);
                }
            }
        }

        let static_leaves = static_colliders.len();
        let dynamic_leaves = dynamic_colliders.len();
        let all_colliders = [static_colliders, dynamic_colliders].concat();

        match query_mode {
            "intersect_ray" => {
                let dir_len = len(ray_dir);
                let dir = if dir_len > 1e-9 {
                    [ray_dir[0] / dir_len, ray_dir[1] / dir_len, ray_dir[2] / dir_len]
                } else {
                    [0.0, 0.0, 1.0]
                };

                let mut closest_t = f64::INFINITY;
                let mut best_hit_id = String::new();
                let mut best_normal = [0.0, 1.0, 0.0];

                for col in &all_colliders {
                    if let Some((t, normal)) = ray_intersect_aabb(ray_origin, dir, col) {
                        if t < closest_t {
                            closest_t = t;
                            best_hit_id = col.id.clone();
                            best_normal = normal;
                        }
                    }
                }

                let mut out = BTreeMap::new();
                if closest_t.is_finite() {
                    let hit_point = [
                        ray_origin[0] + dir[0] * closest_t,
                        ray_origin[1] + dir[1] * closest_t,
                        ray_origin[2] + dir[2] * closest_t,
                    ];
                    out.insert("hit".into(), Value::Bool(true));
                    out.insert("hit_id".into(), Value::String(best_hit_id));
                    out.insert("hit_point".into(), Value::List(vec![Value::F64(hit_point[0]), Value::F64(hit_point[1]), Value::F64(hit_point[2])]));
                    out.insert("hit_normal".into(), Value::List(vec![Value::F64(best_normal[0]), Value::F64(best_normal[1]), Value::F64(best_normal[2])]));
                    out.insert("safe_fraction".into(), Value::F64(closest_t));
                    out.insert("unsafe_fraction".into(), Value::F64(closest_t));
                } else {
                    out.insert("hit".into(), Value::Bool(false));
                    out.insert("hit_id".into(), Value::String("".into()));
                    out.insert("hit_point".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
                    out.insert("hit_normal".into(), Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]));
                    out.insert("safe_fraction".into(), Value::F64(1.0));
                    out.insert("unsafe_fraction".into(), Value::F64(1.0));
                }

                out.insert("static_leaves".into(), Value::I64(static_leaves as i64));
                out.insert("dynamic_leaves".into(), Value::I64(dynamic_leaves as i64));

                Ok(Value::Record(out))
            }

            "cast_motion" | "body_test_motion" => {
                let mut closest_s = f64::INFINITY;
                let mut best_hit_id = String::new();
                let mut best_normal = [0.0, 1.0, 0.0];

                for col in &all_colliders {
                    if let Some((s, normal)) = sweep_sphere_aabb(ray_origin, motion_delta, probe_radius, col) {
                        if s < closest_s {
                            closest_s = s;
                            best_hit_id = col.id.clone();
                            best_normal = normal;
                        }
                    }
                }

                let mut out = BTreeMap::new();
                if closest_s.is_finite() && closest_s <= 1.0 {
                    let s_clamped = closest_s.clamp(0.0, 1.0);
                    let safe_fraction = s_clamped;
                    let unsafe_fraction = (s_clamped + 0.001).min(1.0);

                    let hit_point = [
                        ray_origin[0] + motion_delta[0] * s_clamped - best_normal[0] * probe_radius,
                        ray_origin[1] + motion_delta[1] * s_clamped - best_normal[1] * probe_radius,
                        ray_origin[2] + motion_delta[2] * s_clamped - best_normal[2] * probe_radius,
                    ];

                    out.insert("hit".into(), Value::Bool(true));
                    out.insert("hit_id".into(), Value::String(best_hit_id));
                    out.insert("hit_point".into(), Value::List(vec![Value::F64(hit_point[0]), Value::F64(hit_point[1]), Value::F64(hit_point[2])]));
                    out.insert("hit_normal".into(), Value::List(vec![Value::F64(best_normal[0]), Value::F64(best_normal[1]), Value::F64(best_normal[2])]));
                    out.insert("safe_fraction".into(), Value::F64(safe_fraction));
                    out.insert("unsafe_fraction".into(), Value::F64(unsafe_fraction));
                } else {
                    out.insert("hit".into(), Value::Bool(false));
                    out.insert("hit_id".into(), Value::String("".into()));
                    out.insert("hit_point".into(), Value::List(vec![Value::F64(0.0), Value::F64(0.0), Value::F64(0.0)]));
                    out.insert("hit_normal".into(), Value::List(vec![Value::F64(0.0), Value::F64(1.0), Value::F64(0.0)]));
                    out.insert("safe_fraction".into(), Value::F64(1.0));
                    out.insert("unsafe_fraction".into(), Value::F64(1.0));
                }

                out.insert("static_leaves".into(), Value::I64(static_leaves as i64));
                out.insert("dynamic_leaves".into(), Value::I64(dynamic_leaves as i64));

                Ok(Value::Record(out))
            }

            _ => Err(format!("Unknown query_mode: {query_mode}")),
        }
    }
}

struct XpbdBody {
    id: String,
    position: [f64; 3],
    velocity: [f64; 3],
    radius: f64,
    inv_mass: f64,
}

struct AreaOverride {
    bounds_min: [f64; 3],
    bounds_max: [f64; 3],
    gravity: [f64; 3],
    linear_damping: f64,
    wind: [f64; 3],
}

#[derive(Clone)]
struct SpatialCollider {
    id: String,
    center: [f64; 3],
    half_extents: [f64; 3],
}

fn ray_intersect_aabb(origin: [f64; 3], dir: [f64; 3], col: &SpatialCollider) -> Option<(f64, [f64; 3])> {
    let mut t_near = 0.0f64;
    let mut t_far = f64::INFINITY;
    let mut normal_near = [0.0, 1.0, 0.0];

    for k in 0..3 {
        let b_min = col.center[k] - col.half_extents[k];
        let b_max = col.center[k] + col.half_extents[k];

        if dir[k].abs() < 1e-9 {
            if origin[k] < b_min || origin[k] > b_max {
                return None;
            }
        } else {
            let inv_d = 1.0 / dir[k];
            let mut t0 = (b_min - origin[k]) * inv_d;
            let mut t1 = (b_max - origin[k]) * inv_d;
            let mut n0 = [0.0; 3];
            n0[k] = -1.0;
            let mut n1 = [0.0; 3];
            n1[k] = 1.0;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
                std::mem::swap(&mut n0, &mut n1);
            }

            if t0 > t_near {
                t_near = t0;
                normal_near = n0;
            }
            if t1 < t_far {
                t_far = t1;
            }
            if t_near > t_far {
                return None;
            }
        }
    }

    if t_far >= 0.0 {
        Some((t_near.max(0.0), normal_near))
    } else {
        None
    }
}

fn sweep_sphere_aabb(origin: [f64; 3], delta: [f64; 3], radius: f64, col: &SpatialCollider) -> Option<(f64, [f64; 3])> {
    let mut s_near = 0.0f64;
    let mut s_far = 1.0f64;
    let mut normal_near = [0.0, 1.0, 0.0];

    for k in 0..3 {
        let b_min = col.center[k] - (col.half_extents[k] + radius);
        let b_max = col.center[k] + (col.half_extents[k] + radius);

        if delta[k].abs() < 1e-9 {
            if origin[k] < b_min || origin[k] > b_max {
                return None;
            }
        } else {
            let inv_m = 1.0 / delta[k];
            let mut s0 = (b_min - origin[k]) * inv_m;
            let mut s1 = (b_max - origin[k]) * inv_m;
            let mut n0 = [0.0; 3];
            n0[k] = -1.0;
            let mut n1 = [0.0; 3];
            n1[k] = 1.0;

            if inv_m < 0.0 {
                std::mem::swap(&mut s0, &mut s1);
                std::mem::swap(&mut n0, &mut n1);
            }

            if s0 > s_near {
                s_near = s0;
                normal_near = n0;
            }
            if s1 < s_far {
                s_far = s1;
            }
            if s_near > s_far {
                return None;
            }
        }
    }

    if s_far >= 0.0 && s_near <= 1.0 {
        Some((s_near.max(0.0), normal_near))
    } else {
        None
    }
}

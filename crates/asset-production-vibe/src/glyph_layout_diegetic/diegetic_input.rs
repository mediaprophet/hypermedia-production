//! Bidirectional diegetic surface 3D quad projection and 4-stage fiduciary input routing.

use crate::*;
use std::collections::BTreeMap;
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn project_diegetic_surface(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "ui_image",
                "viewport_width",
                "viewport_height",
                "yaw_deg",
                "pitch_deg",
                "pick_screen_xy",
            ],
        )?;

        let ui_val = required(a, "ui_image")?;
        let ui_rgba = self.rgba(ui_val)?.clone();
        let ui_w = ui_rgba.width;
        let ui_h = ui_rgba.height;
        if ui_w == 0 || ui_h == 0 {
            return Err("ui_image has zero dimension".into());
        }

        let vp_w = count(a, "viewport_width", 256, 1024)?;
        let vp_h = count(a, "viewport_height", 256, 1024)?;

        let yaw = number(a, "yaw_deg", 18.0)?;
        if !(-75.0..=75.0).contains(&yaw) {
            return Err("yaw_deg outside -75.0..=75.0".into());
        }

        let pitch = number(a, "pitch_deg", -10.0)?;
        if !(-75.0..=75.0).contains(&pitch) {
            return Err("pitch_deg outside -75.0..=75.0".into());
        }

        let pick_screen_xy = match a.get("pick_screen_xy") {
            Some(v) => vector::<2>(v)?,
            None => [128.0, 128.0],
        };

        let total_px = vp_w * vp_h;
        self.charge(
            total_px * 8 + (ui_w * ui_h) as usize * 2,
            total_px * 16 + 8192,
        )?;

        let f = (vp_h as f64) * 0.86602540378;
        let aspect = (ui_w as f64) / (ui_h as f64);
        let max_dim = aspect.max(1.0);
        let qw = 1.6 * (aspect / max_dim);
        let qh = 1.6 / max_dim;
        let center = [0.0, 0.0, 2.0];

        let yaw_rad = yaw * std::f64::consts::PI / 180.0;
        let pitch_rad = pitch * std::f64::consts::PI / 180.0;
        let cy = yaw_rad.cos();
        let sy = yaw_rad.sin();
        let cx = pitch_rad.cos();
        let sx = pitch_rad.sin();

        let normal = [-sy * cx, sx, -cy * cx];
        let c_dot_n = center[0] * normal[0] + center[1] * normal[1] + center[2] * normal[2];

        let mut pixels = vec![[0u8, 0, 0, 0]; total_px];

        for y in 0..vp_h {
            let ry = (vp_h as f64 * 0.5) - (y as f64 + 0.5);
            for x in 0..vp_w {
                let rx = (x as f64 + 0.5) - (vp_w as f64 * 0.5);
                let rz = f;

                let d_dot_n = rx * normal[0] + ry * normal[1] + rz * normal[2];
                if d_dot_n.abs() < 1e-8 {
                    continue;
                }
                let t = c_dot_n / d_dot_n;
                if t <= 0.0 {
                    continue;
                }

                let px = t * rx;
                let py = t * ry;
                let pz = t * rz;

                let dx = px - center[0];
                let dy = py - center[1];
                let dz = pz - center[2];

                let loc_x = cy * dx - sy * dz;
                let loc_y = sy * sx * dx + cx * dy + cy * sx * dz;

                let u = loc_x / qw + 0.5;
                let v = 0.5 - loc_y / qh;

                if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
                    let fx = u * (ui_w - 1) as f64;
                    let fy = v * (ui_h - 1) as f64;
                    let x0 = fx.floor() as usize;
                    let y0 = fy.floor() as usize;
                    let x1 = (x0 + 1).min(ui_w as usize - 1);
                    let y1 = (y0 + 1).min(ui_h as usize - 1);
                    let wx = fx - x0 as f64;
                    let wy = fy - y0 as f64;

                    let c00 = ui_rgba.pixels[y0 * ui_w as usize + x0];
                    let c10 = ui_rgba.pixels[y0 * ui_w as usize + x1];
                    let c01 = ui_rgba.pixels[y1 * ui_w as usize + x0];
                    let c11 = ui_rgba.pixels[y1 * ui_w as usize + x1];

                    let mut sampled = [0u8; 4];
                    for ch in 0..4 {
                        let top = (1.0 - wx) * (c00[ch] as f64) + wx * (c10[ch] as f64);
                        let bot = (1.0 - wx) * (c01[ch] as f64) + wx * (c11[ch] as f64);
                        let val = (1.0 - wy) * top + wy * bot;
                        sampled[ch] = val.clamp(0.0, 255.0).round() as u8;
                    }
                    pixels[y * vp_w + x] = sampled;
                }
            }
        }

        let pick_rx = pick_screen_xy[0] - (vp_w as f64 * 0.5);
        let pick_ry = (vp_h as f64 * 0.5) - pick_screen_xy[1];
        let pick_rz = f;

        let pick_d_dot_n = pick_rx * normal[0] + pick_ry * normal[1] + pick_rz * normal[2];

        let (hit_surface, picked_uv, picked_ui_xy) = if pick_d_dot_n.abs() > 1e-8 {
            let t = c_dot_n / pick_d_dot_n;
            if t > 0.0 {
                let px = t * pick_rx;
                let py = t * pick_ry;
                let pz = t * pick_rz;

                let dx = px - center[0];
                let dy = py - center[1];
                let dz = pz - center[2];

                let loc_x = cy * dx - sy * dz;
                let loc_y = sy * sx * dx + cx * dy + cy * sx * dz;

                let u = loc_x / qw + 0.5;
                let v = 0.5 - loc_y / qh;

                let hit = (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v);
                let uv = [u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)];
                let ui_xy = [uv[0] * ui_w as f64, uv[1] * ui_h as f64];
                (hit, uv, ui_xy)
            } else {
                (false, [0.0, 0.0], [0.0, 0.0])
            }
        } else {
            (false, [0.0, 0.0], [0.0, 0.0])
        };

        let projected_rgba = self.store_rgba(vp_w as u32, vp_h as u32, pixels)?;

        let mut record = BTreeMap::new();
        record.insert("projected_rgba".into(), projected_rgba);
        record.insert("hit_surface".into(), Value::Bool(hit_surface));
        record.insert(
            "picked_uv".into(),
            Value::List(vec![Value::F64(picked_uv[0]), Value::F64(picked_uv[1])]),
        );
        record.insert(
            "picked_ui_xy".into(),
            Value::List(vec![
                Value::F64(picked_ui_xy[0]),
                Value::F64(picked_ui_xy[1]),
            ]),
        );

        Ok(Value::Record(record))
    }

    pub(super) fn route_input_action(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "event_type",
                "pointer_xy",
                "hud_rects",
                "diegetic_quad_bounds",
                "action_binding",
                "axis_value",
                "deadzone",
            ],
        )?;

        let event_type = match a.get("event_type") {
            Some(v) => text(v)?,
            None => "pointer_down",
        };
        if event_type != "pointer_down"
            && event_type != "key_chord"
            && event_type != "gamepad_axis"
        {
            return Err("event_type must be 'pointer_down', 'key_chord', or 'gamepad_axis'".into());
        }

        let pointer_xy = match a.get("pointer_xy") {
            Some(v) => vector::<2>(v)?,
            None => [160.0, 120.0],
        };

        let hud_rects = match a.get("hud_rects") {
            Some(v) => list(v)?,
            None => &[],
        };

        let quad_bounds = match a.get("diegetic_quad_bounds") {
            Some(v) => vector::<4>(v)?,
            None => [80.0, 60.0, 200.0, 150.0],
        };
        if quad_bounds[2] <= 0.0 || quad_bounds[3] <= 0.0 {
            return Err("diegetic_quad_bounds width and height must be positive".into());
        }

        let action_binding = match a.get("action_binding") {
            Some(v) => text(v)?,
            None => "action:interact_primary",
        };

        let axis_value = number(a, "axis_value", 0.75)?;
        if !(-1.0..=1.0).contains(&axis_value) {
            return Err("axis_value outside -1.0..=1.0".into());
        }

        let deadzone = number(a, "deadzone", 0.15)?;
        if !(0.0..=0.9).contains(&deadzone) {
            return Err("deadzone outside 0.0..=0.9".into());
        }

        self.charge(128 + hud_rects.len() * 16, 256)?;

        let is_system_shortcut = match event_type {
            "key_chord" => {
                action_binding == "action:escape"
                    || action_binding.contains("sanctuary")
                    || action_binding.contains("system")
                    || action_binding == "Escape"
                    || action_binding == "Pause"
                    || action_binding == "F1"
                    || action_binding == "Alt+F4"
            }
            _ => false,
        };

        if is_system_shortcut {
            let mut record = BTreeMap::new();
            record.insert("consumed_stage".into(), Value::U64(1));
            record.insert(
                "consumed_by_id".into(),
                Value::String("stage_1_system_shortcut".into()),
            );
            record.insert("propagated_to_3d".into(), Value::Bool(false));
            record.insert(
                "diegetic_uv".into(),
                Value::List(vec![Value::F64(0.0), Value::F64(0.0)]),
            );
            record.insert(
                "dispatched_action".into(),
                Value::String(action_binding.to_string()),
            );
            record.insert("remapped_axis_strength".into(), Value::F64(1.0));
            return Ok(Value::Record(record));
        }

        if event_type == "pointer_down" {
            let px = pointer_xy[0];
            let py = pointer_xy[1];

            for rect_val in hud_rects {
                let r = record(rect_val)?;
                let id = text(required(r, "id")?)?;
                let x = number(r, "x", 0.0)?;
                let y = number(r, "y", 0.0)?;
                let w = number(r, "width", 0.0)?;
                let h = number(r, "height", 0.0)?;
                let filter = match r.get("filter") {
                    Some(v) => text(v)?,
                    None => "stop",
                };

                if px >= x && px <= x + w && py >= y && py <= y + h {
                    match filter {
                        "stop" => {
                            let mut record = BTreeMap::new();
                            record.insert("consumed_stage".into(), Value::U64(2));
                            record.insert("consumed_by_id".into(), Value::String(id.to_string()));
                            record.insert("propagated_to_3d".into(), Value::Bool(false));
                            record.insert(
                                "diegetic_uv".into(),
                                Value::List(vec![Value::F64(0.0), Value::F64(0.0)]),
                            );
                            record.insert(
                                "dispatched_action".into(),
                                Value::String(format!("{id}:{action_binding}")),
                            );
                            record.insert("remapped_axis_strength".into(), Value::F64(0.0));
                            return Ok(Value::Record(record));
                        }
                        "pass" => break,
                        "ignore" => continue,
                        _ => return Err("hud filter must be 'stop', 'pass', or 'ignore'".into()),
                    }
                }
            }
        }

        if event_type == "pointer_down" {
            let px = pointer_xy[0];
            let py = pointer_xy[1];
            let qx = quad_bounds[0];
            let qy = quad_bounds[1];
            let qw = quad_bounds[2];
            let qh = quad_bounds[3];

            if px >= qx && px <= qx + qw && py >= qy && py <= qy + qh {
                let u = ((px - qx) / qw).clamp(0.0, 1.0);
                let v = ((py - qy) / qh).clamp(0.0, 1.0);

                let mut record = BTreeMap::new();
                record.insert("consumed_stage".into(), Value::U64(3));
                record.insert(
                    "consumed_by_id".into(),
                    Value::String("stage_3_diegetic_surface".into()),
                );
                record.insert("propagated_to_3d".into(), Value::Bool(true));
                record.insert(
                    "diegetic_uv".into(),
                    Value::List(vec![Value::F64(u), Value::F64(v)]),
                );
                record.insert(
                    "dispatched_action".into(),
                    Value::String(action_binding.to_string()),
                );
                record.insert("remapped_axis_strength".into(), Value::F64(1.0));
                return Ok(Value::Record(record));
            }
        }

        let (propagated_3d, remapped_strength) = if event_type == "gamepad_axis" {
            let abs_val = axis_value.abs();
            if abs_val <= deadzone {
                (false, 0.0)
            } else {
                let scaled = ((abs_val - deadzone) / (1.0 - deadzone)).clamp(0.0, 1.0)
                    * axis_value.signum();
                (true, scaled)
            }
        } else {
            (false, 1.0)
        };

        let mut record = BTreeMap::new();
        record.insert("consumed_stage".into(), Value::U64(4));
        record.insert(
            "consumed_by_id".into(),
            Value::String("stage_4_semantic_input_map".into()),
        );
        record.insert("propagated_to_3d".into(), Value::Bool(propagated_3d));
        record.insert(
            "diegetic_uv".into(),
            Value::List(vec![Value::F64(0.0), Value::F64(0.0)]),
        );
        record.insert(
            "dispatched_action".into(),
            Value::String(action_binding.to_string()),
        );
        record.insert("remapped_axis_strength".into(), Value::F64(remapped_strength));

        Ok(Value::Record(record))
    }
}

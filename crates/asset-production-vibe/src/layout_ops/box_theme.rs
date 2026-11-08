//! Deterministic 2D flex/grid box-tree layout solver and Webizen theme scope binder.

use super::{build_theme_tokens, color_to_value, LayoutBox};
use crate::geometry::color;
use crate::*;
use std::collections::BTreeMap;

impl AssetHost<'_> {
    /// Solves deterministic 2D spatial flex or CSS-grid-like layouts across viewports.
    pub(super) fn solve_box_tree(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "width",
                "height",
                "direction",
                "padding",
                "gap",
                "grid_columns",
                "boxes",
            ],
        )?;

        let viewport_width = number(a, "width", 1280.0)?;
        if !(16.0..=8192.0).contains(&viewport_width) {
            return Err("width outside 16.0..8192.0".into());
        }

        let viewport_height = number(a, "height", 800.0)?;
        if !(16.0..=8192.0).contains(&viewport_height) {
            return Err("height outside 16.0..8192.0".into());
        }

        let direction = match a.get("direction") {
            Some(v) => text(v)?,
            None => "column",
        };
        if direction != "row" && direction != "column" {
            return Err("direction must be 'row' or 'column'".into());
        }

        let padding = number(a, "padding", 16.0)?;
        if !(0.0..=512.0).contains(&padding) {
            return Err("padding outside 0.0..512.0".into());
        }

        let gap = number(a, "gap", 12.0)?;
        if !(0.0..=512.0).contains(&gap) {
            return Err("gap outside 0.0..512.0".into());
        }

        let grid_columns = match a.get("grid_columns") {
            Some(v) => {
                let cols = integer(v, "grid_columns")?;
                if cols > 32 {
                    return Err("grid_columns outside 0..32".into());
                }
                cols
            }
            None => 0,
        };

        let raw_boxes = list(required(a, "boxes")?)?;
        if raw_boxes.is_empty() || raw_boxes.len() > 128 {
            return Err("boxes list outside 1..128 items".into());
        }

        self.charge(raw_boxes.len() * 64 + 128, raw_boxes.len() * 256 + 1024)?;

        struct ParsedItem<'p> {
            id: &'p str,
            role: &'p str,
            label: &'p str,
            fixed_w: Option<f64>,
            fixed_h: Option<f64>,
            flex: f64,
            state: &'p str,
        }

        let mut items = Vec::with_capacity(raw_boxes.len());
        for (idx, item_val) in raw_boxes.iter().enumerate() {
            let item_rec = record(item_val)?;
            fields(
                item_rec,
                &["id", "role", "label", "width", "height", "flex", "state"],
            )?;

            let id = match item_rec.get("id") {
                Some(v) => text(v)?,
                None => return Err(format!("Box index {idx} missing id")),
            };

            let role = match item_rec.get("role") {
                Some(v) => text(v)?,
                None => "panel",
            };
            if !matches!(
                role,
                "panel"
                    | "header"
                    | "sidebar"
                    | "card"
                    | "button"
                    | "input"
                    | "badge"
                    | "canvas"
            ) {
                return Err(format!(
                    "Invalid role '{role}' for box '{id}': expected panel, header, sidebar, card, button, input, badge, or canvas"
                ));
            }

            let label = match item_rec.get("label") {
                Some(v) => text(v)?,
                None => "",
            };

            let fixed_w = match item_rec.get("width") {
                Some(v) => {
                    let w = num(v)?;
                    if w <= 0.0 {
                        return Err(format!("Box '{id}' width must be > 0"));
                    }
                    Some(w)
                }
                None => None,
            };

            let fixed_h = match item_rec.get("height") {
                Some(v) => {
                    let h = num(v)?;
                    if h <= 0.0 {
                        return Err(format!("Box '{id}' height must be > 0"));
                    }
                    Some(h)
                }
                None => None,
            };

            let flex = match item_rec.get("flex") {
                Some(v) => {
                    let f = num(v)?;
                    if f < 0.0 {
                        return Err(format!("Box '{id}' flex must be >= 0"));
                    }
                    f
                }
                None => 1.0,
            };

            let state = match item_rec.get("state") {
                Some(v) => text(v)?,
                None => "default",
            };
            if !matches!(state, "default" | "hover" | "focus" | "active" | "disabled") {
                return Err(format!(
                    "Invalid state '{state}' for box '{id}': expected default, hover, focus, active, or disabled"
                ));
            }

            items.push(ParsedItem {
                id,
                role,
                label,
                fixed_w,
                fixed_h,
                flex,
                state,
            });
        }

        let x0 = padding;
        let y0 = padding;
        let avail_w = (viewport_width - 2.0 * padding).max(0.0);
        let avail_h = (viewport_height - 2.0 * padding).max(0.0);

        let mut solved_boxes = Vec::with_capacity(items.len());

        if grid_columns > 0 {
            let cols = grid_columns;
            let rows = (items.len() + cols - 1) / cols;

            let col_gap_total = (cols as f64 - 1.0).max(0.0) * gap;
            let col_w = ((avail_w - col_gap_total) / cols as f64).max(0.0);

            let row_gap_total = (rows as f64 - 1.0).max(0.0) * gap;
            let avail_rows_h = (avail_h - row_gap_total).max(0.0);

            let mut row_heights = Vec::with_capacity(rows);
            let mut flex_rows = Vec::new();
            let mut fixed_height_sum = 0.0;

            for r in 0..rows {
                let start_idx = r * cols;
                let end_idx = (start_idx + cols).min(items.len());
                let row_slice = &items[start_idx..end_idx];

                let max_fixed = row_slice
                    .iter()
                    .filter_map(|it| it.fixed_h)
                    .fold(0.0_f64, |acc, h| acc.max(h));

                if max_fixed > 0.0 {
                    row_heights.push(max_fixed);
                    fixed_height_sum += max_fixed;
                } else {
                    let row_flex = row_slice
                        .iter()
                        .map(|it| it.flex)
                        .fold(1.0_f64, |acc, f| acc.max(f));
                    row_heights.push(0.0);
                    flex_rows.push((r, row_flex));
                }
            }

            let total_row_flex: f64 = flex_rows.iter().map(|(_, f)| *f).sum();
            let remaining_row_h = (avail_rows_h - fixed_height_sum).max(0.0);

            for (r, f) in flex_rows {
                let h = if total_row_flex > 0.0 {
                    remaining_row_h * (f / total_row_flex)
                } else {
                    remaining_row_h / rows as f64
                };
                row_heights[r] = h;
            }

            let mut row_y = Vec::with_capacity(rows);
            let mut current_y = y0;
            for rh in &row_heights {
                row_y.push(current_y);
                current_y += *rh + gap;
            }

            for (i, item) in items.iter().enumerate() {
                let c = i % cols;
                let r = i / cols;

                let cell_x = x0 + c as f64 * (col_w + gap);
                let cell_y = row_y[r];
                let cell_h = row_heights[r];

                let w = item.fixed_w.unwrap_or(col_w);
                let h = item.fixed_h.unwrap_or(cell_h);

                solved_boxes.push(LayoutBox {
                    id: item.id.to_string(),
                    role: item.role.to_string(),
                    label: item.label.to_string(),
                    state: item.state.to_string(),
                    x: cell_x,
                    y: cell_y,
                    width: w,
                    height: h,
                });
            }
        } else if direction == "row" {
            let gaps_total = (items.len() as f64 - 1.0).max(0.0) * gap;
            let avail_main = (avail_w - gaps_total).max(0.0);

            let fixed_sum: f64 = items.iter().filter_map(|it| it.fixed_w).sum();
            let flex_sum: f64 = items
                .iter()
                .filter(|it| it.fixed_w.is_none())
                .map(|it| it.flex)
                .sum();
            let remaining_main = (avail_main - fixed_sum).max(0.0);
            let unconstrained_count = items.iter().filter(|it| it.fixed_w.is_none()).count();

            let mut cur_x = x0;
            for item in &items {
                let w = match item.fixed_w {
                    Some(w) => w,
                    None => {
                        if flex_sum > 0.0 {
                            remaining_main * (item.flex / flex_sum)
                        } else if unconstrained_count > 0 {
                            remaining_main / unconstrained_count as f64
                        } else {
                            0.0
                        }
                    }
                };
                let h = item.fixed_h.unwrap_or(avail_h);

                solved_boxes.push(LayoutBox {
                    id: item.id.to_string(),
                    role: item.role.to_string(),
                    label: item.label.to_string(),
                    state: item.state.to_string(),
                    x: cur_x,
                    y: y0,
                    width: w,
                    height: h,
                });

                cur_x += w + gap;
            }
        } else {
            let gaps_total = (items.len() as f64 - 1.0).max(0.0) * gap;
            let avail_main = (avail_h - gaps_total).max(0.0);

            let fixed_sum: f64 = items.iter().filter_map(|it| it.fixed_h).sum();
            let flex_sum: f64 = items
                .iter()
                .filter(|it| it.fixed_h.is_none())
                .map(|it| it.flex)
                .sum();
            let remaining_main = (avail_main - fixed_sum).max(0.0);
            let unconstrained_count = items.iter().filter(|it| it.fixed_h.is_none()).count();

            let mut cur_y = y0;
            for item in &items {
                let h = match item.fixed_h {
                    Some(h) => h,
                    None => {
                        if flex_sum > 0.0 {
                            remaining_main * (item.flex / flex_sum)
                        } else if unconstrained_count > 0 {
                            remaining_main / unconstrained_count as f64
                        } else {
                            0.0
                        }
                    }
                };
                let w = item.fixed_w.unwrap_or(avail_w);

                solved_boxes.push(LayoutBox {
                    id: item.id.to_string(),
                    role: item.role.to_string(),
                    label: item.label.to_string(),
                    state: item.state.to_string(),
                    x: x0,
                    y: cur_y,
                    width: w,
                    height: h,
                });

                cur_y += h + gap;
            }
        }

        let box_records = solved_boxes
            .into_iter()
            .map(|b| {
                Value::Record(BTreeMap::from([
                    ("id".into(), Value::String(b.id)),
                    ("role".into(), Value::String(b.role)),
                    ("label".into(), Value::String(b.label)),
                    ("state".into(), Value::String(b.state)),
                    ("x".into(), Value::F64(b.x)),
                    ("y".into(), Value::F64(b.y)),
                    ("width".into(), Value::F64(b.width)),
                    ("height".into(), Value::F64(b.height)),
                ]))
            })
            .collect();

        Ok(Value::Record(BTreeMap::from([
            ("viewport_width".into(), Value::F64(viewport_width)),
            ("viewport_height".into(), Value::F64(viewport_height)),
            ("direction".into(), Value::String(direction.to_string())),
            ("boxes".into(), Value::List(box_records)),
        ])))
    }

    /// Binds structured Webizen scoped design tokens and CSS variables.
    pub(super) fn bind_theme_scope(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["scope", "mode", "accent", "corner_radius"])?;

        let scope = match a.get("scope") {
            Some(v) => text(v)?,
            None => "app",
        };
        if !matches!(scope, "environment" | "app" | "page" | "module") {
            return Err("scope must be 'environment', 'app', 'page', or 'module'".into());
        }

        let mode = match a.get("mode") {
            Some(v) => text(v)?,
            None => "holosonic-dark",
        };
        if !matches!(
            mode,
            "holosonic-dark" | "obsidian-studio" | "daylight-paper" | "high-contrast"
        ) {
            return Err(
                "mode must be 'holosonic-dark', 'obsidian-studio', 'daylight-paper', or 'high-contrast'"
                    .into(),
            );
        }

        let accent = match a.get("accent") {
            Some(v) => color(v)?,
            None => [0, 216, 242, 255],
        };

        let corner_radius = number(a, "corner_radius", 10.0)?;
        if !(0.0..=64.0).contains(&corner_radius) {
            return Err("corner_radius outside 0.0..64.0".into());
        }

        self.charge(64, 1024)?;

        let tokens = build_theme_tokens(scope, mode, accent, corner_radius);

        Ok(Value::Record(BTreeMap::from([
            ("scope".into(), Value::String(tokens.scope)),
            ("mode".into(), Value::String(tokens.mode)),
            ("surface_bg".into(), color_to_value(tokens.surface_bg)),
            ("panel_bg".into(), color_to_value(tokens.panel_bg)),
            ("panel_border".into(), color_to_value(tokens.panel_border)),
            ("text_primary".into(), color_to_value(tokens.text_primary)),
            ("text_muted".into(), color_to_value(tokens.text_muted)),
            ("accent".into(), color_to_value(tokens.accent)),
            ("focus_ring".into(), color_to_value(tokens.focus_ring)),
            ("hover_fill".into(), color_to_value(tokens.hover_fill)),
            ("active_fill".into(), color_to_value(tokens.active_fill)),
            ("corner_radius".into(), Value::F64(tokens.corner_radius)),
            ("css_variables".into(), Value::String(tokens.css_variables)),
        ])))
    }
}

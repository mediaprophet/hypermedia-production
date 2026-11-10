//! Deterministic 2D flex/grid box-tree layout solver and Webizen theme scope binder.

use super::{build_theme_tokens, color_to_value, LayoutBox};
use crate::geometry::color;
use crate::*;
use std::collections::BTreeMap;

/// Freeze-and-redistribute free main-axis space so min/max clamps stay exact.
fn distribute_flex_sizes(flex: &[f64], min: &[f64], max: &[f64], remaining: f64) -> Vec<f64> {
    let n = flex.len();
    let mut size = vec![0.0; n];
    let mut frozen = vec![false; n];
    let mut free = remaining.max(0.0);
    loop {
        let mut open_flex = 0.0;
        let mut open_n = 0usize;
        for i in 0..n {
            if !frozen[i] {
                open_flex += flex[i];
                open_n += 1;
            }
        }
        if open_n == 0 {
            break;
        }
        let mut froze = false;
        let mut consumed = 0.0;
        for i in 0..n {
            if frozen[i] {
                continue;
            }
            let raw = if open_flex > 0.0 {
                free * (flex[i] / open_flex)
            } else {
                free / open_n as f64
            };
            if raw < min[i] - 1e-12 {
                size[i] = min[i];
                frozen[i] = true;
                consumed += min[i];
                froze = true;
            } else if raw > max[i] + 1e-12 {
                size[i] = max[i];
                frozen[i] = true;
                consumed += max[i];
                froze = true;
            }
        }
        if !froze {
            for i in 0..n {
                if frozen[i] {
                    continue;
                }
                size[i] = if open_flex > 0.0 {
                    free * (flex[i] / open_flex)
                } else {
                    free / open_n as f64
                };
                frozen[i] = true;
            }
            break;
        }
        free = (free - consumed).max(0.0);
    }
    size
}

fn boxes_non_overlapping(boxes: &[LayoutBox]) -> bool {
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let a = &boxes[i];
            let b = &boxes[j];
            let overlap = a.x < b.x + b.width - 1e-9
                && b.x < a.x + a.width - 1e-9
                && a.y < b.y + b.height - 1e-9
                && b.y < a.y + a.height - 1e-9;
            if overlap {
                return false;
            }
        }
    }
    true
}

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
            min_size: f64,
            max_size: f64,
            cross_size: Option<f64>,
            span: usize,
            state: &'p str,
        }

        let mut items = Vec::with_capacity(raw_boxes.len());
        for (idx, item_val) in raw_boxes.iter().enumerate() {
            let item_rec = record(item_val)?;
            fields(
                item_rec,
                &[
                    "id",
                    "role",
                    "label",
                    "width",
                    "height",
                    "flex",
                    "state",
                    "min_size",
                    "max_size",
                    "cross_size",
                    "span",
                ],
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
                    | "toolbar"
                    | "footer"
                    | "card"
                    | "button"
                    | "input"
                    | "badge"
                    | "canvas"
            ) {
                return Err(format!(
                    "Invalid role '{role}' for box '{id}': expected panel, header, sidebar, toolbar, footer, card, button, input, badge, or canvas"
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

            let min_size = match item_rec.get("min_size") {
                Some(v) => {
                    let m = num(v)?;
                    if m < 0.0 {
                        return Err(format!("Box '{id}' min_size must be >= 0"));
                    }
                    m
                }
                None => 0.0,
            };
            let max_size = match item_rec.get("max_size") {
                Some(v) => {
                    let m = num(v)?;
                    if m < min_size {
                        return Err(format!("Box '{id}' max_size must be >= min_size"));
                    }
                    m
                }
                None => 8192.0,
            };
            let cross_size = match item_rec.get("cross_size") {
                Some(v) => {
                    let c = num(v)?;
                    if c < 0.0 {
                        return Err(format!("Box '{id}' cross_size must be >= 0"));
                    }
                    if c > 0.0 {
                        Some(c)
                    } else {
                        None
                    }
                }
                None => None,
            };
            let span = match item_rec.get("span") {
                Some(v) => {
                    let s = integer(v, "span")?;
                    if s == 0 || s > 32 {
                        return Err(format!("Box '{id}' span outside 1..32"));
                    }
                    s
                }
                None => 1,
            };

            items.push(ParsedItem {
                id,
                role,
                label,
                fixed_w,
                fixed_h,
                flex,
                min_size,
                max_size,
                cross_size,
                span,
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
            let mut placements: Vec<(usize, usize, usize)> = Vec::with_capacity(items.len());
            let mut col = 0usize;
            let mut row = 0usize;
            for item in &items {
                let span = item.span.clamp(1, cols);
                if col + span > cols {
                    col = 0;
                    row += 1;
                }
                placements.push((col, row, span));
                col += span;
            }
            let rows = row + 1;

            let col_gap_total = (cols as f64 - 1.0).max(0.0) * gap;
            let col_w = ((avail_w - col_gap_total) / cols as f64).max(0.0);

            let row_gap_total = (rows as f64 - 1.0).max(0.0) * gap;
            let avail_rows_h = (avail_h - row_gap_total).max(0.0);

            let mut row_heights = vec![0.0; rows];
            let mut flex_rows: Vec<(usize, f64)> = Vec::new();
            let mut fixed_height_sum = 0.0;

            for r in 0..rows {
                let mut max_fixed = 0.0_f64;
                let mut row_flex = 0.0_f64;
                let mut any_fixed = false;
                for (i, item) in items.iter().enumerate() {
                    if placements[i].1 != r {
                        continue;
                    }
                    if let Some(h) = item.fixed_h {
                        max_fixed = max_fixed.max(h);
                        any_fixed = true;
                    } else {
                        row_flex = row_flex.max(item.flex);
                    }
                }
                if any_fixed {
                    row_heights[r] = max_fixed;
                    fixed_height_sum += max_fixed;
                } else {
                    flex_rows.push((r, row_flex.max(1.0)));
                }
            }

            let total_row_flex: f64 = flex_rows.iter().map(|(_, f)| *f).sum();
            let remaining_row_h = (avail_rows_h - fixed_height_sum).max(0.0);
            if !flex_rows.is_empty() {
                let flex_vals: Vec<f64> = flex_rows.iter().map(|(_, f)| *f).collect();
                let mins: Vec<f64> = flex_rows
                    .iter()
                    .map(|(r, _)| {
                        items
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| placements[*i].1 == *r)
                            .map(|(_, it)| it.min_size)
                            .fold(0.0_f64, f64::max)
                    })
                    .collect();
                let maxs: Vec<f64> = flex_rows
                    .iter()
                    .map(|(r, _)| {
                        items
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| placements[*i].1 == *r)
                            .map(|(_, it)| it.max_size)
                            .fold(8192.0_f64, f64::min)
                    })
                    .collect();
                let distributed = if total_row_flex > 0.0 {
                    distribute_flex_sizes(&flex_vals, &mins, &maxs, remaining_row_h)
                } else {
                    vec![remaining_row_h / flex_rows.len() as f64; flex_rows.len()]
                };
                for (k, (r, _)) in flex_rows.iter().enumerate() {
                    row_heights[*r] = distributed[k];
                }
            }

            let mut row_y = Vec::with_capacity(rows);
            let mut current_y = y0;
            for rh in &row_heights {
                row_y.push(current_y);
                current_y += *rh + gap;
            }

            for (i, item) in items.iter().enumerate() {
                let (c, r, span) = placements[i];
                let cell_x = x0 + c as f64 * (col_w + gap);
                let cell_y = row_y[r];
                let cell_h = row_heights[r];
                let cell_w = col_w * span as f64 + gap * (span.saturating_sub(1) as f64);

                let w = item
                    .fixed_w
                    .unwrap_or(cell_w)
                    .clamp(item.min_size, item.max_size);
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

            let mut widths = vec![0.0; items.len()];
            let mut flex_idx = Vec::new();
            let mut flex_w = Vec::new();
            let mut flex_min = Vec::new();
            let mut flex_max = Vec::new();
            let mut fixed_sum = 0.0;
            for (i, item) in items.iter().enumerate() {
                if let Some(w) = item.fixed_w {
                    widths[i] = w.clamp(item.min_size, item.max_size);
                    fixed_sum += widths[i];
                } else {
                    flex_idx.push(i);
                    flex_w.push(item.flex);
                    flex_min.push(item.min_size);
                    flex_max.push(item.max_size);
                }
            }
            let remaining_main = (avail_main - fixed_sum).max(0.0);
            let distributed = distribute_flex_sizes(&flex_w, &flex_min, &flex_max, remaining_main);
            for (k, &i) in flex_idx.iter().enumerate() {
                widths[i] = distributed[k];
            }

            let mut cur_x = x0;
            for (i, item) in items.iter().enumerate() {
                let w = widths[i];
                let h = item
                    .fixed_h
                    .or(item.cross_size)
                    .unwrap_or(avail_h);

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

            let mut heights = vec![0.0; items.len()];
            let mut flex_idx = Vec::new();
            let mut flex_h = Vec::new();
            let mut flex_min = Vec::new();
            let mut flex_max = Vec::new();
            let mut fixed_sum = 0.0;
            for (i, item) in items.iter().enumerate() {
                if let Some(h) = item.fixed_h {
                    heights[i] = h.clamp(item.min_size, item.max_size);
                    fixed_sum += heights[i];
                } else {
                    flex_idx.push(i);
                    flex_h.push(item.flex);
                    flex_min.push(item.min_size);
                    flex_max.push(item.max_size);
                }
            }
            let remaining_main = (avail_main - fixed_sum).max(0.0);
            let distributed = distribute_flex_sizes(&flex_h, &flex_min, &flex_max, remaining_main);
            for (k, &i) in flex_idx.iter().enumerate() {
                heights[i] = distributed[k];
            }

            let mut cur_y = y0;
            for (i, item) in items.iter().enumerate() {
                let h = heights[i];
                let w = item
                    .fixed_w
                    .or(item.cross_size)
                    .unwrap_or(avail_w);

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

        let non_overlapping = boxes_non_overlapping(&solved_boxes);
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
            ("non_overlapping".into(), Value::Bool(non_overlapping)),
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

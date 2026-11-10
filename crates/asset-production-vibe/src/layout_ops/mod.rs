//! Deterministic spatial box-tree layout solving and Webizen scoped theme/component
//! sheet synthesis for the `layout` category.
//! Enables Synth and Webizen Studio to solve responsive flex/grid layouts, bind scoped
//! theme tokens, and render complete UI sheets to straight-alpha RGBA or standalone SVG.

mod box_theme;
mod sheet_render;

use crate::geometry::color;
use crate::*;

#[derive(Clone, Debug)]
pub(super) struct LayoutBox {
    pub(super) id: String,
    pub(super) role: String,
    pub(super) label: String,
    pub(super) state: String,
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) width: f64,
    pub(super) height: f64,
}

#[derive(Clone, Debug)]
pub(super) struct ThemeTokens {
    pub(super) scope: String,
    pub(super) mode: String,
    pub(super) surface_bg: [u8; 4],
    pub(super) panel_bg: [u8; 4],
    pub(super) panel_border: [u8; 4],
    pub(super) text_primary: [u8; 4],
    pub(super) text_muted: [u8; 4],
    pub(super) accent: [u8; 4],
    pub(super) focus_ring: [u8; 4],
    pub(super) hover_fill: [u8; 4],
    pub(super) active_fill: [u8; 4],
    pub(super) corner_radius: f64,
    pub(super) css_variables: String,
}

impl AssetHost<'_> {
    pub(crate) fn layout_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.layout.solve_box_tree" | "layout.solve_box_tree" | "solve_box_tree" => {
                self.solve_box_tree(a)
            }
            "hypermedia.layout.bind_theme_scope"
            | "layout.bind_theme_scope"
            | "bind_theme_scope" => self.bind_theme_scope(a),
            "hypermedia.layout.render_ui_sheet" | "layout.render_ui_sheet" | "render_ui_sheet" => {
                self.render_ui_sheet(a)
            }
            "hypermedia.layout.project_ui_svg" | "layout.project_ui_svg" | "project_ui_svg" => {
                self.project_ui_svg(a)
            }
            _ => Err(format!("Unknown layout operation: {id}")),
        }
    }
}

pub(super) fn color_to_value(c: [u8; 4]) -> Value {
    Value::List(c.into_iter().map(|b| Value::U64(b as u64)).collect())
}

pub(super) fn rgba_css(c: [u8; 4]) -> String {
    if c[3] == 255 {
        format!("rgb({}, {}, {})", c[0], c[1], c[2])
    } else {
        let a = c[3] as f64 / 255.0;
        format!("rgba({}, {}, {}, {:.3})", c[0], c[1], c[2], a)
    }
}

pub(super) fn blend_color(a: [u8; 4], b: [u8; 4], t: f64) -> [u8; 4] {
    let t = t.clamp(0.0, 1.0);
    let inv = 1.0 - t;
    [
        (a[0] as f64 * inv + b[0] as f64 * t).round() as u8,
        (a[1] as f64 * inv + b[1] as f64 * t).round() as u8,
        (a[2] as f64 * inv + b[2] as f64 * t).round() as u8,
        (a[3] as f64 * inv + b[3] as f64 * t).round() as u8,
    ]
}

pub(super) fn build_theme_tokens(
    scope: &str,
    mode: &str,
    accent: [u8; 4],
    corner_radius: f64,
) -> ThemeTokens {
    let (
        surface_bg,
        panel_bg,
        panel_border,
        text_primary,
        text_muted,
        focus_ring,
        hover_fill,
        active_fill,
    ) = match mode {
        "obsidian-studio" => (
            [10, 10, 12, 255],
            [18, 18, 22, 255],
            [32, 32, 40, 255],
            [232, 232, 238, 255],
            [120, 120, 134, 255],
            [accent[0], accent[1], accent[2], 220],
            [28, 28, 36, 255],
            [36, 36, 48, 255],
        ),
        "daylight-paper" => (
            [248, 249, 250, 255],
            [255, 255, 255, 255],
            [220, 224, 230, 255],
            [20, 24, 32, 255],
            [100, 110, 124, 255],
            [accent[0], accent[1], accent[2], 200],
            [240, 243, 248, 255],
            [230, 234, 242, 255],
        ),
        "high-contrast" => (
            [0, 0, 0, 255],
            [8, 8, 8, 255],
            [255, 255, 255, 255],
            [255, 255, 255, 255],
            [200, 200, 200, 255],
            [accent[0], accent[1], accent[2], 255],
            [40, 40, 40, 255],
            [70, 70, 70, 255],
        ),
        _ => (
            [14, 18, 24, 255],
            [22, 28, 38, 255],
            [38, 48, 64, 255],
            [240, 244, 250, 255],
            [140, 150, 170, 255],
            [accent[0], accent[1], accent[2], 200],
            [32, 40, 54, 255],
            [42, 54, 72, 255],
        ),
    };

    let surface_css = rgba_css(surface_bg);
    let panel_css = rgba_css(panel_bg);
    let border_css = rgba_css(panel_border);
    let text_p_css = rgba_css(text_primary);
    let text_m_css = rgba_css(text_muted);
    let accent_css = rgba_css(accent);
    let focus_css = rgba_css(focus_ring);
    let hover_css = rgba_css(hover_fill);
    let active_css = rgba_css(active_fill);

    let css_variables = format!(
        ":root, [data-scope=\"{scope}\"] {{\n  \
         --webizen-scope: {scope};\n  \
         --webizen-mode: {mode};\n  \
         --webizen-surface-bg: {surface_css};\n  \
         --webizen-panel-bg: {panel_css};\n  \
         --webizen-panel-border: {border_css};\n  \
         --webizen-text-primary: {text_p_css};\n  \
         --webizen-text-muted: {text_m_css};\n  \
         --webizen-accent: {accent_css};\n  \
         --webizen-focus-ring: {focus_css};\n  \
         --webizen-hover-fill: {hover_css};\n  \
         --webizen-active-fill: {active_css};\n  \
         --webizen-corner-radius: {corner_radius:.1}px;\n\
        }}"
    );

    ThemeTokens {
        scope: scope.to_string(),
        mode: mode.to_string(),
        surface_bg,
        panel_bg,
        panel_border,
        text_primary,
        text_muted,
        accent,
        focus_ring,
        hover_fill,
        active_fill,
        corner_radius,
        css_variables,
    }
}

pub(super) fn parse_theme_record(r: &Args) -> Result<ThemeTokens, String> {
    let scope = match r.get("scope") {
        Some(v) => text(v)?.to_string(),
        None => "app".to_string(),
    };
    let mode = match r.get("mode") {
        Some(v) => text(v)?.to_string(),
        None => "holosonic-dark".to_string(),
    };
    let accent = match r.get("accent") {
        Some(v) => color(v)?,
        None => [0, 216, 242, 255],
    };
    let corner_radius = number(r, "corner_radius", 10.0)?;

    let def = build_theme_tokens(&scope, &mode, accent, corner_radius);

    let surface_bg = match r.get("surface_bg") {
        Some(v) => color(v)?,
        None => def.surface_bg,
    };
    let panel_bg = match r.get("panel_bg") {
        Some(v) => color(v)?,
        None => def.panel_bg,
    };
    let panel_border = match r.get("panel_border") {
        Some(v) => color(v)?,
        None => def.panel_border,
    };
    let text_primary = match r.get("text_primary") {
        Some(v) => color(v)?,
        None => def.text_primary,
    };
    let text_muted = match r.get("text_muted") {
        Some(v) => color(v)?,
        None => def.text_muted,
    };
    let focus_ring = match r.get("focus_ring") {
        Some(v) => color(v)?,
        None => def.focus_ring,
    };
    let hover_fill = match r.get("hover_fill") {
        Some(v) => color(v)?,
        None => def.hover_fill,
    };
    let active_fill = match r.get("active_fill") {
        Some(v) => color(v)?,
        None => def.active_fill,
    };
    let css_variables = match r.get("css_variables") {
        Some(v) => text(v)?.to_string(),
        None => def.css_variables,
    };

    Ok(ThemeTokens {
        scope,
        mode,
        surface_bg,
        panel_bg,
        panel_border,
        text_primary,
        text_muted,
        accent,
        focus_ring,
        hover_fill,
        active_fill,
        corner_radius,
        css_variables,
    })
}

pub(super) fn compute_box_corner_radius(role: &str, h: f64, base_r: f64) -> f64 {
    match role {
        "button" => base_r.min(h / 2.0),
        "badge" => (h / 2.0).max(base_r),
        "input" => (base_r * 0.6).min(h / 2.0),
        "header" | "sidebar" | "toolbar" | "footer" => 0.0,
        "canvas" => (base_r * 0.4).min(h / 2.0),
        _ => base_r.min(h / 2.0),
    }
}

pub(super) fn compute_box_colors(b: &LayoutBox, theme: &ThemeTokens) -> ([u8; 4], [u8; 4], f64) {
    let mut fill = match b.role.as_str() {
        "panel" => theme.panel_bg,
        "header" | "toolbar" => blend_color(theme.panel_bg, theme.surface_bg, 0.25),
        "footer" => blend_color(theme.panel_bg, theme.surface_bg, 0.20),
        "sidebar" => blend_color(theme.panel_bg, theme.surface_bg, 0.15),
        "card" => theme.panel_bg,
        "button" => match b.state.as_str() {
            "active" => theme.active_fill,
            "hover" => theme.hover_fill,
            "disabled" => [theme.panel_bg[0], theme.panel_bg[1], theme.panel_bg[2], 90],
            _ => blend_color(theme.panel_bg, theme.accent, 0.15),
        },
        "input" => theme.surface_bg,
        "badge" => [theme.accent[0], theme.accent[1], theme.accent[2], 40],
        "canvas" => [
            theme.surface_bg[0] / 2,
            theme.surface_bg[1] / 2,
            theme.surface_bg[2] / 2,
            255,
        ],
        _ => theme.panel_bg,
    };

    let mut border = match b.role.as_str() {
        "badge" => [theme.accent[0], theme.accent[1], theme.accent[2], 180],
        "input" => {
            if b.state == "focus" {
                theme.focus_ring
            } else {
                theme.panel_border
            }
        }
        "canvas" => [
            theme.panel_border[0],
            theme.panel_border[1],
            theme.panel_border[2],
            120,
        ],
        _ => theme.panel_border,
    };

    let mut border_w = 1.0_f64;

    match b.state.as_str() {
        "hover" => {
            if b.role != "button" {
                fill = blend_color(fill, theme.hover_fill, 0.35);
            }
            border = blend_color(border, theme.accent, 0.5);
            border_w = 1.5;
        }
        "active" => {
            if b.role != "button" {
                fill = blend_color(fill, theme.active_fill, 0.5);
            }
            border_w = 1.5;
        }
        "focus" => {
            border = theme.focus_ring;
            border_w = 2.0;
        }
        "disabled" => {
            fill[3] = (fill[3] as f64 * 0.45).round() as u8;
            border[3] = (border[3] as f64 * 0.45).round() as u8;
        }
        _ => {}
    }

    (fill, border, border_w)
}

pub(super) fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use qualia_core_db::etalon_render::frame_lock::{
        FrameLockPacket, FRAME_LOCK_MAGIC, FRAME_LOCK_VERSION,
    };
    use std::collections::BTreeMap;
    use std::sync::atomic::AtomicBool;

    fn host<'a>(
        workspace: AssetWorkspace,
        inputs: &'a [Resource],
        cancel: &'a AtomicBool,
        limits: &'a ToolLimits,
    ) -> AssetHost<'a> {
        AssetHost {
            workspace,
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn box_spec(id: &str, role: &str, extra: &[(&str, Value)]) -> Value {
        let mut rec = BTreeMap::from([
            ("id".into(), Value::String(id.into())),
            ("role".into(), Value::String(role.into())),
        ]);
        for (k, v) in extra {
            rec.insert((*k).into(), v.clone());
        }
        Value::Record(rec)
    }

    fn box_xywh(list: &[Value], id: &str) -> (f64, f64, f64, f64) {
        for item in list {
            let r = record(item).unwrap();
            if text(r.get("id").unwrap()).unwrap() == id {
                return (
                    num(r.get("x").unwrap()).unwrap(),
                    num(r.get("y").unwrap()).unwrap(),
                    num(r.get("width").unwrap()).unwrap(),
                    num(r.get("height").unwrap()).unwrap(),
                );
            }
        }
        panic!("missing box {id}");
    }

    #[test]
    fn test_layout_box_tree_flex_grid_nonoverlapping() {
        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(AssetWorkspace::default(), &[], &cancel, &limits);

        let mut col = Args::new();
        col.insert("width".into(), Value::F64(400.0));
        col.insert("height".into(), Value::F64(300.0));
        col.insert("direction".into(), Value::String("column".into()));
        col.insert("padding".into(), Value::F64(10.0));
        col.insert("gap".into(), Value::F64(8.0));
        col.insert(
            "boxes".into(),
            Value::List(vec![
                box_spec("header", "header", &[("height", Value::F64(40.0))]),
                box_spec("body", "panel", &[("flex", Value::F64(1.0))]),
                box_spec("footer", "footer", &[("height", Value::F64(32.0))]),
            ]),
        );
        let solved = h
            .layout_op("hypermedia.layout.solve_box_tree", &col)
            .expect("column box-tree");
        let rec = record(&solved).unwrap();
        assert_eq!(rec.get("non_overlapping"), Some(&Value::Bool(true)));
        let boxes = list(rec.get("boxes").unwrap()).unwrap();
        assert_eq!(boxes.len(), 3);
        let (hx, hy, hw, hh) = box_xywh(boxes, "header");
        let (_bx, by, _bw, bh) = box_xywh(boxes, "body");
        let (_fx, fy, _fw, fh) = box_xywh(boxes, "footer");
        assert_eq!((hx, hy, hw, hh), (10.0, 10.0, 380.0, 40.0));
        // avail_h = 280, gaps = 16, fixed = 72, flex body = 192
        assert!((bh - 192.0).abs() < 1e-9, "body height {bh}");
        assert!((by - 58.0).abs() < 1e-9, "body y {by}");
        assert!((fy - (by + bh + 8.0)).abs() < 1e-9);
        assert!((fh - 32.0).abs() < 1e-9);

        let mut row = Args::new();
        row.insert("width".into(), Value::F64(400.0));
        row.insert("height".into(), Value::F64(100.0));
        row.insert("direction".into(), Value::String("row".into()));
        row.insert("padding".into(), Value::F64(0.0));
        row.insert("gap".into(), Value::F64(0.0));
        row.insert(
            "boxes".into(),
            Value::List(vec![
                box_spec(
                    "a",
                    "sidebar",
                    &[("flex", Value::F64(1.0)), ("max_size", Value::F64(80.0))],
                ),
                box_spec("b", "panel", &[("flex", Value::F64(1.0))]),
                box_spec("c", "panel", &[("flex", Value::F64(1.0))]),
            ]),
        );
        let solved_row = h
            .layout_op("hypermedia.layout.solve_box_tree", &row)
            .expect("row clamp");
        let rec_row = record(&solved_row).unwrap();
        assert_eq!(rec_row.get("non_overlapping"), Some(&Value::Bool(true)));
        let row_boxes = list(rec_row.get("boxes").unwrap()).unwrap();
        let (_ax, _, aw, _) = box_xywh(row_boxes, "a");
        let (_bx, _, bw, _) = box_xywh(row_boxes, "b");
        let (_cx, _, cw, _) = box_xywh(row_boxes, "c");
        assert!((aw - 80.0).abs() < 1e-9, "clamped sidebar {aw}");
        assert!((bw - 160.0).abs() < 1e-9, "flex b {bw}");
        assert!((cw - 160.0).abs() < 1e-9, "flex c {cw}");

        let mut grid = Args::new();
        grid.insert("width".into(), Value::F64(220.0));
        grid.insert("height".into(), Value::F64(120.0));
        grid.insert("padding".into(), Value::F64(10.0));
        grid.insert("gap".into(), Value::F64(10.0));
        grid.insert("grid_columns".into(), Value::U64(2));
        grid.insert(
            "boxes".into(),
            Value::List(vec![
                box_spec("g0", "card", &[("span", Value::U64(2))]),
                box_spec("g1", "button", &[]),
                box_spec("g2", "badge", &[]),
            ]),
        );
        let solved_grid = h
            .layout_op("hypermedia.layout.solve_box_tree", &grid)
            .expect("grid span");
        let rec_grid = record(&solved_grid).unwrap();
        assert_eq!(rec_grid.get("non_overlapping"), Some(&Value::Bool(true)));
        let grid_boxes = list(rec_grid.get("boxes").unwrap()).unwrap();
        let (g0x, g0y, g0w, _) = box_xywh(grid_boxes, "g0");
        let (g1x, g1y, g1w, _) = box_xywh(grid_boxes, "g1");
        let (g2x, _, g2w, _) = box_xywh(grid_boxes, "g2");
        assert_eq!((g0x, g0y), (10.0, 10.0));
        assert!((g0w - 200.0).abs() < 1e-9, "span-2 width {g0w}");
        assert!(g1y > g0y);
        assert!((g1w - g2w).abs() < 1e-9);
        assert!(g2x > g1x);
    }

    #[test]
    fn test_layout_framlock_crc32c() {
        let mut pkt = FrameLockPacket {
            magic: FRAME_LOCK_MAGIC,
            protocol_version: FRAME_LOCK_VERSION,
            cluster_id: 1,
            lamport_clock: 7,
            observer_pos: [0.0, 1.65, 0.0],
            solar_irradiance_alpha: 1.0,
            observer_rot: [0.0, 0.0, 0.0, 1.0],
            quantum_branch_q: 0,
            environmental_seed: 2026,
            network_flags: 0,
            packet_crc32c: 0,
        };
        pkt.update_crc();
        assert!(pkt.validate().is_ok(), "FrameLock CRC-32C must validate");
        assert_eq!(
            std::mem::size_of::<FrameLockPacket>(),
            64,
            "FrameLockPacket is a 64-byte UDP multicast record"
        );
        let sealed = pkt.packet_crc32c;
        pkt.lamport_clock = 8;
        assert!(
            pkt.validate().is_err(),
            "tampered FrameLock without CRC refresh must fail-closed"
        );
        pkt.update_crc();
        assert_ne!(pkt.packet_crc32c, sealed);
        assert!(pkt.validate().is_ok());

        let cancel = AtomicBool::new(false);
        let limits = ToolLimits::default();
        let mut h = host(AssetWorkspace::default(), &[], &cancel, &limits);
        let mut args = Args::new();
        args.insert("rows".into(), Value::U64(6));
        args.insert("cols".into(), Value::U64(10));
        args.insert("dome_fov_deg".into(), Value::F64(180.0));
        args.insert("frame_index".into(), Value::U64(7));
        let res = h
            .living_world_anatomy_op(
                "hypermedia.layout.synchronize_companion_frustum",
                &args,
            )
            .expect("FrameLock morphism");
        let rec = record(&res).unwrap();
        assert_eq!(rec.get("wedge_count"), Some(&Value::I64(60)));
        assert_eq!(rec.get("frame_index"), Some(&Value::I64(7)));
        assert_eq!(rec.get("packet_valid_crc32c"), Some(&Value::Bool(true)));
    }
}

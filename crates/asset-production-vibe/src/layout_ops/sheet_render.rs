//! UI sheet RGBA rasterization and standalone SVG vector projection.

use super::{
    build_theme_tokens, compute_box_colors, compute_box_corner_radius, parse_theme_record,
    rgba_css, xml_escape, LayoutBox, ThemeTokens,
};
use crate::*;
use anyrender::{ImageRenderer, PaintScene};
use kurbo::Shape;

impl AssetHost<'_> {
    /// Renders the resolved layout into a straight-alpha RGBA8 raster artifact.
    pub(super) fn render_ui_sheet(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["layout", "theme", "scale"])?;

        let (viewport_width, viewport_height, boxes, theme, scale) =
            self.parse_render_inputs(a, true)?;

        let rw = (viewport_width * scale).round() as u32;
        let rh = (viewport_height * scale).round() as u32;

        if rw == 0 || rw > 1024 || rh == 0 || rh > 1024 {
            return Err(format!(
                "Render dimensions {rw}x{rh} outside 1..1024 bounds (scale {scale:.2})"
            ));
        }

        let pixels_count = rw as usize * rh as usize;
        let bytes_count = pixels_count * 4;
        self.charge(pixels_count + boxes.len() * 128 + 1024, bytes_count * 2 + 4096)?;

        let mut bytes = vec![0u8; bytes_count];
        let mut renderer = vector_render::VelloCpuImageRenderer::new(rw, rh);

        renderer.render(
            |scene| {
                let bg_color = peniko::Color::from_rgba8(
                    theme.surface_bg[0],
                    theme.surface_bg[1],
                    theme.surface_bg[2],
                    theme.surface_bg[3],
                );
                scene.fill(
                    peniko::Fill::NonZero,
                    kurbo::Affine::IDENTITY,
                    bg_color,
                    None,
                    &kurbo::Rect::new(0.0, 0.0, rw as f64, rh as f64),
                );

                for b in &boxes {
                    let bx = b.x * scale;
                    let by = b.y * scale;
                    let bw = b.width * scale;
                    let bh = b.height * scale;

                    if bw <= 0.0 || bh <= 0.0 {
                        continue;
                    }

                    let cr = compute_box_corner_radius(&b.role, bh, theme.corner_radius) * scale;
                    let (fill_c, border_c, border_w) = compute_box_colors(b, &theme);

                    let rect = kurbo::Rect::new(bx, by, bx + bw, by + bh);
                    let shape: kurbo::BezPath = if cr > 0.0 {
                        kurbo::RoundedRect::new(bx, by, bx + bw, by + bh, cr).into_path(0.1)
                    } else {
                        rect.into_path(0.1)
                    };

                    if b.state == "focus" {
                        let pad = 2.5 * scale;
                        let rx0 = (bx - pad).max(0.0);
                        let ry0 = (by - pad).max(0.0);
                        let rx1 = (bx + bw + pad).max(rx0 + 1.0);
                        let ry1 = (by + bh + pad).max(ry0 + 1.0);
                        let ring_shape = kurbo::RoundedRect::new(rx0, ry0, rx1, ry1, cr + pad);
                        let ring_color = peniko::Color::from_rgba8(
                            theme.focus_ring[0],
                            theme.focus_ring[1],
                            theme.focus_ring[2],
                            theme.focus_ring[3],
                        );
                        scene.stroke(
                            &kurbo::Stroke::new(2.0 * scale),
                            kurbo::Affine::IDENTITY,
                            ring_color,
                            None,
                            &ring_shape,
                        );
                    }

                    if b.state == "hover" {
                        let glow_color = peniko::Color::from_rgba8(
                            theme.accent[0],
                            theme.accent[1],
                            theme.accent[2],
                            110,
                        );
                        scene.stroke(
                            &kurbo::Stroke::new(1.5 * scale),
                            kurbo::Affine::IDENTITY,
                            glow_color,
                            None,
                            &shape,
                        );
                    }

                    let fill_brush =
                        peniko::Color::from_rgba8(fill_c[0], fill_c[1], fill_c[2], fill_c[3]);
                    scene.fill(
                        peniko::Fill::NonZero,
                        kurbo::Affine::IDENTITY,
                        fill_brush,
                        None,
                        &shape,
                    );

                    if border_w > 0.0 {
                        let stroke_brush = peniko::Color::from_rgba8(
                            border_c[0],
                            border_c[1],
                            border_c[2],
                            border_c[3],
                        );
                        scene.stroke(
                            &kurbo::Stroke::new(border_w * scale),
                            kurbo::Affine::IDENTITY,
                            stroke_brush,
                            None,
                            &shape,
                        );
                    }

                    if b.role == "header" {
                        let bottom_line = kurbo::Line::new(
                            kurbo::Point::new(bx, by + bh),
                            kurbo::Point::new(bx + bw, by + bh),
                        );
                        let border_brush = peniko::Color::from_rgba8(
                            theme.panel_border[0],
                            theme.panel_border[1],
                            theme.panel_border[2],
                            theme.panel_border[3],
                        );
                        scene.stroke(
                            &kurbo::Stroke::new(1.0 * scale),
                            kurbo::Affine::IDENTITY,
                            border_brush,
                            None,
                            &bottom_line,
                        );
                    } else if b.role == "sidebar" {
                        let right_line = kurbo::Line::new(
                            kurbo::Point::new(bx + bw, by),
                            kurbo::Point::new(bx + bw, by + bh),
                        );
                        let border_brush = peniko::Color::from_rgba8(
                            theme.panel_border[0],
                            theme.panel_border[1],
                            theme.panel_border[2],
                            theme.panel_border[3],
                        );
                        scene.stroke(
                            &kurbo::Stroke::new(1.0 * scale),
                            kurbo::Affine::IDENTITY,
                            border_brush,
                            None,
                            &right_line,
                        );
                    }

                    if !b.label.is_empty() {
                        let label_h = (7.0 * scale).clamp(2.0, (bh * 0.35).max(2.0));
                        let label_w = ((b.label.len() as f64 * 6.5) * scale)
                            .clamp(8.0, (bw - 16.0 * scale).max(8.0));
                        let (lx, ly) = match b.role.as_str() {
                            "button" | "badge" => {
                                (bx + (bw - label_w) / 2.0, by + (bh - label_h) / 2.0)
                            }
                            "input" => (bx + 12.0 * scale, by + (bh - label_h) / 2.0),
                            "header" => (bx + 16.0 * scale, by + (bh - label_h) / 2.0),
                            _ => (bx + 16.0 * scale, by + 16.0 * scale),
                        };

                        let label_pill = kurbo::RoundedRect::new(
                            lx,
                            ly,
                            lx + label_w,
                            ly + label_h,
                            label_h / 2.0,
                        );

                        let label_c = if b.state == "disabled" {
                            [
                                theme.text_muted[0],
                                theme.text_muted[1],
                                theme.text_muted[2],
                                80,
                            ]
                        } else if b.role == "badge" {
                            [theme.accent[0], theme.accent[1], theme.accent[2], 220]
                        } else {
                            [
                                theme.text_primary[0],
                                theme.text_primary[1],
                                theme.text_primary[2],
                                180,
                            ]
                        };

                        let label_brush = peniko::Color::from_rgba8(
                            label_c[0], label_c[1], label_c[2], label_c[3],
                        );
                        scene.fill(
                            peniko::Fill::NonZero,
                            kurbo::Affine::IDENTITY,
                            label_brush,
                            None,
                            &label_pill,
                        );
                    }
                }
            },
            &mut bytes,
        );

        let pixels = bytes
            .chunks_exact(4)
            .map(|p| {
                let alpha = p[3] as u32;
                if alpha == 0 {
                    [0; 4]
                } else {
                    [
                        (p[0] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        (p[1] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        (p[2] as u32 * 255 + alpha / 2)
                            .checked_div(alpha)
                            .unwrap()
                            .min(255) as u8,
                        p[3],
                    ]
                }
            })
            .collect();

        self.store_layout_rgba(rw, rh, pixels)
    }

    /// Projects the resolved layout into a standalone SVG vector resource with scoped CSS tokens.
    pub(super) fn project_ui_svg(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["layout", "theme", "scale"])?;

        let (viewport_width, viewport_height, boxes, theme, scale) =
            self.parse_render_inputs(a, false)?;

        let svg_w = viewport_width * scale;
        let svg_h = viewport_height * scale;

        let mut svg = String::with_capacity(8192);
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {viewport_width:.1} {viewport_height:.1}\" width=\"{svg_w:.1}\" height=\"{svg_h:.1}\">\n"
        ));

        svg.push_str("  <style>\n");
        for line in theme.css_variables.lines() {
            svg.push_str("    ");
            svg.push_str(line);
            svg.push('\n');
        }
        svg.push_str(
            "    .webizen-viewport { background: var(--webizen-surface-bg); font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; }\n\
                 .webizen-box { transition: all 0.15s ease-in-out; }\n\
                 .role-button { cursor: pointer; user-select: none; }\n\
                 .role-header { font-weight: 700; }\n\
                 .state-hover { filter: brightness(1.12); }\n\
                 .state-focus { filter: drop-shadow(0 0 6px var(--webizen-focus-ring)); }\n\
                 .state-disabled { opacity: 0.45; pointer-events: none; }\n\
             </style>\n",
        );

        let surface_css = rgba_css(theme.surface_bg);
        svg.push_str(&format!(
            "  <rect class=\"webizen-viewport\" width=\"100%\" height=\"100%\" fill=\"{surface_css}\" />\n"
        ));

        for b in &boxes {
            let cr = compute_box_corner_radius(&b.role, b.height, theme.corner_radius);
            let (fill_c, border_c, border_w) = compute_box_colors(b, &theme);

            let fill_css = rgba_css(fill_c);
            let border_css = rgba_css(border_c);

            svg.push_str(&format!(
                "  <g id=\"{}\" class=\"webizen-box role-{} state-{}\">\n",
                xml_escape(&b.id),
                xml_escape(&b.role),
                xml_escape(&b.state)
            ));

            if b.state == "focus" {
                let pad = 3.0;
                let focus_css = rgba_css(theme.focus_ring);
                svg.push_str(&format!(
                    "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.5\" />\n",
                    b.x - pad,
                    b.y - pad,
                    b.width + 2.0 * pad,
                    b.height + 2.0 * pad,
                    cr + pad,
                    cr + pad,
                    focus_css
                ));
            }

            svg.push_str(&format!(
                "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{:.1}\" />\n",
                b.x, b.y, b.width, b.height, cr, cr, fill_css, border_css, border_w
            ));

            if b.role == "header" {
                let border_line_css = rgba_css(theme.panel_border);
                svg.push_str(&format!(
                    "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.0\" />\n",
                    b.x, b.y + b.height, b.x + b.width, b.y + b.height, border_line_css
                ));
            } else if b.role == "sidebar" {
                let border_line_css = rgba_css(theme.panel_border);
                svg.push_str(&format!(
                    "    <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{}\" stroke-width=\"1.0\" />\n",
                    b.x + b.width, b.y, b.x + b.width, b.y + b.height, border_line_css
                ));
            }

            if !b.label.is_empty() {
                let escaped = xml_escape(&b.label);
                let (tx, ty, anchor, font_size, weight, tc) = match b.role.as_str() {
                    "button" => (
                        b.x + b.width / 2.0,
                        b.y + b.height / 2.0,
                        "middle",
                        13.0_f64,
                        "600",
                        theme.text_primary,
                    ),
                    "badge" => (
                        b.x + b.width / 2.0,
                        b.y + b.height / 2.0,
                        "middle",
                        11.0_f64,
                        "700",
                        theme.accent,
                    ),
                    "header" => (
                        b.x + 16.0,
                        b.y + b.height / 2.0,
                        "start",
                        16.0_f64,
                        "700",
                        theme.text_primary,
                    ),
                    "input" => (
                        b.x + 12.0,
                        b.y + b.height / 2.0,
                        "start",
                        13.0_f64,
                        "400",
                        theme.text_muted,
                    ),
                    _ => (
                        b.x + 16.0,
                        b.y + 20.0,
                        "start",
                        14.0_f64,
                        "600",
                        theme.text_primary,
                    ),
                };

                let text_css = rgba_css(if b.state == "disabled" {
                    [tc[0], tc[1], tc[2], 100]
                } else {
                    tc
                });

                svg.push_str(&format!(
                    "    <text x=\"{tx:.1}\" y=\"{ty:.1}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"{font_size:.1}\" font-weight=\"{weight}\" fill=\"{text_css}\" text-anchor=\"{anchor}\" dominant-baseline=\"central\">{escaped}</text>\n"
                ));
            }

            svg.push_str("  </g>\n");
        }

        svg.push_str("</svg>\n");

        let svg_bytes = svg.into_bytes();
        let bytes_len = svg_bytes.len();
        self.charge(bytes_len + boxes.len() * 64 + 512, bytes_len * 2 + 2048)?;

        self.encode("svg", svg_bytes)
    }

    fn parse_render_inputs(
        &self,
        a: &Args,
        auto_fit_raster: bool,
    ) -> Result<(f64, f64, Vec<LayoutBox>, ThemeTokens, f64), String> {
        let layout_rec = record(required(a, "layout")?)?;
        let viewport_width = number(layout_rec, "viewport_width", 1280.0)?;
        let viewport_height = number(layout_rec, "viewport_height", 800.0)?;

        let raw_boxes = list(required(layout_rec, "boxes")?)?;
        let mut boxes = Vec::with_capacity(raw_boxes.len());
        for item_val in raw_boxes {
            let r = record(item_val)?;
            let id = text(required(r, "id")?)?.to_string();
            let role = match r.get("role") {
                Some(v) => text(v)?,
                None => "panel",
            }
            .to_string();
            let label = match r.get("label") {
                Some(v) => text(v)?,
                None => "",
            }
            .to_string();
            let state = match r.get("state") {
                Some(v) => text(v)?,
                None => "default",
            }
            .to_string();
            let x = number(r, "x", 0.0)?;
            let y = number(r, "y", 0.0)?;
            let width = number(r, "width", 0.0)?;
            let height = number(r, "height", 0.0)?;

            boxes.push(LayoutBox {
                id,
                role,
                label,
                state,
                x,
                y,
                width,
                height,
            });
        }

        let theme = match a.get("theme") {
            Some(v) => parse_theme_record(record(v)?)?,
            None => build_theme_tokens("app", "holosonic-dark", [0, 216, 242, 255], 10.0),
        };

        let scale = match a.get("scale") {
            Some(v) => {
                let s = num(v)?;
                if !(0.1..=2.0).contains(&s) {
                    return Err("scale outside 0.1..2.0".into());
                }
                s
            }
            None => {
                if auto_fit_raster {
                    let max_dim = viewport_width.max(viewport_height);
                    if max_dim > 1024.0 {
                        (1024.0 / max_dim).clamp(0.1, 1.0)
                    } else {
                        1.0
                    }
                } else {
                    1.0
                }
            }
        };

        Ok((viewport_width, viewport_height, boxes, theme, scale))
    }

    fn store_layout_rgba(
        &mut self,
        width: u32,
        height: u32,
        pixels: Vec<[u8; 4]>,
    ) -> Result<Value, String> {
        self.workspace.insert(
            Artifact::Rgba(crate::graphics::Rgba {
                width,
                height,
                pixels,
            }),
            self.limits,
        )
    }
}

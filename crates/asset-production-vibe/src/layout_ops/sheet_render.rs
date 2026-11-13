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
        fields(a, &["layout", "theme", "scale", "font"])?;

        let (viewport_width, viewport_height, boxes, theme, scale) =
            self.parse_render_inputs(a, true)?;
        let labels = self.ui_label_outlines(a, &boxes, &theme)?;

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

                for (label_index, b) in boxes.iter().enumerate() {
                    let bx = b.x * scale;
                    let by = b.y * scale;
                    let bw = b.width * scale;
                    let bh = b.height * scale;

                    if bw <= 0.0 || bh <= 0.0 {
                        continue;
                    }

                    let cr = compute_box_corner_radius(&b.role, b.height, theme.corner_radius) * scale;
                    let (fill_c, border_c, border_w) = compute_box_colors(b, &theme);

                    let rect = kurbo::Rect::new(bx, by, bx + bw, by + bh);
                    let shape: kurbo::BezPath = if cr > 0.0 {
                        kurbo::RoundedRect::new(bx, by, bx + bw, by + bh, cr).into_path(0.1)
                    } else {
                        rect.into_path(0.1)
                    };

                    if b.state == "focus" {
                        let pad = 2.5 * scale;
                        let rx0 = (b.x - 2.5).max(0.0) * scale;
                        let ry0 = (b.y - 2.5).max(0.0) * scale;
                        let rx1 = (b.x + b.width + 2.5).max((b.x - 2.5).max(0.0) + 1.0) * scale;
                        let ry1 = (b.y + b.height + 2.5).max((b.y - 2.5).max(0.0) + 1.0) * scale;
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

                    if let Some(label) = &labels[label_index] {
                        let transform = kurbo::Affine::scale(scale);
                        let c = label.color;
                        let brush = peniko::Color::from_rgba8(c[0], c[1], c[2], c[3]);
                        scene.push_clip_layer(peniko::Fill::NonZero, transform, &label.clip);
                        if label.stroke_width > 0.0 {
                            let stroke = kurbo::Stroke::new(label.stroke_width)
                                .with_caps(kurbo::Cap::Round).with_join(kurbo::Join::Round);
                            scene.stroke(&stroke, transform, brush, None, &label.path);
                        } else {
                            scene.fill(peniko::Fill::NonZero, transform, brush, None, &label.path);
                        }
                        scene.pop_layer();
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
        fields(a, &["layout", "theme", "scale", "font"])?;

        let (viewport_width, viewport_height, boxes, theme, scale) =
            self.parse_render_inputs(a, false)?;
        let labels = self.ui_label_outlines(a, &boxes, &theme)?;

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
                 .state-disabled { pointer-events: none; }\n\
             </style>\n",
        );

        let surface_css = rgba_css(theme.surface_bg);
        svg.push_str(&format!(
            "  <rect class=\"webizen-viewport\" width=\"100%\" height=\"100%\" fill=\"{surface_css}\" />\n"
        ));

        for (label_index, b) in boxes.iter().enumerate() {
            if b.width <= 0.0 || b.height <= 0.0 { continue; }
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
                let pad = 2.5;
                let focus_css = rgba_css(theme.focus_ring);
                svg.push_str(&format!(
                    "    <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"{:.1}\" ry=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.0\" />\n",
                    (b.x - pad).max(0.0),
                    (b.y - pad).max(0.0),
                    (b.x + b.width + pad).max((b.x - pad).max(0.0) + 1.0) - (b.x - pad).max(0.0),
                    (b.y + b.height + pad).max((b.y - pad).max(0.0) + 1.0) - (b.y - pad).max(0.0),
                    cr + pad,
                    cr + pad,
                    focus_css
                ));
            }

            if b.state == "hover" {
                let glow_css = rgba_css([theme.accent[0], theme.accent[1], theme.accent[2], 110]);
                svg.push_str(&format!(
                    "    <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.5\" />\n",
                    b.x, b.y, b.width, b.height, cr, cr, glow_css));
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

            if let Some(label) = &labels[label_index] {
                let clip_id = format!("synth-label-clip-{label_index}");
                let c = label.clip;
                let css = rgba_css(label.color);
                let (fill, stroke) = if label.stroke_width > 0.0 {
                    ("none", css.as_str())
                } else { (css.as_str(), "none") };
                svg.push_str(&format!(
                    "    <defs><clipPath id=\"{clip_id}\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>\n",
                    c.x0, c.y0, c.width(), c.height()));
                svg.push_str(&format!(
                    "    <path role=\"img\" aria-label=\"{}\" d=\"{}\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\" clip-path=\"url(#{clip_id})\"><title>{}</title></path>\n",
                    xml_escape(&b.label), label.path.to_svg(), label.stroke_width, xml_escape(&b.label)));
            }

            svg.push_str("  </g>\n");
        }

        svg.push_str("</svg>\n");

        let svg_bytes = svg.into_bytes();
        let bytes_len = svg_bytes.len();
        self.charge(bytes_len + boxes.len() * 64 + 512, bytes_len * 2 + 2048)?;

        self.encode("svg", svg_bytes)
    }

    fn ui_label_outlines(
        &mut self, a: &Args, boxes: &[LayoutBox], theme: &ThemeTokens,
    ) -> Result<Vec<Option<super::label_outline::LabelOutline>>, String> {
        let text_bytes: usize = boxes.iter().map(|b| b.label.len()).sum();
        if boxes.len() > 1024 || text_bytes > 16384 {
            return Err("UI labels exceed 1024 boxes or 16384 UTF-8 bytes".into());
        }
        self.charge(text_bytes * 256 + 1024, text_bytes * 2048 + 4096)?;
        let font = match a.get("font") {
            Some(value) => {
                let key = text(value)?;
                let input = self.inputs.iter().find(|r| r.key == key)
                    .ok_or_else(|| format!("Missing retained UI font input: {key}"))?;
                Some(input.bytes.as_slice())
            }
            None => None,
        };
        super::label_outline::shape_labels(boxes, theme, font)
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

#[cfg(test)]
mod state_parity_tests {
    use super::*;

    #[test]
    fn exported_state_artwork_does_not_apply_a_second_visual_effect() {
        let limits = ToolLimits::default();
        let cancel = std::sync::atomic::AtomicBool::new(false);
        let mut host = AssetHost { workspace: AssetWorkspace::default(), inputs: &[], limits: &limits, cancel: &cancel, work: 0 };
        let boxes = ["disabled", "focus", "hover"].iter().enumerate().map(|(i, state)| Value::Record(BTreeMap::from([
            ("id".into(), Value::String(format!("state-{i}"))),
            ("role".into(), Value::String("button".into())),
            ("label".into(), Value::String("SAVE".into())),
            ("state".into(), Value::String((*state).into())),
            ("x".into(), Value::F64(0.0)), ("y".into(), Value::F64(i as f64 * 42.0)),
            ("width".into(), Value::F64(120.0)), ("height".into(), Value::F64(36.0)),
        ]))).collect();
        let args = BTreeMap::from([("layout".into(), Value::Record(BTreeMap::from([
            ("viewport_width".into(), Value::F64(160.0)), ("viewport_height".into(), Value::F64(140.0)),
            ("boxes".into(), Value::List(boxes)),
        ]))), ("scale".into(), Value::F64(0.5))]);
        let svg = host.project_ui_svg(&args).unwrap();
        let text = std::str::from_utf8(&host.workspace.encoded(&svg).unwrap().bytes).unwrap();
        assert!(text.contains("pointer-events: none"));
        assert!(!text.contains("opacity: 0.45"));
        assert!(!text.contains("filter:"));
        assert!(text.contains("stroke-width=\"2.0\""));
        assert!(text.contains("stroke-width=\"1.5\""));
        assert!(text.contains("aria-label=\"SAVE\""));
        let image = host.render_ui_sheet(&args).unwrap();
        let Artifact::Rgba(rgba) = host.workspace.get(&image).unwrap() else { panic!("expected raster"); };
        assert_eq!((rgba.width, rgba.height), (80, 70));
    }
}

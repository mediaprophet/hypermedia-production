use super::*;
use crate::*;
use std::collections::BTreeMap;
use std::fmt::Write;

struct ChartSeriesSpec {
    name: String,
    values: Vec<f64>,
    color: String,
}

const CHART_PALETTE: &[&str] = &[
    "#3b82f6", // Indigo Blue
    "#10b981", // Emerald Green
    "#f59e0b", // Amber Orange
    "#ef4444", // Crimson Red
    "#8b5cf6", // Violet Purple
    "#ec4899", // Fuchsia Pink
    "#06b6d4", // Cyan
    "#14b8a6", // Teal
];

impl AssetHost<'_> {
    // ------------------------------------------------------------------------
    // 4. Standalone SVG Data Discourse Chart Rendering
    // ------------------------------------------------------------------------
    pub(super) fn render_chart_svg(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "chart_type",
                "type",
                "title",
                "width",
                "height",
                "labels",
                "series",
                "values",
                "artifact_id",
                "id",
            ],
        )?;

        let chart_type = a
            .get("chart_type")
            .or_else(|| a.get("type"))
            .map(text)
            .transpose()?
            .unwrap_or("bar");

        if !matches!(chart_type, "bar" | "line" | "area" | "scatter") {
            return Err(format!(
                "Unknown chart_type '{chart_type}'; expected bar|line|area|scatter"
            ));
        }

        let title = a
            .get("title")
            .map(text)
            .transpose()?
            .unwrap_or("Data Discourse Chart");

        let width = a
            .get("width")
            .map(|v| integer(v, "width"))
            .transpose()?
            .unwrap_or(640) as u32;
        if !(120..=4096).contains(&width) {
            return Err("width outside 120..=4096".into());
        }

        let height = a
            .get("height")
            .map(|v| integer(v, "height"))
            .transpose()?
            .unwrap_or(400) as u32;
        if !(120..=4096).contains(&height) {
            return Err("height outside 120..=4096".into());
        }

        // Parse series data
        let mut series_list: Vec<ChartSeriesSpec> = Vec::new();

        if let Some(s_val) = a.get("series") {
            let items = list(s_val)?;
            if !items.is_empty() {
                if let Ok(first_rec) = record(&items[0]) {
                    let _ = first_rec;
                    for (idx, item) in items.iter().enumerate() {
                        let r = record(item)?;
                        let default_name = format!("Series {}", idx + 1);
                        let name = r
                            .get("name")
                            .map(text)
                            .transpose()?
                            .unwrap_or(&default_name)
                            .to_string();
                        let color = r
                            .get("color")
                            .map(text)
                            .transpose()?
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                CHART_PALETTE[idx % CHART_PALETTE.len()].to_string()
                            });
                        let val_list = list(required(r, "values")?)?;
                        let mut values = Vec::with_capacity(val_list.len());
                        for v in val_list {
                            values.push(num(v)?);
                        }
                        series_list.push(ChartSeriesSpec {
                            name,
                            values,
                            color,
                        });
                    }
                } else {
                    let mut values = Vec::with_capacity(items.len());
                    for v in items {
                        values.push(num(v)?);
                    }
                    series_list.push(ChartSeriesSpec {
                        name: "Series 1".to_string(),
                        values,
                        color: CHART_PALETTE[0].to_string(),
                    });
                }
            }
        } else if let Some(v_val) = a.get("values") {
            let items = list(v_val)?;
            let mut values = Vec::with_capacity(items.len());
            for v in items {
                values.push(num(v)?);
            }
            series_list.push(ChartSeriesSpec {
                name: "Series 1".to_string(),
                values,
                color: CHART_PALETTE[0].to_string(),
            });
        } else {
            series_list.push(ChartSeriesSpec {
                name: "Series 1".to_string(),
                values: vec![12.0, 28.0, 19.0, 45.0, 33.0],
                color: CHART_PALETTE[0].to_string(),
            });
        }

        let total_points: usize = series_list.iter().map(|s| s.values.len()).sum();
        if total_points == 0 {
            return Err("Chart requires at least one data point".into());
        }

        let n_categories = series_list
            .iter()
            .map(|s| s.values.len())
            .max()
            .unwrap_or(1);

        // Parse labels
        let mut labels: Vec<String> = Vec::new();
        if let Some(l_val) = a.get("labels") {
            for l in list(l_val)? {
                labels.push(text(l)?.to_string());
            }
        }
        while labels.len() < n_categories {
            labels.push((labels.len() + 1).to_string());
        }

        // Extents
        let mut min_val = f64::INFINITY;
        let mut max_val = f64::NEG_INFINITY;
        for s in &series_list {
            for &v in &s.values {
                if v < min_val {
                    min_val = v;
                }
                if v > max_val {
                    max_val = v;
                }
            }
        }

        let y_min = if matches!(chart_type, "bar" | "area") {
            0.0f64.min(min_val)
        } else {
            min_val
        };

        let mut y_max = 1.0f64.max(max_val);
        if y_max > y_min {
            y_max = y_min + (y_max - y_min) * 1.1;
        } else {
            y_max = y_min + 1.0;
        }
        let v_range = (y_max - y_min).max(1e-6);

        // SVG plotting dimensions
        let margin_top = 50.0f64;
        let margin_bottom = 50.0f64;
        let margin_left = 60.0f64;
        let margin_right = if series_list.len() > 1 {
            135.0f64
        } else {
            30.0f64
        };
        let pw = (width as f64 - margin_left - margin_right).max(10.0);
        let ph = (height as f64 - margin_top - margin_bottom).max(10.0);

        let mut svg = String::new();
        let _ = write!(
            svg,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\">\n"
        );
        svg.push_str("  <style>\n");
        svg.push_str("    .bg { fill: #0f172a; }\n");
        svg.push_str("    .plot-bg { fill: #1e293b; }\n");
        svg.push_str("    .grid { stroke: #334155; stroke-dasharray: 4,4; stroke-width: 1; }\n");
        svg.push_str("    .axis { stroke: #64748b; stroke-width: 1.5; }\n");
        svg.push_str("    .title { font-family: system-ui, -apple-system, sans-serif; font-size: 16px; font-weight: 600; fill: #f8fafc; text-anchor: middle; }\n");
        svg.push_str("    .axis-label { font-family: system-ui, -apple-system, sans-serif; font-size: 11px; fill: #94a3b8; }\n");
        svg.push_str("    .legend-text { font-family: system-ui, -apple-system, sans-serif; font-size: 12px; fill: #cbd5e1; }\n");
        svg.push_str("  </style>\n");

        // Backgrounds
        let _ = write!(
            svg,
            "  <rect width=\"{width}\" height=\"{height}\" rx=\"8\" class=\"bg\"/>\n"
        );
        let _ = write!(
            svg,
            "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" rx=\"4\" class=\"plot-bg\"/>\n",
            margin_left, margin_top, pw, ph
        );

        // Title
        let _ = write!(
            svg,
            "  <text x=\"{:.1}\" y=\"30\" class=\"title\">{}</text>\n",
            width as f64 * 0.5,
            escape_xml(title)
        );

        // Y-axis grid lines and tick labels (5 intervals)
        for k_idx in 0..=4 {
            let frac = k_idx as f64 / 4.0;
            let tick_y = margin_top + ph - frac * ph;
            let tick_val = y_min + frac * v_range;
            let tick_text = if tick_val.abs() >= 1000.0 || (tick_val.abs() < 0.01 && tick_val != 0.0) {
                format!("{:.1e}", tick_val)
            } else if tick_val.fract().abs() < 1e-4 {
                format!("{:.0}", tick_val)
            } else {
                format!("{:.1}", tick_val)
            };

            let _ = write!(
                svg,
                "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" class=\"grid\"/>\n",
                margin_left, tick_y, margin_left + pw, tick_y
            );
            let _ = write!(
                svg,
                "  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" class=\"axis-label\">{}</text>\n",
                margin_left - 8.0,
                tick_y + 4.0,
                tick_text
            );
        }

        // Axes lines
        let _ = write!(
            svg,
            "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" class=\"axis\"/>\n",
            margin_left,
            margin_top + ph,
            margin_left + pw,
            margin_top + ph
        );
        let _ = write!(
            svg,
            "  <line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" class=\"axis\"/>\n",
            margin_left,
            margin_top,
            margin_left,
            margin_top + ph
        );

        // X-axis category labels
        let slot_w = pw / n_categories as f64;
        for (c, label) in labels.iter().enumerate().take(n_categories) {
            let label_x = margin_left + (c as f64 + 0.5) * slot_w;
            let label_y = margin_top + ph + 20.0;
            let _ = write!(
                svg,
                "  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" class=\"axis-label\">{}</text>\n",
                label_x,
                label_y,
                escape_xml(label)
            );
        }

        // Data rendering per chart_type
        match chart_type {
            "bar" => {
                let group_w = slot_w * 0.75;
                let num_series = series_list.len().max(1) as f64;
                let bar_w = (group_w / num_series).max(1.0);
                let pad = (slot_w - group_w) * 0.5;

                for c in 0..n_categories {
                    for (s_idx, s) in series_list.iter().enumerate() {
                        let val = s.values.get(c).copied().unwrap_or(0.0);
                        let bar_h = (((val - y_min) / v_range) * ph).max(0.0);
                        let bar_x = margin_left + c as f64 * slot_w + pad + s_idx as f64 * bar_w;
                        let bar_y = margin_top + ph - bar_h;
                        let actual_w = (bar_w - 1.5).max(1.0);

                        let _ = write!(
                            svg,
                            "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{}\" rx=\"2\"/>\n",
                            bar_x, bar_y, actual_w, bar_h, s.color
                        );
                    }
                }
            }
            "line" => {
                for s in &series_list {
                    let mut pts = String::new();
                    let mut circles = String::new();

                    for c in 0..n_categories {
                        let val = s.values.get(c).copied().unwrap_or(0.0);
                        let px = margin_left
                            + if n_categories > 1 {
                                c as f64 / (n_categories - 1) as f64 * pw
                            } else {
                                pw * 0.5
                            };
                        let py = margin_top + ph - (((val - y_min) / v_range) * ph);

                        let _ = write!(pts, "{:.1},{:.1} ", px, py);
                        let _ = write!(
                            circles,
                            "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"{}\" stroke=\"#0f172a\" stroke-width=\"1.5\"/>\n",
                            px, py, s.color
                        );
                    }

                    let _ = write!(
                        svg,
                        "  <polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
                        pts.trim_end(),
                        s.color
                    );
                    svg.push_str(&circles);
                }
            }
            "area" => {
                let baseline_y = margin_top + ph - (((0.0f64.clamp(y_min, y_max) - y_min) / v_range) * ph);

                for s in &series_list {
                    let mut pts = String::new();
                    let mut first_x = margin_left;
                    let mut last_x = margin_left + pw;

                    for c in 0..n_categories {
                        let val = s.values.get(c).copied().unwrap_or(0.0);
                        let px = margin_left
                            + if n_categories > 1 {
                                c as f64 / (n_categories - 1) as f64 * pw
                            } else {
                                pw * 0.5
                            };
                        let py = margin_top + ph - (((val - y_min) / v_range) * ph);

                        if c == 0 {
                            first_x = px;
                        }
                        if c == n_categories - 1 {
                            last_x = px;
                        }

                        let _ = write!(pts, "{:.1},{:.1} ", px, py);
                    }

                    let _ = write!(
                        svg,
                        "  <polygon points=\"{} {:.1},{:.1} {:.1},{:.1}\" fill=\"{}\" fill-opacity=\"0.25\"/>\n",
                        pts.trim_end(),
                        last_x,
                        baseline_y,
                        first_x,
                        baseline_y,
                        s.color
                    );
                    let _ = write!(
                        svg,
                        "  <polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.5\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
                        pts.trim_end(),
                        s.color
                    );
                }
            }
            "scatter" => {
                for s in &series_list {
                    for c in 0..n_categories {
                        let val = s.values.get(c).copied().unwrap_or(0.0);
                        let px = margin_left
                            + if n_categories > 1 {
                                c as f64 / (n_categories - 1) as f64 * pw
                            } else {
                                pw * 0.5
                            };
                        let py = margin_top + ph - (((val - y_min) / v_range) * ph);

                        let _ = write!(
                            svg,
                            "  <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\" fill=\"{}\" fill-opacity=\"0.85\" stroke=\"#ffffff\" stroke-width=\"1\"/>\n",
                            px, py, s.color
                        );
                    }
                }
            }
            _ => unreachable!(),
        }

        // Legend if multiple series
        if series_list.len() > 1 {
            let legend_x = margin_left + pw + 15.0;
            let mut legend_y = margin_top + 10.0;
            for s in &series_list {
                let _ = write!(
                    svg,
                    "  <rect x=\"{:.1}\" y=\"{:.1}\" width=\"12\" height=\"12\" rx=\"2\" fill=\"{}\"/>\n",
                    legend_x,
                    legend_y - 10.0,
                    s.color
                );
                let _ = write!(
                    svg,
                    "  <text x=\"{:.1}\" y=\"{:.1}\" class=\"legend-text\">{}</text>\n",
                    legend_x + 18.0,
                    legend_y,
                    escape_xml(&s.name)
                );
                legend_y += 20.0;
            }
        }

        svg.push_str("</svg>\n");

        let svg_bytes = svg.into_bytes();
        let byte_len = svg_bytes.len();
        self.charge(total_points * 64 + 512, byte_len * 2 + 1024)?;

        let encoded = self.encode("svg", svg_bytes)?;

        let mut rec = match encoded {
            Value::Record(r) => r,
            _ => BTreeMap::new(),
        };

        let id_str = a
            .get("artifact_id")
            .or_else(|| a.get("id"))
            .and_then(|v| text(v).ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                let h = rec
                    .get("handle")
                    .and_then(|v| match v {
                        Value::U64(n) => Some(*n),
                        Value::I64(n) => Some(*n as u64),
                        _ => None,
                    })
                    .unwrap_or(0);
                format!("discourse-svg-{h}")
            });

        rec.insert("id".into(), Value::String(id_str.clone()));
        rec.insert("resource".into(), Value::String(id_str));
        rec.insert("kind".into(), Value::String("svg".into()));
        rec.insert("chart_type".into(), Value::String(chart_type.to_string()));
        rec.insert("series_count".into(), Value::I64(series_list.len() as i64));
        rec.insert("point_count".into(), Value::I64(total_points as i64));
        rec.insert("byte_len".into(), Value::I64(byte_len as i64));

        Ok(Value::Record(rec))
    }
}

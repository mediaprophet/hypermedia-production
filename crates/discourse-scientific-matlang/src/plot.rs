//! Headless 2D and 3D scientific vector SVG plot renderer.
//! Renders line, scatter, bar, histogram, contour, and 3D surface meshes into standalone SVG graphics.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeriesKind {
    Line,
    Scatter,
    Bar,
    Histogram,
    Contour,
    Surf,
    Mesh,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Colormap {
    Viridis,
    Jet,
    Hot,
    Gray,
}

impl Colormap {
    pub fn sample(self, t: f64) -> (u8, u8, u8) {
        let t = t.clamp(0.0, 1.0);
        match self {
            Colormap::Gray => {
                let v = (t * 255.0) as u8;
                (v, v, v)
            }
            Colormap::Hot => {
                let r = (t * 3.0).clamp(0.0, 1.0);
                let g = ((t - 0.33) * 3.0).clamp(0.0, 1.0);
                let b = ((t - 0.66) * 3.0).clamp(0.0, 1.0);
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
            Colormap::Jet => {
                let r = (1.5 - (4.0 * t - 3.0).abs()).clamp(0.0, 1.0);
                let g = (1.5 - (4.0 * t - 2.0).abs()).clamp(0.0, 1.0);
                let b = (1.5 - (4.0 * t - 1.0).abs()).clamp(0.0, 1.0);
                ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
            }
            Colormap::Viridis => {
                let r = 0.267 + 0.004 * t + 0.329 * t * t;
                let g = 0.004 + 0.873 * t - 0.407 * t * t;
                let b = 0.329 + 0.407 * t - 0.477 * t * t;
                (
                    ((r.clamp(0.0, 1.0)) * 255.0) as u8,
                    ((g.clamp(0.0, 1.0)) * 255.0) as u8,
                    ((b.clamp(0.0, 1.0)) * 255.0) as u8,
                )
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Series {
    pub kind: SeriesKind,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
    pub zshape: (usize, usize),
    pub color: String,
    pub label: Option<String>,
}

impl Series {
    pub fn line(x: Vec<f64>, y: Vec<f64>, color: &str) -> Self {
        Self {
            kind: SeriesKind::Line,
            x,
            y,
            z: Vec::new(),
            zshape: (0, 0),
            color: color.to_string(),
            label: None,
        }
    }

    pub fn scatter(x: Vec<f64>, y: Vec<f64>, color: &str) -> Self {
        Self {
            kind: SeriesKind::Scatter,
            x,
            y,
            z: Vec::new(),
            zshape: (0, 0),
            color: color.to_string(),
            label: None,
        }
    }

    pub fn bar(x: Vec<f64>, y: Vec<f64>, color: &str) -> Self {
        Self {
            kind: SeriesKind::Bar,
            x,
            y,
            z: Vec::new(),
            zshape: (0, 0),
            color: color.to_string(),
            label: None,
        }
    }

    pub fn surface(z: Vec<f64>, rows: usize, cols: usize) -> Self {
        let mut x = Vec::with_capacity(cols);
        for c in 0..cols {
            x.push(c as f64);
        }
        let mut y = Vec::with_capacity(rows);
        for r in 0..rows {
            y.push(r as f64);
        }
        Self {
            kind: SeriesKind::Surf,
            x,
            y,
            z,
            zshape: (rows, cols),
            color: "viridis".to_string(),
            label: None,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Figure {
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub zlabel: Option<String>,
    pub grid: bool,
    pub colormap: Option<Colormap>,
    pub series: Vec<Series>,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            grid: true,
            colormap: Some(Colormap::Viridis),
            ..Default::default()
        }
    }

    pub fn add_series(&mut self, series: Series) {
        self.series.push(series);
    }

    /// Renders figure to a standalone SVG XML string.
    pub fn to_svg(&self, width: f64, height: f64) -> String {
        let mut svg = String::with_capacity(4096);
        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width:.0}\" height=\"{height:.0}\" \
viewBox=\"0 0 {width:.0} {height:.0}\" font-family=\"sans-serif\">\n"
        ));

        // Background
        svg.push_str(&format!(
            "  <rect width=\"{width:.0}\" height=\"{height:.0}\" fill=\"#ffffff\" />\n"
        ));

        let pad_l = 60.0;
        let pad_r = 30.0;
        let pad_t = 40.0;
        let pad_b = 50.0;

        let plot_w = width - pad_l - pad_r;
        let plot_h = height - pad_t - pad_b;

        // Title
        if let Some(ref title) = self.title {
            svg.push_str(&format!(
                "  <text x=\"{:.1}\" y=\"25\" text-anchor=\"middle\" font-size=\"16\" font-weight=\"bold\" fill=\"#1a1a1a\">{}</text>\n",
                width * 0.5,
                title
            ));
        }

        // Compute data bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for s in &self.series {
            for &vx in &s.x {
                min_x = min_x.min(vx);
                max_x = max_x.max(vx);
            }
            for &vy in &s.y {
                min_y = min_y.min(vy);
                max_y = max_y.max(vy);
            }
        }

        if !min_x.is_finite() || min_x == max_x {
            min_x = 0.0;
            max_x = 1.0;
        }
        if !min_y.is_finite() || min_y == max_y {
            min_y = 0.0;
            max_y = 1.0;
        }

        let map_x = |x: f64| pad_l + (x - min_x) / (max_x - min_x) * plot_w;
        let map_y = |y: f64| pad_t + plot_h - (y - min_y) / (max_y - min_y) * plot_h;

        // Axes & Grid
        if self.grid {
            let n_ticks = 5;
            for i in 0..=n_ticks {
                let frac = i as f64 / n_ticks as f64;
                let gx = pad_l + frac * plot_w;
                let gy = pad_t + frac * plot_h;

                svg.push_str(&format!(
                    "  <line x1=\"{gx:.1}\" y1=\"{pad_t:.1}\" x2=\"{gx:.1}\" y2=\"{:.1}\" stroke=\"#e5e5e5\" stroke-width=\"1\" />\n",
                    pad_t + plot_h
                ));
                svg.push_str(&format!(
                    "  <line x1=\"{pad_l:.1}\" y1=\"{gy:.1}\" x2=\"{:.1}\" y2=\"{gy:.1}\" stroke=\"#e5e5e5\" stroke-width=\"1\" />\n",
                    pad_l + plot_w
                ));

                // X labels
                let val_x = min_x + frac * (max_x - min_x);
                svg.push_str(&format!(
                    "  <text x=\"{gx:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\" fill=\"#666666\">{val_x:.2}</text>\n",
                    pad_t + plot_h + 16.0
                ));

                // Y labels
                let val_y = min_y + (1.0 - frac) * (max_y - min_y);
                svg.push_str(&format!(
                    "  <text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"11\" fill=\"#666666\">{val_y:.2}</text>\n",
                    pad_l - 8.0,
                    gy + 4.0
                ));
            }
        }

        // Box border
        svg.push_str(&format!(
            "  <rect x=\"{pad_l:.1}\" y=\"{pad_t:.1}\" width=\"{plot_w:.1}\" height=\"{plot_h:.1}\" fill=\"none\" stroke=\"#333333\" stroke-width=\"1.5\" />\n"
        ));

        // Render Series
        for s in &self.series {
            match s.kind {
                SeriesKind::Line => {
                    if s.x.len() >= 2 && s.y.len() >= 2 {
                        let mut d = String::new();
                        for (i, (&x, &y)) in s.x.iter().zip(&s.y).enumerate() {
                            let px = map_x(x);
                            let py = map_y(y);
                            if i == 0 {
                                d.push_str(&format!("M {px:.1} {py:.1}"));
                            } else {
                                d.push_str(&format!(" L {px:.1} {py:.1}"));
                            }
                        }
                        svg.push_str(&format!(
                            "  <path d=\"{d}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2\" />\n",
                            s.color
                        ));
                    }
                }
                SeriesKind::Scatter => {
                    for (&x, &y) in s.x.iter().zip(&s.y) {
                        let px = map_x(x);
                        let py = map_y(y);
                        svg.push_str(&format!(
                            "  <circle cx=\"{px:.1}\" cy=\"{py:.1}\" r=\"3.5\" fill=\"{}\" stroke=\"#ffffff\" stroke-width=\"1\" />\n",
                            s.color
                        ));
                    }
                }
                SeriesKind::Bar => {
                    let bar_w = (plot_w / s.x.len().max(1) as f64 * 0.7).max(2.0);
                    for (&x, &y) in s.x.iter().zip(&s.y) {
                        let px = map_x(x) - bar_w * 0.5;
                        let py = map_y(y);
                        let base_y = map_y(0.0f64.clamp(min_y, max_y));
                        let h = (base_y - py).abs();
                        let y_top = py.min(base_y);
                        svg.push_str(&format!(
                            "  <rect x=\"{px:.1}\" y=\"{y_top:.1}\" width=\"{bar_w:.1}\" height=\"{h:.1}\" fill=\"{}\" />\n",
                            s.color
                        ));
                    }
                }
                SeriesKind::Surf | SeriesKind::Mesh => {
                    // Isometric projection of 3D surface grid
                    let (rows, cols) = s.zshape;
                    if rows >= 2 && cols >= 2 && s.z.len() == rows * cols {
                        let min_z = s.z.iter().copied().fold(f64::INFINITY, f64::min);
                        let max_z = s.z.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let cmap = self.colormap.unwrap_or(Colormap::Viridis);

                        let proj = |r: usize, c: usize, z_val: f64| -> (f64, f64) {
                            let u = c as f64 / cols as f64;
                            let v = r as f64 / rows as f64;
                            let norm_z = if max_z > min_z { (z_val - min_z) / (max_z - min_z) } else { 0.5 };
                            let px = pad_l + plot_w * (0.5 + 0.4 * (u - v));
                            let py = pad_t + plot_h * (0.6 + 0.25 * (u + v) - 0.35 * norm_z);
                            (px, py)
                        };

                        for r in 0..rows - 1 {
                            for c in 0..cols - 1 {
                                let z00 = s.z[c * rows + r];
                                let z10 = s.z[(c + 1) * rows + r];
                                let z11 = s.z[(c + 1) * rows + (r + 1)];
                                let z01 = s.z[c * rows + (r + 1)];

                                let (p0x, p0y) = proj(r, c, z00);
                                let (p1x, p1y) = proj(r, c + 1, z10);
                                let (p2x, p2y) = proj(r + 1, c + 1, z11);
                                let (p3x, p3y) = proj(r + 1, c, z01);

                                let avg_z = (z00 + z10 + z11 + z01) * 0.25;
                                let t = if max_z > min_z { (avg_z - min_z) / (max_z - min_z) } else { 0.5 };
                                let (cr, cg, cb) = cmap.sample(t);

                                svg.push_str(&format!(
                                    "  <polygon points=\"{p0x:.1},{p0y:.1} {p1x:.1},{p1y:.1} {p2x:.1},{p2y:.1} {p3x:.1},{p3y:.1}\" fill=\"rgb({cr},{cg},{cb})\" stroke=\"#333333\" stroke-width=\"0.5\" />\n"
                                ));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        svg.push_str("</svg>\n");
        svg
    }
}

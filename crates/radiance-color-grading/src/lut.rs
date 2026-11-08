//! Color lookup tables: `.cube` (1D, 3D, or 1D shaper + 3D cube) and Autodesk `.3dl`,
//! with tetrahedral and trilinear 3D interpolation.
//!
//! Grounded in the Cube LUT Specification 1.0 (Adobe) and Autodesk Flame/Lustre `.3dl` format.

use std::fmt::Write as _;

/// A per-channel 1D lookup table.
#[derive(Clone, Debug, PartialEq)]
pub struct Lut1d {
    pub domain_min: [f32; 3],
    pub domain_max: [f32; 3],
    pub data: Vec<[f32; 3]>,
}

impl Lut1d {
    pub fn identity(size: usize) -> Self {
        let n = (size.saturating_sub(1)).max(1) as f32;
        Lut1d {
            domain_min: [0.0; 3],
            domain_max: [1.0; 3],
            data: (0..size).map(|i| [i as f32 / n; 3]).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn apply(&self, rgb: [f32; 3]) -> [f32; 3] {
        let n = self.data.len().saturating_sub(1);
        if n == 0 {
            return rgb;
        }
        let mut o = [0.0; 3];
        for c in 0..3 {
            let span = self.domain_max[c] - self.domain_min[c];
            let t = if span.abs() > 1e-9 {
                ((rgb[c] - self.domain_min[c]) / span).clamp(0.0, 1.0) * n as f32
            } else {
                0.0
            };
            let i = (t as usize).min(n.saturating_sub(1));
            let f = t - i as f32;
            o[c] = self.data[i][c] + (self.data[(i + 1).min(n)][c] - self.data[i][c]) * f;
        }
        o
    }
}

/// A 3D cube lookup table, `size³` entries, red fastest.
#[derive(Clone, Debug, PartialEq)]
pub struct Lut3d {
    pub title: String,
    pub size: usize,
    pub domain_min: [f32; 3],
    pub domain_max: [f32; 3],
    pub data: Vec<[f32; 3]>,
}

impl Lut3d {
    pub fn identity(size: usize) -> Self {
        Self::from_fn(size, |c| c)
    }

    pub fn from_fn(size: usize, f: impl Fn([f32; 3]) -> [f32; 3]) -> Self {
        let n = (size.saturating_sub(1)).max(1) as f32;
        let mut data = Vec::with_capacity(size * size * size);
        for b in 0..size {
            for g in 0..size {
                for r in 0..size {
                    data.push(f([r as f32 / n, g as f32 / n, b as f32 / n]));
                }
            }
        }
        Lut3d {
            title: String::new(),
            size,
            domain_min: [0.0; 3],
            domain_max: [1.0; 3],
            data,
        }
    }

    #[inline]
    pub fn at(&self, r: usize, g: usize, b: usize) -> [f32; 3] {
        self.data[r + g * self.size + b * self.size * self.size]
    }

    #[inline]
    fn coord(&self, v: f32, c: usize) -> f32 {
        let span = self.domain_max[c] - self.domain_min[c];
        if span.abs() > 1e-9 {
            ((v - self.domain_min[c]) / span).clamp(0.0, 1.0) * (self.size - 1) as f32
        } else {
            0.0
        }
    }

    /// Tetrahedral 3D interpolation.
    pub fn apply(&self, rgb: [f32; 3]) -> [f32; 3] {
        let n = self.size;
        if n < 2 {
            return rgb;
        }
        let (x, y, z) = (
            self.coord(rgb[0], 0),
            self.coord(rgb[1], 1),
            self.coord(rgb[2], 2),
        );
        let (r0, g0, b0) = (
            (x as usize).min(n - 2),
            (y as usize).min(n - 2),
            (z as usize).min(n - 2),
        );
        let (fr, fg, fb) = (x - r0 as f32, y - g0 as f32, z - b0 as f32);
        let c = |dr: usize, dg: usize, db: usize| self.at(r0 + dr, g0 + dg, b0 + db);
        let c000 = c(0, 0, 0);
        let c111 = c(1, 1, 1);

        let (ca, cb, w) = if fr > fg {
            if fg > fb {
                (c(1, 0, 0), c(1, 1, 0), [1.0 - fr, fr - fg, fg - fb, fb])
            } else if fr > fb {
                (c(1, 0, 0), c(1, 0, 1), [1.0 - fr, fr - fb, fb - fg, fg])
            } else {
                (c(0, 0, 1), c(1, 0, 1), [1.0 - fb, fb - fr, fr - fg, fg])
            }
        } else if fb > fg {
            (c(0, 0, 1), c(0, 1, 1), [1.0 - fb, fb - fg, fg - fr, fr])
        } else if fb > fr {
            (c(0, 1, 0), c(0, 1, 1), [1.0 - fg, fg - fb, fb - fr, fr])
        } else {
            (c(0, 1, 0), c(1, 1, 0), [1.0 - fg, fg - fr, fr - fb, fb])
        };

        let mut o = [0.0; 3];
        for k in 0..3 {
            o[k] = w[0] * c000[k] + w[1] * ca[k] + w[2] * cb[k] + w[3] * c111[k];
        }
        o
    }

    /// Trilinear 3D interpolation.
    pub fn apply_trilinear(&self, rgb: [f32; 3]) -> [f32; 3] {
        let n = self.size;
        if n < 2 {
            return rgb;
        }
        let (x, y, z) = (
            self.coord(rgb[0], 0),
            self.coord(rgb[1], 1),
            self.coord(rgb[2], 2),
        );
        let (r0, g0, b0) = (
            (x as usize).min(n - 2),
            (y as usize).min(n - 2),
            (z as usize).min(n - 2),
        );
        let (fr, fg, fb) = (x - r0 as f32, y - g0 as f32, z - b0 as f32);
        let mut out = [0f32; 3];
        for (k, o) in out.iter_mut().enumerate() {
            let c = |dr, dg, db| self.at(r0 + dr, g0 + dg, b0 + db)[k];
            let x00 = c(0, 0, 0) + (c(1, 0, 0) - c(0, 0, 0)) * fr;
            let x10 = c(0, 1, 0) + (c(1, 1, 0) - c(0, 1, 0)) * fr;
            let x01 = c(0, 0, 1) + (c(1, 0, 1) - c(0, 0, 1)) * fr;
            let x11 = c(0, 1, 1) + (c(1, 1, 1) - c(0, 1, 1)) * fr;
            let y0 = x00 + (x10 - x00) * fg;
            let y1 = x01 + (x11 - x01) * fg;
            *o = y0 + (y1 - y0) * fb;
        }
        out
    }

    /// Parse a `.cube` file string.
    pub fn parse_cube(text: &str) -> Result<Lut3d, String> {
        let l = Lut::parse_cube(text)?;
        match (l.shaper, l.cube) {
            (None, Some(c)) => Ok(c),
            (Some(_), _) => Err("File contains a 1D shaper; parse as Lut".into()),
            (None, None) => Err("No 3D table found".into()),
        }
    }

    /// Serialize this 3D LUT to `.cube` format string.
    pub fn to_cube(&self) -> String {
        Lut {
            title: self.title.clone(),
            shaper: None,
            cube: Some(self.clone()),
        }
        .to_cube()
    }
}

/// A LUT container with optional 1D shaper and optional 3D cube.
#[derive(Clone, Debug, PartialEq)]
pub struct Lut {
    pub title: String,
    pub shaper: Option<Lut1d>,
    pub cube: Option<Lut3d>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LutFormat {
    Cube,
    ThreeDl,
}

impl Lut {
    pub fn from_cube(cube: Lut3d) -> Self {
        Lut {
            title: cube.title.clone(),
            shaper: None,
            cube: Some(cube),
        }
    }

    pub fn apply(&self, rgb: [f32; 3]) -> [f32; 3] {
        let after_shaper = match &self.shaper {
            Some(s) => s.apply(rgb),
            None => rgb,
        };
        match &self.cube {
            Some(c) => c.apply(after_shaper),
            None => after_shaper,
        }
    }

    /// Parse a `.cube` text representation.
    pub fn parse_cube(text: &str) -> Result<Lut, String> {
        let mut title = String::new();
        let mut size1d = None;
        let mut size3d = None;
        let mut domain_min = [0.0f32; 3];
        let mut domain_max = [1.0f32; 3];
        let mut numbers: Vec<[f32; 3]> = Vec::new();

        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let mut parts = line.split_whitespace();
            let Some(first) = parts.next() else { continue };

            if first.eq_ignore_ascii_case("TITLE") {
                title = line[5..].trim().trim_matches('"').to_string();
            } else if first.eq_ignore_ascii_case("LUT_1D_SIZE") {
                let s: usize = parts
                    .next()
                    .and_then(|t| t.parse().ok())
                    .ok_or("invalid LUT_1D_SIZE")?;
                size1d = Some(s);
            } else if first.eq_ignore_ascii_case("LUT_3D_SIZE") {
                let s: usize = parts
                    .next()
                    .and_then(|t| t.parse().ok())
                    .ok_or("invalid LUT_3D_SIZE")?;
                size3d = Some(s);
            } else if first.eq_ignore_ascii_case("DOMAIN_MIN") {
                let nums = parse_nums3(&mut parts)?;
                domain_min = nums;
            } else if first.eq_ignore_ascii_case("DOMAIN_MAX") {
                let nums = parse_nums3(&mut parts)?;
                domain_max = nums;
            } else if let Ok(r) = first.parse::<f32>() {
                let g: f32 = parts
                    .next()
                    .and_then(|t| t.parse().ok())
                    .ok_or("missing G")?;
                let b: f32 = parts
                    .next()
                    .and_then(|t| t.parse().ok())
                    .ok_or("missing B")?;
                numbers.push([r, g, b]);
            }
        }

        let mut offset = 0;
        let shaper = if let Some(n1) = size1d {
            if numbers.len() < offset + n1 {
                return Err("Insufficient 1D data".into());
            }
            let data = numbers[offset..offset + n1].to_vec();
            offset += n1;
            Some(Lut1d {
                domain_min: [0.0; 3],
                domain_max: [1.0; 3],
                data,
            })
        } else {
            None
        };

        let cube = if let Some(n3) = size3d {
            let total = n3 * n3 * n3;
            if numbers.len() < offset + total {
                return Err("Insufficient 3D data".into());
            }
            let data = numbers[offset..offset + total].to_vec();
            Some(Lut3d {
                title: title.clone(),
                size: n3,
                domain_min,
                domain_max,
                data,
            })
        } else {
            None
        };

        if shaper.is_none() && cube.is_none() {
            return Err("Neither 1D nor 3D LUT declared".into());
        }

        Ok(Lut {
            title,
            shaper,
            cube,
        })
    }

    /// Serialize to `.cube` format string.
    pub fn to_cube(&self) -> String {
        let mut out = String::new();
        if !self.title.is_empty() {
            let _ = writeln!(out, "TITLE \"{}\"", self.title);
        }
        if let Some(s) = &self.shaper {
            let _ = writeln!(out, "LUT_1D_SIZE {}", s.data.len());
            for c in &s.data {
                let _ = writeln!(out, "{:.6} {:.6} {:.6}", c[0], c[1], c[2]);
            }
        }
        if let Some(c) = &self.cube {
            let _ = writeln!(out, "LUT_3D_SIZE {}", c.size);
            if c.domain_min != [0.0; 3] {
                let _ = writeln!(
                    out,
                    "DOMAIN_MIN {:.6} {:.6} {:.6}",
                    c.domain_min[0], c.domain_min[1], c.domain_min[2]
                );
            }
            if c.domain_max != [1.0; 3] {
                let _ = writeln!(
                    out,
                    "DOMAIN_MAX {:.6} {:.6} {:.6}",
                    c.domain_max[0], c.domain_max[1], c.domain_max[2]
                );
            }
            for v in &c.data {
                let _ = writeln!(out, "{:.6} {:.6} {:.6}", v[0], v[1], v[2]);
            }
        }
        out
    }
}

fn parse_nums3<'a>(parts: &mut impl Iterator<Item = &'a str>) -> Result<[f32; 3], String> {
    let a: f32 = parts
        .next()
        .and_then(|t| t.parse().ok())
        .ok_or("missing 1st float")?;
    let b: f32 = parts
        .next()
        .and_then(|t| t.parse().ok())
        .ok_or("missing 2nd float")?;
    let c: f32 = parts
        .next()
        .and_then(|t| t.parse().ok())
        .ok_or("missing 3rd float")?;
    Ok([a, b, c])
}

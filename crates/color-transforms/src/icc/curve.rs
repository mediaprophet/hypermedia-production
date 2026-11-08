//! Tone reproduction curves (ICC `curv` and `para` tag types) and analytic inverses.
//!
//! Gamma and parametric curves are evaluated analytically, including outside `[0, 1]`
//! (negative inputs mirror, values above 1 extend the formula), so unbounded float transforms keep HDR
//! values. Sampled tables clamp their input to `[0, 1]`.

/// A tone reproduction curve.
#[derive(Clone, Debug, PartialEq)]
pub enum Curve {
    Identity,
    /// `y = x^g`.
    Gamma(f64),
    /// ICC `parametricCurveType` function `kind` (0–4) with parameters `[g, a, b, c, d, e, f]`.
    Parametric {
        kind: u16,
        p: [f64; 7],
    },
    /// Samples at evenly spaced inputs over `[0, 1]`, normalised outputs.
    Table(Vec<f32>),
}

/// The sRGB transfer function (IEC 61966-2.1) as ICC parametric type 3.
pub fn srgb_trc() -> Curve {
    Curve::Parametric {
        kind: 3,
        p: [2.4, 1.0 / 1.055, 0.055 / 1.055, 1.0 / 12.92, 0.04045, 0.0, 0.0],
    }
}

impl Curve {
    pub fn param_count(kind: u16) -> usize {
        match kind {
            0 => 1,
            1 => 3,
            2 => 4,
            3 => 5,
            _ => 7,
        }
    }

    pub fn is_identity(&self) -> bool {
        match self {
            Curve::Identity => true,
            Curve::Gamma(g) => (*g - 1.0).abs() < 1e-9,
            Curve::Parametric { kind: 0, p } => (p[0] - 1.0).abs() < 1e-9,
            Curve::Table(t) => {
                let n = t.len();
                n < 2 || t.iter().enumerate().all(|(i, v)| {
                    (*v as f64 - i as f64 / (n - 1) as f64).abs() < 1.0 / 65535.0
                })
            }
            _ => false,
        }
    }

    /// Evaluates the curve at `x` (f64 reference path).
    pub fn eval64(&self, x: f64) -> f64 {
        match self {
            Curve::Identity => x,
            Curve::Gamma(g) => mirror(x, |v| v.powf(*g)),
            Curve::Parametric { kind, p } => para(*kind, p, x),
            Curve::Table(t) => table(t, x as f32) as f64,
        }
    }

    /// Evaluates the inverse curve at `y`.
    pub fn eval_inverse64(&self, y: f64) -> f64 {
        match self {
            Curve::Identity => y,
            Curve::Gamma(g) => {
                if *g == 0.0 {
                    0.0
                } else {
                    mirror(y, |v| v.powf(1.0 / *g))
                }
            }
            Curve::Parametric { kind, p } => para_inverse(*kind, p, y),
            Curve::Table(t) => invert_table(t, y as f32) as f64,
        }
    }

    /// Evaluates the curve at `x` (f32).
    #[inline]
    pub fn eval32(&self, x: f32) -> f32 {
        self.eval64(x as f64) as f32
    }
}

#[inline]
fn mirror(x: f64, f: impl FnOnce(f64) -> f64) -> f64 {
    if x < 0.0 { -f(-x) } else { f(x) }
}

fn para(kind: u16, p: &[f64; 7], x: f64) -> f64 {
    let [g, a, b, c, d, e, f] = *p;
    mirror(x, |v| match kind {
        0 => v.powf(g),
        1 => if v >= -b / a { (a * v + b).powf(g) } else { 0.0 },
        2 => if v >= -b / a { (a * v + b).powf(g) + c } else { c },
        3 => if v >= d { (a * v + b).powf(g) } else { c * v },
        4 => if v >= d { (a * v + b).powf(g) + e } else { c * v + f },
        _ => v,
    })
}

fn para_inverse(kind: u16, p: &[f64; 7], y: f64) -> f64 {
    let [g, a, b, c, d, e, f] = *p;
    let inv_g = if g == 0.0 { 1.0 } else { 1.0 / g };
    mirror(y, |v| match kind {
        0 => v.powf(inv_g),
        1 => (v.powf(inv_g) - b) / a,
        2 => ((v - c).max(0.0).powf(inv_g) - b) / a,
        3 => {
            let threshold = c * d;
            if v >= threshold {
                (v.powf(inv_g) - b) / a
            } else if c != 0.0 {
                v / c
            } else {
                0.0
            }
        }
        4 => {
            let threshold = c * d + f;
            if v >= threshold {
                ((v - e).max(0.0).powf(inv_g) - b) / a
            } else if c != 0.0 {
                (v - f) / c
            } else {
                0.0
            }
        }
        _ => v,
    })
}

fn table(t: &[f32], x: f32) -> f32 {
    let n = t.len();
    if n == 0 {
        return x;
    }
    if n == 1 {
        return t[0];
    }
    let p = x.clamp(0.0, 1.0) * (n - 1) as f32;
    let i = (p as usize).min(n - 2);
    let f = p - i as f32;
    t[i] + (t[i + 1] - t[i]) * f
}

fn invert_table(t: &[f32], y: f32) -> f32 {
    let n = t.len();
    if n < 2 {
        return y;
    }
    let y = y.clamp(t[0].min(t[n - 1]), t[0].max(t[n - 1]));
    // Monotonic search
    let asc = t[n - 1] >= t[0];
    let (mut lo, mut hi) = (0, n - 1);
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if (asc && t[mid] <= y) || (!asc && t[mid] >= y) {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let dy = t[hi] - t[lo];
    let f = if dy.abs() > 1e-9 { (y - t[lo]) / dy } else { 0.0 };
    (lo as f32 + f) / (n - 1) as f32
}

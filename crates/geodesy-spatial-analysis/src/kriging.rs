//! Ordinary Kriging spatial interpolation and empirical semivariogram modeling.
//!
//! Provides Best Linear Unbiased Estimator (BLUE) spatial prediction and uncertainty
//! estimation from scattered observations using Spherical, Exponential, and Gaussian variograms.

use crate::geodesy::{Coord, Extent};
use serde::{Deserialize, Serialize};

/// Theoretical semivariogram models.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VariogramModel {
    Spherical,
    Exponential,
    Gaussian,
}

/// Parameters defining a theoretical semivariogram $\gamma(h)$.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariogramParams {
    pub model: VariogramModel,
    /// Nugget effect $c_0$: measurement error and micro-scale variation.
    pub nugget: f64,
    /// Partial sill $c$: structured variance ($c_0 + c = \text{total sill}$).
    pub sill: f64,
    /// Range $a$: spatial correlation distance limit.
    pub range: f64,
}

impl VariogramParams {
    pub fn new(model: VariogramModel, nugget: f64, sill: f64, range: f64) -> Self {
        Self {
            model,
            nugget: nugget.max(0.0),
            sill: sill.max(0.0),
            range: range.max(1e-9),
        }
    }

    /// Evaluates semivariance $\gamma(h)$ for lag distance $h$.
    pub fn semivariance(&self, h: f64) -> f64 {
        if h <= 0.0 || !h.is_finite() {
            return 0.0;
        }
        let a = self.range;
        let c0 = self.nugget;
        let c = self.sill;

        match self.model {
            VariogramModel::Spherical => {
                if h <= a {
                    c0 + c * (1.5 * (h / a) - 0.5 * (h / a).powi(3))
                } else {
                    c0 + c
                }
            }
            VariogramModel::Exponential => {
                c0 + c * (1.0 - (-3.0 * h / a).exp())
            }
            VariogramModel::Gaussian => {
                c0 + c * (1.0 - (-3.0 * (h / a).powi(2)).exp())
            }
        }
    }
}

/// A spatial observation point with value $z$.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SamplePoint {
    pub coord: Coord,
    pub value: f64,
}

impl SamplePoint {
    pub const fn new(x: f64, y: f64, value: f64) -> Self {
        Self {
            coord: Coord::new(x, y),
            value,
        }
    }
}

/// An empirical semivariogram bin.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct VariogramBin {
    pub lag_distance: f64,
    pub semivariance: f64,
    pub pair_count: usize,
}

/// Computes experimental semivariogram bins from sample points.
pub fn empirical_variogram(points: &[SamplePoint], num_lags: usize, max_lag: Option<f64>) -> Vec<VariogramBin> {
    if points.len() < 2 || num_lags == 0 {
        return Vec::new();
    }

    // Determine max pairwise distance
    let mut max_dist = 0.0f64;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = (points[i].coord.x - points[j].coord.x).hypot(points[i].coord.y - points[j].coord.y);
            if d > max_dist {
                max_dist = d;
            }
        }
    }

    let cutoff = max_lag.unwrap_or(max_dist * 0.5).max(1e-6);
    let lag_size = cutoff / (num_lags as f64);

    let mut sum_sq_diff = vec![0.0f64; num_lags];
    let mut sum_dist = vec![0.0f64; num_lags];
    let mut counts = vec![0usize; num_lags];

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let d = (points[i].coord.x - points[j].coord.x).hypot(points[i].coord.y - points[j].coord.y);
            if d <= cutoff && d > 1e-12 {
                let bin_idx = ((d / lag_size).floor() as usize).min(num_lags - 1);
                let diff = points[i].value - points[j].value;
                sum_sq_diff[bin_idx] += diff * diff;
                sum_dist[bin_idx] += d;
                counts[bin_idx] += 1;
            }
        }
    }

    let mut result = Vec::with_capacity(num_lags);
    for i in 0..num_lags {
        if counts[i] > 0 {
            result.push(VariogramBin {
                lag_distance: sum_dist[i] / (counts[i] as f64),
                semivariance: sum_sq_diff[i] / (2.0 * counts[i] as f64),
                pair_count: counts[i],
            });
        }
    }
    result
}

/// Automatically fits a variogram model to empirical bins using weighted least squares.
pub fn fit_variogram(bins: &[VariogramBin], model: VariogramModel) -> VariogramParams {
    if bins.is_empty() {
        return VariogramParams::new(model, 0.0, 1.0, 1.0);
    }

    let max_dist = bins.last().map(|b| b.lag_distance).unwrap_or(1.0);
    let sample_variance = bins.iter().map(|b| b.semivariance).fold(0.0f64, f64::max);

    // Grid search for best (nugget, sill, range)
    let mut best_loss = f64::INFINITY;
    let mut best_params = VariogramParams::new(model, 0.0, sample_variance, max_dist * 0.7);

    for nugget_factor in [0.0, 0.1, 0.2, 0.3] {
        let nugget = sample_variance * nugget_factor;
        let sill = (sample_variance - nugget).max(1e-6);

        for range_mult in [0.3, 0.5, 0.7, 1.0, 1.5] {
            let range = (max_dist * range_mult).max(1e-6);
            let params = VariogramParams::new(model, nugget, sill, range);

            let mut loss = 0.0;
            for bin in bins {
                let predicted = params.semivariance(bin.lag_distance);
                let diff = predicted - bin.semivariance;
                loss += (bin.pair_count as f64) * diff * diff;
            }

            if loss < best_loss {
                best_loss = loss;
                best_params = params;
            }
        }
    }

    best_params
}

/// Result of Kriging prediction at a single target point.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct KrigingEstimate {
    pub estimate: f64,
    pub variance: f64,
}

/// Solves Ordinary Kriging linear system for a target point.
pub fn predict_kriging(points: &[SamplePoint], target: Coord, params: &VariogramParams) -> Option<KrigingEstimate> {
    let n = points.len();
    if n == 0 {
        return None;
    }
    if n == 1 {
        let dist = (points[0].coord.x - target.x).hypot(points[0].coord.y - target.y);
        return Some(KrigingEstimate {
            estimate: points[0].value,
            variance: params.semivariance(dist),
        });
    }

    let dim = n + 1;
    let mut a = vec![0.0f64; dim * dim];
    let mut b = vec![0.0f64; dim];

    for i in 0..n {
        for j in 0..n {
            let dist = (points[i].coord.x - points[j].coord.x).hypot(points[i].coord.y - points[j].coord.y);
            a[i * dim + j] = params.semivariance(dist);
        }
        a[i * dim + n] = 1.0;
        a[n * dim + i] = 1.0;

        let d_target = (points[i].coord.x - target.x).hypot(points[i].coord.y - target.y);
        b[i] = params.semivariance(d_target);
    }
    a[n * dim + n] = 0.0;
    b[n] = 1.0;

    // Gauss-Jordan elimination with partial pivoting
    if !solve_linear_system(&mut a, &mut b, dim) {
        // Fallback: inverse-distance weighted average if singular
        let mut sum_w = 0.0;
        let mut sum_v = 0.0;
        for p in points {
            let d = (p.coord.x - target.x).hypot(p.coord.y - target.y).max(1e-9);
            let w = 1.0 / (d * d);
            sum_w += w;
            sum_v += w * p.value;
        }
        return Some(KrigingEstimate {
            estimate: sum_v / sum_w,
            variance: params.sill + params.nugget,
        });
    }

    let mut estimate = 0.0;
    let mut variance = 0.0;
    for i in 0..n {
        let weight = b[i];
        estimate += weight * points[i].value;
        let d_target = (points[i].coord.x - target.x).hypot(points[i].coord.y - target.y);
        variance += weight * params.semivariance(d_target);
    }
    // Add Lagrange multiplier
    variance += b[n];

    Some(KrigingEstimate {
        estimate,
        variance: variance.max(0.0),
    })
}

/// In-place linear solver for `Ax = b` using Gauss-Jordan elimination with partial pivoting.
fn solve_linear_system(a: &mut [f64], b: &mut [f64], n: usize) -> bool {
    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        let mut max_val = a[col * n + col].abs();
        for row in (col + 1)..n {
            let val = a[row * n + col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }

        if max_val < 1e-12 {
            return false; // Singular matrix
        }

        // Swap rows
        if max_row != col {
            for j in 0..n {
                a.swap(col * n + j, max_row * n + j);
            }
            b.swap(col, max_row);
        }

        let pivot = a[col * n + col];
        for j in col..n {
            a[col * n + j] /= pivot;
        }
        b[col] /= pivot;

        for row in 0..n {
            if row != col {
                let factor = a[row * n + col];
                if factor.abs() > 1e-12 {
                    for j in col..n {
                        a[row * n + j] -= factor * a[col * n + j];
                    }
                    b[row] -= factor * b[col];
                }
            }
        }
    }
    true
}

/// Kriging 2D grid surface interpolation output.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KrigingSurface {
    pub width: usize,
    pub height: usize,
    pub extent: Extent,
    pub predictions: Vec<f64>,
    pub variances: Vec<f64>,
}

/// Interpolates a regular raster grid from scattered points using Ordinary Kriging.
pub fn ordinary_kriging_grid(
    points: &[SamplePoint],
    extent: Extent,
    width: usize,
    height: usize,
    params: &VariogramParams,
) -> KrigingSurface {
    let mut predictions = Vec::with_capacity(width * height);
    let mut variances = Vec::with_capacity(width * height);

    let dx = extent.width() / (width as f64).max(1.0);
    let dy = extent.height() / (height as f64).max(1.0);

    for r in 0..height {
        let y = extent.max_y - (r as f64 + 0.5) * dy;
        for c in 0..width {
            let x = extent.min_x + (c as f64 + 0.5) * dx;
            if let Some(est) = predict_kriging(points, Coord::new(x, y), params) {
                predictions.push(est.estimate);
                variances.push(est.variance);
            } else {
                predictions.push(f64::NAN);
                variances.push(f64::NAN);
            }
        }
    }

    KrigingSurface {
        width,
        height,
        extent,
        predictions,
        variances,
    }
}

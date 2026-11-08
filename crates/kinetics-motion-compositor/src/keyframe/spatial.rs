//! 3D spatial motion paths and arc-length parameterization.

use crate::keyframe::bezier::bez;

/// Evaluate a 3D cubic Bézier curve at parameter `u` in [0, 1].
pub fn eval_spatial_bezier(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3], u: f64) -> [f64; 3] {
    [
        bez(p0[0], p1[0], p2[0], p3[0], u),
        bez(p0[1], p1[1], p2[1], p3[1], u),
        bez(p0[2], p1[2], p2[2], p3[2], u),
    ]
}

/// Distance between two 3D points.
pub fn distance_3d(a: [f64; 3], b: [f64; 3]) -> f64 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// An arc-length parameterization lookup table for constant-speed traversal.
#[derive(Clone, Debug, PartialEq)]
pub struct ArcLengthTable {
    pub total_length: f64,
    lengths: Vec<f64>,
    samples: usize,
}

impl ArcLengthTable {
    /// Build an arc-length table for a 3D spatial Bézier segment.
    pub fn build(p0: [f64; 3], p1: [f64; 3], p2: [f64; 3], p3: [f64; 3], samples: usize) -> Self {
        let n = samples.max(16);
        let mut lengths = Vec::with_capacity(n + 1);
        lengths.push(0.0);

        let mut prev = p0;
        let mut total = 0.0;

        for i in 1..=n {
            let u = i as f64 / n as f64;
            let pt = eval_spatial_bezier(p0, p1, p2, p3, u);
            let dist = distance_3d(prev, pt);
            total += dist;
            lengths.push(total);
            prev = pt;
        }

        Self {
            total_length: total,
            lengths,
            samples: n,
        }
    }

    /// Map arc-length progress fraction s in [0, 1] to Bézier parameter `u` in [0, 1].
    pub fn u_at_progress(&self, s: f64) -> f64 {
        if s <= 0.0 || self.total_length <= 1e-9 {
            return 0.0;
        }
        if s >= 1.0 {
            return 1.0;
        }

        let target_len = s * self.total_length;
        let mut lo = 0;
        let mut hi = self.samples;

        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if self.lengths[mid] <= target_len {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let l0 = self.lengths[lo];
        let l1 = self.lengths[hi];
        let segment_frac = if (l1 - l0).abs() > 1e-9 {
            (target_len - l0) / (l1 - l0)
        } else {
            0.0
        };

        (lo as f64 + segment_frac) / self.samples as f64
    }
}

//! Wiggly text selector with coherent noise across time and character space.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WigglySelector {
    pub min_amount: f64,
    pub max_amount: f64,
    pub wiggles_per_sec: f64,
    pub correlation: f64,
    pub temporal_phase: f64,
    pub spatial_phase: f64,
    pub random_seed: u32,
}

impl Default for WigglySelector {
    fn default() -> Self {
        Self {
            min_amount: -1.0,
            max_amount: 1.0,
            wiggles_per_sec: 2.0,
            correlation: 0.5,
            temporal_phase: 0.0,
            spatial_phase: 0.0,
            random_seed: 42,
        }
    }
}

impl WigglySelector {
    /// Evaluate wiggly weight for unit `i` at time `t_sec`.
    pub fn evaluate_unit(&self, i: usize, t_sec: f64) -> f64 {
        let time_arg = t_sec * self.wiggles_per_sec + self.temporal_phase / 360.0;
        let space_arg = i as f64 * (1.0 - self.correlation.clamp(0.0, 1.0)) + self.spatial_phase / 360.0;

        let n = smooth_noise_2d(space_arg, time_arg, self.random_seed);
        // Normalize noise [-1, 1] -> [min_amount, max_amount]
        let norm = (n + 1.0) * 0.5;
        self.min_amount + (self.max_amount - self.min_amount) * norm
    }
}

/// 2D coherent gradient noise.
fn smooth_noise_2d(x: f64, y: f64, seed: u32) -> f64 {
    let x0 = x.floor() as i64;
    let y0 = y.floor() as i64;
    let fx = x - x0 as f64;
    let fy = y - y0 as f64;

    // Hermite cubic smoothing: 3f^2 - 2f^3
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);

    let n00 = hash_coord(x0, y0, seed);
    let n10 = hash_coord(x0 + 1, y0, seed);
    let n01 = hash_coord(x0, y0 + 1, seed);
    let n11 = hash_coord(x0 + 1, y0 + 1, seed);

    let nx0 = n00 + sx * (n10 - n00);
    let nx1 = n01 + sx * (n11 - n01);

    nx0 + sy * (nx1 - nx0)
}

fn hash_coord(x: i64, y: i64, seed: u32) -> f64 {
    let mut h = (x as u64).wrapping_mul(374761393)
        ^ (y as u64).wrapping_mul(668265263)
        ^ (seed as u64).wrapping_mul(1442695040888963407);
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    let val = (h & 0xffff) as f64 / 65535.0; // 0..1
    val * 2.0 - 1.0 // -1..1
}

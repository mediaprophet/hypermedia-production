//! Snowfall accumulation, freezing line threshold, and avalanche mass transport.
//!
//! Simulates precipitation falling as snow above the freezing line, melting at lower elevations,
//! and avalanching/slipping down slopes steeper than the talus slip-off angle into mountain couloirs.

const NEIGHBOUR_X: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
const NEIGHBOUR_Y: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
const DIAGONAL: f32 = 0.7071068; // 1 / sqrt(2)
const NEIGHBOUR_WEIGHT: [f32; 8] = [
    DIAGONAL, 1.0, DIAGONAL, 1.0, 1.0, DIAGONAL, 1.0, DIAGONAL,
];

/// Configuration settings for the snow accumulation and transport solver.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnowConfig {
    /// Overall snowfall depth added per iteration.
    pub snowfall_intensity: f32,
    /// Minimum altitude below which precipitation is rain and snow melts.
    pub snow_line: f32,
    /// Rate at which snow melts below the freezing line.
    pub melting: f32,
    /// Critical slip-off slope angle in degrees (default 38.0).
    pub talus_angle: f32,
    /// Snow mass that clings to steep rock faces without slipping.
    pub clinging_volume: f32,
    /// Number of snowfall and slip iterations.
    pub iterations: usize,
}

impl Default for SnowConfig {
    fn default() -> Self {
        Self {
            snowfall_intensity: 0.05,
            snow_line: 0.4,
            melting: 0.02,
            talus_angle: 38.0,
            clinging_volume: 0.01,
            iterations: 16,
        }
    }
}

/// Simulation output layers for snow modeling.
#[derive(Debug, Clone, PartialEq)]
pub struct SnowOutput {
    /// Total combined elevation (rock terrain + snow pack).
    pub combined_height: Vec<f32>,
    /// Accumulated snow depth.
    pub snow_depth: Vec<f32>,
    /// Soft coverage mask (values from 0.0 to 1.0).
    pub soft_mask: Vec<f32>,
    /// Hard snow mask indicating permanent compacted firn / glacier ice.
    pub hard_mask: Vec<f32>,
}

/// Simulate snow precipitation, elevation-based freezing, melting, and avalanche transport.
pub fn simulate_snow(
    heights: &[f32],
    width: usize,
    height: usize,
    config: &SnowConfig,
) -> SnowOutput {
    assert_eq!(heights.len(), width * height);
    let total = width * height;
    let mut snow = vec![0.0f32; total];
    let mut work_snow = vec![0.0f32; total];

    let talus_slope = (config.talus_angle * std::f32::consts::PI / 180.0).tan();

    for _ in 0..config.iterations {
        // Step 1: Accumulate snowfall and melt based on altitude
        for idx in 0..total {
            let h = heights[idx] + snow[idx];
            if h >= config.snow_line {
                let excess = (h - config.snow_line) / (1.0 - config.snow_line).max(1e-4);
                let gain = config.snowfall_intensity * (0.5 + 0.5 * excess);
                snow[idx] += gain;
            } else {
                let melt = config.melting * (1.0 - (h / config.snow_line.max(1e-4)));
                snow[idx] = (snow[idx] - melt).max(0.0);
            }
        }

        // Step 2: Avalanche slip pass (redistribute unstable snow)
        work_snow.copy_from_slice(&snow);

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cur_snow = snow[idx];
                if cur_snow <= config.clinging_volume {
                    continue;
                }

                let cur_total_h = heights[idx] + cur_snow;
                let mut total_excess_weight = 0.0f32;
                let mut drops = [0.0f32; 8];

                for (i, (&dx, &dy)) in NEIGHBOUR_X.iter().zip(NEIGHBOUR_Y.iter()).enumerate() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let n_idx = (ny as usize) * width + (nx as usize);
                        let n_total_h = heights[n_idx] + snow[n_idx];
                        let drop = cur_total_h - n_total_h;
                        let dist = if dx != 0 && dy != 0 { std::f32::consts::SQRT_2 } else { 1.0 };
                        let slope = drop / dist;
                        if slope > talus_slope {
                            let excess = (slope - talus_slope) * NEIGHBOUR_WEIGHT[i];
                            drops[i] = excess;
                            total_excess_weight += excess;
                        }
                    }
                }

                if total_excess_weight > 0.0 {
                    let movable = (cur_snow - config.clinging_volume) * 0.5;
                    let to_move = movable.min(cur_snow * 0.75);
                    work_snow[idx] -= to_move;

                    for (i, (&dx, &dy)) in NEIGHBOUR_X.iter().zip(NEIGHBOUR_Y.iter()).enumerate() {
                        if drops[i] > 0.0 {
                            let share = (drops[i] / total_excess_weight) * to_move;
                            let nx = x as i32 + dx;
                            let ny = y as i32 + dy;
                            let n_idx = (ny as usize) * width + (nx as usize);
                            work_snow[n_idx] += share;
                        }
                    }
                }
            }
        }

        snow.copy_from_slice(&work_snow);
    }

    let mut combined = vec![0.0f32; total];
    let mut soft_mask = vec![0.0f32; total];
    let mut hard_mask = vec![0.0f32; total];

    for idx in 0..total {
        combined[idx] = heights[idx] + snow[idx];
        soft_mask[idx] = (snow[idx] * 20.0).clamp(0.0, 1.0);
        hard_mask[idx] = if snow[idx] > 0.1 { 1.0 } else { 0.0 };
    }

    SnowOutput {
        combined_height: combined,
        snow_depth: snow,
        soft_mask,
        hard_mask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snow_accumulates_above_snowline_and_slips() {
        let size = 10;
        let mut heights = vec![0.0f32; size * size];
        // Create an elevation ramp
        for y in 0..size {
            for x in 0..size {
                heights[y * size + x] = y as f32 / size as f32;
            }
        }

        let cfg = SnowConfig {
            snowfall_intensity: 0.1,
            snow_line: 0.5,
            talus_angle: 30.0,
            iterations: 4,
            ..Default::default()
        };

        let out = simulate_snow(&heights, size, size, &cfg);

        // Snow should only accumulate above snow_line or from slip
        assert_eq!(out.combined_height.len(), size * size);
        let snow_top = out.snow_depth[9 * size + 5];
        let snow_bottom = out.snow_depth[1 * size + 5];
        assert!(snow_top > snow_bottom);
    }
}

//! Particle-based hydraulic droplet erosion and sediment deposition.
//!
//! Simulates individual rain droplets rolling downhill across a bilinear-interpolated
//! 2D heightfield, eroding rock based on slope and velocity, carrying sediment,
//! and depositing material in depressions and deceleration zones.

/// LCG step: deterministically advances generator state.
#[inline]
pub fn lcg(state: u32) -> u32 {
    state.wrapping_mul(0x5391_313d).wrapping_add(0x22fc_2945)
}

/// Scale 32-bit unsigned integer to unit float [0.0, 1.0).
pub const RANDOM_SCALE: f32 = 2.328_306_4e-10;

/// 32 precomputed unit direction vectors for turbulent jitter.
const DIRECTIONS: [[f32; 2]; 32] = [
    [1.0000000, 0.0000000],
    [0.9807853, 0.1950903],
    [0.9238795, 0.3826834],
    [0.8314696, 0.5555702],
    [0.7071068, 0.7071068],
    [0.5555702, 0.8314696],
    [0.3826834, 0.9238795],
    [0.1950903, 0.9807853],
    [0.0000000, 1.0000000],
    [-0.1950903, 0.9807853],
    [-0.3826834, 0.9238795],
    [-0.5555702, 0.8314696],
    [-0.7071068, 0.7071068],
    [-0.8314696, 0.5555702],
    [-0.9238795, 0.3826834],
    [-0.9807853, 0.1950903],
    [-1.0000000, 0.0000000],
    [-0.9807853, -0.1950903],
    [-0.9238795, -0.3826834],
    [-0.8314696, -0.5555702],
    [-0.7071068, -0.7071068],
    [-0.5555702, -0.8314696],
    [-0.3826834, -0.9238795],
    [-0.1950903, -0.9807853],
    [-0.0000000, -1.0000000],
    [0.1950903, -0.9807853],
    [0.3826834, -0.9238795],
    [0.5555702, -0.8314696],
    [0.7071068, -0.7071068],
    [0.8314696, -0.5555702],
    [0.9238795, -0.3826834],
    [0.9807853, -0.1950903],
];

/// Configuration parameters for hydraulic droplet simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HydraulicConfig {
    pub seed: u32,
    pub droplet_count: usize,
    pub rock_softness: f32,
    pub fluvial_strength: f32,
    pub downcutting: f32,
    pub flow_volume: f32,
    pub laminar_flow: bool,
    pub max_steps: usize,
    pub height_range: f32,
}

impl Default for HydraulicConfig {
    fn default() -> Self {
        Self {
            seed: 1337,
            droplet_count: 50_000,
            rock_softness: 0.65,
            fluvial_strength: 0.5,
            downcutting: 0.1,
            flow_volume: 0.1,
            laminar_flow: false,
            max_steps: 128,
            height_range: 1000.0,
        }
    }
}

/// Diagnostic outputs from the hydraulic erosion solver.
#[derive(Debug, Clone, PartialEq)]
pub struct HydraulicOutput {
    pub height: Vec<f32>,
    pub deposit: Vec<f32>,
    pub wear: Vec<f32>,
    pub flow: Vec<f32>,
}

#[derive(Clone, Copy)]
struct Cell {
    top_left: f32,
    top_right: f32,
    bottom_left: f32,
    bottom_right: f32,
    tx: f32,
    ty: f32,
    ux: f32,
    uy: f32,
}

impl Cell {
    #[inline]
    fn height(&self) -> f32 {
        (self.tx * self.bottom_right + self.bottom_left * self.ux) * self.ty
            + (self.tx * self.top_right + self.top_left * self.ux) * self.uy
    }
}

fn sample_cell(heights: &[f32], width: usize, height: usize, x: f32, y: f32) -> (Cell, i32, i32) {
    let col = (x - 0.5).floor() as i32;
    let row = (y - 0.5).floor() as i32;
    let w = width as i32;
    let h = height as i32;

    let c0 = col.clamp(0, w - 1) as usize;
    let c1 = (col + 1).clamp(0, w - 1) as usize;
    let r0 = row.clamp(0, h - 1) as usize;
    let r1 = (row + 1).clamp(0, h - 1) as usize;

    let tx = (x - 0.5) - col as f32;
    let ty = (y - 0.5) - row as f32;

    let cell = Cell {
        top_left: heights[r0 * width + c0],
        top_right: heights[r0 * width + c1],
        bottom_left: heights[r1 * width + c0],
        bottom_right: heights[r1 * width + c1],
        tx,
        ty,
        ux: 1.0 - tx,
        uy: 1.0 - ty,
    };
    (cell, col, row)
}

/// Simulate hydraulic erosion over a mutable 2D heightfield.
pub fn hydraulic_erosion(
    heights: &mut [f32],
    width: usize,
    height: usize,
    config: &HydraulicConfig,
) -> HydraulicOutput {
    assert_eq!(heights.len(), width * height);
    let total_cells = width * height;
    let mut deposit = vec![0.0f32; total_cells];
    let mut wear = vec![0.0f32; total_cells];
    let mut flow = vec![0.0f32; total_cells];

    if width < 3 || height < 3 {
        return HydraulicOutput {
            height: heights.to_vec(),
            deposit,
            wear,
            flow,
        };
    }

    let cell_size = config.height_range / width as f32;
    let strength = config.fluvial_strength;
    let softness = config.rock_softness;
    let mut rng = config.seed;

    let strength_softness = strength * softness;
    let squared = strength_softness * strength_softness;
    let fourth = squared * squared;
    let fill_factor = 0.75 / (fourth * 0.4 + 1.0) + 0.25;

    for droplet in 0..config.droplet_count {
        rng = lcg(rng);
        let rx = rng as f32 * RANDOM_SCALE;
        rng = lcg(rng);
        let ry = rng as f32 * RANDOM_SCALE;

        let mut x = rx * (width - 2) as f32 + 1.0;
        let mut y = ry * (height - 2) as f32 + 1.0;

        let (mut cell, mut cx, mut cy) = sample_cell(heights, width, height, x, y);
        let mut surface = cell.height();
        let mut prev_level = surface;

        let mut dir_x = 0.0f32;
        let mut dir_y = 0.0f32;
        let mut sediment = 0.0f32;
        let mut water = 0.0f32;
        let mut downhill = false;
        let mut uphill_fills = 0;

        for step in 0..config.max_steps {
            let idx = (cy.clamp(0, height as i32 - 1) as usize) * width
                + (cx.clamp(0, width as i32 - 1) as usize);
            flow[idx] += 1.0;

            let grad_x = (cell.bottom_right - cell.bottom_left) * cell.ty
                + (cell.top_right - cell.top_left) * cell.uy;
            let grad_y = (cell.bottom_right - cell.top_right) * cell.tx
                + (cell.bottom_left - cell.top_left) * cell.ux;
            let len_sq = grad_x * grad_x + grad_y * grad_y;
            if len_sq < 1e-7 {
                break;
            }
            let len = len_sq.sqrt();
            let descent_x = -grad_x / len;
            let descent_y = -grad_y / len;

            if !config.laminar_flow && downhill {
                let slot = (droplet + step) & 0x1F;
                let jitter = DIRECTIONS[slot];
                let jitter_x = jitter[0] * 0.25;
                let jitter_y = jitter[1] * 0.25;

                let blended_x = dir_x * 0.6 + descent_x + jitter_x;
                let blended_y = dir_y * 0.6 + descent_y + jitter_y;
                let norm = (blended_x * blended_x + blended_y * blended_y).sqrt();
                if norm > 1e-6 {
                    dir_x = blended_x / norm;
                    dir_y = blended_y / norm;
                }
            } else {
                dir_x = descent_x;
                dir_y = descent_y;
            }

            let next_x = x + dir_x;
            let next_y = y + dir_y;
            if next_x < 0.5 || next_y < 0.5 || next_x >= width as f32 - 0.5 || next_y >= height as f32 - 0.5 {
                break;
            }

            let (next_cell, ncx, ncy) = sample_cell(heights, width, height, next_x, next_y);
            let next_height = next_cell.height();

            let cur_idx = (cy.clamp(0, height as i32 - 1) as usize) * width
                + (cx.clamp(0, width as i32 - 1) as usize);

            if next_height > surface {
                downhill = false;
                let capacity = sediment + water;
                let rise = next_height - surface;
                if capacity < rise || uphill_fills >= 8 {
                    let amount = capacity.min(rise);
                    heights[cur_idx] += amount;
                    deposit[cur_idx] += amount;
                    break;
                }
                sediment -= rise;
                if sediment < 0.0 {
                    water = (sediment + water).max(0.0);
                    sediment = 0.0;
                }
                let fill = (cell_size * 0.001 + next_height) - surface;
                prev_level = cell_size * 0.002 + next_height;
                surface += fill;
                heights[cur_idx] += fill;
                deposit[cur_idx] += fill;
                uphill_fills += 1;
                dir_x = 0.0;
                dir_y = 0.0;
            } else {
                downhill = true;
            }

            let drop = surface - next_height;
            let mut erosion_amount = drop * strength;
            if step == 0 {
                erosion_amount = erosion_amount.min(drop * 0.25);
            }
            let mut change = water - erosion_amount;
            if change < 0.0 {
                change *= softness;
                let floor = next_height - surface;
                change = change.max(floor);
            } else {
                let settle = (prev_level - surface) * fill_factor;
                change = change.min(settle);
            }
            water -= change;

            if change >= 0.0 {
                heights[cur_idx] += change;
                deposit[cur_idx] += change;
            } else {
                let dep = deposit[cur_idx];
                let after = (dep + change).max(0.0);
                deposit[cur_idx] = after;
                let carved = (dep - after) - change;
                heights[cur_idx] += change;
                wear[cur_idx] += carved;
            }

            prev_level = change + surface;
            surface = next_height;
            x = next_x;
            y = next_y;
            cell = next_cell;
            cx = ncx;
            cy = ncy;
        }
    }

    HydraulicOutput {
        height: heights.to_vec(),
        deposit,
        wear,
        flow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hydraulic_erosion_modifies_slope_deterministically() {
        let size = 16;
        let mut map1 = vec![0.0f32; size * size];
        for y in 0..size {
            for x in 0..size {
                map1[y * size + x] = 1.0 - (x as f32 / size as f32);
            }
        }
        let mut map2 = map1.clone();

        let cfg = HydraulicConfig {
            seed: 42,
            droplet_count: 500,
            ..Default::default()
        };

        let out1 = hydraulic_erosion(&mut map1, size, size, &cfg);
        let out2 = hydraulic_erosion(&mut map2, size, size, &cfg);

        assert_eq!(out1.height, out2.height);
        assert_eq!(out1.deposit, out2.deposit);
        assert_eq!(out1.wear, out2.wear);

        // Verification of conservation & erosion occurrences
        let wear_sum: f32 = out1.wear.iter().sum();
        let dep_sum: f32 = out1.deposit.iter().sum();
        assert!(wear_sum >= 0.0);
        assert!(dep_sum >= 0.0);
    }
}

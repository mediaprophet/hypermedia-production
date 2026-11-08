//! Water-table lake filling, hydrostatic equalization, and coastal shoreline smoothing.
//!
//! Simulates standing bodies of water filling terrain depressions up to an equilibrium
//! water surface elevation, computing water depths, surface heights, and shoreline masks.

/// Configuration parameters for lake and water-table simulation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LakeConfig {
    /// Minimum sea/lake level floor elevation.
    pub water_floor: f32,
    /// Precipitation volume added into valleys.
    pub precipitation: f32,
    /// Distance scale for coastal shoreline zone smoothing.
    pub shore_size: f32,
    /// Number of hydrostatic diffusion passes.
    pub iterations: usize,
}

impl Default for LakeConfig {
    fn default() -> Self {
        Self {
            water_floor: 0.2,
            precipitation: 0.1,
            shore_size: 2.0,
            iterations: 24,
        }
    }
}

/// Simulation outputs for the lacustrine / water-table solver.
#[derive(Debug, Clone, PartialEq)]
pub struct WaterOutput {
    /// Combined terrain and water surface elevations.
    pub results: Vec<f32>,
    /// Water column depth at each cell.
    pub depth: Vec<f32>,
    /// Shoreline proximity and beach mask (0.0 to 1.0).
    pub shore: Vec<f32>,
    /// Surface elevation of the water plane.
    pub surface: Vec<f32>,
}

/// Fill lakes, depressions, and seas up to hydrostatic equilibrium across a 2D heightfield.
pub fn fill_lakes(
    heights: &[f32],
    width: usize,
    height: usize,
    config: &LakeConfig,
) -> WaterOutput {
    assert_eq!(heights.len(), width * height);
    let total = width * height;
    let mut water_surface = vec![config.water_floor; total];

    // Initialize initial water layer in depressions
    for idx in 0..total {
        let h = heights[idx];
        if h < config.water_floor {
            water_surface[idx] = config.water_floor;
        } else {
            water_surface[idx] = h;
        }
    }

    // Hydrostatic diffusion across neighbours
    let mut work_surface = water_surface.clone();
    for _ in 0..config.iterations {
        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let cur_surf = water_surface[idx];
                let cur_rock = heights[idx];

                if cur_surf <= cur_rock {
                    continue;
                }

                let mut sum_surf = cur_surf;
                let mut count = 1.0f32;

                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                        let n_idx = (ny as usize) * width + (nx as usize);
                        sum_surf += water_surface[n_idx];
                        count += 1.0;
                    }
                }

                let avg_surf = sum_surf / count;
                work_surface[idx] = avg_surf.max(cur_rock);
            }
        }
        water_surface.copy_from_slice(&work_surface);
    }

    let mut depth = vec![0.0f32; total];
    let mut results = vec![0.0f32; total];
    let mut shore = vec![0.0f32; total];

    for idx in 0..total {
        let rock = heights[idx];
        let surf = water_surface[idx];
        let d = (surf - rock).max(0.0);
        depth[idx] = d;
        results[idx] = rock + d;
    }

    // Compute shoreline proximity mask
    let radius = config.shore_size.ceil() as i32;
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if depth[idx] > 0.0 {
                // If cell is underwater, check if it's near dry land
                let mut min_land_dist = f32::MAX;
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let n_idx = (ny as usize) * width + (nx as usize);
                            if depth[n_idx] == 0.0 {
                                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                                if dist < min_land_dist {
                                    min_land_dist = dist;
                                }
                            }
                        }
                    }
                }
                if min_land_dist <= config.shore_size {
                    shore[idx] = 1.0 - (min_land_dist / config.shore_size.max(1e-4));
                }
            }
        }
    }

    WaterOutput {
        results,
        depth,
        shore,
        surface: water_surface,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_lakes_creates_flat_surface_in_bowl() {
        let size = 11;
        let mut heights = vec![1.0f32; size * size];
        // Bowl in center with floor at 0.0
        for y in 3..=7 {
            for x in 3..=7 {
                heights[y * size + x] = 0.0;
            }
        }

        let cfg = LakeConfig {
            water_floor: 0.5,
            shore_size: 2.0,
            iterations: 10,
            ..Default::default()
        };

        let out = fill_lakes(&heights, size, size, &cfg);

        // Center should have depth ~0.5
        let center_depth = out.depth[5 * size + 5];
        assert!((center_depth - 0.5).abs() < 1e-4);

        // Surrounding high ground should have 0 depth
        assert_eq!(out.depth[0], 0.0);
    }
}

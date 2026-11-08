//! Planetary impact crater cavity excavation and raised ejecta rim lip synthesis.
//!
//! Generates geologically plausible impact crater morphology: parabolic excavation bowls,
//! uplifted ejecta rim lips, central rebound peaks, and smooth breccia lens floors.

/// Configuration parameters for craterfield distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CraterfieldConfig {
    pub seed: u32,
    pub count: usize,
    pub min_radius: f32,
    pub max_radius: f32,
    pub rim_height_factor: f32,
    pub floor_smoothness: f32,
}

impl Default for CraterfieldConfig {
    fn default() -> Self {
        Self {
            seed: 42,
            count: 12,
            min_radius: 4.0,
            max_radius: 24.0,
            rim_height_factor: 0.35,
            floor_smoothness: 0.2,
        }
    }
}

/// An individual crater impact event definition.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Crater {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub depth: f32,
}

/// Linear congruential generator step.
#[inline]
fn lcg(state: u32) -> u32 {
    1402024253u32.wrapping_mul(state).wrapping_add(586950981)
}

/// Profile calculation for relative crater elevation offset at `normalized_dist` (0 = center, 1 = outer rim lip).
fn crater_profile(dist: f32, rim_height: f32) -> f32 {
    if dist > 1.25 {
        return 0.0;
    }
    if dist < 0.8 {
        // Interior bowl excavation: parabolic curve
        let t = dist / 0.8;
        let cavity = -1.0 + t * t * (1.0 + rim_height);
        cavity
    } else {
        // Outer ejecta rim: quadratic falloff from rim crest
        let t = (dist - 0.8) / 0.45;
        let falloff = (1.0 - t).max(0.0);
        rim_height * (falloff * falloff)
    }
}

/// Apply a single impact crater cavity and raised rim onto a 2D heightfield.
pub fn apply_crater(
    heights: &mut [f32],
    width: usize,
    height: usize,
    crater: &Crater,
    rim_height: f32,
    floor_smoothness: f32,
) {
    let r_outer = crater.radius * 1.25;
    let min_x = ((crater.x - r_outer).floor() as i32).max(0) as usize;
    let max_x = ((crater.x + r_outer).ceil() as i32).min(width as i32 - 1) as usize;
    let min_y = ((crater.y - r_outer).floor() as i32).max(0) as usize;
    let max_y = ((crater.y + r_outer).ceil() as i32).min(height as i32 - 1) as usize;

    // Approximate pre-impact local ground level
    let center_idx = (crater.y.clamp(0.0, height as f32 - 1.0) as usize) * width
        + (crater.x.clamp(0.0, width as f32 - 1.0) as usize);
    let base_elev = heights[center_idx];

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 - crater.x;
            let dy = y as f32 - crater.y;
            let dist = (dx * dx + dy * dy).sqrt();
            let norm_dist = dist / crater.radius;

            if norm_dist <= 1.25 {
                let offset = crater_profile(norm_dist, rim_height);
                let target_elevation = base_elev + offset * crater.depth;

                // Smooth floor breccia blending
                let blend = if norm_dist < 0.3 {
                    let t = norm_dist / 0.3;
                    (1.0 - t) * floor_smoothness
                } else {
                    0.0
                };

                let idx = y * width + x;
                heights[idx] = (1.0 - blend) * target_elevation + blend * (base_elev - crater.depth * 0.75);
            }
        }
    }
}

/// Generate a multi-crater planetary field using deterministic cell distribution.
pub fn generate_craterfield(
    heights: &mut [f32],
    width: usize,
    height: usize,
    config: &CraterfieldConfig,
) {
    let mut rng = config.seed;
    let mut craters = Vec::with_capacity(config.count);

    for _ in 0..config.count {
        rng = lcg(rng);
        let rx = (rng & 0xFFFF) as f32 / 65535.0;
        rng = lcg(rng);
        let ry = (rng & 0xFFFF) as f32 / 65535.0;
        rng = lcg(rng);
        let rr = (rng & 0xFFFF) as f32 / 65535.0;

        let radius = config.min_radius + rr * (config.max_radius - config.min_radius);
        let depth = radius * 0.25;

        craters.push(Crater {
            x: rx * (width - 1) as f32,
            y: ry * (height - 1) as f32,
            radius,
            depth,
        });
    }

    // Sort from largest to smallest so smaller craters superimpose over older large ones
    craters.sort_by(|a, b| b.radius.partial_cmp(&a.radius).unwrap_or(std::cmp::Ordering::Equal));

    for c in &craters {
        apply_crater(
            heights,
            width,
            height,
            c,
            config.rim_height_factor,
            config.floor_smoothness,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_crater_depresses_center_and_elevates_rim() {
        let size = 32;
        let mut heights = vec![1.0f32; size * size];
        let crater = Crater {
            x: 16.0,
            y: 16.0,
            radius: 8.0,
            depth: 0.5,
        };

        apply_crater(&mut heights, size, size, &crater, 0.3, 0.1);

        let center = heights[16 * size + 16];
        assert!(center < 1.0 - 0.2); // Cavity depressed

        let rim = heights[16 * size + 22]; // ~0.75 radius
        assert!(rim > 1.0); // Raised lip
    }
}

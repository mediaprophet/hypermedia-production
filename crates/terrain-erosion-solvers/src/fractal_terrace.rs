//! Stratigraphic stepped terrace modulation across geological strata.
//!
//! Subdivides vertical elevation into stepped terrace benches and cliffs,
//! simulating differential weathering across alternating hard and soft sedimentary rock layers.

/// Configuration parameters for stratigraphic terrace modulation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TerraceConfig {
    /// Vertical interval between major terrace strata benches.
    pub spacing: f32,
    /// Number of fractal octave subdivisions.
    pub octaves: i32,
    /// Overall terrace step steepness intensity (0.0 = linear slope, 1.0 = sharp vertical steps).
    pub intensity: f32,
    /// Random seed for strata thickness variations.
    pub seed: i32,
    /// Thickness randomness across strata layers (0.0 = uniform thickness, 1.0 = highly varied).
    pub thickness_randomness: f32,
    /// Hardness variability determining ledge overhang and cliff angle.
    pub hardness_randomness: f32,
}

impl Default for TerraceConfig {
    fn default() -> Self {
        Self {
            spacing: 0.1,
            octaves: 3,
            intensity: 0.85,
            seed: 12345,
            thickness_randomness: 0.35,
            hardness_randomness: 0.4,
        }
    }
}

/// PRNG table generator for strata hash lookup.
fn build_perm_table(seed: i32) -> [u8; 512] {
    let mut table = [0u8; 512];
    let mut state = (seed as u32).wrapping_mul(0x6c078965).wrapping_add(1);
    for i in 0..256 {
        state = state.wrapping_mul(1103515245).wrapping_add(12345);
        let val = ((state >> 16) & 0xFF) as u8;
        table[i] = val;
        table[i + 256] = val;
    }
    table
}

/// Compute single-elevation terraced profile and strata color index.
pub fn evaluate_stratum(
    height: f32,
    config: &TerraceConfig,
    table: &[u8; 512],
) -> (f32, f32) {
    if config.spacing <= 1e-6 {
        return (height, 0.0);
    }
    let scaled = height / config.spacing;
    let stratum = scaled.floor() as i32;
    let fraction = scaled - stratum as f32;

    let hash = table[(stratum.rem_euclid(256)) as usize] as f32 / 255.0;
    let thickness = 0.5 + (hash - 0.5) * config.thickness_randomness;

    // S-curve stepping modulation
    let mut stepped = fraction;
    for oct in 0..config.octaves.clamp(1, 8) {
        let oct_scale = 1.0 / (oct as f32 + 1.0);
        let oct_hash = table[((stratum + oct * 13).rem_euclid(256)) as usize] as f32 / 255.0;
        let hardness = 1.0 + (oct_hash - 0.5) * config.hardness_randomness;

        // Cubic hermite step
        let t = stepped.clamp(0.0, 1.0);
        let smooth = t * t * (3.0 - 2.0 * t);
        let bent = smooth.powf(hardness);

        stepped = (1.0 - config.intensity * oct_scale) * stepped
            + (config.intensity * oct_scale) * bent;
    }

    let terraced_height = (stratum as f32 + stepped) * config.spacing;
    (terraced_height, hash)
}

/// Apply stratigraphic terracing to an entire 2D heightfield in-place.
pub fn apply_terraces(
    heights: &mut [f32],
    width: usize,
    height: usize,
    config: &TerraceConfig,
) {
    assert_eq!(heights.len(), width * height);
    let table = build_perm_table(config.seed);

    for val in heights.iter_mut() {
        let (terraced, _) = evaluate_stratum(*val, config, &table);
        *val = terraced;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terraces_quantize_continuous_slope() {
        let cfg = TerraceConfig {
            spacing: 0.2,
            octaves: 3,
            intensity: 0.9,
            ..Default::default()
        };
        let table = build_perm_table(cfg.seed);

        let h1 = 0.21;
        let h2 = 0.23;
        let (t1, _) = evaluate_stratum(h1, &cfg, &table);
        let (t2, _) = evaluate_stratum(h2, &cfg, &table);

        // Near bench step, delta should be flattened compared to input delta
        assert!((t2 - t1).abs() < (h2 - h1).abs() + 1e-4);
    }
}

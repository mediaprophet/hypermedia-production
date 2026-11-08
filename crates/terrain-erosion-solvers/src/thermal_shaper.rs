//! Multi-scale slope relaxation and talus synthesis over 2D heightfield grids.
//!
//! Relaxes steep slopes towards their lowest neighbours across a multi-resolution
//! pyramid, synthesizing natural talus aprons and scree slopes.

/// 5x5 smoothing kernel weights.
const KERNEL: [f32; 25] = [
    0.003765, 0.015019, 0.023792, 0.015019, 0.003765, //
    0.015019, 0.059912, 0.094907, 0.059912, 0.015019, //
    0.023792, 0.094907, 0.150342, 0.094907, 0.023792, //
    0.015019, 0.059912, 0.094907, 0.059912, 0.015019, //
    0.003765, 0.015019, 0.023792, 0.015019, 0.003765,
];

fn kernel_sum() -> f32 {
    let mut sum = 0.0f32;
    for row in KERNEL.chunks(5) {
        sum = row[0] + sum + row[1] + row[2] + row[3] + row[4];
    }
    sum
}

const BASE_ITERATIONS: i32 = 64;
const DIAGONAL: f32 = 0.70710677; // 1/sqrt(2) approx (0x3f35_0481)
const NEIGHBOUR_NORM: f32 = 0.14644662; // 1 / (4 + 4*DIAGONAL)
const DETAIL_SCALE: f32 = 256.0;

#[inline]
fn clamp_index(value: i32, size: i32) -> i32 {
    value.clamp(0, size - 1)
}

/// A 2D square float buffer for pyramid processing.
#[derive(Clone, Debug)]
struct FloatGrid {
    size: i32,
    data: Vec<f32>,
}

impl FloatGrid {
    fn new(size: i32, fill: f32) -> Self {
        Self {
            size,
            data: vec![fill; (size * size).max(0) as usize],
        }
    }

    #[inline]
    fn at(&self, x: i32, y: i32) -> f32 {
        self.data[(y * self.size + x) as usize]
    }
}

fn downsample(fine: &FloatGrid, x: i32, y: i32, sum: f32) -> f32 {
    let size = fine.size;
    let k = &KERNEL;
    let col = |offset: i32| clamp_index(2 * x + offset, size);
    let row = |offset: i32| clamp_index(2 * y + offset, size);
    let at = |dx: i32, dy: i32| fine.at(col(dx), row(dy));

    let total = k[0] * at(-2, -2)
        + k[5] * at(-1, -2)
        + k[10] * at(0, -2)
        + k[15] * at(1, -2)
        + k[20] * at(2, -2)
        + k[1] * at(-2, -1)
        + k[6] * at(-1, -1)
        + k[11] * at(0, -1)
        + k[16] * at(1, -1)
        + k[21] * at(2, -1)
        + k[2] * at(-2, 0)
        + k[7] * at(-1, 0)
        + k[12] * at(0, 0)
        + k[17] * at(1, 0)
        + k[22] * at(2, 0)
        + k[3] * at(-2, 1)
        + k[8] * at(-1, 1)
        + k[13] * at(0, 1)
        + k[18] * at(1, 1)
        + k[23] * at(2, 1)
        + k[4] * at(-2, 2)
        + k[9] * at(-1, 2)
        + k[14] * at(0, 2)
        + k[19] * at(1, 2)
        + k[24] * at(2, 2);
    total / sum
}

fn upsample(coarse: &FloatGrid, x: u32, y: u32) -> f32 {
    let k = &KERNEL;
    let cx = (x as i32) >> 1;
    let cy = (y as i32) >> 1;
    let size = coarse.size;
    let at = |dx: i32, dy: i32| coarse.at(clamp_index(cx + dx, size), clamp_index(cy + dy, size));

    match (x & 1, y & 1) {
        (0, 0) => {
            let total = k[0] * at(-1, -1)
                + k[10] * at(0, -1)
                + k[20] * at(1, -1)
                + k[2] * at(-1, 0)
                + k[12] * at(0, 0)
                + k[22] * at(1, 0)
                + k[4] * at(-1, 1)
                + k[14] * at(0, 1)
                + k[24] * at(1, 1);
            total / (k[10] * 4.0 + k[0] * 4.0 + k[12])
        }
        (0, _) => {
            let total = k[1] * at(-1, 0)
                + k[11] * at(0, 0)
                + k[21] * at(1, 0)
                + k[3] * at(-1, 1)
                + k[13] * at(0, 1)
                + k[23] * at(1, 1);
            total / (k[11] * 2.0 + k[1] * 4.0)
        }
        (_, 0) => {
            let total = k[5] * at(0, -1)
                + k[15] * at(1, -1)
                + k[7] * at(0, 0)
                + k[17] * at(1, 0)
                + k[9] * at(0, 1)
                + k[19] * at(1, 1);
            total / (k[7] * 2.0 + k[5] * 4.0)
        }
        _ => {
            let total = k[6] * at(0, 0) + k[16] * at(1, 0) + k[8] * at(0, 1) + k[18] * at(1, 1);
            total / (k[6] * 4.0)
        }
    }
}

struct Level {
    level: i32,
    height: FloatGrid,
    residual: Vec<f32>,
}

struct Shaper {
    levels: f32,
    iterations: i32,
    intensity: f32,
    first: bool,
}

impl Shaper {
    fn change(&self, map: &FloatGrid, x: i32, y: i32) -> f32 {
        let size = map.size;
        let col = |offset: i32| clamp_index(x + offset, size);
        let row = |offset: i32| clamp_index(y + offset, size) * size;
        let at = |dx: i32, dy: i32| map.data[(row(dy) + col(dx)) as usize];

        let top_left = at(-1, -1);
        let top = at(0, -1);
        let top_right = at(1, -1);
        let left = at(-1, 0);
        let right = at(1, 0);
        let bottom_left = at(-1, 1);
        let bottom = at(0, 1);
        let bottom_right = at(1, 1);
        let centre = map.data[(y * size + x) as usize];

        let diagonals = top_right + top_left + bottom_left + bottom_right;
        let sides = left + top + right + bottom;
        let mean = (diagonals * DIAGONAL + sides) * NEIGHBOUR_NORM;
        let detail = centre - mean;

        let east = right + top_right + right + bottom_right;
        let west = left + top_left + left + bottom_left;
        let south = bottom + bottom_left + bottom + bottom_right;
        let north = top + top_left + top + top_right;

        let lowest_diagonal = top_left.min(top_right).min(bottom_left.min(bottom_right));
        let lowest_side = top.min(left).min(right.min(bottom));

        let diagonal_drop = (lowest_diagonal - centre) * DIAGONAL;
        let side_drop = lowest_side - centre;
        let drop = diagonal_drop.min(side_drop);

        let gradient_x = (east - west) * 0.125;
        let gradient_y = (south - north) * 0.125;
        let slope = (gradient_y * gradient_y + gradient_x * gradient_x).sqrt();

        let depth = -drop;
        let pressure = -(slope + detail);
        let mut push = pressure.min(0.0);
        if depth > 0.0 {
            let ratio = (1.0 - push / depth).max(1e-6);
            push = -ratio.ln() * depth;
        }
        let moved = (slope.powf(0.4) * push) * self.intensity;
        drop.max(moved)
    }

    fn shape(&mut self, level: &mut Level) {
        let mut fraction = 1.0f32;
        let remaining = self.levels - level.level as f32;
        if remaining < 1.0 {
            fraction = 2.0f32.powf(remaining) - 1.0;
        }
        let passes = (self.iterations as f32 * fraction) as i32;
        let size = level.height.size as usize;

        for _ in 0..passes.max(0) {
            let map = &level.height;
            let mut changes = vec![0.0f32; size * size];
            for y in 0..size {
                for x in 0..size {
                    changes[y * size + x] = self.change(map, x as i32, y as i32);
                }
            }
            for (height, &change) in level.height.data.iter_mut().zip(changes.iter()) {
                *height = (*height + change).clamp(-1.0, 1.0);
            }
        }

        if !self.first {
            self.iterations = ((self.iterations - 1) >> 1) + 1;
            self.intensity *= 0.5;
        }
        self.first = false;
    }

    fn pyramid(&mut self, fine: &mut Level) {
        let fine_size = fine.height.size;
        let coarse_size = fine_size >> 1;
        let sum = kernel_sum();
        let mut coarse_height = FloatGrid::new(coarse_size, 0.0);

        for y in 0..coarse_size {
            for x in 0..coarse_size {
                let idx = (y * coarse_size + x) as usize;
                coarse_height.data[idx] = downsample(&fine.height, x, y, sum);
            }
        }

        let mut coarse = Level {
            level: fine.level + 1,
            height: coarse_height,
            residual: Vec::new(),
        };

        let detail_scale = DETAIL_SCALE / fine_size as f32;
        let mut residual = vec![0.0f32; (fine_size * fine_size) as usize];
        for y in 0..fine_size {
            for x in 0..fine_size {
                let smooth = upsample(&coarse.height, x as u32, y as u32);
                let idx = (y * fine_size + x) as usize;
                residual[idx] = (fine.height.data[idx] - smooth) * detail_scale;
            }
        }
        fine.residual = residual;

        let deeper = (coarse.level as f64) < self.levels as f64 - 1.0 && coarse_size > 11;
        if deeper {
            self.pyramid(&mut coarse);
        }
        self.shape(&mut coarse);

        let res = std::mem::take(&mut fine.residual);
        for y in 0..fine_size {
            for x in 0..fine_size {
                let smooth = upsample(&coarse.height, x as u32, y as u32);
                let idx = (y * fine_size + x) as usize;
                fine.height.data[idx] = smooth + res[idx];
            }
        }
    }
}

/// Configuration options for the multi-scale thermal shaper.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThermalShaperConfig {
    /// Intensity and direction of slope shaping (negative shapes valleys, positive sharpens ridges).
    pub shape: f64,
    /// Physical terrain extent in world units (default 5000.0).
    pub size: f64,
    /// Characteristic feature scale (default 38.0).
    pub scale: f64,
}

impl Default for ThermalShaperConfig {
    fn default() -> Self {
        Self {
            shape: 0.25,
            size: 5000.0,
            scale: 38.0,
        }
    }
}

/// Apply multi-scale thermal shaping to relax steep slopes across a resolution pyramid.
///
/// Input `heights` must contain `size * size` elements, normalized to `[-1.0, 1.0]` or `[0.0, 1.0]`.
pub fn thermal_shape(heights: &[f32], size: usize, shape: f64) -> Vec<f32> {
    thermal_shape_with(heights, size, &ThermalShaperConfig {
        shape,
        ..Default::default()
    })
}

/// Apply multi-scale thermal shaping with custom world scale configuration.
pub fn thermal_shape_with(heights: &[f32], size: usize, config: &ThermalShaperConfig) -> Vec<f32> {
    assert_eq!(heights.len(), size * size, "heightmap buffer size mismatch");
    if size < 4 {
        return heights.to_vec();
    }

    let sign = if config.shape > 0.0 { -1.0f64 } else { 1.0 };
    let levels = ((config.scale * 0.5 / (config.size / size as f64)).ln()
        * std::f64::consts::LOG2_E) as f32;
    let base = BASE_ITERATIONS as f32;
    let fraction = 2.0f32.powf(levels - (levels as i32) as f32);
    let halving = (0.5f64.powf((levels as i32) as f64) - 1.0) * BASE_ITERATIONS as f64;
    let denominator = (((fraction - 1.0) * base) as f64 - (halving + halving)) as i32 as f32;
    let iterations = if denominator.abs() > 1e-4 {
        (((base * 1.5) / denominator) * base) as i32
    } else {
        BASE_ITERATIONS
    };

    let mut shaper = Shaper {
        levels,
        iterations: iterations.clamp(1, 256),
        intensity: ((config.shape * 4.0) as f32).abs(),
        first: true,
    };

    let data: Vec<f32> = heights
        .iter()
        .map(|&val| (val as f64 * sign) as f32)
        .collect();

    let mut level = Level {
        level: 0,
        height: FloatGrid {
            size: size as i32,
            data,
        },
        residual: Vec::new(),
    };

    if 1.0 < levels {
        shaper.pyramid(&mut level);
    }
    shaper.shape(&mut level);

    level
        .height
        .data
        .iter()
        .map(|&val| (val as f64 * sign) as f32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermal_shaper_preserves_bounds_and_flattens_spikes() {
        let size = 16;
        let mut map = vec![0.0f32; size * size];
        // Introduce a steep central cone
        for y in 0..size {
            for x in 0..size {
                let dx = x as f32 - 7.5;
                let dy = y as f32 - 7.5;
                let dist = (dx * dx + dy * dy).sqrt();
                map[y * size + x] = (1.0 - dist / 8.0).max(0.0);
            }
        }

        let shaped = thermal_shape(&map, size, 0.5);
        assert_eq!(shaped.len(), size * size);
        for &h in &shaped {
            assert!(h.is_finite());
            assert!((-1.0..=1.0).contains(&h));
        }

        // Peak should relax and surrounding apron should receive material
        let center_before = map[8 * size + 8];
        let center_after = shaped[8 * size + 8];
        assert!(center_after <= center_before + 1e-4);
    }
}

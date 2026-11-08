//! D8 flow routing, priority-flood depression filling, and dendritic drainage network extraction.
//!
//! Implements priority-flood hydrological conditioning to eliminate false internal depressions,
//! resolves continuous downslope flow paths, computes upstream contributing area (flow accumulation),
//! and carves fluvial river networks.

use std::cmp::Ordering;

/// Neighbour offsets for 8 directions (clockwise from North-West).
pub const D8_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), // 0: NW
    (0, -1),  // 1: N
    (1, -1),  // 2: NE
    (1, 0),   // 3: E
    (1, 1),   // 4: SE
    (0, 1),   // 5: S
    (-1, 1),  // 6: SW
    (-1, 0),  // 7: W
];

pub const D8_DISTANCES: [f32; 8] = [
    std::f32::consts::SQRT_2,
    1.0,
    std::f32::consts::SQRT_2,
    1.0,
    std::f32::consts::SQRT_2,
    1.0,
    std::f32::consts::SQRT_2,
    1.0,
];

#[derive(Clone, Copy)]
struct FloodItem {
    height: f32,
    x: usize,
    y: usize,
}

impl PartialEq for FloodItem {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

impl Eq for FloodItem {}

impl PartialOrd for FloodItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FloodItem {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other
            .height
            .partial_cmp(&self.height)
            .unwrap_or(Ordering::Equal)
    }
}

/// Simple binary min-heap for deterministic priority-flood routing without external dependencies.
struct MinHeap {
    data: Vec<FloodItem>,
}

impl MinHeap {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, item: FloodItem) {
        self.data.push(item);
        let mut idx = self.data.len() - 1;
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[idx] > self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> Option<FloodItem> {
        if self.data.is_empty() {
            return None;
        }
        let top = self.data.swap_remove(0);
        if !self.data.is_empty() {
            let mut idx = 0;
            let len = self.data.len();
            loop {
                let left = 2 * idx + 1;
                let right = 2 * idx + 2;
                let mut best = idx;

                if left < len && self.data[left] > self.data[best] {
                    best = left;
                }
                if right < len && self.data[right] > self.data[best] {
                    best = right;
                }
                if best != idx {
                    self.data.swap(idx, best);
                    idx = best;
                } else {
                    break;
                }
            }
        }
        Some(top)
    }
}

/// Condition a heightfield by filling all internal depression sinks to their spillway elevation.
pub fn priority_flood_fill(heights: &mut [f32], width: usize, height: usize) {
    if width < 3 || height < 3 {
        return;
    }
    let total = width * height;
    let mut visited = vec![false; total];
    let mut heap = MinHeap::new();

    // Push all perimeter cells onto the priority queue
    for x in 0..width {
        let top_idx = x;
        let btm_idx = (height - 1) * width + x;
        visited[top_idx] = true;
        visited[btm_idx] = true;
        heap.push(FloodItem {
            height: heights[top_idx],
            x,
            y: 0,
        });
        heap.push(FloodItem {
            height: heights[btm_idx],
            x,
            y: height - 1,
        });
    }
    for y in 1..height - 1 {
        let l_idx = y * width;
        let r_idx = y * width + (width - 1);
        visited[l_idx] = true;
        visited[r_idx] = true;
        heap.push(FloodItem {
            height: heights[l_idx],
            x: 0,
            y,
        });
        heap.push(FloodItem {
            height: heights[r_idx],
            x: width - 1,
            y,
        });
    }

    // Process from lowest spill rim inward
    while let Some(current) = heap.pop() {
        for (dx, dy) in &D8_OFFSETS {
            let nx = current.x as i32 + dx;
            let ny = current.y as i32 + dy;
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                let n_idx = ny * width + nx;
                if !visited[n_idx] {
                    visited[n_idx] = true;
                    if heights[n_idx] < current.height {
                        heights[n_idx] = current.height;
                    }
                    heap.push(FloodItem {
                        height: heights[n_idx],
                        x: nx,
                        y: ny,
                    });
                }
            }
        }
    }
}

/// Compute D8 steepest descent flow directions for each cell.
///
/// Returns direction index `0..=7` corresponding to `D8_OFFSETS`, or `255` if flat / sink.
pub fn d8_flow_directions(heights: &[f32], width: usize, height: usize) -> Vec<u8> {
    let mut dirs = vec![255u8; width * height];
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let center_h = heights[idx];
            let mut max_slope = 0.0f32;
            let mut best_dir = 255u8;

            for (dir_idx, ((dx, dy), dist)) in D8_OFFSETS.iter().zip(D8_DISTANCES.iter()).enumerate() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let n_h = heights[(ny as usize) * width + (nx as usize)];
                    let drop = center_h - n_h;
                    if drop > 0.0 {
                        let slope = drop / dist;
                        if slope > max_slope {
                            max_slope = slope;
                            best_dir = dir_idx as u8;
                        }
                    }
                }
            }
            dirs[idx] = best_dir;
        }
    }
    dirs
}

/// Compute upstream flow accumulation area for every cell from D8 directions.
pub fn flow_accumulation(
    heights: &[f32],
    d8_dirs: &[u8],
    width: usize,
    height: usize,
) -> Vec<f32> {
    let total = width * height;
    let mut accum = vec![1.0f32; total]; // Each cell contributes unit area

    // Sort cell indices in descending elevation order (topological sorting)
    let mut order: Vec<usize> = (0..total).collect();
    order.sort_unstable_by(|&a, &b| {
        heights[b]
            .partial_cmp(&heights[a])
            .unwrap_or(Ordering::Equal)
    });

    for &idx in &order {
        let dir = d8_dirs[idx];
        if dir < 8 {
            let x = (idx % width) as i32 + D8_OFFSETS[dir as usize].0;
            let y = (idx / width) as i32 + D8_OFFSETS[dir as usize].1;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let target_idx = (y as usize) * width + (x as usize);
                accum[target_idx] += accum[idx];
            }
        }
    }
    accum
}

/// Carve fluvial river channels into a conditioned heightfield along high-accumulation paths.
pub fn carve_channels(
    heights: &mut [f32],
    flow_acc: &[f32],
    width: usize,
    height: usize,
    strength: f32,
    threshold: f32,
) {
    for idx in 0..width * height {
        let acc = flow_acc[idx];
        if acc >= threshold {
            let factor = ((acc - threshold + 1.0).ln() * strength * 0.005).min(heights[idx]);
            heights[idx] = (heights[idx] - factor).max(0.0);
        }
    }
}

/// Complete drainage network analysis output.
#[derive(Debug, Clone, PartialEq)]
pub struct DrainageNetwork {
    pub filled_height: Vec<f32>,
    pub flow_directions: Vec<u8>,
    pub accumulation: Vec<f32>,
    pub carved_height: Vec<f32>,
}

/// Execute full drainage analysis pipeline: priority-flood conditioning, D8 routing, accumulation, and channel carving.
pub fn drainage_network(
    heights: &[f32],
    width: usize,
    height: usize,
    downcutting: f32,
) -> DrainageNetwork {
    let mut filled = heights.to_vec();
    priority_flood_fill(&mut filled, width, height);

    let directions = d8_flow_directions(&filled, width, height);
    let accumulation = flow_accumulation(&filled, &directions, width, height);

    let mut carved = filled.clone();
    let threshold = (width * height) as f32 * 0.02; // Stream initiation threshold at 2% basin area
    carve_channels(&mut carved, &accumulation, width, height, downcutting, threshold);

    DrainageNetwork {
        filled_height: filled,
        flow_directions: directions,
        accumulation,
        carved_height: carved,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_flood_fills_sink_hole() {
        let size = 5;
        let mut map = vec![1.0f32; size * size];
        // Create an interior sink at (2, 2)
        map[2 * size + 2] = 0.2;

        priority_flood_fill(&mut map, size, size);
        assert!((map[2 * size + 2] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn flow_accumulation_accumulates_to_sink() {
        let size = 4;
        let mut map = vec![0.0f32; size * size];
        // Tilt surface towards bottom right corner
        for y in 0..size {
            for x in 0..size {
                map[y * size + x] = 10.0 - (x + y) as f32;
            }
        }

        let dirs = d8_flow_directions(&map, size, size);
        let acc = flow_accumulation(&map, &dirs, size, size);

        // Corner (3,3) should have largest accumulation
        let max_acc = acc.iter().copied().fold(0.0f32, f32::max);
        assert!(acc[3 * size + 3] >= max_acc - 1e-4);
        assert!(max_acc > 1.0);
    }
}

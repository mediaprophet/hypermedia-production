//! Octree palette reduction and color quantization.
//!
//! Builds an 8-ary spatial tree partitioning the RGB color cube, prunes nodes
//! by least pixel frequency until the target palette size is reached, and
//! assigns pixels to the nearest palette entry with optional Floyd-Steinberg
//! error diffusion dithering.

pub const MAX_DEPTH: usize = 8;

#[derive(Clone, Debug)]
struct OctreeNode {
    children: [Option<Box<OctreeNode>>; 8],
    pixel_count: u64,
    sum_r: u64,
    sum_g: u64,
    sum_b: u64,
    sum_a: u64,
    palette_index: Option<usize>,
}

impl OctreeNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            pixel_count: 0,
            sum_r: 0,
            sum_g: 0,
            sum_b: 0,
            sum_a: 0,
            palette_index: None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.pixel_count > 0 && self.children.iter().all(|c| c.is_none())
    }

    fn insert(&mut self, r: u8, g: u8, b: u8, a: u8, depth: usize) {
        if depth == MAX_DEPTH {
            self.pixel_count += 1;
            self.sum_r += r as u64;
            self.sum_g += g as u64;
            self.sum_b += b as u64;
            self.sum_a += a as u64;
            return;
        }

        let shift = 7 - depth;
        let r_bit = ((r >> shift) & 1) << 2;
        let g_bit = ((g >> shift) & 1) << 1;
        let b_bit = (b >> shift) & 1;
        let index = (r_bit | g_bit | b_bit) as usize;

        if self.children[index].is_none() {
            self.children[index] = Some(Box::new(OctreeNode::new()));
        }

        self.children[index]
            .as_mut()
            .unwrap()
            .insert(r, g, b, a, depth + 1);
    }
}

/// Octree color quantizer.
pub struct OctreeQuantizer {
    root: OctreeNode,
    leaf_count: usize,
}

impl OctreeQuantizer {
    pub fn new() -> Self {
        Self {
            root: OctreeNode::new(),
            leaf_count: 0,
        }
    }

    pub fn insert(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.root.insert(r, g, b, a, 0);
    }

    /// Reduce octree leaves to `max_colors`.
    pub fn build_palette(&mut self, max_colors: usize) -> Vec<[u8; 4]> {
        let max_colors = max_colors.max(1);

        // Gather reducible nodes at all depths
        loop {
            let mut leaves = 0;
            count_leaves(&self.root, &mut leaves);
            if leaves <= max_colors {
                break;
            }

            // Find parent node of deepest leaves with smallest pixel count
            let mut best_target: Option<u64> = None;
            find_min_reducible(&self.root, &mut best_target);
            if let Some(min_count) = best_target {
                reduce_node(&mut self.root, min_count);
            } else {
                break;
            }
        }

        let mut palette = Vec::new();
        collect_palette(&mut self.root, &mut palette);
        palette
    }
}

fn count_leaves(node: &OctreeNode, count: &mut usize) {
    if node.is_leaf() {
        *count += 1;
    } else {
        for child in node.children.iter().flatten() {
            count_leaves(child, count);
        }
    }
}

fn find_min_reducible(node: &OctreeNode, min_count: &mut Option<u64>) {
    let mut has_leaf_children = false;
    let mut sum_count = 0;
    for child in node.children.iter().flatten() {
        if child.is_leaf() {
            has_leaf_children = true;
            sum_count += child.pixel_count;
        } else {
            find_min_reducible(child, min_count);
        }
    }
    if has_leaf_children {
        match min_count {
            Some(curr) if sum_count < *curr => *min_count = Some(sum_count),
            None => *min_count = Some(sum_count),
            _ => {}
        }
    }
}

fn reduce_node(node: &mut OctreeNode, target_count: u64) -> bool {
    let mut has_leaf_children = false;
    let mut sum_count = 0;
    for child in node.children.iter().flatten() {
        if child.is_leaf() {
            has_leaf_children = true;
            sum_count += child.pixel_count;
        }
    }

    if has_leaf_children && sum_count == target_count {
        for child in node.children.iter_mut() {
            if let Some(c) = child.take() {
                node.pixel_count += c.pixel_count;
                node.sum_r += c.sum_r;
                node.sum_g += c.sum_g;
                node.sum_b += c.sum_b;
                node.sum_a += c.sum_a;
            }
        }
        return true;
    }

    for child in node.children.iter_mut().flatten() {
        if reduce_node(child, target_count) {
            return true;
        }
    }
    false
}

fn collect_palette(node: &mut OctreeNode, palette: &mut Vec<[u8; 4]>) {
    if node.is_leaf() {
        let count = node.pixel_count.max(1);
        let color = [
            (node.sum_r / count) as u8,
            (node.sum_g / count) as u8,
            (node.sum_b / count) as u8,
            (node.sum_a / count) as u8,
        ];
        node.palette_index = Some(palette.len());
        palette.push(color);
    } else {
        for child in node.children.iter_mut().flatten() {
            collect_palette(child, palette);
        }
    }
}

fn nearest_palette_color(c: [f32; 4], palette: &[[u8; 4]]) -> usize {
    let mut best_idx = 0;
    let mut best_dist = f32::INFINITY;
    for (i, p) in palette.iter().enumerate() {
        let dr = c[0] - p[0] as f32;
        let dg = c[1] - p[1] as f32;
        let db = c[2] - p[2] as f32;
        let da = c[3] - p[3] as f32;
        // Perceptually weighted squared Euclidean distance
        let dist = 2.0 * dr * dr + 4.0 * dg * dg + 3.0 * db * db + da * da;
        if dist < best_dist {
            best_dist = dist;
            best_idx = i;
        }
    }
    best_idx
}

/// Quantize RGBA8 image to at most `max_colors` using Octree reduction with optional Floyd-Steinberg dithering.
///
/// Returns `(quantized_rgba8, palette)`.
pub fn octree_quantize_rgba8(
    src: &[u8],
    width: usize,
    height: usize,
    max_colors: usize,
    dither: bool,
) -> (Vec<u8>, Vec<[u8; 4]>) {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return (Vec::new(), Vec::new());
    }

    let mut quantizer = OctreeQuantizer::new();
    for p in src.chunks_exact(4) {
        quantizer.insert(p[0], p[1], p[2], p[3]);
    }
    let palette = quantizer.build_palette(max_colors);
    if palette.is_empty() {
        return (src.to_vec(), Vec::new());
    }

    let mut out = vec![0u8; width * height * 4];

    if !dither {
        for (i, p) in src.chunks_exact(4).enumerate() {
            let pf = [p[0] as f32, p[1] as f32, p[2] as f32, p[3] as f32];
            let nearest = nearest_palette_color(pf, &palette);
            let color = palette[nearest];
            let out_idx = i * 4;
            out[out_idx..out_idx + 4].copy_from_slice(&color);
        }
        return (out, palette);
    }

    // Floyd-Steinberg error diffusion
    let mut err_r = vec![0.0f32; (width + 2) * (height + 2)];
    let mut err_g = vec![0.0f32; (width + 2) * (height + 2)];
    let mut err_b = vec![0.0f32; (width + 2) * (height + 2)];
    let mut err_a = vec![0.0f32; (width + 2) * (height + 2)];
    let stride = width + 2;

    for y in 0..height {
        let in_row = y * width * 4;
        let out_row = y * width * 4;
        for x in 0..width {
            let in_idx = in_row + x * 4;
            let err_idx = (y + 1) * stride + (x + 1);

            let r = (src[in_idx] as f32 + err_r[err_idx]).clamp(0.0, 255.0);
            let g = (src[in_idx + 1] as f32 + err_g[err_idx]).clamp(0.0, 255.0);
            let b = (src[in_idx + 2] as f32 + err_b[err_idx]).clamp(0.0, 255.0);
            let a = (src[in_idx + 3] as f32 + err_a[err_idx]).clamp(0.0, 255.0);

            let nearest = nearest_palette_color([r, g, b, a], &palette);
            let color = palette[nearest];
            let out_idx = out_row + x * 4;
            out[out_idx..out_idx + 4].copy_from_slice(&color);

            let diff_r = r - color[0] as f32;
            let diff_g = g - color[1] as f32;
            let diff_b = b - color[2] as f32;
            let diff_a = a - color[3] as f32;

            // Distribute error
            let distribute = |err_buf: &mut [f32], diff: f32| {
                err_buf[err_idx + 1] += diff * (7.0 / 16.0);
                err_buf[err_idx + stride - 1] += diff * (3.0 / 16.0);
                err_buf[err_idx + stride] += diff * (5.0 / 16.0);
                err_buf[err_idx + stride + 1] += diff * (1.0 / 16.0);
            };

            distribute(&mut err_r, diff_r);
            distribute(&mut err_g, diff_g);
            distribute(&mut err_b, diff_b);
            distribute(&mut err_a, diff_a);
        }
    }

    (out, palette)
}

/// Quantize RGBA32F image to at most `max_colors` using Octree reduction.
pub fn octree_quantize_rgba32f(
    src: &[f32],
    width: usize,
    height: usize,
    max_colors: usize,
    dither: bool,
) -> (Vec<f32>, Vec<[f32; 4]>) {
    if width == 0 || height == 0 || src.len() != width * height * 4 {
        return (Vec::new(), Vec::new());
    }

    let u8_data: Vec<u8> = src
        .iter()
        .map(|&v| (v * 255.0).round().clamp(0.0, 255.0) as u8)
        .collect();

    let (quantized_u8, palette_u8) =
        octree_quantize_rgba8(&u8_data, width, height, max_colors, dither);

    let quantized_f32: Vec<f32> = quantized_u8
        .iter()
        .map(|&b| b as f32 / 255.0)
        .collect();

    let palette_f32: Vec<[f32; 4]> = palette_u8
        .iter()
        .map(|p| [
            p[0] as f32 / 255.0,
            p[1] as f32 / 255.0,
            p[2] as f32 / 255.0,
            p[3] as f32 / 255.0,
        ])
        .collect();

    (quantized_f32, palette_f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_octree_quantize_palette_budget() {
        let w = 8;
        let h = 8;
        let mut src = vec![0u8; w * h * 4];
        // Populate with 8 distinct colors
        for (i, p) in src.chunks_exact_mut(4).enumerate() {
            let c = (i % 8) as u8 * 32;
            p[0] = c;
            p[1] = 255 - c;
            p[2] = (c + 64) % 256;
            p[3] = 255;
        }

        let (res, palette) = octree_quantize_rgba8(&src, w, h, 4, false);
        assert!(palette.len() <= 4, "palette size {} <= 4", palette.len());
        assert_eq!(res.len(), src.len());
    }

    #[test]
    fn test_octree_quantize_with_dither() {
        let w = 4;
        let h = 4;
        let src = vec![128u8; w * h * 4];
        let (res, palette) = octree_quantize_rgba8(&src, w, h, 2, true);
        assert!(!palette.is_empty());
        assert_eq!(res.len(), src.len());
    }
}

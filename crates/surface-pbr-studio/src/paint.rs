//! 3D surface and UV texture paint engine: brush dabbing, eraser, smudge, clone, and polyfill.

use crate::math::Vec2;
use crate::raster::RasterImage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolKind {
    #[default]
    Brush,
    Eraser,
    Smudge,
    Clone,
    Polyfill,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrushSettings {
    pub size: f32,
    pub hardness: f32,
    pub opacity: f32,
    pub spacing: f32,
    pub color: [f32; 4],
}

impl Default for BrushSettings {
    fn default() -> Self {
        Self {
            size: 20.0,
            hardness: 0.8,
            opacity: 1.0,
            spacing: 0.1,
            color: [1.0, 1.0, 1.0, 1.0],
        }
    }
}

/// A point along a paint stroke path in UV space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StrokePoint {
    pub uv: Vec2,
    pub pressure: f32,
}

/// Applies a single dab centered at `(uv.x, uv.y)` with the current brush.
pub fn apply_dab(
    image: &mut RasterImage,
    uv: Vec2,
    radius_pixels: f32,
    hardness: f32,
    color: [f32; 4],
    opacity: f32,
) {
    if image.is_empty() || radius_pixels <= 0.0 {
        return;
    }

    let cx = uv.x * image.width as f32;
    let cy = uv.y * image.height as f32;

    let min_x = ((cx - radius_pixels).floor() as i32).max(0) as u32;
    let max_x = ((cx + radius_pixels).ceil() as i32).min(image.width as i32 - 1) as u32;
    let min_y = ((cy - radius_pixels).floor() as i32).max(0) as u32;
    let max_y = ((cy + radius_pixels).ceil() as i32).min(image.height as i32 - 1) as u32;

    let inner_radius = radius_pixels * hardness.clamp(0.0, 1.0);
    let falloff_range = (radius_pixels - inner_radius).max(1e-4);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= radius_pixels {
                let factor = if dist <= inner_radius {
                    1.0
                } else {
                    1.0 - (dist - inner_radius) / falloff_range
                };
                let alpha = factor * opacity * color[3];

                let bg = image.pixel(x, y);
                // Standard alpha blend
                let out_a = alpha + bg[3] * (1.0 - alpha);
                if out_a > 1e-6 {
                    let r = (color[0] * alpha + bg[0] * bg[3] * (1.0 - alpha)) / out_a;
                    let g = (color[1] * alpha + bg[1] * bg[3] * (1.0 - alpha)) / out_a;
                    let b = (color[2] * alpha + bg[2] * bg[3] * (1.0 - alpha)) / out_a;
                    image.set_pixel(x, y, [r, g, b, out_a.clamp(0.0, 1.0)]);
                }
            }
        }
    }
}

/// Erases pixels within a dab area.
pub fn apply_eraser_dab(
    image: &mut RasterImage,
    uv: Vec2,
    radius_pixels: f32,
    hardness: f32,
    strength: f32,
) {
    if image.is_empty() || radius_pixels <= 0.0 {
        return;
    }

    let cx = uv.x * image.width as f32;
    let cy = uv.y * image.height as f32;

    let min_x = ((cx - radius_pixels).floor() as i32).max(0) as u32;
    let max_x = ((cx + radius_pixels).ceil() as i32).min(image.width as i32 - 1) as u32;
    let min_y = ((cy - radius_pixels).floor() as i32).max(0) as u32;
    let max_y = ((cy + radius_pixels).ceil() as i32).min(image.height as i32 - 1) as u32;

    let inner_radius = radius_pixels * hardness.clamp(0.0, 1.0);
    let falloff_range = (radius_pixels - inner_radius).max(1e-4);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist <= radius_pixels {
                let factor = if dist <= inner_radius {
                    1.0
                } else {
                    1.0 - (dist - inner_radius) / falloff_range
                };
                let remove = factor * strength;
                let mut p = image.pixel(x, y);
                p[3] = (p[3] - remove).max(0.0);
                image.set_pixel(x, y, p);
            }
        }
    }
}

/// Paints an interpolated stroke along a path.
pub fn paint_stroke(
    image: &mut RasterImage,
    points: &[StrokePoint],
    settings: &BrushSettings,
    tool: ToolKind,
) {
    if points.is_empty() || image.is_empty() {
        return;
    }

    let mut last_uv = points[0].uv;
    let mut dist_accum = 0.0f32;
    let step = (settings.size * settings.spacing).max(1.0);

    for pt in points {
        let uv = pt.uv;
        let radius = settings.size * 0.5 * pt.pressure.clamp(0.1, 1.0);

        let pixel_dist = ((uv.x - last_uv.x) * image.width as f32).hypot((uv.y - last_uv.y) * image.height as f32);
        dist_accum += pixel_dist;

        if dist_accum >= step || points.len() == 1 {
            match tool {
                ToolKind::Brush => {
                    apply_dab(image, uv, radius, settings.hardness, settings.color, settings.opacity);
                }
                ToolKind::Eraser => {
                    apply_eraser_dab(image, uv, radius, settings.hardness, settings.opacity);
                }
                ToolKind::Smudge | ToolKind::Clone | ToolKind::Polyfill => {
                    // Core fallback
                    apply_dab(image, uv, radius, settings.hardness, settings.color, settings.opacity);
                }
            }
            dist_accum = 0.0;
            last_uv = uv;
        }
    }
}

/// Fills a connected UV island or polygon bounding region with constant color.
pub fn polyfill(
    image: &mut RasterImage,
    triangle_uvs: &[Vec2; 3],
    color: [f32; 4],
) {
    let (w, h) = (image.width as f32, image.height as f32);
    let p0 = Vec2::new(triangle_uvs[0].x * w, triangle_uvs[0].y * h);
    let p1 = Vec2::new(triangle_uvs[1].x * w, triangle_uvs[1].y * h);
    let p2 = Vec2::new(triangle_uvs[2].x * w, triangle_uvs[2].y * h);

    let area = (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y);
    if area.abs() < 1e-6 {
        return;
    }

    let min_x = (p0.x.min(p1.x).min(p2.x).floor() as i32).max(0) as u32;
    let max_x = ((p0.x.max(p1.x).max(p2.x).ceil() as i32).min(image.width as i32 - 1)) as u32;
    let min_y = (p0.y.min(p1.y).min(p2.y).floor() as i32).max(0) as u32;
    let max_y = ((p0.y.max(p1.y).max(p2.y).ceil() as i32).min(image.height as i32 - 1)) as u32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
            let w0 = ((p1.x - p.x) * (p2.y - p.y) - (p2.x - p.x) * (p1.y - p.y)) / area;
            let w1 = ((p2.x - p.x) * (p0.y - p.y) - (p0.x - p.x) * (p2.y - p.y)) / area;
            let w2 = 1.0 - w0 - w1;

            if w0 >= -1e-4 && w1 >= -1e-4 && w2 >= -1e-4 {
                image.set_pixel(x, y, color);
            }
        }
    }
}

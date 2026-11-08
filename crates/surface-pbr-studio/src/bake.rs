//! Ray-cast mesh map bakers using BVH acceleration.
//! Produces position, normal, ambient occlusion, thickness, curvature, and material ID maps.

use crate::bvh::Bvh;
use crate::math::{Ray, Vec2, Vec3};
use crate::mesh::Mesh;
use crate::raster::RasterImage;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

/// Types of mesh maps that can be baked.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BakeMapKind {
    Position,
    WorldNormal,
    TangentNormal,
    AmbientOcclusion,
    Thickness,
    Curvature,
    MaterialId,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BakeSettings {
    pub resolution: u32,
    pub samples: u32,
    pub max_distance: f32,
    pub ao_distance: f32,
    pub seed: u32,
}

impl Default for BakeSettings {
    fn default() -> Self {
        Self {
            resolution: 256,
            samples: 16,
            max_distance: 0.05,
            ao_distance: 0.1,
            seed: 42,
        }
    }
}

/// UV-rasterized surface texel attributes.
pub struct SurfaceAttributes {
    pub width: u32,
    pub height: u32,
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub submeshes: Vec<u32>,
    pub valid: Vec<bool>,
}

impl SurfaceAttributes {
    pub fn build(mesh: &Mesh, width: u32, height: u32) -> Self {
        let count = (width as usize).saturating_mul(height as usize);
        let mut surface = Self {
            width,
            height,
            positions: vec![Vec3::ZERO; count],
            normals: vec![Vec3::Y; count],
            submeshes: vec![0; count],
            valid: vec![false; count],
        };

        if width == 0 || height == 0 || mesh.indices.len() < 3 {
            return surface;
        }

        for tri in mesh.indices.chunks_exact(3) {
            let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            let (Some(&p0), Some(&p1), Some(&p2)) = (
                mesh.positions.get(i0),
                mesh.positions.get(i1),
                mesh.positions.get(i2),
            ) else {
                continue;
            };

            let (Some(&uv0), Some(&uv1), Some(&uv2)) =
                (mesh.uvs.get(i0), mesh.uvs.get(i1), mesh.uvs.get(i2))
            else {
                continue;
            };

            let n0 = mesh.normals.get(i0).copied().unwrap_or(Vec3::Y);
            let n1 = mesh.normals.get(i1).copied().unwrap_or(Vec3::Y);
            let n2 = mesh.normals.get(i2).copied().unwrap_or(Vec3::Y);

            surface.rasterize_triangle(p0, p1, p2, uv0, uv1, uv2, n0, n1, n2, 0);
        }

        surface
    }

    #[allow(clippy::too_many_arguments)]
    fn rasterize_triangle(
        &mut self,
        p0: Vec3,
        p1: Vec3,
        p2: Vec3,
        t0: Vec2,
        t1: Vec2,
        t2: Vec2,
        n0: Vec3,
        n1: Vec3,
        n2: Vec3,
        submesh_id: u32,
    ) {
        let (w, h) = (self.width as f32, self.height as f32);
        let a = Vec2::new(t0.x * w, t0.y * h);
        let b = Vec2::new(t1.x * w, t1.y * h);
        let c = Vec2::new(t2.x * w, t2.y * h);

        let area = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
        if area.abs() < 1e-6 {
            return;
        }

        let min_x = (a.x.min(b.x).min(c.x).floor() as i32).max(0) as u32;
        let min_y = (a.y.min(b.y).min(c.y).floor() as i32).max(0) as u32;
        let max_x = ((a.x.max(b.x).max(c.x).ceil() as i32).min(self.width as i32 - 1)) as u32;
        let max_y = ((a.y.max(b.y).max(c.y).ceil() as i32).min(self.height as i32 - 1)) as u32;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec2::new(x as f32 + 0.5, y as f32 + 0.5);
                let w0 = ((b.x - p.x) * (c.y - p.y) - (c.x - p.x) * (b.y - p.y)) / area;
                let w1 = ((c.x - p.x) * (a.y - p.y) - (a.x - p.x) * (c.y - p.y)) / area;
                let w2 = 1.0 - w0 - w1;

                if w0 >= -1e-4 && w1 >= -1e-4 && w2 >= -1e-4 {
                    let idx = (y as usize) * (self.width as usize) + (x as usize);
                    if idx < self.positions.len() {
                        self.positions[idx] = p0 * w0 + p1 * w1 + p2 * w2;
                        self.normals[idx] = (n0 * w0 + n1 * w1 + n2 * w2).normalize();
                        self.submeshes[idx] = submesh_id;
                        self.valid[idx] = true;
                    }
                }
            }
        }
    }
}

/// Bake a mesh map using BVH raycasting.
pub fn bake_map(
    mesh: &Mesh,
    bvh: &Bvh,
    kind: BakeMapKind,
    settings: &BakeSettings,
) -> RasterImage {
    let res = settings.resolution;
    let surface = SurfaceAttributes::build(mesh, res, res);
    let mut image = RasterImage::new(res, res);
    let bounds = mesh.bounds();
    let extent = bounds.extent();
    let diag = extent.length().max(1e-4);

    for y in 0..res {
        for x in 0..res {
            let idx = (y as usize) * (res as usize) + (x as usize);
            if !surface.valid[idx] {
                continue;
            }

            let pos = surface.positions[idx];
            let normal = surface.normals[idx];
            let submesh = surface.submeshes[idx];

            let pixel = match kind {
                BakeMapKind::Position => {
                    let norm_pos = (pos - bounds.min) / diag;
                    [norm_pos.x.clamp(0.0, 1.0), norm_pos.y.clamp(0.0, 1.0), norm_pos.z.clamp(0.0, 1.0), 1.0]
                }
                BakeMapKind::WorldNormal => {
                    let n = normal * 0.5 + Vec3::new(0.5, 0.5, 0.5);
                    [n.x.clamp(0.0, 1.0), n.y.clamp(0.0, 1.0), n.z.clamp(0.0, 1.0), 1.0]
                }
                BakeMapKind::TangentNormal => {
                    [0.5, 0.5, 1.0, 1.0]
                }
                BakeMapKind::AmbientOcclusion => {
                    let mut unoccluded = 0.0f32;
                    let num_samples = settings.samples.max(1);
                    let ao_dist = diag * settings.ao_distance;

                    for s in 0..num_samples {
                        let dir = hemisphere_sample(normal, s, num_samples, settings.seed);
                        let ray = Ray::new(pos + normal * 1e-4, dir);
                        if let Some(hit) = bvh.intersect(mesh, &ray) {
                            if hit.distance > ao_dist {
                                unoccluded += 1.0;
                            }
                        } else {
                            unoccluded += 1.0;
                        }
                    }
                    let ao = (unoccluded / num_samples as f32).clamp(0.0, 1.0);
                    [ao, ao, ao, 1.0]
                }
                BakeMapKind::Thickness => {
                    let ray = Ray::new(pos - normal * 1e-4, -normal);
                    let thickness = if let Some(hit) = bvh.intersect(mesh, &ray) {
                        (1.0 - (hit.distance / (diag * 0.2)).clamp(0.0, 1.0)).clamp(0.0, 1.0)
                    } else {
                        1.0
                    };
                    [thickness, thickness, thickness, 1.0]
                }
                BakeMapKind::Curvature => {
                    // Approximate curvature via normal divergence with neighbors
                    let mut diff = 0.0f32;
                    let mut count = 0.0f32;
                    for dy in [-1, 0, 1] {
                        for dx in [-1, 0, 1] {
                            let nx = (x as i32 + dx).clamp(0, res as i32 - 1) as usize;
                            let ny = (y as i32 + dy).clamp(0, res as i32 - 1) as usize;
                            let n_idx = ny * (res as usize) + nx;
                            if surface.valid[n_idx] {
                                diff += (surface.normals[n_idx] - normal).length();
                                count += 1.0;
                            }
                        }
                    }
                    let curv = if count > 0.0 { (diff / count * 2.0).clamp(0.0, 1.0) } else { 0.5 };
                    [curv, curv, curv, 1.0]
                }
                BakeMapKind::MaterialId => {
                    let hue = ((submesh.wrapping_mul(137) % 360) as f32) / 360.0;
                    let (r, g, b) = hsv_to_rgb(hue, 0.8, 0.9);
                    [r, g, b, 1.0]
                }
            };

            image.set_pixel(x, y, pixel);
        }
    }

    image
}

fn hemisphere_sample(normal: Vec3, index: u32, total: u32, seed: u32) -> Vec3 {
    let u1 = ((index as f32 + 0.5) / total as f32 + (seed as f32 * 0.1)).fract();
    let u2 = ((index.wrapping_mul(2654435761) as f32) / 4294967296.0).fract();

    let phi = u1 * TAU;
    let cos_theta = (1.0 - u2).sqrt();
    let sin_theta = u2.sqrt();

    let local = Vec3::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta);

    // Tangent basis from normal
    let tangent = if normal.y.abs() < 0.9 {
        normal.cross(Vec3::Y).normalize()
    } else {
        normal.cross(Vec3::X).normalize()
    };
    let bitangent = normal.cross(tangent);

    (tangent * local.x + bitangent * local.y + normal * local.z).normalize()
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let i = (h * 6.0).floor() as i32;
    let f = h * 6.0 - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    match i.rem_euclid(6) {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

//! Metre-space brush footprints rasterized into authored PBR UV channels.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::*;
use color_transforms::{Transfer, linear_to_srgb, to_linear};
use raster_blend_operators::{BlendMode, composite};
use surface_pbr_studio::{Mesh, SurfaceAttributes, Vec2, Vec3};

struct Dab {
    point: Vec3,
    normal: Vec3,
    radius: f32,
    hardness: f32,
    opacity: f32,
    channels: Vec<(usize, [f32; 4])>,
}

impl AssetHost<'_> {
    pub(crate) fn paint_3d_texture(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["mesh", "maps", "dabs"])?;
        let mesh_handle = required(args, "mesh")?;
        let maps = record(required(args, "maps")?)?;
        let dabs = list(required(args, "dabs")?)?;
        if !(1..=7).contains(&maps.len()) || !(1..=512).contains(&dabs.len()) {
            return Err("3D paint requires 1 to 7 maps and 1 to 512 dabs".into());
        }
        let TypedArtifact::AttributedMesh(source) = self.workspace.typed(mesh_handle)? else {
            return Err("3D paint requires an attributed mesh".into());
        };
        let vertices = source.surface.mesh.positions.len();
        let triangles = source.surface.mesh.triangles.len();
        if vertices < 3 || triangles == 0 || vertices > 200_000 || triangles > 400_000 {
            return Err("3D paint mesh exceeds its admitted complexity".into());
        }
        let mut dimensions = None;
        let channels = maps.keys().map(|name| name.as_str()).collect::<Vec<_>>();
        for (name, handle) in maps {
            let color = channel_color(name)?;
            let image = self.workspace.get(handle)?;
            let shape = match image {
                Artifact::Rgba(image) => (image.width, image.height),
                Artifact::Rgba16(image) => {
                    validate_transfer(image.transfer, color)?;
                    (image.width, image.height)
                }
                _ => return Err("PBR paint maps require RGBA8 or RGBA16 handles".into()),
            };
            if dimensions.is_some_and(|prior| prior != shape) {
                return Err("PBR paint maps must share their UV raster dimensions".into());
            }
            dimensions = Some(shape);
        }
        let (width, height) = dimensions.ok_or("PBR maps are empty")?;
        if !(1..=2048).contains(&width) || !(1..=2048).contains(&height) {
            return Err("PBR paint requires a UV raster up to 2048 by 2048".into());
        }
        let pixels = width as usize * height as usize;
        let uv = source
            .surface
            .texture_coordinates_0
            .as_ref()
            .ok_or("3D paint requires the UV0 atlas")?;
        if uv.len() != vertices
            || uv.iter().flatten().any(|&v| !(0.0..=1.0).contains(&v))
            || source
                .surface
                .mesh
                .triangles
                .iter()
                .flatten()
                .any(|&v| v as usize >= vertices)
            || source
                .surface
                .mesh
                .positions
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || v.abs() > 1e6)
        {
            return Err("Invalid unit UV atlas or metre-space mesh positions".into());
        }
        let mut raster_work = 0usize;
        for &triangle in &source.surface.mesh.triangles {
            let points = triangle.map(|slot| uv[slot as usize]);
            let extent = |axis: usize| {
                points.iter().map(|p| p[axis]).fold(0.0_f32, f32::max)
                    - points.iter().map(|p| p[axis]).fold(1.0_f32, f32::min)
            };
            let x = (extent(0) * width as f32).ceil() as usize + 2;
            let y = (extent(1) * height as f32).ceil() as usize + 2;
            raster_work = raster_work.saturating_add(x * y * 24);
        }
        self.charge(
            raster_work.saturating_add(pixels.saturating_mul(dabs.len() * (24 + 24 * maps.len()))),
            pixels
                .saturating_mul(32 + maps.len() * 32)
                .saturating_add(vertices * 40 + triangles * 12),
        )?;
        let TypedArtifact::AttributedMesh(source) = self.workspace.typed(mesh_handle)? else {
            unreachable!()
        };
        let uv = source
            .surface
            .texture_coordinates_0
            .as_ref()
            .ok_or("Missing admitted UV0 atlas")?;
        let to_vec3 = |p: [f32; 3]| Vec3::new(p[0], p[1], p[2]);
        let geometry = &source.surface.mesh;
        let mut surface_mesh = Mesh {
            positions: geometry.positions.iter().copied().map(to_vec3).collect(),
            uvs: uv.iter().map(|v| Vec2::new(v[0], v[1])).collect(),
            indices: geometry.triangles.iter().flatten().copied().collect(),
            ..Mesh::default()
        };
        surface_mesh.compute_normals();
        let strokes = dabs
            .iter()
            .map(|value| {
                let dab = record(value)?;
                fields(
                    dab,
                    &[
                        "triangle",
                        "barycentric",
                        "radius_m",
                        "hardness",
                        "opacity",
                        "channels",
                    ],
                )?;
                let index = integer(required(dab, "triangle")?, "triangle")?;
                let hit = geometry
                    .triangles
                    .get(index)
                    .ok_or("Brush triangle index outside mesh")?;
                let bary: [f64; 3] = vector(required(dab, "barycentric")?)?;
                if bary.iter().any(|v| !(0.0..=1.0).contains(v))
                    || (bary.iter().sum::<f64>() - 1.0).abs() > 1e-9
                {
                    return Err("Brush hit needs normalized triangle barycentric weights".into());
                }
                let [a, b, c] = hit.map(|slot| surface_mesh.positions[slot as usize]);
                let normal = (b - a).cross(c - a);
                if normal.length() < 1e-10 {
                    return Err("Brush hit lies on a degenerate triangle".into());
                }
                let radius = num(required(dab, "radius_m")?)?;
                let hardness = number(dab, "hardness", 0.8)?;
                let opacity = number(dab, "opacity", 1.0)?;
                if !(1e-6..=100.0).contains(&radius)
                    || !(0.0..=1.0).contains(&hardness)
                    || !(0.0..=1.0).contains(&opacity)
                {
                    return Err("Brush radius, hardness or opacity is out of range".into());
                }
                let colors = record(required(dab, "channels")?)?;
                if colors.is_empty() {
                    return Err("Dab requires at least one active channel".into());
                }
                let values = colors
                    .iter()
                    .map(|(channel, value)| {
                        let index = channels
                            .iter()
                            .position(|&name| name == channel)
                            .ok_or("Dab names an absent PBR map")?;
                        let color: [f64; 4] = vector(value)?;
                        if color.iter().any(|v| !(0.0..=1.0).contains(v)) {
                            return Err("Dab RGBA must be normalized".into());
                        }
                        Ok((index, color.map(|v| v as f32)))
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(Dab {
                    point: a * bary[0] as f32 + b * bary[1] as f32 + c * bary[2] as f32,
                    normal: normal.normalize(),
                    radius: radius as f32,
                    hardness: hardness as f32,
                    opacity: opacity as f32,
                    channels: values,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let uv_surface = SurfaceAttributes::build(&surface_mesh, width, height);
        let mut images = maps
            .iter()
            .map(|(name, value)| {
                let color = channel_color(name)?;
                let (samples, transfer) = match self.workspace.get(value)? {
                    Artifact::Rgba(image) => (
                        image
                            .pixels
                            .iter()
                            .map(|p| p.map(|v| f32::from(v) / 255.0))
                            .collect::<Vec<_>>(),
                        if color {
                            Transfer::Srgb
                        } else {
                            Transfer::Linear
                        },
                    ),
                    Artifact::Rgba16(image) => (
                        image
                            .pixels
                            .iter()
                            .map(|p| p.map(|v| f32::from(v) / 65535.0))
                            .collect(),
                        validate_transfer(image.transfer, color)?,
                    ),
                    _ => unreachable!(),
                };
                Ok(samples
                    .into_iter()
                    .map(|[r, g, b, a]| {
                        [
                            to_linear(r, transfer),
                            to_linear(g, transfer),
                            to_linear(b, transfer),
                            a,
                        ]
                    })
                    .collect::<Vec<_>>())
            })
            .collect::<Result<Vec<_>, String>>()?;
        let mut affected = vec![false; pixels];
        for dab in &strokes {
            for i in 0..pixels {
                if !uv_surface.valid[i] || dab.normal.dot(uv_surface.normals[i]) <= 0.25 {
                    continue;
                }
                let normalized = (dab.point - uv_surface.positions[i]).length() / dab.radius;
                if normalized >= 1.0 {
                    continue;
                }
                let fade = if normalized <= dab.hardness {
                    1.0
                } else {
                    (1.0 - normalized) / (1.0 - dab.hardness)
                };
                for &(channel, [r, g, b, a]) in &dab.channels {
                    let opacity = dab.opacity * fade * a;
                    if opacity > 0.0 {
                        images[channel][i] =
                            composite(BlendMode::Normal, images[channel][i], [r, g, b, opacity]);
                        affected[i] = true;
                    }
                }
            }
        }
        let mut output = Args::new();
        for (channel, pixel_data) in channels.into_iter().zip(images) {
            let color = channel_color(channel)?;
            let transfer = if color {
                Transfer::Srgb
            } else {
                Transfer::Linear
            };
            let quantize = |v: f32| (v.clamp(0.0, 1.0) * 65535.0).round() as u16;
            let samples = pixel_data
                .into_iter()
                .map(|[r, g, b, a]| {
                    let [r, g, b] = if color {
                        [r, g, b].map(linear_to_srgb)
                    } else {
                        [r, g, b]
                    };
                    [quantize(r), quantize(g), quantize(b), quantize(a)]
                })
                .collect();
            output.insert(
                channel.into(),
                self.store_rgba16(width, height, samples, Some(transfer))?,
            );
        }
        Ok(Value::Record(Args::from([
            ("maps".into(), Value::Record(output)),
            (
                "painted_texels".into(),
                Value::U64(affected.into_iter().filter(|v| *v).count() as u64),
            ),
        ])))
    }
}

fn channel_color(name: &str) -> Result<bool, String> {
    match name {
        "base_color" | "emissive" => Ok(true),
        "roughness" | "metallic" | "occlusion" | "height" | "opacity" => Ok(false),
        _ => Err(format!("Unsupported paint channel {name}")),
    }
}

fn validate_transfer(transfer: Option<Transfer>, color: bool) -> Result<Transfer, String> {
    match transfer {
        Some(Transfer::Linear) | None if !color => Ok(Transfer::Linear),
        Some(Transfer::Linear) => Ok(Transfer::Linear),
        Some(Transfer::Srgb) if color => Ok(Transfer::Srgb),
        _ => {
            Err("PBR paint requires linear scalar maps and tagged sRGB or linear color maps".into())
        }
    }
}

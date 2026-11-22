//! Hemi-octahedral orthographic bake for static, opaque, constant materials.
//! RGBA8 atlas planes are stored in row-major order, albedo/depth before normal/PBR.
//! The latter uses R/G for signed octahedral normals mapped to UNORM, B for
//! metallic, and A for roughness. View coordinates match the native sampler grid.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use qualia_core_db::container_10d::{
    decode_impostor_section, encode_impostor_section, encode_oct16_normal, hemi_octahedral_decode,
    ImpostorAtlasMiniHeader, OpacityMode, ShadingModel, FLAG_DEPTH_IN_ALPHA, FLAG_HEMI_OCTAHEDRAL,
    FLAG_PBR_METALLIC_ROUGHNESS, IMPOSTOR_MINI_HEADER_SIZE, IMPOSTOR_SECTION_VERSION, MAGIC_IMPO,
};

use super::*;

pub(super) fn textured(material: &MaterialRecord) -> bool {
    [
        material.base_color_texture,
        material.normal_texture,
        material.metallic_roughness_texture,
        material.occlusion_texture,
        material.emissive_texture,
        material.stylized_ramp_texture,
    ]
    .iter()
    .any(|digest| *digest != [0; 32])
}

pub(super) fn bake(
    host: &mut AssetHost<'_>,
    mesh: &Mesh,
    materials: &[MaterialRecord],
    bindings: &[Binding],
    grid: usize,
    resolution: usize,
) -> Result<Vec<u8>, String> {
    if materials.iter().any(|material| {
        textured(material)
            || material.opacity_mode != OpacityMode::Opaque
            || material.shading_model != ShadingModel::MetallicRoughness
            || material.emissive != [0.0; 3]
            || material.wind_animated
    }) {
        return Err(
            "Impostor baking accepts opaque static untextured metallic-roughness materials".into(),
        );
    }
    let [cx, cy, cz, radius] = enclosing_sphere(&mesh.positions)?;
    let center = point([cx, cy, cz]);
    let r = f64::from(radius);
    let tile_pixels = resolution * resolution;
    let width = grid * resolution;
    let layer_bytes = 4 * width * width;
    let mut payload = vec![0; layer_bytes * 2];
    let mut z_buffer = vec![f64::NEG_INFINITY; tile_pixels];
    let faces = mesh
        .triangles
        .iter()
        .zip(bindings)
        .map(|(&triangle, &binding)| {
            let positions = triangle.map(|i| sub(point(mesh.positions[i as usize]), center));
            let normal = face_normal(positions)?;
            let material = materials[binding.material_slot as usize];
            Ok((positions, normal, material))
        })
        .collect::<Result<Vec<_>, String>>()?;
    for view_y in 0..grid {
        for view_x in 0..grid {
            host.charge(0, 0)?;
            z_buffer.fill(f64::NEG_INFINITY);
            let uv = [view_x, view_y].map(|coord| coord as f32 / (grid - 1) as f32);
            let direction = normalized(point(hemi_octahedral_decode(uv)))?;
            let vertical = if direction[2] < 0.999_999 {
                [0.0, 0.0, 1.0]
            } else {
                [0.0, 1.0, 0.0]
            };
            let right = normalized(cross(vertical, direction))?;
            let up = cross(direction, right);
            let project = |p: Point| {
                [
                    (0.5 + dot(p, right) / (2.0 * r)) * resolution as f64,
                    (0.5 - dot(p, up) / (2.0 * r)) * resolution as f64,
                    dot(p, direction),
                ]
            };
            for &(points, normal, material) in &faces {
                let front = dot(normal, direction) > 0.0;
                if !material.double_sided && !front {
                    continue;
                }
                let [a, b, c] = points.map(project);
                let orientation = edge(a, b, c[0], c[1]);
                if orientation.abs() < 1e-15 {
                    continue;
                }
                let [lo_x, lo_y] = std::array::from_fn(|axis| {
                    a[axis]
                        .min(b[axis])
                        .min(c[axis])
                        .floor()
                        .clamp(0.0, resolution as f64) as usize
                });
                let [hi_x, hi_y] = std::array::from_fn(|axis| {
                    a[axis]
                        .max(b[axis])
                        .max(c[axis])
                        .ceil()
                        .clamp(0.0, resolution as f64) as usize
                });
                let visible_normal = if front { normal } else { mul(normal, -1.0) };
                let normal_oct = encode_oct16_normal(visible_normal.map(|value| value as f32))
                    .map(|value| unorm8((f32::from(value) / 32767.0 + 1.0) * 0.5));
                let normal_pbr = [
                    normal_oct[0],
                    normal_oct[1],
                    unorm8(material.metallic),
                    unorm8(material.roughness),
                ];
                let color = material.base_color.map(unorm8);
                for y in lo_y..hi_y {
                    for x in lo_x..hi_x {
                        let p_x = x as f64 + 0.5;
                        let p_y = y as f64 + 0.5;
                        let wa = edge(b, c, p_x, p_y) / orientation;
                        let wb = edge(c, a, p_x, p_y) / orientation;
                        let wc = 1.0 - wa - wb;
                        if wa.min(wb).min(wc) < -1e-11 {
                            continue;
                        }
                        let depth = wa * a[2] + wb * b[2] + wc * c[2];
                        let tile_index = y * resolution + x;
                        if depth <= z_buffer[tile_index] {
                            continue;
                        }
                        z_buffer[tile_index] = depth;
                        let pixel =
                            4 * ((view_y * resolution + y) * width + view_x * resolution + x);
                        // Zero denotes uncovered. One is the near sphere plane and 255 the far plane.
                        let relative = (0.5 - depth / (2.0 * r)).clamp(0.0, 1.0);
                        let depth = 1 + (relative * 254.0).round() as u8;
                        payload[pixel..pixel + 4]
                            .copy_from_slice(&[color[0], color[1], color[2], depth]);
                        let normal_offset = layer_bytes + pixel;
                        payload[normal_offset..normal_offset + 4].copy_from_slice(&normal_pbr);
                    }
                }
            }
        }
    }
    if !payload[..layer_bytes]
        .chunks_exact(4)
        .any(|pixel| pixel[3] != 0)
    {
        return Err("Impostor bake produced no covered texels".into());
    }
    let header = ImpostorAtlasMiniHeader {
        magic: MAGIC_IMPO,
        version: IMPOSTOR_SECTION_VERSION,
        flags: FLAG_HEMI_OCTAHEDRAL | FLAG_DEPTH_IN_ALPHA | FLAG_PBR_METALLIC_ROUGHNESS,
        grid_dimension: grid as u16,
        view_resolution: resolution as u16,
        sphere_center: [cx, cy, cz],
        sphere_radius: radius,
        atlas_byte_size: payload.len() as u32,
        channel_format: 0,
        reserved: [0; 31],
    };
    let mut section = vec![0; IMPOSTOR_MINI_HEADER_SIZE + payload.len()];
    encode_impostor_section(&header, &payload, &mut section).map_err(|error| error.to_string())?;
    let (decoded, bytes) = decode_impostor_section(&section).map_err(|error| error.to_string())?;
    if decoded != header || bytes != payload {
        return Err("Impostor atlas failed native byte-for-byte readback".into());
    }
    Ok(section)
}

fn unorm8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn edge(a: Point, b: Point, x: f64, y: f64) -> f64 {
    (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0])
}

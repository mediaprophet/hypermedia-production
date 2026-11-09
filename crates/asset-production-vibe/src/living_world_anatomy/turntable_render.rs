//! Deterministic 3D Software Z-Buffer Turnaround Sheet & Vector Blueprint Renderer.
//!
//! Renders any 3-layer `.10d` anatomical / CAD mesh (`positions`, `normals`, `colors`, `indices`)
//! into:
//! 1. A 3-panel Studio Turnaround Sheet PNG (`900x420`: Side Profile, 3/4 Hero Perspective,
//!    and Anterior/Front View) with hemisphere ambient + Blinn-Phong key/fill/rim lighting,
//!    ground contact shadow grid, and translucent layer-peel compositing.
//! 2. A Vector Orthographic & Anatomical Blueprint SVG with projected wireframe/shaded facets,
//!    coordinate grid, and dimensional telemetry.

use crate::graphics::Rgba;
use png::{BitDepth, ColorType, Encoder};
use std::fmt::Write as _;

/// Renders a 256x256 RGBA studio hero frame suitable for downstream Radiance / CDL / MaterialX morphisms.
pub fn render_hero_rgba_frame(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
) -> Rgba {
    render_cinema_rgba_frame_at(positions, normals, colors, indices, 256, 256, 0.68, 0.20, 0.0, false)
}

/// Renders an N-frame animated 3D cinema sequence (`160x112` H.264 macroblock-aligned) combining
/// camera orbit, biomechanical saltatorial hop / locomotion articulation, and dynamic key lighting.
pub fn render_animated_cinema_frames(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
    frame_count: usize,
    animate_locomotion: bool,
) -> Vec<Rgba> {
    let n = frame_count.clamp(4, 48);
    let mut frames = Vec::with_capacity(n);
    for i in 0..n {
        let phase = (i as f32) / (n as f32);
        let yaw = 0.45 + phase * std::f32::consts::TAU;
        let pitch = 0.16 + (phase * std::f32::consts::TAU).sin() * 0.06;
        frames.push(render_cinema_rgba_frame_at(
            positions,
            normals,
            colors,
            indices,
            160,
            112,
            yaw,
            pitch,
            phase,
            animate_locomotion,
        ));
    }
    frames
}

#[allow(clippy::too_many_arguments)]
pub fn render_cinema_rgba_frame_at(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
    width: usize,
    height: usize,
    yaw: f32,
    pitch: f32,
    phase_01: f32,
    animate_locomotion: bool,
) -> Rgba {
    let mut rgba = vec![0u8; width * height * 4];
    for y in 0..height {
        let ty = y as f32 / height as f32;
        let r = (12.0 + (1.0 - ty) * 14.0) as u8;
        let g = (19.0 + (1.0 - ty) * 18.0) as u8;
        let b = (30.0 + (1.0 - ty) * 24.0) as u8;
        for x in 0..width {
            let idx = (y * width + x) * 4;
            rgba[idx] = r;
            rgba[idx + 1] = g;
            rgba[idx + 2] = b;
            rgba[idx + 3] = 255;
        }
    }

    if !positions.is_empty() && indices.len() >= 3 {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for p in positions {
            for k in 0..3 {
                min[k] = min[k].min(p[k]);
                max[k] = max[k].max(p[k]);
            }
        }
        if !min[0].is_finite() {
            min = [-0.5, 0.0, -0.5];
            max = [0.5, 1.0, 0.5];
        }
        let center = [
            (min[0] + max[0]) * 0.5,
            (min[1] + max[1]) * 0.5,
            (min[2] + max[2]) * 0.5,
        ];
        let span_x = (max[0] - min[0]).max(0.1);
        let span_y = (max[1] - min[1]).max(0.1);
        let span_z = (max[2] - min[2]).max(0.1);
        let max_span = span_x.max(span_y).max(span_z);

        let articulated: Vec<[f32; 3]> = if animate_locomotion {
            let theta = phase_01 * std::f32::consts::TAU;
            let hop_y = theta.sin().abs() * max_span * 0.08;
            positions
                .iter()
                .map(|p| {
                    let rel_z = (p[2] - center[2]) / max_span;
                    let rel_y = (p[1] - min[1]) / max_span;
                    let wave = (theta + rel_z * 3.2).sin() * max_span * 0.035 * (1.0 - rel_y * 0.5);
                    let stride_z = if rel_y < 0.35 {
                        (theta + p[0].signum() * 0.4).cos() * max_span * 0.045 * (0.35 - rel_y)
                    } else {
                        0.0
                    };
                    [p[0], p[1] + hop_y + wave, p[2] + stride_z]
                })
                .collect()
        } else {
            positions.to_vec()
        };

        render_panel(
            &articulated,
            normals,
            colors,
            indices,
            center,
            min[1],
            max_span,
            yaw,
            pitch,
            0,
            0,
            width,
            height,
            width,
            &mut rgba,
        );

        // Draw cinema progress bar along bottom
        let prog_w = ((width as f32) * phase_01).round() as usize;
        for y in (height.saturating_sub(3))..height {
            for x in 0..prog_w.min(width) {
                let idx = (y * width + x) * 4;
                rgba[idx] = 52;
                rgba[idx + 1] = 211;
                rgba[idx + 2] = 153;
            }
        }
    }

    let pixels: Vec<[u8; 4]> = rgba
        .chunks_exact(4)
        .map(|c| [c[0], c[1], c[2], c[3]])
        .collect();
    Rgba {
        width: width as u32,
        height: height as u32,
        pixels,
    }
}

/// Renders a 3-panel (`900x420`) RGBA8 turnaround sheet PNG (`Side Profile`, `3/4 Hero`, `Front`).
pub fn render_turnaround_sheet_png(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
) -> Result<Vec<u8>, String> {
    let width = 900usize;
    let height = 420usize;
    let panel_w = 300usize;
    let mut rgba = vec![0u8; width * height * 4];

    // Studio dark-slate gradient background with subtle panel dividers
    for y in 0..height {
        let ty = y as f32 / height as f32;
        let bg_r = (14.0 + (1.0 - ty) * 10.0) as u8;
        let bg_g = (20.0 + (1.0 - ty) * 14.0) as u8;
        let bg_b = (32.0 + (1.0 - ty) * 20.0) as u8;
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let is_border =
                x % panel_w == 0 || x == width - 1 || y == 0 || y == height - 1 || y == 34;
            if is_border {
                rgba[idx] = 45;
                rgba[idx + 1] = 62;
                rgba[idx + 2] = 88;
                rgba[idx + 3] = 255;
            } else if y < 34 {
                rgba[idx] = 18;
                rgba[idx + 1] = 26;
                rgba[idx + 2] = 42;
                rgba[idx + 3] = 255;
            } else {
                rgba[idx] = bg_r;
                rgba[idx + 1] = bg_g;
                rgba[idx + 2] = bg_b;
                rgba[idx + 3] = 255;
            }
        }
    }

    if positions.is_empty() || indices.len() < 3 {
        return encode_rgba_png(width as u32, height as u32, &rgba);
    }

    // Compute visible bounding box (vertices with alpha > 0.05)
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for (i, p) in positions.iter().enumerate() {
        let a = colors.get(i).map(|c| c[3]).unwrap_or(1.0);
        if a <= 0.04 {
            continue;
        }
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    if !min[0].is_finite() {
        min = [-0.5, 0.0, -0.5];
        max = [0.5, 1.0, 0.5];
    }
    let center = [
        (min[0] + max[0]) * 0.5,
        (min[1] + max[1]) * 0.5,
        (min[2] + max[2]) * 0.5,
    ];
    let span_x = (max[0] - min[0]).max(0.1);
    let span_y = (max[1] - min[1]).max(0.1);
    let span_z = (max[2] - min[2]).max(0.1);
    let max_span = span_x.max(span_y).max(span_z);

    // 3 Camera Views:
    // Panel 0: Side Profile (yaw = 90 deg, pitch = 4 deg)
    // Panel 1: 3/4 Hero View (yaw = 38 deg, pitch = 14 deg)
    // Panel 2: Anterior / Front View (yaw = 10 deg, pitch = 8 deg)
    let views: [(f32, f32, [u8; 3]); 3] = [
        (std::f32::consts::FRAC_PI_2, 0.06, [56, 189, 248]),
        (0.68, 0.22, [52, 211, 153]),
        (0.18, 0.12, [167, 139, 250]),
    ];

    for (panel_idx, &(yaw, pitch, accent)) in views.iter().enumerate() {
        let px0 = panel_idx * panel_w;
        // Draw top accent bar for panel
        for y in 4..10 {
            for x in (px0 + 14)..(px0 + 96) {
                let idx = (y * width + x) * 4;
                rgba[idx] = accent[0];
                rgba[idx + 1] = accent[1];
                rgba[idx + 2] = accent[2];
            }
        }
        render_panel(
            positions,
            normals,
            colors,
            indices,
            center,
            min[1],
            max_span,
            yaw,
            pitch,
            px0,
            35,
            panel_w,
            height - 36,
            width,
            &mut rgba,
        );
    }

    encode_rgba_png(width as u32, height as u32, &rgba)
}

#[allow(clippy::too_many_arguments)]
fn render_panel(
    positions: &[[f32; 3]],
    normals: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
    center: [f32; 3],
    ground_y: f32,
    max_span: f32,
    yaw: f32,
    pitch: f32,
    vp_x: usize,
    vp_y: usize,
    vp_w: usize,
    vp_h: usize,
    stride_w: usize,
    rgba: &mut [u8],
) {
    let mut zbuf = vec![f32::NEG_INFINITY; vp_w * vp_h];
    let scale = (vp_w.min(vp_h) as f32) * 0.78 / max_span;
    let cx = vp_w as f32 * 0.5;
    let cy = vp_h as f32 * 0.52;

    let (sy, cy_rot) = yaw.sin_cos();
    let (sp, cp_rot) = pitch.sin_cos();

    let project_pt = |p: [f32; 3]| -> [f32; 3] {
        let dx = p[0] - center[0];
        let dy = p[1] - center[1];
        let dz = p[2] - center[2];
        // Rotate around Y by yaw, then around X by pitch
        let rx = dx * cy_rot - dz * sy;
        let rz0 = dx * sy + dz * cy_rot;
        let ry = dy * cp_rot - rz0 * sp;
        let rz = dy * sp + rz0 * cp_rot;
        [cx + rx * scale, cy - ry * scale, rz]
    };

    let rotate_norm = |n: [f32; 3]| -> [f32; 3] {
        let rx = n[0] * cy_rot - n[2] * sy;
        let rz0 = n[0] * sy + n[2] * cy_rot;
        let ry = n[1] * cp_rot - rz0 * sp;
        let rz = n[1] * sp + rz0 * cp_rot;
        [rx, ry, rz]
    };

    // Draw subtle ground shadow ellipse
    let g_pt = project_pt([center[0], ground_y, center[2]]);
    let gy_px = g_pt[1].round() as i32;
    let rx_shadow = (max_span * scale * 0.36) as i32;
    let ry_shadow = (max_span * scale * 0.06).max(4.0) as i32;
    if rx_shadow > 2 && ry_shadow > 1 {
        for dy in -ry_shadow..=ry_shadow {
            let py = gy_px + dy;
            if py < 0 || py >= vp_h as i32 {
                continue;
            }
            for dx in -rx_shadow..=rx_shadow {
                let px = (cx as i32) + dx;
                if px < 0 || px >= vp_w as i32 {
                    continue;
                }
                let r2 = (dx as f32 / rx_shadow as f32).powi(2)
                    + (dy as f32 / ry_shadow as f32).powi(2);
                if r2 <= 1.0 {
                    let shade = 1.0 - (1.0 - r2) * 0.42;
                    let idx = ((vp_y + py as usize) * stride_w + (vp_x + px as usize)) * 4;
                    rgba[idx] = (rgba[idx] as f32 * shade) as u8;
                    rgba[idx + 1] = (rgba[idx + 1] as f32 * shade) as u8;
                    rgba[idx + 2] = (rgba[idx + 2] as f32 * shade) as u8;
                }
            }
        }
    }

    // Two-pass rendering: Pass 0 = opaque (alpha >= 0.85), Pass 1 = translucent (0.05..0.85)
    for pass in 0..2 {
        for tri in indices.chunks_exact(3) {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            if i0 >= positions.len() || i1 >= positions.len() || i2 >= positions.len() {
                continue;
            }
            let c0 = colors.get(i0).copied().unwrap_or([0.75, 0.75, 0.78, 1.0]);
            let c1 = colors.get(i1).copied().unwrap_or([0.75, 0.75, 0.78, 1.0]);
            let c2 = colors.get(i2).copied().unwrap_or([0.75, 0.75, 0.78, 1.0]);
            let alpha = (c0[3] + c1[3] + c2[3]) / 3.0;
            if alpha <= 0.05 {
                continue;
            }
            if (pass == 0 && alpha < 0.85) || (pass == 1 && alpha >= 0.85) {
                continue;
            }

            let v0 = project_pt(positions[i0]);
            let v1 = project_pt(positions[i1]);
            let v2 = project_pt(positions[i2]);

            let min_x = v0[0].min(v1[0]).min(v2[0]).floor().max(0.0) as i32;
            let max_x = v0[0]
                .max(v1[0])
                .max(v2[0])
                .ceil()
                .min((vp_w - 1) as f32) as i32;
            let min_y = v0[1].min(v1[1]).min(v2[1]).floor().max(0.0) as i32;
            let max_y = v0[1]
                .max(v1[1])
                .max(v2[1])
                .ceil()
                .min((vp_h - 1) as f32) as i32;
            if min_x > max_x || min_y > max_y {
                continue;
            }

            let area = (v1[0] - v0[0]) * (v2[1] - v0[1]) - (v1[1] - v0[1]) * (v2[0] - v0[0]);
            if area.abs() < 1e-3 {
                continue;
            }
            let inv_area = 1.0 / area;

            let n0 = rotate_norm(normals.get(i0).copied().unwrap_or([0.0, 1.0, 0.0]));
            let n1 = rotate_norm(normals.get(i1).copied().unwrap_or([0.0, 1.0, 0.0]));
            let n2 = rotate_norm(normals.get(i2).copied().unwrap_or([0.0, 1.0, 0.0]));

            for py in min_y..=max_y {
                for px in min_x..=max_x {
                    let pxf = px as f32 + 0.5;
                    let pyf = py as f32 + 0.5;
                    let w0 = ((v1[0] - pxf) * (v2[1] - pyf) - (v1[1] - pyf) * (v2[0] - pxf))
                        * inv_area;
                    let w1 = ((v2[0] - pxf) * (v0[1] - pyf) - (v2[1] - pyf) * (v0[0] - pxf))
                        * inv_area;
                    let w2 = 1.0 - w0 - w1;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }

                    let z = w0 * v0[2] + w1 * v1[2] + w2 * v2[2];
                    let z_idx = (py as usize) * vp_w + (px as usize);
                    if z < zbuf[z_idx] {
                        continue;
                    }
                    if pass == 0 {
                        zbuf[z_idx] = z;
                    }

                    let mut nx = w0 * n0[0] + w1 * n1[0] + w2 * n2[0];
                    let mut ny = w0 * n0[1] + w1 * n1[1] + w2 * n2[1];
                    let mut nz = w0 * n0[2] + w1 * n1[2] + w2 * n2[2];
                    let nlen = (nx * nx + ny * ny + nz * nz).sqrt().max(1e-5);
                    nx /= nlen;
                    ny /= nlen;
                    nz /= nlen;
                    if nz < 0.0 {
                        nx = -nx;
                        ny = -ny;
                        nz = -nz;
                    }

                    // Key light + warm fill + sky hemisphere + rim highlight
                    let key = (nx * 0.48 + ny * 0.62 + nz * 0.62).max(0.0);
                    let fill = (-nx * 0.45 + ny * 0.20 + nz * 0.55).max(0.0) * 0.32;
                    let hemi = 0.30 + 0.16 * (ny * 0.5 + 0.5);
                    let rim = (1.0 - nz).powi(3) * 0.22;
                    let spec = (nx * 0.25 + ny * 0.45 + nz * 0.85).max(0.0).powi(18) * 0.14;
                    let light = (hemi + key * 0.72 + fill + rim).min(1.38);

                    let cr =
                        ((w0 * c0[0] + w1 * c1[0] + w2 * c2[0]) * light + spec).clamp(0.0, 1.0);
                    let cg =
                        ((w0 * c0[1] + w1 * c1[1] + w2 * c2[1]) * light + spec).clamp(0.0, 1.0);
                    let cb =
                        ((w0 * c0[2] + w1 * c1[2] + w2 * c2[2]) * light + spec).clamp(0.0, 1.0);

                    let out_idx = ((vp_y + py as usize) * stride_w + (vp_x + px as usize)) * 4;
                    if pass == 0 {
                        rgba[out_idx] = (cr * 255.0) as u8;
                        rgba[out_idx + 1] = (cg * 255.0) as u8;
                        rgba[out_idx + 2] = (cb * 255.0) as u8;
                    } else {
                        let a = alpha.clamp(0.1, 0.85);
                        rgba[out_idx] =
                            ((cr * 255.0) * a + (rgba[out_idx] as f32) * (1.0 - a)) as u8;
                        rgba[out_idx + 1] =
                            ((cg * 255.0) * a + (rgba[out_idx + 1] as f32) * (1.0 - a)) as u8;
                        rgba[out_idx + 2] =
                            ((cb * 255.0) * a + (rgba[out_idx + 2] as f32) * (1.0 - a)) as u8;
                    }
                }
            }
        }
    }
}

fn encode_rgba_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let mut buf = Vec::with_capacity((width * height * 2) as usize);
    {
        let mut encoder = Encoder::new(&mut buf, width, height);
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder
            .write_header()
            .map_err(|e| format!("PNG header error: {e}"))?;
        writer
            .write_image_data(rgba)
            .map_err(|e| format!("PNG write error: {e}"))?;
    }
    Ok(buf)
}

/// Renders a vector orthographic & anatomical callout blueprint SVG from a 3D mesh.
pub fn render_anatomical_blueprint_svg(
    title: &str,
    positions: &[[f32; 3]],
    colors: &[[f32; 4]],
    indices: &[u32],
    submesh_count: usize,
) -> Vec<u8> {
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for p in positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    if !min[0].is_finite() {
        min = [-0.5, 0.0, -0.5];
        max = [0.5, 1.0, 0.5];
    }
    let cz = (min[2] + max[2]) * 0.5;
    let cy = (min[1] + max[1]) * 0.5;
    let span = (max[2] - min[2]).max(max[1] - min[1]).max(0.2);
    let scale = 340.0 / span;

    let mut svg = String::with_capacity(64 * 1024);
    let _ = write!(
        &mut svg,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 800 460\" width=\"800\" height=\"460\">\
        <rect width=\"800\" height=\"460\" fill=\"#0b111e\"/>\
        <g stroke=\"#1e293b\" stroke-width=\"1\">"
    );
    for gx in (40..800).step_by(40) {
        let _ = write!(&mut svg, "<line x1=\"{gx}\" y1=\"40\" x2=\"{gx}\" y2=\"420\"/>");
    }
    for gy in (40..440).step_by(40) {
        let _ = write!(&mut svg, "<line x1=\"20\" y1=\"{gy}\" x2=\"780\" y2=\"420\"/>");
    }
    svg.push_str("</g>");

    // Collect depth-sorted triangles (subsampled if large so SVG stays fast & crisp)
    let tri_total = indices.len() / 3;
    let step = (tri_total / 900).max(1);
    let mut projected = Vec::with_capacity(tri_total / step + 1);
    for ti in (0..tri_total).step_by(step) {
        let i0 = indices[ti * 3] as usize;
        let i1 = indices[ti * 3 + 1] as usize;
        let i2 = indices[ti * 3 + 2] as usize;
        if i0 >= positions.len() || i1 >= positions.len() || i2 >= positions.len() {
            continue;
        }
        let c0 = colors.get(i0).copied().unwrap_or([0.6, 0.7, 0.8, 1.0]);
        if c0[3] <= 0.04 {
            continue;
        }
        let p0 = positions[i0];
        let p1 = positions[i1];
        let p2 = positions[i2];
        let depth = (p0[0] + p1[0] + p2[0]) / 3.0;
        projected.push((depth, p0, p1, p2, c0));
    }
    projected.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    svg.push_str("<g stroke=\"#38bdf8\" stroke-opacity=\"0.28\" stroke-width=\"0.55\">");
    for &(_, p0, p1, p2, col) in &projected {
        let x0 = 400.0 + (p0[2] - cz) * scale;
        let y0 = 240.0 - (p0[1] - cy) * scale;
        let x1 = 400.0 + (p1[2] - cz) * scale;
        let y1 = 240.0 - (p1[1] - cy) * scale;
        let x2 = 400.0 + (p2[2] - cz) * scale;
        let y2 = 240.0 - (p2[1] - cy) * scale;
        let r = (col[0].clamp(0.0, 1.0) * 255.0) as u8;
        let g = (col[1].clamp(0.0, 1.0) * 255.0) as u8;
        let b = (col[2].clamp(0.0, 1.0) * 255.0) as u8;
        let a = (col[3].clamp(0.15, 0.92)) * 0.82;
        let _ = write!(
            &mut svg,
            "<polygon points=\"{x0:.1},{y0:.1} {x1:.1},{y1:.1} {x2:.1},{y2:.1}\" fill=\"rgb({r},{g},{b})\" fill-opacity=\"{a:.2}\"/>"
        );
    }
    svg.push_str("</g>");

    let _ = write!(
        &mut svg,
        "<text x=\"24\" y=\"28\" fill=\"#38bdf8\" font-family=\"monospace\" font-size=\"13\" font-weight=\"bold\">SYNTH 10D MANIFOLD BLUEPRINT // {}</text>\
        <text x=\"24\" y=\"444\" fill=\"#94a3b8\" font-family=\"monospace\" font-size=\"11\">Vertices: {} | Triangles: {} | Submeshes: {} | Span: {:.2}m x {:.2}m x {:.2}m</text>\
        </svg>",
        title,
        positions.len(),
        tri_total,
        submesh_count,
        max[0] - min[0],
        max[1] - min[1],
        max[2] - min[2],
    );
    svg.into_bytes()
}

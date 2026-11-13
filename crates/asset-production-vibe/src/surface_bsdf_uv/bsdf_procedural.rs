//! Cycles-class physical BSDF closure compilation and procedural field synthesis.

use crate::*;
use qualia_core_db::etalon_render::frame_quality::lighting::{
    evaluate_fragment_radiance, DirectionalLight, SurfaceParameters,
};
use std::collections::BTreeMap;
use std::f64::consts::PI;
use vibe::Value;

impl AssetHost<'_> {
    // ── 3. Cycles-Class Physical BSDF Closure Compilation & Preview ─────────

    pub(super) fn compile_bsdf_graph(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "closure",
                "base_color",
                "roughness",
                "metallic",
                "ior",
                "coat_weight",
                "thin_film_nm",
                "preview_size",
            ],
        )?;

        let closure = match a.get("closure") {
            Some(v) => text(v)?,
            None => "principled_v2",
        };
        if !matches!(
            closure,
            "principled_v2" | "chiang_hair" | "fournier_forand_water" | "random_walk_sss"
        ) {
            return Err(format!(
                "Unknown BSDF closure: '{closure}'; expected principled_v2, chiang_hair, fournier_forand_water, or random_walk_sss"
            ));
        }

        let base_color = match a.get("base_color") {
            Some(v) => {
                let vals = list(v)?;
                if vals.len() < 3 || vals.len() > 4 {
                    return Err("base_color requires 3 or 4 elements".into());
                }
                let r = num(&vals[0])?;
                let g = num(&vals[1])?;
                let b = num(&vals[2])?;
                if !(0.0..=1.0).contains(&r)
                    || !(0.0..=1.0).contains(&g)
                    || !(0.0..=1.0).contains(&b)
                {
                    return Err("base_color components must be in 0.0..=1.0".into());
                }
                [r, g, b]
            }
            None => [0.8, 0.45, 0.15],
        };

        let roughness = number(a, "roughness", 0.25)?;
        if !(0.0..=1.0).contains(&roughness) {
            return Err("roughness outside 0.0..=1.0".into());
        }

        let metallic = number(a, "metallic", 0.8)?;
        if !(0.0..=1.0).contains(&metallic) {
            return Err("metallic outside 0.0..=1.0".into());
        }

        let ior = number(a, "ior", 1.5)?;
        if !(1.0..=3.0).contains(&ior) {
            return Err("ior outside 1.0..=3.0".into());
        }

        let coat_weight = number(a, "coat_weight", 0.2)?;
        if !(0.0..=1.0).contains(&coat_weight) {
            return Err("coat_weight outside 0.0..=1.0".into());
        }

        let thin_film_nm = number(a, "thin_film_nm", 0.0)?;
        if !(0.0..=1200.0).contains(&thin_film_nm) {
            return Err("thin_film_nm outside 0.0..=1200.0".into());
        }

        let preview_size = count(a, "preview_size", 64, 512)?;
        if preview_size < 16 {
            return Err("preview_size minimum is 16".into());
        }

        let n_pixels = preview_size * preview_size;
        self.charge(n_pixels * 6, n_pixels * 4 + 65536)?;

        let albedo_luminance =
            0.2126 * base_color[0] + 0.7152 * base_color[1] + 0.0722 * base_color[2];

        // Sphere preview: QualiaDB/Etalon GGX; closure lobes stay CPU-side and do not duplicate WGSL.
        let ps_f = preview_size as f64;
        let mut pixels = Vec::with_capacity(n_pixels);
        let light_dir = {
            let l = [0.55f64, 0.75, 0.60];
            let len = (l[0] * l[0] + l[1] * l[1] + l[2] * l[2]).sqrt();
            [l[0] / len, l[1] / len, l[2] / len]
        };
        let view_dir = [0.0f64, 0.0, 1.0];
        let sphere_radius = 0.88f64;
        let f0_dielectric = ((ior - 1.0) / (ior + 1.0)).powi(2);
        let sun = DirectionalLight {
            direction: [
                -light_dir[0] as f32,
                -light_dir[1] as f32,
                -light_dir[2] as f32,
            ],
            illuminance_lux: std::f32::consts::PI,
            color_linear: [1.0, 1.0, 1.0],
            cast_shadows: false,
        };
        let surface = SurfaceParameters {
            base_color: [base_color[0] as f32, base_color[1] as f32, base_color[2] as f32],
            metallic: metallic as f32,
            roughness: roughness as f32,
            reflectance_f0: f0_dielectric as f32,
            emissive: [0.0; 3],
            ambient_occlusion: 1.0,
        };
        let coat_surface = SurfaceParameters {
            base_color: [0.0, 0.0, 0.0],
            metallic: 0.0,
            roughness: 0.04,
            reflectance_f0: 0.04,
            emissive: [0.0; 3],
            ambient_occlusion: 1.0,
        };

        for y in 0..preview_size {
            for x in 0..preview_size {
                let u = (x as f64 + 0.5) / ps_f * 2.0 - 1.0;
                let v = 1.0 - (y as f64 + 0.5) / ps_f * 2.0;
                let r2 = u * u + v * v;
                if r2 <= sphere_radius * sphere_radius {
                    let z = (sphere_radius * sphere_radius - r2).sqrt();
                    let n = [u / sphere_radius, v / sphere_radius, z / sphere_radius];
                    let n_f = [n[0] as f32, n[1] as f32, n[2] as f32];
                    let v_f = [view_dir[0] as f32, view_dir[1] as f32, view_dir[2] as f32];
                    let nl =
                        (n[0] * light_dir[0] + n[1] * light_dir[1] + n[2] * light_dir[2]).max(0.0);
                    let nv =
                        (n[0] * view_dir[0] + n[1] * view_dir[1] + n[2] * view_dir[2]).max(1e-4);
                    let vh = {
                        let h = [
                            light_dir[0] + view_dir[0],
                            light_dir[1] + view_dir[1],
                            light_dir[2] + view_dir[2],
                        ];
                        let h_len = (h[0] * h[0] + h[1] * h[1] + h[2] * h[2]).sqrt().max(1e-6);
                        (view_dir[0] * h[0] + view_dir[1] * h[1] + view_dir[2] * h[2]) / h_len
                    }
                    .max(0.0);

                    let mut pbr = evaluate_fragment_radiance(
                        &surface,
                        n_f,
                        v_f,
                        Some(&sun),
                        1.0,
                        [0.06, 0.06, 0.06],
                        None,
                    );
                    if thin_film_nm > 0.0 {
                        let wavelengths = [650.0, 532.0, 450.0];
                        let film_n = 1.38;
                        let sin_t2 = (1.0 - vh * vh) / (film_n * film_n);
                        let cos_t = (1.0 - sin_t2).max(0.0).sqrt();
                        for c in 0..3 {
                            let delta = 4.0 * PI * film_n * thin_film_nm * cos_t / wavelengths[c];
                            pbr[c] = (pbr[c] as f64 * (0.5 + 0.5 * delta.cos())).clamp(0.0, 8.0)
                                as f32;
                        }
                    }

                    let mut final_rgb = match closure {
                        "principled_v2" => {
                            let mut c_out = [pbr[0] as f64, pbr[1] as f64, pbr[2] as f64];
                            if coat_weight > 0.0 {
                                let coat = evaluate_fragment_radiance(
                                    &coat_surface,
                                    n_f,
                                    v_f,
                                    Some(&sun),
                                    1.0,
                                    [0.0, 0.0, 0.0],
                                    None,
                                );
                                for c in 0..3 {
                                    c_out[c] += coat[c] as f64 * coat_weight;
                                }
                            }
                            c_out
                        }
                        "chiang_hair" => {
                            let sin_theta_i = -light_dir[1];
                            let sin_theta_r = view_dir[1];
                            let cos_theta_i = (1.0 - sin_theta_i * sin_theta_i).max(0.0).sqrt();
                            let cos_theta_r = (1.0 - sin_theta_r * sin_theta_r).max(0.0).sqrt();
                            let longitudinal_m =
                                (-(sin_theta_r - sin_theta_i + 0.04).powi(2) / 0.06).exp();
                            let mut c_out = [0.0f64; 3];
                            for c in 0..3 {
                                let r_lobe = pbr[c] as f64 * longitudinal_m * 0.85;
                                let tt_lobe =
                                    base_color[c] * 0.45 * (cos_theta_i * cos_theta_r).max(0.0);
                                c_out[c] = r_lobe + tt_lobe + base_color[c] * 0.08;
                            }
                            c_out
                        }
                        "fournier_forand_water" => {
                            let cos_scatter = view_dir[0] * (-light_dir[0])
                                + view_dir[1] * (-light_dir[1])
                                + view_dir[2] * (-light_dir[2]);
                            let phase_ff = 0.04 / (1.0 - 0.92 * cos_scatter).max(0.02).powf(1.6);
                            let absorption = [0.09, 0.035, 0.008];
                            let mut c_out = [0.0f64; 3];
                            for c in 0..3 {
                                let scatter = phase_ff * base_color[c] * 0.35 * nl;
                                let deep_blue = (1.0 - absorption[c]) * 0.25 * nv;
                                c_out[c] = scatter + deep_blue + pbr[c] as f64 * 0.65;
                            }
                            c_out
                        }
                        "random_walk_sss" => {
                            let wrap_nl = ((nl + 0.35) / 1.35).max(0.0);
                            let rim_trans = (-view_dir[0] * light_dir[0]
                                - view_dir[1] * light_dir[1]
                                - view_dir[2] * light_dir[2])
                                .max(0.0)
                                .powi(3)
                                * (1.0 - nv).powi(2)
                                * 1.2;
                            let mut c_out = [0.0f64; 3];
                            for c in 0..3 {
                                let sss = base_color[c] * (wrap_nl * 0.8 + rim_trans)
                                    + base_color[c] * 0.12;
                                c_out[c] = sss + pbr[c] as f64 * 0.35;
                            }
                            c_out
                        }
                        _ => [0.0, 0.0, 0.0],
                    };

                    // Tonemap and sRGB gamma conversion
                    for c in 0..3 {
                        let lin = (final_rgb[c] / (1.0 + final_rgb[c])).clamp(0.0, 1.0);
                        let srgb = if lin <= 0.0031308 {
                            12.92 * lin
                        } else {
                            1.055 * lin.powf(1.0 / 2.4) - 0.055
                        };
                        final_rgb[c] = srgb.clamp(0.0, 1.0);
                    }

                    let d_edge = sphere_radius - r2.sqrt();
                    let alpha_val = (d_edge * ps_f * 0.9).clamp(0.0, 1.0);

                    pixels.push([
                        (final_rgb[0] * 255.0) as u8,
                        (final_rgb[1] * 255.0) as u8,
                        (final_rgb[2] * 255.0) as u8,
                        (alpha_val * 255.0) as u8,
                    ]);
                } else {
                    pixels.push([0, 0, 0, 0]);
                }
            }
        }

        let preview_rgba_handle =
            self.store_rgba(preview_size as u32, preview_size as u32, pixels)?;

        let preview_model = match closure {
            "random_walk_sss" => "analytic diffuse and rim approximation; no spatial random walk",
            "chiang_hair" => "analytic sphere preview; no groom transport",
            "fournier_forand_water" => "analytic sphere preview; no participating medium",
            _ => "principled sphere preview",
        };
        let json_desc = serde_json::json!({
            "schema": "cycles.bsdf.closure/v2",
            "closure": closure,
            "base_color": base_color,
            "roughness": roughness,
            "metallic": metallic,
            "ior": ior,
            "coat_weight": coat_weight,
            "thin_film_nm": thin_film_nm,
            "energy_conserving": null,
            "energy_conservation_status": "not measured",
            "preview_model": preview_model,
            "albedo_luminance": albedo_luminance,
            "nodes": [
                {
                    "name": "Cycles BSDF Closure Node",
                    "type": closure,
                    "inputs": {
                        "Base Color": [base_color[0], base_color[1], base_color[2], 1.0],
                        "Roughness": roughness,
                        "Metallic": metallic,
                        "IOR": ior,
                        "Coat Weight": coat_weight,
                        "Thin Film Thickness": thin_film_nm
                    }
                }
            ]
        });
        let bsdf_json_handle =
            self.encode("json", serde_json::to_vec_pretty(&json_desc).unwrap())?;

        Ok(Value::Record(BTreeMap::from([
            ("preview_rgba".into(), preview_rgba_handle),
            ("bsdf_json".into(), bsdf_json_handle),
            ("closure".into(), Value::String(closure.to_string())),
            ("energy_conserving".into(), Value::String("unverified".into())),
            ("energy_conservation_status".into(), Value::String("not measured".into())),
            ("preview_model".into(), Value::String(preview_model.into())),
            ("albedo_luminance".into(), Value::F64(albedo_luminance)),
        ])))
    }

    // ── 4. Cycles-Class Procedural Field Synthesis ──────────────────────────

    pub(super) fn sample_procedural_field(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "basis",
                "width",
                "height",
                "scale",
                "anisotropy",
                "orientation_deg",
                "temperature_k",
                "seed",
            ],
        )?;

        let basis = match a.get("basis") {
            Some(v) => text(v)?,
            None => "gabor",
        };
        if !matches!(
            basis,
            "gabor" | "voronoi" | "wave" | "blackbody" | "wavelength"
        ) {
            return Err(format!(
                "Unknown procedural basis: '{basis}'; expected gabor, voronoi, wave, blackbody, or wavelength"
            ));
        }

        let width = count(a, "width", 128, 1024)?;
        let height = count(a, "height", 128, 1024)?;

        let scale = number(a, "scale", 6.0)?;
        if !(0.1..=128.0).contains(&scale) {
            return Err("scale outside 0.1..=128.0".into());
        }

        let anisotropy = number(a, "anisotropy", 0.7)?;
        if !(0.0..=1.0).contains(&anisotropy) {
            return Err("anisotropy outside 0.0..=1.0".into());
        }

        let orientation_deg = number(a, "orientation_deg", 35.0)?;
        if !(-360.0..=360.0).contains(&orientation_deg) {
            return Err("orientation_deg outside -360.0..=360.0".into());
        }

        let temperature_k = number(a, "temperature_k", 4500.0)?;
        if !(800.0..=20000.0).contains(&temperature_k) {
            return Err("temperature_k outside 800.0..=20000.0".into());
        }

        let seed = count(a, "seed", 42, 1_000_000)?;

        let n = width * height;
        self.charge(n * 4, n * 4 + 65536)?;

        #[inline]
        fn hash2d(ix: i32, iy: i32, s: u64) -> (f64, f64) {
            let mut h = s.wrapping_add((ix as u64).wrapping_mul(0x9E3779B97F4A7C15));
            h = h.wrapping_add((iy as u64).wrapping_mul(0xBF58476D1CE4E5B9));
            h ^= h >> 30;
            h = h.wrapping_mul(0xBF58476D1CE4E5B9);
            h ^= h >> 27;
            h = h.wrapping_mul(0x94D049BB133111EB);
            h ^= h >> 31;
            let r1 = ((h & 0xFFFFFFFF) as f64) / 4294967296.0;
            let r2 = (((h >> 32) & 0xFFFFFFFF) as f64) / 4294967296.0;
            (r1, r2)
        }

        #[inline]
        fn blackbody_rgb(temp_k: f64) -> [u8; 3] {
            let t = (temp_k / 100.0).clamp(8.0, 200.0);
            let r = if t <= 66.0 {
                255.0
            } else {
                329.698727446 * (t - 60.0).powf(-0.1332047592)
            };
            let g = if t <= 66.0 {
                99.4708025861 * t.ln() - 161.1195681661
            } else {
                288.1221695283 * (t - 60.0).powf(-0.0755148492)
            };
            let b = if t >= 66.0 {
                255.0
            } else if t <= 19.0 {
                0.0
            } else {
                138.5177312231 * (t - 10.0).ln() - 305.0447927307
            };
            [
                r.clamp(0.0, 255.0) as u8,
                g.clamp(0.0, 255.0) as u8,
                b.clamp(0.0, 255.0) as u8,
            ]
        }

        #[inline]
        fn wavelength_to_srgb(lambda: f64) -> [u8; 3] {
            #[inline]
            fn g(l: f64, mu: f64, s1: f64, s2: f64) -> f64 {
                let s = if l < mu { s1 } else { s2 };
                (-0.5 * ((l - mu) / s).powi(2)).exp()
            }
            let x_bar = 1.056 * g(lambda, 599.8, 37.9, 31.0)
                + 0.362 * g(lambda, 442.0, 16.0, 26.7)
                - 0.065 * g(lambda, 501.1, 20.4, 18.9);
            let y_bar =
                0.821 * g(lambda, 568.8, 46.9, 40.5) + 0.286 * g(lambda, 530.9, 16.3, 31.1);
            let z_bar =
                1.217 * g(lambda, 437.0, 11.8, 36.0) + 0.681 * g(lambda, 459.0, 26.0, 13.8);

            let r_lin = (3.2406 * x_bar - 1.5372 * y_bar - 0.4986 * z_bar).max(0.0);
            let g_lin = (-0.9689 * x_bar + 1.8758 * y_bar + 0.0415 * z_bar).max(0.0);
            let b_lin = (0.0557 * x_bar - 0.2040 * y_bar + 1.0570 * z_bar).max(0.0);

            let scale_l = 1.0 / r_lin.max(g_lin).max(b_lin).max(1e-4);

            #[inline]
            fn srgb_curve(c: f64) -> u8 {
                let s = if c <= 0.0031308 {
                    12.92 * c
                } else {
                    1.055 * c.powf(1.0 / 2.4) - 0.055
                };
                (s * 255.0).clamp(0.0, 255.0) as u8
            }

            [
                srgb_curve(r_lin * scale_l),
                srgb_curve(g_lin * scale_l),
                srgb_curve(b_lin * scale_l),
            ]
        }

        let rad_angle = orientation_deg * PI / 180.0;
        let cos_a = rad_angle.cos();
        let sin_a = rad_angle.sin();
        let seed_u64 = seed as u64;

        let mut pixels = Vec::with_capacity(n);

        for y in 0..height {
            let v = y as f64 / height as f64;
            for x in 0..width {
                let u = x as f64 / width as f64;

                let px = match basis {
                    "gabor" => {
                        let u_rot = u * cos_a + v * sin_a;
                        let v_rot = -u * sin_a + v * cos_a;
                        let gx = u_rot * scale;
                        let gy = v_rot * scale;
                        let ix = gx.floor() as i32;
                        let iy = gy.floor() as i32;

                        let mut acc = 0.0f64;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let (rx, ry) = hash2d(ix + dx, iy + dy, seed_u64);
                                let del_x = gx - ((ix + dx) as f64 + rx);
                                let del_y = gy - ((iy + dy) as f64 + ry);
                                let env_y = del_y * (1.0 / (1.0 - anisotropy * 0.85).max(0.05));
                                let env = (-PI * (del_x * del_x + env_y * env_y) * 0.75).exp();
                                let carrier = (2.0 * PI * del_x * 1.5 + rx * 6.28).cos();
                                acc += env * carrier;
                            }
                        }
                        let b_val = ((acc * 0.45 + 0.5).clamp(0.0, 1.0) * 255.0) as u8;
                        [b_val, b_val, b_val, 255]
                    }
                    "voronoi" => {
                        let gx = u * scale;
                        let gy = v * scale;
                        let ix = gx.floor() as i32;
                        let iy = gy.floor() as i32;
                        let fx = gx - ix as f64;
                        let fy = gy - iy as f64;

                        let mut d1 = f64::INFINITY;
                        let mut d2 = f64::INFINITY;

                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                let (rx, ry) = hash2d(ix + dx, iy + dy, seed_u64);
                                let vx = dx as f64 + rx - fx;
                                let vy = dy as f64 + ry - fy;
                                let dist = (vx * vx + vy * vy).sqrt();
                                if dist < d1 {
                                    d2 = d1;
                                    d1 = dist;
                                } else if dist < d2 {
                                    d2 = dist;
                                }
                            }
                        }
                        let f1_color = (d1.clamp(0.0, 1.0) * 255.0) as u8;
                        let border = ((d2 - d1).clamp(0.0, 1.0) * 255.0) as u8;
                        let f2_color = (d2.clamp(0.0, 1.0) * 255.0) as u8;
                        [f1_color, border, f2_color, 255]
                    }
                    "wave" => {
                        let u_rot = u * cos_a + v * sin_a;
                        let v_rot = -u * sin_a + v * cos_a;
                        let turb = 0.12 * (2.0 * PI * v_rot * scale * 0.5).sin();
                        let wave_val = 0.5 + 0.5 * (2.0 * PI * (u_rot + turb) * scale).cos();
                        let r = (24.0 + wave_val * (245.0 - 24.0)) as u8;
                        let g = (140.0 + wave_val * (158.0 - 140.0)) as u8;
                        let b = (180.0 + wave_val * (36.0 - 180.0)) as u8;
                        [r, g, b, 255]
                    }
                    "blackbody" => {
                        let [r, g, b] = blackbody_rgb(temperature_k * (0.7 + 0.6 * u));
                        [r, g, b, 255]
                    }
                    "wavelength" => {
                        let [r, g, b] = wavelength_to_srgb(380.0 + u * 400.0);
                        [r, g, b, 255]
                    }
                    _ => [0, 0, 0, 255],
                };
                pixels.push(px);
            }
        }

        self.store_rgba(width as u32, height as u32, pixels)
    }
}

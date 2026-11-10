//! Cascaded SDFGI irradiance probes, 3D froxel atmospheric scattering, and
//! hierarchical light tree bounding-cone clustering.
//! Default `backend=cpu` uses Etalon CPU APIs. `backend=gpu` dispatches the
//! Etalon/wgpu compute path in `gpu_gi` (fail-closed without an adapter).

use crate::*;
use qualia_core_db::etalon_render::frame_quality::{
    distance_attenuation, evaluate_environment_lighting, evaluate_fragment_radiance,
    spot_angular_attenuation, DirectionalLight, EnvironmentProbe, EnvironmentProbeSet,
    SurfaceParameters,
};
use qualia_core_db::etalon_render::geo_viewport::sky_limb::{
    evaluate_sky_limb_and_aerial_perspective, AtmosphereParams,
};
use qualia_core_db::etalon_render::ibl_cubemap::{
    eval_procedural_sky_radiance_zero_heap, IblCubemapParams,
};
use std::collections::BTreeMap;
use vibe::Value;

// Internal light record for light tree clustering.
#[derive(Clone, Debug)]
struct LightDef {
    position: [f64; 3],
    direction: [f64; 3],
    color: [f64; 3],
    power: f64,
    angle_rad: f64,
}

// Bounding cone and measure for hierarchical light tree clustering.
#[derive(Clone, Debug)]
struct LightTreeMeasure {
    bbox_min: [f64; 3],
    bbox_max: [f64; 3],
    axis: [f64; 3],
    theta_o: f64,
    theta_e: f64,
    energy: f64,
}

impl LightTreeMeasure {
    fn from_light(l: &LightDef) -> Self {
        Self {
            bbox_min: l.position,
            bbox_max: l.position,
            axis: l.direction,
            theta_o: 0.0,
            theta_e: l.angle_rad,
            energy: l.power,
        }
    }

    fn merge(&self, other: &Self) -> Self {
        let bbox_min = [
            self.bbox_min[0].min(other.bbox_min[0]),
            self.bbox_min[1].min(other.bbox_min[1]),
            self.bbox_min[2].min(other.bbox_min[2]),
        ];
        let bbox_max = [
            self.bbox_max[0].max(other.bbox_max[0]),
            self.bbox_max[1].max(other.bbox_max[1]),
            self.bbox_max[2].max(other.bbox_max[2]),
        ];
        let energy = self.energy + other.energy;

        // Orientation cone merge following Cycles bounding cone algorithm
        let (axis, theta_o) = if self.energy <= 1e-9 && other.energy <= 1e-9 {
            (self.axis, self.theta_o)
        } else if self.theta_o >= std::f64::consts::PI || other.theta_o >= std::f64::consts::PI {
            (self.axis, std::f64::consts::PI)
        } else {
            let dot = (self.axis[0] * other.axis[0]
                + self.axis[1] * other.axis[1]
                + self.axis[2] * other.axis[2])
                .clamp(-1.0, 1.0);
            let theta_d = dot.acos();

            if theta_d + other.theta_o <= self.theta_o {
                (self.axis, self.theta_o)
            } else if theta_d + self.theta_o <= other.theta_o {
                (other.axis, other.theta_o)
            } else {
                let new_theta_o =
                    ((theta_d + self.theta_o + other.theta_o) * 0.5).min(std::f64::consts::PI);
                let phi = new_theta_o - self.theta_o;

                let new_axis = if theta_d > 1e-6 {
                    let mut ortho = [
                        other.axis[0] - dot * self.axis[0],
                        other.axis[1] - dot * self.axis[1],
                        other.axis[2] - dot * self.axis[2],
                    ];
                    let ortho_len =
                        (ortho[0] * ortho[0] + ortho[1] * ortho[1] + ortho[2] * ortho[2]).sqrt();
                    if ortho_len > 1e-6 {
                        ortho = [
                            ortho[0] / ortho_len,
                            ortho[1] / ortho_len,
                            ortho[2] / ortho_len,
                        ];
                        let cos_phi = phi.cos();
                        let sin_phi = phi.sin();
                        let mut rotated = [
                            cos_phi * self.axis[0] + sin_phi * ortho[0],
                            cos_phi * self.axis[1] + sin_phi * ortho[1],
                            cos_phi * self.axis[2] + sin_phi * ortho[2],
                        ];
                        let rot_len = (rotated[0] * rotated[0]
                            + rotated[1] * rotated[1]
                            + rotated[2] * rotated[2])
                            .sqrt();
                        if rot_len > 1e-6 {
                            rotated = [
                                rotated[0] / rot_len,
                                rotated[1] / rot_len,
                                rotated[2] / rot_len,
                            ];
                        }
                        rotated
                    } else {
                        self.axis
                    }
                } else {
                    self.axis
                };
                (new_axis, new_theta_o)
            }
        };

        let theta_e = self.theta_e.max(other.theta_e);

        Self {
            bbox_min,
            bbox_max,
            axis,
            theta_o,
            theta_e,
            energy,
        }
    }
}

enum LightTreeNode {
    Leaf {
        light_index: usize,
        measure: LightTreeMeasure,
    },
    Inner {
        left: Box<LightTreeNode>,
        right: Box<LightTreeNode>,
        measure: LightTreeMeasure,
    },
}

impl LightTreeNode {
    fn measure(&self) -> &LightTreeMeasure {
        match self {
            Self::Leaf { measure, .. } => measure,
            Self::Inner { measure, .. } => measure,
        }
    }

    fn node_and_leaf_count(&self) -> (usize, usize) {
        match self {
            Self::Leaf { .. } => (1, 1),
            Self::Inner { left, right, .. } => {
                let (nl, ll) = left.node_and_leaf_count();
                let (nr, lr) = right.node_and_leaf_count();
                (nl + nr + 1, ll + lr)
            }
        }
    }
}

fn build_bvh_recursive(lights: &[LightDef], indices: &mut [usize]) -> LightTreeNode {
    if indices.len() == 1 {
        let idx = indices[0];
        let m = LightTreeMeasure::from_light(&lights[idx]);
        return LightTreeNode::Leaf {
            light_index: idx,
            measure: m,
        };
    }

    let mut min_pos = lights[indices[0]].position;
    let mut max_pos = lights[indices[0]].position;
    for &idx in indices.iter().skip(1) {
        let p = lights[idx].position;
        for c in 0..3 {
            min_pos[c] = min_pos[c].min(p[c]);
            max_pos[c] = max_pos[c].max(p[c]);
        }
    }

    let extents = [
        max_pos[0] - min_pos[0],
        max_pos[1] - min_pos[1],
        max_pos[2] - min_pos[2],
    ];
    let mut split_axis = 0;
    if extents[1] > extents[0] && extents[1] >= extents[2] {
        split_axis = 1;
    } else if extents[2] > extents[0] && extents[2] > extents[1] {
        split_axis = 2;
    }

    indices.sort_by(|&a, &b| {
        let pa = lights[a].position[split_axis];
        let pb = lights[b].position[split_axis];
        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mid = indices.len() / 2;
    let (left_indices, right_indices) = indices.split_at_mut(mid);
    let left_node = build_bvh_recursive(lights, left_indices);
    let right_node = build_bvh_recursive(lights, right_indices);
    let merged_m = left_node.measure().merge(right_node.measure());

    LightTreeNode::Inner {
        left: Box::new(left_node),
        right: Box::new(right_node),
        measure: merged_m,
    }
}

fn evaluate_tree_lighting(
    node: &LightTreeNode,
    lights: &[LightDef],
    p: [f64; 3],
    n: [f64; 3],
    accum: &mut [f64; 3],
) {
    match node {
        LightTreeNode::Leaf { light_index, .. } => {
            let l = &lights[*light_index];
            let to_light = [
                l.position[0] - p[0],
                l.position[1] - p[1],
                l.position[2] - p[2],
            ];
            let dist_sq = to_light[0] * to_light[0]
                + to_light[1] * to_light[1]
                + to_light[2] * to_light[2];
            let dist = dist_sq.sqrt();
            if dist > 1e-4 {
                let l_dir = [to_light[0] / dist, to_light[1] / dist, to_light[2] / dist];
                let cos_n = (n[0] * l_dir[0] + n[1] * l_dir[1] + n[2] * l_dir[2]).max(0.0);

                let cos_emit = -(l_dir[0] * l.direction[0]
                    + l_dir[1] * l.direction[1]
                    + l_dir[2] * l.direction[2]);
                let cos_cutoff = l.angle_rad.cos();
                if cos_n > 0.0 && cos_emit >= cos_cutoff {
                    let spot_factor = spot_angular_attenuation(
                        cos_emit as f32,
                        1.0,
                        cos_cutoff as f32,
                    ) as f64;
                    let atten = distance_attenuation(dist_sq as f32, 16.0) as f64;
                    let intensity = l.power * cos_n * spot_factor * atten;
                    accum[0] += l.color[0] * intensity;
                    accum[1] += l.color[1] * intensity;
                    accum[2] += l.color[2] * intensity;
                }
            }
        }
        LightTreeNode::Inner {
            left,
            right,
            measure,
        } => {
            if measure.energy > 1e-9 {
                evaluate_tree_lighting(left, lights, p, n, accum);
                evaluate_tree_lighting(right, lights, p, n, accum);
            }
        }
    }
}

impl AssetHost<'_> {
    /// 4. Cascaded SDFGI irradiance probes and 3D froxel atmospheric scattering
    pub(super) fn radiance_solve_sdfgi_froxel(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "image",
                "cascades",
                "fog_density",
                "scattering_anisotropy_g",
                "sun_direction",
                "sky_radiance",
                "bounce_intensity",
                "backend",
                "profile",
            ],
        )?;
        let image_val = required(a, "image")?;
        let img = self.rgba(image_val)?;
        let (w, h) = (img.width, img.height);
        if w == 0 || h == 0 {
            return Err("Image has zero dimensions".into());
        }

        let cascades = count(a, "cascades", 3, 6)?;
        let fog_density = number(a, "fog_density", 0.18)?;
        if !(0.0..=2.0).contains(&fog_density) {
            return Err("fog_density outside 0.0..=2.0".into());
        }

        let g = number(a, "scattering_anisotropy_g", 0.6)?;
        if !(-0.95..=0.95).contains(&g) {
            return Err("scattering_anisotropy_g outside -0.95..=0.95".into());
        }

        let sun_direction = match a.get("sun_direction") {
            Some(v) => vector::<3>(v)?,
            None => [0.4, 0.7, 0.58],
        };
        let sun_len = (sun_direction[0] * sun_direction[0]
            + sun_direction[1] * sun_direction[1]
            + sun_direction[2] * sun_direction[2])
            .sqrt();
        let sun_dir = if sun_len > 1e-6 {
            [
                sun_direction[0] / sun_len,
                sun_direction[1] / sun_len,
                sun_direction[2] / sun_len,
            ]
        } else {
            [0.0, 1.0, 0.0]
        };

        let sky_radiance = match a.get("sky_radiance") {
            Some(v) => vector::<3>(v)?,
            None => [0.35, 0.55, 0.92],
        };

        let bounce_intensity = number(a, "bounce_intensity", 0.45)?;
        if !(0.0..=4.0).contains(&bounce_intensity) {
            return Err("bounce_intensity outside 0.0..=4.0".into());
        }

        let backend = match a.get("backend") {
            None => "cpu",
            Some(v) => text(v)?,
        };
        if backend != "cpu" && backend != "gpu" {
            return Err(format!(
                "Unknown backend '{backend}'; expected cpu or gpu"
            ));
        }

        let n = (w as usize) * (h as usize);
        self.charge(n * (cascades * 3 + 10), n * 4)?;

        if backend == "gpu" {
            let img = self.rgba(image_val)?;
            let gpu = super::gpu_gi::dispatch_gpu_sdfgi(
                &img.pixels,
                super::gpu_gi::GpuGiParams {
                    width: w,
                    height: h,
                    cascades: cascades as u32,
                    fog_density: fog_density as f32,
                    anisotropy_g: g as f32,
                    sun_dir: [sun_dir[0] as f32, sun_dir[1] as f32, sun_dir[2] as f32],
                    sky_radiance: [
                        sky_radiance[0] as f32,
                        sky_radiance[1] as f32,
                        sky_radiance[2] as f32,
                    ],
                    bounce_intensity: bounce_intensity as f32,
                },
            )?;
            let img_handle = self.store_rgba(w, h, gpu.pixels)?;
            return Ok(Value::Record(BTreeMap::from([
                ("adapter".into(), Value::String(gpu.adapter_name)),
                ("backend".into(), Value::String("gpu".into())),
                ("cascades".into(), Value::U64(cascades as u64)),
                ("image".into(), img_handle),
                ("mean_irradiance".into(), Value::F64(gpu.mean_irradiance)),
                (
                    "transmittance".into(),
                    Value::F64(gpu.mean_transmittance),
                ),
            ])));
        }

        let sun_f = [sun_dir[0] as f32, sun_dir[1] as f32, sun_dir[2] as f32];
        let sky_f = [
            sky_radiance[0] as f32,
            sky_radiance[1] as f32,
            sky_radiance[2] as f32,
        ];
        let ibl = IblCubemapParams {
            sun_dir_and_turbidity: [sun_f[0], sun_f[1], sun_f[2], 2.2],
            ground_albedo_and_intensity: [
                sky_f[0],
                sky_f[1],
                sky_f[2],
                bounce_intensity.max(0.05) as f32,
            ],
            zenith_and_mie_g: [0.22, 0.48, 0.92, g.clamp(0.0, 0.96) as f32],
            roughness_and_dims: [0.45, 16.0, 16.0, 1.0],
        };
        let atm = AtmosphereParams {
            planet_radius_m: 6_371_000.0,
            atmosphere_top_m: 100_000.0,
            rayleigh_scale_h_m: 8_000.0,
            mie_scale_h_m: 1_200.0,
            beta_rayleigh: [5.8e-6, 13.5e-6, 33.1e-6],
            mie_g: g as f32,
            beta_mie: [
                2.0e-6 * fog_density as f32 + 1.0e-6,
                2.0e-6 * fog_density as f32 + 1.0e-6,
                2.0e-6 * fog_density as f32 + 1.0e-6,
            ],
            sun_intensity: (12.0 * bounce_intensity.max(0.1)) as f32,
            sun_dir: sun_f,
            ozone_peak_h_m: 25_000.0,
        };
        let mut probes = EnvironmentProbeSet::new();
        for c in 0..cascades.min(4) {
            let scale = 1.0 / ((c + 1) as f32);
            let sky = eval_procedural_sky_radiance_zero_heap([0.0, 1.0, 0.0], &ibl);
            probes.push(EnvironmentProbe {
                position: [0.0, 0.35 * (c as f32 + 1.0), 0.0],
                radius: 6.0 * (c as f32 + 1.0),
                irradiance: [
                    (sky[0] + sky_f[0] * scale) * bounce_intensity as f32,
                    (sky[1] + sky_f[1] * scale) * bounce_intensity as f32,
                    (sky[2] + sky_f[2] * scale) * bounce_intensity as f32,
                ],
                specular_levels: [sky; 5],
                dominant_direction: sun_f,
                valid: true,
            });
        }
        let sun_light = DirectionalLight {
            direction: [-sun_f[0], -sun_f[1], -sun_f[2]],
            illuminance_lux: 1.8,
            color_linear: [1.2, 1.1, 0.9],
            cast_shadows: false,
        };

        let img = self.rgba(image_val)?;
        let mut out_pixels = Vec::with_capacity(n);
        let mut total_transmittance = 0.0f64;
        let mut total_irradiance = 0.0f64;

        for y in 0..h {
            let v = (y as f64 + 0.5) / (h as f64);
            let ny = (0.5 - v) * 2.0;

            for x in 0..w {
                let u = (x as f64 + 0.5) / (w as f64);
                let nx = (u - 0.5) * 2.0;

                let ray_len = (nx * nx + ny * ny + 1.0).sqrt();
                let ray_dir = [nx / ray_len, ny / ray_len, 1.0 / ray_len];
                let ray_f = [ray_dir[0] as f32, ray_dir[1] as f32, ray_dir[2] as f32];

                let dist_m = 400.0 + fog_density as f32 * 1_200.0 * (1.0 - v as f32);
                let (inscatter, trans) =
                    evaluate_sky_limb_and_aerial_perspective(&atm, 80.0, ray_f, dist_m, 6);
                let trans_mean =
                    ((trans[0] + trans[1] + trans[2]) / 3.0).clamp(0.0, 1.0) as f64;
                let fog_t = (-fog_density * (0.35 + 0.45 * (1.0 - v))).exp();
                let transmittance = (0.55 * trans_mean + 0.45 * fog_t).clamp(1e-4, 1.0);

                let sky = eval_procedural_sky_radiance_zero_heap(ray_f, &ibl);
                let p = img.pixels[y as usize * (w as usize) + x as usize];
                let surface = SurfaceParameters {
                    base_color: [
                        p[0] as f32 / 255.0,
                        p[1] as f32 / 255.0,
                        p[2] as f32 / 255.0,
                    ],
                    metallic: 0.0,
                    roughness: 0.55,
                    reflectance_f0: 0.04,
                    emissive: [0.0; 3],
                    ambient_occlusion: 1.0,
                };
                let env = evaluate_environment_lighting(
                    &surface,
                    [nx as f32, 0.0, ny as f32],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.2, 1.0],
                    Some(&probes),
                    sky,
                    None,
                );
                let frag = evaluate_fragment_radiance(
                    &surface,
                    [0.0, 1.0, 0.0],
                    [0.0, 0.2, 1.0],
                    Some(&sun_light),
                    1.0,
                    env.radiance,
                    None,
                );
                let mean_irr_px =
                    ((env.radiance[0] + env.radiance[1] + env.radiance[2]) / 3.0).max(0.0) as f64
                        + bounce_intensity * 0.05;
                total_irradiance += mean_irr_px;
                total_transmittance += transmittance;

                let mut out_rgb = [0u8; 3];
                for ch in 0..3 {
                    let lit = frag[ch] as f64 * transmittance
                        + inscatter[ch] as f64
                        + sky[ch] as f64 * (1.0 - transmittance) * 0.25;
                    let mapped = (lit / (1.0 + lit) * 1.4).clamp(0.0, 1.0);
                    out_rgb[ch] = (mapped * 255.0).round() as u8;
                }

                out_pixels.push([out_rgb[0], out_rgb[1], out_rgb[2], p[3]]);
            }
        }

        let n_f = n as f64;
        let mean_transmittance = total_transmittance / n_f;
        let mean_irradiance = total_irradiance / n_f;
        let img_handle = self.store_rgba(w, h, out_pixels)?;

        Ok(Value::Record(BTreeMap::from([
            ("backend".into(), Value::String("cpu".into())),
            ("cascades".into(), Value::U64(cascades as u64)),
            ("image".into(), img_handle),
            ("mean_irradiance".into(), Value::F64(mean_irradiance)),
            ("transmittance".into(), Value::F64(mean_transmittance)),
        ])))
    }

    /// 5. Hierarchical light tree bounding-cone clustering and direct lighting preview
    pub(super) fn radiance_build_light_tree(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["lights", "width", "height", "profile"])?;
        let lights_val = required(a, "lights")?;
        let lights_list = list(lights_val)?;
        if lights_list.is_empty() || lights_list.len() > 64 {
            return Err("lights must contain between 1 and 64 lights".into());
        }

        let profile = crate::graphics::lattice_profile(a)?;
        let width = count(a, "width", 64, profile.max_dim as usize)?;
        let height = count(a, "height", 64, profile.max_dim as usize)?;

        let mut lights = Vec::with_capacity(lights_list.len());
        for item in lights_list {
            let r = record(item)?;
            fields(
                r,
                &["position", "direction", "color", "power", "angle_rad"],
            )?;
            let position = vector::<3>(required(r, "position")?)?;
            let dir_raw = vector::<3>(required(r, "direction")?)?;
            let d_len = (dir_raw[0] * dir_raw[0]
                + dir_raw[1] * dir_raw[1]
                + dir_raw[2] * dir_raw[2])
                .sqrt();
            let direction = if d_len > 1e-6 {
                [dir_raw[0] / d_len, dir_raw[1] / d_len, dir_raw[2] / d_len]
            } else {
                [0.0, -1.0, 0.0]
            };
            let color = vector::<3>(required(r, "color")?)?;
            let power = number(r, "power", 1.0)?;
            if power < 0.0 || !power.is_finite() {
                return Err("Light power must be non-negative".into());
            }
            let angle_rad = number(r, "angle_rad", std::f64::consts::FRAC_PI_2)?;
            if !(0.0..=std::f64::consts::PI).contains(&angle_rad) {
                return Err("Light angle_rad outside 0.0..=PI".into());
            }

            lights.push(LightDef {
                position,
                direction,
                color,
                power,
                angle_rad,
            });
        }

        let n_px = width * height;
        self.charge(lights.len() * 10 + n_px * 20, n_px * 4)?;

        let mut indices: Vec<usize> = (0..lights.len()).collect();
        let root = build_bvh_recursive(&lights, &mut indices);
        let (node_count, leaf_count) = root.node_and_leaf_count();
        let total_power: f64 = lights.iter().map(|l| l.power).sum();
        let root_measure = root.measure().clone();

        let center = [
            (root_measure.bbox_min[0] + root_measure.bbox_max[0]) * 0.5,
            (root_measure.bbox_min[1] + root_measure.bbox_max[1]) * 0.5,
            (root_measure.bbox_min[2] + root_measure.bbox_max[2]) * 0.5,
        ];
        let extent = (root_measure.bbox_max[0] - root_measure.bbox_min[0])
            .max(root_measure.bbox_max[2] - root_measure.bbox_min[2])
            .max(root_measure.bbox_max[1] - root_measure.bbox_min[1])
            .max(4.0);
        let ground_radius = extent * 1.5;
        let ground_y = (root_measure.bbox_min[1] - 1.0).min(0.0);

        let mut preview_pixels = Vec::with_capacity(n_px);
        for y in 0..height {
            let vz = (y as f64 + 0.5) / (height as f64) - 0.5;
            let pz = center[2] + vz * 2.0 * ground_radius;

            for x in 0..width {
                let vx = (x as f64 + 0.5) / (width as f64) - 0.5;
                let px = center[0] + vx * 2.0 * ground_radius;

                let p_surface = [px, ground_y, pz];
                let normal = [0.0f64, 1.0f64, 0.0f64];

                let mut total_rgb = [0.0f64; 3];
                evaluate_tree_lighting(&root, &lights, p_surface, normal, &mut total_rgb);

                let mapped = [
                    (total_rgb[0] / (1.0 + total_rgb[0]) * 1.3).clamp(0.0, 1.0),
                    (total_rgb[1] / (1.0 + total_rgb[1]) * 1.3).clamp(0.0, 1.0),
                    (total_rgb[2] / (1.0 + total_rgb[2]) * 1.3).clamp(0.0, 1.0),
                ];

                preview_pixels.push([
                    (mapped[0] * 255.0).round() as u8,
                    (mapped[1] * 255.0).round() as u8,
                    (mapped[2] * 255.0).round() as u8,
                    255,
                ]);
            }
        }

        let preview_rgba = self.store_rgba(width as u32, height as u32, preview_pixels)?;

        Ok(Value::Record(BTreeMap::from([
            ("leaf_count".into(), Value::U64(leaf_count as u64)),
            ("node_count".into(), Value::U64(node_count as u64)),
            ("preview_rgba".into(), preview_rgba),
            ("root_theta_e".into(), Value::F64(root_measure.theta_e)),
            ("root_theta_o".into(), Value::F64(root_measure.theta_o)),
            ("total_power".into(), Value::F64(total_power)),
        ])))
    }
}

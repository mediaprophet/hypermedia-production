//! Seeded CPU path tracing against the native production mesh BVH.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{
    f32::consts::{PI, TAU},
    sync::Arc,
};

use crate::artifact::{FieldLattice, LatticeSamples, RadianceLattice};
use crate::primal_replay::{counter_uniform, COUNTER_ALGORITHM};
use crate::*;
use sha2::{Digest, Sha256};
use surface_pbr_studio::{Bvh, Hit, Mesh, Ray, Vec3};

const MAX_VERTICES: usize = 65_536;
const MAX_TRIANGLES: usize = 65_536;
const REC709_LUMINANCE: Vec3 = Vec3::new(0.2126, 0.7152, 0.0722);
const AOV_CHANNELS: [&str; 8] = [
    "primary_coverage",
    "primary_depth_m",
    "normal_x",
    "normal_y",
    "normal_z",
    "albedo_r",
    "albedo_g",
    "albedo_b",
];

#[derive(Clone, Copy)]
struct Material {
    albedo: Vec3,
    emission: Vec3,
}

struct Geometry {
    mesh: Mesh,
    bvh: Bvh,
    materials: Vec<Material>,
}

struct Light {
    position: Vec3,
    intensity: Vec3,
}

struct Camera {
    eye: Vec3,
    forward: Vec3,
    right: Vec3,
    up: Vec3,
    half_height: f32,
}

impl AssetHost<'_> {
    pub(crate) fn path_trace_scene(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "meshes",
                "camera",
                "resolution",
                "spp",
                "max_bounces",
                "seed",
                "environment_linear_rgb",
                "point_lights",
            ],
        )?;
        let [width, height] = list(required(args, "resolution")?)? else {
            return Err("Path trace resolution must be [width,height]".into());
        };
        let (width, height) = (integer(width, "width")?, integer(height, "height")?);
        if !(1..=1024).contains(&width) || !(1..=1024).contains(&height) {
            return Err("Path trace image dimensions must each be within 1 to 1024".into());
        }
        let spp = count(args, "spp", 8, 128)?;
        let bounces = count(args, "max_bounces", 4, 12)?;
        if spp == 0 || bounces == 0 {
            return Err("Path tracing needs positive samples and bounce limits".into());
        }
        let seed = match required(args, "seed")? {
            Value::U64(seed) => *seed,
            Value::I64(seed) if *seed >= 0 => *seed as u64,
            _ => return Err("Path trace seed must be unsigned 64-bit".into()),
        };
        let camera = camera(record(required(args, "camera")?)?)?;
        let environment = args
            .get("environment_linear_rgb")
            .map(|v| vec3(v, 0.0, 10_000.0))
            .transpose()?
            .unwrap_or(Vec3::ZERO);
        let lights = path_lights(args)?;
        let count = width * height;
        let output_bytes = count * (5 + AOV_CHANNELS.len()) * 4;
        if count * AOV_CHANNELS.len() * 4 > self.limits.resource_bytes
            || output_bytes > self.limits.total_resource_bytes
        {
            return Err("Path trace outputs exceed the admitted resource budget".into());
        }
        let geometry = self.path_trace_geometry(required(args, "meshes")?)?;
        let scene_digest = scene_digest(&geometry, &camera, &lights, environment);
        let estimates = geometry.mesh.triangle_count().ilog2() as usize + 1;
        let work = count
            .saturating_mul(spp)
            .saturating_mul(bounces + 1)
            .saturating_mul(1 + lights.len())
            .saturating_mul(24 + estimates * 4);
        self.charge(work, output_bytes + geometry.mesh.triangle_count() * 160)?;
        let mut pixels = Vec::with_capacity(count * 4);
        let mut scalar = Vec::with_capacity(count);
        let mut aovs = vec![0.0_f32; AOV_CHANNELS.len() * count];
        let aspect = width as f32 / height as f32;
        for row in 0..height {
            self.charge(0, 0)?;
            for col in 0..width {
                let pixel = row * width + col;
                let mut rgb = Vec3::ZERO;
                let mut albedo = Vec3::ZERO;
                let mut normals = Vec3::ZERO;
                let mut depth = 0.0;
                let mut hits = 0;
                for sample in 0..spp {
                    let stream = pixel as u64 * 128 + sample as u64;
                    let ray = camera.ray([col, row], [width, height], aspect, seed, stream);
                    let primary = geometry.bvh.intersect(&geometry.mesh, &ray);
                    if let Some(hit) = primary {
                        hits += 1;
                        depth += hit.distance;
                        normals += face_normal(hit.normal, ray.direction);
                        albedo += geometry.materials[hit.triangle_index as usize].albedo;
                    }
                    rgb += radiance(
                        &geometry,
                        &lights,
                        environment,
                        ray,
                        primary,
                        seed,
                        stream,
                        bounces,
                        None,
                    );
                }
                let rgb = rgb / spp as f32;
                let normal = normals.normalize();
                let avg = 1.0 / hits.max(1) as f32;
                let albedo = albedo * avg;
                let entries = [
                    hits as f32 / spp as f32,
                    depth * avg,
                    normal.x,
                    normal.y,
                    normal.z,
                    albedo.x,
                    albedo.y,
                    albedo.z,
                ];
                let luminance = rgb.dot(REC709_LUMINANCE);
                if !luminance.is_finite() || entries.iter().any(|x| !x.is_finite()) {
                    return Err("Path trace produced a non-finite radiance or AOV sample".into());
                }
                pixels.extend([rgb.x, rgb.y, rgb.z, 1.0]);
                scalar.push(luminance);
                for (lane, value) in entries.into_iter().enumerate() {
                    aovs[lane * count + pixel] = value;
                }
            }
        }
        let radiance =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: width as u32,
                height: height as u32,
                channels: 4,
                primaries_iri: "urn:qualia:color:primaries:rec709".into(),
                transfer_iri: "urn:qualia:color:transfer:linear".into(),
                samples: LatticeSamples::F32(pixels),
            })))?;
        let aovs = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![AOV_CHANNELS.len() as u32, height as u32, width as u32],
                samples: aovs,
            },
        )))?;
        let primal = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![height as u32, width as u32],
                samples: scalar,
            },
        )))?;
        Ok(Value::Record(Args::from([
            ("radiance".into(), radiance),
            ("aovs".into(), aovs),
            ("primal_pixel_luminance".into(), primal),
            (
                "aov_channels".into(),
                Value::List(AOV_CHANNELS.map(|name| Value::String(name.into())).into()),
            ),
            ("replay_seed".into(), Value::U64(seed)),
            ("primal_scene_sha256".into(), Value::String(scene_digest)),
            ("replay_rng".into(), Value::String(COUNTER_ALGORITHM.into())),
            ("samples_per_pixel".into(), Value::U64(spp as u64)),
            ("max_bounces".into(), Value::U64(bounces as u64)),
        ])))
    }

    /// Fit one nonnegative environment-radiance scale on the fixed Lambertian sampler.
    /// The derivative is the throughput at environment escape, so replay needs no path tape.
    pub(crate) fn solve_adjoint_prb_inverse_rendering(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "meshes",
                "camera",
                "target",
                "point_lights",
                "environment_basis_rgb",
                "path_depth",
                "spp_budget",
                "radiance_w_sr_m2",
                "seed",
            ],
        )?;
        let target = fit_target(self.workspace.typed(required(args, "target")?)?)?;
        let (width, height) = (target.width, target.height);
        let spp = count(args, "spp_budget", 8, 128)?;
        let bounces = count(args, "path_depth", 4, 64)?;
        let seed = match required(args, "seed")? {
            Value::U64(seed) => *seed,
            Value::I64(seed) if *seed >= 0 => *seed as u64,
            _ => return Err("Replay seed must be unsigned 64-bit".into()),
        };
        let basis = vec3(required(args, "environment_basis_rgb")?, 0.0, 1.0)?;
        let limit = number(args, "radiance_w_sr_m2", 10_000.0)?;
        if basis == Vec3::ZERO || !(0.0..=10_000.0).contains(&limit) {
            return Err(
                "Replay needs a nonzero environment basis and gain cap in [0,10000] W/(sr m²)"
                    .into(),
            );
        }
        let camera = camera(record(required(args, "camera")?)?)?;
        let lights = path_lights(args)?;
        let geometry = self.path_trace_geometry(required(args, "meshes")?)?;
        let digest = scene_digest(&geometry, &camera, &lights, basis);
        let count = width * height;
        if count * size_of::<f32>() > self.limits.resource_bytes {
            return Err("Replay fitted luminance exceeds the admitted resource budget".into());
        }
        let estimates = geometry.mesh.triangle_count().ilog2() as usize + 1;
        let work = count
            .saturating_mul(spp)
            .saturating_mul(bounces + 1)
            .saturating_mul(1 + lights.len())
            .saturating_mul(28 + estimates * 4);
        self.charge(work, count * 20 + geometry.mesh.triangle_count() * 160)?;
        let mut replay = Vec::with_capacity(count);
        let mut target_gradient = 0.0;
        let mut squared_jacobian = 0.0;
        let mut loss_at_zero = 0.0;
        let mut target_digest = Sha256::new();
        target_digest.update(b"qualia:lambertian-replay-luminance-target:v1\0");
        target_digest.update((width as u32).to_le_bytes());
        target_digest.update((height as u32).to_le_bytes());
        let aspect = width as f32 / height as f32;
        for row in 0..height {
            self.charge(0, 0)?;
            for col in 0..width {
                let pixel = row * width + col;
                let mut basal = Vec3::ZERO;
                let mut escaped = Vec3::ZERO;
                for sample in 0..spp {
                    let stream = pixel as u64 * 128 + sample as u64;
                    let ray = camera.ray([col, row], [width, height], aspect, seed, stream);
                    let primary = geometry.bvh.intersect(&geometry.mesh, &ray);
                    let mut environment_weight = Vec3::ZERO;
                    basal += radiance(
                        &geometry,
                        &lights,
                        Vec3::ZERO,
                        ray,
                        primary,
                        seed,
                        stream,
                        bounces,
                        Some(&mut environment_weight),
                    );
                    escaped += environment_weight * basis;
                }
                let intercept = f64::from((basal / spp as f32).dot(REC709_LUMINANCE));
                let jacobian = f64::from((escaped / spp as f32).dot(REC709_LUMINANCE));
                if !intercept.is_finite() || !jacobian.is_finite() {
                    return Err("Replay produced non-finite transport coefficients".into());
                }
                let observed = target.luminance(pixel);
                target_digest.update(observed.to_le_bytes());
                let residual = observed - intercept;
                target_gradient += residual * jacobian;
                squared_jacobian += jacobian * jacobian;
                loss_at_zero += residual * residual;
                replay.push([intercept, jacobian]);
            }
        }
        if squared_jacobian == 0.0 {
            return Err("Environment gain is unidentifiable from these sampled paths".into());
        }
        let gain = (target_gradient / squared_jacobian).clamp(0.0, limit);
        let mut loss = 0.0;
        let fitted: Vec<_> = replay
            .into_iter()
            .enumerate()
            .map(|(pixel, [offset, slope])| {
                let prediction = offset + gain * slope;
                loss += (prediction - target.luminance(pixel)).powi(2);
                prediction as f32
            })
            .collect();
        let mse = loss / count as f64;
        let gradient = 2.0 * (gain * squared_jacobian - target_gradient) / count as f64;
        let mut facts = vec![
            ("gain_w_sr_m2", gain),
            ("upper_bound_w_sr_m2", limit),
            ("mse", mse),
            ("loss_at_zero", loss_at_zero / count as f64),
            ("mse_gradient", gradient),
            ("width", width as f64),
            ("height", height as f64),
            ("samples_per_pixel", spp as f64),
            ("path_depth", bounces as f64),
            ("seed_high", (seed >> 32) as f64),
            ("seed_low", (seed as u32) as f64),
        ];
        let digest_keys = [
            "scene_sha256_0",
            "scene_sha256_1",
            "scene_sha256_2",
            "scene_sha256_3",
            "scene_sha256_4",
            "scene_sha256_5",
            "scene_sha256_6",
            "scene_sha256_7",
        ];
        for (i, key) in digest_keys.into_iter().enumerate() {
            let word =
                u32::from_str_radix(&digest[8 * i..8 * (i + 1)], 16).map_err(|e| e.to_string())?;
            facts.push((key, f64::from(word)));
        }
        let target_digest = target_digest.finalize();
        let target_digest_keys = [
            "target_sha256_0",
            "target_sha256_1",
            "target_sha256_2",
            "target_sha256_3",
            "target_sha256_4",
            "target_sha256_5",
            "target_sha256_6",
            "target_sha256_7",
        ];
        for (key, word) in target_digest_keys
            .into_iter()
            .zip(target_digest.chunks_exact(4))
        {
            facts.push((
                key,
                f64::from(u32::from_be_bytes([word[0], word[1], word[2], word[3]])),
            ));
        }
        let receipt = crate::swarm_render::receipt(
            self,
            "lambertian-replay-environment-fit",
            &fitted,
            &facts,
        )?;
        let fitted = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![height as u32, width as u32],
                samples: fitted,
            },
        )))?;
        Ok(Value::Record(Args::from([
            ("radiance_w_sr_m2".into(), Value::F64(gain)),
            ("mse".into(), Value::F64(mse)),
            ("mse_gradient".into(), Value::F64(gradient)),
            ("fitted_luminance".into(), fitted),
            ("primal_scene_sha256".into(), Value::String(digest)),
            ("receipt".into(), receipt),
        ])))
    }

    fn path_trace_geometry(&mut self, meshes: &Value) -> Result<Geometry, String> {
        let meshes = list(meshes)?;
        if !(1..=32).contains(&meshes.len()) {
            return Err("Path tracing requires 1 to 32 authored mesh parts".into());
        }
        let mut mesh = Mesh::new();
        let mut materials = Vec::new();
        for part in meshes {
            let row = record(part)?;
            fields(row, &["mesh", "albedo_linear_rgb", "emission_linear_rgb"])?;
            let handle = required(row, "mesh")?;
            let TypedArtifact::AttributedMesh(source) = self.workspace.typed(handle)? else {
                return Err("Path trace mesh parts require retained AttributedMesh handles".into());
            };
            let source = &source.surface.mesh;
            if source.positions.is_empty()
                || source.triangles.is_empty()
                || mesh.positions.len() + source.positions.len() > MAX_VERTICES
                || materials.len() + source.triangles.len() > MAX_TRIANGLES
            {
                return Err(
                    "Path trace mesh exceeds the admitted vertex or triangle budget".into(),
                );
            }
            if source
                .positions
                .iter()
                .flatten()
                .any(|x| !(-10_000.0..=10_000.0).contains(x))
            {
                return Err(
                    "Path trace vertices must be finite in the admitted metre frame".into(),
                );
            }
            let material = Material {
                albedo: vec3(required(row, "albedo_linear_rgb")?, 0.0, 1.0)?,
                emission: row
                    .get("emission_linear_rgb")
                    .map(|v| vec3(v, 0.0, 10_000.0))
                    .transpose()?
                    .unwrap_or(Vec3::ZERO),
            };
            let offset = mesh.positions.len() as u32;
            mesh.positions
                .extend(source.positions.iter().copied().map(Vec3::from_array));
            for &[a, b, c] in &source.triangles {
                if [a, b, c]
                    .iter()
                    .any(|&slot| slot as usize >= source.positions.len())
                {
                    return Err("Path trace mesh has an out-of-range triangle index".into());
                }
                let [a, b, c] = [a + offset, b + offset, c + offset];
                let [p, q, r] = [a, b, c].map(|i| mesh.positions[i as usize]);
                if (q - p).cross(r - p).length_squared() <= 1e-16 {
                    return Err("Path trace mesh contains a degenerate triangle".into());
                }
                mesh.indices.extend([a, b, c]);
                materials.push(material);
            }
        }
        self.charge(
            materials.len() * 64,
            materials.len() * 160 + mesh.positions.len() * 12,
        )?;
        let bvh = Bvh::build(&mesh);
        Ok(Geometry {
            mesh,
            bvh,
            materials,
        })
    }
}

struct FitTarget {
    width: usize,
    height: usize,
    samples: FitSamples,
}

enum FitSamples {
    Luminance(Arc<FieldLattice>),
    Radiance(Arc<RadianceLattice>),
}

impl FitTarget {
    fn luminance(&self, pixel: usize) -> f64 {
        match &self.samples {
            FitSamples::Luminance(field) => {
                let FieldLattice::DenseF32 { samples, .. } = field.as_ref() else {
                    unreachable!()
                };
                f64::from(samples[pixel])
            }
            FitSamples::Radiance(image) => {
                let LatticeSamples::F32(samples) = &image.samples else {
                    unreachable!()
                };
                let start = pixel * usize::from(image.channels);
                f64::from(
                    Vec3::new(samples[start], samples[start + 1], samples[start + 2])
                        .dot(REC709_LUMINANCE),
                )
            }
        }
    }
}

fn fit_target(target: &TypedArtifact) -> Result<FitTarget, String> {
    let (width, height, channels, source, samples) = match target {
        TypedArtifact::FieldLattice(field) => {
            let FieldLattice::DenseF32 { axes, samples } = field.as_ref() else {
                return Err("Replay target must be a DenseF32 luminance raster".into());
            };
            let [height, width] = axes.as_slice() else {
                return Err("Replay luminance raster must have shape [height,width]".into());
            };
            (
                *width,
                *height,
                1,
                FitSamples::Luminance(Arc::clone(field)),
                samples,
            )
        }
        TypedArtifact::RadianceLattice(image) => {
            let LatticeSamples::F32(samples) = &image.samples else {
                return Err("Replay radiance target requires F32 samples".into());
            };
            if !matches!(image.channels, 3 | 4)
                || image.primaries_iri != "urn:qualia:color:primaries:rec709"
                || image.transfer_iri != "urn:qualia:color:transfer:linear"
            {
                return Err("Replay radiance target requires linear Rec.709 RGB or RGBA".into());
            }
            (
                image.width,
                image.height,
                usize::from(image.channels),
                FitSamples::Radiance(Arc::clone(image)),
                samples,
            )
        }
        _ => return Err("Replay requires a retained luminance or radiance target".into()),
    };
    if !(1..=1024).contains(&width)
        || !(1..=1024).contains(&height)
        || samples.len() != width as usize * height as usize * channels
        || samples.iter().any(|value| !(0.0..=1e12).contains(value))
    {
        return Err("Replay requires complete finite nonnegative target samples within 1..=1024 pixels per axis".into());
    }
    Ok(FitTarget {
        width: width as usize,
        height: height as usize,
        samples: source,
    })
}

fn path_lights(args: &Args) -> Result<Vec<Light>, String> {
    let lights = args
        .get("point_lights")
        .map(list)
        .transpose()?
        .unwrap_or_default();
    if lights.len() > 8 {
        return Err("Path tracing accepts up to eight point lights".into());
    }
    lights
        .iter()
        .map(|value| {
            let row = record(value)?;
            fields(row, &["position_m", "intensity_rgb"])?;
            Ok(Light {
                position: vec3(required(row, "position_m")?, -10_000.0, 10_000.0)?,
                intensity: vec3(required(row, "intensity_rgb")?, 0.0, 1e8)?,
            })
        })
        .collect()
}

impl Camera {
    fn ray(
        &self,
        [col, row]: [usize; 2],
        [width, height]: [usize; 2],
        aspect: f32,
        seed: u64,
        stream: u64,
    ) -> Ray {
        let u = (col as f32 + counter_uniform(seed, stream, 0) as f32) / width as f32;
        let v = (row as f32 + counter_uniform(seed, stream, 1) as f32) / height as f32;
        let direction = self.forward
            + self.right * ((2.0 * u - 1.0) * self.half_height * aspect)
            + self.up * ((1.0 - 2.0 * v) * self.half_height);
        Ray::new(self.eye, direction)
    }
}

fn camera(args: &Args) -> Result<Camera, String> {
    fields(args, &["eye_m", "target_m", "up", "vertical_fov_deg"])?;
    let eye = vec3(required(args, "eye_m")?, -10_000.0, 10_000.0)?;
    let forward = (vec3(required(args, "target_m")?, -10_000.0, 10_000.0)? - eye).normalize();
    let up_hint = vec3(required(args, "up")?, -1.0, 1.0)?.normalize();
    let right = forward.cross(up_hint).normalize();
    let fov = num(required(args, "vertical_fov_deg")?)?;
    if forward == Vec3::ZERO || right == Vec3::ZERO || !(1.0..=160.0).contains(&fov) {
        return Err("Path trace camera requires a nondegenerate look frame and 1–160° vertical field of view".into());
    }
    Ok(Camera {
        eye,
        forward,
        right,
        up: right.cross(forward),
        half_height: (0.5 * fov as f32).to_radians().tan(),
    })
}

fn vec3(value: &Value, min: f64, max: f64) -> Result<Vec3, String> {
    let v = vector::<3>(value)?;
    if v.iter().any(|x| !(min..=max).contains(x)) {
        return Err(format!(
            "Path trace vector components must be within {min} to {max}"
        ));
    }
    Ok(Vec3::from_array(v.map(|x| x as f32)))
}

fn scene_digest(scene: &Geometry, camera: &Camera, lights: &[Light], environment: Vec3) -> String {
    let mut digest = Sha256::new();
    digest.update(b"qualia:lambertian-primal-scene:v1\0");
    digest.update((scene.mesh.positions.len() as u64).to_le_bytes());
    digest.update((scene.materials.len() as u64).to_le_bytes());
    digest.update((lights.len() as u64).to_le_bytes());
    for &position in &scene.mesh.positions {
        hash_vec(&mut digest, position);
    }
    for &index in &scene.mesh.indices {
        digest.update(index.to_le_bytes());
    }
    for material in &scene.materials {
        hash_vec(&mut digest, material.albedo);
        hash_vec(&mut digest, material.emission);
    }
    for light in lights {
        hash_vec(&mut digest, light.position);
        hash_vec(&mut digest, light.intensity);
    }
    for value in [
        camera.eye,
        camera.forward,
        camera.right,
        camera.up,
        environment,
    ] {
        hash_vec(&mut digest, value);
    }
    digest.update(camera.half_height.to_le_bytes());
    format!("{:x}", digest.finalize())
}

fn hash_vec(digest: &mut Sha256, vector: Vec3) {
    for value in vector.to_array() {
        digest.update(value.to_le_bytes());
    }
}

fn face_normal(normal: Vec3, direction: Vec3) -> Vec3 {
    if normal.dot(direction) > 0.0 {
        -normal
    } else {
        normal
    }
}

fn radiance(
    scene: &Geometry,
    lights: &[Light],
    environment: Vec3,
    mut ray: Ray,
    mut hit: Option<Hit>,
    seed: u64,
    stream: u64,
    bounces: usize,
    mut environment_weight: Option<&mut Vec3>,
) -> Vec3 {
    let mut result = Vec3::ZERO;
    let mut beta = Vec3::ONE;
    for bounce in 0..=bounces {
        let Some(surface) = hit else {
            result += beta * environment;
            if let Some(weight) = environment_weight.take() {
                *weight = beta;
            }
            break;
        };
        let normal = face_normal(surface.normal, ray.direction);
        let material = scene.materials[surface.triangle_index as usize];
        result += beta * material.emission;
        if bounce == bounces {
            break;
        }
        beta = beta * material.albedo;
        let bias = 1e-5 * surface.point.length().max(1.0);
        let origin = surface.point + normal * bias;
        for light in lights {
            let delta = light.position - origin;
            let r2 = delta.length_squared();
            if r2 <= bias * bias {
                continue;
            }
            let distance = r2.sqrt();
            let direction = delta / distance;
            let cosine = normal.dot(direction);
            if cosine > 0.0
                && scene
                    .bvh
                    .intersect(&scene.mesh, &Ray::new(origin, direction))
                    .is_none_or(|blocker| blocker.distance >= distance)
            {
                result += beta * light.intensity * (cosine / (PI * r2));
            }
        }
        let counter = 2 + bounce as u64 * 2;
        let u = counter_uniform(seed, stream, counter) as f32;
        let theta = TAU * counter_uniform(seed, stream, counter + 1) as f32;
        let tangent = (if normal.y.abs() < 0.99 {
            Vec3::Y
        } else {
            Vec3::Z
        })
        .cross(normal)
        .normalize();
        let bitangent = normal.cross(tangent);
        let direction = tangent * (u.sqrt() * theta.cos())
            + bitangent * (u.sqrt() * theta.sin())
            + normal * (1.0 - u).sqrt();
        ray = Ray::new(origin, direction);
        hit = scene.bvh.intersect(&scene.mesh, &ray);
    }
    result
}

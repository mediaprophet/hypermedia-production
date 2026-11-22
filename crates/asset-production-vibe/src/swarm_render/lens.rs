//! Sequential spectral ray tracing through a centered lens prescription.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::{f64::consts::TAU, sync::Arc};

use crate::artifact::{LatticeSamples, RadianceLattice};
use crate::primal_replay::counter_uniform;
use crate::{
    count, fields, list, num, record, required, vector, Args, AssetHost, TypedArtifact, Value,
};

const WAVELENGTHS_UM: [f64; 3] = [0.650, 0.532, 0.450];
const GEOMETRY_EPSILON_M: f64 = 1e-10;

struct Surface {
    vertex_z: f64,
    radius: f64,
    aperture: f64,
    index_after: [f64; 3],
}

struct Ray {
    origin: [f64; 3],
    direction: [f64; 3],
}

impl AssetHost<'_> {
    pub(crate) fn raytrace_multi_element_optical_lens(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(
            args,
            &[
                "image",
                "surfaces",
                "object_z_m",
                "object_height_m",
                "sensor_z_m",
                "sensor_height_m",
                "samples_per_pixel",
                "seed",
            ],
        )?;
        let TypedArtifact::RadianceLattice(image) =
            self.workspace.typed(required(args, "image")?)?
        else {
            return Err(
                "Sequential lens tracing requires a retained linear Rec.709 radiance lattice"
                    .into(),
            );
        };
        let image = Arc::clone(image);
        let (w, h) = (image.width as usize, image.height as usize);
        let channels = usize::from(image.channels);
        if !(1..=2048).contains(&w)
            || !(1..=2048).contains(&h)
            || !(3..=4).contains(&channels)
            || image.primaries_iri != "urn:qualia:color:primaries:rec709"
            || image.transfer_iri != "urn:qualia:color:transfer:linear"
        {
            return Err("Lens tracing requires 1..=2048 pixel dimensions, three or four channels and linear Rec.709".into());
        }
        let expected = w * h * channels;
        let valid = match &image.samples {
            LatticeSamples::U8(samples) => samples.len() == expected,
            LatticeSamples::U16(samples) => samples.len() == expected,
            LatticeSamples::F32(samples) => {
                samples.len() == expected
                    && samples.iter().all(|value| (0.0..=1e12).contains(value))
            }
            LatticeSamples::F16Bits(_) => false,
        };
        if !valid {
            return Err(
                "Lens tracing requires complete linear U8, U16 or finite nonnegative F32 samples"
                    .into(),
            );
        }
        let object_z = num(required(args, "object_z_m")?)?;
        let object_h = num(required(args, "object_height_m")?)?;
        let sensor_z = num(required(args, "sensor_z_m")?)?;
        let sensor_h = num(required(args, "sensor_height_m")?)?;
        if !(-1e4..=100.0).contains(&object_z)
            || !(-100.0..=100.0).contains(&sensor_z)
            || !(1e-6..=1e4).contains(&object_h)
            || !(1e-6..=1.0).contains(&sensor_h)
        {
            return Err(
                "Lens tracing requires finite object and sensor placements in metres".into(),
            );
        }
        let values = list(required(args, "surfaces")?)?;
        if !(1..=64).contains(&values.len()) {
            return Err(
                "Lens prescription requires 1..=64 surfaces ordered from object to sensor".into(),
            );
        }
        let mut surfaces = Vec::with_capacity(values.len());
        let mut previous_z = object_z;
        for value in values {
            let surface = parse_surface(record(value)?)?;
            let sag = surface.sag();
            if surface.vertex_z + sag.min(0.0) <= previous_z + GEOMETRY_EPSILON_M {
                return Err(
                    "Prescription surfaces must be separated along the optical axis".into(),
                );
            }
            previous_z = surface.vertex_z + sag.max(0.0);
            surfaces.push(surface);
        }
        if previous_z + GEOMETRY_EPSILON_M >= sensor_z {
            return Err("The sensor plane must follow the complete prescription".into());
        }
        let spp = count(args, "samples_per_pixel", 8, 64)?;
        let seed = match args.get("seed") {
            None => 0,
            Some(Value::U64(seed)) => *seed,
            Some(Value::I64(seed)) if *seed >= 0 => *seed as u64,
            _ => return Err("Lens sample seed must be an unsigned 64-bit value".into()),
        };
        let work = w
            .saturating_mul(h)
            .saturating_mul(spp)
            .saturating_mul(surfaces.len())
            .saturating_mul(120);
        self.charge(work, w * h * 3 * size_of::<f32>())?;
        let aspect = w as f64 / h as f64;
        let object_w = object_h * aspect;
        let sensor_w = sensor_h * aspect;
        let entrance = surfaces.last().expect("validated nonempty prescription");
        let mut result = Vec::with_capacity(w * h * 3);
        let mut transmitted = 0_u64;
        for y in 0..h {
            self.charge(0, 0)?;
            for x in 0..w {
                let mut color = [0.0; 3];
                for sample in 0..spp {
                    let stream = (w * y + x) as u64;
                    let counter = 4 * sample as u64;
                    let sx = ((x as f64 + counter_uniform(seed, stream, counter)) / w as f64 - 0.5)
                        * sensor_w;
                    let sy = (0.5
                        - (y as f64 + counter_uniform(seed, stream, counter + 1)) / h as f64)
                        * sensor_h;
                    let radius =
                        entrance.aperture * counter_uniform(seed, stream, counter + 2).sqrt();
                    let (s, c) = (TAU * counter_uniform(seed, stream, counter + 3)).sin_cos();
                    let direction = normalized([
                        radius * c - sx,
                        radius * s - sy,
                        entrance.vertex_z - sensor_z,
                    ]);
                    let obliquity = direction[2].powi(4);
                    for (band, lane) in color.iter_mut().enumerate() {
                        let ray = Ray {
                            origin: [sx, sy, sensor_z],
                            direction,
                        };
                        let Some((point, throughput)) = trace(ray, &surfaces, object_z, band)
                        else {
                            continue;
                        };
                        let u = 0.5 + point[0] / object_w;
                        let v = 0.5 - point[1] / object_h;
                        if let Some(radiance) = sample_image(&image, u, v, band) {
                            *lane += radiance * throughput * obliquity;
                            transmitted += 1;
                        }
                    }
                }
                result.extend(color.map(|lane| (lane / spp as f64) as f32));
            }
        }
        let mut facts = vec![
            ("object_z_m", object_z),
            ("object_height_m", object_h),
            ("sensor_z_m", sensor_z),
            ("sensor_height_m", sensor_h),
            ("samples_per_pixel", spp as f64),
            ("sensor_width_px", w as f64),
            ("sensor_height_px", h as f64),
            ("surface_count", surfaces.len() as f64),
            ("transmitted_channel_rays", transmitted as f64),
            ("seed_high", (seed >> 32) as f64),
            ("seed_low", (seed as u32) as f64),
        ];
        let mut prescription = Vec::with_capacity(surfaces.len() * 6);
        for surface in &surfaces {
            prescription.extend([surface.vertex_z, surface.radius, surface.aperture]);
            prescription.extend(surface.index_after);
        }
        // Include ordered surface parameters in the receipt identifier even when every ray is vignetted.
        let keys: Vec<_> = (0..prescription.len())
            .map(|index| format!("prescription_{index:03}"))
            .collect();
        facts.extend(keys.iter().map(String::as_str).zip(prescription));
        let receipt = super::receipt(self, "sequential-centered-lens", &result, &facts)?;
        let image =
            self.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
                width: w as u32,
                height: h as u32,
                channels: 3,
                transfer_iri: image.transfer_iri.clone(),
                primaries_iri: image.primaries_iri.clone(),
                samples: LatticeSamples::F32(result),
            })))?;
        Ok(Value::Record(Args::from([
            ("image".into(), image),
            ("receipt".into(), receipt),
            ("transmitted_channel_rays".into(), Value::U64(transmitted)),
        ])))
    }
}

fn parse_surface(args: &Args) -> Result<Surface, String> {
    fields(
        args,
        &[
            "vertex_z_m",
            "radius_m",
            "clear_radius_m",
            "index_after",
            "sellmeier_after",
        ],
    )?;
    let vertex_z = num(required(args, "vertex_z_m")?)?;
    let radius = num(required(args, "radius_m")?)?;
    let aperture = num(required(args, "clear_radius_m")?)?;
    if !(-100.0..=100.0).contains(&vertex_z)
        || !(1e-6..=1.0).contains(&aperture)
        || (radius != 0.0 && !(aperture..=100.0).contains(&radius.abs()))
    {
        return Err(
            "Lens requires a plane (radius 0) or a spherical cap with a valid clear aperture"
                .into(),
        );
    }
    let index_after = match (args.get("index_after"), args.get("sellmeier_after")) {
        (Some(index), None) => [num(index)?; 3],
        (None, Some(value)) => {
            let coefficients = record(value)?;
            fields(coefficients, &["b", "c_um2"])?;
            let b = vector::<3>(required(coefficients, "b")?)?;
            let c = vector::<3>(required(coefficients, "c_um2")?)?;
            let mut n = [0.0; 3];
            for (value, lambda) in n.iter_mut().zip(WAVELENGTHS_UM) {
                let lambda2 = lambda * lambda;
                let mut n2 = 1.0;
                for i in 0..3 {
                    let delta = lambda2 - c[i];
                    if delta.abs() < 1e-10 {
                        return Err(
                            "Sellmeier resonance intersects a sampled visible wavelength".into(),
                        );
                    }
                    n2 += b[i] * lambda2 / delta;
                }
                *value = n2.sqrt();
            }
            n
        }
        _ => {
            return Err(
                "Each lens surface needs exactly one index_after or sellmeier_after medium".into(),
            )
        }
    };
    if index_after.iter().any(|value| !(1.0..=4.0).contains(value)) {
        return Err("Visible-band lens refractive indices must lie in [1,4]".into());
    }
    Ok(Surface {
        vertex_z,
        radius,
        aperture,
        index_after,
    })
}

impl Surface {
    fn sag(&self) -> f64 {
        if self.radius == 0.0 {
            0.0
        } else {
            let absolute = self.radius.abs();
            let depth = self.aperture.powi(2)
                / (absolute + (absolute.powi(2) - self.aperture.powi(2)).sqrt());
            depth.copysign(self.radius)
        }
    }

    fn hit(&self, ray: &Ray) -> Option<([f64; 3], [f64; 3])> {
        if self.radius == 0.0 {
            let t = (self.vertex_z - ray.origin[2]) / ray.direction[2];
            let point = point_at(ray, t);
            return (t > GEOMETRY_EPSILON_M && inside_stop(point, self.aperture))
                .then_some((point, [0.0, 0.0, 1.0]));
        }
        let center_z = self.vertex_z + self.radius;
        let relative = [ray.origin[0], ray.origin[1], ray.origin[2] - center_z];
        let projection = dot(relative, ray.direction);
        let closest = std::array::from_fn(|axis| relative[axis] - projection * ray.direction[axis]);
        let chord2 = self.radius.powi(2) - dot(closest, closest);
        if chord2 < 0.0 {
            return None;
        }
        let chord = chord2.sqrt();
        for t in [-projection - chord, -projection + chord] {
            if t <= GEOMETRY_EPSILON_M {
                continue;
            }
            let point = point_at(ray, t);
            let radial_z = point[2] - center_z;
            if radial_z * self.radius <= 0.0 && inside_stop(point, self.aperture) {
                let normal = normalized([point[0], point[1], radial_z]);
                return Some((point, normal));
            }
        }
        None
    }
}

fn trace(
    mut ray: Ray,
    surfaces: &[Surface],
    object_z: f64,
    band: usize,
) -> Option<([f64; 3], f64)> {
    let mut throughput = 1.0;
    for i in (0..surfaces.len()).rev() {
        let surface = &surfaces[i];
        let (point, mut normal) = surface.hit(&ray)?;
        let n1 = surface.index_after[band];
        let n2 = i
            .checked_sub(1)
            .map_or(1.0, |i| surfaces[i].index_after[band]);
        let incident = dot(ray.direction, normal);
        if incident > 0.0 {
            normal = normal.map(|x| -x);
        }
        let cos_i = incident.abs().min(1.0);
        let eta = n1 / n2;
        let cos_t2 = 1.0 - eta * eta * (1.0 - cos_i * cos_i);
        if cos_t2 <= 0.0 {
            return None;
        }
        let cos_t = cos_t2.sqrt();
        let rs = (n1 * cos_i - n2 * cos_t) / (n1 * cos_i + n2 * cos_t);
        let rp = (n2 * cos_i - n1 * cos_t) / (n2 * cos_i + n1 * cos_t);
        throughput *= (1.0 - 0.5 * (rs * rs + rp * rp)).clamp(0.0, 1.0);
        let scale = eta * cos_i - cos_t;
        let direction = normalized(std::array::from_fn(|axis| {
            eta * ray.direction[axis] + scale * normal[axis]
        }));
        if direction[2] >= -1e-10 {
            return None;
        }
        ray = Ray {
            origin: point,
            direction,
        };
    }
    let t = (object_z - ray.origin[2]) / ray.direction[2];
    (t > 0.0).then(|| (point_at(&ray, t), throughput))
}

fn sample_image(image: &RadianceLattice, u: f64, v: f64, band: usize) -> Option<f64> {
    if !(0.0..=1.0).contains(&u) || !(0.0..=1.0).contains(&v) {
        return None;
    }
    let (w, h) = (image.width as usize, image.height as usize);
    let x = (u * w as f64 - 0.5).clamp(0.0, (w - 1) as f64);
    let y = (v * h as f64 - 0.5).clamp(0.0, (h - 1) as f64);
    let (x0, y0) = (x as usize, y as usize);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let value = |x, y| {
        let index = (y * w + x) * usize::from(image.channels) + band;
        match &image.samples {
            LatticeSamples::U8(samples) => f64::from(samples[index]) / f64::from(u8::MAX),
            LatticeSamples::U16(samples) => f64::from(samples[index]) / f64::from(u16::MAX),
            LatticeSamples::F32(samples) => f64::from(samples[index]),
            LatticeSamples::F16Bits(_) => unreachable!("image admission requires U8, U16 or F32"),
        }
    };
    let mix = |a, b, f| a * (1.0 - f) + b * f;
    let top = mix(value(x0, y0), value(x1, y0), x.fract());
    let bottom = mix(value(x0, y1), value(x1, y1), x.fract());
    Some(mix(top, bottom, y.fract()))
}

fn point_at(ray: &Ray, t: f64) -> [f64; 3] {
    std::array::from_fn(|axis| ray.origin[axis] + t * ray.direction[axis])
}

fn inside_stop(point: [f64; 3], aperture: f64) -> bool {
    point[0].powi(2) + point[1].powi(2) <= aperture.powi(2)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn normalized(v: [f64; 3]) -> [f64; 3] {
    let length = dot(v, v).sqrt();
    v.map(|component| component / length)
}

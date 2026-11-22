//! Schwarzschild null-ray integration for a prescribed thin emitting disk.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{count, dimensions, numeric_list, scalar};
use crate::{artifact::FieldLattice, fields, list, num, Args, AssetHost, TypedArtifact, Value};
use std::{collections::BTreeMap, f64::consts::PI, sync::Arc};

const MAX_BANDS: usize = 16;
const HORIZON_U: f64 = 1.0 - 1e-7;
const MAX_AZIMUTH: f64 = 16.0 * PI;

struct Disk {
    inner_rs: f64,
    outer_rs: f64,
    temperature_scale_k: f64,
}

struct Observer {
    radius_rs: f64,
    direction: [f64; 3],
    up: [f64; 3],
    angular_step: f64,
    step_limit: usize,
}

struct DiskHit {
    radius_rs: f64,
    frequency_ratio: f64,
}

enum Trace {
    Disk(DiskHit),
    Captured,
    Escaped,
    Truncated,
}

impl AssetHost<'_> {
    pub(crate) fn raymarch_relativistic_disk(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "schwarzschild_radius_m",
                "disk_inner_radius_rs",
                "disk_outer_radius_rs",
                "temperature_scale_k",
                "observer_radius_rs",
                "inclination_degrees",
                "image_size",
                "image_half_height_rs",
                "spectral_bands_nm",
                "max_angular_step_rad",
                "max_steps_per_ray",
            ],
        )?;
        let rs = scalar(args, "schwarzschild_radius_m", None, 1e-6, 1e15)?;
        // Stable circular emitters in Schwarzschild space-time require r >= 3 r_s.
        let inner_rs = scalar(args, "disk_inner_radius_rs", Some(3.0), 3.0, 9999.0)?;
        let outer_rs = scalar(args, "disk_outer_radius_rs", None, inner_rs, 10_000.0)?;
        if outer_rs - inner_rs < 1e-6 {
            return Err("An accretion disk needs a positive radial extent".into());
        }
        let disk = Disk {
            inner_rs,
            outer_rs,
            temperature_scale_k: scalar(args, "temperature_scale_k", None, 1.0, 1e9)?,
        };
        let observer_radius = scalar(
            args,
            "observer_radius_rs",
            Some((10.0 * outer_rs).max(100.0)),
            (2.0 * outer_rs).max(20.0),
            1e6,
        )?;
        let inclination = scalar(args, "inclination_degrees", Some(60.0), 0.0, 89.0)?.to_radians();
        let (sin_i, cos_i) = inclination.sin_cos();
        let observer = Observer {
            radius_rs: observer_radius,
            direction: [sin_i, cos_i, 0.0],
            up: [-cos_i, sin_i, 0.0],
            angular_step: scalar(args, "max_angular_step_rad", Some(0.025), 0.001, 0.05)?,
            step_limit: count(args, "max_steps_per_ray", 2048, 128, 8192)?,
        };
        let [width, height] = dimensions(args, "image_size", [128, 128], 512)?;
        let half_height = scalar(
            args,
            "image_half_height_rs",
            Some(1.15 * outer_rs),
            0.01,
            observer_radius * 0.7,
        )?;
        let bands = if let Some(value) = args.get("spectral_bands_nm") {
            let samples = list(value)?;
            if samples.is_empty() || samples.len() > MAX_BANDS {
                return Err("Select 1 to 16 monochromatic wavelength samples".into());
            }
            samples.iter().map(num).collect::<Result<Vec<_>, _>>()?
        } else {
            vec![450.0, 550.0, 650.0]
        };
        if bands
            .iter()
            .any(|wavelength| !(100.0..=20_000.0).contains(wavelength))
            || bands.windows(2).any(|pair| pair[0] >= pair[1])
        {
            return Err("Spectral samples must be strictly increasing in [100, 20000] nm".into());
        }
        let pixels = width * height;
        self.charge(pixels * 48, pixels * (bands.len() + 3) * 4)?;
        let mut spectral = vec![0.0; pixels * bands.len()];
        let mut geometry = vec![0.0; pixels * 3];
        let mut counts = [0usize; 4];
        for y in 0..height {
            let vertical = half_height * (1.0 - 2.0 * (y as f64 + 0.5) / height as f64);
            for x in 0..width {
                let horizontal = half_height
                    * (2.0 * (x as f64 + 0.5) / width as f64 - 1.0)
                    * (width as f64 / height as f64);
                let (result, steps) = observer.trace(horizontal, vertical, &disk)?;
                self.charge(steps * 64, 0)?;
                let pixel = y * width + x;
                let code = match result {
                    Trace::Disk(hit) => {
                        geometry[pixel] = (hit.radius_rs * rs) as f32;
                        geometry[pixels + pixel] = hit.frequency_ratio as f32;
                        let temperature = disk.temperature_scale_k
                            * (disk.inner_rs / hit.radius_rs).powf(0.75)
                            * (1.0 - (disk.inner_rs / hit.radius_rs).sqrt())
                                .max(0.0)
                                .powf(0.25);
                        for (band, &nm) in bands.iter().enumerate() {
                            let radiance = planck_per_nm(nm, temperature * hit.frequency_ratio);
                            if !radiance.is_finite() || radiance > f64::from(f32::MAX) {
                                return Err("Disk ray produced invalid spectral radiance".into());
                            }
                            spectral[band * pixels + pixel] = radiance as f32;
                        }
                        1
                    }
                    Trace::Escaped => 0,
                    Trace::Captured => 2,
                    Trace::Truncated => 3,
                };
                counts[code] += 1;
                geometry[2 * pixels + pixel] = code as f32;
            }
        }
        let static_disk_axes = vec![bands.len() as u32, height as u32, width as u32];
        let radiance = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: static_disk_axes,
                samples: spectral,
            },
        )))?;
        let geometry = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![3, height as u32, width as u32],
                samples: geometry,
            },
        )))?;
        Ok(Value::Record(BTreeMap::from([
            ("spectral_radiance".into(), radiance),
            ("spectral_bands_nm".into(), numeric_list(bands)),
            (
                "radiance_units".into(),
                Value::String("W·m⁻²·sr⁻¹·nm⁻¹".into()),
            ),
            ("ray_geometry".into(), geometry),
            (
                "geometry_channels".into(),
                super::names(["emission_radius_m", "frequency_ratio", "ray_status"]),
            ),
            (
                "status_codes".into(),
                super::names(["escaped", "disk", "captured", "truncated"]),
            ),
            (
                "ray_counts".into(),
                Value::List(counts.map(|n| Value::U64(n as u64)).to_vec().into()),
            ),
            (
                "model".into(),
                Value::String("schwarzschild_null_geodesics_prescribed_thin_disk".into()),
            ),
        ])))
    }
}

impl Observer {
    fn trace(&self, horizontal: f64, vertical: f64, disk: &Disk) -> Result<(Trace, usize), String> {
        let screen_radius = horizontal.hypot(vertical);
        if screen_radius <= 1e-10 {
            return Ok((Trace::Captured, 0));
        }
        let plane = [
            self.up[0] * vertical / screen_radius,
            self.up[1] * vertical / screen_radius,
            horizontal / screen_radius,
        ];
        let start_u = self.radius_rs.recip();
        let angle = screen_radius.atan2(self.radius_rs);
        let impact = self.radius_rs * angle.sin() / (1.0 - start_u).sqrt();
        let mut u = start_u;
        let mut derivative = angle.cos() / impact;
        let mut azimuth: f64 = 0.0;
        let mut crossing = (-self.direction[1]).atan2(plane[1]).rem_euclid(PI);
        for step in 1..=self.step_limit {
            let angle_bound = self.angular_step.min(0.025 / derivative.abs().max(0.1));
            let h = angle_bound
                .min(crossing - azimuth)
                .min(MAX_AZIMUTH - azimuth);
            if h <= 0.0 {
                return Ok((Trace::Truncated, step - 1));
            }
            (u, derivative) = null_geodesic_step(u, derivative, h);
            azimuth += h;
            if !u.is_finite() || !derivative.is_finite() {
                return Err("Schwarzschild null ray diverged from the finite solver domain".into());
            }
            if u >= HORIZON_U {
                return Ok((Trace::Captured, step));
            }
            if u <= start_u && derivative < 0.0 {
                return Ok((Trace::Escaped, step));
            }
            if crossing - azimuth <= 1e-12 {
                let radius = u.recip();
                if (disk.inner_rs..=disk.outer_rs).contains(&radius) {
                    let frequency_ratio = self.frequency_ratio(u, plane, crossing, impact);
                    if !frequency_ratio.is_finite() || frequency_ratio <= 0.0 {
                        return Err("Disk ray produced an invalid frequency ratio".into());
                    }
                    return Ok((
                        Trace::Disk(DiskHit {
                            radius_rs: radius,
                            frequency_ratio,
                        }),
                        step,
                    ));
                }
                crossing += PI;
            }
            if azimuth >= MAX_AZIMUTH - 1e-12 {
                return Ok((Trace::Truncated, step));
            }
        }
        Ok((Trace::Truncated, self.step_limit))
    }

    fn frequency_ratio(&self, u: f64, plane: [f64; 3], azimuth: f64, impact: f64) -> f64 {
        let (sin, cos) = azimuth.sin_cos();
        let radial: [f64; 3] = std::array::from_fn(|i| self.direction[i] * cos + plane[i] * sin);
        let tangent: [f64; 3] = std::array::from_fn(|i| -self.direction[i] * sin + plane[i] * cos);
        let emitter_direction = [radial[2], 0.0, -radial[0]];
        let photon_tangential = -(impact * u * (1.0 - u).sqrt()).clamp(0.0, 1.0);
        let along_velocity = tangent
            .into_iter()
            .zip(emitter_direction)
            .map(|(a, b)| a * b)
            .sum::<f64>()
            * photon_tangential;
        let beta = (u / (2.0 * (1.0 - u))).sqrt();
        let gravitational = ((1.0 - u) / (1.0 - self.radius_rs.recip())).sqrt();
        gravitational * (1.0 - beta * beta).sqrt() / (1.0 - beta * along_velocity)
    }
}

// Binet's equation for a Schwarzschild photon orbit with u = r_s/r.
fn null_geodesic_step(u: f64, v: f64, h: f64) -> (f64, f64) {
    let acceleration = |u: f64| 1.5 * u * u - u;
    let k1 = [v, acceleration(u)];
    let k2 = [v + 0.5 * h * k1[1], acceleration(u + 0.5 * h * k1[0])];
    let k3 = [v + 0.5 * h * k2[1], acceleration(u + 0.5 * h * k2[0])];
    let k4 = [v + h * k3[1], acceleration(u + h * k3[0])];
    (
        u + (h / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
        v + (h / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
    )
}

fn planck_per_nm(nm: f64, temperature: f64) -> f64 {
    if temperature <= 0.0 {
        return 0.0;
    }
    const H: f64 = 6.626_070_15e-34;
    const C: f64 = 299_792_458.0;
    const K: f64 = 1.380_649e-23;
    let wavelength = nm * 1e-9;
    let exponent = H * C / (wavelength * K * temperature);
    (2.0 * H * C * C * 1e-9) / (wavelength.powi(5) * exponent.exp_m1())
}

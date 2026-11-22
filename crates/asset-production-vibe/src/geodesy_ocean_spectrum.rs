//! Directional finite-depth ocean spectra and periodic surface reconstruction.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::artifact::{AttributedMesh, FieldLattice};
use crate::primal_replay::counter_uniform;
use crate::*;
use qualia_core_db::render::assets::{ImportedMesh, Mesh};
use qualia_core_db::solvers::transforms::spectral_fft::fft_2d;
use std::f64::consts::{PI, TAU};
use std::sync::Arc;

const G: f64 = 9.80665;
const CHANNELS: [&str; 8] = [
    "height_m",
    "displacement_x_m",
    "displacement_z_m",
    "normal_x",
    "normal_y",
    "normal_z",
    "surface_jacobian",
    "whitecap_mask",
];

#[derive(Clone, Copy)]
enum Spectrum {
    PiersonMoskowitz,
    Jonswap,
    Tma,
}

struct Sea {
    spectrum: Spectrum,
    wind_unit: [f64; 2],
    depth: f64,
    peak: f64,
    alpha: f64,
}

impl Sea {
    fn bin(&self, wave: [f64; 2], dk: f64) -> (f64, f64) {
        let k = wave[0].hypot(wave[1]);
        if k == 0.0 {
            return (0.0, 0.0);
        }
        let depth = k * self.depth;
        let t = depth.tanh();
        let omega = (G * k * t).sqrt();
        let omega_prime = G * (t + depth * (1.0 - t * t)) / (2.0 * omega);
        let alignment = ((wave[0] * self.wind_unit[0] + wave[1] * self.wind_unit[1]) / k).max(0.0);
        let direction = 2.0 / PI * alignment * alignment;
        let mut density =
            self.alpha * G * G / omega.powi(5) * (-1.25 * (self.peak / omega).powi(4)).exp();
        if matches!(self.spectrum, Spectrum::Jonswap | Spectrum::Tma) {
            let sigma = if omega <= self.peak { 0.07 } else { 0.09 };
            let peak_shape = (-0.5 * ((omega - self.peak) / (sigma * self.peak)).powi(2)).exp();
            density *= 3.3_f64.powf(peak_shape);
        }
        if matches!(self.spectrum, Spectrum::Tma) {
            let nondimensional = omega * (self.depth / G).sqrt();
            density *= match nondimensional {
                x if x < 1.0 => 0.5 * x * x,
                x if x < 2.0 => 1.0 - 0.5 * (2.0 - x).powi(2),
                _ => 1.0,
            };
        }
        (omega, direction * density * omega_prime / k * dk * dk)
    }
}

impl AssetHost<'_> {
    pub(crate) fn synthesize_ocean_spectrum(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "spectrum",
                "wind_xz_m_s",
                "depth_m",
                "fetch_km",
                "grid_res",
                "period_m",
                "time_s",
                "seed",
                "choppiness",
                "whitecap_jacobian",
            ],
        )?;
        let model = text(required(args, "spectrum")?)?;
        let [x, z] = vector::<2>(required(args, "wind_xz_m_s")?)?;
        let wind = x.hypot(z);
        if !(0.5..=60.0).contains(&wind) {
            return Err("10 m wind speed must be within 0.5 to 60 m/s".into());
        }
        let (spectrum, peak, alpha) =
            match model {
                "pierson_moskowitz" if !args.contains_key("fetch_km") => {
                    (Spectrum::PiersonMoskowitz, 0.877 * G / wind, 0.0081)
                }
                "jonswap" | "tma" => {
                    let fetch = bound(args, "fetch_km", None, 0.05, 10_000.0)? * 1000.0;
                    let dimensionless_fetch = G * fetch / wind.powi(2);
                    let spectrum = if model == "jonswap" {
                        Spectrum::Jonswap
                    } else {
                        Spectrum::Tma
                    };
                    (
                        spectrum,
                        22.0 * (G * G / (wind * fetch)).cbrt(),
                        0.076 * dimensionless_fetch.powf(-0.22),
                    )
                }
                _ => return Err(
                    "Select pierson_moskowitz without fetch_km, or jonswap or tma with fetch_km"
                        .into(),
                ),
            };
        let sea = Sea {
            spectrum,
            wind_unit: [x / wind, z / wind],
            peak,
            alpha,
            depth: bound(args, "depth_m", None, 0.05, 12_000.0)?,
        };
        let n = integer(required(args, "grid_res")?, "grid_res")?;
        if !(16..=256).contains(&n) || !n.is_power_of_two() {
            return Err(
                "Ocean spectrum grid resolution must be a power of two within 16 to 256".into(),
            );
        }
        let period = bound(args, "period_m", None, 1.0, 100_000.0)?;
        let time = bound(args, "time_s", None, -1e8, 1e8)?;
        let choppy = bound(args, "choppiness", Some(1.0), 0.0, 2.0)?;
        let foam = bound(args, "whitecap_jacobian", Some(0.7), 0.05, 0.999)?;
        let seed = match required(args, "seed")? {
            Value::U64(seed) => *seed,
            Value::I64(seed) => {
                u64::try_from(*seed).map_err(|_| "Ocean spectrum requires an unsigned seed")?
            }
            _ => return Err("Ocean spectrum requires an unsigned seed".into()),
        };
        let bins = n * n;
        self.charge(bins * n.ilog2() as usize * 320, bins * 420)?;
        let dk = TAU / period;
        let wave = |index: usize| {
            [index % n, index / n]
                .map(|axis| axis as f64 * dk - if axis >= n / 2 { n as f64 * dk } else { 0.0 })
        };
        let mut initial = Vec::with_capacity(bins);
        let mut frequencies = Vec::with_capacity(bins);
        for index in 0..bins {
            let [kx, kz] = wave(index);
            let (omega, variance) = sea.bin([kx, kz], dk);
            let deviation = if index % n == n / 2 || index / n == n / 2 {
                0.0
            } else {
                (variance * 0.25).sqrt()
            };
            let (sine, cosine) = (TAU * counter_uniform(seed, index as u64, 0)).sin_cos();
            let radius = deviation * (-2.0 * counter_uniform(seed, index as u64, 1).ln()).sqrt();
            initial.push((radius * cosine, radius * sine));
            frequencies.push(omega);
        }
        let harmonics = initial
            .iter()
            .zip(&frequencies)
            .enumerate()
            .map(|(index, (&here, &omega))| {
                let opposite = ((n - index / n) % n) * n + (n - index % n) % n;
                let (s, c) = (-omega * time).sin_cos();
                let a = rotate(here, c, s);
                let b = rotate(initial[opposite], c, s);
                (a.0 + b.0, a.1 - b.1)
            })
            .collect::<Vec<_>>();
        let transform = |factor: &dyn Fn(f64, f64, f64) -> (f64, f64)| {
            let coeffs = harmonics
                .iter()
                .enumerate()
                .map(|(index, &sample)| {
                    let [kx, kz] = wave(index);
                    let (re, im) = factor(kx, kz, kx.hypot(kz).max(f64::MIN_POSITIVE));
                    rotate(sample, re, im)
                })
                .collect::<Vec<_>>();
            fft_2d(&coeffs, n, n, true)
                .map_err(|err| err.to_string())
                .map(|spatial| {
                    spatial
                        .into_iter()
                        .map(|(re, _)| re * bins as f64)
                        .collect::<Vec<_>>()
                })
        };
        let height = transform(&|_, _, _| (1.0, 0.0))?;
        let dx = transform(&|kx, _, k| (0.0, choppy * kx / k))?;
        let dz = transform(&|_, kz, k| (0.0, choppy * kz / k))?;
        let slope_x = transform(&|kx, _, _| (0.0, kx))?;
        let slope_z = transform(&|_, kz, _| (0.0, kz))?;
        let xx = transform(&|kx, _, k| (-choppy * kx * kx / k, 0.0))?;
        let xz = transform(&|kx, kz, k| (-choppy * kx * kz / k, 0.0))?;
        let zz = transform(&|_, kz, k| (-choppy * kz * kz / k, 0.0))?;

        let mut channels = vec![0.0_f32; CHANNELS.len() * bins];
        for i in 0..bins {
            let a = 1.0 + xx[i];
            let b = xz[i];
            let d = 1.0 + zz[i];
            let jacobian = a * d - b * b;
            if jacobian <= 0.0 || a <= 0.0 || !jacobian.is_finite() {
                return Err("Ocean spectrum folds at the selected choppiness".into());
            }
            let nx = b * slope_z[i] - d * slope_x[i];
            let nz = b * slope_x[i] - a * slope_z[i];
            let norm = nx.hypot(jacobian).hypot(nz);
            let data = [
                height[i],
                dx[i],
                dz[i],
                nx / norm,
                jacobian / norm,
                nz / norm,
                jacobian,
                u8::from(jacobian < foam).into(),
            ];
            for (channel, value) in data.into_iter().enumerate() {
                let value = value as f32;
                if !value.is_finite() {
                    return Err("Ocean spectrum produced non-finite samples".into());
                }
                channels[channel * bins + i] = value;
            }
        }
        let delta = period / n as f64;
        let mesh = self.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(grid_mesh(
            n, delta, &channels,
        ))))?;
        let fields = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![CHANNELS.len() as u32, n as u32, n as u32],
                samples: channels,
            },
        )))?;
        let variance = height.iter().map(|sample| sample * sample).sum::<f64>() / bins as f64;
        Ok(Value::Record(Args::from([
            ("mesh".into(), mesh),
            ("fields".into(), fields),
            (
                "channel_names".into(),
                Value::List(CHANNELS.map(|name| Value::String(name.into())).into()),
            ),
            ("cell_size_m".into(), Value::F64(delta)),
            (
                "significant_height_estimate_m".into(),
                Value::F64(4.0 * variance.sqrt()),
            ),
        ])))
    }
}

fn rotate((re, im): (f64, f64), c: f64, s: f64) -> (f64, f64) {
    (re * c - im * s, re * s + im * c)
}

fn bound(args: &Args, key: &str, default: Option<f64>, min: f64, max: f64) -> Result<f64, String> {
    let value = args
        .get(key)
        .map(num)
        .transpose()?
        .or(default)
        .ok_or_else(|| format!("Missing {key}"))?;
    if !(min..=max).contains(&value) {
        return Err(format!("{key} must be within {min} to {max}"));
    }
    Ok(value)
}

fn grid_mesh(n: usize, delta: f64, data: &[f32]) -> AttributedMesh {
    let stride = n + 1;
    let bins = n * n;
    let mut positions = Vec::with_capacity(stride * stride);
    let mut normals = Vec::with_capacity(stride * stride);
    let mut uv = Vec::with_capacity(stride * stride);
    let mut triangles = Vec::with_capacity(2 * bins);
    let (mut min, mut max) = ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]);
    for z in 0..=n {
        for x in 0..=n {
            let i = (z % n) * n + x % n;
            let p = [
                (x as f64 * delta + f64::from(data[bins + i])) as f32,
                data[i],
                (z as f64 * delta + f64::from(data[2 * bins + i])) as f32,
            ];
            for axis in 0..3 {
                min[axis] = min[axis].min(p[axis]);
                max[axis] = max[axis].max(p[axis]);
            }
            positions.push(p);
            normals.push([data[3 * bins + i], data[4 * bins + i], data[5 * bins + i]]);
            uv.push([x as f32 / n as f32, z as f32 / n as f32]);
            if x < n && z < n {
                let a = (z * stride + x) as u32;
                let c = a + stride as u32;
                triangles.extend([[a, c, a + 1], [a + 1, c, c + 1]]);
            }
        }
    }
    AttributedMesh {
        surface: ImportedMesh {
            mesh: Mesh {
                positions,
                triangles,
                min,
                max,
            },
            normals: Some(normals),
            tangents: None,
            texture_coordinates_0: Some(uv),
            materials: Vec::new(),
            texture_dependencies: Vec::new(),
            submeshes: Vec::new(),
        },
        uv1: Vec::new(),
        vertex_colors: Vec::new(),
        skin_weights: Vec::new(),
        morph_targets: Arc::from(Vec::new()),
        protected_faces: Vec::new(),
    }
}

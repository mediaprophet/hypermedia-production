//! Bounded planetary and astronomical asset-production adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{
    artifact::{FieldLattice, LatticeSamples, RadianceLattice},
    fields, integer, list, num, record, required, vector, Args, AssetHost, TypedArtifact, Value,
};
use qualia_core_db::etalon_render::geo_viewport::sky_limb::{
    evaluate_sky_limb_and_aerial_perspective, AtmosphereParams,
};
use std::{collections::BTreeMap, sync::Arc};

mod dielectric;
mod orbit;
mod pic;
mod planet;
mod schwarzschild;
mod tdgl;

const MAX_IMAGE_AXIS: usize = 512;
const MAX_ATMOSPHERE_STEPS: usize = 128;

impl AssetHost<'_> {
    pub(crate) fn render_planetary_limb_sky(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "atmosphere",
                "sun_direction",
                "camera_altitude_m",
                "view_direction",
                "image_size",
                "vertical_fov_degrees",
                "integration_steps",
            ],
        )?;
        let atmosphere = record(required(args, "atmosphere")?)?;
        fields(
            atmosphere,
            &[
                "planet_radius_m",
                "atmosphere_top_m",
                "rayleigh_scale_height_m",
                "mie_scale_height_m",
                "rayleigh_scattering_rgb_per_m",
                "mie_scattering_rgb_per_m",
                "mie_g",
                "sun_intensity",
            ],
        )?;
        let radius = scalar(atmosphere, "planet_radius_m", None, 1.0, 1e9)? as f32;
        let top = scalar(atmosphere, "atmosphere_top_m", None, 1.0, 1e8)? as f32;
        let altitude = scalar(args, "camera_altitude_m", None, 0.0, 1e9)? as f32;
        let rayleigh = nonnegative_rgb(atmosphere, "rayleigh_scattering_rgb_per_m")?;
        let mie = nonnegative_rgb(atmosphere, "mie_scattering_rgb_per_m")?;
        if radius + top <= radius {
            return Err("Atmosphere top collapses at the native integrator's f32 precision".into());
        }
        let sun_dir = unit(
            vector::<3>(required(args, "sun_direction")?)?,
            "sun_direction",
        )?;
        let params = AtmosphereParams {
            planet_radius_m: radius,
            atmosphere_top_m: top,
            rayleigh_scale_h_m: scalar(atmosphere, "rayleigh_scale_height_m", None, 1.0, 1e8)?
                as f32,
            mie_scale_h_m: scalar(atmosphere, "mie_scale_height_m", None, 1.0, 1e8)? as f32,
            beta_rayleigh: rayleigh,
            beta_mie: mie,
            mie_g: scalar(atmosphere, "mie_g", Some(0.0), -0.98, 0.98)? as f32,
            sun_intensity: scalar(atmosphere, "sun_intensity", Some(1.0), 0.0, 1e6)? as f32,
            sun_dir: sun_dir.map(|lane| lane as f32),
            ozone_peak_h_m: 0.0,
        };
        let [width, height] = dimensions(args, "image_size", [256, 128], MAX_IMAGE_AXIS)?;
        let steps = count(args, "integration_steps", 32, 4, MAX_ATMOSPHERE_STEPS)?;
        let vertical_fov = scalar(args, "vertical_fov_degrees", Some(40.0), 0.1, 150.0)?;
        let forward = unit(
            args.get("view_direction")
                .map(vector::<3>)
                .transpose()?
                .unwrap_or([1.0, 0.0, 0.0]),
            "view_direction",
        )?;
        let up = if forward[1].abs() < 0.99 {
            [0.0, 1.0, 0.0]
        } else {
            [0.0, 0.0, 1.0]
        };
        let right = unit(cross(forward, up), "view direction basis")?;
        let up = cross(right, forward);
        let span = (0.5 * vertical_fov.to_radians()).tan();
        let aspect = width as f64 / height as f64;
        let n = width * height;
        self.charge(n * steps * 128, n * 32)?;
        let mut radiance = Vec::with_capacity(n * 4);
        let mut transmittance = Vec::with_capacity(n * 4);
        let max_dist = 2.0 * (radius + top);
        for y in 0..height {
            self.charge(0, 0)?;
            let v = (1.0 - 2.0 * (y as f64 + 0.5) / height as f64) * span;
            for x in 0..width {
                let u = (2.0 * (x as f64 + 0.5) / width as f64 - 1.0) * span * aspect;
                let view: [f64; 3] = std::array::from_fn(|i| forward[i] + u * right[i] + v * up[i]);
                // The native ray solver clips positive intersections. An observer on the
                // surface looking into the ground instead has an opaque ray at t = 0.
                let (light, transmission) = if radius + altitude == radius && view[1] < 0.0 {
                    ([0.0; 3], [0.0; 3])
                } else {
                    evaluate_sky_limb_and_aerial_perspective(
                        &params,
                        altitude,
                        view.map(|lane| lane as f32),
                        max_dist,
                        steps as u32,
                    )
                };
                if light.iter().any(|v| !v.is_finite() || *v < 0.0)
                    || transmission.iter().any(|v| !(0.0..=1.0).contains(v))
                {
                    return Err("Planetary limb solver produced invalid atmospheric samples".into());
                }
                rgba_sample(&mut radiance, light);
                rgba_sample(&mut transmittance, transmission);
            }
        }
        let radiance = rgba_artifact(self, width, height, radiance)?;
        let transmittance = rgba_artifact(self, width, height, transmittance)?;
        Ok(Value::Record(BTreeMap::from([
            ("radiance".into(), radiance),
            ("transmittance".into(), transmittance),
            (
                "model".into(),
                Value::String("spherical_single_scattering_rgb".into()),
            ),
        ])))
    }

    pub(crate) fn step_space_weather_aurora(&mut self, args: &Args) -> Result<Value, String> {
        use qualia_core_db::vibe_host::invoke::space_weather_bridge::{
            evaluate_space_weather_zero_heap, SpaceWeatherInput,
        };
        fields(
            args,
            &[
                "imf_bz_nt",
                "solar_wind_speed_km_s",
                "proton_density_cm3",
                "f107_flux_sfu",
                "subsolar_zenith_degrees",
                "cycle_month",
                "dipole_tilt_degrees",
                "grid_size",
            ],
        )?;
        let input = SpaceWeatherInput {
            imf_bz_nt: scalar(args, "imf_bz_nt", None, -250.0, 250.0)?,
            solar_wind_speed_km_s: scalar(args, "solar_wind_speed_km_s", None, 100.0, 3500.0)?,
            proton_density_cm3: scalar(args, "proton_density_cm3", None, 0.0, 1000.0)?,
            f107_flux_sfu: scalar(args, "f107_flux_sfu", None, 40.0, 1000.0)?,
            subsolar_zenith_deg: scalar(args, "subsolar_zenith_degrees", Some(0.0), 0.0, 180.0)?,
            cycle_month: scalar(args, "cycle_month", Some(0.0), 0.0, 180.0)?,
        };
        let tilt = scalar(args, "dipole_tilt_degrees", Some(0.0), -90.0, 90.0)?.to_radians();
        let [dx, dy, dz] = dimensions(args, "grid_size", [72, 37, 24], 128)?;
        let n = dx * dy * dz;
        if n > 262_144 {
            return Err("Auroral model grid exceeds 262144 voxels".into());
        }
        self.charge(n * 960, n * 12)?;
        let weather = evaluate_space_weather_zero_heap(&input).map_err(|err| err.to_string())?;
        const LINE_CENTERS_KM: [f64; 3] = [110.0, 240.0, 100.0];
        const LINE_SIGMA_KM: [f64; 3] = [22.0, 52.0, 16.0];
        const LINE_RELATIVE_AMPLITUDES: [f64; 3] = [1.0, 0.42, 0.25];
        const ALT_MIN_KM: f64 = 75.0;
        const ALT_MAX_KM: f64 = 400.0;
        let mut samples = vec![0.0; n * 3];
        for y in 0..dy {
            self.charge(0, 0)?;
            let latitude = (-90.0 + 180.0 * (y as f64 + 0.5) / dy as f64).to_radians();
            let (lat_sin, lat_cos) = latitude.sin_cos();
            for x in 0..dx {
                let longitude = std::f64::consts::TAU * (x as f64 + 0.5) / dx as f64;
                let dipole_sine = (lat_sin * tilt.cos() + lat_cos * longitude.cos() * tilt.sin())
                    .clamp(-1.0, 1.0);
                // Sample a smooth curtain two degrees poleward of the fitted equatorward
                // boundary. Intensity is a relative visualization weight, without calibration.
                let oval_center = weather.auroral_oval_equatorward_lat_deg + 2.0;
                let oval = gaussian(dipole_sine.asin().to_degrees().abs() - oval_center, 2.5);
                let night = 0.25 + 0.75 * (0.5 - 0.5 * longitude.cos());
                let intensity = oval * night * (weather.kp_index / 9.0);
                for z in 0..dz {
                    let altitude =
                        ALT_MIN_KM + (ALT_MAX_KM - ALT_MIN_KM) * (z as f64 + 0.5) / dz as f64;
                    let voxel = (z * dy + y) * dx + x;
                    for line in 0..3 {
                        samples[line * n + voxel] = (intensity
                            * LINE_RELATIVE_AMPLITUDES[line]
                            * gaussian(altitude - LINE_CENTERS_KM[line], LINE_SIGMA_KM[line]))
                            as f32;
                    }
                }
            }
        }
        let aurora = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![3, dz as u32, dy as u32, dx as u32],
                samples,
            },
        )))?;
        let output = Value::Record(BTreeMap::from([
            (
                "magnetopause_standoff_earth_radii".into(),
                Value::F64(weather.magnetopause_standoff_re),
            ),
            (
                "bow_shock_earth_radii".into(),
                Value::F64(weather.bow_shock_re),
            ),
            ("dst_index_nt".into(), Value::F64(weather.dst_index_nt)),
            ("kp_index".into(), Value::F64(weather.kp_index)),
            ("tec_tecu".into(), Value::F64(weather.tec_tecu)),
            (
                "auroral_oval_equatorward_lat_deg".into(),
                Value::F64(weather.auroral_oval_equatorward_lat_deg),
            ),
            (
                "geomagnetic_storm_g".into(),
                Value::U64(u64::from(weather.geomag_storm_scale_g)),
            ),
            (
                "sunspot_number".into(),
                Value::F64(weather.sunspot_number_ssn),
            ),
        ]));
        let metadata = Value::Record(BTreeMap::from([
            (
                "axes".into(),
                names([
                    "emission_channel",
                    "altitude_km",
                    "latitude_deg",
                    "longitude_deg",
                ]),
            ),
            ("wavelengths_nm".into(), numeric_list([557.7, 630.0, 427.8])),
            (
                "altitude_bounds_km".into(),
                numeric_list([ALT_MIN_KM, ALT_MAX_KM]),
            ),
            ("latitude_bounds_deg".into(), numeric_list([-90.0, 90.0])),
            ("longitude_bounds_deg".into(), numeric_list([0.0, 360.0])),
            (
                "units".into(),
                Value::String("relative_emission_weight".into()),
            ),
            (
                "sample_location".into(),
                Value::String("cell_center".into()),
            ),
        ]));
        Ok(Value::Record(BTreeMap::from([
            ("space_weather".into(), output),
            ("aurora".into(), aurora),
            ("grid".into(), metadata),
        ])))
    }
}

fn scalar(
    args: &Args,
    key: &str,
    default: Option<f64>,
    low: f64,
    high: f64,
) -> Result<f64, String> {
    let value = match args.get(key) {
        Some(v) => num(v)?,
        None => default.ok_or_else(|| format!("Missing {key}"))?,
    };
    (low..=high)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must be in [{low}, {high}]"))
}

fn count(args: &Args, key: &str, default: usize, min: usize, max: usize) -> Result<usize, String> {
    let value = args
        .get(key)
        .map(|v| integer(v, key))
        .transpose()?
        .unwrap_or(default);
    (min..=max)
        .contains(&value)
        .then_some(value)
        .ok_or_else(|| format!("{key} must be in [{min}, {max}]"))
}

fn dimensions<const N: usize>(
    args: &Args,
    key: &str,
    default: [usize; N],
    max: usize,
) -> Result<[usize; N], String> {
    let Some(value) = args.get(key) else {
        return Ok(default);
    };
    let mut shape = [0; N];
    let axes = list(value)?;
    if axes.len() != N {
        return Err(format!("{key} must have {N} dimensions"));
    }
    for (axis, input) in shape.iter_mut().zip(axes) {
        *axis = integer(input, key)?;
        if !(2..=max).contains(axis) {
            return Err(format!("{key} dimensions must be in [2, {max}]"));
        }
    }
    Ok(shape)
}

fn nonnegative_rgb(args: &Args, key: &str) -> Result<[f32; 3], String> {
    let rgb = vector::<3>(required(args, key)?)?;
    if rgb.iter().any(|x| !(0.0..=1.0).contains(x)) {
        return Err(format!("{key} components must be in [0, 1] per metre"));
    }
    Ok(rgb.map(|v| v as f32))
}

fn unit(v: [f64; 3], key: &str) -> Result<[f64; 3], String> {
    let norm = v[0].hypot(v[1]).hypot(v[2]);
    if !norm.is_finite() || norm <= 1e-12 {
        return Err(format!("{key} must be a finite nonzero direction"));
    }
    Ok(v.map(|x| x / norm))
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn gaussian(delta: f64, sigma: f64) -> f64 {
    (-0.5 * (delta / sigma).powi(2)).exp()
}

fn rgba_sample(output: &mut Vec<f32>, rgb: [f32; 3]) {
    output.extend([rgb[0], rgb[1], rgb[2], 1.0]);
}

fn rgba_artifact(
    host: &mut AssetHost<'_>,
    width: usize,
    height: usize,
    samples: Vec<f32>,
) -> Result<Value, String> {
    host.admit_artifact(TypedArtifact::RadianceLattice(Arc::new(RadianceLattice {
        width: width as u32,
        height: height as u32,
        channels: 4,
        primaries_iri: "http://www.w3.org/ns/ttml/parameter/rec709".into(),
        transfer_iri: "urn:qualia:color:transfer:linear".into(),
        samples: LatticeSamples::F32(samples),
    })))
}

fn numeric_list(values: impl IntoIterator<Item = f64>) -> Value {
    Value::List(
        values
            .into_iter()
            .map(Value::F64)
            .collect::<Vec<_>>()
            .into(),
    )
}

fn names<const N: usize>(names: [&str; N]) -> Value {
    Value::List(names.map(|name| Value::String(name.into())).to_vec().into())
}

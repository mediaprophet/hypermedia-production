//! Local patina film and abrasion integration on a PBR atlas.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::{artifact::FieldLattice, radiance_precision::Rgba16, *};
use color_transforms::{Transfer, linear_to_srgb, to_linear};
use std::sync::Arc;

impl AssetHost<'_> {
    pub(crate) fn simulate_weathering_and_wear(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["mode", "maps", "exposure", "years", "material"])?;
        if text(required(a, "mode")?)? != "local_patina_atlas" {
            return Err("Expected local_patina_atlas with tagged RGBA16 maps".into());
        }
        let maps = record(required(a, "maps")?)?;
        fields(maps, &["base_color", "roughness"])?;
        let exposure = record(required(a, "exposure")?)?;
        fields(
            exposure,
            &[
                "water_film_fraction",
                "runoff_per_year",
                "sliding_distance_m_per_year",
                "contact_pressure_pa",
                "exposure_mask",
            ],
        )?;
        let material = record(required(a, "material")?)?;
        fields(
            material,
            &[
                "patina_rate_per_year",
                "wear_coefficient",
                "hardness_pa",
                "patina_optical_depth_m",
                "patina_color_linear",
                "patina_roughness",
            ],
        )?;
        let years = bounded(num(required(a, "years")?)?, 0.0, 1_000.0, "years")?;
        let film = scalar(exposure, "water_film_fraction", 0.0, 1.0)?;
        let runoff = scalar(exposure, "runoff_per_year", 0.0, 100_000.0)?;
        let sliding = scalar(exposure, "sliding_distance_m_per_year", 0.0, 1e6)?;
        let pressure = scalar(exposure, "contact_pressure_pa", 0.0, 1e9)?;
        let rate = scalar(material, "patina_rate_per_year", 0.0, 100_000.0)?;
        let wear = scalar(material, "wear_coefficient", 0.0, 1.0)?;
        let hardness = scalar(material, "hardness_pa", 1e3, 1e12)?;
        let depth = scalar(material, "patina_optical_depth_m", 1e-9, 0.1)?;
        let patina = vector::<3>(required(material, "patina_color_linear")?)?;
        if patina.iter().any(|c| !(0.0..=1.0).contains(c)) {
            return Err("Patina color must be normalized".into());
        }
        let patina_roughness = scalar(material, "patina_roughness", 0.0, 1.0)?;
        let base_handle = required(maps, "base_color")?;
        let roughness_handle = required(maps, "roughness")?;
        let base = self.rgba16(base_handle)?;
        let shape = (base.width, base.height);
        let transfer = base
            .transfer
            .filter(|value| matches!(value, Transfer::Linear | Transfer::Srgb))
            .ok_or("Base color must carry linear or sRGB transfer metadata")?;
        if !(1..=2048).contains(&shape.0) || !(1..=2048).contains(&shape.1) {
            return Err("Patina atlases are limited to 2048×2048".into());
        }
        let n = shape.0 as usize * shape.1 as usize;
        self.charge(n * 120, n * 36)?;
        let base = self.rgba16(base_handle)?;
        let roughness = scalar_raster(self.rgba16(roughness_handle)?, shape, n)?;
        let mask = exposure
            .get("exposure_mask")
            .map(|handle| {
                self.rgba16(handle)
                    .and_then(|image| scalar_raster(image, shape, n))
            })
            .transpose()?;
        if base.pixels.len() != n {
            return Err("Base color sample count differs from the admitted atlas shape".into());
        }
        // Closed-form integration of dθ/dt = formation*(1-θ) - removal*θ.
        // The independent Archard recession is coefficient*pressure/hardness*distance.
        let formation = rate * film;
        let recession = wear * pressure / hardness * sliding;
        let removal = runoff + recession / depth;
        let lambda = formation + removal;
        let mut output_base = Vec::with_capacity(n);
        let mut output_roughness = Vec::with_capacity(n);
        let mut state = vec![0.0_f32; 2 * n];
        for (i, (&pixel, &rough)) in base.pixels.iter().zip(roughness).enumerate() {
            let e = mask.map_or(1.0, |values| f64::from(values[i][0]) / 65535.0);
            let coverage = if lambda == 0.0 {
                0.0
            } else {
                formation / lambda * -(-lambda * e * years).exp_m1()
            };
            let loss = recession * e * years;
            state[i] = coverage as f32;
            state[n + i] = loss as f32;
            let mix_color = |channel: usize| {
                let source = to_linear(f32::from(pixel[channel]) / 65535.0, transfer) as f64;
                let mixed = (source * (1.0 - coverage) + patina[channel] * coverage) as f32;
                quantize(if matches!(transfer, Transfer::Srgb) {
                    linear_to_srgb(mixed)
                } else {
                    mixed
                })
            };
            let value = f64::from(rough[0]) / 65535.0;
            let mixed = quantize((value * (1.0 - coverage) + patina_roughness * coverage) as f32);
            output_base.push([mix_color(0), mix_color(1), mix_color(2), pixel[3]]);
            output_roughness.push([mixed, mixed, mixed, rough[3]]);
        }
        let base = self.store_rgba16(shape.0, shape.1, output_base, Some(transfer))?;
        let rough =
            self.store_rgba16(shape.0, shape.1, output_roughness, Some(Transfer::Linear))?;
        let artifact = self.admit_artifact(TypedArtifact::FieldLattice(Arc::new(
            FieldLattice::DenseF32 {
                axes: vec![2, shape.1, shape.0],
                samples: state,
            },
        )))?;
        Ok(Value::Record(Args::from([
            (
                "maps".into(),
                Value::Record(Args::from([
                    ("base_color".into(), base),
                    ("roughness".into(), rough),
                ])),
            ),
            ("state".into(), artifact),
            (
                "state_channels".into(),
                Value::List(vec![
                    Value::String("film_fraction".into()),
                    Value::String("wear_depth_m".into()),
                ]),
            ),
        ])))
    }
}

fn scalar(a: &Args, key: &str, low: f64, high: f64) -> Result<f64, String> {
    bounded(num(required(a, key)?)?, low, high, key)
}

fn bounded(v: f64, low: f64, high: f64, key: &str) -> Result<f64, String> {
    if (low..=high).contains(&v) {
        Ok(v)
    } else {
        Err(format!("{key} outside {low}..={high}"))
    }
}

fn scalar_raster(image: &Rgba16, shape: (u32, u32), n: usize) -> Result<&[[u16; 4]], String> {
    if (image.width, image.height) != shape
        || image.pixels.len() != n
        || !matches!(image.transfer, None | Some(Transfer::Linear))
        || image.pixels.iter().any(|p| p[0] != p[1] || p[1] != p[2])
    {
        return Err("Scalar atlas requires matching linear, monochrome RGBA16 samples".into());
    }
    Ok(&image.pixels)
}

fn quantize(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * 65535.0).round() as u16
}

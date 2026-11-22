//! Enthalpy stepping for the water inventory of an M64 stratum.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use super::{bounded_number, AssetHost, Value};
use crate::{fields, integer, record, required, text, Args, Resource};
use qualia_core_db::{
    domains::physical::thermodynamics::phase_change_energy,
    q42::m64_matter::{M64DecodedRecord, M64PayloadInterval, M64ReadLimits, M64Reader},
};
use serde_json::json;
use sha2::{Digest, Sha256};

const ICE_POINT_K: f64 = 273.15;
const MIN_TEMPERATURE_K: f64 = 233.15;
const MAX_TEMPERATURE_K: f64 = 353.15;
const MAX_CHUNKS: usize = 1024;
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;

impl AssetHost<'_> {
    pub(crate) fn solve_m64_thermochem_electro(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["matter", "process", "operating_profile", "dt_sec"])?;
        if text(required(args, "process")?)? != "phase_change_water_ice" {
            return Err("Expected process phase_change_water_ice".into());
        }
        let source = required(args, "matter")?;
        let profile = record(required(args, "operating_profile")?)?;
        fields(
            profile,
            &[
                "chunk_index",
                "record_index",
                "layer_index",
                "column_area_m2",
                "pore_volume_fraction",
                "water_density_kg_m3",
                "solid_heat_capacity_j_kg_k",
                "liquid_heat_capacity_j_kg_k",
                "specific_latent_heat_j_kg",
                "temperature_k",
                "liquid_fraction",
                "net_heat_w",
            ],
        )?;
        let chunk_index = integer(required(profile, "chunk_index")?, "chunk_index")?;
        let record_index = integer(required(profile, "record_index")?, "record_index")?;
        let layer = integer(required(profile, "layer_index")?, "layer_index")?;
        let area = bounded_number(profile, "column_area_m2", None, 1e-6, 1e4)?;
        let porosity = bounded_number(profile, "pore_volume_fraction", None, 1e-6, 1.0)?;
        let density = bounded_number(profile, "water_density_kg_m3", None, 800.0, 1200.0)?;
        let solid_cp = bounded_number(profile, "solid_heat_capacity_j_kg_k", None, 1000.0, 3000.0)?;
        let liquid_cp =
            bounded_number(profile, "liquid_heat_capacity_j_kg_k", None, 3000.0, 5000.0)?;
        let latent = bounded_number(profile, "specific_latent_heat_j_kg", None, 2e5, 4e5)?;
        let before = bounded_number(
            profile,
            "temperature_k",
            None,
            MIN_TEMPERATURE_K,
            MAX_TEMPERATURE_K,
        )?;
        let before_fraction = bounded_number(profile, "liquid_fraction", None, 0.0, 1.0)?;
        if (before < ICE_POINT_K && before_fraction > 0.0)
            || (before > ICE_POINT_K && before_fraction < 1.0)
        {
            return Err("Initial liquid_fraction and temperature_k describe incompatible equilibrium phases".into());
        }
        let net_heat = bounded_number(profile, "net_heat_w", None, -1e8, 1e8)?;
        let dt = bounded_number(args, "dt_sec", None, 1e-6, 86_400.0)?;
        let energy_in = net_heat * dt;

        let source_size = retained_matter(self, source)?.bytes.len();
        let max_source = self.limits.resource_bytes.min(MAX_SOURCE_BYTES);
        if source_size > max_source {
            return Err("Thermal matter source exceeds the byte budget".into());
        }
        let max_chunks = (source_size / 64).min(MAX_CHUNKS);
        self.charge(
            source_size * 4 + 100,
            max_chunks * std::mem::size_of::<M64PayloadInterval>() + 4096,
        )?;
        let bytes = &retained_matter(self, source)?.bytes;
        let reader = M64Reader::parse(
            bytes,
            M64ReadLimits {
                max_file_bytes: max_source,
                max_chunks,
                max_records: (source_size / 64) as u64,
            },
            &mut vec![M64PayloadInterval::default(); max_chunks],
        )
        .map_err(|e| format!("Thermal M64 source: {e:?}"))?;
        if reader.georaster().is_some() {
            return Err("This thermal process requires material-bearing M64 strata records".into());
        }
        let source_chunk = reader
            .chunk(chunk_index)
            .ok_or("Thermal M64 chunk is out of range")?;
        let Some(M64DecodedRecord::Strata(column)) = source_chunk
            .decode_record(record_index)
            .map_err(|e| format!("Thermal M64 record: {e:?}"))?
        else {
            return Err("The selected M64 record is not a strata column".into());
        };
        let thickness_mm = *column
            .layer_thickness_mm
            .get(layer)
            .ok_or("Thermal M64 layer is out of range")?;
        let saturation = f64::from(column.layer_moisture_pct[layer]) / 255.0;
        let mass = area * f64::from(thickness_mm) / 1000.0 * porosity * saturation * density;
        if mass <= 0.0 {
            return Err("The selected M64 stratum contains no modeled pore water".into());
        }
        let digest = format!("{:x}", Sha256::digest(bytes));

        let solid_capacity = mass * solid_cp;
        let liquid_capacity = mass * liquid_cp;
        let melt_energy = phase_change_energy(mass, latent);
        let initial_enthalpy = if before < ICE_POINT_K {
            (before - ICE_POINT_K) * solid_capacity
        } else {
            before_fraction * melt_energy + (before - ICE_POINT_K) * liquid_capacity
        };
        let enthalpy = initial_enthalpy + energy_in;
        let (temperature, liquid_fraction) = if enthalpy < 0.0 {
            (ICE_POINT_K + enthalpy / solid_capacity, 0.0)
        } else if enthalpy <= melt_energy {
            (ICE_POINT_K, enthalpy / melt_energy)
        } else {
            (
                ICE_POINT_K + (enthalpy - melt_energy) / liquid_capacity,
                1.0,
            )
        };
        if !(MIN_TEMPERATURE_K..=MAX_TEMPERATURE_K).contains(&temperature) {
            return Err("Result temperature exceeds the water–ice model range".into());
        }
        let represented_enthalpy = if temperature < ICE_POINT_K {
            (temperature - ICE_POINT_K) * solid_capacity
        } else {
            liquid_fraction * melt_energy + (temperature - ICE_POINT_K) * liquid_capacity
        };
        let telemetry = json!({
            "schema": "qualia.m64-stratum-water-ice-state/v1",
            "source_sha256": digest,
            "chunk_index": chunk_index, "record_index": record_index, "layer_index": layer,
            "layer_thickness_mm": thickness_mm,
            "layer_lut_index": column.layer_lut_index[layer],
            "layer_water_saturation": saturation,
            "pore_volume_fraction": porosity, "column_area_m2": area,
            "water_density_kg_m3": density,
            "solid_heat_capacity_j_kg_k": solid_cp,
            "liquid_heat_capacity_j_kg_k": liquid_cp,
            "specific_latent_heat_j_kg": latent,
            "water_mass_kg": mass, "initial_temperature_k": before,
            "initial_liquid_fraction": before_fraction,
            "temperature_k": temperature, "liquid_fraction": liquid_fraction,
            "latent_energy_capacity_j": melt_energy,
            "net_heat_w": net_heat, "dt_sec": dt,
            "energy_input_j": energy_in, "enthalpy_change_j": represented_enthalpy - initial_enthalpy,
            "energy_residual_j": represented_enthalpy - enthalpy,
            "thermal_model": "lumped_pore_water_fixed_mass",
        });
        let telemetry = self.admit_encoded(
            "json",
            serde_json::to_vec(&telemetry).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record(
            [
                ("temperature_k".into(), Value::F64(temperature)),
                ("liquid_fraction".into(), Value::F64(liquid_fraction)),
                ("telemetry".into(), telemetry),
            ]
            .into(),
        ))
    }
}

fn retained_matter<'a>(host: &'a AssetHost<'_>, handle: &Value) -> Result<&'a Resource, String> {
    let resource = match handle {
        Value::String(key) => host
            .inputs
            .iter()
            .find(|input| input.key == *key)
            .ok_or("Unknown retained M64 matter input")?,
        _ => host.workspace.encoded(handle)?,
    };
    if resource.kind != "m64" {
        return Err("This thermal process requires an M64 input".into());
    }
    Ok(resource)
}

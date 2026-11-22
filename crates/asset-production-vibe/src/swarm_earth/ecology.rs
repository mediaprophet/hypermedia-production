//! Native Etalon wildfire assessment, soil response and cover sampling.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use qualia_core_db::etalon_render::vegetation::ground_cover::{
    TOROIDAL_GRID_DIM, ToroidalGroundCoverClipmap,
};
use qualia_core_db::simulation::environment::{
    BarometricState, BoundaryLayerProfile, ForestFuelStratum, FuelMoistureState,
    SoilMechanicsProfile, SolarWindState, evaluate_heliophysical_fire_weather,
    evaluate_soil_compaction, evaluate_solar_cycle_phase,
    excavation::{ToolEffector, evaluate_excavation_pass},
};
use sha2::{Digest, Sha256};

use crate::{Args, AssetHost, Value, fields, record, required, text, vector};

use super::{bounded, lattice, receipt};

impl AssetHost<'_> {
    pub(crate) fn step_fire_and_soil_coupled(&mut self, args: &Args) -> Result<Value, String> {
        fields(args, &["fuel", "weather", "space_weather", "soil", "cover"])?;
        self.charge(
            TOROIDAL_GRID_DIM * TOROIDAL_GRID_DIM * 48,
            TOROIDAL_GRID_DIM * TOROIDAL_GRID_DIM * 96,
        )?;
        let fuel = record(required(args, "fuel")?)?;
        fields(
            fuel,
            &[
                "surface_litter_t_ha",
                "near_surface_grass_t_ha",
                "elevated_shrub_t_ha",
                "bark_hazard_score",
                "canopy_foliage_t_ha",
                "kbdi_drought_index",
                "grass_curing_percentage",
            ],
        )?;
        let load = |key| bounded(fuel, key, 0.0, 100.0).map(|v| v as f32);
        let fuel_stratum = ForestFuelStratum {
            surface_litter_t_ha: load("surface_litter_t_ha")?,
            near_surface_grass_t_ha: load("near_surface_grass_t_ha")?,
            elevated_shrub_t_ha: load("elevated_shrub_t_ha")?,
            bark_hazard_score: bounded(fuel, "bark_hazard_score", 0.0, 5.0)? as f32,
            canopy_foliage_t_ha: load("canopy_foliage_t_ha")?,
        };
        let fuel_moisture = FuelMoistureState {
            // Etalon derives current dead fuel moisture from the observational weather state.
            dead_fuel_moisture_fraction: 0.0,
            kbdi_drought_index: bounded(fuel, "kbdi_drought_index", 0.0, 200.0)? as f32,
            grass_curing_percentage: bounded(fuel, "grass_curing_percentage", 0.0, 100.0)? as f32,
        };

        let weather = record(required(args, "weather")?)?;
        fields(
            weather,
            &[
                "mslp_hpa",
                "temperature_c",
                "dew_point_c",
                "relative_humidity",
                "wind_vector_10m_m_s",
                "roughness_length_m",
                "zero_plane_displacement_m",
                "slope_deg",
                "elevation_m",
            ],
        )?;
        let wind = vector::<2>(required(weather, "wind_vector_10m_m_s")?)?;
        let speed = wind[0].hypot(wind[1]);
        if speed > 50.0 {
            return Err("10 m wind magnitude must be at most 50 m/s".into());
        }
        let baro = BarometricState {
            mslp_hpa: bounded(weather, "mslp_hpa", 850.0, 1100.0)? as f32,
            temperature_c: bounded(weather, "temperature_c", -80.0, 65.0)? as f32,
            dew_point_c: bounded(weather, "dew_point_c", -120.0, 65.0)? as f32,
            relative_humidity: bounded(weather, "relative_humidity", 0.0, 1.0)? as f32,
            wind_vector_10m: wind.map(|v| v as f32),
        };
        if baro.dew_point_c > baro.temperature_c {
            return Err("Dew point must be at or below ambient temperature".into());
        }
        let roughness = bounded(weather, "roughness_length_m", 0.001, 3.0)? as f32;
        let abl = BoundaryLayerProfile {
            roughness_length_z0: roughness,
            zero_plane_displacement_d: bounded(weather, "zero_plane_displacement_m", 0.0, 30.0)?
                as f32,
            friction_velocity_u_star: speed as f32
                * qualia_core_db::simulation::environment::atmosphere::VON_KARMAN_KAPPA
                / (10.0 / roughness).ln(),
        };

        let space = record(required(args, "space_weather")?)?;
        fields(
            space,
            &[
                "decimal_year",
                "velocity_km_s",
                "proton_density_cm3",
                "imf_total_nt",
                "imf_bz_nt",
            ],
        )?;
        let (cycle, _) =
            evaluate_solar_cycle_phase(bounded(space, "decimal_year", 1900.0, 2150.0)? as f32);
        let velocity = bounded(space, "velocity_km_s", 150.0, 3000.0)? as f32;
        let density = bounded(space, "proton_density_cm3", 0.001, 200.0)? as f32;
        let total = bounded(space, "imf_total_nt", 0.0, 200.0)? as f32;
        let wind_plasma = SolarWindState {
            velocity_km_s: velocity,
            proton_density_cm3: density,
            imf_total_nt: total,
            imf_bz_nt: bounded(space, "imf_bz_nt", -f64::from(total), f64::from(total))? as f32,
            dynamic_pressure_npa: 1.672_621_9e-6 * density * velocity.powi(2),
        };
        let report = evaluate_heliophysical_fire_weather(
            &cycle,
            &wind_plasma,
            &baro,
            &abl,
            &fuel_stratum,
            &fuel_moisture,
            bounded(weather, "slope_deg", -60.0, 60.0)? as f32,
            bounded(weather, "elevation_m", -500.0, 6000.0)? as f32,
        );

        let soil = record(required(args, "soil")?)?;
        fields(
            soil,
            &[
                "profile",
                "temperature_c",
                "moisture_fraction",
                "porosity_fraction",
                "contact_pressure_kpa",
                "excavation_tool",
                "excavation_depth_m",
            ],
        )?;
        let soil_moisture = bounded(soil, "moisture_fraction", 0.0, 1.0)? as f32;
        let profile = match text(required(soil, "profile")?)? {
            "humic_root_mat" => SoilMechanicsProfile::humic_root_mat(),
            "loam" => SoilMechanicsProfile::loam(),
            "illuvial_clay" => SoilMechanicsProfile::illuvial_clay(),
            "saprolite" => SoilMechanicsProfile::saprolite(),
            "bedrock" => SoilMechanicsProfile::bedrock(),
            _ => return Err("Unknown authored soil profile".into()),
        }
        .with_seasonal_state(
            bounded(soil, "temperature_c", -55.0, 65.0)? as f32,
            soil_moisture,
        );
        let initial_porosity = bounded(soil, "porosity_fraction", 0.17, 0.85)? as f32;
        let (porosity, conductivity) = evaluate_soil_compaction(
            bounded(soil, "contact_pressure_kpa", 0.0, 1000.0)? as f32,
            initial_porosity,
            &profile,
        );
        let excavation = match (soil.get("excavation_tool"), soil.get("excavation_depth_m")) {
            (None, None) => serde_json::Value::Null,
            (Some(tool), Some(_)) => {
                let tool = match text(tool)? {
                    "hand_spade" => ToolEffector::hand_spade(),
                    "pickaxe" => ToolEffector::pickaxe(),
                    "mini_excavator" => ToolEffector::mini_excavator(),
                    _ => return Err("Unknown authored excavation tool".into()),
                };
                let verdict = evaluate_excavation_pass(
                    &tool,
                    bounded(soil, "excavation_depth_m", 0.001, 2.0)? as f32,
                    &profile,
                );
                serde_json::json!({
                    "severed": verdict.severed,
                    "required_force_n": verdict.required_force_n,
                    "excavated_bank_volume_m3": verdict.excavated_bank_volume_m3,
                    "excavated_loose_volume_m3": verdict.excavated_loose_volume_m3,
                    "root_mat_severed": verdict.root_mat_severed,
                    "refusal_hit": verdict.refusal_hit,
                })
            }
            _ => return Err("Excavation tool and depth must be supplied together".into()),
        };

        let cover = record(required(args, "cover")?)?;
        fields(
            cover,
            &[
                "center_xz_m",
                "cell_size_m",
                "species_mix",
                "culm_height_m",
                "leaf_area_index",
                "turgor_pressure_mpa",
            ],
        )?;
        let center = vector::<2>(required(cover, "center_xz_m")?)?;
        if center.iter().any(|v| v.abs() > 10_000_000.0) {
            return Err("Cover center must fit the local metre coordinate bounds".into());
        }
        let cell_size = bounded(cover, "cell_size_m", 0.25, 50.0)? as f32;
        let mix = vector::<4>(required(cover, "species_mix")?)?;
        let total_weight = mix.iter().sum::<f64>();
        if mix.iter().any(|&v| v < 0.0) || (total_weight - 1.0).abs() > 1e-5 {
            return Err("Cover species_mix requires four nonnegative weights summing to 1".into());
        }
        let height = bounded(cover, "culm_height_m", 0.0, 2.0)? as f32;
        let leaf_area_index = bounded(cover, "leaf_area_index", 0.0, 25.0)? as f32;
        let turgor = bounded(cover, "turgor_pressure_mpa", 0.0, 10.0)? as f32;
        let mut clipmap =
            ToroidalGroundCoverClipmap::new(center[0] as f32, center[1] as f32, cell_size);
        for patch in &mut clipmap.cells {
            patch.species_mix = mix.map(|v| (v / total_weight) as f32);
            patch.culm_height_m = height;
            patch.leaf_area_index = leaf_area_index;
            patch.turgor_pressure_mpa = turgor;
            patch.soil_moisture = soil_moisture;
        }
        let side = TOROIDAL_GRID_DIM as i32;
        let start = center.map(|v| (v / f64::from(cell_size)).floor() as i32 - side / 2);
        let mut samples = Vec::with_capacity(TOROIDAL_GRID_DIM * TOROIDAL_GRID_DIM * 12);
        let mut cover_hash = Sha256::new();
        for index in start {
            cover_hash.update(index.to_le_bytes());
        }
        cover_hash.update(cell_size.to_le_bytes());
        for gz in start[1]..start[1] + side {
            for gx in start[0]..start[0] + side {
                let patch = &clipmap.cells[clipmap.cell_index_at(gx, gz)];
                let dx = (gx as f64 + 0.5) * f64::from(cell_size) - center[0];
                let dz = (gz as f64 + 0.5) * f64::from(cell_size) - center[1];
                let [fiber_x, fiber_y, fiber_z] = clipmap.evaluate_sggx_fiber_direction(patch);
                let values = [
                    patch.species_mix[0],
                    patch.species_mix[1],
                    patch.species_mix[2],
                    patch.species_mix[3],
                    patch.culm_height_m,
                    patch.leaf_area_index,
                    patch.turgor_pressure_mpa,
                    patch.soil_moisture,
                    fiber_x,
                    fiber_y,
                    fiber_z,
                    clipmap.evaluate_lod(dx.hypot(dz) as f32) as u8 as f32,
                ];
                for value in values {
                    cover_hash.update(value.to_le_bytes());
                }
                samples.extend_from_slice(&values);
            }
        }
        let metadata = serde_json::json!({
            "fireline": {
                "regime_code": report.fireline.regime as u8,
                "intensity_kw_m": report.fireline.fireline_intensity_kw_m,
                "forward_spread_rate_m_s": report.fireline.forward_spread_rate_m_s,
                "flame_length_m": report.fireline.flame_length_m,
                "dead_fuel_moisture_fraction": report.dfmc_fraction,
                "air_density_kg_m3": report.air_density_kg_m3,
                "kp_index": report.storm.kp_index,
            },
            "soil_report": {
                "initial_porosity_fraction": initial_porosity,
                "compacted_porosity_fraction": porosity,
                "hydraulic_conductivity_ratio": conductivity,
                "excavation": excavation,
            },
            "cover_axes": ["z", "x", "channel"],
            "cover_origin_xz_m": start.map(|v| (f64::from(v) + 0.5) * f64::from(cell_size)),
            "cover_cell_size_m": cell_size,
            "cover_channels": ["ryegrass", "clover", "tussock", "herbs", "culm_height_m", "leaf_area_index", "turgor_pressure_mpa", "soil_moisture", "fiber_x", "fiber_y", "fiber_z", "lod"],
        });
        let report_bytes = serde_json::to_vec(&metadata).map_err(|e| e.to_string())?;
        let graph_subject = format!("urn:sha256:{:x}", Sha256::digest(&report_bytes));
        let graph_cover = format!("urn:sha256:{:x}", cover_hash.finalize());
        let mut response = BTreeMap::new();
        response.insert("report".into(), self.encode("json", report_bytes)?);
        response.insert(
            "ground_cover".into(),
            lattice(self, vec![side as u32, side as u32, 12], samples)?,
        );
        response.insert(
            "receipt".into(),
            receipt(
                self,
                "fire-and-soil-step",
                &[
                    (
                        &graph_subject,
                        "http://www.w3.org/ns/prov#wasGeneratedBy",
                        "urn:qualia:earth:heliophysical-fire-and-soil",
                    ),
                    (
                        &graph_subject,
                        "http://www.w3.org/ns/prov#hadMember",
                        &graph_cover,
                    ),
                ],
            )?,
        );
        Ok(Value::Record(response))
    }
}

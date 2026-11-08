//! Deterministic geomorphological heightfield erosion and surface modeling solvers.
//!
//! Provides pure mathematical 2D heightfield solvers for physical terrain processes:
//! - Multi-scale thermal slope relaxation and scree synthesis (`thermal_shaper`)
//! - Particle-based hydraulic droplet erosion and sediment transport (`hydraulic_droplet`)
//! - Hydrological priority-flood depression conditioning and D8 flow routing (`drainage_pyramid`)
//! - Elevation-dependent snowfall, freeze/thaw, and avalanche mass transport (`snow_accumulation`)
//! - Hydrostatic water-table lake filling and coastal shoreline smoothing (`lacustrine_fill`)
//! - Planetary impact crater cavity and ejecta rim lip morphology (`crater_morphology`)
//! - Stratigraphic stepped terrace rock layer modulation (`fractal_terrace`)
//!
//! Grounded in international human rights instruments (UDHR Article 27; ICESCR Article 15)
//! and Informatics Fiduciary duties ensuring scientific rigor, deterministic simulation,
//! and moral attribution in environmental modeling.

#![forbid(unsafe_code)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

pub mod crater_morphology;
pub mod drainage_pyramid;
pub mod fractal_terrace;
pub mod hydraulic_droplet;
pub mod lacustrine_fill;
pub mod snow_accumulation;
pub mod thermal_shaper;

pub use crater_morphology::{
    Crater, CraterfieldConfig, apply_crater, generate_craterfield,
};
pub use drainage_pyramid::{
    D8_DISTANCES, D8_OFFSETS, DrainageNetwork, carve_channels, d8_flow_directions,
    drainage_network, flow_accumulation, priority_flood_fill,
};
pub use fractal_terrace::{TerraceConfig, apply_terraces, evaluate_stratum};
pub use hydraulic_droplet::{HydraulicConfig, HydraulicOutput, hydraulic_erosion};
pub use lacustrine_fill::{LakeConfig, WaterOutput, fill_lakes};
pub use snow_accumulation::{SnowConfig, SnowOutput, simulate_snow};
pub use thermal_shaper::{ThermalShaperConfig, thermal_shape, thermal_shape_with};

/// Errors arising during terrain solver execution or grid validation.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum TerrainError {
    #[error("grid dimensions ({width}x{height}) do not match buffer length {actual}")]
    DimensionMismatch {
        width: usize,
        height: usize,
        actual: usize,
    },
    #[error("invalid parameter: {0}")]
    InvalidParameter(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_geomorphology_pipeline_integration() {
        let size = 16;
        let mut heights = vec![0.5f32; size * size];

        // 1. Crater impact
        let crater = Crater {
            x: 8.0,
            y: 8.0,
            radius: 4.0,
            depth: 0.3,
        };
        apply_crater(&mut heights, size, size, &crater, 0.25, 0.1);
        assert!(heights[8 * size + 8] < 0.5);

        // 2. Terraces
        let t_cfg = TerraceConfig {
            spacing: 0.15,
            intensity: 0.5,
            ..Default::default()
        };
        apply_terraces(&mut heights, size, size, &t_cfg);

        // 3. Thermal slope relaxation
        let relaxed = thermal_shape(&heights, size, 0.2);
        assert_eq!(relaxed.len(), size * size);
        for &h in &relaxed {
            assert!(h.is_finite());
        }

        // 4. Hydrology & Drainage
        let mut d_map = relaxed.clone();
        priority_flood_fill(&mut d_map, size, size);
        let dirs = d8_flow_directions(&d_map, size, size);
        let acc = flow_accumulation(&d_map, &dirs, size, size);
        assert_eq!(acc.len(), size * size);

        // 5. Snow
        let s_cfg = SnowConfig {
            snow_line: 0.45,
            iterations: 2,
            ..Default::default()
        };
        let s_out = simulate_snow(&d_map, size, size, &s_cfg);
        assert_eq!(s_out.combined_height.len(), size * size);

        // 6. Lakes
        let l_cfg = LakeConfig {
            water_floor: 0.3,
            iterations: 4,
            ..Default::default()
        };
        let l_out = fill_lakes(&d_map, size, size, &l_cfg);
        assert_eq!(l_out.results.len(), size * size);
    }
}

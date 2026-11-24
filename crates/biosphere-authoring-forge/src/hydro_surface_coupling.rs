//! # Hydrological Shallow-Flow Wakes, Buoyancy/Drag & Washburn Porous Wetting (`hydro_surface_coupling.rs`)
//!
//! Bridges terrain hydrology (`terrain-erosion-solvers`) and open-water Gerstner spectra
//! (`etalon-render::water_surface`) with real-time asset-water and rain-surface interactions:
//! 1. **2D Saint-Venant Shallow-Water Channel Velocity, Froude Regime & Whitewater Aeration:**
//!    Manning discharge velocity $\mathbf{u}_w = \frac{1}{n} R_h^{2/3} S^{1/2}$, Froude number
//!    $\text{Fr} = |\mathbf{u}_w| / \sqrt{g h_w}$ (subcritical tranquil flow vs. supercritical
//!    rapids/hydraulic jumps), and bed shear stress $\tau_b = \rho_w g h_w S$.
//! 2. **Obstacle Bow Wakes, Kelvin Angles, Karman Vortex Streets & Hydrodynamic Drag/Buoyancy:**
//!    Computes upstream stagnation bow-wave rise $\Delta h_{\text{bow}} = |\mathbf{u}_w|^2 / (2g)$,
//!    downstream Strouhal vortex shedding frequency $f_v = \text{St}\cdot |\mathbf{u}_w| / D$,
//!    Archimedes buoyancy uplift, and hydrodynamic drag force on boulders, logs, reeds, and wading fauna.
//! 3. **Washburn Porous Capillary Wetting, Drying Hysteresis & Lekner–Dorf Optical Darkening:**
//!    Tracks internal pore saturation $S_w \in [0, 1]$ and capillary wicked fringe height above
//!    waterlines; replacing pore air ($n = 1.0$) with water ($n = 1.33$) physically darkens diffuse
//!    albedo by $25\%\text{–}60\%$, increases specular Fresnel $F_0$, and smooths micro-roughness.

use serde::{Deserialize, Serialize};

/// Material porous-media category governing Washburn capillary absorption and optical darkening.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PorousMaterialClass {
    /// Fine clay/loam soil with high capillary rise and strong optical darkening when wet.
    LoamSoil,
    /// Coarse sand / gravel with rapid drainage and moderate darkening.
    CoarseSand,
    /// Porous sandstone or weathered granite rock face.
    WeatheredRock,
    /// Fibrous tree bark (e.g., stringybark / rough eucalypt cortex).
    FibrousBark,
    /// Mammalian pelage (fur) or avian plumage.
    PelageOrPlumage,
    /// Hydrophilic fungal pileus or bryophyte moss cushion.
    BryophyteOrFungalCap,
}

impl PorousMaterialClass {
    /// Returns `(capillary_absorption_rate, drying_rate, max_albedo_darkening_fraction, max_capillary_rise_m)`.
    #[inline]
    pub const fn parameters(self) -> (f32, f32, f32, f32) {
        match self {
            Self::LoamSoil => (0.45, 0.08, 0.58, 0.35),
            Self::CoarseSand => (0.75, 0.28, 0.42, 0.09),
            Self::WeatheredRock => (0.18, 0.22, 0.36, 0.06),
            Self::FibrousBark => (0.38, 0.12, 0.52, 0.22),
            Self::PelageOrPlumage => (0.25, 0.16, 0.45, 0.08),
            Self::BryophyteOrFungalCap => (0.85, 0.06, 0.50, 0.18),
        }
    }
}

/// Complete shallow-water channel flow and obstacle fluid-structure interaction report.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ShallowWaterObstacleReport {
    /// Depth-averaged stream velocity vector `[ux, uz]` in m/s.
    pub flow_velocity_m_s: [f32; 2],
    /// Flow speed $|\mathbf{u}_w|$ in m/s.
    pub flow_speed_m_s: f32,
    /// Dimensionless Froude number $\text{Fr} = |\mathbf{u}_w| / \sqrt{g h_w}$.
    pub froude_number: f32,
    /// `true` if $\text{Fr} > 1.0$ (supercritical rapids / standing hydraulic jump regime).
    pub is_supercritical_rapids: bool,
    /// Whitewater aeration foam fraction in `[0.0, 1.0]`.
    pub whitewater_foam_intensity: f32,
    /// Upstream stagnation bow-wave run-up height $\Delta h_{\text{bow}} = |\mathbf{u}_w|^2 / (2g)$ in metres.
    pub obstacle_bow_runup_m: f32,
    /// Downstream Karman vortex-street shedding frequency $f_v = \text{St} \cdot |\mathbf{u}_w| / D$ in Hz.
    pub karman_vortex_frequency_hz: f32,
    /// Hydrodynamic drag force `[fx, fz]` in Newtons exerted on the submerged obstacle/fauna legs.
    pub hydrodynamic_drag_force_n: [f32; 2],
    /// Archimedes upward buoyancy force in Newtons.
    pub buoyancy_uplift_n: f32,
    /// Bed shear stress $\tau_b$ in Pascals (reduced by riparian root cohesion).
    pub effective_bed_shear_pa: f32,
}

/// Evaluates 2D Saint-Venant / Manning shallow-water channel flow and its hydrodynamic wake,
/// bow run-up, Karman vortex frequency, drag, and buoyancy on a partially submerged `.10d` asset.
pub fn evaluate_shallow_water_obstacle_interaction_zero_heap(
    water_depth_m: f32,
    bed_slope_vector: [f32; 2],
    manning_roughness_n: f32,
    riparian_root_density: f32,
    obstacle_diameter_m: f32,
    obstacle_submerged_height_m: f32,
    obstacle_drag_cd: f32,
) -> ShallowWaterObstacleReport {
    let h = water_depth_m.max(0.0);
    if h <= 1e-4 {
        return ShallowWaterObstacleReport {
            flow_velocity_m_s: [0.0; 2],
            flow_speed_m_s: 0.0,
            froude_number: 0.0,
            is_supercritical_rapids: false,
            whitewater_foam_intensity: 0.0,
            obstacle_bow_runup_m: 0.0,
            karman_vortex_frequency_hz: 0.0,
            hydrodynamic_drag_force_n: [0.0; 2],
            buoyancy_uplift_n: 0.0,
            effective_bed_shear_pa: 0.0,
        };
    }
    let g = 9.80665f32;
    let rho_w = 1000.0f32;
    let sx = bed_slope_vector[0];
    let sz = bed_slope_vector[1];
    let s_mag = (sx * sx + sz * sz).sqrt();
    let n_manning = (manning_roughness_n.max(0.015)) * (1.0 + 0.45 * riparian_root_density.clamp(0.0, 1.0));

    // Manning velocity u = (1 / n) * h^(2/3) * S^(1/2)
    let speed = if s_mag > 1e-6 {
        (1.0 / n_manning) * h.powf(2.0 / 3.0) * s_mag.sqrt()
    } else {
        0.0
    };
    let flow_dir = if s_mag > 1e-6 {
        [-sx / s_mag, -sz / s_mag] // downslope
    } else {
        [0.0, 0.0]
    };
    let vel = [flow_dir[0] * speed, flow_dir[1] * speed];
    let froude = speed / (g * h).sqrt();
    let bow_runup_m = (speed * speed) / (2.0 * g);
    let d_obs = obstacle_diameter_m.max(0.01);
    let h_sub = obstacle_submerged_height_m.clamp(0.0, h);
    let vortex_hz = (0.20 * speed) / d_obs;

    let frontal_area = d_obs * h_sub;
    let drag_mag = 0.5 * rho_w * obstacle_drag_cd.max(0.1) * frontal_area * speed * speed;
    let drag_vec = [flow_dir[0] * drag_mag, flow_dir[1] * drag_mag];
    let submerged_vol = core::f32::consts::PI * (0.5 * d_obs).powi(2) * h_sub;
    let buoyancy_n = rho_w * g * submerged_vol;

    // Whitewater foam from Froude supercriticality + obstacle bow turbulence
    let foam = ((froude - 0.65).max(0.0) * 0.55 + (bow_runup_m / h.max(0.05)).clamp(0.0, 0.6)).clamp(0.0, 1.0);
    let root_shield = 1.0 - 0.65 * riparian_root_density.clamp(0.0, 1.0);
    let bed_shear_pa = rho_w * g * h * s_mag * root_shield;

    ShallowWaterObstacleReport {
        flow_velocity_m_s: vel,
        flow_speed_m_s: speed,
        froude_number: froude,
        is_supercritical_rapids: froude > 1.0,
        whitewater_foam_intensity: foam,
        obstacle_bow_runup_m: bow_runup_m,
        karman_vortex_frequency_hz: vortex_hz,
        hydrodynamic_drag_force_n: drag_vec,
        buoyancy_uplift_n: buoyancy_n,
        effective_bed_shear_pa: bed_shear_pa,
    }
}

/// Optical and physical porous-wetting state resulting from Washburn capillary absorption and drying hysteresis.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PorousWettingOpticalResult {
    /// Updated internal pore water saturation $S_w \in [0.0, 1.0]$.
    pub pore_water_saturation: f32,
    /// Capillary wicked fringe height above standing waterline in metres.
    pub capillary_fringe_height_m: f32,
    /// Lekner–Dorf internal-reflection diffuse albedo multiplier in `[0.40, 1.0]` (wet surfaces darken).
    pub diffuse_albedo_multiplier: f32,
    /// Wet-film specular Fresnel $F_0 \in [0.02, 0.055]$.
    pub specular_f0: f32,
    /// Effective PBR micro-roughness after water meniscus filling.
    pub effective_roughness: f32,
    /// Added water mass per unit surface area in $\text{kg/m}^2$ (loads fur, feathers, moss, and leaves).
    pub retained_water_mass_kg_m2: f32,
}

/// Steps Washburn porous capillary wetting & evaporative drying hysteresis, and computes the
/// Lekner–Dorf optical darkening, Fresnel boost, and roughness smoothing on a dry PBR material.
#[inline]
pub fn evaluate_washburn_porous_wetting_and_optics(
    material: PorousMaterialClass,
    current_saturation: f32,
    height_above_waterline_m: f32,
    rainfall_throughfall_mm_hr: f32,
    vapor_pressure_deficit_kpa: f32,
    wind_speed_m_s: f32,
    dry_roughness: f32,
    dt_hours: f32,
) -> PorousWettingOpticalResult {
    let (k_abs, k_dry, max_darken, max_wick_m) = material.parameters();
    let s0 = current_saturation.clamp(0.0, 1.0);
    let dt = dt_hours.max(0.0);

    // 1. Capillary meniscus wicking above a standing water body or stream
    let wick_saturation = if height_above_waterline_m <= 0.0 {
        1.0
    } else if height_above_waterline_m < max_wick_m {
        (1.0 - (height_above_waterline_m / max_wick_m).powf(1.4)).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // 2. Washburn rain absorption vs. Penman-style aerodynamic evaporation
    let rain_input = (rainfall_throughfall_mm_hr.max(0.0) / 8.0).clamp(0.0, 2.5);
    let washburn_boost = 1.0 / (s0 + 0.15); // Dry pores pull water in faster via capillary suction
    let absorption = k_abs * rain_input * washburn_boost * dt;
    let evap_drive = (vapor_pressure_deficit_kpa.max(0.05)) * (1.0 + 0.25 * wind_speed_m_s.max(0.0));
    let drying = k_dry * evap_drive * s0 * dt;

    let s_new = (s0 + absorption - drying).max(wick_saturation).clamp(0.0, 1.0);

    // 3. Lekner & Dorf (1988) internal reflection darkening: pore water (n=1.33) reduces total internal reflection
    let albedo_mult = (1.0 - max_darken * s_new.powf(0.75)).clamp(0.38, 1.0);
    let f0 = 0.04 * (1.0 - s_new) + 0.021 * s_new + 0.025 * s_new * s_new;
    let rough = (dry_roughness.clamp(0.05, 1.0) * (1.0 - 0.72 * s_new)).clamp(0.04, 1.0);
    let water_mass_kg_m2 = s_new * 1.85;

    PorousWettingOpticalResult {
        pore_water_saturation: s_new,
        capillary_fringe_height_m: max_wick_m,
        diffuse_albedo_multiplier: albedo_mult,
        specular_f0: f0,
        effective_roughness: rough,
        retained_water_mass_kg_m2: water_mass_kg_m2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shallow_water_rapids_generate_bow_wake_karman_vortices_and_washburn_wet_darkening() {
        let flow = evaluate_shallow_water_obstacle_interaction_zero_heap(
            0.45,
            [-0.045, 0.0],
            0.030,
            0.25,
            0.60,
            0.40,
            1.1,
        );
        assert!(flow.flow_speed_m_s > 1.5);
        assert!(flow.obstacle_bow_runup_m > 0.10);
        assert!(flow.karman_vortex_frequency_hz > 0.5);
        assert!(flow.hydrodynamic_drag_force_n[0] > 100.0);
        assert!(flow.buoyancy_uplift_n > 500.0);

        let wet_bark = evaluate_washburn_porous_wetting_and_optics(
            PorousMaterialClass::FibrousBark,
            0.1,
            0.05, // 5 cm above stream waterline -> inside 22 cm capillary fringe
            12.0,
            0.4,
            2.0,
            0.85,
            1.0,
        );
        assert!(wet_bark.pore_water_saturation > 0.75);
        assert!(
            wet_bark.diffuse_albedo_multiplier < 0.65,
            "wet porous bark must physically darken via Lekner-Dorf internal reflection"
        );
        assert!(wet_bark.effective_roughness < 0.35);
    }
}

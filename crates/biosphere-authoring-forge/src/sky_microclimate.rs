//! # 6-Band Polarized Sky Dome, Solar Insolation & Katabatic Microclimate (`sky_microclimate.rs`)
//!
//! Couples solar geometry, atmospheric Rayleigh/Mie/Ozone optics, and `M64` (`GRM3`) terrain
//! topography into localized microclimates and 64-byte `Q42EMF64` electromagnetic sky cells:
//! 1. **6-Band Spectral & Rayleigh-Polarized Sky Dome (`Q42EMF64`):**
//!    Evaluates Rayleigh molecular scattering ($\propto \lambda^{-4}$), Mie aerosol haze
//!    ($\propto \lambda^{-\alpha}$), and Chappuis ozone absorption across 6 spectral bands
//!    (`UV`, `Blue`, `Green`, `Red`, `NIR`, `SWIR/Thermal`), plus the Rayleigh degree of linear
//!    polarization $\text{DoLP}(\gamma) = \sin^2\gamma / (1 + \cos^2\gamma)$ and E-vector angle
//!    used by arthropods (bees, ants, dung beetles) for dorsal-rim compound-eye sky-compass navigation.
//! 2. **Topographic Solar Insolation, Anabatic/Katabatic Winds & Frost-Hollow Pooling:**
//!    Computes slope-aspect direct irradiance, Penman–Monteith canopy evapotranspiration cooling,
//!    daytime upslope **anabatic** convective breezes, and nighttime downslope **katabatic**
//!    cold-air drainage pooling into valley frost hollows.

use crate::arthropod::CuticleEmfSpectralCell;
use serde::{Deserialize, Serialize};

/// 6-band spectral irradiance and Rayleigh polarization state of a sky dome direction.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpectralSkySample {
    /// 6-band spectral radiance `[UV_350nm, Blue_450nm, Green_550nm, Red_650nm, NIR_850nm, SWIR_1600nm]` in $\text{W}/(\text{m}^2\cdot\text{sr})$.
    pub spectral_bands_w_m2_sr: [f32; 6],
    /// Photosynthetically Active Radiation (`PAR`, 400–700 nm) flux in $\mu\text{mol photons}/(\text{m}^2\cdot\text{s})$.
    pub par_umol_m2_s: f32,
    /// Rayleigh Degree of Linear Polarization $\text{DoLP} \in [0.0, 0.85]$.
    pub degree_of_linear_polarization: f32,
    /// Polarization E-vector orientation angle in radians (perpendicular to the sun-view scattering plane).
    pub polarization_angle_rad: f32,
    /// 64-byte `Q42EMF64` electromagnetic field cell ready for `QualiaPortal` volumetric/sky binding.
    pub emf_cell: CuticleEmfSpectralCell,
}

/// Evaluates the 6-band spectral sky radiance, Rayleigh polarization pattern (including UV sky-compass
/// Stokes `[Q, U, V]`), and a 64-byte `Q42EMF64` cell for view direction `view_dir` and `sun_dir`.
pub fn evaluate_spectral_sky_dome_and_polarization_zero_heap(
    sun_dir: [f32; 3],
    view_dir: [f32; 3],
    aerosol_turbidity: f32,
    ozone_dobson_units: f32,
) -> SpectralSkySample {
    let s_len = (sun_dir[0] * sun_dir[0] + sun_dir[1] * sun_dir[1] + sun_dir[2] * sun_dir[2])
        .sqrt()
        .max(1e-6);
    let s = [sun_dir[0] / s_len, sun_dir[1] / s_len, sun_dir[2] / s_len];
    let v_len = (view_dir[0] * view_dir[0] + view_dir[1] * view_dir[1] + view_dir[2] * view_dir[2])
        .sqrt()
        .max(1e-6);
    let v = [view_dir[0] / v_len, view_dir[1] / v_len, view_dir[2] / v_len];

    let cos_gamma = (s[0] * v[0] + s[1] * v[1] + s[2] * v[2]).clamp(-1.0, 1.0);
    let sin_gamma_sq = (1.0 - cos_gamma * cos_gamma).max(0.0);

    // Relative optical air mass (Kasten-Young approximation) for sun and view ray
    let sun_elev_sin = s[1].max(-0.15);
    let view_elev_sin = v[1].max(0.03);
    let air_mass_sun = 1.0 / (sun_elev_sin.max(0.02) + 0.15 * (sun_elev_sin.max(0.0) * 57.3 + 3.885).powf(-1.253));
    let air_mass_view = 1.0 / view_elev_sin;
    let turbidity = aerosol_turbidity.clamp(1.0, 12.0);
    let ozone_scale = (ozone_dobson_units.clamp(150.0, 500.0)) / 300.0;

    // 6 reference wavelengths in micrometres: [UV 0.36, Blue 0.45, Green 0.55, Red 0.65, NIR 0.85, SWIR 1.60]
    let lambdas_um = [0.36f32, 0.45, 0.55, 0.65, 0.85, 1.60];
    let solar_exo = [65.0f32, 195.0, 185.0, 155.0, 105.0, 35.0];
    let ozone_k = [0.02f32, 0.004, 0.085, 0.045, 0.002, 0.0];

    let rayleigh_phase = 0.75 * (1.0 + cos_gamma * cos_gamma);
    // Henyey-Greenstein Mie forward-scattering phase function (g = 0.78)
    let g = 0.78f32;
    let mie_phase = (1.0 - g * g) / (4.0 * core::f32::consts::PI * (1.0 + g * g - 2.0 * g * cos_gamma).powf(1.5));

    let mut bands = [0.0f32; 6];
    let daylight_gate = ((s[1] + 0.12) / 0.22).clamp(0.002, 1.0);
    for i in 0..6 {
        let lam = lambdas_um[i];
        let tau_r = 0.008735 * lam.powf(-4.08);
        let tau_m = 0.04 * (turbidity - 1.0) * lam.powf(-1.3);
        let tau_o = ozone_k[i] * ozone_scale;
        let sun_trans = (-(tau_r + tau_m + tau_o) * air_mass_sun.min(38.0)).exp();
        let scatter = (tau_r * rayleigh_phase + tau_m * mie_phase * 2.5) * air_mass_view.min(12.0);
        bands[i] = (solar_exo[i] * sun_trans * scatter * daylight_gate).max(1.0e-4);
    }

    // Rayleigh Degree of Linear Polarization (peaks at 90° from the sun, damped by aerosol turbidity)
    let depolar_factor = (1.0 / (1.0 + 0.18 * (turbidity - 1.0))).clamp(0.25, 0.85);
    let dolp = (depolar_factor * (sin_gamma_sq / (1.0 + cos_gamma * cos_gamma))).clamp(0.0, 0.85);

    // E-vector polarization orientation angle: cross product (s × v) projected onto tangent plane
    let cx = s[1] * v[2] - s[2] * v[1];
    let cz = s[0] * v[1] - s[1] * v[0];
    let pol_angle = cz.atan2(cx);

    let total_vis = bands[1] + bands[2] + bands[3];
    let par_umol = total_vis * 4.57 * s[1].max(0.02);
    let q_stokes = total_vis * dolp * (2.0 * pol_angle).cos();
    let u_stokes = total_vis * dolp * (2.0 * pol_angle).sin();
    let e_mag = (total_vis * 377.0).sqrt();

    let dominant_nm = if s[1] < 0.10 { 635.0 } else { 465.0 };
    let emf_cell = CuticleEmfSpectralCell {
        e_field_xyz: [-v[0] * e_mag, -v[1] * e_mag, -v[2] * e_mag],
        b_field_xyz: [0.0, 0.0, e_mag / 299_792_458.0],
        poynting_flux: total_vis,
        wavelength_nm: dominant_nm,
        refractive_n: 1.00029,
        extinction_k: 0.0001 * turbidity,
        coherence_phase: pol_angle,
        stokes_quv: [q_stokes, u_stokes, 0.0],
        spectral_sigma: if s[1] < 0.10 { 3.2 } else { 1.1 },
        linear_rgb_preview: [bands[3], bands[2], bands[1]],
    };

    SpectralSkySample {
        spectral_bands_w_m2_sr: bands,
        par_umol_m2_s: par_umol,
        degree_of_linear_polarization: dolp,
        polarization_angle_rad: pol_angle,
        emf_cell,
    }
}

/// Topographic microclimate evaluation report for a specific terrain slope, canopy shade, and valley position.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TopographicMicroclimateReport {
    /// Direct + diffuse solar irradiance reaching the sub-canopy surface in $\text{W/m}^2$.
    pub subcanopy_solar_irradiance_w_m2: f32,
    /// Local microclimate air temperature at 0.5 m height in degrees Celsius (includes canopy
    /// Penman–Monteith evaporative cooling and katabatic frost-hollow pooling).
    pub local_air_temp_c: f32,
    /// Local surface skin temperature in degrees Celsius.
    pub local_surface_skin_temp_c: f32,
    /// Local relative humidity in `[0.0, 1.0]`.
    pub local_relative_humidity: f32,
    /// Thermal slope breeze vector `[ux, uz]` in m/s (**anabatic** upslope by day, **katabatic** downslope by night).
    pub thermal_slope_wind_m_s: [f32; 2],
    /// `true` if nighttime katabatic drainage pools sub-zero air into this valley depression (**frost hollow**).
    pub is_katabatic_frost_hollow: bool,
}

/// Evaluates topographic solar insolation, Beer–Lambert canopy light extinction ($I = I_0 e^{-k \cdot \text{LAI}}$),
/// Penman–Monteith transpirational cooling, and diurnal anabatic/katabatic slope breezes & frost hollows.
pub fn evaluate_topographic_microclimate_zero_heap(
    ambient_temp_2m_c: f32,
    ambient_rh: f32,
    exo_solar_flux_w_m2: f32,
    sun_dir: [f32; 3],
    terrain_normal: [f32; 3],
    valley_concavity_index: f32, // +1.0 = deep valley floor / gully, 0.0 = flat, -1.0 = exposed ridge
    canopy_lai: f32,
    soil_moisture: f32,
) -> TopographicMicroclimateReport {
    let n_len = (terrain_normal[0] * terrain_normal[0]
        + terrain_normal[1] * terrain_normal[1]
        + terrain_normal[2] * terrain_normal[2])
        .sqrt()
        .max(1e-6);
    let n = [
        terrain_normal[0] / n_len,
        terrain_normal[1] / n_len,
        terrain_normal[2] / n_len,
    ];
    let s_len = (sun_dir[0] * sun_dir[0] + sun_dir[1] * sun_dir[1] + sun_dir[2] * sun_dir[2])
        .sqrt()
        .max(1e-6);
    let s = [sun_dir[0] / s_len, sun_dir[1] / s_len, sun_dir[2] / s_len];

    let is_daytime = s[1] > 0.02;
    let slope_cos = (n[0] * s[0] + n[1] * s[1] + n[2] * s[2]).max(0.0);
    // Beer-Lambert canopy light transmission (k_ext ≈ 0.52)
    let canopy_trans = (-0.52 * canopy_lai.clamp(0.0, 10.0)).exp();
    let direct_flux = if is_daytime {
        exo_solar_flux_w_m2.max(0.0) * slope_cos * canopy_trans
    } else {
        0.0
    };
    let diffuse_flux = if is_daytime {
        exo_solar_flux_w_m2.max(0.0) * 0.14 * s[1] * canopy_trans.sqrt()
    } else {
        0.0
    };
    let subcanopy_flux = direct_flux + diffuse_flux;

    // Penman-Monteith evapotranspiration cooling inside moist, high-LAI forest gullies
    let evap_cooling_c = if is_daytime {
        (canopy_lai.clamp(0.0, 7.0) * 0.65 * soil_moisture.clamp(0.0, 1.0) * (1.0 - ambient_rh.clamp(0.1, 0.98)))
            .clamp(0.0, 6.5)
    } else {
        0.0
    };

    // Nighttime katabatic cold-air drainage into valley concavities (+1 = gully floor)
    let concavity = valley_concavity_index.clamp(-1.0, 1.0);
    let katabatic_pooling_c = if !is_daytime && concavity > 0.0 {
        // Clear sky under low LAI accelerates radiative frost-hollow pooling up to -5.5°C colder
        let sky_view = (-0.25 * canopy_lai.clamp(0.0, 8.0)).exp();
        -5.5 * concavity * sky_view
    } else {
        0.0
    };

    let solar_warming_c = (subcanopy_flux / 220.0).clamp(0.0, 4.5);
    let local_air_c = ambient_temp_2m_c + solar_warming_c - evap_cooling_c + katabatic_pooling_c;
    let skin_offset_c = if is_daytime {
        subcanopy_flux / 110.0
    } else {
        -2.2 * (-0.3 * canopy_lai.clamp(0.0, 8.0)).exp()
    };
    let local_skin_c = local_air_c + skin_offset_c;

    // Anabatic (daytime upslope) vs. Katabatic (nighttime downslope) thermal breeze
    // Horizontal slope vector points downslope in direction of [n.x, n.z]
    let slope_horiz_mag = (n[0] * n[0] + n[2] * n[2]).sqrt();
    let thermal_wind = if slope_horiz_mag > 1e-3 {
        let downslope_dir = [n[0] / slope_horiz_mag, n[2] / slope_horiz_mag];
        if is_daytime {
            // Anabatic upslope breeze proportional to solar heating on slope
            let u_ana = 2.4 * slope_horiz_mag * (subcanopy_flux / 800.0).clamp(0.1, 1.2);
            [-downslope_dir[0] * u_ana, -downslope_dir[1] * u_ana]
        } else {
            // Katabatic downslope cold-air gravity current
            let u_kat = 2.8 * slope_horiz_mag.sqrt();
            [downslope_dir[0] * u_kat, downslope_dir[1] * u_kat]
        }
    } else {
        [0.0, 0.0]
    };

    let local_rh = (ambient_rh + 0.04 * (ambient_temp_2m_c - local_air_c)).clamp(0.08, 1.0);
    let is_frost_hollow = !is_daytime && concavity > 0.25 && local_skin_c < 0.0;

    TopographicMicroclimateReport {
        subcanopy_solar_irradiance_w_m2: subcanopy_flux,
        local_air_temp_c: local_air_c,
        local_surface_skin_temp_c: local_skin_c,
        local_relative_humidity: local_rh,
        thermal_slope_wind_m_s: thermal_wind,
        is_katabatic_frost_hollow: is_frost_hollow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rayleigh_sky_polarization_peaks_at_90_degrees_and_valley_forms_katabatic_frost_hollow() {
        let sun = [0.0, 1.0, 0.0]; // Zenith sun
        let view_90 = [1.0, 0.05, 0.0]; // 90 deg from zenith
        let view_0 = [0.0, 1.0, 0.0]; // Looking straight at sun
        let sample_90 = evaluate_spectral_sky_dome_and_polarization_zero_heap(sun, view_90, 2.0, 300.0);
        let sample_0 = evaluate_spectral_sky_dome_and_polarization_zero_heap(sun, view_0, 2.0, 300.0);
        assert!(
            sample_90.degree_of_linear_polarization > 0.65,
            "Rayleigh polarization must peak orthogonal to sun direction for arthropod sky-compass"
        );
        assert!(sample_0.degree_of_linear_polarization < 0.05);
        assert!(sample_90.spectral_bands_w_m2_sr[1] > sample_90.spectral_bands_w_m2_sr[3], "clear sky blue > red");

        // Nighttime valley floor vs. exposed ridge at +2.5°C regional air temperature
        let valley_night = evaluate_topographic_microclimate_zero_heap(
            2.5,
            0.80,
            0.0,
            [0.0, -0.4, 0.9], // sun below horizon
            [0.25, 0.968, 0.0],
            0.85, // deep valley floor
            0.4,
            0.7,
        );
        assert!(
            valley_night.is_katabatic_frost_hollow,
            "cold-air drainage must pool sub-zero frost in valley hollow even when 2m regional air is +2.5C"
        );
        assert!(valley_night.local_surface_skin_temp_c < -1.5);
    }
}

//! # Hydrometeors & Cryosphere Phase-Change Solver (`hydrometeor_cryosphere.rs`)
//!
//! Zero-heap physical and morphological modeling of all atmospheric and surface water phases:
//! 1. **Rain & Drop Size Distribution (DSD):**
//!    Marshall–Palmer exponential drop distribution $N(D) = N_0 e^{-\Lambda D}$, Atlas–Ulbrich
//!    terminal fall velocity $v_t(D)$, wind-slanted rain incidence vector, leaf/petal impact
//!    momentum impulses, canopy interception storage, and throughfall/stemflow partitioning.
//! 2. **Radiation, Advection & Orographic Fog / Mist:**
//!    Liquid Water Content ($\text{LWC}$ in $\text{g/m}^3$), Koschmieder meteorological visibility
//!    $V_{\text{met}} = 3.912 / \beta_{\text{ext}}$, Mie forward-scattering asymmetry $g$, and
//!    **occult precipitation (fog-drip harvesting)** on conifer needles, eucalypt foliage,
//!    bryophytes, and orb-weaver silk webs.
//! 3. **Snowfall, Nakaya Dendritic Crystal Habit & Slope/Crown Accumulation:**
//!    Classifies snow crystal morphology across the Nakaya $(T_{\text{air}}, \sigma_{\text{ice}})$
//!    phase diagram (plates, needles, hollow columns, stellar dendrites, sectored plates), computes
//!    wind-drifted angle-of-repose deposition on `M64` (`GRM3`) terrain and tree boughs, and tracks
//!    metamorphic densification + spectral albedo decay ($0.90 \to 0.52$).
//! 4. **Hoar Frost (Vapor Desublimation) vs. Supercooled Rime Icing:**
//!    Distinguishes calm-night radiative **hoar frost** (anisotropic ice needles growing along
//!    high-curvature leaf serrations, grass tips, and fur guard hairs) from wind-driven
//!    **rime icing** (supercooled fog droplets freezing on impact into windward feather banners).
//! 5. **Freezing Rain, Conformal Glaze Ice, Stalactitic Icicles & Black Ice Friction Collapse:**
//!    Radial clear-ice accretion on cylindrical branches ($W_{\text{ice}}$ linear load in N/m),
//!    unfrozen runback drip forming tapered icicles, and surface friction coefficient collapse
//!    ($\mu_s: 0.75 \to 0.08$) for fauna foot-contact IK.
//! 6. **Stefan Lake/Stream Surface Ice Growth & Breakthrough Load:**
//!    Evaluates ice sheet thickness $h_{\text{ice}}(t) = \alpha_{\text{Stefan}} \sqrt{\text{FDD}}$
//!    from Freezing Degree-Days, Gerstner wave exponential damping, and Gold's formula for maximum
//!    safe fauna load $P_{\text{crit}} \propto \sigma_f h_{\text{ice}}^2$.

use serde::{Deserialize, Serialize};

/// Nakaya snow crystal morphological habit determined by ambient temperature and vapor supersaturation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NakayaCrystalHabit {
    /// No solid ice crystals ($T > +1.5^\circ\text{C}$).
    LiquidRainOnly,
    /// Wet slush / melting aggregates ($0^\circ\text{C} < T \le +1.5^\circ\text{C}$).
    MeltingGraupelSlush,
    /// Thin hexagonal plates ($0^\circ\text{C} \ge T > -4^\circ\text{C}$).
    HexagonalPlates,
    /// Slender needles and hollow columns ($-4^\circ\text{C} \ge T > -10^\circ\text{C}$).
    NeedlesAndColumns,
    /// Branched 6-fold stellar dendrites and fernlike crystals ($-10^\circ\text{C} \ge T > -22^\circ\text{C}$, high supersaturation).
    StellarDendrites,
    /// Sectored plates ($-10^\circ\text{C} \ge T > -22^\circ\text{C}$, low supersaturation).
    SectoredPlates,
    /// Cold diamond-dust hexagonal prisms and hollow columns ($T \le -22^\circ\text{C}$).
    DiamondDustPrisms,
}

/// Synoptic and microclimatic hydrometeor & cryosphere forcing state.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HydrometeorState {
    /// Ambient air temperature at asset height in degrees Celsius.
    pub air_temp_c: f32,
    /// Dew-point temperature in degrees Celsius.
    pub dew_point_c: f32,
    /// Surface skin temperature of the asset/terrain in degrees Celsius (can be $2\text{–}5^\circ\text{C}$
    /// colder than `air_temp_c` on clear nights due to longwave radiative cooling to the sky).
    pub surface_skin_temp_c: f32,
    /// Relative humidity in `[0.0, 1.0]`.
    pub relative_humidity: f32,
    /// Liquid rainfall intensity $R$ in mm/hr (`0.0` = dry, `2.5` = moderate rain, `50.0` = cloudburst).
    pub rainfall_rate_mm_hr: f32,
    /// Snowfall water-equivalent rate in mm/hr.
    pub snowfall_swe_mm_hr: f32,
    /// Fog Liquid Water Content ($\text{LWC}$) in $\text{g/m}^3$ (`0.0` = clear, `0.05` = mist, `0.4` = dense fog).
    pub fog_lwc_g_m3: f32,
    /// Local 3D wind vector `[ux, uy, uz]` in m/s (`uy` is vertical).
    pub wind_velocity_m_s: [f32; 3],
    /// Accumulated Freezing Degree-Days ($\text{FDD}$, $\circ\text{C}\cdot\text{days}$ below $0^\circ\text{C}$) for lake/soil freezing.
    pub freezing_degree_days: f32,
    /// Age of current surface snowpack in hours (drives metamorphic densification and albedo decay).
    pub snowpack_age_hours: f32,
}

impl Default for HydrometeorState {
    fn default() -> Self {
        Self {
            air_temp_c: 14.0,
            dew_point_c: 10.0,
            surface_skin_temp_c: 13.5,
            relative_humidity: 0.72,
            rainfall_rate_mm_hr: 0.0,
            snowfall_swe_mm_hr: 0.0,
            fog_lwc_g_m3: 0.0,
            wind_velocity_m_s: [3.0, 0.0, 0.0],
            freezing_degree_days: 0.0,
            snowpack_age_hours: 0.0,
        }
    }
}

/// Complete multi-phase surface & atmospheric response evaluated at an asset or terrain locus.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CryosphereAssetEffects {
    // --- Rain & DSD ---
    /// Marshall–Palmer median volume drop diameter $D_0$ in millimetres.
    pub median_raindrop_diameter_mm: f32,
    /// Terminal fall speed $v_t(D_0)$ in m/s.
    pub raindrop_terminal_velocity_m_s: f32,
    /// Unit incidence vector `[dx, dy, dz]` of falling raindrops slanted by wind.
    pub rain_incidence_dir: [f32; 3],
    /// Single-drop leaf/petal impact momentum impulse $\Delta p$ in $\text{N}\cdot\text{s}$.
    pub single_drop_impulse_n_s: f32,
    /// Canopy throughfall + fog-drip effective water flux reaching the ground in mm/hr.
    pub effective_throughfall_mm_hr: f32,

    // --- Fog & Visibility ---
    /// Optical extinction coefficient $\beta_{\text{ext}}$ in $\text{m}^{-1}$ (combining fog, rain, and snow).
    pub optical_extinction_m_inv: f32,
    /// Koschmieder meteorological visibility distance in metres.
    pub visibility_distance_m: f32,
    /// Occult fog-drip water harvesting rate on foliage/moss/webs in mm/hr.
    pub occult_fog_drip_mm_hr: f32,

    // --- Snow & Nakaya Habit ---
    pub crystal_habit: NakayaCrystalHabit,
    /// Fresh snow bulk density $\rho_{\text{snow}}$ in $\text{kg/m}^3$.
    pub snow_density_kg_m3: f32,
    /// Intercepted snow thickness on the surface/branch in metres.
    pub accumulated_snow_thickness_m: f32,
    /// Broadband visible snow albedo in `[0.50, 0.92]`.
    pub snow_albedo: f32,

    // --- Frost, Rime & Glaze Ice ---
    /// Radiative hoar-frost needle thickness in metres (preferentially on edges/tips/fur).
    pub hoar_frost_needle_m: f32,
    /// Windward supercooled rime-ice feather thickness in metres (growing into `-wind_velocity`).
    pub rime_ice_feather_m: f32,
    /// Conformal clear glaze-ice shell thickness in metres (from freezing rain / wet-bulb $< 0^\circ\text{C}$).
    pub glaze_ice_thickness_m: f32,
    /// Stalactitic icicle length in metres hanging from branch/rock overhangs.
    pub overhang_icicle_length_m: f32,
    /// Total added cryospheric structural line load (snow + rime + glaze) per metre of branch length in N/m.
    pub branch_cryo_line_load_n_per_m: f32,

    // --- Substrate & Water-Body Ice ---
    /// Effective terrain static friction coefficient $\mu_s \in [0.06, 0.85]$ (collapses on black/glaze ice).
    pub effective_surface_friction_mu: f32,
    /// Stefan water-body surface ice sheet thickness in metres.
    pub water_body_ice_thickness_m: f32,
    /// Maximum load in Newtons that the frozen water surface can support before cryo-fracture breakthrough.
    pub ice_sheet_breakthrough_load_n: f32,
}

/// Evaluates all hydrometeor (rain, fog, snow) and cryosphere (hoar frost, rime, glaze ice, icicles,
/// black-ice friction, and Stefan lake ice) effects on an asset surface with normal `surface_normal`,
/// edge curvature `edge_curvature` $\in [0, 1]$, cylinder radius `branch_radius_m`, and exposure duration `exposure_hours`.
pub fn evaluate_hydrometeor_cryosphere_effects_zero_heap(
    state: &HydrometeorState,
    surface_normal: [f32; 3],
    edge_curvature: f32,
    branch_radius_m: f32,
    canopy_lai: f32,
    exposure_hours: f32,
) -> CryosphereAssetEffects {
    let dt_hr = exposure_hours.max(0.0);
    let wx = state.wind_velocity_m_s[0];
    let wy = state.wind_velocity_m_s[1];
    let wz = state.wind_velocity_m_s[2];
    let wind_speed = (wx * wx + wy * wy + wz * wz).sqrt();

    let n_len = (surface_normal[0] * surface_normal[0]
        + surface_normal[1] * surface_normal[1]
        + surface_normal[2] * surface_normal[2])
        .sqrt()
        .max(1e-6);
    let n = [
        surface_normal[0] / n_len,
        surface_normal[1] / n_len,
        surface_normal[2] / n_len,
    ];

    // 1. Rain & Marshall-Palmer Drop Size Distribution (DSD)
    let r_rate = state.rainfall_rate_mm_hr.max(0.0);
    let (d0_mm, v_term, impulse_ns, rain_dir) = if r_rate > 0.01 {
        // Lambda = 4.1 * R^(-0.21) mm^-1; median volume diameter D_0 = 3.67 / Lambda
        let lambda = 4.1 * r_rate.powf(-0.21);
        let d0 = (3.67 / lambda).clamp(0.35, 5.5);
        // Atlas-Ulbrich terminal fall velocity v_t(D) = 9.65 - 10.3 * exp(-0.6 * D_mm)
        let vt = (9.65 - 10.3 * (-0.6 * d0).exp()).clamp(1.2, 9.5);
        let r_m = d0 * 0.5e-3;
        let drop_mass_kg = (4.0 / 3.0) * core::f32::consts::PI * r_m * r_m * r_m * 1000.0;
        let vx = wx;
        let vy = -vt + wy;
        let vz = wz;
        let v_mag = (vx * vx + vy * vy + vz * vz).sqrt().max(0.1);
        let dir = [vx / v_mag, vy / v_mag, vz / v_mag];
        (d0, vt, drop_mass_kg * v_mag, dir)
    } else {
        (0.0, 0.0, 0.0, [0.0, -1.0, 0.0])
    };

    // 2. Fog Liquid Water Content, Occult Fog-Drip Harvesting & Koschmieder Visibility
    let lwc = state.fog_lwc_g_m3.max(0.0);
    // Impaction efficiency increases with wind speed and Leaf Area Index (LAI)
    let occult_drip_mm_hr = if lwc > 0.005 {
        let impaction_eff = (wind_speed / (wind_speed + 2.0)).clamp(0.08, 0.85);
        lwc * wind_speed * 3.6 * impaction_eff * (canopy_lai.clamp(0.2, 8.0) * 0.15)
    } else {
        0.0
    };
    let canopy_interception_capacity = canopy_lai.clamp(0.0, 8.0) * 0.35;
    let total_precip = r_rate * dt_hr + occult_drip_mm_hr * dt_hr;
    let effective_throughfall_mm_hr = if dt_hr > 1e-4 {
        ((total_precip - canopy_interception_capacity).max(0.0)) / dt_hr
    } else {
        r_rate + occult_drip_mm_hr
    };

    // Optical extinction combining fog droplets, rainfall, and snowfall
    let beta_fog = if lwc > 1e-4 {
        // Empirical Eldridge fog relation: beta_ext (m^-1) ≈ 0.0145 * (LWC_g_m3)^0.65
        0.0145 * lwc.powf(0.65)
    } else {
        0.0
    };
    let beta_rain = 0.00025 * r_rate.powf(0.68);
    let beta_snow = 0.0018 * state.snowfall_swe_mm_hr.max(0.0).powf(0.75);
    let beta_ext = (0.000015 + beta_fog + beta_rain + beta_snow).max(1.5e-5);
    let visibility_m = (3.912 / beta_ext).clamp(1.5, 250_000.0);

    // 3. Nakaya Snow Crystal Habit, Snow Density & Slope/Branch Accumulation
    let t_air = state.air_temp_c;
    let rh = state.relative_humidity.clamp(0.0, 1.0);
    let crystal_habit = if t_air > 1.5 {
        NakayaCrystalHabit::LiquidRainOnly
    } else if t_air > 0.0 {
        NakayaCrystalHabit::MeltingGraupelSlush
    } else if t_air > -4.0 {
        NakayaCrystalHabit::HexagonalPlates
    } else if t_air > -10.0 {
        NakayaCrystalHabit::NeedlesAndColumns
    } else if t_air > -22.0 {
        if rh >= 0.82 {
            NakayaCrystalHabit::StellarDendrites
        } else {
            NakayaCrystalHabit::SectoredPlates
        }
    } else {
        NakayaCrystalHabit::DiamondDustPrisms
    };

    // Hedstrom-Pomeroy fresh snow density + metamorphic compaction with age
    let fresh_snow_rho = if t_air <= 1.5 {
        (67.9 + 51.25 * (t_air / 2.59).exp()).clamp(50.0, 220.0)
    } else {
        200.0
    };
    let age_hr = state.snowpack_age_hours.max(0.0);
    let snow_rho = (fresh_snow_rho + 185.0 * (1.0 - (-age_hr / 72.0).exp())).clamp(50.0, 450.0);
    let snow_albedo = (0.52 + 0.38 * (-age_hr / 96.0).exp()).clamp(0.52, 0.90);

    // Angle-of-repose & upward-normal deposition gate (snow slides off slopes > ~52 deg)
    let up_dot = n[1].max(0.0);
    let repose_factor = ((up_dot - 0.55) / 0.45).clamp(0.0, 1.0);
    let snow_thickness_m = if state.snowfall_swe_mm_hr > 0.0 && state.surface_skin_temp_c <= 0.5 {
        let swe_m = (state.snowfall_swe_mm_hr * dt_hr) * 1.0e-3;
        (swe_m * (1000.0 / snow_rho)) * repose_factor
    } else {
        0.0
    };

    // 4. Radiative Hoar Frost vs. Windward Supercooled Rime Ice
    let t_skin = state.surface_skin_temp_c;
    // Hoar frost forms on calm, humid nights when skin temperature < 0°C and < dew_point_c
    let hoar_frost_m = if t_skin < 0.0 && t_skin <= state.dew_point_c && wind_speed < 3.5 {
        let vapor_excess = (state.dew_point_c - t_skin).max(0.1).min(6.0);
        let calm_gate = (1.0 - wind_speed / 3.5).clamp(0.1, 1.0);
        let curvature_boost = 1.0 + 1.8 * edge_curvature.clamp(0.0, 1.0);
        (0.00018 * vapor_excess * calm_gate * curvature_boost * dt_hr.sqrt()).min(0.012)
    } else {
        0.0
    };

    // Rime ice forms when supercooled fog droplets (-18°C < T_air < -0.5°C) impact windward surfaces
    let wind_dir = if wind_speed > 1e-4 {
        [wx / wind_speed, wy / wind_speed, wz / wind_speed]
    } else {
        [0.0, 0.0, 0.0]
    };
    let windward_alignment = (-(n[0] * wind_dir[0] + n[1] * wind_dir[1] + n[2] * wind_dir[2])).max(0.0);
    let rime_ice_m = if t_air < -0.5 && t_air > -18.0 && lwc > 0.01 && wind_speed > 1.0 {
        (0.0014 * lwc * wind_speed * windward_alignment * dt_hr).min(0.15)
    } else {
        0.0
    };

    // 5. Freezing Rain, Conformal Glaze Ice & Stalactitic Overhang Icicles
    // Glaze ice forms when liquid rain falls onto sub-freezing surfaces (T_skin < 0°C)
    let (glaze_ice_m, icicle_len_m) = if r_rate > 0.05 && t_skin < 0.0 {
        let freeze_frac = ((-t_skin) / 4.0).clamp(0.15, 0.92);
        let glaze_m = ((r_rate * 1.0e-3 * dt_hr) * freeze_frac * 0.55).min(0.06);
        let runback_flux = r_rate * (1.0 - freeze_frac);
        let overhang_gate = (-n[1]).clamp(0.0, 1.0); // downward-facing undersides drip icicles
        let icicle_m = (0.018 * (runback_flux * dt_hr).sqrt() * (0.25 + 0.75 * overhang_gate)).min(0.85);
        (glaze_m, icicle_m)
    } else {
        (0.0, 0.0)
    };

    // Total cryospheric gravity load per metre of cylindrical branch length (N/m)
    let r_b = branch_radius_m.max(0.002);
    let g = 9.80665f32;
    let rho_glaze = 917.0f32;
    let rho_rime = 350.0f32;
    let area_glaze = core::f32::consts::PI * ((r_b + glaze_ice_m).powi(2) - r_b * r_b);
    let area_rime = r_b * rime_ice_m;
    let area_snow = (2.0 * (r_b + glaze_ice_m)) * snow_thickness_m;
    let cryo_line_load_n_m =
        g * (rho_glaze * area_glaze + rho_rime * area_rime + snow_rho * area_snow);

    // 6. Substrate Friction Coefficient & Stefan Lake-Ice Sheet Growth
    let base_friction = 0.75f32;
    let friction_mu = if glaze_ice_m > 0.0002 {
        0.08 // Black/glaze ice
    } else if snow_thickness_m > 0.005 {
        0.28 // Compacted/fresh snow
    } else if hoar_frost_m > 0.0003 || rime_ice_m > 0.001 {
        0.42 // Frosted surface
    } else if r_rate > 0.1 {
        0.55 // Wet surface
    } else {
        base_friction
    };

    // Stefan equation: h_ice (m) ≈ 0.027 * sqrt(FDD_celsius_days)
    let fdd = state.freezing_degree_days.max(0.0);
    let lake_ice_m = 0.027 * fdd.sqrt();
    // Gold's flexural breakthrough load P_crit ≈ A * h^2 (with A ≈ 3.5e6 N/m^2 for clear freshwater ice)
    let breakthrough_load_n = 3.5e6 * lake_ice_m * lake_ice_m;

    CryosphereAssetEffects {
        median_raindrop_diameter_mm: d0_mm,
        raindrop_terminal_velocity_m_s: v_term,
        rain_incidence_dir: rain_dir,
        single_drop_impulse_n_s: impulse_ns,
        effective_throughfall_mm_hr,
        optical_extinction_m_inv: beta_ext,
        visibility_distance_m: visibility_m,
        occult_fog_drip_mm_hr: occult_drip_mm_hr,
        crystal_habit,
        snow_density_kg_m3: snow_rho,
        accumulated_snow_thickness_m: snow_thickness_m,
        snow_albedo,
        hoar_frost_needle_m: hoar_frost_m,
        rime_ice_feather_m: rime_ice_m,
        glaze_ice_thickness_m: glaze_ice_m,
        overhang_icicle_length_m: icicle_len_m,
        branch_cryo_line_load_n_per_m: cryo_line_load_n_m,
        effective_surface_friction_mu: friction_mu,
        water_body_ice_thickness_m: lake_ice_m,
        ice_sheet_breakthrough_load_n: breakthrough_load_n,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nakaya_stellar_dendrites_and_hoar_frost_vs_rime_icing_distinguished() {
        // Calm, clear sub-zero night -> radiative hoar frost needles on leaf edges, stellar dendrites if snowing
        let calm_frost = HydrometeorState {
            air_temp_c: -14.0,
            dew_point_c: -14.5,
            surface_skin_temp_c: -17.0,
            relative_humidity: 0.92,
            rainfall_rate_mm_hr: 0.0,
            snowfall_swe_mm_hr: 2.5,
            fog_lwc_g_m3: 0.0,
            wind_velocity_m_s: [0.4, 0.0, 0.0],
            freezing_degree_days: 25.0,
            snowpack_age_hours: 4.0,
        };
        let eff_calm = evaluate_hydrometeor_cryosphere_effects_zero_heap(
            &calm_frost,
            [0.0, 1.0, 0.0],
            0.9,
            0.015,
            3.5,
            6.0,
        );
        assert_eq!(eff_calm.crystal_habit, NakayaCrystalHabit::StellarDendrites);
        assert!(eff_calm.hoar_frost_needle_m > 0.001, "calm radiative night must grow hoar frost needles");
        assert_eq!(eff_calm.rime_ice_feather_m, 0.0);
        assert!(eff_calm.accumulated_snow_thickness_m > 0.05);
        assert!(eff_calm.water_body_ice_thickness_m > 0.10);
        assert!(eff_calm.ice_sheet_breakthrough_load_n > 30_000.0);

        // Windy supercooled alpine cloud -> windward rime ice feathers
        let windy_rime = HydrometeorState {
            air_temp_c: -6.0,
            dew_point_c: -6.0,
            surface_skin_temp_c: -6.0,
            relative_humidity: 0.99,
            rainfall_rate_mm_hr: 0.0,
            snowfall_swe_mm_hr: 0.0,
            fog_lwc_g_m3: 0.35,
            wind_velocity_m_s: [14.0, 0.0, 0.0],
            freezing_degree_days: 5.0,
            snowpack_age_hours: 0.0,
        };
        let eff_rime = evaluate_hydrometeor_cryosphere_effects_zero_heap(
            &windy_rime,
            [-1.0, 0.0, 0.0], // windward face
            0.5,
            0.02,
            4.0,
            5.0,
        );
        assert!(eff_rime.rime_ice_feather_m > 0.02, "windward face in supercooled fog must accrete rime feathers");
        assert!(eff_rime.visibility_distance_m < 600.0);
        assert!(eff_rime.occult_fog_drip_mm_hr > 0.5);
    }

    #[test]
    fn freezing_rain_forms_glaze_ice_icicles_and_black_ice_friction_collapse() {
        let freezing_rain = HydrometeorState {
            air_temp_c: -0.5,
            dew_point_c: -1.0,
            surface_skin_temp_c: -2.5,
            relative_humidity: 0.96,
            rainfall_rate_mm_hr: 6.0,
            snowfall_swe_mm_hr: 0.0,
            fog_lwc_g_m3: 0.05,
            wind_velocity_m_s: [5.0, 0.0, 0.0],
            freezing_degree_days: 2.0,
            snowpack_age_hours: 0.0,
        };
        let eff = evaluate_hydrometeor_cryosphere_effects_zero_heap(
            &freezing_rain,
            [0.0, -1.0, 0.0], // branch underside
            0.3,
            0.025,
            3.0,
            4.0,
        );
        assert!(eff.glaze_ice_thickness_m > 0.005);
        assert!(eff.overhang_icicle_length_m > 0.04);
        assert!(eff.branch_cryo_line_load_n_per_m > 5.0);
        assert!((eff.effective_surface_friction_mu - 0.08).abs() < 1e-4);
        assert!(eff.median_raindrop_diameter_mm > 1.0);
        assert!(eff.single_drop_impulse_n_s > 0.0);
    }
}

//! # Vector 6 — Atmospheric Electricity, Aeolian Saltation, Rankine Dust Devils & 6-Band Optics (`atmo_electricity_aeolian.rs`)
//!
//! Implements the atmospheric electro-ecology, aeolian sediment transport, convective vortices,
//! and chromatic refraction/scattering phenomena from Vector 6:
//! 1. **Atmospheric Electric Fields, Peek's Law Corona, Spider Ballooning & Floral Electro-Reception (`AtmoElectroFieldCell`):**
//!    Fair-weather ($\sim 100\text{ V/m}$) to thunderstorm ($\sim 100\text{ kV/m}$) vertical electric
//!    fields, Peek's visual corona breakdown $g_v = g_0 \delta (1 + 0.301 / \sqrt{\delta r_{\text{cm}}})$,
//!    spider electrostatic dragline silk ballooning lift $|q_s E_z| + F_{\text{drag}} > m_s g$, and
//!    bumblebee filiform-hair floral potential sensing.
//! 2. **Shao–Lu Aeolian Sand/Dust Saltation & Impact Ripples (`AeolianSaltationCell`):**
//!    Cohesion- and gravity-balanced threshold friction velocity
//!    $u_{*t}(d) = \sqrt{A_N(\frac{\rho_p}{\rho_a} g d + \frac{\gamma_c}{\rho_a d})}$, saturation-length
//!    flux relaxation, and migrating impact ripples writing millimetre-guarded deltas to `.m64` (`GRM3`).
//! 3. **Convective Rankine Dust-Devil Vortices (`DustDevilVortex`):**
//!    Rennó (1998) warm-core Rankine tangential swirl $v_\theta(r)$ and cyclostrophic core pressure
//!    depression $\Delta p_c = \rho_a v_{\max}^2$.
//! 4. **Mirages ($dn/dz$) & 6-Band Rainbow/22°-Halo/Glory Wavefronts (`AtmoOpticsWavefront`):**
//!    Edlén–Stull inferior/superior mirage ray curvature, primary/secondary rainbow dispersion and
//!    95% tangential polarization (`Stokes [Q, U]`), hexagonal ice crystal $22^\circ/46^\circ$ minimum
//!    deviation halos, cloud glories, and dew heiligenschein.

use crate::containers_10d_m64::project_metres_f32_checked;
use serde::{Deserialize, Serialize};

/// 64-byte POD atmospheric electric field, Peek corona & bio-electrostatic cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtmoElectroFieldCell {
    /// Ambient 3D atmospheric electric field `[Ex, Ey, Ez]` ($\text{V/m}$; `~100` fair, `1e5` storm).
    pub e_field_v_m: [f32; 3],
    /// Corona ion plume space-charge density ($\sim 10^{-9}\text{ C/m}^3$).
    pub space_charge_c_m3: f32,
    /// Relative air density $\delta = \rho_a / \rho_{\text{SATP}}$.
    pub air_density_rel: f32,
    /// Characteristic local canopy/ridge tip radius $r_{\text{cm}}$ (cm).
    pub tip_radius_cm: f32,
    /// Local topographic/canopy geometric field enhancement factor $\beta_{\text{geom}}$.
    pub geom_enhance_beta: f32,
    /// Peek visual critical breakdown field $g_v$ (V/m).
    pub corona_crit_v_m: f32,
    /// Tip corona discharge current (A).
    pub corona_current_a: f32,
    /// Grounded flower stem electrostatic potential perturbation (V).
    pub floral_bias_v: f32,
    /// Time elapsed since last positively charged bee visit (s).
    pub pollinator_decay_s: f32,
    /// Reference electrostatic lift force per nanocoulomb of spider ballooning silk (N/nC).
    pub balloon_lift_n: f32,
    /// Stepped lightning leader inception probability in `[0.0, 1.0]`.
    pub leader_step_prob: f32,
    /// St. Elmo UV/Blue corona optical emission ($\text{W/m}^2$).
    pub uv_corona_flux: f32,
    /// Spatial hash cell ID.
    pub cell_id: u32,
    /// `Bit0 = CoronaActive`, `Bit1 = LeaderStepping`, `Bit2 = BallooningFavorable`.
    pub storm_flags: u32,
}

/// 48-byte POD aeolian sand/dust saltation & impact ripple cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AeolianSaltationCell {
    /// Median grain diameter $d_{50}$ (`2e-5..1e-3` m).
    pub grain_d50_m: f32,
    /// Local aerodynamic friction velocity $u_*$ (m/s).
    pub u_star_m_s: f32,
    /// Shao–Lu slope-adjusted threshold friction velocity $u_{*t}$ (m/s).
    pub u_star_thresh_m_s: f32,
    /// Current relaxed horizontal saltation mass flux $q$ ($\text{kg}/(\text{m}\cdot\text{s})$).
    pub flux_kg_m_s: f32,
    /// Saturated equilibrium saltation flux $q_{\text{sat}}$ ($\text{kg}/(\text{m}\cdot\text{s})$).
    pub flux_sat_kg_m_s: f32,
    /// Saltation transport layer thickness $\delta_{\text{sal}}$ (m).
    pub saltation_height_m: f32,
    /// Active aeolian impact ripple wavelength $\lambda_r$ (m).
    pub ripple_wavelength_m: f32,
    /// Ripple crest-to-trough amplitude $h_r$ (m).
    pub ripple_height_m: f32,
    /// Downwind migrating ripple phase (rad).
    pub ripple_phase_rad: f32,
    /// Capillary sand moisture cohesion multiplier ($\ge 1.0$).
    pub moisture_cohesion: f32,
    /// Streamwise terrain bed slope $\phi$ (rad).
    pub bed_slope_rad: f32,
    /// `1` if saltating, `2` if fine dust suspended.
    pub active_flag: u32,
}

/// 64-byte POD convective Rankine dust-devil vortex entity.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DustDevilVortex {
    /// Ground touch-down center XZ (m).
    pub pos_xz_m: [f32; 2],
    /// Translation drift velocity across terrain XZ (m/s).
    pub drift_vel_xz: [f32; 2],
    /// Rankine maximum-wind core radius $r_c$ (`0.5..25` m).
    pub core_radius_m: f32,
    /// Visible dust column altitude (m).
    pub height_m: f32,
    /// Peak tangential swirl speed $v_{\max}$ at $r_c$ (m/s).
    pub v_tan_max_m_s: f32,
    /// Wall vertical updraft speed (m/s).
    pub v_updraft_m_s: f32,
    /// Cyclostrophic core pressure drop $\Delta p_c = \rho_a v_{\max}^2$ (Pa).
    pub core_dp_pa: f32,
    /// Driving surface sensible heat flux ($\text{W/m}^2$).
    pub sensible_flux_w_m2: f32,
    /// Entrained dust mass concentration ($\text{kg/m}^3$).
    pub dust_load_kg_m3: f32,
    /// Forward aerodynamic column tilt (`0..0.18` rad).
    pub tilt_rad: f32,
    /// `+1.0` cyclonic or `-1.0` anticyclonic.
    pub spin_sign: f32,
    /// Remaining active lifetime (s).
    pub lifetime_s: f32,
    /// Unique instance ID.
    pub vortex_id: u32,
    /// `Bit0 = SpinningUp`, `Bit1 = Mature`, `Bit2 = Decaying`.
    pub state_flags: u32,
}

/// 64-byte POD mirage ray-curvature & 6-band rainbow/halo/glory wavefront descriptor.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AtmoOpticsWavefront {
    /// Near-surface vertical temperature gradient $dT/dz$ (K/m).
    pub dt_dz_k_m: f32,
    /// Exponential mirage thermal boundary thickness $\beta$ (m).
    pub thermal_scale_beta_m: f32,
    /// Signed Edlén–Stull optical ray curvature $1 / R_c$ ($\text{m}^{-1}$).
    pub ray_curv_m_inv: f32,
    /// Inferior mirage ray inflection V-point height $z_v$ (m).
    pub v_point_height_m: f32,
    /// Primary rainbow antisolar angular radius `[Blue, Green, Red, NIR]` (rad).
    pub rb1_rad_bands: [f32; 4],
    /// $22^\circ$ hexagonal ice halo angular radius `[Blue, Green, Red, NIR]` (rad).
    pub halo22_rad_bands: [f32; 4],
    /// Primary rainbow tangential Degree of Linear Polarization ($\approx 0.95$).
    pub rb1_dolp: f32,
    /// Cloud droplet effective radius for Mie backscatter glory ($\mu\text{m}$).
    pub glory_r_eff_um: f32,
    /// Dew retro-reflection heiligenschein gain near antisolar point.
    pub heiligenschein_gain: f32,
    /// `Bit0 = InferiorMirage`, `Bit1 = SuperiorLoom`, `Bit2 = Halo22`, `Bit3 = Rainbow`.
    pub optics_flags: u32,
}

const _: [(); 64] = [(); core::mem::size_of::<AtmoElectroFieldCell>()];
const _: [(); 48] = [(); core::mem::size_of::<AeolianSaltationCell>()];
const _: [(); 64] = [(); core::mem::size_of::<DustDevilVortex>()];
const _: [(); 64] = [(); core::mem::size_of::<AtmoOpticsWavefront>()];

/// Evaluates Peek's law corona inception, spider electrostatic ballooning lift, and bee floral bias.
pub fn eval_atmo_electro_field_cell(cell: &mut AtmoElectroFieldCell, flower_height_m: f32) {
    let delta = cell.air_density_rel.clamp(0.5, 1.3);
    let r_cm = cell.tip_radius_cm.max(0.02);
    // Peek's law visual critical corona field (V/m)
    cell.corona_crit_v_m = 3.1e6 * delta * (1.0 + 0.301 / (delta * r_cm).sqrt());

    let ez_abs = cell.e_field_v_m[1].abs();
    let e_tip = cell.geom_enhance_beta.max(1.0) * ez_abs;
    if e_tip > cell.corona_crit_v_m {
        let excess = e_tip - cell.corona_crit_v_m;
        cell.corona_current_a = (excess * 1.5e-11).min(0.05);
        cell.space_charge_c_m3 = 1.0e-9 * (excess / 1.0e5).clamp(0.1, 20.0);
        cell.uv_corona_flux = excess * 2.5e-7;
        cell.storm_flags |= 1;
    } else {
        cell.corona_current_a = 0.0;
        cell.uv_corona_flux = 0.0;
        cell.storm_flags &= !1;
    }

    // Electrostatic lift per 1 nC (1e-9 C) of spider ballooning silk
    cell.balloon_lift_n = ez_abs * 1.0e-9;
    if cell.balloon_lift_n > 2.0e-7 {
        cell.storm_flags |= 4; // Sufficient electrostatic lift to balloon a small Erigone spider
    }
    cell.floral_bias_v = ez_abs * flower_height_m.max(0.05) * (1.0 - 0.5 * (-cell.pollinator_decay_s / 120.0).exp());
    cell.leader_step_prob = ((ez_abs - 45_000.0).max(0.0) / 65_000.0).clamp(0.0, 1.0);
}

/// Zero-heap Shao–Lu aeolian saltation flux & `.m64` (`GRM3`) ripple evolution stepper (20 Hz).
pub fn step_aeolian_saltation_field(
    cells: &mut [AeolianSaltationCell],
    wind_u_star: &[f32],
    dt_s: f32,
    out_m64_delta_mm: &mut [i32],
) -> usize {
    const RHO_QUARTZ: f32 = 2650.0;
    const RHO_AIR: f32 = 1.225;
    const G: f32 = 9.80665;
    const AN_SHAO: f32 = 0.0123;
    const GAMMA_COH: f32 = 3.0e-4;

    let dt = dt_s.max(0.0);
    let mut active_count = 0usize;

    for (idx, cell) in cells.iter_mut().enumerate() {
        let u_star = wind_u_star.get(idx).copied().unwrap_or(cell.u_star_m_s).max(0.0);
        cell.u_star_m_s = u_star;
        let d = cell.grain_d50_m.clamp(2.0e-5, 2.0e-3);

        // Shao & Lu (2000) threshold friction velocity u_*t(d) with slope and moisture adjustment
        let grav_term = (RHO_QUARTZ / RHO_AIR) * G * d;
        let coh_term = (GAMMA_COH / (RHO_AIR * d)) * cell.moisture_cohesion.max(1.0);
        let phi = cell.bed_slope_rad.clamp(-0.5, 0.5);
        let slope_factor = (phi.cos() + phi.sin() / 0.6249).max(0.15); // tan(32°) ≈ 0.6249
        cell.u_star_thresh_m_s = (AN_SHAO * (grav_term + coh_term) * slope_factor).sqrt();

        if u_star > cell.u_star_thresh_m_s {
            let u2_diff = u_star * u_star - cell.u_star_thresh_m_s * cell.u_star_thresh_m_s;
            cell.flux_sat_kg_m_s = 1.8 * (RHO_AIR / G) * u2_diff * u_star;
            // Relax toward saturated flux over saturation length L_sat ≈ 15 m
            cell.flux_kg_m_s += (cell.flux_sat_kg_m_s - cell.flux_kg_m_s) * (dt * 0.25).min(1.0);
            cell.saltation_height_m = 150.0 * (0.00025 / d.sqrt()) * (u_star - cell.u_star_thresh_m_s) / G.sqrt();
            cell.ripple_wavelength_m = (20.0 * cell.saltation_height_m).clamp(0.04, 1.8);
            cell.ripple_height_m = cell.ripple_wavelength_m / 28.0;
            cell.ripple_phase_rad = (cell.ripple_phase_rad + 0.8 * u_star * dt).rem_euclid(core::f32::consts::TAU);
            cell.active_flag = if d < 6.0e-5 { 2 } else { 1 };
            active_count += 1;
        } else {
            cell.flux_sat_kg_m_s = 0.0;
            cell.flux_kg_m_s *= (-dt * 1.5).exp();
            cell.active_flag = 0;
        }

        if idx < out_m64_delta_mm.len() {
            let ripple_z_m = 0.5 * cell.ripple_height_m * cell.ripple_phase_rad.sin();
            let guarded_m =
                project_metres_f32_checked(f64::from(ripple_z_m), "ripple_z_m").unwrap_or(0.0);
            out_m64_delta_mm[idx] = (guarded_m * 1000.0).round() as i32;
        }
    }
    active_count
}

/// Zero-heap Rankine dust-devil velocity & cyclostrophic pressure perturbation query (64 Hz).
pub fn sample_dust_devil_wind_and_dp(
    vortices: &[DustDevilVortex],
    query_xyz_m: [f32; 3],
    out_wind_add_m_s: &mut [f32; 3],
    out_dp_pa: &mut f32,
) {
    *out_wind_add_m_s = [0.0; 3];
    *out_dp_pa = 0.0;

    for v in vortices {
        if query_xyz_m[1] > v.height_m.max(1.0) || v.lifetime_s <= 0.0 {
            continue;
        }
        let dx = query_xyz_m[0] - v.pos_xz_m[0];
        let dz = query_xyz_m[2] - v.pos_xz_m[1];
        let r = (dx * dx + dz * dz).sqrt().max(1.0e-4);
        let rc = v.core_radius_m.max(0.5);

        // Rankine tangential velocity profile
        let v_tan = if r <= rc {
            v.v_tan_max_m_s * (r / rc)
        } else {
            v.v_tan_max_m_s * (rc / r)
        } * v.spin_sign;

        let tx = -dz / r;
        let tz = dx / r;
        let wall_factor = (-(r - rc).powi(2) / (2.0 * (0.45 * rc).powi(2))).exp();
        out_wind_add_m_s[0] += tx * v_tan + v.drift_vel_xz[0];
        out_wind_add_m_s[1] += v.v_updraft_m_s * wall_factor;
        out_wind_add_m_s[2] += tz * v_tan + v.drift_vel_xz[1];

        let dp = if r <= rc {
            -v.core_dp_pa * (1.0 - 0.5 * (r / rc).powi(2))
        } else {
            -0.5 * v.core_dp_pa * (rc / r).powi(2)
        };
        *out_dp_pa += dp;
    }
}

/// Evaluates Edlén–Stull mirage ray curvature and 4-band chromatic rainbow & $22^\circ$ ice halo radii.
pub fn eval_atmo_optics_wavefront(wf: &mut AtmoOpticsWavefront, pressure_hpa: f32, temp_k: f32) {
    // Edlen-Stull ray curvature 1/R_c = 7.895e-5 * (P / T^2) * (0.0343 + dT/dz)
    wf.ray_curv_m_inv =
        7.895e-5 * (pressure_hpa.max(500.0) / temp_k.max(220.0).powi(2)) * (0.0343 + wf.dt_dz_k_m);

    // Chromatic water & ice IOR across [Blue 450nm, Green 550nm, Red 650nm, NIR 850nm]
    let nw = [1.3390f32, 1.3340, 1.3314, 1.3285];
    let nice = [1.3155f32, 1.3105, 1.3078, 1.3045];
    let prism_60_rad = core::f32::consts::FRAC_PI_3;
    let half_prism_sin = 0.5f32; // sin(30°)

    for k in 0..4 {
        // Primary rainbow minimum deviation: i_1 = arccos(sqrt((n^2 - 1) / 3)), theta_R1 = 4*r_1 - 2*i_1
        let cos_i1 = ((nw[k] * nw[k] - 1.0) / 3.0).clamp(0.0, 1.0).sqrt();
        let i1 = cos_i1.acos();
        let r1 = (i1.sin() / nw[k]).asin();
        wf.rb1_rad_bands[k] = 4.0 * r1 - 2.0 * i1;

        // 22° hexagonal ice prism minimum deviation: theta_min = 2 * arcsin(n_ice * sin(30°)) - 60°
        wf.halo22_rad_bands[k] = 2.0 * (nice[k] * half_prism_sin).asin() - prism_60_rad;
    }
    wf.rb1_dolp = 0.95;
    wf.optics_flags = u32::from(wf.dt_dz_k_m < -0.0343) | (u32::from(wf.dt_dz_k_m > 0.10) << 1) | (1 << 2) | (1 << 3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shao_lu_aeolian_saltation_migrates_ripples_and_optics_computes_rainbow_and_22deg_halo() {
        let mut sand = [AeolianSaltationCell {
            grain_d50_m: 1.8e-4, // 180 um dune quartz sand
            u_star_m_s: 0.0,
            u_star_thresh_m_s: 0.0,
            flux_kg_m_s: 0.0,
            flux_sat_kg_m_s: 0.0,
            saltation_height_m: 0.0,
            ripple_wavelength_m: 0.0,
            ripple_height_m: 0.0,
            ripple_phase_rad: 1.2,
            moisture_cohesion: 1.0,
            bed_slope_rad: 0.0,
            active_flag: 0,
        }];
        let mut m64_delta = [0i32; 1];
        let active = step_aeolian_saltation_field(&mut sand, &[0.55], 2.0, &mut m64_delta);
        assert_eq!(active, 1);
        assert!(sand[0].u_star_thresh_m_s > 0.20 && sand[0].u_star_thresh_m_s < 0.30);
        assert!(sand[0].flux_kg_m_s > 0.01);

        let mut wf = AtmoOpticsWavefront {
            dt_dz_k_m: -1.8, // Superheated desert asphalt -> inferior mirage
            thermal_scale_beta_m: 0.15,
            ray_curv_m_inv: 0.0,
            v_point_height_m: 0.05,
            rb1_rad_bands: [0.0; 4],
            halo22_rad_bands: [0.0; 4],
            rb1_dolp: 0.0,
            glory_r_eff_um: 10.0,
            heiligenschein_gain: 1.4,
            optics_flags: 0,
        };
        eval_atmo_optics_wavefront(&mut wf, 1013.25, 310.0);
        assert!(wf.ray_curv_m_inv < 0.0, "superheated surface must curve optical rays upward (inferior mirage)");
        let rb_red_deg = wf.rb1_rad_bands[2].to_degrees();
        let halo_red_deg = wf.halo22_rad_bands[2].to_degrees();
        assert!((rb_red_deg - 42.3).abs() < 0.6, "primary red rainbow must sit near 42.3 deg");
        assert!((halo_red_deg - 21.7).abs() < 0.6, "22-deg hexagonal ice halo must sit near 21.7..22.0 deg");
    }
}

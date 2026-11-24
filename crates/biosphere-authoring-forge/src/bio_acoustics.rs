//! # Vector 3 — Bio-Acoustics, Foliage Aeroacoustics, Minnaert Bubbles & Acoustic Ducting (`bio_acoustics.rs`)
//!
//! Implements the physical audio synthesis and curved-ray atmospheric propagation solvers from Vector 3:
//! 1. **Foliage Aeroacoustics (`AeolianNeedle` vs. `Broadleaf`):**
//!    *Casuarina* she-oak cladode / pine needle Strouhal lift-dipole vortex whistling
//!    ($f_{\text{aeo}} = \text{St}\cdot U / d_n$, radiating $\propto U^6$) vs. broadleaf *Eucalyptus*
//!    wake-turbulence rustling ($\propto U^8$).
//! 2. **Underwater Raindrop Bubble Minnaert Resonance (`Minnaert`):**
//!    Entrained spherical air bubble pulsation $f_M \approx 3.283 / R_0$ (1.6 kHz to 16.4 kHz).
//! 3. **Cicada Tymbal Buckling, Orthoptera Stridulation & Anuran Vocal-Sac Impedance Matching:**
//!    *Cyclochila australasiae* 4-rib sequential tymbal snap exciting a 4.3 kHz abdominal Helmholtz
//!    resonator, cricket peg-and-scraper wing-harp escapement ($Q \approx 20$), and hyperelastic
//!    anuran gular vocal-sac monopole impedance matching + air recycling.
//! 4. **Atmospheric Acoustic Refraction Ducting & Forest Trunk Scattering (`AcousticRayProfile`):**
//!    Vertical effective sound-speed gradient $\partial c_{\text{eff}} / \partial z$ driving
//!    nighttime radiative-inversion surface waveguides ($-3\text{ dB}$ cylindrical spreading) vs.
//!    daytime lapse upward-refraction acoustic shadow zones.

use serde::{Deserialize, Serialize};

/// 64-byte POD compact bio-acoustic / aeroacoustic modal emitter.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BioAcousticEmitter {
    /// Emitter world position `[x, y, z]` (m).
    pub pos_xyz_m: [f32; 3],
    /// Radiation dipole axis (normal to wind for Aeolian needle whistling).
    pub dipole_axis: [f32; 3],
    /// Fundamental / Minnaert / tymbal / peg carrier frequency (Hz).
    pub carrier_freq_hz: f32,
    /// Resonator quality factor $Q$ (e.g. `20.0` cricket harp, `8.0` cicada abdomen).
    pub quality_q: f32,
    /// Syllable / muscle contraction / chirp pulse rate (Hz).
    pub pulse_rate_hz: f32,
    /// Radiated acoustic power (W).
    pub acoustic_power_w: f32,
    /// Anuran vocal-sac radius $R_s$ or Minnaert bubble radius $R_0$ (m).
    pub sac_radius_m: f32,
    /// Vocal-sac radial stretch ratio $\to$ `.10d` throat vertex morph.
    pub sac_strain: f32,
    /// Deterministic oscillator phase in `[0.0, 2π)`.
    pub phase_rad: f32,
    /// Current envelope amplitude in `[0.0, 1.0]`.
    pub env_amp: f32,
    /// `0 = AeolianNeedle`, `1 = BroadleafRustle`, `2 = MinnaertBubble`, `3 = CicadaTymbal`, `4 = CricketHarp`, `5 = AnuranVocalSac`.
    pub source_kind: u32,
    /// Linked `.10d` entity ID for synchronized vertex micro-vibration.
    pub host_entity_id: u32,
}

/// 48-byte POD atmospheric refraction & forest scattering acoustic propagation profile.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcousticRayProfile {
    /// Effective sound speed at $z = 0$ along propagation path (m/s).
    pub c_eff_ground_m_s: f32,
    /// Vertical sound-speed gradient $\partial c_{\text{eff}} / \partial z$ ($\text{s}^{-1}$; $>0$ = downward inversion duct).
    pub dc_eff_dz_s_inv: f32,
    /// Signed ray curvature $1 / R_{\text{ray}}$ ($\text{m}^{-1}$).
    pub ray_curvature_m_inv: f32,
    /// Nighttime inversion waveguide thickness $H_{\text{duct}}$ (m).
    pub duct_height_m: f32,
    /// Daytime upward-refraction acoustic shadow onset distance (m).
    pub shadow_dist_m: f32,
    /// Geometric spreading exponent (`2.0` spherical $\to$ `1.0` stratified cylindrical in duct).
    pub spreading_exp: f32,
    /// Atmospheric molecular absorption at 1 kHz (dB/km).
    pub air_abs_db_km_1k: f32,
    /// Atmospheric molecular absorption at 4 kHz (dB/km).
    pub air_abs_db_km_4k: f32,
    /// Forest trunk & foliage scattering attenuation (dB/m).
    pub forest_scatter_db_m: f32,
    /// Normalized ground acoustic impedance $\text{Re}(Z)$.
    pub ground_impedance_re: f32,
    /// Normalized ground acoustic impedance $\text{Im}(Z)$.
    pub ground_impedance_im: f32,
    /// `0 = NeutralWind`, `1 = DayLapseShadow`, `2 = Cloudy`, `3 = NightInversionDuct`.
    pub regime_flags: u32,
}

const _: [(); 64] = [(); core::mem::size_of::<BioAcousticEmitter>()];
const _: [(); 48] = [(); core::mem::size_of::<AcousticRayProfile>()];

/// Zero-heap bio-acoustic & aeroacoustic oscillator stepper (64 Hz).
pub fn step_bio_acoustic_emitters(
    emitters: &mut [BioAcousticEmitter],
    local_wind_xyz: &[[f32; 3]],
    local_temp_c: &[f32],
    dt_s: f32,
) -> usize {
    let dt = dt_s.max(0.0);
    let mut active_count = 0usize;

    for (idx, em) in emitters.iter_mut().enumerate() {
        let w = local_wind_xyz.get(idx).copied().unwrap_or([2.0, 0.0, 0.0]);
        let u = (w[0] * w[0] + w[1] * w[1] + w[2] * w[2]).sqrt();
        let temp_c = local_temp_c.get(idx).copied().unwrap_or(22.0);

        match em.source_kind {
            0 => {
                // Casuarina / pine needle Aeolian Strouhal tone (d_n ≈ 1.1 mm, St = 0.20, dipole power ~ U^6)
                let d_needle = em.sac_radius_m.clamp(0.0006, 0.0025);
                em.carrier_freq_hz = (0.20 * u) / d_needle;
                em.acoustic_power_w = 1.2e-9 * u.powi(6);
                em.env_amp = (u / 12.0).clamp(0.0, 1.0);
            }
            1 => {
                // Broadleaf Eucalyptus flutter rustle (power ~ U^8 wake turbulence)
                em.carrier_freq_hz = 250.0 + 45.0 * u;
                em.acoustic_power_w = 4.0e-11 * u.powi(8).min(0.05);
                em.env_amp = (u / 10.0).clamp(0.0, 1.0);
            }
            2 => {
                // Underwater raindrop bubble Minnaert resonance f_M ≈ 3.283 / R_0
                let r0 = em.sac_radius_m.clamp(0.00015, 0.003);
                em.carrier_freq_hz = 3.283 / r0;
                em.env_amp = (em.env_amp * (-dt * 35.0).exp()).clamp(0.0, 1.0);
            }
            3 => {
                // Cyclochila australasiae cicada 4-rib tymbal buckling (~4.3 kHz abdominal Helmholtz resonator)
                em.carrier_freq_hz = 4300.0;
                em.pulse_rate_hz = 117.0 * 4.0;
                em.acoustic_power_w = if temp_c > 18.0 { 0.0055 } else { 0.0 };
                em.env_amp = if temp_c > 18.0 { 0.92 } else { 0.0 };
            }
            4 => {
                // Orthoptera tree/field cricket stridulation (Dolbear temperature-shifted carrier 3.0..4.8 kHz)
                em.carrier_freq_hz = (2800.0 + 62.0 * (temp_c - 12.0).max(0.0)).clamp(2500.0, 5200.0);
                em.pulse_rate_hz = (12.0 + 1.4 * (temp_c - 10.0).max(0.0)).clamp(5.0, 65.0);
                em.env_amp = 0.78;
            }
            _ => {
                // Anuran elastic vocal sac monopole impedance matcher (R_rad ~ (k R_s)^2 / (1 + (k R_s)^2))
                let pulse_p = (em.phase_rad * 0.05).sin().max(0.0);
                em.sac_strain = 1.0 + 0.65 * pulse_p;
                let r_eff = em.sac_radius_m.max(0.008) * em.sac_strain;
                let k_wave = (core::f32::consts::TAU * em.carrier_freq_hz.max(200.0)) / 343.0;
                let kr2 = (k_wave * r_eff).powi(2);
                let rad_eff = kr2 / (1.0 + kr2);
                em.acoustic_power_w = 0.012 * rad_eff * pulse_p;
                em.env_amp = pulse_p;
            }
        }

        em.phase_rad =
            (em.phase_rad + core::f32::consts::TAU * em.pulse_rate_hz.max(1.0) * dt).rem_euclid(core::f32::consts::TAU);
        if em.env_amp > 0.01 {
            active_count += 1;
        }
    }
    active_count
}

/// Zero-heap curved-ray atmospheric attenuation, nighttime inversion ducting & forest scattering evaluator (20 Hz).
pub fn eval_acoustic_refraction_loss_db(
    profile: &AcousticRayProfile,
    distance_m: f32,
    freq_hz: f32,
    out_delay_s: &mut f32,
) -> f32 {
    let r = distance_m.max(1.0);
    let c_eff = profile.c_eff_ground_m_s.max(300.0);
    *out_delay_s = r / c_eff;

    // Geometric spreading transitions from spherical (20 log10 r) to cylindrical (10 log10 r) inside a downward inversion duct
    let geom_db = if profile.dc_eff_dz_s_inv > 0.01 && r > profile.duct_height_m.max(15.0) {
        let r_d = profile.duct_height_m.max(15.0);
        20.0 * r_d.log10() + 10.0 * (r / r_d).log10()
    } else {
        10.0 * profile.spreading_exp.clamp(1.0, 2.5) * r.log10()
    };

    // Daytime upward-refraction shadow zone extra diffraction loss
    let shadow_db = if profile.dc_eff_dz_s_inv < -0.01 && r > profile.shadow_dist_m.max(40.0) {
        (18.0 * ((r - profile.shadow_dist_m) / 100.0)).min(24.0)
    } else {
        0.0
    };

    // Frequency-interpolated atmospheric absorption + sqrt(f) forest trunk scattering
    let f_khz = (freq_hz.max(50.0)) * 0.001;
    let alpha_air_km = profile.air_abs_db_km_1k + (profile.air_abs_db_km_4k - profile.air_abs_db_km_1k) * ((f_khz - 1.0) / 3.0).clamp(0.0, 2.0);
    let air_db = alpha_air_km.max(0.2) * (r * 0.001);
    let forest_db = profile.forest_scatter_db_m.max(0.0) * r * f_khz.sqrt();

    geom_db + shadow_db + air_db + forest_db
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nighttime_inversion_duct_reduces_transmission_loss_by_over_15db_vs_daytime_lapse() {
        let night_duct = AcousticRayProfile {
            c_eff_ground_m_s: 338.0,
            dc_eff_dz_s_inv: 0.08, // Downward-refracting temperature inversion
            ray_curvature_m_inv: 0.00023,
            duct_height_m: 25.0,
            shadow_dist_m: 10_000.0,
            spreading_exp: 1.0,
            air_abs_db_km_1k: 4.5,
            air_abs_db_km_4k: 22.0,
            forest_scatter_db_m: 0.015,
            ground_impedance_re: 12.0,
            ground_impedance_im: 14.0,
            regime_flags: 3,
        };
        let day_lapse = AcousticRayProfile {
            dc_eff_dz_s_inv: -0.09, // Upward-refracting daytime lapse
            shadow_dist_m: 85.0,
            spreading_exp: 2.0,
            regime_flags: 1,
            ..night_duct
        };
        let mut delay = 0.0f32;
        let loss_night = eval_acoustic_refraction_loss_db(&night_duct, 400.0, 2000.0, &mut delay);
        let loss_day = eval_acoustic_refraction_loss_db(&day_lapse, 400.0, 2000.0, &mut delay);
        assert!(loss_day - loss_night > 18.0, "nighttime inversion duct must carry sound >18 dB louder than daytime shadow");
    }
}

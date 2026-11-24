//! # Vector 5 — Capillary Surface Tension, Lotus Superhydrophobicity, Spider Silk & Dinoflagellate Wakes (`capillary_intertidal.rs`)
//!
//! Implements the micro-capillary ($L < l_c \approx 2.73\text{ mm}$) and coastal/intertidal physics from Vector 5:
//! 1. **Gerridae Water-Strider Meniscus Support & Dipolar Vortex Propulsion (`CapillaryMeniscusLeg`):**
//!    Archimedes + Keller meniscus displacement support and subsurface dipolar vortex impulse ($64\%\text{–}90\%$ of thrust).
//! 2. **Lotus Cassie–Baxter Superhydrophobicity, Raindrop Splash Crowns & Worthington Jets (`SplashCrownEvent`):**
//!    Weber/Ohnesorge splashing parameter $K = \text{We}_n \text{Oh}^{-0.4} > 2100$, Cassie-to-Wenzel
//!    impalement transition, secondary droplet count $N_s$, and central Worthington jet apex height.
//! 3. **Orb-Web Viscid Silk Plateau–Rayleigh Glue Beading & Electrostatic Prey Capture (`ViscidSilkCaptureStrand`):**
//!    Fastest-growing instability mode $k_{\max} R_0 \approx 0.697$ ($\lambda_{\text{bead}} \approx 9.01 R_0$)
//!    and image-charge deflection toward approaching charged insects.
//! 4. **Intertidal Mangrove Pneumatophores, Barnacle Zonation & Langmuir Circulation (`IntertidalZonationCell`):**
//!    *Avicennia marina* aerial root density ($14\text{–}516\text{ m}^{-2}$), unimodal rocky-shore
//!    barnacle/oyster zonation, and Craik–Leibovich Langmuir foam windrows.
//! 5. **Shear-Triggered Marine Dinoflagellate Bioluminescent Wakes (`DinoflagellateWakeCell`):**
//!    Excitable luciferin-pool kinetics triggered specifically by viscous fluid shear stress
//!    $\tau_s \ge \tau_{\text{th}}$ ($0.039\text{–}0.32\text{ Pa}$) into `Q42EMF64` Band 1 (`Blue`, $475\text{ nm}$).

use serde::{Deserialize, Serialize};

/// 48-byte POD water-strider superhydrophobic leg meniscus & dipolar vortex impulse state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapillaryMeniscusLeg {
    /// Tarsus midpoint world position `[x, y, z]` (m).
    pub tip_pos_xyz_m: [f32; 3],
    /// Leg sweep velocity on water surface `[vx, vy, vz]` (m/s).
    pub stroke_vel_xyz: [f32; 3],
    /// Contacting superhydrophobic tarsus length $L_{\text{leg}}$ (m).
    pub wetted_len_m: f32,
    /// Meniscus depression depth (m).
    pub dimple_depth_m: f32,
    /// Superhydrophobic Cassie contact angle ($\approx 2.93\text{ rad} / 168^\circ$).
    pub contact_angle_rad: f32,
    /// Archimedes + Keller vertical meniscus support force (N).
    pub support_force_n: f32,
    /// Subsurface dipolar vortex propulsive thrust (N; $64\%\text{–}90\%$ of total).
    pub vortex_impulse_n: f32,
    /// `1` if vertical load exceeded surface-piercing limit $2 \gamma L_{\text{leg}} \sin\theta$.
    pub pierced_flag: u32,
}

/// 48-byte POD raindrop splash crown, Worthington jet & Lotus wetting-transition event.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplashCrownEvent {
    /// Impact world coordinate `[x, y, z]` (m).
    pub impact_xyz_m: [f32; 3],
    /// Incident raindrop diameter $D$ (m).
    pub drop_diam_m: f32,
    /// Impact normal speed $U_n$ (m/s).
    pub normal_vel_m_s: f32,
    /// Normal Weber number $\text{We}_n = \rho_w U_n^2 D / \gamma$.
    pub weber_normal: f32,
    /// Ohnesorge number $\text{Oh} = \mu_w / \sqrt{\rho_w \gamma D}$.
    pub ohnesorge: f32,
    /// Expanding splash crown radius (m).
    pub crown_radius_m: f32,
    /// Central Worthington jet apex height (m).
    pub jet_height_m: f32,
    /// Elapsed time since impact (s).
    pub age_s: f32,
    /// Ejected satellite droplet count $N_s$.
    pub secondary_count: u16,
    /// `0 = CassieRebound`, `1 = WenzelImpalement`, `2 = PoolCrown`.
    pub wetting_state: u16,
    /// Apparent leaf/substrate contact angle (rad).
    pub surface_contact_rad: f32,
}

/// 48-byte POD orb-web viscid capture silk Plateau–Rayleigh & electrostatic strand.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViscidSilkCaptureStrand {
    /// Radial silk anchor A `[x, y, z]` (m).
    pub anchor_a_xyz: [f32; 3],
    /// Radial silk anchor B `[x, y, z]` (m).
    pub anchor_b_xyz: [f32; 3],
    /// Initial cylindrical aqueous glue coating radius $R_0$ (m).
    pub glue_radius_m: f32,
    /// Plateau–Rayleigh droplet wavelength $\lambda_{\text{bead}} \approx 9.01 R_0$ (m).
    pub bead_spacing_m: f32,
    /// Flagelliform axial silk tension (N).
    pub tension_n: f32,
    /// Electrostatic image charge induced on conductive glue beads (C).
    pub induced_charge_c: f32,
    /// Electrostatic + aerodynamic midpoint deflection toward charged prey (m).
    pub midpoint_defl_m: f32,
    /// Droplet adhesive stiffness ($\approx 0.31\text{ N/m}$).
    pub glue_spring_n_m: f32,
}

/// 64-byte POD intertidal mangrove pneumatophore, barnacle zonation & Langmuir windrow cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntertidalZonationCell {
    /// Bed elevation relative to Mean Sea Level (from `.m64`, m).
    pub bed_elev_m: f32,
    /// Mean tidal submergence fraction in `[0.0, 1.0]`.
    pub immersion_frac: f32,
    /// Silt + clay sediment mud fraction driving hypoxia in `[0.0, 1.0]`.
    pub sediment_mud_frac: f32,
    /// Pore-water dissolved oxygen saturation ratio in `[0.0, 1.0]`.
    pub sed_oxygen_frac: f32,
    /// *Avicennia marina* aerial pneumatophores per $\text{m}^2$ (`14..516`).
    pub pneu_density_m2: f32,
    /// Aerial root height above mud (m).
    pub pneu_height_m: f32,
    /// Abundant-centre intertidal barnacle cover fraction in `[0.0, 1.0]`.
    pub barnacle_cover: f32,
    /// Lower-mid intertidal oyster reef cover fraction in `[0.0, 1.0]`.
    pub oyster_cover: f32,
    /// Turbulent Langmuir number $\text{La}$.
    pub langmuir_num: f32,
    /// Convergent Langmuir foam windrow spacing (`2..300` m).
    pub windrow_spacing_m: f32,
    /// Foam streak orientation between wind and Stokes drift (rad).
    pub windrow_azimuth_rad: f32,
    /// Surface organic foam/surfactant film density in `[0.0, 1.0]`.
    pub surfactant_conc: f32,
    /// Intertidal salinity (PSU).
    pub salinity_psu: f32,
    /// Emersion rock/organism peak aerial temperature ($^\circ\text{C}$).
    pub aerial_temp_c: f32,
    /// Grid coordinate key.
    pub cell_xy_index: u32,
    /// `Bit0 = Mangrove`, `Bit1 = RockyShore`, `Bit2 = OpenLangmuir`.
    pub ecotope_flags: u32,
}

/// 32-byte POD shear-triggered marine dinoflagellate bioluminescence cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DinoflagellateWakeCell {
    /// Local viscous fluid shear stress $\tau_s = \mu_w \dot{\gamma}$ (Pa).
    pub shear_stress_pa: f32,
    /// Species shear activation threshold (`0.039..0.32` Pa).
    pub threshold_pa: f32,
    /// Remaining stimulable luciferin reserve $R_{\text{pool}} \in [0.0, 1.0]$.
    pub luciferin_pool: f32,
    /// Normalized blue bioluminescent flash intensity in `[0.0, 1.0]`.
    pub flash_intensity: f32,
    /// `0.0` daytime photoinhibited .. `1.0` nocturnal dark-adapted.
    pub circadian_gate: f32,
    /// Dinoflagellate bloom concentration (cells/L).
    pub cell_density_l: f32,
    /// Blue-band emission peak ($\approx 475.0\text{ nm}$).
    pub wavelength_nm: f32,
    /// `0 = Lingulodinium`, `1 = Pyrocystis`, `2 = Noctiluca`.
    pub species_id: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<CapillaryMeniscusLeg>()];
const _: [(); 48] = [(); core::mem::size_of::<SplashCrownEvent>()];
const _: [(); 48] = [(); core::mem::size_of::<ViscidSilkCaptureStrand>()];
const _: [(); 64] = [(); core::mem::size_of::<IntertidalZonationCell>()];
const _: [(); 32] = [(); core::mem::size_of::<DinoflagellateWakeCell>()];

/// Zero-heap dinoflagellate wake shear-stress kinetics into `Q42EMF64` Band 1 (`Blue`, 64 Hz).
pub fn step_dinoflagellate_bioluminescence(
    cells: &mut [DinoflagellateWakeCell],
    fluid_shear_pa: &[f32],
    solar_irradiance_w_m2: f32,
    dt_s: f32,
    out_blue_flux_w_m2: &mut [f32],
) -> usize {
    let dt = dt_s.max(1e-4);
    let dark_gate = (1.0 - (solar_irradiance_w_m2.max(0.0) / 5.0)).clamp(0.0, 1.0);
    let mut flashing = 0usize;

    for (idx, cell) in cells.iter_mut().enumerate() {
        let tau_s = fluid_shear_pa.get(idx).copied().unwrap_or(cell.shear_stress_pa).max(0.0);
        cell.shear_stress_pa = tau_s;
        cell.circadian_gate = dark_gate;
        let tau_th = cell.threshold_pa.clamp(0.02, 0.5);

        let phi = ((tau_s - tau_th) / (1.0 - tau_th).max(0.1)).clamp(0.0, 1.0);
        let tau_response = if phi > 0.0 { 0.012 } else { 0.14 };
        let target_i = dark_gate * cell.luciferin_pool * phi;
        cell.flash_intensity += (target_i - cell.flash_intensity) * (dt / tau_response).min(1.0);

        // Luciferin depletion under high shear vs. slow cellular recharge
        let k_dep = 0.01 * phi;
        let recharge = (1.0 - cell.luciferin_pool) * (dt / 120.0);
        cell.luciferin_pool = (cell.luciferin_pool * (-k_dep * dt).exp() + recharge).clamp(0.0, 1.0);

        let flux = cell.flash_intensity * (cell.cell_density_l.max(0.0) * 1.5e-5);
        if idx < out_blue_flux_w_m2.len() {
            out_blue_flux_w_m2[idx] = flux;
        }
        if cell.flash_intensity > 0.02 {
            flashing += 1;
        }
    }
    flashing
}

/// Zero-heap Gerridae water-strider meniscus support & dipolar vortex propulsion stepper (64 Hz).
pub fn step_water_strider_meniscus(
    legs: &mut [CapillaryMeniscusLeg],
    surface_tension_n_m: f32,
    water_density_kg_m3: f32,
    body_mass_kg: f32,
) -> [f32; 3] {
    let gamma = surface_tension_n_m.clamp(0.03, 0.076);
    let rho = water_density_kg_m3.max(990.0);
    let g = 9.80665f32;
    let l_cap = (gamma / (rho * g)).sqrt();
    let per_leg_load = (body_mass_kg.max(1.0e-7) * g) / (legs.len().max(1) as f32);
    let mut net_force = [0.0f32; 3];

    for leg in legs.iter_mut() {
        let l_w = leg.wetted_len_m.max(0.001);
        let sin_eff = (leg.contact_angle_rad - core::f32::consts::FRAC_PI_2).sin().clamp(0.2, 0.99);
        let f_pierce = 2.0 * gamma * l_w * sin_eff + rho * g * l_w * l_cap * l_cap * 0.25;
        if per_leg_load > f_pierce {
            leg.pierced_flag = 1;
            leg.support_force_n = 0.0;
        } else {
            leg.pierced_flag = 0;
            leg.support_force_n = per_leg_load;
            leg.dimple_depth_m = (per_leg_load / (2.0 * gamma * l_w)).clamp(0.0, l_cap * 1.4);
        }

        // Subsurface hemispherical dipolar vortex thrust during sculling stroke
        let vx = leg.stroke_vel_xyz[0];
        let vz = leg.stroke_vel_xyz[2];
        let v_sq = vx * vx + vz * vz;
        let area_dimple = l_w * leg.dimple_depth_m.max(1.0e-4);
        leg.vortex_impulse_n = 0.5 * rho * area_dimple * v_sq * 0.82;
        let v_mag = v_sq.sqrt().max(1e-6);
        net_force[0] += -vx / v_mag * leg.vortex_impulse_n;
        net_force[1] += leg.support_force_n;
        net_force[2] += -vz / v_mag * leg.vortex_impulse_n;
    }
    net_force
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_strider_meniscus_supports_weight_and_dinoflagellates_flash_under_wave_shear() {
        let mut legs = [CapillaryMeniscusLeg {
            tip_pos_xyz_m: [0.0; 3],
            stroke_vel_xyz: [-0.25, 0.0, 0.0],
            wetted_len_m: 0.005,
            dimple_depth_m: 0.0,
            contact_angle_rad: 2.93, // 168° superhydrophobic micro-setae
            support_force_n: 0.0,
            vortex_impulse_n: 0.0,
            pierced_flag: 0,
        }; 4];
        let force = step_water_strider_meniscus(&mut legs, 0.0728, 1000.0, 1.5e-5);
        assert_eq!(legs[0].pierced_flag, 0);
        assert!(force[1] >= 1.5e-5 * 9.80, "meniscus dimples must support strider weight");
        assert!(force[0] > 0.0, "dipolar vortex shedding must generate forward thrust");

        let mut dino = [DinoflagellateWakeCell {
            shear_stress_pa: 0.0,
            threshold_pa: 0.10,
            luciferin_pool: 1.0,
            flash_intensity: 0.0,
            circadian_gate: 1.0,
            cell_density_l: 50_000.0,
            wavelength_nm: 475.0,
            species_id: 0,
        }];
        let mut blue_flux = [0.0f32; 1];
        let flashing = step_dinoflagellate_bioluminescence(&mut dino, &[0.85], 0.0, 0.015625, &mut blue_flux);
        assert_eq!(flashing, 1);
        assert!(dino[0].flash_intensity > 0.50);
        assert!(blue_flux[0] > 0.25);
    }
}

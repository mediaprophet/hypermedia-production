//! # Vector 2 — Australian Pyro-Botany, Epicormic Regeneration, Serotiny & Pyrocumulus (`pyro_botany.rs`)
//!
//! Implements the Australian sclerophyll bushfire ecology and pyro-convective continuum from Vector 2:
//! 1. **Bark Thermal Insulation, Charring Recession & Arrhenius Cambial Viability (`PyroBotanyStemState`):**
//!    Models conservative bark thermal diffusivity ($\alpha_b \approx 1.35 \times 10^{-7}\text{ m}^2/\text{s}$),
//!    stringybark vs. smooth-gum charring recession $\dot{x}_{\text{char}}$, firebrand shedding, and
//!    Arrhenius vascular cambium survival $V \in [0, 1]$.
//! 2. **Van Wagner (1973/1977) Crown Scorch Height vs. Active Crown Consumption:**
//!    Exact SI lethal scorch height $h_s = 0.74183 I_B^{7/6} / ((0.025574 I_B + U^3)^{1/2}(60 - T_a))$
//!    and critical crown-fire ignition intensity $I_0 = [0.010 z_{\text{cbh}}(460 + 25.9\cdot\text{FMC})]^{3/2}$.
//! 3. **Deep Epicormic Strand ("Fuzzy Tree") & Subterranean Lignotuber Resprouting:**
//!    Activates dormant radial epicormic meristem strands along charred trunks when crown defoliation
//!    exceeds $0.4$ and cambial viability $V > 0.5$, or triggers basal lignotuber coppice shoots when
//!    aerial stems suffer cambial necrosis ($V \le 0.5$).
//! 4. **Serotinous Woody Follicle Dehiscence (`SerotinousFolliclePod`):**
//!    Two-stage thermal gate for *Banksia*, *Hakea*, and *Eucalyptus* capsules: self-sealing junction
//!    waxes ($45\text{–}55^\circ\text{C}$) and Boltzmann sigmoid valve rupture at $T_o \in [60, 76]^\circ\text{C}$.
//! 5. **Pyrocumulus (`pyroCu`) Convective Plumes & DeBano Hydrophobic Ash (`PyroPlumeColumn`):**
//!    Modified Byram convective number $N_c$, bent-over plume ceiling $Z_{\text{top}}$, LCL latent
//!    condensation blow-up, and hydrophilic surface ash overlying a fire-condensed subsurface
//!    water-repellent soil barrier.

use serde::{Deserialize, Serialize};

/// 64-byte POD per-stem/segment pyro-botanical bark insulation, Van Wagner scorch & epicormic state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyroBotanyStemState {
    /// Remaining uncharred bark thickness $x_b$ (m).
    pub bark_thickness_m: f32,
    /// Cumulative charred outer layer thickness $x_{\text{char}}$ (m).
    pub char_depth_m: f32,
    /// Dry bark bulk density ($\text{kg}/\text{m}^3$; low = fibrous insulating stringybark, high = dense).
    pub bark_density_kg_m3: f32,
    /// Bark moisture content (dry-weight fraction).
    pub bark_moisture_frac: f32,
    /// Current outer bark surface temperature ($^\circ\text{C}$).
    pub surface_temp_c: f32,
    /// Inner vascular cambium temperature $T_c$ ($^\circ\text{C}$).
    pub cambium_temp_c: f32,
    /// Arrhenius survival viability $V \in [0.0, 1.0]$ (necrosis at $V < 0.5$).
    pub cambium_viability: f32,
    /// Fraction of crown height $\le$ Van Wagner scorch height $h_s$ in `[0.0, 1.0]`.
    pub crown_scorch_frac: f32,
    /// Fraction of crown combusted when $I_B \ge I_0$ in `[0.0, 1.0]`.
    pub crown_consumed_frac: f32,
    /// Activated radial epicormic strand bud vigor in `[0.0, 1.0]`.
    pub epicormic_vigor: f32,
    /// Subterranean basal lignotuber bud viability in `[0.0, 1.0]`.
    pub lignotuber_viability: f32,
    /// Glowing bark firebrand detachment rate (brands/s, highest in stringybark/ribbon gums).
    pub ember_shed_rate: f32,
    /// Post-fire epicormic or basal coppice shoot length (m).
    pub shoot_length_m: f32,
    /// Elapsed post-fire recovery time (s).
    pub time_since_fire_s: f32,
    /// `0 = SmoothGum`, `1 = Stringybark`, `2 = Ironbark`, `3 = Tessellated`.
    pub bark_type: u32,
    /// `Bit0 = EpicormicStrands`, `Bit1 = Lignotuber`, `Bit2 = ObligateSeeder`.
    pub strategy_flags: u32,
}

/// 48-byte POD serotinous woody fruit/follicle cluster (*Banksia*, *Hakea*, *Eucalyptus*).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerotinousFolliclePod {
    /// Attachment coordinate on `.10d` host branch (m).
    pub pos_local_m: [f32; 3],
    /// Current valve surface temperature ($^\circ\text{C}$).
    pub follicle_temp_c: f32,
    /// Insulated internal seed core temperature ($^\circ\text{C}$).
    pub seed_core_temp_c: f32,
    /// Self-sealing junction wax melting temperature $T_m$ (`45..55` $^\circ\text{C}$).
    pub wax_melt_temp_c: f32,
    /// Boltzmann 50%-opening threshold temperature $T_o$ (`60..76` $^\circ\text{C}$).
    pub open_temp_to_c: f32,
    /// Valve opening ratio in `[0.0, 1.0]`.
    pub open_fraction: f32,
    /// Persistent dead floral mantle fuel mass (kg).
    pub floret_fuel_kg: f32,
    /// Viable seeds held behind hygroscopic separator plate.
    pub seeds_remaining: u32,
    /// Parent `PyroBotanyStemState` index.
    pub stem_index: u32,
    /// `0 = Weak(<2y)`, `1 = Moderate(2-5y)`, `2 = Strong(>5y)`.
    pub serotiny_class: u32,
}

/// 64-byte POD pyrocumulus (`pyroCu`) convective column & ground hydrophobic-ash emitter.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PyroPlumeColumn {
    /// Fireline segment centroid XZ (m).
    pub origin_xz_m: [f32; 2],
    /// Byram fireline intensity $I_B$ (kW/m).
    pub fireline_i_kw_m: f32,
    /// Plume buoyancy flux $F$ ($\text{m}^4/\text{s}^3$).
    pub buoyancy_flux_m4_s3: f32,
    /// Modified Byram convective number $N_c$.
    pub byram_nc: f32,
    /// Bent-over plume ceiling height $Z_{\text{top}}$ (m).
    pub plume_top_z_m: f32,
    /// Ambient Lifting Condensation Level (`LCL`, m).
    pub lcl_height_m: f32,
    /// Condensed pyrocumulus cloud liquid water content ($\text{kg}/\text{m}^3$).
    pub pyro_cu_lwc_kg_m3: f32,
    /// Deposited hydrophilic surface ash mantle thickness (m).
    pub ash_deposit_m: f32,
    /// DeBano condensed subsurface water-repellency index $R_{\text{hyd}} \in [0.0, 1.0]$.
    pub subsurf_hydrophob: f32,
    /// Depth of condensed repellent seal (`0.005..0.06` m).
    pub hydrophobic_depth_m: f32,
    /// Core vertical updraft speed (m/s).
    pub updraft_max_m_s: f32,
    /// Maximum lofting altitude for 50 mm glowing bark firebrands (m).
    pub ember_loft_max_m: f32,
    /// Ambient vertical wind shear $\Lambda$ ($\text{s}^{-1}$).
    pub wind_shear_s_inv: f32,
    /// Brunt–Väisälä stratification frequency $N$ ($\text{s}^{-1}$).
    pub brunt_vaisala_s_inv: f32,
    /// `Bit0 = ActivePyroCu`, `Bit1 = BlowUpRegime`.
    pub plume_flags: u32,
}

const _: [(); 64] = [(); core::mem::size_of::<PyroBotanyStemState>()];
const _: [(); 48] = [(); core::mem::size_of::<SerotinousFolliclePod>()];
const _: [(); 64] = [(); core::mem::size_of::<PyroPlumeColumn>()];

/// Zero-heap pyro-botanical stem heating, Van Wagner scorch/crown check & epicormic update (20 Hz).
pub fn step_pyro_botany_stems(
    stems: &mut [PyroBotanyStemState],
    fireline_i_kw_m: &[f32],
    crown_base_h_m: &[f32],
    foliar_mc_pct: &[f32],
    ambient_temp_c: f32,
    in_stand_wind_m_s: f32,
    dt_s: f32,
) -> u32 {
    const ALPHA_BARK_M2_S: f32 = 1.35e-7;
    let dt = dt_s.max(0.0);
    let t_amb = ambient_temp_c.clamp(-10.0, 52.0);
    let u_wind = in_stand_wind_m_s.max(0.1);
    let mut resprouting_count = 0u32;

    for (idx, stem) in stems.iter_mut().enumerate() {
        let i_b = fireline_i_kw_m.get(idx).copied().unwrap_or(0.0).max(0.0);
        let z_cbh = crown_base_h_m.get(idx).copied().unwrap_or(4.0).max(0.5);
        let fmc = foliar_mc_pct.get(idx).copied().unwrap_or(95.0).clamp(30.0, 200.0);

        if i_b > 5.0 {
            // Active fire exposure: Van Wagner (1973) lethal scorch height h_s and (1977) critical I_0
            let denom = (0.025_574 * i_b + u_wind.powi(3)).sqrt() * (60.0 - t_amb).max(5.0);
            let h_s = (0.741_83 * i_b.powf(7.0 / 6.0)) / denom.max(1e-3);
            let i_0 = (0.010 * z_cbh * (460.0 + 25.9 * fmc)).powf(1.5);

            let crown_top = (z_cbh * 2.5).max(z_cbh + 2.0);
            stem.crown_scorch_frac = ((h_s - z_cbh) / (crown_top - z_cbh)).clamp(0.0, 1.0);
            if i_b >= i_0 {
                stem.crown_consumed_frac =
                    ((i_b - i_0) / i_0.max(100.0) + 0.5).clamp(0.5, 1.0);
            }

            // Outer bark flame heating & charring recession
            let flame_t = (t_amb + 18.0 * i_b.sqrt()).clamp(t_amb, 950.0);
            stem.surface_temp_c = flame_t;
            let char_rate = if stem.bark_type == 1 { 1.4e-5 } else { 0.7e-5 };
            if flame_t > 280.0 {
                let d_char = char_rate * ((flame_t - 280.0) / 400.0) * dt;
                stem.char_depth_m += d_char;
                stem.bark_thickness_m = (stem.bark_thickness_m - d_char * 0.4).max(0.001);
            }
            // Stringybark (bark_type == 1) sheds glowing firebrands under high convective intensity
            stem.ember_shed_rate = if stem.bark_type == 1 && i_b > 500.0 {
                0.0025 * (i_b / 1000.0)
            } else {
                0.0
            };

            // 1D radial heat conduction across remaining uncharred bark thickness x_b
            let xb = stem.bark_thickness_m.max(0.0015);
            let tau_cond = (xb * xb) / (2.0 * ALPHA_BARK_M2_S);
            stem.cambium_temp_c += (stem.surface_temp_c - stem.cambium_temp_c) * (dt / tau_cond).min(1.0);

            // First-order Arrhenius thermal injury to vascular cambium above 52°C
            if stem.cambium_temp_c > 52.0 {
                let injury_rate = 0.015 * ((stem.cambium_temp_c - 60.0) / 6.0).exp();
                stem.cambium_viability = (stem.cambium_viability * (-injury_rate * dt).exp()).clamp(0.0, 1.0);
            }
            stem.time_since_fire_s = 0.0;
        } else {
            // Post-fire cooling & epicormic / lignotuber resprouting phase
            stem.surface_temp_c += (t_amb - stem.surface_temp_c) * (dt * 0.05).min(1.0);
            stem.cambium_temp_c += (t_amb - stem.cambium_temp_c) * (dt * 0.02).min(1.0);
            stem.ember_shed_rate = 0.0;
            stem.time_since_fire_s += dt;

            let defol = stem.crown_scorch_frac.max(stem.crown_consumed_frac);
            let has_epicormic = (stem.strategy_flags & 1) != 0;
            let has_lignotuber = (stem.strategy_flags & 2) != 0;

            if defol > 0.40 && stem.cambium_viability > 0.50 && has_epicormic {
                // Trunk & bough "fuzzy tree" epicormic eruption
                stem.epicormic_vigor = (defol * stem.cambium_viability).clamp(0.0, 1.0);
                stem.shoot_length_m = (stem.shoot_length_m + 2.5e-7 * stem.epicormic_vigor * dt).min(1.8);
                resprouting_count += 1;
            } else if stem.cambium_viability <= 0.50 && stem.lignotuber_viability > 0.50 && has_lignotuber {
                // Aerial trunk killed -> basal lignotuber coppice multi-stem resprouting
                stem.shoot_length_m = (stem.shoot_length_m + 3.2e-7 * stem.lignotuber_viability * dt).min(2.5);
                resprouting_count += 1;
            }
        }
    }
    resprouting_count
}

/// Zero-heap serotinous *Banksia*/*Hakea* follicle thermal rupture & post-fire hygroscopic seed release stepper (20 Hz).
pub fn step_serotinous_follicles(
    pods: &mut [SerotinousFolliclePod],
    local_flame_temp_c: &[f32],
    rh_delta: f32,
    dt_s: f32,
    out_seeds_released: &mut [u32],
) -> u32 {
    let dt = dt_s.max(0.0);
    let mut total_released = 0u32;

    for (idx, pod) in pods.iter_mut().enumerate() {
        let t_ext = local_flame_temp_c.get(idx).copied().unwrap_or(20.0);
        // Retained dead florets ignite briefly to boost valve heating while woody shell insulates seed core
        let floret_boost = if t_ext > 250.0 && pod.floret_fuel_kg > 0.0 {
            pod.floret_fuel_kg = (pod.floret_fuel_kg - 0.002 * dt).max(0.0);
            120.0
        } else {
            0.0
        };
        pod.follicle_temp_c += ((t_ext + floret_boost) - pod.follicle_temp_c) * (dt * 0.12).min(1.0);
        pod.seed_core_temp_c += (pod.follicle_temp_c - pod.seed_core_temp_c) * (dt * 0.008).min(1.0);

        // Boltzmann sigmoid valve rupture above species opening threshold T_o (60..76 C)
        if pod.follicle_temp_c > pod.wax_melt_temp_c {
            let p_open = 1.0 / (1.0 + (-(pod.follicle_temp_c - pod.open_temp_to_c) / 2.2).exp());
            pod.open_fraction = pod.open_fraction.max(p_open);
        }

        // Lethal seed core check (>105 C kills internal seed embryo)
        if pod.seed_core_temp_c > 105.0 {
            pod.seeds_remaining = 0;
        }

        // Post-fire wet-dry humidity cycles (rh_delta > 0.15) ratchet winged seeds past separator plates
        let mut released_here = 0u32;
        if pod.open_fraction > 0.65 && pod.seeds_remaining > 0 && rh_delta.abs() > 0.15 {
            released_here = 1.min(pod.seeds_remaining);
            pod.seeds_remaining -= released_here;
            total_released += released_here;
        }
        if idx < out_seeds_released.len() {
            out_seeds_released[idx] = released_here;
        }
    }
    total_released
}

/// Evaluates Modified Byram convective number $N_c$, pyrocumulus (`pyroCu`) LCL blow-up, and DeBano hydrophobic ash.
pub fn evaluate_pyrocumulus_column_zero_heap(col: &mut PyroPlumeColumn, ambient_temp_k: f32) {
    let g = 9.80665f32;
    let rho_cp_t0 = 1.15 * 1005.0 * ambient_temp_k.clamp(260.0, 325.0);
    // Heat flux Q * S_0 from fireline intensity I_B (kW/m) over 100m segment length
    let total_heat_w = col.fireline_i_kw_m.max(0.0) * 1000.0 * 100.0;
    let f_buoy = (total_heat_w * g) / rho_cp_t0;
    col.buoyancy_flux_m4_s3 = f_buoy;

    let n_bv = col.brunt_vaisala_s_inv.max(0.005);
    let shear = col.wind_shear_s_inv.max(0.001);
    let length_scale = (f_buoy / n_bv.powi(3)).max(1.0).powf(0.25);
    let w_conv = ((2.0 * g * col.fireline_i_kw_m.max(1.0) * 1000.0) / rho_cp_t0).powf(1.0 / 3.0);
    col.byram_nc = w_conv / (length_scale * shear);
    col.plume_top_z_m = (1.29 * col.byram_nc.sqrt() * length_scale).min(5.0 * length_scale);
    col.updraft_max_m_s = w_conv * 1.35;
    col.ember_loft_max_m = col.plume_top_z_m * 0.72;

    let active_pyro_cu = col.plume_top_z_m >= col.lcl_height_m.max(500.0);
    if active_pyro_cu {
        col.pyro_cu_lwc_kg_m3 = ((col.plume_top_z_m - col.lcl_height_m) * 3.5e-7).clamp(0.0001, 0.004);
        col.plume_top_z_m *= 1.25; // Latent heat release amplifies plume top
    } else {
        col.pyro_cu_lwc_kg_m3 = 0.0;
    }
    col.ash_deposit_m = (col.fireline_i_kw_m * 4.5e-6).clamp(0.0, 0.12);
    col.subsurf_hydrophob = (col.fireline_i_kw_m / (col.fireline_i_kw_m + 1800.0)).clamp(0.0, 0.95);
    col.hydrophobic_depth_m = 0.025;
    col.plume_flags = u32::from(active_pyro_cu) | (u32::from(col.byram_nc > 10.0) << 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stringybark_insulates_cambium_for_epicormic_fuzzy_tree_resprouting_and_banksia_serotiny_opens() {
        let mut stems = [PyroBotanyStemState {
            bark_thickness_m: 0.032, // 3.2 cm insulating stringybark
            char_depth_m: 0.0,
            bark_density_kg_m3: 310.0,
            bark_moisture_frac: 0.18,
            surface_temp_c: 25.0,
            cambium_temp_c: 25.0,
            cambium_viability: 1.0,
            crown_scorch_frac: 0.0,
            crown_consumed_frac: 0.0,
            epicormic_vigor: 0.0,
            lignotuber_viability: 1.0,
            ember_shed_rate: 0.0,
            shoot_length_m: 0.0,
            time_since_fire_s: 0.0,
            bark_type: 1, // Stringybark
            strategy_flags: 1 | 2, // Epicormic + Lignotuber
        }];
        // 1. Expose to 1,500 kW/m surface fire for 30 seconds -> full crown scorch, cambium survives
        step_pyro_botany_stems(&mut stems, &[1500.0], &[4.0], &[90.0], 32.0, 3.0, 30.0);
        assert!(stems[0].crown_scorch_frac > 0.85);
        assert!(stems[0].cambium_viability > 0.75, "3.2cm stringybark must insulate vascular cambium");
        assert!(stems[0].ember_shed_rate > 0.0);

        // 2. Post-fire recovery step -> epicormic strands erupt along trunk
        let resprouts = step_pyro_botany_stems(&mut stems, &[0.0], &[4.0], &[90.0], 24.0, 2.0, 86_400.0);
        assert_eq!(resprouts, 1);
        assert!(stems[0].epicormic_vigor > 0.65);
        assert!(stems[0].shoot_length_m > 0.01);

        // 3. Serotinous Banksia follicle ruptures at flame passage and releases seeds on post-fire rain cycle
        let mut pods = [SerotinousFolliclePod {
            pos_local_m: [0.0, 2.5, 0.0],
            follicle_temp_c: 25.0,
            seed_core_temp_c: 25.0,
            wax_melt_temp_c: 50.0,
            open_temp_to_c: 70.0,
            open_fraction: 0.0,
            floret_fuel_kg: 0.015,
            seeds_remaining: 2,
            stem_index: 0,
            serotiny_class: 2,
        }];
        let mut released = [0u32; 1];
        step_serotinous_follicles(&mut pods, &[380.0], 0.0, 10.0, &mut released);
        assert!(pods[0].open_fraction > 0.90);
        assert!(pods[0].seed_core_temp_c < 95.0, "woody follicle must shield seed core from lethal heat");
        let count = step_serotinous_follicles(&mut pods, &[22.0], 0.45, 60.0, &mut released);
        assert_eq!(count, 1);
    }
}

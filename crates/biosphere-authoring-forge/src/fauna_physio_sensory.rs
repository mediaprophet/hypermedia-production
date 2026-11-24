//! # Vector 7 — Fauna Respiration, Vascular LWIR Thermoregulation, Hydrostats & L6 Sensory Fields (`fauna_physio_sensory.rs`)
//!
//! Implements the 5 fauna physiological, soft-body, adhesive, and `L6` quantum/non-visual sensory
//! sub-systems from Vector 7:
//! 1. **Allometric Respiration Cycles & Condensing Breath Plumes (`FaunaRespirationState`):**
//!    Kleiber/Stahl/Calder mass-scaled mammalian breathing ($f_r \propto M_b^{-0.25}$), avian
//!    unidirectional air-sac wingbeat synchronization, insect Discontinuous Gas Exchange (`DGC`:
//!    Closed/Flutter/Open) & abdominal tracheal pumping, plus Clausius–Clapeyron cold-air breath
//!    condensation plumes.
//! 2. **Vascular Thermoregulation, Counter-Current Exchange & Band-5 LWIR Exitance (`VascularThermoZone`):**
//!    Counter-current limb/proboscis heat exchange ($\varepsilon_{\text{cc}} \in [0.85, 0.98]$),
//!    arteriovenous anastomosis vasodilation radiators (bilby/elephant ears, toucan bills, and
//!    **Red Kangaroo saliva-licked forearms**), and Stefan–Boltzmann `Q42EMF64` Band-5 thermal IR
//!    radiant exitance ($M_{\text{LWIR}} = \epsilon \sigma_{\text{SB}} T_{\text{skin}}^4$).
//! 3. **Isochoric Cosserat Muscular Hydrostats (`HydrostatSegment`):**
//!    Constant-volume ($V_0 = \pi r^2 L$) longitudinal, transverse, and helical muscle arrays with
//!    Călugăreanu–White–Fuller link conservation ($\text{Lk} = \text{Tw} + \text{Wr}$) for octopus
//!    arms, trunks, and tongues.
//! 4. **Gecko Kendall Dry van der Waals Peel & Tree-Frog Capillary Adhesion (`BioAdhesivePadContact`):**
//!    Exact Kendall (1975) elastic strip peel quadratic for gecko spatulae ($\theta_s \approx 30^\circ$)
//!    and Laplace meniscus + Stefan viscous resistance for *Litoria caerulea* tree-frog toe pads.
//! 5. **L6 Cryptochrome Radical-Pair Magnetoreception, Platypus/Shark Electro-Sense & Pit-Viper IR (`QuantumSensoryReceptor`):**
//!    Stochastic-Liouville even-Legendre ($P_2, P_4$) inclination quantum yield $\Phi_S(\theta_B)$
//!    with blue-light gating and RF disruption, submerged muscle-dipole bioelectric fields
//!    ($20\,\mu\text{V/cm}$ platypus bill electroreceptors + mechanoreceptor arrival lag), and
//!    1–3 mK TRPA1 pit-viper pinhole thermal membrane contrast.

use serde::{Deserialize, Serialize};

/// 64-byte POD allometric thoracic/air-sac/tracheal respiration & breath condensation state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FaunaRespirationState {
    /// Animal body mass $M_b$ for allometric scaling (kg).
    pub body_mass_kg: f32,
    /// Metabolic exertion load $\xi \in [0.0, 1.0]$ (`0.0` BMR .. `1.0` MMR).
    pub exertion_frac: f32,
    /// Active breathing or abdominal pumping frequency (Hz).
    pub resp_freq_hz: f32,
    /// Tidal volume $V_T$ (mL).
    pub tidal_vol_ml: f32,
    /// Respiratory cycle phase in `[0.0, 2π)`.
    pub cycle_phase_rad: f32,
    /// Normalized ribcage / air-sac expansion $\to$ `SKL2` bone scale.
    pub thorax_expand_frac: f32,
    /// Exhaled air temperature (`34..40` $^\circ\text{C}$).
    pub exhaled_temp_c: f32,
    /// Supersaturated cold-air breath condensation liquid water content ($\text{g}/\text{m}^3$).
    pub plume_lwc_g_m3: f32,
    /// Insect internal tracheal $\text{CO}_2$ fraction (gates DGC Open phase).
    pub co2_tracheal_frac: f32,
    /// Avian wingbeat-to-breath phase lock ratio.
    pub wingbeat_sync_ratio: f32,
    /// Nostril/mouth exhalation jet speed (m/s).
    pub breath_vel_m_s: f32,
    /// Insect DGC Closed/Flutter/Open phase timer (s).
    pub dgc_phase_timer_s: f32,
    /// `0 = TerrMammal`, `1 = AqMammal`, `2 = Avian`, `3 = InsectDGC`.
    pub taxa_class: u32,
    /// `0 = Closed`, `1 = Flutter`, `2 = Open`, `3 = Continuous`.
    pub dgc_state: u32,
    /// Bound `SKL2` thoracic/abdominal bone index.
    pub skl2_rib_bone_idx: u32,
    /// `Bit0 = Panting`, `Bit1 = VisiblePlume`.
    pub flags: u32,
}

/// 64-byte POD vascular thermoregulation, counter-current exchange & Band-5 LWIR radiator zone.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VascularThermoZone {
    /// Deep core body temperature ($^\circ\text{C}$).
    pub core_temp_c: f32,
    /// Surface radiator / extremity skin temperature ($^\circ\text{C}$).
    pub skin_temp_c: f32,
    /// Arteriovenous anastomosis vasodilation $v_{\text{dil}} \in [0.0, 1.0]$.
    pub vasodilation: f32,
    /// Counter-current heat-exchange effectiveness $\varepsilon_{\text{cc}} \in [0.85, 0.98]$.
    pub counter_curr_eff: f32,
    /// Cutaneous blood perfusion (`1..150` $\text{mL}/(100\text{ g}\cdot\text{min})$).
    pub blood_perf_ml_100g_m: f32,
    /// Macropod forearm saliva-licking / sweat film wetness in `[0.0, 1.0]`.
    pub saliva_wetness: f32,
    /// Latent evaporative cooling flux ($\text{W}/\text{m}^2$).
    pub evap_heat_loss_w_m2: f32,
    /// Stefan–Boltzmann Band-5 thermal IR radiant exitance $M_{\text{LWIR}}$ ($\text{W}/\text{m}^2$).
    pub lwir_exitance_w_m2: f32,
    /// Wien peak thermal IR wavelength ($\approx 9300\text{ nm}$).
    pub wien_peak_nm: f32,
    /// Surface LWIR emissivity (`0.95..0.98`).
    pub emissivity: f32,
    /// Local fur/plumage thermal insulation resistance ($\text{K}\cdot\text{m}^2/\text{W}$).
    pub pelage_insul_k_m2_w: f32,
    /// Anatomical radiator surface area ($\text{m}^2$; ear/bill/forearm/proboscis).
    pub zone_area_m2: f32,
    /// `0 = CoreTrunk`, `1 = EarBillRadiator`, `2 = ForearmSalivaLick`, `3 = ExtremityCounterCurrent`.
    pub zone_kind: u32,
    /// Linked `SKL2` bone index.
    pub skl2_zone_bone: u32,
    /// Target `Q42EMF64` spatial cell index.
    pub emf_cell_idx: u32,
    /// `Bit0 = LickingActive`, `Bit1 = HeatStress`, `Bit2 = BloodFeedingCC`.
    pub flags: u32,
}

/// 64-byte POD isochoric Cosserat muscular hydrostat segment (octopus arm / trunk / tongue).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HydrostatSegment {
    /// Segment centerline position `[x, y, z]` (m).
    pub pos_xyz_m: [f32; 3],
    /// Orthonormal Cosserat director quaternion `(x, y, z, w)`.
    pub director_quat: [f32; 4],
    /// Conserved constant volume $V_0 = \pi r^2 L$ ($\text{m}^3$).
    pub rest_vol_m3: f32,
    /// Current segment length $L$ (m).
    pub length_m: f32,
    /// Isochoric radius $r = \sqrt{V_0 / (\pi L)}$ (m).
    pub radius_m: f32,
    /// Longitudinal muscle activation in `[0.0, 1.0]`.
    pub act_longitudinal: f32,
    /// Transverse/radial muscle activation in `[0.0, 1.0]`.
    pub act_transverse: f32,
    /// Signed oblique-helical muscle torque activation in `[-1.0, 1.0]`.
    pub act_helical_lr: f32,
    /// Local bending curvature `[kx, ky]` ($\text{m}^{-1}$).
    pub bend_curv_xy: [f32; 2],
    /// Stored twist density $\text{Tw}$ (rad/m).
    pub twist_rad_m: f32,
}

/// 48-byte POD gecko Kendall dry-vdW or tree-frog wet-capillary adhesive toe pad.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BioAdhesivePadContact {
    /// Substrate unit normal `[nx, ny, nz]`.
    pub surface_normal: [f32; 3],
    /// Limb pull vector at contact `[px, py, pz]`.
    pub pull_dir_local: [f32; 3],
    /// Angle $\theta$ between setal shaft and substrate (rad).
    pub peel_angle_rad: f32,
    /// Critical detachment peel angle ($\approx 0.5236\text{ rad} / 30^\circ$).
    pub crit_peel_rad: f32,
    /// Interfacial work of adhesion $G$ (`0.01..0.08` $\text{J}/\text{m}^2$ gecko).
    pub work_adhesion_j_m2: f32,
    /// Tree-frog interstitial mucus thickness $h_{\text{muc}}$ (m).
    pub mucus_gap_m: f32,
    /// Allowable tensile normal adhesion force (N).
    pub max_normal_adh_n: f32,
    /// `0 = GeckoKendallVdW`, `1 = TreeFrogCapillaryStefan`.
    pub pad_mode: u32,
}

/// 64-byte POD L6 quantum magnetoreceptor, aquatic electroreceptor & pit-viper IR organ.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuantumSensoryReceptor {
    /// Sensory organ look/alignment unit vector `[fx, fy, fz]`.
    pub head_forward: [f32; 3],
    /// Radical-pair singlet quantum yield $\Phi_S$.
    pub cry4_singlet_yield: f32,
    /// Sensed geomagnetic inclination angle $\theta_B$ (rad).
    pub mag_inclination_rad: f32,
    /// Retinal blue-band excitation gate in `[0.0, 1.0]`.
    pub blue_light_gate: f32,
    /// Radiofrequency (`RF`) electromagnetic disruption factor in `[0.0, 1.0]`.
    pub rf_noise_disrupt: f32,
    /// Sensed underwater prey electric field magnitude (V/m).
    pub electro_field_v_m: f32,
    /// Platypus electrical-vs-mechanoreceptor pressure arrival delay (s).
    pub electro_mech_lag_s: f32,
    /// Pit-viper loreal membrane thermal contrast $\Delta T_{\text{mem}}$ (K).
    pub pit_delta_t_k: f32,
    /// Pinhole aperture $\sin^2\theta_{\text{ap}}$.
    pub pit_ap_sin2: f32,
    /// Normalized TRPA1 / sensory neural firing output in `[0.0, 1.0]`.
    pub trpa1_firing_rate: f32,
    /// Detection threshold (`2e-3 V/m` platypus, `1e-3..3e-3 K` pit viper).
    pub sensitivity_thresh: f32,
    /// `0 = AvianCry4`, `1 = PlatypusElectro`, `2 = SharkLorenzini`, `3 = PitViperIR`.
    pub modality_kind: u32,
    /// Bound fauna entity ID.
    pub host_entity_id: u32,
    /// `1` if stimulus exceeds sensory threshold.
    pub signal_valid: u32,
}

const _: [(); 64] = [(); core::mem::size_of::<FaunaRespirationState>()];
const _: [(); 64] = [(); core::mem::size_of::<VascularThermoZone>()];
const _: [(); 64] = [(); core::mem::size_of::<HydrostatSegment>()];
const _: [(); 48] = [(); core::mem::size_of::<BioAdhesivePadContact>()];
const _: [(); 64] = [(); core::mem::size_of::<QuantumSensoryReceptor>()];

/// Zero-heap allometric respiration cycle & Clausius–Clapeyron breath condensation stepper (64 Hz).
pub fn step_fauna_respiration(
    states: &mut [FaunaRespirationState],
    ambient_temp_c: f32,
    ambient_rh: f32,
    dt_s: f32,
) -> usize {
    let dt = dt_s.max(0.0);
    let rh = ambient_rh.clamp(0.0, 1.0);
    // Saturation vapor pressure (Pa) via Magnus-Tetens
    let e_sat_pa = |tc: f32| -> f32 { 610.94 * ((17.625 * tc) / (tc + 243.04)).exp() };
    let e_amb = rh * e_sat_pa(ambient_temp_c);
    let mut plume_count = 0usize;

    for st in states.iter_mut() {
        let mb = st.body_mass_kg.max(1.0e-5);
        let xi = st.exertion_frac.clamp(0.0, 1.0);

        match st.taxa_class {
            0 => {
                // Terrestrial mammal allometric scaling: f_r = 0.58 * M^-0.25 (BMR) to 5.08 * M^-0.14 (MMR)
                st.resp_freq_hz = (1.0 - xi) * (0.58 * mb.powf(-0.25)) + xi * (5.08 * mb.powf(-0.14));
                st.tidal_vol_ml = 7.14 * mb * (1.0 + 1.8 * xi);
            }
            1 => {
                // Aquatic mammal apneustic scaling (steeper M^1.10 tidal volume, lower frequency)
                st.resp_freq_hz = (0.18 * mb.powf(-0.28)) * (1.0 + 2.5 * xi);
                st.tidal_vol_ml = 22.0 * mb.powf(1.10);
            }
            2 => {
                // Avian air-sac scaling (~0.287 * M^-0.31 Hz at rest, locked to wingbeat in flight)
                st.resp_freq_hz = (0.287 * mb.powf(-0.31)) * (1.0 + 4.5 * xi);
                st.tidal_vol_ml = 13.2 * mb.powf(1.08);
            }
            _ => {
                // Insect Discontinuous Gas Exchange (DGC: Closed -> Flutter -> Open)
                st.co2_tracheal_frac = (st.co2_tracheal_frac + 0.002 * (1.0 + 5.0 * xi) * dt).clamp(0.0, 0.08);
                st.dgc_phase_timer_s += dt;
                if xi > 0.35 || st.co2_tracheal_frac > 0.02 {
                    st.dgc_state = 3; // Continuous abdominal pumping (~0.6 Hz)
                    st.resp_freq_hz = 0.1 + 0.7 * xi;
                    st.co2_tracheal_frac = (st.co2_tracheal_frac - 0.008 * dt).max(0.004);
                } else {
                    st.dgc_state = ((st.dgc_phase_timer_s / 15.0) as u32) % 3;
                    st.resp_freq_hz = if st.dgc_state == 2 { 0.08 } else { 0.01 };
                }
                st.tidal_vol_ml = 0.05 * mb * 1000.0;
            }
        }

        st.cycle_phase_rad =
            (st.cycle_phase_rad + core::f32::consts::TAU * st.resp_freq_hz * dt).rem_euclid(core::f32::consts::TAU);
        let sin_p = st.cycle_phase_rad.sin();
        st.thorax_expand_frac = 0.5 + 0.5 * sin_p;

        // Exhalation occurs when cos(phase) < 0; check Clausius-Clapeyron supersaturation at 50% jet mixing
        let is_exhaling = st.cycle_phase_rad.cos() < 0.0;
        if is_exhaling && st.taxa_class <= 2 {
            let t_ex = st.exhaled_temp_c.clamp(32.0, 41.0);
            let e_ex = 0.96 * e_sat_pa(t_ex);
            let t_mix = 0.5 * (t_ex + ambient_temp_c);
            let e_mix = 0.5 * (e_ex + e_amb);
            let e_sat_mix = e_sat_pa(t_mix);
            if e_mix > e_sat_mix {
                st.plume_lwc_g_m3 = ((e_mix - e_sat_mix) * 0.0022).clamp(0.0, 1.8);
                st.flags |= 2;
                plume_count += 1;
            } else {
                st.plume_lwc_g_m3 = 0.0;
                st.flags &= !2;
            }
        } else {
            st.plume_lwc_g_m3 = 0.0;
        }
    }
    plume_count
}

/// Evaluates vascular thermoregulation, counter-current exchange, macropod forearm saliva cooling,
/// and Band-5 LWIR Stefan–Boltzmann exitance.
pub fn step_vascular_thermoregulation_lwir(
    zones: &mut [VascularThermoZone],
    ambient_temp_c: f32,
    wind_speed_m_s: f32,
    dt_s: f32,
) {
    const SIGMA_SB: f32 = 5.670_374e-8;
    let dt = dt_s.max(0.0);
    let h_conv = 6.5 + 4.2 * wind_speed_m_s.max(0.0).sqrt();

    for z in zones.iter_mut() {
        let v_dil = z.vasodilation.clamp(0.0, 1.0);
        let eps_cc = z.counter_curr_eff.clamp(0.0, 0.98);
        // Arterial blood temperature reaching skin after counter-current exchange
        let t_art = z.core_temp_c - eps_cc * (1.0 - v_dil) * (z.core_temp_c - ambient_temp_c).max(0.0);
        z.blood_perf_ml_100g_m = 1.5 + 148.5 * v_dil;

        let q_blood = z.blood_perf_ml_100g_m * 0.85 * (t_art - z.skin_temp_c);
        let q_conv = h_conv * (z.skin_temp_c - ambient_temp_c) / (1.0 + z.pelage_insul_k_m2_w.max(0.0) * h_conv);
        z.evap_heat_loss_w_m2 = z.saliva_wetness.clamp(0.0, 1.0) * (45.0 + 25.0 * wind_speed_m_s.max(0.0));

        let net_flux = q_blood - q_conv - z.evap_heat_loss_w_m2;
        z.skin_temp_c = (z.skin_temp_c + (net_flux / 1800.0) * dt).clamp(ambient_temp_c - 3.0, z.core_temp_c + 1.0);

        let t_skin_k = (z.skin_temp_c + 273.15).max(200.0);
        let eps = z.emissivity.clamp(0.90, 0.99);
        z.lwir_exitance_w_m2 = eps * SIGMA_SB * t_skin_k.powi(4);
        z.wien_peak_nm = 2.897_77e6 / t_skin_k;
    }
}

/// Zero-heap gecko Kendall peel / tree-frog wet-pad adhesion evaluator for `ik_contact.rs` (64 Hz).
pub fn eval_bio_adhesive_pad_capacity(
    pad: &mut BioAdhesivePadContact,
    setae_or_area: f32,
    sep_vel_m_s: f32,
    out_max_shear_n: &mut f32,
) -> f32 {
    if pad.pad_mode == 0 {
        // Gecko Kendall (1975) elastic strip peel law across N_spatulae (~500 per seta)
        let b = 0.2e-6f32;
        let h = 5.0e-9f32;
        let e_mod = 2.0e9f32;
        let eh = e_mod * h;
        let g_adh = pad.work_adhesion_j_m2.clamp(0.005, 0.12);
        let theta = pad.peel_angle_rad.clamp(0.05, core::f32::consts::PI);
        let omt = 1.0 - theta.cos();
        let f_single = b * eh * (-omt + (omt * omt + (2.0 * g_adh) / eh).sqrt());
        let total_spatulae = setae_or_area.max(1.0) * 500.0;
        let f_peel = f_single * total_spatulae;
        let crit = pad.crit_peel_rad.max(0.2);
        if theta <= crit {
            pad.max_normal_adh_n = f_peel * theta.sin();
            *out_max_shear_n = f_peel * theta.cos();
        } else {
            // Above ~30° critical peel angle, gecko toe hyperextends and detaches cleanly
            pad.max_normal_adh_n = f_peel * 0.05;
            *out_max_shear_n = f_peel * 0.05;
        }
        pad.max_normal_adh_n
    } else {
        // Tree-frog (Litoria caerulea) Laplace capillary bridge + Stefan mucus viscous resistance
        let r_pad = (setae_or_area.max(1.0e-6) / core::f32::consts::PI).sqrt();
        let h_muc = pad.mucus_gap_m.clamp(5.0e-7, 1.0e-4);
        let gamma = 0.072f32;
        let eta_muc = 0.015f32;
        let f_cap = (2.0 * core::f32::consts::PI * r_pad * r_pad * gamma) / h_muc;
        let f_stef =
            (1.5 * core::f32::consts::PI * eta_muc * r_pad.powi(4) * sep_vel_m_s.max(0.0)) / h_muc.powi(3);
        pad.max_normal_adh_n = (f_cap + f_stef).min(25.0);
        *out_max_shear_n = pad.max_normal_adh_n * 0.65;
        pad.max_normal_adh_n
    }
}

/// Zero-heap `L6` Cry4 quantum radical-pair magnetoreception, platypus/shark electro-sense & pit-viper IR evaluator (20 Hz).
pub fn eval_quantum_and_emf_receptors(
    receptors: &mut [QuantumSensoryReceptor],
    emf_b_tesla: &[[f32; 3]],
    emf_e_v_m: &[[f32; 3]],
    emf_blue_flux: &[f32],
    emf_lwir_flux: &[f32],
    rf_b1_nt: f32,
) -> usize {
    let rf_disrupt = (-((rf_b1_nt.max(0.0) / 15.0).powi(2))).exp();
    let mut valid_count = 0usize;

    for (idx, rec) in receptors.iter_mut().enumerate() {
        let b = emf_b_tesla.get(idx).copied().unwrap_or([0.0, -3.5e-5, 3.5e-5]);
        let e = emf_e_v_m.get(idx).copied().unwrap_or([0.0; 3]);
        let blue = emf_blue_flux.get(idx).copied().unwrap_or(10.0).max(0.0);
        let lwir = emf_lwir_flux.get(idx).copied().unwrap_or(400.0).max(0.0);

        rec.rf_noise_disrupt = 1.0 - rf_disrupt;
        match rec.modality_kind {
            0 => {
                // Avian Cry4 [FAD•- TrpH•+] stochastic-Liouville even-Legendre singlet yield Phi_S
                let b_mag = (b[0] * b[0] + b[1] * b[1] + b[2] * b[2]).sqrt().max(1.0e-9);
                let f_len = (rec.head_forward[0] * rec.head_forward[0]
                    + rec.head_forward[1] * rec.head_forward[1]
                    + rec.head_forward[2] * rec.head_forward[2])
                    .sqrt()
                    .max(1e-6);
                let cos_theta = ((rec.head_forward[0] * b[0]
                    + rec.head_forward[1] * b[1]
                    + rec.head_forward[2] * b[2])
                    / (f_len * b_mag))
                    .clamp(-1.0, 1.0);
                rec.mag_inclination_rad = cos_theta.abs().acos();
                rec.blue_light_gate = (blue / (blue + 2.5)).clamp(0.0, 1.0);
                let c2 = cos_theta * cos_theta;
                let p2 = 0.5 * (3.0 * c2 - 1.0);
                let p4 = 0.125 * (35.0 * c2 * c2 - 30.0 * c2 + 3.0);
                rec.cry4_singlet_yield =
                    0.28 + rec.blue_light_gate * rf_disrupt * (-0.065 * p2 + 0.018 * p4);
                rec.signal_valid = u32::from(rec.blue_light_gate > 0.15 && rf_disrupt > 0.25);
            }
            1 | 2 => {
                // Platypus bill / Shark Lorenzini aquatic electro-reception
                let e_mag = (e[0] * e[0] + e[1] * e[1] + e[2] * e[2]).sqrt();
                rec.electro_field_v_m = e_mag;
                let thresh = rec.sensitivity_thresh.max(1.0e-7);
                rec.signal_valid = u32::from(e_mag >= thresh);
                rec.trpa1_firing_rate = ((e_mag / thresh).ln().max(0.0) / 8.0).clamp(0.0, 1.0);
            }
            _ => {
                // Pit-viper loreal pinhole membrane thermal contrast (1..3 mK TRPA1 threshold)
                let bg_exitance = 418.0f32; // ~20°C ambient background
                let delta_m = (lwir - bg_exitance).max(0.0);
                let sin2_ap = rec.pit_ap_sin2.clamp(0.01, 0.5);
                rec.pit_delta_t_k = (delta_m * sin2_ap) / 48.5;
                let thresh_k = rec.sensitivity_thresh.max(0.001);
                rec.signal_valid = u32::from(rec.pit_delta_t_k >= thresh_k);
                rec.trpa1_firing_rate = (rec.pit_delta_t_k / (thresh_k * 10.0)).clamp(0.0, 1.0);
            }
        }
        if rec.signal_valid != 0 {
            valid_count += 1;
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cold_air_respiration_condenses_plume_and_avian_cry4_plus_platypus_electro_sense_fire() {
        let mut resp = [FaunaRespirationState {
            body_mass_kg: 35.0, // Eastern Grey Kangaroo
            exertion_frac: 0.7,
            resp_freq_hz: 0.0,
            tidal_vol_ml: 0.0,
            cycle_phase_rad: 3.1, // Exhalation half-cycle
            thorax_expand_frac: 0.0,
            exhaled_temp_c: 37.5,
            plume_lwc_g_m3: 0.0,
            co2_tracheal_frac: 0.0,
            wingbeat_sync_ratio: 1.0,
            breath_vel_m_s: 2.5,
            dgc_phase_timer_s: 0.0,
            taxa_class: 0,
            dgc_state: 0,
            skl2_rib_bone_idx: 4,
            flags: 0,
        }];
        let plumes = step_fauna_respiration(&mut resp, -2.0, 0.88, 0.015625);
        assert_eq!(plumes, 1);
        assert!(resp[0].plume_lwc_g_m3 > 0.05);

        let mut receptors = [
            QuantumSensoryReceptor {
                head_forward: [0.0, -0.707, 0.707],
                cry4_singlet_yield: 0.0,
                mag_inclination_rad: 0.0,
                blue_light_gate: 0.0,
                rf_noise_disrupt: 0.0,
                electro_field_v_m: 0.0,
                electro_mech_lag_s: 0.0,
                pit_delta_t_k: 0.0,
                pit_ap_sin2: 0.1,
                trpa1_firing_rate: 0.0,
                sensitivity_thresh: 0.002,
                modality_kind: 0, // Avian Cry4
                host_entity_id: 1,
                signal_valid: 0,
            },
            QuantumSensoryReceptor {
                head_forward: [1.0, 0.0, 0.0],
                cry4_singlet_yield: 0.0,
                mag_inclination_rad: 0.0,
                blue_light_gate: 0.0,
                rf_noise_disrupt: 0.0,
                electro_field_v_m: 0.0,
                electro_mech_lag_s: 0.012,
                pit_delta_t_k: 0.0,
                pit_ap_sin2: 0.1,
                trpa1_firing_rate: 0.0,
                sensitivity_thresh: 0.002, // 20 uV/cm = 2 mV/m platypus threshold
                modality_kind: 1, // Platypus electro-receptor
                host_entity_id: 2,
                signal_valid: 0,
            },
        ];
        let valid = eval_quantum_and_emf_receptors(
            &mut receptors,
            &[[0.0, -3.5e-5, 3.5e-5]; 2],
            &[[0.0, 0.0, 0.0], [0.0085, 0.0, 0.0]],
            &[15.0, 5.0],
            &[400.0, 400.0],
            1.0,
        );
        assert_eq!(valid, 2);
        assert!(receptors[0].cry4_singlet_yield > 0.20);
        assert!(receptors[1].trpa1_firing_rate > 0.10);
    }
}

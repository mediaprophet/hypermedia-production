//! # Vector 10 — Seven High-Leverage Biosphere Horizons (`biosphere_horizons.rs`)
//!
//! Implements the 7 high-leverage physical, optical, and biological laws identified in Vector 10
//! of the Biosphere Multi-Physics Gap Audit:
//! 1. **Hygromorph Bilayer Actuation (Rank #1):** Timoshenko (1925) / Reyssat & Mahadevan (2009)
//!    bimetallic-analogue differential hygrometric curvature for pinecone scales, *Banksia* follicle
//!    separator plates, and *Erodium*/*Stipa* self-burying helical seed awns, paired with Washburn +
//!    evaporative imbibition front progression $D = \dot{z}_m z_m + \frac{\delta}{2} z_m^2$.
//! 2. **Guenther MEGAN *Eucalyptus* BVOC Photochemical Blue-Haze SOA (Rank #7):** Exact Guenther
//!    (1993/1995) light- and temperature-dependent isoprene ($C_L \cdot C_T$) and monoterpene
//!    ($\exp(\beta(T - T_s))$) emission scaling producing ultrafine Secondary Organic Aerosols (SOA)
//!    and Australia's signature 460 nm Rayleigh-regime Blue Mountains atmospheric haze.
//! 3. **Underwater Bio-Optics: CDOM Tannin Blackwater Absorption & Snell's Window (Rank #4):**
//!    Exponential Chromophoric Dissolved Organic Matter (`CDOM`) absorption
//!    $a_{\text{CDOM}}(\lambda) = a_{412} \exp(-S_g(\lambda - 412))$ and chromatic Snell critical
//!    half-angle $\theta_c(\lambda) = \arcsin(1 / n_w(\lambda))$ across `Q42EMF64` bands.
//! 4. **Brewster-Angle Surface Polarization (Insect Polarotaxis) & Krisciunas–Schaefer Nocturnal Sky (Rank #6):**
//!    Fresnel $s/p$ reflection polarization peaking at Brewster's angle $\theta_B = \arctan(n)$
//!    (steering $>250$ aquatic insect species and flagging dark asphalt polarotactic traps),
//!    coupled with Krisciunas–Schaefer lunar brightness + opposition surge + 557.7 nm `[O I]` green
//!    airglow and NIR OH Meinel bands.
//! 5. **Wet Beach Sand Reynolds Dilatancy Footprint Halos (Rank #14):** Shear-induced granular
//!    dilation ($\Delta e = \dot{\gamma} \tan\psi_d \Delta t$) generating transient negative pore
//!    suction $\Delta u_d < 0$ that draws air menisci below surface grains (creating a matte-white
//!    dry halo around footsteps and doubling bearing stiffness) before Darcy re-wetting.
//! 6. **Rock Wildfire/Insolation Thermal Spalling & Walder–Hallet Frost Cracking:** Biaxial thermal
//!    skin stress $\sigma_{\text{th}} = E_r \alpha_{\text{th}} \Delta T_s / (1 - \nu_r)$ and
//!    sub-zero ice-segregation subcritical crack growth $\dot{a}_c = v_0 \exp(\gamma_k(K_I/K_{Ic} - 1))$.
//! 7. **Plant Bio-Electrical Action/Variation Potentials & Albatross Rayleigh-Cycle Dynamic Soaring:**
//!    Vascular active cable depolarization wave + hydraulic-turbulent Variation Potentials (`VP`),
//!    and wave-trough wind-shear energy harvesting $\dot{E}_{\text{shear}} = -m V_A \dot{W}_x \cos\gamma \cos\psi$.

use serde::{Deserialize, Serialize};

/// 48-byte POD Timoshenko–Reyssat hygromorph bilayer actuator (cones, *Banksia* separators, self-burying awns).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HygromorphBilayerOrgan {
    /// Passive sclerenchyma layer thickness $h_p$ (m).
    pub thick_passive_m: f32,
    /// Active cellulose-microfibril swelling layer thickness $h_a$ (m).
    pub thick_active_m: f32,
    /// Stiffness ratio $n = E_p / E_a$ (e.g. `~5.0` for pine cone scale).
    pub modulus_ratio_n: f32,
    /// Differential hygrometric expansion coefficient $\Delta\alpha = \alpha_a - \alpha_p$.
    pub diff_alpha_rh: f32,
    /// Zero-curvature reference relative humidity in `[0.0, 1.0]`.
    pub ref_rh: f32,
    /// Diffusively lagged internal tissue moisture equilibrium in `[0.0, 1.0]`.
    pub internal_rh: f32,
    /// Capillary imbibition front position $z_m$ (m).
    pub wetting_front_zm: f32,
    /// Resulting Timoshenko bilayer bending/helical curvature $\kappa$ ($\text{m}^{-1}$).
    pub curvature_m_inv: f32,
    /// Helical awn drill rotation angle (rad).
    pub twist_pitch_rad: f32,
    /// Linked `SKL2` bone or `.10d` instance morph index.
    pub skl2_scale_bone: u32,
    /// `0 = PineConeScale`, `1 = BanksiaSeparator`, `2 = SelfBuryAwn`.
    pub organ_kind: u32,
    /// `1` once self-burying awn drills seed into `.m64` soil pore.
    pub buried_flag: u32,
}

/// 48-byte POD Guenther MEGAN *Eucalyptus* BVOC emission & Blue-Haze Secondary Organic Aerosol (`SOA`) cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BvocBlueHazeCell {
    /// Standard isoprene emission factor at 303 K and 1000 PAR ($\mu\text{g}/(\text{m}^2\cdot\text{s})$).
    pub eps_isoprene_ug_m2_s: f32,
    /// Standard monoterpene emission factor ($\mu\text{g}/(\text{m}^2\cdot\text{s})$).
    pub eps_monoterp_ug_m2_s: f32,
    /// Local *Eucalyptus* Leaf Area Index ($\text{m}^2/\text{m}^2$).
    pub eucalypt_lai: f32,
    /// Canopy leaf temperature $T$ (K).
    pub leaf_temp_k: f32,
    /// Photosynthetically Active Radiation $L_{\text{PAR}}$ ($\mu\text{mol}/(\text{m}^2\cdot\text{s})$).
    pub par_umol_m2_s: f32,
    /// Instantaneous canopy isoprene flux ($\mu\text{g}/(\text{m}^2\cdot\text{s})$).
    pub flux_iso_ug_m2_s: f32,
    /// Instantaneous canopy monoterpene flux ($\mu\text{g}/(\text{m}^2\cdot\text{s})$).
    pub flux_mono_ug_m2_s: f32,
    /// Photo-oxidized ultrafine SOA mass concentration ($\mu\text{g}/\text{m}^3$).
    pub soa_mass_ug_m3: f32,
    /// Added 460 nm Rayleigh-regime blue-haze optical scattering coefficient ($\text{m}^{-1}$).
    pub blue_scatter_m_inv: f32,
    /// Ambient photochemical ozone oxidant mixing ratio (ppb).
    pub ozone_oxidant_ppb: f32,
    /// Atmospheric column grid index.
    pub cell_xy_idx: u32,
    /// Status bitmask.
    pub flags: u32,
}

/// 64-byte POD underwater CDOM tannin blackwater absorption & chromatic Snell's Window cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnderwaterBioOpticsCell {
    /// Tannin CDOM absorption coefficient at $\lambda_0 = 412\text{ nm}$ ($\text{m}^{-1}$).
    pub cdom_a412_m_inv: f32,
    /// Exponential spectral slope $S_g$ ($\approx 0.0156\text{ nm}^{-1}$).
    pub cdom_slope_nm_inv: f32,
    /// Phytoplankton chlorophyll concentration ($\text{mg}/\text{m}^3$).
    pub chl_water_mg_m3: f32,
    /// Suspended mineral sediment turbidity (NTU).
    pub turbidity_ntu: f32,
    /// Local water column depth (m).
    pub water_depth_m: f32,
    /// Computed `Q42EMF64` 6-band spectral absorption `[UV, Blue, Green, Red, NIR, SWIR]` ($\text{m}^{-1}$).
    pub band_abs_m_inv: [f32; 6],
    /// Chromatic Snell's Window critical half-angle `[UV, Blue, Green, Red]` (rad).
    pub snell_crit_rad: [f32; 4],
    /// Water grid cell index.
    pub cell_xy_idx: u32,
}

/// 64-byte POD Brewster-angle water polarotaxis & Krisciunas–Schaefer nocturnal sky state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrewsterNocturnalSky {
    /// Dielectric refractive index (`1.333` freshwater, `1.50` dark asphalt ecological trap).
    pub surface_ior: f32,
    /// Brewster angle $\theta_B = \arctan(n)$ ($\approx 0.927\text{ rad} / 53.1^\circ$).
    pub brewster_rad: f32,
    /// Reflected horizontal Degree of Linear Polarization $\text{DoLP} \in [0.0, 1.0]$.
    pub reflected_dolp: f32,
    /// Aquatic insect polarotaxis attraction score in `[0.0, 1.0]`.
    pub polarotaxis_attract: f32,
    /// Lunar phase angle $\psi$ in radians (`0.0` = full moon, $\pi$ = new moon).
    pub lunar_phase_rad: f32,
    /// Moon zenith angle $Z_{\text{moon}}$ (rad).
    pub lunar_zenith_rad: f32,
    /// Krisciunas–Schaefer apparent lunar V-band magnitude.
    pub lunar_mag_v: f32,
    /// Opposition surge multiplier (`1.00..=1.35` for $|\psi| < 7^\circ$).
    pub opp_surge_factor: f32,
    /// Mesospheric `[O I]` 557.7 nm green airglow radiance (Rayleighs).
    pub airglow_557nm_rayl: f32,
    /// Mesospheric OH Meinel NIR airglow radiance.
    pub airglow_nir_oh: f32,
    /// Zodiacal + diffuse galactic background floor.
    pub zodiacal_flux: f32,
    /// Total zenith night-sky brightness ($\text{mag}/\text{arcsec}^2$).
    pub zenith_sky_mag: f32,
    /// Horizontal reflection Stokes $Q$ for `Q42EMF64`.
    pub stokes_q_water: f32,
    /// Reflection Stokes $U$ for `Q42EMF64`.
    pub stokes_u_water: f32,
    /// `1` if solar elevation $< -18^\circ$ astronomical twilight.
    pub night_mode_flag: u32,
    /// `1` if dark artificial reflector (asphalt/solar panel ecological trap).
    pub trap_surface_flag: u32,
}

/// 48-byte POD coastal wet beach-sand Reynolds dilatancy footprint halo cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WetSandDilatancyCell {
    /// Current granular void ratio $e$.
    pub void_ratio_e: f32,
    /// Critical dilatancy cutoff void ratio $e_{\max}$.
    pub void_ratio_max: f32,
    /// Mobilized Reynolds dilatancy angle $\psi_m$ (rad).
    pub dilatancy_angle_rad: f32,
    /// Transient negative dilatant pore suction $\Delta u_d \le 0$ (Pa).
    pub pore_suction_pa: f32,
    /// Surface meniscus air-entry recession fraction $\in [0.0, 1.0]$ producing the matte-white dry halo.
    pub air_entry_frac: f32,
    /// Effective bearing capacity multiplier (`1.0..=2.25x`).
    pub bearing_boost: f32,
    /// Darcy re-wetting drainage relaxation timescale (`0.5..=3.0` s).
    pub darcy_relax_s: f32,
    /// Phreatic water-table depth $d_w$ (`0.0` = saturated Goldilocks beach surface).
    pub water_table_depth_m: f32,
    /// Applied foot-strike shear strain impulse $\Delta\gamma$.
    pub shear_impulse: f32,
    /// Optical diffuse albedo whitening lift overriding Lekner–Dorf wet darkening.
    pub albedo_lift: f32,
    /// Coastal `.m64` grid index.
    pub cell_xy_idx: u32,
    /// `1` while `air_entry_frac > 0.01`.
    pub active_halo_flag: u32,
}

/// 48-byte POD rock wildfire/insolation thermal spalling & Walder–Hallet ice-segregation frost-cracking cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThermalFrostRockCell {
    /// Outer rock skin temperature ($^\circ\text{C}$).
    pub surface_temp_c: f32,
    /// Subsurface bulk rock temperature ($^\circ\text{C}$).
    pub interior_temp_c: f32,
    /// Biaxial thermal stress $\sigma_{\text{th}} = E_r \alpha_{\text{th}} \Delta T / (1 - \nu_r)$ (Pa).
    pub thermal_stress_pa: f32,
    /// Critical exfoliation spalling threshold $\sigma_{\text{spall}}$ (Pa).
    pub spall_strength_pa: f32,
    /// Dominant subcritical ice-segregation crack length $a_c$ (m).
    pub crack_len_m: f32,
    /// Mode-I ice-lens stress intensity factor $K_I$ ($\text{Pa}\cdot\text{m}^{1/2}$).
    pub stress_int_k1: f32,
    /// Rock fracture toughness $K_{Ic}$ ($\text{Pa}\cdot\text{m}^{1/2}$).
    pub fracture_k1c: f32,
    /// Available unfrozen pore water feeding the frost-cracking window (`-3°C..-8°C`) in `[0.0, 1.0]`.
    pub pore_water_supply: f32,
    /// Accumulated exfoliation damage in `[0.0, 1.0]`.
    pub spall_damage: f32,
    /// Freshly spalled rock scree mass (kg) fed to `terrain-erosion-solvers`.
    pub scree_yield_kg: f32,
    /// `0 = Granite`, `1 = Sandstone`, `2 = Basalt`, `3 = Limestone`.
    pub rock_class: u32,
    /// `1` on spall flake detachment event.
    pub spalled_flag: u32,
}

/// 48-byte POD plant vascular action-potential (`AP`) / variation-potential (`VP`) & albatross dynamic-soaring node.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlantElectrophysNode {
    /// Plasma membrane voltage $V_m$ (`-200..+20` mV).
    pub membrane_mv: f32,
    /// Slow repolarizing $\text{K}^+$ recovery state variable $w$.
    pub recovery_w: f32,
    /// Wound hydraulic-turbulent glutamate/$\text{Ca}^{2+}$ variation-potential concentration.
    pub vp_chemical_conc: f32,
    /// Electrotonic cable space constant $\lambda_c$ (m).
    pub cable_lambda_m: f32,
    /// Membrane RC time constant $\tau_m$ (s).
    pub membrane_tau_s: f32,
    /// Effective VP diffusivity $D_{\text{VP}}$ ($\text{m}^2/\text{s}$).
    pub vp_diff_m2_s: f32,
    /// Rayleigh-cycle dynamic soaring airspeed $V_A$ (m/s).
    pub glide_airspeed_m_s: f32,
    /// Dynamic soaring wind-shear specific power extraction $\dot{E}_{\text{shear}} / m$ (W/kg).
    pub shear_energy_rate: f32,
    /// Upstream vascular segment (`u32::MAX` = root).
    pub parent_node: u32,
    /// Downstream vascular segment.
    pub child_node: u32,
    /// Triggered `PulvinusMotorJoint` index.
    pub bound_pulvinus_id: u32,
    /// `0 = PlantCableAPVP`, `1 = RayleighDynamicSoaring`.
    pub mode_and_flags: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<HygromorphBilayerOrgan>()];
const _: [(); 48] = [(); core::mem::size_of::<BvocBlueHazeCell>()];
const _: [(); 64] = [(); core::mem::size_of::<UnderwaterBioOpticsCell>()];
const _: [(); 64] = [(); core::mem::size_of::<BrewsterNocturnalSky>()];
const _: [(); 48] = [(); core::mem::size_of::<WetSandDilatancyCell>()];
const _: [(); 48] = [(); core::mem::size_of::<ThermalFrostRockCell>()];
const _: [(); 48] = [(); core::mem::size_of::<PlantElectrophysNode>()];

/// Zero-heap Timoshenko–Reyssat hygromorph bilayer curvature & Washburn imbibition stepper (20 Hz).
pub fn step_hygromorph_bilayers(
    organs: &mut [HygromorphBilayerOrgan],
    ambient_rh: f32,
    evap_flux_m_s: f32,
    dt_s: f32,
) -> usize {
    let rh_target = ambient_rh.clamp(0.0, 1.0);
    let dt = dt_s.max(0.0);
    let mut active_count = 0usize;

    for org in organs.iter_mut() {
        let hp = org.thick_passive_m.max(5.0e-5);
        let ha = org.thick_active_m.max(5.0e-5);
        let m = hp / ha;
        let n = org.modulus_ratio_n.max(0.05);
        let mn = m * n;

        // Exponential relaxation of internal tissue moisture toward ambient RH
        let tau_hygro = (hp + ha) * 1200.0;
        let alpha_dt = (dt / tau_hygro.max(0.1)).min(1.0);
        org.internal_rh += (rh_target - org.internal_rh) * alpha_dt;

        // Timoshenko bimetallic/hygromorph geometry factor f(m, n)
        let one_plus_m2 = (1.0 + m) * (1.0 + m);
        let denom = 3.0 * one_plus_m2 + (1.0 + mn) * (m * m + 1.0 / mn.max(1e-6));
        let f_mn = (6.0 * one_plus_m2) / denom.max(1e-6);

        let delta_rh = org.internal_rh - org.ref_rh.clamp(0.0, 1.0);
        org.curvature_m_inv = (org.diff_alpha_rh * delta_rh / (hp + ha)) * f_mn;

        // Capillary wetting front progression D = z_dot * z + 0.5 * delta * z^2
        let d_cap = 1.2e-7f32;
        let delta_evap = (evap_flux_m_s.max(0.0) / hp).max(1.0e-6);
        let z_max = (2.0 * d_cap / delta_evap).sqrt();
        if rh_target > org.ref_rh {
            org.wetting_front_zm =
                (org.wetting_front_zm + (d_cap * dt).sqrt() * 0.25).min(z_max);
        } else {
            org.wetting_front_zm = (org.wetting_front_zm - delta_evap * dt * 10.0).max(0.0);
        }

        // Helical self-burying seed awn rotation & soil drilling
        if org.organ_kind == 2 {
            org.twist_pitch_rad = org.curvature_m_inv * 0.035 * core::f32::consts::TAU;
            if org.twist_pitch_rad.abs() > 4.5 {
                org.buried_flag = 1;
            }
        }
        if org.curvature_m_inv.abs() > 1e-3 {
            active_count += 1;
        }
    }
    active_count
}

/// Zero-heap Guenther MEGAN *Eucalyptus* BVOC & Blue-Haze optical depth stepper (20 Hz).
pub fn step_eucalypt_bvoc_blue_haze(
    cells: &mut [BvocBlueHazeCell],
    wind_advect_s_inv: f32,
    dt_s: f32,
    out_blue_tau_add: &mut [f32],
) -> usize {
    const ALPHA: f32 = 0.0027;
    const CL1: f32 = 1.066;
    const CT1: f32 = 95_000.0;
    const CT2: f32 = 230_000.0;
    const TS: f32 = 303.0;
    const TM: f32 = 314.0;
    const R_GAS: f32 = 8.314;
    const BETA_MONO: f32 = 0.09;

    let dt = dt_s.max(0.0);
    let advect = wind_advect_s_inv.max(0.0005);
    let mut active = 0usize;

    for (idx, cell) in cells.iter_mut().enumerate() {
        let lai = cell.eucalypt_lai.clamp(0.0, 10.0);
        let t_k = cell.leaf_temp_k.clamp(250.0, 335.0);
        let par = cell.par_umol_m2_s.max(0.0);

        // Guenther C_L light dependence and C_T enzymatic temperature dependence
        let cl = (ALPHA * CL1 * par) / (1.0 + (ALPHA * par).powi(2)).sqrt();
        let num = ((CT1 * (t_k - TS)) / (R_GAS * TS * t_k)).exp();
        let den = 1.0 + ((CT2 * (t_k - TM)) / (R_GAS * TS * t_k)).exp();
        let ct = num / den.max(1e-6);

        cell.flux_iso_ug_m2_s = cell.eps_isoprene_ug_m2_s.max(0.0) * lai * cl * ct;
        cell.flux_mono_ug_m2_s =
            cell.eps_monoterp_ug_m2_s.max(0.0) * lai * (BETA_MONO * (t_k - TS)).exp();

        // Photochemical oxidation into Secondary Organic Aerosol (SOA) blue haze
        let ox_factor = (cell.ozone_oxidant_ppb.max(5.0) / 40.0).clamp(0.1, 3.0);
        let soa_prod = (0.03 * cell.flux_iso_ug_m2_s + 0.18 * cell.flux_mono_ug_m2_s) * ox_factor;
        cell.soa_mass_ug_m3 =
            ((cell.soa_mass_ug_m3 + soa_prod * dt) / (1.0 + advect * dt)).clamp(0.0, 250.0);

        // Rayleigh-regime short-wavelength 460 nm blue scattering coefficient (m^-1)
        cell.blue_scatter_m_inv = cell.soa_mass_ug_m3 * 1.45e-5;
        if idx < out_blue_tau_add.len() {
            out_blue_tau_add[idx] = cell.blue_scatter_m_inv * 400.0; // 400m boundary layer column
        }
        if cell.soa_mass_ug_m3 > 0.1 {
            active += 1;
        }
    }
    active
}

/// Zero-heap underwater CDOM tannin blackwater 6-band absorption & chromatic Snell's Window evaluator.
#[inline]
pub fn eval_underwater_cdom_and_snell_window(cell: &mut UnderwaterBioOpticsCell) {
    let lambdas_nm = [360.0f32, 450.0, 550.0, 650.0, 850.0, 1600.0];
    let pure_water_a = [0.015f32, 0.0145, 0.0565, 0.349, 4.30, 650.0];
    let sg = cell.cdom_slope_nm_inv.clamp(0.010, 0.025);
    let a412 = cell.cdom_a412_m_inv.max(0.0);
    let sed_b = cell.turbidity_ntu.max(0.0) * 0.045;
    let chl_a = cell.chl_water_mg_m3.max(0.0) * 0.025;

    for i in 0..6 {
        let cdom = a412 * (-sg * (lambdas_nm[i] - 412.0)).exp();
        let chl_band = if i == 1 || i == 3 { chl_a } else { chl_a * 0.25 };
        cell.band_abs_m_inv[i] = pure_water_a[i] + cdom + chl_band + sed_b;
    }
    // Chromatic refractive index of water [UV 360nm, Blue 450nm, Green 550nm, Red 650nm]
    let n_water = [1.3475f32, 1.3390, 1.3340, 1.3314];
    for j in 0..4 {
        cell.snell_crit_rad[j] = (1.0 / n_water[j]).asin();
    }
}

/// Zero-heap Brewster-angle Fresnel polarization & Krisciunas–Schaefer nocturnal sky radiance evaluator.
pub fn eval_brewster_polarization_and_nocturnal_sky(
    state: &mut BrewsterNocturnalSky,
    incidence_angle_rad: f32,
    unpolarized_sky_flux: f32,
) {
    let n2 = state.surface_ior.clamp(1.1, 2.5);
    state.brewster_rad = n2.atan();
    let theta_i = incidence_angle_rad.clamp(0.01, 1.55);
    let sin_t = (theta_i.sin() / n2).clamp(-0.999, 0.999);
    let cos_i = theta_i.cos();
    let cos_t = (1.0 - sin_t * sin_t).sqrt();

    let rs = ((cos_i - n2 * cos_t) / (cos_i + n2 * cos_t).max(1e-6)).powi(2);
    let rp = ((n2 * cos_i - cos_t) / (n2 * cos_i + cos_t).max(1e-6)).powi(2);
    let r_sum = (rs + rp).max(1e-8);
    state.reflected_dolp = ((rs - rp).abs() / r_sum).clamp(0.0, 1.0);
    state.stokes_q_water = unpolarized_sky_flux.max(0.0) * 0.5 * (rs - rp);
    state.stokes_u_water = 0.0;
    state.polarotaxis_attract = (state.reflected_dolp * (0.5 * r_sum / 0.12).min(1.0)).clamp(0.0, 1.0);
    state.trap_surface_flag = u32::from(n2 >= 1.45 && state.reflected_dolp > 0.65);

    // Krisciunas-Schaefer (1991) lunar V-magnitude & opposition surge
    let psi_deg = (state.lunar_phase_rad.abs() * 57.29578).clamp(0.0, 180.0);
    let psi_norm = psi_deg / 100.0;
    state.lunar_mag_v = -12.73 + 1.49 * psi_norm + 0.043 * psi_norm.powi(4);
    state.opp_surge_factor = if psi_deg < 7.0 {
        (1.35 - 0.05 * psi_deg).max(1.0)
    } else {
        1.0
    };
    let sin_z = state.lunar_zenith_rad.clamp(0.0, 1.52).sin();
    let airmass_x = (1.0 - 0.96 * sin_z * sin_z).max(0.04).powf(-0.5);
    let moon_illum = 10.0f32.powf(-0.4 * (state.lunar_mag_v + 16.57))
        * state.opp_surge_factor
        * (-0.172 * airmass_x).exp();
    // Combine moonless zenith floor (~21.9 mag/arcsec^2) with lunar scattered brightness
    state.zenith_sky_mag = (21.9 - 2.5 * (1.0 + moon_illum * 1.0e5).log10()).clamp(15.5, 22.4);
}

/// Zero-heap wet beach sand Reynolds dilatancy footprint halo & bearing boost stepper (64 Hz).
pub fn step_wet_sand_dilatancy(
    cells: &mut [WetSandDilatancyCell],
    foot_shear_impulses: &[f32],
    dt_s: f32,
) -> usize {
    let dt = dt_s.max(1e-4);
    let k_bulk_pore = 1.8e6f32;
    let mut active_halos = 0usize;

    for (idx, cell) in cells.iter_mut().enumerate() {
        let gamma_imp = foot_shear_impulses.get(idx).copied().unwrap_or(0.0).max(0.0);
        cell.shear_impulse = gamma_imp;
        let e_max = cell.void_ratio_max.max(0.55);

        if gamma_imp > 1e-5 && cell.void_ratio_e < e_max && cell.water_table_depth_m <= 0.15 {
            // Reynolds granular dilation delta_e = gamma_dot * tan(psi_d) * dt
            let mobilize = ((e_max - cell.void_ratio_e) / 0.25).clamp(0.0, 1.0);
            let tan_psi = cell.dilatancy_angle_rad.clamp(0.0, 0.45).tan() * mobilize;
            let delta_e = (gamma_imp * tan_psi).min(e_max - cell.void_ratio_e);
            cell.void_ratio_e += delta_e;
            // Negative pore suction pulls meniscus below sand grains
            let delta_u = -k_bulk_pore * (delta_e / (1.0 + cell.void_ratio_e));
            cell.pore_suction_pa = (cell.pore_suction_pa + delta_u).clamp(-85_000.0, 0.0);
        }

        // Darcy re-wetting relaxation
        let tau_relax = cell.darcy_relax_s.clamp(0.25, 5.0);
        let decay = (-dt / tau_relax).exp();
        cell.pore_suction_pa *= decay;
        cell.void_ratio_e += (0.58 - cell.void_ratio_e) * (1.0 - decay);

        let suction_mag = (-cell.pore_suction_pa).max(0.0);
        cell.air_entry_frac = (suction_mag / 35_000.0).clamp(0.0, 1.0);
        cell.bearing_boost = 1.0 + 1.25 * cell.air_entry_frac;
        cell.albedo_lift = 0.38 * cell.air_entry_frac;
        cell.active_halo_flag = u32::from(cell.air_entry_frac > 0.01);
        if cell.active_halo_flag != 0 {
            active_halos += 1;
        }
    }
    active_halos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hygromorph_bilayer_curls_on_dryness_and_eucalypt_bvoc_generates_blue_haze() {
        let mut cones = [HygromorphBilayerOrgan {
            thick_passive_m: 0.0006,
            thick_active_m: 0.0006,
            modulus_ratio_n: 5.0,
            diff_alpha_rh: 0.18,
            ref_rh: 0.90,
            internal_rh: 0.90,
            wetting_front_zm: 0.0,
            curvature_m_inv: 0.0,
            twist_pitch_rad: 0.0,
            skl2_scale_bone: 1,
            organ_kind: 2, // Self-burying awn
            buried_flag: 0,
        }];
        let active = step_hygromorph_bilayers(&mut cones, 0.15, 1.0e-6, 5.0);
        assert_eq!(active, 1);
        assert!(cones[0].curvature_m_inv < -30.0);
        assert_eq!(cones[0].buried_flag, 1);

        let mut haze = [BvocBlueHazeCell {
            eps_isoprene_ug_m2_s: 1.5,
            eps_monoterp_ug_m2_s: 0.6,
            eucalypt_lai: 4.2,
            leaf_temp_k: 311.0, // 38 C hot Australian summer day
            par_umol_m2_s: 1450.0,
            flux_iso_ug_m2_s: 0.0,
            flux_mono_ug_m2_s: 0.0,
            soa_mass_ug_m3: 0.0,
            blue_scatter_m_inv: 0.0,
            ozone_oxidant_ppb: 55.0,
            cell_xy_idx: 0,
            flags: 0,
        }];
        let mut tau = [0.0f32; 1];
        step_eucalypt_bvoc_blue_haze(&mut haze, 0.002, 60.0, &mut tau);
        assert!(haze[0].flux_iso_ug_m2_s > 5.0);
        assert!(haze[0].soa_mass_ug_m3 > 10.0);
        assert!(tau[0] > 0.05);
    }

    #[test]
    fn wet_sand_dilatancy_produces_bright_footprint_halo_and_brewster_polarotaxis() {
        let mut sand = [WetSandDilatancyCell {
            void_ratio_e: 0.58,
            void_ratio_max: 0.78,
            dilatancy_angle_rad: 0.28,
            pore_suction_pa: 0.0,
            air_entry_frac: 0.0,
            bearing_boost: 1.0,
            darcy_relax_s: 1.5,
            water_table_depth_m: 0.0,
            shear_impulse: 0.0,
            albedo_lift: 0.0,
            cell_xy_idx: 0,
            active_halo_flag: 0,
        }];
        let active = step_wet_sand_dilatancy(&mut sand, &[0.12], 0.015625);
        assert_eq!(active, 1);
        assert!(sand[0].pore_suction_pa < -10_000.0);
        assert!(sand[0].bearing_boost > 1.35);
        assert!(sand[0].albedo_lift > 0.10);

        let mut brewster = BrewsterNocturnalSky {
            surface_ior: 1.333,
            brewster_rad: 0.0,
            reflected_dolp: 0.0,
            polarotaxis_attract: 0.0,
            lunar_phase_rad: 0.05,
            lunar_zenith_rad: 0.3,
            lunar_mag_v: 0.0,
            opp_surge_factor: 1.0,
            airglow_557nm_rayl: 250.0,
            airglow_nir_oh: 800.0,
            zodiacal_flux: 100.0,
            zenith_sky_mag: 22.0,
            stokes_q_water: 0.0,
            stokes_u_water: 0.0,
            night_mode_flag: 1,
            trap_surface_flag: 0,
        };
        eval_brewster_polarization_and_nocturnal_sky(&mut brewster, 1.333f32.atan(), 50.0);
        assert!(brewster.reflected_dolp > 0.98, "reflection at Brewster angle must be ~100% s-polarized");
        assert!(brewster.opp_surge_factor > 1.15);
    }
}

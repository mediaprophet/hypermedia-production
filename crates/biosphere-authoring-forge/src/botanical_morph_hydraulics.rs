//! # Vector 4 — Botanical Morphogenesis, Phyllotaxis, Phenology & Hydraulic Cavitation (`botanical_morph_hydraulics.rs`)
//!
//! Implements the 5 botanical morphogenesis, hydraulic, and spectral laws from Vector 4:
//! 1. **Fibonacci Spiral Phyllotaxis & Fern Circinate Vernation (`PhyllotaxisPrimordium`):**
//!    Vogel (1979) exact golden-angle packing $\theta_k = k \cdot \pi(3 - \sqrt{5}) \approx 137.507764^\circ$
//!    for *Banksia* cones, Asteraceae capitula, pinecones, and *Xanthorrhoea* grass-tree skirts,
//!    plus basipetal logarithmic-spiral fern fiddlehead uncoiling behind maturation front $s_f(t)$.
//! 2. **Xylem Hydraulic Cavitation ($P_{50}$) & VPD Leaf-Wilting Tensor (`XylemHydraulicNode`):**
//!    Pammenter & Vander Willigen (1998) vulnerability curve
//!    $\text{PLC}(\Psi_{\min}) = 100 / (1 + \exp(a_{\text{vc}}(\Psi_{\min} - P_{50})))$ with
//!    embolism memory hysteresis, leaf turgor loss $\Psi_{\text{tlp}}$, petiole droop, and
//!    transverse leaf rolling $\kappa_{\text{curl}}$.
//! 3. **Pulvinus Osmotic Motor Joints (`PulvinusMotorJoint`):**
//!    Differential extensor–flexor $\text{K}^+/\text{Cl}^-$ turgor $\Delta P$ driving rapid
//!    *Mimosa pudica* thigmonastic collapse ($\tau_{\text{osm}} \approx 0.25\text{ s}$), diurnal
//!    nyctinastic sleep folding, and solar heliotropic leaf tracking.
//! 4. **PROSPECT-MP 6-Band Leaf Senescence & Red-Edge Wavelength Shift (`LeafSpectralPigments`):**
//!    Couples chlorophyll $a+b$, carotenoids, stress anthocyanins, equivalent water thickness $C_w$,
//!    dry matter $C_m$, and mesophyll structure $N$ to `Q42EMF64` 6-band reflectance/transmittance
//!    and the 680–750 nm Red-Edge Inflection Point (`REIP`).
//! 5. **Bark Rhytidome Circumferential Growth Fracture & Peeling Ribbons (`BarkRhytidomePatch`):**
//!    Secondary cambial expansion hoop strain $\varepsilon_\theta = \Delta R / R$, Griffith
//!    periderm cracking, and hygroscopic outward curling of smooth-gum peeling bark ribbons.

use serde::{Deserialize, Serialize};

/// Exact golden angle $\Phi_g = \pi(3 - \sqrt{5})$ in radians ($137.507764^\circ$).
pub const GOLDEN_ANGLE_RAD: f32 = 2.399_963_2;

/// 48-byte POD spiral phyllotaxis primordium & fern circinate vernation segment.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhyllotaxisPrimordium {
    /// Primordium / rachis node local coordinate (m).
    pub local_pos_m: [f32; 3],
    /// Golden-angle spiral azimuth $k \cdot 2.39996323$ (rad).
    pub azimuth_rad: f32,
    /// Pitch / elevation divergence from meristem axis (rad).
    pub divergence_rad: f32,
    /// Primordium radial scale (m).
    pub scale_factor: f32,
    /// Normalized rachis arc length $s \in [0.0, 1.0]$.
    pub arc_s_norm: f32,
    /// Active circinate vernation curvature $\kappa(s, t)$ ($\text{m}^{-1}$).
    pub coil_curvature: f32,
    /// Basipetal uncoiling wave position $s_f \in [0.0, 1.0]$.
    pub uncoil_front_s: f32,
    /// Spiral index $k$.
    pub organ_index: u32,
    /// Parent stem/rachis index.
    pub parent_node: u32,
    /// `0 = Floret`, `1 = ConeScale`, `2 = XanthorrhoeaSkirt`, `3 = FernPinna`.
    pub organ_kind: u32,
}

/// 64-byte POD xylem cavitation (Pammenter–Vander Willigen) & VPD wilting state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XylemHydraulicNode {
    /// Current xylem water potential ($\le 0.0$ MPa).
    pub psi_xylem_mpa: f32,
    /// Historical minimum water potential (irreversible embolism hysteresis memory, MPa).
    pub psi_min_hist_mpa: f32,
    /// Water potential at 50% conductivity loss (`-0.7..-5.9` MPa).
    pub p50_mpa: f32,
    /// Vulnerability curve slope parameter $a_{\text{vc}}$ ($\text{MPa}^{-1}$).
    pub vc_slope_a: f32,
    /// Percent loss of conductivity normalized to `[0.0, 1.0]`.
    pub plc_frac: f32,
    /// Saturated stem hydraulic conductance ($\text{kg}/(\text{s}\cdot\text{MPa})$).
    pub k_max_kg_s_mpa: f32,
    /// Current leaf mesophyll turgor pressure (Pa).
    pub turgor_pa: f32,
    /// Leaf turgor loss point $\Psi_{\text{tlp}}$ (MPa).
    pub turgor_loss_mpa: f32,
    /// Petiole droop angle under turgor collapse (rad).
    pub wilt_droop_rad: f32,
    /// Transverse leaf rolling curvature ($\text{m}^{-1}$).
    pub curl_curvature_m_inv: f32,
    /// Local canopy Vapor Pressure Deficit (kPa).
    pub vpd_kpa: f32,
    /// Instantaneous branch transpiration flux (kg/s).
    pub transpiration_kg_s: f32,
    /// Overnight embolism refilling rate constant ($\text{s}^{-1}$).
    pub refill_rate_s_inv: f32,
    /// Elevation above root collar (m).
    pub height_z_m: f32,
    /// Upstream hydraulic node index.
    pub parent_branch_id: u32,
    /// `Bit0 = Wilted`, `Bit1 = HydraulicFailure(>P88)`.
    pub state_flags: u32,
}

/// 48-byte POD osmotic pulvinus motor joint (nyctinasty, heliotropism, *Mimosa* thigmonasty).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PulvinusMotorJoint {
    /// Extensor minus flexor turgor pressure $\Delta P$ (Pa).
    pub delta_turgor_pa: f32,
    /// Stimulus-commanded equilibrium $\Delta P_{\text{target}}$ (Pa).
    pub target_turgor_pa: f32,
    /// Current leaflet/petiole deflection angle $\theta$ (rad).
    pub theta_rad: f32,
    /// Angular velocity $\dot{\theta}$ (rad/s).
    pub theta_dot_rad_s: f32,
    /// Open daytime rest angle $\theta_0$ (rad).
    pub theta_rest_rad: f32,
    /// Cylindrical pulvinus diameter $D_p$ (m).
    pub pulvinus_diam_m: f32,
    /// Osmotic time constant (`0.25s` *Mimosa* .. `1800s` nyctinasty).
    pub tau_osmotic_s: f32,
    /// $\text{K}^+/\text{Cl}^-$ channel activation in `[0.0, 1.0]`.
    pub ion_efflux_state: f32,
    /// Recovery timer before re-triggering (s).
    pub refractory_timer_s: f32,
    /// Bound `SKL2` PGA hinge bone index.
    pub skl2_bone_idx: u32,
    /// `0 = Heliotropic`, `1 = Nyctinastic`, `2 = Thigmonastic`.
    pub mode_kind: u32,
    /// Touch / action-potential stimulus bitmask.
    pub trigger_flags: u32,
}

/// 48-byte POD PROSPECT-MP leaf biochemical pigment & 6-band `Q42EMF64` spectral state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeafSpectralPigments {
    /// Chlorophyll $a+b$ surface density (`0..90` $\mu\text{g}/\text{cm}^2$).
    pub chl_ug_cm2: f32,
    /// Carotenoids surface density (`0..25` $\mu\text{g}/\text{cm}^2$).
    pub car_ug_cm2: f32,
    /// Stress/senescence anthocyanins (`0..30` $\mu\text{g}/\text{cm}^2$).
    pub ant_ug_cm2: f32,
    /// Equivalent water thickness $C_w$ (`0.004..0.04` cm).
    pub water_ewt_cm: f32,
    /// Leaf dry matter mass per area $C_m$ (`0.002..0.02` $\text{g}/\text{cm}^2$).
    pub dry_matter_g_cm2: f32,
    /// Mesophyll plate structure parameter $N_{\text{meso}}$ (`1.0..3.5`).
    pub structure_n: f32,
    /// Computed Red-Edge Inflection Point (`680..750` nm).
    pub red_edge_nm: f32,
    /// Cached 4-band reflectance `[Blue, Green, Red, NIR]`.
    pub refl_bands: [f32; 4],
    /// Chlorophyll decay rate $k_{\text{sen}}$ ($\text{s}^{-1}$).
    pub senescence_rate: f32,
}

/// 48-byte POD bark rhytidome circumferential fracture & peeling ribbon patch.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BarkRhytidomePatch {
    /// Current cambial trunk radius $R(t)$ (m).
    pub trunk_radius_m: f32,
    /// Accumulated circumferential hoop strain $\varepsilon_\theta = \Delta R / R$.
    pub hoop_strain: f32,
    /// Dead outer rhytidome layer thickness $h_p$ (m).
    pub periderm_thick_m: f32,
    /// Critical fracture energy release rate $G_{Ic}$ ($\text{J}/\text{m}^2$).
    pub fracture_tough_j_m2: f32,
    /// Ribbon detachment fraction in `[0.0, 1.0]`.
    pub peel_progress: f32,
    /// Outward hygroscopic curl curvature ($\text{m}^{-1}$).
    pub peel_curl_m_inv: f32,
    /// Equilibrium longitudinal furrow spacing (m).
    pub crack_spacing_m: f32,
    /// Fresh exposed underbark periderm color phase (cream/orange/salmon gum).
    pub underbark_hue_idx: f32,
    /// Pending shed bark litter fuel mass dropped to `.m64` (kg).
    pub shed_mass_kg: f32,
    /// `.10d` trunk UV patch identifier.
    pub uv_patch_id: u32,
    /// `0 = RibbonSmooth`, `1 = Tessellated`, `2 = StringyFibrous`, `3 = FurrowedIronbark`.
    pub bark_pattern: u32,
    /// `1` once wind detaches ribbon into ground litter bed.
    pub detached_flag: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<PhyllotaxisPrimordium>()];
const _: [(); 64] = [(); core::mem::size_of::<XylemHydraulicNode>()];
const _: [(); 48] = [(); core::mem::size_of::<PulvinusMotorJoint>()];
const _: [(); 48] = [(); core::mem::size_of::<LeafSpectralPigments>()];
const _: [(); 48] = [(); core::mem::size_of::<BarkRhytidomePatch>()];

/// Populates Vogel golden-angle spiral phyllotaxis primordia and evaluates fern circinate vernation curvature.
pub fn evaluate_vogel_phyllotaxis_and_vernation_zero_heap(
    primordia: &mut [PhyllotaxisPrimordium],
    packing_constant_m: f32,
    cylinder_height_m: f32,
    base_coil_curvature_m_inv: f32,
    uncoil_front_s: f32,
) {
    let n_total = primordia.len().max(1) as f32;
    let c_pack = packing_constant_m.max(0.001);
    let sf = uncoil_front_s.clamp(0.0, 1.0);

    for (k, prim) in primordia.iter_mut().enumerate() {
        let kf = k as f32;
        let u = kf / n_total;
        let theta = (kf * GOLDEN_ANGLE_RAD).rem_euclid(core::f32::consts::TAU);
        let r = if prim.organ_kind == 1 || prim.organ_kind == 2 {
            // Cylindrical / conical Banksia or Xanthorrhoea skirt
            c_pack * (0.35 + 0.65 * (core::f32::consts::PI * u).sin())
        } else {
            // Planar Vogel capitulum r_k = c * sqrt(k)
            c_pack * (kf + 0.5).sqrt()
        };
        let z = cylinder_height_m * u;
        prim.local_pos_m = [r * theta.cos(), z, r * theta.sin()];
        prim.azimuth_rad = theta;
        prim.arc_s_norm = u;
        prim.uncoil_front_s = sf;
        prim.organ_index = k as u32;

        // Fern circinate vernation logarithmic spiral uncoiling behind maturation front s_f
        let logistic_coil = 1.0 / (1.0 + (-(u - sf) / 0.06).exp());
        prim.coil_curvature =
            (base_coil_curvature_m_inv / (0.04 + 0.25 * u)) * logistic_coil;
    }
}

/// Zero-heap Pammenter–Vander Willigen xylem cavitation ($P_{50}$) & VPD leaf-wilting tensor update (20 Hz).
pub fn step_xylem_cavitation_and_wilt(
    nodes: &mut [XylemHydraulicNode],
    soil_psi_mpa: &[f32],
    dt_s: f32,
) -> usize {
    let dt = dt_s.max(0.0);
    let mut cavitated_count = 0usize;

    for (idx, node) in nodes.iter_mut().enumerate() {
        let psi_soil = soil_psi_mpa.get(idx).copied().unwrap_or(-0.3).min(0.0);
        // Gravity head (-0.0098 MPa/m) + transpiration tension across current hydraulic conductance
        let k_curr = (node.k_max_kg_s_mpa.max(1e-5) * (1.0 - node.plc_frac.clamp(0.0, 0.99))).max(1e-6);
        let vpd_boost = 1.0 + 0.35 * node.vpd_kpa.max(0.0);
        let psi_x = psi_soil - 0.0098 * node.height_z_m.max(0.0)
            - (node.transpiration_kg_s.max(0.0) * vpd_boost) / k_curr;
        node.psi_xylem_mpa = psi_x.clamp(-15.0, 0.0);

        // Embolism memory hysteresis: tracks historical minimum unless wet soil allows overnight refilling
        if node.psi_xylem_mpa < node.psi_min_hist_mpa {
            node.psi_min_hist_mpa = node.psi_xylem_mpa;
        } else if psi_soil > -0.25 && node.vpd_kpa < 0.5 {
            node.psi_min_hist_mpa +=
                (node.psi_xylem_mpa - node.psi_min_hist_mpa) * (node.refill_rate_s_inv * dt).clamp(0.0, 0.2);
        }

        // Pammenter & Vander Willigen (1998) logistic vulnerability curve
        let a_vc = node.vc_slope_a.abs().max(0.5);
        let exp_arg = (a_vc * (node.psi_min_hist_mpa - node.p50_mpa)).clamp(-20.0, 20.0);
        node.plc_frac = (1.0 / (1.0 + exp_arg.exp())).clamp(0.0, 1.0);

        // Leaf turgor pressure above turgor-loss point Psi_tlp
        let psi_tlp = node.turgor_loss_mpa.min(-0.5);
        let turgor_mpa = (node.psi_xylem_mpa - psi_tlp).max(0.0);
        node.turgor_pa = turgor_mpa * 1.0e6;
        let p_t0 = (-psi_tlp * 1.0e6).max(1.0e5);
        let turgor_ratio = (node.turgor_pa / p_t0).clamp(0.0, 1.0);

        // Petiole wilting droop and transverse leaf rolling curvature
        node.wilt_droop_rad = 1.15 * (1.0 - turgor_ratio).powi(2);
        node.curl_curvature_m_inv = 45.0 * (1.0 - turgor_ratio).powi(2);
        let wilted = u32::from(turgor_ratio < 0.15);
        let failed = u32::from(node.plc_frac >= 0.88) << 1;
        node.state_flags = wilted | failed;
        if node.plc_frac > 0.50 {
            cavitated_count += 1;
        }
    }
    cavitated_count
}

/// Zero-heap 6-band PROSPECT leaf reflectance/transmittance & Red-Edge Inflection Point (`REIP`) synthesizer for `Q42EMF64`.
pub fn eval_prospect_leaf_emf_bands(
    leaf: &mut LeafSpectralPigments,
    stress_factor: f32,
    dt_s: f32,
    out_six_band_refl: &mut [f32; 6],
    out_six_band_trans: &mut [f32; 6],
) {
    let dt = dt_s.max(0.0);
    let stress = stress_factor.clamp(0.0, 1.0);
    // Senescence chlorophyll breakdown and stress anthocyanin synthesis
    leaf.chl_ug_cm2 = (leaf.chl_ug_cm2 * (-leaf.senescence_rate.max(0.0) * (1.0 + 2.0 * stress) * dt).exp())
        .clamp(0.0, 100.0);
    leaf.ant_ug_cm2 = (leaf.ant_ug_cm2 + 0.05 * stress * dt).clamp(0.0, 35.0);

    let n_meso = leaf.structure_n.clamp(1.0, 4.0);
    let chl = leaf.chl_ug_cm2;
    let car = leaf.car_ug_cm2.max(0.0);
    let ant = leaf.ant_ug_cm2;
    let cw = leaf.water_ewt_cm.max(0.001);
    let cm = leaf.dry_matter_g_cm2.max(0.001);

    // 6-band specific absorption coefficients: [UV 360, Blue 450, Green 550, Red 660, NIR 850, SWIR 1600]
    let k_chl = [0.045f32, 0.042, 0.006, 0.038, 0.0001, 0.0];
    let k_car = [0.060f32, 0.075, 0.008, 0.0005, 0.0, 0.0];
    let k_ant = [0.030f32, 0.025, 0.065, 0.004, 0.0, 0.0];
    let k_w = [1.0f32, 0.5, 0.8, 2.0, 6.5, 145.0];
    let k_m = [85.0f32, 40.0, 25.0, 20.0, 18.0, 65.0];

    for i in 0..6 {
        let k_plate =
            (k_chl[i] * chl + k_car[i] * car + k_ant[i] * ant + k_w[i] * cw + k_m[i] * cm) / n_meso;
        let tau_single = (-k_plate).exp().clamp(0.01, 0.98);
        // Plate multiple scattering across N_meso layers
        let refl = (0.52 * (1.0 - (-0.45 * n_meso).exp()) * tau_single.powf(0.65)).clamp(0.02, 0.65);
        let trans = ((1.0 - refl) * tau_single.powf(n_meso * 0.8)).clamp(0.005, 0.60);
        out_six_band_refl[i] = refl;
        out_six_band_trans[i] = trans;
    }
    leaf.refl_bands = [
        out_six_band_refl[1],
        out_six_band_refl[2],
        out_six_band_refl[3],
        out_six_band_refl[4],
    ];
    // Red-edge inflection point shifts linearly/logarithmically from 735 nm (high Chl) down to 685 nm (chlorotic/senescent)
    leaf.red_edge_nm = (685.0 + 50.0 * (chl / (chl + 22.0))).clamp(680.0, 750.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xylem_cavitation_triggers_wilting_and_prospect_shifts_red_edge_on_senescence() {
        let mut nodes = [XylemHydraulicNode {
            psi_xylem_mpa: -0.5,
            psi_min_hist_mpa: -0.5,
            p50_mpa: -2.8,
            vc_slope_a: 2.2,
            plc_frac: 0.02,
            k_max_kg_s_mpa: 0.05,
            turgor_pa: 1.5e6,
            turgor_loss_mpa: -2.2,
            wilt_droop_rad: 0.0,
            curl_curvature_m_inv: 0.0,
            vpd_kpa: 4.5,
            transpiration_kg_s: 0.06,
            refill_rate_s_inv: 0.001,
            height_z_m: 12.0,
            parent_branch_id: 0,
            state_flags: 0,
        }];
        let cavitated = step_xylem_cavitation_and_wilt(&mut nodes, &[-2.4], 10.0);
        assert_eq!(cavitated, 1);
        assert!(nodes[0].plc_frac > 0.55);
        assert!(nodes[0].wilt_droop_rad > 0.8);
        assert_ne!(nodes[0].state_flags & 1, 0);

        let mut leaf = LeafSpectralPigments {
            chl_ug_cm2: 65.0,
            car_ug_cm2: 12.0,
            ant_ug_cm2: 1.0,
            water_ewt_cm: 0.015,
            dry_matter_g_cm2: 0.008,
            structure_n: 1.8,
            red_edge_nm: 730.0,
            refl_bands: [0.0; 4],
            senescence_rate: 0.15,
        };
        let mut r_healthy = [0.0f32; 6];
        let mut t_healthy = [0.0f32; 6];
        eval_prospect_leaf_emf_bands(&mut leaf, 0.0, 0.0, &mut r_healthy, &mut t_healthy);
        let re_healthy = leaf.red_edge_nm;

        let mut r_senescent = [0.0f32; 6];
        let mut t_senescent = [0.0f32; 6];
        eval_prospect_leaf_emf_bands(&mut leaf, 1.0, 12.0, &mut r_senescent, &mut t_senescent);
        assert!(leaf.red_edge_nm < re_healthy - 15.0, "senescence must shift red-edge blueward");
        assert!(r_senescent[3] > r_healthy[3], "chlorophyll loss must increase red reflectance");
    }
}

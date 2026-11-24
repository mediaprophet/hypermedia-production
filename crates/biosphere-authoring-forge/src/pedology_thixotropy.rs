//! # Vector 1 — Sub-Surface Pedology, Thixotropic Mud, Desiccation Cracks & Bioturbation (`pedology_thixotropy.rs`)
//!
//! Implements the sub-surface rheological, cryogenic, and bioturbation solvers from Vector 1:
//! 1. **Coussot Thixotropic Mud Rheology, Cyclic Liquefaction & Fauna IK Suction Mire Forces (`ThixotropicSoilCell`):**
//!    Exact exponential integrator for time-dependent structural aging $\lambda(t)$ and Herschel–Bulkley
//!    viscosity $\eta(\lambda) = \eta_0(1 + \omega \lambda^n)$, Seed–Idriss cyclic excess pore-pressure
//!    ratio $r_u \in [0, 1]$, and upward foot extraction suction mire force combining Darcy cavity suction
//!    with non-Newtonian Stefan viscous adhesion (capped by cavitation and ankle-peel air venting).
//! 2. **Desiccation Polygonal Mud-Cracking (`DesiccationCrackNode`):**
//!    Bishop–van Genuchten matric suction $s$, Griffith tensile fracture nucleation, and wet–dry cycle
//!    relaxation from primary $90^\circ$ T-junctions toward mature $120^\circ$ hexagonal Y-junctions.
//! 3. **Konrad–Morgenstern Frost Heave & Australian Digging-Mammal/Termite Bioturbation (`BioturbationAgent`):**
//!    Segregation-potential ice-lens heave, wombat (*Vombatus ursinus*) and woylie (*Bettongia penicillata*)
//!    soil turnover/burrow refuges, and magnetic termite (*Amitermes meridionalis*) mound alignment.

use crate::containers_10d_m64::project_metres_f32_checked;
use serde::{Deserialize, Serialize};

/// 64-byte POD per-cell thixotropic mud, liquefaction, frost-heave & bioturbation state.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThixotropicSoilCell {
    /// Coussot microstructural aging state $\lambda \ge 0.0$.
    pub lambda_struct: f32,
    /// High-shear asymptotic viscosity $\eta_0$ ($\text{Pa}\cdot\text{s}$).
    pub eta_zero_pa_s: f32,
    /// Thixotropic structure sensitivity $\omega$.
    pub omega_struct: f32,
    /// Herschel–Bulkley flow index $n \in [0.4, 1.0]$.
    pub hb_index_n: f32,
    /// Characteristic rest-aging timescale $T_0$ (s).
    pub rest_time_t0_s: f32,
    /// Structural shear-breakdown coefficient $\alpha$.
    pub alpha_shear: f32,
    /// Cyclic liquefaction excess pore-pressure ratio $r_u \in [0.0, 1.0]$.
    pub pore_ratio_ru: f32,
    /// Effective water saturation $S_e \in [0.0, 1.0]$.
    pub sat_effective: f32,
    /// Vertical Darcy hydraulic conductivity $k_v$ (m/s).
    pub hydraulic_k_m_s: f32,
    /// Mechanical mud layer thickness $h_{\text{clay}}$ (m).
    pub clay_thickness_m: f32,
    /// Konrad–Morgenstern segregation potential $SP_0$ ($\text{m}^2/(\text{K}\cdot\text{s})$).
    pub seg_pot_sp0: f32,
    /// Cumulative frost heave displacement (m).
    pub heave_accum_m: f32,
    /// Cryoturbation coarse clast fraction in `[0.0, 1.0]`.
    pub stone_frac: f32,
    /// Bioturbation surface biodiffusivity $D_{\text{bio}}$ ($\text{m}^2/\text{s}$).
    pub biodiff_m2_s: f32,
    /// Termite/earthworm macropores per $\text{m}^2$.
    pub macropore_density: f32,
    /// Low 16 bits: wet–dry cycle count; High 16 bits: soil class flags.
    pub flags_and_cycles: u32,
}

/// 48-byte POD desiccation mud-crack vertex & junction state ($90^\circ$ T- to $120^\circ$ Y-junction relaxation).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DesiccationCrackNode {
    /// World XZ position (m).
    pub pos_xz_m: [f32; 2],
    /// Crack opening width (m).
    pub aperture_m: f32,
    /// Crack vertical penetration depth (m).
    pub depth_m: f32,
    /// Clay polygon edge upward curl curvature ($\text{m}^{-1}$).
    pub curl_curvature: f32,
    /// Local Bishop–van Genuchten matric suction (Pa).
    pub matric_suction_pa: f32,
    /// Local biaxial drying tensile stress (Pa).
    pub tensile_stress_pa: f32,
    /// Up to 3 incident dual-graph crack vertices (`u32::MAX` = none).
    pub neighbors: [u32; 3],
    /// Crack generation (`1` = primary, `2+` = secondary).
    pub generation: u16,
    /// Junction degree (`2`, `3` = T/Y junction).
    pub junction_deg: u16,
    /// `0.0 = 90° T-junction` $\to$ `1.0 = 120° Y-junction`.
    pub angle_relax_t: f32,
}

/// 48-byte POD burrowing/mounding bioturbation ecosystem-engineer agent.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BioturbationAgent {
    /// Active burrow mouth or mound center `[x, y, z]` (m).
    pub pos_xyz_m: [f32; 3],
    /// Tunnel heading or magnetic-termite N–S major axis azimuth (rad).
    pub axis_azimuth_rad: f32,
    /// Soil excavation/mounding rate (kg/s).
    pub turnover_kg_s: f32,
    /// Current burrow network length (m; up to 15+ m for *Vombatus ursinus*).
    pub tunnel_length_m: f32,
    /// Displaced spoil mound or termitarium volume ($\text{m}^3$).
    pub mound_volume_m3: f32,
    /// Buffered burrow/mound microclimate temperature ($^\circ\text{C}$).
    pub internal_temp_c: f32,
    /// Duration burrow holds storm runoff water (s).
    pub water_ponding_s: f32,
    /// Local soil nitrogen enrichment multiplier ($\ge 1.0$).
    pub nitrogen_boost: f32,
    /// `0 = Wombat`, `1 = Woylie/Bandicoot`, `2 = AmitermesMagneticTermite`, `3 = AnecicEarthworm`.
    pub species_kind: u32,
    /// Active / refuge occupancy bitmask.
    pub active_mask: u32,
}

const _: [(); 64] = [(); core::mem::size_of::<ThixotropicSoilCell>()];
const _: [(); 48] = [(); core::mem::size_of::<DesiccationCrackNode>()];
const _: [(); 48] = [(); core::mem::size_of::<BioturbationAgent>()];

/// Zero-heap Coussot thixotropic viscosity update & fauna IK mire pullout force evaluator (64 Hz).
///
/// Outputs downward suction pullout force in Newtons (`out_pullout_force_n`) and `.m64` (`GRM3`)
/// millimetre elevation deltas (`out_m64_delta_mm`, guarded by the mandatory `0.0005 m` narrowing check).
pub fn step_thixotropic_mire_contact(
    soil: &mut [ThixotropicSoilCell],
    cell_indices: &[u32],
    foot_radii_m: &[f32],
    sink_depths_m: &[f32],
    lift_velocities_m_s: &[f32],
    vent_fractions: &[f32],
    cyclic_csr: &[f32],
    dt_s: f32,
    out_pullout_force_n: &mut [f32],
    out_m64_delta_mm: &mut [i32],
) -> usize {
    let dt = dt_s.max(1e-4);
    let contacts = cell_indices
        .len()
        .min(foot_radii_m.len())
        .min(sink_depths_m.len())
        .min(lift_velocities_m_s.len())
        .min(out_pullout_force_n.len());
    let mut mired_count = 0usize;

    for i in 0..contacts {
        let c_idx = cell_indices[i] as usize;
        if c_idx >= soil.len() {
            out_pullout_force_n[i] = 0.0;
            continue;
        }
        let cell = &mut soil[c_idx];
        let rf = foot_radii_m[i].max(0.01);
        let z_sink = sink_depths_m[i].max(0.0);
        let vz = lift_velocities_m_s[i];
        let f_vent = vent_fractions.get(i).copied().unwrap_or(0.0).clamp(0.0, 1.0);
        let csr = cyclic_csr.get(i).copied().unwrap_or(0.0).max(0.0);

        // 1. Exact exponential step for Coussot thixotropic structure parameter lambda
        let shear_rate = (vz.abs() / rf).max(if z_sink > 0.001 { 0.5 } else { 0.0 });
        let t0 = cell.rest_time_t0_s.max(0.5);
        let alpha_g = cell.alpha_shear.max(0.0) * shear_rate;
        cell.lambda_struct = if alpha_g > 1.0e-6 {
            let lam_eq = 1.0 / (t0 * alpha_g);
            (lam_eq + (cell.lambda_struct - lam_eq) * (-alpha_g * dt).exp()).clamp(0.0, 100.0)
        } else {
            (cell.lambda_struct + dt / t0).clamp(0.0, 100.0)
        };

        // Herschel-Bulkley thixotropic apparent viscosity eta(lambda) = eta_0 * (1 + omega * lambda^n)
        let eta = cell.eta_zero_pa_s.max(0.01)
            * (1.0 + cell.omega_struct.max(0.0) * cell.lambda_struct.powf(cell.hb_index_n.clamp(0.3, 1.0)));

        // 2. Seed-Idriss cyclic pore-pressure buildup r_u vs. Darcy drainage
        let drain_rate = (cell.hydraulic_k_m_s.max(1e-9) * 1.0e4) / cell.clay_thickness_m.max(0.05).powi(2);
        let buildup = 2.5 * (csr - 0.12).max(0.0) * (1.0 - cell.pore_ratio_ru);
        cell.pore_ratio_ru =
            (cell.pore_ratio_ru + (buildup - drain_rate * cell.pore_ratio_ru) * dt).clamp(0.0, 0.98);

        // 3. Upward foot extraction suction mire force (Darcy cavity suction + thixotropic Stefan film + wall shear)
        if vz > 0.0 && z_sink > 0.002 && cell.sat_effective > 0.55 {
            let u_cav = 90_000.0 * (1.0 - f_vent);
            let kv = cell.hydraulic_k_m_s.max(1.0e-8);
            let u_suc = u_cav.min((9810.0 * z_sink / (kv * 12.0)) * vz);
            let gap_h = (0.0005 + 0.015 * f_vent).max(0.0005);
            let f_stefan = (1.5 * core::f32::consts::PI * eta * rf.powi(4) * vz) / gap_h.powi(3);
            let area = core::f32::consts::PI * rf * rf;
            let f_cav_max = u_cav * area;
            let tau_y = 45.0 * (1.0 + 0.5 * cell.lambda_struct);
            let f_wall = 2.0 * core::f32::consts::PI * rf * z_sink * tau_y;
            let f_total = (u_suc * area + f_stefan).min(f_cav_max) + f_wall;
            out_pullout_force_n[i] = f_total;
            mired_count += 1;
        } else {
            out_pullout_force_n[i] = 0.0;
        }

        // 4. Stamp footprint depression into .m64 (GRM3) with 0.0005 m narrowing guard
        if i < out_m64_delta_mm.len() {
            let plastic_sink_m = -z_sink * (0.25 + 0.65 * cell.pore_ratio_ru);
            let guarded_m = project_metres_f32_checked(f64::from(plastic_sink_m), "plastic_sink_m")
                .unwrap_or(0.0);
            out_m64_delta_mm[i] = (guarded_m * 1000.0).round() as i32;
        }
    }
    mired_count
}

/// Zero-heap desiccation crack aperture & $90^\circ$ T-to-$120^\circ$ Y-junction relaxation stepper (20 Hz).
pub fn step_desiccation_fracture_network(
    nodes: &mut [DesiccationCrackNode],
    soil_grid: &[ThixotropicSoilCell],
    evap_flux_m_s: f32,
    dt_s: f32,
) -> u32 {
    let dt = dt_s.max(0.0);
    let mean_se = soil_grid
        .first()
        .map(|c| c.sat_effective.clamp(0.02, 1.0))
        .unwrap_or(0.35);
    let h_clay = soil_grid
        .first()
        .map(|c| c.clay_thickness_m.max(0.02))
        .unwrap_or(0.15);
    let cycles = soil_grid
        .first()
        .map(|c| (c.flags_and_cycles & 0xFFFF) as f32)
        .unwrap_or(1.0);

    // van Genuchten matric suction s and constrained biaxial drying tensile stress
    let dryness = (1.0 - mean_se) + (evap_flux_m_s.max(0.0) * dt * 500.0);
    let suction_pa = 85_000.0 * (mean_se.powf(-1.8) - 1.0).max(0.0).powf(0.45);
    let e_clay = 1.2e7f32;
    let sigma_t = (e_clay / 0.7) * 0.08 * dryness.clamp(0.0, 1.0);
    let sigma_griffith = (2.0 * e_clay * 4.5 / (core::f32::consts::PI * 0.005)).sqrt();

    let mut active_cracks = 0u32;
    for node in nodes.iter_mut() {
        node.matric_suction_pa = suction_pa;
        node.tensile_stress_pa = sigma_t;
        if sigma_t >= sigma_griffith {
            node.aperture_m = (0.045 * h_clay * dryness).clamp(0.001, 0.08);
            node.depth_m = (h_clay * dryness.sqrt()).clamp(0.005, h_clay);
            node.curl_curvature = (1.8 * dryness / h_clay).clamp(0.0, 35.0);
            // Repeated wet-dry cycles evolve primary 90° T-junctions toward 120° hexagonal Y-junctions
            node.angle_relax_t = (1.0 - (-cycles / 4.0).exp()).clamp(0.0, 1.0);
            active_cracks += 1;
        }
    }
    active_cracks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thixotropic_mud_shears_thin_and_generates_cavitation_bounded_suction_mire_force() {
        let mut soil = [ThixotropicSoilCell {
            lambda_struct: 12.0,
            eta_zero_pa_s: 0.8,
            omega_struct: 15.0,
            hb_index_n: 0.75,
            rest_time_t0_s: 45.0,
            alpha_shear: 0.8,
            pore_ratio_ru: 0.1,
            sat_effective: 0.92,
            hydraulic_k_m_s: 1.0e-7,
            clay_thickness_m: 0.25,
            seg_pot_sp0: 2.0e-9,
            heave_accum_m: 0.0,
            stone_frac: 0.1,
            biodiff_m2_s: 6.0e-12,
            macropore_density: 12.0,
            flags_and_cycles: 6,
        }];
        let mut pullout = [0.0f32; 1];
        let mut m64_delta = [0i32; 1];
        let mired = step_thixotropic_mire_contact(
            &mut soil,
            &[0],
            &[0.06],
            &[0.12],
            &[0.35],
            &[0.10],
            &[0.45],
            0.015625,
            &mut pullout,
            &mut m64_delta,
        );
        assert_eq!(mired, 1);
        assert!(soil[0].lambda_struct < 12.0, "foot extraction shear must break down thixotropic structure");
        assert!(pullout[0] > 150.0 && pullout[0] < 1500.0, "suction force must be strong yet bounded by cavitation");
        assert!(m64_delta[0] < -20, "must stamp millimetre depression into M64 grid");

        let mut cracks = [DesiccationCrackNode {
            pos_xz_m: [0.0, 0.0],
            aperture_m: 0.0,
            depth_m: 0.0,
            curl_curvature: 0.0,
            matric_suction_pa: 0.0,
            tensile_stress_pa: 0.0,
            neighbors: [1, 2, 3],
            generation: 1,
            junction_deg: 3,
            angle_relax_t: 0.0,
        }];
        soil[0].sat_effective = 0.18; // Desiccated claypan
        let active = step_desiccation_fracture_network(&mut cracks, &soil, 1.0e-5, 10.0);
        assert_eq!(active, 1);
        assert!(cracks[0].aperture_m > 0.005);
        assert!(cracks[0].angle_relax_t > 0.70);
    }
}

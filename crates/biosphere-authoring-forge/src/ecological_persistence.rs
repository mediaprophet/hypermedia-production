//! # Vector 9 — Elasto-Plastic Trampling Trails, Epizoochory & Kino/Gall Persistence (`ecological_persistence.rs`)
//!
//! Implements persistent cross-kingdom contact memory from Vector 9:
//! 1. **Cole / Sun–Liddle Elasto-Plastic Vegetation Trampling & Game-Trail Wear (`TramplingPersistenceCell`):**
//!    Separates fast turgor-reversible elastic stem lay-down $\theta_{\text{el}}$ from slow plastic
//!    cover loss $D_p \in [0, 1]$ parameterized by species resistance $N_{50}$ (`210` fern .. `860` matted
//!    graminoid passes) and hemicryptophyte resilience $k_{\text{resil}}$, depressing compacted game
//!    trails into `.m64` (`GRM3`) under the `0.0005 m` half-millimetre narrowing guard.
//! 2. **Couvreur Epizoochory Burr Dispersal & Pelage Mud/Snow Accretion (`PelageDebrisAttachment`):**
//!    Gaussian plant-to-body-zone height overlap $H_{\text{ov}}$, hair undulation capture, and weather-gated
//!    detachment hazard $\lambda_{\text{det}}$ (slower in rain, faster in tall abrasive scrub).
//! 3. **Australian Bloodwood Kino Resin Exudation & Insect Gall/Leaf-Mine Morphogenesis (`PlantWoundKinoGall`):**
//!    Bingham-film polyphenol kino resin bleeding down bark wounds with oxidative viscosity curing
//!    $\eta_k(t) = \eta_{k0} e^{k_{\text{cure}} t}$, plus larval IAA/cytokinin hypertrophic SDF gall
//!    bulging and senescence-defying chlorophyll "green islands".

use crate::containers_10d_m64::project_metres_f32_checked;
use serde::{Deserialize, Serialize};

/// 48-byte POD elasto-plastic vegetation trampling, recovery hysteresis & game-trail cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TramplingPersistenceCell {
    /// Dominant flattened stem lay-down direction XZ.
    pub bend_dir_xz: [f32; 2],
    /// Fast turgor-reversible elastic stem bend angle (rad).
    pub elastic_bend_rad: f32,
    /// Cumulative plastic structural cover loss $D_p \in [0.0, 1.0]$.
    pub plastic_damage: f32,
    /// Passes required for 50% cover loss $N_{50}$ (`210` fern .. `860` matted graminoid).
    pub resistance_n50: f32,
    /// Seasonal biomass recovery rate $k_{\text{resil}}$ ($\text{s}^{-1}$).
    pub resilience_rate: f32,
    /// Turgor elastic rebound timescale (s).
    pub elastic_tau_s: f32,
    /// Decayed effective foot/hoof/wheel pass count.
    pub cumulative_passes: f32,
    /// Exposed mineral bare-soil fraction in `[0.0, 1.0]`.
    pub bare_soil_frac: f32,
    /// Vertical trail tread compaction depression $\to$ `.m64` (m).
    pub trail_compaction_m: f32,
    /// `0 = Graminoid`, `1 = Fern`, `2 = Forb`, `3 = ChamaephyteShrub`.
    pub life_form_kind: u32,
    /// Terrain grid cell index.
    pub cell_xy_idx: u32,
}

/// 48-byte POD fauna pelage mud, snow, pollen & epizoochorous burr entrainment zone.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PelageDebrisAttachment {
    /// Anatomical zone height above ground (hoof/hock/flank/breast, m).
    pub body_zone_height_m: f32,
    /// Pelage hair length (`0.03 m` straight .. `0.10 m` curled).
    pub hair_length_m: f32,
    /// Undulation/wool hook capture factor $c_{\text{fur}}$.
    pub fur_roughness_catch: f32,
    /// Expected attached epizoochorous diaspores (burrs/awns).
    pub burr_count_f: f32,
    /// Adhered floral pollen load (mg).
    pub pollen_mass_mg: f32,
    /// Accreted wet mud clump mass (kg).
    pub mud_mass_kg: f32,
    /// Accreted wet snow balling mass (kg).
    pub snow_mass_kg: f32,
    /// Pelage wetness in `[0.0, 1.0]` (reduces burr drop rate, increases mud adhesion).
    pub moisture_tack: f32,
    /// Current detachment hazard rate $\lambda_{\text{det}}$ ($\text{s}^{-1}$).
    pub detach_hazard_s_inv: f32,
    /// Carried diaspore taxon ID for `.q42` germination provenance.
    pub seed_species_id: u32,
    /// Linked `SKL2` bone index for distal limb inertia & visual tufts.
    pub skl2_zone_bone: u32,
    /// Host fauna entity ID.
    pub fauna_entity_id: u32,
}

/// 64-byte POD plant herbivory, bloodwood kino drip & insect leaf-mine/gall morphogenesis.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlantWoundKinoGall {
    /// Wound / oviposition site on `.10d` host organ (m).
    pub local_pos_m: [f32; 3],
    /// Stripped bark / herbivory bite area ($\text{m}^2$).
    pub wound_area_m2: f32,
    /// Downward-running bloodwood kino resin streak length (m).
    pub kino_drip_len_m: f32,
    /// Oxidative-curing resin viscosity $\eta_k(t)$ ($\text{Pa}\cdot\text{s}$).
    pub kino_viscosity_pa_s: f32,
    /// Glassy dark-red resin hardening fraction in `[0.0, 1.0]`.
    pub kino_cure_frac: f32,
    /// Larval-secreted auxin (`IAA`) phytohormone level.
    pub auxin_iaa_conc: f32,
    /// Larval-secreted cytokinin (`CK`) level (drives hypertrophy & green-island chlorophyll retention).
    pub cytokinin_ck_conc: f32,
    /// Hypertrophic SDF gall bulb radius $R_{\text{gall}}$ (m).
    pub gall_radius_m: f32,
    /// Sub-epidermal leaf-mining gallery length (m).
    pub mine_tunnel_len_m: f32,
    /// Preserved chlorophyll density inside cytokinin "green island" during autumn senescence.
    pub green_island_chl: f32,
    /// Bound `.10d` flora asset ID.
    pub host_entity_id: u32,
    /// Target UV/SDFB attachment patch ID.
    pub uv_patch_id: u32,
    /// `0 = BarkStripKino`, `1 = SphericalGall`, `2 = LeafMine`, `3 = HerbivoryBrowse`.
    pub lesion_kind: u32,
    /// `1` while gall/mine larva actively secretes morphogens.
    pub active_larva_flag: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<TramplingPersistenceCell>()];
const _: [(); 48] = [(); core::mem::size_of::<PelageDebrisAttachment>()];
const _: [(); 64] = [(); core::mem::size_of::<PlantWoundKinoGall>()];

/// Zero-heap Cole elasto-plastic trampling & `.m64` (`GRM3`) game-trail compaction stepper (20 Hz).
pub fn step_vegetation_trampling(
    cells: &mut [TramplingPersistenceCell],
    pass_impulses: &[f32],
    pass_dirs_xz: &[[f32; 2]],
    dt_s: f32,
    out_m64_delta_mm: &mut [i32],
) -> usize {
    let dt = dt_s.max(0.0);
    let ln2 = core::f32::consts::LN_2;
    let mut active_trails = 0usize;

    for (idx, cell) in cells.iter_mut().enumerate() {
        let passes = pass_impulses.get(idx).copied().unwrap_or(0.0).max(0.0);
        let dir = pass_dirs_xz.get(idx).copied().unwrap_or([1.0, 0.0]);

        if passes > 0.0 {
            cell.cumulative_passes += passes;
            let d_norm = (dir[0] * dir[0] + dir[1] * dir[1]).sqrt().max(1e-5);
            cell.bend_dir_xz = [dir[0] / d_norm, dir[1] / d_norm];
            cell.elastic_bend_rad = (cell.elastic_bend_rad + 0.45 * passes).min(1.42);
        }

        // Elastic turgor recovery vs. Cole plastic cover damage dD_p/dt
        let tau_el = cell.elastic_tau_s.max(10.0);
        cell.elastic_bend_rad *= (-dt / tau_el).exp();

        let n50 = cell.resistance_n50.max(25.0);
        let damage_inc = (ln2 / n50) * passes * (1.0 - cell.plastic_damage);
        let recovery = cell.resilience_rate.max(0.0) * cell.plastic_damage * (1.0 - cell.plastic_damage) * dt;
        cell.plastic_damage = (cell.plastic_damage + damage_inc - recovery).clamp(0.0, 1.0);
        cell.bare_soil_frac = (cell.plastic_damage * 0.85).clamp(0.0, 0.95);

        // Compacted game-trail tread depression into .m64 (guarded by 0.0005m narrowing check)
        cell.trail_compaction_m = (0.045 * cell.bare_soil_frac).clamp(0.0, 0.06);
        if idx < out_m64_delta_mm.len() {
            let guarded_m = project_metres_f32_checked(
                f64::from(-cell.trail_compaction_m),
                "trail_compaction_m",
            )
            .unwrap_or(0.0);
            out_m64_delta_mm[idx] = (guarded_m * 1000.0).round() as i32;
        }
        if cell.plastic_damage > 0.05 || cell.elastic_bend_rad > 0.1 {
            active_trails += 1;
        }
    }
    active_trails
}

/// Zero-heap Couvreur epizoochory burr dispersal & pelage mud/snow loading stepper (20 Hz).
pub fn step_pelage_debris_and_epizoochory(
    zones: &mut [PelageDebrisAttachment],
    veg_height_m: &[f32],
    seed_density: &[f32],
    walk_speed_m_s: &[f32],
    is_raining: bool,
    dt_s: f32,
    out_dropped_seeds: &mut [u32],
) -> u32 {
    let dt = dt_s.max(0.0);
    let mut total_dropped = 0u32;

    for (idx, zone) in zones.iter_mut().enumerate() {
        let h_veg = veg_height_m.get(idx).copied().unwrap_or(0.4).max(0.0);
        let rho_seed = seed_density.get(idx).copied().unwrap_or(0.0).max(0.0);
        let v_walk = walk_speed_m_s.get(idx).copied().unwrap_or(0.0).max(0.0);

        if is_raining {
            zone.moisture_tack = (zone.moisture_tack + 0.05 * dt).clamp(0.0, 1.0);
        } else {
            zone.moisture_tack = (zone.moisture_tack - 0.01 * dt).clamp(0.0, 1.0);
        }

        // Gaussian height overlap between plant infructescence height and animal body zone
        let dz = h_veg - zone.body_zone_height_m;
        let h_overlap = (-dz * dz / (2.0 * 0.18 * 0.18)).exp();
        let c_fur = zone.fur_roughness_catch.max(0.1) * (zone.hair_length_m / 0.05).clamp(0.3, 2.5);
        let attach_rate = 0.08 * h_overlap * c_fur * rho_seed * v_walk;

        // Rainy weather slows burr detachment; tall abrasive scrub accelerates detachment
        let rain_retention = 1.0 - 0.55 * zone.moisture_tack;
        let scrub_abrasion = 1.0 + 1.5 * h_overlap * v_walk;
        zone.detach_hazard_s_inv = 0.0012 * rain_retention * scrub_abrasion;

        let detached_f = (zone.burr_count_f * zone.detach_hazard_s_inv * dt).min(zone.burr_count_f);
        zone.burr_count_f = (zone.burr_count_f + attach_rate * dt - detached_f).max(0.0);

        let dropped_int = detached_f.round() as u32;
        total_dropped += dropped_int;
        if idx < out_dropped_seeds.len() {
            out_dropped_seeds[idx] = dropped_int;
        }
    }
    total_dropped
}

/// Steps Australian bloodwood kino Bingham-film resin exudation & larval IAA/CK gall morphogenesis (20 Hz).
pub fn step_plant_wound_kino_and_galls(lesions: &mut [PlantWoundKinoGall], dt_s: f32) -> usize {
    let dt = dt_s.max(0.0);
    let mut active = 0usize;

    for les in lesions.iter_mut() {
        if les.lesion_kind == 0 && les.kino_cure_frac < 0.99 {
            // Oxidative curing of polyphenol kino resin
            les.kino_viscosity_pa_s = (les.kino_viscosity_pa_s.max(0.5) * (0.008 * dt).exp()).min(1.0e5);
            les.kino_cure_frac = (les.kino_viscosity_pa_s.ln() / (1.0e5f32).ln()).clamp(0.0, 1.0);
            let v_kino = ((1150.0 * 9.81 * 0.002 * 0.002) / (3.0 * les.kino_viscosity_pa_s) - 1.0e-6).max(0.0);
            les.kino_drip_len_m = (les.kino_drip_len_m + v_kino * dt).min(0.65);
            active += 1;
        } else if (les.lesion_kind == 1 || les.lesion_kind == 2) && les.active_larva_flag != 0 {
            let morphogen = (les.auxin_iaa_conc + les.cytokinin_ck_conc).max(0.0);
            let r_max = 0.018f32;
            les.gall_radius_m += (r_max - les.gall_radius_m) * (1.0 - (-0.002 * morphogen * dt).exp());
            les.green_island_chl = (45.0 * (les.cytokinin_ck_conc / (les.cytokinin_ck_conc + 0.5))).clamp(0.0, 65.0);
            active += 1;
        }
    }
    active
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_fauna_passes_wear_persistent_m64_game_trail_and_bloodwood_bleeds_kino() {
        let mut trail = [TramplingPersistenceCell {
            bend_dir_xz: [1.0, 0.0],
            elastic_bend_rad: 0.0,
            plastic_damage: 0.0,
            resistance_n50: 210.0, // Fern understorey
            resilience_rate: 1.0e-7,
            elastic_tau_s: 120.0,
            cumulative_passes: 0.0,
            bare_soil_frac: 0.0,
            trail_compaction_m: 0.0,
            life_form_kind: 1,
            cell_xy_idx: 0,
        }];
        let mut m64_delta = [0i32; 1];
        step_vegetation_trampling(&mut trail, &[220.0], &[[1.0, 0.0]], 1.0, &mut m64_delta);
        assert!(trail[0].plastic_damage > 0.45, "220 passes on N50=210 fern must cause ~50% cover loss");
        assert!(m64_delta[0] <= -15, "compacted game trail must depress M64 terrain in millimetres");

        let mut lesions = [PlantWoundKinoGall {
            local_pos_m: [0.0, 1.5, 0.0],
            wound_area_m2: 0.01,
            kino_drip_len_m: 0.0,
            kino_viscosity_pa_s: 1.2,
            kino_cure_frac: 0.0,
            auxin_iaa_conc: 1.0,
            cytokinin_ck_conc: 1.5,
            gall_radius_m: 0.0,
            mine_tunnel_len_m: 0.0,
            green_island_chl: 0.0,
            host_entity_id: 1,
            uv_patch_id: 0,
            lesion_kind: 0, // BarkStripKino
            active_larva_flag: 1,
        }];
        step_plant_wound_kino_and_galls(&mut lesions, 120.0);
        assert!(lesions[0].kino_drip_len_m > 0.01);
        assert!(lesions[0].kino_cure_frac > 0.05);
    }
}

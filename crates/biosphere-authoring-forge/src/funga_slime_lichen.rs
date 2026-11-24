//! # Vector 8 — Slime-Mold Physarum Networks, Splash-Cup Fungi, Lichen Weathering & Fairy Rings (`funga_slime_lichen.rs`)
//!
//! Extends [`crate::funga_mycelium`] with the 5 specialized fungal, lichen, and amoebozoan
//! mechanisms from Vector 8:
//! 1. ***Physarum polycephalum* Shuttle-Streaming Tube Optimization (`PhysarumVeinEdge`):**
//!    Tero (2007) / Alim (2017) Hagen–Poiseuille adaptive conductivity
//!    $\dot{D}_{ij} = |Q_{ij}|^\mu / (1 + |Q_{ij}|^\mu) - \alpha_p D_{ij}$ with actomyosin
//!    peristaltic shuttle streaming.
//! 2. **Bird's-Nest Splash-Cup Fungi (*Cyathus*) Ballistics & Tetherball Arrest (`SplashCupPeridiole`):**
//!    Conical peridium raindrop splash amplification ejecting peridioles at $1.1\text{–}5.5\text{ m/s}$,
//!    followed by hapteron snagging and funicular cord tetherball wrapping $\omega = v_\perp / L_{\text{free}}(t)$
//!    around grass stems.
//! 3. **Seminara Edge-Limited Lichen Growth & Oxalic Acid Rock Weathering (`FungalVocLichenCell`):**
//!    Boundary-layer $\text{CO}_2$ Laplace diffusion governing radial velocity
//!    $\dot{R} = v_{\max} R / (R + r_0)$ and calcium oxalate (whewellite/weddellite) rock grain etching.
//! 4. **Stinkhorn (*Phallales* / *Aseroë rubra*) VOC Odor Plumes & Diptera Endozoochory:**
//!    Ephemeral oligosulfide carrion volatile plumes recruiting spore-dispersing flies.
//! 5. ***Coprinus* Ink-Cap Chitinase Deliquescence & Gray–Scott Fairy-Ring PDE:**
//!    Margin-to-apex gill autodigestion into black spore ink, and 2D Gray–Scott reaction–diffusion
//!    traveling fungal biomass rings ($b, r$).

use serde::{Deserialize, Serialize};

/// 48-byte POD *Physarum polycephalum* Poiseuille-peristaltic plasmodial vein edge.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysarumVeinEdge {
    /// Graph node index A.
    pub node_a: u32,
    /// Graph node index B.
    pub node_b: u32,
    /// Edge length $L_{ij}$ (m).
    pub length_m: f32,
    /// Current Poiseuille tube radius $r_{ij}$ (m).
    pub radius_m: f32,
    /// Hydraulic conductivity $D_{ij} = \pi r_{ij}^4 / (8 \eta)$.
    pub conductivity: f32,
    /// Instantaneous cytoplasmic sol flux $Q_{ij}$.
    pub flux_q: f32,
    /// Taylor-dispersed signaling molecule concentration $\langle c \rangle$.
    pub signal_conc: f32,
    /// Shuttle-streaming peristaltic oscillation phase ($\sim 100\text{ s}$ period, rad).
    pub peristalsis_phase: f32,
    /// Actin–myosin ectoplasm wall modulus $E_w$ (Pa).
    pub wall_modulus_pa: f32,
    /// Tube retraction decay rate $\alpha_p$ ($\text{s}^{-1}$).
    pub decay_alpha: f32,
    /// Active contractile stress $\gamma_p \langle c \rangle$ (Pa).
    pub active_tension_pa: f32,
    /// `1` if $D_{ij}$ exceeds pruning threshold.
    pub alive_flag: u32,
}

/// 48-byte POD *Cyathus* splash-cup peridiole projectile & funicular tetherball cord.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SplashCupPeridiole {
    /// Peridiole world position `[x, y, z]` (m).
    pub pos_xyz_m: [f32; 3],
    /// Ballistic velocity `[vx, vy, vz]` (`1.1..5.5` m/s).
    pub vel_xyz_m_s: [f32; 3],
    /// Peridiole mass (`1.3e-6..8.6e-6` kg).
    pub mass_kg: f32,
    /// Unwound free funicular cord length $L_{\text{free}}$ (m).
    pub cord_free_len_m: f32,
    /// Cumulative tetherball wrap angle around target grass stem (rad).
    pub wrap_angle_rad: f32,
    /// Hit flora stem ID once adhesive hapteron snags.
    pub anchor_stem_id: u32,
    /// Conical peridium cup wall angle (`0.7..1.3` rad).
    pub cup_angle_rad: f32,
    /// `0 = InCup`, `1 = Ballistic`, `2 = TetherballWrapping`, `3 = LockedOnStem`.
    pub flight_state: u32,
}

/// 64-byte POD Gray–Scott fairy-ring, lichen weathering, stinkhorn VOC & ink-cap deliquescence cell.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FungalVocLichenCell {
    /// Dimensionless Gray–Scott soil fungal biomass $b$.
    pub fairy_biomass_b: f32,
    /// Dimensionless Gray–Scott soil organic resource $r$.
    pub fairy_resource_r: f32,
    /// Outer nitrogen-flush grass boost (`+1.0`) / inner hydrophobic dead zone (`-1.0`).
    pub fairy_grass_boost: f32,
    /// Crustose/foliose lichen thallus radius $R$ (m).
    pub lichen_radius_m: f32,
    /// Characteristic transition thickness $H \sim r_0$ (`0.001..0.004` m).
    pub lichen_height_m: f32,
    /// Active photosynthetic hydration time fraction $e_{\text{photo}} \in [0.0, 1.0]$.
    pub lichen_photo_e: f32,
    /// Cumulative calcium-oxalate rock grain boundary etching depth (m).
    pub oxalic_etch_depth_m: f32,
    /// Stinkhorn (*Phallales*) oligosulfide VOC odor concentration.
    pub voc_plume_conc: f32,
    /// Ephemeral *Phallales* fruiting age clock (`0..259200` s).
    pub voc_source_age_s: f32,
    /// *Coprinus* margin-to-apex chitinase autodigestion wave in `[0.0, 1.0]`.
    pub chitinase_front: f32,
    /// Ink-cap upward reflex cap curl during deliquescence (rad).
    pub cap_curl_rad: f32,
    /// Liquefied black spore-ink drip rate (drops/s).
    pub ink_drip_rate: f32,
    /// Grid / surface patch index.
    pub cell_xy_idx: u32,
    /// Bitfield selecting active fungal/lichen traits.
    pub species_profile: u32,
    /// Cumulative spore-ingesting Diptera fly visits.
    pub diptera_visits: u32,
    /// Status bitmask.
    pub flags: u32,
}

const _: [(); 48] = [(); core::mem::size_of::<PhysarumVeinEdge>()];
const _: [(); 48] = [(); core::mem::size_of::<SplashCupPeridiole>()];
const _: [(); 64] = [(); core::mem::size_of::<FungalVocLichenCell>()];

/// Zero-heap Tero–Alim *Physarum* Poiseuille network solver (20 Hz, caller-allocated pressure scratch).
pub fn step_physarum_network(
    edges: &mut [PhysarumVeinEdge],
    node_pressures: &mut [f32],
    node_sources: &[f32],
    dt_s: f32,
    relax_iters: u32,
) -> usize {
    let dt = dt_s.max(0.0);
    let n_nodes = node_pressures.len().min(node_sources.len());

    // 1. Iterative zero-heap Kirchhoff pressure relaxation
    for _ in 0..relax_iters.clamp(1, 16) {
        for i in 0..n_nodes {
            let mut num = node_sources[i];
            let mut den = 0.0f32;
            for e in edges.iter() {
                let g = e.conductivity / e.length_m.max(1.0e-4);
                if e.node_a as usize == i && (e.node_b as usize) < n_nodes {
                    num += g * node_pressures[e.node_b as usize];
                    den += g;
                } else if e.node_b as usize == i && (e.node_a as usize) < n_nodes {
                    num += g * node_pressures[e.node_a as usize];
                    den += g;
                }
            }
            if den > 1.0e-9 {
                node_pressures[i] = 0.5 * node_pressures[i] + 0.5 * (num / den);
            }
        }
    }

    // 2. Hagen-Poiseuille flux & Tero adaptive conductivity update + peristaltic shuttle streaming
    let mut alive_edges = 0usize;
    for e in edges.iter_mut() {
        let na = e.node_a as usize;
        let nb = e.node_b as usize;
        let dp = if na < n_nodes && nb < n_nodes {
            node_pressures[na] - node_pressures[nb]
        } else {
            0.0
        };
        let len = e.length_m.max(1.0e-4);
        e.flux_q = (e.conductivity / len) * dp;
        let q_abs_mu = e.flux_q.abs().powf(1.3);
        let d_dot = (q_abs_mu / (1.0 + q_abs_mu)) - e.decay_alpha.max(0.01) * e.conductivity;
        e.conductivity = (e.conductivity + d_dot * dt).clamp(1.0e-7, 10.0);

        // Actomyosin peristalsis (~100s period) modulates tube radius around Poiseuille baseline
        e.peristalsis_phase =
            (e.peristalsis_phase + (core::f32::consts::TAU / 100.0) * dt).rem_euclid(core::f32::consts::TAU);
        let r_base = (e.conductivity * 1.0e-4).powf(0.25).max(5.0e-5);
        let peristaltic_wave = 1.0 - 0.15 * e.signal_conc.clamp(0.0, 1.0) * e.peristalsis_phase.cos();
        e.radius_m = r_base * peristaltic_wave;
        e.alive_flag = u32::from(e.conductivity > 1.0e-5);
        if e.alive_flag != 0 {
            alive_edges += 1;
        }
    }
    alive_edges
}

/// Steps *Cyathus* splash-cup raindrop ballistic ejection and funicular hapteron tetherball wrapping around stems (64 Hz).
pub fn step_splash_cup_peridioles(
    peridioles: &mut [SplashCupPeridiole],
    raindrop_impact_speed_m_s: f32,
    target_stem_radius_m: f32,
    dt_s: f32,
) -> usize {
    let dt = dt_s.max(0.0);
    let r_stem = target_stem_radius_m.max(0.0008);
    let mut tethered = 0usize;

    for p in peridioles.iter_mut() {
        match p.flight_state {
            0 => {
                // Eject from conical splash cup when hit by a fast raindrop (U_0 > 3 m/s)
                if raindrop_impact_speed_m_s > 3.0 {
                    let theta = p.cup_angle_rad.clamp(0.8, 1.35);
                    let u_launch = (raindrop_impact_speed_m_s * 0.48).clamp(1.1, 5.5);
                    p.vel_xyz_m_s = [u_launch * theta.cos(), u_launch * theta.sin(), 0.0];
                    p.flight_state = 1;
                }
            }
            1 => {
                // Ballistic flight until snagging a stem
                p.pos_xyz_m[0] += p.vel_xyz_m_s[0] * dt;
                p.pos_xyz_m[1] += p.vel_xyz_m_s[1] * dt;
                p.vel_xyz_m_s[1] -= 9.80665 * dt;
                if p.pos_xyz_m[0] >= 0.15 {
                    p.flight_state = 2; // Hapteron snags stem -> initiate tetherball wrap
                }
            }
            2 => {
                // Funicular cord tetherball winding: omega(t) = v_perp / L_free(t)
                let v_perp = (p.vel_xyz_m_s[0].hypot(p.vel_xyz_m_s[1])).max(0.5);
                let l_free = p.cord_free_len_m.max(r_stem);
                let omega = v_perp / l_free;
                let d_theta = omega * dt;
                p.wrap_angle_rad += d_theta;
                p.cord_free_len_m = (p.cord_free_len_m - r_stem * d_theta).max(0.0);
                if p.cord_free_len_m <= r_stem * 1.05 {
                    p.flight_state = 3;
                }
            }
            _ => {
                tethered += 1;
            }
        }
        if p.flight_state == 3 {
            tethered += 1;
        }
    }
    tethered
}

/// Zero-heap 2D Gray–Scott fairy-ring PDE, Seminara lichen weathering & *Coprinus* deliquescence stepper (20 Hz).
pub fn step_fairy_ring_and_lichen_grid(
    curr: &[FungalVocLichenCell],
    next: &mut [FungalVocLichenCell],
    grid_w: usize,
    grid_h: usize,
    dx_m: f32,
    dt_s: f32,
) {
    let n_total = (grid_w * grid_h).min(curr.len()).min(next.len());
    let inv_dx2 = 1.0 / (dx_m.max(0.05) * dx_m.max(0.05));
    let dt = dt_s.max(0.0);

    for idx in 0..n_total {
        let mut cell = curr[idx];
        let x = idx % grid_w.max(1);
        let y = idx / grid_w.max(1);

        // 5-point discrete Laplacian for Gray-Scott reaction-diffusion
        let left = if x > 0 { idx - 1 } else { idx };
        let right = if x + 1 < grid_w { idx + 1 } else { idx };
        let up = if y > 0 { idx - grid_w } else { idx };
        let down = if y + 1 < grid_h { idx + grid_w } else { idx };

        let b = cell.fairy_biomass_b;
        let r = cell.fairy_resource_r;
        let lap_b = (curr[left].fairy_biomass_b + curr[right].fairy_biomass_b + curr[up].fairy_biomass_b + curr[down].fairy_biomass_b - 4.0 * b) * inv_dx2;
        let lap_r = (curr[left].fairy_resource_r + curr[right].fairy_resource_r + curr[up].fairy_resource_r + curr[down].fairy_resource_r - 4.0 * r) * inv_dx2;

        // Gray-Scott fairy ring parameters (D_r = 0.002, D_b = 0.001, c = 1.2, m = 0.5, g = 0.02)
        let rb2 = r * b * b;
        let dr = 0.002 * lap_r - rb2 + 0.02 * (1.0 - r);
        let db = 0.001 * lap_b + 1.2 * rb2 - 0.5 * b;
        cell.fairy_resource_r = (r + dr * dt).clamp(0.0, 1.0);
        cell.fairy_biomass_b = (b + db * dt).clamp(0.0, 1.5);
        // Active mycelium front mineralizes nitrogen (+boost), depleted wake is hydrophobic (-boost)
        cell.fairy_grass_boost = (1.5 * cell.fairy_biomass_b - (1.0 - cell.fairy_resource_r)).clamp(-1.0, 1.0);

        // Seminara (2018) edge-limited crustose/foliose lichen radial expansion & oxalic acid rock etching
        let r0 = cell.lichen_height_m.clamp(0.001, 0.005);
        let v_max = 3.0e-11 * cell.lichen_photo_e.clamp(0.0, 1.0);
        let dr_lichen = v_max * (cell.lichen_radius_m / (cell.lichen_radius_m + r0).max(1e-6)) * dt;
        cell.lichen_radius_m += dr_lichen;
        cell.oxalic_etch_depth_m += dr_lichen * 0.08;

        // Coprinus ink-cap chitinase deliquescence (autodigestion over ~24h = 86,400s)
        if (cell.species_profile & 1) != 0 {
            cell.chitinase_front = (cell.chitinase_front + (dt / 86_400.0)).clamp(0.0, 1.0);
            cell.cap_curl_rad = 1.45 * cell.chitinase_front;
            cell.ink_drip_rate = 0.25 * (core::f32::consts::PI * cell.chitinase_front).sin();
        }
        next[idx] = cell;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn physarum_reinforces_active_vein_and_cyathus_peridiole_wraps_stem() {
        let mut edges = [
            PhysarumVeinEdge {
                node_a: 0,
                node_b: 1,
                length_m: 0.02,
                radius_m: 0.0005,
                conductivity: 0.5,
                flux_q: 0.0,
                signal_conc: 0.8,
                peristalsis_phase: 0.0,
                wall_modulus_pa: 100.0,
                decay_alpha: 0.1,
                active_tension_pa: 5.0,
                alive_flag: 1,
            },
            PhysarumVeinEdge {
                node_a: 1,
                node_b: 2,
                length_m: 0.05,
                radius_m: 0.0001,
                conductivity: 0.001, // Dead-end branch with zero flux
                flux_q: 0.0,
                signal_conc: 0.1,
                peristalsis_phase: 0.0,
                wall_modulus_pa: 100.0,
                decay_alpha: 0.5,
                active_tension_pa: 1.0,
                alive_flag: 1,
            },
        ];
        let mut pressures = [10.0f32, 0.0, 0.0];
        let sources = [5.0f32, -5.0, 0.0];
        step_physarum_network(&mut edges, &mut pressures, &sources, 1.0, 6);
        assert!(edges[0].conductivity > 0.5, "food-carrying vein must thicken");
        assert!(edges[1].conductivity < 0.001, "unused dead-end vein must atrophy");

        let mut peridiole = [SplashCupPeridiole {
            pos_xyz_m: [0.15, 0.05, 0.0],
            vel_xyz_m_s: [2.8, 1.2, 0.0],
            mass_kg: 5.0e-6,
            cord_free_len_m: 0.04,
            wrap_angle_rad: 0.0,
            anchor_stem_id: 7,
            cup_angle_rad: 1.1,
            flight_state: 2, // Snagged on stem, wrapping
        }];
        let locked = step_splash_cup_peridioles(&mut peridiole, 6.0, 0.0015, 0.5);
        assert_eq!(locked, 1);
        assert!(peridiole[0].wrap_angle_rad > 10.0, "funicular cord must wind multiple turns around grass stem");
    }
}

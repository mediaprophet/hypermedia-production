//! # Funga, Mycorrhizal Anastomosis Networks & Fruiting-Body Morphogenesis (`funga_mycelium.rs`)
//!
//! Brings the Third Biological Kingdom (**Funga**) into `biosphere-authoring-forge` with
//! deterministic, zero-heap solvers for:
//! 1. **3D Hyphal Foraging, Branching & Anastomosis ($b_1 > 0$ Betti Loops):**
//!    Unlike vascular tree crowns (which strictly branch divergently as trees with $b_1 = 0$),
//!    fungal mycelium both branches at the apex and fuses laterally (**anastomosis**) when a
//!    foraging hyphal tip approaches an existing hyphal strand within `anastomosis_radius_m`,
//!    forming cyclic transport networks ($b_1 = E - V + C > 0$) that redistribute carbon,
//!    nitrogen, phosphorus, and water between host tree root systems (Common Mycorrhizal Network).
//! 2. **Basidiomycete & Ascomycete Fruiting-Body Morphogenesis:**
//!    Parametric & SDF morphogenesis from primordium pinhead $\to$ negative-gravitropic stipe
//!    (curving upward from inclined bark/soil so the pileus always levels horizontally) $\to$
//!    pileus (umbrella/funnel/bracket) expansion + partial veil rupture.
//! 3. **Turing-Bifurcating Hymenophore Gills (`Lamellae`) & Voronoi Poroid Tubes:**
//!    As pileus circumference $2\pi r$ expands with radial distance $r$ from the stipe, new
//!    secondary and tertiary lamellae nucleate (intercalate) whenever inter-gill spacing
//!    exceeds $2 \lambda_{\text{gill}}$, maintaining constant optimal spacing for Buller's drop
//!    ballistic spore discharge.
//! 4. **Ballistic Spore Discharge (Buller's Drop), Stokes Settling & Wind Advection:**
//!    Models horizontal basidiospore ejection ($a \sim 10^4 g$, stopping at distance $x_{\text{stop}}$
//!    via viscous Stokes drag), vertical gravity settling velocity $v_s$, and downwind Gaussian
//!    plume dispersion.
//! 5. **Fungal Bioluminescence (`Q42EMF64`) & Enzymatic Deadwood Decay Softening:**
//!    Models luciferin / 3-hydroxyhispidin oxidation ($\lambda_{\text{peak}} = 525\text{ nm}$ green
//!    cold emission, as in *Omphalotus nidiformis* ghost fungi and *Mycena*) directly into 64-byte
//!    physical-metre `CuticleEmfSpectralCell` records, plus white-rot/brown-rot lignocellulose
//!    modulus reduction $E(t) = E_0 e^{-k_{\text{decay}} t}$.

use crate::arthropod::CuticleEmfSpectralCell;
use serde::{Deserialize, Serialize};

/// Maximum hyphal nodes in a zero-heap mycorrhizal foraging arena.
pub const MAX_HYPHAL_NODES: usize = 256;
/// Maximum hyphal edges (including cyclic anastomosis bridges) in a zero-heap arena.
pub const MAX_HYPHAL_EDGES: usize = 384;
/// Maximum nutrient / root-exudate attractors in a zero-heap foraging pass.
pub const MAX_MYCORRHIZAL_ATTRACTORS: usize = 64;

/// Ecological guild of a fungal species.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FungalEcologicalGuild {
    /// Forms Hartig net & mantle sheath around host tree root tips (e.g., *Amanita*, *Boletus*, *Pisolithus*).
    Ectomycorrhizal,
    /// Penetrates cortical root cells forming arbuscules (*Glomeromycota*).
    ArbuscularMycorrhizal,
    /// Decomposes both lignin and cellulose, bleaching wood (*Trametes versicolor*, *Omphalotus*).
    SaprotrophicWhiteRot,
    /// Cleaves cellulose while leaving modified lignin matrix (*Fomitopsis*, *Serpula*).
    SaprotrophicBrownRot,
    /// Endophytic / lichenized symbiont.
    LichenizedSymbiont,
}

/// Fruiting-body morphologicalbauplan and hymenophore (spore-bearing surface) architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HymenophoreMorphology {
    /// Radial Turing-bifurcating gills (`lamellae`) under a pileus cap (e.g., Agaricales).
    BifurcatingLamellae,
    /// Packed vertical poroid tubes under a fleshy or woody cap (e.g., Boletales, Polyporales).
    PoroidTubes,
    /// Sessile lateral shelf / bracket on vertical deadwood trunks.
    SessileBracketPolypore,
    /// Upright branching coralloid clubs (*Clavaria*, *Ramaria*).
    CoralloidBranching,
    /// Concave apothecial cup (*Peziza*, *Cookeina*) with ascus phototropic spore cannon.
    ApothecialCup,
    /// Closed gasteroid gleba releasing spores through an apical ostiole (*Lycoperdon* puffball).
    GasteroidPuffball,
}

/// A single 3D node in a hyphal / mycorrhizal network.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HyphalNode {
    /// 3D world or substrate-local position `[x, y, z]` in metres (`y` is vertical elevation).
    pub position_m: [f32; 3],
    /// Hyphal cord / rhizomorph radius in metres (`2e-6` single hypha to `2e-3` rhizomorph cord).
    pub radius_m: f32,
    /// Carbon/nutrient flux potential at this node in `[0.0, 1.0]`.
    pub nutrient_potential: f32,
    /// Host root ID if this node has formed a mycorrhizal Hartig-net sheath (`u16::MAX` if free-living).
    pub attached_host_root_id: u16,
    /// Active growing apex flag (`1` = extending tip, `0` = mature/fused interior node).
    pub is_active_tip: u8,
    /// Branch generation depth from the spore/inoculum origin.
    pub generation: u8,
}

/// Directed or fused transport edge between two hyphal nodes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HyphalEdge {
    pub from_node: u16,
    pub to_node: u16,
    /// `true` if this edge was formed by lateral hyphal fusion (**anastomosis**), creating a cycle.
    pub is_anastomosis_bridge: bool,
    /// Hydraulic / cytoplasmic translocation conductance.
    pub conductance: f32,
}

/// Substrate nutrient or host-tree root exudate attractor driving hyphal chemotaxis.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MycorrhizalAttractor {
    pub position_m: [f32; 3],
    /// Chemotactic concentration (sugars, amino acids, strigolactones, moisture) in `[0.0, 1.0]`.
    pub concentration: f32,
    /// If `< u16::MAX`, identifies a host plant root tip seeking mycorrhizal symbiosis.
    pub host_root_id: u16,
    /// `true` once colonized by a hyphal tip.
    pub colonized: bool,
}

/// Summary receipt from a zero-heap 3D mycorrhizal foraging & anastomosis pass.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MyceliumNetworkReceipt {
    pub node_count: usize,
    pub edge_count: usize,
    /// Number of lateral hyphal fusion events that created closed transport loops.
    pub anastomosis_loop_count: usize,
    /// First Betti number $b_1 = E - V + 1$ measuring cyclic transport redundancy.
    pub first_betti_number_b1: usize,
    /// Number of host tree root tips successfully linked into the Common Mycorrhizal Network.
    pub colonized_host_roots: usize,
    /// Total length of the mycelial network in metres.
    pub total_hyphal_length_m: f32,
}

/// Zero-heap 3D hyphal foraging, apical branching, and lateral anastomosis solver.
///
/// Grows a mycelial graph from initial inoculum nodes toward `attractors` (root exudates and
/// decaying lignocellulose hotspots). When an extending tip comes within `anastomosis_radius_m`
/// of a non-ancestor hyphal node, it fuses (**anastomosis**) to form a cyclic loop ($b_1 > 0$).
pub fn step_mycelium_foraging_and_anastomosis_zero_heap(
    nodes: &mut [HyphalNode],
    initial_node_count: usize,
    edges: &mut [HyphalEdge],
    initial_edge_count: usize,
    attractors: &mut [MycorrhizalAttractor],
    step_length_m: f32,
    influence_radius_m: f32,
    anastomosis_radius_m: f32,
    steps: usize,
) -> MyceliumNetworkReceipt {
    let max_nodes = nodes.len().min(MAX_HYPHAL_NODES);
    let max_edges = edges.len().min(MAX_HYPHAL_EDGES);
    let mut node_count = initial_node_count.min(max_nodes);
    let mut edge_count = initial_edge_count.min(max_edges);
    let mut anastomosis_loops = 0usize;
    let mut colonized_roots = 0usize;
    let step_len = step_length_m.max(0.002);
    let inf_r2 = (influence_radius_m.max(step_len * 2.0)).powi(2);
    let fuse_r2 = (anastomosis_radius_m.max(step_len * 0.75)).powi(2);

    for _step in 0..steps {
        let snapshot_nodes = node_count;
        for idx in 0..snapshot_nodes {
            if nodes[idx].is_active_tip == 0 {
                continue;
            }
            let pos = nodes[idx].position_m;

            // 1. Evaluate chemotactic gradient toward uncolonized attractors
            let mut dir = [0.0f32; 3];
            let mut weight_sum = 0.0f32;
            let mut nearest_att: Option<usize> = None;
            let mut nearest_d2 = f32::MAX;

            for (a_idx, att) in attractors.iter().enumerate() {
                if att.colonized {
                    continue;
                }
                let dx = att.position_m[0] - pos[0];
                let dy = att.position_m[1] - pos[1];
                let dz = att.position_m[2] - pos[2];
                let d2 = dx * dx + dy * dy + dz * dz;
                if d2 <= inf_r2 && d2 > 1e-10 {
                    let d = d2.sqrt();
                    let w = att.concentration.max(0.05) / (d2 + 1e-4);
                    dir[0] += (dx / d) * w;
                    dir[1] += (dy / d) * w;
                    dir[2] += (dz / d) * w;
                    weight_sum += w;
                    if d2 < nearest_d2 {
                        nearest_d2 = d2;
                        nearest_att = Some(a_idx);
                    }
                }
            }

            if weight_sum <= 1e-8 {
                continue;
            }
            let d_norm = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
            if d_norm <= 1e-8 {
                continue;
            }
            let unit_dir = [dir[0] / d_norm, dir[1] / d_norm, dir[2] / d_norm];
            let new_pos = [
                pos[0] + unit_dir[0] * step_len,
                pos[1] + unit_dir[1] * step_len,
                pos[2] + unit_dir[2] * step_len,
            ];

            // 2. Check if an attractor is reached (Hartig-net root colonization)
            let mut attached_root = u16::MAX;
            if let Some(a_idx) = nearest_att {
                if nearest_d2 <= (step_len * 1.6).powi(2) {
                    attractors[a_idx].colonized = true;
                    attached_root = attractors[a_idx].host_root_id;
                    if attached_root != u16::MAX {
                        colonized_roots += 1;
                    }
                }
            }

            // 3. Spawn new hyphal apex node and primary transport edge
            if node_count >= max_nodes || edge_count >= max_edges {
                break;
            }
            let new_idx = node_count;
            let next_gen = nodes[idx].generation.saturating_add(1);
            nodes[new_idx] = HyphalNode {
                position_m: new_pos,
                radius_m: (nodes[idx].radius_m * 0.96).max(5e-5),
                nutrient_potential: (nodes[idx].nutrient_potential * 0.98).clamp(0.1, 1.0),
                attached_host_root_id: attached_root,
                is_active_tip: 1,
                generation: next_gen,
            };
            edges[edge_count] = HyphalEdge {
                from_node: idx as u16,
                to_node: new_idx as u16,
                is_anastomosis_bridge: false,
                conductance: nodes[new_idx].radius_m * 1000.0,
            };
            node_count += 1;
            edge_count += 1;

            // Keep parent tip active for dichotomous branching every 3rd generation, otherwise mature it
            if next_gen % 3 != 0 {
                nodes[idx].is_active_tip = 0;
            }

            // 4. Lateral Anastomosis check: fuse with nearby non-parent hyphal strand to create a Betti loop
            if edge_count < max_edges {
                for other_idx in 0..snapshot_nodes {
                    if other_idx == idx {
                        continue;
                    }
                    let ox = nodes[other_idx].position_m[0] - new_pos[0];
                    let oy = nodes[other_idx].position_m[1] - new_pos[1];
                    let oz = nodes[other_idx].position_m[2] - new_pos[2];
                    let od2 = ox * ox + oy * oy + oz * oz;
                    if od2 <= fuse_r2 && od2 > 1e-9 {
                        // Ensure we don't duplicate an existing edge
                        let already_linked = edges[..edge_count].iter().any(|e| {
                            (e.from_node as usize == new_idx && e.to_node as usize == other_idx)
                                || (e.from_node as usize == other_idx && e.to_node as usize == new_idx)
                        });
                        if !already_linked {
                            edges[edge_count] = HyphalEdge {
                                from_node: new_idx as u16,
                                to_node: other_idx as u16,
                                is_anastomosis_bridge: true,
                                conductance: nodes[new_idx].radius_m * 850.0,
                            };
                            edge_count += 1;
                            anastomosis_loops += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    // Reinforce rhizomorph cord radii along anastomosis bridges and compute total length
    let mut total_len = 0.0f32;
    for e_idx in 0..edge_count {
        let e = edges[e_idx];
        let a = nodes[e.from_node as usize].position_m;
        let b = nodes[e.to_node as usize].position_m;
        let dx = b[0] - a[0];
        let dy = b[1] - a[1];
        let dz = b[2] - a[2];
        total_len += (dx * dx + dy * dy + dz * dz).sqrt();
        if e.is_anastomosis_bridge {
            nodes[e.from_node as usize].radius_m *= 1.15;
            nodes[e.to_node as usize].radius_m *= 1.15;
        }
    }

    let b1 = (edge_count + 1).saturating_sub(node_count);

    MyceliumNetworkReceipt {
        node_count,
        edge_count,
        anastomosis_loop_count: anastomosis_loops,
        first_betti_number_b1: b1.max(anastomosis_loops),
        colonized_host_roots: colonized_roots,
        total_hyphal_length_m: total_len,
    }
}

/// Morphogenesis and biophysical profile for a fungal fruiting body (sporocarp).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FungalFruitingBodySpec {
    pub guild: FungalEcologicalGuild,
    pub hymenophore: HymenophoreMorphology,
    /// Stipe (stem) base radius in metres (`0.0` for sessile brackets).
    pub stipe_base_radius_m: f32,
    /// Stipe apex radius in metres.
    pub stipe_apex_radius_m: f32,
    /// Stipe length in metres.
    pub stipe_height_m: f32,
    /// Substrate surface normal `[nx, ny, nz]` where the primordium erupts; the stipe curves
    /// via negative gravitropism toward `[0.0, 1.0, 0.0]` so the hymenophore is horizontal.
    pub substrate_normal: [f32; 3],
    /// Pileus (cap) outer radius in metres.
    pub pileus_radius_m: f32,
    /// Pileus developmental maturity in `[0.0, 1.0]` (`0.1` button primordium -> `0.5` convex umbrella -> `1.0` plane/funnel).
    pub pileus_maturity: f32,
    /// Optimal inter-gill spacing $\lambda_{\text{gill}}$ in metres (typically `0.0008` to `0.0015` m).
    pub target_gill_spacing_m: f32,
    /// Gill depth in metres.
    pub gill_depth_m: f32,
    /// Nocturnal bioluminescence quantum yield in `[0.0, 1.0]` (`0.85` for *Omphalotus nidiformis*).
    pub bioluminescence_intensity: f32,
    /// Mean spore radius in metres (typically `3.5e-6` to `6.0e-6` m).
    pub spore_radius_m: f32,
}

impl FungalFruitingBodySpec {
    /// Preset for the Australian Ghost Fungus (*Omphalotus nidiformis*): pleurotoid fan/funnel
    /// fruiting on dead eucalypt wood with bifurcate decurrent gills and 525 nm bioluminescence.
    #[inline]
    pub const fn ghost_fungus_omphalotus() -> Self {
        Self {
            guild: FungalEcologicalGuild::SaprotrophicWhiteRot,
            hymenophore: HymenophoreMorphology::BifurcatingLamellae,
            stipe_base_radius_m: 0.009,
            stipe_apex_radius_m: 0.012,
            stipe_height_m: 0.055,
            substrate_normal: [0.866, 0.5, 0.0],
            pileus_radius_m: 0.065,
            pileus_maturity: 0.85,
            target_gill_spacing_m: 0.0011,
            gill_depth_m: 0.0055,
            bioluminescence_intensity: 0.88,
            spore_radius_m: 4.2e-6,
        }
    }

    /// Evaluates the negative-gravitropic stipe centerline `[x, y, z]` and tangent at normalized
    /// arc parameter $u \in [0.0, 1.0]$. Even if the fungus sprouts horizontally from the side
    /// of a tree trunk, the stipe curves smoothly upward so the pileus cap sits level.
    #[inline]
    pub fn evaluate_gravitropic_stipe_frame(&self, u: f32) -> ([f32; 3], [f32; 3], f32) {
        let t = u.clamp(0.0, 1.0);
        let n_len = (self.substrate_normal[0] * self.substrate_normal[0]
            + self.substrate_normal[1] * self.substrate_normal[1]
            + self.substrate_normal[2] * self.substrate_normal[2])
            .sqrt()
            .max(1e-6);
        let n0 = [
            self.substrate_normal[0] / n_len,
            self.substrate_normal[1] / n_len,
            self.substrate_normal[2] / n_len,
        ];
        // Quadratic Bezier curve from initial eruption normal n0 to vertical [0, 1, 0]
        let h = self.stipe_height_m.max(0.002);
        let p1 = [n0[0] * h * 0.45, n0[1] * h * 0.45, n0[2] * h * 0.45];
        let p2 = [p1[0], p1[1] + h * 0.65, p1[2]];
        let omt = 1.0 - t;
        let pos = [
            2.0 * omt * t * p1[0] + t * t * p2[0],
            2.0 * omt * t * p1[1] + t * t * p2[1],
            2.0 * omt * t * p1[2] + t * t * p2[2],
        ];
        let dx = 2.0 * (1.0 - 2.0 * t) * p1[0] + 2.0 * t * p2[0];
        let dy = 2.0 * (1.0 - 2.0 * t) * p1[1] + 2.0 * t * p2[1];
        let dz = 2.0 * (1.0 - 2.0 * t) * p1[2] + 2.0 * t * p2[2];
        let t_len = (dx * dx + dy * dy + dz * dz).sqrt().max(1e-6);
        let tangent = [dx / t_len, dy / t_len, dz / t_len];
        let radius = self.stipe_base_radius_m * (1.0 - t) + self.stipe_apex_radius_m * t;
        (pos, tangent, radius)
    }

    /// Evaluates Turing-bifurcating gill (`lamellae`) count and surface relief at radial distance
    /// $r \in [r_{\text{stipe}}, R_{\text{pileus}}]$ and polar angle $\theta \in [0, 2\pi)$.
    ///
    /// As circumference $C(r) = 2\pi r$ grows with $r$, the number of radial gills doubles by
    /// binary intercalation $N_{\text{gills}}(r) = N_0 \cdot 2^{k(r)}$ so inter-gill spacing
    /// stays bounded near `target_gill_spacing_m`.
    #[inline]
    pub fn evaluate_turing_lamellae_at(&self, r_m: f32, theta_rad: f32) -> (u32, f32) {
        let r_min = self.stipe_apex_radius_m.max(0.002);
        let r = r_m.clamp(r_min, self.pileus_radius_m.max(r_min + 0.001));
        let spacing = self.target_gill_spacing_m.max(0.0004);
        let base_gills = ((2.0 * core::f32::consts::PI * r_min) / spacing)
            .round()
            .max(12.0) as u32;
        let ratio = (r / r_min).max(1.0);
        let bifurcation_tier = ratio.log2().floor().max(0.0) as u32;
        let active_gills = base_gills.saturating_mul(1u32 << bifurcation_tier.min(6));

        // Combine primary gill wave + secondary intercalated gill wave
        let phase_primary = (active_gills as f32 * 0.5) * theta_rad;
        let phase_secondary = (active_gills as f32) * theta_rad;
        let tier_blend = (ratio.log2() - bifurcation_tier as f32).clamp(0.0, 1.0);
        let primary_ridge = phase_primary.cos().max(0.0).powi(3);
        let secondary_ridge = phase_secondary.cos().max(0.0).powi(3) * tier_blend;
        let relief_m = self.gill_depth_m * primary_ridge.max(secondary_ridge);
        (active_gills, relief_m)
    }

    /// Evaluates ballistic Buller's drop basidiospore discharge range, Stokes terminal settling
    /// velocity $v_s$ (m/s), and downwind Gaussian plume concentration at `(downwind_x_m, crosswind_y_m, delta_z_m)`.
    #[inline]
    pub fn evaluate_spore_plume_concentration(
        &self,
        wind_speed_m_s: f32,
        turbulence_intensity: f32,
        downwind_x_m: f32,
        crosswind_y_m: f32,
        delta_z_m: f32,
    ) -> (f32, f32) {
        // Stokes settling velocity v_s = 2 * r^2 * (rho_spore - rho_air) * g / (9 * mu_air)
        let r = self.spore_radius_m.clamp(1.0e-6, 25.0e-6);
        let rho_spore = 1150.0f32;
        let mu_air = 1.81e-5f32;
        let g = 9.80665f32;
        let v_settling = (2.0 * r * r * rho_spore * g) / (9.0 * mu_air);

        if downwind_x_m <= 0.01 {
            return (v_settling, 0.0);
        }
        let u = wind_speed_m_s.max(0.08);
        let turb = turbulence_intensity.clamp(0.04, 0.65);
        let sigma_y = turb * downwind_x_m.powf(0.85) + 0.015;
        let sigma_z = (turb * 0.75) * downwind_x_m.powf(0.85) + 0.010;
        let z_center = -(v_settling / u) * downwind_x_m;
        let dy2 = (crosswind_y_m * crosswind_y_m) / (2.0 * sigma_y * sigma_y);
        let dz2 = ((delta_z_m - z_center) * (delta_z_m - z_center)) / (2.0 * sigma_z * sigma_z);
        let conc = ((-dy2 - dz2).exp()
            / (2.0 * core::f32::consts::PI * u * sigma_y * sigma_z))
            .clamp(0.0, 1.0e4);
        (v_settling, conc)
    }

    /// Synthesizes a 64-byte (`16 × f32`) `Q42EMF64` spectral cell for fungal bioluminescence
    /// (525 nm cold green luciferin emission) and moist chitinous pileus reflectance.
    #[inline]
    pub fn synthesize_bioluminescent_emf_v4_cell(
        &self,
        surface_normal: [f32; 3],
        ambient_lux_factor: f32,
    ) -> CuticleEmfSpectralCell {
        // Circadian / darkness gating: bioluminescence is brightest in nocturnal shade
        let dark_gate = (1.0 - ambient_lux_factor.clamp(0.0, 1.0)).max(0.05);
        let emit_power = self.bioluminescence_intensity.clamp(0.0, 1.0) * dark_gate;
        let lambda_nm = 525.0f32; // Luciferin-luciferase oxyluciferin peak
        let e_mag = (emit_power * 377.0).sqrt();

        CuticleEmfSpectralCell {
            e_field_xyz: [
                surface_normal[0] * e_mag,
                surface_normal[1] * e_mag,
                surface_normal[2] * e_mag,
            ],
            b_field_xyz: [0.0, 0.0, e_mag / 299_792_458.0],
            poynting_flux: emit_power,
            wavelength_nm: lambda_nm,
            refractive_n: 1.51, // hydrated fungal chitin-glucan matrix
            extinction_k: 0.012,
            coherence_phase: 0.0,
            stokes_quv: [0.05 * emit_power, 0.0, 0.0],
            spectral_sigma: 2.15, // Band 2 (Green 525 nm peak)
            linear_rgb_preview: [0.12 * emit_power, 0.95 * emit_power, 0.28 * emit_power],
        }
    }
}

/// Evaluates progressive enzymatic wood-decay softening (white-rot / brown-rot) on fallen logs
/// and standing snags: returns remaining dry mass fraction and effective Young's modulus $E(t)$ in GPa.
#[inline]
pub fn evaluate_fungal_wood_decay_modulus(
    initial_youngs_modulus_gpa: f32,
    guild: FungalEcologicalGuild,
    substrate_moisture: f32,
    temperature_c: f32,
    colonization_years: f32,
) -> (f32, f32) {
    // Q10 temperature response peaking near 26°C and moisture optimum in [0.35, 0.75]
    let temp_factor = (2.0f32.powf((temperature_c.clamp(-5.0, 35.0) - 20.0) / 10.0)).clamp(0.05, 2.5);
    let moist_factor = (1.0 - ((substrate_moisture.clamp(0.0, 1.0) - 0.55) / 0.45).powi(2)).clamp(0.0, 1.0);
    let base_rate = match guild {
        FungalEcologicalGuild::SaprotrophicBrownRot => 0.38, // Rapid depolymerization of cellulose strength
        FungalEcologicalGuild::SaprotrophicWhiteRot => 0.24, // Simultaneous lignin + cellulose decay
        _ => 0.02,
    };
    let k = base_rate * temp_factor * moist_factor;
    let t = colonization_years.max(0.0);
    let mass_remaining = (-0.55 * k * t).exp().clamp(0.08, 1.0);
    let modulus_gpa = (initial_youngs_modulus_gpa.max(0.1) * (-1.35 * k * t).exp()).max(0.02);
    (mass_remaining, modulus_gpa)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mycelium_foraging_creates_anastomosis_betti_loops_and_colonizes_roots() {
        let mut nodes = [HyphalNode {
            position_m: [0.0, 0.0, 0.0],
            radius_m: 0.0004,
            nutrient_potential: 1.0,
            attached_host_root_id: u16::MAX,
            is_active_tip: 0,
            generation: 0,
        }; 64];
        // Two nearby inoculum tips converging toward shared root-exudate attractors
        nodes[0] = HyphalNode {
            position_m: [-0.015, 0.0, 0.0],
            radius_m: 0.0005,
            nutrient_potential: 1.0,
            attached_host_root_id: 1,
            is_active_tip: 1,
            generation: 1,
        };
        nodes[1] = HyphalNode {
            position_m: [0.015, 0.0, 0.0],
            radius_m: 0.0005,
            nutrient_potential: 1.0,
            attached_host_root_id: 2,
            is_active_tip: 1,
            generation: 1,
        };
        let mut edges = [HyphalEdge {
            from_node: 0,
            to_node: 0,
            is_anastomosis_bridge: false,
            conductance: 0.0,
        }; 96];

        let mut attractors = [
            MycorrhizalAttractor {
                position_m: [0.0, 0.0, 0.03],
                concentration: 1.0,
                host_root_id: 10,
                colonized: false,
            },
            MycorrhizalAttractor {
                position_m: [0.0, 0.0, 0.06],
                concentration: 0.9,
                host_root_id: 11,
                colonized: false,
            },
        ];

        let receipt = step_mycelium_foraging_and_anastomosis_zero_heap(
            &mut nodes,
            2,
            &mut edges,
            0,
            &mut attractors,
            0.012,
            0.25,
            0.025,
            6,
        );

        assert!(receipt.node_count > 4);
        assert!(receipt.anastomosis_loop_count >= 1, "converging hyphae must fuse via anastomosis");
        assert!(receipt.first_betti_number_b1 >= 1, "Betti number b1 must be > 0 for cyclic CMN");
        assert!(receipt.colonized_host_roots >= 1, "must colonize host tree root tips");
    }

    #[test]
    fn turing_lamellae_bifurcate_and_ghost_fungus_emits_525nm_emf_cell() {
        let spec = FungalFruitingBodySpec::ghost_fungus_omphalotus();
        let (gills_inner, _) = spec.evaluate_turing_lamellae_at(0.013, 0.4);
        let (gills_outer, relief) = spec.evaluate_turing_lamellae_at(0.060, 0.0);
        assert!(
            gills_outer >= gills_inner * 2,
            "outer pileus margin must intercalate secondary/tertiary Turing gills"
        );
        assert!(relief > 0.0);

        let (_pos, apex_tangent, _) = spec.evaluate_gravitropic_stipe_frame(1.0);
        assert!(
            apex_tangent[1] > 0.85,
            "negative gravitropism must curve stipe apex toward vertical"
        );

        let emf = spec.synthesize_bioluminescent_emf_v4_cell([0.0, -1.0, 0.0], 0.0);
        assert!((emf.wavelength_nm - 525.0).abs() < 1e-3);
        assert!(emf.poynting_flux > 0.5);
    }
}

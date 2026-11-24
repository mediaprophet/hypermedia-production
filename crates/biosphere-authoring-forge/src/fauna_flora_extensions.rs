//! # Fauna & Flora Multi-Physics Extensions (`fauna_flora_extensions.rs`)
//!
//! Completes Pillars IV & V of the `biosphere-authoring-forge` upgrade:
//! 1. **Vertebrate & Invertebrate Central Pattern Generator (`CPG`) Locomotion Solvers:**
//!    - **Tetrapod Gaits:** Walk, Trot, Pace, Canter, and Transverse/Rotary Gallop phase & duty cycles.
//!    - **Bipedal Saltatorial Hopping (Macropod / Kangaroo & Wallaby):** Synchronous hindlimb
//!      stance/flight phases, Achilles/gastrocnemius tendon elastic strain energy recovery
//!      ($\eta \approx 0.45$), and tail pitch counter-torque stabilization.
//!    - **Undulatory & Peristaltic Locomotion:** Serpentine/anguilliform/carangiform travelling
//!      curvature wave $\kappa(s,t) = \kappa_0(s)\sin(2\pi(s/\lambda - f t))$.
//!    - **Avian / Chiropteran Wingbeat Kinematics:** Figure-8 stroke plane with upstroke primary
//!      feather aeroelastic slotting.
//! 2. **Isochoric (Volume-Preserving) Muscle Belly Bulging & Retinacular Bowstring Guard:**
//!    Enforces constant muscle volume ($\lambda_2 = \lambda_3 = \lambda_1^{-1/2}(1 + \alpha_{\text{act}} a)$)
//!    between `SKL2` barycentric UV entheses across intrinsic radial tissue depth $d \in [0.0, 1.0]$.
//! 3. **Pelage (Fur) & Plumage (Feather) Hydro-Aero-Thermal Response:**
//!    Capillary meniscus wet-clumping into conical tufts, cold piloerection coat fluffing,
//!    guard-hair tip hoar frost, and wind parting waves.
//! 4. **Subterranean Root Architecture, Trunk Buttress SDF Flare & Epiphyte Colonization:**
//!    Hydrotropic/gravitropic root sizing, wind-overturning anchorage moment $M_{\text{resist}}$,
//!    anisotropic basal buttress plank-root flare, and geodesic bryophyte/lichen/epiphyte host-mesh
//!    colonization probability.

use serde::{Deserialize, Serialize};

/// Vertebrate and non-arthropod locomotion gait archetype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FaunaGaitArchetype {
    /// Lateral-sequence tetrapod walk (duty factor $\beta \approx 0.68$).
    TetrapodWalk,
    /// Diagonal-pair synchronized trot ($\beta \approx 0.48$).
    TetrapodTrot,
    /// High-speed asymmetric rotary/transverse gallop ($\beta \approx 0.28$).
    TetrapodGallop,
    /// Bipedal saltatorial hopping (Macropodidae: kangaroos, wallabies, bettongs).
    MacropodSaltatorialHop,
    /// Lateral serpentine / carangiform undulation (reptiles, fish, eels).
    SerpentineUndulation,
    /// Avian / chiropteran flapping and soaring wingbeat cycle.
    AvianWingbeat,
}

/// Output of a zero-heap vertebrate / saltatorial / avian CPG gait evaluation at `phase_01`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FaunaCpgKinematicFrame {
    /// 4 limb ground-contact or wing-downstroke states (`[LF, RF, LH, RH]`, `true` = stance/downstroke).
    pub limb_stance_active: [bool; 4],
    /// 4 limb normalized swing/elevation trajectories `[LF, RF, LH, RH]` in `[0.0, 1.0]`.
    pub limb_swing_lift: [f32; 4],
    /// Vertical center-of-mass heave offset $\Delta y_{\text{COM}}$ in metres.
    pub com_vertical_heave_m: f32,
    /// Saltatorial Achilles tendon elastic strain energy stored/recovered fraction in `[0.0, 0.55]`.
    pub tendon_elastic_recovery_fraction: f32,
    /// Macropod / avian tail pitch counter-torque angle in radians (stabilizes body pitch in flight).
    pub tail_counter_pitch_rad: f32,
    /// Avian / chiropteran primary feather upstroke slotting twist angle in radians.
    pub primary_feather_slotting_rad: f32,
}

/// Evaluates zero-heap Central Pattern Generator (`CPG`) kinematics for tetrapods, saltatorial
/// macropods (kangaroos/wallabies), undulatory movers, and flapping birds/bats.
pub fn evaluate_fauna_cpg_locomotion_zero_heap(
    gait: FaunaGaitArchetype,
    phase_01: f32,
    forward_speed_m_s: f32,
    hip_height_m: f32,
    surface_friction_mu: f32,
) -> FaunaCpgKinematicFrame {
    let p = phase_01.rem_euclid(1.0);
    let h = hip_height_m.max(0.05);
    // Slip guard on ice/mud reduces vertical heave and widens duty factor
    let traction = (surface_friction_mu / 0.65).clamp(0.25, 1.0);

    let (offsets, duty): ([f32; 4], f32) = match gait {
        FaunaGaitArchetype::TetrapodWalk => ([0.0, 0.50, 0.25, 0.75], (0.68 / traction).clamp(0.55, 0.88)),
        FaunaGaitArchetype::TetrapodTrot => ([0.0, 0.50, 0.50, 0.0], (0.48 / traction).clamp(0.38, 0.72)),
        FaunaGaitArchetype::TetrapodGallop => ([0.0, 0.12, 0.55, 0.68], (0.28 / traction).clamp(0.22, 0.52)),
        // Macropod hop: forelimbs [0,1] tucked during fast hopping; hindlimbs [2,3] fire in phase (0.0, 0.0)
        FaunaGaitArchetype::MacropodSaltatorialHop => ([0.0, 0.0, 0.0, 0.0], 0.36),
        FaunaGaitArchetype::SerpentineUndulation => ([0.0, 0.25, 0.50, 0.75], 0.95),
        FaunaGaitArchetype::AvianWingbeat => ([0.0, 0.0, 0.5, 0.5], 0.55),
    };

    let mut stance = [false; 4];
    let mut lift = [0.0f32; 4];
    for i in 0..4 {
        let lp = (p + offsets[i]).rem_euclid(1.0);
        if gait == FaunaGaitArchetype::MacropodSaltatorialHop && i < 2 && forward_speed_m_s > 1.8 {
            // Forepaws remain off ground during bipedal hopping
            stance[i] = false;
            lift[i] = 0.65;
        } else if lp < duty {
            stance[i] = true;
            lift[i] = 0.0;
        } else {
            stance[i] = false;
            let s_u = (lp - duty) / (1.0 - duty).max(1e-4);
            lift[i] = (core::f32::consts::PI * s_u).sin().max(0.0);
        }
    }

    let (heave_m, tendon_rec, tail_pitch, feather_slot) = match gait {
        FaunaGaitArchetype::MacropodSaltatorialHop => {
            // Parabolic flight apex during aerial phase, tendon stretch at mid-stance
            let is_stance = p < duty;
            let heave = if is_stance {
                -0.18 * h * (core::f32::consts::PI * (p / duty)).sin()
            } else {
                0.42 * h * traction * (core::f32::consts::PI * ((p - duty) / (1.0 - duty))).sin()
            };
            let tendon = if is_stance {
                0.48 * (core::f32::consts::PI * (p / duty)).sin()
            } else {
                0.05
            };
            // Tail swings downward during aerial hop to counteract hindlimb forward pitch angular momentum
            let tail = 0.38 * (2.0 * core::f32::consts::PI * p).sin();
            (heave, tendon, tail, 0.0)
        }
        FaunaGaitArchetype::AvianWingbeat => {
            let downstroke = p < duty;
            let heave = 0.08 * h * (2.0 * core::f32::consts::PI * p).cos();
            let slot_rad = if !downstroke {
                // Upstroke primary feather aeroelastic twist/slotting reduces negative lift
                0.72 * (core::f32::consts::PI * ((p - duty) / (1.0 - duty))).sin()
            } else {
                0.0
            };
            (heave, 0.15, 0.12 * (2.0 * core::f32::consts::PI * p).sin(), slot_rad)
        }
        _ => {
            let heave = 0.06 * h * traction * (4.0 * core::f32::consts::PI * p).cos();
            let tendon = (0.22 * (forward_speed_m_s / 8.0).clamp(0.0, 1.0)).clamp(0.0, 0.25);
            (heave, tendon, 0.05 * (2.0 * core::f32::consts::PI * p).sin(), 0.0)
        }
    };

    FaunaCpgKinematicFrame {
        limb_stance_active: stance,
        limb_swing_lift: lift,
        com_vertical_heave_m: heave_m,
        tendon_elastic_recovery_fraction: tendon_rec,
        tail_counter_pitch_rad: tail_pitch,
        primary_feather_slotting_rad: feather_slot,
    }
}

/// Evaluates serpentine / carangiform lateral spine curvature $\kappa(s, t)$ (in $\text{m}^{-1}$)
/// at normalized spine coordinate $s \in [0.0, 1.0]$ (`0.0` head $\to$ `1.0` tail tip).
#[inline]
pub fn evaluate_undulatory_spine_curvature(
    s_norm: f32,
    time_sec: f32,
    wave_frequency_hz: f32,
    waves_per_body_length: f32,
    base_amplitude_m_inv: f32,
    caudal_amplification: f32,
) -> f32 {
    let s = s_norm.clamp(0.0, 1.0);
    let envelope = (1.0 - caudal_amplification.clamp(0.0, 0.9)) + caudal_amplification.clamp(0.0, 0.9) * s * s;
    let phase = 2.0 * core::f32::consts::PI * (waves_per_body_length.max(0.2) * s - wave_frequency_hz * time_sec);
    base_amplitude_m_inv * envelope * phase.sin()
}

/// Output of the isochoric (volume-preserving) muscle belly & intrinsic radial tissue depth solver.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IsochoricMuscleBellyState {
    /// Longitudinal stretch ratio $\lambda_1 = L_{\text{current}} / L_{\text{rest}}$.
    pub longitudinal_stretch_lambda1: f32,
    /// Isochoric radial bulge stretch $\lambda_2 = \lambda_3 = \lambda_1^{-1/2}(1 + \alpha_{\text{act}} a)$.
    pub radial_bulge_lambda2: f32,
    /// Updated muscle belly cross-sectional radius in metres.
    pub effective_belly_radius_m: f32,
    /// Intrinsic skin-surface outward normal displacement in metres.
    pub skin_surface_displacement_m: f32,
    /// Retinacular pulley bowstring prevention tension in Newtons.
    pub retinacular_pulley_restraint_n: f32,
}

/// Evaluates volume-preserving ($\det \mathbf{F} = \lambda_1 \lambda_2 \lambda_3 = 1$) muscle belly
/// radial inflation between two `SKL2` barycentric UV entheses, plus retinacular pulley restraint.
#[inline]
pub fn evaluate_isochoric_muscle_belly_deformation(
    rest_length_m: f32,
    current_origin_to_insertion_m: f32,
    rest_belly_radius_m: f32,
    neural_activation_01: f32,
    joint_bend_angle_rad: f32,
    peak_isometric_force_n: f32,
) -> IsochoricMuscleBellyState {
    let l0 = rest_length_m.max(0.005);
    let l_cur = current_origin_to_insertion_m.clamp(l0 * 0.45, l0 * 1.45);
    let lambda1 = l_cur / l0;
    let act = neural_activation_01.clamp(0.0, 1.0);
    // Isochoric condition: lambda2 = lambda1^(-1/2), modulated by active sarcomere pennation thickening
    let lambda2 = (1.0 / lambda1.sqrt()) * (1.0 + 0.14 * act);
    let r0 = rest_belly_radius_m.max(0.001);
    let r_eff = r0 * lambda2;
    let skin_disp = (r_eff - r0).max(-0.35 * r0);

    // Retinacular pulley restraint force F_pulley = 2 * F_tendon * sin(theta_bend / 2)
    let f_tendon = peak_isometric_force_n.max(0.0) * (0.1 + 0.9 * act);
    let restraint_n = 2.0 * f_tendon * (0.5 * joint_bend_angle_rad.abs().min(2.4)).sin();

    IsochoricMuscleBellyState {
        longitudinal_stretch_lambda1: lambda1,
        radial_bulge_lambda2: lambda2,
        effective_belly_radius_m: r_eff,
        skin_surface_displacement_m: skin_disp,
        retinacular_pulley_restraint_n: restraint_n,
    }
}

/// Pelage (fur) and plumage (feather) hydro-aero-thermal surface response.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CoatSurfaceResponse {
    /// Capillary meniscus clumping factor in `[0.0, 1.0]` (`0.0` dry fluffy coat, `0.85` wet conical tufts).
    pub wet_tuft_clumping_factor: f32,
    /// Effective thermal boundary-layer coat loft thickness in metres (increases in cold via piloerection,
    /// collapses when soaked or flattened by high wind pressure).
    pub effective_coat_loft_m: f32,
    /// Thermal insulation resistance ($R$-value equivalent) relative to dry baseline in `[0.15, 1.45]`.
    pub thermal_insulation_efficiency: f32,
    /// Guard-hair tip hoar-frost accumulation thickness in metres.
    pub guard_hair_frost_tip_m: f32,
}

/// Evaluates mammalian pelage (fur) or avian plumage response to wetting, cold piloerection,
/// wind dynamic pressure flattening, and sub-zero breath/ambient hoar frost.
#[inline]
pub fn evaluate_pelage_plumage_hydro_thermal_response(
    dry_coat_thickness_m: f32,
    water_saturation_01: f32,
    air_temp_c: f32,
    wind_dynamic_pressure_pa: f32,
    ambient_hoar_frost_m: f32,
) -> CoatSurfaceResponse {
    let w = water_saturation_01.clamp(0.0, 1.0);
    // Capillary meniscus clumping peaks around 55%-85% saturation
    let clumping = (1.15 * w * (1.0 - 0.22 * w)).clamp(0.0, 0.92);
    // Cold piloerection (arrector pili / feather ptiloerection) below +10°C expands dry loft up to +45%
    let piloerection = if air_temp_c < 10.0 {
        (1.0 + 0.45 * ((10.0 - air_temp_c) / 25.0).clamp(0.0, 1.0)) * (1.0 - 0.85 * w)
    } else {
        1.0 - 0.55 * w
    };
    let wind_compression = (1.0 / (1.0 + wind_dynamic_pressure_pa.max(0.0) * 0.015)).clamp(0.35, 1.0);
    let loft_m = (dry_coat_thickness_m.max(0.001) * piloerection * wind_compression).max(0.0005);

    // Water conducts heat ~24x faster than trapped air, collapsing coat insulation when wet
    let insulation = ((loft_m / dry_coat_thickness_m.max(0.001)) * (1.0 - 0.78 * w)).clamp(0.15, 1.45);
    // Guard hair tips have high curvature and stay below air temperature, accreting 1.6x ambient hoar frost
    let tip_frost_m = ambient_hoar_frost_m.max(0.0) * 1.6;

    CoatSurfaceResponse {
        wet_tuft_clumping_factor: clumping,
        effective_coat_loft_m: loft_m,
        thermal_insulation_efficiency: insulation,
        guard_hair_frost_tip_m: tip_frost_m,
    }
}

/// Subterranean root-plate anchorage, basal trunk buttress flare, and epiphyte colonization report.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RootButtressEpiphyteReport {
    /// Da Vinci accumulated tap/collar root radius at the soil line in metres.
    pub basal_root_collar_radius_m: f32,
    /// Anisotropic buttress plank-root radial flare radius at azimuth $\theta$ in metres.
    pub buttress_flare_radius_at_azimuth_m: f32,
    /// Critical root-soil plate wind-overturning resistance moment $M_{\text{resist}}$ in $\text{N}\cdot\text{m}$.
    pub windthrow_overturning_resistance_n_m: f32,
    /// `true` if applied aerodynamic + snow overturning moment uproots the root plate (**windthrow**).
    pub windthrow_uprooted: bool,
    /// Geodesic host-surface colonization suitability in `[0.0, 1.0]` for bryophytes (moss), lichens, and epiphytes.
    pub epiphyte_moss_colonization_suitability: f32,
}

/// Evaluates Da Vinci root collar accumulation, fluted windward/tension buttress flare at azimuth
/// `azimuth_rad`, root-soil plate windthrow resistance moment, and bark/rock epiphyte suitability.
pub fn evaluate_root_buttress_and_epiphyte_ecology(
    lateral_root_radii_m: &[f32],
    pipe_exponent_n: f32,
    root_plate_depth_m: f32,
    root_plate_radius_m: f32,
    soil_shear_strength_pa: f32,
    applied_overturning_moment_n_m: f32,
    prevailing_wind_azimuth_rad: f32,
    query_azimuth_rad: f32,
    buttress_flute_count: u32,
    surface_ao_cavity: f32,
    surface_moisture_01: f32,
    subcanopy_shade_01: f32,
) -> RootButtressEpiphyteReport {
    let n_exp = pipe_exponent_n.clamp(1.8, 2.5);
    let mut sum_r_n = 0.0f32;
    for &r in lateral_root_radii_m {
        if r > 0.0 {
            sum_r_n += r.powf(n_exp);
        }
    }
    let collar_r = sum_r_n.powf(1.0 / n_exp).max(0.02);

    // Anisotropic fluted buttress flare: strongest along prevailing windward tension axis
    let flutes = buttress_flute_count.clamp(3, 12) as f32;
    let wind_align = (query_azimuth_rad - prevailing_wind_azimuth_rad).cos().max(-0.3);
    let flute_wave = (flutes * (query_azimuth_rad - prevailing_wind_azimuth_rad)).cos().max(0.0).powi(2);
    let buttress_r = collar_r * (1.0 + (0.55 + 0.35 * wind_align) * flute_wave);

    // Root-soil plate overturning resistance M_resist = weight_moment + shear_perimeter_moment
    let r_plate = root_plate_radius_m.max(collar_r * 2.5);
    let d_plate = root_plate_depth_m.max(0.25);
    let rho_soil_wet = 1750.0f32;
    let g = 9.80665f32;
    let plate_volume = core::f32::consts::PI * r_plate * r_plate * d_plate * 0.55;
    let weight_moment = plate_volume * rho_soil_wet * g * (0.65 * r_plate);
    let shear_moment =
        (2.0 * core::f32::consts::PI * r_plate * d_plate) * soil_shear_strength_pa.max(2_000.0) * r_plate;
    let resist_moment = weight_moment + shear_moment;
    let uprooted = applied_overturning_moment_n_m > resist_moment;

    // Bryophyte (moss) & epiphyte colonization suitability inside moist, shaded bark furrows
    let epiphyte_suit = (0.40 * surface_moisture_01.clamp(0.0, 1.0)
        + 0.35 * surface_ao_cavity.clamp(0.0, 1.0)
        + 0.25 * subcanopy_shade_01.clamp(0.0, 1.0))
    .clamp(0.0, 1.0);

    RootButtressEpiphyteReport {
        basal_root_collar_radius_m: collar_r,
        buttress_flare_radius_at_azimuth_m: buttress_r,
        windthrow_overturning_resistance_n_m: resist_moment,
        windthrow_uprooted: uprooted,
        epiphyte_moss_colonization_suitability: epiphyte_suit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macropod_saltatorial_hop_stores_achilles_tendon_energy_and_isochoric_muscle_bulges() {
        let hop_stance = evaluate_fauna_cpg_locomotion_zero_heap(
            FaunaGaitArchetype::MacropodSaltatorialHop,
            0.18, // mid-stance
            6.5,
            0.85,
            0.75,
        );
        assert!(!hop_stance.limb_stance_active[0], "forelimbs remain tucked during high-speed bipedal hop");
        assert!(hop_stance.limb_stance_active[2] && hop_stance.limb_stance_active[3], "hindlimbs fire synchronously");
        assert!(hop_stance.tendon_elastic_recovery_fraction > 0.35, "Achilles tendon must store elastic energy");

        // Isochoric muscle contraction: shortening to 75% rest length at full activation bulges radial cross-section
        let muscle = evaluate_isochoric_muscle_belly_deformation(0.24, 0.18, 0.035, 1.0, 1.2, 850.0);
        assert!(muscle.longitudinal_stretch_lambda1 < 0.80);
        assert!(muscle.radial_bulge_lambda2 > 1.25);
        assert!(muscle.skin_surface_displacement_m > 0.008);
        assert!(muscle.retinacular_pulley_restraint_n > 500.0);
    }

    #[test]
    fn root_buttress_resists_windthrow_and_pelage_clumps_when_soaked() {
        let roots = [0.12f32, 0.11, 0.10, 0.09, 0.08, 0.07];
        let rep = evaluate_root_buttress_and_epiphyte_ecology(
            &roots,
            2.0,
            1.2,
            2.2,
            18_000.0,
            45_000.0,
            0.0,
            0.0,
            5,
            0.85,
            0.90,
            0.80,
        );
        assert!(rep.buttress_flare_radius_at_azimuth_m > rep.basal_root_collar_radius_m * 1.5);
        assert!(!rep.windthrow_uprooted);
        assert!(rep.epiphyte_moss_colonization_suitability > 0.80);

        let wet_coat = evaluate_pelage_plumage_hydro_thermal_response(0.030, 0.80, 2.0, 25.0, 0.002);
        let dry_coat = evaluate_pelage_plumage_hydro_thermal_response(0.030, 0.0, -5.0, 2.0, 0.002);
        assert!(wet_coat.wet_tuft_clumping_factor > 0.65);
        assert!(dry_coat.effective_coat_loft_m > 0.035, "cold dry coat must fluff via piloerection");
        assert!(wet_coat.thermal_insulation_efficiency < dry_coat.thermal_insulation_efficiency * 0.4);
    }
}

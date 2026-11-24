//! # 3D Wind Pressure Tensor & Multi-Asset Fluid-Structure Interaction (`wind_pressure_fsi.rs`)
//!
//! Replaces uniform 2D wind vectors with a spatially varying 3D aerodynamic pressure, shear,
//! and wake field coupled across `M64` (`GRM3`) terrain and `.10d` flora/fauna/water assets:
//! 1. **Orographic Ridge Acceleration, Canyon Venturi & Lee Separation Eddies:**
//!    Jackson–Hunt fractional ridge speed-up $\Delta S \approx 2 H / L$ and leeward adverse
//!    pressure-gradient recirculation vortices behind steep escarpments.
//! 2. **Dynamic Pressure Tensor $q = \frac{1}{2}\rho_{\text{air}} |\mathbf{U}_{\text{rel}}|^2$ & Surface $C_p$:**
//!    Computes windward stagnation pressure ($C_p > 0$), lateral acceleration suction ($C_p < 0$),
//!    and leeward base suction across an asset's surface normal.
//! 3. **3D Canopy Wake Attenuation & Downwind Shelter Cones:**
//!    Upwind trees extract momentum proportional to optical porosity $\beta$ and Leaf Area Density,
//!    casting a 3D velocity-deficit wake and elevated turbulence cone onto downwind assets.
//! 4. **4-Level Hierarchical Tree Aeroelasticity, Vogel Streamlining & Abscission:**
//!    Evaluates (`L0` Trunk cantilever + `L1` Primary bough harmonic sway + `L2` Secondary branch
//!    Vogel crown streamlining $F_{\text{drag}} \propto U^{2+\mathcal{V}}$ + `L3` Petiole Strouhal
//!    vortex-shedding flutter $f = \text{St}\cdot U / L$) and checks branch fracture / leaf abscission
//!    under combined aerodynamic + ice/snow bending moments.
//! 5. **Coherent Kelvin–Helmholtz `Honami` Wave Trains in Grass & Crops:**
//!    Evaluates travelling mixing-layer roller vortices across herbaceous meadows.
//! 6. **Fauna Postural Wind-Lean Torque & Arthropod Anemotactic Crouch Reflex:**
//!    Computes windward center-of-mass lean angle $\theta_{\text{lean}} = \arctan(F_{\text{drag}}/(m g))$
//!    and automatic arthropod body-clearance crouch & stance widening under high dynamic pressure $q$.

use serde::{Deserialize, Serialize};

/// An upwind porous obstacle (tree crown, shrub, boulder, windbreak) that casts a 3D momentum-deficit wake.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindWakeObstacle {
    /// Center of the obstacle crown/body `[x, y, z]` in metres (`y` is elevation).
    pub center_m: [f32; 3],
    /// Characteristic crown radius in metres.
    pub crown_radius_m: f32,
    /// Total obstacle height $H$ in metres.
    pub height_m: f32,
    /// Optical/aerodynamic porosity $\beta \in [0.0, 0.95]$ (`0.0` solid boulder, `0.35` dense tree, `0.70` open eucalypt).
    pub optical_porosity: f32,
}

/// Local 3D aerodynamic field and dynamic pressure state evaluated at a world point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LocalWindPressureField {
    /// Local 3D wind velocity vector `[ux, uy, uz]` in m/s (`uy` is vertical).
    pub velocity_m_s: [f32; 3],
    /// Wind speed magnitude $|\mathbf{U}|$ in m/s.
    pub speed_m_s: f32,
    /// Moist air density $\rho_{\text{air}}$ in $\text{kg/m}^3$.
    pub air_density_kg_m3: f32,
    /// Dynamic stagnation pressure $q = \frac{1}{2} \rho_{\text{air}} |\mathbf{U}|^2$ in Pascals ($\text{N/m}^2$).
    pub dynamic_pressure_pa: f32,
    /// Local turbulence intensity $I_u = \sigma_u / |\mathbf{U}| \in [0.05, 0.85]$.
    pub turbulence_intensity: f32,
    /// Jackson–Hunt orographic speed-up multiplier ($> 1.0$ on ridge crests, $< 1.0$ or negative in lee eddies).
    pub orographic_speedup_factor: f32,
    /// `true` if the locus sits inside a leeward flow-separation recirculation eddy.
    pub in_lee_separation_eddy: bool,
    /// Fraction of free-stream momentum remaining after upwind canopy wake attenuation in `[0.1, 1.0]`.
    pub canopy_wake_transmission: f32,
}

/// Evaluates the 3D wind velocity, dynamic pressure $q = \frac{1}{2}\rho U^2$, orographic ridge
/// speed-up, lee separation eddies, and cumulative upwind canopy wake attenuation at `query_pos_m`.
pub fn evaluate_3d_wind_pressure_field_zero_heap(
    free_stream_wind_10m: [f32; 2],
    air_density_kg_m3: f32,
    query_pos_m: [f32; 3],
    terrain_elevation_m: f32,
    terrain_slope_gradient: [f32; 2],
    ridge_crest_prominence_m: f32,
    ridge_half_length_m: f32,
    upwind_obstacles: &[WindWakeObstacle],
) -> LocalWindPressureField {
    let rho = air_density_kg_m3.clamp(0.4, 1.6);
    let z_agl = (query_pos_m[1] - terrain_elevation_m).max(0.15);
    let u10_x = free_stream_wind_10m[0];
    let u10_z = free_stream_wind_10m[1];
    let u10_mag = (u10_x * u10_x + u10_z * u10_z).sqrt();
    if u10_mag <= 1e-5 {
        return LocalWindPressureField {
            velocity_m_s: [0.0; 3],
            speed_m_s: 0.0,
            air_density_kg_m3: rho,
            dynamic_pressure_pa: 0.0,
            turbulence_intensity: 0.05,
            orographic_speedup_factor: 1.0,
            in_lee_separation_eddy: false,
            canopy_wake_transmission: 1.0,
        };
    }
    let wind_dir_2d = [u10_x / u10_mag, u10_z / u10_mag];

    // 1. Prandtl-von Karman logarithmic height scaling relative to 10m reference
    let z0 = 0.08f32;
    let log_scale = ((z_agl + z0) / z0).ln() / ((10.0 + z0) / z0).ln();

    // 2. Orographic Jackson-Hunt ridge speed-up & leeward separation check
    // Along-wind slope s_parallel = grad(h) . wind_dir_2d
    let s_parallel =
        terrain_slope_gradient[0] * wind_dir_2d[0] + terrain_slope_gradient[1] * wind_dir_2d[1];
    let l_ridge = ridge_half_length_m.max(10.0);
    let delta_s_crest = (2.0 * ridge_crest_prominence_m / l_ridge).clamp(-0.4, 1.35)
        * (-z_agl / (0.5 * l_ridge)).exp();

    // Leeward separation occurs when downwind slope drops steeper than ~18 deg (tan 18° ≈ 0.325)
    let in_lee_eddy = s_parallel < -0.325 && z_agl < ridge_crest_prominence_m.max(8.0) * 0.6;
    let orographic_factor = if in_lee_eddy {
        0.32 // Sheltered recirculation pocket
    } else {
        (1.0 + delta_s_crest + 0.35 * s_parallel.clamp(-0.5, 0.8)).clamp(0.35, 2.35)
    };

    // 3. Cumulative 3D Canopy / Obstacle Wake Attenuation
    let mut wake_transmission = 1.0f32;
    let mut wake_turbulence_boost = 0.0f32;
    for obs in upwind_obstacles {
        let dx = query_pos_m[0] - obs.center_m[0];
        let dz = query_pos_m[2] - obs.center_m[2];
        let downwind_dist = dx * wind_dir_2d[0] + dz * wind_dir_2d[1];
        if downwind_dist <= 0.1 {
            continue;
        }
        let cross_x = dx - downwind_dist * wind_dir_2d[0];
        let cross_z = dz - downwind_dist * wind_dir_2d[1];
        let dy = query_pos_m[1] - obs.center_m[1];
        let r_perp_sq = cross_x * cross_x + cross_z * cross_z + dy * dy * 0.65;
        let h = obs.height_m.max(1.0);
        let r0 = obs.crown_radius_m.max(0.5);
        // Wake expands linearly with downwind fetch
        let sigma_w = r0 + 0.12 * downwind_dist;
        if r_perp_sq < (sigma_w * 2.8).powi(2) {
            let solidity = (1.0 - obs.optical_porosity.clamp(0.0, 0.95)).clamp(0.05, 1.0);
            let axial_decay = (h / (h + 0.35 * downwind_dist)).clamp(0.0, 1.0);
            let radial_gauss = (-r_perp_sq / (2.0 * sigma_w * sigma_w)).exp();
            let deficit = (0.78 * solidity * axial_decay * radial_gauss).clamp(0.0, 0.82);
            wake_transmission *= 1.0 - deficit;
            wake_turbulence_boost += 0.28 * solidity * axial_decay * radial_gauss;
        }
    }
    wake_transmission = wake_transmission.clamp(0.12, 1.0);

    // Combine velocity components (including terrain-following vertical velocity uy)
    let speed_horiz = u10_mag * log_scale * orographic_factor * wake_transmission;
    let dir_sign = if in_lee_eddy { -0.45 } else { 1.0 }; // Reverse eddy flow near ground in lee pocket
    let ux = wind_dir_2d[0] * speed_horiz * dir_sign;
    let uz = wind_dir_2d[1] * speed_horiz * dir_sign;
    let uy = speed_horiz * s_parallel.clamp(-0.6, 0.6) * (-z_agl / l_ridge).exp();
    let total_speed = (ux * ux + uy * uy + uz * uz).sqrt();
    let q_pa = 0.5 * rho * total_speed * total_speed;
    let turb = (0.11 + wake_turbulence_boost + if in_lee_eddy { 0.32 } else { 0.0 }).clamp(0.05, 0.85);

    LocalWindPressureField {
        velocity_m_s: [ux, uy, uz],
        speed_m_s: total_speed,
        air_density_kg_m3: rho,
        dynamic_pressure_pa: q_pa,
        turbulence_intensity: turb,
        orographic_speedup_factor: orographic_factor,
        in_lee_separation_eddy: in_lee_eddy,
        canopy_wake_transmission: wake_transmission,
    }
}

/// Evaluates surface pressure coefficient $C_p \in [-1.2, +1.0]$ and normal pressure force per unit area
/// (Pa) on an asset surface element with normal `surface_normal` inside `wind_field`.
#[inline]
pub fn evaluate_surface_pressure_coefficient_cp(
    wind_field: &LocalWindPressureField,
    surface_normal: [f32; 3],
) -> (f32, f32) {
    if wind_field.speed_m_s <= 1e-4 {
        return (0.0, 0.0);
    }
    let u = [
        wind_field.velocity_m_s[0] / wind_field.speed_m_s,
        wind_field.velocity_m_s[1] / wind_field.speed_m_s,
        wind_field.velocity_m_s[2] / wind_field.speed_m_s,
    ];
    // cos_incidence = +1 on windward stagnation face (normal pointing into oncoming wind: -u)
    let cos_inc = -(surface_normal[0] * u[0] + surface_normal[1] * u[1] + surface_normal[2] * u[2])
        .clamp(-1.0, 1.0);
    let cp = if cos_inc >= 0.0 {
        // Windward stagnation zone (+1.0 at dead center, dropping to -0.8 at lateral shoulders)
        1.8 * cos_inc * cos_inc - 0.8
    } else {
        // Leeward separated wake base suction
        -0.45
    };
    (cp, cp * wind_field.dynamic_pressure_pa)
}

/// Output of the 4-level hierarchical tree aeroelasticity, Vogel crown streamlining, and abscission solver.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HierarchicalFloraWindResponse {
    /// Level 0/1 trunk & primary bough modal deflection angle in radians.
    pub trunk_bough_bend_rad: f32,
    /// Level 2 secondary crown streamlining drag-area reduction ratio $A_{\text{stream}} / A_0 \in [0.25, 1.0]$.
    pub vogel_streamlined_area_ratio: f32,
    /// Aerodynamic drag force in Newtons after Vogel reconfiguration ($F_{\text{drag}} \propto U^{2+\mathcal{V}}$).
    pub reconfigured_drag_force_n: f32,
    /// Level 3 petiole/leaf Strouhal vortex-shedding flutter frequency in Hz ($f = \text{St}\cdot U / L$).
    pub leaf_flutter_frequency_hz: f32,
    /// Peak basal bending moment in $\text{N}\cdot\text{m}$ combining wind drag and snow/ice overhang lever arm.
    pub peak_bending_moment_n_m: f32,
    /// `true` if leaf petioles exceed abscission shear threshold and shed leaves into the wind.
    pub leaf_abscission_triggered: bool,
    /// `true` if the branch/trunk exceeds green-wood modulus of rupture ($\text{MOR}$) and fractures.
    pub structural_branch_fracture: bool,
}

/// Evaluates 4-level hierarchical tree aeroelasticity (trunk/bough bend, Vogel crown streamlining
/// with exponent $\mathcal{V} \in [-0.9, -0.3]$, leaf Strouhal flutter, and snow/ice-coupled fracture).
pub fn evaluate_hierarchical_flora_wind_response(
    wind_field: &LocalWindPressureField,
    unstreamlined_frontal_area_m2: f32,
    vogel_exponent: f32,
    branch_length_m: f32,
    basal_radius_m: f32,
    wood_youngs_modulus_pa: f32,
    wood_rupture_modulus_pa: f32,
    leaf_chord_length_m: f32,
    cryo_line_load_n_per_m: f32,
    time_sec: f32,
) -> HierarchicalFloraWindResponse {
    let u = wind_field.speed_m_s;
    let v_exp = vogel_exponent.clamp(-0.95, 0.0);
    // Reference streamlining velocity U_ref = 3.5 m/s
    let u_ref = 3.5f32;
    let area_ratio = if u > u_ref {
        (u / u_ref).powf(v_exp).clamp(0.22, 1.0)
    } else {
        1.0
    };
    let cd = 0.85f32;
    let gust_mult = 1.0 + 1.45 * wind_field.turbulence_intensity * (time_sec * 2.4).sin().abs();
    let f_drag = wind_field.dynamic_pressure_pa
        * cd
        * unstreamlined_frontal_area_m2.max(0.01)
        * area_ratio
        * gust_mult;

    // Second moment of area I = pi * r^4 / 4 for cylindrical stem/branch
    let r = basal_radius_m.max(0.002);
    let len = branch_length_m.max(0.05);
    let i_moment = core::f32::consts::PI * r.powi(4) * 0.25;
    let ei = (wood_youngs_modulus_pa.max(1.0e7)) * i_moment;

    // Combined bending moment from wind drag (at 0.65 L) + eccentric snow/ice load
    let m_wind = f_drag * (0.65 * len);
    let m_cryo = cryo_line_load_n_per_m.max(0.0) * len * (0.35 * len);
    let total_moment = m_wind + m_cryo;

    let bend_rad = ((total_moment * len) / (2.0 * ei)).clamp(0.0, 1.35);
    // Peak outer-fiber stress sigma_max = M * r / I
    let peak_stress_pa = (total_moment * r) / i_moment;
    let fracture = peak_stress_pa > wood_rupture_modulus_pa.max(5.0e6);

    // Leaf Strouhal vortex shedding f = St * U / L_leaf (St ≈ 0.18 for flexible lamina)
    let flutter_hz = (0.18 * u) / leaf_chord_length_m.max(0.01);
    let abscission = wind_field.dynamic_pressure_pa * gust_mult > 110.0
        || (cryo_line_load_n_per_m > 12.0 && u > 9.0);

    HierarchicalFloraWindResponse {
        trunk_bough_bend_rad: bend_rad,
        vogel_streamlined_area_ratio: area_ratio,
        reconfigured_drag_force_n: f_drag,
        leaf_flutter_frequency_hz: flutter_hz,
        peak_bending_moment_n_m: total_moment,
        leaf_abscission_triggered: abscission,
        structural_branch_fracture: fracture,
    }
}

/// Evaluates coherent Kelvin–Helmholtz `honami` travelling wave deflection `[dx, dz]` and stalk
/// tilt angle (radians) at world coordinates `(x_m, z_m)` and `time_sec` for grass/sedge/crop canopies.
#[inline]
pub fn evaluate_honami_meadow_wave_at(
    wind_field: &LocalWindPressureField,
    canopy_height_m: f32,
    stalk_stiffness: f32,
    x_m: f32,
    z_m: f32,
    time_sec: f32,
) -> ([f32; 2], f32) {
    let u = wind_field.speed_m_s;
    if u <= 0.05 {
        return ([0.0, 0.0], 0.0);
    }
    let dir = [
        wind_field.velocity_m_s[0] / u,
        wind_field.velocity_m_s[2] / u,
    ];
    let h = canopy_height_m.clamp(0.1, 3.0);
    // Kelvin-Helmholtz mixing-layer instability wavelength Lambda ≈ 4.8 * h_c, phase speed c_p ≈ 0.65 * U
    let wavelength_m = (4.8 * h).max(0.8);
    let k = (2.0 * core::f32::consts::PI) / wavelength_m;
    let phase_speed = 0.65 * u;
    let along_wind = x_m * dir[0] + z_m * dir[1];
    let cross_wind = -x_m * dir[1] + z_m * dir[0];

    let kh_envelope =
        0.5 + 0.5 * (k * (along_wind - phase_speed * time_sec) + 0.35 * (cross_wind * k * 0.5).sin()).sin();
    let mean_pressure_ratio =
        (wind_field.dynamic_pressure_pa / (12.0 + 80.0 * stalk_stiffness.clamp(0.05, 1.0))).clamp(0.0, 1.25);
    let tilt_rad = (mean_pressure_ratio * (0.45 + 0.55 * kh_envelope)).clamp(0.0, 1.18);
    let horiz_disp = h * tilt_rad.sin();
    ([dir[0] * horiz_disp, dir[1] * horiz_disp], tilt_rad)
}

/// Evaluates aerodynamic postural compensation for terrestrial fauna & arthropods in high winds:
/// returns `(windward_lean_vector_m, body_clearance_crouch_multiplier, stance_widen_multiplier)`.
#[inline]
pub fn evaluate_fauna_wind_posture_compensation(
    wind_field: &LocalWindPressureField,
    body_mass_kg: f32,
    drag_area_m2: f32,
    center_of_mass_height_m: f32,
    is_arthropod: bool,
) -> ([f32; 3], f32, f32) {
    let f_drag = wind_field.dynamic_pressure_pa * 0.9 * drag_area_m2.max(1.0e-5);
    let weight_n = (body_mass_kg.max(1.0e-5)) * 9.80665;
    let lean_ratio = (f_drag / weight_n).clamp(0.0, 0.75);
    let u = wind_field.speed_m_s.max(1e-4);
    let into_wind = [
        -wind_field.velocity_m_s[0] / u,
        0.0,
        -wind_field.velocity_m_s[2] / u,
    ];
    let com_shift = center_of_mass_height_m.max(0.005) * lean_ratio;
    let lean_vec = [into_wind[0] * com_shift, 0.0, into_wind[2] * com_shift];

    // Arthropods exhibit a sharp anemotactic crouch reflex when dynamic pressure exceeds ~8 Pa (~3.6 m/s)
    let (crouch_mult, widen_mult) = if is_arthropod {
        let crouch = (1.0 - (wind_field.dynamic_pressure_pa / 65.0).clamp(0.0, 0.65)).clamp(0.35, 1.0);
        let widen = (1.0 + (wind_field.dynamic_pressure_pa / 90.0).clamp(0.0, 0.35)).clamp(1.0, 1.35);
        (crouch, widen)
    } else {
        let crouch = (1.0 - 0.25 * lean_ratio).clamp(0.75, 1.0);
        let widen = (1.0 + 0.25 * lean_ratio).clamp(1.0, 1.25);
        (crouch, widen)
    };
    (lean_vec, crouch_mult, widen_mult)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ridge_crest_accelerates_wind_while_upwind_canopy_and_lee_slope_attenuate() {
        let obstacles = [WindWakeObstacle {
            center_m: [0.0, 8.0, 0.0],
            crown_radius_m: 5.0,
            height_m: 16.0,
            optical_porosity: 0.30,
        }];

        // Exposed ridge crest with no upwind obstacle wake
        let crest_wind = evaluate_3d_wind_pressure_field_zero_heap(
            [12.0, 0.0],
            1.225,
            [-20.0, 58.0, 0.0],
            50.0,
            [0.28, 0.0],
            40.0,
            60.0,
            &obstacles,
        );
        // Sheltered point 15m downwind of the dense tree crown
        let sheltered_wind = evaluate_3d_wind_pressure_field_zero_heap(
            [12.0, 0.0],
            1.225,
            [15.0, 8.0, 0.0],
            0.0,
            [0.0, 0.0],
            0.0,
            60.0,
            &obstacles,
        );
        assert!(crest_wind.orographic_speedup_factor > 1.35);
        assert!(sheltered_wind.canopy_wake_transmission < 0.65);
        assert!(crest_wind.dynamic_pressure_pa > sheltered_wind.dynamic_pressure_pa * 3.0);

        // Vogel crown streamlining reduces effective drag area in strong winds
        let flora_resp = evaluate_hierarchical_flora_wind_response(
            &crest_wind,
            18.0,
            -0.68,
            6.5,
            0.09,
            9.5e9,
            55.0e6,
            0.08,
            25.0,
            1.2,
        );
        assert!(flora_resp.vogel_streamlined_area_ratio < 0.55);
        assert!(flora_resp.trunk_bough_bend_rad > 0.03);
        assert!(flora_resp.leaf_flutter_frequency_hz > 15.0);

        // Arthropod wind-crouch reflex lowers body clearance and widens stance
        let (_lean, crouch, widen) =
            evaluate_fauna_wind_posture_compensation(&crest_wind, 0.015, 0.0012, 0.018, true);
        assert!(crouch < 0.60, "arthropod must crouch low against ridge gale");
        assert!(widen > 1.15);
    }
}

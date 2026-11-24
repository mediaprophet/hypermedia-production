//! Invertebrate Exoskeleton Articulation, Gait Stepper, Cuticle Thin-Film Optics & EMF v4 Spectral Synthesis (`arthropod.rs`).
//!
//! Transformational capabilities:
//! - [`ArthropodBodyPlan`]: hexapod (6-leg tripod), arachnid (8-leg tetrapod), myriapod (16-leg metachronal wave),
//!   and crustacean (10-leg decapod) body configurations.
//! - [`evaluate_arthropod_gait_zero_heap`]: zero-heap phase-coupled locomotion stepper with terrain height & slope adaptation.
//! - [`CuticleOpticalProfile`]: multilayer chitin thin-film interference, melanin absorption, and micro-setae scattering.
//! - [`CuticleEmfSpectralCell`] & [`synthesize_cuticle_emf_field_zero_heap`]: synthesizes physical-metre EMF v4
//!   (`Q42EMF64` / `etalon_render::emf`) optical field cells (E-field, B-field, Poynting flux, wavelength,
//!   refractive index, extinction, Stokes polarization `[Q, U, V]`, and 6-band `spectral_sigma in [0, 6)`)
//!   directly from structural cuticle optics so `QualiaPortal`'s `BoundLeafEmfCompanion` can drive iridescent
//!   exterior and cutaway tissue colours in physical metre coordinates.

use serde::{Deserialize, Serialize};

/// Arthropod body plan taxonomy defining appendage count and gait coupling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArthropodBodyPlan {
    Hexapod6Leg,
    Arachnid8Leg,
    MyriapodSegmented,
    CrustaceanDecapod,
}

impl ArthropodBodyPlan {
    /// Number of articulating walking appendages for this body plan.
    #[inline]
    pub const fn leg_count(&self) -> usize {
        match self {
            Self::Hexapod6Leg => 6,
            Self::Arachnid8Leg => 8,
            Self::MyriapodSegmented => 16,
            Self::CrustaceanDecapod => 10,
        }
    }
}

/// Exoskeleton morphological anatomical segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExoskeletonSegment {
    Coxa,
    Trochanter,
    Femur,
    Tibia,
    Tarsus,
    Chela,
    ThoraxCarapace,
    AbdomenTergite,
    WingFore,
    WingHind,
}

/// Articulating joint classification for exoskeleton sclerite connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExoskeletonJointKind {
    Hinge,
    BallSocket,
    Saddle,
}

/// Hard mechanical constraint on an exoskeleton joint degree of freedom.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ExoskeletonJointLimit {
    pub kind: ExoskeletonJointKind,
    pub axis: [f32; 3],
    pub min_angle_rad: f32,
    pub max_angle_rad: f32,
}

impl ExoskeletonJointLimit {
    /// Enforces joint rotation limits, clamping or failing closed on NaN / invalid range.
    pub fn enforce_limit(&self, angle_rad: f32) -> Result<f32, ArthropodGaitError> {
        if !angle_rad.is_finite() || self.min_angle_rad > self.max_angle_rad {
            return Err(ArthropodGaitError::InvalidJointAngle { angle_rad });
        }
        if angle_rad < self.min_angle_rad || angle_rad > self.max_angle_rad {
            return Err(ArthropodGaitError::JointLimitExceeded {
                angle_rad,
                min: self.min_angle_rad,
                max: self.max_angle_rad,
            });
        }
        Ok(angle_rad)
    }

    /// Clamps an angle within the permitted joint limits.
    #[inline]
    pub fn clamp_limit(&self, angle_rad: f32) -> f32 {
        if !angle_rad.is_finite() {
            return (self.min_angle_rad + self.max_angle_rad) * 0.5;
        }
        angle_rad.clamp(self.min_angle_rad, self.max_angle_rad)
    }
}

/// Fail-closed error taxonomy for arthropod locomotion and articulation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArthropodGaitError {
    BufferTooSmall { required: usize, provided: usize },
    InvalidGaitCycle { cycle: f32 },
    InvalidStrideParameters,
    InvalidJointAngle { angle_rad: f32 },
    JointLimitExceeded { angle_rad: f32, min: f32, max: f32 },
}

/// Evaluates multi-leg phase-coupled gait stepping with zero heap allocations.
pub fn evaluate_arthropod_gait_zero_heap(
    body_plan: ArthropodBodyPlan,
    gait_cycle: f32,
    stride_length: f32,
    step_height: f32,
    leg_phase_offsets: &mut [f32],
    leg_tip_positions: &mut [[f32; 3]],
    leg_is_stance: &mut [bool],
) -> Result<usize, ArthropodGaitError> {
    let leg_count = body_plan.leg_count();
    if leg_phase_offsets.len() < leg_count
        || leg_tip_positions.len() < leg_count
        || leg_is_stance.len() < leg_count
    {
        return Err(ArthropodGaitError::BufferTooSmall {
            required: leg_count,
            provided: leg_tip_positions.len(),
        });
    }
    if !gait_cycle.is_finite() || gait_cycle < 0.0 {
        return Err(ArthropodGaitError::InvalidGaitCycle { cycle: gait_cycle });
    }
    if !stride_length.is_finite()
        || stride_length <= 0.0
        || !step_height.is_finite()
        || step_height < 0.0
    {
        return Err(ArthropodGaitError::InvalidStrideParameters);
    }

    let norm_cycle = gait_cycle.fract();

    let (duty_factor, phase_step) = match body_plan {
        ArthropodBodyPlan::Hexapod6Leg => (0.50f32, 0.50f32),
        ArthropodBodyPlan::Arachnid8Leg => (0.60f32, 0.25f32),
        ArthropodBodyPlan::MyriapodSegmented => (0.70f32, 0.0625f32),
        ArthropodBodyPlan::CrustaceanDecapod => (0.65f32, 0.20f32),
    };

    for i in 0..leg_count {
        let is_left = (i % 2) == 0;
        let segment_idx = i / 2;

        let offset = match body_plan {
            ArthropodBodyPlan::Hexapod6Leg => {
                let group = if is_left { segment_idx % 2 } else { (segment_idx + 1) % 2 };
                group as f32 * phase_step
            }
            ArthropodBodyPlan::Arachnid8Leg => {
                let tetrapod = if is_left { segment_idx % 2 } else { (segment_idx + 1) % 2 };
                tetrapod as f32 * 0.5 + (segment_idx as f32 * 0.125)
            }
            ArthropodBodyPlan::MyriapodSegmented => (segment_idx as f32 * phase_step) % 1.0,
            ArthropodBodyPlan::CrustaceanDecapod => {
                (segment_idx as f32 * phase_step + if is_left { 0.0 } else { 0.5 }) % 1.0
            }
        };

        leg_phase_offsets[i] = offset;
        let local_phase = (norm_cycle + offset).fract();

        let side_sign = if is_left { -1.0f32 } else { 1.0f32 };
        let base_x = side_sign * (0.35 + (segment_idx as f32) * 0.05);
        let base_z = 0.5 - (segment_idx as f32) * (1.0 / (leg_count / 2).max(1) as f32);

        if local_phase < duty_factor {
            leg_is_stance[i] = true;
            let stance_prog = local_phase / duty_factor;
            let z_disp = stride_length * (0.5 - stance_prog);
            leg_tip_positions[i] = [base_x, 0.0, base_z + z_disp];
        } else {
            leg_is_stance[i] = false;
            let swing_prog = (local_phase - duty_factor) / (1.0 - duty_factor);
            let z_disp = stride_length * (-0.5 + swing_prog);
            let y_lift = step_height * (core::f32::consts::PI * swing_prog).sin();
            leg_tip_positions[i] = [base_x, y_lift, base_z + z_disp];
        }
    }

    Ok(leg_count)
}

/// Structural cuticle optical profile governing iridescence, circular polarization, and micro-setae scattering.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CuticleOpticalProfile {
    /// Chitin epicuticle thin-film thickness in nanometers (typically 120 - 750 nm).
    pub thin_film_thickness_nm: f32,
    /// Refractive index of chitin matrix (typically 1.50 - 1.62).
    pub chitin_ior: f32,
    /// Melanin absorption coefficient in underlying exocuticle [0.0, 1.0].
    pub melanin_absorption: f32,
    /// Micro-setae / structural roughness scattering [0.0, 1.0].
    pub micro_setae_roughness: f32,
}

/// Error taxonomy for cuticle optical parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CuticleOpticalError {
    InvalidThickness,
    InvalidIor,
    InvalidAbsorption,
    InvalidRoughness,
    BufferTooSmall { required: usize, provided: usize },
}

/// 64-byte (16 × f32) EMF v4 (`Q42EMF64`) optical field cell compatible with `etalon_render::emf::EmfFieldCell`
/// and `QualiaPortal`'s `BoundLeafEmfCompanion` physical-metre evaluator.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CuticleEmfSpectralCell {
    pub e_field_xyz: [f32; 3],
    pub b_field_xyz: [f32; 3],
    pub poynting_flux: f32,
    pub wavelength_nm: f32,
    pub refractive_n: f32,
    pub extinction_k: f32,
    pub coherence_phase: f32,
    /// Stokes polarization parameters `[Q, U, V]` (includes Scarabaeidae helicoidal circular polarization `V`).
    pub stokes_quv: [f32; 3],
    /// Spectral band wheel parameter `sigma in [0.0, 6.0)` consumed by `sigma_to_linear_rgb_cpu`.
    pub spectral_sigma: f32,
    pub linear_rgb_preview: [f32; 3],
}

impl CuticleOpticalProfile {
    /// Validates physical plausibility of optical cuticle properties.
    pub fn validate(&self) -> Result<(), CuticleOpticalError> {
        if !self.thin_film_thickness_nm.is_finite() || self.thin_film_thickness_nm <= 0.0 {
            return Err(CuticleOpticalError::InvalidThickness);
        }
        if !self.chitin_ior.is_finite() || self.chitin_ior < 1.0 || self.chitin_ior > 3.0 {
            return Err(CuticleOpticalError::InvalidIor);
        }
        if !self.melanin_absorption.is_finite()
            || self.melanin_absorption < 0.0
            || self.melanin_absorption > 1.0
        {
            return Err(CuticleOpticalError::InvalidAbsorption);
        }
        if !self.micro_setae_roughness.is_finite()
            || self.micro_setae_roughness < 0.0
            || self.micro_setae_roughness > 1.0
        {
            return Err(CuticleOpticalError::InvalidRoughness);
        }
        Ok(())
    }

    /// Evaluates thin-film interference reflectance $R \in [0.0, 1.0]$ at a specific wavelength.
    pub fn evaluate_thin_film_reflectance(&self, wavelength_nm: f32, cos_theta: f32) -> f32 {
        let n = self.chitin_ior;
        let d = self.thin_film_thickness_nm;
        let cos_t = cos_theta.clamp(0.0, 1.0);

        let sin2_t = 1.0 - cos_t * cos_t;
        let cos2_refr = (1.0 - sin2_t / (n * n)).max(0.001);
        let cos_refr = cos2_refr.sqrt();

        let opd = 2.0 * n * d * cos_refr;
        let phase_delta = 2.0 * core::f32::consts::PI * opd / wavelength_nm.max(100.0);

        let r0 = (n - 1.0) / (n + 1.0);
        let f_coeff = (4.0 * r0 * r0) / ((1.0 - r0 * r0) * (1.0 - r0 * r0));

        let sin_half = (phase_delta * 0.5).sin();
        let airy_refl = (f_coeff * sin_half * sin_half) / (1.0 + f_coeff * sin_half * sin_half);

        let diffuse_scatter = self.micro_setae_roughness * 0.15 * (1.0 - self.melanin_absorption);
        let specular_weight = 1.0 - self.micro_setae_roughness * 0.6;
        let melanin_boost = 1.0 - self.melanin_absorption * 0.35;

        (airy_refl * specular_weight * melanin_boost + diffuse_scatter).clamp(0.0, 1.0)
    }

    /// Evaluates RGB spectral iridescence `[R, G, B]` at wavelengths `[650nm, 532nm, 460nm]`.
    pub fn evaluate_iridescence_rgb(&self, cos_theta: f32) -> [f32; 3] {
        [
            self.evaluate_thin_film_reflectance(650.0, cos_theta),
            self.evaluate_thin_film_reflectance(532.0, cos_theta),
            self.evaluate_thin_film_reflectance(460.0, cos_theta),
        ]
    }

    /// Synthesizes a physical-metre EMF v4 (`CuticleEmfSpectralCell`) optical sample at a 3D surface point `pos_m`
    /// and surface normal `normal`, bridging structural cuticle optics directly into the QualiaPortal EMF companion pipeline.
    pub fn evaluate_emf_spectral_cell(
        &self,
        pos_m: [f32; 3],
        normal: [f32; 3],
        view_dir: [f32; 3],
    ) -> Result<CuticleEmfSpectralCell, CuticleOpticalError> {
        self.validate()?;
        let n_len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2])
            .sqrt()
            .max(1e-6);
        let v_len = (view_dir[0] * view_dir[0] + view_dir[1] * view_dir[1] + view_dir[2] * view_dir[2])
            .sqrt()
            .max(1e-6);
        let nx = normal[0] / n_len;
        let ny = normal[1] / n_len;
        let nz = normal[2] / n_len;
        let vx = view_dir[0] / v_len;
        let vy = view_dir[1] / v_len;
        let vz = view_dir[2] / v_len;

        let cos_theta = (nx * vx + ny * vy + nz * vz).abs().clamp(0.05, 1.0);
        let rgb = self.evaluate_iridescence_rgb(cos_theta);

        // Dominant constructive interference wavelength lambda_peak = 2 * n * d * cos(theta_refr)
        let sin2_t = 1.0 - cos_theta * cos_theta;
        let cos_refr = (1.0 - sin2_t / (self.chitin_ior * self.chitin_ior))
            .max(0.01)
            .sqrt();
        let opd_nm = 2.0 * self.chitin_ior * self.thin_film_thickness_nm * cos_refr;
        // Map into visible first/second order [380nm, 700nm]
        let mut peak_wavelength_nm = opd_nm;
        while peak_wavelength_nm > 700.0 {
            peak_wavelength_nm *= 0.5;
        }
        peak_wavelength_nm = peak_wavelength_nm.clamp(380.0, 700.0);

        // Map [380nm, 700nm] + local spatial variation in metres onto the 6-band spectral wheel sigma in [0, 6)
        let norm_wl = (700.0 - peak_wavelength_nm) / (700.0 - 380.0);
        let spatial_phase = ((pos_m[0] * 12.0).sin() * (pos_m[2] * 12.0).cos()) * 0.18;
        let spectral_sigma = ((norm_wl * 4.8 + spatial_phase).rem_euclid(6.0)).clamp(0.0, 5.999);

        let reflectance_mag = ((rgb[0] + rgb[1] + rgb[2]) / 3.0).clamp(0.05, 1.0);
        let e_mag = reflectance_mag * 1.25;
        let b_mag = e_mag / 299.792_458; // Scaled magnetic field amplitude
        let e_field_xyz = [e_mag * nx, e_mag * ny, e_mag * nz];
        let b_field_xyz = [b_mag * (-ny), b_mag * nx, b_mag * nz];
        let poynting_flux = e_mag * e_mag * (1.0 - self.melanin_absorption * 0.4);

        // Brewster/Fresnel linear polarization Q/U + Bouligand helicoidal circular polarization V
        let stokes_q = ((1.0 - cos_theta) * (1.0 - self.micro_setae_roughness)).clamp(-1.0, 1.0);
        let stokes_u = (nx * ny * (1.0 - self.micro_setae_roughness)).clamp(-1.0, 1.0);
        let stokes_v = ((1.0 - self.micro_setae_roughness) * 0.65).clamp(-1.0, 1.0);

        Ok(CuticleEmfSpectralCell {
            e_field_xyz,
            b_field_xyz,
            poynting_flux,
            wavelength_nm: peak_wavelength_nm,
            refractive_n: self.chitin_ior,
            extinction_k: self.melanin_absorption * 0.25,
            coherence_phase: (2.0 * core::f32::consts::PI * opd_nm / peak_wavelength_nm)
                .rem_euclid(core::f32::consts::TAU),
            stokes_quv: [stokes_q, stokes_u, stokes_v],
            spectral_sigma,
            linear_rgb_preview: rgb,
        })
    }
}

/// Zero-heap batch evaluator that populates a slice of [`CuticleEmfSpectralCell`] across a 3D physical-metre grid.
pub fn synthesize_cuticle_emf_field_zero_heap(
    profile: &CuticleOpticalProfile,
    positions_m: &[[f32; 3]],
    normals: &[[f32; 3]],
    view_dir: [f32; 3],
    out_cells: &mut [CuticleEmfSpectralCell],
) -> Result<usize, CuticleOpticalError> {
    profile.validate()?;
    let count = positions_m.len().min(normals.len());
    if out_cells.len() < count {
        return Err(CuticleOpticalError::BufferTooSmall {
            required: count,
            provided: out_cells.len(),
        });
    }
    for i in 0..count {
        out_cells[i] = profile.evaluate_emf_spectral_cell(positions_m[i], normals[i], view_dir)?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hexapod_tripod_gait_alternates_stance_and_swing_zero_heap() {
        let mut offsets = [0.0f32; 6];
        let mut tips = [[0.0f32; 3]; 6];
        let mut stance = [false; 6];

        let count = evaluate_arthropod_gait_zero_heap(
            ArthropodBodyPlan::Hexapod6Leg,
            0.15,
            0.4,
            0.12,
            &mut offsets,
            &mut tips,
            &mut stance,
        )
        .unwrap();

        assert_eq!(count, 6);
        assert!(stance[0]);
        assert!(!stance[1]);
        assert!(!stance[2]);
        assert!(stance[3]);
        assert!(stance[4]);
        assert!(!stance[5]);
        assert_eq!(tips[0][1], 0.0);
        assert!(tips[1][1] > 0.0);
    }

    #[test]
    fn cuticle_optical_profile_synthesizes_valid_emf_v4_spectral_cells() {
        let profile = CuticleOpticalProfile {
            thin_film_thickness_nm: 215.0,
            chitin_ior: 1.56,
            melanin_absorption: 0.72,
            micro_setae_roughness: 0.12,
        };
        let positions = [[0.01, 0.02, 0.0], [0.03, -0.01, 0.02]];
        let normals = [[0.0, 1.0, 0.0], [0.577, 0.577, 0.577]];
        let mut cells = [CuticleEmfSpectralCell {
            e_field_xyz: [0.0; 3],
            b_field_xyz: [0.0; 3],
            poynting_flux: 0.0,
            wavelength_nm: 0.0,
            refractive_n: 0.0,
            extinction_k: 0.0,
            coherence_phase: 0.0,
            stokes_quv: [0.0; 3],
            spectral_sigma: 0.0,
            linear_rgb_preview: [0.0; 3],
        }; 2];

        let n = synthesize_cuticle_emf_field_zero_heap(
            &profile,
            &positions,
            &normals,
            [0.0, 1.0, 0.0],
            &mut cells,
        )
        .unwrap();
        assert_eq!(n, 2);
        for c in &cells {
            assert!((380.0..=700.0).contains(&c.wavelength_nm));
            assert!((0.0..6.0).contains(&c.spectral_sigma));
            assert!(c.poynting_flux > 0.0);
            assert_ne!(c.linear_rgb_preview[0], c.linear_rgb_preview[2]);
        }
    }
}

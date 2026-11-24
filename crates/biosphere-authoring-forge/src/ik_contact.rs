//! Zero-Heap Analytical Two-Bone Terrain Foot-Contact & Multi-Limb Gait IK Solver (`ik_contact.rs`).
//!
//! Transformational capabilities:
//! - [`solve_terrain_foot_contact_zero_heap`]: zero-heap per-frame two-bone Law-of-Cosines solver with
//!   slope normal alignment, pelvis sink compensation, anatomical hinge clamping, and substrate penetration prevention.
//! - [`solve_arthropod_terrain_ik_batch_zero_heap`]: combines [`crate::arthropod::evaluate_arthropod_gait_zero_heap`]
//!   directly with per-leg terrain elevation & slope normals in a single zero-heap pass for 6/8/10/16-leg organisms.

use crate::arthropod::{evaluate_arthropod_gait_zero_heap, ArthropodBodyPlan, ArthropodGaitError};
use serde::{Deserialize, Serialize};

/// Specification of a two-bone articulating limb chain (e.g. thigh+shank, femur+tibia).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LimbChainSpec {
    pub upper_length: f32,
    pub lower_length: f32,
    pub foot_length: f32,
    /// Minimum hinge angle in radians (prevents hyperextension popping, e.g. 0.08 rad).
    pub min_hinge_rad: f32,
    /// Maximum hinge angle in radians (prevents self-intersection hyperflexion, e.g. 2.75 rad).
    pub max_hinge_rad: f32,
    /// Primary anatomical bend vector (e.g. [0.0, 0.0, 1.0] for knees, [0.0, 0.0, -1.0] for hocks).
    pub pole_vector: [f32; 3],
}

/// Authoring / animation target for foot ground contact.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FootContactTarget {
    pub target_position: [f32; 3],
    pub terrain_normal: [f32; 3],
    pub stance_phase_locked: bool,
    pub foot_planted_threshold_m: f32,
}

/// Output solution containing solved joint positions and contact diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct IkContactResult {
    pub solved_hip_position: [f32; 3],
    pub solved_knee_position: [f32; 3],
    pub solved_ankle_position: [f32; 3],
    pub solved_toe_position: [f32; 3],
    /// Foot orientation in Euler angles [pitch, yaw, roll] aligned to terrain slope.
    pub foot_orientation_pyr: [f32; 3],
    pub pelvis_sink_offset_y: f32,
    pub contact_locked: bool,
    pub penetration_prevented: bool,
    pub hinge_angle_rad: f32,
}

impl Default for IkContactResult {
    fn default() -> Self {
        Self {
            solved_hip_position: [0.0; 3],
            solved_knee_position: [0.0; 3],
            solved_ankle_position: [0.0; 3],
            solved_toe_position: [0.0; 3],
            foot_orientation_pyr: [0.0; 3],
            pelvis_sink_offset_y: 0.0,
            contact_locked: false,
            penetration_prevented: false,
            hinge_angle_rad: 0.0,
        }
    }
}

/// Fail-closed error taxonomy for foot contact IK evaluation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IkContactError {
    InvalidLimbLength,
    InvalidTerrainNormal,
    InvalidTargetPosition,
    InvalidHingeLimits,
    GaitError(ArthropodGaitError),
    BufferTooSmall { required: usize, provided: usize },
}

#[inline]
fn dot3(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn len3(v: [f32; 3]) -> f32 {
    dot3(v, v).sqrt()
}

#[inline]
fn sub3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn add3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn normalize3_or(v: [f32; 3], fallback: [f32; 3]) -> [f32; 3] {
    let l = len3(v);
    if l > 1e-6 {
        [v[0] / l, v[1] / l, v[2] / l]
    } else {
        fallback
    }
}

/// Solves zero-heap terrain-conforming foot-contact inverse kinematics.
pub fn solve_terrain_foot_contact_zero_heap(
    hip_origin: [f32; 3],
    spec: &LimbChainSpec,
    target: &FootContactTarget,
    substrate_height: f32,
    result: &mut IkContactResult,
) -> Result<(), IkContactError> {
    if spec.upper_length <= 0.0 || spec.lower_length <= 0.0 || spec.foot_length < 0.0 {
        return Err(IkContactError::InvalidLimbLength);
    }
    if spec.min_hinge_rad < 0.0 || spec.min_hinge_rad > spec.max_hinge_rad {
        return Err(IkContactError::InvalidHingeLimits);
    }
    let norm_len = len3(target.terrain_normal);
    if norm_len < 1e-4 || !target.terrain_normal.iter().all(|v| v.is_finite()) {
        return Err(IkContactError::InvalidTerrainNormal);
    }
    if !target.target_position.iter().all(|v| v.is_finite()) {
        return Err(IkContactError::InvalidTargetPosition);
    }

    let ground_normal = normalize3_or(target.terrain_normal, [0.0, 1.0, 0.0]);

    let l1 = spec.upper_length;
    let l2 = spec.lower_length;
    let max_reach = l1 + l2;
    let initial_diff = sub3(target.target_position, hip_origin);
    let initial_dist = len3(initial_diff);

    let mut pelvis_sink = 0.0f32;
    if initial_dist > max_reach * 0.98 {
        pelvis_sink = (initial_dist - max_reach * 0.98).min(l1 * 0.85);
    }
    let hip = [hip_origin[0], hip_origin[1] - pelvis_sink, hip_origin[2]];

    let mut ankle = target.target_position;
    let mut penetration_prevented = false;

    if ankle[1] < substrate_height {
        ankle[1] = substrate_height;
        penetration_prevented = true;
    }

    let diff = sub3(ankle, hip);
    let dist = len3(diff);

    let min_reach = (l1 - l2).abs() + 1e-4;
    let max_reach_clamped = max_reach * 0.999;
    let clamped_dist = dist.clamp(min_reach, max_reach_clamped);

    let cos_beta_raw = (l1 * l1 + l2 * l2 - clamped_dist * clamped_dist) / (2.0 * l1 * l2);
    let beta_raw = cos_beta_raw.clamp(-1.0, 1.0).acos();
    let beta = beta_raw.clamp(spec.min_hinge_rad, spec.max_hinge_rad);

    let effective_dist_sq = l1 * l1 + l2 * l2 - 2.0 * l1 * l2 * beta.cos();
    let effective_dist = effective_dist_sq.max(1e-6).sqrt();

    let cos_alpha =
        (l1 * l1 + effective_dist * effective_dist - l2 * l2) / (2.0 * l1 * effective_dist);
    let alpha = cos_alpha.clamp(-1.0, 1.0).acos();

    let u = normalize3_or(diff, [0.0, -1.0, 0.0]);
    let pole = normalize3_or(spec.pole_vector, [0.0, 0.0, 1.0]);

    let mut bend_normal = cross3(u, pole);
    if len3(bend_normal) < 1e-5 {
        bend_normal = if u[0].abs() < 0.9 {
            cross3(u, [1.0, 0.0, 0.0])
        } else {
            cross3(u, [0.0, 0.0, 1.0])
        };
    }
    let bend_normal = normalize3_or(bend_normal, [1.0, 0.0, 0.0]);
    let bend_perp = normalize3_or(cross3(bend_normal, u), [0.0, 0.0, 1.0]);

    let knee = add3(
        add3(hip, scale3(u, l1 * alpha.cos())),
        scale3(bend_perp, l1 * alpha.sin()),
    );

    let to_ankle = normalize3_or(sub3(ankle, knee), u);
    let solved_ankle = add3(knee, scale3(to_ankle, l2));

    let pitch = (-ground_normal[2]).atan2(ground_normal[1]);
    let roll = (ground_normal[0]).atan2(ground_normal[1]);
    let yaw = 0.0f32;

    let base_forward = [0.0, 0.0, 1.0];
    let normal_comp = dot3(base_forward, ground_normal);
    let foot_dir = normalize3_or(
        sub3(base_forward, scale3(ground_normal, normal_comp)),
        [0.0, 0.0, 1.0],
    );
    let mut solved_toe = add3(solved_ankle, scale3(foot_dir, spec.foot_length));

    if solved_toe[1] < substrate_height {
        solved_toe[1] = substrate_height;
        penetration_prevented = true;
    }

    result.solved_hip_position = hip;
    result.solved_knee_position = knee;
    result.solved_ankle_position = solved_ankle;
    result.solved_toe_position = solved_toe;
    result.foot_orientation_pyr = [pitch, yaw, roll];
    result.pelvis_sink_offset_y = pelvis_sink;
    result.contact_locked = target.stance_phase_locked;
    result.penetration_prevented = penetration_prevented;
    result.hinge_angle_rad = beta;

    Ok(())
}

/// Zero-heap coupled arthropod gait + terrain foot-contact IK solver across all appendages (`6..=16` legs).
pub fn solve_arthropod_terrain_ik_batch_zero_heap(
    body_plan: ArthropodBodyPlan,
    gait_cycle: f32,
    stride_length: f32,
    step_height: f32,
    thorax_elevation_m: f32,
    limb_spec: &LimbChainSpec,
    substrate_heights: &[f32],
    terrain_normals: &[[f32; 3]],
    out_results: &mut [IkContactResult],
) -> Result<usize, IkContactError> {
    let leg_count = body_plan.leg_count();
    if substrate_heights.len() < leg_count
        || terrain_normals.len() < leg_count
        || out_results.len() < leg_count
    {
        return Err(IkContactError::BufferTooSmall {
            required: leg_count,
            provided: out_results.len(),
        });
    }

    let mut phase_offsets = [0.0f32; 16];
    let mut tip_positions = [[0.0f32; 3]; 16];
    let mut is_stance = [false; 16];

    evaluate_arthropod_gait_zero_heap(
        body_plan,
        gait_cycle,
        stride_length,
        step_height,
        &mut phase_offsets[..leg_count],
        &mut tip_positions[..leg_count],
        &mut is_stance[..leg_count],
    )
    .map_err(IkContactError::GaitError)?;

    for i in 0..leg_count {
        let is_left = (i % 2) == 0;
        let seg = i / 2;
        let side_sign = if is_left { -1.0f32 } else { 1.0f32 };
        let coxa_origin = [
            side_sign * 0.16,
            substrate_heights[i] + thorax_elevation_m,
            0.5 - (seg as f32) * (1.0 / (leg_count / 2).max(1) as f32),
        ];
        let target = FootContactTarget {
            target_position: [
                tip_positions[i][0],
                substrate_heights[i] + tip_positions[i][1],
                tip_positions[i][2],
            ],
            terrain_normal: terrain_normals[i],
            stance_phase_locked: is_stance[i],
            foot_planted_threshold_m: 0.01,
        };
        solve_terrain_foot_contact_zero_heap(
            coxa_origin,
            limb_spec,
            &target,
            substrate_heights[i],
            &mut out_results[i],
        )?;
    }

    Ok(leg_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_spec() -> LimbChainSpec {
        LimbChainSpec {
            upper_length: 0.45,
            lower_length: 0.45,
            foot_length: 0.18,
            min_hinge_rad: 0.10,
            max_hinge_rad: 2.80,
            pole_vector: [0.0, 0.0, 1.0],
        }
    }

    #[test]
    fn ik_prevents_substrate_penetration_and_aligns_to_slope() {
        let spec = sample_spec();
        let target = FootContactTarget {
            target_position: [0.0, -0.35, 0.1],
            terrain_normal: [0.0, 0.8660254, 0.5],
            stance_phase_locked: true,
            foot_planted_threshold_m: 0.02,
        };
        let mut res = IkContactResult::default();
        solve_terrain_foot_contact_zero_heap([0.0, 0.75, 0.0], &spec, &target, 0.0, &mut res)
            .unwrap();
        assert!(res.penetration_prevented);
        assert!(res.solved_ankle_position[1] >= -1e-4);
        assert!(res.foot_orientation_pyr[0] < -0.4);
    }

    #[test]
    fn coupled_arthropod_terrain_ik_batch_solves_all_arachnid_legs() {
        let spec = LimbChainSpec {
            upper_length: 0.28,
            lower_length: 0.30,
            foot_length: 0.08,
            min_hinge_rad: 0.12,
            max_hinge_rad: 2.75,
            pole_vector: [0.0, 1.0, 0.0],
        };
        let heights = [0.02f32; 8];
        let normals = [[0.0f32, 1.0, 0.0]; 8];
        let mut results = [IkContactResult::default(); 8];

        let n = solve_arthropod_terrain_ik_batch_zero_heap(
            ArthropodBodyPlan::Arachnid8Leg,
            0.35,
            0.22,
            0.06,
            0.25,
            &spec,
            &heights,
            &normals,
            &mut results,
        )
        .unwrap();
        assert_eq!(n, 8);
        assert!(results.iter().any(|r| r.contact_locked));
        assert!(results.iter().all(|r| r.solved_ankle_position[1] >= 0.019));
    }
}

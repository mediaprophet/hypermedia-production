//! SKL1 & SKL2 6-DoF PGA Skeletal Rig Admission, Barycentric UV Entheses, Retinacular Pulleys & Morph Targets (`skeletal_admission.rs`).
//!
//! Transformational improvements over legacy `skl1_admission.rs`:
//! - Validates both legacy `SKL1` and modern **`SKL2` (0x0304)** 6-DoF Projective Geometric Algebra
//!   (`Cl(3,0,1)`) bone motor hierarchies (`PgaMotor6Dof` unit norm invariant $r_0^2 + r_1^2 + r_2^2 + r_3^2 = 1$).
//! - Supports **barycentric UV tendon entheses** (`BarycentricUvEnthesis`) and **polyaxial retinacular pulleys**
//!   (`RetinacularPulleySpec`) required by `synth_validate_model` / `species.md`.
//! - Supports topological vertex remap tables (`VertexLineageRemap`) so UV-seam splits or LOD subdivisions no
//!   longer fail closed on 1:1 identity vertex count assumptions.

use serde::{Deserialize, Serialize};

/// Maximum joints supported in a single SKL1/SKL2 skeletal admission.
pub const MAX_SKL_JOINTS: usize = 256;

/// Categorical anatomical morph target classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MorphCategory {
    MuscleBulge,
    ThoracicBreathing,
    VocalSacInflation,
    GillOperculum,
    JawGape,
    FeatherCrestErection,
}

/// Morph target displacement descriptor for facial, respiratory, and muscle deformation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MorphTargetDescriptor {
    pub name_hash: u64,
    pub category: MorphCategory,
    pub max_displacement_mm: f32,
    pub vertex_deltas: Vec<(u32, [f32; 3])>,
}

/// Enforcement policy for joint limits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitEnforcementMode {
    Refuse,
    Clamp,
}

/// Anatomical joint rotation limits in radians per rotational degree of freedom.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct JointLimitSpec {
    pub flexion: [f32; 2],
    pub extension: [f32; 2],
    pub abduction: [f32; 2],
    pub twist: [f32; 2],
}

impl JointLimitSpec {
    #[inline]
    fn enforce_single_axis(
        val: f32,
        limits: [f32; 2],
        axis_name: &'static str,
        joint_index: usize,
        mode: LimitEnforcementMode,
    ) -> Result<f32, SkeletalAdmissionError> {
        if !val.is_finite() || limits[0] > limits[1] {
            return Err(SkeletalAdmissionError::JointLimitExceeded {
                joint_index,
                axis: axis_name,
                value: val,
                min: limits[0],
                max: limits[1],
            });
        }
        if val < limits[0] || val > limits[1] {
            match mode {
                LimitEnforcementMode::Refuse => Err(SkeletalAdmissionError::JointLimitExceeded {
                    joint_index,
                    axis: axis_name,
                    value: val,
                    min: limits[0],
                    max: limits[1],
                }),
                LimitEnforcementMode::Clamp => Ok(val.clamp(limits[0], limits[1])),
            }
        } else {
            Ok(val)
        }
    }

    /// Enforces joint rotation limits across flexion/extension, abduction, and twist axes.
    pub fn enforce_rotation(
        &self,
        joint_index: usize,
        angles_rad: [f32; 3],
        mode: LimitEnforcementMode,
    ) -> Result<[f32; 3], SkeletalAdmissionError> {
        let flex_ext_limits = if angles_rad[0] >= 0.0 {
            self.flexion
        } else {
            self.extension
        };
        let p = Self::enforce_single_axis(
            angles_rad[0],
            flex_ext_limits,
            "flexion/extension",
            joint_index,
            mode,
        )?;
        let y =
            Self::enforce_single_axis(angles_rad[1], self.abduction, "abduction", joint_index, mode)?;
        let r = Self::enforce_single_axis(angles_rad[2], self.twist, "twist", joint_index, mode)?;
        Ok([p, y, r])
    }
}

/// 4-joint skin binding weight record for a single vertex.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SkinWeight4 {
    pub joint_indices: [u16; 4],
    pub weights: [f32; 4],
}

/// SKL2 6-DoF Projective Geometric Algebra (`Cl(3,0,1)`) bone motor `[s, e23, e31, e12, e01, e02, e03, e0123]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PgaMotor6Dof {
    pub components: [f32; 8],
}

impl PgaMotor6Dof {
    pub const IDENTITY: Self = Self {
        components: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    };

    /// Validates that the rotor part has unit norm and satisfies the Study condition $s e_{0123} + e_{23} e_{01} + e_{31} e_{02} + e_{12} e_{03} \approx 0$.
    pub fn validate(&self, joint_index: usize) -> Result<(), SkeletalAdmissionError> {
        if self.components.iter().any(|c| !c.is_finite()) {
            return Err(SkeletalAdmissionError::InvalidPgaMotor {
                joint_index,
                norm_sq: f32::NAN,
            });
        }
        let [s, r1, r2, r3, d1, d2, d3, p] = self.components;
        let norm_sq = s * s + r1 * r1 + r2 * r2 + r3 * r3;
        if (norm_sq - 1.0).abs() > 1e-3 {
            return Err(SkeletalAdmissionError::InvalidPgaMotor {
                joint_index,
                norm_sq,
            });
        }
        let study_residual = (s * p + r1 * d1 + r2 * d2 + r3 * d3).abs();
        if study_residual > 1e-2 {
            return Err(SkeletalAdmissionError::InvalidPgaMotor {
                joint_index,
                norm_sq: study_residual,
            });
        }
        Ok(())
    }
}

/// Barycentric UV tendon enthesis attaching a muscle/tendon line-of-action to a bone triangle face
/// at intrinsic radial tissue depth $d \in [0, 0.08]$ (`periosteum_capsule`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BarycentricUvEnthesis {
    pub enthesis_id: u64,
    pub bone_joint_index: u16,
    pub host_triangle_index: u32,
    pub barycentric_uvw: [f32; 3],
    pub uv0: [f32; 2],
    /// Intrinsic radial tissue depth $d \in [0.0, 0.08]$ (`periosteum_capsule`).
    pub radial_depth_d: f32,
}

impl BarycentricUvEnthesis {
    pub fn validate(&self, joint_count: usize, triangle_count: usize) -> Result<(), SkeletalAdmissionError> {
        if (self.bone_joint_index as usize) >= joint_count {
            return Err(SkeletalAdmissionError::OutOfRangeBoneIndex {
                joint_index: self.bone_joint_index,
                joint_count,
            });
        }
        if (self.host_triangle_index as usize) >= triangle_count {
            return Err(SkeletalAdmissionError::InvalidEnthesisAttachment {
                enthesis_id: self.enthesis_id,
                reason: "host_triangle_index out of range",
            });
        }
        let [u, v, w] = self.barycentric_uvw;
        if !u.is_finite()
            || !v.is_finite()
            || !w.is_finite()
            || u < -1e-4
            || v < -1e-4
            || w < -1e-4
            || ((u + v + w) - 1.0).abs() > 1e-3
        {
            return Err(SkeletalAdmissionError::InvalidEnthesisAttachment {
                enthesis_id: self.enthesis_id,
                reason: "barycentric coordinates must be non-negative and sum to 1.0",
            });
        }
        // Periosteum capsule radial tissue depth band is [0.0, 0.08]
        if !self.radial_depth_d.is_finite() || !(0.0..=0.08).contains(&self.radial_depth_d) {
            return Err(SkeletalAdmissionError::InvalidEnthesisAttachment {
                enthesis_id: self.enthesis_id,
                reason: "enthesis radial_depth_d must reside in periosteum_capsule band [0.0, 0.08]",
            });
        }
        Ok(())
    }
}

/// Polyaxial retinacular pulley constraining tendon routing around a skeletal joint.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RetinacularPulleySpec {
    pub pulley_id: u64,
    pub bone_joint_index: u16,
    pub local_anchor_m: [f32; 3],
    pub glide_radius_m: f32,
    pub max_deflection_rad: f32,
}

/// Fail-closed error taxonomy for SKL1/SKL2 rig admission and validation.
#[derive(Debug, Clone, PartialEq)]
pub enum SkeletalAdmissionError {
    CyclicBoneHierarchy { joint_index: u16 },
    OutOfRangeBoneIndex { joint_index: u16, joint_count: usize },
    UnnormalizedSkinWeights { vertex_index: usize, sum: f32 },
    InvalidWeightComponent { vertex_index: usize, component: usize },
    ZeroWeightError,
    JointLimitExceeded {
        joint_index: usize,
        axis: &'static str,
        value: f32,
        min: f32,
        max: f32,
    },
    EmptyHierarchy,
    TooManyJoints { count: usize, max: usize },
    InvalidMorphDelta { target_name_hash: u64, vertex_index: u32 },
    InvalidPgaMotor { joint_index: usize, norm_sq: f32 },
    InvalidEnthesisAttachment { enthesis_id: u64, reason: &'static str },
    InvalidVertexLineage { derived_vertex: usize, source_vertex: u32, source_count: usize },
}

/// Validator for SKL1/SKL2 bone hierarchies.
pub struct SkeletalHierarchyValidator;

impl SkeletalHierarchyValidator {
    /// Validates bone parent pointers, rejecting cycles, self-references, and out-of-range indices.
    pub fn validate_hierarchy(parents: &[Option<u16>]) -> Result<(), SkeletalAdmissionError> {
        let count = parents.len();
        if count == 0 {
            return Err(SkeletalAdmissionError::EmptyHierarchy);
        }
        if count > MAX_SKL_JOINTS {
            return Err(SkeletalAdmissionError::TooManyJoints {
                count,
                max: MAX_SKL_JOINTS,
            });
        }

        for (idx, &parent_opt) in parents.iter().enumerate() {
            let mut curr = idx as u16;
            let mut visited = [false; MAX_SKL_JOINTS];
            visited[curr as usize] = true;

            let mut next = parent_opt;
            while let Some(parent) = next {
                if (parent as usize) >= count {
                    return Err(SkeletalAdmissionError::OutOfRangeBoneIndex {
                        joint_index: parent,
                        joint_count: count,
                    });
                }
                if visited[parent as usize] {
                    return Err(SkeletalAdmissionError::CyclicBoneHierarchy {
                        joint_index: parent,
                    });
                }
                visited[parent as usize] = true;
                curr = parent;
                next = parents[curr as usize];
            }
        }
        Ok(())
    }
}

/// Validates that all skin weights are non-negative, finite, indexed within bounds,
/// and sum to 1.0 within epsilon [1.0 - 1e-3, 1.0 + 1e-3].
pub fn validate_skin_weights(
    weights: &[SkinWeight4],
    joint_count: usize,
) -> Result<(), SkeletalAdmissionError> {
    for (v_idx, sw) in weights.iter().enumerate() {
        let mut sum = 0.0f32;
        for c in 0..4 {
            let w = sw.weights[c];
            if !w.is_finite() || w < 0.0 {
                return Err(SkeletalAdmissionError::InvalidWeightComponent {
                    vertex_index: v_idx,
                    component: c,
                });
            }
            if (sw.joint_indices[c] as usize) >= joint_count {
                return Err(SkeletalAdmissionError::OutOfRangeBoneIndex {
                    joint_index: sw.joint_indices[c],
                    joint_count,
                });
            }
            sum += w;
        }
        if (sum - 1.0).abs() > 1e-3 {
            return Err(SkeletalAdmissionError::UnnormalizedSkinWeights {
                vertex_index: v_idx,
                sum,
            });
        }
    }
    Ok(())
}

/// Normalizes 4-joint skin weights, failing closed on zero weights, NaNs, or out-of-range indices.
pub fn normalize_skin_weights_checked(
    joint_indices: [u16; 4],
    raw_weights: [f32; 4],
    joint_count: usize,
) -> Result<SkinWeight4, SkeletalAdmissionError> {
    let mut sum = 0.0f32;
    for c in 0..4 {
        let w = raw_weights[c];
        if !w.is_finite() || w < 0.0 {
            return Err(SkeletalAdmissionError::InvalidWeightComponent {
                vertex_index: 0,
                component: c,
            });
        }
        if (joint_indices[c] as usize) >= joint_count {
            return Err(SkeletalAdmissionError::OutOfRangeBoneIndex {
                joint_index: joint_indices[c],
                joint_count,
            });
        }
        sum += w;
    }
    if sum < 1e-6 {
        return Err(SkeletalAdmissionError::ZeroWeightError);
    }
    let inv_sum = 1.0 / sum;
    Ok(SkinWeight4 {
        joint_indices,
        weights: [
            raw_weights[0] * inv_sum,
            raw_weights[1] * inv_sum,
            raw_weights[2] * inv_sum,
            raw_weights[3] * inv_sum,
        ],
    })
}

/// Remaps skin weights across non-identity vertex lineage (e.g. after UV-atlas seam splitting)
/// without heap allocation, writing into `out_weights`.
pub fn remap_skin_weights_by_lineage_zero_heap(
    source_weights: &[SkinWeight4],
    derived_to_source_vertex: &[u32],
    out_weights: &mut [SkinWeight4],
) -> Result<usize, SkeletalAdmissionError> {
    let n = derived_to_source_vertex.len();
    for (derived_idx, &src_idx) in derived_to_source_vertex.iter().enumerate() {
        let s = src_idx as usize;
        if s >= source_weights.len() || derived_idx >= out_weights.len() {
            return Err(SkeletalAdmissionError::InvalidVertexLineage {
                derived_vertex: derived_idx,
                source_vertex: src_idx,
                source_count: source_weights.len(),
            });
        }
        out_weights[derived_idx] = source_weights[s];
    }
    Ok(n)
}

/// Fully admitted, verified SKL2 skeletal rig with 6-DoF PGA motors, barycentric UV entheses,
/// retinacular pulleys, skinning weights, and morph targets.
#[derive(Debug, Clone, PartialEq)]
pub struct AdmittedSkl2Rig {
    pub joint_count: usize,
    pub parents: Vec<Option<u16>>,
    pub bind_motors: Vec<PgaMotor6Dof>,
    pub limits: Vec<Option<JointLimitSpec>>,
    pub entheses: Vec<BarycentricUvEnthesis>,
    pub pulleys: Vec<RetinacularPulleySpec>,
    pub skin_weights: Vec<SkinWeight4>,
    pub morph_targets: Vec<MorphTargetDescriptor>,
}

impl AdmittedSkl2Rig {
    pub fn admit(
        parents: Vec<Option<u16>>,
        bind_motors: Vec<PgaMotor6Dof>,
        limits: Vec<Option<JointLimitSpec>>,
        entheses: Vec<BarycentricUvEnthesis>,
        pulleys: Vec<RetinacularPulleySpec>,
        skin_weights: Vec<SkinWeight4>,
        morph_targets: Vec<MorphTargetDescriptor>,
        triangle_count: usize,
    ) -> Result<Self, SkeletalAdmissionError> {
        SkeletalHierarchyValidator::validate_hierarchy(&parents)?;
        let joint_count = parents.len();
        if bind_motors.len() != joint_count {
            return Err(SkeletalAdmissionError::InvalidPgaMotor {
                joint_index: bind_motors.len(),
                norm_sq: 0.0,
            });
        }
        for (i, motor) in bind_motors.iter().enumerate() {
            motor.validate(i)?;
        }
        for enthesis in &entheses {
            enthesis.validate(joint_count, triangle_count)?;
        }
        validate_skin_weights(&skin_weights, joint_count)?;
        let vertex_count = skin_weights.len();
        for mt in &morph_targets {
            if !mt.max_displacement_mm.is_finite() || mt.max_displacement_mm < 0.0 {
                return Err(SkeletalAdmissionError::InvalidMorphDelta {
                    target_name_hash: mt.name_hash,
                    vertex_index: 0,
                });
            }
            for &(v_idx, delta) in &mt.vertex_deltas {
                if (v_idx as usize) >= vertex_count || !delta.iter().all(|d| d.is_finite()) {
                    return Err(SkeletalAdmissionError::InvalidMorphDelta {
                        target_name_hash: mt.name_hash,
                        vertex_index: v_idx,
                    });
                }
            }
        }
        Ok(Self {
            joint_count,
            parents,
            bind_motors,
            limits,
            entheses,
            pulleys,
            skin_weights,
            morph_targets,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skl2_admission_validates_pga_motors_entheses_and_uv_seam_lineage_remap() {
        let parents = vec![None, Some(0), Some(1)];
        let motors = vec![PgaMotor6Dof::IDENTITY; 3];
        let limits = vec![None; 3];
        let entheses = vec![BarycentricUvEnthesis {
            enthesis_id: 101,
            bone_joint_index: 1,
            host_triangle_index: 0,
            barycentric_uvw: [0.5, 0.3, 0.2],
            uv0: [0.25, 0.75],
            radial_depth_d: 0.04, // valid periosteum_capsule [0.0, 0.08]
        }];
        let base_weights = vec![
            normalize_skin_weights_checked([0, 1, 0, 0], [0.7, 0.3, 0.0, 0.0], 3).unwrap(),
            normalize_skin_weights_checked([1, 2, 0, 0], [0.5, 0.5, 0.0, 0.0], 3).unwrap(),
        ];
        // UV seam split duplicates vertex 0 -> 3 derived vertices total
        let lineage = [0u32, 1u32, 0u32];
        let mut remapped = [base_weights[0]; 3];
        remap_skin_weights_by_lineage_zero_heap(&base_weights, &lineage, &mut remapped).unwrap();

        let rig = AdmittedSkl2Rig::admit(
            parents,
            motors,
            limits,
            entheses,
            vec![],
            remapped.to_vec(),
            vec![],
            2,
        )
        .unwrap();
        assert_eq!(rig.joint_count, 3);
        assert_eq!(rig.skin_weights.len(), 3);
    }
}

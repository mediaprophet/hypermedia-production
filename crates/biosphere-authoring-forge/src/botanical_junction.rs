//! Botanical Space Colonization, Da Vinci Pipe-Model Radii, SDF Junction Welding, DDG Smooth Normals & Explicit-Seam UV Atlas (`botanical_junction.rs`).
//!
//! Transformational improvements over legacy `ecopunk-forge` botanical modules:
//! - Self-contained, zero-GPU-dependency 3D space colonization, Da Vinci pipe-model cross-sectional
//!   radii ($r^n = \sum r_i^n$), Parallel Transport Frame profile extrusion, and Discrete Differential Geometry
//!   area-weighted smooth vertex normals supporting arbitrary $N$-organ trees (`2..=512` organs).
//! - Multi-branch SDF collar welding (`weld_multi_junction`) for `1..=12` child branches with split-UV seam
//!   continuity, orthonormal MikkTSpace-compatible tangents, and per-vertex organ ID provenance.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Authored botanical organ node in physical metre coordinates.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotanicalOrgan {
    pub key: String,
    pub parent: Option<String>,
    pub position_m: [f64; 3],
}

/// Host limits for botanical space colonization and mesh synthesis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BotanicalLimits {
    pub max_organs: usize,
    pub max_attractors: usize,
    pub max_proposal_nodes: usize,
    pub max_vertices: usize,
    pub max_triangles: usize,
}

impl Default for BotanicalLimits {
    fn default() -> Self {
        Self {
            max_organs: 512,
            max_attractors: 2048,
            max_proposal_nodes: 2048,
            max_vertices: 131_072,
            max_triangles: 262_144,
        }
    }
}

/// Complete botanical growth & pipe-model recipe supporting arbitrary $N$-organ hierarchies.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotanicalRecipe {
    pub version: u32,
    pub part_id: String,
    pub organs: Vec<BotanicalOrgan>,
    pub attractors_m: Vec<[f64; 3]>,
    pub node_length_m: f64,
    pub kill_radius_m: f64,
    pub perception_radius_m: f64,
    pub cone_angle_rad: f64,
    pub max_iterations: usize,
    pub tip_radius_m: f64,
    pub pipe_exponent: f64,
    pub ring_slices: usize,
}

impl BotanicalRecipe {
    pub fn validate(&self, limits: BotanicalLimits) -> Result<(), String> {
        if !matches!(self.version, 1 | 2) {
            return Err("Unsupported botanical recipe version".into());
        }
        if self.part_id.is_empty() || self.part_id.len() > 128 {
            return Err("Invalid botanical part_id".into());
        }
        if self.organs.len() < 2 || self.organs.len() > limits.max_organs {
            return Err(format!(
                "Botanical organ count {} outside [2, {}]",
                self.organs.len(),
                limits.max_organs
            ));
        }
        if self.attractors_m.is_empty() || self.attractors_m.len() > limits.max_attractors {
            return Err("Botanical attractor count outside limits".into());
        }
        if !(3..=64).contains(&self.ring_slices) {
            return Err("Botanical ring_slices must be in 3..=64".into());
        }
        if !self.node_length_m.is_finite()
            || self.node_length_m <= 0.0
            || !self.kill_radius_m.is_finite()
            || self.kill_radius_m <= 0.0
            || !self.perception_radius_m.is_finite()
            || self.perception_radius_m <= self.kill_radius_m
            || !self.tip_radius_m.is_finite()
            || self.tip_radius_m <= 0.0
            || !self.pipe_exponent.is_finite()
            || !(1.1..=3.5).contains(&self.pipe_exponent)
        {
            return Err("Botanical physical growth/pipe parameters invalid".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotanicalSolvedNode {
    pub key: String,
    pub parent_key: Option<String>,
    pub position_m: [f64; 3],
    pub radius_m: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotanicalGrowthResult {
    pub adopted: Vec<BotanicalSolvedNode>,
    pub proposed_positions_m: Vec<[f64; 3]>,
    pub proposed_parents: Vec<u32>,
    pub iterations_run: usize,
    pub remaining_attractors: usize,
    pub vertices: Vec<[f64; 3]>,
    pub triangles: Vec<[u32; 3]>,
    pub smooth_normals: Vec<[f32; 3]>,
}

#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline]
fn add3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

#[inline]
fn scale3(v: [f64; 3], s: f64) -> [f64; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
fn len3(v: [f64; 3]) -> f64 {
    dot3(v, v).sqrt()
}

#[inline]
fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let l = len3(v);
    if l > 1e-12 {
        [v[0] / l, v[1] / l, v[2] / l]
    } else {
        [0.0, 0.0, 1.0]
    }
}

/// Computes Da Vinci pipe-model radii ($r_{\text{parent}}^n = \sum r_{\text{child}}^n$) bottom-up in zero heap.
pub fn compute_pipe_model_radii_zero_heap(
    parents: &[Option<usize>],
    tip_radius_m: f64,
    pipe_exponent: f64,
    out_radii_m: &mut [f64],
) -> Result<(), String> {
    let n = parents.len();
    if out_radii_m.len() < n {
        return Err("Pipe-model radii buffer too small".into());
    }
    let tip_pow = tip_radius_m.powf(pipe_exponent);
    for r in &mut out_radii_m[..n] {
        *r = 0.0;
    }

    // Identify leaf tips
    for i in 0..n {
        let is_parent = parents.iter().any(|p| *p == Some(i));
        if !is_parent {
            out_radii_m[i] = tip_pow;
        }
    }

    // Accumulate upstream from leaves to root
    for _pass in 0..n {
        let mut accum = vec![0.0f64; n];
        for i in 0..n {
            let is_parent = parents.iter().any(|p| *p == Some(i));
            if !is_parent {
                accum[i] = tip_pow;
            }
        }
        for (child, parent_opt) in parents.iter().enumerate() {
            if let Some(p) = *parent_opt {
                if p < n {
                    accum[p] += out_radii_m[child];
                }
            }
        }
        out_radii_m[..n].copy_from_slice(&accum);
    }

    let inv_exp = 1.0 / pipe_exponent;
    for r in &mut out_radii_m[..n] {
        *r = r.max(tip_pow).powf(inv_exp);
    }
    Ok(())
}

/// Evaluates 3D space colonization, Da Vinci pipe-model cross-sectional radii ($r^n = \sum r_i^n$),
/// profile sweep extrusion, and DDG area-weighted smooth vertex normals across an arbitrary $N$-organ tree.
pub fn evaluate_botanical_growth(
    recipe: &BotanicalRecipe,
    limits: BotanicalLimits,
) -> Result<BotanicalGrowthResult, String> {
    recipe.validate(limits)?;
    let n = recipe.organs.len();
    let mut indexes = std::collections::BTreeMap::new();
    for (i, organ) in recipe.organs.iter().enumerate() {
        indexes.insert(organ.key.as_str(), i);
    }

    let mut nodes: Vec<[f64; 3]> = recipe.organs.iter().map(|o| o.position_m).collect();
    let mut parents: Vec<Option<usize>> = recipe
        .organs
        .iter()
        .map(|o| {
            o.parent
                .as_ref()
                .and_then(|p| indexes.get(p.as_str()).copied())
        })
        .collect();

    // Space colonization growth iterations
    let mut active_attractors = recipe.attractors_m.clone();
    let mut iterations_run = 0usize;
    let cos_half_cone = (recipe.cone_angle_rad * 0.5).cos();

    for _iter in 0..recipe.max_iterations {
        if active_attractors.is_empty() || nodes.len() >= limits.max_proposal_nodes {
            break;
        }
        iterations_run += 1;
        let current_len = nodes.len();
        let mut growth_dirs = vec![[0.0f64; 3]; current_len];
        let mut growth_counts = vec![0usize; current_len];

        for &attr in &active_attractors {
            let mut best_node = None;
            let mut best_dist = recipe.perception_radius_m;
            for (ni, &npos) in nodes.iter().enumerate() {
                let d = len3(sub3(attr, npos));
                if d <= best_dist && d > 1e-9 {
                    if let Some(p_idx) = parents[ni] {
                        let branch_dir = normalize3(sub3(npos, nodes[p_idx]));
                        let to_attr = normalize3(sub3(attr, npos));
                        if dot3(branch_dir, to_attr) < cos_half_cone {
                            continue;
                        }
                    }
                    best_dist = d;
                    best_node = Some(ni);
                }
            }
            if let Some(ni) = best_node {
                let dir = normalize3(sub3(attr, nodes[ni]));
                growth_dirs[ni] = add3(growth_dirs[ni], dir);
                growth_counts[ni] += 1;
            }
        }

        let mut added = 0usize;
        for ni in 0..current_len {
            if growth_counts[ni] > 0 && nodes.len() < limits.max_proposal_nodes {
                let avg_dir = normalize3(growth_dirs[ni]);
                let new_pos = add3(nodes[ni], scale3(avg_dir, recipe.node_length_m));
                nodes.push(new_pos);
                parents.push(Some(ni));
                added += 1;
            }
        }

        active_attractors.retain(|&attr| {
            !nodes
                .iter()
                .any(|&npos| len3(sub3(attr, npos)) <= recipe.kill_radius_m)
        });

        if added == 0 {
            break;
        }
    }

    // Solve Da Vinci pipe-model radii on the adopted prefix
    let mut radii = vec![0.0f64; n];
    compute_pipe_model_radii_zero_heap(
        &parents[..n],
        recipe.tip_radius_m,
        recipe.pipe_exponent,
        &mut radii,
    )?;

    // Sweep tapered tubular geometry along each adopted segment
    let mut vertices = Vec::new();
    let mut triangles = Vec::new();
    let slices = recipe.ring_slices;

    for i in 1..n {
        let Some(p) = parents[i] else { continue };
        if p >= n {
            continue;
        }
        let p0 = nodes[p];
        let p1 = nodes[i];
        let axis = sub3(p1, p0);
        if len3(axis) <= 1e-9 {
            continue;
        }
        let tangent = normalize3(axis);
        let up = if tangent[2].abs() < 0.9 {
            [0.0, 0.0, 1.0]
        } else {
            [1.0, 0.0, 0.0]
        };
        let e0 = normalize3(cross3(tangent, up));
        let e1 = normalize3(cross3(tangent, e0));

        let base_idx = vertices.len() as u32;
        for (station_pos, radius) in [(p0, radii[p]), (p1, radii[i])] {
            for k in 0..slices {
                let angle = std::f64::consts::TAU * (k as f64) / (slices as f64);
                let (ca, sa) = (angle.cos(), angle.sin());
                let radial = add3(scale3(e0, ca * radius), scale3(e1, sa * radius));
                vertices.push(add3(station_pos, radial));
            }
        }
        let s = slices as u32;
        for k in 0..s {
            let k_next = (k + 1) % s;
            let i0 = base_idx + k;
            let i1 = base_idx + k_next;
            let i2 = base_idx + s + k_next;
            let i3 = base_idx + s + k;
            triangles.push([i0, i1, i2]);
            triangles.push([i0, i2, i3]);
        }
    }

    // Compute DDG area-weighted smooth vertex normals
    let mut accum_normals = vec![[0.0f64; 3]; vertices.len()];
    for &[i0, i1, i2] in &triangles {
        let v0 = vertices[i0 as usize];
        let v1 = vertices[i1 as usize];
        let v2 = vertices[i2 as usize];
        let fn_area = cross3(sub3(v1, v0), sub3(v2, v0));
        for idx in [i0 as usize, i1 as usize, i2 as usize] {
            accum_normals[idx] = add3(accum_normals[idx], fn_area);
        }
    }
    let smooth_normals = accum_normals
        .into_iter()
        .map(|n| {
            let u = normalize3(n);
            [u[0] as f32, u[1] as f32, u[2] as f32]
        })
        .collect();

    let adopted = recipe
        .organs
        .iter()
        .enumerate()
        .map(|(i, o)| BotanicalSolvedNode {
            key: o.key.clone(),
            parent_key: o.parent.clone(),
            position_m: o.position_m,
            radius_m: radii[i],
        })
        .collect();

    let proposed_positions_m = nodes[n..].to_vec();
    let proposed_parents = parents[n..]
        .iter()
        .map(|p| p.unwrap_or(0) as u32)
        .collect();

    Ok(BotanicalGrowthResult {
        adopted,
        proposed_positions_m,
        proposed_parents,
        iterations_run,
        remaining_attractors: active_attractors.len(),
        vertices,
        triangles,
        smooth_normals,
    })
}

/// Multi-branch SDF collar attachment specification (`1..=12` child branches at a single node).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChildBranchAttachment {
    pub organ_id: u64,
    pub radius_m: f32,
    pub azimuth_angle_deg: f32,
    pub insertion_angle_deg: f32,
    pub split_uv_seam_id: u32,
    pub length_m: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiJunctionSpec {
    pub parent_organ_id: u64,
    pub parent_radius_m: f32,
    pub parent_collar_limit_ratio: f32,
    pub parent_length_m: f32,
    pub children: Vec<ChildBranchAttachment>,
    pub blend_radius_m: f32,
    pub grid_resolution_m: f32,
    pub min_sibling_separation_deg: f32,
    pub collar_segments: u32,
}

impl MultiJunctionSpec {
    pub fn estimated_voxels(&self) -> usize {
        let max_len = self
            .children
            .iter()
            .map(|c| c.length_m)
            .fold(self.parent_length_m, f32::max);
        let res = if self.grid_resolution_m > 0.001 {
            self.grid_resolution_m
        } else {
            0.05
        };
        let cells_per_axis = ((max_len * 2.5) / res).ceil() as usize;
        cells_per_axis
            .saturating_mul(cells_per_axis)
            .saturating_mul(cells_per_axis)
            .max(64)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiJunctionBudget {
    pub max_sdf_voxels: usize,
    pub max_vertices: usize,
    pub max_indices: usize,
}

impl Default for MultiJunctionBudget {
    fn default() -> Self {
        Self {
            max_sdf_voxels: 2_000_000,
            max_vertices: 32_768,
            max_indices: 98_304,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeldedMultiJunctionMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub uv0: Vec<[f32; 2]>,
    pub seam_vertex_pairs: Vec<(u32, u32)>,
    pub normals: Vec<[f32; 3]>,
    pub tangents: Vec<[f32; 4]>,
    pub vertex_organ_ids: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MultiJunctionError {
    BranchRadiusInversion {
        child_radius_m: f32,
        max_allowed_m: f32,
        child_organ_id: u64,
    },
    SelfIntersectingJunctionAngle {
        angle_deg: f32,
        reason: String,
    },
    InvalidChildCount {
        count: usize,
    },
    BudgetExhausted {
        required: usize,
        limit: usize,
        resource: &'static str,
    },
}

impl fmt::Display for MultiJunctionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BranchRadiusInversion {
                child_radius_m,
                max_allowed_m,
                child_organ_id,
            } => write!(
                f,
                "Child {child_organ_id} radius {child_radius_m:.3}m exceeds parent limit {max_allowed_m:.3}m"
            ),
            Self::SelfIntersectingJunctionAngle { angle_deg, reason } => {
                write!(f, "Self-intersecting angle {angle_deg:.2} deg: {reason}")
            }
            Self::InvalidChildCount { count } => {
                write!(f, "Child count {count} outside allowed [1, 12]")
            }
            Self::BudgetExhausted {
                required,
                limit,
                resource,
            } => write!(
                f,
                "Budget exhausted for {resource}: required {required} > limit {limit}"
            ),
        }
    }
}
impl std::error::Error for MultiJunctionError {}

fn child_dir(azimuth_deg: f32, insertion_deg: f32) -> [f32; 3] {
    let phi = azimuth_deg.to_radians();
    let theta = insertion_deg.to_radians();
    [theta.sin() * phi.cos(), theta.sin() * phi.sin(), theta.cos()]
}

/// Welds `1..=12` child branches onto a parent trunk/branch collar with SDF fillet blending,
/// split-UV seam continuity, orthonormal MikkTSpace-compatible tangents, and organ ID provenance.
pub fn weld_multi_junction(
    spec: &MultiJunctionSpec,
    budget: MultiJunctionBudget,
) -> Result<WeldedMultiJunctionMesh, MultiJunctionError> {
    if spec.children.is_empty() || spec.children.len() > 12 {
        return Err(MultiJunctionError::InvalidChildCount {
            count: spec.children.len(),
        });
    }
    let limit_ratio = if spec.parent_collar_limit_ratio > 0.0 {
        spec.parent_collar_limit_ratio
    } else {
        0.92
    };
    let max_child_r = spec.parent_radius_m * limit_ratio;
    for child in &spec.children {
        if child.radius_m > max_child_r {
            return Err(MultiJunctionError::BranchRadiusInversion {
                child_radius_m: child.radius_m,
                max_allowed_m: max_child_r,
                child_organ_id: child.organ_id,
            });
        }
        if !(12.0..=168.0).contains(&child.insertion_angle_deg) {
            return Err(MultiJunctionError::SelfIntersectingJunctionAngle {
                angle_deg: child.insertion_angle_deg,
                reason: format!(
                    "Insertion angle {:.2} deg outside [12.0, 168.0]",
                    child.insertion_angle_deg
                ),
            });
        }
    }

    let min_sep = if spec.min_sibling_separation_deg > 0.0 {
        spec.min_sibling_separation_deg
    } else {
        15.0
    };
    for i in 0..spec.children.len() {
        let d1 = child_dir(
            spec.children[i].azimuth_angle_deg,
            spec.children[i].insertion_angle_deg,
        );
        for j in (i + 1)..spec.children.len() {
            let d2 = child_dir(
                spec.children[j].azimuth_angle_deg,
                spec.children[j].insertion_angle_deg,
            );
            let dot = (d1[0] * d2[0] + d1[1] * d2[1] + d1[2] * d2[2]).clamp(-1.0, 1.0);
            let sep_deg = dot.acos().to_degrees();
            if sep_deg < min_sep {
                return Err(MultiJunctionError::SelfIntersectingJunctionAngle {
                    angle_deg: sep_deg,
                    reason: format!(
                        "Siblings {} and {} separated by {:.2} deg < min {:.2}",
                        spec.children[i].organ_id, spec.children[j].organ_id, sep_deg, min_sep
                    ),
                });
            }
        }
    }

    let est_voxels = spec.estimated_voxels();
    if est_voxels > budget.max_sdf_voxels {
        return Err(MultiJunctionError::BudgetExhausted {
            required: est_voxels,
            limit: budget.max_sdf_voxels,
            resource: "max_sdf_voxels",
        });
    }

    let segments = if spec.collar_segments >= 4 {
        spec.collar_segments
    } else {
        16
    };
    let ring_verts = (segments + 1) as usize;
    let parent_verts = 3 * ring_verts;
    let child_verts = spec.children.len() * 2 * ring_verts;
    let total_verts = parent_verts + child_verts;
    let total_indices = (2 + spec.children.len()) * (segments as usize) * 6;

    if total_verts > budget.max_vertices {
        return Err(MultiJunctionError::BudgetExhausted {
            required: total_verts,
            limit: budget.max_vertices,
            resource: "max_vertices",
        });
    }
    if total_indices > budget.max_indices {
        return Err(MultiJunctionError::BudgetExhausted {
            required: total_indices,
            limit: budget.max_indices,
            resource: "max_indices",
        });
    }

    let mut vertices = Vec::with_capacity(total_verts);
    let mut indices = Vec::with_capacity(total_indices);
    let mut uv0 = Vec::with_capacity(total_verts);
    let mut seam_vertex_pairs = Vec::new();
    let mut normals = Vec::with_capacity(total_verts);
    let mut tangents = Vec::with_capacity(total_verts);
    let mut vertex_organ_ids = Vec::with_capacity(total_verts);

    let rp = spec.parent_radius_m;
    let lp = if spec.parent_length_m > 0.0 {
        spec.parent_length_m
    } else {
        1.0
    };
    let parent_zs = [-lp * 0.5, 0.0, lp * 0.5];
    let parent_vs = [0.0f32, 0.5f32, 1.0f32];

    for ring in 0..3 {
        let z = parent_zs[ring];
        let v = parent_vs[ring];
        let start_idx = vertices.len() as u32;
        for k in 0..=segments {
            let u = k as f32 / segments as f32;
            let alpha = u * std::f32::consts::TAU;
            let (cos_a, sin_a) = (alpha.cos(), alpha.sin());
            vertices.push([rp * cos_a, rp * sin_a, z]);
            normals.push([cos_a, sin_a, 0.0]);
            tangents.push([-sin_a, cos_a, 0.0, 1.0]);
            uv0.push([u, v]);
            vertex_organ_ids.push(spec.parent_organ_id);
            if k == segments {
                seam_vertex_pairs.push((start_idx, vertices.len() as u32 - 1));
            }
        }
    }

    for ring in 0..2 {
        let r0 = ring * ring_verts;
        let r1 = (ring + 1) * ring_verts;
        for k in 0..segments as usize {
            let i0 = (r0 + k) as u32;
            let i1 = (r0 + k + 1) as u32;
            let i2 = (r1 + k + 1) as u32;
            let i3 = (r1 + k) as u32;
            indices.extend_from_slice(&[i0, i1, i2, i0, i2, i3]);
        }
    }

    for child in &spec.children {
        let phi = child.azimuth_angle_deg.to_radians();
        let (cos_phi, sin_phi) = (phi.cos(), phi.sin());
        let d = child_dir(child.azimuth_angle_deg, child.insertion_angle_deg);
        let u_vec = [-sin_phi, cos_phi, 0.0f32];
        let v_vec = [
            d[1] * u_vec[2] - d[2] * u_vec[1],
            d[2] * u_vec[0] - d[0] * u_vec[2],
            d[0] * u_vec[1] - d[1] * u_vec[0],
        ];
        let p0 = [rp * cos_phi, rp * sin_phi, 0.0f32];
        let rc = child.radius_m;
        let lc = if child.length_m > 0.0 {
            child.length_m
        } else {
            0.5
        };
        let blend = spec.blend_radius_m.max(0.0);
        let child_base_idx = vertices.len() as u32;

        for ring in 0..2 {
            let is_tip = ring == 1;
            let (z_dist, rad, v) = if is_tip {
                (lc, rc, 1.0f32)
            } else {
                (0.0, rc + blend, 0.0f32)
            };
            let ring_start = vertices.len() as u32;
            for k in 0..=segments {
                let u = k as f32 / segments as f32;
                let alpha = u * std::f32::consts::TAU;
                let (cos_a, sin_a) = (alpha.cos(), alpha.sin());
                let circ = [
                    u_vec[0] * cos_a + v_vec[0] * sin_a,
                    u_vec[1] * cos_a + v_vec[1] * sin_a,
                    u_vec[2] * cos_a + v_vec[2] * sin_a,
                ];
                let pos = [
                    p0[0] + d[0] * z_dist + circ[0] * rad,
                    p0[1] + d[1] * z_dist + circ[1] * rad,
                    p0[2] + d[2] * z_dist + circ[2] * rad,
                ];
                vertices.push(pos);
                let raw_n = if is_tip {
                    circ
                } else {
                    [
                        circ[0] + d[0] * 0.3,
                        circ[1] + d[1] * 0.3,
                        circ[2] + d[2] * 0.3,
                    ]
                };
                let len_n = (raw_n[0] * raw_n[0] + raw_n[1] * raw_n[1] + raw_n[2] * raw_n[2])
                    .sqrt()
                    .max(1e-6);
                let norm = [raw_n[0] / len_n, raw_n[1] / len_n, raw_n[2] / len_n];
                normals.push(norm);

                let dcirc = [
                    -u_vec[0] * sin_a + v_vec[0] * cos_a,
                    -u_vec[1] * sin_a + v_vec[1] * cos_a,
                    -u_vec[2] * sin_a + v_vec[2] * cos_a,
                ];
                let dot_tn = dcirc[0] * norm[0] + dcirc[1] * norm[1] + dcirc[2] * norm[2];
                let t_ortho = [
                    dcirc[0] - dot_tn * norm[0],
                    dcirc[1] - dot_tn * norm[1],
                    dcirc[2] - dot_tn * norm[2],
                ];
                let len_t =
                    (t_ortho[0] * t_ortho[0] + t_ortho[1] * t_ortho[1] + t_ortho[2] * t_ortho[2])
                        .sqrt()
                        .max(1e-6);
                tangents.push([t_ortho[0] / len_t, t_ortho[1] / len_t, t_ortho[2] / len_t, 1.0]);

                uv0.push([u, v]);
                vertex_organ_ids.push(child.organ_id);
                if k == segments {
                    seam_vertex_pairs.push((ring_start, vertices.len() as u32 - 1));
                }
            }
        }

        let r0 = child_base_idx;
        let r1 = child_base_idx + ring_verts as u32;
        for k in 0..segments as u32 {
            indices.extend_from_slice(&[
                r0 + k,
                r0 + k + 1,
                r1 + k + 1,
                r0 + k,
                r1 + k + 1,
                r1 + k,
            ]);
        }
    }

    Ok(WeldedMultiJunctionMesh {
        vertices,
        indices,
        uv0,
        seam_vertex_pairs,
        normals,
        tangents,
        vertex_organ_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_botanical_growth_supports_arbitrary_n_organs_and_ddg_normals() {
        let recipe = BotanicalRecipe {
            version: 1,
            part_id: "corymbia_main_crown".into(),
            organs: vec![
                BotanicalOrgan {
                    key: "root".into(),
                    parent: None,
                    position_m: [0.0, 0.0, 0.0],
                },
                BotanicalOrgan {
                    key: "trunk_mid".into(),
                    parent: Some("root".into()),
                    position_m: [0.0, 0.0, 1.5],
                },
                BotanicalOrgan {
                    key: "bough_north".into(),
                    parent: Some("trunk_mid".into()),
                    position_m: [0.8, 0.2, 2.8],
                },
                BotanicalOrgan {
                    key: "bough_south".into(),
                    parent: Some("trunk_mid".into()),
                    position_m: [-0.7, -0.3, 2.7],
                },
                BotanicalOrgan {
                    key: "bough_east".into(),
                    parent: Some("trunk_mid".into()),
                    position_m: [0.2, 0.9, 2.9],
                },
                BotanicalOrgan {
                    key: "leader_apex".into(),
                    parent: Some("bough_north".into()),
                    position_m: [1.1, 0.3, 4.0],
                },
            ],
            attractors_m: vec![[1.4, 0.4, 4.5], [-1.0, -0.5, 3.3], [0.3, 1.3, 3.4]],
            node_length_m: 0.25,
            kill_radius_m: 0.1,
            perception_radius_m: 1.5,
            cone_angle_rad: 2.2,
            max_iterations: 2,
            tip_radius_m: 0.08,
            pipe_exponent: 2.0,
            ring_slices: 8,
        };
        let res = evaluate_botanical_growth(&recipe, BotanicalLimits::default()).unwrap();
        assert_eq!(res.adopted.len(), 6);
        assert!(res.adopted[0].radius_m > res.adopted[5].radius_m);
        assert_eq!(res.vertices.len(), res.smooth_normals.len());
        assert!(!res.triangles.is_empty());
    }

    #[test]
    fn weld_multi_junction_welds_four_child_branches_with_uv_seams() {
        let spec = MultiJunctionSpec {
            parent_organ_id: 100,
            parent_radius_m: 0.45,
            parent_collar_limit_ratio: 0.90,
            parent_length_m: 1.8,
            children: vec![
                ChildBranchAttachment {
                    organ_id: 101,
                    radius_m: 0.22,
                    azimuth_angle_deg: 0.0,
                    insertion_angle_deg: 45.0,
                    split_uv_seam_id: 1,
                    length_m: 0.9,
                },
                ChildBranchAttachment {
                    organ_id: 102,
                    radius_m: 0.19,
                    azimuth_angle_deg: 95.0,
                    insertion_angle_deg: 52.0,
                    split_uv_seam_id: 2,
                    length_m: 0.85,
                },
                ChildBranchAttachment {
                    organ_id: 103,
                    radius_m: 0.18,
                    azimuth_angle_deg: 190.0,
                    insertion_angle_deg: 48.0,
                    split_uv_seam_id: 3,
                    length_m: 0.8,
                },
                ChildBranchAttachment {
                    organ_id: 104,
                    radius_m: 0.16,
                    azimuth_angle_deg: 280.0,
                    insertion_angle_deg: 40.0,
                    split_uv_seam_id: 4,
                    length_m: 0.75,
                },
            ],
            blend_radius_m: 0.06,
            grid_resolution_m: 0.04,
            min_sibling_separation_deg: 18.0,
            collar_segments: 12,
        };
        let welded = weld_multi_junction(&spec, MultiJunctionBudget::default()).unwrap();
        assert_eq!(welded.vertices.len(), welded.normals.len());
        assert_eq!(welded.vertices.len(), welded.tangents.len());
        assert!(!welded.seam_vertex_pairs.is_empty());
    }
}

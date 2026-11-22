//! Bounded fabrication analysis, lattice geometry, and print artifact adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

mod bed;
mod cam_preview;
mod flux_network;
mod joinery;
mod lattice;
mod metrology;
mod package;
mod slice;
mod thermal;

use crate::{
    artifact::AttributedMesh, fields, num, required, text, vector, Args, AssetHost, TypedArtifact,
    Value,
};
use qualia_core_db::{
    render::assets::{ImportedMesh, Mesh},
    specialized_libs::computational_geometry::{
        ray_triangle_intersect_3d, self_intersecting_pairs,
        surface_topology_qualification::{
            qualify_surface_topology_budgeted_into, surface_qualification_storage_bounds,
        },
        EdgeSlot, HalfEdge, Point3, RayTriangleResult, TriPair, TriTriError,
    },
};
use serde_json::json;
use std::{collections::BTreeSet, mem::size_of, sync::Arc};

const MAX_MESH_VERTICES: usize = 200_000;
const MAX_MESH_FACES: usize = 400_000;
const MAX_DFM_FACES: usize = 2048;

impl AssetHost<'_> {
    pub(crate) fn certify_dfm_manifold(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "process",
                "minimum_wall_mm",
                "max_overhang_deg",
                "build_envelope_mm",
            ],
        )?;
        let process = text(required(args, "process")?)?;
        if !matches!(process, "fff" | "sla" | "sls") {
            return Err("DFM process must be fff, sla, or sls".into());
        }
        let minimum_wall_mm = bounded_number(args, "minimum_wall_mm", None, 0.01, 1000.0)?;
        let max_overhang_deg = bounded_number(args, "max_overhang_deg", None, 0.0, 90.0)?;
        let envelope = positive_vector(args, "build_envelope_mm", 0.01, 10_000.0)?;
        let mesh = mesh_handle(self, required(args, "mesh")?)?;
        let face_count = mesh.surface.mesh.triangles.len();
        if face_count > MAX_DFM_FACES {
            return Err(format!(
                "DFM analysis supports at most {MAX_DFM_FACES} faces"
            ));
        }
        let vertex_count = mesh.surface.mesh.positions.len();
        let (min, max) = bounds(&mesh.surface.mesh.positions);
        let extent_mm = std::array::from_fn::<_, 3, _>(|i| f64::from(max[i] - min[i]) * 1000.0);
        let envelope_passed = (0..3).all(|i| extent_mm[i] <= envelope[i]);
        let capacity = surface_qualification_storage_bounds(vertex_count as u32, face_count)
            .map_err(|error| format!("DFM topology scratch: {error:?}"))?;
        let pairs = face_count * face_count.saturating_sub(1) / 2;
        let mut topo_work = capacity.fixed_work + capacity.half_edges * 256;
        self.charge(
            topo_work + face_count * face_count * 24,
            pairs * size_of::<TriPair>() + face_count * 1024 + vertex_count * 36,
        )?;

        let triangles = &mesh.surface.mesh.triangles;
        let positions = &mesh.surface.mesh.positions;
        let topology = qualify_surface_topology_budgeted_into(
            vertex_count as u32,
            triangles,
            &mut topo_work,
            &mut vec![HalfEdge::default(); capacity.half_edges],
            &mut vec![EdgeSlot::default(); capacity.edge_slots],
            &mut vec![0; capacity.vertex_starts],
            &mut vec![false; capacity.visited_half_edges],
            &mut vec![0; capacity.face_labels],
            &mut vec![0; capacity.face_queue],
        );
        let vertices: Vec<_> = positions.iter().map(|p| p.map(f64::from)).collect();
        let points: Vec<_> = vertices.iter().copied().map(point).collect();
        let mut edge_set = BTreeSet::new();
        let mut degenerate = Vec::new();
        let mut overhangs = Vec::new();
        let mut faces = Vec::with_capacity(face_count);
        let mut volume = 0.0;
        let origin = vertices[0];
        let scale = extent_mm.into_iter().fold(0.0_f64, f64::max) / 1000.0;
        let floor_tol = 1e-9 * scale.max(0.001);
        for (i, tri) in triangles.iter().enumerate() {
            let [a, b, c] = tri.map(|id| vertices[id as usize]);
            for [left, right] in [[tri[0], tri[1]], [tri[1], tri[2]], [tri[2], tri[0]]] {
                edge_set.insert((left.min(right), left.max(right)));
            }
            let area_vector = cross(sub(b, a), sub(c, a));
            let area = norm(area_vector);
            let normal = if area > 0.0 {
                mul(area_vector, area.recip())
            } else {
                [0.0; 3]
            };
            if area == 0.0 {
                degenerate.push(i);
            }
            let touching_floor = [a, b, c]
                .iter()
                .all(|v| v[2] - f64::from(min[2]) <= floor_tol);
            if !touching_floor
                && normal[2] < 0.0
                && (-normal[2]).clamp(0.0, 1.0).asin().to_degrees() > max_overhang_deg
            {
                overhangs.push(i);
            }
            volume += dot(sub(a, origin), cross(sub(b, origin), sub(c, origin))) / 6.0;
            faces.push((mul(add(add(a, b), c), 1.0 / 3.0), normal));
        }
        let mut conflicts = vec![TriPair { a: 0, b: 0 }; face_count.min(128)];
        let pair_result = if degenerate.is_empty() {
            match self_intersecting_pairs(&points, triangles, &mut conflicts) {
                Ok(count) => {
                    conflicts.truncate(count);
                    Some((
                        count,
                        Some(conflicts.iter().map(|t| [t.a, t.b]).collect::<Vec<_>>()),
                    ))
                }
                Err(TriTriError::OutputTooSmall { required }) => Some((required, None)),
                Err(error) => return Err(format!("DFM self-intersection audit: {error:?}")),
            }
        } else {
            None
        };
        let solid_candidate = topology.is_ok()
            && volume > 0.0
            && pair_result.as_ref().is_some_and(|(count, _)| *count == 0);
        let (sample_min, missing_probes) = if solid_candidate {
            thickness_probes(&faces, triangles, &points, floor_tol, self.cancel)?
        } else {
            (None, face_count)
        };
        let sample_passed = sample_min.is_some_and(|v| v >= minimum_wall_mm) && missing_probes == 0;
        let report = json!({
            "schema": "qualia.dfm-indexed-surface-audit/v1",
            "process": process,
            "certified": false,
            "indexed_closed_connected_oriented_vertex_manifold": topology.is_ok(),
            "topology_error": topology.err().map(|v| format!("{v:?}")),
            "euler_characteristic": vertex_count as i64 - edge_set.len() as i64 + face_count as i64,
            "signed_volume_mm3": volume * 1e9,
            "degenerate_faces": degenerate,
            "disjoint_vertex_intersection_count": pair_result.as_ref().map(|p| p.0),
            "disjoint_vertex_intersection_pairs": pair_result.and_then(|p| p.1),
            "minimum_wall_mm_requested": minimum_wall_mm,
            "minimum_inward_centroid_ray_mm": sample_min,
            "missing_wall_probes": missing_probes,
            "sampled_wall_gate_passed": sample_passed,
            "trapped_cavity_count": null,
            "overhang_faces": overhangs,
            "max_overhang_deg_from_vertical": max_overhang_deg,
            "dimensions_mm": extent_mm,
            "build_envelope_mm": envelope,
            "build_envelope_passed": envelope_passed,
            "scoped_geometry_checks_passed": solid_candidate && envelope_passed && sample_passed && overhangs.is_empty(),
        });
        let receipt = self.admit_encoded(
            "json",
            serde_json::to_vec(&report).map_err(|e| e.to_string())?,
        )?;
        Ok(Value::Record([("receipt".into(), receipt)].into()))
    }
}

fn thickness_probes(
    faces: &[([f64; 3], [f64; 3])],
    triangles: &[[u32; 3]],
    vertices: &[Point3],
    epsilon: f64,
    cancel: &std::sync::atomic::AtomicBool,
) -> Result<(Option<f64>, usize), String> {
    let mut min = f64::INFINITY;
    let mut missing = 0;
    for (i, &(center, normal)) in faces.iter().enumerate() {
        if cancel.load(std::sync::atomic::Ordering::Relaxed) {
            return Err("Production cancelled".into());
        }
        let mut nearest = f64::INFINITY;
        let direction = point(mul(normal, -1.0));
        for (j, &tri) in triangles.iter().enumerate() {
            if i != j {
                let [a, b, c] = tri.map(|id| vertices[id as usize]);
                if let RayTriangleResult::Hit(hit) =
                    ray_triangle_intersect_3d(point(center), direction, a, b, c)
                {
                    if hit.t > epsilon && hit.t < nearest {
                        nearest = hit.t;
                    }
                }
            }
        }
        if nearest.is_finite() {
            min = min.min(nearest * 1000.0);
        } else {
            missing += 1;
        }
    }
    Ok((min.is_finite().then_some(min), missing))
}

fn check_closed_shell(host: &mut AssetHost<'_>, mesh: &Mesh) -> Result<(), String> {
    let capacity =
        surface_qualification_storage_bounds(mesh.positions.len() as u32, mesh.triangles.len())
            .map_err(|error| format!("Closed mesh scratch: {error:?}"))?;
    let mut budget = capacity.fixed_work + capacity.half_edges * 256;
    host.charge(
        budget,
        capacity.half_edges * size_of::<HalfEdge>()
            + capacity.edge_slots * size_of::<EdgeSlot>()
            + capacity.vertex_starts * 4
            + capacity.visited_half_edges
            + 4 * (capacity.face_labels + capacity.face_queue),
    )?;
    qualify_surface_topology_budgeted_into(
        mesh.positions.len() as u32,
        &mesh.triangles,
        &mut budget,
        &mut vec![HalfEdge::default(); capacity.half_edges],
        &mut vec![EdgeSlot::default(); capacity.edge_slots],
        &mut vec![0; capacity.vertex_starts],
        &mut vec![false; capacity.visited_half_edges],
        &mut vec![0; capacity.face_labels],
        &mut vec![0; capacity.face_queue],
    )
    .map_err(|error| format!("Expected one closed indexed vertex-manifold surface: {error:?}"))?;
    Ok(())
}

fn mesh_handle(host: &AssetHost<'_>, handle: &Value) -> Result<Arc<AttributedMesh>, String> {
    let TypedArtifact::AttributedMesh(mesh) = host.workspace.typed(handle)? else {
        return Err("Fabrication requires an AttributedMesh handle".into());
    };
    let surface = &mesh.surface.mesh;
    if surface.positions.is_empty()
        || surface.triangles.is_empty()
        || surface.positions.len() > MAX_MESH_VERTICES
        || surface.triangles.len() > MAX_MESH_FACES
        || surface
            .positions
            .iter()
            .flatten()
            .any(|v| !v.is_finite() || v.abs() > 100.0)
        || surface
            .triangles
            .iter()
            .flatten()
            .any(|&id| id as usize >= surface.positions.len())
        || (!mesh.protected_faces.is_empty()
            && mesh.protected_faces.len() != surface.triangles.len())
    {
        return Err("Fabrication needs bounded metre coordinates, valid indexed faces, and aligned protection masks".into());
    }
    Ok(Arc::clone(mesh))
}

fn mesh_output(
    host: &mut AssetHost<'_>,
    positions: Vec<[f32; 3]>,
    triangles: Vec<[u32; 3]>,
) -> Result<Value, String> {
    let (min, max) = bounds(&positions);
    let imported = ImportedMesh {
        mesh: Mesh {
            positions,
            triangles,
            min,
            max,
        },
        normals: None,
        tangents: None,
        texture_coordinates_0: None,
        materials: Vec::new(),
        texture_dependencies: Vec::new(),
        submeshes: Vec::new(),
    };
    host.admit_artifact(TypedArtifact::AttributedMesh(Arc::new(AttributedMesh {
        surface: imported,
        uv1: Vec::new(),
        vertex_colors: Vec::new(),
        skin_weights: Vec::new(),
        morph_targets: Arc::from(Vec::new()),
        protected_faces: Vec::new(),
    })))
}

fn bounded_number(
    args: &Args,
    key: &str,
    default: Option<f64>,
    min: f64,
    max: f64,
) -> Result<f64, String> {
    let value = args
        .get(key)
        .map(num)
        .transpose()?
        .or(default)
        .ok_or_else(|| format!("Missing {key}"))?;
    if !(min..=max).contains(&value) {
        return Err(format!("{key} must be between {min} and {max}"));
    }
    Ok(value)
}

fn positive_vector(args: &Args, key: &str, min: f64, max: f64) -> Result<[f64; 3], String> {
    let values = vector::<3>(required(args, key)?)?;
    if values.iter().any(|x| !(min..=max).contains(x)) {
        return Err(format!("{key} components must be between {min} and {max}"));
    }
    Ok(values)
}

fn bounds(positions: &[[f32; 3]]) -> ([f32; 3], [f32; 3]) {
    positions.iter().fold(
        ([f32::INFINITY; 3], [f32::NEG_INFINITY; 3]),
        |(min, max), p| {
            (
                std::array::from_fn(|axis| min[axis].min(p[axis])),
                std::array::from_fn(|axis| max[axis].max(p[axis])),
            )
        },
    )
}

fn point(v: [f64; 3]) -> Point3 {
    Point3::new(v[0], v[1], v[2])
}
fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] - b[i])
}
fn add(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}
fn mul(a: [f64; 3], s: f64) -> [f64; 3] {
    a.map(|v| v * s)
}
fn cross([a, b, c]: [f64; 3], [x, y, z]: [f64; 3]) -> [f64; 3] {
    [b * z - c * y, c * x - a * z, a * y - b * x]
}
fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn norm(a: [f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

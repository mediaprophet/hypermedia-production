//! Bounded authored geometry operations and their native production adapters.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use crate::artifact::AttributedMesh;
use crate::{
    count, fields, integer, list, num, number, record, required, text, vector, Args, AssetHost,
    TypedArtifact, Value,
};
use std::collections::{BTreeMap, BTreeSet};

mod cluster;
mod faceted_step;
mod mesh_brush;
mod voronoi;

const MAX_TRIANGLES: usize = 120_000;
const MAX_VERTICES: usize = 160_000;

type Point = [f64; 3];
type Edge = (u32, u32);

impl AssetHost<'_> {
    pub(crate) fn retopologize_quad(&mut self, args: &Args) -> Result<Value, String> {
        fields(
            args,
            &[
                "mesh",
                "target_quad_count",
                "guide_edges",
                "max_dihedral_degrees",
            ],
        )?;
        let handle = required(args, "mesh")?;
        let source = source_mesh(self, handle)?;
        let mesh = &source.surface.mesh;
        let vertex_count = mesh.positions.len();
        let triangle_count = mesh.triangles.len();
        let target = integer(required(args, "target_quad_count")?, "target_quad_count")?;
        let max_dihedral = number(args, "max_dihedral_degrees", 10.0)?;
        if !(1..=triangle_count / 2).contains(&target) || !(0.0..=45.0).contains(&max_dihedral) {
            return Err(
                "Quad target or maximum dihedral angle is outside the admitted range".into(),
            );
        }
        let guide_edges = if let Some(value) = args.get("guide_edges") {
            let rows = list(value)?;
            if rows.len() > triangle_count.saturating_mul(3) {
                return Err("Too many authored quad guide edges".into());
            }
            rows.iter()
                .map(|link| {
                    let [a, b] = list(link)? else {
                        return Err("Quad guide edges need exactly two vertex indices".into());
                    };
                    let a = integer(a, "guide vertex index")?;
                    let b = integer(b, "guide vertex index")?;
                    if a == b || a >= vertex_count || b >= vertex_count {
                        return Err(
                            "Quad guide edge does not reference two distinct mesh vertices".into(),
                        );
                    }
                    Ok(sorted_edge(a as u32, b as u32))
                })
                .collect::<Result<BTreeSet<_>, String>>()?
        } else {
            BTreeSet::new()
        };
        self.charge(
            triangle_count.saturating_mul(700),
            triangle_count.saturating_mul(384),
        )?;
        let source = source_mesh(self, handle)?;
        let mesh = &source.surface.mesh;
        let material = material_slots(source)?;
        let mut edges = BTreeMap::<Edge, Vec<DirectedEdge>>::new();
        let mut normals = Vec::with_capacity(triangle_count);
        for (face, &triangle) in mesh.triangles.iter().enumerate() {
            let p = triangle.map(|id| point(mesh.positions[id as usize]));
            normals.push(face_normal(p)?);
            for i in 0..3 {
                let [from, to, opposite] = std::array::from_fn(|offset| triangle[(i + offset) % 3]);
                let edge = edges.entry(sorted_edge(from, to)).or_default();
                if edge.len() == 2 {
                    return Err("Quad pairing requires manifold triangle edges".into());
                }
                edge.push(DirectedEdge {
                    face,
                    from,
                    to,
                    opposite,
                });
            }
        }
        if guide_edges.iter().any(|edge| !edges.contains_key(edge)) {
            return Err("An authored quad guide edge does not exist in the mesh".into());
        }
        let cosine = max_dihedral.to_radians().cos();
        let mut candidate = Vec::with_capacity(edges.len());
        for (key, incident) in &edges {
            let [a, b] = incident.as_slice() else {
                continue;
            };
            if a.from != b.to || a.to != b.from {
                return Err(
                    "Quad pairing requires consistent face winding across shared edges".into(),
                );
            }
            if guide_edges.contains(key)
                || material[a.face] != material[b.face]
                || protected(source, a.face)
                || protected(source, b.face)
                || dot(normals[a.face], normals[b.face]) + 1e-12 < cosine
                || a.opposite == b.opposite
            {
                continue;
            }
            // Triangle A is (from,to,opposite A) and B is (to,from,opposite B).
            let quad = [a.to, a.opposite, a.from, b.opposite];
            let p = quad.map(|vertex| point(mesh.positions[vertex as usize]));
            let plane = add(normals[a.face], normals[b.face]);
            let mut score = 0.0;
            let mut convex = true;
            for i in 0..4 {
                let before = sub(p[(i + 3) % 4], p[i]);
                let after = sub(p[(i + 1) % 4], p[i]);
                let corner = dot(cross(after, before), plane);
                if corner <= 1e-15 {
                    convex = false;
                    break;
                }
                score += (dot(before, after) / (length(before) * length(after))).abs();
            }
            if convex {
                candidate.push((score, a.face, b.face, quad));
            }
        }
        candidate.sort_unstable_by(|a, b| {
            a.0.total_cmp(&b.0)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.2.cmp(&b.2))
        });
        let mut paired = vec![false; triangle_count];
        let mut quads = Vec::with_capacity(target);
        for (_, a, b, corners) in candidate {
            if paired[a] || paired[b] {
                continue;
            }
            paired[a] = true;
            paired[b] = true;
            quads.push(Value::Record(BTreeMap::from([
                ("vertices".into(), indices(corners)),
                ("source_triangles".into(), indices([a as u32, b as u32])),
            ])));
            if quads.len() == target {
                break;
            }
        }
        if quads.len() != target {
            return Err(format!(
                "Requested {target} quads, but the admitted pairing produced {} with these guides, materials and dihedral limit",
                quads.len()
            ));
        }
        let remainder = paired
            .iter()
            .enumerate()
            .filter(|&(_, &pair)| !pair)
            .map(|(idx, _)| Value::U64(idx as u64))
            .collect();
        Ok(Value::Record(BTreeMap::from([
            ("mesh".into(), handle.clone()),
            ("quads".into(), Value::List(quads)),
            ("remaining_triangle_indices".into(), Value::List(remainder)),
            (
                "quad_fraction".into(),
                Value::F64(target as f64 / (triangle_count - target) as f64),
            ),
        ])))
    }
}

struct DirectedEdge {
    face: usize,
    from: u32,
    to: u32,
    opposite: u32,
}

fn source_mesh<'a>(host: &'a AssetHost<'_>, handle: &Value) -> Result<&'a AttributedMesh, String> {
    let TypedArtifact::AttributedMesh(mesh) = host.workspace.typed(handle)? else {
        return Err("Expected an attributed mesh handle".into());
    };
    let surface = &mesh.surface.mesh;
    let vertices = surface.positions.len();
    let triangles = surface.triangles.len();
    let masks = mesh.protected_faces.len();
    if !(3..=MAX_VERTICES).contains(&vertices)
        || !(1..=MAX_TRIANGLES).contains(&triangles)
        || (masks != 0 && masks != triangles)
        || surface
            .positions
            .iter()
            .flatten()
            .any(|x| !x.is_finite() || x.abs() > 1e6)
        || surface
            .triangles
            .iter()
            .flatten()
            .any(|&id| id as usize >= vertices)
    {
        return Err(
            "Attributed mesh has invalid positions, indices, face alignment or size".into(),
        );
    }
    Ok(mesh)
}

fn material_slots(mesh: &AttributedMesh) -> Result<Vec<Option<(u64, u64)>>, String> {
    let mut slots = vec![None; mesh.surface.mesh.triangles.len()];
    let mut prior_end = 0;
    for range in &mesh.surface.submeshes {
        let start = range.first_index as usize;
        let count = range.index_count as usize;
        let end = start.checked_add(count).ok_or("Submesh range overflow")?;
        if start < prior_end
            || start % 3 != 0
            || count == 0
            || count % 3 != 0
            || end / 3 > slots.len()
        {
            return Err(
                "Attributed submesh ranges must be ordered, disjoint and triangle-aligned".into(),
            );
        }
        slots[start / 3..end / 3].fill(Some((range.material_id, range.semantic_id)));
        prior_end = end;
    }
    Ok(slots)
}

fn protected(mesh: &AttributedMesh, face: usize) -> bool {
    mesh.protected_faces
        .get(face)
        .is_some_and(|&mask| mask != 0)
}

fn indices<const N: usize>(values: [u32; N]) -> Value {
    Value::List(
        values
            .into_iter()
            .map(|id| Value::U64(u64::from(id)))
            .collect(),
    )
}

fn sorted_edge(a: u32, b: u32) -> Edge {
    (a.min(b), a.max(b))
}
fn point(p: [f32; 3]) -> Point {
    p.map(f64::from)
}
fn dot(a: Point, b: Point) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: Point, b: Point) -> Point {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn sub(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] - b[i])
}
fn add(a: Point, b: Point) -> Point {
    std::array::from_fn(|i| a[i] + b[i])
}
fn mul(a: Point, s: f64) -> Point {
    a.map(|x| x * s)
}
fn length(a: Point) -> f64 {
    dot(a, a).sqrt()
}

fn normalized(v: Point) -> Result<Point, String> {
    let norm = length(v);
    if norm.is_finite() && norm > 1e-15 {
        Ok(mul(v, norm.recip()))
    } else {
        Err("Expected a nondegenerate finite 3D direction".into())
    }
}

fn face_normal([a, b, c]: [Point; 3]) -> Result<Point, String> {
    normalized(cross(sub(b, a), sub(c, a)))
}

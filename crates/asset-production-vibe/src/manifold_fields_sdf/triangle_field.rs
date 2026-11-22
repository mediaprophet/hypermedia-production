//! BVH-pruned exact triangle-distance and parity queries for SDF authoring.

use qualia_core_db::render::assets::Mesh;
use qualia_core_db::specialized_libs::computational_geometry::{
    bvh::{build_bvh_recursive, BvhNode, MAX_BVH_DEPTH},
    distance::{point_triangle_distance_sq_3d, ray_triangle_intersect_3d, Aabb, RayTriangleResult},
    Point3,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// The BVH build also holds temporary bounds, Morton keys, and centroids.
pub(super) const SCRATCH_PER_TRIANGLE: usize = 256;

pub(super) struct SamplingBudget<'a> {
    pub(super) remaining: usize,
    pub(super) cancel: &'a AtomicBool,
}

impl SamplingBudget<'_> {
    fn step(&mut self) -> Result<(), String> {
        self.remaining = self
            .remaining
            .checked_sub(1)
            .ok_or("SDF query work budget exhausted")?;
        Ok(())
    }
}

pub(super) struct TriangleField<'a> {
    mesh: &'a Mesh,
    nodes: Vec<BvhNode>,
    indices: Vec<u32>,
}

fn point(coords: [f32; 3]) -> Point3 {
    Point3::new(
        f64::from(coords[0]),
        f64::from(coords[1]),
        f64::from(coords[2]),
    )
}

fn bounds(node: &BvhNode) -> Aabb {
    Aabb::new(point(node.bbox_min), point(node.bbox_max))
}

impl<'a> TriangleField<'a> {
    pub(super) fn build(mesh: &'a Mesh) -> Result<Self, String> {
        if mesh.positions.iter().flatten().any(|v| !v.is_finite()) {
            return Err("SDF source contains non-finite vertices".into());
        }
        let n = mesh.triangles.len();
        let mut triangle_bounds = Vec::with_capacity(n);
        for triangle in &mesh.triangles {
            let [a, b, c] = *triangle;
            let a = *mesh
                .positions
                .get(a as usize)
                .ok_or("SDF triangle index out of bounds")?;
            let b = *mesh
                .positions
                .get(b as usize)
                .ok_or("SDF triangle index out of bounds")?;
            let c = *mesh
                .positions
                .get(c as usize)
                .ok_or("SDF triangle index out of bounds")?;
            triangle_bounds.push(Aabb::new(
                point(std::array::from_fn(|axis| {
                    a[axis].min(b[axis]).min(c[axis])
                })),
                point(std::array::from_fn(|axis| {
                    a[axis].max(b[axis]).max(c[axis])
                })),
            ));
        }
        let mut nodes = vec![BvhNode::default(); n.saturating_mul(2).saturating_sub(1)];
        let mut indices = vec![0; n];
        let mut codes = vec![0; n];
        let mut sort_indices = vec![0; n];
        let (count, _) = build_bvh_recursive(
            &triangle_bounds,
            &mut nodes,
            &mut indices,
            &mut codes,
            &mut sort_indices,
        )
        .map_err(|error| format!("SDF BVH build: {error:?}"))?;
        nodes.truncate(count);
        Ok(Self {
            mesh,
            nodes,
            indices,
        })
    }

    fn triangle(&self, index: u32) -> [Point3; 3] {
        self.mesh.triangles[index as usize].map(|i| point(self.mesh.positions[i as usize]))
    }

    /// Query work is charged on every visited node and evaluated primitive.
    pub(super) fn distance(
        &self,
        coords: [f32; 3],
        budget: &mut SamplingBudget<'_>,
    ) -> Result<f32, String> {
        if budget.cancel.load(Ordering::Relaxed) {
            return Err("SDF sampling cancelled".into());
        }
        if self.nodes.is_empty() {
            return Ok(1.0);
        }
        let p = point(coords);
        let mut best = f64::INFINITY;
        let mut stack = BvhStack::new();
        while let Some(index) = stack.pop() {
            budget.step()?;
            let node = &self.nodes[index as usize];
            if bounds(node).distance_sq_to_point(p) >= best {
                continue;
            }
            if node.node_type == 1 {
                let start = node.left_or_first as usize;
                let end = start + node.right_or_count as usize;
                for &primitive in &self.indices[start..end] {
                    budget.step()?;
                    let [a, b, c] = self.triangle(primitive);
                    best = best.min(point_triangle_distance_sq_3d(p, a, b, c));
                }
            } else {
                let [a, b] = [node.left_or_first, node.right_or_count];
                let left = bounds(&self.nodes[a as usize]).distance_sq_to_point(p);
                let right = bounds(&self.nodes[b as usize]).distance_sq_to_point(p);
                let [near, far] = if left <= right { [a, b] } else { [b, a] };
                stack.push(far)?;
                stack.push(near)?;
            }
        }
        let magnitude = best.sqrt() as f32;
        if !magnitude.is_finite() {
            return Err("SDF sampling produced a non-finite distance".into());
        }
        if magnitude == 0.0 || !bounds(&self.nodes[0]).contains_point(p) {
            return Ok(magnitude);
        }
        for direction in [
            [1.0, 0.173_205_080_756_887_73, 0.271_828_182_845_904_5],
            [0.137_564_728, -1.0, 0.314_159_265_358_979_3],
            [-0.161_803_398_874_989_5, 0.223_606_797_749_978_96, 1.0],
        ] {
            if let Some(inside) = self.parity(p, direction, budget)? {
                return Ok(if inside { -magnitude } else { magnitude });
            }
        }
        Err("SDF sign is ambiguous at a grazing or coincident mesh feature".into())
    }

    fn parity(
        &self,
        p: Point3,
        ray: [f64; 3],
        budget: &mut SamplingBudget<'_>,
    ) -> Result<Option<bool>, String> {
        let direction = Point3::new(ray[0], ray[1], ray[2]);
        let reciprocal = ray.map(f64::recip);
        let mut inside = false;
        let mut stack = BvhStack::new();
        while let Some(index) = stack.pop() {
            budget.step()?;
            let node = &self.nodes[index as usize];
            if !ray_intersects_bounds(node, p, reciprocal) {
                continue;
            }
            if node.node_type != 1 {
                stack.push(node.left_or_first)?;
                stack.push(node.right_or_count)?;
                continue;
            }
            let start = node.left_or_first as usize;
            let end = start + node.right_or_count as usize;
            for &primitive in &self.indices[start..end] {
                budget.step()?;
                let [a, b, c] = self.triangle(primitive);
                if let RayTriangleResult::Hit(hit) =
                    ray_triangle_intersect_3d(p, direction, a, b, c)
                {
                    if hit.t <= 0.0 {
                        continue;
                    }
                    // Retry with another direction instead of counting a shared edge twice.
                    const EDGE_EPSILON: f64 = 64.0 * f64::EPSILON;
                    if hit.u <= EDGE_EPSILON
                        || hit.v <= EDGE_EPSILON
                        || 1.0 - hit.u - hit.v <= EDGE_EPSILON
                    {
                        return Ok(None);
                    }
                    inside = !inside;
                }
            }
        }
        Ok(Some(inside))
    }
}

fn ray_intersects_bounds(node: &BvhNode, p: Point3, reciprocal: [f64; 3]) -> bool {
    let origin = [p.x, p.y, p.z];
    let mut enter = 0.0_f64;
    let mut exit = f64::INFINITY;
    for axis in 0..3 {
        let a = (f64::from(node.bbox_min[axis]) - origin[axis]) * reciprocal[axis];
        let b = (f64::from(node.bbox_max[axis]) - origin[axis]) * reciprocal[axis];
        enter = enter.max(a.min(b));
        exit = exit.min(a.max(b));
    }
    enter <= exit
}

struct BvhStack {
    nodes: [u32; MAX_BVH_DEPTH * 2],
    len: usize,
}

impl BvhStack {
    fn new() -> Self {
        Self {
            nodes: [0; MAX_BVH_DEPTH * 2],
            len: 1,
        }
    }

    fn push(&mut self, node: u32) -> Result<(), String> {
        *self
            .nodes
            .get_mut(self.len)
            .ok_or("SDF BVH depth exceeds traversal bound")? = node;
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<u32> {
        self.len = self.len.checked_sub(1)?;
        Some(self.nodes[self.len])
    }
}

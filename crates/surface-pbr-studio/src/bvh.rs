//! Bounding Volume Hierarchy (BVH) for deterministic, fast ray-mesh queries.
//! Stack-based non-recursive traversal prevents stack overflow on degenerate meshes.

use crate::math::{Bounds, Hit, Ray, Vec2, Vec3, ray_triangle};
use crate::mesh::Mesh;

const MAX_DEPTH: u32 = 32;
const MAX_LEAF: usize = 4;

#[derive(Clone, Copy, Debug)]
struct Node {
    min: Vec3,
    max: Vec3,
    first: u32,
    count: u32,
    right: u32,
}

/// A binary BVH over a mesh's triangle indices.
#[derive(Clone, Debug)]
pub struct Bvh {
    nodes: Vec<Node>,
    order: Vec<u32>,
    bounds: Bounds,
}

impl Bvh {
    /// Build a BVH for a mesh. An empty mesh yields an empty tree.
    pub fn build(mesh: &Mesh) -> Self {
        let triangle_count = mesh.triangle_count();
        if triangle_count == 0 {
            return Self {
                nodes: Vec::new(),
                order: Vec::new(),
                bounds: Bounds::EMPTY,
            };
        }

        let mut order: Vec<u32> = (0..triangle_count as u32).collect();
        let mut nodes: Vec<Node> = Vec::with_capacity(triangle_count.max(1) * 2);
        let bounds = build_range(&mut nodes, &mut order, mesh, 0, triangle_count, 0);

        Self {
            nodes,
            order,
            bounds,
        }
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Finds the closest ray-mesh intersection.
    pub fn intersect(&self, mesh: &Mesh, ray: &Ray) -> Option<Hit> {
        if self.nodes.is_empty() {
            return None;
        }

        let inv_dir = Vec3::new(
            safe_inverse(ray.direction.x),
            safe_inverse(ray.direction.y),
            safe_inverse(ray.direction.z),
        );

        let mut best: Option<Hit> = None;
        let mut best_distance = f32::INFINITY;
        let mut stack = [0u32; 64];
        let mut stack_ptr = 1;
        stack[0] = 0;

        while stack_ptr > 0 {
            stack_ptr -= 1;
            let node_index = stack[stack_ptr];
            let Some(node) = self.nodes.get(node_index as usize) else {
                continue;
            };

            if !aabb_hit(node.min, node.max, ray, inv_dir, best_distance) {
                continue;
            }

            if node.count > 0 {
                // Leaf node
                let start = node.first as usize;
                let end = start + node.count as usize;
                if let Some(tri_slice) = self.order.get(start..end) {
                    for &triangle in tri_slice {
                        if let Some(hit) = intersect_triangle(mesh, ray, triangle) {
                            if hit.distance < best_distance {
                                best_distance = hit.distance;
                                best = Some(hit);
                            }
                        }
                    }
                }
            } else {
                // Internal node
                if stack_ptr + 2 <= stack.len() {
                    stack[stack_ptr] = node.first;
                    stack[stack_ptr + 1] = node.right;
                    stack_ptr += 2;
                }
            }
        }

        best
    }
}

#[inline(always)]
fn safe_inverse(x: f32) -> f32 {
    if x.abs() < 1e-12 {
        if x.is_sign_negative() {
            -1e12
        } else {
            1e12
        }
    } else {
        1.0 / x
    }
}

#[inline(always)]
fn aabb_hit(min: Vec3, max: Vec3, ray: &Ray, inv_dir: Vec3, max_t: f32) -> bool {
    let t0x = (min.x - ray.origin.x) * inv_dir.x;
    let t1x = (max.x - ray.origin.x) * inv_dir.x;
    let tminx = t0x.min(t1x);
    let tmaxx = t0x.max(t1x);

    let t0y = (min.y - ray.origin.y) * inv_dir.y;
    let t1y = (max.y - ray.origin.y) * inv_dir.y;
    let tminy = t0y.min(t1y);
    let tmaxy = t0y.max(t1y);

    let t0z = (min.z - ray.origin.z) * inv_dir.z;
    let t1z = (max.z - ray.origin.z) * inv_dir.z;
    let tminz = t0z.min(t1z);
    let tmaxz = t0z.max(t1z);

    let tmin = tminx.max(tminy).max(tminz).max(0.0);
    let tmax = tmaxx.min(tmaxy).min(tmaxz).min(max_t);

    tmin <= tmax
}

fn intersect_triangle(mesh: &Mesh, ray: &Ray, triangle: u32) -> Option<Hit> {
    let i0 = *mesh.indices.get(triangle as usize * 3)? as usize;
    let i1 = *mesh.indices.get(triangle as usize * 3 + 1)? as usize;
    let i2 = *mesh.indices.get(triangle as usize * 3 + 2)? as usize;

    let v0 = *mesh.positions.get(i0)?;
    let v1 = *mesh.positions.get(i1)?;
    let v2 = *mesh.positions.get(i2)?;

    let (t, u, v) = ray_triangle(ray, v0, v1, v2)?;
    let w = 1.0 - u - v;

    let hit_pos = ray.point_at(t);
    let normal = if !mesh.normals.is_empty() {
        let n0 = mesh.normals.get(i0).copied().unwrap_or(Vec3::Y);
        let n1 = mesh.normals.get(i1).copied().unwrap_or(Vec3::Y);
        let n2 = mesh.normals.get(i2).copied().unwrap_or(Vec3::Y);
        (n0 * w + n1 * u + n2 * v).normalize()
    } else {
        (v1 - v0).cross(v2 - v0).normalize()
    };

    let uv = if !mesh.uvs.is_empty() {
        let uv0 = mesh.uvs.get(i0).copied().unwrap_or(Vec2::ZERO);
        let uv1 = mesh.uvs.get(i1).copied().unwrap_or(Vec2::ZERO);
        let uv2 = mesh.uvs.get(i2).copied().unwrap_or(Vec2::ZERO);
        uv0 * w + uv1 * u + uv2 * v
    } else {
        Vec2::ZERO
    };

    Some(Hit {
        distance: t,
        point: hit_pos,
        normal,
        uv,
        triangle_index: triangle,
    })
}

fn triangle_bounds(mesh: &Mesh, tri_index: u32) -> Bounds {
    let base = tri_index as usize * 3;
    let mut b = Bounds::EMPTY;
    for &idx in &mesh.indices[base..base + 3] {
        if let Some(&p) = mesh.positions.get(idx as usize) {
            b.extend(p);
        }
    }
    b
}

fn build_range(
    nodes: &mut Vec<Node>,
    order: &mut [u32],
    mesh: &Mesh,
    start: usize,
    count: usize,
    depth: u32,
) -> Bounds {
    let mut bounds = Bounds::EMPTY;
    for &tri in &order[start..start + count] {
        bounds = bounds.union(&triangle_bounds(mesh, tri));
    }

    if count <= MAX_LEAF || depth >= MAX_DEPTH {
        nodes.push(Node {
            min: bounds.min,
            max: bounds.max,
            first: start as u32,
            count: count as u32,
            right: 0,
        });
        return bounds;
    }

    let extent = bounds.extent();
    let axis = if extent.x >= extent.y && extent.x >= extent.z {
        0
    } else if extent.y >= extent.z {
        1
    } else {
        2
    };

    let mid = start + count / 2;
    order[start..start + count].select_nth_unstable_by(count / 2, |&a, &b| {
        let ca = triangle_bounds(mesh, a).center();
        let cb = triangle_bounds(mesh, b).center();
        let va = if axis == 0 { ca.x } else if axis == 1 { ca.y } else { ca.z };
        let vb = if axis == 0 { cb.x } else if axis == 1 { cb.y } else { cb.z };
        va.partial_cmp(&vb).unwrap_or(std::cmp::Ordering::Equal)
    });

    let node_idx = nodes.len();
    nodes.push(Node {
        min: bounds.min,
        max: bounds.max,
        first: 0,
        count: 0,
        right: 0,
    });

    let left_count = mid - start;
    let right_count = count - left_count;

    let left_idx = nodes.len() as u32;
    build_range(nodes, order, mesh, start, left_count, depth + 1);

    let right_idx = nodes.len() as u32;
    build_range(nodes, order, mesh, mid, right_count, depth + 1);

    nodes[node_idx].first = left_idx;
    nodes[node_idx].right = right_idx;

    bounds
}

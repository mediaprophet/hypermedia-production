//! Mesh representation, submesh grouping, and primitive builders.

use crate::math::{Bounds, Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};

/// A submesh range within a parent mesh.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Submesh {
    pub name: String,
    pub first_index: u32,
    pub index_count: u32,
    pub material_id: u32,
}

/// A 3D polygonal mesh with vertex attributes and triangle indexing.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub tangents: Vec<Vec4>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
    pub submeshes: Vec<Submesh>,
}

impl Mesh {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn bounds(&self) -> Bounds {
        let mut b = Bounds::EMPTY;
        for p in &self.positions {
            b.extend(*p);
        }
        b
    }

    /// Compute face/area-weighted vertex normals if not provided.
    pub fn compute_normals(&mut self) {
        self.normals = vec![Vec3::ZERO; self.positions.len()];
        for tri in self.indices.chunks_exact(3) {
            let (i0, i1, i2) = (tri[0] as usize, tri[1] as usize, tri[2] as usize);
            if i0 >= self.positions.len() || i1 >= self.positions.len() || i2 >= self.positions.len() {
                continue;
            }
            let (v0, v1, v2) = (self.positions[i0], self.positions[i1], self.positions[i2]);
            let normal = (v1 - v0).cross(v2 - v0);
            self.normals[i0] += normal;
            self.normals[i1] += normal;
            self.normals[i2] += normal;
        }
        for n in &mut self.normals {
            *n = n.normalize();
        }
    }

    /// Creates a unit quad on the XY plane with UVs in [0, 1].
    pub fn create_quad(size: f32) -> Self {
        let half = size * 0.5;
        let positions = vec![
            Vec3::new(-half, -half, 0.0),
            Vec3::new(half, -half, 0.0),
            Vec3::new(half, half, 0.0),
            Vec3::new(-half, half, 0.0),
        ];
        let normals = vec![Vec3::new(0.0, 0.0, 1.0); 4];
        let tangents = vec![Vec4::new(1.0, 0.0, 0.0, 1.0); 4];
        let uvs = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        let submeshes = vec![Submesh {
            name: "Quad".to_string(),
            first_index: 0,
            index_count: 6,
            material_id: 0,
        }];
        Self {
            positions,
            normals,
            tangents,
            uvs,
            indices,
            submeshes,
        }
    }

    /// Creates a unit cube centered at origin.
    pub fn create_cube(size: f32) -> Self {
        let half = size * 0.5;
        let mut mesh = Self::new();

        // 6 faces
        let faces = [
            (Vec3::new(0.0, 0.0, 1.0), Vec3::X, Vec3::Y),   // +Z
            (Vec3::new(0.0, 0.0, -1.0), -Vec3::X, Vec3::Y), // -Z
            (Vec3::new(1.0, 0.0, 0.0), -Vec3::Z, Vec3::Y),  // +X
            (Vec3::new(-1.0, 0.0, 0.0), Vec3::Z, Vec3::Y),  // -X
            (Vec3::new(0.0, 1.0, 0.0), Vec3::X, -Vec3::Z),  // +Y
            (Vec3::new(0.0, -1.0, 0.0), Vec3::X, Vec3::Z),  // -Y
        ];

        for (normal, right, up) in faces {
            let base = mesh.positions.len() as u32;
            let center = normal * half;
            let v0 = center - right * half - up * half;
            let v1 = center + right * half - up * half;
            let v2 = center + right * half + up * half;
            let v3 = center - right * half + up * half;

            mesh.positions.extend_from_slice(&[v0, v1, v2, v3]);
            mesh.normals.extend_from_slice(&[normal, normal, normal, normal]);
            mesh.tangents.extend_from_slice(&[Vec4::new(right.x, right.y, right.z, 1.0); 4]);
            mesh.uvs.extend_from_slice(&[
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ]);
            mesh.indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        mesh.submeshes.push(Submesh {
            name: "Cube".to_string(),
            first_index: 0,
            index_count: mesh.indices.len() as u32,
            material_id: 0,
        });

        mesh
    }
}

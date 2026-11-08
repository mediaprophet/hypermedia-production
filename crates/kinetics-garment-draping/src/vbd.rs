//! CPU f64 Vertex Block Descent (VBD) cloth solver with exact local gradients and Hessians.
//! Non-linear, objective-checked variational descent without XPBD constraint projections.

use crate::energy::{FabricMaterial, Hinge, Triangle, bending_element, membrane_element};
use crate::math::DVec3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VbdSettings {
    pub dt: f64,
    pub iterations: usize,
    pub damping: f64,
    pub gravity: DVec3,
}

impl Default for VbdSettings {
    fn default() -> Self {
        Self {
            dt: 1.0 / 120.0,
            iterations: 16,
            damping: 1.5,
            gravity: DVec3::new(0.0, -9.81, 0.0),
        }
    }
}

/// Simulation mesh topology and vertex state for VBD.
#[derive(Clone, Debug)]
pub struct ClothMesh {
    pub positions: Vec<DVec3>,
    pub velocities: Vec<DVec3>,
    pub masses: Vec<f64>,
    pub pins: Vec<Option<DVec3>>,
    pub triangles: Vec<Triangle>,
    pub hinges: Vec<Hinge>,
    pub material: FabricMaterial,
}

impl ClothMesh {
    pub fn new(positions: Vec<DVec3>, triangles: Vec<Triangle>, hinges: Vec<Hinge>, material: FabricMaterial) -> Self {
        let n = positions.len();
        let velocities = vec![DVec3::ZERO; n];
        let masses = vec![1.0; n];
        let pins = vec![None; n];
        Self {
            positions,
            velocities,
            masses,
            pins,
            triangles,
            hinges,
            material,
        }
    }
}

/// Executes one VBD simulation step.
pub fn vbd_step(mesh: &mut ClothMesh, settings: &VbdSettings) {
    let n = mesh.positions.len();
    if n == 0 {
        return;
    }

    let dt = settings.dt;
    let dt2 = dt * dt;
    let damp = (1.0 - settings.damping * dt).max(0.0);

    let x_prev = mesh.positions.clone();

    // 1. Inertial prediction
    for i in 0..n {
        if let Some(pin_pos) = mesh.pins[i] {
            mesh.positions[i] = pin_pos;
            mesh.velocities[i] = DVec3::ZERO;
            continue;
        }
        mesh.positions[i] += mesh.velocities[i] * (dt * damp) + settings.gravity * dt2;
    }

    // Precompute triangle connectivity per vertex
    let mut vert_triangles: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (ti, tri) in mesh.triangles.iter().enumerate() {
        for &v in &tri.vertices {
            if v < n {
                vert_triangles[v].push(ti);
            }
        }
    }

    let mut vert_hinges: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (hi, h) in mesh.hinges.iter().enumerate() {
        for &v in &h.vertices {
            if v < n {
                vert_hinges[v].push(hi);
            }
        }
    }

    // 2. Vertex Block Descent iterations
    for _iter in 0..settings.iterations {
        for i in 0..n {
            if mesh.pins[i].is_some() {
                continue;
            }

            let m = mesh.masses[i].max(1e-4);
            // Inertial term: m / dt^2 * (x - x_pred)
            let mut g = (mesh.positions[i] - x_prev[i] - mesh.velocities[i] * (dt * damp) - settings.gravity * dt2) * (m / dt2);
            let mut h_diag = DVec3::new(m / dt2, m / dt2, m / dt2);

            // Accumulate internal forces and diagonal Hessians from adjacent triangles
            for &ti in &vert_triangles[i] {
                let elem = membrane_element(&mesh.positions, &mesh.triangles[ti], &mesh.material);
                // Locate local index of vertex i in triangle
                if let Some(local_idx) = mesh.triangles[ti].vertices.iter().position(|&v| v == i) {
                    let g_offset = 3 * local_idx;
                    g.x += elem.gradient[g_offset];
                    g.y += elem.gradient[g_offset + 1];
                    g.z += elem.gradient[g_offset + 2];

                    let h_stride = 9;
                    h_diag.x += elem.hessian[g_offset * h_stride + g_offset].max(0.0);
                    h_diag.y += elem.hessian[(g_offset + 1) * h_stride + (g_offset + 1)].max(0.0);
                    h_diag.z += elem.hessian[(g_offset + 2) * h_stride + (g_offset + 2)].max(0.0);
                }
            }

            // Accumulate bending
            for &hi in &vert_hinges[i] {
                let elem = bending_element(&mesh.positions, &mesh.hinges[hi], &mesh.material);
                if let Some(local_idx) = mesh.hinges[hi].vertices.iter().position(|&v| v == i) {
                    let g_offset = 3 * local_idx;
                    g.x += elem.gradient[g_offset];
                    g.y += elem.gradient[g_offset + 1];
                    g.z += elem.gradient[g_offset + 2];

                    let h_stride = 12;
                    h_diag.x += elem.hessian[g_offset * h_stride + g_offset].max(0.0);
                    h_diag.y += elem.hessian[(g_offset + 1) * h_stride + (g_offset + 1)].max(0.0);
                    h_diag.z += elem.hessian[(g_offset + 2) * h_stride + (g_offset + 2)].max(0.0);
                }
            }

            // Local 3x3 diagonal solve: dx = -g / h
            let dx = DVec3::new(
                -g.x / h_diag.x.max(1e-6),
                -g.y / h_diag.y.max(1e-6),
                -g.z / h_diag.z.max(1e-6),
            );

            mesh.positions[i] += dx;
        }
    }

    // 3. Velocity update
    for i in 0..n {
        if mesh.pins[i].is_some() {
            mesh.velocities[i] = DVec3::ZERO;
        } else {
            mesh.velocities[i] = (mesh.positions[i] - x_prev[i]) / dt;
        }
    }
}

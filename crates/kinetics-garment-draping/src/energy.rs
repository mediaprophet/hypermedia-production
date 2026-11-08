//! Independent nonlinear orthotropic membrane and flat-rest hinge bending energies.
//! Exact local gradients and Hessians via automatic differentiation (Jet<N>).

use crate::math::{DMat2, DVec2, DVec3, Jet, Scalar, cross, dot, sub};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FabricMaterial {
    pub warp: f64,
    pub weft: f64,
    pub shear: f64,
    pub compression_ratio: f64,
    pub compression_transition: f64,
    pub bend_warp: f64,
    pub bend_weft: f64,
    pub density: f64,
    pub thickness: f64,
    pub grain_degrees: f64,
}

impl Default for FabricMaterial {
    fn default() -> Self {
        Self {
            warp: 350.0,
            weft: 180.0,
            shear: 45.0,
            compression_ratio: 0.15,
            compression_transition: 0.005,
            bend_warp: 0.00008,
            bend_weft: 0.00004,
            density: 0.2,
            thickness: 0.001,
            grain_degrees: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub vertices: [usize; 3],
    pub rest: [DVec2; 3],
    pub inv_dm: DMat2,
    pub area: f64,
    pub material: usize,
}

#[derive(Clone, Debug)]
pub struct Hinge {
    pub vertices: [usize; 4],
    pub weight: f64,
    pub across: DVec2,
    pub material: usize,
}

/// Smooth tension/compression blend.
pub fn axial<S: Scalar>(strain: S, stiffness: f64, m: &FabricMaterial) -> S {
    let delta = S::c(m.compression_transition);
    let p = (strain + (strain * strain + delta * delta).sqrt() - delta) * S::c(0.5);
    S::c(0.5 * stiffness)
        * (S::c(m.compression_ratio) * strain * strain + S::c(1.0 - m.compression_ratio) * p * p)
}

/// Nonlinear orthotropic membrane energy on a single triangle.
pub fn membrane<S: Scalar>(x: [[S; 3]; 3], t: &Triangle, m: &FabricMaterial) -> S {
    let ds = [sub(x[1], x[0]), sub(x[2], x[0])];
    let f: [[S; 3]; 2] = [t.inv_dm.x_axis, t.inv_dm.y_axis].map(|c| {
        std::array::from_fn(|k| ds[0][k] * S::c(c.x) + ds[1][k] * S::c(c.y))
    });
    let (s, c) = (m.grain_degrees * std::f64::consts::PI / 180.0).sin_cos();
    let w = std::array::from_fn(|i| f[0][i] * S::c(c) + f[1][i] * S::c(s));
    let v = std::array::from_fn(|i| f[0][i] * S::c(-s) + f[1][i] * S::c(c));
    let ew = (dot(w, w) - S::c(1.0)) * S::c(0.5);
    let ev = (dot(v, v) - S::c(1.0)) * S::c(0.5);
    let shear = dot(w, v);
    S::c(t.area)
        * (axial(ew, m.warp, m) + axial(ev, m.weft, m) + S::c(0.5 * m.shear) * shear * shear)
}

/// Flat-rest hinge bending energy.
pub fn bending<S: Scalar>(x: [[S; 3]; 4], h: &Hinge, m: &FabricMaterial) -> S {
    let edge = sub(x[1], x[0]);
    let n1 = cross(edge, sub(x[2], x[0]));
    let n2 = cross(sub(x[3], x[0]), edge);
    let length1 = dot(n1, n1).sqrt();
    let length2 = dot(n2, n2).sqrt();
    let difference = std::array::from_fn(|i| n1[i] / length1 - n2[i] / length2);
    let (s, c) = (m.grain_degrees * std::f64::consts::PI / 180.0).sin_cos();
    let aw = h.across.x * c + h.across.y * s;
    let af = -h.across.x * s + h.across.y * c;
    S::c(0.5 * h.weight * (m.bend_warp * aw * aw + m.bend_weft * af * af))
        * dot(difference, difference)
}

#[derive(Clone, Debug)]
pub struct Element {
    pub dofs: Vec<usize>,
    pub energy: f64,
    pub gradient: Vec<f64>,
    pub hessian: Vec<f64>,
}

impl Element {
    pub fn from_jet<const N: usize>(vertices: &[usize], jet: Jet<N>) -> Self {
        Self {
            dofs: vertices
                .iter()
                .flat_map(|v| (0..3).map(move |k| 3 * v + k))
                .collect(),
            energy: jet.v,
            gradient: jet.g.to_vec(),
            hessian: jet.h.into_iter().flatten().collect(),
        }
    }
}

pub fn membrane_element(x: &[DVec3], t: &Triangle, m: &FabricMaterial) -> Element {
    let p = std::array::from_fn(|i| {
        let v_idx = t.vertices[i];
        let pos = x[v_idx];
        [
            Jet::<9>::variable(pos.x, 3 * i),
            Jet::<9>::variable(pos.y, 3 * i + 1),
            Jet::<9>::variable(pos.z, 3 * i + 2),
        ]
    });
    Element::from_jet(&t.vertices, membrane(p, t, m))
}

pub fn bending_element(x: &[DVec3], h: &Hinge, m: &FabricMaterial) -> Element {
    let p = std::array::from_fn(|i| {
        let v_idx = h.vertices[i];
        let pos = x[v_idx];
        [
            Jet::<12>::variable(pos.x, 3 * i),
            Jet::<12>::variable(pos.y, 3 * i + 1),
            Jet::<12>::variable(pos.z, 3 * i + 2),
        ]
    });
    Element::from_jet(&h.vertices, bending(p, h, m))
}

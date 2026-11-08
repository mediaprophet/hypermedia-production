//! Finite element shape functions, local natural coordinates, and quadrature rules.
//!
//! Grounded in physical modeling integrity and scientific collaboration (UDHR Art. 19, 27; ICESCR Art. 15).

use serde::{Deserialize, Serialize};

/// Element topology classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementType {
    Tri3,
    Tri6,
    Quad4,
    Quad8,
    Tet4,
    Tet10,
    Hex8,
    Hex20,
}

impl ElementType {
    pub fn node_count(self) -> usize {
        match self {
            ElementType::Tri3 => 3,
            ElementType::Tri6 => 6,
            ElementType::Quad4 => 4,
            ElementType::Quad8 => 8,
            ElementType::Tet4 => 4,
            ElementType::Tet10 => 10,
            ElementType::Hex8 => 8,
            ElementType::Hex20 => 20,
        }
    }

    pub fn dimension(self) -> usize {
        match self {
            ElementType::Tri3 | ElementType::Tri6 | ElementType::Quad4 | ElementType::Quad8 => 2,
            ElementType::Tet4 | ElementType::Tet10 | ElementType::Hex8 | ElementType::Hex20 => 3,
        }
    }
}

/// Linear 3-node triangle (Tri3) shape functions in area coordinates (xi, eta).
pub fn shape_tri3(xi: f64, eta: f64) -> [f64; 3] {
    let zeta = 1.0 - xi - eta;
    [zeta, xi, eta]
}

/// Tri3 shape function derivatives: [d/dxi, d/deta].
pub fn shape_deriv_tri3() -> [[f64; 2]; 3] {
    [[-1.0, -1.0], [1.0, 0.0], [0.0, 1.0]]
}

/// Quadratic 6-node triangle (Tri6) shape functions.
pub fn shape_tri6(xi: f64, eta: f64) -> [f64; 6] {
    let l3 = 1.0 - xi - eta;
    let l1 = xi;
    let l2 = eta;
    [
        l3 * (2.0 * l3 - 1.0),
        l1 * (2.0 * l1 - 1.0),
        l2 * (2.0 * l2 - 1.0),
        4.0 * l1 * l3,
        4.0 * l1 * l2,
        4.0 * l2 * l3,
    ]
}

/// Bilinear 4-node quadrilateral (Quad4) shape functions in [-1, 1]^2.
pub fn shape_quad4(xi: f64, eta: f64) -> [f64; 4] {
    [
        0.25 * (1.0 - xi) * (1.0 - eta),
        0.25 * (1.0 + xi) * (1.0 - eta),
        0.25 * (1.0 + xi) * (1.0 + eta),
        0.25 * (1.0 - xi) * (1.0 + eta),
    ]
}

/// Quad4 derivatives: [[dN_i/dxi, dN_i/deta]; 4].
pub fn shape_deriv_quad4(xi: f64, eta: f64) -> [[f64; 2]; 4] {
    [
        [-0.25 * (1.0 - eta), -0.25 * (1.0 - xi)],
        [0.25 * (1.0 - eta), -0.25 * (1.0 + xi)],
        [0.25 * (1.0 + eta), 0.25 * (1.0 + xi)],
        [-0.25 * (1.0 + eta), 0.25 * (1.0 - xi)],
    ]
}

/// Serendipity 8-node quadrilateral (Quad8) shape functions in [-1, 1]^2.
pub fn shape_quad8(xi: f64, eta: f64) -> [f64; 8] {
    let xi2 = xi * xi;
    let eta2 = eta * eta;
    [
        // Corner nodes (0..4)
        0.25 * (1.0 - xi) * (1.0 - eta) * (-xi - eta - 1.0),
        0.25 * (1.0 + xi) * (1.0 - eta) * (xi - eta - 1.0),
        0.25 * (1.0 + xi) * (1.0 + eta) * (xi + eta - 1.0),
        0.25 * (1.0 - xi) * (1.0 + eta) * (-xi + eta - 1.0),
        // Midside nodes (4..8)
        0.5 * (1.0 - xi2) * (1.0 - eta),
        0.5 * (1.0 + xi) * (1.0 - eta2),
        0.5 * (1.0 - xi2) * (1.0 + eta),
        0.5 * (1.0 - xi) * (1.0 - eta2),
    ]
}

/// Linear 4-node tetrahedron (Tet4) shape functions in volume coordinates (xi, eta, zeta).
pub fn shape_tet4(xi: f64, eta: f64, zeta: f64) -> [f64; 4] {
    let l4 = 1.0 - xi - eta - zeta;
    [l4, xi, eta, zeta]
}

/// Tet4 derivatives: [[d/dxi, d/deta, d/dzeta]; 4].
pub fn shape_deriv_tet4() -> [[f64; 3]; 4] {
    [[-1.0, -1.0, -1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
}

/// Trilinear 8-node hexahedron (Hex8) shape functions in [-1, 1]^3.
pub fn shape_hex8(xi: f64, eta: f64, zeta: f64) -> [f64; 8] {
    [
        0.125 * (1.0 - xi) * (1.0 - eta) * (1.0 - zeta),
        0.125 * (1.0 + xi) * (1.0 - eta) * (1.0 - zeta),
        0.125 * (1.0 + xi) * (1.0 + eta) * (1.0 - zeta),
        0.125 * (1.0 - xi) * (1.0 + eta) * (1.0 - zeta),
        0.125 * (1.0 - xi) * (1.0 - eta) * (1.0 + zeta),
        0.125 * (1.0 + xi) * (1.0 - eta) * (1.0 + zeta),
        0.125 * (1.0 + xi) * (1.0 + eta) * (1.0 + zeta),
        0.125 * (1.0 - xi) * (1.0 + eta) * (1.0 + zeta),
    ]
}

/// Hex8 derivatives: [[d/dxi, d/deta, d/dzeta]; 8].
pub fn shape_deriv_hex8(xi: f64, eta: f64, zeta: f64) -> [[f64; 3]; 8] {
    [
        [
            -0.125 * (1.0 - eta) * (1.0 - zeta),
            -0.125 * (1.0 - xi) * (1.0 - zeta),
            -0.125 * (1.0 - xi) * (1.0 - eta),
        ],
        [
            0.125 * (1.0 - eta) * (1.0 - zeta),
            -0.125 * (1.0 + xi) * (1.0 - zeta),
            -0.125 * (1.0 + xi) * (1.0 - eta),
        ],
        [
            0.125 * (1.0 + eta) * (1.0 - zeta),
            0.125 * (1.0 + xi) * (1.0 - zeta),
            -0.125 * (1.0 + xi) * (1.0 + eta),
        ],
        [
            -0.125 * (1.0 + eta) * (1.0 - zeta),
            0.125 * (1.0 - xi) * (1.0 - zeta),
            -0.125 * (1.0 - xi) * (1.0 + eta),
        ],
        [
            -0.125 * (1.0 - eta) * (1.0 + zeta),
            -0.125 * (1.0 - xi) * (1.0 + zeta),
            0.125 * (1.0 - xi) * (1.0 - eta),
        ],
        [
            0.125 * (1.0 - eta) * (1.0 + zeta),
            -0.125 * (1.0 + xi) * (1.0 + zeta),
            0.125 * (1.0 + xi) * (1.0 - eta),
        ],
        [
            0.125 * (1.0 + eta) * (1.0 + zeta),
            0.125 * (1.0 + xi) * (1.0 + zeta),
            0.125 * (1.0 + xi) * (1.0 + eta),
        ],
        [
            -0.125 * (1.0 + eta) * (1.0 + zeta),
            0.125 * (1.0 - xi) * (1.0 + zeta),
            0.125 * (1.0 - xi) * (1.0 + eta),
        ],
    ]
}

/// Gauss quadrature point and weight.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GaussPoint<const D: usize> {
    pub coords: [f64; D],
    pub weight: f64,
}

/// 2x2 Gauss-Legendre quadrature points for quadrilaterals in [-1, 1]^2.
pub fn gauss_quad_2x2() -> [GaussPoint<2>; 4] {
    let p = 1.0 / 3.0f64.sqrt();
    [
        GaussPoint { coords: [-p, -p], weight: 1.0 },
        GaussPoint { coords: [p, -p], weight: 1.0 },
        GaussPoint { coords: [p, p], weight: 1.0 },
        GaussPoint { coords: [-p, p], weight: 1.0 },
    ]
}

/// 2x2x2 Gauss-Legendre quadrature points for hexahedra in [-1, 1]^3.
pub fn gauss_hex_2x2x2() -> [GaussPoint<3>; 8] {
    let p = 1.0 / 3.0f64.sqrt();
    [
        GaussPoint { coords: [-p, -p, -p], weight: 1.0 },
        GaussPoint { coords: [p, -p, -p], weight: 1.0 },
        GaussPoint { coords: [p, p, -p], weight: 1.0 },
        GaussPoint { coords: [-p, p, -p], weight: 1.0 },
        GaussPoint { coords: [-p, -p, p], weight: 1.0 },
        GaussPoint { coords: [p, -p, p], weight: 1.0 },
        GaussPoint { coords: [p, p, p], weight: 1.0 },
        GaussPoint { coords: [-p, p, p], weight: 1.0 },
    ]
}

/// 1-point quadrature for linear tetrahedron.
pub fn gauss_tet_1pt() -> [GaussPoint<3>; 1] {
    [GaussPoint {
        coords: [0.25, 0.25, 0.25],
        weight: 1.0 / 6.0,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_of_unity() {
        let tri = shape_tri3(0.2, 0.3);
        assert!((tri.iter().sum::<f64>() - 1.0).abs() < 1e-12);

        let quad = shape_quad4(0.4, -0.6);
        assert!((quad.iter().sum::<f64>() - 1.0).abs() < 1e-12);

        let hex = shape_hex8(0.1, -0.2, 0.7);
        assert!((hex.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }
}

//! Finite element method primitives: shape functions, constitutive laws, and structural elements.

pub mod constitutive;
pub mod shape;
pub mod structural;

pub use constitutive::{ElasticIsotropic, J2Plasticity, NeoHookean, SaintVenantKirchhoff};
pub use shape::{
    gauss_hex_2x2x2, gauss_quad_2x2, gauss_tet_1pt, shape_deriv_hex8, shape_deriv_quad4, shape_deriv_tet4,
    shape_deriv_tri3, shape_hex8, shape_quad4, shape_quad8, shape_tet4, shape_tri3, shape_tri6, ElementType,
    GaussPoint,
};
pub use structural::{mitc4_shear_strains, thermal_conduction_quad4, timoshenko_beam_3d};

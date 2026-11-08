//! Computational mesh algorithms: 2D CDT, 3D tetrahedral/hexahedral meshing, and quality metrics.

pub mod cdt;
pub mod hex;
pub mod quality;
pub mod tet;

pub use cdt::{orient2d, triangulate_2d, Point2D, Triangle};
pub use hex::{generate_box_hex_mesh, HexMesh, Hexahedron};
pub use quality::{hex_scaled_jacobian, laplacian_smooth_step};
pub use tet::{hex_to_tets, tet_volume, Point3D, Tetrahedron};

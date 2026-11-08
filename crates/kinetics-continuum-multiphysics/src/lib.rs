//! # Kinetics Continuum Multiphysics Engine
//!
//! A pure Rust, UI-agnostic categorical continuum multiphysics domain crate providing
//! finite element shape functions, hyperelasticity and J2 plasticity constitutive laws,
//! structural MITC4 shells and Timoshenko beams, thermal conduction, deterministic
//! sparse Cholesky and LDL^T solvers, shift-and-invert Lanczos modal eigen-analysis,
//! D3Q19 Lattice Boltzmann CFD, explicit dynamics with Flanagan-Belytschko hourglass
//! control, 2D/3D Delaunay meshing, and open engineering deck interchange (Abaqus,
//! Nastran, LS-DYNA, Gmsh, VTK).
//!
//! Grounded in international human rights guarantees of scientific and technological
//! progress (UDHR Articles 19 & 27; ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod explicit;
pub mod fem;
pub mod interchange;
pub mod lbm;
pub mod mesh;
pub mod solver;

pub use explicit::{
    flanagan_belytschko_hex8, ExplicitIntegrator, ExplicitState, JohnsonCookDamage, GAMMA_HOURGLASS,
};
pub use fem::{
    gauss_hex_2x2x2, gauss_quad_2x2, gauss_tet_1pt, mitc4_shear_strains, shape_deriv_hex8, shape_deriv_quad4,
    shape_deriv_tet4, shape_deriv_tri3, shape_hex8, shape_quad4, shape_quad8, shape_tet4, shape_tri3, shape_tri6,
    thermal_conduction_quad4, timoshenko_beam_3d, ElasticIsotropic, ElementType, GaussPoint, J2Plasticity,
    NeoHookean, SaintVenantKirchhoff,
};
pub use interchange::{
    export_abaqus_inp, export_gmsh_22, export_lsdyna_k, export_nastran_bdf, export_vtu_hex, VTK_HEXAHEDRON,
    VTK_TETRA,
};
pub use lbm::{equilibrium_d3q19, LbmGrid3D, C_DIRS, OPPOSITE, WEIGHTS};
pub use mesh::{
    generate_box_hex_mesh, hex_scaled_jacobian, hex_to_tets, laplacian_smooth_step, orient2d, tet_volume,
    triangulate_2d, HexMesh, Hexahedron, Point2D, Point3D, Tetrahedron, Triangle,
};
pub use solver::{
    lanczos_eigensolver, newton_raphson_solve, CholeskyFactor, CsrMatrix, Eigenpair, EquilibriumStep, Inertia,
    LdltFactor,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_continuum_multiphysics_smoke() {
        // 1. FEM constitutive & shape evaluation
        let mat = ElasticIsotropic::new(210e9, 0.3, 7850.0);
        assert!(mat.lame_lambda() > 0.0);
        assert!(mat.shear_modulus() > 0.0);
        let n_quad = shape_quad4(0.0, 0.0);
        assert!((n_quad.iter().sum::<f64>() - 1.0).abs() < 1e-12);

        // 2. Cholesky linear solve
        let k_dense = [4.0, 1.0, 1.0, 3.0];
        let chol = CholeskyFactor::factorize(2, &k_dense).unwrap();
        let x = chol.solve(&[5.0, 4.0]);
        assert!((x[0] - 1.0).abs() < 1e-10);
        assert!((x[1] - 1.0).abs() < 1e-10);

        // 3. D3Q19 Lattice Boltzmann conservation
        let mut grid = LbmGrid3D::new(4, 4, 4, 0.02);
        let mass_before: f64 = grid.f.iter().sum();
        grid.step();
        let mass_after: f64 = grid.f.iter().sum();
        assert!((mass_after - mass_before).abs() < 1e-6);

        // 4. Hex mesh generation & deck interchange
        let mesh = generate_box_hex_mesh(1, 1, 1, Point3D::new(0.0, 0.0, 0.0), Point3D::new(1.0, 1.0, 1.0));
        assert_eq!(mesh.elements.len(), 1);
        let hex_conn: Vec<[usize; 8]> = mesh.elements.iter().map(|e| e.nodes).collect();
        let abaqus = export_abaqus_inp(&mesh.nodes, &hex_conn, "STEEL", 210e9, 0.3);
        assert!(abaqus.contains("*ELEMENT, TYPE=C3D8"));
    }
}

# `crates/kinetics-continuum-multiphysics`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Explicit dynamics: central-difference time integration, hourglass control, and damage erosion._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl JohnsonCookDamage::fracture_strain` | 4 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:23` | - | Fracture strain eps_f = [d1 + d2 * exp(d3 * sigma*)] * [1 + d4 * ln(eps_dot*)] * [1 + d5 * T*] |
| `impl JohnsonCookDamage::update_damage` | 6 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:43` | - | Update damage index D = sum(delta_eps_p / eps_f). Returns true if element is eroded (D >= 1.0). |
| `johnson_cook_damage_erosion` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/damage.rs:63` | - |  |
| `impl ExplicitIntegrator::new` | 3 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:27` | physics |  |
| `impl ExplicitIntegrator::step` | 3 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:47` | - | Advance one time step using central differences: |
| `single_dof_harmonic_oscillator` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/dynamics.rs:73` | - |  |
| `flanagan_belytschko_hex8` | 5 | `crates/kinetics-continuum-multiphysics/src/explicit/hourglass.rs:14` | - | Compute Flanagan-Belytschko hourglass resisting nodal forces (8 nodes x 3 DOFs). |
| `hourglass_zero_for_rigid_translation` | 0 | `crates/kinetics-continuum-multiphysics/src/explicit/hourglass.rs:53` | - |  |
| `impl ElasticIsotropic::new` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:16` | - |  |
| `impl ElasticIsotropic::lame_lambda` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:25` | - | First Lamé parameter lambda = E * nu / ((1 + nu) * (1 - 2*nu)). |
| `impl ElasticIsotropic::shear_modulus` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:32` | - | Second Lamé parameter (shear modulus) mu = E / (2 * (1 + nu)). |
| `impl ElasticIsotropic::bulk_modulus` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:37` | - | Bulk modulus K = E / (3 * (1 - 2*nu)). |
| `impl ElasticIsotropic::elasticity_matrix_3d` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:42` | numerics | 6x6 elasticity matrix (Voigt notation: [eps_xx, eps_yy, eps_zz, gamma_yz, gamma_xz, gamma_xy]). |
| `impl ElasticIsotropic::plane_stress_matrix` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:62` | numerics | 3x3 plane stress elasticity matrix [eps_xx, eps_yy, gamma_xy]. |
| `impl ElasticIsotropic::plane_strain_matrix` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:74` | numerics | 3x3 plane strain elasticity matrix [eps_xx, eps_yy, gamma_xy]. |
| `impl SaintVenantKirchhoff::new` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:93` | - |  |
| `impl SaintVenantKirchhoff::compute_pk2_stress` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:98` | - | Compute Second Piola-Kirchhoff stress tensor S from Green-Lagrange strain tensor E (3x3). |
| `impl NeoHookean::new` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:122` | - |  |
| `impl NeoHookean::compute_cauchy_stress` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:127` | - | Compute Cauchy stress sigma from deformation gradient F (3x3). |
| `impl J2Plasticity::new` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:169` | - |  |
| `impl J2Plasticity::radial_return` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:178` | - | Classical radial return algorithm: updates stress, backstress, and equivalent plastic strain. |
| `det3x3` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:233` | - |  |
| `elasticity_matrix_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:244` | numerics |  |
| `j2_plasticity_yields_when_exceeding_threshold` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/constitutive.rs:255` | - |  |
| `impl ElementType::node_count` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:21` | graph |  |
| `impl ElementType::dimension` | 1 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:34` | - |  |
| `shape_tri3` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:43` | graph, numerics | Linear 3-node triangle (Tri3) shape functions in area coordinates (xi, eta). |
| `shape_deriv_tri3` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:49` | - | Tri3 shape function derivatives: [d/dxi, d/deta]. |
| `shape_tri6` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:54` | graph | Quadratic 6-node triangle (Tri6) shape functions. |
| `shape_quad4` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:69` | graph | Bilinear 4-node quadrilateral (Quad4) shape functions in [-1, 1]^2. |
| `shape_deriv_quad4` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:79` | - | Quad4 derivatives: [[dN_i/dxi, dN_i/deta]; 4]. |
| `shape_quad8` | 2 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:89` | graph | Serendipity 8-node quadrilateral (Quad8) shape functions in [-1, 1]^2. |
| `shape_tet4` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:107` | graph, numerics | Linear 4-node tetrahedron (Tet4) shape functions in volume coordinates (xi, eta, zeta). |
| `shape_deriv_tet4` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:113` | - | Tet4 derivatives: [[d/dxi, d/deta, d/dzeta]; 4]. |
| `shape_hex8` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:118` | graph | Trilinear 8-node hexahedron (Hex8) shape functions in [-1, 1]^3. |
| `shape_deriv_hex8` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:132` | - | Hex8 derivatives: [[d/dxi, d/deta, d/dzeta]; 8]. |
| `gauss_quad_2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:185` | - | 2x2 Gauss-Legendre quadrature points for quadrilaterals in [-1, 1]^2. |
| `gauss_hex_2x2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:196` | - | 2x2x2 Gauss-Legendre quadrature points for hexahedra in [-1, 1]^3. |
| `gauss_tet_1pt` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:211` | numerics | 1-point quadrature for linear tetrahedron. |
| `partition_of_unity` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/shape.rs:223` | - |  |
| `timoshenko_beam_3d` | 6 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:9` | graph, numerics | 3D Timoshenko beam element stiffness matrix (12x12 for 2 nodes with 6 DOFs per node). |
| `mitc4_shear_strains` | 5 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:88` | graph | MITC4 (Mixed Interpolation of Tensorial Components) 4-node shell element. |
| `thermal_conduction_quad4` | 3 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:106` | graph, numerics | 4-node 2D thermal conduction element conductivity matrix (4x4). |
| `timoshenko_beam_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:160` | - |  |
| `thermal_conduction_matrix_symmetry` | 0 | `crates/kinetics-continuum-multiphysics/src/fem/structural.rs:171` | numerics |  |
| `export_abaqus_inp` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/abaqus.rs:8` | - | Export nodes and hexahedral elements to Abaqus `.inp` format. |
| `abaqus_deck_format_test` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/abaqus.rs:56` | - |  |
| `export_gmsh_22` | 2 | `crates/kinetics-continuum-multiphysics/src/interchange/gmsh.rs:8` | computational geometry | Export mesh to Gmsh MSH 2.2 ASCII format. |
| `gmsh_format_headers` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/gmsh.rs:45` | - |  |
| `export_lsdyna_k` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/lsdyna.rs:8` | - | Export to LS-DYNA `.k` format. |
| `lsdyna_deck_structure` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/lsdyna.rs:55` | - |  |
| `export_nastran_bdf` | 5 | `crates/kinetics-continuum-multiphysics/src/interchange/nastran.rs:8` | - | Export to 8-field small-field Nastran BDF format. |
| `nastran_deck_contains_bulk_and_grid` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/nastran.rs:58` | - |  |
| `export_vtu_hex` | 3 | `crates/kinetics-continuum-multiphysics/src/interchange/vtk.rs:12` | computational geometry | Export hexahedral mesh to VTK XML UnstructuredGrid (.vtu) format with nodal displacements. |
| `vtu_xml_structure_test` | 0 | `crates/kinetics-continuum-multiphysics/src/interchange/vtk.rs:82` | - |  |
| `equilibrium_d3q19` | 2 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:45` | economics | Equilibrium distribution for incompressible Navier-Stokes. |
| `impl LbmGrid3D::new` | 4 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:73` | - |  |
| `impl LbmGrid3D::idx` | 4 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:94` | - |  |
| `impl LbmGrid3D::collide` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:99` | - | Two-relaxation-time (TRT) collision operator with magic parameter Lambda = 1/4. |
| `impl LbmGrid3D::stream` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:151` | - | Stream step with half-way bounce-back on solid boundaries. |
| `impl LbmGrid3D::step` | 1 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:181` | - | Step simulation forward one time increment. |
| `lbm_mass_conservation` | 0 | `crates/kinetics-continuum-multiphysics/src/lbm/d3q19.rs:192` | physics |  |
| `full_continuum_multiphysics_smoke` | 0 | `crates/kinetics-continuum-multiphysics/src/lib.rs:51` | - |  |
| `impl Point2D::new` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:15` | - |  |
| `impl Point2D::dist_sq` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:18` | - |  |
| `orient2d` | 3 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:32` | - | Orientation test: positive if p0 -> p1 -> p2 is counter-clockwise. |
| `in_circle` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:37` | - | In-circle test: positive if p is inside the circumcircle of counter-clockwise (p0, p1, p2). |
| `triangulate_2d` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:51` | computational geometry | Compute 2D Delaunay triangulation via incremental point insertion. |
| `triangulate_four_points` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/cdt.rs:147` | computational geometry |  |
| `generate_box_hex_mesh` | 5 | `crates/kinetics-continuum-multiphysics/src/mesh/hex.rs:22` | computational geometry | Generate regular mapped hexahedral grid of nx x ny x nz cells. |
| `hex_mesh_node_and_element_counts` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/hex.rs:79` | computational geometry, graph |  |
| `hex_scaled_jacobian` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:8` | - | Scaled Jacobian metric for a hexahedral element: in [-1, 1], where 1 is an ideal cube. |
| `laplacian_smooth_step` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:51` | graph | Laplacian smoothing step for interior node positions given neighbor adjacency. |
| `vec_sub` | 2 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:80` | - |  |
| `vec_len` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:84` | - |  |
| `perfect_cube_scaled_jacobian_is_one` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/quality.rs:93` | - |  |
| `impl Point3D::new` | 3 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:16` | - |  |
| `tet_volume` | 4 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:28` | - | Signed volume of a tetrahedron (positive if nodes satisfy right-hand rule). |
| `hex_to_tets` | 1 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:41` | computational geometry | Generate 5 or 6 tetrahedra from a hexahedral bounding box. |
| `tet_volume_unit_cube_subdivision` | 0 | `crates/kinetics-continuum-multiphysics/src/mesh/tet.rs:56` | - |  |
| `impl CholeskyFactor::factorize` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:14` | numerics | Factorize symmetric positive-definite n x n matrix in row-major slice. |
| `impl CholeskyFactor::solve` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:45` | numerics | Solve A * x = b via forward substitution (L * y = b) and backward substitution (L^T * x = y). |
| `cholesky_factorize_and_solve_2x2` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/cholesky.rs:78` | numerics |  |
| `lanczos_eigensolver` | 5 | `crates/kinetics-continuum-multiphysics/src/solver/lanczos.rs:15` | - | Shift-and-invert Lanczos iteration to extract lowest eigenvalues of K * x = lambda * M * x. |
| `lanczos_finds_fundamental_frequency` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/lanczos.rs:82` | - |  |
| `impl LdltFactor::factorize` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:23` | numerics | Factorize symmetric n x n matrix. |
| `impl LdltFactor::inertia` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:63` | - | Compute Sylvester's inertia from the diagonal entries of D. |
| `impl LdltFactor::solve` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:86` | numerics | Solve A * x = b via L * z = b, D * y = z, L^T * x = y. |
| `ldlt_inertia_count` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/ldlt.rs:128` | - |  |
| `newton_raphson_solve` | 7 | `crates/kinetics-continuum-multiphysics/src/solver/nonlinear.rs:17` | numerics | Standard full Newton-Raphson nonlinear solver for residual R(u, lambda) = lambda * P_ref - F_int(u) = 0. |
| `newton_raphson_linear_spring_convergence` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/nonlinear.rs:75` | numerics |  |
| `impl CsrMatrix::new` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:18` | - |  |
| `impl CsrMatrix::from_triplets` | 3 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:29` | numerics | Construct CSR matrix from triplets (row, col, value). |
| `impl CsrMatrix::matvec` | 2 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:70` | numerics | Matrix-vector multiplication y = A * x. |
| `csr_matvec_matches_dense` | 0 | `crates/kinetics-continuum-multiphysics/src/solver/sparse.rs:91` | - |  |

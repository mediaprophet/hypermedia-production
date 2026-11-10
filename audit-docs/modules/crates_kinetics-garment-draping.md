# `crates/kinetics-garment-draping`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_kinetics-garment-draping: UI-agnostic categorical garment mechanics and draping.

Provides second-order forward automatic differentiation (Jet<N>), nonlinear
orthotropic membrane & flat-rest hinge bending energies, calibrated textile libraries,
CPU f64 Vertex Block Descent (VBD), and implicit Euler regularized Newton-PCG solver.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for FabricMaterial::default` | 0 | `crates/kinetics-garment-draping/src/energy.rs:22` | - |  |
| `axial` | 3 | `crates/kinetics-garment-draping/src/energy.rs:56` | - | Smooth tension/compression blend. |
| `membrane` | 3 | `crates/kinetics-garment-draping/src/energy.rs:64` | - | Nonlinear orthotropic membrane energy on a single triangle. |
| `bending` | 3 | `crates/kinetics-garment-draping/src/energy.rs:80` | - | Flat-rest hinge bending energy. |
| `impl Element::from_jet` | 2 | `crates/kinetics-garment-draping/src/energy.rs:103` | - |  |
| `membrane_element` | 3 | `crates/kinetics-garment-draping/src/energy.rs:116` | - |  |
| `bending_element` | 3 | `crates/kinetics-garment-draping/src/energy.rs:129` | - |  |
| `impl FabricPreset::to_material` | 1 | `crates/kinetics-garment-draping/src/fabrics.rs:22` | - |  |
| `standard_fabrics` | 0 | `crates/kinetics-garment-draping/src/fabrics.rs:45` | - |  |
| `get_fabric_by_id` | 1 | `crates/kinetics-garment-draping/src/fabrics.rs:110` | - |  |
| `test_jet_forward_ad` | 0 | `crates/kinetics-garment-draping/src/lib.rs:31` | - |  |
| `test_membrane_and_bending_energy` | 0 | `crates/kinetics-garment-draping/src/lib.rs:50` | - |  |
| `test_vbd_step_simulation` | 0 | `crates/kinetics-garment-draping/src/lib.rs:83` | - |  |
| `test_newton_pcg_solver_step` | 0 | `crates/kinetics-garment-draping/src/lib.rs:113` | - |  |
| `test_bezier_curve_and_arclength` | 0 | `crates/kinetics-garment-draping/src/lib.rs:141` | - |  |
| `impl DVec2::new` | 2 | `crates/kinetics-garment-draping/src/math.rs:18` | - |  |
| `impl DVec2::dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:23` | - |  |
| `impl DVec2::length_squared` | 1 | `crates/kinetics-garment-draping/src/math.rs:28` | - |  |
| `impl DVec2::length` | 1 | `crates/kinetics-garment-draping/src/math.rs:33` | - |  |
| `impl DVec2::distance` | 2 | `crates/kinetics-garment-draping/src/math.rs:38` | - |  |
| `impl DVec2::lerp` | 3 | `crates/kinetics-garment-draping/src/math.rs:43` | - |  |
| `impl DVec2::is_finite` | 1 | `crates/kinetics-garment-draping/src/math.rs:51` | - |  |
| `impl Add for DVec2::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:59` | - |  |
| `impl Sub for DVec2::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:67` | - |  |
| `impl Mul<f64> for DVec2::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:75` | - |  |
| `impl Div<f64> for DVec2::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:83` | - |  |
| `impl DVec3::new` | 3 | `crates/kinetics-garment-draping/src/math.rs:100` | - |  |
| `impl DVec3::dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:105` | - |  |
| `impl DVec3::cross` | 2 | `crates/kinetics-garment-draping/src/math.rs:110` | - |  |
| `impl DVec3::length_squared` | 1 | `crates/kinetics-garment-draping/src/math.rs:119` | - |  |
| `impl DVec3::length` | 1 | `crates/kinetics-garment-draping/src/math.rs:124` | - |  |
| `impl DVec3::normalize` | 1 | `crates/kinetics-garment-draping/src/math.rs:129` | - |  |
| `impl DVec3::is_finite` | 1 | `crates/kinetics-garment-draping/src/math.rs:139` | - |  |
| `impl Add for DVec3::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:147` | - |  |
| `impl AddAssign for DVec3::add_assign` | 2 | `crates/kinetics-garment-draping/src/math.rs:154` | - |  |
| `impl Sub for DVec3::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:164` | - |  |
| `impl SubAssign for DVec3::sub_assign` | 2 | `crates/kinetics-garment-draping/src/math.rs:171` | - |  |
| `impl Neg for DVec3::neg` | 1 | `crates/kinetics-garment-draping/src/math.rs:181` | - |  |
| `impl Mul<f64> for DVec3::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:189` | - |  |
| `impl Div<f64> for DVec3::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:197` | - |  |
| `impl DMat2::from_cols` | 2 | `crates/kinetics-garment-draping/src/math.rs:210` | - |  |
| `impl DMat2::determinant` | 1 | `crates/kinetics-garment-draping/src/math.rs:214` | - |  |
| `impl DMat2::inverse` | 1 | `crates/kinetics-garment-draping/src/math.rs:218` | - |  |
| `trait Scalar::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:240` | - |  |
| `trait Scalar::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:241` | - |  |
| `trait Scalar::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:242` | - |  |
| `trait Scalar::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:243` | - |  |
| `impl Scalar for f64::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:248` | - |  |
| `impl Scalar for f64::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:252` | - |  |
| `impl Scalar for f64::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:256` | - |  |
| `impl Scalar for f64::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:260` | - |  |
| `impl Jet<N>::variable` | 2 | `crates/kinetics-garment-draping/src/math.rs:274` | - |  |
| `impl Jet<N>::chain` | 4 | `crates/kinetics-garment-draping/src/math.rs:280` | - |  |
| `impl Scalar for Jet<N>::c` | 1 | `crates/kinetics-garment-draping/src/math.rs:292` | - |  |
| `impl Scalar for Jet<N>::value` | 1 | `crates/kinetics-garment-draping/src/math.rs:300` | - |  |
| `impl Scalar for Jet<N>::sqrt` | 1 | `crates/kinetics-garment-draping/src/math.rs:304` | - |  |
| `impl Scalar for Jet<N>::ln` | 1 | `crates/kinetics-garment-draping/src/math.rs:309` | - |  |
| `impl Add for Jet<N>::add` | 2 | `crates/kinetics-garment-draping/src/math.rs:316` | - |  |
| `impl Neg for Jet<N>::neg` | 1 | `crates/kinetics-garment-draping/src/math.rs:327` | - |  |
| `impl Sub for Jet<N>::sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:334` | - |  |
| `impl Mul for Jet<N>::mul` | 2 | `crates/kinetics-garment-draping/src/math.rs:341` | - |  |
| `impl Div for Jet<N>::div` | 2 | `crates/kinetics-garment-draping/src/math.rs:359` | - |  |
| `dot` | 2 | `crates/kinetics-garment-draping/src/math.rs:366` | - |  |
| `sub` | 2 | `crates/kinetics-garment-draping/src/math.rs:371` | - |  |
| `add` | 2 | `crates/kinetics-garment-draping/src/math.rs:376` | - |  |
| `cross` | 2 | `crates/kinetics-garment-draping/src/math.rs:381` | - |  |
| `impl Default for NewtonSettings::default` | 0 | `crates/kinetics-garment-draping/src/newton.rs:20` | - |  |
| `newton_step` | 2 | `crates/kinetics-garment-draping/src/newton.rs:41` | physics, computational geometry | Executes one implicit Euler step via regularized Newton-PCG. |
| `dot_vec` | 2 | `crates/kinetics-garment-draping/src/newton.rs:201` | - |  |
| `eval_cubic_bezier` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:8` | - | Cubic Bézier curve evaluator. |
| `flatten_cubic_bezier` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:17` | - | Adaptive subdivision of a cubic Bézier curve within given tolerance in meters. |
| `split` | 4 | `crates/kinetics-garment-draping/src/pattern.rs:19` | - |  |
| `polyline_arclength_point` | 2 | `crates/kinetics-garment-draping/src/pattern.rs:50` | - | Evaluates a point along an arc-length polyline at a normalized fraction in [0, 1]. |
| `impl Default for VbdSettings::default` | 0 | `crates/kinetics-garment-draping/src/vbd.rs:17` | - |  |
| `impl ClothMesh::new` | 4 | `crates/kinetics-garment-draping/src/vbd.rs:40` | - |  |
| `vbd_step` | 2 | `crates/kinetics-garment-draping/src/vbd.rs:58` | computational geometry | Executes one VBD simulation step. |

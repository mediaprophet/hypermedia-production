# `crates/profile-authoring`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Bounded metric profile authoring. Source, units and project history remain host-owned._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl std::fmt::Display for AuthoringError::fmt` | 2 | `crates/profile-authoring/provenance/original-lib.rs:20` | - |  |
| `finite_positive` | 1 | `crates/profile-authoring/provenance/original-lib.rs:28` | - |  |
| `triangle` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:3` | computational geometry |  |
| `part` | 2 | `crates/profile-authoring/provenance/original-spatial-tests.rs:11` | - |  |
| `volume` | 1 | `crates/profile-authoring/provenance/original-spatial-tests.rs:25` | computational geometry |  |
| `concave_channel_caps_preserve_wall_void_and_volume` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:38` | - |  |
| `changed_profile_parameters_change_real_geometry` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:65` | - |  |
| `parent_relative_parts_retain_semantics_material_ranges_and_flat_frames` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:96` | - |  |
| `authored_normals_use_full_world_inverse_transpose_and_uvs_survive` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:112` | - |  |
| `invalid_assemblies_and_expanded_output_budget_fail_without_mutating_inputs` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:132` | - |  |
| `malformed_profiles_and_primitive_budgets_are_rejected` | 0 | `crates/profile-authoring/provenance/original-spatial-tests.rs:151` | - |  |
| `impl Default for Transform::default` | 0 | `crates/profile-authoring/src/assembly.rs:18` | - |  |
| `impl Transform::matrix` | 1 | `crates/profile-authoring/src/assembly.rs:28` | 3d, numerics |  |
| `impl Default for AssemblyLimits::default` | 0 | `crates/profile-authoring/src/assembly.rs:69` | - |  |
| `preflight` | 2 | `crates/profile-authoring/src/assembly.rs:98` | - |  |
| `compile_assembly` | 2 | `crates/profile-authoring/src/assembly.rs:164` | - |  |
| `unit` | 1 | `crates/profile-authoring/src/frames.rs:9` | - |  |
| `record` | 2 | `crates/profile-authoring/src/frames.rs:25` | 3d |  |
| `normal` | 2 | `crates/profile-authoring/src/frames.rs:46` | 3d |  |
| `face` | 3 | `crates/profile-authoring/src/frames.rs:57` | - |  |
| `impl Default for GeometryLimits::default` | 0 | `crates/profile-authoring/src/geometry.rs:20` | - |  |
| `impl GeometryLimits::admit` | 4 | `crates/profile-authoring/src/geometry.rs:31` | - |  |
| `mesh` | 2 | `crates/profile-authoring/src/geometry.rs:52` | computational geometry |  |
| `profile_sweep` | 4 | `crates/profile-authoring/src/geometry.rs:82` | computational geometry | A metric profile swept along a finite 3-D axis. Concave end caps reuse the |
| `sphere` | 4 | `crates/profile-authoring/src/geometry.rs:201` | computational geometry |  |
| `torus` | 5 | `crates/profile-authoring/src/geometry.rs:230` | computational geometry |  |
| `impl std::fmt::Display for AuthoringError::fmt` | 2 | `crates/profile-authoring/src/lib.rs:24` | - |  |
| `rounded_rect` | 4 | `crates/profile-authoring/src/lib.rs:39` | 2d |  |
| `circle` | 2 | `crates/profile-authoring/src/lib.rs:48` | - |  |
| `closed_polygon` | 1 | `crates/profile-authoring/src/lib.rs:52` | computational geometry |  |
| `finite_positive` | 1 | `crates/profile-authoring/src/lib.rs:56` | - |  |
| `impl Default for ProfileLimits::default` | 0 | `crates/profile-authoring/src/profile.rs:36` | - |  |
| `impl ProfileDoc::replace` | 4 | `crates/profile-authoring/src/profile.rs:53` | - | Pure successor, never a second document store or undo journal. |
| `arc` | 7 | `crates/profile-authoring/src/profile.rs:74` | 2d |  |
| `impl Profile::points` | 2 | `crates/profile-authoring/src/profile.rs:94` | - |  |
| `triangle` | 0 | `crates/profile-authoring/src/tests.rs:5` | computational geometry |  |
| `part` | 2 | `crates/profile-authoring/src/tests.rs:14` | - |  |
| `volume` | 1 | `crates/profile-authoring/src/tests.rs:29` | computational geometry |  |
| `malformed_profiles_are_rejected` | 0 | `crates/profile-authoring/src/tests.rs:43` | - |  |
| `changed_profile_parameters_change_section_dimensions_and_area` | 0 | `crates/profile-authoring/src/tests.rs:60` | - |  |
| `open_channel_section_retains_wall_void_and_refuses_caller_budget` | 0 | `crates/profile-authoring/src/tests.rs:97` | - |  |
| `concave_channel_caps_preserve_wall_void_and_volume` | 0 | `crates/profile-authoring/src/tests.rs:122` | - |  |
| `rounded_profile_edit_changes_real_geometry_and_rejects_stale_revision` | 0 | `crates/profile-authoring/src/tests.rs:150` | - |  |
| `parent_relative_parts_retain_semantics_material_ranges_and_flat_frames` | 0 | `crates/profile-authoring/src/tests.rs:196` | - |  |
| `authored_normals_use_full_world_inverse_transpose_and_uvs_survive` | 0 | `crates/profile-authoring/src/tests.rs:213` | - |  |
| `invalid_assemblies_and_expanded_output_budget_fail_without_mutating_inputs` | 0 | `crates/profile-authoring/src/tests.rs:234` | - |  |
| `malformed_profiles_and_primitive_budgets_are_rejected` | 0 | `crates/profile-authoring/src/tests.rs:254` | - |  |

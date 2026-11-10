# `crates/raster-selection-morphology`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Euclidean morphology for caller-owned, row-major selection coverage masks.

Use the checked `try_*` operations to validate finite coverage, dimensions, radius,
and caller-selected work and logical-payload budgets before allocating. The returned
values remain the responsibility of the caller's authoritative editable image state.
This crate owns no document, selection identity, history, canvas-boundary policy,
persistent mask format, or compositor._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/raster-selection-morphology/examples/modify_selection.rs:3` | - |  |
| `edt` | 3 | `crates/raster-selection-morphology/src/distance.rs:15` | 2d | Euclidean distance to the nearest `true` pixel, in row-major order. |
| `for_chunks` | 3 | `crates/raster-selection-morphology/src/distance.rs:75` | - |  |
| `row_distance` | 4 | `crates/raster-selection-morphology/src/distance.rs:88` | - | Lower envelope of parabolas with integer vertical distances. Work is O(row length): each |
| `brute` | 3 | `crates/raster-selection-morphology/src/distance/tests.rs:3` | - |  |
| `matches_brute_force_for_every_small_mask` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:20` | physics |  |
| `rectangular_and_partial_bands_match_the_previous_transform` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:30` | - |  |
| `large_coordinates_keep_nearby_parabolas_distinct` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:47` | - |  |
| `selected_pixels_stay_at_zero_past_coordinate_4096` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:66` | - |  |
| `inconsistent_and_empty_dimensions_do_not_panic` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:73` | - |  |
| `selection_modifiers_keep_fractional_coverage` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:82` | - |  |
| `results_do_not_depend_on_the_worker_count` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:102` | - |  |
| `distance_transform_release_comparison` | 0 | `crates/raster-selection-morphology/src/distance/tests.rs:116` | - |  |
| `old_dt1` | 4 | `crates/raster-selection-morphology/src/distance/tests.rs:191` | - | 1D squared distance transform (Felzenszwalb & Huttenlocher). |
| `old_edt` | 3 | `crates/raster-selection-morphology/src/distance/tests.rs:229` | 2d | Euclidean distance to the nearest `true` pixel. |
| `impl MorphologyKernel::square` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:35` | - | Square structuring element of width `2 * radius + 1`. |
| `impl MorphologyKernel::disk` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:47` | - | Circular disk structuring element of given floating-point radius. |
| `impl MorphologyKernel::diamond` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:74` | - | Diamond structuring element with Manhattan radius `\|dx\| + \|dy\| <= radius`. |
| `impl MorphologyKernel::cross` | 1 | `crates/raster-selection-morphology/src/grayscale.rs:99` | - | Cross structuring element (`dx == 0 \|\| dy == 0`). |
| `impl MorphologyKernel::rectangle` | 2 | `crates/raster-selection-morphology/src/grayscale.rs:124` | - | Flat rectangular structuring element. |
| `impl MorphologyKernel::from_slice` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:137` | - | Custom structuring element from row-major slice. |
| `sample_clamped` | 5 | `crates/raster-selection-morphology/src/grayscale.rs:152` | - |  |
| `grayscale_dilate` | 4 | `crates/raster-selection-morphology/src/grayscale.rs:159` | - | Grayscale morphological dilation: `out(x, y) = max (input + kernel)`. |
| `grayscale_erode` | 4 | `crates/raster-selection-morphology/src/grayscale.rs:193` | - | Grayscale morphological erosion: `out(x, y) = min (input - kernel)`. |
| `apply_morphology` | 5 | `crates/raster-selection-morphology/src/grayscale.rs:227` | - | Applies a morphological method to a 2D grayscale raster. |
| `distance_transform_f32` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:270` | - | Euclidean distance transform on a thresholded raster mask (`>= 0.5` inside). |
| `skeletonize_mask` | 3 | `crates/raster-selection-morphology/src/grayscale.rs:281` | 2d, graph | Topological skeletonization via the Zhang-Suen thinning algorithm. |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-selection-morphology/src/lib.rs:66` | - |  |
| `preflight_dimensions` | 5 | `crates/raster-selection-morphology/src/lib.rs:73` | - | Validates shape, radius, and caller-selected limits before any buffer conversion/allocation. |
| `preflight` | 6 | `crates/raster-selection-morphology/src/lib.rs:136` | - | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm buffers. |
| `edt_scratch_bound` | 3 | `crates/raster-selection-morphology/src/lib.rs:157` | - |  |
| `try_expand` | 5 | `crates/raster-selection-morphology/src/lib.rs:182` | - | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `try_contract` | 5 | `crates/raster-selection-morphology/src/lib.rs:194` | - | Checked Euclidean contraction. |
| `try_border` | 5 | `crates/raster-selection-morphology/src/lib.rs:206` | graph | Checked soft band on both sides of the mask edge. |
| `expand_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:217` | - |  |
| `contract_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:226` | - |  |
| `border_impl` | 4 | `crates/raster-selection-morphology/src/lib.rs:234` | - |  |
| `par_rows` | 4 | `crates/raster-selection-morphology/src/lib.rs:251` | - |  |
| `square` | 2 | `crates/raster-selection-morphology/src/tests.rs:6` | - |  |
| `expand_contract_border` | 0 | `crates/raster-selection-morphology/src/tests.rs:19` | - |  |
| `border_matches_measured_oracle` | 0 | `crates/raster-selection-morphology/src/tests.rs:33` | - |  |
| `fractional_radii_follow_coverage_rules` | 0 | `crates/raster-selection-morphology/src/tests.rs:60` | - |  |
| `preflight_rejects_shape_nonfinite_coverage_and_limits` | 0 | `crates/raster-selection-morphology/src/tests.rs:81` | - |  |
| `zero_radius_keeps_source_operation_behavior` | 0 | `crates/raster-selection-morphology/src/tests.rs:202` | - |  |
| `grayscale_morphology_methods_behave_as_expected` | 0 | `crates/raster-selection-morphology/src/tests.rs:215` | - |  |
| `distance_transform_and_skeletonization_work` | 0 | `crates/raster-selection-morphology/src/tests.rs:243` | - |  |

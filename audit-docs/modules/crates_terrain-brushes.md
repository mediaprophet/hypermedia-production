# `crates/terrain-brushes`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Copy-on-Write (`Arc<Vec<f32>>`) tile heightfield snapshot & delta undo/redo stack
with a local 64-byte in-memory StrokeSample journal; no wire format is implied.

Adapted from EarthCraft-main v0.2.0 under MIT License.
Copyright (c) EarthCraft contributors.
Licensed under the MIT License._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:10` | - |  |
| `impl CowTerrainTileGrid::tile_size` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:25` | - |  |
| `impl CowTerrainTileGrid::cell_size_m` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:26` | - |  |
| `impl CowTerrainTileGrid::byte_budget` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:27` | - |  |
| `impl CowTerrainTileGrid::is_stroke_active` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:28` | - |  |
| `impl CowTerrainTileGrid::tiles` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:29` | - |  |
| `impl CowTerrainTileGrid::undo_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:30` | - |  |
| `impl CowTerrainTileGrid::redo_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:31` | - |  |
| `impl CowTerrainTileGrid::active_samples` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:32` | - |  |
| `impl CowTerrainTileGrid::tile_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:34` | - |  |
| `impl CowTerrainTileGrid::admit` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:39` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/adaptation/grid-impl.rs:49` | - |  |
| `impl CowTerrainTileGrid::load_tile` | 4 | `crates/terrain-brushes/adaptation/grid-impl.rs:65` | - | Import one already-decoded finite height tile before local history begins. |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:85` | - |  |
| `impl CowTerrainTileGrid::validate_sample` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:93` | - |  |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:114` | - | Prepare all touched tile successors, then install atomically for this sample. |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/adaptation/grid-impl.rs:188` | - | Commit preparation refuses without changing preview, redo, IDs or history. |
| `impl CowTerrainTileGrid::cancel_stroke` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:220` | - | Restore the precise original tile Arcs without committing or changing history. |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:228` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:239` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:250` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/adaptation/grid-impl.rs:265` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/adaptation/grid-impl.rs:270` | - |  |
| `impl CowTerrainTileGrid::discard_local_history` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:274` | - | Deliberately discard local undo/redo history, retaining current tile data and Arcs. |
| `impl CowTerrainTileGrid::payload_memory_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:283` | - | Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes. |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/adaptation/grid-impl.rs:285` | - |  |
| `main` | 0 | `crates/terrain-brushes/examples/local-stroke.rs:4` | - |  |
| `impl std::fmt::Display for GeoStudioError::fmt` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:24` | - |  |
| `impl BrushFalloff::from_code` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:49` | - |  |
| `impl BrushFalloff::compute_weight` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:59` | - |  |
| `impl BrushOperation::from_sample` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:93` | - |  |
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:158` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:170` | - |  |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:182` | - |  |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:190` | - |  |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:245` | - |  |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:274` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:283` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:292` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:298` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:303` | - |  |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:307` | - |  |
| `apply_op_to_vertex` | 7 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:325` | 3d |  |
| `make_sample` | 7 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:364` | - |  |
| `stroke_sample_is_64_bytes` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:373` | - |  |
| `cow_untouched_tiles_zero_copy` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:378` | - |  |
| `exact_undo_redo_multi_tile_restoration` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:402` | - |  |
| `deterministic_replay_stroke_journal_and_byte_budget_fail_closed` | 0 | `crates/terrain-brushes/provenance/original-cow_delta_undo.rs:435` | - |  |
| `sample` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:3` | - |  |
| `smoothing_uses_one_input_snapshot_and_preserves_sequential_samples` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:12` | - |  |
| `default_neighborhood_is_legacy_for_existing_journal_callers` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:48` | - |  |
| `hydraulic_drop_is_symmetric_within_one_sample` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:54` | - |  |
| `grid` | 1 | `crates/terrain-brushes/src/adaptation_tests.rs:73` | - |  |
| `dimensions_and_initial_height_refuse_before_allocation_or_mutation` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:80` | - |  |
| `decoded_source_import_preserves_heights_and_refuses_history_replacement` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:93` | - |  |
| `invalid_and_unsupported_samples_preserve_preview_and_journal` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:110` | - |  |
| `cow_and_numeric_budget_refusals_are_atomic` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:140` | - |  |
| `cancel_and_failed_commit_preserve_redo_and_exact_tile_baseline` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:160` | - |  |
| `replay_failure_cancels_all_prior_samples_and_does_not_advance_id` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:181` | - |  |
| `commit_identifier_exhaustion_does_not_destroy_active_preview` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:196` | - |  |
| `late_second_tile_failure_leaves_all_tiles_journal_and_prior_history_unchanged` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:210` | - |  |
| `full_history_accounting_is_bounded_and_explicit_discard_restores_commit_capacity` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:238` | - |  |
| `explicit_history_discard_drops_redo_without_changing_accepted_heights_or_ids` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:261` | - |  |
| `conservative_history_reference_budget_refuses_commit_without_destroying_preview` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:275` | - |  |
| `admitted_operations_match_original_same_platform_float_behavior` | 0 | `crates/terrain-brushes/src/adaptation_tests.rs:290` | - |  |
| `impl std::fmt::Display for GeoStudioError::fmt` | 2 | `crates/terrain-brushes/src/lib.rs:30` | - |  |
| `impl BrushFalloff::from_code` | 1 | `crates/terrain-brushes/src/lib.rs:55` | - |  |
| `impl BrushFalloff::compute_weight` | 3 | `crates/terrain-brushes/src/lib.rs:65` | - |  |
| `impl BrushOperation::from_sample` | 1 | `crates/terrain-brushes/src/lib.rs:99` | - |  |
| `impl CowTerrainTileGrid::new` | 3 | `crates/terrain-brushes/src/lib.rs:185` | - |  |
| `impl CowTerrainTileGrid::new_with_neighborhood_sampling` | 4 | `crates/terrain-brushes/src/lib.rs:190` | - | Explicitly select neighbourhood semantics for new work or known-version replay. |
| `impl CowTerrainTileGrid::tile_size` | 1 | `crates/terrain-brushes/src/lib.rs:205` | - |  |
| `impl CowTerrainTileGrid::cell_size_m` | 1 | `crates/terrain-brushes/src/lib.rs:206` | - |  |
| `impl CowTerrainTileGrid::byte_budget` | 1 | `crates/terrain-brushes/src/lib.rs:207` | - |  |
| `impl CowTerrainTileGrid::neighborhood_sampling` | 1 | `crates/terrain-brushes/src/lib.rs:208` | - |  |
| `impl CowTerrainTileGrid::is_stroke_active` | 1 | `crates/terrain-brushes/src/lib.rs:209` | - |  |
| `impl CowTerrainTileGrid::tiles` | 1 | `crates/terrain-brushes/src/lib.rs:210` | - |  |
| `impl CowTerrainTileGrid::undo_history` | 1 | `crates/terrain-brushes/src/lib.rs:211` | - |  |
| `impl CowTerrainTileGrid::redo_history` | 1 | `crates/terrain-brushes/src/lib.rs:212` | - |  |
| `impl CowTerrainTileGrid::active_samples` | 1 | `crates/terrain-brushes/src/lib.rs:213` | - |  |
| `impl CowTerrainTileGrid::tile_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:215` | - |  |
| `impl CowTerrainTileGrid::admit` | 2 | `crates/terrain-brushes/src/lib.rs:220` | - |  |
| `impl CowTerrainTileGrid::ensure_tile` | 4 | `crates/terrain-brushes/src/lib.rs:230` | - |  |
| `impl CowTerrainTileGrid::load_tile` | 4 | `crates/terrain-brushes/src/lib.rs:246` | - | Import one already-decoded finite height tile before local history begins. |
| `impl CowTerrainTileGrid::begin_stroke` | 1 | `crates/terrain-brushes/src/lib.rs:266` | - |  |
| `impl CowTerrainTileGrid::validate_sample` | 1 | `crates/terrain-brushes/src/lib.rs:274` | - |  |
| `impl CowTerrainTileGrid::apply_stroke_sample` | 2 | `crates/terrain-brushes/src/lib.rs:295` | - | Prepare all touched tile successors, then install atomically for this sample. |
| `impl CowTerrainTileGrid::commit_stroke` | 2 | `crates/terrain-brushes/src/lib.rs:373` | - | Commit preparation refuses without changing preview, redo, IDs or history. |
| `impl CowTerrainTileGrid::cancel_stroke` | 1 | `crates/terrain-brushes/src/lib.rs:405` | - | Restore the precise original tile Arcs without committing or changing history. |
| `impl CowTerrainTileGrid::undo` | 1 | `crates/terrain-brushes/src/lib.rs:413` | - |  |
| `impl CowTerrainTileGrid::redo` | 1 | `crates/terrain-brushes/src/lib.rs:424` | - |  |
| `impl CowTerrainTileGrid::replay_stroke_journal` | 3 | `crates/terrain-brushes/src/lib.rs:435` | - |  |
| `impl CowTerrainTileGrid::sample_height` | 5 | `crates/terrain-brushes/src/lib.rs:450` | - |  |
| `impl CowTerrainTileGrid::tile_arc` | 3 | `crates/terrain-brushes/src/lib.rs:455` | - |  |
| `impl CowTerrainTileGrid::discard_local_history` | 1 | `crates/terrain-brushes/src/lib.rs:459` | - | Deliberately discard local undo/redo history, retaining current tile data and Arcs. |
| `impl CowTerrainTileGrid::payload_memory_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:468` | - | Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes. |
| `impl CowTerrainTileGrid::estimate_memory_bytes` | 1 | `crates/terrain-brushes/src/lib.rs:470` | - |  |
| `sample_height` | 3 | `crates/terrain-brushes/src/lib.rs:487` | - |  |
| `apply_op_to_vertex` | 8 | `crates/terrain-brushes/src/lib.rs:491` | 3d |  |
| `make_sample` | 7 | `crates/terrain-brushes/src/lib.rs:530` | - |  |
| `stroke_sample_is_64_bytes` | 0 | `crates/terrain-brushes/src/lib.rs:539` | - |  |
| `cow_untouched_tiles_zero_copy` | 0 | `crates/terrain-brushes/src/lib.rs:544` | - |  |
| `exact_undo_redo_multi_tile_restoration` | 0 | `crates/terrain-brushes/src/lib.rs:568` | - |  |
| `deterministic_replay_stroke_journal_and_byte_budget_fail_closed` | 0 | `crates/terrain-brushes/src/lib.rs:601` | - |  |

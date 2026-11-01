# Source adaptation and test map

Original: current Poet geo_studio/cow_delta_undo.rs, preserved whole and pinned.
Source-origin MIT declaration and actual EarthCraft LICENSE are retained. No git
revision was queried; paths and SHA-256 identify exact bytes.

Preserved byte-for-byte sections: BrushFalloff/BrushOperation/StrokeSample and
delta/commit record definitions, apply_op_to_vertex, and all four original source
tests. The top misleading StrokeCommand wire claim is corrected to local sample
wording. Grid fields are private; error variants remain unchanged. The grid impl
is replaced by the adaptation/grid-impl.rs text and the source is assembled once
into src/lib.rs. Future source edits operate on that installed lib; the adaptation
file records the initial migration delta and is not an included runtime module.

| Source defect / behavior | Adaptation and active regression |
|---|---|
| tile_size² unchecked; data allocated before budget; nonfinite initial samples | Constructor tile/finite-span bounds and ensure_tile admission before data allocation. dimensions_and_initial_height_refuse_before_allocation_or_mutation. |
| public mutable tiles/dimensions/stacks permit invalid length and metadata changes | Private fields with tiles()/tile_arc/sample_height/undo_history/redo_history/active_samples read APIs. Initial height and structural mutation checked; new tiles refused while stroke active. |
| no safe import seam once mutable state is encapsulated | load_tile accepts an exactly sized finite decoded metre tile only before local history; decoded_source_import_preserves_heights_and_refuses_history_replacement. Canonical decoding/nodata/source metadata stay outside the library. |
| NaN/unsupported op/falloff/layer/flags/padding silently accepted | Explicit validation before edit; invalid_and_unsupported_samples_preserve_preview_and_journal. Legacy decode helpers remain available but apply never admits unknown codes. |
| input coordinate rectangle iteration can traverse billions of absent tiles | Filter/sort only existing tile keys; conservative per-sample full-tile vertex work limit. Bounds use f64 for tile selection; inherited f32 sample/vertex formula stays. Boundary-rounding differences need host large-coordinate qualification. |
| COW payload can exceed budget during apply; numeric output can become nonfinite | Admit conservative successor tile/sample payload; prepare all affected successors and validate finite results before installing. cow_and_numeric_budget_refusals_are_atomic. |
| commit increments ID, clears redo, drains active state before admission | Checked ID/label/history and fallible preparation happen before logical state installation. cancel_and_failed_commit_preserve_redo_and_exact_tile_baseline and commit_identifier_exhaustion_does_not_destroy_active_preview. |
| no cancel operation | cancel_stroke restores baseline Arc identity and leaves local history/IDs unchanged. No tile creation during active stroke. |
| replay failure leaves partial active edits | On sample or commit refusal, replay cancels its entire stroke. replay_failure_cancels_all_prior_samples_and_does_not_advance_id. |
| memory estimator unchecked sums, history-patch walk/HashSet scratch and overbroad byte-budget wording | Saturating conservative tile-reference/sample accounting; no scratch allocation and at most 256 history-entry length reads, never individual patch/sample traversal. Duplicate refs are deliberately charged. Commit admits added history reference charge before install. Scope exclusions explicit in README; no total-heap claim. |
| full history cap had no public recovery action | discard_local_history drops undo+redo, retains current tile Arcs/heights, refuses during active stroke and preserves next ID. Full-cap recovery/redo and conservative commit-budget regressions added. |
| late later-tile failure only inspected | late_second_tile_failure_leaves_all_tiles_journal_and_prior_history_unchanged prepares a finite first tile and overflowing second tile in sorted order, checking both Arc identities, sample count and existing redo history. |
| deterministic source behavior insufficiently qualified | admitted_operations_match_original_same_platform_float_behavior compares all six operation codes and four falloff codes on sequential admitted edits against the original module. Original four tests remain active and run again inside the source oracle; duplicated tests are disclosed, not counted as distinct functionality. |

Still unchanged/unqualified: in-place scan-order smoothing (tile edge clamping),
local averaging named HydraulicDrop, no physical mass conservation, no global
height datum/nodata model, no halo/LOD seam contract, no codec or canonical replay
wire contract, no native author/reviewer UI workflow. Strength units remain
operation-specific. Source seed/timestamp/stroke_id metadata does not by itself
control nondeterministic external effects or grant authority. No such effects
exist in this library. Full native adoption needs a host adapter and explicit
Core/geospatial/simulation/resource mapping, rather than blanket engine claims.

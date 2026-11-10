# `crates/raster-layer-mask-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Checked conversion adapter between QualiaDB's layer-mask operations,
digital painting brush dynamics, and selection-morphology pipelines.

Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/raster-layer-mask-adapter/examples/edit_existing_layer_mask.rs:4` | - |  |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:55` | - |  |
| `preflight_dimensions` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:69` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `preflight` | 6 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:106` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `edt_scratch_bound` | 3 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:115` | - |  |
| `try_expand` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:134` | 2d | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `try_contract` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:140` | - | Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior. |
| `try_border` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:146` | graph | Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle. |
| `expand_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:151` | - |  |
| `contract_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:159` | - |  |
| `border_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:164` | - |  |
| `par_rows` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-after.rs:177` | - |  |
| `impl std::fmt::Display for PreflightError::fmt` | 2 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:55` | - |  |
| `preflight` | 6 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:67` | 2d | Validates shape, radius, coverage, and caller-selected limits without allocating algorithm |
| `edt_scratch_bound` | 3 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:100` | - |  |
| `try_expand` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:119` | 2d | Checked Euclidean expansion. Caller-selected limits are enforced before mask or EDT buffers |
| `try_contract` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:125` | - | Checked Euclidean contraction, preserving PhotoCraft's inversion/expansion/inversion behavior. |
| `try_border` | 5 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:131` | graph | Checked soft band on both sides of the mask edge, following PhotoCraft's measured source oracle. |
| `expand_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:136` | - |  |
| `contract_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:144` | - |  |
| `border_impl` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:149` | - |  |
| `par_rows` | 4 | `crates/raster-layer-mask-adapter/provenance/morphology-lib-before.rs:162` | - |  |
| `impl Default for StrokePoint::default` | 0 | `crates/raster-layer-mask-adapter/src/brush.rs:26` | - |  |
| `impl Dynamic::evaluate` | 4 | `crates/raster-layer-mask-adapter/src/brush.rs:72` | - |  |
| `impl Default for BrushSettings::default` | 0 | `crates/raster-layer-mask-adapter/src/brush.rs:141` | - |  |
| `pseudo_rand` | 3 | `crates/raster-layer-mask-adapter/src/brush.rs:157` | - |  |
| `generate_dabs` | 4 | `crates/raster-layer-mask-adapter/src/brush.rs:168` | - | Generates a list of dabs from discrete input points using brush spacing and dynamics. |
| `impl std::fmt::Display for AdapterError::fmt` | 2 | `crates/raster-layer-mask-adapter/src/lib.rs:55` | - |  |
| `preflight_mask` | 6 | `crates/raster-layer-mask-adapter/src/lib.rs:64` | - | Admits a checked edit without converting or allocating the source mask. |
| `modify_layer_mask` | 6 | `crates/raster-layer-mask-adapter/src/lib.rs:106` | - | Transform the current U8 layer mask and return replacement bytes only after |
| `scale_mask_density` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:12` | - | Scales layer mask density in-place or returns scaled copy. |
| `feather_mask` | 4 | `crates/raster-layer-mask-adapter/src/masks.rs:29` | - | Applies separable 2D Gaussian feathering to an 8-bit mask buffer. |
| `propagate_clipping_mask_alpha` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:86` | - | Propagates clipping mask base alpha onto a clipped layer's alpha buffer. |
| `evaluate_knockout` | 7 | `crates/raster-layer-mask-adapter/src/masks.rs:106` | 2d, numerics | Evaluates knockout composition for a single straight-alpha RGBA pixel. |
| `impl Default for BlendRange::default` | 0 | `crates/raster-layer-mask-adapter/src/masks.rs:162` | - |  |
| `impl BlendRange::is_full` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:173` | - |  |
| `impl BlendRange::from_bytes` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:177` | - |  |
| `impl BlendRange::to_bytes` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:184` | - |  |
| `impl BlendRange::weight` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:189` | - | Evaluates visibility weight (0.0..=1.0) for a sample `v` in `0.0..=255.0`. |
| `impl BlendIf::is_default` | 1 | `crates/raster-layer-mask-adapter/src/masks.rs:212` | - |  |
| `impl BlendIf::get` | 2 | `crates/raster-layer-mask-adapter/src/masks.rs:216` | - |  |
| `impl BlendIf::set` | 3 | `crates/raster-layer-mask-adapter/src/masks.rs:220` | - |  |
| `impl BlendIf::evaluate` | 5 | `crates/raster-layer-mask-adapter/src/masks.rs:231` | 2d | Evaluates effective Blend If multiplier for a pixel. |
| `impl Default for MixerSettings::default` | 0 | `crates/raster-layer-mask-adapter/src/mixer.rs:28` | - |  |
| `impl MixerState::new` | 0 | `crates/raster-layer-mask-adapter/src/mixer.rs:50` | - |  |
| `impl MixerState::with_reservoir` | 1 | `crates/raster-layer-mask-adapter/src/mixer.rs:54` | - |  |
| `impl MixerState::load` | 2 | `crates/raster-layer-mask-adapter/src/mixer.rs:62` | - |  |
| `impl MixerState::clean` | 1 | `crates/raster-layer-mask-adapter/src/mixer.rs:67` | - |  |
| `mix_dab` | 3 | `crates/raster-layer-mask-adapter/src/mixer.rs:75` | 2d | Simulates a single dab of the wet mixer brush over a sampled canvas pixel. |
| `impl NQuin48::new` | 5 | `crates/raster-layer-mask-adapter/src/ontology.rs:15` | - |  |
| `impl NQuin48::precondition` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:19` | - |  |
| `impl NQuin48::postcondition` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:23` | - |  |
| `impl NQuin48::invariant` | 3 | `crates/raster-layer-mask-adapter/src/ontology.rs:27` | - |  |
| `limits` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:5` | - |  |
| `missing_layer_mask_means_full_coverage_and_returns_replacement_bytes` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:20` | - |  |
| `source_border_zero_radius_and_mask_length_refusal_are_preserved` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:28` | - |  |
| `aggregate_limits_include_conversion_and_quantization` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:39` | - |  |
| `test_mask_density_and_feathering` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:51` | - |  |
| `test_blend_if_ramps` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:75` | - |  |
| `test_wet_mixer_brush_strokes` | 0 | `crates/raster-layer-mask-adapter/src/tests.rs:100` | - |  |

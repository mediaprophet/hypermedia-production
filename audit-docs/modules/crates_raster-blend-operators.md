# `crates/raster-blend-operators`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure channel blend operators, HDRI operators, and W3C/PDF blend modes.

Inputs are expected in the normalized channel range `[0, 1]` or HDR floating-point ranges.
This crate does not own persistent image formats, document state, or display pipelines._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `blend_channel` | 3 | `crates/raster-blend-operators/src/lib.rs:20` | - | Evaluates one operator for one backdrop/source channel pair. |
| `color_burn_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:36` | - | Byte-quantized Color Burn operator in `0..=255`. |
| `color_dodge_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:43` | - | Byte-quantized Color Dodge operator in `0..=255`. |
| `vivid_light_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:50` | - | Byte-quantized Vivid Light operator in `0..=255`. |
| `hard_mix_u8` | 2 | `crates/raster-blend-operators/src/lib.rs:57` | - | Byte-quantized Hard Mix operator in `0..=255`. |
| `blend_operator_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:68` | - | Evaluates a `BlendOperator` independently for three RGB channels. |
| `impl BlendMode::label` | 1 | `crates/raster-blend-operators/src/lib.rs:122` | - |  |
| `impl BlendMode::group` | 1 | `crates/raster-blend-operators/src/lib.rs:144` | - | Index of the separator group (0..=5). |
| `impl BlendMode::is_separable` | 1 | `crates/raster-blend-operators/src/lib.rs:156` | - | Whether the mode works on each colour channel separately (all but Hue, Saturation, Color, Luminosity). |
| `impl BlendMode::parse` | 1 | `crates/raster-blend-operators/src/lib.rs:161` | - | Parse a label or identifier, case/space-insensitively. |
| `blend_mode_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:175` | - | Evaluates a `BlendMode` on straight (not premultiplied) RGB in `[0, 1]`. |
| `trait BlendTarget::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:229` | - |  |
| `impl BlendTarget for BlendOperator::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:233` | - |  |
| `impl BlendTarget for BlendMode::apply_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:239` | - |  |
| `blend_rgb` | 3 | `crates/raster-blend-operators/src/lib.rs:245` | - | Evaluates an operator or blend mode independently across RGB channels. |
| `composite` | 3 | `crates/raster-blend-operators/src/lib.rs:250` | - | Source-over compositing of `source` onto `backdrop` with `mode` (straight RGBA in `0..=1`). |
| `cmyk_planes` | 1 | `crates/raster-blend-operators/src/lib.rs:268` | - | Splits a CMYK colour into two additive planes for blending. |
| `planes_cmyk` | 2 | `crates/raster-blend-operators/src/lib.rs:273` | - | Combines blended CMY and K planes back into CMYK. |
| `linear_dodge_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:283` | numerics | HDRI Linear Dodge (Additive): `backdrop + source`. |
| `linear_burn_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:289` | numerics | HDRI Linear Burn: `backdrop + source - 1.0`. |
| `linear_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:295` | numerics | HDRI Linear Light: `backdrop + 2.0 * source - 1.0`. |
| `pin_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:301` | - | HDRI Pin Light: conditional min/max against thresholded source. |
| `pegtop_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:310` | - | HDRI Pegtop Soft Light approximation. |
| `soft_light_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:324` | - | W3C/PDF Soft Light channel blend function. |
| `exclusion_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:341` | - | HDRI Exclusion: `backdrop + source - 2.0 * backdrop * source`. |
| `difference_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:347` | - | HDRI Difference: `\|backdrop - source\|`. |
| `divide_f32` | 2 | `crates/raster-blend-operators/src/lib.rs:353` | - | HDRI Divide: `backdrop / source` (safeguarded against division by zero). |
| `linear_dodge_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:363` | numerics |  |
| `linear_burn_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:367` | numerics |  |
| `linear_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:371` | numerics |  |
| `pin_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:375` | - |  |
| `pegtop_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:379` | - |  |
| `soft_light_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:383` | - |  |
| `exclusion_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:387` | - |  |
| `difference_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:391` | - |  |
| `divide_rgb` | 2 | `crates/raster-blend-operators/src/lib.rs:395` | - |  |
| `vivid_light` | 2 | `crates/raster-blend-operators/src/lib.rs:403` | - |  |
| `color_burn` | 2 | `crates/raster-blend-operators/src/lib.rs:417` | - |  |
| `color_dodge` | 2 | `crates/raster-blend-operators/src/lib.rs:427` | - |  |
| `vivid_light_generic` | 2 | `crates/raster-blend-operators/src/lib.rs:437` | - |  |
| `screen` | 2 | `crates/raster-blend-operators/src/lib.rs:455` | - |  |
| `hard_light` | 2 | `crates/raster-blend-operators/src/lib.rs:459` | - |  |
| `lum` | 1 | `crates/raster-blend-operators/src/lib.rs:467` | - |  |
| `sat` | 1 | `crates/raster-blend-operators/src/lib.rs:471` | - |  |
| `clip_color` | 1 | `crates/raster-blend-operators/src/lib.rs:475` | - |  |
| `set_lum` | 2 | `crates/raster-blend-operators/src/lib.rs:496` | - |  |
| `set_sat` | 2 | `crates/raster-blend-operators/src/lib.rs:501` | - |  |
| `vivid_light_source_extremes_win` | 0 | `crates/raster-blend-operators/src/lib.rs:517` | - |  |
| `hard_mix_uses_generic_vivid_light_edge_rules` | 0 | `crates/raster-blend-operators/src/lib.rs:525` | graph |  |
| `burn_and_dodge_treat_near_extremes_as_exact` | 0 | `crates/raster-blend-operators/src/lib.rs:533` | - |  |
| `u8_operators_behave_consistently` | 0 | `crates/raster-blend-operators/src/lib.rs:541` | - |  |
| `hdri_operators_produce_expected_values` | 0 | `crates/raster-blend-operators/src/lib.rs:552` | - |  |
| `separable_modes_follow_the_reference_formulas` | 0 | `crates/raster-blend-operators/src/lib.rs:563` | - |  |
| `parse_labels_round_trip` | 0 | `crates/raster-blend-operators/src/lib.rs:602` | - |  |
| `vivid_light_ps` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:6` | - |  |
| `color_burn` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:21` | - |  |
| `color_dodge` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:31` | - |  |
| `vivid_light_generic` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:41` | - |  |
| `hard_mix_ps` | 2 | `crates/raster-blend-operators/src/migration_oracle.rs:59` | - |  |
| `selected_operators_are_bit_equal_to_source_on_normalized_grid` | 0 | `crates/raster-blend-operators/src/migration_oracle.rs:69` | - |  |
| `selected_operators_are_bit_equal_around_edges_midpoint_and_extremes` | 0 | `crates/raster-blend-operators/src/migration_oracle.rs:89` | - |  |
| `assert_bit_equal` | 4 | `crates/raster-blend-operators/src/migration_oracle.rs:121` | - |  |

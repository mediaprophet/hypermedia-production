# `crates/vector-compute-shaders`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_This is a utility library to help integrate the [Vello] shader modules into any renderer
project. It provides the necessary metadata to construct the individual compute pipelines
on any GPU API while leaving the responsibility of all API interactions
(such as resource management and command encoding) up to the client.

The shaders can be pre-compiled to any target shading language at build time based on
feature flags. Currently only WGSL and Metal Shading Language are supported.

Your first choice should be to use the build time generated [`SHADERS`].

Alternatively you could use runtime compiled shaders via the [`compile`] module
by enabling the `compile` feature. This option is useful for active development
as it enables you to do hot reloading of shaders.

[Vello]: https://github.com/linebender/vello_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/vector-compute-shaders/build.rs:20` | - |  |
| `write_types` | 2 | `crates/vector-compute-shaders/build.rs:49` | - |  |
| `write_shaders` | 2 | `crates/vector-compute-shaders/build.rs:58` | - |  |
| `write_msl` | 2 | `crates/vector-compute-shaders/build.rs:111` | - |  |
| `write_msl` | 2 | `crates/vector-compute-shaders/build.rs:116` | - |  |
| `impl log::Log for BuildScriptLog::enabled` | 2 | `crates/vector-compute-shaders/build.rs:146` | - |  |
| `impl log::Log for BuildScriptLog::log` | 2 | `crates/vector-compute-shaders/build.rs:150` | - |  |
| `impl log::Log for BuildScriptLog::flush` | 1 | `crates/vector-compute-shaders/build.rs:171` | - |  |
| `impl CargoWarningAdapter<W>::new` | 1 | `crates/vector-compute-shaders/build.rs:184` | - |  |
| `impl fmt::Write for CargoWarningAdapter<W>::write_str` | 2 | `crates/vector-compute-shaders/build.rs:193` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/backdrop.wgsl:24` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/backdrop_dyn.wgsl:28` | 3d |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/bbox_clear.wgsl:13` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/binning.wgsl:55` | 3d |  |
| `search_link` | 2 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:37` | - |  |
| `load_clip_path` | 1 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:70` | - |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/clip_leaf.wgsl:80` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/clip_reduce.wgsl:24` | 3d |  |
| `alloc_cmd` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:68` | - |  |
| `write_path` | 3 | `crates/vector-compute-shaders/shader/coarse.wgsl:88` | - |  |
| `write_color` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:112` | - |  |
| `write_grad` | 3 | `crates/vector-compute-shaders/shader/coarse.wgsl:119` | - |  |
| `write_image` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:127` | - |  |
| `write_begin_clip` | 0 | `crates/vector-compute-shaders/shader/coarse.wgsl:134` | - |  |
| `write_end_clip` | 1 | `crates/vector-compute-shaders/shader/coarse.wgsl:140` | - |  |
| `write_blurred_rounded_rect` | 2 | `crates/vector-compute-shaders/shader/coarse.wgsl:148` | 2d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/coarse.wgsl:156` | 3d |  |
| `read_transform` | 2 | `crates/vector-compute-shaders/shader/draw_leaf.wgsl:37` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/draw_leaf.wgsl:52` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/draw_reduce.wgsl:22` | 3d |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:113` | - |  |
| `fill_path_ms` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:146` | 2d |  |
| `fill_path_ms_evenodd` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:511` | 2d |  |
| `erf7` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:715` | - |  |
| `hypot` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:724` | - |  |
| `read_fill` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:728` | - |  |
| `read_color` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:735` | - |  |
| `read_blur_rect` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:740` | 2d |  |
| `read_lin_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:758` | - |  |
| `read_rad_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:769` | - |  |
| `read_sweep_grad` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:788` | - |  |
| `read_image` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:804` | - |  |
| `read_end_clip` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:829` | - |  |
| `pixel_format` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:838` | 2d |  |
| `maybe_premul_alpha` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:853` | 2d |  |
| `extend_mode_normalized` | 2 | `crates/vector-compute-shaders/shader/fine.wgsl:867` | - |  |
| `extend_mode` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:881` | - |  |
| `cubic_weights` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:925` | - |  |
| `single_weight` | 5 | `crates/vector-compute-shaders/shader/fine.wgsl:936` | - |  |
| `bicubic_sample` | 4 | `crates/vector-compute-shaders/shader/fine.wgsl:950` | 2d |  |
| `fill_path` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:1005` | 2d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/fine.wgsl:1064` | 3d |  |
| `premul_alpha` | 1 | `crates/vector-compute-shaders/shader/fine.wgsl:1400` | - |  |
| `approx_parabola_integral` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:47` | - |  |
| `approx_parabola_inv_integral` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:52` | - |  |
| `cubic_from_points_derivs` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:94` | - | Compute cubic parameters from endpoints and derivatives. |
| `es_params_from_angles` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:135` | - |  |
| `es_params_eval_th` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:160` | - |  |
| `integ_euler_10` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:165` | physics |  |
| `es_params_eval` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:197` | - |  |
| `es_params_eval_with_offset` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:210` | - |  |
| `es_seg_from_params` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:216` | - |  |
| `es_seg_eval_with_offset` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:221` | - |  |
| `pow_1_5_signed` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:227` | - |  |
| `espc_int_approx` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:250` | - |  |
| `espc_int_inv_approx` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:264` | - |  |
| `eval_cubic_and_deriv` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:285` | - |  |
| `cubic_start_tangent` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:295` | - |  |
| `cubic_end_tangent` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:303` | - |  |
| `cubic_start_normal` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:311` | 3d |  |
| `cubic_end_normal` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:316` | 3d |  |
| `flatten_euler` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:328` | physics |  |
| `flatten_arc` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:494` | - |  |
| `draw_cap` | 7 | `crates/vector-compute-shaders/shader/flatten.wgsl:523` | - |  |
| `draw_join` | 8 | `crates/vector-compute-shaders/shader/flatten.wgsl:549` | - |  |
| `read_f32_point` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:633` | - |  |
| `read_i16_point` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:639` | - |  |
| `transform_identity` | 0 | `crates/vector-compute-shaders/shader/flatten.wgsl:651` | - |  |
| `read_transform` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:655` | - |  |
| `transform_apply` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:668` | - |  |
| `round_down` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:674` | - |  |
| `round_up` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:678` | - |  |
| `compute_tag_monoid` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:687` | - |  |
| `read_path_segment` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:710` | - |  |
| `write_line` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:769` | - |  |
| `write_line_with_transform` | 5 | `crates/vector-compute-shaders/shader/flatten.wgsl:776` | - |  |
| `output_line` | 3 | `crates/vector-compute-shaders/shader/flatten.wgsl:782` | - |  |
| `output_line_with_transform` | 4 | `crates/vector-compute-shaders/shader/flatten.wgsl:787` | - |  |
| `output_two_lines_with_transform` | 6 | `crates/vector-compute-shaders/shader/flatten.wgsl:792` | - |  |
| `read_neighboring_segment` | 1 | `crates/vector-compute-shaders/shader/flatten.wgsl:810` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/flatten.wgsl:831` | 3d |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/path_count.wgsl:36` | - |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/path_count.wgsl:51` | 3d |  |
| `main` | 0 | `crates/vector-compute-shaders/shader/path_count_setup.wgsl:17` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/shader/path_tiling.wgsl:29` | - |  |
| `main` | 1 | `crates/vector-compute-shaders/shader/path_tiling.wgsl:39` | 3d |  |
| `main` | 0 | `crates/vector-compute-shaders/shader/path_tiling_setup.wgsl:20` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/pathtag_reduce.wgsl:21` | 3d |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/pathtag_reduce2.wgsl:21` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/pathtag_scan.wgsl:28` | 3d |  |
| `main` | 3 | `crates/vector-compute-shaders/shader/pathtag_scan1.wgsl:26` | 3d |  |
| `bbox_intersect` | 2 | `crates/vector-compute-shaders/shader/shared/bbox.wgsl:21` | - |  |
| `screen` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:24` | 3d |  |
| `color_dodge` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:28` | - |  |
| `color_burn` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:38` | - |  |
| `hard_light` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:48` | 3d |  |
| `soft_light` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:56` | 3d |  |
| `sat` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:69` | 3d |  |
| `lum` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:73` | 3d |  |
| `svg_lum` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:78` | 3d |  |
| `clip_color` | 1 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:83` | 3d |  |
| `set_lum` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:97` | 3d |  |
| `set_sat_inner` | 4 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:101` | - |  |
| `set_sat` | 2 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:117` | 3d |  |
| `blend_mix` | 3 | `crates/vector-compute-shaders/shader/shared/blend.wgsl:147` | 3d |  |
| `bic_combine` | 2 | `crates/vector-compute-shaders/shader/shared/clip.wgsl:9` | - |  |
| `draw_monoid_identity` | 0 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:34` | - |  |
| `combine_draw_monoid` | 2 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:38` | - |  |
| `map_draw_tag` | 1 | `crates/vector-compute-shaders/shader/shared/drawtag.wgsl:47` | - |  |
| `tag_monoid_identity` | 0 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:44` | - |  |
| `combine_tag_monoid` | 2 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:48` | - |  |
| `reduce_tag` | 1 | `crates/vector-compute-shaders/shader/shared/pathtag.wgsl:58` | - |  |
| `transform_apply` | 2 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:11` | 2d |  |
| `transform_inverse` | 1 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:15` | - |  |
| `transform_mul` | 2 | `crates/vector-compute-shaders/shader/shared/transform.wgsl:22` | - |  |
| `read_draw_tag_from_scene` | 1 | `crates/vector-compute-shaders/shader/shared/util.wgsl:15` | - |  |
| `main` | 2 | `crates/vector-compute-shaders/shader/tile_alloc.wgsl:35` | 3d |  |
| `impl fmt::Display for ErrorVec::fmt` | 2 | `crates/vector-compute-shaders/src/compile/mod.rs:48` | - |  |
| `impl Error::new` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:57` | - |  |
| `impl InnerError::emit_msg` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:68` | - |  |
| `impl ShaderInfo::new` | 3 | `crates/vector-compute-shaders/src/compile/mod.rs:92` | - |  |
| `impl ShaderInfo::from_default` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:182` | 3d | Same as [`ShaderInfo::from_dir`] but uses the default shader directory provided by [`shader_dir`]. |
| `impl ShaderInfo::from_dir` | 1 | `crates/vector-compute-shaders/src/compile/mod.rs:186` | 3d |  |
| `postprocess` | 1 | `crates/vector-compute-shaders/src/compile/mod.rs:255` | - |  |
| `shader_dir` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:272` | 3d |  |
| `manifest_dir` | 0 | `crates/vector-compute-shaders/src/compile/mod.rs:289` | - |  |
| `translate` | 1 | `crates/vector-compute-shaders/src/compile/msl.rs:8` | 3d |  |
| `impl BindingIndexIterator::next` | 2 | `crates/vector-compute-shaders/src/compile/msl.rs:64` | - |  |
| `parse` | 1 | `crates/vector-compute-shaders/src/compile/permutations.rs:14` | - |  |
| `get_imports` | 1 | `crates/vector-compute-shaders/src/compile/preprocess.rs:8` | 3d |  |
| `preprocess` | 4 | `crates/vector-compute-shaders/src/compile/preprocess.rs:37` | 3d |  |
| `impl Deref for TypedBufGuard<'_, T>::deref` | 1 | `crates/vector-compute-shaders/src/cpu.rs:77` | - |  |
| `impl Deref for TypedBufGuardMut<'_, T>::deref` | 1 | `crates/vector-compute-shaders/src/cpu.rs:88` | - |  |
| `impl DerefMut for TypedBufGuardMut<'_, T>::deref_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:97` | - |  |
| `impl CpuBinding<'_>::as_typed` | 1 | `crates/vector-compute-shaders/src/cpu.rs:106` | - |  |
| `impl CpuBinding<'_>::as_typed_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:116` | - |  |
| `impl CpuBinding<'_>::as_slice` | 1 | `crates/vector-compute-shaders/src/cpu.rs:128` | - |  |
| `impl CpuBinding<'_>::as_slice_mut` | 1 | `crates/vector-compute-shaders/src/cpu.rs:138` | - |  |
| `impl CpuBinding<'_>::as_tex` | 1 | `crates/vector-compute-shaders/src/cpu.rs:151` | - |  |
| `backdrop_main` | 4 | `crates/vector-compute-shaders/src/cpu/backdrop.rs:8` | - |  |
| `backdrop` | 2 | `crates/vector-compute-shaders/src/cpu/backdrop.rs:25` | - |  |
| `bbox_clear_main` | 2 | `crates/vector-compute-shaders/src/cpu/bbox_clear.rs:8` | - |  |
| `bbox_clear` | 2 | `crates/vector-compute-shaders/src/cpu/bbox_clear.rs:17` | - |  |
| `bbox_intersect` | 2 | `crates/vector-compute-shaders/src/cpu/binning.rs:16` | - |  |
| `binning_main` | 9 | `crates/vector-compute-shaders/src/cpu/binning.rs:25` | - |  |
| `binning` | 2 | `crates/vector-compute-shaders/src/cpu/binning.rs:110` | - |  |
| `clip_leaf_main` | 5 | `crates/vector-compute-shaders/src/cpu/clip_leaf.rs:20` | - |  |
| `clip_leaf` | 2 | `crates/vector-compute-shaders/src/cpu/clip_leaf.rs:75` | - |  |
| `clip_reduce_main` | 5 | `crates/vector-compute-shaders/src/cpu/clip_reduce.rs:10` | - |  |
| `clip_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/clip_reduce.rs:50` | - |  |
| `impl TileState::new` | 1 | `crates/vector-compute-shaders/src/cpu/coarse.rs:38` | - |  |
| `impl TileState::alloc_cmd` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:47` | - |  |
| `impl TileState::write` | 4 | `crates/vector-compute-shaders/src/cpu/coarse.rs:67` | - |  |
| `impl TileState::write_path` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:71` | - |  |
| `impl TileState::write_color` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:99` | - |  |
| `impl TileState::write_image` | 5 | `crates/vector-compute-shaders/src/cpu/coarse.rs:112` | - |  |
| `impl TileState::write_grad` | 7 | `crates/vector-compute-shaders/src/cpu/coarse.rs:125` | - |  |
| `impl TileState::write_blur_rect` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:141` | 2d |  |
| `impl TileState::write_begin_clip` | 4 | `crates/vector-compute-shaders/src/cpu/coarse.rs:156` | - |  |
| `impl TileState::write_end_clip` | 6 | `crates/vector-compute-shaders/src/cpu/coarse.rs:167` | - |  |
| `coarse_main` | 9 | `crates/vector-compute-shaders/src/cpu/coarse.rs:183` | - |  |
| `coarse` | 2 | `crates/vector-compute-shaders/src/cpu/coarse.rs:392` | - |  |
| `draw_leaf_main` | 8 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:14` | - |  |
| `draw_leaf` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:213` | - |  |
| `two_point_to_unit_line` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:233` | 2d |  |
| `from_poly2` | 2 | `crates/vector-compute-shaders/src/cpu/draw_leaf.rs:240` | 2d |  |
| `draw_reduce_main` | 4 | `crates/vector-compute-shaders/src/cpu/draw_reduce.rs:10` | - |  |
| `draw_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/draw_reduce.rs:27` | - |  |
| `impl CubicParams::from_points_derivs` | 5 | `crates/vector-compute-shaders/src/cpu/euler.rs:75` | physics, 2d | Compute parameters from endpoints and derivatives. |
| `impl EulerParams::from_angles` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:165` | - |  |
| `impl EulerParams::eval_th` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:190` | - |  |
| `impl EulerParams::eval` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:197` | 2d | Evaluate the curve at the given parameter. |
| `impl EulerParams::eval_with_offset` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:210` | 2d |  |
| `impl EulerSeg::from_params` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:218` | 2d |  |
| `impl EulerSeg::eval` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:223` | 2d |  |
| `impl EulerSeg::eval_with_offset` | 3 | `crates/vector-compute-shaders/src/cpu/euler.rs:234` | 2d |  |
| `integ_euler_10` | 2 | `crates/vector-compute-shaders/src/cpu/euler.rs:258` | physics | Integrate Euler spiral. |
| `espc_int_approx` | 1 | `crates/vector-compute-shaders/src/cpu/euler.rs:301` | - |  |
| `espc_int_inv_approx` | 1 | `crates/vector-compute-shaders/src/cpu/euler.rs:318` | - |  |
| `read_color` | 2 | `crates/vector-compute-shaders/src/cpu/fine.rs:13` | - |  |
| `read_fill` | 2 | `crates/vector-compute-shaders/src/cpu/fine.rs:23` | - |  |
| `unpack4x8unorm` | 1 | `crates/vector-compute-shaders/src/cpu/fine.rs:34` | - |  |
| `pack4x8unorm` | 1 | `crates/vector-compute-shaders/src/cpu/fine.rs:42` | - |  |
| `fill_path` | 5 | `crates/vector-compute-shaders/src/cpu/fine.rs:51` | - |  |
| `fine_main` | 8 | `crates/vector-compute-shaders/src/cpu/fine.rs:112` | - |  |
| `eval_cubic_and_deriv` | 5 | `crates/vector-compute-shaders/src/cpu/flatten.rs:40` | 2d | Evaluate both the point and derivative of a cubic bezier. |
| `cubic_start_tangent` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:50` | 2d |  |
| `cubic_end_tangent` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:63` | 2d |  |
| `write_line` | 6 | `crates/vector-compute-shaders/src/cpu/flatten.rs:76` | 2d |  |
| `write_line_with_transform` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:98` | 2d |  |
| `output_line` | 6 | `crates/vector-compute-shaders/src/cpu/flatten.rs:117` | 2d |  |
| `output_line_with_transform` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:129` | 2d |  |
| `output_two_lines_with_transform` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:142` | 2d |  |
| `flatten_arc` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:158` | 2d |  |
| `flatten_euler` | 9 | `crates/vector-compute-shaders/src/cpu/flatten.rs:203` | physics, 2d |  |
| `draw_cap` | 10 | `crates/vector-compute-shaders/src/cpu/flatten.rs:386` | 2d |  |
| `draw_join` | 11 | `crates/vector-compute-shaders/src/cpu/flatten.rs:427` | 2d |  |
| `read_f32_point` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:507` | 2d |  |
| `read_i16_point` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:513` | 2d |  |
| `impl Default for IntBbox::default` | 0 | `crates/vector-compute-shaders/src/cpu/flatten.rs:529` | - |  |
| `impl IntBbox::add_pt` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:540` | 2d |  |
| `compute_tag_monoid` | 3 | `crates/vector-compute-shaders/src/cpu/flatten.rs:553` | - |  |
| `read_path_segment` | 3 | `crates/vector-compute-shaders/src/cpu/flatten.rs:581` | - |  |
| `read_neighboring_segment` | 4 | `crates/vector-compute-shaders/src/cpu/flatten.rs:637` | - |  |
| `flatten_main` | 7 | `crates/vector-compute-shaders/src/cpu/flatten.rs:666` | - |  |
| `flatten` | 2 | `crates/vector-compute-shaders/src/cpu/flatten.rs:847` | - |  |
| `path_count_main` | 5 | `crates/vector-compute-shaders/src/cpu/path_count.rs:13` | - |  |
| `path_count` | 2 | `crates/vector-compute-shaders/src/cpu/path_count.rs:156` | - |  |
| `path_count_setup_main` | 2 | `crates/vector-compute-shaders/src/cpu/path_count_setup.rs:10` | - |  |
| `path_count_setup` | 2 | `crates/vector-compute-shaders/src/cpu/path_count_setup.rs:17` | - |  |
| `path_tiling_main` | 6 | `crates/vector-compute-shaders/src/cpu/path_tiling.rs:15` | - |  |
| `path_tiling` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling.rs:168` | - |  |
| `path_tiling_setup_main` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling_setup.rs:10` | - |  |
| `path_tiling_setup` | 2 | `crates/vector-compute-shaders/src/cpu/path_tiling_setup.rs:17` | - |  |
| `pathtag_reduce_main` | 4 | `crates/vector-compute-shaders/src/cpu/pathtag_reduce.rs:10` | - |  |
| `pathtag_reduce` | 2 | `crates/vector-compute-shaders/src/cpu/pathtag_reduce.rs:27` | - |  |
| `pathtag_scan_main` | 5 | `crates/vector-compute-shaders/src/cpu/pathtag_scan.rs:10` | - |  |
| `pathtag_scan` | 2 | `crates/vector-compute-shaders/src/cpu/pathtag_scan.rs:31` | - |  |
| `tile_alloc_main` | 6 | `crates/vector-compute-shaders/src/cpu/tile_alloc.rs:13` | - |  |
| `tile_alloc` | 2 | `crates/vector-compute-shaders/src/cpu/tile_alloc.rs:57` | - |  |
| `impl std::ops::Add for Vec2::add` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:19` | - |  |
| `impl std::ops::Sub for Vec2::sub` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:30` | - |  |
| `impl Mul<f32> for Vec2::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:41` | - |  |
| `impl std::ops::Div<f32> for Vec2::div` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:52` | - |  |
| `impl Mul<Vec2> for f32::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:63` | 2d |  |
| `impl std::ops::Neg for Vec2::neg` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:71` | - |  |
| `impl Vec2::new` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:77` | - |  |
| `impl Vec2::dot` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:81` | - |  |
| `impl Vec2::cross` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:85` | - |  |
| `impl Vec2::length` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:89` | - |  |
| `impl Vec2::length_squared` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:93` | - |  |
| `impl Vec2::distance` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:97` | - |  |
| `impl Vec2::to_array` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:101` | - |  |
| `impl Vec2::from_array` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:105` | - |  |
| `impl Vec2::mix` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:109` | - |  |
| `impl Vec2::normalize` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:115` | - |  |
| `impl Vec2::atan2` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:119` | - |  |
| `impl Vec2::is_nan` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:123` | - |  |
| `impl Vec2::min` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:127` | - |  |
| `impl Vec2::max` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:131` | - |  |
| `impl Transform::identity` | 0 | `crates/vector-compute-shaders/src/cpu/util.rs:140` | - |  |
| `impl Transform::apply` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:144` | 2d |  |
| `impl Transform::inverse` | 1 | `crates/vector-compute-shaders/src/cpu/util.rs:151` | - |  |
| `impl Transform::read` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:170` | - |  |
| `impl Mul for Transform::mul` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:184` | - |  |
| `span` | 2 | `crates/vector-compute-shaders/src/cpu/util.rs:196` | - |  |
| `read_draw_tag_from_scene` | 3 | `crates/vector-compute-shaders/src/cpu/util.rs:206` | - | Read draw tag, guarded by number of draw objects. |
| `impl BindType::is_mutable` | 1 | `crates/vector-compute-shaders/src/types.rs:23` | - |  |
| `impl fmt::Debug for BindingIndex::fmt` | 2 | `crates/vector-compute-shaders/src/types.rs:53` | - |  |

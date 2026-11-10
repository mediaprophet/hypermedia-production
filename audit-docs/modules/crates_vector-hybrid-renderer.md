# `crates/vector-hybrid-renderer`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_A GPU renderer for 2D vector graphics with CPU-side preprocessing.

Vello GPU uses the Sparse Strips architecture to divide work between CPU
preprocessing and GPU rendering:

- The CPU processes paths, tiles, and schedules sparse strips.
- The GPU rasterizes those strips and performs compositing.
- The compact strip representation limits data transfer between them.

## Key Features

- Efficient path rendering with CPU-side processing
- GPU-accelerated compositing and blending
- Support for both windowed and headless rendering

## Feature Flags

- `wgpu` (enabled by default): Enables the GPU rendering backend via wgpu and
includes the required Vello GPU shaders.
- `wgpu_default` (enabled by default): Enables wgpu with its default hardware
backends (such as Vulkan, Metal, and DX12).
- `text` (enabled by default): Enables glyph rendering ([`Scene::glyph_run`]).
- `webgl`: Enables the WebGL rendering backend for browser support, using GLSL
shaders for compatibility.

If you need to customize the set of enabled wgpu features, disable this crate's
default features then enable its `wgpu` feature. You can then depend on wgpu
directly, setting the specific features you require. Don't forget to also
disable wgpu's default features.

## Architecture

The renderer is split into several key components:

- `Scene`: Manages the render context and path processing on the CPU
- `Renderer` or `WebGlRenderer`: Handles GPU resource management and executes
draw operations

See the individual module documentation for more details on usage and
implementation.

## Current state

Vello GPU is a solid GPU-accelerated 2D renderer with broad, reliable feature
support. Although it does not match the compute-centric `vello` renderer's raw
performance on dynamic and vector-heavy workloads, it provides excellent
performance on workloads that benefit from GPU acceleration, such as images,
gradients, and filters. Overall, we still consider it to be slightly less mature
than its CPU-only counterpart Vello CPU.

Vello GPU remains under active development. Known limitations include:

- The following features are not yet supported and will panic: Mask layers,
complex filter graphs as well as certain blend modes for non-isolated
blending.
- Parts of the API and its documentation are still suboptimal, for example the
lifecycle and ownership of external resources through [`Resources`][].
- Some exposed features remain experimental and are not recommended for use,
including glyph caching. Experimental APIs are identified in their method
documentation.
- Parts of the rendering pipeline are not yet fully optimized, particularly the
wgpu backend, but also other aspects.
- Some failures panic instead of being reported through a user-facing error.

With that said, we are continuously improving Vello GPU and will address these
and other limitations in future releases.

## Package rename

This package was previously named `vello_hybrid`. New dependencies and Rust
imports should use `vello_gpu`; no compatibility re-export package is provided
in this repository. Existing crates.io releases under `vello_hybrid` remain
available, and publication under the new name may lag behind the repository
rename._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:22` | - | Main entry point for the headless rendering example. |
| `run` | 0 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:26` | - |  |
| `render_svg` | 3 | `crates/vector-hybrid-renderer/examples/render_to_file.rs:176` | - |  |
| `impl GpuBlendInstance::new` | 3 | `crates/vector-hybrid-renderer/src/blend.rs:37` | - |  |
| `impl GpuBlendInstance::copy_from_scratch` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:85` | - |  |
| `impl BlendStrip::from_fill_segment` | 2 | `crates/vector-hybrid-renderer/src/blend.rs:108` | 2d |  |
| `pack_blend_config` | 6 | `crates/vector-hybrid-renderer/src/blend.rs:128` | - |  |
| `pack_mix` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:144` | - |  |
| `pack_compose` | 1 | `crates/vector-hybrid-renderer/src/blend.rs:165` | - |  |
| `impl Draw::push` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:38` | - |  |
| `impl Clear for Draw::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:57` | - |  |
| `impl OpaqueDraw::push` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:72` | - |  |
| `impl OpaqueDraw::reverse` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:85` | - | Reverse opaque strips for front-to-back rendering. |
| `impl OpaqueDraw::is_empty` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:98` | - |  |
| `impl OpaqueDraw::strips` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:102` | - |  |
| `impl OpaqueDraw::external_texture_runs` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:106` | - |  |
| `impl Clear for OpaqueDraw::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:112` | - |  |
| `impl DrawBuilder<'a, T>::new` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:132` | - |  |
| `impl DrawBuilder<'a, T>::push_draw` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:145` | - |  |
| `impl DrawBuilder<'a, T>::push_opaque` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:159` | - |  |
| `impl DrawBuilder<'a, T>::push_path` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:168` | - |  |
| `impl DrawBuilder<'a, T>::push_rect` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:228` | 2d |  |
| `impl DrawBuilder<'a, T>::push_layer_fill` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:270` | - |  |
| `impl DrawBuilder<'a, T>::push_layer_fill_segment` | 6 | `crates/vector-hybrid-renderer/src/draw.rs:334` | - |  |
| `impl DrawBuffers::clear` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:362` | - |  |
| `impl DrawState<T>::new` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:382` | - |  |
| `impl GpuStrip::from_fill_segment` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:397` | 2d |  |
| `impl GpuStrip::from_rect` | 4 | `crates/vector-hybrid-renderer/src/draw.rs:423` | 2d |  |
| `impl LayerTextureRegion::payload_at` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:438` | - |  |
| `impl ExternalTextureBindings::get_or_insert` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:466` | - | Return the existing slot for `texture_source`, or insert it into the first empty slot. |
| `impl ExternalTextureBindings::as_array` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:486` | - |  |
| `assign_external_texture_slot` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:501` | - |  |
| `impl DepthCounter::next` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:535` | - |  |
| `trait RectU16Ext::to_tile_bounds` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:543` | - |  |
| `impl RectU16Ext for RectU16::to_tile_bounds` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:547` | - |  |
| `trait StripAlphaFillSegmentExt::col_idx` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:566` | - |  |
| `impl StripAlphaFillSegmentExt for StripAlphaFillSegment::col_idx` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:571` | - |  |
| `impl DrawCase<T>::new` | 2 | `crates/vector-hybrid-renderer/src/draw.rs:606` | - |  |
| `impl DrawCase<T>::rect` | 5 | `crates/vector-hybrid-renderer/src/draw.rs:614` | 2d |  |
| `impl DrawCase<T>::layer` | 3 | `crates/vector-hybrid-renderer/src/draw.rs:629` | - |  |
| `solid` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:639` | - |  |
| `indexed` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:643` | - |  |
| `rect` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:647` | 2d |  |
| `no_paints` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:651` | - |  |
| `gpu_strip` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:655` | - |  |
| `strip_xs` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:668` | - |  |
| `external_texture_slots` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:672` | - |  |
| `texture_ids` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:679` | - |  |
| `external_sources` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:683` | - |  |
| `run_states` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:687` | - |  |
| `external` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:698` | - |  |
| `atlas_image` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:715` | - |  |
| `layer` | 1 | `crates/vector-hybrid-renderer/src/draw.rs:732` | - |  |
| `texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:743` | - |  |
| `fifth_external_texture_starts_a_fresh_run` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:771` | - |  |
| `texture_runs_coalesce_distinct_paints_for_same_texture` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:810` | - |  |
| `texture_runs_include_atlas_images` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:827` | - |  |
| `opaque_reverse_rebases_texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:852` | - |  |
| `opaque_reverse_without_texture_runs` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:940` | - |  |
| `draw_clear` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:953` | - |  |
| `opaque_routing` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:979` | - |  |
| `child_binding` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:1002` | - |  |
| `depth_progression` | 0 | `crates/vector-hybrid-renderer/src/draw.rs:1012` | - |  |
| `edge_mode_to_gpu` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:86` | graph |  |
| `pack_header` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:95` | - |  |
| `pack_header_with_gaussian_params` | 4 | `crates/vector-hybrid-renderer/src/filter.rs:101` | graph, numerics |  |
| `impl LinearKernel::new` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:144` | - |  |
| `impl From<&Offset> for GpuOffset::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:204` | - |  |
| `impl From<&Flood> for GpuFlood::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:223` | - |  |
| `impl From<&GaussianBlur> for GpuGaussianBlur::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:248` | - |  |
| `impl From<&DropShadow> for GpuDropShadow::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:287` | - |  |
| `impl GpuFilterData::filter_type` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:326` | - |  |
| `impl GpuFilterData::n_decimations` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:331` | - | Returns the number of decimation levels encoded in the header. |
| `impl GpuFilterData::composite_original` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:335` | - |  |
| `impl GpuFilterData::needs_copy_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:339` | - |  |
| `impl From<T> for GpuFilterData::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:354` | - |  |
| `impl From<&PreparedFilter> for GpuFilterData::from` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:360` | - |  |
| `impl FilterPassPlan::init` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:424` | - |  |
| `impl FilterPassPlan::steps` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:463` | - |  |
| `impl FilterPassPlan::copy_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:467` | - |  |
| `impl FilterPassPlan::clear` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:471` | - |  |
| `impl FilterPassPlan::step_mut` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:476` | - |  |
| `impl FilterPassBuilder<'a>::new` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:503` | - |  |
| `impl FilterPassBuilder<'a>::apply_pass_dimensions` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:520` | - | Compute and update source and destination sizes based on the pass kind. |
| `impl FilterPassBuilder<'a>::emit` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:541` | - | Emit one pass, reading from the current region and writing to the other texture parity. |
| `impl FilterPassBuilder<'a>::emit_blur_sequence` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:572` | - | Apply the sequences of passes that is needed to create a full Gaussian blur with |
| `impl FilterPassBuilder<'a>::ensure_result_in_original` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:600` | - |  |
| `impl FilterPassBuilder<'a>::push_copy_to_scratch_pass` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:606` | - |  |
| `impl FilterContext::clear` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:621` | - |  |
| `impl FilterContext::push` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:625` | - |  |
| `impl FilterContext::is_empty` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:634` | - |  |
| `impl FilterContext::total_texels` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:642` | - |  |
| `impl FilterContext::serialize_to_buffer` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:646` | - |  |
| `impl FilterContext::required_filter_data_height` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:659` | - | Calculate the required height for the filter data texture. |
| `region` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:687` | - |  |
| `filter_op` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:694` | - |  |
| `gpu_offset` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:705` | - |  |
| `gpu_flood` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:709` | - |  |
| `gpu_blur` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:713` | - |  |
| `gpu_shadow` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:717` | - |  |
| `filter_data_height_must_fit_resource_texture_limit` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:730` | - |  |
| `step_layout` | 1 | `crates/vector-hybrid-renderer/src/filter.rs:738` | - |  |
| `pass_batching` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:749` | - |  |
| `plan_reinit` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:784` | - |  |
| `single_pass_filters_finish_in_original` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:806` | - |  |
| `test_offset_conversion` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:824` | - |  |
| `check_round_trip` | 2 | `crates/vector-hybrid-renderer/src/filter.rs:833` | - |  |
| `test_offset_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:843` | - |  |
| `test_flood_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:848` | - |  |
| `test_gaussian_blur_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:856` | - |  |
| `test_drop_shadow_round_trip` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:864` | - |  |
| `check_linear_kernel` | 3 | `crates/vector-hybrid-renderer/src/filter.rs:877` | numerics |  |
| `linear_kernel_size_1` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:889` | numerics |  |
| `linear_kernel_size_3` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:896` | numerics |  |
| `linear_kernel_size_7` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:903` | numerics |  |
| `linear_kernel_size_13` | 0 | `crates/vector-hybrid-renderer/src/filter.rs:910` | numerics |  |
| `impl GradientRampCache::new` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:58` | - | Create a new gradient ramp cache with the specified retained count. |
| `impl GradientRampCache::get_or_create_ramp` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:75` | - |  |
| `impl GradientRampCache::maintain` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:97` | - | Maintain the gradient cache by evicting old entries. |
| `impl GradientRampCache::luts_size` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:106` | - | Get the size of the packed luts. |
| `impl GradientRampCache::is_empty` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:111` | - | Check if the packed luts is empty. |
| `impl GradientRampCache::has_changed` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:116` | - | Check if the luts data has changed. |
| `impl GradientRampCache::mark_synced` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:121` | - | Mark the luts as synced. |
| `impl GradientRampCache::take_luts` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:126` | - | Take ownership of the luts, leaving an empty vector in its place. |
| `impl GradientRampCache::restore_luts` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:131` | - | Restore the luts. The restored luts should have the same logical content as the original. |
| `impl GradientRampCache::evict` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:136` | - | Remove multiple LRU entries and compact the LUTs vector. |
| `impl GradientRampCache::remove_lru_entries` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:152` | - | Identify and remove the LRU cache entries. |
| `impl GradientRampCache::compact_luts` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:188` | - | Remove LUT data for evicted entries with compacting the LUTs vector, and update remaining offsets. |
| `reuse_vec` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:246` | - |  |
| `generate_gradient_lut_impl` | 3 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:276` | - |  |
| `insert_entries` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:297` | - |  |
| `insert_entry` | 2 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:306` | - |  |
| `create_encoded_gradient` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:311` | - |  |
| `create_gradient` | 1 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:320` | - |  |
| `test_cache_empty` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:351` | - |  |
| `test_unique_entry_creation` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:362` | - |  |
| `test_no_eviction_under_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:373` | - |  |
| `test_no_eviction_at_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:382` | - |  |
| `test_eviction_over_limit` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:391` | - |  |
| `test_lut_compaction_and_offset_updates` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:401` | - |  |
| `test_correct_lru_eviction` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:444` | - |  |
| `test_take_and_restore_luts` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:501` | - |  |
| `test_lut_start_invalidation` | 0 | `crates/vector-hybrid-renderer/src/gradient_cache.rs:534` | - |  |
| `impl PackedPaint::payload_at` | 3 | `crates/vector-hybrid-renderer/src/paint.rs:52` | - |  |
| `impl PaintResolver<'a>::new` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:81` | - |  |
| `impl PaintResolver<'a>::with_image_cache` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:90` | - | Add the image cache needed to resolve internal image paints. |
| `impl PaintResolver<'a>::pack` | 2 | `crates/vector-hybrid-renderer/src/paint.rs:96` | - |  |
| `impl RectPart::shift` | 2 | `crates/vector-hybrid-renderer/src/rect.rs:23` | - |  |
| `split_rect` | 1 | `crates/vector-hybrid-renderer/src/rect.rs:50` | 2d |  |
| `pack_unorm4x8` | 1 | `crates/vector-hybrid-renderer/src/rect.rs:140` | - |  |
| `part` | 5 | `crates/vector-hybrid-renderer/src/rect.rs:155` | - |  |
| `splitter_keeps_small_rect_whole` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:163` | 2d |  |
| `splitter_keeps_subpixel_rect_inside_one_pixel` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:180` | 2d |  |
| `splitter_keeps_subpixel_rect_spanning_two_pixels_in_width` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:197` | 2d |  |
| `splitter_keeps_subpixel_rect_spanning_two_pixels_in_height` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:214` | 2d |  |
| `splitter_keeps_multi_pixel_width_rect_within_one_pixel_height` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:231` | 2d |  |
| `splitter_keeps_multi_pixel_height_rect_within_one_pixel_width` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:248` | 2d |  |
| `splitter_splits_large_rect_into_five_parts` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:265` | 2d |  |
| `splitter_omits_unneeded_edge_parts` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:282` | graph |  |
| `splitter_handles_large_rect_with_only_vertical_aa` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:299` | 2d |  |
| `splitter_keeps_large_aligned_rect_as_single_main_rect` | 0 | `crates/vector-hybrid-renderer/src/rect.rs:316` | 2d |  |
| `impl ClearSettings<'_>::clear_color` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:45` | - |  |
| `impl Default for ClearSettings<'_>::default` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:56` | - |  |
| `impl DeviceLimits::resource_texture_dimension_2d` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:71` | - |  |
| `impl ScratchTexture<T>::new` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:104` | - |  |
| `impl ScratchTexture<T>::get` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:108` | - |  |
| `impl MemorySettings::normalize` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:140` | - |  |
| `impl LayersConfig::required_intermediate_texture_size` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:166` | - |  |
| `device_limits` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:227` | - |  |
| `normalize_memory_settings_clamps_atlas_dimensions_to_backend_limits` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:234` | - |  |
| `resource_texture_dimension_is_capped_at_4k` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:254` | - |  |
| `normalize_memory_settings_allows_lazy_initial_atlas_allocation` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:260` | - |  |
| `normalize_memory_settings_clamps_minimum_to_maximum` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:279` | - |  |
| `normalize_memory_settings_clamps_layer_sizes_to_backend_limit` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:302` | - |  |
| `required_intermediate_texture_size_rejects_oversized_layers` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:322` | - |  |
| `required_intermediate_texture_size_rejects_filter_padding_overflow` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:344` | - |  |
| `texels` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:468` | - |  |
| `impl GpuEncodedPaint::as_bytes` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:486` | - | Returns the byte representation of this paint. |
| `impl GpuEncodedPaint::size_texels` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:497` | - | Number of `RGBA32Uint` texels occupied by this paint. |
| `impl GpuEncodedPaint::serialize_to_buffer` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:508` | - | Serialize paint enums directly into the provided buffer. Returns the number of bytes written. |
| `pack_texture_width_and_extend_mode` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:638` | - | Pack `extend_mode` and `texture_width` into a single u32. |
| `pack_radial_kind_and_swapped` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:651` | - | Pack radial gradient `kind` and `f_is_swapped` into a single u32. |
| `pack_image_size` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:661` | - | Pack image `width` and `height` into a single u32. |
| `pack_image_offset` | 2 | `crates/vector-hybrid-renderer/src/render/common.rs:669` | - | Pack image offset coordinates `x` and `y` into a single u32. |
| `pack_image_params` | 3 | `crates/vector-hybrid-renderer/src/render/common.rs:678` | - | Pack image `quality` and extend modes into a single u32. |
| `pack_tint` | 1 | `crates/vector-hybrid-renderer/src/render/common.rs:690` | - | Pack an optional [`Tint`](vello_common::paint::Tint) into a (`tint_color_u32`, `tint_mode_u32`) pair for the GPU. |
| `maybe_warn_about_webgl_feature_conflict` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:704` | - |  |
| `maybe_warn_about_webgl_feature_conflict` | 0 | `crates/vector-hybrid-renderer/src/render/common.rs:722` | - |  |
| `trait WebGlResultExt::map_js_error` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:178` | - | Attach the WebGL operation that produced this JavaScript result. |
| `impl WebGlResultExt<T> for Result<T, JsValue>::map_js_error` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:182` | - |  |
| `impl fmt::Display for MessageSuffix<'_>::fmt` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/error.rs:193` | - |  |
| `get_max_texture_dimension_2d` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:109` | - | Query the WebGL context for the max texture size. |
| `impl WebGlTextureBindings::new` | 0 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:181` | - | Create an empty binding map. |
| `impl WebGlTextureBindings::insert` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:199` | numerics | Insert or replace a texture binding. |
| `impl WebGlTextureBindings::get` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:205` | - | Get a texture binding. |
| `impl WebGlTextureBindings::remove` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:213` | - | Remove a texture binding. |
| `impl WebGlRenderer::new` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:234` | 3d, 2d | Creates a new WebGL2 renderer and its persistent resources. |
| `impl WebGlRenderer::new_with` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:243` | 2d | Creates a new WebGL2 renderer and its persistent resources with specific settings. |
| `impl WebGlRenderer::begin` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:261` | 3d, 2d | Begins creating a WebGL2 renderer and its persistent resources. |
| `impl WebGlRenderer::begin_with` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:268` | 2d | Begins creating a WebGL2 renderer with specific settings. |
| `impl WebGlRenderer::render` | 10 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:437` | - | Render `scene` using WebGL2 |
| `impl WebGlRenderer::render_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:517` | - | Render a `scene` directly into an atlas layer. |
| `impl WebGlRenderer::current_allocations` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:586` | - |  |
| `impl WebGlRenderer::render_scene` | 8 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:601` | - | Shared render pipeline: prepares GPU resources, runs the scheduler, and |
| `impl WebGlRenderer::gl_context` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:712` | - | Get a reference to the underlying WebGL context. |
| `impl WebGlRenderer::upload_image` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:720` | - | Upload image to cache and atlas in one step. Returns the `ImageId`. |
| `impl WebGlRenderer::upload_image_with` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:728` | - |  |
| `impl WebGlRenderer::write_to_atlas` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:758` | 2d | Write pixel data to an existing atlas allocation. |
| `impl WebGlRenderer::destroy_image` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:784` | - | Destroy an image from the cache and clear the allocated slot in the atlas. |
| `impl WebGlRenderer::atlas_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:808` | - | Returns an individual image atlas texture. |
| `impl WebGlRenderer::atlas_info` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:818` | - | Returns the current allocation state of the image/glyph atlas texture. |
| `impl WebGlRenderer::clear_atlas_region` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:828` | - | Clear a specific region of an atlas texture. |
| `impl WebGlRenderer::prepare_gpu_encoded_paints` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:870` | - |  |
| `impl WebGlRenderer::encode_image_paint` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:923` | - |  |
| `impl WebGlRenderer::encode_external_texture_paint` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:949` | - |  |
| `impl WebGlRenderer::encode_gradient_paint` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:974` | - |  |
| `impl WebGlRenderer::encode_blurred_rounded_rect_paint` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1045` | 2d |  |
| `impl WebGlRendererInit::try_finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1069` | 3d | Polls shader compilation and program linking without blocking when |
| `impl WebGlRendererInit::finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1081` | 3d | Blocks until shader compilation and program linking finish. |
| `clear_atlas_region` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1099` | 2d |  |
| `impl ViewFramebuffer::default` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1204` | - |  |
| `impl ViewFramebuffer::offscreen` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1209` | - | If `use_depth_buffer` is true, the framebuffer must have a valid depth attachment! |
| `impl ViewFramebuffer::binding` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1216` | - |  |
| `impl ViewFramebuffer::use_depth_buffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1223` | - |  |
| `impl ViewFramebuffer::is_offscreen` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1232` | - |  |
| `impl ViewFramebuffer::into_framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1236` | - |  |
| `impl WebGlResources::layer_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1318` | - |  |
| `impl WebGlResources::layer_framebuffer` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1323` | - |  |
| `impl WebGlResources::scratch_binding_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1328` | - |  |
| `impl WebGlResources::scratch_framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1336` | - |  |
| `impl WebGlIntermediateTexture::binding_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1348` | - |  |
| `impl WebGlIntermediateTexture::framebuffer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1352` | - |  |
| `impl PendingShaderProgram::new` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1365` | 3d |  |
| `impl PendingShaderProgram::is_complete` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1390` | 3d |  |
| `impl PendingShaderProgram::finish` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1401` | - |  |
| `impl PendingWebGlPrograms::new` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1471` | - | Starts compiling all programs before performing any blocking status queries. |
| `impl PendingWebGlPrograms::is_complete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1521` | - |  |
| `impl PendingWebGlPrograms::finish` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1535` | - |  |
| `impl WebGlPrograms::prepare` | 9 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1575` | - | Prepare resources for rendering. |
| `impl WebGlPrograms::prepare_intermediate_textures` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1607` | - |  |
| `impl WebGlPrograms::maybe_create_atlas_textures` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1670` | - | Create any newly allocated atlas textures. |
| `impl WebGlPrograms::maybe_resize_filter_data_tex` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1686` | - | Grow the filter data texture if needed. |
| `impl WebGlPrograms::upload_filter_data_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1715` | - | Upload filter data to the texture. |
| `impl WebGlPrograms::upload_filter_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1741` | - |  |
| `impl WebGlPrograms::upload_blend_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1757` | - |  |
| `impl WebGlPrograms::upload_copy_instances` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1769` | - |  |
| `impl WebGlPrograms::maybe_resize_alphas_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1782` | - | Grow the alpha texture if needed. |
| `impl WebGlPrograms::maybe_resize_encoded_paints_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1814` | - | Grow the encoded paints texture if needed. |
| `impl WebGlPrograms::maybe_resize_gradient_tex` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1848` | - | Grow the gradient texture if needed. |
| `impl WebGlPrograms::maybe_update_config_buffer` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1884` | - | Update config buffer if dimensions changed. |
| `impl WebGlPrograms::upload_alpha_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1922` | - | Upload alpha data to the texture. |
| `impl WebGlPrograms::upload_encoded_paints_texture` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1956` | - | Upload encoded paints to the texture. |
| `impl WebGlPrograms::upload_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:1983` | - | Upload gradient data to the texture. |
| `impl WebGlPrograms::upload_strip_pair` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2034` | - | Uploads two strip slices (opaque then alpha) into a single GPU buffer. |
| `required_uniform_location` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2083` | - |  |
| `get_strip_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2096` | - | Get the  uniform locations for the `render_strips` program. |
| `get_filter_pass_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2157` | - |  |
| `get_blend_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2173` | - |  |
| `get_copy_uniforms` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2184` | - |  |
| `initialize_filter_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2197` | - |  |
| `initialize_blend_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2217` | - |  |
| `initialize_copy_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2237` | - |  |
| `initialize_packed_u32_attribs` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2253` | - |  |
| `create_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2268` | graph | Create a texture with the requested filtering and clamp-to-edge wrapping. |
| `create_texture_inner` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2281` | - |  |
| `create_texture_storage` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2322` | - | Create a texture backed by immutable storage (`texStorage2D`). |
| `validate_resource_texture_size` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2345` | - | Validate that a resource texture fits within its configured dimension limit. |
| `create_data_texture_storage` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2362` | - | Create a zero-initialized data texture backed by immutable storage (`texStorage2D`). |
| `create_placeholder_rgba32ui_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2379` | - | Create a 1x1 RGBA32UI placeholder texture. |
| `create_placeholder_rgba8_texture` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2391` | - | Create a 1x1 RGBA8 placeholder texture. |
| `upload_layer_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2395` | - |  |
| `create_webgl_resources` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2421` | - | Create all WebGL resources needed for rendering. |
| `create_intermediate_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2532` | - |  |
| `create_atlas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2552` | - | Create an image atlas texture. |
| `create_framebuffer_for_texture` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2568` | - | Create a framebuffer for a texture. |
| `initialize_strip_vao` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2594` | - | Initialize strip VAO. |
| `checked_instance_count` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2636` | - |  |
| `impl WebGlRendererContext<'_>::draw_instanced_quads` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2642` | - | Draw `instance_count` quad instances using the currently bound VAO. |
| `impl WebGlRendererContext<'_>::draw_strips` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2659` | - | Draw `count` strip instances starting at `first_instance`, split into one draw per |
| `impl WebGlRendererContext<'_>::draw_strip_range` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2701` | 3d | Draw `count` strip instances starting at `first_instance`. |
| `impl WebGlRendererContext<'_>::bind_external_textures` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2711` | - | Bind the external texture slots. |
| `impl WebGlRendererContext<'_>::set_strip_attrib_offset` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2736` | 3d | Point the strip vertex attributes at the instance at `first_instance`. |
| `impl WebGlRendererContext<'_>::strip_pass_inner` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2750` | - | Render strips to the specified render target. |
| `impl WebGlRendererContext<'_>::texture_size` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2939` | - |  |
| `impl WebGlRendererContext<'_>::blend_pass_inner` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:2943` | - |  |
| `impl WebGlRendererContext<'_>::filter_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3067` | - |  |
| `impl WebGlRendererContext<'_>::do_filter_instance_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3135` | - |  |
| `impl WebGlRendererContext<'_>::clear_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3173` | - |  |
| `impl Backend for WebGlRendererContext<'_>::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3241` | - |  |
| `impl Backend for WebGlRendererContext<'_>::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3255` | - |  |
| `impl Backend for WebGlRendererContext<'_>::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3270` | - |  |
| `impl Backend for WebGlRendererContext<'_>::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3279` | - |  |
| `impl Backend for WebGlRendererContext<'_>::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3287` | - |  |
| `trait WebGlAtlasWriter::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3308` | - | Get the width of the image. |
| `trait WebGlAtlasWriter::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3310` | - | Get the height of the image. |
| `trait WebGlAtlasWriter::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3313` | - | Write image data to an atlas texture at the specified offset. |
| `impl WebGlAtlasWriter for Pixmap::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3325` | - |  |
| `impl WebGlAtlasWriter for Pixmap::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3329` | - |  |
| `impl WebGlAtlasWriter for Pixmap::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3333` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3368` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3372` | - |  |
| `impl WebGlAtlasWriter for Arc<Pixmap>::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3376` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3391` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3398` | - |  |
| `impl WebGlAtlasWriter for WebGlTexture::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3405` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::width` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3444` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::height` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3448` | - |  |
| `impl WebGlAtlasWriter for WebGlTextureWithDimensions::write_to_atlas` | 6 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3452` | - |  |
| `copy_to_texture` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3466` | - | Copy from a source texture/framebuffer to an atlas texture. |
| `upload_rgba32ui_rows` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3504` | - | Upload full rows of RGBA32UI data into immutable texture storage. |
| `impl DrawPassTarget::negate_ndc` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3558` | - |  |
| `resource_texture_size_must_fit_dimension_limit` | 0 | `crates/vector-hybrid-renderer/src/render/webgl/mod.rs:3569` | - |  |
| `impl WebGlRenderer::probe` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:75` | 2d | Conduct a probing operation. |
| `impl WebGlRenderer::probe_inner` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:86` | - |  |
| `impl WebGlPendingProbe::try_finish` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:200` | - | Try to finish the probe. |
| `impl WebGlPendingProbe::finish_success` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:216` | - |  |
| `impl WebGlPendingProbe::finish_failure` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:248` | - |  |
| `webgl_error_name` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:253` | - |  |
| `launch_probe` | 5 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:269` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:322` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:326` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:330` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:334` | 2d |  |
| `impl vello_common::probe::ProbeRenderer for Scene::push_layer` | 3 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:338` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::push_filter_layer` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:342` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:346` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:350` | - |  |
| `impl vello_common::probe::ProbeRenderer for Scene::reset_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/probe.rs:354` | - |  |
| `trait GlResource::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:18` | - |  |
| `trait GlResource::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:22` | - |  |
| `impl Resource<T>::new` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:36` | - |  |
| `impl Drop for Resource<T>::drop` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:50` | - |  |
| `impl Deref for Resource<T>::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:58` | - |  |
| `impl Deref for WebGlVertexShader::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:69` | - |  |
| `impl Deref for WebGlFragmentShader::deref` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:80` | - |  |
| `impl GlResource for WebGlTexture::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:88` | - |  |
| `impl GlResource for WebGlTexture::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:92` | - |  |
| `impl GlResource for WebGlBuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:100` | - |  |
| `impl GlResource for WebGlBuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:104` | - |  |
| `impl GlResource for WebGlFramebuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:112` | - |  |
| `impl GlResource for WebGlFramebuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:116` | - |  |
| `impl GlResource for WebGlRenderbuffer::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:125` | - |  |
| `impl GlResource for WebGlRenderbuffer::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:129` | - |  |
| `impl GlResource for WebGlSync::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:138` | - |  |
| `impl GlResource for WebGlSync::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:142` | - |  |
| `impl GlResource for WebGlProgram::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:150` | - |  |
| `impl GlResource for WebGlProgram::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:154` | - |  |
| `impl GlResource for WebGlVertexShader::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:162` | - |  |
| `impl GlResource for WebGlVertexShader::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:167` | - |  |
| `impl GlResource for WebGlFragmentShader::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:175` | - |  |
| `impl GlResource for WebGlFragmentShader::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:180` | - |  |
| `impl GlResource for WebGlVertexArrayObject::create` | 1 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:188` | - |  |
| `impl GlResource for WebGlVertexArrayObject::delete` | 2 | `crates/vector-hybrid-renderer/src/render/webgl/resource.rs:192` | - |  |
| `impl TextureBindings::new` | 0 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:96` | - | Create an empty binding map. |
| `impl TextureBindings::insert` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:121` | - | Insert or replace a texture binding. |
| `impl TextureBindings::get` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:127` | - | Get a texture binding. |
| `impl TextureBindings::remove` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:135` | - | Remove a texture binding. |
| `impl Renderer::new` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:161` | - | Creates a new renderer and its persistent resources. |
| `impl Renderer::new_with` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:166` | - | Creates a new renderer and its persistent resources with specific settings. |
| `impl Renderer::create_depth_texture_view` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:216` | - | Creates a depth texture view compatible with [`render`](Self::render). |
| `impl Renderer::render` | 11 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:253` | - | Render `scene`. |
| `impl Renderer::render_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:340` | - | Render a `scene` directly into an atlas layer. |
| `impl Renderer::current_allocations` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:393` | - |  |
| `impl Renderer::render_scene` | 13 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:407` | - | Shared render pipeline: prepares GPU resources, runs the scheduler against |
| `impl Renderer::upload_image` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:519` | - | Upload image to cache and atlas in one step. Returns the `ImageId`. |
| `impl Renderer::upload_image_with` | 7 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:537` | - |  |
| `impl Renderer::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:562` | 2d | Write pixel data to an existing atlas allocation. |
| `impl Renderer::destroy_image` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:594` | - | Destroy an image from the cache and clear the allocated slot in the atlas. |
| `impl Renderer::atlas_texture` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:617` | - | Returns an individual image atlas texture. |
| `impl Renderer::clear_atlas_region` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:626` | - | Clear a specific region of the atlas texture. |
| `impl Renderer::prepare_gpu_encoded_paints` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:668` | - |  |
| `impl Renderer::encode_image_paint` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:721` | - |  |
| `impl Renderer::encode_external_texture_paint` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:747` | - |  |
| `impl Renderer::encode_gradient_paint` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:773` | - |  |
| `impl Renderer::encode_blurred_rounded_rect_paint` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:844` | 2d |  |
| `clear_atlas_region` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:863` | 2d |  |
| `create_arena_buffer` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:960` | - |  |
| `impl StripBufferArena::new` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:970` | - |  |
| `impl StripBufferArena::begin_frame` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:978` | - |  |
| `impl StripBufferArena::alloc` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:989` | - |  |
| `impl GpuResources::layer_views` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1055` | - |  |
| `impl GpuResources::layer_view` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1059` | - |  |
| `impl GpuResources::scratch_view` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1065` | - |  |
| `impl GpuStrip::vertex_attributes` | 0 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1074` | 3d | Vertex attributes for the strip |
| `impl Programs::new` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1087` | - |  |
| `impl Programs::create_intermediate_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1774` | - |  |
| `impl Programs::prepare_intermediate_textures` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1797` | - |  |
| `impl Programs::clear_layer_bind_group_caches` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1856` | - |  |
| `impl Programs::update_scratch_bind_groups` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1862` | - |  |
| `impl Programs::create_blend_layer_bind_group` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1876` | - |  |
| `impl Programs::create_copy_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1902` | - |  |
| `impl Programs::create_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1917` | - |  |
| `impl Programs::create_alphas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1939` | - |  |
| `impl Programs::create_atlas_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1956` | - |  |
| `impl Programs::create_filter_data_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1976` | - |  |
| `impl Programs::create_filter_base_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:1993` | - |  |
| `impl Programs::create_placeholder_external_texture` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2008` | - |  |
| `impl Programs::create_external_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2026` | - |  |
| `impl Programs::create_encoded_paints_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2042` | - |  |
| `impl Programs::create_encoded_paints_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2059` | - |  |
| `impl Programs::create_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2074` | - |  |
| `impl Programs::create_gradient_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2091` | - |  |
| `impl Programs::create_strip_bind_group` | 5 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2106` | - |  |
| `impl Programs::prepare` | 9 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2137` | - | Prepare GPU buffers for rendering, given alphas. |
| `impl Programs::maybe_resize_filter_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2165` | - |  |
| `impl Programs::maybe_resize_alphas_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2204` | - | Update the alpha texture size if needed. |
| `impl Programs::maybe_resize_encoded_paints_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2239` | - | Update the encoded paints texture size if needed. |
| `impl Programs::maybe_resize_gradient_tex` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2282` | - | Update the gradient texture size if needed. |
| `impl Programs::maybe_update_config_buffer` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2320` | - | Update config buffer if dimensions changed. |
| `impl Programs::maybe_create_atlas_textures` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2347` | - | Create any newly allocated atlas textures. |
| `impl Programs::create_run_external_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2361` | - |  |
| `impl Programs::upload_alpha_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2374` | - | Upload alpha data to the texture. |
| `impl Programs::upload_encoded_paints_texture` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2419` | - | Upload encoded paints to the texture. |
| `impl Programs::upload_filter_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2464` | - |  |
| `impl Programs::upload_gradient_texture` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2499` | - | Upload gradient data to the texture. |
| `impl Programs::upload_strip_pair` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2542` | - | Uploads two strip slices (opaque then alpha) into the frame arena and |
| `impl RendererContext<'_>::init_root_clear` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2593` | - |  |
| `impl RendererContext<'_>::finish_root_clear` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2607` | - |  |
| `impl RendererContext<'_>::take_root_load_op` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2615` | - |  |
| `impl RendererContext<'_>::clear_full_target` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2619` | - |  |
| `impl RendererContext<'_>::external_texture_bind_group_for_textures` | 2 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2638` | - |  |
| `impl RendererContext<'_>::strip_pass_inner` | 6 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2668` | - | Render the strips to the specified render target. |
| `impl RendererContext<'_>::texture_size` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2846` | - |  |
| `impl RendererContext<'_>::blend_pass_inner` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2850` | - |  |
| `impl RendererContext<'_>::filter_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:2945` | - |  |
| `impl RendererContext<'_>::clear_pass_inner` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3010` | - |  |
| `impl Backend for RendererContext<'_>::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3098` | - |  |
| `impl Backend for RendererContext<'_>::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3114` | - |  |
| `impl Backend for RendererContext<'_>::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3131` | - |  |
| `impl Backend for RendererContext<'_>::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3142` | - |  |
| `impl Backend for RendererContext<'_>::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3152` | - |  |
| `encode_copy_pass` | 7 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3165` | - |  |
| `encode_filter_pass` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3201` | - |  |
| `create_filter_input_bind_group` | 4 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3243` | - |  |
| `create_filter_original_texture_bind_group` | 3 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3265` | - |  |
| `trait AtlasWriter::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3288` | - | Get the width of the image. |
| `trait AtlasWriter::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3290` | - | Get the height of the image. |
| `trait AtlasWriter::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3293` | - | Write image data to an atlas texture at the specified offset. |
| `impl AtlasWriter for Texture::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3307` | - |  |
| `impl AtlasWriter for Texture::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3311` | - |  |
| `impl AtlasWriter for Texture::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3315` | - |  |
| `impl AtlasWriter for Pixmap::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3353` | - |  |
| `impl AtlasWriter for Pixmap::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3357` | - |  |
| `impl AtlasWriter for Pixmap::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3361` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::width` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3401` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::height` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3405` | - |  |
| `impl AtlasWriter for Arc<Pixmap>::write_to_atlas` | 8 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3409` | - |  |
| `clear_color` | 1 | `crates/vector-hybrid-renderer/src/render/wgpu/mod.rs:3424` | - |  |
| `impl Resources::new` | 1 | `crates/vector-hybrid-renderer/src/resources.rs:26` | - |  |
| `impl RecordedDraw::new_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:68` | - |  |
| `impl RecordedDraw::new_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:72` | 2d |  |
| `impl Drawable for RecordedDraw::bbox` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:82` | - |  |
| `impl Drawable for RecordedDraw::blend_mode` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:106` | - |  |
| `impl Default for LayersConfig::default` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:186` | - |  |
| `impl Default for RenderSettings::default` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:196` | - |  |
| `impl Scene::new` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:232` | - | Create a new render context with the given width and height in pixels. |
| `impl Scene::new_with` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:241` | - | Create a new render context with a specific SIMD level. |
| `impl Scene::active_rect` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:256` | 2d |  |
| `impl Scene::transforms` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:265` | - |  |
| `impl Scene::transforms_mut` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:269` | - |  |
| `impl Scene::effective_path_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:273` | - |  |
| `impl Scene::effective_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:278` | - |  |
| `impl Scene::encode_current_paint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:291` | - | Encode the current paint into a `Paint` that can be used for rendering. |
| `impl Scene::paint_transform_has_area` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:311` | 2d | Whether the effective paint transform maps paint space onto a non-zero area. |
| `impl Scene::paint_has_area` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:316` | - | Whether drawing with the current paint can produce any pixels. |
| `impl Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:321` | - | Fill a path with the current paint and fill rule. |
| `impl Scene::fill_path_with` | 6 | `crates/vector-hybrid-renderer/src/scene.rs:341` | - | Build strips for a filled path with the given properties and record the draw. |
| `impl Scene::push_clip_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:365` | - | Push a new clip path to the clip stack. |
| `impl Scene::push_clip_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:376` | 2d | Push a rectangular clip path. |
| `impl Scene::pop_clip` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:390` | - | Pop the most recently pushed clip path or clip rectangle from the clip stack. |
| `impl Scene::stroke_path` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:395` | - | Stroke a path with the current paint and stroke settings. |
| `impl Scene::stroke_path_with` | 5 | `crates/vector-hybrid-renderer/src/scene.rs:414` | - | Build strips for a stroked path with the given properties and record the draw. |
| `impl Scene::set_aliasing_threshold` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:445` | 2d | Set the aliasing threshold. |
| `impl Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:450` | 2d | Fill a rectangle with the current paint and fill rule. |
| `impl Scene::record_generated_path` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:491` | - |  |
| `impl Scene::fast_rect_bounds` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:514` | 2d |  |
| `impl Scene::stroke_rect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:538` | 2d | Stroke a rectangle with the current paint and stroke settings. |
| `impl Scene::fill_blurred_rounded_rect` | 5 | `crates/vector-hybrid-renderer/src/scene.rs:550` | 2d | Fill a blurred rectangle with the given corner radius and standard deviation. |
| `impl Scene::glyph_run` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:614` | - | Creates a builder for drawing a run of glyphs that have the same attributes. |
| `impl Scene::push_layer` | 6 | `crates/vector-hybrid-renderer/src/scene.rs:636` | - | Push a new layer with the given properties. |
| `impl Scene::push_clip_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:702` | - | Push a new clip layer. |
| `impl Scene::push_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:707` | - | Push a new blend layer. |
| `impl Scene::push_opacity_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:712` | - | Push a new opacity layer. |
| `impl Scene::push_mask_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:721` | - | Push a new mask layer. |
| `impl Scene::push_filter_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:726` | - | Push a new filter layer. |
| `impl Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:731` | - | Pop the last pushed layer. |
| `impl Scene::set_blend_mode` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:744` | - | Set the blend mode for subsequent rendering operations. |
| `impl Scene::set_stroke` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:754` | - | Set the stroke settings for subsequent stroke operations. |
| `impl Scene::stroke` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:759` | - | Get the current stroke. |
| `impl Scene::stroke_mut` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:765` | - | Get a mutable reference to the current stroke. |
| `impl Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:773` | - |  |
| `impl Scene::set_tint` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:778` | - | Set the tint for subsequent image paint operations. |
| `impl Scene::reset_tint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:783` | - | Clear the tint, so subsequent image paints are drawn without tinting. |
| `impl Scene::paint` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:788` | - | Get the current paint. |
| `impl Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:796` | - | Set the current paint transform. |
| `impl Scene::reset_paint_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:801` | - | Reset the current paint transform. |
| `impl Scene::set_fill_rule` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:806` | - | Set the fill rule for subsequent fill operations. |
| `impl Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:811` | - | Set the transform for subsequent rendering operations. |
| `impl Scene::reset_transform` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:816` | - | Reset the transform to identity. |
| `impl Scene::set_filter_effect` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:821` | - | Apply filter to the current paint (affects next drawn element). |
| `impl Scene::reset_filter_effect` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:826` | - | Reset the current filter effect. |
| `impl Scene::with_optional_filter_or_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:830` | - |  |
| `impl Scene::reset_and_resize` | 3 | `crates/vector-hybrid-renderer/src/scene.rs:849` | - | Reset the scene and update its size. |
| `impl Scene::reset` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:857` | - | Reset scene to default values. |
| `impl Scene::width` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:873` | - | Get the width of the render context. |
| `impl Scene::height` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:878` | - | Get the height of the render context. |
| `impl Scene::take_current_state` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:883` | - | Take current rendering state and reset the existing state to its default. |
| `impl Scene::save_current_state` | 1 | `crates/vector-hybrid-renderer/src/scene.rs:888` | - | Save a copy of the current rendering state. |
| `impl Scene::restore_state` | 2 | `crates/vector-hybrid-renderer/src/scene.rs:893` | - | Restore rendering state. |
| `recorded_rect_bbox_rounds_fractional_outward` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:918` | 2d |  |
| `rectangular_clip_keeps_direct_rect_draw_path` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:928` | 2d |  |
| `reset_restores_default_aliasing_threshold` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:943` | - |  |
| `reset_and_resize_updates_scene_size` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:953` | - |  |
| `empty_generated_path_is_not_recorded` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:972` | - |  |
| `zero_area_external_texture_rect_is_a_noop` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:983` | 2d |  |
| `glyph_atlas_resources_are_lazy` | 0 | `crates/vector-hybrid-renderer/src/scene.rs:1000` | - |  |
| `impl Atlases::new` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:34` | - |  |
| `impl Atlases::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:42` | - |  |
| `impl Atlases::texture_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:46` | - |  |
| `impl Atlases::allocate_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:50` | - |  |
| `impl Atlases::deallocate` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:76` | - |  |
| `impl Atlases::require_scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:84` | - |  |
| `impl Atlases::add_layer_atlas` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:88` | - |  |
| `impl LayerAllocationRequest::new` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:112` | - |  |
| `impl LayerAllocationRequest::allocation_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:136` | - |  |
| `impl RegionProps::allocation_size` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:152` | - | Size of the atlas allocation needed to hold the region and its padding. |
| `impl AllocatedTextureRegion::new` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:172` | - |  |
| `impl AllocatedTextureRegion::clear_region` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:180` | - |  |
| `impl AllocatedTextureRegion::allocation_region` | 1 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:185` | - |  |
| `trait AtlasExt::allocate_region` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:196` | - |  |
| `trait AtlasExt::deallocate_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:202` | - |  |
| `impl AtlasExt for Atlas::allocate_region` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:206` | - |  |
| `impl AtlasExt for Atlas::deallocate_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:230` | - |  |
| `request` | 3 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:254` | - |  |
| `layer_requests` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:266` | - |  |
| `page_selection` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:297` | - |  |
| `page_capacity` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:335` | - |  |
| `padded_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:361` | - |  |
| `oversized_regions` | 0 | `crates/vector-hybrid-renderer/src/schedule/allocate.rs:381` | - |  |
| `impl Cursor::new` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:29` | - |  |
| `impl Cursor::current_round` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:37` | - |  |
| `impl Cursor::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:42` | - | Whether access to the scratch texture has been requested. |
| `impl Cursor::require_scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:47` | - | Indicate that access to the scratch texture is required. |
| `impl Cursor::allocate_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:51` | - |  |
| `impl Cursor::allocate_reusing` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:94` | - | Try to accommodate the given layer allocation request, possibly advancing the round counter |
| `impl Cursor::release` | 3 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:114` | - | Mark the given allocation as ready to release in the given round. |
| `impl Cursor::advance` | 1 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:128` | - | Advance to the next round. |
| `cursor` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:148` | - |  |
| `request` | 2 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:152` | - |  |
| `current_space` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:160` | - |  |
| `deferred_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:178` | - |  |
| `page_growth` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:191` | - |  |
| `oversized_layer_reports_texture_dimensions` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:210` | - |  |
| `past_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/cursor.rs:228` | - |  |
| `trait Backend::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:27` | - | Execute the opaque pass against the root target with the given strips. If this method is |
| `trait Backend::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:36` | - | Execute a draw pass against the given target. |
| `trait Backend::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:46` | - | Execute a clear pass with the given rectangles against the target. |
| `trait Backend::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:51` | - | Execute a blend pass between a parent and child texture. |
| `trait Backend::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:61` | - | Execute a filter pass according to the filter plan between two textures. |
| `execute` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:68` | - |  |
| `impl Schedule::execute` | 5 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:84` | - |  |
| `impl Rounds::execute` | 6 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:111` | - |  |
| `impl Call::stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:200` | - |  |
| `impl Backend for Recorder::opaque_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:230` | - |  |
| `impl Backend for Recorder::draw_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:240` | - |  |
| `impl Backend for Recorder::clear_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:251` | - |  |
| `impl Backend for Recorder::blend_pass` | 4 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:261` | - |  |
| `impl Backend for Recorder::filter_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:273` | - |  |
| `layer_id` | 1 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:284` | - |  |
| `execution_case` | 2 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:288` | - |  |
| `chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:289` | - |  |
| `round_order_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:306` | - |  |
| `round_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:343` | - |  |
| `child_bindings` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:375` | - |  |
| `atlas_skips_opaque` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:405` | - |  |
| `empty_passes_are_skipped` | 0 | `crates/vector-hybrid-renderer/src/schedule/execute.rs:413` | - |  |
| `impl Schedule::try_new` | 8 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:166` | - |  |
| `impl Schedule::intermediate_texture_requirements` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:208` | - |  |
| `impl IntermediateTextureAllocations::combine` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:233` | - | Combine the current set of allocations with another one. |
| `impl IntermediateTextureAllocations::texture_count` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:243` | - | Return the total number of physical textures represented by these allocations. |
| `impl IntermediateTextureRequirements::validate` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:251` | - | Validate that satisfying these requirements from the backend's existing allocations stays |
| `impl Scheduler<'a, 'p>::new` | 8 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:296` | - | Create a scheduler. |
| `impl Scheduler<'a, 'p>::build` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:320` | - | Build the complete dependency-ordered schedule. |
| `impl Scheduler<'a, 'p>::schedule_root` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:344` | - | Schedule the root command stream into the configured output target. |
| `impl Scheduler<'a, 'p>::schedule_layer` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:400` | - | Schedule an open layer bottom-up and return its completed, still-live allocation. |
| `impl Scheduler<'a, 'p>::schedule_child_layer` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:485` | graph | Resolve and schedule the optional child layer referenced by a command node. |
| `impl Scheduler<'a, 'p>::open_layer` | 3 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:536` | - | Create an unallocated scheduling view of a recorded layer with the given visible bounds. |
| `impl Scheduler<'a, 'p>::open_root_layer` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:553` | - | Represent the root command stream as an intermediate layer that can be sampled. |
| `impl Scheduler<'a, 'p>::layer_texture_parity` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:565` | - | Select the texture group for a recorded layer depth. |
| `impl Scheduler<'a, 'p>::push_draws` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:570` | - | Append a recorded draw range to the next compatible draw pass for the target. |
| `impl Scheduler<'a, 'p>::compose_layer` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:594` | - | Compose a rendered child into a parent layer. |
| `impl Scheduler<'a, 'p>::compose_simple_layer` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:732` | - | Compose a completed child by drawing it directly into its parent using source-over. |
| `impl Scheduler<'a, 'p>::clear_and_release_allocation` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:763` | - | Schedule an allocation to be cleared and released after the given round. |
| `impl Scheduler<'a, 'p>::ensure_layer_target` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:800` | - | Lazily allocate a target for an open layer and return its scheduling state. |
| `impl LayerSamplePlacement::regular` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:884` | - |  |
| `impl LayerSamplePlacement::filter` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:891` | - |  |
| `impl LayerSamplePlacement::resolve_sample_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:898` | - |  |
| `impl Rounds::build_draw` | 5 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:929` | - |  |
| `impl ScheduleBuffers::clear` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:981` | - |  |
| `impl ScheduleStorage::clear` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1001` | - |  |
| `impl TargetScheduleState<T>::new` | 4 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1017` | - |  |
| `impl TargetScheduleState<T>::next_draw` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1040` | - | Return the earliest point at which another draw can be appended to this target. |
| `impl TargetScheduleState<T>::next_draw_after` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1045` | - | Return the earliest draw point that executes after an external dependency. |
| `impl TargetScheduleState<LayerTextureRegion>::new_layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1053` | - |  |
| `trait ScheduleTarget::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1059` | - |  |
| `trait ScheduleTarget::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1060` | - |  |
| `trait ScheduleTarget::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1061` | - |  |
| `impl ScheduleTarget for RootTarget::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1065` | - |  |
| `impl ScheduleTarget for RootTarget::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1069` | - |  |
| `impl ScheduleTarget for RootTarget::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1073` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1079` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::draw_stage` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1083` | - |  |
| `impl ScheduleTarget for LayerTextureRegion::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/mod.rs:1087` | - |  |
| `impl Rounds::iter` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:48` | - |  |
| `impl Rounds::required_layer_pages` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:53` | - | Derive the required page count for the even and odd texture groups. |
| `impl Rounds::round_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:69` | - |  |
| `impl Rounds::push_layer_clear` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:73` | 2d |  |
| `impl Rounds::ensure_exists` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:84` | - |  |
| `impl Rounds::bind_at_or_after` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:91` | - | Bind the required texture pages in the first compatible round at or after `point`. |
| `impl Rounds::validate` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:130` | - | Validate the texture-binding invariants assumed by the render backends. |
| `impl Round::layer_passes` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:151` | - |  |
| `impl Round::root_draw_pass` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:192` | - |  |
| `impl Round::clear_passes` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:216` | - |  |
| `impl Round::root_draw_mut` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:233` | - |  |
| `impl Round::layer_draw_mut` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:237` | - |  |
| `impl Round::push_blend_op` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:241` | - |  |
| `impl Round::push_filter_op` | 4 | `crates/vector-hybrid-renderer/src/schedule/round.rs:253` | - |  |
| `impl Round::validate` | 3 | `crates/vector-hybrid-renderer/src/schedule/round.rs:269` | - | Validate the bindings required by this round's scheduled work. |
| `impl RoundStage::draw` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:361` | - |  |
| `impl RoundStage::filter` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:368` | - |  |
| `impl RoundStage::blend` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:375` | - |  |
| `impl SchedulePoint::start` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:394` | - |  |
| `impl SchedulePoint::after` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:402` | - | Return the first occurrence of `stage` strictly after this point. |
| `impl SchedulePoint::after_or_at` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:417` | - | Return the first occurrence of `stage` at or after this point. |
| `impl FilterTextureRegions::new` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:493` | - |  |
| `impl FilterTextureRegions::round_bindings` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:500` | - |  |
| `layer_id` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:551` | - |  |
| `region` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:555` | - |  |
| `layer_region` | 2 | `crates/vector-hybrid-renderer/src/schedule/round.rs:562` | - |  |
| `filter_op` | 1 | `crates/vector-hybrid-renderer/src/schedule/round.rs:569` | - |  |
| `schedule_point_stage_occurrences` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:581` | - |  |
| `unused_bindings_do_not_resolve_passes` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:619` | - |  |
| `pass_iterators_skip_empty_parities` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:632` | - |  |
| `validator_accepts_bound_work` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:661` | - |  |
| `validator_rejects_unbound_layer_pass` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:679` | - |  |
| `validator_rejects_unbound_clear` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:692` | - |  |
| `binding_conflicts_even` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:701` | - |  |
| `binding_conflicts_odd` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:740` | - |  |
| `binding_conflicts_skip_rounds` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:773` | - |  |
| `operation_ranges` | 0 | `crates/vector-hybrid-renderer/src/schedule/round.rs:802` | - |  |
| `probe_scene_uses_depth_buffer_when_enabled` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:18` | - |  |
| `intermediate_texture_requirements_validate_limit` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:38` | - |  |
| `blend_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:90` | - |  |
| `root_blend_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:104` | - |  |
| `add_chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:115` | - |  |
| `add_blend_chain` | 2 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:125` | - |  |
| `add_tree` | 3 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:140` | - |  |
| `chain_case` | 1 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:152` | - |  |
| `binding_case` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:159` | - |  |
| `sibling_case` | 1 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:175` | - |  |
| `offset_filter` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:183` | - |  |
| `filter_page_size` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:187` | - |  |
| `empty_scene` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:192` | - |  |
| `root_draws` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:201` | - |  |
| `draw_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:217` | - |  |
| `opaque_root` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:232` | - |  |
| `root_without_depth_preserves_interleaved_painter_order` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:255` | - |  |
| `simple_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:271` | - |  |
| `default_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:288` | - |  |
| `non_default_blend_into_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:299` | - |  |
| `clipped_away_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:308` | - |  |
| `clipped_away_filtered_blend_under_atlas_pressure` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:332` | - |  |
| `blend_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:366` | - |  |
| `root_blend_resources` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:382` | - |  |
| `root_release` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:393` | - |  |
| `root_blend_budget` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:407` | - |  |
| `nested_parity` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:426` | - |  |
| `even_child` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:448` | - |  |
| `odd_child` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:458` | - |  |
| `draw_after_blend` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:469` | - |  |
| `sibling_batch` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:494` | - |  |
| `incompatible_siblings` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:514` | - |  |
| `filter_binding_conflict` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:541` | - |  |
| `deep_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:558` | - |  |
| `deeply_nested_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:584` | - |  |
| `deeply_nested_blend_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:599` | - |  |
| `wide_layers` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:617` | - |  |
| `nested_children` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:636` | - |  |
| `nested_children_spilled` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:662` | - |  |
| `empty_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:692` | - |  |
| `destructive_empty` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:703` | - |  |
| `layer_clip` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:723` | - |  |
| `filter_layer` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:750` | - |  |
| `drop_shadow_only_uses_two_layer_textures` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:772` | - |  |
| `filter_round_resolving` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:794` | - |  |
| `filter_siblings` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:836` | - |  |
| `storage_reuse` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:858` | - |  |
| `blend_is_constrained_to_parent_clip_bbox` | 0 | `crates/vector-hybrid-renderer/src/schedule/schedule_tests.rs:926` | - |  |
| `impl SceneCase::new` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:24` | - |  |
| `impl SceneCase::draw` | 3 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:30` | 2d |  |
| `impl SceneCase::draw_at` | 3 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:35` | - |  |
| `impl SceneCase::layer` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:39` | - |  |
| `impl SceneCase::layer_with` | 5 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:43` | 2d |  |
| `impl SceneCase::schedule` | 5 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:57` | - |  |
| `impl SceneCase::schedule_into` | 6 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:85` | - |  |
| `impl SceneCase::schedule_root` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:107` | - |  |
| `impl ScheduledCase::execute` | 2 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:142` | - |  |
| `impl ScheduledCase::page_counts` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:149` | - |  |
| `impl ScheduledCase::scratch_texture` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:156` | - |  |
| `impl ScheduledCase::views` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:163` | - |  |
| `impl ScheduledCase::opaque_x` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:206` | - |  |
| `impl ScheduledCase::total_clears` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:217` | - |  |
| `impl ScheduledCase::draw_view` | 1 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:226` | - |  |
| `impl ScheduledCase::empty_draw_view` | 0 | `crates/vector-hybrid-renderer/src/schedule/test_support.rs:233` | - |  |
| `impl DrawPassTarget::enable_opaque` | 1 | `crates/vector-hybrid-renderer/src/target.rs:27` | - |  |
| `impl TextureParity::from_parity` | 1 | `crates/vector-hybrid-renderer/src/target.rs:44` | - |  |
| `impl TextureParity::get_parity` | 1 | `crates/vector-hybrid-renderer/src/target.rs:52` | - |  |
| `impl TextureParity::opposite` | 1 | `crates/vector-hybrid-renderer/src/target.rs:59` | - |  |
| `impl From<TextureParity> for u8::from` | 1 | `crates/vector-hybrid-renderer/src/target.rs:68` | - |  |
| `impl From<TextureParity> for u32::from` | 1 | `crates/vector-hybrid-renderer/src/target.rs:74` | - |  |
| `impl LayerTextureId::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:89` | - |  |
| `impl DrawPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:107` | - |  |
| `impl FilterPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:122` | - |  |
| `impl FilterPassBindings::target` | 1 | `crates/vector-hybrid-renderer/src/target.rs:131` | - |  |
| `impl FilterPassBindings::temporary` | 1 | `crates/vector-hybrid-renderer/src/target.rs:135` | - |  |
| `impl FilterPassBindings::input` | 2 | `crates/vector-hybrid-renderer/src/target.rs:139` | - |  |
| `impl FilterPassBindings::output` | 2 | `crates/vector-hybrid-renderer/src/target.rs:147` | - |  |
| `impl BlendPassBindings::new` | 2 | `crates/vector-hybrid-renderer/src/target.rs:166` | - |  |
| `impl BlendPassBindings::target` | 1 | `crates/vector-hybrid-renderer/src/target.rs:175` | - |  |
| `impl BlendPassBindings::layer_id` | 2 | `crates/vector-hybrid-renderer/src/target.rs:179` | - |  |
| `impl RoundBindings::new` | 1 | `crates/vector-hybrid-renderer/src/target.rs:196` | - |  |
| `impl RoundBindings::merge` | 2 | `crates/vector-hybrid-renderer/src/target.rs:205` | - | Try to merge this binding with the other one. |
| `impl RoundBindings::layer_id` | 2 | `crates/vector-hybrid-renderer/src/target.rs:218` | - | Return the bound texture for a parity, if present. |
| `impl RoundBindings::required_textures` | 1 | `crates/vector-hybrid-renderer/src/target.rs:223` | - | Return the required textures for this binding. |
| `impl RoundBindings::page_indices` | 1 | `crates/vector-hybrid-renderer/src/target.rs:232` | - |  |
| `impl LayerTextureRegion::crop_to` | 2 | `crates/vector-hybrid-renderer/src/target.rs:257` | - | Restrict this region to the given scene-space bounds while preserving its texture mapping. |
| `impl LayerTextureRegion::texture_rect` | 2 | `crates/vector-hybrid-renderer/src/target.rs:272` | 2d | Translate a scene-space rectangle within this layer to texture coordinates. |
| `trait DrawTarget::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:282` | - | Whether this target supports the depth-buffer optimization. |
| `trait DrawTarget::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:285` | - | A positional shift that needs to be applied to all geometry when rendering to this target. |
| `impl DrawTarget for RootTarget::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:289` | - |  |
| `impl DrawTarget for RootTarget::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:293` | - |  |
| `impl DrawTarget for LayerTextureRegion::enable_depth` | 1 | `crates/vector-hybrid-renderer/src/target.rs:299` | - |  |
| `impl DrawTarget for LayerTextureRegion::geometry_shift` | 1 | `crates/vector-hybrid-renderer/src/target.rs:303` | - |  |
| `texture_pair_constraints` | 0 | `crates/vector-hybrid-renderer/src/target.rs:319` | - |  |
| `texture_rect_translation` | 0 | `crates/vector-hybrid-renderer/src/target.rs:334` | 2d |  |
| `crop_preserves_texture_mapping` | 0 | `crates/vector-hybrid-renderer/src/target.rs:350` | - |  |
| `impl GlyphAtlasResources::with_config` | 3 | `crates/vector-hybrid-renderer/src/text.rs:38` | - |  |
| `impl GlyphAtlasResources::maintain` | 2 | `crates/vector-hybrid-renderer/src/text.rs:49` | - |  |
| `impl Resources::ensure_glyph_resources` | 1 | `crates/vector-hybrid-renderer/src/text.rs:55` | - |  |
| `impl Resources::atlas_config` | 1 | `crates/vector-hybrid-renderer/src/text.rs:67` | - |  |
| `impl Resources::atlas_count` | 1 | `crates/vector-hybrid-renderer/src/text.rs:71` | - |  |
| `impl Resources::replay_pending_atlas_commands` | 2 | `crates/vector-hybrid-renderer/src/text.rs:75` | - |  |
| `impl Resources::before_render` | 4 | `crates/vector-hybrid-renderer/src/text.rs:98` | - |  |
| `impl Resources::after_render` | 3 | `crates/vector-hybrid-renderer/src/text.rs:131` | 2d |  |
| `impl DrawSink for Scene::set_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:152` | - |  |
| `impl DrawSink for Scene::set_paint` | 2 | `crates/vector-hybrid-renderer/src/text.rs:157` | - |  |
| `impl DrawSink for Scene::set_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:162` | - |  |
| `impl DrawSink for Scene::fill_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:167` | - |  |
| `impl DrawSink for Scene::fill_rect` | 2 | `crates/vector-hybrid-renderer/src/text.rs:172` | 2d |  |
| `impl DrawSink for Scene::push_clip_layer` | 2 | `crates/vector-hybrid-renderer/src/text.rs:177` | - |  |
| `impl DrawSink for Scene::push_clip_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:182` | - |  |
| `impl DrawSink for Scene::push_clip_rect` | 2 | `crates/vector-hybrid-renderer/src/text.rs:187` | 2d |  |
| `impl DrawSink for Scene::push_blend_layer` | 2 | `crates/vector-hybrid-renderer/src/text.rs:192` | - |  |
| `impl DrawSink for Scene::pop_layer` | 1 | `crates/vector-hybrid-renderer/src/text.rs:197` | - |  |
| `impl DrawSink for Scene::pop_clip_path` | 1 | `crates/vector-hybrid-renderer/src/text.rs:202` | - |  |
| `impl DrawSink for Scene::width` | 1 | `crates/vector-hybrid-renderer/src/text.rs:207` | - |  |
| `impl DrawSink for Scene::height` | 1 | `crates/vector-hybrid-renderer/src/text.rs:212` | - |  |
| `impl GpuGlyphRunBackend<'a>::render_glyphs` | 4 | `crates/vector-hybrid-renderer/src/text.rs:226` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::atlas_cache` | 2 | `crates/vector-hybrid-renderer/src/text.rs:260` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::fill_glyphs` | 3 | `crates/vector-hybrid-renderer/src/text.rs:265` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::stroke_glyphs` | 3 | `crates/vector-hybrid-renderer/src/text.rs:276` | - |  |
| `impl GlyphRunBackend<'a> for GpuGlyphRunBackend<'a>::render_decoration` | 8 | `crates/vector-hybrid-renderer/src/text.rs:294` | - |  |
| `impl glifo::GlyphRenderer for Scene::save_state` | 1 | `crates/vector-hybrid-renderer/src/text.rs:319` | - |  |
| `impl glifo::GlyphRenderer for Scene::restore_state` | 2 | `crates/vector-hybrid-renderer/src/text.rs:324` | - |  |
| `impl glifo::GlyphRenderer for Scene::stroke_path` | 2 | `crates/vector-hybrid-renderer/src/text.rs:329` | - |  |
| `impl glifo::GlyphRenderer for Scene::set_paint_image` | 2 | `crates/vector-hybrid-renderer/src/text.rs:334` | - |  |
| `impl glifo::GlyphRenderer for Scene::set_tint` | 2 | `crates/vector-hybrid-renderer/src/text.rs:339` | - |  |
| `impl glifo::GlyphRenderer for Scene::get_context_color` | 1 | `crates/vector-hybrid-renderer/src/text.rs:344` | - |  |
| `impl glifo::GlyphRenderer for Scene::current_paint` | 1 | `crates/vector-hybrid-renderer/src/text.rs:353` | - |  |
| `impl glifo::GlyphRenderer for Scene::atlas_image_source` | 2 | `crates/vector-hybrid-renderer/src/text.rs:358` | - |  |
| `impl glifo::GlyphRenderer for Scene::atlas_paint_transform` | 2 | `crates/vector-hybrid-renderer/src/text.rs:363` | - |  |
| `impl DimensionConstraints::new` | 4 | `crates/vector-hybrid-renderer/src/util.rs:23` | - | Create new constraints with given min/max dimensions |
| `impl DimensionConstraints::calculate_dimensions` | 3 | `crates/vector-hybrid-renderer/src/util.rs:36` | - | Calculate dimensions while preserving aspect ratio within constraints |
| `impl DimensionConstraints::convert_dimension` | 1 | `crates/vector-hybrid-renderer/src/util.rs:82` | - |  |
| `impl Default for DimensionConstraints::default` | 0 | `crates/vector-hybrid-renderer/src/util.rs:91` | - | Creates default constraints with reasonable values |
| `pack_u16_pair` | 2 | `crates/vector-hybrid-renderer/src/util.rs:99` | - |  |
| `pack_opacity` | 1 | `crates/vector-hybrid-renderer/src/util.rs:107` | - |  |
| `impl Ranges::push` | 2 | `crates/vector-hybrid-renderer/src/util.rs:121` | - |  |
| `impl Ranges::len` | 1 | `crates/vector-hybrid-renderer/src/util.rs:132` | - |  |
| `impl Clear for Ranges::clear` | 1 | `crates/vector-hybrid-renderer/src/util.rs:138` | - |  |
| `trait VecExt::push_ranged` | 3 | `crates/vector-hybrid-renderer/src/util.rs:145` | - |  |
| `trait VecExt::ranged` | 2 | `crates/vector-hybrid-renderer/src/util.rs:147` | - |  |
| `impl VecExt<T> for Vec<T>::push_ranged` | 3 | `crates/vector-hybrid-renderer/src/util.rs:151` | - |  |
| `impl VecExt<T> for Vec<T>::ranged` | 2 | `crates/vector-hybrid-renderer/src/util.rs:157` | - |  |
| `impl RangedSlice<'a, T>::empty` | 0 | `crates/vector-hybrid-renderer/src/util.rs:174` | - |  |
| `impl RangedSlice<'a, T>::new` | 2 | `crates/vector-hybrid-renderer/src/util.rs:182` | - |  |
| `impl RangedSlice<'a, T>::len` | 1 | `crates/vector-hybrid-renderer/src/util.rs:190` | - |  |
| `impl RangedSlice<'a, T>::slices` | 1 | `crates/vector-hybrid-renderer/src/util.rs:194` | - |  |
| `impl RangedSlice<'a, T>::iter` | 1 | `crates/vector-hybrid-renderer/src/util.rs:198` | - |  |
| `ranged_slices` | 0 | `crates/vector-hybrid-renderer/src/util.rs:211` | - |  |
| `ranges_clear` | 0 | `crates/vector-hybrid-renderer/src/util.rs:236` | - |  |
| `calculate_dimensions_in_range` | 0 | `crates/vector-hybrid-renderer/src/util.rs:255` | - |  |

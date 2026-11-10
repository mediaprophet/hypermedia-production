# `crates/vector-cpu-render-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_A [`vello_cpu`] backend for the [`anyrender`] 2D drawing abstraction_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `convert_filter` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:9` | - |  |
| `convert_filter_effect` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:23` | - |  |
| `convert_edge_mode` | 1 | `crates/vector-cpu-render-adapter/src/filters.rs:56` | graph |  |
| `impl Default for ImageCacheConfig::default` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:27` | - |  |
| `impl ImageCache::new` | 1 | `crates/vector-cpu-render-adapter/src/image_cache.rs:57` | - |  |
| `impl ImageCache::get_or_register` | 3 | `crates/vector-cpu-render-adapter/src/image_cache.rs:66` | - | Look up the converted image for `image`, converting and registering it |
| `impl ImageCache::maintain` | 2 | `crates/vector-cpu-render-adapter/src/image_cache.rs:94` | - | Advance the frame counter and evict stale entries. |
| `impl ImageCache::clear` | 2 | `crates/vector-cpu-render-adapter/src/image_cache.rs:132` | - | Drop all cached conversions and their registry entries. |
| `image` | 1 | `crates/vector-cpu-render-adapter/src/image_cache.rs:146` | - |  |
| `cache_hit_reuses_registered_image` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:157` | - |  |
| `dead_blob_is_evicted` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:174` | - |  |
| `old_entries_age_out` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:192` | - |  |
| `byte_budget_evicts_lru_but_not_current_frame` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:210` | - |  |
| `clear_destroys_registry_entries` | 0 | `crates/vector-cpu-render-adapter/src/image_cache.rs:227` | - |  |
| `convert_image` | 1 | `crates/vector-cpu-render-adapter/src/image_convert.rs:15` | - | Convert a [`peniko::ImageData`] to a premultiplied RGBA8 [`Pixmap`]. |
| `image` | 1 | `crates/vector-cpu-render-adapter/src/image_convert.rs:61` | - |  |
| `premultiply` | 0 | `crates/vector-cpu-render-adapter/src/image_convert.rs:72` | - |  |
| `premultiply_bgra_opaque` | 0 | `crates/vector-cpu-render-adapter/src/image_convert.rs:82` | - |  |
| `impl VelloCpuImageRenderer::with_image_cache_config` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:12` | - | Create a renderer with a custom image cache configuration. |
| `impl VelloCpuImageRenderer::clear_image_cache` | 1 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:23` | - | Drop all cached image conversions. |
| `impl ImageRenderer for VelloCpuImageRenderer::new` | 2 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:32` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::resize` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:38` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::reset` | 1 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:42` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::render` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:46` | - |  |
| `impl ImageRenderer for VelloCpuImageRenderer::render_to_vec` | 3 | `crates/vector-cpu-render-adapter/src/image_renderer.rs:72` | - |  |
| `impl VelloCpuScenePainter::new` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:20` | - |  |
| `impl VelloCpuScenePainter::with_image_cache_config` | 3 | `crates/vector-cpu-render-adapter/src/scene.rs:24` | - |  |
| `impl VelloCpuScenePainter::convert_paint` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:32` | - |  |
| `impl VelloCpuScenePainter::maintain` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:51` | - | Advance the image cache's frame counter and evict stale entries. |
| `impl VelloCpuScenePainter::clear_image_cache` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:56` | - | Drop all cached image conversions. |
| `impl VelloCpuScenePainter::finish` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:60` | - |  |
| `impl PaintScene for VelloCpuScenePainter::reset` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:69` | - |  |
| `impl PaintScene for VelloCpuScenePainter::push_layer` | 8 | `crates/vector-cpu-render-adapter/src/scene.rs:73` | - |  |
| `impl PaintScene for VelloCpuScenePainter::push_clip_layer` | 4 | `crates/vector-cpu-render-adapter/src/scene.rs:105` | - |  |
| `impl PaintScene for VelloCpuScenePainter::pop_layer` | 1 | `crates/vector-cpu-render-adapter/src/scene.rs:112` | - |  |
| `impl PaintScene for VelloCpuScenePainter::stroke` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:116` | - |  |
| `impl PaintScene for VelloCpuScenePainter::fill` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:134` | - |  |
| `impl PaintScene for VelloCpuScenePainter::draw_glyphs` | 12 | `crates/vector-cpu-render-adapter/src/scene.rs:152` | 2d |  |
| `impl PaintScene for VelloCpuScenePainter::draw_box_shadow` | 6 | `crates/vector-cpu-render-adapter/src/scene.rs:205` | 2d |  |
| `render_clip` | 3 | `crates/vector-cpu-render-adapter/src/scene.rs:229` | - |  |
| `draw_clipped` | 4 | `crates/vector-cpu-render-adapter/src/scene.rs:243` | - |  |
| `assert_pixels` | 2 | `crates/vector-cpu-render-adapter/src/scene.rs:264` | - |  |
| `clip_layers_respect_fill_rule` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:281` | - |  |
| `compositing_layers_respect_fill_rule` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:286` | - |  |
| `scene_replay_preserves_clip_rules` | 0 | `crates/vector-cpu-render-adapter/src/scene.rs:291` | - |  |

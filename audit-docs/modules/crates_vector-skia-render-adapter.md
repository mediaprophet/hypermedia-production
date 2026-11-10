# `crates/vector-skia-render-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for GenerationalCache<K, T>::default` | 0 | `crates/vector-skia-render-adapter/src/cache.rs:23` | - |  |
| `impl GenerationalCache<K, T>::new` | 1 | `crates/vector-skia-render-adapter/src/cache.rs:36` | - |  |
| `impl GenerationalCache<K, T>::next_gen` | 1 | `crates/vector-skia-render-adapter/src/cache.rs:44` | - |  |
| `impl GenerationalCache<K, T>::contains_key` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:53` | - |  |
| `impl GenerationalCache<K, T>::hit` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:60` | - |  |
| `impl GenerationalCache<K, T>::insert` | 3 | `crates/vector-skia-render-adapter/src/cache.rs:72` | - |  |
| `impl Hash for NormalizedTypefaceCacheKey::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:89` | - |  |
| `impl Hash for NormalizedTypefaceCacheKeyBorrowed<'a>::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:105` | - |  |
| `impl Equivalent<NormalizedTypefaceCacheKey> for NormalizedTypefaceCacheKeyBorrowed<'_>::equivalent` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:115` | - |  |
| `impl Hash for FontCacheKeyBorrowed<'a>::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:131` | - |  |
| `impl Hash for FontCacheKey::hash` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:152` | - |  |
| `impl Equivalent<FontCacheKey> for FontCacheKeyBorrowed<'_>::equivalent` | 2 | `crates/vector-skia-render-adapter/src/cache.rs:164` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::new` | 2 | `crates/vector-skia-render-adapter/src/image_renderer.rs:20` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::resize` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:37` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::reset` | 1 | `crates/vector-skia-render-adapter/src/image_renderer.rs:46` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::render_to_vec` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:48` | - |  |
| `impl ImageRenderer for SkiaImageRenderer::render` | 3 | `crates/vector-skia-render-adapter/src/image_renderer.rs:80` | - |  |
| `impl MetalBackend::new` | 4 | `crates/vector-skia-render-adapter/src/metal.rs:27` | - |  |
| `impl SkiaBackend for MetalBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/metal.rs:102` | - |  |
| `impl SkiaBackend for MetalBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/metal.rs:107` | - |  |
| `impl SkiaBackend for MetalBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/metal.rs:141` | - |  |
| `impl OpenGLBackend::new` | 4 | `crates/vector-skia-render-adapter/src/opengl.rs:30` | - |  |
| `impl OpenGLBackend::create_surface` | 6 | `crates/vector-skia-render-adapter/src/opengl.rs:167` | - |  |
| `impl SkiaBackend for OpenGLBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/opengl.rs:201` | - |  |
| `impl SkiaBackend for OpenGLBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/opengl.rs:212` | - |  |
| `impl SkiaBackend for OpenGLBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/opengl.rs:217` | - |  |
| `impl SkiaSceneCache::new` | 0 | `crates/vector-skia-render-adapter/src/scene.rs:45` | - | Create a new `SkiaSceneCache` |
| `impl SkiaSceneCache::next_gen` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:50` | - | Increment the generation on the caches. Should be called once per frame. |
| `impl Default for SkiaSceneCache::default` | 0 | `crates/vector-skia-render-adapter/src/scene.rs:59` | - |  |
| `impl SkiaScenePainter<'_>::new` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:82` | 2d |  |
| `impl SkiaScenePainter<'_>::reset_paint` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:89` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_alpha` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:94` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_filter` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:98` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_blend_mode` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:102` | - |  |
| `impl SkiaScenePainter<'_>::set_matrix` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:108` | numerics |  |
| `impl SkiaScenePainter<'_>::concat_matrix` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:112` | numerics |  |
| `impl SkiaScenePainter<'_>::clip` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:116` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_brush` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:129` | - |  |
| `impl SkiaScenePainter<'_>::set_paint_style` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:176` | - |  |
| `impl SkiaScenePainter<'_>::draw_shape` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:212` | - |  |
| `impl SkiaScenePainter<'_>::draw_shape_with_fill` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:216` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_font` | 5 | `crates/vector-skia-render-adapter/src/scene.rs:248` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_normalized_typeface` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:291` | - |  |
| `f2dot14_to_f32` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:296` | - |  |
| `impl SkiaScenePainter<'_>::get_or_cache_typeface` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:363` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::reset` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:424` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::push_layer` | 8 | `crates/vector-skia-render-adapter/src/scene.rs:428` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-skia-render-adapter/src/scene.rs:462` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::pop_layer` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:475` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::stroke` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:480` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::fill` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:496` | - |  |
| `impl PaintScene for SkiaScenePainter<'_>::draw_glyphs` | 13 | `crates/vector-skia-render-adapter/src/scene.rs:512` | 2d |  |
| `impl PaintScene for SkiaScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-skia-render-adapter/src/scene.rs:562` | 2d |  |
| `lerp_f32` | 3 | `crates/vector-skia-render-adapter/src/scene.rs:599` | - |  |
| `convert_filter` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:603` | - |  |
| `resolve_input` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:607` | - |  |
| `convert_filter_effect` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:614` | - |  |
| `map_transfer_function` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:645` | - |  |
| `to_sk_color_channel` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:906` | - | Convert an anyrender displacement-map [`ColorChannel`] to the corresponding Skia channel. |
| `distant_light_direction` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:918` | - | Compute the direction vector towards a distant light source from its azimuth and elevation. |
| `shader_from_image_brush` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:952` | 3d |  |
| `shader_from_gradient` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:1002` | 3d |  |
| `rad_to_deg` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1006` | - |  |
| `path_fill_type_from_fill` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1144` | - |  |
| `color4f_from_alpha_color` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1151` | - |  |
| `color4f_from_dynamic_color` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1160` | - |  |
| `gradient_shader_cs_from_cs_tag` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1170` | 3d |  |
| `gradient_shader_hue_method_from_hue_direction` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1190` | 3d |  |
| `tile_mode_from_extend` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1202` | - |  |
| `blend_mode_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1210` | - |  |
| `rect_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1262` | 2d |  |
| `rrect_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1271` | - |  |
| `m44_from_affine` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1286` | - |  |
| `matrix_from_affine` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1315` | numerics |  |
| `pt_from` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1337` | - |  |
| `path_from_shape` | 1 | `crates/vector-skia-render-adapter/src/scene.rs:1341` | - |  |
| `append_path_el_to_sk_path` | 2 | `crates/vector-skia-render-adapter/src/scene.rs:1357` | - |  |
| `impl VulkanBackend::new` | 4 | `crates/vector-skia-render-adapter/src/vulkan.rs:79` | - |  |
| `impl VulkanBackend::recreate_swapchain` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:195` | - |  |
| `impl VulkanBackend::prepare_intermediate_surface` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:240` | - | Prepares a Skia-allocated intermediate surface. |
| `impl VulkanBackend::prepare_swapchain_surface` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:281` | - | Prepares a Skia surface that wraps the raw Vulkan swapchain image. |
| `impl VulkanBackend::flush_post_multiplied` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:322` | 3d | Performs the post-processing blit when compositing in POST_MULTIPLIED alpha mode. |
| `impl Drop for VulkanBackend::drop` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:383` | - |  |
| `impl SkiaBackend for VulkanBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:411` | - |  |
| `impl SkiaBackend for VulkanBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:416` | - |  |
| `impl SkiaBackend for VulkanBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:447` | - |  |
| `create_instance` | 2 | `crates/vector-skia-render-adapter/src/vulkan.rs:524` | - |  |
| `pick_physical_device` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:545` | - |  |
| `create_logical_device` | 3 | `crates/vector-skia-render-adapter/src/vulkan.rs:596` | - |  |
| `create_swapchain` | 9 | `crates/vector-skia-render-adapter/src/vulkan.rs:626` | - |  |
| `create_gr_context` | 6 | `crates/vector-skia-render-adapter/src/vulkan.rs:759` | - |  |
| `create_sync_objects` | 1 | `crates/vector-skia-render-adapter/src/vulkan.rs:803` | - |  |
| `trait SkiaBackend::set_size` | 3 | `crates/vector-skia-render-adapter/src/window_renderer.rs:10` | - |  |
| `trait SkiaBackend::prepare` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:12` | - |  |
| `trait SkiaBackend::flush` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:14` | - |  |
| `impl Default for SkiaRendererOptions::default` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:38` | - |  |
| `impl SkiaRendererOptions::new` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:44` | - |  |
| `impl SkiaRendererOptions::base_color` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:51` | - |  |
| `impl SkiaRendererOptions::composite_alpha_mode` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:55` | - |  |
| `impl From<anyrender::RendererConfig> for SkiaRendererOptions::from` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:67` | - |  |
| `impl Default for SkiaWindowRenderer::default` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:86` | - |  |
| `impl SkiaWindowRenderer::new` | 0 | `crates/vector-skia-render-adapter/src/window_renderer.rs:92` | - |  |
| `impl SkiaWindowRenderer::with_options` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:98` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::resume` | 5 | `crates/vector-skia-render-adapter/src/window_renderer.rs:115` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::complete_resume` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:148` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::suspend` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:152` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::is_active` | 1 | `crates/vector-skia-render-adapter/src/window_renderer.rs:156` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::set_size` | 3 | `crates/vector-skia-render-adapter/src/window_renderer.rs:160` | - |  |
| `impl WindowRenderer for SkiaWindowRenderer::render` | 2 | `crates/vector-skia-render-adapter/src/window_renderer.rs:166` | - |  |

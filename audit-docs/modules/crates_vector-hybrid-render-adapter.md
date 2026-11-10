# `crates/vector-hybrid-render-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_A [`vello_gpu`] backend for the [`anyrender`] 2D drawing abstraction_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `convert_filter` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:6` | - |  |
| `convert_filter_effect` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:20` | - |  |
| `convert_edge_mode` | 1 | `crates/vector-hybrid-render-adapter/src/filters.rs:53` | graph |  |
| `anyrender_paint_to_vello_hybrid_paint` | 3 | `crates/vector-hybrid-render-adapter/src/scene.rs:19` | - |  |
| `impl ImageManager<'a>::new` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:75` | - |  |
| `impl ImageManager<'a>::upload_image` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:93` | - |  |
| `impl VelloHybridScenePainter<'_>::new` | 4 | `crates/vector-hybrid-render-adapter/src/scene.rs:137` | - |  |
| `impl VelloHybridScenePainter<'_>::convert_paint` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:152` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::try_register_custom_resource` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:158` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::unregister_resource` | 2 | `crates/vector-hybrid-render-adapter/src/scene.rs:185` | - |  |
| `impl RenderContext for VelloHybridScenePainter<'_>::renderer_specific_context` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:189` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::reset` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:195` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::push_layer` | 8 | `crates/vector-hybrid-render-adapter/src/scene.rs:199` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-hybrid-render-adapter/src/scene.rs:219` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::pop_layer` | 1 | `crates/vector-hybrid-render-adapter/src/scene.rs:227` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::stroke` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:239` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::fill` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:256` | - |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::draw_glyphs` | 12 | `crates/vector-hybrid-render-adapter/src/scene.rs:273` | 2d |  |
| `impl PaintScene for VelloHybridScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-hybrid-render-adapter/src/scene.rs:327` | 2d |  |
| `impl WebGlImageManager<'a>::new` | 3 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:24` | - |  |
| `impl WebGlImageManager<'a>::upload_image` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:36` | - |  |
| `impl WebGlScenePainter<'s>::new` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:68` | - |  |
| `impl WebGlScenePainter<'_>::convert_paint` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:78` | - |  |
| `impl WebGlScenePainter<'_>::convert_image_paint` | 2 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:90` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::reset` | 1 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:107` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::push_layer` | 8 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:111` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::push_clip_layer` | 4 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:131` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::pop_layer` | 1 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:139` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::stroke` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:151` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::fill` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:168` | - |  |
| `impl PaintScene for WebGlScenePainter<'_>::draw_glyphs` | 12 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:185` | 2d |  |
| `impl PaintScene for WebGlScenePainter<'_>::draw_box_shadow` | 6 | `crates/vector-hybrid-render-adapter/src/webgl_scene.rs:239` | 2d |  |
| `spawn_init` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:28` | - | Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask |
| `spawn_init` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:33` | - |  |
| `impl Default for VelloHybridRendererOptions::default` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:77` | - |  |
| `impl VelloHybridRendererOptions::new` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:90` | - |  |
| `impl VelloHybridRendererOptions::features` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:95` | - |  |
| `impl VelloHybridRendererOptions::limits` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:102` | - |  |
| `impl VelloHybridRendererOptions::render_settings` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:109` | - |  |
| `impl VelloHybridRendererOptions::base_color` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:116` | - |  |
| `impl VelloHybridRendererOptions::composite_alpha_mode` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:120` | - |  |
| `impl VelloHybridRendererOptions::desired_maximum_frame_latency` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:130` | - |  |
| `impl From<anyrender::RendererConfig> for VelloHybridRendererOptions::from` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:139` | - |  |
| `impl VelloHybridWindowRenderer::new` | 0 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:161` | - |  |
| `impl VelloHybridWindowRenderer::with_options` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:165` | - |  |
| `impl VelloHybridWindowRenderer::current_device_handle` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:179` | - |  |
| `build_wgpu_context` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:187` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::renderer_specific_context` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:200` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::try_register_custom_resource` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:210` | - |  |
| `impl RenderContext for VelloHybridWindowRenderer::unregister_resource` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:241` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::is_active` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:255` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::is_pending` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:259` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::resume` | 5 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:263` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::complete_resume` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:430` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::suspend` | 1 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:446` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::set_size` | 3 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:450` | - |  |
| `impl WindowRenderer for VelloHybridWindowRenderer::render` | 2 | `crates/vector-hybrid-render-adapter/src/window_renderer.rs:470` | - |  |

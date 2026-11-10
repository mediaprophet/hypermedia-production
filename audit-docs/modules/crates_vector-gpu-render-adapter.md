# `crates/vector-gpu-render-adapter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_A [`vello`] backend for the [`anyrender`] 2D drawing abstraction_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl VelloImageRenderer::with_pipeline_cache` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:21` | - | Like [`ImageRenderer::new`], but reusing compiled pipelines from a cache |
| `impl ImageRenderer for VelloImageRenderer::new` | 2 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:66` | - |  |
| `impl ImageRenderer for VelloImageRenderer::resize` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:70` | - |  |
| `impl ImageRenderer for VelloImageRenderer::reset` | 1 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:74` | - |  |
| `impl ImageRenderer for VelloImageRenderer::render_to_vec` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:78` | - |  |
| `impl ImageRenderer for VelloImageRenderer::render` | 3 | `crates/vector-gpu-render-adapter/src/image_renderer.rs:88` | - |  |
| `impl RenderContext for VelloScenePainter<'_, '_>::try_register_custom_resource` | 2 | `crates/vector-gpu-render-adapter/src/scene.rs:19` | - |  |
| `impl RenderContext for VelloScenePainter<'_, '_>::unregister_resource` | 2 | `crates/vector-gpu-render-adapter/src/scene.rs:38` | - |  |
| `impl RenderContext for VelloScenePainter<'_, '_>::renderer_specific_context` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:47` | - |  |
| `impl VelloScenePainter<'_, '_>::new` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:54` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::reset` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:65` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::push_layer` | 8 | `crates/vector-gpu-render-adapter/src/scene.rs:69` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::push_clip_layer` | 4 | `crates/vector-gpu-render-adapter/src/scene.rs:82` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::pop_layer` | 1 | `crates/vector-gpu-render-adapter/src/scene.rs:86` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::stroke` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:90` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::fill` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:104` | - |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::draw_glyphs` | 12 | `crates/vector-gpu-render-adapter/src/scene.rs:139` | 2d |  |
| `impl PaintScene for VelloScenePainter<'_, '_>::draw_box_shadow` | 6 | `crates/vector-gpu-render-adapter/src/scene.rs:175` | 2d |  |
| `spawn_init` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:30` | - | Drive the wgpu init future. On wasm32 we spawn it onto the JS microtask |
| `spawn_init` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:35` | - |  |
| `impl Default for VelloRendererOptions::default` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:80` | - |  |
| `impl VelloRendererOptions::new` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:86` | - |  |
| `impl VelloRendererOptions::features` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:98` | - |  |
| `impl VelloRendererOptions::limits` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:105` | - |  |
| `impl VelloRendererOptions::base_color` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:112` | - |  |
| `impl VelloRendererOptions::antialiasing_method` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:116` | - |  |
| `impl VelloRendererOptions::composite_alpha_mode` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:123` | - |  |
| `impl VelloRendererOptions::pipeline_cache` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:133` | - |  |
| `impl VelloRendererOptions::desired_maximum_frame_latency` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:140` | - |  |
| `impl From<anyrender::RendererConfig> for VelloRendererOptions::from` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:149` | - |  |
| `impl VelloWindowRenderer::new` | 0 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:174` | - |  |
| `impl VelloWindowRenderer::with_options` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:178` | - |  |
| `impl VelloWindowRenderer::current_device_handle` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:190` | - |  |
| `impl RenderContext for VelloWindowRenderer::try_register_custom_resource` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:199` | - |  |
| `impl RenderContext for VelloWindowRenderer::unregister_resource` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:217` | - |  |
| `impl RenderContext for VelloWindowRenderer::renderer_specific_context` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:227` | - |  |
| `build_wgpu_context` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:238` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::is_active` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:250` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::is_pending` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:254` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::resume` | 5 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:258` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::complete_resume` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:406` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::suspend` | 1 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:422` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::set_size` | 3 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:432` | - |  |
| `impl WindowRenderer for VelloWindowRenderer::render` | 2 | `crates/vector-gpu-render-adapter/src/window_renderer.rs:438` | - |  |

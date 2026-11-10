# `crates/vector-compute-renderer`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Vello is a compute-centric 2D graphics rendering engine written in Rust,
using [`wgpu`].
It efficiently draws large 2D scenes with interactive or near-interactive
performance.

![image](https://github.com/linebender/vello/assets/8573618/cc2b742e-2135-4b70-8051-c49aeddb5d19)


## Motivation

Vello is meant to fill the same place in the graphics stack as other vector graphics renderers like [Skia](https://skia.org/), [Cairo](https://www.cairographics.org/), and its predecessor project [Piet](https://www.cairographics.org/).
On a basic level, that means it provides tools to render shapes, images, gradients, texts, etc, using a PostScript-inspired API, the same that powers SVG files and [the browser `<canvas>` element](https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D).

Vello's selling point is that it gets better performance than other renderers by better leveraging the GPU.
In traditional PostScript-style renderers, some steps of the render process like sorting and clipping either need to be handled in the CPU or done through the use of intermediary textures.
Vello avoids this by using prefix-scan algorithms to parallelize work that usually needs to happen in sequence, so that work can be offloaded to the GPU with minimal use of temporary buffers.

This means that Vello needs a GPU with support for compute shaders to run.


## Getting started

Vello is meant to be integrated deep in UI render stacks.
While drawing in a Vello [`Scene`] is easy, actually rendering that scene to a surface setting up a wgpu context, which is a non-trivial task.

To use Vello as the renderer for your PDF reader / GUI toolkit / etc, your code will have to look roughly like this:

```ignore
// Initialize wgpu and get handles
let (width, height) = ...;
let device: wgpu::Device = ...;
let queue: wgpu::Queue = ...;
let mut renderer = Renderer::new(
&device,
RendererOptions {
use_cpu: false,
antialiasing_support: vello::AaSupport::all(),
num_init_threads: NonZeroUsize::new(1),
},
).expect("Failed to create renderer");

// Create scene and draw stuff in it
let mut scene = vello::Scene::new();
scene.fill(
vello::peniko::Fill::NonZero,
vello::Affine::IDENTITY,
vello::Color::from_rgb8(242, 140, 168),
None,
&vello::Circle::new((420.0, 200.0), 120.0),
);

// Draw more stuff
scene.push_layer(...);
scene.fill(...);
scene.stroke(...);
scene.pop_layer(...);

let texture = device.create_texture(&...);
// Render to a wgpu Texture
renderer
.render_to_texture(
&device,
&queue,
&scene,
&texture,
&vello::RenderParams {
base_color: palette::css::BLACK, // Background color
width,
height,
antialiasing_method: AaConfig::Msaa16,
},
)
.expect("Failed to render to a texture");
// Do things with surface texture, such as blitting it to the Surface using
// wgpu::util::TextureBlitter.
```

See the [`research/examples/`](https://github.com/linebender/vello/tree/main/research/examples) folder to see how that code integrates with frameworks like winit.
Utilities which can be used to create an alternative Vello renderer to [`Renderer`][crate::Renderer].

These APIs have not been carefully designed, and might not be powerful enough for this use case._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Debug for DebugLayers::fmt` | 2 | `crates/vector-compute-renderer/src/debug.rs:22` | - |  |
| `impl DebugLayers::from_bits` | 1 | `crates/vector-compute-renderer/src/debug.rs:72` | - | Construct a `DebugLayers` from the raw bits. |
| `impl DebugLayers::bits` | 1 | `crates/vector-compute-renderer/src/debug.rs:77` | - | Get the raw representation of this value. |
| `impl DebugLayers::none` | 0 | `crates/vector-compute-renderer/src/debug.rs:82` | - | A `DebugLayers` with no layers enabled. |
| `impl DebugLayers::all` | 0 | `crates/vector-compute-renderer/src/debug.rs:87` | - | A `DebugLayers` with all layers enabled. |
| `impl DebugLayers::is_empty` | 1 | `crates/vector-compute-renderer/src/debug.rs:98` | - | True if this `DebugLayers` has no layers enabled. |
| `impl DebugLayers::contains` | 2 | `crates/vector-compute-renderer/src/debug.rs:103` | - | Determine whether `self` is a superset of `mask`. |
| `impl DebugLayers::toggle` | 2 | `crates/vector-compute-renderer/src/debug.rs:108` | - | Toggle the value of the layers specified in mask. |
| `impl std::ops::BitOr for DebugLayers::bitor` | 2 | `crates/vector-compute-renderer/src/debug.rs:117` | - |  |
| `impl DebugRenderer::new` | 3 | `crates/vector-compute-renderer/src/debug/renderer.rs:29` | - |  |
| `impl DebugRenderer::render` | 8 | `crates/vector-compute-renderer/src/debug/renderer.rs:210` | - |  |
| `impl LinepointsUniforms::new` | 2 | `crates/vector-compute-renderer/src/debug/renderer.rs:355` | - |  |
| `impl LineEndpoint::new` | 2 | `crates/vector-compute-renderer/src/debug/validate.rs:23` | - |  |
| `impl fmt::Debug for LineEndpoint::fmt` | 2 | `crates/vector-compute-renderer/src/debug/validate.rs:38` | - |  |
| `validate_line_soup` | 1 | `crates/vector-compute-renderer/src/debug/validate.rs:47` | - |  |
| `impl AaSupport::all` | 0 | `crates/vector-compute-renderer/src/lib.rs:218` | 3d | Support every anti-aliasing method. |
| `impl AaSupport::area_only` | 0 | `crates/vector-compute-renderer/src/lib.rs:229` | - | Support only [`AaConfig::Area`]. |
| `impl FromIterator<AaConfig> for AaSupport::from_iter` | 1 | `crates/vector-compute-renderer/src/lib.rs:239` | - |  |
| `impl Default for RendererOptions::default` | 0 | `crates/vector-compute-renderer/src/lib.rs:411` | - |  |
| `impl Renderer::new` | 2 | `crates/vector-compute-renderer/src/lib.rs:434` | - | Creates a new renderer for the specified device. |
| `impl Renderer::render_to_texture` | 6 | `crates/vector-compute-renderer/src/lib.rs:476` | - | Renders a scene to the target texture. |
| `impl Renderer::override_image` | 3 | `crates/vector-compute-renderer/src/lib.rs:538` | - | Overwrite `image` with `texture`. |
| `impl Renderer::mark_override_image_dirty` | 2 | `crates/vector-compute-renderer/src/lib.rs:555` | - | Marks `image` as dirty in the atlas cache, so its override texture will be recopied on |
| `impl Renderer::register_texture` | 2 | `crates/vector-compute-renderer/src/lib.rs:574` | economics | Register a [`wgpu::Texture`] with Vello, to allow drawing GPU-resident data. |
| `impl Renderer::unregister_texture` | 2 | `crates/vector-compute-renderer/src/lib.rs:603` | - | Unregister a [`wgpu::Texture`] that was registered with [`register_texture`](Self::register_texture). |
| `impl Renderer::reload_shaders` | 2 | `crates/vector-compute-renderer/src/lib.rs:610` | - | Reload the shaders. This should only be used during `vello` development |
| `impl Renderer::render_to_texture_async` | 7 | `crates/vector-compute-renderer/src/lib.rs:644` | - |  |
| `impl Renderer::render_to_texture_async_internal` | 6 | `crates/vector-compute-renderer/src/lib.rs:719` | - |  |
| `impl DebugDownloads<'a>::map` | 3 | `crates/vector-compute-renderer/src/lib.rs:794` | - |  |
| `impl ResourceId::next` | 0 | `crates/vector-compute-renderer/src/recording.rs:16` | - |  |
| `impl Recording::push` | 2 | `crates/vector-compute-renderer/src/recording.rs:118` | - | Appends a [`Command`] to the back of the [`Recording`]. |
| `impl Recording::upload` | 3 | `crates/vector-compute-renderer/src/recording.rs:124` | - | Commands to upload the given data to a new buffer with the given name. |
| `impl Recording::upload_uniform` | 3 | `crates/vector-compute-renderer/src/recording.rs:133` | - | Commands to upload the given data to a new buffer as a uniform with the given name. |
| `impl Recording::upload_image` | 5 | `crates/vector-compute-renderer/src/recording.rs:142` | - | Commands to upload the given data to a new image with the given dimensions and format. |
| `impl Recording::write_image` | 5 | `crates/vector-compute-renderer/src/recording.rs:155` | - |  |
| `impl Recording::dispatch` | 4 | `crates/vector-compute-renderer/src/recording.rs:159` | 3d |  |
| `impl Recording::dispatch_indirect` | 5 | `crates/vector-compute-renderer/src/recording.rs:173` | 3d | Do an indirect dispatch. |
| `impl Recording::draw` | 2 | `crates/vector-compute-renderer/src/recording.rs:189` | - | Issue a draw call |
| `impl Recording::download` | 2 | `crates/vector-compute-renderer/src/recording.rs:197` | - | Prepare a buffer for downloading. |
| `impl Recording::clear_all` | 2 | `crates/vector-compute-renderer/src/recording.rs:202` | - | Commands to clear the whole buffer. |
| `impl Recording::free_buffer` | 2 | `crates/vector-compute-renderer/src/recording.rs:207` | - | Commands to free the given buffer. |
| `impl Recording::free_image` | 2 | `crates/vector-compute-renderer/src/recording.rs:212` | - | Commands to free the given image. |
| `impl Recording::free_resource` | 2 | `crates/vector-compute-renderer/src/recording.rs:217` | - | Commands to free the given resource. |
| `impl Recording::into_commands` | 1 | `crates/vector-compute-renderer/src/recording.rs:230` | - | Returns a [`Vec`] containing all the [`Command`]s in order. |
| `impl BufferProxy::new` | 2 | `crates/vector-compute-renderer/src/recording.rs:236` | - |  |
| `impl ImageFormat::to_wgpu` | 1 | `crates/vector-compute-renderer/src/recording.rs:245` | - |  |
| `impl ImageFormat::from_wgpu` | 1 | `crates/vector-compute-renderer/src/recording.rs:253` | - |  |
| `impl ImageProxy::new` | 3 | `crates/vector-compute-renderer/src/recording.rs:263` | - |  |
| `impl ResourceProxy::new_buf` | 2 | `crates/vector-compute-renderer/src/recording.rs:275` | - |  |
| `impl ResourceProxy::new_image` | 3 | `crates/vector-compute-renderer/src/recording.rs:279` | - |  |
| `impl ResourceProxy::as_buf` | 1 | `crates/vector-compute-renderer/src/recording.rs:283` | - |  |
| `impl ResourceProxy::as_image` | 1 | `crates/vector-compute-renderer/src/recording.rs:290` | - |  |
| `impl From<BufferProxy> for ResourceProxy::from` | 1 | `crates/vector-compute-renderer/src/recording.rs:299` | - |  |
| `impl From<ImageProxy> for ResourceProxy::from` | 1 | `crates/vector-compute-renderer/src/recording.rs:305` | - |  |
| `impl Drop for Render::drop` | 1 | `crates/vector-compute-renderer/src/render.rs:34` | - |  |
| `impl CapturedBuffers::release_buffers` | 2 | `crates/vector-compute-renderer/src/render.rs:77` | - |  |
| `render_full` | 5 | `crates/vector-compute-renderer/src/render.rs:84` | - |  |
| `render_encoding_full` | 5 | `crates/vector-compute-renderer/src/render.rs:99` | - | Create a single recording with both coarse and fine render stages. |
| `impl Default for Render::default` | 0 | `crates/vector-compute-renderer/src/render.rs:115` | - |  |
| `impl Render::new` | 0 | `crates/vector-compute-renderer/src/render.rs:121` | - |  |
| `impl Render::render_encoding_coarse` | 7 | `crates/vector-compute-renderer/src/render.rs:135` | - | Prepare a recording for the coarse rasterization phase. |
| `impl Render::record_fine` | 3 | `crates/vector-compute-renderer/src/render.rs:560` | - | Run fine rasterization assuming the coarse phase succeeded. |
| `impl Render::out_image` | 1 | `crates/vector-compute-renderer/src/render.rs:635` | - | Get the output image. |
| `impl Render::bump_buf` | 1 | `crates/vector-compute-renderer/src/render.rs:639` | - |  |
| `impl Render::take_captured_buffers` | 1 | `crates/vector-compute-renderer/src/render.rs:650` | - |  |
| `impl Scene::new` | 0 | `crates/vector-compute-renderer/src/scene.rs:54` | - | Creates a new scene. |
| `impl Scene::reset` | 1 | `crates/vector-compute-renderer/src/scene.rs:59` | - | Removes all content from the scene. |
| `impl Scene::bump_estimate` | 2 | `crates/vector-compute-renderer/src/scene.rs:68` | - | Tally up the bump allocator estimate for the current state of the encoding, |
| `impl Scene::encoding` | 1 | `crates/vector-compute-renderer/src/scene.rs:74` | - | Returns the underlying raw encoding. |
| `impl Scene::encoding_mut` | 1 | `crates/vector-compute-renderer/src/scene.rs:81` | - | Returns a mutable reference to the underlying raw encoding. |
| `impl Scene::push_layer` | 6 | `crates/vector-compute-renderer/src/scene.rs:101` | - | Pushes a new layer clipped by the specified shape and composed with |
| `impl Scene::push_luminance_mask_layer` | 5 | `crates/vector-compute-renderer/src/scene.rs:146` | - | Pushes a new layer clipped by the specified shape and treated like a luminance |
| `impl Scene::push_clip_layer` | 4 | `crates/vector-compute-renderer/src/scene.rs:180` | - | Pushes a new layer clipped by the specified `clip` shape. |
| `impl Scene::push_layer_inner` | 5 | `crates/vector-compute-renderer/src/scene.rs:190` | - | Helper for logic shared between [`Self::push_layer`] and [`Self::push_luminance_mask_layer`] |
| `impl Scene::pop_layer` | 1 | `crates/vector-compute-renderer/src/scene.rs:239` | - | Pops the current layer. |
| `impl Scene::draw_blurred_rounded_rect` | 6 | `crates/vector-compute-renderer/src/scene.rs:244` | 2d | Draw a rounded rectangle blurred with a gaussian filter. |
| `impl Scene::draw_blurred_rounded_rect_in` | 7 | `crates/vector-compute-renderer/src/scene.rs:270` | 2d | Draw a rounded rectangle blurred with a gaussian filter in `shape`. |
| `impl Scene::fill` | 6 | `crates/vector-compute-renderer/src/scene.rs:300` | - | Fills a shape using the specified style and brush. |
| `impl Scene::stroke` | 6 | `crates/vector-compute-renderer/src/scene.rs:327` | - | Strokes a shape using the specified style and brush. |
| `impl Scene::stroke_gpu_inner` | 4 | `crates/vector-compute-renderer/src/scene.rs:382` | - | Encodes the stroke of a shape using the specified style. The stroke style must have |
| `impl Scene::draw_image` | 3 | `crates/vector-compute-renderer/src/scene.rs:423` | - | Draws an image at its natural size with the given transform. |
| `impl Scene::draw_glyphs` | 2 | `crates/vector-compute-renderer/src/scene.rs:435` | - | Returns a builder for encoding a glyph run. |
| `impl Scene::append` | 3 | `crates/vector-compute-renderer/src/scene.rs:444` | - | Appends a child scene. |
| `impl From<Encoding> for Scene::from` | 1 | `crates/vector-compute-renderer/src/scene.rs:453` | - |  |
| `impl DrawGlyphs<'a>::new` | 2 | `crates/vector-compute-renderer/src/scene.rs:477` | - | Creates a new builder for encoding a glyph run for the specified |
| `impl DrawGlyphs<'a>::transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:506` | numerics | Sets the global transform. This is applied to all glyphs after the offset |
| `impl DrawGlyphs<'a>::glyph_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:517` | physics | Sets the per-glyph transform. This is applied to all glyphs prior to |
| `impl DrawGlyphs<'a>::brush_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:530` | - | Sets the transform applied to the brush contents. |
| `impl DrawGlyphs<'a>::font_size` | 2 | `crates/vector-compute-renderer/src/scene.rs:539` | - | Sets the font size in pixels per em units. |
| `impl DrawGlyphs<'a>::font_embolden` | 2 | `crates/vector-compute-renderer/src/scene.rs:546` | - | Sets synthetic embolden settings. |
| `impl DrawGlyphs<'a>::hint` | 2 | `crates/vector-compute-renderer/src/scene.rs:555` | - | Sets whether to enable hinting. |
| `impl DrawGlyphs<'a>::normalized_coords` | 2 | `crates/vector-compute-renderer/src/scene.rs:562` | - | Sets the normalized design space coordinates for a variable font instance. |
| `impl DrawGlyphs<'a>::brush` | 2 | `crates/vector-compute-renderer/src/scene.rs:581` | - | Sets the brush. |
| `impl DrawGlyphs<'a>::brush_alpha` | 2 | `crates/vector-compute-renderer/src/scene.rs:590` | - | Sets an additional alpha multiplier for the brush. |
| `impl DrawGlyphs<'a>::draw` | 3 | `crates/vector-compute-renderer/src/scene.rs:604` | - | Encodes a fill or stroke for the given sequence of glyphs and consumes the builder. |
| `impl DrawGlyphs<'a>::draw_outline_glyphs` | 3 | `crates/vector-compute-renderer/src/scene.rs:623` | - |  |
| `impl DrawGlyphs<'a>::try_draw_colr` | 3 | `crates/vector-compute-renderer/src/scene.rs:648` | - |  |
| `bitmap_masks` | 1 | `crates/vector-compute-renderer/src/scene.rs:880` | - |  |
| `m` | 2 | `crates/vector-compute-renderer/src/scene.rs:881` | - |  |
| `byte` | 1 | `crates/vector-compute-renderer/src/scene.rs:884` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_transform` | 2 | `crates/vector-compute-renderer/src/scene.rs:949` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:955` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_clip_glyph` | 2 | `crates/vector-compute-renderer/src/scene.rs:959` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_clip_box` | 2 | `crates/vector-compute-renderer/src/scene.rs:977` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_clip` | 1 | `crates/vector-compute-renderer/src/scene.rs:992` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::fill` | 2 | `crates/vector-compute-renderer/src/scene.rs:1000` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::push_layer` | 2 | `crates/vector-compute-renderer/src/scene.rs:1011` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::pop_layer` | 1 | `crates/vector-compute-renderer/src/scene.rs:1038` | - |  |
| `impl ColorPainter for DrawColorGlyphs<'_>::fill_glyph` | 4 | `crates/vector-compute-renderer/src/scene.rs:1042` | - |  |
| `impl OutlinePen for BezPathOutline::move_to` | 3 | `crates/vector-compute-renderer/src/scene.rs:1076` | - |  |
| `impl OutlinePen for BezPathOutline::line_to` | 3 | `crates/vector-compute-renderer/src/scene.rs:1080` | - |  |
| `impl OutlinePen for BezPathOutline::quad_to` | 5 | `crates/vector-compute-renderer/src/scene.rs:1084` | - |  |
| `impl OutlinePen for BezPathOutline::curve_to` | 7 | `crates/vector-compute-renderer/src/scene.rs:1091` | - |  |
| `impl OutlinePen for BezPathOutline::close` | 1 | `crates/vector-compute-renderer/src/scene.rs:1099` | - |  |
| `impl DrawColorGlyphs<'_>::last_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:1105` | - |  |
| `conv_skrifa_transform` | 1 | `crates/vector-compute-renderer/src/scene.rs:1113` | - |  |
| `conv_brush` | 3 | `crates/vector-compute-renderer/src/scene.rs:1120` | - |  |
| `color_index` | 2 | `crates/vector-compute-renderer/src/scene.rs:1177` | - |  |
| `conv_point` | 1 | `crates/vector-compute-renderer/src/scene.rs:1194` | - |  |
| `conv_extend` | 1 | `crates/vector-compute-renderer/src/scene.rs:1198` | - |  |
| `impl ColorStopsSource for ColorStopsConverter<'_>::collect_stops` | 2 | `crates/vector-compute-renderer/src/scene.rs:1211` | - |  |
| `full_shaders` | 3 | `crates/vector-compute-renderer/src/shaders.rs:48` | - |  |
| `impl RenderContext::new` | 0 | `crates/vector-compute-renderer/src/util.rs:32` | - |  |
| `impl RenderContext::create_surface` | 5 | `crates/vector-compute-renderer/src/util.rs:52` | - | Creates a new surface for the specified window and dimensions. |
| `impl RenderContext::create_render_surface` | 5 | `crates/vector-compute-renderer/src/util.rs:69` | - | Creates a new render surface for the specified window and dimensions. |
| `impl RenderContext::resize_surface` | 4 | `crates/vector-compute-renderer/src/util.rs:119` | - | Resizes the surface to the new dimensions. |
| `impl RenderContext::set_present_mode` | 3 | `crates/vector-compute-renderer/src/util.rs:130` | - |  |
| `impl RenderContext::configure_surface` | 2 | `crates/vector-compute-renderer/src/util.rs:139` | - |  |
| `impl RenderContext::device` | 2 | `crates/vector-compute-renderer/src/util.rs:145` | - | Finds or creates a compatible device handle id. |
| `impl RenderContext::new_device` | 2 | `crates/vector-compute-renderer/src/util.rs:162` | - | Creates a compatible device handle id. |
| `create_targets` | 3 | `crates/vector-compute-renderer/src/util.rs:196` | 3d | Vello uses a compute shader to render to the provided texture, which means that it can't bind the surface |
| `impl DeviceHandle::adapter` | 1 | `crates/vector-compute-renderer/src/util.rs:217` | - | Returns the adapter associated with the device. |
| `impl std::fmt::Debug for RenderSurface<'_>::fmt` | 2 | `crates/vector-compute-renderer/src/util.rs:234` | - |  |
| `impl std::task::Wake for NullWake::wake` | 1 | `crates/vector-compute-renderer/src/util.rs:250` | - |  |
| `block_on_wgpu` | 2 | `crates/vector-compute-renderer/src/util.rs:257` | - | Block on a future, polling the device as needed. |
| `impl Shader::select` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:79` | - |  |
| `impl WgpuEngine::new` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:144` | - |  |
| `impl WgpuEngine::use_parallel_initialisation` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:154` | - | Enable creating any remaining shaders in parallel |
| `impl WgpuEngine::build_shaders_if_needed` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:163` | - | Initialise (in parallel) any shaders which are yet to be created |
| `impl WgpuEngine::add_compute_shader` | 6 | `crates/vector-compute-renderer/src/wgpu_engine.rs:248` | 3d | Add a shader. |
| `impl WgpuEngine::add_render_shader` | 10 | `crates/vector-compute-renderer/src/wgpu_engine.rs:317` | 3d |  |
| `impl WgpuEngine::run_recording` | 8 | `crates/vector-compute-renderer/src/wgpu_engine.rs:380` | - |  |
| `impl WgpuEngine::get_download` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:779` | - |  |
| `impl WgpuEngine::free_download` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:783` | - |  |
| `impl WgpuEngine::create_bind_group_layout_entries` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:787` | - |  |
| `impl WgpuEngine::create_compute_pipeline` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:839` | - |  |
| `impl BindMap::insert_buf` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:885` | - |  |
| `impl BindMap::get_gpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:896` | - | Get a buffer, only if it's on GPU. |
| `impl BindMap::get_cpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:906` | - | Get a CPU buffer. |
| `impl BindMap::materialize_cpu_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:913` | - |  |
| `impl BindMap::insert_image` | 4 | `crates/vector-compute-renderer/src/wgpu_engine.rs:924` | - |  |
| `impl BindMap::get_buf` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:928` | - |  |
| `impl BindMap::get_or_create_image` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:932` | - |  |
| `impl ResourcePool::get_buf` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:976` | - | Get a buffer from the pool or create one. |
| `impl ResourcePool::size_class` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1003` | - | Quantize a size up to the nearest size class. |
| `impl BindMapBuffer::upload_if_needed` | 5 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1020` | - |  |
| `impl TransientBindMap<'a>::new` | 1 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1041` | - | Create new transient bind map, seeded from external resources |
| `impl TransientBindMap<'a>::materialize_gpu_buf_for_indirect` | 6 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1057` | - |  |
| `impl TransientBindMap<'a>::materialize_external_image_for_render_pass` | 2 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1073` | - |  |
| `impl TransientBindMap<'a>::create_bind_group` | 8 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1081` | - |  |
| `impl TransientBindMap<'a>::create_cpu_resources` | 3 | `crates/vector-compute-renderer/src/wgpu_engine.rs:1213` | - |  |

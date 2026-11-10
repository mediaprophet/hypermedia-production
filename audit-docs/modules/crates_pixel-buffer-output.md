# `crates/pixel-buffer-output`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_A tiny library providing a GPU-powered pixel buffer.

[`Pixels`] represents a 2D pixel buffer with an explicit image resolution, making it ideal for
prototyping simple pixel-based games, animations, and emulators. The pixel buffer is rendered
entirely on the GPU, allowing developers to easily incorporate special effects with shaders and
a customizable pipeline.

The GPU interface is offered by [`wgpu`](https://crates.io/crates/wgpu), and is re-exported for
your convenience. Use a windowing framework or context manager of your choice;
[`winit`](https://crates.io/crates/winit) is a good place to start. Any windowing framework that
uses [`raw-window-handle`](https://crates.io/crates/raw-window-handle) will work.

# Environment variables

Pixels will default to selecting the most powerful GPU and most modern graphics API available on
the system, and these choices can be overridden with environment variables. These are the same
vars supported by the [`wgpu` examples](https://github.com/gfx-rs/wgpu/tree/v0.10/wgpu#usage).

* `WGPU_BACKEND`: Select the backend (aka graphics API).
* Supported values: `vulkan`, `metal`, `dx11`, `dx12`, `gl`, `webgpu`
* The default depends on capabilities of the host system, with `vulkan` being preferred on
Linux and Windows, and `metal` preferred on macOS.
* `WGPU_ADAPTER_NAME`: Select an adapter (aka GPU) with substring matching.
* E.g. `1080` will match `NVIDIA GeForce 1080ti`
* `WGPU_POWER_PREF`: Select an adapter (aka GPU) that meets the given power profile.
* Supported values: `low`, `high`
* The default is `low`. I.e. an integrated GPU will be preferred over a discrete GPU.

Note that `WGPU_ADAPTER_NAME` and `WGPU_POWER_PREF` are mutually exclusive and that
`WGPU_ADAPTER_NAME` takes precedence._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `vs_main` | 1 | `crates/pixel-buffer-output/shaders/scale.wgsl:14` | 3d, 2d |  |
| `fs_main` | 1 | `crates/pixel-buffer-output/shaders/scale.wgsl:29` | 2d |  |
| `vs_main` | 1 | `crates/pixel-buffer-output/shaders/scale_fill.wgsl:14` | 3d, 2d |  |
| `fs_main` | 1 | `crates/pixel-buffer-output/shaders/scale_fill.wgsl:30` | 2d |  |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::new` | 3 | `crates/pixel-buffer-output/src/builder.rs:50` | physics, 2d | Create a builder that can be finalized into a [`Pixels`] pixel buffer. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::request_adapter_options` | 2 | `crates/pixel-buffer-output/src/builder.rs:73` | - | Add options for requesting a [`wgpu::Adapter`]. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::device_descriptor` | 2 | `crates/pixel-buffer-output/src/builder.rs:82` | - | Add options for requesting a [`wgpu::Device`]. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::wgpu_backend` | 2 | `crates/pixel-buffer-output/src/builder.rs:90` | - | Set which backends wgpu will attempt to use. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::pixel_aspect_ratio` | 2 | `crates/pixel-buffer-output/src/builder.rs:110` | physics, 2d | Set the pixel aspect ratio to simulate non-square pixels. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::enable_vsync` | 2 | `crates/pixel-buffer-output/src/builder.rs:124` | - | Enable or disable Vsync. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::present_mode` | 2 | `crates/pixel-buffer-output/src/builder.rs:142` | - | Optimistically set the `wgpu` present mode. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:155` | 2d | Set the texture format. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::render_texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:170` | 3d | Set the render texture format. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::surface_texture_format` | 2 | `crates/pixel-buffer-output/src/builder.rs:187` | 2d | Set the surface texture format. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::blend_state` | 2 | `crates/pixel-buffer-output/src/builder.rs:211` | - | Set the blend state. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::alpha_mode` | 2 | `crates/pixel-buffer-output/src/builder.rs:231` | - | Sets the alpha mode. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::clear_color` | 2 | `crates/pixel-buffer-output/src/builder.rs:260` | - | Set the clear color. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build_impl` | 1 | `crates/pixel-buffer-output/src/builder.rs:273` | 2d | Create a pixel buffer from the options builder. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build` | 1 | `crates/pixel-buffer-output/src/builder.rs:410` | 2d | Create a pixel buffer from the options builder. |
| `impl PixelsBuilder<'req, 'dev, 'win, W>::build_async` | 1 | `crates/pixel-buffer-output/src/builder.rs:436` | 2d | Create a pixel buffer from the options builder without blocking the current thread. |
| `check_texture_size` | 3 | `crates/pixel-buffer-output/src/builder.rs:447` | - | Compare the given size to the limits defined by `device`. |
| `create_backing_texture` | 9 | `crates/pixel-buffer-output/src/builder.rs:464` | 3d |  |
| `texture_format_size` | 1 | `crates/pixel-buffer-output/src/builder.rs:537` | - |  |
| `impl SurfaceTexture<W>::new` | 3 | `crates/pixel-buffer-output/src/lib.rs:192` | - | Create a logical texture for a window surface. |
| `impl Pixels<'win>::new` | 3 | `crates/pixel-buffer-output/src/lib.rs:250` | 2d | Create a pixel buffer instance with default options. |
| `impl Pixels<'win>::new_async` | 3 | `crates/pixel-buffer-output/src/lib.rs:281` | 2d | Asynchronously create a pixel buffer instance with default options. |
| `impl Pixels<'win>::clear_color` | 2 | `crates/pixel-buffer-output/src/lib.rs:308` | - | Change the clear color. |
| `impl Pixels<'win>::set_scaling_mode` | 2 | `crates/pixel-buffer-output/src/lib.rs:325` | 2d | Set the scaling mode. |
| `impl Pixels<'win>::adapter` | 1 | `crates/pixel-buffer-output/src/lib.rs:343` | - | Returns a reference of the `wgpu` adapter used by the crate. |
| `impl Pixels<'win>::resize_buffer` | 3 | `crates/pixel-buffer-output/src/lib.rs:363` | 2d | Resize the pixel buffer and zero its contents. |
| `impl Pixels<'win>::resize_surface` | 3 | `crates/pixel-buffer-output/src/lib.rs:408` | 2d | Resize the surface upon which the pixel buffer texture is rendered. |
| `impl Pixels<'win>::enable_vsync` | 2 | `crates/pixel-buffer-output/src/lib.rs:445` | - | Enable or disable Vsync. |
| `impl Pixels<'win>::present_mode` | 1 | `crates/pixel-buffer-output/src/lib.rs:458` | - | Get the `wgpu` present mode. |
| `impl Pixels<'win>::set_present_mode` | 2 | `crates/pixel-buffer-output/src/lib.rs:466` | - | Set the `wgpu` present mode. |
| `impl Pixels<'win>::render` | 1 | `crates/pixel-buffer-output/src/lib.rs:498` | 2d | Draw this pixel buffer to the configured [`SurfaceTexture`]. |
| `impl Pixels<'win>::render_with` | 2 | `crates/pixel-buffer-output/src/lib.rs:547` | 2d | Draw this pixel buffer to the configured [`SurfaceTexture`] using a custom user-provided |
| `impl Pixels<'win>::reconfigure_surface` | 1 | `crates/pixel-buffer-output/src/lib.rs:619` | - | Reconfigure the surface. |
| `impl Pixels<'win>::frame_mut` | 1 | `crates/pixel-buffer-output/src/lib.rs:638` | 2d | Get a mutable byte slice for the pixel buffer. The buffer is _not_ cleared for you; it will |
| `impl Pixels<'win>::frame` | 1 | `crates/pixel-buffer-output/src/lib.rs:646` | 2d | Get an immutable byte slice for the pixel buffer. |
| `impl Pixels<'win>::window_pos_to_pixel` | 2 | `crates/pixel-buffer-output/src/lib.rs:677` | 2d | Calculate the pixel location from a physical location on the window, |
| `impl Pixels<'win>::clamp_pixel_pos` | 2 | `crates/pixel-buffer-output/src/lib.rs:730` | 2d | Clamp a pixel position to the pixel buffer texture size. |
| `impl Pixels<'win>::device` | 1 | `crates/pixel-buffer-output/src/lib.rs:740` | - | Provides access to the internal [`wgpu::Device`]. |
| `impl Pixels<'win>::queue` | 1 | `crates/pixel-buffer-output/src/lib.rs:745` | - | Provides access to the internal [`wgpu::Queue`]. |
| `impl Pixels<'win>::texture` | 1 | `crates/pixel-buffer-output/src/lib.rs:752` | 2d | Provides access to the internal source [`wgpu::Texture`]. |
| `impl Pixels<'win>::context` | 1 | `crates/pixel-buffer-output/src/lib.rs:757` | - | Provides access to the internal [`PixelsContext`]. |
| `impl Pixels<'win>::surface_texture_format` | 1 | `crates/pixel-buffer-output/src/lib.rs:765` | - | Get the surface texture format. |
| `impl Pixels<'win>::render_texture_format` | 1 | `crates/pixel-buffer-output/src/lib.rs:773` | - | Get the render texture format. |
| `impl ScalingRenderer::new` | 8 | `crates/pixel-buffer-output/src/renderers.rs:23` | - |  |
| `impl ScalingRenderer::render` | 3 | `crates/pixel-buffer-output/src/renderers.rs:251` | 2d | Draw the pixel buffer to the render target. |
| `impl ScalingRenderer::clip_rect` | 1 | `crates/pixel-buffer-output/src/renderers.rs:291` | 2d | Get the clipping rectangle for the scaling renderer. |
| `impl ScalingRenderer::resize` | 4 | `crates/pixel-buffer-output/src/renderers.rs:295` | - |  |
| `impl ScalingMatrix::new` | 3 | `crates/pixel-buffer-output/src/renderers.rs:317` | - |  |
| `impl ScalingMatrix::clip_rect` | 1 | `crates/pixel-buffer-output/src/renderers.rs:382` | 2d |  |

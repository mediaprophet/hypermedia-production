# `crates/gpu-device-context`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Simple helpers for managing wgpu state and surfaces._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl AlphaConvertBlitter::new` | 3 | `crates/gpu-device-context/src/alpha_blitter.rs:90` | - | Create a blitter that applies `conversion` and writes to a target |
| `impl AlphaConvertBlitter::copy` | 5 | `crates/gpu-device-context/src/alpha_blitter.rs:156` | - | Copy `source` into `target`, applying the alpha conversion. |
| `impl std::fmt::Debug for BufferRenderer::fmt` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:26` | - |  |
| `impl BufferRenderer::new` | 3 | `crates/gpu-device-context/src/buffer_renderer.rs:36` | - | Creates a new render surface for the specified window and dimensions. |
| `impl BufferRenderer::device` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:63` | - |  |
| `impl BufferRenderer::queue` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:67` | - |  |
| `impl BufferRenderer::size` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:71` | - |  |
| `impl BufferRenderer::resize` | 3 | `crates/gpu-device-context/src/buffer_renderer.rs:79` | - |  |
| `impl BufferRenderer::target_texture_view` | 1 | `crates/gpu-device-context/src/buffer_renderer.rs:117` | - |  |
| `impl BufferRenderer::copy_texture_to_vec` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:121` | - |  |
| `impl BufferRenderer::copy_texture_to_buffer` | 2 | `crates/gpu-device-context/src/buffer_renderer.rs:127` | - |  |
| `impl Display for WgpuContextError::fmt` | 2 | `crates/gpu-device-context/src/error.rs:30` | - |  |
| `impl From<wgpu::CreateSurfaceError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:58` | - |  |
| `impl From<RequestAdapterError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:64` | - |  |
| `impl From<RequestDeviceError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:70` | - |  |
| `impl From<PollError> for WgpuContextError::from` | 1 | `crates/gpu-device-context/src/error.rs:76` | - |  |
| `impl DeviceHandle::new_from_compatible_surface` | 4 | `crates/gpu-device-context/src/lib.rs:33` | - | Creates a `DeviceHandle` with `Device` that's compatible with the specified `Surface` |
| `impl DeviceHandle::create_surface` | 4 | `crates/gpu-device-context/src/lib.rs:79` | - | Creates a new surface for the specified window and dimensions. |
| `impl Default for WGPUContext::default` | 0 | `crates/gpu-device-context/src/lib.rs:110` | - |  |
| `impl WGPUContext::new` | 0 | `crates/gpu-device-context/src/lib.rs:116` | - |  |
| `impl WGPUContext::with_features_and_limits` | 2 | `crates/gpu-device-context/src/lib.rs:120` | - |  |
| `impl WGPUContext::extra_features` | 1 | `crates/gpu-device-context/src/lib.rs:141` | - |  |
| `impl WGPUContext::override_limits` | 1 | `crates/gpu-device-context/src/lib.rs:145` | - |  |
| `impl WGPUContext::create_surface` | 2 | `crates/gpu-device-context/src/lib.rs:150` | - | Creates a new surface for the specified window and dimensions. |
| `impl WGPUContext::create_surface_renderer` | 4 | `crates/gpu-device-context/src/lib.rs:158` | - | Creates a new surface for the specified window and dimensions. |
| `impl WGPUContext::create_buffer_renderer` | 2 | `crates/gpu-device-context/src/lib.rs:183` | - | Creates a new `BufferRenderer` for the specified dimensions. |
| `impl WGPUContext::find_or_create_device` | 2 | `crates/gpu-device-context/src/lib.rs:198` | - | Finds or creates a compatible device handle id. |
| `impl WGPUContext::find_compatible_device_handle` | 2 | `crates/gpu-device-context/src/lib.rs:209` | - | Finds or creates a compatible device handle id. |
| `impl WGPUContext::find_existing_device_idx` | 2 | `crates/gpu-device-context/src/lib.rs:218` | - | Finds  a compatible device handle id. |
| `impl WGPUContext::create_device_handle` | 2 | `crates/gpu-device-context/src/lib.rs:230` | - |  |
| `impl WGPUContext::create_device` | 2 | `crates/gpu-device-context/src/lib.rs:243` | - | Creates a compatible device handle id. |
| `impl IntermediateBlitter::copy` | 5 | `crates/gpu-device-context/src/surface_renderer.rs:41` | - |  |
| `impl std::fmt::Debug for SurfaceRenderer<'_>::fmt` | 2 | `crates/gpu-device-context/src/surface_renderer.rs:135` | - |  |
| `impl SurfaceRenderer<'s>::new` | 4 | `crates/gpu-device-context/src/surface_renderer.rs:145` | - | Creates a new render surface for the specified window and dimensions. |
| `impl SurfaceRenderer<'s>::device` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:208` | - |  |
| `impl SurfaceRenderer<'s>::queue` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:212` | - |  |
| `impl SurfaceRenderer<'s>::resize` | 3 | `crates/gpu-device-context/src/surface_renderer.rs:217` | - | Resizes the surface to the new dimensions. |
| `impl SurfaceRenderer<'s>::set_present_mode` | 2 | `crates/gpu-device-context/src/surface_renderer.rs:234` | - |  |
| `impl SurfaceRenderer<'s>::configure` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:239` | - |  |
| `impl SurfaceRenderer<'s>::clear_surface_texture` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:244` | - |  |
| `impl SurfaceRenderer<'s>::acquire` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:249` | - | Acquire the next surface frame. |
| `impl SurfaceRenderer<'s>::acquire_reconfiguring_if_stale` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:258` | - | Acquire a usable surface texture. A `Suboptimal` texture is still |
| `impl SurfaceRenderer<'s>::ensure_current_surface_texture` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:275` | - |  |
| `impl SurfaceRenderer<'s>::target_texture_view` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:291` | - | Get a target texture view to render to. |
| `impl SurfaceRenderer<'s>::maybe_blit_and_present` | 1 | `crates/gpu-device-context/src/surface_renderer.rs:306` | - | Present the texture to the surface. If there is an intermediate texture, this first blits |
| `impl SurfaceRenderer<'s>::blit_from_intermediate_texture_to_surface` | 3 | `crates/gpu-device-context/src/surface_renderer.rs:324` | - | Blit from the intermediate texture to the surface texture |
| `block_on_wgpu` | 2 | `crates/gpu-device-context/src/util.rs:8` | - | Block on a future, polling the device as needed. |
| `create_texture` | 5 | `crates/gpu-device-context/src/util.rs:30` | - | Create a WGPU Texture, returning a default TextureView |

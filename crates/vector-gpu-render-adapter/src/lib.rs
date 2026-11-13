//! A [`vello`] backend for the [`anyrender`] 2D drawing abstraction

#[cfg(not(target_arch = "wasm32"))]
mod image_renderer;
mod scene;
mod window_renderer;

pub use etalon_context::DeviceHandle;
#[cfg(not(target_arch = "wasm32"))]
pub use image_renderer::VelloImageRenderer;
pub use scene::VelloScenePainter;
pub use window_renderer::{VelloRendererOptions, VelloWindowRenderer};

pub use etalon;

#[doc(hidden)]
pub use etalon as wgpu;

use std::num::NonZeroUsize;

#[cfg(target_os = "macos")]
const DEFAULT_THREADS: Option<NonZeroUsize> = NonZeroUsize::new(1);
#[cfg(not(target_os = "macos"))]
const DEFAULT_THREADS: Option<NonZeroUsize> = None;

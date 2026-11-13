// Copyright 2025 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Provides renderer backends which handle GPU resource management and executes draw operations.
//!
//! ## Renderer Backends
//!
//! - `etalon` contains the default renderer backend, leveraging `etalon`.
//! - `webgl` contains a WebGL2 backend if the `webgl` feature is active.

pub(crate) mod common;
#[cfg(feature = "etalon")]
mod etalon;
#[cfg(feature = "webgl")]
mod webgl;

pub use common::{ClearSettings, Config, GpuStrip, RenderSize, TargetInit};

#[cfg(feature = "etalon")]
pub use etalon::{AtlasWriter, RenderTargetConfig, Renderer, TextureBindings};
#[cfg(feature = "webgl")]
pub use webgl::{
    AtlasTextureInfo, IncompatibleContextReason, WebGlAtlasWriter, WebGlContextOperation,
    WebGlDataTransferOperation, WebGlError, WebGlOperation, WebGlRenderer, WebGlRendererInit,
    WebGlRendererInitStatus, WebGlResourceKind, WebGlShaderInterfaceOperation,
    WebGlShaderProgramOperation, WebGlShaderStage, WebGlTextureBindings,
    WebGlTextureWithDimensions,
};
#[cfg(all(feature = "webgl", feature = "probe"))]
pub use webgl::{
    WebGlProbeOperation,
    probe::{WebGlPendingProbe, WebGlProbeError, WebGlProbeStatus},
};

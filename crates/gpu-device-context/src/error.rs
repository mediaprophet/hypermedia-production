//! Error type for Etalon Context

use etalon::{PollError, RequestAdapterError, RequestDeviceError};
use std::error::Error;
use std::fmt::Display;

/// Errors that can occur in EtalonContext.
#[derive(Debug)]
pub enum EtalonContextError {
    /// There is no available device with the features required by Vello.
    NoCompatibleDevice,
    /// Failed to create surface.
    /// See [`etalon::CreateSurfaceError`] for more information.
    EtalonCreateSurfaceError(etalon::CreateSurfaceError),
    /// Surface doesn't support the required texture formats.
    /// Make sure that you have a surface which provides one of
    /// `TextureFormat::Rgba8Unorm`
    /// or `TextureFormat::Bgra8Unorm` as texture formats.
    // TODO: Why does this restriction exist?
    UnsupportedSurfaceFormat,
    /// Etalon failed to request an adapter
    RequestAdapterError(RequestAdapterError),
    /// Etalon failed to request a device
    RequestDeviceError(RequestDeviceError),
    /// Etalon failed to poll a device
    PollError(PollError),
}

impl Display for EtalonContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCompatibleDevice => writeln!(f, "Couldn't find suitable device"),
            Self::EtalonCreateSurfaceError(inner) => {
                writeln!(f, "Couldn't create etalon surface")?;
                inner.fmt(f)
            }
            Self::UnsupportedSurfaceFormat => {
                writeln!(
                    f,
                    "Couldn't find `Rgba8Unorm` or `Bgra8Unorm` texture formats for surface"
                )
            }
            Self::RequestAdapterError(inner) => {
                writeln!(f, "Couldn't request an adapter: {:#}", inner)
            }
            Self::RequestDeviceError(inner) => {
                writeln!(f, "Couldn't request a device: {:#}", inner)
            }
            Self::PollError(inner) => {
                writeln!(f, "Couldn't poll a device: {:#}", inner)
            }
        }
    }
}

impl Error for EtalonContextError {}
impl From<etalon::CreateSurfaceError> for EtalonContextError {
    fn from(value: etalon::CreateSurfaceError) -> Self {
        Self::EtalonCreateSurfaceError(value)
    }
}

impl From<RequestAdapterError> for EtalonContextError {
    fn from(value: RequestAdapterError) -> Self {
        Self::RequestAdapterError(value)
    }
}

impl From<RequestDeviceError> for EtalonContextError {
    fn from(value: RequestDeviceError) -> Self {
        Self::RequestDeviceError(value)
    }
}

impl From<PollError> for EtalonContextError {
    fn from(value: PollError) -> Self {
        Self::PollError(value)
    }
}

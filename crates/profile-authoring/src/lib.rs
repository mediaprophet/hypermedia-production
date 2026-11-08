//! Bounded metric profile authoring. Source, units and project history remain host-owned.
#![forbid(unsafe_code)]

pub mod assembly;
pub mod frames;
pub mod geometry;
pub mod profile;

pub use assembly::{
    AssemblyLimits, AuthoringPart, CompiledAssembly, CompiledPart, Transform, compile_assembly,
};
pub use geometry::{GeometryLimits, profile_sweep, sphere, torus};
pub use profile::{Profile, ProfileDoc, ProfileLimits};

/// Existing source-compatible authoring refusal categories.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthoringError {
    Invalid(&'static str),
    Budget(&'static str),
    Kernel(String),
}

impl std::fmt::Display for AuthoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(m) | Self::Budget(m) => f.write_str(m),
            Self::Kernel(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for AuthoringError {}

pub type ProfileError = AuthoringError;
pub type Profile2D = Profile;
pub type ClosedPolygon = Profile;
pub type OpenChannel = Profile;

pub fn rounded_rect(width: f64, height: f64, radius: f64, segments: usize) -> Profile {
    Profile::RoundedRectangle {
        width,
        height,
        radius,
        segments,
    }
}

pub fn circle(radius: f64, segments: usize) -> Profile {
    Profile::Circle { radius, segments }
}

pub fn closed_polygon(points: Vec<[f64; 2]>) -> Profile {
    Profile::Polyline { points }
}

pub(crate) fn finite_positive(value: f64) -> bool {
    value.is_finite() && value > 0.0 && (value as f32).is_finite()
}

#[cfg(test)]
mod tests;

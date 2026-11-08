//! Pure-Rust ICC v2/v4 color management module.
//!
//! Provides ICC profile parsing & serialization, PCS XYZ/Lab D50 conversion,
//! 4 rendering intents, Black Point Compensation, tetrahedral CLUT interpolation,
//! built-in standard profiles, and out-of-gamut detection.

pub mod builtin;
pub mod clut;
pub mod curve;
pub mod gamut;
pub mod math;
pub mod pipeline;
pub mod profile;
pub mod synth;
pub mod transform;
pub mod write;

pub use builtin::{Builtin, adobe_rgb, display_p3, generic_cmyk, prophoto_rgb, rec2020, srgb};
pub use clut::Clut;
pub use curve::Curve;
pub use gamut::GamutCheck;
pub use profile::{CmsError, ColorSpace, Intent, Lut, Pcs, Profile, ProfileClass};
pub use transform::{Transform, TransformOptions};

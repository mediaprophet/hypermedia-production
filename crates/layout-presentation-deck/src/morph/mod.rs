//! Morph transition and interpolation module.

pub mod interpolate;
pub mod match_pairs;
pub mod transition;

pub use interpolate::{clamp, lerp, lerp_rgba, morph_xfrm};
pub use match_pairs::morph_pairs;
pub use transition::{build_morph_frame, MorphFrame, MorphPath, MorphText};

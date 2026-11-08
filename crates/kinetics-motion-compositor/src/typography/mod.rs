//! Kinetic typography: range and wiggly text selectors and 3D character animators.

pub mod animator;
pub mod selectors;
pub mod wiggly;

pub use animator::{CharTransform, TextAnimator, TextProperties, evaluate_text_animators};
pub use selectors::{BasedOn, Mode, RangeSelector, Shape, shape_value};
pub use wiggly::WigglySelector;

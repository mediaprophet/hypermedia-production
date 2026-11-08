//! Explicit dynamics: central-difference time integration, hourglass control, and damage erosion.

pub mod damage;
pub mod dynamics;
pub mod hourglass;

pub use damage::JohnsonCookDamage;
pub use dynamics::{ExplicitIntegrator, ExplicitState};
pub use hourglass::{flanagan_belytschko_hex8, GAMMA_HOURGLASS};

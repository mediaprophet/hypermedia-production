//! 3D vector geometry: bevel/extrude and surface of revolution.

pub mod bevel;
pub mod revolve;
pub mod visibility;

pub use bevel::{extrude, BevelStyle, Extrude};
pub use revolve::{revolve, Edge, Face, Revolve, V3};
pub use visibility::{visible_faces, VisibleFace};

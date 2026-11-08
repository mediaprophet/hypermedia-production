//! Color swatch and gradient exchange format parsers and writers:
//! Adobe Color Swatch (`.aco`), Adobe Swatch Exchange (`.ase`), GIMP Palette (`.gpl`),
//! and Photoshop Gradient (`.grd`).

pub mod aco;
pub mod ase;
pub mod gpl;
pub mod grd;

pub use aco::{AcoColor, AcoFile, AcoSwatch, PaletteError, parse_aco, write_aco};
pub use ase::{AseColor, AseEntry, AseFile, AseGroup, AseKind, parse_ase, write_ase};
pub use gpl::{GplColor, GplPalette, parse_gpl, write_gpl};
pub use grd::{GrdColor, GrdGradient, GrdStop, parse_grd, write_grd};

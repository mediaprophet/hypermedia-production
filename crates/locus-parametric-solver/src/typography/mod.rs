//! Single-stroke CAM typography and MTEXT layout.

#![forbid(unsafe_code)]

pub mod mtext;
pub mod stroke;

pub use mtext::{
    MTextLayout, MTextParams, MTextPiece, decode_controls, layout_mtext, plain_mtext,
};
pub use stroke::{
    CAP, GAP, StrokeLayout, char_advance, decode_stroke, glyph, layout_stroke_line,
};

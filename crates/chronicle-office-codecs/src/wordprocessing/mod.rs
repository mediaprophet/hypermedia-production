//! Word processing codecs (DOCX).

pub mod docx;

pub use docx::{parse_docx_package, to_docx_package};

//! SMPTE VC-3 Avid DNxHD / DNxHR broadcast intermediate frame decoder.

pub mod bits;
pub mod dct;
pub mod decode;
pub mod header;
pub mod spec_tables;
pub mod tables;

pub use dct::{fdct8x8, idct8x8};
pub use decode::{DecodeOptions, Frame, decode_frame, decode_frame_with, probe};
pub use header::{
    ChromaFormat, ColorVolume, FieldCode, FrameHeader, header_size_for, parse_frame_header,
    write_frame_header,
};
pub use tables::{CIDS, CidInfo, Kind, cid_info, ri_frame_size};

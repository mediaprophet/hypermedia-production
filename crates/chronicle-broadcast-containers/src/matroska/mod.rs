//! EBML / Matroska (.mkv) and WebM container demuxer and muxer.

pub mod demux;
pub mod ebml;
pub mod ids;
pub mod mux;

pub use demux::{MatroskaBlock, MatroskaDemuxer, MatroskaTrack, demux_matroska};
pub use mux::{LacingMode, MkvWriter, MuxOptions, TrackKind, TrackSpec};

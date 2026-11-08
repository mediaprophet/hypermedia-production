//! SMPTE 377M Material Exchange Format (MXF) KLV packet demuxer and muxer.

pub mod demux;
pub mod klv;
pub mod labels;
pub mod meta;
pub mod partition;
pub mod writer;

pub use demux::{MxfDemuxer, MxfEssencePacket, demux_mxf};
pub use klv::{KeyClass, Rational, Ul, ber_length, classify};
pub use meta::{Metadata, Primer, Set};
pub use partition::{OperationalPattern, Partition, PartitionKind};
pub use writer::{MxfTrackSpec, MxfWriter, Pattern, Umid};

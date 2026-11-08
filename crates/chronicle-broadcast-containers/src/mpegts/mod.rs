//! ISO/IEC 13818-1 188-byte MPEG-2 Transport Stream (TS) demuxer and packetizer.

pub mod pes;
pub mod psi;
pub mod ts;

pub use pes::{PesHeader, build_pes_packet, encode_timestamp, parse_pes_header, parse_timestamp};
pub use psi::{Pat, PatProgram, Pmt, PmtStream, build_pat, build_pmt, crc32_mpeg, parse_pat, parse_pmt};
pub use ts::{TS_PACKET_SIZE, TS_SYNC_BYTE, TsPacket, parse_ts_packet, write_ts_packet};

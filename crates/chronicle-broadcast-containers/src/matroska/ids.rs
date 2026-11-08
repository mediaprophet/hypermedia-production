//! EBML / Matroska element IDs (RFC 8794, RFC 9559). IDs keep their VINT marker bits.

#![allow(dead_code)]

// EBML header + global elements
pub const EBML: u32 = 0x1A45_DFA3;
pub const EBML_VERSION: u32 = 0x4286;
pub const EBML_READ_VERSION: u32 = 0x42F7;
pub const EBML_MAX_ID_LENGTH: u32 = 0x42F2;
pub const EBML_MAX_SIZE_LENGTH: u32 = 0x42F3;
pub const DOC_TYPE: u32 = 0x4282;
pub const DOC_TYPE_VERSION: u32 = 0x4287;
pub const DOC_TYPE_READ_VERSION: u32 = 0x4285;
pub const CRC32: u32 = 0xBF;
pub const VOID: u32 = 0xEC;

// Segment and level-1 elements
pub const SEGMENT: u32 = 0x1853_8067;
pub const SEEK_HEAD: u32 = 0x114D_9B74;
pub const INFO: u32 = 0x1549_A966;
pub const TRACKS: u32 = 0x1654_AE6B;
pub const CLUSTER: u32 = 0x1F43_B675;
pub const CUES: u32 = 0x1C53_BB6B;
pub const ATTACHMENTS: u32 = 0x1941_A469;
pub const CHAPTERS: u32 = 0x1043_A770;
pub const TAGS: u32 = 0x1254_C367;

pub fn is_top_level(id: u32) -> bool {
    matches!(id, EBML | SEGMENT | SEEK_HEAD | INFO | TRACKS | CLUSTER | CUES | ATTACHMENTS | CHAPTERS | TAGS)
}

// SeekHead
pub const SEEK: u32 = 0x4DBB;
pub const SEEK_ID: u32 = 0x53AB;
pub const SEEK_POSITION: u32 = 0x53AC;

// Info
pub const SEGMENT_UUID: u32 = 0x73A4;
pub const SEGMENT_FILENAME: u32 = 0x7384;
pub const TIMESTAMP_SCALE: u32 = 0x2A_D7B1;
pub const DURATION: u32 = 0x4489;
pub const DATE_UTC: u32 = 0x4461;
pub const TITLE: u32 = 0x7BA9;
pub const MUXING_APP: u32 = 0x4D80;
pub const WRITING_APP: u32 = 0x5741;

// Cluster
pub const TIMESTAMP: u32 = 0xE7;
pub const POSITION: u32 = 0xA7;
pub const PREV_SIZE: u32 = 0xAB;
pub const SIMPLE_BLOCK: u32 = 0xA3;
pub const BLOCK_GROUP: u32 = 0xA0;
pub const ENCRYPTED_BLOCK: u32 = 0xAF;
pub const SILENT_TRACKS: u32 = 0x5854;
pub const BLOCK: u32 = 0xA1;
pub const BLOCK_ADDITIONS: u32 = 0x75A1;
pub const BLOCK_MORE: u32 = 0xA6;
pub const BLOCK_ADD_ID: u32 = 0xEE;
pub const BLOCK_ADDITIONAL: u32 = 0xA5;
pub const BLOCK_DURATION: u32 = 0x9B;
pub const REFERENCE_PRIORITY: u32 = 0xFA;
pub const REFERENCE_BLOCK: u32 = 0xFB;
pub const CODEC_STATE: u32 = 0xA4;
pub const DISCARD_PADDING: u32 = 0x75A2;

// Tracks
pub const TRACK_ENTRY: u32 = 0xAE;
pub const TRACK_NUMBER: u32 = 0xD7;
pub const TRACK_UID: u32 = 0x73C5;
pub const TRACK_TYPE: u32 = 0x83;
pub const FLAG_ENABLED: u32 = 0xB9;
pub const FLAG_DEFAULT: u32 = 0x88;
pub const FLAG_FORCED: u32 = 0x55AA;
pub const FLAG_LACING: u32 = 0x9C;
pub const DEFAULT_DURATION: u32 = 0x23_E383;
pub const NAME: u32 = 0x536E;
pub const LANGUAGE: u32 = 0x22_B59C;
pub const CODEC_ID: u32 = 0x86;
pub const CODEC_PRIVATE: u32 = 0x63A2;
pub const CODEC_NAME: u32 = 0x25_8688;
pub const CODEC_DELAY: u32 = 0x56AA;
pub const SEEK_PRE_ROLL: u32 = 0x56BB;

// Video
pub const VIDEO: u32 = 0xE0;
pub const FLAG_INTERLACED: u32 = 0x9A;
pub const PIXEL_WIDTH: u32 = 0xB0;
pub const PIXEL_HEIGHT: u32 = 0xBA;
pub const DISPLAY_WIDTH: u32 = 0x54B0;
pub const DISPLAY_HEIGHT: u32 = 0x54BA;

// Audio
pub const AUDIO: u32 = 0xE1;
pub const SAMPLING_FREQUENCY: u32 = 0xB5;
pub const CHANNELS: u32 = 0x9F;
pub const BIT_DEPTH: u32 = 0x6264;

// Cues
pub const CUE_POINT: u32 = 0xBB;
pub const CUE_TIME: u32 = 0xB3;
pub const CUE_TRACK_POSITIONS: u32 = 0xB7;
pub const CUE_TRACK: u32 = 0xF7;
pub const CUE_CLUSTER_POSITION: u32 = 0xF1;
pub const CUE_RELATIVE_POSITION: u32 = 0xF0;
pub const CUE_DURATION: u32 = 0xB2;
pub const CUE_BLOCK_NUMBER: u32 = 0x5378;

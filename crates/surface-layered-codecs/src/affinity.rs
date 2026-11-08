//! # Serif Affinity Suite Layered Document & Container Parser
//!
//! Provides pure-Rust, memory-bounded, zero-unsafe container and descriptor inspection for:
//! - Serif Affinity Photo (`.afphoto`), Designer (`.afdesign`), and Publisher (`.afpub`) files
//! - Chunk-based container header (`\x00\xffKA`) and chained File Allocation Tables (FAT)
//! - Embedded document data descriptors (`doc.dat`, raster tiles, page tables)
//! - Embedded indexed preview extraction (`\xff\xff\xff\xffThmb` PNG inspection)
//!
//! Grounded in international human rights instruments (UDHR Articles 12, 19, 27; ICESCR Article 15)
//! and Informatics Fiduciary duties.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Container magic byte sequence (`\x00\xffKA`).
pub const AFFINITY_MAGIC: &[u8; 4] = b"\x00\xffKA";

/// Document kind signature (`nsrP` - 'Person' reversed).
pub const AFFINITY_DOC_KIND: &[u8; 4] = b"nsrP";

/// Supported container version range (Affinity 1.x through 3.x).
pub const SUPPORTED_VERSIONS: std::ops::RangeInclusive<u16> = 8..=12;

/// Maximum decompression budget per entry (256 MiB).
pub const MAX_ENTRY_BYTES: usize = 256 << 20;

/// Maximum thumbnail size (16 MiB).
pub const MAX_PREVIEW_BYTES: usize = 16 << 20;

/// Container header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffinityHeader {
    pub version: u16,
    pub flags: u16,
    pub fat_offset: u64,
    pub thumbnail_offset: u64,
    pub saved_timestamp: u32,
}

/// A File Allocation Table entry representing an embedded document asset or layer stream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffinityEntry {
    pub id: u32,
    pub name: String,
    pub offset: u64,
    pub size: u64,
    pub compressed_size: u64,
    pub crc: u32,
    pub compression_type: u8,
}

/// Extracted thumbnail / preview image metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffinityPreview {
    pub width: u32,
    pub height: u32,
    pub data_length: usize,
    /// Format: "image/png" if standard PNG thumbnail is detected.
    pub mime_type: String,
}

/// Parsed Affinity archive representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffinityArchive {
    pub header: AffinityHeader,
    pub entries: HashMap<String, AffinityEntry>,
    pub preview: Option<AffinityPreview>,
    pub document_type: String,
}

/// Reads and validates the 72-byte Affinity container header.
pub fn read_affinity_header(bytes: &[u8]) -> Result<AffinityHeader, String> {
    if bytes.len() < 72 {
        return Err("Affinity file too short (minimum 72 bytes for header)".to_string());
    }

    if &bytes[0..4] != AFFINITY_MAGIC {
        return Err("Invalid Affinity container magic (expected \\x00\\xffKA)".to_string());
    }

    if &bytes[4..8] != AFFINITY_DOC_KIND {
        return Err("Invalid Affinity document signature (expected 'nsrP')".to_string());
    }

    let version = u16::from_le_bytes([bytes[8], bytes[9]]);
    if !SUPPORTED_VERSIONS.contains(&version) {
        return Err(format!(
            "Unsupported Affinity container version: {} (supported: 8..=12)",
            version
        ));
    }

    let flags = u16::from_le_bytes([bytes[10], bytes[11]]);
    if (flags & 0x03) != 0 {
        return Err("Unsupported Affinity container variant flags".to_string());
    }

    let fat_offset = u64::from_le_bytes([
        bytes[12], bytes[13], bytes[14], bytes[15], bytes[16], bytes[17], bytes[18], bytes[19],
    ]);

    let thumbnail_offset = u64::from_le_bytes([
        bytes[20], bytes[21], bytes[22], bytes[23], bytes[24], bytes[25], bytes[26], bytes[27],
    ]);

    let saved_timestamp = u32::from_le_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]);

    Ok(AffinityHeader {
        version,
        flags,
        fat_offset,
        thumbnail_offset,
        saved_timestamp,
    })
}

/// Parses an Affinity archive, indexing its FAT entries and preview metadata.
pub fn parse_affinity_archive(bytes: &[u8]) -> Result<AffinityArchive, String> {
    let header = read_affinity_header(bytes)?;
    let mut entries = HashMap::new();

    // Read FAT if within bounds
    let fat_off = header.fat_offset as usize;
    if fat_off > 0 && fat_off < bytes.len() {
        let mut cur = fat_off;
        if cur + 4 <= bytes.len() {
            let entry_count = u32::from_le_bytes([
                bytes[cur],
                bytes[cur + 1],
                bytes[cur + 2],
                bytes[cur + 3],
            ]) as usize;
            cur += 4;

            let max_entries = entry_count.min(4096);
            for _ in 0..max_entries {
                if cur + 36 > bytes.len() {
                    break;
                }
                let id = u32::from_le_bytes([
                    bytes[cur],
                    bytes[cur + 1],
                    bytes[cur + 2],
                    bytes[cur + 3],
                ]);
                let offset = u64::from_le_bytes([
                    bytes[cur + 4],
                    bytes[cur + 5],
                    bytes[cur + 6],
                    bytes[cur + 7],
                    bytes[cur + 8],
                    bytes[cur + 9],
                    bytes[cur + 10],
                    bytes[cur + 11],
                ]);
                let size = u64::from_le_bytes([
                    bytes[cur + 12],
                    bytes[cur + 13],
                    bytes[cur + 14],
                    bytes[cur + 15],
                    bytes[cur + 16],
                    bytes[cur + 17],
                    bytes[cur + 18],
                    bytes[cur + 19],
                ]);
                let compressed_size = u64::from_le_bytes([
                    bytes[cur + 20],
                    bytes[cur + 21],
                    bytes[cur + 22],
                    bytes[cur + 23],
                    bytes[cur + 24],
                    bytes[cur + 25],
                    bytes[cur + 26],
                    bytes[cur + 27],
                ]);
                let crc = u32::from_le_bytes([
                    bytes[cur + 28],
                    bytes[cur + 29],
                    bytes[cur + 30],
                    bytes[cur + 31],
                ]);
                let compression_type = bytes[cur + 32];
                let name_len = bytes[cur + 33] as usize;
                cur += 34;

                let name = if cur + name_len <= bytes.len() {
                    let s = String::from_utf8_lossy(&bytes[cur..cur + name_len]).to_string();
                    cur += name_len;
                    s
                } else {
                    format!("entry_{}", id)
                };

                entries.insert(
                    name.clone(),
                    AffinityEntry {
                        id,
                        name,
                        offset,
                        size,
                        compressed_size,
                        crc,
                        compression_type,
                    },
                );
            }
        }
    }

    // Inspect Preview if offset non-zero
    let preview = extract_affinity_preview(bytes, header.thumbnail_offset).ok();

    let doc_type = if entries.contains_key("doc.dat") {
        "Affinity Document".to_string()
    } else {
        "Affinity Container".to_string()
    };

    Ok(AffinityArchive {
        header,
        entries,
        preview,
        document_type: doc_type,
    })
}

/// Inspects the embedded preview thumbnail record without full image decode.
pub fn extract_affinity_preview(bytes: &[u8], thumbnail_offset: u64) -> Result<AffinityPreview, String> {
    if thumbnail_offset == 0 {
        return Err("No embedded preview offset specified".to_string());
    }

    let offset = thumbnail_offset as usize;
    if offset < 72 || offset + 32 > bytes.len() {
        return Err("Thumbnail offset out of bounds".to_string());
    }

    // Must start with `\xff\xff\xff\xffThmb`
    if &bytes[offset..offset + 8] != b"\xff\xff\xff\xffThmb" {
        return Err("Missing thumbnail record signature (expected \\xff\\xff\\xff\\xffThmb)".to_string());
    }

    let rec_size = u32::from_le_bytes([
        bytes[offset + 24],
        bytes[offset + 25],
        bytes[offset + 26],
        bytes[offset + 27],
    ]) as usize;

    if rec_size > MAX_PREVIEW_BYTES {
        return Err("Preview exceeds maximum size limit (16 MiB)".to_string());
    }

    let png_offset = offset + 29;
    if png_offset + rec_size > bytes.len() {
        return Err("Thumbnail data truncated".to_string());
    }

    let png_data = &bytes[png_offset..png_offset + rec_size];
    if png_data.len() < 24 || &png_data[0..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("Thumbnail data is not a valid PNG".to_string());
    }

    // Read IHDR dimensions (PNG integers are big-endian)
    let width = u32::from_be_bytes([png_data[16], png_data[17], png_data[18], png_data[19]]);
    let height = u32::from_be_bytes([png_data[20], png_data[21], png_data[22], png_data[23]]);

    Ok(AffinityPreview {
        width,
        height,
        data_length: rec_size,
        mime_type: "image/png".to_string(),
    })
}

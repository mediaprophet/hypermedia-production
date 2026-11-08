//! TrueType/CFF glyph outline parser and font subsetter tables.
//!
//! Grounded in authoring rights and digital document preservation (UDHR Art. 19, 27).

use std::collections::HashMap;

/// SFNT table entry in an OpenType / TrueType font.
#[derive(Clone, Debug, PartialEq)]
pub struct SfntTable {
    pub tag: [u8; 4],
    pub checksum: u32,
    pub offset: u32,
    pub length: u32,
    pub data: Vec<u8>,
}

/// Calculate OpenType table checksum (sum of big-endian u32 words).
pub fn calc_table_checksum(data: &[u8]) -> u32 {
    let mut sum: u32 = 0;
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();
    for c in chunks {
        sum = sum.wrapping_add(u32::from_be_bytes([c[0], c[1], c[2], c[3]]));
    }
    if !remainder.is_empty() {
        let mut pad = [0u8; 4];
        pad[..remainder.len()].copy_from_slice(remainder);
        sum = sum.wrapping_add(u32::from_be_bytes(pad));
    }
    sum
}

/// Parse the SFNT table directory from raw font bytes.
pub fn parse_sfnt_directory(bytes: &[u8]) -> Result<Vec<SfntTable>, &'static str> {
    if bytes.len() < 12 {
        return Err("Font data too small for SFNT header");
    }

    let magic = &bytes[0..4];
    if magic != b"\x00\x01\x00\x00" && magic != b"OTTO" && magic != b"true" {
        return Err("Unrecognized SFNT scaler type");
    }

    let num_tables = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    if bytes.len() < 12 + num_tables * 16 {
        return Err("Font data truncated in table directory");
    }

    let mut tables = Vec::with_capacity(num_tables);
    for i in 0..num_tables {
        let rec_start = 12 + i * 16;
        let mut tag = [0u8; 4];
        tag.copy_from_slice(&bytes[rec_start..rec_start + 4]);
        let checksum = u32::from_be_bytes([
            bytes[rec_start + 4],
            bytes[rec_start + 5],
            bytes[rec_start + 6],
            bytes[rec_start + 7],
        ]);
        let offset = u32::from_be_bytes([
            bytes[rec_start + 8],
            bytes[rec_start + 9],
            bytes[rec_start + 10],
            bytes[rec_start + 11],
        ]) as usize;
        let length = u32::from_be_bytes([
            bytes[rec_start + 12],
            bytes[rec_start + 13],
            bytes[rec_start + 14],
            bytes[rec_start + 15],
        ]) as usize;

        if offset + length > bytes.len() {
            return Err("Table data offset exceeds font buffer");
        }

        let data = bytes[offset..offset + length].to_vec();
        tables.push(SfntTable {
            tag,
            checksum,
            offset: offset as u32,
            length: length as u32,
            data,
        });
    }

    Ok(tables)
}

/// Assemble a valid standalone SFNT font binary from a set of table records.
pub fn build_sfnt_font(is_cff: bool, tables: &[SfntTable]) -> Vec<u8> {
    let num_tables = tables.len() as u16;
    let mut out = Vec::new();

    // 1. Header (12 bytes)
    if is_cff {
        out.extend_from_slice(b"OTTO");
    } else {
        out.extend_from_slice(b"\x00\x01\x00\x00");
    }
    out.extend_from_slice(&num_tables.to_be_bytes());

    let entry_selector = (num_tables as f64).log2().floor() as u16;
    let search_range = (1 << entry_selector) * 16;
    let range_shift = num_tables * 16 - search_range;

    out.extend_from_slice(&search_range.to_be_bytes());
    out.extend_from_slice(&entry_selector.to_be_bytes());
    out.extend_from_slice(&range_shift.to_be_bytes());

    // 2. Table records placeholder (16 bytes per table)
    let records_start = out.len();
    out.resize(records_start + tables.len() * 16, 0);

    // 3. Table data (4-byte aligned)
    let mut current_offset = out.len();
    for (i, t) in tables.iter().enumerate() {
        let rec_pos = records_start + i * 16;
        let checksum = calc_table_checksum(&t.data);
        let length = t.data.len() as u32;

        out[rec_pos..rec_pos + 4].copy_from_slice(&t.tag);
        out[rec_pos + 4..rec_pos + 8].copy_from_slice(&checksum.to_be_bytes());
        out[rec_pos + 8..rec_pos + 12].copy_from_slice(&(current_offset as u32).to_be_bytes());
        out[rec_pos + 12..rec_pos + 16].copy_from_slice(&length.to_be_bytes());

        out.extend_from_slice(&t.data);
        // Pad to 4-byte boundary
        while out.len() % 4 != 0 {
            out.push(0);
        }
        current_offset = out.len();
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_checksum_calculation() {
        let data = [0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x20];
        let ck = calc_table_checksum(&data);
        assert_eq!(ck, 0x00010000 + 0x00020020);
    }

    #[test]
    fn build_and_parse_sfnt_roundtrip() {
        let tables = [
            SfntTable {
                tag: *b"head",
                checksum: 0,
                offset: 0,
                length: 54,
                data: vec![0u8; 54],
            },
            SfntTable {
                tag: *b"cmap",
                checksum: 0,
                offset: 0,
                length: 32,
                data: vec![1u8; 32],
            },
        ];
        let bytes = build_sfnt_font(false, &tables);
        let parsed = parse_sfnt_directory(&bytes).unwrap();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].tag, *b"head");
        assert_eq!(parsed[1].tag, *b"cmap");
    }
}

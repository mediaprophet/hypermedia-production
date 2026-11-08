//! PMTiles v3 cloud-native archive header and directory entry decoder.

use serde::{Deserialize, Serialize};

/// PMTiles v3 compression types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Compression {
    Unknown = 0,
    None = 1,
    Gzip = 2,
    Brotli = 3,
    Zstd = 4,
}

impl Compression {
    pub fn from_u8(b: u8) -> Self {
        match b {
            1 => Self::None,
            2 => Self::Gzip,
            3 => Self::Brotli,
            4 => Self::Zstd,
            _ => Self::Unknown,
        }
    }
}

/// PMTiles v3 tile types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileType {
    Unknown = 0,
    Mvt = 1,
    Png = 2,
    Jpeg = 3,
    Webp = 4,
    Avif = 5,
}

impl TileType {
    pub fn from_u8(b: u8) -> Self {
        match b {
            1 => Self::Mvt,
            2 => Self::Png,
            3 => Self::Jpeg,
            4 => Self::Webp,
            5 => Self::Avif,
            _ => Self::Unknown,
        }
    }
}

/// PMTiles v3 127-byte archive header.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PmTilesHeader {
    pub root_dir_offset: u64,
    pub root_dir_len: u64,
    pub metadata_offset: u64,
    pub metadata_len: u64,
    pub leaf_dirs_offset: u64,
    pub leaf_dirs_len: u64,
    pub tile_data_offset: u64,
    pub tile_data_len: u64,
    pub addressed_tiles_count: u64,
    pub tile_entries_count: u64,
    pub tile_contents_count: u64,
    pub clustered: bool,
    pub internal_compression: Compression,
    pub tile_compression: Compression,
    pub tile_type: TileType,
    pub min_zoom: u8,
    pub max_zoom: u8,
    pub min_lon: f64,
    pub min_lat: f64,
    pub max_lon: f64,
    pub max_lat: f64,
    pub center_zoom: u8,
    pub center_lon: f64,
    pub center_lat: f64,
}

/// Parses PMTiles v3 header from a 127-byte slice.
pub fn parse_pmtiles_header(bytes: &[u8]) -> Option<PmTilesHeader> {
    if bytes.len() < 127 || &bytes[0..7] != b"PMTiles" {
        return None;
    }
    let version = bytes[7];
    if version != 3 {
        return None;
    }

    let read_u64 = |idx: usize| -> Option<u64> {
        let b: [u8; 8] = bytes[idx..idx + 8].try_into().ok()?;
        Some(u64::from_le_bytes(b))
    };
    let read_i32 = |idx: usize| -> Option<i32> {
        let b: [u8; 4] = bytes[idx..idx + 4].try_into().ok()?;
        Some(i32::from_le_bytes(b))
    };

    let root_dir_offset = read_u64(8)?;
    let root_dir_len = read_u64(16)?;
    let metadata_offset = read_u64(24)?;
    let metadata_len = read_u64(32)?;
    let leaf_dirs_offset = read_u64(40)?;
    let leaf_dirs_len = read_u64(48)?;
    let tile_data_offset = read_u64(56)?;
    let tile_data_len = read_u64(64)?;
    let addressed_tiles_count = read_u64(72)?;
    let tile_entries_count = read_u64(80)?;
    let tile_contents_count = read_u64(88)?;

    let clustered = bytes[96] == 1;
    let internal_compression = Compression::from_u8(bytes[97]);
    let tile_compression = Compression::from_u8(bytes[98]);
    let tile_type = TileType::from_u8(bytes[99]);
    let min_zoom = bytes[100];
    let max_zoom = bytes[101];

    let min_lon = (read_i32(102)? as f64) / 1e7;
    let min_lat = (read_i32(106)? as f64) / 1e7;
    let max_lon = (read_i32(110)? as f64) / 1e7;
    let max_lat = (read_i32(114)? as f64) / 1e7;
    let center_zoom = bytes[118];
    let center_lon = (read_i32(119)? as f64) / 1e7;
    let center_lat = (read_i32(123)? as f64) / 1e7;

    Some(PmTilesHeader {
        root_dir_offset,
        root_dir_len,
        metadata_offset,
        metadata_len,
        leaf_dirs_offset,
        leaf_dirs_len,
        tile_data_offset,
        tile_data_len,
        addressed_tiles_count,
        tile_entries_count,
        tile_contents_count,
        clustered,
        internal_compression,
        tile_compression,
        tile_type,
        min_zoom,
        max_zoom,
        min_lon,
        min_lat,
        max_lon,
        max_lat,
        center_zoom,
        center_lon,
        center_lat,
    })
}

/// A directory entry mapping a tile ID to byte range in the archive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PmTilesEntry {
    pub tile_id: u64,
    pub offset: u64,
    pub length: u32,
    pub run_length: u32,
}

/// Decodes varint from byte stream.
fn read_varint(bytes: &[u8], offset: &mut usize) -> Option<u64> {
    let mut result = 0u64;
    let mut shift = 0;
    while *offset < bytes.len() {
        let byte = bytes[*offset];
        *offset += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if (byte & 0x80) == 0 {
            return Some(result);
        }
        shift += 7;
        if shift > 63 {
            return None;
        }
    }
    None
}

/// Decodes an uncompressed PMTiles v3 directory into tile entries.
pub fn decode_pmtiles_directory(bytes: &[u8]) -> Vec<PmTilesEntry> {
    let mut entries = Vec::new();
    let mut offset = 0;

    let num_entries = match read_varint(bytes, &mut offset) {
        Some(n) => n as usize,
        None => return entries,
    };

    if num_entries == 0 || num_entries > 1_000_000 {
        return entries;
    }

    let mut last_id = 0u64;
    for _ in 0..num_entries {
        let id_delta = match read_varint(bytes, &mut offset) {
            Some(d) => d,
            None => break,
        };
        last_id += id_delta;
        let run_length = read_varint(bytes, &mut offset).unwrap_or(1) as u32;
        let length = read_varint(bytes, &mut offset).unwrap_or(0) as u32;
        let tile_offset = read_varint(bytes, &mut offset).unwrap_or(0);

        entries.push(PmTilesEntry {
            tile_id: last_id,
            offset: tile_offset,
            length,
            run_length,
        });
    }

    entries
}

/// Converts (zoom, x, y) tile coordinates to PMTiles Hilbert tile ID.
pub fn zxy_to_tile_id(z: u8, x: u32, y: u32) -> u64 {
    if z == 0 {
        return 0;
    }
    let mut acc = 0u64;
    for i in 0..z {
        acc += 1u64 << (2 * i);
    }
    acc + hilbert_xy2d(1 << z, x, y)
}

fn hilbert_xy2d(n: u32, mut x: u32, mut y: u32) -> u64 {
    let mut d = 0u64;
    let mut s = n / 2;
    while s > 0 {
        let rx: u32 = if (x & s) > 0 { 1 } else { 0 };
        let ry: u32 = if (y & s) > 0 { 1 } else { 0 };
        d += (s as u64) * (s as u64) * ((3 * rx as u64) ^ (ry as u64));
        rot(s, &mut x, &mut y, rx, ry);
        s /= 2;
    }
    d
}

fn rot(n: u32, x: &mut u32, y: &mut u32, rx: u32, ry: u32) {
    if ry == 0 {
        if rx == 1 {
            *x = n - 1 - *x;
            *y = n - 1 - *y;
        }
        std::mem::swap(x, y);
    }
}

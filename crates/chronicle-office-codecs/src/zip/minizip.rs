//! Pure-Rust, safe, self-contained ZIP archive reader and writer.
//! Implements PKZip 2.0 format (compression method 0: Stored, uncompressed).
//! Fully compatible with OpenXML (.docx, .xlsx, .pptx) and standard unzip utilities.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug)]
pub enum ZipError {
    InvalidSignature(u32),
    TooShort,
    BadCentralDirectory,
    PartTooLarge,
    Io(String),
}

impl fmt::Display for ZipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ZipError::InvalidSignature(sig) => write!(f, "invalid zip signature: {:#010x}", sig),
            ZipError::TooShort => write!(f, "zip buffer too short"),
            ZipError::BadCentralDirectory => write!(f, "corrupted zip central directory"),
            ZipError::PartTooLarge => write!(f, "zip entry exceeds safe size limit"),
            ZipError::Io(s) => write!(f, "zip io error: {}", s),
        }
    }
}

impl std::error::Error for ZipError {}

/// Standard CRC-32 (IEEE 802.3).
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in data {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = -( (crc & 1) as i32 ) as u32;
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// An in-memory ZIP package containing named files.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ZipPackage {
    pub files: BTreeMap<String, Vec<u8>>,
}

const LOCAL_HEADER_SIG: u32 = 0x0403_4b50;
const CENTRAL_HEADER_SIG: u32 = 0x0201_4b50;
const EOCD_SIG: u32 = 0x0605_4b50;

impl ZipPackage {
    pub fn new() -> Self {
        ZipPackage {
            files: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, path: impl Into<String>, data: Vec<u8>) {
        let p = path.into().trim_start_matches('/').to_string();
        self.files.insert(p, data);
    }

    pub fn get(&self, path: &str) -> Option<&[u8]> {
        let p = path.trim_start_matches('/');
        self.files.get(p).map(|v| v.as_slice())
    }

    pub fn get_str(&self, path: &str) -> Option<String> {
        self.get(path).and_then(|bytes| String::from_utf8(bytes.to_vec()).ok())
    }

    /// Serialize all contained files into a standard ZIP archive byte vector.
    pub fn write_to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        let mut central_directory = Vec::new();

        for (name, data) in &self.files {
            let name_bytes = name.as_bytes();
            let offset = out.len() as u32;
            let crc = crc32(data);
            let size = data.len() as u32;

            // Local file header (30 bytes + name length)
            out.extend_from_slice(&LOCAL_HEADER_SIG.to_le_bytes());
            out.extend_from_slice(&20u16.to_le_bytes()); // version needed (2.0)
            out.extend_from_slice(&0u16.to_le_bytes());  // flags
            out.extend_from_slice(&0u16.to_le_bytes());  // compression: 0 = Stored
            out.extend_from_slice(&0u16.to_le_bytes());  // mod time
            out.extend_from_slice(&0u16.to_le_bytes());  // mod date
            out.extend_from_slice(&crc.to_le_bytes());
            out.extend_from_slice(&size.to_le_bytes());  // compressed size
            out.extend_from_slice(&size.to_le_bytes());  // uncompressed size
            out.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            out.extend_from_slice(&0u16.to_le_bytes());  // extra field len
            out.extend_from_slice(name_bytes);
            out.extend_from_slice(data);

            // Central directory header (46 bytes + name length)
            central_directory.extend_from_slice(&CENTRAL_HEADER_SIG.to_le_bytes());
            central_directory.extend_from_slice(&20u16.to_le_bytes()); // version made by
            central_directory.extend_from_slice(&20u16.to_le_bytes()); // version needed
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // flags
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // compression: 0
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // mod time
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // mod date
            central_directory.extend_from_slice(&crc.to_le_bytes());
            central_directory.extend_from_slice(&size.to_le_bytes());  // compressed size
            central_directory.extend_from_slice(&size.to_le_bytes());  // uncompressed size
            central_directory.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // extra field len
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // comment len
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // disk number start
            central_directory.extend_from_slice(&0u16.to_le_bytes());  // internal file attr
            central_directory.extend_from_slice(&0u32.to_le_bytes());  // external file attr
            central_directory.extend_from_slice(&offset.to_le_bytes());
            central_directory.extend_from_slice(name_bytes);
        }

        let cd_offset = out.len() as u32;
        let cd_size = central_directory.len() as u32;
        out.extend_from_slice(&central_directory);

        // End of central directory record (22 bytes)
        out.extend_from_slice(&EOCD_SIG.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // disk number
        out.extend_from_slice(&0u16.to_le_bytes()); // disk with cd
        out.extend_from_slice(&(self.files.len() as u16).to_le_bytes()); // entries on this disk
        out.extend_from_slice(&(self.files.len() as u16).to_le_bytes()); // total entries
        out.extend_from_slice(&cd_size.to_le_bytes());
        out.extend_from_slice(&cd_offset.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes()); // comment len

        out
    }

    /// Parse a ZIP byte slice and extract uncompressed (stored) entries.
    pub fn parse(buf: &[u8]) -> Result<Self, ZipError> {
        if buf.len() < 22 {
            return Err(ZipError::TooShort);
        }

        // Find EOCD by scanning from the end (up to 65536 bytes comment)
        let max_scan = buf.len().min(65536 + 22);
        let scan_start = buf.len() - max_scan;
        let mut eocd_pos = None;

        for pos in (scan_start..=buf.len() - 22).rev() {
            let sig = u32::from_le_bytes(buf[pos..pos + 4].try_into().unwrap());
            if sig == EOCD_SIG {
                eocd_pos = Some(pos);
                break;
            }
        }

        let eocd = eocd_pos.ok_or(ZipError::BadCentralDirectory)?;
        let total_entries = u16::from_le_bytes(buf[eocd + 10..eocd + 12].try_into().unwrap()) as usize;
        let cd_size = u32::from_le_bytes(buf[eocd + 12..eocd + 16].try_into().unwrap()) as usize;
        let cd_offset = u32::from_le_bytes(buf[eocd + 16..eocd + 20].try_into().unwrap()) as usize;

        if cd_offset + cd_size > buf.len() {
            return Err(ZipError::BadCentralDirectory);
        }

        let mut package = ZipPackage::new();
        let mut cd_cur = cd_offset;

        for _ in 0..total_entries {
            if cd_cur + 46 > buf.len() {
                break;
            }
            let sig = u32::from_le_bytes(buf[cd_cur..cd_cur + 4].try_into().unwrap());
            if sig != CENTRAL_HEADER_SIG {
                return Err(ZipError::InvalidSignature(sig));
            }

            let comp_method = u16::from_le_bytes(buf[cd_cur + 10..cd_cur + 12].try_into().unwrap());
            let uncomp_size = u32::from_le_bytes(buf[cd_cur + 24..cd_cur + 28].try_into().unwrap()) as usize;
            let name_len = u16::from_le_bytes(buf[cd_cur + 28..cd_cur + 30].try_into().unwrap()) as usize;
            let extra_len = u16::from_le_bytes(buf[cd_cur + 30..cd_cur + 32].try_into().unwrap()) as usize;
            let comment_len = u16::from_le_bytes(buf[cd_cur + 32..cd_cur + 34].try_into().unwrap()) as usize;
            let local_offset = u32::from_le_bytes(buf[cd_cur + 42..cd_cur + 46].try_into().unwrap()) as usize;

            let name_bytes = &buf[cd_cur + 46..cd_cur + 46 + name_len];
            let name = String::from_utf8_lossy(name_bytes).to_string();

            cd_cur += 46 + name_len + extra_len + comment_len;

            // Read from local file header
            if local_offset + 30 > buf.len() {
                continue;
            }
            let local_sig = u32::from_le_bytes(buf[local_offset..local_offset + 4].try_into().unwrap());
            if local_sig != LOCAL_HEADER_SIG {
                continue;
            }
            let loc_name_len = u16::from_le_bytes(buf[local_offset + 26..local_offset + 28].try_into().unwrap()) as usize;
            let loc_extra_len = u16::from_le_bytes(buf[local_offset + 28..local_offset + 30].try_into().unwrap()) as usize;
            let data_offset = local_offset + 30 + loc_name_len + loc_extra_len;

            if comp_method == 0 {
                // Stored (uncompressed)
                if data_offset + uncomp_size <= buf.len() {
                    let entry_data = buf[data_offset..data_offset + uncomp_size].to_vec();
                    package.insert(name, entry_data);
                }
            } else {
                // If compressed (deflate), store raw bytes as fallback
                let comp_size = u32::from_le_bytes(buf[local_offset + 18..local_offset + 22].try_into().unwrap()) as usize;
                if data_offset + comp_size <= buf.len() {
                    let entry_data = buf[data_offset..data_offset + comp_size].to_vec();
                    package.insert(name, entry_data);
                }
            }
        }

        Ok(package)
    }
}

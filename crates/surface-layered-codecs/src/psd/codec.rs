//! PSD / PSB binary encoding, decoding, and PackBits RLE codec.

use super::{
    PsdBlendMode, PsdChannelInfo, PsdColorMode, PsdCompression, PsdDocument, PsdHeader, PsdLayer,
    PsdVersion,
};
use std::collections::HashMap;

#[inline]
pub(super) fn read_u16(b: &[u8], o: usize) -> u16 {
    u16::from_be_bytes([b[o], b[o + 1]])
}

#[inline]
pub(super) fn read_i16(b: &[u8], o: usize) -> i16 {
    i16::from_be_bytes([b[o], b[o + 1]])
}

#[inline]
pub(super) fn read_u32(b: &[u8], o: usize) -> u32 {
    u32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

#[inline]
pub(super) fn read_i32(b: &[u8], o: usize) -> i32 {
    i32::from_be_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

#[inline]
fn read_u64(b: &[u8], o: usize) -> u64 {
    u64::from_be_bytes([
        b[o],
        b[o + 1],
        b[o + 2],
        b[o + 3],
        b[o + 4],
        b[o + 5],
        b[o + 6],
        b[o + 7],
    ])
}

/// Apple / Photoshop PackBits RLE compression codec.
pub mod packbits {
    /// Encodes data using PackBits run-length encoding.
    pub fn encode(src: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(src.len() + src.len() / 64 + 1);
        let mut i = 0;
        let n = src.len();

        while i < n {
            // Check for run of identical bytes
            let mut run = 1;
            while i + run < n && run < 128 && src[i + run] == src[i] {
                run += 1;
            }

            if run >= 2 {
                // Emit run packet: (1 - count) as i8 as u8, then the byte
                let header = (1i16 - run as i16) as i8 as u8;
                out.push(header);
                out.push(src[i]);
                i += run;
            } else {
                // Literal run: collect until run of >= 3 or 128 bytes
                let start = i;
                let mut lit_len = 0;
                while i < n && lit_len < 128 {
                    if i + 2 < n && src[i] == src[i + 1] && src[i] == src[i + 2] {
                        break;
                    }
                    i += 1;
                    lit_len += 1;
                }
                let header = (lit_len - 1) as u8;
                out.push(header);
                out.extend_from_slice(&src[start..start + lit_len]);
            }
        }
        out
    }

    /// Decodes PackBits compressed data until `expected_len` bytes are produced.
    pub fn decode(src: &[u8], expected_len: usize) -> Result<Vec<u8>, String> {
        let mut out = Vec::with_capacity(expected_len);
        let mut i = 0;

        while out.len() < expected_len && i < src.len() {
            let h = src[i] as i8;
            i += 1;

            if h >= 0 {
                // Literal: copy (h + 1) bytes
                let count = (h as usize) + 1;
                if i + count > src.len() {
                    return Err("PackBits literal packet exceeds input buffer".to_string());
                }
                let chunk = &src[i..i + count];
                let take = count.min(expected_len - out.len());
                out.extend_from_slice(&chunk[..take]);
                i += count;
            } else if h != -128 {
                // Run: repeat next byte (1 - h) times
                let count = (1i16 - h as i16) as usize;
                if i >= src.len() {
                    return Err("PackBits run packet missing repeated byte".to_string());
                }
                let b = src[i];
                i += 1;
                let take = count.min(expected_len - out.len());
                out.resize(out.len() + take, b);
            }
            // h == -128 is a no-op pad byte
        }

        if out.len() != expected_len {
            return Err(format!(
                "PackBits decompression produced {} bytes, expected {}",
                out.len(),
                expected_len
            ));
        }

        Ok(out)
    }
}

impl PsdDocument {
    /// Creates a blank document with given dimensions and color mode.
    pub fn new(width: u32, height: u32, color_mode: PsdColorMode, is_psb: bool) -> Self {
        let version = if is_psb {
            PsdVersion::Psb
        } else {
            PsdVersion::Psd
        };
        let channels = (color_mode.channel_count() + 1) as u16; // color channels + alpha
        Self {
            header: PsdHeader {
                version,
                channels,
                height,
                width,
                depth: 8,
                color_mode,
            },
            color_mode_data: Vec::new(),
            image_resources: Vec::new(),
            layers: Vec::new(),
            composite_pixels: vec![255; (width * height * channels as u32) as usize],
        }
    }

    /// Decodes a PSD or PSB document from bytes.
    pub fn decode(bytes: &[u8]) -> Result<Self, String> {
        let (header, mut offset) = PsdHeader::decode(bytes)?;

        // 1. Color Mode Data Section
        if offset + 4 > bytes.len() {
            return Err("Unexpected EOF reading color mode data section length".to_string());
        }
        let color_data_len = read_u32(bytes, offset) as usize;
        offset += 4;
        if offset + color_data_len > bytes.len() {
            return Err("Unexpected EOF reading color mode data".to_string());
        }
        let color_mode_data = bytes[offset..offset + color_data_len].to_vec();
        offset += color_data_len;

        // 2. Image Resources Section
        if offset + 4 > bytes.len() {
            return Err("Unexpected EOF reading image resources section length".to_string());
        }
        let res_len = read_u32(bytes, offset) as usize;
        offset += 4;
        if offset + res_len > bytes.len() {
            return Err("Unexpected EOF reading image resources".to_string());
        }
        let image_resources = bytes[offset..offset + res_len].to_vec();
        offset += res_len;

        // 3. Layer and Mask Information Section
        let (layers, new_offset) = Self::decode_layer_section(bytes, offset, header.version)?;
        offset = new_offset;

        // 4. Global Composite Image Data Section
        let mut composite_pixels = Vec::new();
        if offset + 2 <= bytes.len() {
            let comp_method = read_u16(bytes, offset);
            offset += 2;
            let comp_compression = PsdCompression::from_u16(comp_method);
            let total_pixels = (header.width * header.height) as usize;
            let expected_bytes = total_pixels * (header.channels as usize);

            if comp_compression == PsdCompression::Raw {
                let avail = bytes.len() - offset;
                let take = avail.min(expected_bytes);
                composite_pixels.extend_from_slice(&bytes[offset..offset + take]);
            } else if comp_compression == PsdCompression::Rle {
                // Read row counts
                let rows = (header.height as usize) * (header.channels as usize);
                let row_table_bytes = rows * if header.version.is_psb() { 4 } else { 2 };
                if offset + row_table_bytes <= bytes.len() {
                    offset += row_table_bytes;
                    // Decode packed rows
                    let remaining = &bytes[offset..];
                    if let Ok(decoded) = packbits::decode(remaining, expected_bytes) {
                        composite_pixels = decoded;
                    }
                }
            }
        }

        Ok(PsdDocument {
            header,
            color_mode_data,
            image_resources,
            layers,
            composite_pixels,
        })
    }

    fn decode_layer_section(
        bytes: &[u8],
        mut offset: usize,
        version: PsdVersion,
    ) -> Result<(Vec<PsdLayer>, usize), String> {
        let is_psb = version.is_psb();
        let section_len = if is_psb {
            if offset + 8 > bytes.len() {
                return Ok((Vec::new(), offset));
            }
            let len = read_u64(bytes, offset) as usize;
            offset += 8;
            len
        } else {
            if offset + 4 > bytes.len() {
                return Ok((Vec::new(), offset));
            }
            let len = read_u32(bytes, offset) as usize;
            offset += 4;
            len
        };

        let section_end = offset + section_len;
        if section_len == 0 || section_end > bytes.len() {
            return Ok((Vec::new(), section_end.min(bytes.len())));
        }

        // Layer info length
        let layer_info_len = if is_psb {
            let len = read_u64(bytes, offset) as usize;
            offset += 8;
            len
        } else {
            let len = read_u32(bytes, offset) as usize;
            offset += 4;
            len
        };

        if layer_info_len == 0 || offset + 2 > section_end {
            return Ok((Vec::new(), section_end));
        }

        // Layer count
        let raw_count = read_i16(bytes, offset);
        offset += 2;
        let layer_count = raw_count.unsigned_abs() as usize;

        // Parse Layer Records
        struct RawLayerRecord {
            top: i32,
            left: i32,
            bottom: i32,
            right: i32,
            channels: Vec<PsdChannelInfo>,
            blend_mode: PsdBlendMode,
            opacity: u8,
            clipping: bool,
            visible: bool,
            name: String,
        }

        let mut records = Vec::with_capacity(layer_count);

        for _ in 0..layer_count {
            if offset + 16 > section_end {
                break;
            }
            let top = read_i32(bytes, offset);
            let left = read_i32(bytes, offset + 4);
            let bottom = read_i32(bytes, offset + 8);
            let right = read_i32(bytes, offset + 12);
            offset += 16;

            let num_channels = read_u16(bytes, offset) as usize;
            offset += 2;

            let mut channels = Vec::with_capacity(num_channels);
            for _ in 0..num_channels {
                let chan_id = read_i16(bytes, offset);
                offset += 2;
                let chan_len = if is_psb {
                    let len = read_u64(bytes, offset);
                    offset += 8;
                    len
                } else {
                    let len = read_u32(bytes, offset) as u64;
                    offset += 4;
                    len
                };
                channels.push(PsdChannelInfo {
                    id: chan_id,
                    length: chan_len,
                });
            }

            // Blend mode sig ('8BIM') + key
            offset += 4;
            let blend_key = [
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ];
            offset += 4;
            let blend_mode = PsdBlendMode::from_key(&blend_key);

            let opacity = bytes[offset];
            let clipping = bytes[offset + 1] != 0;
            let flags = bytes[offset + 2];
            let visible = (flags & 0x02) == 0; // bit 1: 0 = visible, 1 = invisible
            offset += 4; // opacity, clipping, flags, filler

            // Extra data length
            let extra_len = read_u32(bytes, offset) as usize;
            offset += 4;
            let extra_end = offset + extra_len;

            // Mask data length
            let mask_len = read_u32(bytes, offset) as usize;
            offset += 4 + mask_len;

            // Blending ranges length
            let ranges_len = read_u32(bytes, offset) as usize;
            offset += 4 + ranges_len;

            // Layer name (Pascal string padded to 4 bytes)
            let name_len = bytes[offset] as usize;
            offset += 1;
            let name_bytes = &bytes[offset..offset + name_len];
            let name = String::from_utf8_lossy(name_bytes).to_string();

            offset = extra_end; // Skip any remaining tagged blocks

            records.push(RawLayerRecord {
                top,
                left,
                bottom,
                right,
                channels,
                blend_mode,
                opacity,
                clipping,
                visible,
                name,
            });
        }

        // Parse Channel Image Data for each layer
        let mut layers = Vec::with_capacity(records.len());
        for rec in records {
            let w = (rec.right - rec.left).max(0) as usize;
            let h = (rec.bottom - rec.top).max(0) as usize;
            let expected_channel_len = w * h;

            let mut channel_data = HashMap::new();

            for chan_info in &rec.channels {
                if expected_channel_len == 0 {
                    channel_data.insert(chan_info.id, Vec::new());
                    continue;
                }

                if offset + 2 > section_end {
                    break;
                }
                let comp_id = read_u16(bytes, offset);
                offset += 2;
                let compression = PsdCompression::from_u16(comp_id);

                if compression == PsdCompression::Raw {
                    let take = expected_channel_len.min(bytes.len() - offset);
                    channel_data.insert(chan_info.id, bytes[offset..offset + take].to_vec());
                    offset += take;
                } else if compression == PsdCompression::Rle {
                    // RLE row byte count table
                    let row_table_bytes = h * if is_psb { 4 } else { 2 };
                    if offset + row_table_bytes <= bytes.len() {
                        offset += row_table_bytes;
                        let rle_data_len =
                            (chan_info.length as usize).saturating_sub(2 + row_table_bytes);
                        let rle_slice = if offset + rle_data_len <= bytes.len() {
                            &bytes[offset..offset + rle_data_len]
                        } else {
                            &bytes[offset..]
                        };

                        if let Ok(decoded) = packbits::decode(rle_slice, expected_channel_len) {
                            channel_data.insert(chan_info.id, decoded);
                        } else {
                            channel_data.insert(chan_info.id, vec![0u8; expected_channel_len]);
                        }
                        offset += rle_data_len.min(bytes.len() - offset);
                    }
                }
            }

            layers.push(PsdLayer {
                name: rec.name,
                top: rec.top,
                left: rec.left,
                bottom: rec.bottom,
                right: rec.right,
                blend_mode: rec.blend_mode,
                opacity: rec.opacity,
                clipping: rec.clipping,
                visible: rec.visible,
                channels: channel_data,
            });
        }

        Ok((layers, section_end))
    }

    /// Serializes the document into PSD or PSB binary format with PackBits compression.
    pub fn encode(&self) -> Vec<u8> {
        let is_psb = self.header.version.is_psb();
        let mut out = Vec::new();

        // 1. Header
        out.extend_from_slice(&self.header.encode());

        // 2. Color Mode Data
        out.extend_from_slice(&(self.color_mode_data.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.color_mode_data);

        // 3. Image Resources
        out.extend_from_slice(&(self.image_resources.len() as u32).to_be_bytes());
        out.extend_from_slice(&self.image_resources);

        // 4. Layer & Mask Information Section
        let mut layer_section = Vec::new();
        let mut layer_info = Vec::new();

        let layer_count = self.layers.len() as i16;
        layer_info.extend_from_slice(&layer_count.to_be_bytes());

        // Pre-encode channel data to calculate channel lengths
        struct EncodedChannel {
            comp_type: u16,
            row_lengths: Vec<u8>,
            data: Vec<u8>,
        }

        let mut layer_encoded_channels: Vec<Vec<(i16, EncodedChannel)>> = Vec::new();

        for layer in &self.layers {
            let h = layer.height() as usize;
            let w = layer.width() as usize;
            let mut chans = Vec::new();

            for (&chan_id, raw_data) in &layer.channels {
                if h == 0 || w == 0 {
                    chans.push((
                        chan_id,
                        EncodedChannel {
                            comp_type: 0,
                            row_lengths: Vec::new(),
                            data: Vec::new(),
                        },
                    ));
                    continue;
                }

                // Compress with PackBits
                let mut row_lengths = Vec::new();
                let mut packed_data = Vec::new();

                for y in 0..h {
                    let row_start = y * w;
                    let row_end = (row_start + w).min(raw_data.len());
                    let row_slice = if row_start < raw_data.len() {
                        &raw_data[row_start..row_end]
                    } else {
                        &[]
                    };
                    let compressed = packbits::encode(row_slice);
                    if is_psb {
                        row_lengths.extend_from_slice(&(compressed.len() as u32).to_be_bytes());
                    } else {
                        row_lengths.extend_from_slice(&(compressed.len() as u16).to_be_bytes());
                    }
                    packed_data.extend_from_slice(&compressed);
                }

                chans.push((
                    chan_id,
                    EncodedChannel {
                        comp_type: 1, // RLE
                        row_lengths,
                        data: packed_data,
                    },
                ));
            }
            layer_encoded_channels.push(chans);
        }

        // Write Layer Records
        for (i, layer) in self.layers.iter().enumerate() {
            layer_info.extend_from_slice(&layer.top.to_be_bytes());
            layer_info.extend_from_slice(&layer.left.to_be_bytes());
            layer_info.extend_from_slice(&layer.bottom.to_be_bytes());
            layer_info.extend_from_slice(&layer.right.to_be_bytes());

            let chans = &layer_encoded_channels[i];
            layer_info.extend_from_slice(&(chans.len() as u16).to_be_bytes());

            for (chan_id, enc) in chans {
                layer_info.extend_from_slice(&chan_id.to_be_bytes());
                let total_chan_len = (2 + enc.row_lengths.len() + enc.data.len()) as u64;
                if is_psb {
                    layer_info.extend_from_slice(&total_chan_len.to_be_bytes());
                } else {
                    layer_info.extend_from_slice(&(total_chan_len as u32).to_be_bytes());
                }
            }

            // Blend mode sig
            layer_info.extend_from_slice(b"8BIM");
            layer_info.extend_from_slice(&layer.blend_mode.to_key());
            layer_info.push(layer.opacity);
            layer_info.push(if layer.clipping { 1 } else { 0 });
            let flags = if layer.visible { 0x08 } else { 0x0a }; // visible flag bit 1 = 0
            layer_info.push(flags);
            layer_info.push(0); // filler

            // Extra data
            let name_bytes = layer.name.as_bytes();
            let name_len = name_bytes.len().min(255);
            let pad = (4 - ((name_len + 1) % 4)) % 4;
            let extra_len = 4 + 4 + (1 + name_len + pad);

            layer_info.extend_from_slice(&(extra_len as u32).to_be_bytes());
            layer_info.extend_from_slice(&0u32.to_be_bytes()); // empty mask
            layer_info.extend_from_slice(&0u32.to_be_bytes()); // empty blending ranges
            layer_info.push(name_len as u8);
            layer_info.extend_from_slice(&name_bytes[..name_len]);
            for _ in 0..pad {
                layer_info.push(0);
            }
        }

        // Write Channel Data for each layer
        for chans in layer_encoded_channels {
            for (_, enc) in chans {
                layer_info.extend_from_slice(&enc.comp_type.to_be_bytes());
                layer_info.extend_from_slice(&enc.row_lengths);
                layer_info.extend_from_slice(&enc.data);
            }
        }

        // Layer info wrapper length
        if is_psb {
            layer_section.extend_from_slice(&(layer_info.len() as u64).to_be_bytes());
        } else {
            layer_section.extend_from_slice(&(layer_info.len() as u32).to_be_bytes());
        }
        layer_section.extend_from_slice(&layer_info);

        // Section header length
        if is_psb {
            out.extend_from_slice(&(layer_section.len() as u64).to_be_bytes());
        } else {
            out.extend_from_slice(&(layer_section.len() as u32).to_be_bytes());
        }
        out.extend_from_slice(&layer_section);

        // 5. Global Composite Image
        out.extend_from_slice(&0u16.to_be_bytes()); // Raw compression
        out.extend_from_slice(&self.composite_pixels);

        out
    }
}

//! Adobe Photoshop (.psd) 8BPS v1 encoder and layer/composite importer.

use super::{decode_packbits, decode_psd_channel, parse_dimensions};
use crate::*;
use vibe::Value;

impl AssetHost<'_> {
    pub(super) fn encode_psd(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["width", "height", "layers", "profile"])?;
        let (w, h, n) = parse_dimensions(a)?;
        let profile = crate::graphics::lattice_profile(a)?;
        let depth16 = profile.bit_depth == 16;
        let bpp = if depth16 { 2usize } else { 1 };
        let layers_list = list(required(a, "layers")?)?;
        if layers_list.is_empty() || layers_list.len() > 64 {
            return Err("PSD layers count outside 1..64".into());
        }

        self.charge(
            n * (layers_list.len() + 1) * 4,
            n * (layers_list.len() + 1) * 8 + 65536,
        )?;

        struct LayerInput<'a> {
            name: &'a str,
            opacity: f32,
            pixels: &'a [[u8; 4]],
        }

        let mut parsed = Vec::with_capacity(layers_list.len());
        for v in layers_list {
            let l = record(v)?;
            fields(l, &["name", "image", "opacity"])?;
            let r = self.rgba(required(l, "image")?)?;
            if r.width != w || r.height != h {
                return Err("Layer dimensions must match document dimensions".into());
            }
            let opacity = number(l, "opacity", 1.0)? as f32;
            if !(0.0..=1.0).contains(&opacity) {
                return Err("Opacity outside 0..1".into());
            }
            let name = match l.get("name") {
                Some(v) => text(v)?,
                None => "Layer",
            };
            parsed.push(LayerInput {
                name,
                opacity,
                pixels: &r.pixels,
            });
        }

        // Alpha-over compositing of layers from bottom to top
        let mut composite = vec![[0u8; 4]; n];
        for layer in &parsed {
            let op = layer.opacity;
            for (i, p) in layer.pixels.iter().enumerate() {
                let src_a = (p[3] as f32 / 255.0) * op;
                if src_a <= 0.0 {
                    continue;
                }
                let dst = &mut composite[i];
                let dst_a = dst[3] as f32 / 255.0;
                if dst_a <= 0.0 {
                    *dst = [
                        p[0],
                        p[1],
                        p[2],
                        (src_a * 255.0).round().clamp(0.0, 255.0) as u8,
                    ];
                } else {
                    let out_a = src_a + dst_a * (1.0 - src_a);
                    if out_a > 0.0 {
                        let out_r =
                            (p[0] as f32 * src_a + dst[0] as f32 * dst_a * (1.0 - src_a)) / out_a;
                        let out_g =
                            (p[1] as f32 * src_a + dst[1] as f32 * dst_a * (1.0 - src_a)) / out_a;
                        let out_b =
                            (p[2] as f32 * src_a + dst[2] as f32 * dst_a * (1.0 - src_a)) / out_a;
                        *dst = [
                            out_r.round().clamp(0.0, 255.0) as u8,
                            out_g.round().clamp(0.0, 255.0) as u8,
                            out_b.round().clamp(0.0, 255.0) as u8,
                            (out_a * 255.0).round().clamp(0.0, 255.0) as u8,
                        ];
                    }
                }
            }
        }

        let mut out = Vec::new();

        // 1. File Header (26 bytes)
        out.extend_from_slice(b"8BPS");
        out.extend_from_slice(&1u16.to_be_bytes()); // Version 1
        out.extend_from_slice(&[0u8; 6]); // Reserved
        out.extend_from_slice(&4u16.to_be_bytes()); // 4 channels RGBA
        out.extend_from_slice(&h.to_be_bytes()); // Height
        out.extend_from_slice(&w.to_be_bytes()); // Width
        out.extend_from_slice(&(if depth16 { 16u16 } else { 8u16 }).to_be_bytes());
        out.extend_from_slice(&3u16.to_be_bytes()); // Color mode 3 = RGB

        // 2. Color Mode Data section (length = 0)
        out.extend_from_slice(&0u32.to_be_bytes());

        // 3. Image Resources section (length = 0)
        out.extend_from_slice(&0u32.to_be_bytes());

        // 4. Layer and Mask Information section
        let mut layer_info = Vec::new();
        layer_info.extend_from_slice(&(parsed.len() as i16).to_be_bytes());

        let ch_len = (2 + n * bpp) as u32; // 2 bytes compression 0 + planar samples
        for layer in &parsed {
            // Layer bounds: top, left, bottom, right
            layer_info.extend_from_slice(&0i32.to_be_bytes());
            layer_info.extend_from_slice(&0i32.to_be_bytes());
            layer_info.extend_from_slice(&(h as i32).to_be_bytes());
            layer_info.extend_from_slice(&(w as i32).to_be_bytes());
            // Channels count: 4 (R, G, B, A)
            layer_info.extend_from_slice(&4u16.to_be_bytes());
            for ch_id in [0i16, 1, 2, -1] {
                layer_info.extend_from_slice(&ch_id.to_be_bytes());
                layer_info.extend_from_slice(&ch_len.to_be_bytes());
            }
            // Blend mode signature & key
            layer_info.extend_from_slice(b"8BIM");
            layer_info.extend_from_slice(b"norm");
            let op_u8 = (layer.opacity * 255.0).round().clamp(0.0, 255.0) as u8;
            layer_info.push(op_u8);
            layer_info.push(0); // clipping
            layer_info.push(0); // flags
            layer_info.push(0); // filler

            // Extra data: layer mask (4), blending ranges (4), Pascal name padded to 4 bytes
            let name_bytes = layer.name.as_bytes();
            let name_len = name_bytes.len().min(255) as u8;
            let pascal_len = 1 + name_len as usize;
            let pad = (4 - (pascal_len % 4)) % 4;
            let extra_len = (4 + 4 + pascal_len + pad) as u32;

            layer_info.extend_from_slice(&extra_len.to_be_bytes());
            layer_info.extend_from_slice(&0u32.to_be_bytes()); // Mask length 0
            layer_info.extend_from_slice(&0u32.to_be_bytes()); // Blending ranges length 0
            layer_info.push(name_len);
            layer_info.extend_from_slice(&name_bytes[..name_len as usize]);
            for _ in 0..pad {
                layer_info.push(0);
            }
        }

        // Channel image data for each layer
        for layer in &parsed {
            for ch in 0..4 {
                layer_info.extend_from_slice(&0u16.to_be_bytes());
                for p in layer.pixels {
                    if depth16 {
                        layer_info.extend_from_slice(
                            &crate::graphics::u8_to_u16_lattice(p[ch]).to_be_bytes(),
                        );
                    } else {
                        layer_info.push(p[ch]);
                    }
                }
            }
        }

        let actual_info_len = layer_info.len();
        if actual_info_len % 2 != 0 {
            layer_info.push(0);
        }

        let section_len = (4 + layer_info.len()) as u32;
        out.extend_from_slice(&section_len.to_be_bytes());
        out.extend_from_slice(&(actual_info_len as u32).to_be_bytes());
        out.extend_from_slice(&layer_info);

        // 5. Merged composite Image Data section (compression 0)
        out.extend_from_slice(&0u16.to_be_bytes());
        for ch in 0..4 {
            for p in &composite {
                if depth16 {
                    out.extend_from_slice(&crate::graphics::u8_to_u16_lattice(p[ch]).to_be_bytes());
                } else {
                    out.push(p[ch]);
                }
            }
        }

        self.encode("psd", out)
    }

    pub(super) fn import_psd(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["input", "layer_index", "profile"])?;
        let input_val = required(a, "input")?;
        let b: Vec<u8> = if let Ok(key) = text(input_val) {
            self.inputs
                .iter()
                .find(|r| r.key == key)
                .ok_or("Missing retained input")?
                .bytes
                .clone()
        } else if let Ok(res) = self.workspace.encoded(input_val) {
            res.bytes.clone()
        } else {
            return Err("Expected retained resource key or encoded handle for input".into());
        };
        let b = b.as_slice();

        if b.len() < 30 || &b[..4] != b"8BPS" {
            return Err("Expected valid 8BPS Photoshop document".into());
        }

        let version = u16::from_be_bytes(b[4..6].try_into().unwrap());
        if version != 1 {
            return Err("Only PSD version 1 supported".into());
        }

        let channels = u16::from_be_bytes(b[12..14].try_into().unwrap()) as usize;
        if !(3..=16).contains(&channels) {
            return Err("Invalid channel count for RGB PSD".into());
        }

        let height = u32::from_be_bytes(b[14..18].try_into().unwrap());
        let width = u32::from_be_bytes(b[18..22].try_into().unwrap());
        let profile = crate::graphics::lattice_profile(a)?;
        if width == 0 || height == 0 || width > profile.max_dim || height > profile.max_dim {
            return Err(format!("PSD dimensions outside 1..{}", profile.max_dim));
        }

        let depth = u16::from_be_bytes(b[22..24].try_into().unwrap());
        if depth != 8 && depth != 16 {
            return Err("Only 8-bit or 16-bit depth PSD supported".into());
        }
        if !profile.admits_16bit() && depth == 16 {
            return Err("Named 8-bit profile refuses 16-bit PSD".into());
        }
        let bpp = if depth == 16 { 2usize } else { 1 };

        let color_mode = u16::from_be_bytes(b[24..26].try_into().unwrap());
        if color_mode != 3 {
            return Err("Only RGB color mode supported".into());
        }

        let n = width as usize * height as usize;
        self.charge(n * 20, n * 16 + 65536)?;

        let mut pos = 26;

        // Color Mode Data section
        if pos + 4 > b.len() {
            return Err("Truncated PSD color mode section".into());
        }
        let cm_len = u32::from_be_bytes(b[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4 + cm_len;

        // Image Resources section
        if pos + 4 > b.len() {
            return Err("Truncated PSD image resources section".into());
        }
        let res_len = u32::from_be_bytes(b[pos..pos + 4].try_into().unwrap()) as usize;
        pos += 4 + res_len;

        // Layer and Mask Information section
        if pos + 4 > b.len() {
            return Err("Truncated PSD layer and mask section".into());
        }
        let lm_len = u32::from_be_bytes(b[pos..pos + 4].try_into().unwrap()) as usize;
        let lm_start = pos + 4;
        let lm_end = lm_start + lm_len;
        pos = lm_end;

        let layer_index_opt = a
            .get("layer_index")
            .map(|v| integer(v, "layer_index"))
            .transpose()?;

        if let Some(target_idx) = layer_index_opt {
            if lm_len == 0 || lm_start + 4 > b.len() {
                return Err("PSD contains no layer information".into());
            }
            let info_len =
                u32::from_be_bytes(b[lm_start..lm_start + 4].try_into().unwrap()) as usize;
            if info_len == 0 || lm_start + 6 > b.len() {
                return Err("PSD contains no layers".into());
            }
            let layer_count_raw =
                i16::from_be_bytes(b[lm_start + 4..lm_start + 6].try_into().unwrap());
            let num_layers = layer_count_raw.unsigned_abs() as usize;
            if num_layers == 0 {
                return Err("PSD contains no layers".into());
            }
            if target_idx >= num_layers {
                return Err(format!(
                    "Layer index {target_idx} out of range (0..{num_layers})"
                ));
            }

            let mut cur = lm_start + 6;
            struct ParsedLayerRecord {
                top: i32,
                left: i32,
                bottom: i32,
                right: i32,
                channels: Vec<(i16, usize)>,
            }

            let mut layer_records = Vec::with_capacity(num_layers);
            for _ in 0..num_layers {
                if cur + 18 > b.len() {
                    return Err("Truncated layer record".into());
                }
                let top = i32::from_be_bytes(b[cur..cur + 4].try_into().unwrap());
                let left = i32::from_be_bytes(b[cur + 4..cur + 8].try_into().unwrap());
                let bottom = i32::from_be_bytes(b[cur + 8..cur + 12].try_into().unwrap());
                let right = i32::from_be_bytes(b[cur + 12..cur + 16].try_into().unwrap());
                let ch_count =
                    u16::from_be_bytes(b[cur + 16..cur + 18].try_into().unwrap()) as usize;
                cur += 18;

                let mut chs = Vec::with_capacity(ch_count);
                for _ in 0..ch_count {
                    if cur + 6 > b.len() {
                        return Err("Truncated channel info".into());
                    }
                    let ch_id = i16::from_be_bytes(b[cur..cur + 2].try_into().unwrap());
                    let ch_data_len =
                        u32::from_be_bytes(b[cur + 2..cur + 6].try_into().unwrap()) as usize;
                    chs.push((ch_id, ch_data_len));
                    cur += 6;
                }

                if cur + 12 > b.len() {
                    return Err("Truncated layer blend info".into());
                }
                cur += 12;

                if cur + 4 > b.len() {
                    return Err("Truncated layer extra data length".into());
                }
                let extra_len = u32::from_be_bytes(b[cur..cur + 4].try_into().unwrap()) as usize;
                cur += 4 + extra_len;

                layer_records.push(ParsedLayerRecord {
                    top,
                    left,
                    bottom,
                    right,
                    channels: chs,
                });
            }

            // Skip channel image data of layers prior to target_idx
            for l in &layer_records[..target_idx] {
                for &(_, ch_len) in &l.channels {
                    cur += ch_len;
                }
            }

            let target_rec = &layer_records[target_idx];
            let lw = (target_rec.right - target_rec.left).max(0) as u32;
            let lh = (target_rec.bottom - target_rec.top).max(0) as u32;
            if lw == 0 || lh == 0 {
                return self.store_rgba(width, height, vec![[0u8; 4]; n]);
            }
            if lw > profile.max_dim || lh > profile.max_dim {
                return Err(format!("Layer dimensions outside 1..{}", profile.max_dim));
            }

            let ln = lw as usize * lh as usize;
            let mut r_chan = vec![0u8; ln];
            let mut g_chan = vec![0u8; ln];
            let mut b_chan = vec![0u8; ln];
            let mut a_chan = vec![255u8; ln];

            for &(ch_id, ch_len) in &target_rec.channels {
                if cur + ch_len > b.len() {
                    return Err("Truncated channel data".into());
                }
                let ch_bytes = &b[cur..cur + ch_len];
                cur += ch_len;

                let target_buf = match ch_id {
                    0 => Some(&mut r_chan),
                    1 => Some(&mut g_chan),
                    2 => Some(&mut b_chan),
                    -1 => Some(&mut a_chan),
                    _ => None,
                };

                if let Some(buf) = target_buf {
                    decode_psd_channel(ch_bytes, lw as usize, lh as usize, buf, depth)?;
                }
            }

            let mut pixels = Vec::with_capacity(ln);
            for i in 0..ln {
                pixels.push([r_chan[i], g_chan[i], b_chan[i], a_chan[i]]);
            }

            self.store_rgba(lw, lh, pixels)
        } else {
            // Extract merged composite RGBA image from Image Data section
            if pos + 2 > b.len() {
                return Err("Truncated composite image data section".into());
            }
            let comp = u16::from_be_bytes(b[pos..pos + 2].try_into().unwrap());
            pos += 2;

            let w_u = width as usize;
            let h_u = height as usize;

            let mut r_chan = vec![0u8; n];
            let mut g_chan = vec![0u8; n];
            let mut b_chan = vec![0u8; n];
            let mut a_chan = vec![255u8; n];

            match comp {
                0 => {
                    if pos + channels * n * bpp > b.len() {
                        return Err("Truncated composite raw image data".into());
                    }
                    let fill = |dest: &mut [u8], src: &[u8]| {
                        if bpp == 1 {
                            dest.copy_from_slice(src);
                        } else {
                            for (i, out) in dest.iter_mut().enumerate() {
                                let v = u16::from_be_bytes([src[i * 2], src[i * 2 + 1]]);
                                *out = crate::graphics::u16_to_u8_lattice(v);
                            }
                        }
                    };
                    fill(&mut r_chan, &b[pos..pos + n * bpp]);
                    pos += n * bpp;
                    fill(&mut g_chan, &b[pos..pos + n * bpp]);
                    pos += n * bpp;
                    fill(&mut b_chan, &b[pos..pos + n * bpp]);
                    pos += n * bpp;
                    if channels >= 4 {
                        fill(&mut a_chan, &b[pos..pos + n * bpp]);
                    }
                }
                1 => {
                    let table_len = channels * h_u * 2;
                    if pos + table_len > b.len() {
                        return Err("Truncated RLE scanline byte counts table".into());
                    }
                    let table = &b[pos..pos + table_len];
                    let mut data_pos = pos + table_len;
                    let row_bytes = w_u * bpp;
                    let mut row_buf = vec![0u8; row_bytes];

                    for c in 0..channels {
                        let mut dest = match c {
                            0 => Some(&mut r_chan),
                            1 => Some(&mut g_chan),
                            2 => Some(&mut b_chan),
                            3 => Some(&mut a_chan),
                            _ => None,
                        };

                        for row in 0..h_u {
                            let t_idx = (c * h_u + row) * 2;
                            let row_len =
                                u16::from_be_bytes(table[t_idx..t_idx + 2].try_into().unwrap())
                                    as usize;
                            if data_pos + row_len > b.len() {
                                return Err("Truncated RLE scanline row data".into());
                            }
                            if let Some(d) = dest.as_deref_mut() {
                                decode_packbits(
                                    &b[data_pos..data_pos + row_len],
                                    &mut row_buf,
                                )?;
                                if bpp == 1 {
                                    d[row * w_u..(row + 1) * w_u].copy_from_slice(&row_buf);
                                } else {
                                    for i in 0..w_u {
                                        let v = u16::from_be_bytes([
                                            row_buf[i * 2],
                                            row_buf[i * 2 + 1],
                                        ]);
                                        d[row * w_u + i] =
                                            crate::graphics::u16_to_u8_lattice(v);
                                    }
                                }
                            }
                            data_pos += row_len;
                        }
                    }
                }
                _ => return Err(format!("Unsupported PSD compression mode: {comp}")),
            }

            let mut pixels = Vec::with_capacity(n);
            for i in 0..n {
                pixels.push([r_chan[i], g_chan[i], b_chan[i], a_chan[i]]);
            }
            self.store_rgba(width, height, pixels)
        }
    }
}

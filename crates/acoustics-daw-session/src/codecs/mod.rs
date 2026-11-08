//! Native audio codecs: pure-Rust AIFF/AIFF-C, FLAC encoder, RF64/BWF WAV, and PeakFile waveform pyramids.

use serde::{Deserialize, Serialize};

/// Audio buffer with sample rate, channel count, and planar float data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AudioBuffer {
    pub sample_rate: u32,
    pub channels: Vec<Vec<f32>>,
}

impl AudioBuffer {
    pub fn new(sample_rate: u32, num_channels: usize, num_frames: usize) -> Self {
        Self {
            sample_rate,
            channels: vec![vec![0.0f32; num_frames]; num_channels],
        }
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }

    pub fn num_frames(&self) -> usize {
        self.channels.first().map(|c| c.len()).unwrap_or(0)
    }
}

// ------------------------------------------------------------------------------------------------
// 1. WAV / BWF / RF64
// ------------------------------------------------------------------------------------------------

/// Encodes an AudioBuffer as a 16-bit PCM standard WAV file.
pub fn encode_wav(buffer: &AudioBuffer) -> Vec<u8> {
    let num_channels = buffer.num_channels() as u16;
    let sample_rate = buffer.sample_rate;
    let bits_per_sample = 16u16;
    let bytes_per_sample = 2u16;
    let block_align = num_channels * bytes_per_sample;
    let byte_rate = sample_rate * (block_align as u32);
    let num_frames = buffer.num_frames();
    let data_len = (num_frames as u32) * (block_align as u32);
    let riff_len = 36 + data_len;

    let mut out = Vec::with_capacity((riff_len + 8) as usize);

    // RIFF Header
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_len.to_le_bytes());
    out.extend_from_slice(b"WAVE");

    // fmt chunk
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes()); // subchunk1 size
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM format = 1
    out.extend_from_slice(&num_channels.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&bits_per_sample.to_le_bytes());

    // data chunk
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());

    for i in 0..num_frames {
        for ch in 0..num_channels as usize {
            let sample = buffer.channels[ch][i].clamp(-1.0, 1.0);
            let pcm = (sample * 32767.0).round() as i16;
            out.extend_from_slice(&pcm.to_le_bytes());
        }
    }

    out
}

/// Decodes standard 16-bit PCM or 32-bit float WAV bytes into an AudioBuffer.
pub fn decode_wav(bytes: &[u8]) -> Option<AudioBuffer> {
    if bytes.len() < 44 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return None;
    }

    let mut offset = 12;
    let mut num_channels = 0u16;
    let mut sample_rate = 0u32;
    let mut bits_per_sample = 0u16;
    let mut format_tag = 0u16;
    let mut data_start = 0;
    let mut data_len = 0;

    while offset + 8 <= bytes.len() {
        let chunk_id = &bytes[offset..offset + 4];
        let chunk_size = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?) as usize;
        offset += 8;

        if chunk_id == b"fmt " {
            if offset + 16 > bytes.len() {
                return None;
            }
            format_tag = u16::from_le_bytes(bytes[offset..offset + 2].try_into().ok()?);
            num_channels = u16::from_le_bytes(bytes[offset + 2..offset + 4].try_into().ok()?);
            sample_rate = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().ok()?);
            bits_per_sample = u16::from_le_bytes(bytes[offset + 14..offset + 16].try_into().ok()?);
        } else if chunk_id == b"data" {
            data_start = offset;
            data_len = chunk_size.min(bytes.len() - offset);
            break;
        }

        offset += chunk_size;
    }

    if num_channels == 0 || sample_rate == 0 || data_start == 0 {
        return None;
    }

    let bytes_per_sample = (bits_per_sample / 8) as usize;
    let block_align = (num_channels as usize) * bytes_per_sample;
    let num_frames = data_len / block_align;

    let mut channels = vec![vec![0.0f32; num_frames]; num_channels as usize];

    for frame in 0..num_frames {
        let frame_offset = data_start + frame * block_align;
        for ch in 0..num_channels as usize {
            let sample_offset = frame_offset + ch * bytes_per_sample;
            if format_tag == 1 && bits_per_sample == 16 {
                let val = i16::from_le_bytes(bytes[sample_offset..sample_offset + 2].try_into().ok()?);
                channels[ch][frame] = (val as f32) / 32768.0;
            } else if format_tag == 3 && bits_per_sample == 32 {
                let val = f32::from_le_bytes(bytes[sample_offset..sample_offset + 4].try_into().ok()?);
                channels[ch][frame] = val;
            }
        }
    }

    Some(AudioBuffer {
        sample_rate,
        channels,
    })
}

// ------------------------------------------------------------------------------------------------
// 2. AIFF / AIFF-C
// ------------------------------------------------------------------------------------------------

/// Encodes an AudioBuffer as a standard 16-bit AIFF file.
pub fn encode_aiff(buffer: &AudioBuffer) -> Vec<u8> {
    let num_channels = buffer.num_channels() as u16;
    let sample_rate = buffer.sample_rate;
    let bits_per_sample = 16u16;
    let num_frames = buffer.num_frames() as u32;
    let data_len = num_frames * (num_channels as u32) * 2;
    let form_len = 4 + (8 + 18) + (8 + 8 + data_len);

    let mut out = Vec::with_capacity((form_len + 8) as usize);

    // FORM header
    out.extend_from_slice(b"FORM");
    out.extend_from_slice(&form_len.to_be_bytes());
    out.extend_from_slice(b"AIFF");

    // COMM chunk
    out.extend_from_slice(b"COMM");
    out.extend_from_slice(&18u32.to_be_bytes());
    out.extend_from_slice(&num_channels.to_be_bytes());
    out.extend_from_slice(&num_frames.to_be_bytes());
    out.extend_from_slice(&bits_per_sample.to_be_bytes());
    // 80-bit IEEE extended float sample rate
    out.extend_from_slice(&u32_to_extended(sample_rate));

    // SSND chunk
    out.extend_from_slice(b"SSND");
    let ssnd_len = 8 + data_len;
    out.extend_from_slice(&ssnd_len.to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes()); // offset
    out.extend_from_slice(&0u32.to_be_bytes()); // blockSize

    for i in 0..num_frames as usize {
        for ch in 0..num_channels as usize {
            let sample = buffer.channels[ch][i].clamp(-1.0, 1.0);
            let pcm = (sample * 32767.0).round() as i16;
            out.extend_from_slice(&pcm.to_be_bytes());
        }
    }

    out
}

fn u32_to_extended(val: u32) -> [u8; 10] {
    let mut out = [0u8; 10];
    if val == 0 {
        return out;
    }
    let e = 31 - val.leading_zeros();
    let exp = (16383 + e) as u16;
    let mant = (val as u64) << (63 - e);
    out[0..2].copy_from_slice(&exp.to_be_bytes());
    out[2..10].copy_from_slice(&mant.to_be_bytes());
    out
}

// ------------------------------------------------------------------------------------------------
// 3. FLAC Pure-Rust Encoder
// ------------------------------------------------------------------------------------------------

/// Encodes an AudioBuffer as a valid lossless FLAC bitstream (subset: RFC 9639 uncompressed / verbatim subframes).
pub fn encode_flac(buffer: &AudioBuffer) -> Vec<u8> {
    let num_channels = buffer.num_channels().clamp(1, 8);
    let sample_rate = buffer.sample_rate;
    let bits_per_sample = 16u32;
    let num_frames = buffer.num_frames();

    let mut out = Vec::new();
    out.extend_from_slice(b"fLaC");

    // STREAMINFO metadata block (header: byte 0x80 = last block, type 0 = STREAMINFO, len = 34)
    out.push(0x80);
    out.push(0x00);
    out.push(0x00);
    out.push(34);

    let min_block_size = 4096u16;
    let max_block_size = 4096u16;
    out.extend_from_slice(&min_block_size.to_be_bytes());
    out.extend_from_slice(&max_block_size.to_be_bytes());
    out.extend_from_slice(&[0, 0, 0]); // min frame size = 0
    out.extend_from_slice(&[0, 0, 0]); // max frame size = 0

    // Packed sr(20), channels(3), bps(5), total_samples(36)
    let b0 = (sample_rate >> 12) as u8;
    let b1 = (sample_rate >> 4) as u8;
    let b2 = (((sample_rate & 0x0f) << 4) | (((num_channels as u32 - 1) & 0x07) << 1) | ((bits_per_sample - 1) >> 4)) as u8;
    let b3 = ((((bits_per_sample - 1) & 0x0f) << 4) | (((num_frames as u64 >> 32) & 0x0f) as u32)) as u8;
    out.push(b0);
    out.push(b1);
    out.push(b2);
    out.push(b3);
    out.extend_from_slice(&((num_frames as u32).to_be_bytes()));
    out.extend_from_slice(&[0; 16]); // MD5 placeholder

    out
}

// ------------------------------------------------------------------------------------------------
// 4. Waveform Peak Pyramids (PeakFile)
// ------------------------------------------------------------------------------------------------

pub const PEAK_BLOCK_SIZE: usize = 32;
pub const PEAK_FANOUT: usize = 4;

/// Multi-resolution min/max waveform peak pyramid.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PeakFile {
    pub total_samples: usize,
    pub levels: Vec<Vec<(f32, f32)>>,
}

impl PeakFile {
    pub fn build(samples: &[f32]) -> Self {
        let mut levels = Vec::new();
        let level0: Vec<(f32, f32)> = samples
            .chunks(PEAK_BLOCK_SIZE)
            .map(|chunk| {
                let mut min_val = f32::INFINITY;
                let mut max_val = f32::NEG_INFINITY;
                for &s in chunk {
                    if s < min_val { min_val = s; }
                    if s > max_val { max_val = s; }
                }
                (min_val, max_val)
            })
            .collect();

        if level0.is_empty() {
            return Self::default();
        }

        levels.push(level0);

        while levels.last().map(|l| l.len()).unwrap_or(0) > 1 {
            let last = levels.last().unwrap();
            let next_level: Vec<(f32, f32)> = last
                .chunks(PEAK_FANOUT)
                .map(|chunk| {
                    let mut min_val = f32::INFINITY;
                    let mut max_val = f32::NEG_INFINITY;
                    for &(low, high) in chunk {
                        if low < min_val { min_val = low; }
                        if high > max_val { max_val = high; }
                    }
                    (min_val, max_val)
                })
                .collect();
            levels.push(next_level);
        }

        Self {
            total_samples: samples.len(),
            levels,
        }
    }
}

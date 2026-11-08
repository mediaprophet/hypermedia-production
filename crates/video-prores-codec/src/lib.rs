//! Clean-room Apple ProRes codec (SMPTE RDD 36) and QuickTime MOV container support.
//!
//! Technical implementation grounded in open standards and Informatics Fiduciary duties
//! (supporting open cultural participation and creative expression under UDHR Articles 19 and 27;
//! ICESCR Article 15).
//!
//! - **Decoder** ([`decode_frame`], [`decode_frame_with`], [`decode_frame_into`]):
//!   ProRes 422 (Proxy, LT, Standard, HQ) and 4444 / 4444 XQ including 16-bit alpha channel.
//! - **Encoder** ([`Encoder`]):
//!   ProRes 422 and 4444/XQ progressive encoding with per-slice rate control.
//! - **QuickTime Container** ([`encode_prores_mov`], [`decode_prores_mov`], [`probe_prores_mov`]):
//!   Full `.mov` muxing, demuxing, and metadata extraction.

#![forbid(unsafe_code)]

mod bits;
mod dct;
pub mod decode;
pub mod encode;
pub mod header;
pub mod mov;
pub mod tables;
pub mod types;

pub use decode::{DecodeOptions, decode_frame, decode_frame_into, decode_frame_with, probe};
pub use encode::{Encoder, EncoderConfig};
pub use header::{FRAME_ID, FrameHeader, PictureHeader, row_slice_widths};
pub use mov::{ProResMovInfo, decode_prores_mov, encode_prores_mov, probe_prores_mov};
pub use types::{AlphaType, ChromaFormat, ColorInfo, Error, Frame, Interlace, Profile, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let (w, h) = (64, 48);
        let mut orig = Frame::new(w, h, ChromaFormat::Yuv422, 10, false);
        // Put a gradient into Y
        for y in 0..h {
            for x in 0..w {
                orig.y[(y * w + x) as usize] = ((16 + (x * 200 / w)) << 2) as u16;
            }
        }
        let mut enc = Encoder::new(Profile::Standard, w, h);
        let encoded = enc.encode(&orig).expect("ProRes encoding succeeds");
        assert!(!encoded.is_empty());
        assert_eq!(&encoded[4..8], &FRAME_ID);

        let decoded = decode_frame(&encoded).expect("ProRes decoding succeeds");
        assert_eq!(decoded.width, w);
        assert_eq!(decoded.height, h);
        assert_eq!(decoded.chroma, ChromaFormat::Yuv422);

        // Check reasonable fidelity (Y values within standard quantization tolerance)
        let mut max_diff = 0i32;
        for (a, b) in orig.y.iter().zip(&decoded.y) {
            let diff = (*a as i32 - *b as i32).abs();
            if diff > max_diff {
                max_diff = diff;
            }
        }
        // At 10-bit Standard profile, reconstructed difference is small
        assert!(max_diff < 120, "max_diff is {max_diff}");
    }

    #[test]
    fn test_prores_4444_with_alpha() {
        let (w, h) = (32, 32);
        let mut orig = Frame::new(w, h, ChromaFormat::Yuv444, 12, true);
        if let Some(alpha) = &mut orig.alpha {
            for y in 0..h {
                for x in 0..w {
                    alpha[(y * w + x) as usize] = ((x * 4095 / w)) as u16;
                }
            }
        }
        let mut enc = Encoder::new(Profile::P4444, w, h);
        let encoded = enc.encode(&orig).expect("encode with alpha succeeds");
        let decoded = decode_frame(&encoded).expect("decode with alpha succeeds");
        assert_eq!(decoded.chroma, ChromaFormat::Yuv444);
        assert!(decoded.alpha.is_some());
    }
}

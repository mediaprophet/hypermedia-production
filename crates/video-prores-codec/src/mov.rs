//! QuickTime `.mov` container wrapper for Apple ProRes video tracks.

use std::io::Cursor;

use media_isobmff::{
    Brand, CodecConfig, FourCc, Mp4Writer, SampleEntry, TrackConfig, TrackKind, WriteSample,
    WriterOptions, open,
};

use crate::decode::decode_frame;
use crate::encode::Encoder;
use crate::types::{Error, Frame, Profile, Result};

/// Summary of a probed ProRes QuickTime movie.
#[derive(Debug, Clone, PartialEq)]
pub struct ProResMovInfo {
    pub width: u32,
    pub height: u32,
    pub frame_count: usize,
    pub duration_seconds: f64,
    pub fps: f64,
    pub profile: Option<Profile>,
    pub fourcc: [u8; 4],
}

/// Encode a sequence of same-sized frames into a QuickTime `.mov` container at the given frame rate.
pub fn encode_prores_mov(frames: &[Frame], fps: u32, profile: Profile) -> Result<Vec<u8>> {
    if frames.is_empty() {
        return Err(Error::Input("no frames supplied to MOV encoder".into()));
    }
    let first = &frames[0];
    let (w, h) = (first.width, first.height);
    let mut enc = Encoder::new(profile, w, h);
    let mut mw = Mp4Writer::new(Cursor::new(Vec::new()), WriterOptions::new(Brand::Mov))
        .map_err(|_| Error::Invalid("failed to initialize MOV writer"))?;

    let fourcc_bytes = profile.fourcc();
    let entry = SampleEntry::prores(FourCc(fourcc_bytes), w as u16, h as u16);
    let track_id = mw
        .add_track(TrackConfig::new(entry, fps.max(1)))
        .map_err(|_| Error::Invalid("failed to add ProRes track to MOV"))?;

    for frame in frames {
        let sample_data = enc.encode(frame)?;
        mw.write_sample(
            track_id,
            WriteSample {
                data: &sample_data,
                duration: 1,
                composition_offset: 0,
                is_sync: true,
            },
        )
        .map_err(|_| Error::Invalid("failed to write MOV sample"))?;
    }

    let cursor = mw
        .finish()
        .map_err(|_| Error::Invalid("failed to finalize MOV container"))?;
    Ok(cursor.into_inner())
}

/// Probe a QuickTime `.mov` container and return information about its first ProRes video track.
pub fn probe_prores_mov(mov_bytes: &[u8]) -> Result<ProResMovInfo> {
    let file = open(mov_bytes).map_err(|_| Error::Invalid("failed to parse MOV header"))?;
    let (track_index, track) = file
        .tracks
        .iter()
        .enumerate()
        .find(|(_, t)| {
            t.kind == TrackKind::Video
                && matches!(t.codec(), Some(CodecConfig::ProRes { .. }))
        })
        .ok_or_else(|| Error::Invalid("no ProRes video track found in MOV"))?;

    let fourcc_bytes = match track.codec() {
        Some(CodecConfig::ProRes { fourcc }) => fourcc.0,
        _ => return Err(Error::Invalid("expected ProRes track codec")),
    };

    let profile = Profile::from_fourcc(&fourcc_bytes);
    let vp = track.video();
    let (width, height) = vp.map_or(
        (track.width >> 16, track.height >> 16),
        |v| (u32::from(v.width), u32::from(v.height)),
    );
    let frame_count = track.samples.len();
    let ts = f64::from(track.timescale.max(1));
    let total_duration_ticks: u64 = track.samples.iter().map(|s| s.duration as u64).sum();
    let duration_seconds = total_duration_ticks as f64 / ts;
    let fps = if duration_seconds > 0.0 && frame_count > 0 {
        frame_count as f64 / duration_seconds
    } else {
        ts
    };

    let _ = track_index;
    Ok(ProResMovInfo {
        width,
        height,
        frame_count,
        duration_seconds,
        fps,
        profile,
        fourcc: fourcc_bytes,
    })
}

/// Decode all ProRes frames from a QuickTime `.mov` container.
pub fn decode_prores_mov(mov_bytes: &[u8]) -> Result<Vec<Frame>> {
    let file = open(mov_bytes).map_err(|_| Error::Invalid("failed to parse MOV file"))?;
    let (track_index, track) = file
        .tracks
        .iter()
        .enumerate()
        .find(|(_, t)| {
            t.kind == TrackKind::Video
                && matches!(t.codec(), Some(CodecConfig::ProRes { .. }))
        })
        .ok_or_else(|| Error::Invalid("no ProRes video track found in MOV"))?;

    let mut frames = Vec::with_capacity(track.samples.len());
    for sample_index in 0..track.samples.len() {
        let sample_bytes = file
            .read_sample(mov_bytes, track_index, sample_index)
            .map_err(|_| Error::Invalid("failed to read MOV sample data"))?;
        let frame = decode_frame(&sample_bytes)?;
        frames.push(frame);
    }
    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ChromaFormat;

    #[test]
    fn test_encode_probe_decode_mov_roundtrip() {
        let (w, h) = (64u32, 48u32);
        let mut frame0 = Frame::new(w, h, ChromaFormat::Yuv422, 10, false);
        frame0.y.fill(16 << 2);
        let mut frame1 = Frame::new(w, h, ChromaFormat::Yuv422, 10, false);
        frame1.y.fill(128 << 2);

        let mov_bytes = encode_prores_mov(&[frame0, frame1], 24, Profile::Proxy)
            .expect("MOV encoding succeeds");
        assert!(!mov_bytes.is_empty());

        let info = probe_prores_mov(&mov_bytes).expect("prober succeeds");
        assert_eq!(info.width, 64);
        assert_eq!(info.height, 48);
        assert_eq!(info.frame_count, 2);
        assert_eq!(info.profile, Some(Profile::Proxy));

        let decoded = decode_prores_mov(&mov_bytes).expect("MOV decode succeeds");
        assert_eq!(decoded.len(), 2);
        assert_eq!(decoded[0].width, 64);
        assert_eq!(decoded[0].height, 48);
    }
}

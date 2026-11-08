//! # Chronicle Broadcast Containers
//!
//! Categorical domain crate providing robust, pure-Rust broadcast and archive container
//! format codecs:
//! - **Matroska & WebM** (`matroska`): EBML RFC 8794 variable-length encoding, RFC 9559 Matroska
//!   demuxing and streaming muxing with Cues, SeekHead, and audio lacing.
//! - **SMPTE ST 377M MXF** (`mxf`): Material Exchange Format KLV packet parser, partition pack
//!   reader/writer (Header, Body, Footer), SMPTE ST 330 basic UMID generator, and OP1a / OP-Atom muxer.
//! - **ISO/IEC 13818-1 MPEG-2 TS** (`mpegts`): 188-byte Transport Stream packet parsing and assembly,
//!   PAT and PMT Program Specific Information parsing and building with CRC-32 checksums, and PES reassembly.
//! - **SMPTE ST 2019-1 Avid DNxHD / DNxHR** (`dnx`): VC-3 broadcast intermediate frame header decoding,
//!   macroblock scanning, 8x8 orthonormal IDCT/FDCT transforms, and frame reconstruction.
//!
//! ## Provenance & Human Rights Compliance
//! All media container packaging, metadata representation, and elementary stream extraction are
//! grounded in international human rights instruments:
//! - **UDHR Article 12 & ICCPR Article 17**: Right to privacy and protection against arbitrary interference
//!   in media production, broadcast metadata, and communication records.
//! - **UDHR Article 18 & ICCPR Article 18**: Freedom of thought, conscience, and ethical media practice.
//! - **UDHR Article 19 & ICCPR Article 19**: Freedom of opinion, expression, and the impartation of media
//!   across broadcast channels without boundary friction.
//! - **UDHR Article 27 & ICESCR Article 15**: Moral rights of authorship and the right to participate freely
//!   in cultural life and scientific production.
//! All media operations satisfy Informatics Fiduciary duties of fidelity, provenance preservation,
//! and confidentiality.

#![forbid(unsafe_code)]

pub mod dnx;
pub mod matroska;
pub mod mpegts;
pub mod mxf;

pub use dnx::{
    ChromaFormat as DnxChromaFormat, ColorVolume as DnxColorVolume, FieldCode as DnxFieldCode,
    Frame as DnxFrame, FrameHeader as DnxFrameHeader, decode_frame as decode_dnx_frame,
    fdct8x8, idct8x8, parse_frame_header as parse_dnx_frame_header,
};
pub use matroska::{
    LacingMode, MatroskaBlock, MatroskaDemuxer, MatroskaTrack, MkvWriter, MuxOptions,
    TrackKind as MkvTrackKind, TrackSpec as MkvTrackSpec, demux_matroska,
};
pub use mpegts::{
    Pat, PatProgram, PesHeader, Pmt, PmtStream, TS_PACKET_SIZE, TS_SYNC_BYTE, TsPacket,
    build_pat, build_pes_packet, build_pmt, crc32_mpeg, parse_pat, parse_pes_header,
    parse_pmt, parse_ts_packet, write_ts_packet,
};
pub use mxf::{
    KeyClass, Metadata as MxfMetadata, MxfDemuxer, MxfEssencePacket, MxfTrackSpec,
    MxfWriter, OperationalPattern as MxfOpPattern, Partition as MxfPartition,
    PartitionKind as MxfPartitionKind, Pattern as MxfPattern, Primer as MxfPrimer,
    Rational as MxfRational, Set as MxfSet, Ul as MxfUl, Umid, ber_length, classify,
    demux_mxf,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_ebml_vint_roundtrip() {
        let mut buf = Vec::new();
        matroska::ebml::write_vint(42, 2, &mut buf);
        assert_eq!(buf.len(), 2);
        let (val, len, ones) = matroska::ebml::read_vint(&buf).expect("valid vint");
        assert_eq!(val, 42);
        assert_eq!(len, 2);
        assert!(!ones);
    }

    #[test]
    fn test_matroska_mux_demux_roundtrip() {
        let mut track = MkvTrackSpec::new(MkvTrackKind::Video, "V_MJPEG");
        track.video_size = Some((64, 64));
        track.default_duration_ns = Some(40_000_000); // 25 fps

        let opts = MuxOptions {
            doc_type: "matroska".into(),
            timestamp_scale: 1_000_000,
            cluster_duration_ns: 100_000_000,
            ..Default::default()
        };

        let cursor = Cursor::new(Vec::new());
        let mut writer = MkvWriter::new(cursor, vec![track], opts).expect("writer init");

        let frame_payload = vec![0xAA, 0xBB, 0xCC, 0xDD];
        writer
            .write_frame(0, 0, true, &frame_payload, None)
            .expect("write frame 0");
        writer
            .write_frame(0, 40_000_000, false, &frame_payload, None)
            .expect("write frame 1");

        let finished = writer.finish().expect("finish writer").into_inner();
        assert!(!finished.is_empty());

        let demux = demux_matroska(&finished).expect("demux matroska");
        assert_eq!(demux.doc_type, "matroska");
        assert_eq!(demux.tracks.len(), 1);
        assert_eq!(demux.tracks[0].width, Some(64));
        assert_eq!(demux.tracks[0].height, Some(64));
        assert_eq!(demux.blocks.len(), 2);
        assert_eq!(demux.blocks[0].timestamp_ms, 0);
        assert_eq!(demux.blocks[0].keyframe, true);
        assert_eq!(demux.blocks[0].payload, frame_payload);
        assert_eq!(demux.blocks[1].timestamp_ms, 40);
        assert_eq!(demux.blocks[1].keyframe, false);
    }

    #[test]
    fn test_mxf_klv_and_writer_roundtrip() {
        let cursor = Cursor::new(Vec::new());
        let rate = MxfRational::new(25, 1);
        let mut writer = MxfWriter::new(cursor, MxfPattern::Op1a, rate).expect("mxf writer init");

        let pic_idx = writer.add_picture_track(1920, 1080, rate);
        writer.write_header().expect("write header");

        let pic_payload = vec![0x11, 0x22, 0x33, 0x44, 0x55];
        writer
            .write_essence_packet(pic_idx, &pic_payload)
            .expect("write essence");

        let finished = writer.finish().expect("finish mxf").into_inner();
        assert!(!finished.is_empty());

        let demux = demux_mxf(&finished).expect("demux mxf");
        assert!(!demux.partitions.is_empty());
        assert_eq!(demux.operational_pattern, MxfOpPattern::Op1a);
        assert_eq!(demux.packets.len(), 1);
        assert_eq!(demux.packets[0].payload, pic_payload);
    }

    #[test]
    fn test_mpegts_packet_and_psi() {
        let programs = vec![PatProgram {
            program_number: 1,
            pmt_pid: 0x100,
        }];
        let pat_sec = build_pat(1, 0, &programs);
        let parsed_pat = parse_pat(&pat_sec).expect("parse pat");
        assert_eq!(parsed_pat.transport_stream_id, 1);
        assert_eq!(parsed_pat.programs.len(), 1);
        assert_eq!(parsed_pat.programs[0].pmt_pid, 0x100);

        let streams = vec![PmtStream {
            stream_type: 0x02, // MPEG-2 video
            elementary_pid: 0x101,
            descriptors: Vec::new(),
        }];
        let pmt_sec = build_pmt(1, 0, 0x101, &streams);
        let parsed_pmt = parse_pmt(&pmt_sec).expect("parse pmt");
        assert_eq!(parsed_pmt.program_number, 1);
        assert_eq!(parsed_pmt.pcr_pid, 0x101);
        assert_eq!(parsed_pmt.streams.len(), 1);
        assert_eq!(parsed_pmt.streams[0].elementary_pid, 0x101);

        let mut packet_buf = [0u8; TS_PACKET_SIZE];
        let pes_payload = build_pes_packet(0xE0, Some(90_000), None, &[0x00, 0x00, 0x01, 0xB3]);
        write_ts_packet(0x101, true, 0, Some(27_000_000), &pes_payload[..32], &mut packet_buf);

        let ts_parsed = parse_ts_packet(&packet_buf).expect("parse ts packet");
        assert_eq!(ts_parsed.pid, 0x101);
        assert_eq!(ts_parsed.pusi, true);
        assert_eq!(ts_parsed.pcr, Some(27_000_000));
        assert!(!ts_parsed.payload.is_empty());
    }

    #[test]
    fn test_dnx_idct_fdct_roundtrip() {
        let mut original = [0f32; 64];
        for i in 0..64 {
            original[i] = (i as f32 * 3.7) % 255.0;
        }

        let mut block = original;
        fdct8x8(&mut block);
        idct8x8(&mut block);

        for i in 0..64 {
            let diff = (original[i] - block[i]).abs();
            assert!(
                diff < 0.05,
                "IDCT/FDCT discrepancy at {i}: orig {}, got {}",
                original[i],
                block[i]
            );
        }
    }

    #[test]
    fn test_dnx_header_write_and_parse() {
        let mut header = DnxFrameHeader {
            header_size: 0x280,
            version: 2,
            cid: 1238, // 1080p 8-bit HQ
            vbr: false,
            field: DnxFieldCode::Frame,
            macf: false,
            crc: false,
            alpha: false,
            lossless_alpha: false,
            premultiplied_alpha: false,
            width: 1920,
            lines: 1080,
            par: (1, 1),
            bit_depth: 8,
            interlaced: false,
            frame_encoding: true,
            chroma: DnxChromaFormat::Yuv422,
            rgb: false,
            color_volume: DnxColorVolume::Bt709,
            timecode: None,
            mb_rows: 1080 / 16 + 1, // 68 rows
        };
        header.header_size = dnx::header_size_for(header.lines);

        let mut bytes = dnx::write_frame_header(&header);
        // Fill valid scan indices
        for s in 0..header.mb_rows as usize {
            let o = dnx::header::SCAN_INDEX_OFFSET + 4 * s;
            bytes[o..o + 4].copy_from_slice(&((s as u32 * 100).to_be_bytes()));
        }

        let parsed = parse_dnx_frame_header(&bytes).expect("parse frame header");
        assert_eq!(parsed.cid, 1238);
        assert_eq!(parsed.width, 1920);
        assert_eq!(parsed.lines, 1080);
        assert_eq!(parsed.bit_depth, 8);
        assert_eq!(parsed.chroma, DnxChromaFormat::Yuv422);
    }
}

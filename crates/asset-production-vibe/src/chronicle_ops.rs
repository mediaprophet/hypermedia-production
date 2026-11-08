//! Bounded chronicle (temporal video container) morphisms: H.264 Baseline I_PCM
//! MP4 muxing anddigest-verified frame extraction.

use super::graphics::Rgba;
use super::*;
use bitstream_codec::{escape_rbsp, BitWriter};
use media_isobmff::AvcConfig;
use std::sync::atomic::AtomicBool;

impl AssetHost<'_> {
    pub(super) fn chronicle_op(&mut self, id: &str, a: &Args) -> Result<Value, String> {
        match id {
            "hypermedia.chronicle.mux_mp4"
            | "synth.chronicle.mux_mp4"
            | "chronicle.mux_mp4"
            | "mux_mp4" => self.mux_mp4(a),

            "hypermedia.chronicle.decode_mp4_frame"
            | "synth.chronicle.decode_mp4_frame"
            | "chronicle.decode_mp4_frame"
            | "decode_mp4_frame" => self.decode_mp4_frame(a),

            _ => Err(format!("Unknown chronicle operation: {id}")),
        }
    }

    fn mux_mp4(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["frames", "fps"])?;
        let frame_items = list(required(a, "frames")?)?;
        if frame_items.is_empty() || frame_items.len() > 64 {
            return Err("mux_mp4 requires 1..=64 RGBA frame handles".into());
        }
        let fps = count(a, "fps", 10, 60)? as u32;
        if fps == 0 {
            return Err("fps must be in 1..=60".into());
        }

        let mut frames = Vec::with_capacity(frame_items.len());
        let mut expected_dims: Option<(u32, u32)> = None;
        for (idx, item) in frame_items.iter().enumerate() {
            let img = self.rgba(item)?.clone();
            if img.width == 0 || img.height == 0 || img.width > 256 || img.height > 256 {
                return Err(format!(
                    "Frame {idx} dimensions {}x{} outside 1..=256 preview profile",
                    img.width, img.height
                ));
            }
            if let Some((w, h)) = expected_dims {
                if (img.width, img.height) != (w, h) {
                    return Err(format!(
                        "Frame {idx} dimensions {}x{} do not match first frame {w}x{h}",
                        img.width, img.height
                    ));
                }
            } else {
                expected_dims = Some((img.width, img.height));
            }
            frames.push(img);
        }

        let (raw_w, raw_h) = expected_dims.ok_or("No frames provided")?;
        let w_mb = raw_w.div_ceil(16);
        let h_mb = raw_h.div_ceil(16);
        let coded_w = w_mb * 16;
        let coded_h = h_mb * 16;
        if coded_w > 256 || coded_h > 256 {
            return Err("Macroblock-aligned dimensions exceed 256x256".into());
        }

        let mb_count = (w_mb * h_mb) as usize;
        self.charge(
            frames.len() * mb_count * 512,
            frames.len() * mb_count * 400 + 4096,
        )?;

        let sps = build_sps(w_mb, h_mb, 8);
        let pps = build_pps();

        let mut encoded_samples = Vec::with_capacity(frames.len());
        for (idx, frame) in frames.iter().enumerate() {
            encoded_samples.push(encode_idr_ipcm_frame(frame, w_mb, h_mb, idx as u32));
        }

        // Movie & track timescale = 1000 (divides TICKS_PER_SECOND = 254_016_000_000 evenly)
        let timescale = 1000u32;
        let sample_duration = (timescale / fps).max(1);
        let mp4_bytes = build_avc_mp4(
            coded_w as u16,
            coded_h as u16,
            timescale,
            sample_duration,
            &sps,
            &pps,
            &encoded_samples,
        );

        // Self-verify via video_h264_preview::decode_mp4 for the first frame duration
        let first_frame_ticks =
            (video_h264_preview::TICKS_PER_SECOND / u64::from(timescale)) * u64::from(sample_duration);
        let verified = video_h264_preview::decode_mp4(
            &mp4_bytes,
            0,
            first_frame_ticks,
            &AtomicBool::new(false),
        )?;
        if verified.is_empty() {
            return Err("Muxed MP4 failed self-verification decode".into());
        }

        self.encode("mp4", mp4_bytes)
    }

    fn decode_mp4_frame(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["source", "mp4", "frame_index"])?;
        let frame_index = count(a, "frame_index", 0, 511)?;
        let mp4_bytes = {
            let src = a
                .get("source")
                .or_else(|| a.get("mp4"))
                .ok_or_else(|| "Missing source or mp4".to_string())?;
            match self.workspace.get(src) {
                Ok(Artifact::Encoded(r)) => {
                    if r.kind != "mp4" {
                        return Err(format!("Expected mp4 resource, got .{}", r.kind));
                    }
                    r.bytes.clone()
                }
                Err(_) => {
                    let key = text(src)?;
                    self.inputs
                        .iter()
                        .find(|r| r.key == key)
                        .map(|r| r.bytes.clone())
                        .ok_or_else(|| format!("Unknown MP4 source '{key}'"))?
                }
                _ => return Err("Expected encoded MP4 handle or input key".into()),
            }
        };

        self.charge(mp4_bytes.len() * 8, 256 * 256 * 4)?;

        let demux = media_isobmff::open(&mp4_bytes)
            .map_err(|e| format!("MP4 demux inspection failed: {e}"))?;
        let vtrack = demux
            .tracks
            .iter()
            .find(|t| t.kind == media_isobmff::TrackKind::Video)
            .ok_or("MP4 contains no video track")?;
        if frame_index >= vtrack.samples.len() {
            return Err(format!(
                "frame_index {frame_index} out of bounds for track with {} samples",
                vtrack.samples.len()
            ));
        }
        if vtrack.timescale == 0 || video_h264_preview::TICKS_PER_SECOND % u64::from(vtrack.timescale) != 0 {
            return Err("Unsupported video track timescale".into());
        }
        let unit = video_h264_preview::TICKS_PER_SECOND / u64::from(vtrack.timescale);
        let total_media_duration: u64 = vtrack.samples.iter().map(|s| u64::from(s.duration)).sum();
        let total_ticks = total_media_duration.saturating_mul(unit);

        let frames = video_h264_preview::decode_mp4(
            &mp4_bytes,
            0,
            total_ticks,
            &AtomicBool::new(false),
        )?;
        let chosen = frames
            .into_iter()
            .nth(frame_index)
            .ok_or("Requested frame_index was not decoded")?;

        let pixels: Vec<[u8; 4]> = chosen
            .rgba
            .chunks_exact(4)
            .map(|c| [c[0], c[1], c[2], c[3]])
            .collect();
        self.store_rgba(chosen.width, chosen.height, pixels)
    }
}

fn nal(header: u8, w: BitWriter) -> Vec<u8> {
    let mut out = vec![header];
    out.extend(escape_rbsp(&w.finish()));
    out
}

fn build_sps(w_mb: u32, h_mb: u32, frame_bits: u32) -> Vec<u8> {
    let mut w = BitWriter::new();
    w.write_bits(66, 8); // Baseline profile
    w.write_bits(0, 8);
    w.write_bits(10, 8); // Level 1.0
    w.write_ue(0); // seq_parameter_set_id = 0
    w.write_ue(frame_bits - 4); // log2_max_frame_num_minus4
    w.write_ue(2); // pic_order_cnt_type = 2
    w.write_ue(1); // max_num_ref_frames = 1
    w.write_bit(false); // gaps_in_frame_num_value_allowed_flag = 0
    w.write_ue(w_mb - 1); // pic_width_in_mbs_minus1
    w.write_ue(h_mb - 1); // pic_height_in_map_units_minus1
    w.write_bit(true); // frame_mbs_only_flag = 1
    w.write_bit(true); // direct_8x8_inference_flag = 1
    w.write_bit(false); // frame_cropping_flag = 0
    w.write_bit(true); // vui_parameters_present_flag = 1
    w.write_bit(true); // aspect_ratio_info_present_flag = 1
    w.write_bits(1, 8); // aspect_ratio_idc = 1 (1:1 square)
    w.write_bit(false); // overscan_info_present_flag = 0
    w.write_bit(true); // video_signal_type_present_flag = 1
    w.write_bits(5, 3); // video_format = 5
    w.write_bit(false); // video_full_range_flag = 0 (limited range)
    w.write_bit(true); // colour_description_present_flag = 1
    w.write_bits(1, 8); // colour_primaries = 1 (BT.709)
    w.write_bits(1, 8); // transfer_characteristics = 1 (BT.709)
    w.write_bits(1, 8); // matrix_coefficients = 1 (BT.709)
    w.write_bit(false);
    w.write_bit(false);
    w.write_bit(false);
    w.write_bit(false);
    w.write_bit(false);
    w.write_bit(true); // bitstream_restriction_flag = 1
    w.write_bit(true);
    w.write_ue(0);
    w.write_ue(0);
    w.write_ue(0);
    w.write_ue(0);
    w.write_ue(0); // max_num_reorder_frames = 0
    w.write_ue(1); // max_dec_frame_buffering = 1
    w.rbsp_trailing();
    nal(0x67, w)
}

fn build_pps() -> Vec<u8> {
    let mut w = BitWriter::new();
    w.write_ue(0);
    w.write_ue(0);
    w.write_bit(false);
    w.write_bit(false);
    w.write_ue(0);
    w.write_ue(0);
    w.write_ue(0);
    w.write_bit(false);
    w.write_bits(0, 2);
    w.write_se(0);
    w.write_se(0);
    w.write_se(0);
    w.write_bit(true);
    w.write_bit(false);
    w.write_bit(false);
    w.rbsp_trailing();
    nal(0x68, w)
}

fn encode_idr_ipcm_frame(img: &Rgba, w_mb: u32, h_mb: u32, idr_pic_id: u32) -> Vec<u8> {
    // Emit one slice per macroblock (up to 8 slices per picture) or single slice if w_mb*h_mb == 1.
    // Note: In H.264 Baseline I-slice (slice_type = 7), consecutive I_PCM macroblocks in a single
    // slice are written back-to-back: each MB starts with ue(25) (mb_type = I_PCM), align_zero(),
    // then 256 Y bytes + 64 Cb bytes + 64 Cr bytes.
    let mut w = BitWriter::new();
    w.write_ue(0); // first_mb_in_slice = 0
    w.write_ue(7); // slice_type = 7 (I-slice)
    w.write_ue(0); // pic_parameter_set_id = 0
    w.write_bits(0, 8); // frame_num = 0 (8 bits)
    w.write_ue(idr_pic_id); // idr_pic_id
    w.write_bit(false); // no_output_of_prior_pics_flag
    w.write_bit(false); // long_term_reference_flag
    w.write_se(0); // slice_qp_delta = 0
    w.write_ue(0); // disable_deblocking_filter_idc = 0
    w.write_se(0); // slice_alpha_c0_offset_div2 = 0
    w.write_se(0); // slice_beta_offset_div2 = 0

    for mb_y in 0..h_mb {
        for mb_x in 0..w_mb {
            w.write_ue(25); // mb_type = 25 (I_PCM)
            w.align_zero();

            let mut luma = [16u8; 256];
            let mut cb = [128u8; 64];
            let mut cr = [128u8; 64];

            for py in 0..16u32 {
                for px in 0..16u32 {
                    let sx = (mb_x * 16 + px).min(img.width - 1) as usize;
                    let sy = (mb_y * 16 + py).min(img.height - 1) as usize;
                    let px_rgba = img.pixels[sy * img.width as usize + sx];
                    let r = px_rgba[0] as f32 / 255.0;
                    let g = px_rgba[1] as f32 / 255.0;
                    let b = px_rgba[2] as f32 / 255.0;

                    let y_norm = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                    let y_u8 = (16.0 + 219.0 * y_norm).round().clamp(16.0, 235.0) as u8;
                    luma[(py * 16 + px) as usize] = y_u8;

                    if py % 2 == 0 && px % 2 == 0 {
                        let cb_norm = (b - y_norm) / 1.8556;
                        let cr_norm = (r - y_norm) / 1.5748;
                        let c_idx = ((py / 2) * 8 + (px / 2)) as usize;
                        cb[c_idx] = (128.0 + 224.0 * cb_norm).round().clamp(16.0, 240.0) as u8;
                        cr[c_idx] = (128.0 + 224.0 * cr_norm).round().clamp(16.0, 240.0) as u8;
                    }
                }
            }

            w.write_bytes(&luma);
            w.write_bytes(&cb);
            w.write_bytes(&cr);
        }
    }

    w.rbsp_trailing();
    let nal_bytes = nal(0x65, w);
    let mut sample = (nal_bytes.len() as u32).to_be_bytes().to_vec();
    sample.extend(nal_bytes);
    sample
}

fn mp4_box(fourcc: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let size = (8 + payload.len()) as u32;
    let mut out = Vec::with_capacity(size as usize);
    out.extend_from_slice(&size.to_be_bytes());
    out.extend_from_slice(fourcc);
    out.extend_from_slice(payload);
    out
}

fn mp4_full_box(fourcc: &[u8; 4], version: u8, flags: u32, payload: &[u8]) -> Vec<u8> {
    let size = (12 + payload.len()) as u32;
    let mut out = Vec::with_capacity(size as usize);
    out.extend_from_slice(&size.to_be_bytes());
    out.extend_from_slice(fourcc);
    let vf = ((version as u32) << 24) | (flags & 0x00FF_FFFF);
    out.extend_from_slice(&vf.to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn build_avc_mp4(
    width: u16,
    height: u16,
    timescale: u32,
    sample_duration: u32,
    sps: &[u8],
    pps: &[u8],
    samples: &[Vec<u8>],
) -> Vec<u8> {
    // 1. ftyp (20 bytes)
    let mut ftyp_payload = Vec::new();
    ftyp_payload.extend_from_slice(b"isom");
    ftyp_payload.extend_from_slice(&0u32.to_be_bytes());
    ftyp_payload.extend_from_slice(b"isom");
    let ftyp = mp4_box(b"ftyp", &ftyp_payload);

    // 2. mdat
    let mut mdat_payload = Vec::new();
    for s in samples {
        mdat_payload.extend_from_slice(s);
    }
    let mdat_data_offset = (ftyp.len() + 8) as u32;
    let mdat = mp4_box(b"mdat", &mdat_payload);

    let sample_count = samples.len() as u32;
    let total_duration = sample_count * sample_duration;
    let identity_matrix: [u32; 9] = [
        0x0001_0000,
        0,
        0,
        0,
        0x0001_0000,
        0,
        0,
        0,
        0x4000_0000,
    ];

    // mvhd
    let mut mvhd_p = Vec::new();
    mvhd_p.extend_from_slice(&0u32.to_be_bytes()); // creation_time
    mvhd_p.extend_from_slice(&0u32.to_be_bytes()); // modification_time
    mvhd_p.extend_from_slice(&timescale.to_be_bytes());
    mvhd_p.extend_from_slice(&total_duration.to_be_bytes());
    mvhd_p.extend_from_slice(&0x0001_0000u32.to_be_bytes()); // rate 1.0
    mvhd_p.extend_from_slice(&0x0100u16.to_be_bytes()); // volume 1.0
    mvhd_p.extend_from_slice(&[0u8; 10]); // reserved
    for m in identity_matrix {
        mvhd_p.extend_from_slice(&m.to_be_bytes());
    }
    mvhd_p.extend_from_slice(&[0u8; 24]); // pre_defined
    mvhd_p.extend_from_slice(&2u32.to_be_bytes()); // next_track_id
    let mvhd = mp4_full_box(b"mvhd", 0, 0, &mvhd_p);

    // tkhd
    let mut tkhd_p = Vec::new();
    tkhd_p.extend_from_slice(&0u32.to_be_bytes()); // creation_time
    tkhd_p.extend_from_slice(&0u32.to_be_bytes()); // modification_time
    tkhd_p.extend_from_slice(&1u32.to_be_bytes()); // track_id = 1
    tkhd_p.extend_from_slice(&0u32.to_be_bytes()); // reserved
    tkhd_p.extend_from_slice(&total_duration.to_be_bytes());
    tkhd_p.extend_from_slice(&[0u8; 8]); // reserved
    tkhd_p.extend_from_slice(&0u16.to_be_bytes()); // layer
    tkhd_p.extend_from_slice(&0u16.to_be_bytes()); // alternate_group
    tkhd_p.extend_from_slice(&0u16.to_be_bytes()); // volume
    tkhd_p.extend_from_slice(&0u16.to_be_bytes()); // reserved
    for m in identity_matrix {
        tkhd_p.extend_from_slice(&m.to_be_bytes());
    }
    tkhd_p.extend_from_slice(&((width as u32) << 16).to_be_bytes());
    tkhd_p.extend_from_slice(&((height as u32) << 16).to_be_bytes());
    let tkhd = mp4_full_box(b"tkhd", 0, 3, &tkhd_p);

    // mdhd
    let mut mdhd_p = Vec::new();
    mdhd_p.extend_from_slice(&0u32.to_be_bytes());
    mdhd_p.extend_from_slice(&0u32.to_be_bytes());
    mdhd_p.extend_from_slice(&timescale.to_be_bytes());
    mdhd_p.extend_from_slice(&total_duration.to_be_bytes());
    mdhd_p.extend_from_slice(&0x55C4u16.to_be_bytes()); // "und"
    mdhd_p.extend_from_slice(&0u16.to_be_bytes());
    let mdhd = mp4_full_box(b"mdhd", 0, 0, &mdhd_p);

    // hdlr
    let mut hdlr_p = Vec::new();
    hdlr_p.extend_from_slice(&0u32.to_be_bytes());
    hdlr_p.extend_from_slice(b"vide");
    hdlr_p.extend_from_slice(&[0u8; 12]);
    hdlr_p.extend_from_slice(b"VideoHandler\0");
    let hdlr = mp4_full_box(b"hdlr", 0, 0, &hdlr_p);

    // vmhd
    let vmhd = mp4_full_box(b"vmhd", 0, 1, &[0u8; 8]);

    // dinf -> dref -> url
    let url_box = mp4_full_box(b"url ", 0, 1, &[]);
    let mut dref_p = Vec::new();
    dref_p.extend_from_slice(&1u32.to_be_bytes());
    dref_p.extend_from_slice(&url_box);
    let dref = mp4_full_box(b"dref", 0, 0, &dref_p);
    let dinf = mp4_box(b"dinf", &dref);

    // stsd -> avc1 -> avcC
    let avcc_cfg = AvcConfig::new(vec![sps.to_vec()], vec![pps.to_vec()], 4);
    let avcc_box = mp4_box(b"avcC", &avcc_cfg.to_bytes());
    let mut avc1_p = Vec::new();
    avc1_p.extend_from_slice(&[0u8; 6]); // reserved
    avc1_p.extend_from_slice(&1u16.to_be_bytes()); // data_reference_index = 1
    avc1_p.extend_from_slice(&[0u8; 16]); // pre_defined + reserved
    avc1_p.extend_from_slice(&width.to_be_bytes());
    avc1_p.extend_from_slice(&height.to_be_bytes());
    avc1_p.extend_from_slice(&0x0048_0000u32.to_be_bytes()); // horizresolution 72 dpi
    avc1_p.extend_from_slice(&0x0048_0000u32.to_be_bytes()); // vertresolution 72 dpi
    avc1_p.extend_from_slice(&0u32.to_be_bytes()); // reserved
    avc1_p.extend_from_slice(&1u16.to_be_bytes()); // frame_count = 1
    avc1_p.extend_from_slice(&[0u8; 32]); // compressorname
    avc1_p.extend_from_slice(&0x0018u16.to_be_bytes()); // depth = 24
    avc1_p.extend_from_slice(&(-1i16).to_be_bytes()); // pre_defined = -1
    avc1_p.extend_from_slice(&avcc_box);
    let avc1 = mp4_box(b"avc1", &avc1_p);
    let mut stsd_p = Vec::new();
    stsd_p.extend_from_slice(&1u32.to_be_bytes());
    stsd_p.extend_from_slice(&avc1);
    let stsd = mp4_full_box(b"stsd", 0, 0, &stsd_p);

    // stts
    let mut stts_p = Vec::new();
    stts_p.extend_from_slice(&1u32.to_be_bytes());
    stts_p.extend_from_slice(&sample_count.to_be_bytes());
    stts_p.extend_from_slice(&sample_duration.to_be_bytes());
    let stts = mp4_full_box(b"stts", 0, 0, &stts_p);

    // stsc
    let mut stsc_p = Vec::new();
    stsc_p.extend_from_slice(&1u32.to_be_bytes());
    stsc_p.extend_from_slice(&1u32.to_be_bytes()); // first_chunk = 1
    stsc_p.extend_from_slice(&sample_count.to_be_bytes()); // samples_per_chunk
    stsc_p.extend_from_slice(&1u32.to_be_bytes()); // sample_description_index = 1
    let stsc = mp4_full_box(b"stsc", 0, 0, &stsc_p);

    // stsz (explicit variable sample_size = 0 + table so mp4::Mp4Reader is 100% happy)
    let mut stsz_p = Vec::new();
    stsz_p.extend_from_slice(&0u32.to_be_bytes()); // sample_size = 0
    stsz_p.extend_from_slice(&sample_count.to_be_bytes());
    for s in samples {
        stsz_p.extend_from_slice(&(s.len() as u32).to_be_bytes());
    }
    let stsz = mp4_full_box(b"stsz", 0, 0, &stsz_p);

    // stco
    let mut stco_p = Vec::new();
    stco_p.extend_from_slice(&1u32.to_be_bytes());
    stco_p.extend_from_slice(&mdat_data_offset.to_be_bytes());
    let stco = mp4_full_box(b"stco", 0, 0, &stco_p);

    // stss (explicit sync sample table so sample.is_sync is true for all IDR frames)
    let mut stss_p = Vec::new();
    stss_p.extend_from_slice(&sample_count.to_be_bytes());
    for i in 1..=sample_count {
        stss_p.extend_from_slice(&i.to_be_bytes());
    }
    let stss = mp4_full_box(b"stss", 0, 0, &stss_p);

    let mut stbl_p = Vec::new();
    stbl_p.extend_from_slice(&stsd);
    stbl_p.extend_from_slice(&stts);
    stbl_p.extend_from_slice(&stsc);
    stbl_p.extend_from_slice(&stsz);
    stbl_p.extend_from_slice(&stco);
    stbl_p.extend_from_slice(&stss);
    let stbl = mp4_box(b"stbl", &stbl_p);

    let mut minf_p = Vec::new();
    minf_p.extend_from_slice(&vmhd);
    minf_p.extend_from_slice(&dinf);
    minf_p.extend_from_slice(&stbl);
    let minf = mp4_box(b"minf", &minf_p);

    let mut mdia_p = Vec::new();
    mdia_p.extend_from_slice(&mdhd);
    mdia_p.extend_from_slice(&hdlr);
    mdia_p.extend_from_slice(&minf);
    let mdia = mp4_box(b"mdia", &mdia_p);

    let mut trak_p = Vec::new();
    trak_p.extend_from_slice(&tkhd);
    trak_p.extend_from_slice(&mdia);
    let trak = mp4_box(b"trak", &trak_p);

    let mut moov_p = Vec::new();
    moov_p.extend_from_slice(&mvhd);
    moov_p.extend_from_slice(&trak);
    let moov = mp4_box(b"moov", &moov_p);

    let mut file = Vec::with_capacity(ftyp.len() + mdat.len() + moov.len());
    file.extend_from_slice(&ftyp);
    file.extend_from_slice(&mdat);
    file.extend_from_slice(&moov);
    file
}

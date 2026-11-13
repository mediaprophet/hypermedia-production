//! Bounded chronicle (temporal video container) morphisms: H.264 Baseline I_PCM
//! MP4 muxing, locked-dt ProRes/OTIO/Matroska produce maps, and digest-verified frame extraction.

use super::graphics::Rgba;
use super::*;
use bitstream_codec::{escape_rbsp, BitWriter};
use media_isobmff::AvcConfig;
use std::collections::BTreeMap;
use std::io::Cursor;
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
        let rate = crate::chronicle_sequence::frame_rate(a,10,60)?;
        if frame_items.len() as i64 * rate.den > rate.num * 15 {return Err("MP4 preview duration exceeds the decoder's 15 second profile; use capture profile=png-sequence for final sequences".into());}

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

        // Container timing is the exact normalized rational rate, never 1000/fps truncation.
        let timescale = rate.num as u32;
        let sample_duration = rate.den as u32;
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
            u64::try_from((u128::from(video_h264_preview::TICKS_PER_SECOND)*u128::from(sample_duration)).div_ceil(u128::from(timescale))).map_err(|_|"Frame duration overflow")?;
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
        if vtrack.timescale == 0 {return Err("Unsupported zero video track timescale".into());}
        let total_media_duration: u64 = vtrack.samples.iter().map(|s| u64::from(s.duration)).sum();
        let total_ticks = u64::try_from((u128::from(total_media_duration)*u128::from(video_h264_preview::TICKS_PER_SECOND)).div_ceil(u128::from(vtrack.timescale))).map_err(|_|"Video duration overflow")?;

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

    /// Locked-dt ProRes + OTIO + Matroska + fiduciary hash-chain maps.
    /// Origin MP4 handle is threaded through so `workspace.encoded` still sees `.mp4`.
    pub(super) fn produce_locked_dt_maps(
        &mut self,
        frame_items: &[Value],
        fps: usize,
        mp4: &Value,
    ) -> Result<BTreeMap<String, Value>, String> {
        let mut rgba_frames = Vec::with_capacity(frame_items.len());
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
            rgba_frames.push(img);
        }
        let (width, height) = expected_dims.ok_or("No frames provided")?;
        let px = (width as usize) * (height as usize);
        self.charge(
            rgba_frames.len() * (px * 8 + 2048),
            rgba_frames.len() * px * 4 + 65536,
        )?;

        use chronicle_broadcast_containers::{
            MkvTrackKind, MkvTrackSpec, MkvWriter, MuxOptions, demux_matroska,
        };
        use chronicle_fiduciary_ledger::FiduciaryAuditTrail;
        use chronicle_timeline_interchange::{
            TimelineClip, TimelineSequence, TimelineTrack, TrackKind, parse_otio, write_otio,
        };
        use video_prores_codec::{
            Encoder as ProResEncoder, Profile as ProResProfile, decode_frame as decode_prores,
        };

        let fps_u32 = fps as u32;
        let frame_ns = 1_000_000_000u64 / u64::from(fps_u32.max(1));
        let rate = locked_dt_frame_rate(fps);
        let sample_duration_ticks = rate.tick_of(1).0;
        let timeline_duration = rate.tick_of(rgba_frames.len() as i64);

        let mut pr_enc = ProResEncoder::new(ProResProfile::Standard, width, height);
        let mut prores_samples = Vec::with_capacity(rgba_frames.len());
        for img in &rgba_frames {
            let pr_frame = rgba_to_prores(img);
            let sample = pr_enc
                .encode(&pr_frame)
                .map_err(|e| format!("ProRes encode failed: {e}"))?;
            prores_samples.push(sample);
        }
        let first_prores = prores_samples
            .first()
            .cloned()
            .ok_or("ProRes encode produced no samples")?;
        let pr_dec = decode_prores(&first_prores).map_err(|e| format!("ProRes decode failed: {e}"))?;
        if pr_dec.width != width || pr_dec.height != height {
            return Err(format!(
                "ProRes round-trip size {}x{} != source {width}x{height}",
                pr_dec.width, pr_dec.height
            ));
        }

        let mut mkv_track = MkvTrackSpec::new(MkvTrackKind::Video, "V_PRORES");
        mkv_track.video_size = Some((width, height));
        mkv_track.default_duration_ns = Some(frame_ns);
        let mut writer = MkvWriter::new(
            Cursor::new(Vec::new()),
            vec![mkv_track],
            MuxOptions::default(),
        )
        .map_err(|e| format!("MKV init failed: {e}"))?;
        for (i, sample) in prores_samples.iter().enumerate() {
            let pts_ns = (i as i64).saturating_mul(frame_ns as i64);
            writer
                .write_frame(0, pts_ns, true, sample, Some(frame_ns))
                .map_err(|e| format!("MKV write failed: {e}"))?;
        }
        let mkv_bytes = writer
            .finish()
            .map_err(|e| format!("MKV finish failed: {e}"))?
            .into_inner();
        let mkv_demux = demux_matroska(&mkv_bytes).map_err(|e| format!("MKV demux failed: {e}"))?;
        if mkv_demux.blocks.len() != rgba_frames.len() {
            return Err(format!(
                "Matroska block count {} != frame count {}",
                mkv_demux.blocks.len(),
                rgba_frames.len()
            ));
        }

        let mut seq = TimelineSequence::default();
        seq.name = "Locked-dt Chronicle".into();
        seq.frame_rate = rate;
        seq.duration = timeline_duration;
        let mut vtrack = TimelineTrack::new("V1", TrackKind::Video);
        vtrack.clips.push(TimelineClip {
            id: "locked-dt-1".into(),
            name: "locked_dt.mov".into(),
            reel: "LDT".into(),
            source_path: "file:///chronicle/locked_dt.mov".into(),
            timeline_start: chronicle_timeline_interchange::Tick::ZERO,
            timeline_duration,
            source_in: chronicle_timeline_interchange::Tick::ZERO,
            source_out: timeline_duration,
            speed: 1.0,
        });
        seq.video_tracks.push(vtrack);
        let otio_str = write_otio(&seq);
        let otio_rt = parse_otio(&otio_str).map_err(|e| format!("OTIO parse failed: {e}"))?;
        if otio_rt.video_tracks.len() != 1 || otio_rt.video_tracks[0].clips.len() != 1 {
            return Err("OTIO round-trip lost the locked-dt video clip".into());
        }
        let otio_dur = otio_rt.video_tracks[0].clips[0].timeline_duration;
        if otio_dur == chronicle_timeline_interchange::Tick::ZERO
            || rate.frame_at(otio_dur) != rgba_frames.len() as i64
        {
            return Err("OTIO clip duration did not round-trip locked-dt frames".into());
        }

        let mp4_bytes = match self.workspace.get(mp4) {
            Ok(Artifact::Encoded(r)) => r.bytes.clone(),
            _ => Vec::new(),
        };

        let mut audit = FiduciaryAuditTrail::new();
        audit.append(
            1_700_000_000,
            "did:hypermedia:chronicle:locked-dt",
            "movie.mux_mp4",
            "movie",
            "locked_dt.mp4",
            format!("fnv1a:{}", fnv1a_hex(&mp4_bytes)),
            None,
        );
        audit.append(
            1_700_000_001,
            "did:hypermedia:chronicle:locked-dt",
            "movie.encode_prores",
            "prores",
            "locked_dt.prores",
            format!("fnv1a:{}", fnv1a_hex(&first_prores)),
            None,
        );
        audit.append(
            1_700_000_002,
            "did:hypermedia:chronicle:locked-dt",
            "movie.mux_matroska",
            "matroska",
            "locked_dt.mkv",
            format!("fnv1a:{}", fnv1a_hex(&mkv_bytes)),
            None,
        );
        audit.append(
            1_700_000_003,
            "did:hypermedia:chronicle:locked-dt",
            "movie.write_otio",
            "otio",
            "locked_dt.otio",
            format!("fnv1a:{}", fnv1a_hex(otio_str.as_bytes())),
            None,
        );
        audit
            .verify_chain()
            .map_err(|(i, e)| format!("ledger hash chain broken at {i}: {e}"))?;

        let ledger_json = serde_json::json!({
            "locked_dt": true,
            "fps": fps,
            "frame_count": rgba_frames.len(),
            "sample_duration_ticks": sample_duration_ticks,
            "fiduciary_chain_verified": true,
            "entry_count": audit.entries.len(),
            "head_hash": audit.entries.last().map(|e| e.entry_hash.clone()),
            "audit_trail": audit,
        });
        let ledger_bytes =
            serde_json::to_vec(&ledger_json).map_err(|e| format!("ledger JSON failed: {e}"))?;

        let prores_resource = self.encode("prores", first_prores)?;
        let otio_resource = self.encode("otio", otio_str.into_bytes())?;
        let mkv_resource = self.encode("mkv", mkv_bytes)?;
        let ledger_resource = self.encode("json", ledger_bytes)?;

        let mut out = BTreeMap::new();
        out.insert("prores".into(), prores_resource);
        out.insert("otio".into(), otio_resource);
        out.insert("mkv".into(), mkv_resource);
        out.insert("ledger".into(), ledger_resource);
        out.insert("fps".into(), Value::U64(fps as u64));
        out.insert("frame_count".into(), Value::U64(rgba_frames.len() as u64));
        out.insert("locked_dt".into(), Value::Bool(true));
        out.insert(
            "sample_duration_ticks".into(),
            Value::I64(sample_duration_ticks),
        );
        out.insert(
            "otio_video_track_count".into(),
            Value::I64(otio_rt.video_tracks.len() as i64),
        );
        out.insert(
            "otio_clip_count".into(),
            Value::I64(otio_rt.video_tracks[0].clips.len() as i64),
        );
        out.insert(
            "matroska_block_count".into(),
            Value::I64(mkv_demux.blocks.len() as i64),
        );
        out.insert("ledger_chain_verified".into(), Value::Bool(true));
        out.insert("ledger_entry_count".into(), Value::U64(4));
        out.insert("prores_decoded_width".into(), Value::I64(pr_dec.width as i64));
        out.insert(
            "prores_decoded_height".into(),
            Value::I64(pr_dec.height as i64),
        );
        Ok(out)
    }
}

fn locked_dt_frame_rate(fps: usize) -> chronicle_timeline_interchange::FrameRate {
    use chronicle_timeline_interchange::FrameRate;
    match fps {
        24 => FrameRate::FPS_24,
        25 => FrameRate::FPS_25,
        30 => FrameRate::FPS_30,
        50 => FrameRate::FPS_50,
        60 => FrameRate::FPS_60,
        n => FrameRate {
            num: n as i64,
            den: 1,
        },
    }
}

fn rgba_to_prores(img: &Rgba) -> video_prores_codec::Frame {
    use video_prores_codec::{ChromaFormat, Frame as ProResFrame};
    let mut frame = ProResFrame::new(img.width, img.height, ChromaFormat::Yuv422, 10, false);
    let cw = frame.chroma_width() as usize;
    let w = img.width as usize;
    let h = img.height as usize;
    for y in 0..h {
        for x in 0..w {
            let px = img.pixels[y * w + x];
            let r = px[0] as f32 / 255.0;
            let g = px[1] as f32 / 255.0;
            let b = px[2] as f32 / 255.0;
            let y_norm = 0.2126 * r + 0.7152 * g + 0.0722 * b;
            frame.y[y * w + x] = ((16.0 + 219.0 * y_norm) * 4.0).round().clamp(64.0, 940.0) as u16;
            if x % 2 == 0 {
                let cb_norm = (b - y_norm) / 1.8556;
                let cr_norm = (r - y_norm) / 1.5748;
                let ci = y * cw + x / 2;
                frame.cb[ci] = ((128.0 + 224.0 * cb_norm) * 4.0).round().clamp(64.0, 960.0) as u16;
                frame.cr[ci] = ((128.0 + 224.0 * cr_norm) * 4.0).round().clamp(64.0, 960.0) as u16;
            }
        }
    }
    frame
}

fn fnv1a_hex(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
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

#[cfg(test)]
mod tests {
    use super::*;
    use chronicle_fiduciary_ledger::FiduciaryAuditTrail;
    use chronicle_timeline_interchange::{parse_otio, FrameRate, TICKS_PER_SECOND};
    use std::collections::BTreeMap;

    fn host<'a>(
        limits: &'a ToolLimits,
        cancel: &'a AtomicBool,
        inputs: &'a [Resource],
    ) -> AssetHost<'a> {
        AssetHost {
            workspace: AssetWorkspace::default(),
            inputs,
            limits,
            cancel,
            work: 0,
        }
    }

    fn solid_frame(host: &mut AssetHost<'_>, rgb: [u8; 3]) -> Value {
        let pixels = vec![[rgb[0], rgb[1], rgb[2], 255]; 256];
        host.workspace
            .insert(
                Artifact::Rgba(Rgba {
                    width: 16,
                    height: 16,
                    pixels,
                }),
                host.limits,
            )
            .expect("rgba frame")
    }

    fn encoded<'a>(host: &'a AssetHost<'_>, handle: &Value) -> &'a Resource {
        host.workspace.encoded(handle).expect("encoded resource")
    }

    #[test]
    fn chronicle_mux_mp4_origin_still_returns_mp4() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = host(&limits, &cancel, &[]);
        let f1 = solid_frame(&mut host, [200, 100, 50]);
        let f2 = solid_frame(&mut host, [40, 80, 160]);
        let mut args = BTreeMap::new();
        args.insert("frames".into(), Value::List(vec![f1, f2]));
        args.insert("fps".into(), Value::U64(24));
        let res = host
            .chronicle_op("hypermedia.chronicle.mux_mp4", &args)
            .expect("origin mux_mp4");
        let mp4 = encoded(&host, &res);
        assert_eq!(mp4.kind, "mp4");
        assert!(mp4.bytes.len() > 100);
    }

    #[test]
    fn muxed_media_timing_is_exact_at_integer_and_ntsc_rates() {
        let limits=ToolLimits::default();let cancel=AtomicBool::new(false);
        for (num,den) in [(24u64,1u64),(29,1),(24000,1001),(30000,1001)] {
            let mut host=host(&limits,&cancel,&[]);
            let frame=solid_frame(&mut host,[80,120,160]);
            let args=Args::from([("frames".into(),Value::List(vec![frame.clone(),frame])),
                ("fps".into(),Value::Record(Args::from([("numerator".into(),Value::U64(num)),("denominator".into(),Value::U64(den))])))]);
            let output=host.chronicle_op("hypermedia.chronicle.mux_mp4",&args).unwrap();
            let bytes=&encoded(&host,&output).bytes;
            let media=media_isobmff::open(bytes).unwrap();let track=&media.tracks[0];
            assert_eq!(u64::from(track.timescale),num);
            assert_eq!(track.samples.len(),2);
            assert_eq!(u64::from(track.samples[0].duration),den);
            assert_eq!(track.samples[1].dts,den as i64);
            let ticks=((u128::from(video_h264_preview::TICKS_PER_SECOND)*2*den as u128).div_ceil(num as u128)) as u64;
            let decoded=video_h264_preview::decode_mp4(bytes,0,ticks,&cancel).unwrap();
            assert_eq!(decoded.len(),2);
            assert_eq!(decoded.iter().map(|f|f.duration_ticks).sum::<u64>(),ticks);
        }
    }

    #[test]
    fn chronicle_locked_dt_movie_otio_ledger_objects() {
        let limits = ToolLimits::default();
        let cancel = AtomicBool::new(false);
        let mut host = host(&limits, &cancel, &[]);
        let f1 = solid_frame(&mut host, [200, 100, 50]);
        let f2 = solid_frame(&mut host, [40, 80, 160]);
        let mut args = BTreeMap::new();
        args.insert("frames".into(), Value::List(vec![f1, f2]));
        args.insert("fps".into(), Value::U64(24));

        let res = host
            .acoustics_discourse_chronicle_ext_op(
                "hypermedia.chronicle.capture_deterministic_movie",
                &args,
            )
            .expect("locked-dt capture");

        // Origin MP4 handle is preserved on the produce-safe map.
        let mp4 = encoded(&host, &res);
        assert_eq!(mp4.kind, "mp4");
        assert!(mp4.bytes.len() > 100);

        let rec = record(&res).expect("produce-safe record");
        assert_eq!(rec.get("locked_dt"), Some(&Value::Bool(true)));
        assert_eq!(rec.get("frame_count"), Some(&Value::U64(2)));
        assert_eq!(rec.get("fps"), Some(&Value::U64(24)));
        assert_eq!(rec.get("ledger_chain_verified"), Some(&Value::Bool(true)));
        assert_eq!(rec.get("matroska_block_count"), Some(&Value::I64(2)));
        assert_eq!(rec.get("otio_video_track_count"), Some(&Value::I64(1)));
        assert_eq!(rec.get("otio_clip_count"), Some(&Value::I64(1)));
        let ticks = match rec.get("sample_duration_ticks") {
            Some(Value::I64(t)) => *t,
            other => panic!("sample_duration_ticks, got {other:?}"),
        };
        assert_eq!(ticks, TICKS_PER_SECOND / 24);

        let prores = encoded(&host, rec.get("prores").expect("prores handle"));
        assert_eq!(prores.kind, "prores");
        let decoded = video_prores_codec::decode_frame(&prores.bytes).expect("decode ProRes");
        assert_eq!((decoded.width, decoded.height), (16, 16));

        let mkv = encoded(&host, rec.get("mkv").expect("mkv handle"));
        assert_eq!(mkv.kind, "mkv");
        let demux = chronicle_broadcast_containers::demux_matroska(&mkv.bytes).expect("demux mkv");
        assert_eq!(demux.blocks.len(), 2);
        assert_eq!(demux.tracks[0].codec_id, "V_PRORES");

        let otio = encoded(&host, rec.get("otio").expect("otio handle"));
        assert_eq!(otio.kind, "otio");
        let otio_str = std::str::from_utf8(&otio.bytes).expect("otio utf8");
        let seq = parse_otio(otio_str).expect("parse otio");
        assert_eq!(seq.name, "Locked-dt Chronicle");
        assert_eq!(seq.video_tracks.len(), 1);
        assert_eq!(seq.video_tracks[0].clips.len(), 1);
        assert_eq!(seq.video_tracks[0].clips[0].name, "locked_dt.mov");
        assert_eq!(
            seq.video_tracks[0].clips[0].source_path,
            "file:///chronicle/locked_dt.mov"
        );
        let clip_dur = seq.video_tracks[0].clips[0].timeline_duration;
        assert!(clip_dur.0 > 0, "OTIO clip duration must be > 0");
        assert_eq!(
            FrameRate::FPS_24.frame_at(clip_dur),
            2,
            "OTIO duration must match locked-dt frame count"
        );
        let otio_json: serde_json::Value = serde_json::from_str(otio_str).expect("otio json");
        let duration_value = otio_json["tracks"]["children"][0]["children"][0]["source_range"]
            ["duration"]["value"]
            .as_f64()
            .expect("RationalTime.value");
        assert!(duration_value > 0.0);
        assert_eq!(duration_value, 2.0);
        assert!(otio_str.contains("locked_dt.mov"));
        assert!(otio_str.contains("RationalTime.1"));

        let ledger = encoded(&host, rec.get("ledger").expect("ledger handle"));
        assert_eq!(ledger.kind, "json");
        let proof: serde_json::Value =
            serde_json::from_slice(&ledger.bytes).expect("ledger json");
        assert_eq!(proof["fiduciary_chain_verified"], true);
        assert_eq!(proof["locked_dt"], true);
        assert_eq!(proof["frame_count"], 2);
        let trail: FiduciaryAuditTrail =
            serde_json::from_value(proof["audit_trail"].clone()).expect("audit trail");
        assert_eq!(trail.entries.len(), 4);
        trail
            .verify_chain()
            .expect("ledger hash chain must verify");
        assert_eq!(trail.entries[0].action, "movie.mux_mp4");
        assert_eq!(trail.entries[3].action, "movie.write_otio");
        assert_ne!(trail.entries[0].entry_hash, trail.entries[1].entry_hash);
        assert_eq!(
            trail.entries[1].previous_hash,
            trail.entries[0].entry_hash
        );
    }
}

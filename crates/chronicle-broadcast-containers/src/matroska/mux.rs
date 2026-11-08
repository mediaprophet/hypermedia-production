//! Matroska / WebM muxer: SimpleBlocks (optionally laced), BlockGroups for explicit
//! durations, Cues, SeekHead, Info with Duration.

use std::io::{Seek, SeekFrom, Write};

use crate::matroska::ebml::{write_id, write_size, write_unknown_size};
use crate::matroska::ids::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrackKind {
    Video,
    Audio,
    Subtitle,
    Complex,
    Logo,
    Buttons,
    Control,
    Metadata,
    Other(u8),
}

/// Lacing used for audio tracks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LacingMode {
    None,
    Xiph,
    Ebml,
    FixedOrEbml,
}

/// Track description for [`MkvWriter`].
#[derive(Clone, Debug)]
pub struct TrackSpec {
    pub kind: TrackKind,
    pub codec_id: String,
    pub codec_private: Vec<u8>,
    pub default_duration_ns: Option<u64>,
    pub language: Option<String>,
    pub name: Option<String>,
    pub video_size: Option<(u32, u32)>,
    pub audio: Option<(f64, u64, Option<u64>)>,
    pub codec_delay_ns: u64,
    pub seek_pre_roll_ns: u64,
}

impl TrackSpec {
    pub fn new(kind: TrackKind, codec_id: impl Into<String>) -> Self {
        Self {
            kind,
            codec_id: codec_id.into(),
            codec_private: Vec::new(),
            default_duration_ns: None,
            language: None,
            name: None,
            video_size: None,
            audio: None,
            codec_delay_ns: 0,
            seek_pre_roll_ns: 0,
        }
    }
}

/// Muxer options.
#[derive(Clone, Debug)]
pub struct MuxOptions {
    pub doc_type: String,
    pub timestamp_scale: u64,
    pub cluster_duration_ns: u64,
    pub lacing: LacingMode,
    pub max_lace_frames: usize,
    pub title: Option<String>,
    pub writing_app: String,
}

impl Default for MuxOptions {
    fn default() -> Self {
        Self {
            doc_type: "matroska".into(),
            timestamp_scale: 1_000_000,
            cluster_duration_ns: 1_000_000_000,
            lacing: LacingMode::None,
            max_lace_frames: 8,
            title: None,
            writing_app: "chronicle".into(),
        }
    }
}

fn el(out: &mut Vec<u8>, id: u32, data: &[u8]) {
    write_id(out, id);
    write_size(out, data.len() as u64, 1);
    out.extend_from_slice(data);
}

fn el_uint(out: &mut Vec<u8>, id: u32, v: u64) {
    let n = (8 - v.leading_zeros() as usize / 8).max(1);
    el(out, id, &v.to_be_bytes()[8 - n..]);
}

fn el_float(out: &mut Vec<u8>, id: u32, v: f64) {
    el(out, id, &v.to_be_bytes());
}

fn el_str(out: &mut Vec<u8>, id: u32, s: &str) {
    el(out, id, s.as_bytes());
}

fn void(out: &mut Vec<u8>, total: usize) {
    debug_assert!(total >= 2);
    let sl = if total - 2 <= 126 { 1 } else { 8 };
    write_id(out, VOID);
    write_size(out, (total - 1 - sl) as u64, sl);
    out.resize(out.len() + total - 1 - sl, 0);
}

#[derive(Clone, Debug)]
struct Pending {
    track: usize,
    ts: i64,
    frames: Vec<Vec<u8>>,
}

const SEEK_HEAD_SPACE: usize = 128;

/// Streaming Matroska/WebM writer.
pub struct MkvWriter<W: Write + Seek> {
    w: W,
    opts: MuxOptions,
    tracks: Vec<TrackSpec>,
    segment_size_pos: u64,
    segment_data_start: u64,
    seek_head_pos: u64,
    duration_pos: u64,
    info_pos: u64,
    tracks_pos: u64,
    cluster: Vec<u8>,
    cluster_ts: Option<i64>,
    cluster_start_ns: i64,
    pending: Option<Pending>,
    cues: Vec<(u64, u64, u64)>, // (time ticks, track number, cluster position)
    cued_tracks_in_cluster: Vec<bool>,
    end_ns: i64,
    has_video: bool,
}

impl<W: Write + Seek> MkvWriter<W> {
    pub fn new(mut w: W, tracks: Vec<TrackSpec>, opts: MuxOptions) -> Result<Self, String> {
        if tracks.is_empty() || opts.timestamp_scale == 0 {
            return Err("muxer needs at least one track and non-zero timestamp scale".into());
        }
        let mut out = Vec::new();
        let mut h = Vec::new();
        el_uint(&mut h, EBML_VERSION, 1);
        el_uint(&mut h, EBML_READ_VERSION, 1);
        el_uint(&mut h, EBML_MAX_ID_LENGTH, 4);
        el_uint(&mut h, EBML_MAX_SIZE_LENGTH, 8);
        el_str(&mut h, DOC_TYPE, &opts.doc_type);
        el_uint(&mut h, DOC_TYPE_VERSION, 4);
        el_uint(&mut h, DOC_TYPE_READ_VERSION, 2);
        el(&mut out, EBML, &h);
        write_id(&mut out, SEGMENT);
        let base = w.stream_position().map_err(|e| e.to_string())?;
        let segment_size_pos = base + out.len() as u64;
        write_unknown_size(&mut out, 8);
        let segment_data_start = base + out.len() as u64;
        let seek_head_pos = segment_data_start;
        void(&mut out, SEEK_HEAD_SPACE);

        // Info
        let info_pos = base + out.len() as u64;
        let mut info = Vec::new();
        el_uint(&mut info, TIMESTAMP_SCALE, opts.timestamp_scale);
        el_str(&mut info, MUXING_APP, "chronicle-broadcast-containers");
        el_str(&mut info, WRITING_APP, &opts.writing_app);
        if let Some(t) = &opts.title {
            el_str(&mut info, TITLE, t);
        }
        let dur_off_in_info = info.len() + 2 + 1; // ID (2) + size (1)
        el_float(&mut info, DURATION, 0.0);
        write_id(&mut out, INFO);
        write_size(&mut out, info.len() as u64, 1);
        let duration_pos = base + out.len() as u64 + dur_off_in_info as u64;
        out.extend_from_slice(&info);

        // Tracks
        let tracks_pos = base + out.len() as u64;
        let mut tr = Vec::new();
        for (i, t) in tracks.iter().enumerate() {
            let mut e = Vec::new();
            el_uint(&mut e, TRACK_NUMBER, i as u64 + 1);
            el_uint(&mut e, TRACK_UID, 0x1000 + i as u64 + 1);
            let ty = match t.kind {
                TrackKind::Video => 1,
                TrackKind::Audio => 2,
                TrackKind::Subtitle => 0x11,
                TrackKind::Complex => 3,
                TrackKind::Logo => 0x10,
                TrackKind::Buttons => 0x12,
                TrackKind::Control => 0x20,
                TrackKind::Metadata => 0x21,
                TrackKind::Other(n) => n as u64,
            };
            el_uint(&mut e, TRACK_TYPE, ty);
            el_uint(&mut e, FLAG_LACING, (t.kind == TrackKind::Audio && opts.lacing != LacingMode::None) as u64);
            if let Some(l) = &t.language {
                el_str(&mut e, LANGUAGE, l);
            }
            if let Some(n) = &t.name {
                el_str(&mut e, NAME, n);
            }
            el_str(&mut e, CODEC_ID, &t.codec_id);
            if !t.codec_private.is_empty() {
                el(&mut e, CODEC_PRIVATE, &t.codec_private);
            }
            if let Some(d) = t.default_duration_ns {
                el_uint(&mut e, DEFAULT_DURATION, d);
            }
            if t.codec_delay_ns > 0 {
                el_uint(&mut e, CODEC_DELAY, t.codec_delay_ns);
            }
            if t.seek_pre_roll_ns > 0 {
                el_uint(&mut e, SEEK_PRE_ROLL, t.seek_pre_roll_ns);
            }
            if let Some((pw, ph)) = t.video_size {
                let mut v = Vec::new();
                el_uint(&mut v, PIXEL_WIDTH, pw as u64);
                el_uint(&mut v, PIXEL_HEIGHT, ph as u64);
                el(&mut e, VIDEO, &v);
            }
            if let Some((rate, ch, bits)) = t.audio {
                let mut a = Vec::new();
                el_float(&mut a, SAMPLING_FREQUENCY, rate);
                el_uint(&mut a, CHANNELS, ch);
                if let Some(b) = bits {
                    el_uint(&mut a, BIT_DEPTH, b);
                }
                el(&mut e, AUDIO, &a);
            }
            el(&mut tr, TRACK_ENTRY, &e);
        }
        el(&mut out, TRACKS, &tr);
        w.write_all(&out).map_err(|e| e.to_string())?;

        let has_video = tracks.iter().any(|t| t.kind == TrackKind::Video);
        let n = tracks.len();
        Ok(Self {
            w,
            opts,
            tracks,
            segment_size_pos,
            segment_data_start,
            seek_head_pos,
            duration_pos,
            info_pos,
            tracks_pos,
            cluster: Vec::new(),
            cluster_ts: None,
            cluster_start_ns: 0,
            pending: None,
            cues: Vec::new(),
            cued_tracks_in_cluster: vec![false; n],
            end_ns: 0,
            has_video,
        })
    }

    /// Add one frame.
    pub fn write_frame(
        &mut self,
        track: usize,
        pts_ns: i64,
        keyframe: bool,
        data: &[u8],
        duration_ns: Option<u64>,
    ) -> Result<(), String> {
        let spec = self.tracks.get(track).ok_or_else(|| format!("No such track {track}"))?;
        let data = if spec.codec_id == "V_PRORES" && data.get(4..8) == Some(b"icpf") {
            &data[8..]
        } else {
            data
        };
        let scale = self.opts.timestamp_scale as i64;
        let stored_ns = pts_ns + spec.codec_delay_ns as i64;
        if stored_ns < 0 {
            return Err("Negative timestamp".into());
        }
        let ts = (stored_ns + scale / 2) / scale;
        let dur_end = pts_ns + duration_ns.or(spec.default_duration_ns).unwrap_or(0) as i64;
        self.end_ns = self.end_ns.max(dur_end).max(pts_ns);
        let is_video = spec.kind == TrackKind::Video;
        let lacing = spec.kind == TrackKind::Audio && self.opts.lacing != LacingMode::None;
        let explicit_dur = duration_ns.filter(|&d| Some(d) != spec.default_duration_ns);

        let start_new = match self.cluster_ts {
            None => true,
            Some(cts) => {
                let long = stored_ns - self.cluster_start_ns >= self.opts.cluster_duration_ns as i64;
                (long && (!self.has_video || (is_video && keyframe))) || ts - cts > 30_000 || ts < cts
            }
        };
        if start_new {
            self.flush_cluster()?;
            self.cluster_ts = Some(ts);
            self.cluster_start_ns = stored_ns;
            el_uint(&mut self.cluster, TIMESTAMP, ts as u64);
        }
        let cts = self.cluster_ts.unwrap_or(0);

        if (is_video && keyframe) || (!self.has_video && !self.cued_tracks_in_cluster[track]) {
            self.cued_tracks_in_cluster[track] = true;
            let pos = self.w.stream_position().map_err(|e| e.to_string())? - self.segment_data_start;
            if self.cues.last().is_none_or(|c| c.2 != pos || c.1 != track as u64 + 1) {
                self.cues.push((ts as u64, track as u64 + 1, pos));
            }
        }
        if lacing && explicit_dur.is_none() {
            let same = self
                .pending
                .as_ref()
                .is_some_and(|p| p.track == track && p.frames.len() < self.opts.max_lace_frames);
            if !same {
                self.flush_pending();
            }
            match &mut self.pending {
                Some(p) => p.frames.push(data.to_vec()),
                None => {
                    self.pending = Some(Pending {
                        track,
                        ts: ts - cts,
                        frames: vec![data.to_vec()],
                    })
                }
            }
            return Ok(());
        }
        self.flush_pending();
        let mut block = Vec::new();
        write_size(&mut block, track as u64 + 1, 1);
        block.extend_from_slice(&((ts - cts) as i16).to_be_bytes());
        if let Some(d) = explicit_dur {
            block.push(0);
            block.extend_from_slice(data);
            let mut g = Vec::new();
            el(&mut g, BLOCK, &block);
            el_uint(&mut g, BLOCK_DURATION, (d as i64 / scale) as u64);
            if !keyframe {
                el(&mut g, REFERENCE_BLOCK, &[0xFF]);
            }
            el(&mut self.cluster, BLOCK_GROUP, &g);
        } else {
            block.push(if keyframe { 0x80 } else { 0 });
            block.extend_from_slice(data);
            el(&mut self.cluster, SIMPLE_BLOCK, &block);
        }
        Ok(())
    }

    fn flush_pending(&mut self) {
        let Some(p) = self.pending.take() else { return };
        let mut block = Vec::new();
        write_size(&mut block, p.track as u64 + 1, 1);
        block.extend_from_slice(&(p.ts as i16).to_be_bytes());
        let n = p.frames.len();
        let mode = if n == 1 { LacingMode::None } else { self.opts.lacing };
        let fixed = p.frames.iter().all(|f| f.len() == p.frames[0].len());
        match mode {
            LacingMode::None => block.push(0x80),
            LacingMode::Xiph => {
                block.push(0x80 | 0x02);
                block.push((n - 1) as u8);
                for f in &p.frames[..n - 1] {
                    let mut s = f.len();
                    while s >= 255 {
                        block.push(255);
                        s -= 255;
                    }
                    block.push(s as u8);
                }
            }
            LacingMode::FixedOrEbml if fixed => {
                block.push(0x80 | 0x04);
                block.push((n - 1) as u8);
            }
            LacingMode::Ebml | LacingMode::FixedOrEbml => {
                block.push(0x80 | 0x06);
                block.push((n - 1) as u8);
                write_size(&mut block, p.frames[0].len() as u64, 1);
                for i in 1..n - 1 {
                    let d = p.frames[i].len() as i64 - p.frames[i - 1].len() as i64;
                    let mut len = 1;
                    while len < 8 && d.unsigned_abs() >= (1u64 << (7 * len - 1)) - 1 {
                        len += 1;
                    }
                    let v = (d + (1i64 << (7 * len - 1)) - 1) as u64 | (1u64 << (7 * len));
                    block.extend_from_slice(&v.to_be_bytes()[8 - len..]);
                }
            }
        }
        for f in &p.frames {
            block.extend_from_slice(f);
        }
        el(&mut self.cluster, SIMPLE_BLOCK, &block);
    }

    fn flush_cluster(&mut self) -> Result<(), String> {
        self.flush_pending();
        if self.cluster_ts.is_some() {
            let mut out = Vec::new();
            el(&mut out, CLUSTER, &self.cluster);
            self.w.write_all(&out).map_err(|e| e.to_string())?;
        }
        self.cluster.clear();
        self.cluster_ts = None;
        self.cued_tracks_in_cluster.iter_mut().for_each(|c| *c = false);
        Ok(())
    }

    /// Write Cues, back-patch SeekHead / Segment size / Duration, and return writer.
    pub fn finish(mut self) -> Result<W, String> {
        self.flush_cluster()?;
        let cues_pos = self.w.stream_position().map_err(|e| e.to_string())?;
        let mut cues = Vec::new();
        for &(time, track, pos) in &self.cues {
            let mut ctp = Vec::new();
            el_uint(&mut ctp, CUE_TRACK, track);
            el_uint(&mut ctp, CUE_CLUSTER_POSITION, pos);
            let mut cp = Vec::new();
            el_uint(&mut cp, CUE_TIME, time);
            el(&mut cp, CUE_TRACK_POSITIONS, &ctp);
            el(&mut cues, CUE_POINT, &cp);
        }
        let has_cues = !cues.is_empty();
        if has_cues {
            let mut out = Vec::new();
            el(&mut out, CUES, &cues);
            self.w.write_all(&out).map_err(|e| e.to_string())?;
        }
        let end = self.w.stream_position().map_err(|e| e.to_string())?;

        // SeekHead
        let mut sh = Vec::new();
        let mut entries = vec![(INFO, self.info_pos), (TRACKS, self.tracks_pos)];
        if has_cues {
            entries.push((CUES, cues_pos));
        }
        for (id, pos) in entries {
            let mut s = Vec::new();
            let mut idb = Vec::new();
            write_id(&mut idb, id);
            el(&mut s, SEEK_ID, &idb);
            el_uint(&mut s, SEEK_POSITION, pos - self.segment_data_start);
            el(&mut sh, SEEK, &s);
        }
        let mut out = Vec::new();
        el(&mut out, SEEK_HEAD, &sh);
        if out.len() + 2 > SEEK_HEAD_SPACE {
            return Err("SeekHead does not fit its reserved space".into());
        }
        let rest = SEEK_HEAD_SPACE - out.len();
        void(&mut out, rest);
        self.w.seek(SeekFrom::Start(self.seek_head_pos)).map_err(|e| e.to_string())?;
        self.w.write_all(&out).map_err(|e| e.to_string())?;

        // Duration
        let dur = self.end_ns as f64 / self.opts.timestamp_scale as f64;
        self.w.seek(SeekFrom::Start(self.duration_pos)).map_err(|e| e.to_string())?;
        self.w.write_all(&dur.to_be_bytes()).map_err(|e| e.to_string())?;

        // Segment size
        let mut sz = Vec::new();
        write_size(&mut sz, end - self.segment_data_start, 8);
        self.w.seek(SeekFrom::Start(self.segment_size_pos)).map_err(|e| e.to_string())?;
        self.w.write_all(&sz).map_err(|e| e.to_string())?;
        self.w.seek(SeekFrom::Start(end)).map_err(|e| e.to_string())?;
        self.w.flush().map_err(|e| e.to_string())?;
        Ok(self.w)
    }
}

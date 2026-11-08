//! Timeline inspection, sample table extraction, random-access seeking, and LRU frame caching.

use std::collections::VecDeque;
use std::sync::Arc;

use media_isobmff::{ByteSource, CodecConfig, Mp4File, TrackKind};

use crate::convert::{Matrix, planar_to_rgba, scale_rgba};
use crate::Frame;

/// A random-access byte source a [`Movie`] can read from.
pub type SharedSource = Arc<dyn ByteSource + Send + Sync>;

/// Largest picture permitted to decode (bounds allocations).
const MAX_PIXELS: u64 = 8192 * 8192;

/// Movie timeline and decoding errors.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum MovieError {
    #[error("container error: {0}")]
    Container(String),
    #[error("the movie has no video track")]
    NoVideoTrack,
    #[error("unsupported video format: {0}")]
    Unsupported(String),
    #[error("decode error: {0}")]
    Decode(String),
    #[error("requested time or index is out of range")]
    OutOfRange,
}

/// The size that fits `w` x `h` inside `max_w` x `max_h`, keeping the aspect ratio (>= 1 px).
pub fn fit_size(w: u32, h: u32, max_w: u32, max_h: u32) -> (u32, u32) {
    if w == 0 || h == 0 {
        return (1, 1);
    }
    let s = (f64::from(max_w.max(1)) / f64::from(w))
        .min(f64::from(max_h.max(1)) / f64::from(h))
        .min(1.0);
    (
        ((f64::from(w) * s).round() as u32).max(1),
        ((f64::from(h) * s).round() as u32).max(1),
    )
}

/// Description of the movie picture stream and format.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct MovieInfo {
    pub width: u32,
    pub height: u32,
    pub pixel_aspect: f64,
    pub duration: f64,
    pub frame_rate: f64,
    pub frame_count: usize,
    pub codec: String,
    pub codec_detail: String,
    pub decodable: bool,
    pub has_audio: bool,
    pub quicktime: bool,
}

impl MovieInfo {
    pub fn display_aspect(&self) -> f64 {
        if self.height == 0 {
            16.0 / 9.0
        } else {
            f64::from(self.width) * self.pixel_aspect / f64::from(self.height)
        }
    }
}

enum Codec {
    H264 { avcc: Vec<u8> },
    ProRes,
    Unsupported(String),
}

struct H264State {
    dec: video_h264_codec::Decoder,
    next: usize,
    last_out: Option<usize>,
    ready: std::collections::BTreeMap<usize, video_h264_codec::Picture>,
}

/// An LRU cache of decoded frames.
struct Lru<K: PartialEq + Copy> {
    cap: usize,
    items: VecDeque<(K, Arc<Frame>)>,
}

impl<K: PartialEq + Copy> Lru<K> {
    fn new(cap: usize) -> Self {
        Lru {
            cap: cap.max(1),
            items: VecDeque::new(),
        }
    }
    fn get(&mut self, k: K) -> Option<Arc<Frame>> {
        let i = self.items.iter().position(|(x, _)| *x == k)?;
        let item = self.items.remove(i)?;
        let f = Arc::clone(&item.1);
        self.items.push_back(item);
        Some(f)
    }
    fn put(&mut self, k: K, f: Arc<Frame>) {
        self.items.retain(|(x, _)| *x != k);
        self.items.push_back((k, f));
        while self.items.len() > self.cap {
            self.items.pop_front();
        }
    }
    fn contains(&self, k: K) -> bool {
        self.items.iter().any(|(x, _)| *x == k)
    }
}

/// An opened movie: container inspection index plus decoder state and frame caches.
pub struct Movie {
    src: SharedSource,
    mp4: Mp4File,
    track: usize,
    info: MovieInfo,
    codec: Codec,
    order: Vec<usize>,
    rank: Vec<usize>,
    times: Vec<f64>,
    h264: Option<H264State>,
    cache: Lru<usize>,
    thumbs: Lru<(usize, u32)>,
    pub decoded: u64,
}

impl std::fmt::Debug for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Movie").field("info", &self.info).finish_non_exhaustive()
    }
}

impl Movie {
    /// Open a movie held in memory.
    pub fn open(bytes: impl Into<Arc<[u8]>>) -> Result<Movie, MovieError> {
        let b: Arc<[u8]> = bytes.into();
        Movie::open_source(Arc::new(b))
    }

    /// Open a movie from any random-access byte source.
    pub fn open_source(src: SharedSource) -> Result<Movie, MovieError> {
        let mp4 = media_isobmff::open(&*src).map_err(|e| MovieError::Container(e.to_string()))?;
        let track = mp4
            .tracks
            .iter()
            .position(|t| t.kind == TrackKind::Video && t.enabled && !t.samples.is_empty())
            .or_else(|| mp4.tracks.iter().position(|t| t.kind == TrackKind::Video && !t.samples.is_empty()))
            .ok_or(MovieError::NoVideoTrack)?;
        let has_audio = mp4.tracks.iter().any(|t| t.kind == TrackKind::Audio && !t.samples.is_empty());
        let t = mp4.tracks.get(track).ok_or(MovieError::NoVideoTrack)?;
        let ts = f64::from(t.timescale.max(1));
        let n = t.samples.len();
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_by_key(|&i| (t.samples.get(i).map_or(0, |s| s.pts), i));
        let mut rank = vec![0usize; n];
        for (p, &i) in order.iter().enumerate() {
            if let Some(r) = rank.get_mut(i) {
                *r = p;
            }
        }
        let times: Vec<f64> = order
            .iter()
            .map(|&i| t.presentation_pts(i).unwrap_or(0) as f64 / ts)
            .collect();
        let first = times.first().copied().unwrap_or(0.0);
        let end = order
            .iter()
            .filter_map(|&i| {
                Some(
                    t.presentation_pts(i)?
                        .saturating_add(i64::from(t.samples.get(i)?.duration)),
                )
            })
            .max()
            .unwrap_or(0) as f64
            / ts;
        let duration = (end - first.min(0.0)).max(0.0);

        let mut durs: Vec<u32> = t.samples.iter().map(|s| s.duration).filter(|&d| d > 0).collect();
        durs.sort_unstable();
        let mut best = (0u32, 0usize);
        let mut k = 0;
        while k < durs.len() {
            let d = durs.get(k).copied().unwrap_or(0);
            let run = durs.iter().skip(k).take_while(|&&x| x == d).count().max(1);
            if run > best.1 {
                best = (d, run);
            }
            k += run;
        }
        let frame_rate = if best.0 > 0 {
            ts / f64::from(best.0)
        } else if duration > 0.0 {
            n as f64 / duration
        } else {
            0.0
        };

        let vp = t.video();
        let (mut width, mut height) = vp.map_or(
            (t.width >> 16, t.height >> 16),
            |v| (u32::from(v.width), u32::from(v.height)),
        );
        if width == 0 || height == 0 {
            (width, height) = (t.width >> 16, t.height >> 16);
        }
        let pixel_aspect = vp
            .and_then(|v| v.pixel_aspect)
            .filter(|&(h, v)| h > 0 && v > 0)
            .map_or(1.0, |(h, v)| f64::from(h) / f64::from(v));

        let (codec, name, detail) = match t.codec() {
            Some(CodecConfig::Avc(c)) => {
                let (ok, prof) = match c.profile {
                    66 => (true, "Baseline"),
                    77 => (true, "Main"),
                    88 => (true, "Extended"),
                    100 => (true, "High"),
                    110 => (false, "High 10"),
                    122 => (false, "High 4:2:2"),
                    244 => (false, "High 4:4:4"),
                    _ => (true, "?"),
                };
                let detail = format!("H.264 {prof}");
                if ok {
                    (Codec::H264 { avcc: c.to_bytes() }, "h264", detail)
                } else {
                    (
                        Codec::Unsupported(format!("{detail} (only 8-bit 4:2:0 H.264 is supported)")),
                        "h264",
                        detail,
                    )
                }
            }
            Some(CodecConfig::ProRes { fourcc }) => {
                let f = fourcc.0;
                let flavour = match &f {
                    b"apco" => "422 Proxy",
                    b"apcs" => "422 LT",
                    b"apcn" => "422",
                    b"apch" => "422 HQ",
                    b"ap4h" => "4444",
                    b"ap4x" => "4444 XQ",
                    _ => "",
                };
                (Codec::ProRes, "prores", format!("Apple ProRes {flavour}"))
            }
            Some(other) => {
                let nm = other.name();
                (
                    Codec::Unsupported(format!("{nm} pictures are not supported (H.264 and ProRes are)")),
                    nm,
                    nm.to_ascii_uppercase(),
                )
            }
            None => (
                Codec::Unsupported("the video track has no sample description".into()),
                "unknown",
                "unknown".into(),
            ),
        };

        if u64::from(width) * u64::from(height) > MAX_PIXELS {
            return Err(MovieError::Unsupported(format!(
                "{width}x{height} is larger than maximum permitted picture size"
            )));
        }

        let info = MovieInfo {
            width,
            height,
            pixel_aspect,
            duration,
            frame_rate,
            frame_count: n,
            codec: name.to_string(),
            codec_detail: detail,
            decodable: !matches!(codec, Codec::Unsupported(_)),
            has_audio,
            quicktime: mp4.is_quicktime,
        };

        Ok(Movie {
            src,
            mp4,
            track,
            info,
            codec,
            order,
            rank,
            times,
            h264: None,
            cache: Lru::new(12),
            thumbs: Lru::new(256),
            decoded: 0,
        })
    }

    pub fn info(&self) -> &MovieInfo {
        &self.info
    }

    pub fn set_cache_frames(&mut self, n: usize) {
        self.cache.cap = n.clamp(1, 512);
    }

    pub fn frame_index_at(&self, secs: f64) -> Option<usize> {
        if !secs.is_finite() || secs < 0.0 || secs >= self.info.duration {
            return None;
        }
        let k = self.times.partition_point(|&t| t <= secs + 1e-7);
        Some(k.saturating_sub(1))
    }

    pub fn frame_time(&self, index: usize) -> Option<f64> {
        self.times.get(index).copied()
    }

    pub fn frame_at(&mut self, secs: f64) -> Result<Arc<Frame>, MovieError> {
        let i = self.frame_index_at(secs).ok_or(MovieError::OutOfRange)?;
        self.frame(i)
    }

    pub fn is_cached(&self, index: usize) -> bool {
        self.cache.contains(index)
    }

    pub fn frame(&mut self, index: usize) -> Result<Arc<Frame>, MovieError> {
        if index >= self.order.len() {
            return Err(MovieError::OutOfRange);
        }
        if let Some(f) = self.cache.get(index) {
            return Ok(f);
        }
        let f = match &self.codec {
            Codec::Unsupported(why) => return Err(MovieError::Unsupported(why.clone())),
            Codec::ProRes => {
                let s = self.order.get(index).copied().ok_or(MovieError::OutOfRange)?;
                Arc::new(self.decode_intra(s, index)?)
            }
            Codec::H264 { .. } => self.decode_h264(index)?,
        };
        self.cache.put(index, Arc::clone(&f));
        Ok(f)
    }

    pub fn thumbnail_at(&mut self, secs: f64, max_h: u32) -> Result<Arc<Frame>, MovieError> {
        let i = self.frame_index_at(secs).ok_or(MovieError::OutOfRange)?;
        let max_h = max_h.clamp(4, 1080);
        let key = match self.codec {
            Codec::H264 { .. } => {
                let s = self.order.get(i).copied().ok_or(MovieError::OutOfRange)?;
                let k = self.mp4.tracks.get(self.track).map_or(0, |t| t.sync_sample_before(s));
                self.rank.get(k).copied().unwrap_or(i)
            }
            _ => i,
        };
        if let Some(f) = self.thumbs.get((key, max_h)) {
            return Ok(f);
        }
        let full = if let Some(f) = self.cache.get(key) {
            f
        } else {
            match &self.codec {
                Codec::Unsupported(why) => return Err(MovieError::Unsupported(why.clone())),
                Codec::H264 { avcc } => {
                    let avcc = avcc.clone();
                    let s = self.order.get(key).copied().ok_or(MovieError::OutOfRange)?;
                    Arc::new(self.decode_h264_single(&avcc, s, key)?)
                }
                _ => {
                    let s = self.order.get(key).copied().ok_or(MovieError::OutOfRange)?;
                    Arc::new(self.decode_intra(s, key)?)
                }
            }
        };
        let w = (f64::from(max_h) * self.info.display_aspect())
            .round()
            .clamp(1.0, 4096.0) as u32;
        let t = Arc::new(full.fit(w, max_h));
        self.thumbs.put((key, max_h), Arc::clone(&t));
        Ok(t)
    }

    pub fn thumbnails(&mut self, count: usize, max_h: u32) -> Vec<(f64, Result<Arc<Frame>, MovieError>)> {
        let count = count.clamp(1, 1000);
        let d = self.info.duration;
        (0..count)
            .map(|k| {
                let t = d * k as f64 / count as f64;
                (t, self.thumbnail_at(t, max_h))
            })
            .collect()
    }

    fn read(&self, sample: usize) -> Result<Vec<u8>, MovieError> {
        let t = self.mp4.tracks.get(self.track).ok_or(MovieError::NoVideoTrack)?;
        let size = t.samples.get(sample).map_or(0, |s| s.size);
        if size > 256 << 20 {
            return Err(MovieError::Decode("sample larger than 256 MiB".into()));
        }
        self.mp4
            .read_sample(&*self.src, self.track, sample)
            .map_err(|e| MovieError::Container(e.to_string()))
    }

    fn decode_intra(&mut self, sample: usize, index: usize) -> Result<Frame, MovieError> {
        let data = self.read(sample)?;
        let time = self.times.get(index).copied().unwrap_or(0.0);
        self.decoded += 1;
        match self.codec {
            Codec::ProRes => {
                let opts = video_prores_codec::DecodeOptions { bit_depth: Some(8) };
                let f = video_prores_codec::decode_frame_with(&data, &opts)
                    .map_err(|e| MovieError::Decode(e.to_string()))?;
                if u64::from(f.width) * u64::from(f.height) > MAX_PIXELS {
                    return Err(MovieError::Decode("picture too large".into()));
                }
                let (w, h) = (f.width as usize, f.height as usize);
                let cw = f.chroma_width() as usize;
                let narrow = |p: &[u16]| p.iter().map(|&v| v.min(255) as u8).collect::<Vec<u8>>();
                let (y, u, v) = (narrow(&f.y), narrow(&f.cb), narrow(&f.cr));
                let cx = if cw < w { 2 } else { 1 };
                let m = Matrix::for_code(f.color.matrix, f.height, false);
                let rgba = planar_to_rgba(w, h, &y, w, &u, &v, cw, cx, 1, &m);
                let duration_ticks = 0;
                Ok(Frame {
                    width: f.width,
                    height: f.height,
                    duration_ticks,
                    rgba,
                    index,
                    time,
                })
            }
            _ => Err(MovieError::Unsupported("not an intra-frame codec".into())),
        }
    }

    fn h264_to_frame(&self, p: &video_h264_codec::Picture, index: usize) -> Frame {
        let m = Matrix::for_code(p.color.matrix, p.height, p.color.full_range);
        let rgba = planar_to_rgba(
            p.width as usize,
            p.height as usize,
            &p.y,
            p.y_stride,
            &p.u,
            &p.v,
            p.uv_stride,
            2,
            2,
            &m,
        );
        let time = self.times.get(index).copied().unwrap_or(0.0);
        Frame {
            width: p.width,
            height: p.height,
            duration_ticks: 0,
            rgba,
            index,
            time,
        }
    }

    fn decode_h264_single(&mut self, avcc: &[u8], sample: usize, index: usize) -> Result<Frame, MovieError> {
        let mut dec = video_h264_codec::Decoder::from_avcc(avcc)
            .map_err(|e| MovieError::Decode(e.to_string()))?;
        let data = self.read(sample)?;
        let mut pics = dec
            .decode(&data, index as i64)
            .map_err(|e| MovieError::Decode(e.to_string()))?;
        pics.extend(dec.flush());
        self.decoded += 1;
        let p = pics
            .iter()
            .find(|p| p.pts == index as i64)
            .or(pics.first())
            .ok_or_else(|| MovieError::Decode("no picture decoded".into()))?;
        Ok(self.h264_to_frame(p, index))
    }

    fn decode_h264(&mut self, index: usize) -> Result<Arc<Frame>, MovieError> {
        let Codec::H264 { avcc } = &self.codec else {
            return Err(MovieError::Unsupported("not H.264".into()));
        };
        let target = self.order.get(index).copied().ok_or(MovieError::OutOfRange)?;
        let key = self.mp4.tracks.get(self.track).map_or(0, |t| t.sync_sample_before(target));
        let resume = self.h264.as_ref().is_some_and(|st| {
            st.ready.contains_key(&index)
                || (key < st.next && st.next != usize::MAX && st.last_out.is_none_or(|lo| lo < index))
        });
        if !resume {
            let dec = video_h264_codec::Decoder::from_avcc(avcc)
                .map_err(|e| MovieError::Decode(e.to_string()))?;
            self.h264 = Some(H264State {
                dec,
                next: key,
                last_out: None,
                ready: std::collections::BTreeMap::new(),
            });
        }
        let total = self.order.len();
        let mut last_err: Option<String> = None;
        for _ in 0..=total.saturating_add(1) {
            let Some(st) = self.h264.as_mut() else { break };
            st.ready.retain(|q, _| *q >= index);
            if let Some(p) = st.ready.remove(&index) {
                return Ok(Arc::new(self.h264_to_frame(&p, index)));
            }
            if st.next == usize::MAX {
                break;
            }
            let pics = if st.next < total {
                let s = st.next;
                st.next += 1;
                let pts = self.rank.get(s).copied().unwrap_or(0) as i64;
                let data = match self.mp4.read_sample(&*self.src, self.track, s) {
                    Ok(d) => d,
                    Err(e) => {
                        last_err = Some(e.to_string());
                        continue;
                    }
                };
                let Some(st) = self.h264.as_mut() else { break };
                match st.dec.decode(&data, pts) {
                    Ok(p) => p,
                    Err(e) => {
                        last_err = Some(e.to_string());
                        Vec::new()
                    }
                }
            } else {
                st.next = usize::MAX;
                st.dec.flush()
            };
            let Some(st) = self.h264.as_mut() else { break };
            for p in pics {
                let q = usize::try_from(p.pts).unwrap_or(usize::MAX);
                st.last_out = Some(q);
                self.decoded += 1;
                if q >= index {
                    st.ready.insert(q, p);
                }
            }
            while st.ready.len() > 32 {
                st.ready.pop_last();
            }
        }
        Err(MovieError::Decode(
            last_err.unwrap_or_else(|| format!("frame {index} was not produced")),
        ))
    }
}

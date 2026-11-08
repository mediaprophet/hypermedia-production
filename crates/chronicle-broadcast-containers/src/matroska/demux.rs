//! Clean-room Matroska (MKV) and WebM demuxer.

use crate::matroska::ebml::{Size, read_id, read_size, read_vint};
use crate::matroska::ids::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MatroskaTrack {
    pub number: u64,
    pub uid: u64,
    pub track_type: u64, // 1 = Video, 2 = Audio
    pub codec_id: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub sample_rate: Option<f64>,
    pub channels: Option<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatroskaBlock {
    pub track_number: u64,
    pub timestamp_ms: i64,
    pub keyframe: bool,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MatroskaDemuxer {
    pub doc_type: String,
    pub timestamp_scale: u64,
    pub duration_ms: Option<f64>,
    pub tracks: Vec<MatroskaTrack>,
    pub blocks: Vec<MatroskaBlock>,
}

pub fn demux_matroska(data: &[u8]) -> Result<MatroskaDemuxer, String> {
    let mut demux = MatroskaDemuxer::default();
    demux.timestamp_scale = 1_000_000; // default 1ms

    let mut pos = 0;
    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;

        let data_len = match size {
            Size::Known(s) => s as usize,
            Size::Unknown => data.len() - pos,
        };

        if pos + data_len > data.len() {
            break;
        }
        let elem_data = &data[pos..pos + data_len];

        match id {
            EBML => parse_ebml_header(elem_data, &mut demux)?,
            SEGMENT => parse_segment(elem_data, &mut demux)?,
            _ => {}
        }

        pos += data_len;
    }

    Ok(demux)
}

fn parse_ebml_header(data: &[u8], demux: &mut MatroskaDemuxer) -> Result<(), String> {
    let mut pos = 0;
    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;
        let Size::Known(len) = size else { break };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        if id == DOC_TYPE {
            demux.doc_type = String::from_utf8_lossy(body).trim_matches('\0').to_string();
        }
        pos += len;
    }
    Ok(())
}

fn parse_segment(data: &[u8], demux: &mut MatroskaDemuxer) -> Result<(), String> {
    let mut pos = 0;
    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;
        let Size::Known(len) = size else { break };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        match id {
            TRACKS => parse_tracks(body, demux)?,
            CLUSTER => parse_cluster(body, demux)?,
            INFO => parse_info(body, demux)?,
            _ => {}
        }
        pos += len;
    }
    Ok(())
}

fn parse_info(data: &[u8], demux: &mut MatroskaDemuxer) -> Result<(), String> {
    let mut pos = 0;
    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;
        let Size::Known(len) = size else { break };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        if id == TIMESTAMP_SCALE {
            let mut v = 0u64;
            for &b in body { v = (v << 8) | b as u64; }
            demux.timestamp_scale = v;
        }
        pos += len;
    }
    Ok(())
}

fn parse_tracks(data: &[u8], demux: &mut MatroskaDemuxer) -> Result<(), String> {
    let mut pos = 0;
    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;
        let Size::Known(len) = size else { break };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        if id == TRACK_ENTRY {
            if let Some(t) = parse_track_entry(body) {
                demux.tracks.push(t);
            }
        }
        pos += len;
    }
    Ok(())
}

fn parse_track_entry(data: &[u8]) -> Option<MatroskaTrack> {
    let mut number = 1u64;
    let mut uid = 1u64;
    let mut track_type = 1u64;
    let mut codec_id = String::new();
    let mut width = None;
    let mut height = None;
    let mut sample_rate = None;
    let mut channels = None;

    let mut pos = 0;
    while pos < data.len() {
        let (id, id_len) = read_id(&data[pos..])?;
        pos += id_len;
        let (size, size_len) = read_size(&data[pos..])?;
        pos += size_len;
        let Size::Known(len) = size else { return None };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        match id {
            TRACK_NUMBER => {
                let mut v = 0u64;
                for &b in body { v = (v << 8) | b as u64; }
                number = v;
            }
            TRACK_UID => {
                let mut v = 0u64;
                for &b in body { v = (v << 8) | b as u64; }
                uid = v;
            }
            TRACK_TYPE => {
                let mut v = 0u64;
                for &b in body { v = (v << 8) | b as u64; }
                track_type = v;
            }
            CODEC_ID => {
                codec_id = String::from_utf8_lossy(body).trim_matches('\0').to_string();
            }
            VIDEO => {
                // Parse video sub-elements
                let mut vpos = 0;
                while vpos < body.len() {
                    let (vid, vlen) = read_id(&body[vpos..])?;
                    vpos += vlen;
                    let (vsz, vsz_len) = read_size(&body[vpos..])?;
                    vpos += vsz_len;
                    let Size::Known(v_len) = vsz else { break };
                    let v_len = v_len as usize;
                    if vpos + v_len > body.len() { break; }
                    let v_data = &body[vpos..vpos + v_len];

                    if vid == PIXEL_WIDTH {
                        let mut w = 0u32;
                        for &b in v_data { w = (w << 8) | b as u32; }
                        width = Some(w);
                    } else if vid == PIXEL_HEIGHT {
                        let mut h = 0u32;
                        for &b in v_data { h = (h << 8) | b as u32; }
                        height = Some(h);
                    }
                    vpos += v_len;
                }
            }
            AUDIO => {
                let mut apos = 0;
                while apos < body.len() {
                    let (aid, alen) = read_id(&body[apos..])?;
                    apos += alen;
                    let (asz, asz_len) = read_size(&body[apos..])?;
                    apos += asz_len;
                    let Size::Known(a_len) = asz else { break };
                    let a_len = a_len as usize;
                    if apos + a_len > body.len() { break; }
                    let a_data = &body[apos..apos + a_len];

                    if aid == CHANNELS {
                        let mut c = 0u32;
                        for &b in a_data { c = (c << 8) | b as u32; }
                        channels = Some(c);
                    } else if aid == SAMPLING_FREQUENCY && a_data.len() >= 4 {
                        let rate = f32::from_be_bytes([a_data[0], a_data[1], a_data[2], a_data[3]]) as f64;
                        sample_rate = Some(rate);
                    }
                    apos += a_len;
                }
            }
            _ => {}
        }
        pos += len;
    }

    Some(MatroskaTrack {
        number,
        uid,
        track_type,
        codec_id,
        width,
        height,
        sample_rate,
        channels,
    })
}

fn parse_cluster(data: &[u8], demux: &mut MatroskaDemuxer) -> Result<(), String> {
    let mut cluster_time = 0i64;
    let mut pos = 0;

    while pos < data.len() {
        let Some((id, id_len)) = read_id(&data[pos..]) else { break };
        pos += id_len;
        let Some((size, size_len)) = read_size(&data[pos..]) else { break };
        pos += size_len;
        let Size::Known(len) = size else { break };
        let len = len as usize;
        if pos + len > data.len() { break; }
        let body = &data[pos..pos + len];

        if id == TIMESTAMP {
            let mut t = 0u64;
            for &b in body { t = (t << 8) | b as u64; }
            cluster_time = t as i64;
        } else if id == SIMPLE_BLOCK && body.len() >= 4 {
            let (track_num, v_len, _) = read_vint(body).ok_or("Invalid block track number")?;
            let rel_time = i16::from_be_bytes([body[v_len], body[v_len + 1]]) as i64;
            let flags = body[v_len + 2];
            let keyframe = (flags & 0x80) != 0;
            let payload = body[v_len + 3..].to_vec();

            demux.blocks.push(MatroskaBlock {
                track_number: track_num,
                timestamp_ms: cluster_time + rel_time,
                keyframe,
                payload,
            });
        }
        pos += len;
    }
    Ok(())
}

//! OpenTimelineIO (`.otio`) JSON interchange format reader and writer.
//!
//! Grounded in Academy Software Foundation (ASWF) OpenTimelineIO specifications (OTIO v1.0).

use serde_json::{Value, json};

use crate::model::{TimelineClip, TimelineMarker, TimelineSequence, TimelineTrack, TrackKind};
use crate::time::{FrameRate, Tick};

/// Parse an OpenTimelineIO JSON string into a TimelineSequence.
pub fn parse_otio(json_str: &str) -> Result<TimelineSequence, String> {
    let v: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;

    let schema = v.get("OTIO_SCHEMA").and_then(Value::as_str).unwrap_or("");
    if !schema.starts_with("Timeline") && !schema.starts_with("SerializableCollection") {
        return Err("JSON does not have a top-level OTIO Timeline schema".into());
    }

    let name = v.get("name").and_then(Value::as_str).unwrap_or("OTIO Timeline").to_string();

    let mut seq = TimelineSequence::default();
    seq.name = name;

    let tracks_val = if let Some(tracks) = v.get("tracks") {
        tracks
    } else {
        return Ok(seq);
    };

    let children = tracks_val.get("children").and_then(Value::as_array);
    let Some(track_list) = children else {
        return Ok(seq);
    };

    for track_val in track_list {
        let kind_str = track_val.get("kind").and_then(Value::as_str).unwrap_or("Video");
        let track_kind = if kind_str.eq_ignore_ascii_case("Audio") {
            TrackKind::Audio
        } else {
            TrackKind::Video
        };

        let track_name = track_val.get("name").and_then(Value::as_str).unwrap_or("Track");
        let mut track = TimelineTrack::new(track_name, track_kind);

        let mut cur_timeline = Tick::ZERO;

        if let Some(items) = track_val.get("children").and_then(Value::as_array) {
            for item in items {
                let item_schema = item.get("OTIO_SCHEMA").and_then(Value::as_str).unwrap_or("");
                let src_range = item.get("source_range");
                let dur = src_range
                    .and_then(|r| r.get("duration"))
                    .and_then(read_rational_time)
                    .unwrap_or(Tick::ZERO);

                let start_offset = src_range
                    .and_then(|r| r.get("start_time"))
                    .and_then(read_rational_time)
                    .unwrap_or(Tick::ZERO);

                if item_schema.starts_with("Clip") {
                    let clip_name = item.get("name").and_then(Value::as_str).unwrap_or("Clip");
                    let mut path = String::new();
                    if let Some(media) = item.get("media_reference") {
                        if let Some(target_url) = media.get("target_url").and_then(Value::as_str) {
                            path = target_url.to_string();
                        }
                    }

                    track.clips.push(TimelineClip {
                        id: format!("clip-{}", track.clips.len() + 1),
                        name: clip_name.to_string(),
                        reel: String::new(),
                        source_path: path,
                        timeline_start: cur_timeline,
                        timeline_duration: dur,
                        source_in: start_offset,
                        source_out: start_offset + dur,
                        speed: 1.0,
                    });
                    cur_timeline = cur_timeline + dur;
                } else if item_schema.starts_with("Gap") {
                    cur_timeline = cur_timeline + dur;
                }

                if let Some(markers) = item.get("markers").and_then(Value::as_array) {
                    for m in markers {
                        let marked_range = m.get("marked_range");
                        let m_time = marked_range
                            .and_then(|r| r.get("start_time"))
                            .and_then(read_rational_time)
                            .unwrap_or(cur_timeline);
                        let m_name = m.get("name").and_then(Value::as_str).unwrap_or("Marker");
                        let m_color = m.get("color").and_then(Value::as_str).unwrap_or("GREEN");
                        seq.markers.push(TimelineMarker {
                            id: format!("marker-{}", seq.markers.len() + 1),
                            time: m_time,
                            name: m_name.to_string(),
                            comment: String::new(),
                            color: m_color.to_string(),
                        });
                    }
                }
            }
        }

        match track_kind {
            TrackKind::Video => seq.video_tracks.push(track),
            TrackKind::Audio => seq.audio_tracks.push(track),
        }
    }

    Ok(seq)
}

fn read_rational_time(val: &Value) -> Option<Tick> {
    let rate = val.get("rate")?.as_f64()?;
    let value = val.get("value")?.as_f64()?;
    if rate <= 0.0 {
        return None;
    }
    Some(Tick::from_seconds(value / rate))
}

fn write_rational_time(t: Tick, rate: FrameRate) -> Value {
    let f = rate.frame_at(t);
    json!({
        "OTIO_SCHEMA": "RationalTime.1",
        "rate": rate.as_f64(),
        "value": f as f64
    })
}

fn write_time_range(start: Tick, dur: Tick, rate: FrameRate) -> Value {
    json!({
        "OTIO_SCHEMA": "TimeRange.1",
        "start_time": write_rational_time(start, rate),
        "duration": write_rational_time(dur, rate)
    })
}

/// Serialize a TimelineSequence to OpenTimelineIO JSON format.
pub fn write_otio(seq: &TimelineSequence) -> String {
    let mut tracks_json = Vec::new();

    for track in &seq.video_tracks {
        let mut children = Vec::new();
        for clip in &track.clips {
            let clip_obj = json!({
                "OTIO_SCHEMA": "Clip.1",
                "name": clip.name,
                "source_range": write_time_range(clip.source_in, clip.timeline_duration, seq.frame_rate),
                "media_reference": {
                    "OTIO_SCHEMA": "ExternalReference.1",
                    "target_url": clip.source_path,
                    "available_range": write_time_range(Tick::ZERO, clip.timeline_duration, seq.frame_rate)
                }
            });
            children.push(clip_obj);
        }

        let track_obj = json!({
            "OTIO_SCHEMA": "Track.1",
            "name": track.name,
            "kind": "Video",
            "children": children
        });
        tracks_json.push(track_obj);
    }

    for track in &seq.audio_tracks {
        let mut children = Vec::new();
        for clip in &track.clips {
            let clip_obj = json!({
                "OTIO_SCHEMA": "Clip.1",
                "name": clip.name,
                "source_range": write_time_range(clip.source_in, clip.timeline_duration, seq.frame_rate),
                "media_reference": {
                    "OTIO_SCHEMA": "ExternalReference.1",
                    "target_url": clip.source_path,
                    "available_range": write_time_range(Tick::ZERO, clip.timeline_duration, seq.frame_rate)
                }
            });
            children.push(clip_obj);
        }

        let track_obj = json!({
            "OTIO_SCHEMA": "Track.1",
            "name": track.name,
            "kind": "Audio",
            "children": children
        });
        tracks_json.push(track_obj);
    }

    let root = json!({
        "OTIO_SCHEMA": "Timeline.1",
        "name": seq.name,
        "tracks": {
            "OTIO_SCHEMA": "Stack.1",
            "name": "tracks",
            "children": tracks_json
        }
    });

    serde_json::to_string_pretty(&root).unwrap_or_default()
}

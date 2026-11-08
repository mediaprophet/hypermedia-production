//! Final Cut Pro 7 XML (`xmeml`) parser and writer.
//!
//! Grounded in Apple Final Cut Pro 7 XML interchange specifications (xmeml v4 / v5).

use crate::model::{TimelineClip, TimelineMarker, TimelineSequence, TimelineTrack, TrackKind};
use crate::time::{FrameRate, Tick};
use crate::xml::{XmlNode, XmlWriter};

/// Parse FCP7 XML text into a TimelineSequence.
pub fn parse_fcp7_xml(xml: &str) -> Result<TimelineSequence, String> {
    let root = XmlNode::parse(xml)?;
    let seq_node = if root.tag == "sequence" {
        &root
    } else {
        root.child("sequence")
            .ok_or("No <sequence> element found in FCP7 XML")?
    };

    let name = seq_node
        .child_text("name")
        .unwrap_or("FCP7 Sequence")
        .to_string();

    let rate = if let Some(r_node) = seq_node.child("rate") {
        let timebase: i64 = r_node
            .child_text("timebase")
            .and_then(|t| t.parse().ok())
            .unwrap_or(24);
        let ntsc = r_node
            .child_text("ntsc")
            .map(|s| s.eq_ignore_ascii_case("TRUE"))
            .unwrap_or(false);
        if ntsc {
            if timebase == 30 {
                FrameRate::FPS_29_97
            } else if timebase == 24 {
                FrameRate::FPS_23_976
            } else if timebase == 60 {
                FrameRate::FPS_59_94
            } else {
                FrameRate {
                    num: timebase * 1000,
                    den: 1001,
                }
            }
        } else {
            FrameRate {
                num: timebase,
                den: 1,
            }
        }
    } else {
        FrameRate::FPS_24
    };

    let mut seq = TimelineSequence {
        id: seq_node.attr("id").unwrap_or("sequence-1").to_string(),
        name,
        frame_rate: rate,
        drop_frame: false,
        start_time: Tick::ZERO,
        duration: Tick::ZERO,
        video_tracks: Vec::new(),
        audio_tracks: Vec::new(),
        markers: Vec::new(),
    };

    if let Some(media) = seq_node.child("media") {
        if let Some(video) = media.child("video") {
            for track_node in video.children_named("track") {
                let mut track = TimelineTrack::new("Video", TrackKind::Video);
                for clip_node in track_node.children_named("clipitem") {
                    if let Some(clip) = parse_clipitem(clip_node, rate) {
                        track.clips.push(clip);
                    }
                }
                seq.video_tracks.push(track);
            }
        }
        if let Some(audio) = media.child("audio") {
            for track_node in audio.children_named("track") {
                let mut track = TimelineTrack::new("Audio", TrackKind::Audio);
                for clip_node in track_node.children_named("clipitem") {
                    if let Some(clip) = parse_clipitem(clip_node, rate) {
                        track.clips.push(clip);
                    }
                }
                seq.audio_tracks.push(track);
            }
        }
    }

    for marker_node in seq_node.children_named("marker") {
        let comment = marker_node.child_text("comment").unwrap_or("").to_string();
        let name = marker_node.child_text("name").unwrap_or("Marker").to_string();
        let in_f: i64 = marker_node
            .child_text("in")
            .and_then(|t| t.parse().ok())
            .unwrap_or(0);
        seq.markers.push(TimelineMarker {
            id: format!("marker-{}", seq.markers.len() + 1),
            time: rate.tick_of(in_f),
            name,
            comment,
            color: "green".into(),
        });
    }

    Ok(seq)
}

fn parse_clipitem(node: &XmlNode, rate: FrameRate) -> Option<TimelineClip> {
    let name = node.child_text("name").unwrap_or("Clip").to_string();
    let start_f: i64 = node.child_text("start")?.parse().ok()?;
    let end_f: i64 = node.child_text("end")?.parse().ok()?;
    let in_f: i64 = node.child_text("in").and_then(|t| t.parse().ok()).unwrap_or(0);
    let out_f: i64 = node
        .child_text("out")
        .and_then(|t| t.parse().ok())
        .unwrap_or(end_f - start_f);

    let mut path = String::new();
    if let Some(file_node) = node.child("file") {
        if let Some(p) = file_node.child_text("pathurl") {
            path = p.to_string();
        }
    }

    let dur_f = (end_f - start_f).max(0);
    Some(TimelineClip {
        id: node.attr("id").unwrap_or("clip").to_string(),
        name,
        reel: String::new(),
        source_path: path,
        timeline_start: rate.tick_of(start_f),
        timeline_duration: rate.tick_of(dur_f),
        source_in: rate.tick_of(in_f),
        source_out: rate.tick_of(out_f),
        speed: 1.0,
    })
}

/// Serialize a TimelineSequence to FCP7 XML (`xmeml`) string.
pub fn write_fcp7_xml(seq: &TimelineSequence) -> String {
    let mut w = XmlWriter::new(Some("xmeml"));
    w.open("xmeml", &[("version", "5")]);
    w.open("sequence", &[("id", &seq.id)]);
    w.text("name", &seq.name);

    w.open("rate", &[]);
    let tb = seq.frame_rate.as_f64().round() as i64;
    w.text("timebase", &tb.to_string());
    w.text(
        "ntsc",
        if seq.frame_rate.is_drop_frame_compatible() {
            "TRUE"
        } else {
            "FALSE"
        },
    );
    w.close(); // rate

    w.open("media", &[]);
    w.open("video", &[]);
    for track in &seq.video_tracks {
        w.open("track", &[]);
        for clip in &track.clips {
            write_clipitem(&mut w, clip, seq.frame_rate);
        }
        w.close(); // track
    }
    w.close(); // video

    w.open("audio", &[]);
    for track in &seq.audio_tracks {
        w.open("track", &[]);
        for clip in &track.clips {
            write_clipitem(&mut w, clip, seq.frame_rate);
        }
        w.close(); // track
    }
    w.close(); // audio
    w.close(); // media

    for marker in &seq.markers {
        w.open("marker", &[]);
        w.text("name", &marker.name);
        w.text("comment", &marker.comment);
        let f = seq.frame_rate.frame_at(marker.time);
        w.text("in", &f.to_string());
        w.text("out", &(f + 1).to_string());
        w.close(); // marker
    }

    w.close(); // sequence
    w.close(); // xmeml
    w.finish()
}

fn write_clipitem(w: &mut XmlWriter, clip: &TimelineClip, rate: FrameRate) {
    w.open("clipitem", &[("id", &clip.id)]);
    w.text("name", &clip.name);
    let start_f = rate.frame_at(clip.timeline_start);
    let dur_f = rate.frame_at(clip.timeline_duration);
    let in_f = rate.frame_at(clip.source_in);
    let out_f = rate.frame_at(clip.source_out);

    w.text("start", &start_f.to_string());
    w.text("end", &(start_f + dur_f).to_string());
    w.text("in", &in_f.to_string());
    w.text("out", &out_f.to_string());

    if !clip.source_path.is_empty() {
        w.open("file", &[("id", &format!("{}-file", clip.id))]);
        w.text("name", &clip.name);
        w.text("pathurl", &clip.source_path);
        w.close(); // file
    }
    w.close(); // clipitem
}

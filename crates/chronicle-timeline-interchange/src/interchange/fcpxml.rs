//! Final Cut Pro X XML (`.fcpxml`) parser and writer.
//!
//! Grounded in Apple FCPXML specifications (versions 1.9–1.11).

use crate::model::{TimelineClip, TimelineMarker, TimelineSequence, TimelineTrack, TrackKind};
use crate::time::{FrameRate, TICKS_PER_SECOND, Tick};
use crate::xml::{XmlNode, XmlWriter};

/// Parse an FCPXML time string (`"1001/24000s"`, `"5s"`, `"0s"`, `"3.5s"`) into a `Tick`.
pub fn parse_fcpxml_time(s: &str) -> Option<Tick> {
    let s = s.trim().strip_suffix('s').unwrap_or(s.trim());
    if s.is_empty() {
        return None;
    }
    if let Some((a, b)) = s.split_once('/') {
        let a: i128 = a.trim().parse().ok()?;
        let b: i128 = b.trim().parse().ok()?;
        if b == 0 {
            return None;
        }
        let n = a * TICKS_PER_SECOND as i128;
        Some(Tick((n / b) as i64))
    } else if let Ok(sec) = s.parse::<f64>() {
        Some(Tick::from_seconds(sec))
    } else {
        None
    }
}

/// Format a `Tick` as an FCPXML rational time string.
pub fn format_fcpxml_time(t: Tick, rate: FrameRate) -> String {
    let f = rate.frame_at(t);
    if rate.tick_of(f) == t {
        let num = f * rate.den;
        let den = rate.num;
        format!("{num}/{den}s")
    } else {
        format!("{:.6}s", t.as_seconds())
    }
}

/// Parse an FCPXML document string into a TimelineSequence.
pub fn parse_fcpxml(xml: &str) -> Result<TimelineSequence, String> {
    let root = XmlNode::parse(xml)?;
    if root.tag != "fcpxml" {
        return Err("Root element is not <fcpxml>".into());
    }

    // Look for sequence in library -> event -> project -> sequence
    let mut seq_node_opt = None;
    for elem in &root.children {
        if elem.tag == "library" {
            for event in &elem.children {
                if event.tag == "event" {
                    for proj in &event.children {
                        if proj.tag == "project" {
                            if let Some(s) = proj.child("sequence") {
                                seq_node_opt = Some(s);
                                break;
                            }
                        }
                    }
                }
            }
        } else if elem.tag == "project" {
            if let Some(s) = elem.child("sequence") {
                seq_node_opt = Some(s);
                break;
            }
        } else if elem.tag == "sequence" {
            seq_node_opt = Some(elem);
            break;
        }
    }

    let seq_node = seq_node_opt.ok_or("No <sequence> found in FCPXML")?;
    let dur = seq_node
        .attr("duration")
        .and_then(parse_fcpxml_time)
        .unwrap_or(Tick::ZERO);

    let mut seq = TimelineSequence {
        id: "fcpxml-sequence".into(),
        name: "FCPX Sequence".into(),
        frame_rate: FrameRate::FPS_24,
        drop_frame: false,
        start_time: Tick::ZERO,
        duration: dur,
        video_tracks: Vec::new(),
        audio_tracks: Vec::new(),
        markers: Vec::new(),
    };

    let spine = seq_node.child("spine").ok_or("No <spine> in <sequence>")?;
    let mut primary_track = TimelineTrack::new("Primary Storyline", TrackKind::Video);
    let mut cur_time = Tick::ZERO;

    for item in &spine.children {
        let item_dur = item
            .attr("duration")
            .and_then(parse_fcpxml_time)
            .unwrap_or(Tick::ZERO);
        let item_start = item
            .attr("start")
            .and_then(parse_fcpxml_time)
            .unwrap_or(Tick::ZERO);

        match item.tag.as_str() {
            "clip" | "asset-clip" | "video" => {
                let name = item.attr("name").unwrap_or("Clip").to_string();
                let clip = TimelineClip {
                    id: item.attr("ref").unwrap_or("asset").to_string(),
                    name,
                    reel: String::new(),
                    source_path: String::new(),
                    timeline_start: cur_time,
                    timeline_duration: item_dur,
                    source_in: item_start,
                    source_out: item_start + item_dur,
                    speed: 1.0,
                };
                primary_track.clips.push(clip);
                cur_time = cur_time + item_dur;
            }
            "gap" => {
                cur_time = cur_time + item_dur;
            }
            _ => {}
        }

        for marker_node in item.children_named("marker") {
            let m_start = marker_node
                .attr("start")
                .and_then(parse_fcpxml_time)
                .unwrap_or(cur_time);
            let m_name = marker_node.attr("value").unwrap_or("Marker").to_string();
            seq.markers.push(TimelineMarker {
                id: format!("marker-{}", seq.markers.len() + 1),
                time: m_start,
                name: m_name,
                comment: String::new(),
                color: "cyan".into(),
            });
        }
    }

    if !primary_track.clips.is_empty() {
        seq.video_tracks.push(primary_track);
    }

    Ok(seq)
}

/// Serialize a TimelineSequence to an FCPXML document string.
pub fn write_fcpxml(seq: &TimelineSequence) -> String {
    let mut w = XmlWriter::new(Some("fcpxml"));
    w.open("fcpxml", &[("version", "1.10")]);

    w.open("resources", &[]);
    let rate_str = format_fcpxml_time(seq.frame_rate.tick_of(1), seq.frame_rate);
    w.empty(
        "format",
        &[
            ("id", "r1"),
            ("frameDuration", &rate_str),
            ("width", "1920"),
            ("height", "1080"),
        ],
    );
    w.close(); // resources

    w.open("library", &[]);
    w.open("event", &[("name", "Event")]);
    w.open("project", &[("name", &seq.name)]);

    let dur_str = format_fcpxml_time(seq.duration, seq.frame_rate);
    w.open(
        "sequence",
        &[
            ("format", "r1"),
            ("duration", &dur_str),
        ],
    );

    w.open("spine", &[]);
    if let Some(track) = seq.video_tracks.first() {
        for clip in &track.clips {
            let offset_str = format_fcpxml_time(clip.timeline_start, seq.frame_rate);
            let dur_str = format_fcpxml_time(clip.timeline_duration, seq.frame_rate);
            let start_str = format_fcpxml_time(clip.source_in, seq.frame_rate);

            w.open(
                "clip",
                &[
                    ("name", &clip.name),
                    ("offset", &offset_str),
                    ("duration", &dur_str),
                    ("start", &start_str),
                ],
            );
            w.close(); // clip
        }
    }
    w.close(); // spine
    w.close(); // sequence
    w.close(); // project
    w.close(); // event
    w.close(); // library

    w.close(); // fcpxml
    w.finish()
}

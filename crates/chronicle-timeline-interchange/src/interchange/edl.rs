//! CMX 3600 Edit Decision List (EDL) parser and serializer.
//!
//! Grounded in SMPTE and CMX 3600 industry conventions for interchange.

use std::fmt::Write as _;

use crate::model::{TimelineClip, TimelineMarker, TimelineSequence, TimelineTrack, TrackKind};
use crate::time::{FrameRate, Tick, format_timecode, parse_timecode};

/// Parse a CMX 3600 EDL document into a TimelineSequence.
pub fn parse_edl(text: &str, default_rate: FrameRate) -> Result<TimelineSequence, String> {
    let mut seq = TimelineSequence::default();
    seq.frame_rate = default_rate;

    let mut video_track = TimelineTrack::new("V1", TrackKind::Video);
    let mut audio_track1 = TimelineTrack::new("A1", TrackKind::Audio);
    let mut audio_track2 = TimelineTrack::new("A2", TrackKind::Audio);

    let mut current_clip_name = String::new();
    let mut current_source_file = String::new();
    let mut drop_frame = false;

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("TITLE:") {
            seq.name = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = line.strip_prefix("FCM:") {
            if rest.contains("DROP") {
                drop_frame = true;
                seq.drop_frame = true;
            }
            continue;
        }

        if let Some(rest) = line.strip_prefix("* FROM CLIP NAME:") {
            current_clip_name = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = line.strip_prefix("* SOURCE FILE:") {
            current_source_file = rest.trim().to_string();
            continue;
        }

        if let Some(rest) = line.strip_prefix("* LOC:") {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Some((frames, _)) = parse_timecode(parts[0], seq.frame_rate) {
                    let color = parts[1].to_string();
                    let comment = parts[2..].join(" ");
                    seq.markers.push(TimelineMarker {
                        id: format!("marker_{}", seq.markers.len() + 1),
                        time: seq.frame_rate.tick_of(frames),
                        name: "Marker".into(),
                        comment,
                        color,
                    });
                }
            }
            continue;
        }

        // Event line format:
        // 001  REEL  V  C  00:00:00:00 00:00:05:00 01:00:00:00 01:00:05:00
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 8 && parts[0].chars().all(|c| c.is_ascii_digit()) {
            let reel = parts[1].to_string();
            let channel = parts[2];
            let _trans = parts[3];

            // In/out indices depend on whether dissolve/wipe length is present
            let (src_in_s, src_out_s, rec_in_s, rec_out_s) = if parts.len() >= 9 && parts[4].chars().all(|c| c.is_ascii_digit()) {
                (parts[5], parts[6], parts[7], parts[8])
            } else if parts.len() >= 8 {
                (parts[4], parts[5], parts[6], parts[7])
            } else {
                continue;
            };

            let src_in_frames = parse_timecode(src_in_s, seq.frame_rate).map(|(f, _)| f).unwrap_or(0);
            let src_out_frames = parse_timecode(src_out_s, seq.frame_rate).map(|(f, _)| f).unwrap_or(0);
            let rec_in_frames = parse_timecode(rec_in_s, seq.frame_rate).map(|(f, _)| f).unwrap_or(0);
            let rec_out_frames = parse_timecode(rec_out_s, seq.frame_rate).map(|(f, _)| f).unwrap_or(0);

            let dur_frames = (rec_out_frames - rec_in_frames).max(0);
            let name = if !current_clip_name.is_empty() {
                current_clip_name.clone()
            } else {
                reel.clone()
            };

            let clip = TimelineClip {
                id: format!("clip_{}", parts[0]),
                name,
                reel: reel.clone(),
                source_path: current_source_file.clone(),
                timeline_start: seq.frame_rate.tick_of(rec_in_frames),
                timeline_duration: seq.frame_rate.tick_of(dur_frames),
                source_in: seq.frame_rate.tick_of(src_in_frames),
                source_out: seq.frame_rate.tick_of(src_out_frames),
                speed: 1.0,
            };

            match channel {
                "V" => video_track.clips.push(clip),
                "A" | "A1" => audio_track1.clips.push(clip),
                "A2" => audio_track2.clips.push(clip),
                "B" | "AA" | "AA/V" => {
                    video_track.clips.push(clip.clone());
                    audio_track1.clips.push(clip.clone());
                    audio_track2.clips.push(clip);
                }
                _ => video_track.clips.push(clip),
            }

            current_clip_name.clear();
            current_source_file.clear();
        }
    }

    if !video_track.clips.is_empty() {
        seq.video_tracks.push(video_track);
    }
    if !audio_track1.clips.is_empty() {
        seq.audio_tracks.push(audio_track1);
    }
    if !audio_track2.clips.is_empty() {
        seq.audio_tracks.push(audio_track2);
    }

    Ok(seq)
}

/// Serialize a TimelineSequence to CMX 3600 EDL text format.
pub fn write_edl(seq: &TimelineSequence) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "TITLE: {}", seq.name);
    let _ = writeln!(
        out,
        "FCM: {}",
        if seq.drop_frame {
            "DROP FRAME"
        } else {
            "NON-DROP FRAME"
        }
    );
    let _ = writeln!(out);

    let mut event_idx = 1;

    for track in &seq.video_tracks {
        for clip in &track.clips {
            let reel = if clip.reel.is_empty() { "AX" } else { &clip.reel };
            let src_in = format_timecode(seq.frame_rate.frame_at(clip.source_in), seq.frame_rate, seq.drop_frame);
            let src_out = format_timecode(seq.frame_rate.frame_at(clip.source_out), seq.frame_rate, seq.drop_frame);
            let rec_in = format_timecode(seq.frame_rate.frame_at(clip.timeline_start), seq.frame_rate, seq.drop_frame);
            let rec_end = clip.timeline_start + clip.timeline_duration;
            let rec_out = format_timecode(seq.frame_rate.frame_at(rec_end), seq.frame_rate, seq.drop_frame);

            let _ = writeln!(
                out,
                "{event_idx:03}  {reel:<8} V     C        {src_in} {src_out} {rec_in} {rec_out}"
            );
            if !clip.name.is_empty() {
                let _ = writeln!(out, "* FROM CLIP NAME: {}", clip.name);
            }
            if !clip.source_path.is_empty() {
                let _ = writeln!(out, "* SOURCE FILE: {}", clip.source_path);
            }
            event_idx += 1;
        }
    }

    for track in &seq.audio_tracks {
        for clip in &track.clips {
            let reel = if clip.reel.is_empty() { "AX" } else { &clip.reel };
            let src_in = format_timecode(seq.frame_rate.frame_at(clip.source_in), seq.frame_rate, seq.drop_frame);
            let src_out = format_timecode(seq.frame_rate.frame_at(clip.source_out), seq.frame_rate, seq.drop_frame);
            let rec_in = format_timecode(seq.frame_rate.frame_at(clip.timeline_start), seq.frame_rate, seq.drop_frame);
            let rec_end = clip.timeline_start + clip.timeline_duration;
            let rec_out = format_timecode(seq.frame_rate.frame_at(rec_end), seq.frame_rate, seq.drop_frame);

            let _ = writeln!(
                out,
                "{event_idx:03}  {reel:<8} A     C        {src_in} {src_out} {rec_in} {rec_out}"
            );
            if !clip.name.is_empty() {
                let _ = writeln!(out, "* FROM CLIP NAME: {}", clip.name);
            }
            if !clip.source_path.is_empty() {
                let _ = writeln!(out, "* SOURCE FILE: {}", clip.source_path);
            }
            event_idx += 1;
        }
    }

    out
}

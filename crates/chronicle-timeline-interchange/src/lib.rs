//! `chronicle-timeline-interchange` — Categorical timeline interchange and closed captions.
//!
//! Provides:
//! - CMX 3600 EDL parser and serializer (Cuts, Dissolves, Wipes, Multi-channel audio, Markers)
//! - Final Cut Pro 7 XML (`xmeml`) parser and writer
//! - Final Cut Pro X XML (`.fcpxml`) parser and writer
//! - OpenTimelineIO (`.otio`) JSON parser and writer
//! - Advanced Authoring Format (AAF) interchange object model
//! - SubRip (`.srt`) and WebVTT (`.vtt`) subtitle parser and writer
//! - CEA-608 Line 21 closed caption decoder, parity checker, and 15x32 screen buffer
//! - CEA-708 DTVCC window manager and packet parser
//! - MacCaption MCC (SMPTE ST 334-2 CDP) parser and exporter
//! - Frame-boundary snapping, drift compensation, and caption cue aligner
//!
//! Grounded in international human rights instruments protecting freedom of expression,
//! access to information, and cultural participation (UDHR Articles 12, 19, 27; ICCPR Articles 17, 19;
//! ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod captions;
pub mod interchange;
pub mod model;
pub mod time;
pub mod xml;

pub use captions::{
    AlignOptions, CaptionCue, CaptionDocument, CaptionFormat, align_cues, detect_caption_format,
    parse_mcc, parse_srt, parse_vtt, write_mcc, write_srt, write_vtt,
};
pub use interchange::{
    AafDocument, parse_edl, parse_fcp7_xml, parse_fcpxml, parse_otio, write_edl, write_fcp7_xml,
    write_fcpxml, write_otio,
};
pub use model::{
    TimelineClip, TimelineMarker, TimelineProject, TimelineSequence, TimelineTrack,
    TimelineTransition, TrackKind, TransitionKind,
};
pub use time::{
    FrameRate, TICKS_PER_SECOND, Tick, format_clock, format_timecode, parse_clock, parse_timecode,
};
pub use xml::{XmlNode, XmlWriter};

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_EDL: &str = "TITLE: Test Cut
FCM: NON-DROP FRAME

001  REEL01   V     C        00:00:00:00 00:00:05:00 01:00:00:00 01:00:05:00
* FROM CLIP NAME: scene1.mov
* SOURCE FILE: /media/scene1.mov
* LOC: 01:00:02:12 BLUE    take 2 best

002  REEL02   V     C        00:00:02:00 00:00:06:00 01:00:05:00 01:00:09:00
* FROM CLIP NAME: scene2.mov
";

    #[test]
    fn test_edl_parsing_and_writing() {
        let seq = parse_edl(SAMPLE_EDL, FrameRate::FPS_24).unwrap();
        assert_eq!(seq.name, "Test Cut");
        assert_eq!(seq.video_tracks.len(), 1);
        let track = &seq.video_tracks[0];
        assert_eq!(track.clips.len(), 2);
        assert_eq!(track.clips[0].name, "scene1.mov");
        assert_eq!(track.clips[0].reel, "REEL01");
        assert_eq!(seq.markers.len(), 1);
        assert_eq!(seq.markers[0].color, "BLUE");

        let edl_out = write_edl(&seq);
        assert!(edl_out.contains("TITLE: Test Cut"));
        assert!(edl_out.contains("001  REEL01"));
        assert!(edl_out.contains("scene1.mov"));
    }

    #[test]
    fn test_otio_json_roundtrip() {
        let mut seq = TimelineSequence::default();
        seq.name = "My OTIO Sequence".into();
        let mut v_track = TimelineTrack::new("V1", TrackKind::Video);
        v_track.clips.push(TimelineClip {
            id: "clip-1".into(),
            name: "shot_01.exr".into(),
            reel: "R01".into(),
            source_path: "file:///media/shot_01.exr".into(),
            timeline_start: Tick::ZERO,
            timeline_duration: FrameRate::FPS_24.tick_of(48),
            source_in: Tick::ZERO,
            source_out: FrameRate::FPS_24.tick_of(48),
            speed: 1.0,
        });
        seq.video_tracks.push(v_track);

        let otio_json = write_otio(&seq);
        assert!(otio_json.contains("Timeline.1"));
        assert!(otio_json.contains("shot_01.exr"));

        let roundtrip = parse_otio(&otio_json).unwrap();
        assert_eq!(roundtrip.name, seq.name);
        assert_eq!(roundtrip.video_tracks.len(), 1);
        assert_eq!(roundtrip.video_tracks[0].clips[0].name, "shot_01.exr");
    }

    #[test]
    fn test_fcp7_xml_roundtrip() {
        let mut seq = TimelineSequence::default();
        seq.name = "FCP7 Project".into();
        let mut track = TimelineTrack::new("V1", TrackKind::Video);
        track.clips.push(TimelineClip {
            id: "clip1".into(),
            name: "interview.mov".into(),
            reel: "AX".into(),
            source_path: "/media/interview.mov".into(),
            timeline_start: Tick::ZERO,
            timeline_duration: FrameRate::FPS_24.tick_of(72),
            source_in: Tick::ZERO,
            source_out: FrameRate::FPS_24.tick_of(72),
            speed: 1.0,
        });
        seq.video_tracks.push(track);

        let xml = write_fcp7_xml(&seq);
        assert!(xml.contains("<xmeml version=\"5\">"));
        assert!(xml.contains("interview.mov"));

        let parsed = parse_fcp7_xml(&xml).unwrap();
        assert_eq!(parsed.name, "FCP7 Project");
        assert_eq!(parsed.video_tracks[0].clips[0].name, "interview.mov");
    }

    #[test]
    fn test_fcpxml_roundtrip() {
        let mut seq = TimelineSequence::default();
        seq.name = "FCPX Project".into();
        let mut track = TimelineTrack::new("Spine", TrackKind::Video);
        track.clips.push(TimelineClip {
            id: "asset1".into(),
            name: "landscape.mov".into(),
            reel: "A1".into(),
            source_path: "".into(),
            timeline_start: Tick::ZERO,
            timeline_duration: FrameRate::FPS_24.tick_of(100),
            source_in: Tick::ZERO,
            source_out: FrameRate::FPS_24.tick_of(100),
            speed: 1.0,
        });
        seq.video_tracks.push(track);

        let xml = write_fcpxml(&seq);
        assert!(xml.contains("<fcpxml version=\"1.10\">"));
        assert!(xml.contains("landscape.mov"));

        let parsed = parse_fcpxml(&xml).unwrap();
        assert_eq!(parsed.video_tracks.len(), 1);
        assert_eq!(parsed.video_tracks[0].clips[0].name, "landscape.mov");
    }

    #[test]
    fn test_srt_roundtrip() {
        let srt_input = "1\n00:00:01,000 --> 00:00:03,500\nHello, world!\nSecond line\n\n2\n00:00:04,200 --> 00:00:06,800\nBroadcast caption test.\n";
        let doc = parse_srt(srt_input).unwrap();
        assert_eq!(doc.cues.len(), 2);
        assert_eq!(doc.cues[0].text, "Hello, world!\nSecond line");
        assert_eq!(doc.cues[1].text, "Broadcast caption test.");

        let srt_output = write_srt(&doc);
        assert!(srt_output.contains("00:00:01,000 --> 00:00:03,500"));
        assert!(srt_output.contains("Hello, world!"));
    }

    #[test]
    fn test_vtt_roundtrip() {
        let vtt_input = "WEBVTT\n\n1\n00:00:01.000 --> 00:00:03.500 line:80%\nHello from WebVTT!\n\n2\n00:00:04.000 --> 00:00:06.000\nAccessibility caption.\n";
        let doc = parse_vtt(vtt_input).unwrap();
        assert_eq!(doc.cues.len(), 2);
        assert_eq!(doc.cues[0].text, "Hello from WebVTT!");
        assert_eq!(doc.cues[0].settings, "line:80%");

        let vtt_output = write_vtt(&doc);
        assert!(vtt_output.starts_with("WEBVTT"));
        assert!(vtt_output.contains("00:00:01.000 --> 00:00:03.500 line:80%"));
    }

    #[test]
    fn test_cea608_parity_and_decode() {
        let c = 'A' as u8;
        let with_p = captions::cea608::with_parity(c);
        let stripped = captions::cea608::strip_parity(with_p).unwrap();
        assert_eq!(c, stripped);

        // ResumeCaptionLoading: 0x14, 0x20
        let p1 = captions::cea608::with_parity(0x14);
        let p2 = captions::cea608::with_parity(0x20);
        let cmd = captions::cea608::decode_pair(p1, p2);
        assert_eq!(cmd, captions::cea608::Cea608Command::ResumeCaptionLoading);
    }

    #[test]
    fn test_caption_align_and_overlap_resolution() {
        let mut doc = CaptionDocument {
            cues: vec![
                CaptionCue {
                    id: "1".into(),
                    start: Tick::from_millis(100),
                    end: Tick::from_millis(1500),
                    text: "Cue 1".into(),
                    settings: String::new(),
                },
                CaptionCue {
                    id: "2".into(),
                    start: Tick::from_millis(1200), // Overlaps cue 1
                    end: Tick::from_millis(2500),
                    text: "Cue 2".into(),
                    settings: String::new(),
                },
            ],
        };

        let opts = AlignOptions {
            frame_rate: FrameRate::FPS_24,
            min_duration_ticks: Tick::from_millis(500),
            snap_to_frames: true,
            resolve_overlaps: true,
        };
        align_cues(&mut doc, &opts);

        assert!(doc.cues[0].end <= doc.cues[1].start);
    }
}

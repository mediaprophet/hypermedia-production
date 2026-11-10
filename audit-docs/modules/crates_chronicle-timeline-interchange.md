# `crates/chronicle-timeline-interchange`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Caption formats (SRT, WebVTT, CEA-608, CEA-708, and MacCaption MCC)._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for AlignOptions::default` | 0 | `crates/chronicle-timeline-interchange/src/captions/align.rs:18` | - |  |
| `align_cues` | 2 | `crates/chronicle-timeline-interchange/src/captions/align.rs:29` | - | Align all cues in a caption document according to the specified options. |
| `with_parity` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:6` | - | Add odd parity bit to a 7-bit value. |
| `strip_parity` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:16` | - | Verify and strip odd parity bit from a byte. |
| `basic_char` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:25` | - | Decode basic character set (0x20..=0x7F). |
| `decode_pair` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:60` | - |  |
| `impl Default for ScreenBuffer::default` | 0 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:143` | - |  |
| `impl ScreenBuffer::clear` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:153` | - |  |
| `impl ScreenBuffer::write_char` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:159` | - |  |
| `impl ScreenBuffer::carriage_return` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:166` | - |  |
| `impl ScreenBuffer::render_text` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea608.rs:175` | - |  |
| `parse_packet_header` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:12` | - |  |
| `impl DtvccWindow::new` | 4 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:54` | - |  |
| `impl DtvccWindow::clear` | 1 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:64` | - |  |
| `impl DtvccWindow::append` | 2 | `crates/chronicle-timeline-interchange/src/captions/cea708.rs:68` | - |  |
| `parse_mcc` | 1 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:10` | - |  |
| `decode_mcc_hex` | 1 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:79` | - |  |
| `write_mcc` | 2 | `crates/chronicle-timeline-interchange/src/captions/mcc.rs:133` | - |  |
| `detect_caption_format` | 1 | `crates/chronicle-timeline-interchange/src/captions/mod.rs:44` | - |  |
| `parse_srt` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:10` | - |  |
| `parse_timing_line` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:59` | - |  |
| `write_srt` | 1 | `crates/chronicle-timeline-interchange/src/captions/srt.rs:72` | - |  |
| `parse_vtt` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:10` | - |  |
| `parse_vtt_timing_line` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:81` | - |  |
| `write_vtt` | 1 | `crates/chronicle-timeline-interchange/src/captions/vtt.rs:93` | - |  |
| `impl std::fmt::Debug for MobId::fmt` | 2 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:15` | - |  |
| `impl Default for Segment::default` | 0 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:76` | - |  |
| `impl AafDocument::from_sequence` | 1 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:121` | - | Convert a TimelineSequence into an AAF Document model. |
| `impl AafDocument::to_sequence` | 1 | `crates/chronicle-timeline-interchange/src/interchange/aaf.rs:158` | - | Convert this AAF Document model into a TimelineSequence. |
| `parse_edl` | 2 | `crates/chronicle-timeline-interchange/src/interchange/edl.rs:11` | - | Parse a CMX 3600 EDL document into a TimelineSequence. |
| `write_edl` | 1 | `crates/chronicle-timeline-interchange/src/interchange/edl.rs:142` | - | Serialize a TimelineSequence to CMX 3600 EDL text format. |
| `parse_fcp7_xml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:10` | - | Parse FCP7 XML text into a TimelineSequence. |
| `parse_clipitem` | 2 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:112` | graph |  |
| `write_fcp7_xml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:144` | - | Serialize a TimelineSequence to FCP7 XML (`xmeml`) string. |
| `write_clipitem` | 3 | `crates/chronicle-timeline-interchange/src/interchange/fcp7.rs:200` | - |  |
| `parse_fcpxml_time` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:10` | - | Parse an FCPXML time string (`"1001/24000s"`, `"5s"`, `"0s"`, `"3.5s"`) into a `Tick`. |
| `format_fcpxml_time` | 2 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:31` | - | Format a `Tick` as an FCPXML rational time string. |
| `parse_fcpxml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:43` | - | Parse an FCPXML document string into a TimelineSequence. |
| `write_fcpxml` | 1 | `crates/chronicle-timeline-interchange/src/interchange/fcpxml.rs:155` | - | Serialize a TimelineSequence to an FCPXML document string. |
| `parse_otio` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:11` | - | Parse an OpenTimelineIO JSON string into a TimelineSequence. |
| `read_rational_time` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:117` | - |  |
| `write_rational_time` | 2 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:126` | - |  |
| `write_time_range` | 3 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:135` | - |  |
| `write_otio` | 1 | `crates/chronicle-timeline-interchange/src/interchange/otio.rs:144` | - | Serialize a TimelineSequence to OpenTimelineIO JSON format. |
| `test_edl_parsing_and_writing` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:61` | - |  |
| `test_otio_json_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:79` | - |  |
| `test_fcp7_xml_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:107` | - |  |
| `test_fcpxml_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:134` | - |  |
| `test_srt_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:161` | - |  |
| `test_vtt_roundtrip` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:174` | - |  |
| `test_cea608_parity_and_decode` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:187` | - |  |
| `test_caption_align_and_overlap_resolution` | 0 | `crates/chronicle-timeline-interchange/src/lib.rs:201` | - |  |
| `impl Default for TimelineSequence::default` | 0 | `crates/chronicle-timeline-interchange/src/model.rs:32` | - |  |
| `impl TimelineTrack::new` | 2 | `crates/chronicle-timeline-interchange/src/model.rs:67` | - |  |
| `impl Tick::from_rational` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:17` | - |  |
| `impl Tick::to_rational` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:25` | - |  |
| `impl Tick::from_seconds` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:34` | - |  |
| `impl Tick::as_seconds` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:38` | - |  |
| `impl Tick::from_millis` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:42` | - |  |
| `impl Tick::as_millis` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:46` | - |  |
| `impl std::ops::Add for Tick::add` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:53` | - |  |
| `impl std::ops::Sub for Tick::sub` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:60` | - |  |
| `impl Default for FrameRate::default` | 0 | `crates/chronicle-timeline-interchange/src/time.rs:72` | - |  |
| `impl FrameRate::as_f64` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:87` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:91` | - |  |
| `impl FrameRate::tick_of` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:96` | - |  |
| `impl FrameRate::is_drop_frame_compatible` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:100` | - |  |
| `format_timecode` | 3 | `crates/chronicle-timeline-interchange/src/time.rs:106` | - | Convert frame count to SMPTE timecode (HH:MM:SS:FF or HH:MM:SS;FF). |
| `parse_timecode` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:142` | - | Parse a timecode string (`HH:MM:SS:FF` or `HH:MM:SS;FF`) into frame count. |
| `parse_clock` | 1 | `crates/chronicle-timeline-interchange/src/time.rs:167` | - | Parse clock format: `HH:MM:SS.mmm` or `HH:MM:SS,mmm` into `Tick`. |
| `format_clock` | 2 | `crates/chronicle-timeline-interchange/src/time.rs:200` | - | Format `Tick` as SRT/WebVTT clock (`HH:MM:SS,mmm` or `HH:MM:SS.mmm`). |
| `frame_at_inverts_tick_of` | 0 | `crates/chronicle-timeline-interchange/src/time.rs:216` | - |  |
| `escape_xml` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:8` | - |  |
| `unescape_xml` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:23` | - |  |
| `impl XmlWriter::new` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:74` | - |  |
| `impl XmlWriter::indent` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:85` | - |  |
| `impl XmlWriter::open` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:91` | - |  |
| `impl XmlWriter::close` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:102` | - |  |
| `impl XmlWriter::empty` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:108` | - |  |
| `impl XmlWriter::text` | 3 | `crates/chronicle-timeline-interchange/src/xml.rs:118` | - |  |
| `impl XmlWriter::finish` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:123` | - |  |
| `impl XmlNode::new` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:141` | - |  |
| `impl XmlNode::attr` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:150` | - |  |
| `impl XmlNode::child` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:157` | - |  |
| `impl XmlNode::children_named` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:161` | - |  |
| `impl XmlNode::child_text` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:165` | - |  |
| `impl XmlNode::parse` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:170` | - | Parse an XML text document into an `XmlNode` tree. |
| `tokenize` | 1 | `crates/chronicle-timeline-interchange/src/xml.rs:203` | - |  |
| `parse_attrs` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:274` | - |  |
| `parse_node` | 2 | `crates/chronicle-timeline-interchange/src/xml.rs:292` | graph |  |

# `crates/media-isobmff`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Clean-room ISO Base Media File Format (MP4, ISO/IEC 14496-12/14/15) and QuickTime (MOV) support.

- **Demux:** [`open`] parses box headers plus `moov` (and `moof` for fragmented files) from a
[`ByteSource`] and builds per-track [`Sample`] tables; media data is read on demand with
[`Mp4File::read_sample`].
- **Mux:** [`Mp4Writer`] writes progressive MP4/MOV (optionally faststart);
[`FragmentedWriter`] writes fragmented MP4.

See the crate README for supported boxes and limitations._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl FourCc::new` | 1 | `crates/media-isobmff/src/bytes.rs:12` | - |  |
| `impl FourCc::as_u32` | 1 | `crates/media-isobmff/src/bytes.rs:15` | - |  |
| `impl FourCc::to_string_lossy` | 1 | `crates/media-isobmff/src/bytes.rs:19` | - | Lossy string form; `©` (0xA9) is mapped to U+00A9, other non-printables to `?`. |
| `impl fmt::Display for FourCc::fmt` | 2 | `crates/media-isobmff/src/bytes.rs:32` | - |  |
| `impl fmt::Debug for FourCc::fmt` | 2 | `crates/media-isobmff/src/bytes.rs:37` | - |  |
| `impl From<&[u8; 4]> for FourCc::from` | 1 | `crates/media-isobmff/src/bytes.rs:42` | - |  |
| `impl PartialEq<[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:47` | - |  |
| `impl PartialEq<&[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:52` | - |  |
| `impl PartialEq<&&[u8; 4]> for FourCc::eq` | 2 | `crates/media-isobmff/src/bytes.rs:57` | - |  |
| `impl Cur<'a>::new` | 2 | `crates/media-isobmff/src/bytes.rs:71` | - |  |
| `impl Cur<'a>::remaining` | 1 | `crates/media-isobmff/src/bytes.rs:74` | - |  |
| `impl Cur<'a>::rest` | 1 | `crates/media-isobmff/src/bytes.rs:77` | - |  |
| `impl Cur<'a>::bytes` | 2 | `crates/media-isobmff/src/bytes.rs:80` | - |  |
| `impl Cur<'a>::skip` | 2 | `crates/media-isobmff/src/bytes.rs:86` | - |  |
| `impl Cur<'a>::array` | 1 | `crates/media-isobmff/src/bytes.rs:89` | - |  |
| `impl Cur<'a>::u8` | 1 | `crates/media-isobmff/src/bytes.rs:95` | - |  |
| `impl Cur<'a>::u16` | 1 | `crates/media-isobmff/src/bytes.rs:98` | - |  |
| `impl Cur<'a>::i16` | 1 | `crates/media-isobmff/src/bytes.rs:101` | - |  |
| `impl Cur<'a>::u24` | 1 | `crates/media-isobmff/src/bytes.rs:104` | - |  |
| `impl Cur<'a>::u32` | 1 | `crates/media-isobmff/src/bytes.rs:108` | - |  |
| `impl Cur<'a>::i32` | 1 | `crates/media-isobmff/src/bytes.rs:111` | - |  |
| `impl Cur<'a>::u64` | 1 | `crates/media-isobmff/src/bytes.rs:114` | - |  |
| `impl Cur<'a>::i64` | 1 | `crates/media-isobmff/src/bytes.rs:117` | - |  |
| `impl Cur<'a>::f64` | 1 | `crates/media-isobmff/src/bytes.rs:120` | - |  |
| `impl Cur<'a>::fourcc` | 1 | `crates/media-isobmff/src/bytes.rs:123` | - |  |
| `impl Cur<'a>::full_header` | 1 | `crates/media-isobmff/src/bytes.rs:127` | - | Full-box header: (version, flags). |
| `impl Cur<'a>::count` | 2 | `crates/media-isobmff/src/bytes.rs:132` | - | Read a count and verify that `count * entry_size` bytes remain (allocation guard). |
| `impl Cur<'a>::check_count` | 3 | `crates/media-isobmff/src/bytes.rs:137` | - |  |
| `parse_box_header` | 1 | `crates/media-isobmff/src/bytes.rs:157` | - | Parse a box header from `buf` (needs up to 32 bytes; fewer if available). |
| `boxes` | 1 | `crates/media-isobmff/src/bytes.rs:201` | - |  |
| `impl Iterator for Boxes<'a>::next` | 1 | `crates/media-isobmff/src/bytes.rs:207` | - |  |
| `find` | 2 | `crates/media-isobmff/src/bytes.rs:244` | - | Find the first child of the given type (malformed trailing children are ignored). |
| `impl BoxBuf::new` | 0 | `crates/media-isobmff/src/bytes.rs:255` | - |  |
| `impl BoxBuf::start` | 2 | `crates/media-isobmff/src/bytes.rs:259` | - | Begin a box; returns a marker to pass to [`end`](Self::end). |
| `impl BoxBuf::start_full` | 4 | `crates/media-isobmff/src/bytes.rs:265` | - |  |
| `impl BoxBuf::end` | 2 | `crates/media-isobmff/src/bytes.rs:270` | - |  |
| `impl BoxBuf::leaf` | 3 | `crates/media-isobmff/src/bytes.rs:275` | - | Append a complete box with the given payload. |
| `impl BoxBuf::u8` | 2 | `crates/media-isobmff/src/bytes.rs:280` | - |  |
| `impl BoxBuf::u16` | 2 | `crates/media-isobmff/src/bytes.rs:283` | - |  |
| `impl BoxBuf::i16` | 2 | `crates/media-isobmff/src/bytes.rs:286` | - |  |
| `impl BoxBuf::u32` | 2 | `crates/media-isobmff/src/bytes.rs:289` | - |  |
| `impl BoxBuf::i32` | 2 | `crates/media-isobmff/src/bytes.rs:292` | - |  |
| `impl BoxBuf::u64` | 2 | `crates/media-isobmff/src/bytes.rs:295` | - |  |
| `impl BoxBuf::i64` | 2 | `crates/media-isobmff/src/bytes.rs:298` | - |  |
| `impl BoxBuf::f64` | 2 | `crates/media-isobmff/src/bytes.rs:301` | - |  |
| `impl BoxBuf::bytes` | 2 | `crates/media-isobmff/src/bytes.rs:304` | - |  |
| `impl BoxBuf::zeros` | 2 | `crates/media-isobmff/src/bytes.rs:307` | - |  |
| `impl BoxBuf::len` | 1 | `crates/media-isobmff/src/bytes.rs:310` | - |  |
| `header_forms` | 0 | `crates/media-isobmff/src/bytes.rs:320` | - |  |
| `iterate_children` | 0 | `crates/media-isobmff/src/bytes.rs:338` | - |  |
| `cursor_bounds` | 0 | `crates/media-isobmff/src/bytes.rs:355` | - |  |
| `impl MasteringDisplay::bt2020` | 2 | `crates/media-isobmff/src/codec.rs:62` | - | BT.2020 primaries, D65, with the given peak and black in cd/m². |
| `impl MasteringDisplay::max_nits` | 1 | `crates/media-isobmff/src/codec.rs:71` | - |  |
| `impl MasteringDisplay::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:75` | - | The 24-byte payload shared by `mdcv` and the SEI message. |
| `impl MasteringDisplay::parse` | 1 | `crates/media-isobmff/src/codec.rs:87` | - |  |
| `impl CodecConfig::name` | 1 | `crates/media-isobmff/src/codec.rs:212` | - | A short human-readable codec name. |
| `impl AvcConfig::new` | 3 | `crates/media-isobmff/src/codec.rs:252` | - | Build from parameter sets (NAL units without start codes); profile/level taken from the first SPS. |
| `impl AvcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:257` | - |  |
| `impl AvcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:279` | - |  |
| `impl HevcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:333` | - |  |
| `impl HevcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:382` | - |  |
| `impl HevcConfig::nalus_of` | 2 | `crates/media-isobmff/src/codec.rs:412` | - |  |
| `impl HevcConfig::vps` | 1 | `crates/media-isobmff/src/codec.rs:415` | - |  |
| `impl HevcConfig::sps` | 1 | `crates/media-isobmff/src/codec.rs:418` | - |  |
| `impl HevcConfig::pps` | 1 | `crates/media-isobmff/src/codec.rs:421` | - |  |
| `impl Av1Config::parse` | 1 | `crates/media-isobmff/src/codec.rs:443` | - |  |
| `impl Av1Config::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:466` | - |  |
| `impl VpcConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:501` | - |  |
| `impl VpcConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:532` | - |  |
| `impl AacConfig::from_asc` | 1 | `crates/media-isobmff/src/codec.rs:568` | - | Build from an AudioSpecificConfig, decoding its header fields. |
| `impl AacConfig::channels` | 1 | `crates/media-isobmff/src/codec.rs:573` | - | Channel count implied by the channel configuration (0 if unspecified). |
| `parse_asc` | 1 | `crates/media-isobmff/src/codec.rs:583` | - | Decode (audio object type, sampling frequency, channel configuration) from an AudioSpecificConfig. |
| `impl PcmConfig::bytes_per_frame` | 1 | `crates/media-isobmff/src/codec.rs:613` | - | Bytes per interleaved audio frame (all channels). |
| `impl OpusConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:631` | - |  |
| `impl OpusConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:643` | - |  |
| `impl FlacConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:668` | - |  |
| `impl TimecodeConfig::drop_frame` | 1 | `crates/media-isobmff/src/codec.rs:705` | - |  |
| `impl TimecodeConfig::format_frame` | 2 | `crates/media-isobmff/src/codec.rs:709` | - | Format a frame number as `HH:MM:SS:FF` (`;` before the frames when drop-frame). |
| `impl ApvConfig::from_record` | 1 | `crates/media-isobmff/src/codec.rs:753` | - | Build from a raw `APVDecoderConfigurationRecord` (starting with `configurationVersion = 1`). |
| `impl ApvConfig::parse` | 1 | `crates/media-isobmff/src/codec.rs:789` | - | Parse the payload of an `apvC` box (with or without the 4-byte `FullBox` header). |
| `impl ApvConfig::to_bytes` | 1 | `crates/media-isobmff/src/codec.rs:805` | - | Serialize the full `apvC` box payload (4-byte `FullBox` header + `APVDecoderConfigurationRecord`). |
| `is_video_fourcc` | 1 | `crates/media-isobmff/src/codec.rs:838` | - |  |
| `is_audio_fourcc` | 1 | `crates/media-isobmff/src/codec.rs:868` | - |  |
| `parse_sample_entry` | 4 | `crates/media-isobmff/src/codec.rs:897` | - | Parse one `stsd` entry. `handler` is the track handler type; `is_qt` selects QuickTime |
| `qt_pcm` | 2 | `crates/media-isobmff/src/codec.rs:1068` | - |  |
| `parse_btrt` | 1 | `crates/media-isobmff/src/codec.rs:1106` | - |  |
| `parse_colr` | 1 | `crates/media-isobmff/src/codec.rs:1111` | - |  |
| `descriptor` | 1 | `crates/media-isobmff/src/codec.rs:1121` | - |  |
| `parse_esds` | 1 | `crates/media-isobmff/src/codec.rs:1134` | - |  |
| `put_descriptor` | 3 | `crates/media-isobmff/src/codec.rs:1193` | - |  |
| `esds_payload` | 5 | `crates/media-isobmff/src/codec.rs:1204` | - |  |
| `impl SampleEntry::video` | 4 | `crates/media-isobmff/src/codec.rs:1228` | - | Convenience constructor for a video entry. |
| `impl SampleEntry::audio` | 5 | `crates/media-isobmff/src/codec.rs:1261` | - | Convenience constructor for an audio entry (ISO v0 layout; the muxer upgrades PCM in MOV to v2). |
| `impl SampleEntry::avc` | 3 | `crates/media-isobmff/src/codec.rs:1273` | - | H.264 entry (`avc1`). |
| `impl SampleEntry::hevc` | 3 | `crates/media-isobmff/src/codec.rs:1277` | - | HEVC entry (`hvc1`, parameter sets in the config only). |
| `impl SampleEntry::av1` | 3 | `crates/media-isobmff/src/codec.rs:1281` | - | AV1 entry (`av01`). |
| `impl SampleEntry::prores` | 3 | `crates/media-isobmff/src/codec.rs:1285` | - | ProRes entry; `fourcc` one of `apch`/`apcn`/`apcs`/`apco`/`ap4h`/`ap4x`. |
| `impl SampleEntry::dnx` | 3 | `crates/media-isobmff/src/codec.rs:1295` | - | Avid DNxHD / DNxHR entry (`AVdn`, or `AVdh` for DNxHR). |
| `impl SampleEntry::apv` | 3 | `crates/media-isobmff/src/codec.rs:1303` | - | Advanced Professional Video entry (`apv1` + `apvC`). |
| `impl SampleEntry::jpeg` | 2 | `crates/media-isobmff/src/codec.rs:1311` | - | Motion-JPEG entry (`jpeg`). |
| `impl SampleEntry::aac` | 3 | `crates/media-isobmff/src/codec.rs:1316` | - | AAC entry (`mp4a` + `esds`) from an AudioSpecificConfig. |
| `impl SampleEntry::pcm` | 1 | `crates/media-isobmff/src/codec.rs:1320` | - | PCM entry. The muxer picks the on-disk format (`sowt`/`twos`/`lpcm` for MOV, `ipcm`/`fpcm` for MP4). |
| `impl SampleEntry::write` | 3 | `crates/media-isobmff/src/codec.rs:1325` | - | Serialize as a complete sample entry box. For PCM the format is chosen from the brand. |
| `write_btrt` | 2 | `crates/media-isobmff/src/codec.rs:1554` | - |  |
| `roundtrip` | 3 | `crates/media-isobmff/src/codec.rs:1566` | - |  |
| `avcc_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1574` | - |  |
| `hvcc_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1591` | - |  |
| `av1_vp9_opus_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1616` | - |  |
| `aac_esds_roundtrip` | 0 | `crates/media-isobmff/src/codec.rs:1636` | - |  |
| `quicktime_mpeg_audio_entries` | 0 | `crates/media-isobmff/src/codec.rs:1651` | - | QuickTime's `.mp2` entry (what ffmpeg writes for MP2 in MOV) is MPEG audio, as `.mp3` is, |
| `pcm_entries` | 0 | `crates/media-isobmff/src/codec.rs:1663` | - |  |
| `timecode_and_prores` | 0 | `crates/media-isobmff/src/codec.rs:1680` | - |  |
| `asc_parse` | 0 | `crates/media-isobmff/src/codec.rs:1699` | - |  |
| `impl Default for DemuxLimits::default` | 0 | `crates/media-isobmff/src/demux.rs:31` | - |  |
| `bounded_count` | 2 | `crates/media-isobmff/src/demux.rs:34` | - |  |
| `impl Track::codec` | 1 | `crates/media-isobmff/src/demux.rs:126` | - | The first sample entry's codec configuration. |
| `impl Track::video` | 1 | `crates/media-isobmff/src/demux.rs:129` | - |  |
| `impl Track::display_rotation` | 1 | `crates/media-isobmff/src/demux.rs:140` | numerics | Clockwise quarter turns (0–3) the `tkhd` matrix applies for display, or None when it is not |
| `impl Track::audio` | 1 | `crates/media-isobmff/src/demux.rs:150` | - |  |
| `impl Track::presentation_pts` | 2 | `crates/media-isobmff/src/demux.rs:154` | - | Presentation (edit-adjusted) pts of sample `i`. |
| `impl Track::sample_at_pts` | 2 | `crates/media-isobmff/src/demux.rs:159` | - | Index of the sample being presented at raw media time `pts` (the last sample, in |
| `impl Track::sample_at_presentation_time` | 2 | `crates/media-isobmff/src/demux.rs:167` | - | Like [`sample_at_pts`](Self::sample_at_pts) but for edit-adjusted presentation time. |
| `impl Track::sync_sample_before` | 2 | `crates/media-isobmff/src/demux.rs:171` | - | The closest sync sample at or before `index` (decode order); 0 if there is none. |
| `impl Track::sync_samples` | 1 | `crates/media-isobmff/src/demux.rs:176` | - | Indices of all sync samples. |
| `impl Track::finish_indexes` | 1 | `crates/media-isobmff/src/demux.rs:180` | - |  |
| `impl Metadata::get` | 2 | `crates/media-isobmff/src/demux.rs:195` | - |  |
| `impl Metadata::title` | 1 | `crates/media-isobmff/src/demux.rs:199` | - | Title from `©nam` or the `com.apple.quicktime.title` key. |
| `impl Mp4File::read_sample` | 4 | `crates/media-isobmff/src/demux.rs:228` | - | Read the bytes of sample `index` of track `track` (index into [`tracks`](Self::tracks)). |
| `impl Mp4File::sample_at_pts` | 3 | `crates/media-isobmff/src/demux.rs:233` | - |  |
| `impl Mp4File::sync_sample_before` | 3 | `crates/media-isobmff/src/demux.rs:236` | - |  |
| `impl Mp4File::track_of_kind` | 2 | `crates/media-isobmff/src/demux.rs:240` | - | First track of the given kind. |
| `read_range` | 3 | `crates/media-isobmff/src/demux.rs:245` | - |  |
| `open` | 1 | `crates/media-isobmff/src/demux.rs:256` | - | Parse an MP4/MOV file. Only box headers, `ftyp`, `moov` and `moof` boxes are read. |
| `open_with_limits` | 2 | `crates/media-isobmff/src/demux.rs:260` | - |  |
| `parse_language` | 1 | `crates/media-isobmff/src/demux.rs:394` | - |  |
| `rescale` | 3 | `crates/media-isobmff/src/demux.rs:404` | - |  |
| `parse_trak` | 4 | `crates/media-isobmff/src/demux.rs:411` | - |  |
| `validate_self_contained_data` | 1 | `crates/media-isobmff/src/demux.rs:558` | - | Narrow local-file profile; no external URLs, aliases, extra entries or duplicate tables. |
| `impl SttsCursor<'_>::take` | 2 | `crates/media-isobmff/src/demux.rs:587` | - | Advance by `n` samples, returning the summed duration. |
| `impl SttsCursor<'_>::next_value` | 1 | `crates/media-isobmff/src/demux.rs:603` | - |  |
| `parse_stbl` | 5 | `crates/media-isobmff/src/demux.rs:613` | - |  |
| `parse_moof` | 7 | `crates/media-isobmff/src/demux.rs:847` | - |  |
| `parse_udta` | 2 | `crates/media-isobmff/src/demux.rs:941` | - |  |
| `parse_meta` | 2 | `crates/media-isobmff/src/demux.rs:954` | - |  |
| `language_codes` | 0 | `crates/media-isobmff/src/demux.rs:991` | - |  |
| `stts_cursor` | 0 | `crates/media-isobmff/src/demux.rs:999` | - |  |
| `rejects_no_moov_and_cmov` | 0 | `crates/media-isobmff/src/demux.rs:1009` | - |  |
| `metadata_qt_and_ilst` | 0 | `crates/media-isobmff/src/demux.rs:1020` | - |  |
| `full` | 4 | `crates/media-isobmff/src/demux.rs:1047` | - |  |
| `handmade` | 1 | `crates/media-isobmff/src/demux.rs:1055` | - | Hand-built file: uuid/wide/skip boxes, stz2 sizes, co64, ctts v1 with negative offsets, |
| `handmade_sample_table` | 0 | `crates/media-isobmff/src/demux.rs:1194` | - |  |
| `absurd_counts_are_bounded` | 0 | `crates/media-isobmff/src/demux.rs:1225` | - |  |
| `caller_limits_refuse_input_track_and_table_counts_before_expansion` | 0 | `crates/media-isobmff/src/demux.rs:1243` | - |  |
| `invalid` | 1 | `crates/media-isobmff/src/error.rs:36` | - |  |
| `impl WriterOptions::new` | 1 | `crates/media-isobmff/src/mux.rs:29` | - |  |
| `impl TrackConfig::new` | 2 | `crates/media-isobmff/src/mux.rs:52` | - |  |
| `handler_of` | 1 | `crates/media-isobmff/src/mux.rs:76` | - |  |
| `impl Tables::new` | 0 | `crates/media-isobmff/src/mux.rs:109` | - |  |
| `impl Tables::push_run` | 3 | `crates/media-isobmff/src/mux.rs:112` | - |  |
| `pack_language` | 1 | `crates/media-isobmff/src/mux.rs:136` | - |  |
| `rescale` | 3 | `crates/media-isobmff/src/mux.rs:145` | - |  |
| `write_ftyp` | 2 | `crates/media-isobmff/src/mux.rs:154` | - |  |
| `movie_duration` | 2 | `crates/media-isobmff/src/mux.rs:173` | - |  |
| `effective_edits` | 2 | `crates/media-isobmff/src/mux.rs:177` | - |  |
| `presentation_duration` | 2 | `crates/media-isobmff/src/mux.rs:190` | - |  |
| `write_mvhd` | 4 | `crates/media-isobmff/src/mux.rs:195` | - |  |
| `write_trak` | 6 | `crates/media-isobmff/src/mux.rs:220` | physics |  |
| `write_udta` | 2 | `crates/media-isobmff/src/mux.rs:464` | - |  |
| `validate_track` | 1 | `crates/media-isobmff/src/mux.rs:490` | - |  |
| `impl Mp4Writer<W>::new` | 2 | `crates/media-isobmff/src/mux.rs:521` | - | Start a file: writes `ftyp`, a placeholder box and the `mdat` header. |
| `impl Mp4Writer<W>::add_track` | 2 | `crates/media-isobmff/src/mux.rs:535` | - | Add a track; returns its index (track ids are index + 1). |
| `impl Mp4Writer<W>::add_timecode_track` | 4 | `crates/media-isobmff/src/mux.rs:546` | - | Add a QuickTime timecode track referenced by `video_track`. Its single sample (the start |
| `impl Mp4Writer<W>::write_sample` | 3 | `crates/media-isobmff/src/mux.rs:570` | - | Append one sample to track `track` (decode order). For PCM tracks `data` may hold any whole |
| `impl Mp4Writer<W>::finish_mdat` | 1 | `crates/media-isobmff/src/mux.rs:620` | - |  |
| `impl Mp4Writer<W>::build_moov` | 3 | `crates/media-isobmff/src/mux.rs:662` | physics |  |
| `impl Mp4Writer<W>::finish` | 1 | `crates/media-isobmff/src/mux.rs:677` | - | Finish the file with `moov` after `mdat`. Returns the underlying writer. |
| `impl Mp4Writer<W>::finish_faststart` | 1 | `crates/media-isobmff/src/mux.rs:688` | - | Finish with `moov` in front of `mdat` ("fast start"): media data is shifted forward in place. |
| `impl FragmentedWriter<W>::new` | 2 | `crates/media-isobmff/src/mux.rs:740` | - |  |
| `impl FragmentedWriter<W>::add_track` | 2 | `crates/media-isobmff/src/mux.rs:745` | - | Add a track (before the first fragment). PCM is not supported in fragments. |
| `impl FragmentedWriter<W>::write_init` | 1 | `crates/media-isobmff/src/mux.rs:759` | - |  |
| `impl FragmentedWriter<W>::write_sample` | 3 | `crates/media-isobmff/src/mux.rs:788` | - | Buffer a sample for the next fragment. |
| `impl FragmentedWriter<W>::build_moof` | 2 | `crates/media-isobmff/src/mux.rs:797` | - |  |
| `impl FragmentedWriter<W>::flush_fragment` | 1 | `crates/media-isobmff/src/mux.rs:833` | - | Write all buffered samples as one `moof` + `mdat` fragment (no-op if nothing is buffered). |
| `impl FragmentedWriter<W>::finish` | 1 | `crates/media-isobmff/src/mux.rs:871` | - | Flush remaining samples and return the writer. |
| `avc_entry` | 0 | `crates/media-isobmff/src/mux.rs:884` | - |  |
| `language_pack` | 0 | `crates/media-isobmff/src/mux.rs:889` | - |  |
| `simple_roundtrip_and_faststart` | 0 | `crates/media-isobmff/src/mux.rs:896` | - |  |
| `fragmented_roundtrip` | 0 | `crates/media-isobmff/src/mux.rs:932` | - |  |
| `trait ByteSource::len` | 1 | `crates/media-isobmff/src/source.rs:11` | - | Total length in bytes. |
| `trait ByteSource::read_at` | 3 | `crates/media-isobmff/src/source.rs:13` | - | Fill `buf` entirely with bytes starting at `offset`; fail with `UnexpectedEof` if past the end. |
| `trait ByteSource::is_empty` | 1 | `crates/media-isobmff/src/source.rs:15` | - | True if the source holds no bytes. |
| `eof` | 0 | `crates/media-isobmff/src/source.rs:20` | - |  |
| `impl ByteSource for [u8]::len` | 1 | `crates/media-isobmff/src/source.rs:25` | - |  |
| `impl ByteSource for [u8]::read_at` | 3 | `crates/media-isobmff/src/source.rs:28` | - |  |
| `impl ByteSource for Vec<u8>::len` | 1 | `crates/media-isobmff/src/source.rs:38` | - |  |
| `impl ByteSource for Vec<u8>::read_at` | 3 | `crates/media-isobmff/src/source.rs:41` | - |  |
| `impl ByteSource for &T::len` | 1 | `crates/media-isobmff/src/source.rs:47` | - |  |
| `impl ByteSource for &T::read_at` | 3 | `crates/media-isobmff/src/source.rs:50` | - |  |
| `impl ByteSource for Arc<T>::len` | 1 | `crates/media-isobmff/src/source.rs:56` | - |  |
| `impl ByteSource for Arc<T>::read_at` | 3 | `crates/media-isobmff/src/source.rs:59` | - |  |
| `impl ByteSource for Box<T>::len` | 1 | `crates/media-isobmff/src/source.rs:65` | - |  |
| `impl ByteSource for Box<T>::read_at` | 3 | `crates/media-isobmff/src/source.rs:68` | - |  |
| `impl ByteSource for std::fs::File::len` | 1 | `crates/media-isobmff/src/source.rs:75` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:79` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:83` | - |  |
| `impl ByteSource for std::fs::File::read_at` | 3 | `crates/media-isobmff/src/source.rs:96` | - |  |
| `slice_reads` | 0 | `crates/media-isobmff/src/source.rs:109` | - |  |
| `tool` | 1 | `crates/media-isobmff/tests/common/mod.rs:10` | - | Find a tool silently (see `media_test_support::oracle`). |
| `ffmpeg` | 0 | `crates/media-isobmff/tests/common/mod.rs:19` | - | ffmpeg, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`). |
| `ffprobe` | 0 | `crates/media-isobmff/tests/common/mod.rs:24` | - | ffprobe, or `None` after printing `SKIPPED` (a failure with `FILMCRAFT_REQUIRE_ORACLES=1`). |
| `fixture_dir` | 0 | `crates/media-isobmff/tests/common/mod.rs:28` | - |  |
| `out_dir` | 0 | `crates/media-isobmff/tests/common/mod.rs:32` | - |  |
| `fixture_specs` | 0 | `crates/media-isobmff/tests/common/mod.rs:42` | - | (fixture name, ffmpeg arguments after the global options, output file name derived from the name). |
| `run_ffmpeg` | 3 | `crates/media-isobmff/tests/common/mod.rs:112` | - |  |
| `fixture` | 1 | `crates/media-isobmff/tests/common/mod.rs:128` | - | Path to a fixture, generating it with ffmpeg if needed. `None` if ffmpeg is missing or the |
| `all_fixture_names` | 0 | `crates/media-isobmff/tests/common/mod.rs:143` | - |  |
| `probe_json` | 2 | `crates/media-isobmff/tests/common/mod.rs:149` | - |  |
| `packets` | 1 | `crates/media-isobmff/tests/common/mod.rs:158` | - |  |
| `streams` | 1 | `crates/media-isobmff/tests/common/mod.rs:162` | - |  |
| `int` | 1 | `crates/media-isobmff/tests/common/mod.rs:166` | - |  |
| `ffmpeg_decode_errors` | 1 | `crates/media-isobmff/tests/common/mod.rs:175` | - | Decode a file with ffmpeg to the null muxer; returns stderr (empty on success). |
| `impl Rng::next` | 1 | `crates/media-isobmff/tests/fuzz.rs:10` | - |  |
| `impl Rng::below` | 2 | `crates/media-isobmff/tests/fuzz.rs:17` | - |  |
| `synthetic_files` | 0 | `crates/media-isobmff/tests/fuzz.rs:22` | - |  |
| `exercise` | 1 | `crates/media-isobmff/tests/fuzz.rs:53` | - |  |
| `mutate_all` | 3 | `crates/media-isobmff/tests/fuzz.rs:75` | - |  |
| `mutated_synthetic_files_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:134` | - |  |
| `mutated_ffmpeg_fixtures_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:142` | - |  |
| `truncations_never_panic` | 0 | `crates/media-isobmff/tests/fuzz.rs:154` | - |  |
| `pcm_name_matches` | 2 | `crates/media-isobmff/tests/oracle_demux.rs:9` | - |  |
| `check_stream` | 3 | `crates/media-isobmff/tests/oracle_demux.rs:30` | - |  |
| `gcd` | 2 | `crates/media-isobmff/tests/oracle_demux.rs:121` | - |  |
| `check_packets` | 4 | `crates/media-isobmff/tests/oracle_demux.rs:127` | - | `strict`: ffprobe timestamps must equal ours + edit offset. Otherwise (edit starting mid-frame, |
| `check_fixture` | 1 | `crates/media-isobmff/tests/oracle_demux.rs:174` | - |  |
| `specific_properties` | 0 | `crates/media-isobmff/tests/oracle_demux.rs:247` | - |  |
| `generate_fixtures` | 0 | `crates/media-isobmff/tests/oracle_demux.rs:278` | - | Pre-generate every fixture (`cargo xtask fixtures`). |
| `interleave` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:12` | - | All samples of all tracks in decode order interleaved by decode time: (track, sample index). |
| `track_config` | 3 | `crates/media-isobmff/tests/oracle_mux.rs:24` | - |  |
| `remux` | 5 | `crates/media-isobmff/tests/oracle_mux.rs:39` | - |  |
| `stream_packets` | 1 | `crates/media-isobmff/tests/oracle_mux.rs:86` | - |  |
| `assert_same_packets` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:93` | - | Compare ffprobe's view of two files (per-stream packets: pts, dts, size, key flag). |
| `assert_same_streams` | 2 | `crates/media-isobmff/tests/oracle_mux.rs:107` | - |  |
| `decode_clean` | 1 | `crates/media-isobmff/tests/oracle_mux.rs:120` | - |  |
| `remux_h264_aac_mp4_variants` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:126` | - |  |
| `remux_other_codecs` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:147` | - |  |
| `remux_prores_pcm24_with_timecode` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:181` | - |  |
| `pcm_in_mp4_ipcm` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:200` | - |  |
| `aac_priming_edit` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:210` | - |  |
| `fragmented_writer_ffmpeg` | 0 | `crates/media-isobmff/tests/oracle_mux.rs:232` | - |  |
| `sample_strategy` | 0 | `crates/media-isobmff/tests/roundtrip.rs:15` | - |  |
| `payload` | 3 | `crates/media-isobmff/tests/roundtrip.rs:19` | - |  |
| `entry` | 1 | `crates/media-isobmff/tests/roundtrip.rs:23` | - |  |
| `sample_lookup` | 0 | `crates/media-isobmff/tests/roundtrip.rs:143` | - |  |
| `large_offsets_use_co64_and_largesize_mdat` | 0 | `crates/media-isobmff/tests/roundtrip.rs:167` | - |  |
| `all_empty_samples_roundtrip` | 0 | `crates/media-isobmff/tests/roundtrip.rs:211` | - | Found by `progressive_roundtrip`: only zero-byte samples must not be written as uniform |

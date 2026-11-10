# `crates/chronicle-broadcast-containers`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_SMPTE VC-3 Avid DNxHD / DNxHR broadcast intermediate frame decoder._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Reader<'a>::new` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:15` | - |  |
| `impl Reader<'a>::refill` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:27` | - |  |
| `impl Reader<'a>::consume` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:47` | - |  |
| `impl Reader<'a>::consumed` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:53` | - |  |
| `impl Reader<'a>::overrun` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:58` | - |  |
| `impl Reader<'a>::read` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:63` | - |  |
| `impl Reader<'a>::vlc` | 2 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:74` | - |  |
| `impl Writer::new` | 0 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:97` | - |  |
| `impl Writer::write` | 3 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:102` | - |  |
| `impl Writer::flush` | 1 | `crates/chronicle-broadcast-containers/src/dnx/bits.rs:116` | - |  |
| `idct8x8` | 1 | `crates/chronicle-broadcast-containers/src/dnx/dct.rs:7` | - | In-place inverse DCT. `block` holds dequantised coefficients in raster order |
| `fdct8x8` | 1 | `crates/chronicle-broadcast-containers/src/dnx/dct.rs:71` | - | In-place forward DCT (samples in, coefficients out). |
| `impl Frame::new` | 5 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:29` | - |  |
| `impl Frame::chroma_width` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:63` | - |  |
| `impl Frame::chroma_height` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:70` | - |  |
| `probe` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:83` | - |  |
| `decode_frame` | 1 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:87` | - |  |
| `decode_frame_with` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:91` | - |  |
| `second_unit_offset` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:134` | - |  |
| `impl CodedPlanes::copy_into` | 4 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:158` | - |  |
| `decode_unit` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:243` | - |  |
| `decode_row` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:368` | - |  |
| `scaled_weights` | 2 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:452` | - |  |
| `decode_block` | 5 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:464` | - |  |
| `store` | 8 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:520` | - |  |
| `convert_mb_to_rgb` | 3 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:544` | - |  |
| `decode_rle_alpha` | 4 | `crates/chronicle-broadcast-containers/src/dnx/decode.rs:569` | - |  |
| `impl FrameHeader::rgb_planes` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:56` | - |  |
| `impl FrameHeader::mb_width` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:60` | - |  |
| `impl FrameHeader::coding_unit_size` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:64` | - |  |
| `be16` | 2 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:91` | - |  |
| `be32` | 2 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:95` | - |  |
| `parse_frame_header` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:99` | - |  |
| `header_size_for` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:206` | - |  |
| `write_frame_header` | 1 | `crates/chronicle-broadcast-containers/src/dnx/header.rs:214` | - |  |
| `hd` | 8 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:57` | - |  |
| `ri` | 4 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:87` | - |  |
| `cid_info` | 1 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:120` | - |  |
| `ri_frame_size` | 4 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:124` | - |  |
| `impl Lut::build` | 2 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:149` | - |  |
| `build_set` | 3 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:211` | - |  |
| `vlc` | 1 | `crates/chronicle-broadcast-containers/src/dnx/tables.rs:260` | - |  |
| `test_ebml_vint_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:62` | - |  |
| `test_matroska_mux_demux_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:73` | - |  |
| `test_mxf_klv_and_writer_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:113` | - |  |
| `test_mpegts_packet_and_psi` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:137` | - |  |
| `test_dnx_idct_fdct_roundtrip` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:172` | - |  |
| `test_dnx_header_write_and_parse` | 0 | `crates/chronicle-broadcast-containers/src/lib.rs:194` | - |  |
| `demux_matroska` | 1 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:35` | - |  |
| `parse_ebml_header` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:68` | - |  |
| `parse_segment` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:88` | - |  |
| `parse_info` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:111` | - |  |
| `parse_tracks` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:133` | - |  |
| `parse_track_entry` | 1 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:155` | - |  |
| `parse_cluster` | 2 | `crates/chronicle-broadcast-containers/src/matroska/demux.rs:260` | - |  |
| `vint_len` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:4` | - |  |
| `impl Size::known` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:20` | - |  |
| `read_vint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:29` | - | Decode an unsigned VINT. Returns `(value, length, all_ones)`. |
| `read_size` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:49` | - | Decode an element size. |
| `read_id` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:55` | - | Decode an element ID (marker bits retained). |
| `read_svint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:69` | - | Decode a signed VINT as used by EBML lacing (value − (2^(7·len−1) − 1)). |
| `write_id` | 2 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:76` | - | Encode an element ID. |
| `write_vint` | 3 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:90` | - | Encode an unsigned integer as a VINT with a specific width (1..=8 bytes). |
| `write_size` | 3 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:102` | - | Encode a data size as a VINT using the shortest length (or `min_len` if larger). |
| `write_unknown_size` | 2 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:113` | - | Encode the "unknown size" marker of the given length. |
| `uint` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:121` | - | Big-endian unsigned integer of 0..=8 bytes. |
| `int` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:126` | - | Big-endian two's-complement signed integer of 0..=8 bytes. |
| `float` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:137` | - | IEEE float of 0, 4 or 8 bytes. |
| `string` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ebml.rs:146` | - | String / UTF-8 value: bytes up to the first NUL, lossily decoded. |
| `is_top_level` | 1 | `crates/chronicle-broadcast-containers/src/matroska/ids.rs:28` | - |  |
| `impl TrackSpec::new` | 2 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:47` | - |  |
| `impl Default for MuxOptions::default` | 0 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:76` | - |  |
| `el` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:89` | - |  |
| `el_uint` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:95` | - |  |
| `el_float` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:100` | - |  |
| `el_str` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:104` | - |  |
| `void` | 2 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:108` | - |  |
| `impl MkvWriter<W>::new` | 3 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:147` | - |  |
| `impl MkvWriter<W>::write_frame` | 6 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:268` | - | Add one frame. |
| `impl MkvWriter<W>::flush_pending` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:358` | - |  |
| `impl MkvWriter<W>::flush_cluster` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:405` | - |  |
| `impl MkvWriter<W>::finish` | 1 | `crates/chronicle-broadcast-containers/src/matroska/mux.rs:419` | - | Write Cues, back-patch SeekHead / Segment size / Duration, and return writer. |
| `plain_stream` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:12` | - |  |
| `parse_timestamp` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:16` | - |  |
| `encode_timestamp` | 2 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:24` | - |  |
| `parse_pes_header` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:34` | - |  |
| `build_pes_packet` | 4 | `crates/chronicle-broadcast-containers/src/mpegts/pes.rs:113` | - | Create a PES packet containing `data` with optional PTS/DTS. |
| `crc32_mpeg` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:5` | - | CRC-32 MPEG-2 (polynomial 0x04C11DB7, initial 0xFFFFFFFF). |
| `parse_pat` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:33` | - |  |
| `parse_pmt` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:83` | - |  |
| `build_pat` | 3 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:131` | - | Build a PAT section including CRC-32. |
| `build_pmt` | 4 | `crates/chronicle-broadcast-containers/src/mpegts/psi.rs:152` | - | Build a PMT section including CRC-32. |
| `parse_ts_packet` | 1 | `crates/chronicle-broadcast-containers/src/mpegts/ts.rs:23` | - | Parse a single 188-byte TS packet. |
| `write_ts_packet` | 6 | `crates/chronicle-broadcast-containers/src/mpegts/ts.rs:85` | - | Write a 188-byte TS packet into `out`. |
| `demux_mxf` | 1 | `crates/chronicle-broadcast-containers/src/mxf/demux.rs:22` | - |  |
| `impl Ul::is_smpte` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:12` | - | Every SMPTE label starts with the object identifier 06 0E 2B 34. |
| `impl Ul::matches` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:16` | - | Compare ignoring byte 7 (the registry version, which varies between writers). |
| `impl Ul::item_starts_with` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:20` | - | Bytes 8.. start with `prefix` (registry-independent comparison of the item designator). |
| `impl fmt::Debug for Ul::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:26` | - |  |
| `impl fmt::Display for Ul::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:38` | - |  |
| `impl Rational::new` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:51` | - |  |
| `impl Rational::is_valid` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:54` | - |  |
| `impl Rational::as_f64` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:57` | - |  |
| `classify` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:85` | - |  |
| `ber_length` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:118` | - | Decode a BER length at `b[0..]`: (length, bytes used). `None` if truncated or longer than 8 bytes. |
| `write_ber4` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:134` | - | Write a 4-byte BER length. |
| `impl Cur<'a>::new` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:148` | - |  |
| `impl Cur<'a>::remaining` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:151` | - |  |
| `impl Cur<'a>::bytes` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:154` | - |  |
| `impl Cur<'a>::u16` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:159` | - |  |
| `impl Cur<'a>::u32` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:162` | - |  |
| `impl Cur<'a>::u64` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:165` | - |  |
| `impl Cur<'a>::ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:168` | - |  |
| `int_of` | 2 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:173` | - |  |
| `rational_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:185` | - |  |
| `ul_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:195` | - |  |
| `batch_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:199` | - |  |
| `utf16_of` | 1 | `crates/chronicle-broadcast-containers/src/mxf/klv.rs:218` | - |  |
| `set_key` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:3` | - |  |
| `partition_key` | 2 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:10` | - |  |
| `op1a` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:30` | - |  |
| `element_key` | 4 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:61` | - |  |
| `ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/labels.rs:68` | - |  |
| `impl Primer::parse` | 1 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:11` | - |  |
| `impl Set::parse` | 3 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:52` | - |  |
| `impl Set::kind` | 1 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:75` | - |  |
| `impl Set::get` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:81` | - |  |
| `impl Set::u` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:85` | - |  |
| `impl Set::i` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:89` | - |  |
| `impl Set::rational` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:93` | - |  |
| `impl Set::ul` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:97` | - |  |
| `impl Set::string` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:101` | - |  |
| `impl Metadata::push` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:114` | - |  |
| `impl Metadata::get` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:121` | - |  |
| `impl Metadata::of_kind` | 2 | `crates/chronicle-broadcast-containers/src/mxf/meta.rs:125` | - |  |
| `impl Partition::parse` | 4 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:33` | - |  |
| `impl Partition::closed` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:62` | - |  |
| `impl Partition::complete` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:66` | - |  |
| `impl OperationalPattern::from_ul` | 1 | `crates/chronicle-broadcast-containers/src/mxf/partition.rs:80` | - |  |
| `impl std::fmt::Debug for Umid::fmt` | 2 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:12` | - |  |
| `impl Umid::from_seed` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:21` | - |  |
| `hash128` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:32` | - |  |
| `impl MxfWriter<W>::new` | 3 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:83` | - |  |
| `impl MxfWriter<W>::add_picture_track` | 4 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:102` | - |  |
| `impl MxfWriter<W>::add_sound_track` | 4 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:118` | - |  |
| `impl MxfWriter<W>::write_header` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:134` | - |  |
| `impl MxfWriter<W>::write_essence_packet` | 3 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:206` | - |  |
| `impl MxfWriter<W>::finish` | 1 | `crates/chronicle-broadcast-containers/src/mxf/writer.rs:219` | - |  |

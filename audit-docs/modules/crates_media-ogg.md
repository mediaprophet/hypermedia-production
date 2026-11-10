# `crates/media-ogg`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Clean-room Ogg demuxer, written from RFC 3533 (the Ogg encapsulation format), RFC 7845 (Ogg
Opus) and the Vorbis I specification's Ogg mapping (section A).

[`open`] reads every page (capture pattern, header, lacing values, CRC-32 checked; damaged
regions are skipped by searching for the next `OggS`), groups them by logical bitstream and
assembles packets — including packets that continue across pages — as lists of byte ranges,
with the granule position of the page each packet completes on. Codec headers (Opus:
`OpusHead` + `OpusTags`; Vorbis: identification, comment, setup) are collected separately from
the audio packets.

[`OpusTiming`] implements RFC 7845 §4: packet start positions from granule positions and
TOC-derived durations, the pre-skip, and end trimming of the final page, so a player can seek
to any 48 kHz sample.

Layer L0: no dependencies beyond `std`, no `unsafe`, builds for `wasm32-unknown-unknown`._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `trait ByteSource::len` | 1 | `crates/media-ogg/src/lib.rs:25` | - |  |
| `trait ByteSource::read_at` | 3 | `crates/media-ogg/src/lib.rs:26` | - |  |
| `trait ByteSource::is_empty` | 1 | `crates/media-ogg/src/lib.rs:27` | - |  |
| `impl ByteSource for [u8]::len` | 1 | `crates/media-ogg/src/lib.rs:33` | - |  |
| `impl ByteSource for [u8]::read_at` | 3 | `crates/media-ogg/src/lib.rs:36` | - |  |
| `impl ByteSource for Vec<u8>::len` | 1 | `crates/media-ogg/src/lib.rs:45` | - |  |
| `impl ByteSource for Vec<u8>::read_at` | 3 | `crates/media-ogg/src/lib.rs:48` | - |  |
| `impl ByteSource for &T::len` | 1 | `crates/media-ogg/src/lib.rs:54` | - |  |
| `impl ByteSource for &T::read_at` | 3 | `crates/media-ogg/src/lib.rs:57` | - |  |
| `impl fmt::Display for Error::fmt` | 2 | `crates/media-ogg/src/lib.rs:71` | - |  |
| `impl From<io::Error> for Error::from` | 1 | `crates/media-ogg/src/lib.rs:83` | - |  |
| `sniff` | 1 | `crates/media-ogg/src/lib.rs:91` | - | The capture pattern at the start of a file. |
| `crc32` | 1 | `crates/media-ogg/src/lib.rs:96` | - | CRC-32 of RFC 3533 §6: polynomial 0x04C11DB7, initial value 0, no reflection, no final XOR. |
| `impl Codec::of` | 1 | `crates/media-ogg/src/lib.rs:124` | - |  |
| `impl Codec::header_count` | 1 | `crates/media-ogg/src/lib.rs:140` | - | Header packets before the audio packets. |
| `impl Codec::name` | 1 | `crates/media-ogg/src/lib.rs:148` | - |  |
| `open` | 1 | `crates/media-ogg/src/lib.rs:206` | - | Read every page and assemble the packets of every logical bitstream. |
| `find_capture` | 3 | `crates/media-ogg/src/lib.rs:331` | - |  |
| `read_parts` | 2 | `crates/media-ogg/src/lib.rs:348` | - |  |
| `impl OggFile::stream_of` | 2 | `crates/media-ogg/src/lib.rs:360` | - | The first stream of `codec`. |
| `impl OggFile::read_packet` | 4 | `crates/media-ogg/src/lib.rs:364` | - | The bytes of packet `i` of stream `s`. |
| `opus_packet_samples` | 2 | `crates/media-ogg/src/lib.rs:371` | - | Duration of an Opus packet in 48 kHz samples from its TOC (RFC 6716 §3.1); `None` if invalid. |
| `impl OpusTiming::of` | 2 | `crates/media-ogg/src/lib.rs:411` | - | Timing of the Opus stream `s` (`pre_skip` from its `OpusHead`). |
| `impl VorbisInfo::parse` | 1 | `crates/media-ogg/src/lib.rs:473` | - |  |
| `validate_tags` | 1 | `crates/media-ogg/src/strict_opus.rs:17` | - |  |
| `number` | 2 | `crates/media-ogg/src/strict_opus.rs:18` | - |  |
| `open_bounded_opus` | 2 | `crates/media-ogg/src/strict_opus.rs:45` | - | Validate framing, checksums, sequence and timing before invoking the |
| `page` | 5 | `crates/media-ogg/src/strict_opus.rs:124` | - |  |
| `sample` | 0 | `crates/media-ogg/src/strict_opus.rs:133` | - |  |
| `limits` | 0 | `crates/media-ogg/src/strict_opus.rs:139` | - |  |
| `admits_complete_single_stream_with_final_trim` | 0 | `crates/media-ogg/src/strict_opus.rs:140` | - |  |
| `tags_require_bounded_complete_lengths` | 0 | `crates/media-ogg/src/strict_opus.rs:144` | - |  |
| `refuses_corruption_sequence_limits_and_trailing_bytes` | 0 | `crates/media-ogg/src/strict_opus.rs:152` | - |  |
| `page` | 7 | `crates/media-ogg/src/tests.rs:6` | - | Write one page holding `segments` (lacing already computed by `lace`). |
| `lace` | 2 | `crates/media-ogg/src/tests.rs:22` | - |  |
| `opus_head` | 1 | `crates/media-ogg/src/tests.rs:39` | - |  |
| `pkt` | 2 | `crates/media-ogg/src/tests.rs:51` | - | A CELT 20 ms packet (960 samples): TOC config 31, code 0. |
| `opus_file` | 3 | `crates/media-ogg/src/tests.rs:58` | - | Header pages, then pages of 3 packets each; the last page's granule trims `trim` samples. |
| `crc_matches_known_value` | 0 | `crates/media-ogg/src/tests.rs:83` | - |  |
| `opus_pages_packets_and_timing` | 0 | `crates/media-ogg/src/tests.rs:91` | - |  |
| `packets_spanning_pages` | 0 | `crates/media-ogg/src/tests.rs:111` | - |  |
| `corrupt_pages_are_skipped_and_truncation_is_safe` | 0 | `crates/media-ogg/src/tests.rs:138` | - |  |
| `opus_toc_durations` | 0 | `crates/media-ogg/src/tests.rs:174` | - |  |

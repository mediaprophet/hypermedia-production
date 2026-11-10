# `crates/video-prores-codec`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Clean-room Apple ProRes codec (SMPTE RDD 36) and QuickTime MOV container support.

Technical implementation grounded in open standards and Informatics Fiduciary duties
(supporting open cultural participation and creative expression under UDHR Articles 19 and 27;
ICESCR Article 15).

- **Decoder** ([`decode_frame`], [`decode_frame_with`], [`decode_frame_into`]):
ProRes 422 (Proxy, LT, Standard, HQ) and 4444 / 4444 XQ including 16-bit alpha channel.
- **Encoder** ([`Encoder`]):
ProRes 422 and 4444/XQ progressive encoding with per-slice rate control.
- **QuickTime Container** ([`encode_prores_mov`], [`decode_prores_mov`], [`probe_prores_mov`]):
Full `.mov` muxing, demuxing, and metadata extraction._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Reader<'a>::new` | 1 | `crates/video-prores-codec/src/bits.rs:18` | - |  |
| `impl Reader<'a>::refill` | 1 | `crates/video-prores-codec/src/bits.rs:26` | - | Ensure at least 56 valid bits are cached. |
| `impl Reader<'a>::consume` | 2 | `crates/video-prores-codec/src/bits.rs:46` | - |  |
| `impl Reader<'a>::consumed` | 1 | `crates/video-prores-codec/src/bits.rs:52` | - |  |
| `impl Reader<'a>::bits_left` | 1 | `crates/video-prores-codec/src/bits.rs:57` | - |  |
| `impl Reader<'a>::overrun` | 1 | `crates/video-prores-codec/src/bits.rs:62` | - |  |
| `impl Reader<'a>::only_padding_left` | 1 | `crates/video-prores-codec/src/bits.rs:68` | - | True when every remaining bit is zero (the component's padding), or nothing is left. |
| `impl Reader<'a>::read` | 2 | `crates/video-prores-codec/src/bits.rs:81` | - |  |
| `impl Reader<'a>::read_bit` | 1 | `crates/video-prores-codec/src/bits.rs:93` | - |  |
| `impl Reader<'a>::read_cw` | 2 | `crates/video-prores-codec/src/bits.rs:104` | - | Decode one codeword; `None` for codewords longer than 56 bits (never produced by valid streams). |
| `cw_len` | 2 | `crates/video-prores-codec/src/bits.rs:131` | - | Number of bits of the codeword for `val` in codebook `cb`. |
| `trait Sink::put` | 3 | `crates/video-prores-codec/src/bits.rs:148` | - |  |
| `trait Sink::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:149` | - |  |
| `impl Sink for Counter::put` | 3 | `crates/video-prores-codec/src/bits.rs:158` | - |  |
| `impl Sink for Counter::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:162` | - |  |
| `impl Writer::new` | 0 | `crates/video-prores-codec/src/bits.rs:176` | - |  |
| `impl Writer::finish` | 1 | `crates/video-prores-codec/src/bits.rs:180` | - | Pad with zero bits to a byte boundary and return the bytes. |
| `impl Sink for Writer::put` | 3 | `crates/video-prores-codec/src/bits.rs:191` | - |  |
| `impl Sink for Writer::put_cw` | 3 | `crates/video-prores-codec/src/bits.rs:206` | - |  |
| `codeword_roundtrip_all_codebooks` | 0 | `crates/video-prores-codec/src/bits.rs:241` | - |  |
| `exp_golomb5_equivalence` | 0 | `crates/video-prores-codec/src/bits.rs:266` | - |  |
| `idct8x8` | 1 | `crates/video-prores-codec/src/dct.rs:8` | - | In-place inverse DCT. `block` holds dequantised coefficients in raster order |
| `fdct8x8` | 1 | `crates/video-prores-codec/src/dct.rs:65` | - | In-place forward DCT (samples in, coefficients out; same layout as [`idct8x8`]). |
| `reference_idct` | 1 | `crates/video-prores-codec/src/dct.rs:94` | - |  |
| `idct_matches_double_reference` | 0 | `crates/video-prores-codec/src/dct.rs:116` | - |  |
| `fdct_inverts_idct` | 0 | `crates/video-prores-codec/src/dct.rs:139` | - |  |
| `impl Default for DecodeOptions::default` | 0 | `crates/video-prores-codec/src/decode.rs:18` | - |  |
| `probe` | 1 | `crates/video-prores-codec/src/decode.rs:24` | - | Parse only the frame header (cheap; for probing geometry and format). |
| `decode_frame` | 1 | `crates/video-prores-codec/src/decode.rs:29` | - | Decode one ProRes frame (the contents of one QuickTime sample) with default options. |
| `impl OutConv::new` | 1 | `crates/video-prores-codec/src/decode.rs:43` | - |  |
| `impl OutConv::conv` | 2 | `crates/video-prores-codec/src/decode.rs:55` | - |  |
| `impl Scratch::new` | 0 | `crates/video-prores-codec/src/decode.rs:92` | - |  |
| `decode_frame_with` | 2 | `crates/video-prores-codec/src/decode.rs:101` | - | Decode one ProRes frame. |
| `decode_frame_into` | 3 | `crates/video-prores-codec/src/decode.rs:109` | - | Decode one ProRes frame into `out`, reusing its plane allocations (the fastest way to decode |
| `be16` | 2 | `crates/video-prores-codec/src/decode.rs:237` | - |  |
| `qfactors` | 2 | `crates/video-prores-codec/src/decode.rs:242` | numerics | Dequantisation factors (matrix x scale) for a slice. |
| `decode_slice` | 8 | `crates/video-prores-codec/src/decode.rs:251` | - |  |
| `to_signed` | 1 | `crates/video-prores-codec/src/decode.rs:357` | - |  |
| `decode_component` | 5 | `crates/video-prores-codec/src/decode.rs:363` | - | Entropy-decode and dequantise one slice component into `blocks` (raster-order coefficients). |
| `decode_alpha` | 3 | `crates/video-prores-codec/src/decode.rs:416` | - | Decode a slice's alpha plane (raster order over the slice's 16 lines) into `out` |
| `put_block` | 9 | `crates/video-prores-codec/src/decode.rs:466` | - |  |
| `impl EncoderConfig::new` | 3 | `crates/video-prores-codec/src/encode.rs:29` | - |  |
| `impl Encoder::new` | 3 | `crates/video-prores-codec/src/encode.rs:56` | - |  |
| `impl Encoder::with_config` | 1 | `crates/video-prores-codec/src/encode.rs:60` | - |  |
| `impl Encoder::config` | 1 | `crates/video-prores-codec/src/encode.rs:64` | - |  |
| `impl Encoder::bits_per_mb` | 1 | `crates/video-prores-codec/src/encode.rs:68` | - |  |
| `impl Encoder::target_frame_bytes` | 1 | `crates/video-prores-codec/src/encode.rs:73` | - | The rate-control target for one frame in bytes (excluding alpha). |
| `impl Encoder::encode` | 2 | `crates/video-prores-codec/src/encode.rs:80` | - | Encode one frame. Input planes may be at any bit depth 8..=16 (10 is typical); the frame's |
| `load_block` | 6 | `crates/video-prores-codec/src/encode.rs:191` | - |  |
| `quantise` | 3 | `crates/video-prores-codec/src/encode.rs:207` | - |  |
| `encode_component` | 3 | `crates/video-prores-codec/src/encode.rs:222` | - |  |
| `component_bits` | 1 | `crates/video-prores-codec/src/encode.rs:271` | - |  |
| `impl SliceData::header_size` | 1 | `crates/video-prores-codec/src/encode.rs:283` | - |  |
| `impl SliceData::limit_bits` | 1 | `crates/video-prores-codec/src/encode.rs:286` | - |  |
| `prepare_slice` | 4 | `crates/video-prores-codec/src/encode.rs:291` | - |  |
| `quantise_all` | 3 | `crates/video-prores-codec/src/encode.rs:320` | - |  |
| `slice_bits` | 2 | `crates/video-prores-codec/src/encode.rs:327` | - |  |
| `choose_quantisers` | 3 | `crates/video-prores-codec/src/encode.rs:333` | - |  |
| `write_slice` | 2 | `crates/video-prores-codec/src/encode.rs:383` | - |  |
| `encode_alpha_slice` | 4 | `crates/video-prores-codec/src/encode.rs:411` | - |  |
| `encode_alpha` | 3 | `crates/video-prores-codec/src/encode.rs:432` | - |  |
| `impl FrameHeader::parse` | 1 | `crates/video-prores-codec/src/header.rs:38` | - | Parse the 8-byte frame prefix and frame header. Returns the header, the byte offset of the |
| `impl FrameHeader::write` | 1 | `crates/video-prores-codec/src/header.rs:136` | - | Serialise the frame header (without the 8-byte size/identifier prefix). |
| `impl PictureHeader::parse` | 1 | `crates/video-prores-codec/src/header.rs:187` | - |  |
| `impl PictureHeader::write` | 2 | `crates/video-prores-codec/src/header.rs:201` | - |  |
| `row_slice_widths` | 2 | `crates/video-prores-codec/src/header.rs:211` | - | Slice widths (in macroblocks) of one macroblock row: as many `1 << log2` slices as fit, then |
| `slice_widths` | 0 | `crates/video-prores-codec/src/header.rs:231` | - |  |
| `test_encode_decode_roundtrip` | 0 | `crates/video-prores-codec/src/lib.rs:36` | - |  |
| `test_prores_4444_with_alpha` | 0 | `crates/video-prores-codec/src/lib.rs:68` | - |  |
| `encode_prores_mov` | 3 | `crates/video-prores-codec/src/mov.rs:27` | - | Encode a sequence of same-sized frames into a QuickTime `.mov` container at the given frame rate. |
| `probe_prores_mov` | 1 | `crates/video-prores-codec/src/mov.rs:64` | - | Probe a QuickTime `.mov` container and return information about its first ProRes video track. |
| `decode_prores_mov` | 1 | `crates/video-prores-codec/src/mov.rs:110` | - | Decode all ProRes frames from a QuickTime `.mov` container. |
| `test_encode_probe_decode_mov_roundtrip` | 0 | `crates/video-prores-codec/src/mov.rs:139` | - |  |
| `cb` | 3 | `crates/video-prores-codec/src/tables.rs:12` | - |  |
| `qscale` | 1 | `crates/video-prores-codec/src/tables.rs:96` | - | Quantisation scale for a slice `quantization_index` (1..=224). |
| `scans_are_permutations` | 0 | `crates/video-prores-codec/src/tables.rs:118` | - |  |
| `basis_matches_cosines` | 0 | `crates/video-prores-codec/src/tables.rs:129` | - |  |
| `qscale_mapping` | 0 | `crates/video-prores-codec/src/tables.rs:140` | - |  |
| `impl ChromaFormat::chroma_width` | 2 | `crates/video-prores-codec/src/types.rs:31` | - | Width of a chroma plane for a given luma width. |
| `impl Default for ColorInfo::default` | 0 | `crates/video-prores-codec/src/types.rs:70` | numerics | BT.709 primaries, transfer and matrix. |
| `impl Profile::fourcc` | 1 | `crates/video-prores-codec/src/types.rs:103` | - | The QuickTime sample-entry four-character code. |
| `impl Profile::from_fourcc` | 1 | `crates/video-prores-codec/src/types.rs:114` | - |  |
| `impl Profile::chroma` | 1 | `crates/video-prores-codec/src/types.rs:118` | - |  |
| `impl Profile::nominal_mbps_1080p30` | 1 | `crates/video-prores-codec/src/types.rs:126` | - | Apple's published nominal data rate at 1920x1080, 29.97 fps, in Mbit/s. |
| `impl Profile::nominal_bits_per_mb` | 1 | `crates/video-prores-codec/src/types.rs:139` | - | Nominal coded bits per 16x16 macroblock (the nominal rate spread over the 8160 |
| `impl Frame::new` | 5 | `crates/video-prores-codec/src/types.rs:171` | - | A black (video-range) frame with the given geometry. |
| `impl Frame::chroma_width` | 1 | `crates/video-prores-codec/src/types.rs:191` | - |  |
| `impl Frame::is_interlaced` | 1 | `crates/video-prores-codec/src/types.rs:195` | - |  |

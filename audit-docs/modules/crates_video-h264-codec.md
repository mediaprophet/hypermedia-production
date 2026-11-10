# `crates/video-h264-codec`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Clean-room pure-Rust H.264 / AVC decoder, implemented from ITU-T Rec. H.264 (ISO/IEC 14496-10).

Supported: progressive (frame) coding, 8-bit 4:2:0, Baseline / Main / High profiles — CAVLC and CABAC,
I/P/B slices, 8x8 transform, custom scaling matrices, weighted prediction, spatial/temporal direct,
multiple slices, long-term references and all MMCOs, deblocking.

```no_run
let mut dec = video_h264_codec::Decoder::new();
let stream = std::fs::read("video.h264").unwrap();
for pic in dec.decode(&stream, 0).unwrap() {
println!("{}x{} poc {}", pic.width, pic.height, pic.poc);
}
for pic in dec.flush() {
let _ = pic;
}
```_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Cabac<'a>::new` | 4 | `crates/video-h264-codec/src/cabac.rs:81` | - | Create an engine over `data` (slice RBSP) starting at bit `pos` (must be byte aligned), |
| `impl Cabac<'a>::init_contexts` | 3 | `crates/video-h264-codec/src/cabac.rs:88` | - |  |
| `impl Cabac<'a>::init_engine` | 1 | `crates/video-h264-codec/src/cabac.rs:100` | - | 9.3.1.2: codIRange = 510, codIOffset = read_bits(9) (from the current byte-aligned position). |
| `impl Cabac<'a>::refill` | 1 | `crates/video-h264-codec/src/cabac.rs:113` | - | Load 32 more bits (zeros past the end of the data). |
| `impl Cabac<'a>::bit_pos` | 1 | `crates/video-h264-codec/src/cabac.rs:129` | - | Bit position of the next bit not consumed by the arithmetic decoder (for I_PCM alignment |
| `impl Cabac<'a>::set_bit_pos` | 2 | `crates/video-h264-codec/src/cabac.rs:133` | - | Continue reading at byte-aligned bit position `pos` (call `init_engine` afterwards). |
| `impl Cabac<'a>::data` | 1 | `crates/video-h264-codec/src/cabac.rs:138` | - |  |
| `impl Cabac<'a>::overrun` | 1 | `crates/video-h264-codec/src/cabac.rs:142` | - | True when the reader has run past the end of the data (corrupt stream). |
| `impl Cabac<'a>::renorm` | 1 | `crates/video-h264-codec/src/cabac.rs:147` | - |  |
| `impl Cabac<'a>::decode_decision` | 2 | `crates/video-h264-codec/src/cabac.rs:161` | - | 9.3.3.2.1, branch-free: the MPS / LPS outcome of a context-coded bin is close to random |
| `impl Cabac<'a>::decode_bypass` | 1 | `crates/video-h264-codec/src/cabac.rs:179` | - |  |
| `impl Cabac<'a>::decode_terminate` | 1 | `crates/video-h264-codec/src/cabac.rs:190` | - |  |
| `impl Enc::new` | 1 | `crates/video-h264-codec/src/cabac.rs:216` | - |  |
| `impl Enc::put` | 2 | `crates/video-h264-codec/src/cabac.rs:219` | - |  |
| `impl Enc::renorm` | 1 | `crates/video-h264-codec/src/cabac.rs:230` | - |  |
| `impl Enc::encode` | 3 | `crates/video-h264-codec/src/cabac.rs:245` | - |  |
| `impl Enc::bypass` | 2 | `crates/video-h264-codec/src/cabac.rs:261` | - |  |
| `impl Enc::terminate_flush` | 1 | `crates/video-h264-codec/src/cabac.rs:276` | - |  |
| `context_init_formula` | 0 | `crates/video-h264-codec/src/cabac.rs:293` | - |  |
| `round_trip_random_bins` | 0 | `crates/video-h264-codec/src/cabac.rs:304` | - |  |
| `impl SliceDecoder<'_>::decode_cabac` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:42` | - | Decode slice_data() with CABAC. `rbsp` is the whole slice RBSP. |
| `impl SliceDecoder<'_>::decode_skip_flag` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:68` | - |  |
| `impl SliceDecoder<'_>::decode_mb_type_i` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:77` | - | Intra mb_type suffix / I-slice mb_type (Table 9-36). `base` is 3 (I slice) or 17/32 (prefix |
| `impl SliceDecoder<'_>::decode_mb_type` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:104` | - |  |
| `impl SliceDecoder<'_>::decode_sub_mb_type` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:148` | - |  |
| `impl SliceDecoder<'_>::decode_ref_idx` | 5 | `crates/video-h264-codec/src/cabac_mb.rs:181` | - | ref_idx_lX for the partition whose top-left 4x4 block is (x, y) (9.3.3.1.1.6). |
| `impl SliceDecoder<'_>::decode_mvd` | 6 | `crates/video-h264-codec/src/cabac_mb.rs:216` | - | One mvd component for the (sub-)partition with top-left 4x4 block (x, y) (9.3.3.1.1.7, UEG3). |
| `impl SliceDecoder<'_>::set_cur_ref` | 7 | `crates/video-h264-codec/src/cabac_mb.rs:265` | - |  |
| `impl SliceDecoder<'_>::set_cur_mvd` | 7 | `crates/video-h264-codec/src/cabac_mb.rs:274` | - |  |
| `impl SliceDecoder<'_>::decode_cbp` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:284` | - |  |
| `impl SliceDecoder<'_>::cbp_luma_cond` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:318` | - | condTermFlagN for coded_block_pattern luma bin from a neighbouring MB; `b8n` is the neighbouring |
| `impl SliceDecoder<'_>::decode_qp_delta` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:330` | - |  |
| `impl SliceDecoder<'_>::cbf_luma_cond` | 4 | `crates/video-h264-codec/src/cabac_mb.rs:347` | - | coded_block_flag ctxIdxInc condition for a neighbouring luma 4x4 block (cat 0..2) or chroma. |
| `impl SliceDecoder<'_>::cbf_dc_cond` | 4 | `crates/video-h264-codec/src/cabac_mb.rs:358` | - |  |
| `impl SliceDecoder<'_>::cbf_chroma_ac_cond` | 5 | `crates/video-h264-codec/src/cabac_mb.rs:367` | - |  |
| `impl SliceDecoder<'_>::residual_block_cabac` | 6 | `crates/video-h264-codec/src/cabac_mb.rs:381` | - | residual_block_cabac for ctxBlockCat `cat` (0..=5). Coefficients are written to `coeff` |
| `impl SliceDecoder<'_>::residual_cabac` | 3 | `crates/video-h264-codec/src/cabac_mb.rs:449` | - |  |
| `impl SliceDecoder<'_>::decode_mb_cabac` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:522` | - | Parse and reconstruct one macroblock_layer() with CABAC. |
| `impl SliceDecoder<'_>::decode_t8x8_flag` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:665` | - |  |
| `impl SliceDecoder<'_>::decode_chroma_mode` | 2 | `crates/video-h264-codec/src/cabac_mb.rs:671` | - |  |
| `impl VlcTable::build` | 1 | `crates/video-h264-codec/src/cavlc.rs:15` | - |  |
| `impl VlcTable::decode` | 2 | `crates/video-h264-codec/src/cavlc.rs:46` | - |  |
| `tables` | 0 | `crates/video-h264-codec/src/cavlc.rs:66` | - |  |
| `simple` | 1 | `crates/video-h264-codec/src/cavlc.rs:73` | - |  |
| `init_tables` | 0 | `crates/video-h264-codec/src/cavlc.rs:88` | physics | Force table construction (so the first slice does not pay for it). |
| `residual_block` | 6 | `crates/video-h264-codec/src/cavlc.rs:95` | - | Parse residual_block_cavlc(). Coefficients are written to `coeff[start_idx..=end_idx]` in scan order |
| `kraft` | 1 | `crates/video-h264-codec/src/cavlc.rs:187` | - |  |
| `prefix_free` | 1 | `crates/video-h264-codec/src/cavlc.rs:191` | - |  |
| `coeff_token_tables_are_prefix_codes` | 0 | `crates/video-h264-codec/src/cavlc.rs:204` | - |  |
| `total_zeros_and_run_before_are_complete_prefix_codes` | 0 | `crates/video-h264-codec/src/cavlc.rs:217` | - |  |
| `write_code` | 2 | `crates/video-h264-codec/src/cavlc.rs:241` | - |  |
| `decodes_spec_style_example` | 0 | `crates/video-h264-codec/src/cavlc.rs:248` | - |  |
| `ref_ids8` | 2 | `crates/video-h264-codec/src/deblock.rs:16` | - | Referenced picture ids per 8x8 block and list (u32::MAX = list unused). |
| `block_motion` | 3 | `crates/video-h264-codec/src/deblock.rs:26` | - |  |
| `mv_far` | 2 | `crates/video-h264-codec/src/deblock.rs:34` | - |  |
| `motion_bs` | 2 | `crates/video-h264-codec/src/deblock.rs:39` | - |  |
| `clip3` | 3 | `crates/video-h264-codec/src/deblock.rs:76` | - |  |
| `filter8` | 6 | `crates/video-h264-codec/src/deblock.rs:83` | graph | Filter one set of eight samples p3 p2 p1 p0 \| q0 q1 q2 q3 across an edge (8.7.2.3 / 8.7.2.4): |
| `filter_lanes` | 3 | `crates/video-h264-codec/src/deblock.rs:151` | graph | Filter one edge for `N` lines at once (lane `i` = line `i` across the edge). `s[k][i]` holds |
| `filter_vertical` | 6 | `crates/video-h264-codec/src/deblock.rs:230` | graph | Filter a vertical edge whose q0 column is at `x`, for `N` lines starting at row `y`. |
| `filter_horizontal` | 6 | `crates/video-h264-codec/src/deblock.rs:258` | graph | Filter a horizontal edge whose q0 row is `y`, for `N` columns starting at `x`. |
| `deblock_mb` | 3 | `crates/video-h264-codec/src/deblock.rs:283` | - | Deblock one macroblock (macroblocks must be processed in raster order). |
| `lane_filter_matches_per_line_reference` | 0 | `crates/video-h264-codec/src/deblock.rs:420` | graph | The vectorised edge filter equals the per-line reference [`filter8`] on random edges |
| `edge_filter_speed` | 0 | `crates/video-h264-codec/src/deblock.rs:483` | graph | Rough timing of the edge filters (`cargo test --release -p video-h264-codec --lib edge_filter_speed -- --ignored --nocapture`). |
| `impl Default for Decoder::default` | 0 | `crates/video-h264-codec/src/decoder.rs:104` | - |  |
| `default_threads` | 0 | `crates/video-h264-codec/src/decoder.rs:111` | - | The worker threads [`Decoder::new`] uses: the available cores (at most 16) with the `threads` |
| `impl Decoder::new` | 0 | `crates/video-h264-codec/src/decoder.rs:124` | - | A decoder using all available cores (with the `threads` feature). |
| `impl Decoder::with_threads` | 1 | `crates/video-h264-codec/src/decoder.rs:129` | - | A decoder using up to `threads` worker threads (1 = decode on the calling thread). |
| `impl Decoder::threads` | 1 | `crates/video-h264-codec/src/decoder.rs:164` | - | Worker threads decoding pictures (1: on the calling thread). |
| `impl Decoder::set_draft` | 2 | `crates/video-h264-codec/src/decoder.rs:174` | - | Draft mode (off by default) for reduced-resolution playback: pictures submitted from now |
| `impl Decoder::draft` | 1 | `crates/video-h264-codec/src/decoder.rs:178` | - |  |
| `impl Decoder::from_avcc` | 1 | `crates/video-h264-codec/src/decoder.rs:183` | - | Configure from an `avcC` (AVCDecoderConfigurationRecord) box payload. |
| `impl Decoder::configure_avcc` | 2 | `crates/video-h264-codec/src/decoder.rs:190` | - | Parse an `avcC` record: SPS/PPS and the NAL length size used by [`Decoder::decode`]. |
| `impl Decoder::nal_length_size` | 1 | `crates/video-h264-codec/src/decoder.rs:216` | - | Length of NAL length prefixes (from avcC), or None for Annex-B input. |
| `impl Decoder::decode` | 3 | `crates/video-h264-codec/src/decoder.rs:224` | - | Decode one access unit (all NAL units of one picture, Annex-B or length-prefixed per |
| `impl Decoder::flush` | 1 | `crates/video-h264-codec/src/decoder.rs:256` | - | Output all remaining pictures (end of stream). |
| `impl Decoder::stats` | 1 | `crates/video-h264-codec/src/decoder.rs:269` | - | Statistics accumulated so far (macroblock counts include finished pictures only). |
| `impl Decoder::stat` | 2 | `crates/video-h264-codec/src/decoder.rs:273` | - |  |
| `impl Decoder::take_error` | 1 | `crates/video-h264-codec/src/decoder.rs:281` | - | First error reported by a decoding job since the last call (also returned by |
| `impl Decoder::handle_nal` | 3 | `crates/video-h264-codec/src/decoder.rs:285` | - |  |
| `impl Decoder::level_scale` | 2 | `crates/video-h264-codec/src/decoder.rs:314` | - |  |
| `impl Decoder::handle_slice` | 4 | `crates/video-h264-codec/src/decoder.rs:324` | - |  |
| `impl Decoder::start_picture` | 4 | `crates/video-h264-codec/src/decoder.rs:403` | - |  |
| `impl Decoder::submit_pending` | 1 | `crates/video-h264-codec/src/decoder.rs:467` | - | Start decoding the pending picture and do its reference marking / DPB insertion. |
| `impl Decoder::dispatch` | 4 | `crates/video-h264-codec/src/decoder.rs:483` | - |  |
| `impl Decoder::emit` | 2 | `crates/video-h264-codec/src/decoder.rs:500` | - |  |
| `run_job` | 4 | `crates/video-h264-codec/src/decoder.rs:506` | - | Decode all slices of one picture and publish its rows. |
| `decode_slice` | 2 | `crates/video-h264-codec/src/decoder.rs:571` | - |  |
| `output_meta` | 3 | `crates/video-h264-codec/src/decoder.rs:582` | - |  |
| `make_picture` | 1 | `crates/video-h264-codec/src/decoder.rs:597` | - |  |
| `impl Dpb<F>::new` | 0 | `crates/video-h264-codec/src/dpb.rs:61` | - |  |
| `impl Dpb<F>::frame_num_wrap` | 3 | `crates/video-h264-codec/src/dpb.rs:65` | - |  |
| `impl Dpb<F>::num_refs` | 1 | `crates/video-h264-codec/src/dpb.rs:69` | - |  |
| `impl Dpb<F>::prune` | 1 | `crates/video-h264-codec/src/dpb.rs:74` | - | Remove entries not needed for output and not used for reference. |
| `impl Dpb<F>::bump` | 2 | `crates/video-h264-codec/src/dpb.rs:79` | - | Output the picture with the smallest POC (C.4.5.3 "bumping"). Returns false if none waits. |
| `impl Dpb<F>::flush` | 2 | `crates/video-h264-codec/src/dpb.rs:91` | - | Output everything in POC order and clear non-reference entries. |
| `impl Dpb<F>::idr` | 3 | `crates/video-h264-codec/src/dpb.rs:97` | - | IDR: mark all references unused; output (or drop) prior pictures. |
| `impl Dpb<F>::sliding_window` | 4 | `crates/video-h264-codec/src/dpb.rs:110` | - | Sliding window marking (8.2.5.3). |
| `impl Dpb<F>::fill_frame_num_gap` | 7 | `crates/video-h264-codec/src/dpb.rs:131` | - | Insert "non-existing" frames for a frame_num gap (8.2.5.2). `make_frame` gives each one's |
| `impl Dpb<F>::build_ref_lists` | 5 | `crates/video-h264-codec/src/dpb.rs:164` | - | Initial reference picture lists (8.2.4.2) + modification (8.2.4.3) for a frame slice, each |
| `impl Dpb<F>::apply_mmcos` | 5 | `crates/video-h264-codec/src/dpb.rs:264` | - | Adaptive memory control (8.2.5.4). Returns true if MMCO 5 was executed. |
| `impl Dpb<F>::store_picture` | 8 | `crates/video-h264-codec/src/dpb.rs:332` | - | Reference marking for the just-decoded picture and insertion into the DPB with output bumping. |
| `impl Default for Dpb<F>::default` | 0 | `crates/video-h264-codec/src/dpb.rs:413` | - |  |
| `failed_gap_update_stops_before_frame_synthesis` | 0 | `crates/video-h264-codec/src/dpb.rs:422` | - |  |
| `invalid` | 1 | `crates/video-h264-codec/src/error.rs:22` | - |  |
| `unsupported` | 1 | `crates/video-h264-codec/src/error.rs:26` | - |  |
| `fetch_window` | 6 | `crates/video-h264-codec/src/inter.rs:19` | - | Copy the (bw+5)x(bh+5) window whose top-left is (x-2, y-2) into `win` (stride WIN), clamping |
| `tap6` | 6 | `crates/video-h264-codec/src/inter.rs:44` | - |  |
| `clip1` | 1 | `crates/video-h264-codec/src/inter.rs:49` | - |  |
| `copy_n` | 3 | `crates/video-h264-codec/src/inter.rs:55` | - | Copy `n` bytes with a fixed-size move for the common block widths. |
| `fixed` | 2 | `crates/video-h264-codec/src/inter.rs:56` | - |  |
| `avg` | 2 | `crates/video-h264-codec/src/inter.rs:71` | - |  |
| `htap_row` | 3 | `crates/video-h264-codec/src/inter.rs:77` | - | Horizontal 6-tap (unclipped) at every position of `row` for `n` outputs: out[c] uses row[c..c + 6]. |
| `vtap_row` | 3 | `crates/video-h264-codec/src/inter.rs:86` | - | Vertical 6-tap (unclipped) over six rows at columns 0..n. |
| `mc_luma` | 9 | `crates/video-h264-codec/src/inter.rs:97` | - | Luma sample interpolation for a bw x bh block (8.4.2.2.1). (x, y) is the integer sample position |
| `mc_luma_win` | 8 | `crates/video-h264-codec/src/inter.rs:113` | - | Luma interpolation from a prepared window: `src[0]` is the sample at (x - 2, y - 2) and `ss` the |
| `mc_chroma` | 9 | `crates/video-h264-codec/src/inter.rs:222` | - | Chroma sample interpolation (8.4.2.2.2) for a bw x bh block at integer position (x, y) with |
| `mc_chroma_win` | 8 | `crates/video-h264-codec/src/inter.rs:245` | - | Chroma interpolation from a prepared (bw + 1) x (bh + 1) window whose first sample is the integer |
| `weighted_store` | 8 | `crates/video-h264-codec/src/inter.rs:276` | - | Combine prediction(s) into `dst` (stride `ds`). `p0`/`p1` have stride `ps`. |
| `mc_luma_ref` | 9 | `crates/video-h264-codec/src/inter.rs:332` | - | Luma sample interpolation for a bw x bh block. (x, y) is the integer sample position |
| `plane` | 3 | `crates/video-h264-codec/src/inter.rs:435` | - |  |
| `full_sample_copy_and_clamp` | 0 | `crates/video-h264-codec/src/inter.rs:440` | - |  |
| `half_sample_constant_is_constant` | 0 | `crates/video-h264-codec/src/inter.rs:452` | - |  |
| `half_sample_horizontal_ramp` | 0 | `crates/video-h264-codec/src/inter.rs:467` | - |  |
| `fast_luma_matches_reference` | 0 | `crates/video-h264-codec/src/inter.rs:479` | - |  |
| `weighting` | 0 | `crates/video-h264-codec/src/inter.rs:504` | - |  |
| `impl Edge::t` | 2 | `crates/video-h264-codec/src/intra.rs:23` | - | p[x, -1] for x >= -1 |
| `impl Edge::l` | 2 | `crates/video-h264-codec/src/intra.rs:28` | - | p[-1, y] for y >= -1 |
| `load_edge` | 6 | `crates/video-h264-codec/src/intra.rs:33` | graph |  |
| `pred_nxn` | 6 | `crates/video-h264-codec/src/intra.rs:63` | graph | Directional / DC prediction shared by Intra_4x4 and Intra_8x8 (8.3.1.2.x, 8.3.2.2.x). |
| `pred4x4` | 6 | `crates/video-h264-codec/src/intra.rs:188` | - | Intra_4x4 prediction written into `plane` at (x, y). |
| `pred8x8` | 6 | `crates/video-h264-codec/src/intra.rs:194` | - | Intra_8x8 prediction (with reference sample filtering) written into `plane` at (x, y). |
| `clip` | 1 | `crates/video-h264-codec/src/intra.rs:223` | - |  |
| `pred16x16` | 6 | `crates/video-h264-codec/src/intra.rs:228` | - | Intra_16x16 prediction (8.3.3) for the macroblock at luma (x, y). |
| `pred_chroma` | 6 | `crates/video-h264-codec/src/intra.rs:292` | - | Chroma intra prediction for a 4:2:0 macroblock (8x8 block at chroma (x, y)), 8.3.4. |
| `setup` | 0 | `crates/video-h264-codec/src/intra.rs:381` | - |  |
| `i4_vertical_horizontal_dc` | 0 | `crates/video-h264-codec/src/intra.rs:397` | - |  |
| `i4_diag_down_left_hand_computed` | 0 | `crates/video-h264-codec/src/intra.rs:410` | - |  |
| `i4_diag_down_right_hand_computed` | 0 | `crates/video-h264-codec/src/intra.rs:420` | - |  |
| `i4_top_right_substitution` | 0 | `crates/video-h264-codec/src/intra.rs:432` | - |  |
| `i16_plane_flat_is_flat` | 0 | `crates/video-h264-codec/src/intra.rs:441` | - |  |
| `chroma_dc_rules` | 0 | `crates/video-h264-codec/src/intra.rs:449` | - |  |
| `impl Part::num_parts` | 1 | `crates/video-h264-codec/src/mbtypes.rs:20` | - |  |
| `impl Part::rect` | 2 | `crates/video-h264-codec/src/mbtypes.rs:28` | 2d | (x, y, w, h) of partition `i` in 4x4-block units. |
| `impl SubPart::num_parts` | 1 | `crates/video-h264-codec/src/mbtypes.rs:49` | - |  |
| `impl SubPart::rect` | 2 | `crates/video-h264-codec/src/mbtypes.rs:57` | 2d | (x, y, w, h) of sub-partition `j` relative to the 8x8 block, in 4x4 units. |
| `intra_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:83` | - | Intra mb_type (I-slice numbering 0..=25). |
| `p_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:101` | - | P-slice mb_type 0..=4 (5.. are intra, handled by the caller). |
| `b_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:118` | - | B-slice mb_type 0..=22 (23.. are intra). |
| `p_sub_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:159` | - |  |
| `b_sub_mb_type` | 1 | `crates/video-h264-codec/src/mbtypes.rs:170` | - |  |
| `cbp_table_is_permutation` | 0 | `crates/video-h264-codec/src/mbtypes.rs:206` | - |  |
| `i16_types` | 0 | `crates/video-h264-codec/src/mbtypes.rs:217` | - |  |
| `b_types` | 0 | `crates/video-h264-codec/src/mbtypes.rs:227` | - |  |
| `read_ue_plus` | 2 | `crates/video-h264-codec/src/params.rs:7` | - |  |
| `impl ScalingMatrices::flat` | 0 | `crates/video-h264-codec/src/params.rs:22` | - |  |
| `impl ScalingMatrices::is_flat` | 1 | `crates/video-h264-codec/src/params.rs:25` | - |  |
| `zz4_to_raster` | 1 | `crates/video-h264-codec/src/params.rs:37` | - |  |
| `zz8_to_raster` | 1 | `crates/video-h264-codec/src/params.rs:44` | - |  |
| `impl ZzLists::to_matrices` | 1 | `crates/video-h264-codec/src/params.rs:53` | - |  |
| `impl ZzLists::flat` | 0 | `crates/video-h264-codec/src/params.rs:61` | - |  |
| `parse_scaling_list` | 1 | `crates/video-h264-codec/src/params.rs:67` | - | Result of parsing one scaling_list(): Some(list) or None meaning "use default". |
| `parse_scaling_lists` | 3 | `crates/video-h264-codec/src/params.rs:88` | - | Parse scaling lists with a fall-back rule. `fallback` gives the lists used for rule B |
| `parse_hrd` | 1 | `crates/video-h264-codec/src/params.rs:157` | - |  |
| `impl Default for Vui::default` | 0 | `crates/video-h264-codec/src/params.rs:194` | - |  |
| `parse_vui` | 1 | `crates/video-h264-codec/src/params.rs:247` | - |  |
| `impl Sps::parse` | 1 | `crates/video-h264-codec/src/params.rs:341` | - |  |
| `impl Sps::chroma_array_type` | 1 | `crates/video-h264-codec/src/params.rs:447` | - | ChromaArrayType. |
| `impl Sps::frame_height_in_mbs` | 1 | `crates/video-h264-codec/src/params.rs:450` | - |  |
| `impl Sps::width` | 1 | `crates/video-h264-codec/src/params.rs:454` | - | Decoded (uncropped) luma width. |
| `impl Sps::height` | 1 | `crates/video-h264-codec/src/params.rs:457` | - |  |
| `impl Sps::max_frame_num` | 1 | `crates/video-h264-codec/src/params.rs:460` | - |  |
| `impl Sps::crop_unit` | 1 | `crates/video-h264-codec/src/params.rs:464` | - | (CropUnitX, CropUnitY). |
| `impl Sps::crop_rect` | 1 | `crates/video-h264-codec/src/params.rs:473` | 2d | Cropping rectangle in luma samples: (x, y, width, height). |
| `impl Sps::max_dpb_frames` | 1 | `crates/video-h264-codec/src/params.rs:484` | - | MaxDpbFrames from Table A-1 (and the VUI max_dec_frame_buffering when present). |
| `impl Sps::max_num_reorder_frames` | 1 | `crates/video-h264-codec/src/params.rs:516` | - | Number of frames that may precede a frame in decoding order and follow it in output order. |
| `impl Sps::check_supported` | 1 | `crates/video-h264-codec/src/params.rs:527` | - |  |
| `impl Pps::parse` | 2 | `crates/video-h264-codec/src/params.rs:572` | - | Parse a PPS. Needs the SPS table because the tail depends on chroma_format_idc. |
| `scaling_list_default_signal` | 0 | `crates/video-h264-codec/src/params.rs:677` | - |  |
| `scaling_list_repeat_last` | 0 | `crates/video-h264-codec/src/params.rs:688` | - |  |
| `parse_minimal_baseline_sps` | 0 | `crates/video-h264-codec/src/params.rs:702` | - |  |
| `baseline_with_crop` | 4 | `crates/video-h264-codec/src/params.rs:726` | - |  |
| `sps_crop_offsets_refuse_overflow_without_panicking_and_keep_valid_crop` | 0 | `crates/video-h264-codec/src/params.rs:737` | - |  |
| `exp_golomb_increment_rejects_overflow_and_preserves_valid_value` | 0 | `crates/video-h264-codec/src/params.rs:748` | - |  |
| `impl Planes::new` | 2 | `crates/video-h264-codec/src/picture.rs:17` | - |  |
| `impl Planes::gray` | 2 | `crates/video-h264-codec/src/picture.rs:21` | - |  |
| `impl MbKind::is_intra` | 1 | `crates/video-h264-codec/src/picture.rs:46` | - |  |
| `impl MbState::kind_is_skip` | 1 | `crates/video-h264-codec/src/picture.rs:85` | - |  |
| `impl Default for MbState::default` | 0 | `crates/video-h264-codec/src/picture.rs:91` | - |  |
| `impl Frame::new` | 4 | `crates/video-h264-codec/src/picture.rs:146` | - |  |
| `impl Frame::mb_h` | 1 | `crates/video-h264-codec/src/picture.rs:150` | - |  |
| `impl Frame::publish` | 3 | `crates/video-h264-codec/src/picture.rs:155` | - | Publish MB row `r` (ignored if already published). |
| `impl Frame::is_published` | 2 | `crates/video-h264-codec/src/picture.rs:159` | - |  |
| `impl Frame::row` | 2 | `crates/video-h264-codec/src/picture.rs:165` | - | MB row `r`, waiting until it has been published. |
| `impl Frame::wait_complete` | 1 | `crates/video-h264-codec/src/picture.rs:177` | - | Wait until the whole frame is available. |
| `impl Frame::make_row` | 4 | `crates/video-h264-codec/src/picture.rs:184` | - | Build a published row from a contiguous plane set (rows of MB row `r`) and motion data. |
| `impl Frame::from_planes` | 3 | `crates/video-h264-codec/src/picture.rs:220` | - | A frame with every row published from `planes` and no motion (intra), e.g. "non-existing" |
| `impl Frame::luma_window` | 7 | `crates/video-h264-codec/src/picture.rs:232` | - | Copy the w x h luma window at (x0, y0) (clamped to the picture) into `out` (stride `os`). |
| `impl Frame::chroma_window` | 8 | `crates/video-h264-codec/src/picture.rs:244` | - | Copy the w x h window of chroma component `c` at (x0, y0) (clamped) into `out`. |
| `impl Frame::copy_cropped` | 2 | `crates/video-h264-codec/src/picture.rs:256` | - | Crop and copy the frame into planar buffers (waits for completion). |
| `set_plane_allocator` | 1 | `crates/video-h264-codec/src/picture.rs:281` | - | Where output planes come from: an empty buffer with room for the given number of samples. |
| `plane_allocator` | 0 | `crates/video-h264-codec/src/picture.rs:285` | - |  |
| `copy_line` | 4 | `crates/video-h264-codec/src/picture.rs:292` | - | Copy `w` samples of `line` starting at x0 into `out`; when possible a fixed-size block of `N` |
| `copy_clamped` | 3 | `crates/video-h264-codec/src/picture.rs:306` | graph | Copy `out.len()` samples of `line` starting at x0, replicating edge samples outside the line. |
| `impl RefPic::poc` | 1 | `crates/video-h264-codec/src/picture.rs:328` | - |  |
| `impl RefPic::id` | 1 | `crates/video-h264-codec/src/picture.rs:331` | - |  |
| `impl NalHeader::parse` | 1 | `crates/video-h264-codec/src/slice.rs:35` | - |  |
| `impl SliceType::from_raw` | 1 | `crates/video-h264-codec/src/slice.rs:51` | - |  |
| `impl SliceType::is_intra` | 1 | `crates/video-h264-codec/src/slice.rs:60` | - |  |
| `impl SliceType::is_b` | 1 | `crates/video-h264-codec/src/slice.rs:63` | - |  |
| `impl SliceType::is_p` | 1 | `crates/video-h264-codec/src/slice.rs:66` | - |  |
| `impl SliceHeader::qp` | 2 | `crates/video-h264-codec/src/slice.rs:146` | - |  |
| `impl SliceHeader::has_mmco5` | 1 | `crates/video-h264-codec/src/slice.rs:149` | - |  |
| `impl SliceHeader::parse` | 3 | `crates/video-h264-codec/src/slice.rs:154` | - | Parse the slice header. `pps_lookup` resolves pic_parameter_set_id to (PPS, SPS). |
| `parameter_sets` | 0 | `crates/video-h264-codec/src/slice.rs:388` | - |  |
| `p_slice_rejects_oversized_ref_mod_and_mmco_differences` | 0 | `crates/video-h264-codec/src/slice.rs:407` | - |  |
| `poc_overflow_is_rejected_and_normal_values_survive` | 0 | `crates/video-h264-codec/src/slice.rs:422` | 3d |  |
| `p_slice_ref_count_overflow_is_error_not_panic_or_wrapped_zero` | 0 | `crates/video-h264-codec/src/slice.rs:442` | - |  |
| `is_new_picture` | 3 | `crates/video-h264-codec/src/slice.rs:465` | - | Is `sh` the first slice of a new primary coded picture relative to `prev` (7.4.1.2.4)? |
| `impl Poc::frame` | 1 | `crates/video-h264-codec/src/slice.rs:495` | - |  |
| `impl PocState::compute` | 3 | `crates/video-h264-codec/src/slice.rs:502` | - | Compute TopFieldOrderCnt/BottomFieldOrderCnt of a frame (8.2.1.1-8.2.1.3). |
| `checked` | 1 | `crates/video-h264-codec/src/slice.rs:503` | - |  |
| `narrow` | 1 | `crates/video-h264-codec/src/slice.rs:504` | - |  |
| `impl PocState::frame_num_offset` | 4 | `crates/video-h264-codec/src/slice.rs:550` | - |  |
| `impl PocState::update` | 3 | `crates/video-h264-codec/src/slice.rs:557` | - | Update after a picture has been decoded. `poc` is the value returned by [`PocState::compute`] |
| `impl PocState::update_gap_frame` | 3 | `crates/video-h264-codec/src/slice.rs:578` | - | Account for a "non-existing" frame inferred by the frame_num gap process (8.2.5.2). |
| `temporal_direct_component` | 2 | `crates/video-h264-codec/src/slicedec.rs:17` | - |  |
| `temporal_direct_checks_both_vectors` | 0 | `crates/video-h264-codec/src/slicedec.rs:29` | - |  |
| `impl PicState::new` | 1 | `crates/video-h264-codec/src/slicedec.rs:68` | - |  |
| `impl PicState::reset` | 2 | `crates/video-h264-codec/src/slicedec.rs:88` | - | Reuse the buffers of a finished picture of the same size for a new frame. |
| `impl PicState::mb_done` | 2 | `crates/video-h264-codec/src/slicedec.rs:107` | - | Record a decoded macroblock and deblock / publish rows that became final. |
| `impl PicState::flush_rows` | 2 | `crates/video-h264-codec/src/slicedec.rs:121` | - | Deblock and publish rows. With `finished`, everything remaining is processed (also rows with |
| `impl Scratch::new` | 0 | `crates/video-h264-codec/src/slicedec.rs:195` | - |  |
| `predict_block` | 9 | `crates/video-h264-codec/src/slicedec.rs:241` | - | Interpolate a bw x bh luma block at (px, py) with motion vector `mv` from reference `f` into `y`, |
| `median` | 3 | `crates/video-h264-codec/src/slicedec.rs:260` | - |  |
| `poc_delta` | 2 | `crates/video-h264-codec/src/slicedec.rs:266` | - | H.264 temporal weighting uses clipped POC distances; widen before |
| `clips_extreme_pocs_before_narrowing` | 0 | `crates/video-h264-codec/src/slicedec.rs:274` | - |  |
| `impl SliceDecoder<'a>::new` | 6 | `crates/video-h264-codec/src/slicedec.rs:281` | - |  |
| `impl SliceDecoder<'a>::build_weights` | 4 | `crates/video-h264-codec/src/slicedec.rs:331` | - |  |
| `impl SliceDecoder<'a>::start_mb` | 2 | `crates/video-h264-codec/src/slicedec.rs:377` | - | Set up the current macroblock address and neighbour availability. |
| `impl SliceDecoder<'a>::mb` | 1 | `crates/video-h264-codec/src/slicedec.rs:397` | - |  |
| `impl SliceDecoder<'a>::mb_mut` | 1 | `crates/video-h264-codec/src/slicedec.rs:401` | - |  |
| `impl SliceDecoder<'a>::intra_nb` | 2 | `crates/video-h264-codec/src/slicedec.rs:406` | - | Neighbour MB usable for intra prediction (constrained_intra_pred aware). |
| `impl SliceDecoder<'a>::chroma_qp` | 3 | `crates/video-h264-codec/src/slicedec.rs:413` | - |  |
| `impl SliceDecoder<'a>::nc_luma` | 2 | `crates/video-h264-codec/src/slicedec.rs:418` | - | nC for a luma 4x4 block (raster index) - 9.2.1. |
| `impl SliceDecoder<'a>::nc_chroma` | 3 | `crates/video-h264-codec/src/slicedec.rs:431` | - |  |
| `impl SliceDecoder<'a>::nb_motion` | 4 | `crates/video-h264-codec/src/slicedec.rs:445` | - | Motion data of the neighbouring 4x4 block at (x, y) (4x4 units, relative to the current MB). |
| `impl SliceDecoder<'a>::nb_abc` | 5 | `crates/video-h264-codec/src/slicedec.rs:477` | - | Returns (A, B, C) neighbour motion for a partition at (x, y) of width w (4x4 units), with C |
| `impl SliceDecoder<'a>::mvp` | 8 | `crates/video-h264-codec/src/slicedec.rs:490` | - | Luma motion vector prediction (8.4.1.3). `shape`: 1 = 16x8, 2 = 8x16, 0 = other. |
| `impl SliceDecoder<'a>::median_pred` | 4 | `crates/video-h264-codec/src/slicedec.rs:510` | - |  |
| `impl SliceDecoder<'a>::set_motion` | 7 | `crates/video-h264-codec/src/slicedec.rs:527` | - | Store motion for a rectangle (4x4 units) of the current MB and mark it decoded. |
| `impl SliceDecoder<'a>::colocated` | 2 | `crates/video-h264-codec/src/slicedec.rs:553` | - | Co-located (mvCol, refIdxCol, referenced picture id) for 4x4 block `raster` of the current MB. |
| `impl SliceDecoder<'a>::spatial_direct_base` | 1 | `crates/video-h264-codec/src/slicedec.rs:575` | - | Spatial direct: MB-level reference indices, predicted mvs and directZeroPrediction (8.4.1.2.2). |
| `impl SliceDecoder<'a>::derive_direct` | 2 | `crates/video-h264-codec/src/slicedec.rs:597` | - | Derive direct-mode motion for the 8x8 blocks in `b8_mask` of the current MB. |
| `impl SliceDecoder<'a>::derive_p_skip` | 1 | `crates/video-h264-codec/src/slicedec.rs:652` | - | P_Skip motion (8.4.1.1). |
| `impl SliceDecoder<'a>::derive_inter_motion` | 1 | `crates/video-h264-codec/src/slicedec.rs:664` | - | Derive mvs of the current (non-skip) inter MB from `self.cur` (ref_idx, mvd). |
| `impl SliceDecoder<'a>::weight_for` | 3 | `crates/video-h264-codec/src/slicedec.rs:730` | - |  |
| `impl SliceDecoder<'a>::mc_rect` | 5 | `crates/video-h264-codec/src/slicedec.rs:763` | 2d | Motion-compensated prediction of a rectangle (4x4 units) written into the picture. |
| `impl SliceDecoder<'a>::uniform_motion` | 1 | `crates/video-h264-codec/src/slicedec.rs:815` | - | All 16 blocks of the current MB share references and motion vectors. |
| `impl SliceDecoder<'a>::predict_inter` | 1 | `crates/video-h264-codec/src/slicedec.rs:824` | - | Inter prediction for the whole current MB, using the stored motion. |
| `impl SliceDecoder<'a>::intra_mode_nb` | 4 | `crates/video-h264-codec/src/slicedec.rs:869` | - |  |
| `impl SliceDecoder<'a>::derive_intra_modes` | 2 | `crates/video-h264-codec/src/slicedec.rs:883` | - | Resolve Intra4x4PredMode / Intra8x8PredMode for the current MB from `self.cur.rem_mode`. |
| `impl SliceDecoder<'a>::add_luma_block4` | 2 | `crates/video-h264-codec/src/slicedec.rs:920` | - |  |
| `impl SliceDecoder<'a>::add_luma_block8` | 2 | `crates/video-h264-codec/src/slicedec.rs:934` | - |  |
| `impl SliceDecoder<'a>::add_chroma` | 1 | `crates/video-h264-codec/src/slicedec.rs:948` | - |  |
| `impl SliceDecoder<'a>::luma_avail4` | 3 | `crates/video-h264-codec/src/slicedec.rs:971` | - |  |
| `impl SliceDecoder<'a>::reconstruct_intra` | 1 | `crates/video-h264-codec/src/slicedec.rs:995` | - | Reconstruct an intra MB (prediction + residual) after its syntax has been parsed. |
| `impl SliceDecoder<'a>::reconstruct_inter_residual` | 1 | `crates/video-h264-codec/src/slicedec.rs:1045` | - | Add the residual of an inter MB (prediction already written). |
| `impl SliceDecoder<'a>::write_pcm` | 2 | `crates/video-h264-codec/src/slicedec.rs:1060` | - | Write I_PCM samples. |
| `impl SliceDecoder<'a>::finish_pcm` | 1 | `crates/video-h264-codec/src/slicedec.rs:1078` | - | Common state for an I_PCM MB. |
| `impl SliceDecoder<'a>::apply_qp_delta` | 2 | `crates/video-h264-codec/src/slicedec.rs:1093` | - | Update QP after mb_qp_delta. |
| `impl SliceDecoder<'a>::store_qp` | 1 | `crates/video-h264-codec/src/slicedec.rs:1099` | - |  |
| `impl SliceDecoder<'a>::luma_list4` | 1 | `crates/video-h264-codec/src/slicedec.rs:1111` | - |  |
| `impl SliceDecoder<'a>::put_luma4` | 4 | `crates/video-h264-codec/src/slicedec.rs:1116` | - | Store a 4x4 luma block given in scan order (levels[0..16]); `ac_only` skips position 0. |
| `impl SliceDecoder<'a>::put_luma8` | 3 | `crates/video-h264-codec/src/slicedec.rs:1135` | - | Store an 8x8 luma block given in scan order. |
| `impl SliceDecoder<'a>::put_luma_dc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1157` | - | Intra16x16 DC levels in scan order -> scaled DC of each 4x4 block. |
| `impl SliceDecoder<'a>::put_chroma_dc` | 3 | `crates/video-h264-codec/src/slicedec.rs:1174` | - | Chroma DC levels (4:2:0: 4 values in raster order) for component `c`. |
| `impl SliceDecoder<'a>::put_chroma_ac` | 4 | `crates/video-h264-codec/src/slicedec.rs:1189` | - | Chroma AC levels (scan positions 1..16 in levels[1..]) for block `b` of component `c`. |
| `impl SliceDecoder<'a>::decode_skip` | 1 | `crates/video-h264-codec/src/slicedec.rs:1212` | - | Decode a P_Skip or B_Skip macroblock at the current address. |
| `impl SliceDecoder<'a>::read_te` | 2 | `crates/video-h264-codec/src/slicedec.rs:1229` | - |  |
| `impl SliceDecoder<'a>::decode_mb_cavlc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1234` | - | Parse and reconstruct one macroblock_layer() with CAVLC. |
| `impl SliceDecoder<'a>::no_sub_8x8_lt` | 1 | `crates/video-h264-codec/src/slicedec.rs:1380` | - | Whether transform_size_8x8_flag may be present for an inter MB (NoSubMbPartSizeLessThan8x8 etc.). |
| `impl SliceDecoder<'a>::residual_cavlc` | 3 | `crates/video-h264-codec/src/slicedec.rs:1391` | - |  |
| `impl SliceDecoder<'a>::decode_cavlc` | 2 | `crates/video-h264-codec/src/slicedec.rs:1456` | - | Decode slice_data() with CAVLC. `r` is positioned after the slice header. |
| `nal` | 3 | `crates/video-h264-codec/src/synth_tests.rs:15` | - |  |
| `sps` | 4 | `crates/video-h264-codec/src/synth_tests.rs:30` | - |  |
| `pps` | 2 | `crates/video-h264-codec/src/synth_tests.rs:64` | - |  |
| `impl Pic::is_p` | 1 | `crates/video-h264-codec/src/synth_tests.rs:109` | - |  |
| `impl Pic::inter` | 1 | `crates/video-h264-codec/src/synth_tests.rs:112` | - |  |
| `slice_header` | 3 | `crates/video-h264-codec/src/synth_tests.rs:117` | - |  |
| `impl Enc::new` | 2 | `crates/video-h264-codec/src/synth_tests.rs:212` | - |  |
| `impl Enc::init` | 1 | `crates/video-h264-codec/src/synth_tests.rs:216` | - |  |
| `impl Enc::put` | 3 | `crates/video-h264-codec/src/synth_tests.rs:222` | - |  |
| `impl Enc::renorm` | 2 | `crates/video-h264-codec/src/synth_tests.rs:233` | - |  |
| `impl Enc::encode` | 4 | `crates/video-h264-codec/src/synth_tests.rs:248` | - |  |
| `impl Enc::terminate` | 3 | `crates/video-h264-codec/src/synth_tests.rs:261` | - |  |
| `write_pcm` | 2 | `crates/video-h264-codec/src/synth_tests.rs:276` | - |  |
| `slice_nal` | 2 | `crates/video-h264-codec/src/synth_tests.rs:281` | - |  |
| `random_mb` | 1 | `crates/video-h264-codec/src/synth_tests.rs:343` | - |  |
| `render` | 2 | `crates/video-h264-codec/src/synth_tests.rs:353` | - | Expected cropped planes of a picture made of PCM blocks (`None` = copy from `reference`). |
| `build_and_check` | 2 | `crates/video-h264-codec/src/synth_tests.rs:374` | - |  |
| `cross_check_ffmpeg` | 3 | `crates/video-h264-codec/src/synth_tests.rs:439` | - | Validate the hand-built stream and its expected output with ffmpeg when available. |
| `pcm_long_term_mmco_gaps_cavlc` | 0 | `crates/video-h264-codec/src/synth_tests.rs:462` | - |  |
| `pcm_long_term_mmco_gaps_cabac` | 0 | `crates/video-h264-codec/src/synth_tests.rs:468` | - |  |
| `weighted_check` | 2 | `crates/video-h264-codec/src/synth_tests.rs:473` | - |  |
| `explicit_weighted_prediction_p_and_b_skip` | 0 | `crates/video-h264-codec/src/synth_tests.rs:523` | - |  |
| `poc_type1_reordering` | 0 | `crates/video-h264-codec/src/synth_tests.rs:532` | - | POC type 1: non-reference pictures are displayed before the preceding reference picture. |
| `zigzag8` | 0 | `crates/video-h264-codec/src/tables.rs:9` | - |  |
| `norm_adjust4` | 2 | `crates/video-h264-codec/src/tables.rs:73` | - | normAdjust4x4(m, i, j) for raster position `pos` (i = row, j = col). |
| `norm_adjust8` | 2 | `crates/video-h264-codec/src/tables.rs:85` | - | normAdjust8x8(m, i, j) for raster position `pos` (i = row, j = col). |
| `zigzag8_known_prefix` | 0 | `crates/video-h264-codec/src/tables.rs:151` | - |  |
| `blk4_maps_consistent` | 0 | `crates/video-h264-codec/src/tables.rs:162` | - |  |
| `qpc` | 0 | `crates/video-h264-codec/src/tables.rs:170` | - |  |
| `impl LevelScale::new` | 1 | `crates/video-h264-codec/src/transform.rs:16` | - |  |
| `scale4` | 3 | `crates/video-h264-codec/src/transform.rs:34` | - | Scale one 4x4 AC coefficient (8.5.12.1, not the DC of Intra16x16 / chroma blocks). |
| `scale8` | 3 | `crates/video-h264-codec/src/transform.rs:40` | - | Scale one 8x8 coefficient (8.5.13.1). |
| `luma_dc_dequant` | 3 | `crates/video-h264-codec/src/transform.rs:46` | - | Intra16x16 luma DC: inverse Hadamard + scaling (8.5.10). `c` is raster order (row*4+col); |
| `chroma_dc_dequant_420` | 3 | `crates/video-h264-codec/src/transform.rs:75` | - | 4:2:0 chroma DC: 2x2 transform + scaling (8.5.11). `c` = [c00, c01, c10, c11]. |
| `clip_u8` | 1 | `crates/video-h264-codec/src/transform.rs:87` | - |  |
| `idct4_add` | 3 | `crates/video-h264-codec/src/transform.rs:95` | - | Inverse 4x4 transform of raster-order `d` and add to `dst` (8.5.12.2, 8.5.14). |
| `idct4_dc_add` | 3 | `crates/video-h264-codec/src/transform.rs:128` | - | DC-only shortcut: all AC coefficients zero. |
| `idct8_1d` | 1 | `crates/video-h264-codec/src/transform.rs:138` | - |  |
| `idct8_add` | 3 | `crates/video-h264-codec/src/transform.rs:160` | - | Inverse 8x8 transform of raster-order `d` and add to `dst` (8.5.13.2): rows, then columns, |
| `reference_add` | 4 | `crates/video-h264-codec/src/transform.rs:188` | - | The straightforward per-column formulation (8.5.12.2 / 8.5.13.2) the vectorised |
| `vectorised_transforms_match_reference` | 0 | `crates/video-h264-codec/src/transform.rs:208` | - |  |
| `idct4_dc_only_matches_shortcut` | 0 | `crates/video-h264-codec/src/transform.rs:233` | - |  |
| `idct4_hand_example` | 0 | `crates/video-h264-codec/src/transform.rs:245` | - |  |
| `idct8_dc` | 0 | `crates/video-h264-codec/src/transform.rs:259` | - |  |
| `luma_dc_flat` | 0 | `crates/video-h264-codec/src/transform.rs:268` | - |  |
| `chroma_dc` | 0 | `crates/video-h264-codec/src/transform.rs:277` | - |  |
| `scale_functions` | 0 | `crates/video-h264-codec/src/transform.rs:285` | - |  |
| `huge_levels_dequantise_without_overflow_panics` | 0 | `crates/video-h264-codec/src/transform.rs:296` | - | Corrupt streams can carry huge coefficient levels: dequantisation wraps (as release |

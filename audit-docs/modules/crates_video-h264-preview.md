# `crates/video-h264-preview`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Bounded, silent, P/I/B-frame MP4/H.264 preview using the existing QualiaDB MP4
demux dependency and H.264/color operations. This is an adapter,
not a project/session, HMC format, media authority, or timeline clock.
Informed by open standards and Informatics Fiduciary guarantees under UDHR Articles 19 and 27._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Matrix::new` | 2 | `crates/video-h264-preview/src/convert.rs:15` | numerics | BT.601 (SD) or BT.709 (HD) coefficients. |
| `impl Matrix::for_code` | 3 | `crates/video-h264-preview/src/convert.rs:29` | numerics | Choose by ITU-T H.273 matrix code (1 = 709, 5/6 = 601), else by picture height. |
| `impl Matrix::rgb` | 4 | `crates/video-h264-preview/src/convert.rs:40` | semantics | Convert one 8-bit sample triple. |
| `planar_to_rgba` | 10 | `crates/video-h264-preview/src/convert.rs:57` | numerics | Planar 8-bit Y'CbCr with chroma subsampled by `cx` horizontally and `cy` vertically |
| `scale_rgba` | 5 | `crates/video-h264-preview/src/convert.rs:88` | - | Area-average downscale (or nearest upscale) of packed RGBA. |
| `limited_range_black_white_and_grey` | 0 | `crates/video-h264-preview/src/convert.rs:132` | - |  |
| `scaling_averages` | 0 | `crates/video-h264-preview/src/convert.rs:146` | - |  |
| `impl GapBudget::admit` | 6 | `crates/video-h264-preview/src/lib.rs:26` | - | Mirrors Decoder::start_picture and submit_pending, including a gap on |
| `impl Frame::new` | 4 | `crates/video-h264-preview/src/lib.rs:52` | - |  |
| `impl Frame::fit` | 3 | `crates/video-h264-preview/src/lib.rs:55` | - |  |
| `decode_mp4` | 4 | `crates/video-h264-preview/src/lib.rs:74` | economics | Decode a single traversal of the selected media range. Caller must supply a |
| `decode_mp4_stream` | 5 | `crates/video-h264-preview/src/lib.rs:86` | - | Decode to a caller-owned, backpressured sink without retaining the whole |
| `project_video_container` | 1 | `crates/video-h264-preview/src/lib.rs:105` | 3d | Select one video track without changing source bytes or moving media. For a |
| `children` | 3 | `crates/video-h264-preview/src/lib.rs:106` | - |  |
| `one_child` | 4 | `crates/video-h264-preview/src/lib.rs:114` | - |  |
| `admit_audio_edit` | 4 | `crates/video-h264-preview/src/lib.rs:174` | - |  |
| `admit_sample_table` | 2 | `crates/video-h264-preview/src/lib.rs:186` | - | Bound every table allocation before passing the input to the MP4 parser. |
| `check_top_level` | 1 | `crates/video-h264-preview/src/lib.rs:202` | - | Admission for the owning demuxer: reject fragmented/event/extended boxes |
| `walk` | 6 | `crates/video-h264-preview/src/lib.rs:204` | - |  |
| `decode_inner` | 6 | `crates/video-h264-preview/src/lib.rs:259` | - |  |
| `validate_offsets` | 3 | `crates/video-h264-preview/src/lib.rs:346` | - |  |
| `validate_sizes` | 5 | `crates/video-h264-preview/src/lib.rs:378` | - |  |
| `live` | 1 | `crates/video-h264-preview/src/lib.rs:391` | - |  |
| `check_sample` | 3 | `crates/video-h264-preview/src/lib.rs:396` | - | Reject in-band SPS/PPS changes, B/SP/SI slices and mixed IDR/non-IDR |
| `validate_picture_headers` | 4 | `crates/video-h264-preview/src/lib.rs:431` | - | Match the codec's own primary-picture boundary test before it can create |
| `rgba709` | 2 | `crates/video-h264-preview/src/lib.rs:451` | - |  |
| `actual_gaps_are_budgeted_across_wrap_nonreference_and_mmco5` | 0 | `crates/video-h264-preview/src/lib.rs:471` | - |  |
| `nal` | 2 | `crates/video-h264-preview/src/lib.rs:484` | - |  |
| `sps` | 0 | `crates/video-h264-preview/src/lib.rs:485` | - |  |
| `sps_for_width` | 1 | `crates/video-h264-preview/src/lib.rs:486` | - |  |
| `sps_for_params` | 2 | `crates/video-h264-preview/src/lib.rs:487` | - |  |
| `pps` | 0 | `crates/video-h264-preview/src/lib.rs:498` | - |  |
| `idr` | 1 | `crates/video-h264-preview/src/lib.rs:503` | - |  |
| `metadata_names_in_pixel_payload_do_not_reject_valid_video` | 0 | `crates/video-h264-preview/src/lib.rs:509` | 2d |  |
| `idr_slice` | 3 | `crates/video-h264-preview/src/lib.rs:525` | - |  |
| `idr_slice_bits` | 4 | `crates/video-h264-preview/src/lib.rs:526` | - |  |
| `wider_frame_number_sps_decodes_within_gap_budget` | 0 | `crates/video-h264-preview/src/lib.rs:532` | - |  |
| `two_slice_picture_decodes_and_changed_picture_identity_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:540` | - |  |
| `p_skip` | 0 | `crates/video-h264-preview/src/lib.rs:559` | - |  |
| `reordered_sps` | 0 | `crates/video-h264-preview/src/lib.rs:565` | - |  |
| `reordered_pcm` | 4 | `crates/video-h264-preview/src/lib.rs:576` | - |  |
| `reordered_b_skip` | 0 | `crates/video-h264-preview/src/lib.rs:583` | - |  |
| `synthetic_reordered` | 0 | `crates/video-h264-preview/src/lib.rs:589` | - |  |
| `b_frame_follows_composition_order_and_selection` | 0 | `crates/video-h264-preview/src/lib.rs:599` | - |  |
| `inconsistent_composition_table_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:606` | - |  |
| `synthetic_mp4` | 0 | `crates/video-h264-preview/src/lib.rs:613` | - |  |
| `synthetic_av_mp4` | 0 | `crates/video-h264-preview/src/lib.rs:620` | - |  |
| `add_synthetic_audio_edit` | 1 | `crates/video-h264-preview/src/lib.rs:630` | - |  |
| `boxes` | 3 | `crates/video-h264-preview/src/lib.rs:633` | - |  |
| `combined_audio_edit_preserves_video_timeline` | 0 | `crates/video-h264-preview/src/lib.rs:648` | - |  |
| `video_projects_combined_av_without_changing_source_or_offsets` | 0 | `crates/video-h264-preview/src/lib.rs:662` | - |  |
| `video_requires_local_data_reference_and_entry_binding` | 0 | `crates/video-h264-preview/src/lib.rs:674` | - |  |
| `stream_delivers_more_than_32_frames_and_sink_can_stop` | 0 | `crates/video-h264-preview/src/lib.rs:680` | - |  |
| `synthetic_gop` | 0 | `crates/video-h264-preview/src/lib.rs:690` | - |  |
| `sample_tables_reject_hostile_counts_before_mp4_parser` | 0 | `crates/video-h264-preview/src/lib.rs:698` | - |  |
| `malformed_final_slice_is_not_presented` | 0 | `crates/video-h264-preview/src/lib.rs:713` | - |  |
| `actual_idr_samples_decode_to_different_srgb_frames` | 0 | `crates/video-h264-preview/src/lib.rs:728` | - |  |
| `p_slice_uses_prior_reference_when_range_starts_after_idr` | 0 | `crates/video-h264-preview/src/lib.rs:733` | - |  |
| `selected_p_frame_without_prior_idr_is_refused` | 0 | `crates/video-h264-preview/src/lib.rs:740` | - |  |
| `malformed_nals_refuse_parameter_sets_and_sp_slices` | 0 | `crates/video-h264-preview/src/lib.rs:750` | - |  |
| `declared_sample_sizes_cannot_trigger_unbounded_reads` | 0 | `crates/video-h264-preview/src/lib.rs:764` | - |  |
| `hostile_nested_box_and_sample_size_are_refused_before_decode` | 0 | `crates/video-h264-preview/src/lib.rs:770` | - |  |
| `container_and_cancellation_refuse_safely` | 0 | `crates/video-h264-preview/src/lib.rs:781` | - |  |
| `movie_inspection_and_random_access_h264` | 0 | `crates/video-h264-preview/src/lib.rs:786` | - |  |
| `movie_prores_playback` | 0 | `crates/video-h264-preview/src/lib.rs:804` | - |  |
| `fit_size` | 4 | `crates/video-h264-preview/src/movie.rs:33` | - | The size that fits `w` x `h` inside `max_w` x `max_h`, keeping the aspect ratio (>= 1 px). |
| `impl MovieInfo::display_aspect` | 1 | `crates/video-h264-preview/src/movie.rs:63` | - |  |
| `impl Lru<K>::new` | 1 | `crates/video-h264-preview/src/movie.rs:92` | - |  |
| `impl Lru<K>::get` | 2 | `crates/video-h264-preview/src/movie.rs:98` | - |  |
| `impl Lru<K>::put` | 3 | `crates/video-h264-preview/src/movie.rs:105` | - |  |
| `impl Lru<K>::contains` | 2 | `crates/video-h264-preview/src/movie.rs:112` | - |  |
| `impl std::fmt::Debug for Movie::fmt` | 2 | `crates/video-h264-preview/src/movie.rs:134` | - |  |
| `impl Movie::open` | 1 | `crates/video-h264-preview/src/movie.rs:141` | - | Open a movie held in memory. |
| `impl Movie::open_source` | 1 | `crates/video-h264-preview/src/movie.rs:147` | - | Open a movie from any random-access byte source. |
| `impl Movie::info` | 1 | `crates/video-h264-preview/src/movie.rs:305` | - |  |
| `impl Movie::set_cache_frames` | 2 | `crates/video-h264-preview/src/movie.rs:309` | - |  |
| `impl Movie::frame_index_at` | 2 | `crates/video-h264-preview/src/movie.rs:313` | - |  |
| `impl Movie::frame_time` | 2 | `crates/video-h264-preview/src/movie.rs:321` | - |  |
| `impl Movie::frame_at` | 2 | `crates/video-h264-preview/src/movie.rs:325` | - |  |
| `impl Movie::is_cached` | 2 | `crates/video-h264-preview/src/movie.rs:330` | - |  |
| `impl Movie::frame` | 2 | `crates/video-h264-preview/src/movie.rs:334` | - |  |
| `impl Movie::thumbnail_at` | 3 | `crates/video-h264-preview/src/movie.rs:353` | - |  |
| `impl Movie::thumbnails` | 3 | `crates/video-h264-preview/src/movie.rs:391` | - |  |
| `impl Movie::read` | 2 | `crates/video-h264-preview/src/movie.rs:402` | - |  |
| `impl Movie::decode_intra` | 3 | `crates/video-h264-preview/src/movie.rs:413` | - |  |
| `impl Movie::h264_to_frame` | 3 | `crates/video-h264-preview/src/movie.rs:446` | - |  |
| `impl Movie::decode_h264_single` | 4 | `crates/video-h264-preview/src/movie.rs:471` | - |  |
| `impl Movie::decode_h264` | 2 | `crates/video-h264-preview/src/movie.rs:488` | - |  |
| `err` | 1 | `crates/video-h264-preview/src/synth.rs:16` | - |  |
| `nal_body` | 3 | `crates/video-h264-preview/src/synth.rs:20` | - |  |
| `sps` | 2 | `crates/video-h264-preview/src/synth.rs:26` | - |  |
| `pps` | 0 | `crates/video-h264-preview/src/synth.rs:46` | - |  |
| `pcm_picture` | 5 | `crates/video-h264-preview/src/synth.rs:67` | - |  |
| `length_prefixed` | 1 | `crates/video-h264-preview/src/synth.rs:105` | - |  |
| `frame_color` | 1 | `crates/video-h264-preview/src/synth.rs:113` | - | Limited-range Y'CbCr of frame `i` for synthetic test streams. |
| `h264_mp4_with` | 6 | `crates/video-h264-preview/src/synth.rs:122` | - | Build an H.264 MP4 with customized macroblock dimensions, frame count, and color generator. |
| `h264_mp4` | 2 | `crates/video-h264-preview/src/synth.rs:159` | - | 64x48 H.264 MP4 of `n` solid frames at 24 fps, with an IDR picture every `gop` frames. |
| `test_pattern_mp4` | 1 | `crates/video-h264-preview/src/synth.rs:164` | - | Color-bar test movie for demos, tests, and preview verification (320x192 at 24 fps). |
| `prores_mov` | 1 | `crates/video-h264-preview/src/synth.rs:187` | - | QuickTime MOV containing `n` frames of ProRes 422 HQ (64x48) at 25 fps. |

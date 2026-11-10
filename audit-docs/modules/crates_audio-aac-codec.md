# `crates/audio-aac-codec`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_AAC-LC encoder.

Pipeline per 1024-sample frame and channel element:
1. transient detection on high-passed 128-sample sub-blocks (one frame of look-ahead) →
window sequence (`ONLY_LONG` / `LONG_START` / `EIGHT_SHORT` / `LONG_STOP`) and short-window grouping;
2. windowed MDCT (sine or KBD);
3. psychoacoustic model → per-band masking thresholds and perceptual entropy;
4. TNS analysis (long windows; Levinson–Durbin, applied when the prediction gain is high);
5. per-band M/S decision for channel pairs (common window);
6. rate–distortion loop: a single multiplier λ scales every band's allowed noise; each band gets the
coarsest scalefactor meeting its noise target, sections/codebooks are chosen by dynamic
programming, and λ is bisected until the frame fits its bit budget (CBR) or fixed (VBR);
7. bit reservoir (CBR): budget follows the frame's PE, bounded by the decoder buffer
(6144 bits/channel); overflow is padded with fill elements so the rate stays constant._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl ElementType::channels` | 1 | `crates/audio-aac-codec/src/config.rs:21` | - |  |
| `impl ProgramConfig::parse` | 1 | `crates/audio-aac-codec/src/config.rs:44` | - |  |
| `impl ProgramConfig::write` | 2 | `crates/audio-aac-codec/src/config.rs:90` | - |  |
| `impl ProgramConfig::layout` | 1 | `crates/audio-aac-codec/src/config.rs:125` | - | Elements in output channel order (front, side, back, LFE). |
| `config_layout` | 1 | `crates/audio-aac-codec/src/config.rs:140` | - | Element layout of `channelConfiguration` 1..=7 (tags as conventionally assigned). |
| `read_aot` | 1 | `crates/audio-aac-codec/src/config.rs:173` | - |  |
| `read_rate` | 1 | `crates/audio-aac-codec/src/config.rs:178` | - |  |
| `impl AudioSpecificConfig::lc` | 2 | `crates/audio-aac-codec/src/config.rs:190` | - | A plain AAC-LC configuration. |
| `impl AudioSpecificConfig::parse` | 1 | `crates/audio-aac-codec/src/config.rs:204` | - |  |
| `impl AudioSpecificConfig::to_bytes` | 1 | `crates/audio-aac-codec/src/config.rs:244` | - | Serialise (AAC-LC, no SBR signalling). |
| `impl AudioSpecificConfig::layout` | 1 | `crates/audio-aac-codec/src/config.rs:265` | - | Elements in output channel order. |
| `impl AudioSpecificConfig::channels` | 1 | `crates/audio-aac-codec/src/config.rs:272` | - |  |
| `impl AdtsHeader::parse` | 1 | `crates/audio-aac-codec/src/config.rs:293` | - |  |
| `impl AdtsHeader::to_asc` | 1 | `crates/audio-aac-codec/src/config.rs:323` | - | The equivalent `AudioSpecificConfig`. Channel configuration 0 (PCE in-band) is resolved by the |
| `impl AdtsHeader::write` | 5 | `crates/audio-aac-codec/src/config.rs:328` | - | Write a 7-byte header (no CRC) for a frame whose raw payload is `payload_len` bytes. |
| `split_adts` | 1 | `crates/audio-aac-codec/src/config.rs:352` | - | Split an ADTS stream into `(header, raw payload)` frames. Stops at the first malformed or |
| `asc_roundtrip` | 0 | `crates/audio-aac-codec/src/config.rs:381` | - |  |
| `asc_with_pce` | 0 | `crates/audio-aac-codec/src/config.rs:392` | - |  |
| `he_aac_asc_parses_core` | 0 | `crates/audio-aac-codec/src/config.rs:409` | - |  |
| `adts_roundtrip` | 0 | `crates/audio-aac-codec/src/config.rs:425` | - |  |
| `impl Decoder::new` | 1 | `crates/audio-aac-codec/src/decoder.rs:58` | - | Create a decoder from an `AudioSpecificConfig`. |
| `impl Decoder::from_adts` | 1 | `crates/audio-aac-codec/src/decoder.rs:64` | - | Create a decoder for an ADTS stream from its first header. With channel configuration 0 the |
| `impl Decoder::from_config` | 1 | `crates/audio-aac-codec/src/decoder.rs:68` | - |  |
| `impl Decoder::set_layout` | 2 | `crates/audio-aac-codec/src/decoder.rs:81` | - |  |
| `impl Decoder::config` | 1 | `crates/audio-aac-codec/src/decoder.rs:93` | - |  |
| `impl Decoder::sample_rate` | 1 | `crates/audio-aac-codec/src/decoder.rs:96` | - |  |
| `impl Decoder::sbr` | 1 | `crates/audio-aac-codec/src/decoder.rs:102` | - | Whether the stream is HE-AAC: SBR signalled in the `AudioSpecificConfig` (explicit) or SBR |
| `impl Decoder::channels` | 1 | `crates/audio-aac-codec/src/decoder.rs:105` | - |  |
| `impl Decoder::layout` | 1 | `crates/audio-aac-codec/src/decoder.rs:109` | - | Element layout in output channel order (C, L/R, surround pairs, LFE for configurations 1–7). |
| `impl Decoder::reset` | 1 | `crates/audio-aac-codec/src/decoder.rs:114` | - | Clear the overlap state (e.g. after a seek). |
| `impl Decoder::sf_index` | 1 | `crates/audio-aac-codec/src/decoder.rs:121` | - |  |
| `impl Decoder::decode` | 2 | `crates/audio-aac-codec/src/decoder.rs:126` | - | Decode one access unit (one `raw_data_block`) into 1024 samples per channel. |
| `impl Decoder::decode_block` | 3 | `crates/audio-aac-codec/src/decoder.rs:130` | - |  |
| `impl Decoder::find_slot` | 4 | `crates/audio-aac-codec/src/decoder.rs:211` | - |  |
| `impl Decoder::decode_cpe` | 3 | `crates/audio-aac-codec/src/decoder.rs:226` | - |  |
| `impl Decoder::decode_ics` | 3 | `crates/audio-aac-codec/src/decoder.rs:286` | - |  |
| `impl Decoder::next_random` | 1 | `crates/audio-aac-codec/src/decoder.rs:444` | - |  |
| `impl Decoder::fill_noise` | 3 | `crates/audio-aac-codec/src/decoder.rs:451` | - | Perceptual noise substitution. `pair` = (left channel, ms mask) for the right channel of a CPE: |
| `impl Decoder::finish_channel` | 3 | `crates/audio-aac-codec/src/decoder.rs:486` | - | TNS + filterbank for a single channel element; returns output samples. |
| `synth` | 3 | `crates/audio-aac-codec/src/decoder.rs:495` | - | Inverse MDCT, windowing and overlap-add (§4.6.18), output scaled to ±1.0 full scale. |
| `sbr_fill_element_marks_implicit_he_aac` | 0 | `crates/audio-aac-codec/src/decoder.rs:568` | - | Implicitly signalled HE-AAC (an AAC-LC config) is recognised from the SBR fill element. |
| `impl EncoderConfig::cbr` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:60` | - |  |
| `impl EncoderConfig::vbr` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:72` | - |  |
| `impl EncoderStats::bitrate` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:94` | - | Average bitrate of the raw access units. |
| `impl Encoder::new` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:143` | - |  |
| `impl Encoder::config` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:236` | - |  |
| `impl Encoder::audio_specific_config` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:241` | - | `AudioSpecificConfig` bytes for the MP4 `esds` decoder-specific info. |
| `impl Encoder::priming_samples` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:247` | - | Encoder delay: the first 1024 decoded samples are priming and should be skipped (MP4 edit |
| `impl Encoder::bandwidth` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:252` | - | Audio bandwidth in Hz. |
| `impl Encoder::stats` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:256` | - |  |
| `impl Encoder::encode` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:262` | - | Feed planar samples (±1.0 full scale); returns any complete access units. Missing channels are |
| `impl Encoder::flush` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:283` | - | Encode the remaining input (zero-padded). Afterwards the decoded stream, minus |
| `impl Encoder::available` | 1 | `crates/audio-aac-codec/src/encoder/mod.rs:305` | - | Absolute sample index one past the buffered input. |
| `impl Encoder::needed_for` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:309` | - | Input needed (absolute end) before frame `t` can be encoded (window + transient look-ahead). |
| `impl Encoder::sample` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:313` | - |  |
| `impl Encoder::subblock_energy` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:318` | - |  |
| `impl Encoder::transient` | 4 | `crates/audio-aac-codec/src/encoder/mod.rs:332` | - | Transient in the region covered by frame `t`'s short windows. |
| `impl Encoder::max_sfb` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:349` | - |  |
| `impl Encoder::mdct` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:356` | - |  |
| `impl Encoder::tns_analyse` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:411` | - |  |
| `impl Encoder::analyse_element` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:457` | - |  |
| `impl Encoder::element_overhead` | 3 | `crates/audio-aac-codec/src/encoder/mod.rs:588` | - |  |
| `impl Encoder::quantise_all` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:602` | - |  |
| `impl Encoder::encode_frame` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:616` | - | `last`: final frame of the stream — a CBR stream spends (or pads away) the whole reservoir so |
| `write_fill` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:792` | - | Write fill elements totalling at least `bits - 7` bits (at most `bits + 8`); returns bits written. |
| `interp` | 2 | `crates/audio-aac-codec/src/encoder/mod.rs:820` | - |  |
| `bark` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:22` | - |  |
| `ath_db` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:28` | graph | Terhardt's threshold in quiet (dB SPL), capped at 40 dB so content near the band edge is not |
| `impl BandModel::new` | 3 | `crates/audio-aac-codec/src/encoder/psy.rs:34` | - |  |
| `impl BandModel::num_bands` | 1 | `crates/audio-aac-codec/src/encoder/psy.rs:77` | - |  |
| `impl Psy::new` | 2 | `crates/audio-aac-codec/src/encoder/psy.rs:98` | - |  |
| `impl Psy::analyse` | 3 | `crates/audio-aac-codec/src/encoder/psy.rs:103` | - | Analyse a spectrum: 1024 lines (long) or 8×128 (short). |
| `tone_is_masked_less_than_noise` | 0 | `crates/audio-aac-codec/src/encoder/psy.rs:157` | - |  |
| `impl ChannelInput::new` | 5 | `crates/audio-aac-codec/src/encoder/quant.rs:47` | - | `spec` is window-major (short windows at stride 128); `thr[g][sfb]` and the grouping come from |
| `gain34` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:85` | - |  |
| `gain` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:89` | - |  |
| `quantise` | 5 | `crates/audio-aac-codec/src/encoder/quant.rs:94` | - | Quantise a slice at `sf`, returning the distortion; writes the (signed) values to `out` if given. |
| `search_sf` | 3 | `crates/audio-aac-codec/src/encoder/quant.rs:119` | - | Largest scalefactor whose distortion stays within `allowed`. |
| `candidates` | 1 | `crates/audio-aac-codec/src/encoder/quant.rs:142` | - | Candidate codebooks for a band whose largest magnitude is `m` (0 for all-zero bands). |
| `quantise_channel` | 2 | `crates/audio-aac-codec/src/encoder/quant.rs:155` | - | Quantise one channel at rate-distortion multiplier `lambda` and select sections. |
| `write_ics` | 4 | `crates/audio-aac-codec/src/encoder/quant.rs:318` | - | Write an `individual_channel_stream`; `write_info` is false inside a common-window CPE. |
| `bit_count_matches_written_bits` | 0 | `crates/audio-aac-codec/src/encoder/quant.rs:369` | - |  |
| `book` | 1 | `crates/audio-aac-codec/src/huffman.rs:52` | - | Spectrum codebook 1..=11. |
| `impl Tree::build` | 1 | `crates/audio-aac-codec/src/huffman.rs:64` | - |  |
| `impl Tree::decode` | 2 | `crates/audio-aac-codec/src/huffman.rs:85` | - |  |
| `trees` | 0 | `crates/audio-aac-codec/src/huffman.rs:101` | - |  |
| `decode_sf` | 1 | `crates/audio-aac-codec/src/huffman.rs:108` | - | Decode one scalefactor difference (`-60..=60`). |
| `write_sf` | 2 | `crates/audio-aac-codec/src/huffman.rs:114` | - | Write one scalefactor difference (`-60..=60`). |
| `sf_bits` | 1 | `crates/audio-aac-codec/src/huffman.rs:121` | - | Bits of one scalefactor difference. |
| `decode_spectral` | 3 | `crates/audio-aac-codec/src/huffman.rs:126` | - | Decode one codeword of codebook `cb` (1..=11) into `out[..dim]`, including sign bits and escapes. |
| `index_of` | 2 | `crates/audio-aac-codec/src/huffman.rs:164` | - | Codebook index of the (sign-stripped for unsigned books) values. |
| `escape_bits` | 1 | `crates/audio-aac-codec/src/huffman.rs:174` | - |  |
| `count_bits` | 2 | `crates/audio-aac-codec/src/huffman.rs:182` | - | Bits needed to code `vals` (a multiple of the codebook dimension) with codebook `cb` (1..=11). |
| `write_spectral` | 3 | `crates/audio-aac-codec/src/huffman.rs:202` | - | Write `vals` with codebook `cb` (1..=11). |
| `check_complete` | 1 | `crates/audio-aac-codec/src/huffman.rs:235` | - |  |
| `codebooks_are_complete_prefix_codes` | 0 | `crates/audio-aac-codec/src/huffman.rs:253` | - |  |
| `spectral_roundtrip_every_book` | 0 | `crates/audio-aac-codec/src/huffman.rs:263` | - |  |
| `scalefactor_roundtrip` | 0 | `crates/audio-aac-codec/src/huffman.rs:295` | - |  |
| `impl WindowSequence::from_bits` | 1 | `crates/audio-aac-codec/src/ics.rs:20` | - |  |
| `impl Default for IcsInfo::default` | 0 | `crates/audio-aac-codec/src/ics.rs:42` | - |  |
| `impl IcsInfo::is_short` | 1 | `crates/audio-aac-codec/src/ics.rs:49` | - |  |
| `impl IcsInfo::num_windows` | 1 | `crates/audio-aac-codec/src/ics.rs:53` | - |  |
| `impl IcsInfo::swb_offsets` | 2 | `crates/audio-aac-codec/src/ics.rs:57` | - | Band offsets of one window. |
| `impl IcsInfo::num_swb` | 2 | `crates/audio-aac-codec/src/ics.rs:61` | - | Number of bands for this window length (`num_swb`). |
| `impl IcsInfo::group_start` | 2 | `crates/audio-aac-codec/src/ics.rs:65` | - | First window of group `g`. |
| `impl IcsInfo::parse` | 2 | `crates/audio-aac-codec/src/ics.rs:69` | - |  |
| `impl IcsInfo::write` | 2 | `crates/audio-aac-codec/src/ics.rs:101` | - |  |
| `grouping_roundtrip` | 0 | `crates/audio-aac-codec/src/ics.rs:130` | - |  |
| `impl From<bitstream_codec::BitError> for Error::from` | 1 | `crates/audio-aac-codec/src/lib.rs:69` | - |  |
| `impl Fft::new` | 1 | `crates/audio-aac-codec/src/mdct.rs:15` | numerics |  |
| `impl Fft::run` | 3 | `crates/audio-aac-codec/src/mdct.rs:28` | - |  |
| `impl Mdct::new` | 1 | `crates/audio-aac-codec/src/mdct.rs:69` | - |  |
| `impl Mdct::dct4` | 3 | `crates/audio-aac-codec/src/mdct.rs:87` | - | DCT-IV: `out[k] = Σ u[n] cos(π/M (n+½)(k+½))`. |
| `impl Mdct::forward` | 3 | `crates/audio-aac-codec/src/mdct.rs:109` | - | Forward MDCT of `x` (length 2M, already windowed) into `out` (length M). |
| `impl Mdct::inverse` | 3 | `crates/audio-aac-codec/src/mdct.rs:122` | - | Inverse MDCT (unscaled): `y[n] = Σ X[k] cos(π/M (n + ½ + M/2)(k + ½))`, `y` has length 2M. |
| `mdct_long` | 0 | `crates/audio-aac-codec/src/mdct.rs:137` | - | Cached transforms for 1024 and 128 lines. |
| `mdct_short` | 0 | `crates/audio-aac-codec/src/mdct.rs:141` | - |  |
| `impl WindowShape::from_bit` | 1 | `crates/audio-aac-codec/src/mdct.rs:155` | - |  |
| `impl WindowShape::bit` | 1 | `crates/audio-aac-codec/src/mdct.rs:158` | - |  |
| `sine_half` | 1 | `crates/audio-aac-codec/src/mdct.rs:163` | - |  |
| `bessel_i0` | 1 | `crates/audio-aac-codec/src/mdct.rs:168` | - |  |
| `kbd_half` | 2 | `crates/audio-aac-codec/src/mdct.rs:182` | - |  |
| `window` | 2 | `crates/audio-aac-codec/src/mdct.rs:201` | - | Rising half (length N/2) of the window of the given shape; `long` selects N = 2048, else N = 256. |
| `direct` | 1 | `crates/audio-aac-codec/src/mdct.rs:217` | - |  |
| `forward_matches_definition` | 0 | `crates/audio-aac-codec/src/mdct.rs:223` | - |  |
| `inverse_matches_definition` | 0 | `crates/audio-aac-codec/src/mdct.rs:236` | - |  |
| `tdac_perfect_reconstruction` | 0 | `crates/audio-aac-codec/src/mdct.rs:248` | - |  |
| `sample_rate_index` | 1 | `crates/audio-aac-codec/src/tables.rs:7` | - | Map an arbitrary sample rate to its `samplingFrequencyIndex`, if it is one of the standard rates. |
| `table_index_for_rate` | 1 | `crates/audio-aac-codec/src/tables.rs:13` | - | The table index used for scalefactor-band tables when a non-standard rate is signalled explicitly |
| `swb_offsets_long` | 1 | `crates/audio-aac-codec/src/tables.rs:66` | - | Scalefactor band offsets (`swb_offset_long_window`, 1024-line frames); `len() == num_swb + 1`. |
| `swb_offsets_short` | 1 | `crates/audio-aac-codec/src/tables.rs:79` | - | Scalefactor band offsets for one short window (128 lines). |
| `tns_max_bands` | 2 | `crates/audio-aac-codec/src/tables.rs:90` | - | `TNS_MAX_BANDS` for AAC-LC (long, short). |
| `tns_max_order` | 1 | `crates/audio-aac-codec/src/tables.rs:98` | - | `TNS_MAX_ORDER` for AAC-LC. |
| `pow43` | 1 | `crates/audio-aac-codec/src/tables.rs:103` | - | `x^(4/3)` for the integer quantised magnitudes 0..=8191. |
| `sf_gain` | 1 | `crates/audio-aac-codec/src/tables.rs:115` | - | `2^(0.25 * (sf - 100))`, the dequantisation gain of scalefactor `sf`. |
| `band_tables_are_monotonic_and_complete` | 0 | `crates/audio-aac-codec/src/tables.rs:124` | - |  |
| `impl TnsData::parse` | 2 | `crates/audio-aac-codec/src/tns.rs:38` | - |  |
| `impl TnsData::write` | 3 | `crates/audio-aac-codec/src/tns.rs:70` | - |  |
| `impl TnsData::bit_count` | 2 | `crates/audio-aac-codec/src/tns.rs:96` | - | Bits `write` would produce. |
| `dequant_coef` | 2 | `crates/audio-aac-codec/src/tns.rs:119` | - | Dequantise one reflection coefficient. |
| `quant_coef` | 2 | `crates/audio-aac-codec/src/tns.rs:128` | - | Quantise a reflection coefficient (inverse of [`dequant_coef`], nearest index). |
| `parcor_to_lpc` | 1 | `crates/audio-aac-codec/src/tns.rs:140` | - | Reflection coefficients → direct-form LPC `a[0..=order]` (`a[0] = 1`). |
| `regions` | 3 | `crates/audio-aac-codec/src/tns.rs:155` | - | Filter region `[start, end)` of every TNS filter of window `w`, in spectrum lines of that window. |
| `lpc_of` | 2 | `crates/audio-aac-codec/src/tns.rs:178` | - |  |
| `apply_decoder` | 4 | `crates/audio-aac-codec/src/tns.rs:184` | - | Decoder: apply the all-pole TNS synthesis filters to `spec` (1024 lines; short windows at 128 stride). |
| `apply_encoder` | 4 | `crates/audio-aac-codec/src/tns.rs:214` | - | Encoder: apply the all-zero TNS analysis filters (exact inverse of [`apply_decoder`]). |
| `levinson` | 2 | `crates/audio-aac-codec/src/tns.rs:246` | - | Levinson–Durbin on autocorrelation `r[0..=order]`: returns reflection coefficients and the |
| `coef_quant_roundtrip` | 0 | `crates/audio-aac-codec/src/tns.rs:282` | - |  |
| `encoder_decoder_filters_are_inverse` | 0 | `crates/audio-aac-codec/src/tns.rs:293` | - |  |
| `ffmpeg` | 0 | `crates/audio-aac-codec/tests/common/mod.rs:9` | - | ffmpeg (see `media_test_support::oracle`); `None` after printing `SKIPPED`. |
| `fixtures` | 0 | `crates/audio-aac-codec/tests/common/mod.rs:13` | - |  |
| `impl Rng::next` | 1 | `crates/audio-aac-codec/tests/common/mod.rs:19` | - |  |
| `sweep` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:30` | - | Logarithmic sine sweep 40 Hz → 16 kHz. |
| `noise` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:43` | - |  |
| `speech` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:50` | - | Speech-like: glottal pulse train (gliding pitch) through three resonant formant filters with a |
| `chords` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:87` | - | Music-like: a sequence of decaying harmonic chords. |
| `clicks` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:109` | - | Clicks: sharp impulses every 250 ms over a quiet low tone (transient test). |
| `encode` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:131` | - |  |
| `ffmpeg_decode` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:155` | - | Decode with ffmpeg → planar f32 (ffmpeg's own channel order), plus stderr. |
| `ffmpeg_encode` | 5 | `crates/audio-aac-codec/tests/common/mod.rs:170` | - | Encode with ffmpeg's AAC encoder to ADTS. |
| `our_decode` | 1 | `crates/audio-aac-codec/tests/common/mod.rs:202` | - | Decode an ADTS file with our decoder. |
| `snr` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:217` | - | SNR in dB of `dec` against `refr`. |
| `seg_snr` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:228` | - | Segmental SNR (1024-sample segments above -50 dBFS, each clamped to [-10, 60] dB). |
| `max_abs_diff` | 2 | `crates/audio-aac-codec/tests/common/mod.rs:247` | - |  |
| `lowpass` | 3 | `crates/audio-aac-codec/tests/common/mod.rs:252` | - | Windowed-sinc (Blackman, 401 taps) low-pass at `fc` Hz, zero-phase (delay-compensated). |
| `input` | 2 | `crates/audio-aac-codec/tests/decoder_oracle.rs:15` | - |  |
| `compare` | 1 | `crates/audio-aac-codec/tests/decoder_oracle.rs:28` | - | Returns (max abs error, samples compared). |
| `matches_ffmpeg_decoder` | 0 | `crates/audio-aac-codec/tests/decoder_oracle.rs:52` | - |  |
| `pns_matches_ffmpeg` | 0 | `crates/audio-aac-codec/tests/decoder_oracle.rs:93` | - | PNS output is random by design, and ffmpeg's encoder does not emit it: build noise-substituted |
| `ics` | 3 | `crates/audio-aac-codec/tests/decoder_oracle.rs:97` | - |  |
| `mono` | 2 | `crates/audio-aac-codec/tests/encoder_oracle.rs:15` | - |  |
| `sig_sweep` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:19` | - |  |
| `sig_noise` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:22` | - |  |
| `sig_speech` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:25` | - |  |
| `sig_chords` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:28` | - |  |
| `sig_clicks` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:31` | - |  |
| `run` | 3 | `crates/audio-aac-codec/tests/encoder_oracle.rs:48` | - |  |
| `print` | 1 | `crates/audio-aac-codec/tests/encoder_oracle.rs:106` | - |  |
| `check` | 2 | `crates/audio-aac-codec/tests/encoder_oracle.rs:124` | - |  |
| `stereo_matrix_44k_48k` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:133` | numerics |  |
| `mono_and_surround` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:168` | - |  |
| `kbd_windows_no_tns_no_ms` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:190` | - |  |
| `sample_rates_8k_to_96k` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:206` | - |  |
| `channel_layouts_3_to_8` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:233` | - |  |
| `vbr_quality_scale` | 0 | `crates/audio-aac-codec/tests/encoder_oracle.rs:249` | - |  |
| `corpus` | 0 | `crates/audio-aac-codec/tests/robustness.rs:9` | - |  |
| `mutated_access_units_never_panic` | 0 | `crates/audio-aac-codec/tests/robustness.rs:24` | - |  |
| `random_configs_and_adts_never_panic` | 0 | `crates/audio-aac-codec/tests/robustness.rs:68` | - |  |
| `encoder_handles_hostile_input` | 0 | `crates/audio-aac-codec/tests/robustness.rs:96` | - |  |
| `roundtrip_and_delay` | 0 | `crates/audio-aac-codec/tests/robustness.rs:137` | - |  |

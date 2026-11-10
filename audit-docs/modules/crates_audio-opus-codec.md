# `crates/audio-opus-codec`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_CELT layer decoder (RFC 6716 §4.3)._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `lcg_rand` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:12` | - |  |
| `decode_pulses` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:17` | - | Decodes a PVQ codeword of `k` pulses in `n` dimensions; returns the sum of squares. |
| `exp_rotation1` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:63` | - |  |
| `exp_rotation` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:85` | - | Inverse spreading rotation (decoder direction). |
| `extract_collapse_mask` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:112` | - |  |
| `renormalise_vector` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:126` | - |  |
| `haar1` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:134` | - |  |
| `ordery` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:147` | - |  |
| `deinterleave_hadamard` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:151` | - |  |
| `interleave_hadamard` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:172` | - |  |
| `frac_mul16` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:194` | - |  |
| `bitexact_cos` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:198` | - |  |
| `bitexact_log2tan` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:205` | - |  |
| `isqrt32` | 1 | `crates/audio-opus-codec/src/celt/bands.rs:213` | - |  |
| `compute_qn` | 5 | `crates/audio-opus-codec/src/celt/bands.rs:224` | - |  |
| `sdiv` | 2 | `crates/audio-opus-codec/src/celt/bands.rs:243` | - | C-style truncating signed division. |
| `impl BandCtx<'_, '_, '_>::compute_theta` | 8 | `crates/audio-opus-codec/src/celt/bands.rs:273` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_band_n1` | 4 | `crates/audio-opus-codec/src/celt/bands.rs:346` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_partition` | 9 | `crates/audio-opus-codec/src/celt/bands.rs:366` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_band` | 10 | `crates/audio-opus-codec/src/celt/bands.rs:469` | - |  |
| `impl BandCtx<'_, '_, '_>::quant_band_stereo` | 10 | `crates/audio-opus-codec/src/celt/bands.rs:551` | - |  |
| `stereo_merge` | 3 | `crates/audio-opus-codec/src/celt/bands.rs:637` | - |  |
| `quant_all_bands` | 19 | `crates/audio-opus-codec/src/celt/bands.rs:664` | - | Decodes all band shapes (`quant_all_bands`, decoder side). |
| `anti_collapse` | 12 | `crates/audio-opus-codec/src/celt/bands.rs:825` | - | Anti-collapse noise injection (RFC 6716 §4.3.5). |
| `denormalise_bands` | 8 | `crates/audio-opus-codec/src/celt/bands.rs:879` | - | Multiplies the normalised shapes by the band energies (`denormalise_bands`). |
| `bitexact_trig_endpoints` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:912` | - |  |
| `isqrt_exact` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:921` | - |  |
| `pvq_roundtrip_all_codewords_small` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:929` | - |  |
| `haar_is_involution` | 0 | `crates/audio-opus-codec/src/celt/bands.rs:953` | - |  |
| `laplace_get_freq1` | 2 | `crates/audio-opus-codec/src/celt/energy.rs:11` | - |  |
| `laplace_decode` | 3 | `crates/audio-opus-codec/src/celt/energy.rs:17` | - | Decodes a Laplace-distributed integer (`fs` = probability of 0 in Q15, `decay` in Q14). |
| `unquant_coarse_energy` | 7 | `crates/audio-opus-codec/src/celt/energy.rs:48` | - | `unquant_coarse_energy`. |
| `unquant_fine_energy` | 6 | `crates/audio-opus-codec/src/celt/energy.rs:77` | - |  |
| `unquant_energy_finalise` | 8 | `crates/audio-opus-codec/src/celt/energy.rs:92` | - |  |
| `laplace_encode` | 4 | `crates/audio-opus-codec/src/celt/energy.rs:125` | - |  |
| `laplace_roundtrip` | 0 | `crates/audio-opus-codec/src/celt/energy.rs:157` | - |  |
| `impl Cpx::mul` | 2 | `crates/audio-opus-codec/src/celt/mdct.rs:15` | - |  |
| `impl Cpx::add` | 2 | `crates/audio-opus-codec/src/celt/mdct.rs:19` | - |  |
| `impl Fft::new` | 1 | `crates/audio-opus-codec/src/celt/mdct.rs:32` | numerics |  |
| `impl Fft::process` | 3 | `crates/audio-opus-codec/src/celt/mdct.rs:51` | - |  |
| `impl Fft::rec` | 7 | `crates/audio-opus-codec/src/celt/mdct.rs:57` | - |  |
| `impl Imdct::new` | 1 | `crates/audio-opus-codec/src/celt/mdct.rs:96` | - |  |
| `impl Imdct::backward` | 7 | `crates/audio-opus-codec/src/celt/mdct.rs:106` | - | Inverse transform of `n/2` coefficients read from `input` at `stride`. |
| `fft_matches_dft` | 0 | `crates/audio-opus-codec/src/celt/mdct.rs:161` | numerics |  |
| `imdct_matches_direct_formula` | 0 | `crates/audio-opus-codec/src/celt/mdct.rs:181` | - | The folded output must equal the direct IMDCT formula folded by TDAC symmetry. |
| `mode` | 0 | `crates/audio-opus-codec/src/celt/mod.rs:24` | - |  |
| `impl CeltDecoder::new` | 2 | `crates/audio-opus-codec/src/celt/mod.rs:64` | - | `sample_rate` is the output rate (48000, 24000, 16000, 12000 or 8000). |
| `impl CeltDecoder::reset` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:99` | - | `OPUS_RESET_STATE`. |
| `impl CeltDecoder::mem_len` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:117` | - |  |
| `impl CeltDecoder::decode` | 6 | `crates/audio-opus-codec/src/celt/mod.rs:124` | - | Decodes one CELT frame of `frame_size` samples (at the output rate) into interleaved `pcm` |
| `impl CeltDecoder::decode_frame` | 5 | `crates/audio-opus-codec/src/celt/mod.rs:162` | - |  |
| `impl CeltDecoder::synthesis` | 9 | `crates/audio-opus-codec/src/celt/mod.rs:384` | - | Denormalisation + IMDCT into the decode memory (`celt_synthesis`). |
| `impl CeltDecoder::deemphasis` | 4 | `crates/audio-opus-codec/src/celt/mod.rs:426` | - |  |
| `impl CeltDecoder::decode_lost` | 3 | `crates/audio-opus-codec/src/celt/mod.rs:457` | - | Packet-loss concealment: pitch-periodic extension for the first lost frames, then |
| `find_pitch` | 1 | `crates/audio-opus-codec/src/celt/mod.rs:540` | - |  |
| `tf_decode` | 6 | `crates/audio-opus-codec/src/celt/mod.rs:564` | - |  |
| `comb_filter` | 10 | `crates/audio-opus-codec/src/celt/mod.rs:593` | - | Pitch post-filter (in place on `buf[off .. off + n]`, reading history before `off`). |
| `get_pulses` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:15` | - | Number of pulses represented by pseudo-pulse index `i`. |
| `log2_frac` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:20` | - | Conservative (rounded up) log2 of `val` with `frac` fractional bits. |
| `pvq_v` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:47` | - | PVQ codebook size V(N, K) as u128 (saturating far above 2^32). |
| `fits_in32` | 2 | `crates/audio-opus-codec/src/celt/rate.rs:71` | - |  |
| `impl Mode::new` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:90` | - |  |
| `impl Mode::cache` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:114` | - |  |
| `impl Mode::bits2pulses` | 4 | `crates/audio-opus-codec/src/celt/rate.rs:119` | - |  |
| `impl Mode::pulses2bits` | 4 | `crates/audio-opus-codec/src/celt/rate.rs:136` | - |  |
| `impl Mode::init_caps` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:141` | - | Per-band allocation caps for this frame (`init_caps`). |
| `compute_pulse_cache` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:151` | - |  |
| `required_bits` | 3 | `crates/audio-opus-codec/src/celt/rate.rs:197` | - | Bits (1/2^frac units, rounded up) needed for each K in `0..=maxk` for dimension `n`. |
| `compute_caps` | 1 | `crates/audio-opus-codec/src/celt/rate.rs:212` | - |  |
| `compute_allocation` | 10 | `crates/audio-opus-codec/src/celt/rate.rs:285` | - | `clt_compute_allocation` (decoder side). |
| `interp_bits2pulses` | 15 | `crates/audio-opus-codec/src/celt/rate.rs:384` | - |  |
| `log2_frac_values` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:552` | - |  |
| `pvq_counts` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:562` | - |  |
| `caps_match_reference_shape` | 0 | `crates/audio-opus-codec/src/celt/rate.rs:573` | - |  |
| `transpose` | 1 | `crates/audio-opus-codec/src/celt/tables.rs:41` | - |  |
| `impl StreamDecoder::new` | 2 | `crates/audio-opus-codec/src/decoder.rs:34` | - | Creates a decoder producing `channels` (1 or 2) at `rate` (8/12/16/24/48 kHz). |
| `impl StreamDecoder::sample_rate` | 1 | `crates/audio-opus-codec/src/decoder.rs:58` | - |  |
| `impl StreamDecoder::channels` | 1 | `crates/audio-opus-codec/src/decoder.rs:62` | - |  |
| `impl StreamDecoder::final_range` | 1 | `crates/audio-opus-codec/src/decoder.rs:67` | - | Final range coder state of the last decoded frame (for conformance checks). |
| `impl StreamDecoder::last_packet_duration` | 1 | `crates/audio-opus-codec/src/decoder.rs:72` | - | Duration (samples per channel) of the last decoded packet. |
| `impl StreamDecoder::set_gain_q8` | 2 | `crates/audio-opus-codec/src/decoder.rs:77` | - | Sets the output gain in Q8 dB (as in the OpusHead header). |
| `impl StreamDecoder::set_phase_inversion_disabled` | 2 | `crates/audio-opus-codec/src/decoder.rs:82` | - | Disables the 180-degree intensity-stereo phase inversion (RFC 8251 §10). |
| `impl StreamDecoder::reset` | 1 | `crates/audio-opus-codec/src/decoder.rs:87` | - | `OPUS_RESET_STATE`. |
| `impl StreamDecoder::decode_interleaved` | 3 | `crates/audio-opus-codec/src/decoder.rs:102` | - | Decodes a packet into interleaved samples appended to `out`. `None` conceals one lost |
| `impl StreamDecoder::conceal` | 3 | `crates/audio-opus-codec/src/decoder.rs:113` | - | Conceals `n` samples (a multiple of 2.5 ms). |
| `impl StreamDecoder::decode_fec` | 4 | `crates/audio-opus-codec/src/decoder.rs:130` | - | Decodes the in-band FEC (LBRR) of `packet` for the previous (lost) packet of `n` samples. |
| `impl StreamDecoder::decode_packet` | 3 | `crates/audio-opus-codec/src/decoder.rs:152` | - |  |
| `impl StreamDecoder::decode_frame` | 5 | `crates/audio-opus-codec/src/decoder.rs:172` | - | `opus_decode_frame`: decodes one frame (or conceals when `data` is `None`/≤1 byte). |
| `smooth_fade` | 7 | `crates/audio-opus-codec/src/decoder.rs:408` | - |  |
| `impl OpusHead::simple` | 1 | `crates/audio-opus-codec/src/header.rs:28` | - | A family-0 configuration for plain mono/stereo streams (no header present). |
| `impl OpusHead::parse` | 1 | `crates/audio-opus-codec/src/header.rs:45` | - | Parses an `OpusHead` packet (Ogg first packet, Matroska `CodecPrivate`). |
| `impl OpusHead::from_dops` | 1 | `crates/audio-opus-codec/src/header.rs:62` | - | Parses the payload of an ISO-BMFF `dOps` box (Opus in MP4, big-endian fields). |
| `impl OpusHead::to_bytes` | 1 | `crates/audio-opus-codec/src/header.rs:77` | - | Serialises as an `OpusHead` packet. |
| `impl OpusHead::build` | 6 | `crates/audio-opus-codec/src/header.rs:93` | - |  |
| `impl OpusHead::gain_linear` | 1 | `crates/audio-opus-codec/src/header.rs:127` | numerics | Linear output gain factor. |
| `parse_family0_and_roundtrip` | 0 | `crates/audio-opus-codec/src/header.rs:137` | - |  |
| `parse_family1_51` | 0 | `crates/audio-opus-codec/src/header.rs:151` | - |  |
| `rejects_garbage` | 0 | `crates/audio-opus-codec/src/header.rs:161` | - |  |
| `dops` | 0 | `crates/audio-opus-codec/src/header.rs:173` | - |  |
| `impl std::fmt::Display for Error::fmt` | 2 | `crates/audio-opus-codec/src/lib.rs:32` | - |  |
| `impl Decoder::new` | 1 | `crates/audio-opus-codec/src/multistream.rs:24` | - | Creates a 48 kHz decoder from an `OpusHead` packet (Ogg / Matroska `CodecPrivate`). |
| `impl Decoder::simple` | 2 | `crates/audio-opus-codec/src/multistream.rs:29` | - | Creates a decoder for plain mono/stereo packets without a header. |
| `impl Decoder::from_head` | 2 | `crates/audio-opus-codec/src/multistream.rs:34` | - | Creates a decoder for a parsed header at `sample_rate` (8/12/16/24/48 kHz). |
| `impl Decoder::head` | 1 | `crates/audio-opus-codec/src/multistream.rs:45` | - |  |
| `impl Decoder::sample_rate` | 1 | `crates/audio-opus-codec/src/multistream.rs:49` | - |  |
| `impl Decoder::channels` | 1 | `crates/audio-opus-codec/src/multistream.rs:53` | - |  |
| `impl Decoder::pre_skip` | 1 | `crates/audio-opus-codec/src/multistream.rs:58` | - | Pre-skip in samples at the output rate. |
| `impl Decoder::set_trim_pre_skip` | 2 | `crates/audio-opus-codec/src/multistream.rs:63` | - | When enabled, the first [`Self::pre_skip`] output samples are dropped. |
| `impl Decoder::set_phase_inversion_disabled` | 2 | `crates/audio-opus-codec/src/multistream.rs:69` | - | Disables intensity-stereo phase inversion (useful when downmixing; RFC 8251 section 10). |
| `impl Decoder::final_range` | 1 | `crates/audio-opus-codec/src/multistream.rs:76` | - | Final range coder state of the first stream (conformance testing). |
| `impl Decoder::reset` | 1 | `crates/audio-opus-codec/src/multistream.rs:80` | - |  |
| `impl Decoder::decode` | 2 | `crates/audio-opus-codec/src/multistream.rs:88` | - | Decodes one packet (`None` = lost packet, concealed) into planar channels. |
| `reframe` | 1 | `crates/audio-opus-codec/src/multistream.rs:154` | - | Rebuilds a regular (code 3 VBR) packet from a parsed self-delimited one. |
| `impl Bandwidth::celt_end_band` | 1 | `crates/audio-opus-codec/src/packet.rs:30` | - | Number of CELT bands coded for this bandwidth. |
| `impl Toc::parse` | 1 | `crates/audio-opus-codec/src/packet.rs:52` | - |  |
| `impl Toc::frame_samples_48k` | 1 | `crates/audio-opus-codec/src/packet.rs:69` | - | Samples per frame at 48 kHz. |
| `impl Toc::frame_samples` | 2 | `crates/audio-opus-codec/src/packet.rs:74` | - | Samples per frame at `rate`. |
| `impl Toc::channels` | 1 | `crates/audio-opus-codec/src/packet.rs:93` | - |  |
| `parse_size` | 1 | `crates/audio-opus-codec/src/packet.rs:107` | - |  |
| `impl Packet<'a>::parse` | 1 | `crates/audio-opus-codec/src/packet.rs:117` | - | Parses a packet (RFC 6716 section 3.2), enforcing requirements R1 to R7. |
| `impl Packet<'a>::parse_self_delimited` | 1 | `crates/audio-opus-codec/src/packet.rs:123` | - | Parses a self-delimited packet (RFC 6716 Appendix B, used for all but the last stream of |
| `impl Packet<'a>::parse_impl` | 2 | `crates/audio-opus-codec/src/packet.rs:127` | - |  |
| `packet_samples_48k` | 1 | `crates/audio-opus-codec/src/packet.rs:264` | - | Number of samples (at 48 kHz) in a packet, or an error for malformed framing. |
| `toc_configs` | 0 | `crates/audio-opus-codec/src/packet.rs:274` | - |  |
| `frame_codes` | 0 | `crates/audio-opus-codec/src/packet.rs:289` | - |  |
| `two_byte_sizes` | 0 | `crates/audio-opus-codec/src/packet.rs:316` | - |  |
| `self_delimited` | 0 | `crates/audio-opus-codec/src/packet.rs:326` | - |  |
| `ilog` | 1 | `crates/audio-opus-codec/src/range.rs:20` | - | Number of bits needed to represent `x` (0 for 0). |
| `impl RangeDecoder<'a>::new` | 1 | `crates/audio-opus-codec/src/range.rs:42` | - |  |
| `impl RangeDecoder<'a>::storage` | 1 | `crates/audio-opus-codec/src/range.rs:63` | - | Size of the buffer in bytes as seen by the decoder. |
| `impl RangeDecoder<'a>::shrink_storage` | 2 | `crates/audio-opus-codec/src/range.rs:68` | - | Shrinks the buffer seen by the decoder (raw bits are then read from the new end). |
| `impl RangeDecoder<'a>::rng` | 1 | `crates/audio-opus-codec/src/range.rs:72` | - |  |
| `impl RangeDecoder<'a>::read_byte` | 1 | `crates/audio-opus-codec/src/range.rs:77` | - |  |
| `impl RangeDecoder<'a>::read_byte_from_end` | 1 | `crates/audio-opus-codec/src/range.rs:88` | - |  |
| `impl RangeDecoder<'a>::normalize` | 1 | `crates/audio-opus-codec/src/range.rs:98` | - |  |
| `impl RangeDecoder<'a>::decode` | 2 | `crates/audio-opus-codec/src/range.rs:111` | - | Returns the cumulative frequency of the next symbol for total `ft` (call [`Self::update`] next). |
| `impl RangeDecoder<'a>::decode_bin` | 2 | `crates/audio-opus-codec/src/range.rs:120` | - | Like [`Self::decode`] with `ft = 1 << bits`. |
| `impl RangeDecoder<'a>::update` | 4 | `crates/audio-opus-codec/src/range.rs:128` | - | Consumes the symbol with cumulative range `[fl, fh)` of total `ft`. |
| `impl RangeDecoder<'a>::update_bin` | 4 | `crates/audio-opus-codec/src/range.rs:138` | - | Like [`Self::update`] for a symbol decoded with [`Self::decode_bin`]. |
| `impl RangeDecoder<'a>::bit_logp` | 2 | `crates/audio-opus-codec/src/range.rs:149` | - | Decodes a binary symbol whose probability of being 1 is `1/2^logp`. |
| `impl RangeDecoder<'a>::icdf` | 3 | `crates/audio-opus-codec/src/range.rs:164` | - | Decodes a symbol with an "inverse CDF" table (`icdf[i] = ft - cdf[i+1]`, `ft = 1 << ftb`). |
| `impl RangeDecoder<'a>::uint` | 2 | `crates/audio-opus-codec/src/range.rs:190` | - | Decodes a uniformly distributed integer in `[0, ft)` (`ft >= 2`). |
| `impl RangeDecoder<'a>::bits` | 2 | `crates/audio-opus-codec/src/range.rs:213` | - | Reads `bits` raw bits (0..=25) from the end of the buffer. |
| `impl RangeDecoder<'a>::skip_to_end` | 2 | `crates/audio-opus-codec/src/range.rs:238` | - | Advances the bit counter so that [`Self::tell`] reports `total` (CELT silence frames). |
| `impl RangeDecoder<'a>::tell` | 1 | `crates/audio-opus-codec/src/range.rs:245` | - | Number of bits consumed so far (rounded up). |
| `impl RangeDecoder<'a>::tell_frac` | 1 | `crates/audio-opus-codec/src/range.rs:250` | - | Number of 1/8 bits consumed so far (rounded up). |
| `impl RangeEncoder::new` | 1 | `crates/audio-opus-codec/src/range.rs:282` | - |  |
| `impl RangeEncoder::carry_out` | 2 | `crates/audio-opus-codec/src/range.rs:285` | - |  |
| `impl RangeEncoder::normalize` | 1 | `crates/audio-opus-codec/src/range.rs:303` | - |  |
| `impl RangeEncoder::encode` | 4 | `crates/audio-opus-codec/src/range.rs:310` | - |  |
| `impl RangeEncoder::bit_logp` | 3 | `crates/audio-opus-codec/src/range.rs:320` | - |  |
| `impl RangeEncoder::icdf` | 4 | `crates/audio-opus-codec/src/range.rs:332` | - |  |
| `impl RangeEncoder::uint` | 3 | `crates/audio-opus-codec/src/range.rs:342` | - |  |
| `impl RangeEncoder::bits` | 3 | `crates/audio-opus-codec/src/range.rs:355` | - |  |
| `impl RangeEncoder::done` | 1 | `crates/audio-opus-codec/src/range.rs:371` | - | Finalises and returns a buffer of exactly `size` bytes. |
| `impl Lcg::next` | 1 | `crates/audio-opus-codec/src/range.rs:413` | - |  |
| `roundtrip_mixed_symbols` | 0 | `crates/audio-opus-codec/src/range.rs:420` | - |  |
| `tell_starts_at_one_bit` | 0 | `crates/audio-opus-codec/src/range.rs:479` | - |  |
| `empty_and_garbage_never_panic` | 0 | `crates/audio-opus-codec/src/range.rs:487` | - |  |
| `smulwb` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:9` | - |  |
| `smlawb` | 3 | `crates/audio-opus-codec/src/silk/lpc.rs:13` | - |  |
| `smulww` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:17` | - |  |
| `smmul` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:21` | - |  |
| `rshift_round` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:25` | - |  |
| `rshift_round64` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:29` | - |  |
| `sat16` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:33` | - |  |
| `inverse32_varq` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:38` | - | `silk_INVERSE32_varQ`: approximation of `(1 << q_res) / b32`. |
| `div32_varq` | 3 | `crates/audio-opus-codec/src/silk/lpc.rs:59` | - | `silk_DIV32_varQ`: approximation of `(a32 << q_res) / b32`. |
| `sqrt_approx` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:81` | - |  |
| `nlsf_decode` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:93` | - | Decodes the stage-1/stage-2 indices into stabilised NLSFs (Q15). |
| `nlsf_stabilize` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:133` | - |  |
| `bwexpander_32` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:180` | - |  |
| `bwexpander` | 2 | `crates/audio-opus-codec/src/silk/lpc.rs:191` | - | `silk_bwexpander` on Q12 coefficients (16-bit). |
| `lpc_inverse_pred_gain` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:201` | - |  |
| `nlsf2a` | 1 | `crates/audio-opus-codec/src/silk/lpc.rs:256` | - | Converts NLSFs (Q15) to stable Q12 LPC coefficients (`silk_NLSF2A`). |
| `flat_nlsf_gives_small_lpc` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:343` | - |  |
| `stabilize_enforces_spacing` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:354` | - |  |
| `inverse_approximations` | 0 | `crates/audio-opus-codec/src/silk/lpc.rs:365` | - |  |
| `impl Channel::new` | 0 | `crates/audio-opus-codec/src/silk/mod.rs:87` | - |  |
| `impl Channel::set_fs` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:120` | - |  |
| `impl Channel::decode_indices` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:142` | - |  |
| `impl Channel::decode_frame` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:207` | - |  |
| `impl Channel::decode_parameters` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:236` | - |  |
| `impl Channel::decode_core` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:309` | - |  |
| `impl Channel::conceal` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:419` | - | Basic concealment: pitch-periodic repetition for voiced history, otherwise LPC-shaped |
| `log2lin` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:482` | - |  |
| `lpc_analysis_filter` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:494` | - |  |
| `decode_pulses` | 5 | `crates/audio-opus-codec/src/silk/mod.rs:509` | - |  |
| `shell_decode` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:563` | - |  |
| `split` | 4 | `crates/audio-opus-codec/src/silk/mod.rs:564` | - |  |
| `stereo_decode_pred` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:595` | - |  |
| `stereo_ms_to_lr` | 6 | `crates/audio-opus-codec/src/silk/mod.rs:615` | - |  |
| `impl SilkDecoder::new` | 2 | `crates/audio-opus-codec/src/silk/mod.rs:663` | - |  |
| `impl SilkDecoder::reset` | 1 | `crates/audio-opus-codec/src/silk/mod.rs:678` | - |  |
| `impl SilkDecoder::set_stream` | 3 | `crates/audio-opus-codec/src/silk/mod.rs:685` | - |  |
| `impl SilkDecoder::decode` | 6 | `crates/audio-opus-codec/src/silk/mod.rs:691` | - | Decodes one SILK frame (10 or 20 ms) into interleaved `out` at the API rate. |
| `gcd` | 2 | `crates/audio-opus-codec/src/silk/resampler.rs:27` | - |  |
| `design_fractional` | 5 | `crates/audio-opus-codec/src/silk/resampler.rs:34` | - | Least-squares fractional-delay interpolator: taps at offsets `-past+1 ..= future` (in input |
| `impl Resampler::new` | 2 | `crates/audio-opus-codec/src/silk/resampler.rs:73` | - |  |
| `impl Resampler::rates` | 1 | `crates/audio-opus-codec/src/silk/resampler.rs:127` | - |  |
| `impl Resampler::process` | 3 | `crates/audio-opus-codec/src/silk/resampler.rs:132` | - | Resamples `input` (16-bit scale) and appends the output samples. |
| `upsample_sine_and_delay` | 0 | `crates/audio-opus-codec/src/silk/resampler.rs:176` | - |  |
| `icdf` | 1 | `crates/audio-opus-codec/src/silk/tables.rs:7` | - | Converts a PDF (summing to 256) into an inverse CDF (`256 - cdf[i+1]`). |

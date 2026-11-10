# `crates/acoustics-daw-session`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Native audio codecs: pure-Rust AIFF/AIFF-C, FLAC encoder, RF64/BWF WAV, and PeakFile waveform pyramids._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl AudioBuffer::new` | 3 | `crates/acoustics-daw-session/src/codecs/mod.rs:13` | - |  |
| `impl AudioBuffer::num_channels` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:20` | - |  |
| `impl AudioBuffer::num_frames` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:24` | - |  |
| `encode_wav` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:34` | - | Encodes an AudioBuffer as a 16-bit PCM standard WAV file. |
| `decode_wav` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:78` | - | Decodes standard 16-bit PCM or 32-bit float WAV bytes into an AudioBuffer. |
| `encode_aiff` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:148` | - | Encodes an AudioBuffer as a standard 16-bit AIFF file. |
| `u32_to_extended` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:190` | - |  |
| `encode_flac` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:208` | - | Encodes an AudioBuffer as a valid lossless FLAC bitstream (subset: RFC 9639 uncompressed / verbatim subframes). |
| `impl PeakFile::build` | 1 | `crates/acoustics-daw-session/src/codecs/mod.rs:260` | - |  |
| `impl Default for BiquadCoeffs::default` | 0 | `crates/acoustics-daw-session/src/dsp/biquad.rs:30` | - |  |
| `impl BiquadCoeffs::design` | 5 | `crates/acoustics-daw-session/src/dsp/biquad.rs:45` | - | Designs an audio filter from frequency, Q factor, gain (dB), and sample rate. |
| `impl BiquadFilter::new` | 1 | `crates/acoustics-daw-session/src/dsp/biquad.rs:152` | - |  |
| `impl BiquadFilter::reset` | 1 | `crates/acoustics-daw-session/src/dsp/biquad.rs:160` | - |  |
| `impl BiquadFilter::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/biquad.rs:166` | - |  |
| `impl BiquadFilter::process_buffer` | 2 | `crates/acoustics-daw-session/src/dsp/biquad.rs:180` | - |  |
| `k_weight_stage1_coeffs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:7` | - | BS.1770 K-weighting pre-filter stage 1 (high shelf: +4 dB high boost). |
| `k_weight_stage2_coeffs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:16` | - | BS.1770 K-weighting pre-filter stage 2 (high pass: 100 Hz cutoff). |
| `impl KWeightingChannel::new` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:30` | - |  |
| `impl KWeightingChannel::filter_sample` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:38` | - |  |
| `impl EbuR128Meter::new` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:57` | - |  |
| `impl EbuR128Meter::process_interleaved` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:75` | - | Feeds interleaved multi-channel audio samples into the loudness meter. |
| `impl EbuR128Meter::integrated_lufs` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:99` | - | Computes the gated integrated loudness in LUFS (-70 LUFS absolute gate, -10 LU relative gate). |
| `impl TruePeakMeter::new` | 0 | `crates/acoustics-daw-session/src/dsp/meter.rs:145` | - |  |
| `impl TruePeakMeter::process_samples` | 2 | `crates/acoustics-daw-session/src/dsp/meter.rs:150` | - | Feeds samples and updates the max inter-sample true-peak value. |
| `impl TruePeakMeter::true_peak_db` | 1 | `crates/acoustics-daw-session/src/dsp/meter.rs:168` | - |  |
| `impl Default for Oscillator::default` | 0 | `crates/acoustics-daw-session/src/dsp/osc.rs:26` | - |  |
| `impl Oscillator::new` | 3 | `crates/acoustics-daw-session/src/dsp/osc.rs:32` | - |  |
| `impl Oscillator::set_frequency` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:42` | - |  |
| `impl Oscillator::next_sample` | 1 | `crates/acoustics-daw-session/src/dsp/osc.rs:48` | - | Generates the next output sample and advances phase. |
| `impl Oscillator::fill_buffer` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:75` | - | Fills a buffer with generated oscillator samples. |
| `polyblep` | 2 | `crates/acoustics-daw-session/src/dsp/osc.rs:84` | - | PolyBLEP residual function for smoothing step discontinuities across [-dt, +dt]. |
| `stereo_pan_gains` | 2 | `crates/acoustics-daw-session/src/dsp/pan.rs:17` | - | Computes left and right channel gains for a given pan position (-1.0 hard left .. +1.0 hard right). |
| `impl SpeakerPosition::new` | 4 | `crates/acoustics-daw-session/src/dsp/pan.rs:45` | - |  |
| `impl SpeakerPosition::to_cartesian` | 1 | `crates/acoustics-daw-session/src/dsp/pan.rs:50` | - | Unit Cartesian direction vector (x: right, y: front, z: up). |
| `impl VbapPanner<'a>::new` | 1 | `crates/acoustics-daw-session/src/dsp/pan.rs:90` | - |  |
| `impl VbapPanner<'a>::pan` | 4 | `crates/acoustics-daw-session/src/dsp/pan.rs:95` | - | Computes speaker gains for a sound source at (azimuth_deg, elevation_deg). |
| `detect_pitch_yin` | 5 | `crates/acoustics-daw-session/src/dsp/pitch_detect.rs:16` | - | Computes the fundamental frequency $f_0$ of an audio frame using the YIN algorithm with parabolic interpolation. |
| `hz_to_midi` | 1 | `crates/acoustics-daw-session/src/dsp/pitch_detect.rs:88` | - | Converts a frequency in Hz to closest MIDI note number (A4 = 440 Hz = 69). |
| `impl Eq7::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:14` | - |  |
| `impl Eq7::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:28` | - |  |
| `impl Compressor::new` | 6 | `crates/acoustics-daw-session/src/dsp/plugins.rs:49` | - |  |
| `impl Compressor::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:62` | - |  |
| `impl ExpanderGate::new` | 5 | `crates/acoustics-daw-session/src/dsp/plugins.rs:99` | - |  |
| `impl ExpanderGate::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:111` | - |  |
| `impl Maximizer::new` | 3 | `crates/acoustics-daw-session/src/dsp/plugins.rs:148` | - |  |
| `impl Maximizer::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:162` | - |  |
| `impl DeEsser::new` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:197` | - |  |
| `impl DeEsser::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:208` | - |  |
| `impl ChannelStrip::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:225` | - |  |
| `impl ChannelStrip::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:234` | - |  |
| `impl FdnReverb::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:252` | - |  |
| `impl FdnReverb::process_sample` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:282` | - |  |
| `impl SubtractiveSynth::new` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:324` | - |  |
| `impl SubtractiveSynth::note_on` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:333` | - |  |
| `impl SubtractiveSynth::note_off` | 2 | `crates/acoustics-daw-session/src/dsp/plugins.rs:342` | - |  |
| `impl SubtractiveSynth::process_sample` | 1 | `crates/acoustics-daw-session/src/dsp/plugins.rs:350` | - |  |
| `impl FlexWarpMode::window_params` | 1 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:20` | - | Window size in seconds and maximum search tolerance fraction for WSOLA. |
| `impl WarpPoint::new` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:39` | - |  |
| `evaluate_warp_timeline` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:45` | numerics | Evaluates source playback time $t_{\text{src}}$ corresponding to destination time $t_{\text{dst}}$ using piecewise linear interpolation. |
| `impl WsolaTimeStretch::new` | 2 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:80` | - |  |
| `impl WsolaTimeStretch::process` | 3 | `crates/acoustics-daw-session/src/elastic_warp/mod.rs:88` | - | Stretches input audio samples to output length using WSOLA. |
| `test_ebu_r128_metering` | 0 | `crates/acoustics-daw-session/src/lib.rs:40` | - |  |
| `test_vbap_panning` | 0 | `crates/acoustics-daw-session/src/lib.rs:59` | - |  |
| `test_yin_pitch_detection` | 0 | `crates/acoustics-daw-session/src/lib.rs:70` | - |  |
| `test_camelot_harmony_relations` | 0 | `crates/acoustics-daw-session/src/lib.rs:86` | - |  |
| `test_wav_codec_roundtrip` | 0 | `crates/acoustics-daw-session/src/lib.rs:99` | - |  |
| `test_pdc_mix_engine` | 0 | `crates/acoustics-daw-session/src/lib.rs:116` | - |  |
| `impl AutomationCurve::evaluate_at` | 2 | `crates/acoustics-daw-session/src/mix_session/mod.rs:31` | - |  |
| `impl DawTrack::new` | 3 | `crates/acoustics-daw-session/src/mix_session/mod.rs:83` | - |  |
| `impl DawSession::new` | 3 | `crates/acoustics-daw-session/src/mix_session/mod.rs:110` | - |  |
| `impl PdcMixEngine::new` | 0 | `crates/acoustics-daw-session/src/mix_session/mod.rs:127` | - |  |
| `impl PdcMixEngine::render_block` | 4 | `crates/acoustics-daw-session/src/mix_session/mod.rs:134` | - | Renders a block of audio (left, right) for the session at `start_sample` with exact latency alignment. |
| `impl CamelotKey::new` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:14` | - |  |
| `impl CamelotKey::to_string` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:23` | - |  |
| `impl CamelotKey::parse` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:27` | - |  |
| `impl HarmonicRelation::between` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:50` | - |  |
| `detect_camelot_key` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:71` | - | Krumhansl-Schmuckler key detection from a 12-element chroma profile. |
| `segment_musical_phrases` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:129` | - | Detects 8-bar musical phrases from energy envelopes across bars. |
| `parse_scratch_dsl` | 1 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:181` | - | Parses a turntable scratch pattern string into beats of actions (e.g. "AEA.AEA."). |
| `evaluate_scratch_position` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:210` | - | Evaluates turntable needle position (in beats) and mute gain at beat time $t$. |
| `encode_mtc_quarter_frame` | 2 | `crates/acoustics-daw-session/src/music_intelligence/mod.rs:242` | - | Encodes an MTC quarter-frame MIDI message (0xF1, data_byte). |

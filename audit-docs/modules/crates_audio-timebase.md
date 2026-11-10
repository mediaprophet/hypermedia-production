# `crates/audio-timebase`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Audio timebase, synchronization, metric and SMPTE conversions.

Provides deterministic conversions between musical time (bars, beats, ticks at PPQ = 960),
real elapsed seconds, raw sample indices across sample rates (8 kHz to 768 kHz),
drop-frame and non-drop-frame SMPTE timecodes, 35mm film Feet+Frames, and dynamic tempo maps.

Grounded in international human rights instruments (UDHR Articles 19, 27; ICESCR Article 15)
and Informatics Fiduciary duties ensuring lossless temporal synchronization and moral attribution._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl NoteValue::ticks` | 1 | `crates/audio-timebase/src/grid.rs:31` | - | Ticks in a straight note of this value in 4/4. |
| `impl NoteValue::label` | 1 | `crates/audio-timebase/src/grid.rs:43` | - |  |
| `impl Default for GridDivision::default` | 0 | `crates/audio-timebase/src/grid.rs:77` | - |  |
| `impl GridDivision::label` | 1 | `crates/audio-timebase/src/grid.rs:87` | - |  |
| `impl GridDivision::note_ticks` | 5 | `crates/audio-timebase/src/grid.rs:105` | - |  |
| `impl GridDivision::step_samples` | 5 | `crates/audio-timebase/src/grid.rs:122` | - | Step size in samples at position `at`. |
| `impl GridDivision::snap` | 5 | `crates/audio-timebase/src/grid.rs:144` | - | Snap `pos` to the nearest grid line. |
| `impl GridDivision::line_at_or_before` | 5 | `crates/audio-timebase/src/grid.rs:161` | - | The grid line at or immediately before `pos`. |
| `impl GridDivision::next_line` | 5 | `crates/audio-timebase/src/grid.rs:185` | - | The first grid line strictly after `line`. |
| `impl GridDivision::lines` | 7 | `crates/audio-timebase/src/grid.rs:202` | - | List all grid lines within `[start, end)`, capped at `max` entries. |
| `sixteenth_grid_at_120` | 0 | `crates/audio-timebase/src/grid.rs:232` | - |  |
| `absolute_grids` | 0 | `crates/audio-timebase/src/grid.rs:245` | - |  |
| `lines_are_capped` | 0 | `crates/audio-timebase/src/grid.rs:253` | - |  |
| `triplets_and_dots` | 0 | `crates/audio-timebase/src/grid.rs:262` | - |  |
| `full_pipeline_round_trip` | 0 | `crates/audio-timebase/src/lib.rs:54` | - |  |
| `grid_snapping_consistency` | 0 | `crates/audio-timebase/src/lib.rs:77` | - |  |
| `to_samples` | 1 | `crates/audio-timebase/src/rate.rs:18` | - | Round a float sample count to `i64`, mapping NaN to 0 and saturating infinities. |
| `impl SampleRate::new` | 1 | `crates/audio-timebase/src/rate.rs:48` | - |  |
| `impl SampleRate::hz` | 1 | `crates/audio-timebase/src/rate.rs:56` | - |  |
| `impl SampleRate::as_f64` | 1 | `crates/audio-timebase/src/rate.rs:60` | - |  |
| `impl SampleRate::seconds` | 2 | `crates/audio-timebase/src/rate.rs:65` | - | Convert samples to real seconds. |
| `impl SampleRate::samples` | 2 | `crates/audio-timebase/src/rate.rs:70` | - | Convert real seconds to samples, rounded to nearest and saturated. |
| `impl Default for SampleRate::default` | 0 | `crates/audio-timebase/src/rate.rs:76` | - |  |
| `impl SampleRange::new` | 2 | `crates/audio-timebase/src/rate.rs:89` | - |  |
| `impl SampleRange::point` | 1 | `crates/audio-timebase/src/rate.rs:96` | - |  |
| `impl SampleRange::len` | 1 | `crates/audio-timebase/src/rate.rs:99` | - |  |
| `impl SampleRange::is_empty` | 1 | `crates/audio-timebase/src/rate.rs:102` | - |  |
| `impl SampleRange::contains` | 2 | `crates/audio-timebase/src/rate.rs:105` | - |  |
| `impl SampleRange::overlaps` | 2 | `crates/audio-timebase/src/rate.rs:108` | - |  |
| `impl SampleRange::intersect` | 2 | `crates/audio-timebase/src/rate.rs:111` | - |  |
| `impl SampleRange::shifted` | 2 | `crates/audio-timebase/src/rate.rs:116` | - |  |
| `sample_rate_bounds` | 0 | `crates/audio-timebase/src/rate.rs:129` | - |  |
| `seconds_round_trip` | 0 | `crates/audio-timebase/src/rate.rs:136` | - |  |
| `to_samples_is_total` | 0 | `crates/audio-timebase/src/rate.rs:143` | - |  |
| `range_ops` | 0 | `crates/audio-timebase/src/rate.rs:150` | - |  |
| `impl TimeSig::new` | 2 | `crates/audio-timebase/src/tempo.rs:21` | - |  |
| `impl TimeSig::ticks_per_beat` | 1 | `crates/audio-timebase/src/tempo.rs:26` | - | Number of ticks in one beat under this meter. |
| `impl TimeSig::ticks_per_bar` | 1 | `crates/audio-timebase/src/tempo.rs:32` | - | Number of ticks in a complete bar under this meter. |
| `impl Default for TimeSig::default` | 0 | `crates/audio-timebase/src/tempo.rs:38` | - |  |
| `impl MeterPoint::new` | 3 | `crates/audio-timebase/src/tempo.rs:64` | - |  |
| `impl MeterPoint::numerator` | 1 | `crates/audio-timebase/src/tempo.rs:71` | - |  |
| `impl MeterPoint::denominator` | 1 | `crates/audio-timebase/src/tempo.rs:75` | - |  |
| `impl MeterPoint::ticks_per_beat` | 1 | `crates/audio-timebase/src/tempo.rs:79` | - |  |
| `impl MeterPoint::ticks_per_bar` | 1 | `crates/audio-timebase/src/tempo.rs:83` | - |  |
| `impl BarBeat::new` | 3 | `crates/audio-timebase/src/tempo.rs:97` | - |  |
| `impl Default for TempoMap::default` | 0 | `crates/audio-timebase/src/tempo.rs:110` | - |  |
| `valid_bpm` | 1 | `crates/audio-timebase/src/tempo.rs:118` | - |  |
| `impl TempoMap::new` | 3 | `crates/audio-timebase/src/tempo.rs:127` | - |  |
| `impl TempoMap::tempos` | 1 | `crates/audio-timebase/src/tempo.rs:134` | - |  |
| `impl TempoMap::meters` | 1 | `crates/audio-timebase/src/tempo.rs:138` | - |  |
| `impl TempoMap::set_tempo` | 3 | `crates/audio-timebase/src/tempo.rs:143` | - | Insert or update the tempo at `tick`. |
| `impl TempoMap::remove_tempo` | 2 | `crates/audio-timebase/src/tempo.rs:158` | - | Remove a tempo event (the initial event at tick 0 is pinned). |
| `impl TempoMap::set_meter` | 4 | `crates/audio-timebase/src/tempo.rs:168` | - | Insert or update the meter at `tick` (snapped to the bar line at or before it). |
| `impl TempoMap::remove_meter` | 2 | `crates/audio-timebase/src/tempo.rs:190` | - | Remove a meter event (the initial event at tick 0 is pinned). |
| `impl TempoMap::tempo_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:200` | - | Active tempo at `tick`. |
| `impl TempoMap::meter_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:213` | - | Active meter at `tick`. |
| `impl TempoMap::samples_per_tick` | 2 | `crates/audio-timebase/src/tempo.rs:228` | - |  |
| `impl TempoMap::tick_to_samples` | 3 | `crates/audio-timebase/src/tempo.rs:233` | - | Convert ticks to samples. |
| `impl TempoMap::tick_to_samples_f` | 3 | `crates/audio-timebase/src/tempo.rs:237` | - |  |
| `impl TempoMap::samples_to_ticks_f` | 3 | `crates/audio-timebase/src/tempo.rs:253` | - | Convert samples to fractional ticks. |
| `impl TempoMap::samples_to_ticks` | 3 | `crates/audio-timebase/src/tempo.rs:271` | - | Convert samples to integer ticks. |
| `impl TempoMap::bar_start_tick` | 2 | `crates/audio-timebase/src/tempo.rs:276` | - | Starting tick of 1-based `bar`. |
| `impl TempoMap::bar_beat_at_tick` | 2 | `crates/audio-timebase/src/tempo.rs:299` | - | Musical `BarBeat` position at `tick`. |
| `impl TempoMap::tick_at_bar_beat` | 2 | `crates/audio-timebase/src/tempo.rs:338` | - | Musical `BarBeat` to absolute `tick`. |
| `impl TempoMap::bar_beat_at` | 3 | `crates/audio-timebase/src/tempo.rs:347` | - | Musical `BarBeat` at sample index `s`. |
| `impl TempoMap::samples_at_bar_beat` | 3 | `crates/audio-timebase/src/tempo.rs:352` | - | Sample index at musical `BarBeat`. |
| `impl TempoMap::scale_tempos` | 2 | `crates/audio-timebase/src/tempo.rs:357` | - | Scale all tempos by a constant ratio. |
| `default_map_is_120_4_4` | 0 | `crates/audio-timebase/src/tempo.rs:372` | - |  |
| `tempo_change_is_integrated` | 0 | `crates/audio-timebase/src/tempo.rs:384` | - |  |
| `meter_change_counts_bars` | 0 | `crates/audio-timebase/src/tempo.rs:399` | - |  |
| `eighth_note_meters` | 0 | `crates/audio-timebase/src/tempo.rs:416` | - |  |
| `rejects_bad_values` | 0 | `crates/audio-timebase/src/tempo.rs:429` | - |  |
| `negative_positions` | 0 | `crates/audio-timebase/src/tempo.rs:439` | - |  |
| `impl TimecodeRate::rational` | 1 | `crates/audio-timebase/src/timecode.rs:50` | - | Exact rational frame rate (numerator, denominator). |
| `impl TimecodeRate::nominal` | 1 | `crates/audio-timebase/src/timecode.rs:69` | - | Nominal frames per second integer base. |
| `impl TimecodeRate::is_drop` | 1 | `crates/audio-timebase/src/timecode.rs:75` | - | Whether this is a drop-frame SMPTE timecode rate. |
| `impl TimecodeRate::label` | 1 | `crates/audio-timebase/src/timecode.rs:80` | - | Short human-readable display label. |
| `impl TimecodeRate::frames_at` | 3 | `crates/audio-timebase/src/timecode.rs:101` | - | Total whole frames at a sample position (floored). |
| `impl TimecodeRate::samples_at_frame` | 3 | `crates/audio-timebase/src/timecode.rs:112` | - | Sample position at the beginning of frame `f`. |
| `impl TimecodeRate::samples_per_frame` | 2 | `crates/audio-timebase/src/timecode.rs:122` | - | Samples per frame (fractional). |
| `impl Timecode::new` | 5 | `crates/audio-timebase/src/timecode.rs:139` | - |  |
| `impl Timecode::from_frames` | 2 | `crates/audio-timebase/src/timecode.rs:150` | - | Compute timecode label for an absolute frame count. |
| `impl Timecode::to_frames` | 2 | `crates/audio-timebase/src/timecode.rs:179` | - | Absolute frame count corresponding to this timecode label. |
| `impl Timecode::from_samples` | 3 | `crates/audio-timebase/src/timecode.rs:191` | - | Derive timecode label from sample position. |
| `impl Timecode::to_samples` | 3 | `crates/audio-timebase/src/timecode.rs:196` | - | Sample index at the start of this timecode. |
| `impl Timecode::format` | 2 | `crates/audio-timebase/src/timecode.rs:201` | - | Format as standard `HH:MM:SS:FF` or drop-frame `HH:MM:SS;FF`. |
| `impl Timecode::parse` | 1 | `crates/audio-timebase/src/timecode.rs:211` | - | Parse timecode string (`HH:MM:SS:FF`, with `:` / `;` / `.` separators). |
| `impl FeetFrames::new` | 2 | `crates/audio-timebase/src/timecode.rs:255` | - |  |
| `impl FeetFrames::from_samples` | 3 | `crates/audio-timebase/src/timecode.rs:260` | - | Derive FeetFrames from raw audio samples. |
| `impl FeetFrames::to_total_frames` | 1 | `crates/audio-timebase/src/timecode.rs:269` | - | Total frames represented by this FeetFrames value. |
| `impl FeetFrames::to_samples` | 3 | `crates/audio-timebase/src/timecode.rs:274` | - | Convert FeetFrames to audio samples. |
| `impl FeetFrames::format` | 1 | `crates/audio-timebase/src/timecode.rs:279` | - | Format as standard `FEET+FRAMES` (e.g. `1+08`). |
| `impl FeetFrames::parse` | 1 | `crates/audio-timebase/src/timecode.rs:284` | - | Parse `FEET+FRAMES` format string. |
| `feet_frames` | 3 | `crates/audio-timebase/src/timecode.rs:294` | - | Compatibility function returning raw `(feet, frames)` tuple for 35mm film. |
| `non_drop_round_trip` | 0 | `crates/audio-timebase/src/timecode.rs:304` | - |  |
| `drop_frame_labels` | 0 | `crates/audio-timebase/src/timecode.rs:312` | - |  |
| `parse_variants` | 0 | `crates/audio-timebase/src/timecode.rs:323` | - |  |
| `feet_and_frames` | 0 | `crates/audio-timebase/src/timecode.rs:333` | - |  |
| `impl TimeFormat::label` | 1 | `crates/audio-timebase/src/transport.rs:29` | - |  |
| `impl TimeFormat::id` | 1 | `crates/audio-timebase/src/transport.rs:39` | - |  |
| `impl TimeFormat::from_id` | 1 | `crates/audio-timebase/src/transport.rs:49` | - |  |
| `impl TransportPosition::from_samples` | 5 | `crates/audio-timebase/src/transport.rs:69` | - | Calculate all multi-timebase coordinates from raw audio samples. |
| `impl TransportPosition::format` | 3 | `crates/audio-timebase/src/transport.rs:87` | - | Format this position according to the specified `TimeFormat`. |
| `min_secs` | 2 | `crates/audio-timebase/src/transport.rs:110` | - |  |
| `format_position` | 6 | `crates/audio-timebase/src/transport.rs:120` | - | Format an absolute sample position into a string. |
| `format_length` | 6 | `crates/audio-timebase/src/transport.rs:146` | - | Format a duration or length starting at `start`. |
| `parse_position` | 6 | `crates/audio-timebase/src/transport.rs:174` | - | Parse a user-entered position string into samples. |
| `ctx` | 0 | `crates/audio-timebase/src/transport.rs:232` | - |  |
| `format_min_secs` | 0 | `crates/audio-timebase/src/transport.rs:237` | - |  |
| `transport_position_snapshot` | 0 | `crates/audio-timebase/src/transport.rs:250` | - |  |
| `parse_various_formats` | 0 | `crates/audio-timebase/src/transport.rs:259` | - |  |

# `crates/timeline-editing`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Application-neutral editorial time and transition editing.

The selected chronicle media operations are adapted to a small public surface with
no application model, codec, renderer, file, GPU, or runtime dependencies.
`Tick` uses the chronicle media exact media scale. Simulation time remains owned by
simulation systems; this crate describes editorial positions and frame
boundaries._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `main` | 0 | `crates/timeline-editing/examples/transition_edit.rs:7` | - |  |
| `impl Tick::from_rational` | 3 | `crates/timeline-editing/src/timebase.rs:16` | - | Convert a rational timestamp `pts * num / den` seconds. A zero timebase |
| `impl Tick::to_rational_floor` | 3 | `crates/timeline-editing/src/timebase.rs:34` | - | Inverse rational conversion rounded down in the requested timebase. |
| `impl Tick::from_units` | 2 | `crates/timeline-editing/src/timebase.rs:47` | - | Convert a count of units at `per_second`, such as audio samples, |
| `impl Tick::to_units_floor` | 2 | `crates/timeline-editing/src/timebase.rs:60` | - | Convert to units at `per_second`, rounded down (unlike `from_units`, |
| `impl Tick::checked_add` | 2 | `crates/timeline-editing/src/timebase.rs:70` | - |  |
| `impl Tick::checked_sub` | 2 | `crates/timeline-editing/src/timebase.rs:74` | - |  |
| `impl Tick::mul_ratio` | 3 | `crates/timeline-editing/src/timebase.rs:80` | - | Floor-scale by a rational factor. A zero divisor maps to zero, matching |
| `impl Tick::max` | 2 | `crates/timeline-editing/src/timebase.rs:90` | - |  |
| `impl Tick::min` | 2 | `crates/timeline-editing/src/timebase.rs:94` | - |  |
| `impl TimeRange::new` | 2 | `crates/timeline-editing/src/timebase.rs:107` | - |  |
| `impl TimeRange::end` | 1 | `crates/timeline-editing/src/timebase.rs:111` | - |  |
| `impl TimeRange::is_empty` | 1 | `crates/timeline-editing/src/timebase.rs:117` | - |  |
| `impl Default for FrameRate::default` | 0 | `crates/timeline-editing/src/timebase.rs:130` | - |  |
| `impl FrameRate::new` | 2 | `crates/timeline-editing/src/timebase.rs:148` | - |  |
| `impl FrameRate::sane` | 1 | `crates/timeline-editing/src/timebase.rs:157` | - | Invalid or non-positive rates use the source's documented default. |
| `impl FrameRate::frame_duration` | 1 | `crates/timeline-editing/src/timebase.rs:165` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/timeline-editing/src/timebase.rs:174` | - | Index of the frame containing `time`, rounded down. |
| `impl FrameRate::tick_of` | 2 | `crates/timeline-editing/src/timebase.rs:180` | - | Start tick for frame `frame`, rounded up so the tick belongs to that frame. |
| `impl FrameRate::snap` | 2 | `crates/timeline-editing/src/timebase.rs:186` | - | Snap down to a frame boundary. |
| `frame_index` | 2 | `crates/timeline-editing/src/timebase.rs:194` | - | Keep the mathematical frame index wide until the public API needs an i64. |
| `frame_boundary` | 2 | `crates/timeline-editing/src/timebase.rs:202` | - | Ceiling division for a frame boundary, matching the chronicle media exact rational |
| `clamp_i64` | 1 | `crates/timeline-editing/src/timebase.rs:214` | - |  |
| `gcd` | 2 | `crates/timeline-editing/src/timebase.rs:218` | - |  |
| `rational_rates_and_audio_units_have_exact_boundaries` | 0 | `crates/timeline-editing/src/timebase.rs:230` | - |  |
| `signed_unit_conversion_keeps_source_truncation_distinct_from_floor` | 0 | `crates/timeline-editing/src/timebase.rs:240` | - |  |
| `rational_container_time_round_trips_to_its_timebase` | 0 | `crates/timeline-editing/src/timebase.rs:246` | - |  |
| `invalid_rates_and_zero_denominators_do_not_panic` | 0 | `crates/timeline-editing/src/timebase.rs:254` | - |  |
| `snap_keeps_exotic_frame_indices_wide_until_the_tick_endpoint` | 0 | `crates/timeline-editing/src/timebase.rs:261` | - |  |
| `impl ClipSpan::new` | 2 | `crates/timeline-editing/src/transition.rs:12` | - |  |
| `impl TransitionSpan::checked_end` | 1 | `crates/timeline-editing/src/transition.rs:40` | - |  |
| `transition_bounds` | 1 | `crates/timeline-editing/src/transition.rs:79` | - | Return the allowed interval and cut for the referenced clips. |
| `transition_alignment` | 4 | `crates/timeline-editing/src/transition.rs:111` | - | Infer the alignment stored or shown for a span over the supplied cut. |
| `set_transition_span` | 4 | `crates/timeline-editing/src/transition.rs:140` | - | Validate an edit before a host mutates its sequence/session. On refusal the |
| `start_for_alignment` | 6 | `crates/timeline-editing/src/transition.rs:176` | - | Compute a new start when a user changes the typed duration/alignment. The |
| `drag_transition_span` | 5 | `crates/timeline-editing/src/transition.rs:209` | graph | Compute a drag preview without changing a document. The host commits the |
| `frame` | 1 | `crates/timeline-editing/src/transition.rs:276` | - |  |
| `two_clip_context` | 1 | `crates/timeline-editing/src/transition.rs:280` | - |  |
| `span_alignment_preserves_center_start_end_custom_and_odd_frame_rules` | 0 | `crates/timeline-editing/src/transition.rs:289` | - |  |
| `valid_edit_returns_frame_exact_span_and_alignment` | 0 | `crates/timeline-editing/src/transition.rs:308` | - |  |
| `refused_edits_cover_locks_short_spans_overhangs_cut_and_overflow` | 0 | `crates/timeline-editing/src/transition.rs:321` | - |  |
| `one_sided_fades_keep_their_clip_edge` | 0 | `crates/timeline-editing/src/transition.rs:358` | graph |  |
| `duration_change_and_drag_preview_respect_cut_and_clip_bounds` | 0 | `crates/timeline-editing/src/transition.rs:390` | - |  |

# `crates/vector-surface-brush`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Categorical surface brush procedural engines: calligraphic, scatter, art spine warp,
pattern, and bristle brushes._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for ArtScale::default` | 0 | `crates/vector-surface-brush/src/art.rs:32` | - |  |
| `impl Default for ArtBrush::default` | 0 | `crates/vector-surface-brush/src/art.rs:50` | - |  |
| `default_arrow_art` | 0 | `crates/vector-surface-brush/src/art.rs:62` | - |  |
| `impl ArtBrush::expand` | 3 | `crates/vector-surface-brush/src/art.rs:79` | - |  |
| `impl ArtBrush::orient_art` | 1 | `crates/vector-surface-brush/src/art.rs:134` | 2d |  |
| `warp_path_geometry` | 2 | `crates/vector-surface-brush/src/art.rs:151` | - |  |
| `art_brush_warps_along_curve` | 0 | `crates/vector-surface-brush/src/art.rs:192` | - |  |
| `impl Default for BristleBrush::default` | 0 | `crates/vector-surface-brush/src/bristle.rs:54` | - |  |
| `impl BristleBrush::expand` | 4 | `crates/vector-surface-brush/src/bristle.rs:68` | - |  |
| `offset_track` | 4 | `crates/vector-surface-brush/src/bristle.rs:113` | - |  |
| `bristle_bundles` | 0 | `crates/vector-surface-brush/src/bristle.rs:147` | - |  |
| `impl Default for CalligraphicBrush::default` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:40` | - |  |
| `impl CalligraphicBrush::modes` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:54` | - |  |
| `impl CalligraphicBrush::uses_pressure` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:58` | - |  |
| `impl CalligraphicBrush::expand` | 5 | `crates/vector-surface-brush/src/calligraphic.rs:62` | - |  |
| `sample_pressure` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:87` | - |  |
| `randomized` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:107` | - |  |
| `pressured` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:121` | - |  |
| `tolerance` | 1 | `crates/vector-surface-brush/src/calligraphic.rs:135` | - |  |
| `impl Nib::new` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:149` | - |  |
| `impl Nib::support` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:157` | 2d |  |
| `rot` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:164` | 2d |  |
| `samples_along` | 2 | `crates/vector-surface-brush/src/calligraphic.rs:168` | 2d |  |
| `sweep` | 3 | `crates/vector-surface-brush/src/calligraphic.rs:199` | - |  |
| `nib_support_axes` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:228` | - |  |
| `calligraphic_sweep` | 0 | `crates/vector-surface-brush/src/calligraphic.rs:237` | - |  |
| `impl Default for BrushInputSample::default` | 0 | `crates/vector-surface-brush/src/lib.rs:35` | - |  |
| `expand_brush_stroke` | 4 | `crates/vector-surface-brush/src/lib.rs:67` | - | Expands any procedural brush stroke into vector geometric pieces. |
| `test_line` | 0 | `crates/vector-surface-brush/src/lib.rs:130` | - |  |
| `expand_all_five_brush_types` | 0 | `crates/vector-surface-brush/src/lib.rs:138` | - |  |
| `impl Default for PatternBrush::default` | 0 | `crates/vector-surface-brush/src/pattern.rs:39` | - |  |
| `default_stitch_tile` | 0 | `crates/vector-surface-brush/src/pattern.rs:55` | - |  |
| `impl PatternBrush::expand` | 3 | `crates/vector-surface-brush/src/pattern.rs:60` | - |  |
| `pattern_brush_repeats` | 0 | `crates/vector-surface-brush/src/pattern.rs:125` | - |  |
| `impl Default for ScatterBrush::default` | 0 | `crates/vector-surface-brush/src/scatter.rs:26` | - |  |
| `default_dot_art` | 0 | `crates/vector-surface-brush/src/scatter.rs:39` | - |  |
| `impl ScatterBrush::modes` | 1 | `crates/vector-surface-brush/src/scatter.rs:46` | - |  |
| `impl ScatterBrush::expand` | 5 | `crates/vector-surface-brush/src/scatter.rs:52` | - |  |
| `impl ScatterBrush::eval_params` | 3 | `crates/vector-surface-brush/src/scatter.rs:95` | - |  |
| `sample_pressure` | 2 | `crates/vector-surface-brush/src/scatter.rs:111` | - |  |
| `scatter_stamps` | 0 | `crates/vector-surface-brush/src/scatter.rs:136` | - |  |
| `unit` | 1 | `crates/vector-surface-brush/src/track.rs:23` | 2d |  |
| `normal` | 1 | `crates/vector-surface-brush/src/track.rs:29` | 3d, 2d | Left-hand normal in y-down document space (for a path running +x it points +y). |
| `impl Track::new` | 2 | `crates/vector-surface-brush/src/track.rs:34` | - |  |
| `impl Track::len` | 1 | `crates/vector-surface-brush/src/track.rs:84` | - |  |
| `impl Track::is_empty` | 1 | `crates/vector-surface-brush/src/track.rs:88` | - |  |
| `impl Track::segments` | 1 | `crates/vector-surface-brush/src/track.rs:92` | - |  |
| `impl Track::in_out` | 2 | `crates/vector-surface-brush/src/track.rs:97` | 3d, 2d | Incoming and outgoing unit directions at vertex `i`. |
| `impl Track::corner_positions` | 1 | `crates/vector-surface-brush/src/track.rs:117` | - | Arc-length positions of the corner vertices. |
| `impl Track::frame` | 2 | `crates/vector-surface-brush/src/track.rs:123` | 2d | Point and unit tangent at arc length `s`. |
| `impl Track::map` | 3 | `crates/vector-surface-brush/src/track.rs:148` | 3d | The point at arc length `s`, offset `d` along the left normal. |
| `tracks` | 2 | `crates/vector-surface-brush/src/track.rs:155` | - | Flatten `bp` into tracks (one per subpath) with `tol` accuracy. |
| `impl Rng::new` | 1 | `crates/vector-surface-brush/src/track.rs:185` | - |  |
| `impl Rng::next_u64` | 1 | `crates/vector-surface-brush/src/track.rs:188` | - |  |
| `impl Rng::unit` | 1 | `crates/vector-surface-brush/src/track.rs:196` | - | Uniform in [0, 1). |
| `impl Rng::range` | 3 | `crates/vector-surface-brush/src/track.rs:200` | - | Uniform in [a, b]. |
| `seed_of` | 2 | `crates/vector-surface-brush/src/track.rs:207` | - | A stable seed for a path (FNV-1a over its coordinates' bits, quantised to 1/64 pt). |
| `straight_track_frames` | 0 | `crates/vector-surface-brush/src/track.rs:236` | - |  |

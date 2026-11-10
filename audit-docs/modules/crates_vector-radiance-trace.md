# `crates/vector-radiance-trace`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Vector Radiance Trace: raster -> vector image tracing.

A clean-room image tracer:
1. **Quantise**: Black and White threshold, Grayscale k-means, or Color median-cut/k-means.
2. **Denoise**: 4-connected component filtering and pinhole closing.
3. **Follow boundaries**: pixel-crack contour walking into closed loops.
4. **Polygon**: Douglas-Peucker reduction removing staircases.
5. **Bézier fit**: Philip J. Schneider least-squares cubic Bézier fitting with corner preservation.
6. **Centerline**: Zhang-Suen morphological thinning and skeleton graph extraction for stroked paths._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `lines` | 6 | `crates/vector-radiance-trace/src/centerline.rs:25` | - | The lines of colour `label` in `labels` (`w` x `h`, row-major) no wider than `max_width` |
| `neighbours` | 3 | `crates/vector-radiance-trace/src/centerline.rs:73` | - |  |
| `distance` | 3 | `crates/vector-radiance-trace/src/centerline.rs:92` | - |  |
| `impl Grid::ring` | 2 | `crates/vector-radiance-trace/src/centerline.rs:152` | - |  |
| `impl Grid::interior` | 1 | `crates/vector-radiance-trace/src/centerline.rs:166` | - |  |
| `impl Grid::neighbours` | 2 | `crates/vector-radiance-trace/src/centerline.rs:170` | - |  |
| `impl Grid::degree` | 2 | `crates/vector-radiance-trace/src/centerline.rs:185` | - |  |
| `impl Grid::centre` | 2 | `crates/vector-radiance-trace/src/centerline.rs:188` | - |  |
| `thin` | 1 | `crates/vector-radiance-trace/src/centerline.rs:196` | - |  |
| `impl Run::length` | 1 | `crates/vector-radiance-trace/src/centerline.rs:265` | - |  |
| `trace_line` | 4 | `crates/vector-radiance-trace/src/centerline.rs:278` | - |  |
| `prune` | 3 | `crates/vector-radiance-trace/src/centerline.rs:407` | - |  |
| `join` | 2 | `crates/vector-radiance-trace/src/centerline.rs:431` | - |  |
| `right_of` | 1 | `crates/vector-radiance-trace/src/contour.rs:32` | - |  |
| `step` | 1 | `crates/vector-radiance-trace/src/contour.rs:41` | - |  |
| `label` | 3 | `crates/vector-radiance-trace/src/contour.rs:51` | 2d | 4-connected component labels (`u32::MAX` outside) and per-component pixel counts. |
| `trace_mask` | 3 | `crates/vector-radiance-trace/src/contour.rs:91` | - | Trace every component of `mask` (row-major, `w x h`). |
| `seg_dist` | 3 | `crates/vector-radiance-trace/src/fit.rs:19` | - |  |
| `rdp_slice` | 5 | `crates/vector-radiance-trace/src/fit.rs:30` | - | Douglas-Peucker on `pts[a..=b]`, marking kept indices. |
| `polygon` | 2 | `crates/vector-radiance-trace/src/fit.rs:53` | computational geometry, 2d | The polygon approximating a closed loop of pixel-corner vertices within `tol`. |
| `dejag` | 1 | `crates/vector-radiance-trace/src/fit.rs:85` | 2d | Replace pixel-staircase jogs by their midpoints. |
| `fit_loop` | 2 | `crates/vector-radiance-trace/src/fit.rs:115` | 2d | Fit one loop into a closed Bézier subpath (pixel coordinates). |
| `fit_polyline` | 3 | `crates/vector-radiance-trace/src/fit.rs:140` | - | Fit a centre line through `pts` into a Bézier subpath. |
| `snap_to_lines` | 2 | `crates/vector-radiance-trace/src/fit.rs:188` | - | Retract the handles of curves that stay within `tol` of their chord. |
| `impl Default for SimplifyOptions::default` | 0 | `crates/vector-radiance-trace/src/fit.rs:221` | - |  |
| `impl Seg::line` | 2 | `crates/vector-radiance-trace/src/fit.rs:237` | - |  |
| `segs_to_subpath` | 2 | `crates/vector-radiance-trace/src/fit.rs:245` | - |  |
| `turn_deg` | 2 | `crates/vector-radiance-trace/src/fit.rs:275` | - |  |
| `no_worse` | 2 | `crates/vector-radiance-trace/src/fit.rs:280` | - |  |
| `subpath_segs` | 1 | `crates/vector-radiance-trace/src/fit.rs:288` | - |  |
| `simplify_with` | 2 | `crates/vector-radiance-trace/src/fit.rs:297` | - |  |
| `rdp_points` | 2 | `crates/vector-radiance-trace/src/fit.rs:360` | - |  |
| `unit` | 1 | `crates/vector-radiance-trace/src/fit.rs:398` | 2d |  |
| `start_tangent` | 1 | `crates/vector-radiance-trace/src/fit.rs:407` | 2d |  |
| `end_tangent` | 1 | `crates/vector-radiance-trace/src/fit.rs:414` | 2d |  |
| `fit_cubics` | 4 | `crates/vector-radiance-trace/src/fit.rs:421` | 2d |  |
| `dedup` | 1 | `crates/vector-radiance-trace/src/fit.rs:431` | - |  |
| `fit_rec` | 6 | `crates/vector-radiance-trace/src/fit.rs:441` | 2d |  |
| `fit_single` | 3 | `crates/vector-radiance-trace/src/fit.rs:459` | 2d |  |
| `fit_single_from` | 4 | `crates/vector-radiance-trace/src/fit.rs:463` | 2d |  |
| `chord_params` | 1 | `crates/vector-radiance-trace/src/fit.rs:488` | - |  |
| `bern` | 1 | `crates/vector-radiance-trace/src/fit.rs:504` | - |  |
| `generate` | 4 | `crates/vector-radiance-trace/src/fit.rs:509` | 2d |  |
| `max_error` | 3 | `crates/vector-radiance-trace/src/fit.rs:544` | - |  |
| `newton` | 3 | `crates/vector-radiance-trace/src/fit.rs:557` | - |  |
| `is_straight` | 2 | `crates/vector-radiance-trace/src/fit.rs:576` | - |  |
| `sample` | 4 | `crates/vector-radiance-trace/src/fit.rs:590` | - |  |
| `impl Raster::new` | 3 | `crates/vector-radiance-trace/src/lib.rs:56` | - |  |
| `impl Raster::from_fn` | 3 | `crates/vector-radiance-trace/src/lib.rs:65` | - |  |
| `impl Raster::pixel` | 3 | `crates/vector-radiance-trace/src/lib.rs:75` | 2d |  |
| `impl Default for TraceParams::default` | 0 | `crates/vector-radiance-trace/src/lib.rs:150` | - |  |
| `impl TraceParams::polygon_tolerance` | 1 | `crates/vector-radiance-trace/src/lib.rs:174` | computational geometry |  |
| `impl TraceParams::fit_tolerance` | 1 | `crates/vector-radiance-trace/src/lib.rs:179` | - |  |
| `impl TraceParams::corner_angle` | 1 | `crates/vector-radiance-trace/src/lib.rs:184` | - |  |
| `impl TracePreset::to_options` | 1 | `crates/vector-radiance-trace/src/lib.rs:208` | - |  |
| `preset` | 1 | `crates/vector-radiance-trace/src/lib.rs:241` | - |  |
| `presets` | 0 | `crates/vector-radiance-trace/src/lib.rs:292` | - |  |
| `impl TraceResult::anchor_count` | 1 | `crates/vector-radiance-trace/src/lib.rs:316` | - |  |
| `is_white` | 1 | `crates/vector-radiance-trace/src/lib.rs:321` | - |  |
| `trace_rgba` | 4 | `crates/vector-radiance-trace/src/lib.rs:326` | - | High-level function tracing raw RGBA8 slice buffer into vector geometry. |
| `trace` | 2 | `crates/vector-radiance-trace/src/lib.rs:347` | - | Trace `img` with `params`. |
| `trace_within` | 3 | `crates/vector-radiance-trace/src/lib.rs:352` | - | Trace `img` with `params`, bounded by `max_anchors`. |
| `impl Layers<'_>::layer` | 3 | `crates/vector-radiance-trace/src/lib.rs:423` | - |  |
| `parallel_map` | 2 | `crates/vector-radiance-trace/src/lib.rs:501` | - |  |
| `parallel_map` | 2 | `crates/vector-radiance-trace/src/lib.rs:533` | - |  |
| `mosaic` | 3 | `crates/vector-radiance-trace/src/mosaic.rs:7` | - | Average colour (straight RGBA) of each of `cols` x `rows` tiles covering `img`, row by row. |
| `averages_each_tile` | 0 | `crates/vector-radiance-trace/src/mosaic.rs:50` | - |  |
| `luma` | 1 | `crates/vector-radiance-trace/src/quantize.rs:16` | - |  |
| `quantize` | 2 | `crates/vector-radiance-trace/src/quantize.rs:21` | - | Reduce `img` to a palette according to `params.mode`. |
| `compact` | 2 | `crates/vector-radiance-trace/src/quantize.rs:110` | - | Drop unused palette entries and merge duplicates. |
| `impl Bin::mean` | 1 | `crates/vector-radiance-trace/src/quantize.rs:149` | - |  |
| `key` | 3 | `crates/vector-radiance-trace/src/quantize.rs:159` | - |  |
| `nearest_1d` | 2 | `crates/vector-radiance-trace/src/quantize.rs:163` | - |  |
| `kmeans_1d` | 2 | `crates/vector-radiance-trace/src/quantize.rs:175` | - | 1-D k-means over a 256-bin histogram (Lloyd iterations, evenly spaced start). |
| `dist2` | 2 | `crates/vector-radiance-trace/src/quantize.rs:212` | - |  |
| `nearest_rgb` | 2 | `crates/vector-radiance-trace/src/quantize.rs:217` | - |  |
| `color_palette` | 2 | `crates/vector-radiance-trace/src/quantize.rs:235` | - | Color mode's palette for the histogram bins `used`, as `params.palette` asks. |
| `flat_colors` | 2 | `crates/vector-radiance-trace/src/quantize.rs:258` | - |  |
| `library_colors` | 3 | `crates/vector-radiance-trace/src/quantize.rs:288` | - |  |
| `centers` | 1 | `crates/vector-radiance-trace/src/quantize.rs:317` | - |  |
| `kmeans_rgb` | 2 | `crates/vector-radiance-trace/src/quantize.rs:321` | - |  |
| `median_cut` | 3 | `crates/vector-radiance-trace/src/quantize.rs:325` | - |  |
| `refine` | 2 | `crates/vector-radiance-trace/src/quantize.rs:398` | - |  |
| `denoise` | 4 | `crates/vector-radiance-trace/src/quantize.rs:430` | - | Merge 4-connected same-label components smaller than `min_area` pixels into their most common |
| `bw` | 0 | `crates/vector-radiance-trace/src/tests.rs:7` | - |  |
| `disc` | 4 | `crates/vector-radiance-trace/src/tests.rs:14` | - |  |
| `net_area` | 1 | `crates/vector-radiance-trace/src/tests.rs:25` | - |  |
| `black_disc_is_one_closed_path_with_disc_area` | 0 | `crates/vector-radiance-trace/src/tests.rs:30` | - |  |
| `small_disc_area_within_tolerance` | 0 | `crates/vector-radiance-trace/src/tests.rs:45` | - |  |
| `ring_has_outer_and_hole` | 0 | `crates/vector-radiance-trace/src/tests.rs:55` | - |  |
| `stripes_give_one_path_per_stripe` | 0 | `crates/vector-radiance-trace/src/tests.rs:75` | - |  |
| `two_colour_stripes_without_ignore_white_trace_both_colours` | 0 | `crates/vector-radiance-trace/src/tests.rs:93` | - |  |
| `noise_removal_drops_speckles` | 0 | `crates/vector-radiance-trace/src/tests.rs:109` | - |  |
| `noise_removal_fills_pinholes` | 0 | `crates/vector-radiance-trace/src/tests.rs:127` | - |  |
| `diagonal_pixels_stay_separate` | 0 | `crates/vector-radiance-trace/src/tests.rs:143` | - |  |
| `contour_loops_are_consistent` | 0 | `crates/vector-radiance-trace/src/tests.rs:156` | - |  |
| `color_mode_palette_size_matches_request` | 0 | `crates/vector-radiance-trace/src/tests.rs:167` | - |  |
| `test_trace_rgba_direct` | 0 | `crates/vector-radiance-trace/src/tests.rs:198` | - |  |
| `test_preset_conversion` | 0 | `crates/vector-radiance-trace/src/tests.rs:218` | - |  |

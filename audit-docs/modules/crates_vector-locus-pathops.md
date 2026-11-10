# `crates/vector-locus-pathops`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Vector effects: distortion, 15 warp envelopes, and scribble hatching._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `diag` | 1 | `crates/vector-locus-pathops/src/effects/distort.rs:10` | 2d |  |
| `normal_at` | 2 | `crates/vector-locus-pathops/src/effects/distort.rs:14` | 3d, 2d |  |
| `free_distort` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:31` | computational geometry, 2d | Free Distort: bilinear mapping of bounding box onto four corners. |
| `pucker_bloat` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:63` | 2d | Pucker & Bloat: anchors move toward/away from center, handles move opposite. |
| `roughen` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:89` | 2d | Roughen: resamples segments and jitters points. |
| `tweak` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:137` | 2d | Tweak: random displacement of anchors and/or control points. |
| `twist` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:173` | 2d | Twist: rotation strongest at center, fading to zero at bounding circle. |
| `zig_zag` | 3 | `crates/vector-locus-pathops/src/effects/distort.rs:192` | 3d, 2d | Zig Zag: ridges extra points per segment, alternately displaced along normal. |
| `pucker_bloat_alters_path` | 0 | `crates/vector-locus-pathops/src/effects/distort.rs:262` | - |  |
| `twist_alters_coordinates` | 0 | `crates/vector-locus-pathops/src/effects/distort.rs:270` | - |  |
| `flatten` | 1 | `crates/vector-locus-pathops/src/effects/scribble.rs:10` | - |  |
| `scribble` | 3 | `crates/vector-locus-pathops/src/effects/scribble.rs:32` | 2d | Scribble: generates zigzag hatching across a shape. |
| `scribble_generates_hatching` | 0 | `crates/vector-locus-pathops/src/effects/scribble.rs:122` | - |  |
| `num` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:6` | - |  |
| `flag` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:24` | - |  |
| `text` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:32` | - |  |
| `seed` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:36` | - |  |
| `smooth_points` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:40` | - |  |
| `mean_size` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:44` | 2d |  |
| `noise` | 4 | `crates/vector-locus-pathops/src/effects/util.rs:49` | - | Deterministic hash noise in [-1, 1] using splitmix64. |
| `seg_cubic` | 2 | `crates/vector-locus-pathops/src/effects/util.rs:61` | - |  |
| `poly_len` | 1 | `crates/vector-locus-pathops/src/effects/util.rs:70` | - |  |
| `catmull_rom` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:75` | - | Smooth subpath through points using Catmull-Rom tangents. |
| `map_nonlinear` | 3 | `crates/vector-locus-pathops/src/effects/util.rs:108` | numerics | Non-linear path mapping subdividing segments into pieces of at most `max_piece` length. |
| `impl WarpStyle::from_id` | 1 | `crates/vector-locus-pathops/src/effects/warp.rs:34` | - |  |
| `warp_point` | 6 | `crates/vector-locus-pathops/src/effects/warp.rs:59` | - | The warp map on normalized coordinates (x, y) in [-1, 1]^2: |
| `warp` | 4 | `crates/vector-locus-pathops/src/effects/warp.rs:116` | computational geometry, 2d | Apply a warp envelope to `path` within bounding box `b`. |
| `all_15_warp_presets_evaluate_without_nan` | 0 | `crates/vector-locus-pathops/src/effects/warp.rs:146` | - |  |
| `warp_path_transforms_rectangle` | 0 | `crates/vector-locus-pathops/src/effects/warp.rs:173` | - |  |
| `full_pathops_pipeline_smoke` | 0 | `crates/vector-locus-pathops/src/lib.rs:39` | - |  |
| `impl BoolOp::apply` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:29` | - |  |
| `polygon_area` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:40` | computational geometry | Signed area of a polyline ring. |
| `area` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:53` | - | Total signed area of a path. |
| `point_in_polygon` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:67` | computational geometry | Point-in-polygon test (winding number test). |
| `is_left` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:83` | - |  |
| `line_intersection` | 4 | `crates/vector-locus-pathops/src/pathops/boolean.rs:88` | computational geometry | Line segment intersection: returns parameter t in [0, 1] for segment (p0, p1) and u in [0, 1] for (p2, p3). |
| `flatten_subpath` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:106` | - | Flatten a subpath into an ordered sequence of points. |
| `clip_polygon` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:122` | computational geometry | Greiner-Hormann polygon clipping between two simple polygons. |
| `rings_to_path` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:423` | computational geometry | Convert clipped polygon rings into smooth cubic Bézier subpaths. |
| `boolean` | 3 | `crates/vector-locus-pathops/src/pathops/boolean.rs:496` | - | Execute a binary boolean operation on two paths. |
| `unite_all` | 1 | `crates/vector-locus-pathops/src/pathops/boolean.rs:537` | - | Execute N-ary union of all input paths. |
| `normalize` | 2 | `crates/vector-locus-pathops/src/pathops/boolean.rs:549` | - | Normalize path by removing degenerate slivers. |
| `boolean_union_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:565` | - |  |
| `boolean_intersect_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:575` | - |  |
| `boolean_difference_rectangles` | 0 | `crates/vector-locus-pathops/src/pathops/boolean.rs:585` | - |  |
| `impl Default for SimplifyOptions::default` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:23` | - |  |
| `simplify` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:33` | - | Simplify with default corner threshold (30°). |
| `impl Seg::line` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:44` | - |  |
| `subpath_segs` | 1 | `crates/vector-locus-pathops/src/pathops/edit.rs:52` | - |  |
| `segs_to_subpath` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:56` | - |  |
| `turn_deg` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:71` | - |  |
| `no_worse` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:76` | - |  |
| `simplify_with` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:85` | - | Least-squares Bézier refit keeping corners sharper than the threshold. |
| `rdp` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:144` | - |  |
| `smooth` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:173` | - | Smooth handles towards Catmull-Rom tangents. |
| `remove_redundant_points` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:204` | - | Remove redundant points along straight edges and collinear joints. |
| `merge_pair` | 3 | `crates/vector-locus-pathops/src/pathops/edit.rs:242` | - |  |
| `add_anchor_points` | 1 | `crates/vector-locus-pathops/src/pathops/edit.rs:264` | - | Add an anchor point at the midpoint of every segment. |
| `average` | 3 | `crates/vector-locus-pathops/src/pathops/edit.rs:283` | - | Average the position of selected anchors along `axis`. |
| `join` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:307` | - | Join open subpaths nearest-endpoint-first. |
| `split_into_grid` | 4 | `crates/vector-locus-pathops/src/pathops/edit.rs:387` | 2d | Split a rectangular boundary into a regular grid of paths. |
| `end_point` | 2 | `crates/vector-locus-pathops/src/pathops/edit.rs:410` | - |  |
| `add_anchor_points_doubles_segments` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:425` | - |  |
| `split_into_grid_generates_cells` | 0 | `crates/vector-locus-pathops/src/pathops/edit.rs:433` | - |  |
| `unit` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:11` | 2d | Unit vector or `None` if nearly zero. |
| `start_tangent` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:21` | 2d | Direction the curve leaves `p0` (robust to retracted handles). |
| `end_tangent` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:29` | 2d | Direction the curve arrives at `p3` (forward travel direction). |
| `fit_cubics` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:37` | 2d | Fit a chain of cubics through `pts` with maximum parametric error `tol`. |
| `dedup` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:47` | - |  |
| `fit_rec` | 6 | `crates/vector-locus-pathops/src/pathops/fit.rs:57` | 2d |  |
| `fit_single` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:76` | 2d | Best single cubic for `pts` with the given end tangents. |
| `fit_single_from` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:81` | 2d | Fit single starting from given parameter values `u`. |
| `chord_params` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:101` | - |  |
| `bern` | 1 | `crates/vector-locus-pathops/src/pathops/fit.rs:117` | - |  |
| `generate` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:122` | 2d |  |
| `max_error` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:157` | - |  |
| `newton` | 3 | `crates/vector-locus-pathops/src/pathops/fit.rs:170` | - |  |
| `is_straight` | 2 | `crates/vector-locus-pathops/src/pathops/fit.rs:190` | - | Is the cubic (within `tol`) a straight line from p0 to p3 with handles inside the chord? |
| `sample` | 4 | `crates/vector-locus-pathops/src/pathops/fit.rs:205` | - | Sample a cubic at `n+1` evenly spaced parameters. |
| `fits_a_cubic_exactly` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:217` | - |  |
| `splits_when_needed` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:227` | - |  |
| `straight_detection` | 0 | `crates/vector-locus-pathops/src/pathops/fit.rs:237` | - |  |
| `close_all` | 1 | `crates/vector-locus-pathops/src/pathops/offset.rs:14` | - |  |
| `stroke_bez` | 5 | `crates/vector-locus-pathops/src/pathops/offset.rs:36` | - |  |
| `outline_stroke` | 5 | `crates/vector-locus-pathops/src/pathops/offset.rs:42` | - | Outline stroke: generate the filled compound path representing the painted stroke of `path`. |
| `offset_path` | 4 | `crates/vector-locus-pathops/src/pathops/offset.rs:51` | - | Offset path: expands (positive) or contracts (negative) a closed path by `offset`. |
| `stroke_region` | 1 | `crates/vector-locus-pathops/src/pathops/offset.rs:65` | - | Area covered by multiple stroke outlines as a single clean compound path. |
| `outline_stroke_expands_line` | 0 | `crates/vector-locus-pathops/src/pathops/offset.rs:84` | - |  |
| `offset_path_inflates_rectangle` | 0 | `crates/vector-locus-pathops/src/pathops/offset.rs:93` | - |  |
| `impl Shape::new` | 3 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:18` | - |  |
| `pathfinder` | 2 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:56` | - | Execute a Pathfinder operation on a stack of shapes. |
| `regions` | 1 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:158` | - | Extract all distinct planar faces / regions covered by the shapes. |
| `pathfinder_unite_two_shapes` | 0 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:188` | - |  |
| `pathfinder_minus_front` | 0 | `crates/vector-locus-pathops/src/pathops/pathfinder.rs:197` | - |  |
| `interior_point` | 1 | `crates/vector-locus-pathops/src/pathops/planar.rs:12` | computational geometry | Find an interior point inside a closed simple polygon. |
| `shape_builder` | 1 | `crates/vector-locus-pathops/src/pathops/planar.rs:31` | - | Shape builder planar region extractor. |
| `cut_out` | 2 | `crates/vector-locus-pathops/src/pathops/planar.rs:57` | - | Cut out the portion of `target` that is covered by `tool`. |
| `shape_builder_extracts_faces` | 0 | `crates/vector-locus-pathops/src/pathops/planar.rs:67` | - |  |
| `cut_out_removes_overlapping_area` | 0 | `crates/vector-locus-pathops/src/pathops/planar.rs:75` | - |  |
| `impl Default for Extrude::default` | 0 | `crates/vector-locus-pathops/src/three_d/bevel.rs:34` | - |  |
| `impl Extrude::rotate` | 2 | `crates/vector-locus-pathops/src/three_d/bevel.rs:49` | - |  |
| `extrude` | 2 | `crates/vector-locus-pathops/src/three_d/bevel.rs:60` | - | Generate 3D extruded and beveled faces from a 2D path. |
| `extrude_rectangle_generates_six_faces` | 0 | `crates/vector-locus-pathops/src/three_d/bevel.rs:148` | - |  |
| `impl V3::sub` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:15` | - |  |
| `impl V3::cross` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:18` | - |  |
| `impl V3::dot` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:25` | - |  |
| `impl V3::unit` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:28` | - |  |
| `impl Default for Revolve::default` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:65` | - |  |
| `finite` | 4 | `crates/vector-locus-pathops/src/three_d/revolve.rs:84` | - |  |
| `impl Revolve::sanitized` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:93` | - |  |
| `impl Revolve::rotate` | 2 | `crates/vector-locus-pathops/src/three_d/revolve.rs:110` | - |  |
| `impl Revolve::light_direction` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:119` | - |  |
| `impl Face::path` | 1 | `crates/vector-locus-pathops/src/three_d/revolve.rs:136` | - |  |
| `revolve` | 3 | `crates/vector-locus-pathops/src/three_d/revolve.rs:150` | 2d | Evaluate a profile in document space. All subpaths share one axis from `bounds`. |
| `sample_line` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:266` | 2d |  |
| `cylinder_revolves_successfully` | 0 | `crates/vector-locus-pathops/src/three_d/revolve.rs:274` | - |  |
| `bounds` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:29` | 2d |  |
| `area` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:36` | - |  |
| `clean` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:49` | - |  |
| `overlaps` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:63` | 2d |  |
| `split` | 4 | `crates/vector-locus-pathops/src/three_d/visibility.rs:67` | - |  |
| `subtract` | 4 | `crates/vector-locus-pathops/src/three_d/visibility.rs:89` | computational geometry |  |
| `impl Index::tiles` | 2 | `crates/vector-locus-pathops/src/three_d/visibility.rs:119` | 2d |  |
| `impl Index::insert` | 3 | `crates/vector-locus-pathops/src/three_d/visibility.rs:128` | 2d |  |
| `visible_faces` | 1 | `crates/vector-locus-pathops/src/three_d/visibility.rs:144` | - | Remove covered portions of opaque faces in back-to-front paint order. |

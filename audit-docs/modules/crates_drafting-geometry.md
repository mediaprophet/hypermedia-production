# `crates/drafting-geometry`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Double-precision 2D/3D math for drafting.

Everything a drafting kernel needs below the document model: vectors and affine transforms,
bounding boxes, angles, lines, circular arcs (and polyline bulges), ellipses, NURBS splines,
intersections, closest points, offsets and tessellation. All angles are radians,
counter-clockwise from +X, as in DXF._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for Bounds2::default` | 0 | `crates/drafting-geometry/src/bounds.rs:13` | - |  |
| `impl Bounds2::new` | 2 | `crates/drafting-geometry/src/bounds.rs:21` | 2d |  |
| `impl Bounds2::from_points` | 1 | `crates/drafting-geometry/src/bounds.rs:24` | 2d |  |
| `impl Bounds2::is_empty` | 1 | `crates/drafting-geometry/src/bounds.rs:31` | - |  |
| `impl Bounds2::add` | 2 | `crates/drafting-geometry/src/bounds.rs:34` | 2d |  |
| `impl Bounds2::union` | 2 | `crates/drafting-geometry/src/bounds.rs:40` | - |  |
| `impl Bounds2::width` | 1 | `crates/drafting-geometry/src/bounds.rs:49` | - |  |
| `impl Bounds2::height` | 1 | `crates/drafting-geometry/src/bounds.rs:52` | - |  |
| `impl Bounds2::center` | 1 | `crates/drafting-geometry/src/bounds.rs:55` | 2d |  |
| `impl Bounds2::contains` | 2 | `crates/drafting-geometry/src/bounds.rs:66` | 2d |  |
| `impl Bounds2::contains_box` | 2 | `crates/drafting-geometry/src/bounds.rs:70` | - | True when `o` lies entirely inside. |
| `impl Bounds2::intersects` | 2 | `crates/drafting-geometry/src/bounds.rs:73` | - |  |
| `impl Bounds2::expand` | 2 | `crates/drafting-geometry/src/bounds.rs:76` | - |  |
| `impl Bounds2::corners` | 1 | `crates/drafting-geometry/src/bounds.rs:82` | 2d |  |
| `center_stays_finite_for_opposite_large_coordinates` | 0 | `crates/drafting-geometry/src/bounds.rs:92` | - |  |
| `impl Line::new` | 2 | `crates/drafting-geometry/src/curve.rs:13` | 2d |  |
| `impl Line::len` | 1 | `crates/drafting-geometry/src/curve.rs:16` | - |  |
| `impl Line::dir` | 1 | `crates/drafting-geometry/src/curve.rs:19` | 2d |  |
| `impl Line::mid` | 1 | `crates/drafting-geometry/src/curve.rs:22` | 2d |  |
| `impl Line::param_of` | 2 | `crates/drafting-geometry/src/curve.rs:26` | 3d, 2d | Parameter (0..1 on the segment) of the projection of `p`. |
| `impl Line::at` | 2 | `crates/drafting-geometry/src/curve.rs:31` | 2d |  |
| `impl Line::closest` | 2 | `crates/drafting-geometry/src/curve.rs:35` | computational geometry, 2d | Closest point on the segment. |
| `impl Line::project` | 2 | `crates/drafting-geometry/src/curve.rs:39` | 2d | Foot of the perpendicular on the infinite line. |
| `impl Line::dist` | 2 | `crates/drafting-geometry/src/curve.rs:42` | 2d |  |
| `impl Line::side` | 2 | `crates/drafting-geometry/src/curve.rs:46` | 2d | Signed side: > 0 left of a→b. |
| `impl Line::offset` | 2 | `crates/drafting-geometry/src/curve.rs:49` | - |  |
| `impl Arc::new` | 4 | `crates/drafting-geometry/src/curve.rs:65` | 2d |  |
| `impl Arc::sweep` | 1 | `crates/drafting-geometry/src/curve.rs:68` | - |  |
| `impl Arc::start_point` | 1 | `crates/drafting-geometry/src/curve.rs:71` | 2d |  |
| `impl Arc::end_point` | 1 | `crates/drafting-geometry/src/curve.rs:74` | 2d |  |
| `impl Arc::mid_point` | 1 | `crates/drafting-geometry/src/curve.rs:77` | 2d |  |
| `impl Arc::len` | 1 | `crates/drafting-geometry/src/curve.rs:80` | - |  |
| `impl Arc::contains_angle` | 2 | `crates/drafting-geometry/src/curve.rs:83` | - |  |
| `impl Arc::at` | 2 | `crates/drafting-geometry/src/curve.rs:87` | 2d | Point at parameter `t` in 0..1 along the sweep. |
| `impl Arc::from_3_points` | 3 | `crates/drafting-geometry/src/curve.rs:91` | 2d | Arc through three points, or `None` when collinear. |
| `impl Arc::from_start_center_end` | 3 | `crates/drafting-geometry/src/curve.rs:100` | 2d | Arc from start point, center and end point (CCW). |
| `impl Arc::closest` | 2 | `crates/drafting-geometry/src/curve.rs:103` | 2d |  |
| `impl Arc::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:113` | - |  |
| `impl Arc::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:123` | 2d |  |
| `impl Circle::new` | 2 | `crates/drafting-geometry/src/curve.rs:140` | 2d |  |
| `impl Circle::from_3_points` | 3 | `crates/drafting-geometry/src/curve.rs:143` | 2d |  |
| `impl Circle::from_2_points` | 2 | `crates/drafting-geometry/src/curve.rs:168` | 2d |  |
| `impl Circle::closest` | 2 | `crates/drafting-geometry/src/curve.rs:171` | 2d |  |
| `impl Circle::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:175` | - |  |
| `impl Circle::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:178` | 2d |  |
| `impl Circle::tangent_points` | 2 | `crates/drafting-geometry/src/curve.rs:182` | 2d | Tangent points from external point `p`. |
| `impl Circle::common_tangents` | 2 | `crates/drafting-geometry/src/curve.rs:195` | 2d | Lines tangent to both `self` and `other`, as (point on `self`, point on `other`) pairs: |
| `impl Ellipse::full` | 3 | `crates/drafting-geometry/src/curve.rs:235` | 2d |  |
| `impl Ellipse::is_full` | 1 | `crates/drafting-geometry/src/curve.rs:238` | - |  |
| `impl Ellipse::minor` | 1 | `crates/drafting-geometry/src/curve.rs:241` | 2d |  |
| `impl Ellipse::at_param` | 2 | `crates/drafting-geometry/src/curve.rs:244` | 2d |  |
| `impl Ellipse::sweep` | 1 | `crates/drafting-geometry/src/curve.rs:247` | - |  |
| `impl Ellipse::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:250` | 2d |  |
| `impl Ellipse::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:258` | - |  |
| `impl Ellipse::param_of` | 2 | `crates/drafting-geometry/src/curve.rs:272` | 2d | Parameter of the point on the ellipse nearest the direction of `p` (approximate). |
| `impl Ellipse::closest` | 2 | `crates/drafting-geometry/src/curve.rs:284` | 2d |  |
| `nearest_on_polyline` | 2 | `crates/drafting-geometry/src/curve.rs:292` | 2d | The nearest point to `p` on a polyline (through tessellated points). |
| `impl Segment::start` | 1 | `crates/drafting-geometry/src/curve.rs:318` | 2d |  |
| `impl Segment::end` | 1 | `crates/drafting-geometry/src/curve.rs:325` | 2d |  |
| `impl Segment::len` | 1 | `crates/drafting-geometry/src/curve.rs:332` | - |  |
| `impl Segment::mid` | 1 | `crates/drafting-geometry/src/curve.rs:338` | 2d |  |
| `impl Segment::closest` | 2 | `crates/drafting-geometry/src/curve.rs:344` | 2d |  |
| `impl Segment::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:350` | - |  |
| `impl Segment::tessellate` | 3 | `crates/drafting-geometry/src/curve.rs:357` | 2d | Points along the segment in its direction, including both ends. |
| `impl Segment::at` | 2 | `crates/drafting-geometry/src/curve.rs:373` | 2d | Point at parameter `t` (0..1) in segment direction. |
| `impl Segment::reversed` | 1 | `crates/drafting-geometry/src/curve.rs:380` | - |  |
| `impl Segment::tangent` | 2 | `crates/drafting-geometry/src/curve.rs:387` | 2d | Tangent direction (unit) at parameter t. |
| `impl Segment::offset` | 2 | `crates/drafting-geometry/src/curve.rs:398` | - | Offset by `d` to the left of travel direction. `None` if an arc collapses. |
| `impl PolyVertex::new` | 1 | `crates/drafting-geometry/src/curve.rs:426` | 2d |  |
| `impl PolyVertex::with_bulge` | 2 | `crates/drafting-geometry/src/curve.rs:429` | 2d |  |
| `bulge_to_arc` | 3 | `crates/drafting-geometry/src/curve.rs:435` | 2d | Convert a bulge segment to an arc. `None` for a straight (zero-bulge or degenerate) segment. |
| `arc_to_bulge` | 1 | `crates/drafting-geometry/src/curve.rs:455` | - | Bulge for an arc traversed from `a` to `b` with the given sweep (signed: + = CCW). |
| `impl Polyline::from_points` | 2 | `crates/drafting-geometry/src/curve.rs:467` | 2d |  |
| `impl Polyline::segments` | 1 | `crates/drafting-geometry/src/curve.rs:471` | - | The segments, including the closing segment when closed. |
| `impl Polyline::len` | 1 | `crates/drafting-geometry/src/curve.rs:487` | - |  |
| `impl Polyline::bounds` | 1 | `crates/drafting-geometry/src/curve.rs:490` | - |  |
| `impl Polyline::tessellate` | 2 | `crates/drafting-geometry/src/curve.rs:497` | 2d |  |
| `impl Polyline::area` | 1 | `crates/drafting-geometry/src/curve.rs:511` | - | Signed area (CCW positive) of a closed polyline including bulge areas. |
| `impl Polyline::closest` | 2 | `crates/drafting-geometry/src/curve.rs:538` | 2d |  |
| `impl Polyline::contains` | 2 | `crates/drafting-geometry/src/curve.rs:550` | 2d | True if `p` is inside a closed polyline (even-odd on the tessellation). |
| `shoelace` | 1 | `crates/drafting-geometry/src/curve.rs:556` | computational geometry, 2d | Signed polygon area (CCW positive). |
| `point_in_polygon` | 2 | `crates/drafting-geometry/src/curve.rs:576` | computational geometry, 2d | Even-odd point-in-polygon test. |
| `impl DxfEntity::layer` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:47` | - |  |
| `impl std::fmt::Display for DxfError::fmt` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:73` | - |  |
| `impl Xf::apply` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:95` | - |  |
| `impl Xf::ocs` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:104` | - |  |
| `impl GroupReader<'r, 'a>::s` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:157` | - |  |
| `impl GroupReader<'r, 'a>::f` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:160` | - |  |
| `impl GroupReader<'r, 'a>::i` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:163` | - |  |
| `impl GroupReader<'r, 'a>::pt` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:166` | - |  |
| `impl GroupReader<'r, 'a>::extrusion` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:169` | - |  |
| `impl GroupReader<'r, 'a>::layer` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:175` | - |  |
| `bulged_points` | 2 | `crates/drafting-geometry/src/dxf_reader.rs:180` | - |  |
| `parse_dxf` | 1 | `crates/drafting-geometry/src/dxf_reader.rs:216` | - | Parses a DXF ASCII string into drafting entities. |
| `impl EarthModel::code` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:23` | - |  |
| `impl EarthModel::distance_km` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:30` | - |  |
| `impl EarthModel::central_angle_degrees` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:37` | - |  |
| `haversine_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:44` | - | Great-circle distance between two (latitude, longitude) pairs in degrees. |
| `vincenty_wgs84_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:55` | - | Vincenty inverse geodesic calculation on the WGS84 ellipsoid. |
| `impl GeodesicRuler::add_point` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:146` | - |  |
| `impl GeodesicRuler::paths` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:161` | - |  |
| `impl GeodesicRuler::points` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:165` | - |  |
| `impl GeodesicRuler::point_count` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:169` | - |  |
| `impl GeodesicRuler::point` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:173` | - |  |
| `impl GeodesicRuler::begin_move` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:177` | - |  |
| `impl GeodesicRuler::move_point` | 3 | `crates/drafting-geometry/src/geodesic_ruler.rs:182` | - |  |
| `impl GeodesicRuler::end_move` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:201` | - |  |
| `impl GeodesicRuler::record` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:209` | - |  |
| `impl GeodesicRuler::can_undo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:217` | - |  |
| `impl GeodesicRuler::can_redo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:225` | - |  |
| `impl GeodesicRuler::undo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:233` | - |  |
| `impl GeodesicRuler::redo` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:242` | - |  |
| `impl GeodesicRuler::remove_point` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:252` | - |  |
| `impl GeodesicRuler::clear` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:273` | - |  |
| `impl GeodesicRuler::distance_km` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:282` | - |  |
| `impl GeodesicRuler::total_distance_km` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:297` | - |  |
| `impl GeodesicRuler::angular_distance_degrees` | 2 | `crates/drafting-geometry/src/geodesic_ruler.rs:301` | - |  |
| `impl GeodesicRuler::last_leg` | 1 | `crates/drafting-geometry/src/geodesic_ruler.rs:316` | - |  |
| `line_line_infinite` | 4 | `crates/drafting-geometry/src/intersect.rs:4` | computational geometry, 2d | Intersection of two infinite lines through (a1,a2) and (b1,b2), with parameters on each. |
| `line_line` | 2 | `crates/drafting-geometry/src/intersect.rs:17` | computational geometry, 2d | Intersection of two finite segments. |
| `line_circle` | 2 | `crates/drafting-geometry/src/intersect.rs:24` | 2d | Intersections of the infinite line through `l` with a circle. Returns points with line params. |
| `circle_circle` | 2 | `crates/drafting-geometry/src/intersect.rs:55` | 2d | Intersections of two full circles. |
| `on_arc` | 2 | `crates/drafting-geometry/src/intersect.rs:91` | 2d |  |
| `intersect_segments` | 2 | `crates/drafting-geometry/src/intersect.rs:96` | 2d | Intersections between two segments (finite). |
| `intersect_segments_ext` | 3 | `crates/drafting-geometry/src/intersect.rs:102` | 2d, graph | Intersections; with `extend` true, lines are infinite and arcs are full circles |
| `norm_angle` | 1 | `crates/drafting-geometry/src/lib.rs:42` | - | Normalize an angle to `[0, 2π)`. |
| `ccw_sweep` | 2 | `crates/drafting-geometry/src/lib.rs:51` | - | Counter-clockwise sweep from `start` to `end`, in `(0, 2π]`. |
| `angle_in_sweep` | 3 | `crates/drafting-geometry/src/lib.rs:57` | - | True when angle `a` lies on the CCW sweep from `start` to `end` (inclusive, with tolerance). |
| `deg` | 1 | `crates/drafting-geometry/src/lib.rs:63` | - |  |
| `rad` | 1 | `crates/drafting-geometry/src/lib.rs:66` | - |  |
| `arc_segments` | 3 | `crates/drafting-geometry/src/lib.rs:71` | - | Number of segments needed to approximate an arc of `radius` and `sweep` within `tol`. |
| `impl Default for Mat3::default` | 0 | `crates/drafting-geometry/src/mat.rs:17` | - |  |
| `impl Mat3::translate` | 1 | `crates/drafting-geometry/src/mat.rs:25` | 2d |  |
| `impl Mat3::scale` | 2 | `crates/drafting-geometry/src/mat.rs:28` | - |  |
| `impl Mat3::rotate` | 1 | `crates/drafting-geometry/src/mat.rs:31` | - |  |
| `impl Mat3::scale_about` | 2 | `crates/drafting-geometry/src/mat.rs:36` | 2d | Scale about `base`. |
| `impl Mat3::rotate_about` | 2 | `crates/drafting-geometry/src/mat.rs:39` | 2d |  |
| `impl Mat3::mirror` | 2 | `crates/drafting-geometry/src/mat.rs:43` | 2d | Reflection across the line through `p1`, `p2`. |
| `impl Mat3::then_before` | 2 | `crates/drafting-geometry/src/mat.rs:53` | - | `self * o`: apply `o` first, then `self`. |
| `impl Mat3::then` | 2 | `crates/drafting-geometry/src/mat.rs:64` | - | Apply `self` first, then `o`. |
| `impl Mat3::apply` | 2 | `crates/drafting-geometry/src/mat.rs:67` | 2d |  |
| `impl Mat3::apply_vec` | 2 | `crates/drafting-geometry/src/mat.rs:70` | 2d |  |
| `impl Mat3::det` | 1 | `crates/drafting-geometry/src/mat.rs:73` | - |  |
| `impl Mat3::inverse` | 1 | `crates/drafting-geometry/src/mat.rs:76` | - |  |
| `impl Mat3::scale_factor` | 1 | `crates/drafting-geometry/src/mat.rs:91` | - | Uniform scale factor (geometric mean of the axis scales). |
| `impl Mat3::rotation` | 1 | `crates/drafting-geometry/src/mat.rs:95` | - | Rotation angle of the transformed X axis. |
| `impl Mat3::is_mirroring` | 1 | `crates/drafting-geometry/src/mat.rs:98` | - |  |
| `impl Default for Mat4::default` | 0 | `crates/drafting-geometry/src/mat.rs:110` | - |  |
| `impl Mat4::mul` | 2 | `crates/drafting-geometry/src/mat.rs:117` | 3d |  |
| `impl Mat4::apply` | 2 | `crates/drafting-geometry/src/mat.rs:126` | 3d |  |
| `impl Mat4::from_mat3` | 1 | `crates/drafting-geometry/src/mat.rs:136` | 3d |  |
| `impl Mat4::ocs` | 1 | `crates/drafting-geometry/src/mat.rs:140` | 3d | The "arbitrary axis algorithm" frame for an extrusion direction (DXF OCS → WCS). |
| `key` | 3 | `crates/drafting-geometry/src/region.rs:41` | 2d |  |
| `build` | 2 | `crates/drafting-geometry/src/region.rs:55` | 2d, graph |  |
| `trace` | 4 | `crates/drafting-geometry/src/region.rs:174` | 3d, graph | Trace the face left of half-edge u→v (vertex indices), giving up after `max_steps` edges. |
| `points` | 2 | `crates/drafting-geometry/src/region.rs:206` | 2d, graph |  |
| `outer_face` | 2 | `crates/drafting-geometry/src/region.rs:211` | 3d, 2d, graph | The face around `p`, as vertex indices (counter-clockwise). |
| `components` | 1 | `crates/drafting-geometry/src/region.rs:246` | 3d, graph | Connected component id of every vertex. |
| `prune` | 1 | `crates/drafting-geometry/src/region.rs:269` | graph | The graph without dangling ends: vertices of degree one are removed until none remain, so |
| `islands` | 3 | `crates/drafting-geometry/src/region.rs:291` | 3d, 2d, graph | Outlines of the groups of curves inside the face `outer` (vertex indices into `g`). |
| `enclosing_loop` | 3 | `crates/drafting-geometry/src/region.rs:361` | 2d | The closed loop enclosing `p`, counter-clockwise, or `None`. |
| `boundary_at` | 3 | `crates/drafting-geometry/src/region.rs:367` | 2d | The closed loop enclosing `p` plus the islands inside it, or `None` when `p` isn't enclosed. |
| `square` | 3 | `crates/drafting-geometry/src/region.rs:379` | 2d |  |
| `finds_square_around_point` | 0 | `crates/drafting-geometry/src/region.rs:384` | - |  |
| `boundaries_at_large_world_coordinates_keep_distinct_vertices` | 0 | `crates/drafting-geometry/src/region.rs:390` | - |  |
| `crossing_lines_make_faces` | 0 | `crates/drafting-geometry/src/region.rs:407` | - |  |
| `overlapping_separate_lines` | 0 | `crates/drafting-geometry/src/region.rs:415` | - |  |
| `open_shape_has_no_boundary` | 0 | `crates/drafting-geometry/src/region.rs:428` | - |  |
| `innermost_of_nested` | 0 | `crates/drafting-geometry/src/region.rs:435` | - |  |
| `lines_of` | 1 | `crates/drafting-geometry/src/region.rs:443` | 2d |  |
| `island_from_separate_lines` | 0 | `crates/drafting-geometry/src/region.rs:448` | - |  |
| `island_outline_ignores_dangling_ends_and_inner_edges` | 0 | `crates/drafting-geometry/src/region.rs:470` | - |  |
| `open_curves_and_attached_lines_are_not_islands` | 0 | `crates/drafting-geometry/src/region.rs:482` | - |  |
| `nested_islands_and_outside_curves` | 0 | `crates/drafting-geometry/src/region.rs:496` | - |  |
| `hostile_input_does_not_panic` | 0 | `crates/drafting-geometry/src/region.rs:514` | - |  |
| `impl Spline::from_control` | 2 | `crates/drafting-geometry/src/spline.rs:22` | 2d | A clamped uniform spline over control points. |
| `impl Spline::from_fit_points` | 1 | `crates/drafting-geometry/src/spline.rs:31` | 2d, numerics | Cubic interpolation through fit points (chord-length parametrisation, natural-ish ends |
| `impl Spline::from_fit_points_closed` | 1 | `crates/drafting-geometry/src/spline.rs:94` | 2d | Smooth closed (periodic) cubic interpolation through fit points. Chord-length parameters |
| `impl Spline::from_fit` | 2 | `crates/drafting-geometry/src/spline.rs:154` | 2d | Interpolation through fit points, open or closed (see `from_fit_points_closed`). |
| `impl Spline::is_periodic` | 1 | `crates/drafting-geometry/src/spline.rs:159` | - | Closed with an unclamped (periodic) knot vector, as DXF flags it (group 70 bit 2). |
| `impl Spline::is_valid` | 1 | `crates/drafting-geometry/src/spline.rs:163` | - |  |
| `impl Spline::domain` | 1 | `crates/drafting-geometry/src/spline.rs:168` | - |  |
| `impl Spline::eval` | 2 | `crates/drafting-geometry/src/spline.rs:175` | 2d | Evaluate at parameter `t` (de Boor via basis functions). |
| `impl Spline::tessellate` | 2 | `crates/drafting-geometry/src/spline.rs:197` | 2d |  |
| `impl Spline::bounds` | 1 | `crates/drafting-geometry/src/spline.rs:209` | - |  |
| `clamped_uniform_knots` | 2 | `crates/drafting-geometry/src/spline.rs:214` | - |  |
| `find_span` | 4 | `crates/drafting-geometry/src/spline.rs:224` | - |  |
| `basis` | 4 | `crates/drafting-geometry/src/spline.rs:249` | - |  |
| `solve` | 2 | `crates/drafting-geometry/src/spline.rs:274` | numerics | Gaussian elimination with partial pivoting. |
| `close` | 2 | `crates/drafting-geometry/src/tests.rs:3` | - |  |
| `angles_normalize` | 0 | `crates/drafting-geometry/src/tests.rs:8` | - |  |
| `mat_inverse_roundtrip` | 0 | `crates/drafting-geometry/src/tests.rs:16` | - |  |
| `mirror_matrix` | 0 | `crates/drafting-geometry/src/tests.rs:25` | numerics |  |
| `circle_through_three_points` | 0 | `crates/drafting-geometry/src/tests.rs:32` | - |  |
| `arc_three_points_direction` | 0 | `crates/drafting-geometry/src/tests.rs:40` | - |  |
| `arc_bounds_include_quadrants` | 0 | `crates/drafting-geometry/src/tests.rs:49` | - |  |
| `bulge_semicircle` | 0 | `crates/drafting-geometry/src/tests.rs:57` | - |  |
| `bulge_quarter_arc` | 0 | `crates/drafting-geometry/src/tests.rs:70` | - |  |
| `polyline_area_and_length` | 0 | `crates/drafting-geometry/src/tests.rs:78` | - |  |
| `line_intersections` | 0 | `crates/drafting-geometry/src/tests.rs:87` | - |  |
| `line_circle_hits` | 0 | `crates/drafting-geometry/src/tests.rs:96` | - |  |
| `circle_circle_hits` | 0 | `crates/drafting-geometry/src/tests.rs:106` | - |  |
| `tiny_circle_secants_and_separated_circles` | 0 | `crates/drafting-geometry/src/tests.rs:113` | - |  |
| `huge_circle_secants_remain_finite` | 0 | `crates/drafting-geometry/src/tests.rs:125` | - |  |
| `spline_interpolates_fit_points` | 0 | `crates/drafting-geometry/src/tests.rs:135` | - |  |
| `closed_spline_is_smooth_at_the_seam` | 0 | `crates/drafting-geometry/src/tests.rs:149` | - |  |
| `ellipse_points` | 0 | `crates/drafting-geometry/src/tests.rs:172` | - |  |
| `segment_offset` | 0 | `crates/drafting-geometry/src/tests.rs:180` | - |  |
| `arc_segments_bounded` | 0 | `crates/drafting-geometry/src/tests.rs:192` | - |  |
| `ocs_identity_for_z` | 0 | `crates/drafting-geometry/src/tests.rs:199` | - |  |
| `common_tangents_of_two_circles` | 0 | `crates/drafting-geometry/src/tests.rs:207` | - |  |
| `circle_three_points_preserves_large_world_coordinates` | 0 | `crates/drafting-geometry/src/tests.rs:231` | - |  |
| `circle_three_points_accepts_small_valid_triangles` | 0 | `crates/drafting-geometry/src/tests.rs:241` | - |  |
| `ellipse_bounds_find_analytic_extrema` | 0 | `crates/drafting-geometry/src/tests.rs:251` | - |  |
| `elliptical_arc_bounds_exclude_extrema_outside_the_sweep` | 0 | `crates/drafting-geometry/src/tests.rs:265` | - |  |
| `polyline_area_is_exact_for_bulge_segments` | 0 | `crates/drafting-geometry/src/tests.rs:275` | - |  |
| `polyline_area_is_stable_far_from_the_origin` | 0 | `crates/drafting-geometry/src/tests.rs:285` | - |  |
| `shoelace_preserves_small_polygon_area_at_large_coordinates` | 0 | `crates/drafting-geometry/src/tests.rs:296` | computational geometry |  |
| `near_tangent_long_line_does_not_invent_a_circle_hit` | 0 | `crates/drafting-geometry/src/tests.rs:304` | - |  |
| `long_secant_keeps_both_intersections` | 0 | `crates/drafting-geometry/src/tests.rs:316` | - |  |
| `geodesic_ruler_measures_distances_and_supports_undo_redo` | 0 | `crates/drafting-geometry/src/tests.rs:327` | - |  |
| `dxf_reader_parses_entities_without_error` | 0 | `crates/drafting-geometry/src/tests.rs:344` | - |  |
| `impl Vec2::new` | 2 | `crates/drafting-geometry/src/vec.rs:17` | - |  |
| `impl Vec2::from_angle` | 1 | `crates/drafting-geometry/src/vec.rs:21` | - | Unit vector at `angle` radians. |
| `impl Vec2::polar` | 3 | `crates/drafting-geometry/src/vec.rs:24` | 2d |  |
| `impl Vec2::dot` | 2 | `crates/drafting-geometry/src/vec.rs:27` | 2d |  |
| `impl Vec2::cross` | 2 | `crates/drafting-geometry/src/vec.rs:31` | 2d | z of the 3D cross product. |
| `impl Vec2::len` | 1 | `crates/drafting-geometry/src/vec.rs:34` | - |  |
| `impl Vec2::len2` | 1 | `crates/drafting-geometry/src/vec.rs:37` | - |  |
| `impl Vec2::dist` | 2 | `crates/drafting-geometry/src/vec.rs:40` | 2d |  |
| `impl Vec2::angle` | 1 | `crates/drafting-geometry/src/vec.rs:43` | - |  |
| `impl Vec2::angle_to` | 2 | `crates/drafting-geometry/src/vec.rs:46` | 2d |  |
| `impl Vec2::normalized` | 1 | `crates/drafting-geometry/src/vec.rs:50` | 2d | Unit vector, or zero for a zero-length vector. |
| `impl Vec2::perp` | 1 | `crates/drafting-geometry/src/vec.rs:55` | 2d | Rotated 90° counter-clockwise. |
| `impl Vec2::rotate` | 2 | `crates/drafting-geometry/src/vec.rs:58` | 2d |  |
| `impl Vec2::rotate_about` | 3 | `crates/drafting-geometry/src/vec.rs:62` | 2d |  |
| `impl Vec2::lerp` | 3 | `crates/drafting-geometry/src/vec.rs:65` | 2d |  |
| `impl Vec2::mid` | 2 | `crates/drafting-geometry/src/vec.rs:68` | 2d |  |
| `impl Vec2::is_finite` | 1 | `crates/drafting-geometry/src/vec.rs:71` | - |  |
| `impl Vec2::near` | 3 | `crates/drafting-geometry/src/vec.rs:74` | 2d |  |
| `impl Vec2::to3` | 2 | `crates/drafting-geometry/src/vec.rs:79` | 3d |  |
| `impl Vec2::min` | 2 | `crates/drafting-geometry/src/vec.rs:82` | 2d |  |
| `impl Vec2::max` | 2 | `crates/drafting-geometry/src/vec.rs:85` | 2d |  |
| `impl Vec2::mirror` | 3 | `crates/drafting-geometry/src/vec.rs:89` | 2d | Mirror across the infinite line through `a` and `b`. |
| `impl Add for Vec2::add` | 2 | `crates/drafting-geometry/src/vec.rs:102` | 2d |  |
| `impl AddAssign for Vec2::add_assign` | 2 | `crates/drafting-geometry/src/vec.rs:107` | 2d |  |
| `impl Sub for Vec2::sub` | 2 | `crates/drafting-geometry/src/vec.rs:114` | 2d |  |
| `impl SubAssign for Vec2::sub_assign` | 2 | `crates/drafting-geometry/src/vec.rs:119` | 2d |  |
| `impl Mul<f64> for Vec2::mul` | 2 | `crates/drafting-geometry/src/vec.rs:126` | 2d |  |
| `impl Div<f64> for Vec2::div` | 2 | `crates/drafting-geometry/src/vec.rs:132` | 2d |  |
| `impl Neg for Vec2::neg` | 1 | `crates/drafting-geometry/src/vec.rs:138` | 2d |  |
| `impl Vec3::new` | 3 | `crates/drafting-geometry/src/vec.rs:154` | - |  |
| `impl Vec3::xy` | 1 | `crates/drafting-geometry/src/vec.rs:157` | 2d |  |
| `impl Vec3::dot` | 2 | `crates/drafting-geometry/src/vec.rs:160` | 3d |  |
| `impl Vec3::cross` | 2 | `crates/drafting-geometry/src/vec.rs:163` | 3d |  |
| `impl Vec3::len` | 1 | `crates/drafting-geometry/src/vec.rs:166` | - |  |
| `impl Vec3::normalized` | 1 | `crates/drafting-geometry/src/vec.rs:170` | 3d |  |
| `impl Vec3::is_finite` | 1 | `crates/drafting-geometry/src/vec.rs:174` | - |  |
| `impl Add for Vec3::add` | 2 | `crates/drafting-geometry/src/vec.rs:181` | 3d |  |
| `impl Sub for Vec3::sub` | 2 | `crates/drafting-geometry/src/vec.rs:187` | 3d |  |
| `impl Mul<f64> for Vec3::mul` | 2 | `crates/drafting-geometry/src/vec.rs:193` | 3d |  |
| `impl Neg for Vec3::neg` | 1 | `crates/drafting-geometry/src/vec.rs:199` | 3d |  |
| `near_uses_finite_nonnegative_tolerance_without_squaring` | 0 | `crates/drafting-geometry/src/vec.rs:209` | - |  |
| `large_finite_vec3_has_finite_length_and_unit_direction` | 0 | `crates/drafting-geometry/src/vec.rs:220` | 3d |  |

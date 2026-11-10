# `crates/vector-locus-geometry`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Categorical locus vector geometry kernel.

The editable path model is anchor based: a [`PathData`] is a list of [`SubPath`]s,
each a list of [`Anchor`]s with absolute in/out handle positions.

Coordinates are document points (1/72 in), y pointing down._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl ArcPath::new` | 1 | `crates/vector-locus-geometry/src/arc.rs:19` | - |  |
| `impl ArcPath::len` | 1 | `crates/vector-locus-geometry/src/arc.rs:33` | - | The path's length. |
| `impl ArcPath::is_empty` | 1 | `crates/vector-locus-geometry/src/arc.rs:38` | - | True if no segment has any length. |
| `impl ArcPath::is_closed` | 1 | `crates/vector-locus-geometry/src/arc.rs:42` | - |  |
| `impl ArcPath::at` | 2 | `crates/vector-locus-geometry/src/arc.rs:47` | 2d | Point and unit tangent at distance `s` along the path. |
| `impl ArcPath::nearest` | 2 | `crates/vector-locus-geometry/src/arc.rs:60` | - | The distance along the path of its point nearest `p`. |
| `impl ArcPath::fraction_at` | 2 | `crates/vector-locus-geometry/src/arc.rs:72` | - | Distance as a fraction (0..1) of the path's length. |
| `points_and_nearest_distances_along_a_path` | 0 | `crates/vector-locus-geometry/src/arc.rs:84` | - |  |
| `subpaths` | 1 | `crates/vector-locus-geometry/src/bez.rs:10` | - | Element ranges of the subpaths of `bp` (each with whether it is closed). |
| `segments` | 1 | `crates/vector-locus-geometry/src/bez.rs:35` | - | The segments of one subpath. |
| `tangent` | 2 | `crates/vector-locus-geometry/src/bez.rs:40` | 2d | Unit tangent of `s` at `t`, robust at ends where control points coincide with the end point. |
| `unit` | 1 | `crates/vector-locus-geometry/src/bez.rs:62` | 2d | `v` normalised, or +x when it has no length. |
| `kink` | 2 | `crates/vector-locus-geometry/src/bez.rs:68` | - | Do segments `a` and `b` (`b` following `a`) meet at a corner? |
| `subpaths_and_tangents` | 0 | `crates/vector-locus-geometry/src/bez.rs:77` | - |  |
| `impl Corner::of` | 3 | `crates/vector-locus-geometry/src/corners.rs:34` | - | The corner of anchor `i` of `sp`, whose index in the path is `index`; `None` when the |
| `impl Corner::cos` | 1 | `crates/vector-locus-geometry/src/corners.rs:51` | - | The cosine of the angle between the sides. |
| `impl Corner::sin` | 1 | `crates/vector-locus-geometry/src/corners.rs:56` | - | The sine of the angle between the sides. |
| `impl Corner::angle_deg` | 1 | `crates/vector-locus-geometry/src/corners.rs:61` | - | The angle between the sides, in degrees (90 at a rectangle's corners). |
| `impl Corner::setback` | 2 | `crates/vector-locus-geometry/src/corners.rs:66` | - | How far from the corner a round cut of radius `r` meets each side: r / tan(angle / 2). |
| `impl Corner::max_radius` | 1 | `crates/vector-locus-geometry/src/corners.rs:73` | - | The largest radius the corner draws: its cut then reaches halfway along the shorter side, |
| `impl Corner::fitted` | 2 | `crates/vector-locus-geometry/src/corners.rs:79` | - | The radius the corner is drawn with: `radius`, no larger than [`Corner::max_radius`], 0 when |
| `impl Corner::on_bisector` | 2 | `crates/vector-locus-geometry/src/corners.rs:85` | - | The point `k` along the bisector, measured in (u + v): the centre of a round cut of radius |
| `impl Corner::radius_change` | 2 | `crates/vector-locus-geometry/src/corners.rs:91` | 2d | How much the radius grows when the centre of the round cut moves by `d`: `d`'s part along |
| `impl Corner::cut` | 3 | `crates/vector-locus-geometry/src/corners.rs:97` | - | The two anchors replacing the corner when cut with `radius` (> 0, fitted): where the cut |
| `square_to` | 2 | `crates/vector-locus-geometry/src/corners.rs:118` | 2d | The unit direction square to unit `a`, on `b`'s side. |
| `arc_handle` | 1 | `crates/vector-locus-geometry/src/corners.rs:126` | - | The handle length, per unit of radius, of a cubic drawing a circular arc that spans an angle |
| `path_corners` | 1 | `crates/vector-locus-geometry/src/corners.rs:135` | - | The corners of `path` Live Corners can cut, in anchor order. |
| `cut_corners` | 3 | `crates/vector-locus-geometry/src/corners.rs:150` | - | `path` with each of its corners ([`path_corners`]) cut by the radius in `radii` and the kind in |
| `near` | 2 | `crates/vector-locus-geometry/src/corners.rs:196` | - |  |
| `a_star_has_ten_corners_cut_as_circles` | 0 | `crates/vector-locus-geometry/src/corners.rs:201` | - |  |
| `radii_stop_at_half_the_shorter_side_per_corner` | 0 | `crates/vector-locus-geometry/src/corners.rs:222` | - |  |
| `ends_curves_smooth_and_straight_anchors_are_no_corners` | 0 | `crates/vector-locus-geometry/src/corners.rs:237` | - |  |
| `kinds_share_their_ends` | 0 | `crates/vector-locus-geometry/src/corners.rs:258` | - |  |
| `fill_contains` | 3 | `crates/vector-locus-geometry/src/hit.rs:10` | - | Is `p` inside the filled area of `path` under `rule`? Open subpaths are implicitly closed. |
| `closed` | 1 | `crates/vector-locus-geometry/src/hit.rs:19` | - | `path` with each open subpath closed by a straight line back to its start. |
| `distance_to_outline` | 2 | `crates/vector-locus-geometry/src/hit.rs:57` | - | Distance from `p` to the nearest point on the path outline. |
| `stroke_contains` | 4 | `crates/vector-locus-geometry/src/hit.rs:62` | - | Is `p` within `tol` of the path's stroke of width `width`? |
| `intersects_rect` | 2 | `crates/vector-locus-geometry/src/hit.rs:68` | 2d | Does the path intersect (or lie inside) the rect? |
| `fill_hit` | 0 | `crates/vector-locus-geometry/src/hit.rs:99` | - |  |
| `stroke_hit` | 0 | `crates/vector-locus-geometry/src/hit.rs:106` | - |  |
| `rect_intersection` | 0 | `crates/vector-locus-geometry/src/hit.rs:113` | computational geometry, 2d |  |
| `union_opt` | 2 | `crates/vector-locus-geometry/src/lib.rs:48` | 2d | Union of two optional rects. |
| `normalize_deg` | 1 | `crates/vector-locus-geometry/src/lib.rs:57` | - | Normalize an angle in degrees to (-180, 180]. |
| `constrain_angle` | 2 | `crates/vector-locus-geometry/src/lib.rs:68` | 2d | Snap a vector to the nearest multiple of `step_deg`. |
| `constrain_angle_from` | 3 | `crates/vector-locus-geometry/src/lib.rs:73` | 2d | [`constrain_angle`] in steps counted from `base_deg`. |
| `rect_from_points` | 2 | `crates/vector-locus-geometry/src/lib.rs:84` | 2d | Rect from two corner points in any order. |
| `reference_point` | 2 | `crates/vector-locus-geometry/src/lib.rs:89` | 2d | The 9 reference points of a rect (row-major from top-left). |
| `angle_normalization` | 0 | `crates/vector-locus-geometry/src/lib.rs:100` | - |  |
| `constrain_45` | 0 | `crates/vector-locus-geometry/src/lib.rs:108` | - |  |
| `reference_points` | 0 | `crates/vector-locus-geometry/src/lib.rs:116` | - |  |
| `union_options` | 0 | `crates/vector-locus-geometry/src/lib.rs:124` | - |  |
| `type_aliases_work` | 0 | `crates/vector-locus-geometry/src/lib.rs:133` | - |  |
| `is_corner` | 1 | `crates/vector-locus-geometry/src/path.rs:53` | - |  |
| `impl From<AnchorRepr> for Anchor::from` | 1 | `crates/vector-locus-geometry/src/path.rs:58` | - |  |
| `impl From<Anchor> for AnchorRepr::from` | 1 | `crates/vector-locus-geometry/src/path.rs:65` | - |  |
| `impl Serialize for WirePoint::serialize` | 2 | `crates/vector-locus-geometry/src/path.rs:74` | - |  |
| `impl Deserialize<'de> for WirePoint::deserialize` | 1 | `crates/vector-locus-geometry/src/path.rs:80` | - |  |
| `impl serde::de::Visitor<'de> for V::expecting` | 2 | `crates/vector-locus-geometry/src/path.rs:84` | - |  |
| `impl serde::de::Visitor<'de> for V::visit_seq` | 2 | `crates/vector-locus-geometry/src/path.rs:87` | - |  |
| `impl serde::de::Visitor<'de> for V::visit_map` | 2 | `crates/vector-locus-geometry/src/path.rs:92` | - |  |
| `impl Anchor::corner` | 1 | `crates/vector-locus-geometry/src/path.rs:115` | - | A corner anchor without handles. |
| `impl Anchor::smooth` | 2 | `crates/vector-locus-geometry/src/path.rs:119` | - | A smooth anchor with a symmetric out handle `out` (the in handle is mirrored). |
| `impl Anchor::with_handles` | 3 | `crates/vector-locus-geometry/src/path.rs:122` | - |  |
| `impl Anchor::has_in` | 1 | `crates/vector-locus-geometry/src/path.rs:127` | - |  |
| `impl Anchor::has_out` | 1 | `crates/vector-locus-geometry/src/path.rs:130` | - |  |
| `impl Anchor::transform` | 2 | `crates/vector-locus-geometry/src/path.rs:133` | - |  |
| `impl Anchor::translate` | 2 | `crates/vector-locus-geometry/src/path.rs:136` | 2d |  |
| `impl Anchor::set_handle` | 4 | `crates/vector-locus-geometry/src/path.rs:143` | - | Place the end of the outgoing (`out`) or incoming handle at `pos`. `independent` makes the |
| `impl Anchor::reversed` | 1 | `crates/vector-locus-geometry/src/path.rs:161` | - | Swap handles (used when reversing path direction). |
| `impl Anchor::retract` | 1 | `crates/vector-locus-geometry/src/path.rs:165` | - | Remove both handles (convert to a sharp corner). |
| `is_collinear` | 3 | `crates/vector-locus-geometry/src/path.rs:172` | - |  |
| `impl SubPath::new` | 2 | `crates/vector-locus-geometry/src/path.rs:188` | - |  |
| `impl SubPath::polyline` | 2 | `crates/vector-locus-geometry/src/path.rs:192` | - | Polyline from points. |
| `impl SubPath::segment_count` | 1 | `crates/vector-locus-geometry/src/path.rs:196` | - | Number of segments (a closed path has one more, back to the start). |
| `impl SubPath::segment` | 2 | `crates/vector-locus-geometry/src/path.rs:207` | - | Segment `i` as a cubic (lines are cubics with handles on the anchors). |
| `impl SubPath::segment_is_line` | 2 | `crates/vector-locus-geometry/src/path.rs:214` | - | True if segment `i` is a straight line. |
| `impl SubPath::to_bezpath_into` | 2 | `crates/vector-locus-geometry/src/path.rs:220` | - |  |
| `impl SubPath::reverse` | 1 | `crates/vector-locus-geometry/src/path.rs:235` | - |  |
| `impl SubPath::signed_area` | 1 | `crates/vector-locus-geometry/src/path.rs:243` | - | Signed area, closing an open subpath with a line: positive when it runs clockwise on |
| `impl SubPath::insert_anchor` | 3 | `crates/vector-locus-geometry/src/path.rs:252` | - | Split segment `seg` at parameter `t`, inserting a new anchor. Returns the new anchor index. |
| `impl SubPath::smooth_anchor` | 2 | `crates/vector-locus-geometry/src/path.rs:272` | - | Make anchor `i` smooth: handles in line with its neighbours (its one neighbour at an open |
| `impl SubPath::cut_at` | 2 | `crates/vector-locus-geometry/src/path.rs:310` | - | Cut at the anchors `at`: each becomes the end of one open piece and the start of the next, |
| `impl SubPath::area` | 1 | `crates/vector-locus-geometry/src/path.rs:340` | - | Signed area via the shoelace formula on the Bézier path (positive = clockwise in y-down). |
| `impl PathData::new` | 1 | `crates/vector-locus-geometry/src/path.rs:354` | - |  |
| `impl PathData::single` | 1 | `crates/vector-locus-geometry/src/path.rs:357` | - |  |
| `impl PathData::is_empty` | 1 | `crates/vector-locus-geometry/src/path.rs:360` | - |  |
| `impl PathData::anchor_count` | 1 | `crates/vector-locus-geometry/src/path.rs:363` | - |  |
| `impl PathData::to_bezpath` | 1 | `crates/vector-locus-geometry/src/path.rs:366` | - |  |
| `impl PathData::from_bezpath` | 1 | `crates/vector-locus-geometry/src/path.rs:374` | - | Convert a kurbo path to anchors. Quadratics are elevated to cubics. |
| `impl PathData::bounds` | 1 | `crates/vector-locus-geometry/src/path.rs:435` | 2d | Tight geometric bounds (curve extrema, not control points). |
| `impl PathData::control_bounds` | 1 | `crates/vector-locus-geometry/src/path.rs:448` | 2d | Bounds including control handles (used for quick culling). |
| `impl PathData::transform` | 2 | `crates/vector-locus-geometry/src/path.rs:453` | - |  |
| `impl PathData::transformed` | 2 | `crates/vector-locus-geometry/src/path.rs:460` | - |  |
| `impl PathData::is_closed` | 1 | `crates/vector-locus-geometry/src/path.rs:465` | - |  |
| `impl PathData::nearest` | 2 | `crates/vector-locus-geometry/src/path.rs:469` | - | Nearest point on any segment: (subpath, segment, t, point, distance). |
| `impl PathData::anchors` | 1 | `crates/vector-locus-geometry/src/path.rs:485` | - | Iterate (subpath index, anchor index, anchor). |
| `impl PathData::anchor_mut` | 3 | `crates/vector-locus-geometry/src/path.rs:488` | - |  |
| `impl PathData::reverse` | 1 | `crates/vector-locus-geometry/src/path.rs:491` | - |  |
| `impl PathData::length` | 1 | `crates/vector-locus-geometry/src/path.rs:497` | - | Total length of all segments. |
| `push_curve` | 5 | `crates/vector-locus-geometry/src/path.rs:502` | - |  |
| `square` | 0 | `crates/vector-locus-geometry/src/path.rs:514` | - |  |
| `anchors_serialize_compactly_and_read_v1_maps` | 0 | `crates/vector-locus-geometry/src/path.rs:519` | - |  |
| `square_bounds_and_area` | 0 | `crates/vector-locus-geometry/src/path.rs:532` | - |  |
| `bezpath_roundtrip_polyline` | 0 | `crates/vector-locus-geometry/src/path.rs:540` | - |  |
| `bezpath_roundtrip_curves` | 0 | `crates/vector-locus-geometry/src/path.rs:551` | - |  |
| `insert_anchor_on_line_keeps_shape` | 0 | `crates/vector-locus-geometry/src/path.rs:563` | - |  |
| `insert_anchor_on_curve_keeps_shape` | 0 | `crates/vector-locus-geometry/src/path.rs:574` | - |  |
| `nearest_on_edge` | 0 | `crates/vector-locus-geometry/src/path.rs:583` | graph |  |
| `reverse_flips_area_sign` | 0 | `crates/vector-locus-geometry/src/path.rs:592` | - |  |
| `transform_moves_bounds` | 0 | `crates/vector-locus-geometry/src/path.rs:600` | - |  |
| `smooth_detection` | 0 | `crates/vector-locus-geometry/src/path.rs:606` | - |  |
| `length_of_square` | 0 | `crates/vector-locus-geometry/src/path.rs:614` | - |  |
| `serde_roundtrip` | 0 | `crates/vector-locus-geometry/src/path.rs:619` | - |  |
| `impl Plane::parse` | 1 | `crates/vector-locus-geometry/src/perspective.rs:26` | - |  |
| `impl Plane::id` | 1 | `crates/vector-locus-geometry/src/perspective.rs:36` | - |  |
| `two` | 0 | `crates/vector-locus-geometry/src/perspective.rs:54` | - |  |
| `yes` | 0 | `crates/vector-locus-geometry/src/perspective.rs:57` | - |  |
| `is_zero` | 1 | `crates/vector-locus-geometry/src/perspective.rs:60` | - |  |
| `impl PerspectiveGrid::preset` | 2 | `crates/vector-locus-geometry/src/perspective.rs:115` | computational geometry, 2d | Preset for `kind` (1, 2, or 3) fitted to a bounding box. |
| `impl PerspectiveGrid::offset` | 2 | `crates/vector-locus-geometry/src/perspective.rs:146` | 3d | Where `plane` is along its normal (points; 0: default position). |
| `impl PerspectiveGrid::set_offset` | 3 | `crates/vector-locus-geometry/src/perspective.rs:156` | 3d | Move `plane` to `offset` along its normal. |
| `impl PerspectiveGrid::validate` | 1 | `crates/vector-locus-geometry/src/perspective.rs:165` | - |  |
| `impl PerspectiveGrid::foreshortening` | 1 | `crates/vector-locus-geometry/src/perspective.rs:199` | - | Foreshortening ratios along receding axes: left, right, vertical. |
| `impl PerspectiveGrid::axis_col` | 2 | `crates/vector-locus-geometry/src/perspective.rs:208` | - | The image of `axis`'s point at infinity. |
| `impl PerspectiveGrid::axes` | 2 | `crates/vector-locus-geometry/src/perspective.rs:220` | 3d | A plane's axes: `[u, v, normal]`. |
| `impl PerspectiveGrid::homography` | 2 | `crates/vector-locus-geometry/src/perspective.rs:232` | - | The homography of `plane` at its current offset. |
| `impl PerspectiveGrid::homography_at` | 3 | `crates/vector-locus-geometry/src/perspective.rs:237` | 3d | The homography of the plane parallel to `plane` that lies `depth` points along its normal. |
| `impl PerspectiveGrid::frame_homography` | 4 | `crates/vector-locus-geometry/src/perspective.rs:243` | - | The plane spanned by axes `u` and `v` through the point `at.1` along axis `at.0`. |
| `impl PerspectiveGrid::to_page` | 3 | `crates/vector-locus-geometry/src/perspective.rs:258` | - | Project a 2D point on `plane` (in plane coordinates, points) to page coordinates. |
| `impl PerspectiveGrid::from_page` | 3 | `crates/vector-locus-geometry/src/perspective.rs:263` | - | Map a page coordinate point to plane coordinates. |
| `perspective_grid_preset_and_projections` | 0 | `crates/vector-locus-geometry/src/perspective.rs:273` | - |  |
| `plane_offsets` | 0 | `crates/vector-locus-geometry/src/perspective.rs:299` | - |  |
| `impl Homography::from_cols` | 3 | `crates/vector-locus-geometry/src/projective.rs:13` | - | From the images of (1,0,0), (0,1,0) and (0,0,1). |
| `impl Homography::from_affine` | 1 | `crates/vector-locus-geometry/src/projective.rs:18` | - | The affine map `a` as a homography. |
| `impl Homography::from_array` | 1 | `crates/vector-locus-geometry/src/projective.rs:24` | - | From nine row-major numbers; `None` unless all are finite and the map is invertible. |
| `impl Homography::to_array` | 1 | `crates/vector-locus-geometry/src/projective.rs:33` | - | The nine row-major numbers. |
| `impl Homography::apply` | 2 | `crates/vector-locus-geometry/src/projective.rs:39` | - | Map a point; `None` when it lands on or beyond the horizon (w ≤ 0). |
| `impl Homography::det` | 1 | `crates/vector-locus-geometry/src/projective.rs:50` | - |  |
| `impl Homography::inverse` | 1 | `crates/vector-locus-geometry/src/projective.rs:57` | - | Inverse. |
| `impl Homography::then_after` | 2 | `crates/vector-locus-geometry/src/projective.rs:79` | - | `self ∘ other`: `other` first. |
| `impl Homography::conjugated` | 2 | `crates/vector-locus-geometry/src/projective.rs:91` | - | `self` seen through `a`: the map `a ∘ self ∘ a⁻¹`. |
| `impl Homography::affine_at` | 2 | `crates/vector-locus-geometry/src/projective.rs:99` | - | The affine map that matches `self` to first order at `p` (its derivative there). |
| `impl Homography::map_rect_bbox` | 2 | `crates/vector-locus-geometry/src/projective.rs:109` | computational geometry, 2d | The bounding box of the image of `r`. |
| `h` | 0 | `crates/vector-locus-geometry/src/projective.rs:124` | - |  |
| `compose_invert_and_round_trip` | 0 | `crates/vector-locus-geometry/src/projective.rs:129` | - |  |
| `affine_and_conjugation` | 0 | `crates/vector-locus-geometry/src/projective.rs:146` | - |  |
| `affine_at_is_the_derivative` | 0 | `crates/vector-locus-geometry/src/projective.rs:156` | - |  |
| `rect_bbox_is_the_corners_box` | 0 | `crates/vector-locus-geometry/src/projective.rs:168` | 2d |  |
| `bbox` | 1 | `crates/vector-locus-geometry/src/recognize.rs:33` | 2d |  |
| `path_len` | 1 | `crates/vector-locus-geometry/src/recognize.rs:37` | - |  |
| `dist_to_segment` | 3 | `crates/vector-locus-geometry/src/recognize.rs:41` | - |  |
| `simplify` | 2 | `crates/vector-locus-geometry/src/recognize.rs:52` | - | Douglas–Peucker simplification with bounded stack use. |
| `simplify_freehand` | 2 | `crates/vector-locus-geometry/src/recognize.rs:91` | - | Alias for [`simplify`] for freehand strokes. |
| `resample` | 2 | `crates/vector-locus-geometry/src/recognize.rs:96` | - | Sample by distance. |
| `closed_corners` | 2 | `crates/vector-locus-geometry/src/recognize.rs:119` | - |  |
| `area` | 1 | `crates/vector-locus-geometry/src/recognize.rs:144` | - |  |
| `is_rectangle` | 3 | `crates/vector-locus-geometry/src/recognize.rs:149` | - |  |
| `right_angles` | 2 | `crates/vector-locus-geometry/src/recognize.rs:156` | - |  |
| `polygon_rotation` | 2 | `crates/vector-locus-geometry/src/recognize.rs:166` | computational geometry |  |
| `rectangle_rotation` | 1 | `crates/vector-locus-geometry/src/recognize.rs:186` | - |  |
| `ellipse_rotation` | 1 | `crates/vector-locus-geometry/src/recognize.rs:194` | - |  |
| `oriented_bounds` | 2 | `crates/vector-locus-geometry/src/recognize.rs:208` | 2d |  |
| `reversals` | 2 | `crates/vector-locus-geometry/src/recognize.rs:220` | - |  |
| `recognize` | 1 | `crates/vector-locus-geometry/src/recognize.rs:231` | - | Recognise a freehand stroke. |
| `recognize_shape` | 1 | `crates/vector-locus-geometry/src/recognize.rs:303` | - | Alias for [`recognize`]. |
| `is_polygonal` | 4 | `crates/vector-locus-geometry/src/recognize.rs:307` | - |  |
| `wobbly` | 3 | `crates/vector-locus-geometry/src/recognize.rs:323` | - |  |
| `circle` | 4 | `crates/vector-locus-geometry/src/recognize.rs:338` | - |  |
| `rough_rectangle_and_square` | 0 | `crates/vector-locus-geometry/src/recognize.rs:348` | - |  |
| `ellipse_circle_triangle_hexagon_line` | 0 | `crates/vector-locus-geometry/src/recognize.rs:358` | - |  |
| `scribble_and_noise` | 0 | `crates/vector-locus-geometry/src/recognize.rs:367` | - |  |
| `simplify_freehand_test` | 0 | `crates/vector-locus-geometry/src/recognize.rs:374` | - |  |
| `rectangle` | 1 | `crates/vector-locus-geometry/src/shapes.rs:14` | 2d | Rectangle with 4 corner anchors, clockwise from the top-left. |
| `rounded_rectangle` | 2 | `crates/vector-locus-geometry/src/shapes.rs:20` | 2d | Rounded rectangle (8 anchors). The radius is clamped to half the shorter side. |
| `impl CornerKind::next` | 1 | `crates/vector-locus-geometry/src/shapes.rs:41` | - | The kind after this one (cycling through them). |
| `max_corner_radius` | 2 | `crates/vector-locus-geometry/src/shapes.rs:51` | - | The largest corner radius a `w` × `h` rectangle draws: half its shorter side. |
| `fitted_corner_radius` | 3 | `crates/vector-locus-geometry/src/shapes.rs:56` | - | The radius a corner of a `w` × `h` rectangle is drawn with. |
| `rectangle_with_corners` | 3 | `crates/vector-locus-geometry/src/shapes.rs:62` | 2d | Rectangle whose corners (top-left, top-right, bottom-right, bottom-left, y down) each have |
| `ellipse` | 1 | `crates/vector-locus-geometry/src/shapes.rs:67` | 2d | Ellipse inscribed in `r` with 4 smooth anchors (left, top, right, bottom). |
| `ellipse_pie` | 3 | `crates/vector-locus-geometry/src/shapes.rs:86` | 2d | The ellipse inscribed in `r` cut as a pie: from `start` to `end` degrees, measured |
| `polygon` | 4 | `crates/vector-locus-geometry/src/shapes.rs:126` | computational geometry, 3d | Regular polygon with `sides` (≥ 3), first vertex straight up from the centre. |
| `star` | 5 | `crates/vector-locus-geometry/src/shapes.rs:139` | - | Star with `points` tips (≥ 2), outer radius `r1`, inner radius `r2`, first tip up. |
| `straight_star_inner_radius` | 2 | `crates/vector-locus-geometry/src/shapes.rs:153` | - | Inner radius of a "straight-armed" star (shoulders aligned). |
| `line` | 2 | `crates/vector-locus-geometry/src/shapes.rs:159` | - | Open straight line. |
| `arc` | 4 | `crates/vector-locus-geometry/src/shapes.rs:164` | - | Quarter-ellipse arc from `a` to `b`. |
| `spiral` | 5 | `crates/vector-locus-geometry/src/shapes.rs:179` | - | Archimedean-style spiral (decay-based, `segments` quarter turns). |
| `rectangular_grid` | 4 | `crates/vector-locus-geometry/src/shapes.rs:201` | 2d | Rectangular grid as separate open lines (plus an optional frame rectangle). |
| `polar_grid` | 3 | `crates/vector-locus-geometry/src/shapes.rs:219` | 2d | Polar grid: concentric ellipses plus radial dividers. |
| `corner_sources` | 2 | `crates/vector-locus-geometry/src/shapes.rs:238` | 2d |  |
| `a_pie_is_its_arc_closed_through_the_centre` | 0 | `crates/vector-locus-geometry/src/shapes.rs:243` | - |  |
| `rect_is_4_corners_clockwise` | 0 | `crates/vector-locus-geometry/src/shapes.rs:259` | 2d |  |
| `rect_normalizes_negative` | 0 | `crates/vector-locus-geometry/src/shapes.rs:267` | 2d |  |
| `ellipse_bounds_and_area` | 0 | `crates/vector-locus-geometry/src/shapes.rs:273` | - |  |
| `rounded_rect_clamps` | 0 | `crates/vector-locus-geometry/src/shapes.rs:282` | 2d |  |
| `each_corner_takes_its_own_radius_and_kind` | 0 | `crates/vector-locus-geometry/src/shapes.rs:290` | - |  |
| `polygon_first_vertex_up` | 0 | `crates/vector-locus-geometry/src/shapes.rs:307` | computational geometry, 3d |  |
| `star_alternates_radii` | 0 | `crates/vector-locus-geometry/src/shapes.rs:315` | - |  |
| `straight_star_five_points` | 0 | `crates/vector-locus-geometry/src/shapes.rs:323` | - |  |
| `spiral_has_segments` | 0 | `crates/vector-locus-geometry/src/shapes.rs:329` | - |  |
| `grids` | 0 | `crates/vector-locus-geometry/src/shapes.rs:336` | - |  |
| `arc_open_and_closed` | 0 | `crates/vector-locus-geometry/src/shapes.rs:342` | - |  |
| `snap_to_grid` | 3 | `crates/vector-locus-geometry/src/snap.rs:6` | - | Snap a value to a grid with the given spacing and origin. |
| `snap_point_to_grid` | 2 | `crates/vector-locus-geometry/src/snap.rs:14` | - | Snap a 2D point to an orthogonal grid with uniform spacing. |
| `closest` | 3 | `crates/vector-locus-geometry/src/snap.rs:26` | - | Find the closest target within `tol` to `v`. Returns `(target, delta)`. |
| `snap_point` | 3 | `crates/vector-locus-geometry/src/snap.rs:32` | - | Snap a point `p` against a collection of candidate points within tolerance `tol`. |
| `grid` | 0 | `crates/vector-locus-geometry/src/snap.rs:52` | - |  |
| `closest_target` | 0 | `crates/vector-locus-geometry/src/snap.rs:61` | - |  |
| `snap_point_candidates` | 0 | `crates/vector-locus-geometry/src/snap.rs:67` | - |  |
| `impl Unit::points` | 1 | `crates/vector-locus-geometry/src/units.rs:36` | - | Points per unit. |
| `impl Unit::suffix` | 1 | `crates/vector-locus-geometry/src/units.rs:49` | - |  |
| `impl Unit::label` | 1 | `crates/vector-locus-geometry/src/units.rs:63` | - |  |
| `impl Unit::ruler_step` | 2 | `crates/vector-locus-geometry/src/units.rs:79` | - | The rulers' label step at `zoom` (screen pixels per point), in this unit. |
| `impl Unit::snap_to_ruler_tick` | 3 | `crates/vector-locus-geometry/src/units.rs:87` | - | Snap `v` (points) to the nearest ruler tick at `zoom`. |
| `impl Unit::from_pt` | 2 | `crates/vector-locus-geometry/src/units.rs:95` | - |  |
| `impl Unit::to_pt` | 2 | `crates/vector-locus-geometry/src/units.rs:99` | - |  |
| `impl Unit::format` | 2 | `crates/vector-locus-geometry/src/units.rs:104` | - | Format a point value in this unit (e.g. `12.5 pt`). |
| `impl Unit::number` | 2 | `crates/vector-locus-geometry/src/units.rs:109` | - | Format without suffix. |
| `impl Unit::parse` | 2 | `crates/vector-locus-geometry/src/units.rs:117` | - | Parse a length string in this unit with optional arithmetic (`+ - * /`). |
| `impl Unit::key` | 1 | `crates/vector-locus-geometry/src/units.rs:122` | - | Canonical key name. |
| `impl Unit::named` | 1 | `crates/vector-locus-geometry/src/units.rs:138` | - | Find a unit by label, suffix, or key. |
| `impl Unit::readout` | 2 | `crates/vector-locus-geometry/src/units.rs:144` | - |  |
| `parse_measure` | 2 | `crates/vector-locus-geometry/src/units.rs:149` | - |  |
| `points_conversion` | 0 | `crates/vector-locus-geometry/src/units.rs:193` | - |  |
| `parse_with_units_and_arithmetic` | 0 | `crates/vector-locus-geometry/src/units.rs:201` | - |  |
| `formatting` | 0 | `crates/vector-locus-geometry/src/units.rs:211` | - |  |
| `unit_named` | 0 | `crates/vector-locus-geometry/src/units.rs:218` | - |  |

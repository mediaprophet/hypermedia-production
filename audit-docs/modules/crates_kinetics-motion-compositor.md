# `crates/kinetics-motion-compositor`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Keyframe structures, temporal ease velocity graphs, and spatial paths._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `bez` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:4` | - | 1D cubic Bézier point evaluation: B(u) for u in [0, 1]. |
| `bez_d` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:10` | - | Derivative of 1D cubic Bézier: B'(u). |
| `solve_u` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:16` | numerics | Solve `bez(x0, x1, x2, x3, u) = x` for parameter `u` in [0, 1] using Newton-Raphson with bisection fallback. |
| `ease_progress` | 5 | `crates/kinetics-motion-compositor/src/keyframe/bezier.rs:59` | - | Compute temporal progress in [0, 1] for a segment given normalized speeds and influences. |
| `impl Default for Ease::default` | 0 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:29` | - |  |
| `impl Value::as_f64` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:54` | - |  |
| `impl Value::lerp` | 3 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:63` | - |  |
| `impl Keyframe::new` | 2 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:102` | - |  |
| `impl Keyframe::hold` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:118` | - |  |
| `impl Keyframe::eased` | 1 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:124` | - |  |
| `impl KeyframeTrack::evaluate` | 2 | `crates/kinetics-motion-compositor/src/keyframe/mod.rs:140` | - |  |
| `eval_spatial_bezier` | 5 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:6` | - | Evaluate a 3D cubic Bézier curve at parameter `u` in [0, 1]. |
| `distance_3d` | 2 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:15` | - | Distance between two 3D points. |
| `impl ArcLengthTable::build` | 5 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:32` | - | Build an arc-length table for a 3D spatial Bézier segment. |
| `impl ArcLengthTable::u_at_progress` | 2 | `crates/kinetics-motion-compositor/src/keyframe/spatial.rs:57` | - | Map arc-length progress fraction s in [0, 1] to Bézier parameter `u` in [0, 1]. |
| `test_cubic_bezier_solve_and_ease_progress` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:45` | numerics |  |
| `test_spatial_arc_length_table` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:56` | - |  |
| `test_keyframe_track_evaluation` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:72` | - |  |
| `test_ncc_point_tracker` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:88` | - |  |
| `test_corner_pin_homography` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:109` | - |  |
| `test_lottie_parsing_and_evaluation` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:121` | - |  |
| `test_kinetic_text_animators` | 0 | `crates/kinetics-motion-compositor/src/lib.rs:157` | - |  |
| `eval_scalar_prop` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:18` | - | Evaluate a scalar property at frame `f`. |
| `extract_val` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:62` | - |  |
| `eval_multi_prop` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:77` | - | Evaluate a 2D multi-dimensional property at frame `f`. |
| `extract_multi_val` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:121` | - |  |
| `eval_layer_transform` | 2 | `crates/kinetics-motion-compositor/src/lottie/eval.rs:131` | - | Evaluate the transform of a layer at frame `f`. |
| `parse_lottie` | 1 | `crates/kinetics-motion-compositor/src/lottie/mod.rs:12` | - | Parse a Lottie JSON document string. |
| `write_lottie` | 1 | `crates/kinetics-motion-compositor/src/lottie/mod.rs:17` | - | Serialize a LottieAnimation structure to JSON string. |
| `default_sr` | 0 | `crates/kinetics-motion-compositor/src/lottie/schema.rs:37` | - |  |
| `impl Tick::from_rational` | 3 | `crates/kinetics-motion-compositor/src/time.rs:13` | - |  |
| `impl Tick::to_rational` | 3 | `crates/kinetics-motion-compositor/src/time.rs:21` | - |  |
| `impl Tick::from_seconds` | 1 | `crates/kinetics-motion-compositor/src/time.rs:30` | - |  |
| `impl Tick::as_seconds` | 1 | `crates/kinetics-motion-compositor/src/time.rs:34` | - |  |
| `impl std::ops::Add for Tick::add` | 2 | `crates/kinetics-motion-compositor/src/time.rs:41` | - |  |
| `impl std::ops::Sub for Tick::sub` | 2 | `crates/kinetics-motion-compositor/src/time.rs:48` | - |  |
| `impl FrameRate::as_f64` | 1 | `crates/kinetics-motion-compositor/src/time.rs:65` | - |  |
| `impl FrameRate::frame_at` | 2 | `crates/kinetics-motion-compositor/src/time.rs:69` | - |  |
| `impl FrameRate::tick_of_frame` | 2 | `crates/kinetics-motion-compositor/src/time.rs:73` | - |  |
| `impl GrayImage::new` | 2 | `crates/kinetics-motion-compositor/src/track/correlation.rs:14` | - |  |
| `impl GrayImage::from_luma` | 3 | `crates/kinetics-motion-compositor/src/track/correlation.rs:22` | - |  |
| `impl GrayImage::get` | 3 | `crates/kinetics-motion-compositor/src/track/correlation.rs:32` | - |  |
| `track_point_ncc` | 5 | `crates/kinetics-motion-compositor/src/track/correlation.rs:43` | - | Match template inside target image within search window using Normalized Cross-Correlation. |
| `impl Default for Homography::default` | 0 | `crates/kinetics-motion-compositor/src/track/homography.rs:10` | - |  |
| `impl Homography::transform_point` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:21` | - | Transform a 2D point [x, y] by this homography. |
| `solve_corner_pin_homography` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:34` | numerics | Solve for the 3x3 homography mapping 4 source points to 4 destination points using DLT. |
| `solve_8x8` | 2 | `crates/kinetics-motion-compositor/src/track/homography.rs:62` | numerics |  |
| `impl ImagePyramid::build` | 2 | `crates/kinetics-motion-compositor/src/track/klt.rs:24` | - | Construct a Gaussian pyramid with `num_levels` downsampled by 2x each. |
| `detect_shi_tomasi_corners` | 4 | `crates/kinetics-motion-compositor/src/track/klt.rs:54` | - | Detect good features to track using the Shi-Tomasi minimum eigenvalue criterion. |
| `track_klt_point` | 6 | `crates/kinetics-motion-compositor/src/track/klt.rs:148` | - | Track a point from frame I to frame J using pyramidal Lucas-Kanade. |
| `sample_bilinear` | 3 | `crates/kinetics-motion-compositor/src/track/klt.rs:254` | - |  |
| `impl Default for CharTransform::default` | 0 | `crates/kinetics-motion-compositor/src/typography/animator.rs:18` | - |  |
| `impl TextAnimator::evaluate_character` | 4 | `crates/kinetics-motion-compositor/src/typography/animator.rs:48` | - |  |
| `evaluate_text_animators` | 3 | `crates/kinetics-motion-compositor/src/typography/animator.rs:66` | - | Evaluate all characters of a string through a set of kinetic text animators. |
| `impl Mode::initial` | 1 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:32` | - |  |
| `impl Mode::combine` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:39` | - |  |
| `impl Default for RangeSelector::default` | 0 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:78` | - |  |
| `impl RangeSelector::evaluate_unit` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:96` | - | Evaluate the selector weight in [-1, 1] for unit index `i` out of `total_units`. |
| `shape_value` | 2 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:122` | - |  |
| `apply_ease` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:160` | - |  |
| `pseudo_permute` | 3 | `crates/kinetics-motion-compositor/src/typography/selectors.rs:175` | - |  |
| `impl Default for WigglySelector::default` | 0 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:17` | - |  |
| `impl WigglySelector::evaluate_unit` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:32` | - | Evaluate wiggly weight for unit `i` at time `t_sec`. |
| `smooth_noise_2d` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:44` | - | 2D coherent gradient noise. |
| `hash_coord` | 3 | `crates/kinetics-motion-compositor/src/typography/wiggly.rs:65` | - |  |

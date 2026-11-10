# `crates/layout-presentation-deck`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Presentation deck model, shape morphing, transition interpolation, and animation timeline.
Grounded in UDHR Articles 19 & 27 and Informatics Fiduciary standards._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for Theme::default` | 0 | `crates/layout-presentation-deck/src/model/master.rs:17` | - |  |
| `impl Layout::new` | 3 | `crates/layout-presentation-deck/src/model/master.rs:36` | - |  |
| `impl Master::new` | 2 | `crates/layout-presentation-deck/src/model/master.rs:56` | - |  |
| `impl Slide::new` | 2 | `crates/layout-presentation-deck/src/model/master.rs:78` | - |  |
| `impl Slide::shape_by_id` | 2 | `crates/layout-presentation-deck/src/model/master.rs:89` | - |  |
| `impl Slide::shape_by_id_mut` | 2 | `crates/layout-presentation-deck/src/model/master.rs:93` | - |  |
| `impl Default for Presentation::default` | 0 | `crates/layout-presentation-deck/src/model/master.rs:122` | - |  |
| `impl Presentation::master_for_slide` | 2 | `crates/layout-presentation-deck/src/model/master.rs:135` | - |  |
| `impl Ctx<'a>::for_slide` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:25` | - |  |
| `impl Ctx<'a>::for_layout` | 3 | `crates/layout-presentation-deck/src/model/resolve.rs:36` | - |  |
| `impl Ctx<'a>::for_master` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:46` | - |  |
| `compatible` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:57` | - |  |
| `find_ph` | 3 | `crates/layout-presentation-deck/src/model/resolve.rs:67` | - |  |
| `master_ph` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:84` | - |  |
| `parents` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:94` | - | Layout and master placeholders a shape inherits from. |
| `resolve_xfrm` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:108` | - | Effective bounding transform of a shape after placeholder resolution. |
| `resolve_fill` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:127` | - | Effective fill of a shape after placeholder resolution. |
| `resolve_line` | 2 | `crates/layout-presentation-deck/src/model/resolve.rs:146` | - | Effective line/border of a shape after placeholder resolution. |
| `impl Run::new` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:30` | - |  |
| `impl Paragraph::new` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:64` | - |  |
| `impl Paragraph::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:71` | - |  |
| `impl TextBody::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:83` | - |  |
| `impl Default for TableCell::default` | 0 | `crates/layout-presentation-deck/src/model/shape.rs:98` | - |  |
| `impl Default for SlideTransition::default` | 0 | `crates/layout-presentation-deck/src/model/shape.rs:141` | - |  |
| `impl Shape::new` | 3 | `crates/layout-presentation-deck/src/model/shape.rs:171` | - |  |
| `impl Shape::with_text` | 2 | `crates/layout-presentation-deck/src/model/shape.rs:185` | - |  |
| `impl Shape::with_paragraphs` | 2 | `crates/layout-presentation-deck/src/model/shape.rs:193` | - | One DrawingML paragraph per string so U+000A never lives in a single `a:t`. |
| `impl Shape::text` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:209` | - |  |
| `impl Shape::ph_type` | 1 | `crates/layout-presentation-deck/src/model/shape.rs:216` | - |  |
| `impl PhType::is_title` | 1 | `crates/layout-presentation-deck/src/model/types.rs:41` | - |  |
| `impl Xfrm::new` | 4 | `crates/layout-presentation-deck/src/model/types.rs:58` | - |  |
| `impl Rgba::new` | 4 | `crates/layout-presentation-deck/src/model/types.rs:94` | - |  |
| `impl Default for ColorScheme::default` | 0 | `crates/layout-presentation-deck/src/model/types.rs:135` | - |  |
| `impl ColorScheme::get` | 2 | `crates/layout-presentation-deck/src/model/types.rs:154` | - |  |
| `impl ColorRef::resolve` | 2 | `crates/layout-presentation-deck/src/model/types.rs:179` | - |  |
| `impl Default for Line::default` | 0 | `crates/layout-presentation-deck/src/model/types.rs:201` | - |  |
| `clamp` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:5` | - |  |
| `lerp` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:17` | - |  |
| `lerp_rgba` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:28` | - |  |
| `morph_xfrm` | 3 | `crates/layout-presentation-deck/src/morph/interpolate.rs:42` | - | Interpolate a box from `a` to `b` at `t` (0..1): |
| `text_of` | 1 | `crates/layout-presentation-deck/src/morph/match_pairs.rs:7` | - |  |
| `morph_pairs` | 2 | `crates/layout-presentation-deck/src/morph/match_pairs.rs:16` | - | Pairs of (shape on `a`, shape on `b`) that Morph animates from one to the other: |
| `impl MorphFrame::opacity_of` | 2 | `crates/layout-presentation-deck/src/morph/transition.rs:37` | - |  |
| `build_morph_frame` | 3 | `crates/layout-presentation-deck/src/morph/transition.rs:43` | - | Compute a morph transition frame between slide `a` and slide `b` at progress `t` (0.0 .. 1.0). |
| `impl NQuin48::new` | 5 | `crates/layout-presentation-deck/src/ontology.rs:17` | - |  |
| `impl NQuin48::is_valid` | 1 | `crates/layout-presentation-deck/src/ontology.rs:29` | - |  |
| `impl Default for Animation::default` | 0 | `crates/layout-presentation-deck/src/timeline/anim.rs:41` | - |  |
| `tokenize` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:17` | - |  |
| `impl MotionPath::parse` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:57` | - |  |
| `impl MotionPath::total_length` | 1 | `crates/layout-presentation-deck/src/timeline/path.rs:146` | - |  |
| `impl MotionPath::point_at` | 2 | `crates/layout-presentation-deck/src/timeline/path.rs:151` | - | Evaluate position (fractional x, y) at progress t (0.0 .. 1.0). |
| `safe_url` | 1 | `crates/layout-presentation-deck/src/timeline/show.rs:16` | - |  |
| `show_order` | 1 | `crates/layout-presentation-deck/src/timeline/show.rs:43` | - | Slide indices visited in order, filtering out hidden slides and applying ranges/custom shows. |
| `impl ShowState::next` | 3 | `crates/layout-presentation-deck/src/timeline/show.rs:70` | - |  |
| `impl ShowState::prev` | 2 | `crates/layout-presentation-deck/src/timeline/show.rs:101` | - |  |
| `impl Timeline::compile` | 1 | `crates/layout-presentation-deck/src/timeline/timeline.rs:21` | - |  |
| `impl Timeline::duration_of_step` | 2 | `crates/layout-presentation-deck/src/timeline/timeline.rs:90` | - |  |
| `impl Timeline::is_shape_visible` | 4 | `crates/layout-presentation-deck/src/timeline/timeline.rs:95` | - | Check if shape is visible at a given step and progress t. |
| `test_placeholder_inheritance` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:4` | - |  |
| `test_morph_matching_and_interpolation` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:50` | - |  |
| `test_motion_path_spline` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:82` | - |  |
| `test_timeline_and_show_progression` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:97` | - |  |
| `test_ontology_contract_parity` | 0 | `crates/layout-presentation-deck/tests/deck_tests.rs:144` | semantics |  |

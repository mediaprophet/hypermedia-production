# `crates/layout-box-autolayout`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`layout-box-autolayout` — UI-agnostic categorical Flexbox and CSS Grid auto-layout engine.

Provides:
- Full Flexbox auto-layout (`LayoutDirection::{Horizontal, Vertical}`)
- CSS Grid auto-layout with fractional `fr` tracks (`LayoutDirection::Grid`, `GridTrack`)
- Sizing modes: `Fixed`, `Hug` (fit content), `Fill` (share free space)
- Free space distribution algorithm (`distribute`) respecting min/max bounds
- Multi-line wrapping with content alignment (`ContentAlign`) and cross alignment (`CrossAlign`)
- Text baseline alignment (`CrossAlign::Baseline`)
- Constraint propagation on frame resize (`HConstraint`, `VConstraint`, `apply_constraints`)
- Top-level `solve_autolayout` orchestrator with nested container support_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `apply_constraints` | 5 | `crates/layout-box-autolayout/src/constraints.rs:9` | - | Adjusts children positions and sizes when their parent changes size from `(ow, oh)` to `(nw, nh)`. |
| `main_offsets` | 5 | `crates/layout-box-autolayout/src/flex.rs:23` | - |  |
| `distribute` | 3 | `crates/layout-box-autolayout/src/flex.rs:44` | - | Distributes free main-axis space among open fill items like CSS flexbox. |
| `solve_flex` | 3 | `crates/layout-box-autolayout/src/flex.rs:67` | numerics |  |
| `impl LayoutNode::new` | 3 | `crates/layout-box-autolayout/src/frame.rs:39` | - |  |
| `impl LayoutNode::with_sizing` | 3 | `crates/layout-box-autolayout/src/frame.rs:63` | - |  |
| `impl LayoutNode::with_constraints` | 3 | `crates/layout-box-autolayout/src/frame.rs:69` | - |  |
| `impl Default for AutoLayoutFrame::default` | 0 | `crates/layout-box-autolayout/src/frame.rs:99` | - |  |
| `impl AutoLayoutFrame::new` | 1 | `crates/layout-box-autolayout/src/frame.rs:123` | - |  |
| `impl AutoLayoutFrame::with_padding` | 2 | `crates/layout-box-autolayout/src/frame.rs:130` | - |  |
| `impl AutoLayoutFrame::with_gap` | 2 | `crates/layout-box-autolayout/src/frame.rs:135` | - |  |
| `impl AutoLayoutFrame::with_wrap` | 2 | `crates/layout-box-autolayout/src/frame.rs:140` | - |  |
| `impl AutoLayoutFrame::add_child` | 2 | `crates/layout-box-autolayout/src/frame.rs:145` | - |  |
| `grid_place` | 2 | `crates/layout-box-autolayout/src/grid.rs:20` | - |  |
| `resolve_tracks` | 3 | `crates/layout-box-autolayout/src/grid.rs:127` | - | Resolves track sizes for a list of track specs, given content sizes and available space. |
| `solve_grid` | 3 | `crates/layout-box-autolayout/src/grid.rs:168` | numerics |  |
| `solve_autolayout` | 3 | `crates/layout-box-autolayout/src/solver.rs:13` | numerics | Solves auto-layout on a frame with the given available dimensions. |
| `test_horizontal_flex_basic` | 0 | `crates/layout-box-autolayout/src/tests.rs:16` | - |  |
| `test_hug_container_sizing` | 0 | `crates/layout-box-autolayout/src/tests.rs:40` | - |  |
| `test_fill_distribution_with_clamping` | 0 | `crates/layout-box-autolayout/src/tests.rs:60` | - |  |
| `test_multi_line_wrapping` | 0 | `crates/layout-box-autolayout/src/tests.rs:84` | - |  |
| `test_baseline_alignment` | 0 | `crates/layout-box-autolayout/src/tests.rs:104` | - |  |
| `test_css_grid_tracks_and_spans` | 0 | `crates/layout-box-autolayout/src/tests.rs:125` | - |  |
| `test_constraints_propagation` | 0 | `crates/layout-box-autolayout/src/tests.rs:171` | - |  |
| `impl MainAlign::is_auto_gap` | 1 | `crates/layout-box-autolayout/src/types.rs:35` | - |  |
| `impl Default for GridTrack::default` | 0 | `crates/layout-box-autolayout/src/types.rs:96` | - |  |
| `impl Rect::new` | 4 | `crates/layout-box-autolayout/src/types.rs:110` | - |  |
| `impl Rect::right` | 1 | `crates/layout-box-autolayout/src/types.rs:113` | - |  |
| `impl Rect::bottom` | 1 | `crates/layout-box-autolayout/src/types.rs:116` | - |  |
| `impl Padding::new` | 4 | `crates/layout-box-autolayout/src/types.rs:130` | - |  |
| `impl Padding::uniform` | 1 | `crates/layout-box-autolayout/src/types.rs:133` | - |  |
| `impl Padding::symmetric` | 2 | `crates/layout-box-autolayout/src/types.rs:136` | - |  |
| `impl Padding::horizontal` | 1 | `crates/layout-box-autolayout/src/types.rs:139` | - |  |
| `impl Padding::vertical` | 1 | `crates/layout-box-autolayout/src/types.rs:142` | - |  |
| `clamp_size` | 3 | `crates/layout-box-autolayout/src/types.rs:147` | - |  |

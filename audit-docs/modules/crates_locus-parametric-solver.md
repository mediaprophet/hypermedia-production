# `crates/locus-parametric-solver`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`locus-parametric-solver` — Categorical 2D sketch constraint solver and CAD typography.

Provides:
- Weighted minimum-norm damped Gauss-Newton (Levenberg-Marquardt) 2D parametric sketch constraint solver (`solver`)
- Conflict detection and minimal conflicting subset reporting
- Over-constraint / redundancy check via Jacobian rank
- Parametric formula expression evaluator with dependency environment and cycle detection
- Heuristic autoconstrain inference across geometric primitives
- Single-stroke CAM/plotter stick font polyline evaluator (`typography::stroke`)
- AutoCAD MTEXT formatting parser, control code decoder, and word wrap layout engine (`typography::mtext`)_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Default for InferOptions::default` | 0 | `crates/locus-parametric-solver/src/solver/auto.rs:21` | - |  |
| `collect` | 1 | `crates/locus-parametric-solver/src/solver/auto.rs:36` | - |  |
| `ang_between` | 2 | `crates/locus-parametric-solver/src/solver/auto.rs:97` | 2d |  |
| `infer` | 2 | `crates/locus-parametric-solver/src/solver/auto.rs:106` | - |  |
| `impl fmt::Display for SolveError::fmt` | 2 | `crates/locus-parametric-solver/src/solver/conflict.rs:23` | - |  |
| `ids_text` | 1 | `crates/locus-parametric-solver/src/solver/conflict.rs:37` | - |  |
| `eval_target` | 2 | `crates/locus-parametric-solver/src/solver/conflict.rs:57` | - | Evaluates target value for dimensional constraints from environment. |
| `check_redundancy` | 4 | `crates/locus-parametric-solver/src/solver/conflict.rs:75` | - | Check if a candidate constraint is redundant with existing active constraints. |
| `solve_system` | 4 | `crates/locus-parametric-solver/src/solver/conflict.rs:109` | numerics | Solve the constraint system on the given model entities. |
| `impl fmt::Display for ExprError::fmt` | 2 | `crates/locus-parametric-solver/src/solver/expr.rs:24` | - |  |
| `valid_name` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:46` | - |  |
| `impl Parser<'_>::ws` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:61` | - |  |
| `impl Parser<'_>::peek` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:66` | - |  |
| `impl Parser<'_>::enter` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:70` | - |  |
| `impl Parser<'_>::sum` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:74` | - |  |
| `impl Parser<'_>::product` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:85` | - |  |
| `impl Parser<'_>::unary` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:94` | - |  |
| `impl Parser<'_>::atom` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:113` | - |  |
| `impl Parser<'_>::number` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:133` | - |  |
| `impl Parser<'_>::name` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:142` | - |  |
| `parse` | 1 | `crates/locus-parametric-solver/src/solver/expr.rs:155` | - |  |
| `impl Env::new` | 0 | `crates/locus-parametric-solver/src/solver/expr.rs:178` | - |  |
| `impl Env::insert` | 3 | `crates/locus-parametric-solver/src/solver/expr.rs:181` | - |  |
| `impl Env::eval` | 2 | `crates/locus-parametric-solver/src/solver/expr.rs:184` | - |  |
| `impl Env::eval_ast` | 3 | `crates/locus-parametric-solver/src/solver/expr.rs:189` | - |  |
| `max_abs` | 1 | `crates/locus-parametric-solver/src/solver/lm.rs:18` | - |  |
| `norm2` | 1 | `crates/locus-parametric-solver/src/solver/lm.rs:22` | - |  |
| `jacobian` | 4 | `crates/locus-parametric-solver/src/solver/lm.rs:27` | - | Numerical Jacobian over the free parameters (row-major, rows x free.len()). |
| `solve_dense` | 3 | `crates/locus-parametric-solver/src/solver/lm.rs:60` | numerics | Solve the dense system `a y = b` (n x n, row-major) by Gaussian elimination with partial pivoting. |
| `rank` | 3 | `crates/locus-parametric-solver/src/solver/lm.rs:107` | numerics | Numerical rank of a row-major `rows x cols` matrix (rows normalised first). |
| `solve` | 6 | `crates/locus-parametric-solver/src/solver/lm.rs:153` | numerics | Run the damped Levenberg-Marquardt solver from `x0`. Only `free` parameters change. |
| `impl Handle::new` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:22` | - |  |
| `impl Sub::parse` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:41` | - |  |
| `impl GeomRef::new` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:69` | - |  |
| `impl GeomRef::whole` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:72` | - |  |
| `p2` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:141` | 2d |  |
| `impl Model::build` | 1 | `crates/locus-parametric-solver/src/solver/model.rs:146` | - |  |
| `impl Model::slot` | 2 | `crates/locus-parametric-solver/src/solver/model.rs:191` | - |  |
| `impl Model::point` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:195` | 2d |  |
| `impl Model::line` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:244` | 2d |  |
| `impl Model::circle` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:262` | 2d |  |
| `impl Model::arc` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:270` | 2d |  |
| `impl Model::extract` | 3 | `crates/locus-parametric-solver/src/solver/model.rs:279` | - | Extract updated entity from solved parameters. |
| `impl ConstraintKind::is_dimensional` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:39` | - |  |
| `impl ConstraintKind::name` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:42` | - |  |
| `dir_n` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:105` | 2d |  |
| `sgn` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:111` | - |  |
| `wrap` | 1 | `crates/locus-parametric-solver/src/solver/residual.rs:115` | - |  |
| `r_at` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:121` | - |  |
| `impl Built::new` | 4 | `crates/locus-parametric-solver/src/solver/residual.rs:126` | - |  |
| `impl Built::eval` | 4 | `crates/locus-parametric-solver/src/solver/residual.rs:317` | - |  |
| `r_at_self` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:499` | - |  |
| `pad` | 2 | `crates/locus-parametric-solver/src/solver/residual.rs:503` | - |  |
| `test_dense_linear_solver_and_rank` | 0 | `crates/locus-parametric-solver/src/tests.rs:19` | numerics |  |
| `test_lm_minimum_norm` | 0 | `crates/locus-parametric-solver/src/tests.rs:35` | - |  |
| `test_parametric_sketch_horizontal_and_distance` | 0 | `crates/locus-parametric-solver/src/tests.rs:45` | - |  |
| `test_coincident_and_tangent_solve` | 0 | `crates/locus-parametric-solver/src/tests.rs:98` | numerics |  |
| `test_conflict_detection_fails_closed` | 0 | `crates/locus-parametric-solver/src/tests.rs:151` | - |  |
| `test_expression_evaluation` | 0 | `crates/locus-parametric-solver/src/tests.rs:190` | - |  |
| `test_autoconstrain_inference` | 0 | `crates/locus-parametric-solver/src/tests.rs:211` | - |  |
| `test_single_stroke_cam_rendering` | 0 | `crates/locus-parametric-solver/src/tests.rs:242` | - |  |
| `test_mtext_parsing_and_layout` | 0 | `crates/locus-parametric-solver/src/tests.rs:253` | - |  |
| `decode_controls` | 1 | `crates/locus-parametric-solver/src/typography/mtext.rs:16` | - | Decode `%%` control codes: |
| `plain_mtext` | 1 | `crates/locus-parametric-solver/src/typography/mtext.rs:57` | - | Convert MTEXT formatting codes to plain text with `\n` line breaks. |
| `impl Default for MTextParams::default` | 0 | `crates/locus-parametric-solver/src/typography/mtext.rs:138` | - |  |
| `layout_mtext` | 2 | `crates/locus-parametric-solver/src/typography/mtext.rs:169` | - | Layout an MTEXT string into positioned lines and strokes. |
| `glyph` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:14` | - |  |
| `char_advance` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:119` | - |  |
| `decode_stroke` | 1 | `crates/locus-parametric-solver/src/typography/stroke.rs:127` | 2d |  |
| `layout_stroke_line` | 4 | `crates/locus-parametric-solver/src/typography/stroke.rs:158` | - |  |

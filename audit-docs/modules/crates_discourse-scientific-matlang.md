# `crates/discourse-scientific-matlang`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Pure-Rust M-code interpreter module._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Value::as_matrix` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:17` | numerics |  |
| `impl Value::as_num` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:26` | - |  |
| `impl Value::is_truthy` | 1 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:35` | - |  |
| `impl Default for Workspace::default` | 0 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:51` | - |  |
| `impl Workspace::new` | 0 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:57` | - |  |
| `impl Workspace::exec` | 2 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:67` | - |  |
| `impl Workspace::eval_expr` | 2 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:118` | - |  |
| `impl Workspace::eval_binop` | 4 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:210` | - |  |
| `impl Workspace::eval_call` | 3 | `crates/discourse-scientific-matlang/src/interpreter/eval.rs:251` | - |  |
| `impl fmt::Display for LangError::fmt` | 2 | `crates/discourse-scientific-matlang/src/interpreter/lexer.rs:12` | - |  |
| `lex` | 1 | `crates/discourse-scientific-matlang/src/interpreter/lexer.rs:54` | - |  |
| `eval_m_code` | 1 | `crates/discourse-scientific-matlang/src/interpreter/mod.rs:12` | - |  |
| `impl Parser::new` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:12` | - |  |
| `impl Parser::peek` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:16` | - |  |
| `impl Parser::next` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:20` | - |  |
| `impl Parser::skip_newlines_and_semicolons` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:30` | - |  |
| `impl Parser::parse` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:36` | - |  |
| `impl Parser::statement` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:46` | - |  |
| `impl Parser::parse_if` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:85` | - |  |
| `impl Parser::parse_for` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:128` | - |  |
| `impl Parser::parse_while` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:155` | - |  |
| `impl Parser::expression` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:175` | - |  |
| `impl Parser::parse_logical_or` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:179` | - |  |
| `impl Parser::parse_logical_and` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:189` | - |  |
| `impl Parser::parse_comparison` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:199` | - |  |
| `impl Parser::parse_range` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:218` | - |  |
| `impl Parser::parse_add_sub` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:235` | - |  |
| `impl Parser::parse_mul_div` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:248` | - |  |
| `impl Parser::parse_power` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:263` | - |  |
| `impl Parser::parse_unary` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:277` | - |  |
| `impl Parser::parse_postfix` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:291` | - |  |
| `impl Parser::parse_primary` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:300` | - |  |
| `impl Parser::parse_matrix_literal` | 1 | `crates/discourse-scientific-matlang/src/interpreter/parser.rs:338` | numerics |  |
| `test_matrix_decompositions` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:24` | numerics |  |
| `test_matrix_slicing` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:73` | numerics |  |
| `test_m_code_evaluation` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:89` | - |  |
| `test_svg_figure_generation` | 0 | `crates/discourse-scientific-matlang/src/lib.rs:111` | - |  |
| `impl fmt::Display for MatError::fmt` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:17` | - |  |
| `impl Matrix::zeros` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:45` | - |  |
| `impl Matrix::ones` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:53` | - |  |
| `impl Matrix::eye` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:61` | - |  |
| `impl Matrix::scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:69` | - |  |
| `impl Matrix::row_vector` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:77` | - |  |
| `impl Matrix::col_vector` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:85` | - |  |
| `impl Matrix::from_col_major` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:94` | - |  |
| `impl Matrix::from_row_major` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:101` | - |  |
| `impl Matrix::get` | 3 | `crates/discourse-scientific-matlang/src/matrix.rs:115` | - |  |
| `impl Matrix::map` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:123` | - |  |
| `impl Matrix::set` | 4 | `crates/discourse-scientific-matlang/src/matrix.rs:132` | - |  |
| `impl Matrix::shape` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:141` | - |  |
| `impl Matrix::numel` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:145` | - |  |
| `impl Matrix::is_scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:149` | - |  |
| `impl Matrix::as_scalar` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:153` | - |  |
| `impl Matrix::slice` | 5 | `crates/discourse-scientific-matlang/src/matrix.rs:162` | - | 2D slicing: extracts submatrix over [r_start..r_end] x [c_start..c_end] (0-based, half-open). |
| `impl Matrix::transpose` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:180` | - |  |
| `impl Matrix::matmul` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:190` | - |  |
| `impl Matrix::elem_add` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:209` | - |  |
| `impl Matrix::elem_sub` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:220` | - |  |
| `impl Matrix::elem_mul` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:231` | - |  |
| `impl Matrix::scale` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:242` | - |  |
| `impl Matrix::norm` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:250` | - |  |
| `impl Matrix::det` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:254` | - |  |
| `impl Matrix::solve` | 2 | `crates/discourse-scientific-matlang/src/matrix.rs:294` | numerics |  |
| `impl Matrix::inverse` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:359` | - |  |
| `lu` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:368` | numerics | LU decomposition with partial pivoting: P A = L U. |
| `chol` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:439` | numerics | Cholesky factorization: R' R = A for symmetric positive-definite A. |
| `qr` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:467` | numerics | Householder QR decomposition: A = Q R. |
| `eig_symmetric` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:535` | numerics | Jacobi eigenvalue decomposition of a symmetric matrix: V' A V = diag(lambda). |
| `svd` | 1 | `crates/discourse-scientific-matlang/src/matrix.rs:596` | 3d, numerics | SVD via normal equations A'A: A = U S V'. |
| `impl Colormap::sample` | 2 | `crates/discourse-scientific-matlang/src/plot.rs:26` | - |  |
| `impl Series::line` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:71` | - |  |
| `impl Series::scatter` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:83` | - |  |
| `impl Series::bar` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:95` | - |  |
| `impl Series::surface` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:107` | - |  |
| `impl Figure::new` | 0 | `crates/discourse-scientific-matlang/src/plot.rs:140` | - |  |
| `impl Figure::add_series` | 2 | `crates/discourse-scientific-matlang/src/plot.rs:148` | - |  |
| `impl Figure::to_svg` | 3 | `crates/discourse-scientific-matlang/src/plot.rs:153` | - | Renders figure to a standalone SVG XML string. |

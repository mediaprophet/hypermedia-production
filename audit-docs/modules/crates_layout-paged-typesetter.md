# `crates/layout-paged-typesetter`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Citation and bibliography engine._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl Citation::format` | 3 | `crates/layout-paged-typesetter/src/citations/csl.rs:39` | - |  |
| `format_full_entry` | 1 | `crates/layout-paged-typesetter/src/citations/csl.rs:73` | - |  |
| `impl Bibliography::generate` | 2 | `crates/layout-paged-typesetter/src/citations/csl.rs:106` | - |  |
| `zotero_code` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:13` | - |  |
| `stored_code` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:19` | - |  |
| `format_csl_citation_field` | 1 | `crates/layout-paged-typesetter/src/citations/zotero_fields.rs:23` | - |  |
| `impl Math::new` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:17` | - |  |
| `impl Math::is_empty` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:25` | - |  |
| `impl MRun::new` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:70` | - |  |
| `impl MRun::italic` | 1 | `crates/layout-paged-typesetter/src/equation/ast.rs:77` | - |  |
| `parse_latex` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:5` | - |  |
| `to_latex` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:13` | - |  |
| `impl Parser::peek` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:132` | - |  |
| `impl Parser::next` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:136` | - |  |
| `impl Parser::skip_whitespace` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:142` | - |  |
| `impl Parser::parse_arg` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:152` | - |  |
| `impl Parser::parse_command` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:265` | - |  |
| `impl Parser::parse_group` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:278` | - |  |
| `impl Parser::parse_script_target` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:294` | - |  |
| `impl Parser::parse_until` | 2 | `crates/layout-paged-typesetter/src/equation/latex.rs:298` | - |  |
| `impl Parser::parse_until_right` | 1 | `crates/layout-paged-typesetter/src/equation/latex.rs:310` | - |  |
| `impl Parser::parse_nary` | 2 | `crates/layout-paged-typesetter/src/equation/latex.rs:322` | - |  |
| `impl MathLayout::height` | 1 | `crates/layout-paged-typesetter/src/equation/layout.rs:37` | - |  |
| `impl MathLayout::shift` | 3 | `crates/layout-paged-typesetter/src/equation/layout.rs:41` | - |  |
| `layout_equation` | 2 | `crates/layout-paged-typesetter/src/equation/layout.rs:63` | - |  |
| `parse_linear` | 1 | `crates/layout-paged-typesetter/src/equation/linear.rs:5` | numerics |  |
| `to_linear` | 1 | `crates/layout-paged-typesetter/src/equation/linear.rs:92` | numerics |  |
| `is_east_asian` | 1 | `crates/layout-paged-typesetter/src/hyphenation/kinsoku.rs:15` | - | Determines if character `c` is East Asian text (CJK ideographs, kana, bopomofo, CJK punctuation). |
| `line_break_opportunities` | 2 | `crates/layout-paged-typesetter/src/hyphenation/kinsoku.rs:33` | - | Computes valid line-break opportunities (byte offsets) in `text` respecting Kinsoku rules. |
| `impl Patterns::new` | 0 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:15` | - |  |
| `impl Patterns::parse` | 1 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:20` | - | Parse TeX-style patterns, whitespace-separated (e.g. `hy3ph he2n hena4`). |
| `impl Patterns::english_standard` | 0 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:56` | - | Standard English patterns compiled for common typography. |
| `impl Patterns::points` | 2 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:70` | - | Break points (character indices where a hyphen may be inserted before that char) |
| `impl Patterns::hyphenate` | 3 | `crates/layout-paged-typesetter/src/hyphenation/liang.rs:109` | - | Hyphenates a word, inserting hyphen strings. |
| `impl OntologicalTriple::new` | 4 | `crates/layout-paged-typesetter/src/ontology.rs:20` | - |  |
| `impl OntologicalTriple::verify_parity` | 1 | `crates/layout-paged-typesetter/src/ontology.rs:31` | - |  |
| `impl Default for ParaDoc::default` | 0 | `crates/layout-paged-typesetter/src/pagination/flow.rs:21` | - |  |
| `impl ParaDoc::new` | 1 | `crates/layout-paged-typesetter/src/pagination/flow.rs:38` | - |  |
| `impl ParaDoc::heading` | 2 | `crates/layout-paged-typesetter/src/pagination/flow.rs:45` | - |  |
| `impl Typesetter::new` | 1 | `crates/layout-paged-typesetter/src/pagination/flow.rs:71` | - |  |
| `impl Typesetter::paginate` | 3 | `crates/layout-paged-typesetter/src/pagination/flow.rs:76` | 2d | Paginates paragraphs into pages. |
| `impl Point::new` | 2 | `crates/layout-paged-typesetter/src/pagination/types.rs:12` | - |  |
| `impl Rect::new` | 4 | `crates/layout-paged-typesetter/src/pagination/types.rs:26` | - |  |
| `impl Rect::right` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:30` | - |  |
| `impl Rect::bottom` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:34` | - |  |
| `impl Rect::intersects` | 2 | `crates/layout-paged-typesetter/src/pagination/types.rs:38` | 2d |  |
| `impl Default for SectionProps::default` | 0 | `crates/layout-paged-typesetter/src/pagination/types.rs:56` | - |  |
| `impl SectionProps::content_width` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:72` | - |  |
| `impl SectionProps::content_height` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:76` | - |  |
| `impl SectionProps::column_width` | 1 | `crates/layout-paged-typesetter/src/pagination/types.rs:80` | - |  |
| `autofit_widths` | 3 | `crates/layout-paged-typesetter/src/table_layout/autofit.rs:9` | - | Auto-fits column widths to available table width. |
| `impl TableCellData::new` | 1 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:15` | - |  |
| `impl TableCellData::with_span` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:25` | - |  |
| `impl TableData::col_count` | 1 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:37` | - |  |
| `measure_table_columns` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:54` | - | Measures the (min_width, max_width) for each column in points. |
| `measure_cell_text` | 2 | `crates/layout-paged-typesetter/src/table_layout/measure.rs:112` | - |  |
| `test_equation_latex_and_unicode_math` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:4` | - |  |
| `test_liang_hyphenation` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:24` | - |  |
| `test_kinsoku_line_breaking` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:34` | - |  |
| `test_table_autofit` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:45` | - |  |
| `test_pagination_widow_orphan` | 0 | `crates/layout-paged-typesetter/tests/typesetter_tests.rs:60` | - |  |

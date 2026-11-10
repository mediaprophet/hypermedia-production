# `crates/discourse-academic-tex`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Academic bibliography utilities: identifier parsing, format conversion, and audit._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `audit_bib` | 1 | `crates/discourse-academic-tex/src/bibliography/audit.rs:42` | - | Run audit rules over a [`BibFile`]. |
| `audit_entry_fields` | 2 | `crates/discourse-academic-tex/src/bibliography/audit.rs:114` | - |  |
| `test_audit_duplicates` | 0 | `crates/discourse-academic-tex/src/bibliography/audit.rs:164` | - |  |
| `scrub` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:24` | - |  |
| `percent_decode` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:28` | - |  |
| `looks_like_doi` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:48` | - |  |
| `looks_like_arxiv` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:54` | - |  |
| `arxiv_body` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:68` | - |  |
| `parse_identifier` | 1 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:76` | - | Parse any pasted identifier into a normalized [`Ident`]. |
| `doi_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:148` | - |  |
| `arxiv_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:156` | - |  |
| `pubmed_forms` | 0 | `crates/discourse-academic-tex/src/bibliography/identifiers.rs:163` | - |  |
| `sniff` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:19` | - | Sniff source format: JSON openers -> CSL-JSON, `TY  -` tagged lines -> RIS. |
| `bibval` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:36` | - | BibTeX-safe value: braces kept balanced. |
| `emit` | 3 | `crates/discourse-academic-tex/src/bibliography/import.rs:58` | - | Emit `@kind{key, fields…}` from `(name, value)` pairs. |
| `year_of` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:70` | - | First 4-digit run. |
| `norm_title` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:75` | - | Normalized title for duplicate detection. |
| `csl_kind` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:80` | - |  |
| `csl_names` | 2 | `crates/discourse-academic-tex/src/bibliography/import.rs:100` | - |  |
| `csl_to_bibtex` | 2 | `crates/discourse-academic-tex/src/bibliography/import.rs:125` | - |  |
| `ris_kind` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:171` | - |  |
| `ris_entries` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:184` | - |  |
| `flush` | 6 | `crates/discourse-academic-tex/src/bibliography/import.rs:191` | - |  |
| `convert` | 1 | `crates/discourse-academic-tex/src/bibliography/import.rs:262` | - | Convert source buffer (CSL-JSON or RIS) to BibTeX entries. |
| `test_sniff` | 0 | `crates/discourse-academic-tex/src/bibliography/import.rs:290` | - |  |
| `test_ris_conversion` | 0 | `crates/discourse-academic-tex/src/bibliography/import.rs:296` | - |  |
| `req` | 1 | `crates/discourse-academic-tex/src/catalog.rs:54` | - |  |
| `opt` | 1 | `crates/discourse-academic-tex/src/catalog.rs:57` | - |  |
| `lbl` | 1 | `crates/discourse-academic-tex/src/catalog.rs:60` | - |  |
| `key` | 1 | `crates/discourse-academic-tex/src/catalog.rs:63` | - |  |
| `file` | 1 | `crates/discourse-academic-tex/src/catalog.rs:66` | - |  |
| `len` | 1 | `crates/discourse-academic-tex/src/catalog.rs:69` | - |  |
| `col` | 1 | `crates/discourse-academic-tex/src/catalog.rs:72` | - |  |
| `mth` | 1 | `crates/discourse-academic-tex/src/catalog.rs:75` | - |  |
| `cmd` | 4 | `crates/discourse-academic-tex/src/catalog.rs:79` | - |  |
| `env` | 4 | `crates/discourse-academic-tex/src/catalog.rs:82` | - |  |
| `dcmd` | 4 | `crates/discourse-academic-tex/src/catalog.rs:85` | - |  |
| `denv` | 4 | `crates/discourse-academic-tex/src/catalog.rs:88` | - |  |
| `catalog` | 0 | `crates/discourse-academic-tex/src/catalog.rs:388` | - | The full catalog, sorted by [`CommandSpec::name`]. |
| `find_prefix` | 1 | `crates/discourse-academic-tex/src/catalog.rs:393` | - | Commands and environments whose `name` starts with `prefix` (case-sensitive). |
| `find_exact` | 1 | `crates/discourse-academic-tex/src/catalog.rs:400` | - | Exact lookup by `name` (case-sensitive). |
| `packages` | 0 | `crates/discourse-academic-tex/src/catalog.rs:405` | - | Distinct package names present in the catalog, sorted. |
| `environments` | 0 | `crates/discourse-academic-tex/src/catalog.rs:413` | - | The catalog entries that are environments (`\begin{name}`…`\end{name}`). |
| `catalog_is_large_enough` | 0 | `crates/discourse-academic-tex/src/catalog.rs:422` | - |  |
| `catalog_is_sorted_and_unique` | 0 | `crates/discourse-academic-tex/src/catalog.rs:427` | - |  |
| `every_env_is_reachable_via_environments` | 0 | `crates/discourse-academic-tex/src/catalog.rs:434` | - |  |
| `find_prefix_finds_subsection_family` | 0 | `crates/discourse-academic-tex/src/catalog.rs:441` | - |  |
| `packages_include_core_packages` | 0 | `crates/discourse-academic-tex/src/catalog.rs:449` | - |  |
| `find_exact_hits_and_misses` | 0 | `crates/discourse-academic-tex/src/catalog.rs:461` | - |  |
| `every_entry_has_a_doc_line` | 0 | `crates/discourse-academic-tex/src/catalog.rs:467` | - |  |
| `full_discourse_pipeline_smoke` | 0 | `crates/discourse-academic-tex/src/lib.rs:32` | - |  |
| `propose_edits` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:45` | - | Propose a set of deterministic line edits to repair document structure. |
| `insert_before_edit` | 5 | `crates/discourse-academic-tex/src/review/ai_repair.rs:128` | - |  |
| `stray_end_edit` | 4 | `crates/discourse-academic-tex/src/review/ai_repair.rs:138` | - |  |
| `apply_edits` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:164` | - | Apply a sequence of edits bottom-up so line numbers remain stable. |
| `has_later_end` | 3 | `crates/discourse-academic-tex/src/review/ai_repair.rs:204` | - |  |
| `op_rank` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:209` | - |  |
| `source_hash` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:217` | - |  |
| `line_at` | 2 | `crates/discourse-academic-tex/src/review/ai_repair.rs:223` | - |  |
| `eof_anchor` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:227` | - |  |
| `parse_unclosed_begin` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:235` | - |  |
| `parse_end_mismatch` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:239` | - |  |
| `parse_stray_end` | 1 | `crates/discourse-academic-tex/src/review/ai_repair.rs:246` | - |  |
| `unclosed_envs_close_innermost_first` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:258` | - |  |
| `end_mismatch_replaces_wrong_env_name` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:271` | - |  |
| `stray_end_deletes_line` | 0 | `crates/discourse-academic-tex/src/review/ai_repair.rs:284` | - |  |
| `impl TeXDiagnostic::new` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:25` | - |  |
| `enrich_diagnostics` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:152` | - | Enrich diagnostic items in place. |
| `fill` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:166` | - |  |
| `extract_quoted` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:172` | - |  |
| `extract_command` | 1 | `crates/discourse-academic-tex/src/review/logrules.rs:179` | - |  |
| `suggested_package` | 2 | `crates/discourse-academic-tex/src/review/logrules.rs:188` | - |  |
| `undefined_command_gets_hint` | 0 | `crates/discourse-academic-tex/src/review/logrules.rs:201` | - |  |
| `emergency_stop_is_cascade` | 0 | `crates/discourse-academic-tex/src/review/logrules.rs:211` | - |  |
| `global_snippets_path` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:52` | - | Global snippets path: `$ACADEMIC_TEX_HOME/snippets.json`, else `~/.academic-tex/snippets.json`. |
| `parse_snippets` | 2 | `crates/discourse-academic-tex/src/review/snippets.rs:66` | - | Parse a snippet JSON string: accepts `[{trigger,body,...}]` or `{trigger: body}` / `{trigger: {body,...}}`. |
| `read_snippets_file` | 2 | `crates/discourse-academic-tex/src/review/snippets.rs:110` | - | Read a snippet file bounded; missing or invalid returns empty vec. |
| `load_snippets` | 1 | `crates/discourse-academic-tex/src/review/snippets.rs:121` | - | Load and merge all snippets given an optional project directory root. |
| `expand` | 1 | `crates/discourse-academic-tex/src/review/snippets.rs:152` | - | Render `body`: `${n:default}` -> `default`, `$n` -> removed; caret lands |
| `expands_defaults_and_caret` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:199` | - |  |
| `dollar_one_wins_over_zero` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:206` | - |  |
| `bare_tabstops_drop` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:213` | - |  |
| `malformed_files_are_empty_not_fatal` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:220` | - |  |
| `builtins_are_available` | 0 | `crates/discourse-academic-tex/src/review/snippets.rs:227` | - |  |
| `occurrences` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:45` | - |  |
| `context_ok` | 5 | `crates/discourse-academic-tex/src/review/suggest.rs:49` | - |  |
| `resolve` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:56` | - | Resolve anchor against source text. |
| `context_around` | 3 | `crates/discourse-academic-tex/src/review/suggest.rs:70` | - | Capture context around a range in `src` on char boundaries. |
| `apply_suggestion` | 2 | `crates/discourse-academic-tex/src/review/suggest.rs:79` | - | Apply a suggestion to `src`, returning the new source if uniquely anchored. |
| `propose_then_apply` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:102` | - |  |
| `anchor_survives_prefix_shift` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:117` | - |  |
| `stale_returns_err` | 0 | `crates/discourse-academic-tex/src/review/suggest.rs:132` | - |  |
| `impl BibEntry::field` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:22` | - | Case-insensitive field lookup. |
| `impl BibFile::get` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:42` | - | Entry lookup by key (exact match). |
| `parse_bib` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:48` | - | Parse a `.bib` source string. Always returns a file object; problems land in `issues`. |
| `impl Parser::run` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:63` | - |  |
| `impl Parser::cur` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:86` | - |  |
| `impl Parser::pos` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:90` | - |  |
| `impl Parser::issue` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:105` | - |  |
| `impl Parser::issue_at` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:110` | - |  |
| `impl Parser::skip_ws` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:114` | - |  |
| `impl Parser::skip_ws_and_comments` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:120` | - |  |
| `impl Parser::ident` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:133` | - |  |
| `impl Parser::open_delim` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:148` | - |  |
| `impl Parser::comment_entry` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:163` | - |  |
| `impl Parser::preamble_body` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:179` | - |  |
| `impl Parser::bib_entry` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:211` | - |  |
| `impl Parser::value` | 1 | `crates/discourse-academic-tex/src/texparse/bib.rs:285` | - |  |
| `impl Parser::quoted` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:334` | - |  |
| `impl Parser::braced` | 2 | `crates/discourse-academic-tex/src/texparse/bib.rs:364` | - |  |
| `impl Parser::skip_balanced` | 3 | `crates/discourse-academic-tex/src/texparse/bib.rs:401` | - |  |
| `basic_entry` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:427` | - |  |
| `strings_and_concat` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:446` | - |  |
| `quoted_values_and_comments` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:461` | - |  |
| `parens_delimiters` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:469` | - |  |
| `malformed_recovers` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:476` | - |  |
| `hostile_never_panics` | 0 | `crates/discourse-academic-tex/src/texparse/bib.rs:484` | - |  |
| `impl DepKind::default_ext` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:36` | - |  |
| `extract_deps` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:57` | - | Extract every file-level dependency reference from `src`. |
| `resolve` | 2 | `crates/discourse-academic-tex/src/texparse/graph.rs:116` | - |  |
| `build` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:141` | - |  |
| `declares_document_class` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:182` | - |  |
| `file_arg` | 4 | `crates/discourse-academic-tex/src/texparse/graph.rs:186` | - |  |
| `push_dep` | 4 | `crates/discourse-academic-tex/src/texparse/graph.rs:202` | - |  |
| `canon` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:209` | - |  |
| `file_name_lossy` | 1 | `crates/discourse-academic-tex/src/texparse/graph.rs:213` | - |  |
| `dep` | 3 | `crates/discourse-academic-tex/src/texparse/graph.rs:221` | - |  |
| `extracts_all_dep_kinds` | 0 | `crates/discourse-academic-tex/src/texparse/graph.rs:226` | - |  |
| `lex` | 1 | `crates/discourse-academic-tex/src/texparse/lex.rs:44` | - | Tokenize `src`. Never fails: every byte lands in some token. |
| `bump` | 3 | `crates/discourse-academic-tex/src/texparse/lex.rs:50` | - |  |
| `kinds` | 1 | `crates/discourse-academic-tex/src/texparse/lex.rs:148` | - |  |
| `commands_and_groups` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:153` | - |  |
| `comments_eat_to_eol` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:170` | - |  |
| `positions_are_one_based` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:176` | - |  |
| `math_shift_and_brackets` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:183` | - |  |
| `hostile_input_never_panics` | 0 | `crates/discourse-academic-tex/src/texparse/lex.rs:189` | - |  |
| `impl Magic::engine_hint` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:32` | - | Normalised build engine hinted by `program`/`ts-program`, falling back to `% !BIB program`. |
| `parse` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:56` | - | Read magic-comment header of a `.tex` source. |
| `directive` | 1 | `crates/discourse-academic-tex/src/texparse/magic.rs:80` | - |  |
| `assign` | 4 | `crates/discourse-academic-tex/src/texparse/magic.rs:98` | - |  |
| `floor_boundary` | 2 | `crates/discourse-academic-tex/src/texparse/magic.rs:121` | - |  |
| `tex_directives` | 0 | `crates/discourse-academic-tex/src/texparse/magic.rs:134` | - |  |
| `bib_directives` | 0 | `crates/discourse-academic-tex/src/texparse/magic.rs:146` | - |  |
| `analyze` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:41` | - | Compute stats for `.tex` source `src` in single O(n) pass. |
| `is_section` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:135` | - |  |
| `is_math_env` | 1 | `crates/discourse-academic-tex/src/texparse/stats.rs:139` | - |  |
| `plain_text_words_chars_lines` | 0 | `crates/discourse-academic-tex/src/texparse/stats.rs:149` | - |  |
| `inline_math_skipped_but_counted` | 0 | `crates/discourse-academic-tex/src/texparse/stats.rs:158` | - |  |
| `parse_structure` | 1 | `crates/discourse-academic-tex/src/texparse/structure.rs:73` | - | Lex `src` and extract its structure. Always succeeds — malformed input lands in `issues`. |
| `end_pos` | 1 | `crates/discourse-academic-tex/src/texparse/structure.rs:287` | - |  |
| `skip_ws` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:291` | - |  |
| `opt_arg` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:302` | - |  |
| `group_text` | 4 | `crates/discourse-academic-tex/src/texparse/structure.rs:335` | - |  |
| `push_surface` | 2 | `crates/discourse-academic-tex/src/texparse/structure.rs:379` | - |  |
| `full_document` | 0 | `crates/discourse-academic-tex/src/texparse/structure.rs:408` | - |  |
| `unclosed_environments_reported` | 0 | `crates/discourse-academic-tex/src/texparse/structure.rs:441` | - |  |
| `impl std::fmt::Display for PdfError::fmt` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:18` | - |  |
| `be16` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:25` | - |  |
| `be32` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:28` | - |  |
| `be16s` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:31` | - |  |
| `impl Ttf::parse` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:50` | - | Parse TrueType table directory + consumed tables. |
| `impl Ttf::gid` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:131` | - | Unicode scalar -> glyph ID (0 = .notdef). |
| `impl Ttf::advance` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:149` | - | Advance width of a glyph in font units. |
| `impl Ttf::char_width` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:154` | - | Advance width of a character in font units (0 for .notdef). |
| `impl Ttf::scale` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:159` | - | Font-units -> pt scale at `size` pt. |
| `impl Ttf::width_pt` | 3 | `crates/discourse-academic-tex/src/texpdf.rs:164` | - | Advance of `c` in pt at `size`. |
| `name_string` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:169` | - |  |
| `cmap12_lookup` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:210` | - |  |
| `cmap4_lookup` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:231` | - |  |
| `impl Page::new` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:287` | - |  |
| `impl Page::text` | 6 | `crates/discourse-academic-tex/src/texpdf.rs:290` | - |  |
| `impl Page::rule` | 5 | `crates/discourse-academic-tex/src/texpdf.rs:293` | - |  |
| `impl Doc::new` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:306` | - |  |
| `impl Doc::font` | 4 | `crates/discourse-academic-tex/src/texpdf.rs:310` | - |  |
| `impl Doc::add_page` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:315` | - |  |
| `impl Doc::save` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:320` | - | Serialize document to valid PDF 1.5 format. |
| `impl Default for Doc::default` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:395` | - |  |
| `out_as_string` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:400` | - |  |
| `impl std::fmt::Write for OutString<'_>::write_str` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:407` | - |  |
| `stream` | 2 | `crates/discourse-academic-tex/src/texpdf.rs:413` | - |  |
| `widths_array` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:422` | - |  |
| `tounicode_stream` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:437` | - |  |
| `page_stream` | 1 | `crates/discourse-academic-tex/src/texpdf.rs:459` | - |  |
| `document_shape_is_valid_pdf` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:482` | - |  |
| `empty_doc_is_an_error` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:497` | - |  |
| `hostile_font_bytes_error_not_panic` | 0 | `crates/discourse-academic-tex/src/texpdf.rs:502` | - |  |
| `symbol_of` | 1 | `crates/discourse-academic-tex/src/typeset.rs:59` | - | Greek + symbol command names -> their Unicode scalar. |
| `char_class` | 1 | `crates/discourse-academic-tex/src/typeset.rs:168` | - | Class of a raw character for spacing purposes. |
| `sep` | 3 | `crates/discourse-academic-tex/src/typeset.rs:179` | - | Space (in fractions of `size`) between adjacent atoms. |
| `parse` | 1 | `crates/discourse-academic-tex/src/typeset.rs:189` | - |  |
| `parse_list` | 3 | `crates/discourse-academic-tex/src/typeset.rs:198` | - |  |
| `inner` | 4 | `crates/discourse-academic-tex/src/typeset.rs:200` | - |  |
| `parse_script` | 3 | `crates/discourse-academic-tex/src/typeset.rs:288` | - |  |
| `attach_script` | 3 | `crates/discourse-academic-tex/src/typeset.rs:306` | - |  |
| `braced` | 3 | `crates/discourse-academic-tex/src/typeset.rs:325` | - |  |
| `force_regular` | 1 | `crates/discourse-academic-tex/src/typeset.rs:346` | physics |  |
| `impl Lay<'a>::text` | 5 | `crates/discourse-academic-tex/src/typeset.rs:362` | - |  |
| `layout_atom` | 5 | `crates/discourse-academic-tex/src/typeset.rs:372` | - |  |
| `width_of` | 3 | `crates/discourse-academic-tex/src/typeset.rs:443` | - |  |
| `scripts` | 6 | `crates/discourse-academic-tex/src/typeset.rs:452` | - |  |
| `ch_w` | 4 | `crates/discourse-academic-tex/src/typeset.rs:477` | - |  |
| `math` | 3 | `crates/discourse-academic-tex/src/typeset.rs:485` | - | Typeset a math fragment to positioned items. |
| `fixed` | 3 | `crates/discourse-academic-tex/src/typeset.rs:513` | - |  |
| `plain_math_lays_out` | 0 | `crates/discourse-academic-tex/src/typeset.rs:518` | - |  |
| `scripts_and_frac` | 0 | `crates/discourse-academic-tex/src/typeset.rs:525` | - |  |
| `greek_names_resolve` | 0 | `crates/discourse-academic-tex/src/typeset.rs:534` | - |  |
| `outside_subset_falls_back` | 0 | `crates/discourse-academic-tex/src/typeset.rs:539` | - |  |
| `hostile_never_panics` | 0 | `crates/discourse-academic-tex/src/typeset.rs:546` | - |  |

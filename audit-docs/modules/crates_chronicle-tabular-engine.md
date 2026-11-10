# `crates/chronicle-tabular-engine`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Calculation engine: DAG recalculation, evaluator, workbook model._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl From<Value> for Ev::from` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:31` | - |  |
| `trait Host::workbook` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:37` | - |  |
| `trait Host::cell_value` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:38` | - |  |
| `trait Host::now_serial` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:39` | - |  |
| `trait Host::random` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:40` | - |  |
| `trait Host::note_volatile` | 1 | `crates/chronicle-tabular-engine/src/calc/eval.rs:41` | - |  |
| `trait Host::spill_range` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:42` | - |  |
| `trait Host::range_values` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:45` | - |  |
| `impl Evaluator<'h>::new` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:59` | - |  |
| `impl Evaluator<'h>::eval` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:69` | - |  |
| `impl Evaluator<'h>::eval_inner` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:79` | - |  |
| `impl Evaluator<'h>::eval_ref` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:121` | - |  |
| `impl Evaluator<'h>::to_value` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:153` | - |  |
| `impl Evaluator<'h>::eval_unary` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:176` | - |  |
| `impl Evaluator<'h>::eval_binary` | 4 | `crates/chronicle-tabular-engine/src/calc/eval.rs:223` | - |  |
| `impl Evaluator<'h>::eval_call` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:275` | - |  |
| `impl Evaluator<'h>::eval_let` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:313` | - |  |
| `impl Evaluator<'h>::eval_lambda` | 2 | `crates/chronicle-tabular-engine/src/calc/eval.rs:338` | - |  |
| `impl Evaluator<'h>::eval_invoke` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:358` | - |  |
| `precedents` | 3 | `crates/chronicle-tabular-engine/src/calc/eval.rs:382` | - |  |
| `impl Default for CalcSettings::default` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:30` | - |  |
| `impl Formula::new` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:55` | - |  |
| `impl Formula::expr` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:69` | - |  |
| `impl Sheet::new` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:90` | - |  |
| `impl Sheet::value` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:100` | - |  |
| `impl Sheet::set_value` | 3 | `crates/chronicle-tabular-engine/src/calc/model.rs:107` | - |  |
| `impl Sheet::set_formula` | 3 | `crates/chronicle-tabular-engine/src/calc/model.rs:111` | - |  |
| `impl Sheet::used_range` | 1 | `crates/chronicle-tabular-engine/src/calc/model.rs:116` | - |  |
| `impl Default for Workbook::default` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:148` | - |  |
| `impl Workbook::new` | 0 | `crates/chronicle-tabular-engine/src/calc/model.rs:159` | - |  |
| `impl Workbook::sheet` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:163` | - |  |
| `impl Workbook::sheet_mut` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:167` | - |  |
| `impl Workbook::add_sheet` | 2 | `crates/chronicle-tabular-engine/src/calc/model.rs:172` | - |  |
| `impl Hasher for FxHasher::finish` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:16` | - |  |
| `impl Hasher for FxHasher::write` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:19` | - |  |
| `impl Hasher for FxHasher::write_u64` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:24` | - |  |
| `impl Graph::new` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:47` | - |  |
| `impl Graph::insert` | 3 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:51` | graph |  |
| `impl Graph::dependents` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:60` | - |  |
| `impl Default for Calc::default` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:72` | - |  |
| `impl Calc::new` | 0 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:82` | - |  |
| `impl Calc::rebuild` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:86` | - |  |
| `impl Calc::recalc_all` | 2 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:100` | - |  |
| `impl Host for PassHost<'_>::workbook` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:251` | - |  |
| `impl Host for PassHost<'_>::cell_value` | 3 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:255` | - |  |
| `impl Host for PassHost<'_>::now_serial` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:259` | - |  |
| `impl Host for PassHost<'_>::random` | 1 | `crates/chronicle-tabular-engine/src/calc/recalc.rs:263` | - |  |
| `impl CellRef::new` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:20` | - |  |
| `impl CellRef::a1` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:25` | - | `A1`-style text, e.g. `B7`. |
| `impl CellRef::is_valid` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:29` | - |  |
| `impl CellRef::offset` | 3 | `crates/chronicle-tabular-engine/src/core/addr.rs:34` | - | Moved by `(dr, dc)`, `None` when the result leaves the sheet. |
| `impl CellRef::offset_clamped` | 3 | `crates/chronicle-tabular-engine/src/core/addr.rs:45` | - | Moved by `(dr, dc)` and clamped to the sheet boundaries. |
| `impl CellRef::parse` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:52` | - | Parses `A1`, `$A$1`, `xfd1048576` (case-insensitive). `None` if malformed or out of range. |
| `impl fmt::Display for CellRef::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:62` | - |  |
| `parse_a1_prefix` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:68` | - | Parses an A1 reference at the start of `s`: `(col, row, col_abs, row_abs, rest)`. |
| `col_to_letters` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:102` | - | 0 -> `A`, 25 -> `Z`, 26 -> `AA`, 16383 -> `XFD`. |
| `letters_to_col` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:115` | - | `A` -> 0, `xfd` -> 16383. `None` for empty, non-letters or beyond `XFD`. |
| `impl RangeRef::new` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:140` | - |  |
| `impl RangeRef::cell` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:147` | - |  |
| `impl RangeRef::rows` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:152` | - | Whole rows `r0..=r1`. |
| `impl RangeRef::cols` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:157` | - | Whole columns `c0..=c1`. |
| `impl RangeRef::all` | 0 | `crates/chronicle-tabular-engine/src/core/addr.rs:161` | - |  |
| `impl RangeRef::height` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:165` | - |  |
| `impl RangeRef::width` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:169` | - |  |
| `impl RangeRef::count` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:173` | - |  |
| `impl RangeRef::is_single` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:177` | - |  |
| `impl RangeRef::is_full_rows` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:181` | - |  |
| `impl RangeRef::is_full_cols` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:185` | - |  |
| `impl RangeRef::contains` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:189` | - |  |
| `impl RangeRef::contains_range` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:193` | - |  |
| `impl RangeRef::intersects` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:197` | - |  |
| `impl RangeRef::intersection` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:201` | computational geometry |  |
| `impl RangeRef::union` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:211` | - |  |
| `impl RangeRef::iter` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:219` | - | Row-major iterator over the cells. |
| `impl RangeRef::a1` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:225` | - | `A1:B2`, `A1` for single cells, `1:3` for full rows, `A:C` for full columns. |
| `impl RangeRef::parse` | 1 | `crates/chronicle-tabular-engine/src/core/addr.rs:240` | - | Parses `A1`, `A1:B2`, `$A$1:$B$2`, `A:C`, `3:5` (no sheet prefix). |
| `impl fmt::Display for RangeRef::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/addr.rs:261` | - |  |
| `days_from_civil` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:31` | - | Days from 0000-03-01 style civil algorithm (Howard Hinnant), days since 1970-01-01. |
| `civil_from_days` | 1 | `crates/chronicle-tabular-engine/src/core/date.rs:41` | - |  |
| `is_leap` | 1 | `crates/chronicle-tabular-engine/src/core/date.rs:57` | - |  |
| `days_in_month` | 2 | `crates/chronicle-tabular-engine/src/core/date.rs:61` | - |  |
| `days_in_month_in` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:71` | - |  |
| `serial_from_ymd` | 4 | `crates/chronicle-tabular-engine/src/core/date.rs:79` | - |  |
| `datetime_from_serial` | 2 | `crates/chronicle-tabular-engine/src/core/date.rs:101` | - |  |
| `time_fraction` | 3 | `crates/chronicle-tabular-engine/src/core/date.rs:143` | - |  |
| `parse_input` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:12` | - |  |
| `parse_number_text` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:56` | - |  |
| `parse_numeric` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:60` | - |  |
| `parse_plain` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:79` | - |  |
| `parse_fraction` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:121` | - |  |
| `parse_datetime` | 2 | `crates/chronicle-tabular-engine/src/core/parse.rs:146` | - |  |
| `parse_iso_date` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:161` | - |  |
| `parse_us_date` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:174` | - |  |
| `parse_time_only` | 1 | `crates/chronicle-tabular-engine/src/core/parse.rs:190` | - |  |
| `impl CellError::as_str` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:39` | - |  |
| `impl CellError::code` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:55` | - |  |
| `impl CellError::parse` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:71` | - |  |
| `impl fmt::Display for CellError::fmt` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:78` | - |  |
| `impl Array::new` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:92` | - |  |
| `impl Array::filled` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:99` | - |  |
| `impl Array::single` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:108` | - |  |
| `impl Array::get` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:116` | - |  |
| `impl Array::get_mut` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:124` | - |  |
| `impl Value::number` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:147` | - |  |
| `impl Value::text` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:151` | - |  |
| `impl Value::is_empty` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:155` | - |  |
| `impl Value::is_error` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:159` | - |  |
| `impl Value::as_f64` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:163` | - |  |
| `impl Value::as_str` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:172` | - |  |
| `impl Value::to_display_string` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:179` | - |  |
| `number_to_text` | 1 | `crates/chronicle-tabular-engine/src/core/value.rs:197` | - |  |
| `compare_numbers` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:215` | - |  |
| `compare_text` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:219` | - |  |
| `compare` | 2 | `crates/chronicle-tabular-engine/src/core/value.rs:223` | - |  |
| `sort_compare` | 3 | `crates/chronicle-tabular-engine/src/core/value.rs:254` | - |  |
| `shift_relative` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:7` | - |  |
| `shift_anchor` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:17` | - |  |
| `shift_line` | 4 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:30` | - |  |
| `shift_ref` | 3 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:42` | - |  |
| `adjust` | 4 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:72` | - |  |
| `adjust_ref` | 2 | `crates/chronicle-tabular-engine/src/formula/adjust.rs:92` | - |  |
| `impl Anchor::cell` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:19` | - |  |
| `impl Reference::range` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:50` | - |  |
| `impl Reference::sheet_name` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:59` | - |  |
| `impl BinOp::symbol` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:119` | - |  |
| `impl BinOp::precedence` | 1 | `crates/chronicle-tabular-engine/src/formula/ast.rs:139` | - |  |
| `impl Expr::walk` | 2 | `crates/chronicle-tabular-engine/src/formula/ast.rs:172` | - |  |
| `impl Expr::map` | 2 | `crates/chronicle-tabular-engine/src/formula/ast.rs:190` | - |  |
| `impl fmt::Display for LexError::fmt` | 2 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:42` | - |  |
| `is_word_char` | 1 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:49` | - |  |
| `tokenize` | 1 | `crates/chronicle-tabular-engine/src/formula/lexer.rs:53` | - |  |
| `impl fmt::Display for ParseError::fmt` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:18` | - |  |
| `parse` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:25` | - |  |
| `filter_spaces` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:37` | - |  |
| `impl Parser::peek` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:47` | - |  |
| `impl Parser::pos` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:51` | - |  |
| `impl Parser::next` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:55` | - |  |
| `impl Parser::err` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:65` | - |  |
| `impl Parser::expr` | 2 | `crates/chronicle-tabular-engine/src/formula/parser.rs:72` | - |  |
| `impl Parser::primary` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:118` | - |  |
| `normalize_function_name` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:238` | - |  |
| `parse_cell_reference` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:248` | - |  |
| `parse_col_reference` | 1 | `crates/chronicle-tabular-engine/src/formula/parser.rs:256` | - |  |
| `print` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:8` | - |  |
| `needs_quotes` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:14` | - |  |
| `quote_sheet` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:25` | - |  |
| `sheet_prefix` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:33` | - |  |
| `anchor_a1` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:48` | - |  |
| `reference_a1` | 1 | `crates/chronicle-tabular-engine/src/formula/printer.rs:58` | - |  |
| `write_expr` | 3 | `crates/chronicle-tabular-engine/src/formula/printer.rs:78` | - |  |
| `sequence` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:7` | - |  |
| `filter` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:30` | - |  |
| `unique` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:94` | - |  |
| `sort` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:159` | - |  |
| `take` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:204` | - |  |
| `drop` | 1 | `crates/chronicle-tabular-engine/src/functions/dynamic.rs:238` | - |  |
| `pmt_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:13` | - |  |
| `fv_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:21` | - |  |
| `pv_calc` | 5 | `crates/chronicle-tabular-engine/src/functions/financial.rs:29` | - |  |
| `ipmt_calc` | 6 | `crates/chronicle-tabular-engine/src/functions/financial.rs:37` | - |  |
| `pmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:47` | - |  |
| `fv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:57` | - |  |
| `pv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:64` | - |  |
| `nper` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:71` | - |  |
| `ipmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:93` | - |  |
| `ppmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:103` | - |  |
| `cumipmt` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:115` | - |  |
| `cumprinc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:130` | - |  |
| `solve_rate` | 2 | `crates/chronicle-tabular-engine/src/functions/financial.rs:151` | numerics |  |
| `rate` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:199` | - |  |
| `npv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:218` | - |  |
| `dated_flows` | 2 | `crates/chronicle-tabular-engine/src/functions/financial.rs:245` | - |  |
| `xnpv_calc` | 3 | `crates/chronicle-tabular-engine/src/functions/financial.rs:266` | - |  |
| `xnpv` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:271` | - |  |
| `xirr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:280` | - |  |
| `irr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:290` | - |  |
| `mirr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:314` | - |  |
| `effect` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:346` | - |  |
| `nominal` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:355` | - |  |
| `dollarde` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:364` | - |  |
| `dollarfr` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:374` | - |  |
| `sln` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:388` | - |  |
| `syd` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:396` | - |  |
| `db` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:404` | - |  |
| `ddb` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:426` | - |  |
| `vdb` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:442` | - |  |
| `duration` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:462` | - |  |
| `mduration` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:485` | - |  |
| `price_bond` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:496` | economics |  |
| `yield_maturity` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:513` | - |  |
| `disc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:535` | - |  |
| `tbilleq` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:546` | - |  |
| `tbillprice` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:555` | - |  |
| `tbillyield` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:564` | - |  |
| `received` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:573` | - |  |
| `intrate` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:584` | - |  |
| `pricedisc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:595` | - |  |
| `yielddisc` | 1 | `crates/chronicle-tabular-engine/src/functions/financial.rs:606` | - |  |
| `match_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:6` | - |  |
| `index_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:46` | - |  |
| `xlookup` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:65` | - |  |
| `vlookup` | 1 | `crates/chronicle-tabular-engine/src/functions/lookup.rs:139` | - |  |
| `sum` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:6` | - |  |
| `product` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:12` | - |  |
| `abs` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:21` | - |  |
| `round` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:26` | - |  |
| `floor` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:33` | - |  |
| `ceiling` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:41` | - |  |
| `mod_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:49` | - |  |
| `sqrt` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:57` | - |  |
| `power` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:65` | - |  |
| `ln` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:70` | - |  |
| `log10` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:78` | - |  |
| `exp_fn` | 1 | `crates/chronicle-tabular-engine/src/functions/math.rs:86` | - |  |
| `dispatch` | 3 | `crates/chronicle-tabular-engine/src/functions/mod.rs:14` | - |  |
| `std_norm_pdf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:12` | 3d | Standard normal PDF: phi(x) |
| `erf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:17` | - | Error function erf(x) via Chebyshev approximation |
| `std_norm_cdf` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:33` | 3d | Standard normal CDF: Phi(x) |
| `std_norm_inv` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:38` | 3d | Inverse standard normal CDF (Acklam approximation) |
| `norm_s_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:91` | - |  |
| `norm_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:101` | - |  |
| `norm_inv` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:115` | - |  |
| `poisson_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:128` | - |  |
| `gamma_ln` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:156` | - |  |
| `gamma_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:177` | - |  |
| `weibull_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:199` | - |  |
| `chisq_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:213` | - |  |
| `t_dist` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:222` | - |  |
| `average` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:253` | - |  |
| `count` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:262` | - |  |
| `max` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:267` | - |  |
| `min` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:276` | - |  |
| `var_s` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:285` | - |  |
| `stdev_s` | 1 | `crates/chronicle-tabular-engine/src/functions/stat.rs:295` | - |  |
| `concat` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:6` | - |  |
| `textjoin` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:14` | - |  |
| `left` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:40` | - |  |
| `right` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:47` | - |  |
| `mid` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:56` | - |  |
| `len` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:67` | - |  |
| `trim` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:72` | - |  |
| `upper` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:78` | - |  |
| `lower` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:83` | - |  |
| `exact` | 1 | `crates/chronicle-tabular-engine/src/functions/text.rs:88` | - |  |
| `num` | 2 | `crates/chronicle-tabular-engine/src/functions/util.rs:8` | - |  |
| `opt_num` | 3 | `crates/chronicle-tabular-engine/src/functions/util.rs:13` | - |  |
| `to_f64` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:20` | - |  |
| `flatten_numbers` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:37` | - |  |
| `as_array` | 1 | `crates/chronicle-tabular-engine/src/functions/util.rs:56` | - |  |
| `builtin_format` | 1 | `crates/chronicle-tabular-engine/src/numfmt/builtin.rs:42` | - |  |
| `builtin_id` | 1 | `crates/chronicle-tabular-engine/src/numfmt/builtin.rs:46` | - |  |
| `decimal_digits` | 1 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:5` | - |  |
| `fixed_from_digits` | 3 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:18` | - |  |
| `fixed` | 2 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:64` | - |  |
| `format_general_fit` | 2 | `crates/chronicle-tabular-engine/src/numfmt/decimal.rs:72` | - |  |
| `impl Default for NumberFormat::default` | 0 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:24` | - |  |
| `impl NumberFormat::parse` | 1 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:30` | - |  |
| `impl NumberFormat::general` | 0 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:38` | - |  |
| `impl NumberFormat::format` | 3 | `crates/chronicle-tabular-engine/src/numfmt/mod.rs:42` | - |  |
| `impl DatePart::is_calendar` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:18` | - |  |
| `impl Tok::literal_text` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:40` | - |  |
| `parse_sections` | 1 | `crates/chronicle-tabular-engine/src/numfmt/parse.rs:89` | - |  |
| `impl Out::push` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:14` | - |  |
| `impl Out::prepend` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:17` | - |  |
| `render_text` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:22` | - |  |
| `render_general` | 3 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:34` | - |  |
| `render_date` | 3 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:46` | - |  |
| `render_number` | 2 | `crates/chronicle-tabular-engine/src/numfmt/render.rs:109` | - |  |
| `impl OntologicalTriple::new` | 4 | `crates/chronicle-tabular-engine/src/ontology.rs:21` | - |  |
| `impl OntologicalTriple::verify_parity` | 1 | `crates/chronicle-tabular-engine/src/ontology.rs:32` | - |  |
| `test_formula_parsing` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:6` | - |  |
| `test_dag_recalculation_and_cycles` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:14` | - |  |
| `test_financial_pmt_npv_xirr` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:42` | - |  |
| `test_dynamic_arrays_and_spill` | 0 | `crates/chronicle-tabular-engine/tests/tabular_tests.rs:87` | - |  |

# `crates/kinetics-schedule-cpm`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_`kinetics-schedule-cpm`

UI-agnostic project schedule and CPM computation engine:
- Unified document model (`model`)
- CPM forward/backward passes, EVM, levelling, Monte Carlo (`schedule`)
- Multi-format import/export & SVG Gantt chart renderer (`format`)
- Content-addressed immutable change DAG and role enforcement (`change_dag`)_

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl SyncRole::as_str` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:35` | - |  |
| `impl SyncRole::parse` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:45` | - |  |
| `impl SyncRole::can_read` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:55` | - |  |
| `impl SyncRole::can_comment` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:59` | - |  |
| `impl SyncRole::can_write_tasks` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:63` | - |  |
| `impl SyncRole::can_manage_roles` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:67` | - |  |
| `impl SyncRole::allows_command` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:71` | - |  |
| `impl fmt::Display for SyncError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:106` | - |  |
| `impl ChangeEnvelope::new` | 6 | `crates/kinetics-schedule-cpm/src/change_dag.rs:150` | - |  |
| `impl ChangeDag::new` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:188` | - |  |
| `impl ChangeDag::record_command` | 5 | `crates/kinetics-schedule-cpm/src/change_dag.rs:192` | - |  |
| `impl ChangeDag::add_comment` | 5 | `crates/kinetics-schedule-cpm/src/change_dag.rs:216` | - |  |
| `impl ChangeDag::merge_remote` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:250` | - |  |
| `impl ChangeDag::inspect_post_merge_conflicts` | 2 | `crates/kinetics-schedule-cpm/src/change_dag.rs:269` | - |  |
| `extract_mentions` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:304` | - |  |
| `fnv1a_hex` | 1 | `crates/kinetics-schedule-cpm/src/change_dag.rs:320` | - |  |
| `change_dag_records_and_enforces_roles` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:334` | - |  |
| `comments_and_mentions` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:344` | - |  |
| `merge_remote_idempotent` | 0 | `crates/kinetics-schedule-cpm/src/change_dag.rs:353` | - |  |
| `impl fmt::Display for FormatError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:28` | - |  |
| `check_size` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:38` | - |  |
| `to_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:47` | - |  |
| `from_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:51` | - |  |
| `to_csv` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:62` | - |  |
| `from_csv` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:106` | - |  |
| `parse_pred_token` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:187` | - |  |
| `csv_escape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:207` | - |  |
| `split_csv_row` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:215` | - |  |
| `to_jira_json` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:242` | - |  |
| `to_mspdi_xml` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:292` | - |  |
| `from_mspdi_xml` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:356` | - |  |
| `parse_iso8601_duration_days` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:437` | - |  |
| `xml_escape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:456` | - |  |
| `xml_unescape` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:460` | - |  |
| `extract_tag` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:464` | - |  |
| `extract_blocks` | 2 | `crates/kinetics-schedule-cpm/src/format.rs:472` | - |  |
| `to_primavera_xer` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:491` | - |  |
| `from_primavera_xer` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:556` | - |  |
| `clean_tab` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:668` | - |  |
| `render_gantt_svg` | 1 | `crates/kinetics-schedule-cpm/src/format.rs:672` | - |  |
| `json_and_csv_round_trip` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:764` | - |  |
| `mspdi_and_xer_export` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:777` | - |  |
| `svg_gantt_renders` | 0 | `crates/kinetics-schedule-cpm/src/format.rs:795` | - |  |
| `impl fmt::Display for CoreError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:19` | - |  |
| `impl ItemKind::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:43` | - |  |
| `impl ItemKind::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:54` | - |  |
| `impl TaskStatus::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:88` | - |  |
| `impl TaskStatus::label` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:99` | - |  |
| `impl TaskStatus::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:110` | - |  |
| `impl Priority::as_str` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:133` | - |  |
| `impl Priority::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:142` | - |  |
| `impl Priority::weight` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:151` | - |  |
| `impl DepKind::code` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:175` | - |  |
| `impl DepKind::parse` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:184` | - |  |
| `impl TaskConstraint::label` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:209` | - |  |
| `impl TaskConstraint::parse` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:222` | - |  |
| `impl fmt::Display for FieldValue::fmt` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:247` | - |  |
| `impl Task::new` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:302` | - |  |
| `impl Task::pert_expected_days` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:339` | - |  |
| `impl Task::pert_std_dev` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:346` | - |  |
| `impl Task::wbs_depth` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:353` | - |  |
| `impl Resource::rate_on_day` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:389` | - |  |
| `impl Default for Calendar::default` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:421` | - |  |
| `impl Calendar::work_day_to_calendar_day` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:433` | - |  |
| `impl Default for Project::default` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:509` | - |  |
| `impl Project::new` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:515` | - |  |
| `impl Project::repair_counters` | 1 | `crates/kinetics-schedule-cpm/src/model.rs:541` | - |  |
| `impl Project::add_task` | 4 | `crates/kinetics-schedule-cpm/src/model.rs:551` | - |  |
| `impl Project::task` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:568` | - |  |
| `impl Project::task_mut` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:572` | - |  |
| `impl Project::delete_task` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:576` | - |  |
| `impl Project::add_dependency` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:592` | - |  |
| `impl Project::remove_dependency` | 3 | `crates/kinetics-schedule-cpm/src/model.rs:611` | - |  |
| `impl Project::add_resource` | 5 | `crates/kinetics-schedule-cpm/src/model.rs:617` | - |  |
| `impl Project::assign_resource` | 4 | `crates/kinetics-schedule-cpm/src/model.rs:644` | - |  |
| `impl Project::add_sprint` | 6 | `crates/kinetics-schedule-cpm/src/model.rs:660` | - |  |
| `impl Project::planned_task_cost` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:674` | - |  |
| `impl Project::capture_baseline` | 2 | `crates/kinetics-schedule-cpm/src/model.rs:688` | - |  |
| `impl Project::sample_launch_program` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:706` | - |  |
| `sample_program_builds_and_tracks_costs` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:865` | - |  |
| `calendar_skips_weekends_and_holidays` | 0 | `crates/kinetics-schedule-cpm/src/model.rs:874` | - |  |
| `impl fmt::Display for SchedError::fmt` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:33` | - |  |
| `topological_sort` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:53` | graph | Topologically sort task indices, returning [`SchedError::CycleDetected`] if any cycle exists. |
| `schedule_project` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:104` | - | Run CPM forward and backward pass on `project`, updating every task's schedule fields in place. |
| `compute_baseline_variance` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:287` | - |  |
| `impl WorkloadReport::total_overallocated_days` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:338` | - |  |
| `compute_workload` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:343` | - |  |
| `level_resources` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:412` | - |  |
| `compute_evm` | 2 | `crates/kinetics-schedule-cpm/src/schedule.rs:515` | - |  |
| `planned_fraction_at` | 4 | `crates/kinetics-schedule-cpm/src/schedule.rs:584` | - |  |
| `impl XorShift64::new` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:632` | - |  |
| `impl XorShift64::next_u64` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:636` | - |  |
| `impl XorShift64::next_f64` | 1 | `crates/kinetics-schedule-cpm/src/schedule.rs:645` | - |  |
| `impl XorShift64::sample_triangular` | 4 | `crates/kinetics-schedule-cpm/src/schedule.rs:649` | - |  |
| `run_monte_carlo` | 3 | `crates/kinetics-schedule-cpm/src/schedule.rs:663` | - |  |
| `cpm_schedules_sample_program` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:840` | - |  |
| `evm_computation` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:848` | - |  |
| `resource_levelling` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:858` | - |  |
| `monte_carlo_simulation` | 0 | `crates/kinetics-schedule-cpm/src/schedule.rs:865` | - |  |

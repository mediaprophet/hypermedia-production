//! `kinetics-schedule-cpm`
//!
//! UI-agnostic project schedule and CPM computation engine:
//! - Unified document model (`model`)
//! - CPM forward/backward passes, EVM, levelling, Monte Carlo (`schedule`)
//! - Multi-format import/export & SVG Gantt chart renderer (`format`)
//! - Content-addressed immutable change DAG and role enforcement (`change_dag`)

#![forbid(unsafe_code)]

pub mod change_dag;
pub mod format;
pub mod model;
pub mod schedule;

pub use change_dag::{ChangeDag, ChangeEnvelope, SemanticConflict, SyncError, SyncRole, TaskComment};
pub use format::{
    from_csv, from_json, from_mspdi_xml, from_primavera_xer, render_gantt_svg,
    to_csv, to_jira_json, to_json, to_mspdi_xml, to_primavera_xer, FormatError,
};
pub use model::{
    Assignment, Baseline, BaselineTask, Calendar, CoreError, DepKind, Dependency, FieldValue,
    ItemKind, Priority, Project, RateEntry, Resource, Sprint, Task, TaskConstraint, TaskStatus,
};
pub use schedule::{
    compute_baseline_variance, compute_evm, compute_workload, level_resources, run_monte_carlo,
    schedule_project, topological_sort, EvmCurvePoint, EvmMetrics, LevelAction, LevelResult,
    MonteCarloReport, ResourceWorkload, SchedError, ScheduleSummary, TaskCriticality, TaskVariance,
    WorkloadReport,
};

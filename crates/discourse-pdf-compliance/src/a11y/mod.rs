//! PDF/UA and WCAG accessibility compliance module.

#![forbid(unsafe_code)]

pub mod checker;
pub mod report;
pub mod rules;

pub use checker::{
    AccessibleDocSnapshot, AccessibleFigure, AccessibleFormField, AccessibleHeading, AccessibleList,
    AccessibleTable, RuleVerdict, Status, check_accessibility,
};
pub use report::{AuditSummary, generate_html_report, summarize};
pub use rules::{Category, Rule};

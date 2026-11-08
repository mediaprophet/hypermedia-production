//! Structured accessibility audit report generator (HTML and structured JSON summary).

#![forbid(unsafe_code)]

use std::fmt::Write as _;

use super::checker::{RuleVerdict, Status};
use super::rules::Category;

#[derive(Clone, Debug, PartialEq)]
pub struct AuditSummary {
    pub passed: usize,
    pub failed: usize,
    pub needs_manual_check: usize,
    pub skipped: usize,
}

pub fn summarize(verdicts: &[RuleVerdict]) -> AuditSummary {
    let mut s = AuditSummary {
        passed: 0,
        failed: 0,
        needs_manual_check: 0,
        skipped: 0,
    };
    for v in verdicts {
        match v.status {
            Status::Passed => s.passed += 1,
            Status::Failed => s.failed += 1,
            Status::NeedsManualCheck => s.needs_manual_check += 1,
            Status::Skipped => s.skipped += 1,
        }
    }
    s
}

pub fn generate_html_report(title: &str, verdicts: &[RuleVerdict]) -> String {
    let mut out = String::new();
    let summary = summarize(verdicts);

    let _ = writeln!(out, "<!DOCTYPE html>");
    let _ = writeln!(out, "<html lang=\"en\">");
    let _ = writeln!(out, "<head><meta charset=\"utf-8\"><title>Accessibility Audit Report: {}</title>", title);
    let _ = writeln!(out, "<style>");
    let _ = writeln!(out, "body {{ font-family: system-ui, -apple-system, sans-serif; margin: 2rem; color: #1e293b; background: #f8fafc; }}");
    let _ = writeln!(out, ".card {{ background: white; border-radius: 8px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,0.1); }}");
    let _ = writeln!(out, ".badge {{ display: inline-block; padding: 0.25rem 0.5rem; border-radius: 4px; font-weight: 600; font-size: 0.85rem; }}");
    let _ = writeln!(out, ".pass {{ background: #dcfce7; color: #166534; }}");
    let _ = writeln!(out, ".fail {{ background: #fee2e2; color: #991b1b; }}");
    let _ = writeln!(out, ".manual {{ background: #fef3c7; color: #92400e; }}");
    let _ = writeln!(out, ".rule-item {{ padding: 0.75rem 0; border-bottom: 1px solid #e2e8f0; }}");
    let _ = writeln!(out, "</style></head><body>");

    let _ = writeln!(out, "<h1>PDF/UA & WCAG Accessibility Audit</h1>");
    let _ = writeln!(out, "<h2>Document: {}</h2>", title);

    let _ = writeln!(out, "<div class=\"card\">");
    let _ = writeln!(out, "<h3>Summary</h3>");
    let _ = writeln!(out, "<p><span class=\"badge pass\">{} Passed</span> <span class=\"badge fail\">{} Failed</span> <span class=\"badge manual\">{} Manual Check</span></p>", summary.passed, summary.failed, summary.needs_manual_check);
    let _ = writeln!(out, "</div>");

    for cat in Category::ALL {
        let cat_verdicts: Vec<_> = verdicts.iter().filter(|v| v.rule.category() == cat).collect();
        if cat_verdicts.is_empty() {
            continue;
        }
        let _ = writeln!(out, "<div class=\"card\">");
        let _ = writeln!(out, "<h3>{}</h3>", cat.label());
        for v in cat_verdicts {
            let (cls, text) = match v.status {
                Status::Passed => ("pass", "Passed"),
                Status::Failed => ("fail", "Failed"),
                Status::NeedsManualCheck => ("manual", "Needs Manual Check"),
                Status::Skipped => ("manual", "Skipped"),
            };
            let _ = writeln!(out, "<div class=\"rule-item\">");
            let _ = writeln!(out, "<span class=\"badge {}\">{}</span> <strong>{}</strong>", cls, text, v.rule.title());
            if !v.details.is_empty() {
                let _ = writeln!(out, "<ul>");
                for d in &v.details {
                    let _ = writeln!(out, "<li>{}</li>", d);
                }
                let _ = writeln!(out, "</ul>");
            }
            let _ = writeln!(out, "</div>");
        }
        let _ = writeln!(out, "</div>");
    }

    let _ = writeln!(out, "</body></html>");
    out
}

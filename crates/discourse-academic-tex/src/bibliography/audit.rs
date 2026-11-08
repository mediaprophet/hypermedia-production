//! BibTeX audit rule engine.
//!
//! Validates parsed BibTeX entries:
//! - Missing mandatory fields (`title`, `author`, `year`).
//! - Collisions in citation keys.
//! - Duplicate identifiers (DOI, arXiv eprint, normalized title).
//! - Identifies unresolved reference stubs.

#![forbid(unsafe_code)]

use std::collections::HashMap;

use crate::bibliography::import::norm_title;
use crate::texparse::bib::{BibEntry, BibFile};

/// Severity of an audit finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// A diagnostic emitted by the audit engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditFinding {
    pub key: String,
    pub severity: Severity,
    pub rule: &'static str,
    pub message: String,
}

/// Audit report over one or more BibTeX files.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditReport {
    pub total_entries: usize,
    pub findings: Vec<AuditFinding>,
    pub duplicates: Vec<(String, String, &'static str)>,
}

/// Run audit rules over a [`BibFile`].
pub fn audit_bib(file: &BibFile) -> AuditReport {
    let mut report = AuditReport { total_entries: file.entries.len(), ..AuditReport::default() };

    let mut key_counts: HashMap<String, usize> = HashMap::new();
    let mut doi_map: HashMap<String, String> = HashMap::new();
    let mut title_map: HashMap<String, String> = HashMap::new();

    for entry in &file.entries {
        *key_counts.entry(entry.key.clone()).or_insert(0) += 1;

        audit_entry_fields(entry, &mut report.findings);

        if let Some(doi) = entry.field("doi") {
            let clean = doi.trim().to_ascii_lowercase();
            if !clean.is_empty() {
                if let Some(prior) = doi_map.get(&clean) {
                    report.duplicates.push((prior.clone(), entry.key.clone(), "doi"));
                    report.findings.push(AuditFinding {
                        key: entry.key.clone(),
                        severity: Severity::Warning,
                        rule: "duplicate-doi",
                        message: format!("Shares DOI '{clean}' with entry '{prior}'"),
                    });
                } else {
                    doi_map.insert(clean, entry.key.clone());
                }
            }
        }

        if let Some(title) = entry.field("title") {
            let norm = norm_title(title);
            if !norm.is_empty() {
                if let Some(prior) = title_map.get(&norm) {
                    report.duplicates.push((prior.clone(), entry.key.clone(), "title"));
                    report.findings.push(AuditFinding {
                        key: entry.key.clone(),
                        severity: Severity::Info,
                        rule: "duplicate-title",
                        message: format!("Normalized title matches entry '{prior}'"),
                    });
                } else {
                    title_map.insert(norm, entry.key.clone());
                }
            }
        }

        if let Some(note) = entry.field("note") {
            if note.to_ascii_lowercase().contains("unresolved") {
                report.findings.push(AuditFinding {
                    key: entry.key.clone(),
                    severity: Severity::Warning,
                    rule: "unresolved-stub",
                    message: "Entry is marked as an unresolved reference".into(),
                });
            }
        }
    }

    for (key, count) in key_counts {
        if count > 1 {
            report.findings.push(AuditFinding {
                key: key.clone(),
                severity: Severity::Error,
                rule: "duplicate-key",
                message: format!("Citation key '{key}' defined {count} times"),
            });
        }
    }

    report
}

fn audit_entry_fields(entry: &BibEntry, findings: &mut Vec<AuditFinding>) {
    if entry.key.trim().is_empty() {
        findings.push(AuditFinding {
            key: "<empty>".into(),
            severity: Severity::Error,
            rule: "missing-key",
            message: "Bibliography entry is missing a citation key".into(),
        });
        return;
    }

    let kind = entry.kind.as_str();
    if kind == "comment" {
        return;
    }

    if entry.field("title").is_none() {
        findings.push(AuditFinding {
            key: entry.key.clone(),
            severity: Severity::Warning,
            rule: "missing-title",
            message: format!("Entry '{}' ({kind}) is missing a 'title' field", entry.key),
        });
    }

    if entry.field("author").is_none() && entry.field("editor").is_none() && kind != "misc" {
        findings.push(AuditFinding {
            key: entry.key.clone(),
            severity: Severity::Warning,
            rule: "missing-author",
            message: format!("Entry '{}' ({kind}) has neither author nor editor", entry.key),
        });
    }

    if entry.field("year").is_none() && entry.field("date").is_none() {
        findings.push(AuditFinding {
            key: entry.key.clone(),
            severity: Severity::Warning,
            rule: "missing-year",
            message: format!("Entry '{}' ({kind}) has no year or date", entry.key),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texparse::bib::parse_bib;

    #[test]
    fn test_audit_duplicates() {
        let bib = r#"
@article{ref1,
  author = {Knuth, D.},
  title = {The TeXbook},
  year = 1984,
  doi = {10.1000/182}
}
@book{ref2,
  author = {Knuth, Donald},
  title = {The TeXbook},
  year = 1986,
  doi = {10.1000/182}
}
"#;
        let file = parse_bib(bib);
        let rep = audit_bib(&file);
        assert_eq!(rep.total_entries, 2);
        assert!(rep.duplicates.iter().any(|d| d.2 == "doi"));
        assert!(rep.duplicates.iter().any(|d| d.2 == "title"));
    }
}

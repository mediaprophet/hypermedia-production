//! Deterministic model-free compile repair engine.
//!
//! Grounded in authoring rights and scientific preservation (UDHR Art. 19, 27; ICESCR Art. 15).
//!
//! This module analyzes structural issues detected by `texparse` and proposes
//! minimal, convergent line-level patches.
//!
//! Handled repair classes:
//! - Unclosed environments (closes innermost first before `\end{document}` or at EOF)
//! - `\end{...}` mismatches (rewrites mismatched environment name)
//! - Stray `\end{...}` commands without matching `\begin` (removes or blanks line)

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::texparse::DocStructure;

pub const MAX_EDITS: usize = 32;
pub const MAX_EDIT_TEXT: usize = 4096;

/// Specific line-level edit operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditKind {
    InsertBefore,
    ReplaceLine,
    DeleteLine,
}

/// A deterministic edit proposed to repair a structural defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditOp {
    pub kind: EditKind,
    /// 1-based line number.
    pub line: usize,
    /// Existing content at `line` that must match for verification.
    pub before: String,
    /// New text (for `InsertBefore` and `ReplaceLine`).
    pub after: Option<String>,
    /// Human-readable rationale for this fix.
    pub reason: String,
}

/// Propose a set of deterministic line edits to repair document structure.
pub fn propose_edits(d: &DocStructure, lines: &[&str]) -> Vec<EditOp> {
    let mut unclosed: Vec<String> = Vec::new();
    for issue in &d.issues {
        if let Some(name) = parse_unclosed_begin(&issue.message) {
            unclosed.push(name);
        }
    }

    let mut edits = Vec::new();
    let mut closed: Vec<String> = Vec::new();

    for issue in &d.issues {
        if edits.len() >= MAX_EDITS {
            break;
        }
        let message = issue.message.as_str();
        let line = issue.pos.line;

        if parse_unclosed_begin(message).is_some() {
            continue;
        }

        if let Some((got, want)) = parse_end_mismatch(message) {
            if closed.iter().any(|n| n == &want) {
                if unclosed.iter().any(|n| n == &got) {
                    closed.push(got);
                } else {
                    stray_end_edit(&mut edits, lines, line, &got);
                }
            } else if has_later_end(lines, &want, line) {
                stray_end_edit(&mut edits, lines, line, &got);
            } else if got == "document" {
                insert_before_edit(
                    &mut edits,
                    lines,
                    line,
                    &format!("\\end{{{want}}}"),
                    &format!("close \\begin{{{want}}} before \\end{{document}}"),
                );
                closed.push(want);
                closed.push("document".to_string());
            } else {
                let before = line_at(lines, line);
                let after = before.replacen(&format!("\\end{{{got}}}"), &format!("\\end{{{want}}}"), 1);
                if after != before {
                    closed.push(want.clone());
                    edits.push(EditOp {
                        kind: EditKind::ReplaceLine,
                        line,
                        before: before.to_string(),
                        after: Some(after),
                        reason: format!("\\end{{{got}}} does not match open \\begin{{{want}}}"),
                    });
                }
            }
            continue;
        }

        if let Some(name) = parse_stray_end(message) {
            stray_end_edit(&mut edits, lines, line, &name);
        }
    }

    // Close remaining open environments, innermost first
    let mut texts = Vec::new();
    for name in unclosed.iter().rev() {
        if let Some(i) = closed.iter().position(|n| n == name) {
            closed.remove(i);
            continue;
        }
        texts.push(format!("\\end{{{name}}}"));
    }

    if !texts.is_empty() && edits.len() < MAX_EDITS {
        let anchor = d.envs.iter().find(|e| e.name == "document").map(|e| e.end.line);
        let line = anchor.unwrap_or_else(|| eof_anchor(lines));
        insert_before_edit(&mut edits, lines, line, &texts.join("\n"), "close unclosed environment(s)");
    }

    edits.retain(|e| e.after.as_ref().map(|a| a.len() <= MAX_EDIT_TEXT).unwrap_or(true));
    edits
}

fn insert_before_edit(edits: &mut Vec<EditOp>, lines: &[&str], line: usize, text: &str, reason: &str) {
    edits.push(EditOp {
        kind: EditKind::InsertBefore,
        line,
        before: line_at(lines, line).to_string(),
        after: Some(text.to_string()),
        reason: reason.to_string(),
    });
}

fn stray_end_edit(edits: &mut Vec<EditOp>, lines: &[&str], line: usize, name: &str) {
    let before = line_at(lines, line);
    let after = before.replacen(&format!("\\end{{{name}}}"), "", 1);
    if after == before {
        return;
    }
    if after.trim().is_empty() {
        edits.push(EditOp {
            kind: EditKind::DeleteLine,
            line,
            before: before.to_string(),
            after: None,
            reason: format!("\\end{{{name}}} has no matching \\begin"),
        });
    } else {
        edits.push(EditOp {
            kind: EditKind::ReplaceLine,
            line,
            before: before.to_string(),
            after: Some(after),
            reason: format!("\\end{{{name}}} has no matching \\begin"),
        });
    }
}

/// Apply a sequence of edits bottom-up so line numbers remain stable.
pub fn apply_edits(src: &str, edits: &[EditOp]) -> Result<String, String> {
    let mut lines: Vec<String> = src.split('\n').map(str::to_string).collect();
    let mut order: Vec<usize> = (0..edits.len()).collect();
    order.sort_by(|a, b| {
        let la = edits[*a].line;
        let lb = edits[*b].line;
        lb.cmp(&la).then_with(|| op_rank(&edits[*a]).cmp(&op_rank(&edits[*b])))
    });

    for &i in &order {
        let edit = &edits[i];
        let line = edit.line;
        let before = &edit.before;
        let current = lines.get(line.saturating_sub(1)).map(String::as_str).unwrap_or("");
        if current != before {
            return Err(format!("buffer changed since preview at line {line}"));
        }
        match edit.kind {
            EditKind::InsertBefore => {
                let after = edit.after.as_deref().unwrap_or_default();
                lines.insert(line.saturating_sub(1).min(lines.len()), after.to_string());
            }
            EditKind::ReplaceLine => {
                let after = edit.after.as_deref().unwrap_or_default();
                if line == 0 || line > lines.len() {
                    return Err(format!("repair edit line {line} is out of range"));
                }
                lines[line - 1] = after.to_string();
            }
            EditKind::DeleteLine => {
                if line == 0 || line > lines.len() {
                    return Err(format!("repair edit line {line} is out of range"));
                }
                lines.remove(line - 1);
            }
        }
    }
    Ok(lines.join("\n"))
}

fn has_later_end(lines: &[&str], name: &str, line: usize) -> bool {
    let needle = format!("\\end{{{name}}}");
    lines.iter().skip(line).any(|l| l.contains(&needle))
}

fn op_rank(edit: &EditOp) -> u8 {
    match edit.kind {
        EditKind::DeleteLine => 0,
        EditKind::ReplaceLine => 1,
        EditKind::InsertBefore => 2,
    }
}

pub fn source_hash(src: &str) -> u64 {
    let mut h = DefaultHasher::new();
    src.hash(&mut h);
    h.finish()
}

fn line_at<'a>(lines: &'a [&'a str], line: usize) -> &'a str {
    lines.get(line.saturating_sub(1)).copied().unwrap_or("")
}

fn eof_anchor(lines: &[&str]) -> usize {
    if lines.last() == Some(&"") {
        lines.len()
    } else {
        lines.len() + 1
    }
}

fn parse_unclosed_begin(message: &str) -> Option<String> {
    message.strip_prefix("unclosed \\begin{")?.strip_suffix('}').map(str::to_string)
}

fn parse_end_mismatch(message: &str) -> Option<(String, String)> {
    let rest = message.strip_prefix("\\end{")?;
    let (got, rest) = rest.split_once("} but \\begin{")?;
    let want = rest.strip_suffix("} is still open")?;
    Some((got.to_string(), want.to_string()))
}

fn parse_stray_end(message: &str) -> Option<String> {
    let rest = message.strip_prefix("\\end{")?;
    let (name, suffix) = rest.split_once('}')?;
    (suffix == " without \\begin").then(|| name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texparse::structure::parse_structure;

    #[test]
    fn unclosed_envs_close_innermost_first() {
        let src = "\\documentclass{article}\n\\begin{document}\n\\begin{itemize}\n\\item x\n";
        let doc = parse_structure(src);
        let lines: Vec<&str> = src.split('\n').collect();
        let edits = propose_edits(&doc, &lines);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].kind, EditKind::InsertBefore);
        assert_eq!(edits[0].after.as_deref(), Some("\\end{itemize}\n\\end{document}"));
        let patched = apply_edits(src, &edits).unwrap();
        assert!(patched.ends_with("\\end{itemize}\n\\end{document}\n"));
    }

    #[test]
    fn end_mismatch_replaces_wrong_env_name() {
        let src = "\\begin{document}\n\\begin{a}\n\\end{b}\n\\end{document}\n";
        let doc = parse_structure(src);
        let lines: Vec<&str> = src.split('\n').collect();
        let edits = propose_edits(&doc, &lines);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].kind, EditKind::ReplaceLine);
        assert!(edits[0].after.as_ref().unwrap().contains("\\end{a}"));
        let patched = apply_edits(src, &edits).unwrap();
        assert!(patched.contains("\\end{a}"));
    }

    #[test]
    fn stray_end_deletes_line() {
        let src = "\\begin{document}\n\\end{tabular}\n\\end{document}\n";
        let doc = parse_structure(src);
        let lines: Vec<&str> = src.split('\n').collect();
        let edits = propose_edits(&doc, &lines);
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].kind, EditKind::DeleteLine);
        let patched = apply_edits(src, &edits).unwrap();
        assert!(!patched.contains("\\end{tabular}"));
    }
}

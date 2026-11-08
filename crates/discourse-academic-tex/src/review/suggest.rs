//! W3C Web Annotation collaborative review proposals (`TextQuoteSelector`).
//!
//! Grounded in international human rights instruments (UDHR Art. 19, 27; ICESCR Art. 15)
//! protecting moral rights of authorship and freedom of scientific and artistic contribution,
//! and Informatics Fiduciary duties ensuring transparent, uncorrupted provenance of proposed changes.
//!
//! Proposals are anchored by a `{exact, prefix, suffix}` triple rather than rigid byte offsets.
//! Survives unrelated edits that shift offsets; when quoted text is gone or ambiguous,
//! reports `stale` or `ambiguous` rather than patching the wrong span.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Context captured around `exact` when not supplied by caller.
const CTX: usize = 40;

/// One pending review proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    /// Quoted span the proposal is about.
    pub exact: String,
    /// Text expected immediately before `exact`.
    pub prefix: String,
    /// Text expected immediately after `exact`.
    pub suffix: String,
    /// Replacement text — `None` for a comment-only review annotation.
    pub replace: Option<String>,
    /// Free-form reviewer note.
    pub comment: Option<String>,
}

/// Anchor resolution status against live buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolveStatus {
    /// Exactly one anchor-consistent occurrence at `[start, end)`.
    Unique(usize, usize),
    /// `exact` occurs multiple times and context cannot disambiguate.
    Ambiguous(usize),
    /// `exact` does not occur at all in the buffer.
    Missing,
}

fn occurrences(src: &str, needle: &str) -> Vec<usize> {
    src.match_indices(needle).take(256).map(|(i, _)| i).collect()
}

fn context_ok(src: &str, start: usize, end: usize, prefix: &str, suffix: &str) -> bool {
    let before = src.get(..start).unwrap_or("");
    let after = src.get(end..).unwrap_or("");
    before.ends_with(prefix) && after.starts_with(suffix)
}

/// Resolve anchor against source text.
pub fn resolve(src: &str, s: &Suggestion) -> ResolveStatus {
    let occ = occurrences(src, &s.exact);
    if occ.is_empty() {
        return ResolveStatus::Missing;
    }
    let hits: Vec<usize> = occ.iter().copied().filter(|&start| context_ok(src, start, start + s.exact.len(), &s.prefix, &s.suffix)).collect();
    match hits.as_slice() {
        [one] => ResolveStatus::Unique(*one, *one + s.exact.len()),
        [] => ResolveStatus::Missing,
        _ => ResolveStatus::Ambiguous(hits.len()),
    }
}

/// Capture context around a range in `src` on char boundaries.
pub fn context_around(src: &str, start: usize, end: usize) -> (String, String) {
    let pre = src.get(..start).unwrap_or("");
    let post = src.get(end..).unwrap_or("");
    let take_tail: usize = pre.chars().rev().take(CTX).map(char::len_utf8).sum();
    let take_head: usize = post.chars().take(CTX).map(char::len_utf8).sum();
    (pre.get(pre.len() - take_tail..).unwrap_or("").to_string(), post.get(..take_head).unwrap_or("").to_string())
}

/// Apply a suggestion to `src`, returning the new source if uniquely anchored.
pub fn apply_suggestion(src: &str, s: &Suggestion) -> Result<String, &'static str> {
    match resolve(src, s) {
        ResolveStatus::Unique(start, end) => {
            if let Some(new_text) = &s.replace {
                let mut buf = String::with_capacity(src.len() + new_text.len());
                buf.push_str(src.get(..start).unwrap_or(""));
                buf.push_str(new_text);
                buf.push_str(src.get(end..).unwrap_or(""));
                Ok(buf)
            } else {
                Ok(src.to_string())
            }
        }
        ResolveStatus::Ambiguous(_) => Err("proposal anchor is ambiguous"),
        ResolveStatus::Missing => Err("proposal anchor is stale/missing"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_then_apply() {
        let src = "one two three";
        let sug = Suggestion {
            id: "1".into(),
            exact: "two".into(),
            prefix: "one ".into(),
            suffix: " three".into(),
            replace: Some("TWO".into()),
            comment: None,
        };
        let updated = apply_suggestion(src, &sug).unwrap();
        assert_eq!(updated, "one TWO three");
    }

    #[test]
    fn anchor_survives_prefix_shift() {
        let sug = Suggestion {
            id: "1".into(),
            exact: "ccc".into(),
            prefix: "bbb ".into(),
            suffix: "".into(),
            replace: Some("CCC".into()),
            comment: None,
        };
        let shifted_src = "PREFIX aaa bbb ccc";
        let updated = apply_suggestion(shifted_src, &sug).unwrap();
        assert_eq!(updated, "PREFIX aaa bbb CCC");
    }

    #[test]
    fn stale_returns_err() {
        let sug = Suggestion {
            id: "1".into(),
            exact: "world".into(),
            prefix: "".into(),
            suffix: "".into(),
            replace: Some("earth".into()),
            comment: None,
        };
        assert!(apply_suggestion("hello universe", &sug).is_err());
    }
}

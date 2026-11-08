//! Word / Zotero CSL field storage format.

use serde::{Deserialize, Serialize};

pub const PREFIX: &str = "ADDIN ZOTERO_";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ZField {
    pub code: String,
    pub note_index: u32,
}

pub fn zotero_code(instr: &str) -> Option<&str> {
    let t = instr.trim_start();
    let head = t.get(..PREFIX.len())?;
    head.eq_ignore_ascii_case(PREFIX).then(|| t.get(PREFIX.len()..).unwrap_or("").trim())
}

pub fn stored_code(code: &str) -> String {
    format!("{PREFIX}{}", code.trim())
}

pub fn format_csl_citation_field(citation_json: &str) -> String {
    stored_code(&format!("ITEM CSL_CITATION {citation_json}"))
}

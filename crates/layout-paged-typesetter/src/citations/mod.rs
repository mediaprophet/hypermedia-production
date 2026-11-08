//! Citation and bibliography engine.

pub mod csl;
pub mod zotero_fields;

pub use csl::{Bibliography, Citation, CitationAuthor, CitationItem, CitationStyle, format_full_entry};
pub use zotero_fields::{PREFIX, ZField, format_csl_citation_field, stored_code, zotero_code};

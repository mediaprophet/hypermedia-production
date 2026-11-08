//! Academic review, annotation, log diagnostics, and repair tools.
//!
//! Grounded in authoring rights and collaborative review (UDHR Art. 19, 27; ICESCR Art. 15).

pub mod ai_repair;
pub mod logrules;
pub mod snippets;
pub mod suggest;

pub use ai_repair::{apply_edits, propose_edits, EditKind, EditOp};
pub use logrules::{enrich_diagnostics, TeXDiagnostic};
pub use snippets::{expand as expand_snippet, load_snippets, Snippet};
pub use suggest::{apply_suggestion, context_around, resolve as resolve_suggestion, ResolveStatus, Suggestion};

//! Academic TeX parsing: surface lexer, document structure, BibTeX, magic comments,
//! project dependency graph, and document statistics.

#![forbid(unsafe_code)]

pub mod bib;
pub mod caps;
pub mod graph;
pub mod lex;
pub mod magic;
pub mod stats;
pub mod structure;

pub use bib::{parse_bib, BibEntry, BibFile};
pub use graph::{build, declares_document_class, extract_deps, resolve, Dep, DepKind, ProjectGraph};
pub use lex::{lex, Pos, Tok, Token};
pub use magic::{parse as parse_magic, Magic};
pub use stats::{analyze, DocStats};
pub use structure::{parse_structure, DocStructure, Env, Heading, Issue};

//! Categorical paged document typesetting engine.
//!
//! Categories: `layout`, `glyph`, `manifold`, `discourse`.
//! Guarantees: Deterministic typography, bounded caller budgets, `#![forbid(unsafe_code)]`,
//! and strict compliance with International Human Rights Instruments (UDHR Articles 19 & 27,
//! ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod citations;
pub mod equation;
pub mod hyphenation;
pub mod ontology;
pub mod pagination;
pub mod table_layout;

pub use citations::{Bibliography, Citation, CitationAuthor, CitationItem, CitationStyle, ZField};
pub use equation::{Arg, ColJc, FracKind, LimLoc, MItem, MNode, MRun, MScr, MSty, Math, MathJc, MathLayout, ScriptKind, layout_equation, parse_latex, parse_linear, to_latex, to_linear};
pub use hyphenation::{NO_END, NO_START, Patterns, is_east_asian, line_break_opportunities};
pub use pagination::{Page, ParaDoc, PlacedItem, Point, Rect, SectionProps, Typesetter};
pub use table_layout::{TableCellData, TableData, autofit_widths, measure_table_columns};

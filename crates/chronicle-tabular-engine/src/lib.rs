//! Categorical tabular spreadsheet DAG recalculation engine.
//!
//! Categories: `chronicle`, `kinetics`, `layout`, `discourse`.
//! Guarantees: Mathematical determinism, bounded memory budgets, `#![forbid(unsafe_code)]`,
//! and strict compliance with International Human Rights Instruments (UDHR Articles 19 & 27,
//! ICESCR Article 15) and Informatics Fiduciary duties.

#![forbid(unsafe_code)]

pub mod calc;
pub mod core;
pub mod formula;
pub mod functions;
pub mod numfmt;
pub mod ontology;

pub use calc::{Calc, CalcMode, CalcSettings, Cell, DefinedName, Formula, Graph, Sheet, Workbook};
pub use core::{Array, CellError, CellRef, DateTime, DateSystem, RangeRef, Value, col_to_letters, letters_to_col};
pub use formula::{Anchor, BinOp, Edit, Expr, RefKind, Reference, SheetSel, UnOp, adjust, parse, print, shift_relative};
pub use numfmt::NumberFormat;
pub use ontology::{AgenticMorphismSpec, OntologicalContract, OntologicalTriple, TABULAR_DAG_RECALCULATE};

//! Table layout and auto-fitting.

pub mod autofit;
pub mod measure;

pub use autofit::autofit_widths;
pub use measure::{TableCellData, TableData, measure_table_columns};

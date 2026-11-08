//! Core tabular primitives: cell references, date arithmetic, values and parsing.

pub mod addr;
pub mod date;
pub mod parse;
pub mod value;

pub use addr::{CellRef, MAX_COLS, MAX_ROWS, RangeRef, col_to_letters, letters_to_col, parse_a1_prefix};
pub use date::{DateTime, DateSystem, days_from_civil, civil_from_days, is_leap, days_in_month, serial_from_ymd, datetime_from_serial, time_fraction};
pub use parse::{Parsed, parse_input, parse_number_text};
pub use value::{Array, CellError, Value, compare, compare_numbers, compare_text, number_to_text, sort_compare};

//! Interactive AcroForm models, validation, flattening, and XFDF import/export.

#![forbid(unsafe_code)]

pub mod flatten;
pub mod model;
pub mod validation;
pub mod xfdf;

pub use flatten::{FlattenReport, FlattenedElement, flatten_form};
pub use model::{AcroForm, AcroFormField, FieldKind, flags};
pub use validation::{
    CalcOp, CalculateRule, FormatRule, ValidateRule, execute_calculation, format_field,
    validate_field,
};
pub use xfdf::{export_xfdf, import_xfdf};

//! Form flattening: converts interactive AcroForm fields into static visual page content.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use super::model::{AcroForm, FieldKind};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlattenedElement {
    pub page: usize,
    pub rect: [f64; 4],
    pub text: String,
    pub is_checked: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlattenReport {
    pub flattened_count: usize,
    pub elements: Vec<FlattenedElement>,
}

/// Flattens an AcroForm: renders the appearance of every field into static elements
/// and empties the interactive form fields collection.
pub fn flatten_form(form: &mut AcroForm) -> FlattenReport {
    let mut elements = Vec::new();
    let count = form.fields.len();

    for field in &form.fields {
        match field.kind {
            FieldKind::Text | FieldKind::Combo | FieldKind::List => {
                elements.push(FlattenedElement {
                    page: field.page,
                    rect: field.rect,
                    text: field.value.clone(),
                    is_checked: false,
                });
            }
            FieldKind::CheckBox | FieldKind::Radio => {
                let checked = field.value != "Off" && !field.value.is_empty();
                elements.push(FlattenedElement {
                    page: field.page,
                    rect: field.rect,
                    text: if checked { "✓".into() } else { "".into() },
                    is_checked: checked,
                });
            }
            FieldKind::PushButton | FieldKind::Signature => {
                elements.push(FlattenedElement {
                    page: field.page,
                    rect: field.rect,
                    text: field.value.clone(),
                    is_checked: false,
                });
            }
        }
    }

    form.fields.clear();
    form.calc_order.clear();

    FlattenReport {
        flattened_count: count,
        elements,
    }
}

//! Interactive AcroForm field model and flag definitions (ISO 32000-2 §12.7).

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldKind {
    Text,
    CheckBox,
    Radio,
    PushButton,
    Combo,
    List,
    Signature,
}

pub mod flags {
    pub const READ_ONLY: u32 = 1;
    pub const REQUIRED: u32 = 1 << 1;
    pub const MULTILINE: u32 = 1 << 12;
    pub const PASSWORD: u32 = 1 << 13;
    pub const NO_TOGGLE_TO_OFF: u32 = 1 << 14;
    pub const RADIO: u32 = 1 << 15;
    pub const PUSH_BUTTON: u32 = 1 << 16;
    pub const COMBO: u32 = 1 << 17;
    pub const EDIT: u32 = 1 << 18;
    pub const SORT: u32 = 1 << 19;
    pub const MULTI_SELECT: u32 = 1 << 21;
    pub const COMB: u32 = 1 << 24;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcroFormField {
    pub name: String,
    pub kind: FieldKind,
    pub flags: u32,
    pub value: String,
    pub default_value: String,
    pub options: Vec<String>,
    pub page: usize,
    pub rect: [f64; 4], // [x0, y0, x1, y1]
    pub tooltip: Option<String>,
}

impl AcroFormField {
    pub fn new_text(name: &str, page: usize, rect: [f64; 4]) -> Self {
        AcroFormField {
            name: name.to_string(),
            kind: FieldKind::Text,
            flags: 0,
            value: String::new(),
            default_value: String::new(),
            options: Vec::new(),
            page,
            rect,
            tooltip: None,
        }
    }

    pub fn new_checkbox(name: &str, page: usize, rect: [f64; 4], checked: bool) -> Self {
        AcroFormField {
            name: name.to_string(),
            kind: FieldKind::CheckBox,
            flags: 0,
            value: if checked { "Yes".into() } else { "Off".into() },
            default_value: "Off".into(),
            options: vec!["Yes".into(), "Off".into()],
            page,
            rect,
            tooltip: None,
        }
    }

    pub fn is_read_only(&self) -> bool {
        self.flags & flags::READ_ONLY != 0
    }

    pub fn is_required(&self) -> bool {
        self.flags & flags::REQUIRED != 0
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AcroForm {
    pub fields: Vec<AcroFormField>,
    pub calc_order: Vec<String>,
    pub need_appearances: bool,
}

impl AcroForm {
    pub fn field(&self, name: &str) -> Option<&AcroFormField> {
        self.fields.iter().find(|f| f.name == name)
    }

    pub fn field_mut(&mut self, name: &str) -> Option<&mut AcroFormField> {
        self.fields.iter_mut().find(|f| f.name == name)
    }
}

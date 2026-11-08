//! Shape, text, table, and transition data models.

use serde::{Deserialize, Serialize};
use super::types::{ColorRef, Fill, Line, PhType, Rect, Rgba, ShapeId, ShapeKind, Xfrm};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Placeholder {
    pub kind: PhType,
    pub idx: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RunProps {
    pub font_family: Option<String>,
    pub font_size: Option<f64>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub fill: Option<ColorRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Run {
    pub text: String,
    pub props: RunProps,
}

impl Run {
    pub fn new(text: impl Into<String>) -> Self {
        Run {
            text: text.into(),
            props: RunProps::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ParaProps {
    pub align: TextAlign,
    pub level: u32,
    pub line_spacing: Option<f64>,
    pub space_before: Option<f64>,
    pub space_after: Option<f64>,
    pub bullet: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Paragraph {
    pub runs: Vec<Run>,
    pub props: ParaProps,
}

impl Paragraph {
    pub fn new(text: impl Into<String>) -> Self {
        Paragraph {
            runs: vec![Run::new(text)],
            props: ParaProps::default(),
        }
    }

    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TextBody {
    pub paragraphs: Vec<Paragraph>,
    pub inset: Option<Rect>,
}

impl TextBody {
    pub fn text(&self) -> String {
        self.paragraphs.iter().map(|p| p.text()).collect::<Vec<_>>().join("\n")
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCell {
    pub text: TextBody,
    pub fill: Option<Fill>,
    pub border: Option<Line>,
    pub col_span: usize,
    pub row_span: usize,
}

impl Default for TableCell {
    fn default() -> Self {
        TableCell {
            text: TextBody::default(),
            fill: None,
            border: None,
            col_span: 1,
            row_span: 1,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TableRow {
    pub height: f64,
    pub cells: Vec<TableCell>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Table {
    pub col_widths: Vec<f64>,
    pub rows: Vec<TableRow>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TransitionKind {
    #[default]
    None,
    Cut,
    Fade,
    Push,
    Wipe,
    Split,
    Morph,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlideTransition {
    pub kind: TransitionKind,
    pub duration: f64,
    pub advance_after: Option<f64>,
}

impl Default for SlideTransition {
    fn default() -> Self {
        SlideTransition {
            kind: TransitionKind::None,
            duration: 0.5,
            advance_after: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShapeContent {
    None,
    Text(TextBody),
    Table(Table),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Shape {
    pub id: ShapeId,
    pub name: String,
    pub kind: ShapeKind,
    pub xfrm: Xfrm,
    pub fill: Option<Fill>,
    pub line: Option<Line>,
    pub content: ShapeContent,
    pub ph: Option<Placeholder>,
    pub geom_path: Option<String>,
}

impl Shape {
    pub fn new(id: ShapeId, name: impl Into<String>, xfrm: Xfrm) -> Self {
        Shape {
            id,
            name: name.into(),
            kind: ShapeKind::Rect,
            xfrm,
            fill: None,
            line: None,
            content: ShapeContent::None,
            ph: None,
            geom_path: None,
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        let mut tb = TextBody::default();
        tb.paragraphs.push(Paragraph::new(text));
        self.content = ShapeContent::Text(tb);
        self
    }

    pub fn text(&self) -> Option<String> {
        match &self.content {
            ShapeContent::Text(tb) => Some(tb.text()),
            _ => None,
        }
    }

    pub fn ph_type(&self) -> Option<PhType> {
        self.ph.as_ref().map(|p| p.kind)
    }
}

//! Unified Flow document model for universal interchange across Markdown, HTML, RTF, LaTeX, and DOCX.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowFmt {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub code: bool,
    pub sup: bool,
    pub sub: bool,
    pub link: Option<String>,
    pub font_size: Option<f64>,
    pub color_rgb: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FlowInline {
    Text(String, FlowFmt),
    Math { expr: String, display: bool },
    Image { src: String, alt: String, width: Option<f64>, height: Option<f64> },
    Anchor(String),
}

impl FlowInline {
    pub fn plain_text(text: impl Into<String>) -> Self {
        FlowInline::Text(text.into(), FlowFmt::default())
    }

    pub fn text_content(&self) -> &str {
        match self {
            FlowInline::Text(t, _) => t.as_str(),
            FlowInline::Math { expr, .. } => expr.as_str(),
            FlowInline::Image { alt, .. } => alt.as_str(),
            FlowInline::Anchor(_) => "",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FlowKind {
    #[default]
    Normal,
    Heading(u8),
    Title,
    Quote,
    Code,
    Rule,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowListInfo {
    pub ordered: bool,
    pub level: u8,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowPara {
    pub kind: FlowKind,
    pub list: Option<FlowListInfo>,
    pub align: Option<String>,
    pub inlines: Vec<FlowInline>,
}

impl FlowPara {
    pub fn new(kind: FlowKind) -> Self {
        FlowPara {
            kind,
            list: None,
            align: None,
            inlines: Vec::new(),
        }
    }

    pub fn with_text(kind: FlowKind, text: impl Into<String>) -> Self {
        let mut p = FlowPara::new(kind);
        p.inlines.push(FlowInline::plain_text(text));
        p
    }

    pub fn plain_text(&self) -> String {
        self.inlines.iter().map(|i| i.text_content()).collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowCell {
    pub blocks: Vec<FlowBlock>,
    pub col_span: usize,
    pub row_span: usize,
}

impl FlowCell {
    pub fn with_text(text: impl Into<String>) -> Self {
        FlowCell {
            blocks: vec![FlowBlock::Para(FlowPara::with_text(FlowKind::Normal, text))],
            col_span: 1,
            row_span: 1,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowRow {
    pub cells: Vec<FlowCell>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowTable {
    pub col_widths: Vec<f64>,
    pub rows: Vec<FlowRow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FlowBlock {
    Para(FlowPara),
    Table(FlowTable),
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FlowDoc {
    pub title: Option<String>,
    pub author: Option<String>,
    pub blocks: Vec<FlowBlock>,
}

impl FlowDoc {
    pub fn new() -> Self {
        FlowDoc::default()
    }

    pub fn push_para(&mut self, para: FlowPara) {
        self.blocks.push(FlowBlock::Para(para));
    }

    pub fn push_table(&mut self, table: FlowTable) {
        self.blocks.push(FlowBlock::Table(table));
    }
}

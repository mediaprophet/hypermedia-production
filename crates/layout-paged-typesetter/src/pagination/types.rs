//! Geometric types and structures for paged document layout.

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    pub fn intersects(&self, o: &Rect) -> bool {
        self.x < o.right() && o.x < self.right() && self.y < o.bottom() && o.y < self.bottom()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectionProps {
    pub page_w: f32,
    pub page_h: f32,
    pub margin_top: f32,
    pub margin_bottom: f32,
    pub margin_left: f32,
    pub margin_right: f32,
    pub num_columns: usize,
    pub column_gap: f32,
}

impl Default for SectionProps {
    fn default() -> Self {
        // Standard Letter (8.5 x 11 inches) at 72 dpi: 612 x 792 pt, 1-inch margins (72 pt)
        SectionProps {
            page_w: 612.0,
            page_h: 792.0,
            margin_top: 72.0,
            margin_bottom: 72.0,
            margin_left: 72.0,
            margin_right: 72.0,
            num_columns: 1,
            column_gap: 18.0,
        }
    }
}

impl SectionProps {
    pub fn content_width(&self) -> f32 {
        (self.page_w - self.margin_left - self.margin_right).max(0.0)
    }

    pub fn content_height(&self) -> f32 {
        (self.page_h - self.margin_top - self.margin_bottom).max(0.0)
    }

    pub fn column_width(&self) -> f32 {
        let total_w = self.content_width();
        let gaps = (self.num_columns.saturating_sub(1) as f32) * self.column_gap;
        (total_w - gaps).max(0.0) / (self.num_columns.max(1) as f32)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlacedItem {
    Line {
        rect: Rect,
        text: String,
        font_size: f32,
    },
    Figure {
        rect: Rect,
        alt: String,
    },
    Footnote {
        rect: Rect,
        number: usize,
        text: String,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub number: usize,
    pub items: Vec<PlacedItem>,
    pub footnotes: Vec<PlacedItem>,
}

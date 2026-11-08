//! Core types for box auto-layout, constraints, alignments, and grid tracks.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayoutDirection {
    #[default]
    Horizontal,
    Vertical,
    Grid,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SizingMode {
    #[default]
    Fixed,
    Hug,
    Fill,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MainAlign {
    #[default]
    Min,
    Center,
    Max,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl MainAlign {
    pub fn is_auto_gap(self) -> bool {
        matches!(self, MainAlign::SpaceBetween | MainAlign::SpaceAround | MainAlign::SpaceEvenly)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrossAlign {
    #[default]
    Min,
    Center,
    Max,
    Baseline,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentAlign {
    #[default]
    Min,
    Center,
    Max,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CellAlign {
    #[default]
    Min,
    Center,
    Max,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HConstraint {
    #[default]
    Left,
    Right,
    LeftRight,
    Center,
    Scale,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VConstraint {
    #[default]
    Top,
    Bottom,
    TopBottom,
    Center,
    Scale,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum GridTrack {
    Fixed(f64),
    Hug,
    Fr(f64),
}

impl Default for GridTrack {
    fn default() -> Self {
        GridTrack::Fr(1.0)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub const fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Rect { x, y, width, height }
    }
    pub fn right(self) -> f64 {
        self.x + self.width
    }
    pub fn bottom(self) -> f64 {
        self.y + self.height
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Padding {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Padding {
    pub const fn new(top: f64, right: f64, bottom: f64, left: f64) -> Self {
        Padding { top, right, bottom, left }
    }
    pub const fn uniform(p: f64) -> Self {
        Padding { top: p, right: p, bottom: p, left: p }
    }
    pub const fn symmetric(v: f64, h: f64) -> Self {
        Padding { top: v, right: h, bottom: v, left: h }
    }
    pub fn horizontal(self) -> f64 {
        self.left + self.right
    }
    pub fn vertical(self) -> f64 {
        self.top + self.bottom
    }
}

pub fn clamp_size(v: f64, min: Option<f64>, max: Option<f64>) -> f64 {
    let mut v = if v.is_finite() { v.max(0.0) } else { 0.0 };
    if let Some(m) = max.filter(|m| m.is_finite()) {
        v = v.min(m);
    }
    if let Some(m) = min.filter(|m| m.is_finite()) {
        v = v.max(m);
    }
    v
}

//! Auto-layout frame and node representation.

#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

use super::types::{
    CellAlign, ContentAlign, CrossAlign, GridTrack, HConstraint, LayoutDirection, MainAlign, Padding, Rect,
    SizingMode, VConstraint,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutNode {
    pub id: u64,
    pub rect: Rect,
    pub sizing_h: SizingMode,
    pub sizing_v: SizingMode,
    pub min_w: Option<f64>,
    pub max_w: Option<f64>,
    pub min_h: Option<f64>,
    pub max_h: Option<f64>,
    pub h_constraint: HConstraint,
    pub v_constraint: VConstraint,
    pub absolute: bool,
    /// Baseline offset from top of node box (if available, e.g. text)
    pub baseline_offset: Option<f64>,
    // Grid child properties
    pub grid_col: Option<usize>,
    pub grid_row: Option<usize>,
    pub grid_col_span: usize,
    pub grid_row_span: usize,
    pub align_self_h: Option<CellAlign>,
    pub align_self_v: Option<CellAlign>,
    // Sub-frame if this node is itself a container
    pub frame: Option<AutoLayoutFrame>,
}

impl LayoutNode {
    pub fn new(id: u64, width: f64, height: f64) -> Self {
        LayoutNode {
            id,
            rect: Rect::new(0.0, 0.0, width, height),
            sizing_h: SizingMode::Fixed,
            sizing_v: SizingMode::Fixed,
            min_w: None,
            max_w: None,
            min_h: None,
            max_h: None,
            h_constraint: HConstraint::Left,
            v_constraint: VConstraint::Top,
            absolute: false,
            baseline_offset: None,
            grid_col: None,
            grid_row: None,
            grid_col_span: 1,
            grid_row_span: 1,
            align_self_h: None,
            align_self_v: None,
            frame: None,
        }
    }

    pub fn with_sizing(mut self, h: SizingMode, v: SizingMode) -> Self {
        self.sizing_h = h;
        self.sizing_v = v;
        self
    }

    pub fn with_constraints(mut self, h: HConstraint, v: VConstraint) -> Self {
        self.h_constraint = h;
        self.v_constraint = v;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoLayoutFrame {
    pub direction: LayoutDirection,
    pub padding: Padding,
    pub gap: f64,
    pub cross_gap: Option<f64>,
    pub wrap: bool,
    pub main_align: MainAlign,
    pub cross_align: CrossAlign,
    pub content_align: ContentAlign,
    pub sizing_h: SizingMode,
    pub sizing_v: SizingMode,
    pub min_w: Option<f64>,
    pub max_w: Option<f64>,
    pub min_h: Option<f64>,
    pub max_h: Option<f64>,
    // Grid specific tracks
    pub grid_columns: Vec<GridTrack>,
    pub grid_rows: Vec<GridTrack>,
    pub children: Vec<LayoutNode>,
}

impl Default for AutoLayoutFrame {
    fn default() -> Self {
        AutoLayoutFrame {
            direction: LayoutDirection::Horizontal,
            padding: Padding::default(),
            gap: 0.0,
            cross_gap: None,
            wrap: false,
            main_align: MainAlign::Min,
            cross_align: CrossAlign::Min,
            content_align: ContentAlign::Min,
            sizing_h: SizingMode::Fixed,
            sizing_v: SizingMode::Fixed,
            min_w: None,
            max_w: None,
            min_h: None,
            max_h: None,
            grid_columns: Vec::new(),
            grid_rows: Vec::new(),
            children: Vec::new(),
        }
    }
}

impl AutoLayoutFrame {
    pub fn new(direction: LayoutDirection) -> Self {
        AutoLayoutFrame {
            direction,
            ..Default::default()
        }
    }

    pub fn with_padding(mut self, p: Padding) -> Self {
        self.padding = p;
        self
    }

    pub fn with_gap(mut self, gap: f64) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn add_child(&mut self, child: LayoutNode) {
        self.children.push(child);
    }
}

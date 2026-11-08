//! Unit tests for layout-box-autolayout.

#![forbid(unsafe_code)]

use super::constraints::apply_constraints;
use super::flex::solve_flex;
use super::frame::{AutoLayoutFrame, LayoutNode};
use super::grid::{grid_place, resolve_tracks, solve_grid};
use super::solver::solve_autolayout;
use super::types::{
    CellAlign, CrossAlign, GridTrack, HConstraint, LayoutDirection, MainAlign, Padding, Rect,
    SizingMode, VConstraint,
};

#[test]
fn test_horizontal_flex_basic() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
        .with_padding(Padding::uniform(10.0))
        .with_gap(5.0);

    let node1 = LayoutNode::new(1, 40.0, 30.0);
    let node2 = LayoutNode::new(2, 60.0, 50.0);
    frame.add_child(node1);
    frame.add_child(node2);

    let (w, h) = solve_autolayout(&mut frame, 200.0, 100.0);
    assert_eq!(w, 200.0);
    assert_eq!(h, 100.0);

    // Child 1 at (10, 10)
    assert_eq!(frame.children[0].rect.x, 10.0);
    assert_eq!(frame.children[0].rect.y, 10.0);

    // Child 2 at (10 + 40 + 5, 10) = (55, 10)
    assert_eq!(frame.children[1].rect.x, 55.0);
    assert_eq!(frame.children[1].rect.y, 10.0);
}

#[test]
fn test_hug_container_sizing() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
        .with_padding(Padding::symmetric(10.0, 20.0))
        .with_gap(10.0);
    frame.sizing_h = SizingMode::Hug;
    frame.sizing_v = SizingMode::Hug;

    let node1 = LayoutNode::new(1, 50.0, 30.0);
    let node2 = LayoutNode::new(2, 70.0, 40.0);
    frame.add_child(node1);
    frame.add_child(node2);

    let (w, h) = solve_autolayout(&mut frame, 1000.0, 1000.0);
    // Width = pl + 50 + gap(10) + 70 + pr = 20 + 50 + 10 + 70 + 20 = 170
    assert_eq!(w, 170.0);
    // Height = pt + max(30, 40) + pb = 10 + 40 + 10 = 60
    assert_eq!(h, 60.0);
}

#[test]
fn test_fill_distribution_with_clamping() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
        .with_padding(Padding::uniform(0.0))
        .with_gap(0.0);

    let mut node1 = LayoutNode::new(1, 0.0, 50.0).with_sizing(SizingMode::Fill, SizingMode::Fixed);
    node1.max_w = Some(40.0); // clamped at 40

    let node2 = LayoutNode::new(2, 0.0, 50.0).with_sizing(SizingMode::Fill, SizingMode::Fixed);
    let node3 = LayoutNode::new(3, 0.0, 50.0).with_sizing(SizingMode::Fill, SizingMode::Fixed);

    frame.add_child(node1);
    frame.add_child(node2);
    frame.add_child(node3);

    // 160 available. node1 takes 40, remaining 120 shared between node2 and node3 -> 60 each
    let (w, _) = solve_autolayout(&mut frame, 160.0, 50.0);
    assert_eq!(w, 160.0);
    assert_eq!(frame.children[0].rect.width, 40.0);
    assert_eq!(frame.children[1].rect.width, 60.0);
    assert_eq!(frame.children[2].rect.width, 60.0);
}

#[test]
fn test_multi_line_wrapping() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
        .with_padding(Padding::uniform(0.0))
        .with_gap(10.0)
        .with_wrap(true);

    frame.add_child(LayoutNode::new(1, 60.0, 30.0));
    frame.add_child(LayoutNode::new(2, 60.0, 30.0));
    frame.add_child(LayoutNode::new(3, 60.0, 30.0));

    // Available width 100:
    // Line 1 fits item 1 (60). Item 2 needs 60 + 10 = 70 (total 130 > 100), so breaks to Line 2!
    // Line 2 has item 2 (60). Item 3 breaks to Line 3 (60).
    solve_autolayout(&mut frame, 100.0, 300.0);
    assert_eq!(frame.children[0].rect.y, 0.0);
    assert_eq!(frame.children[1].rect.y, 40.0); // 30 + gap 10
    assert_eq!(frame.children[2].rect.y, 80.0);
}

#[test]
fn test_baseline_alignment() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Horizontal)
        .with_padding(Padding::uniform(0.0));
    frame.cross_align = CrossAlign::Baseline;

    let mut node1 = LayoutNode::new(1, 50.0, 40.0);
    node1.baseline_offset = Some(30.0); // Baseline at 30 from top

    let mut node2 = LayoutNode::new(2, 50.0, 20.0);
    node2.baseline_offset = Some(10.0); // Baseline at 10 from top

    frame.add_child(node1);
    frame.add_child(node2);

    solve_autolayout(&mut frame, 200.0, 100.0);
    // Node 1 baseline at y=30, node 2 baseline offset should shift node 2 down by 20 so both baselines align at y=30
    assert_eq!(frame.children[0].rect.y, 0.0);
    assert_eq!(frame.children[1].rect.y, 20.0);
}

#[test]
fn test_css_grid_tracks_and_spans() {
    let mut frame = AutoLayoutFrame::new(LayoutDirection::Grid)
        .with_padding(Padding::uniform(10.0))
        .with_gap(10.0);

    // 2 columns: 1fr and 2fr
    frame.grid_columns = vec![GridTrack::Fr(1.0), GridTrack::Fr(2.0)];
    // 2 rows: 50px fixed, and 1fr
    frame.grid_rows = vec![GridTrack::Fixed(50.0), GridTrack::Fr(1.0)];

    let mut child1 = LayoutNode::new(1, 0.0, 0.0).with_sizing(SizingMode::Fill, SizingMode::Fill);
    child1.grid_col = Some(0);
    child1.grid_row = Some(0);

    let mut child2 = LayoutNode::new(2, 0.0, 0.0).with_sizing(SizingMode::Fill, SizingMode::Fill);
    child2.grid_col = Some(1);
    child2.grid_row = Some(0);

    let mut child3 = LayoutNode::new(3, 0.0, 0.0).with_sizing(SizingMode::Fill, SizingMode::Fill);
    child3.grid_col_span = 2; // Spanning both columns on row 1

    frame.add_child(child1);
    frame.add_child(child2);
    frame.add_child(child3);

    // Available 330w x 180h
    // Inner width = 330 - 20 (padding) - 10 (gap) = 300.
    // Col 1 (1fr) = 100, Col 2 (2fr) = 200.
    // Inner height = 180 - 20 - 10 = 150. Row 1 = 50, Row 2 (1fr) = 100.
    solve_autolayout(&mut frame, 330.0, 180.0);

    assert_eq!(frame.children[0].rect.x, 10.0);
    assert_eq!(frame.children[0].rect.width, 100.0);
    assert_eq!(frame.children[0].rect.height, 50.0);

    assert_eq!(frame.children[1].rect.x, 120.0); // 10 + 100 + 10
    assert_eq!(frame.children[1].rect.width, 200.0);
    assert_eq!(frame.children[1].rect.height, 50.0);

    assert_eq!(frame.children[2].rect.x, 10.0);
    assert_eq!(frame.children[2].rect.y, 70.0); // 10 + 50 + 10
    assert_eq!(frame.children[2].rect.width, 310.0); // 100 + 10 + 200 = 310
    assert_eq!(frame.children[2].rect.height, 100.0);
}

#[test]
fn test_constraints_propagation() {
    let mut children = vec![
        // Left constraint: stays anchored at left
        LayoutNode::new(1, 40.0, 40.0).with_constraints(HConstraint::Left, VConstraint::Top),
        // Right constraint: moves with right edge
        LayoutNode::new(2, 40.0, 40.0).with_constraints(HConstraint::Right, VConstraint::Top),
        // LeftRight constraint: stretches
        LayoutNode::new(3, 40.0, 40.0).with_constraints(HConstraint::LeftRight, VConstraint::Top),
        // Center constraint: centers
        LayoutNode::new(4, 40.0, 40.0).with_constraints(HConstraint::Center, VConstraint::Top),
    ];
    children[0].rect.x = 10.0;
    children[1].rect.x = 150.0; // right = 190 in 200
    children[2].rect.x = 10.0;  // 10..50
    children[3].rect.x = 80.0;  // center of 200

    // Resize parent from 200 to 300 (diff = +100)
    apply_constraints(&mut children, 200.0, 100.0, 300.0, 100.0);

    assert_eq!(children[0].rect.x, 10.0);
    assert_eq!(children[0].rect.width, 40.0);

    assert_eq!(children[1].rect.x, 250.0); // 150 + 100
    assert_eq!(children[1].rect.width, 40.0);

    assert_eq!(children[2].rect.x, 10.0);
    assert_eq!(children[2].rect.width, 140.0); // stretched by 100

    assert_eq!(children[3].rect.x, 130.0); // shifted by +50
}

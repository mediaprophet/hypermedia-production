//! Top-level solver function orchestrating Flexbox and Grid auto-layout.

#![forbid(unsafe_code)]

use super::constraints::apply_constraints;
use super::flex::solve_flex;
use super::frame::AutoLayoutFrame;
use super::grid::solve_grid;
use super::types::LayoutDirection;

/// Solves auto-layout on a frame with the given available dimensions.
/// Returns the resolved `(width, height)` of the frame.
pub fn solve_autolayout(frame: &mut AutoLayoutFrame, available_w: f64, available_h: f64) -> (f64, f64) {
    // 1. Recursively solve any nested sub-frames inside child nodes first
    for child in &mut frame.children {
        if let Some(subframe) = &mut child.frame {
            let (sw, sh) = solve_autolayout(subframe, child.rect.width, child.rect.height);
            child.rect.width = sw;
            child.rect.height = sh;
        }
    }

    let orig_w = available_w;
    let orig_h = available_h;

    // 2. Solve current frame per layout direction
    let (final_w, final_h) = match frame.direction {
        LayoutDirection::Horizontal | LayoutDirection::Vertical => solve_flex(frame, available_w, available_h),
        LayoutDirection::Grid => solve_grid(frame, available_w, available_h),
    };

    // 3. Apply constraints to absolute positioned children
    let mut absolute_children: Vec<_> = frame.children.iter().filter(|c| c.absolute).cloned().collect();
    if !absolute_children.is_empty() {
        apply_constraints(&mut absolute_children, orig_w, orig_h, final_w, final_h);
        for updated in absolute_children {
            if let Some(slot) = frame.children.iter_mut().find(|c| c.id == updated.id) {
                *slot = updated;
            }
        }
    }

    (final_w, final_h)
}

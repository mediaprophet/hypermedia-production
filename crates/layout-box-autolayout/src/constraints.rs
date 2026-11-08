//! Constraint propagation on parent resize.

#![forbid(unsafe_code)]

use super::frame::LayoutNode;
use super::types::{HConstraint, VConstraint};

/// Adjusts children positions and sizes when their parent changes size from `(ow, oh)` to `(nw, nh)`.
pub fn apply_constraints(children: &mut [LayoutNode], ow: f64, oh: f64, nw: f64, nh: f64) {
    if (nw - ow).abs() < 1e-9 && (nh - oh).abs() < 1e-9 {
        return;
    }
    let sx = if ow > 1e-9 { nw / ow } else { 1.0 };
    let sy = if oh > 1e-9 { nh / oh } else { 1.0 };

    for child in children.iter_mut() {
        let (mut x0, mut x1) = (child.rect.x, child.rect.x + child.rect.width);
        let (mut y0, mut y1) = (child.rect.y, child.rect.y + child.rect.height);

        match child.h_constraint {
            HConstraint::Left => {}
            HConstraint::Right => {
                x0 += nw - ow;
                x1 += nw - ow;
            }
            HConstraint::LeftRight => {
                x1 += nw - ow;
            }
            HConstraint::Center => {
                let d = (nw - ow) * 0.5;
                x0 += d;
                x1 += d;
            }
            HConstraint::Scale => {
                x0 *= sx;
                x1 *= sx;
            }
        }

        match child.v_constraint {
            VConstraint::Top => {}
            VConstraint::Bottom => {
                y0 += nh - oh;
                y1 += nh - oh;
            }
            VConstraint::TopBottom => {
                y1 += nh - oh;
            }
            VConstraint::Center => {
                let d = (nh - oh) * 0.5;
                y0 += d;
                y1 += d;
            }
            VConstraint::Scale => {
                y0 *= sy;
                y1 *= sy;
            }
        }

        child.rect.x = x0;
        child.rect.y = y0;
        child.rect.width = (x1 - x0).max(0.0);
        child.rect.height = (y1 - y0).max(0.0);
    }
}

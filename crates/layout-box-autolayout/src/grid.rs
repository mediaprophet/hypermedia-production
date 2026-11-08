//! CSS Grid auto-layout solver with fractional `fr` tracks and cell placement.

#![forbid(unsafe_code)]

use super::frame::AutoLayoutFrame;
use super::types::{CellAlign, GridTrack, MainAlign, CrossAlign, SizingMode, clamp_size};

pub const MAX_GRID_CELLS: usize = 1_000_000;
pub const MAX_TRACKS: usize = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridCell {
    pub child_index: usize,
    pub col: usize,
    pub row: usize,
    pub col_span: usize,
    pub row_span: usize,
}

pub fn grid_place(frame: &AutoLayoutFrame, n_cols: usize) -> (Vec<GridCell>, usize) {
    let cols = n_cols.clamp(1, MAX_TRACKS);
    let max_rows = (MAX_GRID_CELLS / cols).clamp(1, MAX_TRACKS);
    let mut occ: Vec<Vec<bool>> = Vec::new();

    let free = |occ: &Vec<Vec<bool>>, c: usize, r: usize, cs: usize, rs: usize| {
        (r..r + rs).all(|ri| occ.get(ri).is_none_or(|row| (c..c + cs).all(|ci| !row.get(ci).copied().unwrap_or(false))))
    };

    let mark = |occ: &mut Vec<Vec<bool>>, c: usize, r: usize, cs: usize, rs: usize| {
        while occ.len() < r + rs {
            occ.push(vec![false; cols]);
        }
        for row in occ.iter_mut().skip(r).take(rs) {
            for cell in row.iter_mut().skip(c).take(cs) {
                *cell = true;
            }
        }
    };

    let flow_indices: Vec<usize> = frame
        .children
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.absolute)
        .map(|(i, _)| i)
        .collect();

    let mut placed: Vec<Option<GridCell>> = vec![None; flow_indices.len()];

    // 1. Fully anchored children
    for (i, &idx) in flow_indices.iter().enumerate() {
        let child = &frame.children[idx];
        let cs = child.grid_col_span.clamp(1, cols);
        let rs = child.grid_row_span.clamp(1, max_rows);
        if let (Some(c), Some(r)) = (child.grid_col, child.grid_row) {
            let c_clamped = c.min(cols - cs);
            let r_clamped = r.min(max_rows - rs);
            mark(&mut occ, c_clamped, r_clamped, cs, rs);
            placed[i] = Some(GridCell {
                child_index: idx,
                col: c_clamped,
                row: r_clamped,
                col_span: cs,
                row_span: rs,
            });
        }
    }

    // 2. Unanchored / partially anchored children
    let mut cursor = (0usize, 0usize); // (row, col)
    for (i, &idx) in flow_indices.iter().enumerate() {
        if placed[i].is_some() {
            continue;
        }
        let child = &frame.children[idx];
        let cs = child.grid_col_span.clamp(1, cols);
        let rs = child.grid_row_span.clamp(1, max_rows);
        let last_row = max_rows.saturating_sub(rs);

        let (c, r) = match (child.grid_col, child.grid_row) {
            (Some(c), None) => {
                let c_clamped = c.min(cols - cs);
                let found_r = (0..=last_row).find(|&r| free(&occ, c_clamped, r, cs, rs)).unwrap_or(last_row);
                (c_clamped, found_r)
            }
            (None, Some(r)) => {
                let r_clamped = r.min(last_row);
                let found_c = (0..=cols - cs).find(|&c| free(&occ, c, r_clamped, cs, rs)).unwrap_or(0);
                (found_c, r_clamped)
            }
            _ => {
                let mut found = None;
                let (mut cr_r, mut cr_c) = cursor;
                while cr_r <= last_row {
                    if cr_c + cs <= cols && free(&occ, cr_c, cr_r, cs, rs) {
                        found = Some((cr_c, cr_r));
                        break;
                    }
                    cr_c += 1;
                    if cr_c + cs > cols {
                        cr_c = 0;
                        cr_r += 1;
                    }
                }
                let (c, r) = found.unwrap_or((0, last_row));
                cursor = (r, c + cs);
                (c, r)
            }
        };

        mark(&mut occ, c, r, cs, rs);
        placed[i] = Some(GridCell {
            child_index: idx,
            col: c,
            row: r,
            col_span: cs,
            row_span: rs,
        });
    }

    let cells: Vec<GridCell> = placed.into_iter().flatten().collect();
    let total_rows = cells.iter().map(|c| c.row + c.row_span).max().unwrap_or(1);
    (cells, total_rows)
}

/// Resolves track sizes for a list of track specs, given content sizes and available space.
pub fn resolve_tracks(specs: &[GridTrack], content: &[f64], avail: Option<f64>) -> Vec<f64> {
    let n = content.len();
    let mut out: Vec<f64> = (0..n)
        .map(|i| match specs.get(i).copied().unwrap_or(GridTrack::Fr(1.0)) {
            GridTrack::Fixed(v) => v.max(0.0),
            _ => content.get(i).copied().unwrap_or(0.0),
        })
        .collect();

    let Some(a) = avail else { return out };

    let fr = |i: usize| match specs.get(i).copied().unwrap_or(GridTrack::Fr(1.0)) {
        GridTrack::Fr(v) => v.max(0.0),
        _ => 0.0,
    };

    let (mut flex, fixed): (Vec<usize>, Vec<usize>) = (0..n).partition(|&i| {
        matches!(specs.get(i).copied().unwrap_or(GridTrack::Fr(1.0)), GridTrack::Fr(_))
    });

    let mut used: f64 = fixed.iter().map(|&i| out[i]).sum();

    while !flex.is_empty() {
        let total_fr: f64 = flex.iter().map(|&i| fr(i)).sum();
        let unit = if total_fr > 1e-12 { (a - used).max(0.0) / total_fr } else { 0.0 };
        let (open, frozen): (Vec<usize>, Vec<usize>) = flex.iter().partition(|&&i| unit * fr(i) >= out[i]);
        if frozen.is_empty() {
            for &i in &open {
                out[i] = unit * fr(i);
            }
            break;
        }
        for &i in &frozen {
            used += out[i];
        }
        flex = open;
    }

    out
}

pub fn solve_grid(frame: &mut AutoLayoutFrame, available_w: f64, available_h: f64) -> (f64, f64) {
    let n_cols = frame.grid_columns.len().max(1);
    let (cells, n_rows) = grid_place(frame, n_cols);

    let gap_x = frame.gap.max(0.0);
    let gap_y = frame.cross_gap.unwrap_or(frame.gap).max(0.0);

    let pt = frame.padding.top;
    let pr = frame.padding.right;
    let pb = frame.padding.bottom;
    let pl = frame.padding.left;

    let avail_inner_w = (available_w - pl - pr - gap_x * (n_cols - 1) as f64).max(0.0);
    let avail_inner_h = (available_h - pt - pb - gap_y * (n_rows - 1) as f64).max(0.0);

    // Compute content max sizes per single-span track
    let mut col_content = vec![0.0; n_cols];
    let mut row_content = vec![0.0; n_rows];

    for c in &cells {
        let child = &frame.children[c.child_index];
        if c.col_span == 1 && c.col < n_cols {
            if child.rect.width > col_content[c.col] {
                col_content[c.col] = child.rect.width;
            }
        }
        if c.row_span == 1 && c.row < n_rows {
            if child.rect.height > row_content[c.row] {
                row_content[c.row] = child.rect.height;
            }
        }
    }

    let opt_avail_w = if frame.sizing_h == SizingMode::Hug { None } else { Some(avail_inner_w) };
    let opt_avail_h = if frame.sizing_v == SizingMode::Hug { None } else { Some(avail_inner_h) };

    let col_widths = resolve_tracks(&frame.grid_columns, &col_content, opt_avail_w);
    let row_heights = resolve_tracks(&frame.grid_rows, &row_content, opt_avail_h);

    // Compute track starts
    let mut col_starts = Vec::with_capacity(n_cols);
    let mut cur_x = pl;
    for &w in &col_widths {
        col_starts.push(cur_x);
        cur_x += w + gap_x;
    }
    let total_grid_w = (cur_x - gap_x + pr).max(pl + pr);

    let mut row_starts = Vec::with_capacity(n_rows);
    let mut cur_y = pt;
    for &h in &row_heights {
        row_starts.push(cur_y);
        cur_y += h + gap_y;
    }
    let total_grid_h = (cur_y - gap_y + pb).max(pt + pb);

    // Position children in cells
    let frame_align_h = match frame.main_align {
        MainAlign::Center => CellAlign::Center,
        MainAlign::Max => CellAlign::Max,
        _ => CellAlign::Min,
    };
    let frame_align_v = match frame.cross_align {
        CrossAlign::Center => CellAlign::Center,
        CrossAlign::Max => CellAlign::Max,
        _ => CellAlign::Min,
    };

    let offset = |align: CellAlign, free_space: f64| match align {
        CellAlign::Min => 0.0,
        CellAlign::Center => free_space * 0.5,
        CellAlign::Max => free_space,
    };

    for c in &cells {
        let x0 = col_starts.get(c.col).copied().unwrap_or(pl);
        let cell_w = (0..c.col_span)
            .map(|offset| col_widths.get(c.col + offset).copied().unwrap_or(0.0))
            .sum::<f64>()
            + gap_x * (c.col_span - 1) as f64;

        let y0 = row_starts.get(c.row).copied().unwrap_or(pt);
        let cell_h = (0..c.row_span)
            .map(|offset| row_heights.get(c.row + offset).copied().unwrap_or(0.0))
            .sum::<f64>()
            + gap_y * (c.row_span - 1) as f64;

        let child = &mut frame.children[c.child_index];
        let fill_w = child.sizing_h == SizingMode::Fill;
        let fill_h = child.sizing_v == SizingMode::Fill;

        if fill_w {
            child.rect.width = clamp_size(cell_w, child.min_w, child.max_w);
        }
        if fill_h {
            child.rect.height = clamp_size(cell_h, child.min_h, child.max_h);
        }

        let ox = offset(child.align_self_h.unwrap_or(frame_align_h), cell_w - child.rect.width);
        let oy = offset(child.align_self_v.unwrap_or(frame_align_v), cell_h - child.rect.height);

        child.rect.x = x0 + ox;
        child.rect.y = y0 + oy;
    }

    (
        clamp_size(total_grid_w, frame.min_w, frame.max_w),
        clamp_size(total_grid_h, frame.min_h, frame.max_h),
    )
}

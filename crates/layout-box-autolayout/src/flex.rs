//! Flexbox layout solver: main-axis distribution, wrapping, baseline alignment, and hugging.

#![forbid(unsafe_code)]

use super::frame::AutoLayoutFrame;
use super::types::{ContentAlign, CrossAlign, LayoutDirection, MainAlign, SizingMode, clamp_size};

#[derive(Clone, Debug)]
pub struct FlexItem {
    pub index: usize,
    pub main: f64,
    pub cross: f64,
    pub fill_main: bool,
    pub fill_cross: bool,
    pub min_main: Option<f64>,
    pub max_main: Option<f64>,
    pub min_cross: Option<f64>,
    pub max_cross: Option<f64>,
    pub baseline: Option<f64>,
    pub base_off: f64,
}

pub fn main_offsets(align: MainAlign, inner: f64, total: f64, n: usize, gap: f64) -> (f64, f64) {
    let gaps = gap * n.saturating_sub(1) as f64;
    let free = inner - total;
    let k = n.max(1) as f64;
    match align {
        MainAlign::Min => (0.0, gap),
        MainAlign::Center => ((free - gaps) / 2.0, gap),
        MainAlign::Max => (free - gaps, gap),
        MainAlign::SpaceBetween => (0.0, if n > 1 { (free / (n - 1) as f64).max(0.0) } else { 0.0 }),
        MainAlign::SpaceAround => {
            let g = (free / k).max(0.0);
            (g / 2.0, g)
        }
        MainAlign::SpaceEvenly => {
            let g = (free / (k + 1.0)).max(0.0);
            (g, g)
        }
    }
}

/// Distributes free main-axis space among open fill items like CSS flexbox.
pub fn distribute(items: &mut [FlexItem], mut open: Vec<usize>, mut free: f64) {
    free = free.max(0.0);
    while !open.is_empty() {
        let share = free / open.len() as f64;
        let sized: Vec<(usize, f64)> = open
            .iter()
            .filter_map(|&i| items.get(i).map(|it| (i, clamp_size(share, it.min_main, it.max_main))))
            .collect();
        let violation: f64 = sized.iter().map(|(_, v)| v - share).sum();
        let frozen = |v: f64| violation.abs() < 1e-9 || (violation > 0.0 && v > share + 1e-9) || (violation < 0.0 && v < share - 1e-9);
        for (i, v) in &sized {
            if frozen(*v) {
                if let Some(it) = items.get_mut(*i) {
                    it.main = *v;
                    free -= *v;
                }
            }
        }
        free = free.max(0.0);
        open = sized.into_iter().filter(|(_, v)| !frozen(*v)).map(|(i, _)| i).collect();
    }
}

pub fn solve_flex(frame: &mut AutoLayoutFrame, available_w: f64, available_h: f64) -> (f64, f64) {
    let horizontal = frame.direction == LayoutDirection::Horizontal;
    let hug_main = if horizontal { frame.sizing_h == SizingMode::Hug } else { frame.sizing_v == SizingMode::Hug };
    let hug_cross = if horizontal { frame.sizing_v == SizingMode::Hug } else { frame.sizing_h == SizingMode::Hug };

    let pt = frame.padding.top;
    let pr = frame.padding.right;
    let pb = frame.padding.bottom;
    let pl = frame.padding.left;

    let pad_main = if horizontal { pl + pr } else { pt + pb };
    let pad_cross = if horizontal { pt + pb } else { pl + pr };

    let mut inner_main_avail = if horizontal { available_w - pad_main } else { available_h - pad_main };
    let mut inner_cross_avail = if horizontal { available_h - pad_cross } else { available_w - pad_cross };
    if inner_main_avail < 0.0 { inner_main_avail = 0.0; }
    if inner_cross_avail < 0.0 { inner_cross_avail = 0.0; }

    let mut items = Vec::new();
    let flow_indices: Vec<usize> = frame
        .children
        .iter()
        .enumerate()
        .filter(|(_, c)| !c.absolute)
        .map(|(i, _)| i)
        .collect();

    for &idx in &flow_indices {
        let child = &frame.children[idx];
        let (cw, ch) = (child.rect.width, child.rect.height);
        let (sm, sc) = if horizontal { (child.sizing_h, child.sizing_v) } else { (child.sizing_v, child.sizing_h) };
        let (min_main, max_main, min_cross, max_cross) = if horizontal {
            (child.min_w, child.max_w, child.min_h, child.max_h)
        } else {
            (child.min_h, child.max_h, child.min_w, child.max_w)
        };

        items.push(FlexItem {
            index: idx,
            main: if horizontal { cw } else { ch },
            cross: if horizontal { ch } else { cw },
            fill_main: sm == SizingMode::Fill && !hug_main,
            fill_cross: sc == SizingMode::Fill && !hug_cross,
            min_main,
            max_main,
            min_cross,
            max_cross,
            baseline: child.baseline_offset,
            base_off: 0.0,
        });
    }

    let gap = frame.gap.max(0.0);
    let cross_gap = frame.cross_gap.unwrap_or(gap).max(0.0);
    let wraps = frame.wrap && !hug_main;

    // Line breaking
    let mut lines: Vec<Vec<usize>> = Vec::new();
    if !items.is_empty() {
        if !wraps {
            lines.push((0..items.len()).collect());
        } else {
            let mut cur_line = Vec::new();
            let mut cur_main = 0.0;
            for (i, it) in items.iter().enumerate() {
                let item_gap = if cur_line.is_empty() { 0.0 } else { gap };
                if !cur_line.is_empty() && cur_main + item_gap + it.main > inner_main_avail + 1e-9 {
                    lines.push(cur_line);
                    cur_line = vec![i];
                    cur_main = it.main;
                } else {
                    cur_line.push(i);
                    cur_main += item_gap + it.main;
                }
            }
            if !cur_line.is_empty() {
                lines.push(cur_line);
            }
        }
    }

    // Distribute fill main space per line
    for line in &lines {
        let mut fixed_main = 0.0;
        let mut open_fill = Vec::new();
        for &i in line {
            if items[i].fill_main {
                open_fill.push(i);
            } else {
                fixed_main += items[i].main;
            }
        }
        let gaps_total = gap * line.len().saturating_sub(1) as f64;
        let free_for_fill = (inner_main_avail - fixed_main - gaps_total).max(0.0);
        if !open_fill.is_empty() {
            distribute(&mut items, open_fill, free_for_fill);
        }
    }

    // Baseline calculation for horizontal layouts
    if horizontal && frame.cross_align == CrossAlign::Baseline {
        for line in &lines {
            let mut max_base: f64 = 0.0;
            for &i in line {
                if let Some(b) = items[i].baseline {
                    if b > max_base {
                        max_base = b;
                    }
                }
            }
            for &i in line {
                items[i].base_off = max_base - items[i].baseline.unwrap_or(0.0);
            }
        }
    }

    // Compute line extents (used_main, line_cross)
    let mut line_extents: Vec<(f64, f64)> = Vec::new();
    for line in &lines {
        let mut line_m = 0.0;
        let mut line_c: f64 = 0.0;
        for (pos_in_line, &i) in line.iter().enumerate() {
            if pos_in_line > 0 {
                line_m += gap;
            }
            line_m += items[i].main;
            let item_c = if items[i].fill_cross && !hug_cross {
                clamp_size(inner_cross_avail, items[i].min_cross, items[i].max_cross)
            } else {
                items[i].cross + items[i].base_off
            };
            if item_c > line_c {
                line_c = item_c;
            }
        }
        line_extents.push((line_m, line_c));
    }

    let max_line_m = line_extents.iter().map(|l| l.0).fold(0.0f64, f64::max);
    let total_lines_c = line_extents.iter().map(|l| l.1).sum::<f64>()
        + cross_gap * lines.len().saturating_sub(1) as f64;

    let final_inner_main = if hug_main { max_line_m } else { inner_main_avail };
    let final_inner_cross = if hug_cross { total_lines_c } else { inner_cross_avail };

    // Positioning
    let mut cross_cursor = if horizontal { pt } else { pl };
    let n_lines = lines.len();

    let (mut cross_start_offset, effective_cross_gap) = if n_lines > 1 {
        let sum_c: f64 = line_extents.iter().map(|l| l.1).sum();
        let free_c = final_inner_cross - sum_c;
        match frame.content_align {
            ContentAlign::Min => (0.0, cross_gap),
            ContentAlign::Center => ((free_c - cross_gap * (n_lines - 1) as f64) * 0.5, cross_gap),
            ContentAlign::Max => (free_c - cross_gap * (n_lines - 1) as f64, cross_gap),
            ContentAlign::SpaceBetween => (0.0, if n_lines > 1 { free_c / (n_lines - 1) as f64 } else { 0.0 }),
            ContentAlign::SpaceAround => {
                let g = free_c / n_lines as f64;
                (g * 0.5, g)
            }
            ContentAlign::SpaceEvenly => {
                let g = free_c / (n_lines + 1) as f64;
                (g, g)
            }
        }
    } else {
        (0.0, cross_gap)
    };
    cross_cursor += cross_start_offset;

    for (li, line) in lines.iter().enumerate() {
        let (used_m, line_c) = line_extents.get(li).copied().unwrap_or((0.0, 0.0));
        let (mut main_pos, step_gap) = main_offsets(frame.main_align, final_inner_main, used_m, line.len(), gap);
        main_pos += if horizontal { pl } else { pt };

        for &i in line {
            let it = &items[i];
            let actual_cross = if it.fill_cross && !hug_cross {
                clamp_size(line_c, it.min_cross, it.max_cross)
            } else {
                it.cross
            };

            let cross_off = match frame.cross_align {
                CrossAlign::Min => 0.0,
                CrossAlign::Baseline => it.base_off,
                CrossAlign::Center => (line_c - actual_cross) * 0.5,
                CrossAlign::Max => line_c - actual_cross,
            };

            let (child_x, child_y) = if horizontal {
                (main_pos, cross_cursor + cross_off)
            } else {
                (cross_cursor + cross_off, main_pos)
            };

            let (final_w, final_h) = if horizontal {
                (it.main, actual_cross)
            } else {
                (actual_cross, it.main)
            };

            let child_node = &mut frame.children[it.index];
            child_node.rect.x = child_x;
            child_node.rect.y = child_y;
            child_node.rect.width = final_w;
            child_node.rect.height = final_h;

            main_pos += it.main + step_gap;
        }

        cross_cursor += line_c + effective_cross_gap;
    }

    let resolved_width = if horizontal {
        final_inner_main + pad_main
    } else {
        final_inner_cross + pad_main
    };
    let resolved_height = if horizontal {
        final_inner_cross + pad_cross
    } else {
        final_inner_main + pad_cross
    };

    (
        clamp_size(resolved_width, frame.min_w, frame.max_w),
        clamp_size(resolved_height, frame.min_h, frame.max_h),
    )
}

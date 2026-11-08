//! Area text and text-on-path layout engine.
//!
//! Grounded in authoring rights and creative preservation (UDHR Art. 19, 27).

use kurbo::{Point, Rect, Vec2};
use serde::{Deserialize, Serialize};

/// Alignment method for the first line's baseline inside an area text frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FirstBaseline {
    #[default]
    Ascent,
    CapHeight,
    Leading,
    Fixed(f64),
}

/// Options for area text frames (multi-column, insets, first baseline).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaTextOptions {
    /// Top, right, bottom, left insets in document points.
    pub insets: [f64; 4],
    /// Number of vertical columns (at least 1).
    pub columns: usize,
    /// Gutter space between columns in points.
    pub column_gutter: f64,
    /// Baseline offset calculation for first line.
    pub first_baseline: FirstBaseline,
    /// Default line height in points.
    pub line_height: f64,
}

impl Default for AreaTextOptions {
    fn default() -> Self {
        Self {
            insets: [0.0; 4],
            columns: 1,
            column_gutter: 12.0,
            first_baseline: FirstBaseline::Ascent,
            line_height: 14.0,
        }
    }
}

/// A positioned line box in a layout.
#[derive(Clone, Debug, PartialEq)]
pub struct LineBox {
    pub text: String,
    pub origin: Point,
    pub width: f64,
    pub height: f64,
    pub baseline_y: f64,
}

/// Layout result containing laid-out line boxes and overflow state.
#[derive(Clone, Debug, PartialEq)]
pub struct LayoutResult {
    pub lines: Vec<LineBox>,
    pub overflow: bool,
    pub text_bounds: Rect,
}

/// Lay out simple area text into a bounded rectangle.
pub fn layout_area_text(bounds: Rect, text: &str, opts: &AreaTextOptions) -> LayoutResult {
    let top_inset = opts.insets[0];
    let right_inset = opts.insets[1];
    let bottom_inset = opts.insets[2];
    let left_inset = opts.insets[3];

    let avail_w = (bounds.width() - left_inset - right_inset).max(0.0);
    let avail_h = (bounds.height() - top_inset - bottom_inset).max(0.0);

    let cols = opts.columns.max(1);
    let total_gutter = opts.column_gutter * (cols - 1) as f64;
    let col_w = ((avail_w - total_gutter) / cols as f64).max(0.0);

    let mut lines = Vec::new();
    let mut curr_col = 0;
    let mut curr_y = top_inset + match opts.first_baseline {
        FirstBaseline::Ascent => opts.line_height * 0.8,
        FirstBaseline::CapHeight => opts.line_height * 0.7,
        FirstBaseline::Leading => opts.line_height,
        FirstBaseline::Fixed(v) => v,
    };
    let mut overflow = false;

    // Simple whitespace word-wrapping
    for para in text.split('\n') {
        let words: Vec<&str> = para.split_whitespace().collect();
        if words.is_empty() {
            curr_y += opts.line_height;
            continue;
        }

        let mut line_words = Vec::new();
        let mut line_w = 0.0;
        let char_w = opts.line_height * 0.55;

        for word in words {
            let word_w = word.len() as f64 * char_w;
            let space_w = if line_words.is_empty() { 0.0 } else { char_w * 0.5 };

            if line_w + space_w + word_w <= col_w || line_words.is_empty() {
                line_words.push(word);
                line_w += space_w + word_w;
            } else {
                // Emit line
                if curr_y + opts.line_height * 0.2 > bounds.height() - bottom_inset {
                    curr_col += 1;
                    if curr_col >= cols {
                        overflow = true;
                        break;
                    }
                    curr_y = top_inset + opts.line_height * 0.8;
                }

                let col_x = bounds.x0 + left_inset + curr_col as f64 * (col_w + opts.column_gutter);
                lines.push(LineBox {
                    text: line_words.join(" "),
                    origin: Point::new(col_x, bounds.y0 + curr_y - opts.line_height * 0.8),
                    width: line_w,
                    height: opts.line_height,
                    baseline_y: bounds.y0 + curr_y,
                });

                curr_y += opts.line_height;
                line_words = vec![word];
                line_w = word_w;
            }
        }

        if overflow {
            break;
        }

        if !line_words.is_empty() {
            if curr_y + opts.line_height * 0.2 > bounds.height() - bottom_inset {
                curr_col += 1;
                if curr_col >= cols {
                    overflow = true;
                    break;
                }
                curr_y = top_inset + opts.line_height * 0.8;
            }
            let col_x = bounds.x0 + left_inset + curr_col as f64 * (col_w + opts.column_gutter);
            lines.push(LineBox {
                text: line_words.join(" "),
                origin: Point::new(col_x, bounds.y0 + curr_y - opts.line_height * 0.8),
                width: line_w,
                height: opts.line_height,
                baseline_y: bounds.y0 + curr_y,
            });
            curr_y += opts.line_height;
        }
    }

    let text_bounds = if lines.is_empty() {
        Rect::ZERO
    } else {
        lines.iter().fold(
            Rect::from_origin_size(lines[0].origin, (lines[0].width, lines[0].height)),
            |acc, l| acc.union(Rect::from_origin_size(l.origin, (l.width, l.height))),
        )
    };

    LayoutResult {
        lines,
        overflow,
        text_bounds,
    }
}

/// Text on path alignment option.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum PathAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

/// Text on path placement options.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextOnPathOptions {
    pub align: PathAlign,
    pub start_offset: f64,
    pub flip: bool,
    pub char_width: f64,
}

impl Default for TextOnPathOptions {
    fn default() -> Self {
        Self {
            align: PathAlign::Left,
            start_offset: 0.0,
            flip: false,
            char_width: 8.0,
        }
    }
}

/// Lay out characters along a sequence of 2D curve points.
pub fn layout_text_on_path(
    curve_points: &[Point],
    text: &str,
    opts: &TextOnPathOptions,
) -> Vec<(char, Point, f64)> {
    if curve_points.len() < 2 || text.is_empty() {
        return Vec::new();
    }

    let mut lengths = vec![0.0];
    let mut total_len = 0.0;
    for w in curve_points.windows(2) {
        total_len += w[0].distance(w[1]);
        lengths.push(total_len);
    }

    let text_len = text.chars().count() as f64 * opts.char_width;
    let initial_offset = match opts.align {
        PathAlign::Left => opts.start_offset,
        PathAlign::Center => (total_len - text_len) * 0.5 + opts.start_offset,
        PathAlign::Right => (total_len - text_len) + opts.start_offset,
        PathAlign::Justify => opts.start_offset,
    };

    let sample_at = |dist: f64| -> (Point, f64) {
        let dist = dist.clamp(0.0, total_len);
        let idx = match lengths.binary_search_by(|v| v.partial_cmp(&dist).unwrap()) {
            Ok(i) => i.min(curve_points.len() - 1),
            Err(i) => (i.saturating_sub(1)).min(curve_points.len() - 2),
        };
        let p0 = curve_points[idx];
        let p1 = curve_points[(idx + 1).min(curve_points.len() - 1)];
        let seg_len = p0.distance(p1).max(1e-9);
        let seg_t = ((dist - lengths[idx]) / seg_len).clamp(0.0, 1.0);
        let pt = p0.lerp(p1, seg_t);
        let angle = (p1.y - p0.y).atan2(p1.x - p0.x);
        (pt, angle)
    };

    let mut out = Vec::new();
    let mut cur_dist = initial_offset;

    for ch in text.chars() {
        if cur_dist > total_len {
            break;
        }
        let (pt, mut angle) = sample_at(cur_dist + opts.char_width * 0.5);
        if opts.flip {
            angle += std::f64::consts::PI;
        }
        out.push((ch, pt, angle));
        cur_dist += opts.char_width;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_text_layout_wraps_lines() {
        let r = Rect::new(0.0, 0.0, 100.0, 100.0);
        let text = "One two three four five six seven eight nine ten";
        let res = layout_area_text(r, text, &AreaTextOptions::default());
        assert!(res.lines.len() >= 2);
        assert!(!res.overflow);
    }

    #[test]
    fn text_on_path_positions_glyphs() {
        let pts = [Point::new(0.0, 0.0), Point::new(100.0, 0.0)];
        let res = layout_text_on_path(&pts, "ABC", &TextOnPathOptions::default());
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].0, 'A');
        assert!(res[0].1.x >= 0.0);
    }
}

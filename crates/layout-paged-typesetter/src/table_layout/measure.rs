//! Table column min/max content width measurement.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableCellData {
    pub text: String,
    pub col_span: usize,
    pub row_span: usize,
    pub margin_left: f32,
    pub margin_right: f32,
}

impl TableCellData {
    pub fn new(text: impl Into<String>) -> Self {
        TableCellData {
            text: text.into(),
            col_span: 1,
            row_span: 1,
            margin_left: 5.4,
            margin_right: 5.4,
        }
    }

    pub fn with_span(mut self, col_span: usize) -> Self {
        self.col_span = col_span.max(1);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableData {
    pub rows: Vec<Vec<TableCellData>>,
}

impl TableData {
    pub fn col_count(&self) -> usize {
        let mut max_cols = 0;
        for row in &self.rows {
            let mut cols = 0;
            for cell in row {
                cols += cell.col_span;
            }
            max_cols = max_cols.max(cols);
        }
        max_cols
    }
}

/// Measures the (min_width, max_width) for each column in points.
///
/// - `min_width`: Longest non-breakable word plus padding.
/// - `max_width`: Entire content on a single line plus padding.
pub fn measure_table_columns(table: &TableData, font_size: f32) -> Vec<(f32, f32)> {
    let num_cols = table.col_count();
    if num_cols == 0 {
        return Vec::new();
    }

    let mut col_bounds = vec![(0.0f32, 0.0f32); num_cols];
    let char_width = font_size * 0.55;

    // Single-column cells pass
    for row in &table.rows {
        let mut col_idx = 0;
        for cell in row {
            let padding = cell.margin_left + cell.margin_right;
            if cell.col_span == 1 && col_idx < num_cols {
                let (min_w, max_w) = measure_cell_text(&cell.text, char_width);
                col_bounds[col_idx].0 = col_bounds[col_idx].0.max(min_w + padding);
                col_bounds[col_idx].1 = col_bounds[col_idx].1.max(max_w + padding);
            }
            col_idx += cell.col_span;
        }
    }

    // Multi-column spanning cells pass: distribute excess evenly
    for row in &table.rows {
        let mut col_idx = 0;
        for cell in row {
            let padding = cell.margin_left + cell.margin_right;
            if cell.col_span > 1 && col_idx < num_cols {
                let (min_w, max_w) = measure_cell_text(&cell.text, char_width);
                let needed_min = min_w + padding;
                let needed_max = max_w + padding;

                let end_col = (col_idx + cell.col_span).min(num_cols);
                let current_min: f32 = col_bounds[col_idx..end_col].iter().map(|c| c.0).sum();
                let current_max: f32 = col_bounds[col_idx..end_col].iter().map(|c| c.1).sum();

                let span_f = (end_col - col_idx) as f32;
                if needed_min > current_min {
                    let diff = (needed_min - current_min) / span_f;
                    for c in &mut col_bounds[col_idx..end_col] {
                        c.0 += diff;
                    }
                }
                if needed_max > current_max {
                    let diff = (needed_max - current_max) / span_f;
                    for c in &mut col_bounds[col_idx..end_col] {
                        c.1 += diff;
                    }
                }
            }
            col_idx += cell.col_span;
        }
    }

    col_bounds
}

fn measure_cell_text(text: &str, char_w: f32) -> (f32, f32) {
    if text.is_empty() {
        return (char_w, char_w);
    }
    let mut max_word_len = 0;
    let mut total_len = 0;
    for line in text.lines() {
        total_len = total_len.max(line.chars().count());
        for word in line.split_whitespace() {
            max_word_len = max_word_len.max(word.chars().count());
        }
    }
    (max_word_len as f32 * char_w, total_len as f32 * char_w)
}

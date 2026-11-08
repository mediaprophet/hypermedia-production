//! Multi-page and multi-column pagination engine with widow/orphan control.

use serde::{Deserialize, Serialize};
use crate::pagination::types::{Page, PlacedItem, Rect, SectionProps};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParaDoc {
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub space_before: f32,
    pub space_after: f32,
    pub keep_with_next: bool,
    pub keep_lines: bool,
    pub page_break_before: bool,
    pub widow_orphan_control: bool,
    pub footnotes: Vec<String>,
}

impl Default for ParaDoc {
    fn default() -> Self {
        ParaDoc {
            text: String::new(),
            font_size: 11.0,
            line_height: 14.0,
            space_before: 0.0,
            space_after: 6.0,
            keep_with_next: false,
            keep_lines: false,
            page_break_before: false,
            widow_orphan_control: true,
            footnotes: Vec::new(),
        }
    }
}

impl ParaDoc {
    pub fn new(text: impl Into<String>) -> Self {
        ParaDoc {
            text: text.into(),
            ..Default::default()
        }
    }

    pub fn heading(text: impl Into<String>, level: u8) -> Self {
        let (font_size, line_height, space_before, space_after) = match level {
            1 => (18.0, 22.0, 12.0, 6.0),
            2 => (14.0, 18.0, 10.0, 4.0),
            _ => (12.0, 16.0, 8.0, 4.0),
        };
        ParaDoc {
            text: text.into(),
            font_size,
            line_height,
            space_before,
            space_after,
            keep_with_next: true,
            keep_lines: true,
            page_break_before: false,
            widow_orphan_control: true,
            footnotes: Vec::new(),
        }
    }
}

pub struct Typesetter {
    pub section: SectionProps,
}

impl Typesetter {
    pub fn new(section: SectionProps) -> Self {
        Typesetter { section }
    }

    /// Paginates paragraphs into pages.
    pub fn paginate(&self, paras: &[ParaDoc], figures: &[Rect]) -> Vec<Page> {
        let col_w = self.section.column_width();
        let num_cols = self.section.num_columns.max(1);
        let col_gap = self.section.column_gap;
        let content_h = self.section.content_height();

        let mut pages: Vec<Page> = Vec::new();
        let mut curr_page_idx = 1;
        let mut curr_col_idx = 0;
        let mut curr_y = 0.0f32;
        let mut footnote_counter = 1usize;
        let mut curr_page = Page {
            number: curr_page_idx,
            items: Vec::new(),
            footnotes: Vec::new(),
        };

        let mut p_idx = 0;
        while p_idx < paras.len() {
            let p = &paras[p_idx];

            if p.page_break_before && (!curr_page.items.is_empty() || curr_col_idx > 0) {
                // Force page break
                pages.push(curr_page);
                curr_page_idx += 1;
                curr_page = Page {
                    number: curr_page_idx,
                    items: Vec::new(),
                    footnotes: Vec::new(),
                };
                curr_col_idx = 0;
                curr_y = 0.0;
            }

            // Break lines
            let char_w = p.font_size * 0.55;
            let max_chars_per_line = ((col_w / char_w).floor() as usize).max(1);
            let words: Vec<&str> = p.text.split_whitespace().collect();
            let mut lines: Vec<String> = Vec::new();
            let mut curr_line = String::new();

            for word in words {
                if curr_line.is_empty() {
                    curr_line.push_str(word);
                } else if curr_line.len() + 1 + word.len() <= max_chars_per_line {
                    curr_line.push(' ');
                    curr_line.push_str(word);
                } else {
                    lines.push(std::mem::take(&mut curr_line));
                    curr_line.push_str(word);
                }
            }
            if !curr_line.is_empty() {
                lines.push(curr_line);
            }
            if lines.is_empty() {
                lines.push(String::new());
            }

            let total_lines = lines.len();
            let para_h = (total_lines as f32) * p.line_height + p.space_before + p.space_after;

            // Check keep_with_next: if heading doesn't leave room for at least 2 lines of next para
            if p.keep_with_next && p_idx + 1 < paras.len() {
                let next_p = &paras[p_idx + 1];
                let needed_next = 2.0 * next_p.line_height;
                if curr_y + para_h + needed_next > content_h {
                    // Advance to next column or page
                    curr_col_idx += 1;
                    curr_y = 0.0;
                    if curr_col_idx >= num_cols {
                        pages.push(curr_page);
                        curr_page_idx += 1;
                        curr_page = Page {
                            number: curr_page_idx,
                            items: Vec::new(),
                            footnotes: Vec::new(),
                        };
                        curr_col_idx = 0;
                    }
                }
            }

            // Check keep_lines
            if p.keep_lines && curr_y + para_h > content_h && curr_y > 0.0 {
                curr_col_idx += 1;
                curr_y = 0.0;
                if curr_col_idx >= num_cols {
                    pages.push(curr_page);
                    curr_page_idx += 1;
                    curr_page = Page {
                        number: curr_page_idx,
                        items: Vec::new(),
                        footnotes: Vec::new(),
                    };
                    curr_col_idx = 0;
                }
            }

            // Layout lines with widow/orphan control
            let mut l_idx = 0;
            while l_idx < total_lines {
                let remaining_lines = total_lines - l_idx;
                let available_lines = ((content_h - curr_y - p.space_before) / p.line_height).floor() as usize;

                if available_lines == 0 && curr_y > 0.0 {
                    // Advance column / page
                    curr_col_idx += 1;
                    curr_y = 0.0;
                    if curr_col_idx >= num_cols {
                        pages.push(curr_page);
                        curr_page_idx += 1;
                        curr_page = Page {
                            number: curr_page_idx,
                            items: Vec::new(),
                            footnotes: Vec::new(),
                        };
                        curr_col_idx = 0;
                    }
                    continue;
                }

                let mut take_lines = available_lines.min(remaining_lines).max(1);

                // Widow/orphan rule: minimum 2 lines at bottom and top
                if p.widow_orphan_control && total_lines >= 4 {
                    if take_lines == 1 && remaining_lines > 1 {
                        // Orphan: don't leave 1 line at the bottom
                        curr_col_idx += 1;
                        curr_y = 0.0;
                        if curr_col_idx >= num_cols {
                            pages.push(curr_page);
                            curr_page_idx += 1;
                            curr_page = Page {
                                number: curr_page_idx,
                                items: Vec::new(),
                                footnotes: Vec::new(),
                            };
                            curr_col_idx = 0;
                        }
                        continue;
                    } else if remaining_lines - take_lines == 1 {
                        // Widow: don't leave 1 line alone at top of next page
                        take_lines = take_lines.saturating_sub(1);
                    }
                }

                if l_idx == 0 {
                    curr_y += p.space_before;
                }

                let col_x = self.section.margin_left + (curr_col_idx as f32) * (col_w + col_gap);
                for i in 0..take_lines {
                    let line_text = &lines[l_idx + i];
                    let line_rect = Rect::new(col_x, self.section.margin_top + curr_y, col_w, p.line_height);
                    curr_page.items.push(PlacedItem::Line {
                        rect: line_rect,
                        text: line_text.clone(),
                        font_size: p.font_size,
                    });
                    curr_y += p.line_height;
                }

                l_idx += take_lines;

                if l_idx == total_lines {
                    curr_y += p.space_after;
                } else {
                    // Split occurred, advance column/page
                    curr_col_idx += 1;
                    curr_y = 0.0;
                    if curr_col_idx >= num_cols {
                        pages.push(curr_page);
                        curr_page_idx += 1;
                        curr_page = Page {
                            number: curr_page_idx,
                            items: Vec::new(),
                            footnotes: Vec::new(),
                        };
                        curr_col_idx = 0;
                    }
                }
            }

            // Append footnotes
            for fn_text in &p.footnotes {
                let fn_h = 12.0;
                let fn_y = self.section.page_h - self.section.margin_bottom - (curr_page.footnotes.len() as f32 + 1.0) * fn_h;
                let fn_rect = Rect::new(self.section.margin_left, fn_y, self.section.content_width(), fn_h);
                curr_page.footnotes.push(PlacedItem::Footnote {
                    rect: fn_rect,
                    number: footnote_counter,
                    text: fn_text.clone(),
                });
                footnote_counter += 1;
            }

            p_idx += 1;
        }

        if !curr_page.items.is_empty() || !curr_page.footnotes.is_empty() {
            pages.push(curr_page);
        }

        pages
    }
}

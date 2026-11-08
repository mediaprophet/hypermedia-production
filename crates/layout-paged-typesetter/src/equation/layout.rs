//! 2D spatial equation box layout.

use crate::equation::ast::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MItem {
    Text {
        x: f32,
        y: f32,
        text: String,
        font_size: f32,
    },
    Line {
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        thickness: f32,
    },
    Rect {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MathLayout {
    pub width: f32,
    pub ascent: f32,
    pub descent: f32,
    pub items: Vec<MItem>,
}

impl MathLayout {
    pub fn height(&self) -> f32 {
        self.ascent + self.descent
    }

    pub fn shift(&mut self, dx: f32, dy: f32) {
        for item in &mut self.items {
            match item {
                MItem::Text { x, y, .. } => {
                    *x += dx;
                    *y += dy;
                }
                MItem::Line { x0, y0, x1, y1, .. } => {
                    *x0 += dx;
                    *x1 += dx;
                    *y0 += dy;
                    *y1 += dy;
                }
                MItem::Rect { x, y, .. } => {
                    *x += dx;
                    *y += dy;
                }
            }
        }
    }
}

pub fn layout_equation(nodes: &[MNode], font_size: f32) -> MathLayout {
    let mut layout = MathLayout::default();
    let em = font_size;
    let mut x_cursor = 0.0f32;

    for node in nodes {
        match node {
            MNode::Run(r) => {
                let char_w = em * 0.55;
                let w = r.text.chars().count() as f32 * char_w;
                let asc = em * 0.8;
                let desc = em * 0.2;
                layout.items.push(MItem::Text {
                    x: x_cursor,
                    y: 0.0,
                    text: r.text.clone(),
                    font_size,
                });
                x_cursor += w;
                layout.ascent = layout.ascent.max(asc);
                layout.descent = layout.descent.max(desc);
            }
            MNode::Frac { num, den, .. } => {
                let sub_size = font_size * 0.75;
                let num_layout = layout_equation(num, sub_size);
                let den_layout = layout_equation(den, sub_size);
                let frac_w = num_layout.width.max(den_layout.width) + em * 0.2;

                let rule_y = em * 0.25;
                let gap = em * 0.15;

                // Center numerator above rule
                let num_x = x_cursor + (frac_w - num_layout.width) / 2.0;
                let num_y = rule_y + gap + num_layout.descent;
                let mut num_shifted = num_layout.clone();
                num_shifted.shift(num_x, num_y);
                layout.items.extend(num_shifted.items);

                // Center denominator below rule
                let den_x = x_cursor + (frac_w - den_layout.width) / 2.0;
                let den_y = rule_y - gap - den_layout.ascent;
                let mut den_shifted = den_layout.clone();
                den_shifted.shift(den_x, den_y);
                layout.items.extend(den_shifted.items);

                // Draw fraction bar
                layout.items.push(MItem::Line {
                    x0: x_cursor,
                    y0: rule_y,
                    x1: x_cursor + frac_w,
                    y1: rule_y,
                    thickness: (em * 0.06).max(1.0),
                });

                layout.ascent = layout.ascent.max(num_y + num_layout.ascent);
                layout.descent = layout.descent.max(-den_y + den_layout.descent);
                x_cursor += frac_w + em * 0.1;
            }
            MNode::Script { kind, base, sub, sup } => {
                let mut base_layout = layout_equation(base, font_size);
                base_layout.shift(x_cursor, 0.0);
                layout.items.extend(base_layout.items);
                x_cursor += base_layout.width;

                let script_size = font_size * 0.7;
                let sup_layout = layout_equation(sup, script_size);
                let sub_layout = layout_equation(sub, script_size);

                match kind {
                    ScriptKind::Sup => {
                        let sup_y = em * 0.45;
                        let mut s_shift = sup_layout.clone();
                        s_shift.shift(x_cursor, sup_y);
                        layout.items.extend(s_shift.items);
                        layout.ascent = layout.ascent.max(sup_y + sup_layout.ascent);
                        x_cursor += sup_layout.width;
                    }
                    ScriptKind::Sub => {
                        let sub_y = -em * 0.25;
                        let mut s_shift = sub_layout.clone();
                        s_shift.shift(x_cursor, sub_y);
                        layout.items.extend(s_shift.items);
                        layout.descent = layout.descent.max(-sub_y + sub_layout.descent);
                        x_cursor += sub_layout.width;
                    }
                    ScriptKind::SubSup | ScriptKind::Pre => {
                        let sup_y = em * 0.45;
                        let sub_y = -em * 0.25;
                        let mut sup_shift = sup_layout.clone();
                        let mut sub_shift = sub_layout.clone();
                        sup_shift.shift(x_cursor, sup_y);
                        sub_shift.shift(x_cursor, sub_y);
                        layout.items.extend(sup_shift.items);
                        layout.items.extend(sub_shift.items);
                        layout.ascent = layout.ascent.max(sup_y + sup_layout.ascent);
                        layout.descent = layout.descent.max(-sub_y + sub_layout.descent);
                        x_cursor += sup_layout.width.max(sub_layout.width);
                    }
                }
            }
            MNode::Rad { deg, e, .. } => {
                let e_layout = layout_equation(e, font_size);
                let surd_w = em * 0.6;
                let top_y = e_layout.ascent + em * 0.15;

                // Radical line over the expression
                layout.items.push(MItem::Line {
                    x0: x_cursor + surd_w,
                    y0: top_y,
                    x1: x_cursor + surd_w + e_layout.width,
                    y1: top_y,
                    thickness: (em * 0.06).max(1.0),
                });

                // Surd checkmark
                layout.items.push(MItem::Text {
                    x: x_cursor,
                    y: 0.0,
                    text: "√".to_string(),
                    font_size: font_size * 1.2,
                });

                let mut e_shifted = e_layout.clone();
                e_shifted.shift(x_cursor + surd_w, 0.0);
                layout.items.extend(e_shifted.items);

                layout.ascent = layout.ascent.max(top_y + em * 0.05);
                layout.descent = layout.descent.max(e_layout.descent);
                x_cursor += surd_w + e_layout.width + em * 0.1;
            }
            MNode::Delim { beg, end, elems } => {
                if let Some(b) = beg {
                    layout.items.push(MItem::Text {
                        x: x_cursor,
                        y: 0.0,
                        text: b.to_string(),
                        font_size: font_size * 1.1,
                    });
                    x_cursor += em * 0.4;
                }
                for elem in elems {
                    let mut el = layout_equation(elem, font_size);
                    el.shift(x_cursor, 0.0);
                    layout.items.extend(el.items);
                    layout.ascent = layout.ascent.max(el.ascent);
                    layout.descent = layout.descent.max(el.descent);
                    x_cursor += el.width;
                }
                if let Some(e) = end {
                    layout.items.push(MItem::Text {
                        x: x_cursor,
                        y: 0.0,
                        text: e.to_string(),
                        font_size: font_size * 1.1,
                    });
                    x_cursor += em * 0.4;
                }
            }
            MNode::Nary { chr, sub, sup, e, .. } => {
                let nary_w = em * 0.8;
                layout.items.push(MItem::Text {
                    x: x_cursor,
                    y: 0.0,
                    text: chr.to_string(),
                    font_size: font_size * 1.4,
                });
                let mut e_layout = layout_equation(e, font_size);
                e_layout.shift(x_cursor + nary_w, 0.0);
                layout.items.extend(e_layout.items);
                layout.ascent = layout.ascent.max(e_layout.ascent).max(em);
                layout.descent = layout.descent.max(e_layout.descent).max(em * 0.4);
                x_cursor += nary_w + e_layout.width + em * 0.1;
            }
            MNode::Matrix { rows, .. } => {
                let row_gap = em * 0.2;
                let col_gap = em * 0.4;
                let mut max_col_w = Vec::new();

                for r in rows {
                    for (ci, c) in r.iter().enumerate() {
                        let l = layout_equation(c, font_size);
                        if ci >= max_col_w.len() {
                            max_col_w.push(l.width);
                        } else {
                            max_col_w[ci] = max_col_w[ci].max(l.width);
                        }
                    }
                }

                let total_w: f32 = max_col_w.iter().sum::<f32>() + (max_col_w.len().saturating_sub(1) as f32) * col_gap;
                let total_h = rows.len() as f32 * (em + row_gap);

                let mut curr_y = (total_h / 2.0) - em * 0.5;
                for r in rows {
                    let mut curr_x = x_cursor;
                    for (ci, c) in r.iter().enumerate() {
                        let mut cell_l = layout_equation(c, font_size);
                        let w = max_col_w.get(ci).copied().unwrap_or(0.0);
                        let offset_x = curr_x + (w - cell_l.width) / 2.0;
                        cell_l.shift(offset_x, curr_y);
                        layout.items.extend(cell_l.items);
                        curr_x += w + col_gap;
                    }
                    curr_y -= em + row_gap;
                }

                layout.ascent = layout.ascent.max(total_h / 2.0);
                layout.descent = layout.descent.max(total_h / 2.0);
                x_cursor += total_w + em * 0.1;
            }
            MNode::Acc { chr, e } => {
                let mut e_l = layout_equation(e, font_size);
                e_l.shift(x_cursor, 0.0);
                layout.items.extend(e_l.items);
                let acc_y = e_l.ascent + em * 0.05;
                layout.items.push(MItem::Text {
                    x: x_cursor + e_l.width / 2.0 - em * 0.2,
                    y: acc_y,
                    text: chr.to_string(),
                    font_size: font_size * 0.9,
                });
                layout.ascent = layout.ascent.max(acc_y + em * 0.3);
                layout.descent = layout.descent.max(e_l.descent);
                x_cursor += e_l.width + em * 0.05;
            }
        }
    }

    layout.width = x_cursor;
    layout
}
